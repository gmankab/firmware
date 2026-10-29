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
    ('controller-initialize-stage3-flags', SOURCE_DIR / 'agent13-runtime-controller-init-stage3-part1.S', 0x5df0, 0xb4),
    ('controller-initialize-stage3-state', SOURCE_DIR / 'agent13-runtime-controller-init-stage3-part2.S', 0x5ea4, 0x5c),
    ('controller-initialize-stage3-mask', SOURCE_DIR / 'agent13-runtime-controller-init-stage3-part3.S', 0x5f00, 0x100),
    ('controller-initialize-stage3-validation', SOURCE_DIR / 'agent13-runtime-controller-init-stage3-part4.S', 0x6000, 0x100),
    ('controller-initialize-stage3-loop-entry', SOURCE_DIR / 'agent13-runtime-controller-init-stage3-part5.S', 0x6100, 0x48),
    ('controller-loop-start', SOURCE_DIR / 'agent13-runtime-controller-loop-start.S', 0x6148, 0x18),
    ('controller-loop-registers', SOURCE_DIR / 'agent13-runtime-controller-loop-registers.S', 0x6160, 0x08),
    ('controller-loop-duplex', SOURCE_DIR / 'agent13-runtime-controller-loop-duplex.S', 0x6168, 0x04),
    ('controller-loop-tail', SOURCE_DIR / 'agent13-runtime-controller-loop-tail.S', 0x616c, 0x10),
    ('controller-loop-publish', SOURCE_DIR / 'agent13-runtime-controller-loop-publish.S', 0x617c, 0x44),
    ('controller-loop-table', SOURCE_DIR / 'agent13-runtime-controller-loop-table.S', 0x61c0, 0x54),
    ('controller-select-planes', SOURCE_DIR / 'agent13-runtime-controller-select-planes.S', 0x6214, 0xec),
    ('controller-report-status', SOURCE_DIR / 'agent13-runtime-controller-report-status.S', 0x6300, 0x80),
    ('controller-report-callbacks', SOURCE_DIR / 'agent13-runtime-controller-report-callbacks.S', 0x6380, 0x40),
    ('controller-register-updates', SOURCE_DIR / 'agent13-runtime-controller-register-updates.S', 0x63c0, 0x44),
    ('controller-init-finalize', SOURCE_DIR / 'agent13-runtime-controller-init-finalize.S', 0x6404, 0xfc),
    ('session-buffer-loop-start', SOURCE_DIR / 'agent13-runtime-session-buffer-loop-start.S', 0x6500, 0x100),
    ('session-buffer-loop-body', SOURCE_DIR / 'agent13-runtime-session-buffer-loop-body.S', 0x6600, 0x100),
    ('session-buffer-iteration', SOURCE_DIR / 'agent13-runtime-session-buffer-iteration.S', 0x6700, 0x100),
    ('session-buffer-selection', SOURCE_DIR / 'agent13-runtime-session-buffer-selection.S', 0x6800, 0x100),
    ('session-buffer-plane-copies', SOURCE_DIR / 'agent13-runtime-session-buffer-plane-copies.S', 0x6900, 0x108),
    ('session-buffer-loop-final', SOURCE_DIR / 'agent13-runtime-session-buffer-loop-final.S', 0x6a08, 0x194),
    ('session-metadata-prefix', SOURCE_DIR / 'agent13-runtime-session-metadata-prefix.S', 0x6b9c, 0x264),
    ('session-metadata-final', SOURCE_DIR / 'agent13-runtime-session-metadata-final.S', 0x6e00, 0x11c),
    ('record-flag-probe', SOURCE_DIR / 'agent13-runtime-record-flag-probe.S', 0x6f1c, 0x98),
    ('record-status-compact', SOURCE_DIR / 'agent13-runtime-record-status-compact.S', 0x6fb4, 0x5c),
    ('record-callback-thunk', SOURCE_DIR / 'agent13-runtime-record-callback-thunk.S', 0x7010, 0x18),
    ('record-state-clear', SOURCE_DIR / 'agent13-runtime-record-state-clear.S', 0x7028, 0x34),
    ('controller-record-validate', SOURCE_DIR / 'agent13-runtime-controller-record-validate.S', 0x705c, 0x12c),
    ('session-process-prefix', SOURCE_DIR / 'agent13-runtime-session-process-prefix.S', 0x7188, 0x8c),
    ('session-record-loop', SOURCE_DIR / 'agent13-runtime-session-record-loop.S', 0x7214, 0xe4),
    ('session-record-update', SOURCE_DIR / 'agent13-runtime-session-record-update.S', 0x72f8, 0xa4),
    ('session-error-paths', SOURCE_DIR / 'agent13-runtime-session-errors.S', 0x739c, 0x104),
    ('session-parameter-dispatch', SOURCE_DIR / 'agent13-runtime-session-parameter-dispatch.S', 0x74a0, 0xc0),
    ('session-parameter-report', SOURCE_DIR / 'agent13-runtime-session-parameter-report.S', 0x7560, 0x58),
    ('session-state-prefix', SOURCE_DIR / 'agent13-runtime-session-state-prefix.S', 0x75b8, 0x28),
    ('session-state-extended-duplex', SOURCE_DIR / 'agent13-runtime-session-state-extended-branch.S', 0x75e0, 0x08),
    ('session-state-register-setup', SOURCE_DIR / 'agent13-runtime-session-state-setup.S', 0x75e8, 0x60),
    ('session-mode-10', SOURCE_DIR / 'agent13-runtime-session-mode-10-ext.S', 0x7648, 0x08),
    ('session-mode-13', SOURCE_DIR / 'agent13-runtime-session-mode-13-ext.S', 0x7650, 0x08),
    ('session-mode-16', SOURCE_DIR / 'agent13-runtime-session-mode-16-ext.S', 0x7658, 0x08),
    ('session-mode-17', SOURCE_DIR / 'agent13-runtime-session-mode-17-ext.S', 0x7660, 0x08),
    ('session-mode-flags', SOURCE_DIR / 'agent13-runtime-session-state-b0-cc.S', 0x7668, 0x1c),
    ('session-mode-29', SOURCE_DIR / 'agent13-runtime-session-mode-29-ext.S', 0x7684, 0x08),
    ('session-mode-call', SOURCE_DIR / 'agent13-runtime-session-state-d4-e4.S', 0x768c, 0x10),
    ('session-mode-40', SOURCE_DIR / 'agent13-runtime-session-mode-40-ext.S', 0x769c, 0x08),
    ('session-mode-traces', SOURCE_DIR / 'agent13-runtime-session-state-ec-f8.S', 0x76a4, 0x0c),
    ('session-mode-51', SOURCE_DIR / 'agent13-runtime-session-mode-51-ext.S', 0x76b0, 0x08),
    ('session-mode-state', SOURCE_DIR / 'agent13-runtime-session-state-100-144.S', 0x76b8, 0x44),
    ('session-mode-9', SOURCE_DIR / 'agent13-runtime-session-mode-9-ext.S', 0x76fc, 0x08),
    ('session-mode-9-tail', SOURCE_DIR / 'agent13-runtime-session-mode-9-tail.S', 0x7704, 0x54),
    ('session-case-1a0-1e4', SOURCE_DIR / 'agent13-runtime-session-case-1a0-1e4.S', 0x7758, 0x44),
    ('session-case-1e4-1f0', SOURCE_DIR / 'agent13-runtime-session-case-1e4-1f0.S', 0x779c, 0xc),
    ('session-case-1f0-264', SOURCE_DIR / 'agent13-runtime-session-case-1f0-264.S', 0x77a8, 0x74),
    ('session-case-264-2a0', SOURCE_DIR / 'agent13-runtime-session-case-264-2a0.S', 0x781c, 0x3c),
    ('session-case-2a0-2c0', SOURCE_DIR / 'agent13-runtime-session-case-2a0-2c0.S', 0x7858, 0x20),
    ('session-case-2c0-2e4', SOURCE_DIR / 'agent13-runtime-session-case-2c0-2e4.S', 0x7878, 0x24),
    ('session-case-2e4-328', SOURCE_DIR / 'agent13-runtime-session-case-2e4-328.S', 0x789c, 0x44),
    ('session-case-328-388', SOURCE_DIR / 'agent13-runtime-session-case-328-388.S', 0x78e0, 0x60),
    ('session-case-388-3b0', SOURCE_DIR / 'agent13-runtime-session-case-388-3b0.S', 0x7940, 0x28),
    ('session-case-3b0-3d8', SOURCE_DIR / 'agent13-runtime-session-case-3b0-3d8.S', 0x7968, 0x28),
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
    duplex_source=SOURCE_DIR/'agent13-runtime-session-state-short-duplex.S'
    if not duplex_source.is_file():
        raise ValueError('missing independent duplex semantic source')
    if any(raw in duplex_source.read_text() for raw in ('.incbin', '.byte', '.word', '.quad', '.long', '.fill')):
        raise ValueError('duplex helper contains forbidden opaque firmware bytes')
    for name,source,_,_ in fragments:
        if name.startswith('session-mode-') and source.name.endswith('-ext.S'):
            short=source.with_name(source.name.replace('-ext.S','-duplex.S'))
            if not short.is_file() or any(x in short.read_text() for x in ('.word','.byte','.incbin','.fill','.quad','.long')):
                raise ValueError(f'{name}: invalid duplex semantic source')
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


def assemble_controller_loop_registers(parent, llvm6: str, source: Path) -> bytes:
    """Turn a source-generated two-word *closed* register packet into one
    continuous packet, immediately followed by a distinct original duplex.

    LLVM6 assembles both `r4=add(r4,#-1)` and `r3=add(r3,#488)` with
    correct opcode bits, but closes the packet after the second word.
    The original four-operation packet ends after a separate predicate/load
    duplex word; ONLY the parse field (bits 15:14) of the second assembled
    instruction changes from `0b11` (end) to `0b01` (continuation).
    Neither original/reference opcode bits nor OEM payload bytes are inputs.
    """
    asm = source.read_text()
    if 'r4 = add(r4,#-1)' not in asm or 'r3 = add(r3,#488)' not in asm:
        raise ValueError('controller loop register semantics changed')
    raw=parent.assemble(source,llvm6,'hexagonv65',expected_length=8)
    first=int.from_bytes(raw[:4],'little')
    last=int.from_bytes(raw[4:8],'little')
    if (first>>14)&0b11 != 0b01 or (last>>14)&0b11 != 0b11:
        raise ValueError('compiler controller-loop packet marker unexpected')
    last=(last & ~(0b11<<14)) | (0b01<<14)
    return raw[:4]+last.to_bytes(4,'little')


def assemble_extended_duplex(parent, llvm6: str, extender_source: Path) -> bytes:
    """Build the two-word Hexagon packet entirely from semantic source.

    LLVM6 can encode the short jump/assignment as one duplex when assembled
    alone, and emits the signed long-branch extender when assembled alone;
    it cannot combine these two instruction forms inside one source packet.
    Combine *only compiler generated parts*, never reference opcode bytes.
    """
    duplex_source=SOURCE_DIR/'agent13-runtime-session-state-short-duplex.S'
    if 'jump ##0x4a0' not in extender_source.read_text():
        raise ValueError('unexpected extended jump displacement semantics')
    if 'r1 = #63; jump 0x80' not in duplex_source.read_text():
        raise ValueError('unexpected duplex register-and-branch semantics')
    extended = parent.assemble(extender_source, llvm6, 'hexagonv65', expected_length=8)
    duplex = parent.assemble(duplex_source, llvm6, 'hexagonv65', expected_length=4)
    e=int.from_bytes(extended[:4], 'little')
    d=int.from_bytes(duplex, 'little')
    if (e>>14)&3 != 1 or (d>>14)&3 != 3:
        raise ValueError('invalid Hexagon extender/duplex packet ending')
    return extended[:4]+duplex


def assemble_mode_case_duplex(parent, llvm6: str, source: Path) -> bytes:
    """Combine LLVM-assembled signed extender and register/branch duplex.

    The short and extended forms are assembled from two distinct source files
    because LLVM6 cannot schedule a single duplex with an immediate extender.
    Assert packet markers; no reference firmware bytes participate in build.
    """
    duplex_source=source.with_name(source.name.replace('-ext.S','-duplex.S'))
    if source==duplex_source or not duplex_source.is_file():
        raise ValueError('missing independently authored duplex source')
    first=parent.assemble(source,llvm6,'hexagonv65',expected_length=8)
    second=parent.assemble(duplex_source,llvm6,'hexagonv65',expected_length=4)
    high=int.from_bytes(first[:4],'little')
    low=int.from_bytes(second,'little')
    if (high>>14)&3 != 1 or (low>>14)&3 != 3:
        raise ValueError('extended duplex does not form a valid 2-word packet')
    return first[:4]+second


def build(llvm6: str):
    mod = load_parent()
    end = validate_manifest()
    new_parts = []
    for name, source, _, size in FRAGMENTS:
        if name == 'bitmap-predicate-pair':
            part = assemble_predicate_pair(mod, llvm6, source)
        elif name == 'compound-branch':
            part = assemble_compound_branch(mod, llvm6, source)
        elif name == 'controller-loop-registers':
            part = assemble_controller_loop_registers(mod, llvm6, source)
        elif name == 'session-state-extended-duplex':
            part = assemble_extended_duplex(mod, llvm6, source)
        elif name.startswith('session-mode-') and name in {
            'session-mode-10','session-mode-13','session-mode-16',
            'session-mode-17','session-mode-29','session-mode-40','session-mode-51',
            'session-mode-9'}:
            part = assemble_mode_case_duplex(mod, llvm6, source)
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
    report={'agent':13,'checkpoint':'52-disjoint',
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
