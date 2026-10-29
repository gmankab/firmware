// SPDX-License-Identifier: GPL-3.0-only
//! Standalone verifier for the new controller function prefix.
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_init_dispatch as controller;
use std::{env, fs, error::Error};
fn main()->Result<(),Box<dyn Error>> {
 let args:Vec<_>=env::args().skip(1).collect();
 if args.len()!=2 { return Err("usage: cdsp-late-controller-init <emit|verify|emit-prefix|verify-prefix> <output|reference.mbn>".into()); }
 match args[0].as_str() {
   "emit"=>fs::write(&args[1],controller::build_controller_input_start()?)?,
   "verify"=>controller::verify(&fs::read(&args[1])?)?,
   "emit-prefix"=>fs::write(&args[1],controller::build_prefix()?)?,
   "verify-prefix"=>controller::verify_prefix(&fs::read(&args[1])?)?,
   _=>return Err("unrecognized controller-init subcommand".into()),
 }
 println!("CDSP controller init {}: file 0x{:x}..0x{:x}; {} exact executable bytes; prefix {} bytes; independently verified only with reference",args[0],controller::FILE_OFFSET,controller::END_FILE_OFFSET,controller::EXECUTABLE_LEN,controller::FULL_RWE_LAYOUT_LEN);
 Ok(())
}
