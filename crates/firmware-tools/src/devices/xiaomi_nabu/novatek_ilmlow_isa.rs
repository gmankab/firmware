// SPDX-License-Identifier: GPL-3.0-only
//! Checked NDS32 instruction encoders for Agent 17's lower ILM source.
//! Instruction words are big-endian. No OEM byte arrays are accepted here.
//! Opcode fields were checked against Andes-contributed GNU binutils and
//! Ghidra NDS32 SLEIGH. See docs/agent20-17-inherited-encoder-audit.md.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reg(u8);
impl Reg {
    pub const fn new(n: u8) -> Self { assert!(n < 32); Self(n) }
    pub fn number(self) -> u32 { u32::from(self.0) }
    fn small(self) -> Result<u16, String> {
        if self.0 > 7 { Err("NDS32 three-bit register must be r0..r7".into()) }
        else { Ok(u16::from(self.0)) }
    }
    pub fn compact(self) -> Result<u16, String> {
        match self.0 {
            0..=11 => Ok(u16::from(self.0)),
            16..=19 => Ok(u16::from(self.0 - 4)),
            _ => Err("NDS32 compact register must be r0..r11 or r16..r19".into()),
        }
    }
}
pub const R0: Reg = Reg::new(0);
pub const R1: Reg = Reg::new(1);
pub const R2: Reg = Reg::new(2);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csr { Psw, Ivb, IntMask, IntMask2, IntMask3 }
impl Csr {
    fn index(self) -> u32 {
        match self { Self::Psw => 0x80, Self::Ivb => 0x89,
            Self::IntMask => 0xc0, Self::IntMask2 => 0xc1, Self::IntMask3 => 0xc2 }
    }
    fn name(self) -> &'static str {
        match self { Self::Psw => "psw", Self::Ivb => "ivb", Self::IntMask => "int_mask",
            Self::IntMask2 => "int_mask2", Self::IntMask3 => "int_mask3" }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Insn {
    StoreByteGp { source: Reg, address: u32 },
    ReadSystem { dest: Reg, csr: Csr },
    WriteSystem { source: Reg, csr: Csr },
    /// Execute a separately supplied instruction-table entry; not a SWI.
    Ex9(u16),
    LowBit { dest: Reg, source: Reg },
    Move { dest: Reg, source: Reg },
    BranchZero { reg: Reg, nonzero: bool, target: u32 },
    BitClearImmediate { dest: Reg, source: Reg, mask: u16 },
    SetHigh { dest: Reg, value: u32 },
    AddImmediate { dest: Reg, source: Reg, value: i32 },
    MoveImmediate { dest: Reg, value: i32 },
    DataSync,
    Stack { pop: bool, last: Reg, extra: u16 },
}

fn signed(value: i64, bits: u32) -> Result<u32, String> {
    if value < -(1_i64 << (bits-1)) || value >= (1_i64 << (bits-1)) {
        return Err(format!("signed {bits}-bit operand out of range: {value}"));
    }
    Ok((value as u32) & ((1_u32 << bits) - 1))
}
fn displacement(pc: u32, target: u32, bits: u32) -> Result<u32, String> {
    if (pc | target) & 1 != 0 { return Err("unaligned NDS32 branch address".into()); }
    signed((i64::from(target)-i64::from(pc))/2, bits)
}
fn u15(value: u16) -> Result<u32, String> {
    if value >= 0x8000 { Err("unsigned immediate exceeds 15 bits".into()) }
    else { Ok(u32::from(value)) }
}

impl Insn {
    pub fn encode(self, pc: u32, gp: u32) -> Result<Vec<u8>, String> {
        use Insn::*;
        let word = match self {
            StoreByteGp { source, address } => (0x1f << 25) | (source.number() << 20)
                | signed(i64::from(address)-i64::from(gp),19)?,
            ReadSystem { dest, csr } => (0x32 << 25) | (dest.number() << 20) | (csr.index() << 10) | 2,
            WriteSystem { source, csr } => (0x32 << 25) | (source.number() << 20) | (csr.index() << 10) | 3,
            BitClearImmediate { dest, source, mask } => (0x33 << 25) | (dest.number()<<20)
                | (source.number()<<15) | u15(mask)?,
            SetHigh { dest, value } => {
                if value >= 1 << 20 { return Err("SETHI operand exceeds 20 bits".into()); }
                (0x23 << 25) | (dest.number()<<20) | value
            }
            AddImmediate { dest, source, value } => (0x28 << 25) | (dest.number()<<20)
                | (source.number()<<15) | signed(i64::from(value),15)?,
            MoveImmediate { dest, value } => (0x22 << 25) | (dest.number()<<20) | signed(i64::from(value),20)?,
            DataSync => (0x32 << 25) | 8,
            _ => return Ok(self.short(pc)?.to_be_bytes().to_vec()),
        };
        Ok(word.to_be_bytes().to_vec())
    }
    fn short(self, pc: u32) -> Result<u16, String> {
        use Insn::*;
        Ok(match self {
            Ex9(index) => {
                if index > 511 { return Err("EX9 index exceeds 511".into()); }
                if index < 32 { 0xdd40 | index } else { 0xea00 | index }
            }
            LowBit { dest, source } => 0x9604 | dest.small()?<<6 | source.small()?<<3,
            Move { dest, source } => {
                if dest.number()==31 && source.number()==31 {
                    return Err("MOV55 sp,sp is the reserved IFRET16 alias".into());
                }
                0x8000 | (dest.number() as u16)<<5 | source.number() as u16
            }
            BranchZero { reg, nonzero, target } => (if nonzero {0xc800} else {0xc000})
                | reg.small()?<<8 | displacement(pc,target,8)? as u16,
            Stack { pop, last, extra } => {
                let group = match last.number() { 6=>0, 8=>1, 10=>2, 14=>3,
                    _=>return Err("PUSH25/POP25 last register must be r6/r8/r10/r14".into()) };
                if extra % 8 != 0 || extra > 248 { return Err("invalid PUSH25/POP25 reservation".into()); }
                0xfc00 | (u16::from(pop)<<7) | group<<5 | (extra/8)
            }
            _ => return Err("instruction has no 16-bit form".into()),
        })
    }
    /// Exact mnemonic form for GNU 2.25.1, which retains EX9 and IFC support.
    /// Later GNU releases removed those opcode entries. CSR 0xc2 is written
    /// numerically because the older assembler lacks its INT_MASK3 alias.
    pub fn assembly(self, pc: u32, gp: u32) -> String {
        use Insn::*;
        let r=|x: Reg|format!("$r{}",x.number());
        match self {
            StoreByteGp { source,address } => format!("sbi.gp {},[{}]",r(source),i64::from(address)-i64::from(gp)),
            ReadSystem { dest,csr } => format!("mfsr {},{}",r(dest),if csr==Csr::IntMask3 {"0xc2".into()} else {format!("${}",csr.name())}),
            WriteSystem { source,csr } => format!("mtsr {},{}",r(source),if csr==Csr::IntMask3 {"0xc2".into()} else {format!("${}",csr.name())}),
            Ex9(i) => format!("ex9.it {i}"),
            LowBit { dest,source } => format!("xlsb33 {},{}",r(dest),r(source)),
            Move { dest,source } => format!("mov55 {},{}",r(dest),r(source)),
            BranchZero { reg,nonzero,target } => format!("{} {},.{:+}",if nonzero{"bnez38"}else{"beqz38"},r(reg),i64::from(target)-i64::from(pc)),
            BitClearImmediate { dest,source,mask } => format!("bitci {},{},{}",r(dest),r(source),mask),
            SetHigh { dest,value } => format!("sethi {},{}",r(dest),value),
            AddImmediate { dest,source,value } => format!("addi {},{},{}",r(dest),r(source),value),
            MoveImmediate { dest,value } => format!("movi {},{}",r(dest),value),
            DataSync => "dsb".into(),
            Stack { pop,last,extra } => format!("{} {},{}",if pop{"pop25"}else{"push25"},r(last),extra),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compact_register_map_rejects_holes() {
        assert_eq!(Reg::new(11).compact().unwrap(),11);
        assert!(Reg::new(12).compact().is_err());
        assert!(Reg::new(15).compact().is_err());
        assert_eq!(Reg::new(16).compact().unwrap(),12);
        assert_eq!(Reg::new(19).compact().unwrap(),15);
        assert!(Reg::new(20).compact().is_err());
    }
    #[test]
    fn ex9_encoding_boundaries_are_not_software_breaks() {
        for (i,expected) in [(0,0xdd40_u16),(31,0xdd5f),(32,0xea20),(511,0xebff)] {
            assert_eq!(Insn::Ex9(i).encode(0,0).unwrap(),expected.to_be_bytes());
        }
        assert!(Insn::Ex9(512).encode(0,0).is_err());
    }
    #[test]
    fn branch_and_immediate_limits() {
        assert!(Insn::BranchZero{reg:R0,nonzero:false,target:3}.encode(0,0).is_err());
        assert!(Insn::BranchZero{reg:R0,nonzero:false,target:256}.encode(0,0).is_err());
        assert_eq!(Insn::BranchZero{reg:R1,nonzero:false,target:0x484}.encode(0x47e,0).unwrap(),[0xc1,3]);
        assert!(Insn::Move{dest:Reg::new(31),source:Reg::new(31)}.encode(0,0).is_err());
        assert!(Insn::MoveImmediate{dest:R0,value:0x80000}.encode(0,0).is_err());
        assert!(Insn::StoreByteGp{source:R0,address:0x80000}.encode(0,0).is_err());
    }
}
