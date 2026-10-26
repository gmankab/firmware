#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent SHA-256 verifier for Novatek DLM *external* seed bytes.

The TSV stores digests and positions only, never private payloads. This is a
revision guard, not proof of source authorship or a new OEM signature.
"""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import re

PINNED = "8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5"
DLM_START, DLM_END = 0x22000, 0x2E190
HEADERS = (
    "# novatek-dlm-external-seeds-v1",
    "# profile postmarketos-nabu",
    "# reference-sha256 " + PINNED,
    "# ranges 60",
    "# external-bytes 27215",
    "# start-inclusive end-exclusive sha256",
)
RECORD = re.compile(r"(0x[0-9a-f]{6})\t(0x[0-9a-f]{6})\t([0-9a-f]{64})\Z")


def parse_manifest(text: str) -> list[tuple[int, int, str]]:
    lines = text.splitlines()
    if tuple(lines[:6]) != HEADERS:
        raise ValueError("manifest version/profile/reference hash/plan headers mismatch")
    if len(lines) != 66:
        raise ValueError(f"expected 60 DLM seed records, got {len(lines)-6}")
    found = []
    prior_end = DLM_START
    for i, line in enumerate(lines[6:]):
        match = RECORD.fullmatch(line)
        if match is None:
            raise ValueError(f"noncanonical DLM seed manifest row {i}")
        a, b = int(match[1], 16), int(match[2], 16)
        if not (DLM_START <= a < b <= DLM_END and a >= prior_end):
            raise ValueError(f"bad, duplicate, overlapping or out-of-bounds seed range {i}")
        found.append((a, b, match[3]))
        prior_end = b
    if len(found) != 60 or sum(b-a for a, b, _ in found) != 27215:
        raise ValueError("DLM external seed count/byte totals differ from pinned profile")
    return found


def verify_plan(plan: dict, entries: list[tuple[int, int, str]]) -> None:
    expected = {"schema":"agent20-19-dlm-sparse-v1", "source_dlm_bytes":9353,
                "dependent_unique_bytes":12984,"required_external_seed_bytes":27215,
                "independent_dependent_crosschecks":40}
    for field, value in expected.items():
        if plan.get(field) != value:
            raise ValueError(f"DLM sparse plan {field} does not match pinned manifest")
    ranges = plan.get("seed_ranges")
    if not isinstance(ranges, list) or len(ranges) != len(entries):
        raise ValueError("DLM sparse plan range count differs from manifest")
    for i, (region, (a, b, _)) in enumerate(zip(ranges, entries)):
        if (region.get("file_start"),region.get("file_end"),region.get("bytes")) != (a,b,b-a):
            raise ValueError(f"DLM sparse plan range #{i} differs from pinned manifest")


def verify_seeds(entries: list[tuple[int, int, str]], seeds: list[tuple[int, bytes]]) -> int:
    if len(entries) != len(seeds):
        raise ValueError("missing/extra external seeds")
    for i, ((a,b,sha),(where,blob)) in enumerate(zip(entries,seeds)):
        if where != a or len(blob) != b-a:
            raise ValueError(f"DLM seed #{i} wrong file offset or length")
        if hashlib.sha256(blob).hexdigest() != sha:
            raise ValueError(f"DLM seed #{i} SHA-256 mismatch at 0x{a:x} (wrong revision or altered input)")
    return sum(len(data) for _, data in seeds)


def from_reference(entries: list[tuple[int, int, str]], image: bytes) -> list[tuple[int,bytes]]:
    if len(image) != 196608 or hashlib.sha256(image).hexdigest() != PINNED:
        raise ValueError("reference Novatek profile, length or whole-image SHA-256 mismatch")
    return [(a,image[a:b]) for a,b,_ in entries]


def from_seed_dir(entries: list[tuple[int,int,str]], directory: Path) -> list[tuple[int,bytes]]:
    wanted = {f"dlm-external-{a:06x}-{b:06x}.bin" for a,b,_ in entries}
    seen = {p.name for p in directory.iterdir() if p.name.startswith("dlm-external-") and p.name.endswith(".bin")}
    if seen != wanted:
        raise ValueError(f"missing/extra external seed filenames: missing {len(wanted-seen)} extra {len(seen-wanted)}")
    return [(a,(directory/f"dlm-external-{a:06x}-{b:06x}.bin").read_bytes()) for a,b,_ in entries]


def run(argv=None):
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest',required=True,type=Path)
    parser.add_argument('--plan',required=True,type=Path)
    group=parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--reference',type=Path)
    group.add_argument('--seed-dir',type=Path)
    args=parser.parse_args(argv)
    entries=parse_manifest(args.manifest.read_text())
    verify_plan(json.loads(args.plan.read_text()),entries)
    seeds=from_reference(entries,args.reference.read_bytes()) if args.reference else from_seed_dir(entries,args.seed_dir)
    checked=verify_seeds(entries,seeds)
    print(json.dumps({'schema':'agent20-19-dlm-seed-sha256-v1','reference_revision':PINNED,
                      'source_credits_added':0,'external_seed_count':len(entries),
                      'external_bytes_verified':checked,'verified':True},sort_keys=True))
    return 0


if __name__=='__main__':
    try:raise SystemExit(run())
    except (ValueError,OSError,KeyError) as exc:
        raise SystemExit(f"DLM seed verification failed: {exc}")
