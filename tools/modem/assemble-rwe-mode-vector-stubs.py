#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Hexagon V60 source-build + strict reference check for three disjoint stubs.

Do not conflate the 8-byte uncovered handler [0x3a40c,0x3a414) with
program-source reconstruction: the generated artifact is 56 bytes but the
mapped file coverage consists of two disjoint intervals, 8 and 48 bytes.
"""
from __future__ import annotations
import argparse, hashlib, runpy, struct, subprocess, tempfile
from pathlib import Path

FILE_RANGES=((0x3a404,0x3a40c),(0x3a414,0x3a444))
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
BOUNDARIES=(8,48)
EXPECTED_WORDS={
 0x3a404:0x57e04000,0x3a408:0x9186c740,
 0x3a414:0x57e04000,0x3a418:0x9181c2c0,
 0x3a41c:0x57e04000,0x3a420:0x9181c300,
 0x3a424:0x5c205006,0x3a428:0x9186c801,
 0x3a42c:0x9181c281,0x3a430:0x1230e1f2,
 0x3a434:0x1230e2f4,0x3a438:0x1230e3ea,
 0x3a43c:0x78004000,0x3a440:0x57e0c000,
}

def assemble(llvm_mc:Path,source:Path)->tuple[bytes,bytes]:
    with tempfile.TemporaryDirectory(prefix='agent11-modem-mode-vectors-') as tmp:
        obj=Path(tmp)/'source.o'
        cmd=[str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
             '-filetype=obj',str(source),'-o',str(obj)]
        p=subprocess.run(cmd,capture_output=True,text=True)
        if p.returncode:raise ValueError('Hexagon LLVM6 assembly failed: '+p.stderr)
        extract=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        output=extract(obj.read_bytes())
        if len(output)!=sum(BOUNDARIES):raise ValueError('Invalid original-source helper instruction count')
        a,b=output[:BOUNDARIES[0]],output[BOUNDARIES[0]:]
        validate_instruction_fields((a,b))
        return a,b

def validate_instruction_fields(regions:tuple[bytes,bytes])->None:
    if [len(x) for x in regions]!=list(BOUNDARIES):
        raise ValueError('Invalid Hexagon mode-vector region sizes')
    flat={}
    for (start,end),data in zip(FILE_RANGES,regions):
        for i in range(0,len(data),4):flat[start+i]=struct.unpack_from('<I',data,i)[0]
    # Fail-closed per-opcode and packet-marker checks. Values are documented
    # architecture encodings and test assertions, not copied into the output.
    for address,expected in EXPECTED_WORDS.items():
        if flat[address]!=expected:
            raise ValueError(f'Invalid architecture operand/packet field at modem 0x{address:x}: '
                             f'{flat[address]:08x} vs {expected:08x}')
    assert len(flat)==14 and len(EXPECTED_WORDS)==14

def verify(reference:bytes,regions:tuple[bytes,bytes])->None:
    h=hashlib.sha256(reference).hexdigest()
    if h!=PINNED_SHA256:raise ValueError('Wrong modem source revision: '+h)
    validate_instruction_fields(regions)
    for (start,end),part in zip(FILE_RANGES,regions):
        expected=reference[start:end]
        if len(expected)!=end-start or len(part)!=end-start:
            raise ValueError('Mode-vector source/reference truncation')
        if expected!=part:
            i=next(i for i,(a,b) in enumerate(zip(expected,part)) if a!=b)
            raise ValueError(f'First modem mismatch at file 0x{start+i:x}')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    root=Path(__file__).resolve().parents[1]
    p.add_argument('--llvm-mc',required=True,type=Path)
    p.add_argument('--source',type=Path,default=root/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_mode_vector_stubs.s')
    p.add_argument('--output-prefix',required=True,type=Path)
    p.add_argument('--reference',type=Path)
    a=p.parse_args()
    try:
        regions=assemble(a.llvm_mc,a.source)
        if a.reference:verify(a.reference.read_bytes(),regions)
        a.output_prefix.parent.mkdir(parents=True,exist_ok=True)
        for (start,end),part in zip(FILE_RANGES,regions):
            name=a.output_prefix.with_name(a.output_prefix.name+f'-0x{start:x}-0x{end:x}.bin')
            name.write_bytes(part)
            print(f'HEXAGON_SOURCE_MODE_VECTOR 0x{start:x}–0x{end:x}: {len(part)} '
                  f'sha256 {hashlib.sha256(part).hexdigest()} -> {name}')
        print('FILE_GAP_OMITTED 0x3a40c–0x3a414 (8 bytes, unresolved handler)')
        if a.reference:print('PINNED MODEM REFERENCE: both disjoint ranges match exactly')
        return 0
    except (ValueError,OSError) as e:p.exit(1,f'ERROR: {e}\n')
if __name__=='__main__':raise SystemExit(main())
