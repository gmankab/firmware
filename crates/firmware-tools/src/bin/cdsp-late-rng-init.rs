// SPDX-License-Identifier: GPL-3.0-only
//! CDSP seed input callback function through file 0x563c0.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_rng_seed_state as seed;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: cdsp-late-rng-init <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match args[0].as_str(){
        "emit"=>{let b=seed::build_seed_initialization()?;fs::write(&args[1],&b)?;println!("Emitted {} typed Hexagon initialization bytes",b.len());},
        "verify"=>{seed::verify_seed_initialization(&fs::read(&args[1])?)?;println!("Verified {} exact code bytes",seed::EXECUTABLE_LEN);},
        "emit-prefix"=>{let b=seed::build_prefix_through_seed_state()?;fs::write(&args[1],&b)?;println!("Emitted {} RWE layout bytes: {} executable, {} NOP/fill",seed::RWE_LAYOUT_LEN,seed::PREFIX_EXECUTABLE_LEN,seed::PREFIX_LAYOUT_NOP_LEN);},
        "verify-prefix"=>{seed::verify_seed_state_prefix(&fs::read(&args[1])?)?;println!("Verified {} complete RWE source-plus-layout bytes",seed::RWE_LAYOUT_LEN);},
        _=>return Err("unknown operation".into()),
    }
    Ok(())
}
