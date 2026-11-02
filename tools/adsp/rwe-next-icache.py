#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-build ADSP V66 instruction-cache tag/data snapshot, separately.

The previous source-authored fragment ends at 0x10674. The 116 intervening
zero bytes through 0x106e8 are linker fill and must not be counted as code.
This executable routine independently covers file [0x106e8,0x1084c).
"""
from __future__ import annotations
import argparse, hashlib, importlib.util, struct, sys
from pathlib import Path
import sys
TOOLS=Path(__file__).resolve().parent
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from adsp_hexagon_v66 import (SystemRegister,ControlRegister,
    InstructionCacheRead,read_system_register,write_system_register,
    read_control_register,read_instruction_cache_word,packet_continuation)
FILE_START=0x106e8
FILE_END=0x1084c
SIZE=FILE_END-FILE_START
FILL_START=0x10674
FILL_END=FILE_START
FOLLOW_FILL_END=0x10c30
SOURCE=TOOLS/'rwe-next-icache-snapshot.S'
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def base():
    spec=importlib.util.spec_from_file_location('adsp_validated_mc',TOOLS.parent/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Cannot find the ADSP source assembler')
    m=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m

def emit(llvm_mc:str)->bytes:
    """Compile ordinary Hexagon V65-compatible packet source and patch V66 slots.

    No firmware byte is read; all instructions are source-compiled or built
    using narrowly checked Hexagon ISA operand fields. Only exact NOP slots
    with expected packet continuation are substituted.
    """
    maker=base()
    b=bytearray(maker.assemble(llvm_mc,SOURCE,SIZE))
    mid=struct.pack('<I',0x7f004000)
    last=struct.pack('<I',0x7f00c000)
    patches={
        0x00:(last,read_system_register(0,SystemRegister.REV)),
        0x40:(last,read_system_register(0,SystemRegister.SYSCFG)),
        0x48:(last,write_system_register(SystemRegister.SYSCFG,0)),
        0x5c:(mid,packet_continuation(read_control_register(10,ControlRegister.LC1))),
        0x6c:(mid,packet_continuation(read_control_register(10,ControlRegister.LC0))),
        0xa8:(last,read_instruction_cache_word(1,10,InstructionCacheRead.TAG)),
        0xf8:(last,read_instruction_cache_word(1,10,InstructionCacheRead.DATA)),
        0x124:(last,read_system_register(1,SystemRegister.SYSCFG)),
        0x12c:(last,write_system_register(SystemRegister.SYSCFG,1)),
    }
    for off,(placeholder,encoded) in patches.items():
        if b[off:off+4]!=placeholder:
            raise maker.AdspError(f'ICache V66 privileged slot moved at 0x{off:x}')
        b[off:off+4]=struct.pack('<I',encoded)
    # The nested endloop01 packet contains three architecturally valid NOPs;
    # do NOT mistake its terminal NOP for a remaining privileged sentinel.
    # The nine exact source positions above are checked and all replaced.
    if b[0x118:0x124]!=struct.pack('<III',0x7f008000,0x7f008000,0x7f00c000):
        raise maker.AdspError('Hexagon dual-loop terminal NOP packet changed')
    return bytes(b)

def verify(original:bytes,built:bytes)->None:
    if hashlib.sha256(original).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong ADSP reference firmware revision')
    if len(built)!=SIZE or built!=original[FILE_START:FILE_END]:
        raise ValueError('ICache snapshot is not reference-identical')
    if any(original[FILL_START:FILL_END]):
        raise ValueError('Preceding linker fill is nonzero')
    if any(original[FILE_END:FOLLOW_FILL_END]):
        raise ValueError('Following linker fill is nonzero')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args()
    b=emit(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),b)
    args.output.write_bytes(b)
    print('V66 ICache source fragment',len(b),'bytes file',hex(FILE_START),'..',hex(FILE_END),
          'physical',hex(0x8be00000+FILE_START-0x3000),
          'sha256',hashlib.sha256(b).hexdigest(),'; gap/linker fill excluded')
    return 0
if __name__=='__main__':sys.exit(main())
