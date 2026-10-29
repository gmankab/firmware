// SPDX-License-Identifier: GPL-3.0-only
//! Isolated CDSP Agent 09 forward reconstruction CLI; original image stays private.
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_worker_decision as decision,cdsp_early_worker_command as command,cdsp_early_worker_callbacks as callbacks,cdsp_early_worker_privileged_state as privstate,cdsp_early_worker_snapshot as snapshot,cdsp_early_worker_final_calls as finalcalls,cdsp_early_worker_guard_exit as guardexit,cdsp_early_worker_sync as sync,cdsp_early_worker_handoff as handoff,cdsp_early_worker_recovery as recovery,cdsp_early_revision_guard as revguard,cdsp_early_dcache_tags as tags,cdsp_early_icache_guard as icache};
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
        [action,path] if action=="emit-sync" => fs::write(path,sync::build_worker_sync()?)?,
        [action,path] if action=="verify-sync" => sync::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-handoff" => fs::write(path,handoff::build_worker_handoff()?)?,
        [action,path] if action=="verify-handoff" => handoff::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-recovery" => fs::write(path,recovery::build_worker_recovery()?)?,
        [action,path] if action=="verify-recovery" => recovery::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-revguard" => fs::write(path,revguard::build_revision_guard()?)?,
        [action,path] if action=="verify-revguard" => revguard::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-dcache-tags" => fs::write(path,tags::build_dcache_tag_recovery()?)?,
        [action,path] if action=="verify-dcache-tags" => tags::verify_reference(&fs::read(path)?)?,
        [action,path] if action=="emit-icache-guard" => fs::write(path,icache::build_icache_guard()?)?,
        [action,path] if action=="verify-icache-guard" => icache::verify_reference(&fs::read(path)?)?,
        _=>return Err("usage: cdsp-agent09-forward [emit-decision OUTPUT | verify-decision REFERENCE | emit-command OUTPUT | verify-command REFERENCE | emit-callbacks OUTPUT | verify-callbacks REFERENCE | emit-privstate OUTPUT | verify-privstate REFERENCE | emit-snapshot OUTPUT | verify-snapshot REFERENCE | emit-finalcalls OUTPUT | verify-finalcalls REFERENCE | emit-guard-exit OUTPUT | verify-guard-exit REFERENCE | emit-sync OUTPUT | verify-sync REFERENCE | emit-handoff OUTPUT | verify-handoff REFERENCE | emit-recovery OUTPUT | verify-recovery REFERENCE | emit-revguard OUTPUT | verify-revguard REFERENCE | emit-dcache-tags OUTPUT | verify-dcache-tags REFERENCE]".into())
    }
    let (start,end)=match args[0].as_str(){
        "emit-decision"|"verify-decision"=>(decision::FILE_OFFSET,decision::REGION_END),
        "emit-command"|"verify-command"=>(command::FILE_OFFSET,command::REGION_END),
        "emit-callbacks"|"verify-callbacks"=>(callbacks::FILE_OFFSET,callbacks::REGION_END),
        "emit-privstate"|"verify-privstate"=>(privstate::FILE_OFFSET,privstate::REGION_END),
        "emit-snapshot"|"verify-snapshot"=>(snapshot::FILE_OFFSET,snapshot::REGION_END),
        "emit-finalcalls"|"verify-finalcalls"=>(finalcalls::FILE_OFFSET,finalcalls::REGION_END),
        "emit-guard-exit"|"verify-guard-exit"=>(guardexit::FILE_OFFSET,guardexit::REGION_END),
        "emit-sync"|"verify-sync"=>(sync::FILE_OFFSET,sync::REGION_END),
        "emit-handoff"|"verify-handoff"=>(handoff::FILE_OFFSET,handoff::REGION_END),
        "emit-recovery"|"verify-recovery"=>(recovery::FILE_OFFSET,recovery::REGION_END),
        "emit-revguard"|"verify-revguard"=>(revguard::FILE_OFFSET,revguard::REGION_END),
        "emit-dcache-tags"|"verify-dcache-tags"=>(tags::FILE_OFFSET,tags::REGION_END),
        "emit-icache-guard"|"verify-icache-guard"=>(icache::FILE_OFFSET,icache::REGION_END),
        _=>unreachable!(),
    };
    println!("Agent09 forward worker {}: CDSP file {:#x}..{:#x}, {} typed V66 bytes; partial, not signed/flashable",
        args[0],start,end,end-start);
    Ok(())
}
