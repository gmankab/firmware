#!/usr/bin/env python3
"""Strictly assemble/verify Agent 20-04's [0x40000,0x68000) Venus region.

All input fragments MUST come from the agent's typed Rust emitters. No OEM
bytes are read except in optional --reference *comparison*, and unknown holes
are never filled. This is not a bootable or newly signed firmware image.
"""
from __future__ import annotations
import argparse,hashlib,re,sys
from pathlib import Path
START=0x40000
MIDDLE_START=0x40a2c
MIDDLE_END=0x55400
TAIL_END=0x68000
MBN_FILE_BASE=0x4000
EXPECTED_TOTAL=TAIL_END-START

def assemble(prefix:bytes, middle:list[tuple[int,bytes]], tail:bytes)->bytes:
    if len(prefix)!=MIDDLE_START-START:
        raise ValueError(f"prefix must be {MIDDLE_START-START} bytes, got {len(prefix)}")
    out=bytearray(prefix)
    next_address=MIDDLE_START
    if not middle:raise ValueError("no middle fragments received")
    for index,(start,part) in enumerate(middle):
        if not part or len(part)%4:raise ValueError(f"middle fragment {index} is empty or unaligned")
        if start!=next_address:raise ValueError(f"overlap or hole: expected ARM 0x{next_address:x}, got 0x{start:x}")
        if start+len(part)>MIDDLE_END:raise ValueError("middle fragment exceeded 0x55400")
        out.extend(part)
        next_address+=len(part)
    if next_address!=MIDDLE_END:raise ValueError(f"middle fragments stop at 0x{next_address:x}, expected 0x55400")
    if len(tail)!=TAIL_END-MIDDLE_END:raise ValueError(f"continuation length {len(tail)} does not reach 0x68000")
    out.extend(tail)
    if len(out)!=EXPECTED_TOTAL:raise ValueError("final range length differs from 0x28000")
    return bytes(out)

def collect_middle(prefix:Path)->list[tuple[int,bytes]]:
    files=prefix.parent.glob(prefix.name+"-*.bin")
    result=[]
    for p in files:
        m=re.fullmatch(re.escape(prefix.name)+r"-([0-9a-fA-F]+)\.bin",p.name)
        if m is None:continue
        result.append((int(m[1],16),p.read_bytes()))
    result.sort(key=lambda x:x[0])
    return result

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--prefix',type=Path,required=True,help='source-emitted 2604-byte ARM32 fragment')
    p.add_argument('--middle-prefix',type=Path,required=True,help='basename of agent20_04_venus_middle --emit outputs')
    p.add_argument('--tail',type=Path,required=True,help='source-emitted 0x55400-0x68000 span')
    p.add_argument('--reference',type=Path,help='private pinned venus.mbn for exact comparison; never copied into generator')
    p.add_argument('--output',type=Path,help='write the combined source-emitted 163840-byte fragment')
    args=p.parse_args()
    try:
        result=assemble(args.prefix.read_bytes(),collect_middle(args.middle_prefix),args.tail.read_bytes())
        if args.reference:
            reference=args.reference.read_bytes()
            b=reference[START+MBN_FILE_BASE:TAIL_END+MBN_FILE_BASE]
            if len(b)!=EXPECTED_TOTAL:raise ValueError('reference MBN lacks the full requested executable span')
            if result!=b:
                first=next(i for i,(a,x) in enumerate(zip(result,b)) if a!=x)
                raise ValueError(f"byte mismatch at runtime 0x{START+first:x}, MBN offset 0x{START+MBN_FILE_BASE+first:x}")
        if args.output:args.output.write_bytes(result)
        print(f"Agent 20-04 complete: {len(result)} bytes VMA [0x{START:x},0x{TAIL_END:x})",
              f"MBN [0x{START+MBN_FILE_BASE:x},0x{TAIL_END+MBN_FILE_BASE:x})",
              f"SHA-256 {hashlib.sha256(result).hexdigest()}",
              'original reference exact' if args.reference else 'reference comparison not requested',sep='; ')
    except (OSError,ValueError) as exc:
        print(f'Agent 20-04 full span: {exc}',file=sys.stderr)
        return 1
    return 0

if __name__=='__main__':sys.exit(main())
