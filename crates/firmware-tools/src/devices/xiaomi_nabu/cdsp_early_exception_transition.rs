// SPDX-License-Identifier: GPL-3.0-only
//! First exception-controller transition: reset state and branch on an
//! independently read controller pointer. Stops before the MODECTL loop.
//! Hexagon instruction fields and packet markers are reconstructed from
//! typed registers, immediates, and packet-relative branch destinations.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_exception_setup;
use super::cdsp_early_supervisor_vectors;

pub const FILE_OFFSET:usize=0x4860;
pub const REGION_END:usize=0x48a4;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1860;

fn write_word(src:Gpr,base:Gpr)->u32{
    0xa180_c000 | (u32::from(base.number())<<16) | (u32::from(src.number())<<8)
}
fn read_word(dst:Gpr,base:Gpr)->u32{
    0x9180_c000 | (u32::from(base.number())<<16) | u32::from(dst.number())
}
/// Forward `if (!P0.new) jump:nt`, with a signed packet-start-relative
/// displacement represented in halfwords. Only the confirmed forward mode
/// is supported; unsupported long/backwards targets are rejected.
fn jump_if_not_new_p0_nt(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start%4!=0 || target%4!=0 || target<=packet_start {
        return Err("CDSP exception conditional branch addresses must be aligned and forward".into());
    }
    let delta=target-packet_start;
    if delta>0x7fe || delta%2!=0 {
        return Err("CDSP exception conditional branch outside checked short range".into());
    }
    let half=delta/2;
    Ok(0x5c20_c800 | ((half>>9)<<16) | (half&0x1ff))
}
fn emit(out:&mut Vec<u8>,word:u32,parse:P) {
    out.extend_from_slice(&parse.apply(word).to_le_bytes());
}

pub fn build_exception_transition()->Result<Vec<u8>,String>{
    let r=|x|Gpr::new(x);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    // Supervisor exception selector -> '3'.
    let s=cdsp_boot::encode_extended_add(r(0)?,r(23)?,-0x1fda71c,false);
    emit(&mut out,s[0],P::ContinuationA);
    emit(&mut out,s[1],P::ContinuationA);
    emit(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:3}.encode()?,P::Final);
    emit(&mut out,write_word(r(1)?,r(0)?),P::Final);
    // Walk the privileged S54-relative controller globals.
    let address=cdsp_boot::encode_extended_add(r(1)?,r(23)?,-0x1fc6e38,true);
    emit(&mut out,address[0],P::ContinuationA);
    emit(&mut out,address[1],P::Final);
    emit(&mut out,read_word(r(1)?,r(1)?),P::Final);
    let status=cdsp_boot::encode_extended_add(r(24)?,r(1)?,-0x1fdb300,true);
    emit(&mut out,status[0],P::ContinuationA);
    emit(&mut out,status[1],P::Final);
    emit(&mut out,read_word(r(24)?,r(24)?),P::Final);
    emit(&mut out,H::PredicateP0EqualsZero{src:r(24)?}.encode()?,P::ContinuationA);
    emit(&mut out,jump_if_not_new_p0_nt(0x96c0_1888,0x96c0_1924)?,P::Final);
    emit(&mut out,cdsp_early_supervisor_vectors::short_call(0x96c0_1890,0x96c0_1adc)?,P::Final);
    // Controller startup handoff -> '4'.
    let next=cdsp_boot::encode_extended_add(r(0)?,r(23)?,-0x1fda71c,false);
    emit(&mut out,next[0],P::ContinuationA);
    emit(&mut out,next[1],P::ContinuationA);
    emit(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:4}.encode()?,P::Final);
    emit(&mut out,write_word(r(1)?,r(0)?),P::Final);
    if out.len()!=FUNCTION_BYTES {return Err("CDSP transition length mismatch".into());}
    Ok(out)
}

pub fn build_supervisor_with_transition()->Result<Vec<u8>,String>{
    if cdsp_early_exception_setup::REGION_END!=FILE_OFFSET {
        return Err("CDSP setup to transition has uncovered executable bytes".into());
    }
    let mut all=cdsp_early_exception_setup::build_supervisor_with_setup()?;
    all.extend(build_exception_transition()?);
    Ok(all)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_exception_setup::verify_reference(reference)?;
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_exception_transition()?.as_slice()) {
        return Err("CDSP exception transition differs from pinned reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn transition_uses_checked_register_and_branch_fields(){
        assert_eq!(FUNCTION_BYTES,68);
        assert_eq!(build_exception_transition().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
        assert_eq!(jump_if_not_new_p0_nt(0x96c0_1888,0x96c0_1924).unwrap(),0x5c20_c84e);
        assert!(jump_if_not_new_p0_nt(0x96c0_1888,0x96c0_1888).is_err());
        assert!(jump_if_not_new_p0_nt(0x96c0_1888,0x96c0_2188).is_err());
        assert_ne!(read_word(Gpr::new(24).unwrap(),Gpr::new(24).unwrap()),
                   read_word(Gpr::new(23).unwrap(),Gpr::new(24).unwrap()));
    }
}
