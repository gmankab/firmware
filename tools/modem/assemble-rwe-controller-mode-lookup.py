#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build Hexagon controller mode-1 lookup and trap from ISA-based source."""
from __future__ import annotations
import argparse,hashlib,struct,runpy,subprocess,tempfile
from pathlib import Path
FILE_START=0x3a50c
FILE_END=0x3a5a0
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'

def assemble(mc:Path,src:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-v60-mode-lookup-') as td:
        obj=Path(td)/'mode-lookup.o'
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(src),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:raise ValueError('Hexagon V60 assembly failed: '+p.stderr)
        extract=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        data=extract(obj.read_bytes())
    if len(data)!=FILE_END-FILE_START:raise ValueError('Mode-lookup code length wrong')
    w=struct.unpack('<'+'I'*(len(data)//4),data)
    for off,base,special,source,marker in [
            (0x14,0x6c200000,None,0x60,3),
            (0x74,0x67000000,6,15,1),
            (0x88,0x67000000,3,7,3),
            (0x8c,0x6c200000,None,0x80,3)]:
        x=w[off//4]
        if special is None:valid=x==(base|(marker<<14)|source)
        else:valid=x==(base|(marker<<14)|(source<<16)|special)
        if not valid:raise ValueError(f'Unexpected privileged mode instruction at +0x{off:x}')
    if w[0x78//4]&0xc000!=0x4000 or w[0x7c//4]&0xc000!=0x4000 or w[0x80//4]&0xc000!=0xc000:
        raise ValueError('Unexpected packed device status packet markers')
    # The three guarded branches must share the same symbolically assembled
    # error target. Mutated branch-source labels must be detected even if
    # users omit optional reference verification.
    if w[0x2c//4]&0xffffc000!=0x5c00c000 or w[0x44//4]&0xffffc000!=0x5c00c000 or w[0x50//4]&0xffffc000!=0x5c00c000:
        raise ValueError('Wrong Hexagon conditional branch class')
    return data

def verify(ref:bytes,b:bytes)->None:
    sha=hashlib.sha256(ref).hexdigest()
    if sha!=PINNED_SHA256:raise ValueError('Wrong modem revision: '+sha)
    if ref[FILE_START:FILE_END]!=b:raise ValueError('Mode lookup bytes differ from modem reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    repo=Path(__file__).resolve().parents[1]
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--source',type=Path,default=repo/'sources/xiaomi-nabu/modem/modem_rwe_controller_mode_lookup.s')
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args()
    try:
        data=assemble(a.llvm_mc,a.source)
        if a.reference:verify(a.reference.read_bytes(),data)
        a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_bytes(data)
        print(f'MODE LOOKUP [0x{FILE_START:x},0x{FILE_END:x}) {len(data)} source-assembled bytes, SHA256 {hashlib.sha256(data).hexdigest()}')
        if a.reference:print('PINNED MODEM: ZERO BYTE DIFFERENCES')
        return 0
    except (OSError,ValueError) as exc:p.exit(1,f'ERROR: {exc}\n')
if __name__=='__main__':raise SystemExit(main())
