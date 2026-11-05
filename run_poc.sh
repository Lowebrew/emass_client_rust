#!/bin/bash
# Wrapper to run the Rust POC emass binary with env loaded from .env.
set -a; [ -f .env ] && set -a && source .env; set +a
exec ./target/debug/emasser "$@"
