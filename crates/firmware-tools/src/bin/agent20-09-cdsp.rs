// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_tlb as tlb,cdsp_early_copier as copier};
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: agent20-09-cdsp emit FILE | verify REFERENCE".into());}
    match args[0].as_str(){
        "emit"=>fs::write(&args[1],copier::build_joined()?)?,
        "verify"=>copier::verify_joined(&fs::read(&args[1])?)?,
        _=>return Err("unsupported action: use emit or verify".into()),
    }
    println!("CDSP typed controller fragment: {:#x}..{:#x}, physical {:#x}, {} verified/source-emitted bytes",tlb::FILE_OFFSET,copier::REGION_END,tlb::PHYSICAL_ADDRESS,copier::REGION_END-tlb::FILE_OFFSET);
    Ok(())
}
