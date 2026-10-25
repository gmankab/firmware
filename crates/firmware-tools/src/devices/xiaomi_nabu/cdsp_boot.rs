// SPDX-License-Identifier: GPL-3.0-only
//! Independently encoded Qualcomm CDSP Hexagon entry vectors.
//!
//! This module emits the first 17 decoded, four-byte instruction packets
//! beginning at the reference's first executable PT_LOAD, file offset 0x3000.
//! Some privileged V66 system-register opcodes are outside the available LLVM
//! 6 assembler, and are reconstructed with checked architecture bitfields
//! corroborated by the Rizin Hexagon decoder. No OEM instruction bytes are
//! imported as literal arrays.
//!
//! All direct-jump displacements are relative to the instruction's address
//! and counted in **bytes**. The Hexagon immediate stores halfword units.
//!
//! Only the evidenced scalar forms are implemented. This is a partial
//! firmware reconstruction, not a bootable or authenticated image.

use crate::parse_elf32;

pub const START_FILE_OFFSET: usize = 0x3000;
pub const REBUILT_LEN: usize = 9 * 4;
/// Entry jump vectors and the entire following supervisor configuration.
pub const INIT_PREFIX_LEN: usize = 0x44;
pub const ENTRY_PHYS: u32 = 0x96c0_0000;
pub const ENTRY_ELF_VADDR: u32 = 0xfe00_0000;
pub const FIRST_EXECUTABLE_SIZE: u32 = 0x37b34;
pub const HEXAGON_ELF_MACHINE: u16 = 164;
pub const CDSP_ELF_FLAGS: u32 = 0x66;

/// Register operand for the supported subset of Hexagon V6 instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneralRegister(u8);

impl GeneralRegister {
    pub fn new(index: u8) -> Result<Self, String> {
        if index > 31 {
            return Err(format!("Hexagon register out of range: r{index}"));
        }
        Ok(Self(index))
    }

    pub fn number(self) -> u8 { self.0 }
}

/// Independently evidenced scalar instructions, restricted to operand forms
/// whose bit fields have been cross-checked against LLVM or the Hexagon
/// decoder. No duplex-instruction support is implied by these constructors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarPacket {
    /// One four-byte instruction packet, `jump` with a relative byte offset.
    /// This narrow encoding only supports nonnegative, halfword-aligned
    /// displacement up to 0x1ffe. Larger or negative branches require the
    /// full Hexagon immediate/extension encoding and deliberately fail.
    JumpRelative { displacement_bytes: u32 },
    /// One four-byte instruction packet `rN = #unsigned_small`.
    /// Only the 7-bit subset independently checked against the assembler is
    /// accepted; no sign extension is inferred from the original firmware.
    AssignSmallImmediate { dst: GeneralRegister, value: u8 },
    /// Hexagon V66 special-register write (`SSR=Rn` or `SYSCFG=Rn`).
    /// The selectors were independently decoded with Rizin's Hexagon backend.
    WriteSystem { system: SystemRegister, src: GeneralRegister },
    /// Hexagon V66 `Rn = SYSCFG`; only this documented register is supported.
    ReadSyscfg { dst: GeneralRegister },
    /// `Rdest=add(Rsrc,##signed32)` extended immediate pair. The result
    /// is TWO Hexagon instruction words; its consumer may be part of a
    /// larger packet, as in the CDSP initial TLB walker.
    AddExtendedImmediate { dst: GeneralRegister, src: GeneralRegister, value: i32 },
    /// Hexagon post-incrementing 64-bit load to an even/odd register pair.
    LoadDoublePostIncrement { low: GeneralRegister, base: GeneralRegister, increment: u8 },
    /// `Rd=extractu(Rs,#width,#offset)` using a 32-bit source register.
    ExtractUnsigned { dst: GeneralRegister, src: GeneralRegister, width: u8, offset: u8 },
    /// Hexagon reverse subtraction: `Rd=sub(#unsigned_small,Rs)`.
    ReverseSubtractImmediate { dst: GeneralRegister, src: GeneralRegister, value: u8 },
    /// Within a two-operation Hexagon packet, branch when producer's new
    /// register value is nonzero; the .new source is implicit packet state.
    BranchOnNewValueNonzero { displacement_bytes: u16 },
    /// `P0=cmp.eq(Rsrc,#0); if (!P0.new) jump:t PC+rel`, a single compound
    /// 32-bit instruction. Narrow encoder validated against LLVM6 backbranches.
    LoopOnNotP0New { src: GeneralRegister, displacement_bytes: i16 },
    /// `Rd=add(Rs,#unsigned_small)` with checked operand/immediate widths.
    AddSmallImmediate { dst: GeneralRegister, src: GeneralRegister, value: u8 },
    /// `p0=cmp.eq(Rsrc,#0)`; P0 is the only modeled predicate register.
    PredicateP0EqualsZero { src: GeneralRegister },
    /// Conditional new-predicate branch predicted not taken, relative to
    /// its packet's first instruction. Only a verified forward range works.
    JumpIfNewP0NotTaken { displacement_bytes: u32 },
    /// Hexagon register-width logical shift right with constant amount.
    LogicalShiftRight { dst: GeneralRegister, src: GeneralRegister, amount: u8 },
    /// Hexagon insert register bit-field. Width + offset must fit 32 bits.
    InsertBitField { dst: GeneralRegister, src: GeneralRegister, width: u8, offset: u8 },
    /// Hexagon 12-bit unsigned immediate assignment without an immext word.
    AssignUnsignedImmediate { dst: GeneralRegister, value: u16 },
    /// `R(odd):R(even)=combine(#0,#0)`; even low register required.
    CombineZeroPair { low: GeneralRegister },
    /// `tlbw(R(odd):R(even),R0)`, a V66 privileged TLB write.
    /// Rizin V6 corroborates register-pair field positions; LLVM6 cannot
    /// assemble this privileged form and is not presented as a second encoder.
    TlbWriteIndexedR0 { low: GeneralRegister },
    /// Hexagon `Rd = setbit(Rs,#index)`, 0 <= index < 32.
    SetBit { dst: GeneralRegister, src: GeneralRegister, position: u8 },
    /// Hexagon `Rd = clrbit(Rs,#index)`.
    ClearBit { dst: GeneralRegister, src: GeneralRegister, position: u8 },
    /// Hexagon `Rd = or(Rs,#imm)` with the confirmed nonnegative immediate field.
    OrSmallImmediate { dst: GeneralRegister, src: GeneralRegister, value: u8 },
    /// Hexagon `Rd = add(Rs,Rt)`, with register fields confirmed using LLVM.
    AddRegisters { dst: GeneralRegister, left: GeneralRegister, right: GeneralRegister },
    /// Hexagon `Rd = sub(Rs,Rt)`; fields confirmed with distinct operands.
    SubtractRegisters { dst: GeneralRegister, left: GeneralRegister, right: GeneralRegister },
    /// Hexagon `jumpr Rn` (not a return/stack ABI assumption).
    JumpRegister { source: GeneralRegister },
    /// Exact documented duplex `R0=#0; R3:2=combine(#0,#0)` group.
    /// LLVM6 encodes it as a single 32-bit duplex word; no generic support
    /// for duplex operand field layouts is implied.
    DuplexClearR0AndPair32,
    /// `P0=cmp.eq(Rleft,Rright)` for register sources, no immediate.
    PredicateP0EqualsRegisters { left: GeneralRegister, right: GeneralRegister },
    /// `if (P0.new) jumpr:nt LR` in the same packet as its P0 producer.
    ReturnIfNewP0NotTaken,
    /// Direct backward jump counted in bytes from the start of its packet.
    /// Only short negative packet-aligned Hexagon displacements are supported.
    JumpBackward { displacement_bytes: i16 },
    /// `Rd=asl(Rs,#N)` (arithmetic shift left by a 5-bit constant).
    ArithmeticShiftLeft { dst: GeneralRegister, src: GeneralRegister, amount: u8 },
    /// Compound Hexagon packet `P0=cmp.eq(Rsrc,#0); if(P0.new) jump:t`.
    /// This is one word with a short forward branch counted in bytes.
    SkipIfSourceZero { src: GeneralRegister, displacement_bytes: u16 },
    /// Hexagon GP-relative 32-bit load uses a separate checked extender.
    /// Compound zero/new branch predicted not taken, forward only.
    SkipIfSourceZeroNotTaken { src: GeneralRegister, displacement_bytes: u16 },
    /// Predicate P0 compare `R2 > #0x20`; the exact non-extender form is
    /// confirmed by independent Rizin V66 decoding. Other sources remain
    /// unsupported until independently assembled.
    PredicateP0R2GreaterThan32,
    /// `if (!P0.new) jump:nt +disp`, predicted not taken.
    JumpIfNotNewP0NotTaken { displacement_bytes: u16 },
    /// L2 prefetch with a general-register control descriptor.
    L2Fetch { address: GeneralRegister, control: GeneralRegister },
    /// Clear an entire cache line addressed by the given register.
    CacheZeroLine { address: GeneralRegister },
    /// Select a hardware-loop counter and target within this routine.
    HardwareLoop { loop_index: u8, counter: GeneralRegister, displacement_bytes: u16 },
    /// Store a 64-bit register pair then increment the pointer by 16 bytes.
    MemDoubleStorePostIncrement { low: GeneralRegister, base: GeneralRegister, increment: u8 },
    /// One Hexagon singleton NOP. Its packet marker can also denote endloop.
    Nop,
    /// Cache and instruction synchronization (singleton packets).
    Dckill,
    Ickill,
    Isync,
}

/// V66 system register IDs *as used by this exact instruction family*.
/// Unsupported special registers must not be guessed from their numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemRegister {
    Ssr,
    Syscfg,
}
impl SystemRegister {
    fn opcode_selector(self) -> u32 {
        match self { Self::Ssr => 6, Self::Syscfg => 18 }
    }
}

impl ScalarPacket {
    pub fn encode(self) -> Result<u32, String> {
        match self {
            Self::JumpRelative { displacement_bytes } => {
                if displacement_bytes > 0x1ffe || displacement_bytes & 1 != 0 {
                    return Err(format!(
                        "Hexagon jump displacement outside confirmed encoding: {displacement_bytes:#x}"
                    ));
                }
                // 0x5800 selects Hexagon direct jump; bits 15:14 == 3 mark
                // the last instruction of a singleton packet, not a gap.
                Ok(0x5800_c000 | (displacement_bytes / 2))
            }
            Self::AssignSmallImmediate { dst, value } => {
                if value > 127 {
                    return Err("Hexagon small immediate must be <= 127".into());
                }
                Ok(0x7800_c000 | (u32::from(value) << 5) | u32::from(dst.number()))
            }
            Self::WriteSystem { system, src } => {
                // `SSR=R0` and `SYSCFG=R0` use instruction family 0x67.
                // Source general-register number occupies bits 20:16;
                // destination system-register selector occupies bits 4:0.
                Ok(0x6700_c000 | (u32::from(src.number()) << 16)
                    | system.opcode_selector())
            }
            Self::ReadSyscfg { dst } => {
                // The observed SYSCFG selector is 0x12 in bits 20:16,
                // destination register in bits 4:0. Rizin V66 confirms it.
                Ok(0x6e80_c000 | (SystemRegister::Syscfg.opcode_selector() << 16)
                    | u32::from(dst.number()))
            }
            Self::AddExtendedImmediate { .. } => Err("Hexagon extended ADD must be emitted as a two-word instruction via encode_extended_add".into()),
            Self::LoadDoublePostIncrement { low, base, increment } => {
                if low.number()%2!=0 || low.number()>30 || increment%8!=0 || increment>248 {
                    return Err("Hexagon MEMD postincrement requires an even pair and 8-byte stride".into());
                }
                Ok(0x9bc0_c000 | (u32::from(base.number()) << 16)
                   | (u32::from(increment) << 2) | u32::from(low.number()))
            }
            Self::ExtractUnsigned { dst, src, width, offset } => {
                if width==0 || width>31 || offset>=32 || u16::from(width)+u16::from(offset)>32 {
                    return Err("Hexagon EXTRACTU fields exceed 32 bits".into());
                }
                Ok(0x8d00_c000 | (u32::from(offset&16)<<18)
                   | (u32::from(src.number()) << 16)
                   | (u32::from(width) << 8) | (u32::from(offset&15) << 5)
                   | u32::from(dst.number()))
            }
            Self::ReverseSubtractImmediate { dst, src, value } => {
                if value>127 {return Err("Hexagon reverse SUB immediate >127".into());}
                Ok(0x7640_c000 | (u32::from(src.number()) << 16)
                    | (u32::from(value) << 5) | u32::from(dst.number()))
            }
            Self::BranchOnNewValueNonzero { displacement_bytes } => {
                if displacement_bytes==0 || displacement_bytes>0x1fe || displacement_bytes&1!=0 {
                    return Err("Hexagon NEW comparison branch outside narrow tested range".into());
                }
                Ok(0x2442_e000 | (u32::from(displacement_bytes) >> 1))
            }
            Self::LoopOnNotP0New { src, displacement_bytes } => {
                if src.number()>7 || displacement_bytes>=0 || displacement_bytes < -256 || displacement_bytes&1!=0 {
                    return Err("Hexagon compound P0 loop requires low src register and backward halfword displacement".into());
                }
                let half=i32::from(displacement_bytes)/2;
                Ok(0x1070_6000 | (u32::from(src.number()) << 16)
                    | ((half as u32)&0xff))
            }
            Self::AddSmallImmediate { dst, src, value } => {
                if value>127 {return Err("Hexagon add immediate >127".into());}
                Ok(0xb000_c000 | (u32::from(src.number()) << 16)
                    | (u32::from(value) << 5) | u32::from(dst.number()))
            }
            Self::PredicateP0EqualsZero { src } => {
                Ok(0x7500_c000 | (u32::from(src.number()) << 16))
            }
            Self::JumpIfNewP0NotTaken { displacement_bytes } => {
                if displacement_bytes==0 || displacement_bytes>0x7fe || displacement_bytes&1!=0 {
                    return Err("Hexagon P0.new branch outside verified forward range".into());
                }
                Ok(0x5c00_c800 | (displacement_bytes >> 1))
            }
            Self::LogicalShiftRight { dst, src, amount } => {
                if amount>=32 { return Err("Hexagon LSR immediate amount >=32".into()); }
                Ok(0x8c00_c020 | (u32::from(src.number()) << 16)
                    | (u32::from(amount) << 8) | u32::from(dst.number()))
            }
            Self::InsertBitField { dst, src, width, offset } => {
                if width==0 || width>31 || offset>=32 || u16::from(width)+u16::from(offset)>32 {
                    return Err("Hexagon INSERT bitfield outside 32-bit register".into());
                }
                Ok(0x8f00_c000 | (u32::from(offset & 16) << 18)
                    | (u32::from(src.number()) << 16)
                    | (u32::from(width) << 8)
                    | (u32::from(offset & 15) << 5) | u32::from(dst.number()))
            }
            Self::AssignUnsignedImmediate { dst, value } => {
                if value>4095 { return Err("Hexagon positive 12-bit immediate >4095".into()); }
                Ok(0x7800_c000 | (u32::from(value >> 9) << 16)
                    | (u32::from(value & 0x1ff) << 5) | u32::from(dst.number()))
            }
            Self::CombineZeroPair { low } => {
                if low.number()%2!=0 || low.number()>30 {
                    return Err("Hexagon register pair requires even low register".into());
                }
                Ok(0x7c00_c000 | u32::from(low.number()))
            }
            Self::TlbWriteIndexedR0 { low } => {
                if low.number()%2!=0 || low.number()>30 {
                    return Err("Hexagon TLB register pair requires even low register".into());
                }
                Ok(0x6c00_c000 | (u32::from(low.number()) << 16))
            }
            Self::SetBit { dst, src, position } => {
                if position >= 32 {
                    return Err(format!("Hexagon setbit position out of range: {position}"));
                }
                // Confirmed against LLVM 6's independent Hexagon assembler
                // with distinct source/destination registers and bit indices.
                Ok(0x8cc0_c000 | (u32::from(src.number()) << 16)
                    | (u32::from(position) << 8) | u32::from(dst.number()))
            }
            Self::ClearBit { dst, src, position } => {
                if position >= 32 { return Err("Hexagon clrbit position >= 32".into()); }
                Ok(0x8cc0_c020 | (u32::from(src.number()) << 16)
                    | (u32::from(position) << 8) | u32::from(dst.number()))
            }
            Self::OrSmallImmediate { dst, src, value } => {
                if value > 127 { return Err("Hexagon OR immediate > 127".into()); }
                Ok(0x7680_c000 | (u32::from(src.number()) << 16)
                    | (u32::from(value) << 5) | u32::from(dst.number()))
            }
            Self::AddRegisters { dst, left, right } => {
                // ADD and SUB use different operand-field orderings in
                // these Hexagon instruction families. LLVM cross-check:
                // r2=add(r4,r5) -> f304c502.
                Ok(0xf300_c000 | (u32::from(left.number()) << 16)
                    | (u32::from(right.number()) << 8) | u32::from(dst.number()))
            }
            Self::SubtractRegisters { dst, left, right } => {
                // Hexagon subtraction register fields are **not** laid out
                // like the ARM32 data-processing fields. Independent LLVM
                // samples include r31 = sub(r31,r25), r1 = sub(r2,r3).
                Ok(0xf320_c000 | (u32::from(right.number()) << 16)
                    | (u32::from(left.number()) << 8) | u32::from(dst.number()))
            }
            Self::JumpRegister { source } =>
                Ok(0x5280_c000 | (u32::from(source.number()) << 16)),
            Self::DuplexClearR0AndPair32 => Ok(0x2800_3c01),
            Self::PredicateP0EqualsRegisters { left, right } => {
                // Independently LLVM-assembled P0 compares: R0,R5 and R3,R7.
                Ok(0xf200_c000 | (u32::from(left.number()) << 16)
                   | (u32::from(right.number()) << 8))
            }
            Self::ReturnIfNewP0NotTaken => Ok(0x535f_c800),
            Self::JumpBackward { displacement_bytes } => {
                if displacement_bytes >= 0 || displacement_bytes < -0x1000 ||
                   displacement_bytes % 4 != 0 {
                    return Err("Hexagon backward jump must be aligned, negative and within -0x1000 bytes".into());
                }
                let halves = (i32::from(displacement_bytes) / 2) as u32;
                Ok(0x59ff_c000 | (halves & 0x3fff))
            }
            Self::ArithmeticShiftLeft { dst, src, amount } => {
                if amount >= 32 { return Err("Hexagon ASL immediate shift must be < 32".into()); }
                Ok(0x8c00_c040 | (u32::from(src.number()) << 16)
                   | (u32::from(amount) << 8) | u32::from(dst.number()))
            }
            Self::SkipIfSourceZero { src, displacement_bytes } => {
                if src.number()>7 || displacement_bytes==0 ||
                   displacement_bytes>0x1fe || displacement_bytes % 2 != 0 {
                    return Err("CDSP compound zero compare needs low-register and short forward branch".into());
                }
                Ok(0x1000_e000 | (u32::from(src.number()) << 16)
                   | u32::from(displacement_bytes/2))
            }
            Self::SkipIfSourceZeroNotTaken { src, displacement_bytes } => {
                if src.number()>7 || displacement_bytes==0 ||
                   displacement_bytes>0x1fe || displacement_bytes%2!=0 {
                    return Err("Hexagon P0.new :nt branch needs low register and short aligned displacement".into());
                }
                Ok(0x1000_c000 | (u32::from(src.number())<<16)
                   | u32::from(displacement_bytes/2))
            }
            Self::PredicateP0R2GreaterThan32 => Ok(0x7542_c400),
            Self::JumpIfNotNewP0NotTaken { displacement_bytes } => {
                if displacement_bytes==0 || displacement_bytes>0x1fe || displacement_bytes%2!=0 {
                    return Err("Hexagon negative-P0.new :nt branch outside tested range".into());
                }
                Ok(0x5c20_c800 | u32::from(displacement_bytes/2))
            }
            Self::L2Fetch { address, control } => {
                Ok(0xa600_c000 | (u32::from(address.number()) << 16)
                    | (u32::from(control.number()) << 8))
            }
            Self::CacheZeroLine { address } => {
                Ok(0xa0c0_c000 | (u32::from(address.number()) << 16))
            }
            Self::HardwareLoop { loop_index, counter, displacement_bytes } => {
                if loop_index > 1 || displacement_bytes == 0 || displacement_bytes > 0x1ff8 || displacement_bytes%4!=0 {
                    return Err("Hexagon hardware loop supports loop0/1, positive aligned short displacements".into());
                }
                Ok(0x6000_c000 | (u32::from(loop_index) << 21)
                    | (u32::from(counter.number()) << 16)
                    | (u32::from(displacement_bytes) << 1))
            }
            Self::MemDoubleStorePostIncrement { low, base, increment } => {
                if low.number()%2!=0 || low.number()>30 || increment!=16 {
                    return Err("Hexagon MEMD postincrement store requires even pair and verified +16-byte stride".into());
                }
                Ok(0xabc0_c000 | (u32::from(base.number()) << 16)
                    | (u32::from(low.number()) << 8) | u32::from(increment))
            }
            Self::Nop => Ok(0x7f00_c000),
            Self::Dckill => Ok(0xa200_c000),
            Self::Ickill => Ok(0x56c0_d000),
            Self::Isync => Ok(0x57c0_c002),
        }
    }
}

/// Encode a Hexagon extended signed 32-bit assignment as a TWO-WORD bundle.
/// `end_packet` describes the last word's parse marker. In the supervisor
/// relocation trampoline, this assignment and the next add instruction must
/// occur within the same Hexagon instruction packet, so end_packet=false.
/// The immediate fields were confirmed using LLVM 6 for multiple operands
/// and both positive and negative extended values. This is not a raw opcode
/// copy from the vendor firmware.
pub fn encode_extended_assignment(dst: GeneralRegister, value: i32,
                                  end_packet: bool) -> [u32; 2] {
    let bits=value as u32;
    // immext carries the high twelve and middle fourteen immediate bits;
    // each instruction includes its own packet parse marker.
    let extender=((bits >> 20) << 16) | ((bits >> 6) & 0x3fff) | 0x4000;
    let mut assignment=0x7800_c000 | ((bits & 0x3f) << 5) | u32::from(dst.number());
    if !end_packet { assignment &= !0x8000; }
    [extender,assignment]
}

/// Construct a two-word extended ADD-immediate, including a shared Hexagon
/// immext prefix. Caller selects whether the operand body terminates the
/// packet. Both positive/negative 32-bit immediate split is independently
/// confirmed by LLVM6.
pub fn encode_extended_add(dst: GeneralRegister, src: GeneralRegister,
                           value: i32, end_packet: bool) -> [u32;2] {
    let bits=value as u32;
    let ext=((bits>>20)<<16) | ((bits>>6)&0x3fff) | 0x4000;
    let body=0xb000_c000 | (u32::from(src.number())<<16)
        | ((bits&0x3f)<<5) |u32::from(dst.number());
    [ext, if end_packet {body} else {body&!0x8000}]
}

/// Two-word V66 `Rd=memw(GP+##unsigned32)` in its extended-offset
/// addressing form. Hexagon immext stores bits 31:6; the consumer stores
/// bits 5:0 in the lower immediate field. This form is corroborated by V66
/// Rizin and by LLVM's independent GP load encoding (which sometimes emits
/// a shorter GP-relative alternative for the same absolute address).
pub fn encode_extended_gp_word_load(dst: GeneralRegister, absolute: u32) -> [u32; 2] {
    let ext=((absolute>>20)<<16) | ((absolute>>6)&0x3fff) | 0x4000;
    let load=0x4980_c000 | ((absolute&0x3f)<<5) | u32::from(dst.number());
    [ext,load]
}

/// Preserve the parse bits chosen for a multi-instruction Hexagon packet.
/// ScalarPacket::encode() yields a standalone final instruction (0xc000);
/// packet interior words can carry 0x4000 or 0x0000. This helper rejects
/// unsupported parse-bit patterns rather than fabricating unrelated words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketMarker { ContinuationA, ContinuationB, LoopEnd, Final }
impl PacketMarker {
    pub fn apply(self, standalone: u32) -> u32 {
        let marker = match self {
            Self::ContinuationA => 0x4000,
            Self::ContinuationB => 0x0000,
            Self::LoopEnd => 0x8000,
            Self::Final => 0xc000,
        };
        (standalone & !0xc000) | marker
    }
}

/// The reference opens with a selector branch, six guarded self-branches,
/// a relocation-free jump into the entry dispatch, then initializes `r0`.
/// The target at runtime physical address `0x96c00cac` remains outside this
/// generated fragment and must be reconstructed separately.
fn entry_packets() -> Result<[ScalarPacket; 9], String> {
    use ScalarPacket::*;
    let r0 = GeneralRegister::new(0)?;
    Ok([
        JumpRelative { displacement_bytes: 0x1c },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0xc90 },
        AssignSmallImmediate { dst: r0, value: 0 },
    ])
}

pub fn build_entry() -> Result<Vec<u8>, String> {
    let mut output = Vec::with_capacity(REBUILT_LEN);
    for packet in entry_packets()? {
        output.extend_from_slice(&packet.encode()?.to_le_bytes());
    }
    if output.len() != REBUILT_LEN {
        return Err("CDSP entry emitter produced an incorrect length".into());
    }
    Ok(output)
}

/// Extend the verified nine-packet entry with eight system-init packets.
/// These privilege-state and cache-control operations were independently
/// *decoded* with the Rizin Hexagon V6 backend; LLVM 6 cannot assemble the
/// special-register forms, so the emitted bytes are not claimed to have a
/// complete second-toolchain assembler round-trip. Nothing after 0x3044
/// (including the reference's invalid/reserved 0x3044 word) is filled in.
pub fn build_init_prefix() -> Result<Vec<u8>, String> {
    use ScalarPacket::*;
    let r0 = GeneralRegister::new(0)?;
    let mut output = build_entry()?;
    let privileged = [
        WriteSystem { system: SystemRegister::Ssr, src: r0 },
        ReadSyscfg { dst: r0 },
        SetBit { dst: r0, src: r0, position: 3 },
        SetBit { dst: r0, src: r0, position: 5 },
        WriteSystem { system: SystemRegister::Syscfg, src: r0 },
        Dckill,
        Ickill,
        JumpRelative { displacement_bytes: 0x104 },
    ];
    for packet in privileged {
        output.extend_from_slice(&packet.encode()?.to_le_bytes());
    }
    if output.len() != INIT_PREFIX_LEN {
        return Err("CDSP initialization emitter produced an incorrect length".into());
    }
    Ok(output)
}

pub fn verify_init_prefix(image: &[u8]) -> Result<(), String> {
    validate_reference(image)?;
    let generated = build_init_prefix()?;
    let expected = &image[START_FILE_OFFSET..START_FILE_OFFSET + INIT_PREFIX_LEN];
    if let Some((i, (original, built))) = expected.iter().zip(&generated)
        .enumerate().find(|(_, (a, b))| a != b)
    {
        return Err(format!("CDSP initialization differs at file {:#x}: expected {original:02x}, built {built:02x}",
            START_FILE_OFFSET + i));
    }
    Ok(())
}

/// Check that the user's private reference is the expected CDSP ELF32 image,
/// without confusing ELF vaddr, physical load address, and file offset.
/// This intentionally doesn't purport to authenticate the vendor image.
pub fn validate_reference(image: &[u8]) -> Result<(), String> {
    let elf = parse_elf32(image)?;
    if elf.machine != HEXAGON_ELF_MACHINE {
        return Err(format!("expected Hexagon ELF machine 164, found {}", elf.machine));
    }
    if elf.entry != ENTRY_PHYS {
        return Err(format!("unexpected CDSP physical entrypoint: {:#x}", elf.entry));
    }
    if image.get(36..40) != Some(&CDSP_ELF_FLAGS.to_le_bytes()) {
        return Err("unexpected CDSP ELF e_flags".into());
    }
    let segment = elf.program_headers.iter().find(|ph| {
        ph.kind == 1 && ph.offset as usize == START_FILE_OFFSET &&
            ph.file_size == FIRST_EXECUTABLE_SIZE
    }).ok_or("expected CDSP first executable PT_LOAD not found")?;
    if segment.vaddr != ENTRY_ELF_VADDR || segment.paddr != ENTRY_PHYS ||
        segment.flags & 1 == 0 || segment.file_size > segment.mem_size
    {
        return Err("CDSP first PT_LOAD mapping or permissions differ".into());
    }
    let end = START_FILE_OFFSET.checked_add(REBUILT_LEN)
        .ok_or("CDSP entry offset overflow")?;
    if image.get(START_FILE_OFFSET..end).is_none() {
        return Err("truncated CDSP entry instructions".into());
    }
    Ok(())
}

pub fn verify_reference(image: &[u8]) -> Result<(), String> {
    validate_reference(image)?;
    let output = build_entry()?;
    let reference = &image[START_FILE_OFFSET..START_FILE_OFFSET + REBUILT_LEN];
    if let Some((position, (expected, found))) = reference.iter().zip(&output)
        .enumerate().find(|(_, (expected, found))| expected != found)
    {
        return Err(format!(
            "CDSP entry differs at file offset {:#x}: reference {expected:02x} generated {found:02x}",
            START_FILE_OFFSET + position
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encodes_direct_jumps_and_checks_bounds() {
        assert_eq!(ScalarPacket::JumpRelative { displacement_bytes: 0 }.encode().unwrap(), 0x5800_c000);
        assert_eq!(ScalarPacket::JumpRelative { displacement_bytes: 0x1c }.encode().unwrap(), 0x5800_c00e);
        assert_eq!(ScalarPacket::JumpRelative { displacement_bytes: 0xc90 }.encode().unwrap(), 0x5800_c648);
        assert!(ScalarPacket::JumpRelative { displacement_bytes: 1 }.encode().is_err());
        assert!(ScalarPacket::JumpRelative { displacement_bytes: 0x2000 }.encode().is_err());
    }

    #[test]
    fn assigns_small_immediates_to_checked_registers() {
        assert!(GeneralRegister::new(32).is_err());
        let r0 = GeneralRegister::new(0).unwrap();
        let r3 = GeneralRegister::new(3).unwrap();
        assert_eq!(ScalarPacket::AssignSmallImmediate { dst: r0, value: 0 }.encode().unwrap(), 0x7800_c000);
        assert_eq!(ScalarPacket::AssignSmallImmediate { dst: r3, value: 12 }.encode().unwrap(), 0x7800_c183);
        assert!(ScalarPacket::AssignSmallImmediate { dst: r3, value: 128 }.encode().is_err());
    }

    #[test]
    fn entry_has_nine_singleton_packets_not_filler() {
        let b = build_entry().unwrap();
        assert_eq!(b.len(), REBUILT_LEN);
        for word in b.chunks_exact(4) {
            assert_eq!(u32::from_le_bytes(word.try_into().unwrap()) & 0xc000, 0xc000);
        }
        assert_eq!(entry_packets().unwrap().len(), 9);
    }
    #[test]
    fn privileged_system_operations_use_checked_hexagon_fields() {
        use ScalarPacket::*;
        let r0 = GeneralRegister::new(0).unwrap();
        let r15 = GeneralRegister::new(15).unwrap();
        assert_eq!(WriteSystem { system: SystemRegister::Ssr, src: r0 }.encode().unwrap(), 0x6700_c006);
        assert_eq!(WriteSystem { system: SystemRegister::Syscfg, src: r0 }.encode().unwrap(), 0x6700_c012);
        assert_eq!(WriteSystem { system: SystemRegister::Syscfg, src: r15 }.encode().unwrap(), 0x670f_c012);
        assert_eq!(ReadSyscfg { dst: r0 }.encode().unwrap(), 0x6e92_c000);
        assert_eq!(Dckill.encode().unwrap(), 0xa200_c000);
        assert_eq!(Ickill.encode().unwrap(), 0x56c0_d000);
    }

    #[test]
    fn bit_instruction_is_encoded_independently_of_registers() {
        use ScalarPacket::SetBit;
        let r0 = GeneralRegister::new(0).unwrap();
        let r1 = GeneralRegister::new(1).unwrap();
        let r5 = GeneralRegister::new(5).unwrap();
        let r7 = GeneralRegister::new(7).unwrap();
        assert_eq!(SetBit { dst: r0, src: r0, position: 3 }.encode().unwrap(), 0x8cc0_c300);
        assert_eq!(SetBit { dst: r5, src: r7, position: 2 }.encode().unwrap(), 0x8cc7_c205);
        assert_eq!(SetBit { dst: r1, src: r1, position: 3 }.encode().unwrap(), 0x8cc1_c301);
        assert!(SetBit { dst: r0, src: r0, position: 32 }.encode().is_err());
    }

    #[test]
    fn hexagon_conditional_bit_and_arithmetic_fields_match_independent_llvm() {
        use ScalarPacket::*;
        let reg = |n| GeneralRegister::new(n).unwrap();
        assert_eq!(ClearBit { dst: reg(15), src: reg(15), position: 1 }.encode().unwrap(), 0x8ccf_c12f);
        assert_eq!(OrSmallImmediate { dst: reg(15), src: reg(15), value: 3 }.encode().unwrap(), 0x768f_c06f);
        assert_eq!(OrSmallImmediate { dst: reg(1), src: reg(2), value: 3 }.encode().unwrap(), 0x7682_c061);
        assert_eq!(SubtractRegisters { dst: reg(31), left: reg(31), right: reg(25) }.encode().unwrap(), 0xf339_df1f);
        assert_eq!(SubtractRegisters { dst: reg(1), left: reg(2), right: reg(3) }.encode().unwrap(), 0xf323_c201);
        assert_eq!(JumpRegister { source: reg(31) }.encode().unwrap(), 0x529f_c000);
        assert_eq!(Isync.encode().unwrap(), 0x57c0_c002);
        assert!(ClearBit { dst: reg(15), src: reg(15), position: 32 }.encode().is_err());
        assert!(OrSmallImmediate { dst: reg(15), src: reg(15), value: 128 }.encode().is_err());
    }

    #[test]
    fn privileged_initialization_has_exact_packet_coverage() {
        let b = build_init_prefix().unwrap();
        assert_eq!(b.len(), INIT_PREFIX_LEN);
        assert_eq!(&b[..REBUILT_LEN], build_entry().unwrap().as_slice());
        for word in b.chunks_exact(4) {
            assert_eq!(u32::from_le_bytes(word.try_into().unwrap()) & 0xc000, 0xc000);
        }
        assert_eq!(u32::from_le_bytes(b[64..68].try_into().unwrap()), 0x5800_c082);
    }

    #[test]
    fn extended_signed_immediates_and_addition_match_independent_llvm() {
        let r=|n| GeneralRegister::new(n).unwrap();
        assert_eq!(encode_extended_assignment(r(30), -0x1fff280, false),
                   [0x0fe0_4036, 0x7800_401e]);
        assert_eq!(encode_extended_assignment(r(30), -0x1fff280, true),
                   [0x0fe0_4036, 0x7800_c01e]);
        assert_eq!(encode_extended_assignment(r(30), 0x1234_5678, true),
                   [0x0123_5159, 0x7800_c71e]);
        assert_eq!(ScalarPacket::AddRegisters { dst: r(31), left:r(31), right:r(25) }.encode().unwrap(),
                   0xf31f_d91f);
        assert_eq!(ScalarPacket::AddRegisters { dst: r(2), left:r(4), right:r(5) }.encode().unwrap(),
                   0xf304_c502);
    }

    #[test]
    fn hexagon_tlb_instruction_fields_and_parse_markers() {
        use ScalarPacket::*;
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(LogicalShiftRight{dst:r(2),src:r(24),amount:11}.encode().unwrap(),0x8c18_cb22);
        assert_eq!(InsertBitField{dst:r(2),src:r(4),width:11,offset:21}.encode().unwrap(),0x8f44_cba2);
        assert_eq!(InsertBitField{dst:r(3),src:r(5),width:12,offset:20}.encode().unwrap(),0x8f45_cc83);
        assert_eq!(AssignUnsignedImmediate{dst:r(4),value:0x7c0}.encode().unwrap(),0x7803_f804);
        assert_eq!(AssignUnsignedImmediate{dst:r(5),value:0xc00}.encode().unwrap(),0x7806_c005);
        assert_eq!(CombineZeroPair{low:r(2)}.encode().unwrap(),0x7c00_c002);
        assert_eq!(TlbWriteIndexedR0{low:r(2)}.encode().unwrap(),0x6c02_c000);
        assert_eq!(TlbWriteIndexedR0{low:r(6)}.encode().unwrap(),0x6c06_c000);
        assert_eq!(PacketMarker::ContinuationA.apply(0x8c18_cb22),0x8c18_4b22);
        assert_eq!(PacketMarker::ContinuationB.apply(0x7803_f804),0x7803_3804);
        assert!(LogicalShiftRight{dst:r(2),src:r(24),amount:32}.encode().is_err());
        assert!(InsertBitField{dst:r(2),src:r(4),width:13,offset:21}.encode().is_err());
        assert!(CombineZeroPair{low:r(3)}.encode().is_err());
        assert!(TlbWriteIndexedR0{low:r(31)}.encode().is_err());
        assert!(AssignUnsignedImmediate{dst:r(5),value:4096}.encode().is_err());
    }

    #[test]
    fn predicate_p0_branch_uses_checked_rel_word_fields() {
        use ScalarPacket::*;
        let r25=GeneralRegister::new(25).unwrap();
        let predicate=PredicateP0EqualsZero{src:r25}.encode().unwrap();
        let branch=JumpIfNewP0NotTaken{displacement_bytes:0x30}.encode().unwrap();
        assert_eq!(predicate,0x7519_c000);
        assert_eq!(PacketMarker::ContinuationA.apply(predicate),0x7519_4000);
        assert_eq!(branch,0x5c00_c818);
        assert!(JumpIfNewP0NotTaken{displacement_bytes:0}.encode().is_err());
        assert!(JumpIfNewP0NotTaken{displacement_bytes:31}.encode().is_err());
        assert!(JumpIfNewP0NotTaken{displacement_bytes:0x800}.encode().is_err());
    }

    #[test]
    fn cdsp_tlb_walker_packet_fields_match_independent_hexagon_llvm() {
        use ScalarPacket::*;
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(encode_extended_add(r(1),r(25),-0x1fdb700,true),
                   [0x0fe0_4924,0xb019_c001]);
        assert_eq!(LoadDoublePostIncrement{low:r(2),base:r(1),increment:8}.encode().unwrap(),
                   0x9bc1_c022);
        assert_eq!(ExtractUnsigned{dst:r(4),src:r(3),width:7,offset:20}.encode().unwrap(),
                   0x8d43_c784);
        assert_eq!(ReverseSubtractImmediate{dst:r(4),src:r(4),value:125}.encode().unwrap(),
                   0x7644_cfa4);
        assert_eq!(PacketMarker::ContinuationA.apply(ReverseSubtractImmediate{
            dst:r(4),src:r(4),value:125}.encode().unwrap()),0x7644_4fa4);
        assert_eq!(BranchOnNewValueNonzero{displacement_bytes:12}.encode().unwrap(),0x2442_e006);
        assert_eq!(LoopOnNotP0New{src:r(2),displacement_bytes:-24}.encode().unwrap(),0x1072_60f4);
        assert_eq!(AddSmallImmediate{dst:r(0),src:r(0),value:1}.encode().unwrap(),0xb000_c020);
        assert!(LoadDoublePostIncrement{low:r(3),base:r(1),increment:8}.encode().is_err());
        assert!(ExtractUnsigned{dst:r(4),src:r(3),width:21,offset:20}.encode().is_err());
        assert!(ReverseSubtractImmediate{dst:r(4),src:r(4),value:128}.encode().is_err());
        assert!(LoopOnNotP0New{src:r(2),displacement_bytes:0}.encode().is_err());
    }

    #[test]
    fn hexagon_tlb_clear_loop_fields_are_checked_and_llvm_corroborated() {
        use ScalarPacket::*;
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(DuplexClearR0AndPair32.encode().unwrap(),0x2800_3c01);
        assert_eq!(PredicateP0EqualsRegisters{left:r(0),right:r(5)}.encode().unwrap(),
                   0xf200_c500);
        assert_eq!(PredicateP0EqualsRegisters{left:r(3),right:r(7)}.encode().unwrap(),
                   0xf203_c700);
        assert_eq!(ReturnIfNewP0NotTaken.encode().unwrap(),0x535f_c800);
        assert_eq!(JumpBackward{displacement_bytes:-12}.encode().unwrap(),0x59ff_fffa);
        assert_eq!(JumpBackward{displacement_bytes:-16}.encode().unwrap(),0x59ff_fff8);
        assert_eq!(ArithmeticShiftLeft{dst:r(2),src:r(0),amount:3}.encode().unwrap(),0x8c00_c342);
        assert!(JumpBackward{displacement_bytes:0}.encode().is_err());
        assert!(JumpBackward{displacement_bytes:4}.encode().is_err());
        assert!(JumpBackward{displacement_bytes:-5}.encode().is_err());
        assert!(JumpBackward{displacement_bytes:-0x1004}.encode().is_err());
        assert!(ArithmeticShiftLeft{dst:r(2),src:r(0),amount:32}.encode().is_err());
    }

    #[test]
    fn compound_forward_zero_branch_matches_llvm_v60() {
        use ScalarPacket::*;
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(SkipIfSourceZero{src:r(2),displacement_bytes:16}.encode().unwrap(),
                   0x1002_e008);
        assert_eq!(SkipIfSourceZero{src:r(3),displacement_bytes:12}.encode().unwrap(),
                   0x1003_e006);
        assert!(SkipIfSourceZero{src:r(8),displacement_bytes:16}.encode().is_err());
        assert!(SkipIfSourceZero{src:r(2),displacement_bytes:0}.encode().is_err());
        assert!(SkipIfSourceZero{src:r(2),displacement_bytes:15}.encode().is_err());
    }

    #[test]
    fn vectorized_cache_copy_entry_fields_are_restricted() {
        use ScalarPacket::*;
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(SkipIfSourceZeroNotTaken{src:r(2),displacement_bytes:104}.encode().unwrap(),0x1002_c034);
        assert_eq!(SkipIfSourceZeroNotTaken{src:r(0),displacement_bytes:84}.encode().unwrap(),0x1000_c02a);
        assert_eq!(PredicateP0R2GreaterThan32.encode().unwrap(),0x7542_c400);
        assert_eq!(JumpIfNotNewP0NotTaken{displacement_bytes:92}.encode().unwrap(),0x5c20_c82e);
        assert_eq!(encode_extended_gp_word_load(r(0),0xc067_3260),[0x0c06_5cc9,0x4980_c400]);
        assert_eq!(encode_extended_gp_word_load(r(3),0xc067_3220),[0x0c06_5cc8,0x4980_c403]);
        assert!(SkipIfSourceZeroNotTaken{src:r(8),displacement_bytes:104}.encode().is_err());
        assert!(JumpIfNotNewP0NotTaken{displacement_bytes:0}.encode().is_err());
        assert!(JumpIfNotNewP0NotTaken{displacement_bytes:17}.encode().is_err());
    }

    #[test]
    fn hexagon_cache_zero_and_doubleword_loop_encodings_are_checked() {
        use ScalarPacket::*;
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(L2Fetch{address:r(1),control:r(7)}.encode().unwrap(),0xa601_c700);
        assert_eq!(CacheZeroLine{address:r(1)}.encode().unwrap(),0xa0c1_c000);
        assert_eq!(HardwareLoop{loop_index:1,counter:r(6),displacement_bytes:12}.encode().unwrap(),0x6026_c018);
        assert_eq!(HardwareLoop{loop_index:0,counter:r(3),displacement_bytes:4}.encode().unwrap(),0x6003_c008);
        assert_eq!(MemDoubleStorePostIncrement{low:r(6),base:r(1),increment:16}.encode().unwrap(),0xabc1_c610);
        assert_eq!(MemDoubleStorePostIncrement{low:r(8),base:r(5),increment:16}.encode().unwrap(),0xabc5_c810);
        assert_eq!(PacketMarker::LoopEnd.apply(Nop.encode().unwrap()),0x7f00_8000);
        assert_eq!(PacketMarker::LoopEnd.apply(MemDoubleStorePostIncrement{
            low:r(6),base:r(1),increment:16}.encode().unwrap()),0xabc1_8610);
        assert!(HardwareLoop{loop_index:2,counter:r(3),displacement_bytes:4}.encode().is_err());
        assert!(HardwareLoop{loop_index:0,counter:r(3),displacement_bytes:1}.encode().is_err());
        assert!(MemDoubleStorePostIncrement{low:r(7),base:r(1),increment:16}.encode().is_err());
        assert!(MemDoubleStorePostIncrement{low:r(6),base:r(1),increment:8}.encode().is_err());
    }

    #[test]
    fn rejects_wrong_reference_header() {
        assert!(validate_reference(&vec![0; 36]).is_err());
    }
}
