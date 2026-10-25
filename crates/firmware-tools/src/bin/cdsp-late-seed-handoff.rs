// SPDX-License-Identifier: GPL-3.0-only
//! Command line for independently generated CDSP V66 seed controller handoff.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_seed_handoff as handoff;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: cdsp-late-seed-handoff <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match args[0].as_str(){
        "emit"=>{fs::write(&args[1],handoff::build_seed_handoff()?)?;println!("Emitted {} source-built CDSP handoff executable bytes",handoff::EXECUTABLE_LEN);},
        "verify"=>{handoff::verify_handoff(&fs::read(&args[1])?)?;println!("Verified {} exact source CDSP handoff bytes",handoff::EXECUTABLE_LEN);},
        "emit-prefix"=>{fs::write(&args[1],handoff::build_rwe_prefix_through_handoff()?)?;println!("Emitted {} contiguous RWE file bytes, {} executable + {} layout fill",handoff::FULL_RWE_LAYOUT_LEN,handoff::FULL_RWE_EXECUTABLE_LEN,handoff::RWE_LAYOUT_NOP_LEN);},
        "verify-prefix"=>{handoff::verify_prefix(&fs::read(&args[1])?)?;println!("Verified contiguous {} CDSP RWE bytes",handoff::FULL_RWE_LAYOUT_LEN);},
        _=>return Err("unknown handoff subcommand".into()),
    }
    Ok(())
}
