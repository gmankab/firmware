// SPDX-License-Identifier: GPL-3.0-only
//! Emit and verify controller record cursor/return code in the original CDSP.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_record_advance as advance;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<_>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-record-advance <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
  "emit"=>fs::write(&args[1],advance::build_record_advance()?)?,
  "verify"=>advance::verify(&fs::read(&args[1])?)?,
  "emit-prefix"=>fs::write(&args[1],advance::build_prefix()?)?,
  "verify-prefix"=>advance::verify_prefix(&fs::read(&args[1])?)?,
  _=>return Err("unsupported controller record advance operation".into()),
 }
 println!("record advance {}: file [{:#x},{:#x}), {} executable, {} prefix bytes",args[0],advance::FILE_OFFSET,advance::END_FILE_OFFSET,advance::EXECUTABLE_LEN,advance::FULL_RWE_LAYOUT_LEN);
 Ok(())
}
