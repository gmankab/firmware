// SPDX-License-Identifier: GPL-3.0-only
//! Emits and checks 64 source-built Hexagon V66 bytes, outside other CDSP
//! agents' code/file ranges. Does not build or sign a complete MBN.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_callback_registry as r;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: cdsp-late-callback-registry <emit OUT|verify PRIVATE_CDSP_MBN>".into());}
    match args[0].as_str(){
        "emit"=>{
            let bytes=r::build_runtime_registry_cleanup()?;
            fs::write(&args[1],&bytes)?;
            println!("Generated {} independent Hexagon callback-registry bytes",bytes.len());
        },
        "verify"=>{
            r::verify_runtime_registry_cleanup(&fs::read(&args[1])?)?;
            println!("Verified {} source-authored bytes at CDSP file 0x{:x}..0x{:x}",
                r::EXECUTABLE_LEN,r::FILE_OFFSET,r::END_FILE_OFFSET);
        },
        _=>return Err("expected emit or verify".into()),
    }
    Ok(())
}
