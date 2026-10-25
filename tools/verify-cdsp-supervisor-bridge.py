#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently assemble the CDSP Hexagon relocator trampoline.

LLVM Hexagon (v60) supports this packet's extended signed immediate and
register arithmetic, including the parse bits indicating shared packets.
This corroborates but does not substitute source-level reconstruction.
"""
import argparse
import importlib.util
from pathlib import Path
import subprocess
import tempfile

ENTRY_OFFSET = 0x3d6c
TRAMPOLINE_SIZE = 20


def make_asm():
    return '''.text
.global cdsp_relocator_trampoline
cdsp_relocator_trampoline:
{
    r30 = ##-0x1fff280
    r31 = add(r31,r25)
}
{ r30 = add(r30,r25) }
{ jumpr r30 }
'''


def parse_hexagon_elf_text(data: bytes) -> bytes:
    # Import the validated dedicated entry-verifier parser by pathname to
    # avoid a new dependency and independent duplication of ELF parsing.
    path = Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec = importlib.util.spec_from_file_location('cdsp_entry_checker', path)
    if spec is None or spec.loader is None:
        raise ValueError('unable to load CDSP entry ELF parser')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.parse_elf32_text(data)


def verify(llvm_mc: Path, rust_output: Path, reference: Path | None) -> None:
    with tempfile.TemporaryDirectory(prefix='nabu-hexagon-trampoline-') as directory:
        asm=Path(directory)/'relocator.s'; obj=Path(directory)/'relocator.o'
        asm.write_text(make_asm())
        subprocess.run([str(llvm_mc), '-triple=hexagon', '-mcpu=hexagonv60',
                        '-filetype=obj','-o',str(obj),str(asm)],check=True)
        independent = parse_hexagon_elf_text(obj.read_bytes())
    source = rust_output.read_bytes()
    if len(independent)!=TRAMPOLINE_SIZE or source!=independent:
        raise ValueError('CDSP relocator Rust-vs-LLVM mismatch')
    print(f'Independent LLVM+Rust Hexagon relocator: {len(source)} identical bytes')
    if reference is not None:
        image=reference.read_bytes()
        if image[ENTRY_OFFSET:ENTRY_OFFSET+len(source)] != source:
            raise ValueError('reference mismatch in CDSP relocator')
        print(f'CDSP reference relocator matches file 0x{ENTRY_OFFSET:x}..0x{ENTRY_OFFSET+len(source):x}')


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--llvm-mc',type=Path,required=True)
    parser.add_argument('--rust-output',type=Path,required=True)
    parser.add_argument('--reference',type=Path)
    args=parser.parse_args()
    verify(args.llvm_mc,args.rust_output,args.reference)

if __name__=='__main__':main()
