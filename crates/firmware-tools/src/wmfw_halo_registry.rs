// SPDX-License-Identifier: GPL-3.0-only
//! HALO *runtime* firmware and algorithm directory, as described by Linux's
//! `wmfw_v3_id_hdr`, `wmfw_halo_id_hdr`, and `wmfw_halo_alg_hdr` structures.
//!
//! This is NOT a parser for the packed XM payload in a WMFW file. The Linux
//! loader reads the directory from the running HALO processor using unpacked
//! 32-bit register accesses. The caller must provide that actual byte stream
//! (big-endian 32-bit device words); fabricating it from the WMFW static
//! segments would lead to invalid algorithm/control addresses.
//!
//! Once such a runtime dump is supplied, the named controls from the WMFW
//! metadata can be resolved against their corresponding algorithm's XM/YM
//! base safely, instead of relying on guessed DSP addresses.
//!
//! The identifier header describes the firmware's own memory allocation.
//! Its following `algorithm_count` entries describe separate algorithms;
//! their memory is not required to lie inside the header's allocation.
//! Linux v6.12 `cs_dsp_halo_setup_algs()` registers the header's firmware ID
//! before iterating the following algorithm records:
//! https://github.com/torvalds/linux/blob/v6.12/drivers/firmware/cirrus/cs_dsp.c
//! The public record layout is defined in:
//! https://github.com/torvalds/linux/blob/v6.12/include/linux/firmware/cirrus/wmfw.h
use crate::{wmfw_algorithms,wmfw_control_ranges::{self,ControlMemory}};
use std::{collections::BTreeMap,ops::Range};

pub const RUNTIME_ID_HEADER_BYTES: usize=40;
pub const RUNTIME_ALGORITHM_BYTES: usize=24;
pub const MAX_RUNTIME_ALGORITHMS: usize=256;
pub const RUNTIME_DIRECTORY_TERMINATOR: u32=0x00be_dead;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct HaloRuntimeHeader {
    pub core_id:u32,
    pub block_revision:u32,
    pub vendor_id:u32,
    pub firmware_id:u32,
    pub firmware_version:u32,
    pub xm_base:u32,
    pub xm_size:u32,
    pub ym_base:u32,
    pub ym_size:u32,
    pub algorithm_count:u32,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct HaloRuntimeAlgorithm {
    pub id:u32,
    pub version:u32,
    pub xm_base:u32,
    pub xm_size:u32,
    pub ym_base:u32,
    pub ym_size:u32,
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct HaloRuntimeRegistry {
    pub header:HaloRuntimeHeader,
    pub algorithms:Vec<HaloRuntimeAlgorithm>,
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ResolvedControl {
    pub algorithm_id:u32,
    pub control_name:String,
    pub memory:ControlMemory,
    pub dsp_word_span:Range<u32>,
}

fn read_u32_be(data:&[u8], offset:usize)->Result<u32,String> {
    let chunk=data.get(offset..offset.checked_add(4).ok_or("HALO u32 offset overflow")?)
        .ok_or("truncated HALO runtime registry field")?;
    Ok(u32::from_be_bytes(chunk.try_into().unwrap()))
}

fn checked_region(base:u32,size:u32,capacity:u32,name:&str)->Result<Range<u32>,String> {
    let end=base.checked_add(size).ok_or_else(||format!("{name} region address overflow"))?;
    if end>capacity {return Err(format!("{name} region exceeds DSP word address space"));}
    Ok(base..end)
}

fn checked_algorithm_regions(registry:&HaloRuntimeRegistry,
    x_capacity_words:u32,y_capacity_words:u32)
    ->Result<BTreeMap<u32,HaloRuntimeAlgorithm>,String> {
    if registry.header.algorithm_count as usize>MAX_RUNTIME_ALGORITHMS {
        return Err(format!("HALO algorithm count {} exceeds safety limit",
            registry.header.algorithm_count));
    }
    if registry.header.algorithm_count as usize!=registry.algorithms.len() {
        return Err("HALO registry algorithm count differs from table length".into());
    }
    let h=registry.header;
    let firmware=HaloRuntimeAlgorithm{id:h.firmware_id,version:h.firmware_version,
        xm_base:h.xm_base,xm_size:h.xm_size,ym_base:h.ym_base,ym_size:h.ym_size};
    let mut by_id=BTreeMap::new();
    let mut intervals_x=Vec::new();
    let mut intervals_y=Vec::new();
    for a in std::iter::once(&firmware).chain(registry.algorithms.iter()) {
        if by_id.insert(a.id,*a).is_some() {
            return Err(format!("duplicate HALO algorithm ID 0x{:x}",a.id));
        }
        let x=checked_region(a.xm_base,a.xm_size,x_capacity_words,"algorithm XM")?;
        let y=checked_region(a.ym_base,a.ym_size,y_capacity_words,"algorithm YM")?;
        if a.xm_size!=0 {intervals_x.push((x,a.id));}
        if a.ym_size!=0 {intervals_y.push((y,a.id));}
    }
    // Firmware and algorithm allocations are peers. In this checked model
    // nonempty allocations must be disjoint, including the firmware's own.
    for (name,mut intervals) in [("XM",intervals_x),("YM",intervals_y)] {
        intervals.sort_unstable_by_key(|(r,_)|r.start);
        for pair in intervals.windows(2){
            if pair[1].0.start<pair[0].0.end {
                return Err(format!("HALO {name} algorithm regions overlap: 0x{:x} and 0x{:x}",pair[0].1,pair[1].1));
            }
        }
    }
    Ok(by_id)
}

/// Recheck a parsed or manually constructed registry against caller-supplied
/// device capacities, including the firmware-header allocation. The header's
/// own X/Y sizes are not device capacities or an enclosing algorithm arena.
pub fn validate_runtime_registry(registry:&HaloRuntimeRegistry,
    x_capacity_words:u32,y_capacity_words:u32)->Result<(),String> {
    checked_algorithm_regions(registry,x_capacity_words,y_capacity_words).map(|_|())
}

/// Parse either the exact header/table prefix (`40 + 24*n` bytes) or that
/// prefix followed by the big-endian `0x00bedead` directory terminator.
/// Linux `cs_dsp_read_algs()` reads that terminator separately from the table.
/// If included here it must be correct; incomplete trailers and other trailing
/// bytes are rejected. Accepting the prefix alone does not validate a sentinel.
pub fn parse_runtime_registry(data:&[u8], x_capacity_words:u32, y_capacity_words:u32)
    ->Result<HaloRuntimeRegistry,String> {
    if data.len()<RUNTIME_ID_HEADER_BYTES {return Err("truncated HALO ID header".into());}
    let mut words=[0u32;10];
    for (i,word) in words.iter_mut().enumerate(){*word=read_u32_be(data,i*4)?;}
    let [core_id,block_revision,vendor_id,firmware_id,firmware_version,
         xm_base,xm_size,ym_base,ym_size,algorithm_count]=words;
    if algorithm_count as usize>MAX_RUNTIME_ALGORITHMS {
        return Err(format!("HALO algorithm count {algorithm_count} exceeds safety limit"));
    }
    let expected=RUNTIME_ID_HEADER_BYTES.checked_add((algorithm_count as usize)
        .checked_mul(RUNTIME_ALGORITHM_BYTES).ok_or("algorithm table length overflow")?)
        .ok_or("HALO registry size overflow")?;
    let terminated=expected.checked_add(4).ok_or("HALO terminated registry size overflow")?;
    if data.len()==terminated {
        if read_u32_be(data,expected)?!=RUNTIME_DIRECTORY_TERMINATOR {
            return Err("HALO registry directory terminator differs from 0x00bedead".into());
        }
    } else if data.len()!=expected {
        return Err(format!("HALO registry length {} != prefix {expected} or terminated {terminated}",data.len()));
    }
    let header=HaloRuntimeHeader{core_id,block_revision,vendor_id,
        firmware_id,firmware_version,xm_base,xm_size,ym_base,ym_size,
        algorithm_count};
    let mut result=Vec::with_capacity(algorithm_count as usize);
    for i in 0..algorithm_count as usize {
        let at=RUNTIME_ID_HEADER_BYTES+i*RUNTIME_ALGORITHM_BYTES;
        let mut w=[0u32;6];
        for (j,field) in w.iter_mut().enumerate() {*field=read_u32_be(data,at+j*4)?;}
        let [id,version,xm_base,xm_size,ym_base,ym_size]=w;
        result.push(HaloRuntimeAlgorithm{id,version,xm_base,xm_size,ym_base,ym_size});
    }
    let registry=HaloRuntimeRegistry{header,algorithms:result};
    validate_runtime_registry(&registry,x_capacity_words,y_capacity_words)?;
    Ok(registry)
}

/// Validate and resolve independently named WMFW v3 control offset ranges
/// against the runtime registry, without assuming address bases from the WMFW
/// payload or counting these addresses as recovered firmware instructions.
/// The firmware ID resolves from the identifier header, without requiring a
/// duplicate table entry. This function rechecks structural integrity; call
/// `validate_runtime_registry` with device capacities after manual edits.
pub fn resolve_named_controls(registry:&HaloRuntimeRegistry)->Result<Vec<ResolvedControl>,String>{
    // The existing public registry structure does not retain parse-time
    // capacities. Recheck arithmetic, counts, IDs and overlaps here; device
    // bounds are checked by the parser/validator and the control-bus mapper.
    let by_id=checked_algorithm_regions(registry,u32::MAX,u32::MAX)?;
    let slots=wmfw_control_ranges::compile_control_slots(&[
        wmfw_algorithms::framework(),wmfw_algorithms::cspl(),wmfw_algorithms::event_logger()])?;
    let mut absolute=Vec::with_capacity(slots.len());
    for slot in slots {
        let a=by_id.get(&slot.algorithm_id)
            .ok_or_else(||format!("missing runtime HALO algorithm 0x{:x}",slot.algorithm_id))?;
        let (base,size)=match slot.memory {
            ControlMemory::X=>(a.xm_base,a.xm_size),
            ControlMemory::Y=>(a.ym_base,a.ym_size),
        };
        if slot.word_span.end>size {
            return Err(format!("control {} exceeds algorithm-relative {:?} memory size",slot.name,slot.memory));
        }
        let start=base.checked_add(slot.word_span.start).ok_or("HALO control start address overflow")?;
        let end=base.checked_add(slot.word_span.end).ok_or("HALO control end address overflow")?;
        absolute.push(ResolvedControl {algorithm_id:slot.algorithm_id,
            control_name:slot.name,memory:slot.memory,dsp_word_span:start..end});
    }
    Ok(absolute)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn encode_table(fw:&[u32;10],algs:&[[u32;6]])->Vec<u8>{
        let mut bytes=Vec::new();
        for n in fw {bytes.extend_from_slice(&n.to_be_bytes());}
        for alg in algs {for n in alg {bytes.extend_from_slice(&n.to_be_bytes());}}
        bytes
    }
    fn synthetic_firmware_and_two_algorithms()->Vec<u8>{
        // Synthetic host-side *unpacked* HALO registry, not copied from WMFW.
        // XM and YM algorithm address ranges are disjoint and capacity-bounded.
        encode_table(
            &[4,1,1,0x400a4,0x010203,0x20,0x20,0,0,2],
            &[
                [0xcd,0x010203,0x100,0x100,0x100,0x100],
                [0xf20b,0x010203,0x300,0x100,0x300,0],
            ],
        )
    }
    #[test]
    fn decodes_cirrus_halo_v3_runtime_structure_and_resolves_controls() {
        let data=synthetic_firmware_and_two_algorithms();
        assert_eq!(data.len(),40+2*24);
        let registry=parse_runtime_registry(&data,0x5ac,0x7fc).unwrap();
        assert_eq!(registry.header.core_id,4);
        assert_eq!(registry.header.algorithm_count,2);
        assert_eq!(registry.algorithms[0].xm_base,0x100);
        assert!(registry.algorithms.iter().all(|a|a.id!=registry.header.firmware_id));
        let controls=resolve_named_controls(&registry).unwrap();
        assert_eq!(controls.len(),57);
        let cspl_y=controls.iter().find(|c|c.algorithm_id==0xcd && c.memory==ControlMemory::Y).unwrap();
        assert_eq!(cspl_y.dsp_word_span,0x100..0x164);
        let f=controls.iter().find(|c|c.algorithm_id==0x400a4 && c.memory==ControlMemory::X).unwrap();
        assert_eq!(f.dsp_word_span,0x20..0x21);
    }
    #[test]
    fn rejects_wrong_length_count_endian_alias_and_bad_ranges() {
        let mut data=synthetic_firmware_and_two_algorithms();
        assert!(parse_runtime_registry(&data[..39],0x5ac,0x7fc).unwrap_err().contains("truncated"));
        assert!(parse_runtime_registry(&data[..data.len()-1],0x5ac,0x7fc).unwrap_err().contains("length"));
        data[36..40].copy_from_slice(&257u32.to_be_bytes());
        assert!(parse_runtime_registry(&data,0x5ac,0x7fc).unwrap_err().contains("safety limit"));
        let mut data=synthetic_firmware_and_two_algorithms();
        data[40..44].copy_from_slice(&0x400a4u32.to_be_bytes());
        assert!(parse_runtime_registry(&data,0x5ac,0x7fc).unwrap_err().contains("duplicate"));
        let mut data=synthetic_firmware_and_two_algorithms();
        data[40+8..40+12].copy_from_slice(&0x30u32.to_be_bytes());
        assert!(parse_runtime_registry(&data,0x5ac,0x7fc).unwrap_err().contains("overlap"));
        let mut data=synthetic_firmware_and_two_algorithms();
        data[40+24+8..40+24+12].copy_from_slice(&0x600u32.to_be_bytes());
        assert!(parse_runtime_registry(&data,0x5ac,0x7fc).unwrap_err().contains("exceeds"));
    }
    #[test]
    fn rejects_out_of_bounds_control_and_missing_runtime_algorithm() {
        let data=synthetic_firmware_and_two_algorithms();
        let mut registry=parse_runtime_registry(&data,0x5ac,0x7fc).unwrap();
        registry.algorithms[0].xm_size=3;
        assert!(resolve_named_controls(&registry).unwrap_err().contains("exceeds"));
        registry.algorithms[0].xm_size=0x100;
        registry.algorithms.pop();
        registry.header.algorithm_count-=1;
        assert!(resolve_named_controls(&registry).unwrap_err().contains("missing"));
    }

    #[test]
    fn firmware_header_allocation_uses_device_bounds_and_cannot_overlap_algorithms() {
        let mut registry=parse_runtime_registry(&synthetic_firmware_and_two_algorithms(),0x5ac,0x7fc).unwrap();
        registry.header.xm_base=0x590;
        assert!(validate_runtime_registry(&registry,0x5ac,0x7fc).unwrap_err().contains("exceeds"));
        registry.header.xm_base=0x100;
        assert!(validate_runtime_registry(&registry,0x5ac,0x7fc).unwrap_err().contains("overlap"));
        registry.header.xm_base=0x20;
        registry.header.ym_base=0x180;
        registry.header.ym_size=1;
        assert!(validate_runtime_registry(&registry,0x5ac,0x7fc).unwrap_err().contains("overlap"));
        registry.header.ym_size=0;
        validate_runtime_registry(&registry,0x5ac,0x7fc).unwrap();
    }

    #[test]
    fn resolver_rechecks_manually_modified_registry_structure() {
        let original=parse_runtime_registry(&synthetic_firmware_and_two_algorithms(),0x5ac,0x7fc).unwrap();
        let mut changed=original.clone();
        changed.header.algorithm_count=3;
        assert!(resolve_named_controls(&changed).unwrap_err().contains("count"));
        let mut changed=original.clone();
        changed.algorithms[1].id=changed.header.firmware_id;
        assert!(resolve_named_controls(&changed).unwrap_err().contains("duplicate"));
        let mut changed=original.clone();
        changed.algorithms[1].id=changed.algorithms[0].id;
        assert!(resolve_named_controls(&changed).unwrap_err().contains("duplicate"));
        let mut changed=original.clone();
        changed.algorithms[0].xm_base=u32::MAX;
        assert!(resolve_named_controls(&changed).unwrap_err().contains("overflow"));
        let mut changed=original;
        changed.algorithms[0].xm_base=0x20;
        assert!(resolve_named_controls(&changed).unwrap_err().contains("overlap"));
    }

    #[test]
    fn accepts_optional_directory_terminator_and_rejects_invalid_trailers() {
        let prefix=synthetic_firmware_and_two_algorithms();
        let expected=parse_runtime_registry(&prefix,0x5ac,0x7fc).unwrap();
        let mut terminated=prefix.clone();
        terminated.extend(RUNTIME_DIRECTORY_TERMINATOR.to_be_bytes());
        assert_eq!(parse_runtime_registry(&terminated,0x5ac,0x7fc).unwrap(),expected);
        let last=terminated.len()-1;
        terminated[last]^=1;
        assert!(parse_runtime_registry(&terminated,0x5ac,0x7fc).unwrap_err().contains("terminator"));
        for trailer in [&[0xbe,0xde,0xad][..], &[0,0xbe,0xde,0xad,0][..]] {
            let mut invalid=prefix.clone();
            invalid.extend(trailer);
            assert!(parse_runtime_registry(&invalid,0x5ac,0x7fc).unwrap_err().contains("length"));
        }
    }
}
