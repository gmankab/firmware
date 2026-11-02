// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 HVX startup: initialize vector register patterns, reset all
//! vector register file slots, set the Q-lane multiplexed format masks, and
//! iterate across four HVX vector register groups.
//!
//! All 68 executable words are constructed from checked scalar/renderer
//! operand families. Packet markers and the `.new` backward branch remain
//! explicit. The input OEM code is never stored as an opcode array.
use super::cdsp_boot::{
    self, GeneralRegister as Gpr, PacketMarker as P, ScalarPacket as H,
    SystemRegister,
};
use super::cdsp_early_format_guard_final as previous;

pub const FILE_OFFSET: usize = 0x5880;
pub const REGION_END: usize = 0x5990;
pub const FUNCTION_BYTES: usize = REGION_END - FILE_OFFSET;
pub const PHYSICAL_ADDRESS: u32 = 0x96c0_2880;

/// Strictly bounded Hexagon vector-register operand (HVX V0–V31).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vreg(u8);
impl Vreg {
    fn new(index: u8) -> Result<Self, String> {
        if index > 31 { return Err(format!("V{} outside 32 HVX vector registers", index)); }
        Ok(Self(index))
    }
    fn bits(self) -> u32 { u32::from(self.0) }
}

/// Verified Hexagon V60/V66 HVX vector store, 128-byte postincrement.
fn store_vector(base:Gpr, src:Vreg, increment:u8) -> Result<u32,String> {
    if base.number()>31 || increment!=1 {
        return Err("VMEM vector postincrement only verified in units of one HVX vector".into());
    }
    Ok(0x2920_c100 | (u32::from(base.number())<<16) | src.bits())
}
/// Store current HVX lane mask, no pointer advance.
fn store_vector_current(base:Gpr, src:Vreg, offset:u8)->Result<u32,String>{
    if offset!=0 || base.number()>31{return Err("VMEM current vector requires zero offset".into());}
    Ok(0x2820_c000 | (u32::from(base.number())<<16)|src.bits())
}
fn splat_word(dst:Vreg,src:Gpr)->u32{
    0x19a0_c020 | (u32::from(src.number())<<16) | dst.bits()
}
fn mux_by_quad(dst:Vreg, predicate:u8, yes:Vreg, no:Vreg)->Result<u32,String>{
    if predicate>3 {return Err("HVX Q mux supports only Q0..Q3".into());}
    Ok(0x1ee0_e000 | (no.bits()<<16) | (yes.bits()<<8)
        | (u32::from(predicate)<<5) |dst.bits())
}
fn vector_or(dst:Vreg,left:Vreg,right:Vreg)->u32{
    0x1c20_c0c0 | (right.bits()<<16)|(left.bits()<<8)|dst.bits()
}
#[derive(Clone,Copy,Debug)] enum LaneWidth{Word,Halfword}
fn vector_shift_left(dst:Vreg,src:Vreg,amount:Gpr,width:LaneWidth)->u32{
    let code=match width{LaneWidth::Word=>0x1960_c0e0, LaneWidth::Halfword=>0x1980_c000};
    code | (u32::from(amount.number())<<16)|(src.bits()<<8)|dst.bits()
}
/// Hexagon HVX lane initialization `V16/17=zextract(R2)`; not emitted by
/// LLVM6, but Rizin validates both register fields in this exact family.
fn vector_zero_extract(dst:Vreg,src:Gpr)->Result<u32,String>{
    if !(16..=17).contains(&dst.0){return Err("tested zero-extract vector must be V16 or V17".into());}
    Ok(0x19a0_c120 | (u32::from(src.number())<<16)|dst.bits())
}
/// Independent LLVM6-assembled 13-bit unsigned assignment; old generic
/// helper deliberately accepts only 12 bits and is not relaxed globally.
fn assign_unsigned_13bit(dst:Gpr, value:u16)->Result<u32,String>{
    if value>8191 {return Err("13-bit Hexagon scalar immediate overflow".into());}
    Ok(0x7800_c000 | ((u32::from(value)>>9)<<16)
        |((u32::from(value)&0x1ff)<<5)|u32::from(dst.number()))
}
/// Hexagon `Rd += mpyi(Rs,Rt)` with three checked GPR operands. LLVM6
/// independently reproduces operand fields for R0/R13/R14.
fn multiply_accumulate_immediate(dst:Gpr,a:Gpr,b:Gpr)->u32{
    0xef00_c000|(u32::from(a.number())<<16)|(u32::from(b.number())<<8)|u32::from(dst.number())
}
/// Return a backward new-value predicate branch in the **same packet** as
/// `R13 = add(R13,#1)`, with the originally verified immediate `#4`.
fn until_four_iterations(packet_start:u32,target:u32,count:u8)->Result<u32,String>{
    if count!=4 || packet_start!=0x96c0_2984 || target>=packet_start
      || target&3!=0 || packet_start-target>0x1fc {
        return Err("HVX 4-iteration backward branch outside checked profile".into());
    }
    let displacement=((i64::from(target)-i64::from(packet_start))/2) as i32 as u32;
    Ok(0x2472_c400|(displacement&0xff))
}

fn put(out:&mut Vec<u8>,instr:u32,packet:P){out.extend_from_slice(&packet.apply(instr).to_le_bytes());}

/// Build the entire HVX vector-register setup routine, including the original
/// hardware-loop exit and scalar return instruction.
pub fn build_hvx_startup()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let v=|n|Vreg::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,H::ReadSystem{system:SystemRegister::Ssr,dst:r(12)?}.encode()?,P::Final);
    put(&mut out,H::SetBit{dst:r(12)?,src:r(12)?,position:31}.encode()?,P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(13)?,value:0}.encode()?,P::Final);
    put(&mut out,H::AddSmallImmediate{dst:r(14)?,src:r(13)?,value:4}.encode()?,P::Final);
    put(&mut out,H::InsertBitField{dst:r(12)?,src:r(14)?,width:3,offset:27}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:SystemRegister::Ssr,src:r(12)?}.encode()?,P::Final);
    let start=cdsp_boot::encode_extended_assignment(r(0)?,0xfe01_7100u32 as i32,false);
    put(&mut out,start[0],P::ContinuationA);
    put(&mut out,start[1],P::ContinuationA);
    put(&mut out,assign_unsigned_13bit(r(14)?,0x11f8)?,P::Final);
    put(&mut out,multiply_accumulate_immediate(r(0)?,r(13)?,r(14)?),P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(2)?,value:0}.encode()?,P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(0)?,1)?,P::Final);
    put(&mut out,splat_word(v(0)?,r(2)?),P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(1)?,1)?,P::Final);
    let mask=cdsp_boot::encode_extended_assignment(r(2)?,0x0101_0101,false);
    put(&mut out,mask[0],P::ContinuationA);
    put(&mut out,mask[1],P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(2)?,1)?,P::Final);
    put(&mut out,splat_word(v(1)?,r(2)?),P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(3)?,1)?,P::Final);
    put(&mut out,mux_by_quad(v(3)?,0,v(1)?,v(0)?)?,P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(4)?,1)?,P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(2)?,value:1}.encode()?,P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(5)?,1)?,P::Final);
    put(&mut out,vector_shift_left(v(1)?,v(1)?,r(2)?,LaneWidth::Word),P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(6)?,1)?,P::Final);
    put(&mut out,mux_by_quad(v(2)?,1,v(1)?,v(0)?)?,P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(7)?,1)?,P::Final);
    put(&mut out,vector_or(v(3)?,v(3)?,v(2)?),P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(8)?,1)?,P::Final);
    put(&mut out,vector_shift_left(v(1)?,v(1)?,r(2)?,LaneWidth::Halfword),P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(9)?,1)?,P::Final);
    put(&mut out,mux_by_quad(v(2)?,2,v(1)?,v(0)?)?,P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(10)?,1)?,P::Final);
    put(&mut out,vector_or(v(3)?,v(3)?,v(2)?),P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(11)?,1)?,P::Final);
    put(&mut out,vector_shift_left(v(1)?,v(1)?,r(2)?,LaneWidth::Halfword),P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(12)?,1)?,P::Final);
    put(&mut out,mux_by_quad(v(2)?,3,v(1)?,v(0)?)?,P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(13)?,1)?,P::Final);
    put(&mut out,vector_or(v(3)?,v(3)?,v(2)?),P::ContinuationA);
    for bank in 14..=27 {
        put(&mut out,store_vector(r(0)?,v(bank)?,1)?,P::Final);
    }
    put(&mut out,H::AssignSmallImmediate{dst:r(2)?,value:0}.encode()?,P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(28)?,1)?,P::Final);
    put(&mut out,vector_zero_extract(v(16)?,r(2)?)?,P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(29)?,1)?,P::Final);
    put(&mut out,H::AddUnsignedImmediate12{dst:r(2)?,src:r(2)?,value:128}.encode()?,P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(30)?,1)?,P::Final);
    put(&mut out,vector_zero_extract(v(17)?,r(2)?)?,P::ContinuationA);
    put(&mut out,store_vector(r(0)?,v(31)?,1)?,P::Final);
    for bank in [16,17] {put(&mut out,store_vector(r(0)?,v(bank)?,1)?,P::Final);}
    put(&mut out,store_vector_current(r(0)?,v(3)?,0)?,P::Final);
    put(&mut out,H::AddSmallImmediate{dst:r(13)?,src:r(13)?,value:1}.encode()?,P::ContinuationA);
    put(&mut out,until_four_iterations(0x96c0_2984,0x96c0_288c,4)?,P::Final);
    put(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err(format!("HVX initialization generated {} vs {} expected bytes",out.len(),FUNCTION_BYTES));}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    previous::verify_reference(original)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("HVX startup not adjacent to earlier CDSP codec classifier".into());}
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_hvx_startup()?.as_slice()){
        return Err("source-derived HVX startup differs from pinned CDSP MBN".into());
    }
    Ok(())
}

#[cfg(test)] mod tests{
    use super::*;
    #[test]fn hvx_register_fields_are_range_checked(){
        let r=|n|Gpr::new(n).unwrap();let v=|n|Vreg::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,272);
        assert_eq!(store_vector(r(0),v(7),1).unwrap(),0x2920_c107);
        assert_eq!(store_vector(r(5),v(7),1).unwrap(),0x2925_c107);
        assert!(store_vector(r(0),v(7),2).is_err());
        assert!(Vreg::new(32).is_err());
        assert_eq!(splat_word(v(1),r(2)),0x19a2_c021);
        assert_eq!(mux_by_quad(v(2),1,v(1),v(0)).unwrap(),0x1ee0_e122);
        assert!(mux_by_quad(v(2),4,v(1),v(0)).is_err());
        assert_eq!(vector_or(v(3),v(3),v(2)),0x1c22_c3c3);
        assert_eq!(vector_shift_left(v(1),v(1),r(2),LaneWidth::Word),0x1962_c1e1);
        assert_eq!(vector_shift_left(v(1),v(1),r(2),LaneWidth::Halfword),0x1982_c101);
        assert_eq!(vector_zero_extract(v(16),r(2)).unwrap(),0x19a2_c130);
        assert!(vector_zero_extract(v(15),r(2)).is_err());
        assert_eq!(assign_unsigned_13bit(r(14),0x11f8).unwrap(),0x7808_ff0e);
        assert!(assign_unsigned_13bit(r(14),8192).is_err());
        assert_eq!(multiply_accumulate_immediate(r(0),r(13),r(14)),0xef0d_ce00);
        assert_eq!(until_four_iterations(0x96c0_2984,0x96c0_288c,4).unwrap(),0x2472_c484);
        assert!(until_four_iterations(0x96c0_2984,0x96c0_288c,5).is_err());
        assert_eq!(build_hvx_startup().unwrap().len(),FUNCTION_BYTES);
    }
}
