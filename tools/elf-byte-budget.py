#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Metadata-only byte inventory for ELF32 firmware.

Does not export instruction bytes, certificate bytes, or decrypted payloads.
Span 'executable' means PF_X only: it is NOT a verified code-only range.
"""
import argparse
import hashlib
import json
from pathlib import Path
import struct


def inventory(path):
    b = path.read_bytes()
    if len(b) < 4096 or b[:7] != b'\x7fELF\x01\x01\x01':
        raise ValueError('not supported ELF32LE')
    phoff, = struct.unpack_from('<I',b,28)
    phsize, nph = struct.unpack_from('<HH',b,42)
    if phsize != 32 or phoff + phsize*nph > 4096:
        raise ValueError('unrecognized PHDR layout')
    segments=[]
    for i in range(nph):
        typ, off, vaddr, paddr, fs, ms, flags, align = struct.unpack_from('<IIIIIIII',b,phoff+i*phsize)
        if not fs:
            continue  # BSS mapping can have an arbitrary p_offset at or beyond EOF
        if off + fs > len(b):
            raise ValueError(f'out-of-bounds file-backed segment {i}')
        if flags & 0x07000000 == 0x02000000:
            kind='auth_hash_segment'
        elif typ == 1:
            kind='executable_load' if flags & 1 else 'nonexecutable_load'
        else:
            kind='other_segment'
        segments.append({'index':i,'category':kind,'offset':off,'length':fs,
                         'flags':f'0x{flags:08x}','type':typ})
    # Priority matters because NOTE and DYNAMIC may be nested inside LOAD.
    cats=('header', 'auth_hash_segment', 'executable_load','nonexecutable_load','other_segment','unmapped')
    rank={c:i for i,c in enumerate(cats)}
    events=[(0,4096,'header')]
    events += [(s['offset'],s['offset']+s['length'],s['category']) for s in segments]
    marks=sorted(set([0,len(b)]+[x for a,z,_ in events for x in (a,z)]))
    counts={c:0 for c in cats}
    unmapped_nonzero=0
    unmapped_ff=0
    for a,z in zip(marks,marks[1:]):
        k=min([c for l,r,c in events if l <= a and r >= z],key=lambda c:rank[c],default='unmapped')
        counts[k]+=z-a
        if k=='unmapped':
            unmapped_nonzero+=sum(x!=0 for x in b[a:z])
            unmapped_ff+=sum(x==255 for x in b[a:z])
    assert sum(counts.values())==len(b)
    return {'file':path.name,'sha256':hashlib.sha256(b).hexdigest(),'length':len(b),
            'first_page_accounted_bytes':4096,'classes':counts,
            'unmapped_nonzero_count':unmapped_nonzero,'unmapped_ff_count':unmapped_ff,
            'segments':segments}


def main():
    p=argparse.ArgumentParser()
    p.add_argument('firmware_path',type=Path, help="ELF32LE file or directory (recursively scanned)")
    p.add_argument('output',type=Path)
    a=p.parse_args()
    if a.firmware_path.is_file():
        paths = [a.firmware_path]
    else:
        paths = sorted(p for p in a.firmware_path.rglob('*') if p.is_file()
                       and p.suffix.lower() in ('.mbn', '.elf', '.mdt'))
    paths = [p for p in paths if p.open('rb').read(6) == b'\x7fELF\x01\x01']
    if not paths:
        p.error('no little-endian ELF32 inputs found')
    records=[inventory(path) for path in paths]
    a.output.parent.mkdir(parents=True, exist_ok=True)
    a.output.write_text(json.dumps({'semantics':'PF_X describes executable segments, not verified instruction bytes; class counts are disjoint and exclusive; auth bytes are NOT rebuilt','records':records},indent=2)+'\n')
    for i in records:
        print(f"{i['file']:16s} total={i['length']:>9} first_page={i['first_page_accounted_bytes']:>6} "
              f"auth={i['classes']['auth_hash_segment']:>7} exec-marked={i['classes']['executable_load']:>9} "
              f"unmapped={i['classes']['unmapped']:>7}")

if __name__=='__main__':
    main()
