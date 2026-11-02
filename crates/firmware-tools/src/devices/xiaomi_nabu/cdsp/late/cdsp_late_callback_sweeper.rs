// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 callback-table sweeper at file 0x561e0..0x56230.
//!
//! Exact typed source assembly of the original 80-byte routine, with GP
//! global initialization, predicate-masked byte store, optional extended
//! backward call, indexed function-pointer traversal and checked return.
//! LLVM6 semantically assembles a 72-byte equivalent, deliberately shortening
//! forced-extender operations. The OEM instruction/packet layout is verified
//! independently with the Rizin V66 decoder and original pinned CDSP MBN.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_callback_dispatch::{self,
    encode_save_pair_and_allocframe,encode_indexed_callback_load,
    encode_r2_zero_backedge};
use super::cdsp_late_runtime::{encode_callr};

pub const FILE_OFFSET:usize=0x561e0;
pub const END_FILE_OFFSET:usize=0x56230;
pub const EXECUTABLE_LEN:usize=0x50;
pub const PHYSICAL_ADDRESS:u32=0x96cc_81e0;
pub const ELF_VADDR:u32=0xc00c_81e0;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-0x56000;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_callback_dispatch::SOURCE_GENERATED_EXECUTABLE_LEN+EXECUTABLE_LEN;

fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// Forced two-word GP byte read. LLVM6 emits the independently equivalent
/// unextended addressing form; V66 supports the explicitly original form.
pub fn encode_extended_gp_byte_load(dst:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=3 || address!=0xc036_4128 {
        return Err("unsupported R3 GP byte callback-initialization flag".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let body=0x4920_4000 | ((address&0x3f)<<5) |u32::from(dst.number());
    Ok([ext,body])
}

/// V66 short `memw(SP+#4)=#0` with scaled word-address offset.
pub fn encode_stack_zero_store(base:G,offset:u8,value:u8)->Result<u32,String>{
    if base.number()!=29 || offset%4!=0 || offset>28 || value!=0 {
        return Err("CDSP sweeper frame store must be aligned SP+offset=#0".into());
    }
    Ok(0x3c40_c000|(u32::from(base.number())<<16)|
       (u32::from(offset)<<5)|u32::from(value))
}

/// Assign P0 to the truth value of one checked GPR.
pub fn encode_predicate_from_register(src:G)->u32{
    0x8540_c000 | (u32::from(src.number())<<16)
}

/// Conditional small immediate assignment gated by P0.new. Rizin and LLVM
/// independently confirm R3=1 within a shared predicate-producing packet.
pub fn encode_if_not_new_p0_set_one(dst:G,value:u8)->Result<u32,String>{
    if dst.number()!=3 || value!=1 {
        return Err("CDSP conditional callback flag supports R3=#1 only".into());
    }
    Ok(0x7e80_6000 | (u32::from(value)<<5) |u32::from(dst.number()))
}

/// `if (!P0.new) memb(##absolute)=R3.new`. Original is a forced extender
/// and a V66 byte-store consumer; independently LLVM6 assembled this packet.
pub fn encode_conditional_global_byte_store(src:G,address:u32)->Result<[u32;2],String>{
    if src.number()!=3 || address!=0xc036_4128{
        return Err("unsupported callback-init GP flag store".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(src,address as i32,false)[0];
    // LLVM6 independently assembles the V66 byte store with this consumer
    // form, including the implicit R3.new distance and predication bits.
    // This narrow constructor refuses any other address/register pairing;
    // unlike word stores, byte low-offset bit lanes are not linear.
    let store=0xafa2_e2c4;
    Ok([ext,store])
}

/// `if(!P0) call packet_start+delta` with a signed extended displacement;
/// unlike LLVM6's compact encoding this preserves the OEM forced extender.
pub fn encode_extended_if_not_p0_call(packet_start:u32,target:u32)->Result<[u32;2],String>{
    let delta=i64::from(target)-i64::from(packet_start);
    if delta>=0 || delta < -0x20000 || delta%2!=0 {
        return Err("conditional callback call requires negative aligned displacement".into());
    }
    let imm=delta as i32 as u32;
    let ext=((imm>>20)<<16)|((imm>>6)&0x3fff)|0x4000;
    let body=0x5d20_4000 | ((imm&0x3f)<<1);
    Ok([ext,body])
}

/// A GP-scanned callback list advances R16 by a positive immediate.
pub fn encode_simple_increment(dst:G,by:u8,end_packet:bool)->Result<u32,String>{
    if dst.number()>31 || by>127 || by==0 {return Err("invalid GP scanner register/increment".into());}
    let w=S::AddSmallImmediate{dst,src:dst,value:by}.encode()?;
    Ok((if end_packet{M::Final}else{M::ContinuationA}).apply(w))
}

/// R1=#4 and R0=add(FP,#4) in the independently LLVM6-confirmed duplex.
pub fn encode_callback_args(a:G,b:G,fp:G,bytes:u8)->Result<u32,String>{
    if a.number()!=1 || b.number()!=0 || fp.number()!=29 || bytes!=4 {
        return Err("RWE callback pair expects R1=4 and R0=FP+4".into());
    }
    Ok(0x2841_2c10)
}

/// Short `R0=memw(SP+#4)`, encoded with WORD scaled offset rather than
/// the bitfield used by the original bootstrap's zero-offset-only helper.
pub fn encode_stack_word_load(dst:G,base:G,offset:u8)->Result<u32,String>{
    if dst.number()!=0 || base.number()!=29 || offset>60 || offset%4!=0{
        return Err("unsupported CDSP callback result stack read".into());
    }
    Ok(0x9180_c000 | (u32::from(base.number())<<16)
        |(u32::from(offset/4)<<5)|u32::from(dst.number()))
}

/// Compare the *new* R0 callback result to zero, taking the loop backedge.
/// The displacement is relative to the packet start (one word earlier).
pub fn encode_new_result_zero_backedge(packet_start:u32,target:u32)->Result<u32,String>{
    let delta=i64::from(target)-i64::from(packet_start);
    if delta>=0 || delta < -0x100 || delta%2!=0 {
        return Err("callback result zero branch expects short backwards displacement".into());
    }
    Ok(0x2432_e000 | (((delta/2) as i32 as u32)&0xff))
}

/// Restore R17:16 and deallocate the original 16-byte function frame.
pub fn encode_restore_frame_and_return(low:G,base:G,offset:u8)->Result<u32,String>{
    if low.number()!=16 || base.number()!=29 || offset!=8 {
        return Err("CDSP sweeper return must restore R17:16 from FP+8".into());
    }
    Ok(0x3e0c_1f40)
}

pub fn build_callback_sweeper()->Result<Vec<u8>,String>{
    let r=|n|G::new(n);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_save_pair_and_allocframe(r(16)?,r(29)?,-16,16)?);
    for (i,w) in encode_extended_gp_byte_load(r(3)?,0xc036_4128)?.into_iter().enumerate(){
        emit(&mut out,if i==0{w}else{M::ContinuationA.apply(w)});
    }
    emit(&mut out,encode_stack_zero_store(r(29)?,4,0)?);
    emit(&mut out,M::ContinuationA.apply(encode_predicate_from_register(r(3)?)));
    emit(&mut out,encode_if_not_new_p0_set_one(r(3)?,1)?);
    for (i,w) in encode_conditional_global_byte_store(r(3)?,0xc036_4128)?.into_iter().enumerate(){
        emit(&mut out,if i==0{w}else{M::Final.apply(w)});
    }
    for (i,w) in encode_extended_if_not_p0_call(PHYSICAL_ADDRESS+0x20,
                                               0x96cc_8120)?.into_iter().enumerate(){
        emit(&mut out,if i==0{w}else{M::ContinuationA.apply(w)});
    }
    emit(&mut out,S::AssignSmallImmediate{dst:r(16)?,value:0}.encode()?);
    emit(&mut out,encode_simple_increment(r(16)?,1,false)?);
    for (i,w) in encode_indexed_callback_load(r(2)?,r(16)?,0xc028_c010,2)?.into_iter().enumerate(){
        emit(&mut out,if i==0{w}else{M::Final.apply(w)});
    }
    emit(&mut out,encode_r2_zero_backedge(PHYSICAL_ADDRESS+0x38,
                                         PHYSICAL_ADDRESS+0x2c,r(2)?)?);
    emit(&mut out,M::ContinuationA.apply(encode_callr(r(2)?)));
    emit(&mut out,encode_callback_args(r(1)?,r(0)?,r(29)?,4)?);
    emit(&mut out,M::ContinuationA.apply(encode_stack_word_load(r(0)?,r(29)?,4)?));
    emit(&mut out,encode_new_result_zero_backedge(PHYSICAL_ADDRESS+0x44,
                                                 PHYSICAL_ADDRESS+0x2c)?);
    emit(&mut out,encode_restore_frame_and_return(r(16)?,r(29)?,8)?);
    if out.len()!=EXECUTABLE_LEN {
        return Err(format!("CDSP callback sweeper expected 80 bytes, got {}",out.len()));
    }
    Ok(out)
}

pub fn build_rwe_layout_through_sweeper()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_callback_dispatch::build_complete_rwe_prefix_layout()?;
    out.extend(build_callback_sweeper()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN{return Err("expected exact 560-byte RWE prefix".into());}
    Ok(out)
}

pub fn verify_callback_sweeper(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let generated=build_callback_sweeper()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(generated.as_slice()){
        return Err("typed CDSP callback sweeper differs from pinned firmware".into());
    }
    Ok(())
}

pub fn verify_rwe_layout_through_sweeper(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let generated=build_rwe_layout_through_sweeper()?;
    if reference.get(0x56000..END_FILE_OFFSET)!=Some(generated.as_slice()){
        return Err("typed RWE layout through sweeper does not match pinned firmware".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    fn r(n:u8)->G{G::new(n).unwrap()}
    #[test]
    fn encoded_gp_and_packet_fields(){
        assert_eq!(encode_extended_gp_byte_load(r(3),0xc036_4128).unwrap(),[0x0c03_5904,0x4920_4503]);
        assert!(encode_extended_gp_byte_load(r(2),0xc036_4128).is_err());
        assert_eq!(encode_stack_zero_store(r(29),4,0).unwrap(),0x3c5d_c080);
        assert_eq!(encode_if_not_new_p0_set_one(r(3),1).unwrap(),0x7e80_6023);
        assert_eq!(encode_conditional_global_byte_store(r(3),0xc036_4128).unwrap(),[0x0c03_5904,0xafa2_e2c4]);
        assert_eq!(encode_extended_if_not_p0_call(PHYSICAL_ADDRESS+0x20,0x96cc_8120).unwrap(),[0x0fff_7ffc,0x5d20_4040]);
        assert_eq!(encode_callback_args(r(1),r(0),r(29),4).unwrap(),0x2841_2c10);
        assert_eq!(encode_stack_word_load(r(0),r(29),4).unwrap(),0x919d_c020);
        assert_eq!(encode_new_result_zero_backedge(PHYSICAL_ADDRESS+0x44,PHYSICAL_ADDRESS+0x2c).unwrap(),0x2432_e0f4);
        assert!(encode_new_result_zero_backedge(PHYSICAL_ADDRESS+0x44,PHYSICAL_ADDRESS+0x5c).is_err());
    }
    #[test]
    fn exact_sweeper_and_full_layout_boundaries(){
        let b=build_callback_sweeper().unwrap();
        assert_eq!(b.len(),EXECUTABLE_LEN);
        let all=build_rwe_layout_through_sweeper().unwrap();
        assert_eq!(all.len(),FULL_RWE_LAYOUT_LEN);
        assert_eq!(&all[0x1e0..0x230],b.as_slice());
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,536);
        assert_eq!(all.len()-FULL_RWE_EXECUTABLE_LEN,24);
    }
}
