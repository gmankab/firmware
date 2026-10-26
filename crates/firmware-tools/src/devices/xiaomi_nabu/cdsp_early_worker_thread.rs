// SPDX-License-Identifier: GPL-3.0-only
//! CDSP worker-thread initialization after the worker idle/controller gate.
//! Selects hardware thread stack, propagates predicate state and publishes
//! MODECTL transition to the controller. All V66 words are assembled from
//! checked register/immediate operands, not copied from vendor instruction arrays.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::{cdsp_early_worker_idle,cdsp_early_controllers};

pub const FILE_OFFSET:usize=0x4d24;
pub const REGION_END:usize=0x4d78;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1d24;
fn put(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}

/// New-value compare-and-skip for the following GP-extended word load.
/// The 9-bit halfword displacement is split: bit 8 maps to opcode bit 20;
/// remaining bits occupy the low immediate field. A packet-relative target
/// and R0.new producer are required (the producer is the second packet word).
fn gp_state_not_one(packet:u32,target:u32,compare_value:u8)->Result<u32,String>{
    if packet%4!=0||target%4!=0||target<=packet {return Err("CDSP worker GP branch requires aligned forward packet target".into());}
    let distance=target-packet;
    if distance%2!=0||distance>0x3fe || compare_value!=1 {return Err("CDSP worker GP compare/branch unsupported immediate or displacement".into());}
    let half=distance/2;
    Ok(0x2402_c000 | ((half&0x100)<<12) | (half&0x1ff))
}
/// `Rn=HTID`, V66 supervisor special register selector 8. Rizin supports
/// the privileged read; LLVM6 rejects the special-register name.
fn read_hardware_thread(dst:Gpr)->u32{0x6e88_c000|u32::from(dst.number())}
/// `Rdst=P3:0`, generalized predicate-group selector from architecture docs
/// and LLVM6 independent assembly. Only the observed destination is used.
fn read_predicate_group(dst:Gpr)->u32{0x6a04_c000|u32::from(dst.number())}
/// `SP=addasl(SP,Rthread,#5)` with checked thread register and stack shift.
fn thread_stack(thread:Gpr,scale:u8)->Result<u32,String>{
    if scale>7{return Err("worker thread stack scale out of confirmed range".into());}
    Ok(0xc400_c000|(u32::from(thread.number())<<16)| (u32::from(scale)<<5)
      |(u32::from(Gpr::new(29)?.number())<<8)|u32::from(Gpr::new(29)?.number()))
}
/// `memw(Rbase=##absolute)=R0.new` when the producer appears immediately
/// before its two-word extended address store in the same Hexagon packet.
fn publish_state(base:Gpr,producer:Gpr,address:u32)->Result<[u32;2],String>{
    if base.number()>7||producer.number()!=0||address%4!=0{return Err("worker controller new-value store has invalid base, producer or alignment".into());}
    Ok([((address>>20)<<16)|((address>>6)&0x3fff)|0x4000,
        0xaba0_d280|(u32::from(base.number())<<16)|(address&0x3f)])
}
/// Worker gate and per-thread stack state. This is a bounded prefix of a
/// larger worker/controller routine; no fake return is appended.
pub fn build_worker_thread_start()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    let read=cdsp_boot::encode_extended_gp_word_load(r(0)?,0xfe02_4d00);
    put(&mut out,read[0],P::ContinuationA);
    put(&mut out,read[1],P::ContinuationA);
    put(&mut out,gp_state_not_one(PHYSICAL_ADDRESS,0x96c0_1fd0,1)?,P::Final);
    let value=cdsp_boot::encode_extended_assignment(r(0)?,27,false);
    put(&mut out,value[0],P::ContinuationA);
    put(&mut out,value[1],P::ContinuationA);
    let state=publish_state(r(1)?,r(0)?,0xfe00_0044)?;
    put(&mut out,state[0],P::ContinuationA);
    put(&mut out,state[1],P::Final);
    put(&mut out,0xa000_0000 | (u32::from(r(1)?.number())<<16),P::Final); // dccleana(R1)
    put(&mut out,read_hardware_thread(r(1)?),P::Final);
    let stack=cdsp_boot::encode_extended_assignment(r(29)?,-0x1fc8e18,true);
    put(&mut out,stack[0],P::ContinuationA);
    put(&mut out,stack[1],P::Final);
    put(&mut out,thread_stack(r(1)?,5)?,P::Final);
    put(&mut out,read_predicate_group(r(28)?),P::ContinuationA);
    put(&mut out,H::AssignRegister{dst:r(19)?,src:r(2)?}.encode()?,P::Final);
    let call=cdsp_early_controllers::encode_extended_call(0x96c0_1d5c,0x96c0_c004)?;
    put(&mut out,call[0],P::ContinuationA);
    put(&mut out,call[1],P::Final);
    put(&mut out,H::AssignRegister{dst:r(2)?,src:r(19)?}.encode()?,P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:5}.encode()?,P::ContinuationA);
    let new_store=publish_state(r(1)?,r(0)?,0xfe02_4d00)?;
    put(&mut out,new_store[0],P::ContinuationA);
    put(&mut out,new_store[1],P::Final);
    put(&mut out,0xa000_0000|(u32::from(r(1)?.number())<<16),P::Final);
    if out.len()!=FUNCTION_BYTES {return Err("CDSP worker thread start has unexpected V66 packet count".into());}
    Ok(out)
}
pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_worker_idle::verify_reference(reference)?;
    if cdsp_early_worker_idle::REGION_END!=FILE_OFFSET{return Err("CDSP worker thread does not follow worker idle path".into());}
    let generated=build_worker_thread_start()?;
    if reference.get(FILE_OFFSET..REGION_END)!=Some(generated.as_slice()){
        return Err("CDSP worker thread start differs from pinned CDSP reference".into());
    }
    Ok(())
}
#[cfg(test)] mod tests {
 use super::*;
 #[test] fn worker_thread_init_encodes_checked_operands(){
    let r=|n|Gpr::new(n).unwrap();
    assert_eq!(build_worker_thread_start().unwrap().len(),FUNCTION_BYTES);
    assert_eq!(gp_state_not_one(PHYSICAL_ADDRESS,0x96c0_1fd0,1).unwrap(),0x2412_c156);
    assert!(gp_state_not_one(PHYSICAL_ADDRESS,PHYSICAL_ADDRESS,1).is_err());
    assert!(gp_state_not_one(PHYSICAL_ADDRESS,0x96c0_1fd0,2).is_err());
    assert_eq!(thread_stack(r(1),5).unwrap(),0xc401_ddbd);
    assert!(thread_stack(r(1),8).is_err());
    assert_eq!(read_predicate_group(r(28)),0x6a04_c01c);
    assert_eq!(publish_state(r(1),r(0),0xfe00_0044).unwrap(),[0x0fe0_4001,0xaba1_d284]);
    assert!(publish_state(r(1),r(0),0xfe00_0045).is_err());
 }
}
