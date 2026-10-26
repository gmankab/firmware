// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 runtime address-range validator, file 0x56650..0x56698.
//!
//! Original packet positions and physical branch targets are kept explicit.
//! This only covers a partial larger function continuing beyond 0x56698.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_large_prologue;

pub const FILE_OFFSET:usize=0x56650;
pub const END_FILE_OFFSET:usize=0x56698;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc8650;
pub const RWE_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_large_prologue::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_large_prologue::RWE_LAYOUT_NOP_LEN;

fn r(i:u8)->Result<G,String>{G::new(i)}
fn emit(o:&mut Vec<u8>,word:u32){o.extend_from_slice(&word.to_le_bytes());}

/// A single-word relative call to `0x96cd0480`, with checked fields; LLVM6
/// emits this same packet when source call displacement is `0x7e30` bytes.
pub fn encode_controller_copy_service_call(packet:u32,target:u32)->Result<u32,String>{
    if packet!=PHYSICAL_ADDRESS||target!=0x96cd0480{
        return Err("unvalidated controller copy service target".into());
    }
    let delta=target.checked_sub(packet).ok_or("controller call target below caller")?;
    if delta&1!=0||delta>0x1fff_fe{
        return Err("controller call immediate out of range".into());
    }
    let word=0x5a00_c000 | (((delta>>15)&0x3f)<<16) | ((delta>>1)&0x3fff);
    Ok(M::ContinuationA.apply(word))
}

/// Duplex constant zero/-1 pair from independent Hexagon assembler.
pub fn encode_zero_negative_pair(low:G,zero:u8,negative_one:i8)->Result<u32,String>{
    if low.number()!=2||zero!=0||negative_one!=-1{
        return Err("controller combine requires R3:2=combine(0,-1)".into());
    }
    Ok(0x7c7f_6002)
}

/// Duplex `R1=R17 ; R0=memw(R3+#16)` preserving controller linkage.
pub fn encode_controller_handle_pair(output:G,source:G,load:G,base:G,offset:u8)->Result<u32,String>{
    if output.number()!=1||source.number()!=17||load.number()!=0||base.number()!=3||offset!=16{
        return Err("controller handle duplex requires R1=R17 and R0=memw(R3+16)".into());
    }
    Ok(0x5091_0430)
}

/// `.new` null check at physical 0x96cc865c, target 0x96cc88bc.
pub fn encode_missing_handle_branch(src:G,packet:u32,target:u32)->Result<u32,String>{
    if src.number()!=16||packet!=PHYSICAL_ADDRESS+0xc||target!=0x96cc88bc{
        return Err("controller handle null check uses R16.new and original return target".into());
    }
    let dist=target.checked_sub(packet).ok_or("null handle backwards branch")?;
    if dist&1!=0||dist>0x3fe{return Err("out-of-range null handle branch".into());}
    let h=dist>>1;
    Ok(0x2402_c000 | (h&0xff) | ((h&0x100)<<12))
}

/// `R25=add(R24,#0x4000)` from unchanged LLVM6 construction.
pub fn encode_upper_bank_boundary(dst:G,src:G,increment:u32)->Result<u32,String>{
    if dst.number()!=25||src.number()!=24||increment!=0x4000{
        return Err("CDSP controller upper bank requires R25=R24+0x4000".into());
    }
    Ok(0xb418_c019)
}

/// `R23=memw(R16+#0)` original packet, independently assembled by LLVM6.
pub fn encode_controller_stream_pointer(dst:G,base:G,offset:u8)->Result<u32,String>{
    if dst.number()!=23||base.number()!=16||offset!=0{
        return Err("controller stream pointer requires R23=memw(R16+0)".into());
    }
    Ok(0x9190_c017)
}

/// Unsigned predicate `P0=cmp.gtu(Rleft,Rright)`, confirmed by LLVM6 and
/// the Rizin backend for the three registers in this controller path.
pub fn encode_greater_unsigned(lhs:G,rhs:G)->Result<u32,String>{
    if ![(23,24),(23,25)].contains(&(lhs.number(),rhs.number())){
        return Err("unsupported controller-bank compare registers".into());
    }
    Ok(0xf260_c000 | (u32::from(lhs.number())<<16) |
       (u32::from(rhs.number())<<8))
}

/// Negated unsigned predicate `P0=!cmp.gtu(Rleft,Rright)`.
pub fn encode_not_greater_unsigned(lhs:G,rhs:G)->Result<u32,String>{
    if ![(24,2),(25,2)].contains(&(lhs.number(),rhs.number())){
        return Err("unsupported negated bank comparison".into());
    }
    Ok(0xf260_c010 | (u32::from(lhs.number())<<16) |
       (u32::from(rhs.number())<<8))
}

/// P0.new taken branch, with the original packet-relative branch bit.
pub fn encode_new_p0_taken(packet:u32,target:u32)->Result<u32,String>{
    if packet!=PHYSICAL_ADDRESS+0x30||target!=0x96cc8698{
        return Err("controller primary bank target differs".into());
    }
    let delta=target.checked_sub(packet).ok_or("P0 taken target behind source")?;
    if delta==0||delta>0x1fe||delta&1!=0{
        return Err("P0 taken branch outside original width".into());
    }
    Ok(0x5c00_d800|(delta>>1))
}

/// Negated-new predicate branch predicted not taken.
pub fn encode_if_not_new_p0_nt(packet:u32,target:u32)->Result<u32,String>{
    if packet!=PHYSICAL_ADDRESS+0x40||target!=0x96cc86c0{
        return Err("unexpected false-bank branch physical target".into());
    }
    let d=target.checked_sub(packet).ok_or("P0 false branch backwards")?;
    if d==0||d>0x1fe||d&1!=0{
        return Err("P0 false branch outside checked width".into());
    }
    Ok(0x5c20_c800|(d>>1))
}

pub fn build_validator_prefix()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_controller_copy_service_call(PHYSICAL_ADDRESS,0x96cd0480)?);
    emit(&mut out,M::ContinuationA.apply(encode_zero_negative_pair(r(2)?,0,-1)?));
    emit(&mut out,encode_controller_handle_pair(r(1)?,r(17)?,r(0)?,r(3)?,16)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignUnsignedImmediate{dst:r(18)?,value:174}.encode()?));
    emit(&mut out,M::ContinuationA.apply(S::AssignRegister{dst:r(16)?,src:r(0)?}.encode()?));
    emit(&mut out,encode_missing_handle_branch(r(16)?,PHYSICAL_ADDRESS+0xc,0x96cc88bc)?);
    emit(&mut out,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(22)?,src:r(22)?,amount:12}.encode()?));
    emit(&mut out,encode_upper_bank_boundary(r(25)?,r(24)?,0x4000)?);
    emit(&mut out,encode_controller_stream_pointer(r(23)?,r(16)?,0)?);
    emit(&mut out,M::ContinuationA.apply(encode_greater_unsigned(r(23)?,r(24)?)?));
    emit(&mut out,M::ContinuationA.apply(S::AddRegisters{dst:r(2)?,left:r(23)?,right:r(17)?}.encode()?));
    emit(&mut out,S::JumpIfNewP0NotTaken{displacement_bytes:0x14}.encode()?);
    emit(&mut out,M::ContinuationA.apply(encode_not_greater_unsigned(r(24)?,r(2)?)?));
    emit(&mut out,encode_new_p0_taken(PHYSICAL_ADDRESS+0x30,0x96cc8698)?);
    emit(&mut out,M::ContinuationA.apply(encode_greater_unsigned(r(23)?,r(25)?)?));
    emit(&mut out,S::JumpIfNewP0NotTaken{displacement_bytes:0x38}.encode()?);
    emit(&mut out,M::ContinuationA.apply(encode_not_greater_unsigned(r(25)?,r(2)?)?));
    emit(&mut out,encode_if_not_new_p0_nt(PHYSICAL_ADDRESS+0x40,0x96cc86c0)?);
    if out.len()!=EXECUTABLE_LEN{return Err("controller range check needs 72 bytes".into());}
    Ok(out)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_large_prologue::build_prefix()?;
    out.extend(build_validator_prefix()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN{return Err("controller range-check RWE prefix mismatch".into());}
    Ok(out)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_validator_prefix()?.as_slice()){
        return Err("range-check 72 bytes differ from original CDSP".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("controller range-check prefix differs from original CDSP".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_range_checks_reject_unsupported_operands(){
        assert_eq!(build_validator_prefix().unwrap().len(),0x48);
        assert_eq!(encode_controller_copy_service_call(PHYSICAL_ADDRESS,0x96cd0480).unwrap(),0x5a007f18);
        assert_eq!(encode_zero_negative_pair(r(2).unwrap(),0,-1).unwrap(),0x7c7f6002);
        assert_eq!(encode_controller_handle_pair(r(1).unwrap(),r(17).unwrap(),r(0).unwrap(),r(3).unwrap(),16).unwrap(),0x50910430);
        assert_eq!(encode_missing_handle_branch(r(16).unwrap(),PHYSICAL_ADDRESS+0xc,0x96cc88bc).unwrap(),0x2412c030);
        assert!(encode_upper_bank_boundary(r(25).unwrap(),r(24).unwrap(),0x1000).is_err());
        assert_eq!(M::ContinuationA.apply(encode_greater_unsigned(r(23).unwrap(),r(24).unwrap()).unwrap()),0xf277_5800);
        assert_eq!(M::ContinuationA.apply(encode_greater_unsigned(r(23).unwrap(),r(25).unwrap()).unwrap()),0xf277_5900);
        assert_eq!(M::ContinuationA.apply(encode_not_greater_unsigned(r(24).unwrap(),r(2).unwrap()).unwrap()),0xf278_4210);
        assert_eq!(M::ContinuationA.apply(encode_not_greater_unsigned(r(25).unwrap(),r(2).unwrap()).unwrap()),0xf279_4210);
        assert_eq!(encode_new_p0_taken(PHYSICAL_ADDRESS+0x30,0x96cc8698).unwrap(),0x5c00_d80c);
        assert!(encode_greater_unsigned(r(21).unwrap(),r(25).unwrap()).is_err());
        assert!(encode_new_p0_taken(PHYSICAL_ADDRESS+0x30,0x96cc869c).is_err());
    }
    #[test]fn prefix_separates_code_from_structural_fill(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,0x698);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1644);
        assert_eq!(RWE_LAYOUT_NOP_LEN,44);
        assert_eq!(build_prefix().unwrap().len(),0x698);
    }
}
