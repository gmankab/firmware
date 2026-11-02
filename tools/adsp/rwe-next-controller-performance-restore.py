#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-assemble controller PMU register restore [0x138f0,0x139c8).

LLVM6 reconstructs the loop and atomic controller transactions exactly;
16 system-register transfers use bounded source/destination encoders at
named architectural NOP placeholders. OEM bytes are verification-only.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,read_system_register,write_system_register
START=0x138f0;END=0x139c8;SIZE=END-START;PHYSICAL=0x8be108f0
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-performance-restore.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)
# Source-authored addresses, system-register identities and GPRs.
SYSTEM_TRANSFERS={
 0x13920:read_system_register(0,SystemRegister.SYSCFG),
 0x13928:write_system_register(SystemRegister.SYSCFG,0),
 0x1393c:write_system_register(SystemRegister.PMUCNT0,4),
 0x13944:write_system_register(SystemRegister.PMUCNT1,4),
 0x1394c:write_system_register(SystemRegister.PMUCNT2,4),
 0x13954:write_system_register(SystemRegister.PMUCNT3,4),
 0x1395c:write_system_register(SystemRegister.S44,4),
 0x13964:write_system_register(SystemRegister.S45,4),
 0x1396c:write_system_register(SystemRegister.S46,4),
 0x13974:write_system_register(SystemRegister.S47,4),
 0x1397c:write_system_register(SystemRegister.PMUCFG,4),
 0x13984:write_system_register(SystemRegister.S55,4),
 0x1398c:write_system_register(SystemRegister.PMUEVTCFG,4),
 0x13994:write_system_register(SystemRegister.S54,2),
 0x139ac:read_system_register(0,SystemRegister.SYSCFG),
 0x139b4:write_system_register(SystemRegister.SYSCFG,0),
}

def render(llvm_mc:str)->bytes:
 sp=importlib.util.spec_from_file_location('hexagon_emitter',T/'emit-adsp-entry.py')
 if not sp or not sp.loader:raise RuntimeError('Missing independent LLVM6 emitter')
 m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
 b=bytearray(m.assemble(llvm_mc,SOURCE,SIZE))
 for addr,op in SYSTEM_TRANSFERS.items():
  off=addr-START
  if b[off:off+4]!=NOP:raise m.AdspError(f'Expected one-word system-register NOP at 0x{addr:x}')
  struct.pack_into('<I',b,off,op)
 if any(b[i:i+4]==NOP for i in range(0,SIZE,4)):
  raise m.AdspError('Unexpected unresolved system-register placeholder')
 return bytes(b)

def verify(ref:bytes,built:bytes)->None:
 if hashlib.sha256(ref).hexdigest()!=REFERENCE_SHA:
  raise ValueError('Wrong pinned ADSP reference revision')
 if len(built)!=SIZE or built!=ref[START:END]:
  raise ValueError('ADSP PMU state restore differs from reference')
 if ref[START-4:START]!=bytes(4):
  raise ValueError('Expected four-byte linker fill before PMU restoration')

def main()->int:
 p=argparse.ArgumentParser(description=__doc__)
 p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
 p.add_argument('--output',type=Path,required=True)
 a=p.parse_args();code=render(a.llvm_mc)
 if a.reference:verify(a.reference.read_bytes(),code)
 a.output.write_bytes(code)
 print(f'ADSP PMU restoration: {SIZE} byte-identical executable bytes '
       f'file [0x{START:x},0x{END:x}), physical [0x{PHYSICAL:x},0x{PHYSICAL+SIZE:x}); '
       f'38 LLVM6 instructions and 16 typed PMU/SYSCFG transfers; '
       f'sha256={hashlib.sha256(code).hexdigest()}')
 return 0
if __name__=='__main__':raise SystemExit(main())
