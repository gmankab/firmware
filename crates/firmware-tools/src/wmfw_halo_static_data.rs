// SPDX-License-Identifier: GPL-3.0-only
//! Typed reconstruction of the static HALO firmware directory in packed XM.
//!
//! The layout and packing are defined by Cirrus's public firmware converter:
//! https://github.com/CirrusLogic/mcu-drivers/blob/a05b2878be911ae9ffaa440840cde17af98ba48e/tools/firmware_converter/wmfw_parser.py
//! (`halo_firmware_id_block`, `memory_type_converter`). Image-specific IDs,
//! versions and placements were recovered from the SHA-pinned reference.
//! They are reference-derived typed data, not independently recovered DSP
//! algorithms. A static allocation is not evidence of a live device address.
//!
//! No reference bytes are read by the encoder. No PM bytes are generated.

use crate::{
    image_plan::ImagePlan, wmfw_algorithms, wmfw_compact_y, wmfw_control_ranges,
    wmfw_external_payload, wmfw_halo_memory::{self, HaloMemoryKind},
    wmfw_halo_registry::{self, HaloRuntimeAlgorithm, HaloRuntimeHeader, HaloRuntimeRegistry},
    wmfw_plan, wmfw_sha256,
};
use std::{fs, path::Path};

pub const DIRECTORY_FILE_OFFSET: usize = 0x68;
pub const DIRECTORY_PACKED_BYTES: usize = 72;
pub const DIRECTORY_FIELD_BYTES: usize = 69;
pub const DIRECTORY_PADDING_BYTES: usize = 3;
pub const DIRECTORY_SHA256: &str =
    "44510542e96ef4ba1c5a4fea0cb0b516cd81698bc83cb254d30b055f1329dd7f";
pub const EXTERNAL_BYTES_WITH_DIRECTORY: usize =
    wmfw_compact_y::EXTERNAL_SOURCE_BYTES - DIRECTORY_PACKED_BYTES;
pub const X_CAPACITY_WORDS: u32 = 0x5ac;
pub const Y_CAPACITY_WORDS: u32 = 0x7fc;

/// Numeric identity/allocation fields as stored in this image's static XM
/// directory. This type deliberately offers no hardware-register resolver.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticDirectory {
    pub header: HaloRuntimeHeader,
    pub algorithms: Vec<HaloRuntimeAlgorithm>,
}

const fn version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 16) | (minor << 8) | patch
}

pub fn nabu_static_directory() -> Result<StaticDirectory, String> {
    // Allocation lengths of the named XM controls are independently checked
    // against the already authored WMFW metadata. Base addresses and version
    // numbers remain explicit reference-derived fields.
    let coverage = wmfw_control_ranges::coverage()?;
    let high_word = |id| -> Result<u32, String> {
        coverage.iter().find(|c| c.algorithm_id == id)
            .map(|c| c.x_highest_word).ok_or_else(|| format!("missing control schema {id:#x}"))
    };
    let framework_id = wmfw_algorithms::framework().id;
    let cspl_id = wmfw_algorithms::cspl().id;
    let events_id = wmfw_algorithms::event_logger().id;
    let algorithms = vec![
        HaloRuntimeAlgorithm {
            id: cspl_id, version: version(29, 53, 0),
            xm_base: 0x94, xm_size: high_word(cspl_id)?,
            ym_base: 14, ym_size: 0x7f7 - 14,
        },
        HaloRuntimeAlgorithm {
            id: events_id, version: version(0, 0, 1),
            xm_base: 0x170, xm_size: high_word(events_id)?,
            ym_base: 0, ym_size: 0,
        },
    ];
    let header = HaloRuntimeHeader {
        core_id: 4 << 16, block_revision: 3 << 16, vendor_id: 2,
        firmware_id: framework_id, firmware_version: version(0, 33, 0),
        xm_base: 0x1e0, xm_size: high_word(framework_id)?,
        ym_base: 0, ym_size: 0, algorithm_count: algorithms.len() as u32,
    };
    let result = StaticDirectory { header, algorithms };
    result.validate()?;
    Ok(result)
}

impl StaticDirectory {
    pub fn validate(&self) -> Result<(), String> {
        // Reuse structural validation without presenting this as a captured
        // runtime registry or deriving hardware addresses from it.
        wmfw_halo_registry::validate_runtime_registry(
            &HaloRuntimeRegistry { header: self.header, algorithms: self.algorithms.clone() },
            X_CAPACITY_WORDS, Y_CAPACITY_WORDS,
        )?;
        if self.words_unpadded().iter().any(|w| *w > 0x00ff_ffff) {
            return Err("static directory field exceeds a 24-bit DSP word".into());
        }
        Ok(())
    }

    fn words_unpadded(&self) -> Vec<u64> {
        let h = self.header;
        let mut words: Vec<u64> = [h.core_id, h.block_revision, h.vendor_id,
            h.firmware_id, h.firmware_version, h.xm_base, h.xm_size,
            h.ym_base, h.ym_size, h.algorithm_count]
            .into_iter().map(u64::from).collect();
        for a in &self.algorithms {
            words.extend([a.id, a.version, a.xm_base, a.xm_size, a.ym_base, a.ym_size]
                .into_iter().map(u64::from));
        }
        words.push(u64::from(wmfw_halo_registry::RUNTIME_DIRECTORY_TERMINATOR));
        words
    }

    pub fn words(&self) -> Result<Vec<u64>, String> {
        self.validate()?;
        let mut words = self.words_unpadded();
        // Complete packed register blocks require four logical 24-bit words.
        // This is explicit directory padding, not an unknown-memory default.
        while words.len() % 4 != 0 { words.push(0); }
        Ok(words)
    }

    pub fn encode(&self) -> Result<Vec<u8>, String> {
        wmfw_halo_memory::pack_words(HaloMemoryKind::PackedX, &self.words()?)
    }
}

pub fn build_static_directory() -> Result<Vec<u8>, String> {
    let bytes = nabu_static_directory()?.encode()?;
    if bytes.len() != DIRECTORY_PACKED_BYTES || wmfw_sha256::hex_digest(&bytes) != DIRECTORY_SHA256 {
        return Err("source-generated static directory differs from pinned variant".into());
    }
    Ok(bytes)
}

/// Authored WMFW envelope plus typed static directory; every other DSP byte
/// stays missing. `build_complete` still refuses to emit a complete image.
pub fn partial_with_directory() -> Result<ImagePlan, String> {
    let mut plan = wmfw_plan::partial_container()?;
    plan.place("reference-derived typed static XM directory with explicit padding",
        DIRECTORY_FILE_OFFSET, &build_static_directory()?)?;
    Ok(plan)
}

pub fn verify_reference(reference: &[u8]) -> Result<(), String> {
    if wmfw_sha256::hex_digest(reference) != wmfw_external_payload::REFERENCE_SHA256 {
        return Err("HALO reference SHA-256 mismatch".into());
    }
    wmfw_plan::verify_container(reference)?;
    partial_with_directory()?.compare_known(reference)
}

/// Optional complete *external repack*. Only the first XM directory is
/// source-generated here; all other nonzero DSP payloads stay explicit inputs.
pub fn build_with_static_directory<F, Y>(mut read_other: F, read_y: Y) -> Result<Vec<u8>, String>
where F: FnMut(wmfw_external_payload::OpaqueRegion) -> Result<Vec<u8>, String>,
      Y: FnMut(wmfw_compact_y::YPart) -> Result<Vec<u8>, String> {
    let directory = build_static_directory()?;
    wmfw_compact_y::build_with_compact_y(|region| {
        if region.file_offset == DIRECTORY_FILE_OFFSET {
            if region.kind != 0x11 || region.size != DIRECTORY_PACKED_BYTES {
                return Err("unexpected static directory region".into());
            }
            Ok(directory.clone())
        } else { read_other(region) }
    }, read_y)
}

pub fn assemble_directory(path: &Path) -> Result<Vec<u8>, String> {
    build_with_static_directory(
        |r| fs::read(path.join(r.filename())).map_err(|e| format!("{}: {e}", r.filename())),
        |r| fs::read(path.join(r.name)).map_err(|e| format!("{}: {e}", r.name)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_uses_authored_control_lengths_and_explicit_padding() {
        let d = nabu_static_directory().unwrap();
        assert_eq!((d.header.xm_size, d.algorithms[0].xm_size, d.algorithms[1].xm_size), (14, 77, 75));
        assert_eq!(d.header.algorithm_count, 2);
        let words = d.words().unwrap();
        assert_eq!(words.len(), 24);
        assert_eq!(words[22], 0x00be_dead);
        assert_eq!(words[23], 0);
        let bytes = build_static_directory().unwrap();
        assert_eq!(wmfw_sha256::hex_digest(&bytes), DIRECTORY_SHA256);
        assert_eq!(&bytes[68..71], &[0, 0, 0]);
        assert_eq!(DIRECTORY_FIELD_BYTES + DIRECTORY_PADDING_BYTES, bytes.len());
        assert_eq!(wmfw_halo_memory::unpack_words(HaloMemoryKind::PackedX, &bytes).unwrap(), words);
    }

    #[test]
    fn rejects_unknown_field_width_and_allocation_errors() {
        let mut d = nabu_static_directory().unwrap();
        d.header.vendor_id = 1 << 24;
        assert!(d.encode().unwrap_err().contains("24-bit"));
        let mut d = nabu_static_directory().unwrap();
        d.algorithms[0].xm_base = d.header.xm_base;
        assert!(d.encode().unwrap_err().contains("overlap"));
        let mut d = nabu_static_directory().unwrap();
        d.header.algorithm_count = 3;
        assert!(d.encode().is_err());
    }

    #[test]
    fn partial_builder_leaves_program_and_unimplemented_data_missing() {
        let p = partial_with_directory().unwrap();
        assert_eq!(p.filled_bytes(), 3984 + 72);
        assert_eq!(p.holes().first().unwrap(), &(0xb8..0x2c8));
        assert!(p.holes().contains(&(0x278c..0x76c4)));
        assert!(p.build_complete().is_err());
        assert_eq!(EXTERNAL_BYTES_WITH_DIRECTORY, 24432);
        assert!(verify_reference(&vec![0; wmfw_plan::FILE_SIZE]).is_err());
    }
}
