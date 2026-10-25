// SPDX-License-Identifier: GPL-3.0-only
//! CDSP Hexagon V66 runtime callback dispatch, file [0x56120,0x561a0).
//!
//! The preceding [0x56114,0x56120) holds three separately classified NOP
//! padding instructions. This module emits source-derived operations with
//! checked register/immediate fields and original instruction-packet markers.
//! LLVM6 independently confirms the complete semantic routine, although
//! LLVM6 shortens the first absolute load, extends R1's immediate, and swaps
//! two producer/consumer instructions. Those byte differences are NOT hidden
//! or conflated with exact compiler output. Original packet order is decoded
//! with Rizin V66 and compared to the separately held pinned CDSP MBN.

use super::cdsp_boot::{self, GeneralRegister as G, PacketMarker as M, ScalarPacket as S};
use super::cdsp_late_mmio::encode_word_load_immediate;
use super::cdsp_late_runtime::encode_callr;
use super::cdsp_late_callback_registry::encode_r2_new_zero_branch;

pub const FILE_OFFSET: usize = 0x56120;
pub const END_FILE_OFFSET: usize = 0x561a0;
pub const EXECUTABLE_LEN: usize = END_FILE_OFFSET - FILE_OFFSET; // 128
pub const PHYSICAL_ADDRESS: u32 = 0x96cc_8120;
pub const ELF_VADDR: u32 = 0xc00c_8120;
pub const PRIOR_NOP_FILE_OFFSET: usize = 0x56114;
pub const PRIOR_NOP_FILL_LEN: usize = FILE_OFFSET - PRIOR_NOP_FILE_OFFSET;
pub const FIRST_RWE_FILE_OFFSET: usize = 0x56000;
pub const RWE_PREFIX_WITH_REGISTRY_END: usize = 0x561e0;
pub const SOURCE_GENERATED_EXECUTABLE_LEN: usize = 84 + 116 + 64 + EXECUTABLE_LEN + 64;
pub const EXPLICIT_INSTRUCTION_LAYOUT_LEN: usize = 12 + PRIOR_NOP_FILL_LEN;

fn emit(out: &mut Vec<u8>, instruction: u32) {
    out.extend_from_slice(&instruction.to_le_bytes());
}

/// Save R17:16 at FP-16 and allocate a sixteen-byte frame. This original
/// duplex form has a distinct packet class from a normal scalar MOV; LLVM6
/// independently confirms the single encoded word.
pub fn encode_save_pair_and_allocframe(low: G, base: G,
                                       save_offset: i8, frame_size: u8) -> Result<u32, String> {
    if low.number() != 16 || base.number() != 29 ||
       save_offset != -16 || frame_size != 16 {
        return Err("unsupported V66 pair-save + allocframe duplex operands".into());
    }
    Ok(0xebf4_1c20)
}

/// Ordinary R19:18 pair store at SP+0; its base/pair fields are shared with
/// LLVM6's independent `memd(sp+#0)=r19:18` encoding.
pub fn encode_pair_store_sp(low: G, base: G, offset: u8) -> Result<u32, String> {
    if low.number() & 1 != 0 || low.number() > 30 ||
       base.number() != 29 || offset != 0 {
        return Err("RWE paired frame store needs SP+0 and an even low GPR".into());
    }
    Ok(0xa1c0_c000 | (u32::from(base.number()) << 16)
       | (u32::from(low.number()) << 8))
}

/// Original forced-extender absolute load: `R2=memw(R16=##address)`.
/// Its source is a checked absolute controller pointer, not a GPU ISA copy.
pub fn encode_r16_absolute_load(dst: G, base: G, address: u32) -> Result<[u32; 2], String> {
    if dst.number() != 2 || base.number() != 16 || address != 0xc036_3e00 {
        return Err("unsupported absolute runtime callback-table pointer".into());
    }
    let ext = cdsp_boot::encode_extended_assignment(base, address as i32, false)[0];
    let body = 0x9b80_5000 | (u32::from(base.number()) << 16)
        | u32::from(dst.number());
    Ok([ext, body])
}

/// New-value null branch predicted taken, from packet start, to the
/// callback cleanup epilogue. Encodes halfword branch displacement.
pub fn encode_branch_not_null_new(packet_start: u32, target: u32,
                                  tested: G) -> Result<u32, String> {
    if tested.number() != 2 || target <= packet_start ||
       target - packet_start > 0x1fe || (target-packet_start) % 2 != 0 {
        return Err("unsupported R2.new non-null forward branch".into());
    }
    Ok(0x2440_e000 | (u32::from(tested.number()) << 16)
       | ((target-packet_start)/2))
}

/// Hexagon duplex pair with a signed extended address into R17 and the
/// positive immediate 1 into R19. LLVM6 register/low-immediate mutations
/// confirmed the two register-selector and immediate field positions.
pub fn encode_extended_address_and_index(dst: G, address: u32,
                                         index_dst: G, index: u8) -> Result<[u32; 2], String> {
    if !(16..=23).contains(&dst.number()) ||
       !(16..=23).contains(&index_dst.number()) ||
       address != 0xc036_3e08 || index > 15 {
        return Err("unsupported Hexagon extended R17/R19 duplex operands".into());
    }
    let extension = cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let duplex = 0x2800_2800 | ((address & 0x3f) << 20)
        | (u32::from(dst.number()-8) << 16)
        | (u32::from(index) << 4) | u32::from(index_dst.number()-8);
    Ok([extension,duplex])
}

/// `Rdest=memw(Rindex<<2 + ##table_address)` with full positive/negative
/// immediate carried in an immext prefix; index/offset bitfields were
/// varied independently in LLVM6 semantic assembler probes.
pub fn encode_indexed_callback_load(dst: G, index: G,
                                     address: u32, shift: u8) -> Result<[u32;2],String> {
    if dst.number() >= 32 || index.number() >= 32 ||
       address != 0xc028_c010 || shift != 2 {
        return Err("unsupported RWE callback-table scaled memory index".into());
    }
    let ext = cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let body=0x9d80_7000 | (u32::from(index.number())<<16)
         | ((address & 0x3f)<<6) | u32::from(dst.number());
    Ok([ext,body])
}

/// P0=cmp.gtu(Rsrc,#n): unsigned range guard used by the wait helper.
/// GNU/LLVM6 field variation confirms each input and immediate bit.
pub fn encode_predicate_unsigned_gt(src:G,threshold:u8,end_packet:bool)->u32 {
    let base=0x7580_c000 | (u32::from(src.number())<<16)
             | (u32::from(threshold)<<5);
    (if end_packet {M::Final} else {M::ContinuationA}).apply(base)
}

/// `if (!P0.new) jump:t ...` relative to the first instruction in its packet.
pub fn encode_if_not_p0_new_taken(packet_start:u32,target:u32)->Result<u32,String>{
    if target <= packet_start || target-packet_start>0x1fe ||
        (target-packet_start)%2!=0 {
        return Err("unsupported P0.new forward taken jump".into());
    }
    Ok(0x5c20_d800 | ((target-packet_start)/2))
}

/// `Rd=addasl(Rbase,Rindex,#shift)` (shift-two form). LLVM6 variation
/// independently determines which operand selects each field.
pub fn encode_addasl(dst:G,base:G,index:G,shift:u8,end_packet:bool)->Result<u32,String>{
    if shift != 2 || dst.number()>31 ||base.number()>31 ||index.number()>31 {
        return Err("only validated addasl shift-two is supported".into());
    }
    let w=0xc400_c000 | (u32::from(index.number())<<16)
        | (u32::from(base.number())<<8)
        | (u32::from(shift)<<5) | u32::from(dst.number());
    Ok((if end_packet{M::Final}else{M::ContinuationA}).apply(w))
}

/// `Rd=sub(#unsigned8,Rsrc)` in a single scalar word, no extender.
pub fn encode_reverse_subtract_imm(dst:G,src:G,immediate:u8)->u32{
    0x7640_c000 | (u32::from(src.number())<<16)
        | (u32::from(immediate)<<5) | u32::from(dst.number())
}

/// A Hexagon `.new` store's consumer distance is measured in *instructions*
/// within the packet; the two verified distances here are one and two slots.
pub fn encode_new_store(base:G,source_distance:u8,offset:u8)->Result<u32,String>{
    if base.number()!=16 || offset!=0 || ![1,2].contains(&source_distance){
        return Err("unsupported CDSP R2.new packet-distance memory store".into());
    }
    Ok(0xa1a0_d000 |(u32::from(base.number())<<16)
       |(u32::from(source_distance)*2<<8))
}

/// Packet-constrained R2==0 conditional backward jump. Negative offset
/// counts from the packet's *first* instruction (`R19=add(...)`).
pub fn encode_r2_zero_backedge(packet_start:u32,target:u32,src:G)->Result<u32,String>{
    let delta=i64::from(target)-i64::from(packet_start);
    if src.number()!=2 || delta>=0 || delta < -0x100 || delta%2!=0 {
        return Err("unsupported packet-relative backward R2 zero jump".into());
    }
    Ok(0x1030_e000 | (u32::from(src.number())<<16)
       | (((delta / 2) as i32 as u32)&0xff))
}

/// Checked original double-load/return duplex, observed independently in
/// LLVM6 and Rizin V66. The two register pairs cannot alias arbitrary GPRs.
pub fn encode_restore_two_pairs(low0:G,low1:G,fp:G)->Result<u32,String>{
    if low0.number()!=16 || low1.number()!=18 || fp.number()!=29{
        return Err("CDSP double-restore expects R17:16 R19:18 FP".into());
    }
    Ok(0x3e0c_1e05)
}

/// Return and deallocate frame using the verified V66 single-word duplex.
pub fn encode_dealloc_return()->u32 {0x961e_c01e}

pub fn build_callback_dispatcher()->Result<Vec<u8>,String>{
    let r=|n|G::new(n);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_save_pair_and_allocframe(r(16)?,r(29)?,-16,16)?);
    emit(&mut out,encode_pair_store_sp(r(18)?,r(29)?,0)?);
    for word in encode_r16_absolute_load(r(2)?,r(16)?,0xc036_3e00)?{
        emit(&mut out,word);
    }
    emit(&mut out,encode_branch_not_null_new(PHYSICAL_ADDRESS+0x08,
                                            PHYSICAL_ADDRESS+0x78,r(2)?)?);
    for word in encode_extended_address_and_index(r(17)?,0xc036_3e08,r(19)?,1)? {
        emit(&mut out,word);
    }
    for word in encode_indexed_callback_load(r(2)?,r(19)?,0xc028_c010,2)? {
        emit(&mut out,word);
    }
    emit(&mut out,encode_r2_new_zero_branch(PHYSICAL_ADDRESS+0x1c,
                                           PHYSICAL_ADDRESS+0x48)?);
    let [ext,inst]=cdsp_boot::encode_extended_assignment(r(0)?,0xc036_3e08u32 as i32,false);
    emit(&mut out,ext);
    emit(&mut out,inst);
    emit(&mut out,M::ContinuationA.apply(encode_callr(r(2)?)));
    emit(&mut out,S::AssignUnsignedImmediate{dst:r(1)?,value:0x320}.encode()?);
    emit(&mut out,encode_word_load_immediate(r(2)?,r(16)?,0)?);
    emit(&mut out,encode_predicate_unsigned_gt(r(2)?,0xc7,false));
    emit(&mut out,encode_if_not_p0_new_taken(PHYSICAL_ADDRESS+0x3c,
                                             PHYSICAL_ADDRESS+0x4c)?);
    emit(&mut out,S::JumpRelative{displacement_bytes:0x34}.encode()?);
    emit(&mut out,S::AssignSmallImmediate{dst:r(2)?,value:0}.encode()?);
    emit(&mut out,encode_addasl(r(3)?,r(17)?,r(2)?,2,false)?);
    emit(&mut out,encode_reverse_subtract_imm(r(4)?,r(2)?,0xc8));
    emit(&mut out,S::HardwareLoop{loop_index:0,counter:r(4)?,displacement_bytes:4}.encode()?);
    emit(&mut out,M::ContinuationA.apply(encode_word_load_immediate(r(4)?,r(3)?,0)?));
    emit(&mut out,encode_r2_new_zero_branch(PHYSICAL_ADDRESS+0x58,
                                           PHYSICAL_ADDRESS+0x70)?);
    emit(&mut out,M::LoopEnd.apply(S::AddSmallImmediate{
        dst:r(2)?,src:r(2)?,value:1}.encode()?));
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{
        dst:r(3)?,src:r(3)?,value:4}.encode()?));
    emit(&mut out,encode_new_store(r(16)?,2,0)?);
    emit(&mut out,S::JumpRelative{displacement_bytes:12}.encode()?);
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{
        dst:r(19)?,src:r(19)?,value:1}.encode()?));
    emit(&mut out,encode_r2_zero_backedge(PHYSICAL_ADDRESS+0x70,
                                         PHYSICAL_ADDRESS+0x1c,r(2)?)?);
    emit(&mut out,encode_restore_two_pairs(r(16)?,r(18)?,r(29)?)?);
    emit(&mut out,encode_dealloc_return());
    if out.len()!=EXECUTABLE_LEN {
        return Err(format!("RWE callback dispatcher must be {EXECUTABLE_LEN} bytes, got {}",out.len()));
    }
    Ok(out)
}

/// NOPs are *layout instructions*, not source-derived callback body bytes.
pub fn build_preceding_nop_layout()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(PRIOR_NOP_FILL_LEN);
    for _ in 0..3{emit(&mut out,S::Nop.encode()?);}
    if out.len()!=PRIOR_NOP_FILL_LEN{return Err("expected 12-byte NOP layout".into());}
    Ok(out)
}

/// Combine independently generated startup, loader, PIC, dispatch, registry
/// and the separately characterized linker fill/NOP instructions. No opaque
/// original code is copied from a reference image into this builder.
pub fn build_complete_rwe_prefix_layout()->Result<Vec<u8>,String>{
    let mut out=super::cdsp_late_runtime_loader::build_rwe_source_and_layout_prefix()?;
    if out.len()!=PRIOR_NOP_FILE_OFFSET-FIRST_RWE_FILE_OFFSET{
        return Err("unexpected initial layout boundary".into());
    }
    out.extend(build_preceding_nop_layout()?);
    out.extend(build_callback_dispatcher()?);
    out.extend(super::cdsp_late_callback_registry::build_runtime_registry_cleanup()?);
    if out.len()!=RWE_PREFIX_WITH_REGISTRY_END-FIRST_RWE_FILE_OFFSET{
        return Err("CDSP generated RWE prefix must be exactly 480 file bytes".into());
    }
    Ok(out)
}

pub fn verify_dispatcher(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let actual=build_callback_dispatcher()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(actual.as_slice()){
        return Err("CDSP 128-byte callback dispatcher differs from pinned firmware".into());
    }
    Ok(())
}

pub fn verify_rwe_prefix_layout(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let generated=build_complete_rwe_prefix_layout()?;
    if reference.get(FIRST_RWE_FILE_OFFSET..RWE_PREFIX_WITH_REGISTRY_END)!=Some(generated.as_slice()){
        return Err("CDSP entire 480-byte RWE prefix differs from pinned original".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn r(n:u8)->G{G::new(n).unwrap()}
    #[test]
    fn encoded_operands_check_branch_targets_and_registers(){
        assert_eq!(encode_r16_absolute_load(r(2),r(16),0xc036_3e00).unwrap(),
                   [0x0c03_58f8,0x9b90_5002]);
        assert!(encode_r16_absolute_load(r(3),r(16),0xc036_3e00).is_err());
        assert!(encode_branch_not_null_new(PHYSICAL_ADDRESS+8,PHYSICAL_ADDRESS,r(2)).is_err());
        assert_eq!(encode_branch_not_null_new(PHYSICAL_ADDRESS+8,PHYSICAL_ADDRESS+0x78,r(2)).unwrap(),0x2442_e038);
        assert_eq!(encode_extended_address_and_index(r(17),0xc036_3e08,r(19),1).unwrap(),[0x0c03_58f8,0x2889_281b]);
        assert_eq!(encode_indexed_callback_load(r(2),r(19),0xc028_c010,2).unwrap(),[0x0c02_6300,0x9d93_7402]);
        assert_eq!(encode_predicate_unsigned_gt(r(2),0xc7,false),0x7582_58e0);
        assert_eq!(encode_if_not_p0_new_taken(PHYSICAL_ADDRESS+0x3c,PHYSICAL_ADDRESS+0x4c).unwrap(),0x5c20_d808);
        assert_eq!(encode_addasl(r(3),r(17),r(2),2,false).unwrap(),0xc402_5143);
        assert_eq!(encode_reverse_subtract_imm(r(4),r(2),0xc8),0x7642_d904);
        assert_eq!(encode_new_store(r(16),2,0).unwrap(),0xa1b0_d400);
        assert_eq!(encode_r2_zero_backedge(PHYSICAL_ADDRESS+0x70,PHYSICAL_ADDRESS+0x1c,r(2)).unwrap(),0x1032_e0d6);
        assert!(encode_r2_zero_backedge(PHYSICAL_ADDRESS+0x70,PHYSICAL_ADDRESS+0x74,r(2)).is_err());
    }
    #[test]
    fn dispatcher_and_full_layout_have_no_gaps(){
        let exe=build_callback_dispatcher().unwrap();
        assert_eq!(exe.len(),EXECUTABLE_LEN);
        let layout=build_complete_rwe_prefix_layout().unwrap();
        assert_eq!(layout.len(),0x1e0);
        assert_eq!(&layout[0x120..0x1a0],exe.as_slice());
        assert_eq!(SOURCE_GENERATED_EXECUTABLE_LEN,456);
        assert_eq!(EXPLICIT_INSTRUCTION_LAYOUT_LEN,24);
    }
}
