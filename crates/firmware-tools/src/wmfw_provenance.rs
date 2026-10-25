// SPDX-License-Identifier: GPL-3.0-only
//! Exact byte-level provenance ledger for the Cirrus WMFW v3 reconstruction.
//!
//! This prevents an imported proprietary HALO program, or zeros inferred by
//! comparing to the original, from being reported as source-rebuilt code.
//! The ledger covers each file byte exactly once. It is deterministic and
//! contains no OEM payload bytes, signing material or machine instructions.
use crate::{wmfw_compact_y,wmfw_external_payload,wmfw_plan};
use std::ops::Range;

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
pub enum ByteOrigin {
    AuthoredContainerMetadata,
    ReferenceObservedZero,
    ExternalOpaqueDsp,
}
impl ByteOrigin {
    pub fn key(self)->&'static str {
        match self {
            Self::AuthoredContainerMetadata=>"authored_metadata",
            Self::ReferenceObservedZero=>"reference_observed_zero",
            Self::ExternalOpaqueDsp=>"external_opaque_dsp",
        }
    }
}

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct ByteSpan {
    pub original_file:Range<usize>,
    pub origin:ByteOrigin,
}

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct CoverageLedger {
    pub total_bytes:usize,
    pub spans:Vec<ByteSpan>,
    pub authored_metadata_bytes:usize,
    pub reference_observed_zero_bytes:usize,
    pub external_opaque_bytes:usize,
}

fn mark(map:&mut [Option<ByteOrigin>],start:usize,length:usize,origin:ByteOrigin)->Result<(),String>{
    if length==0 {return Err("empty WMFW provenance region".into());}
    let end=start.checked_add(length).ok_or("WMFW provenance region overflow")?;
    let slice=map.get_mut(start..end).ok_or("WMFW provenance region outside file")?;
    if slice.iter().any(Option::is_some) {return Err(format!("WMFW provenance overlap at 0x{start:x}"));}
    slice.fill(Some(origin));
    Ok(())
}

pub fn build_ledger()->Result<CoverageLedger,String>{
    let mut map=vec![None;wmfw_plan::FILE_SIZE];
    let independently_generated=wmfw_plan::partial_container()?;
    for region in independently_generated.regions() {
        mark(&mut map,region.offset,region.len,ByteOrigin::AuthoredContainerMetadata)?;
    }
    let ym_byte_ranges=wmfw_compact_y::ZERO_BYTE_RANGES;
    let mut opaque_count=0;
    for region in wmfw_external_payload::OPAQUE_REGIONS {
        if region.kind!=0x12 {
            mark(&mut map,region.file_offset,region.size,ByteOrigin::ExternalOpaqueDsp)?;
            opaque_count+=region.size;
            continue;
        }
        let first=region.file_offset;
        let mut cursor=0;
        for (start_byte,end_byte) in ym_byte_ranges {
            if start_byte<cursor || end_byte<=start_byte || end_byte>region.size {
                return Err("invalid sparse Y provenance interval".into());
            }
            if start_byte>cursor {
                mark(&mut map,first+cursor,start_byte-cursor,ByteOrigin::ExternalOpaqueDsp)?;
                opaque_count+=start_byte-cursor;
            }
            mark(&mut map,first+start_byte,end_byte-start_byte,ByteOrigin::ReferenceObservedZero)?;
            cursor=end_byte;
        }
        if cursor<region.size {
            mark(&mut map,first+cursor,region.size-cursor,ByteOrigin::ExternalOpaqueDsp)?;
            opaque_count+=region.size-cursor;
        }
    }
    if let Some(hole)=map.iter().position(Option::is_none) {
        return Err(format!("unaccounted WMFW file byte at 0x{hole:x}"));
    }
    let authored=map.iter().filter(|x|**x==Some(ByteOrigin::AuthoredContainerMetadata)).count();
    let zeros=map.iter().filter(|x|**x==Some(ByteOrigin::ReferenceObservedZero)).count();
    if authored!=wmfw_plan::INDEPENDENT_METADATA_BYTES ||
        zeros!=wmfw_compact_y::GENERATED_ZERO_BYTES ||
        opaque_count!=wmfw_compact_y::EXTERNAL_SOURCE_BYTES ||
        authored+zeros+opaque_count!=map.len() {
        return Err("WMFW source/zero/external provenance byte counts differ".into());
    }
    let mut spans=Vec::new();
    let mut i=0;
    while i<map.len() {
        let kind=map[i].ok_or("unaccounted WMFW byte")?;
        let mut end=i+1;
        while end<map.len() && map[end]==Some(kind) {end+=1;}
        spans.push(ByteSpan{original_file:i..end,origin:kind});
        i=end;
    }
    Ok(CoverageLedger{total_bytes:map.len(),spans,
        authored_metadata_bytes:authored,reference_observed_zero_bytes:zeros,
        external_opaque_bytes:opaque_count})
}

impl CoverageLedger {
    /// JSON is handcrafted only from fixed numeric data and closed enum
    /// values. No vendor-derived content or user-provided strings are included.
    pub fn to_json(&self)->String{
        let mut out=format!("{{\n  \"format\": \"Cirrus HALO WMFW v3\",\n  \"reference_sha256\": \"{}\",\n  \"total_bytes\": {},\n  \"source_authored_metadata_bytes\": {},\n  \"reference_observed_zero_bytes\": {},\n  \"external_opaque_dsp_bytes\": {},\n  \"independently_decompiled_halo_instructions\": 0,\n  \"spans\": [\n",
            wmfw_external_payload::REFERENCE_SHA256,self.total_bytes,
            self.authored_metadata_bytes,self.reference_observed_zero_bytes,
            self.external_opaque_bytes);
        for (i,span) in self.spans.iter().enumerate(){
            out.push_str(&format!("    {{\"start\": {}, \"end\": {}, \"origin\": \"{}\"}}{}\n",
                span.original_file.start,span.original_file.end,span.origin.key(),
                if i+1==self.spans.len(){""}else{","}));
        }
        out.push_str("  ]\n}\n");
        out
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn exact_provenance_accounting_exposes_all_gaps_and_zero_runs(){
        let ledger=build_ledger().unwrap();
        assert_eq!(ledger.total_bytes,34236);
        assert_eq!(ledger.authored_metadata_bytes,3984);
        assert_eq!(ledger.reference_observed_zero_bytes,5748);
        assert_eq!(ledger.external_opaque_bytes,24504);
        assert_eq!(ledger.spans.first().unwrap().original_file.start,0);
        assert_eq!(ledger.spans.last().unwrap().original_file.end,34236);
        for adjacent in ledger.spans.windows(2){
            assert_eq!(adjacent[0].original_file.end,adjacent[1].original_file.start);
            assert_ne!(adjacent[0].origin,adjacent[1].origin);
        }
        let zeros=ledger.spans.iter().filter(|s|s.origin==ByteOrigin::ReferenceObservedZero)
            .map(|s|s.original_file.clone()).collect::<Vec<_>>();
        assert_eq!(zeros,vec![0xfbd..0x10ec,0x122d..0x2772]);
        assert!(ledger.spans.iter().any(|s|s.original_file==(0x278c..0x76c4) && s.origin==ByteOrigin::ExternalOpaqueDsp));
    }
    #[test]
    fn provenance_json_contains_no_vendor_bytes_or_false_source_claims(){
        let json=build_ledger().unwrap().to_json();
        assert!(json.contains("\"independently_decompiled_halo_instructions\": 0"));
        assert!(json.contains("\"source_authored_metadata_bytes\": 3984"));
        assert!(json.contains("\"reference_observed_zero_bytes\": 5748"));
        assert!(json.contains("\"external_opaque_dsp_bytes\": 24504"));
        assert!(json.ends_with("}\n"));
    }
    #[test]
    fn provenance_overlaps_and_unbounded_regions_rejected(){
        let mut map=vec![None;8];
        mark(&mut map,0,4,ByteOrigin::AuthoredContainerMetadata).unwrap();
        assert!(mark(&mut map,3,2,ByteOrigin::ExternalOpaqueDsp).unwrap_err().contains("overlap"));
        assert!(mark(&mut map,7,2,ByteOrigin::ExternalOpaqueDsp).unwrap_err().contains("outside"));
        assert!(mark(&mut map,1,0,ByteOrigin::ExternalOpaqueDsp).is_err());
    }
}
