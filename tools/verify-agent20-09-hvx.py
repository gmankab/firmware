#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent V66/HVX startup cross-check: original SHA, Rizin semantics,
LLVM6 operand encodings and original instruction packet markers.

Rizin checks *all* 68 words. LLVM6 v60+hvx independently assembles the
scalar and supported HVX forms; it doesn't support zextract into V16/V17
and does not independently certify the privileged target's behavior.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess, tempfile

START=0x5880
END=0x5990
PC=0x96c0_2880
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'

def expect_semantics():
    lines=[
        'R12 = SSR',
        'R12 = setbit(R12,#0x1f)',
        'R13 = ##0x0',
        'R14 = add(R13,##0x4)',
        'R12 = insert(R14,#0x3,#0x1b)',
        'SSR = R12',
        'immext(##0xfe017100)',
        'R0 = ##-0x1fe8f00',
        'R14 = ##0x11f8',
        'R0 += mpyi(R13,R14)',
        'R2 = ##0x0',
        'vmem(R0++#0x1) = V0',
        'V0 = vsplat(R2)',
        'vmem(R0++#0x1) = V1',
        'immext(##0x1010100)',
        'R2 = ##0x1010101',
        'vmem(R0++#0x1) = V2',
        'V1 = vsplat(R2)',
        'vmem(R0++#0x1) = V3',
        'V3 = vmux(Q0,V1,V0)',
        'vmem(R0++#0x1) = V4',
        'R2 = ##0x1',
        'vmem(R0++#0x1) = V5',
        'V1.w = vasl(V1.w,R2)',
        'vmem(R0++#0x1) = V6',
        'V2 = vmux(Q1,V1,V0)',
        'vmem(R0++#0x1) = V7',
        'V3 = vor(V3,V2)',
        'vmem(R0++#0x1) = V8',
        'V1.h = vasl(V1.h,R2)',
        'vmem(R0++#0x1) = V9',
        'V2 = vmux(Q2,V1,V0)',
        'vmem(R0++#0x1) = V10',
        'V3 = vor(V3,V2)',
        'vmem(R0++#0x1) = V11',
        'V1.h = vasl(V1.h,R2)',
        'vmem(R0++#0x1) = V12',
        'V2 = vmux(Q3,V1,V0)',
        'vmem(R0++#0x1) = V13',
        'V3 = vor(V3,V2)',
    ]
    lines += [f'vmem(R0++#0x1) = V{i}' for i in range(14,28)]
    lines += [
        'R2 = ##0x0',
        'vmem(R0++#0x1) = V28',
        'V16 = zextract(R2)',
        'vmem(R0++#0x1) = V29',
        'R2 = add(R2,##0x80)',
        'vmem(R0++#0x1) = V30',
        'V17 = zextract(R2)',
        'vmem(R0++#0x1) = V31',
        'vmem(R0++#0x1) = V16',
        'vmem(R0++#0x1) = V17',
        'vmem(R0+#0x0) = V3',
        'R13 = add(R13,##0x1)',
        'if (!cmp.eq(R13.new,#0x4)) jump:nt 0x96c0288c',
        'jumpr LR',
    ]
    assert len(lines)==68,len(lines)
    return lines

def norm(v:str)->str:
    return ''.join(v.lower().split()).replace('r31','lr')

def llvm_bytes(mc:Path,source:str)->bytes:
    spec=importlib.util.spec_from_file_location('elf_reader', Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
    assert spec and spec.loader
    helper=importlib.util.module_from_spec(spec);spec.loader.exec_module(helper)
    with tempfile.TemporaryDirectory(prefix='cdsp-hvx-llvm6-') as td:
        d=Path(td);p=d/'code.s';p.write_text('.text\n.globl start\nstart:\n'+source+'\n')
        call=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-mattr=+hvx','-filetype=obj','-o',str(d/'code.o'),str(p)],capture_output=True,text=True)
        if call.returncode:raise ValueError(f'LLVM6 failed assembling {source!r}: {call.stderr}')
        return helper.parse_elf32_text((d/'code.o').read_bytes())

def check(output:Path,reference:Path,rz:Path,mc:Path):
    ref=reference.read_bytes();generated=output.read_bytes()
    if hashlib.sha256(ref).hexdigest()!=SHA:raise ValueError('CDSP revision does not match pinned SHA')
    if len(generated)!=END-START or generated!=ref[START:END]:
        raise ValueError('HVX fragment not byte-identical to original CDSP')
    decoded=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',generated.hex()],check=True,capture_output=True,text=True).stdout
    pairs=[]
    for line in decoded.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:pairs.append((int(m[1],16),m[2].split('<endloop')[0]))
    expected=expect_semantics()
    if len(pairs)!=len(expected):raise ValueError(f'Rizin decoded {len(pairs)} words, expected {len(expected)}')
    for i,((pc,semantic),want) in enumerate(zip(pairs,expected)):
        if pc!=PC+4*i or norm(semantic)!=norm(want):raise ValueError(f'Rizin differs at word {i}: {semantic!r} != {want!r}')
    # Independent LLVM6 assembly: compare instruction fields excluding only
    # packet parse bits, since LLVM can choose different legal packet grouping.
    samples={
         1:'{r12=setbit(r12,#31)}',
         2:'{r13=#0}',
         3:'{r14=add(r13,#4)}',
         4:'{r12=insert(r14,#3,#27)}',
         8:'{r14=#4600}',
         9:'{r0+=mpyi(r13,r14)}',
        10:'{r2=#0}',
        12:'{v0=vsplat(r2)}',
        17:'{v1=vsplat(r2)}',
        19:'{v3=vmux(q0,v1,v0)}',
        21:'{r2=#1}',
        23:'{v1.w=vasl(v1.w,r2)}',
        25:'{v2=vmux(q1,v1,v0)}',
        27:'{v3=vor(v3,v2)}',
        29:'{v1.h=vasl(v1.h,r2)}',
        31:'{v2=vmux(q2,v1,v0)}',
        33:'{v3=vor(v3,v2)}',
        35:'{v1.h=vasl(v1.h,r2)}',
        37:'{v2=vmux(q3,v1,v0)}',
        39:'{v3=vor(v3,v2)}',
        54:'{r2=#0}',
        58:'{r2=add(r2,#128)}',
        65:'{r13=add(r13,#1)}',
        67:'{jumpr lr}',
    }
    for i in list(range(32,40))+list(range(40,54))+list(range(55,64)):
        if expected[i].startswith('vmem('):
            v=int(re.search(r'V(\d+)',expected[i]).group(1))
            samples[i]=f'{{vmem(r0++#1)=v{v}}}'
    for i in [11,13,16,18,20,22,24,26,28,30,32,34,36,38]:
        v=int(re.search(r'V(\d+)',expected[i]).group(1))
        samples[i]=f'{{vmem(r0++#1)=v{v}}}'
    for i in [41,42,43,44,45,46,47,48,49,50,51,52,53]:
        v=int(re.search(r'V(\d+)',expected[i]).group(1))
        samples[i]=f'{{vmem(r0++#1)=v{v}}}'
    samples[64]='{vmem(r0+#0)=v3}'
    samples[62]='{vmem(r0++#1)=v16}'
    samples[63]='{vmem(r0++#1)=v17}'
    samples[60] = None # LLVM6 cannot independently assemble zextract for V17.
    # Multiword immediate assignments at indexes 6:7 and 14:15.
    multi=[('{r0=##0xfe017100}',[6,7]),('{r2=##0x1010101}',[14,15])]
    assembled={}; matched=set()
    for i,source in samples.items():
        if source is None:continue
        if source not in assembled:assembled[source]=llvm_bytes(mc,source)
        data=assembled[source]
        if len(data)!=4:raise ValueError(f'LLVM sample at {i}: expected 4 bytes, got {len(data)}')
        actual=int.from_bytes(data,'little')&~0xc000
        want=int.from_bytes(generated[4*i:4*i+4],'little')&~0xc000
        if actual!=want:raise ValueError(f'LLVM operand mismatch at word {i}, LLVM={actual:#x}, original={want:#x}, source {source}')
        matched.add(i)
    for source,positions in multi:
        data=llvm_bytes(mc,source)
        if len(data)!=4*len(positions):raise ValueError('LLVM extended immediate length mismatch')
        for j,i in enumerate(positions):
            got=int.from_bytes(data[4*j:4*j+4],'little')&~0xc000
            want=int.from_bytes(generated[4*i:4*i+4],'little')&~0xc000
            if got!=want:raise ValueError(f'LLVM immediate at {i}: {got:#x} != {want:#x}')
            matched.add(i)
    # Verify branch's backwards displacement with LLVM at the same relative
    # packet offset: earlier is 0xf8 before the producer packet.
    branch_source='earlier:\n{nop}\n.space 244\n{r13=add(r13,#1)\n if(!cmp.eq(r13.new,#4)) jump:nt earlier}'
    branch=llvm_bytes(mc,branch_source)
    if len(branch)!=256:raise ValueError(f'LLVM backward branch expected 256 bytes, got {len(branch)}')
    for j,i in enumerate([65,66]):
        got=int.from_bytes(branch[248+4*j:252+4*j],'little')&~0xc000
        want=int.from_bytes(generated[4*i:4*i+4],'little')&~0xc000
        if got!=want:raise ValueError(f'LLVM branch compare at word {i}: {got:#x} vs {want:#x}')
        matched.add(i)
    if len(matched)<59:raise ValueError(f'Only {len(matched)} independently LLVM-assembled words')
    print(f'CDSP HVX initialization: {len(generated)} reference-exact bytes, {len(pairs)} Rizin V66 semantic words, {len(matched)} LLVM6 independently matched operand words. Zextract V16/V17 and privileged forms not fully LLVM-proven.')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for arg in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+arg,type=Path,required=True)
    a=p.parse_args();check(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
