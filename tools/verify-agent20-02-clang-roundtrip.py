#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check the 992-byte Venus controller seam using independent Clang ARM A32.

This script requires the original reference `venus.mbn` at runtime, outside Git.
It reconstructs the section from ARM mnemonics, separating jump-table and
literal data, then verifies its binary against the original AND an optional
source-generated Rust fragment. Never writes OEM code into the repository.

Usage:
  python3 tools/verify-agent20-02-clang-roundtrip.py /private/venus.mbn \
     --emitted /tmp/venus-agent20-02-seam.bin
"""
from __future__ import annotations
import argparse, re, subprocess, tempfile
from pathlib import Path

START, END, FILE_BASE = 0x18000, 0x183e0, 0x4000
DATA_RANGES = ((0x18010,0x1802c),(0x18330,0x18340),(0x183dc,0x183e0))
LINE = re.compile(r'^\s*([0-9a-f]+):\s+([0-9a-f]{8})\s+([a-z0-9.]+)(?:\s+(.*))?$',re.I)
BRANCH = {'b','beq','bne','bhi','bhs','blo','bcc','bcs','bl','blx','bge','blt','ble','bgt'}


def run(args:list[str])->str:
    return subprocess.run(args,check=True,capture_output=True,text=True).stdout


def main()->None:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('reference',type=Path)
    p.add_argument('--emitted',type=Path,help='optional Rust-emitted 992-byte fragment')
    p.add_argument('--clang',default='clang')
    p.add_argument('--objdump',default='llvm-objdump')
    p.add_argument('--objcopy',default='llvm-objcopy')
    a=p.parse_args()
    firmware=a.reference.read_bytes()
    reference=firmware[FILE_BASE+START:FILE_BASE+END]
    if len(reference)!=END-START:raise RuntimeError('Venus reference is truncated')
    with tempfile.TemporaryDirectory(prefix='venus-seam-a32-') as temp:
        d=Path(temp)
        (d/'reference.bin').write_bytes(reference)
        (d/'reference.S').write_text('.syntax unified\n.arm\n.text\n.global entry\nentry:\n.incbin "'+str(d/'reference.bin')+'"\n')
        run([a.clang,'--target=armv7a-none-eabi','-c',str(d/'reference.S'),'-o',str(d/'reference.o')])
        disasm=run([a.objdump,'-d','--adjust-vma='+hex(START),str(d/'reference.o')])
        records={}
        for line in disasm.splitlines():
            m=LINE.match(line)
            if not m:continue
            addr=int(m[1],16)
            if START<=addr<END:
                if addr in records:raise RuntimeError(f'duplicate decoded word {addr:#x}')
                records[addr]=(m[3],(m[4] or '').split(' @ ')[0].strip())
        if len(records)!=(END-START)//4:raise RuntimeError('disassembler missed words')
        asm=['.syntax unified','.arm','.text',f'.org 0x{START:x}']
        typed_instructions=0
        for addr in range(START,END,4):
            mn,ops=records[addr]
            if any(lo<=addr<hi for lo,hi in DATA_RANGES):
                # No byte-level instruction copying: explicit original *data*
                # words must remain literal values in the assembled table.
                word=int.from_bytes(reference[addr-START:addr-START+4],'little')
                asm.append(f'  .word 0x{word:08x} // explicit data at 0x{addr:x}')
                continue
            ops=re.sub(r'\s+<[^>]+>','',ops).strip()
            if mn in BRANCH and ops.startswith('0x'):
                target=int(ops.split()[0],16)
                ops=f'. + (0x{target:x} - 0x{addr:x})'
            asm.append(f'  {mn} {ops} // A32 at 0x{addr:x}')
            typed_instructions+=1
        (d/'roundtrip.S').write_text('\n'.join(asm)+'\n')
        run([a.clang,'--target=armv7a-none-eabi','-c',str(d/'roundtrip.S'),'-o',str(d/'roundtrip.o')])
        run([a.objcopy,'--only-section=.text','-O','binary',str(d/'roundtrip.o'),str(d/'roundtrip.bin')])
        rebuilt=(d/'roundtrip.bin').read_bytes()[-(END-START):]
        if rebuilt!=reference:
            delta=next(i for i,(x,y) in enumerate(zip(rebuilt,reference)) if x!=y)
            raise RuntimeError(f'Clang mnemonic reassembly differs at runtime 0x{START+delta:x}')
        if a.emitted is not None:
            source_bytes=a.emitted.read_bytes()
            if source_bytes!=reference:
                if len(source_bytes)!=len(reference):raise RuntimeError('Rust fragment length differs from 992')
                delta=next(i for i,(x,y) in enumerate(zip(source_bytes,reference)) if x!=y)
                raise RuntimeError(f'Rust emitter differs at runtime 0x{START+delta:x}')
        print(f'PASS: {len(rebuilt)} bytes, {typed_instructions} A32 instructions, 12 data words, zero mismatches')

if __name__=='__main__':main()
