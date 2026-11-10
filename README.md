# emass_client_rust

Rust client for the [MITRE eMASS (EaaS) REST API](https://github.com/mitre/emass_client), regenerated from MITRE's `eMASSRestOpenApi.yaml` OpenAPI spec. This repo supplies the **Rust leg** of MITRE's client library that the upstream project does not ship natively — MITRE generates the client in **Ruby, TypeScript, and Python**, but not Rust.

## What's here

The repo is a two-crate Rust workspace:

| Crate | What it is |
| --- | --- |
| [`client/`](client/) | **`emass_client`** (`v3.22.0`) — the generated eMASS REST API client. This crate is **generated, not hand-maintained**: it is reproduced from the spec through a patched [`openapi-generator`](https://github.com/OpenAPITools/openapi-generator) 7.12.0 pipeline (see [Codegen](#codegen--regeneration)). |
| [`emasser`](Cargo.toml) | The hand-authored **`emasser`** binary + crate (`src/`, `tests/`) — the CLI front-end that drives `emass_client`. |

The regeneration tooling (patched generator + custom template + staged class dirs) lives in [`references/pipeline_inputs/`](references/pipeline_inputs/).

## Quick Start

| Step | Command |
| --- | --- |
| Clone & build | `git clone <repo> emass_client_rust && cd emass_client_rust` |
| Test (parity + wire-capture) | `cargo test --workspace` |
| Build the binary | `cargo build` → run `./target/debug/emasser` |

End-to-end CLI wrapper (loads a local `.env` into the process environment, then runs the binary):

```bash
./run_emass.sh <subcommand>   # e.g. ./run_emass.sh scan clear
```

## Layout

| Path | Description |
| --- | --- |
| `client/` | Vendored `emass_client` crate (generated: `src/`, `Cargo.toml`, `README.md`). |
| `src/` | Hand-authored top-level crate: the `emasser` CLI, wire-format converters, and `emass` parity tests. |
| `tests/` | Hand-authored integration tests: byte-parity and full wire-body capture. |
| `spec/` | The eMASS OpenAPI spec (`eMASSRestOpenApi.yaml`) that drives codegen. |
| `references/pipeline_inputs/` | The codegen recovery kit — patched generator, custom template, and staged class dirs. See [Codegen](#codegen--regeneration). |

## Codegen & regeneration

The committed `client/` crate is the **output** of a custom `openapi-generator` 7.12.0 Rust pipeline (patched `RustClientCodegen` generator + custom mustache templates) — **not** stock template output. To keep it bit-exact after any spec or template change, run the drift guard:

```bash
source ~/.local/toolchain/env.sh   # sets JAVA_HOME + OGCJAR
./regen.sh verify
```

`regen.sh verify` regenerates `client/` into a temp dir and **fails if any file diverges** from the committed crate — this is the guard that proves regeneration is zero-divergence (parity). To instead see how the spec differs from stock generator output, `./regen_client.sh summary` prints the divergent set (the custom-template additions).

> Regeneration notes: [`REGEN_NOTES.md`](REGEN_NOTES.md).

## Continuous Integration

- **[`ci.yml`](.github/workflows/ci.yml)** — builds and lints (`clippy -D warnings` over the whole workspace; `rustfmt` on the hand-authored crate only) and runs the parity + wire-body test suite on `stable` and `beta`. The generated `client/` is exempt from lint/fmt by construction.
- **[`codegen.yml`](.github/workflows/codegen.yml)** — on spec change, nightly, and on dispatch, regenerates with the pinned toolchain and asserts 196/196 parity, failing the build on drift.

## Fit with MITRE's work

MITRE's client library (`mitre/emass_client`) is generated for **Ruby** (gem `emass_client` — the dependency of the `mitre/emasser` CLI), **TypeScript** (`@mitre/emass_client`, used by the SAF CLI), and **Python** — Rust is not among them. `mitre/emasser` is MITRE's official **Ruby** CLI: a thin business-logic wrapper around the REST API, not the API layer itself.

This repo reproduces the missing **Rust** leg from the same upstream `eMASSRestOpenApi.yaml` through the patched pipeline — role-for-role, the Rust equivalent of the Ruby gem. It keeps the upstream contract intact: crate `emass_client` (`v3.22.0`) and the `emasser` binary name.

## License

The generated `client/` crate is MIT-licensed (inherited from the upstream codegen conventions). See [`LICENSE`](LICENSE).

## Credits

- [MITRE eMASS (EaaS) REST API](https://github.com/mitre/emass_client) — the spec this client is generated from.
- [MITRE emasser](https://github.com/mitre/emasser) — reference Ruby CLI.
- [openapi-generator](https://github.com/OpenAPITools/openapi-generator) — the generator behind the custom pipeline.
