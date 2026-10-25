// SPDX-License-Identifier: GPL-3.0-only
//! Standalone agent-09 inspector. This bin references an agent-owned module
//! directly, so it can be cherry-picked onto checkpoint 50 without touching
//! central device `mod.rs` or unrelated firmware subcommands.
#[path = "../devices/xiaomi_nabu/novatek_layout.rs"]
mod novatek_layout;

use novatek_layout::{NabuNovatekLayout, HEADER_BYTES, IMAGE_LENGTH};
use std::{env, fs, process};

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 || args.len() > 3 {
        return Err("usage: novatek-nabu <inspect|emit-header|emit-padding|verify-header> <private-reference.bin> [output]".into());
    }
    let command = &args[0];
    let reference = fs::read(&args[1]).map_err(|e| e.to_string())?;
    let layout = NabuNovatekLayout::parse(&reference)?;
    match command.as_str() {
        "inspect" if args.len() == 2 => {
            let (gap_start, gap_end) = layout.padding_range()?;
            println!("Nabu Novatek image length 0x{IMAGE_LENGTH:x}, header 0x{HEADER_BYTES:x}, 2 metadata banks");
            for (name, p) in [("ILM",layout.ilm),("DLM",layout.dlm)] {
                println!("{name}: file 0x{:x}–0x{:x}; SRAM 0x{:x}; stored_length_minus_one 0x{:x}; stored_controller_crc 0x{:08x}",
                    p.file_offset,p.end()?,p.sram_address,p.size_minus_one,p.stored_crc);
            }
            for (i,bank) in layout.banks.iter().enumerate() {
                println!("bank {i}: mode={} vendor_flags=0x{:08x}",bank.mode,bank.options);
                for (j,info) in bank.info.iter().enumerate() {
                    println!("  Info-{j}: file=0x{:x} sram=0x{:x} size_minus_one=0x{:x} stored_crc=0x{:08x}",
                        info.file_offset,info.sram_address,info.size_minus_one,info.stored_crc);
                }
            }
            layout.write_padding(&reference)?;
            println!("independently constructible zero spacer 0x{gap_start:x}–0x{gap_end:x} = {} bytes",gap_end-gap_start);
            println!("CRC calculation and MCU ISA remain unverified; not a flashable reconstructed firmware");
        }
        "emit-header" if args.len() == 3 => {
            fs::write(&args[2],layout.write_header()).map_err(|e| e.to_string())?;
            println!("wrote {HEADER_BYTES} structured metadata bytes (CRC scalars are externally supplied)");
        }
        "emit-padding" if args.len() == 3 => {
            let padding=layout.write_padding(&reference)?;
            fs::write(&args[2],&padding).map_err(|e| e.to_string())?;
            println!("wrote {} independently constructed zero-padding bytes",padding.len());
        }
        "verify-header" if args.len() == 2 => {
            if layout.write_header() != reference[..HEADER_BYTES] {
                return Err("header does not round-trip".into());
            }
            let padding=layout.write_padding(&reference)?;
            println!("metadata verified: {HEADER_BYTES} structured bytes; {} independently constructed padding bytes; NVT suffix present",padding.len());
        }
        _ => return Err("bad command/arguments".into()),
    }
    Ok(())
}
fn main() { if let Err(e)=run() {eprintln!("error: {e}");process::exit(1);} }
