// SPDX-License-Identifier: GPL-3.0-only
//! Exact byte accounting for Agent 16's partially reconstructed HALO image.
//!
//! Authored container metadata, reference-derived static directory fields,
//! explicit directory padding, reference-observed named initializers and
//! reference-observed zero bytes retain distinct provenance. Named initializer
//! bytes take precedence where they overlap an observed zero byte, but only
//! when the generated value is zero. Unknown bytes stay absent until supplied
//! as separately SHA-256-pinned external inputs. All PM bytes remain external.
//! This is static image construction, not a live registry or hardware result.

use crate::{
    image_plan::ImagePlan, wmfw_algorithms, wmfw_compact_y, wmfw_external_payload,
    wmfw_halo_control_defaults, wmfw_halo_static_data, wmfw_metadata,
    wmfw_plan, wmfw_sha256, wmfw_sparse_ym,
};
use std::{fs, ops::Range, path::Path};

pub const KNOWN_BYTES: usize = 10_299;
pub const EXTERNAL_BYTES: usize = 23_937;
pub const EXTERNAL_PM_BYTES: usize = 20_280;
pub const EXTERNAL_XY_BYTES: usize = 3_657;
pub const CONTROL_ZERO_OVERLAP_BYTES: usize = 297;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteOrigin {
    SourceAuthoredContainerMetadata,
    ReferenceDerivedStaticDirectory,
    ExplicitDirectoryPadding,
    ReferenceObservedControlInitialization,
    ReferenceObservedZero,
    ExternalOpaqueDsp,
}

impl ByteOrigin {
    pub fn key(self) -> &'static str {
        match self {
            Self::SourceAuthoredContainerMetadata => "source_authored_container_metadata",
            Self::ReferenceDerivedStaticDirectory => "reference_derived_static_directory",
            Self::ExplicitDirectoryPadding => "explicit_directory_padding",
            Self::ReferenceObservedControlInitialization => "reference_observed_control_initialization",
            Self::ReferenceObservedZero => "reference_observed_zero",
            Self::ExternalOpaqueDsp => "external_opaque_dsp",
        }
    }
}

const ORIGINS: [ByteOrigin; 6] = [
    ByteOrigin::SourceAuthoredContainerMetadata,
    ByteOrigin::ReferenceDerivedStaticDirectory,
    ByteOrigin::ExplicitDirectoryPadding,
    ByteOrigin::ReferenceObservedControlInitialization,
    ByteOrigin::ReferenceObservedZero,
    ByteOrigin::ExternalOpaqueDsp,
];
const EXPECTED_COUNTS: [usize; 6] = [3984, 69, 3, 792, 5451, EXTERNAL_BYTES];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct KnownByte {
    value: u8,
    origin: ByteOrigin,
}

fn put_known(cells: &mut [Option<KnownByte>], start: usize, data: &[u8],
    origin: ByteOrigin) -> Result<(), String> {
    if data.is_empty() || origin == ByteOrigin::ExternalOpaqueDsp {
        return Err("known HALO bytes require nonempty data and a nonexternal origin".into());
    }
    let end = start.checked_add(data.len()).ok_or("HALO known-byte extent overflow")?;
    let target = cells.get_mut(start..end).ok_or("HALO known-byte range outside image")?;
    if target.iter().any(Option::is_some) {
        return Err(format!("overlapping HALO known-byte sources at {start:#x}..{end:#x}"));
    }
    for (cell, value) in target.iter_mut().zip(data) {
        *cell = Some(KnownByte { value: *value, origin });
    }
    Ok(())
}

/// Zero ranges can overlap only explicitly named zero control initializers.
/// Validate the whole range before writing, preserving the typed origin.
fn add_observed_zeros(cells: &mut [Option<KnownByte>], range: Range<usize>)
    -> Result<usize, String> {
    if range.start >= range.end { return Err("empty or reversed HALO zero-byte range".into()); }
    let target = cells.get_mut(range).ok_or("HALO zero-byte range outside image")?;
    let mut overlap = 0;
    for cell in target.iter().flatten() {
        if cell.origin != ByteOrigin::ReferenceObservedControlInitialization || cell.value != 0 {
            return Err("HALO observed-zero range conflicts with a generated source".into());
        }
        overlap += 1;
    }
    for cell in target {
        if cell.is_none() {
            *cell = Some(KnownByte { value: 0, origin: ByteOrigin::ReferenceObservedZero });
        }
    }
    Ok(overlap)
}

fn origin_of(cell: &Option<KnownByte>) -> ByteOrigin {
    cell.map(|b| b.origin).unwrap_or(ByteOrigin::ExternalOpaqueDsp)
}

fn counts(cells: &[Option<KnownByte>]) -> [usize; 6] {
    ORIGINS.map(|origin| cells.iter().filter(|cell| origin_of(cell) == origin).count())
}

fn source_cells() -> Result<Vec<Option<KnownByte>>, String> {
    let regions = wmfw_plan::regions()?;
    let mut cells = vec![None; wmfw_plan::FILE_SIZE];
    let metadata = ByteOrigin::SourceAuthoredContainerMetadata;
    put_known(&mut cells, 0, &wmfw_metadata::build_wmfw_header(), metadata)?;
    for descriptor in wmfw_metadata::WMFW_REGIONS {
        put_known(&mut cells, descriptor.file_offset, &descriptor.encode(), metadata)?;
    }
    let info = regions.first().ok_or("missing WMFW information region")?;
    if info.kind != 0xff || info.payload_length != wmfw_plan::FIRMWARE_INFO.len() {
        return Err("WMFW information region disagrees with authored source".into());
    }
    put_known(&mut cells, info.payload_offset, wmfw_plan::FIRMWARE_INFO.as_bytes(), metadata)?;
    let algorithm_regions: Vec<_> = regions.iter().filter(|r| r.kind == 0xf2).collect();
    let algorithms = [wmfw_algorithms::framework(), wmfw_algorithms::cspl(),
        wmfw_algorithms::event_logger()];
    if algorithm_regions.len() != algorithms.len() {
        return Err("unexpected authored WMFW algorithm-region count".into());
    }
    for (region, algorithm) in algorithm_regions.iter().zip(&algorithms) {
        let bytes = wmfw_algorithms::encode_algorithm(algorithm)?;
        if bytes.len() != region.payload_length {
            return Err("authored WMFW algorithm metadata length mismatch".into());
        }
        put_known(&mut cells, region.payload_offset, &bytes, metadata)?;
    }

    let directory = wmfw_halo_static_data::build_static_directory()?;
    let start = wmfw_halo_static_data::DIRECTORY_FILE_OFFSET;
    // Padding word 23 occupies lanes 8..11 of the final register group.
    // The terminator's remaining high byte follows it at packed offset 71.
    if directory.len() != 72 || directory[68..71].iter().any(|b| *b != 0) {
        return Err("static HALO directory padding differs from its schema".into());
    }
    put_known(&mut cells, start, &directory[..68], ByteOrigin::ReferenceDerivedStaticDirectory)?;
    put_known(&mut cells, start + 68, &directory[68..71], ByteOrigin::ExplicitDirectoryPadding)?;
    put_known(&mut cells, start + 71, &directory[71..], ByteOrigin::ReferenceDerivedStaticDirectory)?;

    let initializers = wmfw_halo_control_defaults::generated_bytes()?;
    if initializers.len() != 792 { return Err("unexpected HALO control-initializer byte count".into()); }
    for (offset, value) in initializers {
        if !regions.iter().any(|r| matches!(r.kind, 0x11 | 0x12)
            && (r.payload_offset..r.end()).contains(&offset)) {
            return Err("named HALO initializer is outside XM/YM file payloads".into());
        }
        put_known(&mut cells, offset, &[value], ByteOrigin::ReferenceObservedControlInitialization)?;
    }
    let mut overlap = 0usize;
    for (start, end) in wmfw_compact_y::ZERO_BYTE_RANGES {
        if start >= end || end > wmfw_sparse_ym::YM_PAYLOAD_BYTES {
            return Err("observed YM zero-byte interval outside payload".into());
        }
        let file_start = wmfw_sparse_ym::YM_FILE_START.checked_add(start)
            .ok_or("YM zero-byte start overflow")?;
        let file_end = wmfw_sparse_ym::YM_FILE_START.checked_add(end)
            .ok_or("YM zero-byte end overflow")?;
        overlap += add_observed_zeros(&mut cells, file_start..file_end)?;
    }
    if overlap != CONTROL_ZERO_OVERLAP_BYTES || counts(&cells) != EXPECTED_COUNTS {
        return Err("HALO known/zero/external provenance accounting mismatch".into());
    }
    if cells[0x278c..0x76c4].iter().any(Option::is_some) {
        return Err("HALO program bytes must remain entirely external".into());
    }
    Ok(cells)
}

/// Build known bytes only. Unknown memory is absent, including every PM byte.
/// Finalization fails until every external gap has been explicitly supplied.
pub fn partial_image() -> Result<ImagePlan, String> {
    let cells = source_cells()?;
    let mut plan = ImagePlan::new(cells.len())?;
    let mut start = 0;
    while start < cells.len() {
        let Some(first) = cells[start] else { start += 1; continue; };
        let mut end = start + 1;
        while end < cells.len() && cells[end].is_some_and(|b| b.origin == first.origin) { end += 1; }
        let bytes: Vec<_> = cells[start..end].iter().map(|b| b.expect("known span").value).collect();
        plan.place(first.origin.key(), start, &bytes)?;
        start = end;
    }
    if plan.filled_bytes() != KNOWN_BYTES { return Err("HALO partial image coverage mismatch".into()); }
    Ok(plan)
}

/// One exact-version external gap. Only bounds and digest are stored in source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExternalPart {
    pub start: usize,
    pub end: usize,
    pub sha256: &'static str,
}

impl ExternalPart {
    pub fn filename(self) -> String { format!("halo-external-{:06x}-{:06x}.bin", self.start, self.end) }
}

// Computed from the SHA-pinned private reference only after the complete known
// source map compared equal. No external payload bytes are embedded here.
pub const EXTERNAL_PARTS: [ExternalPart; 17] = [
    ExternalPart { start: 0xb8, end: 0x1d8, sha256: "7dbdf2e83158ab2601837f2867eee660607cd7fb6db5a90fda88fc1eec84a325" },
    ExternalPart { start: 0x20c, end: 0x20e, sha256: "96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7" },
    ExternalPart { start: 0x210, end: 0x214, sha256: "df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119" },
    ExternalPart { start: 0x2bc, end: 0x2bd, sha256: "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d" },
    ExternalPart { start: 0x2c0, end: 0x2c8, sha256: "af5570f5a1810b7af78caf4bc70a660f0df51e42baf91d4de5b2328de0e83dfc" },
    ExternalPart { start: 0x2d0, end: 0x360, sha256: "284327a5d25edf337f7bc817e23df0b0ee9f390c19a3c2ff3bfc587a958f3e90" },
    ExternalPart { start: 0x440, end: 0x443, sha256: "709e80c88487a2411e1ee4dfb9f22a861492d20c4765150c0c794abd70f8147c" },
    ExternalPart { start: 0x444, end: 0x4b0, sha256: "fe9e01f87ecf4dbfcc5de31e36f924ef10616610e283b6cd3fdfb0ba042a0956" },
    ExternalPart { start: 0x4d8, end: 0x4da, sha256: "ab28109c76051d674842e56aeb856eb2b1931187a6f4464a927ed0067c936c48" },
    ExternalPart { start: 0x4dc, end: 0x900, sha256: "26bd6350c451338dfd56af497bdfda0663fd209be11ab981d86523d0175ba124" },
    ExternalPart { start: 0x908, end: 0xe0c, sha256: "c7f0a17c379cd0e7bd7bd54685cfef97db7857ff43616b5b111670520894caa5" },
    ExternalPart { start: 0xe14, end: 0xf88, sha256: "e599a34d425aa7136cabd30cc375212584c2f3c3541a8b6939b810bd10fe09d4" },
    ExternalPart { start: 0xf90, end: 0xfb8, sha256: "043d2dec2d99ac3a021b351d86342be9efafe5547ad0f7ed16a3d808d5784293" },
    ExternalPart { start: 0xfba, end: 0xfbc, sha256: "eec4121f2a07b61aba16414812aa9afc39ab0a136360a5ace2240dc19b0464eb" },
    ExternalPart { start: 0x10ec, end: 0x122d, sha256: "26a9d0ff548e3bd433d6916a19033e26fe0b0aa08b17ca60e4fc8b1b7a22f921" },
    ExternalPart { start: 0x2772, end: 0x2784, sha256: "9a2123ce83ba0651e0672c7ce6fe5d992ced8ef9d71e101a9b580faf354c20d7" },
    ExternalPart { start: 0x278c, end: 0x76c4, sha256: "0edacef35965ea2bd8c709f8cad73fd08714bb47922065d25e3b9ba1bf34c517" },
];

fn validate_manifest() -> Result<(), String> {
    let holes = partial_image()?.holes();
    if holes.len() != EXTERNAL_PARTS.len() {
        return Err("HALO external manifest does not match all missing image spans".into());
    }
    let mut external = 0usize;
    for (hole, part) in holes.iter().zip(EXTERNAL_PARTS) {
        if (part.start..part.end) != *hole || part.start >= part.end
            || part.sha256.len() != 64
            || !part.sha256.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
            return Err("invalid HALO external gap descriptor".into());
        }
        external = external.checked_add(part.end - part.start).ok_or("HALO external byte count overflow")?;
    }
    if external != EXTERNAL_BYTES || !holes.contains(&(0x278c..0x76c4)) {
        return Err("HALO external byte/PM accounting mismatch".into());
    }
    Ok(())
}

fn validate_part(part: ExternalPart, bytes: &[u8]) -> Result<(), String> {
    let length = part.end.checked_sub(part.start).filter(|n| *n > 0)
        .ok_or("invalid HALO external part bounds")?;
    if bytes.len() != length { return Err(format!("{} length mismatch", part.filename())); }
    if wmfw_sha256::hex_digest(bytes) != part.sha256 {
        return Err(format!("{} SHA-256 mismatch", part.filename()));
    }
    Ok(())
}

/// Verify both exact firmware revision and every currently generated byte.
pub fn verify_reference(reference: &[u8]) -> Result<(), String> {
    if wmfw_sha256::hex_digest(reference) != wmfw_external_payload::REFERENCE_SHA256 {
        return Err("HALO memory-plan reference SHA-256 mismatch".into());
    }
    wmfw_plan::verify_container(reference)?;
    partial_image()?.compare_known(reference)?;
    validate_manifest()
}

/// Atomically extract only missing byte spans into a new private directory.
/// All input checks finish before the existing private writer touches output.
pub fn extract_inputs(reference: &[u8], output: &Path) -> Result<(), String> {
    verify_reference(reference)?;
    let mut entries = Vec::with_capacity(EXTERNAL_PARTS.len());
    for part in EXTERNAL_PARTS {
        let bytes = reference.get(part.start..part.end).ok_or("HALO external part outside reference")?;
        validate_part(part, bytes)?;
        entries.push((part.filename(), bytes.to_vec()));
    }
    wmfw_external_payload::write_private_region_directory(output, &entries)
}

fn assemble_with_external<F>(mut read: F) -> Result<Vec<u8>, String>
where F: FnMut(ExternalPart) -> Result<Vec<u8>, String> {
    validate_manifest()?;
    let mut plan = partial_image()?;
    for part in EXTERNAL_PARTS {
        let bytes = read(part)?;
        validate_part(part, &bytes)?;
        plan.place(&format!("EXTERNAL opaque HALO {}", part.filename()), part.start, &bytes)?;
    }
    let image = plan.build_complete()?;
    if wmfw_sha256::hex_digest(&image) != wmfw_external_payload::REFERENCE_SHA256 {
        return Err("assembled HALO memory-plan image SHA-256 mismatch".into());
    }
    Ok(image)
}

/// Repack a complete image with every missing original span supplied explicitly.
/// The resulting byte identity does not establish source or hardware behavior.
pub fn assemble_directory(input: &Path) -> Result<Vec<u8>, String> {
    assemble_with_external(|part| fs::read(input.join(part.filename()))
        .map_err(|e| format!("reading {}: {e}", part.filename())))
}

/// A complete, nonoverlapping byte ledger, with maximal consecutive spans.
pub fn ledger_json() -> Result<String, String> {
    validate_manifest()?;
    let cells = source_cells()?;
    let mut json = format!("{{\n  \"format\": \"Cirrus HALO WMFW v3 Agent 16 memory plan\",\n  \"reference_sha256\": \"{}\",\n  \"total_bytes\": {},\n",
        wmfw_external_payload::REFERENCE_SHA256, cells.len());
    for (origin, count) in ORIGINS.into_iter().zip(counts(&cells)) {
        json.push_str(&format!("  \"{}_bytes\": {},\n", origin.key(), count));
    }
    let first_unknown = cells.iter().position(Option::is_none).ok_or("missing HALO external provenance")?;
    json.push_str(&format!("  \"known_bytes\": {KNOWN_BYTES},\n  \"zero_initialization_overlap_bytes\": {CONTROL_ZERO_OVERLAP_BYTES},\n  \"external_pm_bytes\": {EXTERNAL_PM_BYTES},\n  \"external_xy_bytes\": {EXTERNAL_XY_BYTES},\n  \"external_part_count\": {},\n  \"first_unreconstructed_file_offset\": {first_unknown},\n  \"independently_reconstructed_halo_instruction_bytes\": 0,\n  \"live_runtime_dump_supplied\": false,\n  \"hardware_register_addresses_asserted\": false,\n  \"spans\": [\n", EXTERNAL_PARTS.len()));
    let mut start = 0;
    while start < cells.len() {
        let origin = origin_of(&cells[start]);
        let mut end = start + 1;
        while end < cells.len() && origin_of(&cells[end]) == origin { end += 1; }
        json.push_str(&format!("    {{\"start\": {start}, \"end\": {end}, \"origin\": \"{}\"}}{}\n",
            origin.key(), if end == cells.len() { "" } else { "," }));
        start = end;
    }
    json.push_str("  ]\n}\n");
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_byte_accounting_preserves_padding_initializers_and_all_pm_holes() {
        let cells = source_cells().unwrap();
        assert_eq!(counts(&cells), [3984, 69, 3, 792, 5451, 23937]);
        assert_eq!(counts(&cells).iter().sum::<usize>(), wmfw_plan::FILE_SIZE);
        assert!(cells[0xac..0xaf].iter().all(|c| origin_of(c) == ByteOrigin::ExplicitDirectoryPadding));
        assert_eq!(origin_of(&cells[0xaf]), ByteOrigin::ReferenceDerivedStaticDirectory);
        assert!(cells[0x278c..0x76c4].iter().all(Option::is_none));
        let partial = partial_image().unwrap();
        assert_eq!(partial.filled_bytes(), KNOWN_BYTES);
        assert!(partial.build_complete().unwrap_err().contains("incomplete"));
        assert_eq!(partial.holes().first().unwrap().start, 0xb8);
        assert_eq!(partial.holes().iter().map(|r| r.end-r.start).sum::<usize>(), EXTERNAL_BYTES);
        validate_manifest().unwrap();
        let json = ledger_json().unwrap();
        assert!(json.contains("\"reference_observed_zero_bytes\": 5451"));
        assert!(json.contains("\"known_bytes\": 10299"));
        assert!(json.contains("\"hardware_register_addresses_asserted\": false"));
    }

    #[test]
    fn canonical_coverage_matches_generated_disjoint_memory_plan() {
        let name = "cs35l41-dsp1-spk-prot.wmfw";
        let identity = crate::firmware_manifest_identity::canonical_image(name).unwrap();
        let manifest = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../reference/xiaomi-nabu/coverage.toml"));
        let entry = manifest.split("[[images]]").find(|entry| entry.lines()
            .any(|line| line.trim() == format!("name = \"{name}\""))).unwrap();
        let field = |key: &str| entry.lines().filter_map(|line| line.split_once('='))
            .find(|(name, _)| name.trim() == key).unwrap().1.trim();
        let credit: usize = field("credit_bytes").parse().unwrap();
        let external: usize = field("external_dsp_payload_bytes").parse().unwrap();
        let cells = source_cells().unwrap();
        let disjoint = counts(&cells);
        let partial = partial_image().unwrap();
        assert_eq!(disjoint, [3984, 69, 3, 792, 5451, 23937]);
        assert_eq!(credit, disjoint[..5].iter().sum::<usize>());
        assert_eq!(credit, partial.filled_bytes());
        assert_eq!(credit, KNOWN_BYTES);
        assert_eq!(credit, 10_299);
        assert_eq!(external, disjoint[5]);
        assert_eq!(external, partial.holes().iter().map(|r| r.end-r.start).sum::<usize>());
        assert_eq!(external, EXTERNAL_BYTES);
        assert_eq!(external, EXTERNAL_PM_BYTES + EXTERNAL_XY_BYTES);
        assert_eq!(external, EXTERNAL_PARTS.iter().map(|p| p.end-p.start).sum::<usize>());
        let zero_bytes: usize = wmfw_compact_y::ZERO_BYTE_RANGES.iter()
            .map(|(start, end)| end-start).sum();
        assert_eq!(zero_bytes - disjoint[4], CONTROL_ZERO_OVERLAP_BYTES);
        assert_eq!(CONTROL_ZERO_OVERLAP_BYTES, 297);
        assert_eq!(credit + external, identity.size);
        assert_eq!(partial.total_bytes(), identity.size);
        assert_eq!(identity.sha256, wmfw_external_payload::REFERENCE_SHA256);
        assert_eq!(field("credit_kind"), "\"structural_and_reference_observed\"");
        assert!(!entry.lines().any(|line| line.trim() == "complete_source = true"));
        assert!(partial.build_complete().is_err());
    }

    #[test]
    fn typed_zero_overlap_has_single_provenance_and_nonzero_conflicts_are_atomic() {
        let mut cells = vec![None; 6];
        put_known(&mut cells, 2, &[0, 0], ByteOrigin::ReferenceObservedControlInitialization).unwrap();
        assert_eq!(add_observed_zeros(&mut cells, 1..5).unwrap(), 2);
        assert_eq!(origin_of(&cells[1]), ByteOrigin::ReferenceObservedZero);
        assert_eq!(origin_of(&cells[2]), ByteOrigin::ReferenceObservedControlInitialization);
        assert_eq!(origin_of(&cells[3]), ByteOrigin::ReferenceObservedControlInitialization);
        assert_eq!(origin_of(&cells[4]), ByteOrigin::ReferenceObservedZero);
        assert!(cells[0].is_none() && cells[5].is_none());
        let mut cells = vec![None; 6];
        put_known(&mut cells, 3, &[1], ByteOrigin::ReferenceObservedControlInitialization).unwrap();
        let before = cells.clone();
        assert!(add_observed_zeros(&mut cells, 1..5).unwrap_err().contains("conflicts"));
        assert_eq!(cells, before);
        let mut cells = vec![None; 6];
        put_known(&mut cells, 3, &[0], ByteOrigin::SourceAuthoredContainerMetadata).unwrap();
        assert!(add_observed_zeros(&mut cells, 1..5).is_err());
    }

    #[test]
    fn source_placement_rejects_overlap_bounds_and_implicit_external_fills() {
        let mut cells = vec![None; 4];
        let origin = ByteOrigin::SourceAuthoredContainerMetadata;
        put_known(&mut cells, 1, &[2], origin).unwrap();
        assert!(put_known(&mut cells, 1, &[2], origin).unwrap_err().contains("overlap"));
        assert!(put_known(&mut cells, usize::MAX, &[0], origin).unwrap_err().contains("overflow"));
        assert!(put_known(&mut cells, 4, &[0], origin).unwrap_err().contains("outside"));
        assert!(put_known(&mut cells, 0, &[0], ByteOrigin::ExternalOpaqueDsp).is_err());
        assert!(add_observed_zeros(&mut cells, 0..5).is_err());
        assert!(add_observed_zeros(&mut cells, 2..2).is_err());
    }

    #[test]
    fn absent_wrong_length_and_corrupted_external_inputs_never_complete_an_image() {
        assert!(assemble_with_external(|_| Err("missing external input".into()))
            .unwrap_err().contains("missing"));
        assert!(assemble_with_external(|_| Ok(vec![])).unwrap_err().contains("length"));
        assert!(assemble_with_external(|part| Ok(vec![0; part.end-part.start]))
            .unwrap_err().contains("SHA-256"));
        assert!(verify_reference(&vec![0; wmfw_plan::FILE_SIZE]).unwrap_err().contains("SHA-256"));
    }
}
