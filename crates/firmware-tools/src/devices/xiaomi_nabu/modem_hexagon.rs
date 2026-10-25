// SPDX-License-Identifier: GPL-3.0-only
//! Small, checked Hexagon v6 instruction encoder for the modem's reset entry.
//!
//! The opcode layouts are from Qualcomm Hexagon V6 architecture patterns,
//! independently corroborated against Rizin's Hexagon ISA specification and
//! LLVM 6's Hexagon MC assembler. Only supported forms may be emitted.
//! Rizin identifies the system registers as S6 (SSR), S18 (SYSCFG).
//!
//! `Packet::single` marks the end of a single-instruction packet. This is
//! essential: Hexagon's parse bits are inside each 32-bit instruction word.
//! Unsupported operations never silently return NOP or copied OEM bytes.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SpecialReg {
    Ssr,
    Syscfg,
}
impl SpecialReg {
    const fn number(self) -> u32 {
        match self {
            Self::Ssr => 6,
            Self::Syscfg => 18,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CacheCommand {
    DataKill,
    InstructionKill,
}

const PACKET_END: u32 = 0b11 << 14;

/// Readable representation of one 32-bit instruction in a single packet.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Operation {
    /// Packet-relative branch displacement in bytes.
    Jump { offset_bytes: i32 },
    ClearRegister { dst: u8 },
    /// General register to privileged system register.
    WriteSystem { dst: SpecialReg, src: u8 },
    /// Privileged system register to general register.
    ReadSystem { dst: u8, src: SpecialReg },
    SetBit { dst: u8, src: u8, index: u8 },
    Cache(CacheCommand),
    /// S2_clrbit_i: `Rd = clrbit(Rs,#bit)`.
    ClearBit {dst:u8,src:u8,index:u8},
    /// Y2_isync instruction barrier.
    InstructionSync,
    /// Y2_tlbw(Rss,Rt), with `pair_low` the even low register number.
    WriteTlb {pair_low:u8,index:u8},
    /// J2_jumpr Rs.
    JumpRegister {src:u8},
}

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
pub enum PacketMarker {
    Continue,
    End,
}
impl PacketMarker {
    const fn bits(self)->u32 { match self {Self::Continue=>1u32<<14,Self::End=>PACKET_END} }
}

fn general_register(reg: u8) -> Result<u32, String> {
    if reg > 31 {
        Err(format!("Hexagon general-purpose register r{reg} is out of range"))
    } else {
        Ok(reg as u32)
    }
}

/// Encode an architectural operation with packet-end markers. No opcode
/// literals from the reference firmware are embedded in this implementation.
pub fn encode_single(op: Operation) -> Result<u32, String> {
    encode(op,PacketMarker::End)
}

pub fn encode(op:Operation,marker:PacketMarker)->Result<u32,String> {
    let word = match op {
        Operation::Jump { offset_bytes } => {
            // J2_jump uses a signed packet-relative displacement, represented
            // as 9 high bits at [24:16] and 13 low bits at [13:1]. The
            // destination and packet addresses are both four-byte aligned.
            if offset_bytes % 4 != 0 || !(-(1 << 22)..(1 << 22)).contains(&offset_bytes) {
                return Err(format!("Hexagon jump displacement {offset_bytes} is unaligned or out of range"));
            }
            let raw = offset_bytes as u32;
            0x5800_0000 | (((raw >> 14) & 0x1ff) << 16) | ((raw >> 1) & 0x1fff)
        }
        Operation::ClearRegister { dst } => {
            // A2_tfrsi, with the immediate field identically zero.
            0x7800_0000 | general_register(dst)?
        }
        Operation::WriteSystem { dst, src } => {
            // Y2_tfrsrcr: 01100111000sssssPP0000000ddddddd.
            0x6700_0000 | (general_register(src)? << 16) | dst.number()
        }
        Operation::ReadSystem { dst, src } => {
            // Y2_tfrscrr: 011011101sssssssPP000000000ddddd.
            0x6e80_0000 | (src.number() << 16) | general_register(dst)?
        }
        Operation::SetBit { dst, src, index } => {
            // S2_setbit_i: 10001100110sssssPP0iiiii000ddddd.
            if index > 31 {
                return Err(format!("setbit index {index} exceeds 31"));
            }
            0x8cc0_0000 | (general_register(src)? << 16) | ((index as u32) << 8)
                | general_register(dst)?
        }
        Operation::Cache(cmd) => match cmd {
            CacheCommand::DataKill => 0xa200_0000, // Y2_dckill
            CacheCommand::InstructionKill => 0x56c0_1000, // Y2_ickill
        },
        Operation::ClearBit {dst,src,index} => {
            if index>31 {return Err("Hexagon clrbit index must be in 0..31".into());}
            // S2_clrbit_i: 10001100110sssssPP0iiiii001ddddd.
            0x8cc0_0020|(general_register(src)?<<16)|((index as u32)<<8)|general_register(dst)?
        },
        Operation::InstructionSync => 0x57c0_0002, // Y2_isync
        Operation::WriteTlb {pair_low,index} => {
            if pair_low>30 || pair_low%2!=0 {
                return Err("Hexagon TLB entry pair must be an even rN:rN+1 register".into());
            }
            // Y2_tlbw: 01101100000sssssPP0ttttt00000000
            0x6c00_0000|((pair_low as u32)<<16)|(general_register(index)?<<8)
        },
        Operation::JumpRegister {src} => {
            // J2_jumpr: 01010010100sssssPP00000000000000.
            0x5280_0000|(general_register(src)?<<16)
        },
    };
    Ok(word | marker.bits())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn branch_displacements_have_strict_bounds_and_alignment() {
        assert!(encode_single(Operation::Jump { offset_bytes: 0 }).is_ok());
        assert!(encode_single(Operation::Jump { offset_bytes: 0x1c }).is_ok());
        assert!(encode_single(Operation::Jump { offset_bytes: 3 }).is_err());
        assert!(encode_single(Operation::Jump { offset_bytes: 1 << 22 }).is_err());
        assert!(encode_single(Operation::Jump { offset_bytes: -(1 << 22) - 4 }).is_err());
    }
    #[test]
    fn bad_registers_and_bit_indices_are_rejected() {
        assert!(encode_single(Operation::ClearRegister { dst: 32 }).is_err());
        assert!(encode_single(Operation::ReadSystem { dst: 42, src: SpecialReg::Ssr }).is_err());
        assert!(encode_single(Operation::WriteSystem { dst: SpecialReg::Syscfg, src: 32 }).is_err());
        assert!(encode_single(Operation::SetBit { dst: 0, src: 0, index: 32 }).is_err());
    }
    #[test]
    fn packet_marker_is_always_present_for_single_packets() {
        for x in [
            Operation::Jump {offset_bytes: 0},
            Operation::ClearRegister {dst: 0},
            Operation::ReadSystem {dst: 0, src: SpecialReg::Syscfg},
            Operation::Cache(CacheCommand::DataKill),
        ] {
            assert_eq!(encode_single(x).unwrap() & (0b11 << 14), PACKET_END);
        }
    }
}
