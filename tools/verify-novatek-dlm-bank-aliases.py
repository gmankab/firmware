#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent Python verification of four external-input DLM bank aliases.

This does not reconstruct the unknown earlier-bank payload. Even when the
upper bank can be derived by copying lower-bank bytes, both are still
reference-dependent and source coverage remains unchanged.
"""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path

REFERENCE_SHA256 = '8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5'
IMAGE_BYTES = 196608
DLM_START, DLM_END = 0x22000, 0x2e190
BANK_DELTA = 0x4000
ALIASES = (
    ('codec-bank-staging', 0x2387c, 406),
    ('format-index-scratch', 0x23df8, 228),
    ('frame-bank-defaults', 0x24186, 182),
    ('input-ring-metadata', 0x2574c, 616),
)


def verify_aliases(image: bytes, ledger: dict | None = None, check_hash: bool = True) -> dict:
    if check_hash and (len(image) != IMAGE_BYTES or hashlib.sha256(image).hexdigest() != REFERENCE_SHA256):
        raise ValueError('Novatek reference is not pinned postmarketOS revision')
    if ledger is not None:
        if ledger.get('source_dlm_data_bytes') != 9353 or ledger.get('unknown_dlm_bytes') != 40199:
            raise ValueError('unexpected DLM independent-source ledger')
        regions = ledger['regions']
    else:
        regions = []
    prev_end = DLM_START
    results = []
    for name, earlier, length in ALIASES:
        later = earlier+BANK_DELTA
        if earlier < prev_end or length <= 0 or later+length > DLM_END:
            raise ValueError('invalid overlapping or out-of-range DLM alias')
        prev_end = earlier+length
        if len(image)<later+length:
            raise ValueError(f'{name} reference-dependent input is incomplete')
        primary=image[earlier:earlier+length]
        duplicate=bytes(primary)  # zero-byte-credit reference-assisted copy
        actual=image[later:later+length]
        for index,(x,y) in enumerate(zip(duplicate,actual)):
            if x!=y:
                raise ValueError(f'{name} bank alias mismatch at file 0x{later+index:x}')
        for start,end in ((earlier,earlier+length),(later,later+length)):
            if not regions:
                continue
            at=start
            for row in regions:
                if row['file_end']<=at or row['file_start']>=end:
                    continue
                if row['file_start']>at or row['kind']!='unknown_dlm':
                    raise ValueError(f'{name} bank alias overlaps source-owned DLM')
                at=min(end,row['file_end'])
            if at!=end:
                raise ValueError(f'{name} bank alias lacks unknown DLM provenance')
        results.append({'label':name,'primary_file_start':earlier,'primary_file_end':earlier+length,
                        'dependent_file_start':later,'dependent_file_end':later+length,'bytes':length})
    return {'schema':'agent20-19-bank-alias-v1',
            'reference_verified':bool(check_hash),
            'source_inputs_required':len(ALIASES),
            'reference_dependent_matched_bytes':sum(r['bytes'] for r in results),
            'new_source_dlm_bytes':0,
            'source_dlm_data_bytes':ledger['source_dlm_data_bytes'] if ledger else None,
            'unknown_dlm_bytes':ledger['unknown_dlm_bytes'] if ledger else None,
            'aliases':results}


def main() -> None:
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('reference',type=Path)
    parser.add_argument('--ledger',type=Path,help='independent source ledger from novatek-dlm ledger')
    args=parser.parse_args()
    image=args.reference.read_bytes()
    led=json.loads(args.ledger.read_text()) if args.ledger else None
    print(json.dumps(verify_aliases(image,led),indent=2,sort_keys=True))

if __name__ == '__main__':main()
