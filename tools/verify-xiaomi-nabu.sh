#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Reference comparison against a user-supplied, legally obtained Nabu firmware tree.
set -euo pipefail
if (( $# != 1 )); then
  echo "Usage: $0 PATH_TO_EXTRACTED_USR_LIB_FIRMWARE" >&2
  exit 2
fi
ROOT=$(cd "$(dirname "$0")/.." && pwd)
FW=$(realpath "$1")
QCOM="$FW/qcom/sm8150/Xiaomi/nabu"
[[ -d "$QCOM" ]] || { echo "Cannot find $QCOM" >&2; exit 2; }
"$ROOT/tools/verify.sh"
EXE="$ROOT/target/release/firmware-tools"
"$EXE" verify-modemuw "$QCOM/modemuw.jsn"
"$EXE" verify-a640-routines "$QCOM/a640_zap.mbn"
for name in a640_zap.mbn adsp.mbn cdsp.mbn modem.mbn venus.mbn wlanmdsp.mbn; do
  "$EXE" verify-elf-prefix "$name" "$QCOM/$name"
done
"$EXE" verify-wmfw-structure "$FW/cirrus/cs35l41-dsp1-spk-prot.wmfw"
(cd "$FW" && sha256sum -c "$ROOT/reference/xiaomi-nabu/firmware.sha256")
printf 'Xiaomi Nabu reference comparisons passed; full executable firmware and signatures are not rebuilt.\n'
