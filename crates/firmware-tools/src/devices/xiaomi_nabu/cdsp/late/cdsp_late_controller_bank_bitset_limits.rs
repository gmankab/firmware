// SPDX-License-Identifier: GPL-3.0-only
//! CDSP Hexagon V66 bank-bitmap capacity guards, reconstructed from typed
//! register fields, independent LLVM6 semantics and Rizin packet boundaries.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_bank_bitset_scan;
pub const FILE_OFFSET:usize=0x56bd0;
pub const END_FILE_OFFSET:usize=0x56bf0;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8bd0;
pub const RWE_START:usize=cdsp_late_controller_bank_bitset_scan::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_bank_bitset_scan::RWE_PREFIX_LAYOUT_LEN;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_bank_bitset_scan::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
const BAD_PLANE:u32=0x96cc_8d40;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn put(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}
/// `R4=and(R1,R3)` independently assembled using distinct operand fields.
pub fn encode_bitmap_mask_and(dst:G,lhs:G,rhs:G)->Result<u32,String>{
    if (dst.number(),lhs.number(),rhs.number())!=(4,1,3) {
        return Err("unverified CDSP bitmap mask register tuple".into());
    }
    Ok(0xf100_c000 | (u32::from(lhs.number())<<16) |
       (u32::from(rhs.number())<<8) | u32::from(dst.number()))
}
/// Fused unsigned `if(cmp.gtu(R17.new,R19)) jump:nt`; distance measured
/// from original Hexagon packet start, not the separate branch instruction.
pub fn encode_reject_high_plane(new_index:G,limit:G,packet:u32,target:u32)->Result<u32,String>{
    if (new_index.number(),limit.number(),packet,target)!=(17,19,PHYSICAL_ADDRESS+8,BAD_PLANE){
        return Err("unverified upper-plane comparison operands or target".into());
    }
    let bytes=target.checked_sub(packet).ok_or("upper-plane guard backwards")?;
    if bytes==0 ||bytes>0x1fe ||bytes&1!=0{return Err("upper-plane guard distance invalid".into());}
    Ok(0x2102_d300 | ((bytes>>1)&0xff))
}
/// Accumulate `R4+=add(R16,##-1)` in the independently verified V66
/// register-and-negative-immediate instruction family.
pub fn encode_plus_shifted_bank(dst:G,src:G,constant:i16)->Result<u32,String>{
    if (dst.number(),src.number(),constant)!=(4,16,-1){
        return Err("unverified CDSP bank accumulator operands or immediate".into());
    }
    Ok(0xe200_5fe0 | (u32::from(src.number())<<16) | u32::from(dst.number()))
}
/// Fused check `if(cmp.gtu(R18,R4.new)) jump:nt`, relative to packet start.
pub fn encode_reject_plane_capacity(limit:G,new_count:G,packet:u32,target:u32)->Result<u32,String>{
    if (limit.number(),new_count.number(),packet,target)!=(18,4,PHYSICAL_ADDRESS+0x14,BAD_PLANE){
        return Err("unverified secondary plane limit or packet origin".into());
    }
    let bytes=target.checked_sub(packet).ok_or("secondary plane guard backwards")?;
    if bytes==0||bytes>0x1fe||bytes&1!=0{return Err("secondary plane branch invalid".into());}
    Ok(0x2202_d200 | ((bytes>>1)&0xff))
}
pub fn build_bitmap_limits()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    put(&mut out,S::ReverseSubtractImmediate{dst:r(3)?,src:r(16)?,value:0}.encode()?);
    put(&mut out,encode_bitmap_mask_and(r(4)?,r(1)?,r(3)?)?);
    put(&mut out,M::ContinuationA.apply(S::ExtractUnsigned{dst:r(17)?,src:r(4)?,width:20,offset:0}.encode()?));
    put(&mut out,encode_reject_high_plane(r(17)?,r(19)?,PHYSICAL_ADDRESS+8,BAD_PLANE)?);
    put(&mut out,S::AssignRegister{dst:r(4)?,src:r(17)?}.encode()?);
    put(&mut out,M::ContinuationA.apply(encode_plus_shifted_bank(r(4)?,r(16)?,-1)?));
    put(&mut out,encode_reject_plane_capacity(r(18)?,r(4)?,PHYSICAL_ADDRESS+0x14,BAD_PLANE)?);
    put(&mut out,M::ContinuationA.apply(S::ExtractUnsigned{dst:r(4)?,src:r(0)?,width:23,offset:1}.encode()?));
    if out.len()!=EXECUTABLE_LEN{return Err("CDSP bitmap limits must have eight words".into());}
    Ok(out)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_bank_bitset_scan::build_prefix()?;
    out.extend(build_bitmap_limits()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN || out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN {
        return Err("bitmap-limit RWE file layout accounting wrong".into());
    }
    Ok(out)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bitmap_limits()?.as_slice()){
        return Err("CDSP bitmap limits differ from reference".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("CDSP bitmap limits joined RWE prefix differs from reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn bitmap_capacity_checks_use_original_fields(){
        let b=build_bitmap_limits().unwrap();assert_eq!(b.len(),32);
        let w:Vec<u32>=b.chunks_exact(4).map(|x|u32::from_le_bytes(x.try_into().unwrap())).collect();
        assert_eq!(w,[0x7650_c003,0xf101_c304,0x8d04_5411,0x2102_d3b4,
                     0x7071_c004,0xe210_5fe4,0x2202_d2ae,0x8d00_5724]);
        assert!(encode_bitmap_mask_and(r(4).unwrap(),r(2).unwrap(),r(3).unwrap()).is_err());
        assert!(encode_reject_high_plane(r(17).unwrap(),r(19).unwrap(),PHYSICAL_ADDRESS+4,BAD_PLANE).is_err());
        assert!(encode_plus_shifted_bank(r(4).unwrap(),r(16).unwrap(),0).is_err());
        assert!(encode_reject_plane_capacity(r(18).unwrap(),r(4).unwrap(),PHYSICAL_ADDRESS+0x14,BAD_PLANE+2).is_err());
    }
    #[test]fn joined_prefix_includes_distinct_alignment_bytes(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,3056);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2996);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,60);
        assert_eq!(build_prefix().unwrap().len(),3056);
    }
}
