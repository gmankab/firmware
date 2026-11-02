// SPDX-License-Identifier: GPL-3.0-only
//! CDSP worker packet: predicate ownership check, atomic 32-bit lock read,
//! conditionally jump to another hardware thread, then allocate a small
//! frame and issue the state callback. The function continues beyond this
//! verified instruction interval; no artificial return or padding is added.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::{cdsp_early_worker_guard};
pub const FILE_OFFSET:usize=0x4db0;
pub const REGION_END:usize=0x4e04;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1db0;
fn put(out:&mut Vec<u8>,insn:u32,marker:P){out.extend_from_slice(&marker.apply(insn).to_le_bytes());}
fn signed_extract(dst:Gpr,src:Gpr,width:u8,offset:u8)->Result<u32,String>{
    if width==0||width>31||offset>=32||u16::from(width)+u16::from(offset)>32 {
        return Err("V66 signed extract field outside GPR bounds".into());
    }
    Ok(0x8d80_c000|(u32::from(src.number())<<16)|(u32::from(width)<<8)
       |(u32::from(offset&15)<<5)|u32::from(dst.number())
       |(u32::from(offset&16)<<18))
}
fn compare_regs(pred:u8,left:Gpr,right:Gpr)->Result<u32,String>{
    if pred>3{return Err("V66 comparison predicate outside P0..P3".into());}
    Ok(0xf200_c000|(u32::from(left.number())<<16)|(u32::from(right.number())<<8)|u32::from(pred))
}
fn and_predicates(dst:u8,left:u8,right:u8)->Result<u32,String>{
    if dst>3||left>3||right>3{return Err("V66 boolean predicate outside P0..P3".into());}
    Ok(0x6b00_c000|(u32::from(right)<<16)|(u32::from(left)<<8)|u32::from(dst))
}
/// Direct packet-relative NEW-predicate not-taken branch. The predicate
/// selector sits in bits 9:8; displacement halfword bit 8 becomes bit 13.
fn branch_predicate_new_nt(pred:u8,start:u32,target:u32)->Result<u32,String>{
    if pred>3||start%4!=0||target%4!=0||target<=start {
        return Err("new predicate branch needs P0..P3 and aligned forward target".into());
    }
    let delta=target-start;
    if delta>0x3fe||delta%2!=0{return Err("V66 predicate branch too distant".into());}
    let half=delta/2;
    Ok(0x5c00_c800|(u32::from(pred)<<8)|((half&0x100)<<5)|(half&0xff))
}
fn read_thread(dst:Gpr)->u32{0x6e88_c000|u32::from(dst.number())}
fn read_locked_word(dst:Gpr,base:Gpr)->u32{0x9200_c000|(u32::from(base.number())<<16)|u32::from(dst.number())}
fn test_bit_predicate_reg(dst:u8,src:Gpr,index:Gpr)->Result<u32,String>{
    if dst>3{return Err("predicate number out of bounds".into());}
    Ok(0xc700_c000|(u32::from(src.number())<<16)|(u32::from(index.number())<<8)|u32::from(dst))
}
fn write_predicate_group(src:Gpr)->u32{0x6220_c004|(u32::from(src.number())<<16)}
fn allocframe(size:u16)->Result<u32,String>{
    if size==0||size>0x1ff8||size%8!=0{return Err("worker frame must be nonzero and eight-byte aligned".into());}
    Ok(0xa09d_c000|u32::from(size/8))
}
fn store_pair(low:Gpr,base:Gpr)->Result<u32,String>{
    if low.number()%2!=0{return Err("Hexagon 64-bit store needs even low register".into());}
    Ok(0xa1c0_c000|(u32::from(base.number())<<16)|(u32::from(low.number())<<8))
}
/// Direct call packet relative from its first instruction (MPYI) with the
/// Hexagon displacement split into 14 low and eight upper bits.
fn checked_worker_call(start:u32,target:u32)->Result<u32,String>{
    if start%4!=0||target%4!=0{return Err("CDSP worker CALL address not word aligned".into());}
    let d=i64::from(target)-i64::from(start);
    if d%2!=0||!(-0x100000..=0xffffc).contains(&d){return Err("CDSP worker CALL displacement outside signed field".into());}
    let half=(d/2) as i32 as u32;
    Ok(0x5a00_c000|((half>>14&0xff)<<16)|(half&0x3fff))
}
fn mpyi_accumulate(dst:Gpr,src:Gpr,factor:u8)->u32{
    0xe100_c000|(u32::from(src.number())<<16)|(u32::from(factor)<<5)|u32::from(dst.number())
}

pub fn build_worker_lock_prefix()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,signed_extract(r(19)?,r(0)?,3,0)?,P::ContinuationA);
    put(&mut out,H::TestBitPredicate{predicate:0,src:r(0)?,bit:30}.encode()?,P::Final);
    put(&mut out,read_thread(r(1)?),P::Final);
    put(&mut out,compare_regs(1,r(19)?,r(1)?)?,P::Final);
    put(&mut out,and_predicates(2,0,1)?,P::ContinuationA);
    put(&mut out,branch_predicate_new_nt(2,0x96c0_1dc0,0x96c0_1f24)?,P::Final);
    let addr=cdsp_boot::encode_extended_assignment(r(19)?,-0x1fc8dac,true);
    put(&mut out,addr[0],P::ContinuationA);
    put(&mut out,addr[1],P::Final);
    put(&mut out,read_locked_word(r(0)?,r(19)?),P::Final);
    put(&mut out,test_bit_predicate_reg(0,r(0)?,r(1)?)?,P::ContinuationA);
    put(&mut out,branch_predicate_new_nt(0,0x96c0_1dd4,0x96c0_2040)?,P::Final);
    put(&mut out,write_predicate_group(r(28)?),P::ContinuationA);
    let base=cdsp_boot::encode_extended_assignment(r(0)?,-0x1f54408,false);
    put(&mut out,base[0],P::ContinuationA);
    put(&mut out,base[1],P::ContinuationA);
    put(&mut out,H::AssignRegister{dst:r(28)?,src:r(1)?}.encode()?,P::Final);
    put(&mut out,allocframe(8)?,P::Final);
    put(&mut out,H::AddSmallImmediate{dst:r(1)?,src:r(0)?,value:104}.encode()?,P::ContinuationA);
    put(&mut out,store_pair(r(0)?,r(29)?)?,P::Final);
    put(&mut out,mpyi_accumulate(r(1)?,r(28)?,224),P::ContinuationA);
    put(&mut out,checked_worker_call(0x96c0_1df8,0x96c3_251c)?,P::ContinuationA);
    put(&mut out,H::AssignRegister{dst:r(0)?,src:r(18)?}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES {return Err("CDSP worker lock prefix instruction count mismatch".into());}
    Ok(out)
}
pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_worker_guard::verify_reference(reference)?;
    if cdsp_early_worker_guard::REGION_END!=FILE_OFFSET{return Err("CDSP worker lock does not follow guard".into());}
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_worker_lock_prefix()?.as_slice()) {
        return Err("CDSP worker lock prefix differs from original firmware".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn worker_lock_predicates_and_frame_are_checked(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,84);
        assert_eq!(build_worker_lock_prefix().unwrap().len(),84);
        assert_eq!(branch_predicate_new_nt(2,0x96c0_1dc0,0x96c0_1f24).unwrap(),0x5c00_cab2);
        assert_eq!(branch_predicate_new_nt(0,0x96c0_1dd4,0x96c0_2040).unwrap(),0x5c00_e836);
        assert!(branch_predicate_new_nt(4,0x96c0_1dc0,0x96c0_1f24).is_err());
        assert!(branch_predicate_new_nt(0,0x96c0_1dc0,0x96c0_3fc0).is_err());
        assert_eq!(allocframe(8).unwrap(),0xa09d_c001);
        assert!(allocframe(7).is_err());
        assert_eq!(checked_worker_call(0x96c0_1df8,0x96c3_251c).unwrap(),0x5a06_c392);
        assert!(checked_worker_call(0x96c0_1df9,0x96c3_251c).is_err());
        assert_eq!(signed_extract(r(19),r(0),3,0).unwrap(),0x8d80_c313);
    }
}
