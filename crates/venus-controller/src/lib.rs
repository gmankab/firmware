// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller A32 reconstruction; independently compiled.
#[path = "venus_controller_codec_class_values.rs"]
pub mod venus_controller_codec_class_values;
#[path = "venus_controller_codec_controller.rs"]
pub mod venus_controller_codec_controller;
#[path = "venus_controller_codec_mode_packets.rs"]
pub mod venus_controller_codec_mode_packets;
#[path = "venus_controller_codec_table_bundles.rs"]
pub mod venus_controller_codec_table_bundles;
#[path = "venus_controller_command_serializer.rs"]
pub mod venus_controller_command_serializer;
#[path = "venus_controller_component_geometry.rs"]
pub mod venus_controller_component_geometry;
#[path = "venus_controller_control_fields.rs"]
pub mod venus_controller_control_fields;
#[path = "venus_controller_extended_controls.rs"]
pub mod venus_controller_extended_controls;
#[path = "venus_controller_feature_extension.rs"]
pub mod venus_controller_feature_extension;
#[path = "venus_controller_flags.rs"]
pub mod venus_controller_flags;
#[path = "venus_controller_format_registers.rs"]
pub mod venus_controller_format_registers;
#[path = "venus_controller_late_controller_masks.rs"]
pub mod venus_controller_late_controller_masks;
#[path = "venus_controller_middle.rs"]
pub mod venus_controller_middle;
#[path = "venus_controller_packet_features.rs"]
pub mod venus_controller_packet_features;
#[path = "venus_controller_packet_sequence.rs"]
pub mod venus_controller_packet_sequence;
#[path = "venus_controller_packets.rs"]
pub mod venus_controller_packets;
#[path = "venus_controller_plane_strides.rs"]
pub mod venus_controller_plane_strides;
#[path = "venus_controller_prefix.rs"]
pub mod venus_controller_prefix;
#[path = "venus_controller_signed_frame_records.rs"]
pub mod venus_controller_signed_frame_records;
#[path = "venus_controller_span_4c000.rs"]
pub mod venus_controller_span_4c000;
#[path = "venus_controller_span_4e000.rs"]
pub mod venus_controller_span_4e000;
#[path = "venus_controller_span_4f000.rs"]
pub mod venus_controller_span_4f000;
#[path = "venus_controller_span_50000.rs"]
pub mod venus_controller_span_50000;
#[path = "venus_controller_span_51000.rs"]
pub mod venus_controller_span_51000;
#[path = "venus_controller_span_52000.rs"]
pub mod venus_controller_span_52000;
#[path = "venus_controller_span_53000.rs"]
pub mod venus_controller_span_53000;
#[path = "venus_controller_span_53400.rs"]
pub mod venus_controller_span_53400;
#[path = "venus_controller_span_53944.rs"]
pub mod venus_controller_span_53944;
#[path = "venus_controller_span_53b24.rs"]
pub mod venus_controller_span_53b24;
#[path = "venus_controller_span_54028.rs"]
pub mod venus_controller_span_54028;
#[path = "venus_controller_span_54854.rs"]
pub mod venus_controller_span_54854;
#[path = "venus_controller_span_54c1c.rs"]
pub mod venus_controller_span_54c1c;
#[path = "venus_controller_span_54cfc.rs"]
pub mod venus_controller_span_54cfc;
#[path = "venus_controller_span_55400.rs"]
pub mod venus_controller_span_55400;
#[path = "venus_controller_span_55800.rs"]
pub mod venus_controller_span_55800;
#[path = "venus_controller_span_56000.rs"]
pub mod venus_controller_span_56000;
#[path = "venus_controller_span_56800.rs"]
pub mod venus_controller_span_56800;
#[path = "venus_controller_span_57000.rs"]
pub mod venus_controller_span_57000;
#[path = "venus_controller_span_58000.rs"]
pub mod venus_controller_span_58000;
#[path = "venus_controller_span_59000.rs"]
pub mod venus_controller_span_59000;
#[path = "venus_controller_span_5a000.rs"]
pub mod venus_controller_span_5a000;
#[path = "venus_controller_span_5b000.rs"]
pub mod venus_controller_span_5b000;
#[path = "venus_controller_span_5c000.rs"]
pub mod venus_controller_span_5c000;
#[path = "venus_controller_span_5d000.rs"]
pub mod venus_controller_span_5d000;
#[path = "venus_controller_span_5e000.rs"]
pub mod venus_controller_span_5e000;
#[path = "venus_controller_span_5f000.rs"]
pub mod venus_controller_span_5f000;
#[path = "venus_controller_span_60000.rs"]
pub mod venus_controller_span_60000;
#[path = "venus_controller_span_61000.rs"]
pub mod venus_controller_span_61000;
#[path = "venus_controller_span_62000.rs"]
pub mod venus_controller_span_62000;
#[path = "venus_controller_span_63000.rs"]
pub mod venus_controller_span_63000;
#[path = "venus_controller_span_64000.rs"]
pub mod venus_controller_span_64000;
#[path = "venus_controller_span_65000.rs"]
pub mod venus_controller_span_65000;
#[path = "venus_controller_span_66000.rs"]
pub mod venus_controller_span_66000;
#[path = "venus_controller_span_67000.rs"]
pub mod venus_controller_span_67000;
#[path = "venus_controller_stream_class_commands.rs"]
pub mod venus_controller_stream_class_commands;

pub fn fragments() -> Result<Vec<(&'static str, usize, Vec<u8>)>, String> {
    let fragments = vec![
        ("venus_controller_prefix", venus_controller_prefix::START, venus_controller_prefix::fragment().map_err(|error| format!("venus_controller_prefix: {error}"))?),
        ("venus_controller_middle", venus_controller_middle::FIRST_START, venus_controller_middle::first_fragment().map_err(|error| format!("venus_controller_middle: {error}"))?),
        ("venus_controller_codec_controller", venus_controller_codec_controller::START, venus_controller_codec_controller::fragment().map_err(|error| format!("venus_controller_codec_controller: {error}"))?),
        ("venus_controller_format_registers", venus_controller_format_registers::START, venus_controller_format_registers::fragment().map_err(|error| format!("venus_controller_format_registers: {error}"))?),
        ("venus_controller_packets", venus_controller_packets::START, venus_controller_packets::fragment().map_err(|error| format!("venus_controller_packets: {error}"))?),
        ("venus_controller_flags", venus_controller_flags::START, venus_controller_flags::fragment().map_err(|error| format!("venus_controller_flags: {error}"))?),
        ("venus_controller_codec_mode_packets", venus_controller_codec_mode_packets::START, venus_controller_codec_mode_packets::fragment().map_err(|error| format!("venus_controller_codec_mode_packets: {error}"))?),
        ("venus_controller_command_serializer", venus_controller_command_serializer::START, venus_controller_command_serializer::fragment().map_err(|error| format!("venus_controller_command_serializer: {error}"))?),
        ("venus_controller_packet_sequence", venus_controller_packet_sequence::START, venus_controller_packet_sequence::fragment().map_err(|error| format!("venus_controller_packet_sequence: {error}"))?),
        ("venus_controller_extended_controls", venus_controller_extended_controls::START, venus_controller_extended_controls::fragment().map_err(|error| format!("venus_controller_extended_controls: {error}"))?),
        ("venus_controller_control_fields", venus_controller_control_fields::START, venus_controller_control_fields::fragment().map_err(|error| format!("venus_controller_control_fields: {error}"))?),
        ("venus_controller_codec_table_bundles", venus_controller_codec_table_bundles::START, venus_controller_codec_table_bundles::fragment().map_err(|error| format!("venus_controller_codec_table_bundles: {error}"))?),
        ("venus_controller_feature_extension", venus_controller_feature_extension::START, venus_controller_feature_extension::fragment().map_err(|error| format!("venus_controller_feature_extension: {error}"))?),
        ("venus_controller_packet_features", venus_controller_packet_features::START, venus_controller_packet_features::fragment().map_err(|error| format!("venus_controller_packet_features: {error}"))?),
        ("venus_controller_component_geometry", venus_controller_component_geometry::START, venus_controller_component_geometry::fragment().map_err(|error| format!("venus_controller_component_geometry: {error}"))?),
        ("venus_controller_plane_strides", venus_controller_plane_strides::START, venus_controller_plane_strides::fragment().map_err(|error| format!("venus_controller_plane_strides: {error}"))?),
        ("venus_controller_late_controller_masks", venus_controller_late_controller_masks::START, venus_controller_late_controller_masks::fragment().map_err(|error| format!("venus_controller_late_controller_masks: {error}"))?),
        ("venus_controller_codec_class_values", venus_controller_codec_class_values::START, venus_controller_codec_class_values::fragment().map_err(|error| format!("venus_controller_codec_class_values: {error}"))?),
        ("venus_controller_stream_class_commands", venus_controller_stream_class_commands::START, venus_controller_stream_class_commands::fragment().map_err(|error| format!("venus_controller_stream_class_commands: {error}"))?),
        ("venus_controller_span_4c000", venus_controller_span_4c000::START, venus_controller_span_4c000::fragment().map_err(|error| format!("venus_controller_span_4c000: {error}"))?),
        ("venus_controller_signed_frame_records", venus_controller_signed_frame_records::START, venus_controller_signed_frame_records::fragment().map_err(|error| format!("venus_controller_signed_frame_records: {error}"))?),
        ("venus_controller_span_4e000", venus_controller_span_4e000::START, venus_controller_span_4e000::fragment().map_err(|error| format!("venus_controller_span_4e000: {error}"))?),
        ("venus_controller_span_4f000", venus_controller_span_4f000::START, venus_controller_span_4f000::fragment().map_err(|error| format!("venus_controller_span_4f000: {error}"))?),
        ("venus_controller_span_50000", venus_controller_span_50000::START, venus_controller_span_50000::fragment().map_err(|error| format!("venus_controller_span_50000: {error}"))?),
        ("venus_controller_span_51000", venus_controller_span_51000::START, venus_controller_span_51000::fragment().map_err(|error| format!("venus_controller_span_51000: {error}"))?),
        ("venus_controller_span_52000", venus_controller_span_52000::START, venus_controller_span_52000::fragment().map_err(|error| format!("venus_controller_span_52000: {error}"))?),
        ("venus_controller_span_53000", venus_controller_span_53000::START, venus_controller_span_53000::fragment().map_err(|error| format!("venus_controller_span_53000: {error}"))?),
        ("venus_controller_span_53400", venus_controller_span_53400::START, venus_controller_span_53400::fragment().map_err(|error| format!("venus_controller_span_53400: {error}"))?),
        ("venus_controller_span_53944", venus_controller_span_53944::START, venus_controller_span_53944::fragment().map_err(|error| format!("venus_controller_span_53944: {error}"))?),
        ("venus_controller_span_53b24", venus_controller_span_53b24::START, venus_controller_span_53b24::fragment().map_err(|error| format!("venus_controller_span_53b24: {error}"))?),
        ("venus_controller_span_54028", venus_controller_span_54028::START, venus_controller_span_54028::fragment().map_err(|error| format!("venus_controller_span_54028: {error}"))?),
        ("venus_controller_span_54854", venus_controller_span_54854::START, venus_controller_span_54854::fragment().map_err(|error| format!("venus_controller_span_54854: {error}"))?),
        ("venus_controller_span_54c1c", venus_controller_span_54c1c::START, venus_controller_span_54c1c::fragment().map_err(|error| format!("venus_controller_span_54c1c: {error}"))?),
        ("venus_controller_span_54cfc", venus_controller_span_54cfc::START, venus_controller_span_54cfc::fragment().map_err(|error| format!("venus_controller_span_54cfc: {error}"))?),
        ("venus_controller_span_55400", venus_controller_span_55400::START, venus_controller_span_55400::fragment().map_err(|error| format!("venus_controller_span_55400: {error}"))?),
        ("venus_controller_span_55800", venus_controller_span_55800::START, venus_controller_span_55800::fragment().map_err(|error| format!("venus_controller_span_55800: {error}"))?),
        ("venus_controller_span_56000", venus_controller_span_56000::START, venus_controller_span_56000::fragment().map_err(|error| format!("venus_controller_span_56000: {error}"))?),
        ("venus_controller_span_56800", venus_controller_span_56800::START, venus_controller_span_56800::fragment().map_err(|error| format!("venus_controller_span_56800: {error}"))?),
        ("venus_controller_span_57000", venus_controller_span_57000::START, venus_controller_span_57000::fragment().map_err(|error| format!("venus_controller_span_57000: {error}"))?),
        ("venus_controller_span_58000", venus_controller_span_58000::START, venus_controller_span_58000::fragment().map_err(|error| format!("venus_controller_span_58000: {error}"))?),
        ("venus_controller_span_59000", venus_controller_span_59000::START, venus_controller_span_59000::fragment().map_err(|error| format!("venus_controller_span_59000: {error}"))?),
        ("venus_controller_span_5a000", venus_controller_span_5a000::START, venus_controller_span_5a000::fragment().map_err(|error| format!("venus_controller_span_5a000: {error}"))?),
        ("venus_controller_span_5b000", venus_controller_span_5b000::START, venus_controller_span_5b000::fragment().map_err(|error| format!("venus_controller_span_5b000: {error}"))?),
        ("venus_controller_span_5c000", venus_controller_span_5c000::START, venus_controller_span_5c000::fragment().map_err(|error| format!("venus_controller_span_5c000: {error}"))?),
        ("venus_controller_span_5d000", venus_controller_span_5d000::START, venus_controller_span_5d000::fragment().map_err(|error| format!("venus_controller_span_5d000: {error}"))?),
        ("venus_controller_span_5e000", venus_controller_span_5e000::START, venus_controller_span_5e000::fragment().map_err(|error| format!("venus_controller_span_5e000: {error}"))?),
        ("venus_controller_span_5f000", venus_controller_span_5f000::START, venus_controller_span_5f000::fragment().map_err(|error| format!("venus_controller_span_5f000: {error}"))?),
        ("venus_controller_span_60000", venus_controller_span_60000::START, venus_controller_span_60000::fragment().map_err(|error| format!("venus_controller_span_60000: {error}"))?),
        ("venus_controller_span_61000", venus_controller_span_61000::START, venus_controller_span_61000::fragment().map_err(|error| format!("venus_controller_span_61000: {error}"))?),
        ("venus_controller_span_62000", venus_controller_span_62000::START, venus_controller_span_62000::fragment().map_err(|error| format!("venus_controller_span_62000: {error}"))?),
        ("venus_controller_span_63000", venus_controller_span_63000::START, venus_controller_span_63000::fragment().map_err(|error| format!("venus_controller_span_63000: {error}"))?),
        ("venus_controller_span_64000", venus_controller_span_64000::START, venus_controller_span_64000::fragment().map_err(|error| format!("venus_controller_span_64000: {error}"))?),
        ("venus_controller_span_65000", venus_controller_span_65000::START, venus_controller_span_65000::fragment().map_err(|error| format!("venus_controller_span_65000: {error}"))?),
        ("venus_controller_span_66000", venus_controller_span_66000::START, venus_controller_span_66000::fragment().map_err(|error| format!("venus_controller_span_66000: {error}"))?),
        ("venus_controller_span_67000", venus_controller_span_67000::START, venus_controller_span_67000::fragment().map_err(|error| format!("venus_controller_span_67000: {error}"))?),
    ];
    let mut next = 0x40000;
    for (name, start, bytes) in &fragments {
        if *start != next || bytes.is_empty() {
            return Err(format!("Venus controller {name}: expected 0x{next:x}, got 0x{start:x}"));
        }
        next += bytes.len();
    }
    if next != 0x68000 {
        return Err(format!("Venus controller ends at 0x{next:x}, expected 0x68000"));
    }
    Ok(fragments)
}
