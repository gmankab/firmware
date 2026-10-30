// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_bank_predicates as pred;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<String>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-bank-predicates <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
  "emit"=>fs::write(&args[1],pred::build_controller_predicates()?)?,
  "verify"=>pred::verify(&fs::read(&args[1])?)?,
  "emit-prefix"=>fs::write(&args[1],pred::build_prefix()?)?,
  "verify-prefix"=>pred::verify_prefix(&fs::read(&args[1])?)?,
  _=>return Err("invalid controller predicate operation".into()),
 }
 println!("CDSP bank predicates {}: file [0x{:x},0x{:x}), {} executable bytes; {}-byte RWE prefix ({} executable + {} layout)",
 args[0],pred::FILE_OFFSET,pred::END_FILE_OFFSET,pred::EXECUTABLE_LEN,pred::FULL_RWE_LAYOUT_LEN,pred::FULL_RWE_EXECUTABLE_LEN,pred::RWE_PREFIX_LAYOUT_LEN);
 Ok(())
}
