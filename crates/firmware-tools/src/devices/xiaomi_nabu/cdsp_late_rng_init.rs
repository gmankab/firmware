// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 controller seed/update routine prologue, file 0x56320..0x56364.
//!
//! The original Hexagon hardware/RNG routine continues after 0x56364;
//! this is only an independently encoded source fragment and not callable
//! stand-alone. The source-build retains packet ordering and forced 32-bit
//! GP addressing rejected/shortened by older LLVM6 instruction scheduling.
use super::cdsp_boot::{self, GeneralRegister as G, PacketMarker as M, ScalarPacket as S};
use super::cdsp_late_callback_completion::{self};

pub const FILE_OFFSET:usize=0x56320;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8320;
pub const VERIFIED_LEN:usize=0x44;
pub const END_FILE_OFFSET:usize=FILE_OFFSET+VERIFIED_LEN;
pub const RWE_LAYOUT_END:usize=END_FILE_OFFSET;
pub const RWE_LAYOUT_LEN:usize=RWE_LAYOUT_END-0x56000;
pub const EXECUTABLE_BYTES_IN_PREFIX:usize=cdsp_late_callback_completion::RWE_EXECUTABLE_LEN+VERIFIED_LEN;
pub const NON_CODE_LAYOUT_BYTES:usize=cdsp_late_callback_completion::RWE_FILL_BYTES;

fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// `allocframe(SP,#48)` at entry, uses the confirmed scaled-eight-bit unit.
pub fn encode_raw_allocframe48(stack_bytes:u16)->Result<u32,String>{
    if stack_bytes!=48{return Err("CDSP RNG routine requires exactly 48-byte frame".into());}
    Ok(0xa09d_c000|u32::from(stack_bytes/8))
}

/// `R17:16=combine(R0,R3)` in original two-slot controller packet.
pub fn encode_combine_r3_r0(low:G,first:G,second:G)->Result<u32,String>{
    if low.number()!=16 ||first.number()!=0 ||second.number()!=3 {
        return Err("CDSP RNG input pair must combine R0 and R3 into R17:16".into());
    }
    Ok(0xf500_4000|(u32::from(second.number())<<8)|u32::from(low.number()))
}

/// Pairwise save of two register pairs on a 48-byte FP frame, with nibble
/// immediates independent of the referenced machine-code bytes.
pub fn encode_save_two_pairs(fp:G,first_low:G,second_low:G,first_off:u8,second_off:u8)->Result<u32,String>{
    if fp.number()!=29 || ![(16,18,40,32),(20,22,24,16)].contains(&
        (first_low.number(),second_low.number(),first_off,second_off)) {
        return Err("Unsupported CDSP RNG save-pair registers/offsets".into());
    }
    Ok(if first_low.number()==16 {0xea2c_0a25} else {0xea1e_0a17})
}

/// Conditional assignment of a checked extended absolute address to R20;
/// the immediately preceding immext encodes the high 26 address bits.
pub fn encode_if_not_p0_set_extended(dst:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=20 ||address!=0xc036_4158 {
        return Err("Unsupported guarded CDSP RNG pointer".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let consumer=0x7e80_4000|((address&0x3f)<<5)|u32::from(dst.number());
    Ok([ext,consumer])
}

/// `if (!P0) R23=#1` and `if(P0) R0=#0` in two original packet slots.
pub fn encode_predicated_small_assignment(dst:G,value:u8,when_zero:bool,last:bool)->Result<u32,String>{
    if value>1 || ![(23,1,false),(0,0,true)].contains(&(dst.number(),value,when_zero)){
        return Err("Only validated CDSP P0-gated assignments are supported".into());
    }
    let base=if when_zero{0x7e00_c000}else{0x7e80_c000};
    let body=base|(u32::from(value)<<5)|u32::from(dst.number());
    Ok((if last{M::Final}else{M::ContinuationA}).apply(body))
}

/// `if(P0) jump:nt` packet-relative halfword displacement; the Hexagon
/// packet begins at the earlier R23 assignment, not at the branch consumer.
pub fn encode_p0_jump_not_taken(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start!=0x96cc_8344 ||target!=0x96cc_8438 || target<=packet_start ||
       target-packet_start>0x1fe ||(target-packet_start)&1!=0{
        return Err("Unsupported CDSP RNG branch packet/target".into());
    }
    Ok(0x5c00_4000|(target-packet_start)/2)
}

/// Store R25:24 to SP+#8, with only checked register/stack addressing.
pub fn encode_r25_pair_store(low:G,stack:G,offset:u8)->Result<u32,String>{
    if low.number()!=24||stack.number()!=29||offset!=8{
        return Err("CDSP RNG stack spill only supports R25:24 at SP+8".into());
    }
    Ok(0xa1dd_d000|(u32::from(offset)<<8)|u32::from(low.number()-23))
}

/// Extended signed address in R18 or R19; value fields originate from the
/// named CDSP global RNG addresses and preserve the original parse markers.
pub fn encode_rng_pointer(dst:G,address:u32,last:bool)->Result<[u32;2],String>{
    if ![(18,0xc036_4150),(19,0xc036_4148)].contains(&(dst.number(),address)){
        return Err("Unsupported CDSP seed-array register/absolute address".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let body=0x7800_c000|((address&0x3f)<<5)|u32::from(dst.number());
    Ok([ext, if last {M::Final.apply(body)} else {M::ContinuationA.apply(body)}])
}

pub fn build_seed_prologue()->Result<Vec<u8>,String>{
    let r=|n|G::new(n);
    let mut out=Vec::with_capacity(VERIFIED_LEN);
    for (index,word) in cdsp_boot::encode_extended_gp_word_load(r(2)?,0xfe0a_d660).into_iter().enumerate(){
        emit(&mut out,if index==0{word}else{M::ContinuationA.apply(word)});
    }
    emit(&mut out,encode_raw_allocframe48(48)?);
    emit(&mut out,M::ContinuationA.apply(S::PredicateP0EqualsZero{src:r(2)?}.encode()?));
    emit(&mut out,M::ContinuationA.apply(encode_combine_r3_r0(r(16)?,r(0)?,r(3)?)?));
    emit(&mut out,encode_save_two_pairs(r(29)?,r(16)?,r(18)?,40,32)?);
    for w in encode_if_not_p0_set_extended(r(20)?,0xc036_4158)?{emit(&mut out,w);}
    emit(&mut out,encode_save_two_pairs(r(29)?,r(20)?,r(22)?,24,16)?);
    emit(&mut out,encode_predicated_small_assignment(r(23)?,1,false,false)?);
    emit(&mut out,encode_predicated_small_assignment(r(0)?,0,true,false)?);
    emit(&mut out,encode_p0_jump_not_taken(0x96cc_8344,0x96cc_8438)?);
    emit(&mut out,encode_r25_pair_store(r(24)?,r(29)?,8)?);
    for w in encode_rng_pointer(r(18)?,0xc036_4150,false)?{emit(&mut out,w);}
    for w in encode_rng_pointer(r(19)?,0xc036_4148,true)?{emit(&mut out,w);}
    if out.len()!=VERIFIED_LEN{return Err(format!("CDSP seed prologue generated {} bytes, expected {}",out.len(),VERIFIED_LEN));}
    Ok(out)
}

pub fn build_prefix_with_seed_prologue()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_callback_completion::build_prefix_with_completion()?;
    out.extend(build_seed_prologue()?);
    if out.len()!=RWE_LAYOUT_LEN {return Err("CDSP contiguous 0x56000..0x56364 layout mismatch".into());}
    Ok(out)
}

pub fn verify_seed_prologue(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let b=build_seed_prologue()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(b.as_slice()){
        return Err("CDSP seed-routine prologue differs from pinned reference".into());
    }
    Ok(())
}

pub fn verify_prefix_with_seed_prologue(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let b=build_prefix_with_seed_prologue()?;
    if reference.get(0x56000..END_FILE_OFFSET)!=Some(b.as_slice()){
        return Err("CDSP RWE prefix through RNG initialization differs from reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    fn r(i:u8)->G{G::new(i).unwrap()}
    #[test]
    fn seed_generator_has_checked_field_encodings(){
        assert_eq!(encode_raw_allocframe48(48).unwrap(),0xa09d_c006);
        assert!(encode_raw_allocframe48(24).is_err());
        assert_eq!(encode_combine_r3_r0(r(16),r(0),r(3)).unwrap(),0xf500_4310);
        assert_eq!(encode_save_two_pairs(r(29),r(16),r(18),40,32).unwrap(),0xea2c_0a25);
        assert_eq!(encode_save_two_pairs(r(29),r(20),r(22),24,16).unwrap(),0xea1e_0a17);
        assert_eq!(encode_if_not_p0_set_extended(r(20),0xc0364158).unwrap(),[0x0c03_5905,0x7e80_4314]);
        assert_eq!(encode_predicated_small_assignment(r(23),1,false,false).unwrap(),0x7e80_4037);
        assert_eq!(encode_predicated_small_assignment(r(0),0,true,false).unwrap(),0x7e00_4000);
        assert_eq!(encode_p0_jump_not_taken(0x96cc8344,0x96cc8438).unwrap(),0x5c00_407a);
        assert_eq!(encode_r25_pair_store(r(24),r(29),8).unwrap(),0xa1dd_d801);
        assert_eq!(encode_rng_pointer(r(18),0xc0364150,false).unwrap(),[0x0c03_5905,0x7800_4212]);
        assert_eq!(encode_rng_pointer(r(19),0xc0364148,true).unwrap(),[0x0c03_5905,0x7800_c113]);
        assert!(encode_rng_pointer(r(19),0xc0364150,false).is_err());
    }
    #[test]
    fn seed_partial_does_not_claim_full_prng_routine(){
        let b=build_seed_prologue().unwrap();assert_eq!(b.len(),68);
        let prefix=build_prefix_with_seed_prologue().unwrap();
        assert_eq!(prefix.len(),0x364);
        assert_eq!(EXECUTABLE_BYTES_IN_PREFIX,836);
        assert_eq!(NON_CODE_LAYOUT_BYTES,32);
    }
}
