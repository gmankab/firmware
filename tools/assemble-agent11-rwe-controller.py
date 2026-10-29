#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble a source-only Hexagon V60 controller dispatch island, fail closed.

The source instructions are emitted through LLVM 6 Hexagon assembler and
validated public architectural fields. Original modem.mbn is never used to
supply emitted opcodes; it is read only for optional SHA-pinned verification.
"""
from __future__ import annotations
import argparse, hashlib, struct, subprocess, tempfile, runpy
from pathlib import Path

FILE_START = 0x39de8
FILE_END = 0x39f8c
VIRTUAL_START = 0xc0841de8
PHYSICAL_START = 0x8e041de8
PINNED_SHA256 = '6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'

def normalize_v60_gp(source:bytes)->bytes:
    """Restore V60 GP instruction class, retaining LLVM-generated operands.

    At file 0x39dfc, LLVM6 emits a legal generic r28 load class 0x919c;
    the modem compiler selected 0x4980. Check the immediate extender, target
    register, low operand fields and packet marker before rewriting only class.
    """
    if len(source)!=FILE_END-FILE_START:
        raise ValueError(f'Unexpected controller section length {len(source)}')
    words=list(struct.unpack('<'+'I'*(len(source)//4),source))
    if words[(0x39df8-FILE_START)//4]!=0x0c084529:
        raise ValueError('Unexpected signed V60 GP-load extender')
    index=(0x39dfc-FILE_START)//4
    if words[index]!=0x919cc681:
        raise ValueError('Unexpected generic GP-load operands or packet marker')
    words[index]=(0x4980<<16)|(words[index]&0xffff)
    # LLVM6 schedules the branch ahead of the mov/duplex in the same legal
    # call packet. Preserve original dependency order, correcting only the
    # architectural signed branch displacement for the moved branch slot.
    # All operand, immediate and parse fields are checked on LLVM-built code.
    a=(0x39e20-FILE_START)//4
    source=tuple(words[a:a+3])
    if source != (0x5dde609a, 0x707c4000, 0x59000011):
        raise ValueError('Unexpected independent predicate/call packet')
    branch,mov,duplex=source
    if (branch & (3<<14))!=(1<<14) or (duplex & (3<<14)) != 0:
        raise ValueError('Incompatible Hexagon packet markers')
    # The moved branch targets the same absolute callable handler.
    # Encoding stores the displacement in signed halfword units.
    words[a:a+3]=[mov,duplex,branch-4]
    # Two other predicate-producing Hexagon packets have the same legal
    # producer-first / branch-first ordering choice. Guard all assembled
    # register, condition and immediate fields before any swap.
    for address,expected_branch,expected_pred,terminal in (
        (0x39f34,0x5c004810,0x751e4460,False),
        (0x39f44,0x5c004808,0x751ec3a0,True),
    ):
        k=(address-FILE_START)//4
        if tuple(words[k:k+2]) != (expected_branch,expected_pred):
            raise ValueError(f'Unexpected video-frame predicate packet at 0x{address:x}')
        b,p=words[k:k+2]
        if terminal:
            # The compare must continue and the conditional jump must close
            # the original packet. No non-marker operand bits are changed.
            words[k:k+2]=[(p&~(3<<14))|(1<<14), (b&~(3<<14))|(3<<14)]
        else:
            words[k:k+2]=[p,b]
    return struct.pack('<'+'I'*len(words),*words)

def assemble(llvm_mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='modem-rwe-controller-') as tmp:
        out=Path(tmp)/'controller.o'
        proc=subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
                             '-filetype=obj',str(source),'-o',str(out)],capture_output=True,text=True)
        if proc.returncode:raise ValueError('Hexagon V60 assembler: '+proc.stderr)
        lib=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))
        return normalize_v60_gp(lib['text_from_object'](out.read_bytes()))

def verify(original:bytes,generated:bytes)->None:
    sha=hashlib.sha256(original).hexdigest()
    if sha!=PINNED_SHA256:raise ValueError(f'Wrong modem source revision: {sha}')
    expected=original[FILE_START:FILE_END]
    if generated!=expected:
        i=next((i for i,(x,y) in enumerate(zip(generated,expected)) if x!=y),0)
        raise ValueError(f'First mismatch at file 0x{FILE_START+i:x}')

def main()->int:
    root=Path(__file__).resolve().parents[1]
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--source',type=Path,default=root/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_exception_controller.s')
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    args=p.parse_args()
    try:
        b=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),b)
        args.output.write_bytes(b)
    except (OSError,ValueError) as exc:
        p.exit(1,f'ERROR: {exc}\n')
    print(f'Controller entry [file 0x{FILE_START:x},0x{FILE_END:x}): {len(b)} Hexagon bytes, sha256 {hashlib.sha256(b).hexdigest()}')
    if args.reference:print('Pinned modem reference matches exactly')
    return 0
if __name__=='__main__':raise SystemExit(main())
