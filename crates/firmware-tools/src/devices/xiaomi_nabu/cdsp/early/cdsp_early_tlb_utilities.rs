// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 supervisor TLB lookup/clear and indexed snapshot utilities.
//! Both functions use checked architecture fields; the privileged TLB
//! instructions are corroborated with Rizin semantics, not LLVM6 assembly.
use super::cdsp_boot::{GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_dispatch_helpers as previous;

pub const FILE_OFFSET:usize=0x5afc;
pub const REGION_END:usize=0x5b6c;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2afc;
fn emit(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}

fn tlb_control(operation:TlbControl)->u32{
    match operation{TlbControl::Lock=>0x6c20_c020,TlbControl::Unlock=>0x6c20_c040,TlbControl::Isync=>0x57c0_c002}
}
#[derive(Clone,Copy)]enum TlbControl{Lock,Unlock,Isync}
fn probe_tlb(dst:Gpr,index:Gpr)->u32{0x6c80_c000|(u32::from(index.number())<<16)|u32::from(dst.number())}
fn read_tlb(low:Gpr,index:Gpr)->Result<u32,String>{
    if low.number()%2!=0||low.number()>30{return Err("TLBR requires aligned 64-bit register pair".into());}
    Ok(0x6c40_c000|(u32::from(index.number())<<16)|u32::from(low.number()))
}
fn write_tlb(low:Gpr,index:Gpr)->Result<u32,String>{
    if low.number()%2!=0||low.number()>30{return Err("TLBW requires aligned 64-bit register pair".into());}
    Ok(0x6c00_c000|(u32::from(low.number())<<16)|u32::from(index.number()))
}
fn jump_if_p0_set(packet:u32,target:u32)->Result<u32,String>{
    if packet%4!=0||target%4!=0||target<=packet||target-packet>0xfe{
        return Err("CDSP TLB predicate jump needs bounded packet-relative target".into());
    }
    Ok(0x5c00_c000|(target-packet)/2)
}
fn return_zero(dst:Gpr,ret:Gpr)->Result<u32,String>{
    if dst.number()!=0||ret.number()!=31{return Err("compact return-zero verified for R0,LR only".into());}
    Ok(0x4800_3fc0)
}

pub fn build_tlb_utilities()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    // First helper: remove a TLB hit after clearing its valid bit.
    emit(&mut out,H::ArithmeticShiftLeft{dst:r(1)?,src:r(1)?,amount:20}.encode()?,P::ContinuationA);
    emit(&mut out,H::LogicalShiftRight{dst:r(0)?,src:r(0)?,amount:12}.encode()?,P::Final);
    emit(&mut out,H::InsertBitField{dst:r(1)?,src:r(0)?,width:20,offset:0}.encode()?,P::Final);
    emit(&mut out,tlb_control(TlbControl::Lock),P::Final);
    emit(&mut out,probe_tlb(r(0)?,r(1)?),P::Final);
    emit(&mut out,H::TestBitPredicate{predicate:0,src:r(0)?,bit:31}.encode()?,P::ContinuationA);
    emit(&mut out,H::AssignSmallImmediate{dst:r(11)?,value:0}.encode()?,P::Final);
    emit(&mut out,H::ExtractUnsigned{dst:r(0)?,src:r(0)?,width:8,offset:0}.encode()?,P::ContinuationA);
    emit(&mut out,jump_if_p0_set(0x96c0_2b18,0x96c0_2b30)?,P::Final);
    emit(&mut out,read_tlb(r(2)?,r(0)?)?,P::Final);
    emit(&mut out,H::ClearBit{dst:r(3)?,src:r(3)?,position:31}.encode()?,P::Final);
    emit(&mut out,write_tlb(r(2)?,r(0)?)?,P::Final);
    emit(&mut out,tlb_control(TlbControl::Isync),P::Final);
    emit(&mut out,tlb_control(TlbControl::Unlock),P::Final);
    emit(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=0x3c{return Err("V66 TLB invalidate routine should be 60 bytes".into());}
    // Second helper: return one TLB pair after probing and unlocking.
    emit(&mut out,H::ArithmeticShiftLeft{dst:r(5)?,src:r(1)?,amount:20}.encode()?,P::ContinuationA);
    emit(&mut out,H::LogicalShiftRight{dst:r(4)?,src:r(0)?,amount:12}.encode()?,P::Final);
    emit(&mut out,H::InsertBitField{dst:r(5)?,src:r(4)?,width:20,offset:0}.encode()?,P::Final);
    emit(&mut out,tlb_control(TlbControl::Lock),P::Final);
    emit(&mut out,probe_tlb(r(6)?,r(5)?),P::Final);
    emit(&mut out,H::TestBitPredicate{predicate:0,src:r(6)?,bit:31}.encode()?,P::Final);
    emit(&mut out,H::ExtractUnsigned{dst:r(6)?,src:r(6)?,width:8,offset:0}.encode()?,P::ContinuationA);
    emit(&mut out,jump_if_p0_set(0x96c0_2b50,0x96c0_2b64)?,P::Final);
    emit(&mut out,read_tlb(r(0)?,r(6)?)?,P::Final);
    emit(&mut out,tlb_control(TlbControl::Unlock),P::Final);
    emit(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    emit(&mut out,tlb_control(TlbControl::Unlock),P::Final);
    emit(&mut out,return_zero(r(0)?,r(31)?)?,P::ContinuationB);
    if out.len()!=FUNCTION_BYTES{return Err("V66 early TLB utility combined fragment not 112 bytes".into());}
    Ok(out)
}
pub fn verify_reference(original:&[u8])->Result<(),String>{
    previous::verify_reference(original)?;
    if previous::REGION_END!=FILE_OFFSET || original.get(FILE_OFFSET..REGION_END)!=Some(build_tlb_utilities()?.as_slice()){
        return Err("CDSP early TLB utility source differs from pinned original".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn tlb_lookup_and_invalidation_bitfields_are_bounded(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,112);
        assert_eq!(probe_tlb(r(0),r(1)),0x6c81_c000);
        assert_eq!(read_tlb(r(2),r(0)).unwrap(),0x6c40_c002);
        assert_eq!(write_tlb(r(2),r(0)).unwrap(),0x6c02_c000);
        assert!(read_tlb(r(3),r(0)).is_err());
        assert!(write_tlb(r(1),r(0)).is_err());
        assert_eq!(jump_if_p0_set(0x96c0_2b18,0x96c0_2b30).unwrap(),0x5c00_c00c);
        assert!(jump_if_p0_set(0x96c0_2b18,0x96c0_2b14).is_err());
        assert_eq!(return_zero(r(0),r(31)).unwrap(),0x4800_3fc0);
        assert!(return_zero(r(1),r(31)).is_err());
        assert_eq!(build_tlb_utilities().unwrap().len(),FUNCTION_BYTES);
    }
}
