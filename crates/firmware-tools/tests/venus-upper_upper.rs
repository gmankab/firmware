// SPDX-License-Identifier: GPL-3.0-only
// Standalone optional-reference tests: no OEM firmware is committed.
use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper as upper;

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

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_control_flags as control_flags;

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

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_color_unpack as color_unpack;
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

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_quality as quality;
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

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_record_metrics as record_metrics;
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

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_magnitude as magnitude;
#[test]
fn magnitude_bucket_matches_optional_reference(){
    let b=magnitude::fragment().unwrap();assert_eq!(b.len(),magnitude::END-magnitude::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE") {
        let original=std::fs::read(reference).unwrap();
        let off=magnitude::START+magnitude::FILE_BASE;
        assert_eq!(&b[..],original.get(off..off+b.len()).unwrap());
    }
}

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_alt_occupancy as alt_occupancy;
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

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_occupancy_commit as occupancy_commit;
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

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_record_append as record_append;
#[test]fn record_append_matches_optional_reference(){
    let bytes=record_append::fragment().unwrap();
    assert_eq!(bytes.len(),record_append::END-record_append::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let fw=std::fs::read(reference).unwrap();let off=record_append::START+record_append::FILE_BASE;
        assert_eq!(&bytes[..],fw.get(off..off+bytes.len()).unwrap());
    }
}

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_record_select as record_select;
#[test]fn controller_record_selection_matches_optional_reference(){
    let b=record_select::fragment().unwrap();assert_eq!(b.len(),record_select::END-record_select::START);
    if let Some(ref_name)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let ref_image=std::fs::read(ref_name).unwrap();let off=record_select::START+record_select::FILE_BASE;
        assert_eq!(&b[..],ref_image.get(off..off+b.len()).unwrap());
    }
}

#[test]
fn merged_controller_interval_is_strictly_gapless(){
    let pieces: [(usize,usize,fn()->Result<Vec<u8>,String>);15] = [
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
        (controller_alloc::START,controller_alloc::END,controller_alloc::fragment),
        (controller_run_prefix::START,controller_run_prefix::END,controller_run_prefix::fragment),
        (controller_run_tail::START,controller_run_tail::END,controller_run_tail::fragment),
        (controller_small_helpers::START,controller_small_helpers::END,controller_small_helpers::fragment),
    ];
    let mut combined=Vec::new();
    let mut cursor=0x710c0;
    for (start,end,emit) in pieces {
        assert_eq!(start,cursor,"noncontiguous controller function");
        let bytes=emit().unwrap();
        assert_eq!(bytes.len(),end-start,"incorrect emitted function size");
        combined.extend_from_slice(&bytes);cursor=end;
    }
    assert_eq!(cursor,0x73e10);
    assert_eq!(combined.len(),11600);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let fw=std::fs::read(reference).unwrap();
        let file_offset=0x710c0+0x4000;
        assert_eq!(&combined[..],fw.get(file_offset..file_offset+combined.len()).unwrap());
    }
}

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_controller_limits as controller_limits;
#[test]fn controller_limits_prefix_matches_optional_reference(){
    let p=controller_limits::fragment().unwrap();
    assert_eq!(p.len(),controller_limits::END-controller_limits::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE") {
        let data=std::fs::read(reference).unwrap();
        let offset=controller_limits::START+controller_limits::FILE_BASE;
        assert_eq!(&p[..],data.get(offset..offset+p.len()).unwrap());
    }
}

use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_controller_limits_tail as controller_limits_tail;
use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_throughput_tiers as throughput_tiers;
use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_controller_alloc as controller_alloc;
use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_controller_run_prefix as controller_run_prefix;
use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_controller_run_tail as controller_run_tail;
use firmware_tools::devices::xiaomi_nabu::venus_plan::venus_upper_controller_small_helpers as controller_small_helpers;

#[test]
fn controller_continuation_bounds_and_control_flow() {
    use firmware_tools::devices::xiaomi_nabu::venus_upper_controller_continuation as continuation;
    let bytes = continuation::fragment().unwrap();
    assert_eq!(continuation::START, 0x68000);
    assert_eq!(continuation::END, 0x68220);
    assert_eq!(continuation::FILE_MAPPING_DELTA, 0x4000);
    assert_eq!(bytes.len(), 544);
    assert_eq!(bytes.len() / 4, 136);
    assert_eq!(continuation::END - 0x680cc, 340);
    for (offset, expected) in [
        (0, arm32::branch(arm32::Condition::Al, 0x68000, 0x6800c, false).unwrap()),
        (28, arm32::branch_exchange_immediate(0x6801c, 0x83c4c).unwrap()),
        (48, arm32::branch(arm32::Condition::Cc, 0x68030, 0x67fd4, false).unwrap()),
        (52, arm32::branch(arm32::Condition::Al, 0x68034, 0x67f54, false).unwrap()),
        (84, arm32::branch(arm32::Condition::Lt, 0x68054, 0x67ee4, false).unwrap()),
        (104, arm32::branch_exchange_immediate(0x68068, 0x83c4c).unwrap()),
        (136, arm32::branch(arm32::Condition::Eq, 0x68088, 0x69b10, false).unwrap()),
        (196, arm32::branch(arm32::Condition::Gt, 0x680c4, 0x68084, false).unwrap()),
        (200, arm32::branch(arm32::Condition::Al, 0x680c8, 0x67ee4, false).unwrap()),
    ] {
        assert_eq!(&bytes[offset..offset + 4], &expected.to_le_bytes());
    }
}

#[test]
fn controller_continuation_weighted_loop_operands() {
    use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
    use firmware_tools::devices::xiaomi_nabu::venus_upper_controller_continuation as continuation;
    let bytes = continuation::fragment().unwrap();
    let r3 = R::new(3).unwrap();
    let r4 = R::new(4).unwrap();
    let r5 = R::new(5).unwrap();
    let r6 = R::new(6).unwrap();
    let r7 = R::new(7).unwrap();
    for (at, expected) in [
        (0x6804c, arm32::mov_shifted_register(C::Al, r5, r4, Shift::Asr, 16).unwrap()),
        (0x6807c, arm32::data_register(C::Al, Op::Add, R::R2, R::R2, r4, Shift::Asr, 14, false).unwrap()),
        (0x680a0, arm32::transfer_word(C::Al, false, r4, R::R2, -24).unwrap()),
        (0x680a4, arm32::multiply_accumulate(C::Al, r4, r7, R::R0, r3).unwrap()),
        (0x680b0, arm32::transfer_word(C::Al, true, r3, R::PC, 0xfec).unwrap()),
        (0x680b8, arm32::multiply_long(C::Al, false, r7, r6, R::R0, r3).unwrap()),
        (0x680c0, arm32::mov_shifted_register(C::Al, R::R0, r6, Shift::Lsr, 5).unwrap()),
    ] {
        let offset = at - continuation::START;
        assert_eq!(&bytes[offset..offset + 4], &expected.to_le_bytes());
    }
    assert_eq!(0x680b0 + 8 + 0xfec, 0x690a4);
}

#[test]
fn controller_continuation_table_initialization_and_tiers() {
    use arm32::{Condition as C, DataOp as Op, Register as R};
    use firmware_tools::devices::xiaomi_nabu::venus_upper_controller_continuation as continuation;
    let bytes = continuation::fragment().unwrap();
    let r8 = R::new(8).unwrap();
    for (at, expected) in [
        (0x680cc, arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0).unwrap()),
        (0x680dc, arm32::transfer_byte(C::Al, false, R::R1, r8, 0x9c).unwrap()),
        (0x680e0, arm32::transfer_byte(C::Al, false, R::R2, r8, 0x9d).unwrap()),
        (0x680e4, arm32::branch(C::Ne, 0x680e4, 0x6814c, false).unwrap()),
        (0x68154, arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0xf00).unwrap()),
        (0x68158, arm32::branch(C::Cc, 0x68158, 0x68198, false).unwrap()),
        (0x68194, arm32::branch(C::Al, 0x68194, 0x68214, false).unwrap()),
        (0x68198, arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x780).unwrap()),
        (0x6819c, arm32::branch(C::Cc, 0x6819c, 0x681dc, false).unwrap()),
        (0x681d8, arm32::branch(C::Al, 0x681d8, 0x68214, false).unwrap()),
        (0x68214, arm32::data_immediate(C::Al, Op::Add, R::R0, r8, 0x1cc).unwrap()),
        (0x68218, arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x58).unwrap()),
    ] {
        let offset = at - continuation::START;
        assert_eq!(&bytes[offset..offset + 4], &expected.to_le_bytes());
    }
    for at in [
        0x680f8, 0x68108, 0x68118, 0x68128, 0x68138, 0x68148,
        0x68168, 0x6817c, 0x6818c, 0x681ac, 0x681c0, 0x681d0,
        0x681e8, 0x681fc, 0x6820c, 0x6821c,
    ] {
        let offset = at - continuation::START;
        let expected = arm32::branch(C::Al, at as u32, 0x471c, true).unwrap();
        assert_eq!(&bytes[offset..offset + 4], &expected.to_le_bytes());
    }
    for (at, offset, target) in [
        (0x680e8, 0xfd8, 0x690c8), (0x680ec, 0xfd8, 0x690cc),
        (0x6815c, 0xf98, 0x690fc), (0x681a0, 0xf50, 0x690f8),
        (0x681dc, 0xee8, 0x690cc), (0x68210, 0xf14, 0x6912c),
    ] {
        assert_eq!(at + 8 + offset, target);
        assert!(target >= continuation::END);
        let register = if at == 0x680e8 { R::new(6).unwrap() }
            else if at == 0x680ec { R::R0 } else { R::R1 };
        let expected = arm32::transfer_word(C::Al, true, register, R::PC, offset as i32).unwrap();
        let index = at - continuation::START;
        assert_eq!(&bytes[index..index + 4], &expected.to_le_bytes());
    }
}

#[test]
fn controller_continuation_matches_pinned_reference_when_available() {
    use firmware_tools::devices::xiaomi_nabu::venus_upper_controller_continuation as continuation;
    if let Some(reference) = std::env::var_os("NABU_VENUS_REFERENCE") {
        let original = std::fs::read(reference).unwrap();
        assert_eq!(firmware_tools::wmfw_sha256::hex_digest(&original),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        let bytes = continuation::fragment().unwrap();
        let start = continuation::START + continuation::FILE_MAPPING_DELTA;
        let end = continuation::END + continuation::FILE_MAPPING_DELTA;
        assert_eq!(bytes.as_slice(), original.get(start..end).unwrap());
    }
}
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

#[test]fn controller_allocator_matches_private_reference_when_available(){
    let b=controller_alloc::fragment().unwrap();
    assert_eq!(b.len(),controller_alloc::END-controller_alloc::START);
    if let Some(original)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let fw=std::fs::read(original).unwrap();let off=controller_alloc::START+controller_alloc::FILE_BASE;
        assert_eq!(&b[..],fw.get(off..off+b.len()).unwrap());
    }
}

#[test]fn controller_runner_prefix_matches_private_reference_if_connected(){
    let b=controller_run_prefix::fragment().unwrap();
    assert_eq!(b.len(),controller_run_prefix::END-controller_run_prefix::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let fw=std::fs::read(reference).unwrap();let off=controller_run_prefix::START+controller_run_prefix::FILE_BASE;
        assert_eq!(&b[..],fw.get(off..off+b.len()).unwrap());
    }
}

#[test]fn controller_run_tail_matches_private_original_if_supplied(){
    let b=controller_run_tail::fragment().unwrap();assert_eq!(b.len(),controller_run_tail::END-controller_run_tail::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let fw=std::fs::read(reference).unwrap();let off=controller_run_tail::START+controller_run_tail::FILE_BASE;
        assert_eq!(&b[..],fw.get(off..off+b.len()).unwrap());
    }
}

#[test]fn small_controller_helpers_match_private_original_if_available(){
    let bytes=controller_small_helpers::fragment().unwrap();
    assert_eq!(bytes.len(),controller_small_helpers::END-controller_small_helpers::START);
    if let Some(reference)=std::env::var_os("NABU_VENUS_REFERENCE"){
        let fw=std::fs::read(reference).unwrap();let off=controller_small_helpers::START+controller_small_helpers::FILE_BASE;
        assert_eq!(&bytes[..],fw.get(off..off+bytes.len()).unwrap());
    }
}
