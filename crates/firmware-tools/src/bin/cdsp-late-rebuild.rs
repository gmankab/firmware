// SPDX-License-Identifier: GPL-3.0-only
//! Agent20-10 independently generated CDSP late Hexagon MMIO setup.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_mmio as late;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2 {return Err("usage: cdsp-late-rebuild <emit FILE|verify REFERENCE_CDSP_MBN>".into());}
    match args[0].as_str(){
        "emit"=>{let bytes=late::build_mmio_setup()?;fs::write(&args[1],&bytes)?;println!("Emitted {} source-generated CDSP Hexagon bytes file {:#x}–{:#x}",bytes.len(),late::FILE_OFFSET,late::FILE_OFFSET+bytes.len());},
        "verify"=>{let orig=fs::read(&args[1])?;late::verify_mmio_setup(&orig)?;println!("Verified {} CDSP Hexagon bytes against original; no opcode copying",late::FIRST_ENTRY_LEN);},
        "emit-joined"=>{let bytes=late::build_joined_initial_controller()?;fs::write(&args[1],&bytes)?;println!("Emitted {} independently generated CDSP late controller bytes file {:#x}–{:#x}",bytes.len(),late::FILE_OFFSET,late::FILE_OFFSET+bytes.len());},
        "verify-joined"=>{let orig=fs::read(&args[1])?;late::verify_joined_controller(&orig)?;println!("Verified {} typed Hexagon bytes against original, no copied opcodes",late::JOINED_LEN);},
        "emit-mode-prefix"=>{let bytes=late::build_joined_with_mode_dispatch()?;fs::write(&args[1],&bytes)?;println!("Emitted {} Hexagon controller bytes file {:#x}–{:#x}",bytes.len(),late::FILE_OFFSET,late::FILE_OFFSET+bytes.len());},
        "verify-mode-prefix"=>{let orig=fs::read(&args[1])?;late::verify_joined_with_mode_dispatch(&orig)?;println!("Verified {} generated CDSP controller bytes against pinned OEM reference",late::THREE_STAGE_LEN);},
        "emit-configurations"=>{let bytes=late::build_all_controller_mode_programs()?;fs::write(&args[1],&bytes)?;println!("Emitted {} source-built Hexagon config bytes file {:#x}–{:#x}",bytes.len(),late::FILE_OFFSET,late::FILE_OFFSET+bytes.len());},
        "verify-configurations"=>{late::verify_all_controller_mode_programs(&fs::read(&args[1])?)?;println!("Verified {} typed CDSP Hexagon bytes through hardware mode configuration",late::COMPLETE_CONFIG_PREFIX_LEN);},
        "emit-flags"=>{let bytes=late::build_controller_through_flag_clear()?;fs::write(&args[1],&bytes)?;println!("Emitted {} Hexagon bytes including register flag commits",bytes.len());},
        "verify-flags"=>{late::verify_controller_through_flag_clear(&fs::read(&args[1])?)?;println!("Verified {} exact CDSP MMIO controller bytes",late::FLAG_CLEAR_PREFIX_LEN);},
        "emit-wait-handoff"=>{let bytes=late::build_controller_through_wait_handoff()?;fs::write(&args[1],&bytes)?;println!("Emitted {} Hexagon controller bytes through counter wait handoff",bytes.len());},
        "verify-wait-handoff"=>{late::verify_controller_through_wait_handoff(&fs::read(&args[1])?)?;println!("Verified {} exact CDSP Hexagon bytes through wait handoff",late::ENABLE_AND_WAIT_LEN);},
        "emit-cache-spin"=>{let bytes=late::build_feature_spin_and_cache_updates()?;fs::write(&args[1],&bytes)?;println!("Emitted {} Hexagon controller bytes through cache feature MMIO updates",bytes.len());},
        "verify-cache-spin"=>{late::verify_feature_spin_and_cache_updates(&fs::read(&args[1])?)?;println!("Verified {} exact CDSP Hexagon bytes through cache MMIO updates",late::FEATURE_SPIN_LEN);},
        _=>return Err("usage: cdsp-late-rebuild <emit|verify|emit-joined|verify-joined|emit-mode-prefix|verify-mode-prefix|emit-configurations|verify-configurations|emit-flags|verify-flags|emit-wait-handoff|verify-wait-handoff> <OUTPUT|CDSP_MBN>".into()),
    }
    Ok(())
}
