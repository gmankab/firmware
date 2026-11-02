#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild Hexagon EXTRACTU+RTE pair closing the 8-byte controller gap.

LLVM6 understands the genuine unsigned extraction, but not privileged RTE
inside the same packet. Changing the source-assembled EXTRACTU packet marker
from 3 to 1 is allowed only for an exact architecture-checked operand word.
The original MBN is used solely for pinned revision/byte equality checks.
"""
from __future__ import annotations
import argparse, hashlib, runpy, struct, subprocess, tempfile, importlib.util
from pathlib import Path

FILE_START=0x3a40c
FILE_END=0x3a414
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
LLVM_STANDALONE_EXTRACTU=0x8d2fc700  # Verified opcode/operands r0=extractu(r15,#7,#8), marker 3
RTE_END=0x57e0c000                # Hexagon privileged return from exception, marker 3

def normalize_packet(code:bytes)->bytes:
    if len(code)!=8:raise ValueError('Invalid controller-mode EXTRACTU/RTE instruction count')
    extractu,rte=struct.unpack('<II',code)
    if extractu!=LLVM_STANDALONE_EXTRACTU or rte!=RTE_END:
        raise ValueError('Unrecognized source-assembled EXTRACTU operands or privileged RTE')
    # Clear the packet end bit 15 in an LLVM-generated, operand-checked word.
    # Preserve V60 EXTRACTU function class and r15/width 7/offset 8/r0 fields.
    if (extractu & 0xc000)!=0xc000:raise ValueError('EXTRACTU missing expected LLVM packet end')
    packed=(extractu & ~(1<<15))
    if (packed & 0xc000)!=0x4000:raise ValueError('Invalid continued EXTRACTU packet parse bits')
    return struct.pack('<II',packed,rte)

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-hexagon-mode-extract-') as tmp:
        obj=Path(tmp)/'mode-extract.o'
        r=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60',
            '-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if r.returncode:raise ValueError('Hexagon assembler rejected source: '+r.stderr)
        get_text=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        return normalize_packet(get_text(obj.read_bytes()))

def verify(reference:bytes,data:bytes)->None:
    sha=hashlib.sha256(reference).hexdigest()
    if sha!=PINNED_SHA256:raise ValueError('Wrong modem source revision: '+sha)
    if len(data)!=FILE_END-FILE_START:raise ValueError('Invalid generated packet length')
    expect=reference[FILE_START:FILE_END]
    if data!=expect:
        i=next(i for i,(a,b) in enumerate(zip(data,expect)) if a!=b)
        raise ValueError(f'Hexagon controller mode packet mismatch at 0x{FILE_START+i:x}')

def emit_contiguous_64(mc:Path,reference:bytes|None,mode_data:bytes,output:Path)->None:
    """Join three source-built intervals, never padding with vendor bytes."""
    helper=Path(__file__).with_name('assemble-rwe-mode-vector-stubs.py')
    spec=importlib.util.spec_from_file_location('agent11_mode_vector_stubs',helper)
    lib=importlib.util.module_from_spec(spec);spec.loader.exec_module(lib)
    src=helper.parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_mode_vector_stubs.s'
    first,second=lib.assemble(mc,src)
    if reference is not None:lib.verify(reference,(first,second))
    combined=first+mode_data+second
    assert len(combined)==64
    if reference is not None:
        if reference[0x3a404:0x3a444]!=combined:raise ValueError('Full mode dispatch is not byte-exact')
    output.write_bytes(combined)
    print(f'FULL_MODE_DISPATCH 0x3a404–0x3a444 64 source-built contiguous bytes sha256 {hashlib.sha256(combined).hexdigest()}')

def main()->int:
    args=argparse.ArgumentParser(description=__doc__)
    root=Path(__file__).resolve().parents[1]
    args.add_argument('--llvm-mc',type=Path,required=True)
    args.add_argument('--source',type=Path,default=root/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_mode_extract_rte.s')
    args.add_argument('--reference',type=Path)
    args.add_argument('--output',type=Path,required=True)
    args.add_argument('--output-combined-64',type=Path)
    a=args.parse_args()
    try:
        data=assemble(a.llvm_mc,a.source)
        reference=a.reference.read_bytes() if a.reference else None
        if reference is not None:verify(reference,data)
        a.output.parent.mkdir(parents=True,exist_ok=True)
        a.output.write_bytes(data)
        print(f'MODE3_EXTRACTU_RTE 0x{FILE_START:x}–0x{FILE_END:x}: 8 exact bytes sha256 {hashlib.sha256(data).hexdigest()}')
        if a.output_combined_64:emit_contiguous_64(a.llvm_mc,reference,data,a.output_combined_64)
        if reference is not None:print('PINNED MODEM REFERENCE VERIFIED')
        return 0
    except (ValueError,OSError) as err:args.exit(1,f'ERROR: {err}\n')
if __name__=='__main__':raise SystemExit(main())
