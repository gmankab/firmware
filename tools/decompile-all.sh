#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Decompiles functions that Ghidra identifies, with bounded analysis and process time.
# Its output is research evidence derived from vendor binaries, NOT GPL-licensed code.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
FW=${1:?Usage: tools/decompile-all.sh ORIGINAL_FIRMWARE_ROOT GHIDRA_INSTALL_DIR OUTPUT_DIRECTORY}
GHIDRA=${2:?The extracted Ghidra 12.1.x directory is required}
OUT=${3:?Provide an output folder separate from the Git repo}
ANALYSIS_SEC=${ANALYSIS_SEC:-70}
DECOMP_SEC=${DECOMP_SEC:-95}
MAX_FUNCS=${MAX_FUNCS:-100000}
GHIDRA_HEADLESS_MAXMEM=${GHIDRA_HEADLESS_MAXMEM:-1850M}
export GHIDRA_HEADLESS_MAXMEM
mkdir -p "$OUT/projects" "$OUT/layouts"
for file in \
    "$FW/qcom/sm8150/Xiaomi/nabu"/*.mbn \
    "$FW/cirrus/cs35l41-dsp1-spk-prot.wmfw" \
    "$FW/novatek/novatek_nt36523_fw.bin" \
    "$FW/qcom/sm8150/Xiaomi/nabu/modemuw.jsn"; do
    [ -f "$file" ] || { printf 'missing: %s\n' "$file" >&2; exit 1; }
    "$ROOT/target/release/nabu-firmware-research" inspect-image "$file" \
        > "$OUT/layouts/$(basename "$file").txt"
done
for f in a640_zap.mbn venus.mbn wlanmdsp.mbn modem.mbn cdsp.mbn adsp.mbn; do
    file="$FW/qcom/sm8150/Xiaomi/nabu/$f"
    printf 'Running Ghidra native analysis: %s\n' "$f" >&2
    # Kernel processes can recover after analysis timeout. Preserve status honestly.
    timeout "$((ANALYSIS_SEC + DECOMP_SEC + 65))s" \
        "$GHIDRA/support/analyzeHeadless" "$OUT/projects" "nabu_${f%%.*}" \
        -import "$file" -overwrite \
        -analysisTimeoutPerFile "$ANALYSIS_SEC" \
        -scriptPath "$ROOT/tools/ghidra" \
        -postScript DecompileNabu.java "$OUT/$f.c" "$MAX_FUNCS" "$DECOMP_SEC" \
        -max-cpu 2 -deleteProject >"$OUT/$f.log" 2>&1 || {
            printf 'Incomplete analysis: %s, check %s\n' "$f" "$OUT/$f.log" >&2
        }
done
printf 'NOTE: auto-decompiler outputs are OEM-derived evidence, NOT newly authored Rust firmware.\n' >&2
