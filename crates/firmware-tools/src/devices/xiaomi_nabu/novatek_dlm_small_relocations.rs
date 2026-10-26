// SPDX-License-Identifier: GPL-3.0-only
//! Reference-dependent relocation of the seven mutable bytes in each of 30
//! paired 40-byte DLM descriptors. Two little-endian indices shift by
//! 0x10, 0x64 or 0xc8; a third selector byte acquires bit 2 for eight records.
//!
//! These are **observed bank relations**, not evidence of the original
//! controller configuration semantics or authorship of unknown input values.
//! The lower-bank inputs remain externally supplied. ZERO new source credit.
use super::novatek_dlm_small_records::{RECORD_START,RECORD_COUNT,RECORD_BYTES,
    BANK_DELTA,OPAQUE_START,OPAQUE_BYTES};

pub const UNIQUE_DEPENDENT_BYTES:usize=RECORD_COUNT*OPAQUE_BYTES;
const SMALL_SHIFT:u16=0x0010;
const MEDIUM_SHIFT:u16=0x0064;
const LARGE_SHIFT:u16=0x00c8;
const FLAG_BIT:u8=0x04;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct MiddleSpan {pub record:usize}
impl MiddleSpan {
    pub const fn earlier_start(self)->usize{
        RECORD_START+self.record*RECORD_BYTES+OPAQUE_START
    }
    pub const fn later_start(self)->usize{self.earlier_start()+BANK_DELTA}
    pub const fn earlier_end(self)->usize{self.earlier_start()+OPAQUE_BYTES}
    pub const fn later_end(self)->usize{self.later_start()+OPAQUE_BYTES}
}

pub fn spans()->Vec<MiddleSpan>{(0..RECORD_COUNT).map(|record|MiddleSpan{record}).collect()}

fn shift_for(record:usize)->u16{
    match record {1..=8=>SMALL_SHIFT,9|11=>MEDIUM_SHIFT,20=>0,_=>LARGE_SHIFT}
}
fn is_flagged(record:usize)->bool{(12..=19).contains(&record)}

/// Transform the seven unknown data bytes of an earlier-bank descriptor.
/// Fail closed on malformed span identities, length, or overflowing indices.
/// Note: the two u16 original indices are inputs, not independently authored.
pub fn relocate_from_external(span:MiddleSpan,primary:&[u8])->Result<[u8;OPAQUE_BYTES],String>{
    if span.record>=RECORD_COUNT{return Err("invalid 40-byte descriptor identity".into());}
    let mut middle:[u8;OPAQUE_BYTES]=primary.try_into()
        .map_err(|_|format!("DLM descriptor requires exactly {OPAQUE_BYTES} index/flag bytes"))?;
    let shift=shift_for(span.record);
    for at in [0usize,2] {
        let original=u16::from_le_bytes([middle[at],middle[at+1]]);
        let relocated=original.checked_add(shift)
            .ok_or_else(||format!("DLM bank descriptor {} index +{at} relocation overflow",span.record))?;
        middle[at..at+2].copy_from_slice(&relocated.to_le_bytes());
    }
    if is_flagged(span.record) {middle[6]|=FLAG_BIT;}
    Ok(middle)
}

/// Compare the independently computed bank relation to a privately supplied
/// exact firmware reference; the verifier neither embeds nor emits its bytes.
pub fn verify_reference_dependent(image:&[u8])->Result<usize,String>{
    let mut verified=0;
    for span in spans() {
        let first=image.get(span.earlier_start()..span.earlier_end())
            .ok_or("missing lower DLM mutable descriptor bytes")?;
        let later=image.get(span.later_start()..span.later_end())
            .ok_or("missing upper DLM mutable descriptor bytes")?;
        let expected=relocate_from_external(span,first)?;
        if let Some(i)=expected.iter().zip(later).position(|(a,b)|a!=b) {
            return Err(format!("DLM descriptor bank relocation mismatch at file 0x{:x}",span.later_start()+i));
        }
        verified+=OPAQUE_BYTES;
    }
    if verified!=UNIQUE_DEPENDENT_BYTES{return Err("incomplete 40-byte descriptor bank relocation".into());}
    Ok(verified)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn thirty_middle_spans_are_disjoint_and_uncredited() {
        let spans=spans();assert_eq!(spans.len(),30);
        assert_eq!(UNIQUE_DEPENDENT_BYTES,210);
        assert!(spans.windows(2).all(|s|s[0].earlier_end()<=s[1].earlier_start()));
        assert_eq!(spans[0].earlier_start(),RECORD_START+24);
        assert_eq!(spans[29].later_end(),RECORD_START+29*40+31+0x4000);
    }
    #[test] fn checked_class_specific_index_relocations_and_flags() {
        let original=[0x20,0x07,0x30,0x07,0xaa,0xbb,0xfa];
        for (index,shift,flag) in [(0,0xc8,0xfa),(1,0x10,0xfa),(8,0x10,0xfa),
                                  (9,0x64,0xfa),(11,0x64,0xfa),
                                  (12,0xc8,0xfe),(19,0xc8,0xfe),
                                  (20,0,0xfa),(29,0xc8,0xfa)] {
            let result=relocate_from_external(MiddleSpan{record:index},&original).unwrap();
            assert_eq!(u16::from_le_bytes([result[0],result[1]]),0x0720+shift);
            assert_eq!(u16::from_le_bytes([result[2],result[3]]),0x0730+shift);
            assert_eq!(&result[4..6],&[0xaa,0xbb]);assert_eq!(result[6],flag);
        }
        assert!(relocate_from_external(MiddleSpan{record:30},&original).is_err());
        assert!(relocate_from_external(MiddleSpan{record:3},&original[..6]).is_err());
        let overflow=[0xff,0xff,0xff,0xff,0,0,0];
        assert!(relocate_from_external(MiddleSpan{record:3},&overflow).unwrap_err().contains("overflow"));
    }
    #[test] fn mismatching_secondary_indices_or_flag_are_rejected() {
        let spans=spans();let mut image=vec![0;spans.last().unwrap().later_end()];
        for span in spans {
            let original=[2,1,9,3,4,5,if span.record==12{0xfa}else{0}];
            image[span.earlier_start()..span.earlier_end()].copy_from_slice(&original);
            let expected=relocate_from_external(span,&original).unwrap();
            image[span.later_start()..span.later_end()].copy_from_slice(&expected);
        }
        assert_eq!(verify_reference_dependent(&image).unwrap(),210);
        let bad=MiddleSpan{record:12}.later_start()+6;
        image[bad]^=4;
        assert!(verify_reference_dependent(&image).unwrap_err().contains(&format!("0x{bad:x}")));
        assert!(verify_reference_dependent(&image[..image.len()-1]).is_err());
    }
}
