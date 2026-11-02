// SPDX-License-Identifier: GPL-3.0-only
//! Reproducible standalone emitter/comparator for Agent 20-04 fragments.
use firmware_tools::devices::xiaomi_nabu::venus_controller_middle as v;
use firmware_tools::devices::xiaomi_nabu::venus_controller_flags as flags;
use firmware_tools::devices::xiaomi_nabu::venus_controller_format_registers as format;
use firmware_tools::devices::xiaomi_nabu::venus_controller_codec_controller as codec;
use firmware_tools::devices::xiaomi_nabu::venus_controller_packets as packets;
use firmware_tools::devices::xiaomi_nabu::venus_controller_codec_mode_packets as modepackets;
use firmware_tools::devices::xiaomi_nabu::venus_controller_command_serializer as serializer;
use firmware_tools::devices::xiaomi_nabu::venus_controller_packet_sequence as seq;
use firmware_tools::devices::xiaomi_nabu::venus_controller_extended_controls as ctrls;
use firmware_tools::devices::xiaomi_nabu::venus_controller_control_fields as fields;
use firmware_tools::devices::xiaomi_nabu::venus_controller_codec_table_bundles as caps;
use firmware_tools::devices::xiaomi_nabu::venus_controller_feature_extension as extended;
use firmware_tools::devices::xiaomi_nabu::venus_controller_packet_features as masks;
use firmware_tools::devices::xiaomi_nabu::venus_controller_component_geometry as geometry;
use firmware_tools::devices::xiaomi_nabu::venus_controller_plane_strides as stride;
use firmware_tools::devices::xiaomi_nabu::venus_controller_late_controller_masks as later;
use firmware_tools::devices::xiaomi_nabu::venus_controller_codec_class_values as classes;
use firmware_tools::devices::xiaomi_nabu::venus_controller_stream_class_commands as class_commands;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_4c000 as controller_span_4c000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_signed_frame_records as signed_frame_records;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_4e000 as controller_span_4e000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_4f000 as controller_span_4f000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_50000 as controller_span_50000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_51000 as controller_span_51000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_52000 as controller_span_52000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_53000 as controller_span_53000;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_53400 as controller_span_53400;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_53944 as controller_span_53944;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_53b24 as controller_span_53b24;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_54028 as controller_span_54028;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_54854 as controller_span_54854;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_54c1c as controller_span_54c1c;
use firmware_tools::devices::xiaomi_nabu::venus_controller_span_54cfc as controller_span_54cfc;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=2 || !["emit","verify"].contains(&args[0].as_str()){
        return Err("usage: cargo run --example venus_controller_middle -- <emit output|verify private/venus.mbn>".into());
    }
    let fragments=[(v::FIRST_START,v::first_fragment()?), (codec::START,codec::fragment()?), (format::START,format::fragment()?), (packets::START,packets::fragment()?), (flags::START,flags::fragment()?), (modepackets::START,modepackets::fragment()?), (serializer::START,serializer::fragment()?), (seq::START,seq::fragment()?), (ctrls::START,ctrls::fragment()?), (fields::START,fields::fragment()?), (caps::START,caps::fragment()?), (extended::START,extended::fragment()?), (masks::START,masks::fragment()?), (geometry::START,geometry::fragment()?), (stride::START,stride::fragment()?), (later::START,later::fragment()?), (classes::START,classes::fragment()?), (class_commands::START,class_commands::fragment()?), (controller_span_4c000::START,controller_span_4c000::fragment()?), (signed_frame_records::START,signed_frame_records::fragment()?), (controller_span_4e000::START,controller_span_4e000::fragment()?), (controller_span_4f000::START,controller_span_4f000::fragment()?), (controller_span_50000::START,controller_span_50000::fragment()?), (controller_span_51000::START,controller_span_51000::fragment()?), (controller_span_52000::START,controller_span_52000::fragment()?), (controller_span_53000::START,controller_span_53000::fragment()?), (controller_span_53400::START,controller_span_53400::fragment()?), (controller_span_53944::START,controller_span_53944::fragment()?), (controller_span_53b24::START,controller_span_53b24::fragment()?), (controller_span_54028::START,controller_span_54028::fragment()?), (controller_span_54854::START,controller_span_54854::fragment()?), (controller_span_54c1c::START,controller_span_54c1c::fragment()?), (controller_span_54cfc::START,controller_span_54cfc::fragment()?)];
    if args[0]=="emit" {
        for (start,bytes) in &fragments {fs::write(format!("{}-{start:x}.bin",args[1]),bytes)?;}
    } else {
        let original=fs::read(&args[1])?;
        for (start,bytes) in &fragments {
            let offset=start+v::FILE_BASE;
            let reference=original.get(offset..offset+bytes.len()).ok_or("reference too short")?;
            if let Some(idx)=bytes.iter().zip(reference).position(|(a,b)|a!=b){
                return Err(format!("mismatch at Venus VMA 0x{:x}, MBN file 0x{:x}",start+idx,offset+idx).into());
            }
        }
    }
    for (start,bytes) in &fragments {
        println!("Agent20-04 {}: {} source-derived A32 bytes, runtime [0x{start:x},0x{:x}), MBN [0x{:x},0x{:x}), no mismatch",args[0],bytes.len(),start+bytes.len(),start+v::FILE_BASE,start+v::FILE_BASE+bytes.len());
    }
    Ok(())
}
