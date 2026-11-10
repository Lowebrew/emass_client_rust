// CLI smoke test.
//
// End-to-end gate that points the real `emasser` POC binary at a throwaway
// in-process HTTP server and drives the full command surface through it. This
// is deliberately lightweight next to `parity_capture.rs` (which asserts
// exact wire-body parity): here we care that every ported command (a) exits
// successfully, (b) makes exactly one outbound request, and (c) targets the
// expected resource path and HTTP verb. It is the "does the CLI still call the
// right emass_client endpoint" regression, mirroring the mock-server approach
// in `parity_capture.rs`.
//
// The POC forces `https` only when the host lacks a scheme; feeding it a plain
// `http://127.0.0.1:PORT` keeps the transport TLS-free so a std::net server can
// capture the request. No mTLS client cert is configured, so the POC skips the
// client-cert step and connects over plain HTTP.

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

/// A parsed HTTP request captured from the POC binary.
struct Captured {
    method: String,
    path: String,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

/// Path portion of the captured request line, query string stripped.
fn path_only(cap: &Captured) -> &str {
    cap.path.split('?').next().unwrap_or("")
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

/// Run the POC binary once against a throwaway server.
///
/// Returns the captured request plus whether the process exited successfully
/// (exit code 0). The process is fully drained via `wait()` so a nonzero exit
/// is caught rather than masked.
fn capture(command: &[&str]) -> (Captured, bool) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let addr = listener.local_addr().expect("local addr");

    let mut child = Command::new(poc_bin())
        .args(command)
        .env("EMASSER_HOST_URL", format!("http://{}", addr))
        .env("EMASSER_API_KEY", "test-key")
        .env("EMASSER_USER_UID", "123")
        .env("EMASSER_VERIFY_SSL", "false")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn emasser");

    let (mut sock, _peer) = listener.accept().expect("accept");
    let captured = read_request(&mut sock).expect("read request");
    respond(&mut sock, r#"{"data":{"status":"ok","mockType":"stable"}}"#);
    let exited_ok = child.wait().map(|s| s.success()).unwrap_or(false);
    (captured, exited_ok)
}

/// Assert the captured request exits cleanly, hits `expected_path` (query
/// stripped), and uses `expected_method`.
fn assert_request(cap: &Captured, exited_ok: bool, expected_method: &str, expected_path: &str) {
    assert!(
        exited_ok,
        "emasser exited nonzero; request was {} {}",
        cap.method, cap.path
    );
    assert_eq!(
        cap.method, expected_method,
        "unexpected method for {}",
        cap.path
    );
    assert_eq!(
        path_only(cap),
        expected_path,
        "unexpected path: {} (want {})",
        cap.path,
        expected_path
    );
    // Body-bearing requests carry a JSON content type; GET requests have none.
    if !cap.body.is_empty() {
        assert_eq!(
            cap.headers.get("content-type").map(String::as_str),
            Some("application/json"),
            "missing/incorrect Content-Type for {}",
            cap.path
        );
    }
}

#[test]
fn test_connection_gets_root() {
    let (cap, ok) = capture(&["get", "test", "connection"]);
    assert_request(&cap, ok, "GET", "/api");
}

#[test]
fn get_systems_all() {
    let (cap, ok) = capture(&["get", "systems", "all"]);
    assert_request(&cap, ok, "GET", "/api/systems");
}

#[test]
fn get_system_by_id() {
    let (cap, ok) = capture(&["get", "systems", "byId", "--systemId", "1"]);
    assert_request(&cap, ok, "GET", "/api/systems/1");
}

#[test]
fn get_roles_all() {
    let (cap, ok) = capture(&["get", "roles", "all"]);
    assert_request(&cap, ok, "GET", "/api/system-roles");
}

#[test]
fn get_roles_by_category() {
    let (cap, ok) = capture(&[
        "get",
        "roles",
        "byCategory",
        "--roleCategory",
        "PAC",
        "--role",
        "SCA",
    ]);
    assert_request(&cap, ok, "GET", "/api/system-roles/PAC");
}

#[test]
fn get_poams_for_system() {
    let (cap, ok) = capture(&["get", "poams", "forSystem", "--systemId", "1"]);
    assert_request(&cap, ok, "GET", "/api/systems/1/poams");
}

#[test]
fn post_poams_add() {
    let (cap, ok) = capture(&[
        "post",
        "poams",
        "add",
        "--systemId",
        "1",
        "--poamId",
        "1",
        "--poamCategory",
        "Control",
        "--poamName",
        "TestPoam",
        "--poamScore",
        "100",
    ]);
    assert_request(&cap, ok, "POST", "/api/systems/1/poams");
    // Content-Length must match the emitted body.
    let cl: usize = cap
        .headers
        .get("content-length")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    assert_eq!(cl, cap.body.len(), "content-length must match body bytes");
}

#[test]
fn put_controls_update() {
    let (cap, ok) = capture(&[
        "put",
        "controls",
        "update",
        "--systemId",
        "1",
        "--controlId",
        "100",
        "--implementationStatus",
        "Implemented",
        "--responsibleEntities",
        "Admin",
        "--acronym",
        "XYZ",
        "--slcmCriticality",
        "High",
        "--slcmFrequency",
        "Weekly",
        "--slcmMethod",
        "Manual",
        "--slcmReporting",
        "None",
        "--slcmTracking",
        "None",
        "--slcmComments",
        "Note",
    ]);
    assert_request(&cap, ok, "PUT", "/api/systems/1/controls");
}

#[test]
fn put_poams_update() {
    let (cap, ok) = capture(&[
        "put",
        "poams",
        "update",
        "--systemId",
        "1",
        "--poamId",
        "1",
        "--poamName",
        "Updated",
        "--poamScore",
        "100",
    ]);
    assert_request(&cap, ok, "PUT", "/api/systems/1/poams");
}

#[test]
fn delete_poams_remove() {
    let (cap, ok) = capture(&[
        "delete",
        "poams",
        "remove",
        "--systemId",
        "1",
        "--poamId",
        "1",
    ]);
    assert_request(&cap, ok, "DELETE", "/api/systems/1/poams");
}

#[test]
fn post_scan_clear() {
    let (cap, ok) = capture(&[
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
    assert_request(&cap, ok, "POST", "/api/systems/1/static-code-scans");
}

#[test]
fn post_scan_static_code() {
    let findings_arg = r#"[{"rawSeverity":"critical","codeCheckName":"CSP001","count":3,"scanDate":1700000001,"cweId":"CWE-79"}]"#;
    let (cap, ok) = capture(&[
        "post",
        "scan",
        "add",
        "static",
        "code",
        "--systemId",
        "1",
        "--name",
        "Inventory",
        "--version",
        "2.0",
        "--findings",
        findings_arg,
    ]);
    assert_request(&cap, ok, "POST", "/api/systems/1/static-code-scans");
}
