// SPDX-License-Identifier: GPL-3.0-only
//! Controller function beginning at CDSP file offset 0x56930 (physical
//! 0x96cc8930). The caller's frame and four-character controller identifiers
//! have been emitted by `cdsp_late_controller_frame_setup` immediately before
//! this region. This is a partial function, not an independently bootable DSP.
//!
//! Encoders below use checked V66 register/immediate fields corroborated by
//! an independent LLVM 6 instruction fixture and Rizin V66 decoding. LLVM
//! schedules its middle four operations in a different order, so the emitter
//! retains the original packet topology after checking their semantics.
use super::cdsp_boot::{self, GeneralRegister as G, PacketMarker as M};
use super::cdsp_late_controller_frame_setup;

pub const FILE_OFFSET: usize = 0x56930;
pub const END_FILE_OFFSET: usize = 0x56954;
pub const EXECUTABLE_LEN: usize = END_FILE_OFFSET - FILE_OFFSET;
pub const PHYSICAL_ADDRESS: u32 = 0x96cc_8930;
pub const RWE_START: usize = cdsp_late_controller_frame_setup::RWE_START;
pub const FULL_RWE_LAYOUT_LEN: usize = END_FILE_OFFSET - RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN: usize =
    cdsp_late_controller_frame_setup::FULL_RWE_EXECUTABLE_LEN + EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN: usize = cdsp_late_controller_frame_setup::RWE_PREFIX_LAYOUT_LEN;

const CONTROLLER_LOG_SLOT: u32 = 0xc036_4ae0;
const INPUT_MODE_ADDR: u32 = 0xc02a_0e40;
const INIT_SERVICE: u32 = 0x96dc_4248;
const EMPTY_MODE_TARGET: u32 = 0x96cc_8ad0;

fn r(n: u8) -> Result<G, String> { G::new(n) }
fn emit(v: &mut Vec<u8>, w: u32) { v.extend_from_slice(&w.to_le_bytes()); }

/// Normal unextended Hexagon direct call, relative to physical packet start.
/// Deliberately restrict to the independently verified controller service.
pub fn encode_init_call(at: u32, to: u32) -> Result<u32, String> {
    if (at, to) != (PHYSICAL_ADDRESS, INIT_SERVICE) {
        return Err("unverified CDSP controller init call source/target".into());
    }
    let d = to.checked_sub(at).ok_or("init service call must be forward")?;
    if d & 1 != 0 || d > 0x1fff_fe {
        return Err("init service call displacement outside Hexagon narrow encoding".into());
    }
    Ok(M::ContinuationA.apply(0x5a00_4000 | (((d >> 15) & 0x3f) << 16) | ((d >> 1) & 0x3fff)))
}

/// V66 `memw(R18=##absolute)=R19`. The extended assignment comes from a
/// regular checked immext, but this particular store-and-assign consumer is
/// limited to the pointer, registers and alignment independently assembled.
pub fn encode_controller_log_store(base: G, value: G, address: u32) -> Result<[u32; 2], String> {
    if (base.number(), value.number(), address) != (18, 19, CONTROLLER_LOG_SLOT) {
        return Err("unsupported controller log memory register/absolute address".into());
    }
    let [prefix, _assignment] = cdsp_boot::encode_extended_assignment(base, address as i32, false);
    let consumer = 0xab80_c000 | (u32::from(base.number()) << 16)
        | (u32::from(value.number()) << 8) | ((address & 0x3f) << 2) | 0x20;
    Ok([M::ContinuationA.apply(prefix), M::Final.apply(consumer)])
}

/// Byte read `R2=memb(R20=##0xc02a0e40)` using a checked extended
/// 32-bit absolute source address; nonzero lower address bits are unsupported.
pub fn encode_controller_byte_load(dst: G, base: G, address: u32) -> Result<[u32; 2], String> {
    if (dst.number(), base.number(), address) != (2, 20, INPUT_MODE_ADDR) || address & 0x3f != 0 {
        return Err("unsupported controller input byte location or destination".into());
    }
    let [prefix, _assignment] = cdsp_boot::encode_extended_assignment(base, address as i32, false);
    let body = 0x9b00_d000 | (u32::from(base.number()) << 16) | u32::from(dst.number());
    Ok([M::ContinuationA.apply(prefix), M::Final.apply(body)])
}

/// The independently LLVM-assembled V66 duplex operation
/// `R17:16=combine(#0,R20)`. Other register pairs must be independently
/// confirmed before being accepted.
pub fn encode_controller_combine(low: G, src: G) -> Result<u32, String> {
    if (low.number(), src.number()) != (16, 20) {
        return Err("unsupported controller pointer/byte-index duplex".into());
    }
    Ok(0x7300_6000 | (u32::from(src.number()) << 16)
        | (u32::from(low.number()) << 0) | 0x0020_0000)
}

/// P0=cmp.eq(R2,#0); if (P0.new) jump:nt +0x18c. The branch is
/// **packet-relative**, so its base is `0x96cc8944`, not the next PC.
pub fn encode_empty_input_skip(src: G, packet_at: u32, to: u32) -> Result<u32, String> {
    if src.number() != 2 || (packet_at, to) != (0x96cc_8944, EMPTY_MODE_TARGET) {
        return Err("unverified controller empty-input packet/branch".into());
    }
    let d = to.checked_sub(packet_at).ok_or("empty input target behind packet")?;
    if d & 1 != 0 || d > 0x1fe { return Err("empty input skip displacement invalid".into()); }
    Ok(M::Final.apply(0x1000_c000 | (u32::from(src.number()) << 16) | ((d >> 1) & 0x1ff)))
}

/// Standalone `R2=P0` (P0 is the only encoded predicate used here).
pub fn encode_predicate_to_register(dst: G, source: u8) -> Result<u32, String> {
    if dst.number() != 2 || source != 0 {
        return Err("controller needs only R2=P0".into());
    }
    Ok(M::ContinuationA.apply(0x8940_0000 | u32::from(dst.number())))
}

/// V66 implicit-new word store `memw(SP+20)=R2.new`. The packet stores a
/// `.new` distance, not the original source register number; it was checked
/// by varying both stack displacement and source registers in LLVM 6.
pub fn encode_new_stack_result(stack: G, byte_offset: u8, distance: u8) -> Result<u32, String> {
    if (stack.number(), byte_offset, distance) != (29, 20, 2) {
        return Err("controller expects SP+20 and new-value distance 2".into());
    }
    Ok(M::Final.apply(0xa1a0_d000 | (u32::from(stack.number()) << 16)
        | (u32::from(distance) << 8) | u32::from(byte_offset / 4)))
}

pub fn build_controller_input_start() -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out, encode_init_call(PHYSICAL_ADDRESS, INIT_SERVICE)?);
    for w in encode_controller_log_store(r(18)?, r(19)?, CONTROLLER_LOG_SLOT)? { emit(&mut out, w); }
    for w in encode_controller_byte_load(r(2)?, r(20)?, INPUT_MODE_ADDR)? { emit(&mut out, w); }
    emit(&mut out, M::ContinuationA.apply(encode_controller_combine(r(16)?, r(20)?)?));
    emit(&mut out, encode_empty_input_skip(r(2)?, 0x96cc_8944, EMPTY_MODE_TARGET)?);
    emit(&mut out, encode_predicate_to_register(r(2)?, 0)?);
    emit(&mut out, encode_new_stack_result(r(29)?, 20, 2)?);
    if out.len() != EXECUTABLE_LEN { return Err("controller input prefix must be exactly 36 bytes".into()); }
    Ok(out)
}

pub fn build_prefix() -> Result<Vec<u8>, String> {
    let mut p = cdsp_late_controller_frame_setup::build_prefix()?;
    p.extend(build_controller_input_start()?);
    if p.len() != FULL_RWE_LAYOUT_LEN || p.len() != FULL_RWE_EXECUTABLE_LEN + RWE_PREFIX_LAYOUT_LEN {
        return Err("controller-input prefix code/fill accounting mismatch".into());
    }
    Ok(p)
}

pub fn verify(reference: &[u8]) -> Result<(), String> {
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET) != Some(build_controller_input_start()?.as_slice()) {
        return Err("controller input prefix differs from pinned CDSP".into());
    }
    Ok(())
}

pub fn verify_prefix(reference: &[u8]) -> Result<(), String> {
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET) != Some(build_prefix()?.as_slice()) {
        return Err("full RWE prefix through input start differs from pinned CDSP".into());
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test]
    fn controller_input_fields_and_packet_targets_are_checked() {
        let bytes = build_controller_input_start().unwrap();
        let w: Vec<u32> = bytes.chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
        assert_eq!(w.len(), 9);
        assert_eq!(w[0], 0x5a1f_5c8c);
        assert_eq!(w[1], 0x0c03_592b);
        assert_eq!(w[2], 0xab92_d3a0);
        assert_eq!(w[3], 0x0c02_6839);
        assert_eq!(w[4], 0x9b14_d002);
        assert_eq!(w[5], 0x7334_6010);
        assert_eq!(w[6], 0x1002_c0c6);
        assert_eq!(w[7], 0x8940_4002);
        assert_eq!(w[8], 0xa1bd_d205);
        assert!(encode_init_call(PHYSICAL_ADDRESS+4,INIT_SERVICE).is_err());
        assert!(encode_controller_log_store(r(17).unwrap(),r(19).unwrap(),CONTROLLER_LOG_SLOT).is_err());
        assert!(encode_controller_byte_load(r(2).unwrap(),r(20).unwrap(),INPUT_MODE_ADDR+1).is_err());
        assert!(encode_controller_combine(r(16).unwrap(),r(21).unwrap()).is_err());
        assert!(encode_empty_input_skip(r(2).unwrap(),0x96cc8948,EMPTY_MODE_TARGET).is_err());
        assert!(encode_predicate_to_register(r(1).unwrap(),0).is_err());
        assert!(encode_new_stack_result(r(29).unwrap(),16,2).is_err());
    }
    #[test]
    fn prefix_tracks_executable_and_layout_bytes_separately() {
        assert_eq!(build_prefix().unwrap().len(), 0x954);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2332);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,56);
    }
}
