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
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum AluRegOp { Add, Sub, And, Xor, Or, Nor, Slt, Slts }
impl AluRegOp {
    const fn bits(self)->u32 {match self {Self::Add=>0,Self::Sub=>1,Self::And=>2,Self::Xor=>3,Self::Or=>4,Self::Nor=>5,Self::Slt=>6,Self::Slts=>7}}
    const fn mnemonic(self)->&'static str {match self {Self::Add=>"add",Self::Sub=>"sub",Self::And=>"and",Self::Xor=>"xor",Self::Or=>"or",Self::Nor=>"nor",Self::Slt=>"slt",Self::Slts=>"slts"}}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CompactMisc { Neg, Not, Mul, Xor, And, Or }
impl CompactMisc {
    const fn opcode(self)->u16 {match self {Self::Neg=>2,Self::Not=>3,Self::Mul=>4,Self::Xor=>5,Self::And=>6,Self::Or=>7}}
    const fn mnemonic(self)->&'static str {match self {Self::Neg=>"neg33",Self::Not=>"not33",Self::Mul=>"mul33",Self::Xor=>"xor33",Self::And=>"and33",Self::Or=>"or33"}}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignCondition { GeZero, LtZero, GtZero, LeZero, GeZeroLink, LtZeroLink }
impl SignCondition {
    const fn bits(self)->u32 {match self {
        Self::GeZero=>4,Self::LtZero=>5,Self::GtZero=>6,Self::LeZero=>7,
        Self::GeZeroLink=>12,Self::LtZeroLink=>13}}
    const fn mnemonic(self)->&'static str {match self {
        Self::GeZero=>"bgez",Self::LtZero=>"bltz",Self::GtZero=>"bgtz",
        Self::LeZero=>"blez",Self::GeZeroLink=>"bgezal",Self::LtZeroLink=>"bltzal"}}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csr { Psw, Ivb, IntMask, IntMask2, IntMask3, MiscCtl, Number(u16) }
impl Csr {
    fn index(self) -> u32 {
        match self { Self::Psw => 0x80, Self::Ivb => 0x89,
            Self::IntMask => 0xc0, Self::IntMask2 => 0xc1, Self::IntMask3 => 0xc2,
            Self::MiscCtl => 0x179, Self::Number(n)=>u32::from(n) }
    }
    fn name(self) -> &'static str {
        match self { Self::Psw => "psw", Self::Ivb => "ivb", Self::IntMask => "int_mask",
            Self::IntMask2 => "int_mask2", Self::IntMask3 => "int_mask3",
            Self::MiscCtl => "misc_ctl", Self::Number(_) => "unknown_system_register" }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Insn {
    StoreByteGp { source: Reg, address: u32 },
    StoreWordGp { source: Reg, address: u32 },
    LoadByteGp { dest: Reg, address: u32 },
    /// LBGP bit19 selects signed-byte LBSI.GP instead of zero-extended LBI.GP.
    LoadSignedByteGp {dest:Reg,address:u32},
    ReadSystem { dest: Reg, csr: Csr },
    WriteSystem { source: Reg, csr: Csr },
    /// Execute a separately supplied instruction-table entry; not a SWI.
    Ex9(u16),
    LowBit { dest: Reg, source: Reg },
    Move { dest: Reg, source: Reg },
    BranchZero { reg: Reg, nonzero: bool, target: u32 },
    BranchZeroWide { reg: Reg, nonzero: bool, target: u32 },
    /// N32 BR1 BEQ/BNE: compare two registers and branch on equality.
    CompareRegistersBranch {left:Reg,right:Reg,nonzero:bool,target:u32},
    BranchSignWide { reg: Reg, comparison: SignCondition, target: u32 },
    BitClearImmediate { dest: Reg, source: Reg, mask: u16 },
    SetHigh { dest: Reg, value: u32 },
    AddImmediate { dest: Reg, source: Reg, value: i32 },
    MoveImmediate { dest: Reg, value: i32 },
    MoveSmall { dest: Reg, value: i8 },
    MovePositive { dest: Reg, value: u8 },
    LoadWordGp { dest: Reg, address: u32 },
    HalfGp { store: bool, signed: bool, reg: Reg, address: u32 },
    ShortMemory { store: bool, width: Width, reg: Reg, base: Reg, offset: u8 },
    /// N16 LWI333.BI/SWI333.BI: word transfer at Ra then write back Ra+offset.
    ShortPostIndexedWord {store:bool,reg:Reg,base:Reg,offset:u8},
    ImmediateMemory { store: bool, width: Width, reg: Reg, base: Reg, offset: i32 },
    /// N32 .BI: access address at Ra, then apply signed scaled post-index to Ra.
    PostIndexedMemory {store:bool,width:Width,reg:Reg,base:Reg,offset:i32},
    /// N32 LDI/SDI .BI use signed 15-bit doubleword units and update Ra.
    DoubleWordMemory {store:bool,postindexed:bool,first:Reg,base:Reg,offset:i32},
    SignedMemoryImmediate {width:Width,dest:Reg,base:Reg,offset:i32},
    /// N32 LBSI.BI / LHSI.BI: signed byte/halfword read with base
    /// postincrement. Major opcodes 0x14/0x15 (N32_OP6_LBSI_BI/LHSI_BI).
    SignedPostIndexedMemory {width:Width,dest:Reg,base:Reg,offset:i32},
    /// LWI45.FE: load a word from the implicit $r8 frame/end register at
    /// a four-byte aligned negative offset, [-128,-4]. V3MUP extension.
    WordAtFrameEnd { dest: Reg, offset: i16 },
    /// N16 MOVD44 (V3MUP): move an even-register 64-bit pair. Both operands
    /// are encoded as the architectural register index divided by two.
    MoveDoubleCompact {dest_even:Reg,source_even:Reg},
    WordAtRegister { store: bool, reg: Reg, base: Reg },
    WordAtFrame { store: bool, reg: Reg, offset: u16 },
    WordAtStack { store: bool, reg: Reg, offset: u16 },
    ShiftRightSmall { reg: Reg, amount: u8 },
    /// N16 SRAI45 (major field 0x8): destructive signed right shift.
    ShiftArithmeticRightSmall { reg:Reg, amount:u8 },
    ShiftRight { dest: Reg, source: Reg, amount: u8 },
    AluShiftLeft { op: AluOp, dest: Reg, left: Reg, right: Reg, shift: u8 },
    VariableShiftLeft { dest: Reg, source: Reg, amount: Reg },
    /// N32 ALU1 SRL (subopcode 0x0d): shift Ra by low five bits of Rb.
    VariableShiftRight { dest: Reg, source: Reg, amount: Reg },
    /// N32 ALU1 SRA (subopcode 0x0e): arithmetic right shift by a register.
    VariableArithmeticShiftRight { dest: Reg, source: Reg, amount: Reg },
    /// N32 ALU1 CMOVZ/CMOVN, conditional on a register being zero or nonzero.
    ConditionalMove {dest:Reg,source:Reg,condition:Reg,nonzero:bool},
    /// N32 ALU1 SEH, sign-extend low 16 bits of the source register.
    /// N32 ALU1 SEB (subopcode 0x10): sign extend lowest source byte.
    SignExtendByteWide {dest:Reg,source:Reg},
    SignExtendHalfWide {dest:Reg,source:Reg},
    BranchImmediate { reg: Reg, value: i16, nonzero: bool, target: u32 },
    BranchA5 { reg: Reg, nonzero: bool, target: u32 },
    AddSmall { reg: Reg, value: u8, subtract: bool },
    /// ADD45/SUB45: compact destructive register arithmetic.
    AddSmallRegister { dest: Reg, source: Reg, subtract: bool },
    ZeroExtend { dest: Reg, source: Reg, half: bool },
    SignExtendByte { dest: Reg, source: Reg },
    SignExtendHalf { dest: Reg, source: Reg },
    BitMaskShort { reg: Reg, bit: u8 },
    BitFieldExtractImmediate { reg: Reg, bit: u8 },
    CompactOr { dest: Reg, source: Reg },
    CompactMiscOp { dest: Reg, source: Reg, op: CompactMisc },
    OrImmediate { dest: Reg, source: Reg, value: u16 },
    XorImmediate { dest:Reg,source:Reg,value:u16 },
    AndImmediate { dest: Reg, source: Reg, value: u16 },
    SubtractSmall3 { dest: Reg, source: Reg, value: u8 },
    AddSmall3 { dest:Reg,source:Reg,value:u8 },
    ShiftLeftSmall3 { dest:Reg,source:Reg,amount:u8 },
    CompactAlu3 { dest: Reg, left: Reg, right: Reg, subtract: bool },
    DivideRemainder { quotient: Reg, dividend: Reg, divisor: Reg, remainder: Reg, signed: bool },
    ArithmeticShiftRight { dest: Reg, source: Reg, amount: u8 },
    ShiftLeftImmediate {dest:Reg,source:Reg,amount:u8},
    /// N32 ALU2/MADDR32: multiply-add rounded 32-bit DSP operation.
    MultiplyAddRounded32 { dest:Reg, left:Reg, right:Reg },
    MultiplyWide { dest:Reg, left:Reg, right:Reg },
    /// N32 ALU2 ABS subopcode 0x03: absolute signed source value.
    AbsoluteRegister {dest:Reg,source:Reg},
    MaximumRegister {dest:Reg,left:Reg,right:Reg},
    /// N32 ALU2 MIN: unsigned minimum selected from two registers.
    MinimumRegister {dest:Reg,left:Reg,right:Reg},
    MultiplyRegister64 {dest:Reg,left:Reg,right:Reg,signed:bool},
    RegisterAlu {op:AluRegOp,dest:Reg,left:Reg,right:Reg,shift:u8},
    OrShiftRight {dest:Reg,left:Reg,right:Reg,shift:u8},
    /// N32 ALU1 ADD/SUB/AND/XOR_SRLI subops 0x1c..0x1f.
    RegisterAluShiftRight {op:AluRegOp,dest:Reg,left:Reg,right:Reg,shift:u8},
    ZeroExtendWide {dest:Reg,source:Reg},
    ReverseSubtractImmediate {dest:Reg,source:Reg,value:i16},
    /// SLTI45 writes the implicit temporary register TA.
    LessThanSmallImmediate { reg: Reg, value: u8 },
    LessThanSmallRegister { left: Reg, right: Reg },
    LessThanSignedSmallRegister { left: Reg, right: Reg },
    LessThanSignedSmallImmediate { reg: Reg, value: u8 },
    BranchTa { nonzero: bool, target: u32 },
    BitSet { dest: Reg, source: Reg, bit: u8 },
    /// N32 ALU2 BCLR immediate bit index, distinct from N32 BITCI mask.
    BitClearAt {dest:Reg, source:Reg, bit:u8},
    /// N32 ALU2/BTST: test source bit (immediate position) into destination.
    BitTestImmediate {dest:Reg,source:Reg,bit:u8},
    IndexedByte { store: bool, reg: Reg, base: Reg, index: Reg, shift: u8 },
    IndexedMemory { store:bool, width:Width, signed:bool, reg:Reg, base:Reg,index:Reg,shift:u8 },
    /// N32 indexed load/store with base writeback (LBI/LHI/LWI.BI and
    /// SBI/SHI/SWI.BI; GNU MEM subopcodes 4..6,12..14).
    IndexedPostIndexedMemory {store:bool,width:Width,reg:Reg,base:Reg,index:Reg,shift:u8},
    JumpShort { target: u32 },
    JumpRelative { target: u32, link: bool },
    /// IFC_LP/IFC_ON state differs from conventional JAL/RET.
    InlineCall { target: u32 },
    /// N32 BR2/SOP0 IFCALL: signed 16-bit halfword PC displacement.
    /// Unlike IFCALL9, this form can target a preceding instruction.
    InlineCallWide { target: u32 },
    /// Conditional return through IFC_LP; falls through with IFC_ON clear.
    InlineReturn,
    Return { reg: Reg },
    JumpRegisterLink { reg: Reg },
    /// N16 JR5: jump to address in compact register, without setting LP.
    JumpRegister { reg:Reg },
    /// N32 JREG/JR: nonlinking indirect jump via a general register.
    JumpRegisterWide { reg: Reg },
    /// Little-endian 35-entry halfword branch-target displacement table.
    /// NDS32 code fetch is big-endian, but data loads here use little endian.
    JumpTableOffsets16 { base:u32, targets:[u32;35] },
    /// Code-adjacent 15-way halfword-offset dispatch using LE table data.
    JumpTableOffsets15 {base:u32,targets:[u32;15]},
    JumpTableOffsets12 {base:u32,targets:[u32;12]},
    /// LE halfword offsets after a fourteen-way NDS32 indirect branch.
    JumpTableOffsets14 {base:u32,targets:[u32;14]},
    /// LE halfword offsets after a sixteen-way indirect branch.
    JumpTableOffsets16Entries {base:u32,targets:[u32;16]},
    /// Thirteen little-endian halfword offsets from an indirect format switch.
    JumpTableOffsets13 {base:u32,targets:[u32;13]},
    /// Unsigned byte offsets for a twelve-way indirect branch.
    JumpTableOffsets12Bytes {base:u32,targets:[u32;12]},
    /// Five byte offsets, followed by one zero byte for code alignment.
    JumpTableOffsets5Padded {base:u32,targets:[u32;5]},
    /// Four little-endian halfword offsets selected by a 2-bit index.
    JumpTableOffsets4Halfwords {base:u32,targets:[u32;4]},
    /// Six byte offset entries selected by an unsigned index.
    JumpTableOffsets6Bytes {base:u32,targets:[u32;6]},
    JumpTableOffsets6 {base:u32,targets:[u32;6]},
    /// NDS32 jump-table byte offsets, not executable instructions. `base` is
    /// the register-held table pointer. Each entry is derived from a named
    /// branch destination and checked to fit an unsigned byte displacement.
    JumpTableOffsets10 {base:u32,targets:[u32;10]},
    /// NDS32 eight-way direct byte offset table; register holds base.
    JumpTableOffsets8 {base:u32,targets:[u32;8]},
    DataSync,
    StackAdjust { bytes: i16 },
    AddressFromStack { dest: Reg, offset: u16 },
    LessThanWide { dest: Reg, source: Reg, value: u16, signed: bool },
    Stack { pop: bool, last: Reg, extra: u16 },
    /// The observed paired forms are after-decrement and before-increment.
    /// `specials` is the architectural four-bit FP/GP/LP/SP register mask.
    MultipleWords { store: bool, first: Reg, base: Reg, last: Reg, specials: u8, decrement: bool, writeback: bool },
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

fn emit_little_endian_branch_offsets(base:u32,targets:&[u32])->Result<Vec<u8>,String>{
    let mut bytes=Vec::with_capacity(targets.len()*2);
    for (idx,target) in targets.iter().enumerate(){
        let delta=target.checked_sub(base).ok_or_else(||format!("dispatch target {idx} precedes base"))?;
        if delta>u16::MAX as u32 || delta&1!=0 {
            return Err(format!("dispatch target {idx} is not an aligned 16-bit offset"));
        }
        bytes.extend_from_slice(&(delta as u16).to_le_bytes());
    }
    Ok(bytes)
}

impl Insn {
    pub fn encode(self, pc: u32, gp: u32) -> Result<Vec<u8>, String> {
        use Insn::*;
        if let JumpTableOffsets12{base,targets}=self {
            return emit_little_endian_branch_offsets(base,&targets);
        }
        if let JumpTableOffsets14{base,targets}=self {
            return emit_little_endian_branch_offsets(base,&targets);
        }
        if let JumpTableOffsets16Entries{base,targets}=self {
            return emit_little_endian_branch_offsets(base,&targets);
        }
        if let JumpTableOffsets13{base,targets}=self {
            return emit_little_endian_branch_offsets(base,&targets);
        }
        if let JumpTableOffsets5Padded{base,targets}=self {
            let mut bytes=Vec::with_capacity(6);
            for (i,target) in targets.into_iter().enumerate(){
                let delta=target.checked_sub(base).ok_or_else(||format!("five-way target {i} precedes base"))?;
                if delta>255 || delta&1!=0{return Err(format!("five-way target {i} invalid byte offset"));}
                bytes.push(delta as u8);
            }
            bytes.push(0); // alignment byte after exactly five cases
            return Ok(bytes);
        }
        if let JumpTableOffsets4Halfwords{base,targets}=self {
            return emit_little_endian_branch_offsets(base,&targets);
        }
        if let JumpTableOffsets6Bytes{base,targets}=self {
            let mut bytes=Vec::with_capacity(6);
            for (i,target) in targets.into_iter().enumerate(){
                let delta=target.checked_sub(base).ok_or_else(||format!("six-way target {i} precedes base"))?;
                if delta>255 || delta&1!=0{return Err(format!("six-way target {i} invalid byte offset"));}
                bytes.push(delta as u8);
            }
            return Ok(bytes);
        }
        if let JumpTableOffsets12Bytes{base,targets}=self {
            let mut bytes=Vec::with_capacity(12);
            for (i,target) in targets.into_iter().enumerate() {
                let delta=target.checked_sub(base).ok_or_else(||format!("12-way target {i} precedes table"))?;
                if delta>255 || delta&1!=0{return Err(format!("12-way target {i} unaligned/outside byte table"));}
                bytes.push(delta as u8);
            }
            return Ok(bytes);
        }
        if let JumpTableOffsets6{base,targets}=self {
            return emit_little_endian_branch_offsets(base,&targets);
        }
        if let JumpTableOffsets15{base,targets}=self {
            let mut bytes=Vec::with_capacity(30);
            for (idx,target) in targets.into_iter().enumerate() {
                if target<base || target-base>u16::MAX as u32 || (target-base)&1!=0 {
                    return Err(format!("15-way jump-table entry {idx} outside aligned 16-bit offset"));
                }
                bytes.extend_from_slice(&((target-base) as u16).to_le_bytes());
            }
            return Ok(bytes);
        }
        if let JumpTableOffsets16{base,targets}=self {
            let mut bytes=Vec::with_capacity(70);
            for (idx,target) in targets.into_iter().enumerate() {
                if target<base || (target-base)>u16::MAX as u32 || (target-base)&1!=0 {
                    return Err(format!("halfword jump-table entry {idx} outside aligned 16-bit offset"));
                }
                bytes.extend_from_slice(&((target-base) as u16).to_le_bytes());
            }
            return Ok(bytes);
        }
        if let JumpTableOffsets8{base,targets}=self {
            let mut offsets=Vec::with_capacity(targets.len());
            for (i,target) in targets.into_iter().enumerate() {
                if target<base || target-base>255 || (target-base)&1!=0 {
                    return Err(format!("eight-way jump target {i} 0x{target:x} outside table at 0x{base:x}"));
                }
                offsets.push((target-base) as u8);
            }
            return Ok(offsets);
        }
        if let JumpTableOffsets10{base,targets}=self {
            let mut offsets=Vec::with_capacity(targets.len());
            for (i,target) in targets.into_iter().enumerate() {
                if target<base || target-base>255 || (target-base)&1!=0 {
                    return Err(format!("jump table entry {i} target 0x{target:x} invalid for base 0x{base:x}"));
                }
                offsets.push((target-base) as u8);
            }
            return Ok(offsets);
        }
        let word = match self {
            StoreByteGp { source, address } => (0x1f << 25) | (source.number() << 20)
                | signed(i64::from(address)-i64::from(gp),19)?,
            StoreWordGp { source,address } => {
                let delta=i64::from(address)-i64::from(gp);
                if delta%4!=0{return Err("unaligned GP word address".into());}
                (0x1e<<25)|(source.number()<<20)|(7<<17)|signed(delta/4,17)?
            }
            LoadByteGp { dest,address } => (0x17<<25)|(dest.number()<<20)
                |signed(i64::from(address)-i64::from(gp),19)?,
            LoadSignedByteGp { dest,address } => (0x17<<25)|(dest.number()<<20)
                |(1<<19)|signed(i64::from(address)-i64::from(gp),19)?,
            ReadSystem { dest, csr } if csr.index()<1024 => (0x32 << 25) | (dest.number() << 20) | (csr.index() << 10) | 2,
            WriteSystem { source, csr } if csr.index()<1024 => (0x32 << 25) | (source.number() << 20) | (csr.index() << 10) | 3,
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
            HalfGp { store,signed:is_signed,reg,address } => {
                if store && is_signed {return Err("signed GP halfword store is impossible".into());}
                let delta=i64::from(address)-i64::from(gp);
                if delta&1 != 0 {return Err("unaligned GP halfword address".into());}
                let type_code=if store{4}else if is_signed{2}else{0};
                (0x1e<<25)|(reg.number()<<20)|(type_code<<17)|signed(delta/2,18)?
            }
            SignedMemoryImmediate{width,dest,base,offset} => {
                let (op,align)=match width{Width::Byte=>(0x10,1),Width::Half=>(0x11,2),
                   Width::Word=>return Err("signed word immediate load is undefined".into())};
                if offset%align !=0 {return Err("unaligned signed memory immediate".into());}
                (op<<25)|(dest.number()<<20)|(base.number()<<15)
                 |signed(i64::from(offset/align),15)?
            },
            SignedPostIndexedMemory{width,dest,base,offset} => {
                let (opcode,align)=match width {Width::Byte=>(0x14,1),
                    Width::Half=>(0x15,2),
                    Width::Word=>return Err("signed postindexed word operation is reserved".into())};
                if offset%align!=0 {return Err("signed postindexed displacement misaligned".into());}
                if dest==base {return Err("signed postindexed destination aliases writeback base".into());}
                (opcode<<25)|(dest.number()<<20)|(base.number()<<15)
                    |signed(i64::from(offset/align),15)?
            },
            DoubleWordMemory {store,postindexed,first,base,offset} => {
                if offset%8!=0{return Err("unaligned doubleword displacement".into());}
                if first.number()>30{return Err("NDS32 doubleword transfer requires a register pair".into());}
                if !store && (first==base || first.number()+1==base.number()) {
                    return Err("doubleword load writeback aliases data registers".into());
                }
                let major=match (store,postindexed) {(false,false)=>3,(false,true)=>7,
                    (true,false)=>11,(true,true)=>15};
                (major<<25)|(first.number()<<20)|(base.number()<<15)
                    |signed(i64::from(offset/8),15)?
            },
            PostIndexedMemory {store,width,reg,base,offset} => {
                let scale=i32::from(width.scale());
                if offset%scale!=0{return Err("unaligned post-indexed memory offset".into());}
                if !store && reg==base {
                    return Err("NDS32 post-indexed load cannot write back into destination register".into());
                }
                let operation=match width {Width::Byte=>4,Width::Half=>5,Width::Word=>6};
                ((operation+u32::from(store)*8)<<25)|(reg.number()<<20)|(base.number()<<15)
                    |signed(i64::from(offset/scale),15)?
            },
            ImmediateMemory { store,width,reg,base,offset } => {
                let scale=i32::from(width.scale());
                if offset%scale!=0{return Err("unaligned immediate memory offset".into());}
                let operation=match width{Width::Byte=>0,Width::Half=>1,Width::Word=>2};
                ((operation+u32::from(store)*8)<<25)|(reg.number()<<20)|(base.number()<<15)
                    |signed(i64::from(offset/scale),15)?
            }
            ShiftRight { dest,source,amount } => {
                if amount>31{return Err("SRLI amount exceeds 31".into());}
                (0x20<<25)|(dest.number()<<20)|(source.number()<<15)|(u32::from(amount)<<10)|9
            }
            AluShiftLeft { op,dest,left,right,shift } => {
                if shift>31{return Err("ALU shift exceeds 31".into());}
                (0x20<<25)|(dest.number()<<20)|(left.number()<<15)|(right.number()<<10)
                    |(u32::from(shift)<<5)|match op{AluOp::Add=>0,AluOp::Or=>4}
            }
            VariableShiftLeft { dest,source,amount } => (0x20<<25)|(dest.number()<<20)|(source.number()<<15)|(amount.number()<<10)|12,
            VariableShiftRight { dest,source,amount } => (0x20<<25)|(dest.number()<<20)|(source.number()<<15)|(amount.number()<<10)|13,
            VariableArithmeticShiftRight {dest,source,amount} => (0x20<<25)|(dest.number()<<20)|(source.number()<<15)|(amount.number()<<10)|14,
            ConditionalMove{dest,source,condition,nonzero} =>
                (0x20<<25)|(dest.number()<<20)|(source.number()<<15)
                    |(condition.number()<<10)|(if nonzero{0x1b}else{0x1a}),
            SignExtendByteWide{dest,source} => (0x20<<25)
                |(dest.number()<<20)|(source.number()<<15)|0x10,
            SignExtendHalfWide{dest,source} => (0x20<<25)
                |(dest.number()<<20)|(source.number()<<15)|0x11,
            BranchImmediate { reg,value,nonzero,target } => (0x2d<<25)|(reg.number()<<20)
                |(u32::from(nonzero)<<19)|(signed(i64::from(value),11)?<<8)|displacement(pc,target,8)?,
            BranchSignWide { reg,comparison,target } => (0x27<<25)|(reg.number()<<20)
                | (comparison.bits()<<16) | displacement(pc,target,16)?,
            CompareRegistersBranch {left,right,nonzero,target} =>
                (0x26<<25)|(left.number()<<20)|(right.number()<<15)
                |(u32::from(nonzero)<<14)|displacement(pc,target,14)?,
            BranchZeroWide { reg,nonzero,target } => (0x27<<25)|(reg.number()<<20)
                |((2+u32::from(nonzero))<<16)|displacement(pc,target,16)?,
            OrImmediate { dest,source,value } => (0x2c<<25)|(dest.number()<<20)|(source.number()<<15)|u15(value)?,
            XorImmediate { dest,source,value } => (0x2b<<25)|(dest.number()<<20)|(source.number()<<15)|u15(value)?,
            AndImmediate { dest,source,value } => (0x2a<<25)|(dest.number()<<20)|(source.number()<<15)|u15(value)?,
            BitTestImmediate {dest,source,bit} => {
                if bit>31{return Err("BTST bit exceeds 31".into());}
                (0x21<<25)|(dest.number()<<20)|(source.number()<<15)|(u32::from(bit)<<10)|0x0b
            },
            BitSet { dest,source,bit } => {
                if bit>31{return Err("BSET bit exceeds 31".into());}
                (0x21<<25)|(dest.number()<<20)|(source.number()<<15)|(u32::from(bit)<<10)|8
            }
            BitClearAt {dest,source,bit} => {
                if bit>31{return Err("BCLR bit exceeds 31".into());}
                (0x21<<25)|(dest.number()<<20)|(source.number()<<15)|(u32::from(bit)<<10)|9
            }
            IndexedPostIndexedMemory{store,width,reg,base,index,shift} => {
                if shift>3{return Err("NDS32 MEM post-indexed shift exceeds 3".into());}
                if !store && reg==base{return Err("NDS32 postindexed load aliases destination with writeback base".into());}
                let sub=match width{Width::Byte=>4,Width::Half=>5,Width::Word=>6}+if store{8}else{0};
                (0x1c<<25)|(reg.number()<<20)|(base.number()<<15)
                |(index.number()<<10)|(u32::from(shift)<<8)|sub
            },
            IndexedMemory {store,width,signed,reg,base,index,shift} => {
                if shift>3 || (store && signed) {return Err("invalid indexed memory shift/sign".into());}
                let op=match (store,width,signed){
                  (false,Width::Byte,false)=>0,(false,Width::Half,false)=>1,(false,Width::Word,false)=>2,
                  (true,Width::Byte,false)=>8,(true,Width::Half,false)=>9,(true,Width::Word,false)=>10,
                  (false,Width::Byte,true)=>16,(false,Width::Half,true)=>17,
                  _=>return Err("no signed word/indexed store encoding".into())};
                (0x1c<<25)|(reg.number()<<20)|(base.number()<<15)|(index.number()<<10)
                 |(u32::from(shift)<<8)|op
            },
            IndexedByte { store,reg,base,index,shift } => {
                if shift>3{return Err("indexed byte shift exceeds 3".into());}
                (0x1c<<25)|(reg.number()<<20)|(base.number()<<15)|(index.number()<<10)
                    |(u32::from(shift)<<8)|(u32::from(store)<<3)
            }
            JumpRelative { target,link } => (0x24<<25)|(u32::from(link)<<24)|displacement(pc,target,24)?,
            InlineCallWide { target } => (0x27<<25)|displacement(pc,target,16)?,
            JumpRegisterWide{reg} => (0x25<<25)|(reg.number()<<10),
            MultipleWords { store,first,base,last,specials,decrement,writeback } => {
                if first.number()>last.number() || specials>15 {
                    return Err("invalid multiple-word register range or special-register mask".into());
                }
                (0x1d<<25)|(first.number()<<20)|(base.number()<<15)|(last.number()<<10)
                    |(u32::from(specials)<<6)|(u32::from(store)<<5)
                    |(if decrement{0x18}else{0})|(u32::from(writeback)<<2)
            }
            ReadSystem{..}|WriteSystem{..} => return Err("NDS32 CSR index exceeds 10 bits".into()),
            ReverseSubtractImmediate{dest,source,value} => (0x29<<25)
                |(dest.number()<<20)|(source.number()<<15)|signed(i64::from(value),15)?,
            ZeroExtendWide{dest,source} => (0x20<<25)|(dest.number()<<20)
                |(source.number()<<15)|0x13,
            RegisterAluShiftRight{op,dest,left,right,shift} => {
                if shift>31{return Err("NDS32 ALU1 shifted right register exceeds 31 bits".into());}
                let sub=match op {AluRegOp::Add=>0x1c,AluRegOp::Sub=>0x1d,
                    AluRegOp::And=>0x1e,AluRegOp::Xor=>0x1f,
                    _=>return Err("unsupported NDS32 ALU1 right-shift arithmetic opcode".into())};
                (0x20<<25)|(dest.number()<<20)|(left.number()<<15)
                 |(right.number()<<10)|(u32::from(shift)<<5)|sub
            },
            OrShiftRight{dest,left,right,shift} => {
                if shift>31{return Err("OR_SRLI amount exceeds 31".into());}
                (0x20<<25)|(dest.number()<<20)|(left.number()<<15)|(right.number()<<10)
                 |(u32::from(shift)<<5)|0x15
            },
            MultiplyRegister64{dest,left,right,signed:is_signed} =>
                (0x21<<25)|(dest.number()<<20)|(left.number()<<15)|(right.number()<<10)
                    |if is_signed {0x68}else{0x69},
            AbsoluteRegister{dest,source} => (0x21<<25)
                |(dest.number()<<20)|(source.number()<<15)|3,
            MaximumRegister{dest,left,right} => (0x21<<25)
                |(dest.number()<<20)|(left.number()<<15)|(right.number()<<10),
            MinimumRegister{dest,left,right} => (0x21<<25)
                |(dest.number()<<20)|(left.number()<<15)|(right.number()<<10)|1,
            MultiplyWide{dest,left,right} => (0x21<<25)|(dest.number()<<20)
                |(left.number()<<15)|(right.number()<<10)|0x24,
            MultiplyAddRounded32 {dest,left,right} =>
                (0x21<<25)|(dest.number()<<20)|(left.number()<<15)|(right.number()<<10)|0x73,
            RegisterAlu{op,dest,left,right,shift} => {
                if shift>31{return Err("NDS32 ALU1 shifted register exceeds 31 bits".into());}
                (0x20<<25)|(dest.number()<<20)|(left.number()<<15)|(right.number()<<10)
                    |(u32::from(shift)<<5)|op.bits()
            },
            ShiftLeftImmediate {dest,source,amount} => {
                if amount>31{return Err("SLLI shift exceeds 31".into());}
                (0x20<<25)|(dest.number()<<20)|(source.number()<<15)
                |(u32::from(amount)<<10)|8
            },
            ArithmeticShiftRight { dest, source, amount } => {
                if amount>31{return Err("SRAI amount exceeds 31".into());}
                (0x20 << 25) | (dest.number()<<20) | (source.number()<<15)
                    | (u32::from(amount)<<10) | 0x0a
            },
            DivideRemainder {quotient,dividend,divisor,remainder,signed:is_signed} => {
                (0x20 << 25) | (quotient.number()<<20) | (dividend.number()<<15)
                  | (divisor.number()<<10) | (remainder.number()<<5)
                  | (if is_signed {0x16}else{0x17})
            },
            DataSync => (0x32 << 25) | 8,
            LessThanWide { dest,source,value,signed:is_signed } => {
                if value >= 0x8000 { return Err("SLTI wide immediate exceeds 15 bits".into()); }
                ((if is_signed {0x2f} else {0x2e})<<25)
                    | (dest.number()<<20) | (source.number()<<15) | u32::from(value)
            },
            _ => return Ok(self.short(pc)?.to_be_bytes().to_vec()),
        };
        Ok(word.to_be_bytes().to_vec())
    }
    fn short(self, pc: u32) -> Result<u16, String> {
        use Insn::*;
        Ok(match self {
            AddressFromStack { dest,offset } => {
                if offset>252 || offset%4 != 0 {return Err("ADDRI36.SP uses unsigned six-bit word offset".into());}
                0xb000 | dest.small()?<<6 | offset/4
            }
            StackAdjust { bytes } => {
                if bytes & 1 != 0 || bytes < -512 || bytes > 510 {
                    return Err("ADDI10.SP requires even signed 10-bit operand".into());
                }
                0xec00 | (signed(i64::from(bytes),10)? as u16)
            }
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
            MovePositive { dest,value } => {
                if !(16..=47).contains(&value){return Err("MOVPI45 value must be 16..47".into());}
                0xfa00|dest.compact()?<<5|u16::from(value-16)
            }
            ShortPostIndexedWord{store,reg,base,offset} => {
                if offset%4!=0 || offset>28{return Err("N16 word post-index must be 0..28 and 4-byte aligned".into());}
                if !store && reg==base{return Err("unpredictable N16 load/writeback register alias".into());}
                let top=if store{0xaa00u16}else{0xa200u16};
                top|(reg.small()?<<6)|(base.small()?<<3)|u16::from(offset/4)
            },
            ShortMemory { store,width,reg,base,offset } => {
                if offset%width.scale()!=0 || offset/width.scale()>7{return Err("invalid compact memory offset".into());}
                let operation=match width{Width::Word=>0xa000,Width::Half=>0xa400,Width::Byte=>0xa600};
                operation | (u16::from(store)<<11) | reg.small()?<<6 | base.small()?<<3 | u16::from(offset/width.scale())
            }
            MoveDoubleCompact {dest_even,source_even} => {
                if dest_even.number()>30 || source_even.number()>30 ||
                    (dest_even.number()&1)!=0 || (source_even.number()&1)!=0 {
                    return Err("MOVD44 requires even GPRs r0..r30, in 64-bit register pairs".into());
                }
                0xfd00 | ((dest_even.number()/2) as u16)<<4 | (source_even.number()/2) as u16
            },
            WordAtFrameEnd { dest,offset } => {
                if offset < -128 || offset > -4 || offset%4 != 0 {
                    return Err("LWI45.FE offset must be a negative word offset from -128 through -4".into());
                }
                // GNU NDS32 N16_TYPE45 encoding: rt4 uses the compact
                // r45 mapping, fe5 is (offset + 128)/4.
                0xb200 | (dest.compact()?<<5) | ((offset + 128)/4) as u16
            },
            WordAtRegister { store,reg,base } => 0xb400|(u16::from(store)<<9)|(reg.compact()?<<5)|base.number() as u16,
            WordAtStack {store,reg,offset} => {
                if offset%4!=0 || offset>508 {return Err("invalid SP-relative compact word offset".into());}
                0xf000 | reg.small()?<<8 | u16::from(store)<<7 | offset/4
            },
            WordAtFrame { store,reg,offset } => {
                if offset%4!=0 || offset>508{return Err("invalid FP-relative compact word offset".into());}
                0xb800 | reg.small()?<<8 | u16::from(store)<<7 | offset/4
            }
            ShiftArithmeticRightSmall { reg,amount } => {
                if amount>31{return Err("SRAI45 amount exceeds 31".into());}
                0x9000 | reg.compact()? << 5 | u16::from(amount)
            },
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
            AddSmallRegister { dest,source,subtract } => 0x8800|u16::from(subtract)<<9|dest.compact()?<<5|source.number() as u16,
            ZeroExtend { dest,source,half } => 0x9600|dest.small()?<<6|source.small()?<<3|u16::from(half),
            SignExtendByte { dest,source } => 0x9602|dest.small()?<<6|source.small()?<<3,
            SignExtendHalf { dest,source } => 0x9603|dest.small()?<<6|source.small()?<<3,
            CompactAlu3 { dest,left,right,subtract } => 0x9800 | (u16::from(subtract)<<9)
                | (dest.small()?<<6) | (left.small()?<<3) | right.small()?,
            ShiftLeftSmall3{dest,source,amount} => {
                if amount>7{return Err("SLLI333 shift exceeds 7".into());}
                0x9400|dest.small()?<<6|source.small()?<<3|u16::from(amount)
            },
            AddSmall3{dest,source,value} => {
                if value>7{return Err("ADDI333 operand exceeds 7".into());}
                0x9c00|dest.small()?<<6|source.small()?<<3|u16::from(value)
            },
            SubtractSmall3 { dest,source,value } => {
                if value>7{return Err("SUBI333 value exceeds 7".into());}
                0x9e00|dest.small()?<<6|source.small()?<<3|u16::from(value)
            }
            LessThanSmallRegister {left,right} => 0xe200|left.compact()?<<5|right.number() as u16,
            LessThanSignedSmallRegister {left,right} => 0xe000|left.compact()?<<5|right.number() as u16,
            LessThanSignedSmallImmediate {reg,value} => {
                if value>31{return Err("SLTSI45 value exceeds 31".into());}
                0xe400|reg.compact()?<<5|u16::from(value)
            },
            LessThanSmallImmediate { reg,value } => {
                if value>31{return Err("SLTI45 value exceeds 31".into());}
                0xe600|reg.compact()?<<5|u16::from(value)
            }
            BranchTa { nonzero,target } => (if nonzero{0xe900}else{0xe800})|displacement(pc,target,8)? as u16,
            BitFieldExtractImmediate {reg,bit} => {
                if bit>7 {return Err("FEXTI33 bit exceeds 7".into());}
                0x9607|reg.small()?<<6|u16::from(bit)<<3
            },
            BitMaskShort { reg,bit } => {
                if bit>7{return Err("BMSKI33 bit exceeds 7".into());}
                0x9606|reg.small()?<<6|u16::from(bit)<<3
            }
            CompactOr { dest,source } => 0xfe07|dest.small()?<<6|source.small()?<<3,
            CompactMiscOp { dest,source,op } => 0xfe00 | dest.small()?<<6 | source.small()?<<3 | op.opcode(),
            JumpShort { target } => 0xd500|displacement(pc,target,8)? as u16,
            InlineCall { target } => {
                if (pc|target)&1!=0 || target<pc || target-pc>1022{return Err("invalid IFCALL9 forward target".into());}
                0xf800 | ((target-pc)/2) as u16
            }
            InlineReturn => 0x83ff,
            Return { reg } => 0xdd80|reg.number() as u16,
            JumpRegisterLink { reg } => 0xdd20|reg.number() as u16,
            JumpRegister { reg } => 0xdd00|reg.number() as u16,
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
            StoreWordGp { source,address } => format!("swi.gp {},[{}]",r(source),i64::from(address)-i64::from(gp)),
            LoadByteGp { dest,address } => format!("lbi.gp {},[{}]",r(dest),i64::from(address)-i64::from(gp)),
            LoadSignedByteGp { dest,address } => format!("lbsi.gp {},[{}]",r(dest),i64::from(address)-i64::from(gp)),
            ReadSystem { dest,csr } => format!("mfsr {},{}",r(dest),if let Csr::Number(index)=csr {format!("0x{index:x}")} else if csr==Csr::IntMask3 {"0xc2".into()} else {format!("${}",csr.name())}),
            WriteSystem { source,csr } => format!("mtsr {},{}",r(source),if let Csr::Number(index)=csr {format!("0x{index:x}")} else if csr==Csr::IntMask3 {"0xc2".into()} else {format!("${}",csr.name())}),
            JumpTableOffsets14{base,targets}=>format!("jump.offsets14 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets13{base,targets}=>format!("jump.offsets13 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets16Entries{base,targets}=>format!("jump.offsets16entries 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets5Padded{base,targets}=>format!("jump.offsets5padded 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets4Halfwords{base,targets}=>format!("jump.offsets4halfs 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets6Bytes{base,targets}=>format!("jump.offsets6bytes 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets12Bytes{base,targets}=>format!("jump.offsets12bytes 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets8{base,targets} => format!("jump.offsets8 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets10{base,targets} => format!("jump.offsets10 {}",std::iter::once(format!("0x{base:x}"))
                .chain(targets.iter().map(|target|format!("0x{target:x}")))
                .collect::<Vec<_>>().join(",")),
            StackAdjust { bytes } => format!("addi10.sp {bytes}"),
            AddressFromStack {dest,offset} => format!("addri36.sp {},{}",r(dest),offset),
            ReverseSubtractImmediate{dest,source,value} => format!("subri {},{},{}",r(dest),r(source),value),
            ZeroExtendWide{dest,source} => format!("zeh {},{}",r(dest),r(source)),
            SignExtendByteWide{dest,source} => format!("seb {},{}",r(dest),r(source)),
            OrShiftRight{dest,left,right,shift} => format!("or_srli {},{},{},{}",r(dest),r(left),r(right),shift),
            RegisterAluShiftRight{op,dest,left,right,shift} =>
                format!("{}_srli {},{},{},{}",op.mnemonic(),r(dest),r(left),r(right),shift),
            AbsoluteRegister{dest,source}=>format!("abs {},{}",r(dest),r(source)),
            MultiplyRegister64{dest,left,right,signed:is_signed} => format!("{} {},{},{}",if is_signed{"mulsr64"}else{"mulr64"},r(dest),r(left),r(right)),
            MaximumRegister{dest,left,right} => format!("max {},{},{}",r(dest),r(left),r(right)),
            MinimumRegister{dest,left,right} => format!("min {},{},{}",r(dest),r(left),r(right)),
            MultiplyWide{dest,left,right} => format!("mul {},{},{}",r(dest),r(left),r(right)),
            MultiplyAddRounded32{dest,left,right} => format!("maddr32 {},{},{}",r(dest),r(left),r(right)),
            RegisterAlu{op,dest,left,right,shift} => format!("{}_slli {},{},{},{}",op.mnemonic(),r(dest),r(left),r(right),shift),
            ShiftLeftImmediate {dest,source,amount} => format!("slli {},{},{}",r(dest),r(source),amount),
            ArithmeticShiftRight { dest,source,amount } => format!("srai {},{},{}",r(dest),r(source),amount),
            DivideRemainder { quotient,dividend,divisor,remainder,signed:is_signed } => format!("{} {},{},{},{}",if is_signed {"divsr"}else{"divr"},r(quotient),r(dividend),r(divisor),r(remainder)),
            LessThanWide { dest,source,value,signed:is_signed } => format!("{} {},{},{}", if is_signed {"sltsi"}else{"slti"},r(dest),r(source),value),
            Ex9(i) => format!("ex9.it {i}"),
            LowBit { dest,source } => format!("xlsb33 {},{}",r(dest),r(source)),
            Move { dest,source } => format!("mov55 {},{}",r(dest),r(source)),
            BranchZero { reg,nonzero,target } => format!("{} {},.{:+}",if nonzero{"bnez38"}else{"beqz38"},r(reg),i64::from(target)-i64::from(pc)),
            CompareRegistersBranch {left,right,nonzero,target} =>
                format!("{} {},{},.{:+}",if nonzero{"bne"}else{"beq"},r(left),r(right),i64::from(target)-i64::from(pc)),
            BranchZeroWide { reg,nonzero,target } => format!("{} {},.{:+}",if nonzero{"bnez"}else{"beqz"},r(reg),i64::from(target)-i64::from(pc)),
            BranchSignWide { reg,comparison,target } => format!("{} {},.{:+}",comparison.mnemonic(),r(reg),i64::from(target)-i64::from(pc)),
            BitClearImmediate { dest,source,mask } => format!("bitci {},{},{}",r(dest),r(source),mask),
            SetHigh { dest,value } => format!("sethi {},{}",r(dest),value),
            AddImmediate { dest,source,value } => format!("addi {},{},{}",r(dest),r(source),value),
            MoveImmediate { dest,value } => format!("movi {},{}",r(dest),value),
            MoveSmall { dest,value } => format!("movi55 {},{}",r(dest),value),
            MovePositive { dest,value } => format!("movpi45 {},{}",r(dest),value),
            LoadWordGp { dest,address } => format!("lwi.gp {},[{}]",r(dest),i64::from(address)-i64::from(gp)),
            HalfGp { store,signed,reg,address } => format!("{} {},[{}]",if store{"shi.gp"}else if signed{"lhsi.gp"}else{"lhi.gp"},r(reg),i64::from(address)-i64::from(gp)),
            ShortMemory { store,width,reg,base,offset } => format!("{}{}i333 {},[{}+{}]",if store{"s"}else{"l"},width.letter(),r(reg),r(base),offset),
            ShortPostIndexedWord{store,reg,base,offset} => format!("{}wi333.bi {},[{}],{}",if store{"s"}else{"l"},r(reg),r(base),offset),
            SignedMemoryImmediate{width,dest,base,offset} => format!("l{}si {},[{}{:+}]",width.letter(),r(dest),r(base),offset),
            ImmediateMemory { store,width,reg,base,offset } => format!("{}{}i {},[{}{:+}]",if store{"s"}else{"l"},width.letter(),r(reg),r(base),offset),
            PostIndexedMemory{store,width,reg,base,offset} => format!("{}{}i.bi {},[{}],{}",if store{"s"}else{"l"},width.letter(),r(reg),r(base),offset),
            SignedPostIndexedMemory{width,dest,base,offset} =>
                format!("l{}si.bi {},[{}],{}",width.letter(),r(dest),r(base),offset),
            DoubleWordMemory{store,postindexed,first,base,offset} => {
                if postindexed {format!("{}di.bi {},[{}],{}",if store{"s"}else{"l"},r(first),r(base),offset)}
                else {format!("{}di {},[{}{:+}]",if store{"s"}else{"l"},r(first),r(base),offset)}
            },
            MoveDoubleCompact {dest_even,source_even} =>
                format!("movd44 {},{}",r(dest_even),r(source_even)),
            WordAtFrameEnd {dest,offset} => format!("lwi45.fe {},{}",r(dest),offset),
            WordAtRegister { store,reg,base } => format!("{}wi450 {},[{}]",if store{"s"}else{"l"},r(reg),r(base)),
            WordAtStack {store,reg,offset} => format!("{}wi37.sp {},[$sp+{}]",if store{"s"}else{"l"},r(reg),offset),
            WordAtFrame { store,reg,offset } => format!("{}wi37 {},[$fp+{}]",if store{"s"}else{"l"},r(reg),offset),
            ShiftRightSmall { reg,amount } => format!("srli45 {},{}",r(reg),amount),
            ShiftArithmeticRightSmall {reg,amount} => format!("srai45 {},{}",r(reg),amount),
            ShiftRight { dest,source,amount } => format!("srli {},{},{}",r(dest),r(source),amount),
            AluShiftLeft { op,dest,left,right,shift } => format!("{}_slli {},{},{},{}",match op{AluOp::Add=>"add",AluOp::Or=>"or"},r(dest),r(left),r(right),shift),
            VariableShiftLeft { dest,source,amount } => format!("sll {},{},{}",r(dest),r(source),r(amount)),
            VariableShiftRight { dest,source,amount } => format!("srl {},{},{}",r(dest),r(source),r(amount)),
            VariableArithmeticShiftRight{dest,source,amount} => format!("sra {},{},{}",r(dest),r(source),r(amount)),
            ConditionalMove{dest,source,condition,nonzero} =>
                format!("{} {},{},{}",if nonzero{"cmovn"}else{"cmovz"},r(dest),r(source),r(condition)),
            SignExtendHalfWide{dest,source} => format!("seh {},{}",r(dest),r(source)),
            BranchImmediate { reg,value,nonzero,target } => format!("{} {},{},.{:+}",if nonzero{"bnec"}else{"beqc"},r(reg),value,i64::from(target)-i64::from(pc)),
            BranchA5 { reg,nonzero,target } => format!("{} {},.{:+}",if nonzero{"bnes38"}else{"beqs38"},r(reg),i64::from(target)-i64::from(pc)),
            AddSmall { reg,value,subtract } => format!("{} {},{}",if subtract{"subi45"}else{"addi45"},r(reg),value),
            AddSmallRegister { dest,source,subtract } => format!("{} {},{}",if subtract{"sub45"}else{"add45"},r(dest),r(source)),
            ZeroExtend { dest,source,half } => format!("{} {},{}",if half{"zeh33"}else{"zeb33"},r(dest),r(source)),
            SignExtendByte { dest,source } => format!("seb33 {},{}",r(dest),r(source)),
            SignExtendHalf { dest,source } => format!("seh33 {},{}",r(dest),r(source)),
            ShiftLeftSmall3{dest,source,amount} => format!("slli333 {},{},{}",r(dest),r(source),amount),
            AddSmall3{dest,source,value} => format!("addi333 {},{},{}",r(dest),r(source),value),
            SubtractSmall3 { dest,source,value } => format!("subi333 {},{},{}",r(dest),r(source),value),
            CompactAlu3 { dest,left,right,subtract } => format!("{} {},{},{}",if subtract {"sub333"}else{"add333"},r(dest),r(left),r(right)),
            LessThanSmallImmediate { reg,value } => format!("slti45 {},{}",r(reg),value),
            LessThanSmallRegister {left,right} => format!("slt45 {},{}",r(left),r(right)),
            LessThanSignedSmallRegister {left,right} => format!("slts45 {},{}",r(left),r(right)),
            LessThanSignedSmallImmediate {reg,value} => format!("sltsi45 {},{}",r(reg),value),
            BranchTa { nonzero,target } => format!("{} .{:+}",if nonzero{"bnezs8"}else{"beqzs8"},i64::from(target)-i64::from(pc)),
            BitMaskShort { reg,bit } => format!("bmski33 {},{}",r(reg),bit),
            BitFieldExtractImmediate {reg,bit} => format!("fexti33 {},{}",r(reg),bit),
            CompactOr { dest,source } => format!("or33 {},{}",r(dest),r(source)),
            CompactMiscOp { dest,source,op } => format!("{} {},{}",op.mnemonic(),r(dest),r(source)),
            OrImmediate { dest,source,value } => format!("ori {},{},{}",r(dest),r(source),value),
            XorImmediate{dest,source,value} => format!("xori {},{},{}",r(dest),r(source),value),
            AndImmediate { dest,source,value } => format!("andi {},{},{}",r(dest),r(source),value),
            BitTestImmediate{dest,source,bit} => format!("btst {},{},{}",r(dest),r(source),bit),
            BitSet { dest,source,bit } => format!("bset {},{},{}",r(dest),r(source),bit),
            BitClearAt { dest,source,bit } => format!("bclr {},{},{}",r(dest),r(source),bit),
            IndexedPostIndexedMemory{store,width,reg,base,index,shift} => format!("{}{}i.bi {},[{}],({},<<{})",if store{"s"}else{"l"},width.letter(),r(reg),r(base),r(index),shift),
            IndexedMemory {store,width,signed,reg,base,index,shift} => format!("{}{}{} {},[{}+({}<<{})]",if store{"s"}else{"l"},width.letter(),if signed{"s"}else{""},r(reg),r(base),r(index),shift),
            IndexedByte { store,reg,base,index,shift } => format!("{}b {},[{}+({}<<{})]",if store{"s"}else{"l"},r(reg),r(base),r(index),shift),
            JumpShort { target } => format!("j8 .{:+}",i64::from(target)-i64::from(pc)),
            JumpRelative { target,link } => format!("{} .{:+}",if link{"jal"}else{"j"},i64::from(target)-i64::from(pc)),
            // GNU's IFCALL9 operand is a displacement, not a target expression.
            InlineCall { target } => format!("ifcall9 {}",i64::from(target)-i64::from(pc)),
            InlineCallWide { target } => format!("ifcall {}",i64::from(target)-i64::from(pc)),
            InlineReturn => "ifret16".into(),
            Return { reg } => format!("ret5 {}",r(reg)),
            JumpRegisterLink { reg } => format!("jral5 {}",r(reg)),
            JumpRegister { reg } => format!("jr5 {}",r(reg)),
            JumpRegisterWide{reg} => format!("jr {}",r(reg)),
            JumpTableOffsets16{base,targets} => format!("jump.offsets16 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets15{base,targets} => format!("jump.offsets15 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets12{base,targets} => format!("jump.offsets12 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            JumpTableOffsets6{base,targets} => format!("jump.offsets6 0x{base:x} {}",targets.iter().map(|x|format!("0x{x:x}")).collect::<Vec<_>>().join(",")),
            DataSync => "dsb".into(),
            Stack { pop,last,extra } => format!("{} {},{}",if pop{"pop25"}else{"push25"},r(last),extra),
            MultipleWords { store,first,base,last,specials,decrement,writeback } => format!("{}mw.{}{} {},[{}],{},{}",
                if store{"s"}else{"l"},if decrement{"ad"}else{"bi"},if writeback{"m"}else{""},r(first),r(base),r(last),specials),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn thirteen_halfword_switch_rejects_invalid_target(){
        let a=0xf3b4_u32;
        let targets=[0xf3ce,0xf43e,0xf49e,0xf4de,0xf51a,0xf55a,0xf564,
                     0xf56e,0xf582,0xf5ba,0xf58c,0xf5c0,0xf5c6];
        let ins=Insn::JumpTableOffsets13{base:a,targets};
        let encoded=ins.encode(a,0).unwrap();
        assert_eq!(encoded.len(),26);
        let mut bad=targets;bad[1]=a+1;
        assert!(Insn::JumpTableOffsets13{base:a,targets:bad}.encode(a,0).is_err());
    }

    #[test]
    fn five_six_byte_and_four_halfword_tables_are_strict(){
        let five=Insn::JumpTableOffsets5Padded{base:0xbedc,
            targets:[0xbee2,0xbee8,0xbee8,0xbef2,0xbef8]};
        assert_eq!(five.encode(0,0).unwrap(),[6,12,12,22,28,0]);
        let four=Insn::JumpTableOffsets4Halfwords{base:0xbf20,
            targets:[0xbf28,0xbf54,0xbf70,0xc022]};
        assert_eq!(four.encode(0,0).unwrap(),[8,0,52,0,80,0,2,1]);
        let six=Insn::JumpTableOffsets6Bytes{base:0xdb94,
            targets:[0xdb9a,0xdbb8,0xdbd4,0xdc62,0xdbf4,0xdc2a]};
        assert_eq!(six.encode(0,0).unwrap(),[6,36,64,206,96,150]);
        let invalid=Insn::JumpTableOffsets5Padded{base:0xbedc,targets:[0xbee2,0xbee3,0xbee8,0xbef2,0xbef8]};
        assert!(invalid.encode(0,0).is_err());
    }

    #[test]
    fn wide_sign_extend_byte_encodes_signed_byte_lanes(){
        let r=Reg::new;
        assert_eq!(Insn::SignExtendByteWide{dest:r(9),source:r(2)}
           .encode(0xb3fa,0x2d9e0).unwrap(),[0x40,0x91,0x00,0x10]);
    }

    #[test]
    fn postindexed_indexed_loads_have_correct_mem_subop_and_reject_aliasing(){
        let r=Reg::new;
        assert_eq!(Insn::IndexedPostIndexedMemory{store:false,width:Width::Half,
            reg:r(0),base:r(1),index:r(4),shift:0}.encode(0xa862,0x2d9e0).unwrap(),
            [0x38,0x00,0x90,0x05]);
        assert!(Insn::IndexedPostIndexedMemory{store:false,width:Width::Half,
            reg:r(1),base:r(1),index:r(4),shift:0}.encode(0xa862,0).is_err());
        assert!(Insn::IndexedPostIndexedMemory{store:true,width:Width::Word,
            reg:r(0),base:r(1),index:r(4),shift:4}.encode(0xa862,0).is_err());
    }

    #[test]
    fn abs_and_shifted_right_alu_are_checked_and_have_distinct_opcodes(){
        let r=Reg::new;
        assert_eq!(Insn::AbsoluteRegister{dest:r(1),source:r(5)}
            .encode(0x78b0,0x2d9e0).unwrap(),[0x42,0x12,0x80,0x03]);
        assert_eq!(Insn::RegisterAluShiftRight{op:AluRegOp::Add,dest:r(2),left:r(2),right:r(17),shift:4}
            .encode(0x7a86,0x2d9e0).unwrap(),[0x40,0x21,0x44,0x9c]);
        assert!(Insn::RegisterAluShiftRight{op:AluRegOp::Slts,dest:r(2),left:r(2),right:r(17),shift:4}
            .encode(0x7a86,0x2d9e0).is_err());
    }
    #[test]
    fn sized_jump_tables_validate_offset_and_alignment(){
        let twelve:[u32;12]=std::array::from_fn(|i|0x8d10+((i+1)*2) as u32);
        assert_eq!(Insn::JumpTableOffsets12Bytes{base:0x8d10,targets:twelve}
            .encode(0,0).unwrap(),(1u8..=12).map(|x|x*2).collect::<Vec<_>>());
        let fourteen:[u32;14]=std::array::from_fn(|i|0x8c44+(i*2) as u32);
        assert_eq!(Insn::JumpTableOffsets14{base:0x8c44,targets:fourteen}.encode(0,0).unwrap().len(),28);
        let sixteen:[u32;16]=std::array::from_fn(|i|0x9cf0+(i*2) as u32);
        assert_eq!(Insn::JumpTableOffsets16Entries{base:0x9cf0,targets:sixteen}.encode(0,0).unwrap().len(),32);
        let mut bad=twelve;bad[7]=0x8d10+300;
        assert!(Insn::JumpTableOffsets12Bytes{base:0x8d10,targets:bad}.encode(0,0).is_err());
    }

    use super::*;
    #[test]
    fn six_and_twelve_way_offsets_are_le_halfwords(){
        let six=Insn::JumpTableOffsets6{base:0x4e18,targets:[0x4e24,0x4e4c,0x4e76,0x4f38,0x4f3e,0x4e7c]};
        assert_eq!(six.encode(0,0).unwrap(),[0x0c,0x00,0x34,0x00,0x5e,0x00,0x20,0x01,0x26,0x01,0x64,0x00]);
        let twelve=Insn::JumpTableOffsets12{base:0x4dac,targets:[0x4dc4,0x4dfe,0x4f68,0x4f80,0x4fe6,0x4ff4,0x5010,0x504e,0x504e,0x504e,0x5020,0x503c]};
        let data=twelve.encode(0,0).unwrap();
        assert_eq!(data.len(),24);
        assert_eq!(&data[0..6],[0x18,0,0x52,0,0xbc,0x01]);
        assert_eq!(&data[20..],[0x74,0x02,0x90,0x02]);
        let mut bad=[0x4dc4;12];bad[5]=0x4dad;
        assert!(Insn::JumpTableOffsets12{base:0x4dac,targets:bad}.encode(0,0).is_err());
    }
    #[test]
    fn alu2_bclr_preserves_index_and_rejects_overflow(){
        let b=Insn::BitClearAt{dest:R0,source:R0,bit:27};
        assert_eq!(b.encode(0,0).unwrap(),[0x42,0x00,0x6c,0x09]);
        assert_eq!(b.assembly(0,0),"bclr $r0,$r0,27");
        assert!(Insn::BitClearAt{dest:R0,source:R0,bit:32}.encode(0,0).is_err());
    }
    #[test]
    fn fifteen_way_jump_table_is_little_endian_data() {
        let t=[0x4554,0x455e,0x4568,0x45c6,0x45ec,0x45f2,0x462a,
               0x4604,0x4610,0x4630,0x454e,0x461e,0x462a,0x462a,0x4624];
        let encoded=Insn::JumpTableOffsets15{base:0x4530,targets:t}.encode(0,0).unwrap();
        assert_eq!(encoded.len(),30);
        assert_eq!(&encoded[..6],[0x24,0x00,0x2e,0x00,0x38,0x00]);
        assert_eq!(&encoded[28..],[0xf4,0x00]);
        let mut broken=t;broken[5]=0x4531;
        assert!(Insn::JumpTableOffsets15{base:0x4530,targets:broken}.encode(0,0).is_err());
    }
    #[test]
    fn nds32_srai45_compact_register_signed_shift() {
        let x=Insn::ShiftArithmeticRightSmall{reg:R1,amount:8};
        assert_eq!(x.encode(0,0).unwrap(),[0x90,0x28]);
        assert_eq!(x.assembly(0,0),"srai45 $r1,8");
        assert_ne!(x.encode(0,0),Insn::ShiftRightSmall{reg:R1,amount:8}.encode(0,0));
        assert!(Insn::ShiftArithmeticRightSmall{reg:R1,amount:32}.encode(0,0).is_err());
        assert!(Insn::ShiftArithmeticRightSmall{reg:Reg::new(20),amount:1}.encode(0,0).is_err());
    }
    #[test]
    fn nds32_sra_register_preserves_signed_shift_opcode() {
        let example=Insn::VariableArithmeticShiftRight{dest:R0,source:R0,amount:R1};
        assert_eq!(example.encode(0,0).unwrap(),[0x40,0x00,0x04,0x0e]);
        assert_eq!(example.assembly(0,0),"sra $r0,$r0,$r1");
        assert_ne!(example.encode(0,0),Insn::VariableShiftRight{dest:R0,source:R0,amount:R1}.encode(0,0));
    }

    #[test]
    fn gprel_signed_byte_uses_distinct_lbsi_opcode_bit() {
        let addr=0x2c6b9;
        let gp=0x2d9e0;
        assert_eq!(Insn::LoadSignedByteGp{dest:R0,address:addr}.encode(0,gp).unwrap(),[0x2e,0x0f,0xec,0xd9]);
        assert_eq!(Insn::LoadByteGp{dest:R0,address:addr}.encode(0,gp).unwrap(),[0x2e,0x07,0xec,0xd9]);
        assert_eq!(Insn::LoadSignedByteGp{dest:R0,address:addr}.assembly(0,gp),"lbsi.gp $r0,[-4903]");
        assert!(Insn::LoadSignedByteGp{dest:R0,address:0x1000000}.encode(0,gp).is_err());
    }

    #[test]
    fn variable_shift_right_matches_nds32_alu1_srl() {
        let r=Reg::new;
        assert_eq!(Insn::VariableShiftRight{dest:r(4),source:r(5),amount:r(4)}
            .encode(0x3792,0).unwrap(),[0x40,0x42,0x90,0x0d]);
        assert_eq!(Insn::VariableShiftRight{dest:r(4),source:r(5),amount:r(4)}
            .assembly(0x3792,0),"srl $r4,$r5,$r4");
        assert_ne!(Insn::VariableShiftRight{dest:r(4),source:r(5),amount:r(4)}
            .encode(0,0).unwrap(), Insn::VariableShiftLeft{dest:r(4),source:r(5),amount:r(4)}
            .encode(0,0).unwrap());
    }

    #[test]
    fn indexed_halfword_case_jump_and_little_endian_data_table() {
        let r15=Reg::new(15);
        assert_eq!(Insn::JumpRegisterWide{reg:r15}.encode(0x3828,0).unwrap(),[0x4a,0x00,0x3c,0x00]);
        let targets=std::array::from_fn(|i|0x382c+2*i as u32);
        let bytes=Insn::JumpTableOffsets16{base:0x382c,targets}.encode(0x382c,0).unwrap();
        assert_eq!(bytes.len(),70);
        assert_eq!(&bytes[..6], &[0,0,2,0,4,0]);
        let mut bad=targets;bad[1]=0x382f;
        assert!(Insn::JumpTableOffsets16{base:0x382c,targets:bad}.encode(0,0).is_err());
        bad=targets;bad[3]=0x1382c;
        assert!(Insn::JumpTableOffsets16{base:0x382c,targets:bad}.encode(0,0).is_err());
    }

    #[test]
    fn jr5_preserves_nonlinking_semantics(){
        assert_eq!(Insn::JumpRegister {reg:Reg::new(15)}.encode(0,0).unwrap(),[0xdd,0x0f]);
        assert_eq!(Insn::JumpRegisterLink {reg:Reg::new(15)}.encode(0,0).unwrap(),[0xdd,0x2f]);
        assert_ne!(Insn::JumpRegister {reg:Reg::new(15)}.assembly(0,0),
            Insn::JumpRegisterLink {reg:Reg::new(15)}.assembly(0,0));
    }

    #[test]
    fn wide_register_compare_branch_encodes_br1_fields(){
        assert_eq!(Insn::CompareRegistersBranch {left:Reg::new(5),right:Reg::new(12),
            nonzero:true,target:0x229c}.encode(0x2242,0).unwrap(),[0x4c,0x56,0x40,0x2d]);
        assert_eq!(Insn::CompareRegistersBranch {left:R1,right:R0,
            nonzero:false,target:0x1004}.encode(0x1000,0).unwrap(),[0x4c,0x10,0,2]);
        assert!(Insn::CompareRegistersBranch {left:R1,right:R2,
            nonzero:false,target:0x1003}.encode(0x1000,0).is_err());
    }

    #[test]
    fn semantic_jumptable_offsets_from_targets() {
        let targets=[0x252e,0x2536,0x253a,0x253e,0x2542,0x2554,0x2554,0x2546,0x254a,0x2550];
        assert_eq!(Insn::JumpTableOffsets10{base:0x2524,targets}.encode(0,0).unwrap(),
            [10,18,22,26,30,48,48,34,38,44]);
        let mut bad=targets;bad[0]=0x2626;
        assert!(Insn::JumpTableOffsets10{base:0x2524,targets:bad}.encode(0,0).is_err());
        bad=targets;bad[1]=0x2537;
        assert!(Insn::JumpTableOffsets10{base:0x2524,targets:bad}.encode(0,0).is_err());
    }

    #[test]
    fn wide_sign_extend_halfword_preserves_alu1_registers() {
        let r=Reg::new;
        assert_eq!(Insn::SignExtendHalfWide{dest:r(23),source:r(1)}
            .encode(0,0).unwrap(),[0x41,0x70,0x80,0x11]);
        assert_eq!(Insn::SignExtendHalfWide{dest:r(23),source:r(1)}
            .assembly(0,0),"seh $r23,$r1");
    }

    #[test]
    fn unsigned_minimum_alu2_subopcode_has_named_operands(){
        let r=Reg::new;
        assert_eq!(Insn::MinimumRegister{dest:r(0),left:r(1),right:r(0)}
            .encode(0,0).unwrap(),[0x42,0x00,0x80,0x01]);
        assert_eq!(Insn::MinimumRegister{dest:r(0),left:r(1),right:r(0)}
            .assembly(0,0),"min $r0,$r1,$r0");
    }

    #[test]
    fn conditional_move_on_nonzero_preserves_n32_alu1_semantics() {
        let r=Reg::new;
        let on=Insn::ConditionalMove{dest:r(4),source:r(1),condition:r(15),nonzero:true};
        assert_eq!(on.encode(0,0).unwrap(),[0x40,0x40,0xbc,0x1b]);
        assert_eq!(on.assembly(0,0),"cmovn $r4,$r1,$r15");
        let off=Insn::ConditionalMove{dest:r(4),source:r(1),condition:r(15),nonzero:false};
        assert_eq!(off.encode(0,0).unwrap(),[0x40,0x40,0xbc,0x1a]);
    }

    #[test]
    fn eight_way_indirect_dispatch_byte_offsets_are_checked() {
        let dst=[0x5f24,0x5f3e,0x5f50,0x5f62,
            0x5f8e,0x5fa8,0x5fba,0x5fcc];
        assert_eq!(Insn::JumpTableOffsets8{base:0x5f1c,targets:dst}
            .encode(0,0).unwrap(),[8,34,52,70,114,140,158,176]);
        let mut invalid=dst;invalid[0]=0x5f23;
        assert!(Insn::JumpTableOffsets8{base:0x5f1c,targets:invalid}
            .encode(0,0).is_err());
        invalid=dst;invalid[0]=0x5f1c+256;
        assert!(Insn::JumpTableOffsets8{base:0x5f1c,targets:invalid}
            .encode(0,0).is_err());
    }

    #[test]
    fn signed_postindex_halfword_and_byte_have_checked_isa_fields() {
        let reg=Reg::new;
        assert_eq!(Insn::SignedPostIndexedMemory{width:Width::Half,dest:reg(5),
            base:reg(1),offset:2}.encode(0,0).unwrap(),[0x2a,0x50,0x80,0x01]);
        assert_eq!(Insn::SignedPostIndexedMemory{width:Width::Half,dest:reg(5),
            base:reg(1),offset:-2}.encode(0,0).unwrap(),[0x2a,0x50,0xff,0xff]);
        assert_eq!(Insn::SignedPostIndexedMemory{width:Width::Byte,dest:reg(5),
            base:reg(1),offset:1}.encode(0,0).unwrap(),[0x28,0x50,0x80,0x01]);
        assert!(Insn::SignedPostIndexedMemory{width:Width::Word,dest:reg(5),
            base:reg(1),offset:4}.encode(0,0).is_err());
        assert!(Insn::SignedPostIndexedMemory{width:Width::Half,dest:reg(5),
            base:reg(1),offset:3}.encode(0,0).is_err());
        assert!(Insn::SignedPostIndexedMemory{width:Width::Half,dest:reg(1),
            base:reg(1),offset:2}.encode(0,0).is_err());
    }

    #[test]
    fn n16_movd44_register_pair_encoding_and_reserved_operands() {
        let r=Reg::new;
        for (dest,source,bytes) in [(4,0,[0xfd,0x20]),
                                     (6,12,[0xfd,0x36]),
                                     (2,8,[0xfd,0x14])] {
            let pair=Insn::MoveDoubleCompact{dest_even:r(dest),source_even:r(source)};
            assert_eq!(pair.encode(0,0).unwrap(),bytes);
            assert_eq!(pair.assembly(0,0),format!("movd44 $r{dest},$r{source}"));
        }
        for (dest,source) in [(1,0),(2,3),(31,0),(0,31)] {
            assert!(Insn::MoveDoubleCompact{dest_even:r(dest),source_even:r(source)}
                .encode(0,0).is_err());
        }
    }

    #[test]
    fn checked_n16_word_postindex_matches_architecture() {
        assert_eq!(Insn::ShortPostIndexedWord{store:false,reg:R3,base:Reg::new(5),offset:4}.encode(0,0).unwrap(),[0xa2,0xe9]);
        assert_eq!(Insn::ShortPostIndexedWord{store:true,reg:R1,base:R4,offset:12}.encode(0,0).unwrap(),[0xaa,0x63]);
        assert!(Insn::ShortPostIndexedWord{store:false,reg:R0,base:R0,offset:0}.encode(0,0).is_err());
        assert!(Insn::ShortPostIndexedWord{store:true,reg:R0,base:R1,offset:30}.encode(0,0).is_err());
    }

    #[test]
    fn xor_immediate_encoding_and_operand_limits() {
        assert_eq!(Insn::XorImmediate{dest:R0,source:R0,value:1}.encode(0,0).unwrap(),
            [0x56,0x00,0x00,0x01]);
        assert!(Insn::XorImmediate{dest:R1,source:R2,value:32768}.encode(0,0).is_err());
    }

    #[test]
    fn checked_postindexed_doubleword_offsets_and_pairs() {
        assert_eq!(Insn::DoubleWordMemory{store:true,postindexed:true,first:Reg::new(3),
            base:R0,offset:98576}.encode(0,0).unwrap(),[0x1e,0x30,0x30,0x22]);
        assert_eq!(Insn::DoubleWordMemory{store:false,postindexed:false,first:R2,
            base:R4,offset:-8}.encode(0,0).unwrap(),
            ((3u32<<25)|(2<<20)|(4<<15)|0x7fff).to_be_bytes());
        assert!(Insn::DoubleWordMemory{store:true,postindexed:true,first:R3,
            base:R4,offset:3}.encode(0,0).is_err());
        assert!(Insn::DoubleWordMemory{store:false,postindexed:true,first:Reg::new(31),
            base:R1,offset:0}.encode(0,0).is_err());
        assert!(Insn::DoubleWordMemory{store:false,postindexed:true,first:R1,
            base:R2,offset:0}.encode(0,0).is_err());
    }

    #[test]
    fn checked_post_indexed_memory_matches_nds32_isa() {
        assert_eq!(Insn::PostIndexedMemory{store:false,width:Width::Half,reg:R1,
            base:R4,offset:11316}.encode(0,0).unwrap(), [0x0a,0x12,0x16,0x1a]);
        assert_eq!(Insn::PostIndexedMemory{store:true,width:Width::Word,reg:R2,
            base:R3,offset:-16}.encode(0,0).unwrap(),
            ((0xeu32<<25)|(2<<20)|(3<<15)|0x7ffc).to_be_bytes());
        assert!(Insn::PostIndexedMemory{store:true,width:Width::Half,reg:R1,
            base:R2,offset:3}.encode(0,0).is_err());
        assert!(Insn::PostIndexedMemory{store:false,width:Width::Word,reg:R1,
            base:R2,offset:65536}.encode(0,0).is_err());
        assert!(Insn::PostIndexedMemory{store:false,width:Width::Word,reg:R1,
            base:R1,offset:4}.encode(0,0).is_err());
    }

    #[test]
    fn bit_test_immediate_alu2_field_matches_gnu_instruction_table() {
        assert_eq!(Insn::BitTestImmediate{dest:R1,source:R1,bit:1}.encode(0,0).unwrap(),[0x42,0x10,0x84,0x0b]);
        assert!(Insn::BitTestImmediate{dest:R1,source:R1,bit:32}.encode(0,0).is_err());
    }
    #[test]
    fn wide_ifcall_preserves_signed_halfword_displacement() {
        assert_eq!(Insn::InlineCallWide{target:0x3c1c}.encode(0x3c2a,0).unwrap(), [0x4e,0x00,0xff,0xf9]);
        assert_eq!(Insn::InlineCallWide{target:0x1020}.encode(0x1000,0).unwrap(), [0x4e,0x00,0x00,0x10]);
        assert!(Insn::InlineCallWide{target:0x1001}.encode(0x1000,0).is_err());
        assert!(Insn::InlineCallWide{target:0x20000}.encode(0x1000,0).is_err());
    }

    #[test]
    fn lwi45_fe_uses_implicit_r8_and_negative_word_aligned_offset(){
        assert_eq!(Insn::WordAtFrameEnd {dest:R0,offset:-32}.encode(0,0).unwrap(), [0xb2,0x18]);
        assert_eq!(Insn::WordAtFrameEnd {dest:R1,offset:-128}.encode(0,0).unwrap(), [0xb2,0x20]);
        assert_eq!(Insn::WordAtFrameEnd {dest:Reg::new(16),offset:-4}.encode(0,0).unwrap(),[0xb3,0x9f]);
        assert!(Insn::WordAtFrameEnd {dest:Reg::new(20),offset:-32}.encode(0,0).is_err());
        for bad in [-132,-126,0,4] {
            assert!(Insn::WordAtFrameEnd {dest:R0,offset:bad}.encode(0,0).is_err());
        }
    }

    #[test]
    fn short_signed_comparisons_halfword_gp_and_register_add() {
        assert_eq!(Insn::AddSmallRegister{dest:R0,source:R1,subtract:false}.encode(0,0).unwrap(),[0x88,0x01]);
        assert_eq!(Insn::AddSmallRegister{dest:Reg::new(16),source:R1,subtract:true}.encode(0,0).unwrap(),[0x8b,0x81]);
        assert!(Insn::AddSmallRegister{dest:Reg::new(12),source:R1,subtract:false}.encode(0,0).is_err());
        assert_eq!(Insn::LessThanSmallRegister{left:R1,right:R2}.encode(0,0).unwrap(),[0xe2,0x22]);
        assert_eq!(Insn::LessThanSignedSmallRegister{left:R1,right:R2}.encode(0,0).unwrap(),[0xe0,0x22]);
        assert_eq!(Insn::LessThanSignedSmallImmediate{reg:R1,value:2}.encode(0,0).unwrap(),[0xe4,0x22]);
        assert!(Insn::LessThanSignedSmallImmediate{reg:R0,value:32}.encode(0,0).is_err());
        assert_eq!(Insn::BitFieldExtractImmediate{reg:R1,bit:2}.encode(0,0).unwrap(),[0x96,0x57]);
        assert!(Insn::HalfGp{store:true,signed:true,reg:R0,address:0x2d9e0}.encode(0,0x2d9e0).is_err());
        assert!(Insn::HalfGp{store:true,signed:false,reg:R0,address:0x2d9e1}.encode(0,0x2d9e0).is_err());
        assert_eq!(Insn::HalfGp{store:false,signed:false,reg:R0,address:0x2d9e0}.encode(0,0x2d9e0).unwrap(),[0x3c,0,0,0]);
    }
    #[test]
    fn extended_instruction_forms_encode_checked_operands() {
        assert_eq!(Insn::ArithmeticShiftRight{dest:R2,source:R5,amount:31}
            .encode(0,0).unwrap(),0x4022_fc0au32.to_be_bytes());
        assert_eq!(Insn::RegisterAlu{op:AluRegOp::Sub,dest:R1,left:Reg::new(16),right:R2,shift:0}
            .encode(0,0).unwrap(),0x4018_0801u32.to_be_bytes());
        assert_eq!(Insn::IndexedMemory{store:false,width:Width::Half,signed:true,
            reg:R1,base:Reg::new(7),index:R0,shift:1}
            .encode(0,0).unwrap(),0x3813_8111u32.to_be_bytes());
        assert_eq!(Insn::WordAtStack{store:false,reg:R0,offset:4}
            .encode(0,0).unwrap(),0xf001u16.to_be_bytes());
        assert!(Insn::ArithmeticShiftRight{dest:R0,source:R1,amount:32}.encode(0,0).is_err());
        assert!(Insn::RegisterAlu{op:AluRegOp::Sub,dest:R0,left:R1,right:R2,shift:32}
            .encode(0,0).is_err());
        assert!(Insn::IndexedMemory{store:true,width:Width::Half,signed:true,
            reg:R0,base:R1,index:R2,shift:0}.encode(0,0).is_err());
        assert!(Insn::WordAtStack{store:false,reg:R0,offset:3}.encode(0,0).is_err());
        assert!(Insn::WordAtStack{store:false,reg:Reg::new(8),offset:4}.encode(0,0).is_err());
        assert!(Insn::LessThanWide{dest:R0,source:R1,value:32768,signed:true}.encode(0,0).is_err());
        assert!(Insn::ReadSystem{dest:R0,csr:Csr::Number(0x400)}.encode(0,0).is_err());
        assert!(Insn::StackAdjust{bytes:-513}.encode(0,0).is_err());
    }
    #[test]
    fn sensor_dsp_and_signed_memory_forms_have_strict_limits(){
        assert_eq!(Insn::AddSmall3{dest:R1,source:R2,value:7}.encode(0,0).unwrap(),
            0x9c57_u16.to_be_bytes());
        assert!(Insn::ShiftLeftSmall3{dest:R1,source:R2,amount:8}.encode(0,0).is_err());
        assert!(Insn::AddSmall3{dest:R1,source:R2,value:8}.encode(0,0).is_err());
        assert!(Insn::SignedMemoryImmediate{width:Width::Half,dest:R1,base:R2,offset:3}
            .encode(0,0).is_err());
        assert!(Insn::SignedMemoryImmediate{width:Width::Word,dest:R1,base:R2,offset:4}
            .encode(0,0).is_err());
        assert_eq!(Insn::SignedMemoryImmediate{width:Width::Byte,dest:R1,base:R2,offset:-1}
            .encode(0,0).unwrap(), ((0x10u32<<25)|(1<<20)|(2<<15)|0x7fff).to_be_bytes());
        assert!(Insn::OrShiftRight{dest:R0,left:R1,right:R2,shift:32}.encode(0,0).is_err());
        assert!(Insn::ShiftLeftImmediate{dest:R0,source:R1,amount:32}.encode(0,0).is_err());
        assert!(Insn::ReverseSubtractImmediate{dest:R0,source:R1,value:-16385}
            .encode(0,0).is_err());
        assert_eq!(Insn::MaximumRegister{dest:R0,left:R1,right:R2}
            .encode(0,0).unwrap(), ((0x21u32<<25)|(1<<15)|(2<<10)).to_be_bytes());
        assert_eq!(Insn::MultiplyRegister64{dest:R0,left:R1,right:R2,signed:true}
            .encode(0,0).unwrap(),((0x21u32<<25)|(1<<15)|(2<<10)|0x68).to_be_bytes());
    }
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
    #[test]
    fn startup_memory_ranges_and_multiple_register_masks_are_checked() {
        assert!(Insn::StoreWordGp{source:R0,address:3}.encode(0,0).is_err());
        assert!(Insn::StoreWordGp{source:R0,address:0x40000}.encode(0,0).is_err());
        assert!(Insn::LoadByteGp{dest:R0,address:0x40000}.encode(0,0).is_err());
        assert!(Insn::ImmediateMemory{store:true,width:Width::Word,reg:R0,base:R1,offset:-2}.encode(0,0).is_err());
        assert!(Insn::ImmediateMemory{store:false,width:Width::Word,reg:R0,base:R1,offset:0x10000}.encode(0,0).is_err());
        assert!(Insn::MultipleWords{store:true,first:R6,base:Reg::new(31),last:R5,specials:2,decrement:true,writeback:true}.encode(0,0).is_err());
        assert!(Insn::MultipleWords{store:true,first:R6,base:Reg::new(31),last:Reg::new(8),specials:16,decrement:true,writeback:true}.encode(0,0).is_err());
    }
    #[test]
    fn startup_immediate_boundaries_reject_truncation() {
        for value in [15,48] {assert!(Insn::MovePositive{dest:R0,value}.encode(0,0).is_err());}
        assert!(Insn::MovePositive{dest:Reg::new(12),value:16}.encode(0,0).is_err());
        assert!(Insn::SubtractSmall3{dest:R0,source:R1,value:8}.encode(0,0).is_err());
        assert!(Insn::LessThanSmallImmediate{reg:R0,value:32}.encode(0,0).is_err());
        assert!(Insn::AndImmediate{dest:R0,source:R1,value:0x8000}.encode(0,0).is_err());
        assert!(Insn::ShiftRight{dest:R0,source:R1,amount:32}.encode(0,0).is_err());
        assert!(Insn::BranchZeroWide{reg:R0,nonzero:false,target:0x10000}.encode(0,0).is_err());
        assert!(Insn::BranchTa{nonzero:false,target:3}.encode(0,0).is_err());
    }
}
