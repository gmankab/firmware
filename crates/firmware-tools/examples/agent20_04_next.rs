// SPDX-License-Identifier: GPL-3.0-only
//! Lightweight standalone exact-byte verifier for newly emitted Agent 04
//! controller routines. Reference firmware is private and not in Git.
use firmware_tools::devices::xiaomi_nabu::venus_agent20_04_controller_span_53944 as span;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>> {
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2 || !["emit","verify"].contains(&args[0].as_str()) {
        return Err("usage: cargo run --example agent20_04_next -- <emit output.bin|verify /private/venus.mbn>".into());
    }
    let generated=span::fragment()?;
    match args[0].as_str(){
        "emit"=>fs::write(&args[1],&generated)?,
        "verify"=>{
            let reference=fs::read(&args[1])?;
            let expected=reference.get(span::FILE_OFFSET..span::FILE_OFFSET+generated.len()).ok_or("reference too short")?;
            if let Some(i)=generated.iter().zip(expected).position(|(a,b)|a!=b) {
                return Err(format!("byte mismatch at runtime 0x{:x} (file 0x{:x})",span::START+i,span::FILE_OFFSET+i).into());
            }
        }
        _=>unreachable!(),
    }
    println!("agent 04 {}: {} bytes exact runtime 0x{:x}–0x{:x} file 0x{:x}–0x{:x}",args[0],generated.len(),span::START,span::END,span::FILE_OFFSET,span::FILE_OFFSET+generated.len());
    Ok(())
}
