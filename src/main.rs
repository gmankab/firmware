// SPDX-License-Identifier: GPL-3.0-only
use nabu_firmware_research::{build_modemuw, machine_name, make_ghidra_bootstrap_view, parse_elf32, parse_novatek_candidate, parse_wmfw, qcom_mdt_hash_segments, wmfw_type_name};
use std::{env, error::Error, fs, path::PathBuf};

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 3 && args[0] == "make-ghidra-bootstrap" {
        let original = fs::read(&args[1])?;
        let prepared = make_ghidra_bootstrap_view(&original)?;
        fs::write(&args[2], prepared)?;
        println!("TEMPORARY ANALYSIS ONLY (DO NOT FLASH): wrote {}", args[2]);
        return Ok(());
    }
    if args.len() != 2 {
        return Err("usage: nabu-firmware-research <rebuild-modemuw|verify-modemuw|inspect-elf32|inspect-image> PATH \
            OR make-ghidra-bootstrap ORIGINAL OUTPUT-ANALYSIS-ONLY".into());
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
        "inspect-image" => {
            let b = fs::read(&target)?;
            println!("file: {}; bytes: {}", target.display(), b.len());
            if b.starts_with(b"\x7fELF") {
                let elf = parse_elf32(&b)?;
                let hashes = qcom_mdt_hash_segments(&elf);
                println!("ELF32 architecture={} entry=0x{:x} phdrs={} qcom_hash_segments={}",
                    machine_name(elf.machine), elf.entry, elf.program_headers.len(), hashes.len());
                for (index, ph) in elf.program_headers.iter().enumerate() {
                    println!("  [{}] kind=0x{:x} flags=0x{:08x} offset=0x{:x} size={} memory={} vaddr=0x{:x} paddr=0x{:x} hash={}",
                        index, ph.kind, ph.flags, ph.offset, ph.file_size, ph.mem_size, ph.vaddr, ph.paddr,
                        ph.flags & 0x0700_0000 == 0x0200_0000);
                    if ph.file_size != 0 && (ph.offset as u64 + ph.file_size as u64) > b.len() as u64 {
                        println!("  INVALID: nonempty segment runs past EOF");
                    }
                }
                if !hashes.is_empty() {
                    println!("WARNING: auth metadata present; does not establish signature validity or device enforcement");
                }
            } else if b.starts_with(b"WMFW") {
                let firmware = parse_wmfw(&b)?;
                println!("WMFW version={} DSP_core={} revision={} regions={}",
                    firmware.version, firmware.core, firmware.revision, firmware.region_headers.len());
                for r in &firmware.region_headers {
                    println!("  @0x{:x} type=0x{:02x} ({}) dsp_offset=0x{:x} payload_bytes={}",
                        r.file_offset, r.region_type, wmfw_type_name(r.region_type), r.dsp_offset, r.payload_length);
                }
            } else if b.starts_with(b"{") {
                println!("text configuration candidate; no executable decompilation");
            } else if b.ends_with(b"NVT") {
                // The Novatek updater uses an 'NVT' end marker: a format marker, not a signature.
                println!("raw Novatek-style image: NVT end marker present; no executable header");
                match parse_novatek_candidate(&b) {
                    Ok(h) => {
                        println!("TENTATIVE NT36xxx-family format: header={} NVT_marker={} descriptors={}", h.header_length, h.has_nvt_marker, h.partitions.len());
                        for p in h.partitions.iter().take(4) {
                            println!("  {}: file=0x{:x} sram=0x{:x} size_minus_one={} checksum_word=0x{:08x}",
                                p.kind, p.file_offset, p.sram_address, p.size_minus_one, p.checksum_word);
                        }
                        println!("WARNING: candidate descriptor map is NOT verified for NT36523");
                    }
                    Err(e) => println!("candidate header parse rejected: {e}"),
                }
                println!("CPU instruction set not identified; no pseudocode claimed");
            } else {
                println!("unknown raw firmware (no supported executable header); CPU ISA must be established before disassembly");
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
