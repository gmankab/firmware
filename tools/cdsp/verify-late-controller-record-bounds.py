#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6 + Rizin check of CDSP indexed-record capacity guards.

Original code fuses producer R2.new/R3.new comparisons into branch words.
LLVM6 can reorder the word load and emit explicit P1 predicates instead.
Report this difference; do not imply exact independent binary assembly for
those producer/consumer packets. The Rust builder and reference must be exact.
"""
from __future__ import annotations
import argparse, importlib.util, os, subprocess, tempfile
from pathlib import Path

START=0x56a74
END=0x56aa8
PHYS=0x96cc8a74

def as_words(b:bytes)->list[int]:
    if len(b)%4:raise ValueError('misaligned packet output')
    return [int.from_bytes(b[i:i+4],'little') for i in range(0,len(b),4)]

def text_section(raw:bytes)->bytes:
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('hexagon_elf32',p)
    if spec is None or spec.loader is None:raise ValueError('ELF32 parser missing')
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module.parse_elf32_text(raw)

def disasm(rz:Path,b:bytes)->str:
    env=os.environ.copy()
    # The supplied Rizin binary dynamically links the self-contained Ubuntu
    # runtime archives bundled with the toolchain; no private data is needed.
    bundled=next((parent/'ubuntu-runtime/build-deps-flat' for parent in rz.resolve().parents
                  if (parent/'ubuntu-runtime/build-deps-flat').is_dir()),None)
    if bundled is not None:env['LD_LIBRARY_PATH']=str(bundled)+':'+env.get('LD_LIBRARY_PATH','')
    cmd=[str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',b.hex()]
    run=subprocess.run(cmd,env=env,capture_output=True,text=True,check=True)
    lines=[x for x in run.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=len(b)//4 or any('invalid' in x.lower() for x in lines):
        raise ValueError('Rizin V66 did not decode all words')
    return '\n'.join(lines).lower().replace(' ','')

def verify(llvm:Path,rz:Path,generated:Path,ref:Path,prefix:Path|None):
    source=generated.read_bytes(); original=ref.read_bytes()
    if len(source)!=52 or source!=original[START:END]:
        raise ValueError('52 source-encoded CDSP record bounds bytes do not match pinned reference')
    if prefix is not None and (prefix.read_bytes()!=original[0x56000:END] or prefix.stat().st_size!=2728):
        raise ValueError('contiguous 2728-byte RWE prefix differs from original')
    asm=Path(__file__).with_name('late-controller-record-bounds-semantic.s')
    with tempfile.TemporaryDirectory(prefix='cdsp-record-bounds-') as temp:
        obj=Path(temp)/'bounds.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(asm),'-o',str(obj)],capture_output=True,text=True,check=True)
        compiled=text_section(obj.read_bytes())[:52]
    if len(compiled)!=52:raise ValueError('independent 13-word fixture length wrong')
    a,b=as_words(source),as_words(compiled)
    for i in [0,1,5,7,12]:
        if a[i]!=b[i]:raise ValueError(f'LLVM operand differs at exact word {i}')
    for i in [4,6,8,10,11]:
        if (a[i]&~0xc000)!=(b[i]&~0xc000):
            raise ValueError(f'LLVM differs beyond packet parse bits at word {i}')
    src,ind=disasm(rz,source),disasm(rz,compiled)
    for name in ['r4=memw(r2+#0x10)','r5=memw(r16+#0x10)','r3=memw(r5+r23<<#0x0)',
                 'r6+=lsr(r4,#0xc)','r3+=lsr(r5,#0xc)',
                 'r3=##0x8;r0=r17','call0x96cd0480','jump:t0x96cc8aa8']:
        if name not in src or name not in ind:
            raise ValueError('operand, target, or Rizin packet decode differs: '+name)
    for name in ['if(cmp.gtu(r3,r2.new))','if(cmp.gtu(r6,r3.new))']:
        if name not in src:raise ValueError('missing exact fused .new guard: '+name)
    for name in ['p1=cmp.gtu(r3,r2)','p1=cmp.gtu(r6,r3)']:
        if name not in ind:raise ValueError('LLVM6 explicit predicate operand mismatch: '+name)
    print('PASS: 52 source-encoded CDSP bytes, 13 original V66 packet words, zero reference differences')
    print('PASS: LLVM6 semantic instruction fixture; five exact words and five parse-marker-only changes')
    print('PASS: two original fused new-value branches versus LLVM6 explicit P1 guards to 0x96cc8aa8')
    if prefix:print('PASS: contiguous RWE layout 2728 bytes = 2672 executable + 56 alignment')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ('llvm-mc','rz-asm','generated','reference','prefix'):
        p.add_argument('--'+name,type=Path,required=name!='prefix')
    ns=p.parse_args()
    verify(ns.llvm_mc,ns.rz_asm,ns.generated,ns.reference,ns.prefix)
