#!/usr/bin/env bash
#
# regen.sh — repo-local regeneration of the emass_client crate.
#
# Unlike regen_client.sh (which diffs against STOCK openapi-generator 7.12.0
# output and will always report the ~53 divergent files introduced by the
# custom template), this script regenerates with the COMMITTED custom template
# (references/pipeline_inputs/ct) plus the committed patched generator class
# (references/pipeline_inputs/RustClientCodegen.java, recompiled at run time).
#
# The 'verify' subcommand (the default) regenerates into a temp dir and asserts
# that the output is BYTE-FOR-BYTE identical to the committed client/src
# (196/196 files). Any drift = failure. This is the real drift guard: it catches
# decay in the template or the generator patch, and proves regen is 0-divergence
# reproducible. See REGEN_NOTES.md for the customization-recovery spec.
#
# Locations are relative to this script (REPO_ROOT), so it works from any
# checkout. Toolchain is discovered in this order: caller env (CI sets
# JAVA_HOME + OGCJAR), then the local ~/.local/toolchain, then generic candidates.
#
# Usage:
#   source .local/toolchain/env.sh  # optional: pins JAVA_HOME / OGCJAR locally
#   ./regen.sh verify               # regenerate + assert 196/196 parity (default)
#   ./regen.sh regen <spec> <out>   # regenerate <spec> into <out> (no assertion)
#
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ---------------------------------------------------------------------------
# Toolchain discovery (honor caller env first — CI sets JAVA_HOME/OGCJAR).
# ---------------------------------------------------------------------------
ENV_SH=""
for cand in "$HOME/.local/toolchain/env.sh" "$REPO_ROOT/.local/toolchain/env.sh"; do
  if [[ -n "${JAVA_HOME:-}" || -n "${OGCJAR:-}" ]]; then break; fi
  if [[ -f "$cand" ]]; then ENV_SH="$cand"; break; fi
done
[[ -n "$ENV_SH" ]] && source "$ENV_SH" 2>/dev/null || true

# JDK: prefer JAVA_HOME, else candidate dirs on this machine.
if [[ -z "${JAVA_HOME:-}" || ! -x "${JAVA_HOME}/bin/java" ]]; then
  for d in "$HOME/.local/toolchain" "$REPO_ROOT/.local/toolchain" /usr/lib/jvm; do
    if [[ -d "$d" && -x "$d/jdk-21.0.13+9/bin/java" ]]; then
      export JAVA_HOME="$d/jdk-21.0.13+9"; break
    fi
  done
fi
if [[ -z "${JAVA_HOME:-}" || ! -x "${JAVA_HOME}/bin/java" ]]; then
  if command -v java >/dev/null 2>&1; then export JAVA_HOME="$(dirname "$(dirname "$(command -v java)")")"; fi
fi
if [[ -z "${JAVA_HOME:-}" || ! -x "${JAVA_HOME}/bin/java" ]]; then
  echo "ERROR: Temurin 21 JDK not found (set JAVA_HOME or install to ~/.local/toolchain)" >&2; exit 2
fi

# Generator jar: prefer OGCJAR (CI), else candidate paths.
if [[ -z "${OGCJAR:-}" || ! -f "${OGCJAR}" ]]; then
  for cand in "$REPO_ROOT/openapi-generator-cli-7.12.0.jar" \
              "$HOME/.local/toolchain/openapi-generator-cli-7.12.0.jar" \
              "$REPO_ROOT/.local/toolchain/openapi-generator-cli-7.12.0.jar"; do
    if [[ -f "$cand" ]]; then OGCJAR="$cand"; break; fi
  done
fi
if [[ -z "${OGCJAR:-}" || ! -f "${OGCJAR}" ]]; then
  echo "ERROR: openapi-generator-cli-7.12.0.jar not found (set OGCJAR or download it)" >&2; exit 2
fi

SPEC="${SPEC:-$REPO_ROOT/spec/eMASSRestOpenApi.yaml}"
PIN_DIR="$REPO_ROOT/references/pipeline_inputs"
NEW_CLASSES="$PIN_DIR/new_classes"                 # AbstractRustCodegen (on classpath)
PATCH_SRC="$PIN_DIR/RustClientCodegen.java"        # patched generator, recompiled here
TEMPLATE_DIR="$PIN_DIR/ct"                         # custom 7.12.0 mustache template

# ---------------------------------------------------------------------------
# Core regen: recompile the patch -> generate -> return <out>/src
# ---------------------------------------------------------------------------
regen_into() {
  local spec="$1" out="$2"
  if [[ ! -f "$PATCH_SRC" ]]; then
    echo "ERROR: patched generator $PATCH_SRC not found" >&2; return 2
  fi
  if [[ ! -f "$TEMPLATE_DIR/lib.mustache" ]]; then
    echo "ERROR: custom template $TEMPLATE_DIR/lib.mustache not found" >&2; return 2
  fi
  local classes_dir
  classes_dir="$(mktemp -d "${TMPDIR:-/tmp}/emass_classes.XXXXXX")"
  # Recompile the staged patch (proves source -> class -> gen reproducibility).
  if ! "$JAVA_HOME/bin/javac" -proc:none -cp "${NEW_CLASSES}:${OGCJAR}" -d "$classes_dir" "$PATCH_SRC" >"$classes_dir/compile.log" 2>&1; then
    echo "ERROR: javac failed (see $classes_dir/compile.log)" >&2; cat "$classes_dir/compile.log" >&2; rm -rf "$classes_dir"; return 2
  fi
  # Generate from the patched class dirs + custom template.
  if ! "$JAVA_HOME/bin/java" -cp "${NEW_CLASSES}:${classes_dir}:${OGCJAR}" \
        org.openapitools.codegen.OpenAPIGenerator generate \
        --input-spec "$spec" --generator-name rust -o "$out" \
        --template-dir "$TEMPLATE_DIR" -p packageName=emass_client >"$out/generate.log" 2>&1; then
    echo "ERROR: openapi-generator failed (see $out/generate.log)" >&2; rm -rf "$classes_dir"; return 1
  fi
  rm -rf "$classes_dir"
  echo "$out/src"
}

# ---------------------------------------------------------------------------
# verify: regenerate + assert 196/196 byte parity against committed client/src.
# ---------------------------------------------------------------------------
do_verify() {
  if [[ ! -f "$SPEC" ]]; then
    echo "ERROR: spec not found at $SPEC" >&2; return 3
  fi
  local out; out="$(mktemp -d "${TMPDIR:-/tmp}/emass_verify.XXXXXX")"
  local gen_src
  if ! gen_src="$(regen_into "$SPEC" "$out")"; then echo "regen: FAIL" >&2; return $?; fi

  local n_total n_differ
  n_total="$(find "$REPO_ROOT/client/src" -type f | wc -l | tr -d ' ')"
  n_differ="$(diff -rq "$REPO_ROOT/client/src" "$gen_src" 2>/dev/null | grep -c 'differ' || true)"

  if [[ "$n_differ" -eq 0 ]]; then
    echo "verify: PASS — $n_total / $n_total src files byte-identical to committed client/src"
    rm -rf "$out"; return 0
  else
    echo "verify: FAIL — $n_differ / $n_total src files differ from committed client/src"
    echo "The committed client is stale vs the custom template + patched class."
    echo "Re-run 'bash regen.sh regen \"$SPEC\" <out>' then stage client/."
    echo
    diff -rq "$REPO_ROOT/client/src" "$gen_src" | head -n 40
    rm -rf "$out"; return 1
  fi
}

# ---------------------------------------------------------------------------
# dispatch
# ---------------------------------------------------------------------------
case "${1:-verify}" in
  verify) do_verify ;;
  regen)
    [[ $# -ge 3 ]] || { echo "usage: regen.sh regen <input-spec> <out-dir>" >&2; exit 2; }
    gen="$(regen_into "$2" "$3")" || exit $?
    echo "regen: wrote to $gen"
    echo "NOTE: this used the custom template. Stage client/ if you want the committed crate updated."
    ;;
  --help|-h|help)
    sed -n '2,25p' "${BASH_SOURCE[0]}"
    ;;
  *)
    echo "unknown subcommand: $1 (use: verify | regen <spec> <out> | help)" >&2; exit 2 ;;
esac
