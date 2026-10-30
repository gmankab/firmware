// SPDX-License-Identifier: GPL-3.0-only
//! CDSP Hexagon V66 system-state restoration and supervisor worker startup.
//! The first 72 bytes restore the SSR, SGP0, IMASK, CCR, and USR state and
//! jump into a later supervisor trampoline; the next 36 bytes start a task.
//! No instruction words are imported from the OEM firmware.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H,SystemRegister};
use super::cdsp_early_tlb_state as previous;
use super::cdsp_early_controllers::encode_extended_call;

pub const FILE_OFFSET:usize=0x59f0;
pub const REGION_END:usize=0x5a5c;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_29f0;
fn put(o:&mut Vec<u8>,code:u32,p:P){o.extend_from_slice(&p.apply(code).to_le_bytes());}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum ControllerRegister{Sgp0,InterruptMask,Control,User}
fn write_special(src:Gpr,reg:ControllerRegister)->Result<u32,String>{
    let op=match reg{
        ControllerRegister::Sgp0=>0x6700_c000,
        ControllerRegister::InterruptMask=>0x6700_c00a,
        ControllerRegister::Control=>0x6700_c007,
        ControllerRegister::User=>0x6220_c008,
    };
    Ok(op|(u32::from(src.number())<<16))
}
/// Hexagon `start(R0)` privileged controller launch. This deliberately
/// refuses untested register variants rather than manufacturing an opcode.
fn start_supervisor(src:Gpr)->Result<u32,String>{
    if src.number()!=0{return Err("CDSP start instruction tested only for R0".into());}
    Ok(0x6460_c020)
}
/// Extended jump displacement measured at the **packet start**, which is
/// the previous singleton `USR=R2` and not the immext itself. LLVM6 confirms
/// the extended jump split into high14 and low6 signed delta bits.
fn extended_jump(start:u32,target:u32)->Result<[u32;2],String>{
    if start%4!=0||target%4!=0{return Err("Hexagon long jump addresses unaligned".into());}
    let delta=i64::from(target)-i64::from(start);
    if delta<i64::from(i32::MIN)||delta>i64::from(i32::MAX){return Err("Hexagon jump displacement outside i32".into());}
    let bits=delta as i32 as u32;
    let ext=((bits>>20)<<16)|((bits>>6)&0x3fff)|0x4000;
    let body=0x5800_c000|((bits&0x3f)<<1);
    Ok([ext,body])
}

fn restore_controller_registers(o:&mut Vec<u8>)->Result<(),String>{
    let r=|n|Gpr::new(n);
    let flags=cdsp_boot::encode_extended_assignment(r(4)?,0xfbbb,false);
    put(o,flags[0],P::ContinuationA);put(o,flags[1],P::ContinuationA);
    let status=cdsp_boot::encode_extended_gp_word_load(r(0)?,0xfe02_6e00);
    put(o,status[0],P::ContinuationA);put(o,status[1],P::Final);
    put(o,H::ClearBit{dst:r(0)?,src:r(0)?,position:16}.encode()?,P::ContinuationA);
    let sgp=cdsp_boot::encode_extended_gp_word_load(r(28)?,0xfe03_7ac0);
    put(o,sgp[0],P::ContinuationA);put(o,sgp[1],P::Final);
    let ccr=cdsp_boot::encode_extended_gp_word_load(r(1)?,0xfe03_7aac);
    put(o,ccr[0],P::ContinuationA);put(o,ccr[1],P::ContinuationA);
    let user=cdsp_boot::encode_extended_gp_word_load(r(2)?,0xfe03_7ab0);
    put(o,user[0],P::ContinuationA);put(o,user[1],P::Final);
    put(o,write_special(r(28)?,ControllerRegister::Sgp0)?,P::Final);
    put(o,write_special(r(4)?,ControllerRegister::InterruptMask)?,P::Final);
    put(o,H::WriteSystem{system:SystemRegister::Ssr,src:r(0)?}.encode()?,P::Final);
    put(o,write_special(r(1)?,ControllerRegister::Control)?,P::Final);
    put(o,write_special(r(2)?,ControllerRegister::User)?,P::ContinuationA);
    let jump=extended_jump(0x96c0_2a2c,0x96c0_8860)?;
    put(o,jump[0],P::ContinuationA);put(o,jump[1],P::Final);
    if o.len()!=0x48 {return Err("CDSP SSR restore must end at file 0x5a38".into());}
    Ok(())
}
fn launch_worker(o:&mut Vec<u8>)->Result<(),String>{
    let r=|n|Gpr::new(n);
    let ep=cdsp_boot::encode_extended_gp_word_load(r(0)?,0xfe03_10e0);
    put(o,ep[0],P::ContinuationA);put(o,ep[1],P::ContinuationA);
    let sp=cdsp_boot::encode_extended_gp_word_load(r(29)?,0xfe03_7ac0);
    put(o,sp[0],P::ContinuationA);put(o,sp[1],P::Final);
    put(o,start_supervisor(r(0)?)?,P::Final);
    let jump=encode_extended_call(0x96c0_2a4c,0x96c3_4400)?;
    put(o,jump[0],P::ContinuationA);put(o,jump[1],P::ContinuationA);
    put(o,H::AssignSmallImmediate{dst:r(0)?,value:0}.encode()?,P::Final);
    put(o,H::JumpBackward{displacement_bytes:-12}.encode()?,P::Final);
    if o.len()!=FUNCTION_BYTES{return Err("CDSP worker start output not 108 bytes".into());}
    Ok(())
}
pub fn build_controller_restore()->Result<Vec<u8>,String>{
    let mut result=Vec::with_capacity(FUNCTION_BYTES);
    restore_controller_registers(&mut result)?;
    launch_worker(&mut result)?;
    Ok(result)
}
pub fn verify_reference(reference:&[u8])->Result<(),String>{
    previous::verify_reference(reference)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("CDSP controller state not adjacent to TLB snapshot".into());}
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_controller_restore()?.as_slice()){
        return Err("source-rebuilt CDSP supervisor restore differs from pinned firmware".into());
    }
    Ok(())
}
#[cfg(test)] mod tests{
    use super::*;
    #[test]fn extended_worker_control_respects_packet_start_and_registers(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,108);
        assert_eq!(extended_jump(0x96c0_2a2c,0x96c0_8860).unwrap(),[0x4178,0x5800_c068]);
        assert!(extended_jump(0x96c0_2a2d,0x96c0_8860).is_err());
        assert_eq!(write_special(r(28),ControllerRegister::Sgp0).unwrap(),0x671c_c000);
        assert_eq!(write_special(r(4),ControllerRegister::InterruptMask).unwrap(),0x6704_c00a);
        assert_eq!(write_special(r(1),ControllerRegister::Control).unwrap(),0x6701_c007);
        assert_eq!(write_special(r(2),ControllerRegister::User).unwrap(),0x6222_c008);
        assert!(start_supervisor(r(1)).is_err());
        assert_eq!(build_controller_restore().unwrap().len(),FUNCTION_BYTES);
    }
}
