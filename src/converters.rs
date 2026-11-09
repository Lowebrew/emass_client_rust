//! Output formatting ported from `emasser/lib/emasser/output_converters.rb`.
//!
//! Responsibilities, mirroring the reference `to_output_hash`:
//!   1. If the eMASS response is an error envelope (`Error message` +
//!      `Response body`), unwrap it and pretty-print the inner body.
//!   2. **Always** compact null elements out of every array (the reference model
//!      `to_body`/`to_hash` does this unconditionally, so `[null]` -> `[]`).
//!   3. When `display_nulls=false`, additionally strip null-valued keys from
//!      objects under `data` (the reference cleaning loop) and re-emit only
//!      meta/data/pagination.
//!   4. When `display_datetime=true`, convert epoch-second timestamps to
//!      ISO-8601 under recognizable time keys.
//!
//!   5. Finally, pretty-print the value with a printer that reproduces the reference
//!      `JSON.pretty_generate` byte-for-byte, including the empty-array
//!      `[\n\n  ]` and empty-object `{\n  }` rendering that
//!      `serde_json` cannot produce.

use serde_json::{Map, Value};
use time::format_description::well_known::Rfc3339;

/// Format an API response for terminal output.
pub fn to_output_hash(value: &Value, display_nulls: bool, display_dt: bool) -> Value {
    // 1. Unwrap an eMASS error envelope: { "Error message": "...", "Response body": "{...}" }
    let mut out = unwrap_error_envelope(value);

    // 2. to_body compaction: remove null elements from every array (always).
    out = compact_arrays(&out);

    // 3. display_nulls=false cleaning loop.
    if !display_nulls {
        out = clean_data(&out);
    }

    // 4. display_datetime conversion.
    if display_dt {
        out = convert_datetime(&out);
    }

    out
}

/// Reproduce the reference `JSON.pretty_generate` exactly, including the empty-array
/// `[\n\n  ]` (blank line inside) and empty-object `{\n  }` rendering that
/// `serde_json::to_string_pretty` cannot emit.
pub fn pretty_generate(value: &Value) -> String {
    let mut buf = String::new();
    pretty_value(value, 0, &mut buf);
    buf
}

fn pretty_value(value: &Value, level: usize, buf: &mut String) {
    match value {
        Value::Object(map) => {
            if map.is_empty() {
                // reference: empty object -> "{\n" + pad(level) + "}"
                buf.push_str("{\n");
                pad(level, buf);
                buf.push('}');
                return;
            }
            buf.push_str("{\n");
            let entries: Vec<(&String, &Value)> = map.iter().collect();
            for (i, (k, v)) in entries.iter().enumerate() {
                pad(level + 1, buf);
                buf.push_str(&serde_json::to_string(k).unwrap()); // JSON-escaped key
                buf.push_str(": ");
                pretty_value(v, level + 1, buf);
                if i + 1 < entries.len() {
                    buf.push_str(",\n");
                } else {
                    buf.push('\n');
                }
            }
            pad(level, buf);
            buf.push('}');
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                // reference: empty array -> "[\n\n" + pad(level) + "]"
                buf.push_str("[\n");
                buf.push('\n');
                pad(level, buf);
                buf.push(']');
                return;
            }
            buf.push_str("[\n");
            for (i, v) in arr.iter().enumerate() {
                pad(level + 1, buf);
                pretty_value(v, level + 1, buf);
                if i + 1 < arr.len() {
                    buf.push_str(",\n");
                } else {
                    buf.push('\n');
                }
            }
            pad(level, buf);
            buf.push(']');
        }
        // Scalars (null / bool / number / string) — match serde_json exactly.
        other => {
            buf.push_str(&serde_json::to_string(other).unwrap());
        }
    }
}

fn pad(level: usize, buf: &mut String) {
    for _ in 0..level {
        buf.push_str("  ");
    }
}

/// Remove `null` elements from every array, recursively. Mirrors the reference
/// model's `to_hash`/`_to_hash`, which does `value.compact.map` on arrays.
fn compact_arrays(value: &Value) -> Value {
    match value {
        Value::Array(arr) => {
            let filtered: Vec<Value> = arr
                .iter()
                .filter(|v| !v.is_null())
                .map(compact_arrays)
                .collect();
            Value::Array(filtered)
        }
        Value::Object(map) => {
            let out: Map<String, Value> = map
                .iter()
                .map(|(k, v)| (k.clone(), compact_arrays(v)))
                .collect();
            Value::Object(out)
        }
        _ => value.clone(),
    }
}

/// display_nulls=false cleaning loop. reference re-emits only meta/data/pagination
/// keys; under `data` it compacts each object element (dropping null-valued
/// keys) and drops empty/null results.
fn clean_data(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out: Map<String, Value> = Map::new();
            for (k, v) in map.iter() {
                let lk = k.to_lowercase();
                // Only the `data` key needs per-element cleaning; everything
                // else (meta, pagination, and any other key) is copied as-is.
                if lk.contains("data") {
                    out.insert(k.clone(), clean_data_value(v));
                } else {
                    out.insert(k.clone(), v.clone());
                }
            }
            Value::Object(out)
        }
        other => other.clone(),
    }
}

/// Apply `elements.compact` (one level) to each element of a `data` array.
fn clean_data_value(v: &Value) -> Value {
    if let Some(arr) = v.as_array() {
        let cleaned: Vec<Value> = arr
            .iter()
            .map(compact_object)
            .filter(|c| !c.is_null() && !is_empty_object(c))
            .collect();
        Value::Array(cleaned)
    } else {
        // Non-array data: reference does `value.compact` (object one-level) or nil.
        compact_object(v)
    }
}

/// One-level Hash#compact: drop keys whose value is null (non-recursive).
fn compact_object(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let out: Map<String, Value> = map
                .iter()
                .filter(|(_, val)| !val.is_null())
                .map(|(k, val)| (k.clone(), val.clone()))
                .collect();
            Value::Object(out)
        }
        other => other.clone(),
    }
}

fn is_empty_object(v: &Value) -> bool {
    matches!(v, Value::Object(m) if m.is_empty())
}

fn unwrap_error_envelope(value: &Value) -> Value {
    if let Some(obj) = value.as_object() {
        let is_err = obj
            .keys()
            .any(|k| k.to_lowercase().contains("error message"));
        if is_err {
            if let Some(rb) = obj.get("Response body") {
                if let Some(s) = rb.as_str() {
                    if let Ok(inner) = serde_json::from_str::<Value>(s) {
                        return inner;
                    }
                }
            }
        }
    }
    value.clone()
}

/// Convert epoch-second integers under a recognizable time key to ISO-8601.
/// Mirrors the reference regex `/(DATE|TIMESTAMP|LASTSEEN|TIME|ATD)/`.
fn convert_datetime(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let out: Map<String, Value> = map
                .iter()
                .map(|(k, val)| {
                    let nk = k.clone();
                    let nv = convert_epoch_value(k, val);
                    (nk, nv)
                })
                .collect();
            Value::Object(out)
        }
        Value::Array(arr) => {
            let out: Vec<Value> = arr.iter().map(convert_datetime).collect();
            Value::Array(out)
        }
        other => other.clone(),
    }
}

fn convert_epoch_value(key: &str, val: &Value) -> Value {
    if val.is_array() {
        return convert_datetime(val);
    }
    if is_dt_key(key) {
        if let Some(n) = val.as_i64() {
            if n > 100_000_000 {
                if let Ok(t) = time::OffsetDateTime::from_unix_timestamp(n) {
                    return Value::String(t.format(&Rfc3339).unwrap_or_else(|_| t.to_string()));
                }
            }
        }
    }
    // Recurse into nested objects (e.g. `data` when byId returns a single
    // system object rather than an array).
    match val {
        Value::Object(_) | Value::Array(_) => convert_datetime(val),
        _ => val.clone(),
    }
}

fn is_dt_key(k: &str) -> bool {
    let up = k.to_uppercase();
    ["DATE", "TIMESTAMP", "LASTSEEN", "TIME", "ATD"]
        .iter()
        .any(|p| up.contains(p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compacts_null_array_elements_by_default() {
        // The `[null]` -> `[]` transformation that enables byte parity.
        let v: Value = serde_json::from_str(r#"{"meta":{"code":200},"data":[null]}"#).unwrap();
        let out = to_output_hash(&v, true, false);
        assert_eq!(out, serde_json::json!({"meta":{"code":200},"data":[]}));
    }

    #[test]
    fn empty_array_pretty_print_is_blank_line() {
        let v: Value = serde_json::from_str(r#"{"meta":{"code":200},"data":[]}"#).unwrap();
        let s = pretty_generate(&v);
        // Must match emass's 54-byte output byte-for-byte.
        assert_eq!(
            s,
            "{\n  \"meta\": {\n    \"code\": 200\n  },\n  \"data\": [\n\n  ]\n}"
        );
    }

    #[test]
    fn empty_object_pretty_print_has_newline_indent() {
        let v: Value = serde_json::from_str(r#"{"a":{},"b":[1]}"#).unwrap();
        let s = pretty_generate(&v);
        assert_eq!(s, "{\n  \"a\": {\n  },\n  \"b\": [\n    1\n  ]\n}");
    }

    #[test]
    fn keeps_nulls_when_displayed_but_still_compacts_arrays() {
        // display_nulls=true keeps null-valued object keys, but array
        // compaction (to_body) still happens.
        let v: Value = serde_json::from_str(r#"{"a":null,"arr":[null,1]}"#).unwrap();
        let out = to_output_hash(&v, true, false);
        assert_eq!(out, serde_json::json!({"a":null,"arr":[1]}));
    }

    #[test]
    fn converts_epoch_to_datetime() {
        let v: Value =
            serde_json::from_str(r#"{"systemId":1,"dateLastAudit":1700000000}"#).unwrap();
        let out = to_output_hash(&v, true, true);
        let s = out["dateLastAudit"]
            .as_str()
            .expect("epoch should become a string");
        assert!(s.contains("2023"), "unexpected datetime: {s}");
    }

    #[test]
    fn converts_epoch_in_nested_object_by_id_shape() {
        // Mirrors the byId response: `data` is a single object (not an array),
        // so conversion must recurse into nested objects.
        let v: Value = serde_json::from_str(
            r#" {
            "data": { "acronym": "PM-6", "authorizationDate": 1638741660 },
            "meta": { "code": 200 }
        }"#,
        )
        .unwrap();
        let out = to_output_hash(&v, true, true);
        let s = out["data"]["authorizationDate"]
            .as_str()
            .expect("nested epoch should become a string");
        assert!(s.contains("2021"), "unexpected datetime: {s}");
    }
}
