#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Stand-alone, source-only WLAN Hexagon v66 continuation beyond checkpoint 25.

The *required* prior Agent 13 checkpoint-25 Git bundle is NOT present in this
session. This tool therefore intentionally emits only the new disjoint range
starting at file 0x4568c; it never invents the missing intermediate bytes,
and it never claims a contiguous prefix from 0x40000 without that base.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE_DIR = ROOT / 'sources/xiaomi-nabu/wlanmdsp'
BASE_BUILDER = ROOT / 'tools/build-wlanmdsp-startup.py'
FILE_BASE = 0x40000
VADDR = 0xb0000000
START = 0x568c
FRAGMENTS = (
    ('controller-init', SOURCE_DIR / 'agent13-runtime-controller-initialize.S', 0x568c, 0x394),
)


def load_parent():
    spec = importlib.util.spec_from_file_location('wlan_hexagon_parent', BASE_BUILDER)
    if spec is None or spec.loader is None:
        raise ValueError('Agent 7 assembler/parser is not available')
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def validate_manifest(fragments=FRAGMENTS):
    previous = START
    for name, path, off, size in fragments:
        if off != previous:
            raise ValueError(f'non-adjacent fragment {name}: expected {previous:#x}, got {off:#x}')
        if size <= 0 or size % 4:
            raise ValueError(f'unaligned or empty fragment {name}')
        if not path.is_file():
            raise ValueError(f'missing independent Hexagon assembly: {path}')
        source = path.read_text()
        if any(raw in source for raw in ('.incbin', '.byte', '.word', '.quad', '.long', '.fill')):
            raise ValueError(f'{name}: copying opaque original firmware bytes is prohibited')
        previous += size
    if previous + FILE_BASE > 0x100000:
        raise ValueError('Agent 13 attempted to cross the exclusive file 0x100000 boundary')
    return previous


def build(llvm6: str):
    mod = load_parent()
    end = validate_manifest()
    content = b''.join(mod.assemble(source, llvm6, 'hexagonv65', expected_length=size)
                       for _, source, _, size in FRAGMENTS)
    if len(content) != end - START:
        raise ValueError('unexpected emitted fragment length')
    return content, end


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--llvm6',required=True,help='Path to LLVM6 Hexagon llvm-mc executable')
    p.add_argument('--reference',type=Path,help='Original pinned wlanmdsp.mbn, for optional compare only')
    p.add_argument('--output',required=True,type=Path)
    p.add_argument('--manifest',type=Path)
    args=p.parse_args()
    fragment,end=build(args.llvm6)
    if args.reference:
        parent=load_parent()
        parent.check_reference(fragment,args.reference.read_bytes(),runtime_offset=START)
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_bytes(fragment)
    report={'agent':13,'checkpoint':'26-disjoint',
            'source_base':'Agent 13 checkpoint 16 (7c730bf); checkpoint25 source unavailable',
            'is_checkpoint25_descendant':False,
            'file_start':FILE_BASE+START,'file_end':FILE_BASE+end,
            'vaddr_start':VADDR+START,'vaddr_end':VADDR+end,
            'independent_source_assembly_bytes':len(fragment),
            'external_original_code_bytes':0,'layout_fill_bytes':0,
            'sha256':hashlib.sha256(fragment).hexdigest(),
            'reference_checked':args.reference is not None,
            'complete_wlan_firmware':False,
            'fragments':[{'name':name,'file_start':FILE_BASE+off,'file_end':FILE_BASE+off+size} for name,_,off,size in FRAGMENTS]}
    if args.manifest:
        args.manifest.parent.mkdir(exist_ok=True,parents=True)
        args.manifest.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2))

if __name__=='__main__':main()
