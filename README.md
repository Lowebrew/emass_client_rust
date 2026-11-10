# emass_client_rust

Rust client for the [MITRE eMASS (EaaS) REST API](https://github.com/mitre/emass_client), regenerated from MITRE's `eMASSRestOpenApi.yaml` OpenAPI spec. This repo supplies the **Rust leg** of MITRE's client library that the upstream project does not ship natively — MITRE generates the client in **Ruby, TypeScript, and Python**, but not Rust.

> **Scope — generated vs hand-authored.** The generated `client/` crate (`emass_client`) is reproduced from MITRE's spec; its size is generated, not hand-written, and it is not hand-maintained between regenerations. The hand-authored portion of this repo is the thin `emasser` CLI wrapper in `src/` and the parity/wire-body test harness in `tests/`. See [`Codegen & regeneration`](#codegen--regeneration) for how the client is produced.

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
./run_emass.sh <subcommand>   # e.g. ./run_emass.sh get test connection
```

## Usage

`emasser` is a read-focused CLI: every operation is a `get` resource, with `post` / `put` / `delete` verbs available for the write paths. Authentication is **mTLS client certificate + API key**, provided entirely through environment variables — no secret is stored in the code.

### 1. Configure credentials

`configure` writes a `.env` template to the repo root (existing keys are left untouched). Fill in your eMASS material:

```bash
./run_emass.sh configure        # writes .env; ./target/debug/emasser configure also works
```

`.env` keys. The **mTLS cert + private key are a matched pair** — if you set one, set the other too.

| Key | Required | Purpose |
| --- | --- | --- |
| `EMASSER_HOST_URL` | yes | eMASS host, e.g. `https://emas.example.gov` |
| `EMASSER_API_KEY` | yes | API key / user-uid value |
| `EMASSER_CERT_FILE_PATH` | yes | mTLS client certificate (PEM) |
| `EMASSER_KEY_FILE_PATH` | yes | mTLS private key (PEM) |
| `EMASSER_KEY_FILE_PASSWORD` | no | passphrase for the encrypted private key |
| `EMASSER_USER_UID` | no | user-uid (falls back to `123` if unset) |
| `EMASSER_VERIFY_SSL` | no | `false` to accept self-signed servers |
| `EMASSER_PREFER` | no | mock header, e.g. `code=200, dynamic=true` |
| `EMASSER_CLI_DISPLAY_NULL` | no | `false` to strip null fields from output |
| `EMASSER_EPOCH_TO_DATETIME` | no | `true` to convert epoch timestamps |
| `EMASSER_DOWNLOAD_DIR` | no | where `get artifacts export` writes files |

`run_emass.sh` loads `.env` into the process environment automatically before invoking the binary, so you rarely set these by hand.

### 2. Verify connectivity

Always start here — it validates host + credentials before you run real queries:

```bash
./run_emass.sh get test connection
```

### 3. Run queries

Reads read as `get <resource> [subcommand]`:

```bash
# List systems (optionally filtered by CAMS id)
./run_emass.sh get systems all [--coams-id ID]

# Controls in a system
./run_emass.sh get controls forSystem --systemId 12345

# POA&M items in a system
./run_emass.sh get poams forSystem --systemId 12345

# One POA&M item
./run_emass.sh get poams byPoamId --systemId 12345 --poamId 6789

# Download a system's artifact bundle
./run_emass.sh get artifacts export --systemId 12345 --filename artifacts.tar.gz
```

### Command reference

All reads hang off `get`; `post` / `put` / `delete` are the write verbs, and `configure` sets up credentials. Invoke every read as `emasser get <resource> [subcommand] [options]`:

**Systems**
- `test connection` — test the eMASS API connection
- `systems all [--coams-id ID]` — list systems
- `systems byId --systemId <id>` — get a system by id
- `system id [--systemName <name>] [--systemOwner <owner>]` — find system(s) by name/owner
- `system byId --systemId <id>` — get a single system

**Controls & assessments**
- `roles all` — list system roles
- `controls forSystem --systemId <id>` — controls in a system
- `cac controls --systemId <id>` — CAC location of controls
- `pac package --systemId <id>` — PAC location of the system package
- `ccmc assessments --sinceDate <date>` — CMC assessment information
- `dashboards <name> --orgId <id>` — a system dashboard

**POA&M**
- `poams forSystem --systemId <id>` — POA&M items in a system
- `poams byPoamId --systemId <id> --poamId <id>` — one POA&M item
- `milestones byPoamId --systemId <id> --poamId <id>` — milestones for a POA&M
- `milestones byMilestoneId --poamId <id> --milestoneId <id>` — one milestone

**Results, assets & artifacts**
- `testResults forSystem --systemId <id>` — test results in a system
- `hardware assets --systemId <id>` — hardware assets
- `software assets --systemId <id>` — software assets
- `artifacts forSystem --systemId <id>` — artifacts for a system
- `artifacts export --systemId <id> --filename <f>` — download an artifact (`-C` compress, `-o` print to stdout)

**Workflows**
- `workflow_definitions forSite` — all workflow definitions
- `workflow_instances all` — all workflow instances
- `workflow_instances byInstanceId --workflowInstanceId <id>` — one workflow instance

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

Two different licenses apply here — the generated client and the hand-authored wrapper are separate:

- **`client/` crate (`emass_client`)** — **Apache-2.0**, as declared in `client/Cargo.toml`. This is the generated client reproduced from MITRE's upstream spec, so its license is carried from upstream, not set by this repo.
- **`emasser` wrapper and `tests/`** (the hand-authored layer) — **MIT**, per the root [`LICENSE`](LICENSE).

## Credits

- [MITRE eMASS (EaaS) REST API](https://github.com/mitre/emass_client) — the spec this client is generated from.
- [MITRE emasser](https://github.com/mitre/emasser) — reference Ruby CLI.
- [openapi-generator](https://github.com/OpenAPITools/openapi-generator) — the generator behind the custom pipeline.
