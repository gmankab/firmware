#!/usr/bin/env python3
"""Independent cross-bank DLM descriptor pointer-geometry/reference alias check.

40-bit or NDS32 instruction bytes are not emitted. Repeated pointer values and
profile signatures are constrained by measured structure, not independently
recovered program/data source. Both spans remain UNKNOWN in the ledger.
"""
import argparse
import hashlib
import json
from pathlib import Path
import struct

BASE=0x22000
DELTA=0x4000
RECORD_SIZE=0x8c
POINTER_OFFSET=0x48
INDEX_MASK=0x7fff
PAIRS=((0x22a64,17),(0x25230,4))
SIGNATURE_PRIMARY=0x22710
SIGNATURE_COPY=0x2df8c
SIGNATURE_SIZE=12
PINNED_SHA256='8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5'


def geometry_index(start, count, bank):
    if (start,count) not in PAIRS or bank not in (0,1):
        raise ValueError('unrecognized DLM descriptor geometry')
    array_end=start+bank*DELTA+count*RECORD_SIZE
    if array_end<BASE or (array_end-BASE)%4:
        raise ValueError('misaligned descriptor array end')
    index=(array_end-BASE)//4
    if index>INDEX_MASK:
        raise ValueError('descriptor target exceeds 15-bit index')
    return index


def verify_geometry(image):
    checked=0
    for first,count in PAIRS:
        for bank in (0,1):
            expected=geometry_index(first,count,bank)
            for number in range(count):
                at=first+bank*DELTA+number*RECORD_SIZE+POINTER_OFFSET
                if at+2>len(image):
                    raise ValueError(f'truncated descriptor at file 0x{at:x}')
                index=struct.unpack_from('<H',image,at)[0]
                if index&INDEX_MASK!=expected:
                    raise ValueError(f'DLM descriptor geometry mismatch at file 0x{at:x}')
                checked+=1
    if checked!=42:raise AssertionError('unexpected record count')
    return checked


def verify_signature(image):
    if SIGNATURE_COPY+SIGNATURE_SIZE>len(image):
        raise ValueError('truncated dependent profile signature')
    source=image[SIGNATURE_PRIMARY:SIGNATURE_PRIMARY+SIGNATURE_SIZE]
    copy=image[SIGNATURE_COPY:SIGNATURE_COPY+SIGNATURE_SIZE]
    for i,(x,y) in enumerate(zip(source,copy)):
        if x!=y:raise ValueError(f'DLM repeated profile signature differs at file 0x{SIGNATURE_COPY+i:x}')
    return SIGNATURE_SIZE


def _ensure_unknown(rows,begin,end):
    at=begin
    for r in rows:
        if r['file_end']<=at or r['file_start']>=end:continue
        if r['file_start']>at or r['kind']!='unknown_dlm':
            raise ValueError(f'DLM descriptor/signature at 0x{at:x} overlaps credited source')
        at=max(at,min(end,r['file_end']))
    if at!=end:raise ValueError(f'DLM descriptor/signature at 0x{at:x} has incomplete ledger coverage')


def verify_no_source_credit(ledger):
    rows=ledger['regions']
    for first,count in PAIRS:
        for bank in (0,1):
            for number in range(count):
                at=first+bank*DELTA+number*RECORD_SIZE+POINTER_OFFSET
                _ensure_unknown(rows,at,at+2)
    for start in (SIGNATURE_PRIMARY,SIGNATURE_COPY):
        _ensure_unknown(rows,start,start+SIGNATURE_SIZE)
    return True


def verify(image, ledger, check_hash=True):
    if check_hash and hashlib.sha256(image).hexdigest()!=PINNED_SHA256:
        raise ValueError('SHA-256 differs from pinned postmarketOS Novatek firmware')
    count=verify_geometry(image)
    sig=verify_signature(image)
    verify_no_source_credit(ledger)
    if ledger.get('source_dlm_data_bytes')!=9353 or ledger.get('unknown_dlm_bytes')!=40199:
        raise ValueError('unexpected DLM independent source credit')
    return {
        'schema':'agent20-19-descriptor-geometry-v1',
        'reference_verified':bool(check_hash),
        'source_dlm_data_bytes':ledger['source_dlm_data_bytes'],
        'unknown_dlm_bytes':ledger['unknown_dlm_bytes'],
        'geometric_pointer_records_checked':count,
        'dependent_profile_signature_bytes':sig,
        'additional_independently_source_authored_bytes':0,
        'pointer_field_semantics_verified':False,
        'selector_flag_source_unknown':True,
        'profile_signature_source_unknown':True,
        'directory_group_data':[{'file_start':x,'count':n,'lower_word_index':geometry_index(x,n,0),'upper_word_index':geometry_index(x,n,1)} for x,n in PAIRS]
    }


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('reference',type=Path)
    parser.add_argument('--ledger',type=Path,required=True)
    p=parser.parse_args()
    report=verify(p.reference.read_bytes(),json.loads(p.ledger.read_text()))
    print(json.dumps(report,indent=2))

if __name__=='__main__':main()
