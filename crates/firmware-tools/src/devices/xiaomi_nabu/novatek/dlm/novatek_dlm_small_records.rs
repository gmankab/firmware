// SPDX-License-Identifier: GPL-3.0-only
//! Reference-dependent bytes in 30 paired 40-byte descriptor records.
//!
//! A record contains 24 identical prefix bytes and nine identical tail
//! bytes when comparing the lower/upper DLM banks (file offset +0x4000).
//! The *seven intervening bytes* encode independently changing index and
//! controller flag fields. Their bank-dependent relocations are implemented
//! by the separate `novatek_dlm_small_relocations` module; this module
//! deliberately handles only the identical head/tail copies.
//! The recurrence is an exact observed relation, but original controller
//! instruction semantics for these records remain unproven: ZERO new
//! source-authored DLM credit follows from these dependent copies.

pub const RECORD_START:usize=0x233b0;
pub const RECORD_COUNT:usize=30;
pub const RECORD_BYTES:usize=0x28;
pub const BANK_DELTA:usize=0x4000;
pub const PREFIX_BYTES:usize=24;
pub const OPAQUE_START:usize=24;
pub const OPAQUE_BYTES:usize=7;
pub const TAIL_START:usize=OPAQUE_START+OPAQUE_BYTES;
pub const TAIL_BYTES:usize=RECORD_BYTES-TAIL_START;
pub const SPANS:usize=RECORD_COUNT*2;
pub const UNIQUE_DEPENDENT_BYTES:usize=RECORD_COUNT*(PREFIX_BYTES+TAIL_BYTES);

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct CopySpan {pub earlier_start:usize,pub byte_len:usize}
impl CopySpan {
    pub const fn later_start(self)->usize{self.earlier_start+BANK_DELTA}
    pub const fn earlier_end(self)->usize{self.earlier_start+self.byte_len}
    pub const fn later_end(self)->usize{self.later_start()+self.byte_len}
}

pub fn dependent_spans()->Vec<CopySpan>{
    let mut result=Vec::with_capacity(SPANS);
    for record in 0..RECORD_COUNT {
        let first=RECORD_START+record*RECORD_BYTES;
        result.push(CopySpan{earlier_start:first,byte_len:PREFIX_BYTES});
        result.push(CopySpan{earlier_start:first+TAIL_START,byte_len:TAIL_BYTES});
    }
    result
}

pub fn copy_from_supplied_primary(span:CopySpan,blob:&[u8])->Result<Vec<u8>,String>{
    if !dependent_spans().contains(&span){return Err("unrecognized 40-byte DLM record copy".into());}
    if blob.len()!=span.byte_len{return Err(format!("40-byte record alias requires {} external bytes",span.byte_len));}
    Ok(blob.to_vec())
}

pub fn verify_reference_dependent(image:&[u8])->Result<usize,String>{
    let mut bytes=0usize;
    for span in dependent_spans() {
        let earlier=image.get(span.earlier_start..span.earlier_end())
            .ok_or("missing lower 40-byte DLM record bytes")?;
        let later=image.get(span.later_start()..span.later_end())
            .ok_or("missing upper 40-byte DLM record bytes")?;
        if let Some(i)=earlier.iter().zip(later).position(|(a,b)|a!=b) {
            return Err(format!("40-byte record bank-copy mismatch at file 0x{:x}",span.later_start()+i));
        }
        bytes+=span.byte_len;
    }
    if bytes!=UNIQUE_DEPENDENT_BYTES{return Err("incomplete 40-byte DLM record verification".into());}
    Ok(bytes)
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]
    fn thirty_records_have_typed_prefix_opaque_middle_and_tail(){
        assert_eq!(RECORD_COUNT,30);
        assert_eq!(TAIL_BYTES,9);
        assert_eq!(OPAQUE_BYTES,7);
        assert_eq!(UNIQUE_DEPENDENT_BYTES,990);
        let spans=dependent_spans();assert_eq!(spans.len(),60);
        assert_eq!(spans[0].earlier_start,RECORD_START);
        assert_eq!(spans[1].earlier_start,RECORD_START+31);
        assert_eq!(spans.last().unwrap().earlier_end(),RECORD_START+RECORD_COUNT*40);
        assert!(spans.windows(2).all(|p|p[0].earlier_end()<=p[1].earlier_start));
        for record in 0..RECORD_COUNT {
            let a=spans[2*record];let b=spans[2*record+1];
            assert_eq!(b.earlier_start-a.earlier_end(),7);
        }
    }
    #[test]
    fn external_copies_are_conditional_and_corruption_is_reported(){
        let spans=dependent_spans();let mut image=vec![0;spans.last().unwrap().later_end()];
        for (n,span) in spans.iter().enumerate(){
            let part=(0..span.byte_len).map(|i|((n*13+i*37)%256) as u8).collect::<Vec<_>>();
            image[span.earlier_start..span.earlier_end()].copy_from_slice(&part);
            image[span.later_start()..span.later_end()].copy_from_slice(&part);
        }
        assert_eq!(verify_reference_dependent(&image).unwrap(),990);
        let mutated=spans[27].later_start()+2;
        image[mutated]^=0x80;
        assert!(verify_reference_dependent(&image).unwrap_err().contains(&format!("0x{mutated:x}")));
        assert!(copy_from_supplied_primary(spans[0],&[0;23]).is_err());
        assert!(copy_from_supplied_primary(CopySpan{earlier_start:0,byte_len:24},&[0;24]).is_err());
        assert!(verify_reference_dependent(&image[..image.len()-2]).is_err());
    }
}
