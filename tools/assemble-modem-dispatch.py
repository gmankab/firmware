#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build typed Hexagon modem boot dispatch and four bounded map lookups."""
from pathlib import Path
import struct
import argparse
import hashlib
import runpy
import subprocess
import sys
import tempfile

FILE_START = 0x3644
FILE_END = 0x3b68
CONTIGUOUS_START = 0x3000
LENGTH = FILE_END - FILE_START
PREFIX_LENGTH = FILE_START - CONTIGUOUS_START
EXPECTED_MODEM_SHA256 = '6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
text_from_object = runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']


# LLVM 6 packet scheduler canonicalizes the predicate consumer (branch)
# ahead of the producer (cmp). The modem has four legal three-instruction
# packets in the inverse ordering. Restore semantic producer-first order,
# refusing to handle any unexpected instruction class or packet marker.
PREDICATE_LOOKUP_PACKETS = (0x3710, 0x3758, 0x37a0, 0x37e8)
PREDICATE_BRANCH_PACKETS = (0x3814,)
IMMEDIATE_BEFORE_BRANCH_PACKETS = (0x39a0,)
TLB_LOOP_PACKETS = (0x3aa8,)
MARKER_MASK = 3 << 14
CONTINUE = 1 << 14
END_PACKET = 3 << 14


def restore_predicate_order(data: bytes) -> bytes:
    if len(data) != LENGTH or len(data) % 4:
        raise ValueError('unexpected code size for checked packet scheduling')
    words = list(struct.unpack('<'+'I'*(len(data)//4),data))
    for addr in PREDICATE_LOOKUP_PACKETS:
        at=(addr-FILE_START)//4
        jump, comparison, conditional_move = words[at:at+3]
        if (jump >> 24) != 0x5c or (comparison >> 24) != 0xf2 or (conditional_move >> 24) != 0x74:
            raise ValueError(f'unexpected predicated packet opcode class at 0x{addr:x}')
        if [w & MARKER_MASK for w in (jump, comparison, conditional_move)] != [CONTINUE, CONTINUE, END_PACKET]:
            raise ValueError(f'unexpected predicated packet markers at 0x{addr:x}')
        words[at:at+3] = [comparison, jump, conditional_move]
    for addr in PREDICATE_BRANCH_PACKETS:
        at=(addr-FILE_START)//4
        branch, comparison = words[at:at+2]
        if (branch >> 24) != 0x5c or (comparison >> 24) != 0x75:
            raise ValueError(f'unexpected conditional branch packet class at {addr:#x}')
        if [branch & MARKER_MASK,comparison & MARKER_MASK] != [CONTINUE,END_PACKET]:
            raise ValueError(f'unexpected conditional branch packet markers at {addr:#x}')
        words[at:at+2] = [(comparison & ~MARKER_MASK)|CONTINUE,
                          (branch & ~MARKER_MASK)|END_PACKET]
    for addr in IMMEDIATE_BEFORE_BRANCH_PACKETS:
        at=(addr-FILE_START)//4
        branch, extender, destination = words[at:at+3]
        if (branch >> 24) != 0x10 or (extender >> 24) != 0x0c or (destination >> 24) != 0x78:
            raise ValueError(f'unexpected constant-load/jump instruction class at {addr:#x}')
        if [w & MARKER_MASK for w in (branch, extender, destination)] != [CONTINUE,CONTINUE,END_PACKET]:
            raise ValueError(f'unexpected constant-load/jump markers at {addr:#x}')
        words[at:at+3] = [extender, (destination & ~MARKER_MASK)|CONTINUE,
                          (branch & ~MARKER_MASK)|END_PACKET]
    for addr in TLB_LOOP_PACKETS:
        at=(addr-FILE_START)//4
        jump,conditional_decrement,index_increment=words[at:at+3]
        if (jump >> 24) != 0x59 or (conditional_decrement >> 24) != 0x74 or (index_increment >> 24) != 0xb0:
            raise ValueError(f'unexpected Hexagon TLB loop packet class at {addr:#x}')
        if [w & MARKER_MASK for w in (jump,conditional_decrement,index_increment)] != [CONTINUE,CONTINUE,END_PACKET]:
            raise ValueError(f'unexpected Hexagon TLB loop packet markers at {addr:#x}')
        words[at:at+3] = [conditional_decrement,(index_increment & ~MARKER_MASK)|CONTINUE,
                          (jump & ~MARKER_MASK)|END_PACKET]
    return struct.pack('<'+'I'*len(words),*words)

def assemble(llvm_mc: Path) -> bytes:
    source = Path(__file__).resolve().parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_boot_dispatch.s'
    with tempfile.TemporaryDirectory(prefix='modem-dispatch-') as tmp:
        obj = Path(tmp)/'dispatch.o'
        cmd=[str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)]
        result=subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode:
            raise RuntimeError('Hexagon assembler failed: '+result.stderr)
        compiled=text_from_object(obj.read_bytes())
    if len(compiled)!=LENGTH:
        raise ValueError(f'dispatcher length {len(compiled)} bytes, expected {LENGTH}')
    return restore_predicate_order(compiled)


def verify(reference: bytes, generated: bytes, file_start: int) -> None:
    digest=hashlib.sha256(reference).hexdigest()
    if digest!=EXPECTED_MODEM_SHA256:
        raise ValueError(f'wrong modem reference digest {digest}')
    if len(reference)<file_start+len(generated):
        raise ValueError('reference truncated before the reconstructed region ends')
    expected=reference[file_start:file_start+len(generated)]
    if expected!=generated:
        i=next(i for i,(a,b) in enumerate(zip(expected,generated)) if a!=b)
        raise ValueError(f'Hexagon mismatch at file offset {file_start+i:#x}: want {expected[i]:02x}, got {generated[i]:02x}')


def main() -> None:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--prefix',type=Path,help='prior 1604-byte reconstructed bootstrap')
    p.add_argument('--reference',type=Path,help='private unmodified modem.mbn')
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args()
    fragment=assemble(args.llvm_mc)
    if args.prefix is not None:
        prior=args.prefix.read_bytes()
        if len(prior)!=PREFIX_LENGTH:
            raise ValueError(f'prefix size {len(prior)}, expected {PREFIX_LENGTH}')
        generated,start=prior+fragment,CONTIGUOUS_START
    else:
        generated,start=fragment,FILE_START
    if args.reference is not None:
        verify(args.reference.read_bytes(),generated,start)
        print(f'Exact modem reference verification [{start:#x},{start+len(generated):#x})')
    args.output.write_bytes(generated)
    print(f'Produced {len(generated)} independent bytes, SHA-256 {hashlib.sha256(generated).hexdigest()} (PARTIAL firmware)')


if __name__=='__main__':
    try: main()
    except (ValueError,RuntimeError,OSError) as e:
        print(f'ERROR: {e}',file=sys.stderr);sys.exit(1)
