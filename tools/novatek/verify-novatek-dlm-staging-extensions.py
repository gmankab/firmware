#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent cross-revision verifier for two *dependent* staging tails.

Matching source and destination does NOT establish source authorship. Values
stay externally supplied in all sparse rebuilds; this script has no fallback
that fills unknown bytes with original firmware payload.
"""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path

SPANS=((0x23860,24),(0x23a13,98))
DELTA=0x4000
POSTMARKETOS='8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5'
MAP220V='72364eca0a91855d59e043371aca774354e5ad0885df5024edcec8f976543ad2'


def checked_dependency(spans=SPANS):
    last=0
    for start,count in spans:
        if not(0x22000<=start and count>0 and start>=last and start+DELTA+count<=0x2e190):
            raise ValueError('invalid staging dependency geometry')
        last=start+count
    if sum(n for _,n in spans)!=122:
        raise ValueError('incorrect dependent staging byte total')
    return spans


def dependent_copy(span,source):
    if span not in SPANS or len(source)!=span[1]:
        raise ValueError('unknown staging source or wrong input length')
    return bytes(source)


def compare_image(image, expected_sha256):
    if len(image)!=196608 or hashlib.sha256(image).hexdigest()!=expected_sha256:
        raise ValueError('incorrect firmware revision or length')
    checked_dependency()
    for i,span in enumerate(SPANS):
        start,n=span
        if dependent_copy(span,image[start:start+n])!=image[start+DELTA:start+DELTA+n]:
            raise ValueError(f'staging dependency mismatch at range #{i}')
    return sum(n for _,n in SPANS)


def main(argv=None):
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--postmarketos',type=Path,required=True)
    p.add_argument('--map220v',type=Path,required=True)
    args=p.parse_args(argv)
    results={'schema':'agent20-19-staging-extensions-v1','dependent_ranges':len(SPANS),
             'dependent_bytes':122,'independently_authored_new_bytes':0,
             'postmarketos':compare_image(args.postmarketos.read_bytes(),POSTMARKETOS),
             'map220v':compare_image(args.map220v.read_bytes(),MAP220V)}
    print(json.dumps(results,sort_keys=True))
    return 0

if __name__=='__main__':
    try:raise SystemExit(main())
    except ValueError as error:raise SystemExit(f'DLM staging verification failed: {error}')
