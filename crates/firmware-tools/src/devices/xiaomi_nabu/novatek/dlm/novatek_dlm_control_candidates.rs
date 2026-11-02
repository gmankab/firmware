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

/// A compact candidate for the 22 adjacent 40-bit transfer words.
///
/// Payloads repeat a 0x190 (400) stride and select a handful of slots from
/// three observed base addresses. The recurrence is fully checked against the
/// *reference* but the controller opcode semantics and the origin of all
/// three base addresses are still unknown. Therefore this output MUST NOT be
/// returned from novatek_dlm::fragments() or credited as SourceDlmData.
///
/// The companion parser deliberately accepts other legal external payloads:
/// only verify_reference_assisted_candidates() pins the exact recurrence.
const TRANSFER_INDEX_STRIDE:u16=0x0190;
const SMALL_SELECTOR_STRIDE:u16=0x0010;
const TRANSFER_BASE_A:u16=0x80c8;
const TRANSFER_BASE_B:u16=0x9068;
const SMALL_SELECTOR_BASE:u16=0x4bf4;

pub fn transfer_word_candidate()->Vec<u8>{
    let mut out=Vec::with_capacity(TRANSFER_RECORDS*WORD_BYTES);
    fn write(out:&mut Vec<u8>,class:u8,payload:u16){
        out.extend_from_slice(&[0x20,0x00,class]);
        out.extend_from_slice(&payload.to_le_bytes());
    }
    write(&mut out,0x19,0x9004);
    for index in [4,0,1,2,3,4]{
        write(&mut out,0x32,TRANSFER_BASE_A+index*TRANSFER_INDEX_STRIDE);
    }
    for index in [0,2]{
        write(&mut out,0x19,TRANSFER_BASE_B+index*TRANSFER_INDEX_STRIDE);
    }
    for index in [0,1]{
        write(&mut out,0x02,SMALL_SELECTOR_BASE+index*SMALL_SELECTOR_STRIDE);
    }
    for index in [1,3]{
        write(&mut out,0x19,TRANSFER_BASE_B+index*TRANSFER_INDEX_STRIDE);
    }
    for index in [0,1]{
        write(&mut out,0x02,SMALL_SELECTOR_BASE+index*SMALL_SELECTOR_STRIDE);
    }
    write(&mut out,0x19,TRANSFER_BASE_B+5*TRANSFER_INDEX_STRIDE);
    for index in [4,0,1,2,3,4]{
        write(&mut out,0x32,TRANSFER_BASE_B+index*TRANSFER_INDEX_STRIDE);
    }
    assert_eq!(out.len(),TRANSFER_RECORDS*WORD_BYTES);
    out
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
    let candidate=transfer_word_candidate();
    if let Some(i)=candidate.iter().zip(transfer).position(|(a,b)|a!=b){
        return Err(format!("five-byte transfer recurrence mismatch at file 0x{:x}",
            TRANSFER_RECORDS_START+i));
    }
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
    fn transfer_recursion_has_stride_and_permutation_symmetry(){
        let raw=transfer_word_candidate();
        let parsed=inspect_external_transfer_records(&raw).unwrap();
        assert_eq!(parsed.len(),22);
        assert_eq!(parsed[0].class,0x19);
        assert_eq!(parsed[0].externally_supplied_payload,0x9004);
        for i in 1..7{
            assert_eq!(parsed[i].class,0x32);
            assert_eq!(parsed[i].externally_supplied_payload,
                TRANSFER_BASE_A+[4u16,0,1,2,3,4][i-1]*TRANSFER_INDEX_STRIDE);
        }
        for (i,idx) in [0u16,2,1,3,5].iter().enumerate(){
            assert_eq!(parsed[[7,8,11,12,15][i]].externally_supplied_payload,
                TRANSFER_BASE_B+*idx*TRANSFER_INDEX_STRIDE);
        }
        for (i,idx) in [4u16,0,1,2,3,4].iter().enumerate(){
            assert_eq!(parsed[16+i].externally_supplied_payload,
                TRANSFER_BASE_B+*idx*TRANSFER_INDEX_STRIDE);
        }
        for i in [9,13]{
            assert_eq!(parsed[i].class,0x02);
            assert_eq!(parsed[i].externally_supplied_payload,SMALL_SELECTOR_BASE);
            assert_eq!(parsed[i+1].externally_supplied_payload,
                SMALL_SELECTOR_BASE+SMALL_SELECTOR_STRIDE);
        }
    }
    #[test]
    fn altered_generated_candidate_is_rejected_by_explicit_reference_verifier(){
        let mut image=vec![0;TRANSFER_RECORDS_END];
        image[MIRRORED_WORDS_START..MIRRORED_WORDS_END]
            .copy_from_slice(&mirrored_control_word_candidate());
        image[TRANSFER_RECORDS_START..TRANSFER_RECORDS_END]
            .copy_from_slice(&transfer_word_candidate());
        assert_eq!(verify_reference_assisted_candidates(&image).unwrap(),(160,110));
        image[MIRRORED_WORDS_START+25]^=0x02;
        assert!(verify_reference_assisted_candidates(&image).unwrap_err().contains("0x2d3c9"));
        image[MIRRORED_WORDS_START..MIRRORED_WORDS_END]
            .copy_from_slice(&mirrored_control_word_candidate());
        image[TRANSFER_RECORDS_START+4]^=1;
        assert!(verify_reference_assisted_candidates(&image).unwrap_err()
            .contains("five-byte transfer recurrence"));
    }
}
