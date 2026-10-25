// SPDX-License-Identifier: GPL-3.0-only
//! Checked CSPL successful-calibration records and relative control payloads.
//!
//! Public contract: CirrusLogic/mcu-drivers commit
//! a05b2878be911ae9ffaa440840cde17af98ba48e, cs35l41/cs35l41.c, defines
//! success=1, waiting-for-data=2, and out-of-range=3. Its successful restore
//! sequence writes CAL_R, CAL_STATUS=1, and finally CAL_CHECKSUM=CAL_R+1.
//! https://github.com/CirrusLogic/mcu-drivers/blob/a05b2878be911ae9ffaa440840cde17af98ba48e/cs35l41/cs35l41.c
//!
//! Algorithm ID, memory kind, names and relative offsets below match the
//! repository's source-authored metadata for the pinned CS35L41 WMFW. They
//! are not absolute device register addresses. The caller may supply ambient
//! temperature explicitly in integer degrees Celsius; this optional value
//! is placed before the successful restore sequence. No runtime writes or
//! firmware-state transitions are performed by this module.
//! Canonical payload parsing deliberately requires a zero high byte. Linux
//! may sign-extend a negative host ambient value into that unused byte;
//! callers must normalize such input explicitly. Signed limits here describe
//! the representation, not an independently verified hardware temperature range.
//!
//! CAL_R remains an encoded unsigned word: no unverified Ohms/Q13 conversion
//! or physical calibration range is inferred. The checksum contract is only
//! accepted for SUCCESS, never extrapolated to arbitrary failing statuses.
//! This schema adds no independently reconstructed WMFW payload bytes.

pub const CSPL_ALGORITHM_ID: u32 = 0x00cd;
pub const XM_CONTROL_MEMORY_TYPE: u16 = 5;
pub const DATA_WORD_MASK: u32 = 0x00ff_ffff;
pub const SIGNED_DATA_MIN: i32 = -(1 << 23);
pub const SIGNED_DATA_MAX: i32 = (1 << 23) - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CalibrationStatus {
    Success = 1,
    WaitingForData = 2,
    OutOfRange = 3,
}

impl CalibrationStatus {
    pub fn from_word(word: u32) -> Result<Self, String> {
        match word {
            1 => Ok(Self::Success),
            2 => Ok(Self::WaitingForData),
            3 => Ok(Self::OutOfRange),
            _ => Err(format!("unknown CSPL calibration status {word}")),
        }
    }
}

/// One named control payload; `word_offset` is algorithm-relative, not AHB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelativeControlWrite {
    pub algorithm_id: u32,
    pub memory_type: u16,
    pub name: &'static str,
    pub word_offset: u16,
    /// Zero-padded, big-endian, unpacked-24 control-register representation.
    pub payload: [u8; 4],
}

/// A checked success record, with optional caller-supplied temperature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SuccessfulCalibration {
    encoded_resistance: u32,
    ambient_degrees_c: Option<i32>,
}

impl SuccessfulCalibration {
    /// Validate representation limits, not acoustic or electrical suitability.
    ///
    /// The resistance must fit 24 bits and leave room for its success checksum
    /// `resistance + 1`. In particular, 0xffffff is rejected instead of allowing
    /// an overflowing checksum to wrap to zero.
    pub fn new(
        encoded_resistance: u32,
        ambient_degrees_c: Option<i32>,
    ) -> Result<Self, String> {
        if encoded_resistance > DATA_WORD_MASK {
            return Err("CSPL encoded resistance exceeds 24-bit range".into());
        }
        let checksum = encoded_resistance
            .checked_add(CalibrationStatus::Success as u32)
            .ok_or("CSPL calibration checksum arithmetic overflow")?;
        if checksum > DATA_WORD_MASK {
            return Err("CSPL success checksum exceeds 24-bit range".into());
        }
        if let Some(ambient) = ambient_degrees_c {
            if !(SIGNED_DATA_MIN..=SIGNED_DATA_MAX).contains(&ambient) {
                return Err("ambient degrees Celsius exceeds signed 24-bit range".into());
            }
        }
        Ok(Self { encoded_resistance, ambient_degrees_c })
    }

    /// Validate an observed successful record without accepting failure states.
    pub fn from_readback(
        encoded_resistance: u32,
        status_word: u32,
        checksum_word: u32,
        ambient_degrees_c: Option<i32>,
    ) -> Result<Self, String> {
        let status = CalibrationStatus::from_word(status_word)?;
        if status != CalibrationStatus::Success {
            return Err(format!("CSPL calibration is not successful: {status:?}"));
        }
        let record = Self::new(encoded_resistance, ambient_degrees_c)?;
        if checksum_word > DATA_WORD_MASK {
            return Err("CSPL readback checksum exceeds 24-bit range".into());
        }
        if checksum_word != record.checksum() {
            return Err("CSPL successful calibration checksum mismatch".into());
        }
        Ok(record)
    }

    /// Parse separately read, canonical unpacked-24 control payloads.
    ///
    /// Supplying these bytes does not establish their source or the identity of
    /// a live device. A caller must obtain and associate the named controls.
    pub fn from_control_payloads(
        resistance: [u8; 4],
        status: [u8; 4],
        checksum: [u8; 4],
        ambient: Option<[u8; 4]>,
    ) -> Result<Self, String> {
        let resistance = decode_unsigned_control("CAL_R", resistance)?;
        let status = decode_unsigned_control("CAL_STATUS", status)?;
        let checksum = decode_unsigned_control("CAL_CHECKSUM", checksum)?;
        let ambient = ambient.map(|bytes| {
            let bits = decode_unsigned_control("CAL_AMBIENT", bytes)?;
            Ok::<i32, String>(if bits & (1 << 23) != 0 {
                bits as i32 - (1 << 24)
            } else {
                bits as i32
            })
        }).transpose()?;
        Self::from_readback(resistance, status, checksum, ambient)
    }

    pub fn encoded_resistance(self) -> u32 {
        self.encoded_resistance
    }

    pub fn ambient_degrees_c(self) -> Option<i32> {
        self.ambient_degrees_c
    }

    /// Guaranteed by the constructor to fit a 24-bit control payload.
    pub fn checksum(self) -> u32 {
        self.encoded_resistance + CalibrationStatus::Success as u32
    }

    /// Deterministic relative write plan; checksum is always the final item.
    pub fn relative_write_plan(self) -> Vec<RelativeControlWrite> {
        let mut plan = Vec::with_capacity(if self.ambient_degrees_c.is_some() { 4 } else { 3 });
        if let Some(ambient) = self.ambient_degrees_c {
            plan.push(control("CAL_AMBIENT", 7, ambient as u32 & DATA_WORD_MASK));
        }
        plan.push(control("CAL_R", 6, self.encoded_resistance));
        plan.push(control("CAL_STATUS", 8, CalibrationStatus::Success as u32));
        plan.push(control("CAL_CHECKSUM", 9, self.checksum()));
        plan
    }
}

fn decode_unsigned_control(name: &str, bytes: [u8; 4]) -> Result<u32, String> {
    if bytes[0] != 0 {
        return Err(format!("nonzero padding byte in CSPL {name} control"));
    }
    Ok(u32::from_be_bytes(bytes))
}

fn control(name: &'static str, word_offset: u16, value: u32) -> RelativeControlWrite {
    RelativeControlWrite {
        algorithm_id: CSPL_ALGORITHM_ID,
        memory_type: XM_CONTROL_MEMORY_TYPE,
        name,
        word_offset,
        payload: value.to_be_bytes(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_checksum_is_resistance_plus_one_and_checked_on_readback() {
        let record = SuccessfulCalibration::new(0x012345, None).unwrap();
        assert_eq!(record.encoded_resistance(), 0x012345);
        assert_eq!(record.checksum(), 0x012346);
        assert_eq!(SuccessfulCalibration::from_readback(0x012345, 1, 0x012346, None)
            .unwrap(), record);
        assert!(SuccessfulCalibration::from_readback(0x012345, 1, 0x012347, None)
            .unwrap_err().contains("mismatch"));
    }

    #[test]
    fn failing_status_is_rejected_even_when_a_plausible_checksum_is_supplied() {
        for status in [2, 3] {
            for checksum in [101, 100 + status] {
                assert!(SuccessfulCalibration::from_readback(100, status, checksum, None)
                    .unwrap_err().contains("not successful"));
            }
        }
        assert_eq!(CalibrationStatus::from_word(2).unwrap(), CalibrationStatus::WaitingForData);
        assert_eq!(CalibrationStatus::from_word(3).unwrap(), CalibrationStatus::OutOfRange);
        assert!(SuccessfulCalibration::from_readback(100, 0, 101, None)
            .unwrap_err().contains("unknown"));
        assert!(CalibrationStatus::from_word(u32::MAX).is_err());
    }

    #[test]
    fn resistance_and_checksum_limits_do_not_wrap() {
        assert_eq!(SuccessfulCalibration::new(0, None).unwrap().checksum(), 1);
        assert_eq!(SuccessfulCalibration::new(0xfffffe, None).unwrap().checksum(), 0xffffff);
        assert!(SuccessfulCalibration::new(0xffffff, None)
            .unwrap_err().contains("checksum"));
        assert!(SuccessfulCalibration::new(0x01000000, None)
            .unwrap_err().contains("resistance"));
        assert!(SuccessfulCalibration::new(u32::MAX, None).is_err());
        assert!(SuccessfulCalibration::from_readback(0xfffffe, 1, 0x01000000, None)
            .unwrap_err().contains("24-bit"));
    }

    #[test]
    fn named_relative_plan_preserves_success_order_and_checksum_last() {
        let plan = SuccessfulCalibration::new(0x012345, None).unwrap().relative_write_plan();
        assert_eq!(plan.iter().map(|p| (p.name, p.word_offset)).collect::<Vec<_>>(),
            vec![("CAL_R", 6), ("CAL_STATUS", 8), ("CAL_CHECKSUM", 9)]);
        assert_eq!(plan.iter().map(|p| p.payload).collect::<Vec<_>>(),
            vec![[0, 1, 0x23, 0x45], [0, 0, 0, 1], [0, 1, 0x23, 0x46]]);
        assert!(plan.iter().all(|p| p.algorithm_id == 0xcd && p.memory_type == 5));
    }

    #[test]
    fn explicit_negative_ambient_is_signed24_and_precedes_the_success_sequence() {
        let record = SuccessfulCalibration::new(0x1200, Some(-20)).unwrap();
        let plan = record.relative_write_plan();
        assert_eq!(record.ambient_degrees_c(), Some(-20));
        assert_eq!((plan[0].name, plan[0].word_offset, plan[0].payload),
            ("CAL_AMBIENT", 7, [0, 0xff, 0xff, 0xec]));
        assert_eq!(plan.last().unwrap().name, "CAL_CHECKSUM");
        assert_eq!(plan.len(), 4);
        let parsed = SuccessfulCalibration::from_control_payloads(
            plan[1].payload, plan[2].payload, plan[3].payload, Some(plan[0].payload),
        ).unwrap();
        assert_eq!(parsed, record);
        let minimum = SuccessfulCalibration::new(1, Some(SIGNED_DATA_MIN)).unwrap();
        assert_eq!(minimum.relative_write_plan()[0].payload, [0, 0x80, 0, 0]);
        assert!(SuccessfulCalibration::new(1, Some(SIGNED_DATA_MIN - 1)).is_err());
        assert!(SuccessfulCalibration::new(1, Some(SIGNED_DATA_MAX + 1)).is_err());
    }

    #[test]
    fn payload_parser_rejects_noncanonical_padding_and_wrong_checksum() {
        let valid = SuccessfulCalibration::from_control_payloads(
            [0, 0, 0x12, 0x34], [0, 0, 0, 1], [0, 0, 0x12, 0x35], None,
        ).unwrap();
        assert_eq!(valid.encoded_resistance(), 0x1234);
        let good = [[0, 0, 0x12, 0x34], [0, 0, 0, 1], [0, 0, 0x12, 0x35]];
        for index in 0..good.len() {
            let mut bad = good;
            bad[index][0] = 0xff;
            assert!(SuccessfulCalibration::from_control_payloads(bad[0], bad[1], bad[2], None)
                .unwrap_err().contains("padding"));
        }
        assert!(SuccessfulCalibration::from_control_payloads(good[0], good[1], good[2],
            Some([0xff, 0xff, 0xff, 0xec])).unwrap_err().contains("padding"));
        assert!(SuccessfulCalibration::from_control_payloads(good[0], good[1],
            [0, 0, 0x12, 0x36], None).unwrap_err().contains("mismatch"));
    }
}
