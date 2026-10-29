// SPDX-License-Identifier: GPL-3.0-only
//! Emit and verify the source-built Hexagon controller record-bound checks.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_record_bounds as bounds;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let a:Vec<String>=env::args().skip(1).collect();
 if a.len()!=2{return Err("usage: cdsp-late-controller-record-bounds <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match a[0].as_str(){
  "emit"=>fs::write(&a[1],bounds::build_record_bounds()?)?,
  "verify"=>bounds::verify(&fs::read(&a[1])?)?,
  "emit-prefix"=>fs::write(&a[1],bounds::build_prefix()?)?,
  "verify-prefix"=>bounds::verify_prefix(&fs::read(&a[1])?)?,
  _=>return Err("invalid record bounds operation".into()),
 }
 println!("controller record bounds {}: file [{:#x},{:#x}), executable={}, contiguous prefix={}",a[0],bounds::FILE_OFFSET,bounds::END_FILE_OFFSET,bounds::EXECUTABLE_LEN,bounds::FULL_RWE_LAYOUT_LEN);
 Ok(())
}
