// SPDX-License-Identifier: GPL-3.0-only
//! Isolated CDSP Agent 09 forward reconstruction CLI; original image stays private.
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_worker_decision as decision,cdsp_early_worker_command as command,cdsp_early_worker_callbacks as callbacks,cdsp_early_worker_privileged_state as privstate,cdsp_early_worker_snapshot as snapshot,cdsp_early_worker_final_calls as finalcalls,cdsp_early_worker_guard_exit as guardexit};
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
        [action,path] if action=="emit-callbacks" => fs::write(path,callbacks::build_worker_callbacks()?)?,
        [action,path] if action=="verify-callbacks" => callbacks::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-privstate" => fs::write(path,privstate::build_privileged_worker_state()?)?,
        [action,path] if action=="verify-privstate" => privstate::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-snapshot" => fs::write(path,snapshot::build_worker_snapshot()?)?,
        [action,path] if action=="verify-snapshot" => snapshot::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-finalcalls" => fs::write(path,finalcalls::build_worker_final_calls()?)?,
        [action,path] if action=="verify-finalcalls" => finalcalls::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-guard-exit" => fs::write(path,guardexit::build_guard_exit()?)?,
        [action,path] if action=="verify-guard-exit" => guardexit::verify_reference(&fs::read(path)?)?,
        _=>return Err("usage: cdsp-agent09-forward [emit-decision OUTPUT | verify-decision REFERENCE | emit-command OUTPUT | verify-command REFERENCE | emit-callbacks OUTPUT | verify-callbacks REFERENCE | emit-privstate OUTPUT | verify-privstate REFERENCE | emit-snapshot OUTPUT | verify-snapshot REFERENCE | emit-finalcalls OUTPUT | verify-finalcalls REFERENCE | emit-guard-exit OUTPUT | verify-guard-exit REFERENCE]".into())
    }
    let (start,end)=match args[0].as_str(){
        "emit-decision"|"verify-decision"=>(decision::FILE_OFFSET,decision::REGION_END),
        "emit-command"|"verify-command"=>(command::FILE_OFFSET,command::REGION_END),
        "emit-callbacks"|"verify-callbacks"=>(callbacks::FILE_OFFSET,callbacks::REGION_END),
        "emit-privstate"|"verify-privstate"=>(privstate::FILE_OFFSET,privstate::REGION_END),
        "emit-snapshot"|"verify-snapshot"=>(snapshot::FILE_OFFSET,snapshot::REGION_END),
        "emit-finalcalls"|"verify-finalcalls"=>(finalcalls::FILE_OFFSET,finalcalls::REGION_END),
        "emit-guard-exit"|"verify-guard-exit"=>(guardexit::FILE_OFFSET,guardexit::REGION_END),
        _=>unreachable!(),
    };
    println!("Agent09 forward worker {}: CDSP file {:#x}..{:#x}, {} typed V66 bytes; partial, not signed/flashable",
        args[0],start,end,end-start);
    Ok(())
}
