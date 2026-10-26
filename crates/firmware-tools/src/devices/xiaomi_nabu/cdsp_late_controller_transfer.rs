// SPDX-License-Identifier: GPL-3.0-only
//! CDSP Hexagon V66 large-controller transfer entry, file 0x56698..0x566c0.
//!
//! Calls a controller helper with two GP-buffer pointer payloads, checks
//! the returned handle through an original .new predicate loop, and branches
//! to the shared error path when no valid buffer is found. The caller's full
//! function continues after 0x566c0: this is a source-emitted partial range,
//! not a claim of complete CDSP executable or re-signed firmware.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_validate;

pub const FILE_OFFSET:usize=0x56698;
pub const END_FILE_OFFSET:usize=0x566c0;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8698;
pub const RWE_START:usize=0x56000;
pub const RWE_PREFIX_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const RWE_PREFIX_CODE_LEN:usize=cdsp_late_controller_validate::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_validate::RWE_LAYOUT_NOP_LEN;

fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(o:&mut Vec<u8>,w:u32){o.extend_from_slice(&w.to_le_bytes());}

/// Duplex: R1=R16 or R17; R2=memw(R19+#0). LLVM6 independently assembles
/// both forms without changing the OEM-selected duplex encoding.
pub fn encode_pointer_dispatch_pair(copy_from:G,load_base:G,dest:G,offset:u8)->Result<u32,String>{
    if ![16,17].contains(&copy_from.number()) || load_base.number()!=19 || dest.number()!=2 || offset!=0 {
        return Err("controller transfer only supports R16/R17 into R1 and memw(R19+0) into R2".into());
    }
    Ok(0x5081_00b2 | (u32::from(copy_from.number()-16)<<20))
}

/// A32-independent Hexagon direct call, 22-bit packet-relative displacement
/// split across a high six-bit field and a low 14-bit halfword field. Only
/// the two physically verified CDSP controller call sites are supported.
pub fn encode_two_buffer_call(from:u32,target:u32)->Result<u32,String>{
    if ![(0x96cc_869c,0x96cd_1020),(0x96cc_86a8,0x96cd_0480)].contains(&(from,target)){
        return Err("controller transfer call target or source has not been decoded".into());
    }
    let delta=target.checked_sub(from).ok_or("controller helper call is backwards")?;
    if delta&1!=0 || delta>0x1fff_fe{return Err("controller call displacement out of range".into());}
    Ok(0x5a00_4000 | (((delta>>15)&0x3f)<<16) | ((delta>>1)&0x3fff))
}

/// `R0=memw(R2+#16)`. The 32-bit word displacement uses 4-byte units.
pub fn encode_controller_handle_word(dst:G,base:G,offset:u8)->Result<u32,String>{
    if dst.number()!=0 || base.number()!=2 || offset!=16 {
        return Err("controller expected 16-byte offset from R2 to return handle".into());
    }
    Ok(0x9180_c000 | (u32::from(base.number())<<16) | (u32::from(offset)<<3) | u32::from(dst.number()))
}

/// Original .new comparison of R16 against zero, followed by a signed short
/// branch. The high two displacement bits occur at [21:20], with the low
/// eight signed halfword units at [7:0]. The packet starts one word earlier.
pub fn encode_controller_retry_if_nonzero(src:G,packet:u32,target:u32)->Result<u32,String>{
    if src.number()!=16 || packet!=0x96cc_86b4 || target!=0x96cc_8670{
        return Err("unexpected transfer retry predicate or physical backedge".into());
    }
    let dist=i64::from(target)-i64::from(packet);
    if dist>=0 || dist < -0x200 || dist%2!=0 {
        return Err("new-value backward branch displacement outside checked signed range".into());
    }
    let half=(dist/2) as i32 as u32;
    Ok(0x2442_e000 | (half&0xff) | (((half>>8)&3)<<20))
}

/// Exact 10 instruction words for file 0x56698..0x566c0, kept as checked
/// operand encodings instead of a vendor opcode slice.
pub fn build_transfer_prefix()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_pointer_dispatch_pair(r(16)?,r(19)?,r(2)?,0)?);
    emit(&mut out,encode_two_buffer_call(PHYSICAL_ADDRESS+4,0x96cd_1020)?);
    emit(&mut out,encode_controller_handle_word(r(0)?,r(2)?,16)?);
    emit(&mut out,encode_pointer_dispatch_pair(r(17)?,r(19)?,r(2)?,0)?);
    emit(&mut out,encode_two_buffer_call(PHYSICAL_ADDRESS+0x10,0x96cd_0480)?);
    emit(&mut out,cdsp_late_controller_validate::encode_zero_negative_pair(r(2)?,0,-1)?);
    emit(&mut out,encode_controller_handle_word(r(0)?,r(2)?,16)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignRegister{dst:r(16)?,src:r(0)?}.encode()?));
    emit(&mut out,encode_controller_retry_if_nonzero(r(16)?,PHYSICAL_ADDRESS+0x1c,0x96cc_8670)?);
    emit(&mut out,S::JumpRelative{displacement_bytes:0x200}.encode()?);
    if out.len()!=EXECUTABLE_LEN {return Err("CDSP controller transfer must emit 40 bytes".into());}
    Ok(out)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut b=cdsp_late_controller_validate::build_prefix()?;
    b.extend(build_transfer_prefix()?);
    if b.len()!=RWE_PREFIX_LEN || RWE_PREFIX_CODE_LEN+RWE_PREFIX_LAYOUT_LEN!=b.len(){
        return Err("controller transfer source/layout length mismatch".into());
    }
    Ok(b)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_transfer_prefix()?.as_slice()){
        return Err("controller transfer source differs from pinned CDSP reference".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("joined CDSP controller RWE prefix differs from pinned original".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn paired_controller_transfer_checks_operand_fields(){
        assert_eq!(encode_pointer_dispatch_pair(r(16).unwrap(),r(19).unwrap(),r(2).unwrap(),0).unwrap(),0x5081_00b2);
        assert_eq!(encode_pointer_dispatch_pair(r(17).unwrap(),r(19).unwrap(),r(2).unwrap(),0).unwrap(),0x5091_00b2);
        assert_eq!(encode_two_buffer_call(0x96cc_869c,0x96cd_1020).unwrap(),0x5a01_44c2);
        assert_eq!(encode_two_buffer_call(0x96cc_86a8,0x96cd_0480).unwrap(),0x5a00_7eec);
        assert_eq!(encode_controller_handle_word(r(0).unwrap(),r(2).unwrap(),16).unwrap(),0x9182_c080);
        assert_eq!(encode_controller_retry_if_nonzero(r(16).unwrap(),0x96cc_86b4,0x96cc_8670).unwrap(),0x2472_e0de);
        assert!(encode_pointer_dispatch_pair(r(18).unwrap(),r(19).unwrap(),r(2).unwrap(),0).is_err());
        assert!(encode_two_buffer_call(0x96cc_869c,0x96cd_1024).is_err());
        assert!(encode_controller_handle_word(r(0).unwrap(),r(2).unwrap(),20).is_err());
        assert!(encode_controller_retry_if_nonzero(r(16).unwrap(),0x96cc_86b4,0x96cc_8674).is_err());
    }
    #[test]fn joined_controller_transfer_is_contiguous_and_fill_explicit(){
        assert_eq!(build_transfer_prefix().unwrap().len(),40);
        assert_eq!(build_prefix().unwrap().len(),0x6c0);
        assert_eq!(RWE_PREFIX_CODE_LEN,1684);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,44);
    }
}
