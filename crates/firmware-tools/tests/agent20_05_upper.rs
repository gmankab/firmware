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

#[path = "../src/devices/xiaomi_nabu/venus_agent20_05_record_metrics.rs"]
mod record_metrics;
#[test]
fn rolling_record_metrics_matches_optional_reference(){
    let bytes=record_metrics::fragment().expect("typed 64-bit metrics");
    assert_eq!(bytes.len(),record_metrics::END-record_metrics::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE") {
        let original=std::fs::read(reference).unwrap();
        let off=record_metrics::START+record_metrics::FILE_BASE;
        assert_eq!(&bytes[..],original.get(off..off+bytes.len()).unwrap());
    }
}

#[path = "../src/devices/xiaomi_nabu/venus_agent20_05_magnitude.rs"]
mod magnitude;
#[test]
fn magnitude_bucket_matches_optional_reference(){
    let b=magnitude::fragment().unwrap();assert_eq!(b.len(),magnitude::END-magnitude::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE") {
        let original=std::fs::read(reference).unwrap();
        let off=magnitude::START+magnitude::FILE_BASE;
        assert_eq!(&b[..],original.get(off..off+b.len()).unwrap());
    }
}

#[path="../src/devices/xiaomi_nabu/venus_agent20_05_alt_occupancy.rs"]
mod alt_occupancy;
#[test]
fn alternate_occupancy_matches_optional_reference(){
    let bytes=alt_occupancy::fragment().unwrap();
    assert_eq!(bytes.len(),alt_occupancy::END-alt_occupancy::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE") {
        let firmware=std::fs::read(reference).unwrap();
        let off=alt_occupancy::START+alt_occupancy::FILE_BASE;
        assert_eq!(&bytes[..],firmware.get(off..off+bytes.len()).unwrap());
    }
}

#[path="../src/devices/xiaomi_nabu/venus_agent20_05_occupancy_commit.rs"]
mod occupancy_commit;
#[test]
fn committed_occupancy_matches_optional_reference(){
    let bytes=occupancy_commit::fragment().unwrap();
    assert_eq!(bytes.len(),occupancy_commit::END-occupancy_commit::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let fw=std::fs::read(reference).unwrap();
        let off=occupancy_commit::START+occupancy_commit::FILE_BASE;
        assert_eq!(&bytes[..],fw.get(off..off+bytes.len()).unwrap());
    }
}

#[path="../src/devices/xiaomi_nabu/venus_agent20_05_record_append.rs"]
mod record_append;
#[test]fn record_append_matches_optional_reference(){
    let bytes=record_append::fragment().unwrap();
    assert_eq!(bytes.len(),record_append::END-record_append::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let fw=std::fs::read(reference).unwrap();let off=record_append::START+record_append::FILE_BASE;
        assert_eq!(&bytes[..],fw.get(off..off+bytes.len()).unwrap());
    }
}

#[path="../src/devices/xiaomi_nabu/venus_agent20_05_record_select.rs"]
mod record_select;
#[test]fn controller_record_selection_matches_optional_reference(){
    let b=record_select::fragment().unwrap();assert_eq!(b.len(),record_select::END-record_select::START);
    if let Some(ref_name)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let ref_image=std::fs::read(ref_name).unwrap();let off=record_select::START+record_select::FILE_BASE;
        assert_eq!(&b[..],ref_image.get(off..off+b.len()).unwrap());
    }
}

#[test]
fn merged_controller_interval_is_strictly_gapless(){
    let pieces: [(usize,usize,fn()->Result<Vec<u8>,String>);11] = [
        (quality::START,quality::END,quality::fragment),
        (record_metrics::START,record_metrics::END,record_metrics::fragment),
        (magnitude::START,magnitude::END,magnitude::fragment),
        (alt_occupancy::START,alt_occupancy::END,alt_occupancy::fragment),
        (occupancy_commit::START,occupancy_commit::END,occupancy_commit::fragment),
        (record_append::START,record_append::END,record_append::fragment),
        (record_select::START,record_select::END,record_select::fragment),
        (color_unpack::START,color_unpack::END,color_unpack::fragment),
        (controller_limits::START,controller_limits::END,controller_limits::fragment),
        (controller_limits_tail::START,controller_limits_tail::END,controller_limits_tail::fragment),
        (throughput_tiers::START,throughput_tiers::END,throughput_tiers::fragment),
    ];
    let mut combined=Vec::new();
    let mut cursor=0x710c0;
    for (start,end,emit) in pieces {
        assert_eq!(start,cursor,"noncontiguous controller function");
        let bytes=emit().unwrap();
        assert_eq!(bytes.len(),end-start,"incorrect emitted function size");
        combined.extend_from_slice(&bytes);cursor=end;
    }
    assert_eq!(cursor,0x72bc8);
    assert_eq!(combined.len(),6920);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let fw=std::fs::read(reference).unwrap();
        let file_offset=0x710c0+0x4000;
        assert_eq!(&combined[..],fw.get(file_offset..file_offset+combined.len()).unwrap());
    }
}

#[path="../src/devices/xiaomi_nabu/venus_agent20_05_controller_limits.rs"]
mod controller_limits;
#[test]fn controller_limits_prefix_matches_optional_reference(){
    let p=controller_limits::fragment().unwrap();
    assert_eq!(p.len(),controller_limits::END-controller_limits::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE") {
        let data=std::fs::read(reference).unwrap();
        let offset=controller_limits::START+controller_limits::FILE_BASE;
        assert_eq!(&p[..],data.get(offset..offset+p.len()).unwrap());
    }
}

#[path="../src/devices/xiaomi_nabu/venus_agent20_05_controller_limits_tail.rs"]
mod controller_limits_tail;
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_throughput_tiers.rs"]
mod throughput_tiers;
#[test]fn controller_limits_tail_matches_optional_reference(){
    let b=controller_limits_tail::fragment().unwrap();
    assert_eq!(b.len(),controller_limits_tail::END-controller_limits_tail::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let orig=std::fs::read(reference).unwrap();let off=controller_limits_tail::START+controller_limits_tail::FILE_BASE;
        assert_eq!(&b[..],orig.get(off..off+b.len()).unwrap());
    }
}

#[test] fn throughput_tiers_match_optional_original_venus() {
    let output=throughput_tiers::fragment().unwrap();
    assert_eq!(output.len(),throughput_tiers::END-throughput_tiers::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE") {
        let fw=std::fs::read(reference).unwrap();
        let off=throughput_tiers::START+throughput_tiers::FILE_BASE;
        assert_eq!(&output[..],fw.get(off..off+output.len()).unwrap());
    }
}
