// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_bank_bitset_limits as limits;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<_>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-bank-bitset-limits <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
  "emit"=>fs::write(&args[1],limits::build_bitmap_limits()?)?,
  "verify"=>limits::verify(&fs::read(&args[1])?)?,
  "emit-prefix"=>fs::write(&args[1],limits::build_prefix()?)?,
  "verify-prefix"=>limits::verify_prefix(&fs::read(&args[1])?)?,
  _=>return Err("unsupported bank-limit operation".into()),
 }
 println!("CDSP limits {}: file [0x{:x},0x{:x}), {} executable bytes; prefix {} bytes ({} executable, {} layout)",
 args[0],limits::FILE_OFFSET,limits::END_FILE_OFFSET,limits::EXECUTABLE_LEN,
 limits::FULL_RWE_LAYOUT_LEN,limits::FULL_RWE_EXECUTABLE_LEN,limits::RWE_PREFIX_LAYOUT_LEN);
 Ok(())
}
