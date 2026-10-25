// SPDX-License-Identifier: GPL-3.0-only
//! Standalone agent-09 inspector. This bin references an agent-owned module
//! directly, so it can be cherry-picked onto checkpoint 50 without touching
//! central device `mod.rs` or unrelated firmware subcommands.
#[path = "../devices/xiaomi_nabu/novatek_layout.rs"]
mod novatek_layout;
#[path = "../devices/xiaomi_nabu/novatek_reassemble.rs"]
mod novatek_reassemble;

use novatek_layout::{NabuNovatekLayout, HEADER_BYTES, IMAGE_LENGTH, RESERVED_TAIL_START, END_MARKER_OFFSET};
use std::{env, fs, process};

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("assemble-structural-unverified") && args.len() == 6 {
        let meta=fs::read(&args[1]).map_err(|e|e.to_string())?;
        let layout=NabuNovatekLayout::parse(&meta)?;
        let ilm=fs::read(&args[2]).map_err(|e|e.to_string())?;
        let dlm=fs::read(&args[3]).map_err(|e|e.to_string())?;
        let aux=fs::read(&args[4]).map_err(|e|e.to_string())?;
        let built=novatek_reassemble::assemble_structural_unverified(&layout,
            novatek_reassemble::AuthoredPayloads{
                ilm:Some(&ilm),dlm:Some(&dlm),auxiliary:Some(&aux)
            })?;
        fs::write(&args[5],&built.bytes).map_err(|e|e.to_string())?;
        println!("Wrote {} structural bytes; hardware CRC verified: {}. WARNING: ISA semantics and payload provenance NOT verified; do NOT flash.", built.bytes.len(),built.is_hardware_crc_verified());
        return Ok(());
    }
    if args.len() < 2 || args.len() > 3 {
        return Err("usage: novatek-nabu <inspect|emit-header|emit-padding|emit-tail-padding|emit-marker|verify-header|verify-envelope> <private-reference.bin> [output] | assemble-structural-unverified <private-meta-reference> <independently-authored-ilm> <independently-authored-dlm> <independently-authored-aux> <output>".into());
    }
    let command = &args[0];
    let reference = fs::read(&args[1]).map_err(|e| e.to_string())?;
    let layout = NabuNovatekLayout::parse(&reference)?;
    // Structural assembly has no implicit fallback to the uploaded reference.
    // Only the explicit assemble-structural-unverified command supplies parts.
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
            let independent = layout.coverage()?.iter().filter(|r| r.source_independent()).map(|r| r.length()).sum::<usize>();
            let unresolved: usize = layout.coverage()?.iter().filter(|r|r.is_unimplemented())
                .map(|r|r.length()).sum();
            println!("independently reconstructible envelope bytes: {independent}");
            println!("unreconstructed firmware payload bytes: {unresolved}");
            println!("verified reserved tail: 0x{RESERVED_TAIL_START:x}–0x{END_MARKER_OFFSET:x}");
            for region in novatek_reassemble::projected_regions(&layout)? { let range=region.range; println!("  {:>6} bytes 0x{:05x}..0x{:05x} {:?} {}",
                range.length(),range.start,range.end,range.kind,range.name); }
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
        "emit-tail-padding" if args.len() == 3 => {
            let padding=layout.write_reserved_tail(&reference)?;
            fs::write(&args[2],&padding).map_err(|e|e.to_string())?;
            println!("wrote {} independently constructed end-padding bytes",padding.len());
        }
        "emit-marker" if args.len() == 3 => {
            fs::write(&args[2],layout.write_marker()).map_err(|e|e.to_string())?;
            println!("wrote {} fixed suffix bytes",layout.write_marker().len());
        }
        "verify-envelope" if args.len() == 2 => {
            layout.verify_envelope(&reference)?;
            let independent=layout.coverage()?.iter().filter(|r|r.source_independent())
                .map(|r|r.length()).sum::<usize>();
            let projection=novatek_reassemble::projected_regions(&layout)?;
            let absent=projection.iter().filter(|r|r.bytes.is_none()).count();
            println!("Novatek metadata/envelope verified; {independent} independently constructible bytes; {absent} ILM/DLM/auxiliary holes remain unknown");
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
