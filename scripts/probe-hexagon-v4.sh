#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# LOCAL DEVICE FIRMWARE ANALYSIS ONLY. Does not output original instruction bytes to repo.
# Usage: scripts/probe-hexagon-v4.sh LLVM6_BIN_DIR A640_MBN OUTPUT_DIR
set -euo pipefail
LLVM=${1:?Supply directory with LLVM6 llvm-mc and llvm-objdump binaries}
FIRMWARE=${2:?Supply locally extracted a640_zap.mbn}
OUT=${3:?Output directory OUTSIDE the GPLv3 source repository}
mkdir -p "$OUT"
test -x "$LLVM/llvm-mc"
test -x "$LLVM/llvm-objdump"
"$LLVM/llvm-objdump" --version > "$OUT/llvm6-version.txt" 2>&1
# llvm-objdump cannot always disassemble MDТ files without section tables.
"$LLVM/llvm-objdump" -d "$FIRMWARE" > "$OUT/llvm6-elf-disasm.txt" 2> "$OUT/llvm6-elf-disasm.err" || true
python3 - "$FIRMWARE" "$OUT" <<'PY'
from pathlib import Path
import sys,struct
b=Path(sys.argv[1]).read_bytes(); out=Path(sys.argv[2])
assert b[:7] == b'\x7fELF\x01\x01\x01' and struct.unpack_from('<H',b,18)[0] == 164
entry, phoff=struct.unpack_from('<II',b,24)
phsz,phn=struct.unpack_from('<HH',b,42)
for i in range(phn):
  kind,off,va,pa,fs,ms,flags,align=struct.unpack_from('<IIIIIIII',b,phoff+i*phsz)
  if kind==1 and flags&1 and ((pa <= entry < pa+fs) or (va<=entry < va+fs)):
    start=off+(entry-pa if pa <= entry < pa+fs else entry-va)
    d=b[start:min(start+512,off+fs)]
    d=d[:len(d)//4*4]
    with (out/'entry-hex-llvm-mc-input.txt').open('w') as f:
      for n in range(0,len(d),4):
        f.write(' '.join(f'0x{x:02x}' for x in d[n:n+4])+'\n')
    print(f'A640 entry 0x{entry:x} maps to file offset 0x{start:x}; read {len(d)} bytes')
    break
else: raise SystemExit('No executable file-backed segment contains ELF entry')
PY
"$LLVM/llvm-mc" --disassemble -triple=hexagon -mcpu=hexagonv4 \
    < "$OUT/entry-hex-llvm-mc-input.txt" > "$OUT/hexagon-v4-llvm6.txt" \
    2> "$OUT/hexagon-v4-llvm6.err" || true
"$LLVM/llvm-mc" --disassemble -triple=hexagon -mcpu=hexagonv5 \
    < "$OUT/entry-hex-llvm-mc-input.txt" > "$OUT/hexagon-v5-llvm6.txt" \
    2> "$OUT/hexagon-v5-llvm6.err" || true
printf 'V4 disassembler report: %s\n' "$OUT/hexagon-v4-llvm6.txt"
printf 'WARNING: nonempty disassembly is NOT automatically validated decompilation.\n'
