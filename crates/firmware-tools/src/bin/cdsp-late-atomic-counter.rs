// SPDX-License-Identifier: GPL-3.0-only
//! Emit/compare typed lock-free CDSP counter and contiguous RWE layout.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_atomic_counter as c;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: cdsp-late-atomic-counter <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match args[0].as_str(){
        "emit"=>{fs::write(&args[1],c::build_atomic_counter()?)?;println!("CDSP source counter {} bytes",c::EXECUTABLE_LEN);},
        "verify"=>{c::verify_atomic(&fs::read(&args[1])?)?;println!("Verified {} exact CDSP counter bytes",c::EXECUTABLE_LEN);},
        "emit-prefix"=>{fs::write(&args[1],c::build_rwe_prefix_through_atomic()?)?;println!("CDSP prefix {} file bytes: {} executable, {} layout",c::FULL_RWE_LAYOUT_LEN,c::FULL_RWE_EXECUTABLE_LEN,c::RWE_LAYOUT_NOP_LEN);},
        "verify-prefix"=>{c::verify_prefix(&fs::read(&args[1])?)?;println!("Verified {} exact CDSP RWE prefix bytes",c::FULL_RWE_LAYOUT_LEN);},
        _=>return Err("unknown operation".into()),
    }
    Ok(())
}
