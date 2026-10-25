// SPDX-License-Identifier: GPL-3.0-only
//! Optional reference verification or independent emission of Agent 20-02's seam.
use firmware_tools::devices::xiaomi_nabu::venus_runtime_seam as seam;
use std::{env,fs};
fn main()->Result<(),Box<dyn std::error::Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: cargo run --example agent20_02_venus_seam -- <emit|verify> path".into());}
    match args[0].as_str(){
      "emit"=>fs::write(&args[1],seam::fragment()?)?,
      "verify"=>{seam::verify_reference(&fs::read(&args[1])?)?;},
      _=>return Err("unknown mode: use emit or verify".into()),
    }
    println!("{}: {} typed A32 bytes, runtime [0x{:x},0x{:x}), MBN file [0x{:x},0x{:x}); source-encoded, partial firmware",args[0],seam::END-seam::START,seam::START,seam::END,seam::FILE_OFFSET,seam::FILE_OFFSET+seam::END-seam::START);
    Ok(())
}
