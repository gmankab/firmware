// SPDX-License-Identifier: GPL-3.0-only
//! Strictly contiguous, independently regenerated Venus bridge bytes.
//! This assembler intentionally refuses to substitute missing firmware bytes.
use super::{venus_runtime, venus_runtime_bridge_lookup, venus_runtime_bridge_records, venus_runtime_bridge_state, venus_runtime_bridge_slots, venus_runtime_bridge_irq_ready, venus_runtime_bridge_resource, venus_runtime_bridge_queue_release, venus_runtime_bridge_controller_queues, venus_runtime_bridge_service_link, venus_plan};
use crate::image_plan::ImagePlan;

pub const START: usize = venus_runtime_bridge_lookup::START;
pub const END: usize = venus_runtime_bridge_service_link::END;
pub const FILE_OFFSET: usize = venus_runtime_bridge_lookup::FILE_OFFSET;
pub const BRIDGE_LENGTH: usize = END - START;

pub fn fragment() -> Result<Vec<u8>, String> {
    let first = venus_runtime_bridge_lookup::fragment()?;
    let additional = venus_runtime_bridge_records::fragment()?;
    if venus_runtime_bridge_lookup::END != venus_runtime_bridge_records::START ||
       venus_runtime_bridge_records::FILE_OFFSET != venus_runtime_bridge_lookup::END + 0x4000 {
        return Err("Venus record/queue bridge fragments are not adjacent".into());
    }
    let state = venus_runtime_bridge_state::fragment()?;
    if venus_runtime_bridge_records::END != venus_runtime_bridge_state::START ||
       venus_runtime_bridge_state::FILE_OFFSET != venus_runtime_bridge_records::END + 0x4000 {
        return Err("Venus queue/state bridge fragments are not adjacent".into());
    }
    let slots = venus_runtime_bridge_slots::fragment()?;
    if venus_runtime_bridge_state::END != venus_runtime_bridge_slots::START ||
       venus_runtime_bridge_slots::FILE_OFFSET != venus_runtime_bridge_state::END + 0x4000 {
        return Err("Venus state/slot bridge fragments are not adjacent".into());
    }
    let mut result = first;
    result.extend_from_slice(&additional);
    result.extend_from_slice(&state);
    result.extend_from_slice(&slots);
    if venus_runtime_bridge_slots::END != venus_runtime_bridge_irq_ready::START ||
       venus_runtime_bridge_irq_ready::FILE_OFFSET != venus_runtime_bridge_slots::END+0x4000 {
        return Err("Venus IRQ-ready fragments are not adjacent".into());
    }
    result.extend_from_slice(&venus_runtime_bridge_irq_ready::fragment()?);
    if venus_runtime_bridge_irq_ready::END != venus_runtime_bridge_resource::START ||
       venus_runtime_bridge_resource::FILE_OFFSET != venus_runtime_bridge_irq_ready::END+0x4000 {
        return Err("Venus IRQ/resource fragments are not adjacent".into());
    }
    result.extend_from_slice(&venus_runtime_bridge_resource::fragment()?);
    if venus_runtime_bridge_resource::END != venus_runtime_bridge_queue_release::START ||
       venus_runtime_bridge_queue_release::FILE_OFFSET != venus_runtime_bridge_resource::END+0x4000 {
        return Err("Venus resource/release fragments are not adjacent".into());
    }
    result.extend_from_slice(&venus_runtime_bridge_queue_release::fragment()?);
    if venus_runtime_bridge_queue_release::END != venus_runtime_bridge_controller_queues::START ||
       venus_runtime_bridge_controller_queues::FILE_OFFSET != venus_runtime_bridge_queue_release::END+0x4000 {
        return Err("Venus release/controller queue fragments are not adjacent".into());
    }
    result.extend_from_slice(&venus_runtime_bridge_controller_queues::fragment()?);
    if venus_runtime_bridge_controller_queues::END != venus_runtime_bridge_service_link::START ||
       venus_runtime_bridge_service_link::FILE_OFFSET != venus_runtime_bridge_controller_queues::END+0x4000 {
        return Err("Venus queue/service link fragments are not adjacent".into());
    }
    result.extend_from_slice(&venus_runtime_bridge_service_link::fragment()?);
    if result.len() != BRIDGE_LENGTH {
        return Err("Venus bridge fragment coverage gap".into());
    }
    Ok(result)
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
    plan.place("typed A32 record lookup and queue selection (partial bridge)", initial.len(), &bridge)?;
    if !plan.holes().is_empty() {
        return Err("bridge layout has an uncovered byte".into());
    }
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_bridge_reaches_service_start_without_holes() {
        assert_eq!(START, 0x8490);
        assert_eq!(END, 0xaa0c);
        assert_eq!(BRIDGE_LENGTH, 9596);
        assert_eq!(FILE_OFFSET, 0xc490);
        assert_eq!(venus_runtime_bridge_service_link::FILE_OFFSET +
                   (venus_runtime_bridge_service_link::END -
                    venus_runtime_bridge_service_link::START), 0xea0c);
        let rebuilt = fragment().unwrap();
        assert_eq!(rebuilt.len(), 9596);
        let plan = contiguous_boot_and_bridge().unwrap();
        assert!(plan.holes().is_empty());
        assert_eq!(plan.filled_bytes(), 43532);
        assert_eq!(plan.build_complete().unwrap().len(), 43532);
    }
    #[test]
    fn boot_and_bridge_are_adjacent() {
        let plan = contiguous_boot_and_bridge().unwrap();
        assert_eq!(plan.total_bytes(), END);
        assert!(plan.holes().is_empty());
        assert_eq!(plan.regions().len(), 2);
        assert_eq!(plan.filled_bytes(), END);
    }
}
