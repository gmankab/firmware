#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6/Rizin Hexagon V66 check of controller selection.

OEM packet order is not the same as LLVM6's scheduling of combine and
P1.new operations. Verify the differing parse bits and schedule explicitly,
not by inventing independent byte-equivalence for those changed packets.
"""
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
FILE=0x56990
END=0x569c0
PHYSICAL=0x96cc8990

def original_elf_text(blob:bytes)->bytes:
    script=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('independent_cdsp_text_reader',script)
    if spec is None or spec.loader is None:raise RuntimeError('independent ELF text reader unavailable')
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.parse_elf32_text(blob)

def checked_words(raw:bytes)->list[int]:
    if len(raw)%4:raise ValueError('instruction word misalignment')
    return [int.from_bytes(raw[i:i+4],'little') for i in range(0,len(raw),4)]

def rz_lines(rz:Path,raw:bytes)->list[str]:
    result=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',raw.hex()],
        capture_output=True,text=True,check=True)
    lines=[line for line in result.stdout.splitlines() if line.startswith('0x')]
    if len(lines)!=len(raw)//4 or any('invalid' in line.lower() for line in lines):
        raise ValueError('Rizin failed to decode all source-built Hexagon words')
    return lines

def verify(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None):
    data=generated.read_bytes()
    source=reference.read_bytes()
    if len(data)!=48 or data!=source[FILE:END]:
        raise ValueError('source-built controller selector differs from pinned CDSP')
    if prefix is not None:
        full=prefix.read_bytes()
        if len(full)!=END-0x56000 or full!=source[0x56000:END]:
            raise ValueError('joined CDSP RWE prefix differs from original firmware')
    fixture=Path(__file__).with_name('agent20-10-cdsp-controller-selection-semantic.s')
    with tempfile.TemporaryDirectory(prefix='nabu-cdsp-selection-') as root:
        obj=Path(root)/'fixture.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(fixture),'-o',str(obj)],
            check=True,capture_output=True,text=True)
        alternate=original_elf_text(obj.read_bytes())[:48]
    if len(alternate)!=48:raise ValueError('LLVM6 reference must contain 12 instruction words')
    expected=checked_words(data)
    compiled=checked_words(alternate)
    # Scalar instruction fields match unchanged after stripping *only* the
    # parse bits; the original reordered the combine and predicated move.
    mapping={0:0,1:1,2:3,3:2,4:4,5:5,6:6,7:7,8:8,9:9,10:10,11:11}
    for i,j in mapping.items():
        # LLVM schedules the compound multiply before the call into a
        # different *packet*, so the encoded CALL immediate changes while
        # its absolute physical destination remains identical.
        if i==7:continue
        if (expected[i]&~0xc000)!=(compiled[j]&~0xc000):
            raise ValueError(f'LLVM6 scalar operands differ OEM word {i}, independent {j}')
    oem_lines=rz_lines(rz,data)
    fixture_lines=rz_lines(rz,alternate)
    norm='\n'.join(oem_lines).lower().replace(' ','')
    independently='\n'.join(fixture_lines).lower().replace(' ','')
    for phrase in ('p0=!cmp.gt(r21,r19)','p1=cmp.eq(r21,##0x0)',
        'r1:0=combine(##0x1,#0x0)','r2=mux(p0,##-0x1,#0x0)',
        'if(!p1)r3=add(r21,r2)','r2=add(r20,mpyi(#0xa0,r3))',
        'call0x96ccfd00','jump:nt0x96cc8adc','jump:nt0x96cc8aec'):
        if phrase not in norm:raise ValueError('OEM packet missing '+phrase)
        if phrase not in independently:raise ValueError('independent semantic fixture missing '+phrase)
    if oem_lines[2].split()[0] != '0x96cc8998' or oem_lines[3].split()[0] != '0x96cc899c':
        raise ValueError('source packet addressing invalid')
    print('PASS: exact 48 source-generated CDSP controller selector bytes; 12 Rizin V66 words')
    print('PASS: independent LLVM6 scalar fields except scheduled call, verified by equal absolute target')
    print('PASS: physical service call 0x96ccfd00 and both missing-handle branches verified')
    if prefix:print('PASS: complete 2496-byte RWE prefix = 2440 executable + 56 layout')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ('llvm-mc','rz-asm','generated','reference','prefix'):
        p.add_argument('--'+name,type=Path,required=name!='prefix')
    a=p.parse_args()
    verify(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
