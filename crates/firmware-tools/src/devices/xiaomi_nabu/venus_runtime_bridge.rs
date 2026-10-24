// SPDX-License-Identifier: GPL-3.0-only
//! Strictly contiguous, independently regenerated Venus bridge bytes.
//! This assembler intentionally refuses to substitute missing firmware bytes.
use super::{venus_runtime, venus_runtime_bridge_lookup, venus_plan};
use crate::image_plan::ImagePlan;

pub const START: usize = venus_runtime_bridge_lookup::START;
pub const END: usize = venus_runtime_bridge_lookup::END;
pub const FILE_OFFSET: usize = venus_runtime_bridge_lookup::FILE_OFFSET;
pub const BRIDGE_LENGTH: usize = END - START;

pub fn fragment() -> Result<Vec<u8>, String> {
    venus_runtime_bridge_lookup::fragment()
}

/// Build a checked file-offset-aware prefix composed only of authored pieces.
/// The service region is intentionally not included until the full gap closes.
pub fn contiguous_boot_and_bridge() -> Result<ImagePlan, String> {
    let initial = venus_plan::first_contiguous_region()?.build_complete()?;
    let bridge = fragment()?;
    if START != 0x1000 + venus_runtime::PREFIX_SIZE ||
       FILE_OFFSET != venus_runtime::FILE_OFFSET + venus_runtime::PREFIX_SIZE {
        return Err("the Venus bridge must start exactly after the existing runtime prefix".into());
    }
    let mut plan = ImagePlan::new(initial.len() + bridge.len())?;
    plan.place("independently authored boot and runtime", 0, &initial)?;
    plan.place("typed A32 record lookup (partial bridge)", initial.len(), &bridge)?;
    if !plan.holes().is_empty() {
        return Err("bridge layout has an uncovered byte".into());
    }
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boot_and_bridge_are_adjacent() {
        let plan = contiguous_boot_and_bridge().unwrap();
        assert_eq!(plan.total_bytes(), END);
        assert!(plan.holes().is_empty());
        assert_eq!(plan.regions().len(), 2);
        assert_eq!(plan.filled_bytes(), END);
    }
}
