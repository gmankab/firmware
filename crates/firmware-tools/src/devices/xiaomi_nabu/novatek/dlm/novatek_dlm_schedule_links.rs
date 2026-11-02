// SPDX-License-Identifier: GPL-3.0-only
//! Source-authored packed-schedule link fields in Nabu touchscreen DLM.
//!
//! Runtime NDS32 decoder `[0xc7ea,0xc87a)` loads a u32 at descriptor
//! displacement `0x50`, **masks to 15 bits**, multiplies the result by four,
//! and adds SRAM DLM base `0x20000` to locate the first packed 72-byte
//! schedule. Its caller selects 0x8c-byte descriptor records. The initial
//! 17 records in either bank point to the same 24-row wide schedule table.
//! Four following records per bank point to their separate 20-row narrow
//! schedule table. The latter four records may carry an UNKNOWN bit-15
//! selector flag in the low halfword, so only whole bytes unambiguously
//! implied by the 15-bit index are independently credited.
//!
//! Only the *low 16 bits* of each packed descriptor word are independently
//! emitted here. The upper 16 bits have not been semantically established and
//! remain external DLM data, even though they appear structured. That avoids
//! promoting unproven bytes to source credit.

use super::novatek_dlm::DataFragment;
use super::novatek_layout::{DLM_FILE_START, DLM_FILE_END};

pub const DESCRIPTOR_RECORD_BYTES: usize = 0x8c;
pub const LINK_DISPLACEMENT: usize = 0x50;
pub const DESCRIPTORS_PER_BANK: usize = 17;
pub const PRIMARY_DESCRIPTORS: usize = 0x22a64;
pub const SECONDARY_DESCRIPTORS: usize = 0x26a64;
pub const PRIMARY_SCHEDULE: usize = 0x24904;
pub const SECONDARY_SCHEDULE: usize = 0x28904;
pub const SRAM_DLM_BASE: usize = 0x20000;
pub const NARROW_RECORDS_PER_BANK:usize=4;
pub const FIRST_NARROW_RECORDS:usize=0x25230;
pub const SECOND_NARROW_RECORDS:usize=0x29230;
pub const FIRST_NARROW_SCHEDULE:usize=0x25d4c;
pub const SECOND_NARROW_SCHEDULE:usize=0x29d4c;
// The first two narrow records per bank hold an unflagged pointer. The next
// two can have an independently set bit 15; we only credit the low byte there.
pub const UNFLAGGED_NARROW_RECORDS:usize=2;

/// Calculate a file-backed pointer as a 15-bit word index relative to DLM.
/// The firmware turns this index into an SRAM pointer using `index * 4 +
/// 0x20000`. No vendor pointer bytes are included in this source file.
pub fn schedule_word_index(schedule_file_start: usize) -> Result<u16, String> {
    let delta = schedule_file_start.checked_sub(DLM_FILE_START)
        .ok_or("schedule precedes DLM")?;
    if schedule_file_start >= DLM_FILE_END || delta % 4 != 0 {
        return Err("schedule address not an aligned DLM file offset".into());
    }
    let index = delta / 4;
    if index > 0x7fff {
        return Err("schedule pointer exceeds 15-bit word-index field".into());
    }
    Ok(index as u16)
}

/// Inverse formula following the actual NDS32 consumer, with checked range.
pub fn decode_schedule_file(word: u32) -> Result<usize, String> {
    let index = (word & 0x7fff) as usize;
    let start = DLM_FILE_START.checked_add(index * 4)
        .ok_or("schedule address arithmetic overflow")?;
    if start >= DLM_FILE_END { return Err("schedule pointer outside DLM".into()); }
    Ok(start)
}

pub fn fragments() -> Result<Vec<DataFragment>, String> {
    let mut out = Vec::with_capacity(2 * (DESCRIPTORS_PER_BANK + NARROW_RECORDS_PER_BANK));
    for (records_start, schedule_file) in [
        (PRIMARY_DESCRIPTORS, PRIMARY_SCHEDULE),
        (SECONDARY_DESCRIPTORS, SECONDARY_SCHEDULE),
    ] {
        let bytes = schedule_word_index(schedule_file)?.to_le_bytes().to_vec();
        for record in 0..DESCRIPTORS_PER_BANK {
            let start = records_start + record * DESCRIPTOR_RECORD_BYTES + LINK_DISPLACEMENT;
            if start + bytes.len() > DLM_FILE_END {
                return Err("schedule link extends beyond DLM".into());
            }
            out.push(DataFragment {
                name: "wide-schedule-word-link",
                file_start: start,
                bytes: bytes.clone(),
                evidence: "NDS32 consumer: (u32 at record+0x50 & 0x7fff)*4 + 0x20000; 17 records per 0x8c-byte bank",
            });
        }
    }
    for (records_start, schedule_file) in [
        (FIRST_NARROW_RECORDS,FIRST_NARROW_SCHEDULE),
        (SECOND_NARROW_RECORDS,SECOND_NARROW_SCHEDULE),
    ] {
        let bytes=schedule_word_index(schedule_file)?.to_le_bytes();
        for record in 0..NARROW_RECORDS_PER_BANK {
            let file_start=records_start+record*DESCRIPTOR_RECORD_BYTES+LINK_DISPLACEMENT;
            let known_len=if record<UNFLAGGED_NARROW_RECORDS {2} else {1};
            if file_start+known_len>DLM_FILE_END {
                return Err("narrow schedule link extends beyond DLM".into());
            }
            out.push(DataFragment {
                name:"narrow-schedule-word-link",
                file_start,
                bytes:bytes[..known_len].to_vec(),
                evidence:"NDS32 15-bit word index resolves narrow schedule; flagged descriptor bit 15 is intentionally UNKNOWN",
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn schedule_indices_reverse_to_pinned_table_offsets() {
        assert_eq!(schedule_word_index(PRIMARY_SCHEDULE).unwrap(), 0x0a41);
        assert_eq!(schedule_word_index(SECONDARY_SCHEDULE).unwrap(), 0x1a41);
        assert_eq!(decode_schedule_file(0x07a20a41).unwrap(), PRIMARY_SCHEDULE);
        assert_eq!(decode_schedule_file(0x17a21a41).unwrap(), SECONDARY_SCHEDULE);
    }
    #[test]
    fn exact_nonoverlapping_17_record_layout_per_bank() {
        let data=fragments().unwrap();
        assert_eq!(data.len(),42);
        assert_eq!(data.iter().map(|f|f.bytes.len()).sum::<usize>(),80);
        for (i, part) in data.iter().take(34).enumerate() {
            let (record_start,expected) = if i<17 {(PRIMARY_DESCRIPTORS,i)}
                else {(SECONDARY_DESCRIPTORS,i-17)};
            assert_eq!(part.file_start,record_start+0x50+expected*0x8c);
            let word=u16::from_le_bytes(part.bytes[..2].try_into().unwrap());
            assert_eq!(decode_schedule_file(u32::from(word)).unwrap(),
                if i<17 {PRIMARY_SCHEDULE} else {SECONDARY_SCHEDULE});
            assert!(part.bytes[1]&0x80 == 0);
        }
    }
    #[test]
    fn narrow_rows_omit_unproven_selector_flag_byte() {
        let data=fragments().unwrap();
        let narrow:&Vec<_>=&data.iter().filter(|f|f.name=="narrow-schedule-word-link").collect();
        assert_eq!(narrow.len(),8);
        assert_eq!(narrow.iter().map(|f|f.bytes.len()).sum::<usize>(),12);
        for (i,part) in narrow.iter().enumerate() {
            let (start,table)=if i<4 {(FIRST_NARROW_RECORDS,FIRST_NARROW_SCHEDULE)}
                else {(SECOND_NARROW_RECORDS,SECOND_NARROW_SCHEDULE)};
            let rec=i%4;
            assert_eq!(part.file_start,start+rec*DESCRIPTOR_RECORD_BYTES+LINK_DISPLACEMENT);
            let index=schedule_word_index(table).unwrap().to_le_bytes();
            assert_eq!(&part.bytes[..],&index[..if rec<2 {2} else {1}]);
        }
        assert_eq!(schedule_word_index(FIRST_NARROW_SCHEDULE).unwrap(),0x0f53);
        assert_eq!(schedule_word_index(SECOND_NARROW_SCHEDULE).unwrap(),0x1f53);
    }
    #[test]
    fn pointer_rejects_misalignment_and_invalid_bounds() {
        for a in [DLM_FILE_START-4, DLM_FILE_END, PRIMARY_SCHEDULE+1] {
            assert!(schedule_word_index(a).is_err());
        }
        assert!(decode_schedule_file(0x7fff).is_err());
    }
}
