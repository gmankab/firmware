// SPDX-License-Identifier: GPL-3.0-only
//! Reproducible contiguous early Venus code-and-data range. The remaining
//! firmware image and OEM authentication are deliberately out of scope.
use crate::image_plan::ImagePlan;
use super::{venus_boot, venus_runtime};

pub const START_FILE_OFFSET: usize = venus_boot::FILE_OFFSET;
pub const CONTIGUOUS_SIZE: usize = venus_boot::FIRST_PAGE_SIZE + venus_runtime::PREFIX_SIZE;

pub fn first_contiguous_region() -> Result<ImagePlan, String> {
    if venus_runtime::FILE_OFFSET !=
       START_FILE_OFFSET + venus_boot::FIRST_PAGE_SIZE {
        return Err("Venus first runtime block is not adjacent to boot page".into());
    }
    let mut plan = ImagePlan::new(CONTIGUOUS_SIZE)?;
    plan.place("typed ARM32 boot page with named literals and zero fill",
        0, &venus_boot::first_page()?)?;
    plan.place("typed ARM32 runtime entry stubs",
        venus_boot::FIRST_PAGE_SIZE, &venus_runtime::prefix()?)?;
    if !plan.holes().is_empty() {
        return Err("Venus early boot image has uncovered bytes".into());
    }
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strict_contiguous_venus_boot_region() {
        let plan = first_contiguous_region().unwrap();
        assert_eq!(plan.total_bytes(), 7872);
        assert_eq!(plan.filled_bytes(), 7872);
        assert!(plan.holes().is_empty());
        assert_eq!(plan.regions().len(), 2);
        assert!(plan.build_complete().is_ok());
    }
}
