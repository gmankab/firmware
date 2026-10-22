#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
FW=${1:?Usage: scripts/inspect-all.sh ORIGINAL_FIRMWARE_ROOT OUTPUT_DIR}
OUT=${2:?Output directory required}
mkdir -p "$OUT"
find "$FW" -type f -print0 | while IFS= read -r -d '' f; do
  "$ROOT/target/release/nabu-firmware-research" inspect-image "$f" > "$OUT/$(basename "$f").txt"
done
