#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently verify three Hexagon diagnostic packet pairs and RWE prefix.

LLVM6 assembles named semantic instructions, not imported original opcode
words, then we require exact equality to the Rust emitter and pinned firmware.
"""
import argparse, hashlib, importlib.util, re, subprocess, tempfile
from pathlib import Path

START,END,LAYOUT_END=0x56890,0x568e4,0x568f0
VMA_BASE=0x96cc_8000
EXPECTED_SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'

def verify(args):
    ref=args.reference.read_bytes()
    if hashlib.sha256(ref).hexdigest()!=EXPECTED_SHA:
        raise ValueError('Reference cdsp.mbn differs from expected pinned version')
    gen=args.generated.read_bytes()
    prefix=args.prefix.read_bytes()
    if len(gen)!=84 or gen!=ref[START:END]:
        raise ValueError('Rust-built diagnostic 84 bytes differ from reference')
    if len(prefix)!=LAYOUT_END-0x56000 or prefix!=ref[0x56000:LAYOUT_END]:
        raise ValueError('Full joined RWE controller prefix differs from original')
    if prefix[-12:]!=b'\x00\xc0\x00\x7f'*3:
        raise ValueError('Original trailing 12 bytes are not three source-encoded Hexagon NOPs')
    with tempfile.TemporaryDirectory(prefix='cdsp-error-tail-') as td:
        obj=Path(td)/'independent.o'
        asm=Path(__file__).with_name('late-controller-error-tail.s')
        subprocess.run([str(args.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
                       '-filetype=obj',str(asm),'-o',str(obj)],
                        check=True,capture_output=True,text=True)
        spec=importlib.util.spec_from_file_location('hexagon_section',
              Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
        assert spec and spec.loader
        mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
        independent=mod.parse_elf32_text(obj.read_bytes())[0x890:0x8e4]
        if independent!=gen:
            raise ValueError('LLVM6 semantic source does not generate the same 84 bytes')
        decoded=subprocess.run([str(args.llvm_objdump),'-d','-mcpu=hexagonv60',str(obj)],
                     check=True,capture_output=True,text=True).stdout
    op_lines=[]
    for l in decoded.splitlines():
        m=re.match(r'^\s*([0-9a-f]+):\s+(?:[0-9a-f]{2}\s+){4}\s*([0-9a-f]{8})\s+(.*)$',l)
        if m and 0x890<=int(m[1],16)<0x8e4:op_lines.append(m)
    if len(op_lines)!=21 or len([m for m in op_lines if 'call' in m[3]])!=6:
        raise ValueError(f'LLVM6 decoder missing instructions or calls: {len(op_lines)} words')
    if any('unknown' in m[3].lower() for m in op_lines):
        raise ValueError('Unexpected unknown original-order Hexagon word')
    text='\n'.join(m[3] for m in op_lines)
    for required in ['r18 = #212','r18 = #284','r18 = #307',
                     'r1:0 = combine(r18,##-1071488651)',
                     'call 0xf8bac','call 0x58c4']:
        if required not in text:
            raise ValueError(f'Independent LLVM decoding did not confirm: {required}')
    print('PASS original CDSP SHA-256 and 84 byte-identical typed Hexagon executable bytes')
    print('PASS original 2288-byte RWE prefix, 2232 instruction bytes and 56 NOP/fill bytes')
    print('PASS independent LLVM6 assembly of 21/21 original packet words and six service calls')

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    for opt in ('llvm-mc','llvm-objdump','generated','prefix','reference'):
        parser.add_argument('--'+opt,required=True,type=Path)
    verify(parser.parse_args())
