// SPDX-License-Identifier: GPL-3.0-only
//! Strict assembly of independently authored ELF/MBNv6 source fragments.
//!
//! File provenance and ELF permissions are deliberately separate: data and
//! alignment can occur in an executable segment, and writable executable
//! segments must not be counted twice. A source claim is supplied by the
//! caller and requires independent evidence; this API cannot prove that a
//! caller did not copy an OEM payload. There is no reference-payload importer.
//! Unknown bytes are never implicitly filled, and BSS never becomes file data.

use std::ops::Range;

use crate::{
    elf32_prefix::{write_elf32_first_page, Elf32Spec, ELF_PAGE_LEN},
    image_plan::{CoverageReport, ImagePlan},
    mbn_source_builder::MbnSourceBuilder,
    parse_elf32,
    qcom_mbn_profiles::unsigned_header_and_policy,
    qcom_mbn_validation::sha384_table_from_generated_plan,
};

const PT_LOAD: u32 = 1;
const PF_X: u32 = 1;
const PF_W: u32 = 2;
const PF_R: u32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Code,
    Data,
    Metadata,
}

/// A source fragment is anchored to an exact program header and a file offset,
/// not an inferred virtual-to-file translation. Inputs are borrowed only for
/// the call. Adjacent fragments may arrive in any order; overlap is an error,
/// including an identical duplicate. A fragment may not cross its segment.
#[derive(Debug, Clone, Copy)]
pub struct SourceSlice<'a> {
    pub segment_index: usize,
    pub file_offset: usize,
    pub name: &'a str,
    pub kind: SourceKind,
    pub bytes: &'a [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceKind {
    Code,
    Data,
    Metadata,
    LayoutFill,
    ExternalAuthentication,
}

impl ProvenanceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Code => "source_code",
            Self::Data => "source_data",
            Self::Metadata => "source_metadata",
            Self::LayoutFill => "explicit_layout_fill",
            Self::ExternalAuthentication => "external_authentication",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRegion {
    pub name: String,
    pub file_range: Range<usize>,
    pub segment_index: Option<usize>,
    pub kind: ProvenanceKind,
}

/// These categories are disjoint. In particular, `code` counts the caller's
/// explicitly classified instructions, not every byte of a PF_X segment.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ProvenanceBytes {
    pub code: usize,
    pub data: usize,
    pub metadata: usize,
    pub layout_fill: usize,
    pub external_authentication: usize,
    pub missing: usize,
}

impl ProvenanceBytes {
    pub fn generated(&self) -> usize {
        self.code + self.data + self.metadata + self.layout_fill
    }

    pub fn total(&self) -> usize {
        self.generated() + self.external_authentication + self.missing
    }

    fn add(&mut self, kind: ProvenanceKind, amount: usize) {
        match kind {
            ProvenanceKind::Code => self.code += amount,
            ProvenanceKind::Data => self.data += amount,
            ProvenanceKind::Metadata => self.metadata += amount,
            ProvenanceKind::LayoutFill => self.layout_fill += amount,
            ProvenanceKind::ExternalAuthentication => self.external_authentication += amount,
        }
    }
}

/// Disjoint file-byte union of PT_LOAD permissions. Overlapping aliases use
/// the union of permissions at each file byte. Non-load NOTE/DYNAMIC records
/// never add duplicate load bytes. These are declared permissions, not source
/// recovery totals or a claim that all executable-segment bytes are code.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LoadPermissionBytes {
    pub executable_not_writable: usize,
    pub writable_not_executable: usize,
    pub executable_and_writable: usize,
    pub neither_executable_nor_writable: usize,
}

impl LoadPermissionBytes {
    pub fn total(&self) -> usize {
        self.executable_not_writable
            + self.writable_not_executable
            + self.executable_and_writable
            + self.neither_executable_nor_writable
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegmentCoverage {
    pub index: usize,
    pub kind: u32,
    pub flags: u32,
    /// Empty for BSS, even when its p_offset is beyond physical EOF.
    pub file_range: Range<usize>,
    /// File-backed addresses only. The memory-only tail is counted below.
    pub virtual_file_range: Range<u64>,
    pub physical_file_range: Range<u64>,
    pub executable: bool,
    pub writable: bool,
    pub readable: bool,
    pub memory_only_bytes: u64,
    pub bytes: ProvenanceBytes,
    pub holes: Vec<Range<usize>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCoverage {
    pub total_file_bytes: usize,
    pub bytes: ProvenanceBytes,
    pub holes: Vec<Range<usize>>,
    pub regions: Vec<SourceRegion>,
    pub segments: Vec<SegmentCoverage>,
    pub load_permissions: LoadPermissionBytes,
    /// Geometric file gaps outside all nonempty program-header ranges. These
    /// may contain explicitly generated layout fill or still-unknown bytes;
    /// their existence does not authorize filling them automatically.
    pub unassigned_file_ranges: Vec<Range<usize>>,
    /// Sum of PT_LOAD p_memsz - p_filesz; address aliases are not deduplicated.
    /// This is a memory mapping total and never part of total_file_bytes.
    pub memory_only_bytes: u64,
    pub hash_file_range: Range<usize>,
    pub authentication_file_range: Range<usize>,
    pub hashes_generated: bool,
}

/// State of one ELF program-header SHA-384 hash entry. The MBN hash table
/// has one fixed-size entry per program header; its own hash/auth segment and
/// zero-file-size entries are zero placeholders by convention.
///
/// A ready entry is hashed only from independently generated plan bytes. A
/// pending entry reports the first still-unknown span and total missing file
/// bytes; it never reads reference firmware to fill the gap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentHashStatus {
    ZeroPlaceholder,
    Ready { sha384: [u8; 48] },
    Pending { first_missing: Range<usize>, missing_bytes: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegmentHashEntry {
    pub segment_index: usize,
    pub segment_kind: u32,
    pub file_range: Range<usize>,
    pub status: SegmentHashStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialSegmentHashReport {
    pub entries: Vec<SegmentHashEntry>,
    pub ready_entries: usize,
    pub ready_hashed_bytes: usize,
    pub pending_entries: usize,
    pub zero_placeholders: usize,
    /// Hashes are complete only when every non-placeholder input is ready.
    /// This property is distinct from the source-image auth/metadata holes.
    pub all_input_hashes_ready: bool,
}

/// A stricter fragment-oriented companion to `MbnSourceBuilder`.
///
/// Metadata uses the six known Nabu policies from the existing builder.
/// Complete file emission is forbidden until all declared source, explicit
/// padding, derived hashes, and (if requested) original external auth exist.
/// There is no mutable accessor to bypass fragment provenance or geometry.
#[derive(Debug, Clone)]
pub struct MbnSourceSlices<'a> {
    spec: &'a Elf32Spec,
    plan: ImagePlan,
    regions: Vec<SourceRegion>,
    hash_segment_index: usize,
    hash_file_range: Range<usize>,
    authentication_file_range: Range<usize>,
    hashes_generated: bool,
}

impl<'a> MbnSourceSlices<'a> {
    pub fn new(spec: &'a Elf32Spec, total_file_bytes: usize) -> Result<Self, String> {
        let plan = MbnSourceBuilder::new(spec, total_file_bytes)?.into_plan();
        let (header, _) = unsigned_header_and_policy(spec)?;
        let (hash_segment_index, hash) = spec.headers.iter().enumerate()
            .find(|(_, h)| h.flags & 0x0700_0000 == 0x0200_0000)
            .ok_or("missing MBNv6 hash segment")?;
        let hash_start = hash.offset as usize;
        let layout = header.layout(hash.filesz as usize)?;
        if header.qti_signature_size != 0 || header.qti_certificate_size != 0 {
            return Err("dual-signer MBNv6 is not supported".into());
        }
        let auth = (hash_start + layout.oem_signature.start)
            ..(hash_start + layout.oem_certificate.end);
        let hashes = (hash_start + layout.hashes.start)..(hash_start + layout.hashes.end);
        let hash_segment_range = checked_file_range(hash.offset, hash.filesz)?;
        for (index, ph) in spec.headers.iter().enumerate() {
            if ph.kind == PT_LOAD {
                if ph.filesz > ph.memsz {
                    return Err(format!("ELF PT_LOAD {index} p_filesz exceeds p_memsz"));
                }
                for (address, label) in [(ph.vaddr, "virtual"), (ph.paddr, "physical")] {
                    if u64::from(address) + u64::from(ph.memsz) > (1u64 << 32) {
                        return Err(format!("ELF PT_LOAD {index} {label} memory range exceeds ELF32"));
                    }
                }
            }
            if index != hash_segment_index && ph.filesz != 0
                && intersects(&checked_file_range(ph.offset, ph.filesz)?, &hash_segment_range)
            {
                return Err(format!("ELF segment {index} overlaps signed MBNv6 hash/authentication"));
            }
        }
        let header_end = 52usize.checked_add(spec.headers.len().checked_mul(32)
            .ok_or("program-header size overflow")?).ok_or("ELF header size overflow")?;
        // The first-page builder has an explicit zero-alignment policy. It
        // cannot silently stand in for a payload that a changed descriptor
        // places inside those zeros, even if that payload is itself all zero.
        let first_page_fill = header_end..ELF_PAGE_LEN;
        for (index, ph) in spec.headers.iter().enumerate() {
            if ph.filesz != 0 && intersects(&checked_file_range(ph.offset, ph.filesz)?, &first_page_fill) {
                return Err(format!("ELF segment {index} overlaps generated first-page alignment"));
            }
        }
        let mut regions = vec![SourceRegion {
            name: "typed ELF32 header and program table".into(),
            file_range: 0..header_end,
            segment_index: None,
            kind: ProvenanceKind::Metadata,
        }];
        if header_end < ELF_PAGE_LEN {
            regions.push(SourceRegion {
                name: "ELF first-page zero alignment".into(),
                file_range: header_end..ELF_PAGE_LEN,
                segment_index: None,
                kind: ProvenanceKind::LayoutFill,
            });
        }
        for (name, range) in [
            ("typed MBNv6 unsigned header", layout.header),
            ("typed OEM image policy", layout.oem_metadata),
        ] {
            regions.push(SourceRegion {
                name: name.into(),
                file_range: (hash_start + range.start)..(hash_start + range.end),
                segment_index: Some(hash_segment_index),
                kind: ProvenanceKind::Metadata,
            });
        }
        Ok(Self { spec, plan, regions, hash_segment_index, hash_file_range: hashes,
            authentication_file_range: auth, hashes_generated: false })
    }

    pub fn plan(&self) -> &ImagePlan { &self.plan }

    pub fn coverage(&self) -> CoverageReport { self.plan.coverage() }

    /// Transactional batch insertion: invalid bounds, permissions, duplicate
    /// bytes, or any later fragment error leave the original plan unchanged.
    pub fn add_source_slices(&mut self, slices: &[SourceSlice<'_>]) -> Result<(), String> {
        if slices.is_empty() { return Err("source-slice batch cannot be empty".into()); }
        let mut candidate = self.plan.clone();
        let mut additions = Vec::with_capacity(slices.len());
        for slice in slices {
            let kind = match slice.kind {
                SourceKind::Code => ProvenanceKind::Code,
                SourceKind::Data => ProvenanceKind::Data,
                SourceKind::Metadata => ProvenanceKind::Metadata,
            };
            let range = self.check_source_span(slice.segment_index, slice.file_offset,
                slice.bytes.len(), kind)?;
            candidate.place(slice.name, slice.file_offset, slice.bytes)?;
            additions.push(SourceRegion { name: slice.name.into(), file_range: range,
                segment_index: Some(slice.segment_index), kind });
        }
        self.plan = candidate;
        self.regions.extend(additions);
        Ok(())
    }

    pub fn add_source_slice(&mut self, slice: SourceSlice<'_>) -> Result<(), String> {
        self.add_source_slices(&[slice])
    }

    /// Explicit format-defined zero fill outside *all* file-backed headers.
    /// The name should identify the independent layout rule. A hole or zero
    /// bytes seen in a reference alone are not evidence that code is padding.
    pub fn add_verified_zero_padding(&mut self, name: &str, span: Range<usize>)
        -> Result<(), String>
    {
        self.check_padding_span(&span)?;
        for (index, ph) in self.spec.headers.iter().enumerate() {
            if ph.filesz != 0 && intersects(&span, &checked_file_range(ph.offset, ph.filesz)?) {
                return Err(format!("layout fill overlaps file-backed ELF segment {index}"));
            }
        }
        self.place_padding(name, span, None)
    }

    /// Deterministically generate the next power-of-two alignment boundary.
    /// No padding is emitted if the rule is invalid, already aligned, crosses
    /// a declared segment, or would overwrite another source contribution.
    pub fn add_alignment_padding(&mut self, name: &str, start: usize, alignment: usize)
        -> Result<Range<usize>, String>
    {
        if !alignment.is_power_of_two() {
            return Err("alignment must be a nonzero power of two".into());
        }
        let end = start.checked_add(alignment - 1).ok_or("alignment offset overflow")?
            & !(alignment - 1);
        let span = start..end;
        self.add_verified_zero_padding(name, span.clone())?;
        Ok(span)
    }

    /// Explicit, source-described zero padding *inside* one segment. It is
    /// always classified as layout fill, even when the PT_LOAD is executable.
    /// It does not authorize treating a missing instruction region as padding.
    pub fn add_segment_zero_padding(&mut self, segment_index: usize, name: &str,
        span: Range<usize>) -> Result<(), String>
    {
        self.check_padding_span(&span)?;
        self.check_source_span(segment_index, span.start, span.len(), ProvenanceKind::LayoutFill)?;
        self.place_padding(name, span, Some(segment_index))
    }

    /// Report which ELF SHA-384 entries can be computed **now** from source,
    /// without requiring the entire MBN image to be complete. This is useful
    /// during multi-agent integrations: a complete code/data segment can be
    /// independently hashed even while other segments contain missing bytes.
    ///
    /// Hash readiness is distinct from `generate_hashes`, which correctly
    /// still refuses to publish the final SHA-384 table while any non-auth
    /// file byte remains unimplemented. A reference-free digest does not
    /// establish byte equality with a vendor firmware revision; compare each
    /// ready digest against a pinned reference independently.
    pub fn partial_segment_hashes(&self) -> Result<PartialSegmentHashReport, String> {
        let mut entries = Vec::with_capacity(self.spec.headers.len());
        let holes = self.plan.holes();
        let mut ready_entries = 0;
        let mut ready_hashed_bytes = 0usize;
        let mut pending_entries = 0;
        let mut zero_placeholders = 0;
        for (segment_index, ph) in self.spec.headers.iter().enumerate() {
            let file_range = checked_file_range(ph.offset, ph.filesz)?;
            let status = if segment_index == self.hash_segment_index || ph.filesz == 0 {
                zero_placeholders += 1;
                SegmentHashStatus::ZeroPlaceholder
            } else {
                let missing = intersections(&file_range, &holes);
                if let Some(first_missing) = missing.first() {
                    pending_entries += 1;
                    SegmentHashStatus::Pending {
                        first_missing: first_missing.clone(),
                        missing_bytes: missing.iter().map(|range| range.len()).sum(),
                    }
                } else {
                    // Never hash a borrowed authentication byte. The checked
                    // constructor forbids such overlaps today, but the slice
                    // accessor is a second guard against future regressions.
                    let payload = self.plan.generated_slice(file_range.clone())?;
                    ready_entries += 1;
                    ready_hashed_bytes += payload.len();
                    SegmentHashStatus::Ready { sha384: crate::sha384::digest(&payload) }
                }
            };
            entries.push(SegmentHashEntry {
                segment_index, segment_kind: ph.kind, file_range, status,
            });
        }
        Ok(PartialSegmentHashReport { entries, ready_entries,
            ready_hashed_bytes, pending_entries, zero_placeholders,
            all_input_hashes_ready: pending_entries == 0 })
    }

    /// Compute the hash table only after every hashed input and every other
    /// non-authentication file byte has a source or explicit padding rule.
    /// A failure is transactional. Original OEM digest tables are never read.
    pub fn generate_hashes(&mut self) -> Result<(), String> {
        if self.hashes_generated { return Err("SHA-384 table already generated".into()); }
        let elf = parse_elf32(&write_elf32_first_page(self.spec)?)?;
        let hashes = sha384_table_from_generated_plan(&elf, self.hash_segment_index, &self.plan)?;
        if hashes.len() != self.hash_file_range.len() {
            return Err("generated SHA-384 size contradicts signed header".into());
        }
        let excluded = [self.hash_file_range.clone(), self.authentication_file_range.clone()];
        for hole in self.plan.holes() {
            if let Some(unknown) = subtract_ranges(hole, &excluded).first() {
                return Err(format!("cannot finalize hashes: unimplemented file/layout byte at 0x{:x}",
                    unknown.start));
            }
        }
        self.plan.place("derived source-only SHA-384 table", self.hash_file_range.start, &hashes)?;
        self.regions.push(SourceRegion { name: "derived source-only SHA-384 table".into(),
            file_range: self.hash_file_range.clone(), segment_index: Some(self.hash_segment_index),
            kind: ProvenanceKind::Metadata });
        self.hashes_generated = true;
        Ok(())
    }

    /// Admit only the exact original-authentication field and only after the
    /// source-only image is complete apart from that field. This is provenance
    /// accounting, not cryptographic signature verification or signing.
    pub fn add_external_oem_authentication(&mut self, name: &str, bytes: &[u8])
        -> Result<(), String>
    {
        if !self.hashes_generated {
            return Err("generate source-only hashes before adding external authentication".into());
        }
        if bytes.len() != self.authentication_file_range.len() {
            return Err(format!("expected {} external authentication bytes, got {}",
                self.authentication_file_range.len(), bytes.len()));
        }
        if self.plan.holes() != vec![self.authentication_file_range.clone()] {
            return Err("source image must be complete except for its original authentication field".into());
        }
        self.plan.place_external_authentication(name, self.authentication_file_range.start, bytes)?;
        self.regions.push(SourceRegion { name: name.into(),
            file_range: self.authentication_file_range.clone(), segment_index: Some(self.hash_segment_index),
            kind: ProvenanceKind::ExternalAuthentication });
        Ok(())
    }

    pub fn build_complete(&self) -> Result<Vec<u8>, String> { self.plan.build_complete() }

    pub fn build_independently_generated(&self) -> Result<Vec<u8>, String> {
        self.plan.build_independently_generated()
    }

    /// Canonical report order depends on file offsets, not insertion order.
    /// Per-segment totals can overlap because NOTE/DYNAMIC aliases are real;
    /// only the top-level totals and PT_LOAD permission union are additive.
    pub fn report(&self) -> SourceCoverage {
        let holes = self.plan.holes();
        let mut regions = self.regions.clone();
        regions.sort_by(|a, b| (a.file_range.start, a.file_range.end, &a.name)
            .cmp(&(b.file_range.start, b.file_range.end, &b.name)));
        let bytes = count_provenance(0..self.plan.total_bytes(), &regions, &holes);
        let mut segments = Vec::with_capacity(self.spec.headers.len());
        let mut memory_only_bytes = 0u64;
        let mut assigned = Vec::new();
        for (index, ph) in self.spec.headers.iter().enumerate() {
            let file_range = (ph.offset as usize)..(ph.offset as usize + ph.filesz as usize);
            let memory_only = if ph.kind == PT_LOAD { u64::from(ph.memsz - ph.filesz) } else { 0 };
            memory_only_bytes += memory_only;
            let segment_holes = intersections(&file_range, &holes);
            segments.push(SegmentCoverage {
                index, kind: ph.kind, flags: ph.flags, file_range: file_range.clone(),
                virtual_file_range: u64::from(ph.vaddr)..(u64::from(ph.vaddr) + u64::from(ph.filesz)),
                physical_file_range: u64::from(ph.paddr)..(u64::from(ph.paddr) + u64::from(ph.filesz)),
                executable: ph.kind == PT_LOAD && ph.flags & PF_X != 0,
                writable: ph.kind == PT_LOAD && ph.flags & PF_W != 0,
                readable: ph.kind == PT_LOAD && ph.flags & PF_R != 0,
                memory_only_bytes: memory_only,
                bytes: count_provenance(file_range.clone(), &regions, &holes),
                holes: segment_holes,
            });
            if ph.filesz != 0 { assigned.push(file_range); }
        }
        SourceCoverage { total_file_bytes: self.plan.total_bytes(), bytes, holes, regions, segments,
            load_permissions: permission_union(self.spec),
            unassigned_file_ranges: subtract_ranges(0..self.plan.total_bytes(), &assigned),
            memory_only_bytes, hash_file_range: self.hash_file_range.clone(),
            authentication_file_range: self.authentication_file_range.clone(),
            hashes_generated: self.hashes_generated }
    }

    fn check_source_span(&self, index: usize, offset: usize, len: usize,
        kind: ProvenanceKind) -> Result<Range<usize>, String>
    {
        let ph = self.spec.headers.get(index).ok_or("ELF program-header index out of bounds")?;
        if index == self.hash_segment_index {
            return Err("source slice cannot import signed hash/authentication bytes".into());
        }
        if ph.filesz == 0 { return Err("BSS has no file bytes; source slice forbidden".into()); }
        if len == 0 { return Err("source slice cannot be empty".into()); }
        if kind == ProvenanceKind::Code && (ph.kind != PT_LOAD || ph.flags & PF_X == 0) {
            return Err(format!("source code requires executable PT_LOAD, not ELF segment {index}"));
        }
        let end = offset.checked_add(len).ok_or("source slice offset overflow")?;
        let segment = checked_file_range(ph.offset, ph.filesz)?;
        if offset < segment.start || end > segment.end {
            return Err(format!("source slice 0x{offset:x}..0x{end:x} is outside ELF segment {index} 0x{:x}..0x{:x}",
                segment.start, segment.end));
        }
        Ok(offset..end)
    }

    fn check_padding_span(&self, span: &Range<usize>) -> Result<(), String> {
        if span.start >= span.end { return Err("padding must be a nonempty forward range".into()); }
        if span.end > self.plan.total_bytes() { return Err("padding extends outside file".into()); }
        Ok(())
    }

    fn place_padding(&mut self, name: &str, span: Range<usize>, segment_index: Option<usize>)
        -> Result<(), String>
    {
        self.plan.place(name, span.start, &vec![0; span.len()])?;
        self.regions.push(SourceRegion { name: name.into(), file_range: span, segment_index,
            kind: ProvenanceKind::LayoutFill });
        Ok(())
    }
}

/// Rebuild the A640 source-only container through the strict fragment API.
///
/// This preserves the existing typed QRISC/IR3 emitters and their provisional
/// barrier semantics. The four-byte QRISC firmware header is metadata, and
/// the terminal IR3 NOP after `End` is explicit layout fill. Neither is added
/// to the instruction-byte count. This helper reads no reference firmware,
/// imports no authentication, and intentionally cannot emit a complete MBN.
pub fn a640_source_plan() -> Result<MbnSourceSlices<'static>, String> {
    use crate::devices::xiaomi_nabu::{a640_plan, elf_specs::KNOWN_ELFS};
    const FILE_START: usize = 0x3000;
    const HEADER_LEN: usize = 4;
    const TERMINAL_PADDING_START: usize = a640_plan::SHADER_OFFSET + 63 * 8;
    const TERMINAL_PADDING_END: usize = TERMINAL_PADDING_START + 8;
    let spec = KNOWN_ELFS.iter().find(|s| s.name == "a640_zap.mbn")
        .ok_or("A640 ELF specification unavailable")?;
    let code = spec.headers.get(2).ok_or("A640 executable segment missing")?;
    if spec.headers.len() != 3 || code.offset as usize != FILE_START
        || code.kind != PT_LOAD || code.flags & PF_X == 0
        || code.filesz as usize != a640_plan::EXECUTABLE_SIZE
    {
        return Err("A640 executable geometry changed".into());
    }
    let executable = a640_plan::partial_executable()?;
    let source = executable.build_independently_generated()?;
    if source.get(TERMINAL_PADDING_START..TERMINAL_PADDING_END) != Some(&[0u8; 8][..]) {
        return Err("typed A640 terminal IR3 NOP is not the expected zero padding".into());
    }
    let mut plan = MbnSourceSlices::new(spec, FILE_START + source.len())?;
    let classified_ranges = [
        (0..HEADER_LEN, SourceKind::Metadata),
        (HEADER_LEN..TERMINAL_PADDING_START, SourceKind::Code),
        (TERMINAL_PADDING_END..source.len(), SourceKind::Code),
    ];
    let mut pieces = Vec::new();
    for region in executable.regions() {
        for (classification, kind) in &classified_ranges {
            let first = region.offset.max(classification.start);
            let end = (region.offset + region.len).min(classification.end);
            if first < end {
                pieces.push(SourceSlice { segment_index: 2, file_offset: FILE_START + first,
                    name: &region.name, kind: *kind, bytes: &source[first..end] });
            }
        }
    }
    plan.add_source_slices(&pieces)?;
    plan.add_segment_zero_padding(2, "typed terminal IR3 NOP following End",
        (FILE_START + TERMINAL_PADDING_START)..(FILE_START + TERMINAL_PADDING_END))?;
    let padding = plan.add_alignment_padding("A640 4 KiB executable file alignment",
        plan.authentication_file_range.end, 4096)?;
    if padding.end != FILE_START { return Err("A640 executable alignment changed".into()); }
    plan.generate_hashes()?;
    Ok(plan)
}

fn checked_file_range(offset: u32, len: u32) -> Result<Range<usize>, String> {
    let start = usize::try_from(offset).map_err(|_| "ELF offset exceeds host usize")?;
    let size = usize::try_from(len).map_err(|_| "ELF size exceeds host usize")?;
    Ok(start..start.checked_add(size).ok_or("ELF file range overflow")?)
}

fn intersects(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start < b.end && b.start < a.end
}

fn intersections(span: &Range<usize>, ranges: &[Range<usize>]) -> Vec<Range<usize>> {
    ranges.iter().filter_map(|r| {
        let start = span.start.max(r.start);
        let end = span.end.min(r.end);
        (start < end).then_some(start..end)
    }).collect()
}

fn subtract_ranges(span: Range<usize>, excluded: &[Range<usize>]) -> Vec<Range<usize>> {
    let mut remaining = Vec::new();
    let mut cursor = span.start;
    let mut excludes = intersections(&span, excluded);
    excludes.sort_by_key(|r| (r.start, r.end));
    for range in excludes {
        if cursor < range.start { remaining.push(cursor..range.start); }
        cursor = cursor.max(range.end);
    }
    if cursor < span.end { remaining.push(cursor..span.end); }
    remaining
}

fn count_provenance(span: Range<usize>, regions: &[SourceRegion], holes: &[Range<usize>])
    -> ProvenanceBytes
{
    let mut count = ProvenanceBytes::default();
    for region in regions {
        let start = span.start.max(region.file_range.start);
        let end = span.end.min(region.file_range.end);
        if start < end { count.add(region.kind, end - start); }
    }
    count.missing = intersections(&span, holes).iter().map(|r| r.len()).sum();
    count
}

fn permission_union(spec: &Elf32Spec) -> LoadPermissionBytes {
    let loads = spec.headers.iter().filter(|h| h.kind == PT_LOAD && h.filesz != 0)
        .map(|h| ((h.offset as usize)..(h.offset as usize + h.filesz as usize), h.flags))
        .collect::<Vec<_>>();
    let mut endpoints = loads.iter().flat_map(|(r, _)| [r.start, r.end]).collect::<Vec<_>>();
    endpoints.sort_unstable();
    endpoints.dedup();
    let mut result = LoadPermissionBytes::default();
    for pair in endpoints.windows(2) {
        let mut flags = 0;
        let mut present = false;
        for (range, permissions) in &loads {
            if range.start <= pair[0] && pair[1] <= range.end {
                present = true;
                flags |= permissions;
            }
        }
        if !present { continue; }
        let size = pair[1] - pair[0];
        match (flags & PF_X != 0, flags & PF_W != 0) {
            (true, false) => result.executable_not_writable += size,
            (false, true) => result.writable_not_executable += size,
            (true, true) => result.executable_and_writable += size,
            (false, false) => result.neither_executable_nor_writable += size,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::xiaomi_nabu::{a640_plan, elf_specs::KNOWN_ELFS};
    use crate::elf32_prefix::Elf32Program;

    fn a640() -> &'static Elf32Spec { &KNOWN_ELFS[0] }
    fn plan() -> MbnSourceSlices<'static> { MbnSourceSlices::new(a640(), 0x37b0).unwrap() }
    fn executable() -> Vec<u8> {
        a640_plan::partial_executable().unwrap().build_independently_generated().unwrap()
    }
    fn slice<'a>(name: &'a str, offset: usize, bytes: &'a [u8]) -> SourceSlice<'a> {
        SourceSlice { segment_index: 2, file_offset: offset, name, kind: SourceKind::Code, bytes }
    }
    fn complete_source() -> MbnSourceSlices<'static> {
        let mut p = plan();
        p.add_source_slice(slice("independent A640 program", 0x3000, &executable())).unwrap();
        p.add_alignment_padding("4 KiB executable alignment", 0x2a38, 4096).unwrap();
        p.generate_hashes().unwrap();
        p
    }

    #[test]
    fn partial_segment_hashes_respect_source_only_segments_and_placeholders() {
        let mut p = plan();
        let report = p.partial_segment_hashes().unwrap();
        assert_eq!(report.entries.len(), 3);
        assert_eq!(report.ready_entries, 1); // exact source-only ELF first page
        assert_eq!(report.ready_hashed_bytes, 148);
        assert_eq!(report.zero_placeholders, 1); // MBN hashes/auth is excluded
        assert_eq!(report.pending_entries, 1);
        assert!(!report.all_input_hashes_ready);
        assert!(matches!(report.entries[0].status, SegmentHashStatus::Ready { .. }));
        assert_eq!(report.entries[1].status, SegmentHashStatus::ZeroPlaceholder);
        assert_eq!(report.entries[2].status, SegmentHashStatus::Pending {
            first_missing: 0x3000..0x37b0, missing_bytes: 0x7b0,
        });
        let code = executable();
        p.add_source_slice(slice("first fragment", 0x3000, &code[..0x100])).unwrap();
        p.add_source_slice(slice("last fragment", 0x37ac, &code[0x7ac..])).unwrap();
        let partial = p.partial_segment_hashes().unwrap();
        assert_eq!(partial.entries[2].status, SegmentHashStatus::Pending {
            first_missing: 0x3100..0x37ac, missing_bytes: 0x6ac,
        });
        // Hash publication remains fail-closed even when one input is ready.
        assert!(p.generate_hashes().is_err());
        p.add_source_slice(slice("middle fragment", 0x3100, &code[0x100..0x7ac])).unwrap();
        let done = p.partial_segment_hashes().unwrap();
        assert!(done.all_input_hashes_ready);
        assert_eq!(done.ready_entries, 2);
        assert_eq!(done.ready_hashed_bytes, 148 + code.len());
        assert_eq!(done.entries[2].status, SegmentHashStatus::Ready {
            sha384: crate::sha384::digest(&code),
        });
        // Image still has unsigned layout gaps until padded explicitly.
        assert!(p.generate_hashes().is_err());
    }

    #[test]
    fn partial_segment_hashes_report_all_nabu_elf_program_headers() {
        for spec in KNOWN_ELFS {
            let file_size = spec.headers.iter().filter(|p| p.filesz != 0)
                .map(|p| p.offset as usize + p.filesz as usize).max().unwrap();
            let p = MbnSourceSlices::new(spec, file_size).unwrap();
            let report = p.partial_segment_hashes().unwrap();
            assert_eq!(report.entries.len(), spec.headers.len(), "{}", spec.name);
            assert_eq!(report.ready_entries, 1, "{}", spec.name);
            assert_eq!(report.ready_hashed_bytes, spec.headers[0].filesz as usize, "{}", spec.name);
            assert_eq!(report.entries[0].file_range.start, 0, "{}", spec.name);
            assert!(report.pending_entries > 0, "{}", spec.name);
            assert_eq!(report.entries.iter().filter(|entry|
                matches!(entry.status, SegmentHashStatus::ZeroPlaceholder)).count(),
                report.zero_placeholders);
            assert!(report.entries.iter().all(|entry|
                entry.file_range.is_empty() || entry.file_range.end <= file_size
                || matches!(entry.status, SegmentHashStatus::ZeroPlaceholder)));
        }
    }

    #[test]
    fn unordered_adjacent_fragments_have_deterministic_bytes_and_provenance() {
        let code = executable();
        let mut a = plan();
        let parts = [slice("A640 beginning", 0x3000, &code[..173]),
            slice("A640 middle", 0x30ad, &code[173..1200]),
            slice("A640 end", 0x34b0, &code[1200..])];
        a.add_source_slices(&parts).unwrap();
        let mut b = plan();
        b.add_source_slices(&[parts[2], parts[0], parts[1]]).unwrap();
        assert_eq!(a.plan().generated_slice(0x3000..0x37b0).unwrap(), code);
        assert_eq!(a.report(), b.report());
        for p in [&mut a, &mut b] {
            p.add_alignment_padding("4 KiB executable alignment", 0x2a38, 4096).unwrap();
            p.generate_hashes().unwrap();
        }
        assert_eq!(a.report(), b.report());
        assert_eq!(a.coverage().generated_bytes, 7856);
        assert_eq!(a.coverage().holes, vec![0x1138..0x2a38]);
    }

    #[test]
    fn failed_batch_does_not_keep_earlier_valid_fragments() {
        let mut p = plan();
        let before = p.report();
        assert!(p.add_source_slices(&[slice("good", 0x3000, &[1, 2]),
            slice("overlap", 0x3001, &[2, 3])]).unwrap_err().contains("overlap"));
        assert_eq!(p.report(), before);
        assert!(p.add_source_slices(&[slice("good", 0x3000, &[1, 2]),
            slice("outside", 0x37af, &[1, 2])]).unwrap_err().contains("outside"));
        assert_eq!(p.report(), before);
        assert!(p.add_source_slices(&[]).is_err());
    }

    #[test]
    fn duplicates_bounds_empty_names_and_overflow_are_rejected() {
        let mut p = plan();
        for s in [slice("", 0x3000, &[1]), slice("empty", 0x3000, &[]),
            slice("overflow", usize::MAX, &[1]), slice("below", 0x2fff, &[1]),
            slice("crosses", 0x37af, &[1, 2])] {
            assert!(p.add_source_slice(s).is_err());
        }
        p.add_source_slice(slice("one", 0x3000, &[1])).unwrap();
        let before = p.report();
        assert!(p.add_source_slice(slice("identical duplicate", 0x3000, &[1])).is_err());
        assert_eq!(p.report(), before);
        assert!(p.add_source_slice(SourceSlice { segment_index: 1,
            file_offset: 0x10a8, name: "imported hashes", kind: SourceKind::Data,
            bytes: &[0; 48] }).is_err());
    }

    #[test]
    fn missing_source_and_layout_block_hashes_without_changing_plan() {
        let mut p = plan();
        let before = p.report();
        assert!(p.generate_hashes().unwrap_err().contains("0x3000"));
        assert_eq!(p.report(), before);
        p.add_source_slice(slice("A640 program", 0x3000, &executable())).unwrap();
        let before = p.report();
        assert!(p.generate_hashes().unwrap_err().contains("0x2a38"));
        assert_eq!(p.report(), before);
        p.add_alignment_padding("4 KiB executable alignment", 0x2a38, 4096).unwrap();
        p.generate_hashes().unwrap();
        let before = p.report();
        assert!(p.generate_hashes().unwrap_err().contains("already"));
        assert_eq!(p.report(), before);
    }

    #[test]
    fn code_data_metadata_and_padding_are_disjoint_even_inside_executable_load() {
        let mut p = plan();
        p.add_source_slice(slice("instruction", 0x3000, &[1, 2, 3, 4])).unwrap();
        p.add_source_slice(SourceSlice { segment_index: 2, file_offset: 0x3004,
            name: "literal", kind: SourceKind::Data, bytes: &[5, 6, 7, 8] }).unwrap();
        p.add_segment_zero_padding(2, "declared table alignment", 0x3008..0x3010).unwrap();
        let report = p.report();
        assert_eq!(report.bytes.code, 4);
        assert_eq!(report.bytes.data, 4);
        assert_eq!(report.bytes.metadata, 148 + 168);
        assert_eq!(report.bytes.layout_fill, 4096 - 148 + 8);
        assert_eq!(report.bytes.total(), 0x37b0);
        assert_eq!(report.load_permissions.executable_and_writable, 1968);
        assert_eq!(report.load_permissions.executable_not_writable, 0);
        assert_eq!(report.segments[2].bytes.code, 4);
        assert_eq!(report.segments[2].bytes.data, 4);
        assert_eq!(report.segments[2].bytes.layout_fill, 8);
        assert_eq!(report.segments[2].holes, vec![0x3010..0x37b0]);
    }

    #[test]
    fn external_auth_requires_complete_source_and_never_becomes_generated() {
        let mut p = plan();
        assert!(p.add_external_oem_authentication("synthetic auth", &[0x55; 6400])
            .unwrap_err().contains("hashes"));
        p = complete_source();
        let before = p.report();
        assert!(p.add_external_oem_authentication("wrong size", &[0; 6399]).is_err());
        assert_eq!(p.report(), before);
        p.add_external_oem_authentication("synthetic external auth fixture", &[0x55; 6400]).unwrap();
        let report = p.report();
        assert_eq!(report.bytes.generated(), 7856);
        assert_eq!(report.bytes.external_authentication, 6400);
        assert_eq!(report.bytes.missing, 0);
        assert_eq!(report.bytes.total(), 14256);
        let image = p.build_complete().unwrap();
        crate::qcom_mbn_validation::verify_elf32_mbnv6_hashes(&image).unwrap();
        assert!(p.build_independently_generated().unwrap_err().contains("external"));
        assert!(p.add_external_oem_authentication("duplicate", &[0x55; 6400]).is_err());
    }

    #[test]
    fn alignment_and_padding_cannot_hide_segment_holes() {
        let mut p = plan();
        for align in [0, 3, 4095] {
            assert!(p.add_alignment_padding("bad", 0x2a38, align).is_err());
        }
        assert!(p.add_alignment_padding("empty", 0x3000, 4096).is_err());
        assert!(p.add_alignment_padding("overflow", usize::MAX, 4096).is_err());
        assert!(p.add_verified_zero_padding("outside", 0x37b0..0x37b1).is_err());
        assert!(p.add_verified_zero_padding("reversed", 8..7).is_err());
        assert!(p.add_verified_zero_padding("code masquerading as fill", 0x3000..0x3010).is_err());
        assert!(p.add_segment_zero_padding(1, "auth masquerading as fill", 0x1138..0x1140).is_err());
        assert_eq!(p.coverage().generated_bytes, 4264);
    }

    #[test]
    fn known_profiles_have_exact_union_accounting_and_memory_only_bss() {
        for spec in KNOWN_ELFS {
            let file_size = spec.headers.iter().filter(|p| p.filesz != 0)
                .map(|p| p.offset as usize + p.filesz as usize).max().unwrap();
            let p = MbnSourceSlices::new(spec, file_size).unwrap();
            let report = p.report();
            assert_eq!(report.bytes.generated(), 4264, "{}", spec.name);
            assert_eq!(report.bytes.total(), file_size);
            assert!(report.load_permissions.total() <= file_size);
            assert_eq!(report.memory_only_bytes, spec.headers.iter().filter(|h| h.kind == PT_LOAD)
                .map(|h| u64::from(h.memsz - h.filesz)).sum::<u64>());
            for segment in &report.segments {
                assert_eq!(segment.bytes.total(), segment.file_range.len());
                if segment.file_range.is_empty() {
                    assert_eq!(segment.bytes.total(), 0);
                    assert!(segment.holes.is_empty());
                }
            }
        }
    }

    #[test]
    fn bss_cannot_accept_source_and_data_segment_cannot_claim_instructions() {
        let spec = KNOWN_ELFS.iter().find(|s| s.name == "cdsp.mbn").unwrap();
        let size = spec.headers.iter().filter(|p| p.filesz != 0)
            .map(|p| p.offset as usize + p.filesz as usize).max().unwrap();
        let mut p = MbnSourceSlices::new(spec, size).unwrap();
        let bss = spec.headers.iter().position(|h| h.kind == PT_LOAD && h.filesz == 0).unwrap();
        assert!(p.add_source_slice(SourceSlice { segment_index: bss,
            file_offset: spec.headers[bss].offset as usize, name: "BSS", kind: SourceKind::Data,
            bytes: &[0] }).unwrap_err().contains("BSS"));
        let data = spec.headers.iter().position(|h| h.kind == PT_LOAD && h.filesz != 0 && h.flags & PF_X == 0).unwrap();
        assert!(p.add_source_slice(SourceSlice { segment_index: data,
            file_offset: spec.headers[data].offset as usize, name: "not code", kind: SourceKind::Code,
            bytes: &[1] }).unwrap_err().contains("executable PT_LOAD"));
    }

    #[test]
    fn nested_note_and_dynamic_views_are_not_duplicate_load_bytes() {
        let spec = KNOWN_ELFS.iter().find(|s| s.name == "wlanmdsp.mbn").unwrap();
        let report = MbnSourceSlices::new(spec, 4331136).unwrap().report();
        let load_sum: usize = spec.headers.iter().filter(|h| h.kind == PT_LOAD)
            .map(|h| h.filesz as usize).sum();
        assert_eq!(report.load_permissions.total(), load_sum);
        assert_eq!(report.segments.iter().filter(|s| s.kind == 2 || s.kind == 4).count(), 2);
    }

    #[test]
    fn overlapping_load_permissions_have_disjoint_union_counts() {
        let load = |offset, filesz, flags| Elf32Program { kind: PT_LOAD, offset, vaddr: 0,
            paddr: 0, filesz, memsz: filesz, flags, align: 1 };
        let headers: &'static [Elf32Program] = Box::leak(vec![
            load(10, 10, 5), load(15, 10, 6), load(30, 10, 7), load(50, 10, 4)
        ].into_boxed_slice());
        let spec = Elf32Spec { headers, ..*a640() };
        assert_eq!(permission_union(&spec), LoadPermissionBytes {
            executable_not_writable: 5, writable_not_executable: 5,
            executable_and_writable: 15, neither_executable_nor_writable: 10,
        });
    }

    #[test]
    fn malformed_hash_overlap_and_elf32_address_wrap_fail_at_construction() {
        let mut headers = a640().headers.to_vec();
        headers[2].offset = 0x1100;
        let spec = Elf32Spec { headers: Box::leak(headers.into_boxed_slice()), ..*a640() };
        assert!(MbnSourceSlices::new(&spec, 0x37b0).unwrap_err().contains("overlaps"));
        let mut headers = a640().headers.to_vec();
        headers[2].vaddr = u32::MAX - 2;
        let spec = Elf32Spec { headers: Box::leak(headers.into_boxed_slice()), ..*a640() };
        assert!(MbnSourceSlices::new(&spec, 0x37b0).unwrap_err().contains("exceeds ELF32"));
        let mut headers = a640().headers.to_vec();
        headers[2].offset = 0x100;
        let spec = Elf32Spec { headers: Box::leak(headers.into_boxed_slice()), ..*a640() };
        assert!(MbnSourceSlices::new(&spec, 0x37b0).unwrap_err().contains("first-page alignment"));
    }

    #[test]
    fn real_a640_source_helper_separates_header_and_terminal_shader_padding() {
        let plan = a640_source_plan().unwrap();
        let report = plan.report();
        assert_eq!(report.bytes.code, 1956);
        assert_eq!(report.bytes.data, 0);
        assert_eq!(report.bytes.metadata, 464);
        assert_eq!(report.bytes.layout_fill, 5436);
        assert_eq!(report.bytes.generated(), 7856);
        assert_eq!(report.bytes.external_authentication, 0);
        assert_eq!(report.holes, vec![0x1138..0x2a38]);
        assert_eq!(report.segments[2].bytes.code, 1956);
        assert_eq!(report.segments[2].bytes.metadata, 4);
        assert_eq!(report.segments[2].bytes.layout_fill, 8);
        assert!(plan.build_complete().is_err());
        let previous = crate::devices::xiaomi_nabu::a640_container::partial_container().unwrap();
        for region in previous.regions() {
            let span = region.offset..(region.offset + region.len);
            assert_eq!(plan.plan().generated_slice(span.clone()).unwrap(),
                previous.generated_slice(span).unwrap());
        }
    }
}
