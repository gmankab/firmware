// SPDX-License-Identifier: GPL-3.0-only
//! Two additional lower-to-upper codec-bank staging dependencies.
//!
//! The values are not reconstructed firmware *source*: each destination is
//! copied from its still externally supplied primary bytes. These patterns
//! occur immediately before/after an older codec-bank-staging alias, with
//! intervening unmatched boundary bytes explicitly left external. Both
//! provided postmarketOS and map220v revisions satisfy the same relations.
use super::novatek_dlm_provenance::{LedgerKind,ProvenanceLedger};

pub const BANK_DELTA:usize=0x4000;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct StagingCopy {pub first:usize,pub count:usize}
impl StagingCopy {
    pub const fn first_end(self)->usize{self.first+self.count}
    pub const fn second(self)->usize{self.first+BANK_DELTA}
    pub const fn second_end(self)->usize{self.second()+self.count}
}
pub const SPANS:[StagingCopy;2]=[
    StagingCopy{first:0x23860,count:24},
    StagingCopy{first:0x23a13,count:98},
];
pub const NEW_DEPENDENT_BYTES:usize=122;

pub fn validate_geometry()->Result<(),String>{
    let mut last=0;
    for span in SPANS {
        if span.first<0x22000 || span.first<last || span.second_end()>0x2e190 ||
            span.count==0 || span.first_end()>span.second() {
            return Err("invalid additional DLM bank-staging span".into());
        }
        last=span.first_end();
    }
    if SPANS.iter().map(|s|s.count).sum::<usize>()!=NEW_DEPENDENT_BYTES {
        return Err("incorrect DLM staging extension ledger".into());
    }
    Ok(())
}

pub fn from_external_primary(span:StagingCopy,first:&[u8])->Result<Vec<u8>,String>{
    validate_geometry()?;
    if !SPANS.contains(&span) || first.len()!=span.count {
        return Err("unknown or incorrectly sized DLM staging dependency".into());
    }
    Ok(first.to_vec())
}

pub fn verify_reference_dependent(image:&[u8])->Result<usize,String>{
    validate_geometry()?;
    let mut checked=0;
    for span in SPANS {
        let primary=image.get(span.first..span.first_end()).ok_or("truncated primary staging bank")?;
        let actual=image.get(span.second()..span.second_end()).ok_or("truncated dependent staging bank")?;
        let expected=from_external_primary(span,primary)?;
        if let Some(i)=expected.iter().zip(actual).position(|(a,b)|a!=b) {
            return Err(format!("DLM staging-bank mismatch at file 0x{:x}",span.second()+i));
        }
        checked+=span.count;
    }
    Ok(checked)
}

/// Both sides of a repeated *observed* span remain unknown source. This
/// guards against accidentally promoting an exact-copy relation to authorship.
pub fn assert_uncredited(ledger:&ProvenanceLedger)->Result<(),String>{
    validate_geometry()?;
    for span in SPANS {
        for (first,end) in [(span.first,span.first_end()),(span.second(),span.second_end())] {
            let mut at=first;
            for row in &ledger.rows {
                if row.file_end<=at || row.file_start>=end{continue;}
                if row.file_start>at || row.kind!=LedgerKind::UnknownDlm {
                    return Err(format!("staging dependency is not unknown DLM source at 0x{at:x}"));
                }
                at=at.max(row.file_end.min(end));
            }
            if at!=end {return Err(format!("incomplete unknown DLM staging span at 0x{at:x}"));}
        }
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    use super::super::novatek_dlm_provenance::{build_ledger,DlmClaim};
    #[test]fn adjacent_to_existing_bank_alias_and_uncredited(){
        validate_geometry().unwrap();
        assert_eq!(SPANS[0].first_end(),0x23878);
        assert_eq!(SPANS[1].first,0x23a13);
        assert_eq!(SPANS.iter().map(|x|x.count).sum::<usize>(),122);
        let ledger=build_ledger(&[]).unwrap();
        assert_uncredited(&ledger).unwrap();
        let altered=build_ledger(&[DlmClaim{file_start:SPANS[0].first,
            file_end:SPANS[0].first+1,evidence:"false source credit"}]).unwrap();
        assert!(assert_uncredited(&altered).is_err());
    }
    #[test]fn dependent_second_bank_follows_nonzero_primary(){
        let mut image=vec![0u8;SPANS.last().unwrap().second_end()];
        for span in SPANS {
            let primary=(0..span.count).map(|i|((i*37+13)%253) as u8).collect::<Vec<_>>();
            let copied=from_external_primary(span,&primary).unwrap();
            assert_eq!(primary,copied);
            assert!(copied.iter().any(|&b|b!=0));
            image[span.first..span.first_end()].copy_from_slice(&primary);
            image[span.second()..span.second_end()].copy_from_slice(&copied);
        }
        assert_eq!(verify_reference_dependent(&image).unwrap(),122);
        image[SPANS[1].second()+22]^=1;
        assert!(verify_reference_dependent(&image).is_err());
        assert!(verify_reference_dependent(&image[..image.len()-1]).is_err());
        assert!(from_external_primary(SPANS[1],&[0;5]).is_err());
    }
}
