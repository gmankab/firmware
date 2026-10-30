#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""V66 supervisor restore: pinned bytes, Rizin per-word semantics, LLVM6 samples.

This script intentionally distinguishes Rizin validation of privileged
system-register instructions from exact independent LLVM6 encoding matches.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess
START=0x59f0;END=0x5a5c;PC=0x96c0_29f0
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
EXPECTED=[
'immext(##0xfb80)','R4 = ##0xfbbb',
'immext(##0xfe026e00)','R0 = memw(GP+##0xfe026e00)',
'R0 = clrbit(R0,#0x10)',
'immext(##0xfe037ac0)','R28 = memw(GP+##0xfe037ac0)',
'immext(##0xfe037a80)','R1 = memw(GP+##0xfe037aac)',
'immext(##0xfe037a80)','R2 = memw(GP+##0xfe037ab0)',
'SGP0 = R28','IMASK = R4','SSR = R0','CCR = R1','USR = R2',
'immext(##0x5e00)','jump 0x96c08860',
'immext(##0xfe0310c0)','R0 = memw(GP+##0xfe0310e0)',
'immext(##0xfe037ac0)','SP = memw(GP+##0xfe037ac0)',
'start(R0)','immext(##0x31980)','call 0x96c34400',
'R0 = ##0x0','jump 0x96c02a4c',
]
assert len(EXPECTED)==27

def normalize(v):return ''.join(v.lower().split()).replace('r31','lr')

def verify(binary:Path,reference:Path,rz:Path,llvm:Path):
    original=reference.read_bytes();actual=binary.read_bytes()
    if hashlib.sha256(original).hexdigest()!=SHA:raise ValueError('Wrong pinned CDSP revision')
    if actual!=original[START:END] or len(actual)!=END-START:raise ValueError('CDSP supervisor restore bytes differ')
    stdout=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',actual.hex()],check=True,capture_output=True,text=True).stdout
    lines=[]
    for line in stdout.splitlines():
        m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
        if m:lines.append((int(m[1],16),m[2].split('<endloop')[0]))
    if len(lines)!=27:raise ValueError(f'Rizin decoded only {len(lines)} of 27')
    for i,((pc,ins),want) in enumerate(zip(lines,EXPECTED)):
        if pc!=PC+i*4 or normalize(ins)!=normalize(want):raise ValueError(f'Rizin mismatch at word {i}: {ins!r} vs {want!r}')
    spec=importlib.util.spec_from_file_location('hvx_helper',Path(__file__).with_name('verify-agent20-09-hvx.py'))
    assert spec and spec.loader
    helper=importlib.util.module_from_spec(spec);spec.loader.exec_module(helper)
    # LLVM6 emits different legal extended GP and long-jump instruction
    # families. Do not count those as exact independent operand encodings.
    independent={
      4:'{r0=clrbit(r0,#16)}',
      15:'{usr=r2}',
      25:'{r0=#0}',
      26:'{jump #-12}',
    }
    checked=[]
    for i,asm in independent.items():
        compiled=helper.llvm_bytes(llvm,asm)
        if len(compiled)!=4:raise ValueError(f'LLVM sample length mismatch at {i}')
        opcode=int.from_bytes(compiled,'little')&~0xc000
        expected=int.from_bytes(actual[4*i:4*i+4],'little')&~0xc000
        if opcode!=expected:raise ValueError(f'LLVM mismatch word {i}: {opcode:#x} vs {expected:#x}')
        checked.append(i)
    # The instruction extenders for r4 and the long jump have independent
    # LLVM6 verification even though LLVM6 packs the consumer differently.
    for i,source in [(0,'{r4=##0xfbbb}'),(16,'{jump ##0x5e34}')]:
        bytes_=helper.llvm_bytes(llvm,source)
        if len(bytes_)!=8:raise ValueError(f'Expected LLVM extended operation at {i}')
        if int.from_bytes(bytes_[:4],'little')&~0xc000!=int.from_bytes(actual[4*i:4*i+4],'little')&~0xc000:
            raise ValueError(f'LLVM extender high bits differ at word {i}')
        checked.append(i)
    if len(checked)!=6:raise ValueError('expected 6 exact LLVM6 samples')
    print(f'CDSP supervisor restore: {len(actual)} reference-exact bytes; 27/27 Rizin V66 semantic words; {len(checked)} LLVM6 exact sample word fields; privileged registers/long-call body Rizin/reference only.')

if __name__=='__main__':
    p=argparse.ArgumentParser()
    for a in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+a,type=Path,required=True)
    a=p.parse_args();verify(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
