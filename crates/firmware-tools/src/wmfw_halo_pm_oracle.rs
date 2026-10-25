// SPDX-License-Identifier: GPL-3.0-only
//! Bit-accurate *acceptance gate* for externally source-assembled HALO
//! instruction candidates, whose architecture/semantics must first be proved.
//!
//! No opcode interpretation lives here. An unsupported ISA cannot be
//! "disassembled" by treating arbitrary 40-bit program words as two 20-bit
//! instructions. This validator accepts only explicit 20/40-bit candidate
//! spans with a nonempty external source/assembler evidence identifier.
//! It checks that their values match the reference in-place without copying
//! any OEM bytes into the built Rust library or output.
//!
//! Even a passing match only proves chosen candidate bits equal the OEM
//! reference; it does NOT prove a real HALO assembler, source provenance,
//! high-level behavior, program entrypoints or CPU semantics. Human audit of
//! each `source_evidence_id` is mandatory before counting source recovery.
use crate::wmfw_halo_pm_transport::{PM_WORD_BYTES, PM_WORD_COUNT, PM_PAYLOAD_BYTES, PM_MAX_WORD};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Width {
    Short20,
    Long40,
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Candidate<'a> {
    /// Storage word index in `[0,4056)`, not an established HALO PC address.
    pub pm_word_index: usize,
    /// For Short20: high half is 0 and low half is 1. Long40 must use 0.
    pub half: usize,
    pub width: Width,
    pub encoded_bits: u64,
    /// Trusted evidence linking these bits to independently authored code.
    /// An identifier is not proof; audit the referenced source separately.
    pub source_evidence_id: &'a str,
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct CandidateCoverage {
    pub matched_20bit_slots: usize,
    pub fully_covered_storage_words: usize,
    pub matched_candidate_count: usize,
    pub first_uncovered_storage_word: Option<usize>,
    pub first_uncovered_half: Option<usize>,
    pub coverage_complete: bool,
}

/// Compare candidate encodings to the reference. Empty candidates are legal:
/// they give the honest zero-source-coverage report for an unknown HALO ISA.
pub fn compare_candidates(reference_pm: &[u8], candidates: &[Candidate<'_>])
    -> Result<CandidateCoverage,String> {
    if reference_pm.len() != PM_PAYLOAD_BYTES {
        return Err(format!("expected exact {PM_PAYLOAD_BYTES} packed PM bytes, got {}",reference_pm.len()));
    }
    let mut slots=vec![false;PM_WORD_COUNT*2];
    for (i,candidate) in candidates.iter().enumerate() {
        if candidate.source_evidence_id.trim().is_empty() {
            return Err(format!("candidate {i} has no source/assembler evidence identifier"));
        }
        if candidate.pm_word_index>=PM_WORD_COUNT || candidate.half>1 {
            return Err(format!("candidate {i} outside PM storage word/half range"));
        }
        if candidate.width==Width::Long40 && candidate.half!=0 {
            return Err(format!("candidate {i} 40-bit instruction must start at high half"));
        }
        let (slot_count,mask) = match candidate.width {
            Width::Short20 => (1usize,(1u64<<20)-1),
            Width::Long40 => (2usize,PM_MAX_WORD),
        };
        if candidate.encoded_bits>mask {
            return Err(format!("candidate {i} exceeds its {}-bit field",slot_count*20));
        }
        let slot=candidate.pm_word_index*2+candidate.half;
        let end=slot.checked_add(slot_count).ok_or("candidate slot end overflow")?;
        if end>slots.len() || slots[slot..end].iter().any(|v|*v) {
            return Err(format!("candidate {i} overlaps or exceeds a PM instruction half-slot"));
        }
        let byte_index=candidate.pm_word_index*PM_WORD_BYTES;
        let storage = reference_pm[byte_index..byte_index+PM_WORD_BYTES]
            .iter().fold(0u64,|acc,b|(acc<<8)|u64::from(*b));
        let reference_bits=if candidate.width==Width::Long40 {storage}
            else if candidate.half==0 {(storage>>20)&mask}
            else {storage&mask};
        if candidate.encoded_bits!=reference_bits {
            return Err(format!("candidate {i} differs at PM word {} half {}",
                candidate.pm_word_index,candidate.half));
        }
        slots[slot..end].fill(true);
    }
    let first_uncovered=slots.iter().position(|v|!*v);
    Ok(CandidateCoverage {
        matched_20bit_slots:slots.iter().filter(|v|**v).count(),
        fully_covered_storage_words:slots.chunks_exact(2).filter(|pair|pair==&[true,true]).count(),
        matched_candidate_count:candidates.len(),
        first_uncovered_storage_word:first_uncovered.map(|n|n/2),
        first_uncovered_half:first_uncovered.map(|n|n%2),
        coverage_complete:first_uncovered.is_none(),
    })
}

/// Construct a complete PM *storage-byte candidate* only if every 20-bit
/// half-slot is explicitly supplied and nonoverlapping. Numeric bit fields
/// are caller inputs from a separate, human-audited assembler: this function
/// does not establish HALO opcode meanings and cannot infer missing bits.
/// Incomplete candidates fail closed, with no output or implicit zero-fill.
pub fn assemble_candidate_storage(candidates:&[Candidate<'_>])->Result<Vec<u8>,String>{
    let mut halves=vec![None::<u32>;PM_WORD_COUNT*2];
    for (i,c) in candidates.iter().enumerate(){
        if c.source_evidence_id.trim().is_empty(){
            return Err(format!("candidate {i} lacks independently audited source identity"));
        }
        if c.pm_word_index>=PM_WORD_COUNT || c.half>1 ||
           (c.width==Width::Long40 && c.half!=0) {
            return Err(format!("candidate {i} is outside a valid 20/40-bit PM span"));
        }
        let slot=c.pm_word_index*2+c.half;
        match c.width {
            Width::Short20 => {
                if c.encoded_bits >= 1<<20 || halves[slot].is_some() {
                    return Err(format!("candidate {i} overwide or overlapping short PM slot"));
                }
                halves[slot]=Some(c.encoded_bits as u32);
            },
            Width::Long40 => {
                if c.encoded_bits>PM_MAX_WORD || halves[slot].is_some() || halves[slot+1].is_some() {
                    return Err(format!("candidate {i} overwide or overlapping long PM slot"));
                }
                halves[slot]=Some((c.encoded_bits>>20) as u32);
                halves[slot+1]=Some((c.encoded_bits&0xfffff) as u32);
            }
        }
    }
    if let Some(hole)=halves.iter().position(Option::is_none){
        return Err(format!("HALO ISA source coverage incomplete at PM word {} half {}",
            hole/2,hole%2));
    }
    let mut out=Vec::with_capacity(PM_PAYLOAD_BYTES);
    for pair in halves.chunks_exact(2){
        let word=(u64::from(pair[0].unwrap())<<20)|u64::from(pair[1].unwrap());
        for shift in (0..PM_WORD_BYTES).rev(){out.push((word>>(shift*8))as u8);}
    }
    debug_assert_eq!(out.len(),PM_PAYLOAD_BYTES);
    Ok(out)
}

#[cfg(test)]mod tests {
    use super::*;
    const EVIDENCE:&str="test: independently authored synthetic 20/40-bit fixtures";
    fn synthetic_pm() -> Vec<u8> {
        let mut reference=vec![0u8;PM_PAYLOAD_BYTES];
        reference[..5].copy_from_slice(&[0x12,0x34,0x56,0x78,0x9a]);
        reference[5..10].copy_from_slice(&[0x03,0x25,0xab,0xcd,0xef]);
        reference
    }
    fn short(word:usize,half:usize,bits:u64)->Candidate<'static>{
        Candidate{pm_word_index:word,half,width:Width::Short20,encoded_bits:bits,source_evidence_id:EVIDENCE}
    }
    #[test]fn unknown_isa_explicitly_counts_zero_recovered_words() {
        let r=compare_candidates(&synthetic_pm(),&[]).unwrap();
        assert_eq!((r.matched_20bit_slots,r.fully_covered_storage_words,r.matched_candidate_count),(0,0,0));
        assert_eq!((r.first_uncovered_storage_word,r.first_uncovered_half),(Some(0),Some(0)));
        assert!(!r.coverage_complete);
    }
    #[test]fn one_long_and_two_short_fields_cover_two_full_words() {
        let candidates=[
            Candidate{pm_word_index:0,half:0,width:Width::Long40,encoded_bits:0x123456789a,source_evidence_id:EVIDENCE},
            short(1,0,0x0325a),short(1,1,0xbcdef),
        ];
        let result=compare_candidates(&synthetic_pm(),&candidates).unwrap();
        assert_eq!((result.matched_candidate_count,result.matched_20bit_slots,
                    result.fully_covered_storage_words),(3,4,2));
        assert_eq!((result.first_uncovered_storage_word,result.first_uncovered_half),(Some(2),Some(0)));
    }
    #[test]fn high_or_low_halves_alone_do_not_count_as_a_full_word() {
        let result=compare_candidates(&synthetic_pm(),&[short(1,1,0xbcdef)]).unwrap();
        assert_eq!(result.fully_covered_storage_words,0);
        assert_eq!(result.matched_20bit_slots,1);
        assert_eq!(result.first_uncovered_storage_word,Some(0));
    }
    #[test]fn overlapping_claims_or_unaligned_long_instruction_rejected() {
        let long=Candidate{pm_word_index:0,half:0,width:Width::Long40,
            encoded_bits:0x123456789a,source_evidence_id:EVIDENCE};
        assert!(compare_candidates(&synthetic_pm(),&[long.clone(),short(0,1,0x6789a)]).is_err());
        let bad=Candidate{half:1,..long};
        assert!(compare_candidates(&synthetic_pm(),&[bad]).is_err());
    }
    #[test]fn out_of_bounds_or_overwide_claims_rejected() {
        assert!(compare_candidates(&synthetic_pm(),&[short(PM_WORD_COUNT,0,0)]).is_err());
        assert!(compare_candidates(&synthetic_pm(),&[short(0,2,0)]).is_err());
        assert!(compare_candidates(&synthetic_pm(),&[short(0,0,1<<20)]).is_err());
        assert!(compare_candidates(&synthetic_pm(),&[Candidate{
            pm_word_index:0,half:0,width:Width::Long40,encoded_bits:1<<40,source_evidence_id:EVIDENCE,
        }]).is_err());
        assert!(compare_candidates(&[0u8;5],&[]).is_err());
    }
    #[test]fn mismatching_external_assembler_candidate_rejected() {
        assert!(compare_candidates(&synthetic_pm(),&[short(0,0,0xfffff)]).is_err());
    }
    #[test]fn no_anonymous_proprietary_opcode_claims(){
        assert!(compare_candidates(&synthetic_pm(),&[Candidate{
            pm_word_index:0,half:0,width:Width::Short20,encoded_bits:0x12345,
            source_evidence_id:"   ",
        }]).is_err());
    }
    #[test]fn incomplete_candidate_storage_cannot_be_silently_zero_filled(){
        assert!(assemble_candidate_storage(&[]).unwrap_err().contains("incomplete at PM word 0 half 0"));
        assert!(assemble_candidate_storage(&[short(0,0,0)]).unwrap_err()
            .contains("incomplete at PM word 0 half 1"));
    }
    #[test]fn source_assembled_synthetic_storage_can_cover_all_program_memory(){
        let independent_synthetic:(Vec<Candidate<'static>>,Vec<u8>) = {
            let mut cs=Vec::with_capacity(PM_WORD_COUNT);
            let mut reference=Vec::with_capacity(PM_PAYLOAD_BYTES);
            for index in 0..PM_WORD_COUNT {
                let word=((index as u64*0x1234567)^(index as u64*0x121233455)) & PM_MAX_WORD;
                cs.push(Candidate {pm_word_index:index,half:0,width:Width::Long40,
                    encoded_bits:word,source_evidence_id:EVIDENCE});
                for shift in (0..PM_WORD_BYTES).rev(){reference.push((word>>(shift*8))as u8);}
            }
            (cs,reference)
        };
        let (cs,reference)=independent_synthetic;
        assert_eq!(assemble_candidate_storage(&cs).unwrap(),reference);
        let c=compare_candidates(&reference,&cs).unwrap();
        assert!(c.coverage_complete);
        assert_eq!((c.matched_20bit_slots,c.fully_covered_storage_words),(8112,4056));
    }
    #[test]fn candidate_storage_rejects_duplicate_and_oversize_values(){
        assert!(assemble_candidate_storage(&[short(0,0,0),short(0,0,0)]).is_err());
        assert!(assemble_candidate_storage(&[short(0,0,1<<20)]).is_err());
    }
    #[test]fn one_word_can_be_claimed_in_two_nonoverlapping_halves(){
        let result=compare_candidates(&synthetic_pm(),&[short(0,1,0x6789a),short(0,0,0x12345)]).unwrap();
        assert_eq!(result.fully_covered_storage_words,1);
    }
}
