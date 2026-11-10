// End-to-end body-parity capture test.
//
// `byte_parity.rs` asserts the *model serialization* in isolation. These tests
// go one step further: they point the real POC binary at a throwaway HTTP
// server and capture the exact request bytes it writes on the wire, then
// compare them field-by-field against the eMASS reference tool's expected
// shape. This exercises the full stack (handler -> client serialization ->
// reqwest) exactly like `run_poc.sh`'s `verify_body_parity`, but as a
// deterministic, CI-runnable regression test with no live eMASS behind it.
//
// The POC forces `https` only when the host lacks a scheme; feeding it a plain
// `http://127.0.0.1:PORT` keeps the transport TLS-free so a std::net server can
// capture the request. No mTLS client cert is configured, so the POC skips the
// client-cert step and connects over plain HTTP.

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

/// A parsed HTTP request captured from the POC binary.
struct Captured {
    method: String,
    path: String,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

/// Path to the freshly-built emasser binary.
///
/// Uses Cargo's `CARGO_BIN_EXE_<bin>` env var, which is guaranteed to point at
/// the sibling binary cargo builds *for this test run* — no manual
/// `target/debug`/`target/release` globbing, so there is no build-ordering race
/// when `cargo test` also recompiles the client crate.
fn poc_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_emasser"))
}

/// Index of the CRLF+LFCR header/body terminator, if present.
fn header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

/// Read one HTTP/1.1 request (request line + headers + body) from the socket.
fn read_request(sock: &mut TcpStream) -> std::io::Result<Captured> {
    sock.set_read_timeout(Some(Duration::from_secs(15))).ok();
    let mut buf: Vec<u8> = Vec::new();

    // 1) Read until the end of the header block.
    loop {
        let mut tmp = [0u8; 4096];
        let n = sock.read(&mut tmp)?;
        if n == 0 {
            break; // connection closed before headers/body completed
        }
        buf.extend_from_slice(&tmp[..n]);
        if header_end(&buf).is_some() {
            break;
        }
    }
    let he = header_end(&buf).ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "no header terminator")
    })?;
    let header_text = String::from_utf8_lossy(&buf[..he]).to_string();

    let mut method = String::new();
    let mut path = String::new();
    let mut headers = BTreeMap::new();
    for (i, line) in header_text.split("\r\n").enumerate() {
        if i == 0 {
            let mut toks = line.split_whitespace();
            method = toks.next().unwrap_or("").to_string();
            path = toks.next().unwrap_or("").to_string();
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.insert(k.trim().to_lowercase(), v.trim().to_string());
            if k.eq_ignore_ascii_case("expect") && v.contains("100-continue") {
                // Handshake: let the client send the body.
                let _ = sock.write_all(b"HTTP/1.1 100 Continue\r\n\r\n");
                let _ = sock.flush();
            }
        }
    }

    // 2) Read exactly Content-Length body bytes.
    let content_length: usize = headers
        .get("content-length")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let body_start = he + 4;
    while buf.len() - body_start < content_length {
        let mut tmp = [0u8; 4096];
        let n = sock.read(&mut tmp)?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&tmp[..n]);
    }
    let end = (body_start + content_length).min(buf.len());
    let body = buf[body_start..end].to_vec();
    Ok(Captured {
        method,
        path,
        headers,
        body,
    })
}

/// Minimal HTTP/1.1 response the POC's reqwest client can consume.
fn respond(sock: &mut TcpStream, body: &str) {
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = sock.write_all(resp.as_bytes());
    let _ = sock.flush();
}

/// Run the POC binary once against a throwaway server and capture the request.
fn capture(command: &[&str]) -> Captured {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let addr = listener.local_addr().expect("local addr");

    let mut child = Command::new(poc_bin())
        .args(command)
        .env("EMASSER_HOST_URL", format!("http://{}", addr))
        .env("EMASSER_API_KEY", "test-key")
        .env("EMASSER_USER_UID", "123")
        .env("EMASSER_VERIFY_SSL", "false")
        .spawn()
        .expect("spawn emasser");

    let (mut sock, _peer) = listener.accept().expect("accept");
    let captured = read_request(&mut sock).expect("read request");
    respond(&mut sock, r#"{"data":{"status":"ok","mockType":"stable"}}"#);
    let _ = child.wait();
    captured
}

#[test]
fn clear_posts_single_element_array() {
    let cap = capture(&[
        "post",
        "scan",
        "clear",
        "--systemId",
        "1",
        "--name",
        "SomeApp",
        "--version",
        "1.2.3",
        "--clearFindings",
        "true",
    ]);

    // Method + path parity (client builds /api/systems/{id}/static-code-scans).
    assert_eq!(cap.method, "POST", "clear must POST");
    assert_eq!(
        cap.path, "/api/systems/1/static-code-scans",
        "unexpected path: {}",
        cap.path
    );

    // Header parity: a JSON body sized to match the payload.
    assert_eq!(
        cap.headers.get("content-type").map(String::as_str),
        Some("application/json")
    );
    let cl: usize = cap
        .headers
        .get("content-length")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    assert_eq!(cl, cap.body.len(), "content-length must match body bytes");

    // Body parity: exact JSON shape (object-key order independent).
    let got: serde_json::Value = serde_json::from_slice(&cap.body).expect("POC emitted valid JSON");
    let expected = serde_json::json!([{
        "application": { "applicationName": "SomeApp", "version": "1.2.3" },
        "applicationFindings": [ { "clearFindings": true } ],
    }]);
    println!("clear wire body: {}", String::from_utf8_lossy(&cap.body));
    assert_eq!(
        got, expected,
        "clear wire body must match the reference tool's shape"
    );
    assert!(
        got.is_array(),
        "clear body must be a JSON array, got object"
    );
}

#[test]
fn add_static_code_posts_the_body_array() {
    let findings_arg = r#"[{"rawSeverity":"critical","codeCheckName":"CSP001","count":3,"scanDate":1700000001,"cweId":"CWE-79"}]"#;
    let cap = capture(&[
        "post",
        "scan",
        "add",
        "static",
        "code",
        "--systemId",
        "2",
        "--name",
        "Inventory",
        "--version",
        "2.0",
        "--findings",
        findings_arg,
    ]);

    assert_eq!(cap.method, "POST");
    assert_eq!(
        cap.path, "/api/systems/2/static-code-scans",
        "unexpected path: {}",
        cap.path
    );

    let got: serde_json::Value = serde_json::from_slice(&cap.body).expect("POC emitted valid JSON");
    println!("add wire body: {}", String::from_utf8_lossy(&cap.body));

    // One-element array wrapping a single application object.
    let arr = got.as_array().expect("add body must be a JSON array");
    assert_eq!(arr.len(), 1, "must wrap exactly one application object");
    let obj = &arr[0];
    assert_eq!(obj["application"]["applicationName"], json_str("Inventory"));
    assert_eq!(obj["application"]["version"], json_str("2.0"));

    let f_arr = obj["applicationFindings"]
        .as_array()
        .expect("findings array");
    assert_eq!(f_arr.len(), 1);
    // The client canonicalizes rawSeverity to title-case on the wire.
    assert_eq!(f_arr[0]["rawSeverity"], json_str("Critical"));
    assert_eq!(f_arr[0]["codeCheckName"], json_str("CSP001"));
    assert_eq!(f_arr[0]["count"], serde_json::Value::from(3));
}

fn json_str(s: &str) -> serde_json::Value {
    serde_json::Value::String(s.to_string())
}
