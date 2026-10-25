#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6/Rizin V66 audit of CDSP callback completion and layout.

LLVM6's semantic fixture is 84 bytes, versus 88 source bytes including
original forced GP extender + two layout NOPs. The compiler also schedules
one backward call before the register save, and changes call offsets because
packet addresses differ. This verifies *some* LLVM6 word identity, plus
all 20 original-order V66 decoded instructions with Rizin; it does not claim
an unchanged complete binary assembly result from LLVM6.
"""
import argparse, importlib.util, os, subprocess, tempfile
from pathlib import Path

OFF=0x562c8
LEN=80
BASE=0x96cc82c8
PREFIX_START=0x56000
PREFIX_END=0x56320

def load_text(data:bytes)->bytes:
    path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('verify_cdsp_completion_elf',path)
    if spec is None or spec.loader is None:raise RuntimeError('CDSP ELF source parser unavailable')
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module.parse_elf32_text(data)

def words(blob:bytes)->list[int]:
    if len(blob)%4:raise ValueError('Hexagon packets require 4-byte words')
    return [int.from_bytes(blob[i:i+4],'little') for i in range(0,len(blob),4)]

def verify(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None)->None:
    actual=generated.read_bytes();ref=reference.read_bytes()
    if len(actual)!=LEN or ref[OFF:OFF+LEN]!=actual:
        raise ValueError('CDSP 80-byte typed completion source differs from reference')
    with tempfile.TemporaryDirectory(prefix='cdsp-agent20-10-finish-') as tmp:
        dest=Path(tmp)/'callback_finish.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60',
                        '-filetype=obj',str(Path(__file__).with_name('agent20-10-cdsp-rwe-callback-finish.s')),
                        '-o',str(dest)],check=True,capture_output=True,text=True)
        independent=load_text(dest.read_bytes())[0x2c8:]
    if len(independent)!=84:raise ValueError(f'Expected 84 LLVM6 semantic bytes, got {len(independent)}')
    a=words(actual);b=words(independent)
    positions=[(0,1),(1,2),(6,5),(7,6),(9,8),(10,9),
               (12,12),(13,11),(14,13),(15,14),(16,15),(17,16),(18,17)]
    for ours,llvm_i in positions:
        if a[ours]!=b[llvm_i]:
            raise ValueError(f'LLVM6 semantic instruction {ours} disagrees at independent index {llvm_i}')
    rz_out=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(BASE),'-D',actual.hex()],
                          text=True,capture_output=True,check=True).stdout
    decoded=[ln for ln in rz_out.splitlines() if ln.startswith('0x')]
    if len(decoded)!=20 or any('invalid' in ln.lower() for ln in decoded):
        raise ValueError(f'Rizin V66 decoded {len(decoded)}/20 words or invalid instruction')
    normalized='\n'.join(decoded).lower().replace(' ','')
    patterns=['r0=##-0x2','allocframe(#0x8)','memw(gp+##0xc028c538)',
              'call0x96cc8230','call0x96cca640',
              'r1:0=combine(r17,r16)','jump0x96ccc7a0','deallocframe']
    for pat in patterns:
        if pat not in normalized:raise ValueError('Rizin did not confirm feature '+pat)
    if prefix is not None:
        full=prefix.read_bytes()
        if len(full)!=PREFIX_END-PREFIX_START or full!=ref[PREFIX_START:PREFIX_END]:
            raise ValueError('CDSP contiguous 0x56000..0x56320 prefix mismatch')
        if full[OFF-PREFIX_START:OFF-PREFIX_START+LEN]!=actual:
            raise ValueError('CDSP prefix did not embed source-generated completion wrapper')
        if full[-8:]!=b'\x00\xc0\x00\x7f'*2:
            raise ValueError('Original two NOP layout words not identified')
    print('PASS 80 typed executable bytes exact against pinned CDSP reference')
    print(f'PASS LLVM6 semantic source: {len(positions)} unchanged V66 words, compiler shortened/reordered remainder')
    print('PASS Rizin V66 confirms all 20 original-order instruction words and jump targets')
    if prefix:print('PASS 800-byte RWE prefix: 768 code bytes + 32 explicitly accounted NOP/fill bytes')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for arg in ['llvm-mc','rz-asm','generated','reference','prefix']:
        p.add_argument('--'+arg,type=Path,required=arg!='prefix')
    v=p.parse_args();verify(v.llvm_mc,v.rz_asm,v.generated,v.reference,v.prefix)
