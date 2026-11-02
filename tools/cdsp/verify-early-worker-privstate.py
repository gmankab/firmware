#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent V66 state snapshot/cache controller/SSR register verification."""
from pathlib import Path
import argparse,hashlib,re,subprocess,importlib.util,tempfile
START=0x4ec8;END=0x4f1c;PHYS=0x96c0_1ec8
HASH='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
WORDS=['r1=htid','immext(##0xfe016e00)','r0=##-0x1fe9200',
 'r0+=mpyi(r1,##0x4)','r1=modectl','memw(r0+##0x0)=r1.new',
 'dccleana(r0)','dcinva(r0)','r1=memw(r0+##0x0)',
 'r0=syscfg','r0=clrbit(r0,#0x4)','syscfg=r0',
 'r0=ssr','r1=##0x0','r0=insert(r1,#0x3,#0x10)','ssr=r0',
 'immext(##0x0)','r0=##0x24','immext(##0xfe000040)',
 'memw(r1=##0xfe000044)=r0.new','dccleana(r1)']
assert len(WORDS)==21
ASM='''{ r0+=mpyi(r1,#4) }
{ dccleana(r0) }
{ dcinva(r0) }
{ r1=memw(r0+#0) }
{ r0=clrbit(r0,#4) }
{ r1=#0 }
{ r0=insert(r1,#3,#16) }
{ r0=##36; memw(r1=##0xfe000044)=r0.new }
{ dccleana(r1) }
'''
INDEX=[3,6,7,8,10,13,14,16,17,18,19,20]
def elf_text(b):
 f=Path(__file__).with_name('verify-cdsp-hexagon-entry.py');sp=importlib.util.spec_from_file_location('cdsp34_elf',f);m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
 return m.parse_elf32_text(b)
def run(args):
 ref=args.reference.read_bytes();built=args.rust_output.read_bytes()
 if hashlib.sha256(ref).hexdigest()!=HASH:raise ValueError('CDSP pinned SHA-256 differs')
 if len(built)!=END-START or built!=ref[START:END]:raise ValueError('source-generated CDSP sysreg bytes mismatch')
 v=subprocess.run([str(args.rz_asm),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',built.hex()],text=True,capture_output=True,check=True)
 rows=[]
 for line in v.stdout.splitlines():
  m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:rows.append((int(m.group(1),16),''.join(m.group(2).lower().split())))
 if len(rows)!=len(WORDS):raise ValueError('Rizin missing CDSP V66 instructions')
 for i,((addr,decoded),want) in enumerate(zip(rows,WORDS)):
  if addr!=PHYS+i*4 or want not in decoded:
   raise ValueError(f'V66 unexpected at {addr:#x}: {decoded!r} vs {want!r}')
 with tempfile.TemporaryDirectory(prefix='cdsp34-') as d:
  path=Path(d);a=path/'source.s';o=path/'source.o';a.write_text('.text\n.global test\ntest:\n'+ASM)
  subprocess.run([str(args.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(o),str(a)],text=True,capture_output=True,check=True)
  assembled=elf_text(o.read_bytes())
 if len(assembled)!=4*len(INDEX):raise ValueError('LLVM6 assembled unexpected word count')
 for j,i in enumerate(INDEX):
  if assembled[j*4:j*4+4]!=built[i*4:i*4+4]:raise ValueError(f'LLVM6 scalar mismatch at {START+i*4:#x}')
 print(f'CDSP worker privileged state: {len(built)} exact bytes, {len(WORDS)} V66 words, {len(INDEX)} independent LLVM6 word matches; privileged registers checked by Rizin')
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__)
 for n in ('reference','rust-output','rz-asm','llvm-mc'):p.add_argument('--'+n,required=True,type=Path)
 run(p.parse_args())
