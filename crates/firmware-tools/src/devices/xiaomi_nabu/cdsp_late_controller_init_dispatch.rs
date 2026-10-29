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
use super::cdsp_boot::{self, GeneralRegister as G, PacketMarker as M, ScalarPacket as S};
use super::cdsp_late_controller_frame_setup;

pub const FILE_OFFSET: usize = 0x56930;
pub const END_FILE_OFFSET: usize = 0x56990;
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

/// Skip invoking the per-item service on a null controller entry.
pub fn encode_skip_empty_item(src:G,from:u32,to:u32)->Result<u32,String>{
    if src.number()!=17 || (from,to)!=(0x96cc_8954,0x96cc_8968){
        return Err("controller scan expected R17 and fixed service-skip target".into());
    }
    let d=to.checked_sub(from).ok_or("scan skip must be forward")?;
    if d&1!=0||d>0x1fe {return Err("scan skip not in checked halfword range".into());}
    // V66 uses an extended register selector bit: R17 appears in the low
    // compare register bank as index 9 for this fused new-predicate branch.
    Ok(M::Final.apply(0x1009_c000|((d>>1)&0xff)))
}

/// Direct call to the item-validation helper, physical target 0x96ddf2c0.
pub fn encode_scan_callback(from:u32,to:u32)->Result<u32,String>{
    if (from,to)!=(0x96cc_8958,0x96dd_f2c0){
        return Err("unverified controller enumerator callback target".into());
    }
    let d=to.checked_sub(from).ok_or("scan callback behind callsite")?;
    if d&1!=0||d>0x1fff_fe {return Err("scan callback displacement invalid".into());}
    Ok(M::ContinuationA.apply(0x5a00_4000|(((d>>15)&0x3f)<<16)|((d>>1)&0x3fff)))
}

/// Original duplex `R1=add(FP,#24); R0=R16`. LLVM6 emits equivalent
/// separate operations; Rizin confirms the original compressed pair.
pub fn encode_duplex_fp_addr_and_pointer(dst:G,fp:G,off:u8,pointer:G,from:G)->Result<u32,String>{
    if (dst.number(),fp.number(),off,pointer.number(),from.number())!=(1,29,24,0,16){
        return Err("controller cursor call must pass FP+24 and R16".into());
    }
    // LLVM6 emits the two semantic operations as separate words, whereas
    // original V66 uses this independently decoded, tuple-restricted duplex.
    Ok(0x2c61_3080)
}

/// If P0.new, move R17 into R19 using the original conditional ADD-zero.
pub fn encode_predicated_cursor_copy(dst:G,src:G,offset:u8)->Result<u32,String>{
    if (dst.number(),src.number(),offset)!=(19,17,0){
        return Err("unsupported predicated enumerator cursor assignment".into());
    }
    Ok(M::Final.apply(0x7400_e000|(u32::from(src.number())<<16)|u32::from(dst.number())))
}

/// `R2=memb(R16+#160)`, a single ordinary byte load (not an extender).
pub fn encode_byte_at_next_entry(dst:G,base:G,offset:u16)->Result<u32,String>{
    if (dst.number(),base.number(),offset)!=(2,16,160){
        return Err("enumerator byte load expects R2 and R16+160".into());
    }
    Ok(M::Final.apply(0x9100_c000|(u32::from(base.number())<<16)|
        (u32::from(offset)<<5)|u32::from(dst.number())))
}

/// Conditional stack-result word load following zero predicate loop.
pub fn encode_guarded_stack_mode_load(dst:G,base:G,offset:u8)->Result<u32,String>{
    if (dst.number(),base.number(),offset)!=(2,29,20){
        return Err("controller fallback reads R2 from SP+20 only".into());
    }
    Ok(M::Final.apply(0x4380_c000|(u32::from(base.number())<<16)|
        (u32::from(offset)<<3)|u32::from(dst.number())))
}

/// `P0=R2` before the guarded controller-mode clear and exit.
pub fn encode_predicate_from_data(src:G)->Result<u32,String>{
    if src.number()!=2{return Err("controller predicate source must be R2".into());}
    Ok(M::ContinuationA.apply(0x8540_c000|(u32::from(src.number())<<16)))
}

/// `if(!P0.new) R21=#0`; checked V66 predicated small immediate.
pub fn encode_clear_mode_if_p0_false(dst:G)->Result<u32,String>{
    if dst.number()!=21{return Err("controller empty-mode assignment must target R21".into());}
    Ok(M::ContinuationA.apply(0x7e80_6000|u32::from(dst.number())))
}

/// `if(P0.new) jump:nt` from enumerator packet at 0x96cc8988.
pub fn encode_guarded_exit(from:u32,to:u32)->Result<u32,String>{
    if (from,to)!=(0x96cc_8988,EMPTY_MODE_TARGET){
        return Err("unsupported enumerator final guarded exit".into());
    }
    let d=to.checked_sub(from).ok_or("exit must be forward")?;
    if d&1!=0||d>0x1fe{return Err("controller exit displacement not encodable".into());}
    // V66 P0.new :nt form adds eight bytes of packet bias in this class.
    Ok(M::ContinuationA.apply(0x5c00_4800|((d+8)>>1)))
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
    emit(&mut out, encode_skip_empty_item(r(17)?, 0x96cc_8954, 0x96cc_8968)?);
    emit(&mut out, encode_scan_callback(0x96cc_8958, 0x96dd_f2c0)?);
    emit(&mut out, encode_duplex_fp_addr_and_pointer(r(1)?,r(29)?,24,r(0)?,r(16)?)?);
    emit(&mut out, M::ContinuationA.apply(S::PredicateP0EqualsZero{src:r(0)?}.encode()?));
    emit(&mut out, encode_predicated_cursor_copy(r(19)?,r(17)?,0)?);
    emit(&mut out, M::ContinuationA.apply(S::AddSmallImmediate{dst:r(17)?,src:r(17)?,value:1}.encode()?));
    emit(&mut out, M::ContinuationA.apply(S::AddUnsignedImmediate12{dst:r(16)?,src:r(16)?,value:160}.encode()?));
    emit(&mut out, encode_byte_at_next_entry(r(2)?,r(16)?,160)?);
    emit(&mut out, M::ContinuationA.apply(S::LoopOnNotP0New{src:r(2)?,displacement_bytes:-32}.encode()?));
    emit(&mut out, M::ContinuationA.apply(S::Nop.encode()?));
    emit(&mut out, encode_guarded_stack_mode_load(r(2)?,r(29)?,20)?);
    emit(&mut out, encode_predicate_from_data(r(2)?)?);
    emit(&mut out, encode_clear_mode_if_p0_false(r(21)?)?);
    emit(&mut out, encode_guarded_exit(0x96cc_8988,EMPTY_MODE_TARGET)?);
    emit(&mut out, S::Nop.encode()?);
    if out.len() != EXECUTABLE_LEN { return Err("controller input+enumeration must contain exactly 96 bytes".into()); }
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
        assert_eq!(w.len(), 24);
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
        assert_eq!(w[9],0x1009_c00a);
        assert_eq!(w[10],0x5a22_74b4);
        assert_eq!(w[11],0x2c61_3080);
        assert_eq!(w[12],0x7500_4000);
        assert_eq!(w[13],0x7411_e013);
        assert_eq!(w[14],0xb011_4031);
        assert_eq!(w[15],0xb010_5410);
        assert_eq!(w[16],0x9110_d402);
        assert_eq!(w[17],0x1072_60f0);
        assert_eq!(w[18],0x7f00_4000);
        assert_eq!(w[19],0x439d_c0a2);
        assert_eq!(w[20],0x8542_4000);
        assert_eq!(w[21],0x7e80_6015);
        assert_eq!(w[22],0x5c00_48a8);
        assert_eq!(w[23],0x7f00_c000);
        assert!(encode_skip_empty_item(r(16).unwrap(),0x96cc8954,0x96cc8968).is_err());
        assert!(encode_duplex_fp_addr_and_pointer(r(1).unwrap(),r(29).unwrap(),20,r(0).unwrap(),r(16).unwrap()).is_err());
        assert!(encode_byte_at_next_entry(r(2).unwrap(),r(16).unwrap(),156).is_err());
        assert!(encode_guarded_exit(0x96cc8988,0x96cc8ad4).is_err());
    }
    #[test]
    fn item_enumerator_has_original_packet_relative_branches() {
        let bytes=build_controller_input_start().unwrap();
        assert_eq!(bytes.len(),0x60);
        assert_eq!(encode_skip_empty_item(r(17).unwrap(),0x96cc8954,0x96cc8968).unwrap(),0x1009_c00a);
        assert_eq!(encode_scan_callback(0x96cc8958,0x96ddf2c0).unwrap(),0x5a22_74b4);
        let loop_instr=S::LoopOnNotP0New{src:r(2).unwrap(),displacement_bytes:-32}.encode().unwrap();
        assert_eq!(M::ContinuationA.apply(loop_instr),0x1072_60f0);
        assert_eq!(encode_guarded_exit(0x96cc8988,0x96cc8ad0).unwrap(),0x5c00_48a8);
    }

    #[test]
    fn prefix_tracks_executable_and_layout_bytes_separately() {
        assert_eq!(build_prefix().unwrap().len(), 0x990);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2392);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,56);
    }
}
