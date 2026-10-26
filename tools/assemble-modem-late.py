#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build two independent Hexagon v60 modem *later segment* function groups.

The generated fragments are source-derived, not copied from the OEM firmware.
The original reference is only needed for optional SHA-pinned verification.
All offsets below are ELF file offsets; virtual and physical addresses differ.
"""
from __future__ import annotations

import argparse
import hashlib
from pathlib import Path
import struct
import subprocess
import sys
import tempfile

EXPECTED_REFERENCE_SHA256 = "6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782"
ROOT = Path(__file__).resolve().parents[1]
ATOMIC_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_atomic.s"
AUX_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_boot.s"
CONTROL_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_control.s"
AUX_INIT_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_init.s"
AUX_GUARD_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_guard.s"
AUX_STAGE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_stage.s"
AUX_MMIO_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_mmio.s"
SERVICE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_service_probe.s"
SERVICE_MMIO_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_service_mmio.s"
SERVICE_FLAGS_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_service_flags.s"
SERVICE_ALLOC_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_service_alloc.s"
AUX_HEAP_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_heap.s"
AUX_WORKER_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_worker.s"
AUX_LOOKUP_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_registry_lookup.s"
AUX_REGISTRY_CORE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_registry_core.s"
AUX_NOTIFY_SCAN_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_notify_callbacks.s"
AUX_CLEANUP_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_cleanup.s"
AUX_FACTORY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_object_factory.s"
AUX_FIND_FORMAT_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_find_format.s"
AUX_FIND_CLASS_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_find_codec_class.s"
AUX_CONTROLLER_SLOT_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_controller_slot.s"
AUX_LARGE_CONTROLLER_PREFIX_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_prefix.s"
AUX_LARGE_CONTROLLER_MID_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_mid.s"
AUX_LARGE_CONTROLLER_PHASE2_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase2.s"
AUX_LARGE_CONTROLLER_PHASE3_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase3.s"
AUX_LARGE_CONTROLLER_PHASE4_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase4.s"
AUX_LARGE_CONTROLLER_PHASE5_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase5.s"
AUX_DISPATCH_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_dispatch_helpers.s"
AUX_TYPE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_type_config.s"
AUX_NOTIFY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_controller_notify.s"
AUX_ENTRY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_entry.s"
AUX_VALIDATOR_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_input_validator.s"
AUX_RUNTIME_INIT_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_runtime_init.s"
AUX_VENEERS_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_control_veneers.s"
AUX_REGISTRATION_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_controller_registration.s"
AUX_DIAGNOSTIC_HELPERS_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_diagnostic_helpers.s"
AUX_SESSION_PREFIX_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_session_prefix.s"
AUX_SESSION_VALIDATION_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_session_validation.s"
AUX_FIND_FREE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_find_free_entry.s"
AUX_RESOLVE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_resolve_handle.s"
AUX_SESSION_REGISTRY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_session_registry_insert.s"
AUX_INSERT_CONTROLLER_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_insert_controller.s"
AUX_REMOVE_CONTROLLER_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_remove_controller.s"
AUX_CALLBACK_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_register_callback.s"
AUX_FIND_CONTROLLER_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_find_controller.s"
AUX_RELEASE_REGISTRY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_release_registry.s"
ATOMIC_START = 0x7C000
ATOMIC_END = 0x7C040
SERVICE_START = 0x7C2EC
SERVICE_END = 0x7C324
SERVICE_MMIO_START = 0x7C324
SERVICE_MMIO_END = 0x7C3A8
SERVICE_FLAGS_START = 0x7C3A8
SERVICE_FLAGS_END = 0x7C40C
SERVICE_ALLOC_START = 0x7C40C
SERVICE_ALLOC_END = 0x7C4B0
CONTROL_START = 0x7C040
CONTROL_END = 0x7C0A0
AUX_START = 0x446000
AUX_END = 0x4460D8
AUX_DYNAMIC_START = AUX_END  # source-derived ELF interpreter/dynsym/dynstr/SysV hash
AUX_DYNAMIC_END = 0x446200
AUX_INIT_START = 0x446200
AUX_INIT_END = 0x446254
AUX_INIT_ALIGN_START = AUX_INIT_END  # checked file alignment, not executable instructions
AUX_INIT_ALIGN_END = 0x446260
AUX_GUARD_START = 0x446260
AUX_GUARD_END = 0x446278
AUX_STAGE_START = 0x446278
AUX_STAGE_END = 0x4462E0
AUX_MMIO_START = 0x4462E0
AUX_MMIO_END = 0x4462F8
AUX_HEAP_START = 0x4462F8
AUX_HEAP_END = 0x4463B4
AUX_WORKER_START = 0x4463B4
AUX_WORKER_END = 0x446420
AUX_LOOKUP_START = 0x446420
AUX_LOOKUP_END = 0x446488
AUX_REGISTRY_CORE_START = 0x446488
AUX_REGISTRY_CORE_END = 0x446650
AUX_DISPATCH_START = 0x446650
AUX_DISPATCH_END = 0x446694
AUX_TYPE_START = 0x446694
AUX_TYPE_END = 0x4467A0
AUX_NOTIFY_START = 0x4467A0
AUX_NOTIFY_END = 0x4467F0
AUX_ENTRY_START = 0x4467F0
AUX_ENTRY_END = 0x446804
AUX_VALIDATOR_START = 0x446804
AUX_VALIDATOR_END = 0x446A68
AUX_RUNTIME_INIT_START = 0x446A68
AUX_RUNTIME_INIT_END = 0x446BDC
AUX_VENEERS_START = 0x446BDC
AUX_VENEERS_END = 0x446C18
AUX_REGISTRATION_START = 0x446C18
AUX_REGISTRATION_END = 0x446E1C
AUX_DIAGNOSTIC_HELPERS_START = 0x446E1C
AUX_DIAGNOSTIC_HELPERS_END = 0x446E90
AUX_SESSION_PREFIX_START = 0x446E90
AUX_SESSION_PREFIX_END = 0x446EE8
AUX_SESSION_VALIDATION_START = 0x446EE8
AUX_SESSION_VALIDATION_END = 0x447028
AUX_FIND_FREE_START = 0x447028
AUX_FIND_FREE_END = 0x447078
AUX_RESOLVE_START = 0x447078
AUX_RESOLVE_END = 0x447100
AUX_SESSION_REGISTRY_START = 0x447100
AUX_SESSION_REGISTRY_END = 0x447234
AUX_INSERT_CONTROLLER_START = 0x447234
AUX_INSERT_CONTROLLER_END = 0x4472D0
AUX_REMOVE_CONTROLLER_START = 0x4472D0
AUX_REMOVE_CONTROLLER_END = 0x447324
AUX_CALLBACK_START = 0x447324
AUX_CALLBACK_END = 0x447388
AUX_FIND_CONTROLLER_START = 0x447388
AUX_FIND_CONTROLLER_END = 0x447410
AUX_RELEASE_REGISTRY_START = 0x447410
AUX_RELEASE_REGISTRY_END = 0x447448
AUX_NOTIFY_SCAN_START = AUX_RELEASE_REGISTRY_END
AUX_NOTIFY_SCAN_END = 0x4474B0
AUX_CLEANUP_START = AUX_NOTIFY_SCAN_END
AUX_CLEANUP_END = 0x4475C0
AUX_FACTORY_START = AUX_CLEANUP_END
AUX_FACTORY_END = 0x447608
AUX_FIND_FORMAT_START = AUX_FACTORY_END
AUX_FIND_FORMAT_END = 0x447664
AUX_FIND_CLASS_START = AUX_FIND_FORMAT_END
AUX_FIND_CLASS_END = 0x4476B8
AUX_CONTROLLER_SLOT_START = AUX_FIND_CLASS_END
AUX_CONTROLLER_SLOT_END = 0x4476BC
AUX_LARGE_CONTROLLER_PREFIX_START = AUX_CONTROLLER_SLOT_END
AUX_LARGE_CONTROLLER_PREFIX_END = 0x447724
AUX_LARGE_CONTROLLER_MID_START = AUX_LARGE_CONTROLLER_PREFIX_END
AUX_LARGE_CONTROLLER_MID_END = 0x4477FC
AUX_LARGE_CONTROLLER_PHASE2_START = AUX_LARGE_CONTROLLER_MID_END
AUX_LARGE_CONTROLLER_PHASE2_END = 0x4478A4
AUX_LARGE_CONTROLLER_PHASE3_START = AUX_LARGE_CONTROLLER_PHASE2_END
AUX_LARGE_CONTROLLER_PHASE3_END = 0x44795C
AUX_LARGE_CONTROLLER_PHASE4_START = AUX_LARGE_CONTROLLER_PHASE3_END
AUX_LARGE_CONTROLLER_PHASE4_END = 0x4479A8
AUX_LARGE_CONTROLLER_PHASE5_START = AUX_LARGE_CONTROLLER_PHASE4_END
AUX_LARGE_CONTROLLER_PHASE5_END = 0x4479D0
MARKER_MASK = 3 << 14
CONTINUE_MARKER = 1 << 14
END_MARKER = 3 << 14


def extract_text(obj: bytes) -> bytes:
    """Return exactly one executable Hexagon ELF32 little-endian PROGBITS."""
    if not obj.startswith(b"\x7fELF\x01\x01") or len(obj) < 52:
        raise ValueError("expected little-endian ELF32 assembler object")
    machine = struct.unpack_from("<H", obj, 18)[0]
    if machine != 164:
        raise ValueError(f"assembler object e_machine={machine}, expected Hexagon 164")
    shoff = struct.unpack_from("<I", obj, 32)[0]
    shentsize, shnum = struct.unpack_from("<HH", obj, 46)
    if shentsize != 40 or not shnum or shoff + shnum * shentsize > len(obj):
        raise ValueError("invalid assembler section directory")
    sections = [struct.unpack_from("<10I", obj, shoff + i * shentsize) for i in range(shnum)]
    text = [s for s in sections if s[1] == 1 and (s[2] & 4) and s[5]]
    if len(text) != 1:
        raise ValueError(f"expected one nonempty executable PROGBITS section, got {len(text)}")
    start, length = text[0][4:6]
    if start + length > len(obj):
        raise ValueError("assembler .text section exceeds object file")
    return obj[start : start + length]


# Auxiliary ELF32 symbol metadata is emitted from *its meaning*, not by
# pasting a reference slice. Hexagon executable code is assembled separately.
# ELF32_SYM fields: symbol, virtual address, size, type/binding, shndx.
AUX_EXPORTS = (
    ("dlsym",       0x1E400E90, 0x198, 2, 7),
    ("dladdr",      0x1E401100, 0x134, 2, 7),
    ("dlerror",     0x1E400650, 0x00C, 2, 7),
    ("dlopen",      0x1E400C18, 0x204, 2, 7),
    ("dlclose",     0x1E4003B4, 0x06C, 2, 7),
    ("dlinfo",      0x1E400694, 0x100, 2, 7),
    ("_rtld_debug", 0x1E419F1C, 0x010, 1, 15),
)


def elf32_sysv_hash(name: str) -> int:
    """ELF SysV .hash algorithm (ELF ABI), using ASCII symbol names."""
    result = 0
    for c in name.encode("ascii"):
        result = (result << 4) + c
        high = result & 0xF0000000
        if high:
            result ^= high >> 24
        result &= ~high
    return result & 0xFFFFFFFF


def make_elf32_sysv_hash(export_names: tuple[str, ...], buckets: int = 8) -> bytes:
    """ELF32 SysV hash: two sizes, buckets, then collision-linked chains."""
    if not export_names or len(set(export_names)) != len(export_names):
        raise ValueError("ELF hash requires distinct symbols")
    if not (1 <= buckets <= 65535):
        raise ValueError("ELF hash bucket count must be in 1..65535")
    chain = [0] * (len(export_names) + 1)  # zero ELF symbol is STN_UNDEF
    heads = [0] * buckets
    for index, name in enumerate(export_names, 1):
        bucket = elf32_sysv_hash(name) % buckets
        chain[index] = heads[bucket]
        heads[bucket] = index
    return struct.pack("<" + "I" * (2 + buckets + len(chain)),
                       buckets, len(chain), *heads, *chain)


def emit_auxiliary_elf_symbols() -> bytes:
    """Produce 296 source-authored ELF metadata bytes for auxiliary library.

    Layout: 36-byte interpreter string/padding; seven ELF32 Sym entries,
    56-byte NUL-prefixed name table, 8-byte hash alignment, 72-byte ELF SysV
    hash table, and 16-byte ending alignment. No executable code is copied.
    """
    interp = b"/usr/lib/libc.so.1\0"
    if len(interp) > 0x24:
        raise ValueError("auxiliary ELF interpreter name overflows reserved field")
    out = bytearray(interp.ljust(0x24, b"\0"))
    names = bytearray(b"\0")
    records=[]
    for name, address, size, kind, shndx in AUX_EXPORTS:
        string_offset = len(names)
        names.extend(name.encode("ascii") + b"\0")
        if not (0 < string_offset < 0x10000 and 0 <= address <= 0xFFFFFFFF
                and 0 < size <= 0xFFFFFFFF and kind in (1, 2)
                and shndx in (7, 15)):
            raise ValueError("invalid auxiliary ELF32 symbol value")
        records.append(struct.pack("<IIIBBH", string_offset, address, size,
                                   (1 << 4) | kind, 0, shndx))
    out.extend(b"".join(records))
    out.extend(names)
    while len(out) % 8:
        out.append(0)  # ELF hash table 8-byte alignment, not an instruction
    out.extend(make_elf32_sysv_hash(tuple(x[0] for x in AUX_EXPORTS)))
    out.extend(b"\0" * 16)  # file-backed linker section alignment
    if len(out) != AUX_DYNAMIC_END - AUX_DYNAMIC_START:
        raise ValueError(f"auxiliary ELF32 metadata must be {AUX_DYNAMIC_END - AUX_DYNAMIC_START} bytes, got {len(out)}")
    return bytes(out)


def emit_auxiliary_code_alignment() -> bytes:
    """Twelve file-backed zero bytes after init; not Hexagon opcodes."""
    if AUX_INIT_ALIGN_START != AUX_INIT_END or AUX_INIT_ALIGN_END-AUX_INIT_END != 12:
        raise ValueError("auxiliary init linker alignment changed")
    return bytes(AUX_INIT_ALIGN_END-AUX_INIT_END)


def restore_atomic_packet_order(assembled: bytes) -> bytes:
    """Correct LLVM6's legal, but non-byte-identical scheduling at 0x7c010.

    Original packet source states the predicated ADD *before* the compound
    `cmp.eq; if(p0.new)jump`. LLVM6 places that compound instruction first.
    Class, marker and register checks reject changes before restoring only
    the original source order and packet-end marker bits. No reference opcode
    bytes are inserted here.
    """
    if len(assembled) != ATOMIC_END - ATOMIC_START or len(assembled) % 4:
        raise ValueError(f"late modem atomics require {ATOMIC_END - ATOMIC_START} bytes")
    words = list(struct.unpack("<" + "I" * (len(assembled) // 4), assembled))
    branch, predicated_add = words[4:6]
    if branch >> 24 != 0x10 or predicated_add >> 24 != 0x74:
        raise ValueError("expected compound comparison-branch followed by predicated ADD")
    if branch & MARKER_MASK != CONTINUE_MARKER or predicated_add & MARKER_MASK != END_MARKER:
        raise ValueError("unexpected Hexagon packet parse markers at modem 0x7c010")
    words[4:6] = [(predicated_add & ~MARKER_MASK) | CONTINUE_MARKER,
                  (branch & ~MARKER_MASK) | END_MARKER]
    return struct.pack("<" + "I" * len(words), *words)


def restore_controller_packet_order(assembled: bytes) -> bytes:
    """Restore two compare+immediate packets and a three-word control packet.

    LLVM6 schedulers move new-value branch consumers ahead of the preceding
    extended immediate/assignment and terminate the nearby parameter move
    prematurely. Each rewrite is checked against documented Hexagon opcode
    *classes* and parse markers; no OEM opcode data is used.
    """
    if len(assembled) != CONTROL_END - CONTROL_START:
        raise ValueError("late modem controller requires 96 source-built bytes")
    words = list(struct.unpack("<" + "I" * (len(assembled)//4),assembled))
    for first, value_class in [(0x14//4, 0x78), (0x4c//4, 0x7e)]:
        pred, extender, value = words[first:first+3]
        if [w >> 24 for w in (pred,extender,value)] != [0x10,0x0c,value_class]:
            raise ValueError(f"controller conditional/extended packet classes changed at {CONTROL_START + first*4:#x}")
        if [w & MARKER_MASK for w in (pred,extender,value)] != [CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
            raise ValueError(f"controller conditional/extended packet markers changed at {CONTROL_START + first*4:#x}")
        words[first:first+3] = [extender,
                               (value & ~MARKER_MASK) | CONTINUE_MARKER,
                               (pred & ~MARKER_MASK) | END_MARKER]
    # With a valid *source-assembled* three-word sequence, change only the
    # MOV's end-of-packet to continue so it shares the following duplex
    # memory-load/immediate instruction, as the original producer ordering.
    extender, move, duplex = words[0x2c//4:0x38//4]
    if [w >> 24 for w in (extender,move,duplex)] != [0x0c,0x78,0x48]:
        raise ValueError("controller extended constant and duplex instruction classes changed")
    if [w & MARKER_MASK for w in (extender,move,duplex)] != [CONTINUE_MARKER,END_MARKER,0]:
        raise ValueError("unexpected controller param-load packet markers")
    words[0x30//4]=(move & ~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<"+"I"*len(words),*words)


def restore_aux_init_packet(assembled: bytes) -> bytes:
    """Join one scalar R2 init with its immediately following duplex packet.

    LLVM6 ends the scalar instruction packet; the architectural source
    specifies the same three zero register writes in one packet. The duplex
    uses the Hexagon 00 parse form, so only the scalar parse marker changes.
    """
    if len(assembled) != AUX_INIT_END - AUX_INIT_START:
        raise ValueError("modem auxiliary initializer must be 84 bytes")
    words = list(struct.unpack("<" + "I" * (len(assembled) // 4), assembled))
    scalar, duplex = words[0x18//4 : 0x20//4]
    if scalar >> 24 != 0x78 or duplex >> 24 != 0x28:
        raise ValueError("auxiliary startup scalar/duplex instruction class mismatch")
    if (scalar & MARKER_MASK) != END_MARKER or (duplex & MARKER_MASK) != 0:
        raise ValueError("auxiliary startup packet marker mismatch")
    words[0x18//4] = (scalar & ~MARKER_MASK) | CONTINUE_MARKER
    return struct.pack("<" + "I" * len(words), *words)


def restore_aux_stage_packets(assembled: bytes) -> bytes:
    """Re-create checked Hexagon packet grouping for auxiliary resource setup.

    LLVM6 disallows two branch-class instructions in the same packet and
    terminates three other packets earlier than the original target compiler.
    Each source-assembled instruction has matching semantics and relative
    displacement. Recombine only parse-marker bits, after checking classes.
    """
    if len(assembled) != AUX_STAGE_END - AUX_STAGE_START:
        raise ValueError("modem auxiliary stage must compile to 104 bytes")
    words = list(struct.unpack("<"+"I"*(len(assembled)//4),assembled))
    rewrites = [
        (0x1c, 0xB0, END_MARKER, CONTINUE_MARKER),  # scalar frame-relative pointer
        (0x28, 0x3C, END_MARKER, CONTINUE_MARKER),  # first extended MMIO write
        (0x44, 0x5A, END_MARKER, CONTINUE_MARKER),  # call and immediately extended args
        (0x5c, 0x5A, CONTINUE_MARKER, END_MARKER),  # stand-alone worker call
        (0x60, 0x78, END_MARKER, CONTINUE_MARKER),  # r0 clears before return
    ]
    for off, opcode, old_marker, new_marker in rewrites:
        index=off//4
        original=words[index]
        if original>>24 != opcode or original & MARKER_MASK != old_marker:
            raise ValueError(f"auxiliary stage instruction class or packet marker changed at {AUX_STAGE_START+off:#x}")
        words[index]=(original & ~MARKER_MASK)|new_marker
    if words[0x20//4]>>24 != 0x28 or words[0x20//4]&MARKER_MASK != 0:
        raise ValueError("auxiliary stage expected a valid final duplex instruction")
    return struct.pack("<"+"I"*len(words),*words)


def restore_service_probe_packets(assembled: bytes) -> bytes:
    """Preserve two source-semantic instruction packets combined by target compiler.

    LLVM6 prematurely ends a call before an extended absolute register load,
    and an absolute register load before another extender. The target compiler
    uses two longer legal packets. Refuse unexpected instruction types/parse
    fields and alter *only* the two terminating Hexagon parse bits.
    """
    if len(assembled) != SERVICE_END - SERVICE_START:
        raise ValueError("modem late service probe requires 56 compiled bytes")
    words=list(struct.unpack("<"+"I"*(len(assembled)//4),assembled))
    for at,opcode,following in [(0x14,0x5a,0x0c),(0x24,0x78,0x0c)]:
        i=at//4
        if (words[i]>>24,words[i+1]>>24)!=(opcode,following):
            raise ValueError(f"late modem service probe instruction classes changed at {SERVICE_START+at:#x}")
        if (words[i]&MARKER_MASK,words[i+1]&MARKER_MASK)!=(END_MARKER,CONTINUE_MARKER):
            raise ValueError(f"late modem service probe packet markers changed at {SERVICE_START+at:#x}")
        words[i]=(words[i]&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<"+"I"*len(words),*words)


def restore_service_mmio_packets(assembled: bytes) -> bytes:
    """Join two controller calls to the following immediate-extended move.

    The original compiler groups calls with the next constant-extended
    address declaration. LLVM6 terminates the calls separately; check both
    classes and markers then continue the packet without touching opcodes.
    """
    if len(assembled) != SERVICE_MMIO_END-SERVICE_MMIO_START:
        raise ValueError("late modem service MMIO requires 132 bytes")
    words=list(struct.unpack("<"+"I"*(len(assembled)//4),assembled))
    for off,opcode in [(0x14,0x5A),(0x20,0x5B)]:
        i=off//4
        if [words[i]>>24,words[i+1]>>24]!=[opcode,0x0c]:
            raise ValueError(f"MMIO service call/extender opcode classes changed at {SERVICE_MMIO_START+off:#x}")
        if [words[i]&MARKER_MASK,words[i+1]&MARKER_MASK]!=[END_MARKER,CONTINUE_MARKER]:
            raise ValueError(f"MMIO service packet markers changed at {SERVICE_MMIO_START+off:#x}")
        words[i]=(words[i]&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<"+"I"*len(words),*words)


def restore_service_flag_packets(assembled: bytes) -> bytes:
    """Preserve a producer/consumer Hexagon V60 packet for flag access.

    LLVM6 schedules the compound compare/branch ahead of its two-slot
    predicated register-assignment and constant extender. The original
    program instead orders extender, predicated move, comparison branch.
    Reorder only those three independently assembled instructions and set
    legal packet markers after checking opcode classes and source grouping.
    This NEVER inserts reference words into the generated fragment.
    """
    if len(assembled) != SERVICE_FLAGS_END - SERVICE_FLAGS_START:
        raise ValueError("service flag helpers require exactly 100 source bytes")
    words = list(struct.unpack("<25I", assembled))
    i = (0x7C3E8 - SERVICE_FLAGS_START) // 4
    cmp_jump, extender, predicated = words[i:i+3]
    if [x >> 24 for x in (cmp_jump, extender, predicated)] != [0x10, 0x0e, 0x7e]:
        raise ValueError("unexpected Hexagon service flag predicate opcode classes")
    if [x & MARKER_MASK for x in (cmp_jump, extender, predicated)] != [
        CONTINUE_MARKER, CONTINUE_MARKER, END_MARKER
    ]:
        raise ValueError("unexpected Hexagon service flag packet parse markers")
    words[i:i+3] = [extender,
                    (predicated & ~MARKER_MASK) | CONTINUE_MARKER,
                    (cmp_jump & ~MARKER_MASK) | END_MARKER]
    return struct.pack("<25I", *words)


def restore_service_alloc_return_packet(assembled: bytes) -> bytes:
    """Preserve source-ordered return register move in compound frame restore.

    LLVM6 terminates an otherwise valid standalone `r0=r16` packet before
    the following 64-bit frame-load and `dealloc_return`. Target Hexagon V60
    uses the scalar move as a continuation instead. Check both instruction
    classes and source parse markers before changing this *single* field.
    Neither external OEM bytes nor any replacement instruction are used.
    """
    if len(assembled) != SERVICE_ALLOC_END-SERVICE_ALLOC_START:
        raise ValueError("service allocator requires exactly 164 source bytes")
    words = list(struct.unpack("<41I", assembled))
    index=0x98//4
    move, load_pair, ret=words[index:index+3]
    if [x>>24 for x in (move,load_pair,ret)]!=[0x70,0x3e,0x96]:
        raise ValueError("service allocator return instruction classes changed")
    if [x&MARKER_MASK for x in (move,load_pair,ret)]!=[
        END_MARKER,0,END_MARKER
    ]:
        raise ValueError("service allocator return packet markers changed")
    words[index]=(move & ~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<41I", *words)


def restore_aux_heap_predicate_packet(assembled: bytes) -> bytes:
    """Preserve new-register compare/branch source order in heap slot scan.

    LLVM6 hoists the compound new-value compare/branch ahead of the scalar
    `r2=#0` that it tests, but the original target packer put the producer
    first. Validate opcode classes, packet marker fields and 188-byte size,
    then swap exactly the two independently assembled instructions.
    """
    if len(assembled) != AUX_HEAP_END-AUX_HEAP_START:
        raise ValueError("auxiliary heap allocator requires exactly 188 bytes")
    words=list(struct.unpack("<47I",assembled))
    idx=0x6c//4
    branch, zero = words[idx:idx+2]
    if (branch>>24, zero>>24)!=(0x10,0x78):
        raise ValueError("heap new-value branch and producer opcode class mismatch")
    if (branch&MARKER_MASK,zero&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("heap new-value producer packet markers changed")
    words[idx:idx+2]=[(zero&~MARKER_MASK)|CONTINUE_MARKER,
                      (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack("<47I",*words)


def restore_aux_worker_packets(assembled: bytes) -> bytes:
    """Restore original Hexagon packet position and PC-relative jump semantics.

    LLVM6 emits scalar -1 setup as a separate packet even though target
    Hexagon V60 pairs it with the following unconditional jump. The jump
    packet-relative displacement differs by one Hexagon branch unit for the
    target assembler's split packet address convention; retain the named
    `.Ldone` source target, validate the compiler's original field, and
    adjust *only* its immediate field by two encoded steps (8 bytes).
    A subsequent service call is likewise part of the following packet.
    """
    if len(assembled) != AUX_WORKER_END-AUX_WORKER_START:
        raise ValueError("auxiliary worker must assemble into exactly 108 bytes")
    words=list(struct.unpack("<27I",assembled))
    init_idx=0x24//4
    mov,jmp=words[init_idx:init_idx+2]
    if (mov>>24,jmp>>24)!=(0x78,0x58):
        raise ValueError("aux worker const/jump ISA classes changed")
    if (mov & MARKER_MASK,jmp & MARKER_MASK)!=(END_MARKER,END_MARKER):
        raise ValueError("aux worker const/jump packet markers changed")
    if jmp&0x3f != 0x1e:
        raise ValueError("aux worker PC-relative branch immediate no longer matches named target")
    words[init_idx] = (mov & ~MARKER_MASK) | CONTINUE_MARKER
    words[init_idx+1] = (jmp&~0x3f) | ((jmp&0x3f) + 2)
    call_idx=0x3c//4
    call,next_combine=words[call_idx:call_idx+2]
    if (call>>24,next_combine>>24)!=(0x5a,0x73):
        raise ValueError("aux worker controller update/combine instruction classes changed")
    if (call&MARKER_MASK,next_combine&MARKER_MASK)!=(END_MARKER,CONTINUE_MARKER):
        raise ValueError("aux worker controller update packet markers changed")
    words[call_idx] = (call&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<27I",*words)


def restore_aux_lookup_return_packet(assembled: bytes) -> bytes:
    """Rejoin a valid scalar return move with an adjacent duplex restore.

    LLVM6 emits the `r0=r16` instruction in an isolated packet. The original
    controller's return path has a continuation marker before a compound
    64-bit register load and `dealloc_return`. Only this parse marker changes
    after ISA class/length validation; code bytes are source-assembled.
    """
    if len(assembled) != AUX_LOOKUP_END-AUX_LOOKUP_START:
        raise ValueError("aux registry lookup requires exactly 104 source bytes")
    words=list(struct.unpack("<26I",assembled))
    i=0x60//4
    move,stack_restore=words[i:i+2]
    if [v>>24 for v in (move,stack_restore)]!=[0x70,0x3e]:
        raise ValueError("aux registry return opcode classes changed")
    if [v&MARKER_MASK for v in (move,stack_restore)]!=[
        END_MARKER,0
    ]:
        raise ValueError("aux registry return packet marker mismatch")
    words[i]=(move&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<26I",*words)


def restore_aux_type_return_packets(assembled: bytes) -> bytes:
    """Join error-return constants with following Hexagon frame restorations.

    Two codec format branches return -1 after a failed allocation. LLVM6
    marks the immediate move as a complete packet, whereas the original
    target scheduler uses it as a continuation preceding a duplex
    memory-restore/dealloc return. Validate both instruction pairs and
    reconstruct only the parse-marker bits; the three trailing `nop`
    instructions are explicit source and are retained as alignment code.
    """
    if len(assembled) != AUX_TYPE_END-AUX_TYPE_START:
        raise ValueError("aux codec-type routine must produce 268 bytes")
    words=list(struct.unpack('<67I',assembled))
    for at in (0x34,0x80):
        i=at//4
        scalar,restore=words[i:i+2]
        if (scalar>>24,restore>>24)!=(0x78,0x3e):
            raise ValueError(f"aux codec-type early return opcode classes changed at {at:#x}")
        if (scalar&MARKER_MASK,restore&MARKER_MASK)!=(END_MARKER,0):
            raise ValueError(f"aux codec-type early return packet markers changed at {at:#x}")
        words[i]=(scalar&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<67I',*words)


def restore_aux_controller_notify_packets(assembled: bytes) -> bytes:
    """Restore target producer-order packets for controller notification.

    LLVM6 moves the branch consumer before its `p0` comparison and source
    move, and moves the frame reload before its extended MMIO store. The
    target toolchain keeps each three-word group in explicit source order.
    Only ISA-checked instruction ordering and 2-bit parse markers change.
    No private OEM opcode word is read or copied into the generated result.
    """
    if len(assembled)!=AUX_NOTIFY_END-AUX_NOTIFY_START:
        raise ValueError("auxiliary controller notifier must emit exactly 80 bytes")
    words=list(struct.unpack('<20I',assembled))
    groups=[
       (0x10//4,[0x5c,0xf2,0x70]),
       (0x38//4,[0x91,0x01,0x48]),
    ]
    for i,classes in groups:
        seq=words[i:i+3]
        if [x>>24 for x in seq]!=classes:
            raise ValueError(f"aux notifier packet classes differ at offset {i*4:#x}")
        if [x&MARKER_MASK for x in seq]!=[
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER
        ]:
            raise ValueError(f"aux notifier packet marker mismatch at offset {i*4:#x}")
        consumer,first,second=seq
        words[i:i+3]=[first,(second&~MARKER_MASK)|CONTINUE_MARKER,
                      (consumer&~MARKER_MASK)|END_MARKER]
    return struct.pack('<20I',*words)


def restore_aux_validator_predicate_packets(assembled: bytes) -> bytes:
    """Place new-value producer comparisons before their branch consumers.

    LLVM6 packet scheduling sometimes emits the compound predicated branch
    before its generating compare in the same legal V60 packet. The target
    assembler emitted the reverse order, and parse markers signal the same
    one-packet ordering. Check lengths, opcode classes and parse fields
    before rearranging the *independently assembled* operations. No source
    firmware instruction bytes are imported.
    """
    if len(assembled) != AUX_VALIDATOR_END - AUX_VALIDATOR_START:
        raise ValueError("auxiliary validator expects exactly 612 source bytes")
    words=list(struct.unpack('<'+'I'*(len(assembled)//4),assembled))
    for relative in (0x4468F0 - AUX_VALIDATOR_START, 0x446994 - AUX_VALIDATOR_START):
        i=relative//4
        branch, comparison=words[i:i+2]
        if (branch>>24,comparison>>24) != (0x5c,0x75):
            raise ValueError(f"validator predicate instruction classes changed at {AUX_VALIDATOR_START+relative:#x}")
        old=(branch & MARKER_MASK,comparison & MARKER_MASK)
        if relative==0x4468F0-AUX_VALIDATOR_START:
            if old != (CONTINUE_MARKER,CONTINUE_MARKER):
                raise ValueError("validator first predicate packet markers changed")
        elif old!=(CONTINUE_MARKER,END_MARKER):
            raise ValueError("validator second predicate packet markers changed")
        words[i]=(comparison & ~MARKER_MASK)|CONTINUE_MARKER
        words[i+1]=(branch & ~MARKER_MASK)|(END_MARKER if relative==0x446994-AUX_VALIDATOR_START else CONTINUE_MARKER)
    return struct.pack('<'+'I'*len(words),*words)


def restore_aux_runtime_init_packets(assembled: bytes) -> bytes:
    """Preserve producer-first packets and one late controller-state store.

    LLVM6 schedules two single-register parameter producers after their
    predicated uses. It also places an independent memory load before the
    accompanying 32-bit absolute-address write. Validate the original
    SOURCE-assembled operation classes and packet markers, then reorder
    them to the target compiler's legal Hexagon V60 packet order. Each
    rewrite manipulates only assembled instructions and 2-bit parse fields.
    """
    if len(assembled) != AUX_RUNTIME_INIT_END - AUX_RUNTIME_INIT_START:
        raise ValueError("aux runtime initializer requires 372 source bytes")
    words=list(struct.unpack('<'+'I'*(len(assembled)//4),assembled))
    for off,branch_class,param_class in [(0x1c,0x5c,0x7e),(0x50,0x10,0x78)]:
        i=off//4
        branch,parameter=words[i:i+2]
        if (branch>>24,parameter>>24)!=(branch_class,param_class):
            raise ValueError("aux runtime predicate opcode classes changed")
        if (branch&MARKER_MASK,parameter&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
            raise ValueError("aux runtime predicate packet markers changed")
        words[i]=(parameter&~MARKER_MASK)|CONTINUE_MARKER
        words[i+1]=(branch&~MARKER_MASK)|END_MARKER
    i=0xe4//4
    early_load,extender,write=words[i:i+3]
    if (early_load>>24,extender>>24,write>>24)!=(0x91,0x01,0x48):
        raise ValueError("aux runtime MMIO packet opcode classes changed")
    if (early_load&MARKER_MASK,extender&MARKER_MASK,write&MARKER_MASK)!=       (CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError("aux runtime MMIO packet markers changed")
    words[i:i+3]=[extender,(write&~MARKER_MASK)|CONTINUE_MARKER,
                  (early_load&~MARKER_MASK)|END_MARKER]
    return struct.pack('<'+'I'*len(words),*words)


def compact_aux_controller_veneer_duplex(assembled: bytes) -> bytes:
    """Reassemble a scalar move+jump as one documented Hexagon duplex.

    LLVM6 expands the symbolic extended tail call into an immediate
    extender, scalar branch, and separate `r3=#0` move. Hexagon V60 also
    defines a 32-bit duplex encoding of `r3=#0 ; jump`, selected by the
    target compiler. Validate every instruction class, register and marker
    before computing the duplex from the SOURCE-assembled operand fields.
    The following tail-call packet is regrouped only by its parse markers.
    """
    if len(assembled) != AUX_VENEERS_END-AUX_VENEERS_START + 4:
        raise ValueError("auxiliary veneers expect 64 source-assembled bytes")
    words=list(struct.unpack('<'+'I'*(len(assembled)//4),assembled))
    # Source offset 0x24: extender, 0x28: scalar branch, 0x2c: scalar r3=#0.
    a=0x24//4
    extender,jump,move=words[a:a+3]
    if extender>>24 != 0x0f or jump>>24 != 0x58 or move>>24 != 0x78:
        raise ValueError("auxiliary veneer extender/branch/move opcode classes changed")
    if (extender&MARKER_MASK,jump&MARKER_MASK,move&MARKER_MASK) !=       (CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError("auxiliary veneer scalar packet markers changed")
    # A2_tfrsi r3,#0: register is low nibble, immediate and other bits zero.
    if (move&~MARKER_MASK) != (0x78000000 | 3):
        raise ValueError("auxiliary veneer source is not r3=#0")
    duplex=(0x16<<24)|(3<<16)|((jump&0x3fff)|END_MARKER)
    words[a:a+3]=[extender,duplex]
    # The second tail-call packet starts at output offset 0x34 after the
    # four-byte shrink, with a packed `r3:2=combine(0,0)` and branch.
    b=0x34//4
    branch,pair=words[b:b+2]
    if (branch>>24,pair>>24)!=(0x59,0x7c):
        raise ValueError("auxiliary second veneer opcode classes changed")
    if (branch&MARKER_MASK,pair&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("auxiliary second veneer packet markers changed")
    if (pair&~MARKER_MASK)!=0x7c000002:
        raise ValueError("auxiliary veneer source is not zero pair")
    words[b:b+2]=[(pair&~MARKER_MASK)|CONTINUE_MARKER,
                   (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<'+'I'*len(words),*words)


def restore_aux_registration_new_value_packet(assembled: bytes) -> bytes:
    """Move predicated flag write before its new-value comparison/branch.

    The source explicitly requests the original producer/consumer ordering.
    LLVM6 reschedules the compare before the conditional move, still in one
    packet. Reorder these two *assembler-emitted* instructions and retain
    their position-specific packet markers. No OEM instructions are copied.
    """
    if len(assembled) != AUX_REGISTRATION_END-AUX_REGISTRATION_START:
        raise ValueError("aux registration expects 516 source-assembled bytes")
    words=list(struct.unpack('<'+'I'*(len(assembled)//4),assembled))
    i=(0x446D04-AUX_REGISTRATION_START)//4
    compare,flag_write=words[i:i+2]
    if (compare>>24,flag_write>>24)!=(0x11,0xfd):
        raise ValueError("aux registration predicate opcode classes changed")
    if (compare&MARKER_MASK,flag_write&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("aux registration predicate packet markers changed")
    words[i:i+2]=[(flag_write&~MARKER_MASK)|CONTINUE_MARKER,
                  (compare&~MARKER_MASK)|END_MARKER]
    return struct.pack('<'+'I'*len(words),*words)


def encode_v60_framekey(rd: int, *, end_packet: bool = False) -> int:
    """A32-independent Hexagon V60 ``Rd = framekey`` ISA field encoder.

    Hexagon control-register instruction class 0x6a11 reads the frame-key
    special register; the 5-bit destination field has valid GPRs R1–R7 or
    R16–R23.  LLVM6's MC *disassembler* decodes this class, but its assembler
    does not accept the mnemonic. Build the class + GPR + packet parse bits
    from operands and verify the generated instruction against the reference,
    rather than importing a private OEM opcode word.
    """
    if 1 <= rd <= 7:
        encoded_rd=rd
    elif 16 <= rd <= 23:
        encoded_rd=(rd-16)|0x10
    else:
        raise ValueError("framekey Rd must be R1–R7 or R16–R23")
    # Stable architectural opcode bits, not OEM firmware bytes.
    return 0x6a110000 | encoded_rd | (END_MARKER if end_packet else CONTINUE_MARKER)


def restore_aux_session_validation_packets(assembled: bytes) -> bytes:
    """Reconcile checked Hexagon V60 source packets and encode framekey.

    The first four bytes of the independent LLVM6 source are an *alignment
    placeholder* solely because LLVM6 cannot encode `r3 = framekey` itself.
    Replace the zero placeholder with a public-ISA-encoded control-register
    instruction, then only reorder verified independently assembled packets,
    preserving source operand fields and updating short-branch immediates
    when the packet scheduler moves the source label within the same packet.
    """
    if len(assembled) != AUX_SESSION_VALIDATION_END-AUX_SESSION_VALIDATION_START:
        raise ValueError("session validation expects 320 independently assembled bytes")
    words=list(struct.unpack('<80I',assembled))
    if words[0]!=0:
        raise ValueError("framekey layout slot must be zero before ISA encoding")
    words[0]=encode_v60_framekey(3)

    def reassign_packet(offset, old_classes, old_markers, indexes):
        i=offset//4
        group=words[i:i+len(indexes)]
        if [w>>24 for w in group]!=old_classes:
            raise ValueError(f"session packet ISA opcode classes changed at {offset:#x}")
        if [w&MARKER_MASK for w in group]!=old_markers:
            raise ValueError(f"session packet parser bits changed at {offset:#x}")
        n=len(indexes)
        words[i:i+n]=[(group[source]&~MARKER_MASK)|
                         (END_MARKER if dest==n-1 else CONTINUE_MARKER)
                         for dest,source in enumerate(indexes)]

    # The target compiler puts source value producers ahead of their
    # compare-and-branch consumers. LLVM6 otherwise shuffles same-packet ops.
    reassign_packet(0x58,[0x10,0xfd],[CONTINUE_MARKER,END_MARKER],[1,0])
    reassign_packet(0x78,[0x5c,0x75,0xfd],
                    [CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER],[1,2,0])
    reassign_packet(0x84,[0x5c,0x75],[CONTINUE_MARKER,END_MARKER],[1,0])
    reassign_packet(0xa4,[0x46,0x58],[END_MARKER,END_MARKER],[1,0])
    # At 0xa4 the original jump shares its packet with the conditional store
    # that follows; the LLVM split boundary is corrected after opcode checks.
    words[0xa4//4]=(words[0xa4//4]&~MARKER_MASK)|CONTINUE_MARKER
    for off in (0xac,0xcc):
        i=off//4
        x=words[i]
        if x>>24!=0xb0 or x&MARKER_MASK!=END_MARKER:
            raise ValueError(f"session stack-pointer add/next packet differs at {off:#x}")
        words[i]=(x&~MARKER_MASK)|CONTINUE_MARKER
    i=0x118//4
    if words[i]>>24!=0x50 or words[i]&MARKER_MASK!=END_MARKER:
        raise ValueError("session callr opcode/parse state changed")
    words[i]=(words[i]&~MARKER_MASK)|CONTINUE_MARKER
    i=0x11c//4
    call,extender,packed=words[i:i+3]
    if [x>>24 for x in (call,extender,packed)]!=[0x5b,0x01,0x6b]:
        raise ValueError("session final diagnostic call opcode classes changed")
    if [x&MARKER_MASK for x in (call,extender,packed)]!=[CONTINUE_MARKER,CONTINUE_MARKER,0]:
        raise ValueError("session final diagnostic packet markers changed")
    words[i:i+3]=[extender,packed,call]

    # Adjust only ISA-defined short branch displacement fields.  Packet
    # boundaries from LLVM6 shift these named-target relocations by 2/4
    # encoded units compared with the target compiler's grouping.
    for offset, opcode, before, delta in [
        (0x48,0x16,0x34,+2),
        (0xa4,0x58,0x26,+4),
        (0xfc,0x58,0x1a,+2),
        (0x108,0x24,0x0e,+4),
        (0x124,0x5b,0x0e,-4),
    ]:
        i=offset//4
        x=words[i]
        if x>>24!=opcode or x&0x3f!=before:
            raise ValueError(f"session packet branch opcode/PC-relative target differs at {offset:#x}")
        if x&MARKER_MASK not in (CONTINUE_MARKER,END_MARKER):
            raise ValueError(f"session branch parser bits changed at {offset:#x}")
        words[i]=(x&~0x3f)|(before+delta)
    return struct.pack('<80I',*words)


def restore_aux_registry_core_packets(assembled: bytes) -> bytes:
    """Match V60 source producer-order packets and symbolic loop branches.

    The registry core uses new predicates and absolute MMIO writes. LLVM6
    groups certain equivalent instructions in a different legal order from
    the target producer. Perform only class/marker-checked reorderings and
    well-defined PC-relative short-branch adjustments; never substitute OEM
    opcode bytes or rely on the original binary during emission.
    """
    length=AUX_REGISTRY_CORE_END-AUX_REGISTRY_CORE_START
    if len(assembled)!=length:
        raise ValueError("aux registry core requires 456 source-assembled bytes")
    words=list(struct.unpack('<114I',assembled))
    a=0x10//4
    predicate,producer=words[a:a+2]
    if [w>>24 for w in (predicate,producer)] != [0x10,0x74]:
        raise ValueError("registry core conditional jump/producer ISA classes changed")
    if [w&MARKER_MASK for w in (predicate,producer)]!=[CONTINUE_MARKER]*2:
        raise ValueError("registry core initial predicate parse markers changed")
    words[a:a+2]=[producer,predicate]

    i=0x108//4
    branch,extender,predmove=words[i:i+3]
    if [w>>24 for w in (branch,extender,predmove)]!=[0x5c,0x01,0x7e]:
        raise ValueError("registry core extended predicated branch classes changed")
    if [w&MARKER_MASK for w in (branch,extender,predmove)]!=[
        CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("registry core extended predicated packet markers changed")
    words[i:i+3]=[extender,(predmove&~MARKER_MASK)|CONTINUE_MARKER,
                  (branch&~MARKER_MASK)|END_MARKER]

    i=0x17c//4
    predload,linkstore=words[i:i+2]
    if [w>>24 for w in (predload,linkstore)]!=[0x47,0xa1]:
        raise ValueError("registry core link update opcode classes changed")
    if [w&MARKER_MASK for w in (predload,linkstore)]!=[
        CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("registry core link update parse markers changed")
    words[i:i+2]=[(linkstore&~MARKER_MASK)|CONTINUE_MARKER,
                  (predload&~MARKER_MASK)|END_MARKER]

    # LLVM6 terminates two independent calls in advance of their next
    # extender/predicate packets. Target generated continued call packets.
    for off in (0x13c,0x18c):
        idx=off//4
        x=words[idx]
        if x>>24!=0x5a or x&MARKER_MASK!=END_MARKER:
            raise ValueError(f"registry core callback packet marker changed at {off:#x}")
        words[idx]=(x&~MARKER_MASK)|CONTINUE_MARKER

    # Branch labels are restored to their original LLVM-independent
    # controller loop heads (Hexagon short branch: 2-byte immediate units).
    for off,opcode,immediate in [(0xd4,0x24,0x12),(0xdc,0x59,0x0c)]:
        idx=off//4
        branch=words[idx]
        if branch>>24!=opcode or branch&MARKER_MASK!=END_MARKER or branch&0x3f!=immediate:
            raise ValueError(f"registry core loop branch class/immediate changed at {off:#x}")
        words[idx]=(branch&~0x3f)|(immediate+4)
    return struct.pack('<114I',*words)


def compact_register_index(register: int) -> int:
    """Hexagon duplex Rx encoding (architectural R0–R7 or R16–R23)."""
    if 0 <= register <= 7:
        return register
    if 16 <= register <= 23:
        return register - 8
    raise ValueError(f"not a compact Hexagon duplex register: R{register}")


def encode_v60_duplex_addi_inc(*, add_dest: int, add_src: int,
                                add_imm: int, inc_dest: int,
                                inc_src: int) -> int:
    """Pack a two-A1-instruction Hexagon V60 duplex (manual §§10.2–10.3).

    Slot 1 is A1 add-immediate; its compact destination Rx identifies the
    source (R0–R15), with upper mapped destination registers R16–R23. Slot 0
    is the A1 increment subinstruction: opcode SA1_inc, Rd, Rs.  For this
    pair the source is R13, destination R21, and the increment is R20 += 1.

    No OEM instruction word is read: the result is built solely from named
    instruction classes, signed immediate and compressed register fields.
    """
    rx = compact_register_index(add_dest)
    if add_src != rx or not 0 <= add_imm <= 31:
        raise ValueError("duplex addi requires mapped Rx and unsigned 5-bit immediate")
    rd = compact_register_index(inc_dest)
    rs = compact_register_index(inc_src)
    add_subinstruction = ((add_imm & 0x1f) << 4) | rx
    inc_subinstruction = 0x1100 | (rs << 4) | rd  # SA1_inc: add(Rs,#1)
    iclass = 3  # A/A duplex, table 10-5 of Hexagon programmer manual.
    return (((iclass >> 1) << 29) | ((iclass & 1) << 13) |
            (add_subinstruction << 16) | inc_subinstruction)


def restore_aux_session_registry_packets(assembled: bytes) -> bytes:
    """Restore source packet order and compact an architected A/A duplex.

    LLVM6 emits two legal full-width scalar ADDs instead of the controller's
    single V60 duplex at file 0x447218.  The short encoding is built from the
    A1 opcode and register fields.  With one word removed, the source labels
    after it shift back four bytes; four forward and one backward branch
    immediates are corrected by the documented two-byte branch units.
    Fail closed if ISA opcode classes, packet markers or target fields change.
    """
    original_size = AUX_SESSION_REGISTRY_END - AUX_SESSION_REGISTRY_START
    if len(assembled) != original_size + 4:
        raise ValueError("controller registry insertion requires 312 independently assembled bytes")
    words=list(struct.unpack('<'+'I'*(len(assembled)//4),assembled))
    i=0x18//4
    predbranch,write=words[i:i+2]
    if (predbranch>>24,write>>24)!=(0x10,0x74):
        raise ValueError("registry conditional branch/predicated move opcode classes changed")
    if [x&MARKER_MASK for x in (predbranch,write)]!=[CONTINUE_MARKER,END_MARKER]:
        raise ValueError("registry conditional branch/predicated move packet markers changed")
    words[i:i+2]=[(write & ~MARKER_MASK)|CONTINUE_MARKER,
                  (predbranch & ~MARKER_MASK)|END_MARKER]

    k=0x118//4
    scalar_add,scalar_inc=words[k:k+2]
    if [x>>24 for x in (scalar_add,scalar_inc)]!=[0xb0,0xb0]:
        raise ValueError("duplex source must be two scalar A2 add instructions")
    if [x&MARKER_MASK for x in (scalar_add,scalar_inc)]!=[CONTINUE_MARKER,END_MARKER]:
        raise ValueError("duplex source packet parse markers changed")
    # The source assembly specifies `r21=add(r13,#16)` followed by
    # `r20=add(r20,#1)`. The ordinary 32-bit A2 destination register
    # index occupies bits 0..4; verify these instead of trusting ordering.
    if scalar_add & 0x1f != 21 or scalar_inc & 0x1f != 20:
        raise ValueError("registry duplex operand destination register mismatch")
    duplex=encode_v60_duplex_addi_inc(add_dest=21,add_src=13,add_imm=16,
                                      inc_dest=20,inc_src=20)
    words[k:k+2]=[duplex]
    if len(words) != original_size//4:
        raise ValueError("registry duplex did not shorten by one word")

    # The assembler originally targeted a label four bytes later than the
    # compacted code. These corrections update only branch immediate fields.
    for offset, opcode_class, delta in [
        (0x50,0x58,-2), (0x90,0x58,-2),
        (0xb4,0x24,-2), (0x114,0x25,-2),
        (0x120,0x21,+2),
    ]:
        idx=offset//4
        branch=words[idx]
        if (branch>>24)!=opcode_class or branch&MARKER_MASK!=END_MARKER:
            raise ValueError(f"registry branch ISA class/marker changed at {offset:#x}")
        oldimm=branch&0x3f
        if not 0 <= oldimm + delta < 64:
            raise ValueError("registry branch displacement overflow")
        words[idx]=(branch&~0x3f)|(oldimm+delta)
    return struct.pack('<'+'I'*len(words),*words)


def restore_aux_insert_controller_packets(assembled: bytes) -> bytes:
    """Correct LLVM6 packet order for original source-encoded list insertion.

    The source is written to keep two full-width memory instructions that
    LLVM6 normally packs as shorter duplexes. Only source-assembled opcode
    classes and packet parse bits are changed here; new-value compares are
    kept producer-first, and one independent load/store pair retains original
    instruction order. No bytes from the reference are read or imported.
    """
    if len(assembled)!=AUX_INSERT_CONTROLLER_END-AUX_INSERT_CONTROLLER_START:
        raise ValueError("aux insertion requires 156 compiled source bytes")
    words=list(struct.unpack('<39I',assembled))
    for addr,opcode,expected_source in [
        (0x44723c,0xa1,0xa1dd1200),
        (0x4472c4,0xa1,0xa1830200),
    ]:
        # `expected_source` carries opcode/register/offset/format fields
        # with parse markers masked. These are ISA operand constraints, not
        # reference opcode words; full source encoding is still from LLVM.
        i=(addr-AUX_INSERT_CONTROLLER_START)//4
        x=words[i]
        if x>>24!=opcode or (x&~MARKER_MASK)!=expected_source:
            raise ValueError(f"aux insertion store instruction semantics changed at {addr:#x}")
        if x&MARKER_MASK!=END_MARKER:
            raise ValueError(f"aux insertion full-width store packet marker changed at {addr:#x}")
        words[i]=(x&~MARKER_MASK)|CONTINUE_MARKER
    # Comparison should precede its new-predicate back-edge, rather than
    # LLVM6's equivalent legal reversed order within one packet.
    i=(0x4472a4-AUX_INSERT_CONTROLLER_START)//4
    branch,pred=words[i:i+2]
    if (branch>>24,pred>>24)!=(0x5c,0xf2):
        raise ValueError("aux insertion new-value compare/branch opcode classes changed")
    if (branch&MARKER_MASK,pred&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("aux insertion new-value compare/branch packet markers changed")
    words[i:i+2]=[(pred&~MARKER_MASK)|CONTINUE_MARKER,
                  (branch&~MARKER_MASK)|END_MARKER]
    # Producer store precedes neighboring array read in the original packet.
    i=(0x4472b0-AUX_INSERT_CONTROLLER_START)//4
    array_load,tail_store=words[i:i+2]
    if (array_load>>24,tail_store>>24)!=(0x91,0xa1):
        raise ValueError("aux insertion list-store/load opcode classes changed")
    if (array_load&MARKER_MASK,tail_store&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("aux insertion list-store/load packet markers changed")
    words[i:i+2]=[(tail_store&~MARKER_MASK)|CONTINUE_MARKER,
                  (array_load&~MARKER_MASK)|END_MARKER]
    return struct.pack('<39I',*words)


def restore_aux_remove_controller_packets(assembled: bytes) -> bytes:
    """Preserve separate 32-bit store/restore instructions and producer order.

    LLVM6 can compact two source operations into a duplex that the original
    producer emitted separately. The assembly source intentionally specifies
    two packets for `memw(r2+#20)=#0` and `r17:16=memd(r29+#0)` to retain their
    32-bit instruction encodings. Join them with a checked marker change.
    The predicated load in the later 3-word packet is reordered after the
    `memw(r3+#16)=r2` producer, restoring original operation ordering. All
    transformations are constrained to source-generated operation classes.
    """
    if len(assembled)!=AUX_REMOVE_CONTROLLER_END-AUX_REMOVE_CONTROLLER_START:
        raise ValueError("aux remove-controller must emit 84 source bytes")
    words=list(struct.unpack('<21I',assembled))
    i=(0x447300-AUX_REMOVE_CONTROLLER_START)//4
    store,restore=words[i:i+2]
    if [x>>24 for x in (store,restore)]!=[0x3c,0x91]:
        raise ValueError("aux remove-controller store/restore opcode classes changed")
    if [x&MARKER_MASK for x in (store,restore)]!=[END_MARKER,END_MARKER]:
        raise ValueError("aux remove-controller store/restore packet markers changed")
    if (store&~MARKER_MASK)!=0x3c420280:
        raise ValueError("aux remove-controller non-duplex source no longer memw(r2+#20)=#0")
    words[i]=(store&~MARKER_MASK)|CONTINUE_MARKER
    j=(0x447310-AUX_REMOVE_CONTROLLER_START)//4
    predicated_load,list_store=words[j:j+2]
    if [x>>24 for x in (predicated_load,list_store)]!=[0x47,0xa1]:
        raise ValueError("aux remove-controller predicated load/store opcode classes changed")
    if [x&MARKER_MASK for x in (predicated_load,list_store)]!=[
        CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("aux remove-controller predicated load/store packet markers changed")
    words[j:j+2]=[(list_store&~MARKER_MASK)|CONTINUE_MARKER,
                   (predicated_load&~MARKER_MASK)|END_MARKER]
    return struct.pack('<21I',*words)


def restore_aux_callback_extender_packet(assembled: bytes) -> bytes:
    """Mark a symbolic extended address load as continuing into a duplex.

    LLVM6 places the original three-operand packet into two adjacent packets
    when the immediate-extended `r6` assignment is written separately from
    a compound `r5=#0; r4=memw`. Its instruction values, relative branches
    and program lengths are already exactly source assembled. Hexagon permits
    the scalar `r6` to share the following duplex: only the parse marker on
    this checked operation changes. No vendor instruction bytes are inserted.
    """
    if len(assembled) != AUX_CALLBACK_END - AUX_CALLBACK_START:
        raise ValueError("aux callback registration must emit 100 source-assembled bytes")
    words = list(struct.unpack('<25I', assembled))
    idx = (0x447340 - AUX_CALLBACK_START)//4
    extender,scalar,duplex=words[idx-1:idx+2]
    if [v>>24 for v in (extender,scalar,duplex)]!=[0x01,0x78,0x48]:
        raise ValueError("aux callback register load/duplex opcode classes changed")
    if [v & MARKER_MASK for v in (extender,scalar,duplex)]!=[
        CONTINUE_MARKER,END_MARKER,0
    ]:
        raise ValueError("aux callback register load/duplex packet markers changed")
    if (scalar & ~MARKER_MASK)!=0x78000086:
        raise ValueError("aux callback register operand is no longer r6 = constant")
    words[idx]=(scalar & ~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<25I',*words)


def restore_aux_notify_callback_packets(assembled: bytes) -> bytes:
    """Restore compiler-independent packet order in the next notify routine.

    LLVM6 reorders a memory load/save packet, schedules a conditional jump
    ahead of its immediate extension/producer and exchanges a new-value
    predicated `combine` with a compare+branch. The source is already an
    independent textual Hexagon program; we only move checked architectural
    instructions and update packet parse markers. No reference bytes used.
    """
    if len(assembled) != AUX_NOTIFY_SCAN_END-AUX_NOTIFY_SCAN_START:
        raise ValueError("auxiliary notification callback requires 104 source bytes")
    words=list(struct.unpack('<26I',assembled))
    load,save=words[0x08//4:0x10//4]
    if [x>>24 for x in (load,save)] != [0x91,0xa1]:
        raise ValueError("notification memory load/save opcode classes changed")
    if [x&MARKER_MASK for x in (load,save)] != [CONTINUE_MARKER,END_MARKER]:
        raise ValueError("notification memory packet markers changed")
    words[0x08//4:0x10//4]=[(save&~MARKER_MASK)|CONTINUE_MARKER,
                           (load&~MARKER_MASK)|END_MARKER]
    branch,extender,pred_add=words[0x14//4:0x20//4]
    if [x>>24 for x in (branch,extender,pred_add)] != [0x5c,0x00,0x74]:
        raise ValueError("notification branch/extender/predicated add ISA classes changed")
    if [x&MARKER_MASK for x in (branch,extender,pred_add)] != [
        CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("notification branch packet markers changed")
    words[0x14//4:0x20//4]=[extender,
        (pred_add&~MARKER_MASK)|CONTINUE_MARKER,
        (branch&~MARKER_MASK)|END_MARKER]
    cmp_jump,pred_combine=words[0x28//4:0x30//4]
    if [x>>24 for x in (cmp_jump,pred_combine)]!=[0x10,0xfd]:
        raise ValueError("notification conditional branch/combine ISA classes changed")
    if [x&MARKER_MASK for x in (cmp_jump,pred_combine)]!=[
        CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("notification conditional branch packet markers changed")
    words[0x28//4:0x30//4]=[(pred_combine&~MARKER_MASK)|CONTINUE_MARKER,
                            (cmp_jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<26I',*words)


def restore_aux_cleanup_compare_packets(assembled: bytes) -> bytes:
    """Restore target compiler's compare+dec+new-value store packet layout.

    Hexagon LLVM6 reorders the compare/conditional branch before an integer
    decrement; the OEM toolchain schedules the decrement first and the
    compare/new-value branch second. Both are V60 instructions already
    source-assembled. The producer index in the consumer store changes from
    slot 1 to slot 2 when the order is restored. Only checked packet fields
    are updated; no original vendor word is copied into the output.
    """
    if len(assembled)!=AUX_CLEANUP_END-AUX_CLEANUP_START:
        raise ValueError("aux cleanup needs 272 independently source-assembled bytes")
    words=list(struct.unpack('<68I',assembled))
    for off in (0x88,0xb4):
        j=off//4
        branch,sub,consumer=words[j:j+3]
        if [x>>24 for x in (branch,sub,consumer)] != [0x10,0xbf,0xa1]:
            raise ValueError(f"aux cleanup decrement/branch/store ISA classes changed at {AUX_CLEANUP_START+off:#x}")
        if [x&MARKER_MASK for x in (branch,sub,consumer)] != [CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
            raise ValueError(f"aux cleanup decrement packet markers changed at {AUX_CLEANUP_START+off:#x}")
        if ((consumer>>9)&0x3)!=1:
            raise ValueError("aux cleanup new-value producer slot is no longer one")
        # Hexagon new-value store refers to producer slot 2 after reordering.
        store_changed=(consumer&~(0x3<<9))|(2<<9)
        words[j:j+3]=[sub,branch,store_changed]
    return struct.pack('<68I',*words)


def restore_aux_find_format_predicate_packet(assembled: bytes) -> bytes:
    """Reorder checked V60 new-value compare/branch in codec format scan.

    LLVM6 schedules a conditional jump before the immediate-extended compare
    and the producer's predicated move. Relocate the *already assembled*
    instructions and recompute their packet markers, without OEM operands.
    """
    if len(assembled) != AUX_FIND_FORMAT_END - AUX_FIND_FORMAT_START:
        raise ValueError("auxiliary format scanner must emit 92 compiled bytes")
    words=list(struct.unpack('<23I',assembled))
    jump,extend,compare,predmove=words[0x24//4:0x34//4]
    if [v>>24 for v in (jump,extend,compare,predmove)]!=[0x5c,0x00,0x75,0x7e]:
        raise ValueError("format scanner jump/extender/compare/move opcode classes changed")
    if [v&MARKER_MASK for v in (jump,extend,compare,predmove)] != [
        CONTINUE_MARKER,CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("format scanner predicate packet markers changed")
    words[0x24//4:0x34//4]=[extend,compare,
        (predmove&~MARKER_MASK)|CONTINUE_MARKER,
        (jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<23I',*words)


def restore_aux_find_class_new_predicate(assembled: bytes) -> bytes:
    """Restore new-value predicate and branch order for class scan loop.

    Hexagon's original packet encodes an immediate predicated subtraction
    before its conditional branch; LLVM6 schedules the jump first and the
    subsequent predicated memory read with the previous-packet P0 form.
    A typed V60 P0.new field is enabled only on the exact expected load.
    """
    if len(assembled)!=AUX_FIND_CLASS_END-AUX_FIND_CLASS_START:
        raise ValueError("auxiliary codec class scanner must emit 84 compiled bytes")
    words=list(struct.unpack('<21I',assembled))
    jump,compare,sub=words[0x08//4:0x14//4]
    if [v>>24 for v in (jump,compare,sub)] != [0x5c,0xf2,0xfb]:
        raise ValueError("codec class compare/branch/sub ISA classes changed")
    if [v&MARKER_MASK for v in (jump,compare,sub)]!=[
        CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("codec class predicate packet markers changed")
    words[0x08//4:0x14//4]=[compare,
        (sub&~MARKER_MASK)|CONTINUE_MARKER,
        (jump&~MARKER_MASK)|CONTINUE_MARKER]
    load=words[0x14//4]
    # Opcode 0x45 is the P0-old predicated load, 0x47 the .new form.
    # The typed operands (r6, r5, 28) are verified before editing field 25.
    operand_mask=0x00ffffff & ~MARKER_MASK
    if load>>24!=0x45 or load&operand_mask!=(0x85c0e6&operand_mask):
        raise ValueError("codec class P0-old predicated load operand/ISA class changed")
    if (load&MARKER_MASK)!=END_MARKER:
        raise ValueError("codec class predicated load packet marker changed")
    words[0x14//4]=load|(1<<25)
    return struct.pack('<21I',*words)


def restore_aux_large_controller_phase2_packet(assembled: bytes) -> bytes:
    """Preserve one Hexagon V60 predicated conditional branch packet.

    LLVM6 schedules a conditional jump before the immediate extender and
    P0.new producer, whereas the reference architecture places the extender
    and predicated constant move before the branch within one packet. This
    rearranges only independently assembled ISA instructions, checked for
    expected packet markers and operand fields; it never imports OEM bytes.
    """
    if len(assembled) != AUX_LARGE_CONTROLLER_PHASE2_END-AUX_LARGE_CONTROLLER_PHASE2_START:
        raise ValueError("controller phase2 must assemble 168 bytes")
    words=list(struct.unpack('<42I',assembled))
    index=0x70//4
    jump,ext,predmove=words[index:index+3]
    if (jump>>24,ext>>24,predmove>>24)!=(0x5c,0x01,0x7e):
        raise ValueError("controller phase2 predicate packet ISA opcode classes changed")
    if (jump&MARKER_MASK,ext&MARKER_MASK,predmove&MARKER_MASK)!=(
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError("controller phase2 predicate packet markers changed")
    if jump&0x3f != 0x12 or predmove&0x3f != 0x03:
        raise ValueError("controller phase2 conditional target or register changed")
    words[index:index+3]=[ext,(predmove&~MARKER_MASK)|CONTINUE_MARKER,
                            (jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<42I',*words)


def restore_aux_large_controller_phase3_packets(assembled: bytes) -> bytes:
    """Reconstruct two source-assembled V60 predicate/loop packet orders.

    LLVM6 schedules the P0.new producer before the compare, and a later
    conditional branch ahead of its immediate-extended comparison. Both
    reorderings remain within their original packet and are fail-closed on
    unexpected ISA opcode classes, registers, immediates or parse markers.
    """
    expected_len=AUX_LARGE_CONTROLLER_PHASE3_END-AUX_LARGE_CONTROLLER_PHASE3_START
    if len(assembled)!=expected_len:
        raise ValueError("controller phase3 must assemble 184 bytes")
    words=list(struct.unpack('<46I',assembled))
    i=0x08//4
    p0branch,prodmov=words[i:i+2]
    if (p0branch>>24,prodmov>>24)!=(0x10,0x7e):
        raise ValueError("controller phase3 predicate/move opcode classes changed")
    if (p0branch&MARKER_MASK,prodmov&MARKER_MASK)!=(
            CONTINUE_MARKER,CONTINUE_MARKER):
        raise ValueError("controller phase3 predicate/move packet markers changed")
    if prodmov&0x3f != 0x16 or p0branch&0x3f != 0x3c:
        raise ValueError("controller phase3 predicate/move operands changed")
    words[i:i+2]=[prodmov,p0branch]
    j=0x28//4
    jump,ext,compare=words[j:j+3]
    if [v>>24 for v in (jump,ext,compare)] != [0x5c,0x00,0x75]:
        raise ValueError("controller phase3 loop jump/extender/compare classes changed")
    if [v&MARKER_MASK for v in (jump,ext,compare)] != [
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase3 loop branch packet markers changed")
    if (jump&0x3f)!=0x04 or (compare&0x3f)!=0x00:
        raise ValueError("controller phase3 loop branch target/compare operands changed")
    words[j:j+3]=[ext,(compare&~MARKER_MASK)|CONTINUE_MARKER,
                        (jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<46I',*words)


def restore_aux_large_controller_phase5_packets(assembled: bytes) -> bytes:
    """Checked fixup for V60 slot-conflicting four-operation branch packet.

    LLVM6 cannot assemble the compound `r2 += add(...)` in the same
    packet as a P0.new branch and predicated store, although this valid
    packet occurs in the target V60 executable. Assemble the same source
    instructions separately, join their existing parse markers and restore
    the defined two-byte-unit branch displacement. Also join the earlier
    resource callback with its independently emitted arithmetic packet.
    """
    n=AUX_LARGE_CONTROLLER_PHASE5_END-AUX_LARGE_CONTROLLER_PHASE5_START
    if len(assembled)!=n:
        raise ValueError("controller phase5 must source-assemble 40 bytes")
    words=list(struct.unpack('<10I',assembled))
    call,arith,branch=words[0],words[0x18//4],words[0x20//4]
    if [x>>24 for x in (call,arith,branch)]!=[0x5a,0xe2,0x5c]:
        raise ValueError("controller phase5 call/add/branch opcode classes changed")
    if [x&MARKER_MASK for x in (call,arith,branch)]!=[
            END_MARKER,END_MARKER,CONTINUE_MARKER]:
        raise ValueError("controller phase5 source packet markers changed")
    if (call&0x3f,arith&0x3f,branch&0x3f)!=(0x04,0x22,0x14):
        raise ValueError("controller phase5 call/operand/branch relocation changed")
    words[0]=(call&~MARKER_MASK)|CONTINUE_MARKER
    words[0x18//4]=(arith&~MARKER_MASK)|CONTINUE_MARKER
    words[0x20//4]=(branch&~0x3f)|0x16
    return struct.pack('<10I',*words)


def assemble(llvm_mc: Path, source: Path) -> bytes:
    with tempfile.TemporaryDirectory(prefix="nabu-modem-late-") as tmp:
        obj = Path(tmp) / "late.o"
        command = [str(llvm_mc), "-triple=hexagon", "-mcpu=hexagonv60",
                   "-filetype=obj", str(source), "-o", str(obj)]
        result = subprocess.run(command, text=True, capture_output=True)
        if result.returncode:
            raise RuntimeError(f"Hexagon LLVM MC assembly failed for {source.name}: {result.stderr}")
        return extract_text(obj.read_bytes())


def output_fragments(llvm_mc: Path) -> dict[int, bytes]:
    atomic = restore_atomic_packet_order(assemble(llvm_mc, ATOMIC_SOURCE))
    controller = restore_controller_packet_order(assemble(llvm_mc, CONTROL_SOURCE))
    auxiliary = assemble(llvm_mc, AUX_SOURCE)
    if len(auxiliary) != AUX_END - AUX_START:
        raise ValueError(f"auxiliary boot source emitted {len(auxiliary)} instead of {AUX_END-AUX_START} bytes")
    aux_init = restore_aux_init_packet(assemble(llvm_mc,AUX_INIT_SOURCE))
    aux_dynamic = emit_auxiliary_elf_symbols()
    aux_align = emit_auxiliary_code_alignment()
    aux_guard = assemble(llvm_mc, AUX_GUARD_SOURCE)
    if len(aux_guard) != AUX_GUARD_END - AUX_GUARD_START:
        raise ValueError("auxiliary guard instruction block has invalid length")
    aux_stage = restore_aux_stage_packets(assemble(llvm_mc,AUX_STAGE_SOURCE))
    aux_mmio = assemble(llvm_mc,AUX_MMIO_SOURCE)
    aux_heap = restore_aux_heap_predicate_packet(assemble(llvm_mc,AUX_HEAP_SOURCE))
    aux_worker = restore_aux_worker_packets(assemble(llvm_mc,AUX_WORKER_SOURCE))
    aux_lookup = restore_aux_lookup_return_packet(assemble(llvm_mc,AUX_LOOKUP_SOURCE))
    aux_registry_core=restore_aux_registry_core_packets(assemble(llvm_mc,AUX_REGISTRY_CORE_SOURCE))
    aux_notify_scan=restore_aux_notify_callback_packets(assemble(llvm_mc,AUX_NOTIFY_SCAN_SOURCE))
    aux_cleanup=restore_aux_cleanup_compare_packets(assemble(llvm_mc,AUX_CLEANUP_SOURCE))
    aux_factory=assemble(llvm_mc,AUX_FACTORY_SOURCE)
    aux_find_format=restore_aux_find_format_predicate_packet(assemble(llvm_mc,AUX_FIND_FORMAT_SOURCE))
    aux_find_class=restore_aux_find_class_new_predicate(assemble(llvm_mc,AUX_FIND_CLASS_SOURCE))
    aux_controller_slot=assemble(llvm_mc,AUX_CONTROLLER_SLOT_SOURCE)
    aux_large_controller_prefix=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PREFIX_SOURCE)
    aux_large_controller_mid=assemble(llvm_mc,AUX_LARGE_CONTROLLER_MID_SOURCE)
    aux_large_controller_phase2=restore_aux_large_controller_phase2_packet(assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE2_SOURCE))
    aux_large_controller_phase3=restore_aux_large_controller_phase3_packets(assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE3_SOURCE))
    aux_large_controller_phase4=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE4_SOURCE)
    aux_large_controller_phase5=restore_aux_large_controller_phase5_packets(assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE5_SOURCE))
    aux_dispatch = assemble(llvm_mc,AUX_DISPATCH_SOURCE)
    aux_type = restore_aux_type_return_packets(assemble(llvm_mc,AUX_TYPE_SOURCE))
    aux_notify = restore_aux_controller_notify_packets(assemble(llvm_mc,AUX_NOTIFY_SOURCE))
    aux_entry = assemble(llvm_mc,AUX_ENTRY_SOURCE)
    aux_validator = restore_aux_validator_predicate_packets(assemble(llvm_mc,AUX_VALIDATOR_SOURCE))
    aux_runtime_init = restore_aux_runtime_init_packets(assemble(llvm_mc,AUX_RUNTIME_INIT_SOURCE))
    aux_veneers = compact_aux_controller_veneer_duplex(assemble(llvm_mc,AUX_VENEERS_SOURCE))
    aux_registration = restore_aux_registration_new_value_packet(assemble(llvm_mc,AUX_REGISTRATION_SOURCE))
    aux_diagnostic_helpers=assemble(llvm_mc,AUX_DIAGNOSTIC_HELPERS_SOURCE)
    aux_session_prefix=assemble(llvm_mc,AUX_SESSION_PREFIX_SOURCE)
    aux_session_validation=restore_aux_session_validation_packets(assemble(llvm_mc,AUX_SESSION_VALIDATION_SOURCE))
    aux_find_free=assemble(llvm_mc,AUX_FIND_FREE_SOURCE)
    aux_resolve=assemble(llvm_mc,AUX_RESOLVE_SOURCE)
    aux_session_registry=restore_aux_session_registry_packets(assemble(llvm_mc,AUX_SESSION_REGISTRY_SOURCE))
    aux_insert_controller=restore_aux_insert_controller_packets(assemble(llvm_mc,AUX_INSERT_CONTROLLER_SOURCE))
    if len(aux_session_registry)!=AUX_SESSION_REGISTRY_END-AUX_SESSION_REGISTRY_START:
        raise ValueError("aux registry insertion fragment length mismatch")
    if len(aux_insert_controller)!=AUX_INSERT_CONTROLLER_END-AUX_INSERT_CONTROLLER_START:
        raise ValueError("aux insert-controller length mismatch")
    aux_remove_controller=restore_aux_remove_controller_packets(assemble(llvm_mc,AUX_REMOVE_CONTROLLER_SOURCE))
    if len(aux_remove_controller)!=AUX_REMOVE_CONTROLLER_END-AUX_REMOVE_CONTROLLER_START:
        raise ValueError("aux remove-controller length mismatch")
    aux_callback=restore_aux_callback_extender_packet(assemble(llvm_mc,AUX_CALLBACK_SOURCE))
    aux_find_controller=assemble(llvm_mc,AUX_FIND_CONTROLLER_SOURCE)
    aux_release_registry=assemble(llvm_mc,AUX_RELEASE_REGISTRY_SOURCE)
    if (len(aux_callback),len(aux_find_controller),len(aux_release_registry))!=(100,136,56):
        raise ValueError("late auxiliary callback/find/release routines have invalid byte lengths")
    if (AUX_CALLBACK_END != AUX_FIND_CONTROLLER_START or
            AUX_FIND_CONTROLLER_END != AUX_RELEASE_REGISTRY_START):
        raise ValueError("late auxiliary callback/find/release addresses are not contiguous")
    if len(aux_find_free)!=AUX_FIND_FREE_END-AUX_FIND_FREE_START:
        raise ValueError("aux free-entry finder must emit exactly 80 bytes")
    if len(aux_resolve)!=AUX_RESOLVE_END-AUX_RESOLVE_START:
        raise ValueError("aux handle resolver must emit exactly 136 bytes")
    if len(aux_diagnostic_helpers)!=AUX_DIAGNOSTIC_HELPERS_END-AUX_DIAGNOSTIC_HELPERS_START:
        raise ValueError("aux diagnostic helpers must emit 116 bytes")
    if len(aux_registry_core)!=AUX_REGISTRY_CORE_END-AUX_REGISTRY_CORE_START:
        raise ValueError("aux registry core source length mismatch")
    if len(aux_notify_scan)!=AUX_NOTIFY_SCAN_END-AUX_NOTIFY_SCAN_START:
        raise ValueError("aux notification source length mismatch")
    if len(aux_cleanup)!=AUX_CLEANUP_END-AUX_CLEANUP_START:
        raise ValueError("aux cleanup source length mismatch")
    for fragment,start,end,label in [
        (aux_factory,AUX_FACTORY_START,AUX_FACTORY_END,'object factory'),
        (aux_find_format,AUX_FIND_FORMAT_START,AUX_FIND_FORMAT_END,'format scanner'),
        (aux_find_class,AUX_FIND_CLASS_START,AUX_FIND_CLASS_END,'class scanner'),
        (aux_controller_slot,AUX_CONTROLLER_SLOT_START,AUX_CONTROLLER_SLOT_END,'controller slot'),
        (aux_large_controller_prefix,AUX_LARGE_CONTROLLER_PREFIX_START,AUX_LARGE_CONTROLLER_PREFIX_END,'large controller prefix'),
        (aux_large_controller_mid,AUX_LARGE_CONTROLLER_MID_START,AUX_LARGE_CONTROLLER_MID_END,'large controller continuation'),
        (aux_large_controller_phase2,AUX_LARGE_CONTROLLER_PHASE2_START,AUX_LARGE_CONTROLLER_PHASE2_END,'large controller phase2'),
        (aux_large_controller_phase3,AUX_LARGE_CONTROLLER_PHASE3_START,AUX_LARGE_CONTROLLER_PHASE3_END,'large controller phase3'),
        (aux_large_controller_phase4,AUX_LARGE_CONTROLLER_PHASE4_START,AUX_LARGE_CONTROLLER_PHASE4_END,'large controller phase4'),
        (aux_large_controller_phase5,AUX_LARGE_CONTROLLER_PHASE5_START,AUX_LARGE_CONTROLLER_PHASE5_END,'large controller phase5'),
    ]:
        if len(fragment)!=end-start:
            raise ValueError(f"auxiliary {label} source length mismatch")
    if len(aux_session_validation)!=AUX_SESSION_VALIDATION_END-AUX_SESSION_VALIDATION_START:
        raise ValueError("auxiliary session validation length mismatch")
    if len(aux_session_prefix)!=AUX_SESSION_PREFIX_END-AUX_SESSION_PREFIX_START:
        raise ValueError("aux session prefix must emit 88 bytes")
    if len(aux_entry)!=AUX_ENTRY_END-AUX_ENTRY_START:
        raise ValueError("auxiliary entry veneer must compile to 20 bytes")
    if len(aux_dispatch) != AUX_DISPATCH_END-AUX_DISPATCH_START:
        raise ValueError("aux callback dispatcher must emit exactly 68 bytes")
    assert AUX_DISPATCH_END == AUX_TYPE_START and AUX_TYPE_END == AUX_NOTIFY_START
    if len(aux_mmio) != AUX_MMIO_END-AUX_MMIO_START:
        raise ValueError("auxiliary MMIO initialization source has invalid length")
    assert AUX_END == AUX_DYNAMIC_START and AUX_DYNAMIC_END == AUX_INIT_START
    assert AUX_INIT_ALIGN_START == AUX_INIT_END and AUX_INIT_ALIGN_END == AUX_GUARD_START
    assert AUX_GUARD_END == AUX_STAGE_START and AUX_STAGE_END == AUX_MMIO_START
    assert AUX_MMIO_END == AUX_HEAP_START and AUX_HEAP_END == AUX_WORKER_START
    assert AUX_WORKER_END == AUX_LOOKUP_START
    service = restore_service_probe_packets(assemble(llvm_mc,SERVICE_SOURCE))
    service_mmio = restore_service_mmio_packets(assemble(llvm_mc,SERVICE_MMIO_SOURCE))
    service_flags = restore_service_flag_packets(assemble(llvm_mc,SERVICE_FLAGS_SOURCE))
    service_alloc = restore_service_alloc_return_packet(assemble(llvm_mc,SERVICE_ALLOC_SOURCE))
    assert SERVICE_END == SERVICE_MMIO_START and SERVICE_MMIO_END == SERVICE_FLAGS_START
    assert SERVICE_FLAGS_END == SERVICE_ALLOC_START
    # All adjacent auxiliary controller routines are now source generated:
    # 0x446650..0x447448 is one continuous verified executable block.
    registry=(aux_dispatch+aux_type+aux_notify+aux_entry+aux_validator+
              aux_runtime_init+aux_veneers+aux_registration+
              aux_diagnostic_helpers+aux_session_prefix+aux_session_validation+
              aux_find_free+aux_resolve+aux_session_registry+aux_insert_controller+
              aux_remove_controller+aux_callback+aux_find_controller+
              aux_release_registry)
    if len(registry)!=AUX_RELEASE_REGISTRY_END-AUX_DISPATCH_START:
        raise ValueError("source-generated auxiliary registry block is not contiguous")
    if not (AUX_RELEASE_REGISTRY_END==AUX_NOTIFY_SCAN_START and
            AUX_NOTIFY_SCAN_END==AUX_CLEANUP_START and
            AUX_CLEANUP_END==AUX_FACTORY_START and
            AUX_FACTORY_END==AUX_FIND_FORMAT_START and
            AUX_FIND_FORMAT_END==AUX_FIND_CLASS_START and
            AUX_FIND_CLASS_END==AUX_CONTROLLER_SLOT_START and
            AUX_CONTROLLER_SLOT_END==AUX_LARGE_CONTROLLER_PREFIX_START and
            AUX_LARGE_CONTROLLER_PREFIX_END==AUX_LARGE_CONTROLLER_MID_START and
            AUX_LARGE_CONTROLLER_MID_END==AUX_LARGE_CONTROLLER_PHASE2_START and
            AUX_LARGE_CONTROLLER_PHASE2_END==AUX_LARGE_CONTROLLER_PHASE3_START and
            AUX_LARGE_CONTROLLER_PHASE3_END==AUX_LARGE_CONTROLLER_PHASE4_START and
            AUX_LARGE_CONTROLLER_PHASE4_END==AUX_LARGE_CONTROLLER_PHASE5_START):
        raise ValueError("missing source after controller registry block")
    if AUX_LOOKUP_END!=AUX_REGISTRY_CORE_START or AUX_REGISTRY_CORE_END!=AUX_DISPATCH_START:
        raise ValueError("modem auxiliary registry source intervals have a gap")
    return {
        ATOMIC_START:atomic+controller,
        SERVICE_START:service+service_mmio+service_flags+service_alloc,
        AUX_START:auxiliary+aux_dynamic+aux_init+aux_align+
                  aux_guard+aux_stage+aux_mmio+aux_heap+aux_worker+
                        aux_lookup+aux_registry_core+registry+aux_notify_scan+aux_cleanup+
                        aux_factory+aux_find_format+aux_find_class+aux_controller_slot+
                        aux_large_controller_prefix+aux_large_controller_mid+aux_large_controller_phase2+
                        aux_large_controller_phase3+aux_large_controller_phase4+
                        aux_large_controller_phase5,
    }


def executable_segments(reference: bytes) -> dict[int, tuple[int, int, int, int]]:
    if reference[:6] != b"\x7fELF\x01\x01" or len(reference) < 52:
        raise ValueError("expected ELF32 little-endian original modem MBN")
    if struct.unpack_from("<H", reference, 18)[0] != 164:
        raise ValueError("original MBN is not Hexagon ELF")
    phoff = struct.unpack_from("<I", reference, 28)[0]
    phentsize, phnum = struct.unpack_from("<HH", reference, 42)
    if phentsize != 32 or phoff + phnum * phentsize > len(reference):
        raise ValueError("invalid original modem ELF program headers")
    segments = {}
    for i in range(phnum):
        ptype, offset, vma, pma, filesz, memsz, flags, align = struct.unpack_from(
            "<8I", reference, phoff + i * phentsize
        )
        if ptype == 1 and flags & 1:
            if offset in segments:
                raise ValueError(f"duplicate executable load at file offset {offset:#x}")
            if filesz > memsz or offset + filesz > len(reference):
                raise ValueError(f"invalid executable load at {offset:#x}")
            segments[offset] = (vma, pma, filesz, memsz)
    return segments


def verify_reference(original: bytes, fragments: dict[int, bytes]) -> None:
    digest = hashlib.sha256(original).hexdigest()
    if digest != EXPECTED_REFERENCE_SHA256:
        raise ValueError(f"wrong modem firmware revision, SHA256={digest}")
    segments = executable_segments(original)
    expected_map = {
        0x7C000: (0xC0890000, 0x8E090000, 0x183CF0),
        0x446000: (0x1E400000, 0x90000000, 0x11DA4),
    }
    for start, (vma, pma, min_filesz) in expected_map.items():
        actual = segments.get(start)
        if actual is None or actual[:3] != (vma, pma, min_filesz):
            raise ValueError(f"unexpected mapped executable segment {start:#x}: {actual}")
    for start, rebuilt in fragments.items():
        covering = [(s, p) for s, p in segments.items() if s <= start < s + p[2]]
        if len(covering) != 1:
            raise ValueError(f"fragment at {start:#x} is not wholly in exactly one executable segment")
        segment_start, (vma, pma, filesz, memsz) = covering[0]
        if start + len(rebuilt) > segment_start + filesz:
            raise ValueError(f"generated fragment at {start:#x} exceeds executable p_filesz")
        expected = original[start : start + len(rebuilt)]
        if expected != rebuilt:
            i = next(i for i, (a, b) in enumerate(zip(expected, rebuilt)) if a != b)
            raise ValueError(f"Hexagon mismatch at modem file {start+i:#x}: expected {expected[i]:02x}, generated {rebuilt[i]:02x}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--llvm-mc", required=True, type=Path)
    ap.add_argument("--reference", type=Path, help="original pinned modem.mbn stored outside Git")
    ap.add_argument("--output-dir", type=Path, help="directory for the generated, *partial* executable fragments")
    args = ap.parse_args()
    fragments = output_fragments(args.llvm_mc)
    if args.reference:
        verify_reference(args.reference.read_bytes(), fragments)
    if args.output_dir:
        args.output_dir.mkdir(parents=True, exist_ok=True)
        for start, blob in fragments.items():
            path = args.output_dir / f"modem-late-file-{start:06x}-{start+len(blob):06x}.bin"
            path.write_bytes(blob)
    for start, blob in fragments.items():
        print(f"Hexagon V60 independent source: file [{start:#x},{start+len(blob):#x}) "
              f"= {len(blob)} bytes, SHA256={hashlib.sha256(blob).hexdigest()}")
    metadata=AUX_DYNAMIC_END-AUX_DYNAMIC_START
    structural=AUX_INIT_ALIGN_END-AUX_INIT_ALIGN_START
    code_bytes=sum(len(x) for x in fragments.values())-metadata-structural
    print(f"Total generated {sum(len(x) for x in fragments.values())} PARTIAL modem bytes in "
          f"{len(fragments)} verified fragments, of which {code_bytes} are "
          f"source-encoded Hexagon executable code, {metadata} independently "
          f"derived ELF32 metadata and {structural} linker-alignment zeros; "
          f"original reference {'matched' if args.reference else 'not supplied'}; "
          "NOT complete, not new OEM authentication")
    return 0

if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, RuntimeError, OSError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        sys.exit(1)
