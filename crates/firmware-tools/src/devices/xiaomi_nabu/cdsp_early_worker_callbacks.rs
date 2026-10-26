// SPDX-License-Identifier: GPL-3.0-only
//! Issue the worker's extended status and supervisor callbacks after the
//! controller command update. Public V66 scalar operators are combined with
//! checked packet-relative calls and named controller register addresses.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::{cdsp_early_controllers,cdsp_early_worker_command as previous};

pub const FILE_OFFSET:usize=0x4e74;
pub const REGION_END:usize=0x4ec8;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1e74;
fn put(out:&mut Vec<u8>,word:u32,mark:P){out.extend_from_slice(&mark.apply(word).to_le_bytes());}

pub fn build_worker_callbacks()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:2}.encode()?,P::ContinuationA);
    let status=previous::publish_new_state(r(1)?,r(0)?,0xfe02_4d00)?;
    put(&mut out,status[0],P::ContinuationA);
    put(&mut out,status[1],P::Final);
    put(&mut out,previous::cache_clean(r(1)?),P::Final);
    for (source,target) in [
        (0x96c0_1e84,0x96c0_23dc),
        (0x96c0_1e88,0x96c0_2418),
    ] {
        put(&mut out,previous::direct_call(source,target)?,P::Final);
    }
    // Four-word status packet: R0=##30, memw(R1=##0xfe000044)=R0.new.
    // The zero extender preceding the scalar assignment is part of the
    // originally scheduled packet; removing it changes V66 packet bytes.
    let value=cdsp_boot::encode_extended_assignment(r(0)?,30,false);
    put(&mut out,value[0],P::ContinuationA);
    put(&mut out,value[1],P::ContinuationA);
    let diagnostic=previous::publish_new_state(r(1)?,r(0)?,0xfe00_0044)?;
    put(&mut out,diagnostic[0],P::ContinuationA);
    put(&mut out,diagnostic[1],P::Final);
    put(&mut out,previous::cache_clean(r(1)?),P::Final);
    for (source,target) in [
        (0x96c0_1ea0,0x96c0_1364),
        (0x96c0_1ea8,0x96c0_2458),
        (0x96c0_1eb0,0x96c0_cdb0),
        (0x96c0_1eb8,0x96c0_c004),
    ] {
        let call=cdsp_early_controllers::encode_extended_call(source,target)?;
        put(&mut out,call[0],P::ContinuationA);
        put(&mut out,call[1],P::Final);
    }
    for (source,target) in [
        (0x96c0_1ec0,0x96c0_2044),
        (0x96c0_1ec4,0x96c3_736c),
    ] {
        put(&mut out,previous::direct_call(source,target)?,P::Final);
    }
    if out.len()!=FUNCTION_BYTES {return Err("CDSP worker callback burst length incorrect".into());}
    Ok(out)
}
pub fn verify_reference(image:&[u8])->Result<(),String>{
    previous::verify_reference(image)?;
    if previous::REGION_END!=FILE_OFFSET {return Err("CDSP callback burst must directly follow controller command".into());}
    let ours=build_worker_callbacks()?;
    if image.get(FILE_OFFSET..REGION_END)!=Some(ours.as_slice()){
        return Err("CDSP worker callback burst not byte-identical to reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn callback_status_and_calls_are_aligned(){
        assert_eq!(FUNCTION_BYTES,84);
        assert_eq!(build_worker_callbacks().unwrap().len(),84);
        assert_eq!(previous::publish_new_state(Gpr::new(1).unwrap(),Gpr::new(0).unwrap(),0xfe02_4d00).unwrap(),[0x0fe0_4934,0xaba1_d280]);
        assert_eq!(previous::direct_call(0x96c0_1ec4,0x96c3_736c).unwrap(),0x5a06_ea54);
    }
}
