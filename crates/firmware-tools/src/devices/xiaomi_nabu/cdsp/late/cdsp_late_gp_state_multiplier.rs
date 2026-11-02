// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 ten-word GP-state multiplier and caller result publication.
//!
//! File [0x56540,0x56568): signed multiplier constant, P0.new loop,
//! GP-relative source/destination with forced immext and duplex scalar
//! initialization. Source words are re-encoded from bounded operands.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_callback_service;

pub const FILE_OFFSET:usize=0x56540;
pub const END_FILE_OFFSET:usize=0x56568;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc8540;
pub const RWE_LAYOUT_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_LAYOUT_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_callback_service::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_callback_service::RWE_LAYOUT_NOP_LEN;
fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// Read R3 from absolute GP status word 0xc028c02c, preserving original
/// forced extender even where LLVM6 chooses a shorter GP-relative load.
pub fn encode_extended_gp_state_load(dst:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=3||address!=0xc028_c02c{return Err("State load expects R3 from GP 0xc028c02c".into());}
    let ext=cdsp_boot::encode_extended_assignment(dst,address as i32,false)[0];
    let body=0x4980_c000|((address&0x3f)<<5)|u32::from(dst.number());
    Ok([ext,body])
}

/// Original V66 duplex `R3=#1 ; R4=R3`. This 32-bit packed form is
/// independently produced unchanged by LLVM6's Hexagon backend.
pub fn encode_duplex_initial_state(dst:G,value:u8,copy:G)->Result<u32,String>{
    if dst.number()!=3||value!=1||copy.number()!=4{
        return Err("Counter seed duplex requires R3=1; R4=R3".into());
    }
    Ok(0x2813_3034)
}

/// Checked two-word `R3+=mpyi(R4,##0x15a4e35)`, with 26-bit extender and
/// original continuation parse marker on the consuming arithmetic word.
pub fn encode_extended_accumulate(dst:G,factor:G,multiplier:u32)->Result<[u32;2],String>{
    if dst.number()!=3||factor.number()!=4||multiplier!=0x015a_4e35{
        return Err("Only validated CDSP GP multiplier 0x015a4e35 is supported".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(dst,multiplier as i32,false)[0];
    let body=0xe100_0000|(u32::from(factor.number())<<16)|((multiplier&0x3f)<<5)|u32::from(dst.number());
    Ok([ext,M::ContinuationA.apply(body)])
}

/// `P0=cmp.eq(R4,0); if(P0.new) jump:t 0x56548`, packet-relative
/// signed displacement from packet start `0x5654c`.
pub fn encode_multiplier_backedge(src:G,compare:u8,start:u32,target:u32)->Result<u32,String>{
    if src.number()!=4||compare!=0||start!=0x96cc_854c||target!=0x96cc_8548{
        return Err("Unexpected CDSP GP multiplier counter/branch".into());
    }
    let delta=(target as i64)-(start as i64);
    if delta>=0||delta< -256||delta&1!=0{return Err("GP multiplier backward jump out of range".into());}
    Ok(0x1030_e000|(u32::from(src.number())<<16)|(((delta/2) as i32 as u32)&0xff))
}

/// Forced GP status store with explicit extender and original packet marker.
pub fn encode_extended_gp_state_store(src:G,address:u32)->Result<[u32;2],String>{
    if src.number()!=3||address!=0xc028_c02c{
        return Err("Expected memw(GP+0xc028c02c)=R3".into());
    }
    let ext=cdsp_boot::encode_extended_assignment(src,address as i32,false)[0];
    let body=M::ContinuationA.apply(0x4880_c000|(u32::from(src.number())<<8)|(address&0x3f));
    Ok([ext,body])
}

pub fn encode_result_store(base:G,value:G,offset:u8)->Result<u32,String>{
    if base.number()!=0||value.number()!=4||offset!=0{
        return Err("CDSP GP multiplier publishes R4 through R0+0".into());
    }
    Ok(0xa180_c000|(u32::from(value.number())<<8))
}

pub fn build_state_multiplier()->Result<Vec<u8>,String>{
    let r=|n|G::new(n);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    for w in encode_extended_gp_state_load(r(3)?,0xc028_c02c)?{emit(&mut out,w);}
    emit(&mut out,encode_duplex_initial_state(r(3)?,1,r(4)?)?);
    for w in encode_extended_accumulate(r(3)?,r(4)?,0x015a_4e35)?{emit(&mut out,w);}
    emit(&mut out,encode_multiplier_backedge(r(4)?,0,0x96cc_854c,0x96cc_8548)?);
    emit(&mut out,M::ContinuationA.apply(S::JumpRegister{source:r(31)?}.encode()?));
    for w in encode_extended_gp_state_store(r(3)?,0xc028_c02c)?{emit(&mut out,w);}
    emit(&mut out,encode_result_store(r(0)?,r(4)?,0)?);
    if out.len()!=EXECUTABLE_LEN{return Err("CDSP GP multiplier must occupy 40 bytes".into());}
    Ok(out)
}

pub fn build_rwe_prefix_through_multiplier()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_callback_service::build_rwe_prefix_through_callback()?;
    out.extend(build_state_multiplier()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN{return Err("CDSP GP multiplier prefix wrong length".into());}
    Ok(out)
}

pub fn verify_multiplier(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let bytes=build_state_multiplier()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(bytes.as_slice()){
        return Err("CDSP GP multiplier differs from pinned reference".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let bytes=build_rwe_prefix_through_multiplier()?;
    if reference.get(RWE_LAYOUT_START..END_FILE_OFFSET)!=Some(bytes.as_slice()){
        return Err("CDSP RWE prefix through GP multiplier differs from original".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    fn r(n:u8)->G{G::new(n).unwrap()}
    #[test]fn state_multiplier_immediates_and_backedges_checked(){
        assert_eq!(encode_extended_gp_state_load(r(3),0xc028c02c).unwrap(),[0x0c02_6300,0x4980_c583]);
        assert_eq!(encode_duplex_initial_state(r(3),1,r(4)).unwrap(),0x2813_3034);
        assert_eq!(encode_extended_accumulate(r(3),r(4),0x015a4e35).unwrap(),[0x0015_6938,0xe104_46a3]);
        assert_eq!(encode_multiplier_backedge(r(4),0,0x96cc854c,0x96cc8548).unwrap(),0x1034_e0fe);
        assert_eq!(encode_extended_gp_state_store(r(3),0xc028c02c).unwrap(),[0x0c02_6300,0x4880_432c]);
        assert_eq!(encode_result_store(r(0),r(4),0).unwrap(),0xa180_c400);
        assert!(encode_multiplier_backedge(r(4),0,0x96cc854c,0x96cc8540).is_err());
        assert!(encode_extended_accumulate(r(3),r(4),0x015a4e36).is_err());
    }
    #[test]fn gp_multiplier_has_exact_file_boundaries(){
        assert_eq!(build_state_multiplier().unwrap().len(),40);
        assert_eq!(build_rwe_prefix_through_multiplier().unwrap().len(),0x568);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1352);
        assert_eq!(RWE_LAYOUT_NOP_LEN,32);
    }
}
