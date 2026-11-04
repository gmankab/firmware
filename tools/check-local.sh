#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# GitHub Actions are intentionally not used. All gates run locally/offline.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    git diff --check
    git diff --cached --check
fi
if git ls-files | grep -Ei '(^|/)[^/]*agent[^/]*($|/)' >/dev/null; then
    echo 'ERROR: contributor-specific tracked filename detected' >&2
    exit 1
fi
python3 -m tools.firmware status --check
./tools/low-memory-build.sh build
TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target}"
if [[ "$TARGET_DIR" != /* ]]; then
    TARGET_DIR="$PWD/$TARGET_DIR"
fi
FIRMWARE_TOOLS_BIN="$TARGET_DIR/debug/firmware-tools" FIRMWARE_TOOLS_INTEGRATION=1 \
    python3 -m unittest discover -s tests -p 'test_*.py'
if [[ "${1:-}" == '--full' ]]; then
    ./tools/low-memory-build.sh test
    ./tools/low-memory-build.sh release
    python3 -m tools.firmware verify-reproducible
fi
printf '%s\n' 'Local quality gate passed.'
