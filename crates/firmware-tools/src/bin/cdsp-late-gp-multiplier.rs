// SPDX-License-Identifier: GPL-3.0-only
//! Source-emitted CDSP GP multiplier and its contiguous RWE prefix.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_gp_state_multiplier as s;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: cdsp-late-gp-multiplier <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match args[0].as_str(){
        "emit"=>{fs::write(&args[1],s::build_state_multiplier()?)?;println!("Emitted {} CDSP GP multiplier bytes",s::EXECUTABLE_LEN);},
        "verify"=>{s::verify_multiplier(&fs::read(&args[1])?)?;println!("Verified {} original byte-exact CDSP bytes",s::EXECUTABLE_LEN);},
        "emit-prefix"=>{fs::write(&args[1],s::build_rwe_prefix_through_multiplier()?)?;println!("Emitted RWE prefix {} bytes, {} executable and {} layout",s::FULL_RWE_LAYOUT_LEN,s::FULL_RWE_EXECUTABLE_LEN,s::RWE_LAYOUT_NOP_LEN);},
        "verify-prefix"=>{s::verify_prefix(&fs::read(&args[1])?)?;println!("Verified {} byte RWE prefix",s::FULL_RWE_LAYOUT_LEN);},
        _=>return Err("unknown action".into()),
    }
    Ok(())
}
