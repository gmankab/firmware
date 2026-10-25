// SPDX-License-Identifier: GPL-3.0-only
//! Agent 03: opt-in verification and emission of independently encoded A32.
use firmware_tools::devices::xiaomi_nabu::venus_runtime_late as late;
use std::{env,fs,process};
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let args:Vec<String>=env::args().collect();
    if args.len()!=3 {
        eprintln!("usage: cargo run --example agent03_venus_late -- <emit|verify> <path>");
        process::exit(2);
    }
    match args[1].as_str() {
        "emit" => {
            let bytes=late::fragment()?;
            fs::write(&args[2], &bytes)?;
            println!("emitted {} bytes, ARM 0x{:x}–0x{:x}; disjoint, NOT flashable",bytes.len(),late::START,late::END);
        },
        "verify" => {
            let private_reference=fs::read(&args[2])?;
            let len=late::verify_reference(&private_reference)?;
            println!("verified {len} independent late Venus bytes, runtime 0x{:x}–0x{:x}, file 0x{:x}–0x{:x}",late::START,late::END,late::FILE_BASE+late::START,late::FILE_BASE+late::END);
        },
        _ => {eprintln!("unsupported mode {}",args[1]);process::exit(2);}
    }
    Ok(())
}
