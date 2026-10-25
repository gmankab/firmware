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
            0x7D: {"operation": "mtsr", "source_register": 0, "csr": 0xC1, "csr_name": "int_mask2"},
            0x162: {"operation": "mtsr", "source_register": 0, "csr": 0xC2, "csr_name": "int_mask3"},
            0x6A: {"operation": "ori", "destination_register": 0, "source_register": 0, "immediate": 1},
        },
    },
    "map220v": {
        "sha256": "72364eca0a91855d59e043371aca774354e5ad0885df5024edcec8f976543ad2",
        "size": 196608,
        "gp": 0x2D7F0,
        "itb": 0x17010,
        "ilm_length": 0x19E90,
        "ex9": {
            0x7A: {"operation": "mtsr", "source_register": 0, "csr": 0xC1, "csr_name": "int_mask2"},
            0x16D: {"operation": "mtsr", "source_register": 0, "csr": 0xC2, "csr_name": "int_mask3"},
            0x43: {"operation": "ori", "destination_register": 0, "source_register": 0, "immediate": 1},
        },
    },
}

FILE_START = 0x66A
ASSIGNED_END = 0x10000
ILM_FILE_BASE = 0x200


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


def executable(value):
    resolved = shutil.which(value)
    require(resolved is not None, f"required executable not found: {value}")
    return str(Path(resolved).resolve())


def parse_listing(text, profile):
    lines = text.splitlines()
    require(lines and lines[0].startswith("# "), "emitter listing lacks its metadata header")
    pairs = re.findall(r"([a-z_]+)=([^\s]+)", lines[0])
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
        address, size, assembly = int(parts[0], 16), int(parts[1]), parts[2].strip()
        require(address == pc, f"listing gap/overlap at runtime 0x{pc:x}")
        require(size in (2, 4), f"invalid instruction width {size} at 0x{address:x}")
        tokens = assembly.split(None, 1)
        require(bool(tokens), f"empty assembly at 0x{address:x}")
        mnemonic = tokens[0].lower()
        require(re.fullmatch(r"[a-z][a-z0-9_.]*", mnemonic) is not None,
                f"assembly directives are forbidden in emitter listing: {assembly}")
        require(re.fullmatch(r"[A-Za-z0-9_$.,+\-\[\] #]*", assembly) is not None,
                f"unsupported assembly syntax at runtime 0x{address:x}: {assembly}")
        require(mnemonic not in {"byte", "short", "word", "long", "incbin", "fill", "space", "org", "zero"},
                "raw data is not an independently assembled instruction")
        instructions.append({"runtime_address": address, "file_offset": address + ILM_FILE_BASE,
                             "length": size, "mnemonic": mnemonic,
                             "operands": tokens[1] if len(tokens) == 2 else "", "assembly": assembly})
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


def verify_effect(table_bytes, effect):
    """Check documented operand fields against the expected effect, not copied opcode constants."""
    require(len(table_bytes) == 4, "truncated EX9 dependency entry")
    word = int.from_bytes(table_bytes, "big")
    operation = effect["operation"]
    if operation == "mtsr":
        return ((word >> 25) == 0x32 and (word >> 20 & 31) == effect["source_register"]
                and (word >> 10 & 1023) == effect["csr"] and (word & 1023) == 3)
    if operation == "ori":
        return ((word >> 25) == 0x2C and (word >> 20 & 31) == effect["destination_register"]
                and (word >> 15 & 31) == effect["source_register"]
                and (word & 0x7FFF) == effect["immediate"])
    raise VerificationFailure(f"unsupported expected EX9 effect: {operation}")


def inspect_ex9(reference, profile, metadata, instructions):
    expected = PROFILES[profile]["ex9"]
    uses = []
    entries = {}
    for instruction in instructions:
        if instruction["mnemonic"] != "ex9.it":
            continue
        require(instruction["length"] == 2, "EX9.IT must be a 16-bit instruction")
        index = int(instruction["operands"].strip().lstrip("#"), 0)
        require(0 <= index <= 511, f"invalid EX9 index {index}")
        require(index in expected, f"EX9 index 0x{index:x} lacks an independently specified semantic expectation")
        effect = expected[index]
        table_runtime = metadata["itb"] + 4 * index
        table_file = table_runtime + ILM_FILE_BASE
        table_bytes = reference[table_file:table_file + 4]
        require(verify_effect(table_bytes, effect),
                f"EX9 index 0x{index:x} at table file 0x{table_file:x} does not match expected {effect}")
        uses.append({"runtime_address": instruction["runtime_address"], "file_offset": instruction["file_offset"],
                     "index": index, "expected_effect": effect})
        entries[index] = {"index": index, "runtime_address": table_runtime, "file_offset": table_file,
                          "bytes": 4, "sha256": sha256(table_bytes), "expected_effect": effect}
    return {"status": "passed", "uses": len(uses), "distinct_entries": len(entries),
            "external_reference_bytes": 4 * len(entries), "source_owner": "Agent 18",
            "counted_as_agent17_reconstructed_bytes": 0, "use_sites": uses,
            "entries": [entries[i] for i in sorted(entries)]}


def control_flow(instructions, start, end):
    branches = {"beqz38", "bnez38", "beqs38", "bnes38", "beqzs8", "bnezs8", "j8",
                "j", "jal", "beq", "bne", "beqz", "bnez", "bgez", "bltz", "bgtz", "blez",
                "bgezal", "bltzal", "beqc", "bnec", "ifcall", "ifcall9"}
    calls = {"jal", "bgezal", "bltzal", "ifcall", "ifcall9"}
    result = []
    indirect = []
    for instruction in instructions:
        mnemonic = instruction["mnemonic"]
        pc = instruction["runtime_address"]
        if mnemonic in {"jr", "jr5", "jral", "jral5"}:
            indirect.append({"runtime_address": pc, "instruction": instruction["assembly"],
                             "target_status": "register-dependent; unresolved by static listing"})
        if mnemonic not in branches:
            continue
        operand = instruction["operands"].rsplit(",", 1)[-1].strip()
        relative = re.fullmatch(r"\.([+-])(0x[0-9a-fA-F]+|[0-9]+)", operand)
        if relative:
            displacement = int(relative[2], 0) * (1 if relative[1] == "+" else -1)
            target = pc + displacement
        else:
            require(mnemonic == "ifcall9", f"direct branch must use explicit PC-relative target: {instruction['assembly']}")
            target = pc + int(operand.lstrip("#"), 0)
        result.append({"kind": "call" if mnemonic in calls else "branch", "source_runtime_address": pc,
                       "target_runtime_address": target, "target_file_offset": target + ILM_FILE_BASE,
                       "outside_emitted_interval": not start <= target < end,
                       "outside_agent17_assignment": not FILE_START - ILM_FILE_BASE <= target < ASSIGNED_END - ILM_FILE_BASE})
    return {"direct_edges": result, "external_targets": [edge for edge in result if edge["outside_emitted_interval"]],
            "indirect_transfers": indirect}


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
    run([paths["assembler"], "-march=v3", "-mno-all-relax", "-EL", "-mall-ext", "-o", obj, source], report)
    run([paths["linker"], "--no-relax", "-T", linker_script, "-o", elf, obj], report)
    run([paths["objcopy"], "-O", "binary", "-j", ".text", elf, binary], report)
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
    run([emitter, "emit", args.profile, emitted_path], report)
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
    if metadata["file_end"] == 0x6BC:
        require((len(instructions), widths[4], widths[2], report["ex9_dependencies"]["uses"],
                 report["ex9_dependencies"]["distinct_entries"]) == (25, 16, 9, 5, 3),
                "checkpoint-01 instruction/dependency accounting changed unexpectedly")
    report["control_flow"] = control_flow(instructions, report["interval"]["runtime_start"], report["interval"]["runtime_end"])
    report["independent_assembly"] = independent_check(args, directory, metadata, instructions, emitted, report)
    report["limitations"] = [
        "EX9 instruction-table entries remain external Agent 18 dependencies; their bytes are not generated by this span.",
        "The interval is a partial firmware reconstruction; byte equality does not establish complete high-level semantics or hardware execution.",
        "The inherited routine prefix before file 0x66a is not rebuilt by this verifier.",
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
