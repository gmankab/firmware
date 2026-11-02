// SPDX-License-Identifier: GPL-3.0-only
//! Independently generated Gaussian coefficient tables for the pinned Nabu DLM.
//!
//! Each table contains 81 little-endian u16 coefficients, ordered x=80..0:
//! `round(400000 / (sigma * sqrt(2*pi)) * exp(-x*x/(2*sigma*sigma)))`.
//! Sigma is supplied in exact integer tenths, not as opaque coefficient arrays.
//! The mathematical identity matches all 1,458 coefficients at the declared
//! file offsets. This establishes a compact source description of the data;
//! the controller's use of the tables and original generation code remain
//! inferred, not recovered executable semantics.
//!
//! Emitted coefficients use Q48 fixed-point and integer Taylor terms, without
//! libm, so the output is deterministic across supported Rust platforms.
//! The f64 helper below is retained *only* as a diagnostic for comparison.
//! All parameter tuples must additionally be checked against the pinned bytes.

pub const GAUSSIAN_HALF_WIDTH: usize = 80;
pub const GAUSSIAN_SAMPLE_COUNT: usize = GAUSSIAN_HALF_WIDTH + 1;
pub const GAUSSIAN_KERNEL_BYTES: usize = 2 * GAUSSIAN_SAMPLE_COUNT;
pub const GAUSSIAN_AREA_SCALE: f64 = 400_000.0;

/// Generator parameters inferred from, and separately compared with, the exact
/// postmarketOS SHA-256 8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5.
/// These are semantic width parameters, not copied sample coefficients.
pub const KERNEL_PROFILES: [(usize, u16); 18] = [
    (0x22488, 284),
    (0x2252a, 260),
    (0x225cc, 272),
    (0x2266e, 284),
    (0x2d7f4, 256),
    (0x2d896, 256),
    (0x2d938, 256),
    (0x2d9da, 260),
    (0x2da7c, 256),
    (0x2db1e, 268),
    (0x2dbc0, 252),
    (0x2dc62, 256),
    (0x2dd04, 252),
    (0x2dda6, 252),
    (0x2de48, 252),
    (0x2deea, 252),
    (0x2df98, 400),
    (0x2e03a, 800),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GaussianFragment {
    pub file_start: usize,
    pub bytes: Vec<u8>,
    pub sigma_tenths: u16,
    pub evidence: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoundingAnalysis {
    pub sigma_tenths: u16,
    pub sample_count: usize,
    pub minimum_half_integer_distance: f64,
    pub closest_x: usize,
    pub closest_unrounded_value: f64,
}

fn sigma(sigma_tenths: u16) -> Result<f64, String> {
    if sigma_tenths == 0 {
        return Err("Gaussian sigma must be positive".into());
    }
    Ok(f64::from(sigma_tenths) / 10.0)
}

fn scaled_density(sigma: f64, x: i32) -> f64 {
    let x = f64::from(x);
    GAUSSIAN_AREA_SCALE / (sigma * (2.0 * std::f64::consts::PI).sqrt())
        * (-x * x / (2.0 * sigma * sigma)).exp()
}

fn checked_coefficient(sigma: f64, x: usize) -> Result<(f64, u16), String> {
    if x > GAUSSIAN_HALF_WIDTH {
        return Err(format!("Gaussian sample x={x} exceeds fixed half-width {GAUSSIAN_HALF_WIDTH}"));
    }
    let value = scaled_density(sigma, x as i32);
    let rounded = value.round();
    if !value.is_finite() || rounded < 0.0 || rounded > f64::from(u16::MAX) {
        return Err(format!("Gaussian coefficient at x={x} is not representable as u16"));
    }
    Ok((value, rounded as u16))
}

// The rounded 48-bit fixed-point square root of 2π.
// sqrt(2*pi)*2^48 = 705553135224033. ... (rounded half up).
const Q48: u128 = 1u128 << 48;
const SQRT_TWO_PI_Q48: u128 = 705_553_135_224_033;

/// Compute exp(-z) for z in [0,16], using exp(z)'s strictly positive
/// Taylor series followed by reciprocal Q48 division. Stopping once a term
/// truncates to zero avoids cancellation. For z >= 16 any u16-bounded
/// Gaussian peak multiplied by exp(-z) rounds to zero.
fn exp_neg_q48(z: u128) -> Result<u128, String> {
    if z >= 16 * Q48 { return Ok(0); }
    let mut term = Q48;
    let mut total = Q48;
    for n in 1..=128u128 {
        term = term.checked_mul(z).ok_or("Gaussian series overflow")? / (Q48 * n);
        if term == 0 { break; }
        total = total.checked_add(term).ok_or("Gaussian series overflow")?;
    }
    Ok(Q48.checked_mul(Q48).ok_or("Gaussian reciprocal overflow")? / total)
}

/// Pure integer source specification, rounded to the nearest positive u16.
/// Sigma uses exact integer tenths and x is measured in whole sample slots.
fn fixed_coefficient(sigma_tenths:u16, x:usize)->Result<u16,String>{
    if sigma_tenths == 0 {return Err("Gaussian sigma must be positive".into());}
    if x > GAUSSIAN_HALF_WIDTH {return Err("Gaussian sample outside half-width".into());}
    let sigma_u = sigma_tenths as u128;
    let peak_num = 4_000_000u128.checked_mul(Q48*Q48)
        .ok_or("Gaussian amplitude overflow")?;
    let peak_q48 = peak_num / (sigma_u * SQRT_TWO_PI_Q48);
    if peak_q48 + Q48/2 > u16::MAX as u128 * Q48 + Q48 {
        return Err("Gaussian coefficient is not representable as u16".into());
    }
    let z = 50u128 * (x as u128).pow(2) * Q48 / sigma_u.pow(2);
    let decayed = peak_q48.checked_mul(exp_neg_q48(z)?)
        .ok_or("Gaussian density overflow")? / Q48;
    let rounded = (decayed + Q48/2) / Q48;
    u16::try_from(rounded).map_err(|_|"Gaussian coefficient is not representable as u16".into())
}

/// Generate exactly 81 coefficients using only deterministic integer math.
/// Output has the same u16 little-endian, x=80..0 layout as the pinned file.
pub fn gaussian_half_kernel(sigma_tenths: u16) -> Result<Vec<u8>, String> {
    fixed_coefficient(sigma_tenths, 0)?;
    let mut bytes = Vec::with_capacity(GAUSSIAN_KERNEL_BYTES);
    for x in (0..GAUSSIAN_SAMPLE_COUNT).rev() {
        bytes.extend_from_slice(&fixed_coefficient(sigma_tenths, x)?.to_le_bytes());
    }
    Ok(bytes)
}

/// Inspect all samples' distance from the nearest half-integer rounding
/// boundary. This is a diagnostic, not a guarantee about every platform's exp.
pub fn rounding_analysis(sigma_tenths: u16) -> Result<RoundingAnalysis, String> {
    let sigma = sigma(sigma_tenths)?;
    checked_coefficient(sigma, 0)?;
    let mut analysis = RoundingAnalysis {
        sigma_tenths,
        sample_count: GAUSSIAN_SAMPLE_COUNT,
        minimum_half_integer_distance: f64::INFINITY,
        closest_x: 0,
        closest_unrounded_value: 0.0,
    };
    for x in 0..GAUSSIAN_SAMPLE_COUNT {
        let (value, _) = checked_coefficient(sigma, x)?;
        let distance = (value.fract() - 0.5).abs();
        if distance < analysis.minimum_half_integer_distance {
            analysis.minimum_half_integer_distance = distance;
            analysis.closest_x = x;
            analysis.closest_unrounded_value = value;
        }
    }
    Ok(analysis)
}

fn validate_fragment_layout(start: usize, length: usize) -> Result<usize, String> {
    if length != GAUSSIAN_KERNEL_BYTES {
        return Err(format!("Gaussian table must contain exactly {GAUSSIAN_KERNEL_BYTES} bytes"));
    }
    let end = start.checked_add(length).ok_or("Gaussian fragment address overflow")?;
    if start < 0x22000 || end > 0x2e190 || start % 2 != 0 {
        return Err("Gaussian fragment outside aligned owned DLM range".into());
    }
    Ok(end)
}

/// Emit the 18 verified parameterized kernels without reading the reference.
/// Byte equality must be verified separately before accepting provenance claims.
pub fn fragments() -> Result<Vec<GaussianFragment>, String> {
    let mut result = Vec::with_capacity(KERNEL_PROFILES.len());
    let mut previous_end = 0x22000;
    for (file_start, sigma_tenths) in KERNEL_PROFILES {
        let bytes = gaussian_half_kernel(sigma_tenths)?;
        let end = validate_fragment_layout(file_start, bytes.len())?;
        if file_start < previous_end {
            return Err("overlapping Gaussian fragment profiles".into());
        }
        previous_end = end;
        result.push(GaussianFragment {
            file_start,
            bytes,
            sigma_tenths,
            evidence: "Gaussian density scaled by 400000; integer-tenths sigma; x=80..0; rounded u16",
        });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coefficients(sigma_tenths: u16) -> Vec<u16> {
        gaussian_half_kernel(sigma_tenths).unwrap().chunks_exact(2)
            .map(|p| u16::from_le_bytes([p[0], p[1]])).collect()
    }

    #[test]
    fn fixed_width_half_kernel_has_analytic_center_and_tail() {
        let narrow = coefficients(400);
        let broad = coefficients(800);
        assert_eq!(narrow.len(), 81);
        assert_eq!(broad.len(), 81);
        // Independently evaluated normal-density values at x=80 and x=0.
        assert_eq!((narrow[0], narrow[80]), (540, 3989));
        assert_eq!((broad[0], broad[80]), (1210, 1995));
    }

    #[test]
    fn even_density_increases_monotonically_toward_its_center() {
        for sigma_tenths in [252, 284, 400, 800] {
            let values = coefficients(sigma_tenths);
            assert!(values.windows(2).all(|p| p[0] <= p[1]));
            let s = sigma(sigma_tenths).unwrap();
            for x in 1..=80 {
                assert_eq!(scaled_density(s, -x), scaled_density(s, x));
                assert!(scaled_density(s, x) < scaled_density(s, 0));
            }
        }
    }

    #[test]
    fn wider_sigma_lowers_peak_and_increases_truncated_variance() {
        fn variance(values: &[u16]) -> f64 {
            let center = f64::from(values[80]);
            let mass = center + 2.0 * values[..80].iter().map(|v| f64::from(*v)).sum::<f64>();
            let moment = 2.0 * values[..80].iter().enumerate()
                .map(|(i, value)| ((80 - i) * (80 - i)) as f64 * f64::from(*value))
                .sum::<f64>();
            moment / mass
        }
        let narrow = coefficients(252);
        let medium = coefficients(400);
        let broad = coefficients(800);
        assert!(narrow[80] > medium[80] && medium[80] > broad[80]);
        assert!(variance(&narrow) < variance(&medium));
        assert!(variance(&medium) < variance(&broad));
    }

    #[test]
    fn invalid_sigma_overflow_and_sample_sizes_are_rejected() {
        assert!(gaussian_half_kernel(0).unwrap_err().contains("positive"));
        for sigma_tenths in [1, 24] {
            assert!(gaussian_half_kernel(sigma_tenths).unwrap_err().contains("u16"));
        }
        assert!(gaussian_half_kernel(25).is_ok());
        assert!(gaussian_half_kernel(u16::MAX).is_ok());
        assert!(rounding_analysis(0).is_err());
        assert!(checked_coefficient(40.0, 81).is_err());
        assert!(validate_fragment_layout(0x22488, 0).is_err());
        assert!(validate_fragment_layout(0x22488, 160).is_err());
        assert!(validate_fragment_layout(usize::MAX, GAUSSIAN_KERNEL_BYTES).is_err());
        assert!(validate_fragment_layout(0x22001, GAUSSIAN_KERNEL_BYTES).is_err());
        assert!(validate_fragment_layout(0x2e190, GAUSSIAN_KERNEL_BYTES).is_err());
    }

    #[test]
    fn integer_q48_matches_independent_f64_at_all_profiles() {
        for (_, width) in KERNEL_PROFILES {
            let sigma = sigma(width).unwrap();
            for x in 0..GAUSSIAN_SAMPLE_COUNT {
                assert_eq!(fixed_coefficient(width,x).unwrap(), checked_coefficient(sigma,x).unwrap().1,
                    "Gaussian width {width}, sample {x}");
            }
        }
    }

    #[test]
    fn profile_rounding_margins_are_measured_for_every_sample() {
        let mut closest = rounding_analysis(252).unwrap();
        for sigma_tenths in [252, 256, 260, 268, 272, 284, 400, 800] {
            let analysis = rounding_analysis(sigma_tenths).unwrap();
            assert_eq!(analysis.sample_count, 81);
            assert!(analysis.minimum_half_integer_distance > 0.0015);
            assert!(analysis.minimum_half_integer_distance <= 0.5);
            if analysis.minimum_half_integer_distance < closest.minimum_half_integer_distance {
                closest = analysis;
            }
        }
        assert_eq!((closest.sigma_tenths, closest.closest_x), (400, 72));
        assert!((closest.minimum_half_integer_distance - 0.0015830089415576845).abs() < 1e-10);
    }

    #[test]
    fn declared_fragments_are_disjoint_aligned_data_intervals() {
        let fragments = fragments().unwrap();
        assert_eq!(fragments.len(), 18);
        assert_eq!(fragments.iter().map(|f| f.bytes.len()).sum::<usize>(), 2916);
        for fragment in &fragments {
            validate_fragment_layout(fragment.file_start, fragment.bytes.len()).unwrap();
            assert!(!fragment.evidence.is_empty());
        }
        for pair in fragments.windows(2) {
            assert!(pair[0].file_start + pair[0].bytes.len() <= pair[1].file_start);
        }
    }
}
