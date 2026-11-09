# `emass_client` Codegen — Regeneration Notes

The `emass_client` crate (`client/`) is generated from
`spec/eMASSRestOpenApi.yaml` via `openapi-generator` (`-g rust`). The crate is
built with a **custom openapi-generator `rust` template** carrying several
customizations above stock output. Regeneration is a **recovery of the committed
template** — not a fresh build — and is **byte-for-byte reproducible**: the
committed `client/src` matches the regen output on **196 / 196** files.

Two pipelines touch this crate; do not confuse them:

- **`regen_client.sh`** — the stock drift guard. It regenerates from **stock**
  7.12.0 options (no custom template) into `/tmp`, then diffs against
  `client/`. Because it omits the custom template + patches, it shows the
  *structural* divergence below. See "Baseline drift".
- **`./regen.sh`** (repo-local — see "Repo-local regen") — the real,
  self-contained reproducible pipeline. It compiles the patched generator
  classes, drives the **committed custom `ct/` template**, and reproduces
  `client/src` on **196 / 196** files. It is the one to use to rebuild the
  crate, and its `verify` subcommand asserts 196 / 196 parity — this is the CI
  drift guard. The skill copy
  (`~/.hermes/skills/emass-client-codegen-recovery/scripts/regen.sh`) is a
  byte-identical mirror read from the skill's `references/`; the repo-local copy
  is the canonical entrypoint.

## Toolchain (pinned, no sudo)

| Item | Value |
| --- | --- |
| Generator | `openapi-generator-cli` **7.12.0** |
| Java | Temurin **JDK 21** (`~/.local/toolchain/jdk-21.0.13+9`) |
| Real regen script | `./regen.sh` (repo-local) — patched classes + `ct/` template |
| Stock drift script | `regen_client.sh` — stock 7.12.0, drift guard only |
| Env | `~/.local/toolchain/env.sh` (`JAVA_HOME`, `OGCJAR`, `PATH`) |
| Custom template (committed) | `references/pipeline_inputs/ct/` (repo) **and** skill `references/pipeline_inputs/ct/` |
| Regen input spec (tracked) | `spec/eMASSRestOpenApi.yaml` |

## Repo-local regen (`./regen.sh`)

The repo ships its own entrypoint, so regeneration is self-contained — no
external skill path required. Paths resolve from the script location.

```bash
# Default subcommand: regenerate + assert 196/196 parity (the drift guard).
./regen.sh verify

# Rebuild only (manual regen into <out>), then stage client/:
./regen.sh regen spec/eMASSRestOpenApi.yaml /tmp/emass_regen
```

`./regen.sh` is repo-relative (toolchain auto-discovered: caller env →
`~/.local/toolchain` → generic candidates), so it runs from any checkout. It
recompiles the staged `RustClientCodegen.java` at run time — into a temp dir,
never mutating `references/` — and drives the custom `ct/` template, reproducing
`client/src` on **196 / 196** files. `verify` diffs the regen output against the
committed `client/src` and **fails on any difference**.

The stock `regen_client.sh` is separate: it uses **stock** 7.12.0 output (no
custom template) and always reports ~53 structural differ — expected, not a
failure (see "Baseline drift").

## Regeneration procedure (reproducible, non-destructive)

```bash
source ~/.local/toolchain/env.sh        # optional; ./regen.sh auto-discovers toolchain
./regen.sh verify                       # regenerate + assert 196/196 parity
```

After editing the template and staging `client/`, `verify` reproduces
**196 / 196**.

### Sanity check after editing the template

Because the template is the single source of `client/src`, any hand edit to
`ct/lib.mustache` **must** be mirrored to both copies
(`references/pipeline_inputs/ct/lib.mustache` and the skill copy) and the regen
re-run. `./regen.sh verify` enforces this automatically — it fails if the
committed `client/src` drifts from the custom-template regen (expect
**196 / 196, 0 differ**; `lib.rs` already ships the clippy allows from the
template, so it is byte-identical too).
```

## Baseline drift (stock vs committed)

With **stock** options and **no** template/patches (what `regen_client.sh` does):
- **~143 / 196** committed `client/src` files are byte-identical.
- The rest differ **structurally** — exactly the files touched by the 5
  customizations below (most `apis/*.rs`, `configuration.rs`, the
  `static_code_scans_api.rs` `Vec` signature, etc.).

This drift is expected and **not** a problem: it measures stock output against
the custom template output. Use `regen.sh` (which carries the template) for a
faithful rebuild — that path reproduces 196 / 196.

## Customizations (recovered by `regen.sh`)

1. **`Accept` header.** Every request build ends with
   `.header("Accept", "application/json").build()?` (stock emits `.build()?`).
   One occurrence per API file.

2. **Dedicated security-scheme fields.** The emass `user-uid` / `Prefer`
   security schemes map to dedicated `Configuration` fields, not stock
   `ApiKey`:
   - `Configuration` gains `user_uid: Option<String>` and `prefer: Option<String>`.
   - API files set headers directly (replacing the stock api-key-`Prefer` block).

3. **Scoped local-var naming.** Locals are prefixed by scope, so
   `p_system_id` → `p_path_system_id`, `p_coams_id` → `p_query_coams_id`,
   body stays `p_body_...`. Scope is determined by usage (path/query/body).

4. **`configuration.rs` extras.** Custom `ApiKey` `Debug` impl that redacts the
   credential, a custom `Configuration` `Debug` impl that omits
   `secret_key` / `user_uid`, and `pub use reqwest::Client;` re-export.

5. **`static_code_scans_api.rs` `Vec` signature.**
   `add_static_code_scans_by_system_id(&self, system_id, static_code: Vec<StaticCodeRequestPostBody>)`;
   serde serializes the `Vec` as the required single-element array
   `[ {...} ]`. (Covered by `tests/byte_parity.rs` / `tests/parity_capture.rs`.)

## Generated-code lint suppression (clippy)

The `rust` template emits two idioms that reappear on **every** regen
(`clippy::needless_return` × ~209, `clippy::derivable_impls` × ~53 — all inside
the generated `client/`). These are template-level, so they are suppressed at
the crate root in `ct/lib.mustache`, which mirrors into
`client/src/lib.rs`:

```rust
#![allow(clippy::needless_return)]
#![allow(clippy::derivable_impls)]
```

This lets `cargo clippy --workspace --all-targets -- -D warnings` pass for the
**whole** two-crate workspace (both the generated `client/` and the
hand-authored `emasser` bin), rather than excluding `client/` from the gate.

## CI gates (stable-Rust compatible)

The generator emits idioms on 100% of the generated code, so the CI gates are
scoped to keep the generated crate excluded **by construction**, not by hand:

- **Clippy:** `cargo clippy --workspace --all-targets -- -D warnings` — passes
  because `client/src/lib.rs` carries the crate-level allows above.
- **rustfmt:** `cargo fmt -p emasser -- --check` — scopes to the hand-authored
  package (`src/` + `tests/`). `client/` is generated and must NOT be
  reformatted; on stable the rustfmt  `ignore` list is a nightly-only feature,
  so scoping is done via `-p emasser` (see `rustfmt.toml`).

- **Drift guard (`codegen.yml`):** `./regen.sh verify` regenerates from the
  committed custom template + patched class and asserts 196 / 196 parity. A
  template or patch that drifts out of sync with `client/src` **fails the job**,
  forcing a regen + recovery. A follow-up step regenerates from **stock** 7.12.0
  for the informational baseline-drift list (see "Baseline drift").

## Build/run rule

The client defaults to `native-tls`; the standing rule requires
`--no-default-features --features rustls` on every build/run (no system
OpenSSL). Enforced at the workspace level — do not build the client in
isolation with its default feature set.
