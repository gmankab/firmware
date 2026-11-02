// SPDX-License-Identifier: GPL-3.0-only
//! Checked Hexagon V66 command dispatch, supervisor snapshots and one-word
//! fallback stubs. These form three small adjacent functions after the SIAD
//! callback dispatcher. All words are synthesized from typed operands.
use super::cdsp_boot::{GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_supervisor_dispatch as previous;

pub const FILE_OFFSET:usize=0x5aa8;
pub const REGION_END:usize=0x5afc;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2aa8;
fn emit(out:&mut Vec<u8>,word:u32,parse:P){out.extend_from_slice(&parse.apply(word).to_le_bytes());}

/// Return -1 in R0 using the original one-word Hexagon duplex return.
fn return_error_minus_one(dst:Gpr,target:Gpr)->Result<u32,String>{
    if dst.number()!=0 || target.number()!=31 {
        return Err("V66 compact -1 return must use R0 and LR".into());
    }
    Ok(0x5a00_3fc0)
}
/// Compound P0=(R5>#n); if(P0.new) jump:nt, relative to packet start.
fn compare_gtu_and_jump(packet:u32,target:u32,src:Gpr,limit:u8)->Result<u32,String>{
    if src.number()!=5||limit>31||packet%4!=0||target%4!=0||target<=packet||target-packet>0x1fc {
        return Err("Hexagon checked P0 greater-than forward branch bounds".into());
    }
    Ok(0x1100_c000 | (u32::from(src.number())<<16) | (u32::from(limit)<<8)
       |((target-packet)/2))
}
/// An aligned 32-bit memory read using bounded non-extender byte offset.
fn load_word(dst:Gpr,base:Gpr,offset:u16)->Result<u32,String>{
    if offset>1020||offset%4!=0 {return Err("word load offset unaligned or too large".into());}
    Ok(0x9180_c000 | (u32::from(base.number())<<16)
       | (u32::from(offset)<<3) | u32::from(dst.number()))
}
/// Producer+consumer packet: R7=memw(...); if(!cmp.eq(R7.new,#4)) jump:nt.
fn skip_unexpected_word(packet:u32,target:u32,register:Gpr,expected:u8)->Result<u32,String>{
    if register.number()!=7||expected!=4||packet%4!=0||target%4!=0||target<=packet||target-packet>0xfe {
        return Err("V66 checked new-value compare/memw branch bounds".into());
    }
    Ok(0x2442_c000 |(u32::from(expected)<<8)|((target-packet)/2))
}
fn long_jump(packet:u32,target:u32)->Result<[u32;2],String>{
    if packet%4!=0||target%4!=0||target<=packet {
        return Err("V66 direct jump cannot use unaligned/backward target here".into());
    }
    let distance=target-packet;
    if distance>0x1f_ffff{return Err("V66 jump exceeds tested extended range".into());}
    let extender=((distance>>20)<<16)|((distance>>6)&0x3fff)|0x4000;
    let consumer=0x5800_c000|((distance&0x3f)<<1);
    Ok([extender,consumer])
}
fn compare_eq_and_jump(packet:u32,target:u32,src:Gpr,compare:u8)->Result<u32,String>{
    if src.number()!=5||compare>31||packet%4!=0||target%4!=0||target<=packet||target-packet>0x1fc {
        return Err("V66 P0.eq immediate branch outside checked range".into());
    }
    Ok(0x1000_c000|(u32::from(src.number())<<16)
      |(u32::from(compare)<<8)|((target-packet)/2))
}
fn indexed_table_word(dst:Gpr,src:Gpr,address:u32)->Result<[u32;2],String>{
    if address%4!=0 {return Err("indexed callback address must be word-aligned".into());}
    let ext=((address>>20)<<16)|((address>>6)&0x3fff)|0x4000;
    let body=0x9d80_f000|(u32::from(src.number())<<16)
          |(((address&0x3f)>>2)<<8)|u32::from(dst.number());
    Ok([ext,body])
}
fn register_call(src:Gpr)->u32{0x50a0_c000 |(u32::from(src.number())<<16)}
fn read_sgp0(dst:Gpr)->u32{0x6e80_c000|u32::from(dst.number())}
fn load_double_extended(low:Gpr,base:Gpr,address_offset:u16)->Result<[u32;2],String>{
    if low.number()%2!=0||low.number()>30||address_offset%8!=0{
        return Err("64-bit read needs even destination pair and 8-byte aligned offset".into());
    }
    let address=u32::from(address_offset);
    let ext=((address>>20)<<16)|((address>>6)&0x3fff)|0x4000;
    let body=0x91c0_c000 |(u32::from(base.number())<<16)
           |((address&0x3f)<<5)|u32::from(low.number());
    Ok([ext,body])
}

pub fn build_dispatch_helpers()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut result=Vec::with_capacity(FUNCTION_BYTES);
    emit(&mut result,return_error_minus_one(r(0)?,r(31)?)?,P::ContinuationB);
    emit(&mut result,compare_gtu_and_jump(0x96c0_2aac,0x96c0_2af8,r(5)?,5)?,P::ContinuationA);
    emit(&mut result,load_word(r(7)?,r(6)?,256)?,P::Final);
    emit(&mut result,load_word(r(7)?,r(7)?,16)?,P::ContinuationA);
    emit(&mut result,skip_unexpected_word(0x96c0_2ab4,0x96c0_2ac4,r(7)?,4)?,P::Final);
    let jump=long_jump(0x96c0_2abc,0x96c0_f1a0)?;
    emit(&mut result,jump[0],P::ContinuationA);
    emit(&mut result,jump[1],P::Final);
    emit(&mut result,compare_eq_and_jump(0x96c0_2ac4,0x96c0_2ad8,r(5)?,2)?,P::ContinuationA);
    let indexed=indexed_table_word(r(7)?,r(5)?,0xfe02_4d60)?;
    emit(&mut result,indexed[0],P::ContinuationA);emit(&mut result,indexed[1],P::Final);
    emit(&mut result,H::JumpRegister{source:r(7)?}.encode()?,P::Final);
    emit(&mut result,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    emit(&mut result,0xa09d_c000,P::Final); // allocframe(SP,#0), narrow verified form.
    emit(&mut result,register_call(r(7)?),P::Final);
    emit(&mut result,read_sgp0(r(6)?),P::Final);
    emit(&mut result,load_word(r(6)?,r(6)?,0)?,P::Final);
    let snapshot=load_double_extended(r(2)?,r(6)?,136)?;
    emit(&mut result,snapshot[0],P::ContinuationA);emit(&mut result,snapshot[1],P::Final);
    emit(&mut result,0x901e_c01e,P::Final); // deallocframe, checked profile.
    emit(&mut result,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    emit(&mut result,return_error_minus_one(r(0)?,r(31)?)?,P::ContinuationB);
    if result.len()!=FUNCTION_BYTES{return Err("Hexagon dispatch helper stream length differs".into());}
    Ok(result)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    previous::verify_reference(reference)?;
    if previous::REGION_END!=FILE_OFFSET ||
       reference.get(FILE_OFFSET..REGION_END)!=Some(build_dispatch_helpers()?.as_slice()){
        return Err("CDSP dispatch helper stream differs from pinned reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn dispatch_helpers_use_bounded_typed_fields(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,84);
        assert_eq!(return_error_minus_one(r(0),r(31)).unwrap(),0x5a00_3fc0);
        assert_eq!(compare_gtu_and_jump(0x96c0_2aac,0x96c0_2af8,r(5),5).unwrap(),0x1105_c526);
        assert_eq!(skip_unexpected_word(0x96c0_2ab4,0x96c0_2ac4,r(7),4).unwrap(),0x2442_c408);
        assert_eq!(long_jump(0x96c0_2abc,0x96c0_f1a0).unwrap(),[0x431b,0x5800_c048]);
        assert_eq!(indexed_table_word(r(7),r(5),0xfe02_4d60).unwrap(),[0x0fe0_4935,0x9d85_f807]);
        assert_eq!(load_double_extended(r(2),r(6),136).unwrap(),[0x4002,0x91c6_c102]);
        assert!(load_word(r(7),r(6),1023).is_err());
        assert!(load_double_extended(r(3),r(6),136).is_err());
        assert!(compare_eq_and_jump(0x96c0_2ac4,0x96c0_2ad8,r(6),2).is_err());
        assert_eq!(build_dispatch_helpers().unwrap().len(),FUNCTION_BYTES);
    }
}
