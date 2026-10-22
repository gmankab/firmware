#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
set -euo pipefail
# Usage: REA_BINWALK_COMMAND=/abs/binwalk scripts/run-rea-scans.sh /path/to/firmware /path/to/rea/scripts/rea.mjs /path/to/output
FW="${1:?firmware directory required}"
REA="${2:?REA scripts/rea.mjs required}"
DEST="${3:?output directory required}"
: "${REA_BINWALK_COMMAND:?export REA_BINWALK_COMMAND as the absolute path to binwalk 3.1.x}"
NODE="${REA_NODE_COMMAND:-$(command -v node)}"
mkdir -p "$DEST"
find "$FW" -type f -print0 | sort -z | while IFS= read -r -d '' target; do
  rel="${target#"$FW"/}"
  name="${rel//\//__}"
  printf 'REA scanning: %s\n' "$rel" >&2
  "$NODE" "$REA" inspect-firmware-regions "$target" --format json > "$DEST/$name.json"
done
