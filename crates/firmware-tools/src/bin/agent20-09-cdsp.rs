// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_tlb as tlb,cdsp_early_cfgbase as cfgbase,cdsp_early_system_start as system_start,cdsp_early_interrupt_debug as interrupt_debug};
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: agent20-09-cdsp emit FILE | verify REFERENCE".into());}
    match args[0].as_str(){
        "emit"=>fs::write(&args[1],cfgbase::build_joined()?)?,
        "verify"=>interrupt_debug::verify_reference(&fs::read(&args[1])?)?,
        "emit-interrupt-debug"=>fs::write(&args[1],interrupt_debug::build_interrupt_debug()?)?,
        "emit-system-start"=>fs::write(&args[1],system_start::build_system_start()?)?,
        _=>return Err("unsupported action: use emit, verify or emit-interrupt-debug".into()),
    }
    println!("CDSP typed controller fragment: {:#x}..{:#x}, physical {:#x}, {} verified/source-emitted bytes",tlb::FILE_OFFSET,cfgbase::REGION_END,tlb::PHYSICAL_ADDRESS,cfgbase::REGION_END-tlb::FILE_OFFSET);
    Ok(())
}
