#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Reconstruct the first controller-global startup packets at ADSP 0x11450.

This 88-byte source-level Hexagon reconstruction is a function PREFIX, not a
complete loader, executable region, or signed image. Ordinary packets are
assembled independently by LLVM6; the three V66 privileged operations use
separately checked typed operand-field encoders. The reference is read only
for strict revision and byte-equality verification, never during generation.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
TOOLS=Path(__file__).resolve().parent
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import (ControllerAddressCommand,
    controller_address_command, SystemRegister, write_system_register)
SOURCE=TOOLS/'rwe-next-controller-global-startup.S'
FILE_START=0x11450
FILE_END=0x114a8
SIZE=FILE_END-FILE_START
PHYSICAL_START=0x8be0e450
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
REPLACEMENTS={
    0x11468:controller_address_command(ControllerAddressCommand.CACHE_SOFTWARE_INTERRUPT,0),
    0x1146c:controller_address_command(ControllerAddressCommand.CACHE_INVALIDATE_ADDRESS,0),
    0x11474:write_system_register(SystemRegister.IMASK,1),
}

def assembler():
    spec=importlib.util.spec_from_file_location('adsp_entry',TOOLS.parent/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing ADSP assembler')
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module

def render(llvm_mc:str)->bytes:
    a=assembler();assembled=bytearray(a.assemble(llvm_mc,SOURCE,SIZE))
    marker=struct.pack('<I',0x7f00c000)
    for addr,code in REPLACEMENTS.items():
        offset=addr-FILE_START
        if assembled[offset:offset+4]!=marker:
            raise a.AdspError(f'Privileged placeholder is not NOP at ADSP 0x{addr:x}')
        assembled[offset:offset+4]=struct.pack('<I',code)
    if any(assembled[i:i+4]==marker for i in range(0,len(assembled),4)):
        raise a.AdspError('Unresolved source placeholder in controller startup')
    return bytes(assembled)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong pinned ADSP firmware revision')
    if len(built)!=SIZE or built!=reference[FILE_START:FILE_END]:
        raise ValueError('Source-generated global controller packets differ at 0x11450')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();built=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),built)
    a.output.write_bytes(built)
    print('ADSP V66 88-byte controller-global startup prefix: '
          f'file [0x{FILE_START:x},0x{FILE_END:x}), '
          f'physical 0x{PHYSICAL_START:x}, '
          f'sha256={hashlib.sha256(built).hexdigest()}')
    return 0
if __name__=='__main__':sys.exit(main())
