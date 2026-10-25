#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build Agent 13 WLAN Hexagon v66 code adjacent to Agent 7's prefix.

No OEM executable words are embedded. Each fragment is built from audited
Hexagon packet syntax with llvm-mc. A reference is read *only* to compare
outputs after an exact hash/ELF mapping check. --join-agent07 independently
assembles the Agent 7 prefix first, then appends strictly adjacent fragments.
The result is NOT a complete or independently signed firmware image.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
WLAN_SRC = ROOT / "sources/xiaomi-nabu/wlanmdsp"
PARENT_BUILDER = ROOT / "tools/build-wlanmdsp-startup.py"
FILE_OFFSET = 0x40000
BASE_VADDR = 0xB0000000
START = 0x2708
FRAGMENTS = (
    ("record-lookup", WLAN_SRC / "agent13-runtime-record-check.S", START, 0x98),
)


def load_parent():
    spec = importlib.util.spec_from_file_location("nabu_wlan_agent07_builder", PARENT_BUILDER)
    if spec is None or spec.loader is None:
        raise ValueError("cannot load Agent 7's pinned Hexagon assembler")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def validate_fragment_layout(fragments=FRAGMENTS):
    """Never hide overlaps or gaps using implicit byte zeroes."""
    previous = START
    for name, source, offset, length in fragments:
        if offset != previous:
            raise ValueError(f"fragment {name} is not adjacent to previous region at 0x{previous:x}")
        if length <= 0 or length % 4:
            raise ValueError(f"fragment {name} has invalid A32-aligned length")
        if not source.is_file():
            raise ValueError(f"missing independent source file: {source}")
        asm = source.read_text(encoding="utf-8")
        if any(x in asm for x in (".incbin", ".byte", ".word", ".long", ".quad", ".fill")):
            raise ValueError(f"fragment {name} contains opaque bytes or data directives")
        previous += length
    if previous > 0x100000:
        raise ValueError("Agent 13 fragment oversteps assigned file 0x100000 limit")
    return previous


def reconstruct_parent(mod, llvm6: str, llvm22: str):
    """Rebuild original Agent 7 prefix, including audited linker fill."""
    prior = mod.assemble(mod.SOURCE, llvm6, "hexagonv65")
    prior = mod.emit_with_plt(prior, llvm6, "hexagonv65")
    prior = mod.emit_with_loader_hooks(prior, llvm6, "hexagonv65")
    prior = mod.emit_with_config_lookup(prior, llvm6, "hexagonv65")
    prior = mod.emit_with_record_prefix(prior, llvm6, "hexagonv65")
    prior = mod.emit_with_complete_record_consumer(prior, llvm22)
    prior = mod.emit_with_full_runtime_helpers(prior, llvm6, "hexagonv65")
    prior = mod.emit_with_runtime_format_handler(prior, llvm22)
    prior = mod.emit_with_runtime_session_init(prior, llvm22)
    prior = mod.emit_with_runtime_reporting(prior, llvm22)
    prior = mod.emit_with_runtime_registration(prior, llvm22)
    if len(prior) != START:
        raise ValueError(f"Agent 7 prefix should be 0x{START:x} bytes, got 0x{len(prior):x}")
    return prior


def run(args):
    if args.llvm6 is None and args.join_agent07:
        raise ValueError("--join-agent07 also requires --llvm6 for the prior program")
    end = validate_fragment_layout()
    parent = load_parent()
    new_parts = [parent.assemble(path, args.llvm22, "hexagonv66", expected_length=length)
                 for name, path, offset, length in FRAGMENTS]
    added = b"".join(new_parts)
    if len(added) != end - START:
        raise ValueError("unexpected fragment byte count")
    fragment = reconstruct_parent(parent, args.llvm6, args.llvm22) + added if args.join_agent07 else added
    runtime_offset = 0 if args.join_agent07 else START
    if args.reference is not None:
        # Checks SHA-256, ELF32 machine=164, program header, exact file offsets and bytes.
        parent.check_reference(fragment, args.reference.read_bytes(), runtime_offset=runtime_offset)
    args.output.parent.mkdir(exist_ok=True, parents=True)
    args.output.write_bytes(fragment)
    prefix_length = START if args.join_agent07 else 0
    report = {
        "agent": 13,
        "module": "wlanmdsp.mbn",
        "base_git": "Agent 7 d6104bb (portable branch; not checkpoint50 descendant)",
        "file_offset_start": FILE_OFFSET + runtime_offset,
        "file_offset_end": FILE_OFFSET + runtime_offset + len(fragment),
        "vaddr_start": BASE_VADDR + runtime_offset,
        "vaddr_end": BASE_VADDR + runtime_offset + len(fragment),
        "new_instruction_bytes": len(added),
        "prior_source_instruction_bytes": 6216 if args.join_agent07 else 0,
        "prior_linker_layout_fill_bytes": 3776 if args.join_agent07 else 0,
        "total_layout_bytes": len(fragment),
        "sha256": hashlib.sha256(fragment).hexdigest(),
        "verified_reference": args.reference is not None,
        "complete_firmware": False,
        "validated_boundaries": [{"name": name, "file_start": FILE_OFFSET+offset,
                                  "file_end": FILE_OFFSET+offset+length} for name, _, offset, length in FRAGMENTS],
    }
    if args.manifest:
        args.manifest.parent.mkdir(exist_ok=True, parents=True)
        args.manifest.write_text(json.dumps(report, indent=2, sort_keys=True)+"\n")
    print(f"Agent13 WLAN: {len(added)} newly assembled Hexagon v66 bytes at file "
          f"0x{FILE_OFFSET+START:x}–0x{FILE_OFFSET+end:x}; "
          f"output {len(fragment)} layout bytes; reference match={args.reference is not None}")
    print("Partial program only: external authentication and remaining executable are NOT rebuilt.")
    return report


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument("output",type=Path)
    ap.add_argument("--llvm22",required=True,help="LLVM 22 Hexagon v66 llvm-mc")
    ap.add_argument("--llvm6",help="LLVM6 Hexagon llvm-mc; required with --join-agent07")
    ap.add_argument("--join-agent07",action="store_true",help="also reassemble all 9992 prior prefix bytes")
    ap.add_argument("--reference",type=Path,help="strict revision-pinned opt-in comparison")
    ap.add_argument("--manifest",type=Path)
    args=ap.parse_args()
    try:
        run(args)
    except (ValueError,OSError) as exc:
        ap.error(str(exc))

if __name__=="__main__": main()
