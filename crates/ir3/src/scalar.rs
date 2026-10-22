// SPDX-License-Identifier: GPL-3.0-only
//! Typed subset of Adreno IR3 category-2 scalar ALU and category-6 local store.
//!
//! The encoding fields are described in Mesa Freedreno's MIT-licensed
//! `src/freedreno/isa/ir3-{common,cat2,cat6}.xml`. Only the operations and
//! operand forms represented here are supported. This is an encoder, not a
//! shader emulator or an assertion that a particular firmware is safe.

use crate::Gpr;

/// Source of a category-2 full-precision 32-bit arithmetic instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarSource {
    Register(Gpr),
    /// A signed 11-bit immediate encoded with the public `#multisrc-immed` tag.
    Immediate(i16),
}

fn register(gpr: Gpr) -> Result<u8, &'static str> {
    // Only ordinary GPRs are accepted here; predicate destinations are
    // distinguished explicitly in the instruction types below.
    if gpr.vector > 60 || gpr.component > 3 {
        return Err("IR3 scalar source/destination is not an ordinary GPR");
    }
    Ok(gpr.vector * 4 + gpr.component)
}

impl ScalarSource {
    fn bits(self) -> Result<u16, &'static str> {
        match self {
            Self::Register(gpr) => Ok(u16::from(register(gpr)?)),
            Self::Immediate(value) if (-1024..=1023).contains(&value) => {
                // `100` in bits 11..13 identifies a full-precision immediate.
                Ok((1 << 13) | ((value as u16) & 0x07ff))
            }
            Self::Immediate(_) => Err("IR3 category-2 signed immediate exceeds 11 bits"),
        }
    }
}

/// Publicly documented category-2 operations with full-precision u32 sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarAlu {
    MultiplyU24 {
        dst: Gpr,
        left: ScalarSource,
        right: ScalarSource,
        /// Synchronize before executing this instruction.
        sync: bool,
    },
    AddU32 {
        dst: Gpr,
        left: ScalarSource,
        right: ScalarSource,
        /// Scalar-ALU synchronization bit, printed `(ss)` by Mesa.
        scalar_sync: bool,
    },
    CompareU32Equal {
        /// Component of the special predicate destination register p0.
        predicate_component: u8,
        left: ScalarSource,
        right: ScalarSource,
    },
}

impl ScalarAlu {
    pub fn encode(self) -> Result<[u8; 8], &'static str> {
        let (opcode, destination, left, right, sync, scalar_sync, comparison) = match self {
            Self::MultiplyU24 { dst, left, right, sync } => {
                (0b110000u64, u64::from(register(dst)?), left, right, sync, false, false)
            }
            Self::AddU32 { dst, left, right, scalar_sync } => {
                (0b010000u64, u64::from(register(dst)?), left, right, false, scalar_sync, false)
            }
            Self::CompareU32Equal { predicate_component, left, right } => {
                if predicate_component > 3 {
                    return Err("IR3 predicate destination component must be 0..3");
                }
                // Predicate register p0 is GPR number 62 in category 2.
                (0b010100u64, u64::from(62 * 4 + predicate_component), left, right, false, false, true)
            }
        };
        let mut instruction = (2u64 << 61)
            | (opcode << 53)
            | (1u64 << 52) // full-precision 32-bit source operands
            | (destination << 32)
            | u64::from(left.bits()?)
            | (u64::from(right.bits()?) << 16)
            | ((sync as u64) << 60)
            | ((scalar_sync as u64) << 44);
        if comparison {
            instruction |= 4u64 << 48; // `eq` in the three-bit #cond enum
        }
        Ok(instruction.to_le_bytes())
    }
}

/// A local-memory store to `l[address]`, encoded using IR3 category 6's
/// A3xx/A6xx-compatible `stl` form. Both operands are ordinary GPRs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreLocalU32 {
    pub address: Gpr,
    pub source: Gpr,
    /// Offset in local memory, signed 13-bit field.
    pub offset: i16,
    /// Number of consecutive 32-bit scalar values; 1..=8.
    pub count: u8,
}

impl StoreLocalU32 {
    pub fn encode(self) -> Result<[u8; 8], &'static str> {
        if !(-4096..=4095).contains(&self.offset) {
            return Err("IR3 local-store offset exceeds signed 13 bits");
        }
        if !(1..=8).contains(&self.count) {
            return Err("IR3 local-store count must be 1..8");
        }
        let offset = (self.offset as u16) & 0x1fff;
        let instruction = (6u64 << 61) // category 6
            | (4u64 << 54) // local store opcode
            | (3u64 << 49) // u32 type
            | (u64::from(register(self.address)?) << 41)
            | (1u64 << 40) // A3xx local-store encoding marker
            | (u64::from(offset & 0xff) << 32)
            | (u64::from(self.count) << 24)
            | (1u64 << 23) // store addressing form
            | (u64::from(offset >> 8) << 9)
            | (u64::from(register(self.source)?) << 1);
        Ok(instruction.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn g(vector: u8, component: u8) -> Gpr { Gpr { vector, component } }
    fn bits(bytes: [u8; 8]) -> u64 { u64::from_le_bytes(bytes) }

    #[test]
    fn scalar_arithmetic_uses_public_categories_and_instruction_forms() {
        let multiply = bits(ScalarAlu::MultiplyU24 {
            dst: g(2, 1), left: ScalarSource::Register(g(2, 0)),
            right: ScalarSource::Immediate(96), sync: true,
        }.encode().unwrap());
        assert_eq!(multiply >> 61, 2);
        assert_eq!((multiply >> 53) & 0x3f, 0b110000);
        assert_eq!((multiply >> 60) & 1, 1);
        assert_eq!((multiply >> 32) & 0xff, 9);
        assert_eq!((multiply >> 16) & 0xffff, 0x2000 | 96);
        let compare = bits(ScalarAlu::CompareU32Equal {
            predicate_component: 2, left: ScalarSource::Register(g(1, 1)),
            right: ScalarSource::Register(g(1, 2)),
        }.encode().unwrap());
        assert_eq!((compare >> 32) & 0xff, 250);
        assert_eq!((compare >> 48) & 7, 4); // equal predicate
        let add = bits(ScalarAlu::AddU32 {
            dst: g(1, 0), left: ScalarSource::Register(g(1, 0)),
            right: ScalarSource::Immediate(1), scalar_sync: true,
        }.encode().unwrap());
        assert_eq!((add >> 44) & 1, 1);
    }

    #[test]
    fn local_store_has_type_address_data_and_count_fields() {
        let s = bits(StoreLocalU32 {
            address: g(3, 3), source: g(2, 0), offset: -2, count: 4,
        }.encode().unwrap());
        assert_eq!(s >> 61, 6);
        assert_eq!((s >> 54) & 0x1f, 4);
        assert_eq!((s >> 49) & 7, 3);
        assert_eq!((s >> 41) & 0xff, 15);
        assert_eq!((s >> 1) & 0xff, 8);
        assert_eq!((s >> 24) & 0xff, 4);
        assert_eq!(((s >> 9) & 0x1f) << 8 | ((s >> 32) & 0xff), 0x1ffe);
    }

    #[test]
    fn invalid_operands_are_rejected() {
        assert!(ScalarAlu::AddU32 { dst: g(61, 0), left: ScalarSource::Immediate(0),
            right: ScalarSource::Immediate(2), scalar_sync: false }.encode().is_err());
        assert!(ScalarAlu::MultiplyU24 { dst: g(1, 0), left: ScalarSource::Immediate(1024),
            right: ScalarSource::Immediate(0), sync: false }.encode().is_err());
        assert!(ScalarAlu::CompareU32Equal { predicate_component: 4,
            left: ScalarSource::Immediate(0), right: ScalarSource::Immediate(0) }.encode().is_err());
        assert!(StoreLocalU32 { address: g(0, 0), source: g(1, 0), offset: 4096, count: 1 }.encode().is_err());
        assert!(StoreLocalU32 { address: g(0, 0), source: g(1, 0), offset: 0, count: 0 }.encode().is_err());
    }
}
