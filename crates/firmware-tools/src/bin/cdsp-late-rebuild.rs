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
        _=>return Err("usage: cdsp-late-rebuild <emit|verify|emit-joined|verify-joined|emit-mode-prefix|verify-mode-prefix> <OUTPUT|CDSP_MBN>".into()),
    }
    Ok(())
}
