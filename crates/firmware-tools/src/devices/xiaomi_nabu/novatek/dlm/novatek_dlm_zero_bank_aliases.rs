// SPDX-License-Identifier: GPL-3.0-only
//! Optional externally-dependent copy of three DLM bank windows observed
//! identically zero in both known postmarketOS and map220v Novatek revisions.
//!
//! This does NOT independently establish that the original source declares
//! these buffers zero-initialized, nor their runtime semantics. In particular
//! these positions must not be silently filled with zero. The linker requires
//! all 1,812+336+395 bytes (directly or through earlier dependencies) from
//! unknown lower-bank input before reproducing them in the later bank.
//!
//! The last span's primary bytes themselves include the destinations of the
//! first two spans: this is a checked **ordered dependency chain**, not a
//! circular recipe and not new independently authored source.

pub const BANK_DELTA:usize=0x4000;
pub const ALIAS_BYTES:usize=336+395+1812;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Alias{pub source:usize,pub byte_len:usize}
impl Alias {
    pub const fn source_end(self)->usize {self.source+self.byte_len}
    pub const fn destination(self)->usize {self.source+BANK_DELTA}
    pub const fn destination_end(self)->usize {self.destination()+self.byte_len}
}

/// Topological order: the last input overlaps the first two outputs.
pub const ALIASES:[Alias;3]=[
    Alias{source:0x2271c,byte_len:336},
    Alias{source:0x22875,byte_len:395},
    Alias{source:0x262ec,byte_len:1812},
];

pub fn from_external_primary(alias:Alias,primary:&[u8])->Result<Vec<u8>,String>{
    if !ALIASES.contains(&alias) {return Err("unknown zero-window bank alias".into());}
    if primary.len()!=alias.byte_len {
        return Err(format!("zero-window alias needs {} independently supplied or covered bytes",alias.byte_len));
    }
    // Do not manufacture zeroes: copying externally provided nonzero data is
    // valid for a synthetic proof of dependency, but only an exact pinned
    // firmware comparison can establish this relation for a real revision.
    Ok(primary.to_vec())
}

pub fn verify_reference_dependent(image:&[u8])->Result<usize,String>{
    let mut checked=0;
    for alias in ALIASES {
        let first=image.get(alias.source..alias.source_end())
            .ok_or("missing bank window primary bytes")?;
        let second=image.get(alias.destination()..alias.destination_end())
            .ok_or("missing bank window dependent bytes")?;
        if let Some(delta)=first.iter().zip(second).position(|(a,b)|a!=b){
            return Err(format!("bank window copy differs at file 0x{:x}",alias.destination()+delta));
        }
        checked+=alias.byte_len;
    }
    if checked!=ALIAS_BYTES {return Err("incomplete zero-tail bank copy".into());}
    Ok(checked)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn ranges_disjoint_and_topologically_ordered(){
        assert_eq!(ALIAS_BYTES,2543);
        assert_eq!(ALIASES[0].destination(),0x2671c);
        assert_eq!(ALIASES[1].destination_end(),0x26a00);
        assert_eq!(ALIASES[2].source,0x262ec);
        assert_eq!(ALIASES[2].destination_end(),0x2aa00);
        assert!(ALIASES[0].destination_end()<=ALIASES[1].destination());
        assert!(ALIASES[1].destination_end()<=ALIASES[2].destination());
        for (i,span) in ALIASES.iter().enumerate(){
            assert!(span.source<span.destination());
            for following in ALIASES.iter().skip(i+1){
                assert!(span.destination_end()<=following.destination());
            }
        }
    }
    #[test]fn cannot_fabricate_unknown_source_as_zero(){
        let alias=ALIASES[0];
        let mut input=(0..alias.byte_len).map(|i|((i*13+7)%256) as u8).collect::<Vec<_>>();
        let result=from_external_primary(alias,&input).unwrap();
        assert_eq!(result,input);assert!(result.iter().any(|&b|b!=0));
        input[0]^=0xaa;
        assert_eq!(from_external_primary(alias,&input).unwrap()[0],input[0]);
        assert!(from_external_primary(alias,&input[..8]).is_err());
        assert!(from_external_primary(Alias{source:0,byte_len:336},&input).is_err());
    }
    #[test]fn rejects_missing_modified_or_truncated_second_bank(){
        let mut image=vec![0;ALIASES.last().unwrap().destination_end()];
        let mut written=vec![false;image.len()];
        for alias in ALIASES {
            // Use synthetic nonzero earlier-bank contents. Never overwrite a
            // previously copied dependency when it becomes a later primary.
            for i in alias.source..alias.source_end() {
                if !written[i] {image[i]=(i.wrapping_mul(37)%251) as u8;written[i]=true;}
            }
            let primary=image[alias.source..alias.source_end()].to_vec();
            image[alias.destination()..alias.destination_end()].copy_from_slice(&primary);
            for i in alias.destination()..alias.destination_end(){written[i]=true;}
        }
        assert_eq!(verify_reference_dependent(&image).unwrap(),ALIAS_BYTES);
        image[ALIASES[2].destination()+44]^=0x80;
        assert!(verify_reference_dependent(&image).unwrap_err().contains(&format!("0x{:x}",ALIASES[2].destination()+44)));
        assert!(verify_reference_dependent(&image[..image.len()-1]).is_err());
    }
}
