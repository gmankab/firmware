// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 controller state transition: initialize the hardware state word,
//! flush its cache line and reset SYSCFG/SSR before the supervisor callbacks.
//! Encoded from register operands/packet markers; this is a partial routine.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H,SystemRegister};
use super::cdsp_early_controllers;
use super::cdsp_early_worker_runtime;
use super::cdsp_early_supervisor_vectors;

pub const FILE_OFFSET:usize=0x4c64;
pub const REGION_END:usize=0x4cb4;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1c64;
fn put(out:&mut Vec<u8>,op:u32,m:P){out.extend_from_slice(&m.apply(op).to_le_bytes());}

/// Store a packet new-value register word at a 32-bit absolute address,
/// while assigning the base register in the same packet. Verified for the
/// 32-bit `memw(R1=##addr)=R0.new` form, no arbitrary register reordering.
fn new_value_word_store_absolute(base:Gpr,producer:Gpr,absolute:u32)->Result<[u32;2],String>{
    if base.number()>7 || producer.number()!=0 || absolute%4!=0 {
        return Err("CDSP absolute .new store requires low base register, R0 producer and aligned address".into());
    }
    let ext=((absolute>>20)<<16)|((absolute>>6)&0x3fff)|0x4000;
    let body=0xaba0_d280|(u32::from(base.number())<<16)|(absolute&0x3f);
    Ok([ext,body])
}
fn clean_cache_address(address:Gpr)->u32{0xa000_c000|(u32::from(address.number())<<16)}
fn read_ssr(dst:Gpr)->u32{0x6e80_c000|(6<<16)|u32::from(dst.number())}

pub fn build_worker_state_prefix()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    // The small immediate and absolute write form a coupled instruction
    // packet. The producer and the consumer's address use different fields.
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:2}.encode()?,P::ContinuationA);
    let initial=new_value_word_store_absolute(r(1)?,r(0)?,0xfe02_4d00)?;
    put(&mut out,initial[0],P::ContinuationA);
    put(&mut out,initial[1],P::Final);
    put(&mut out,clean_cache_address(r(1)?),P::Final);
    put(&mut out,cdsp_early_supervisor_vectors::short_call(0x96c0_1c74,0x96c0_2418)?,P::Final);
    let callback=cdsp_early_controllers::encode_extended_call(0x96c0_1c78,0x96c0_c004)?;
    put(&mut out,callback[0],P::ContinuationA);
    put(&mut out,callback[1],P::Final);
    put(&mut out,cdsp_early_supervisor_vectors::short_call(0x96c0_1c80,0x96c0_2044)?,P::Final);
    put(&mut out,H::ReadSyscfg{dst:r(0)?}.encode()?,P::Final);
    put(&mut out,H::ClearBit{dst:r(0)?,src:r(0)?,position:4}.encode()?,P::ContinuationA);
    put(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:0}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:SystemRegister::Syscfg,src:r(0)?}.encode()?,P::Final);
    put(&mut out,read_ssr(r(0)?),P::Final);
    put(&mut out,H::InsertBitField{dst:r(0)?,src:r(1)?,width:3,offset:16}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:SystemRegister::Ssr,src:r(0)?}.encode()?,P::Final);
    // Forced immediate extension even for small literal 36 is intentional:
    // the command packet has three operations and two different extenders.
    let set_result=cdsp_boot::encode_extended_assignment(r(0)?,36,false);
    put(&mut out,set_result[0],P::ContinuationA);
    put(&mut out,set_result[1],P::ContinuationA);
    let later=new_value_word_store_absolute(r(1)?,r(0)?,0xfe00_0044)?;
    put(&mut out,later[0],P::ContinuationA);
    put(&mut out,later[1],P::Final);
    put(&mut out,clean_cache_address(r(1)?),P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP state update has wrong instruction length".into());}
    Ok(out)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_worker_runtime::verify_reference(reference)?;
    if cdsp_early_worker_runtime::REGION_END!=FILE_OFFSET{return Err("worker state has unimplemented gap".into());}
    let rebuilt=build_worker_state_prefix()?;
    if reference.get(FILE_OFFSET..REGION_END)!=Some(rebuilt.as_slice()){
        return Err("CDSP worker state differs from pinned OEM reference".into());
    }
    Ok(())
}
pub fn build_supervisor_with_worker_state()->Result<Vec<u8>,String>{
    let mut output=cdsp_early_worker_runtime::build_supervisor_with_worker_runtime()?;
    output.extend(build_worker_state_prefix()?);
    Ok(output)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn worker_state_new_value_packets_are_checked(){
        assert_eq!(FUNCTION_BYTES,80);
        assert_eq!(build_worker_state_prefix().unwrap().len(),80);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
        assert_eq!(new_value_word_store_absolute(Gpr::new(1).unwrap(),Gpr::new(0).unwrap(),0xfe024d00).unwrap(),[0x0fe0_4934,0xaba1_d280]);
        assert_eq!(new_value_word_store_absolute(Gpr::new(1).unwrap(),Gpr::new(0).unwrap(),0xfe000044).unwrap(),[0x0fe0_4001,0xaba1_d284]);
        assert!(new_value_word_store_absolute(Gpr::new(1).unwrap(),Gpr::new(0).unwrap(),0xfe000045).is_err());
        assert!(new_value_word_store_absolute(Gpr::new(8).unwrap(),Gpr::new(0).unwrap(),0xfe000044).is_err());
        assert!(new_value_word_store_absolute(Gpr::new(1).unwrap(),Gpr::new(2).unwrap(),0xfe000044).is_err());
    }
}
