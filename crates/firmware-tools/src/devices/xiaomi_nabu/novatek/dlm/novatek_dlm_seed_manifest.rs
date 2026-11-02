// SPDX-License-Identifier: GPL-3.0-only
//! Fixed-revision SHA-256 checks for *external* Novatek DLM seed ranges.
//!
//! Only digests, offsets and provenance are stored here: **no private OEM
//! seed bytes**. This does NOT make these bytes independently reconstructed
//! source, nor does it authenticate a new firmware/signature. Exact hashes
//! intentionally lock the conditional linker to the pinned postmarketOS
//! Novatek revision; the unpinned library `assemble` remains available only
//! for synthetic unit tests and layout experiments.
use crate::wmfw_sha256;
use super::novatek_dlm_sparse::{ExternalSeed,SeedRange,SparsePlan};

pub const POSTMARKETOS_REFERENCE_SHA256:&str=
    "8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5";
pub const MANIFEST:&str=include_str!("novatek_dlm_seed_hashes.tsv");

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct PinnedSeed {
    pub range:SeedRange,
    pub sha256:String,
}

fn parse_offset(value:&str)->Result<usize,String>{
    if value.len()!=8 || !value.starts_with("0x") ||
        !value.as_bytes()[2..].iter().all(|c|c.is_ascii_hexdigit() &&
            !c.is_ascii_uppercase()) {
        return Err(format!("noncanonical DLM seed offset {value:?}"));
    }
    usize::from_str_radix(&value[2..],16).map_err(|e|format!("invalid DLM seed offset: {e}"))
}

/// Check the exact revision fingerprint, sorted disjoint span boundaries,
/// count, expected lengths and lower-case SHA-256. A wrong-revision manifest
/// must not be substituted when assembling payloads.
pub fn parse_manifest(text:&str,plan:&SparsePlan)->Result<Vec<PinnedSeed>,String>{
    let mut lines=text.lines();
    let expected_headers=[
        "# novatek-dlm-external-seeds-v1",
        "# profile postmarketos-nabu",
        "# reference-sha256 8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5",
        "# ranges 61",
        "# external-bytes 27093",
        "# start-inclusive end-exclusive sha256",
    ];
    for header in expected_headers {
        if lines.next()!=Some(header){return Err("DLM seed manifest schema/profile/reference/plan header mismatch".into());}
    }
    if plan.ranges.len()!=61 || plan.externally_required_bytes!=27093 ||
        plan.source_bytes!=9353 || plan.dependent_bytes!=13106 {
        return Err("DLM sparse plan differs from pinned manifest revision; regenerate only after source review".into());
    }
    let mut out=Vec::with_capacity(plan.ranges.len());
    for (index,range) in plan.ranges.iter().enumerate(){
        let record=lines.next().ok_or("DLM seed manifest missing range")?;
        let fields:Vec<_>=record.split('\t').collect();
        if fields.len()!=3 {return Err(format!("seed manifest range #{index} has wrong column count"));}
        let start=parse_offset(fields[0])?;
        let end=parse_offset(fields[1])?;
        if start!=range.file_start || end!=range.file_end || end<=start {
            return Err(format!("DLM seed manifest boundary mismatch at range #{index} (0x{:x}..0x{:x})",range.file_start,range.file_end));
        }
        let hex=fields[2];
        if hex.len()!=64 || !hex.as_bytes().iter().all(|c|c.is_ascii_hexdigit() && !c.is_ascii_uppercase()) {
            return Err(format!("noncanonical DLM seed SHA-256 at range #{index}"));
        }
        out.push(PinnedSeed{range:*range,sha256:hex.to_string()});
    }
    if lines.next().is_some(){return Err("DLM seed manifest contains extra rows".into());}
    Ok(out)
}

pub fn pinned_manifest(plan:&SparsePlan)->Result<Vec<PinnedSeed>,String>{
    parse_manifest(MANIFEST,plan)
}

/// Fail closed before sparse assembly or writing an output file; this checks
/// every supplied external unknown against the exact firmware revision.
pub fn verify_pinned_seeds(plan:&SparsePlan,seeds:&[ExternalSeed])->Result<(),String>{
    let expected=pinned_manifest(plan)?;
    if seeds.len()!=expected.len(){
        return Err(format!("expected {} pinned external seeds, got {}",expected.len(),seeds.len()));
    }
    for (i,(range,seed)) in expected.iter().zip(seeds).enumerate(){
        if seed.file_start!=range.range.file_start || seed.bytes.len()!=range.range.len(){
            return Err(format!("DLM pinned seed boundary/length mismatch at range #{i}, file 0x{:x}",range.range.file_start));
        }
        if wmfw_sha256::hex_digest(&seed.bytes)!=range.sha256 {
            return Err(format!("DLM pinned seed SHA-256 mismatch at range #{i}, file 0x{:x}; wrong revision, altered or incomplete external input",range.range.file_start));
        }
    }
    Ok(())
}

#[cfg(test)] mod tests{
    use super::*;
    use super::super::{novatek_dlm,novatek_dlm_gaussian,
        novatek_dlm_schedule_links,novatek_dlm_state_defaults,novatek_dlm_sparse};
    fn current()->SparsePlan{
        let mut parts=novatek_dlm::fragments().unwrap();
        parts.extend(novatek_dlm_gaussian::fragments().unwrap().into_iter().map(|f|
            novatek_dlm::DataFragment{name:"gaussian",file_start:f.file_start,
                bytes:f.bytes,evidence:f.evidence}));
        parts.extend(novatek_dlm_schedule_links::fragments().unwrap());
        parts.extend(novatek_dlm_state_defaults::fragments().unwrap());
        novatek_dlm_sparse::plan(&parts).unwrap()
    }
    #[test] fn pinned_seed_manifest_agrees_with_actual_sparse_plan(){
        let p=current();let rows=pinned_manifest(&p).unwrap();
        assert_eq!(rows.len(),61);
        assert_eq!(rows.iter().map(|r|r.range.len()).sum::<usize>(),27093);
        for (row,span) in rows.iter().zip(&p.ranges){assert_eq!(row.range,*span);}
    }
    #[test] fn wrong_profile_missing_extra_and_reordered_entries_are_rejected(){
        let p=current();
        assert!(parse_manifest(&MANIFEST.replace("postmarketos-nabu","map220v-nabu"),&p).is_err());
        assert!(parse_manifest(&MANIFEST.replace(POSTMARKETOS_REFERENCE_SHA256,&"f".repeat(64)),&p).is_err());
        let rows:Vec<_>=MANIFEST.lines().collect();
        assert!(parse_manifest(&format!("{}\n",rows[..rows.len()-1].join("\n")),&p).is_err());
        assert!(parse_manifest(&(MANIFEST.to_string()+"0x000000\t0x000001\t"+&"0".repeat(64)+"\n"),&p).is_err());
        let altered=MANIFEST.replace("0x022000\t0x022488","0x022001\t0x022488");
        assert!(parse_manifest(&altered,&p).is_err());
        let mut reversed=rows.clone();reversed.swap(6,7);
        assert!(parse_manifest(&(reversed.join("\n")+"\n"),&p).is_err());
    }
    #[test] fn invalid_digests_and_wrong_content_rejected_before_assembly(){
        let p=current();
        let original=pinned_manifest(&p).unwrap();
        let bad=MANIFEST.replace(&original[0].sha256,&"g".repeat(64));
        assert!(parse_manifest(&bad,&p).is_err());
        let bad=MANIFEST.replace(&original[0].sha256,&original[0].sha256.to_uppercase());
        assert!(parse_manifest(&bad,&p).is_err());
        let fake=p.ranges.iter().map(|range|ExternalSeed{file_start:range.file_start,
            bytes:vec![0;range.len()]}).collect::<Vec<_>>();
        assert!(verify_pinned_seeds(&p,&fake).unwrap_err().contains("SHA-256 mismatch"));
        assert!(verify_pinned_seeds(&p,&fake[1..]).is_err());
        let mut incorrect=fake.clone();incorrect[0].file_start+=1;
        assert!(verify_pinned_seeds(&p,&incorrect).is_err());
    }
}
