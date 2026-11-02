#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-assemble the separately mapped Hexagon RWE exception-vector table.

The output has 49 executable branch entries, 4 bytes of intentional file
layout separation, and 14 architectural RTE instructions. All source
operands are independently authored and OEM reference bytes are read only
when the caller requests pinned-revision validation.
"""
from __future__ import annotations
import argparse,hashlib,runpy,struct,subprocess,sys,tempfile
from pathlib import Path

FILE_START=0x38000
FILE_END=0x38100
ELF_VADDR=0xc0840000
PHYSICAL_START=0x8e040000
PINNED_REFERENCE_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
BRANCH_BYTES=49*4
PADDING_BYTES=4
RTE_BYTES=14*4


def assemble(llvm_mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='modem-vectors-') as temp:
        obj=Path(temp)/'vectors.o'
        p=subprocess.run([str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:raise ValueError(f'Hexagon V60 vector assembler failure: {p.stderr}')
        load=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        data=load(obj.read_bytes())
        if len(data)!=FILE_END-FILE_START:
            raise ValueError(f'RWE vector layout has {len(data)} vs {FILE_END-FILE_START} bytes')
        if len(data)!=BRANCH_BYTES+PADDING_BYTES+RTE_BYTES:
            raise ValueError('vector layout accounting inconsistent')
        words=list(struct.unpack('<64I',data))
        # Decode 49 ISA jump operations via the architectural immediate fields.
        # LLVM6 generated every branch from named destination labels.
        for index,word in enumerate(words[:49]):
            if (word & 0xc000)!=(3<<14) or (word & 0xfe000000) not in (0x58000000,0x5a000000):
                raise ValueError(f'vector branch at +0x{4*index:x} not a standalone Hexagon jump')
        if words[49]!=0:raise ValueError('expected one explicit non-executable separator word')
        if any((word&~0xc000)!=0x57e00000 or (word&0xc000)!=(3<<14) for word in words[50:]):
            raise ValueError('expected exactly fourteen standalone Hexagon RTE operations')
        return data


def verify_pinned(reference:bytes,data:bytes)->None:
    digest=hashlib.sha256(reference).hexdigest()
    if digest!=PINNED_REFERENCE_SHA256:
        raise ValueError(f'wrong modem.mbn reference revision: {digest}')
    expected=reference[FILE_START:FILE_END]
    if len(expected)!=len(data):raise ValueError('reference file truncated before last RTE')
    if data!=expected:
        i=next(i for i,(a,b) in enumerate(zip(data,expected)) if a!=b)
        raise ValueError(f'first vector mismatch at file 0x{FILE_START+i:x}: generated {data[i]:02x}, reference {expected[i]:02x}')


def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    root=Path(__file__).resolve().parents[1]
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--source',type=Path,default=root/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_irq_vectors.s')
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    a=p.parse_args()
    try:
        data=assemble(a.llvm_mc,a.source)
        if a.reference:verify_pinned(a.reference.read_bytes(),data)
        a.output.write_bytes(data)
        print(f'Hexagon modem RWE exception-vector file [0x{FILE_START:x},0x{FILE_END:x}), physical [0x{PHYSICAL_START:x},0x{PHYSICAL_START+len(data):x}): {BRANCH_BYTES+RTE_BYTES} instruction bytes + {PADDING_BYTES} explicit separator bytes')
        print(f'SHA256 {hashlib.sha256(data).hexdigest()}')
        if a.reference:print('Pinned reference matches all 256 layout bytes exactly')
        print('Disjoint from prior source fragments; no authenticated full modem image generated')
        return 0
    except (OSError,ValueError) as exc:
        print('ERROR:',exc,file=sys.stderr)
        return 1

if __name__=='__main__':sys.exit(main())
