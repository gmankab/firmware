// SPDX-License-Identifier: GPL-3.0-only
//! Isolated CDSP Agent 09 forward reconstruction CLI; original image stays private.
use firmware_tools::devices::xiaomi_nabu::cdsp_early_worker_decision as decision;
use std::{env,fs};
fn main()->Result<(),Box<dyn std::error::Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    match args.as_slice(){
        [action,path] if action=="emit-decision" => {
            fs::write(path,decision::build_worker_decision()?)?;
        }
        [action,path] if action=="verify-decision" => {
            decision::verify_reference(&fs::read(path)?)?;
        }
        _=>return Err("usage: cdsp-agent09-forward [emit-decision OUTPUT | verify-decision REFERENCE]".into())
    }
    println!("Agent09 worker decision: {} source-generated executable bytes, file {:#x}..{:#x}, physical {:#x}; incomplete following worker function, not signed/flashable",
        decision::FUNCTION_BYTES,decision::FILE_OFFSET,decision::REGION_END,decision::PHYSICAL_ADDRESS);
    Ok(())
}
