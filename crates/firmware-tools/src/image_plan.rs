// SPDX-License-Identifier: GPL-3.0-only
//! Generic, checked image layout for independently generated firmware bytes.
//!
//! A partially reconstructed program is not a flashable binary. This builder
//! refuses to create a final image until *every byte* has been supplied by a
//! named source region. Unknown bytes must never be silently zero-filled.

use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub name: String,
    pub offset: usize,
    pub len: usize,
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
    pub fn place(&mut self, name: &str, offset: usize, generated: &[u8]) -> Result<(), String> {
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
        self.regions.push(Region { name: name.into(), offset, len: generated.len() });
        Ok(())
    }

    pub fn filled_bytes(&self) -> usize {
        self.regions.iter().map(|r| r.len).sum()
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

    /// Only a completely covered image can be emitted. Avoid constructing a
    /// visually convincing but incomplete binary padded with placeholder data.
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
