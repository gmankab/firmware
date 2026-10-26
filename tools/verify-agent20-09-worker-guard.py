#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent V66 check for worker GP guard and duplex zeroing packets."""
from pathlib import Path
import argparse,subprocess,re,importlib.util,tempfile
START=0x4d78;END=0x4db0;PHYS=0x96c0_1d78
EXPECTED=[
'immext(##0xfe0abbc0)','r0=memw(gp+##0xfe0abbf8)',
'immext(##0xfe0ac080)','r1=memw(gp+##0xfe0ac0a0)',
'p0=cmp.eq(r1,#0x0);if(p0.new)jump:t0x96c01db0',
'immext(##0xfe0a5dc0)','r1=memw(gp+##0xfe0a5ddc)',
'p0=cmp.eq(r1,#0x0);if(!p0.new)jump:nt0x96c01db0',
'immext(##0xfe037ac0)','r1=memw(gp+##0xfe037ad0)',
'p0=cmp.eq(r1,#0x0);if(p0.new)jump:t0x96c01db0',
'memw(r1+#0x24)=#0;memw(r1+#0x20)=#0',
'memw(r1+#0x10)=#0;memw(r1+#0x14)=#0',
'memw(r1+##0x4)=clrbit(#0x15)',
]
assert len(EXPECTED)==14

def llvm(compiler:Path,code:str)->bytes:
    with tempfile.TemporaryDirectory(prefix='cdsp-guard-llvm-') as d:
        path=Path(d);a=path/'a.s';o=path/'b.o';a.write_text('.text\n.global sample\nsample:\n'+code)
        subprocess.run([str(compiler),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(o),str(a)],capture_output=True,text=True,check=True)
        loader=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('cdsp_guard_elf',loader)
        mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
        return mod.parse_elf32_text(o.read_bytes())

def check(reference:Path,output:Path,rz:Path,compiler:Path)->None:
    firmware=reference.read_bytes();actual=output.read_bytes();ref=firmware[START:END]
    if len(actual)!=56 or ref!=actual:raise ValueError('worker guard differs from pinned CDSP reference')
    run=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',actual.hex()],capture_output=True,text=True,check=True)
    ins={}
    for line in run.stdout.splitlines():
        m=re.match(r'^0x([a-fA-F0-9]+)\s+4\s+[a-fA-F0-9]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:ins[int(m[1],16)]=''.join(m[2].lower().split())
    if len(ins)!=14:raise ValueError(f'Rizin decoded {len(ins)} of 14 words')
    for i,want in enumerate(EXPECTED):
        text=ins.get(PHYS+4*i,'')
        if want not in text:raise ValueError(f'Rizin mismatch file {START+4*i:#x}: {text} != {want}')
    for file,mode,diff in [(0x4d88,'p0=cmp.eq(r1,#0); if(p0.new) jump:t target',0x28),
                           (0x4d94,'p0=cmp.eq(r1,#0); if(!p0.new) jump:nt target',0x1c),
                           (0x4da0,'p0=cmp.eq(r1,#0); if(p0.new) jump:t target',0x10)]:
        word=llvm(compiler,'{ '+mode+' }\n'+f'.space {diff-4}\ntarget:\n{{ jumpr lr }}\n')[:4]
        got=actual[file-START:file-START+4]
        if (int.from_bytes(word,'little')&~0xc000)!=(int.from_bytes(got,'little')&~0xc000):
            raise ValueError(f'LLVM predicated-branch mismatch at {file:#x}')
    code='''{ memw(r1+#36)=#0; memw(r1+#32)=#0 }
{ memw(r1+#16)=#0; memw(r1+#20)=#0 }
{ memw(r1+#4)=clrbit(#21) }
'''
    b=llvm(compiler,code)
    if b!=actual[0x4da4-START:0x4db0-START]:
        raise ValueError('LLVM duplex pair/atomic bit-clear disagree')
    print('CDSP guard: 56 byte exact; 14 Rizin V66 words; 6 LLVM6 instruction encodings checked; GP extended loads Rizin/reference only')

if __name__=='__main__':
    a=argparse.ArgumentParser(description=__doc__)
    for key in ('reference','rust-output','rz-asm','llvm-mc'):
        a.add_argument('--'+key,required=True,type=Path)
    c=a.parse_args();check(c.reference,c.rust_output,c.rz_asm,c.llvm_mc)
