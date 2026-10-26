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
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE_DIR = ROOT / 'sources/xiaomi-nabu/wlanmdsp'
BASE_BUILDER = ROOT / 'tools/build-wlanmdsp-startup.py'
FILE_BASE = 0x40000
VADDR = 0xb0000000
START = 0x568c
FRAGMENTS = (
    ('controller-init', SOURCE_DIR / 'agent13-runtime-controller-initialize.S', 0x568c, 0x394),
    ('controller-table-create', SOURCE_DIR / 'agent13-runtime-controller-table-create.S', 0x5a20, 0xa8),
    ('controller-table-search', SOURCE_DIR / 'agent13-runtime-controller-table-search.S', 0x5ac8, 0x90),
    ('bitmap-scan-prefix', SOURCE_DIR / 'agent13-runtime-bitmap-scan-prefix.S', 0x5b58, 0x20),
    ('bitmap-predicate-pair', SOURCE_DIR / 'agent13-runtime-bitmap-scan-predicate-packet.S', 0x5b78, 0x08),
    ('bitmap-duplex', SOURCE_DIR / 'agent13-runtime-bitmap-scan-duplex.S', 0x5b80, 0x04),
    ('bitmap-scan-tail', SOURCE_DIR / 'agent13-runtime-bitmap-scan-tail.S', 0x5b84, 0x88),
    ('controller-snapshot-wrapper', SOURCE_DIR / 'agent13-runtime-controller-snapshot-wrapper.S', 0x5c0c, 0x14),
    ('controller-initialize-stage1', SOURCE_DIR / 'agent13-runtime-controller-init-stage1.S', 0x5c20, 0xd8),
    ('compound-branch', SOURCE_DIR / 'agent13-runtime-controller-init-branch-packet.S', 0x5cf8, 0x08),
    ('compound-memory', SOURCE_DIR / 'agent13-runtime-controller-init-memory-packet.S', 0x5d00, 0x08),
    ('controller-initialize-stage2', SOURCE_DIR / 'agent13-runtime-controller-init-stage2.S', 0x5d08, 0xe8),
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


def assemble_predicate_pair(parent, llvm6: str, source: Path) -> bytes:
    """Build two continuing words of a 3-word packet from semantic source.

    LLVM6 does not reliably pair the specific original duplex when predicate
    instructions share the packet. Pairing-disabled assembly preserves the
    exact original two predicate encodings/parse flags; the *final* slot is
    independently compiled from two register operations by LLVM6 normally.
    The sixteen-byte temporary assembler output is not a reference payload,
    and only its two source-generated predicate encodings are retained.
    """
    asm = source.read_text(encoding='utf-8')
    if 'p0 = cmp.eq(r2,#0)' not in asm or 'p1 = cmp.eq(r2,#1)' not in asm:
        raise ValueError('unexpected predicate register semantics')
    with tempfile.TemporaryDirectory(prefix='wlan-typed-predicates-') as tmp:
        obj = Path(tmp)/'predicates.o'
        proc = subprocess.run([llvm6,'-triple=hexagon','-mcpu=hexagonv65',
                               '-mno-pairing','-filetype=obj','-o',str(obj),str(source)],
                              text=True,capture_output=True,timeout=45)
        if proc.returncode != 0:
            raise ValueError('Hexagon predicates failed source assembly: '+proc.stderr[:500])
        encoded=parent.read_resolved_text(obj.read_bytes(),expected_length=16)
    # Two predicate instructions remain separate; the last compiler-generated
    # operations are replaced by separately compiled original duplex source.
    return encoded[:8]


def assemble_compound_branch(parent, llvm6: str, source: Path) -> bytes:
    """Source-build extended predicated jump and change ONLY packet parse bits.

    LLVM6 cannot schedule original one-packet compound predicate jump plus a
    new-value memory load/store (out-of-slots error). Compile both pairs from
    semantic source; the branch is the second of four packet words. Replace its
    Hexagon packet parse field bits 15:14 from end-of-packet (`0b11`) to
    continue (`0b01`), retaining all independently assembled opcode bits.
    The second source-built pair supplies the last word with the final marker.
    """
    text = source.read_text()
    if 'p0 = cmp.eq(r2,#0); if (p0.new) jump:nt ##0x434' not in text:
        raise ValueError('compound branch semantic source differs')
    words=parent.assemble(source,llvm6,'hexagonv65',expected_length=8)
    first=int.from_bytes(words[0:4],'little')
    last=int.from_bytes(words[4:8],'little')
    if (first>>14)&0b11 != 0b01 or (last>>14)&0b11 != 0b11:
        raise ValueError('LLVM6 extended branch parse markers unexpectedly changed')
    last=(last & ~(0b11<<14)) | (0b01<<14)
    return words[:4]+last.to_bytes(4,'little')


def build(llvm6: str):
    mod = load_parent()
    end = validate_manifest()
    new_parts = []
    for name, source, _, size in FRAGMENTS:
        if name == 'bitmap-predicate-pair':
            part = assemble_predicate_pair(mod, llvm6, source)
        elif name == 'compound-branch':
            part = assemble_compound_branch(mod, llvm6, source)
        else:
            part = mod.assemble(source, llvm6, 'hexagonv65', expected_length=size)
        if len(part) != size:
            raise ValueError(f'fragment {name} produced {len(part)} rather than {size} bytes')
        new_parts.append(part)
    content = b''.join(new_parts)
    if len(content) != end - START:
        raise ValueError('unexpected fragment byte count')
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
    report={'agent':13,'checkpoint':'30-disjoint',
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
