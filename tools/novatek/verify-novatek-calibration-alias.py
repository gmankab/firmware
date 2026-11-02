#!/usr/bin/env python3
"""Independent, read-only DLM calibration-duplicate observation.

The alias consumes an externally supplied *primary* 428-byte region. Its
exact equality with the later DLM region is not independent source authorship.
The pinned Novatek firmware is used only for verification, never embedded.
"""
import argparse
import hashlib
import json
import struct
from pathlib import Path

START=0x22224
END=0x223d0
MIRROR_START=0x2d648
MIRROR_END=0x2d7f4
LENGTH=428
REFERENCE_SHA256='8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5'


def derive_reference_dependent_alias(primary: bytes) -> bytes:
    if len(primary)!=LENGTH:
        raise ValueError('primary profile must contain exactly 428 externally supplied bytes')
    return bytes(primary)


def inspect_observed_profiles(primary: bytes):
    derive_reference_dependent_alias(primary)
    out=[]
    for offset in (4,0xd6):
        span,rows,parameter,enabled=struct.unpack_from('<4H',primary,offset)
        out.append({'relative_offset':offset,'mapped_span':span,'rows':rows,
                    'controller_parameter':parameter,'enabled':enabled})
    return out


def verify_pair(image:bytes,check_hash=False):
    if check_hash and hashlib.sha256(image).hexdigest()!=REFERENCE_SHA256:
        raise ValueError('wrong reference image revision or tampered SHA256')
    if len(image)<MIRROR_END:
        raise ValueError('incomplete DLM profile span')
    primary=image[START:END]
    derived=derive_reference_dependent_alias(primary)
    actual=image[MIRROR_START:MIRROR_END]
    for index,(x,y) in enumerate(zip(derived,actual)):
        if x!=y:raise ValueError(f'repeated DLM calibration differs at file 0x{MIRROR_START+index:x}')
    return {'schema':'agent20-19-dlm-calibration-alias-v1',
            'primary_file_start':START,'primary_file_end':END,
            'mirror_file_start':MIRROR_START,'mirror_file_end':MIRROR_END,
            'matched_reference_dependent_bytes':LENGTH,
            'additional_independently_authored_source_bytes':0,
            'profile_headers_observed_only':inspect_observed_profiles(primary),
            'hash_verified':bool(check_hash)}


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('reference',type=Path,help='Externally supplied pinned Novatek original')
    args=parser.parse_args()
    print(json.dumps(verify_pair(args.reference.read_bytes(),check_hash=True),indent=2))


if __name__=='__main__':main()
