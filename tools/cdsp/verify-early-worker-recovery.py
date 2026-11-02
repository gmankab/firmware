#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Corroborate Agent 09's 140-byte controller worker recovery section.

The Rust writer independently emits checked V66 operands. This tool checks
exact bytes against a pinned private CDSP revision, independently decodes all
35 words with Rizin V66, and assembles all 31 nonprivileged words with LLVM6
from original assembly *source* (not OEM object bytes). Privileged SGP0/HTID/
ELR encodings are validated by Rizin and exact reference comparison only.
"""
import argparse,hashlib,importlib.util,re,struct,subprocess,tempfile
from pathlib import Path

START=0x50f8; END=0x5184; PHYSICAL=0x96c0_20f8
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
 'allocframe(SP,#0x10)', 'memd(SP+##0x8)=R19:18', 'call0x96c32430',
 'R1=setbit(R1,#0x1d)', 'R28=R0', 'memw(SP+##0x0)=R0', 'R1=SGP0',
 'R0=R28','P0=cmp.eq(R0,#0x0);R1=memw(R1+#0x0)',
 'immext(##0x30340)', 'if(P0)call0x96c32464','R0=add(R0,##0x8)',
 'R9=SGP0','R2=memw(R9+##0x0)', 'R5:4=memd(R2+##0x18)',
 'R7:6=memd(R2+##0x20)','ELR=R4', 'R7:6=memd(R2+##0x28)',
 'memd(R9+##0x50)=R7:6','immext(##0xfe0abbc0)', 'R0=##-0x1f54408',
 'memd(R9+##0x48)=R7:6','R3=HTID','R1=add(R0,##0x68)',
 'R0=R9','R1+=mpyi(R3,##0xe0)', 'immext(##0x303c0)',
 'if(P0)call0x96c3251c', 'immext(##0x35200)',
 'if(P0)call0x96c3736c','R0=memw(SP+##0x0)',
 'P0=cmp.eq(R0,#0x0);if(!P0.new)jump:nt0x96c02180',
 'R19:18=memd(SP+##0x8)','LR:FP=dealloc_return(FP)',
 'jump0x96c02180',
]
assert len(EXPECTED)==35
PRIVILEGED={6,12,16,22}
NORMALIZED_SAMPLE_INDICES=[0,1,3,4,5,7,8,11,13,14,15,17,18,19,20,21,23,24,25,30,31,32,33,34]
ASM_SAMPLE='''
{ allocframe(#16) }
{ memd(sp+#8) = r19:18 }
{ r1 = setbit(r1,#29) }
{ r28 = r0 }
{ memw(sp+#0) = r0 }
{ r0 = r28; p0 = cmp.eq(r0,#0); r1 = memw(r1+#0) }
{ r0 = add(r0,#8) }
{ r2 = memw(r9+#0) }
{ r5:4 = memd(r2+#24) }
{ r7:6 = memd(r2+#32) }
{ r7:6 = memd(r2+#40) }
{ memd(r9+#80) = r7:6 }
{ r0 = ##0xfe0abbf8 }
{ memd(r9+#72) = r7:6 }
{ r1 = add(r0,#104) }
{ r0 = r9 }
{ r1 += mpyi(r3,#224) }
{ r0 = memw(sp+#0) }
{ p0=cmp.eq(r0,#0); if(!p0.new) jump:nt target }
{ r19:18 = memd(sp+#8) }
{ dealloc_return }
target:
{ jump target }
'''
# Full packet instructions with verified packet-relative call displacements.
CALL_PACKETS={
 'direct':('''{ call callback
  r1 = setbit(r1,#29)
  r28 = r0 }
.space (0x30330-12)
callback:
{ jumpr lr }
''',[2,3,4]),
 'a':('''{ if(p0) call callback
  r0 = add(r0,#8) }
.space (0x30348-12)
callback:
{ jumpr lr }
''',[9,10,11]),
 'b':('''{ r1 += mpyi(r3,#224)
  if(p0) call callback }
.space (0x303c0-12)
callback:
{ jumpr lr }
''',[25,26,27]),
 'c':('''{ if(p0) call callback }
.space (0x35204-8)
callback:
{ jumpr lr }
''',[28,29]),
}

def text_from_obj(binary:bytes)->bytes:
    spec=importlib.util.spec_from_file_location('cdsp_elf',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
    assert spec and spec.loader
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod.parse_elf32_text(binary)

def llvm_assemble(mc:Path,src:str)->bytes:
    with tempfile.TemporaryDirectory(prefix='cdsp-recovery-llvm6-') as d:
        d=Path(d)
        (d/'code.s').write_text('.text\n.global entry\nentry:\n'+src)
        process=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60',
          '-filetype=obj','-o',str(d/'code.o'),str(d/'code.s')],text=True,capture_output=True)
        if process.returncode:raise ValueError('LLVM6 rejected independent source: '+process.stderr)
        return text_from_obj((d/'code.o').read_bytes())

def verify(ours:Path,ref:Path,decoder:Path,mc:Path):
    orig=ref.read_bytes();data=ours.read_bytes()
    if hashlib.sha256(orig).hexdigest()!=SHA:raise ValueError('CDSP reference revision differs from pinned SHA256')
    if len(data)!=END-START or data!=orig[START:END]:raise ValueError('source-generated recovery differs from original CDSP')
    decoded=subprocess.run([str(decoder),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],
                           text=True,capture_output=True,check=True)
    actual=[]
    for line in decoded.stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:actual.append((int(m.group(1),16),''.join(m.group(2).lower().split())))
    if len(actual)!=35:raise ValueError(f'Rizin produced {len(actual)} instead of 35 V66 words')
    for i,((pc,meaning),item) in enumerate(zip(actual,EXPECTED)):
        if pc!=PHYSICAL+i*4 or ''.join(item.lower().split()) not in meaning:
            raise ValueError(f'Decoder semantic discrepancy at CDSP file {START+4*i:#x}: {meaning!r} vs {item!r}')
    llvm_indices=set()
    sample=llvm_assemble(mc,ASM_SAMPLE)
    if len(sample)<len(NORMALIZED_SAMPLE_INDICES)*4:raise ValueError('independent LLVM sample too short')
    for k,i in enumerate(NORMALIZED_SAMPLE_INDICES):
        a=int.from_bytes(sample[k*4:k*4+4],'little')&~0xc000
        b=int.from_bytes(data[i*4:i*4+4],'little')&~0xc000
        if a!=b:raise ValueError(f'Independent LLVM scalar opcode mismatch for recovery word {i}: {a:#x} vs {b:#x}')
        llvm_indices.add(i)
    for name,(program,indices) in CALL_PACKETS.items():
        assembled=llvm_assemble(mc,program)
        if len(assembled)<len(indices)*4:raise ValueError('LLVM call packet too short '+name)
        for j,i in enumerate(indices):
            if assembled[j*4:j*4+4]!=data[i*4:i*4+4]:
                raise ValueError(f'LLVM {name} packet differs from reference at word {i}')
            llvm_indices.add(i)
    if llvm_indices!=set(range(35))-PRIVILEGED:
        raise ValueError('LLVM coverage unexpectedly missing ordinary instructions: '+str(sorted(set(range(35))-PRIVILEGED-llvm_indices)))
    print(f'CDSP recovery: {len(data)} exact source-built bytes, 35 Rizin V66 words, '
          f'{len(llvm_indices)} independently LLVM6 assembled ordinary words; '
          'four privileged SGP0/ELR/HTID words decoder/reference verified only')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    p.add_argument('--rust-output',required=True,type=Path)
    p.add_argument('--reference',required=True,type=Path)
    p.add_argument('--rz-asm',required=True,type=Path)
    p.add_argument('--llvm-mc',required=True,type=Path)
    a=p.parse_args()
    verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
