// SPDX-License-Identifier: GPL-3.0-only
//! Seven exact lower-to-upper DLM bank dependencies (REFERENCE-ASSISTED).
//!
//! Unknown earlier-bank bytes must be passed in by a caller. These aliases
//! do not independently reconstruct any sample or calibration value. A
//! reference-assisted exact re-assembly is not a source-reconstructed image.
use super::novatek_dlm_provenance::{LedgerKind,ProvenanceLedger};

pub const BANK_DELTA:usize=0x4000;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct BankAlias {
    pub name:&'static str,
    pub earlier_start:usize,
    pub byte_len:usize,
}
impl BankAlias {
    pub const fn later_start(self)->usize{ self.earlier_start+BANK_DELTA }
    pub const fn earlier_end(self)->usize{ self.earlier_start+self.byte_len }
    pub const fn later_end(self)->usize{ self.later_start()+self.byte_len }
}

/// These source and destination spans were found to be identical in the
/// SHA-256-pinned postmarketOS reference. Neither is independently authored.
/// Other large equality runs overlap previously generated schedules or
/// covered descriptor-bank transforms and are deliberately omitted.
pub const ALIASES:[BankAlias;7]=[
    BankAlias{name:"codec-bank-staging",earlier_start:0x2387c,byte_len:406},
    BankAlias{name:"format-index-scratch",earlier_start:0x23df8,byte_len:228},
    BankAlias{name:"frame-bank-defaults",earlier_start:0x24186,byte_len:182},
    BankAlias{name:"pre-wide-schedule",earlier_start:0x245a6,byte_len:862},
    BankAlias{name:"post-wide-schedule",earlier_start:0x24fc4,byte_len:604},
    BankAlias{name:"input-ring-metadata",earlier_start:0x2574c,byte_len:616},
    BankAlias{name:"pre-narrow-schedule",earlier_start:0x25c5e,byte_len:238},
];
pub const DEPENDENT_BYTES:usize=3136;

pub fn validate_alias_geometry()->Result<(),String>{
    let mut prev_end=0usize;
    for entry in ALIASES {
        if entry.byte_len==0 || entry.earlier_start<0x22000 || entry.later_end()>0x2e190
            || entry.earlier_end()>entry.later_start() {
            return Err(format!("invalid DLM bank alias {}",entry.name));
        }
        if entry.earlier_start<prev_end {
            return Err(format!("overlapping DLM bank aliases at 0x{:x}",entry.earlier_start));
        }
        prev_end=entry.earlier_end();
    }
    if ALIASES.iter().map(|a|a.byte_len).sum::<usize>()!=DEPENDENT_BYTES {
        return Err("incorrect DLM bank alias byte ledger".into());
    }
    Ok(())
}

/// Checked destination generated from an EXTERNALLY SUPPLIED primary block.
/// This must not be used to award authored data coverage.
pub fn duplicate_from_external_primary(alias:BankAlias,primary:&[u8])->Result<Vec<u8>,String>{
    validate_alias_geometry()?;
    if !ALIASES.contains(&alias){return Err("unknown DLM bank alias descriptor".into());}
    if primary.len()!=alias.byte_len{
        return Err(format!("{} requires {} external bytes",alias.name,alias.byte_len));
    }
    Ok(primary.to_vec())
}

/// Reject overlays that silently attempt to mark either dependent or primary
/// unknown image bytes as source-owned data.
pub fn assert_uncredited_on_ledger(ledger:&ProvenanceLedger)->Result<(),String>{
    validate_alias_geometry()?;
    for alias in ALIASES {
        for (start,end) in [(alias.earlier_start,alias.earlier_end()),
                            (alias.later_start(),alias.later_end())] {
            let mut at=start;
            for row in &ledger.rows {
                if row.file_end<=at || row.file_start>=end {continue;}
                if row.file_start>at || row.kind!=LedgerKind::UnknownDlm {
                    return Err(format!("bank alias {} overlaps source-owned DLM at 0x{at:x}",alias.name));
                }
                at=at.max(row.file_end.min(end));
            }
            if at!=end{
                return Err(format!("bank alias {} missing unknown DLM coverage",alias.name));
            }
        }
    }
    Ok(())
}

/// Verify a dependency using private caller-supplied original input only.
/// An exact match does not mean the input was authored from reconstructed code.
pub fn verify_reference_assisted_aliases(reference:&[u8])->Result<usize,String>{
    validate_alias_geometry()?;
    let mut total=0;
    for alias in ALIASES {
        let src=reference.get(alias.earlier_start..alias.earlier_end())
            .ok_or("truncated primary DLM bank alias")?;
        let later=reference.get(alias.later_start()..alias.later_end())
            .ok_or("truncated upper DLM bank alias")?;
        let expected=duplicate_from_external_primary(alias,src)?;
        if let Some(mismatch)=expected.iter().zip(later).position(|(a,b)|a!=b){
            return Err(format!("DLM bank alias {} mismatch at file 0x{:x}",
                alias.name,alias.later_start()+mismatch));
        }
        total+=expected.len();
    }
    Ok(total)
}

#[cfg(test)]mod tests {
    use super::*;
    use super::super::novatek_dlm_provenance::{build_ledger,DlmClaim};

    #[test]
    fn seven_bank_aliases_are_disjoint_exact_and_uncredited(){
        validate_alias_geometry().unwrap();
        assert_eq!(DEPENDENT_BYTES,3136);
        let ledger=build_ledger(&[]).unwrap();
        assert_uncredited_on_ledger(&ledger).unwrap();
        let mut image=vec![0;ALIASES.last().unwrap().later_end()];
        for alias in ALIASES {
            let first=(0..alias.byte_len).map(|i|((i*47+alias.earlier_start)%256) as u8).collect::<Vec<_>>();
            let second=duplicate_from_external_primary(alias,&first).unwrap();
            image[alias.earlier_start..alias.earlier_end()].copy_from_slice(&first);
            image[alias.later_start()..alias.later_end()].copy_from_slice(&second);
        }
        assert_eq!(verify_reference_assisted_aliases(&image).unwrap(),DEPENDENT_BYTES);
        image[ALIASES[2].later_start()+79]^=1;
        assert!(verify_reference_assisted_aliases(&image).unwrap_err().contains("0x281d5"));
        assert!(verify_reference_assisted_aliases(&image[..image.len()-1]).is_err());
    }

    #[test]
    fn dependent_copy_requires_exact_input_and_ledger_excludes_source_credit(){
        let alias=ALIASES[0];
        assert!(duplicate_from_external_primary(alias,&[0;405]).is_err());
        assert!(duplicate_from_external_primary(BankAlias{name:"fake",earlier_start:0x2387c,byte_len:406},&[0;406]).is_err());
        let mut claim=Vec::new();
        claim.push(DlmClaim{file_start:alias.earlier_start,file_end:alias.earlier_start+1,
            evidence:"tampered source credit"});
        let bad=build_ledger(&claim).unwrap();
        assert!(assert_uncredited_on_ledger(&bad).is_err());
        claim[0].file_start=alias.later_start();claim[0].file_end=alias.later_start()+1;
        let bad=build_ledger(&claim).unwrap();
        assert!(assert_uncredited_on_ledger(&bad).is_err());
    }
}
