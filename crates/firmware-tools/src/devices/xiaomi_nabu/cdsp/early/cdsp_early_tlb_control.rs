// SPDX-License-Identifier: GPL-3.0-only
//! Three small Hexagon V66 TLB control routines: page-table probe, selected
//! TLB pair write and read. All instructions are explicit typed encodings.
//! The 44-byte linker-zero region following them is not executable source.
use super::cdsp_boot::{GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_tlb_encode as previous;

pub const FILE_OFFSET:usize=0x5bc0;
pub const REGION_END:usize=0x5c14;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const ALIGNMENT_END:usize=0x5c40;
pub const ALIGNMENT_BYTES:usize=ALIGNMENT_END-REGION_END;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2bc0;
fn emit(out:&mut Vec<u8>,word:u32,p:P){out.extend_from_slice(&p.apply(word).to_le_bytes());}
#[derive(Clone,Copy)]enum TlbOp{Lock,Unlock,Synchronize}
fn tlb_op(op:TlbOp)->u32{match op{
    TlbOp::Lock=>0x6c20_c020,TlbOp::Unlock=>0x6c20_c040,TlbOp::Synchronize=>0x57c0_c002,
}}
fn tlb_probe(dst:Gpr,index:Gpr)->u32{
    0x6c80_c000|(u32::from(index.number())<<16)|u32::from(dst.number())
}
fn tlb_read(low:Gpr,index:Gpr)->Result<u32,String>{
    if low.number()>30||low.number()%2!=0{return Err("TLBR needs even low register in a 64-bit pair".into());}
    Ok(0x6c40_c000|(u32::from(index.number())<<16)|u32::from(low.number()))
}
fn tlb_write(low:Gpr,index:Gpr)->Result<u32,String>{
    if low.number()>30||low.number()%2!=0{return Err("TLBW needs even low register in a 64-bit pair".into());}
    Ok(0x6c00_c000|(u32::from(low.number())<<16)|u32::from(index.number()))
}
fn if_p0_new_goto(packet:u32,target:u32)->Result<u32,String>{
    if packet%4!=0||target%4!=0||target<=packet||target-packet>0x1fc {
        return Err("V66 new-predicate TLB probe branch out of tested range".into());
    }
    Ok(0x5c00_c800 | ((target-packet)/2))
}
fn negative_one(dst:Gpr)->Result<u32,String>{
    if dst.number()!=0 {return Err("V66 negative-one TLB probe sentinel only verified for R0".into());}
    Ok(0x78df_ffe0)
}

pub fn build_tlb_control()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    // Probe an address and either return its status or -1 on TLB hit.
    emit(&mut out,tlb_op(TlbOp::Lock),P::Final);
    emit(&mut out,H::ArithmeticShiftLeft{dst:r(1)?,src:r(1)?,amount:20}.encode()?,P::Final);
    emit(&mut out,H::LogicalShiftRight{dst:r(0)?,src:r(0)?,amount:12}.encode()?,P::Final);
    emit(&mut out,H::InsertBitField{dst:r(1)?,src:r(0)?,width:20,offset:0}.encode()?,P::Final);
    emit(&mut out,tlb_probe(r(0)?,r(1)?),P::Final);
    emit(&mut out,tlb_op(TlbOp::Unlock),P::Final);
    emit(&mut out,H::TestBitPredicate{predicate:0,src:r(0)?,bit:31}.encode()?,P::ContinuationA);
    emit(&mut out,if_p0_new_goto(0x96c0_2bd8,0x96c0_2be4)?,P::Final);
    emit(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    emit(&mut out,negative_one(r(0)?)?,P::Final);
    emit(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=0x2c {return Err("CDSP TLB probe should end at 0x5bec".into());}
    // Install a given 64-bit TLB entry and synchronize.
    emit(&mut out,tlb_op(TlbOp::Lock),P::Final);
    emit(&mut out,tlb_write(r(2)?,r(0)?)?,P::Final);
    emit(&mut out,tlb_op(TlbOp::Synchronize),P::Final);
    emit(&mut out,tlb_op(TlbOp::Unlock),P::Final);
    emit(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:0}.encode()?,P::Final);
    emit(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=0x44 {return Err("CDSP TLB write helper should end at 0x5c04".into());}
    // Read a TLB entry indexed by the caller's R0.
    emit(&mut out,tlb_op(TlbOp::Lock),P::Final);
    emit(&mut out,tlb_read(r(0)?,r(0)?)?,P::Final);
    emit(&mut out,tlb_op(TlbOp::Unlock),P::Final);
    emit(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP TLB probe/read/write have missing words".into());}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    previous::verify_reference(original)?;
    if previous::REGION_END!=FILE_OFFSET ||
       original.get(FILE_OFFSET..REGION_END)!=Some(build_tlb_control()?.as_slice()){
        return Err("CDSP TLB control helpers differ from pinned firmware".into());
    }
    if original.get(REGION_END..ALIGNMENT_END)!=Some([0u8;ALIGNMENT_BYTES].as_slice()){
        return Err("CDSP TLB utility linker alignment must be 44 zero bytes".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn tlb_control_profiles_have_checked_layout_and_register_fields(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,84);
        assert_eq!(ALIGNMENT_BYTES,44);
        assert_eq!(tlb_probe(r(0),r(1)),0x6c81_c000);
        assert_eq!(tlb_write(r(2),r(0)).unwrap(),0x6c02_c000);
        assert_eq!(tlb_read(r(0),r(0)).unwrap(),0x6c40_c000);
        assert!(tlb_write(r(1),r(0)).is_err());
        assert!(tlb_read(r(3),r(0)).is_err());
        assert_eq!(if_p0_new_goto(0x96c0_2bd8,0x96c0_2be4).unwrap(),0x5c00_c806);
        assert!(if_p0_new_goto(0x96c0_2bd8,0x96c0_2bd4).is_err());
        assert_eq!(negative_one(r(0)).unwrap(),0x78df_ffe0);
        assert!(negative_one(r(1)).is_err());
        assert_eq!(build_tlb_control().unwrap().len(),FUNCTION_BYTES);
    }
}
