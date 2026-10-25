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
