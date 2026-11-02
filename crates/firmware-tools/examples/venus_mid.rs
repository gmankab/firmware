// SPDX-License-Identifier: GPL-3.0-only
//! Emit/verify standalone typed A32 Venus interval. The OEM MBN remains private.
use firmware_tools::devices::xiaomi_nabu::venus_runtime_mid as mid;
use std::{env,fs};
fn main()->Result<(),Box<dyn std::error::Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: <emit output.bin|verify private/venus.mbn>".into());}
    match args[0].as_str(){
      "emit"=>fs::write(&args[1],mid::fragment()?)?,
      "verify"=>{mid::verify_reference(&fs::read(&args[1])?)?;},
      _=>return Err("unsupported mode".into())
    }
    println!("agent20-03 {}: {} typed bytes runtime 0x{:x}..0x{:x} file 0x{:x}..0x{:x}; fragment only, not flashable",args[0],mid::END-mid::START,mid::START,mid::END,mid::FILE_OFFSET,mid::FILE_OFFSET+mid::END-mid::START);
    Ok(())
}
