// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon CDSP bounded memory-copy helper with fallback delegation.
//!
//! File [0x565c0,0x565fc) is 60 independently rebuilt executable bytes.
//! File [0x565fc,0x56600) is one structural NOP/fill, not original source
//! execution. The routine copies unsigned bytes in a hardware loop when the
//! destination is strictly within its bounded source, or calls the shared
//! fallback service when the preconditions are not met. Packet positions and
//! V66 predicate `.new` ordering are explicit.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_bounded_compare;
use super::cdsp_late_null_callback::encode_null_trampoline_frame;

pub const FILE_OFFSET:usize=0x565c0;
pub const END_FILE_OFFSET:usize=0x565fc;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const ALIGNMENT_LEN:usize=4;
pub const NEXT_ROUTINE:usize=END_FILE_OFFSET+ALIGNMENT_LEN;
pub const PHYSICAL_ADDRESS:u32=0x96cc85c0;
pub const RWE_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=NEXT_ROUTINE-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_bounded_compare::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_bounded_compare::RWE_LAYOUT_NOP_LEN+ALIGNMENT_LEN;

fn emit(v:&mut Vec<u8>,w:u32){v.extend_from_slice(&w.to_le_bytes());}
fn r(n:u8)->Result<G,String>{G::new(n)}

/// The previous same-packet assignment produces R16.new; branch to the
/// original return if the copy count is zero. Packet-relative displacements
/// are measured at the packet's first instruction, not the branch word.
pub fn encode_new_zero_skip(source:G,packet_start:u32,target:u32)->Result<u32,String>{
    if source.number()!=16||packet_start!=PHYSICAL_ADDRESS+4{
        return Err("copy helper requires R16.new after R16=R2".into());
    }
    let delta=target.checked_sub(packet_start).ok_or("backwards copy skip")?;
    if delta==0||delta>0x1fe||delta&1!=0{
        return Err("invalid forward copy-skip displacement".into());
    }
    Ok(0x2402_c000|(delta>>1))
}

/// Compound `P0=cmp.gtu(R1,R0); if(P0.new) jump:nt`, following the R2 SUB
/// in this packet. LLVM6 picks another semantic order and parse marker.
pub fn encode_source_after_dest_guard(lhs:G,rhs:G,packet_start:u32,target:u32)->Result<u32,String>{
    if lhs.number()!=1||rhs.number()!=0||packet_start!=PHYSICAL_ADDRESS+0xc{
        return Err("copy requires unsigned R1>R0 bounds guard".into());
    }
    let delta=target.checked_sub(packet_start).ok_or("guard backwards target")?;
    if delta==0||delta>0x1fe||delta&1!=0{
        return Err("unsupported bounds-guard displacement".into());
    }
    Ok(0x1500_c000|(u32::from(lhs.number())<<16)|(delta>>1))
}

/// Negated signed `P0=!cmp.gt(R16,R2)` from independent LLVM6 fixture.
/// Changing the packet marker from LLVM's final to the original continuation
/// does not alter operands or predicate polarity.
pub fn encode_not_signed_gt(dst:G,src:G)->Result<u32,String>{
    if dst.number()!=16||src.number()!=2{
        return Err("copy hardware-loop branch compares R16 and R2".into());
    }
    Ok(0xf250_c010|(u32::from(src.number())<<8))
}

/// Byte-unsigned postincrement from an R1 source, with the original loop-end
/// marker selected when emitted inside the two-word hardware-loop packet.
pub fn encode_unsigned_byte_postincrement(dst:G,base:G,increment:u8)->Result<u32,String>{
    if dst.number()!=2||base.number()!=1||increment!=1{
        return Err("byte-copy hardware loop expects memub(R1++#1) -> R2".into());
    }
    Ok(0x9b20_c000 |(u32::from(base.number())<<16)|
       (u32::from(increment)<<5)|u32::from(dst.number()))
}

/// `memb(R0++#1)=R2.new`, independently LLVM6 emitted this original word.
pub fn encode_new_byte_postincrement_store(base:G,source:G,increment:u8)->Result<u32,String>{
    if base.number()!=0||source.number()!=2||increment!=1{
        return Err("byte-copy store requires memb(R0++#1)=R2.new".into());
    }
    Ok(0xaba0_c000 |(u32::from(source.number())<<8)|u32::from(increment)<<3)
}

/// One-word Hexagon direct call to a checked shared copy fallback target.
/// The split high/low branch fields are independently confirmed by LLVM6.
pub fn encode_delegate_call(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start!=PHYSICAL_ADDRESS+0x2c||target!=0x96dd_b740{
        return Err("unexpected CDSP delegate call site or destination".into());
    }
    let delta=target.checked_sub(packet_start).ok_or("delegate target below source")?;
    if delta&1!=0||delta>0x1fff_fe{return Err("delegate call displacement outside single-word range".into());}
    let word=0x5a00_c000 | (((delta>>15)&0x3f)<<16) | ((delta>>1)&0x3fff);
    Ok(M::ContinuationA.apply(word))
}

/// Adjacent pair in original final packet: R0=R16 and frame return.
pub fn encode_return_copied_length(result:G,copied:G)->Result<u32,String>{
    if result.number()!=0||copied.number()!=16{
        return Err("copy return must restore R0 from saved R16".into());
    }
    Ok(M::ContinuationA.apply(S::AssignRegister{dst:result,src:copied}.encode()?))
}

pub fn build_copy_or_delegate()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_null_trampoline_frame(r(29)?,r(16)?,8)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignRegister{dst:r(16)?,src:r(2)?}.encode()?));
    emit(&mut out,encode_new_zero_skip(r(16)?,PHYSICAL_ADDRESS+4,PHYSICAL_ADDRESS+0x34)?);
    emit(&mut out,M::ContinuationA.apply(S::SubtractRegisters{dst:r(2)?,left:r(0)?,right:r(1)?}.encode()?));
    emit(&mut out,encode_source_after_dest_guard(r(1)?,r(0)?,PHYSICAL_ADDRESS+0xc,PHYSICAL_ADDRESS+0x2c)?);
    emit(&mut out,M::ContinuationA.apply(S::HardwareLoop{loop_index:0,counter:r(16)?,displacement_bytes:12}.encode()?));
    emit(&mut out,M::ContinuationA.apply(encode_not_signed_gt(r(16)?,r(2)?)?));
    emit(&mut out,S::JumpIfNewP0NotTaken{displacement_bytes:24}.encode()?);
    emit(&mut out,M::LoopEnd.apply(encode_unsigned_byte_postincrement(r(2)?,r(1)?,1)?));
    emit(&mut out,encode_new_byte_postincrement_store(r(0)?,r(2)?,1)?);
    emit(&mut out,S::JumpRelative{displacement_bytes:12}.encode()?);
    emit(&mut out,encode_delegate_call(PHYSICAL_ADDRESS+0x2c,0x96dd_b740)?);
    emit(&mut out,S::AssignRegister{dst:r(2)?,src:r(16)?}.encode()?);
    emit(&mut out,encode_return_copied_length(r(0)?,r(16)?)?);
    emit(&mut out,super::cdsp_late_null_callback::encode_null_frame_return(r(29)?,r(16)?,0)?);
    if out.len()!=EXECUTABLE_LEN{return Err("copy/delegate must encode 60 executable bytes".into());}
    Ok(out)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut bytes=cdsp_late_bounded_compare::build_prefix()?;
    bytes.extend(build_copy_or_delegate()?);
    emit(&mut bytes,S::Nop.encode()?); // exactly one original linker NOP
    if bytes.len()!=FULL_RWE_LAYOUT_LEN{return Err("copy/delegate RWE prefix length mismatch".into());}
    Ok(bytes)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let bytes=build_copy_or_delegate()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(bytes.as_slice()){
        return Err("typed copy/delegate bytes differ from CDSP reference".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let bytes=build_prefix()?;
    if reference.get(RWE_START..NEXT_ROUTINE)!=Some(bytes.as_slice()){
        return Err("copy/delegate prefix differs from pinned CDSP reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test] fn checked_copy_loop_operands(){
        let bytes=build_copy_or_delegate().unwrap();assert_eq!(bytes.len(),60);
        assert_eq!(encode_new_zero_skip(r(16).unwrap(),PHYSICAL_ADDRESS+4,PHYSICAL_ADDRESS+0x34).unwrap(),0x2402_c018);
        assert_eq!(encode_source_after_dest_guard(r(1).unwrap(),r(0).unwrap(),PHYSICAL_ADDRESS+0xc,PHYSICAL_ADDRESS+0x2c).unwrap(),0x1501_c010);
        assert_eq!(M::LoopEnd.apply(encode_unsigned_byte_postincrement(r(2).unwrap(),r(1).unwrap(),1).unwrap()),0x9b21_8022);
        assert_eq!(encode_new_byte_postincrement_store(r(0).unwrap(),r(2).unwrap(),1).unwrap(),0xaba0_c208);
        assert_eq!(encode_delegate_call(PHYSICAL_ADDRESS+0x2c,0x96dd_b740).unwrap(),0x5a22_58aa);
        assert!(encode_new_zero_skip(r(15).unwrap(),PHYSICAL_ADDRESS+4,PHYSICAL_ADDRESS+0x34).is_err());
        assert!(encode_unsigned_byte_postincrement(r(2).unwrap(),r(1).unwrap(),2).is_err());
        assert!(encode_delegate_call(PHYSICAL_ADDRESS+0x2c,0x96dd_b742).is_err());
    }
    #[test] fn joined_prefix_counts_code_and_fill(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,1536);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1492);
        assert_eq!(RWE_LAYOUT_NOP_LEN,44);
        assert_eq!(build_prefix().unwrap().len(),1536);
    }
}
