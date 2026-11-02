#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Resume function export from a previously generated Ghidra functions.c file.
# Usage: tools/analysis/continue-decompilation.sh ELF_FILE GHIDRA_INSTALL PRIOR_FUNCTIONS_C OUTPUT_DIRECTORY
set -euo pipefail
if (( $# != 4 )); then
  echo "Usage: $0 ELF_FILE GHIDRA_INSTALL PRIOR_FUNCTIONS_C PRIVATE_OUTPUT_DIRECTORY" >&2
  exit 2
fi
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
INPUT=$(realpath "$1")
GHIDRA=$(realpath "$2")
PRIOR=$(realpath "$3")
OUT=$(realpath -m "$4")
[[ -f "$INPUT" && -f "$PRIOR" && -x "$GHIDRA/support/analyzeHeadless" ]] || {
  echo 'Original ELF, earlier decompilation, or Ghidra installation missing' >&2; exit 2;
}
case "$OUT/" in "$ROOT/"*) echo 'Output must be outside the source tree' >&2; exit 2;; esac
mkdir -p "$OUT/projects"
ANALYSIS_SEC=${ANALYSIS_SEC:-300}
DECOMP_SEC=${DECOMP_SEC:-300}
MAX_FUNCS=${MAX_FUNCS:-100000}
GHIDRA_HEADLESS_MAXMEM=${GHIDRA_HEADLESS_MAXMEM:-2G}
export GHIDRA_HEADLESS_MAXMEM
filehash=$(sha256sum "$INPUT" | cut -c1-12)
rc=0
timeout "$((ANALYSIS_SEC+DECOMP_SEC+90))s" \
  "$GHIDRA/support/analyzeHeadless" "$OUT/projects" "resume_${filehash}" \
  -import "$INPUT" -overwrite -analysisTimeoutPerFile "$ANALYSIS_SEC" \
  -scriptPath "$ROOT/tools/ghidra" \
  -postScript DecompileFirmware.java "$OUT/functions.pass2.c" "$MAX_FUNCS" "$DECOMP_SEC" "$PRIOR" \
  -max-cpu 2 -deleteProject > "$OUT/ghidra.pass2.log" 2>&1 || rc=$?
printf '%s\n' "$rc" > "$OUT/exit-code.pass2.txt"
if ((rc)); then echo "Ghidra continuation incomplete (exit=$rc), consult logs" >&2; fi
exit "$rc"
