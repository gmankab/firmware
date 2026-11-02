#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Hexagon V60 secondary-mode lookup and IRQ bit-clear assembler."""
from __future__ import annotations
import argparse,hashlib,struct,runpy,subprocess,tempfile
from pathlib import Path
START=0x3a5a0
END=0x3a648
PINNED_SHA256='6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'

def assemble(mc:Path,source:Path)->bytes:
    with tempfile.TemporaryDirectory(prefix='agent11-controller-irq-secondary-') as td:
        obj=Path(td)/'second.o'
        p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)],capture_output=True,text=True)
        if p.returncode:raise ValueError('LLVM6 refused source: '+p.stderr)
        get=runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        built=get(obj.read_bytes())
    if len(built)!=END-START:raise ValueError('Wrong assembled secondary mode size')
    words=struct.unpack('<'+'I'*(len(built)//4),built)
    # Fail closed on the architecturally privileged register/action fields,
    # status register S6 and the two independent RTE epilogues.
    checks={0x00:0x6c20c060,0x60:0x670f4006,0x74:0x6707c003,
            0x78:0x6c20c080,0x7c:0x57e0c000,0x80:0x6c20c060,
            0x94:0x670c4006,0x9c:0x670bc003,0xa0:0x6c20c080,
            0xa4:0x57e0c000}
    for off,expect in checks.items():
        if words[off//4]!=expect:
            raise ValueError(f'Unexpected source-assembled V60 privileged field at +0x{off:x}')
    # Check branch class & packet continuation of tstbit and its skip path.
    if (words[0x88//4] & 0xc000)!=0x4000 or (words[0x8c//4]&0xc000)!=0x4000 or (words[0x90//4]&0xc000)!=0xc000:
        raise ValueError('Malformed IRQ test/conditional-branch packet')
    if words[0x8c//4]&0xff00c000!=0x5c004000:
        raise ValueError('Unexpected IRQ conditional-branch opcode class')
    return built

def verify(ref:bytes,b:bytes)->None:
    digest=hashlib.sha256(ref).hexdigest()
    if digest!=PINNED_SHA256:raise ValueError('Wrong modem reference revision: '+digest)
    if len(b)!=END-START or ref[START:END]!=b:raise ValueError('Secondary mode bytes differ from pinned reference')

def main()->int:
    a=argparse.ArgumentParser(description=__doc__)
    root=Path(__file__).resolve().parents[1]
    a.add_argument('--llvm-mc',type=Path,required=True)
    a.add_argument('--reference',type=Path)
    a.add_argument('--source',type=Path,default=root/'sources/xiaomi-nabu/modem/modem_rwe_controller_mode_secondary.s')
    a.add_argument('--output',type=Path,required=True)
    args=a.parse_args()
    try:
        b=assemble(args.llvm_mc,args.source)
        if args.reference:verify(args.reference.read_bytes(),b)
        args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_bytes(b)
        print(f'SECONDARY MODE [0x{START:x},0x{END:x}) {len(b)} source-assembled bytes SHA256 {hashlib.sha256(b).hexdigest()}')
        if args.reference:print('PINNED MODEM: EXACT')
        return 0
    except (OSError,ValueError) as exc:a.exit(1,f'ERROR: {exc}\n')
if __name__=='__main__':raise SystemExit(main())
