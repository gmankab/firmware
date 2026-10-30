// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_bank_bitset_entry as bank;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<String>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-bank-bitset-entry <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
  "emit"=>fs::write(&args[1],bank::build_bank_bitset_entry()?)?,
  "verify"=>bank::verify(&fs::read(&args[1])?)?,
  "emit-prefix"=>fs::write(&args[1],bank::build_prefix()?)?,
  "verify-prefix"=>bank::verify_prefix(&fs::read(&args[1])?)?,
  _=>return Err("unsupported CDSP bank bitmap operation".into()),
 }
 println!("CDSP bank bitmap {}: file [0x{:x},0x{:x}), {} executable bytes; prefix {} bytes ({} executable, {} fill)",
    args[0],bank::FILE_OFFSET,bank::END_FILE_OFFSET,bank::EXECUTABLE_LEN,
    bank::FULL_RWE_LAYOUT_LEN,bank::FULL_RWE_EXECUTABLE_LEN,bank::RWE_PREFIX_LAYOUT_LEN);
 Ok(())
}
