// SPDX-License-Identifier: GPL-3.0-only
//! IR3 category-7 memory synchronization instruction fields.
//!
//! Mesa documents `bar` (category 7, opcode 0) with a discriminator bit at
//! position 49. Some older A6xx firmware uses the same category, operation,
//! and memory-space fields without that discriminator. Its exact semantics
//! have **not** been validated on silicon, so compatibility encoding is
//! intentionally explicit and must not be assumed interchangeable with
//! Mesa's modern `bar` encoding.

/// Named category-7 memory spaces and accesses. Each flag corresponds to a
/// field in Mesa Freedreno `src/freedreno/isa/ir3-cat7.xml` (MIT license).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoryScope {
    pub global: bool,
    pub local: bool,
    pub read: bool,
    pub write: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarrierFormat {
    /// Public Mesa IR3 barrier discriminator is set (bit 49).
    MesaDocumented,
    /// Older A6xx firmware encoding has the same fields but **no** bit 49.
    /// The operational meaning has not been established independently.
    LegacyA6xxCandidate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryBarrier {
    pub scope: MemoryScope,
    pub format: BarrierFormat,
    pub synchronize: bool,
    pub scalar_synchronize: bool,
    pub jump_point: bool,
}

impl MemoryBarrier {
    pub fn encode(self) -> Result<[u8; 8], &'static str> {
        if !self.scope.global && !self.scope.local && !self.scope.read && !self.scope.write {
            return Err("memory barrier must select at least one memory scope or access");
        }
        let marker = match self.format {
            BarrierFormat::MesaDocumented => 1u64,
            BarrierFormat::LegacyA6xxCandidate => 0u64,
        };
        // Mesa category 7: opcode=0 for bar, G/L/R/W at bits 54/53/52/51,
        // marker at bit49, synchronization flags at 60/59/44. All unspecified
        // fields are zero. The legacy variant does not copy any source opcode.
        let instruction = (7u64 << 61)
            | ((self.synchronize as u64) << 60)
            | ((self.jump_point as u64) << 59)
            | ((self.scope.global as u64) << 54)
            | ((self.scope.local as u64) << 53)
            | ((self.scope.read as u64) << 52)
            | ((self.scope.write as u64) << 51)
            | (marker << 49)
            | ((self.scalar_synchronize as u64) << 44);
        Ok(instruction.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn category_seven_memory_barriers_have_explicit_versioned_encoding() {
        let mut op = MemoryBarrier {
            scope: MemoryScope { global: true, ..MemoryScope::default() },
            format: BarrierFormat::MesaDocumented,
            synchronize: false, scalar_synchronize: false, jump_point: false,
        };
        let modern = u64::from_le_bytes(op.encode().unwrap());
        assert_eq!(modern >> 61, 7);
        assert_eq!((modern >> 55) & 0xf, 0); // bar
        assert_eq!((modern >> 54) & 1, 1); // .g
        assert_eq!((modern >> 49) & 1, 1); // Mesa discriminator
        op.format = BarrierFormat::LegacyA6xxCandidate;
        let legacy = u64::from_le_bytes(op.encode().unwrap());
        assert_eq!(legacy ^ modern, 1 << 49);
        assert_eq!((legacy >> 54) & 1, 1);
        assert_eq!((legacy >> 49) & 1, 0);
    }
    #[test]
    fn rejects_unspecified_scope_and_encodes_all_flags_independently() {
        let mut op = MemoryBarrier {
            scope: MemoryScope::default(), format: BarrierFormat::MesaDocumented,
            synchronize: false, scalar_synchronize: false, jump_point: false,
        };
        assert!(op.encode().is_err());
        op.scope = MemoryScope { local: true, read: true, write: true, global: false };
        op.synchronize = true;
        op.scalar_synchronize = true;
        op.jump_point = true;
        let bits = u64::from_le_bytes(op.encode().unwrap());
        for pos in [60, 59, 53, 52, 51, 49, 44] { assert_eq!((bits >> pos) & 1, 1); }
        assert_eq!((bits >> 54) & 1, 0);
    }
}
