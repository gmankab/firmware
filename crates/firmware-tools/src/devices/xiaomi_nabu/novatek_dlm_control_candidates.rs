// SPDX-License-Identifier: GPL-3.0-only
//! Two strongly structured, but still semantically UNPROVEN, DLM control
//! spans. These are intentionally not returned by novatek_dlm::fragments and
//! MUST NOT contribute to provenance's SourceDlmData credit.
//!
//! The packed 40-bit words at [0x2d3b0,0x2d450) obey an exact mirror/phase
//! recurrence, but no independent NDS32 consumer or documented Novatek
//! command-format specification establishes their operation-bit meaning.
//! The adjacent [0x2d450,0x2d4be) words have a common prefix and observed
//! three-value class field, but their payloads are externally provided and
//! therefore cannot be source-authored by guessing a table of numbers.

pub const MIRRORED_WORDS_START: usize=0x2d3b0;
pub const MIRRORED_WORDS_END: usize=0x2d450;
pub const WORD_BYTES:usize=5;
pub const SELECTOR_PAIRS:usize=8;
pub const PHASES:usize=2;
const OBSERVED_UNEXPLAINED_OPERATION_BITS:u64=0x8000_064800;

pub const TRANSFER_RECORDS_START:usize=0x2d450;
pub const TRANSFER_RECORDS_END:usize=0x2d4be;
pub const TRANSFER_RECORDS:usize=22;

/// Generate a *candidate* for thirty-two 40-bit control words. The values
/// are reconstructed from an observed recurrence; they are NOT promoted to
/// authored source because the static operation bits are still unexplained.
/// The serializer uses the Nabu firmware's five-byte little-endian words.
pub fn mirrored_control_word_candidate() -> Vec<u8> {
    let mut result=Vec::with_capacity(MIRRORED_WORDS_END-MIRRORED_WORDS_START);
    for row in 0..SELECTOR_PAIRS {
        for selector in [16+row,31-row] {
            for phase in 0..PHASES {
                let raw=OBSERVED_UNEXPLAINED_OPERATION_BITS
                    | ((selector as u64)<<24) | ((phase as u64)<<23);
                result.extend_from_slice(&raw.to_le_bytes()[..WORD_BYTES]);
            }
        }
    }
    debug_assert_eq!(result.len(),MIRRORED_WORDS_END-MIRRORED_WORDS_START);
    result
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ObservedTransferWord {
    pub class:u8,
    pub externally_supplied_payload:u16,
}

/// Classify the adjacent 22 observed 5-byte words, checking their shared
/// header and field widths. The `payload` is deliberately reported as
/// externally supplied data rather than inferred from the reference.
/// Classification is not a firmware encoder and gives no source credit.
pub fn inspect_external_transfer_records(input:&[u8])->Result<Vec<ObservedTransferWord>,String>{
    if input.len()!=TRANSFER_RECORDS*WORD_BYTES {
        return Err("expected exactly 22 five-byte transfer records".into());
    }
    let mut result=Vec::with_capacity(TRANSFER_RECORDS);
    for (i,record) in input.chunks_exact(WORD_BYTES).enumerate() {
        if record[..2]!=[0x20,0x00] {
            return Err(format!("transfer record {i} has unsupported prefix"));
        }
        if ![0x02,0x19,0x32].contains(&record[2]) {
            return Err(format!("transfer record {i} has unknown class"));
        }
        result.push(ObservedTransferWord {
            class:record[2],
            externally_supplied_payload:u16::from_le_bytes([record[3],record[4]]),
        });
    }
    Ok(result)
}

/// Verify both candidate observations against a caller-supplied reference.
/// This must never be used as evidence of new source-owned DLM bytes.
pub fn verify_reference_assisted_candidates(image:&[u8])->Result<(usize,usize),String>{
    let observed=image.get(MIRRORED_WORDS_START..MIRRORED_WORDS_END)
        .ok_or("missing mirrored candidate bytes")?;
    let generated=mirrored_control_word_candidate();
    if let Some(i)=generated.iter().zip(observed).position(|(a,b)|a!=b) {
        return Err(format!("mirrored control candidate mismatch at file 0x{:x}",
            MIRRORED_WORDS_START+i));
    }
    let transfer=image.get(TRANSFER_RECORDS_START..TRANSFER_RECORDS_END)
        .ok_or("missing external transfer record bytes")?;
    let parsed=inspect_external_transfer_records(transfer)?;
    Ok((generated.len(),parsed.len()*WORD_BYTES))
}

#[cfg(test)] mod tests{
    use super::*;
    #[test]
    fn mirrored_forty_bit_candidates_have_phase_and_selector_symmetry(){
        let result=mirrored_control_word_candidate();
        assert_eq!(result.len(),160);
        for (i,word) in result.chunks_exact(WORD_BYTES).enumerate(){
            let n=u64::from_le_bytes([word[0],word[1],word[2],word[3],word[4],0,0,0]);
            let row=i/4;
            let selector=if i%4<2 {16+row}else{31-row};
            assert_eq!((n>>24)&0x1f,selector as u64);
            assert_eq!((n>>23)&1,(i%2) as u64);
            assert_eq!(n&!(0x1f<<24)&!(1<<23),OBSERVED_UNEXPLAINED_OPERATION_BITS);
        }
    }
    #[test]
    fn transfer_candidate_parser_validates_length_prefix_and_class(){
        let mut external=Vec::new();
        for i in 0..TRANSFER_RECORDS {
            external.extend_from_slice(&[0x20,0x00,[0x19,0x02,0x32][i%3],i as u8,0x80]);
        }
        let p=inspect_external_transfer_records(&external).unwrap();
        assert_eq!(p.len(),22);
        assert_eq!(p[0].externally_supplied_payload,0x8000);
        assert_eq!(p[21].externally_supplied_payload,0x8015);
        external[4]^=0x01; // This payload remains outside source ownership.
        assert!(inspect_external_transfer_records(&external).is_ok());
        external[0]^=0x20;
        assert!(inspect_external_transfer_records(&external).is_err());
        external[0]^=0x20;
        external[2]=0xfe;
        assert!(inspect_external_transfer_records(&external).is_err());
        assert!(inspect_external_transfer_records(&external[..109]).is_err());
    }
    #[test]
    fn altered_generated_candidate_is_rejected_by_explicit_reference_verifier(){
        let mut image=vec![0;TRANSFER_RECORDS_END];
        image[MIRRORED_WORDS_START..MIRRORED_WORDS_END]
            .copy_from_slice(&mirrored_control_word_candidate());
        for record in image[TRANSFER_RECORDS_START..TRANSFER_RECORDS_END].chunks_exact_mut(5){
            record.copy_from_slice(&[0x20,0x00,0x19,0,0]);
        }
        assert_eq!(verify_reference_assisted_candidates(&image).unwrap(),(160,110));
        image[MIRRORED_WORDS_START+25]^=0x02;
        assert!(verify_reference_assisted_candidates(&image).unwrap_err().contains("0x2d3c9"));
    }
}
