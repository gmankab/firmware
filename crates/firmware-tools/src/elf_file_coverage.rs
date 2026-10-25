// SPDX-License-Identifier: GPL-3.0-only
//! Auditable file-byte inventory for ELF32 firmware. A program's BSS exists in
//! *memory*, not as bytes to synthesize in an MBN file. Alignment gaps are not
//! assumed to be zeros unless an independent generator expressly models them.
use std::ops::Range;
use crate::{elf32_prefix::{Elf32Spec, ELF_PAGE_LEN, write_elf32_first_page},
    image_plan::{ImagePlan, CoverageReport}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfFileInventory {
    pub name: &'static str,
    pub file_size: usize,
    pub program_header_count: usize,
    /// Union of all program-header file spans, avoiding double counting
    /// overlapping PT_NOTE, PT_LOAD and the initial metadata header record.
    pub file_segment_bytes: usize,
    /// Sum of PT_LOAD p_memsz - p_filesz values; not file bytes and not
    /// necessarily unique virtual addresses across sections.
    pub zero_fill_memory_bytes: u64,
    pub executable_file_ranges: Vec<Range<usize>>,
    pub authentication_file_range: Range<usize>,
    pub coverage: CoverageReport,
}

/// Generate the strict, original-source-dependent ELF32 first page only.
/// The total size is provided by a caller or an actual private reference;
/// no inaccessible alignment, signature or executable contents are inferred.
pub fn checked_elf_header_plan(
    spec: &Elf32Spec, total_file_bytes: usize,
) -> Result<(ImagePlan, ElfFileInventory), String> {
    if total_file_bytes < ELF_PAGE_LEN { return Err("firmware file smaller than ELF page".into()); }
    if spec.headers.is_empty() { return Err("ELF spec contains no program headers".into()); }
    let mut auth_segments = Vec::new();
    let mut file_spans = Vec::<Range<usize>>::new();
    let mut exec = Vec::new();
    let mut memory_only = 0u64;
    for (index, header) in spec.headers.iter().enumerate() {
        let start = header.offset as usize;
        let end = start.checked_add(header.filesz as usize)
            .ok_or("ELF file span overflow")?;
        // Zero-length file segments can be pure BSS whose p_offset is at
        // or *beyond* physical EOF. They consume no file bytes and are not
        // evidence of a truncated image (common in Qualcomm MDTs).
        if header.filesz != 0 && end > total_file_bytes {
            return Err(format!("{}: program header {index} exceeds file at 0x{end:x}",spec.name));
        }
        if header.kind == 1 {
            if header.filesz > header.memsz {
                return Err(format!("{}: PT_LOAD index {index} p_filesz exceeds p_memsz",spec.name));
            }
            memory_only = memory_only.checked_add((header.memsz - header.filesz) as u64)
                .ok_or("ELF memory-only sum overflow")?;
            if header.flags & 1 != 0 && header.filesz != 0 { exec.push(start..end); }
        }
        if header.flags & 0x0700_0000 == 0x0200_0000 {
            if end == start { return Err(format!("{}: empty MBN authentication segment",spec.name)); }
            auth_segments.push(start..end);
        }
        if end > start { file_spans.push(start..end); }
    }
    if auth_segments.len() != 1 {
        return Err(format!("{}: expected exactly one MBN authentication segment, got {}",
            spec.name, auth_segments.len()));
    }
    file_spans.sort_by_key(|span| (span.start, span.end));
    let mut covered = 0usize;
    let mut last_end = 0usize;
    for span in &file_spans {
        let begin = span.start.max(last_end);
        if span.end > begin { covered = covered.checked_add(span.end - begin)
            .ok_or("ELF file-backed byte count overflow")?; }
        last_end = last_end.max(span.end);
    }
    let mut plan = ImagePlan::new(total_file_bytes)?;
    let source_generated = write_elf32_first_page(spec)?;
    plan.place("source-generated ELF32 header/program-table page",0,&source_generated)?;
    let inventory = ElfFileInventory {
        name:spec.name, file_size:total_file_bytes,
        program_header_count:spec.headers.len(),file_segment_bytes:covered,
        zero_fill_memory_bytes:memory_only,executable_file_ranges:exec,
        authentication_file_range:auth_segments.remove(0), coverage:plan.coverage(),
    };
    Ok((plan,inventory))
}

/// Verify exact reference geometry and bytes for the independently generated
/// first page. This comparison never imports signed material into the plan.
pub fn compare_known_first_page(
    spec: &Elf32Spec, reference: &[u8],
) -> Result<ElfFileInventory, String> {
    let (plan,inventory)=checked_elf_header_plan(spec,reference.len())?;
    plan.compare_known(reference)?;
    Ok(inventory)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::xiaomi_nabu::elf_specs::KNOWN_ELFS;
    #[test]
    fn all_six_specs_cover_exactly_a_page_without_implicit_bss() {
        assert_eq!(KNOWN_ELFS.len(),6);
        for spec in KNOWN_ELFS {
            let max_file=spec.headers.iter()
                .filter_map(|ph|(ph.offset as usize).checked_add(ph.filesz as usize))
                .max().unwrap().max(4096);
            let (plan,inv)=checked_elf_header_plan(spec,max_file).unwrap();
            assert_eq!(plan.filled_bytes(),ELF_PAGE_LEN);
            assert_eq!(inv.file_size,max_file);
            assert_eq!(inv.coverage.generated_bytes,ELF_PAGE_LEN);
            assert_eq!(inv.coverage.externally_supplied_bytes,0);
            assert_eq!(inv.coverage.missing_bytes,max_file - ELF_PAGE_LEN);
            assert!(inv.file_segment_bytes<=max_file);
            assert_eq!(inv.authentication_file_range.start,0x1000);
            assert!(inv.program_header_count>=3);
            for x in &inv.executable_file_ranges {
                assert!(!x.is_empty());
                assert!(x.end<=max_file);
            }
            assert!(plan.build_complete().is_err());
        }
    }
    #[test]
    fn rejects_missing_hash_segments_and_file_overruns() {
        const HEADERS: &[crate::elf32_prefix::Elf32Program] = &[
            crate::elf32_prefix::Elf32Program { kind:1,offset:0x1000,
                vaddr:0,paddr:0,filesz:0x200,memsz:0x200,flags:0x5,align:0x1000 },
        ];
        let no_auth=Elf32Spec{name:"fixture",machine:164,entry:0,flags:0,
            section_header_size:0,headers:HEADERS};
        assert!(checked_elf_header_plan(&no_auth,0x1200).is_err());
        assert!(checked_elf_header_plan(KNOWN_ELFS.first().unwrap(),0x1000)
            .unwrap_err().contains("exceeds file"));
        let (p,inv)=checked_elf_header_plan(KNOWN_ELFS.first().unwrap(),0x37b0).unwrap();
        assert_eq!(inv.file_segment_bytes,148+0x1a38+0x7b0);
        assert_eq!(p.coverage().holes,vec![0x1000..0x37b0]);
    }
    #[test]
    fn accepts_bss_with_non_file_offset_beyond_eof() {
        const PROGRAMS: &[crate::elf32_prefix::Elf32Program] = &[
            crate::elf32_prefix::Elf32Program{kind:0,offset:0x1000,vaddr:0,paddr:0,
                filesz:0x20,memsz:0,flags:0x02000000,align:0x1000},
            crate::elf32_prefix::Elf32Program{kind:1,offset:0x10000,vaddr:0x80000000,
                paddr:0x80000000,filesz:0,memsz:0x3000,flags:6,align:0x1000},
        ];
        let spec=Elf32Spec{name:"BSS fixture",machine:164,entry:0,flags:0,
            section_header_size:0,headers:PROGRAMS};
        let (_,inv)=checked_elf_header_plan(&spec,0x1020).unwrap();
        assert_eq!(inv.zero_fill_memory_bytes,0x3000);
        assert_eq!(inv.file_segment_bytes,0x20);
        assert!(inv.executable_file_ranges.is_empty());
    }
    #[test]
    fn reference_verification_reports_first_mismatching_header_byte() {
        let spec=KNOWN_ELFS.first().unwrap();
        let (plan,_) = checked_elf_header_plan(spec,0x37b0).unwrap();
        let mut reference=vec![0;0x37b0];
        reference[..4096].copy_from_slice(&write_elf32_first_page(spec).unwrap());
        compare_known_first_page(spec,&reference).unwrap();
        reference[0x38]^=0x80;
        assert!(compare_known_first_page(spec,&reference).unwrap_err()
            .contains("0x38"));
        assert!(plan.build_complete().is_err());
    }
}
