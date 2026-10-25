// SPDX-License-Identifier: GPL-3.0-only
//! Typed representation of the Nabu ILM's *candidate* handler-vector table.
//! These are caller-provided addresses, not independently rebuilt executable
//! instructions or evidence of an MCU ISA. The trailing 80 bytes are reserved.
use super::novatek_layout::{NabuNovatekLayout, HEADER_BYTES};

pub const VECTOR_FILE_OFFSET: usize = HEADER_BYTES;
pub const VECTOR_BYTES: usize = 0x200;
pub const VECTOR_FIRST_CODE: usize = 0x450;
pub const VECTOR_RESERVED_BYTES: usize = VECTOR_FIRST_CODE-(VECTOR_FILE_OFFSET+VECTOR_BYTES);
pub const VECTOR_COUNT: usize = VECTOR_BYTES/4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateVectorTable {
    /// First word is a controller-specific marker/configuration (NOT a target).
    pub initial_control_word: u32,
    /// 127 opaque handler-address/descriptor scalars; odd entries are allowed.
    /// No ARM/Thumb/NDS32 instruction interpretation is inferred here.
    pub opaque_slots: [u32; VECTOR_COUNT-1],
}

impl CandidateVectorTable {
    pub fn parse(image: &[u8], layout: &NabuNovatekLayout) -> Result<Self, String> {
        if image.len() != layout.image_len {return Err("incorrect Novatek image length".into());}
        if layout.ilm.file_offset as usize != VECTOR_FILE_OFFSET || layout.ilm.end()? <= VECTOR_FIRST_CODE {
            return Err("Novatek vector layout is outside ILM".into());
        }
        Self::from_bytes(&image[VECTOR_FILE_OFFSET..VECTOR_FILE_OFFSET+VECTOR_BYTES],layout)
    }

    pub fn from_bytes(words: &[u8], layout: &NabuNovatekLayout) -> Result<Self,String> {
        if words.len()!=VECTOR_BYTES {return Err("expected 512 external vector metadata bytes".into());}
        let u32_at = |off: usize| -> Result<u32,String> {
            let slice=words.get(off..off+4).ok_or("missing vector word")?;
            Ok(u32::from_le_bytes(slice.try_into().unwrap()))
        };
        let initial_control_word=u32_at(0)?;
        let mut opaque_slots=[0u32;VECTOR_COUNT-1];
        for (i,slot) in opaque_slots.iter_mut().enumerate() {
            let value=u32_at(4*(i+1))?;
            // The candidate handler field's address is a *hypothesis* from
            // the vector layout, not an ISA-decoded control-flow edge.
            if value != 0 {
                let begin=layout.ilm.sram_address as u64;
                let end=begin+(layout.ilm.actual_length()? as u64);
                if !(begin..end).contains(&(value as u64)) {
                    return Err(format!("candidate vector #{i} is outside ILM SRAM"));
                }
            }
            *slot=value;
        }
        Ok(Self{initial_control_word,opaque_slots})
    }

    /// Rebuild vector-table bytes from caller-provided address words, without
    /// copying an opaque executable region or requiring reference bytes.
    pub fn write(&self)->[u8;VECTOR_BYTES] {
        let mut out=[0u8;VECTOR_BYTES];
        out[0..4].copy_from_slice(&self.initial_control_word.to_le_bytes());
        for (i,entry) in self.opaque_slots.iter().enumerate() {
            let start=4*(i+1);
            out[start..start+4].copy_from_slice(&entry.to_le_bytes());
        }
        out
    }

    /// The vector block's reserved gap immediately precedes probable ILM code.
    /// Emitted independently as zeros once reference padding is validated.
    pub fn reserved_padding(reference: &[u8]) -> Result<[u8;VECTOR_RESERVED_BYTES],String> {
        if reference.len() <= VECTOR_FIRST_CODE {return Err("short vector reference".into());}
        if let Some((i,_)) = reference[VECTOR_FILE_OFFSET+VECTOR_BYTES..VECTOR_FIRST_CODE]
            .iter().enumerate().find(|(_,x)|**x != 0) {
            return Err(format!("nonzero reserved vector padding at 0x{:x}",
                VECTOR_FILE_OFFSET+VECTOR_BYTES+i));
        }
        Ok([0;VECTOR_RESERVED_BYTES])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::novatek_layout::{CorePartition,DescriptorBank,InfoDescriptor,IMAGE_LENGTH};
    fn layout()->NabuNovatekLayout{
        let ilm=CorePartition{file_offset:0x200,sram_address:0,size_minus_one:0x5ff,stored_crc:123};
        let dlm=CorePartition{file_offset:0x1000,sram_address:0x20000,size_minus_one:15,stored_crc:123};
        let info=InfoDescriptor{sram_address:0x20000,size_minus_one:0,file_offset:0x1000,stored_crc:0};
        NabuNovatekLayout{ilm,dlm,image_len:IMAGE_LENGTH,banks:[
            DescriptorBank{mode:3,options:0,info:[info;3],auxiliary:info},
            DescriptorBank{mode:1,options:0,info:[info;3],auxiliary:info}]}
    }
    fn firmware()->Vec<u8>{
        let layout=layout();let mut b=vec![0;IMAGE_LENGTH];
        b[..HEADER_BYTES].copy_from_slice(&layout.write_header());
        b[IMAGE_LENGTH-3..].copy_from_slice(b"NVT");
        b
    }
    #[test]
    fn vector_table_roundtrips_typed_words_and_unaligned_targets() {
        let mut raw=firmware();
        raw[0x200..0x204].copy_from_slice(&0xa201_0048u32.to_le_bytes());
        raw[0x224..0x228].copy_from_slice(&0x04ddu32.to_le_bytes());
        raw[0x230..0x234].copy_from_slice(&0x0480u32.to_le_bytes());
        let parsed=CandidateVectorTable::parse(&raw,&layout()).unwrap();
        assert_eq!(parsed.initial_control_word,0xa201_0048);
        assert_eq!(parsed.write().as_slice(),&raw[0x200..0x400]);
    }
    #[test]
    fn reserved_vector_zeros_are_independent() {
        let mut raw=firmware();
        assert_eq!(CandidateVectorTable::reserved_padding(&raw).unwrap().len(),VECTOR_RESERVED_BYTES);
        raw[0x400]=0x6;
        assert!(CandidateVectorTable::reserved_padding(&raw).is_err());
    }
    #[test]
    fn rejects_unmapped_vector_without_guessing_an_isa(){
        let mut raw=firmware();
        raw[0x204..0x208].copy_from_slice(&0xDEADBEEFu32.to_le_bytes());
        assert!(CandidateVectorTable::parse(&raw,&layout()).is_err());
    }
}
