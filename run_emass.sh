#!/bin/bash
# Wrapper to run the emass_client Rust binary with env loaded from .env.
set -a; [ -f .env ] && set -a && source .env; set +a
exec ./target/debug/emasser "$@"
