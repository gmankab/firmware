#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build the next independent ADSP RWE Hexagon record-controller prefix.

The instructions begin at firmware file 0x10c40, after 996 true zero-fill
bytes [0x1084c,0x10c30) and a separate 16-byte unclassified table. No
reference bytes are used in output. The unfinished routine is not yet a
whole callable function and must not be misrepresented as one.
"""
from __future__ import annotations
import argparse,hashlib,importlib.util,sys
from pathlib import Path
TOOLS=Path(__file__).resolve().parent
SOURCE=TOOLS/'adsp-agent20-rwe-next-controller-records.S'
FILE_START=0x10c40
FILE_END=0x10cb4
SIZE=FILE_END-FILE_START
REFERENCE_SHA256='4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'

def base():
    spec=importlib.util.spec_from_file_location('validated_hexagon_source',TOOLS/'emit-adsp-entry.py')
    if not spec or not spec.loader:raise RuntimeError('Missing ADSP source assembler')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m

def render(mc:str)->bytes:
    return base().assemble(mc,SOURCE,SIZE)

def verify(reference:bytes,built:bytes)->None:
    if hashlib.sha256(reference).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('Wrong ADSP reference')
    if len(built)!=SIZE or built!=reference[FILE_START:FILE_END]:
        raise ValueError('ADSP controller records differ from reference')
    if any(reference[0x1084c:0x10c30]):
        raise ValueError('Expected unimplemented linker zeros before controller records')
    # The 16-byte table is not emitted as executable code; its identity is
    # checked for reporting only. A different table requires further analysis.
    if reference[0x10c30:0x10c40]!=(b'\xff'*8+b'\x00'*8):
        raise ValueError('Unexpected auxiliary record table before function')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',required=True)
    p.add_argument('--reference',type=Path)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();output=render(args.llvm_mc)
    if args.reference:verify(args.reference.read_bytes(),output)
    args.output.write_bytes(output)
    print('ADSP controller record prefix:',len(output),'exact code bytes',
          f'[0x{FILE_START:x},0x{FILE_END:x}), sha256',hashlib.sha256(output).hexdigest(),
          '; original linker fill/table excluded')
    return 0
if __name__=='__main__':sys.exit(main())
