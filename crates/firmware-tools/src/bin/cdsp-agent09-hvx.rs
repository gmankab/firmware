// SPDX-License-Identifier: GPL-3.0-only
//! Standalone partial CDSP HVX startup emitter/verifier, not signed firmware.
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_hvx_init as hvx, cdsp_early_tlb_state as tlb,cdsp_early_controller_restoration as controller,cdsp_early_supervisor_dispatch as supervisor,cdsp_early_dispatch_helpers as helpers,cdsp_early_tlb_utilities as utilities,cdsp_early_tlb_encode as tlbencode};
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
        [action,path] if action=="emit-supervisor"=>fs::write(path,supervisor::build_supervisor_dispatch()?)?,
        [action,path] if action=="verify-supervisor"=>supervisor::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-helpers"=>fs::write(path,helpers::build_dispatch_helpers()?)?,
        [action,path] if action=="verify-helpers"=>helpers::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-utilities"=>fs::write(path,utilities::build_tlb_utilities()?)?,
        [action,path] if action=="verify-utilities"=>utilities::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-tlbencode"=>fs::write(path,tlbencode::build_tlb_entry()?)?,
        [action,path] if action=="verify-tlbencode"=>tlbencode::verify_reference(&fs::read(path)?)?,
        _=>return Err("usage: cdsp-agent09-hvx [emit|verify|emit-tlb|verify-tlb|emit-controller|verify-controller|emit-supervisor|verify-supervisor|emit-helpers|verify-helpers|emit-utilities|verify-utilities|emit-tlbencode|verify-tlbencode PATH]".into()),
    }
    let (start,end)=if args[0].ends_with("tlb") {(tlb::FILE_OFFSET,tlb::REGION_END)} else if args[0].ends_with("controller") {(controller::FILE_OFFSET,controller::REGION_END)} else if args[0].ends_with("supervisor") {(supervisor::FILE_OFFSET,supervisor::REGION_END)} else if args[0].ends_with("helpers") {(helpers::FILE_OFFSET,helpers::REGION_END)} else if args[0].ends_with("utilities") {(utilities::FILE_OFFSET,utilities::REGION_END)} else if args[0].ends_with("tlbencode") {(tlbencode::FILE_OFFSET,tlbencode::REGION_END)} else {(hvx::FILE_OFFSET,hvx::REGION_END)};
    println!("CDSP source-built {}: {} bytes file {:#x}..{:#x}; not whole firmware",args[0],end-start,start,end);
    Ok(())
}
