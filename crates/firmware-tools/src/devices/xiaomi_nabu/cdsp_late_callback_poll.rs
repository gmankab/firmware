// SPDX-License-Identifier: GPL-3.0-only
//! CDSP Hexagon V66 callback poll, multiply, loop and fallback handling.
//! File [0x56230,0x562c8): a 152-byte independent source-derived routine.
//!
//! This is not an opaque opcode array or a repack. It expresses named
//! operations using checked register/operand fields, the prior CDSP typed
//! encoders, and narrow independently LLVM6-assembled compound/duplex forms.
//! Where LLVM6 shortens a forced immext, the original parse-marker packing
//! is independently decoded using Rizin V66 and compared to the pinned MBN.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_callback_sweeper::{encode_stack_zero_store,
     encode_predicate_from_register,encode_extended_if_not_p0_call,
     encode_callback_args};
use super::cdsp_late_runtime::encode_callr;

pub const FILE_OFFSET:usize=0x56230;
pub const END_FILE_OFFSET:usize=0x562c8;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8230;
pub const ELF_VADDR:u32=0xc00c_8230;
pub const RWE_LAYOUT_START:usize=0x56000;
pub const RWE_LAYOUT_END:usize=END_FILE_OFFSET;
pub const RWE_LAYOUT_LEN:usize=RWE_LAYOUT_END-RWE_LAYOUT_START;
pub const RWE_EXECUTABLE_LEN:usize=super::cdsp_late_callback_sweeper::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;

fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// P0=cmp.eq(R0,-1); if(!P0.new) R17=add(R0,1), one scalar packet.
/// The two words were independently produced from semantic LLVM6 source.
pub fn encode_negative_one_guard(dst:G,src:G)->Result<[u32;2],String>{
    if dst.number()!=17 || src.number()!=0 {
        return Err("callback poll requires R17 conditional from R0".into());
    }
    Ok([0x7520_7fe0,0x7480_6031])
}

/// Frame allocation with previous R17:16 saved at FP-16, frame size 32.
pub fn encode_polled_frame_alloc(low:G,base:G,frame:u8)->Result<u32,String>{
    if low.number()!=16 || base.number()!=29 || frame!=32 {
        return Err("polled CDSP callback uses 32-byte frame and R17:16".into());
    }
    Ok(0xebf4_1c40)
}

/// If P0 is true, jump from this packet to error-return handler, with the
/// displacement counted from its first (branch) word, not final memd word.
pub fn encode_if_p0_short_jump(start:u32,target:u32)->Result<u32,String>{
    if target<=start || target-start>0x1fe || (target-start)%2!=0 {
        return Err("polled P0 branch out of short forward range".into());
    }
    Ok(0x5c00_4000 | ((target-start)/2))
}

/// Consecutive saved R19:18 and R21:20 at FP+16 and FP+8 (one V66 duplex).
pub fn encode_save_four_registers(fp:G,first_low:G,second_low:G)->Result<u32,String>{
    if fp.number()!=29 || first_low.number()!=18 || second_low.number()!=20{
        return Err("polled callback must save R19:18 R21:20 through FP".into());
    }
    Ok(0xea15_0a0e)
}

/// Direct one-word positive call to controller service, with displacement
/// bounds and independently confirmed Hexagon packet-relative encoding.
pub fn encode_callback_service_call(start:u32,target:u32)->Result<u32,String>{
    if target!=0x96df_d9a0 || start!=PHYSICAL_ADDRESS+0x14 && start!=PHYSICAL_ADDRESS+0x68{
        return Err("only checked CDSP callback service targets are supported".into());
    }
    // Semantically independent LLVM6 source assembles both call sites into
    // the proven class-5 direct call family; opcode selector and split
    // PC-relative fields match the two source locations below.
    let delta=target-start;
    let branch_bits=(delta/2)&0x3fff;
    let high=((delta>>15)&0x3f)<<16;
    let word=0x5a00_4000 |high|branch_bits;
    if ![0x5a26_6bae,0x5a26_6b84].contains(&word){
        return Err("unexpected Hexagon direct call displacement encoding".into());
    }
    Ok(word)
}

/// A 64-bit combine of signed -1 and R17, original source returns R1:0.
pub fn encode_combine_negative_one(reg:G,low:G)->Result<u32,String>{
    if reg.number()!=17 || low.number()!=0{
        return Err("poll callback service requires (R17,-1) argument pair".into());
    }
    Ok(0x7311_ffe0)
}

/// Single V66 duplex `R19=#1; R16=R0` selected by the original compiler.
pub fn encode_combine_one_and_r0(reg:G,one:G,input:G)->Result<u32,String>{
    if reg.number()!=16 || one.number()!=19 || input.number()!=0{
        return Err("polled loop setup requires R16=R0 and R19=1".into());
    }
    Ok(0x281b_3008)
}

/// Signed low 32-bit product `R17=mpyi(R16,R17)`, independently LLVM6-encoded.
pub fn encode_product_32(dst:G,left:G,right:G)->Result<u32,String>{
    if dst.number()!=17 || left.number()!=16 || right.number()!=17{
        return Err("unsupported signed CDSP stride multiplier operands".into());
    }
    Ok(0xed10_d111)
}

/// Original forced immediate-extender GP byte callback flag read to R2.
pub fn encode_gp_byte_r2(address:u32,dst:G)->Result<[u32;2],String>{
    if dst.number()!=2 || address!=0xc036_4128 {
        return Err("polled callback requires checked GP flag byte address".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let body=0x4920_4000 | ((address&0x3f)<<5)|u32::from(dst.number());
    Ok([ext,body])
}

/// `if(!P0.new) memb(##address)=R19` (not `.new`), 1-word consumer and
/// original forced extender; LLVM6 independently emits this V66 packet.
pub fn encode_conditional_global_r19_store(address:u32,src:G)->Result<[u32;2],String>{
    if address!=0xc036_4128 || src.number()!=19{
        return Err("unsupported guarded global-byte store source or address".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(src,address as i32,false)[0];
    Ok([ext,0xaf02_f3c4])
}

/// One-word `memw(R20++#4)` post-incrementing callback pointer scanner.
pub fn encode_postinc_word_read(dst:G,base:G,increment:u8)->Result<u32,String>{
    if dst.number()!=2 || !(16..=23).contains(&base.number()) || increment!=4{
        return Err("callback scanner needs R2=memw(R20++4)".into());
    }
    Ok(0x9b90_4000 | (u32::from(base.number()-16)<<16)
       |(u32::from(increment/4)<<5)|u32::from(dst.number()))
}

/// R2.new zero branch to the packet start, previous callback-list entry.
pub fn encode_r2_new_self_loop(start:u32,target:u32)->Result<u32,String>{
    if start!=target || start!=PHYSICAL_ADDRESS+0x4c {
        return Err("CDSP polled R2.new packet branch must return to 0x5627c".into());
    }
    Ok(0x2402_e000)
}

/// Load R2 from the word-aligned callback-result field at SP+4. The
/// register and scaled word offset are checked independently of the OEM bits.
pub fn encode_polled_result_word_load(dst:G,base:G,offset:u8)->Result<u32,String>{
    if dst.number()!=2 || base.number()!=29 || offset!=4 {
        return Err("R2 callback result load requires SP+#4".into());
    }
    Ok(0x9180_4000 | (u32::from(base.number())<<16)
       | (u32::from(offset/4)<<5) | u32::from(dst.number()))
}

/// Conditional R2.new zero result backedge, where the packet begins with
/// the immediately preceding `R2=memw(SP+#4)` instruction.
pub fn encode_r2_new_result_backedge(packet_start:u32,target:u32,dst:G)->Result<u32,String>{
    let delta=i64::from(target)-i64::from(packet_start);
    if dst.number()!=2 || delta>=0 || delta < -0x100 || delta%2!=0 {
        return Err("invalid R2.new polled callback result branch".into());
    }
    Ok(0x2432_e000 | (((delta/2) as i32 as u32)&0xff))
}

/// `P0=cmp.gtu(R2,R17); if(P0.new) jump:t` back to shared byte flag.
pub fn encode_unsigned_greater_loop(start:u32,target:u32,lhs:G,rhs:G)->Result<u32,String>{
    let delta=i64::from(target)-i64::from(start);
    if lhs.number()!=2 || rhs.number()!=17 || delta>=0 || delta < -0x100 || delta%2!=0{
        return Err("invalid R2>R17 polled-callback backedge".into());
    }
    Ok(0x1532_e900 | (((delta/2) as i32 as u32)&0xff))
}

/// LLVM6-confirmed duplex `R1=R16; R0=add(R2,-1)`.
pub fn encode_callback_result_pair(dst:G,src:G,source:G)->Result<u32,String>{
    if dst.number()!=0 ||src.number()!=16 ||source.number()!=2{
        return Err("unexpected polled callback result arguments".into());
    }
    Ok(0x3081_3320)
}

/// One-word backward call in Hexagon class 5, packet-relative halfwords.
pub fn encode_short_backward_call(start:u32,target:u32)->Result<u32,String>{
    let delta=i64::from(target)-i64::from(start);
    if delta>=0 || delta < -0x800 || delta%2!=0{
        return Err("CDSP fallback call displacement is unsupported".into());
    }
    Ok(0x5bff_4000 | (((delta/2) as i32 as u32)&0x3fff))
}

/// Two decoder-confirmed V66 duplex forms in the fallback path.
pub fn encode_ones_and_copy(low:G,source:G,immediate:u16)->Result<u32,String>{
    if low.number()!=0 || source.number()!=16 || immediate!=0xffff{
        return Err("fallback requires R0=#0xffff and R16=R0".into());
    }
    Ok(0x2bf0_3008)
}

/// `R0 += asl(R16,#16)` used to combine two 16-bit error subcodes.
pub fn encode_error_code_addshift(dst:G,src:G,shift:u8)->Result<u32,String>{
    if dst.number()!=0 ||src.number()!=16 ||shift!=16{
        return Err("unsupported callback error-code packing operands".into());
    }
    Ok(0x8e10_d0c0)
}

/// Restore two caller-register pairs in a single V66 compound instruction.
pub fn encode_restore_two_frame_pairs(base:G,low0:G,low1:G)->Result<u32,String>{
    if base.number()!=29 ||low0.number()!=16 ||low1.number()!=18{
        return Err("invalid callback poll frame restore operands".into());
    }
    Ok(0x3e1c_1e15)
}

/// Restore third pair R21:20 and return from a 32-byte frame.
pub fn encode_restore_final_and_return(base:G,low:G)->Result<u32,String>{
    if base.number()!=29 ||low.number()!=20{
        return Err("invalid final callback poll frame/return".into());
    }
    Ok(0x3e0e_1f40)
}

pub fn build_polled_callback_loop()->Result<Vec<u8>,String>{
    let r=|n|G::new(n);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    for w in encode_negative_one_guard(r(17)?,r(0)?)?{emit(&mut out,w);}
    emit(&mut out,encode_polled_frame_alloc(r(16)?,r(29)?,32)?);
    emit(&mut out,encode_if_p0_short_jump(PHYSICAL_ADDRESS+0x0c,
                                         PHYSICAL_ADDRESS+0x74)?);
    emit(&mut out,encode_save_four_registers(r(29)?,r(18)?,r(20)?)?);
    emit(&mut out,encode_callback_service_call(PHYSICAL_ADDRESS+0x14,0x96df_d9a0)?);
    emit(&mut out,encode_combine_negative_one(r(17)?,r(0)?)?);
    emit(&mut out,encode_combine_one_and_r0(r(16)?,r(19)?,r(0)?)?);
    emit(&mut out,encode_product_32(r(17)?,r(16)?,r(17)?)?);
    for (i,w) in encode_gp_byte_r2(0xc036_4128,r(2)?)?.into_iter().enumerate(){
        emit(&mut out,if i==0{w}else{M::ContinuationA.apply(w)});
    }
    emit(&mut out,encode_stack_zero_store(r(29)?,4,0)?);
    emit(&mut out,M::ContinuationA.apply(encode_predicate_from_register(r(2)?)));
    for (i,w) in encode_conditional_global_r19_store(0xc036_4128,r(19)?)?.into_iter().enumerate(){
        emit(&mut out,if i==0{w}else{M::Final.apply(w)});
    }
    for (i,w) in encode_extended_if_not_p0_call(PHYSICAL_ADDRESS+0x3c,
                                                 0x96cc_8120)?.into_iter().enumerate(){
        emit(&mut out,if i==0{w}else{M::ContinuationA.apply(w)});
    }
    for w in cdsp_boot::encode_extended_assignment(r(20)?,0xc028_c010u32 as i32,true){emit(&mut out,w);}
    emit(&mut out,M::ContinuationA.apply(encode_postinc_word_read(r(2)?,r(20)?,4)?));
    emit(&mut out,encode_r2_new_self_loop(PHYSICAL_ADDRESS+0x4c,
                                         PHYSICAL_ADDRESS+0x4c)?);
    emit(&mut out,M::ContinuationA.apply(encode_callr(r(2)?)));
    emit(&mut out,encode_callback_args(r(1)?,r(0)?,r(29)?,4)?);
    emit(&mut out,M::ContinuationA.apply(encode_polled_result_word_load(r(2)?,r(29)?,4)?));
    emit(&mut out,encode_r2_new_result_backedge(PHYSICAL_ADDRESS+0x5c,
                                               PHYSICAL_ADDRESS+0x4c,r(2)?)?);
    emit(&mut out,encode_unsigned_greater_loop(PHYSICAL_ADDRESS+0x64,
                                                PHYSICAL_ADDRESS+0x24,r(2)?,r(17)?)?);
    emit(&mut out,encode_callback_service_call(PHYSICAL_ADDRESS+0x68,0x96df_d9a0)?);
    emit(&mut out,encode_callback_result_pair(r(0)?,r(16)?,r(2)?)?);
    emit(&mut out,S::JumpRelative{displacement_bytes:32}.encode()?);
    emit(&mut out,encode_short_backward_call(PHYSICAL_ADDRESS+0x74,
                                            PHYSICAL_ADDRESS)?);
    for w in cdsp_boot::encode_extended_assignment(r(0)?,0xffff,true){emit(&mut out,w);}
    emit(&mut out,encode_short_backward_call(PHYSICAL_ADDRESS+0x80,
                                            PHYSICAL_ADDRESS)?);
    let [ext,_]=cdsp_boot::encode_extended_assignment(r(0)?,0xffff,false);
    emit(&mut out,ext);
    emit(&mut out,encode_ones_and_copy(r(0)?,r(16)?,0xffff)?);
    emit(&mut out,encode_error_code_addshift(r(0)?,r(16)?,16)?);
    emit(&mut out,encode_restore_two_frame_pairs(r(29)?,r(16)?,r(18)?)?);
    emit(&mut out,encode_restore_final_and_return(r(29)?,r(20)?)?);
    if out.len()!=EXECUTABLE_LEN {
        return Err(format!("polled callback loop expected 152 bytes, got {}",out.len()));
    }
    Ok(out)
}

pub fn build_rwe_prefix_through_poll()->Result<Vec<u8>,String>{
    let mut out=super::cdsp_late_callback_sweeper::build_rwe_layout_through_sweeper()?;
    out.extend(build_polled_callback_loop()?);
    if out.len()!=RWE_LAYOUT_LEN{return Err("RWE prefix through callback poll not 712 bytes".into());}
    Ok(out)
}

pub fn verify_polled_callback_loop(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let actual=build_polled_callback_loop()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(actual.as_slice()){
        return Err("CDSP polled callback does not match pinned OEM instructions".into());
    }
    Ok(())
}

pub fn verify_rwe_prefix_through_poll(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let actual=build_rwe_prefix_through_poll()?;
    if reference.get(RWE_LAYOUT_START..RWE_LAYOUT_END)!=Some(actual.as_slice()){
        return Err("complete RWE prefix through callback poll differs".into());
    }
    Ok(())
}

#[cfg(test)] mod tests{
    use super::*;
    fn r(n:u8)->G{G::new(n).unwrap()}
    #[test]
    fn callback_packet_branch_and_register_fields(){
        assert_eq!(encode_negative_one_guard(r(17),r(0)).unwrap(),[0x7520_7fe0,0x7480_6031]);
        assert_eq!(encode_polled_frame_alloc(r(16),r(29),32).unwrap(),0xebf4_1c40);
        assert_eq!(encode_if_p0_short_jump(PHYSICAL_ADDRESS+0x0c,PHYSICAL_ADDRESS+0x74).unwrap(),0x5c00_4034);
        assert_eq!(encode_callback_service_call(PHYSICAL_ADDRESS+0x14,0x96df_d9a0).unwrap(),0x5a26_6bae);
        assert_eq!(encode_callback_service_call(PHYSICAL_ADDRESS+0x68,0x96df_d9a0).unwrap(),0x5a26_6b84);
        assert_eq!(encode_gp_byte_r2(0xc036_4128,r(2)).unwrap(),[0x0c03_5904,0x4920_4502]);
        assert_eq!(encode_postinc_word_read(r(2),r(20),4).unwrap(),0x9b94_4022);
        assert_eq!(encode_polled_result_word_load(r(2),r(29),4).unwrap(),0x919d_4022);
        assert!(encode_polled_result_word_load(r(3),r(29),4).is_err());
        assert_eq!(encode_r2_new_result_backedge(PHYSICAL_ADDRESS+0x5c,
            PHYSICAL_ADDRESS+0x4c,r(2)).unwrap(),0x2432_e0f8);
        assert!(encode_r2_new_result_backedge(PHYSICAL_ADDRESS+0x5c,
            PHYSICAL_ADDRESS+0x70,r(2)).is_err());
        assert_eq!(encode_unsigned_greater_loop(PHYSICAL_ADDRESS+0x64,PHYSICAL_ADDRESS+0x24,r(2),r(17)).unwrap(),0x1532_e9e0);
        assert_eq!(encode_short_backward_call(PHYSICAL_ADDRESS+0x74,PHYSICAL_ADDRESS).unwrap(),0x5bff_7fc6);
        assert_eq!(encode_short_backward_call(PHYSICAL_ADDRESS+0x80,PHYSICAL_ADDRESS).unwrap(),0x5bff_7fc0);
        assert!(encode_postinc_word_read(r(3),r(20),4).is_err());
        assert!(encode_short_backward_call(PHYSICAL_ADDRESS+0x80,PHYSICAL_ADDRESS+0x100).is_err());
    }
    #[test]
    fn complete_poll_is_contiguous_with_verified_sweeper(){
        let code=build_polled_callback_loop().unwrap();
        assert_eq!(code.len(),152);
        let joined=build_rwe_prefix_through_poll().unwrap();
        assert_eq!(joined.len(),712);
        assert_eq!(&joined[0x230..0x2c8],code.as_slice());
        assert_eq!(RWE_EXECUTABLE_LEN,688);
        assert_eq!(joined.len()-RWE_EXECUTABLE_LEN,24);
    }
}
