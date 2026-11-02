// SPDX-License-Identifier: GPL-3.0-only
//! Bank enumerator controller-service result: fused P0.new branch, 32-bit
//! duplex stack load, callback dispatch, and status-mask predicate logic.
//! Source-built original V66 packet words, CDSP file [0x56b78,0x56b98).
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_bank_initialize;
pub const FILE_OFFSET:usize=0x56b78;
pub const END_FILE_OFFSET:usize=0x56b98;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8b78;
pub const RWE_START:usize=cdsp_late_controller_bank_initialize::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=
    cdsp_late_controller_bank_initialize::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_bank_initialize::RWE_PREFIX_LAYOUT_LEN;
const MISSING_HANDLE:u32=0x96cc_8e90;
const ERROR_HANDLER:u32=0x96cc_8ea0;
const BANK_SERVICE:u32=0x96cc_e768;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn put(b:&mut Vec<u8>,w:u32){b.extend_from_slice(&w.to_le_bytes());}

/// `P0=cmp.eq(R0,#0); if(!P0.new) jump:nt missing_handle` uses a
/// packet-relative signed displacement with a high field checked by LLVM6.
pub fn encode_skip_valid_controller(src:G,pred:u8,packet:u32,target:u32)->Result<u32,String>{
    if (src.number(),pred,packet,target)!=(0,0,PHYSICAL_ADDRESS,MISSING_HANDLE){
        return Err("controller result predicate/branch target not verified".into());
    }
    let delta=target.checked_sub(packet).ok_or("controller result jump backwards")?;
    if delta&1!=0||delta>0x3fe{return Err("controller result jump exceeds narrow span".into());}
    let half=delta/2;
    // The branch class extends bit 8 of its displacement into the opcode's
    // higher field, as confirmed with the independent LLVM6 fixture.
    Ok(M::Final.apply(0x1050_0000|(half&0xff)))
}
/// `P0=or(P0,!P0)` normalizes the callback's predicate state.
pub fn encode_invert_or_predicate(predicate:u8)->Result<u32,String>{
    if predicate!=0{return Err("only original P0 feedback predicate is supported".into());}
    Ok(M::ContinuationA.apply(0x6be0_c000))
}
/// One-word Hexagon V66 duplex: `R0=#3; R1=memw(FP+28)`. The paired
/// immediate/load operands are checked, unlike a raw two-word replacement.
pub fn encode_controller_callback_duplex(dst:G,constant:u8,loaded:G,frame:G,offset:u8)->Result<u32,String>{
    if (dst.number(),constant,loaded.number(),frame.number(),offset)!=(0,3,1,29,28){
        return Err("unverified callback argument/load duplex".into());
    }
    if offset%4!=0{return Err("duplex stack load must be word aligned".into());}
    // Fixed-register duplex opcode layout independently decoded by Rizin.
    Ok(0x4800_0000|(u32::from(constant)<<20)|0x0000_3c00|
        (u32::from(offset/4)<<4)|u32::from(loaded.number()))
}
/// Ordinary packet-relative call from 0x96cc8b84 to the controller service.
pub fn encode_controller_callback_call(packet:u32,target:u32)->Result<u32,String>{
    if (packet,target)!=(0x96cc_8b84,BANK_SERVICE){return Err("unknown bank callback packet/target".into());}
    let delta=target.checked_sub(packet).ok_or("backwards controller callback")?;
    if delta&1!=0||delta>0x1ffffe{return Err("controller callback displacement invalid".into());}
    Ok(M::ContinuationA.apply(0x5a00_0000|
        (((delta>>15)&0x3f)<<16)|((delta>>1)&0x3fff)))
}
/// Convert P0 to R2 after the controller callback changes predicate state.
pub fn encode_result_predicate_move(dst:G,predicate:u8)->Result<u32,String>{
    if (dst.number(),predicate)!=(2,0){return Err("bank callback status requires R2=P0".into());}
    Ok(M::Final.apply(0x8940_0000|u32::from(dst.number())))
}
/// One signed ADD by minus one, counting active banks prior to the error path.
pub fn encode_prior_bank_count(dst:G,src:G,delta:i16)->Result<u32,String>{
    if (dst.number(),src.number(),delta)!=(19,16,-1){return Err("bank count requires R19=R16-1".into());}
    Ok(M::ContinuationA.apply(S::AddSignedSmallImmediate{dst,src,value:delta}.encode()?))
}
/// Within the P0.new packet, R24 is set to the 128-byte status flag.
pub fn encode_accepted_bank_flag(dst:G,predicate:u8,value:u16)->Result<u32,String>{
    if (dst.number(),predicate,value)!=(24,0,128){return Err("unexpected bank status flag operands".into());}
    Ok(M::ContinuationA.apply(0x7e00_f000|u32::from(dst.number())))
}
/// `P0=cmp.gt(R0,-1); if(!P0.new) jump:nt` references the *packet*
/// beginning at 0x96cc8b90, not the final consumer at 0x96cc8b94.
pub fn encode_bank_service_failed(src:G,packet:u32,target:u32)->Result<u32,String>{
    if src.number()!=0||(packet,target)!=(0x96cc_8b90,ERROR_HANDLER){
        return Err("unsupported service failure predicate/packet".into());
    }
    let delta=target.checked_sub(packet).ok_or("bank failure target backwards")?;
    if delta&1!=0||delta>0x3fe{return Err("bank failure displacement out of tested range".into());}
    Ok(M::Final.apply(0x11d0_0000|((delta/2+2)&0x1ff)))
}
pub fn build_controller_predicates()->Result<Vec<u8>,String>{
    let mut v=Vec::with_capacity(EXECUTABLE_LEN);
    put(&mut v,encode_skip_valid_controller(r(0)?,0,PHYSICAL_ADDRESS,MISSING_HANDLE)?);
    put(&mut v,encode_invert_or_predicate(0)?);
    put(&mut v,encode_controller_callback_duplex(r(0)?,3,r(1)?,r(29)?,28)?);
    put(&mut v,encode_controller_callback_call(0x96cc_8b84,BANK_SERVICE)?);
    put(&mut v,encode_result_predicate_move(r(2)?,0)?);
    put(&mut v,encode_prior_bank_count(r(19)?,r(16)?,-1)?);
    put(&mut v,encode_accepted_bank_flag(r(24)?,0,128)?);
    put(&mut v,encode_bank_service_failed(r(0)?,0x96cc_8b90,ERROR_HANDLER)?);
    if v.len()!=EXECUTABLE_LEN{return Err("eight controller predicate words required".into());}
    Ok(v)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut result=cdsp_late_controller_bank_initialize::build_prefix()?;
    result.extend(build_controller_predicates()?);
    if result.len()!=FULL_RWE_LAYOUT_LEN||result.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("controller predicate RWE prefix accounting differs".into());
    }
    Ok(result)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_controller_predicates()?.as_slice()){
        return Err("controller predicate source does not match original CDSP".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("joined controller predicate RWE prefix differs".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn bank_predicate_words_and_operand_rejections(){
        let b=build_controller_predicates().unwrap();assert_eq!(b.len(),32);
        let got:Vec<u32>=b.chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect();
        assert_eq!(got,[0x1050_c08c,0x6be0_4000,0x4830_3c71,0x5a00_6df2,
            0x8940_c002,0xbff0_7ff3,0x7e00_7018,0x11d0_c18a]);
        assert!(encode_skip_valid_controller(r(1).unwrap(),0,PHYSICAL_ADDRESS,MISSING_HANDLE).is_err());
        assert!(encode_controller_callback_duplex(r(0).unwrap(),3,r(1).unwrap(),r(29).unwrap(),24).is_err());
        assert!(encode_prior_bank_count(r(19).unwrap(),r(16).unwrap(),-2).is_err());
        assert!(encode_bank_service_failed(r(0).unwrap(),0x96cc8b94,ERROR_HANDLER).is_err());
    }
    #[test]fn predicate_prefix_includes_all_prior_code_without_implicit_fill(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,2968);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2908);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,60);
        assert_eq!(build_prefix().unwrap().len(),2968);
    }
}
