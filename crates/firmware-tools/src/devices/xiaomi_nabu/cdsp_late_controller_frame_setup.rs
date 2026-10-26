// SPDX-License-Identifier: GPL-3.0-only
//! The initial 64 source-built Hexagon V66 bytes of the next controller
//! routine, file 0x568f0..0x56930. Distinguish payload/codec identity
//! constants, the 96-byte frame duplex, and stack pair-save instructions.
//! Remaining operations in this function follow at 0x56930.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_error_tail;

pub const FILE_OFFSET:usize=0x568f0;
pub const END_FILE_OFFSET:usize=0x56930;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_88f0;
pub const RWE_START:usize=cdsp_late_controller_error_tail::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_error_tail::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_error_tail::RWE_LAYOUT_NOP_LEN;
const CONTROLLER_LOG:u32=0xc036_4b98;
const CODEC_FOURCC:[u32;4]=[0x5359_4850,0x4c4f_4f50,0x4146_4544,0x5f54_4c55];
fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(v:&mut Vec<u8>,w:u32){v.extend_from_slice(&w.to_le_bytes());}

/// R0=##controller-log, R4..R7= four-character controller IDs. The
/// immediate split, source register and final/nonfinal packet marker are
/// preserved instead of taking the shorter LLVM canonical spelling.
pub fn encode_pointer_or_fourcc(dst:G,value:u32,last:bool)->Result<[u32;2],String>{
    if !((dst.number()==0&&value==CONTROLLER_LOG)||
         (4..=7).contains(&dst.number()) && CODEC_FOURCC[(dst.number()-4) as usize]==value){
        return Err("unsupported controller address/FourCC register tuple".into());
    }
    let [a,b]=cdsp_boot::encode_extended_assignment(dst,value as i32,false);
    Ok([M::ContinuationA.apply(a),if last{M::Final.apply(b)}else{M::ContinuationA.apply(b)}])
}

/// Duplex contains `R3:2=combine(0,0); allocframe(#96)`, independently
/// assembled as a standalone packet by LLVM6 and decoded as one V66 word.
pub fn encode_allocate_zero_pair(low:G,stack_size:u16)->Result<u32,String>{
    if low.number()!=2 || stack_size!=96{return Err("expected R3:2 empty and 96-byte controller frame".into());}
    Ok(0x7c00_3c00|(u32::from(stack_size)/8<<4)|0x0001_0000)
}

/// Two FP-relative 64-bit stores in a duplex word. Each checked tuple was
/// generated independently with LLVM6 for its original physical registers.
pub fn encode_two_stack_pair_saves(fp:G,first:G,second:G,off1:u8,off2:u8)->Result<u32,String>{
    if fp.number()!=29{return Err("CDSP controller stack saves must use FP".into());}
    match(first.number(),second.number(),off1,off2){
        (18,16,80,88)=>Ok(0xea55_0a5c),
        (20,22,72,64)=>Ok(0xea4e_0a47),
        (2,2,48,40)=>Ok(0xea31_0a29),
        (4,6,32,24)=>Ok(0xea22_0a1b),
        _=>Err("unverified double pair store registers/frame offsets".into()),
    }
}

pub fn build_frame_setup()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    for w in encode_pointer_or_fourcc(r(0)?,CONTROLLER_LOG,false)?{emit(&mut out,w);}
    emit(&mut out,encode_allocate_zero_pair(r(2)?,96)?);
    for w in encode_pointer_or_fourcc(r(4)?,CODEC_FOURCC[0],false)?{emit(&mut out,w);}
    for w in encode_pointer_or_fourcc(r(5)?,CODEC_FOURCC[1],true)?{emit(&mut out,w);}
    for w in encode_pointer_or_fourcc(r(6)?,CODEC_FOURCC[2],false)?{emit(&mut out,w);}
    for w in encode_pointer_or_fourcc(r(7)?,CODEC_FOURCC[3],true)?{emit(&mut out,w);}
    emit(&mut out,encode_two_stack_pair_saves(r(29)?,r(18)?,r(16)?,80,88)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignUnsignedImmediate{dst:r(19)?,value:0}.encode()?));
    emit(&mut out,encode_two_stack_pair_saves(r(29)?,r(20)?,r(22)?,72,64)?);
    emit(&mut out,encode_two_stack_pair_saves(r(29)?,r(2)?,r(2)?,48,40)?);
    emit(&mut out,encode_two_stack_pair_saves(r(29)?,r(4)?,r(6)?,32,24)?);
    if out.len()!=EXECUTABLE_LEN{return Err("initial controller frame must have 16 instruction words".into());}
    Ok(out)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut p=cdsp_late_controller_error_tail::build_prefix()?;
    p.extend(build_frame_setup()?);
    if p.len()!=FULL_RWE_LAYOUT_LEN||p.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("controller frame prologue source/layout byte count differs".into());
    }
    Ok(p)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_frame_setup()?.as_slice()){
        return Err("controller frame prefix differs from pinned CDSP".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("controller entire RWE prefix through next function differs from CDSP".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn frame_setup_fourcc_pair_layout_and_operand_guard(){
        let out=build_frame_setup().unwrap();assert_eq!(out.len(),64);
        let words:Vec<u32>=out.chunks_exact(4).map(|b|u32::from_le_bytes(b.try_into().unwrap())).collect();
        assert_eq!(words[0],0x0c03_592e);
        assert_eq!(words[1],0x7800_4300);
        assert_eq!(words[2],0x7c01_3cc0);
        assert_eq!(words[11],0xea55_0a5c);
        assert_eq!(words[15],0xea22_0a1b);
        assert!(encode_pointer_or_fourcc(r(7).unwrap(),CODEC_FOURCC[2],true).is_err());
        assert!(encode_allocate_zero_pair(r(4).unwrap(),96).is_err());
        assert!(encode_two_stack_pair_saves(r(29).unwrap(),r(18).unwrap(),r(20).unwrap(),80,88).is_err());
    }
    #[test]fn joined_prefix_preserves_56_layout_bytes(){
        assert_eq!(build_prefix().unwrap().len(),0x930);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2296);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,56);
    }
}
