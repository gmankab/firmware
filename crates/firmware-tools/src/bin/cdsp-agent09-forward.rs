// SPDX-License-Identifier: GPL-3.0-only
//! Isolated CDSP Agent 09 forward reconstruction CLI; original image stays private.
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_worker_decision as decision,cdsp_early_worker_command as command};
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
        [action,path] if action=="emit-command" => {
            fs::write(path,command::build_worker_command()?)?;
        }
        [action,path] if action=="verify-command" => {
            command::verify_reference(&fs::read(path)?)?;
        }
        _=>return Err("usage: cdsp-agent09-forward [emit-decision OUTPUT | verify-decision REFERENCE | emit-command OUTPUT | verify-command REFERENCE]".into())
    }
    println!("Agent09 forward worker {}: latest range file {:#x}..{:#x}, {} independently source-encoded bytes; partial firmware only",
        args[0],command::FILE_OFFSET,command::REGION_END,command::FUNCTION_BYTES);
    Ok(())
}
