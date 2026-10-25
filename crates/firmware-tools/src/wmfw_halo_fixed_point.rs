// SPDX-License-Identifier: GPL-3.0-only
//! Checked numeric values for HALO's unpacked 24-bit control registers.
//!
//! The Cirrus Linux helper `cs_dsp_chunk_write` masks a DSP data word to
//! 24 bits before writing its big-endian 32-bit register representation:
//! https://codebrowser.dev/linux/linux/drivers/firmware/cirrus/cs_dsp.c.html
//! This module chooses that canonical zero-padded representation, followed
//! by the 24-bit two's-complement value. Rejecting a nonzero high byte is an
//! API normalization policy, not a claim that hardware rejects sign-extended
//! writes. Some Linux host write paths sign-extend negative ambient values.
//!
//! This module supplies arithmetic and register serialization only. A caller
//! must establish that a particular control is signed and supply its actual
//! fractional-bit count. WMFW control names and byte lengths do not establish
//! those facts. In particular, this module neither assumes that HALO data is
//! universally Q23 nor interprets the packed XM/YM file transport. It adds no
//! independently reconstructed firmware bytes.

/// A signed integer whose exact bit representation fits one DSP data word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Signed24(i32);

impl Signed24 {
    pub const MIN: i32 = -(1 << 23);
    pub const MAX: i32 = (1 << 23) - 1;
    pub const MASK: u32 = (1 << 24) - 1;

    /// Reject overflow instead of wrapping or saturating a control value.
    pub fn new(value: i64) -> Result<Self, String> {
        if value < i64::from(Self::MIN) || value > i64::from(Self::MAX) {
            return Err("signed HALO data value exceeds 24-bit range".into());
        }
        Ok(Self(value as i32))
    }

    /// Decode a 24-bit two's-complement bit pattern, rejecting wider input.
    pub fn from_bits(bits: u32) -> Result<Self, String> {
        if bits & !Self::MASK != 0 {
            return Err("HALO data bit pattern exceeds 24 bits".into());
        }
        let value = if bits & (1 << 23) != 0 {
            bits as i32 - (1 << 24)
        } else {
            bits as i32
        };
        Ok(Self(value))
    }

    pub fn value(self) -> i32 {
        self.0
    }

    pub fn bits(self) -> u32 {
        self.0 as u32 & Self::MASK
    }

    /// One unpacked-24 register: zero padding byte plus 24 value bits.
    pub fn to_unpacked_be32(self) -> [u8; 4] {
        self.bits().to_be_bytes()
    }

    /// Accept the canonical zero-padded representation, never sign extension.
    pub fn from_unpacked_be32(bytes: [u8; 4]) -> Result<Self, String> {
        if bytes[0] != 0 {
            return Err("nonzero padding byte in unpacked HALO 24-bit register".into());
        }
        Self::from_bits(u32::from_be_bytes(bytes))
    }
}

/// Explicit quantization policy; no implicit floating-point rounding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rounding {
    /// Discard the fractional magnitude, for either sign.
    TowardZero,
    /// Round to the closest integer; exact halves select the even integer.
    NearestTiesEven,
}

/// A signed 24-bit word with a caller-established binary-point position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedPoint24 {
    raw: Signed24,
    fractional_bits: u8,
}

impl FixedPoint24 {
    /// `fractional_bits` is explicit and must lie in 0..=23.
    pub fn from_raw(raw: Signed24, fractional_bits: u8) -> Result<Self, String> {
        if fractional_bits > 23 {
            return Err("HALO fractional-bit count must be between 0 and 23".into());
        }
        Ok(Self { raw, fractional_bits })
    }

    /// Quantize `numerator / denominator` using exact integer arithmetic.
    ///
    /// The denominator must be positive. An i128 intermediate represents
    /// every i64 numerator scaled by at most 2^23, including i64::MIN,
    /// without loss of precision. Range checking applies to the final
    /// quantized word; an out-of-range result is rejected, not saturated.
    pub fn from_rational(
        numerator: i64,
        denominator: u64,
        fractional_bits: u8,
        rounding: Rounding,
    ) -> Result<Self, String> {
        if fractional_bits > 23 {
            return Err("HALO fractional-bit count must be between 0 and 23".into());
        }
        if denominator == 0 {
            return Err("HALO fixed-point denominator must be positive".into());
        }
        let scaled = i128::from(numerator)
            .checked_mul(1i128 << fractional_bits)
            .ok_or("HALO fixed-point scaling overflow")?;
        let divisor = i128::from(denominator);
        let mut quantized = scaled / divisor;
        if rounding == Rounding::NearestTiesEven {
            // The remainder has magnitude below u64::MAX, so doubling it
            // and taking its absolute value cannot overflow i128.
            let twice_remainder = (scaled % divisor).abs() * 2;
            if twice_remainder > divisor
                || (twice_remainder == divisor && quantized % 2 != 0)
            {
                let direction = if scaled < 0 { -1 } else { 1 };
                quantized = quantized
                    .checked_add(direction)
                    .ok_or("HALO fixed-point rounding overflow")?;
            }
        }
        if quantized < i128::from(Signed24::MIN)
            || quantized > i128::from(Signed24::MAX)
        {
            return Err("quantized HALO fixed-point value exceeds 24-bit range".into());
        }
        Self::from_raw(Signed24::new(quantized as i64)?, fractional_bits)
    }

    pub fn raw(self) -> Signed24 {
        self.raw
    }

    pub fn fractional_bits(self) -> u8 {
        self.fractional_bits
    }

    /// Exact, possibly unreduced ratio; its denominator preserves the scale.
    pub fn as_ratio(self) -> (i32, u32) {
        (self.raw.value(), 1u32 << self.fractional_bits)
    }

    pub fn to_unpacked_be32(self) -> [u8; 4] {
        self.raw.to_unpacked_be32()
    }

    pub fn from_unpacked_be32(
        bytes: [u8; 4],
        fractional_bits: u8,
    ) -> Result<Self, String> {
        Self::from_raw(Signed24::from_unpacked_be32(bytes)?, fractional_bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_word_limits_are_exact_and_do_not_wrap() {
        assert_eq!(Signed24::new(-8_388_608).unwrap().bits(), 0x80_0000);
        assert_eq!(Signed24::new(8_388_607).unwrap().bits(), 0x7f_ffff);
        assert_eq!(Signed24::from_bits(0xff_ffff).unwrap().value(), -1);
        assert_eq!(Signed24::from_bits(0x80_0000).unwrap().value(), -8_388_608);
        assert!(Signed24::new(-8_388_609).is_err());
        assert!(Signed24::new(8_388_608).is_err());
        assert!(Signed24::new(i64::MIN).is_err());
        assert!(Signed24::from_bits(0x0100_0000).is_err());
    }

    #[test]
    fn register_values_use_zero_padding_even_for_negative_numbers() {
        let fixtures = [
            (-8_388_608, [0x00, 0x80, 0x00, 0x00]),
            (-1, [0x00, 0xff, 0xff, 0xff]),
            (0, [0x00, 0x00, 0x00, 0x00]),
            (1, [0x00, 0x00, 0x00, 0x01]),
            (8_388_607, [0x00, 0x7f, 0xff, 0xff]),
        ];
        for (value, bytes) in fixtures {
            assert_eq!(Signed24::new(value).unwrap().to_unpacked_be32(), bytes);
            assert_eq!(Signed24::from_unpacked_be32(bytes).unwrap().value(), value as i32);
        }
        assert!(Signed24::from_unpacked_be32([0xff, 0xff, 0xff, 0xff])
            .unwrap_err().contains("padding"));
        assert!(Signed24::from_unpacked_be32([1, 0, 0, 0]).is_err());
    }

    #[test]
    fn binary_point_is_explicit_and_changes_positive_unity_limit() {
        let positive = FixedPoint24::from_rational(1, 1, 22, Rounding::TowardZero).unwrap();
        assert_eq!(positive.as_ratio(), (4_194_304, 4_194_304));
        assert!(FixedPoint24::from_rational(1, 1, 23, Rounding::TowardZero).is_err());
        let negative = FixedPoint24::from_rational(-1, 1, 23, Rounding::TowardZero).unwrap();
        assert_eq!(negative.as_ratio(), (-8_388_608, 8_388_608));
        assert_eq!(negative.to_unpacked_be32(), [0, 0x80, 0, 0]);
        assert_eq!(negative.fractional_bits(), 23);
        assert_eq!(FixedPoint24::from_unpacked_be32([0, 0x80, 0, 0], 23).unwrap(), negative);
        assert!(FixedPoint24::from_raw(Signed24::new(0).unwrap(), 24).is_err());
        assert!(FixedPoint24::from_rational(0, 1, 255, Rounding::TowardZero).is_err());
    }

    #[test]
    fn truncation_moves_toward_zero_for_both_signs() {
        for (numerator, expected) in [(1, 2), (-1, -2)] {
            let value = FixedPoint24::from_rational(numerator, 3, 3, Rounding::TowardZero).unwrap();
            assert_eq!(value.as_ratio(), (expected, 8));
        }
        assert_eq!(FixedPoint24::from_rational(-1, 2, 0, Rounding::TowardZero)
            .unwrap().raw().value(), 0);
    }

    #[test]
    fn nearest_ties_even_is_symmetric_and_rounds_odd_halves() {
        for (numerator, expected) in [(1, 0), (3, 2), (5, 2), (7, 4)] {
            for sign in [-1, 1] {
                let value = FixedPoint24::from_rational(
                    sign * numerator, 2, 0, Rounding::NearestTiesEven,
                ).unwrap();
                assert_eq!(value.raw().value(), (sign * expected) as i32);
            }
        }
        for sign in [-1, 1] {
            let value = FixedPoint24::from_rational(sign, 3, 3, Rounding::NearestTiesEven).unwrap();
            assert_eq!(value.raw().value(), (sign * 3) as i32);
        }
    }

    #[test]
    fn denominator_and_post_rounding_overflow_are_checked() {
        assert!(FixedPoint24::from_rational(1, 0, 0, Rounding::TowardZero)
            .unwrap_err().contains("denominator"));
        // MAX + 1/2 ties toward MAX+1, because MAX is odd.
        let numerator = i64::from(Signed24::MAX) * 2 + 1;
        assert!(FixedPoint24::from_rational(numerator, 2, 0, Rounding::NearestTiesEven)
            .unwrap_err().contains("24-bit range"));
        assert_eq!(FixedPoint24::from_rational(numerator, 2, 0, Rounding::TowardZero)
            .unwrap().raw().value(), Signed24::MAX);
        assert!(FixedPoint24::from_rational(i64::from(Signed24::MIN) - 1, 1, 0,
            Rounding::TowardZero).is_err());
    }

    #[test]
    fn largest_input_types_do_not_lose_precision_or_panic() {
        assert!(FixedPoint24::from_rational(i64::MAX, 1, 23, Rounding::TowardZero).is_err());
        assert!(FixedPoint24::from_rational(i64::MIN, 1, 23, Rounding::NearestTiesEven).is_err());
        let value = FixedPoint24::from_rational(i64::MIN, u64::MAX, 23,
            Rounding::NearestTiesEven).unwrap();
        assert_eq!(value.raw().value(), -4_194_304);
        let exact = FixedPoint24::from_rational(i64::MIN, 1u64 << 63, 23,
            Rounding::TowardZero).unwrap();
        assert_eq!(exact.raw().value(), Signed24::MIN);
    }
}
