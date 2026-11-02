#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon V66 lock/thread ownership and worker callback checks."""
from pathlib import Path
import argparse,subprocess,re,importlib.util,tempfile
START=0x4db0;END=0x4e04;PHYS=0x96c01db0
EXPECTED={
0x4db0:'r19=extract(r0,#0x3,#0x0)',0x4db4:'p0=tstbit(r0,#0x1e)',
0x4db8:'r1=htid',0x4dbc:'p1=cmp.eq(r19,r1)',
0x4dc0:'p2=and(p0,p1)',0x4dc4:'if(p2.new)jump:nt0x96c01f24',
0x4dc8:'immext(##0xfe037240)',0x4dcc:'r19=##-0x1fc8dac',
0x4dd0:'r0=memw_locked(r19)',0x4dd4:'p0=tstbit(r0,r1)',
0x4dd8:'if(p0.new)jump:nt0x96c02040',0x4ddc:'p3:0=r28',
0x4de0:'immext(##0xfe0abbc0)',0x4de4:'r0=##-0x1f54408',
0x4de8:'r28=r1',0x4dec:'allocframe(sp,#0x8)',
0x4df0:'r1=add(r0,##0x68)',0x4df4:'memd(sp+##0x0)=r1:0',
0x4df8:'r1+=mpyi(r28,##0xe0)',0x4dfc:'call0x96c3251c',0x4e00:'r0=r18'
}
assert len(EXPECTED)==21

def elf_text(data):
    src=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    sp=importlib.util.spec_from_file_location('cdsp_worker_lock_elf',src)
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    return m.parse_elf32_text(data)

def assemble(tool,source):
    with tempfile.TemporaryDirectory() as d:
        a=Path(d)/'t.s';b=Path(d)/'t.o';a.write_text('.text\n.global sample\nsample:\n'+source)
        subprocess.run([str(tool),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj','-o',str(b),str(a)],
                       text=True,capture_output=True,check=True)
        return elf_text(b.read_bytes())

def check(reference,output,rz,mc):
    actual=output.read_bytes();ref=reference.read_bytes()[START:END]
    if len(actual)!=END-START or actual!=ref:
        raise ValueError('typed CDSP lock helper differs from SHA-256 pinned reference')
    result=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',actual.hex()],
                         text=True,capture_output=True,check=True)
    found={}
    for l in result.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[a-fA-F0-9]{8}\s+[?/\\\[|]\s+(.*)',l)
        if m:found[int(m[1],16)]=''.join(m[2].lower().split())
    if len(found)!=21:raise ValueError(f'expected 21 Rizin words, got {len(found)}')
    for addr,want in EXPECTED.items():
        got=found.get(PHYS+addr-START,'')
        if want not in got:raise ValueError(f'Rizin mismatch {addr:#x} {got!r} wanted {want!r}')
    count=0
    # LLVM independently assembles each non-privileged entry in this packet.
    asm='''{ r19=extract(r0,#3,#0); p0=tstbit(r0,#30) }
{ p1=cmp.eq(r19,r1) }
{ p2=and(p0,p1); if(p2.new) jump:nt target }
{ r0=memw_locked(r19) }
{ p0=tstbit(r0,r1); if(p0.new) jump:nt target }
{ p3:0=r28 }
.space 0x140
target:
{ jumpr lr }
'''
    words=assemble(mc,asm)
    for a,index in [(0x4db0,0),(0x4db4,1),(0x4dbc,2),(0x4dc0,3),(0x4dd0,5),(0x4dd4,6),(0x4ddc,8)]:
        from_llvm=words[index*4:index*4+4]
        encoded=actual[a-START:a-START+4]
        if (int.from_bytes(from_llvm,'little')&~0xc000)!=(int.from_bytes(encoded,'little')&~0xc000):
            raise ValueError(f'LLVM normal/privileged instruction mismatch {a:#x}')
        count+=1
    for file,llvm_src,diff in [
        (0x4dc4,'{ p2=and(p0,p1); if(p2.new) jump:nt target }',0x164),
        (0x4dd8,'{ p0=tstbit(r0,r1); if(p0.new) jump:nt target }',0x26c)]:
        ins=assemble(mc,llvm_src+'\n'+f'.space {diff-8}\ntarget:\n{{ jumpr lr }}\n')
        b=ins[4:8]
        if b!=actual[file-START:file-START+4]:raise ValueError(f'LLVM packet branch mismatch {file:#x}')
        count+=1
    # Two-word extended signed controller pointer.
    assignment=assemble(mc,'{ r19=##-0x1fc8dac }\n')
    if assignment[:8]!=actual[0x4dc8-START:0x4dd0-START]:raise ValueError('LLVM locked pointer mismatch')
    count+=2
    final=assemble(mc,'''{ r0=##-0x1f54408; r28=r1 }
{ allocframe(#8) }
{ r1=add(r0,#104); memd(sp+#0)=r1:0 }
{ r1+=mpyi(r28,#224); call target; r0=r18 }
.space 0x30718
target:
{ jumpr lr }
''')
    data=actual[0x4de0-START:0x4e04-START]
    if final[:len(data)] != data:
        raise ValueError(f'LLVM final frame/MPY/CALL mismatch: {final[:len(data)].hex()} != {data.hex()}')
    count+=len(data)//4
    print(f'CDSP worker lock {len(actual)} exact bytes; {len(found)} Rizin V66 words; {count} LLVM6 word-level validations, privileged HTID Rizin only')

if __name__=='__main__':
    a=argparse.ArgumentParser(description=__doc__)
    for name in ('reference','rust-output','rz-asm','llvm-mc'):
        a.add_argument('--'+name,required=True,type=Path)
    p=a.parse_args();check(p.reference,p.rust_output,p.rz_asm,p.llvm_mc)
