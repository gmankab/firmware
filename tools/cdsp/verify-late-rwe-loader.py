#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Check CDSP Hexagon RWE constructor loader: exact Rust, LLVM6 and Rizin."""
from __future__ import annotations
import argparse,importlib.util,subprocess,tempfile
from pathlib import Path
OFFSET=0x56060
PHYS=0x96cc8060
LEN=0x74

def verify(llvm:Path,rz:Path,generated:Path,reference:Path|None)->None:
    blob=generated.read_bytes()
    if len(blob)==0x114:code=blob[0x60:0x60+LEN]
    elif len(blob)==LEN:code=blob
    else:raise ValueError('Expected 116-byte typed callback loader or full 276-byte RWE prefix')
    with tempfile.TemporaryDirectory(prefix='cdsp-rwe-callback-loader-') as tmp:
        obj=Path(tmp)/'loader.o'
        subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
                        str(Path(__file__).with_name('late-rwe-loader.s')),'-o',str(obj)],check=True)
        dep=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
        spec=importlib.util.spec_from_file_location('cdsp_text_parser',dep)
        assert spec and spec.loader
        mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
        independently_assembled=mod.parse_elf32_text(obj.read_bytes())[0x60:]
    if len(independently_assembled)!=LEN-4 or independently_assembled[4:]!=code[8:]:
        raise ValueError('LLVM6 semantics for 112 following instruction bytes disagree with source')
    # LLVM6 chooses a shorter GP-relative store at the beginning, whereas
    # original V66 uses forced immext. The two words are checked separately.
    decoder=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYS),'-D',code.hex()],
                           capture_output=True,text=True,check=True)
    lines=[s for s in decoder.stdout.splitlines() if s.startswith('0x')]
    if len(lines)!=LEN//4 or any('invalid' in s.lower() for s in lines):
        raise ValueError('Rizin V66 must decode all 29 Hexagon words')
    text='\n'.join(lines).replace(' ','').lower()
    for token in ['memw(gp+##0xc036488c)=r3','call0x96ccd8e0',
                  'sp=add(sp,##-0x8)','call0x96dc3344','jump0x96cc80d0']:
        if token not in text:raise ValueError('Missing decoded loader instruction '+token)
    if reference and reference.read_bytes()[OFFSET:OFFSET+LEN]!=code:
        raise ValueError('Typed CDSP runtime loader differs from pinned reference')
    print('PASS: 116 byte-exact CDSP loader bytes; LLVM6 reproduces 112 tail after compressed GP store, Rizin validates forced-extended packet')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True);p.add_argument('--rz-asm',type=Path,required=True)
    p.add_argument('--generated',type=Path,required=True);p.add_argument('--reference',type=Path)
    a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference)
