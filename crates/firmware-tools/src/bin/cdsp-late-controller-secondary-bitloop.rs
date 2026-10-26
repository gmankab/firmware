// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_secondary_bitloop as c;
use std::{env,fs,error::Error};
fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<_>=env::args().skip(1).collect();
 if args.len()!=2{return Err("usage: cdsp-late-controller-secondary-bitloop <emit|verify|emit-prefix|verify-prefix> FILE".into());}
 match args[0].as_str(){
 "emit"=>fs::write(&args[1],c::build_secondary_bitloop()?)?,
 "verify"=>c::verify(&fs::read(&args[1])?)?,
 "emit-prefix"=>fs::write(&args[1],c::build_prefix()?)?,
 "verify-prefix"=>c::verify_prefix(&fs::read(&args[1])?)?,
 _=>return Err("invalid command".into()),
 }
 println!("CDSP secondary bitloop: {} exact code bytes; prefix {} = {} executable + {} layout",
          c::EXECUTABLE_LEN,c::FULL_RWE_LAYOUT_LEN,c::FULL_RWE_EXECUTABLE_LEN,c::RWE_PREFIX_LAYOUT_LEN);
 Ok(())
}
