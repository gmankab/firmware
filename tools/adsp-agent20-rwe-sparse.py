#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent, *disjoint* ADSP RWE source-built Hexagon packets after linker fill.

The preceding RWE executable fragment ends at 0x6c6c. The file interval
[0x6c6c, 0xff20) is zero fill; [0xff20,0xffb0) is nonzero, unclassified
metadata and [0xffb0,0x10330) is zero. None of these are silently emitted as
instructions or inferred function bodies. Source reconstruction resumes at
file 0x10330; one Hexagon packet spans the tail-call boundary at 0x10340.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,struct,sys
from pathlib import Path
TOOLS=Path(__file__).resolve().parent
if str(TOOLS) not in sys.path:sys.path.insert(0,str(TOOLS))
from adsp_hexagon_v66 import (SystemRegister, read_system_register,
    read_system_register_pair, write_system_register, packet_continuation)
SOURCE=TOOLS/'adsp-agent20-rwe-next-getter.S'
FILE_START=0x10330
FILE_END=0x104b8
SIZE=FILE_END-FILE_START
GETTER_SIZE=0x1c
THREAD_CYCLES_SOURCE=TOOLS/'adsp-agent20-rwe-next-thread-cycles.S'
THREAD_CYCLES_SIZE=0x58
ISDB_GUARD_SOURCE=TOOLS/'adsp-agent20-rwe-next-isdb-guard.S'
ISDB_GUARD_SIZE=0x2c
SRAM_CLONE_SOURCE=TOOLS/'adsp-agent20-rwe-next-sram-clone.S'
SRAM_CLONE_SIZE=0x60
CONTROLLER_POLL_SOURCE=TOOLS/'adsp-agent20-rwe-next-status-poll.S'
CONTROLLER_POLL_SIZE=0x30
DEBUG_SNAPSHOT_SOURCE=TOOLS/'adsp-agent20-rwe-next-debug-snapshot.S'
DEBUG_SNAPSHOT_SIZE=0x58
PHYS_START=0x8be0d330
SHA256_REFERENCE='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
FILL_START=0x6c6c
FILL_END=0xff20
UNKNOWN_RECORD_START=0xff20
UNKNOWN_RECORD_END=0xffb0
FILL2_START=0xffb0
FILL2_END=FILE_START

def base():
    spec=importlib.util.spec_from_file_location('adsp_original_encoder',TOOLS/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('missing verified ELF/Hexagon assembler')
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def isdb_guard(llvm_mc:str,previous)->bytes:
    """Guard V66 debugger enable and clear SYSCFG bit 5 under ISYNC.

    Five checked register slots are decoded from named Hexagon registers,
    retaining packet continuation on the combined SGP0/zero-store packet.
    """
    b=bytearray(previous.assemble(llvm_mc,ISDB_GUARD_SOURCE,ISDB_GUARD_SIZE))
    final=struct.pack('<I',0x7f00c000)
    continuing=struct.pack('<I',0x7f004000)
    replacements={
        0x00:(continuing,packet_continuation(read_system_register(29,SystemRegister.SGP0))),
        0x08:(final,read_system_register(0,SystemRegister.ISDBEN)),
        0x10:(final,read_system_register(1,SystemRegister.SYSCFG)),
        0x18:(final,write_system_register(SystemRegister.SYSCFG,1)),
        0x20:(final,read_system_register(0,SystemRegister.ISDBEN)),
    }
    for at,(expected,replacement) in replacements.items():
        if b[at:at+4]!=expected:
            raise previous.AdspError(f'ISDB guard register slot moved +0x{at:x}')
        b[at:at+4]=struct.pack('<I',replacement)
    if any(b[i:i+4] in (final,continuing) for i in range(0,len(b),4)):
        raise previous.AdspError('unresolved ISDB guard privileged opcode')
    return bytes(b)


def debug_snapshot(llvm_mc:str,previous)->bytes:
    """Snapshot seven V66 debug/breakpoint registers with `.new` stores.

    LLVM6 emits an independently source-assembled add-by-zero producer at
    each operation's exact location because it cannot assemble these V66
    privileged registers. The checked typed encoder supplies the original
    side-effecting register read while retaining the source's `.new`
    consumer and original continuing packet marker.
    """
    b=bytearray(previous.assemble(llvm_mc,DEBUG_SNAPSHOT_SOURCE,DEBUG_SNAPSHOT_SIZE))
    expected=struct.pack('<I',0xb0004000)
    regs=(SystemRegister.ISDBCFG0,SystemRegister.ISDBCFG1,
          SystemRegister.BRKPTPC0,SystemRegister.BRKPTCFG0,
          SystemRegister.BRKPTPC1,SystemRegister.BRKPTCFG1,
          SystemRegister.ISDBGPR)
    for i,which in enumerate(regs):
        at=0x08+i*8
        if b[at:at+4]!=expected:
            raise previous.AdspError(f'V66 debugger producer position changed +0x{at:x}')
        b[at:at+4]=struct.pack('<I',packet_continuation(read_system_register(0,which)))
    return bytes(b)


def render(llvm_mc: str)->bytes:
    # All seven V66 instructions are independently LLVM6-assembled, with no
    # reference bytes. The first packet sequence crosses a C function
    # boundary, retaining jumpr's continuation parse marker.
    b=base()
    getter=b.assemble(llvm_mc,SOURCE,GETTER_SIZE)
    trace=bytearray(b.assemble(llvm_mc,THREAD_CYCLES_SOURCE,
                               THREAD_CYCLES_SIZE))
    final=struct.pack('<I',0x7f00c000)
    middle=struct.pack('<I',0x7f004000)
    checked={
        0x1c:(final,read_system_register(6,SystemRegister.HTID)),
        0x20:(final,read_system_register_pair(4,30)),
        0x40:(middle,packet_continuation(read_system_register(12,SystemRegister.HTID))),
        0x4c:(final,packet_continuation(read_system_register_pair(4,30))),
    }
    for off,(marker,word) in checked.items():
        if trace[off:off+4]!=marker:
            raise b.AdspError(f'V66 PCYCLE/HTID sentinel moved +0x{off:x}')
        trace[off:off+4]=struct.pack('<I',word)
    if any(trace[i:i+4] in (final,middle) for i in range(0,len(trace),4)):
        raise b.AdspError('unresolved thread cycle special instruction')
    # This is a real Hexagon packet even though LLVM6 cannot assemble the
    # privileged R5:4=PCYCLE instruction with the following mpyi increment.
    # The packet continuation bit comes from the typed architecture encoder.
    return (getter+bytes(trace)+isdb_guard(llvm_mc,b)
            +b.assemble(llvm_mc,SRAM_CLONE_SOURCE,SRAM_CLONE_SIZE)
            +b.assemble(llvm_mc,CONTROLLER_POLL_SOURCE,CONTROLLER_POLL_SIZE)
            +debug_snapshot(llvm_mc,b))

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=SHA256_REFERENCE:
        raise ValueError('Wrong ADSP reference SHA-256')
    if len(built)!=SIZE or built!=reference[FILE_START:FILE_END]:
        raise ValueError('source-generated ADSP RWE code differs')
    if any(reference[FILL_START:FILL_END]):
        raise ValueError('expected earlier sparse RWE linker fill is not zero')
    if any(reference[FILL2_START:FILL2_END]):
        raise ValueError('expected following sparse RWE linker fill is not zero')
    # Unknown table is NOT generated from source yet. Reject a missing table
    # but do not count its bytes as reconstructed or include them in output.
    table=reference[UNKNOWN_RECORD_START:UNKNOWN_RECORD_END]
    if len(table)!=0x90 or not any(table):
        raise ValueError('missing/unexpected unclassified RWE table')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',required=True,type=Path)
    args=p.parse_args()
    built=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),built)
    args.output.write_bytes(built)
    print('ADSP next disjoint V66 fragment:',len(built),'bytes file',hex(FILE_START),hex(FILE_END),
          'physical',hex(PHYS_START),'SHA256',hashlib.sha256(built).hexdigest(),
          '; separate from 0x6030..0x6c6c; zero fill and 0xff20 table not source code')
    return 0
if __name__=='__main__':sys.exit(main())
