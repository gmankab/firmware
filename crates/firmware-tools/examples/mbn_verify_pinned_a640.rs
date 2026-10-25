// SPDX-License-Identifier: GPL-3.0-only
//! Verify exact A640 original-byte equivalence with *borrowed* OEM authentication.
//! This command emits no complete firmware, OEM payload or credentials.
//! Checking the original RSA-PSS chain separately is still required and does
//! not establish Qualcomm hardware trust.
use firmware_tools::mbn_source_slices::a640_source_plan;
use std::{env, error::Error, fs};

/// Separately pinned SHA-384 of original A640 reference revision whose SHA-256
/// is bb1b61bae977aab5e5a68bd88848f4479c8d156911d96c7de73f7195b674db06.
/// This public digest is not secret signing material.
const A640_ORIGINAL_SHA384: &str =
    "7fbd85387967cc51e7ffc1a4a42b68287745fba6f54fd3024fbc6879a5289f13d2a40cff4fa5168891a7d64316ce020b";

fn hex_value(ch: u8) -> Result<u8, String> {
    match ch { b'0'..=b'9' => Ok(ch - b'0'), b'a'..=b'f' => Ok(10 + ch - b'a'),
        _ => Err("invalid built-in A640 SHA-384 pin".into()) }
}

fn digest_pin() -> Result<[u8;48], String> {
    let mut digest = [0u8; 48];
    for (i, pair) in A640_ORIGINAL_SHA384.as_bytes().chunks_exact(2).enumerate() {
        digest[i] = (hex_value(pair[0])? << 4) | hex_value(pair[1])?;
    }
    Ok(digest)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 1 {
        return Err("usage: cargo run --example mbn_verify_pinned_a640 -- /private/a640_zap.mbn".into());
    }
    let original = fs::read(&args[0])?;
    let mut source = a640_source_plan()?;
    let receipt = source.admit_pinned_original_authentication(&original, &digest_pin()?)?;
    assert_eq!(source.build_complete()?, original);
    println!("{{\"image\":\"a640_zap.mbn\",\"file_bytes\":{},\"generated_bytes\":{},\"borrowed_oem_authentication_bytes\":{},\"external_auth_range\":[{},{}],\"byte_mismatches\":0,\"original_sha384_pinned\":true,\"segment_hashes_verified\":{},\"hardware_trust_verified\":{}}}",
        receipt.file_bytes, receipt.independently_generated_file_bytes,
        receipt.borrowed_original_authentication_bytes,
        receipt.borrowed_original_authentication_range.start,
        receipt.borrowed_original_authentication_range.end,
        receipt.elf_segment_hashes_verified, receipt.device_hardware_trust_verified);
    Ok(())
}
