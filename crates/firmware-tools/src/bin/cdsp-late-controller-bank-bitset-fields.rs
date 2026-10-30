// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_bank_bitset_fields as fields;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
 let a:Vec<_>=env::args().skip(1).collect();
 if a.len()!=2{return Err("usage: cdsp-late-controller-bank-bitset-fields <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match a[0].as_str(){
  "emit"=>fs::write(&a[1],fields::build_bitmap_fields()?)?,
  "verify"=>fields::verify(&fs::read(&a[1])?)?,
  "emit-prefix"=>fs::write(&a[1],fields::build_prefix()?)?,
  "verify-prefix"=>fields::verify_prefix(&fs::read(&a[1])?)?,
  _=>return Err("unsupported controller bitmap field operation".into()),
 }
 println!("CDSP bitmap fields {}: file [0x{:x},0x{:x}), {} executable bytes; prefix {} bytes ({} executable, {} fill)",a[0],fields::FILE_OFFSET,fields::END_FILE_OFFSET,fields::EXECUTABLE_LEN,fields::FULL_RWE_LAYOUT_LEN,fields::FULL_RWE_EXECUTABLE_LEN,fields::RWE_PREFIX_LAYOUT_LEN);
 Ok(())
}
