// SPDX-License-Identifier: GPL-3.0-only
//! Bounds-checked inspection of Cirrus HALO *packed* DSP memories.
//!
//! As Linux `cs_dsp_halo_region_to_reg` documents, XM/YM packed memory uses
//! three bytes per DSP word (four words across three 32-bit host registers),
//! whereas packed PM uses five bytes per program word. The normalized `u64`
//! values below are only reversible transport representations; they DO NOT
//! decode the proprietary HALO instruction ISA or make a redistributable ROM.

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum HaloMemoryKind {
    PackedProgram,
    PackedX,
    PackedY,
}

impl HaloMemoryKind {
    pub fn from_wmfw_type(kind: u8) -> Result<Self,String> {
        match kind {
            0x10 => Ok(Self::PackedProgram),
            0x11 => Ok(Self::PackedX),
            0x12 => Ok(Self::PackedY),
            _ => Err(format!("WMFW memory type 0x{kind:02x} is not packed HALO DSP memory")),
        }
    }

    pub fn bytes_per_word(self) -> usize {
        match self { Self::PackedProgram => 5, Self::PackedX | Self::PackedY => 3 }
    }

    pub fn word_mask(self) -> u64 {
        match self { Self::PackedProgram => (1 << 40)-1, Self::PackedX | Self::PackedY => (1 << 24)-1 }
    }

    /// Regmap address of a packed memory region at DSP word offset `offset`.
    /// XM/YM bus transfers begin on a 32-bit register boundary.
    pub fn mapped_register(self, base: u32, offset: u32) -> Result<u32, String> {
        let stride = u32::try_from(self.bytes_per_word()).unwrap();
        let reg = base.checked_add(offset.checked_mul(stride)
            .ok_or("packed memory address multiplication overflow")?)
            .ok_or("packed memory address addition overflow")?;
        Ok(match self {Self::PackedProgram => reg, Self::PackedX | Self::PackedY => reg & !3})
    }
}

/// Preserve big-endian wire-order 24-/40-bit packed values without attaching
/// speculative meanings to unknown HALO opcodes.
pub fn unpack_words(kind: HaloMemoryKind, source: &[u8]) -> Result<Vec<u64>, String> {
    let width = kind.bytes_per_word();
    if source.len() % width != 0 {
        return Err(format!("packed DSP payload {} is not divisible by {width}",source.len()));
    }
    let mut words = Vec::with_capacity(source.len()/width);
    for chunk in source.chunks_exact(width) {
        let mut word = 0u64;
        for byte in chunk { word = (word << 8) | u64::from(*byte); }
        words.push(word);
    }
    Ok(words)
}

pub fn pack_words(kind: HaloMemoryKind, words: &[u64]) -> Result<Vec<u8>, String> {
    let width = kind.bytes_per_word();
    let total = words.len().checked_mul(width).ok_or("packed DSP byte length overflow")?;
    let mut result = Vec::with_capacity(total);
    for (index, value) in words.iter().enumerate() {
        if value & !kind.word_mask() != 0 {
            return Err(format!("packed DSP word {index} exceeds {} bits",width*8));
        }
        for shift in (0..width).rev() {
            result.push((value >> (shift*8)) as u8);
        }
    }
    Ok(result)
}

pub struct HaloRegionSummary {
    pub word_count: usize,
    pub nonzero_word_count: usize,
    pub word_width_bits: usize,
}

pub fn summarize(kind: HaloMemoryKind, payload: &[u8]) -> Result<HaloRegionSummary,String> {
    let words = unpack_words(kind,payload)?;
    Ok(HaloRegionSummary {
        word_count: words.len(),
        nonzero_word_count: words.iter().filter(|w| **w != 0).count(),
        word_width_bits: kind.bytes_per_word()*8,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_distinct_packed_memory_widths_roundtrip() {
        for (kind, source) in [
            (HaloMemoryKind::PackedProgram, &[0xde,0xad,0xbe,0xef,0xaa][..]),
            (HaloMemoryKind::PackedX, &[0x11,0x22,0x33][..]),
            (HaloMemoryKind::PackedY, &[0,0,0][..]),
        ] {
            let words=unpack_words(kind,source).unwrap();
            assert_eq!(words.len(),1);
            assert_eq!(pack_words(kind,&words).unwrap(),source);
        }
        assert!(unpack_words(HaloMemoryKind::PackedProgram,&[1,2,3]).is_err());
        assert!(pack_words(HaloMemoryKind::PackedX,&[1<<24]).is_err());
        assert!(pack_words(HaloMemoryKind::PackedProgram,&[1<<40]).is_err());
        assert!(HaloMemoryKind::from_wmfw_type(0xf2).is_err());
    }

    #[test]
    fn register_mapping_obeys_cirrus_packed_stride() {
        let x=HaloMemoryKind::PackedX;
        assert_eq!(x.mapped_register(0x1000,0).unwrap(),0x1000);
        assert_eq!(x.mapped_register(0x1000,1).unwrap(),0x1000);
        assert_eq!(x.mapped_register(0x1000,2).unwrap(),0x1004);
        assert_eq!(x.mapped_register(0x1000,4).unwrap(),0x100c);
        assert_eq!(HaloMemoryKind::PackedProgram.mapped_register(0x1000,2).unwrap(),0x100a);
        assert!(x.mapped_register(u32::MAX,1).is_err());
        assert!(x.mapped_register(0, u32::MAX).is_err());
    }
}
