#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent CDSP L2 sync checks: reference equality, Rizin V66 decode and
LLVM6 reproducible ordinary arithmetic / memory / branch forms.

The privileged L2 operations and SYSCFG read/write are corroborated by the
V66 decoder only; LLVM6 does NOT independently assemble privileged L2 ops.
"""
import argparse,importlib.util,re,subprocess,tempfile
from pathlib import Path
START=0x4a08
PADDR=0x96c01a08
SIZE=48
EXPECTED={
 0x4a08:'r0=syscfg',
 0x4a0c:'immext(##0xfe026cc0)',
 0x4a10:'r1=memw(r23+##-0x1fd9334)',
 0x4a14:'r0=insert(r1,#0x3,#0x10)',
 0x4a18:'syscfg=r0',
 0x4a1c:'isync',
 0x4a20:'l2gunlock',
 0x4a24:'l2gclean',
 0x4a28:'r0=syscfg',
 0x4a2c:'p0=tstbit(r0,#0x1f)',
 0x4a30:'if(p0.new)jump:t0x96c01a28',
 0x4a34:'jumprlr'
}
ASM=''' .text
.global sample
sample:
{ r1=memw(r23+##-0x1fd9334) }
{ r0=insert(r1,#3,#16) }
{ isync }
{ r0=insert(r1,#2,#14) }
{ r2=memw(r20+##0x1fd334) }
{ jumpr lr }
'''

def elf_text(p:Path)->bytes:
    spec=importlib.util.spec_from_file_location('cdsp_elf_loader',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
    assert spec and spec.loader
    lib=importlib.util.module_from_spec(spec);spec.loader.exec_module(lib)
    return lib.parse_elf32_text(p.read_bytes())

def verify(llvm:Path,rz:Path,emitted:Path,reference:Path):
    b=emitted.read_bytes();original=reference.read_bytes()
    if len(b)!=SIZE or original[START:START+SIZE]!=b:
        raise ValueError('Typed CDSP L2 sync does not match pinned firmware')
    cmd=[str(rz),'-a','hexagon','-b','32','-o',hex(PADDR),'-D',b.hex()]
    decoded=subprocess.run(cmd,capture_output=True,text=True,check=True)
    lines={}
    for text in decoded.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+([0-9a-fA-F]{8})\s+([?/\\\[|])\s+(.*)',text)
        if m:lines[int(m[1],16)]=(''.join(m[4].lower().split()),m[3])
    if len(lines)!=SIZE//4:raise ValueError(f'Rizin decoded {len(lines)} instead of {SIZE//4} words')
    for addr,want in EXPECTED.items():
        got,_=lines[PADDR+addr-START]
        if want not in got:raise ValueError(f'Wrong decoded instruction at {addr:#x}: {got}, expected {want}')
    if lines[PADDR+0x4a2c-START][1]!='/' or lines[PADDR+0x4a30-START][1]!='\\':
        raise ValueError('CDSP branch parse-markers not preserved')
    with tempfile.TemporaryDirectory(prefix='cdsp-l2-check-') as t:
        src=Path(t)/'l2.s';obj=Path(t)/'l2.o';src.write_text(ASM)
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(obj),str(src)],capture_output=True,text=True,check=True)
        independent=elf_text(obj)
    for offset,mapped in [(0x4a0c,0),(0x4a10,4),(0x4a14,8),(0x4a1c,12),(0x4a34,28)]:
        got=int.from_bytes(b[offset-START:offset-START+4],'little')
        independent_word=int.from_bytes(independent[mapped:mapped+4],'little')
        if (got&~0xc000)!=(independent_word&~0xc000):
            raise ValueError(f'LLVM6 scalar mismatch CDSP {offset:#x}: {got:#x} vs {independent_word:#x}')
    print('CDSP L2 sync: 48 exact source-encoded bytes, 12 Rizin V66 semantics, 5 LLVM6 independent ordinary instructions; privileged L2/SYSCFG decode-only.')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for arg in ('llvm-mc','rz-asm','rust-output','reference'):p.add_argument('--'+arg,required=True,type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.rust_output,a.reference)
