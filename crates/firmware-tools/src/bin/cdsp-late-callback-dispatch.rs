// SPDX-License-Identifier: GPL-3.0-only
//! Standalone typed-CDSP Hexagon callback dispatcher and RWE prefix verifier.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_callback_dispatch as dispatch;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{
        return Err("usage: cdsp-late-callback-dispatch <emit|verify|emit-prefix|verify-prefix> FILE".into());
    }
    match args[0].as_str(){
        "emit"=>{
            let b=dispatch::build_callback_dispatcher()?;
            fs::write(&args[1],&b)?;
            println!("Emitted {} independently typed Hexagon callback bytes",b.len());
        },
        "verify"=>{
            dispatch::verify_dispatcher(&fs::read(&args[1])?)?;
            println!("Verified {} independently typed Hexagon callback bytes at CDSP file 0x56120",dispatch::EXECUTABLE_LEN);
        },
        "emit-prefix"=>{
            let b=dispatch::build_complete_rwe_prefix_layout()?;
            fs::write(&args[1],&b)?;
            println!("Emitted {} RWE bytes: {} executable, {} NOP/fill",b.len(),dispatch::SOURCE_GENERATED_EXECUTABLE_LEN,dispatch::EXPLICIT_INSTRUCTION_LAYOUT_LEN);
        },
        "verify-prefix"=>{
            dispatch::verify_rwe_prefix_layout(&fs::read(&args[1])?)?;
            println!("Verified 480-byte RWE layout: {} source-generated executable + {} structural NOP/fill bytes",dispatch::SOURCE_GENERATED_EXECUTABLE_LEN,dispatch::EXPLICIT_INSTRUCTION_LAYOUT_LEN);
        },
        _=>return Err("unknown command".into()),
    }
    Ok(())
}
