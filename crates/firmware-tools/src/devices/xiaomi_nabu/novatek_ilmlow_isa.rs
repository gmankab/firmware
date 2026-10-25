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
pub const R3: Reg = Reg::new(3);
pub const R4: Reg = Reg::new(4);
pub const R5: Reg = Reg::new(5);
pub const R6: Reg = Reg::new(6);
pub const LP: Reg = Reg::new(30);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Width { Byte, Half, Word }
impl Width {
    fn scale(self)->u8 {match self{Self::Byte=>1,Self::Half=>2,Self::Word=>4}}
    fn letter(self)->char {match self{Self::Byte=>'b',Self::Half=>'h',Self::Word=>'w'}}
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AluOp { Add, Or }

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
    MoveSmall { dest: Reg, value: i8 },
    LoadWordGp { dest: Reg, address: u32 },
    ShortMemory { store: bool, width: Width, reg: Reg, base: Reg, offset: u8 },
    WordAtRegister { store: bool, reg: Reg, base: Reg },
    WordAtFrame { store: bool, reg: Reg, offset: u16 },
    ShiftRightSmall { reg: Reg, amount: u8 },
    AluShiftLeft { op: AluOp, dest: Reg, left: Reg, right: Reg, shift: u8 },
    VariableShiftLeft { dest: Reg, source: Reg, amount: Reg },
    BranchImmediate { reg: Reg, value: i16, nonzero: bool, target: u32 },
    BranchA5 { reg: Reg, nonzero: bool, target: u32 },
    AddSmall { reg: Reg, value: u8, subtract: bool },
    ZeroExtend { dest: Reg, source: Reg, half: bool },
    BitMaskShort { reg: Reg, bit: u8 },
    CompactOr { dest: Reg, source: Reg },
    OrImmediate { dest: Reg, source: Reg, value: u16 },
    BitSet { dest: Reg, source: Reg, bit: u8 },
    IndexedByte { store: bool, reg: Reg, base: Reg, index: Reg, shift: u8 },
    JumpShort { target: u32 },
    JumpRelative { target: u32, link: bool },
    /// IFC_LP/IFC_ON state differs from conventional JAL/RET.
    InlineCall { target: u32 },
    /// Conditional return through IFC_LP; falls through with IFC_ON clear.
    InlineReturn,
    Return { reg: Reg },
    JumpRegisterLink { reg: Reg },
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
            LoadWordGp { dest,address } => {
                let delta=i64::from(address)-i64::from(gp);
                if delta%4!=0{return Err("unaligned GP word address".into());}
                (0x1e<<25)|(dest.number()<<20)|(6<<17)|signed(delta/4,17)?
            }
            AluShiftLeft { op,dest,left,right,shift } => {
                if shift>31{return Err("ALU shift exceeds 31".into());}
                (0x20<<25)|(dest.number()<<20)|(left.number()<<15)|(right.number()<<10)
                    |(u32::from(shift)<<5)|match op{AluOp::Add=>0,AluOp::Or=>4}
            }
            VariableShiftLeft { dest,source,amount } => (0x20<<25)|(dest.number()<<20)|(source.number()<<15)|(amount.number()<<10)|12,
            BranchImmediate { reg,value,nonzero,target } => (0x2d<<25)|(reg.number()<<20)
                |(u32::from(nonzero)<<19)|(signed(i64::from(value),11)?<<8)|displacement(pc,target,8)?,
            OrImmediate { dest,source,value } => (0x2c<<25)|(dest.number()<<20)|(source.number()<<15)|u15(value)?,
            BitSet { dest,source,bit } => {
                if bit>31{return Err("BSET bit exceeds 31".into());}
                (0x21<<25)|(dest.number()<<20)|(source.number()<<15)|(u32::from(bit)<<10)|8
            }
            IndexedByte { store,reg,base,index,shift } => {
                if shift>3{return Err("indexed byte shift exceeds 3".into());}
                (0x1c<<25)|(reg.number()<<20)|(base.number()<<15)|(index.number()<<10)
                    |(u32::from(shift)<<8)|(u32::from(store)<<3)
            }
            JumpRelative { target,link } => (0x24<<25)|(u32::from(link)<<24)|displacement(pc,target,24)?,
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
            MoveSmall { dest,value } => 0x8400 | (dest.number() as u16)<<5 | signed(i64::from(value),5)? as u16,
            ShortMemory { store,width,reg,base,offset } => {
                if offset%width.scale()!=0 || offset/width.scale()>7{return Err("invalid compact memory offset".into());}
                let operation=match width{Width::Word=>0xa000,Width::Half=>0xa400,Width::Byte=>0xa600};
                operation | (u16::from(store)<<11) | reg.small()?<<6 | base.small()?<<3 | u16::from(offset/width.scale())
            }
            WordAtRegister { store,reg,base } => 0xb400|(u16::from(store)<<9)|(reg.compact()?<<5)|base.number() as u16,
            WordAtFrame { store,reg,offset } => {
                if offset%4!=0 || offset>508{return Err("invalid FP-relative compact word offset".into());}
                0xb800 | reg.small()?<<8 | u16::from(store)<<7 | offset/4
            }
            ShiftRightSmall { reg,amount } => {
                if amount>31{return Err("SRLI45 amount exceeds 31".into());}
                0x9200 | reg.compact()?<<5 | u16::from(amount)
            }
            BranchA5 { reg,nonzero,target } => {
                if reg.number()==5{return Err("r5 selects a reserved BEQS38/BNES38 opcode subspace".into());}
                (if nonzero{0xd800}else{0xd000})|reg.small()?<<8|displacement(pc,target,8)? as u16
            }
            AddSmall { reg,value,subtract } => {
                if value>31{return Err("ADDI45/SUBI45 value exceeds 31".into());}
                0x8c00|u16::from(subtract)<<9|reg.compact()?<<5|u16::from(value)
            }
            ZeroExtend { dest,source,half } => 0x9600|dest.small()?<<6|source.small()?<<3|u16::from(half),
            BitMaskShort { reg,bit } => {
                if bit>7{return Err("BMSKI33 bit exceeds 7".into());}
                0x9606|reg.small()?<<6|u16::from(bit)<<3
            }
            CompactOr { dest,source } => 0xfe07|dest.small()?<<6|source.small()?<<3,
            JumpShort { target } => 0xd500|displacement(pc,target,8)? as u16,
            InlineCall { target } => {
                if (pc|target)&1!=0 || target<pc || target-pc>1022{return Err("invalid IFCALL9 forward target".into());}
                0xf800 | ((target-pc)/2) as u16
            }
            InlineReturn => 0x83ff,
            Return { reg } => 0xdd80|reg.number() as u16,
            JumpRegisterLink { reg } => 0xdd20|reg.number() as u16,
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
            MoveSmall { dest,value } => format!("movi55 {},{}",r(dest),value),
            LoadWordGp { dest,address } => format!("lwi.gp {},[{}]",r(dest),i64::from(address)-i64::from(gp)),
            ShortMemory { store,width,reg,base,offset } => format!("{}{}i333 {},[{}+{}]",if store{"s"}else{"l"},width.letter(),r(reg),r(base),offset),
            WordAtRegister { store,reg,base } => format!("{}wi450 {},[{}]",if store{"s"}else{"l"},r(reg),r(base)),
            WordAtFrame { store,reg,offset } => format!("{}wi37 {},[$fp+{}]",if store{"s"}else{"l"},r(reg),offset),
            ShiftRightSmall { reg,amount } => format!("srli45 {},{}",r(reg),amount),
            AluShiftLeft { op,dest,left,right,shift } => format!("{}_slli {},{},{},{}",match op{AluOp::Add=>"add",AluOp::Or=>"or"},r(dest),r(left),r(right),shift),
            VariableShiftLeft { dest,source,amount } => format!("sll {},{},{}",r(dest),r(source),r(amount)),
            BranchImmediate { reg,value,nonzero,target } => format!("{} {},{},.{:+}",if nonzero{"bnec"}else{"beqc"},r(reg),value,i64::from(target)-i64::from(pc)),
            BranchA5 { reg,nonzero,target } => format!("{} {},.{:+}",if nonzero{"bnes38"}else{"beqs38"},r(reg),i64::from(target)-i64::from(pc)),
            AddSmall { reg,value,subtract } => format!("{} {},{}",if subtract{"subi45"}else{"addi45"},r(reg),value),
            ZeroExtend { dest,source,half } => format!("{} {},{}",if half{"zeh33"}else{"zeb33"},r(dest),r(source)),
            BitMaskShort { reg,bit } => format!("bmski33 {},{}",r(reg),bit),
            CompactOr { dest,source } => format!("or33 {},{}",r(dest),r(source)),
            OrImmediate { dest,source,value } => format!("ori {},{},{}",r(dest),r(source),value),
            BitSet { dest,source,bit } => format!("bset {},{},{}",r(dest),r(source),bit),
            IndexedByte { store,reg,base,index,shift } => format!("{}b {},[{}+({}<<{})]",if store{"s"}else{"l"},r(reg),r(base),r(index),shift),
            JumpShort { target } => format!("j8 .{:+}",i64::from(target)-i64::from(pc)),
            JumpRelative { target,link } => format!("{} .{:+}",if link{"jal"}else{"j"},i64::from(target)-i64::from(pc)),
            // GNU's IFCALL9 operand is a displacement, not a target expression.
            InlineCall { target } => format!("ifcall9 {}",i64::from(target)-i64::from(pc)),
            InlineReturn => "ifret16".into(),
            Return { reg } => format!("ret5 {}",r(reg)),
            JumpRegisterLink { reg } => format!("jral5 {}",r(reg)),
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
    #[test]
    fn reserved_branch_register_is_not_an_indirect_call() {
        for nonzero in [false,true] {
            assert!(Insn::BranchA5{reg:R5,nonzero,target:0x4c8}.encode(0x484,0).is_err());
        }
        assert_eq!(Insn::JumpRegisterLink{reg:R2}.encode(0,0).unwrap(),[0xdd,0x22]);
        assert_eq!(Insn::JumpRegisterLink{reg:Reg::new(15)}.encode(0,0).unwrap(),[0xdd,0x2f]);
    }
    #[test]
    fn inline_calls_are_unsigned_and_return_alias_is_explicit() {
        assert_eq!(Insn::InlineCall{target:0x526}.encode(0x516,0).unwrap(),[0xf8,8]);
        assert!(Insn::InlineCall{target:0x514}.encode(0x516,0).is_err());
        assert!(Insn::InlineCall{target:0x916}.encode(0x516,0).is_err());
        assert_eq!(Insn::InlineReturn.encode(0,0).unwrap(),[0x83,0xff]);
    }
    #[test]
    fn compact_memory_checks_width_and_register_holes() {
        assert!(Insn::ShortMemory{store:false,width:Width::Half,reg:R0,base:R1,offset:3}.encode(0,0).is_err());
        assert!(Insn::WordAtRegister{store:false,reg:Reg::new(12),base:R0}.encode(0,0).is_err());
        assert_eq!(Insn::WordAtRegister{store:false,reg:Reg::new(16),base:R0}.encode(0,0).unwrap(),[0xb5,0x80]);
        assert!(Insn::LoadWordGp{dest:R0,address:3}.encode(0,0).is_err());
    }
}
