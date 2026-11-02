// SPDX-License-Identifier: GPL-3.0-only
//! Finish the CDSP supervisor reset ladder and SYSCFG handoff. The short
//! source-generated region precedes a separate context-capture function at
//! file 0x495c. The remote initialization call uses signed packet-relative
//! Hexagon IMMEXT/CALL fields, not a raw copied opcode.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H,SystemRegister};
use super::cdsp_early_controllers;
use super::cdsp_early_exception_wait;
use super::cdsp_early_supervisor_vectors;

pub const FILE_OFFSET:usize=0x48e8;
pub const REGION_END:usize=0x495c;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_18e8;
fn put(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}
fn store_word(src:Gpr,base:Gpr)->u32{
    0xa180_c000 |(u32::from(base.number())<<16)|(u32::from(src.number())<<8)
}
fn read_special_s54(dst:Gpr)->u32{0x6e80_c000|(54<<16)|u32::from(dst.number())}
/// Packet-local branch on a just-produced zero. Its immediate is a positive
/// halfword count from the packet's *first* instruction, unlike the ordinary
/// new-register nonzero loop in the previous routine.
fn skip_if_new_r0_zero(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start%4!=0||target%4!=0||target<=packet_start {
        return Err("Hexagon new-value zero branch requires increasing aligned addresses".into());
    }
    let delta=target-packet_start;
    if delta>0x1fe||delta%2!=0{return Err("CDSP supervisor forward branch out of range".into());}
    Ok(0x2402_c000|(delta/2))
}

pub fn build_exception_finalize()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    for (immediate,callback) in [(5,0x96c0_1a08),(6,0x96c0_1a84)] {
        let start=PHYSICAL_ADDRESS+out.len() as u32;
        let pointer=cdsp_boot::encode_extended_add(r(0)?,r(23)?,-0x1fda71c,false);
        put(&mut out,pointer[0],P::ContinuationA);
        put(&mut out,pointer[1],P::ContinuationA);
        put(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:immediate}.encode()?,P::Final);
        put(&mut out,store_word(r(1)?,r(0)?),P::Final);
        put(&mut out,cdsp_early_supervisor_vectors::short_call(start+16,callback)?,P::Final);
    }
    put(&mut out,read_special_s54(r(23)?),P::Final);
    let seven=cdsp_boot::encode_extended_add(r(0)?,r(23)?,-0x1fda71c,false);
    put(&mut out,seven[0],P::ContinuationA);
    put(&mut out,seven[1],P::ContinuationA);
    put(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:7}.encode()?,P::Final);
    put(&mut out,store_word(r(1)?,r(0)?),P::Final);
    // OEM includes an explicit zero IMMEXT and a new-value conditional
    // branch; normalizing it to an unconditional jump would change bytes.
    let initial=cdsp_boot::encode_extended_assignment(r(0)?,0,false);
    put(&mut out,initial[0],P::ContinuationA);
    put(&mut out,initial[1],P::ContinuationA);
    put(&mut out,skip_if_new_r0_zero(0x96c0_1924,0x96c0_1ad4)?,P::Final);
    put(&mut out,H::ReadSyscfg{dst:r(0)?}.encode()?,P::Final);
    let status=cdsp_boot::encode_extended_word_load(r(1)?,r(23)?,-0x1fd9334)?;
    put(&mut out,status[0],P::ContinuationA);
    put(&mut out,status[1],P::Final);
    put(&mut out,H::InsertBitField{dst:r(0)?,src:r(1)?,width:3,offset:16}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:SystemRegister::Syscfg,src:r(0)?}.encode()?,P::Final);
    put(&mut out,H::Isync.encode()?,P::Final);
    let code=cdsp_boot::encode_extended_assignment(r(0)?,128,true);
    put(&mut out,code[0],P::ContinuationA);
    put(&mut out,code[1],P::Final);
    let async_call=cdsp_early_controllers::encode_extended_call(0x96c0_1950,0x98c0_0000)?;
    put(&mut out,async_call[0],P::ContinuationA);
    put(&mut out,async_call[1],P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0x17c}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP exception finalize emitted incorrect byte length".into());}
    Ok(out)
}

pub fn build_supervisor_with_finalize()->Result<Vec<u8>,String>{
    if cdsp_early_exception_wait::REGION_END!=FILE_OFFSET{return Err("CDSP exception finalization has uncovered predecessor bytes".into());}
    let mut previous=cdsp_early_exception_wait::build_supervisor_with_wait()?;
    previous.extend(build_exception_finalize()?);
    Ok(previous)
}
pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_exception_wait::verify_reference(original)?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_exception_finalize()?.as_slice()){
        return Err("CDSP exception finalize differs from original pinned firmware".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn finalize_preserves_original_packet_displacements(){
        assert_eq!(FUNCTION_BYTES,116);
        assert_eq!(build_exception_finalize().unwrap().len(),116);
        assert_eq!(skip_if_new_r0_zero(0x96c0_1924,0x96c0_1ad4).unwrap(),0x2402_c0d8);
        assert!(skip_if_new_r0_zero(0x96c0_1924,0x96c0_1924).is_err());
        assert!(skip_if_new_r0_zero(0x96c0_1924,0x96c0_1f24).is_err());
        assert_eq!(cdsp_early_controllers::encode_extended_call(0x96c0_1950,0x98c0_0000).unwrap()[1]&!0xc000,0x5a00_0060);
    }
}
