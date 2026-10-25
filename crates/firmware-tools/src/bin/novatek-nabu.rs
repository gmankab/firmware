// SPDX-License-Identifier: GPL-3.0-only
//! Standalone agent-09 inspector. This bin references an agent-owned module
//! directly, so it can be cherry-picked onto checkpoint 50 without touching
//! central device `mod.rs` or unrelated firmware subcommands.
#[path = "../devices/xiaomi_nabu/novatek_layout.rs"]
mod novatek_layout;
#[path = "../devices/xiaomi_nabu/novatek_reassemble.rs"]
mod novatek_reassemble;
#[path = "../devices/xiaomi_nabu/novatek_vector.rs"]
mod novatek_vector;
#[path = "../devices/xiaomi_nabu/novatek_crc.rs"]
mod novatek_crc;
#[path = "../devices/xiaomi_nabu/novatek_nds32.rs"]
mod novatek_nds32;

use novatek_layout::{NabuNovatekLayout, HEADER_BYTES, IMAGE_LENGTH, RESERVED_TAIL_START, END_MARKER_OFFSET, DLM_FILE_END, POST_DLM_PADDING_END};
use std::{env, fs, process};

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("assemble-crc-validated") && args.len() == 6 {
        let meta=fs::read(&args[1]).map_err(|e|e.to_string())?;
        let layout=NabuNovatekLayout::parse(&meta)?;
        let vector_words=fs::read(&args[2]).map_err(|e|e.to_string())?;
        let vectors=novatek_vector::CandidateVectorTable::from_bytes(&vector_words,&layout)?;
        let ilm=fs::read(&args[3]).map_err(|e|e.to_string())?;
        let dlm=fs::read(&args[4]).map_err(|e|e.to_string())?;
        let built=novatek_reassemble::assemble_crc_validated(&layout,
            novatek_reassemble::AuthoredPayloads{
                vector_table:Some(&vectors),ilm:Some(&ilm),dlm:Some(&dlm)
            })?;
        if built.flashable() || built.verified_mcu_program() {
            return Err("unexpectedly marked unauthenticated payload as flashable".into());
        }
        fs::write(&args[5],&built.bytes).map_err(|e|e.to_string())?;
        println!("Wrote {} structural bytes; recomputed {} CRC-32/MPEG-2 fields. WARNING: remaining ILM/DLM program provenance NOT verified; do NOT flash.",built.bytes.len(),built.crc_fields_generated());
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("assemble-structural-unverified") && args.len() == 6 {
        let meta=fs::read(&args[1]).map_err(|e|e.to_string())?;
        let layout=NabuNovatekLayout::parse(&meta)?;
        let vector_words=fs::read(&args[2]).map_err(|e|e.to_string())?;
        let vectors=novatek_vector::CandidateVectorTable::from_bytes(&vector_words,&layout)?;
        let ilm=fs::read(&args[3]).map_err(|e|e.to_string())?;
        let dlm=fs::read(&args[4]).map_err(|e|e.to_string())?;
        let built=novatek_reassemble::assemble_structural_unverified(&layout,
            novatek_reassemble::AuthoredPayloads{
                vector_table:Some(&vectors),ilm:Some(&ilm),dlm:Some(&dlm)
            })?;
        fs::write(&args[5],&built.bytes).map_err(|e|e.to_string())?;
        println!("Wrote {} structural bytes; hardware CRC verified: {}. WARNING: remaining ILM/DLM provenance and behavior NOT verified; do NOT flash.", built.bytes.len(),built.is_hardware_crc_verified());
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("emit-nds32-isr") && args.len()==2 {
        let bytes=novatek_nds32::initial_interrupt_handler()?;
        fs::write(&args[1],&bytes).map_err(|e|e.to_string())?;
        println!("Independently generated {} NDS32 executable bytes at file offsets 0x{:x}–0x{:x}",
            bytes.len(),novatek_nds32::FIRST_HANDLER_OFFSET,novatek_nds32::FIRST_HANDLER_END);
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("emit-nds32-extra-paths") && args.len()==2 {
        let bytes=novatek_nds32::additional_interrupt_paths()?;
        fs::write(&args[1],&bytes).map_err(|e|e.to_string())?;
        println!("Independently generated {} further NDS32 executable bytes at 0x{:x}..0x{:x}",
            bytes.len(),novatek_nds32::FIRST_HANDLER_END,novatek_nds32::EXTRA_INTERRUPTS_END);
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("verify-nds32-extra-paths") && args.len()==2 {
        let input=fs::read(&args[1]).map_err(|e|e.to_string())?;
        let layout=NabuNovatekLayout::parse(&input)?;
        if layout.ilm.end()? < novatek_nds32::EXTRA_INTERRUPTS_END {
            return Err("NDS32 extra paths outside ILM partition".into());
        }
        novatek_nds32::verify_additional_interrupt_paths(&input)?;
        println!("NDS32: {} more source-derived executable bytes exactly verified; remaining ILM/DLM unknown",
            novatek_nds32::EXTRA_INTERRUPTS_END - novatek_nds32::FIRST_HANDLER_END);
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("verify-nds32-isr") && args.len()==2 {
        let input=fs::read(&args[1]).map_err(|e|e.to_string())?;
        let layout=NabuNovatekLayout::parse(&input)?;
        if layout.ilm.end()? < novatek_nds32::FIRST_HANDLER_END {
            return Err("NDS32 ISR outside ILM partition".into());
        }
        novatek_nds32::verify_first_handler(&input)?;
        println!("NDS32 instruction stream: {}-byte interrupt-handler path reconstruction matches the reference exactly; remaining firmware code is incomplete",
            novatek_nds32::FIRST_HANDLER_END-novatek_nds32::FIRST_HANDLER_OFFSET);
        return Ok(());
    }
    if args.len() < 2 || args.len() > 3 {
        return Err("usage: novatek-nabu <emit-nds32-isr out.bin|verify-nds32-isr reference.bin|emit-nds32-extra-paths out.bin|verify-nds32-extra-paths reference.bin> | <inspect|verify-dlm-mirror|verify-crcs|emit-recomputed-crc-header|emit-header|emit-padding|emit-vector|emit-vector-padding|emit-post-dlm-padding|emit-tail-padding|emit-marker|verify-header|verify-envelope|verify-vectors> <private-reference.bin> [output] | assemble-structural-unverified <private-meta-reference> <externally-authored-vector-words> <independently-authored-ilm-after-0x4aa> <independently-authored-dlm> <output> | assemble-crc-validated <private-meta-reference> <authored-vector-words> <authored-ilm> <authored-dlm> <output>".into());
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
            layout.write_post_dlm_padding(&reference)?;
            println!("post-DLM generated zero padding 0x{DLM_FILE_END:x}–0x{POST_DLM_PADDING_END:x} = {} bytes", POST_DLM_PADDING_END-DLM_FILE_END);
            println!("independently constructible zero spacer 0x{gap_start:x}–0x{gap_end:x} = {} bytes",gap_end-gap_start);
            let independent = layout.coverage()?.iter().filter(|r| r.source_independent()).map(|r| r.length()).sum::<usize>();
            let unresolved: usize = layout.coverage()?.iter().filter(|r|r.is_unimplemented())
                .map(|r|r.length()).sum();
            println!("independently reconstructible bytes, including typed NDS32 ISR: {independent}");
            println!("unreconstructed firmware payload bytes: {unresolved}");
            println!("verified reserved tail: 0x{RESERVED_TAIL_START:x}–0x{END_MARKER_OFFSET:x}");
            for region in novatek_reassemble::projected_regions(&layout)? { let range=region.range; println!("  {:>6} bytes 0x{:05x}..0x{:05x} {:?} {}",
                range.length(),range.start,range.end,range.kind,range.name); }
            println!("CRC-32/MPEG-2 and NDS32 ISA are established. The ISR entry path is independently built; remaining MCU program is missing; not flashable");
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
        "emit-vector" if args.len() == 3 => {
            let vectors=novatek_vector::CandidateVectorTable::parse(&reference,&layout)?;
            fs::write(&args[2],vectors.write()).map_err(|e|e.to_string())?;
            println!("wrote {} typed candidate-vector words from externally supplied address metadata; NOT decompiled CPU code",novatek_vector::VECTOR_COUNT);
        }
        "emit-vector-padding" if args.len() == 3 => {
            let bytes=novatek_vector::CandidateVectorTable::reserved_padding(&reference)?;
            fs::write(&args[2],bytes).map_err(|e|e.to_string())?;
            println!("wrote {} independently generated vector-gap bytes",bytes.len());
        }
        "verify-vectors" if args.len() == 2 => {
            let vectors=novatek_vector::CandidateVectorTable::parse(&reference,&layout)?;
            if vectors.write() != reference[0x200..0x400] {return Err("vector words differ".into());}
            novatek_vector::CandidateVectorTable::reserved_padding(&reference)?;
            println!("typed candidate vector metadata and reserved zeros verified; NDS32 ISR reconstruction is separate");
        }
        "emit-post-dlm-padding" if args.len() == 3 => {
            let generated=layout.write_post_dlm_padding(&reference)?;
            fs::write(&args[2],&generated).map_err(|e|e.to_string())?;
            println!("wrote {} independently generated post-DLM zeros",generated.len());
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
        "emit-recomputed-crc-header" if args.len() == 3 => {
            let mut reconstructed = reference.clone();
            let count=novatek_crc::recompute_all_crcs(&mut reconstructed)?;
            if reconstructed[..HEADER_BYTES]!=reference[..HEADER_BYTES] {
                return Err("CRC-derived header differs from the supplied firmware".into());
            }
            fs::write(&args[2],&reconstructed[..HEADER_BYTES]).map_err(|e|e.to_string())?;
            println!("wrote {HEADER_BYTES} structured metadata bytes with {} independently recomputed CRC-32/MPEG-2 fields; only the ISR prefix is independently recompiled",count.stored_fields());
        }
        "verify-crcs" if args.len() == 2 => {
            let count=novatek_crc::verify_all_crcs(&layout,&reference)?;
            println!("validated {} CRC-32/MPEG-2 checksum fields ({} logical checks, including duplicated core fields)",
                count.stored_fields(),count.total_checks());
        }
        "verify-dlm-mirror" if args.len() == 2 => {
            layout.verify_dlm_mirror(&reference)?;
            println!("verified {} DLM-prefix mirror bytes; the DLM executable itself remains unauthored",novatek_layout::DLM_MIRROR_BYTES);
        }
        "verify-envelope" if args.len() == 2 => {
            layout.verify_envelope(&reference)?;
            let independent=layout.coverage()?.iter().filter(|r|r.source_independent())
                .map(|r|r.length()).sum::<usize>();
            let projection=novatek_reassemble::projected_regions(&layout)?;
            let absent=projection.iter().filter(|r|matches!(r.range.kind,
                novatek_layout::RegionKind::StructuredVectorsWithOpaqueTargets |
                novatek_layout::RegionKind::UnimplementedIlm |
                novatek_layout::RegionKind::UnimplementedDlm)).count();
            novatek_vector::CandidateVectorTable::parse(&reference,&layout)?;
            novatek_vector::CandidateVectorTable::reserved_padding(&reference)?;
            println!("Novatek metadata/envelope and NDS32 ISR verified; {independent} independently constructible bytes; {absent} vector/remaining ILM/DLM holes remain unknown; DLM mirror generated from authored DLM");
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
