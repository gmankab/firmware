// SPDX-License-Identifier: GPL-3.0-only
//! Reference-dependent 96-byte controller-record suffix copies.
//!
//! In two banks of 0x60-byte records, the first nine bytes encode fields
//! that change between banks. The remaining 87 bytes are exactly repeated
//! at file+0x4000 for 23 record pairs. Two additional first-bank records
//! are already accounted for by Agent 19's seven larger bank aliases and
//! MUST NOT be counted here. This is an externally supplied source-copy
//! relationship, *not* new independent controller algorithm reconstruction.

pub const BANK_DELTA:usize=0x4000;
pub const RECORD_SIZE:usize=0x60;
pub const EXTERNAL_HEAD_BYTES:usize=9;
pub const COPIED_SUFFIX_BYTES:usize=RECORD_SIZE-EXTERNAL_HEAD_BYTES;
pub const EARLY_RECORDS_START:usize=0x23edc;
pub const EARLY_RECORD_COUNT:usize=18;
pub const LATER_RECORDS_START:usize=0x259b4;
pub const LATER_RECORD_COUNT:usize=7;
pub const SKIPPED_EARLY_RECORDS:[usize;2]=[7,8];
pub const COPY_COUNT:usize=EARLY_RECORD_COUNT+LATER_RECORD_COUNT-SKIPPED_EARLY_RECORDS.len();
pub const UNIQUE_DEPENDENT_BYTES:usize=COPY_COUNT*COPIED_SUFFIX_BYTES;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct CopySpan{pub earlier_start:usize,pub byte_len:usize}
impl CopySpan {
    pub const fn later_start(self)->usize{self.earlier_start+BANK_DELTA}
    pub const fn earlier_end(self)->usize{self.earlier_start+self.byte_len}
    pub const fn later_end(self)->usize{self.later_start()+self.byte_len}
}

pub fn dependent_spans()->Vec<CopySpan>{
    let mut out=Vec::with_capacity(COPY_COUNT);
    for i in 0..EARLY_RECORD_COUNT {
        if SKIPPED_EARLY_RECORDS.contains(&i){continue;}
        out.push(CopySpan {earlier_start:EARLY_RECORDS_START+i*RECORD_SIZE+EXTERNAL_HEAD_BYTES,
            byte_len:COPIED_SUFFIX_BYTES});
    }
    for i in 0..LATER_RECORD_COUNT {
        out.push(CopySpan {earlier_start:LATER_RECORDS_START+i*RECORD_SIZE+EXTERNAL_HEAD_BYTES,
            byte_len:COPIED_SUFFIX_BYTES});
    }
    out
}

pub fn copy_from_supplied_primary(span:CopySpan,provided:&[u8])->Result<Vec<u8>,String>{
    if !dependent_spans().contains(&span){return Err("unknown 96-byte controller-record suffix".into());}
    if provided.len()!=COPIED_SUFFIX_BYTES {
        return Err(format!("expected {COPIED_SUFFIX_BYTES} externally supplied record bytes"));
    }
    Ok(provided.to_vec())
}

/// Verification of the *relationship only*; original reference values remain
/// private and the result never grants authored source-byte credit.
pub fn verify_reference_dependent(image:&[u8])->Result<usize,String>{
    let mut total=0;
    for span in dependent_spans() {
        let earlier=image.get(span.earlier_start..span.earlier_end())
            .ok_or("missing earlier 96-byte controller record")?;
        let later=image.get(span.later_start()..span.later_end())
            .ok_or("missing later 96-byte controller record")?;
        if let Some(i)=earlier.iter().zip(later).position(|(a,b)|a!=b){
            return Err(format!("DLM record suffix differs at file 0x{:x}",span.later_start()+i));
        }
        total+=span.byte_len;
    }
    if total!=UNIQUE_DEPENDENT_BYTES{return Err("incomplete 96-byte record suffix coverage".into());}
    Ok(total)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test]
    fn record_suffix_geometry_is_nonoverlapping_and_has_no_double_count(){
        let all=dependent_spans();
        assert_eq!(all.len(),23);
        assert_eq!(UNIQUE_DEPENDENT_BYTES,2001);
        assert_eq!(all[0].earlier_start,0x23ee5);
        assert_eq!(all[7].earlier_start,0x24245); // prior alias occupies records 7/8
        assert_eq!(all.last().unwrap().earlier_start,0x25bfd);
        assert!(all.windows(2).all(|w|w[0].later_end()<=w[1].later_start()));
        for record in [7,8] {
            assert!(!all.iter().any(|span|span.earlier_start==EARLY_RECORDS_START+record*RECORD_SIZE+9));
        }
    }
    #[test]
    fn external_copy_only_and_mismatches_fail_without_credit(){
        let all=dependent_spans();let end=all.last().unwrap().later_end();
        let mut image=vec![0;end];
        for (n,span) in all.iter().enumerate() {
            let b=(0..span.byte_len).map(|i|((i*37+n*19)%256) as u8).collect::<Vec<_>>();
            image[span.earlier_start..span.earlier_end()].copy_from_slice(&b);
            image[span.later_start()..span.later_end()].copy_from_slice(&b);
        }
        assert_eq!(verify_reference_dependent(&image).unwrap(),2001);
        let mutated=all[3].later_start()+12;
        image[mutated]^=1;
        assert!(verify_reference_dependent(&image).unwrap_err().contains(&format!("0x{mutated:x}")));
        assert!(verify_reference_dependent(&image[..end-1]).is_err());
        assert!(copy_from_supplied_primary(all[0],&[0;86]).is_err());
        assert!(copy_from_supplied_primary(CopySpan{earlier_start:0,byte_len:87},&[0;87]).is_err());
    }
}
