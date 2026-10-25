// SPDX-License-Identifier: GPL-3.0-only
//! Bounds-checked inspection of Cirrus HALO *packed* DSP memories.
//!
//! As Linux `cs_dsp_halo_region_to_reg` documents, XM/YM packed memory uses
//! three bytes per DSP word (four words across three 32-bit host registers),
//! whereas packed PM uses five bytes per program word. XM/YM values are logical
//! 24-bit data words recovered from three big-endian 32-bit host registers per
//! four-word group, not consecutive big-endian three-byte slices. PM values
//! remain reversible 40-bit transport representations, without HALO instruction
//! semantics. No original DSP payload is embedded in this module.
//!
//! The XM/YM lane conversion follows Cirrus Logic's public `unpack_memory`
//! and `memory_type_converter` (`p32` to `u24`) at pinned revision:
//! https://github.com/CirrusLogic/mcu-drivers/blob/a05b2878be911ae9ffaa440840cde17af98ba48e/tools/firmware_converter/wmfw_parser.py

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

/// Decode complete XM/YM host-register groups into logical 24-bit DSP words.
/// PM decoding retains five-byte big-endian transport order only and does not
/// attach instruction semantics. Incomplete XM/YM groups are rejected instead
/// of guessing the missing register lanes or adding padding.
pub fn unpack_words(kind: HaloMemoryKind, source: &[u8]) -> Result<Vec<u64>, String> {
    if kind==HaloMemoryKind::PackedProgram {
        if source.len()%5!=0 {
            return Err(format!("packed PM payload {} is not divisible by 5",source.len()));
        }
        let mut words=Vec::with_capacity(source.len()/5);
        for chunk in source.chunks_exact(5) {
            let mut word=0u64;
            for byte in chunk {word=(word<<8)|u64::from(*byte);}
            words.push(word);
        }
        return Ok(words);
    }
    if source.len()%12!=0 {
        return Err(format!("packed XM/YM payload {} is not complete 12-byte register blocks",source.len()));
    }
    let mut words=Vec::with_capacity(source.len()/3);
    for group in source.chunks_exact(12) {
        let r0=u32::from_be_bytes(group[0..4].try_into().unwrap());
        let r1=u32::from_be_bytes(group[4..8].try_into().unwrap());
        let r2=u32::from_be_bytes(group[8..12].try_into().unwrap());
        words.extend([
            u64::from(r0&0x00ff_ffff),
            u64::from((r0>>24)|((r1&0x0000_ffff)<<8)),
            u64::from((r1>>16)|((r2&0x0000_00ff)<<16)),
            u64::from(r2>>8),
        ]);
    }
    Ok(words)
}

/// Encode complete groups of four logical XM/YM words to three host registers.
/// PM uses unchanged five-byte transport order, without decoding instructions.
pub fn pack_words(kind: HaloMemoryKind, words: &[u64]) -> Result<Vec<u8>, String> {
    let width = kind.bytes_per_word();
    let total = words.len().checked_mul(width).ok_or("packed DSP byte length overflow")?;
    for (index, value) in words.iter().enumerate() {
        if value & !kind.word_mask() != 0 {
            return Err(format!("packed DSP word {index} exceeds {} bits",width*8));
        }
    }
    let mut result=Vec::with_capacity(total);
    if kind==HaloMemoryKind::PackedProgram {
        for value in words {
            for shift in (0..5).rev() {
                result.push((value>>(shift*8)) as u8);
            }
        }
        return Ok(result);
    }
    if words.len()%4!=0 {
        return Err(format!("packed XM/YM word count {} is not complete four-word register groups",words.len()));
    }
    for group in words.chunks_exact(4) {
        let a=group[0] as u32;
        let b=group[1] as u32;
        let c=group[2] as u32;
        let d=group[3] as u32;
        let r0=a|((b&0xff)<<24);
        let r1=(b>>8)|((c&0xffff)<<16);
        let r2=(c>>16)|(d<<8);
        for register in [r0,r1,r2] {result.extend(register.to_be_bytes());}
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
    fn x_and_y_use_four_word_register_lanes_not_three_byte_chunks() {
        // Synthetic logical words and their independently specified register
        // lanes; consecutive three-byte BE parsing gives different values.
        let words=[0x112233,0x445566,0x778899,0xaabbcc];
        let bytes=[0x66,0x11,0x22,0x33,0x88,0x99,0x44,0x55,0xaa,0xbb,0xcc,0x77];
        for kind in [HaloMemoryKind::PackedX,HaloMemoryKind::PackedY] {
            assert_eq!(unpack_words(kind,&bytes).unwrap(),words);
            assert_eq!(pack_words(kind,&words).unwrap(),bytes);
            let two_groups=[bytes,bytes].concat();
            assert_eq!(unpack_words(kind,&two_groups).unwrap(),[words,words].concat());
            assert_eq!(pack_words(kind,&[0,0,0,0]).unwrap(),[0;12]);
        }
    }

    #[test]
    fn program_words_remain_five_byte_transport_only() {
        let source=[0xde,0xad,0xbe,0xef,0xaa];
        let words=unpack_words(HaloMemoryKind::PackedProgram,&source).unwrap();
        assert_eq!(words,[0xdeadbeefaa]);
        assert_eq!(pack_words(HaloMemoryKind::PackedProgram,&words).unwrap(),source);
        assert!(unpack_words(HaloMemoryKind::PackedProgram,&[1,2,3]).is_err());
        assert!(pack_words(HaloMemoryKind::PackedProgram,&[1<<40]).is_err());
        assert!(HaloMemoryKind::from_wmfw_type(0xf2).is_err());
    }

    #[test]
    fn rejects_partial_x_y_register_groups_and_out_of_range_words() {
        for kind in [HaloMemoryKind::PackedX,HaloMemoryKind::PackedY] {
            for length in 1..12 {assert!(unpack_words(kind,&vec![0;length]).is_err());}
            for count in 1..4 {assert!(pack_words(kind,&vec![0;count]).is_err());}
            assert!(pack_words(kind,&[1<<24,0,0,0]).unwrap_err().contains("exceeds"));
            assert_eq!(unpack_words(kind,&[0xff;12]).unwrap(),[0xff_ffff;4]);
            assert_eq!(pack_words(kind,&[0xff_ffff;4]).unwrap(),[0xff;12]);
        }
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
