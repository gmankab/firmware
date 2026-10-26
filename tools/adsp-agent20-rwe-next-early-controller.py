#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild Hexagon V66 ADSP controller startup and SGP0 status getter."""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
TOOLS=Path(__file__).resolve().parent
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
from adsp_hexagon_v66 import SystemRegister,read_system_register,write_system_register,KernelZeroLock,TlbControl
FILE_START=0x10ec0
FILE_END=0x10f64
SIZE=FILE_END-FILE_START
FILL_START=0x10eb4
FILL_END=FILE_START
SOURCE=TOOLS/'adsp-agent20-rwe-next-early-controller.S'
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def tool():
    s=importlib.util.spec_from_file_location('emit_adsp_entry',TOOLS/'emit-adsp-entry.py')
    if not s or not s.loader:raise RuntimeError('ADSP emitter unavailable')
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
    return m

def render(mc:str)->bytes:
    m=tool()
    source=bytearray(m.assemble(mc,SOURCE,SIZE))
    placeholder=struct.pack('<I',0x7f00c000)
    replacements={
        0x10ed0-FILE_START:read_system_register(2,SystemRegister.SGP0),
        0x10ef8-FILE_START:read_system_register(16,SystemRegister.HTID),
        0x10f1c-FILE_START:read_system_register(0,SystemRegister.SYSCFG),
        0x10f24-FILE_START:write_system_register(SystemRegister.SYSCFG,0),
        0x10f38-FILE_START:int(KernelZeroLock.UNLOCK),
        0x10f3c-FILE_START:int(TlbControl.UNLOCK),
        0x10f50-FILE_START:read_system_register(2,SystemRegister.SGP0),
    }
    for at,opcode in replacements.items():
        if source[at:at+4]!=placeholder:raise m.AdspError(f'Unexpected privilege placeholder at +0x{at:x}')
        source[at:at+4]=struct.pack('<I',opcode)
    return bytes(source)

def verify(original:bytes,built:bytes)->None:
    if hashlib.sha256(original).hexdigest()!=REFERENCE_SHA256:raise ValueError('Wrong ADSP reference version')
    if len(built)!=SIZE or built != original[FILE_START:FILE_END]:raise ValueError('Controller differs from ADSP reference')
    if original[FILL_START:FILL_END] != b'\0'*(FILL_END-FILL_START):raise ValueError('Unexpected pre-controller fill')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',required=True,type=Path)
    a=p.parse_args()
    built=render(a.llvm_mc)
    if a.reference: verify(a.reference.read_bytes(),built)
    a.output.write_bytes(built)
    print(f'ADSP V66 controller startup {len(built)} bytes [0x{FILE_START:x},0x{FILE_END:x}) sha256={hashlib.sha256(built).hexdigest()}')
    return 0
if __name__=='__main__':sys.exit(main())
