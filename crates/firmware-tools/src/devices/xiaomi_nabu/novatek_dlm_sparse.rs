// SPDX-License-Identifier: GPL-3.0-only
//! Sparse Novatek DLM linker: separately authored data, externally required
//! seed ranges, and strictly *dependent* copies/relocations.
//!
//! This module is deliberately NOT a way to claim independent firmware
//! authorship. It can rebuild a byte-identical DLM *conditional on* the
//! external primary bytes. A complete source-only build still has holes.
//! The OEM reference is never consulted by the plan or reconstruction code.
use super::{novatek_dlm::DataFragment, novatek_dlm_bank_aliases,
    novatek_dlm_bank_transform, novatek_dlm_calibration_alias,
    novatek_dlm_descriptor_geometry, novatek_dlm_record_suffix,
    novatek_dlm_small_records,
    novatek_layout::{DLM_FILE_START, DLM_FILE_END}};

pub const DLM_BYTES:usize=DLM_FILE_END-DLM_FILE_START;
const SOURCE:u8=1;
const DEPENDENT:u8=2;
const EXTERNAL:u8=3;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct SeedRange {pub file_start:usize,pub file_end:usize}
impl SeedRange {pub fn len(self)->usize {self.file_end-self.file_start}}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ExternalSeed {pub file_start:usize,pub bytes:Vec<u8>}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct SparsePlan {
    pub ranges:Vec<SeedRange>,
    pub source_bytes:usize,
    pub dependent_bytes:usize,
    pub externally_required_bytes:usize,
    // Source data inside the transform output is cross-checked, not counted
    // twice as reference-dependent bytes.
    pub source_and_dependent_crosschecks:usize,
}

fn index(file_offset:usize,len:usize)->Result<std::ops::Range<usize>,String>{
    if file_offset<DLM_FILE_START || file_offset.checked_add(len).is_none_or(|end|end>DLM_FILE_END) {
        return Err(format!("DLM span at file 0x{file_offset:x} length {len} exceeds 0x{DLM_FILE_START:x}..0x{DLM_FILE_END:x}"));
    }
    let at=file_offset-DLM_FILE_START;
    Ok(at..at+len)
}

fn derived_destinations()->Vec<SeedRange>{
    let mut spans=Vec::new();
    for (_,later,n) in novatek_dlm_bank_transform::PAIRED_GROUPS {
        spans.push(SeedRange{file_start:later,file_end:later+n*novatek_dlm_bank_transform::DESCRIPTOR_BYTES});
    }
    for alias in novatek_dlm_bank_aliases::ALIASES {
        spans.push(SeedRange{file_start:alias.later_start(),file_end:alias.later_end()});
    }
    for span in novatek_dlm_record_suffix::dependent_spans() {
        spans.push(SeedRange{file_start:span.later_start(),file_end:span.later_end()});
    }
    for span in novatek_dlm_small_records::dependent_spans() {
        spans.push(SeedRange{file_start:span.later_start(),file_end:span.later_end()});
    }
    spans.push(SeedRange{file_start:novatek_dlm_calibration_alias::MIRROR_FILE_START,
        file_end:novatek_dlm_calibration_alias::MIRROR_FILE_END});
    spans.push(SeedRange{file_start:novatek_dlm_descriptor_geometry::REPEATED_SIGNATURE_START,
        file_end:novatek_dlm_descriptor_geometry::REPEATED_SIGNATURE_START+
            novatek_dlm_descriptor_geometry::SIGNATURE_BYTES});
    spans
}

pub fn plan(parts:&[DataFragment])->Result<SparsePlan,String>{
    let mut ownership=vec![0u8;DLM_BYTES];
    let mut authored=0usize;
    for part in parts {
        if part.bytes.is_empty(){return Err("empty source fragment".into());}
        for i in index(part.file_start,part.bytes.len())? {
            if ownership[i]!=0{return Err(format!("duplicate authored DLM byte at file 0x{:x}",DLM_FILE_START+i));}
            ownership[i]=SOURCE;authored+=1;
        }
    }
    let mut dependent=0usize;
    let mut crosschecked=0usize;
    for span in derived_destinations() {
        for i in index(span.file_start,span.len())? {
            match ownership[i] {
                0=>{ownership[i]=DEPENDENT;dependent+=1;},
                SOURCE=>{crosschecked+=1;},
                DEPENDENT=>return Err(format!("overlapping DLM dependency at file 0x{:x}",DLM_FILE_START+i)),
                _=>unreachable!(),
            }
        }
    }
    let mut ranges=Vec::new();let mut p=0usize;
    while p<DLM_BYTES {
        if ownership[p]!=0 {p+=1;continue;}
        let start=p;
        while p<DLM_BYTES&&ownership[p]==0 {ownership[p]=EXTERNAL;p+=1;}
        ranges.push(SeedRange{file_start:DLM_FILE_START+start,file_end:DLM_FILE_START+p});
    }
    let required=ranges.iter().map(|x|x.len()).sum::<usize>();
    if authored+dependent+required!=DLM_BYTES{return Err("incomplete DLM sparse coverage".into());}
    Ok(SparsePlan{ranges,source_bytes:authored,dependent_bytes:dependent,
        externally_required_bytes:required,source_and_dependent_crosschecks:crosschecked})
}

fn put_source(dst:&mut[u8],state:&mut[u8],part:&DataFragment)->Result<(),String>{
    for (i,&b) in index(part.file_start,part.bytes.len())?.zip(&part.bytes) {
        if state[i]!=0{return Err(format!("overlapping source at file 0x{:x}",DLM_FILE_START+i));}
        dst[i]=b;state[i]=SOURCE;
    }
    Ok(())
}

fn put_derived(dst:&mut[u8],state:&mut[u8],file_start:usize,blob:&[u8])->Result<(),String>{
    for (i,&b) in index(file_start,blob.len())?.zip(blob) {
        match state[i]{
            0=>{dst[i]=b;state[i]=DEPENDENT;},
            SOURCE if dst[i]==b=>{},
            SOURCE=>return Err(format!("derived value conflicts with independently sourced DLM at file 0x{:x}",DLM_FILE_START+i)),
            _=>return Err(format!("overlapping derived or external DLM at file 0x{:x}",DLM_FILE_START+i)),
        }
    }
    Ok(())
}

fn read_covered(dst:&[u8],state:&[u8],start:usize,end:usize)->Result<Vec<u8>,String>{
    let at=index(start,end.checked_sub(start).ok_or("reversed DLM dependency")?)?;
    for n in at.clone(){if state[n]==0{return Err(format!("unpopulated DLM dependency at file 0x{:x}",n+DLM_FILE_START));}}
    Ok(dst[at].to_vec())
}

/// Rebuild the complete 49,552-byte DLM from independently generated parts
/// and the **minimal** external seed ranges. No source-reference fallback;
/// missing, reordered and oversized seeds fail instead of becoming padding.
/// Dependent destinations are generated and their overlap with authored
/// bytes is explicitly checked. The return value is a DLM slice, NOT a
/// flashable firmware or a newly signed NVT file.
pub fn assemble(parts:&[DataFragment],seeds:&[ExternalSeed])->Result<Vec<u8>,String>{
    let plan=plan(parts)?;
    if seeds.len()!=plan.ranges.len(){return Err(format!("expected {} external DLM seed ranges, got {}",plan.ranges.len(),seeds.len()));}
    let mut dst=vec![0u8;DLM_BYTES];let mut state=vec![0u8;DLM_BYTES];
    for part in parts {put_source(&mut dst,&mut state,part)?;}
    for (expected,provided) in plan.ranges.iter().zip(seeds){
        if expected.file_start!=provided.file_start || expected.len()!=provided.bytes.len() {
            return Err(format!("external DLM seed boundary or length mismatch at file 0x{:x}",expected.file_start));
        }
        for (i,&b) in index(provided.file_start,provided.bytes.len())?.zip(&provided.bytes){
            if state[i]!=0 {return Err("seed overlaps source or another seed".into());}
            dst[i]=b;state[i]=EXTERNAL;
        }
    }
    for (first,later,count) in novatek_dlm_bank_transform::PAIRED_GROUPS {
        let length=count*novatek_dlm_bank_transform::DESCRIPTOR_BYTES;
        let primary=read_covered(&dst,&state,first,first+length)?;
        let rebuilt=novatek_dlm_bank_transform::translate_records_from_external(&primary)?;
        put_derived(&mut dst,&mut state,later,&rebuilt)?;
    }
    for alias in novatek_dlm_bank_aliases::ALIASES {
        let primary=read_covered(&dst,&state,alias.earlier_start,alias.earlier_end())?;
        let rebuilt=novatek_dlm_bank_aliases::duplicate_from_external_primary(alias,&primary)?;
        put_derived(&mut dst,&mut state,alias.later_start(),&rebuilt)?;
    }
    for span in novatek_dlm_record_suffix::dependent_spans() {
        let primary=read_covered(&dst,&state,span.earlier_start,span.earlier_end())?;
        let rebuilt=novatek_dlm_record_suffix::copy_from_supplied_primary(span,&primary)?;
        put_derived(&mut dst,&mut state,span.later_start(),&rebuilt)?;
    }
    for span in novatek_dlm_small_records::dependent_spans() {
        let primary=read_covered(&dst,&state,span.earlier_start,span.earlier_end())?;
        let rebuilt=novatek_dlm_small_records::copy_from_supplied_primary(span,&primary)?;
        put_derived(&mut dst,&mut state,span.later_start(),&rebuilt)?;
    }
    {
        use novatek_dlm_calibration_alias as profile;
        let primary=read_covered(&dst,&state,profile::PRIMARY_FILE_START,profile::PRIMARY_FILE_END)?;
        let rebuilt=profile::duplicate_from_external_primary(&primary)?;
        put_derived(&mut dst,&mut state,profile::MIRROR_FILE_START,&rebuilt)?;
    }
    {
        use novatek_dlm_descriptor_geometry as geometry;
        let first=read_covered(&dst,&state,geometry::FIRST_SIGNATURE_START,
            geometry::FIRST_SIGNATURE_START+geometry::SIGNATURE_BYTES)?;
        put_derived(&mut dst,&mut state,geometry::REPEATED_SIGNATURE_START,&first)?;
    }
    if let Some(i)=state.iter().position(|&flag|flag==0) {
        return Err(format!("unpopulated DLM file byte 0x{:x}",DLM_FILE_START+i));
    }
    Ok(dst)
}

/// PRIVATE REFERENCE HELPER: extract only those sparse seed ranges whose
/// source is unknown, with no copies, 0-padding or dependent targets. Never
/// write this output into Git/bundles or mark it as reconstructed source.
pub fn seeds_from_private_reference(image:&[u8],plan:&SparsePlan)->Result<Vec<ExternalSeed>,String>{
    if image.len()!=super::novatek_layout::IMAGE_LENGTH {return Err("expected complete externally supplied Novatek image".into());}
    let mut result=Vec::with_capacity(plan.ranges.len());
    for range in &plan.ranges {
        result.push(ExternalSeed{file_start:range.file_start,
            bytes:image.get(range.file_start..range.file_end)
                .ok_or("reference missing seed range")?.to_vec()});
    }
    Ok(result)
}

#[cfg(test)] mod tests {
    use super::*;
    use super::super::{novatek_dlm,novatek_dlm_gaussian,
        novatek_dlm_schedule_links,novatek_dlm_state_defaults};
    fn authored()->Vec<DataFragment>{
        let mut parts=novatek_dlm::fragments().unwrap();
        parts.extend(novatek_dlm_gaussian::fragments().unwrap().into_iter().map(|f|DataFragment{
            name:"kernel",file_start:f.file_start,bytes:f.bytes,evidence:f.evidence}));
        parts.extend(novatek_dlm_schedule_links::fragments().unwrap());
        parts.extend(novatek_dlm_state_defaults::fragments().unwrap());
        parts.sort_by_key(|p|p.file_start);parts
    }
    fn fake_seeds(plan:&SparsePlan)->Vec<ExternalSeed>{
        plan.ranges.iter().map(|r|ExternalSeed{file_start:r.file_start,
            bytes:vec![0;r.len()]}).collect()
    }
    #[test]
    fn exact_mutually_exclusive_source_external_and_derived_plan(){
        let p=plan(&authored()).unwrap();
        assert_eq!(p.source_bytes,9353);
        assert_eq!(p.source_and_dependent_crosschecks,40);
        assert_eq!(p.dependent_bytes,9467);
        assert_eq!(p.externally_required_bytes,30732);
        assert_eq!(p.source_bytes+p.dependent_bytes+p.externally_required_bytes,DLM_BYTES);
        for pair in p.ranges.windows(2){assert!(pair[0].file_end<pair[1].file_start);}
        assert_eq!(p.ranges.first().unwrap().file_start,DLM_FILE_START);
    }
    #[test]
    fn minimal_seeds_can_rebuild_synthetic_not_oem_data(){
        let parts=authored();let plan=plan(&parts).unwrap();
        let seeds=fake_seeds(&plan);
        let output=assemble(&parts,&seeds).unwrap();
        assert_eq!(output.len(),DLM_BYTES);
        for part in &parts {
            assert_eq!(&output[index(part.file_start,part.bytes.len()).unwrap()],part.bytes);
        }
        // All dependent relationships must hold for this synthetic input.
        let mut image=vec![0;DLM_FILE_END];
        image[DLM_FILE_START..DLM_FILE_END].copy_from_slice(&output);
        assert_eq!(novatek_dlm_bank_transform::verify_against_supplied_reference(&image).unwrap(),2940);
        assert_eq!(novatek_dlm_bank_aliases::verify_reference_assisted_aliases(&image).unwrap(),3136);
        assert_eq!(novatek_dlm_record_suffix::verify_reference_dependent(&image).unwrap(),2001);
        assert_eq!(novatek_dlm_small_records::verify_reference_dependent(&image).unwrap(),990);
        assert_eq!(novatek_dlm_calibration_alias::verify_reference_assisted_copy(&image).unwrap(),428);
        assert_eq!(novatek_dlm_descriptor_geometry::verify_distant_signature_alias(&image).unwrap(),12);
    }
    #[test]
    fn opaque_seven_byte_middle_is_not_silently_derived() {
        let p=plan(&authored()).unwrap();
        for record in 0..novatek_dlm_small_records::RECORD_COUNT {
            let middle=novatek_dlm_small_records::RECORD_START
                +novatek_dlm_small_records::BANK_DELTA
                +record*novatek_dlm_small_records::RECORD_BYTES
                +novatek_dlm_small_records::OPAQUE_START;
            // Every byte, including source-equal coincidences, remains
            // externally required until those fields acquire semantics.
            for at in middle..middle+novatek_dlm_small_records::OPAQUE_BYTES {
                assert!(p.ranges.iter().any(|r|r.file_start<=at&&at<r.file_end),
                    "unproven 40-byte record middle unexpectedly recovered at 0x{at:x}");
            }
        }
    }
    #[test]
    fn rejects_missing_overlong_reordered_or_repeated_external_seeds(){
        let parts=authored();let p=plan(&parts).unwrap();
        let mut seeds=fake_seeds(&p);
        assert!(assemble(&parts,&seeds[1..]).is_err());
        seeds[0].bytes.push(0);
        assert!(assemble(&parts,&seeds).is_err());
        seeds[0].bytes.pop();
        seeds.swap(0,1);assert!(assemble(&parts,&seeds).is_err());
        seeds.swap(0,1);seeds[0].file_start=p.ranges[1].file_start;
        assert!(assemble(&parts,&seeds).is_err());
    }
    #[test]
    fn rejects_a_primary_index_that_cannot_relocate(){
        let parts=authored();let p=plan(&parts).unwrap();let mut seeds=fake_seeds(&p);
        let addr=novatek_dlm_bank_transform::FIRST_GROUP_EARLIER+0x48;
        let seed=seeds.iter_mut().find(|seed|
            seed.file_start<=addr && addr+2<=seed.file_start+seed.bytes.len()).unwrap();
        let i=addr-seed.file_start;seed.bytes[i..i+2].copy_from_slice(&0x7fff_u16.to_le_bytes());
        assert!(assemble(&parts,&seeds).unwrap_err().contains("cannot shift banks"));
    }
    #[test]
    fn authored_source_must_not_disagree_with_dependent_transform(){
        let mut parts=authored();
        let target=novatek_dlm_bank_transform::FIRST_GROUP_LATER+0x50;
        let edited=parts.iter_mut().find(|p|p.file_start==target).unwrap();
        edited.bytes[0]^=1;
        let p=plan(&parts).unwrap();
        assert!(assemble(&parts,&fake_seeds(&p)).unwrap_err().contains("conflicts with independently sourced"));
    }
    #[test]
    fn refuses_duplicate_source_and_out_of_range_input(){
        let mut parts=authored();let extra=parts[0].clone();parts.push(extra);
        assert!(plan(&parts).unwrap_err().contains("duplicate authored"));
        let mut parts=authored();parts[0].file_start=DLM_FILE_START-1;
        assert!(plan(&parts).unwrap_err().contains("exceeds"));
    }
}
