// SPDX-License-Identifier: GPL-3.0-only
// Standalone optional-reference tests: no OEM firmware is committed.
#[path = "../src/devices/xiaomi_nabu/venus_agent20_05_upper.rs"]
mod upper;

#[test]
fn scaled_dimensions_rebuild_and_optional_reference() {
    let bytes=upper::fragment().expect("typed A32 reconstruction");
    assert_eq!(bytes.len(),upper::VERIFIED_END-upper::VERIFIED_START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE") {
        let reference=std::fs::read(&reference).expect("read private supplied Venus reference");
        let offset=upper::VERIFIED_START+upper::FILE_MAPPING_DELTA;
        let expected=reference.get(offset..offset+bytes.len()).expect("reference too small");
        if bytes != expected {
            let mismatch=bytes.iter().zip(expected).position(|(a,b)|a!=b).unwrap();
            panic!("agent05 first differing runtime 0x{:x}, file 0x{:x}",upper::VERIFIED_START+mismatch,offset+mismatch)
        }
    }
}

#[path = "../src/devices/xiaomi_nabu/venus_agent20_05_control_flags.rs"]
mod control_flags;

#[test]
fn control_flags_fragment_matches_optional_reference(){
    let bytes=control_flags::fragment().expect("typed control flags");
    assert_eq!(bytes.len(),control_flags::END-control_flags::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let original=std::fs::read(reference).expect("read original");
        let off=control_flags::START+control_flags::FILE_BASE;
        assert_eq!(&bytes[..],original.get(off..off+bytes.len()).expect("short original"));
    }
}

#[path = "../src/devices/xiaomi_nabu/venus_agent20_05_color_unpack.rs"]
mod color_unpack;
#[test]
fn color_unpack_matches_optional_reference(){
    let bytes=color_unpack::fragment().expect("typed unpack routine");
    assert_eq!(bytes.len(),color_unpack::END-color_unpack::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let orig=std::fs::read(reference).unwrap();
        let offset=color_unpack::START+color_unpack::FILE_BASE;
        assert_eq!(&bytes[..],orig.get(offset..offset+bytes.len()).unwrap());
    }
}

#[path = "../src/devices/xiaomi_nabu/venus_agent20_05_quality.rs"]
mod quality;
#[test]
fn quality_tier_and_allocators_match_optional_reference(){
    let bytes=quality::fragment().expect("quality-tier and allocation typed A32");
    assert_eq!(bytes.len(),quality::END-quality::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let original=std::fs::read(reference).expect("read original Venus");
        let off=quality::START+quality::FILE_BASE;
        assert_eq!(&bytes[..],original.get(off..off+bytes.len()).expect("short original"));
    }
}
