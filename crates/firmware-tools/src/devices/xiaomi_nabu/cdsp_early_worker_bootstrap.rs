// SPDX-License-Identifier: GPL-3.0-only
//! Worker-controller bootstrap and state handoff. This function follows the
//! variable-width dispatcher and its separately verified linker zeros, so
//! the emitted instructions start at file 0x4bc0 rather than 0x4bb8.
//! Each word is generated from explicit Hexagon V66 operands and parse bits.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_controllers;
use super::cdsp_early_memory_dispatch;

pub const FILE_OFFSET:usize=0x4bc0;
pub const REGION_END:usize=0x4c1c;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1bc0;

fn put(out:&mut Vec<u8>,instruction:u32,marker:P){out.extend_from_slice(&marker.apply(instruction).to_le_bytes());}
fn frame_allocation(bytes:u16)->Result<u32,String>{
    if bytes==0||bytes%8!=0||bytes>0x1ff8{return Err("CDSP worker frame must be nonzero, 8-byte aligned, within 13 bits".into());}
    Ok(0xa09d_c000|u32::from(bytes/8))
}
fn stack_double(src:Gpr,base:Gpr,load:bool,offset:u16)->Result<u32,String>{
    if src.number()%2!=0||offset%8!=0||offset>0x3f8{return Err("CDSP worker MEMD register pair or offset misaligned".into());}
    if load {Ok(0x91c0_c000 |(u32::from(base.number())<<16)|(u32::from(offset/8)<<8)|u32::from(src.number()))}
    else {Ok(0xa1c0_c000 |(u32::from(base.number())<<16)|(u32::from(src.number())<<8)|u32::from(offset/8))}
}
fn worker_word_load(dst:Gpr,base:Gpr)->u32{
    0x9180_c000|(u32::from(base.number())<<16)|u32::from(dst.number())
}
fn conditional_start_branch(source:Gpr,packet_start:u32,target:u32)->Result<u32,String>{
    if source.number()>7||packet_start%4!=0||target%4!=0||target<=packet_start||target-packet_start>0x1fe{
        return Err("CDSP worker start branch needs low register and short forward target".into());
    }
    Ok(0x1040_c000|(u32::from(source.number())<<16)|((target-packet_start)/2))
}
fn read_privileged_sgp0(dst:Gpr)->u32{0x6e80_c000|u32::from(dst.number())}
fn restore_saved_frame()->u32{0x901e_c01e} // Architectural Hexagon deallocframe(FP); no register operands.
/// Direct Hexagon jump to the controller worker at physical 0x96c047a8.
/// The short forward displacement is stored in halfwords with an opcode-
/// reserved bit 11. Other targets with nonrepresentable displacement are
/// rejected rather than silently truncating the displacement.
fn jump_to_controller(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start%4!=0||target%4!=0||target<=packet_start{
        return Err("CDSP worker jump must be aligned and forward".into());
    }
    let delta=target-packet_start;
    let half=delta/2;
    if half&!0x17ff !=0{return Err("CDSP worker jump displacement cannot fit the verified field".into());}
    Ok(0x5800_c000|half)
}

pub fn build_worker_bootstrap()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,frame_allocation(8)?,P::Final);
    let callback=cdsp_early_controllers::encode_extended_call(0x96c0_1bc4,0x96c0_74b8)?;
    put(&mut out,callback[0],P::ContinuationA);
    put(&mut out,callback[1],P::ContinuationA);
    put(&mut out,stack_double(r(0)?,r(29)?,false,0)?,P::Final);
    put(&mut out,conditional_start_branch(r(0)?,0x96c0_1bd0,0x96c0_1be0)?,P::Final);
    put(&mut out,H::ReadSyscfg{dst:r(1)?}.encode()?,P::Final);
    put(&mut out,H::ClearBit{dst:r(1)?,src:r(1)?,position:4}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:cdsp_boot::SystemRegister::Syscfg,src:r(1)?}.encode()?,P::Final);
    put(&mut out,read_privileged_sgp0(r(0)?),P::Final);
    put(&mut out,worker_word_load(r(0)?,r(0)?),P::Final);
    let ready=cdsp_boot::encode_extended_word_load(r(0)?,r(0)?,0x100)?;
    put(&mut out,ready[0],P::ContinuationA);
    put(&mut out,ready[1],P::ContinuationA);
    put(&mut out,H::BranchIfNewLoadZeroForward{displacement_bytes:0x28}.encode()?,P::Final);
    let pending=cdsp_boot::encode_extended_word_load(r(0)?,r(0)?,0x10)?;
    put(&mut out,pending[0],P::ContinuationA);
    put(&mut out,pending[1],P::ContinuationA);
    put(&mut out,H::BranchIfNewLoadZeroForward{displacement_bytes:0x1c}.encode()?,P::Final);
    put(&mut out,stack_double(r(0)?,r(29)?,true,0)?,P::Final);
    put(&mut out,restore_saved_frame(),P::Final);
    put(&mut out,H::AssignRegister{dst:r(0)?,src:r(18)?}.encode()?,P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0x10}.encode()?,P::Final);
    put(&mut out,stack_double(r(0)?,r(29)?,true,0)?,P::Final);
    put(&mut out,restore_saved_frame(),P::Final);
    put(&mut out,jump_to_controller(0x96c0_1c18,0x96c0_47a8)?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP worker bootstrap has incorrect instruction count".into());}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_memory_dispatch::verify_reference(original)?;
    if cdsp_early_memory_dispatch::ALIGNMENT_END!=FILE_OFFSET {return Err("CDSP worker bootstrap does not follow verified linker alignment".into());}
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_worker_bootstrap()?.as_slice()){
        return Err("CDSP worker bootstrap differs from SHA-256 pinned reference".into());
    }
    Ok(())
}
pub fn build_supervisor_with_worker()->Result<Vec<u8>,String>{
    let mut prefix=cdsp_early_memory_dispatch::build_supervisor_with_dispatch()?;
    // The linker alignment is an independently checked *file layout* span,
    // not source-encoded executable instructions.
    prefix.extend([0u8;cdsp_early_memory_dispatch::ALIGNMENT_BYTES]);
    prefix.extend(build_worker_bootstrap()?);
    Ok(prefix)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn worker_bootstrap_packet_boundaries_and_stack_rules(){
        assert_eq!(FUNCTION_BYTES,92);
        assert_eq!(build_worker_bootstrap().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
        assert!(frame_allocation(7).is_err());
        assert!(stack_double(Gpr::new(1).unwrap(),Gpr::new(29).unwrap(),false,0).is_err());
        assert!(conditional_start_branch(Gpr::new(10).unwrap(),0x96c0_1bd0,0x96c0_1be0).is_err());
        assert!(jump_to_controller(0x96c0_1c18,0x96c0_1c14).is_err());
        assert_eq!(jump_to_controller(0x96c0_1c18,0x96c0_47a8).unwrap(),0x5800_d5c8);
        assert!(jump_to_controller(0x96c0_1c18,0x96c0_5c18).is_err());
    }
}
