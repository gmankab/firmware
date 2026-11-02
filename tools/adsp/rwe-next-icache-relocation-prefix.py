#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-build first 108 bytes of Hexagon V66 i-cache relocation routine.

The remainder of this function is not implemented at this checkpoint.
Instruction-stream bytes come from mnemonic-only LLVM6 source plus checked
EVB register transfer, never copied from the private ADSP reference.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
TOOLS=Path(__file__).resolve().parent
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,read_system_register
SOURCE=TOOLS/'rwe-next-icache-relocation-prefix.S'
FILE_START=0x11170
FILE_END=0x111dc
SIZE=FILE_END-FILE_START
FILL_START=0x11164
FILL_END=FILE_START
PHYSICAL_START=0x8be0e170
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def base():
    spec=importlib.util.spec_from_file_location('adsp_encoder',TOOLS.parent/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing ADSP Hexagon source assembler')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m

def render(llvm_mc:str)->bytes:
    m=base();b=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
    at=0x11174-FILE_START
    if b[at:at+4]!=struct.pack('<I',0x7f00c000):
        raise m.AdspError('V66 EVB read source slot moved')
    b[at:at+4]=struct.pack('<I',read_system_register(2,SystemRegister.EVB))
    if any(b[i:i+4]==struct.pack('<I',0x7f00c000) for i in range(0,len(b),4)):
        raise m.AdspError('Remaining privileged NOP placeholder')
    return bytes(b)

def verify(original:bytes,built:bytes)->None:
    if hashlib.sha256(original).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong ADSP reference firmware revision')
    if len(built)!=SIZE or original[FILE_START:FILE_END]!=built:
        raise ValueError('ADSP cache relocation prefix differs from reference')
    if original[FILL_START:FILL_END]!=b'\0'*(FILL_END-FILL_START):
        raise ValueError('Cache relocation linker-fill bytes unexpectedly differ')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP source-built relocation prefix {len(b)} bytes '
          f'file [0x{FILE_START:x},0x{FILE_END:x}), physical '
          f'[0x{PHYSICAL_START:x},0x{PHYSICAL_START+SIZE:x}) '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':sys.exit(main()
)
