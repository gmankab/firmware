// SPDX-License-Identifier: GPL-3.0-only
//! Isolated Agent17 CLI: no central registry or Cargo manifest changes.
#[path="../devices/xiaomi_nabu/novatek_ilmlow.rs"]
mod lower;
use std::{env,fs,process};
fn run()->Result<(),String>{
    let a:Vec<_>=env::args().skip(1).collect();
    if a.len()<2{return Err("usage: novatek-ilm-low emit|verify|listing PROFILE [FILE]".into());}
    let profile=lower::Profile::parse(&a[1])?;
    match a[0].as_str(){
        "emit" if a.len()==3=>{let bytes=lower::build(profile)?;fs::write(&a[2],&bytes).map_err(|e|e.to_string())?;
            println!("{}: {} instruction bytes, file [0x{:x},0x{:x}), runtime [0x{:x},0x{:x})",profile.name(),bytes.len(),lower::FILE_START,lower::FILE_END,lower::FILE_START-lower::ILM_FILE_BASE,lower::FILE_END-lower::ILM_FILE_BASE);},
        "verify" if a.len()==3=>{let firmware=fs::read(&a[2]).map_err(|e|e.to_string())?;lower::compare(&firmware,profile)?;
            println!("{}: zero byte mismatches in {} bytes; caller must pin whole-image SHA256 {}",profile.name(),lower::FILE_END-lower::FILE_START,profile.reference_sha256());},
        "listing" if a.len()==2=>{println!("# profile={} gp=0x{:x} itb=0x{:x} file_start=0x{:x} file_end=0x{:x}",profile.name(),profile.gp(),profile.itb(),lower::FILE_START,lower::FILE_END);
            for item in lower::program(profile){let b=item.instruction.encode(item.pc,profile.gp())?;
                println!("{:x}\t{}\t{}",item.pc,b.len(),item.instruction.assembly(item.pc,profile.gp()));}},
        _=>return Err("invalid novatek-ilm-low command or argument count".into()),
    }Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");process::exit(1)}}
