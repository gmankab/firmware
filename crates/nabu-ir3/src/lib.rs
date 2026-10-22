// SPDX-License-Identifier: GPL-3.0-only
//! Small, typed subset of the public Adreno IR3 shader ISA.
//!
//! The bit fields follow Mesa Freedreno's MIT-licensed `ir3-cat1.xml`.
//! This encoder supports only category-1 `mov.u32u32` with an unsigned integer
//! immediate. It does **not** assemble an arbitrary IR3 program or interpret
//! the protected firmware's shader algorithm.

/// An ordinary IR3 general-purpose vector register and one component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gpr {
    pub vector: u8,
    /// X=0, Y=1, Z=2, W=3.
    pub component: u8,
}

impl Gpr {
    fn encode(self) -> Result<u8, &'static str> {
        // GPR indices 61 and 62 select special address/predicate registers;
        // keep this initial implementation restricted to ordinary vectors.
        if self.vector > 60 { return Err("IR3 GPR number must be at most 60"); }
        if self.component > 3 { return Err("IR3 component must be 0..3"); }
        Ok(self.vector * 4 + self.component)
    }
}

/// A category-1 unsigned 32-bit immediate register move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovU32Immediate {
    pub dst: Gpr,
    pub immediate: u32,
    /// Mesa's 2-bit REPEAT field: 0 = once; 3 = four successive components.
    pub repeat: u8,
}

impl MovU32Immediate {
    /// Encode a complete IR3 instruction slot (two little-endian 32-bit words).
    pub fn encode(self) -> Result<[u8; 8], &'static str> {
        if self.repeat > 3 { return Err("IR3 repeat must be 0..3"); }
        if u16::from(self.dst.component) + u16::from(self.repeat) > 3 {
            return Err("IR3 repeated move crosses a vector register");
        }
        let dst = u64::from(self.dst.encode()?);
        // Mesa ir3-cat1.xml: category 1 in [61:63], immediate src form
        // in [53:54], uint32 source [50:52], uint32 destination [46:48],
        // repeat [40:41], destination component [32:39].
        let slot = (1u64 << 61)
            | (2u64 << 53)
            | (3u64 << 50)
            | (3u64 << 46)
            | (u64::from(self.repeat) << 40)
            | (dst << 32)
            | u64::from(self.immediate);
        Ok(slot.to_le_bytes())
    }
}

/// Generate a deterministic sequence that zeros four consecutive scalar
/// components of each vector register. No OEM instruction words are inputs.
/// This is an *instruction generator*, not a shader or GPU register emulator.
pub fn zero_vector_registers(first: u8, count: u8) -> Result<Vec<u8>, String> {
    let last = u16::from(first) + u16::from(count);
    if count == 0 || last > 61 {
        return Err("IR3 zero range must contain 1..61 ordinary vector registers".into());
    }
    let mut out = Vec::with_capacity(usize::from(count) * 8);
    for vector in first..first + count {
        let word = MovU32Immediate {
            dst: Gpr { vector, component: 0 }, immediate: 0, repeat: 3,
        };
        out.extend_from_slice(&word.encode().map_err(str::to_string)?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_public_isa_fields() {
        let move_one = MovU32Immediate {
            dst: Gpr { vector: 7, component: 2 }, immediate: 0xabc1, repeat: 1,
        }.encode().unwrap();
        let slot = u64::from_le_bytes(move_one);
        assert_eq!((slot >> 61) & 7, 1); // cat1
        assert_eq!((slot >> 53) & 3, 2); // immediate form
        assert_eq!((slot >> 50) & 7, 3); // u32 src
        assert_eq!((slot >> 46) & 7, 3); // u32 dst
        assert_eq!((slot >> 40) & 3, 1); // repeat next component
        assert_eq!((slot >> 32) & 0xff, 7 * 4 + 2);
        assert_eq!(slot as u32, 0xabc1);
    }

    #[test]
    fn generates_distinct_register_operands_without_embedded_opcodes() {
        let bank = zero_vector_registers(0, 48).unwrap();
        assert_eq!(bank.len(), 48 * 8);
        for (vector, bytes) in bank.chunks_exact(8).enumerate() {
            let slot = u64::from_le_bytes(bytes.try_into().unwrap());
            assert_eq!((slot >> 32) & 0xff, vector as u64 * 4);
            assert_eq!((slot >> 40) & 3, 3);
            assert_eq!(slot as u32, 0);
        }
    }

    #[test]
    fn rejects_invalid_registers_components_repeats_and_ranges() {
        assert!(MovU32Immediate {
            dst: Gpr { vector: 61, component: 0 }, immediate: 0, repeat: 0,
        }.encode().is_err());
        assert!(MovU32Immediate {
            dst: Gpr { vector: 5, component: 4 }, immediate: 0, repeat: 0,
        }.encode().is_err());
        assert!(MovU32Immediate {
            dst: Gpr { vector: 5, component: 2 }, immediate: 0, repeat: 3,
        }.encode().is_err());
        assert!(MovU32Immediate {
            dst: Gpr { vector: 5, component: 0 }, immediate: 0, repeat: 4,
        }.encode().is_err());
        assert!(zero_vector_registers(48, 14).is_err());
        assert!(zero_vector_registers(0, 0).is_err());
        assert_eq!(zero_vector_registers(60, 1).unwrap().len(), 8);
    }
}
