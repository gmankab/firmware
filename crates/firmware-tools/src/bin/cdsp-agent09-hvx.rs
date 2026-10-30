// SPDX-License-Identifier: GPL-3.0-only
//! Standalone partial CDSP HVX startup emitter/verifier, not signed firmware.
use firmware_tools::devices::xiaomi_nabu::cdsp_early_hvx_init as hvx;
use std::{env,fs};
fn main()->Result<(),Box<dyn std::error::Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    match args.as_slice(){
        [action,path] if action=="emit"=>fs::write(path,hvx::build_hvx_startup()?)?,
        [action,path] if action=="verify"=>hvx::verify_reference(&fs::read(path)?)?,
        _=>return Err("usage: cdsp-agent09-hvx [emit OUTPUT | verify PRIVATE-CDSP]".into()),
    }
    println!("CDSP HVX startup {}: {} independently encoded bytes file {:#x}..{:#x}; not whole firmware",args[0],hvx::FUNCTION_BYTES,hvx::FILE_OFFSET,hvx::REGION_END);
    Ok(())
}
