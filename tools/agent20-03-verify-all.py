#!/usr/bin/env python3
"""Validate a complete agent03 contiguous typed Venus fragment using named data intervals.

Run: python3 tools/agent20-03-verify-all.py --disasm /private/venus-disasm.txt
     --reference /private/venus.mbn --generated /tmp/fragment.bin --workdir /tmp/verify

The helper delegates to the independent Clang/LLD assembler and additionally
checks that the data ledger neither overlaps nor exceeds the generated range.
Only literal addresses/ranges are stored in Git, not OEM binary data.
"""
import argparse,json,subprocess,sys
from pathlib import Path
here=Path(__file__).resolve().parent
p=argparse.ArgumentParser()
for flag in ('disasm','reference','generated','workdir'):
 p.add_argument('--'+flag,required=True,type=Path)
a=p.parse_args()
ledger=json.loads((here/'agent20-03-venus-mid-ranges.json').read_text())
start=int(ledger['runtime_start'],16)
end=start+a.generated.stat().st_size
limit=int(ledger['reservation_end'],16)
if not start<end<=limit or (end-start)%4:
 p.error('generated fragment has invalid boundary/length')
ranges=sorted((int(x,16),int(y,16)) for x,y in ledger['data_ranges'])
active=[];prior=start
for l,r in ranges:
 if r<=end:
  if not (start<=l<r<=end and (l|r)%4==0 and prior<=l):
   p.error(f'invalid overlapping/unaligned literal interval {l:#x}..{r:#x}')
  active.append((l,r));prior=r
 elif l<end:
  p.error(f'partial data pool crosses current end: {l:#x}..{r:#x}')
command=[sys.executable,str(here/'agent20-03-a32-mnemonic-roundtrip.py'),
         '--disasm',str(a.disasm),'--start',hex(start),'--end',hex(end),
         '--reference',str(a.reference),'--generated',str(a.generated),
         '--workdir',str(a.workdir)]
for l,r in active:command.extend(['--data',f'{l:#x}:{r:#x}'])
print(f'Verifying {end-start} source-encoded bytes through {end:#x}: '
      f'{sum((r-l)//4 for l,r in active)} data words in {len(active)} pools',flush=True)
subprocess.run(command,check=True)
