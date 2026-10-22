// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::{build_modemuw, machine_name, make_ghidra_bootstrap_view, parse_elf32, parse_novatek_candidate, parse_wmfw, qcom_mdt_hash_segments, wmfw_type_name};
use std::{env, error::Error, fs, path::PathBuf};
use firmware_tools::{elf32_prefix::write_elf32_first_page, devices::xiaomi_nabu::elf_specs::KNOWN_ELFS, wmfw_metadata::{build_wmfw_header, verify_wmfw_structure}};

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 2 && args[0] == "emit-venus-contiguous-boot" {
        let output = firmware_tools::devices::xiaomi_nabu::venus_plan::first_contiguous_region()?
            .build_complete()?;
        fs::write(&args[1], &output)?;
        println!("Emitted {} contiguous initial Venus bytes. Not a complete firmware image.", output.len());
        return Ok(());
    }
    if args.len() == 2 && args[0] == "verify-venus-contiguous-boot" {
        let reference = fs::read(&args[1])?;
        let offset = firmware_tools::devices::xiaomi_nabu::venus_plan::START_FILE_OFFSET;
        let plan = firmware_tools::devices::xiaomi_nabu::venus_plan::first_contiguous_region()?;
        let output = plan.build_complete()?;
        if reference.get(offset..offset + output.len()) != Some(output.as_slice()) {
            return Err("Venus contiguous ARM32 boot image differs from reference".into());
        }
        println!("Exact Venus contiguous boot region: {} bytes at file offset 0x{:x}; not a complete firmware image.", output.len(), offset);
        return Ok(());
    }
    if args.len() == 2 && args[0] == "emit-venus-runtime-prefix" {
        let output = firmware_tools::devices::xiaomi_nabu::venus_runtime::prefix()?;
        fs::write(&args[1], &output)?;
        println!("Emitted {} bytes of Venus ARM32 runtime stubs; partial, not runnable firmware.", output.len());
        return Ok(());
    }
    if args.len() == 2 && args[0] == "verify-venus-runtime-prefix" {
        let reference = fs::read(&args[1])?;
        let offset = firmware_tools::devices::xiaomi_nabu::venus_runtime::FILE_OFFSET;
        let output = firmware_tools::devices::xiaomi_nabu::venus_runtime::prefix()?;
        if reference.get(offset..offset + output.len()) != Some(output.as_slice()) {
            return Err("Venus ARM32 runtime prefix differs from reference".into());
        }
        println!("Exact Venus runtime prefix: {} bytes at file offset 0x{:x}.", output.len(), offset);
        return Ok(());
    }
    if args.len() == 2 && args[0] == "emit-venus-boot-page" {
        let output = firmware_tools::devices::xiaomi_nabu::venus_boot::first_page()?;
        fs::write(&args[1], &output)?;
        println!("Emitted {}-byte Venus boot page, with checked ARM32 code, data and zero-fill.", output.len());
        println!("Partial Venus firmware only; no authentication or full image produced.");
        return Ok(());
    }
    if args.len() == 2 && args[0] == "verify-venus-boot-page" {
        let reference = fs::read(&args[1])?;
        let offset = firmware_tools::devices::xiaomi_nabu::venus_boot::FILE_OFFSET;
        let page = firmware_tools::devices::xiaomi_nabu::venus_boot::first_page()?;
        let end = offset.checked_add(page.len()).ok_or("Venus boot-page offset overflow")?;
        if reference.get(offset..end) != Some(page.as_slice()) {
            return Err("Venus ARM32 boot page differs from reference firmware".into());
        }
        println!("Exact Venus boot page: {} bytes at file offset 0x{:x} ({} structured bytes plus {} zero-fill).",
            page.len(), offset, firmware_tools::devices::xiaomi_nabu::venus_boot::PREFIX_SIZE,
            page.len() - firmware_tools::devices::xiaomi_nabu::venus_boot::PREFIX_SIZE);
        return Ok(());
    }
    if args.len() == 2 && args[0] == "emit-venus-boot-prefix" {
        let output = firmware_tools::devices::xiaomi_nabu::venus_boot::prefix()?;
        fs::write(&args[1], &output)?;
        println!("Emitted {} typed ARM32 boot-prefix bytes (not complete Venus firmware).", output.len());
        return Ok(());
    }
    if args.len() == 2 && args[0] == "verify-venus-boot-prefix" {
        let reference = fs::read(&args[1])?;
        let offset = firmware_tools::devices::xiaomi_nabu::venus_boot::FILE_OFFSET;
        let output = firmware_tools::devices::xiaomi_nabu::venus_boot::prefix()?;
        let end = offset.checked_add(output.len()).ok_or("Venus boot range overflow")?;
        if reference.get(offset..end) != Some(output.as_slice()) {
            return Err("Venus typed ARM32 boot prefix differs from the reference".into());
        }
        println!("Exact Venus ARM32 boot-prefix match: {} bytes at file offset 0x{:x}.", output.len(), offset);
        return Ok(());
    }
    if args.len() == 3 && args[0] == "assemble-a640-with-external-auth" {
        let external_authentication = fs::read(&args[1])?;
        let completed = firmware_tools::devices::xiaomi_nabu::a640_container::
            assemble_with_external_authentication(&external_authentication)?;
        fs::write(&args[2], &completed)?;
        println!("Assembled {}-byte A640 MBN with externally provided OEM authentication bytes.", completed.len());
        println!("SHA-384 ELF segment integrity verified. OEM signature, certificate trust, and hardware acceptance were NOT verified.");
        return Ok(());
    }
    if args.len() == 2 && args[0] == "verify-mbnv6-hashes" {
        let firmware = fs::read(&args[1])?;
        let report = firmware_tools::qcom_mbn_validation::verify_elf32_mbnv6_hashes(&firmware)?;
        println!("Valid MBNv6 SHA-384 hash table: {} hashed ELF segments, {} un-hashed entries.",
            report.hashed_count, report.zero_placeholder_count);
        println!("Authentication segment at 0x{:x}, {} bytes; signer material present: {}.",
            report.hash_segment_file_offset, report.hash_segment_length, report.has_signatures);
        println!("HASH AGREEMENT ONLY. OEM/QTI certificate chains, hardware trust and signatures were NOT verified.");
        return Ok(());
    }
    if args.len() == 1 && args[0] == "a640-coverage" {
        let plan = firmware_tools::devices::xiaomi_nabu::a640_plan::partial_executable()?;
        println!("A640 typed executable coverage: {} / {} bytes ({:.1}%)",
            plan.filled_bytes(), plan.total_bytes(),
            plan.filled_bytes() as f64 * 100.0 / plan.total_bytes() as f64);
        for gap in plan.holes() {
            println!("  UNIMPLEMENTED 0x{:x}..0x{:x} ({} bytes)",
                gap.start, gap.end, gap.end-gap.start);
        }
        println!("Only executable-segment emission is allowed; the signed .mbn container remains incomplete.");
        return Ok(());
    }
    if args.len() == 1 && args[0] == "a640-container-coverage" {
        let plan = firmware_tools::devices::xiaomi_nabu::a640_container::partial_container()?;
        println!("A640 signed container: {} / {} bytes regenerated from Rust ({:.1}%)",
            plan.filled_bytes(), plan.total_bytes(),
            plan.filled_bytes() as f64 * 100.0 / plan.total_bytes() as f64);
        for gap in plan.holes() {
            println!("  UNIMPLEMENTED signed authentication 0x{:x}..0x{:x} ({} bytes)",
                gap.start, gap.end, gap.end-gap.start);
        }
        println!("Firmware emission prohibited while signing/authentication bytes are missing.");
        return Ok(());
    }
    if args.len() == 2 && args[0] == "emit-a640-executable" {
        let program = firmware_tools::devices::xiaomi_nabu::a640_plan::partial_executable()?
            .build_complete()?;
        fs::write(&args[1], &program)?;
        println!("Emitted {} bytes of QRISC/IR3 code; unsigned executable segment only, NOT an .mbn firmware file.", program.len());
        return Ok(());
    }
    if args.len() == 2 && args[0] == "emit-a640-fragments" {
        // Generate the matching fragments using typed ISA operations only.
        // No original firmware is needed to build these partial outputs.
        use firmware_tools::devices::xiaomi_nabu::{A640_FAMILY, A640_REVISION};
        let output = PathBuf::from(&args[1]);
        fs::create_dir_all(&output)?;
        let prefix = firmware_tools::a640_routines::security_setup_prefix(
            A640_FAMILY, A640_REVISION
        )?;
        let idle = firmware_tools::a640_routines::wait_for_idle()?;
        let launch = firmware_tools::devices::xiaomi_nabu::a640_launch::protected_shader_register_setup()?;
        let config = firmware_tools::devices::xiaomi_nabu::a640_launch::compute_register_configuration()?;
        let pipeline = firmware_tools::devices::xiaomi_nabu::a640_pipeline::protected_compute_and_resolve()?;
        fs::write(output.join("a640-qrisc-compute-dispatch.bin"), &pipeline)?;
        let finalize = firmware_tools::devices::xiaomi_nabu::a640_finalize::finish_protected_compute()?;
        fs::write(output.join("a640-qrisc-security-finalize.bin"), &finalize)?;
        let shader_regions = firmware_tools::devices::xiaomi_nabu::a640_shader::verified_candidates()?;
        fs::write(output.join("a640-qrisc-security-prefix.bin"), &prefix)?;
        fs::write(output.join("a640-qrisc-wait-idle.bin"), &idle)?;
        fs::write(output.join("a640-qrisc-launch-setup.bin"), &launch)?;
        fs::write(output.join("a640-qrisc-compute-config.bin"), &config)?;
        let mut total = prefix.len() + idle.len() + launch.len() + config.len() + pipeline.len() + finalize.len();
        for region in &shader_regions {
            fs::write(output.join(format!("a640-ir3-slot-{:02}.bin", region.first_slot)), &region.bytes)?;
            total += region.bytes.len();
        }
        println!("Emitted typed A640 fragments: QRISC={} bytes + IR3={} bytes = {} total; NOT flashable firmware.",
            prefix.len() + idle.len() + launch.len() + config.len() + pipeline.len() + finalize.len(), total - prefix.len() - idle.len() - launch.len() - config.len() - pipeline.len() - finalize.len(), total);
        return Ok(());
    }
    if args.len() == 2 && args[0] == "verify-a640-routines" {
        let input = fs::read(&args[1])?;
        let layout = firmware_tools::a640_microcode::inspect(&input)?;
        let (stream, _, _) = firmware_tools::a640_microcode::executable_segment(&input)?;
        let revision = (u32::from_le_bytes(stream[..4].try_into()?) & 0xfff) as u16;
        let entry = firmware_tools::a640_routines::preemption_guard(layout.firmware_id, revision)?;
        let dispatch = firmware_tools::a640_routines::security_setup_prefix(layout.firmware_id, revision)?;
        let idle = firmware_tools::a640_routines::wait_for_idle()?;
        if !dispatch.starts_with(&entry) { return Err("A640 dispatch does not include preemption guard".into()); }
        if !stream.starts_with(&dispatch) { return Err("A640 typed Rust security-dispatch prefix mismatches".into()); }
        if !stream.ends_with(&idle) { return Err("A640 typed Rust wait routine mismatches".into()); }
        let launch = firmware_tools::devices::xiaomi_nabu::a640_launch::protected_shader_register_setup()?;
        let launch_offset = firmware_tools::devices::xiaomi_nabu::a640_launch::START_WORD * 4;
        if stream.get(launch_offset..launch_offset+launch.len()) != Some(launch.as_slice()) {
            return Err("A640 typed Rust protected shader-launch setup mismatches".into());
        }
        let config = firmware_tools::devices::xiaomi_nabu::a640_launch::compute_register_configuration()?;
        let config_offset = firmware_tools::devices::xiaomi_nabu::a640_launch::CONFIG_START_WORD * 4;
        if stream.get(config_offset..config_offset+config.len()) != Some(config.as_slice()) {
            return Err("A640 typed Rust compute register configuration mismatches".into());
        }
        let pipeline = firmware_tools::devices::xiaomi_nabu::a640_pipeline::protected_compute_and_resolve()?;
        let pipeline_offset = firmware_tools::devices::xiaomi_nabu::a640_pipeline::START_WORD * 4;
        if stream.get(pipeline_offset..pipeline_offset+pipeline.len()) != Some(pipeline.as_slice()) {
            return Err("A640 typed Rust protected compute pipeline mismatches".into());
        }
        let finalize = firmware_tools::devices::xiaomi_nabu::a640_finalize::finish_protected_compute()?;
        let finalize_offset = firmware_tools::devices::xiaomi_nabu::a640_finalize::START_WORD * 4;
        if stream.get(finalize_offset..finalize_offset+finalize.len()) != Some(finalize.as_slice()) {
            return Err("A640 typed Rust final secure-state/event routines mismatch".into());
        }
        // Check separately authored control and initialization operations,
        // leaving uncovered shader slots explicitly unimplemented.
        let shader = layout.shader.ok_or("A640 embedded IR3 region not detected")?;
        let shader_regions = firmware_tools::devices::xiaomi_nabu::a640_shader::verified_candidates()?;
        let mut total_shader = 0;
        for region in &shader_regions {
            let start = shader.start_word * 4 + region.first_slot * 8;
            let stop = start.checked_add(region.bytes.len()).ok_or("IR3 offset overflow")?;
            if stop > shader.end_word * 4 || stream.get(start..stop) != Some(region.bytes.as_slice()) {
                return Err(format!("IR3 fragment mismatch at shader slot {} ({})", region.first_slot, region.purpose).into());
            }
            total_shader += region.bytes.len();
        }
        println!("Exact match: {}-byte typed QRISC security prefix (including {}-byte entry guard), {}-byte protected shader-launch setup, {}-byte compute register configuration, {}-byte compute pipeline, {}-byte finalize routines, {}-byte idle routine, and {} bytes across {} IR3 fragments; {} distinct executable-region bytes.",
            dispatch.len(), entry.len(), launch.len(), config.len(), pipeline.len(), finalize.len(), idle.len(), total_shader, shader_regions.len(),
            dispatch.len() + launch.len() + config.len() + pipeline.len() + finalize.len() + idle.len() + total_shader);
        let plan = firmware_tools::devices::xiaomi_nabu::a640_plan::partial_executable()?;
        plan.compare_known(stream)?;
        println!("Checked executable image: {} / {} generated bytes, {} unimplemented ranges; signed firmware emission is disabled.",
            plan.filled_bytes(), plan.total_bytes(), plan.holes().len());
        let signed_plan = firmware_tools::devices::xiaomi_nabu::a640_container::partial_container()?;
        signed_plan.compare_known(&input)?;
        println!("Checked A640 container: {} / {} generated bytes; unimplemented authentication {} bytes.",
            signed_plan.filled_bytes(), signed_plan.total_bytes(),
            signed_plan.holes().iter().map(|gap| gap.end-gap.start).sum::<usize>());
        println!("Executable instruction bytes match. Signed authentication material and hardware behavior are NOT reconstructed or validated. OEM analysis informed the typed implementation.");
        return Ok(());
    }
    if args.len() == 2 && args[0] == "verify-a640-isa-fields" {
        let firmware = fs::read(&args[1])?;
        let layout = firmware_tools::a640_microcode::inspect(&firmware)?;
        let (stream, _, _) = firmware_tools::a640_microcode::executable_segment(&firmware)?;
        let bounds = layout.shader.as_ref().map(|s| (s.start_word, s.end_word));
        let (encodable, unknown) = firmware_tools::qrisc_codec::check_isa_fields(stream, bounds)?;
        println!("QRISC field decoding/reencoding: {} control words roundtrip, {} unknown (excluding embedded IR3).", encodable, unknown);
        // Distinguish Mesa's normal RET from IRET and summarize an
        // otherwise undocumented bit outside the published RET operands.
        let mut plain_return = 0;
        let mut marked_return = 0;
        let mut interrupt_return = 0;
        for (i, bytes) in stream.chunks_exact(4).enumerate() {
            if bounds.is_some_and(|(start,end)| (start..end).contains(&i)) { continue; }
            let value = u32::from_le_bytes(bytes.try_into()?);
            match firmware_tools::qrisc_codec::QriscWord::decode(value) {
                Some(firmware_tools::qrisc_codec::QriscWord::Return { ignored_bits: 0 }) => plain_return += 1,
                Some(firmware_tools::qrisc_codec::QriscWord::Return { .. }) => marked_return += 1,
                Some(firmware_tools::qrisc_codec::QriscWord::InterruptReturn { .. }) => interrupt_return += 1,
                _ => {}
            }
        }
        let typed_id = firmware_tools::qrisc_codec::firmware_header(
            layout.firmware_id, (u32::from_le_bytes(stream[..4].try_into()?) & 0xfff) as u16
        )?.encode()?;
        println!("Normal RET: plain={plain_return}, uninterpreted-bits-set={marked_return}; IRET={interrupt_return}");
        println!("Header metadata via typed Rust fields matches source: {}", typed_id.to_le_bytes() == stream[..4]);
        println!("This is encoding-field verification against OEM inputs, NOT independently authored executable source.");
        return Ok(());
    }
    if args.len() == 2 && args[0] == "inspect-a640-flow" {
        let firmware = fs::read(&args[1])?;
        let layout = firmware_tools::a640_microcode::inspect(&firmware)?;
        let (stream, _, _) = firmware_tools::a640_microcode::executable_segment(&firmware)?;
        let shader = layout.shader.as_ref().map(|s| s.start_word..s.end_word);
        let flow = qrisc::flow::inspect(stream, 0, shader)?;
        println!("Potentially reachable QRISC words: {} / {}; branch targets: {}; call targets: {}; returns: {}",
            flow.reachable_words.len(), layout.total_words, flow.branch_targets.len(), flow.call_targets.len(), flow.returns);
        println!("This is conservative control-flow analysis; no GPU behavioral equivalence or safe replacement is demonstrated.");
        return Ok(());
    }
    if args.len() == 3 && args[0] == "extract-a640-isa" {
        let firmware = fs::read(&args[1])?;
        let layout = firmware_tools::a640_microcode::inspect(&firmware)?;
        let (segment, _, _) = firmware_tools::a640_microcode::executable_segment(&firmware)?;
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
        return Err("usage: firmware-tools a640-coverage OR a640-container-coverage \
            OR emit-a640-executable OUTPUT_BIN OR emit-a640-fragments OUTDIR \
            OR verify-a640-routines ORIGINAL_MBN \
            OR verify-mbnv6-hashes ORIGINAL_MBN \
            OR <rebuild-modemuw|verify-modemuw|inspect-elf32|inspect-image> PATH \
            OR make-ghidra-bootstrap ORIGINAL OUTPUT-ANALYSIS-ONLY \
            OR rebuild-elf-prefix NAME OUTPUT OR verify-elf-prefix NAME ORIGINAL \
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
