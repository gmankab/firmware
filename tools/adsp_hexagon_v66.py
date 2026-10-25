#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Typed Hexagon system-register/one-word instruction encodings.

LLVM 6's Hexagon MC assembler does not accept the ADSP's special-register
and cache-kill mnemonics. The small subset below encodes explicit Hexagon
instruction fields, rather than pasting OEM instruction arrays.

Each encoding produces a single end-of-packet 32-bit little-endian instruction.
This module intentionally does not implement arbitrary V66 packets or claim
architecture-wide correctness. Validate new opcode families separately.
"""
from __future__ import annotations

from enum import IntEnum
import struct


class HexagonEncodingError(ValueError):
    pass


class SystemRegister(IntEnum):
    SSR = 6
    SYSCFG = 18


class CacheCommand(IntEnum):
    # Nullary ISA opcodes: these have no variable operands.
    DATA_CACHE_KILL = 0xA200C000
    INSTRUCTION_CACHE_KILL = 0x56C0D000


def gpr(index: int) -> int:
    if not isinstance(index, int) or not (0 <= index <= 31):
        raise HexagonEncodingError(f"Not a Hexagon GPR: {index!r}")
    return index


def read_system_register(destination: int, sysreg: SystemRegister) -> int:
    """Rd = SysReg; 5-bit dest in bits[4:0], system reg in bits[20:16]."""
    return 0x6E80C000 | (int(sysreg) << 16) | gpr(destination)


def write_system_register(sysreg: SystemRegister, source: int) -> int:
    """SysReg = Rs; 5-bit source in bits[20:16], system reg bits[4:0]."""
    return 0x6700C000 | (gpr(source) << 16) | int(sysreg)


def set_bit(destination: int, source: int, bit: int) -> int:
    """Rd = setbit(Rs,#bit), 0..31; long literal extenders not permitted."""
    if not isinstance(bit, int) or not 0 <= bit <= 31:
        raise HexagonEncodingError(f"Not a Hexagon bit index: {bit}")
    return (0x8CC0C000 | (gpr(source) << 16) |
            (bit << 8) | gpr(destination))


def jump_near(current_relative_pc: int, target_relative_pc: int) -> int:
    """Unconditional near jump with the supported nonnegative halfword offset.

    The instruction encodes byte displacement / 2. This narrow range is
    sufficient for the ADSP bootstrap's forward stage-two branch; callers
    outside it must instead use a verified assembler/supported long branch.
    """
    displacement = target_relative_pc - current_relative_pc
    if current_relative_pc % 4 or target_relative_pc % 4:
        raise HexagonEncodingError("Hexagon jump targets must be word aligned")
    if displacement < 0 or displacement > 0x1FFE:
        raise HexagonEncodingError(f"Out-of-range forward Hexagon jump: {displacement}")
    return 0x5800C000 | (displacement >> 1)


def boot_special_code() -> bytes:
    """Eight typed V66 bootstrap instructions immediately after the vectors.

    Entry ELF RX-relative offsets 0x24..0x44. These are *program operations*,
    not a vendor machine-word table. No reference input is accessed here.
    """
    words = (
        write_system_register(SystemRegister.SSR, 0),
        read_system_register(0, SystemRegister.SYSCFG),
        set_bit(0, 0, 3),
        set_bit(0, 0, 5),
        write_system_register(SystemRegister.SYSCFG, 0),
        CacheCommand.DATA_CACHE_KILL,
        CacheCommand.INSTRUCTION_CACHE_KILL,
        jump_near(0x40, 0x63C),
    )
    encoded = b"".join(struct.pack("<I", value) for value in words)
    assert len(encoded) == 0x20
    return encoded


def syscfg_handoff_steps() -> tuple[tuple[str, int | str, int], ...]:
    """Build a source-level, 21-word Hexagon SYS_CFG and cache sequence.

    ``isa`` operations are checked typed system-register/only-one-opcode
    forms; ``asm`` operations are original instruction source emitted by the
    Hexagon assembler. Packet sizing is explicit to prevent implicit gaps.
    This entry lives at ADSP file [0x46a0,0x46f4), after the 0x41e0 setup.
    """
    syscfg = SystemRegister.SYSCFG
    return (
        ("isa", read_system_register(15, syscfg), 4),
        ("asm", "{ r15 = clrbit(r15,#1) }", 4),
        ("isa", write_system_register(syscfg, 15), 4),
        ("asm", "{ isync }", 4),
        ("isa", int(CacheCommand.INSTRUCTION_CACHE_KILL), 4),
        ("asm", "{ isync }", 4),
        ("asm", "{ r15 = or(r15,#3) }", 4),
        ("isa", write_system_register(syscfg, 15), 4),
        ("asm", "{ isync }", 4),
        ("asm", "{ lr = sub(lr,r25) }", 4),
        ("asm", "{ jumpr lr }", 4),
        ("asm", "{ fp = ##-0xfffe920; lr = add(lr,r25) }", 12),
        ("asm", "{ fp = add(fp,r25) }", 4),
        ("asm", "{ jumpr fp }", 4),
        ("isa", read_system_register(15, syscfg), 4),
        ("asm", "{ r15 = clrbit(r15,#0) }", 4),
        ("isa", write_system_register(syscfg, 15), 4),
        ("asm", "{ isync }", 4),
        ("asm", "{ jumpr lr }", 4),
    )
