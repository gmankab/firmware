// SPDX-License-Identifier: GPL-3.0-only
//! Emit and verify a bounded Hexagon V66 controller record lookup.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_record_lookup as lookup;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<_>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-record-lookup <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
  "emit"=>fs::write(&args[1],lookup::build_indexed_record_lookup()?)?,
  "verify"=>lookup::verify(&fs::read(&args[1])?)?,
  "emit-prefix"=>fs::write(&args[1],lookup::build_prefix()?)?,
  "verify-prefix"=>lookup::verify_prefix(&fs::read(&args[1])?)?,
  _=>return Err("invalid controller-record operation".into()),
 }
 println!("CDSP record lookup {}: file 0x{:x}..0x{:x}, {} executable bytes, {} prefix bytes",args[0],lookup::FILE_OFFSET,lookup::END_FILE_OFFSET,lookup::EXECUTABLE_LEN,lookup::FULL_RWE_LAYOUT_LEN);
 Ok(())
}
