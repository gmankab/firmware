// SPDX-License-Identifier: GPL-3.0-only
//! Standalone reconstruction CLI for CDSP record error packets and prefix.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_record_diagnostics as diag;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let a:Vec<String>=env::args().skip(1).collect();
 if a.len()!=2{return Err("usage: cdsp-late-controller-record-diagnostics <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match a[0].as_str(){
  "emit"=>fs::write(&a[1],diag::build_record_diagnostics()?)?,
  "verify"=>diag::verify(&fs::read(&a[1])?)?,
  "emit-prefix"=>fs::write(&a[1],diag::build_prefix()?)?,
  "verify-prefix"=>diag::verify_prefix(&fs::read(&a[1])?)?,
  _=>return Err("invalid diagnostics operation".into()),
 }
 println!("controller diagnostics {}: file [0x{:x},0x{:x}) executable={} prefix-layout={}",a[0],diag::FILE_OFFSET,diag::END_FILE_OFFSET,diag::EXECUTABLE_LEN,diag::FULL_RWE_LAYOUT_LEN);
 Ok(())
}
