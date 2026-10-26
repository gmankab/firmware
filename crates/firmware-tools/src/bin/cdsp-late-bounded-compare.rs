// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_bounded_compare as c;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let a:Vec<String>=env::args().skip(1).collect();
    if a.len()!=2 {return Err("usage: cdsp-late-bounded-compare <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match a[0].as_str(){
        "emit"=>fs::write(&a[1],c::build_bounded_compare()?)?,
        "verify"=>c::verify(&fs::read(&a[1])?)?,
        "emit-prefix"=>fs::write(&a[1],c::build_prefix()?)?,
        "verify-prefix"=>c::verify_prefix(&fs::read(&a[1])?)?,
        _=>return Err("Unknown action".into()),
    }
    println!("Agent20-10 bounded comparison: {} executable CDSP bytes; source-built RWE prefix {} = {} executable + {} linker fill",c::EXECUTABLE_LEN,c::FULL_RWE_LAYOUT_LEN,c::FULL_RWE_EXECUTABLE_LEN,c::RWE_LAYOUT_NOP_LEN);
    Ok(())
}
