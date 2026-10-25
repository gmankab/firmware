// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_tlb as tlb,cdsp_early_cfgbase as cfgbase,cdsp_early_system_start as system_start,cdsp_early_interrupt_debug as interrupt_debug,cdsp_early_supervisor_vectors as vectors,cdsp_early_exception_entry as exception_entry};
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: agent20-09-cdsp emit FILE | verify REFERENCE".into());}
    match args[0].as_str(){
        "emit"=>fs::write(&args[1],cfgbase::build_joined()?)?,
        "verify"=>exception_entry::verify_reference(&fs::read(&args[1])?)?,
        "emit-exception-entry"=>fs::write(&args[1],exception_entry::build_exception_entry()?)?,
        "emit-supervisor"=>fs::write(&args[1],exception_entry::build_supervisor_with_entry()?)?,
        "emit-vectors or emit-exception-entry"=>fs::write(&args[1],vectors::build_supervisor_vectors()?)?,
        "emit-interrupt-debug"=>fs::write(&args[1],interrupt_debug::build_interrupt_debug()?)?,
        "emit-system-start"=>fs::write(&args[1],system_start::build_system_start()?)?,
        _=>return Err("unsupported action: use emit, verify, emit-interrupt-debug, emit-supervisor or emit-vectors".into()),
    }
    println!("CDSP typed functions: {:#x}..{:#x} and {:#x}..{:#x}, {}+{}={} generated executable bytes (232 separate linker-alignment bytes)",tlb::FILE_OFFSET,cfgbase::REGION_END,system_start::FILE_OFFSET,exception_entry::REGION_END,cfgbase::REGION_END-tlb::FILE_OFFSET,exception_entry::REGION_END-system_start::FILE_OFFSET,(cfgbase::REGION_END-tlb::FILE_OFFSET)+(exception_entry::REGION_END-system_start::FILE_OFFSET));
    Ok(())
}
