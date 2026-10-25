// SPDX-License-Identifier: GPL-3.0-only
//! CDSP RWE runtime callback initializer byte emitter, independent of OEM code.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_runtime as runtime;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=2 {return Err("usage: cdsp-late-runtime <emit OUTPUT|verify PRIVATE_CDSP_MBN>".into());}
    match args[0].as_str(){
        "emit"=>{
            let bytes=runtime::build_runtime_callback_initializer()?;
            fs::write(&args[1],&bytes)?;
            println!("Emitted {} typed Hexagon RWE startup instruction bytes from file offset {:#x}",bytes.len(),runtime::FILE_OFFSET);
        }
        "verify"=>{
            runtime::verify_runtime_callback_initializer(&fs::read(&args[1])?)?;
            println!("Verified {} exact CDSP callback entry executable bytes",runtime::FIRST_FUNCTION_SIZE);
        }
        _=>return Err("usage: cdsp-late-runtime <emit|verify> <output|CDSP_MBN>".into())
    }
    Ok(())
}
