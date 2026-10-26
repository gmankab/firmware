#!/usr/bin/env python3
"""Independent Clang/LLVM A32 mnemonic round-trip to corroborate typed Rust.

Reads a privately extracted disassembly and manually identified data ranges.
Instruction hexadecimal text from disassembly is never emitted as .word
(except independently classified literal data with explicitly named values).
This is not itself reverse-engineered source reconstruction.
"""
from pathlib import Path
import sys, argparse, re, subprocess, tempfile
p=argparse.ArgumentParser();p.add_argument('--disasm',type=Path,required=True);p.add_argument('--start',type=lambda x:int(x,0),required=True);p.add_argument('--end',type=lambda x:int(x,0),required=True);p.add_argument('--data',action='append',default=[],help='start:end:comma-separated-literal-word-values');p.add_argument('--reference',type=Path,required=True);p.add_argument('--file-offset',type=lambda x:int(x,0),required=True);p.add_argument('--compare',type=Path,help='emitted Rust fragment bytes');p.add_argument('--output',type=Path);arg=p.parse_args()
DATA={}
for spec in arg.data:
 start,end,values=spec.split(':',2);start=int(start,0);end=int(end,0);words=[int(v,0) for v in values.split(',')];assert end-start==4*len(words) and start%4==0
 for a,value in zip(range(start,end,4),words):DATA[a]=value
pat=re.compile(r'^\s*([0-9a-f]+):\s+([0-9a-f]{8})\s+([a-z0-9.<>]+)\s*(.*)$',re.I)
rows={}
for line in arg.disasm.read_text().splitlines():
 match=pat.match(line)
 if not match:continue
 at=int(match[1],16)
 if arg.start<=at<arg.end:rows[at]=(match[3],match[4].split(' @ ')[0].strip())
asm=['.syntax unified','.arm','.text','.global rebuild','rebuild:'];total=0
for at in range(arg.start,arg.end,4):
 if at in DATA: asm.append('    .word 0x%08x // independently classified literal data' % DATA[at]);continue
 if at not in rows:raise ValueError(f'missing instruction {at:#x}; possible data pool')
 mn,opers=rows[at]
 if mn in ('<unknown>',):raise ValueError(f'unrecognized instruction at {at:#x}; classify manually')
 op=re.sub(r'\s*<[^>]*>','',opers).strip()
 if re.fullmatch(r'b(?:l(?:x)?)?(?:eq|ne|cc|lo|cs|hs|hi|ls|ge|gt|le|lt|mi|pl)?',mn):
  m=re.match(r'^(0x[0-9a-f]+)\b',op)
  if m:
   target=int(m[1],16)
   op=f'. + {target-at}' if target>=at else f'. - {at-target}'
 asm.append(f'    {mn} {op}'.rstrip());total+=1
with tempfile.TemporaryDirectory() as d:
 d=Path(d);a=d/'block.S';o=d/'block.o';b=d/'block.bin';a.write_text('\n'.join(asm)+'\n')
 comp=subprocess.run(['clang','--target=armv7a-none-eabi','-c',str(a),'-o',str(o)],capture_output=True,text=True)
 if comp.returncode:print(comp.stderr,file=sys.stderr);print(a.read_text()[-1500:]);sys.exit(1)
 subprocess.run(['llvm-objcopy','--only-section=.text','-O','binary',str(o),str(b)],check=True)
 generated=b.read_bytes()
 original=arg.reference.read_bytes()[arg.file_offset:arg.file_offset+len(generated)]
 if len(generated)!=arg.end-arg.start:raise ValueError('size wrong '+str(len(generated)))
 if generated!=original:
  diff=next((i for i,(x,y) in enumerate(zip(generated,original)) if x!=y),None)
  raise ValueError(f'Clang diff at VMA {arg.start+diff:#x}, got {generated[diff:diff+4].hex()}, expected {original[diff:diff+4].hex()}')
 if arg.compare:
  b2=arg.compare.read_bytes()
  if b2!=generated:raise ValueError('typed Rust fragment differs from independent LLVM assembler')
 if arg.output:arg.output.write_bytes(generated)
 print(f'Clang verified {len(generated)} exact bytes: {total} instructions and {len(DATA)} independently identified data words; runtime [{arg.start:#x},{arg.end:#x})')
