// SPDX-License-Identifier: GPL-3.0-only
//! Guarded CDSP codec / HW format classification, first instruction packets.
//! This is the beginning of a longer routine: no synthetic return or raw OEM
//! instruction words are included. Conditions are re-encoded from operands.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P, ScalarPacket as H};
use super::cdsp_early_cache_helpers as previous;

pub const FILE_OFFSET: usize = 0x5488;
pub const REGION_END: usize = 0x5538;
pub const FUNCTION_BYTES: usize = REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS: u32 = 0x96c0_2488;

pub(super) fn put(out:&mut Vec<u8>,op:u32,marker:P){out.extend_from_slice(&marker.apply(op).to_le_bytes());}
fn allocframe(bytes:u16)->Result<u32,String>{
    if bytes==0 || bytes%8!=0 || bytes>0x1ff8{return Err("Hexagon frame must be nonzero and 8-byte aligned".into());}
    Ok(0xa09d_c000|u32::from(bytes/8))
}
pub(super) fn small_and(dst:Gpr,src:Gpr,mask:u16)->Result<u32,String>{
    // The small V66 form contains an 8-bit unsigned mask in bits 12:5.
    if mask>255 {return Err("narrow Hexagon AND mask exceeds 8 bits".into());}
    Ok(0x7600_c000|(u32::from(src.number())<<16)|(u32::from(mask)<<5)|u32::from(dst.number()))
}
pub(super) fn compare_p0_imm(src:Gpr,value:u8)->Result<u32,String>{
    if value>127 {return Err("Hexagon small immediate comparison >127".into());}
    Ok(0x7500_c000|(u32::from(src.number())<<16)|(u32::from(value)<<5))
}
pub(super) fn p0_assign_small(dst:Gpr,value:u8)->Result<u32,String>{
    if value>127 {return Err("Hexagon predicated assignment >127".into());}
    Ok(0x7e00_c000|(u32::from(value)<<5)|u32::from(dst.number()))
}
pub(super) fn p0new_jump(packet_start:u32,target:u32,predicted_taken:bool,negated:bool)->Result<u32,String>{
    if packet_start%4!=0||target%4!=0||target<=packet_start||target-packet_start>0x3fe {
        return Err("P0.new branch must be positive, 4-byte aligned and within 0x3fe bytes".into());
    }
    let half=(target-packet_start)/2;
    // Hexagon P0.new packet-relative branch with prediction and negation
    // in the separate opcode family fields. Verified by independent LLVM6.
    let base=if negated {0x5c20_c800} else if predicted_taken {0x5c00_d800} else {0x5c00_c800};
    Ok(base|((half&0x100)<<5)|(half&0xff))
}
pub(super) fn extended_compare_p0(src:Gpr,value:u32)->[u32;2]{
    let ext=((value>>20)<<16)|((value>>6)&0x3fff)|0x4000;
    let cmp=0x7500_c000|(u32::from(src.number())<<16)|((value&0x3f)<<5);
    [ext,cmp]
}

/// Decode the format bit-class and branch to the corresponding hardware
/// stream setup. Current fragment ends at the next case, where continuation
/// will be reconstructed in the next checkpoint.
pub fn build_format_guard_prefix()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,allocframe(16)?,P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(9)?,value:0}.encode()?,P::Final);
    put(&mut out,H::AssignRegister{dst:r(11)?,src:r(0)?}.encode()?,P::Final);
    put(&mut out,H::LogicalShiftRight{dst:r(11)?,src:r(11)?,amount:14}.encode()?,P::Final);
    put(&mut out,small_and(r(11)?,r(11)?,3)?,P::Final);
    put(&mut out,H::ArithmeticShiftLeft{dst:r(11)?,src:r(11)?,amount:30}.encode()?,P::Final);
    put(&mut out,H::PredicateP0EqualsZero{src:r(11)?}.encode()?,P::Final);
    put(&mut out,p0_assign_small(r(9)?,1)?,P::Final);
    put(&mut out,compare_p0_imm(r(9)?,1)?,P::ContinuationA);
    put(&mut out,p0new_jump(0x96c0_24a8,0x96c0_2680,true,false)?,P::Final);

    put(&mut out,H::AssignSmallImmediate{dst:r(9)?,value:0}.encode()?,P::Final);
    put(&mut out,H::AssignRegister{dst:r(11)?,src:r(0)?}.encode()?,P::Final);
    put(&mut out,H::LogicalShiftRight{dst:r(11)?,src:r(11)?,amount:27}.encode()?,P::Final);
    put(&mut out,small_and(r(11)?,r(11)?,31)?,P::Final);
    put(&mut out,H::ArithmeticShiftLeft{dst:r(11)?,src:r(11)?,amount:27}.encode()?,P::Final);
    let compare1=extended_compare_p0(r(11)?,0x1000_0000);
    put(&mut out,compare1[0],P::ContinuationA);put(&mut out,compare1[1],P::Final);
    put(&mut out,p0_assign_small(r(9)?,1)?,P::Final);
    put(&mut out,compare_p0_imm(r(9)?,1)?,P::ContinuationA);
    put(&mut out,p0new_jump(0x96c0_24d0,0x96c0_2504,false,true)?,P::Final);

    put(&mut out,H::AssignSmallImmediate{dst:r(9)?,value:0}.encode()?,P::Final);
    put(&mut out,H::AssignRegister{dst:r(11)?,src:r(0)?}.encode()?,P::Final);
    put(&mut out,H::LogicalShiftRight{dst:r(11)?,src:r(11)?,amount:25}.encode()?,P::Final);
    put(&mut out,small_and(r(11)?,r(11)?,3)?,P::Final);
    put(&mut out,H::ArithmeticShiftLeft{dst:r(11)?,src:r(11)?,amount:30}.encode()?,P::Final);
    let compare2=extended_compare_p0(r(11)?,0xc000_0000);
    put(&mut out,compare2[0],P::ContinuationA);put(&mut out,compare2[1],P::Final);
    put(&mut out,p0_assign_small(r(9)?,1)?,P::Final);
    put(&mut out,compare_p0_imm(r(9)?,1)?,P::ContinuationA);
    put(&mut out,p0new_jump(0x96c0_24f8,0x96c0_2504,false,false)?,P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0x2c}.encode()?,P::Final);

    put(&mut out,H::AssignSmallImmediate{dst:r(9)?,value:0}.encode()?,P::Final);
    put(&mut out,H::AssignRegister{dst:r(11)?,src:r(0)?}.encode()?,P::Final);
    put(&mut out,H::LogicalShiftRight{dst:r(11)?,src:r(11)?,amount:27}.encode()?,P::Final);
    put(&mut out,small_and(r(11)?,r(11)?,31)?,P::Final);
    put(&mut out,H::ArithmeticShiftLeft{dst:r(11)?,src:r(11)?,amount:27}.encode()?,P::Final);
    let compare3=extended_compare_p0(r(11)?,0x2000_0000);
    put(&mut out,compare3[0],P::ContinuationA);put(&mut out,compare3[1],P::Final);
    put(&mut out,p0_assign_small(r(9)?,1)?,P::Final);
    put(&mut out,compare_p0_imm(r(9)?,1)?,P::ContinuationA);
    put(&mut out,p0new_jump(0x96c0_2524,0x96c0_2538,false,true)?,P::Final);
    let unavailable=cdsp_boot::encode_extended_assignment(r(0)?,-0x2002,false);
    put(&mut out,unavailable[0],P::ContinuationA);
    put(&mut out,unavailable[1],P::ContinuationA);
    put(&mut out,H::JumpRelative{displacement_bytes:0x34c}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err(format!("format guard generated {} bytes, expected {}",out.len(),FUNCTION_BYTES));}
    Ok(out)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    previous::verify_reference(reference)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("format guard is not adjacent to CDSP cache helper group".into());}
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_format_guard_prefix()?.as_slice()){
        return Err("CDSP format guard bytes differ from pinned original".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn format_guard_encodes_bitfields_branches_and_boundary(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,176);
        assert_eq!(build_format_guard_prefix().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(allocframe(16).unwrap(),0xa09d_c002);
        assert_eq!(small_and(r(11),r(11),3).unwrap(),0x760b_c06b);
        assert_eq!(compare_p0_imm(r(9),1).unwrap(),0x7509_c020);
        assert_eq!(p0_assign_small(r(9),1).unwrap(),0x7e00_c029);
        assert_eq!(p0new_jump(0x96c0_24a8,0x96c0_2680,true,false).unwrap(),0x5c00_d8ec);
        assert_eq!(p0new_jump(0x96c0_24d0,0x96c0_2504,false,true).unwrap(),0x5c20_c81a);
        assert_eq!(p0new_jump(0x96c0_24f8,0x96c0_2504,false,false).unwrap(),0x5c00_c806);
        assert_eq!(p0new_jump(0x96c0_2524,0x96c0_2538,false,true).unwrap(),0x5c20_c80a);
        assert!(p0new_jump(0x96c0_24d0,0x96c0_24d0,false,true).is_err());
        assert!(p0new_jump(0x96c0_24d0,0x96c0_30d0,false,true).is_err());
        assert!(small_and(r(11),r(11),256).is_err());
    }
}
