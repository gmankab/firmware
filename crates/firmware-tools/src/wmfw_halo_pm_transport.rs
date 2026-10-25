// SPDX-License-Identifier: GPL-3.0-only
//! Checked CS35L41 HALO *packed-program-memory transport* and host AHB writes.
//!
//! HALO PM is addressed in 40-bit **storage words**; public Linux `cs_dsp`
//! documents five transport bytes per DSP word. Four such words occupy five
//! 32-bit AHB registers. This module preserves their order and addresses,
//! including host register crossings, without presuming which 20-bit halves
//! (if any) are executable instructions. No OEM PM payload is linked or copied
//! into the source-generated result.
//!
//! Based on the public Linux cs_dsp_halo_region_to_reg implementation and
//! CS35L41 PMEM register definitions. It is NOT a HALO ISA decoder.

use crate::wmfw_cs35l41_bus::{PM_PACKED_BASE, PM_PACKED_LAST};

pub const PM_PAYLOAD_FILE_START: usize = 0x278c;
pub const PM_PAYLOAD_FILE_END: usize = 0x76c4;
pub const PM_PAYLOAD_BYTES: usize = PM_PAYLOAD_FILE_END - PM_PAYLOAD_FILE_START;
pub const PM_WORD_BITS: usize = 40;
pub const PM_WORD_BYTES: usize = 5;
pub const PM_WORD_COUNT: usize = PM_PAYLOAD_BYTES / PM_WORD_BYTES;
pub const PM_BLOCK_WORDS: usize = 4;
pub const PM_BLOCK_BYTES: usize = PM_BLOCK_WORDS * PM_WORD_BYTES;
pub const PM_HOST_REGS_PER_BLOCK: usize = 5;
pub const PM_BLOCK_COUNT: usize = PM_WORD_COUNT / PM_BLOCK_WORDS;
pub const PM_MAX_WORD: u64 = (1_u64 << PM_WORD_BITS) - 1;

/// Maximum byte-address *exclusive* of this firmware's CS35L41 PM writes.
pub const PM_AHB_EXCLUSIVE_END: u32 = PM_PACKED_BASE + PM_PAYLOAD_BYTES as u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackedPmBlock(pub [u8; PM_BLOCK_BYTES]);

impl PackedPmBlock {
    pub fn from_slice(bytes: &[u8]) -> Result<Self, String> {
        let block: [u8; PM_BLOCK_BYTES] = bytes.try_into()
            .map_err(|_| format!("HALO PM block requires exactly {PM_BLOCK_BYTES} bytes"))?;
        Ok(Self(block))
    }

    /// Construct one transport block from four 40-bit *values*, with no
    /// instruction meaning assigned to their bit fields.
    pub fn from_words(words: [u64; PM_BLOCK_WORDS]) -> Result<Self, String> {
        let mut packed = [0u8; PM_BLOCK_BYTES];
        for (index, word) in words.into_iter().enumerate() {
            if word > PM_MAX_WORD {
                return Err(format!("HALO PM word {index} exceeds {PM_WORD_BITS}-bit limit"));
            }
            for byte_idx in 0..PM_WORD_BYTES {
                packed[index * PM_WORD_BYTES + byte_idx] =
                    (word >> (8 * (PM_WORD_BYTES - 1 - byte_idx))) as u8;
            }
        }
        Ok(Self(packed))
    }

    pub fn words(&self) -> [u64; PM_BLOCK_WORDS] {
        std::array::from_fn(|index| {
            self.0[index * PM_WORD_BYTES..(index + 1) * PM_WORD_BYTES]
                .iter().fold(0u64, |acc, byte| (acc << 8) | u64::from(*byte))
        })
    }

    /// Exact host register values as interpreted by `regmap` big-endian
    /// accesses. This is *five* 32-bit registers, not four 40-bit AHB writes.
    pub fn host_registers(&self) -> [u32; PM_HOST_REGS_PER_BLOCK] {
        std::array::from_fn(|index| {
            let off = index * 4;
            u32::from_be_bytes(self.0[off..off + 4].try_into().unwrap())
        })
    }

    pub fn from_host_registers(registers: [u32; PM_HOST_REGS_PER_BLOCK]) -> Self {
        let mut packed = [0u8; PM_BLOCK_BYTES];
        for (index, value) in registers.into_iter().enumerate() {
            packed[4 * index..4 * index + 4].copy_from_slice(&value.to_be_bytes());
        }
        Self(packed)
    }

    /// Structural 20-bit split, **not** a decoded HALO instruction pair.
    /// There is no evidence that either half is an independent instruction.
    pub fn halves_20(&self) -> [(u32, u32); PM_BLOCK_WORDS] {
        self.words().map(|word| ((word >> 20) as u32, (word & 0xfffff) as u32))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostWrite {
    /// Actual 4-byte-aligned byte address in the CS35L41 PMEM regmap.
    pub register_address: u32,
    pub big_endian_value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PmWordRegisterWindow {
    pub word_index: usize,
    pub first_register: u32,
    pub first_byte_lane: u8,
    pub last_register: u32,
}

/// Locate the host registers containing one five-byte DSP transport word.
/// A 40-bit value necessarily crosses two adjacent 32-bit registers.
/// Its first-byte lane depends on the word index.
pub fn window_for_word(word_index: usize) -> Result<PmWordRegisterWindow, String> {
    if word_index >= PM_WORD_COUNT {
        return Err(format!("HALO PM index {word_index} outside firmware PM range"));
    }
    let first = word_index.checked_mul(PM_WORD_BYTES).ok_or("PM word offset overflow")?;
    let last = first.checked_add(PM_WORD_BYTES - 1).ok_or("PM word end overflow")?;
    let first_reg_offset = first & !3usize;
    let last_reg_offset = last & !3usize;
    let first_register = PM_PACKED_BASE.checked_add(
        u32::try_from(first_reg_offset).map_err(|_|"PM register offset out of range")?
    ).ok_or("PM first register overflow")?;
    let last_register = PM_PACKED_BASE.checked_add(
        u32::try_from(last_reg_offset).map_err(|_|"PM register offset out of range")?
    ).ok_or("PM last register overflow")?;
    if last_register > PM_PACKED_LAST {
        return Err("PM word accesses beyond CS35L41 PMEM register window".into());
    }
    Ok(PmWordRegisterWindow {
        word_index, first_register, first_byte_lane: (first & 3) as u8,
        last_register,
    })
}

/// Produce an exact ordered host-write sequence for a *fully aligned* slice.
/// Unaligned starts/sizes are rejected instead of inferring the missing
/// neighboring transport bytes from an OEM blob or overwriting other words.
pub fn host_writes_for_blocks(start_word: usize, payload: &[u8]) -> Result<Vec<HostWrite>, String> {
    if start_word % PM_BLOCK_WORDS != 0 {
        return Err("PM host write starts inside a four-word transport block".into());
    }
    if payload.is_empty() || payload.len() % PM_BLOCK_BYTES != 0 {
        return Err("PM host writes require nonempty full 20-byte transport blocks".into());
    }
    let bytes_before = start_word.checked_mul(PM_WORD_BYTES).ok_or("PM offset overflow")?;
    let end_byte = bytes_before.checked_add(payload.len()).ok_or("PM byte extent overflow")?;
    if end_byte > PM_PAYLOAD_BYTES {
        return Err("PM host write outside assigned firmware program-memory section".into());
    }
    let base = PM_PACKED_BASE.checked_add(
        u32::try_from(bytes_before).map_err(|_|"PM start beyond u32")?
    ).ok_or("PM start register overflow")?;
    let mut writes = Vec::with_capacity(payload.len()/4);
    for (index, chunk) in payload.chunks_exact(4).enumerate() {
        let offset=u32::try_from(index.checked_mul(4).ok_or("PM register stride overflow")?)
            .map_err(|_| "PM register offset beyond u32")?;
        let register_address = base.checked_add(offset).ok_or("PM register address overflow")?;
        if register_address > PM_PACKED_LAST {
            return Err("PM host write exceeds hardware register range".into());
        }
        writes.push(HostWrite {register_address,
            big_endian_value: u32::from_be_bytes(chunk.try_into().unwrap())});
    }
    Ok(writes)
}

/// Reconstruct packed transport bytes from a sequence of host register writes.
/// No missing registers, reorder, duplicates, or gaps are accepted.
pub fn blocks_from_host_writes(start_word: usize, writes: &[HostWrite]) -> Result<Vec<u8>, String> {
    if start_word % PM_BLOCK_WORDS != 0 || writes.is_empty() ||
        writes.len() % PM_HOST_REGS_PER_BLOCK != 0 {
        return Err("PM host register list requires five registers per block".into());
    }
    let count = writes.len().checked_mul(4).ok_or("host register byte count overflow")?;
    let byte_base=start_word.checked_mul(PM_WORD_BYTES).ok_or("start word offset overflow")?;
    if byte_base.checked_add(count).ok_or("PM write end overflow")? > PM_PAYLOAD_BYTES {
        return Err("PM host register list outside firmware range".into());
    }
    let first = PM_PACKED_BASE.checked_add(
        u32::try_from(byte_base).map_err(|_|"start PM address too large")?
    ).ok_or("PM start address overflow")?;
    let mut out=Vec::with_capacity(count);
    for (i,write) in writes.iter().enumerate() {
        let expected=first.checked_add(
            u32::try_from(i.checked_mul(4).ok_or("PM host write index overflow")?)
                .map_err(|_|"host write stride overflow")?
        ).ok_or("PM host address overflow")?;
        if write.register_address!=expected {
            return Err(format!("PM register {i} at 0x{:08x}, expected 0x{expected:08x}",write.register_address));
        }
        if write.register_address>PM_PACKED_LAST {
            return Err("PM host register outside device PM window".into());
        }
        out.extend_from_slice(&write.big_endian_value.to_be_bytes());
    }
    Ok(out)
}

/// Atomic bounded patch to one transport word. Only five requested bytes
/// change; all surrounding word bytes are preserved. No executable meaning.
pub fn patch_pm_word(pm: &mut [u8], index: usize, word: u64) -> Result<(), String> {
    if word > PM_MAX_WORD {return Err("PM patch value exceeds 40 bits".into());}
    if pm.len()%PM_WORD_BYTES!=0 || pm.len()>PM_PAYLOAD_BYTES {
        return Err("PM patch buffer has invalid word count".into());
    }
    let start=index.checked_mul(PM_WORD_BYTES).ok_or("PM word index overflow")?;
    let end=start.checked_add(PM_WORD_BYTES).ok_or("PM patch extent overflow")?;
    let slice=pm.get_mut(start..end).ok_or("PM patch index outside buffer")?;
    for (i,byte) in slice.iter_mut().enumerate() {
        *byte=(word >> (8*(PM_WORD_BYTES-1-i))) as u8;
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;

    #[test] fn transport_properties_and_four_word_boundary(){
        assert_eq!(PM_PAYLOAD_BYTES,20280);
        assert_eq!(PM_WORD_COUNT,4056);
        assert_eq!(PM_BLOCK_COUNT,1014);
        assert_eq!(PM_AHB_EXCLUSIVE_END,0x03804f38);
        assert_eq!(PM_AHB_EXCLUSIVE_END-4,0x03804f34);
        assert!(PM_PACKED_LAST>PM_AHB_EXCLUSIVE_END-4);
    }
    #[test] fn reconstruct_twenty_byte_block_from_5_host_registers(){
        let b=PackedPmBlock::from_words([
            0x0102030405,0x060708090a,0x0b0c0d0e0f,0x1011121314,
        ]).unwrap();
        assert_eq!(b.0, [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]);
        assert_eq!(b.host_registers(),[0x01020304,0x05060708,0x090a0b0c,0x0d0e0f10,0x11121314]);
        assert_eq!(PackedPmBlock::from_host_registers(b.host_registers()),b);
        assert_eq!(b.words(),[0x0102030405,0x060708090a,0x0b0c0d0e0f,0x1011121314]);
        assert_eq!(b.halves_20()[0],(0x01020,0x30405));
    }
    #[test] fn bitwise_exact_all_160_positions(){
        let zeros=PackedPmBlock([0;20]);
        for bit in 0..160 {
            let mut bytes=zeros.0;
            bytes[bit/8] |= 1 << (7-(bit%8));
            let block=PackedPmBlock::from_slice(&bytes).unwrap();
            assert_eq!(PackedPmBlock::from_words(block.words()).unwrap(),block);
            assert_eq!(PackedPmBlock::from_host_registers(block.host_registers()),block);
        }
    }
    #[test] fn rejects_overwide_and_truncated_blocks(){
        assert!(PackedPmBlock::from_words([1u64<<40,0,0,0]).is_err());
        assert!(PackedPmBlock::from_slice(&[0u8;19]).is_err());
        assert!(PackedPmBlock::from_slice(&[0u8;21]).is_err());
    }
    #[test] fn word_register_mapping_spans_lanes_including_three_register_case(){
        let expected=[(0,4,0),(4,8,1),(8,12,2),(12,16,3)];
        for (index,(start,end,lane)) in expected.into_iter().enumerate(){
            let w=window_for_word(index).unwrap();
            assert_eq!((w.first_register-PM_PACKED_BASE,w.last_register-PM_PACKED_BASE,w.first_byte_lane),
                       (start,end,lane));
        }
        let last=window_for_word(PM_WORD_COUNT-1).unwrap();
        assert_eq!(last.last_register,PM_AHB_EXCLUSIVE_END-4);
        assert_eq!(last.first_byte_lane,3);
        assert!(window_for_word(PM_WORD_COUNT).is_err());
        assert!(window_for_word(usize::MAX).is_err());
    }
    #[test] fn full_bus_block_transfer_and_reassemble_strictly(){
        let mut blocks=Vec::new();
        for i in 0..PM_BLOCK_COUNT {
            let data:Vec<u8>=(0..20).map(|j| ((i*13+j*47)&255)as u8).collect();
            blocks.extend(data);
        }
        let list=host_writes_for_blocks(0,&blocks).unwrap();
        assert_eq!(list.len(),5070);
        assert_eq!(list[0].register_address,PM_PACKED_BASE);
        assert_eq!(list[list.len()-1].register_address,PM_AHB_EXCLUSIVE_END-4);
        assert_eq!(blocks_from_host_writes(0,&list).unwrap(),blocks);
        let mut bad=list.clone();bad[200].register_address+=4;
        assert!(blocks_from_host_writes(0,&bad).is_err());
        let mut short=list.clone();short.pop();
        assert!(blocks_from_host_writes(0,&short).is_err());
    }
    #[test] fn bounds_and_alignment_fail_without_filling_missing_oem_bytes(){
        let full=[0u8;20];
        assert!(host_writes_for_blocks(1,&full).is_err());
        assert!(host_writes_for_blocks(0,&[]).is_err());
        assert!(host_writes_for_blocks(0,&[0u8;15]).is_err());
        assert!(host_writes_for_blocks(PM_WORD_COUNT,&full).is_err());
        assert!(host_writes_for_blocks(usize::MAX-3,&full).is_err());
        assert!(blocks_from_host_writes(1,&[]).is_err());
    }
    #[test] fn isolated_patch_preserves_neighboring_words_and_is_atomic(){
        let mut block=[0xa5u8;40];
        patch_pm_word(&mut block,3,0x0112233445).unwrap();
        assert_eq!(&block[15..20], &[1,0x12,0x23,0x34,0x45]);
        assert!(block[..15].iter().all(|b|*b==0xa5));
        assert!(block[20..].iter().all(|b|*b==0xa5));
        let old=block;
        assert!(patch_pm_word(&mut block,8,0).is_err());
        assert!(patch_pm_word(&mut block,0,1<<40).is_err());
        assert_eq!(block,old);
    }
}
