// SPDX-License-Identifier: GPL-3.0-only
//! Standalone original-reference verifier for the Hexagon V66 bank prologue.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_bank_entry as bank;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<String>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-bank-entry <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
  "emit"=>fs::write(&args[1],bank::build_bank_entry()?)?,
  "verify"=>bank::verify(&fs::read(&args[1])?)?,
  "emit-prefix"=>fs::write(&args[1],bank::build_prefix()?)?,
  "verify-prefix"=>bank::verify_prefix(&fs::read(&args[1])?)?,
  _=>return Err("unsupported controller bank entry operation".into()),
 }
 println!("CDSP bank entry {}: file 0x{:x}..0x{:x}, {} generated executable bytes; prefix {} layout bytes ({} executable + {} layout)",
   args[0],bank::FILE_OFFSET,bank::END_FILE_OFFSET,bank::EXECUTABLE_LEN,
   bank::FULL_RWE_LAYOUT_LEN,bank::FULL_RWE_EXECUTABLE_LEN,bank::RWE_PREFIX_LAYOUT_LEN);
 Ok(())
}
