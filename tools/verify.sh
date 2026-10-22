#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Device-independent local build validation.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
echo 'Workspace build and synthetic tests passed; this is not a device boot test.'
