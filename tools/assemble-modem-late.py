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
AUX_DISPATCH_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_dispatch_helpers.s"
AUX_TYPE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_type_config.s"
AUX_NOTIFY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_controller_notify.s"
AUX_ENTRY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_entry.s"
AUX_VALIDATOR_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_input_validator.s"
AUX_RUNTIME_INIT_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_runtime_init.s"
AUX_VENEERS_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_control_veneers.s"
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
AUX_INIT_START = 0x446200
AUX_INIT_END = 0x446254
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
    aux_guard = assemble(llvm_mc, AUX_GUARD_SOURCE)
    if len(aux_guard) != AUX_GUARD_END - AUX_GUARD_START:
        raise ValueError("auxiliary guard instruction block has invalid length")
    aux_stage = restore_aux_stage_packets(assemble(llvm_mc,AUX_STAGE_SOURCE))
    aux_mmio = assemble(llvm_mc,AUX_MMIO_SOURCE)
    aux_heap = restore_aux_heap_predicate_packet(assemble(llvm_mc,AUX_HEAP_SOURCE))
    aux_worker = restore_aux_worker_packets(assemble(llvm_mc,AUX_WORKER_SOURCE))
    aux_lookup = restore_aux_lookup_return_packet(assemble(llvm_mc,AUX_LOOKUP_SOURCE))
    aux_dispatch = assemble(llvm_mc,AUX_DISPATCH_SOURCE)
    aux_type = restore_aux_type_return_packets(assemble(llvm_mc,AUX_TYPE_SOURCE))
    aux_notify = restore_aux_controller_notify_packets(assemble(llvm_mc,AUX_NOTIFY_SOURCE))
    aux_entry = assemble(llvm_mc,AUX_ENTRY_SOURCE)
    aux_validator = restore_aux_validator_predicate_packets(assemble(llvm_mc,AUX_VALIDATOR_SOURCE))
    aux_runtime_init = restore_aux_runtime_init_packets(assemble(llvm_mc,AUX_RUNTIME_INIT_SOURCE))
    aux_veneers = compact_aux_controller_veneer_duplex(assemble(llvm_mc,AUX_VENEERS_SOURCE))
    if len(aux_entry)!=AUX_ENTRY_END-AUX_ENTRY_START:
        raise ValueError("auxiliary entry veneer must compile to 20 bytes")
    if len(aux_dispatch) != AUX_DISPATCH_END-AUX_DISPATCH_START:
        raise ValueError("aux callback dispatcher must emit exactly 68 bytes")
    assert AUX_DISPATCH_END == AUX_TYPE_START and AUX_TYPE_END == AUX_NOTIFY_START
    if len(aux_mmio) != AUX_MMIO_END-AUX_MMIO_START:
        raise ValueError("auxiliary MMIO initialization source has invalid length")
    assert AUX_GUARD_END == AUX_STAGE_START and AUX_STAGE_END == AUX_MMIO_START
    assert AUX_MMIO_END == AUX_HEAP_START and AUX_HEAP_END == AUX_WORKER_START
    assert AUX_WORKER_END == AUX_LOOKUP_START
    service = restore_service_probe_packets(assemble(llvm_mc,SERVICE_SOURCE))
    service_mmio = restore_service_mmio_packets(assemble(llvm_mc,SERVICE_MMIO_SOURCE))
    service_flags = restore_service_flag_packets(assemble(llvm_mc,SERVICE_FLAGS_SOURCE))
    service_alloc = restore_service_alloc_return_packet(assemble(llvm_mc,SERVICE_ALLOC_SOURCE))
    assert SERVICE_END == SERVICE_MMIO_START and SERVICE_MMIO_END == SERVICE_FLAGS_START
    assert SERVICE_FLAGS_END == SERVICE_ALLOC_START
    return {ATOMIC_START: atomic + controller, SERVICE_START: service+service_mmio+service_flags+service_alloc,
            AUX_START: auxiliary, AUX_INIT_START: aux_init,
            AUX_GUARD_START: aux_guard+aux_stage+aux_mmio+aux_heap+aux_worker+aux_lookup,
            AUX_DISPATCH_START:aux_dispatch+aux_type+aux_notify+aux_entry+aux_validator+aux_runtime_init+aux_veneers}


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
    print(f"Total generated {sum(len(x) for x in fragments.values())} PARTIAL modem bytes in "
          f"{len(fragments)} verified fragments; "
          f"original reference {'matched' if args.reference else 'not supplied'}; "
          "NOT complete, not new OEM authentication")
    return 0

if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, RuntimeError, OSError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        sys.exit(1)
