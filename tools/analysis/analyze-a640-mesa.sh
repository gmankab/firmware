#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Private analysis only. Disassembled OEM code is NOT GPL-licensed by this script.
# Usage: tools/analysis/analyze-a640-mesa.sh A640_ZAP_MBN /path/to/freedreno/bin OUTDIR
set -euo pipefail
if (( $# != 3 )); then
  printf 'Usage: %s A640_ZAP_MBN FREEDRENO_BIN_DIR OUTDIR\n' "$0" >&2
  exit 2
fi
FIRMWARE=$(realpath "$1")
BINDIR=$(realpath "$2")
OUTDIR=$(realpath -m "$3")
mkdir -p "$OUTDIR"
STAGE=$(dirname "$BINDIR")
# The Ubuntu packaged tools can run on Debian, but QRISC requires RNN's XML database.
if [[ -f "$STAGE/share/freedreno/registers/adreno.xml" ]]; then
  export RNN_PATH="$STAGE/share/freedreno/registers"
elif [[ -n "${RNN_PATH:-}" && -f "$RNN_PATH/adreno.xml" ]]; then
  : # The caller supplied Mesa's register XML path (e.g. for an uninstalled build).
else
  echo 'Warning: RNN_PATH is unset or missing adreno.xml; QRISC register names may fail to decode' >&2
fi
if [[ -d "$STAGE/runtime" ]]; then
  export LD_LIBRARY_PATH="$STAGE/runtime${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
fi

python3 - "$FIRMWARE" "$OUTDIR" <<'PY'
import hashlib, json, struct, sys
from pathlib import Path
b=Path(sys.argv[1]).read_bytes(); out=Path(sys.argv[2])
if b[:6]!=b'\x7fELF\x01\x01': raise SystemExit('not little-endian ELF32')
if len(b)<52: raise SystemExit('truncated ELF32 header')
machine=struct.unpack_from('<H',b,18)[0]
entry,phoff=struct.unpack_from('<II',b,24)
flags=struct.unpack_from('<I',b,36)[0]
if machine!=164: raise SystemExit(f'Unexpected e_machine {machine}; expected ELF wrapper 164')
phsize,phnum=struct.unpack_from('<HH',b,42)
if phsize<32 or phoff+phsize*phnum>len(b): raise SystemExit('invalid program header table')
segments=[]
for i in range(phnum):
    typ,off,vaddr,paddr,filesz,memsz,fl,align=struct.unpack_from('<IIIIIIII',b,phoff+phsize*i)
    if typ==1 and fl&1 and filesz:
        if off+filesz>len(b): raise SystemExit('segment exceeds file')
        segments.append((off,filesz,vaddr))
if len(segments)!=1: raise SystemExit(f'Expected 1 executable A640 segment; got {len(segments)}')
off,size,vaddr=segments[0]
if entry!=vaddr:raise SystemExit('entry not at beginning of executable segment')
seg=b[off:off+size]
if len(seg)%4:raise SystemExit('partial 32-bit instruction')
words=struct.unpack('<'+'I'*(len(seg)//4),seg)
fwid=(words[0]>>12)&0xfff
shader_candidates=[]
for i,w in enumerate(words[:-3]):
    if (w & 0xffff0000)!=0xc8000000:continue # QRISC unconditional relative jump
    displacement=struct.unpack('<h',struct.pack('<H',w&0xffff))[0]
    if displacement<=0:continue
    target=i+displacement
    start=i+3
    if not (start<target<=len(words)) or start%2 or (target-start)%2 or target-start<32:continue
    if words[i+1]>>24!=1 or words[i+2]>>24!=1:continue # A6xx QRISC NOPs
    shader_candidates.append((start,target,i))
shader=max(shader_candidates,key=lambda x:x[1]-x[0]) if shader_candidates else None
(out/'a640-pf_x.bin').write_bytes(seg)
if shader:
    start,end,_=shader
    (out/'a640-candidate-ir3.bin').write_bytes(seg[start*4:end*4])
meta={
 'firmware_sha256':hashlib.sha256(b).hexdigest(),
 'firmware_length':len(b),'elf_machine':machine,'elf_flags':flags,'elf_entry':entry,
 'segment_file_offset':off,'segment_size':size,'segment_virtual_address':vaddr,
 'segment_sha256':hashlib.sha256(seg).hexdigest(),
 'qrisc_firmware_id':f'0x{fwid:03x}',
 'qrisc_firmware_id_known_to_unmodified_mesa':fwid in (0x6ee,0x6dd,0x6dc),
 'candidate_shader':None if not shader else {
  'start_word':shader[0],'end_word_exclusive':shader[1],
  'skip_jump_word':shader[2], 'slots':(shader[1]-shader[0])//2,
  'sha256':hashlib.sha256(seg[shader[0]*4:shader[1]*4]).hexdigest()},
 'warning':'Candidate boundaries; validate IR3 with an independent ISA decoder. Disassembly is OEM-derived; NOT independently authored GPL source.'
}
(out/'input-metadata.json').write_text(json.dumps(meta,indent=2)+'\n')
print(f'Extracted {size} bytes, {size//4} QRISC/IR3 32-bit words, wrapper flags 0x{flags:x}, embedded firmware ID 0x{fwid:x}')
print(f'Shader candidate: {shader} (indices in 32-bit words)')
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
  # -g overrides 0x6ff: Mesa's utility lists A630/A650/A660 but not A640 ZAP.
  # The three IDs share the QRISC A6xx decoder family, not their full chipset semantics.
  run_probe qrisc-a630-raw "$BINDIR/qrisc-disasm" -r -g 6ee -v "$OUTDIR/a640-pf_x.bin"
  run_probe qrisc-a650-raw "$BINDIR/qrisc-disasm" -r -g 6dd -v "$OUTDIR/a640-pf_x.bin"
  run_probe qrisc-a660-raw "$BINDIR/qrisc-disasm" -r -g 6dc -v "$OUTDIR/a640-pf_x.bin"
else
  echo 'qrisc-disasm executable not found' >&2
fi
IR3_DISASM=${IR3_DISASM:-$BINDIR/ir3-disasm}
if [[ ! -x "$IR3_DISASM" && -x "$BINDIR/../isa/ir3-disasm" ]]; then
  IR3_DISASM="$BINDIR/../isa/ir3-disasm"
fi
if [[ -x "$IR3_DISASM" ]]; then
  # The proper IR3 CLI identifier is FD640; -g 640 is rejected.
  if [[ -f "$OUTDIR/a640-candidate-ir3.bin" ]]; then
    run_probe ir3-embedded-fd640 "$IR3_DISASM" -g FD640 "$OUTDIR/a640-candidate-ir3.bin"
  fi
  run_probe ir3-whole-segment-negative-control "$IR3_DISASM" -g FD640 "$OUTDIR/a640-pf_x.bin"
else
  echo 'ir3-disasm executable not found' >&2
fi

python3 - "$OUTDIR" <<'PY'
import json,re,sys
from pathlib import Path
out=Path(sys.argv[1]); m=json.loads((out/'input-metadata.json').read_text()); shader=m['candidate_shader']
summary={'qrisc':{},'ir3':{}}
for gpu in ('a630','a650','a660'):
    p=out/f'qrisc-{gpu}-raw.stdout.txt'
    if not p.exists(): continue
    rows={}
    for line in p.read_text().splitlines():
        r=re.match(r'\s*([0-9a-f]{4}):\s*([0-9a-f]{8})\s*(.*)$',line)
        if not r:continue
        index=int(r[1],16);tail=r[3].strip()
        if tail and not re.fullmatch(r'(?:l|fxn)\d+:',tail): rows[index]=tail
    control=[i for i in rows if not (shader and shader['start_word']<=i<shader['end_word_exclusive'])]
    unresolved=[i for i in control if rows[i].startswith('[')]
    summary['qrisc'][gpu]={'total_words':len(rows),'control_words':len(control),
        'instruction_words_recognized':len(control)-len(unresolved),
        'raw_control_words':len(unresolved)}
file=out/'ir3-embedded-fd640.stdout.txt'
if file.exists():
    instr={}
    for line in file.read_text().splitlines():
        r=re.match(r'\s*(\d+)\[[0-9a-f]+_[0-9a-f]+\]\s*(.*)$',line)
        if r and not re.fullmatch(r'l\d+:',r[2].strip()): instr[int(r[1])]=r[2]
    summary['ir3']={'total_slots':len(instr),'recognized':sum('no match:' not in x for x in instr.values()),
      'unrecognized':sum('no match:' in x for x in instr.values()),
      'warning_slots':sum('WARNING:' in x for x in instr.values())}
(out/'decoder-coverage.json').write_text(json.dumps(summary,indent=2)+'\n')
print('Coverage summary:',json.dumps(summary))
PY
printf '\nResults are private firmware-derived analysis, NOT GPLv3 program sources.\n'
