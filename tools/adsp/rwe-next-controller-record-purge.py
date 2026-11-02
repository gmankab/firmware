#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-build V66 ADSP controller record purge [0x13610,0x13744).

The routine scans the global controller record chain, removes active
entries, drains the linked children while holding K0 and invokes cleanup.
LLVM6 encodes all supported packets. Four V66 system operations use checked
architectural encoders at named NOP placeholders; no OEM opcodes are copied.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
import sys
T=Path(__file__).resolve().parent
if str(T) not in sys.path:sys.path.insert(0,str(T))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import SystemRegister,read_system_register,KernelZeroLock
START=0x13610;END=0x13744;SIZE=END-START;PHYSICAL=0x8be10610
SOURCE=T/'rwe-next-controller-record-purge.S'
REFERENCE_SHA='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
NOP=struct.pack('<I',0x7f00c000)
V66_OPS={0x13624:read_system_register(2,SystemRegister.SGP0),
         0x13650:int(KernelZeroLock.UNLOCK),
         0x136a0:int(KernelZeroLock.LOCK),
         0x13704:int(KernelZeroLock.LOCK)}

def render(llvm_mc:str)->bytes:
 spec=importlib.util.spec_from_file_location('adsp_llvm',T/'emit-adsp-entry.py')
 if not spec or not spec.loader:raise RuntimeError('Missing original source assembler')
 em=importlib.util.module_from_spec(spec);spec.loader.exec_module(em)
 b=bytearray(em.assemble(llvm_mc,SOURCE,SIZE))
 for file_offset,opcode in V66_OPS.items():
  at=file_offset-START
  if b[at:at+4]!=NOP:raise em.AdspError(f'Expected isolated V66 NOP at 0x{file_offset:x}')
  struct.pack_into('<I',b,at,opcode)
 # Two original architectural NOPs are deliberate branch packet slots.
 expected_padding={0x136cc-START}
 if b[0x136c8-START:0x136cc-START]!=struct.pack('<I',0x7f004000):
  raise em.AdspError('Expected source-assembled packet-continuation NOP')
 remaining={i for i in range(0,SIZE,4) if b[i:i+4]==NOP}
 if remaining!=expected_padding:
  raise em.AdspError(f'Unexpected NOP slots: {sorted(remaining)}')
 return bytes(b)

def verify(reference:bytes,generated:bytes)->None:
 if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA:
  raise ValueError('Incorrect pinned ADSP firmware revision')
 if len(generated)!=SIZE or generated!=reference[START:END]:
  raise ValueError('Controller record purge differs from ADSP reference')

def main()->int:
 p=argparse.ArgumentParser(description=__doc__)
 p.add_argument('--llvm-mc',required=True);p.add_argument('--reference',type=Path)
 p.add_argument('--output',type=Path,required=True)
 a=p.parse_args();b=render(a.llvm_mc)
 if a.reference:verify(a.reference.read_bytes(),b)
 a.output.write_bytes(b)
 print(f'ADSP controller record purge: {SIZE} byte-identical executable bytes; '
       f'file [0x{START:x},0x{END:x}), physical [0x{PHYSICAL:x},0x{PHYSICAL+SIZE:x}); '
       f'73 LLVM6 instructions and four checked V66 ops; '
       f'sha256={hashlib.sha256(b).hexdigest()}')
 return 0
if __name__=='__main__':raise SystemExit(main())
