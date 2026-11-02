#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check 80 source-generated CDSP controller bytes against independent LLVM6
semantic assembly, Rizin Hexagon V66 packets and the unchanged original MBN.
Original executable words are never emitted by this verification script.
"""
import argparse,importlib.util,os,subprocess,tempfile,re
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'

START,END=0x56734,0x56784
PHYSICAL=0x96cc8734

def decode(exe,content,origin):
    env=os.environ.copy()
    cp=subprocess.run([str(exe),'-a','hexagon','-b','32','-o',hex(origin),'-D',content.hex()],
                      capture_output=True,text=True,env=env,check=True)
    lines=[x for x in cp.stdout.splitlines() if x.startswith('0x')]
    if not lines or any('invalid' in x.lower() for x in lines):
        raise RuntimeError('unsupported Hexagon instruction or packet: '+cp.stdout)
    pat=re.compile(r'^0x[0-9a-f]+\s+\d+\s+[a-f0-9]{8}\s+\S\s+(.*)$',re.I)
    ops=[]
    for line in lines:
        m=pat.match(line)
        if m is None:raise RuntimeError('unexpected Rizin line: '+line)
        if 'immext(' not in m[1].lower():ops.append(re.sub(r'\s+','',m[1]).lower())
    return lines,ops

def verify(llvm,rz,reference,generated,prefix):
    original=reference.read_bytes()
    span=generated.read_bytes()
    full=prefix.read_bytes()
    if len(span)!=END-START or span!=original[START:END]:raise RuntimeError('new 80-byte source region is not exact')
    if len(full)!=0x784 or full!=original[0x56000:END] or full[-len(span):]!=span:
        raise RuntimeError('1,924-byte joined RWE prefix differs from original')
    original_lines,orig_ops=decode(rz,span,PHYSICAL)
    if len(original_lines)!=20:raise RuntimeError(f'expected 20 original words, saw {len(original_lines)}')
    text='\n'.join(original_lines).lower().replace(' ','')
    for op in ['r2=memw(r19+#0x0);r3=memw(r20+#0x0)', 'call0x96ccaa38',
               'r19=r0;r2=memw(r19+#0x0)', 'r3=memw(gp+##0xfe0c6680)',
               'r22=insert(r21,#0x14,#0x0)', 'r23=lsr(r21,#0xc)']:
        if op not in text:raise RuntimeError('missing Rizin independently decoded operation '+op)
    with tempfile.TemporaryDirectory(prefix='cdsp-mask-llvm6-') as d:
        fixture=SOURCE_DIR / 'late-controller-mask-setup.s'
        obj=Path(d)/'source.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60', '-filetype=obj',str(fixture),'-o',str(obj)],
                       capture_output=True,text=True,check=True)
        entry=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('cdsp_origin',entry)
        assert spec and spec.loader
        m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
        asm=m.parse_elf32_text(obj.read_bytes())[0x734:]
    llvm_lines,llvm_ops=decode(rz,asm,PHYSICAL)
    if len(asm)!=76 or len(llvm_lines)!=19:
        raise RuntimeError('LLVM fixture should assemble 19 words (one short GP access), got '+str(len(asm)))
    # LLVM6 chooses a one-word shorter GP-relative access and different packet
    # grouping, but must preserve non-extension operation sequence exactly.
    # LLVM6 contracts the GP-relative address to its low 16 bits, whereas
    # the OEM explicitly prefixes the full 32-bit address with immext. The
    # spelling is not fully equivalent in the abstract without a known GP
    # base. Check the low field separately; all *other* operations must agree.
    expected_gp='r3=memw(gp+##0xfe0c6680)'
    llvm_gp='r3=memw(gp+##0x6680)'
    if orig_ops.count(expected_gp)!=1 or llvm_ops.count(llvm_gp)!=1:
        raise RuntimeError('expected exactly one original forced-extended GP and one LLVM short GP form')
    llvm_normalized=[expected_gp if op==llvm_gp else op for op in llvm_ops]
    if orig_ops!=llvm_normalized:
        raise RuntimeError('LLVM semantic mismatch outside the known GP-address contraction: '+repr(list(zip(orig_ops,llvm_ops))))
    equal=sum(span[i:i+4]==asm[i:i+4] for i in range(0,36,4))
    if equal<5:raise RuntimeError('LLVM source unexpectedly changed prefix instruction fields')
    print(f'PASS {len(span)} generated source bytes and {len(full)} joined-prefix file bytes, zero differences')
    print(f'PASS Rizin V66 {len(original_lines)} original instruction words; exact call targets and packet structure')
    print(f'PASS LLVM6 independently reconstructed {len(llvm_lines)} words; {equal}/9 aligned prefix words unchanged, one shortened GP form')
    print('NOTE shortened GP address does not independently prove the original full GP effective address; Rizin verifies original, full-byte LLVM identity not claimed')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for k in ['llvm-mc','rz-asm','reference','generated','prefix']:
        p.add_argument('--'+k,type=Path,required=True)
    a=p.parse_args()
    verify(a.llvm_mc,a.rz_asm,a.reference,a.generated,a.prefix)
