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
# Relative CARGO_TARGET_DIR values are relative to the Cargo workspace.
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
if [[ "$TARGET_DIR" != /* ]]; then
  TARGET_DIR="$ROOT/$TARGET_DIR"
fi
EXE="$TARGET_DIR/release/firmware-tools"
"$EXE" verify-modemuw "$QCOM/modemuw.jsn"
"$EXE" verify-a640-routines "$QCOM/a640_zap.mbn"
"$EXE" verify-a640-complete "$QCOM/a640_zap.mbn"
"$EXE" verify-venus-contiguous-boot "$QCOM/venus.mbn"
"$EXE" verify-venus-upper-dispatch "$QCOM/venus.mbn"
# Optional reference types: compare if present, without silently requiring
# alternative vendor directory structures in old extracted firmware trees.
ATH10K="$FW/ath10k/WCN3990/hw1.0"
if [[ -f "$ATH10K/firmware-5.bin" ]]; then
  "$EXE" verify-ath10k-metadata "$ATH10K/firmware-5.bin"
fi
if [[ -f "$ATH10K/board-2.bin" ]]; then
  "$EXE" verify-board2-framing "$ATH10K/board-2.bin"
fi
if [[ -f "$FW/qca/crbtfw32.tlv" ]]; then
  "$EXE" verify-qca-bt-patch-header "$FW/qca/crbtfw32.tlv"
fi
if [[ -f "$FW/qca/crnv32.bin" ]]; then
  "$EXE" verify-qca-bt-nvm-headers "$FW/qca/crnv32.bin"
fi
if [[ -f "$FW/qcom/a640_gmu.bin" ]]; then
  "$EXE" verify-gmu-framing "$FW/qcom/a640_gmu.bin"
fi
for channel in BL BR TL TR; do
  item="$FW/cirrus/${channel}-cs35l41-dsp1-spk-prot.bin"
  if [[ -f "$item" ]]; then
    "$EXE" verify-cirrus-tuning "$channel" "$item"
  fi
done
for name in a640_zap.mbn adsp.mbn cdsp.mbn modem.mbn venus.mbn wlanmdsp.mbn; do
  "$EXE" verify-elf-prefix "$name" "$QCOM/$name"
  "$EXE" audit-mbn-unsigned "$name" "$QCOM/$name" >/dev/null
  "$EXE" verify-mbnv6-hashes "$QCOM/$name"
  if command -v openssl >/dev/null 2>&1; then
    python3 "$ROOT/tools/verify/verify-mbn-signatures.py" "$QCOM/$name" >/dev/null
  fi
done
"$EXE" verify-wmfw-structure "$FW/cirrus/cs35l41-dsp1-spk-prot.wmfw"
(cd "$FW" && sha256sum -c "$ROOT/reference/xiaomi-nabu/firmware.sha256")
printf 'Xiaomi Nabu reference comparisons passed; A640 executable, Venus contiguous boot/runtime region and six MBNv6 hash tables match. OEM signatures verified when OpenSSL is installed.\n'
