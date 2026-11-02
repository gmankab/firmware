#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V66 K0 locked context, global status, SSR, and SGP0 helpers.

Only 160 executable bytes are built; the 8 following zero-filled file bytes
belong to linker layout and are never counted as independently authored code.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
TOOLS=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,read_system_register,KernelZeroLock
SOURCE=ASSEMBLY_DIR / 'rwe-next-locked-controller.S'
FILE_START=0x10fe8
FILE_END=0x11088
SIZE=FILE_END-FILE_START
FILL_START=FILE_END
FILL_END=0x11090
PHYSICAL_START=0x8be0dfe8
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
REPLACEMENTS={
    0x10fe8: read_system_register(2,SystemRegister.SGP0),
    0x10ff0: int(KernelZeroLock.LOCK),
    0x11018: int(KernelZeroLock.UNLOCK),
    0x11038: int(KernelZeroLock.LOCK),
    0x11048: int(KernelZeroLock.UNLOCK),
    0x11050: read_system_register(0,SystemRegister.SSR),
    0x11060: read_system_register(2,SystemRegister.SGP0),
}

def base():
    p=TOOLS/'emit-adsp-entry.py';spec=importlib.util.spec_from_file_location('adsp_encoder',p)
    if not spec or not spec.loader:raise RuntimeError('Missing source assembler')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

def render(llvm_mc:str)->bytes:
    m=base();data=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
    marker=struct.pack('<I',0x7f00c000) # source-authored NOP at original privileged instruction positions
    for addr,encoded in REPLACEMENTS.items():
        off=addr-FILE_START
        if data[off:off+4]!=marker:
            raise m.AdspError(f'V66 privileged placeholder moved at ADSP file 0x{addr:x}')
        data[off:off+4]=struct.pack('<I',encoded)
    if any(data[i:i+4]==marker for i in range(0,len(data),4)):
        raise m.AdspError('Unresolved privileged placeholder')
    return bytes(data)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong pinned ADSP firmware SHA-256')
    if len(built)!=SIZE or built!=reference[FILE_START:FILE_END]:
        raise ValueError('ADSP locked controller bytes differ from pinned reference')
    if reference[FILL_START:FILL_END]!=b'\0'*(FILL_END-FILL_START):
        raise ValueError('ADSP linker fill unexpectedly changed')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();b=render(a.llvm_mc)
    if a.reference:verify(a.reference.read_bytes(),b)
    a.output.write_bytes(b)
    print(f'ADSP V66 {len(b)} source-built controller bytes file '
          f'[0x{FILE_START:x},0x{FILE_END:x}) physical '
          f'[0x{PHYSICAL_START:x},0x{PHYSICAL_START+SIZE:x}) '
          f'sha256={hashlib.sha256(b).hexdigest()}')
    return 0
if __name__=='__main__':sys.exit(main())
