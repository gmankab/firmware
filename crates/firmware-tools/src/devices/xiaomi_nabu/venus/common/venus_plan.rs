// SPDX-License-Identifier: GPL-3.0-only
//! Sparse full-file reconstruction; manifest credit and pending emitted bytes
//! are checked separately. Unknown firmware and authentication remain holes.
use crate::image_plan::ImagePlan;
use super::{venus_boot, venus_runtime};

pub const START_FILE_OFFSET: usize = venus_boot::FILE_OFFSET;
pub const CONTIGUOUS_SIZE: usize = venus_boot::FIRST_PAGE_SIZE + venus_runtime::PREFIX_SIZE;

pub fn first_contiguous_region() -> Result<ImagePlan, String> {
    if venus_runtime::FILE_OFFSET !=
       START_FILE_OFFSET + venus_boot::FIRST_PAGE_SIZE {
        return Err("Venus first runtime block is not adjacent to boot page".into());
    }
    let mut plan = ImagePlan::new(CONTIGUOUS_SIZE)?;
    plan.place("typed ARM32 boot page with named literals and zero fill",
        0, &venus_boot::first_page()?)?;
    plan.place("typed ARM32 runtime entry stubs",
        venus_boot::FIRST_PAGE_SIZE, &venus_runtime::prefix()?)?;
    if !plan.holes().is_empty() {
        return Err("Venus early boot image has uncovered bytes".into());
    }
    Ok(plan)
}

#[path = "../upper/venus_upper.rs"]
pub mod venus_upper;
#[path = "../upper/venus_upper_alt_occupancy.rs"]
pub mod venus_upper_alt_occupancy;
#[path = "../upper/venus_upper_color_unpack.rs"]
pub mod venus_upper_color_unpack;
#[path = "../upper/venus_upper_control_flags.rs"]
pub mod venus_upper_control_flags;
#[path = "../upper/venus_upper_controller_alloc.rs"]
pub mod venus_upper_controller_alloc;
#[path = "../upper/venus_upper_controller_limits.rs"]
pub mod venus_upper_controller_limits;
#[path = "../upper/venus_upper_controller_limits_tail.rs"]
pub mod venus_upper_controller_limits_tail;
#[path = "../upper/venus_upper_controller_run_prefix.rs"]
pub mod venus_upper_controller_run_prefix;
#[path = "../upper/venus_upper_controller_run_tail.rs"]
pub mod venus_upper_controller_run_tail;
#[path = "../upper/venus_upper_controller_small_helpers.rs"]
pub mod venus_upper_controller_small_helpers;
#[path = "../upper/venus_upper_magnitude.rs"]
pub mod venus_upper_magnitude;
#[path = "../upper/venus_upper_occupancy_commit.rs"]
pub mod venus_upper_occupancy_commit;
#[path = "../upper/venus_upper_quality.rs"]
pub mod venus_upper_quality;
#[path = "../upper/venus_upper_record_append.rs"]
pub mod venus_upper_record_append;
#[path = "../upper/venus_upper_record_metrics.rs"]
pub mod venus_upper_record_metrics;
#[path = "../upper/venus_upper_record_select.rs"]
pub mod venus_upper_record_select;
#[path = "../upper/venus_upper_throughput_tiers.rs"]
pub mod venus_upper_throughput_tiers;

const COVERAGE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../reference/xiaomi-nabu/coverage.toml"));
pub const FILE_BASE: usize = 0x4000;

fn canonical_record() -> Result<&'static str, String> {
    COVERAGE.split("[[images]]").find(|record| {
        record.lines().any(|line| line.trim() == "name = \"venus.mbn\"")
    }).ok_or_else(|| "missing canonical Venus coverage record".into())
}

fn field<'a>(record: &'a str, name: &str) -> Result<&'a str, String> {
    record.lines().find_map(|line| {
        let (key, value) = line.split_once('=')?;
        (key.trim() == name).then_some(value.trim())
    }).ok_or_else(|| format!("missing Venus {name}"))
}

pub fn full_file() -> Result<ImagePlan, String> {
    use super::{venus_runtime_bridge, venus_runtime_forward, venus_runtime_seam,
        venus_runtime_late, venus_runtime_mid, venus_upper_dispatch, venus_upper_controller_continuation, venus_tail_program};
    let size = field(canonical_record()?, "size")?.parse::<usize>().map_err(|e| e.to_string())?;
    let mut plan = ImagePlan::new(size)?;
    plan.place("venus_boot", venus_boot::FILE_OFFSET, &venus_boot::first_page()?)?;
    plan.place("venus_runtime", venus_runtime::FILE_OFFSET, &venus_runtime::prefix()?)?;
    macro_rules! place {
        ($module:ident, $start:expr) => {
            plan.place(stringify!($module), FILE_BASE + $start, &$module::fragment().map_err(|error| format!("{}: {error}", stringify!($module)))?)?;
        };
    }
    place!(venus_runtime_bridge, venus_runtime_bridge::START);
    place!(venus_runtime_forward, venus_runtime_forward::START);
    place!(venus_runtime_seam, venus_runtime_seam::START);
    place!(venus_runtime_late, venus_runtime_late::START);
    place!(venus_runtime_mid, venus_runtime_mid::START);
    for (name, start, bytes) in venus_controller::fragments()? {
        plan.place(name, FILE_BASE + start, &bytes)?;
    }
    place!(venus_upper, venus_upper::VERIFIED_START);
    place!(venus_upper_alt_occupancy, venus_upper_alt_occupancy::START);
    place!(venus_upper_color_unpack, venus_upper_color_unpack::START);
    place!(venus_upper_control_flags, venus_upper_control_flags::START);
    place!(venus_upper_controller_alloc, venus_upper_controller_alloc::START);
    place!(venus_upper_controller_limits, venus_upper_controller_limits::START);
    place!(venus_upper_controller_limits_tail, venus_upper_controller_limits_tail::START);
    place!(venus_upper_controller_run_prefix, venus_upper_controller_run_prefix::START);
    place!(venus_upper_controller_run_tail, venus_upper_controller_run_tail::START);
    place!(venus_upper_controller_small_helpers, venus_upper_controller_small_helpers::START);
    place!(venus_upper_magnitude, venus_upper_magnitude::START);
    place!(venus_upper_occupancy_commit, venus_upper_occupancy_commit::START);
    place!(venus_upper_quality, venus_upper_quality::START);
    place!(venus_upper_record_append, venus_upper_record_append::START);
    place!(venus_upper_record_metrics, venus_upper_record_metrics::START);
    place!(venus_upper_record_select, venus_upper_record_select::START);
    place!(venus_upper_throughput_tiers, venus_upper_throughput_tiers::START);
    place!(venus_upper_controller_continuation, venus_upper_controller_continuation::START);
    place!(venus_upper_dispatch, venus_upper_dispatch::START);
    place!(venus_tail_program, venus_tail_program::START);
    Ok(plan)
}

pub fn compare_pinned_reference(reference: &[u8]) -> Result<(), String> {
    let record = canonical_record()?;
    let size = field(record, "size")?.parse::<usize>().map_err(|e| e.to_string())?;
    if reference.len() != size {
        return Err("wrong full Venus reference length".into());
    }
    if crate::wmfw_sha256::hex_digest(reference) != field(record, "sha256")?.trim_matches('"') {
        return Err("wrong pinned Venus reference SHA-256".into());
    }
    full_file()?.compare_known(reference)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_file_union_matches_canonical_manifest() {
        let record = canonical_record().unwrap();
        let plan = full_file().unwrap();
        assert_eq!(plan.total_bytes(), 1159200);
        let mut expected = vec![false; plan.total_bytes()];
        let mut groups = 0;
        for region in record.split("[[images.regions]]").skip(1) {
            let number = |key| usize::from_str_radix(field(region, key).unwrap().trim_start_matches("0x"), 16).unwrap();
            let span = FILE_BASE + number("start")..FILE_BASE + number("end");
            assert!(expected[span.clone()].iter().all(|filled| !filled));
            expected[span].fill(true);
            groups += 1;
        }
        assert_eq!(groups, 11);
        let mut actual = vec![false; plan.total_bytes()];
        for region in plan.regions() {
            assert_eq!(region.origin, crate::image_plan::RegionOrigin::Generated);
            let span = region.offset..region.offset + region.len;
            assert!(actual[span.clone()].iter().all(|filled| !filled));
            actual[span].fill(true);
        }
        let credit = field(record, "credit_bytes").unwrap().parse::<usize>().unwrap();
        assert_eq!(credit, 532640);
        assert_eq!(expected.iter().filter(|&&filled| filled).count(), credit);

        assert_eq!(actual.iter().zip(&expected).position(|(a, b)| a != b), None);
        assert_eq!(plan.filled_bytes(), credit);
        assert_eq!(plan.coverage().generated_bytes, credit);
        assert_eq!(plan.coverage().externally_supplied_bytes, 0);
        assert_eq!(plan.coverage().missing_bytes, plan.total_bytes() - credit);
        assert!(plan.build_complete().is_err());
        assert!(plan.build_independently_generated().is_err());
        for hole in plan.holes() {
            assert!(plan.generated_slice(hole).is_err());
        }
    }

    #[test]
    fn full_file_optional_pinned_reference() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        compare_pinned_reference(&reference).unwrap();
    }

    #[test]
    fn full_file_rejects_unpinned_reference() {
        assert!(compare_pinned_reference(&[]).is_err());
        assert!(compare_pinned_reference(&vec![0; 1159200]).is_err());
    }

    #[test]
    fn strict_contiguous_venus_boot_region() {
        let plan = first_contiguous_region().unwrap();
        assert_eq!(plan.total_bytes(), 33936);
        assert_eq!(plan.filled_bytes(), 33936);
        assert!(plan.holes().is_empty());
        assert_eq!(plan.regions().len(), 2);
        assert!(plan.build_complete().is_ok());
    }
}
