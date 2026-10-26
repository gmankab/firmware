#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Read-only DLM access map from independently reconstructed upper NDS32 ILM.

The tool reads source operations from Agent 18 and optional existing DLM
provenance from Agent 19. It does not use original firmware, infer initial
values from writes, or grant any independent-data or executable-byte credit.
A GP write during packet handling is NOT by itself proof of an initializer.
"""
from __future__ import annotations

import argparse
from collections import Counter
from dataclasses import asdict, dataclass
import json
from pathlib import Path
import re

GP = 0x2D9E0
SRAM_FILE_DELTA = 0x2000
DLM_START, DLM_END = 0x22000, 0x2E190
PINNED_SHA256 = "8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5"
WIDTH = {
    "LwiGp": 4, "SwiGp": 4,
    "LhiGp": 2, "LhsiGp": 2, "ShiGp": 2,
    "LbiGp": 1, "LbsiGp": 1, "SbiGp": 1,
}
# The source CODE list and EX9 table have distinct syntax; failing closed on
# missing or duplicated entries prevents xref reports from silently shrinking.
ILM_PATTERN = re.compile(r"^\s*\(0x([0-9a-f]+),\s*([A-Za-z]\w*)\(([^\n)]*)\)\),", re.M)
EX9_PATTERN = re.compile(r"^\s*([A-Za-z]\w*)\(([^\n)]*)\),\s*// slot (\d+),", re.M)
GP_PARAMS = re.compile(r"^(R\d+),\s*(-?(?:0x[0-9a-f]+|\d+))$", re.I)


@dataclass(frozen=True)
class MemoryAccess:
    ilm_file_pc: int
    source_operation: str
    gp_displacement: int
    sram_address: int
    dlm_file_address: int
    width_bytes: int
    direction: str
    credited_region_kind: str
    ex9_slot: int | None = None


def parse_upper_ilm(text: str) -> list[tuple[int, str, str]]:
    result: list[tuple[int, str, str]] = []
    for match in ILM_PATTERN.finditer(text):
        pc = int(match.group(1), 16)
        if result and pc <= result[-1][0]:
            raise ValueError(f"out-of-order or duplicate ILM instruction at 0x{pc:x}")
        result.append((pc, match.group(2), match.group(3)))
    if not result:
        raise ValueError("missing typed NDS32 upper ILM instructions")
    return result


def parse_ex9_table(text: str) -> dict[int, tuple[str, str]]:
    out: dict[int, tuple[str, str]] = {}
    for match in EX9_PATTERN.finditer(text):
        slot = int(match.group(3))
        if slot in out:
            raise ValueError(f"duplicate EX9 slot {slot}")
        out[slot] = (match.group(1), match.group(2))
    if not out:
        raise ValueError("missing typed EX9 instructions")
    # Sparse synthetic fixtures are accepted, but a real table must declare
    # every slot. At runtime, every referenced index must exist regardless.
    return out


def validate_ledger(ledger: dict) -> list[dict]:
    if ledger.get("schema") != "agent20-19-novatek-dlm-v1":
        raise ValueError("unexpected DLM provenance schema")
    if ledger.get("profile") != "postmarketos-nabu" or ledger.get("reference_sha256") != PINNED_SHA256:
        raise ValueError("wrong Novatek profile or reference digest")
    if ledger.get("total_bytes") != 196608 or ledger.get("new_executable_bytes") != 0:
        raise ValueError("DLM ledger image length or executable provenance changed")
    rows = ledger.get("regions")
    if not isinstance(rows, list) or not rows:
        raise ValueError("DLM provenance has no regions")
    cursor = 0
    for row in rows:
        start, end = row["file_start"], row["file_end"]
        if start != cursor or end <= start or end > 196608:
            raise ValueError(f"nonpartitioning DLM provenance at 0x{start:x}")
        if not isinstance(row.get("kind"), str):
            raise ValueError("missing provenance kind")
        cursor = end
    if cursor != 196608:
        raise ValueError("DLM provenance does not cover whole image")
    dlm_rows = [row for row in rows if row["file_start"] < DLM_END and row["file_end"] > DLM_START]
    if sum(row["file_end"]-row["file_start"] for row in dlm_rows) != DLM_END-DLM_START:
        raise ValueError("DLM provenance region size disagreement")
    n_source = sum(row["file_end"]-row["file_start"] for row in dlm_rows if row["kind"] == "source_dlm_data")
    n_unknown = sum(row["file_end"]-row["file_start"] for row in dlm_rows if row["kind"] == "unknown_dlm")
    if n_source != ledger.get("source_dlm_data_bytes") or n_unknown != ledger.get("unknown_dlm_bytes"):
        raise ValueError("DLM source/unknown accounting mismatch")
    if n_source+n_unknown != DLM_END-DLM_START:
        raise ValueError("unrecognized DLM source category")
    return dlm_rows


def source_kinds(file_offset: int, width: int, rows: list[dict] | None) -> str:
    if rows is None:
        return "unchecked"
    kinds = {row["kind"] for row in rows if row["file_start"] < file_offset+width
             and file_offset < row["file_end"]}
    if not kinds:
        raise ValueError(f"unclassified DLM access 0x{file_offset:x}")
    return "+".join(sorted(kinds))


def collect_accesses(ilm: str, ex9: str, ledger: dict | None = None) -> list[MemoryAccess]:
    table = parse_ex9_table(ex9)
    rows = validate_ledger(ledger) if ledger is not None else None
    out = []
    for pc, op, params in parse_upper_ilm(ilm):
        slot = None
        if op == "Ex9It":
            slot = int(params, 0)
            if slot not in table:
                raise ValueError(f"missing EX9 operation {slot} at ILM 0x{pc:x}")
            op, params = table[slot]
        if op.endswith("Gp") and op not in WIDTH:
            raise ValueError(f"unknown GP-width operation {op} at 0x{pc:x}")
        width = WIDTH.get(op)
        if width is None:
            continue
        operand = GP_PARAMS.fullmatch(params.strip())
        if operand is None:
            raise ValueError(f"invalid GP operands at 0x{pc:x}: {op}({params})")
        gp_offset = int(operand.group(2), 0)
        sram = GP+gp_offset
        file_offset = sram+SRAM_FILE_DELTA
        if file_offset+width <= DLM_START or file_offset >= DLM_END:
            continue
        if file_offset < DLM_START or file_offset+width > DLM_END:
            raise ValueError(f"straddling DLM access at 0x{file_offset:x}, width {width}")
        kind = source_kinds(file_offset, width, rows)
        out.append(MemoryAccess(pc, op, gp_offset, sram, file_offset, width,
                                "read" if op.startswith("L") else "write", kind, slot))
    return out


def summarize(accesses: list[MemoryAccess], ledger: dict | None) -> dict:
    unknown_reads = sorted({access.dlm_file_address for access in accesses
                            if access.direction == "read" and "unknown_dlm" in access.credited_region_kind})
    # Count only instruction locations, not reference bytes or uniqueness of
    # physical data. Repeated writes do not imply static initializer source.
    kind_counts = dict(sorted(Counter(x.credited_region_kind for x in accesses).items()))
    widths = {str(n): sum(x.width_bytes == n for x in accesses) for n in [1,2,4]}
    return {
        "schema":"agent20-19-dlm-consumer-map-v1", "reference_sha256":PINNED_SHA256,
        "source_provenance": "Agent 18 independently reconstructed typed upper ILM + EX9",
        "observed_file_offset_bias":SRAM_FILE_DELTA,"postmarketos_gp":GP,
        "scanned_dlm_range":[DLM_START,DLM_END],"gp_memory_operation_sites":len(accesses),
        "unique_dlm_byte_offsets":len({x.dlm_file_address for x in accesses}),
        "read_sites":sum(x.direction == "read" for x in accesses),
        "write_sites":sum(x.direction == "write" for x in accesses),
        "expanded_ex9_sites":sum(x.ex9_slot is not None for x in accesses),
        "width_sites":widths,"provenance_class_sites":kind_counts,
        "unresolved_read_offsets":unknown_reads,
        "new_source_dlm_bytes":0,"new_executable_bytes":0,
        "source_dlm_data_bytes":ledger["source_dlm_data_bytes"] if ledger else None,
        "unresolved_dlm_bytes":ledger["unknown_dlm_bytes"] if ledger else None,
        "accesses":[asdict(x) for x in accesses],
    }



# These counts are deliberately a strict snapshot of Agent 18 checkpoint 02,
# NOT a universal truth about every possible Novatek source revision.
EXPECTED_AGENT18_CHECKPOINT02 = {
    "gp_memory_operation_sites": 79,
    "unique_dlm_byte_offsets": 37,
    "read_sites": 45,
    "write_sites": 34,
    "expanded_ex9_sites": 10,
    "width_sites": {"1":35,"2":23,"4":21},
    "provenance_class_sites": {"source_dlm_data":17,"unknown_dlm":62},
}


def verify_agent18_checkpoint02_snapshot(report: dict) -> None:
    if report.get("source_dlm_data_bytes") != 9353 or report.get("unresolved_dlm_bytes") != 40199:
        raise ValueError("Agent 19 checkpoint09 provenance was altered; cannot certify snapshot")
    for name, value in EXPECTED_AGENT18_CHECKPOINT02.items():
        if report.get(name) != value:
            raise ValueError(f"Agent18 checkpoint02 DLM xref snapshot changed at {name}")
    if report.get("unresolved_read_offsets", []) != sorted(report.get("unresolved_read_offsets", [])):
        raise ValueError("unknown read offsets are not stable and sorted")
    for required in (0x22002,0x2e106,0x2e128,0x2e184):
        if required not in report["unresolved_read_offsets"]:
            raise ValueError(f"missing observed unknown DLM read 0x{required:x}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upper-ilm-rs",type=Path,required=True)
    parser.add_argument("--upper-ex9-rs",type=Path,required=True)
    parser.add_argument("--ledger",type=Path,help="JSON from novatek-dlm ledger")
    parser.add_argument("--output",type=Path,help="write JSON report to a local file")
    parser.add_argument("--verify-checkpoint02-snapshot",action="store_true",
        help="reject incomplete/mutated Agent18 checkpoint02 source and Agent19 checkpoint09 ledger")
    args=parser.parse_args()
    ledger=json.loads(args.ledger.read_text()) if args.ledger else None
    observations = collect_accesses(args.upper_ilm_rs.read_text(),args.upper_ex9_rs.read_text(),ledger)
    report=summarize(observations,ledger)
    if args.verify_checkpoint02_snapshot:
        if ledger is None:
            raise ValueError("pinned checkpoint snapshot requires --ledger")
        verify_agent18_checkpoint02_snapshot(report)
        report["agent18_checkpoint02_snapshot_verified"] = True
    dumped=json.dumps(report,indent=2,sort_keys=True)+"\n"
    if args.output: args.output.write_text(dumped)
    else: print(dumped,end="")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
