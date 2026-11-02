#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Build the complete Rust workspace within the 4-GiB compilation limit.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
: "${CARGO_BUILD_JOBS:=1}"
: "${CARGO_INCREMENTAL:=0}"
: "${CARGO_PROFILE_DEV_DEBUG:=0}"
: "${CARGO_PROFILE_TEST_DEBUG:=0}"
: "${CARGO_PROFILE_RELEASE_DEBUG:=0}"
: "${CARGO_PROFILE_RELEASE_OPT_LEVEL:=0}"
: "${CARGO_PROFILE_RELEASE_CODEGEN_UNITS:=64}"
: "${RUSTFLAGS:=-Cdebuginfo=0 -Copt-level=0 -Ccodegen-units=64}"
export CARGO_BUILD_JOBS CARGO_INCREMENTAL CARGO_PROFILE_DEV_DEBUG CARGO_PROFILE_TEST_DEBUG
export CARGO_PROFILE_RELEASE_DEBUG CARGO_PROFILE_RELEASE_OPT_LEVEL CARGO_PROFILE_RELEASE_CODEGEN_UNITS RUSTFLAGS
case "${1:-}" in
    check) exec cargo check --workspace --offline --locked --jobs 1 ;;
    build) exec cargo build --workspace --offline --locked --jobs 1 ;;
    test) exec cargo test --workspace --offline --locked --jobs 1 ;;
    release) exec cargo build --workspace --release --offline --locked --jobs 1 ;;
    python) exec python3 -m unittest discover -s tests -p 'test_*.py' ;;
    *) printf '%s\n' 'Usage: tools/low-memory-build.sh check|build|test|release|python' >&2; exit 2 ;;
esac
