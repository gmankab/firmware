// SPDX-License-Identifier: GPL-3.0-only
//! Reproducible standalone emitter/comparator for Agent 20-04 fragments.
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_middle as v;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_controller_flags as flags;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_format_registers as format;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_codec_controller as codec;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_controller_packets as packets;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_codec_mode_packets as modepackets;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_command_serializer as serializer;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_packet_sequence as seq;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_extended_controls as ctrls;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_control_fields as fields;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_codec_table_bundles as caps;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_feature_extension as extended;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_packet_features as masks;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_component_geometry as geometry;
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_plane_strides as stride;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=2 || !["emit","verify"].contains(&args[0].as_str()){
        return Err("usage: cargo run --example agent20_04_venus_middle -- <emit output|verify private/venus.mbn>".into());
    }
    let fragments=[(v::FIRST_START,v::first_fragment()?), (codec::START,codec::fragment()?), (format::START,format::fragment()?), (packets::START,packets::fragment()?), (flags::START,flags::fragment()?), (modepackets::START,modepackets::fragment()?), (serializer::START,serializer::fragment()?), (seq::START,seq::fragment()?), (ctrls::START,ctrls::fragment()?), (fields::START,fields::fragment()?), (caps::START,caps::fragment()?), (extended::START,extended::fragment()?), (masks::START,masks::fragment()?), (geometry::START,geometry::fragment()?), (stride::START,stride::fragment()?)];
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
