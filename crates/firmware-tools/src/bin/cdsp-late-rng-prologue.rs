// SPDX-License-Identifier: GPL-3.0-only
//! Typed CDSP V66 controller seed prologue and RWE prefix checks.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_rng_init as rng;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let a:Vec<_>=env::args().skip(1).collect();
    if a.len()!=2{return Err("usage: cdsp-late-rng-prologue <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match a[0].as_str(){
        "emit"=>{fs::write(&a[1],rng::build_seed_prologue()?)?;println!("Emitted {} independently encoded Hexagon bytes (partial function)",rng::VERIFIED_LEN);},
        "verify"=>{rng::verify_seed_prologue(&fs::read(&a[1])?)?;println!("Verified {} V66 RNG prologue bytes",rng::VERIFIED_LEN);},
        "emit-prefix"=>{fs::write(&a[1],rng::build_prefix_with_seed_prologue()?)?;println!("Emitted {} RWE layout bytes including {} executable",rng::RWE_LAYOUT_LEN,rng::EXECUTABLE_BYTES_IN_PREFIX);},
        "verify-prefix"=>{rng::verify_prefix_with_seed_prologue(&fs::read(&a[1])?)?;println!("Verified {} RWE layout bytes: {} executable and {} layout",rng::RWE_LAYOUT_LEN,rng::EXECUTABLE_BYTES_IN_PREFIX,rng::NON_CODE_LAYOUT_BYTES);},
        _=>return Err("unsupported operation".into()),
    }
    Ok(())
}
