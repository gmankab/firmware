// SPDX-License-Identifier: GPL-3.0-only
//! Emit or check controller status forwarding and scan-loop initialization.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_status as status;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<_>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-status <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
  "emit"=>fs::write(&args[1],status::build_controller_status()?)?,
  "verify"=>status::verify(&fs::read(&args[1])?)?,
  "emit-prefix"=>fs::write(&args[1],status::build_prefix()?)?,
  "verify-prefix"=>status::verify_prefix(&fs::read(&args[1])?)?,
  _=>return Err("invalid controller status operation".into()),
 }
 println!("CDSP status {}: file 0x{:x}..0x{:x}, {} executable bytes, {} RWE prefix",args[0],status::FILE_OFFSET,status::END_FILE_OFFSET,status::EXECUTABLE_LEN,status::FULL_RWE_LAYOUT_LEN);
 Ok(())
}
