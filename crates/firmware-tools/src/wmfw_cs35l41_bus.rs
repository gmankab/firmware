// SPDX-License-Identifier: GPL-3.0-only
//! Checked CS35L41-specific AHB/regmap destination ranges for HALO packed
//! program/X/Y memory. Public constants match Linux `include/sound/cs35l41.h`.
//! This maps where *externally supplied* payload bytes would be loaded; it
//! does not independently rebuild or execute those proprietary DSP bytes.

use crate::wmfw_halo_memory::HaloMemoryKind;
use crate::wmfw_plan::{self,WmfwLayoutRegion};
use std::ops::Range;

pub const XM_PACKED_BASE:u32=0x0200_0000;
pub const YM_PACKED_BASE:u32=0x02c0_0000;
pub const PM_PACKED_BASE:u32=0x0380_0000;
// Last 32-bit memory-mapped register address inclusive, from Linux CS35L41.
pub const XM_PACKED_LAST:u32=0x0200_2ff0;
pub const YM_PACKED_LAST:u32=0x02c0_17f0;
pub const PM_PACKED_LAST:u32=0x0380_4fe8;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct BusTransfer {
    pub memory:HaloMemoryKind,
    pub original_file_span:Range<usize>,
    pub dsp_word_span:Range<u32>,
    /// AHB byte-address span [start,end). Not an executable instruction map.
    pub ahb_register_span:Range<u32>,
}

fn for_kind(kind:HaloMemoryKind)->(u32,u32){
    match kind {
        HaloMemoryKind::PackedX=>(XM_PACKED_BASE,XM_PACKED_LAST),
        HaloMemoryKind::PackedY=>(YM_PACKED_BASE,YM_PACKED_LAST),
        HaloMemoryKind::PackedProgram=>(PM_PACKED_BASE,PM_PACKED_LAST),
    }
}

pub fn transfer_for_region(r:&WmfwLayoutRegion)->Result<BusTransfer,String>{
    let kind=HaloMemoryKind::from_wmfw_type(r.kind)?;
    let word_count=r.word_count.ok_or("invalid WMFW packed payload word count")?;
    let word_count=u32::try_from(word_count).map_err(|_|"too many DSP memory words")?;
    let end_word=r.dsp_address.checked_add(word_count).ok_or("DSP word address overflow")?;
    let (base,last_reg)=for_kind(kind);
    let start=kind.mapped_register(base,r.dsp_address)?;
    let end=base.checked_add(end_word.checked_mul(kind.bytes_per_word() as u32)
        .ok_or("packed DSP memory range overflow")?).ok_or("packed DSP bus address overflow")?;
    if start & 3 != 0 || end & 3 != 0 || r.payload_length & 3 != 0 {
        return Err(format!("WMFW region @0x{:x} is not 32-bit AHB-aligned",r.payload_offset));
    }
    if end>last_reg.checked_add(4).ok_or("register-end overflow")? || start>=end {
        return Err(format!("WMFW region @0x{:x} exceeds CS35L41 packed AHB range",r.payload_offset));
    }
    if (end-start) as usize != r.payload_length {
        return Err("WMFW packed bus span does not match payload length".into());
    }
    Ok(BusTransfer {memory:kind,
        original_file_span:r.payload_offset..r.end(),
        dsp_word_span:r.dsp_address..end_word,
        ahb_register_span:start..end})
}

pub fn all_transfers()->Result<Vec<BusTransfer>,String>{
    let all=wmfw_plan::regions()?;
    let mut bus=Vec::new();
    for r in &all {
        if HaloMemoryKind::from_wmfw_type(r.kind).is_ok(){bus.push(transfer_for_region(r)?);}
    }
    // Verify no independent region can overwrite an earlier loaded packed
    // AHB range, even if its WMFW file offsets are structurally valid.
    for (i,r) in bus.iter().enumerate(){
        for p in &bus[..i] {
            if p.memory==r.memory &&
               r.ahb_register_span.start<p.ahb_register_span.end &&
               p.ahb_register_span.start<r.ahb_register_span.end {
                return Err("overlapping HALO CS35L41 packed bus writes".into());
            }
        }
    }
    Ok(bus)
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]
    fn all_seven_reference_regions_map_inside_real_hardware_registers(){
        let mapped=all_transfers().unwrap();
        assert_eq!(mapped.len(),7);
        assert_eq!(mapped[0].ahb_register_span,XM_PACKED_BASE..XM_PACKED_BASE+72);
        assert_eq!(mapped[1].ahb_register_span,(XM_PACKED_BASE+0x9c)..(XM_PACKED_BASE+0x9c+528));
        assert_eq!(mapped[5].ahb_register_span,YM_PACKED_BASE..YM_PACKED_BASE+6132);
        assert_eq!(mapped[6].ahb_register_span,PM_PACKED_BASE..PM_PACKED_BASE+20280);
        assert!(mapped[5].ahb_register_span.end<=YM_PACKED_LAST+4);
        assert!(mapped[6].ahb_register_span.end<=PM_PACKED_LAST+4);
    }
    #[test]
    fn bus_map_rejects_bad_alignment_and_exceeding_device_memory(){
        let mut r=wmfw_plan::regions().unwrap()[1].clone();
        r.dsp_address+=1;
        assert!(transfer_for_region(&r).unwrap_err().contains("aligned") ||
            transfer_for_region(&r).unwrap_err().contains("span"));
        let mut r=wmfw_plan::regions().unwrap()[7].clone();
        r.dsp_address=0x2000;
        assert!(transfer_for_region(&r).unwrap_err().contains("exceeds"));
        assert!(transfer_for_region(&wmfw_plan::regions().unwrap()[0]).is_err());
    }
}
