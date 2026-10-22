// SPDX-License-Identifier: GPL-3.0-only
//! Complete executable-segment plan for the reference A640 ZAP firmware revision.
//! The generic image planner is reusable by unrelated devices and ISAs.

use crate::image_plan::ImagePlan;
use super::{a640_finalize, a640_launch, a640_pipeline, a640_shader, A640_FAMILY, A640_REVISION};

pub const EXECUTABLE_SIZE: usize = 1968;
pub const SHADER_OFFSET: usize = 0x440;

pub fn partial_executable() -> Result<ImagePlan, String> {
    let mut image = ImagePlan::new(EXECUTABLE_SIZE)?;
    image.place("QRISC security command dispatch", 0, &qrisc::a640::security_setup_prefix(
        A640_FAMILY, A640_REVISION
    )?)?;
    image.place("QRISC protected shader register setup", a640_launch::START_WORD * 4,
        &a640_launch::protected_shader_register_setup()?)?;
    image.place("QRISC compute register configuration", a640_launch::CONFIG_START_WORD * 4,
        &a640_launch::compute_register_configuration()?)?;
    image.place("QRISC protected compute, shader address and result resolve",
        a640_pipeline::START_WORD * 4, &a640_pipeline::protected_compute_and_resolve()?)?;
    for fragment in a640_shader::verified_candidates()? {
        image.place(fragment.purpose, SHADER_OFFSET + fragment.first_slot * 8,
            &fragment.bytes)?;
    }
    image.place("QRISC final security-state commit and event helpers",
        a640_finalize::START_WORD * 4, &a640_finalize::finish_protected_compute()?)?;
    let idle = qrisc::a640::wait_for_idle()?;
    image.place("QRISC wait-for-idle and return", EXECUTABLE_SIZE - idle.len(), &idle)?;
    Ok(image)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_executable_regions_are_nonoverlapping_and_complete() {
        let plan = partial_executable().unwrap();
        assert_eq!(plan.filled_bytes(), EXECUTABLE_SIZE);
        assert_eq!(plan.total_bytes(), EXECUTABLE_SIZE);
        assert!(plan.holes().is_empty());
        assert_eq!(plan.regions().len(), 15);
        assert_eq!(plan.build_complete().unwrap().len(), EXECUTABLE_SIZE);

    }
}
