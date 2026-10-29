// SPDX-License-Identifier: GPL-3.0-only
//! Two V66 controller-record diagnostic packets, followed by a separate NOP.
//! Original CDSP file executable [0x56adc,0x56afc), layout through 0x56b00.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_record_advance;

pub const FILE_OFFSET:usize=0x56adc;
pub const END_FILE_OFFSET:usize=0x56afc;
pub const END_LAYOUT_OFFSET:usize=0x56b00;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const NEW_ALIGNMENT_BYTES:usize=END_LAYOUT_OFFSET-END_FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8adc;
pub const RWE_START:usize=cdsp_late_controller_record_advance::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_LAYOUT_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_record_advance::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_record_advance::RWE_PREFIX_LAYOUT_LEN+NEW_ALIGNMENT_BYTES;
const TRACE_SERVICE:u32=0x96cc_d8c4;
const CONTROLLER_DIAGNOSTIC:u32=0xc022_6175;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes())}

/// Packet-relative direct call, original controller error sites only.
pub fn encode_diagnostic_call(site:u32,target:u32)->Result<u32,String>{
    if ![0x96cc_8adc,0x96cc_8aec].contains(&site)||target!=TRACE_SERVICE{
        return Err("unverified CDSP record diagnostic call or destination".into());
    }
    let diff=target.checked_sub(site).ok_or("diagnostic target precedes source")?;
    if diff&1!=0||diff>0x1fff_fe{return Err("invalid diagnostic call distance".into());}
    Ok(M::ContinuationA.apply(0x5a00_0000|(((diff>>15)&0x3f)<<16)|((diff>>1)&0x3fff)))
}
/// V66 immediate error codes 365 and 366 share the same unsigned 12-bit form.
pub fn encode_error_number(dst:G,value:u16)->Result<u32,String>{
    if dst.number()!=1||![365,366].contains(&value){return Err("unknown error code/address register".into());}
    Ok(M::ContinuationA.apply(S::AssignUnsignedImmediate{dst,value}.encode()?))
}
/// Extended R0 pointer to diagnostic string, split into immext and consumer.
pub fn encode_diagnostic_message(dst:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=0||address!=CONTROLLER_DIAGNOSTIC{return Err("unexpected diagnostic pointer".into());}
    let [ext,consumer]=cdsp_boot::encode_extended_assignment(dst,address as i32,false);
    Ok([M::ContinuationA.apply(ext),M::Final.apply(consumer)])
}
fn emit_one(out:&mut Vec<u8>,site:u32,code:u16)->Result<(),String>{
    if ![(0x96cc8adc,365),(0x96cc8aec,366)].contains(&(site,code)){
        return Err("unverified controller diagnostic packet".into());
    }
    emit(out,encode_diagnostic_call(site,TRACE_SERVICE)?);
    emit(out,encode_error_number(r(1)?,code)?);
    for v in encode_diagnostic_message(r(0)?,CONTROLLER_DIAGNOSTIC)?{emit(out,v);}
    Ok(())
}
pub fn build_record_diagnostics()->Result<Vec<u8>,String>{
    let mut b=Vec::with_capacity(EXECUTABLE_LEN);
    emit_one(&mut b,PHYSICAL_ADDRESS,365)?;
    emit_one(&mut b,PHYSICAL_ADDRESS+16,366)?;
    if b.len()!=EXECUTABLE_LEN{return Err("two controller diagnostics must contain eight instruction words".into());}
    Ok(b)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut b=cdsp_late_controller_record_advance::build_prefix()?;
    b.extend(build_record_diagnostics()?);
    for _ in 0..NEW_ALIGNMENT_BYTES/4{emit(&mut b,S::Nop.encode()?);}
    if b.len()!=FULL_RWE_LAYOUT_LEN || b.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("CDSP record diagnostics layout/source accounting differs".into());
    }
    Ok(b)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_record_diagnostics()?.as_slice()){
        return Err("CDSP record diagnostic instructions differ from original".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_LAYOUT_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("CDSP controller diagnostic RWE prefix differs".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn two_diagnostic_packets_have_original_source_fields(){
        let b=build_record_diagnostics().unwrap();assert_eq!(b.len(),32);
        let words:Vec<u32>=b.chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect();
        assert_eq!(words,[0x5a00_66f4,0x7800_6da1,0x0c02_4985,0x7800_c6a0,
                          0x5a00_66ec,0x7800_6dc1,0x0c02_4985,0x7800_c6a0]);
        assert!(encode_diagnostic_call(0x96cc8adc,0x96ccd8c8).is_err());
        assert!(encode_error_number(r(1).unwrap(),364).is_err());
        assert!(encode_diagnostic_message(r(0).unwrap(),0xc0226179).is_err());
    }
    #[test]fn one_trailing_nop_is_layout_not_executable(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,2816);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2756);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,60);
        assert_eq!(build_prefix().unwrap().len(),2816);
        assert_eq!(&build_prefix().unwrap()[2812..], &[0x00,0xc0,0x00,0x7f]);
    }
}
