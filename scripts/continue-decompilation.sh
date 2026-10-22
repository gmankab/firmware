#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Optional second attempt for large Hexagon files that exceeded a prior budget.
# The generated pseudocode is OEM-derived research data, not GPLv3 Rust source.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
FW=${1:?Usage: scripts/continue-decompilation.sh FW_ROOT GHIDRA_DIR PRIOR_OUTPUT_DIR}
G=${2:?Must supply Ghidra install directory}
OUT=${3:?Must supply output directory from first run}
export GHIDRA_HEADLESS_MAXMEM=${GHIDRA_HEADLESS_MAXMEM:-1950M}
mkdir -p "$OUT/projects"
for file in wlanmdsp.mbn cdsp.mbn adsp.mbn; do
    if [[ "$file" == "adsp.mbn" ]]; then
        # Huge instruction stream may exhaust the default autoanalysis timeout.
        auto=${ANALYSIS_SEC_ADSP:-340}
        secs=${DECOMP_SEC_ADSP:-175}
        args=()
    else
        auto=${ANALYSIS_SEC:-95}
        secs=${DECOMP_SEC:-210}
        # Skip function addresses previously attempted, not all functions;
        # retaining both passes makes the level of coverage auditable.
        args=("$OUT/$file.c")
    fi
    printf 'Ghidra continuation %s\n' "$file" >&2
    timeout "$((auto+secs+75))s" "$G/support/analyzeHeadless" \
        "$OUT/projects" "nabu_cont_${file%.*}" \
        -import "$FW/qcom/sm8150/Xiaomi/nabu/$file" -overwrite \
        -analysisTimeoutPerFile "$auto" \
        -scriptPath "$ROOT/scripts/ghidra" \
        -postScript DecompileNabu.java "$OUT/$file.pass2.c" 100000 "$secs" "${args[@]}" \
        -max-cpu 3 -deleteProject > "$OUT/$file.pass2.log" 2>&1 || {
            echo "Warning: check $file.pass2.log (incomplete or failed)" >&2
        }
done
