# emasser — Rust POC for the eMASS (EaaS) REST API

A Rust port of [`mitre/emasser`](https://github.com/mitre/emasser), a thin
reference CLI that wraps the [eMASS](https://www.dcsa.mil/is/emass/) (Enterprise
Mission Assurance Support System) Representational State Transfer (REST) API.

This is a **proof-of-concept**. The goal is byte-for-byte parity with the
upstream reference tool for the command surface it supports: identical dispatch,
identical JSON request bodies (including key insertion order and
Integer-vs-Float numeric fidelity), and the same env-driven mutual-TLS +
API-key authentication scheme.

## What this contains

- `src/main.rs` — the `emasser` binary. Supports `get`, `post`, `put`,
  `delete`, and `configure` subcommands.
- `client/` — the eMASS OpenAPI-generated Rust client, **vendored locally**
  (path dependency `emass_client = { path = "client" }`). It is fully
  self-contained; there is no private crates.io dependency. Regenerated from
  the spec in `spec/`.
- `spec/` — the upstream eMASS OpenAPI specification the client is generated
  from (kept for provenance/regeneration reference).
- `run_poc.sh` — convenience wrapper that loads `.env` and execs the built
  binary.
- `examples/` — reserved for example payloads.

## Requirements

- A Rust toolchain (`cargo`). Builds to `target/debug/emasser`.
- **No system OpenSSL required.** The dependency graph is pinned to
  pure-Rust [`rustls`](https://github.com/rustls/rustls) (see `Cargo.toml`),
  so there is nothing to install via the OS package manager.

## Quick start

```bash
cargo build                      # produces ./target/debug/emasser

# Create/refresh a .env from process env (or your shell), mirroring the tool's
# documented .env template. .env is git-ignored and holds only your local creds.
./run_poc.sh configure

# Talk to eMASS. Example:
./run_poc.sh get test connection
```

`run_poc.sh` simply does `set -a; source .env; set +a; exec ./target/debug/emasser "$@"`
so you can also run `./target/debug/emasser <args>` directly.

## Configuration (environment)

Every value comes from the environment — nothing is committed. `emasser configure`
materializes them into a local `.env` (which is git-ignored). See `emasser`'s
built-in help (`emasser` with no args) for the full list. The key ones:

| Variable                  | Purpose                                            |
| ------------------------- | -------------------------------------------------- |
| `EMASSER_HOST_URL`        | eMASS host, e.g. `https://emas.example.gov`        |
| `EMASSER_API_KEY`         | API key / user-uid value                           |
| `EMASSER_CERT_FILE_PATH`  | mTLS client certificate (PEM)                      |
| `EMASSER_KEY_FILE_PATH`   | mTLS private key (PEM)                             |
| `EMASSER_KEY_FILE_PASSWORD` | password for the private key (optional)          |
| `EMASSER_USER_UID`        | user-uid value (needed for post/put/delete)        |
| `EMASSER_PREFER`          | header for the mock, e.g. `code=200, dynamic=true` |
| `EMASSER_VERIFY_SSL`      | set `'false'` to accept self-signed servers        |
| `EMASSER_EPOCH_TO_DATETIME` | convert epoch timestamps to datetime             |

## Security notes

- Credentials and mTLS material live only in your local `.env`, which is
  git-ignored. Never commit `.env`, `*.pem`, `*.key`, or key files.
- The client authenticates with mTLS + an API key; treat those values as
  secrets and rotate them if exposed anywhere.

## Attribution & provenance

This POC ports the MITRE [`emasser`](https://github.com/mitre/emasser) project.
The vendored `client/` crate and `spec/` were generated from the upstream eMASS
OpenAPI specification and carry their original **Apache-2.0** license headers,
which are preserved in `client/`.

## License

The code contributed to this repository is provided under the MIT License (see
`LICENSE`). The vendored `client/` crate remains Apache-2.0, as generated.
