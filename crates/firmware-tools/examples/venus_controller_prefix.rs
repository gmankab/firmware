// SPDX-License-Identifier: GPL-3.0-only
//! Independently generated A32 controller prefix. Reference is private input.
use firmware_tools::devices::xiaomi_nabu::venus_controller_prefix as prefix;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<String>=env::args().skip(1).collect();
 if args.len()!=2 || !["emit","verify"].contains(&args[0].as_str()){
    return Err("usage: cargo run --example venus_controller_prefix -- <emit fragment.bin|verify private/venus.mbn>".into());
 }
 let bytes=prefix::fragment()?;
 assert_eq!(prefix::START,0x40000);
 assert_eq!(prefix::END,0x40a2c);
 assert_eq!(bytes.len(),2604);
 match args[0].as_str(){
    "emit"=>fs::write(&args[1],&bytes)?,
    "verify"=>{
        let original=fs::read(&args[1])?;
        let expected=original.get(prefix::FILE_OFFSET..prefix::FILE_OFFSET+bytes.len()).ok_or("Venus reference too short")?;
        if let Some(i)=bytes.iter().zip(expected).position(|(a,b)|a!=b){
           return Err(format!("Agent04 prefix mismatch at VMA 0x{:x}, MBN offset 0x{:x}",prefix::START+i,prefix::FILE_OFFSET+i).into());
        }
    },
    _=>unreachable!(),
 }
 println!("Agent 20-04 prefix {}: {} source-encoded bytes, {} instructions, {} data words; VMA 0x{:x}..0x{:x}",
         args[0],bytes.len(),prefix::INSTRUCTION_COUNT,prefix::DATA_WORD_COUNT,prefix::START,prefix::END);
 Ok(())
}
