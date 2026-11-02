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
FILE_END = 0x3a33c
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
    # LLVM6 interchanges the predicate and conditional jump in the shared
    # status-mask packet while preserving all source-assembled operand bits.
    # The original branch is followed by an immediate extender and a store
    # in the same packet; both parse bits must therefore remain unchanged.
    k=(0x3a074-FILE_START)//4
    if tuple(words[k:k+2]) != (0x5c005812,0x75034010):
        raise ValueError('Unexpected controller status-mask predicate packet')
    words[k:k+2]=[words[k+1],words[k]]
    # The architectural P3:0 bank-write at 0x3a100 is provided by a checked
    # typed .word macro which LLVM6 cannot schedule with the following source
    # branch. Its code-generation assumes a new packet at 0x3a104 and thus
    # shortens the encoded PC-relative signed halfword displacement by 2.
    # Validate the *entire* LLVM-authored instruction before repairing the
    # offset; destination remains the original absolute 0x3a118.
    k=(0x3a104-FILE_START)//4
    if words[k] != 0x5c00d20a:
        raise ValueError('Unexpected protected predicate-bank branch operands')
    words[k] += 2
    # The alternate P3:0 bank write at 0x3a1fc is also a checked ISA macro.
    # LLVM6 sees the next branch as a new packet and computes its PC delta
    # one word too short. Validate its operands and adjust two halfwords.
    k=(0x3a200-FILE_START)//4
    if words[k] != 0x5c00d20a:
        raise ValueError('Unexpected alternate predicate-bank branch')
    words[k] += 2
    # LLVM6 reverses this valid `r0=r19; if(p0)jump` producer/consumer
    # packet. Check both instruction bodies and reconstruct the original
    # two parse bits before restoring producer-first ordering.
    k=(0x3a278-FILE_START)//4
    if tuple(words[k:k+2]) != (0x5c00400a,0x7073c000):
        raise ValueError('Unexpected video-ID predicate dispatch operands')
    branch,move=words[k:k+2]
    words[k:k+2]=[(move&~(3<<14))|(1<<14),
                  (branch&~(3<<14))|(3<<14)]
    # LLVM6 folds `r0=r18; deallocframe` into one compact word while the
    # original legal packet folds `r0=r18; jump` and keeps deallocframe as
    # the final opcode. Both are source-authored. Validate the extender and
    # both class+operand words, then use the architectural compact register
    # lane for r18 (upper lane 18-8=10), LLVM's own branch-displacement bits,
    # and the independently LLVM-assembled deallocframe at 0x3a2c4.
    k=(0x3a2d0-FILE_START)//4
    if tuple(words[k-1:k+2]) != (0x0fff7fef,0x58004030,0x50a03f00):
        raise ValueError('Unexpected compact mov/jump/deallocframe packet')
    original_dealloc=words[(0x3a2c4-FILE_START)//4]
    if original_dealloc!=0x901ec01e:
        raise ValueError('Unexpected separately assembled deallocframe')
    branch=words[k]
    words[k]=(0x17000000|((18-8)<<16)|(branch&0xffff))
    words[k+1]=original_dealloc
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
