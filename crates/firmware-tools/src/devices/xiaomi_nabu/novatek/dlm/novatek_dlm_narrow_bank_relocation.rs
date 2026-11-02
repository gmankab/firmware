// SPDX-License-Identifier: GPL-3.0-only
//! Reference-assisted relocation of two additional narrow-layout DLM spans.
//!
//! The lower bank is **externally supplied**. The upper bank can be produced
//! without supplying it twice:
//!  * a four-word descriptor index directory shifts 15-bit indices by 0x1000
//!    when mapped 0x4000 bytes higher in the firmware;
//!  * a narrow-input record window shifts two little-endian 16-bit indices
//!    by 0x10 for all but two special control records.
//!
//! All other bits are preserved, including the unproven selector/flag bits.
//! This expresses a verified inter-bank relationship, NOT independent source
//! values or knowledge of the original controller operation codes.

pub const BANK_DELTA:usize=0x4000;
pub const DIRECTORY_START:usize=0x25220;
pub const DIRECTORY_BYTES:usize=16;
pub const INPUT_WINDOW_START:usize=0x25460;
pub const INPUT_WINDOW_END:usize=0x2574c;
pub const INPUT_WINDOW_BYTES:usize=INPUT_WINDOW_END-INPUT_WINDOW_START;
pub const INDEX_RECORD_START:usize=0x254a0;
pub const INDEX_RECORD_BYTES:usize=0x28;
pub const INDEX_RECORD_COUNT:usize=18;
pub const EXEMPT_INDEX_RECORDS:[usize;2]=[8,9];
pub const DEPENDENT_BYTES:usize=DIRECTORY_BYTES+INPUT_WINDOW_BYTES;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct RelocationSpan{pub earlier_start:usize,pub bytes:usize}
impl RelocationSpan {
    pub const fn earlier_end(self)->usize{self.earlier_start+self.bytes}
    pub const fn later_start(self)->usize{self.earlier_start+BANK_DELTA}
    pub const fn later_end(self)->usize{self.later_start()+self.bytes}
}

pub const SPANS:[RelocationSpan;2]=[
    RelocationSpan{earlier_start:DIRECTORY_START,bytes:DIRECTORY_BYTES},
    RelocationSpan{earlier_start:INPUT_WINDOW_START,bytes:INPUT_WINDOW_BYTES},
];

/// Move each bounded 15-bit index to the other bank while retaining the
/// selector bit and higher configuration bits as external, unproved input.
fn shift_word_index(word:u32)->Result<u32,String>{
    let low=word&0x7fff;
    let shifted=low.checked_add(0x1000)
        .filter(|value|*value<=0x7fff)
        .ok_or("DLM narrow directory word index cannot shift bank")?;
    Ok((word & !0x7fff)|shifted)
}

pub fn relocate_from_external(span:RelocationSpan,primary:&[u8])->Result<Vec<u8>,String>{
    if !SPANS.contains(&span){return Err("unknown DLM narrow-bank relocation span".into());}
    if primary.len()!=span.bytes{return Err(format!("DLM narrow-bank relocation needs exactly {} external bytes",span.bytes));}
    let mut output=primary.to_vec();
    match span.earlier_start {
        DIRECTORY_START=>{
            for block in output.chunks_exact_mut(4) {
                let word=u32::from_le_bytes(block.try_into().unwrap());
                block.copy_from_slice(&shift_word_index(word)?.to_le_bytes());
            }
        },
        INPUT_WINDOW_START=>{
            for record in 0..INDEX_RECORD_COUNT {
                let at=(INDEX_RECORD_START-INPUT_WINDOW_START)+record*INDEX_RECORD_BYTES;
                if at+4>output.len(){return Err("narrow input record index falls outside the supplied span".into());}
                if EXEMPT_INDEX_RECORDS.contains(&record){continue;}
                for offset in [0usize,2] {
                    let word=u16::from_le_bytes([output[at+offset],output[at+offset+1]]);
                    let new=word.checked_add(0x10)
                        .ok_or_else(||format!("DLM narrow input index relocation overflow, record {record}"))?;
                    output[at+offset..at+offset+2].copy_from_slice(&new.to_le_bytes());
                }
            }
        },
        _=>unreachable!(),
    }
    Ok(output)
}

pub fn verify_reference_dependent(image:&[u8])->Result<usize,String>{
    let mut total=0;
    for span in SPANS {
        let primary=image.get(span.earlier_start..span.earlier_end())
            .ok_or("missing narrow-bank input from reference")?;
        let later=image.get(span.later_start()..span.later_end())
            .ok_or("missing narrow-bank relocation destination from reference")?;
        let generated=relocate_from_external(span,primary)?;
        if let Some(at)=generated.iter().zip(later).position(|(a,b)|a!=b) {
            return Err(format!("narrow-bank relocation differs at file 0x{:x}",span.later_start()+at));
        }
        total+=span.bytes;
    }
    if total!=DEPENDENT_BYTES {return Err("incomplete narrow-bank relocation verification".into());}
    Ok(total)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn narrow_geometry_is_disjoint_and_source_unknown(){
        assert_eq!(DEPENDENT_BYTES,764);
        assert_eq!(SPANS[0].later_end(),0x29230);
        assert_eq!(SPANS[1].later_start(),0x29460);
        assert_eq!(SPANS[1].later_end(),0x2974c);
        assert_eq!(SPANS[1].later_end(),0x2574c+BANK_DELTA);
        assert!(SPANS[0].later_end()<SPANS[1].later_start());
    }
    #[test]fn transforms_four_directory_indices_with_flags_intact(){
        let old:[u32;4]=[0x0000_0c8c,0x8000_0caf,0x0300_0cd2,0x8000_0cf5];
        let input=old.iter().flat_map(|x|x.to_le_bytes()).collect::<Vec<_>>();
        let output=relocate_from_external(SPANS[0],&input).unwrap();
        let expected=[0x0000_1c8c,0x8000_1caf,0x0300_1cd2,0x8000_1cf5];
        for (a,e) in output.chunks_exact(4).zip(expected){assert_eq!(u32::from_le_bytes(a.try_into().unwrap()),e);}
        assert!(shift_word_index(0x0000_7fff).is_err());
        assert!(shift_word_index(0x8000_7900).is_err());
        assert_eq!(shift_word_index(0x4000_0000).unwrap(),0x4000_1000);
    }
    #[test]fn sixteen_pairwise_index_shifts_preserve_two_special_records(){
        let mut primary=vec![0xa5;INPUT_WINDOW_BYTES];
        for i in 0..INDEX_RECORD_COUNT {
            let at=INDEX_RECORD_START-INPUT_WINDOW_START+i*INDEX_RECORD_BYTES;
            primary[at..at+4].copy_from_slice(&[7,0,15,0]);
        }
        let after=relocate_from_external(SPANS[1],&primary).unwrap();
        for i in 0..INDEX_RECORD_COUNT {
            let at=INDEX_RECORD_START-INPUT_WINDOW_START+i*INDEX_RECORD_BYTES;
            let expected=if EXEMPT_INDEX_RECORDS.contains(&i){[7,0,15,0]}else{[23,0,31,0]};
            assert_eq!(&after[at..at+4],&expected);
            if at+8<=after.len(){assert_eq!(&after[at+4..at+8],&primary[at+4..at+8]);}
        }
        assert_eq!(&after[..0x40],&primary[..0x40]);
        assert!(relocate_from_external(SPANS[1],&primary[..20]).is_err());
        assert!(relocate_from_external(RelocationSpan{earlier_start:0,bytes:764},&primary).is_err());
        let at=INDEX_RECORD_START-INPUT_WINDOW_START;
        primary[at..at+2].copy_from_slice(&0xfff5u16.to_le_bytes());
        assert!(relocate_from_external(SPANS[1],&primary).unwrap_err().contains("overflow"));
    }
    #[test]fn mutation_and_truncation_are_rejected(){
        let mut image=vec![0u8;SPANS[1].later_end()];
        for span in SPANS {
            let earlier=(0..span.bytes).map(|i|(i%17) as u8).collect::<Vec<_>>();
            image[span.earlier_start..span.earlier_end()].copy_from_slice(&earlier);
            let generated=relocate_from_external(span,&earlier).unwrap();
            image[span.later_start()..span.later_end()].copy_from_slice(&generated);
        }
        assert_eq!(verify_reference_dependent(&image).unwrap(),DEPENDENT_BYTES);
        image[SPANS[1].later_start()+0x41]^=1;
        assert!(verify_reference_dependent(&image).unwrap_err().contains(&format!("0x{:x}",SPANS[1].later_start()+0x41)));
        assert!(verify_reference_dependent(&image[..SPANS[1].later_end()-1]).is_err());
    }
}
