// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::cdsp_late_controller_bank_initialize as bank;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=2 {return Err("usage: cdsp-late-controller-bank-initialize <emit|verify|emit-prefix|verify-prefix> FILE".into());}
    match args[0].as_str(){
        "emit"=>fs::write(&args[1],bank::build_bank_initializer()?)?,
        "verify"=>bank::verify(&fs::read(&args[1])?)?,
        "emit-prefix"=>fs::write(&args[1],bank::build_prefix()?)?,
        "verify-prefix"=>bank::verify_prefix(&fs::read(&args[1])?)?,
        _=>return Err("unsupported bank initializer command".into()),
    }
    println!("CDSP bank initializer {}: file 0x{:x}..0x{:x} {} executable bytes; prefix {} bytes ({} executable + {} layout)",
       args[0],bank::FILE_OFFSET,bank::END_FILE_OFFSET,bank::EXECUTABLE_LEN,
       bank::FULL_RWE_LAYOUT_LEN,bank::FULL_RWE_EXECUTABLE_LEN,bank::RWE_PREFIX_LAYOUT_LEN);
    Ok(())
}
