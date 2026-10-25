// SPDX-License-Identifier: GPL-3.0-only
//! Typed CDSP V66 seed mixing/return and full RWE layout verification.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_rng_update as upd;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let a:Vec<_>=env::args().skip(1).collect();
    if a.len()!=2{return Err("usage: cdsp-late-rng-update <emit|verify|emit-seed|verify-seed|emit-prefix|verify-prefix> FILE".into());}
    match a[0].as_str(){
        "emit"=>{let b=upd::build_seed_update_and_return()?;fs::write(&a[1],&b)?;println!("Emitted {} typed V66 seed-update bytes",b.len());},
        "verify"=>{upd::verify_seed_update(&fs::read(&a[1])?)?;println!("Verified {} exact CDSP seed-update bytes",upd::EXECUTABLE_LEN);},
        "emit-seed"=>{fs::write(&a[1],upd::build_full_seed_function()?)?;println!("Emitted full CDSP 296-byte state-update function");},
        "verify-seed"=>{upd::verify_full_seed_function(&fs::read(&a[1])?)?;println!("Verified entire CDSP 296-byte seed function");},
        "emit-prefix"=>{fs::write(&a[1],upd::build_rwe_prefix_through_seed_return()?)?;println!("Emitted {} contiguous RWE bytes: {} executable, {} NOP/fill",upd::FULL_RWE_LAYOUT_LEN,upd::FULL_RWE_EXECUTABLE_LEN,upd::RWE_LAYOUT_NOP_LEN);},
        "verify-prefix"=>{upd::verify_rwe_prefix_through_seed_return(&fs::read(&a[1])?)?;println!("Verified {} CDSP RWE layout bytes",upd::FULL_RWE_LAYOUT_LEN);},
        _=>return Err("unknown operation".into()),
    }
    Ok(())
}
