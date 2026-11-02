// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 entropy/seed-state initialization and callback guards,
//! file `[0x56364,0x563c0)` (92 independent executable bytes).
//!
//! A checked typed subset encodes register operations, GP addresses, packet
//! continuations and packet-relative calls. The later xorshift/update loop
//! starts at `0x563c0` and remains outside this partial function fragment.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_callback_completion::encode_minus_two;
use super::cdsp_late_callback_poll::encode_short_backward_call;
use super::cdsp_late_callback_sweeper::encode_predicate_from_register;
use super::cdsp_late_rng_init;

pub const FILE_OFFSET:usize=0x56364;
pub const END_FILE_OFFSET:usize=0x563c0;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8364;
pub const RWE_LAYOUT_START:usize=0x56000;
pub const RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_LAYOUT_START;
pub const PREFIX_EXECUTABLE_LEN:usize=cdsp_late_rng_init::EXECUTABLE_BYTES_IN_PREFIX+EXECUTABLE_LEN;
pub const PREFIX_LAYOUT_NOP_LEN:usize=cdsp_late_rng_init::NON_CODE_LAYOUT_BYTES;

fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// V66 `Rn=UPCYCLELO`, selected from the verified special-register family.
pub fn encode_upcyclelo(dst:G)->Result<u32,String>{
    if dst.number()!=22{return Err("CDSP seed update reads UPCYCLELO into R22".into());}
    Ok(0x6a0e_c000|u32::from(dst.number()))
}

/// Short 32-bit direct call to the CDSP timing controller. Hexagon call
/// immediate bits are split: 6 high bits and a 14-bit halfword displacement.
pub fn encode_positive_service_call(start:u32,target:u32)->Result<u32,String>{
    let bytes=target.checked_sub(start).ok_or("CDSP physical callback target below packet")?;
    if start!=PHYSICAL_ADDRESS+4||target!=0x96dc_0540||bytes&1!=0{
        return Err("Only verified CDSP controller timing service call is supported".into());
    }
    if bytes>0x1fff_fe {return Err("Hexagon split call displacement outside validated bounds".into());}
    Ok(0x5a00_4000|(((bytes>>15)&0x3f)<<16)|((bytes/2)&0x3fff))
}

/// GP-relative load of the single input-status flag at 0xc0364140, keeping
/// the original immext rather than LLVM6's compressed GP instruction.
pub fn encode_extended_gp_byte_flag(dst:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=2||address!=0xc036_4140{
        return Err("Seed-state GP flag read requires R2 and checked 0xc0364140 address".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let body=0x4920_c000|((address&0x3f)<<5)|u32::from(dst.number());
    Ok([ext,body])
}

/// `if(!P0.new) R0=-2` within the compound predicate-producing packet.
pub fn encode_if_not_new_p0_minus_two(dst:G,value:i32)->Result<u32,String>{
    if dst.number()!=0||value!=-2{return Err("Only guarded R0=-2 supported in seed-state packet".into());}
    Ok(0x7e8f_4000|(((value as u32)&0x1ff)<<5)|u32::from(dst.number()))
}

/// Conditional branch uses its original packet start `P0=R2` at `0x5637c`,
/// not the branch's own instruction address `0x56384`.
pub fn encode_new_p0_jump(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start!=0x96cc_837c||target!=0x96cc_83c0||target<=packet_start||
       (target-packet_start)>0x1fe||(target-packet_start)&1!=0 {
        return Err("Unsupported seed-state P0.new branch packet/target".into());
    }
    Ok(0x5c00_4800|((target-packet_start)/2))
}

/// `if(P0.new) R2=memw(R20+#0)` guard evaluated from same V66 packet.
pub fn encode_guarded_pointer_load(dst:G,base:G,offset:u8)->Result<u32,String>{
    if dst.number()!=2||base.number()!=20||offset!=0{
        return Err("Seed-state guarded pointer load requires R2 from R20+0".into());
    }
    Ok(0x4380_c000|(u32::from(base.number())<<16)|u32::from(dst.number()))
}

/// R2.new word store, source distance one/two earlier instructions within a
/// valid V66 packet. Base register must be one of the three RNG state slots.
pub fn encode_seed_word_store_new(base:G,producer:G,producer_distance:u8)->Result<u32,String>{
    if ![18,19,20].contains(&base.number())||producer.number()!=2||
       ![1,2].contains(&producer_distance){
        return Err("R2.new seed word store requires checked GP seed base/distance".into());
    }
    Ok(0xa1a0_d000|(u32::from(base.number())<<16)|(u32::from(producer_distance)<<9))
}

/// `memb(GP+##0xc0364140)=R23` with forced immext and packed byte-store form.
pub fn encode_gp_seed_flag_store(src:G,address:u32)->Result<[u32;2],String>{
    if src.number()!=23||address!=0xc036_4140{
        return Err("CDSP initialized seed flag must store R23 to GP 0xc0364140".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(src,address as i32,false)[0];
    let body=0x4800_c000|(u32::from(src.number())<<8)|((address&0x3f)<<5);
    Ok([ext,body])
}

/// The 23 original V66 words. Unlike an OEM opcode blob, every source word
/// is recomputed from checked architectural operands and packet markers.
pub fn build_seed_initialization()->Result<Vec<u8>,String>{
    let r=|n|G::new(n);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_upcyclelo(r(22)?)?);
    emit(&mut out,encode_positive_service_call(PHYSICAL_ADDRESS+4,0x96dc_0540)?);
    for w in cdsp_boot::encode_extended_assignment(r(0)?,0xc036_4130u32 as i32,true){emit(&mut out,w);}
    for w in encode_extended_gp_byte_flag(r(2)?,0xc036_4140)?{emit(&mut out,w);}
    emit(&mut out,M::ContinuationA.apply(encode_predicate_from_register(r(2)?)));
    emit(&mut out,encode_if_not_new_p0_minus_two(r(0)?,-2)?);
    emit(&mut out,encode_new_p0_jump(0x96cc_837c,0x96cc_83c0)?);
    emit(&mut out,encode_guarded_pointer_load(r(2)?,r(20)?,0)?);
    emit(&mut out,M::Final.apply(encode_short_backward_call(PHYSICAL_ADDRESS+0x28,0x96cc_8230)?));
    emit(&mut out,encode_short_backward_call(PHYSICAL_ADDRESS+0x2c,0x96cc_8230)?);
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(2)?,src:r(0)?,value:1}.encode()?));
    emit(&mut out,encode_minus_two(r(0)?,-2,M::ContinuationA)?);
    emit(&mut out,encode_seed_word_store_new(r(19)?,r(2)?,2)?);
    emit(&mut out,encode_short_backward_call(PHYSICAL_ADDRESS+0x3c,0x96cc_8230)?);
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(2)?,src:r(0)?,value:1}.encode()?));
    emit(&mut out,encode_minus_two(r(0)?,-2,M::ContinuationA)?);
    emit(&mut out,encode_seed_word_store_new(r(18)?,r(2)?,2)?);
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(2)?,src:r(0)?,value:1}.encode()?));
    emit(&mut out,encode_seed_word_store_new(r(20)?,r(2)?,1)?);
    for w in encode_gp_seed_flag_store(r(23)?,0xc036_4140)?{emit(&mut out,w);}
    if out.len()!=EXECUTABLE_LEN{return Err(format!("seed state initialization requires 92 bytes, got {}",out.len()));}
    Ok(out)
}

pub fn build_prefix_through_seed_state()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_rng_init::build_prefix_with_seed_prologue()?;
    out.extend(build_seed_initialization()?);
    if out.len()!=RWE_LAYOUT_LEN{return Err("CDSP 0x56000..0x563c0 RWE prefix length mismatch".into());}
    Ok(out)
}

pub fn verify_seed_initialization(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let code=build_seed_initialization()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(code.as_slice()) {
        return Err("CDSP seed initialization differs from pinned private firmware".into());
    }
    Ok(())
}

pub fn verify_seed_state_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let b=build_prefix_through_seed_state()?;
    if reference.get(RWE_LAYOUT_START..END_FILE_OFFSET)!=Some(b.as_slice()){
        return Err("CDSP RNG initialization prefix through 0x563c0 differs from pinned reference".into());
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    fn r(n:u8)->G{G::new(n).unwrap()}
    #[test]
    fn seed_callbacks_and_predicates_use_checked_hexagon_fields(){
        assert_eq!(encode_upcyclelo(r(22)).unwrap(),0x6a0e_c016);
        assert!(encode_upcyclelo(r(21)).is_err());
        assert_eq!(encode_positive_service_call(0x96cc8368,0x96dc0540).unwrap(),0x5a1f_40ec);
        assert!(encode_positive_service_call(0x96cc8368,0x96dc0544).is_err());
        assert_eq!(encode_extended_gp_byte_flag(r(2),0xc0364140).unwrap(),[0x0c03_5905,0x4920_c002]);
        assert_eq!(encode_if_not_new_p0_minus_two(r(0),-2).unwrap(),0x7e8f_7fc0);
        assert_eq!(encode_new_p0_jump(0x96cc837c,0x96cc83c0).unwrap(),0x5c00_4822);
        assert_eq!(encode_guarded_pointer_load(r(2),r(20),0).unwrap(),0x4394_c002);
        assert_eq!(encode_seed_word_store_new(r(19),r(2),2).unwrap(),0xa1b3_d400);
        assert_eq!(encode_seed_word_store_new(r(20),r(2),1).unwrap(),0xa1b4_d200);
        assert!(encode_seed_word_store_new(r(20),r(2),3).is_err());
        assert_eq!(encode_gp_seed_flag_store(r(23),0xc0364140).unwrap(),[0x0c03_5905,0x4800_d700]);
    }
    #[test]
    fn rng_init_has_no_unaccounted_padding_and_remains_partial(){
        assert_eq!(build_seed_initialization().unwrap().len(),92);
        assert_eq!(build_prefix_through_seed_state().unwrap().len(),0x3c0);
        assert_eq!(PREFIX_EXECUTABLE_LEN,928);
        assert_eq!(PREFIX_LAYOUT_NOP_LEN,32);
    }
}
