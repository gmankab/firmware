// SPDX-License-Identifier: GPL-3.0-only
//! CDSP supervisor-guard cleanup after the callback burst. Reads three
//! guarded GP addresses and clears controller state only when all are ready.
//! Packet boundaries and compact duplex operands are rebuilt from typed V66
//! encodings, not embedded reference instruction words.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_worker_final_calls as previous;

pub const FILE_OFFSET:usize=0x4fd0;
pub const REGION_END:usize=0x5004;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1fd0;
fn put(out:&mut Vec<u8>,word:u32,mark:P){out.extend_from_slice(&mark.apply(word).to_le_bytes());}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum GuardCondition { Zero, NonZero }
/// A producer-new-value predicate in a three-word GP-load packet. Addressing
/// starts at the first packet word rather than the compound branch word.
fn conditional_gp_guard(packet:u32,exit:u32,zero:GuardCondition)->Result<u32,String>{
    if packet%4!=0||exit%4!=0||exit<=packet||exit-packet>0x1fe {
        return Err("CDSP GP state guard requires aligned bounded forward branch".into());
    }
    let branch=match zero {GuardCondition::Zero=>0x2402_e000,GuardCondition::NonZero=>0x2402_c000};
    Ok(branch|(exit-packet)/2)
}
/// V66 compact paired stores of zero from the small indexed GPR base.
fn clear_two_words(base:Gpr,first:u8,second:u8)->Result<u32,String>{
    if base.number()>7||first>124||second>124||first%4!=0||second%4!=0{
        return Err("CDSP zero-store pair requires compact GPR and aligned offsets".into());
    }
    Ok(0xf000_1000|(u32::from(base.number())<<20)|(u32::from(first/4)<<16)
       |(u32::from(base.number())<<4)|u32::from(second/4))
}
fn clear_mem_bit(base:Gpr,bit:u8)->Result<u32,String>{
    if base.number()>7||bit>31{return Err("CDSP memory clrbit register or bit invalid".into());}
    Ok(0x3f40_c0c0|(u32::from(base.number())<<16)|u32::from(bit))
}

pub fn build_guard_exit()->Result<Vec<u8>,String>{
    let r0=Gpr::new(0)?;
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    for (here,gp,condition) in [
        (0x96c0_1fd0,0xfe0a_c0a0,GuardCondition::Zero),
        (0x96c0_1fdc,0xfe0a_5ddc,GuardCondition::Zero),
        (0x96c0_1fe8,0xfe03_7ad0,GuardCondition::NonZero),
    ] {
        let load=cdsp_boot::encode_extended_gp_word_load(r0,gp);
        put(&mut out,load[0],P::ContinuationA);
        put(&mut out,load[1],P::ContinuationA);
        put(&mut out,conditional_gp_guard(here,0x96c0_2000,condition)?,P::Final);
    }
    // Four zero stores coalesced into two compact Hexagon duplex instructions.
    for (a,b) in [(36,32),(16,20)] {
        out.extend_from_slice(&clear_two_words(r0,a,b)?.to_le_bytes());
    }
    put(&mut out,clear_mem_bit(r0,21)?,P::Final);
    // Deliberate self-jump is the controller's terminal guard state.
    put(&mut out,H::JumpRelative{displacement_bytes:0}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP cleanup guard has incorrect byte count".into());}
    Ok(out)
}

pub fn verify_reference(ref_image:&[u8])->Result<(),String>{
    previous::verify_reference(ref_image)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("CDSP worker guard exit not adjacent to callbacks".into());}
    let ours=build_guard_exit()?;
    if ref_image.get(FILE_OFFSET..REGION_END)!=Some(ours.as_slice()){
        return Err("CDSP worker state cleanup not byte-identical to reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn guards_duplexes_and_self_jump_are_bounded(){
        let r0=Gpr::new(0).unwrap();
        assert_eq!(FUNCTION_BYTES,52);
        assert_eq!(build_guard_exit().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(conditional_gp_guard(0x96c0_1fd0,0x96c0_2000,GuardCondition::Zero).unwrap(),0x2402_e018);
        assert_eq!(conditional_gp_guard(0x96c0_1fe8,0x96c0_2000,GuardCondition::NonZero).unwrap(),0x2402_c00c);
        assert_eq!(clear_two_words(r0,36,32).unwrap(),0xf009_1008);
        assert_eq!(clear_mem_bit(r0,21).unwrap(),0x3f40_c0d5);
        assert!(conditional_gp_guard(0x96c0_2000,0x96c0_2000,GuardCondition::Zero).is_err());
        assert!(clear_two_words(r0,13,12).is_err());
        assert!(clear_mem_bit(r0,32).is_err());
    }
}
