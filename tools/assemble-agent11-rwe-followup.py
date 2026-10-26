#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble and strictly compare Agent 11's next *disjoint* Hexagon helper.

Instruction operands and packet semantics are authored in Hexagon assembly.
The original reference is optional, is hash-pinned, and is never copied into
an emitted byte fragment. The gap before this fragment is explicitly omitted.
"""
import argparse
import hashlib
import runpy
import struct
import subprocess
import tempfile
from pathlib import Path

FILE_START=0x39920
FILE_END=0x39bc0
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'



def normalize_hexagon_gp_relative(data: bytes) -> bytes:
    """Translate LLVM6's generic GP register-index memory class into V60 GP-relative.

    For this one 32-bit load the independently assembled register, displacement,
    extender and marker fields must agree. Fail closed rather than substituting
    a copied instruction word from the private firmware. The replacement high
    16-bit class is public V60 Hexagon GP-relative load encoding.
    """
    # Every load below is emitted by LLVM6 using the exact source operand
    # extender and register. V60 GP-relative memory is a distinct ISA class.
    # Each mapping is guarded by the generic assembler's register/marker word.
    forms=((0x39aec,0x919cc101,0x0c085164),  # R1 = memw(GP+##offset)
           (0x39b20,0x919cc105,0x0c085164), # R5 = memw(GP+##offset)
           (0x39ba0,0x919cc104,0x0c085164)) # R4 = memw(GP+##offset)
    repaired=bytearray(data)
    V60_GP_RELATIVE_WORD_LOAD_CLASS=0x4980
    for address,expected,extword in forms:
        offset=address-FILE_START
        if len(repaired)<offset+4:
            continue
        before=struct.unpack_from('<I',repaired,offset)[0]
        extender=struct.unpack_from('<I',repaired,offset-4)[0]
        if before!=expected or extender!=extword:
            raise ValueError(f'LLVM6 GP-relative controller load class/operands differ at {address:#x}')
        struct.pack_into('<I',repaired,offset,
            (V60_GP_RELATIVE_WORD_LOAD_CLASS<<16)|(before&0xffff))
    if len(repaired) >= 0x39bc0-FILE_START:
        # LLVM6 schedules the conditional compare/jump ahead of its preceding
        # saturating decrement even though both packet orders are legal. The
        # reference uses decrement as continuation, then branch as packet end.
        offset=0x39b80-FILE_START
        branch,subtract=struct.unpack_from('<II',repaired,offset)
        if branch!=0x10444008 or subtract!=0xbfe4ffe4:
            raise ValueError('unexpected Hexagon MODECTL decrement/branch packet')
        continuation_marker=1<<14;end_marker=3<<14;mask=3<<14
        struct.pack_into('<II',repaired,offset,
            (subtract &~mask)|continuation_marker,
            (branch &~mask)|end_marker)
    return bytes(repaired)


def assemble(llvm_mc:Path, source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='modem-rwe-lock-') as tmp:
        obj=Path(tmp)/'followup.o'
        p=subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:
            raise ValueError('Hexagon V60 assembler failed: '+p.stderr)
        data=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object'](obj.read_bytes())
        if len(data)!=FILE_END-FILE_START:
            raise ValueError(f'Unexpected helper length {len(data)}, expected {FILE_END-FILE_START}')
        return normalize_hexagon_gp_relative(data)


def verify(original:bytes,generated:bytes)->None:
    digest=hashlib.sha256(original).hexdigest()
    if digest!=PINNED_SHA256:
        raise ValueError(f'Wrong modem source revision: {digest}')
    data=original[FILE_START:FILE_END]
    if data!=generated:
        at=next((i for i,(a,b) in enumerate(zip(data,generated)) if a!=b),0)
        raise ValueError(f'First mismatch at file 0x{FILE_START+at:x}')


def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    root=Path(__file__).resolve().parents[1]
    p.add_argument('--llvm-mc',required=True,type=Path)
    p.add_argument('--source',type=Path,default=root/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_exception_followup.s')
    p.add_argument('--output',required=True,type=Path)
    p.add_argument('--reference',type=Path)
    args=p.parse_args()
    try:
        data=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),data)
        args.output.write_bytes(data)
    except (OSError,ValueError) as exc:
        p.exit(1,f'ERROR: {exc}\n')
    print(f'Modem second exception helper: [file 0x{FILE_START:x},0x{FILE_END:x}), {len(data)} independently assembled bytes, sha256 {hashlib.sha256(data).hexdigest()}')
    if args.reference:print('Pinned reference matches exactly')
    return 0
if __name__=='__main__':raise SystemExit(main())
