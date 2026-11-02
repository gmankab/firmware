// SPDX-License-Identifier: GPL-3.0-only
//! Externally observed DLM record-directory geometry and profile alias.
//!
//! The first halfword at byte +0x48 in each 0x8c-byte descriptor record is
//! an indexed pointer to the end of the record array. The consumer masks its
//! upper flag bit and scales the remaining 15 bits by four. This is an
//! independently CHECKABLE layout relationship, but the external directory
//! locations/counts and flag bits have not been sourced from original code.
//! Hence no source-byte credit is awarded for matching these pointers.
use super::novatek_dlm_provenance::{LedgerKind,ProvenanceLedger};

pub const FILE_DLM_START:usize=0x22000;
pub const BANK_OFFSET:usize=0x4000;
pub const RECORD_BYTES:usize=0x8c;
pub const POINTER_OFFSET:usize=0x48;
pub const INDEX_MASK:u16=0x7fff;
pub const INDEX_FLAG:u16=0x8000;
pub const FIRST_SIGNATURE_START:usize=0x22710;
pub const REPEATED_SIGNATURE_START:usize=0x2df8c;
pub const SIGNATURE_BYTES:usize=12;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct DirectoryGroup{pub start:usize,pub records:usize}

pub const DIRECTORY_GROUPS:[DirectoryGroup;2]=[
    DirectoryGroup{start:0x22a64,records:17},
    DirectoryGroup{start:0x25230,records:4},
];
pub const PAIRS_OF_DIRECTORY_RECORDS:usize=2*(17+4);

pub fn directory_array_end(group:DirectoryGroup,bank:usize)->Result<usize,String>{
    if !DIRECTORY_GROUPS.contains(&group) || bank>1 {
        return Err("unknown Novatek DLM descriptor group or bank".into());
    }
    group.start.checked_add(bank.checked_mul(BANK_OFFSET).ok_or("bank offset overflow")?)
        .and_then(|x|x.checked_add(group.records.checked_mul(RECORD_BYTES)?))
        .ok_or("descriptor table address overflow".into())
}

pub fn pointer_index_from_directory_geometry(group:DirectoryGroup,bank:usize)->Result<u16,String>{
    let end=directory_array_end(group,bank)?;
    if end<FILE_DLM_START || (end-FILE_DLM_START)%4!=0 {
        return Err("directory target not word aligned or below DLM base".into());
    }
    let n=(end-FILE_DLM_START)/4;
    u16::try_from(n).ok().filter(|x|*x<=INDEX_MASK)
        .ok_or("directory target cannot fit 15-bit word index".into())
}

fn ensure_unknown(ledger:&ProvenanceLedger,start:usize,end:usize)->Result<(),String>{
    let mut at=start;
    for region in &ledger.rows {
        if region.file_end<=at || region.file_start>=end {continue;}
        if region.file_start>at || region.kind!=LedgerKind::UnknownDlm {
            return Err(format!("geometry dependency at file 0x{at:x} must remain unknown DLM"));
        }
        at=at.max(region.file_end.min(end));
    }
    if at!=end {return Err(format!("geometry dependency at file 0x{at:x} lacks DLM ledger coverage"));}
    Ok(())
}

/// Verify both groups and banks without reading or copying an OEM payload to
/// build any output bytes. The high bit is a selector flag whose semantics are
/// not known; its value is not asserted or assigned source ownership.
pub fn verify_record_first_indices(image:&[u8])->Result<usize,String>{
    let mut count=0usize;
    for group in DIRECTORY_GROUPS {
        for bank in 0..2 {
            let expected=pointer_index_from_directory_geometry(group,bank)?;
            for entry in 0..group.records {
                let address=group.start+bank*BANK_OFFSET+entry*RECORD_BYTES+POINTER_OFFSET;
                let bytes=image.get(address..address+2)
                    .ok_or("truncated descriptor first-index halfword")?;
                let word=u16::from_le_bytes([bytes[0],bytes[1]]);
                if (word&INDEX_MASK)!=expected {
                    return Err(format!("record first-index geometry mismatch at file 0x{address:x}: index 0x{:x} != 0x{expected:x}", word&INDEX_MASK));
                }
                count+=1;
            }
        }
    }
    Ok(count)
}

/// Compare a short repeated profile signature by exact byte equality. Both
/// its original value and copy remain external/reference-dependent, not source.
pub fn verify_distant_signature_alias(image:&[u8])->Result<usize,String>{
    let source=image.get(FIRST_SIGNATURE_START..FIRST_SIGNATURE_START+SIGNATURE_BYTES)
        .ok_or("truncated original DLM profile signature")?;
    let target=image.get(REPEATED_SIGNATURE_START..REPEATED_SIGNATURE_START+SIGNATURE_BYTES)
        .ok_or("truncated repeated DLM profile signature")?;
    if let Some(idx)=source.iter().zip(target).position(|(a,b)|a!=b){
        return Err(format!("DLM profile signature differs at file 0x{:x}", REPEATED_SIGNATURE_START+idx));
    }
    Ok(SIGNATURE_BYTES)
}

pub fn assert_geometry_remains_uncredited(ledger:&ProvenanceLedger)->Result<(),String>{
    for group in DIRECTORY_GROUPS {
        for bank in 0..2 {
            let end=directory_array_end(group,bank)?;
            let start=end-group.records*RECORD_BYTES;
            for index in 0..group.records {
                let off=start+index*RECORD_BYTES+POINTER_OFFSET;
                ensure_unknown(ledger,off,off+2)?;
            }
        }
    }
    for off in [FIRST_SIGNATURE_START,REPEATED_SIGNATURE_START] {
        ensure_unknown(ledger,off,off+SIGNATURE_BYTES)?;
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    use super::super::novatek_dlm_provenance::{build_ledger,DlmClaim};

    #[test]
    fn geometry_address_and_bank_index_are_calculated_not_transcribed(){
        let expected=[[0x4ec,0x14ec],[0xd18,0x1d18]];
        for (i,group) in DIRECTORY_GROUPS.iter().copied().enumerate(){
            for bank in 0..2 {
                assert_eq!(pointer_index_from_directory_geometry(group,bank).unwrap(),expected[i][bank]);
            }
        }
        assert_eq!(DIRECTORY_GROUPS.iter().map(|x|x.records).sum::<usize>()*2,PAIRS_OF_DIRECTORY_RECORDS);
        assert!(pointer_index_from_directory_geometry(DIRECTORY_GROUPS[0],2).is_err());
        assert!(pointer_index_from_directory_geometry(DirectoryGroup{start:0,records:0},0).is_err());
    }

    #[test]
    fn consumer_index_mask_preserves_unknown_flags_and_detects_mutation(){
        let max=DIRECTORY_GROUPS.last().unwrap().start+BANK_OFFSET+4*RECORD_BYTES;
        let mut image=vec![0u8;max.max(REPEATED_SIGNATURE_START+SIGNATURE_BYTES)];
        for group in DIRECTORY_GROUPS {
            for bank in 0..2 {
                let expected=pointer_index_from_directory_geometry(group,bank).unwrap();
                for entry in 0..group.records {
                    let at=group.start+bank*BANK_OFFSET+entry*RECORD_BYTES+POINTER_OFFSET;
                    let with_flag=expected | if entry%3==2 {INDEX_FLAG} else {0};
                    image[at..at+2].copy_from_slice(&with_flag.to_le_bytes());
                }
            }
        }
        assert_eq!(verify_record_first_indices(&image).unwrap(),PAIRS_OF_DIRECTORY_RECORDS);
        image[DIRECTORY_GROUPS[1].start+POINTER_OFFSET]^=1;
        assert!(verify_record_first_indices(&image).unwrap_err().contains("0x25278"));
        assert!(verify_record_first_indices(&image[..0x22a64]).is_err());
    }

    #[test]
    fn signature_alias_is_reference_dependent_and_fail_closed(){
        let mut image=vec![0;REPEATED_SIGNATURE_START+SIGNATURE_BYTES];
        let supplied=[1,11,4,8,2,9,6,5,4,3,2,1];
        image[FIRST_SIGNATURE_START..FIRST_SIGNATURE_START+SIGNATURE_BYTES].copy_from_slice(&supplied);
        image[REPEATED_SIGNATURE_START..].copy_from_slice(&supplied);
        assert_eq!(verify_distant_signature_alias(&image).unwrap(),SIGNATURE_BYTES);
        image[REPEATED_SIGNATURE_START+3]^=1;
        assert!(verify_distant_signature_alias(&image).unwrap_err().contains("0x2df8f"));
        assert!(verify_distant_signature_alias(&image[..REPEATED_SIGNATURE_START]).is_err());
    }

    #[test]
    fn no_claims_may_promote_pointer_or_signature_source_credit(){
        let ledger=build_ledger(&[]).unwrap();
        assert_geometry_remains_uncredited(&ledger).unwrap();
        let claimed=build_ledger(&[DlmClaim{
            file_start:DIRECTORY_GROUPS[0].start+POINTER_OFFSET,
            file_end:DIRECTORY_GROUPS[0].start+POINTER_OFFSET+1,
            evidence:"invalid guessed pointer source",
        }]).unwrap();
        assert!(assert_geometry_remains_uncredited(&claimed).is_err());
        let claimed=build_ledger(&[DlmClaim{
            file_start:REPEATED_SIGNATURE_START,file_end:REPEATED_SIGNATURE_START+1,
            evidence:"invalid guessed source signature",
        }]).unwrap();
        assert!(assert_geometry_remains_uncredited(&claimed).is_err());
    }
}
