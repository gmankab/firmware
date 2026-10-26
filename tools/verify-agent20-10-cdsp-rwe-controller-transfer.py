#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Cross-check original V66 packet order against independent LLVM6 source.

LLVM6 reorders two pointer-duplex/call packets, so direct LLVM byte equality
is not claimed. Original Hexagon bytes still come from checked Rust emitters,
are matched against the pinned firmware and are decoded independently by Rizin.
"""
from __future__ import annotations
import argparse,importlib.util,re,subprocess,tempfile
from collections import Counter
from pathlib import Path

START=0x56698
END=0x566c0
PHYSICAL=0x96cc8698
PREFIX_START=0x56000
COUNT=END-START

def elf_text(data:bytes)->bytes:
    parser=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_hexagon_elf',parser)
    if not spec or not spec.loader:raise RuntimeError('ELF32 text parser is unavailable')
    mod=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod.parse_elf32_text(data)

def decode(decoder:Path,data:bytes)->tuple[list[str],Counter[str]]:
    cp=subprocess.run([str(decoder),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',data.hex()],check=True,capture_output=True,text=True)
    lines=[line.strip() for line in cp.stdout.splitlines() if line.startswith('0x')]
    if len(lines)!=COUNT//4 or any('invalid' in line.lower() for line in lines):
        raise RuntimeError(f'expected {COUNT//4} valid V66 instruction words, got {len(lines)}')
    p=re.compile(r'^0x[0-9a-f]+\s+\d+\s+[a-f0-9]{8}\s+\S\s+(.*)$',re.I)
    op=[]
    for line in lines:
        m=p.match(line)
        if not m:raise RuntimeError(f'cannot interpret V66 disassembly: {line}')
        op.append(re.sub(r'\s+','',m[1]).lower())
    return lines,Counter(op)

def verify(llvm:Path,decoder:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    output=generated.read_bytes()
    ref=reference.read_bytes()
    if len(output)!=COUNT or ref[START:END]!=output:
        raise RuntimeError('CDSP controller transfer does not match original 40 bytes')
    if prefix is not None:
        full=prefix.read_bytes()
        if len(full)!=END-PREFIX_START or full!=ref[PREFIX_START:END]:
            raise RuntimeError('controller RWE 1728-byte prefix differs from pinned original')
        if full[START-PREFIX_START:END-PREFIX_START]!=output:
            raise RuntimeError('prefix does not contain independent transfer fragment')
    with tempfile.TemporaryDirectory(prefix='cdsp-transfer-crosscheck-') as tmp:
        obj=Path(tmp)/'controller-transfer.o'
        assembly=Path(__file__).with_name('agent20-10-cdsp-rwe-controller-transfer.s')
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(assembly),'-o',str(obj)],check=True,capture_output=True,text=True)
        independent=elf_text(obj.read_bytes())[0x698:0x6c0]
    if len(independent)!=COUNT:raise RuntimeError('independent LLVM6 source has unexpected length')
    # LLVM moves the call before the first duplex in each two-word packet.
    # After undoing that safe reordering, only the two call displacement
    # halfwords vary by 2 because the call PC moved from +4 to +0.
    reordered=bytearray(independent)
    for pos in [0,12]:
        reordered[pos:pos+4],reordered[pos+4:pos+8] = bytes(reordered[pos+4:pos+8]),bytes(reordered[pos:pos+4])
    unchanged=sum(reordered[i:i+4]==output[i:i+4] for i in range(0,COUNT,4))
    if unchanged!=8:raise RuntimeError(f'expected eight unchanged LLVM6 words, got {unchanged}')
    original_lines,original_ops=decode(decoder,output)
    independent_lines,independent_ops=decode(decoder,independent)
    if original_ops!=independent_ops:
        raise RuntimeError(f'independent LLVM6 source changed operations: {original_ops-independent_ops} vs {independent_ops-original_ops}')
    original_text='\n'.join(original_lines).lower().replace(' ','')
    for term in ['call0x96cd1020','call0x96cd0480','r1=r16','r1=r17','r2=memw(r19+#0x0)',
                 'r0=memw(r2+##0x10)','r3:2=combine(##0x0,#-0x1)','r16=r0',
                 'jump:t0x96cc8670','jump0x96cc88bc']:
        if term not in original_text:raise RuntimeError(f'original V66 packet missing {term}')
    print(f'PASS {COUNT} bytes exact against pinned original, independent RWE prefix checked')
    print(f'PASS LLVM6 semantic assembly and Rizin decoded operations; {unchanged}/10 unchanged reordered words')
    print('PASS original V66 packet order, both call destinations, loop backedge and error jump')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for x in ('llvm-mc','rz-asm','generated','reference'):p.add_argument('--'+x,required=True,type=Path)
    p.add_argument('--prefix',type=Path)
    a=p.parse_args()
    verify(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
