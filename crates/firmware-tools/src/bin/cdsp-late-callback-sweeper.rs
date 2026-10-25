// SPDX-License-Identifier: GPL-3.0-only
//! CDSP callback-table sweeper and contiguous RWE-prefix emitter.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_callback_sweeper as sweeper;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let a:Vec<_>=env::args().skip(1).collect();
    if a.len()!=2 {return Err("usage: cdsp-late-callback-sweeper <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match a[0].as_str(){
        "emit"=>{
            let out=sweeper::build_callback_sweeper()?;
            fs::write(&a[1],&out)?;
            println!("Emitted {} exact-operand Hexagon executable bytes",out.len());
        },
        "verify"=>{
            sweeper::verify_callback_sweeper(&fs::read(&a[1])?)?;
            println!("Verified {} CDSP Hexagon bytes at 0x{:x}",sweeper::EXECUTABLE_LEN,sweeper::FILE_OFFSET);
        },
        "emit-prefix"=>{
            let out=sweeper::build_rwe_layout_through_sweeper()?;
            fs::write(&a[1],&out)?;
            println!("Emitted {} RWE prefix bytes: {} executable and 24 NOP/fill",out.len(),sweeper::FULL_RWE_EXECUTABLE_LEN);
        },
        "verify-prefix"=>{
            sweeper::verify_rwe_layout_through_sweeper(&fs::read(&a[1])?)?;
            println!("Verified {} RWE layout bytes",sweeper::FULL_RWE_LAYOUT_LEN);
        },
        _=>return Err("unknown operation".into()),
    }
    Ok(())
}
