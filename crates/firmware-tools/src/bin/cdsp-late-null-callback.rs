// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_null_callback as c;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let a:Vec<_>=env::args().skip(1).collect();
    if a.len()!=2{return Err("usage: cdsp-late-null-callback <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match a[0].as_str(){
        "emit"=>{fs::write(&a[1],c::build_null_callback()?)?;println!("Generated {} source-derived null callback bytes",c::EXECUTABLE_LEN);},
        "verify"=>{c::verify_null_callback(&fs::read(&a[1])?)?;println!("Verified {} byte-identical callback bytes",c::EXECUTABLE_LEN);},
        "emit-prefix"=>{fs::write(&a[1],c::build_rwe_prefix_through_null_callback()?)?;println!("Generated {} RWE file bytes: {} executable + {} fill",c::FULL_RWE_LAYOUT_LEN,c::FULL_RWE_EXECUTABLE_LEN,c::RWE_LAYOUT_NOP_LEN);},
        "verify-prefix"=>{c::verify_prefix(&fs::read(&a[1])?)?;println!("Verified {} original RWE prefix bytes",c::FULL_RWE_LAYOUT_LEN);},
        _=>return Err("Unknown action".into()),
    }
    Ok(())
}
