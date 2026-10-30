// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_bank_bitset_scan as scan;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<_>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-bank-bitset-scan <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
  "emit"=>fs::write(&args[1],scan::build_bitmap_scan()?)?,
  "verify"=>scan::verify(&fs::read(&args[1])?)?,
  "emit-prefix"=>fs::write(&args[1],scan::build_prefix()?)?,
  "verify-prefix"=>scan::verify_prefix(&fs::read(&args[1])?)?,
  _=>return Err("unsupported bank-scan operation".into()),
 }
 println!("CDSP scan {}: file [0x{:x},0x{:x}), {} executable bytes; prefix {} bytes ({} executable, {} layout)",
 args[0],scan::FILE_OFFSET,scan::END_FILE_OFFSET,scan::EXECUTABLE_LEN,
 scan::FULL_RWE_LAYOUT_LEN,scan::FULL_RWE_EXECUTABLE_LEN,scan::RWE_PREFIX_LAYOUT_LEN);
 Ok(())
}
