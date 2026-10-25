#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build typed Hexagon modem early-dispatch continuation [0x3644,0x36e0)."""
from pathlib import Path
import argparse
import hashlib
import runpy
import subprocess
import sys
import tempfile

FILE_START = 0x3644
FILE_END = 0x36e0
CONTIGUOUS_START = 0x3000
LENGTH = FILE_END - FILE_START
PREFIX_LENGTH = FILE_START - CONTIGUOUS_START
EXPECTED_MODEM_SHA256 = '6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'
text_from_object = runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']


def assemble(llvm_mc: Path) -> bytes:
    source = Path(__file__).resolve().parents[1]/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_boot_dispatch.s'
    with tempfile.TemporaryDirectory(prefix='modem-dispatch-') as tmp:
        obj = Path(tmp)/'dispatch.o'
        cmd=[str(llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(source),'-o',str(obj)]
        result=subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode:
            raise RuntimeError('Hexagon assembler failed: '+result.stderr)
        compiled=text_from_object(obj.read_bytes())
    if len(compiled)!=LENGTH:
        raise ValueError(f'dispatcher length {len(compiled)} bytes, expected {LENGTH}')
    return compiled


def verify(reference: bytes, generated: bytes, file_start: int) -> None:
    digest=hashlib.sha256(reference).hexdigest()
    if digest!=EXPECTED_MODEM_SHA256:
        raise ValueError(f'wrong modem reference digest {digest}')
    if len(reference)<file_start+len(generated):
        raise ValueError('reference truncated before the reconstructed region ends')
    expected=reference[file_start:file_start+len(generated)]
    if expected!=generated:
        i=next(i for i,(a,b) in enumerate(zip(expected,generated)) if a!=b)
        raise ValueError(f'Hexagon mismatch at file offset {file_start+i:#x}: want {expected[i]:02x}, got {generated[i]:02x}')


def main() -> None:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm-mc',type=Path,required=True)
    p.add_argument('--prefix',type=Path,help='prior 1604-byte reconstructed bootstrap')
    p.add_argument('--reference',type=Path,help='private unmodified modem.mbn')
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args()
    fragment=assemble(args.llvm_mc)
    if args.prefix is not None:
        prior=args.prefix.read_bytes()
        if len(prior)!=PREFIX_LENGTH:
            raise ValueError(f'prefix size {len(prior)}, expected {PREFIX_LENGTH}')
        generated,start=prior+fragment,CONTIGUOUS_START
    else:
        generated,start=fragment,FILE_START
    if args.reference is not None:
        verify(args.reference.read_bytes(),generated,start)
        print(f'Exact modem reference verification [{start:#x},{start+len(generated):#x})')
    args.output.write_bytes(generated)
    print(f'Produced {len(generated)} independent bytes, SHA-256 {hashlib.sha256(generated).hexdigest()} (PARTIAL firmware)')


if __name__=='__main__':
    try: main()
    except (ValueError,RuntimeError,OSError) as e:
        print(f'ERROR: {e}',file=sys.stderr);sys.exit(1)
