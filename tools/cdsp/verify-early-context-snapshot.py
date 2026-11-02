#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Validate CDSP exception register snapshot: byte-identical generated
source, V66 privileged-register/packet disassembly, and 22 independent
LLVM6 memory/frame/ABI encoding checks. Privileged registers are not
misrepresented as LLVM-assembled.
"""
import argparse,importlib.util,re,subprocess,tempfile
from pathlib import Path
START=0x495c;PHYSICAL=0x96c0_195c;SIZE=172
EXPECTED={
0x495c:'r19=p3:0',0x4960:'r18=r28',
0x4964:'allocframe(sp,#0x8)',0x4968:'memd(sp+##0x0)=r19:18',
0x496c:'r18=pmucfg',0x4970:'r19=modectl',
0x4974:'memd(r0+##0x0)=r19:18',0x4978:'r19:18=utimer',
0x497c:'memd(r0+##0x8)=r19:18',0x4980:'r18=evb',
0x4984:'r19=diag',0x4988:'memd(r0+##0x10)=r19:18',
0x4994:'memd(r0+##0x18)=r19:18',
0x49a0:'r18=r1',0x49a4:'r19=##0x0',
0x49a8:'memd(r0+##0x28)=r19:18',
0x49b0:'memd(r0+##0x40)=r19:18',
0x49b4:'r28=getimask(r1)',
0x49bc:'memw(r0+##0x30)=r28',
0x49c0:'r28=getimask(r1)',
0x49c8:'memw(r0+##0x34)=r28',
0x49cc:'r28=getimask(r1)',
0x49d4:'memw(r0+##0x38)=r28',
0x49d8:'r28=getimask(r1)',
0x49dc:'r19:18=memd(sp+##0x0)',
0x49e0:'memw(r0+##0x3c)=r28',
0x49e4:'r28=s63',0x49e8:'memw(r0+##0x44)=r28',
0x49ec:'r28=s60',0x49f0:'memw(r0+##0x48)=r28',
0x49f4:'r28=s35',0x49f8:'memw(r0+##0x58)=r28',
0x49fc:'p3:0=r19',0x4a00:'r28=r18',0x4a04:'lr:fp=dealloc_return(fp)',
}
LLVM_ASM='''.text
.global context
context:
{ r18=r28
  allocframe(#8) }
{ memd(sp+#0)=r19:18 }
{ memd(r0+#0)=r19:18 }
{ memd(r0+#8)=r19:18 }
{ memd(r0+#16)=r19:18 }
{ memd(r0+#24)=r19:18 }
{ r18=r1
  r19=#0
  memd(r0+#40)=r19:18 }
{ r1=#0
  memd(r0+#64)=r19:18 }
{ memw(r0+#48)=r28 }
{ memw(r0+#52)=r28 }
{ memw(r0+#56)=r28 }
{ r19:18=memd(sp+#0)
  memw(r0+#60)=r28 }
{ memw(r0+#68)=r28 }
{ memw(r0+#72)=r28 }
{ memw(r0+#88)=r28 }
{ r28=r18 }
{ dealloc_return }
'''

def extract_elf(obj:Path)->bytes:
 helper=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
 spec=importlib.util.spec_from_file_location('agent20_snapshot_elf',helper)
 assert spec and spec.loader
 module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
 return module.parse_elf32_text(obj.read_bytes())

def verify(llvm:Path,rz:Path,generated:Path,reference:Path):
 b=generated.read_bytes()
 if len(b)!=SIZE or reference.read_bytes()[START:START+SIZE]!=b:
  raise ValueError('CDSP snapshot typed source differs from original CDSP reference')
 decoded={}
 result=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',b.hex()],capture_output=True,text=True,check=True)
 for row in result.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?\\/\[|]\s+(.*)',row)
  if m:decoded[int(m[1],16)]=''.join(m[2].lower().split())
 if len(decoded)!=43:raise ValueError(f'Expected 43 decoded Hexagon words, got {len(decoded)}')
 for offset,text in EXPECTED.items():
  got=decoded.get(PHYSICAL+offset-START,'')
  if text not in got:raise ValueError(f'Unexpected V66 at {offset:#x}: {got}, wanted {text}')
 with tempfile.TemporaryDirectory(prefix='hexagon-snapshot-') as d:
  src=Path(d)/'llvm.s';obj=Path(d)/'llvm.o';src.write_text(LLVM_ASM)
  subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(src)],capture_output=True,text=True,check=True)
  llvmcode=extract_elf(obj)
 comparisons=[(0x4960,0),(0x4964,4),(0x4968,8),(0x4974,12),
 (0x497c,16),(0x4988,20),(0x4994,24),(0x49a0,28),
 (0x49a4,32),(0x49a8,36),(0x49ac,40),(0x49b0,44),
 (0x49bc,48),(0x49c8,52),(0x49d4,56),(0x49dc,60),
 (0x49e0,64),(0x49e8,68),(0x49f0,72),(0x49f8,76),
 (0x4a00,80),(0x4a04,84)]
 for a,off in comparisons:
  value=int.from_bytes(b[a-START:a-START+4],'little')
  independently=int.from_bytes(llvmcode[off:off+4],'little')
  if (value&~0xc000)!=(independently&~0xc000):
   raise ValueError(f'LLVM6 independent scalar/memory mismatch at {a:#x}')
 print('CDSP context snapshot: 172 exact typed source bytes, 43 Rizin V66 words, 22 independent LLVM6 instruction-field matches; S54/MODECTL/PMUCFG etc decoder-only.')
def main():
 ap=argparse.ArgumentParser(description=__doc__)
 for item in ('llvm-mc','rz-asm','rust-output','reference'):ap.add_argument('--'+item,required=True,type=Path)
 a=ap.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
if __name__=='__main__':main()
