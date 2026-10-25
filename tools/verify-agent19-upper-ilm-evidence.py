#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify optional read-only Agent18 consumer evidence for DLM state defaults.

This verifier inspects independently reconstructed *source operations*, not
original Novatek payload bytes. A successful cross-check still does not prove
all command/calibration semantics or OEM signing. The full pinned-image byte
comparison is an independent, separately required gate.
"""
from __future__ import annotations
import argparse, json, re
from pathlib import Path

GP = 0x2D9E0
SRAM_TO_FILE = 0x2000
DLM_START, DLM_END = 0x22000, 0x2E190

# runtime instruction addresses, signed GP displacement, original reset width,
# and semantic value. The four pairs are found in Agent18's typed NDS32 ILM.
RESET_PROOFS = (
    (0x155e2, "Movi55(R0, -1)", 0x155e4, "SwiGp(R0, -6244)", -6244, 4, -1),
    (0x14e16, "Movi55(R0, -1)", 0x14e18, "SwiGp(R0, -6240)", -6240, 4, -1),
    (0x1570a, "Movi55(R1, 1)", 0x15738, "SwiGp(R1, -6232)", -6232, 4, 1),
    (0x16dee, "Movi55(R0, -1)", 0x16df8, "SbiGp(R0, -6228)", -6228, 1, -1),
)


def parse_ilm(text: str) -> dict[int, str]:
    items = {}
    for match in re.finditer(r'^\s*\(0x([0-9a-fA-F]+),\s*([A-Za-z]\w*\([^\n]*?\))\),', text, re.M):
        address = int(match.group(1), 16)
        if address in items:
            raise ValueError(f"duplicate ILM address 0x{address:x}")
        items[address] = match.group(2)
    return items


def verify_ilm(text: str, ex9_table: str | None = None) -> dict:
    instructions = parse_ilm(text)
    if len(instructions) < 100:
        raise ValueError("upper-ILM source is missing or incomplete")
    results = []
    for mov_address, mov, store_address, store, gp_offset, width, value in RESET_PROOFS:
        if instructions.get(mov_address) != mov:
            raise ValueError(f"source reset operation differs at 0x{mov_address:x}")
        if instructions.get(store_address) != store:
            raise ValueError(f"source GP store differs at 0x{store_address:x}")
        file_address = GP + gp_offset + SRAM_TO_FILE
        if not DLM_START <= file_address < file_address + width <= DLM_END:
            raise ValueError(f"mapped GP store 0x{file_address:x} outside DLM")
        if not (0 <= value < 256 or -128 <= value < 0) and width == 1:
            raise ValueError("unrepresentable signed status byte")
        results.append({"source_load_at": hex(mov_address), "source_store_at": hex(store_address),
                        "file_offset": hex(file_address), "bytes": width, "reset_value": value})

    # Check in the longer monitor-flag block that R1 is not overwritten between
    # the MOVI(1) and store; its EX9.IT 436 expansion must be a store of R0,
    # not a hidden move to R1. Avoid false proof if the EX9 dependency changed.
    between = [(at, op) for at, op in instructions.items() if 0x1570a < at < 0x15738]
    for at, op in between:
        if re.match(r'^(?:Movi|Mov55|Add|Sub|Lw|Lh|Lb|And|Or|Xor|Sll|Srl)\w*\(R1,', op):
            raise ValueError(f"monitor flag R1 clobbered at 0x{at:x}")
    if instructions.get(0x15726) != "Ex9It(436)":
        raise ValueError("monitor setup's EX9 instruction index changed")
    if ex9_table is None:
        raise ValueError("upper-ILM EX9 source required to verify register preservation")
    match = re.search(r'^\s*(SbiGp\(R0, -1691\)),\s*// slot 436,', ex9_table, re.M)
    if match is None:
        raise ValueError("EX9 slot 436 changed; monitor R1 preservation unproven")

    # The adjacent word at 0x2e184 is read as a counter; there is no proof here
    # of its initial value, and the bytes 0x2e18d..0x2e190 are not claimed.
    assert sum(a["bytes"] for a in results) == 13
    return {"schema": "agent20-19-upper-ilm-gp-default-proof-v1",
            "base_gp": hex(GP), "independently_source_reconstructed_initial_bytes": 13,
            "uncredited_counter": "0x2e184..0x2e188",
            "uncredited_trailing_flags": "0x2e18d..0x2e190",
            "source_checks": results}


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--upper-ilm-rs", required=True, type=Path)
    p.add_argument("--upper-ex9-rs", required=True, type=Path)
    args = p.parse_args()
    report = verify_ilm(args.upper_ilm_rs.read_text(), args.upper_ex9_rs.read_text())
    print(json.dumps(report, sort_keys=True, indent=2))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
