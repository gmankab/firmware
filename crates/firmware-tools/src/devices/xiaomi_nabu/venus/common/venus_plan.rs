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

#[cfg(test)]
const COVERAGE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../reference/xiaomi-nabu/coverage.toml"));
pub const FILE_BASE: usize = 0x4000;

#[cfg(test)]
fn canonical_record() -> Result<&'static str, String> {
    COVERAGE.split("[[images]]").find(|record| {
        record.lines().any(|line| line.trim() == "name = \"venus.mbn\"")
    }).ok_or_else(|| "missing canonical Venus coverage record".into())
}

#[cfg(test)]
fn field<'a>(record: &'a str, name: &str) -> Result<&'a str, String> {
    record.lines().find_map(|line| {
        let (key, value) = line.split_once('=')?;
        (key.trim() == name).then_some(value.trim())
    }).ok_or_else(|| format!("missing Venus {name}"))
}

pub fn full_file() -> Result<ImagePlan, String> {
    use super::{venus_runtime_bridge, venus_runtime_forward, venus_runtime_seam,
        venus_runtime_late, venus_runtime_mid, venus_upper_dispatch, venus_upper_controller_continuation, venus_upper_controller_parameters,
        venus_upper_controller_type_selection, venus_upper_controller_aggregate,
        venus_upper_controller_budget_entry, venus_upper_controller_budget_records, venus_upper_controller_thresholds, venus_upper_controller_epilogue, venus_upper_weighted_metrics, venus_upper_frame_distortion, venus_upper_ring_ingestion, venus_upper_update_entry, venus_upper_allocation_transition, venus_upper_state_smoothing, venus_upper_indexed_transition, venus_upper_selector_state, venus_upper_paired_accumulator, venus_upper_scaled_accumulation, venus_upper_candidate_bounds, venus_upper_adjustment_records, venus_upper_target_bits_clamps, venus_upper_mode_qp_selection, venus_upper_qp_commit_caps, venus_upper_history_qp_candidates, venus_upper_scene_target_model, venus_upper_subgop_transition, venus_upper_adaptive_rate_update, venus_upper_history_packed_tables, venus_upper_frame_output_record, venus_upper_history_lookup_helpers, venus_heap_bin_classifiers, venus_heap_free_insertion, venus_heap_lifetime, venus_heap_allocation, venus_heap_bin_traversal, venus_vppsg_lifecycle, venus_vppsg_object_transform, venus_vsp_task_generation, venus_vppsg_task_packing, venus_tail_program};
    use super::{venus_heap_release, venus_heap_reallocation, venus_heap_queries_iterator, venus_heap_statistics_callbacks, venus_heap_interrupt_helpers};
    use super::{venus_service_queue_helpers, venus_service_state_dispatch, venus_service_loop_setup, venus_service_register_output};
    use super::{venus_descriptor_state_writer, venus_descriptor_context_writer, venus_descriptor_argument_writer, venus_descriptor_flag_writer};
    use super::{venus_descriptor_paired_variants, venus_descriptor_multi_record, venus_descriptor_offset_variants, venus_descriptor_dispatch_tail};
    use super::{venus_vppsg_process, venus_vspap_lifecycle, venus_vspap_process, venus_worker_extra_data};
    use super::venus_worker_process;
    use super::{venus_worker_continuation, venus_bitstream_primitives, venus_bitstream_selectors, venus_bitstream_writer, venus_h264_header_writer, venus_h264_parameter_writer, venus_h264_auxiliary_writers};
    use super::{venus_hevc_state_writers, venus_hevc_reference_sets, venus_hevc_parameter_records, venus_hevc_header_writers};
    use super::{venus_hevc_hdr_metadata, venus_vpx_header_tables, venus_vpx_header_commit};
    use super::{venus_vbuffer_manager, venus_vbuffer_release_queries, venus_vbuffer_external_attach, venus_vbuffer_array_attach};
    use super::{venus_arithmetic_wrappers, venus_thumb_normalization, venus_thumb_binary64_add, venus_thumb_binary32_add, venus_thumb_float_compare, venus_thumb_binary64_divide, venus_thumb_binary32_divide, venus_thumb_numeric_conversions, venus_thumb_binary64_multiply, venus_thumb_binary32_multiply, venus_thumb_integer_division};
    use super::{venus_vbuffer_acquire, venus_vbuffer_accessors, venus_dispatch_allocator, venus_runtime_utilities, venus_vpx_probability_init, venus_vpx_context_tail};
    let size = crate::firmware_manifest_identity::canonical_image("venus.mbn")?.size;
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
    place!(venus_upper_controller_parameters, venus_upper_controller_parameters::START);
    place!(venus_upper_controller_type_selection, venus_upper_controller_type_selection::START);
    place!(venus_upper_controller_aggregate, venus_upper_controller_aggregate::START);
    place!(venus_upper_controller_budget_entry, venus_upper_controller_budget_entry::START);
    place!(venus_upper_controller_budget_records, venus_upper_controller_budget_records::START);
    place!(venus_upper_controller_thresholds, venus_upper_controller_thresholds::START);
    place!(venus_upper_controller_epilogue, venus_upper_controller_epilogue::START);
    place!(venus_upper_dispatch, venus_upper_dispatch::START);
    place!(venus_upper_weighted_metrics, venus_upper_weighted_metrics::START);
    place!(venus_upper_frame_distortion, venus_upper_frame_distortion::START);
    place!(venus_upper_ring_ingestion, venus_upper_ring_ingestion::START);
    place!(venus_upper_update_entry, venus_upper_update_entry::START);
    place!(venus_upper_allocation_transition, venus_upper_allocation_transition::START);
    place!(venus_upper_state_smoothing, venus_upper_state_smoothing::START);
    place!(venus_upper_indexed_transition, venus_upper_indexed_transition::START);
    place!(venus_upper_selector_state, venus_upper_selector_state::START);
    place!(venus_upper_paired_accumulator, venus_upper_paired_accumulator::START);
    place!(venus_upper_scaled_accumulation, venus_upper_scaled_accumulation::START);
    place!(venus_upper_candidate_bounds, venus_upper_candidate_bounds::START);
    place!(venus_upper_adjustment_records, venus_upper_adjustment_records::START);
    place!(venus_upper_target_bits_clamps, venus_upper_target_bits_clamps::START);
    place!(venus_upper_mode_qp_selection, venus_upper_mode_qp_selection::START);
    place!(venus_upper_qp_commit_caps, venus_upper_qp_commit_caps::START);
    place!(venus_upper_history_qp_candidates, venus_upper_history_qp_candidates::START);
    place!(venus_upper_scene_target_model, venus_upper_scene_target_model::START);
    place!(venus_upper_subgop_transition, venus_upper_subgop_transition::START);
    place!(venus_upper_adaptive_rate_update, venus_upper_adaptive_rate_update::START);
    place!(venus_upper_history_packed_tables, venus_upper_history_packed_tables::START);
    place!(venus_upper_frame_output_record, venus_upper_frame_output_record::START);
    place!(venus_upper_history_lookup_helpers, venus_upper_history_lookup_helpers::START);
    place!(venus_heap_bin_classifiers, venus_heap_bin_classifiers::START);
    place!(venus_heap_free_insertion, venus_heap_free_insertion::START);
    place!(venus_heap_lifetime, venus_heap_lifetime::START);
    place!(venus_heap_allocation, venus_heap_allocation::START);
    place!(venus_heap_bin_traversal, venus_heap_bin_traversal::START);
    place!(venus_heap_release, venus_heap_release::START);
    place!(venus_heap_reallocation, venus_heap_reallocation::START);
    place!(venus_heap_queries_iterator, venus_heap_queries_iterator::START);
    place!(venus_heap_statistics_callbacks, venus_heap_statistics_callbacks::START);
    place!(venus_heap_interrupt_helpers, venus_heap_interrupt_helpers::START);
    place!(venus_service_queue_helpers, venus_service_queue_helpers::START);
    place!(venus_service_state_dispatch, venus_service_state_dispatch::START);
    place!(venus_service_loop_setup, venus_service_loop_setup::START);
    place!(venus_service_register_output, venus_service_register_output::START);
    place!(venus_descriptor_state_writer, venus_descriptor_state_writer::START);
    place!(venus_descriptor_context_writer, venus_descriptor_context_writer::START);
    place!(venus_descriptor_argument_writer, venus_descriptor_argument_writer::START);
    place!(venus_descriptor_flag_writer, venus_descriptor_flag_writer::START);
    place!(venus_descriptor_paired_variants, venus_descriptor_paired_variants::START);
    place!(venus_descriptor_multi_record, venus_descriptor_multi_record::START);
    place!(venus_descriptor_offset_variants, venus_descriptor_offset_variants::START);
    place!(venus_descriptor_dispatch_tail, venus_descriptor_dispatch_tail::START);
    place!(venus_vppsg_process, venus_vppsg_process::START);
    place!(venus_vspap_lifecycle, venus_vspap_lifecycle::START);
    place!(venus_vspap_process, venus_vspap_process::START);
    place!(venus_worker_extra_data, venus_worker_extra_data::START);
    place!(venus_worker_process, venus_worker_process::START);
    place!(venus_worker_continuation, venus_worker_continuation::START);
    place!(venus_bitstream_primitives, venus_bitstream_primitives::START);
    place!(venus_bitstream_selectors, venus_bitstream_selectors::START);
    place!(venus_bitstream_writer, venus_bitstream_writer::START);
    place!(venus_h264_header_writer, venus_h264_header_writer::START);
    place!(venus_h264_parameter_writer, venus_h264_parameter_writer::START);
    place!(venus_h264_auxiliary_writers, venus_h264_auxiliary_writers::START);
    place!(venus_hevc_state_writers, venus_hevc_state_writers::START);
    place!(venus_hevc_reference_sets, venus_hevc_reference_sets::START);
    place!(venus_hevc_parameter_records, venus_hevc_parameter_records::START);
    place!(venus_hevc_header_writers, venus_hevc_header_writers::START);
    place!(venus_hevc_hdr_metadata, venus_hevc_hdr_metadata::START);
    place!(venus_vpx_header_tables, venus_vpx_header_tables::START);
    place!(venus_vpx_header_commit, venus_vpx_header_commit::START);
    place!(venus_vbuffer_manager, venus_vbuffer_manager::START);
    place!(venus_vbuffer_release_queries, venus_vbuffer_release_queries::START);
    place!(venus_vbuffer_external_attach, venus_vbuffer_external_attach::START);
    place!(venus_vbuffer_array_attach, venus_vbuffer_array_attach::START);
    place!(venus_vbuffer_acquire, venus_vbuffer_acquire::START);
    place!(venus_vbuffer_accessors, venus_vbuffer_accessors::START);
    place!(venus_dispatch_allocator, venus_dispatch_allocator::START);
    place!(venus_runtime_utilities, venus_runtime_utilities::START);
    place!(venus_arithmetic_wrappers, venus_arithmetic_wrappers::START);
    place!(venus_thumb_normalization, venus_thumb_normalization::START);
    place!(venus_thumb_binary64_add, venus_thumb_binary64_add::START);
    place!(venus_thumb_binary32_add, venus_thumb_binary32_add::START);
    place!(venus_thumb_float_compare, venus_thumb_float_compare::START);
    place!(venus_thumb_binary64_divide, venus_thumb_binary64_divide::START);
    place!(venus_thumb_binary32_divide, venus_thumb_binary32_divide::START);
    place!(venus_thumb_numeric_conversions, venus_thumb_numeric_conversions::START);
    place!(venus_thumb_binary64_multiply, venus_thumb_binary64_multiply::START);
    place!(venus_thumb_binary32_multiply, venus_thumb_binary32_multiply::START);
    place!(venus_thumb_integer_division, venus_thumb_integer_division::START);
    place!(venus_vpx_probability_init, venus_vpx_probability_init::START);
    place!(venus_vpx_context_tail, venus_vpx_context_tail::START);
    plan.place("venus_vspap_lifecycle_helpers", FILE_BASE + venus_vspap_lifecycle::LIFECYCLE_START, &venus_vspap_lifecycle::lifecycle_fragment()?)?;
    place!(venus_vppsg_lifecycle, venus_vppsg_lifecycle::START);
    place!(venus_vppsg_object_transform, venus_vppsg_object_transform::START);
    place!(venus_vsp_task_generation, venus_vsp_task_generation::START);
    place!(venus_vppsg_task_packing, venus_vppsg_task_packing::START);
    place!(venus_tail_program, venus_tail_program::START);
    Ok(plan)
}

pub fn compare_pinned_reference(reference: &[u8]) -> Result<(), String> {
    let identity = crate::firmware_manifest_identity::canonical_image("venus.mbn")?;
    if reference.len() != identity.size {
        return Err("wrong full Venus reference length".into());
    }
    if crate::wmfw_sha256::hex_digest(reference) != identity.sha256 {
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
        assert_eq!(groups, 16);
        let mut actual = vec![false; plan.total_bytes()];
        for region in plan.regions() {
            assert_eq!(region.origin, crate::image_plan::RegionOrigin::Generated);
            let span = region.offset..region.offset + region.len;
            assert!(actual[span.clone()].iter().all(|filled| !filled));
            actual[span].fill(true);
        }
        let credit = field(record, "credit_bytes").unwrap().parse::<usize>().unwrap();
        assert_eq!(credit, 668880);
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
    fn full_file_rejects_corrupted_generated_region() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        compare_pinned_reference(&reference).unwrap();
        let source = full_file().unwrap();
        let mut corrupted = ImagePlan::new(source.total_bytes()).unwrap();
        let target = source.regions().iter()
            .find(|region| region.name == "venus_upper_controller_continuation").unwrap();
        for region in source.regions() {
            let mut bytes = source.generated_slice(region.offset..region.offset + region.len).unwrap();
            if region.name == target.name {
                bytes[0] ^= 1;
            }
            corrupted.place(&region.name, region.offset, &bytes).unwrap();
        }
        assert_eq!(corrupted.compare_known(&reference).unwrap_err(),
            format!("byte mismatch in {} at image offset {} (0x{:x})",
                target.name, target.offset, target.offset));
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
