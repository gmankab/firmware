#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Source-rebuild two Hexagon V60 controller system-register accessors.

LLVM6 cannot display C6/other privileged architectural register names; the
source generates their instruction fields with checked macros. The generated
instructions do not contain copied OEM code words, and the original modem
image is loaded only for an optional pinned-revision comparison.
"""
from __future__ import annotations
import argparse,hashlib,runpy,struct,subprocess,tempfile
from pathlib import Path
START=0x3aab4
END=0x3ab1c
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'

def creg_read(dest:int,selector:int,marker:int=1)->int:
    if not(0<=dest<=31 and 0<=selector<=63 and marker in (1,3)):
        raise ValueError('Invalid privileged controller read operands')
    return 0x6e800000 | (selector<<16) | (marker<<14) | dest

def creg_write(sel:int,src:int,marker:int=3)->int:
    if not(0<=sel<=63 and 0<=src<=31 and marker in (1,3)):
        raise ValueError('Invalid privileged controller write operands')
    return 0x6c800000 | (src<<16) | (marker<<14) | sel

def validate(data:bytes)->bytes:
    if len(data)!=END-START:raise ValueError(f'Expected {END-START} bytes, got {len(data)}')
    w=struct.unpack('<26I',data)
    for idx,target in [(0,creg_read(2,6)),(6,creg_write(2,2)),(13,creg_read(2,6)),(19,creg_write(1,2))]:
        if w[idx]!=target:raise ValueError(f'Unexpected special register opcode at +{idx*4:x}')
    for idx in (12,23,25):
        if w[idx]!=0x529fc000:raise ValueError(f'Unexpected Hexagon jumpr return at +{idx*4:x}')
    if w[7]&0xc000 !=0x4000 or w[8]&0xc000 !=0x4000 or w[9]&0xc000 !=0xc000:
        raise ValueError('Unexpected predicate/conditional store packet markers')
    return data

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-special-registers-') as td:
        obj=Path(td)/'ctrl.o'
        proc=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if proc.returncode:raise ValueError('LLVM6 refused special access source: '+proc.stderr)
        extract=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        return validate(extract(obj.read_bytes()))

def verify(ref:bytes,b:bytes)->None:
    got=hashlib.sha256(ref).hexdigest()
    if got!=PINNED_SHA256:raise ValueError('Wrong pinned modem revision: '+got)
    validate(b)
    if b!=ref[START:END]:raise ValueError('Controller register accessors differ from reference')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    repo=Path(__file__).resolve().parents[1]
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--source',type=Path,default=repo/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_controller_special_ctrl_access.s')
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args()
    try:
        b=assemble(a.llvm_mc,a.source)
        if a.reference:verify(a.reference.read_bytes(),b)
        a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_bytes(b)
        print(f'PROTECTED CONTROLLER ACCESS [0x{START:x},0x{END:x}) {len(b)} bytes SHA256 {hashlib.sha256(b).hexdigest()}')
        if a.reference:print('PINNED MODEM: ZERO BYTE DIFFERENCES')
        return 0
    except (OSError,ValueError) as exc:p.exit(1,f'ERROR: {exc}\n')
if __name__=='__main__':raise SystemExit(main())
