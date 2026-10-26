// SPDX-License-Identifier: GPL-3.0-only
//! Start of the large-controller bit-plane iterator, CDSP file 0x566c0.
//! Source-encoded Hexagon V66 packets compute the current word-plane size,
//! terminate if the active stream is empty and guard a later MMIO loop.
//! The complete loop and function epilogue remain to be reconstructed.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_transfer;

pub const FILE_OFFSET:usize=0x566c0;
pub const END_FILE_OFFSET:usize=0x566d8;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_86c0;
pub const RWE_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_transfer::RWE_PREFIX_CODE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_controller_transfer::RWE_PREFIX_LAYOUT_LEN;

fn r(i:u8)->Result<G,String>{G::new(i)}
fn emit(v:&mut Vec<u8>,w:u32){v.extend_from_slice(&w.to_le_bytes());}

/// Original low-register duplex pair `R25:24=combine(#1,#7)`. The checked
/// operands are assembled from two signed halfword values, not copied data.
pub fn encode_plane_size_pair(low:G,high:i8,low_value:i8)->Result<u32,String>{
    if low.number()!=24||high!=1||low_value!=7{
        return Err("controller uses R25:24=combine(1,7)".into());
    }
    Ok(0x7c03_6000 | (u32::from(low_value as u8)<<3) | u32::from(low.number()))
}

/// `P0=cmp.eq(R17,#0); if(P0.new) jump:nt skip` in a single fused V66
/// instruction, with immediate branch displacement including the 8-byte
/// advance implicit in this original decode family. Physical targets were
/// verified with Rizin against the complete original stream.
pub fn encode_skip_if_empty(src:G,packet:u32,target:u32)->Result<u32,String>{
    if src.number()!=17 || packet!=PHYSICAL_ADDRESS+8 || target!=0x96cc_8734{
        return Err("unsupported bitloop empty-stream predicate/target".into());
    }
    let d=target.checked_sub(packet).ok_or("empty stream branch must be forward")?;
    if d==0||d>0x1f8||d%2!=0{return Err("empty stream branch displacement too large".into());}
    Ok(0x1009_c000 | ((d+8)>>1))
}

/// `R3=cl0(R17)`; only the witnessed general-register form is accepted.
pub fn encode_count_leading_zeros(dst:G,src:G)->Result<u32,String>{
    if dst.number()!=3||src.number()!=17{return Err("bitloop CL0 requires R3,R17".into());}
    Ok(0x8c00_40a0 | (u32::from(src.number())<<16) | u32::from(dst.number()))
}

/// Reject too-small output bitplane: `if(!cmp.gtu(R24,R2.new)) jump:nt`.
/// The branch consumes R2.new immediately after the negative add.
pub fn encode_if_insufficient_bits(reg:G,packet:u32,target:u32)->Result<u32,String>{
    if reg.number()!=24||packet!=PHYSICAL_ADDRESS+0x14||target!=0x96cc_8890{
        return Err("controller bit capacity branch has unsupported source/target".into());
    }
    let d=target.checked_sub(packet).ok_or("bit capacity branch must be forward")?;
    if d==0||d>0x3f8||d%2!=0{return Err("bit capacity branch out of range".into());}
    Ok(0x2242_d800 | ((d+8)>>1))
}

pub fn build_bitloop_prefix()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_plane_size_pair(r(24)?,1,7)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignUnsignedImmediate{dst:r(18)?,value:16}.encode()?));
    emit(&mut out,encode_skip_if_empty(r(17)?,PHYSICAL_ADDRESS+8,0x96cc_8734)?);
    emit(&mut out,encode_count_leading_zeros(r(3)?,r(17)?)?);
    emit(&mut out,M::ContinuationA.apply(S::AddSignedSmallImmediate{dst:r(2)?,src:r(18)?,value:-16}.encode()?));
    emit(&mut out,encode_if_insufficient_bits(r(24)?,PHYSICAL_ADDRESS+0x14,0x96cc_8890)?);
    if out.len()!=EXECUTABLE_LEN{return Err("bitloop prefix must contain six V66 instructions".into());}
    Ok(out)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_transfer::build_prefix()?;
    out.extend(build_bitloop_prefix()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN || out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_LAYOUT_NOP_LEN{
        return Err("bitloop source code/fill bookkeeping mismatch".into());
    }
    Ok(out)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bitloop_prefix()?.as_slice()){
        return Err("bitloop prefix bytes differ from pinned CDSP original".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("joined RWE bitloop layout differs from pinned original".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn bitloop_fields_and_branches_are_checked(){
        let bytes=build_bitloop_prefix().unwrap();
        assert_eq!(bytes.len(),24);
        let words:Vec<u32>=bytes.chunks_exact(4).map(|w|u32::from_le_bytes(w.try_into().unwrap())).collect();
        assert_eq!(words,[0x7c03_6038,0x7800_4212,0x1009_c03a,0x8c11_40a3,0xbff2_7e02,0x2242_d8e2]);
        assert!(encode_plane_size_pair(r(24).unwrap(),2,7).is_err());
        assert!(encode_skip_if_empty(r(17).unwrap(),PHYSICAL_ADDRESS+8,0x96cc_8738).is_err());
        assert!(encode_count_leading_zeros(r(2).unwrap(),r(17).unwrap()).is_err());
        assert!(encode_if_insufficient_bits(r(24).unwrap(),PHYSICAL_ADDRESS+0x14,0x96cc_8894).is_err());
    }
    #[test]fn prefix_counts_layout_fill_separately(){
        assert_eq!(build_prefix().unwrap().len(),0x6d8);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1708);
        assert_eq!(RWE_LAYOUT_NOP_LEN,44);
    }
}
