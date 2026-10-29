// SPDX-License-Identifier: GPL-3.0-only
//! Emit/check checked Hexagon controller candidate selection and RWE prefix.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_selection as selection;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<_>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-selection <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
  "emit"=>fs::write(&args[1],selection::build_candidate_selection()?)?,
  "verify"=>selection::verify(&fs::read(&args[1])?)?,
  "emit-prefix"=>fs::write(&args[1],selection::build_prefix()?)?,
  "verify-prefix"=>selection::verify_prefix(&fs::read(&args[1])?)?,
  _=>return Err("invalid controller-selection action".into()),
 }
 println!("CDSP controller selection {}: file {:#x}..{:#x}, {} executable bytes, {} total RWE prefix bytes",args[0],selection::FILE_OFFSET,selection::END_FILE_OFFSET,selection::EXECUTABLE_LEN,selection::FULL_RWE_LAYOUT_LEN);
 Ok(())
}
