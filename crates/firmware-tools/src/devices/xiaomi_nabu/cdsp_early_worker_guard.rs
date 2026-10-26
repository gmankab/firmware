// SPDX-License-Identifier: GPL-3.0-only
//! Guard other hardware threads' pending interrupt/controller state before
//! continuing the worker. Hexagon V66 GP load packets and three predicated
//! branches are represented as typed operands; no OEM bytes are embedded.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P};
use super::cdsp_early_worker_thread;
pub const FILE_OFFSET:usize=0x4d78;
pub const REGION_END:usize=0x4db0;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1d78;
fn emit(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum GuardBranch { EqualTaken, InvertedNotTaken }
/// Compound P0=cmp.eq(Rsource,#0);if(.new)jump where all relative offsets
/// count halfwords from the start of this branch's instruction packet.
fn compare_guard(src:Gpr,source:u8,mode:GuardBranch,here:u32,target:u32)->Result<u32,String>{
    if src.number()!=1 || source!=0 || here%4!=0 || target%4!=0 || target<=here {
        return Err("worker branch needs R1 eq #0, aligned forward relative target".into());
    }
    let half=(target-here)/2;
    if half==0||half>0x1fe{return Err("worker branch displacement beyond checked V66 range".into());}
    let base=match mode { GuardBranch::EqualTaken=>0x1001_e000, GuardBranch::InvertedNotTaken=>0x1041_c000 };
    Ok(base|half)
}
/// Compact pair of word-clearing stores for a specified general-purpose
/// controller base. Hexagon duplex fields encode each quarterword offset.
fn two_zero_stores(base:Gpr,first:u8,second:u8)->Result<u32,String>{
    if base.number()>7||first>124||second>124||first%4!=0||second%4!=0 {
        return Err("Hexagon duplex zero stores require small GPR base and aligned offsets".into());
    }
    Ok(0xf000_1000 |(u32::from(base.number())<<20) |(u32::from(first/4)<<16)
       |(u32::from(base.number())<<4) |u32::from(second/4))
}
fn clear_controller_bit(base:Gpr,bit:u8)->Result<u32,String>{
    if base.number()>7||bit>=32{return Err("atomic controller clearbit register/bit out of bounds".into());}
    Ok(0x3f40_c0c0|(u32::from(base.number())<<16)|u32::from(bit))
}

pub fn build_worker_guard()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    let state=cdsp_boot::encode_extended_gp_word_load(r(0)?,0xfe0a_bbf8);
    emit(&mut out,state[0],P::ContinuationA);
    emit(&mut out,state[1],P::ContinuationA);
    let active=cdsp_boot::encode_extended_gp_word_load(r(1)?,0xfe0a_c0a0);
    emit(&mut out,active[0],P::ContinuationA);
    emit(&mut out,active[1],P::Final);
    emit(&mut out,compare_guard(r(1)?,0,GuardBranch::EqualTaken,0x96c0_1d88,0x96c0_1db0)?,P::ContinuationA);
    let standby=cdsp_boot::encode_extended_gp_word_load(r(1)?,0xfe0a_5ddc);
    emit(&mut out,standby[0],P::ContinuationA);
    emit(&mut out,standby[1],P::Final);
    emit(&mut out,compare_guard(r(1)?,0,GuardBranch::InvertedNotTaken,0x96c0_1d94,0x96c0_1db0)?,P::ContinuationA);
    let cleanup=cdsp_boot::encode_extended_gp_word_load(r(1)?,0xfe03_7ad0);
    emit(&mut out,cleanup[0],P::ContinuationA);
    emit(&mut out,cleanup[1],P::Final);
    emit(&mut out,compare_guard(r(1)?,0,GuardBranch::EqualTaken,0x96c0_1da0,0x96c0_1db0)?,P::Final);
    for (first,second) in [(36,32),(16,20)] {
        out.extend_from_slice(&two_zero_stores(r(1)?,first,second)?.to_le_bytes());
    }
    emit(&mut out,clear_controller_bit(r(1)?,21)?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP worker guard instruction count mismatch".into());}
    Ok(out)
}
pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_worker_thread::verify_reference(reference)?;
    if cdsp_early_worker_thread::REGION_END!=FILE_OFFSET {return Err("worker guard must follow thread init".into());}
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_worker_guard()?.as_slice()){
        return Err("CDSP worker guard differs from source reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn guard_branches_and_duplex_stores_are_bounded(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,56);
        assert_eq!(build_worker_guard().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(compare_guard(r(1),0,GuardBranch::EqualTaken,0x96c0_1da0,0x96c0_1db0).unwrap(),0x1001_e008);
        assert_eq!(compare_guard(r(1),0,GuardBranch::InvertedNotTaken,0x96c0_1d94,0x96c0_1db0).unwrap(),0x1041_c00e);
        assert!(compare_guard(r(2),0,GuardBranch::EqualTaken,0x96c0_1da0,0x96c0_1db0).is_err());
        assert!(compare_guard(r(1),1,GuardBranch::EqualTaken,0x96c0_1da0,0x96c0_1db0).is_err());
        assert_eq!(two_zero_stores(r(1),36,32).unwrap(),0xf019_1018);
        assert!(two_zero_stores(r(1),35,32).is_err());
        assert_eq!(clear_controller_bit(r(1),21).unwrap(),0x3f41_c0d5);
        assert!(clear_controller_bit(r(1),32).is_err());
    }
}
