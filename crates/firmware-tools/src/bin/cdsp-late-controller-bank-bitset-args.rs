// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_bank_bitset_args as args;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let a:Vec<_>=env::args().skip(1).collect();
 if a.len()!=2{return Err("usage: cdsp-late-controller-bank-bitset-args <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match a[0].as_str(){
  "emit"=>fs::write(&a[1],args::build_bitmap_args()?)?,
  "verify"=>args::verify(&fs::read(&a[1])?)?,
  "emit-prefix"=>fs::write(&a[1],args::build_prefix()?)?,
  "verify-prefix"=>args::verify_prefix(&fs::read(&a[1])?)?,
  _=>return Err("unsupported bitmap arguments operation".into()),
 }
 println!("CDSP bitmap args {}: file [0x{:x},0x{:x}), {} executable bytes; prefix {} bytes ({} executable, {} fill)",a[0],args::FILE_OFFSET,args::END_FILE_OFFSET,args::EXECUTABLE_LEN,args::FULL_RWE_LAYOUT_LEN,args::FULL_RWE_EXECUTABLE_LEN,args::RWE_PREFIX_LAYOUT_LEN);
 Ok(())
}
