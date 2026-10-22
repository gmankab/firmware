#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Generate local, private disassembly hypotheses for GPU ZAP firmware.
# No source output is asserted correct without independent ISA validation.
set -euo pipefail
if (( $# != 3 )); then
  echo 'Usage: analyze-a640-mesa.sh A640_ZAP_MBN MESA_BIN_DIRECTORY OUTDIR' >&2
  exit 2
fi
FIRMWARE=$(realpath "$1")
BINDIR=$(realpath "$2")
OUTDIR=$(realpath -m "$3")
mkdir -p "$OUTDIR"
command -v python3 >/dev/null
python3 - "$FIRMWARE" "$OUTDIR/a640-pf_x.bin" "$OUTDIR/input-metadata.json" <<'PY'
import hashlib,json,struct,sys
from pathlib import Path
b=Path(sys.argv[1]).read_bytes()
if b[:6]!=b'\x7fELF\x01\x01': raise SystemExit('not little-endian ELF32')
machine = struct.unpack_from('<H', b, 18)[0]
entry, phoff = struct.unpack_from('<II', b, 24)
flags = struct.unpack_from('<I', b, 36)[0]
if machine!=164: raise SystemExit(f'Unexpected machine: {machine}; expected Hexagon ELF container')
phsize,phnum=struct.unpack_from('<HH',b,42)
if phsize!=32 or phoff+phsize*phnum>len(b): raise SystemExit('bad ELF32 header table')
segments=[]
for i in range(phnum):
    typ,off,vaddr,paddr,filesz,memsz,fl,align=struct.unpack_from('<IIIIIIII',b,phoff+32*i)
    if typ==1 and fl&1 and filesz:
        if off+filesz>len(b): raise SystemExit('bad segment')
        segments.append((off,filesz,vaddr,paddr))
if len(segments)!=1: raise SystemExit(f'Expected exactly one executable A640 segment; found {len(segments)}')
off,size,vm,pa=segments[0]
segment=b[off:off+size]
Path(sys.argv[2]).write_bytes(segment)
Path(sys.argv[3]).write_text(json.dumps({'firmware_sha256':hashlib.sha256(b).hexdigest(),
    'firmware_length':len(b),'elf_flags':flags,'elf_entry':entry,
    'segment_file_offset':off,'segment_size':size,'segment_virtual_address':vm,
    'segment_sha256':hashlib.sha256(segment).hexdigest(),
    'warning':'PF_X alone does NOT establish the instruction ISA'},indent=2)+'\n')
print(f'Extracted {size} bytes at file 0x{off:x}; ELF entry 0x{entry:x}; e_flags 0x{flags:x}')
PY

run_probe() {
  local name="$1"; shift
  printf '\n== %s ==\n' "$name" | tee -a "$OUTDIR/probes.log"
  local rc=0
  timeout 30s "$@" > "$OUTDIR/$name.stdout.txt" 2> "$OUTDIR/$name.stderr.txt" || rc=$?
  printf '%s exit=%s\n' "$name" "$rc" | tee -a "$OUTDIR/probes.log"
  return 0
}
if [[ -x "$BINDIR/qrisc-disasm" ]]; then
  run_probe qrisc-help "$BINDIR/qrisc-disasm" -h
  # Mesa's QRISC CLI expects a raw stream with -r; without -g it infers a
  # firmware ID from word[1], which is meaningless for an ELF-wrapped blob.
  # A640 is not directly listed among Mesa's named versions; A630/A650/A660
  # below are comparison hypotheses, NOT a claim that ZAP contains QRISC.
  run_probe qrisc-a630-raw "$BINDIR/qrisc-disasm" -r -g 6ee -v "$OUTDIR/a640-pf_x.bin"
  run_probe qrisc-a650-raw "$BINDIR/qrisc-disasm" -r -g 6dc -v "$OUTDIR/a640-pf_x.bin"
  run_probe qrisc-a660-raw "$BINDIR/qrisc-disasm" -r -g 6dd -v "$OUTDIR/a640-pf_x.bin"
  # Deliberately test the whole ELF as a negative control, not as firmware.
  run_probe qrisc-elf-negative-control "$BINDIR/qrisc-disasm" -r -g 6ee "$FIRMWARE"
else
  echo 'qrisc-disasm not found; run Codex Mesa toolchain script' >&2
fi
if [[ -x "$BINDIR/ir3-disasm" ]]; then
  run_probe ir3-help "$BINDIR/ir3-disasm" -h
  run_probe ir3-whole-segment "$BINDIR/ir3-disasm" -g 640 "$OUTDIR/a640-pf_x.bin"
fi
printf '\nDecoding hypothesis only: do not treat Hexagon ELF header flags or PF_X as conclusive ISA evidence.\n'
