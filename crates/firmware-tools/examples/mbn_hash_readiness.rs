// SPDX-License-Identifier: GPL-3.0-only
//! Deterministic per-program-header SHA-384 source readiness for Nabu MBNv6.
//! The output contains only coverage, offsets and digests of source-generated
//! bytes; it never reads or exports OEM firmware/signatures/certificates.
use firmware_tools::{
    devices::xiaomi_nabu::elf_specs::KNOWN_ELFS,
    mbn_source_slices::{a640_source_plan, MbnSourceSlices, SegmentHashStatus},
};

fn hex(digest: &[u8]) -> String {
    digest.iter().map(|x| format!("{x:02x}")).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for spec in KNOWN_ELFS {
        let total = spec.headers.iter()
            .filter(|ph| ph.filesz > 0)
            .map(|ph| (ph.offset as usize) + (ph.filesz as usize))
            .max().ok_or("ELF has no file-backed segments")?;
        let source = if spec.name == "a640_zap.mbn" {
            a640_source_plan()?
        } else {
            MbnSourceSlices::new(spec, total)?
        };
        let report = source.partial_segment_hashes()?;
        print!("{{\"image\":\"{}\",\"file_bytes\":{},\"ready_entries\":{},\"ready_hashed_bytes\":{},\"pending_entries\":{},\"zero_placeholders\":{},\"all_input_hashes_ready\":{},\"segments\":[",
            spec.name, total, report.ready_entries, report.ready_hashed_bytes,
            report.pending_entries, report.zero_placeholders,
            report.all_input_hashes_ready);
        for (i, entry) in report.entries.iter().enumerate() {
            if i != 0 { print!(","); }
            print!("{{\"index\":{},\"kind\":{},\"file_range\":[{},{}]",
                entry.segment_index, entry.segment_kind,
                entry.file_range.start, entry.file_range.end);
            match &entry.status {
                SegmentHashStatus::ZeroPlaceholder => print!(",\"status\":\"placeholder\""),
                SegmentHashStatus::Ready { sha384 } => print!(
                    ",\"status\":\"ready\",\"sha384\":\"{}\"", hex(sha384)),
                SegmentHashStatus::Pending { first_missing, missing_bytes } => print!(
                    ",\"status\":\"pending\",\"first_missing\":[{},{}],\"missing_bytes\":{}",
                    first_missing.start, first_missing.end, missing_bytes),
            }
            print!("}}");
        }
        println!("]}}");
    }
    Ok(())
}
