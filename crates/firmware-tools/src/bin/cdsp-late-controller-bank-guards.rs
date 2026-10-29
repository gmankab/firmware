// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_bank_guards as g;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<String>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-bank-guards <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
  "emit"=>fs::write(&args[1],g::build_bank_guards()?)?,
  "verify"=>g::verify(&fs::read(&args[1])?)?,
  "emit-prefix"=>fs::write(&args[1],g::build_prefix()?)?,
  "verify-prefix"=>g::verify_prefix(&fs::read(&args[1])?)?,
  _=>return Err("unsupported bank guard command".into())
 }
 println!("CDSP guards {}: file [0x{:x},0x{:x}), {} V66 executable bytes, joined prefix {} bytes",args[0],g::FILE_OFFSET,g::END_FILE_OFFSET,g::EXECUTABLE_LEN,g::FULL_RWE_LAYOUT_LEN);
 Ok(())
}
