# `emass_client` Codegen — Regeneration Notes

The `emass_client` crate (`client/`) is generated from
`spec/eMASSRestOpenApi.yaml` via `openapi-generator` (`-g rust`). Regeneration
is a **recovery**, not a fresh build: the committed client carries several
customizations above stock output, so a raw regen only matches the client on
the *bulk* generated code, not byte-identical. See `regen_client.sh` for the
reproducible, non-destructive regen + drift check.

## Toolchain (pinned, no sudo)

| Item | Value |
| --- | --- |
| Generator | `openapi-generator-cli` **7.12.0** |
| Java | Temurin **JDK 21** (`~/.local/toolchain/jdk-21.0.13+9`) |
| Script | `regen_client.sh` → regenerates into `/tmp`, diffs vs `client/` |
| Env | `~/.local/toolchain/env.sh` (`JAVA_HOME`, `OGCJAR`, `PATH`) |

Regen command (what `regen_client.sh` runs):

```bash
java -jar openapi-generator-cli-7.12.0.jar generate \
  -i spec/eMASSRestOpenApi.yaml -g rust -o /tmp/emass_client_regen \
  --skip-validate-spec
```

## Baseline drift measurement

With **stock** options and no customization:
- **143 / 196** committed `client/src` files are byte-identical.
- **53** files differ — and the divergence is **structural**, not cosmetic.
- The 53 divergent files are exactly those containing HTTP/header logic:
  all 51 `apis/*.rs` + `configuration.rs` + `static_code_scans_api.rs` + 2 models.

Root cause: the committed client was produced from a **custom openapi-generator
`rust` template** (the user's bespoke work). MITRE only ships official
**Ruby** and **TypeScript** clients for `emass_client`; the Rust client is the
user's own, with a template that customizes the emass security schemes. That
template's source was removed by commit `2f8a30a` ("Remove Ruby tooling
artifacts", which deleted the `.openapi-generator` metadata), so byte-identical
regen must *recover* the template's customizations.

## Customization-recovery specification

Recover each to turn a stock 7.12.0 regen into the committed client:

1. **`Accept` header.** Every request build ends with
   `.header("Accept", "application/json").build()?` (stock emits `.build()?`).
   One occurrence per API file.

2. **Dedicated security-scheme fields.** The emass `user-uid` / `Prefer`
   security schemes are mapped to **dedicated** `Configuration` fields, not the
   stock `ApiKey`:
   - `Configuration` gains `user_uid: Option<String>` and `prefer: Option<String>`.
   - API files set headers directly:
     ```rust
     if let Some(ref prefer) = configuration.prefer {
         req_builder = req_builder.header("Prefer", prefer.clone());
     }
     if let Some(ref user_uid) = configuration.user_uid {
         req_builder = req_builder.header("user-uid", user_uid.clone());
     }
     ```
   - This replaces the stock `if let Some(ref apikey) = configuration.api_key {
     ...build `Prefer` from the api key...}` block (present in **every** API
     file).

3. **Scoped local-var naming.** The template prefixes locals by scope, so
   stock `p_system_id` → committed `p_path_system_id`, `p_coams_id` →
   `p_query_coams_id`, `p_body_...` → `p_body_...` (unchanged). Scope is
   determined by usage: path params go in `p_path_`, query params in
   `p_query_`, body in `p_body_`.

4. **`configuration.rs` extras.** Custom `ApiKey` `Debug` impl that redacts the
   credential (`ApiKey { key, prefix }` → prints the prefix, not the key), plus
   a custom `Configuration` `Debug` impl that omits `secret_key` / `user_uid`,
   and `pub use reqwest::Client;` re-export.

5. **`static_code_scans_api.rs` `Vec` signature.**
   `add_static_code_scans_by_system_id(&self, system_id, static_code: Vec<StaticCodeRequestPostBody>)`
   (custom commit `e442205`); serde then serializes the `Vec` as the required
   single-element array `[ {...} ]`.

## Regeneration procedure (non-destructive)

```bash
source ~/.local/toolchain/env.sh
./regen_client.sh                 # temp regen + drift report
./regen_client.sh context=3       # unified diff for review
```

Only after the 5 recoveries above are applied (e.g. via the user's custom
template) will the regen be byte-identical. Until the custom template is
recreated, `regen_client.sh` is a **drift guard**: if the spec changes, the
divergent set changes and the client is stale.

## Build/run rule

The client defaults to `native-tls`; the standing rule requires
`--no-default-features --features rustls` on every build/run (no system
OpenSSL). Enforced at the workspace level — do not build the client in
isolation with its default feature set.
