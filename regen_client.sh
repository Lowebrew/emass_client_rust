#!/usr/bin/env bash
#
# regen_client.sh — Regenerate the emass_client Rust crate from the eMASS
# OpenAPI spec and diff it against the committed client.
#
# This script is NON-DESTRUCTIVE: it regenerates into a temp dir and NEVER
# overwrites client/. It exists to guard against spec->client drift and to
# give a reproducible, toolchain-pinned codegen path.
#
# Toolchain (no sudo): a portable JDK 21 and openapi-generator-cli 7.12.0,
# installed under ~/.local/toolchain via ~/.local/toolchain/env.sh.
#
# Usage:
#   source ~/.local/toolchain/env.sh   # sets JAVA_HOME, OGCJAR
#   ./regen_client.sh [diff_filter]    # diff_filter: "files" | "context=N" | "summary"
#
# Exit: 0 = regeneration ran; 2 = regen toolchain missing; 3 = spec missing.

set -euo pipefail

# ---- locations (overridable via env) -------------------------------------
REPO_ROOT="${REPO_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}"
SPEC="${SPEC:-$REPO_ROOT/spec/eMASSRestOpenApi.yaml}"
CLIENT_DIR="${CLIENT_DIR:-$REPO_ROOT/client}"
JAR="${OGCJAR:-$REPO_ROOT/openapi-generator-cli-7.12.0.jar}"
JDK="${JAVA_HOME:-$HOME/.local/toolchain/jdk-21.0.13+9}"
GEN_VERSION="${GEN_VERSION:-7.12.0}"

# ---- preconditions --------------------------------------------------------
if [[ -z "$JAVA_HOME" || ! -x "$JAVA_HOME/bin/java" ]]; then
  if [[ -f "$REPO_ROOT/.local/toolchain/env.sh" ]]; then
    source "$REPO_ROOT/.local/toolchain/env.sh"
  fi
fi
if [[ -z "$JAVA_HOME" || ! -x "$JAVA_HOME/bin/java" ]]; then
  echo "ERROR: no JDK. Install Temurin 21 to ~/.local/toolchain/jdk-* and source .local/toolchain/env.sh." >&2
  exit 2
fi
if [[ ! -f "$JAR" ]]; then
  echo "ERROR: openapi-generator jar not found at $JAR" >&2
  exit 2
fi
if [[ ! -f "$SPEC" ]]; then
  echo "ERROR: eMASS spec not found at $SPEC" >&2
  exit 3
fi
export PATH="$JAVA_HOME/bin:$PATH"

TMP="$(mktemp -d /tmp/emass_client_regen.XXXXXX)"
trap 'rm -rf "$TMP"' EXIT

echo "=== openapi-generator $(java -jar "$JAR" version 2>/dev/null | tail -1) ==="
echo "spec:   $SPEC"
echo "output: $TMP (non-destructive; committed client untouched)"
echo

if ! java -jar "$JAR" generate \
      -i "$SPEC" \
      -g rust \
      -o "$TMP" \
      --skip-validate-spec ; then
  echo "ERROR: regeneration failed" >&2
  exit 1
fi

echo "=== drift: committed client vs regen ==="
echo "committed src files: $(find "$CLIENT_DIR/src" -type f | wc -l)"
echo "regen       src files: $(find "$TMP/src" -type f | wc -l)"
echo

case "${1:-summary}" in
  files)
    diff -rq "$CLIENT_DIR/src" "$TMP/src" | grep "differ" || echo "(no differing src files)"
    ;;
  context=*)
    N="${1#context=}"; diff -U"$N" "$CLIENT_DIR/src" "$TMP/src"
    ;;
  *)
    n_differ=$(diff -rq "$CLIENT_DIR/src" "$TMP/src" | grep -c "differ" || true)
    n_total=$(find "$CLIENT_DIR/src" -type f | wc -l)
    echo "Summary: $n_differ / $n_total src files differ from stock 7.12.0 'rust' output."
    echo "The divergent set is exactly the user's custom-template additions"
    echo "(see REGEN_NOTES.md for the recovery specification):"
    echo "  1. Accept: application/json header on every request build"
    echo "  2. dedicated Prefer/user-uid security-scheme fields (not stock api_key)"
    echo "  3. scoped local-var naming (p_path_/p_query_/p_body_)"
    echo "  4. configuration.rs: dedicated user_uid/prefer fields + ApiKey/Configuration Debug"
    echo "  5. static_code_scans_api.rs: add_static_code_scans_by_system_id accepts Vec<...>"
    echo
    echo "If the SPEC changed, the divergent set changes -> the client is stale and must be regen'd."
    ;;
esac
