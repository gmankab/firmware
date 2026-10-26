// SPDX-License-Identifier: GPL-3.0-only
//! Bounded signed-byte comparison, file 0x56590..0x565c0, Hexagon V66.
//!
//! This routine compares a zero-terminated signed byte input with an indexed
//! second buffer for at most R2 elements. `R0` counts compared elements; a
//! bounded early return reports zero on exhausted input. Packets preserve the
//! predicate producer and the *original* new-predicate branch order, which
//! differs from LLVM6's independently compiled scheduling of the same code.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_null_callback;

pub const FILE_OFFSET:usize=0x56590;
pub const END_FILE_OFFSET:usize=0x565c0;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc8590;
pub const RWE_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_null_callback::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_null_callback::RWE_LAYOUT_NOP_LEN;

fn emit(v:&mut Vec<u8>,word:u32){v.extend_from_slice(&word.to_le_bytes());}
fn r(n:u8)->Result<G,String>{G::new(n)}

/// Duplex `R0=#0; R3=R0`. The exact packed form is assembled independently
/// by LLVM6; other register combinations have not been independently proven.
pub fn encode_zero_counter_and_pointer(count:G,pointer:G,value:u8)->Result<u32,String>{
    if count.number()!=0||pointer.number()!=3||value!=0 {
        return Err("bounded compare expects duplex R0=0 and R3=R0".into());
    }
    Ok(0x2800_3003)
}

/// Predicate + branch is an original V66 two-word packet. LLVM6 places the
/// branch first, but its independently compiled predicate opcode is the same
/// after changing the parse marker from `Final` to `ContinuationA`.
pub fn encode_p0_not_gtu_and_jump(lhs:G,rhs:G,packet_start:u32,target:u32)->Result<[u32;2],String>{
    if lhs.number()!=2||rhs.number()!=0||packet_start!=PHYSICAL_ADDRESS+4 {
        return Err("unexpected P0 unsigned bounds comparison".into());
    }
    let displacement=target.checked_sub(packet_start).ok_or("branch target before bounds check")?;
    if displacement==0||displacement>0x7fe||displacement&1!=0 {
        return Err("bounds branch displacement out of checked range".into());
    }
    // Base predicate comes from the independent LLVM6 semantic assembler;
    // register fields are checked by the two explicit operands.
    let pred=M::ContinuationA.apply(0xf260_c010|(u32::from(lhs.number())<<16));
    let branch=0x5c00_c800|(displacement>>1);
    Ok([pred,branch])
}

/// `Rd=memb(Rbase+#0)`; signed byte load.  Uses ordinary GPR fields.
pub fn encode_signed_byte_load(dst:G,base:G,offset:i8)->Result<u32,String>{
    if offset!=0{return Err("bounded compare requires byte load with no displacement".into());}
    Ok(0x9100_c000|(u32::from(base.number())<<16)|u32::from(dst.number()))
}

/// Branch if the previous packet's newly loaded R4 is zero.  Displacement
/// counts from the packet's first instruction, not the branch word.
pub fn encode_new_byte_zero_branch(src:G,packet_start:u32,target:u32)->Result<u32,String>{
    if src.number()!=4||packet_start!=PHYSICAL_ADDRESS+0xc {
        return Err("bounded compare expects R4.new byte test".into());
    }
    let displacement=target.checked_sub(packet_start).ok_or("negative new-byte branch")?;
    if displacement==0||displacement>0x7fe||displacement&1!=0 {
        return Err("byte guard target outside checked forward interval".into());
    }
    Ok(0x2402_c000|(displacement>>1))
}

/// `Rd=memb(Rbase++#1)` preserves signedness and pointer postincrement.
pub fn encode_signed_byte_postincrement(dst:G,base:G,increment:u8)->Result<u32,String>{
    if increment!=1{return Err("signed postincrement supports +1 only".into());}
    Ok(0x9b00_c000|(u32::from(base.number())<<16)|
       (u32::from(increment)<<5)|u32::from(dst.number()))
}

/// `if(cmp.eq(R5.new,R4)) jump:t <backedge>`, original four-word packet tail.
pub fn encode_matching_byte_backedge(new_byte:G,compare:G,start:u32,target:u32)->Result<u32,String>{
    if new_byte.number()!=5||compare.number()!=4||start!=PHYSICAL_ADDRESS+0x14 {
        return Err("byte backedge must compare R5.new with R4".into());
    }
    let delta=i64::from(target)-i64::from(start);
    if delta>=0||delta < -0x100||delta&1!=0 {
        return Err("bounded compare backward branch exceeds confirmed encoding".into());
    }
    Ok(0x2032_e000|(u32::from(compare.number())<<8)|((delta/2) as i32 as u32&0xff))
}

/// V66 duplex `R0=#0; jumpr R31` produced identically by LLVM6.
pub fn encode_zero_and_return(result:G,link:G,value:u8)->Result<u32,String>{
    if result.number()!=0||link.number()!=31||value!=0 {
        return Err("bounded zero return requires R0=0 and R31 link".into());
    }
    Ok(0x4800_3fc0)
}

pub fn build_bounded_compare()->Result<Vec<u8>,String>{
    let mut b=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut b,encode_zero_counter_and_pointer(r(0)?,r(3)?,0)?);
    for word in encode_p0_not_gtu_and_jump(r(2)?,r(0)?,PHYSICAL_ADDRESS+4,PHYSICAL_ADDRESS+0x28)? {
        emit(&mut b,word);
    }
    emit(&mut b,M::ContinuationA.apply(encode_signed_byte_load(r(4)?,r(1)?,0)?));
    emit(&mut b,encode_new_byte_zero_branch(r(4)?,PHYSICAL_ADDRESS+0xc,PHYSICAL_ADDRESS+0x2c)?);
    emit(&mut b,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:1}.encode()?));
    emit(&mut b,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(1)?,src:r(1)?,value:1}.encode()?));
    emit(&mut b,M::ContinuationA.apply(encode_signed_byte_postincrement(r(5)?,r(3)?,1)?));
    emit(&mut b,encode_matching_byte_backedge(r(5)?,r(4)?,PHYSICAL_ADDRESS+0x14,PHYSICAL_ADDRESS+4)?);
    emit(&mut b,S::JumpRegister{source:r(31)?}.encode()?);
    for _ in 0..2{emit(&mut b,encode_zero_and_return(r(0)?,r(31)?,0)?);}
    if b.len()!=EXECUTABLE_LEN {return Err("bounded compare code layout mismatch".into());}
    Ok(b)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut b=cdsp_late_null_callback::build_rwe_prefix_through_null_callback()?;
    b.extend(build_bounded_compare()?);
    if b.len()!=FULL_RWE_LAYOUT_LEN{return Err("bounded compare RWE prefix length mismatch".into());}
    Ok(b)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let b=build_bounded_compare()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(b.as_slice()){
        return Err("bounded compare 48 bytes differ from pinned reference".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let b=build_prefix()?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(b.as_slice()){
        return Err("bounded compare prefix differs from original".into());
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test]fn checked_bounded_semantic_operands(){
        let code=build_bounded_compare().unwrap();assert_eq!(code.len(),48);
        assert_eq!(encode_zero_counter_and_pointer(r(0).unwrap(),r(3).unwrap(),0).unwrap(),0x28003003);
        assert_eq!(M::ContinuationA.apply(encode_signed_byte_load(r(4).unwrap(),r(1).unwrap(),0).unwrap()),0x91014004);
        assert_eq!(M::ContinuationA.apply(encode_signed_byte_postincrement(r(5).unwrap(),r(3).unwrap(),1).unwrap()),0x9b034025);
        assert_eq!(encode_zero_and_return(r(0).unwrap(),r(31).unwrap(),0).unwrap(),0x48003fc0);
        assert!(encode_p0_not_gtu_and_jump(r(2).unwrap(),r(0).unwrap(),PHYSICAL_ADDRESS+4,PHYSICAL_ADDRESS).is_err());
        assert!(encode_new_byte_zero_branch(r(3).unwrap(),PHYSICAL_ADDRESS+0xc,PHYSICAL_ADDRESS+0x2c).is_err());
        assert!(encode_matching_byte_backedge(r(5).unwrap(),r(4).unwrap(),PHYSICAL_ADDRESS+0x14,PHYSICAL_ADDRESS+0x114).is_err());
    }
    #[test]fn prefix_accounts_code_and_fill_separately(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,0x5c0);
        assert_eq!(RWE_LAYOUT_NOP_LEN,40);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1432);
        assert_eq!(build_prefix().unwrap().len(),1472);
    }
}
