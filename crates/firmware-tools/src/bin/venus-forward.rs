// SPDX-License-Identifier: GPL-3.0-only
//! Standalone source-built Venus forward-fragment verifier. Shared image
//! assembly and signing are deliberately out of scope of this agent.
use firmware_tools::devices::xiaomi_nabu::venus_runtime_forward as forward;
use std::{env, fs, error::Error};

fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2 || !["emit","verify"].contains(&args[0].as_str()){
        return Err("usage: venus-forward <emit output.bin|verify reference/venus.mbn>".into());
    }
    let bytes=forward::fragment()?;
    match args[0].as_str(){
        "emit"=>fs::write(&args[1],&bytes)?,
        "verify"=>{
            let orig=fs::read(&args[1])?;
            let start=forward::FILE_OFFSET;
            let slice=orig.get(start..start+bytes.len()).ok_or("reference image too short")?;
            if slice!=bytes {return Err("Venus partial forward bytes differ from reference".into());}
        }
        _=>unreachable!(),
    }
    println!("agent01 {}: {} exact typed A32 bytes, runtime 0x{:x}..0x{:x}, MBN file 0x{:x}..0x{:x}; partial only",
        args[0],bytes.len(),forward::START,forward::END,forward::FILE_OFFSET,forward::FILE_OFFSET+bytes.len());
    Ok(())
}
