// SPDX-License-Identifier: GPL-3.0-only
//! CDSP partial reconstruction command. Reference bytes stay outside Git.
use firmware_tools::devices::xiaomi_nabu::cdsp_boot as cdsp;
use std::{env, fs};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    match arguments.as_slice() {
        [action, output] if action == "emit-entry" => {
            let bytes = cdsp::build_entry()?;
            fs::write(output, bytes)?;
            println!("Emitted {} independently encoded CDSP Hexagon entry bytes, file offsets {:#x}..{:#x}; partial firmware only.",
                cdsp::REBUILT_LEN, cdsp::START_FILE_OFFSET, cdsp::START_FILE_OFFSET + cdsp::REBUILT_LEN);
        }
        [action, original] if action == "verify-entry" => {
            let image = fs::read(original)?;
            cdsp::verify_reference(&image)?;
            println!("Verified {} CDSP Hexagon instruction bytes at file offsets {:#x}..{:#x} against private reference; remaining CDSP executable not rebuilt.",
                cdsp::REBUILT_LEN, cdsp::START_FILE_OFFSET, cdsp::START_FILE_OFFSET + cdsp::REBUILT_LEN);
        }
        [action, original] if action == "inspect" => {
            let image = fs::read(original)?;
            cdsp::validate_reference(&image)?;
            println!("CDSP machine={} ELF flags={:#x} entry physical={:#x}; first executable PT_LOAD file offset={:#x}, vaddr={:#x}, paddr={:#x}, filesz={:#x}.",
                cdsp::HEXAGON_ELF_MACHINE, cdsp::CDSP_ELF_FLAGS, cdsp::ENTRY_PHYS,
                cdsp::START_FILE_OFFSET, cdsp::ENTRY_ELF_VADDR, cdsp::ENTRY_PHYS, cdsp::FIRST_EXECUTABLE_SIZE);
        }
        _ => return Err("Usage: cargo run -p firmware-tools --bin cdsp-rebuild -- [inspect REFERENCE | verify-entry REFERENCE | emit-entry OUTPUT]".into()),
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("CDSP: {error}");
        std::process::exit(1);
    }
}
