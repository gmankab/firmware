#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build the V60 duplex descriptor shims and controller mask/trap helper."""
from __future__ import annotations
import argparse,hashlib,struct,runpy,subprocess,tempfile
from pathlib import Path
START=0x3a72c
END=0x3a778
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-register-mask-') as td:
        obj=Path(td)/'register-mask.o'
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:raise ValueError('Hexagon LLVM6 rejected source: '+p.stderr)
        extractor=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        data=extractor(obj.read_bytes())
    if len(data)!=END-START:raise ValueError('Hexagon register mask has incorrect instruction size')
    words=struct.unpack('<'+'I'*(len(data)//4),data)
    # Validate three architectural dual-memory duplex words using their
    # register/index field encodings. No OEM bytes are used to generate them.
    for off,expected in [(0x00,0xc0862adc),(0x04,0xc0862a74),(0x08,0xc0862ae0)]:
        if words[off//4]!=expected:
            raise ValueError(f'Unexpected source-built duplex operand at +0x{off:x}')
    privileged={0x18:(0x6e800000,0,25),0x1c:(0x6e800000,6,24),
                0x30:(0x67000000,4,6),0x38:(0x67000000,24,6)}
    for off,(base,reg,sysreg) in privileged.items():
        got=words[off//4]
        want=(base|(3<<14)|(reg<<16)|sysreg)
        if got!=want:raise ValueError(f'Unexpected protected V60 SYS encoding at +0x{off:x}')
    if words[0x34//4]!=0x5400c000 or words[-1]!=0x961ec01e:
        raise ValueError('Trap or dealloc_return source encoding differs')
    return data

def verify(reference:bytes,b:bytes)->None:
    digest=hashlib.sha256(reference).hexdigest()
    if digest!=PINNED_SHA256:raise ValueError('Wrong pinned modem revision: '+digest)
    if len(b)!=END-START or b!=reference[START:END]:raise ValueError('Controller register-mask bytes mismatch')

def main()->int:
    repo=Path(__file__).resolve().parents[1]
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True,type=Path)
    p.add_argument('--reference',type=Path)
    p.add_argument('--source',type=Path,default=repo/'sources/xiaomi-nabu/modem/modem_rwe_controller_register_mask.s')
    p.add_argument('--output',required=True,type=Path)
    a=p.parse_args()
    try:
        data=assemble(a.llvm_mc,a.source)
        if a.reference:verify(a.reference.read_bytes(),data)
        a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_bytes(data)
        print(f'CONTROLLER REGISTER MASK [0x{START:x},0x{END:x}) {len(data)} bytes SHA256 {hashlib.sha256(data).hexdigest()}')
        if a.reference:print('PINNED MODEM: EXACT')
        return 0
    except (OSError,ValueError) as exc:p.exit(1,f'ERROR: {exc}\n')
if __name__=='__main__':raise SystemExit(main())
