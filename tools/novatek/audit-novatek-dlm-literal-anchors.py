#!/usr/bin/env python3
"""Classify absolute DLM address constants from independently typed NDS32 sources.

Only immediate MOVI operands are extracted. An absolute pointer is not a
memory access and does not prove initial data values. Never credit source
bytes based on this map. The input ILM/EX9 source is read only and belongs to
Agent 18; neither that source nor the private OEM reference is embedded.
"""
from __future__ import annotations
import argparse
import json
import re
from pathlib import Path

DLM_SRAM_START=0x20000
DLM_SRAM_END=0x2c190
FILE_BIAS=0x2000
ITB_FILE_START=0x17dc0
PINNED_COUNTS={'upper_ilm':28,'upper_ex9':6}

MOVI=re.compile(r'\bMovi\((R\d+),\s*(0x[0-9a-f]+|\d+)\)')
UPPER_LINE=re.compile(r'^\s*\((0x[0-9a-f]+),\s*Movi\((R\d+),\s*(0x[0-9a-f]+|\d+)\)\),')
EX9_LINE=re.compile(r'^\s*Movi\((R\d+),\s*(0x[0-9a-f]+|\d+)\),\s*// slot (\d+), file (0x[0-9a-f]+):')


def parse_upper_file(source):
    matches=[]
    seen_pc=set()
    for line in source.splitlines():
        m=UPPER_LINE.match(line)
        if not m:continue
        pc=int(m[1],16);register=m[2];value=int(m[3],0)
        if pc in seen_pc:raise ValueError(f'duplicate typed upper-ILM PC 0x{pc:x}')
        seen_pc.add(pc)
        if DLM_SRAM_START<=value<DLM_SRAM_END:
            matches.append(dict(source='upper_ilm',file_pc=pc,slot=None,register=register,sram_target=value,dlm_file_target=value+FILE_BIAS))
    if not seen_pc:raise ValueError('no upper-ILM MOVI instructions in supplied source')
    return matches


def parse_ex9_file(source):
    entries=[]
    seen=set()
    for line in source.splitlines():
        m=EX9_LINE.match(line)
        if not m:continue
        reg=m[1];value=int(m[2],0);slot=int(m[3]);pc=int(m[4],16)
        if pc!=ITB_FILE_START+4*slot or slot>=512:
            raise ValueError(f'EX9 MOVI slot {slot} has invalid source file address 0x{pc:x}')
        if slot in seen:raise ValueError(f'duplicate EX9 MOVI slot {slot}')
        seen.add(slot)
        if DLM_SRAM_START<=value<DLM_SRAM_END:
            entries.append(dict(source='upper_ex9',file_pc=pc,slot=slot,register=reg,sram_target=value,dlm_file_target=value+FILE_BIAS))
    if not seen:raise ValueError('no typed EX9 MOVI entries in supplied source')
    return entries


def ledger_kind(ledger,address):
    for row in ledger['regions']:
        if row['file_start']<=address<row['file_end']:
            if row['kind'] not in ('source_dlm_data','unknown_dlm'):
                raise ValueError('absolute DLM anchor has invalid provenance kind')
            return row['kind']
    raise ValueError(f'DLM anchor target 0x{address:x} not mapped by ledger')


def audit(upper_text,ex9_text,ledger,strict_snapshot=False):
    found=parse_upper_file(upper_text)+parse_ex9_file(ex9_text)
    counters={name:sum(x['source']==name for x in found) for name in PINNED_COUNTS}
    if strict_snapshot and counters!=PINNED_COUNTS:
        raise ValueError(f'Agent18 checkpoint02 absolute DLM MOVI snapshot changed: {counters}')
    for x in found:x['target_provenance']=ledger_kind(ledger,x['dlm_file_target'])
    found.sort(key=lambda x:(x['source'],x['file_pc']))
    unresolved=[x for x in found if x['target_provenance']=='unknown_dlm']
    if ledger.get('source_dlm_data_bytes')!=9353 or ledger.get('unknown_dlm_bytes')!=40199:
        raise ValueError('Agent19 DLM source provenance unexpectedly changed')
    return {'schema':'agent20-19-dlm-absolute-literals-v1',
            'absolute_dlm_literals':len(found),
            'upper_ilm_literals':counters['upper_ilm'],
            'ex9_literals':counters['upper_ex9'],
            'unresolved_target_literals':len(unresolved),
            'distinct_unknown_targets':len({x['dlm_file_target'] for x in unresolved}),
            'source_owned_target_literals':len(found)-len(unresolved),
            'new_source_data_bytes':0,
            'new_executable_bytes':0,
            'independently_recovered_semantics':False,
            'explanation':'MOVI supplies an address constant; it does not prove a dereference, write, or initialized value',
            'cross_references':found}


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--upper-ilm-rs',type=Path,required=True)
    p.add_argument('--upper-ex9-rs',type=Path,required=True)
    p.add_argument('--ledger',type=Path,required=True)
    p.add_argument('--verify-checkpoint02-snapshot',action='store_true')
    p.add_argument('--output',type=Path)
    a=p.parse_args()
    result=audit(a.upper_ilm_rs.read_text(),a.upper_ex9_rs.read_text(),json.loads(a.ledger.read_text()),a.verify_checkpoint02_snapshot)
    output=json.dumps(result,indent=2)+'\n'
    if a.output:a.output.write_text(output)
    else:print(output,end='')

if __name__=='__main__':main()
