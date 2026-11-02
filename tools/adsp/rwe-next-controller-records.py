#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build the next independent ADSP RWE Hexagon controller-record routine.

The instructions begin at firmware file 0x10c40, after 996 true zero-fill
bytes [0x1084c,0x10c30) and a separate 16-byte unclassified table. No
reference bytes are used in output. The source-built function completes its
record processing and returns by 0x10dc4. The original routine has a real return at 0x10dc0.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,sys,struct
from pathlib import Path
import sys
TOOLS=Path(__file__).resolve().parent
ASSEMBLY_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/adsp/rwe'
SOURCE=ASSEMBLY_DIR / 'rwe-next-controller-records.S'
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import (SystemRegister,read_system_register,packet_continuation)
FILE_START=0x10c40
FILE_END=0x10dc4
SIZE=FILE_END-FILE_START
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def base():
    spec=importlib.util.spec_from_file_location('validated_hexagon_source',TOOLS/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing ADSP source assembler')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m

def render(mc:str)->bytes:
    maker=base()
    built=bytearray(maker.assemble(mc,SOURCE,SIZE))
    # The original CPU has a packet containing a P0 comparison followed by
    # a duplex R5=0 / R1:0=0. LLVM6 emits the canonical *separate* packet
    # for P0; preserve identical source-built opcode and adjust only the
    # architecturally checked continue-packet marker to group the P0+duplex.
    at=0x10d54-FILE_START
    single=struct.pack('<I',0x7504c000)
    if built[at:at+4]!=single:
        raise maker.AdspError('Hexagon controller P0 producer slot moved')
    built[at:at+4]=struct.pack('<I',packet_continuation(0x7504c000))
    # Hexagon LLVM6 cannot parse V66 PCYCLELO. Verify the single NOP slot
    # generated from mnemonic source before writing narrow, checked S30.
    cycle=0x10d5c-FILE_START
    if built[cycle:cycle+4]!=struct.pack('<I',0x7f00c000):
        raise maker.AdspError('Hexagon V66 PCYCLELO slot moved')
    built[cycle:cycle+4]=struct.pack('<I',
        read_system_register(4,SystemRegister.PCYCLELO))
    return bytes(built)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong ADSP reference')
    if len(built)!=SIZE or built!=reference[FILE_START:FILE_END]:
        raise ValueError('ADSP controller records differ from reference')
    if any(reference[0x1084c:0x10c30]):
        raise ValueError('Expected unimplemented linker zeros before controller records')
    # The 16-byte table is not emitted as executable code; its identity is
    # checked for reporting only. A different table requires further analysis.
    if reference[0x10c30:0x10c40]!=(b'\xff'*8+b'\x00'*8):
        raise ValueError('Unexpected auxiliary record table before function')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();output=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),output)
    args.output.write_bytes(output)
    print('ADSP controller record writer:',len(output),'exact code bytes',
          f'[0x{FILE_START:x},0x{FILE_END:x}), sha256',hashlib.sha256(output).hexdigest(),
          '; original linker fill/table excluded')
    return 0
if __name__=='__main__':sys.exit(main())
