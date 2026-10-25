#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble a source-authored Hexagon ADSP bootstrap and verify its exact bytes.

This tool emits ONLY independently reconstructed fragments of ADSP RX
segment code. The fragments may be disjoint or partial. It does not splice missing vendor bytes, nor does it produce a
complete MBN image. Original firmware is read only for explicit --reference
verification, never to manufacture output.
"""
from __future__ import annotations

import argparse
import hashlib
import pathlib
import struct
import subprocess
import sys
import tempfile

# Source modules are imported lazily by the instruction encoders. Keep this
# executable usable both as `python tools/emit-adsp-entry.py` and when a test
# imports it by file path via importlib.util.spec_from_file_location.
_TOOLS_DIR = str(pathlib.Path(__file__).resolve().parent)
if _TOOLS_DIR not in sys.path:
    sys.path.insert(0, _TOOLS_DIR)

ELF_MACHINE_HEXAGON = 164
OEM_FILE_OFFSET = 0x3000
OEM_RUNTIME_VADDR = 0xF0000000
OEM_PHYSICAL_ENTRY = 0x8BE00000
GENERATED_LENGTH = 0x24
REFERENCE_SHA256 = "4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e"
ASM_SOURCE = pathlib.Path(__file__).with_name("adsp-boot-vectors.S")
CONTROLLER_SOURCE = pathlib.Path(__file__).with_name("adsp-counter-controller.S")
TLB_MAP_SOURCE = pathlib.Path(__file__).with_name("adsp-tlb-map.S")
TLB_FLUSH_SOURCE = pathlib.Path(__file__).with_name("adsp-tlb-flush.S")
MEMORY_COPY_SOURCE = pathlib.Path(__file__).with_name("adsp-dsp-memory-copy.S")
CACHELINE_COPY_SOURCE = pathlib.Path(__file__).with_name("adsp-cacheline-copy.S")
TLB_RECONCILE_SOURCE = pathlib.Path(__file__).with_name("adsp-tlb-reconcile.S")
CONTROLLER_BOOTSTRAP_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-bootstrap.S")
CONTROLLER_CACHE_COPY_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-cache-copy.S")
CONTROLLER_PHYSICAL_COPY_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-physical-copy.S")
CONTROLLER_PHYSICAL_COMPARE_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-physical-compare.S")
CONTROLLER_CFGBASE_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-cfgbase.S")
CONTROLLER_ALIGNMENT_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-nop-padding.S")
CONTROLLER_STARTUP_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-startup.S")
CONTROLLER_DEBUG_CHECK_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-debug-check.S")
CONTROLLER_VECTOR_HANDOFF_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-vector-handoff.S")
CONTROLLER_PRIVILEGED_RESTORE_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-privileged-restore.S")
CONTROLLER_RESTORE_PADDING_SOURCE = pathlib.Path(__file__).with_name("adsp-controller-restore-padding.S")
FRAGMENTS = {
    "vectors": (ASM_SOURCE, OEM_FILE_OFFSET, GENERATED_LENGTH),
    "boot-prefix": (ASM_SOURCE, OEM_FILE_OFFSET, 0x44),
    "counter-controller": (CONTROLLER_SOURCE, 0x41e0, 0xc3c),
    "controller-bootstrap": (CONTROLLER_BOOTSTRAP_SOURCE, 0x4980, 0x2c),
    "controller-cache-copy": (CONTROLLER_CACHE_COPY_SOURCE, 0x49ac, 0x5c),
    "controller-physical-copy": (CONTROLLER_PHYSICAL_COPY_SOURCE, 0x4a08, 0x4c),
    "controller-physical-compare": (CONTROLLER_PHYSICAL_COMPARE_SOURCE, 0x4a54, 0x90),
    "controller-cfgbase": (CONTROLLER_CFGBASE_SOURCE, 0x4ae4, 0x24),
    "controller-nop-padding": (CONTROLLER_ALIGNMENT_SOURCE, 0x4b08, 0xf8),
    "controller-startup": (CONTROLLER_STARTUP_SOURCE, 0x4c00, 0x68),
    "controller-debug-check": (CONTROLLER_DEBUG_CHECK_SOURCE, 0x4c68, 0x58),
    "controller-vector-handoff": (CONTROLLER_VECTOR_HANDOFF_SOURCE, 0x4cc0, 0x28),
    "controller-privileged-restore": (CONTROLLER_PRIVILEGED_RESTORE_SOURCE, 0x4ce8, 0x34),
    "controller-restore-padding": (CONTROLLER_RESTORE_PADDING_SOURCE, 0x4d1c, 0x100),
    "tlb-reconcile": (TLB_RECONCILE_SOURCE, 0x48d0, 0xb0),
    "cacheline-copy": (CACHELINE_COPY_SOURCE, 0x4818, 0x78),
    "tlb-flush": (TLB_FLUSH_SOURCE, 0x4788, 0x90),
    "tlb-map": (TLB_MAP_SOURCE, 0x46f4, 0x94),
    "syscfg-handoff": (CONTROLLER_SOURCE, 0x46a0, 0x54),
    "memory-copy": (MEMORY_COPY_SOURCE, 0x4890, 0x40),
}


class AdspError(ValueError):
    """Malformed ELF, unsupported operand layout, or a byte-identity failure."""


def checked_interval(size: int, offset: int, length: int, description: str) -> None:
    if offset < 0 or length < 0 or offset > size or length > size - offset:
        raise AdspError(
            f"{description} outside input: offset=0x{offset:x}, "
            f"length=0x{length:x}, size=0x{size:x}"
        )


def elf32_header(data: bytes) -> dict[str, int]:
    if len(data) < 52:
        raise AdspError("Truncated ELF32 header")
    if data[:4] != b"\x7fELF" or data[4:6] != b"\x01\x01":
        raise AdspError("Expected ELF32 little-endian")
    fields = struct.unpack_from("<HHIIIIIHHHHHH", data, 16)
    keys = (
        "type", "machine", "version", "entry", "phoff", "shoff", "flags",
        "ehsize", "phentsize", "phnum", "shentsize", "shnum", "shstrndx"
    )
    result = dict(zip(keys, fields, strict=True))
    if result["machine"] != ELF_MACHINE_HEXAGON:
        raise AdspError(f"Expected Hexagon ELF machine 164, got {result['machine']}")
    if result["version"] != 1 or result["ehsize"] < 52:
        raise AdspError("Invalid ELF32 header version/size")
    return result


def read_text_section(object_data: bytes, expected_length: int = GENERATED_LENGTH) -> bytes:
    """Extract executable .text from a relocatable ELF32 Hexagon object.

    Deliberately rejects relocations, missing section names, overlapping bounds,
    multiple .text sections and unexpected input size; these indicate an
    unresolved or non-deterministic bootstrap, not an acceptable partial build.
    """
    h = elf32_header(object_data)
    if h["type"] != 1:
        raise AdspError("Assembler must emit ET_REL, not linked executable")
    if h["shnum"] == 0 or h["shentsize"] < 40:
        raise AdspError("Missing valid ELF32 section headers")
    checked_interval(len(object_data), h["shoff"],
                     h["shnum"] * h["shentsize"], "section headers")

    sections = []
    for i in range(h["shnum"]):
        offset = h["shoff"] + i * h["shentsize"]
        name, typ, flags, addr, start, size, link, info, alignment, entsize = (
            struct.unpack_from("<IIIIIIIIII", object_data, offset)
        )
        if typ in (4, 9) and size:
            raise AdspError("Unresolved RELA/REL relocations are forbidden")
        if typ != 8:  # NOBITS is not stored in file
            checked_interval(len(object_data), start, size, f"section {i}")
        sections.append((name, typ, flags, start, size))

    if h["shstrndx"] >= len(sections):
        raise AdspError("Invalid shstrndx")
    _, shstr_type, _, shstr_start, shstr_size = sections[h["shstrndx"]]
    if shstr_type != 3:
        raise AdspError("Section-name table is not STRTAB")
    names = object_data[shstr_start:shstr_start + shstr_size]

    hits = []
    for name_index, typ, flags, start, size in sections:
        if name_index >= len(names):
            raise AdspError("Section name outside string table")
        end = names.find(b"\x00", name_index)
        if end < 0:
            raise AdspError("Unterminated section name")
        if names[name_index:end] == b".text":
            if typ != 1 or not (flags & 4):
                raise AdspError("Expected executable PROGBITS .text")
            hits.append(object_data[start:start + size])
    if len(hits) != 1:
        raise AdspError(f"Expected exactly one .text, got {len(hits)}")
    if len(hits[0]) != expected_length:
        raise AdspError(f"Expected {expected_length} generated bytes, got {len(hits[0])}")
    return hits[0]


def verify_reference(assembled: bytes, reference: bytes, file_offset: int = OEM_FILE_OFFSET) -> None:
    """Verify exact ADSP revision and offset; never copy from reference."""
    h = elf32_header(reference)
    if h["type"] != 2:
        raise AdspError("Expected a firmware ET_EXEC image")
    if h["entry"] != OEM_PHYSICAL_ENTRY:
        raise AdspError(f"ADSP physical entry differs: 0x{h['entry']:x}")
    if h["phentsize"] < 32 or h["phnum"] == 0:
        raise AdspError("Missing program headers")
    checked_interval(len(reference), h["phoff"],
                     h["phentsize"] * h["phnum"], "program headers")
    rx_segments = []
    for i in range(h["phnum"]):
        o = h["phoff"] + i * h["phentsize"]
        typ, offset, vaddr, paddr, filesz, memsz, flags, align = (
            struct.unpack_from("<IIIIIIII", reference, o)
        )
        if typ != 1:
            continue
        if filesz:
            checked_interval(len(reference), offset, filesz, f"PT_LOAD {i}")
        if flags & 1:
            rx_segments.append((offset, vaddr, paddr, filesz, flags))

    first = min(rx_segments) if rx_segments else None
    if first is None or first[0:3] != (
        OEM_FILE_OFFSET, OEM_RUNTIME_VADDR, OEM_PHYSICAL_ENTRY
    ):
        raise AdspError(f"First executable PT_LOAD mismatch: {first}")
    if file_offset < first[0] or file_offset + len(assembled) > first[0] + first[3]:
        raise AdspError("Reference RX segment cannot contain generated fragment")

    digest = hashlib.sha256(reference).hexdigest()
    if digest != REFERENCE_SHA256:
        raise AdspError(
            f"Wrong Xiaomi Nabu ADSP revision: sha256={digest}, "
            f"expected={REFERENCE_SHA256}"
        )
    original = reference[file_offset:file_offset + len(assembled)]
    if assembled != original:
        different = next(i for i, (a, b) in enumerate(zip(assembled, original))
                         if a != b)
        raise AdspError(
            f"ADSP byte mismatch at file 0x{file_offset + different:x}, "
            f"RX-relative 0x{file_offset - OEM_FILE_OFFSET + different:x}, "
            f"generated={assembled[different]:02x}, reference={original[different]:02x}"
        )


def assemble(llvm_mc: str, source: pathlib.Path = ASM_SOURCE,
             expected_length: int = GENERATED_LENGTH) -> bytes:
    if not source.is_file():
        raise AdspError(f"Missing source assembly {source}")
    with tempfile.TemporaryDirectory(prefix="adsp-bootstrap-") as tmp:
        obj = pathlib.Path(tmp) / "entry.o"
        # ADSP ELF flags identify v66. The v65 tool is used only for the
        # architecturally common 9 entry instructions, *not* the rest of v66.
        result = subprocess.run(
            [llvm_mc, "-triple=hexagon", "-mcpu=hexagonv65", "-filetype=obj",
             str(source), "-o", str(obj)],
            text=True, capture_output=True, check=False,
        )
        if result.returncode:
            raise AdspError(f"Hexagon assembler failed: {result.stderr.strip()}")
        return read_text_section(obj.read_bytes(), expected_length)


def assemble_syscfg_handoff(llvm_mc: str) -> bytes:
    """Rebuild the V66-only SYSCFG handoff after the 1216-byte controller.

    General Hexagon operations are compiled from their original assembly
    expressions. Narrow special-register/cache opcodes are synthesized from
    separately validated ISA fields. No reference bytes are read or embedded.
    """
    from adsp_hexagon_v66 import syscfg_handoff_steps

    output = bytearray()
    with tempfile.TemporaryDirectory(prefix="adsp-syscfg-handoff-") as tmp:
        for index, (kind, operand, size) in enumerate(syscfg_handoff_steps()):
            if kind == "isa":
                if size != 4 or not isinstance(operand, int):
                    raise AdspError("Invalid V66 typed instruction descriptor")
                output.extend(struct.pack("<I", operand))
            elif kind == "asm":
                if not isinstance(operand, str):
                    raise AdspError("Malformed Hexagon source instruction")
                source = pathlib.Path(tmp) / f"op-{index:02d}.S"
                source.write_text(".text\n" + operand + "\n")
                output.extend(assemble(llvm_mc, source, size))
            else:
                raise AdspError(f"Unknown Hexagon instruction kind {kind!r}")
    if len(output) != 0x54:
        raise AdspError("V66 handoff must generate exactly 84 bytes")
    return bytes(output)


def assemble_tlb_map(llvm_mc: str, source: pathlib.Path = TLB_MAP_SOURCE) -> bytes:
    """Replace three source-level TLBW placeholders with typed ISA operations.

    LLVM6 assembles all regular instructions *and the branch packet structure*
    from labels in the original source. Three single-packet ``nop`` sentinels
    occupy locations that the V66 toolchain cannot express. Replace only
    these precisely checked slots with the independently decoded, fixed
    operand ``tlbw(R3:2,R0)`` architectural opcode; reject any changed layout.
    """
    from adsp_hexagon_v66 import tlb_write_pair_r3_r2_index_r0

    out = bytearray(assemble(llvm_mc, source, 0x94))
    sentinel = struct.pack("<I", 0x7F00C000)  # Architecturally encoded NOP.
    replacement = struct.pack("<I", tlb_write_pair_r3_r2_index_r0())
    for offset in (0x20, 0x5C, 0x88):
        if out[offset:offset + 4] != sentinel:
            raise AdspError(f"Expected TLB placeholder at 0x{offset:x}")
        out[offset:offset + 4] = replacement
    if sentinel in (out[i:i + 4] for i in range(0, len(out), 4)):
        raise AdspError("Unresolved TLB placeholder in final generated code")
    return bytes(out)


def assemble_tlb_flush(llvm_mc: str, source: pathlib.Path = TLB_FLUSH_SOURCE) -> bytes:
    """Build all 144 bytes of TLB clear-all and indexed restore helpers.

    LLVM6 encodes every ordinary source instruction and conditional branch
    including packet scheduling. Only five architecturally unsupported TLBW
    operations are substituted from the narrow, checked v66 encoding.
    """
    from adsp_hexagon_v66 import tlb_write_pair_r3_r2_index_r0

    code = bytearray(assemble(llvm_mc, source, 0x90))
    sentinel = struct.pack("<I", 0x7F00C000)
    replacement = struct.pack("<I", tlb_write_pair_r3_r2_index_r0())
    for offset in (0x04, 0x20, 0x48, 0x64, 0x80):
        if code[offset:offset + 4] != sentinel:
            raise AdspError(f"Expected TLB placeholder at 0x{offset:x}")
        code[offset:offset + 4] = replacement
    if any(code[i:i + 4] == sentinel for i in range(0, len(code), 4)):
        raise AdspError("Unresolved TLBW sentinel in output")
    return bytes(code)


def assemble_tlb_reconcile(
    llvm_mc: str, source: pathlib.Path = TLB_RECONCILE_SOURCE
) -> bytes:
    """Build TLB reconciliation source with seven narrow v66 TLB opcodes.

    Compiler-derived packet boundaries, loop counters, branch offsets and
    extender bytes are preserved. Every unsupported instruction is encoded
    independently from its architectural operation and operand identity.
    """
    from adsp_hexagon_v66 import (
        TlbControl,
        tlb_read_pair_r5_r4_index_r3,
        tlb_write_pair_r7_r6_index_r3,
        tlb_write_pair_r1_r0_index_r3,
    )

    output = bytearray(assemble(llvm_mc, source, 0xB0))
    mappings = (
        (0x30, tlb_read_pair_r5_r4_index_r3()),
        (0x58, int(TlbControl.LOCK)),
        (0x5C, tlb_write_pair_r7_r6_index_r3()),
        (0x60, int(TlbControl.UNLOCK)),
        (0x8C, int(TlbControl.LOCK)),
        (0x90, tlb_write_pair_r1_r0_index_r3()),
        (0x94, int(TlbControl.UNLOCK)),
    )
    nop = struct.pack("<I", 0x7F00C000)
    for offset, instruction in mappings:
        if output[offset:offset+4] != nop:
            raise AdspError(f"Expected privileged TLB placeholder at 0x{offset:x}")
        output[offset:offset+4] = struct.pack("<I", instruction)
    # One genuine NOP remains at 0x4938, paired with the hardware-loop
    # increment and endloop0 marker. It is not a TLB substitute.
    legitimate_nops = {0x68}
    unresolved = [
        i for i in range(0, len(output), 4)
        if output[i:i+4] == nop and i not in legitimate_nops
    ]
    if unresolved:
        raise AdspError(f"Unresolved TLB placeholder(s): {unresolved}")
    if output[0x68:0x6C] != nop:
        raise AdspError("The hardware-loop endloop0 NOP is missing")
    return bytes(output)


def assemble_physical_copy(
    llvm_mc: str, source: pathlib.Path = CONTROLLER_PHYSICAL_COPY_SOURCE
) -> bytes:
    """Assemble a 76-byte physical address copy routine.

    LLVM6 reconstructs the branch encodings, one hardware loop and all
    integer/GP addressing instructions from Hexagon source. Its v65 backend
    lacks the v66 privileged `memw_phys` instruction, so one `nop` sentinel
    marks its slot. We validate that single sentinel before substituting the
    checked v66 ISA field encoder, not a reference byte sequence.
    """
    from adsp_hexagon_v66 import physical_word_load

    compiled = bytearray(assemble(llvm_mc, source, 0x4c))
    offset = 0x4a44 - 0x4a08
    placeholder = struct.pack("<I", 0x7F00C000)
    if compiled[offset:offset + 4] != placeholder:
        raise AdspError("Expected one unsupported physical-load sentinel at 0x4a44")
    compiled[offset:offset + 4] = struct.pack("<I", physical_word_load(5, 0, 4))
    if any(compiled[i:i + 4] == placeholder for i in range(0, len(compiled), 4)):
        raise AdspError("Unresolved physical-load placeholder")
    return bytes(compiled)


def assemble_physical_compare(
    llvm_mc: str, source: pathlib.Path = CONTROLLER_PHYSICAL_COMPARE_SOURCE
) -> bytes:
    """Assemble physical word comparison and its packetized endloop.

    LLVM6 assembles all source-level branches, register ops, extender values
    and hardware-loop packet boundaries. The single v66 privileged physical
    load is encoded by a 3-register checked ISA constructor after the exact
    one-word NOP sentinel is confirmed. A genuine *middle-of-packet* NOP at
    0x4ad8 remains LLVM-assembled and is never substituted.
    """
    from adsp_hexagon_v66 import physical_word_load

    code = bytearray(assemble(llvm_mc, source, 0x90))
    offset = 0x4ac0 - 0x4a54
    placeholder = struct.pack("<I", 0x7F00C000)
    if code[offset:offset + 4] != placeholder:
        raise AdspError("Expected privileged physical compare load sentinel at 0x4ac0")
    if code[0x4ad8 - 0x4a54:0x4adc - 0x4a54] != struct.pack("<I", 0x7F008000):
        raise AdspError("Physical comparison loop NOP parse flags are wrong")
    code[offset:offset + 4] = struct.pack("<I", physical_word_load(5, 3, 4))
    if any(code[i:i + 4] == placeholder for i in range(0, len(code), 4)):
        raise AdspError("Unresolved physical comparison instruction")
    return bytes(code)


def assemble_cfgbase_helper(
    llvm_mc: str, source: pathlib.Path = CONTROLLER_CFGBASE_SOURCE
) -> bytes:
    """Reconstruct v66 CFGBASE read and physical load in a 36-byte helper.

    LLVM6 assembles the extended return-packet GP store and shift operations.
    It cannot parse CFGBASE or physical word accesses, which are represented by
    two verified *temporary* placeholders, then generated from typed ISA
    register fields. The GP store uses the new-value of r2 after its shift.
    """
    from adsp_hexagon_v66 import SystemRegister, read_system_register, physical_word_load

    result = bytearray(assemble(llvm_mc, source, 0x24))
    nop = struct.pack("<I", 0x7F00C000)
    for offset, operation in (
        (0x00, read_system_register(1, SystemRegister.CFGBASE)),
        (0x10, physical_word_load(2, 0, 1)),
    ):
        if result[offset:offset + 4] != nop:
            raise AdspError(f"CFGBASE helper slot 0x{offset:x} was not its placeholder")
        result[offset:offset + 4] = struct.pack("<I", operation)
    if any(result[i:i+4] == nop for i in range(0,len(result),4)):
        raise AdspError("Unresolved privileged instruction in CFGBASE helper")
    return bytes(result)


def assemble_controller_startup(
    llvm_mc: str, source: pathlib.Path = CONTROLLER_STARTUP_SOURCE
) -> bytes:
    """Assemble control-register save and GP-relative initialization.

    Four privileged register access instructions are encoded from their
    register identifiers and operands, with exact ELF packet size/positions
    verified. The remaining 22 instruction words are assembled from Hexagon
    labels and operations, including a branch beyond the generated fragment.
    """
    from adsp_hexagon_v66 import SystemRegister, read_system_register, write_system_register

    output = bytearray(assemble(llvm_mc, source, 0x68))
    nop = struct.pack("<I", 0x7F00C000)
    replacements = (
        (0x00, read_system_register(23, SystemRegister.SYSCFG)),
        (0x04, write_system_register(SystemRegister.PMUCFG, 23)),
        (0x0C, write_system_register(SystemRegister.SYSCFG, 0)),
        (0x14, read_system_register(23, SystemRegister.S54)),
    )
    for position, operand in replacements:
        if output[position:position + 4] != nop:
            raise AdspError(f"Missing controller startup sentinel at 0x{position:x}")
        output[position:position + 4] = struct.pack("<I", operand)
    if any(output[i:i+4] == nop for i in range(0,len(output),4)):
        raise AdspError("Unresolved control register placeholder in startup")
    return bytes(output)


def assemble_controller_debug_check(
    llvm_mc: str, source: pathlib.Path = CONTROLLER_DEBUG_CHECK_SOURCE
) -> bytes:
    """Rebuild v66 debug protection and controller physical-value check.

    All branches, extenders, insertions, comparisons and packet structure
    are assembled independently. Six architecturally unsupported v66 register
    instructions are placed in narrowly checked packet-aware sentinel slots.
    """
    from adsp_hexagon_v66 import (
        SystemRegister, DebugCommand, read_system_register,
        write_system_register, physical_word_load, packet_continuation,
    )

    result = bytearray(assemble(llvm_mc, source, 0x58))
    packets = (
        (0x00, 0x7F00C000, read_system_register(1, SystemRegister.ISDBEN)),
        (0x0C, 0x7F004000, packet_continuation(
            read_system_register(1, SystemRegister.CFGBASE))),
        (0x20, 0x7F00C000, physical_word_load(1, 0, 1)),
        (0x34, 0x7F00C000, physical_word_load(1, 0, 1)),
        (0x4C, 0x7F00C000, write_system_register(SystemRegister.ISDBEN, 1)),
        (0x54, 0x7F00C000, int(DebugCommand.BREAKPOINT)),
    )
    for offset, placeholder, instruction in packets:
        if result[offset:offset+4] != struct.pack("<I",placeholder):
            raise AdspError(f"Expected debug control sentinel at 0x{offset:x}")
        result[offset:offset+4] = struct.pack("<I", instruction)
    if any(result[i:i+4] == struct.pack("<I",0x7F00C000)
           for i in range(0,len(result),4)):
        raise AdspError("Unresolved debug privilege instruction")
    return bytes(result)


def assemble_vector_handoff(
    llvm_mc: str, source: pathlib.Path = CONTROLLER_VECTOR_HANDOFF_SOURCE
) -> bytes:
    """Reassemble the EVB handoff with source-defined out-of-range targets.

    Only the privileged `EVB = R1` instruction is patched from a checked
    typed system register encoding. LLVM6 emits all branch displacements
    directly from `.set` symbols, with zero relocations.
    """
    from adsp_hexagon_v66 import SystemRegister, write_system_register

    b = bytearray(assemble(llvm_mc, source, 0x28))
    offset = 0x4cc8 - 0x4cc0
    if b[offset:offset+4] != struct.pack("<I", 0x7F00C000):
        raise AdspError("Expected EVB write sentinel at 0x4cc8")
    b[offset:offset+4] = struct.pack("<I", write_system_register(SystemRegister.EVB, 1))
    if b.count(struct.pack("<I",0x7F00C000)):
        raise AdspError("Unresolved EVB placeholder")
    return bytes(b)


def assemble_privileged_restore(
    llvm_mc: str, source: pathlib.Path = CONTROLLER_PRIVILEGED_RESTORE_SOURCE
) -> bytes:
    """Hexagon privileged state restoration for the reset/thread handoff.

    LLVM6 assembles the architectural R23 constant/relative jump and their
    packet parse bits. The seven V66-only instructions use narrow typed
    encoders, replacing exactly seven expected one-word assembler NOPs.
    """
    from adsp_hexagon_v66 import (
        SystemRegister, read_system_register, read_system_register_pair,
        write_predicate_group_from_gpr, start_thread_from_register,
    )
    b = bytearray(assemble(llvm_mc, source, 0x34))
    instructions = (
        (0x00, read_system_register(24, SystemRegister.PMUEVTCFG)),
        (0x04, write_predicate_group_from_gpr(24)),
        (0x08, read_system_register_pair(0, 48)),
        (0x0c, read_system_register_pair(24, 44)),
        (0x10, read_system_register(29, SystemRegister.S47)),
        (0x14, read_system_register_pair(30, 50)),
        (0x20, start_thread_from_register(23)),
    )
    sentinel = struct.pack("<I", 0x7f00c000)
    for offset, instruction in instructions:
        if b[offset:offset+4] != sentinel:
            raise AdspError(f"Expected privileged state sentinel at 0x{offset:x}")
        b[offset:offset+4] = struct.pack("<I", instruction)
    # The last packet includes three *intentional* NOPs. Reject unpatched
    # stand-alone sentinel operations only before the branch packet.
    if sentinel in [b[i:i+4] for i in range(0,0x24,4)]:
        raise AdspError("Unresolved privileged state placeholder")
    return bytes(b)


def run(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--llvm-mc", required=True,
                    help="Path to Hexagon-capable llvm-mc (tested LLVM 6)")
    ap.add_argument("--output", required=True, type=pathlib.Path,
                    help="Destination for source-generated fragment bytes")
    ap.add_argument("--segment", choices=tuple(FRAGMENTS), default="vectors",
                    help="Source-authored ADSP fragment to generate (default: vectors)")
    ap.add_argument("--source", type=pathlib.Path,
                    help="Explicit assembler source override for the selected segment")
    ap.add_argument("--reference", type=pathlib.Path,
                    help="Opt-in Xiaomi Nabu adsp.mbn reference comparison")
    args = ap.parse_args(argv)
    try:
        source, file_offset, expected_length = FRAGMENTS[args.segment]
        if args.segment == "boot-prefix":
            # LLVM6 cannot assemble all V66 system instructions. Combine
            # source-assembled v65/v66 common instructions with checked
            # field-level V66 encodings, never with reference-derived bytes.
            from adsp_hexagon_v66 import boot_special_code
            out = assemble(args.llvm_mc, args.source or source,
                           GENERATED_LENGTH) + boot_special_code()
            if len(out) != expected_length:
                raise AdspError("Invalid combined ADSP boot-prefix size")
        elif args.segment == "syscfg-handoff":
            if args.source is not None:
                raise AdspError("--source override is not supported for the typed SYS_CFG handoff")
            out = assemble_syscfg_handoff(args.llvm_mc)
        elif args.segment == "tlb-map":
            out = assemble_tlb_map(args.llvm_mc, args.source or source)
        elif args.segment == "tlb-flush":
            out = assemble_tlb_flush(args.llvm_mc, args.source or source)
        elif args.segment == "controller-vector-handoff":
            out = assemble_vector_handoff(args.llvm_mc, args.source or source)
        elif args.segment == "controller-privileged-restore":
            out = assemble_privileged_restore(args.llvm_mc, args.source or source)
        elif args.segment == "controller-debug-check":
            out = assemble_controller_debug_check(args.llvm_mc, args.source or source)
        elif args.segment == "controller-startup":
            out = assemble_controller_startup(args.llvm_mc, args.source or source)
        elif args.segment == "controller-cfgbase":
            out = assemble_cfgbase_helper(args.llvm_mc, args.source or source)
        elif args.segment == "controller-physical-compare":
            out = assemble_physical_compare(args.llvm_mc, args.source or source)
        elif args.segment == "controller-physical-copy":
            out = assemble_physical_copy(args.llvm_mc, args.source or source)
        elif args.segment == "tlb-reconcile":
            out = assemble_tlb_reconcile(args.llvm_mc, args.source or source)
        elif args.segment == "counter-controller":
            # Compile the ISA-common controller and its V66-only tails
            # independently, then emit a strictly adjacent 0xc3c-byte span (including explicit NOP alignment).
            if args.source is not None:
                raise AdspError("--source override is not supported for the combined controller")
            out = (assemble(args.llvm_mc, CONTROLLER_SOURCE, 0x4c0) +
                   assemble_syscfg_handoff(args.llvm_mc) +
                   assemble_tlb_map(args.llvm_mc) +
                   assemble_tlb_flush(args.llvm_mc) +
                   assemble(args.llvm_mc, CACHELINE_COPY_SOURCE, 0x78) +
                   assemble(args.llvm_mc, MEMORY_COPY_SOURCE, 0x40) +
                   assemble_tlb_reconcile(args.llvm_mc) +
                   assemble(args.llvm_mc, CONTROLLER_BOOTSTRAP_SOURCE, 0x2c) +
                   assemble(args.llvm_mc, CONTROLLER_CACHE_COPY_SOURCE, 0x5c) +
                   assemble_physical_copy(args.llvm_mc) +
                   assemble_physical_compare(args.llvm_mc) +
                   assemble_cfgbase_helper(args.llvm_mc) +
                   assemble(args.llvm_mc, CONTROLLER_ALIGNMENT_SOURCE, 0xf8) +
                   assemble_controller_startup(args.llvm_mc) +
                   assemble_controller_debug_check(args.llvm_mc) +
                   assemble_vector_handoff(args.llvm_mc) +
                   assemble_privileged_restore(args.llvm_mc) +
                   assemble(args.llvm_mc, CONTROLLER_RESTORE_PADDING_SOURCE, 0x100))
        else:
            out = assemble(args.llvm_mc, args.source or source, expected_length)
        if len(out) != expected_length:
            raise AdspError("Invalid reconstructed fragment length")
        if args.reference:
            verify_reference(out, args.reference.read_bytes(), file_offset)
        args.output.write_bytes(out)
        print(
            f"ADSP source-generated {args.segment}: {len(out)} bytes, "
            f"file offset [0x{file_offset:x},0x{file_offset+len(out):x}), "
            f"paddr [0x{OEM_PHYSICAL_ENTRY+file_offset-OEM_FILE_OFFSET:x},"
            f"0x{OEM_PHYSICAL_ENTRY+file_offset-OEM_FILE_OFFSET+len(out):x}), "
            f"sha256={hashlib.sha256(out).hexdigest()}"
        )
        if args.reference:
            print("Verified against exact Nabu ADSP reference: zero byte differences")
        print("INCOMPLETE: all missing intervals remain unimplemented; "
              "these disjoint fragments are not an ADSP firmware image")
        return 0
    except (OSError, AdspError) as err:
        print(f"ADSP reconstruction failed: {err}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(run())
