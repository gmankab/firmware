// SPDX-License-Identifier: GPL-3.0-only
//! Checked CS35L41 register windows for *runtime* HALO coefficient controls.
//!
//! Linux configures `WMFW_ADSP2_XM` and `WMFW_ADSP2_YM` using the CS35L41
//! **unpacked-24** register windows, `0x02800000` and `0x03400000`. Its HALO
//! `region_to_reg` maps an absolute DSP control-word address `w` to
//! `mem.base + 4*w`. These are NOT the packed XM/YM memory upload addresses;
//! they are also NOT the narrower unpacked-32 memory windows.
//!
//! This module does not infer per-algorithm bases from the packed WMFW. The
//! caller must provide a validated *live* HALO runtime algorithm directory.
//! No HALO instruction or WMFW payload bytes are reconstructed here.

use std::{collections::BTreeMap, ops::Range};
use crate::wmfw_control_ranges::ControlMemory;
use crate::wmfw_halo_registry::{self, HaloRuntimeRegistry, ResolvedControl};

// Linux include/sound/cs35l41.h (last valid 32-bit register inclusive).
pub const XM_UNPACK24_BASE: u32 = 0x0280_0000;
pub const XM_UNPACK24_LAST: u32 = 0x0280_3ff4;
pub const YM_UNPACK24_BASE: u32 = 0x0340_0000;
pub const YM_UNPACK24_LAST: u32 = 0x0340_1ff4;

pub const REGISTER_BYTES_PER_DSP_WORD: u32 = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedControlBusSpan {
    pub algorithm_id: u32,
    pub control_name: String,
    pub memory: ControlMemory,
    pub dsp_word_span: Range<u32>,
    /// Half-open byte-addressed AHB register interval [start, end).
    pub register_span: Range<u32>,
}

fn register_bounds(kind: ControlMemory) -> (u32,u32) {
    match kind {
        ControlMemory::X => (XM_UNPACK24_BASE, XM_UNPACK24_LAST),
        ControlMemory::Y => (YM_UNPACK24_BASE, YM_UNPACK24_LAST),
    }
}

/// Convert a checked *absolute DSP word address interval*, not a packed WMFW
/// payload offset, to the corresponding unpacked-24 host register range.
pub fn word_span_to_registers(kind: ControlMemory, words: Range<u32>) -> Result<Range<u32>, String> {
    if words.start >= words.end {
        return Err("empty or reversed HALO control DSP word span".into());
    }
    let (base,last_reg)=register_bounds(kind);
    let start_offset=words.start.checked_mul(REGISTER_BYTES_PER_DSP_WORD)
        .ok_or("HALO control register start multiplication overflow")?;
    let end_offset=words.end.checked_mul(REGISTER_BYTES_PER_DSP_WORD)
        .ok_or("HALO control register end multiplication overflow")?;
    let start=base.checked_add(start_offset)
        .ok_or("HALO control register start address overflow")?;
    let end=base.checked_add(end_offset)
        .ok_or("HALO control register end address overflow")?;
    let exclusive_limit=last_reg.checked_add(REGISTER_BYTES_PER_DSP_WORD)
        .ok_or("HALO control register range limit overflow")?;
    if start < base || end > exclusive_limit || start >= end {
        return Err(format!("HALO {:?} control exceeds CS35L41 unpacked-24 register window",kind));
    }
    Ok(start..end)
}

/// Convert named DSP controls to their exact host-side 32-bit register spans.
/// Reject overlaps within one memory window even when users pass custom
/// (previously resolved) controls rather than `resolve_named_controls()`.
pub fn resolved_controls_to_bus(controls: &[ResolvedControl]) -> Result<Vec<ResolvedControlBusSpan>, String> {
    let mut out=Vec::with_capacity(controls.len());
    let mut overlap_check: BTreeMap<ControlMemory, Vec<(Range<u32>, String)>>=BTreeMap::new();
    for ctl in controls {
        let span=word_span_to_registers(ctl.memory, ctl.dsp_word_span.clone())?;
        overlap_check.entry(ctl.memory).or_default().push((span.clone(), ctl.control_name.clone()));
        out.push(ResolvedControlBusSpan {
            algorithm_id:ctl.algorithm_id, control_name:ctl.control_name.clone(),
            memory:ctl.memory, dsp_word_span:ctl.dsp_word_span.clone(),
            register_span:span,
        });
    }
    for (memory,spans) in &mut overlap_check {
        spans.sort_by_key(|x|x.0.start);
        for pair in spans.windows(2) {
            if pair[1].0.start < pair[0].0.end {
                return Err(format!("overlapping HALO {:?} control registers: {} / {}",
                    memory,pair[0].1,pair[1].1));
            }
        }
    }
    Ok(out)
}

/// Resolve the 57 independently named control records using per-algorithm
/// DSP word bases from a real, caller-supplied unpacked runtime registry.
pub fn resolve_control_bus_spans(registry: &HaloRuntimeRegistry) -> Result<Vec<ResolvedControlBusSpan>, String> {
    let controls=wmfw_halo_registry::resolve_named_controls(registry)?;
    resolved_controls_to_bus(&controls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wmfw_halo_registry::parse_runtime_registry;

    fn synthetic_three_algorithm_directory() -> Vec<u8> {
        let header:[u32;10]=[4,1,1,0x400a4,0x010203,0,0x5ac,0,0x7fc,3];
        let algs:[[u32;6];3]=[[0x400a4,0x010203,0x20,0x20,0,0],
                  [0xcd,0x010203,0x100,0x100,0x100,0x100],
                  [0xf20b,0x010203,0x300,0x100,0x300,0]];
        let mut output=Vec::new();
        for v in header {output.extend(v.to_be_bytes());}
        for alg in algs {for v in alg {output.extend(v.to_be_bytes());}}
        output
    }

    #[test]
    fn resolves_all_named_controls_in_linux_unpacked24_ahb_windows() {
        let raw=synthetic_three_algorithm_directory();
        let registry=parse_runtime_registry(&raw,0x5ac,0x7fc).unwrap();
        let bus=resolve_control_bus_spans(&registry).unwrap();
        assert_eq!(bus.len(),57);
        assert_eq!(bus.iter().filter(|x|x.memory==ControlMemory::X).count(),56);
        assert_eq!(bus.iter().filter(|x|x.memory==ControlMemory::Y).count(),1);
        let cspl_ym=bus.iter().find(|x|x.memory==ControlMemory::Y).unwrap();
        assert_eq!(cspl_ym.algorithm_id,0xcd);
        assert_eq!(cspl_ym.dsp_word_span,0x100..0x164);
        assert_eq!(cspl_ym.register_span,0x0340_0400..0x0340_0590);
        assert_eq!(cspl_ym.register_span.end-cspl_ym.register_span.start,400);
        let framework=bus.iter().find(|x|x.algorithm_id==0x400a4).unwrap();
        assert!(framework.register_span.start>=XM_UNPACK24_BASE+0x80);
        assert!(bus.iter().all(|x|x.register_span.start%4==0 && x.register_span.end%4==0));
    }

    #[test]
    fn last_valid_register_is_allowed_and_next_is_rejected() {
        assert_eq!(word_span_to_registers(ControlMemory::X,0xffd..0xffe).unwrap(),
            0x0280_3ff4..0x0280_3ff8);
        assert_eq!(word_span_to_registers(ControlMemory::Y,0x7fd..0x7fe).unwrap(),
            0x0340_1ff4..0x0340_1ff8);
        assert!(word_span_to_registers(ControlMemory::X,0xffd..0xfff)
            .unwrap_err().contains("exceeds"));
        assert!(word_span_to_registers(ControlMemory::Y,0x7fc..0x7ff)
            .unwrap_err().contains("exceeds"));
    }

    #[test]
    fn rejects_empty_reversed_overflow_and_wrong_memory_window() {
        assert!(word_span_to_registers(ControlMemory::X,5..5).unwrap_err().contains("empty"));
        assert!(word_span_to_registers(ControlMemory::Y,6..5).unwrap_err().contains("reversed"));
        assert!(word_span_to_registers(ControlMemory::X,u32::MAX..u32::MAX)
            .unwrap_err().contains("empty"));
        assert!(word_span_to_registers(ControlMemory::X,(u32::MAX/4)..u32::MAX)
            .unwrap_err().contains("overflow"));
        assert!(word_span_to_registers(ControlMemory::Y,0x900..0x901)
            .unwrap_err().contains("exceeds"));
    }

    #[test]
    fn refuses_overlapping_absolute_register_controls() {
        let a=ResolvedControl{algorithm_id:0x10,control_name:"A".to_string(),
            memory:ControlMemory::X,dsp_word_span:0x100..0x103};
        let mut b=a.clone();
        b.algorithm_id=0x11;
        b.control_name="B".to_string();
        b.dsp_word_span=0x102..0x107;
        assert!(resolved_controls_to_bus(&[a.clone(),b.clone()])
            .unwrap_err().contains("overlapping"));
        b.memory=ControlMemory::Y;
        let permitted=resolved_controls_to_bus(&[a,b]).unwrap();
        assert_eq!(permitted.len(),2);
        assert_eq!(permitted[0].register_span,0x0280_0400..0x0280_040c);
        assert_eq!(permitted[1].register_span,0x0340_0408..0x0340_041c);
    }
}
