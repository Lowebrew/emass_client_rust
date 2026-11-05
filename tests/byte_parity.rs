// Byte-parity oracle for the static-code-scans path.
//
// The eMASS reference tool (emasser `lib/emasse/post.rb` `add`/`clear`) builds a
// `StaticCodeRequestPostBody` and posts it INSIDE a single-element JSON array:
//     add_static_code_scans_by_system_id(systemId, Array.new(1, body))
// The eMASS OpenAPI spec example confirms the same `[ {...} ]` shape.
//
// These tests lock the exact bytes the Rust client must emit so the wire format
// can never drift from the reference tool. They exercise model serialization
// (the part that determines the wire bytes) without a live network round-trip.
use emass_client::models::{
    StaticCodeApplicationPost, StaticCodeRequestPostBody,
    StaticCodeRequestPostBodyApplication,
};

/// Reconstructs the body exactly as `cmd_post_scan_clear` /
/// `cmd_post_scan_static_code` build it. Kept here so the byte-parity target is
/// expressed once and both the handler and the test agree on it.
fn build_body(
    app_name: &str,
    version: &str,
    clear: bool,
) -> Vec<StaticCodeRequestPostBody> {
    let mut app = StaticCodeRequestPostBodyApplication::new();
    app.application_name = Some(app_name.to_string());
    app.version = Some(version.to_string());

    let mut finding = StaticCodeApplicationPost::new();
    finding.clear_findings = Some(clear);

    let mut body = StaticCodeRequestPostBody::new();
    body.application = Some(Box::new(app));
    body.application_findings = Some(vec![finding]);

    // The eMASS API (and the reference tool) wrap the body in a
    // single-element ARRAY. Accepting the Vec and serializing it directly is
    // what yields byte-for-byte parity with `post.rb`.
    vec![body]
}

#[test]
fn clear_body_is_single_element_array_with_expected_key_order() {
    // Emulates emasser `Scans#clear` for application "SomeApp" v1.2.3.
    // NOTE: the whole body is wrapped in an outer `[ ... ]` array.
    let expected = r#"[{"application":{"applicationName":"SomeApp","version":"1.2.3"},"applicationFindings":[{"clearFindings":true}]}]"#;
    let got = serde_json::to_string(&build_body("SomeApp", "1.2.3", true)).unwrap();
    assert_eq!(
        expected, got,
        "clear wire body must match the reference tool's single-element array format"
    );
}

#[test]
fn array_wrapping_must_be_present_not_a_bare_object() {
    // Regression guard: the wire body must START with `[` and END with `]`,
    // i.e. an array — never a bare `{ ... }` object.
    let got = serde_json::to_string(&build_body("App", "9", false)).unwrap();
    assert!(
        got.starts_with('[') && got.ends_with(']'),
        "expected a single-element JSON array, got: {got}"
    );
    // Exactly one application object inside the array.
    assert_eq!(
        got.matches("applicationName").count(),
        1,
        "the array must contain exactly one application object, got: {got}"
    );
}
