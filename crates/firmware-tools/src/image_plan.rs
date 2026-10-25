// SPDX-License-Identifier: GPL-3.0-only
//! Generic, checked image layout for independently generated firmware bytes.
//!
//! A partially reconstructed program is not a flashable binary. This builder
//! refuses to create a final image until *every byte* has been supplied by a
//! named source region. Unknown bytes must never be silently zero-filled.

use std::ops::Range;

/// Tracks where bytes came from, *not* whether a signature or device accepts
/// them. It is never safe to count externally imported bytes as decompiled
/// or independently generated implementation coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionOrigin {
    Generated,
    ExternalAuthentication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub name: String,
    pub offset: usize,
    pub len: usize,
    pub origin: RegionOrigin,
}

/// Disjoint byte counts; `covered_bytes` may include external authentication
/// but `generated_bytes` never does. Unknown bytes are always reported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageReport {
    pub total_bytes: usize,
    pub generated_bytes: usize,
    pub externally_supplied_bytes: usize,
    pub missing_bytes: usize,
    pub holes: Vec<Range<usize>>,
}

impl CoverageReport {
    pub fn covered_bytes(&self) -> usize {
        self.generated_bytes + self.externally_supplied_bytes
    }
    pub fn is_independently_complete(&self) -> bool {
        self.missing_bytes == 0 && self.externally_supplied_bytes == 0
    }
}

#[derive(Debug, Clone)]
pub struct ImagePlan {
    contents: Vec<Option<u8>>,
    regions: Vec<Region>,
}

impl ImagePlan {
    /// Create a bounded image plan without assuming a particular device.
    pub fn new(total_bytes: usize) -> Result<Self, String> {
        if total_bytes == 0 { return Err("image size must be nonzero".into()); }
        Ok(Self { contents: vec![None; total_bytes], regions: Vec::new() })
    }

    pub fn total_bytes(&self) -> usize { self.contents.len() }
    pub fn regions(&self) -> &[Region] { &self.regions }

    /// Place independently authored bytes into a nonoverlapping region.
    /// This is a claim by the caller; it must be substantiated with tests.
    pub fn place(&mut self, name: &str, offset: usize, generated: &[u8]) -> Result<(), String> {
        self.place_with_origin(name, offset, generated, RegionOrigin::Generated)
    }

    /// Place *externally supplied* signature/certificate field bytes in a
    /// checked range, retaining their provenance in coverage and finalization.
    /// Does not perform cryptographic validation of the supplied bytes.
    pub fn place_external_authentication(
        &mut self, name: &str, offset: usize, supplied: &[u8],
    ) -> Result<(), String> {
        self.place_with_origin(name, offset, supplied, RegionOrigin::ExternalAuthentication)
    }

    fn place_with_origin(
        &mut self, name: &str, offset: usize, generated: &[u8], origin: RegionOrigin,
    ) -> Result<(), String> {
        if name.trim().is_empty() { return Err("image region name cannot be empty".into()); }
        if generated.is_empty() { return Err(format!("empty image region: {name}")); }
        let end = offset.checked_add(generated.len()).ok_or("image offset overflow")?;
        if end > self.contents.len() {
            return Err(format!("region {name} exceeds image bounds: {offset}..{end}"));
        }
        if self.contents[offset..end].iter().any(Option::is_some) {
            return Err(format!("region {name} overlaps an earlier image region at {offset}..{end}"));
        }
        for (cell, byte) in self.contents[offset..end].iter_mut().zip(generated) {
            *cell = Some(*byte);
        }
        self.regions.push(Region { name: name.into(), offset, len: generated.len(), origin });
        Ok(())
    }

    pub fn filled_bytes(&self) -> usize {
        self.regions.iter().map(|r| r.len).sum()
    }

    /// Reporting is independent of insertion order. BSS is not file-backed
    /// content and must not be silently counted as reconstructed file bytes.
    pub fn coverage(&self) -> CoverageReport {
        let holes = self.holes();
        let missing_bytes = holes.iter().map(|r| r.end - r.start).sum();
        CoverageReport {
            total_bytes: self.total_bytes(),
            generated_bytes: self.regions.iter().filter(|r| r.origin == RegionOrigin::Generated)
                .map(|r| r.len).sum(),
            externally_supplied_bytes: self.regions.iter()
                .filter(|r| r.origin == RegionOrigin::ExternalAuthentication)
                .map(|r| r.len).sum(),
            missing_bytes,
            holes,
        }
    }

    /// Every maximal missing-byte range, including a final trailing hole.
    pub fn holes(&self) -> Vec<Range<usize>> {
        let mut holes = Vec::new();
        let mut start: Option<usize> = None;
        for (index, value) in self.contents.iter().enumerate() {
            match (start, value) {
                (None, None) => start = Some(index),
                (Some(begin), Some(_)) => {
                    holes.push(begin..index);
                    start = None;
                }
                _ => {}
            }
        }
        if let Some(begin) = start { holes.push(begin..self.contents.len()); }
        holes
    }

    /// Extract a file-backed range only if **every byte is independently
    /// generated**, rather than unknown or imported as OEM authentication.
    /// This is the safe input to segment hashing during fresh image assembly.
    pub fn generated_slice(&self, span: Range<usize>) -> Result<Vec<u8>, String> {
        if span.start > span.end || span.end > self.contents.len() {
            return Err("generated slice lies outside the image".into());
        }
        let mut covered = vec![false; span.len()];
        for region in &self.regions {
            let first = region.offset.max(span.start);
            let end = (region.offset + region.len).min(span.end);
            if first >= end { continue; }
            if region.origin != RegionOrigin::Generated {
                return Err(format!("cannot hash externally supplied authentication at file offset 0x{first:x}"));
            }
            covered[first - span.start..end - span.start].fill(true);
        }
        if let Some(index) = covered.iter().position(|is_generated| !is_generated) {
            return Err(format!("cannot hash unimplemented file byte at 0x{:x}", span.start + index));
        }
        Ok(self.contents[span].iter().copied().map(|v| v.expect("generated coverage checked")).collect())
    }

    /// Compare *only* the independently generated regions with a reference.
    /// This does not imply that unimplemented bytes are correct.
    pub fn compare_known(&self, reference: &[u8]) -> Result<(), String> {
        if reference.len() != self.contents.len() {
            return Err(format!("reference size {} does not equal planned size {}",
                reference.len(), self.contents.len()));
        }
        for region in &self.regions {
            for i in region.offset..region.offset + region.len {
                if let Some(expected) = self.contents[i] {
                    if reference[i] != expected {
                        return Err(format!("byte mismatch in {} at image offset {i} (0x{i:x})", region.name));
                    }
                }
            }
        }
        Ok(())
    }

    /// Refuse to call an externally authenticated image independently rebuilt.
    /// This stricter method is intended for complete original-source builds.
    pub fn build_independently_generated(&self) -> Result<Vec<u8>, String> {
        let result = self.build_complete()?;
        if self.coverage().externally_supplied_bytes != 0 {
            return Err(format!("image contains {} externally supplied authentication bytes; independent build forbidden",
                self.coverage().externally_supplied_bytes));
        }
        Ok(result)
    }

    /// Only a completely covered image can be emitted. Externally supplied
    /// authentication bytes are permitted but remain classified as external.
    /// Emission is NOT evidence of a newly signed or hardware-trusted image.
    pub fn build_complete(&self) -> Result<Vec<u8>, String> {
        if let Some(first) = self.holes().first() {
            return Err(format!("image incomplete: {} / {} bytes generated; first gap 0x{:x}..0x{:x}",
                self.filled_bytes(), self.total_bytes(), first.start, first.end));
        }
        Ok(self.contents.iter().map(|x| x.expect("holes checked")).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gaps_and_finalization_are_strict() {
        let mut plan = ImagePlan::new(12).unwrap();
        plan.place("first", 0, &[1, 2, 3]).unwrap();
        plan.place("last", 8, &[9, 10, 11, 12]).unwrap();
        assert_eq!(plan.filled_bytes(), 7);
        assert_eq!(plan.holes(), vec![3..8]);
        assert!(plan.build_complete().unwrap_err().contains("incomplete"));
        let mut reference = [1, 2, 3, 0, 0, 0, 0, 0, 9, 10, 11, 12];
        plan.compare_known(&reference).unwrap();
        reference[9] = 0;
        assert!(plan.compare_known(&reference).unwrap_err().contains("last"));
        plan.place("middle", 3, &[4, 5, 6, 7, 8]).unwrap();
        assert_eq!(plan.holes(), Vec::<Range<usize>>::new());
        assert_eq!(plan.build_complete().unwrap(), (1..=12).collect::<Vec<_>>());
    }
    #[test]
    fn coverage_distinguishes_external_authentication_and_holes() {
        let mut plan = ImagePlan::new(13).unwrap();
        plan.place("source metadata", 0, &[0xa1, 0xa2, 0xa3]).unwrap();
        plan.place_external_authentication("OEM cert from owner", 6, &[7, 8, 9, 10]).unwrap();
        assert_eq!(plan.coverage(), CoverageReport {
            total_bytes: 13, generated_bytes: 3, externally_supplied_bytes: 4,
            missing_bytes: 6, holes: vec![3..6, 10..13],
        });
        assert!(!plan.coverage().is_independently_complete());
        assert!(plan.build_complete().is_err());
        plan.place("rest", 3, &[4, 5, 6]).unwrap();
        plan.place("tail", 10, &[11, 12, 13]).unwrap();
        assert_eq!(plan.coverage().covered_bytes(), 13);
        assert_eq!(plan.coverage().generated_bytes, 9);
        assert_eq!(plan.coverage().externally_supplied_bytes, 4);
        assert!(plan.build_complete().is_ok());
        assert!(plan.build_independently_generated().unwrap_err().contains("externally supplied"));
        assert!(plan.place_external_authentication("bad overlap", 6, &[0x99]).is_err());
    }
    #[test]
    fn generated_slice_rejects_out_of_bounds_unknown_and_authentication() {
        let mut p = ImagePlan::new(8).unwrap();
        p.place("start", 0, &[1, 2]).unwrap();
        p.place_external_authentication("auth", 2, &[3, 4]).unwrap();
        p.place("finish", 5, &[6, 7, 8]).unwrap();
        assert_eq!(p.generated_slice(0..2).unwrap(), vec![1, 2]);
        assert_eq!(p.generated_slice(0..0).unwrap(), Vec::<u8>::new());
        assert!(p.generated_slice(2..3).unwrap_err().contains("externally supplied"));
        assert!(p.generated_slice(4..6).unwrap_err().contains("unimplemented"));
        assert!(p.generated_slice(8..9).is_err());
        assert!(p.generated_slice(7..6).is_err());
        assert!(p.generated_slice(0..8).unwrap_err().contains("externally supplied"));
    }
    #[test]
    fn generated_coverage_can_be_finalized_independently() {
        let mut plan = ImagePlan::new(2).unwrap();
        plan.place("generated", 0, &[0xaa, 0xbb]).unwrap();
        assert_eq!(plan.build_independently_generated().unwrap(), vec![0xaa, 0xbb]);
        assert!(plan.coverage().is_independently_complete());
    }
    #[test]
    fn rejects_overlaps_bounds_empty_inputs_and_overflow() {
        let mut plan = ImagePlan::new(4).unwrap();
        assert!(ImagePlan::new(0).is_err());
        assert!(plan.place("", 0, &[1]).is_err());
        assert!(plan.place("empty", 0, &[]).is_err());
        assert!(plan.place("overflow", usize::MAX, &[1, 2]).is_err());
        assert!(plan.place("oversized", 3, &[1, 2]).is_err());
        plan.place("good", 1, &[2, 3]).unwrap();
        assert!(plan.place("overlap", 2, &[3]).is_err());
        assert_eq!(plan.filled_bytes(), 2);
        assert_eq!(plan.holes(), vec![0..1, 3..4]);
        assert!(plan.compare_known(&[]).is_err());
    }
}
