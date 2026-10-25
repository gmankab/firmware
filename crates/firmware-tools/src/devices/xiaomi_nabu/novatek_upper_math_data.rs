// SPDX-License-Identifier: GPL-3.0-only
//! Mathematical construction of eight Novatek upper-ILM data tables.
//!
//! These formulas reproduce all 528 little-endian `u16` samples in the pinned
//! postmarketOS Nabu reference, without reading that image during emission.
//! They establish a reproducible construction, not the original vendor's
//! generation procedure or the physical purpose of any table. The adjacent
//! irregular calibration-like arrays remain outside this module's coverage.
//!
//! Four scalar tables contain 50 or 32 centered coordinates and symmetric
//! widths. Four others contain degree-indexed sine/cosine/tangent samples
//! scaled by 100, or integer squares. Two zero alignment bytes follow each
//! degree-indexed table. Instructions and metadata are not emitted here.
//!
//! Trigonometry uses a fixed Taylor evaluation on [0, pi/2], not a platform
//! libm implementation. An independent 85-digit Decimal calculation matched
//! every rounded sample. Its nearest rounding boundary is >0.003807 for
//! sine/cosine and >0.007923 for tangent, far above binary64 evaluation error.

pub const SOURCE_DATA_BYTES: usize = 1_056;
pub const ALIGNMENT_BYTES: usize = 8;
pub const GENERATED_BYTES: usize = SOURCE_DATA_BYTES + ALIGNMENT_BYTES;
pub const TABLE_COUNT: usize = 8;
pub const MAX_DEGREES: u32 = 90;
pub const TANGENT_RIGHT_ANGLE_SENTINEL: u16 = 10_000;

pub const COORDINATES_50_FILE: u32 = 0x195b0;
pub const WIDTHS_50_FILE: u32 = 0x19614;
pub const COORDINATES_32_FILE: u32 = 0x19678;
pub const WIDTHS_32_FILE: u32 = 0x196b8;
pub const SINE_FILE: u32 = 0x198dc;
pub const COSINE_FILE: u32 = 0x19994;
pub const TANGENT_FILE: u32 = 0x19a4c;
pub const SQUARES_FILE: u32 = 0x19b04;
pub const TABLES_END_FILE: u32 = 0x19bbc;

/// Nearest-integer centers of equally spaced cells. All arithmetic is integer;
/// ties round up. The table format stores coordinates in unsigned 16 bits.
pub fn centered_coordinates(extent: u32, count: u32) -> Result<Vec<u16>, String> {
    if extent == 0 || extent > u16::MAX as u32 || count == 0 || count > extent {
        return Err("coordinate extent must fit u16 and 1 <= count <= extent".into());
    }
    let extent = u64::from(extent);
    let count = u64::from(count);
    let denominator = 2 * count;
    (0..count)
        .map(|i| {
            // round(extent * (i + 1/2) / count), with an exact rational tie.
            let numerator = extent * (2 * i + 1) + count;
            u16::try_from(numerator / denominator)
                .map_err(|_| "rounded coordinate does not fit u16".into())
        })
        .collect()
}

/// Widths whose midpoint is each supplied center and whose cells tile extent.
/// A center is measured from zero; the next cell starts at the preceding end.
pub fn symmetric_widths(centers: &[u16], extent: u32) -> Result<Vec<u16>, String> {
    if centers.is_empty() || extent == 0 || extent > u16::MAX as u32 {
        return Err("symmetric widths require centers and a positive u16 extent".into());
    }
    let mut left = 0_u32;
    let mut widths = Vec::with_capacity(centers.len());
    for &center in centers {
        let half = u32::from(center)
            .checked_sub(left)
            .ok_or("a coordinate precedes its cell's left boundary")?;
        let width = 2 * half;
        if width == 0 || width > u16::MAX as u32 {
            return Err("a symmetric cell width must be positive and fit u16".into());
        }
        left = left
            .checked_add(width)
            .filter(|&end| end <= extent)
            .ok_or("symmetric cell extends beyond extent")?;
        widths.push(width as u16);
    }
    if left != extent {
        return Err("symmetric cells do not tile the requested extent".into());
    }
    Ok(widths)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrigonometricFunction {
    Sine,
    Cosine,
    Tangent,
}

/// Fixed-order sine/cosine evaluation for the small closed interval used here.
/// The exact endpoints avoid tiny residuals at pi/2. Twenty correction terms
/// give a negligible Taylor remainder compared with binary64 rounding error.
fn sine_cosine(degrees: u32) -> Result<(f64, f64), String> {
    if degrees > MAX_DEGREES {
        return Err("angle is outside the inclusive 0..90 degree table".into());
    }
    if degrees == 0 {
        return Ok((0.0, 1.0));
    }
    if degrees == MAX_DEGREES {
        return Ok((1.0, 0.0));
    }
    let x = f64::from(degrees) * std::f64::consts::PI / 180.0;
    let x_squared = x * x;
    let mut sine_term = x;
    let mut cosine_term = 1.0;
    let mut sine = sine_term;
    let mut cosine = cosine_term;
    for k in 1..=20_u32 {
        sine_term = -sine_term * x_squared / f64::from((2 * k) * (2 * k + 1));
        cosine_term = -cosine_term * x_squared / f64::from((2 * k - 1) * (2 * k));
        sine += sine_term;
        cosine += cosine_term;
    }
    Ok((sine, cosine))
}

/// Construct one scaled trigonometric sample. The last tangent entry is an
/// explicit finite endpoint sentinel; it is not a claim that tan(90) is finite.
pub fn trigonometric_sample(
    function: TrigonometricFunction,
    degrees: u32,
) -> Result<u16, String> {
    let (sine, cosine) = sine_cosine(degrees)?;
    if function == TrigonometricFunction::Tangent && degrees == MAX_DEGREES {
        return Ok(TANGENT_RIGHT_ANGLE_SENTINEL);
    }
    let value = 100.0
        * match function {
            TrigonometricFunction::Sine => sine,
            TrigonometricFunction::Cosine => cosine,
            TrigonometricFunction::Tangent => sine / cosine,
        };
    if !value.is_finite() || value < 0.0 || value > f64::from(u16::MAX) {
        return Err("scaled trigonometric sample cannot be represented as u16".into());
    }
    // Every value is nonnegative, so round() implements nearest with ties up.
    Ok(value.round() as u16)
}

pub fn squared_sample(index: u32) -> Result<u16, String> {
    if index > MAX_DEGREES {
        return Err("square index is outside the inclusive 0..90 table".into());
    }
    Ok((index * index) as u16)
}

fn serialize_samples(samples: &[u16], align_to_four: bool) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 2 + 2);
    for &sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    if align_to_four {
        while bytes.len() % 4 != 0 {
            bytes.push(0);
        }
    }
    bytes
}

/// Emit eight disjoint/adjacent fragments with their absolute firmware offsets.
/// Holes between fragments are deliberately absent from the result.
pub fn emit() -> Result<Vec<(u32, Vec<u8>)>, String> {
    let centers_50 = centered_coordinates(2_560, 50)?;
    let widths_50 = symmetric_widths(&centers_50, 2_560)?;
    let centers_32 = centered_coordinates(1_600, 32)?;
    let widths_32 = symmetric_widths(&centers_32, 1_600)?;
    let mut fragments = vec![
        (COORDINATES_50_FILE, serialize_samples(&centers_50, false)),
        (WIDTHS_50_FILE, serialize_samples(&widths_50, false)),
        (COORDINATES_32_FILE, serialize_samples(&centers_32, false)),
        (WIDTHS_32_FILE, serialize_samples(&widths_32, false)),
    ];
    for (file_offset, function) in [
        (SINE_FILE, TrigonometricFunction::Sine),
        (COSINE_FILE, TrigonometricFunction::Cosine),
        (TANGENT_FILE, TrigonometricFunction::Tangent),
    ] {
        let values: Result<Vec<u16>, String> = (0..=MAX_DEGREES)
            .map(|degrees| trigonometric_sample(function, degrees))
            .collect();
        fragments.push((file_offset, serialize_samples(&values?, true)));
    }
    let squares: Result<Vec<u16>, String> = (0..=MAX_DEGREES).map(squared_sample).collect();
    fragments.push((SQUARES_FILE, serialize_samples(&squares?, true)));
    let emitted: usize = fragments.iter().map(|(_, bytes)| bytes.len()).sum();
    if fragments.len() != TABLE_COUNT || emitted != GENERATED_BYTES {
        return Err("mathematical data byte accounting changed".into());
    }
    Ok(fragments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centered_cells_tile_the_two_extents() {
        for (extent, count, first, last) in [(2_560, 50, 26, 2_534), (1_600, 32, 25, 1_575)] {
            let centers = centered_coordinates(extent, count).unwrap();
            let widths = symmetric_widths(&centers, extent).unwrap();
            assert_eq!(centers.len(), count as usize);
            assert_eq!((centers[0], *centers.last().unwrap()), (first, last));
            assert!(centers.windows(2).all(|w| w[0] < w[1]));
            assert_eq!(widths.iter().map(|&w| u32::from(w)).sum::<u32>(), extent);
            let mut left = 0_u32;
            for (&center, width) in centers.iter().zip(widths) {
                assert_eq!(2 * u32::from(center), 2 * left + u32::from(width));
                left += u32::from(width);
            }
        }
    }

    #[test]
    fn sample_endpoints_and_monotonicity() {
        use TrigonometricFunction::*;
        assert_eq!(trigonometric_sample(Sine, 0).unwrap(), 0);
        assert_eq!(trigonometric_sample(Sine, 90).unwrap(), 100);
        assert_eq!(trigonometric_sample(Cosine, 0).unwrap(), 100);
        assert_eq!(trigonometric_sample(Cosine, 90).unwrap(), 0);
        assert_eq!(trigonometric_sample(Tangent, 0).unwrap(), 0);
        assert_eq!(trigonometric_sample(Tangent, 45).unwrap(), 100);
        assert_eq!(trigonometric_sample(Tangent, 90).unwrap(), 10_000);
        for function in [Sine, Cosine, Tangent] {
            let values: Vec<_> = (0..=90)
                .map(|i| trigonometric_sample(function, i).unwrap())
                .collect();
            assert!(values.windows(2).all(|w| if function == Cosine { w[0] >= w[1] } else { w[0] <= w[1] }));
        }
        for degrees in 0..=90 {
            assert_eq!(trigonometric_sample(Sine, degrees).unwrap(), trigonometric_sample(Cosine, 90 - degrees).unwrap());
            assert_eq!(squared_sample(degrees).unwrap(), (degrees * degrees) as u16);
        }
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        for (extent, count) in [(0, 50), (2_560, 0), (32, 50), (65_536, 50)] {
            assert!(centered_coordinates(extent, count).is_err());
        }
        assert!(symmetric_widths(&[], 100).is_err());
        assert!(symmetric_widths(&[25, 20], 100).is_err());
        assert!(symmetric_widths(&[25], 100).is_err());
        assert!(symmetric_widths(&[0], 100).is_err());
        assert!(symmetric_widths(&[60], 100).is_err());
        assert!(trigonometric_sample(TrigonometricFunction::Sine, 91).is_err());
        assert!(trigonometric_sample(TrigonometricFunction::Tangent, u32::MAX).is_err());
        assert!(squared_sample(91).is_err());
    }

    #[test]
    fn fragment_ranges_and_provenance_counts() {
        let fragments = emit().unwrap();
        assert_eq!(fragments.len(), TABLE_COUNT);
        assert_eq!(fragments.iter().map(|(_, b)| b.len()).sum::<usize>(), GENERATED_BYTES);
        for pair in fragments.windows(2) {
            assert!(pair[0].0 + pair[0].1.len() as u32 <= pair[1].0);
        }
        assert_eq!(fragments[3].0 + fragments[3].1.len() as u32, 0x196f8);
        assert_eq!(fragments[4].0, 0x198dc);
        assert_eq!(fragments[7].0 + fragments[7].1.len() as u32, TABLES_END_FILE);
        for (_, bytes) in &fragments[4..] {
            assert_eq!(bytes.len(), 184);
            assert_eq!(&bytes[182..], &[0, 0]);
        }
        assert_eq!(SOURCE_DATA_BYTES, (50 * 2 + 32 * 2 + 91 * 4) * 2);
        assert_eq!(ALIGNMENT_BYTES, 4 * 2);
    }
}
