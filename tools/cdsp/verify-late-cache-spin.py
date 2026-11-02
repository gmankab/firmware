#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6 and Rizin V66 validation of CDSP feature-spin/cache update."""
from __future__ import annotations
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path

FILE_OFFSET=0x5539c
PHYSICAL=0x96cc739c
SIZE=0x6c


def independent_assembly()->str:
    return '''.text
.global cdsp_cache_spin
cdsp_cache_spin:
.space 0x38c
wait_bit:
.space 0x10
feature_bit_spin:
{ r1=memw(r4+#0) }
{ r3=and(r1,r2) }
{ p0=cmp.eq(r2,r3)
  if (!p0.new) jump:t wait_bit }
{ r0=##0x0830020c }
{ r7=r0
  r5.h=#0 }
{ r5.l=#1
  dcinva(r7) }
{ r6=sub(#-1,r5)
  r4=memw(r0+#0) }
{ r4=and(r4,r6) }
{ r4=or(r4,r5) }
{ memw(r0+#0)=r4 }
{ barrier }
{ r0=##0x08300200 }
{ r7=r0
  r5.h=#0 }
{ r5.l=#1
  dcinva(r7) }
{ r6=sub(#-1,r5)
  r4=memw(r0+#0) }
{ r4=and(r4,r6) }
{ r4=or(r4,r5) }
{ memw(r0+#0)=r4 }
{ barrier }
'''


def verify(llvm:Path,rz:Path,generated:Path,reference:Path|None)->None:
    inp=generated.read_bytes()
    data=inp[0x39c:] if len(inp)==0x408 else inp
    if len(data)!=SIZE:raise ValueError('Expected 108 bytes or 1032-byte complete controller prefix')
    with tempfile.TemporaryDirectory(prefix='hexagon-cache-spin-') as temp:
        asm=Path(temp)/'spin.s';obj=Path(temp)/'spin.o';asm.write_text(independent_assembly())
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(asm),'-o',str(obj)],check=True)
        spec=importlib.util.spec_from_file_location('verify_cdsp_text',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
        assert spec and spec.loader
        mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
        independent=mod.parse_elf32_text(obj.read_bytes())[0x39c:]
    if independent!=data:
        d=next(i for i,(a,b) in enumerate(zip(independent,data)) if a!=b)
        raise ValueError(f'LLVM6 source mismatch file {FILE_OFFSET+d:#x}')
    decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],check=True,text=True,capture_output=True)
    lines=[x for x in decoded.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=SIZE//4 or any('invalid' in x.lower() for x in lines):
        raise ValueError('Rizin V66 cannot decode all 27 instruction words')
    all_text='\n'.join(lines).replace(' ','').lower()
    for term in ['p0=cmp.eq(r2,r3)','jump:t0x96cc738c','dcinva(r7)','r0=##0x830020c','r0=##0x8300200','memw(r0+##0x0)=r4']:
        if term not in all_text:raise ValueError('Rizin missing semantic fragment '+term)
    if reference and reference.read_bytes()[FILE_OFFSET:FILE_OFFSET+SIZE]!=data:
        raise ValueError('Generated CDSP code differs from pinned original')
    print('PASS: 108 exact Hexagon feature-spin/cache-MMIO bytes; LLVM6 and Rizin V66 independent checks')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True);p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--generated',type=Path,required=True);p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference)
