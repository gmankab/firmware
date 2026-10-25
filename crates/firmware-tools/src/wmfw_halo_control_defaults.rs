// SPDX-License-Identifier: GPL-3.0-only
//! Named static control initializers for the pinned CS35L41 WMFW image.
//!
//! Every value in this module is a REFERENCE-OBSERVED INITIALIZATION value,
//! expressed through the existing named control schema. These are not the
//! original speaker-protection algorithm source, a live runtime snapshot, or
//! successful calibration results. In particular CAL_STATUS is initially 0,
//! and the zero CAL_CHECKSUM must not be interpreted as a success checksum.
//!
//! The 100-word CSPL_UPDATE_PARAMS_CONFIG initializer is a named zero buffer;
//! its internal field semantics are still unknown. Unnamed gaps between
//! controls, other coefficient tables, and all PM bytes stay unimplemented.
//!
//! Packed byte lanes follow Cirrus's public memory_type_converter at:
//! https://github.com/CirrusLogic/mcu-drivers/blob/a05b2878be911ae9ffaa440840cde17af98ba48e/tools/firmware_converter/wmfw_parser.py
//! Static algorithm allocations come from the separately typed XM directory.
//! No hardware register addresses or runtime write operations are produced.

use crate::{
    wmfw_algorithms::{self, Algorithm},
    wmfw_control_ranges::{self, ControlMemory},
    wmfw_halo_fixed_point::Signed24,
    wmfw_halo_static_data::{self, StaticDirectory},
    wmfw_plan::{self, WmfwLayoutRegion},
};
use std::collections::{BTreeMap, BTreeSet};

pub const NAMED_CONTROL_COUNT: usize = 57;
pub const INITIALIZED_LOGICAL_WORDS: usize = 264;
pub const GENERATED_INITIALIZER_BYTES: usize = INITIALIZED_LOGICAL_WORDS * 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultOrigin {
    ReferenceObservedInitialization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedControlDefault {
    pub algorithm_id: u32,
    pub name: String,
    pub memory: ControlMemory,
    pub relative_word_offset: u32,
    /// Logical static DSP word address, not a live AHB register address.
    pub static_dsp_word_address: u32,
    pub values: Vec<u64>,
    pub origin: DefaultOrigin,
}

#[derive(Debug, Clone, Copy)]
enum Initializer {
    Scalar(u64),
    ZeroBuffer(usize),
    AsciiWords { text: &'static str, words: usize },
}

impl Initializer {
    fn values(self) -> Result<Vec<u64>, String> {
        match self {
            Self::Scalar(value) => {
                if value > 0x00ff_ffff {
                    return Err("named HALO scalar exceeds 24 bits".into());
                }
                Ok(vec![value])
            }
            Self::ZeroBuffer(words) => {
                if words == 0 {
                    return Err("named HALO zero buffer must not be empty".into());
                }
                Ok(vec![0; words])
            }
            Self::AsciiWords { text, words } => {
                let capacity = words.checked_mul(3).ok_or("HALO ASCII capacity overflow")?;
                if words == 0 || !text.is_ascii() || text.len() > capacity {
                    return Err("HALO ASCII initializer exceeds its declared word buffer".into());
                }
                let mut out = vec![0; words];
                for (i, byte) in text.bytes().enumerate() {
                    out[i / 3] |= u64::from(byte) << (16 - 8 * (i % 3));
                }
                Ok(out)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct InitializerSpec {
    name: &'static str,
    memory: ControlMemory,
    offset: u32,
    initializer: Initializer,
}

const fn scalar(name: &'static str, offset: u32, value: u64) -> InitializerSpec {
    InitializerSpec { name, memory: ControlMemory::X, offset, initializer: Initializer::Scalar(value) }
}

const fn zero_buffer(name: &'static str, memory: ControlMemory, offset: u32, words: usize)
    -> InitializerSpec {
    InitializerSpec { name, memory, offset, initializer: Initializer::ZeroBuffer(words) }
}

const FRAMEWORK: &[InitializerSpec] = &[
    scalar("HALO_STATE", 0x00, 0),
    scalar("HALO_HEARTBEAT", 0x01, 0),
    scalar("AUDIO_BLK_SIZE", 0x02, 32),
    InitializerSpec { name: "BUILD_JOB_NAME", memory: ControlMemory::X, offset: 0x03,
        initializer: Initializer::AsciiWords { text: "Manual", words: 3 } },
    scalar("BUILD_JOB_NUMBER", 0x06, 0),
    scalar("GLOBAL_SAMPLE_RATE_HW", 0x07, 0),
    scalar("MULTIRATE_NUMBER_BANDS", 0x08, 1),
    scalar("FIRMWARE_STATE", 0x09, 0),
    scalar("EVENT_TIMEOUT", 0x0a, 12000),
    scalar("DC_OFFSET_HOLD_TIME", 0x0b, 0),
    scalar("ENABLE_FULL_US_BYPASS", 0x0c, 0),
    scalar("MAX_LRCLK_DELAY", 0x0d, 16),
];

const CSPL: &[InitializerSpec] = &[
    scalar("CSPL_ENABLE", 0x00, 1),
    scalar("CSPL_COMMAND", 0x01, 0),
    scalar("CSPL_STATE", 0x02, 0),
    scalar("CSPL_ERRORNO", 0x03, 0),
    scalar("CSPL_TEMPERATURE", 0x04, 0),
    scalar("UPDT_PRMS", 0x05, 0),
    scalar("CAL_R", 0x06, 0),
    scalar("CAL_AMBIENT", 0x07, 23),
    scalar("CAL_STATUS", 0x08, 0),
    scalar("CAL_CHECKSUM", 0x09, 0),
    scalar("CAL_R_SELECTED", 0x0a, 0),
    scalar("CAL_SET_STATUS", 0x0b, 0),
    scalar("DIAG_F0", 0x0c, 0),
    scalar("DIAG_Z_LOW_DIFF", 0x0d, 0),
    scalar("DIAG_F0_STATUS", 0x0e, 0),
    scalar("RTLOG_ENABLE", 0x0f, 0),
    scalar("RTLOG_STATE", 0x10, 0),
    scalar("RTLOG_COUNT", 0x11, 0),
    // Offsets 0x12 and 0x13 are unnamed and deliberately not initialized.
    scalar("RTLOG_FRMPRWIN", 0x14, 0),
    zero_buffer("RTLOG_VARIABLE", ControlMemory::X, 0x15, 15),
    zero_buffer("RTLOG_DATA", ControlMemory::X, 0x24, 30),
    scalar("BDLOG_MAX_TEMP", 0x42, 0),
    scalar("BDLOG_MAX_EXC", 0x43, 0),
    scalar("BDLOG_OVER_TEMP_COUNT", 0x44, 0),
    scalar("BDLOG_OVER_EXC_COUNT", 0x45, 0),
    scalar("BDLOG_ABNORMAL_MUTE", 0x46, 0),
    scalar("BDLOG_VMON_LOW_FLAG", 0x47, 0),
    scalar("REDUCE_POWER", 0x48, 0),
    scalar("CH_BAL", 0x49, 0),
    scalar("ATTENUATION", 0x4a, Signed24::MAX as u64),
    scalar("SPK_OUTPUT_POWER", 0x4b, 0),
    scalar("ALGO_FRAME_DELAY", 0x4c, 5),
    zero_buffer("CSPL_UPDATE_PARAMS_CONFIG", ControlMemory::Y, 0x00, 100),
];

const EVENTS: &[InitializerSpec] = &[
    scalar("EVNTLG_ENABLED", 0x00, 0),
    scalar("WAS_DISABLED", 0x01, 1),
    scalar("BUFFER", 0x02, 0),
    scalar("BUFFER_SIZE", 0x03, 32),
    scalar("TRANSFER_COMPLETED", 0x04, 1),
    scalar("NUM_LOST_EVENTS", 0x05, 0),
    scalar("LOST_EVENT_COUNTER", 0x06, 0),
    scalar("TRIGGER_INTERRUPT", 0x07, 0),
    scalar("BUFFER_COUNT", 0x08, 1),
    scalar("FREE_SLOTS", 0x09, 32),
    scalar("W_PTR", 0x0a, 0),
    zero_buffer("EVENTS", ControlMemory::X, 0x0b, 64),
];

fn allocation(directory: &StaticDirectory, id: u32, memory: ControlMemory)
    -> Result<(u32, u32), String> {
    let (xb, xs, yb, ys) = if id == directory.header.firmware_id {
        let h = directory.header;
        (h.xm_base, h.xm_size, h.ym_base, h.ym_size)
    } else {
        let a = directory.algorithms.iter().find(|a| a.id == id)
            .ok_or_else(|| format!("unknown static HALO algorithm {id:#x}"))?;
        (a.xm_base, a.xm_size, a.ym_base, a.ym_size)
    };
    Ok(match memory { ControlMemory::X => (xb, xs), ControlMemory::Y => (yb, ys) })
}

fn compile_defaults(algorithms: &[Algorithm], directory: &StaticDirectory)
    -> Result<Vec<NamedControlDefault>, String> {
    directory.validate()?;
    let slots = wmfw_control_ranges::compile_control_slots(algorithms)?;
    let groups = [(0x400a4, FRAMEWORK), (0x00cd, CSPL), (0xf20b, EVENTS)];
    let mut seen = BTreeSet::new();
    let mut out = Vec::with_capacity(NAMED_CONTROL_COUNT);
    for (id, specs) in groups {
        for spec in specs {
            if !seen.insert((id, spec.name)) {
                return Err("duplicate named HALO initializer".into());
            }
            let slot = slots.iter().find(|s| s.algorithm_id == id && s.name == spec.name)
                .ok_or_else(|| format!("unknown HALO initializer field {id:#x}:{}", spec.name))?;
            let values = spec.initializer.values()?;
            let length = u32::try_from(values.len()).map_err(|_| "HALO initializer size overflow")?;
            let end = spec.offset.checked_add(length).ok_or("HALO initializer offset overflow")?;
            if slot.memory != spec.memory || slot.word_span != (spec.offset..end) {
                return Err(format!("HALO initializer memory/offset/size disagreement for {}", spec.name));
            }
            let (base, size) = allocation(directory, id, spec.memory)?;
            if end > size {
                return Err(format!("HALO initializer {} exceeds static allocation", spec.name));
            }
            let address = base.checked_add(spec.offset).ok_or("static HALO initializer address overflow")?;
            out.push(NamedControlDefault {
                algorithm_id: id, name: spec.name.into(), memory: spec.memory,
                relative_word_offset: spec.offset, static_dsp_word_address: address,
                values, origin: DefaultOrigin::ReferenceObservedInitialization,
            });
        }
    }
    if seen.len() != slots.len() {
        return Err("HALO metadata contains a control with no explicit initializer".into());
    }
    if out.len() != NAMED_CONTROL_COUNT
        || out.iter().map(|c| c.values.len()).sum::<usize>() != INITIALIZED_LOGICAL_WORDS
    {
        return Err("named HALO initializer accounting mismatch".into());
    }
    Ok(out)
}

/// Explicit initial values for all 57 named controls in this pinned image.
pub fn nabu_control_defaults() -> Result<Vec<NamedControlDefault>, String> {
    compile_defaults(&[wmfw_algorithms::framework(), wmfw_algorithms::cspl(),
        wmfw_algorithms::event_logger()], &wmfw_halo_static_data::nabu_static_directory()?)
}

/// High-to-low logical word bytes within one 12-byte packed register group.
const WORD_LANES: [[usize; 3]; 4] = [[1, 2, 3], [6, 7, 0], [11, 4, 5], [8, 9, 10]];

fn word_byte_offsets(regions: &[WmfwLayoutRegion], memory: ControlMemory, address: u32)
    -> Result<[usize; 3], String> {
    let kind = match memory { ControlMemory::X => 0x11, ControlMemory::Y => 0x12 };
    let mut matched = None;
    for region in regions.iter().filter(|r| r.kind == kind) {
        if region.dsp_address % 4 != 0 || region.payload_length % 12 != 0 {
            return Err("HALO initializer mapping requires complete aligned register groups".into());
        }
        let count = u32::try_from(region.payload_length / 3).map_err(|_| "HALO region word count overflow")?;
        let end = region.dsp_address.checked_add(count).ok_or("HALO region address overflow")?;
        if address < region.dsp_address || address >= end { continue; }
        if matched.is_some() { return Err("overlapping HALO memory descriptors".into()); }
        let relative = usize::try_from(address - region.dsp_address).map_err(|_| "HALO word offset overflow")?;
        let group = (relative / 4).checked_mul(12).ok_or("HALO register block offset overflow")?;
        let start = region.payload_offset.checked_add(group).ok_or("HALO file offset overflow")?;
        let mut offsets = [0; 3];
        for (out, lane) in offsets.iter_mut().zip(WORD_LANES[relative % 4]) {
            *out = start.checked_add(lane).ok_or("HALO byte lane offset overflow")?;
            if *out >= region.end() { return Err("HALO byte lane outside payload".into()); }
        }
        matched = Some(offsets);
    }
    matched.ok_or_else(|| format!("static HALO {memory:?} word {address:#x} has no file payload"))
}

fn map_defaults(defaults: &[NamedControlDefault]) -> Result<BTreeMap<usize, u8>, String> {
    let expected = nabu_control_defaults()?;
    if defaults.len() != expected.len() { return Err("named HALO initializer count mismatch".into()); }
    let regions = wmfw_plan::regions()?;
    let mut seen = BTreeSet::new();
    let mut output = BTreeMap::new();
    for entry in defaults {
        if !seen.insert((entry.algorithm_id, entry.name.as_str())) {
            return Err("overlapping duplicate named HALO initializer".into());
        }
        let schema = expected.iter().find(|e| e.algorithm_id == entry.algorithm_id && e.name == entry.name)
            .ok_or_else(|| format!("unknown named HALO initializer {}", entry.name))?;
        if entry.memory != schema.memory || entry.relative_word_offset != schema.relative_word_offset
            || entry.static_dsp_word_address != schema.static_dsp_word_address
            || entry.values.len() != schema.values.len() || entry.origin != schema.origin
        {
            return Err(format!("named HALO initializer address/size disagreement for {}", entry.name));
        }
        for (i, value) in entry.values.iter().copied().enumerate() {
            if value > 0x00ff_ffff { return Err("named HALO initializer exceeds 24 bits".into()); }
            let address = entry.static_dsp_word_address
                .checked_add(u32::try_from(i).map_err(|_| "HALO initializer word index overflow")?)
                .ok_or("HALO initializer address overflow")?;
            let offsets = word_byte_offsets(&regions, entry.memory, address)?;
            for (offset, shift) in offsets.into_iter().zip([16, 8, 0]) {
                if output.insert(offset, (value >> shift) as u8).is_some() {
                    return Err("overlapping HALO initializer file bytes".into());
                }
            }
        }
    }
    if output.len() != GENERATED_INITIALIZER_BYTES {
        return Err("named HALO initializer byte accounting mismatch".into());
    }
    Ok(output)
}

/// Map only explicitly initialized logical words into their actual file lanes.
/// No neighboring byte lanes are filled to complete a register block.
pub fn generated_bytes() -> Result<BTreeMap<usize, u8>, String> {
    map_defaults(&nabu_control_defaults()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named<'a>(items: &'a [NamedControlDefault], id: u32, name: &str) -> &'a NamedControlDefault {
        items.iter().find(|c| c.algorithm_id == id && c.name == name).unwrap()
    }

    #[test]
    fn every_named_control_has_an_explicit_reference_initialization() {
        let defaults = nabu_control_defaults().unwrap();
        assert_eq!(defaults.len(), 57);
        assert_eq!(defaults.iter().map(|c| c.values.len()).sum::<usize>(), 264);
        assert!(defaults.iter().all(|c| c.origin == DefaultOrigin::ReferenceObservedInitialization));
        assert_eq!(named(&defaults, 0x400a4, "BUILD_JOB_NAME").values,
            [0x4d616e, 0x75616c, 0]);
        assert_eq!(named(&defaults, 0x400a4, "AUDIO_BLK_SIZE").values, [32]);
        assert_eq!(named(&defaults, 0x400a4, "EVENT_TIMEOUT").values, [12000]);
        assert_eq!(named(&defaults, 0xcd, "CAL_AMBIENT").values, [23]);
        assert_eq!(named(&defaults, 0xcd, "ATTENUATION").values, [Signed24::MAX as u64]);
        assert_eq!(named(&defaults, 0xcd, "ALGO_FRAME_DELAY").values, [5]);
        assert_eq!(named(&defaults, 0xf20b, "BUFFER_SIZE").values, [32]);
        assert_eq!(named(&defaults, 0xf20b, "FREE_SLOTS").values, [32]);
    }

    #[test]
    fn initial_calibration_is_not_a_success_record_and_config_fields_stay_unknown() {
        let defaults = nabu_control_defaults().unwrap();
        assert_eq!(named(&defaults, 0xcd, "CAL_R").values, [0]);
        assert_eq!(named(&defaults, 0xcd, "CAL_STATUS").values, [0]);
        assert_eq!(named(&defaults, 0xcd, "CAL_CHECKSUM").values, [0]);
        let config = named(&defaults, 0xcd, "CSPL_UPDATE_PARAMS_CONFIG");
        assert_eq!(config.memory, ControlMemory::Y);
        assert_eq!(config.static_dsp_word_address, 14);
        assert_eq!(config.values, vec![0; 100]);
        assert_eq!(named(&defaults, 0xf20b, "EVENTS").values.len(), 64);
    }

    #[test]
    fn maps_each_packed_word_lane_without_filling_unnamed_holes_or_pm() {
        let regions = wmfw_plan::regions().unwrap();
        let x = regions.iter().find(|r| r.kind == 0x11 && r.dsp_address == 0x34).unwrap();
        for lane in 0..4 {
            let offsets = word_byte_offsets(&regions, ControlMemory::X, 0x34 + lane as u32).unwrap();
            assert_eq!(offsets, WORD_LANES[lane].map(|o| x.payload_offset + o));
        }
        let generated = generated_bytes().unwrap();
        assert_eq!(generated.len(), 792);
        assert!(generated.keys().all(|o| *o < 0x278c));
        for relative_hole in [0x12, 0x13] {
            for offset in word_byte_offsets(&regions, ControlMemory::X, 0x94 + relative_hole).unwrap() {
                assert!(!generated.contains_key(&offset));
            }
        }
        assert!(word_byte_offsets(&regions, ControlMemory::X, 0x18).is_err());
        assert!(word_byte_offsets(&regions, ControlMemory::Y, 0x7fc).is_err());
        let mut duplicate = regions.clone();
        duplicate.push(x.clone());
        assert!(word_byte_offsets(&duplicate, ControlMemory::X, 0x34).unwrap_err().contains("overlap"));
    }

    #[test]
    fn rejects_unknown_fields_schema_size_changes_and_static_allocation_errors() {
        let directory = wmfw_halo_static_data::nabu_static_directory().unwrap();
        let algorithms = vec![wmfw_algorithms::framework(), wmfw_algorithms::cspl(), wmfw_algorithms::event_logger()];
        let mut changed = algorithms.clone();
        changed[0].controls[0].name = "UNKNOWN_FIELD".into();
        assert!(compile_defaults(&changed, &directory).unwrap_err().contains("unknown"));
        let mut changed = algorithms.clone();
        changed[2].controls.last_mut().unwrap().size_bytes -= 4;
        assert!(compile_defaults(&changed, &directory).unwrap_err().contains("disagreement"));
        let mut changed_directory = directory;
        changed_directory.header.xm_size -= 1;
        assert!(compile_defaults(&algorithms, &changed_directory).unwrap_err().contains("allocation"));
    }

    #[test]
    fn rejects_duplicate_address_size_and_word_width_changes() {
        let good = nabu_control_defaults().unwrap();
        let mut bad = good.clone();
        bad[1] = bad[0].clone();
        assert!(map_defaults(&bad).unwrap_err().contains("duplicate"));
        let mut bad = good.clone();
        bad[0].static_dsp_word_address = u32::MAX;
        assert!(map_defaults(&bad).unwrap_err().contains("disagreement"));
        let mut bad = good.clone();
        bad[0].values.push(0);
        assert!(map_defaults(&bad).unwrap_err().contains("disagreement"));
        let mut bad = good;
        bad[0].values[0] = 1 << 24;
        assert!(map_defaults(&bad).unwrap_err().contains("24 bits"));
    }
}
