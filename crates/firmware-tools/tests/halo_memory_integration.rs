// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::{wmfw_algorithms, wmfw_halo_calibration::SuccessfulCalibration,
    wmfw_halo_fixed_point::Signed24};

#[test]
fn calibration_plan_agrees_with_named_wmfw_metadata_and_numeric_codec() {
    let schema = wmfw_algorithms::cspl();
    let plan = SuccessfulCalibration::new(8192, Some(-5)).unwrap().relative_write_plan();
    for write in &plan {
        let control = schema.controls.iter().find(|c| c.name == write.name).unwrap();
        assert_eq!(write.algorithm_id, schema.id);
        assert_eq!(write.word_offset, control.offset);
        assert_eq!(write.memory_type, control.memory_type);
        assert_eq!(control.size_bytes, write.payload.len() as u32);
    }
    assert_eq!(plan[0].payload, Signed24::new(-5).unwrap().to_unpacked_be32());
    assert_eq!(plan.last().unwrap().name, "CAL_CHECKSUM");
    assert_eq!(plan.last().unwrap().payload, 8193u32.to_be_bytes());
}
