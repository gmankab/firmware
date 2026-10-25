// SPDX-License-Identifier: GPL-3.0-only
//! Reproducible standalone emitter/comparator for Agent 20-04 fragments.
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_middle as v;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=2 || !["emit","verify"].contains(&args[0].as_str()){
        return Err("usage: cargo run --example agent20_04_venus_middle -- <emit output|verify private/venus.mbn>".into());
    }
    let bytes=v::first_fragment()?;
    if args[0]=="emit" {fs::write(&args[1],&bytes)?;}
    else {
        let original=fs::read(&args[1])?;
        let offset=v::FIRST_START+v::FILE_BASE;
        let reference=original.get(offset..offset+bytes.len()).ok_or("reference too short")?;
        if let Some(idx)=bytes.iter().zip(reference).position(|(a,b)|a!=b){
            return Err(format!("mismatch at Venus VMA 0x{:x}, MBN file 0x{:x}",v::FIRST_START+idx,offset+idx).into());
        }
    }
    println!("Agent20-04 {}: {} source-derived A32 bytes, runtime [0x{:x},0x{:x}), MBN [0x{:x},0x{:x}), no mismatch",args[0],bytes.len(),v::FIRST_START,v::FIRST_END,v::FIRST_START+v::FILE_BASE,v::FIRST_END+v::FILE_BASE);
    Ok(())
}
