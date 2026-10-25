// SPDX-License-Identifier: GPL-3.0-only
//! Agent 20-10 Hexagon callback wrapper, independent source-built verification.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_callback_completion as done;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2 {return Err("usage: cdsp-late-callback-completion <emit|verify|emit-prefix|verify-prefix> OUTPUT_OR_REFERENCE".into());}
    match args[0].as_str(){
        "emit"=>{let out=done::build_completion_wrapper()?; fs::write(&args[1],&out)?;println!("Emitted {} source-derived V66 executable bytes",out.len());},
        "verify"=>{done::verify_completion_wrapper(&fs::read(&args[1])?)?;println!("Verified {} exact CDSP callback-wrapper bytes",done::EXECUTABLE_LEN);},
        "emit-prefix"=>{let out=done::build_prefix_with_completion()?;fs::write(&args[1],&out)?;println!("Emitted {} RWE bytes: {} typed instructions plus {} NOP/fill bytes",out.len(),done::RWE_EXECUTABLE_LEN,done::RWE_FILL_BYTES);},
        "verify-prefix"=>{done::verify_prefix_with_completion(&fs::read(&args[1])?)?;println!("Verified {} CDSP RWE prefix bytes",done::RWE_LAYOUT_LEN);},
        _=>return Err("unknown CDSP completion operation".into()),
    }
    Ok(())
}
