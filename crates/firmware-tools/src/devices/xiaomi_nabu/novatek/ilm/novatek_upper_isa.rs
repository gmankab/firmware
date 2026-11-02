// SPDX-License-Identifier: GPL-3.0-only
//! Checked NDS32 encodings used by the Novatek upper-ILM reconstruction.
//!
//! Each variant names an instruction and its semantic operands. Instruction
//! bytes are always big-endian, including on a little-endian data target.
//! Displacements passed to memory operations are byte displacements; scaling
//! is checked here. Branch operands are absolute firmware-file offsets. Their
//! differences equal the runtime-address differences within the ILM mapping.
//!
//! Encoding references: GNU binutils 2.30 `opcodes/nds32-asm.c` operand and
//! opcode tables, and `include/opcode/nds32.h`, contributed by Andes Technology.
//! In particular, a compressed four-bit register field maps to r0..r11 and
//! r16..r19, rather than r0..r15. EX9.IT executes an instruction-table entry;
//! it is not assigned a software-interrupt meaning by this encoder.

/// Architectural register identity, never an already-encoded field value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpr {
    R0, R1, R2, R3, R4, R5, R6, R7,
    R8, R9, R10, R11, R12, R13, R14, R15,
    R16, R17, R18, R19, R20, R21, R22, R23,
    R24, R25, R26, R27, R28, R29, R30, R31,
}

impl Gpr {
    pub const fn number(self) -> u32 { self as u32 }

    pub fn new(number: u8) -> Result<Self, String> {
        const REGISTERS: [Gpr; 32] = [
            Gpr::R0, Gpr::R1, Gpr::R2, Gpr::R3, Gpr::R4, Gpr::R5, Gpr::R6, Gpr::R7,
            Gpr::R8, Gpr::R9, Gpr::R10, Gpr::R11, Gpr::R12, Gpr::R13, Gpr::R14, Gpr::R15,
            Gpr::R16, Gpr::R17, Gpr::R18, Gpr::R19, Gpr::R20, Gpr::R21, Gpr::R22, Gpr::R23,
            Gpr::R24, Gpr::R25, Gpr::R26, Gpr::R27, Gpr::R28, Gpr::R29, Gpr::R30, Gpr::R31,
        ];
        REGISTERS.get(number as usize).copied()
            .ok_or_else(|| format!("NDS32 register r{number} is outside r0..r31"))
    }
}


/// GNU `SRIDX(1, 8, 1)` and `SRIDX(1, 9, 1)` system-register selectors.
pub const SYS_INT_MASK2: u16 = 0xc1;
pub const SYS_INT_PEND2: u16 = 0xc9;

/// Width-preserving instructions; no pseudo-instruction relaxation is done.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    /// Highest saved register (r6/r8/r10/r14), extra stack reservation in bytes.
    Push25(Gpr, u16),
    Pop25(Gpr, u16),
    Mov55(Gpr, Gpr),
    Movi55(Gpr, i32),
    Movi(Gpr, i32),
    LwiGp(Gpr, i32),
    SbiGp(Gpr, i32),
    LbiGp(Gpr, i32),
    Swi37Sp(Gpr, u16),
    Ex9It(u16),
    Beq(Gpr, Gpr, u32),
    Bne(Gpr, Gpr, u32),
    Beqz38(Gpr, u32),
    Bnez38(Gpr, u32),
    Bnec(Gpr, i32, u32),
    Beqc(Gpr, i32, u32),
    Sbi333(Gpr, Gpr, u8),
    Lbi333(Gpr, Gpr, u8),
    Sbi(Gpr, Gpr, i32),
    J8(u32),
    Jal(u32),
    Addri36Sp(Gpr, u16),
    Lb(Gpr, Gpr, Gpr, u8),
    Sb(Gpr, Gpr, Gpr, u8),
    Addi45(Gpr, u8),
    Addi(Gpr, Gpr, i32),
    Lwi(Gpr, Gpr, i32),
    Lwi450(Gpr, Gpr),
    Srli45(Gpr, u8),
    Zeb33(Gpr, Gpr),
    /// Last included bit, 0..7; extracts the low (last_bit + 1) bits.
    Fexti33(Gpr, u8),
    /// Signed comparison with implicit result register r15 (TA).
    Slts45(Gpr, Gpr),
    Beqzs8(u32),
    Bnezs8(u32),
    Add333(Gpr, Gpr, Gpr),
    /// Unsigned comparison with immediate; implicit result register r15 (TA).
    Slti45(Gpr, u8),
    Slt45(Gpr, Gpr),
    Ret5(Gpr),
    // Additional physical instructions observed in the upper ILM and EX9 table.
    Add(Gpr, Gpr, Gpr),
    Sub(Gpr, Gpr, Gpr),
    And(Gpr, Gpr, Gpr),
    Xor(Gpr, Gpr, Gpr),
    Or(Gpr, Gpr, Gpr),
    Slt(Gpr, Gpr, Gpr),
    Sll(Gpr, Gpr, Gpr),
    Srl(Gpr, Gpr, Gpr),
    Cmovz(Gpr, Gpr, Gpr),
    Cmovn(Gpr, Gpr, Gpr),
    Max(Gpr, Gpr, Gpr),
    Min(Gpr, Gpr, Gpr),
    Mul(Gpr, Gpr, Gpr),
    Mulr64(Gpr, Gpr, Gpr),
    Maddr32(Gpr, Gpr, Gpr),
    Msubr32(Gpr, Gpr, Gpr),
    AddSlli(Gpr, Gpr, Gpr, u8),
    SubSlli(Gpr, Gpr, Gpr, u8),
    OrSlli(Gpr, Gpr, Gpr, u8),
    OrSrli(Gpr, Gpr, Gpr, u8),
    AddSrli(Gpr, Gpr, Gpr, u8),
    AndSrli(Gpr, Gpr, Gpr, u8),
    Lh(Gpr, Gpr, Gpr, u8),
    Lw(Gpr, Gpr, Gpr, u8),
    Lhs(Gpr, Gpr, Gpr, u8),
    Sh(Gpr, Gpr, Gpr, u8),
    Sw(Gpr, Gpr, Gpr, u8),
    LbBi(Gpr, Gpr, Gpr, u8),
    SbBi(Gpr, Gpr, Gpr, u8),
    Slli(Gpr, Gpr, u8),
    Srli(Gpr, Gpr, u8),
    Srai(Gpr, Gpr, u8),
    Bset(Gpr, Gpr, u8),
    Btgl(Gpr, Gpr, u8),
    Btst(Gpr, Gpr, u8),
    Slli333(Gpr, Gpr, u8),
    Addi333(Gpr, Gpr, u8),
    Subi333(Gpr, Gpr, u8),
    Lwi333(Gpr, Gpr, u8),
    Lhi333(Gpr, Gpr, u8),
    Swi333(Gpr, Gpr, u8),
    Shi333(Gpr, Gpr, u8),
    Seh(Gpr, Gpr),
    Zeh(Gpr, Gpr),
    Abs(Gpr, Gpr),
    Clz(Gpr, Gpr),
    Add45(Gpr, Gpr),
    Sub45(Gpr, Gpr),
    Swi450(Gpr, Gpr),
    Movd44(Gpr, Gpr),
    Zeh33(Gpr, Gpr),
    Seb33(Gpr, Gpr),
    Seh33(Gpr, Gpr),
    Xlsb33(Gpr, Gpr),
    X11b33(Gpr, Gpr),
    Neg33(Gpr, Gpr),
    Not33(Gpr, Gpr),
    Mul33(Gpr, Gpr),
    Xor33(Gpr, Gpr),
    And33(Gpr, Gpr),
    Or33(Gpr, Gpr),
    Divsr(Gpr, Gpr, Gpr, Gpr),
    Divr(Gpr, Gpr, Gpr, Gpr),
    Subri(Gpr, Gpr, i32),
    Andi(Gpr, Gpr, i32),
    Xori(Gpr, Gpr, i32),
    Ori(Gpr, Gpr, i32),
    Slti(Gpr, Gpr, i32),
    Sltsi(Gpr, Gpr, i32),
    Bitci(Gpr, Gpr, i32),
    Lbi(Gpr, Gpr, i32),
    Lhi(Gpr, Gpr, i32),
    Shi(Gpr, Gpr, i32),
    Swi(Gpr, Gpr, i32),
    LbiBi(Gpr, Gpr, i32),
    LhiBi(Gpr, Gpr, i32),
    LwiBi(Gpr, Gpr, i32),
    SbiBi(Gpr, Gpr, i32),
    ShiBi(Gpr, Gpr, i32),
    SwiBi(Gpr, Gpr, i32),
    Lbsi(Gpr, Gpr, i32),
    Lhsi(Gpr, Gpr, i32),
    LhsiBi(Gpr, Gpr, i32),
    Sethi(Gpr, i32),
    LhiGp(Gpr, i32),
    LhsiGp(Gpr, i32),
    ShiGp(Gpr, i32),
    SwiGp(Gpr, i32),
    LbsiGp(Gpr, i32),
    Mfsr(Gpr, u16),
    Mtsr(Gpr, u16),
    Lwi37(Gpr, u16),
    Lwi37Sp(Gpr, u16),
    LmwBi(Gpr, Gpr, Gpr, u8),
    LmwBim(Gpr, Gpr, Gpr, u8),
    SmwBi(Gpr, Gpr, Gpr, u8),
    SmwBim(Gpr, Gpr, Gpr, u8),
    SmwAdm(Gpr, Gpr, Gpr, u8),
    Beqz(Gpr, u32),
    Bnez(Gpr, u32),
    Bgez(Gpr, u32),
    Bltz(Gpr, u32),
    Bgtz(Gpr, u32),
    Blez(Gpr, u32),
    Beqs38(Gpr, u32),
    Bnes38(Gpr, u32),
    J(u32),
    Ifcall(u32),
    Ifcall9(u32),
    Ex9J(u32),
    Ex9Jal(u32),
    Jr(Gpr),
    Jr5(Gpr),
    Subi45(Gpr, u8),
    Srai45(Gpr, u8),
    Sltsi45(Gpr, u8),
    Movpi45(Gpr, u8),
    Bmski33(Gpr, u8),
    Sub333(Gpr, Gpr, Gpr),
    Addi10Sp(i32),
    Nop,
    Nop16,
    Ifret16,

}

fn small_register(reg: Gpr) -> Result<u32, String> {
    if reg.number() <= 7 { Ok(reg.number()) }
    else { Err(format!("compressed three-bit field cannot encode {reg:?}")) }
}

fn compact_register(reg: Gpr) -> Result<u32, String> {
    match reg.number() {
        n @ 0..=11 => Ok(n),
        n @ 16..=19 => Ok(n - 4),
        _ => Err(format!("compressed four-bit field cannot encode {reg:?}")),
    }
}

fn signed_field(value: i64, bits: u32, name: &str) -> Result<u32, String> {
    let limit = 1_i64 << (bits - 1);
    if !(-limit..limit).contains(&value) {
        return Err(format!("{name}: {value} does not fit a signed {bits}-bit field"));
    }
    Ok((value as u32) & ((1_u32 << bits) - 1))
}

fn unsigned_field(value: u32, bits: u32, name: &str) -> Result<u32, String> {
    if value >= 1_u32 << bits {
        return Err(format!("{name}: {value} does not fit an unsigned {bits}-bit field"));
    }
    Ok(value)
}

fn signed_scaled(value: i32, bits: u32, shift: u32, name: &str) -> Result<u32, String> {
    let scale = 1_i64 << shift;
    if i64::from(value) % scale != 0 {
        return Err(format!("{name}: displacement {value} must be a multiple of {scale}"));
    }
    signed_field(i64::from(value) / scale, bits, name)
}

fn unsigned_scaled(value: u16, bits: u32, shift: u32, name: &str) -> Result<u32, String> {
    let scale = 1_u32 << shift;
    if u32::from(value) % scale != 0 {
        return Err(format!("{name}: displacement {value} must be a multiple of {scale}"));
    }
    unsigned_field(u32::from(value) / scale, bits, name)
}

fn branch_field(at: u32, target: u32, bits: u32) -> Result<u32, String> {
    if (at | target) & 1 != 0 {
        return Err("NDS32 branch source and target must be halfword aligned".into());
    }
    // Widen before subtracting: a backward branch must never wrap at zero.
    signed_field((i64::from(target) - i64::from(at)) / 2, bits, "branch displacement")
}

fn push_register(last: Gpr) -> Result<u32, String> {
    match last {
        Gpr::R6 => Ok(0), Gpr::R8 => Ok(1), Gpr::R10 => Ok(2), Gpr::R14 => Ok(3),
        _ => Err("PUSH25/POP25 highest register must be r6, r8, r10 or r14".into()),
    }
}

fn unsigned_i32(value: i32, bits: u32, name: &str) -> Result<u32, String> {
    let value = u32::try_from(value).map_err(|_| format!("{name}: negative unsigned operand"))?;
    unsigned_field(value, bits, name)
}

fn even_register(reg: Gpr) -> Result<u32, String> {
    if reg.number() & 1 != 0 {
        return Err(format!("register-pair base {reg:?} must be even"));
    }
    Ok(reg.number() / 2)
}

fn multiple_register_fields(first: Gpr, base: Gpr, last: Gpr, mask: u8) -> Result<u32, String> {
    if first.number() > last.number() {
        return Err("LMW/SMW first register must not exceed last register".into());
    }
    Ok(first.number() << 20 | base.number() << 15 | last.number() << 10
        | unsigned_field(u32::from(mask), 4, "LMW/SMW special-register mask")? << 6)
}

fn instruction_table_jump(target_runtime: u32) -> Result<u32, String> {
    if target_runtime & 1 != 0 {
        return Err("EX9 table jump target must be halfword aligned".into());
    }
    // EX9 J/JAL joins these low 25 address bits with the calling PC's high
    // seven bits. It does not branch relative to the table-slot address.
    unsigned_field(target_runtime >> 1, 24, "EX9 absolute target low 25 bits")
}

impl Instruction {
    pub const fn width(self) -> usize {
        use Instruction::*;
        match self {
            Push25(..) | Pop25(..) | Mov55(..) | Movi55(..) | Swi37Sp(..)
            | Ex9It(..) | Beqz38(..) | Bnez38(..) | Sbi333(..) | Lbi333(..)
            | J8(..) | Addri36Sp(..) | Addi45(..) | Lwi450(..) | Srli45(..)
            | Zeb33(..) | Fexti33(..) | Slts45(..) | Beqzs8(..) | Bnezs8(..)
            | Add333(..) | Slti45(..) | Slt45(..) | Ret5(..) | Slli333(..)
            | Addi333(..) | Subi333(..) | Lwi333(..) | Lhi333(..) | Swi333(..)
            | Shi333(..) | Add45(..) | Sub45(..) | Swi450(..) | Movd44(..)
            | Zeh33(..) | Seb33(..) | Seh33(..) | Xlsb33(..) | X11b33(..)
            | Neg33(..) | Not33(..) | Mul33(..) | Xor33(..) | And33(..)
            | Or33(..) | Lwi37(..) | Lwi37Sp(..) | Beqs38(..) | Bnes38(..)
            | Ifcall9(..) | Jr5(..) | Subi45(..) | Srai45(..) | Sltsi45(..)
            | Movpi45(..) | Bmski33(..) | Sub333(..) | Addi10Sp(..)
            | Nop16 | Ifret16 => 2,
            _ => 4,
        }
    }

    /// Encode at the start of an instruction, without implicit PC advance.
    pub fn encode(self, at_file: u32) -> Result<Vec<u8>, String> {
        use Instruction::*;
        if at_file & 1 != 0 {
            return Err(format!("instruction offset 0x{at_file:x} is not halfword aligned"));
        }
        at_file.checked_add(self.width() as u32)
            .ok_or("instruction extends past the u32 address space")?;

        let word: u32 = match self {
            Push25(last, extra) | Pop25(last, extra) => {
                let base = if matches!(self, Pop25(..)) { 0xfc80 } else { 0xfc00 };
                base | push_register(last)? << 5
                    | unsigned_scaled(extra, 5, 3, "PUSH25/POP25 stack reservation")?
            }
            Mov55(rt, ra) => 0x8000 | rt.number() << 5 | ra.number(),
            Movi55(rt, immediate) => 0x8400 | rt.number() << 5
                | signed_field(i64::from(immediate), 5, "MOVI55 immediate")?,
            Movi(rt, immediate) => (0x22 << 25) | rt.number() << 20
                | signed_field(i64::from(immediate), 20, "MOVI immediate")?,
            LwiGp(rt, displacement) => (0x1e << 25) | (6 << 17) | rt.number() << 20
                | signed_scaled(displacement, 17, 2, "LWI.GP")?,
            SbiGp(rt, displacement) | LbiGp(rt, displacement) => {
                let opcode = if matches!(self, SbiGp(..)) { 0x1f } else { 0x17 };
                (opcode << 25) | rt.number() << 20
                    | signed_field(i64::from(displacement), 19, "byte GP displacement")?
            }
            Swi37Sp(rt, displacement) => 0xf080 | small_register(rt)? << 8
                | unsigned_scaled(displacement, 7, 2, "SWI37.SP")?,
            Ex9It(index) => {
                let index = unsigned_field(u32::from(index), 9, "EX9.IT index")?;
                if index < 32 { 0xdd40 | index } else { 0xea00 | index }
            }
            Beq(rt, ra, target) | Bne(rt, ra, target) => {
                let condition = if matches!(self, Bne(..)) { 1 << 14 } else { 0 };
                (0x26 << 25) | rt.number() << 20 | ra.number() << 15 | condition
                    | branch_field(at_file, target, 14)?
            }
            Beqz38(rt, target) | Bnez38(rt, target) => {
                let base = if matches!(self, Bnez38(..)) { 0xc800 } else { 0xc000 };
                base | small_register(rt)? << 8 | branch_field(at_file, target, 8)?
            }
            Bnec(rt, immediate, target) | Beqc(rt, immediate, target) => {
                let condition = if matches!(self, Bnec(..)) { 1 << 19 } else { 0 };
                (0x2d << 25) | rt.number() << 20 | condition
                    | signed_field(i64::from(immediate), 11, "BR3 comparison constant")? << 8
                    | branch_field(at_file, target, 8)?
            }
            Sbi333(rt, ra, displacement) | Lbi333(rt, ra, displacement) => {
                let base = if matches!(self, Sbi333(..)) { 0xae00 } else { 0xa600 };
                base | small_register(rt)? << 6 | small_register(ra)? << 3
                    | unsigned_field(u32::from(displacement), 3, "byte 333 displacement")?
            }
            Sbi(rt, ra, displacement) => (0x08 << 25) | rt.number() << 20 | ra.number() << 15
                | signed_field(i64::from(displacement), 15, "SBI displacement")?,
            J8(target) => 0xd500 | branch_field(at_file, target, 8)?,
            Jal(target) => (0x24 << 25) | (1 << 24) | branch_field(at_file, target, 24)?,
            Addri36Sp(rt, displacement) => 0xb000 | small_register(rt)? << 6
                | unsigned_scaled(displacement, 6, 2, "ADDRI36.SP")?,
            Lb(rt, ra, rb, shift) | Sb(rt, ra, rb, shift) => {
                let sub = if matches!(self, Sb(..)) { 8 } else { 0 };
                (0x1c << 25) | rt.number() << 20 | ra.number() << 15 | rb.number() << 10
                    | unsigned_field(u32::from(shift), 2, "MEM index shift")? << 8 | sub
            }
            Addi45(rt, immediate) | Srli45(rt, immediate) | Slti45(rt, immediate) => {
                let base = match self { Addi45(..) => 0x8c00, Srli45(..) => 0x9200, _ => 0xe600 };
                base | compact_register(rt)? << 5
                    | unsigned_field(u32::from(immediate), 5, "compact immediate")?
            }
            Addi(rt, ra, immediate) => (0x28 << 25) | rt.number() << 20 | ra.number() << 15
                | signed_field(i64::from(immediate), 15, "ADDI immediate")?,
            Lwi(rt, ra, displacement) => (0x02 << 25) | rt.number() << 20 | ra.number() << 15
                | signed_scaled(displacement, 15, 2, "LWI")?,
            Lwi450(rt, ra) => 0xb400 | compact_register(rt)? << 5 | ra.number(),
            Zeb33(rt, ra) => 0x9600 | small_register(rt)? << 6 | small_register(ra)? << 3,
            Fexti33(rt, last_bit) => 0x9607 | small_register(rt)? << 6
                | unsigned_field(u32::from(last_bit), 3, "FEXTI33 last bit")? << 3,
            Slts45(rt, ra) | Slt45(rt, ra) => {
                let base = if matches!(self, Slt45(..)) { 0xe200 } else { 0xe000 };
                base | compact_register(rt)? << 5 | ra.number()
            }
            Beqzs8(target) => 0xe800 | branch_field(at_file, target, 8)?,
            Bnezs8(target) => 0xe900 | branch_field(at_file, target, 8)?,
            Add333(rt, ra, rb) => 0x9800 | small_register(rt)? << 6
                | small_register(ra)? << 3 | small_register(rb)?,
            Ret5(ra) => 0xdd80 | ra.number(),
            Add(rt, ra, rb) | Sub(rt, ra, rb) | And(rt, ra, rb) | Xor(rt, ra, rb) | Or(rt, ra, rb) | Slt(rt, ra, rb) | Sll(rt, ra, rb) | Srl(rt, ra, rb) | Cmovz(rt, ra, rb) | Cmovn(rt, ra, rb) => {
                let sub = match self {
                    Add(..) => 0x0,
                    Sub(..) => 0x1,
                    And(..) => 0x2,
                    Xor(..) => 0x3,
                    Or(..) => 0x4,
                    Slt(..) => 0x6,
                    Sll(..) => 0xc,
                    Srl(..) => 0xd,
                    Cmovz(..) => 0x1a,
                    Cmovn(..) => 0x1b,
                    _ => unreachable!(),
                };
                (0x20 << 25) | rt.number() << 20 | ra.number() << 15 | rb.number() << 10 | sub
            }
            AddSlli(rt, ra, rb, shift) | SubSlli(rt, ra, rb, shift) | OrSlli(rt, ra, rb, shift) | OrSrli(rt, ra, rb, shift) | AddSrli(rt, ra, rb, shift) | AndSrli(rt, ra, rb, shift) => {
                let sub = match self {
                    AddSlli(..) => 0x0,
                    SubSlli(..) => 0x1,
                    OrSlli(..) => 0x4,
                    OrSrli(..) => 0x15,
                    AddSrli(..) => 0x1c,
                    AndSrli(..) => 0x1e,
                    _ => unreachable!(),
                };
                (0x20 << 25) | rt.number() << 20 | ra.number() << 15 | rb.number() << 10
                    | unsigned_field(u32::from(shift), 5, "ALU shift")? << 5 | sub
            }
            Slli(rt, ra, shift) | Srli(rt, ra, shift) | Srai(rt, ra, shift) => {
                let sub = match self {
                    Slli(..) => 0x8,
                    Srli(..) => 0x9,
                    Srai(..) => 0xa,
                    _ => unreachable!(),
                };
                (0x20 << 25) | rt.number() << 20 | ra.number() << 15
                    | unsigned_field(u32::from(shift), 5, "shift immediate")? << 10 | sub
            }
            Seh(rt, ra) | Zeh(rt, ra) => {
                let sub = if matches!(self, Seh(..)) { 0x11 } else { 0x13 };
                (0x20 << 25) | rt.number() << 20 | ra.number() << 15 | sub
            }
            Divsr(rt, rd, ra, rb) | Divr(rt, rd, ra, rb) => {
                let sub = if matches!(self, Divsr(..)) { 0x16 } else { 0x17 };
                (0x20 << 25) | rt.number() << 20 | rd.number() << 5
                    | ra.number() << 15 | rb.number() << 10 | sub
            }
            Max(rt, ra, rb) | Min(rt, ra, rb) | Mul(rt, ra, rb) | Mulr64(rt, ra, rb) | Maddr32(rt, ra, rb) | Msubr32(rt, ra, rb) => {
                let sub = match self {
                    Max(..) => 0x0,
                    Min(..) => 0x1,
                    Mul(..) => 0x24,
                    Mulr64(..) => 0x69,
                    Maddr32(..) => 0x73,
                    Msubr32(..) => 0x75,
                    _ => unreachable!(),
                };
                (0x21 << 25) | rt.number() << 20 | ra.number() << 15 | rb.number() << 10 | sub
            }
            Abs(rt, ra) | Clz(rt, ra) => {
                let sub = if matches!(self, Abs(..)) { 3 } else { 7 };
                (0x21 << 25) | rt.number() << 20 | ra.number() << 15 | sub
            }
            Bset(rt, ra, bit) | Btgl(rt, ra, bit) | Btst(rt, ra, bit) => {
                let sub = match self {
                    Bset(..) => 0x8,
                    Btgl(..) => 0xa,
                    Btst(..) => 0xb,
                    _ => unreachable!(),
                };
                (0x21 << 25) | rt.number() << 20 | ra.number() << 15
                    | unsigned_field(u32::from(bit), 5, "bit number")? << 10 | sub
            }
            Subri(rt, ra, immediate) | Slti(rt, ra, immediate) | Sltsi(rt, ra, immediate) => {
                let opcode = match self {
                    Subri(..) => 0x29,
                    Slti(..) => 0x2e,
                    Sltsi(..) => 0x2f,
                    _ => unreachable!(),
                };
                (opcode << 25) | rt.number() << 20 | ra.number() << 15
                    | signed_field(i64::from(immediate), 15, "arithmetic immediate")?
            }
            Andi(rt, ra, immediate) | Xori(rt, ra, immediate) | Ori(rt, ra, immediate) | Bitci(rt, ra, immediate) => {
                let opcode = match self {
                    Andi(..) => 0x2a,
                    Xori(..) => 0x2b,
                    Ori(..) => 0x2c,
                    Bitci(..) => 0x33,
                    _ => unreachable!(),
                };
                (opcode << 25) | rt.number() << 20 | ra.number() << 15
                    | unsigned_i32(immediate, 15, "logical immediate")?
            }
            Sethi(rt, immediate) => (0x23 << 25) | rt.number() << 20
                | unsigned_i32(immediate, 20, "SETHI immediate")?,
            LhiGp(rt, displacement) | LhsiGp(rt, displacement) | ShiGp(rt, displacement) | SwiGp(rt, displacement) => {
                let sub = match self {
                    LhiGp(..) => 0x0,
                    LhsiGp(..) => 0x2,
                    ShiGp(..) => 0x4,
                    SwiGp(..) => 0x7,
                    _ => unreachable!(),
                };
                let (bits, shift) = if matches!(self, SwiGp(..)) { (17, 2) } else { (18, 1) };
                (0x1e << 25) | sub << 17 | rt.number() << 20
                    | signed_scaled(displacement, bits, shift, "GP displacement")?
            }
            LbsiGp(rt, displacement) => (0x17 << 25) | (1 << 19) | rt.number() << 20
                | signed_field(i64::from(displacement), 19, "LBSI.GP displacement")?,
            Lbi(rt, ra, displacement) | Lhi(rt, ra, displacement) | Shi(rt, ra, displacement) | Swi(rt, ra, displacement) | LbiBi(rt, ra, displacement) | LhiBi(rt, ra, displacement) | LwiBi(rt, ra, displacement) | SbiBi(rt, ra, displacement) | ShiBi(rt, ra, displacement) | SwiBi(rt, ra, displacement) | Lbsi(rt, ra, displacement) | Lhsi(rt, ra, displacement) | LhsiBi(rt, ra, displacement) => {
                let (opcode, shift) = match self {
                    Lbi(..) => (0x0, 0),
                    Lhi(..) => (0x1, 1),
                    Shi(..) => (0x9, 1),
                    Swi(..) => (0xa, 2),
                    LbiBi(..) => (0x4, 0),
                    LhiBi(..) => (0x5, 1),
                    LwiBi(..) => (0x6, 2),
                    SbiBi(..) => (0xc, 0),
                    ShiBi(..) => (0xd, 1),
                    SwiBi(..) => (0xe, 2),
                    Lbsi(..) => (0x10, 0),
                    Lhsi(..) => (0x11, 1),
                    LhsiBi(..) => (0x15, 1),
                    _ => unreachable!(),
                };
                (opcode << 25) | rt.number() << 20 | ra.number() << 15
                    | signed_scaled(displacement, 15, shift, "memory displacement")?
            }
            Lh(rt, ra, rb, shift) | Lw(rt, ra, rb, shift) | Lhs(rt, ra, rb, shift) | Sh(rt, ra, rb, shift) | Sw(rt, ra, rb, shift) | LbBi(rt, ra, rb, shift) | SbBi(rt, ra, rb, shift) => {
                let sub = match self {
                    Lh(..) => 0x1,
                    Lw(..) => 0x2,
                    Lhs(..) => 0x11,
                    Sh(..) => 0x9,
                    Sw(..) => 0xa,
                    LbBi(..) => 0x4,
                    SbBi(..) => 0xc,
                    _ => unreachable!(),
                };
                (0x1c << 25) | rt.number() << 20 | ra.number() << 15 | rb.number() << 10
                    | unsigned_field(u32::from(shift), 2, "memory index shift")? << 8 | sub
            }
            Mfsr(rt, index) | Mtsr(rt, index) => {
                let sub = if matches!(self, Mfsr(..)) { 2 } else { 3 };
                (0x32 << 25) | rt.number() << 20
                    | unsigned_field(u32::from(index), 10, "system-register index")? << 10 | sub
            }
            LmwBi(first, base, last, mask) | LmwBim(first, base, last, mask) | SmwBi(first, base, last, mask) | SmwBim(first, base, last, mask) | SmwAdm(first, base, last, mask) => {
                let sub = match self {
                    LmwBi(..) => 0x0,
                    LmwBim(..) => 0x4,
                    SmwBi(..) => 0x20,
                    SmwBim(..) => 0x24,
                    SmwAdm(..) => 0x3c,
                    _ => unreachable!(),
                };
                (0x1d << 25) | multiple_register_fields(first, base, last, mask)? | sub
            }
            Beqz(rt, target) | Bnez(rt, target) | Bgez(rt, target) | Bltz(rt, target) | Bgtz(rt, target) | Blez(rt, target) => {
                let sub = match self {
                    Beqz(..) => 0x2,
                    Bnez(..) => 0x3,
                    Bgez(..) => 0x4,
                    Bltz(..) => 0x5,
                    Bgtz(..) => 0x6,
                    Blez(..) => 0x7,
                    _ => unreachable!(),
                };
                (0x27 << 25) | rt.number() << 20 | sub << 16 | branch_field(at_file, target, 16)?
            }
            Beqs38(rt, target) | Bnes38(rt, target) => {
                if rt == Gpr::R5 {
                    return Err("BEQS38/BNES38 r5 encodings are reserved for other compact instructions".into());
                }
                let base = if matches!(self, Beqs38(..)) { 0xd000 } else { 0xd800 };
                base | small_register(rt)? << 8 | branch_field(at_file, target, 8)?
            }
            J(target) => (0x24 << 25) | branch_field(at_file, target, 24)?,
            Ifcall(target) => (0x27 << 25) | branch_field(at_file, target, 16)?,
            Ifcall9(target) => {
                if target < at_file || target & 1 != 0 {
                    return Err("IFCALL9 needs a forward, halfword-aligned target".into());
                }
                0xf800 | unsigned_field((target - at_file) / 2, 9, "IFCALL9 forward displacement")?
            }
            Ex9J(target_runtime) | Ex9Jal(target_runtime) => {
                let link = if matches!(self, Ex9Jal(..)) { 1 << 24 } else { 0 };
                (0x24 << 25) | link | instruction_table_jump(target_runtime)?
            }
            Jr(rb) => (0x25 << 25) | rb.number() << 10,
            Jr5(ra) => 0xdd00 | ra.number(),
            Nop => (0x20 << 25) | 9,
            Nop16 => 0x9200,
            Ifret16 => 0x83ff,
            Add45(rt, ra) | Sub45(rt, ra) | Swi450(rt, ra) => {
                let base = match self {
                    Add45(..) => 0x8800,
                    Sub45(..) => 0x8a00,
                    Swi450(..) => 0xb600,
                    _ => unreachable!(),
                };
                base | compact_register(rt)? << 5 | ra.number()
            }
            Subi45(rt, immediate) | Srai45(rt, immediate) | Sltsi45(rt, immediate) => {
                let base = match self {
                    Subi45(..) => 0x8e00,
                    Srai45(..) => 0x9000,
                    Sltsi45(..) => 0xe400,
                    _ => unreachable!(),
                };
                base | compact_register(rt)? << 5
                    | unsigned_field(u32::from(immediate), 5, "compact immediate")?
            }
            Movpi45(rt, immediate) => {
                let field = immediate.checked_sub(16).ok_or("MOVPI45 immediate must be 16..47")?;
                0xfa00 | compact_register(rt)? << 5 | unsigned_field(u32::from(field), 5, "MOVPI45 immediate")?
            }
            Movd44(rt, ra) => 0xfd00 | even_register(rt)? << 4 | even_register(ra)?,
            Addi10Sp(immediate) => 0xec00 | signed_field(i64::from(immediate), 10, "ADDI10.SP immediate")?,
            Lwi333(rt, ra, displacement) | Lhi333(rt, ra, displacement) | Swi333(rt, ra, displacement) | Shi333(rt, ra, displacement) => {
                let (base, shift) = match self {
                    Lwi333(..) => (0xa000, 2),
                    Lhi333(..) => (0xa400, 1),
                    Swi333(..) => (0xa800, 2),
                    Shi333(..) => (0xac00, 1),
                    _ => unreachable!(),
                };
                base | small_register(rt)? << 6 | small_register(ra)? << 3
                    | unsigned_scaled(u16::from(displacement), 3, shift, "compact memory displacement")?
            }
            Lwi37(rt, displacement) | Lwi37Sp(rt, displacement) => {
                let base = if matches!(self, Lwi37(..)) { 0xb800 } else { 0xf000 };
                base | small_register(rt)? << 8 | unsigned_scaled(displacement, 7, 2, "LWI37 displacement")?
            }
            Slli333(rt, ra, immediate) | Addi333(rt, ra, immediate) | Subi333(rt, ra, immediate) => {
                let base = match self {
                    Slli333(..) => 0x9400,
                    Addi333(..) => 0x9c00,
                    Subi333(..) => 0x9e00,
                    _ => unreachable!(),
                };
                base | small_register(rt)? << 6 | small_register(ra)? << 3
                    | unsigned_field(u32::from(immediate), 3, "compact arithmetic immediate")?
            }
            Zeh33(rt, ra) | Seb33(rt, ra) | Seh33(rt, ra) | Xlsb33(rt, ra) | X11b33(rt, ra) | Neg33(rt, ra) | Not33(rt, ra) | Mul33(rt, ra) | Xor33(rt, ra) | And33(rt, ra) | Or33(rt, ra) => {
                let base = match self {
                    Zeh33(..) => 0x9601,
                    Seb33(..) => 0x9602,
                    Seh33(..) => 0x9603,
                    Xlsb33(..) => 0x9604,
                    X11b33(..) => 0x9605,
                    Neg33(..) => 0xfe02,
                    Not33(..) => 0xfe03,
                    Mul33(..) => 0xfe04,
                    Xor33(..) => 0xfe05,
                    And33(..) => 0xfe06,
                    Or33(..) => 0xfe07,
                    _ => unreachable!(),
                };
                base | small_register(rt)? << 6 | small_register(ra)? << 3
            }
            Bmski33(rt, last_bit) => 0x9606 | small_register(rt)? << 6
                | unsigned_field(u32::from(last_bit), 3, "BMSKI33 last bit")? << 3,
            Sub333(rt, ra, rb) => 0x9a00 | small_register(rt)? << 6 | small_register(ra)? << 3 | small_register(rb)?,
        };
        Ok(if self.width() == 2 { (word as u16).to_be_bytes().to_vec() }
            else { word.to_be_bytes().to_vec() })
    }
}

#[cfg(test)]
mod tests {
    use super::{Gpr::*, Instruction::*};

    #[test]
    fn physical_registers_do_not_alias_compressed_register_fields() {
        assert_eq!(Addi45(R16, 1).encode(0).unwrap(), [0x8d, 0x81]);
        assert_eq!(Srli45(R19, 31).encode(0).unwrap(), [0x93, 0xff]);
        assert_eq!(Lwi450(R16, R31).encode(0).unwrap(), [0xb5, 0x9f]);
        for r in [R12, R13, R14, R15, R20, R31] {
            for instruction in [Addi45(r, 1), Srli45(r, 1), Slti45(r, 1),
                Slts45(r, R0), Slt45(r, R0), Lwi450(r, R0)] {
                assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
            }
            // Full-width register fields continue to accept those identities.
            assert!(Mov55(r, R31).encode(0).is_ok());
        }
        assert_eq!(super::Gpr::new(31).unwrap(), R31);
        assert!(super::Gpr::new(32).is_err());
    }

    #[test]
    fn byte_displacements_are_scaled_only_after_alignment_and_range_checks() {
        assert_eq!(LwiGp(R0, -82992).encode(0).unwrap(), [0x3c, 0x0d, 0xae, 0xf4]);
        assert_eq!(Lwi(R0, R6, 80).encode(0).unwrap(), [0x04, 0x03, 0x00, 0x14]);
        for instruction in [LwiGp(R0, -3), LwiGp(R0, 262144), LwiGp(R0, -262148),
            Lwi(R0, R0, 2), Lwi(R0, R0, 65536), Lwi(R0, R0, -65540),
            Swi37Sp(R0, 2), Swi37Sp(R0, 512), Addri36Sp(R0, 2), Addri36Sp(R0, 256)] {
            assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
        }
        for instruction in [LwiGp(R31, -262144), LwiGp(R31, 262140),
            Lwi(R31, R31, -65536), Lwi(R31, R31, 65532),
            Swi37Sp(R7, 508), Addri36Sp(R7, 252)] {
            assert!(instruction.encode(0).is_ok(), "rejected {instruction:?}");
        }
    }

    #[test]
    fn relative_branches_use_instruction_start_and_checked_signed_halfwords() {
        assert_eq!(Jal(0xfffc).encode(0x1003a).unwrap(), [0x49, 0xff, 0xff, 0xe1]);
        assert_eq!(J8(0x10116).encode(0x1015a).unwrap(), [0xd5, 0xde]);
        assert_eq!(Beq(R5, R0, 0x101aa).encode(0x10028).unwrap(), [0x4c, 0x50, 0x00, 0xc1]);
        assert_eq!(Bnec(R0, 32, 0x10056).encode(0x10060).unwrap(), [0x5a, 0x08, 0x20, 0xfb]);
        assert_eq!(J8(0xf00).encode(0x1000).unwrap(), [0xd5, 0x80]);
        assert_eq!(J8(0x10fe).encode(0x1000).unwrap(), [0xd5, 0x7f]);
        assert!(J8(0xefe).encode(0x1000).is_err());
        assert!(J8(0x1100).encode(0x1000).is_err());
        assert!(J8(0x1001).encode(0x1000).is_err());
        assert!(J8(0x1000).encode(0x1001).is_err());
        assert!(Beq(R0, R1, 0x8000).encode(0x4000).is_err());
        assert!(Jal(0x2000000).encode(0).is_err());
        // Widened subtraction must reject huge wraps rather than truncate them.
        assert!(J8(u32::MAX - 1).encode(0).is_err());
        assert!(Jal(0).encode(u32::MAX - 5).is_err());
    }

    #[test]
    fn immediates_do_not_silently_truncate_or_change_signedness() {
        assert_eq!(Movi55(R31, -16).encode(0).unwrap(), [0x87, 0xf0]);
        assert_eq!(Movi55(R16, 9).encode(0).unwrap(), [0x86, 0x09]);
        assert_eq!(Addi(R15, R1, -32).encode(0).unwrap(), [0x50, 0xf0, 0xff, 0xe0]);
        for instruction in [Movi55(R0, -17), Movi55(R0, 16), Movi(R0, -524289),
            Movi(R0, 524288), Addi(R0, R0, -16385), Addi(R0, R0, 16384),
            Bnec(R0, -1025, 0), Beqc(R0, 1024, 0), SbiGp(R0, -262145),
            LbiGp(R0, 262144), Sbi(R0, R0, 16384), Addi45(R0, 32),
            Srli45(R0, 32), Slti45(R0, 32), Fexti33(R0, 8),
            Sbi333(R0, R0, 8), Lb(R0, R0, R0, 4)] {
            assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
        }
    }

    #[test]
    fn ex9_encodings_preserve_the_split_index_domain() {
        assert_eq!(Ex9It(0).encode(0).unwrap(), [0xdd, 0x40]);
        assert_eq!(Ex9It(31).encode(0).unwrap(), [0xdd, 0x5f]);
        assert_eq!(Ex9It(32).encode(0).unwrap(), [0xea, 0x20]);
        assert_eq!(Ex9It(205).encode(0).unwrap(), [0xea, 0xcd]);
        assert_eq!(Ex9It(511).encode(0).unwrap(), [0xeb, 0xff]);
        assert!(Ex9It(512).encode(0).is_err());
    }

    #[test]
    fn stack_encodings_check_register_groups_and_reservation_units() {
        assert_eq!(Push25(R10, 8).encode(0).unwrap(), [0xfc, 0x41]);
        assert_eq!(Pop25(R10, 8).encode(0).unwrap(), [0xfc, 0xc1]);
        assert_eq!(Push25(R14, 248).encode(0).unwrap(), [0xfc, 0x7f]);
        for r in [R0, R7, R9, R12, R16, R31] {
            assert!(Push25(r, 0).encode(0).is_err());
            assert!(Pop25(r, 0).encode(0).is_err());
        }
        assert!(Push25(R6, 1).encode(0).is_err());
        assert!(Pop25(R6, 256).encode(0).is_err());
    }

    #[test]
    fn small_register_and_instruction_address_constraints_are_enforced() {
        for instruction in [Beqz38(R8, 0), Bnez38(R8, 0), Swi37Sp(R8, 0),
            Addri36Sp(R8, 0), Sbi333(R8, R0, 0), Lbi333(R0, R8, 0),
            Zeb33(R0, R8), Fexti33(R8, 0), Add333(R0, R0, R8)] {
            assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
        }
        assert!(Ret5(R30).encode(1).is_err());
        assert!(Movi(R0, 0).encode(u32::MAX - 1).is_err());
        assert_eq!(Ret5(R30).encode(0).unwrap(), [0xdd, 0x9e]);
    }

    #[test]
    fn widths_preserve_full_and_compact_forms() {
        let short = [Push25(R6, 0), Pop25(R6, 0), Mov55(R31, R0), Movi55(R0, 0),
            Swi37Sp(R0, 0), Ex9It(0), Ex9It(511), Beqz38(R0, 0), Bnez38(R0, 0),
            Sbi333(R0, R0, 0), Lbi333(R0, R0, 0), J8(0), Addri36Sp(R0, 0),
            Addi45(R0, 0), Lwi450(R0, R0), Srli45(R0, 0), Zeb33(R0, R0),
            Fexti33(R0, 0), Slts45(R0, R0), Beqzs8(0), Bnezs8(0),
            Add333(R0, R0, R0), Slti45(R0, 0), Slt45(R0, R0), Ret5(R30)];
        let wide = [Movi(R0, 0), LwiGp(R0, 0), SbiGp(R0, 0), LbiGp(R0, 0),
            Beq(R0, R0, 0), Bne(R0, R0, 0), Bnec(R0, 0, 0), Beqc(R0, 0, 0),
            Sbi(R0, R0, 0), Jal(0), Lb(R0, R0, R0, 0), Sb(R0, R0, R0, 0),
            Addi(R0, R0, 0), Lwi(R0, R0, 0)];
        for (instructions, expected) in [(&short[..], 2), (&wide[..], 4)] {
            for instruction in instructions {
                let bytes = instruction.encode(0).unwrap();
                assert_eq!(instruction.width(), expected);
                assert_eq!(bytes.len(), expected);
                assert_eq!(bytes[0] >> 7, if expected == 2 { 1 } else { 0 });
            }
        }
    }

    #[test]
    fn ifcall_forms_have_distinct_signed_and_forward_ranges_without_pc_bias() {
        assert_eq!(Ifcall9(0x102dc).encode(0x102c0).unwrap(), [0xf8, 0x0e]);
        assert_eq!(Ifcall(0x10378).encode(0x10410).unwrap(), [0x4e, 0x00, 0xff, 0xb4]);
        assert_eq!(Ifcall9(0x1000).encode(0x1000).unwrap(), [0xf8, 0x00]);
        assert_eq!(Ifcall9(0x13fe).encode(0x1000).unwrap(), [0xf9, 0xff]);
        for target in [0x0ffe, 0x1001, 0x1400] {
            assert!(Ifcall9(target).encode(0x1000).is_err());
        }
        assert!(Ifcall(0).encode(0x10000).is_ok());
        assert!(Ifcall(0x1fffe).encode(0x10000).is_ok());
        assert!(Ifcall(0x20000).encode(0x10000).is_err());
    }

    #[test]
    fn ex9_table_jumps_store_absolute_low_address_bits_independent_of_slot() {
        let call = Ex9Jal(0x84aa);
        assert_eq!(call.encode(0).unwrap(), [0x49, 0x00, 0x42, 0x55]);
        assert_eq!(call.encode(0).unwrap(), call.encode(0x17dc0).unwrap());
        assert_eq!(Ex9J(0x2aa).encode(0x17dc0).unwrap(), [0x48, 0x00, 0x01, 0x55]);
        assert!(Ex9J(0x1fffffe).encode(0).is_ok());
        assert!(Ex9J(0x2000000).encode(0).is_err());
        assert!(Ex9Jal(0x84ab).encode(0).is_err());
        // A normal relative JAL remains placement-dependent.
        assert_ne!(Jal(0x84aa).encode(0).unwrap(), Jal(0x84aa).encode(0x1000).unwrap());
    }

    #[test]
    fn multiword_and_register_pair_forms_reject_invalid_ranges() {
        assert!(LmwBi(R16, R0, R15, 0).encode(0).is_err());
        assert!(LmwBim(R0, R1, R2, 16).encode(0).is_err());
        assert!(SmwBi(R0, R1, R2, 255).encode(0).is_err());
        assert!(SmwBim(R3, R0, R2, 0).encode(0).is_err());
        assert!(SmwAdm(R31, R31, R31, 10).encode(0).is_ok());
        assert!(Movd44(R0, R30).encode(0).is_ok());
        assert!(Movd44(R1, R0).encode(0).is_err());
        assert!(Movd44(R0, R31).encode(0).is_err());
        assert_eq!(Movd44(R6, R10).encode(0).unwrap(), [0xfd, 0x35]);
    }

    #[test]
    fn new_memory_families_keep_byte_displacements_and_checked_scales() {
        for instruction in [LhiGp(R31, -262144), LhsiGp(R31, 262142),
            ShiGp(R31, -262144), SwiGp(R31, 262140), LbsiGp(R31, -262144),
            Lbi(R31, R31, -16384), Lbsi(R31, R31, 16383),
            LhiBi(R31, R31, 32766), SwiBi(R31, R31, -65536)] {
            assert!(instruction.encode(0).is_ok(), "rejected {instruction:?}");
        }
        for instruction in [LhiGp(R0, 1), LhsiGp(R0, 262144), ShiGp(R0, -262146),
            SwiGp(R0, 2), LbsiGp(R0, 262144), LbiBi(R0, R0, 16384),
            LhsiBi(R0, R0, -3), SwiBi(R0, R0, -65540),
            Lhi333(R0, R1, 1), Lhi333(R0, R1, 16),
            Swi333(R0, R1, 30), Shi333(R0, R1, 16),
            Lwi37(R0, 3), Lwi37Sp(R0, 512), Sw(R0, R1, R2, 4)] {
            assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
        }
    }

    #[test]
    fn arithmetic_and_logical_immediates_have_different_signedness() {
        // SLTI uses an unsigned comparison with a sign-extended immediate.
        assert!(Slti(R1, R2, -1).encode(0).is_ok());
        assert!(Sltsi(R1, R2, -16384).encode(0).is_ok());
        assert!(Subri(R1, R2, 16383).encode(0).is_ok());
        for instruction in [Andi(R1, R2, -1), Ori(R1, R2, 32768),
            Xori(R1, R2, -1), Bitci(R1, R2, 32768),
            Subri(R1, R2, -16385), Slti(R1, R2, 16384),
            Sethi(R0, -1), Sethi(R0, 0x100000),
            Slli(R0, R1, 32), AddSlli(R0, R1, R2, 32),
            Bset(R0, R1, 32), Btgl(R0, R1, 32), Btst(R0, R1, 32)] {
            assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
        }
        assert!(Sethi(R0, 0xfffff).encode(0).is_ok());
    }

    #[test]
    fn new_compact_operands_cannot_alias_reserved_or_unrepresentable_fields() {
        for instruction in [Add45(R12, R0), Sub45(R15, R0), Swi450(R20, R0),
            Sltsi45(R31, 0), Movpi45(R0, 15), Movpi45(R0, 48),
            Slli333(R0, R1, 8), Addi333(R0, R1, 8), Subi333(R0, R1, 8),
            Zeh33(R8, R0), Or33(R0, R8), Bmski33(R0, 8),
            Lwi37(R8, 0), Beqs38(R5, 0), Bnes38(R5, 0),
            Addi10Sp(-513), Addi10Sp(512)] {
            assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
        }
        assert_eq!(Movpi45(R16, 16).encode(0).unwrap(), [0xfb, 0x80]);
        assert_eq!(Movpi45(R19, 47).encode(0).unwrap(), [0xfb, 0xff]);
        assert!(Addi10Sp(-512).encode(0).is_ok());
        assert!(Addi10Sp(511).encode(0).is_ok());
    }

    #[test]
    fn system_register_selectors_are_checked_ten_bit_operands() {
        assert_eq!(Mfsr(R1, super::SYS_INT_MASK2).encode(0).unwrap(), [0x64, 0x13, 0x04, 0x02]);
        assert_eq!(Mtsr(R0, super::SYS_INT_PEND2).encode(0).unwrap(), [0x64, 0x03, 0x24, 0x03]);
        assert!(Mfsr(R31, 1023).encode(0).is_ok());
        assert!(Mfsr(R0, 1024).encode(0).is_err());
        assert!(Mtsr(R0, u16::MAX).encode(0).is_err());
    }

    #[test]
    fn explicit_fixed_instructions_and_new_widths_are_preserved() {
        assert_eq!(Nop.encode(0).unwrap(), [0x40, 0x00, 0x00, 0x09]);
        assert_eq!(Nop16.encode(0).unwrap(), [0x92, 0x00]);
        assert_eq!(Ifret16.encode(0).unwrap(), [0x83, 0xff]);
        for instruction in [Ifcall9(0), Nop16, Ifret16, Movd44(R0, R2),
            Movpi45(R0, 16), Addi10Sp(0), Lwi37(R0, 0), Jr5(R15)] {
            assert_eq!(instruction.width(), 2);
            assert_eq!(instruction.encode(0).unwrap().len(), 2);
        }
        for instruction in [Ifcall(0), Ex9J(0), Ex9Jal(0), Nop, Jr(R15),
            Mfsr(R1, 0), LmwBim(R0, R1, R2, 0), Divsr(R0, R1, R2, R3)] {
            assert_eq!(instruction.width(), 4);
            assert_eq!(instruction.encode(0).unwrap().len(), 4);
        }
    }

}
