// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_transfer as code;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let a:Vec<String>=env::args().skip(1).collect();
    if a.len()!=2 {return Err("usage: cdsp-late-controller-transfer <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match a[0].as_str(){
        "emit"=>fs::write(&a[1],code::build_transfer_prefix()?)?,
        "verify"=>code::verify(&fs::read(&a[1])?)?,
        "emit-prefix"=>fs::write(&a[1],code::build_prefix()?)?,
        "verify-prefix"=>code::verify_prefix(&fs::read(&a[1])?)?,
        _=>return Err("unrecognized CDSP controller action".into()),
    }
    println!("CDSP controller transfer: {} exact executable bytes; RWE {} = {} code + {} layout; unfinished function",
             code::EXECUTABLE_LEN,code::RWE_PREFIX_LEN,code::RWE_PREFIX_CODE_LEN,code::RWE_PREFIX_LAYOUT_LEN);
    Ok(())
}
