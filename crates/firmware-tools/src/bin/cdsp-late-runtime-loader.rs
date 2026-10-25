// SPDX-License-Identifier: GPL-3.0-only
//! CDSP RWE constructor callback emitter with strict linker-fill accounting.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_runtime_loader as runtime;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=2 {return Err("usage: cdsp-late-runtime-loader <emit OUTPUT|verify PRIVATE_CDSP_MBN|emit-layout OUTPUT|verify-layout PRIVATE_CDSP_MBN>".into());}
    match args[0].as_str(){
        "emit"=>{let data=runtime::build_runtime_loader()?;fs::write(&args[1],&data)?;println!("Emitted {} Hexagon RWE constructor loader bytes",data.len());}
        "verify"=>{runtime::verify_runtime_loader(&fs::read(&args[1])?)?;println!("Verified {} exact CDSP constructor loader bytes",runtime::FUNCTION_LEN);}
        "emit-layout"=>{let data=runtime::build_rwe_source_and_layout_prefix()?;fs::write(&args[1],&data)?;println!("Emitted {} RWE layout bytes; {} executable source, {} zero linker fill",data.len(),runtime::EXECUTABLE_BYTES_IN_LAYOUT_PREFIX,runtime::LAYOUT_FILL_BYTES_IN_PREFIX);}
        "verify-layout"=>{runtime::verify_rwe_source_and_layout_prefix(&fs::read(&args[1])?)?;println!("Verified {} RWE layout bytes (264 executable + 12 linker fill)",runtime::RWE_LAYOUT_PREFIX_LEN);}
        _=>return Err("unsupported CDSP RWE runtime loader command".into())
    }
    Ok(())
}
