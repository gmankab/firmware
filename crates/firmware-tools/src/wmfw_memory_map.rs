// SPDX-License-Identifier: GPL-3.0-only
//! Cross-check the WMFW v3 declared DSP memory capacities against payload
//! offsets, keeping **DSP address holes** distinct from missing file bytes.
//!
//! XM is sparsely initialized by 5 firmware sections; YM and PM cover all
//! words declared in the target image. A DSP-memory hole has no corresponding
//! WMFW payload and is *not* permission to invent file bytes in a recompiler.

use crate::wmfw_halo_memory::HaloMemoryKind;
use crate::wmfw_metadata::build_wmfw_header;
use crate::wmfw_plan::{self,WmfwLayoutRegion};
use std::ops::Range;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct DspMemoryLayout {
    pub kind:HaloMemoryKind,
    pub capacity_words:u32,
    pub explicitly_initialized_words:u32,
    pub allocated_intervals:Vec<Range<u32>>,
    pub uninitialized_intervals:Vec<Range<u32>>,
}

pub fn calculate_layout(kind:HaloMemoryKind,capacity_words:u32,regions:&[WmfwLayoutRegion])
    -> Result<DspMemoryLayout,String> {
    if capacity_words==0 {return Err("DSP memory capacity cannot be zero".into());}
    let mut spans=Vec::new();
    for r in regions.iter().filter(|r|HaloMemoryKind::from_wmfw_type(r.kind).ok()==Some(kind)) {
        let count=r.word_count.ok_or("packed DSP region has no valid word count")?;
        let count=u32::try_from(count).map_err(|_|"DSP word count exceeds u32")?;
        let end=r.dsp_address.checked_add(count).ok_or("DSP memory range arithmetic overflow")?;
        if end>capacity_words {
            return Err(format!("{} memory extends past declared {} words at 0x{:x}",
                r.name(),capacity_words,r.dsp_address));
        }
        spans.push(r.dsp_address..end);
    }
    spans.sort_by_key(|r|r.start);
    if spans.is_empty() {return Err("no packed memory sections for declared DSP memory".into());}
    let mut holes=Vec::new();
    let mut cursor=0u32;
    let mut used=0u32;
    for region in &spans {
        if region.start<cursor {return Err(format!("DSP memory ranges overlap at address 0x{:x}",region.start));}
        if region.start>cursor {holes.push(cursor..region.start);}
        used=used.checked_add(region.end-region.start).ok_or("DSP memory used-word count overflow")?;
        cursor=region.end;
    }
    if cursor<capacity_words {holes.push(cursor..capacity_words);}
    Ok(DspMemoryLayout {kind,capacity_words,explicitly_initialized_words:used,
        allocated_intervals:spans,uninitialized_intervals:holes})
}

pub fn layouts()->Result<Vec<DspMemoryLayout>,String> {
    let metadata=build_wmfw_header();
    let sizes:Vec<u32>=metadata[12..28].chunks_exact(4)
        .map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect();
    let entries=wmfw_plan::regions()?;
    Ok(vec![
        calculate_layout(HaloMemoryKind::PackedX,sizes[0],&entries)?,
        calculate_layout(HaloMemoryKind::PackedY,sizes[1],&entries)?,
        calculate_layout(HaloMemoryKind::PackedProgram,sizes[2],&entries)?,
    ])
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]
    fn x_memory_has_four_sparse_gaps_not_file_payload_gaps() {
        let x=layouts().unwrap().remove(0);
        assert_eq!(x.capacity_words,0x5ac);
        assert_eq!(x.explicitly_initialized_words,1280);
        assert_eq!(x.allocated_intervals,vec![0..0x18,0x34..0xe4,
            0x140..0x350,0x354..0x500,0x530..0x5ac]);
        assert_eq!(x.uninitialized_intervals,vec![0x18..0x34,0xe4..0x140,
            0x350..0x354,0x500..0x530]);
        assert_eq!(x.uninitialized_intervals.iter().map(|r|r.end-r.start).sum::<u32>(),172);
    }
    #[test]
    fn pm_and_ym_fully_cover_declared_words() {
        let all=layouts().unwrap();
        assert_eq!((all[1].capacity_words,all[1].explicitly_initialized_words),(0x7fc,0x7fc));
        assert_eq!((all[2].capacity_words,all[2].explicitly_initialized_words),(0xfd8,0xfd8));
        assert!(all[1].uninitialized_intervals.is_empty());
        assert!(all[2].uninitialized_intervals.is_empty());
        assert_eq!(all[2].allocated_intervals,vec![0..0xfd8]);
    }
    #[test]
    fn overlapping_and_out_of_bounds_regions_are_rejected() {
        let mut rs=wmfw_plan::regions().unwrap();
        rs[2].dsp_address=12;
        assert!(calculate_layout(HaloMemoryKind::PackedX,0x5ac,&rs).unwrap_err().contains("overlap"));
        let mut rs=wmfw_plan::regions().unwrap();
        rs[4].dsp_address=0x600;
        assert!(calculate_layout(HaloMemoryKind::PackedX,0x5ac,&rs).unwrap_err().contains("past declared"));
        assert!(calculate_layout(HaloMemoryKind::PackedY,0,&rs).is_err());
        assert!(calculate_layout(HaloMemoryKind::PackedProgram,10,&[]).is_err());
    }
}
