// SPDX-License-Identifier: GPL-3.0-only
//! Agent 20-18: the complete Novatek EX9 instruction table.
//!
//! Every slot is a typed physical NDS32 instruction with semantic operands.
//! The emitter reads no original image and has no raw instruction-word path.
//! EX9.IT index n selects one four-byte entry at ITB + 4*n. This image sets
//! ITB to runtime 0x17bc0, corresponding to firmware file offset 0x17dc0.
//! The 512 slots occupy [0x17dc0,0x185c0), separately from ordinary code.
//!
//! J/JAL have EX9-specific semantics: their encoded low-25-bit target is
//! combined with the executing caller PC's high seven bits. They are not
//! relative to the instruction-table location. This firmware's call sites
//! and targets are in the low-25-bit address region, so Ex9J/Ex9Jal below
//! explicitly name absolute runtime targets. Other instruction operands use
//! architectural register identities and byte displacements.
//!
//! Independent encoding reference: unmodified GNU binutils 2.30,
//! opcodes/nds32-asm.c and opcodes/nds32-dis.c (NDS32_PARSE_EX9TAB), and
//! bfd/elf32-nds32.c (nds32_elf_ex9_reloc_jmp). The accompanying GNU assembly
//! uses ordinary assembler `. + target` expressions solely to verify the
//! special absolute halfword field independently. It does not reinterpret
//! the table as sequential executable code.
//!
//! GNU source: https://ftp.gnu.org/gnu/binutils/binutils-2.30.tar.xz
//! Source archive SHA-256:
//! 6e46b8aeae2f727a36f0bd9505e405768a72218f1796f0d09757d45209871ae6
//! Firmware table SHA-256:
//! a58613094f4ce93767d70120205520453bc193fea1c7f0fcaf7f1f0cb124a23c

use super::novatek_upper_isa::{
    Gpr::*, Instruction, Instruction::*, SYS_INT_MASK2, SYS_INT_PEND2,
};

pub const SOURCE_START: u32 = 0x17dc0;
pub const SOURCE_END: u32 = 0x185c0;
pub const ITB_RUNTIME_BASE: u32 = 0x17bc0;
pub const ILM_FILE_BIAS: u32 = 0x200;
pub const SLOT_COUNT: usize = 512;

/// Numeric architectural CSR index observed in a table entry; its controller
/// role is not inferred from the neighboring named interrupt registers.
const NOVATEK_CSR_C2: u16 = 0x00c2;

/// Instruction-table entries in EX9.IT index order. Comments give slot index,
/// firmware-file location, and decoded operation for integration review.
pub const CODE: [Instruction; SLOT_COUNT] = [
    LwiGp(R0, -5436), // slot 000, file 0x17dc0: lwi.gp $r0, [ + #-5436]
    Ex9Jal(0x84aa), // slot 001, file 0x17dc4: jal PC(31,25)|#0x84aa
    Ex9Jal(0x23de), // slot 002, file 0x17dc8: jal PC(31,25)|#0x23de
    Movi(R0, 0x3f04a), // slot 003, file 0x17dcc: movi $r0, #0x3f04a
    Movi(R1, 0x55), // slot 004, file 0x17dd0: movi $r1, #0x55
    Ex9Jal(0x25fe), // slot 005, file 0x17dd4: jal PC(31,25)|#0x25fe
    Ex9Jal(0xe328), // slot 006, file 0x17dd8: jal PC(31,25)|#0xe328
    Ex9Jal(0x94f0), // slot 007, file 0x17ddc: jal PC(31,25)|#0x94f0
    Ex9Jal(0x17ae4), // slot 008, file 0x17de0: jal PC(31,25)|#0x17ae4
    Ex9Jal(0x94f0), // slot 009, file 0x17de4: jal PC(31,25)|#0x94f0
    Ex9Jal(0x85e8), // slot 010, file 0x17de8: jal PC(31,25)|#0x85e8
    Movi(R0, 0x38d40), // slot 011, file 0x17dec: movi $r0, #0x38d40
    LwiGp(R0, -5356), // slot 012, file 0x17df0: lwi.gp $r0, [ + #-5356]
    Ex9J(0x2aa), // slot 013, file 0x17df4: j PC(31,25)|#0x2aa
    Ex9Jal(0x84aa), // slot 014, file 0x17df8: jal PC(31,25)|#0x84aa
    Ex9Jal(0x433c), // slot 015, file 0x17dfc: jal PC(31,25)|#0x433c
    Ex9Jal(0x85e8), // slot 016, file 0x17e00: jal PC(31,25)|#0x85e8
    Ex9Jal(0x23de), // slot 017, file 0x17e04: jal PC(31,25)|#0x23de
    LwiGp(R1, -5436), // slot 018, file 0x17e08: lwi.gp $r1, [ + #-5436]
    LwiGp(R0, -4500), // slot 019, file 0x17e0c: lwi.gp $r0, [ + #-4500]
    Ex9Jal(0x8c04), // slot 020, file 0x17e10: jal PC(31,25)|#0x8c04
    LwiGp(R1, -5284), // slot 021, file 0x17e14: lwi.gp $r1, [ + #-5284]
    LbiGp(R0, -4629), // slot 022, file 0x17e18: lbi.gp $r0, [ + #-4629]
    Ex9Jal(0x854a), // slot 023, file 0x17e1c: jal PC(31,25)|#0x854a
    LbiGp(R0, -5619), // slot 024, file 0x17e20: lbi.gp $r0, [ + #-5619]
    LwiGp(R0, -5284), // slot 025, file 0x17e24: lwi.gp $r0, [ + #-5284]
    LwiGp(R1, -5356), // slot 026, file 0x17e28: lwi.gp $r1, [ + #-5356]
    LwiGp(R0, -5440), // slot 027, file 0x17e2c: lwi.gp $r0, [ + #-5440]
    Ex9Jal(0xe328), // slot 028, file 0x17e30: jal PC(31,25)|#0xe328
    Movi(R2, 0x190), // slot 029, file 0x17e34: movi $r2, #0x190
    LbiGp(R0, -5596), // slot 030, file 0x17e38: lbi.gp $r0, [ + #-5596]
    LmwBim(R6, R31, R6, 0x2), // slot 031, file 0x17e3c: lmw.bim $r6, [$sp], $r6, #0x2
    Ex9J(0x12340), // slot 032, file 0x17e40: j PC(31,25)|#0x12340
    SwiGp(R0, -5292), // slot 033, file 0x17e44: swi.gp $r0, [ + #-5292]
    Movi(R3, 0x190), // slot 034, file 0x17e48: movi $r3, #0x190
    LbsiGp(R0, -4902), // slot 035, file 0x17e4c: lbsi.gp $r0, [ + #-4902]
    Movi(R1, 0x2fef0), // slot 036, file 0x17e50: movi $r1, #0x2fef0
    LwiGp(R0, -5292), // slot 037, file 0x17e54: lwi.gp $r0, [ + #-5292]
    Ex9Jal(0x1755c), // slot 038, file 0x17e58: jal PC(31,25)|#0x1755c
    Ex9Jal(0x17380), // slot 039, file 0x17e5c: jal PC(31,25)|#0x17380
    Ex9Jal(0xf66c), // slot 040, file 0x17e60: jal PC(31,25)|#0xf66c
    SbiGp(R0, -4902), // slot 041, file 0x17e64: sbi.gp $r0, [ + #-4902]
    Ex9Jal(0x17188), // slot 042, file 0x17e68: jal PC(31,25)|#0x17188
    Movi(R2, 0x180), // slot 043, file 0x17e6c: movi $r2, #0x180
    Sbi(R0, R31, 7), // slot 044, file 0x17e70: sbi $r0, [$sp + #7]
    Ex9Jal(0xe11c), // slot 045, file 0x17e74: jal PC(31,25)|#0xe11c
    SwiGp(R0, -4772), // slot 046, file 0x17e78: swi.gp $r0, [ + #-4772]
    LbiGp(R0, 0xe64), // slot 047, file 0x17e7c: lbi.gp $r0, [ + #0xe64]
    Movi(R0, 0x55), // slot 048, file 0x17e80: movi $r0, #0x55
    SwiGp(R0, -5296), // slot 049, file 0x17e84: swi.gp $r0, [ + #-5296]
    Nop, // slot 050, file 0x17e88: nop
    Addi(R0, R6, -184), // slot 051, file 0x17e8c: addi $r0, $r6, #-184
    Add(R15, R0, R15), // slot 052, file 0x17e90: add $r15, $r0, $r15
    AddSlli(R3, R2, R3, 2), // slot 053, file 0x17e94: add_slli $r3, $r2, $r3, #2
    Movi(R3, 0x55), // slot 054, file 0x17e98: movi $r3, #0x55
    LbiGp(R0, 0x130d), // slot 055, file 0x17e9c: lbi.gp $r0, [ + #0x130d]
    LbiGp(R0, -55620), // slot 056, file 0x17ea0: lbi.gp $r0, [ + #-55620]
    Ex9Jal(0xbd64), // slot 057, file 0x17ea4: jal PC(31,25)|#0xbd64
    Movi(R2, 0x3f04a), // slot 058, file 0x17ea8: movi $r2, #0x3f04a
    SbiGp(R0, -5586), // slot 059, file 0x17eac: sbi.gp $r0, [ + #-5586]
    Movi(R2, 0x55), // slot 060, file 0x17eb0: movi $r2, #0x55
    Movi(R0, -86), // slot 061, file 0x17eb4: movi $r0, #-86
    Ex9Jal(0x2d6a), // slot 062, file 0x17eb8: jal PC(31,25)|#0x2d6a
    LwiGp(R0, -5328), // slot 063, file 0x17ebc: lwi.gp $r0, [ + #-5328]
    Ex9Jal(0xfb8c), // slot 064, file 0x17ec0: jal PC(31,25)|#0xfb8c
    Bitci(R0, R0, 0xe0), // slot 065, file 0x17ec4: bitci $r0, $r0, #0xe0
    Btst(R0, R0, 2), // slot 066, file 0x17ec8: btst $r0, $r0, #2
    Movi(R3, 0x38d41), // slot 067, file 0x17ecc: movi $r3, #0x38d41
    Andi(R0, R0, 0x7fff), // slot 068, file 0x17ed0: andi $r0, $r0, #0x7fff
    Ex9Jal(0x88d0), // slot 069, file 0x17ed4: jal PC(31,25)|#0x88d0
    LbiGp(R1, -4742), // slot 070, file 0x17ed8: lbi.gp $r1, [ + #-4742]
    SbiGp(R0, -5665), // slot 071, file 0x17edc: sbi.gp $r0, [ + #-5665]
    SbiGp(R0, -5596), // slot 072, file 0x17ee0: sbi.gp $r0, [ + #-5596]
    SwiGp(R0, 0xe80), // slot 073, file 0x17ee4: swi.gp $r0, [ + #0xe80]
    Btst(R0, R0, 0x1b), // slot 074, file 0x17ee8: btst $r0, $r0, #0x1b
    Ex9Jal(0x17b4c), // slot 075, file 0x17eec: jal PC(31,25)|#0x17b4c
    Lhsi(R1, R1, 2), // slot 076, file 0x17ef0: lhsi $r1, [$r1 + #2]
    AddSlli(R0, R1, R0, 2), // slot 077, file 0x17ef4: add_slli $r0, $r1, $r0, #2
    Divsr(R0, R0, R0, R1), // slot 078, file 0x17ef8: divsr $r0, $r0, $r0, $r1
    Sbi(R0, R31, 0xf), // slot 079, file 0x17efc: sbi $r0, [$sp + #0xf]
    Movi(R0, 0x2c504), // slot 080, file 0x17f00: movi $r0, #0x2c504
    Sbi(R1, R0, 0x1c), // slot 081, file 0x17f04: sbi $r1, [$r0 + #0x1c]
    Sbi(R9, R7, 0), // slot 082, file 0x17f08: sbi $r9, [$r7 + #0]
    Movi(R1, 0x3f04a), // slot 083, file 0x17f0c: movi $r1, #0x3f04a
    Shi(R0, R31, 6), // slot 084, file 0x17f10: shi $r0, [$sp + #6]
    LmwBi(R2, R2, R3, 0x0), // slot 085, file 0x17f14: lmw.bi $r2, [$r2], $r3, #0x0
    Btst(R0, R0, 4), // slot 086, file 0x17f18: btst $r0, $r0, #4
    Movi(R0, 0x2eca8), // slot 087, file 0x17f1c: movi $r0, #0x2eca8
    Ex9Jal(0x2cec), // slot 088, file 0x17f20: jal PC(31,25)|#0x2cec
    Ex9Jal(0xec40), // slot 089, file 0x17f24: jal PC(31,25)|#0xec40
    Ex9Jal(0x25fe), // slot 090, file 0x17f28: jal PC(31,25)|#0x25fe
    Ex9Jal(0x23e6), // slot 091, file 0x17f2c: jal PC(31,25)|#0x23e6
    SbiGp(R0, -4901), // slot 092, file 0x17f30: sbi.gp $r0, [ + #-4901]
    LwiGp(R0, -5124), // slot 093, file 0x17f34: lwi.gp $r0, [ + #-5124]
    SwiGp(R0, 0xe7c), // slot 094, file 0x17f38: swi.gp $r0, [ + #0xe7c]
    Ori(R0, R0, 4), // slot 095, file 0x17f3c: ori $r0, $r0, #4
    Ex9Jal(0x41d0), // slot 096, file 0x17f40: jal PC(31,25)|#0x41d0
    LbiGp(R0, -4782), // slot 097, file 0x17f44: lbi.gp $r0, [ + #-4782]
    LwiGp(R1, -5340), // slot 098, file 0x17f48: lwi.gp $r1, [ + #-5340]
    Sbi(R0, R9, 1), // slot 099, file 0x17f4c: sbi $r0, [$r9 + #1]
    Ex9J(0xa4e6), // slot 100, file 0x17f50: j PC(31,25)|#0xa4e6
    Lhi(R0, R31, 6), // slot 101, file 0x17f54: lhi $r0, [$sp + #6]
    Andi(R6, R6, 0x7fff), // slot 102, file 0x17f58: andi $r6, $r6, #0x7fff
    Ex9Jal(0xfb8c), // slot 103, file 0x17f5c: jal PC(31,25)|#0xfb8c
    Movi(R0, 0x2fef0), // slot 104, file 0x17f60: movi $r0, #0x2fef0
    Ex9Jal(0x1737c), // slot 105, file 0x17f64: jal PC(31,25)|#0x1737c
    Ori(R0, R0, 1), // slot 106, file 0x17f68: ori $r0, $r0, #1
    Ex9Jal(0x28e2), // slot 107, file 0x17f6c: jal PC(31,25)|#0x28e2
    Movi(R0, 0x2e144), // slot 108, file 0x17f70: movi $r0, #0x2e144
    LwiGp(R0, -444), // slot 109, file 0x17f74: lwi.gp $r0, [ + #-444]
    Lb(R0, R15, R0, 0), // slot 110, file 0x17f78: lb $r0, [$r15 + ($r0 << #0)]
    LbiGp(R0, -5676), // slot 111, file 0x17f7c: lbi.gp $r0, [ + #-5676]
    LwiGp(R1, -4500), // slot 112, file 0x17f80: lwi.gp $r1, [ + #-4500]
    Ex9Jal(0xe8b6), // slot 113, file 0x17f84: jal PC(31,25)|#0xe8b6
    Ex9Jal(0xfaf2), // slot 114, file 0x17f88: jal PC(31,25)|#0xfaf2
    Movi(R7, 0x3f04a), // slot 115, file 0x17f8c: movi $r7, #0x3f04a
    LhiGp(R0, -5542), // slot 116, file 0x17f90: lhi.gp $r0, [ + #-5542]
    LwiGp(R0, -5000), // slot 117, file 0x17f94: lwi.gp $r0, [ + #-5000]
    SwiGp(R0, -5440), // slot 118, file 0x17f98: swi.gp $r0, [ + #-5440]
    Movi(R0, 0xff), // slot 119, file 0x17f9c: movi $r0, #0xff
    Movi(R9, 0x55), // slot 120, file 0x17fa0: movi $r9, #0x55
    Movi(R1, 0x31c20), // slot 121, file 0x17fa4: movi $r1, #0x31c20
    Ex9Jal(0xbd64), // slot 122, file 0x17fa8: jal PC(31,25)|#0xbd64
    Andi(R9, R0, 0xff), // slot 123, file 0x17fac: andi $r9, $r0, #0xff
    LwiGp(R0, -5288), // slot 124, file 0x17fb0: lwi.gp $r0, [ + #-5288]
    Mtsr(R0, SYS_INT_MASK2), // slot 125, file 0x17fb4: mtsr $r0, $int_mask2
    SmwBim(R16, R1, R25, 0x0), // slot 126, file 0x17fb8: smw.bim $r16, [$r1], $r25, #0x0
    Ex9Jal(0x8c04), // slot 127, file 0x17fbc: jal PC(31,25)|#0x8c04
    SbiGp(R0, -6328), // slot 128, file 0x17fc0: sbi.gp $r0, [ + #-6328]
    Ex9Jal(0xdf48), // slot 129, file 0x17fc4: jal PC(31,25)|#0xdf48
    Shi(R0, R31, 0x14), // slot 130, file 0x17fc8: shi $r0, [$sp + #0x14]
    Movi(R0, 0x2edc0), // slot 131, file 0x17fcc: movi $r0, #0x2edc0
    Movi(R0, 0x2fe50), // slot 132, file 0x17fd0: movi $r0, #0x2fe50
    Movi(R0, 0x50), // slot 133, file 0x17fd4: movi $r0, #0x50
    Movi(R1, 0x3f403), // slot 134, file 0x17fd8: movi $r1, #0x3f403
    LwiGp(R0, -5416), // slot 135, file 0x17fdc: lwi.gp $r0, [ + #-5416]
    Sbi(R0, R6, 0x50), // slot 136, file 0x17fe0: sbi $r0, [$r6 + #0x50]
    SbiGp(R0, -4802), // slot 137, file 0x17fe4: sbi.gp $r0, [ + #-4802]
    Sbi(R0, R9, 0), // slot 138, file 0x17fe8: sbi $r0, [$r9 + #0]
    Lbi(R0, R9, 1), // slot 139, file 0x17fec: lbi $r0, [$r9 + #1]
    Movi(R1, 0x64), // slot 140, file 0x17ff0: movi $r1, #0x64
    Lbi(R0, R10, 0), // slot 141, file 0x17ff4: lbi $r0, [$r10 + #0]
    Ex9Jal(0x43a0), // slot 142, file 0x17ff8: jal PC(31,25)|#0x43a0
    Ex9Jal(0xffac), // slot 143, file 0x17ffc: jal PC(31,25)|#0xffac
    Movi(R3, 0x80), // slot 144, file 0x18000: movi $r3, #0x80
    SbiGp(R0, -4899), // slot 145, file 0x18004: sbi.gp $r0, [ + #-4899]
    ShiGp(R0, -5546), // slot 146, file 0x18008: shi.gp $r0, [ + #-5546]
    Shi(R0, R31, 0x16), // slot 147, file 0x1800c: shi $r0, [$sp + #0x16]
    LbiGp(R0, -5697), // slot 148, file 0x18010: lbi.gp $r0, [ + #-5697]
    SbiGp(R0, -5672), // slot 149, file 0x18014: sbi.gp $r0, [ + #-5672]
    LbiGp(R0, 0x13d2), // slot 150, file 0x18018: lbi.gp $r0, [ + #0x13d2]
    Movi(R6, 0x3f04a), // slot 151, file 0x1801c: movi $r6, #0x3f04a
    LwiGp(R0, -440), // slot 152, file 0x18020: lwi.gp $r0, [ + #-440]
    SbiGp(R0, -5686), // slot 153, file 0x18024: sbi.gp $r0, [ + #-5686]
    Ex9Jal(0xe542), // slot 154, file 0x18028: jal PC(31,25)|#0xe542
    LbiGp(R0, -5657), // slot 155, file 0x1802c: lbi.gp $r0, [ + #-5657]
    SbiGp(R2, -6311), // slot 156, file 0x18030: sbi.gp $r2, [ + #-6311]
    Andi(R1, R1, 0x7fff), // slot 157, file 0x18034: andi $r1, $r1, #0x7fff
    Add(R15, R1, R15), // slot 158, file 0x18038: add $r15, $r1, $r15
    LbiGp(R0, -55621), // slot 159, file 0x1803c: lbi.gp $r0, [ + #-55621]
    Lhi(R15, R15, 0), // slot 160, file 0x18040: lhi $r15, [$r15 + #0]
    Movi(R0, 0x20000), // slot 161, file 0x18044: movi $r0, #0x20000
    Sbi(R1, R0, 0x51), // slot 162, file 0x18048: sbi $r1, [$r0 + #0x51]
    LmwBim(R16, R0, R25, 0x0), // slot 163, file 0x1804c: lmw.bim $r16, [$r0], $r25, #0x0
    Ex9Jal(0x10bb8), // slot 164, file 0x18050: jal PC(31,25)|#0x10bb8
    Ex9Jal(0xf63e), // slot 165, file 0x18054: jal PC(31,25)|#0xf63e
    ShiGp(R0, 0x1338), // slot 166, file 0x18058: shi.gp $r0, [ + #0x1338]
    Andi(R0, R0, 0xfe), // slot 167, file 0x1805c: andi $r0, $r0, #0xfe
    Movi(R6, 0x2fef0), // slot 168, file 0x18060: movi $r6, #0x2fef0
    Movi(R0, 0x69), // slot 169, file 0x18064: movi $r0, #0x69
    Ori(R0, R0, 0x20), // slot 170, file 0x18068: ori $r0, $r0, #0x20
    LwiGp(R0, 0x12c8), // slot 171, file 0x1806c: lwi.gp $r0, [ + #0x12c8]
    Divsr(R2, R2, R2, R3), // slot 172, file 0x18070: divsr $r2, $r2, $r2, $r3
    Sbi(R0, R10, 0), // slot 173, file 0x18074: sbi $r0, [$r10 + #0]
    SbiGp(R0, -5641), // slot 174, file 0x18078: sbi.gp $r0, [ + #-5641]
    Movi(R1, 0x2fe50), // slot 175, file 0x1807c: movi $r1, #0x2fe50
    LbiGp(R0, -4912), // slot 176, file 0x18080: lbi.gp $r0, [ + #-4912]
    SwiGp(R0, 0xe60), // slot 177, file 0x18084: swi.gp $r0, [ + #0xe60]
    SbiGp(R0, -5649), // slot 178, file 0x18088: sbi.gp $r0, [ + #-5649]
    Movi(R1, 0x32), // slot 179, file 0x1808c: movi $r1, #0x32
    Movi(R0, 0x32), // slot 180, file 0x18090: movi $r0, #0x32
    Addi(R2, R31, 7), // slot 181, file 0x18094: addi $r2, $sp, #7
    Lbi(R0, R0, 0x51), // slot 182, file 0x18098: lbi $r0, [$r0 + #0x51]
    Movi(R0, 0x30000), // slot 183, file 0x1809c: movi $r0, #0x30000
    Lhsi(R3, R1, 0), // slot 184, file 0x180a0: lhsi $r3, [$r1 + #0]
    Bitci(R0, R0, 2), // slot 185, file 0x180a4: bitci $r0, $r0, #2
    Bitci(R0, R0, 4), // slot 186, file 0x180a8: bitci $r0, $r0, #4
    Ori(R0, R0, 0x10), // slot 187, file 0x180ac: ori $r0, $r0, #0x10
    Movi(R1, 0x26238), // slot 188, file 0x180b0: movi $r1, #0x26238
    Movi(R1, 0x4c), // slot 189, file 0x180b4: movi $r1, #0x4c
    LbsiGp(R0, -4901), // slot 190, file 0x180b8: lbsi.gp $r0, [ + #-4901]
    Ex9Jal(0x7bce), // slot 191, file 0x180bc: jal PC(31,25)|#0x7bce
    Ex9J(0x2e2), // slot 192, file 0x180c0: j PC(31,25)|#0x2e2
    Movi(R7, 0x55), // slot 193, file 0x180c4: movi $r7, #0x55
    Divsr(R0, R0, R1, R0), // slot 194, file 0x180c8: divsr $r0, $r0, $r1, $r0
    SbiGp(R0, -5594), // slot 195, file 0x180cc: sbi.gp $r0, [ + #-5594]
    LbiGp(R0, -5640), // slot 196, file 0x180d0: lbi.gp $r0, [ + #-5640]
    Ex9J(0x250), // slot 197, file 0x180d4: j PC(31,25)|#0x250
    Sw(R3, R1, R2, 2), // slot 198, file 0x180d8: sw $r3, [$r1 + ($r2 << #2)]
    ShiGp(R0, -4080), // slot 199, file 0x180dc: shi.gp $r0, [ + #-4080]
    Movi(R0, 0x78), // slot 200, file 0x180e0: movi $r0, #0x78
    Abs(R2, R2), // slot 201, file 0x180e4: abs $r2, $r2
    Movi(R0, 0x373e8), // slot 202, file 0x180e8: movi $r0, #0x373e8
    LwiGp(R0, -352), // slot 203, file 0x180ec: lwi.gp $r0, [ + #-352]
    Divsr(R1, R1, R0, R1), // slot 204, file 0x180f0: divsr $r1, $r1, $r0, $r1
    LwiGp(R5, -5376), // slot 205, file 0x180f4: lwi.gp $r5, [ + #-5376]
    Bitci(R0, R0, 0x10), // slot 206, file 0x180f8: bitci $r0, $r0, #0x10
    Abs(R1, R1), // slot 207, file 0x180fc: abs $r1, $r1
    Ex9Jal(0xe91a), // slot 208, file 0x18100: jal PC(31,25)|#0xe91a
    Ex9Jal(0xd3d4), // slot 209, file 0x18104: jal PC(31,25)|#0xd3d4
    SbiGp(R0, -5620), // slot 210, file 0x18108: sbi.gp $r0, [ + #-5620]
    LbiGp(R0, -5665), // slot 211, file 0x1810c: lbi.gp $r0, [ + #-5665]
    Ex9Jal(0x105cc), // slot 212, file 0x18110: jal PC(31,25)|#0x105cc
    Slti(R0, R0, 1), // slot 213, file 0x18114: slti $r0, $r0, #1
    Addi(R0, R0, 0x4c), // slot 214, file 0x18118: addi $r0, $r0, #0x4c
    LbiGp(R0, -6327), // slot 215, file 0x1811c: lbi.gp $r0, [ + #-6327]
    LbiGp(R0, -4918), // slot 216, file 0x18120: lbi.gp $r0, [ + #-4918]
    Sbi(R0, R6, 0x41), // slot 217, file 0x18124: sbi $r0, [$r6 + #0x41]
    LbiGp(R0, 0xe84), // slot 218, file 0x18128: lbi.gp $r0, [ + #0xe84]
    AddSlli(R0, R0, R6, 2), // slot 219, file 0x1812c: add_slli $r0, $r0, $r6, #2
    SwiGp(R0, -5216), // slot 220, file 0x18130: swi.gp $r0, [ + #-5216]
    Movi(R0, 0x4c), // slot 221, file 0x18134: movi $r0, #0x4c
    LbiGp(R0, -5592), // slot 222, file 0x18138: lbi.gp $r0, [ + #-5592]
    Andi(R1, R1, 0xfe), // slot 223, file 0x1813c: andi $r1, $r1, #0xfe
    Movi(R2, 0x2c548), // slot 224, file 0x18140: movi $r2, #0x2c548
    ShiGp(R0, -4078), // slot 225, file 0x18144: shi.gp $r0, [ + #-4078]
    LbiGp(R0, -4203), // slot 226, file 0x18148: lbi.gp $r0, [ + #-4203]
    Sbi(R0, R11, 0), // slot 227, file 0x1814c: sbi $r0, [$r11 + #0]
    Ori(R0, R0, 2), // slot 228, file 0x18150: ori $r0, $r0, #2
    Ex9Jal(0xea44), // slot 229, file 0x18154: jal PC(31,25)|#0xea44
    LwiGp(R0, -348), // slot 230, file 0x18158: lwi.gp $r0, [ + #-348]
    Ex9Jal(0xe8b6), // slot 231, file 0x1815c: jal PC(31,25)|#0xe8b6
    Divsr(R1, R1, R1, R7), // slot 232, file 0x18160: divsr $r1, $r1, $r1, $r7
    Ori(R0, R0, 0x80), // slot 233, file 0x18164: ori $r0, $r0, #0x80
    SbiGp(R0, -5657), // slot 234, file 0x18168: sbi.gp $r0, [ + #-5657]
    SbiGp(R0, -5619), // slot 235, file 0x1816c: sbi.gp $r0, [ + #-5619]
    Lbi(R0, R9, 0), // slot 236, file 0x18170: lbi $r0, [$r9 + #0]
    SwiGp(R6, -5072), // slot 237, file 0x18174: swi.gp $r6, [ + #-5072]
    AddSlli(R3, R4, R3, 8), // slot 238, file 0x18178: add_slli $r3, $r4, $r3, #8
    SbiBi(R1, R5, 1), // slot 239, file 0x1817c: sbi.bi $r1, [$r5], #1
    LbiGp(R0, -5603), // slot 240, file 0x18180: lbi.gp $r0, [ + #-5603]
    SbiGp(R0, -4190), // slot 241, file 0x18184: sbi.gp $r0, [ + #-4190]
    Ex9Jal(0x16d7c), // slot 242, file 0x18188: jal PC(31,25)|#0x16d7c
    LbiGp(R0, -4508), // slot 243, file 0x1818c: lbi.gp $r0, [ + #-4508]
    Addi(R1, R31, 0xf), // slot 244, file 0x18190: addi $r1, $sp, #0xf
    LbiGp(R0, -4104), // slot 245, file 0x18194: lbi.gp $r0, [ + #-4104]
    ShiGp(R0, -5492), // slot 246, file 0x18198: shi.gp $r0, [ + #-5492]
    LwiGp(R0, -2980), // slot 247, file 0x1819c: lwi.gp $r0, [ + #-2980]
    Addi(R5, R5, 0xf00), // slot 248, file 0x181a0: addi $r5, $r5, #0xf00
    Ex9Jal(0x854a), // slot 249, file 0x181a4: jal PC(31,25)|#0x854a
    Movi(R1, 0x2c3be), // slot 250, file 0x181a8: movi $r1, #0x2c3be
    Ex9Jal(0xd200), // slot 251, file 0x181ac: jal PC(31,25)|#0xd200
    Addi(R1, R1, 0xfff), // slot 252, file 0x181b0: addi $r1, $r1, #0xfff
    Addi(R0, R31, 0xe), // slot 253, file 0x181b4: addi $r0, $sp, #0xe
    Ex9Jal(0x11854), // slot 254, file 0x181b8: jal PC(31,25)|#0x11854
    Mfsr(R1, SYS_INT_MASK2), // slot 255, file 0x181bc: mfsr $r1, $int_mask2
    ShiGp(R0, -4828), // slot 256, file 0x181c0: shi.gp $r0, [ + #-4828]
    Movi(R0, 0x12c), // slot 257, file 0x181c4: movi $r0, #0x12c
    LwiGp(R0, 0xe60), // slot 258, file 0x181c8: lwi.gp $r0, [ + #0xe60]
    Ex9Jal(0x2ac8), // slot 259, file 0x181cc: jal PC(31,25)|#0x2ac8
    Sb(R2, R1, R0, 0), // slot 260, file 0x181d0: sb $r2, [$r1 + ($r0 << #0)]
    Movi(R1, 0x3f248), // slot 261, file 0x181d4: movi $r1, #0x3f248
    Ex9Jal(0x17a90), // slot 262, file 0x181d8: jal PC(31,25)|#0x17a90
    Movi(R2, 0x2eca8), // slot 263, file 0x181dc: movi $r2, #0x2eca8
    Movi(R4, 0x2d428), // slot 264, file 0x181e0: movi $r4, #0x2d428
    Lw(R0, R9, R1, 2), // slot 265, file 0x181e4: lw $r0, [$r9 + ($r1 << #2)]
    LwiGp(R1, -5440), // slot 266, file 0x181e8: lwi.gp $r1, [ + #-5440]
    Ex9Jal(0x1037c), // slot 267, file 0x181ec: jal PC(31,25)|#0x1037c
    SbiGp(R0, -4753), // slot 268, file 0x181f0: sbi.gp $r0, [ + #-4753]
    SbiGp(R0, -5603), // slot 269, file 0x181f4: sbi.gp $r0, [ + #-5603]
    Shi(R0, R31, 4), // slot 270, file 0x181f8: shi $r0, [$sp + #4]
    Lwi(R6, R5, 0x58), // slot 271, file 0x181fc: lwi $r6, [$r5 + #0x58]
    LbiGp(R1, 0x130d), // slot 272, file 0x18200: lbi.gp $r1, [ + #0x130d]
    LbiGp(R0, -5710), // slot 273, file 0x18204: lbi.gp $r0, [ + #-5710]
    LbiGp(R0, -55474), // slot 274, file 0x18208: lbi.gp $r0, [ + #-55474]
    SbiGp(R0, -4191), // slot 275, file 0x1820c: sbi.gp $r0, [ + #-4191]
    Movi(R2, 0x2ed80), // slot 276, file 0x18210: movi $r2, #0x2ed80
    Movi(R1, -131072), // slot 277, file 0x18214: movi $r1, #-131072
    Ex9Jal(0x10c06), // slot 278, file 0x18218: jal PC(31,25)|#0x10c06
    Divsr(R1, R1, R1, R3), // slot 279, file 0x1821c: divsr $r1, $r1, $r1, $r3
    Sltsi(R15, R0, 0x33), // slot 280, file 0x18220: sltsi $r15, $r0, #0x33
    LwiGp(R1, -4964), // slot 281, file 0x18224: lwi.gp $r1, [ + #-4964]
    Movi(R0, 0x2fef1), // slot 282, file 0x18228: movi $r0, #0x2fef1
    Sw(R5, R6, R3, 2), // slot 283, file 0x1822c: sw $r5, [$r6 + ($r3 << #2)]
    Lhi(R15, R10, 0), // slot 284, file 0x18230: lhi $r15, [$r10 + #0]
    SbiGp(R0, -5695), // slot 285, file 0x18234: sbi.gp $r0, [ + #-5695]
    Ex9Jal(0x950a), // slot 286, file 0x18238: jal PC(31,25)|#0x950a
    Movi(R0, 0x64), // slot 287, file 0x1823c: movi $r0, #0x64
    Movi(R1, 0x3f1c0), // slot 288, file 0x18240: movi $r1, #0x3f1c0
    Andi(R0, R0, 0xf0), // slot 289, file 0x18244: andi $r0, $r0, #0xf0
    Movi(R1, 0x2eca8), // slot 290, file 0x18248: movi $r1, #0x2eca8
    Ex9Jal(0x7bce), // slot 291, file 0x1824c: jal PC(31,25)|#0x7bce
    Movi(R2, 0xffff), // slot 292, file 0x18250: movi $r2, #0xffff
    Movi(R0, 0x2dce8), // slot 293, file 0x18254: movi $r0, #0x2dce8
    Addi(R0, R31, 6), // slot 294, file 0x18258: addi $r0, $sp, #6
    LwiGp(R1, -5416), // slot 295, file 0x1825c: lwi.gp $r1, [ + #-5416]
    Movi(R0, 0x397de), // slot 296, file 0x18260: movi $r0, #0x397de
    Movi(R0, 0x397dc), // slot 297, file 0x18264: movi $r0, #0x397dc
    Andi(R0, R0, 0xfd), // slot 298, file 0x18268: andi $r0, $r0, #0xfd
    Lhi(R1, R31, 0), // slot 299, file 0x1826c: lhi $r1, [$sp + #0]
    Movi(R1, 0x2dce8), // slot 300, file 0x18270: movi $r1, #0x2dce8
    LwiGp(R0, -1524), // slot 301, file 0x18274: lwi.gp $r0, [ + #-1524]
    Mtsr(R1, SYS_INT_PEND2), // slot 302, file 0x18278: mtsr $r1, $int_pend2
    Maddr32(R0, R1, R2), // slot 303, file 0x1827c: maddr32 $r0, $r1, $r2
    Lwi(R1, R6, 0x20), // slot 304, file 0x18280: lwi $r1, [$r6 + #0x20]
    SwiGp(R6, -3772), // slot 305, file 0x18284: swi.gp $r6, [ + #-3772]
    SbiGp(R0, 0x133e), // slot 306, file 0x18288: sbi.gp $r0, [ + #0x133e]
    Addi(R3, R31, 7), // slot 307, file 0x1828c: addi $r3, $sp, #7
    LbiGp(R1, -5619), // slot 308, file 0x18290: lbi.gp $r1, [ + #-5619]
    Sw(R0, R1, R6, 2), // slot 309, file 0x18294: sw $r0, [$r1 + ($r6 << #2)]
    ShiGp(R0, -4804), // slot 310, file 0x18298: shi.gp $r0, [ + #-4804]
    Movi(R1, 0x38d40), // slot 311, file 0x1829c: movi $r1, #0x38d40
    Ex9Jal(0x86ee), // slot 312, file 0x182a0: jal PC(31,25)|#0x86ee
    LbiGp(R0, -4798), // slot 313, file 0x182a4: lbi.gp $r0, [ + #-4798]
    LhiGp(R2, -55724), // slot 314, file 0x182a8: lhi.gp $r2, [ + #-55724]
    LbiGp(R3, -55774), // slot 315, file 0x182ac: lbi.gp $r3, [ + #-55774]
    Andi(R10, R0, 0xff), // slot 316, file 0x182b0: andi $r10, $r0, #0xff
    Abs(R0, R0), // slot 317, file 0x182b4: abs $r0, $r0
    LbiGp(R0, -4911), // slot 318, file 0x182b8: lbi.gp $r0, [ + #-4911]
    Movi(R7, 0x2c7e4), // slot 319, file 0x182bc: movi $r7, #0x2c7e4
    SbiGp(R0, -4104), // slot 320, file 0x182c0: sbi.gp $r0, [ + #-4104]
    SwiGp(R0, -4768), // slot 321, file 0x182c4: swi.gp $r0, [ + #-4768]
    SbiGp(R0, -4483), // slot 322, file 0x182c8: sbi.gp $r0, [ + #-4483]
    Movi(R1, 0x36510), // slot 323, file 0x182cc: movi $r1, #0x36510
    Movi(R2, 0x2e050), // slot 324, file 0x182d0: movi $r2, #0x2e050
    Slt(R0, R1, R0), // slot 325, file 0x182d4: slt $r0, $r1, $r0
    Lbi(R0, R6, 0x7d), // slot 326, file 0x182d8: lbi $r0, [$r6 + #0x7d]
    Seh(R15, R15), // slot 327, file 0x182dc: seh $r15, $r15
    Ex9Jal(0x921c), // slot 328, file 0x182e0: jal PC(31,25)|#0x921c
    Lbi(R0, R31, 7), // slot 329, file 0x182e4: lbi $r0, [$sp + #7]
    SbiGp(R0, -5570), // slot 330, file 0x182e8: sbi.gp $r0, [ + #-5570]
    Ex9Jal(0x12fb2), // slot 331, file 0x182ec: jal PC(31,25)|#0x12fb2
    SwiGp(R1, -5296), // slot 332, file 0x182f0: swi.gp $r1, [ + #-5296]
    Sethi(R5, 0xfff), // slot 333, file 0x182f4: sethi $r5, #0xfff
    Ex9Jal(0x942e), // slot 334, file 0x182f8: jal PC(31,25)|#0x942e
    Movi(R0, 0x38d42), // slot 335, file 0x182fc: movi $r0, #0x38d42
    LwiGp(R0, -5376), // slot 336, file 0x18300: lwi.gp $r0, [ + #-5376]
    Slti(R15, R0, 0x12c), // slot 337, file 0x18304: slti $r15, $r0, #0x12c
    Btst(R0, R0, 1), // slot 338, file 0x18308: btst $r0, $r0, #1
    Movi(R0, 0x52), // slot 339, file 0x1830c: movi $r0, #0x52
    Ex9Jal(0xecf8), // slot 340, file 0x18310: jal PC(31,25)|#0xecf8
    Lbi(R1, R9, 1), // slot 341, file 0x18314: lbi $r1, [$r9 + #1]
    ShiGp(R1, -5498), // slot 342, file 0x18318: shi.gp $r1, [ + #-5498]
    LbiGp(R0, -5629), // slot 343, file 0x1831c: lbi.gp $r0, [ + #-5629]
    Movi(R2, 0x2cd94), // slot 344, file 0x18320: movi $r2, #0x2cd94
    Movi(R6, 0x64), // slot 345, file 0x18324: movi $r6, #0x64
    LhiGp(R0, -4080), // slot 346, file 0x18328: lhi.gp $r0, [ + #-4080]
    Movi(R1, 0x2dc20), // slot 347, file 0x1832c: movi $r1, #0x2dc20
    Movi(R1, 0x2c401), // slot 348, file 0x18330: movi $r1, #0x2c401
    Ex9Jal(0x1046a), // slot 349, file 0x18334: jal PC(31,25)|#0x1046a
    Movi(R1, 0x2c6d0), // slot 350, file 0x18338: movi $r1, #0x2c6d0
    LbiGp(R0, -5618), // slot 351, file 0x1833c: lbi.gp $r0, [ + #-5618]
    Movi(R3, 0x32), // slot 352, file 0x18340: movi $r3, #0x32
    LmwBi(R0, R0, R0, 0x0), // slot 353, file 0x18344: lmw.bi $r0, [$r0], $r0, #0x0
    Mtsr(R0, NOVATEK_CSR_C2), // slot 354, file 0x18348: mtsr $r0, 194
    Btst(R0, R0, 0x1c), // slot 355, file 0x1834c: btst $r0, $r0, #0x1c
    LbiGp(R0, -5664), // slot 356, file 0x18350: lbi.gp $r0, [ + #-5664]
    SbiGp(R0, -4912), // slot 357, file 0x18354: sbi.gp $r0, [ + #-4912]
    Srli(R0, R6, 8), // slot 358, file 0x18358: srli $r0, $r6, #8
    Cmovz(R0, R1, R15), // slot 359, file 0x1835c: cmovz $r0, $r1, $r15
    SbiGp(R0, -5670), // slot 360, file 0x18360: sbi.gp $r0, [ + #-5670]
    AddSlli(R1, R3, R1, 2), // slot 361, file 0x18364: add_slli $r1, $r3, $r1, #2
    Ex9Jal(0x3ac8), // slot 362, file 0x18368: jal PC(31,25)|#0x3ac8
    Ex9Jal(0x15582), // slot 363, file 0x1836c: jal PC(31,25)|#0x15582
    ShiGp(R0, -54758), // slot 364, file 0x18370: shi.gp $r0, [ + #-54758]
    LwiGp(R0, 0x774), // slot 365, file 0x18374: lwi.gp $r0, [ + #0x774]
    LhiGp(R1, -6362), // slot 366, file 0x18378: lhi.gp $r1, [ + #-6362]
    SwiGp(R0, -5084), // slot 367, file 0x1837c: swi.gp $r0, [ + #-5084]
    Movi(R0, 0x26238), // slot 368, file 0x18380: movi $r0, #0x26238
    Ex9Jal(0x8596), // slot 369, file 0x18384: jal PC(31,25)|#0x8596
    LbiGp(R0, -4791), // slot 370, file 0x18388: lbi.gp $r0, [ + #-4791]
    Ex9Jal(0x3e3c), // slot 371, file 0x1838c: jal PC(31,25)|#0x3e3c
    Ex9Jal(0x12388), // slot 372, file 0x18390: jal PC(31,25)|#0x12388
    Maddr32(R16, R7, R2), // slot 373, file 0x18394: maddr32 $r16, $r7, $r2
    Movi(R0, 0x37a28), // slot 374, file 0x18398: movi $r0, #0x37a28
    SwiGp(R0, -5072), // slot 375, file 0x1839c: swi.gp $r0, [ + #-5072]
    Lhi(R0, R31, 4), // slot 376, file 0x183a0: lhi $r0, [$sp + #4]
    Movi(R0, 0x3f7ce), // slot 377, file 0x183a4: movi $r0, #0x3f7ce
    SbiGp(R0, -4203), // slot 378, file 0x183a8: sbi.gp $r0, [ + #-4203]
    Movi(R1, 0x21680), // slot 379, file 0x183ac: movi $r1, #0x21680
    Ex9Jal(0x862e), // slot 380, file 0x183b0: jal PC(31,25)|#0x862e
    Ex9Jal(0xed04), // slot 381, file 0x183b4: jal PC(31,25)|#0xed04
    LbiGp(R0, -4786), // slot 382, file 0x183b8: lbi.gp $r0, [ + #-4786]
    Btst(R0, R0, 3), // slot 383, file 0x183bc: btst $r0, $r0, #3
    LwiGp(R1, -4636), // slot 384, file 0x183c0: lwi.gp $r1, [ + #-4636]
    Ex9Jal(0xd2fc), // slot 385, file 0x183c4: jal PC(31,25)|#0xd2fc
    Movi(R1, 0x34c10), // slot 386, file 0x183c8: movi $r1, #0x34c10
    LbiGp(R0, -5653), // slot 387, file 0x183cc: lbi.gp $r0, [ + #-5653]
    Ex9Jal(0x432c), // slot 388, file 0x183d0: jal PC(31,25)|#0x432c
    Movi(R4, 0x64), // slot 389, file 0x183d4: movi $r4, #0x64
    Movi(R2, 0x64), // slot 390, file 0x183d8: movi $r2, #0x64
    SwiGp(R0, 0xe6c), // slot 391, file 0x183dc: swi.gp $r0, [ + #0xe6c]
    Ex9Jal(0xffb6), // slot 392, file 0x183e0: jal PC(31,25)|#0xffb6
    Ex9Jal(0x9230), // slot 393, file 0x183e4: jal PC(31,25)|#0x9230
    Slli(R2, R2, 0xf), // slot 394, file 0x183e8: slli $r2, $r2, #0xf
    Lwi(R1, R3, 0x50), // slot 395, file 0x183ec: lwi $r1, [$r3 + #0x50]
    Sbi(R1, R0, 0x3c), // slot 396, file 0x183f0: sbi $r1, [$r0 + #0x3c]
    Bitci(R1, R1, 2), // slot 397, file 0x183f4: bitci $r1, $r1, #2
    Ex9Jal(0x93d6), // slot 398, file 0x183f8: jal PC(31,25)|#0x93d6
    Shi(R0, R6, 0x12), // slot 399, file 0x183fc: shi $r0, [$r6 + #0x12]
    Movi(R1, 0x30fa0), // slot 400, file 0x18400: movi $r1, #0x30fa0
    Movi(R8, 0x55), // slot 401, file 0x18404: movi $r8, #0x55
    LbiGp(R0, -5587), // slot 402, file 0x18408: lbi.gp $r0, [ + #-5587]
    SbiGp(R0, -5693), // slot 403, file 0x1840c: sbi.gp $r0, [ + #-5693]
    LwiGp(R1, -4704), // slot 404, file 0x18410: lwi.gp $r1, [ + #-4704]
    Sbi(R9, R0, 0), // slot 405, file 0x18414: sbi $r9, [$r0 + #0]
    LbiGp(R5, 0x130d), // slot 406, file 0x18418: lbi.gp $r5, [ + #0x130d]
    LwiGp(R0, -4060), // slot 407, file 0x1841c: lwi.gp $r0, [ + #-4060]
    Sbi(R1, R0, 0x52), // slot 408, file 0x18420: sbi $r1, [$r0 + #0x52]
    Ex9Jal(0x949e), // slot 409, file 0x18424: jal PC(31,25)|#0x949e
    Addi(R0, R9, 1), // slot 410, file 0x18428: addi $r0, $r9, #1
    Divsr(R0, R0, R2, R0), // slot 411, file 0x1842c: divsr $r0, $r0, $r2, $r0
    OrSrli(R5, R5, R4, 0x1f), // slot 412, file 0x18430: or_srli $r5, $r5, $r4, #0x1f
    Movi(R7, 0x20000), // slot 413, file 0x18434: movi $r7, #0x20000
    Movi(R0, 0x3f41c), // slot 414, file 0x18438: movi $r0, #0x3f41c
    Ex9Jal(0xe26c), // slot 415, file 0x1843c: jal PC(31,25)|#0xe26c
    Sbi(R0, R6, 0x7d), // slot 416, file 0x18440: sbi $r0, [$r6 + #0x7d]
    AddSlli(R15, R9, R15, 1), // slot 417, file 0x18444: add_slli $r15, $r9, $r15, #1
    SbiGp(R0, -4745), // slot 418, file 0x18448: sbi.gp $r0, [ + #-4745]
    Shi(R1, R0, 0x20), // slot 419, file 0x1844c: shi $r1, [$r0 + #0x20]
    Movi(R0, 0x3f00e), // slot 420, file 0x18450: movi $r0, #0x3f00e
    SbiGp(R0, -4786), // slot 421, file 0x18454: sbi.gp $r0, [ + #-4786]
    Ex9Jal(0x899a), // slot 422, file 0x18458: jal PC(31,25)|#0x899a
    Lh(R0, R15, R0, 1), // slot 423, file 0x1845c: lh $r0, [$r15 + ($r0 << #1)]
    Ex9Jal(0xffac), // slot 424, file 0x18460: jal PC(31,25)|#0xffac
    Ex9Jal(0xcee4), // slot 425, file 0x18464: jal PC(31,25)|#0xcee4
    SbiGp(R0, -55474), // slot 426, file 0x18468: sbi.gp $r0, [ + #-55474]
    Movi(R0, 0x96), // slot 427, file 0x1846c: movi $r0, #0x96
    SbiGp(R0, -4790), // slot 428, file 0x18470: sbi.gp $r0, [ + #-4790]
    Ex9Jal(0xecd2), // slot 429, file 0x18474: jal PC(31,25)|#0xecd2
    Movi(R0, 0x38d41), // slot 430, file 0x18478: movi $r0, #0x38d41
    Ori(R0, R0, 8), // slot 431, file 0x1847c: ori $r0, $r0, #8
    Lh(R0, R0, R5, 1), // slot 432, file 0x18480: lh $r0, [$r0 + ($r5 << #1)]
    Slti(R15, R0, 0x33), // slot 433, file 0x18484: slti $r15, $r0, #0x33
    LbiGp(R0, -4756), // slot 434, file 0x18488: lbi.gp $r0, [ + #-4756]
    LwiGp(R1, 0x778), // slot 435, file 0x1848c: lwi.gp $r1, [ + #0x778]
    SbiGp(R0, -1691), // slot 436, file 0x18490: sbi.gp $r0, [ + #-1691]
    LmwBi(R16, R2, R19, 0x0), // slot 437, file 0x18494: lmw.bi $r16, [$r2], $r19, #0x0
    Movi(R1, 0xa5), // slot 438, file 0x18498: movi $r1, #0xa5
    Movi(R1, 0x50), // slot 439, file 0x1849c: movi $r1, #0x50
    Ex9Jal(0x486a), // slot 440, file 0x184a0: jal PC(31,25)|#0x486a
    Ex9Jal(0x10a68), // slot 441, file 0x184a4: jal PC(31,25)|#0x10a68
    Ex9Jal(0x11ca4), // slot 442, file 0x184a8: jal PC(31,25)|#0x11ca4
    LbiGp(R1, -6327), // slot 443, file 0x184ac: lbi.gp $r1, [ + #-6327]
    SwiGp(R0, -5204), // slot 444, file 0x184b0: swi.gp $r0, [ + #-5204]
    Movi(R1, 0x2c730), // slot 445, file 0x184b4: movi $r1, #0x2c730
    LbiGp(R0, -4784), // slot 446, file 0x184b8: lbi.gp $r0, [ + #-4784]
    Movi(R0, 0x3f7c8), // slot 447, file 0x184bc: movi $r0, #0x3f7c8
    LbiGp(R0, -4483), // slot 448, file 0x184c0: lbi.gp $r0, [ + #-4483]
    Lw(R1, R1, R11, 2), // slot 449, file 0x184c4: lw $r1, [$r1 + ($r11 << #2)]
    SbiGp(R0, -5607), // slot 450, file 0x184c8: sbi.gp $r0, [ + #-5607]
    LwiGp(R0, -5072), // slot 451, file 0x184cc: lwi.gp $r0, [ + #-5072]
    Sbi(R0, R9, 0x46), // slot 452, file 0x184d0: sbi $r0, [$r9 + #0x46]
    Movi(R0, 0x2fe60), // slot 453, file 0x184d4: movi $r0, #0x2fe60
    Ex9Jal(0xed40), // slot 454, file 0x184d8: jal PC(31,25)|#0xed40
    Abs(R15, R0), // slot 455, file 0x184dc: abs $r15, $r0
    AddSlli(R0, R3, R0, 2), // slot 456, file 0x184e0: add_slli $r0, $r3, $r0, #2
    Lhsi(R5, R1, 0), // slot 457, file 0x184e4: lhsi $r5, [$r1 + #0]
    LwiGp(R1, 0xe6c), // slot 458, file 0x184e8: lwi.gp $r1, [ + #0xe6c]
    Sltsi(R15, R2, 0x33), // slot 459, file 0x184ec: sltsi $r15, $r2, #0x33
    Ex9Jal(0x88c2), // slot 460, file 0x184f0: jal PC(31,25)|#0x88c2
    Lbi(R4, R0, 0x34), // slot 461, file 0x184f4: lbi $r4, [$r0 + #0x34]
    SbiGp(R0, -4911), // slot 462, file 0x184f8: sbi.gp $r0, [ + #-4911]
    Addi(R0, R0, 0x40), // slot 463, file 0x184fc: addi $r0, $r0, #0x40
    SbiGp(R0, -5669), // slot 464, file 0x18500: sbi.gp $r0, [ + #-5669]
    Andi(R1, R0, 3), // slot 465, file 0x18504: andi $r1, $r0, #3
    LwiGp(R0, -384), // slot 466, file 0x18508: lwi.gp $r0, [ + #-384]
    Lb(R1, R15, R1, 0), // slot 467, file 0x1850c: lb $r1, [$r15 + ($r1 << #0)]
    SwiGp(R0, -5420), // slot 468, file 0x18510: swi.gp $r0, [ + #-5420]
    Lbi(R3, R0, 0xe), // slot 469, file 0x18514: lbi $r3, [$r0 + #0xe]
    Divsr(R1, R1, R1, R11), // slot 470, file 0x18518: divsr $r1, $r1, $r1, $r11
    Sbi(R1, R0, 8), // slot 471, file 0x1851c: sbi $r1, [$r0 + #8]
    Ex9Jal(0x87de), // slot 472, file 0x18520: jal PC(31,25)|#0x87de
    Sbi(R0, R6, 0x40), // slot 473, file 0x18524: sbi $r0, [$r6 + #0x40]
    Swi(R10, R31, 4), // slot 474, file 0x18528: swi $r10, [$sp + #4]
    Addi(R0, R0, 0x240), // slot 475, file 0x1852c: addi $r0, $r0, #0x240
    LbiGp(R5, -4559), // slot 476, file 0x18530: lbi.gp $r5, [ + #-4559]
    ShiGp(R1, -5534), // slot 477, file 0x18534: shi.gp $r1, [ + #-5534]
    Lwi(R2, R7, 0x28), // slot 478, file 0x18538: lwi $r2, [$r7 + #0x28]
    Lwi(R2, R7, 0x30), // slot 479, file 0x1853c: lwi $r2, [$r7 + #0x30]
    Srai(R3, R2, 0x1f), // slot 480, file 0x18540: srai $r3, $r2, #0x1f
    SbiGp(R0, -5600), // slot 481, file 0x18544: sbi.gp $r0, [ + #-5600]
    Movi(R1, 0x2c434), // slot 482, file 0x18548: movi $r1, #0x2c434
    Lhi(R0, R31, 2), // slot 483, file 0x1854c: lhi $r0, [$sp + #2]
    Movi(R1, 0x2c39a), // slot 484, file 0x18550: movi $r1, #0x2c39a
    Movi(R1, 0x21810), // slot 485, file 0x18554: movi $r1, #0x21810
    Ex9Jal(0x950a), // slot 486, file 0x18558: jal PC(31,25)|#0x950a
    Movi(R1, 0x2dba8), // slot 487, file 0x1855c: movi $r1, #0x2dba8
    SwiGp(R1, -3776), // slot 488, file 0x18560: swi.gp $r1, [ + #-3776]
    Ex9Jal(0x11966), // slot 489, file 0x18564: jal PC(31,25)|#0x11966
    LwiGp(R0, -5336), // slot 490, file 0x18568: lwi.gp $r0, [ + #-5336]
    LwiGp(R0, -4392), // slot 491, file 0x1856c: lwi.gp $r0, [ + #-4392]
    Movi(R2, 0xb6), // slot 492, file 0x18570: movi $r2, #0xb6
    SwiGp(R6, -5440), // slot 493, file 0x18574: swi.gp $r6, [ + #-5440]
    Ex9Jal(0x12f9a), // slot 494, file 0x18578: jal PC(31,25)|#0x12f9a
    Slti(R15, R0, 0xa0), // slot 495, file 0x1857c: slti $r15, $r0, #0xa0
    LwiGp(R1, -444), // slot 496, file 0x18580: lwi.gp $r1, [ + #-444]
    Ex9Jal(0x1934), // slot 497, file 0x18584: jal PC(31,25)|#0x1934
    LbiGp(R1, -4629), // slot 498, file 0x18588: lbi.gp $r1, [ + #-4629]
    Movi(R0, 0x2c4c4), // slot 499, file 0x1858c: movi $r0, #0x2c4c4
    SwiGp(R0, 0xe70), // slot 500, file 0x18590: swi.gp $r0, [ + #0xe70]
    Addi(R2, R31, 6), // slot 501, file 0x18594: addi $r2, $sp, #6
    Sh(R1, R0, R6, 1), // slot 502, file 0x18598: sh $r1, [$r0 + ($r6 << #1)]
    LbiGp(R1, -4825), // slot 503, file 0x1859c: lbi.gp $r1, [ + #-4825]
    LwiGp(R3, -5388), // slot 504, file 0x185a0: lwi.gp $r3, [ + #-5388]
    ShiGp(R0, -5548), // slot 505, file 0x185a4: shi.gp $r0, [ + #-5548]
    Ex9Jal(0x19ba), // slot 506, file 0x185a8: jal PC(31,25)|#0x19ba
    AddSlli(R0, R1, R0, 8), // slot 507, file 0x185ac: add_slli $r0, $r1, $r0, #8
    LhiGp(R0, -54758), // slot 508, file 0x185b0: lhi.gp $r0, [ + #-54758]
    Ex9Jal(0xec90), // slot 509, file 0x185b4: jal PC(31,25)|#0xec90
    LbiGp(R0, -55774), // slot 510, file 0x185b8: lbi.gp $r0, [ + #-55774]
    Maddr32(R2, R1, R4), // slot 511, file 0x185bc: maddr32 $r2, $r1, $r4
];

/// Emit all EX9 table slots with their instruction widths and bounds checked.
pub fn emit() -> Result<Vec<u8>, String> {
    let mut bytes = Vec::with_capacity((SOURCE_END - SOURCE_START) as usize);
    for (index, instruction) in CODE.iter().copied().enumerate() {
        if instruction.width() != 4 {
            return Err(format!("EX9 slot {index} must contain a 32-bit instruction"));
        }
        let at = SOURCE_START.checked_add((index as u32) * 4)
            .ok_or("EX9 table position overflow")?;
        let encoded = instruction.encode(at)?;
        if encoded.len() != 4 {
            return Err(format!("EX9 slot {index} encoded an invalid width"));
        }
        bytes.extend_from_slice(&encoded);
    }
    if bytes.len() != (SOURCE_END - SOURCE_START) as usize {
        return Err("EX9 table does not cover its declared interval".into());
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_geometry_matches_itb_and_four_byte_indexing() {
        assert_eq!(SOURCE_START - ILM_FILE_BIAS, ITB_RUNTIME_BASE);
        assert_eq!(CODE.len(), 512);
        assert_eq!(SOURCE_END - SOURCE_START, 4 * SLOT_COUNT as u32);
        assert!(CODE.iter().all(|instruction| instruction.width() == 4));
        assert_eq!(emit().unwrap().len(), 2048);
    }

    #[test]
    fn table_jump_fields_are_independent_of_table_pc() {
        assert_eq!(CODE[1], Ex9Jal(0x84aa));
        let mut absolute_branch_slots = 0;
        for (index, instruction) in CODE.iter().copied().enumerate() {
            if matches!(instruction, Ex9J(..) | Ex9Jal(..)) {
                absolute_branch_slots += 1;
                let at = SOURCE_START + 4 * index as u32;
                assert_eq!(instruction.encode(at).unwrap(),
                    instruction.encode(0).unwrap(), "slot {index} became PC-relative");
            }
        }
        assert_eq!(absolute_branch_slots, 100);
    }
}
