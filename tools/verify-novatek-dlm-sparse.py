#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently validate conditional DLM reassembly with externally supplied data.

Not a source recovery tool: 33,723 unknown bytes remain externally provided.
The goal is deterministic, checked linker behavior and accurate provenance.
Never ship reference-derived seeds or the complete result in a Git bundle.
"""
from __future__ import annotations
import argparse, hashlib, json
from pathlib import Path

DLM_START,DLM_END=0x22000,0x2e190
DLM_SIZE=DLM_END-DLM_START
PINNED='8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5'
# Source/DLM dependency constants are independently represented here, not
# imported from Rust structs. The *source* values are supplied to the CLI;
# the targets are derived from the rules below and never credited as source.
PAIRS=((0x22a64,0x26a64,17),(0x25230,0x29230,4))
DESCRIPTOR_BYTES=0x8c
ALIASES=((0x2387c,406),(0x23df8,228),(0x24186,182),
         (0x245a6,862),(0x24fc4,604),(0x2574c,616),(0x25c5e,238))
PROFILE=(0x22224,0x223d0,0x2d648)
SIGNATURE=(0x22710,0x2df8c,12)


def span(start,length):
    if not(DLM_START<=start<=DLM_END and 0<=length<=DLM_END-start):
        raise ValueError(f'DLM out-of-bounds span at {start:#x}, size {length}')
    return range(start-DLM_START,start-DLM_START+length)


def destinations():
    for src,dest,count in PAIRS:
        yield dest,count*DESCRIPTOR_BYTES
    for start,n in ALIASES:
        yield start+0x4000,n
    yield PROFILE[2],PROFILE[1]-PROFILE[0]
    yield SIGNATURE[1],SIGNATURE[2]


def plan(fragments):
    ownership=bytearray(DLM_SIZE)
    for start,data in fragments:
        if not data:raise ValueError('empty authored part')
        for i in span(start,len(data)):
            if ownership[i]:raise ValueError(f'overlapping authored byte {DLM_START+i:#x}')
            ownership[i]=1
    checks=0
    for start,length in destinations():
        for i in span(start,length):
            if ownership[i]==1:checks+=1
            elif ownership[i]==0:ownership[i]=2
            else:raise ValueError(f'overlapping dependent region {DLM_START+i:#x}')
    holes=[];k=0
    while k<DLM_SIZE:
        if ownership[k]:k+=1;continue
        start=k
        while k<DLM_SIZE and ownership[k]==0:
            ownership[k]=3;k+=1
        holes.append((DLM_START+start,DLM_START+k))
    kinds={n:ownership.count(n) for n in [1,2,3]}
    if sum(kinds.values())!=DLM_SIZE:raise ValueError('unassigned DLM byte')
    return holes,kinds,checks


def rebuild(fragments,seeds):
    holes,kinds,checks=plan(fragments)
    if len(seeds)!=len(holes):raise ValueError('missing/extra external seeds')
    data=bytearray(DLM_SIZE);owned=bytearray(DLM_SIZE)
    def place(start,blob,kind):
        for i,b in zip(span(start,len(blob)),blob):
            if owned[i]==0:
                data[i]=b;owned[i]=kind
            elif kind==2 and owned[i]==1 and data[i]==b:
                # The independently authored target is revalidated without
                # inflating the dependence or authored totals.
                pass
            else:
                raise ValueError(f'overlapping or disagreeing DLM values at {DLM_START+i:#x}')
    for start,blob in fragments:place(start,blob,1)
    for (a,b),(s,blob) in zip(holes,seeds):
        if s!=a or len(blob)!=b-a:raise ValueError(f'wrong external seed for {a:#x}')
        place(s,blob,3)
    def read(a,b):
        indices=span(a,b-a)
        if not all(owned[i]!=0 for i in indices):raise ValueError(f'missing primary DLM at {a:#x}')
        return bytes(data[indices.start:indices.stop])
    for start,target,n in PAIRS:
        buf=bytearray(read(start,start+n*DESCRIPTOR_BYTES))
        for record in range(n):
            for idx in range(9):
                i=record*DESCRIPTOR_BYTES+0x48+2*idx
                value=int.from_bytes(buf[i:i+2],'little')
                high=value&0x8000
                low=value&0x7fff
                if low+0x1000>0x7fff:raise ValueError('relocation overflows 15-bit word index')
                buf[i:i+2]=(high|(low+0x1000)).to_bytes(2,'little')
        place(target,buf,2)
    for start,n in ALIASES:place(start+0x4000,read(start,start+n),2)
    first,last,target=PROFILE
    place(target,read(first,last),2)
    first,target,n=SIGNATURE
    place(target,read(first,first+n),2)
    if 0 in owned:raise ValueError('incomplete DLM')
    return bytes(data),holes,kinds,checks


def read_fragments(source_dir:Path):
    manifest=json.loads((source_dir/'manifest.json').read_text())
    if manifest.get('reference_verified') is not False:
        raise ValueError('expected emitter output with unverified input provenance')
    if manifest.get('source_dlm_data_bytes')!=9353:
        raise ValueError('expected 9353 source bytes in checkpoint14')
    result=[]
    for f in manifest['fragments']:
        expected_name=f"novatek-dlm-{f['file_start']:06x}-{f['file_end']:06x}.bin"
        if f['file']!=expected_name:
            raise ValueError('emitter fragment path mismatch or unsafe path')
        path=source_dir/expected_name
        b=path.read_bytes()
        if len(b)!=f['bytes'] or hashlib.sha256(b).hexdigest()!=f['sha256']:
            raise ValueError(f'fragment length or digest mismatch: {path}')
        if len(b)!=f['file_end']-f['file_start']:
            raise ValueError('fragment address geometry mismatch')
        result.append((f['file_start'],b))
    return result


def main():
    a=argparse.ArgumentParser(description=__doc__)
    a.add_argument('--reference',required=True,type=Path)
    a.add_argument('--emitted-dir',required=True,type=Path)
    a.add_argument('--sparse-plan',required=True,type=Path)
    a.add_argument('--output-report',type=Path)
    args=a.parse_args()
    original=args.reference.read_bytes()
    if len(original)!=196608 or hashlib.sha256(original).hexdigest()!=PINNED:
        raise ValueError('input is not SHA-256 pinned postmarketOS Novatek reference')
    fragments=read_fragments(args.emitted_dir)
    ranges,counts,checks=plan(fragments)
    submitted=json.loads(args.sparse_plan.read_text())
    if submitted.get('schema')!='agent20-19-dlm-sparse-v1':
        raise ValueError('incorrect Rust sparse plan schema')
    for k,v in [('source_dlm_bytes',counts[1]),('dependent_unique_bytes',counts[2]),
                ('required_external_seed_bytes',counts[3]),('independent_dependent_crosschecks',checks)]:
        if submitted[k]!=v:raise ValueError(f'wrong sparse plan totals: {k}')
    if submitted['seed_ranges']!=[{'file_start':a,'file_end':b,'bytes':b-a} for a,b in ranges]:
        raise ValueError('Rust/Python sparse plan boundary disagreement')
    seeds=[(a,original[a:b]) for a,b in ranges]
    rebuilt,_,_,_=rebuild(fragments,seeds)
    target=original[DLM_START:DLM_END]
    if rebuilt!=target:
        mismatch=next(i for i,(x,y) in enumerate(zip(rebuilt,target)) if x!=y)
        raise ValueError(f'DLM mismatch at file {DLM_START+mismatch:#x}')
    # Mutate representative caller-provided input and assert reconstruction
    # cannot erroneously claim the pinned original still matched.
    tampered=list(seeds)
    first,blob=tampered[0];changed=bytearray(blob);changed[0]^=0x40
    tampered[0]=(first,bytes(changed))
    altered,_,_,_=rebuild(fragments,tampered)
    if altered==target:raise ValueError('rebuild did not incorporate external seed mutation')
    report={'schema':'agent20-19-sparse-independent-v1','image_sha256':PINNED,
            'source_dlm_bytes':counts[1],'unique_derived_bytes':counts[2],
            'external_seed_bytes':counts[3],'overlap_crosschecks':checks,
            'external_seed_ranges':len(ranges),'dlm_size':DLM_SIZE,
            'mismatched_bytes':0,'new_source_credit':0,'flashable':False}
    dumped=json.dumps(report,indent=2)+'\n'
    if args.output_report:args.output_report.write_text(dumped)
    print(dumped,end='')

if __name__=='__main__':main()
