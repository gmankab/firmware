#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent verification for 52-byte guarded cleanup after CDSP callbacks.

Rizin V66 independently decodes all 13 instruction words; LLVM6 reassembles
branch, duplex stores, CLRBIT memory operation, and self-loop. GP-relative
extended address selection is semantically decoded and reference byte checked;
LLVM6 shortens those loads and is not claimed to reproduce their exact packets.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess,tempfile

START=0x4fd0
END=0x5004
PHYS=0x96c01fd0
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
DESCRIPTIONS=[
 'immext(##0xfe0ac080)', 'r0=memw(gp+##0xfe0ac0a0)',
 'if(cmp.eq(r0.new,#0x0))jump:t0x96c02000',
 'immext(##0xfe0a5dc0)', 'r0=memw(gp+##0xfe0a5ddc)',
 'if(cmp.eq(r0.new,#0x0))jump:t0x96c02000',
 'immext(##0xfe037ac0)', 'r0=memw(gp+##0xfe037ad0)',
 'if(cmp.eq(r0.new,#0x0))jump:nt0x96c02000',
 'memw(r0+#0x24)=#0;memw(r0+#0x20)=#0',
 'memw(r0+#0x10)=#0;memw(r0+#0x14)=#0',
 'memw(r0+##0x4)=clrbit(#0x15)',
 'jump0x96c02000',
]
assert len(DESCRIPTIONS)==13

def assemble(mc:Path,asm:str)->bytes:
 with tempfile.TemporaryDirectory(prefix='cdsp37-') as t:
  root=Path(t)
  (root/'sample.s').write_text('.text\n.global entry\nentry:\n'+asm+'\n')
  proc=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
    '-o',str(root/'sample.o'),str(root/'sample.s')],text=True,capture_output=True)
  if proc.returncode:raise ValueError('LLVM6 rejected independent semantics: '+proc.stderr)
  helper=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
  spec=importlib.util.spec_from_file_location('cdsp_guard_elf',helper)
  assert spec and spec.loader
  mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
  return mod.parse_elf32_text((root/'sample.o').read_bytes())

def verify(rust:Path,ref:Path,rz:Path,mc:Path)->None:
 raw=ref.read_bytes();out=rust.read_bytes()
 if hashlib.sha256(raw).hexdigest()!=SHA:raise ValueError('Wrong CDSP firmware revision')
 if len(out)!=END-START or out!=raw[START:END]:raise ValueError('typed CDSP guard differs from source firmware')
 result=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',out.hex()],
                        capture_output=True,text=True,check=True)
 decoded=[]
 for line in result.stdout.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:decoded.append((int(m[1],16),''.join(m[2].lower().split())))
 if len(decoded)!=len(DESCRIPTIONS):raise ValueError('Rizin decoded unexpected number of V66 words')
 for i,((at,text),expected) in enumerate(zip(decoded,DESCRIPTIONS)):
  if at!=PHYS+4*i or expected not in text:
   raise ValueError(f'Rizin word {i} at {at:#x} expected {expected!r} got {text!r}')
 # Predicated new-value branch displacement comes from packet start and is
 # independently assembled at +0x30 using a normal load of the same producer.
 branch=assemble(mc,'{ r0=memw(r0+#0); if (cmp.eq(r0.new,#0)) jump:t target }\n.space 40\ntarget:\n{ jumpr lr }')
 if branch[4:8]!=out[8:12]:raise ValueError('LLVM6 zero-guard branch does not match typed encoding')
 ordinary=assemble(mc,'''{ memw(r0+#36)=#0; memw(r0+#32)=#0 }
{ memw(r0+#16)=#0; memw(r0+#20)=#0 }
{ memw(r0+#4)=clrbit(#21) }
loop:
{ jump loop }''')
 if ordinary!=out[36:]:raise ValueError('LLVM6 duplex+CLRBIT+self jump group does not match')
 print('CDSP guard-exit: 52 reference-exact bytes; 13 Rizin-decoded instructions; 5 exact LLVM6 words; extended GP addresses are Rizin+reference-verified')

if __name__=='__main__':
 a=argparse.ArgumentParser();a.add_argument('--rust-output',type=Path,required=True);a.add_argument('--reference',type=Path,required=True);a.add_argument('--rz-asm',type=Path,required=True);a.add_argument('--llvm-mc',type=Path,required=True);args=a.parse_args()
 verify(args.rust_output,args.reference,args.rz_asm,args.llvm_mc)
