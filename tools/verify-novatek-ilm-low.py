#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify Agent 17's source-built Novatek ILM with an independent GNU assembler.

The reference is used only for comparisons and EX9 dependency inspection.
It is never an input to the Rust emitter or the assembly source generator.
An explicit --skip-assembler writes a PARTIAL report and exits with status 2;
it must not satisfy the complete checkpoint gate. No binary-data directives
are substituted for unsupported instructions.
"""

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys


PROFILES = {
    "postmarketos": {
        "sha256": "8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5",
        "size": 196608,
        "gp": 0x2D9E0,
        "itb": 0x17BC0,
        "ilm_length": 0x1AA80,
        "ex9": {
            0x1: {"operation": "jal", "target_low25": 0x84AA},
            0x2: {"operation": "jal", "target_low25": 0x23DE},
            0x3: {"operation": "movi", "destination_register": 0, "immediate": 0x3F04A},
            0x4: {"operation": "movi", "destination_register": 1, "immediate": 0x55},
            0x5: {"operation": "jal", "target_low25": 0x25FE},
            0xA: {"operation": "jal", "target_low25": 0x85E8},
            0x24: {"operation": "movi", "destination_register": 1, "immediate": 0x2FEF0},
            0x36: {"operation": "movi", "destination_register": 3, "immediate": 0x55},
            0x3A: {"operation": "movi", "destination_register": 2, "immediate": 0x3F04A},
            0x3C: {"operation": "movi", "destination_register": 2, "immediate": 0x55},
            0x3F: {"operation": "lwi.gp", "destination_register": 0, "address": 0x2C510},
            0x53: {"operation": "movi", "destination_register": 1, "immediate": 0x3F04A},
            0x58: {"operation": "jal", "target_low25": 0x2CEC},
            0x66: {"operation": "andi", "destination_register": 6, "source_register": 6, "immediate": 0x7FFF},
            0x68: {"operation": "movi", "destination_register": 0, "immediate": 0x2FEF0},
            0x6A: {"operation": "ori", "destination_register": 0, "source_register": 0, "immediate": 1},
            0x7D: {"operation": "mtsr", "source_register": 0, "csr": 0xC1, "csr_name": "int_mask2"},
            0xA1: {"operation": "movi", "destination_register": 0, "immediate": 0x20000},
            0xAF: {"operation": "movi", "destination_register": 1, "immediate": 0x2FE50},
            0xDC: {"operation": "swi.gp", "source_register": 0, "address": 0x2C580},
            0x11A: {"operation": "movi", "destination_register": 0, "immediate": 0x2FEF1},
            0x121: {"operation": "andi", "destination_register": 0, "source_register": 0, "immediate": 0xF0},
            0x162: {"operation": "mtsr", "source_register": 0, "csr": 0xC2, "csr_name": "int_mask3"},
            0x166: {"operation": "srli", "destination_register": 0, "source_register": 6, "shift": 8},
            0x18C: {"operation": "sbi", "source_register": 1, "base_register": 0, "displacement": 0x3C},
            0x1A4: {"operation": "movi", "destination_register": 0, "immediate": 0x3F00E},
            0x1AF: {"operation": "ori", "destination_register": 0, "source_register": 0, "immediate": 8},
            0x1BC: {"operation": "swi.gp", "source_register": 0, "address": 0x2C58C},
            0x1C5: {"operation": "movi", "destination_register": 0, "immediate": 0x2FE60},
            0x2E: {"operation": "swi.gp", "source_register": 0, "address": 0x2C73C},
            0xAB: {"operation": "lwi.gp", "destination_register": 0, "address": 0x2ECA8},
            0x141: {"operation": "swi.gp", "source_register": 0, "address": 0x2C740},
            0x0: {'operation': 'lwi.gp', 'destination_register': 0, 'address': 181412},
            0x6: {'operation': 'jal', 'target_low25': 58152},
            0x12: {'operation': 'lwi.gp', 'destination_register': 1, 'address': 181412},
            0x14: {'operation': 'jal', 'target_low25': 35844},
            0x17: {'operation': 'jal', 'target_low25': 34122},
            0x19: {'operation': 'lwi.gp', 'destination_register': 0, 'address': 181564},
            0x1b: {'operation': 'lwi.gp', 'destination_register': 0, 'address': 181408},
            0x1f: {'operation': 'lmw', 'first_register': 6, 'base_register': 31, 'last_register': 6, 'special_mask': 2, 'decrement': False, 'writeback': True},
            0x28: {'operation': 'jal', 'target_low25': 63084},
            0x30: {'operation': 'movi', 'destination_register': 0, 'immediate': 85},
            0x39: {'operation': 'jal', 'target_low25': 48484},
            0x3d: {'operation': 'movi', 'destination_register': 0, 'immediate': -86},
            0x59: {'operation': 'jal', 'target_low25': 60480},
            0x7e: {'operation': 'smw', 'first_register': 16, 'base_register': 1, 'last_register': 25, 'special_mask': 0, 'decrement': False, 'writeback': True},
            0x84: {'operation': 'movi', 'destination_register': 0, 'immediate': 196176},
            0x95: {'operation': 'sbi.gp', 'source_register': 0, 'address': 181176},
            0x9a: {'operation': 'jal', 'target_low25': 58690},
            0xa3: {'operation': 'lmw', 'first_register': 16, 'base_register': 0, 'last_register': 25, 'special_mask': 0, 'decrement': False, 'writeback': True},
            0xa9: {'operation': 'movi', 'destination_register': 0, 'immediate': 105},
            0xd0: {'operation': 'jal', 'target_low25': 59674},
            0xe0: {'operation': 'movi', 'destination_register': 2, 'immediate': 181576},
            0x122: {'operation': 'movi', 'destination_register': 1, 'immediate': 191656},
            0x132: {'operation': 'sbi.gp', 'source_register': 0, 'address': 191774},
            0x154: {'operation': 'jal', 'target_low25': 60664},
            0x15c: {'operation': 'movi', 'destination_register': 1, 'immediate': 181249},
            0x17d: {'operation': 'jal', 'target_low25': 60676},
            0x19f: {'operation': 'jal', 'target_low25': 57964},
            0x1ad: {'operation': 'jal', 'target_low25': 60626},
            0x1c6: {'operation': 'jal', 'target_low25': 60736},
            0x1f8: {'operation': 'lwi.gp', 'destination_register': 3, 'address': 181460},
            0x1f9: {'operation': 'shi.gp', 'source_register': 0, 'address': 181300},
        },
    },
    "map220v": {
        "sha256": "72364eca0a91855d59e043371aca774354e5ad0885df5024edcec8f976543ad2",
        "size": 196608,
        "gp": 0x2D7F0,
        "itb": 0x17010,
        "ilm_length": 0x19E90,
        "ex9": {
            0x1: {"operation": "jal", "target_low25": 0x7C78},
            0x2: {"operation": "jal", "target_low25": 0x22AC},
            0x3: {"operation": "movi", "destination_register": 0, "immediate": 0x3F04A},
            0x4: {"operation": "movi", "destination_register": 1, "immediate": 0x55},
            0x5: {"operation": "jal", "target_low25": 0x24C2},
            0xA: {"operation": "jal", "target_low25": 0x7DB6},
            0x21: {"operation": "movi", "destination_register": 1, "immediate": 0x2FEF0},
            0x28: {"operation": "movi", "destination_register": 2, "immediate": 0x55},
            0x31: {"operation": "movi", "destination_register": 3, "immediate": 0x55},
            0x36: {"operation": "movi", "destination_register": 2, "immediate": 0x3F04A},
            0x3F: {"operation": "lwi.gp", "destination_register": 0, "address": 0x2C4E8},
            0x40: {"operation": "movi", "destination_register": 1, "immediate": 0x3F04A},
            0x43: {"operation": "ori", "destination_register": 0, "source_register": 0, "immediate": 1},
            0x5B: {"operation": "jal", "target_low25": 0x2BA4},
            0x65: {"operation": "andi", "destination_register": 6, "source_register": 6, "immediate": 0x7FFF},
            0x66: {"operation": "movi", "destination_register": 0, "immediate": 0x2FEF0},
            0x7A: {"operation": "mtsr", "source_register": 0, "csr": 0xC1, "csr_name": "int_mask2"},
            0x9D: {"operation": "movi", "destination_register": 0, "immediate": 0x20000},
            0xAC: {"operation": "movi", "destination_register": 1, "immediate": 0x2FE50},
            0xB4: {"operation": "ori", "destination_register": 0, "source_register": 0, "immediate": 8},
            0xDA: {"operation": "swi.gp", "source_register": 0, "address": 0x2C558},
            0x11D: {"operation": "movi", "destination_register": 0, "immediate": 0x2FEF1},
            0x121: {"operation": "andi", "destination_register": 0, "source_register": 0, "immediate": 0xF0},
            0x16D: {"operation": "mtsr", "source_register": 0, "csr": 0xC2, "csr_name": "int_mask3"},
            0x173: {"operation": "srli", "destination_register": 0, "source_register": 6, "shift": 8},
            0x190: {"operation": "sbi", "source_register": 1, "base_register": 0, "displacement": 0x3C},
            0x1AD: {"operation": "movi", "destination_register": 0, "immediate": 0x3F00E},
            0x1C3: {"operation": "movi", "destination_register": 0, "immediate": 0x2FE60},
        },
    },
}

FILE_START = 0x66A
ASSIGNED_END = 0x10000
ILM_FILE_BASE = 0x200
# Semantic operands above are explicit expectations, not instruction-table
# byte snapshots. The table remains a comparison-only external dependency.
COMMON_CHECKPOINT_COUNTS = {
    # file end: (instructions, 32-bit, 16-bit, EX9 uses, distinct EX9 entries)
    0x6BC: (25, 16, 9, 5, 3),
    0x79C: (117, 36, 81, 19, 13),
    0x95A: (292, 84, 208, 40, 20),
}
CHECKPOINT_COUNTS = {profile: dict(COMMON_CHECKPOINT_COUNTS) for profile in PROFILES}
CHECKPOINT_COUNTS["postmarketos"][0xB1A] = (455, 145, 310, 55, 29)
CHECKPOINT_COUNTS["map220v"][0xB1C] = (455, 146, 309, 54, 28)
CHECKPOINT_COUNTS["postmarketos"][0xB22] = (458, 146, 312, 55, 29)
CHECKPOINT_COUNTS["map220v"][0xB24] = (458, 147, 311, 54, 28)
CHECKPOINT_COUNTS["postmarketos"][0xB42] = (471, 149, 322, 58, 32)
CHECKPOINT_COUNTS["postmarketos"][0xE34] = (758, 239, 519, 132, 63)
SUPPORTED_MNEMONICS = {
    2: frozenset({
        "addi45", "beqs38", "bnes38", "beqz38", "bnez38", "beqzs8", "bnezs8",
        "add45", "bmski33", "fexti33", "slt45", "ex9.it", "ifcall9", "ifret16", "j8", "jr5", "jral5",
        "lbi333", "lhi333", "lwi333", "lwi37", "lwi450", "mov55", "movi55", "movpi45", "or33",
        "pop25", "push25", "ret5", "sbi333", "seb33", "shi333", "slti45", "srli45", "subi333", "subi45",
        "swi450", "xlsb33", "zeb33", "zeh33",
    }),
    4: frozenset({
        "add_slli", "addi", "andi", "beqc", "beqz", "bitci", "bnec", "bset", "dsb",
        "ifcall", "lhi.gp", "lmw.bi", "smw.bi", "j", "jal", "jr", "jral", "lb", "lbi.gp", "lmw.bim", "lwi", "lwi.gp", "mfsr", "movi",
        "mtsr", "or_slli", "ori", "sb", "sbi", "sbi.gp", "sethi", "sll", "smw.adm", "srli", "swi.gp",
    }),
}
# No directives, labels, strings, statement separators or assembler comments.
# '<' occurs only as the complete shift token '<<' in indexed memory syntax.
SAFE_ASSEMBLY = re.compile(r"(?:[A-Za-z0-9_$.,+\-\[\]() #]|<<)+")
GNU_NDS32_SOURCE = (
    "https://chromium.googlesource.com/native_client/nacl-binutils/"
    "+/refs/tags/binutils-2_25_1/opcodes/nds32-dis.c"
)


class VerificationFailure(Exception):
    pass


def require(condition, message):
    if not condition:
        raise VerificationFailure(message)


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def run(command, report):
    record = {"argv": [str(x) for x in command]}
    report["commands"].append(record)
    try:
        proc = subprocess.run(record["argv"], text=True, capture_output=True, timeout=60)
    except (OSError, subprocess.TimeoutExpired) as exc:
        record["error"] = str(exc)
        raise VerificationFailure(str(exc)) from exc
    record.update(returncode=proc.returncode, stdout=proc.stdout, stderr=proc.stderr)
    require(proc.returncode == 0,
            f"command failed ({proc.returncode}): {record['argv'][0]}: {proc.stderr.strip()}")
    return proc.stdout


def run_producing(command, output, report, inputs=()):
    """Require a fresh product even when a producer incorrectly exits zero."""
    output = Path(output)
    protected = [report.get("reference", {}).get("path"), report.get("emitter", {}).get("path"),
                 command[0], *inputs]
    require(all(output.resolve() != Path(path).resolve() for path in protected if path is not None),
            f"tool output would overwrite a protected input: {output}")
    output.unlink(missing_ok=True)
    stdout = run(command, report)
    require(output.is_file(), f"command succeeded without creating its output: {output}")
    return stdout


def executable(value):
    resolved = shutil.which(value)
    require(resolved is not None, f"required executable not found: {value}")
    return str(Path(resolved).resolve())


def integer_operand(text):
    value = text.strip()
    if value.startswith("#"):
        value = value[1:]
    require(re.fullmatch(r"[+-]?(?:0[xX][0-9a-fA-F]+|0|[1-9][0-9]*)", value) is not None,
            f"expected a literal decimal or hexadecimal integer: {text}")
    return int(value, 0)


def register_number(text):
    name = text.strip().lower().removeprefix("$")
    if re.fullmatch(r"r(?:[0-9]|[12][0-9]|3[01])", name):
        return int(name[1:])
    return {"ta": 15, "fp": 28, "gp": 29, "lp": 30, "sp": 31}.get(name)


def validate_assembly(assembly, size, address):
    tokens = assembly.split(None, 1)
    require(bool(tokens), f"empty assembly at 0x{address:x}")
    mnemonic = tokens[0].lower()
    require(re.fullmatch(r"[a-z][a-z0-9_.]*", mnemonic) is not None,
            f"assembly directives are forbidden in emitter listing: {assembly}")
    require(mnemonic in SUPPORTED_MNEMONICS[size],
            f"unsupported {size}-byte instruction mnemonic: {mnemonic}")
    require(SAFE_ASSEMBLY.fullmatch(assembly) is not None,
            f"unsupported assembly syntax at runtime 0x{address:x}: {assembly}")
    operands = tokens[1] if len(tokens) == 2 else ""
    # '#' is permitted as a numeric-immediate prefix, never as a comment.
    require(re.search(r"#(?![+-]?(?:0[xX][0-9a-fA-F]+|[0-9]+)(?=$|[,\s)\]]))",
                      operands) is None,
            f"non-numeric immediate or comment in assembly: {assembly}")
    brackets = []
    for char in operands:
        if char in "[(":
            brackets.append(char)
        elif char in "])":
            require(brackets and brackets.pop() == {"]": "[", ")": "("}[char],
                    f"unbalanced memory operand: {assembly}")
    require(not brackets, f"unbalanced memory operand: {assembly}")
    fields = [field.strip() for field in operands.split(",")]
    if mnemonic == "mov55" and len(fields) == 2:
        require([register_number(field) for field in fields] != [31, 31],
                "MOV55 sp,sp is the reserved IFRET16 alias; use ifret16 explicitly")
    if mnemonic in {"beqs38", "bnes38"}:
        require(register_number(fields[0]) != 5,
                f"{mnemonic.upper()} r5 occupies the reserved jump/EX9 instruction subspace")
    if mnemonic == "ifcall9":
        displacement = integer_operand(operands)
        require(0 <= displacement <= 1022 and displacement % 2 == 0,
                "IFCALL9 requires an unsigned even byte displacement in [0,1022]")
    return mnemonic, operands


def parse_listing(text, profile):
    lines = text.splitlines()
    require(lines and lines[0].startswith("# "), "emitter listing lacks its metadata header")
    fields = lines[0][2:].split()
    require(all(re.fullmatch(r"[a-z_]+=[^\s=]+", field) for field in fields),
            "malformed listing metadata")
    pairs = [field.split("=", 1) for field in fields]
    metadata = dict(pairs)
    require(len(metadata) == len(pairs), "duplicate listing metadata keys")
    require(metadata.get("profile") == profile, "listing uses the wrong firmware profile")
    for key in ("gp", "itb", "file_start", "file_end"):
        require(key in metadata, f"listing lacks {key}")
        metadata[key] = int(metadata[key], 0)
    require(metadata["gp"] == PROFILES[profile]["gp"], "listing GP does not match the pinned profile")
    require(metadata["itb"] == PROFILES[profile]["itb"], "listing ITB does not match the pinned profile")
    require(metadata["file_start"] == FILE_START, "listing must include the whole Agent 17 span from 0x66a")
    require(FILE_START < metadata["file_end"] <= ASSIGNED_END, "listing exceeds the exclusive lower-ILM reservation")
    instructions = []
    pc = metadata["file_start"] - ILM_FILE_BASE
    for line_number, line in enumerate(lines[1:], 2):
        require(bool(line.strip()), f"blank instruction row {line_number}")
        parts = line.split("\t", 2)
        require(len(parts) == 3, f"malformed instruction row {line_number}")
        require(re.fullmatch(r"[0-9a-fA-F]+", parts[0]) is not None,
                f"malformed runtime address in instruction row {line_number}")
        address, size, assembly = int(parts[0], 16), int(parts[1]), parts[2].strip()
        require(address == pc, f"listing gap/overlap at runtime 0x{pc:x}")
        require(size in (2, 4), f"invalid instruction width {size} at 0x{address:x}")
        mnemonic, operands = validate_assembly(assembly, size, address)
        instructions.append({"runtime_address": address, "file_offset": address + ILM_FILE_BASE,
                             "length": size, "mnemonic": mnemonic,
                             "operands": operands, "assembly": assembly})
        pc += size
    require(pc + ILM_FILE_BASE == metadata["file_end"], "listing end does not match its claimed interval")
    return metadata, instructions


def compare_bytes(actual, expected, file_start, description):
    count = sum(a != b for a, b in zip(actual, expected)) + abs(len(actual) - len(expected))
    first = next((i for i, (a, b) in enumerate(zip(actual, expected)) if a != b), None)
    if first is None and len(actual) != len(expected):
        first = min(len(actual), len(expected))
    result = {"actual_bytes": len(actual), "expected_bytes": len(expected), "mismatched_bytes": count,
              "first_mismatch_file_offset": None if first is None else file_start + first,
              "actual_sha256": sha256(actual), "expected_sha256": sha256(expected)}
    if count:
        raise VerificationFailure(f"{description}: {count} byte mismatches; first at file 0x{file_start + first:x}")
    return result


def signed_field(word, bits):
    value = word & ((1 << bits) - 1)
    return value - (1 << bits) if value & (1 << (bits - 1)) else value


def verify_effect(table_bytes, effect, gp=None):
    """Check documented operand fields against the expected effect, not copied opcode constants."""
    require(len(table_bytes) == 4, "truncated EX9 dependency entry")
    word = int.from_bytes(table_bytes, "big")
    operation = effect["operation"]
    if operation == "mtsr":
        return ((word >> 25) == 0x32 and (word >> 20 & 31) == effect["source_register"]
                and (word >> 10 & 1023) == effect["csr"] and (word & 1023) == 3)
    if operation in {"andi", "ori"}:
        opcode = {"andi": 0x2A, "ori": 0x2C}[operation]
        return ((word >> 25) == opcode and (word >> 20 & 31) == effect["destination_register"]
                and (word >> 15 & 31) == effect["source_register"]
                and (word & 0x7FFF) == effect["immediate"])
    if operation == "movi":
        return ((word >> 25) == 0x22 and (word >> 20 & 31) == effect["destination_register"]
                and signed_field(word, 20) == effect["immediate"])
    if operation == "srli":
        return ((word >> 25) == 0x20 and (word >> 20 & 31) == effect["destination_register"]
                and (word >> 15 & 31) == effect["source_register"]
                and (word >> 10 & 31) == effect["shift"] and (word & 1023) == 9)
    if operation in {"j", "jal"}:
        return ((word >> 25) == 0x24 and (word >> 24 & 1) == (operation == "jal")
                and (word & 0xFFFFFF) << 1 == effect["target_low25"])
    if operation in {"lhi.gp", "lhsi.gp", "shi.gp"}:
        require(gp is not None, "GP-relative halfword operation requires pinned GP")
        t = {"lhi.gp": 0, "lhsi.gp": 2, "shi.gp": 4}[operation]
        reg = effect["source_register" if t == 4 else "destination_register"]
        address = (gp + signed_field(word, 18) * 2) & 0xFFFFFFFF
        return ((word >> 25) == 0x1E and (word >> 20 & 31) == reg
                and (word >> 18 & 3) == t // 2 and address == effect["address"])
    if operation == "sbi.gp":
        require(gp is not None, "GP-relative byte operation requires pinned GP")
        address = (gp + signed_field(word, 19)) & 0xFFFFFFFF
        return ((word >> 25) == 0x1F and (word >> 20 & 31) == effect["source_register"]
                and address == effect["address"])
    if operation in {"smw", "lmw"}:
        return ((word >> 25) == 0x1D and bool(word & 0x20) == (operation == "smw")
                and (word >> 20 & 31) == effect["first_register"]
                and (word >> 15 & 31) == effect["base_register"]
                and (word >> 10 & 31) == effect["last_register"]
                and (word >> 6 & 15) == effect["special_mask"]
                and bool(word & 0x18) == effect["decrement"]
                and bool(word & 4) == effect["writeback"])
    if operation in {"lwi.gp", "swi.gp"}:
        require(gp is not None and 0 <= gp <= 0xFFFFFFFF,
                "GP-relative EX9 effect requires the pinned profile GP")
        load = operation == "lwi.gp"
        register = effect["destination_register" if load else "source_register"]
        # HWGP selectors 6/7 carry a signed 17-bit word displacement.
        # GNU 2.25.1 opcodes/nds32-asm.c: lwi.gp/swi.gp, operand i17s2.
        address = (gp + signed_field(word, 17) * 4) & 0xFFFFFFFF
        return ((word >> 25) == 0x1E and (word >> 20 & 31) == register
                and (word >> 17 & 7) == (6 if load else 7) and address == effect["address"])
    if operation == "sbi":
        return ((word >> 25) == 0x8 and (word >> 20 & 31) == effect["source_register"]
                and (word >> 15 & 31) == effect["base_register"]
                and signed_field(word, 15) == effect["displacement"])
    raise VerificationFailure(f"unsupported expected EX9 effect: {operation}")


def ex9_jump_target(table_word, call_site_pc):
    """Resolve table J/JAL with the EX9 call-site PC, never the table address.

    GNU 2.25.1 nds32-dis.c, nds32_ex9_info and nds32_parse_opcode:
    PC(31,25) | Inst(23,0)<<1. Unlike an ordinary J/JAL, the low target
    is absolute within the call site's 32 MiB region. Mask before combining
    to avoid extending a normal J/JAL's signed relative interpretation.
    """
    require(0 <= table_word <= 0xFFFFFFFF and table_word >> 25 == 0x24,
            "EX9 jump target requested for a non-J/JAL table instruction")
    require(0 <= call_site_pc <= 0xFFFFFFFF and call_site_pc % 2 == 0,
            "invalid EX9 call-site PC")
    return (call_site_pc & 0xFE000000) | ((table_word & 0xFFFFFF) << 1)


def inspect_ex9(reference, profile, metadata, instructions):
    expected = PROFILES[profile]["ex9"]
    uses = []
    entries = {}
    for instruction in instructions:
        if instruction["mnemonic"] != "ex9.it":
            continue
        require(instruction["length"] == 2, "EX9.IT must be a 16-bit instruction")
        index = integer_operand(instruction["operands"])
        require(0 <= index <= 511, f"invalid EX9 index {index}")
        require(index in expected, f"EX9 index 0x{index:x} lacks an independently specified semantic expectation")
        effect = expected[index]
        table_runtime = metadata["itb"] + 4 * index
        table_file = table_runtime + ILM_FILE_BASE
        table_bytes = reference[table_file:table_file + 4]
        require(verify_effect(table_bytes, effect, gp=PROFILES[profile]["gp"]),
                f"EX9 index 0x{index:x} at table file 0x{table_file:x} does not match expected {effect}")
        use = {"runtime_address": instruction["runtime_address"], "file_offset": instruction["file_offset"],
               "index": index, "expected_effect": effect}
        if effect["operation"] in {"j", "jal"}:
            target = ex9_jump_target(int.from_bytes(table_bytes, "big"), instruction["runtime_address"])
            use["target_runtime_address"] = target
            use["target_file_offset"] = target + ILM_FILE_BASE
            use["target_encoding"] = "EX9-call-site-PC-high7 | table-instruction-low24<<1"
        uses.append(use)
        entries[index] = {"index": index, "runtime_address": table_runtime, "file_offset": table_file,
                          "bytes": 4, "sha256": sha256(table_bytes), "expected_effect": effect}
    return {"status": "passed", "uses": len(uses), "distinct_entries": len(entries),
            "external_reference_bytes": 4 * len(entries), "source_owner": "Agent 18",
            "control_transfer_uses": sum("target_runtime_address" in use for use in uses),
            "jump_semantics_source": GNU_NDS32_SOURCE,
            "gp_assumption": PROFILES[profile]["gp"],
            "counted_as_agent17_reconstructed_bytes": 0, "use_sites": uses,
            "entries": [entries[i] for i in sorted(entries)]}


def verify_checkpoint_counts(profile, file_end, counts):
    expected = CHECKPOINT_COUNTS[profile]
    known_ends = {end for checkpoints in CHECKPOINT_COUNTS.values() for end in checkpoints}
    require(file_end not in known_ends or file_end in expected,
            f"checkpoint end 0x{file_end:x} belongs to a different firmware profile")
    if file_end in expected:
        require(tuple(counts) == expected[file_end],
                f"{profile} checkpoint ending at file 0x{file_end:x} changed instruction/dependency accounting unexpectedly")


def control_flow(instructions, start, end, ex9_dependencies=None):
    branches = {"beqz38", "bnez38", "beqs38", "bnes38", "beqzs8", "bnezs8", "j8",
                "j", "jal", "beq", "bne", "beqz", "bnez", "bgez", "bltz", "bgtz", "blez",
                "bgezal", "bltzal", "beqc", "bnec", "ifcall", "ifcall9"}
    calls = {"jal", "bgezal", "bltzal", "ifcall", "ifcall9"}
    result = []
    indirect = []
    ifc_returns = []
    boundaries = {instruction["runtime_address"] for instruction in instructions}
    ex9_uses = {use["runtime_address"]: use for use in (ex9_dependencies or {}).get("use_sites", [])}
    for instruction in instructions:
        mnemonic = instruction["mnemonic"]
        pc = instruction["runtime_address"]
        if mnemonic in {"jr", "jr5", "jral", "jral5", "ret5", "pop25"}:
            indirect.append({"runtime_address": pc, "instruction": instruction["assembly"],
                             "target_status": "register-dependent; unresolved by static listing"})
        if mnemonic == "ifret16":
            ifc_returns.append({"runtime_address": pc, "instruction": instruction["assembly"],
                                "target_status": "IFC-state-dependent return or fallthrough; not resolved here"})
        if mnemonic == "ex9.it":
            require(pc in ex9_uses, f"EX9 control flow lacks a verified dependency at runtime 0x{pc:x}")
            use = ex9_uses[pc]
            if "target_runtime_address" not in use:
                continue
            target = use["target_runtime_address"]
            edge = {"kind": "call" if use["expected_effect"]["operation"] == "jal" else "branch",
                    "via_ex9_index": use["index"], "target_encoding": use["target_encoding"]}
        elif mnemonic in branches:
            operand = instruction["operands"].rsplit(",", 1)[-1].strip()
            if mnemonic == "ifcall9":
                displacement = integer_operand(operand)
                require(0 <= displacement <= 1022 and displacement % 2 == 0,
                        "IFCALL9 requires an unsigned even byte displacement in [0,1022]")
                encoding = "unsigned-byte-displacement-from-instruction-PC"
            else:
                relative = re.fullmatch(r"\.([+-])(0x[0-9a-fA-F]+|[0-9]+)", operand)
                require(relative is not None,
                        f"direct branch must use explicit PC-relative target: {instruction['assembly']}")
                displacement = integer_operand(relative[2]) * (1 if relative[1] == "+" else -1)
                encoding = "explicit-PC-relative-target"
            target = pc + displacement
            edge = {"kind": "call" if mnemonic in calls else "branch",
                    "byte_displacement": displacement, "target_encoding": encoding}
            if mnemonic in {"ifcall", "ifcall9"}:
                edge["return_semantics"] = "IFC-state-dependent; ordinary call-stack semantics are not assumed"
        else:
            continue
        require(0 <= target <= 0xFFFFFFFF and target % 2 == 0,
                f"invalid direct target 0x{target:x} at runtime 0x{pc:x}")
        require(not start <= target < end or target in boundaries,
                f"direct target 0x{target:x} lies inside an instruction")
        edge.update(source_runtime_address=pc, instruction=instruction["assembly"],
                    target_runtime_address=target, target_file_offset=target + ILM_FILE_BASE,
                    outside_emitted_interval=not start <= target < end,
                    outside_agent17_assignment=not FILE_START - ILM_FILE_BASE <= target < ASSIGNED_END - ILM_FILE_BASE)
        result.append(edge)
    return {"direct_edges": result, "external_targets": [edge for edge in result if edge["outside_emitted_interval"]],
            "indirect_transfers": indirect, "ifc_return_sites": ifc_returns,
            "scope": "Explicit transfer targets including verified EX9 J/JAL effects; no full IFC state simulation."}


def write_assembly(directory, metadata, instructions):
    source = directory / "independent.s"
    lines = ["# Source-authored exact instruction mnemonics; no reference bytes.",
             '.section .text.agent17,"ax",@progbits', ".balign 2", ".global agent17_start", "agent17_start:"]
    for instruction in instructions:
        lines.append(f"# runtime 0x{instruction['runtime_address']:x}; file 0x{instruction['file_offset']:x}")
        lines.append("  " + instruction["assembly"])
    lines.extend([".global agent17_end", "agent17_end:", ""])
    source.write_text("\n".join(lines))
    linker_script = directory / "independent.ld"
    linker_script.write_text("ENTRY(agent17_start)\nSECTIONS {\n"
                             f" .text 0x{metadata['file_start'] - ILM_FILE_BASE:x} : SUBALIGN(2) {{ *(.text.agent17) }}\n"
                             "}\n")
    return source, linker_script


def parse_disassembly(text):
    decoded = []
    for line in text.splitlines():
        match = re.match(r"\s*([0-9a-fA-F]+):\s*((?:[0-9a-fA-F]{2}[ \t]+)+)(.+)$", line)
        if match:
            raw = bytes.fromhex(match[2])
            decoded.append({"runtime_address": int(match[1], 16), "length": len(raw),
                            "assembly": match[3].strip(), "mnemonic": match[3].split()[0].lower()})
    return decoded


def independent_check(args, directory, metadata, instructions, emitted, report):
    source, linker_script = write_assembly(directory, metadata, instructions)
    if args.skip_assembler:
        return {"status": "skipped", "reason": "explicit --skip-assembler; full checkpoint gate remains unsatisfied",
                "source": str(source), "binary_data_fallback_used": False}
    paths = {name: executable(getattr(args, name)) for name in ("assembler", "objcopy", "linker", "objdump")}
    versions = {name: run([path, "--version"], report).splitlines()[0] for name, path in paths.items()}
    obj = directory / "independent.o"
    elf = directory / "independent.elf"
    binary = directory / "independent.bin"
    run_producing([paths["assembler"], "-march=v3", "-mno-all-relax", "-EL", "-mall-ext", "-o", obj, source],
                  obj, report, inputs=(source,))
    run_producing([paths["linker"], "--no-relax", "-T", linker_script, "-o", elf, obj],
                  elf, report, inputs=(linker_script, obj))
    run_producing([paths["objcopy"], "-O", "binary", "-j", ".text", elf, binary],
                  binary, report, inputs=(elf,))
    comparison = compare_bytes(binary.read_bytes(), emitted, metadata["file_start"], "independent assembly")
    disassembly = run([paths["objdump"], "-d", "-z", elf], report)
    (directory / "independent-disassembly.txt").write_text(disassembly)
    decoded = parse_disassembly(disassembly)
    require(len(decoded) == len(instructions),
            f"independent decoder found {len(decoded)} instructions, source lists {len(instructions)}")
    for actual, expected in zip(decoded, instructions):
        require((actual["runtime_address"], actual["length"]) == (expected["runtime_address"], expected["length"]),
                f"independent instruction boundary mismatch at runtime 0x{expected['runtime_address']:x}")
        require("unknown" not in actual["assembly"].lower(),
                f"unsupported instruction in independent decoder: {actual}")
        require(actual["mnemonic"] == expected["mnemonic"],
                f"independent mnemonic differs at runtime 0x{expected['runtime_address']:x}: "
                f"{actual['mnemonic']} != {expected['mnemonic']}")
    return {"status": "passed", "tool_versions": versions, "comparison_with_rust": comparison,
            "assembled_instructions": len(instructions), "decoded_instructions": len(decoded),
            "source": str(source), "elf": str(elf), "binary": str(binary),
            "binary_data_fallback_used": False, "decoded": decoded}


def verify(args, report):
    profile = PROFILES[args.profile]
    directory = args.output_directory.resolve()
    reference_path = args.reference.resolve()
    data = reference_path.read_bytes()
    report["reference"] = {"path": str(reference_path), "profile": args.profile, "size": len(data),
                           "sha256": sha256(data), "expected_size": profile["size"],
                           "expected_sha256": profile["sha256"]}
    require(len(data) == profile["size"] and sha256(data) == profile["sha256"],
            "whole-firmware reference size/SHA-256 does not match the selected pinned profile")
    ilm_file, ilm_sram, ilm_size_minus_one = struct.unpack_from("<III", data)
    require((ilm_file, ilm_sram, ilm_size_minus_one + 1) == (ILM_FILE_BASE, 0, profile["ilm_length"]),
            "unexpected ILM file-to-runtime mapping")
    emitter = executable(str(args.emitter))
    report["emitter"] = {"path": emitter, "sha256": sha256(Path(emitter).read_bytes())}
    listing = run([emitter, "listing", args.profile], report)
    metadata, instructions = parse_listing(listing, args.profile)
    emitted_path = directory / "generated.bin"
    require(emitted_path != reference_path, "output would overwrite the original reference")
    run_producing([emitter, "emit", args.profile, emitted_path], emitted_path, report)
    (directory / "source-listing.tsv").write_text(listing)
    emitted = emitted_path.read_bytes()
    reference_span = data[metadata["file_start"]:metadata["file_end"]]
    report["interval"] = {"file_start": metadata["file_start"], "file_end": metadata["file_end"],
                          "runtime_start": metadata["file_start"] - ILM_FILE_BASE,
                          "runtime_end": metadata["file_end"] - ILM_FILE_BASE}
    report["source_reference_comparison"] = compare_bytes(emitted, reference_span, metadata["file_start"], "source reconstruction")
    widths = Counter(instruction["length"] for instruction in instructions)
    report["counts"] = {"instruction_count": len(instructions), "instructions_16_bit": widths[2],
                        "instructions_32_bit": widths[4], "executable_bytes": len(emitted),
                        "metadata_bytes": 0, "layout_fill_bytes": 0, "external_bytes_in_generated_span": 0}
    report["instructions"] = instructions
    report["ex9_dependencies"] = inspect_ex9(data, args.profile, metadata, instructions)
    verify_checkpoint_counts(args.profile, metadata["file_end"],
                             (len(instructions), widths[4], widths[2], report["ex9_dependencies"]["uses"],
                              report["ex9_dependencies"]["distinct_entries"]))
    report["control_flow"] = control_flow(instructions, report["interval"]["runtime_start"],
                                          report["interval"]["runtime_end"], report["ex9_dependencies"])
    report["independent_assembly"] = independent_check(args, directory, metadata, instructions, emitted, report)
    report["limitations"] = [
        "EX9 instruction-table entries remain external Agent 18 dependencies; their bytes are not generated by this span.",
        "The interval is a partial firmware reconstruction; byte equality does not establish complete high-level semantics or hardware execution.",
        "The inherited routine prefix before file 0x66a is not rebuilt by this verifier.",
        "IFCALL9 targets and IFRET16 sites are recorded, but nested/implicit IFC return state is not simulated.",
    ]
    report["overall_status"] = "partial" if args.skip_assembler else "passed"


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reference", type=Path, required=True)
    parser.add_argument("--profile", choices=PROFILES, required=True)
    parser.add_argument("--emitter", type=Path, required=True)
    parser.add_argument("--as", dest="assembler", default="nds32le-elf-as")
    parser.add_argument("--objcopy", default="nds32le-elf-objcopy")
    parser.add_argument("--ld", dest="linker", default="nds32le-elf-ld")
    parser.add_argument("--objdump", default="nds32le-elf-objdump")
    parser.add_argument("--output-directory", type=Path, required=True)
    parser.add_argument("--skip-assembler", action="store_true",
                        help="explicitly write a partial report and exit 2 instead of satisfying the full verification gate")
    args = parser.parse_args(argv)
    args.output_directory.mkdir(parents=True, exist_ok=True)
    report = {"schema": "agent20-17-novatek-ilm-low-verification-v1", "overall_status": "failed", "commands": []}
    try:
        verify(args, report)
    except (VerificationFailure, OSError, ValueError, KeyError) as exc:
        report["error"] = str(exc)
    path = args.output_directory.resolve() / "verification.json"
    path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": report["overall_status"], "report": str(path), "error": report.get("error")}, sort_keys=True))
    return {"passed": 0, "partial": 2, "failed": 1}[report["overall_status"]]


if __name__ == "__main__":
    sys.exit(main())
