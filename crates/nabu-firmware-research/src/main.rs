// SPDX-License-Identifier: GPL-3.0-only
use nabu_firmware_research::{build_modemuw, machine_name, make_ghidra_bootstrap_view, parse_elf32, parse_novatek_candidate, parse_wmfw, qcom_mdt_hash_segments, wmfw_type_name};
use std::{env, error::Error, fs, path::PathBuf};
use nabu_firmware_research::{elf32_prefix::write_elf32_first_page, elf_specs::KNOWN_ELFS, wmfw_metadata::{build_wmfw_header, verify_wmfw_structure}};

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 2 && args[0] == "verify-a640-routines" {
        let input = fs::read(&args[1])?;
        let layout = nabu_firmware_research::a640_microcode::inspect(&input)?;
        let (stream, _, _) = nabu_firmware_research::a640_microcode::executable_segment(&input)?;
        let revision = (u32::from_le_bytes(stream[..4].try_into()?) & 0xfff) as u16;
        let entry = nabu_firmware_research::a640_routines::preemption_guard(layout.firmware_id, revision)?;
        let dispatch = nabu_firmware_research::a640_routines::security_setup_prefix(layout.firmware_id, revision)?;
        let idle = nabu_firmware_research::a640_routines::wait_for_idle()?;
        if !dispatch.starts_with(&entry) { return Err("A640 dispatch does not include preemption guard".into()); }
        if !stream.starts_with(&dispatch) { return Err("A640 typed Rust security-dispatch prefix mismatches".into()); }
        if !stream.ends_with(&idle) { return Err("A640 typed Rust wait routine mismatches".into()); }
        println!("Exact match: {}-byte typed QRISC security prefix (including {}-byte entry guard) plus {}-byte idle routine, {} distinct bytes.", dispatch.len(), entry.len(), idle.len(), dispatch.len() + idle.len());
        println!("Only these fragments are rebuilt; OEM-derived analysis guided their design. Whole image and signatures remain outstanding.");
        return Ok(());
    }
    if args.len() == 2 && args[0] == "verify-a640-isa-fields" {
        let firmware = fs::read(&args[1])?;
        let layout = nabu_firmware_research::a640_microcode::inspect(&firmware)?;
        let (stream, _, _) = nabu_firmware_research::a640_microcode::executable_segment(&firmware)?;
        let bounds = layout.shader.as_ref().map(|s| (s.start_word, s.end_word));
        let (encodable, unknown) = nabu_firmware_research::qrisc_codec::check_isa_fields(stream, bounds)?;
        println!("QRISC field decoding/reencoding: {} control words roundtrip, {} unknown (excluding embedded IR3).", encodable, unknown);
        // Distinguish Mesa's normal RET from IRET and summarize an
        // otherwise undocumented bit outside the published RET operands.
        let mut plain_return = 0;
        let mut marked_return = 0;
        let mut interrupt_return = 0;
        for (i, bytes) in stream.chunks_exact(4).enumerate() {
            if bounds.is_some_and(|(start,end)| (start..end).contains(&i)) { continue; }
            let value = u32::from_le_bytes(bytes.try_into()?);
            match nabu_firmware_research::qrisc_codec::QriscWord::decode(value) {
                Some(nabu_firmware_research::qrisc_codec::QriscWord::Return { ignored_bits: 0 }) => plain_return += 1,
                Some(nabu_firmware_research::qrisc_codec::QriscWord::Return { .. }) => marked_return += 1,
                Some(nabu_firmware_research::qrisc_codec::QriscWord::InterruptReturn { .. }) => interrupt_return += 1,
                _ => {}
            }
        }
        let typed_id = nabu_firmware_research::qrisc_codec::firmware_header(
            layout.firmware_id, (u32::from_le_bytes(stream[..4].try_into()?) & 0xfff) as u16
        )?.encode()?;
        println!("Normal RET: plain={plain_return}, uninterpreted-bits-set={marked_return}; IRET={interrupt_return}");
        println!("Header metadata via typed Rust fields matches source: {}", typed_id.to_le_bytes() == stream[..4]);
        println!("This is encoding-field verification against OEM inputs, NOT independently authored executable source.");
        return Ok(());
    }
    if args.len() == 2 && args[0] == "inspect-a640-flow" {
        let firmware = fs::read(&args[1])?;
        let layout = nabu_firmware_research::a640_microcode::inspect(&firmware)?;
        let (stream, _, _) = nabu_firmware_research::a640_microcode::executable_segment(&firmware)?;
        let shader = layout.shader.as_ref().map(|s| s.start_word..s.end_word);
        let flow = nabu_qrisc::flow::inspect(stream, 0, shader)?;
        println!("Potentially reachable QRISC words: {} / {}; branch targets: {}; call targets: {}; returns: {}",
            flow.reachable_words.len(), layout.total_words, flow.branch_targets.len(), flow.call_targets.len(), flow.returns);
        println!("This is conservative control-flow analysis; no GPU behavioral equivalence or safe replacement is demonstrated.");
        return Ok(());
    }
    if args.len() == 3 && args[0] == "extract-a640-isa" {
        let firmware = fs::read(&args[1])?;
        let layout = nabu_firmware_research::a640_microcode::inspect(&firmware)?;
        let (segment, _, _) = nabu_firmware_research::a640_microcode::executable_segment(&firmware)?;
        let output = PathBuf::from(&args[2]);
        fs::create_dir_all(&output)?;
        fs::write(output.join("a640-qrisc-pf_x.bin"), segment)?;
        if let Some(shader) = &layout.shader {
            fs::write(output.join("a640-candidate-ir3.bin"), &segment[shader.start_word*4..shader.end_word*4])?;
        }
        println!("A640: {} words, firmware-ID 0x{:03x}, branch={} call={} return={}",
            layout.total_words, layout.firmware_id, layout.qrisc_branches, layout.qrisc_calls, layout.qrisc_returns);
        if let Some(s) = &layout.shader {
            println!("IR3 CANDIDATE ONLY: segment words {}..{}, {} slots; independently validate with ir3-disasm", s.start_word, s.end_word, s.ir3_slots());
        } else { println!("No embedded IR3 region recognized by limited jump-over-data heuristic"); }
        println!("Analysis-only files written to {}; NOT firmware build outputs", output.display());
        return Ok(());
    }
    if args.len() == 3 && args[0] == "make-ghidra-bootstrap" {
        let original = fs::read(&args[1])?;
        let prepared = make_ghidra_bootstrap_view(&original)?;
        fs::write(&args[2], prepared)?;
        println!("TEMPORARY ANALYSIS ONLY (DO NOT FLASH): wrote {}", args[2]);
        return Ok(());
    }
    if args.len() == 2 && args[0] == "rebuild-wmfw-header" {
        fs::write(&args[1], build_wmfw_header())?;
        println!("Generated 40-byte WMFW metadata header (NOT DSP firmware)");
        return Ok(());
    }
    if args.len() == 2 && args[0] == "verify-wmfw-structure" {
        let matched = verify_wmfw_structure(&fs::read(&args[1])?)?;
        println!("Matched {} bytes of independently rebuilt WMFW metadata; NOT executable payload", matched);
        return Ok(());
    }
    if args.len() == 3 && args[0] == "rebuild-elf-prefix" {
        let spec = KNOWN_ELFS.iter().find(|x| x.name == args[1])
            .ok_or("unknown ELF32 firmware filename")?;
        fs::write(&args[2], write_elf32_first_page(spec)?)?;
        println!("Generated {} bytes of metadata-only ELF header for {} (NOT firmware)", 4096, spec.name);
        return Ok(());
    }
    if args.len() == 3 && args[0] == "verify-elf-prefix" {
        let spec = KNOWN_ELFS.iter().find(|x| x.name == args[1])
            .ok_or("unknown ELF32 firmware filename")?;
        let expected = write_elf32_first_page(spec)?;
        let reference = fs::read(&args[2])?;
        if reference.len() < expected.len() || reference[..expected.len()] != expected {
            return Err("ELF metadata-prefix byte mismatch".into());
        }
        println!("Byte-identical metadata prefix: {} ({} bytes; payload not rebuilt)", spec.name, expected.len());
        return Ok(());
    }
    if args.len() != 2 {
        return Err("usage: nabu-firmware-research <rebuild-modemuw|verify-modemuw|inspect-elf32|inspect-image> PATH \
            OR make-ghidra-bootstrap ORIGINAL OUTPUT-ANALYSIS-ONLY OR rebuild-elf-prefix NAME OUTPUT OR verify-elf-prefix NAME ORIGINAL \
            OR extract-a640-isa ORIGINAL_MBN OUTDIR OR verify-a640-isa-fields ORIGINAL_MBN".into());
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
