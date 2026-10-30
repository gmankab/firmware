// SPDX-License-Identifier: GPL-3.0-only
//! Standalone partial CDSP HVX startup emitter/verifier, not signed firmware.
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_hvx_init as hvx, cdsp_early_tlb_state as tlb,cdsp_early_controller_restoration as controller};
use std::{env,fs};
fn main()->Result<(),Box<dyn std::error::Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    match args.as_slice(){
        [action,path] if action=="emit"=>fs::write(path,hvx::build_hvx_startup()?)?,
        [action,path] if action=="verify"=>hvx::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-tlb"=>fs::write(path,tlb::build_tlb_state()?)?,
        [action,path] if action=="verify-tlb"=>tlb::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-controller"=>fs::write(path,controller::build_controller_restore()?)?,
        [action,path] if action=="verify-controller"=>controller::verify_reference(&fs::read(path)?)?,
        _=>return Err("usage: cdsp-agent09-hvx [emit|verify|emit-tlb|verify-tlb|emit-controller|verify-controller PATH]".into()),
    }
    let (start,end)=if args[0].ends_with("tlb") {(tlb::FILE_OFFSET,tlb::REGION_END)} else if args[0].ends_with("controller") {(controller::FILE_OFFSET,controller::REGION_END)} else {(hvx::FILE_OFFSET,hvx::REGION_END)};
    println!("CDSP source-built {}: {} bytes file {:#x}..{:#x}; not whole firmware",args[0],end-start,start,end);
    Ok(())
}
