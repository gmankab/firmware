// SPDX-License-Identifier: GPL-3.0-only
//! CDSP controller bitplane pointer and bank-mask setup, file 0x56734..0x56784.
//!
//! The two V66 duplex operations are source-encoded under checked register
//! constraints and have also been independently assembled with LLVM6.
//! The GP access retains the source firmware's *extended* form (LLVM6 may
//! choose a shorter form). Original packet parse bits are explicit. This is
//! still inside the controller function begun at file 0x56600.
use super::cdsp_boot::{self, GeneralRegister as G, PacketMarker as M, ScalarPacket as S};
use super::cdsp_late_controller_bitloop;

pub const FILE_OFFSET: usize = 0x56734;
pub const END_FILE_OFFSET: usize = 0x56784;
pub const EXECUTABLE_LEN: usize = END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS: u32 = 0x96cc_8734;
pub const RWE_START: usize = cdsp_late_controller_bitloop::RWE_START;
pub const FULL_RWE_LAYOUT_LEN: usize = END_FILE_OFFSET-RWE_START;
pub const RWE_PREFIX_LAYOUT_LEN: usize = cdsp_late_controller_bitloop::RWE_LAYOUT_NOP_LEN;
pub const FULL_RWE_EXECUTABLE_LEN: usize = cdsp_late_controller_bitloop::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
const CONTROLLER_GP_BANK: u32 = 0xfe0c_6680;
const PLANE_HEADROOM: u32 = 0x003f_ffff;
const BANK_HIGH_MASK: u32 = 0xffc0_0000;

fn r(i: u8) -> Result<G,String> { G::new(i) }
fn emit(out: &mut Vec<u8>, word: u32) {out.extend_from_slice(&word.to_le_bytes());}

/// Independent LLVM6 duplex `R2=memw(R19+0); R3=memw(R20+0)`.
/// The verified register tuple avoids arbitrary untested duplex fields.
pub fn encode_duplex_plane_pointer_load(first_dst:G,first_base:G,
                                        second_dst:G,second_base:G)->Result<u32,String>{
    if (first_dst.number(),first_base.number(),second_dst.number(),second_base.number())!=(2,19,3,20) {
        return Err("unsupported duplex plane-pointer register mapping".into());
    }
    // Low-register half-block bits established by LLVM6 V60 fixture.
    Ok(0x00b2_0000 | (u32::from(second_dst.number()) << 6)
       | u32::from(first_dst.number()) | (u32::from(first_base.number()&1) << 0))
}

/// LLVM6 source: `R19=R0; R2=memw(R19+0)` sharing one duplex word.
pub fn encode_duplex_handle_and_word(copy_to:G,copy_from:G,load_dst:G,load_base:G)->Result<u32,String>{
    if (copy_to.number(),copy_from.number(),load_dst.number(),load_base.number())!=(19,0,2,19){
        return Err("unsupported duplex handle transfer".into());
    }
    Ok(0x500b_0000 | (u32::from(load_dst.number()) << 6) | 0x32)
}

/// Two absolute controller calls use packet-relative halfword displacements.
/// The second call's packet has an earlier LSR producer and must continue.
pub fn encode_controller_lookup_call(from:u32,target:u32,continue_packet:bool)->Result<u32,String>{
    if ![(0x96cc_8740,false),(0x96cc_874c,true)].contains(&(from,continue_packet)) || target!=0x96cc_aa38 {
        return Err("unverified controller lookup call/packet location".into());
    }
    let distance=target.checked_sub(from).ok_or("lookup call direction incorrect")?;
    if distance&1!=0 || distance>0x1fff_fe{return Err("lookup branch displacement exceeds encoder".into());}
    let standalone=0x5a00_c000 | (((distance>>15)&0x3f)<<16) | ((distance>>1)&0x3fff);
    Ok((if continue_packet { M::ContinuationA } else { M::Final }).apply(standalone))
}

/// Checked load with 4-byte word offsets. Original packet grouping is chosen
/// by the caller and is not inferred by the compiler's scheduler.
pub fn encode_word_deref(dst:G,base:G,offset_bytes:u8)->Result<u32,String>{
    if ![(0,2,0),(22,20,0)].contains(&(dst.number(),base.number(),offset_bytes)) {
        return Err("unsupported controller word dereference".into());
    }
    Ok(0x9180_c000 | (u32::from(base.number())<<16) |
       (u32::from(offset_bytes)<<3) | u32::from(dst.number()))
}

/// Byte-exact source-generated fragment, including packet markers and
/// forced extended GP forms but no OEM instruction-word array.
pub fn build_bank_setup()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_duplex_plane_pointer_load(r(2)?,r(19)?,r(3)?,r(20)?)?);
    emit(&mut out,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(1)?,src:r(3)?,amount:12}.encode()?));
    emit(&mut out,encode_word_deref(r(0)?,r(2)?,0)?);
    emit(&mut out,encode_controller_lookup_call(PHYSICAL_ADDRESS+0x0c,0x96cc_aa38,false)?);
    emit(&mut out,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(1)?,src:r(20)?,amount:12}.encode()?));
    emit(&mut out,encode_duplex_handle_and_word(r(19)?,r(0)?,r(2)?,r(19)?)?);
    emit(&mut out,encode_controller_lookup_call(PHYSICAL_ADDRESS+0x18,0x96cc_aa38,true)?);
    emit(&mut out,encode_word_deref(r(0)?,r(2)?,0)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignRegister{dst:r(18)?,src:r(0)?}.encode()?));
    for w in cdsp_boot::encode_extended_gp_word_load(r(3)?,CONTROLLER_GP_BANK){
        // The original GP load shares its packet with a following dereference.
        let word=if w&0xf000_0000==0x4000_0000 {M::ContinuationA.apply(w)} else {w};
        emit(&mut out,word);
    }
    emit(&mut out,encode_word_deref(r(22)?,r(20)?,0)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignUnsignedImmediate{dst:r(17)?,value:32}.encode()?));
    for w in cdsp_boot::encode_extended_add(r(2)?,r(3)?,PLANE_HEADROOM as i32,true){emit(&mut out,w);}
    // AND ##-0x400000 with mask field extended from 32-bit signed value.
    let and_extension=cdsp_boot::encode_extended_assignment(r(21)?,BANK_HIGH_MASK as i32,false)[0];
    emit(&mut out,and_extension);
    emit(&mut out,M::ContinuationA.apply(0x7600_c000 | (u32::from(r(2)?.number())<<16)
                                     | u32::from(r(21)?.number())));
    emit(&mut out,S::AssignUnsignedImmediate{dst:r(2)?,value:1}.encode()?);
    emit(&mut out,M::ContinuationA.apply(S::InsertBitField{dst:r(22)?,src:r(21)?,width:20,offset:0}.encode()?));
    emit(&mut out,S::LogicalShiftRight{dst:r(23)?,src:r(21)?,amount:12}.encode()?);
    if out.len()!=EXECUTABLE_LEN{return Err("controller pointer/bank setup must be 80 bytes".into());}
    Ok(out)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_bitloop::build_prefix()?;
    out.extend(build_bank_setup()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN ||
       out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN {
        return Err("controller pointer setup prefix size/code/fill mismatch".into());
    }
    Ok(out)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bank_setup()?.as_slice()){
        return Err("controller pointer setup differs from pinned firmware".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("controller pointer prefix differs from pinned firmware".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn fields_and_duplex_packets_reject_unverified_operands(){
        let out=build_bank_setup().unwrap();assert_eq!(out.len(),80);
        let words:Vec<u32>=out.chunks_exact(4).map(|w|u32::from_le_bytes(w.try_into().unwrap())).collect();
        assert_eq!(words[0],0x00b2_00c3);
        assert_eq!(words[5],0x500b_00b2);
        assert_eq!(words[9],0x0fe0_719a);
        assert_eq!(words[10],0x4980_4003);
        assert!(encode_duplex_plane_pointer_load(r(3).unwrap(),r(19).unwrap(),r(2).unwrap(),r(20).unwrap()).is_err());
        assert!(encode_controller_lookup_call(0x96cc_8740,0x96cc_aa3c,false).is_err());
        assert!(encode_word_deref(r(0).unwrap(),r(3).unwrap(),0).is_err());
    }
    #[test]fn layout_counts_code_separately(){
        assert_eq!(build_prefix().unwrap().len(),0x784);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1880);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,44);
    }
}
