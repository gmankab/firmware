// SPDX-License-Identifier: GPL-3.0-only
//! CDSP RWE controller PIC and GP-zero packet verifier, without OEM code.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_runtime_more as late;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=2 {return Err("usage: cdsp-late-runtime-more <emit OUTPUT|verify PRIVATE_CDSP_MBN>".into());}
    match args[0].as_str(){
        "emit"=>{
            let data=late::build_link_state_and_gp_clear()?;
            fs::write(&args[1],&data)?;
            println!("Emitted {} typed Hexagon PIC/GP runtime bytes",data.len());
        }
        "verify"=>{
            late::verify_link_state_and_gp_clear(&fs::read(&args[1])?)?;
            println!("Verified {} exact CDSP runtime PIC/GP bytes",late::VERIFIED_SIZE);
        }
        _=>return Err("usage: cdsp-late-runtime-more <emit|verify> <OUTPUT|CDSP_MBN>".into())
    }
    Ok(())
}
