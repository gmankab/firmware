// SPDX-License-Identifier: GPL-3.0-only
//! Construct a 64-bit Hexagon TLB descriptor from address, size and attribute
//! fields. The 21 V66 scalar instructions are independently LLVM6-assembled
//! and source-encoded with checked register/shift/bitfield operations.
use super::cdsp_boot::{GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_tlb_utilities as previous;

pub const FILE_OFFSET:usize=0x5b6c;
pub const REGION_END:usize=0x5bc0;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2b6c;
fn emit(out:&mut Vec<u8>,word:u32,p:P){out.extend_from_slice(&p.apply(word).to_le_bytes());}

fn load_sp_word(dst:Gpr,offset:u16)->Result<u32,String>{
    if offset>1020||offset%4!=0{return Err("stack word load offset must be aligned and <1024".into());}
    Ok(0x9180_c000|(29u32<<16)|(u32::from(offset)<<3)|u32::from(dst.number()))
}
fn or_reg(dst:Gpr,left:Gpr,right:Gpr)->u32{
    0xf120_c000|(u32::from(left.number())<<16)
       |(u32::from(right.number())<<8)|u32::from(dst.number())
}
fn variable_asl(dst:Gpr,source:Gpr,amount:Gpr)->u32{
    0xc640_c080|(u32::from(source.number())<<16)
       |(u32::from(amount.number())<<8)|u32::from(dst.number())
}
fn jump_if_not_new_equal_minus_one(packet:u32,target:u32,source:Gpr)->Result<u32,String>{
    if source.number()!=5||packet%4!=0||target%4!=0||target<=packet||target-packet>0xfe{
        return Err("Hexagon negative-one new-value branch outside checked profile".into());
    }
    Ok(0x11c0_e000|(u32::from(source.number())<<16)|((target-packet)/2))
}
fn combine_register_pair(low:Gpr,high_source:Gpr,low_source:Gpr)->Result<u32,String>{
    if low.number()%2!=0||low.number()>30{
        return Err("Hexagon pair destination must use even low register".into());
    }
    Ok(0xf500_c000 |(u32::from(high_source.number())<<16)
       |(u32::from(low_source.number())<<8)|u32::from(low.number()))
}

pub fn build_tlb_entry()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    emit(&mut out,load_sp_word(r(6)?,0)?,P::Final);
    emit(&mut out,H::LogicalShiftRight{dst:r(8)?,src:r(1)?,amount:12}.encode()?,P::ContinuationA);
    emit(&mut out,H::ArithmeticShiftLeft{dst:r(12)?,src:r(6)?,amount:20}.encode()?,P::Final);
    emit(&mut out,or_reg(r(8)?,r(8)?,r(12)?),P::Final);
    emit(&mut out,H::ArithmeticShiftLeft{dst:r(8)?,src:r(8)?,amount:1}.encode()?,P::ContinuationA);
    emit(&mut out,H::AssignSmallImmediate{dst:r(14)?,value:1}.encode()?,P::Final);
    emit(&mut out,variable_asl(r(9)?,r(14)?,r(2)?),P::Final);
    emit(&mut out,H::ArithmeticShiftLeft{dst:r(10)?,src:r(3)?,amount:24}.encode()?,P::ContinuationA);
    emit(&mut out,H::ArithmeticShiftLeft{dst:r(11)?,src:r(4)?,amount:29}.encode()?,P::ContinuationA);
    emit(&mut out,or_reg(r(8)?,r(8)?,r(9)?),P::Final);
    emit(&mut out,or_reg(r(8)?,r(8)?,r(10)?),P::Final);
    emit(&mut out,or_reg(r(8)?,r(8)?,r(11)?),P::Final);
    emit(&mut out,H::LogicalShiftRight{dst:r(9)?,src:r(0)?,amount:12}.encode()?,P::ContinuationA);
    emit(&mut out,H::ArithmeticShiftLeft{dst:r(10)?,src:r(5)?,amount:20}.encode()?,P::Final);
    emit(&mut out,jump_if_not_new_equal_minus_one(0x96c0_2ba4,0x96c0_2bb0,r(5)?)?,P::Final);
    emit(&mut out,H::SetBit{dst:r(9)?,src:r(9)?,position:30}.encode()?,P::ContinuationA);
    emit(&mut out,H::AssignSmallImmediate{dst:r(10)?,value:0}.encode()?,P::Final);
    emit(&mut out,or_reg(r(9)?,r(9)?,r(10)?),P::Final);
    emit(&mut out,H::SetBit{dst:r(9)?,src:r(9)?,position:31}.encode()?,P::Final);
    emit(&mut out,combine_register_pair(r(0)?,r(9)?,r(8)?)?,P::Final);
    emit(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP 64-bit TLB descriptor requires 84 bytes".into());}
    Ok(out)
}
pub fn verify_reference(original:&[u8])->Result<(),String>{
    previous::verify_reference(original)?;
    if previous::REGION_END!=FILE_OFFSET || original.get(FILE_OFFSET..REGION_END)!=Some(build_tlb_entry()?.as_slice()) {
        return Err("CDSP TLB descriptor builder differs from reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn tlb_descriptor_encoding_has_checked_field_boundaries(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,84);
        assert_eq!(load_sp_word(r(6),0).unwrap(),0x919d_c006);
        assert!(load_sp_word(r(6),3).is_err());
        assert_eq!(or_reg(r(8),r(8),r(12)),0xf128_cc08);
        assert_eq!(variable_asl(r(9),r(14),r(2)),0xc64e_c289);
        assert_eq!(jump_if_not_new_equal_minus_one(0x96c0_2ba4,0x96c0_2bb0,r(5)).unwrap(),0x11c5_e006);
        assert!(jump_if_not_new_equal_minus_one(0x96c0_2ba4,0x96c0_2ba0,r(5)).is_err());
        assert_eq!(combine_register_pair(r(0),r(9),r(8)).unwrap(),0xf509_c800);
        assert!(combine_register_pair(r(1),r(9),r(8)).is_err());
        assert_eq!(build_tlb_entry().unwrap().len(),FUNCTION_BYTES);
    }
}
