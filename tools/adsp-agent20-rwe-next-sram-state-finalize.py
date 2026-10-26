#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Complete ADSP SRAM controller finalization [0x1180c,0x11894).

Build from LLVM6 mnemonic packet source plus four checked V66 privileged
register operations, replacing only validated LLVM6 NOP sentinels. The ADSP
reference is read ONLY when --reference is supplied for SHA and byte checks.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
from adsp_hexagon_v66 import SystemRegister,read_system_register,write_system_register

START=0x1180c
END=0x11894
SIZE=END-START
PHYSICAL=0x8be0e80c
NEXT_FILL=(0x11894,0x11898)
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
SOURCE=T/'adsp-agent20-rwe-next-sram-state-finalize.S'
PRIVILEGED={
    0x1184c:read_system_register(0,SystemRegister.SYSCFG),
    0x11854:write_system_register(SystemRegister.SYSCFG,0),
    0x11860:read_system_register(2,SystemRegister.SGP0),
    0x11878:read_system_register(2,SystemRegister.SGP0),
}
NOP=struct.pack('<I',0x7f00c000)

def render(llvm_mc:str)->bytes:
    spec=importlib.util.spec_from_file_location('adsp',T/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing independent LLVM6 assembler')
    a=importlib.util.module_from_spec(spec);spec.loader.exec_module(a)
    data=bytearray(a.assemble(llvm_mc,SOURCE,SIZE))
    for pos,opcode in PRIVILEGED.items():
        i=pos-START
        if data[i:i+4]!=NOP:
            raise a.AdspError(f'V66 placeholder expected at 0x{pos:x}')
        data[i:i+4]=struct.pack('<I',opcode)
    if any(data[i:i+4]==NOP for i in range(0,SIZE,4)):
        raise a.AdspError('Unreplaced privileged placeholder')
    return bytes(data)

def verify(reference:bytes,rebuilt:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
        raise ValueError('Wrong original ADSP reference revision')
    if reference[NEXT_FILL[0]:NEXT_FILL[1]]!=bytes(NEXT_FILL[1]-NEXT_FILL[0]):
        raise ValueError('Following linker fill no longer zero')
    if len(rebuilt)!=SIZE or rebuilt!=reference[START:END]:
        raise ValueError('SRAM finalization differs from pinned reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();result=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),result)
    args.output.write_bytes(result)
    print(f'ADSP SRAM controller finalizer: {SIZE} bytes file [0x{START:x},0x{END:x}), '
          f'physical [0x{PHYSICAL:x},0x{PHYSICAL+SIZE:x}), '
          f'sha256={hashlib.sha256(result).hexdigest()}')
    return 0
if __name__=='__main__':raise SystemExit(main())
