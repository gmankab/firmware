// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_mask_setup as c;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
    let a:Vec<_>=env::args().skip(1).collect();
    if a.len()!=2{return Err("usage: cdsp-late-controller-mask-setup <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match a[0].as_str(){
        "emit"=>fs::write(&a[1],c::build_bank_setup()?)?,
        "verify"=>c::verify(&fs::read(&a[1])?)?,
        "emit-prefix"=>fs::write(&a[1],c::build_prefix()?)?,
        "verify-prefix"=>c::verify_prefix(&fs::read(&a[1])?)?,
        _=>return Err("invalid command".into()),
    }
    println!("CDSP controller-bank setup: {} typed executable bytes; prefix {} file bytes = {} code + {} layout",
             c::EXECUTABLE_LEN,c::FULL_RWE_LAYOUT_LEN,c::FULL_RWE_EXECUTABLE_LEN,c::RWE_PREFIX_LAYOUT_LEN);
    Ok(())
}
