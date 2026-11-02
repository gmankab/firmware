#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Bounded source/firmware inspection; never load whole files or dump huge logs."""
from __future__ import annotations
import argparse
from pathlib import Path
import sys

MAX_OUTPUT=64*1024
MAX_LINES=120
MAX_BINARY=4096


def inspect_lines(path:Path, start:int, count:int)->str:
    if start<1 or count<1 or count>MAX_LINES:
        raise ValueError('invalid line window (start>=1, count=1..120)')
    chunks=[];used=0
    with path.open('rb') as f:
        for i in range(1,start+count):
            data=f.readline(MAX_OUTPUT+1)
            if not data:break
            # Python file iteration can buffer an entire giant single line.
            # A bounded readline rejects such files before allocating them.
            if len(data)>MAX_OUTPUT or (len(data)==MAX_OUTPUT and not data.endswith(b'\n')):
                raise ValueError('source line exceeds 64 KiB inspection cap')
            if i>=start:
                line=f'{i:>8}  '.encode()+data
                used+=len(line)
                if used>MAX_OUTPUT:raise ValueError('requested output exceeds 64 KiB')
                chunks.append(line.decode('utf-8',errors='replace'))
    return ''.join(chunks)


def inspect_bytes(path:Path,offset:int,length:int)->bytes:
    if offset<0 or not 1<=length<=MAX_BINARY:
        raise ValueError('invalid byte window (offset>=0, length=1..4096)')
    with path.open('rb') as f:
        f.seek(offset)
        result=f.read(length)
    if len(result)!=length:raise ValueError('requested bytes extend beyond EOF')
    return result


def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    subs=p.add_subparsers(dest='cmd',required=True)
    sizes=subs.add_parser('sizes',help='metadata only')
    sizes.add_argument('paths',nargs='+',type=Path)
    lines=subs.add_parser('lines',help='up to 120 lines and 64 KiB')
    lines.add_argument('path',type=Path);lines.add_argument('start',type=int);lines.add_argument('count',type=int)
    blob=subs.add_parser('bytes',help='up to 4096 bytes in hex')
    blob.add_argument('path',type=Path);blob.add_argument('offset',type=lambda x:int(x,0));blob.add_argument('length',type=lambda x:int(x,0))
    args=p.parse_args()
    try:
        if args.cmd=='sizes':
            if len(args.paths)>100:raise ValueError('at most 100 paths per request')
            for name in args.paths:print(f'{name.stat().st_size:>12} {name}')
        elif args.cmd=='lines':sys.stdout.write(inspect_lines(args.path,args.start,args.count))
        elif args.cmd=='bytes':print(inspect_bytes(args.path,args.offset,args.length).hex(' '))
    except (OSError,ValueError) as exc:
        print(f'bounded-inspect: {exc}',file=sys.stderr)
        return 2
    return 0

if __name__=='__main__':raise SystemExit(main())
