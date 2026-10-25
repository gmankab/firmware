// SPDX-License-Identifier: GPL-3.0-only
//! Reversible transform between the two *reference-supplied* descriptor banks.
//!
//! For two groups of 0x8c-byte records, the later bank is structurally
//! identical to the earlier bank except for nine file-relative word indices
//! in the directory spanning offsets +0x48..+0x5a: adding 0x4000 to the
//! file location corresponds to adding 0x1000 to each 16-bit word index.
//! The tenth halfword is an independently shared cross-bank pointer.
//!
//! **Provenance:** this is a reference-ASSISTED derivation, NOT an
//! independently authored copy of the original descriptor data. The caller
//! must provide all source records; no such records are included here or
//! promoted from UnknownDlm to SourceDlmData by the provenance ledger.

pub const DESCRIPTOR_BYTES:usize=0x8c;
pub const FIRST_GROUP_RECORDS:usize=17;
pub const SECOND_GROUP_RECORDS:usize=4;
pub const FIRST_GROUP_EARLIER:usize=0x22a64;
pub const FIRST_GROUP_LATER:usize=0x26a64;
pub const SECOND_GROUP_EARLIER:usize=0x25230;
pub const SECOND_GROUP_LATER:usize=0x29230;
pub const BANK_FILE_DELTA:usize=0x4000;
pub const WORD_INDEX_DELTA:u16=0x1000;
pub const RELOCATED_HALFWORD_COUNT:usize=9;

/// Deterministically derive later-bank records from an externally supplied
/// earlier bank. Nine directory word indices are incremented by 0x1000;
/// every other byte is retained. Reject invalid widths and pointer overflow.
pub fn translate_records_from_external(first:&[u8])->Result<Vec<u8>,String>{
    if first.is_empty() || first.len()%DESCRIPTOR_BYTES != 0 {
        return Err("descriptor-bank transform requires complete 0x8c-byte records".into());
    }
    let mut result=first.to_vec();
    for record in result.chunks_exact_mut(DESCRIPTOR_BYTES) {
        for index in 0..RELOCATED_HALFWORD_COUNT {
            let at=0x48+index*2;
            let word=u16::from_le_bytes([record[at],record[at+1]]);
            // Bit 15 can independently flag the descriptor selector; the
            // consumer masks it before interpreting the 15-bit word index.
            let index=word & 0x7fff;
            let relocated=index.checked_add(WORD_INDEX_DELTA)
                .filter(|value|*value<=0x7fff)
                .ok_or_else(||format!("descriptor word index at +0x{at:x} cannot shift banks"))?;
            record[at..at+2].copy_from_slice(&((word&0x8000)|relocated).to_le_bytes());
        }
    }
    Ok(result)
}

/// Return all *dependency boundaries*, without performing any source-credit
/// accounting: these outputs remain dependent on the earlier unknown records.
pub const PAIRED_GROUPS:[(usize,usize,usize);2]=[
    (FIRST_GROUP_EARLIER,FIRST_GROUP_LATER,FIRST_GROUP_RECORDS),
    (SECOND_GROUP_EARLIER,SECOND_GROUP_LATER,SECOND_GROUP_RECORDS),
];

pub fn verify_against_supplied_reference(image:&[u8])->Result<usize,String>{
    let mut verified=0;
    for (earlier,later,count) in PAIRED_GROUPS {
        let len=count.checked_mul(DESCRIPTOR_BYTES).ok_or("record length overflow")?;
        let first=image.get(earlier..earlier+len).ok_or("missing earlier descriptor bank")?;
        let second=image.get(later..later+len).ok_or("missing later descriptor bank")?;
        let emitted=translate_records_from_external(first)?;
        if let Some(mismatch)=emitted.iter().zip(second).position(|(a,b)|a!=b) {
            return Err(format!("paired DLM descriptor differs at file 0x{:x}",later+mismatch));
        }
        verified+=len;
    }
    Ok(verified)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]
    fn paired_bank_sizes_are_checked_and_never_counted_as_independent_source(){
        assert_eq!(PAIRED_GROUPS.iter().map(|(_,_,count)|count*DESCRIPTOR_BYTES).sum::<usize>(),2940);
        assert_eq!(FIRST_GROUP_LATER-FIRST_GROUP_EARLIER,BANK_FILE_DELTA);
        assert_eq!(SECOND_GROUP_LATER-SECOND_GROUP_EARLIER,BANK_FILE_DELTA);
    }
    #[test]
    fn transform_preserves_non_pointer_bytes_and_is_reversible(){
        let mut first=vec![0x3c;2*DESCRIPTOR_BYTES];
        for rec in first.chunks_exact_mut(DESCRIPTOR_BYTES) {
            for index in 0..RELOCATED_HALFWORD_COUNT {
                let at=0x48+index*2;
                rec[at..at+2].copy_from_slice(&(0x300+index as u16).to_le_bytes());
            }
            rec[0x5a..0x5c].copy_from_slice(&0x177e_u16.to_le_bytes());
        }
        let result=translate_records_from_external(&first).unwrap();
        for (a,b) in first.chunks_exact(DESCRIPTOR_BYTES).zip(result.chunks_exact(DESCRIPTOR_BYTES)) {
            for i in 0..DESCRIPTOR_BYTES {
                if !(0..RELOCATED_HALFWORD_COUNT).any(|j| i==0x48+2*j || i==0x49+2*j) {
                    assert_eq!(a[i],b[i]);
                }
            }
            assert_eq!(&b[0x5a..0x5c],&0x177e_u16.to_le_bytes());
            for j in 0..RELOCATED_HALFWORD_COUNT {
                let at=0x48+2*j;
                assert_eq!(u16::from_le_bytes(b[at..at+2].try_into().unwrap()),0x1300+j as u16);
            }
        }
    }
    #[test]
    fn flagged_word_selectors_shift_index_but_preserve_selector_flag(){
        let mut record=vec![0;DESCRIPTOR_BYTES];
        for at in (0..RELOCATED_HALFWORD_COUNT).map(|i|0x48+2*i) {
            record[at..at+2].copy_from_slice(&0x8d18_u16.to_le_bytes());
        }
        let out=translate_records_from_external(&record).unwrap();
        for at in (0..RELOCATED_HALFWORD_COUNT).map(|i|0x48+2*i) {
            assert_eq!(u16::from_le_bytes(out[at..at+2].try_into().unwrap()),0x9d18);
        }
    }
    #[test]
    fn rejects_truncated_records_and_overflow(){
        assert!(translate_records_from_external(&[]).is_err());
        assert!(translate_records_from_external(&[0;71]).is_err());
        let mut record=vec![0;DESCRIPTOR_BYTES];
        record[0x48..0x4a].copy_from_slice(&0xf500_u16.to_le_bytes());
        assert!(translate_records_from_external(&record).is_err());
    }
}
