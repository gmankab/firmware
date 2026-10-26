#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent CDSP V66 worker thread-init semantics and regular LLVM6 ISA.

The checked Rust emitter is the generator; this script verifies against the
pinned OEM reference, Rizin's separate V66 decoder, and LLVM6 generated forms.
Not a device execution or source-level equivalence proof.
"""
from pathlib import Path
import argparse,importlib.util,re,os,subprocess,tempfile
START=0x4d24;END=0x4d78;PHYS=0x96c01d24
EXPECTED=[
 'immext(##0xfe024d00)', 'r0=memw(gp+##0xfe024d00)',
 'if(cmp.eq(r0.new,#0x1))jump:nt0x96c01fd0',
 'immext(##0x0)','r0=##0x1b',
 'immext(##0xfe000040)','memw(r1=##0xfe000044)=r0.new',
 'dccleana(r1)','r1=htid',
 'immext(##0xfe0371c0)','sp=##-0x1fc8e18',
 'sp=addasl(sp,r1,#0x5)','r28=p3:0','r19=r2',
 'immext(##0xa280)','call0x96c0c004','r2=r19','r0=##0x5',
 'immext(##0xfe024d00)','memw(r1=##0xfe024d00)=r0.new','dccleana(r1)'
]
assert len(EXPECTED)==21

def elf_text(data):
    src=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_elf_decode_28',src)
    module=importlib.util.module_from_spec(spec);assert spec.loader;spec.loader.exec_module(module)
    return module.parse_elf32_text(data)

def llvm(mctool:Path,source:str)->bytes:
    with tempfile.TemporaryDirectory() as tmp:
        script=Path(tmp)/'sample.s';object_file=Path(tmp)/'sample.o'
        script.write_text('.text\n.global sample\nsample:\n'+source)
        subprocess.run([str(mctool),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
                        '-o',str(object_file),str(script)],capture_output=True,text=True,check=True)
        return elf_text(object_file.read_bytes())

def verify(llvm_tool,rz_tool,reference,rebuilt):
    actual=rebuilt.read_bytes();origin=reference.read_bytes()
    if len(actual)!=END-START or actual!=origin[START:END]:
        raise ValueError('worker thread typed output differs from original CDSP executable')
    decoded=subprocess.run([str(rz_tool),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',actual.hex()],
                           capture_output=True,text=True,check=True)
    meanings={}
    for line in decoded.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:meanings[int(m[1],16)]=''.join(m[2].lower().split())
    if len(meanings)!=len(EXPECTED):raise ValueError(f'Rizin decoded {len(meanings)} of 21 words')
    for i,want in enumerate(EXPECTED):
        found=meanings.get(PHYS+4*i,'')
        if want not in found:raise ValueError(f'wrong V66 semantic at {START+4*i:#x}: {found}; wanted {want}')
    # Six standalone instructions assembled independently; privileged HTID
    # cannot be assembled by the public LLVM6 tool and stays Rizin-only.
    source='''{ sp=addasl(sp,r1,#5) }
{ r28=p3:0 }
{ r19=r2 }
{ r2=r19 }
{ r0=#5 }
{ dccleana(r1) }
'''
    assembled=llvm(llvm_tool,source)
    offsets=[0x4d50,0x4d54,0x4d58,0x4d64,0x4d68,0x4d40]
    for j,addr in enumerate(offsets):
        word=assembled[4*j:4*j+4]
        got=actual[addr-START:addr-START+4]
        if addr in (0x4d54,0x4d68):
            # These instructions are packet-internal in the reference; the
            # exact condition/operands must match after masking parse bits.
            if (int.from_bytes(word,'little')&~0xc000)!=(int.from_bytes(got,'little')&~0xc000):
                raise ValueError(f'wrong operand encoding at {addr:#x}')
        elif word!=got:raise ValueError(f'wrong LLVM independent encoding at {addr:#x}')
    print(f'CDSP worker thread {len(actual)} exact bytes; {len(EXPECTED)} Rizin V66 semantic words; {len(offsets)} independently LLVM6 verified operands/words; privileged system registers Rizin only')

if __name__=='__main__':
    a=argparse.ArgumentParser()
    for name in ('llvm-mc','rz-asm','reference','rust-output'):
        a.add_argument('--'+name,type=Path,required=True)
    p=a.parse_args();verify(p.llvm_mc,p.rz_asm,p.reference,p.rust_output)
