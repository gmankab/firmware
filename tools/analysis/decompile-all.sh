#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Batch Ghidra inspection for *any* ELF device firmware. Inputs stay untouched.
# Usage: tools/analysis/decompile-all.sh FILE_OR_DIRECTORY GHIDRA_INSTALL OUTPUT_DIRECTORY
set -euo pipefail
if (( $# != 3 )); then
  echo "Usage: $0 ELF_FILE_OR_TREE GHIDRA_INSTALL PRIVATE_OUTPUT_DIRECTORY" >&2
  exit 2
fi
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
INPUT=$(realpath "$1")
GHIDRA=$(realpath "$2")
OUT=$(realpath -m "$3")
[[ -d "$INPUT" || -f "$INPUT" ]] || { echo "Input missing: $INPUT" >&2; exit 2; }
[[ -x "$GHIDRA/support/analyzeHeadless" ]] || { echo 'Ghidra analyzeHeadless missing' >&2; exit 2; }
# Keep proprietary machine-code listings / pseudocode out of the source tree.
case "$OUT/" in "$ROOT/"*) echo 'Output must be outside the repository' >&2; exit 2;; esac
[[ "$OUT" != / ]] || { echo 'Refusing to write to /' >&2; exit 2; }
mkdir -p "$OUT/projects" "$OUT/images"
ANALYSIS_SEC=${ANALYSIS_SEC:-150}
DECOMP_SEC=${DECOMP_SEC:-120}
MAX_FUNCS=${MAX_FUNCS:-5000}
GHIDRA_HEADLESS_MAXMEM=${GHIDRA_HEADLESS_MAXMEM:-2G}
export GHIDRA_HEADLESS_MAXMEM
[[ "$ANALYSIS_SEC" =~ ^[0-9]+$ && "$DECOMP_SEC" =~ ^[0-9]+$ && "$MAX_FUNCS" =~ ^[0-9]+$ ]] || {
  echo 'Timeouts and MAX_FUNCS must be nonnegative integers' >&2; exit 2;
}
if [[ -f "$INPUT" ]]; then
  files=("$INPUT")
else
  mapfile -d '' -t files < <(find "$INPUT" -type f -print0 | sort -z)
fi
(( ${#files[@]} > 0 )) || { echo 'No input files found' >&2; exit 1; }
examined=0
attempted=0
failed=0
for file in "${files[@]}"; do
  ((examined+=1))
  # This workflow is intentionally ELF-only. Raw MCU/DSP blobs need a
  # deliberate architecture, memory mapping and Ghidra BinaryLoader settings.
  [[ $(head -c 4 "$file") == $'\177ELF' ]] || continue
  ((attempted+=1))
  filehash=$(printf %s "$file" | sha256sum | cut -c1-12)
  safe_name=$(basename "$file" | tr -c 'A-Za-z0-9_.-' '_')
  record="$OUT/images/${safe_name}-${filehash}"
  mkdir -p "$record"
  printf '%s\n' "$file" > "$record/source-path.txt"
  sha256sum "$file" > "$record/input.sha256"
  echo "Ghidra ELF import: $file" >&2
  rc=0
  timeout "$((ANALYSIS_SEC+DECOMP_SEC+90))s" \
    "$GHIDRA/support/analyzeHeadless" "$OUT/projects" "analysis_${filehash}" \
    -import "$file" -overwrite \
    -analysisTimeoutPerFile "$ANALYSIS_SEC" \
    -scriptPath "$ROOT/tools/ghidra" \
    -postScript DecompileFirmware.java "$record/functions.c" "$MAX_FUNCS" "$DECOMP_SEC" \
    -max-cpu 2 -deleteProject > "$record/ghidra.log" 2>&1 || rc=$?
  printf '%s\n' "$rc" > "$record/exit-code.txt"
  if (( rc != 0 )); then
    ((failed+=1))
    echo "Warning: incomplete analysis of $file (exit $rc), see $record/ghidra.log" >&2
  fi
done
printf 'ELF batch: examined %d files, attempted %d, subprocess failures %d. Output: %s\n' \
  "$examined" "$attempted" "$failed" "$OUT"
echo 'Note: an exit code of 0 does not prove coverage or instruction-decoding correctness.'
(( attempted > 0 && failed == 0 ))
