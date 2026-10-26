// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 worker final controller command/callback sequence.
//! The first branch skips optional callbacks, which are stored as typed
//! packet-relative direct and extended call instructions. This is not a
//! complete image or independent replacement for Qualcomm firmware signing.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::{cdsp_early_controllers,cdsp_early_worker_command as callbacks,
            cdsp_early_worker_snapshot as snapshot};

pub const FILE_OFFSET:usize=0x4f64;
pub const REGION_END:usize=0x4fd0;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1f64;
fn put(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}

/// The single V66 word `P0=cmp.eq(Rsource,#1); if(P0.new) jump:nt target`.
/// The 8-bit halfword displacement is measured from this packet's address.
fn branch_if_input_one(src:Gpr,compare:u8,packet:u32,target:u32)->Result<u32,String>{
    if src.number()>7 || compare!=1 || packet%4!=0 || target%4!=0 || target<=packet {
        return Err("V66 input-one branch requires compact source, #1 and aligned forward target".into());
    }
    let delta=target-packet;
    if delta>0x1fc || delta%2!=0 {return Err("input-one branch outside verified 8-bit halfword range".into());}
    Ok(0x1000_c000 | (u32::from(src.number())<<16)
       |(u32::from(compare)<<8)|((delta/2)&0xff))
}

pub fn build_worker_final_calls()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,branch_if_input_one(r(1)?,1,PHYSICAL_ADDRESS,0x96c0_1fd0)?,P::Final);
    put(&mut out,callbacks::direct_call(0x96c0_1f68,0x96c0_29b0)?,P::Final);
    for (here,target) in [(0x96c0_1f6c,0x96c0_1318),(0x96c0_1f74,0x96c0_c004)] {
        let pair=cdsp_early_controllers::encode_extended_call(here,target)?;
        put(&mut out,pair[0],P::ContinuationA);
        put(&mut out,pair[1],P::Final);
    }
    // Packet `R0=#3; memw(R1=##0xfe024d00)=R0.new`.
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:3}.encode()?,P::ContinuationA);
    let first=callbacks::publish_new_state(r(1)?,r(0)?,0xfe02_4d00)?;
    put(&mut out,first[0],P::ContinuationA);
    put(&mut out,first[1],P::Final);
    put(&mut out,callbacks::cache_clean(r(1)?),P::Final);
    for (here,target) in [(0x96c0_1f8c,0x96c0_2184),
                          (0x96c0_1f90,0x96c0_2268),
                          (0x96c0_1f94,0x96c0_2418)] {
        put(&mut out,callbacks::direct_call(here,target)?,P::Final);
    }
    // Four-word new-value packet with an explicit zero extender.
    let assign=cdsp_boot::encode_extended_assignment(r(0)?,30,false);
    put(&mut out,assign[0],P::ContinuationA);
    put(&mut out,assign[1],P::ContinuationA);
    let second=callbacks::publish_new_state(r(1)?,r(0)?,0xfe00_0044)?;
    put(&mut out,second[0],P::ContinuationA);
    put(&mut out,second[1],P::Final);
    put(&mut out,callbacks::cache_clean(r(1)?),P::Final);
    for (here,target) in [
        (0x96c0_1fac,0x96c0_1364),
        (0x96c0_1fb4,0x96c0_2458),
        (0x96c0_1fbc,0x96c0_cdb0),
        (0x96c0_1fc4,0x96c0_c004),
    ] {
        let pair=cdsp_early_controllers::encode_extended_call(here,target)?;
        put(&mut out,pair[0],P::ContinuationA);
        put(&mut out,pair[1],P::Final);
    }
    put(&mut out,callbacks::direct_call(0x96c0_1fcc,0x96c0_2044)?,P::Final);
    if out.len()!=FUNCTION_BYTES {
        return Err(format!("CDSP final callbacks emitted {} instead of {} bytes",out.len(),FUNCTION_BYTES));
    }
    Ok(out)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    snapshot::verify_reference(reference)?;
    if snapshot::REGION_END!=FILE_OFFSET {
        return Err("worker final calls must directly follow supervisor snapshot".into());
    }
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_worker_final_calls()?.as_slice()) {
        return Err("worker final calls differ from CDSP reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn final_callbacks_have_guarded_targets(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(build_worker_final_calls().unwrap().len(),108);
        assert_eq!(branch_if_input_one(r(1),1,PHYSICAL_ADDRESS,0x96c0_1fd0).unwrap(),0x1001_c136);
        assert!(branch_if_input_one(r(8),1,PHYSICAL_ADDRESS,0x96c0_1fd0).is_err());
        assert!(branch_if_input_one(r(1),0,PHYSICAL_ADDRESS,0x96c0_1fd0).is_err());
        assert!(branch_if_input_one(r(1),1,PHYSICAL_ADDRESS,PHYSICAL_ADDRESS).is_err());
        assert!(branch_if_input_one(r(1),1,PHYSICAL_ADDRESS,PHYSICAL_ADDRESS+0x200).is_err());
        assert_eq!(callbacks::direct_call(0x96c0_1fcc,0x96c0_2044).unwrap(),0x5a00_c03c);
    }
}
