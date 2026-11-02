// SPDX-License-Identifier: GPL-3.0-only
//! CDSP supervisor transfer: one long-range controller trampoline, SSR reset,
//! callback cascade, SYSCFG restore and final controller status notification.
//! Encodings retain V66 packet boundaries and typed immediate/call semantics.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H,SystemRegister};
use super::cdsp_early_worker_sync as previous;
use super::cdsp_early_worker_command as op;
use super::cdsp_early_controllers;

pub const FILE_OFFSET:usize=0x5060;
pub const REGION_END:usize=0x50f8;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2060;
fn put(out:&mut Vec<u8>,word:u32,mark:P){out.extend_from_slice(&mark.apply(word).to_le_bytes());}

/// Encode a direct forward V66 jump using the two noncontiguous immediate
/// slices used by the ordinary CALL instruction class. This is not the short
/// scalar JumpRelative subset, which deliberately covers only 0x1ffe bytes.
fn long_forward_jump(start:u32,target:u32)->Result<u32,String>{
    if start%4!=0||target%4!=0||target<=start {
        return Err("long Hexagon jump needs aligned strictly forward destination".into());
    }
    let distance=target-start;
    if distance>0xffffc || distance%2!=0{
        return Err("long Hexagon jump exceeds verified signed 20-bit byte range".into());
    }
    let half=distance/2;
    Ok(0x5800_c000|((half>>14&0xff)<<16)|(half&0x3fff))
}
/// Signed near CALL with an extra high sign field compared with the
/// positive-only subset used by existing controller short-call helpers.
fn signed_near_call(start:u32,target:u32)->Result<u32,String>{
    if start%4!=0||target%4!=0{return Err("Hexagon near CALL addresses must align".into());}
    let bytes=i64::from(target)-i64::from(start);
    if bytes%2!=0 || !(-0x100000..=0xffffc).contains(&bytes){
        return Err("Hexagon near CALL displacement outside evidenced range".into());
    }
    let half=(bytes/2) as i32 as u32;
    Ok(0x5a00_c000|((half>>14&0x1ff)<<16)|(half&0x3fff))
}
fn read_status_register(dst:Gpr)->u32{0x6e86_c000|u32::from(dst.number())}

pub fn build_worker_handoff()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    // Tail trampoline to the controller implementation.
    put(&mut out,H::AddSmallImmediate{dst:r(1)?,src:r(1)?,value:16}.encode()?,P::Final);
    put(&mut out,long_forward_jump(0x96c0_2064,0x96c3_23c4)?,P::Final);
    // Reset SSR slice, notify hardware and dispatch both controller callbacks.
    put(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:0}.encode()?,P::Final);
    put(&mut out,read_status_register(r(0)?),P::Final);
    put(&mut out,H::InsertBitField{dst:r(0)?,src:r(1)?,width:3,offset:16}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:SystemRegister::Ssr,src:r(0)?}.encode()?,P::Final);
    put(&mut out,op::direct_call(0x96c0_2078,0x96c0_29b0)?,P::Final);
    for (here,target) in [(0x96c0_207c,0x96c0_1318),(0x96c0_2084,0x96c0_c004)] {
        let extended=cdsp_early_controllers::encode_extended_call(here,target)?;
        put(&mut out,extended[0],P::ContinuationA);
        put(&mut out,extended[1],P::Final);
    }
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:2}.encode()?,P::ContinuationA);
    let ready=op::publish_new_state(r(1)?,r(0)?,0xfe02_4d00)?;
    put(&mut out,ready[0],P::ContinuationA);
    put(&mut out,ready[1],P::Final);
    put(&mut out,op::cache_clean(r(1)?),P::Final);
    put(&mut out,op::direct_call(0x96c0_209c,0x96c0_2418)?,P::Final);
    for (here,target) in [
        (0x96c0_20a0,0x96c0_1364),
        (0x96c0_20a8,0x96c0_cdb0),
        (0x96c0_20b0,0x96c0_c004),
    ] {
        let pair=cdsp_early_controllers::encode_extended_call(here,target)?;
        put(&mut out,pair[0],P::ContinuationA);
        put(&mut out,pair[1],P::Final);
    }
    put(&mut out,signed_near_call(0x96c0_20b8,0x96c0_2044)?,P::Final);
    // Restore SYSCFG and clear the original three-bit SSR field, retaining
    // the original two-instruction packet for the status read and assignment.
    put(&mut out,H::ReadSyscfg{dst:r(0)?}.encode()?,P::Final);
    put(&mut out,H::ClearBit{dst:r(0)?,src:r(0)?,position:4}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:SystemRegister::Syscfg,src:r(0)?}.encode()?,P::Final);
    put(&mut out,read_status_register(r(0)?),P::ContinuationA);
    put(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:0}.encode()?,P::Final);
    put(&mut out,H::InsertBitField{dst:r(0)?,src:r(1)?,width:3,offset:16}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:SystemRegister::Ssr,src:r(0)?}.encode()?,P::Final);
    // Status #36 deliberately uses an additional zero extender, as in the
    // reference's four-word producer/new-value memory packet.
    let current=cdsp_boot::encode_extended_assignment(r(0)?,36,false);
    put(&mut out,current[0],P::ContinuationA);
    put(&mut out,current[1],P::ContinuationA);
    let state=op::publish_new_state(r(1)?,r(0)?,0xfe00_0044)?;
    put(&mut out,state[0],P::ContinuationA);
    put(&mut out,state[1],P::Final);
    put(&mut out,op::cache_clean(r(1)?),P::Final);
    let callback=cdsp_early_controllers::encode_extended_call(0x96c0_20ec,0x96c3_26d0)?;
    put(&mut out,callback[0],P::ContinuationA);
    put(&mut out,callback[1],P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP worker handoff byte count mismatch".into());}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    previous::verify_reference(original)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("worker handoff must follow controller synchronization".into());}
    let ours=build_worker_handoff()?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(ours.as_slice()){
        return Err("CDSP handoff/callback sequence differs from pinned reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn supervisor_handoff_branch_flags_and_calls_are_checked(){
        assert_eq!(FUNCTION_BYTES,152);
        assert_eq!(build_worker_handoff().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(long_forward_jump(0x96c0_2064,0x96c3_23c4).unwrap(),0x5806_c1b0);
        assert!(long_forward_jump(0x96c0_2064,0x96c0_2064).is_err());
        assert!(long_forward_jump(0x96c0_2064,0x96e0_0000).is_err());
        assert_eq!(read_status_register(Gpr::new(0).unwrap()),0x6e86_c000);
        assert_eq!(signed_near_call(0x96c0_20b8,0x96c0_2044).unwrap(),0x5bff_ffc6);
    }
}
