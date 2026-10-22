// SPDX-License-Identifier: GPL-3.0-only
use nabu_firmware_research::{build_modemuw, machine_name, parse_elf32};
use std::{env, error::Error, fs, path::PathBuf};

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: nabu-firmware-research <rebuild-modemuw|verify-modemuw|inspect-elf32> PATH".into());
    }
    let target = PathBuf::from(&args[1]);
    match args[0].as_str() {
        "rebuild-modemuw" => {
            fs::write(&target, build_modemuw().as_bytes())?;
            println!("generated {} bytes at {}", build_modemuw().len(), target.display());
        }
        "verify-modemuw" => {
            let original = fs::read(&target)?;
            let rebuilt = build_modemuw();
            if original != rebuilt.as_bytes() {
                return Err(format!("NOT byte-identical: reference {} bytes, rebuilt {} bytes", original.len(), rebuilt.len()).into());
            }
            println!("byte-identical: {} ({} bytes)", target.display(), original.len());
        }
        "inspect-elf32" => {
            let elf = parse_elf32(&fs::read(&target)?)?;
            println!("architecture: {} (e_machine {})", machine_name(elf.machine), elf.machine);
            println!("entry: 0x{:x}; program headers: {}", elf.entry, elf.program_headers.len());
            for (i, ph) in elf.program_headers.iter().enumerate() {
                println!("{i}: type {:#x} file @{:#x} size {:#x}, virtual {:#x}, physical {:#x}, memory {:#x}, flags {:#x}",
                    ph.kind, ph.offset, ph.file_size, ph.vaddr, ph.paddr, ph.mem_size, ph.flags);
            }
        }
        _ => return Err(format!("unknown subcommand: {}", args[0]).into()),
    }
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
