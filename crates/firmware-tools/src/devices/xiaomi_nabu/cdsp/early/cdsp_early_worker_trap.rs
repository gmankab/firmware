// SPDX-License-Identifier: GPL-3.0-only
//! CDSP supervisor fatal-controller path: two extended callbacks and a
//! packetized `R2=-1; R3=-1; R0=24; R1=-1` diagnostic trap.
//! The error path terminates in a self-loop; alternate controller states
//! branch around it and start at the next file offset 0x4cd8.
use super::cdsp_boot::{GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_controllers;
use super::cdsp_early_worker_state;

pub const FILE_OFFSET:usize=0x4cb4;
pub const REGION_END:usize=0x4cd8;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1cb4;
fn put(out:&mut Vec<u8>,word:u32,p:P){out.extend_from_slice(&p.apply(word).to_le_bytes());}

/// Hexagon signed 16-bit immediate assignment of the observed `#-1` value.
/// Register destination controls the bottom five bits; independent LLVM6
/// samples confirm both R2 and R3, including packet marker independence.
fn signed_minus_one(dst:Gpr)->u32{
    0x78df_c000|(u32::from(0x1ffu16)<<5)|u32::from(dst.number())
}
/// Compact Hexagon duplex `R0 = #N; R1 = #-1`. LLVM6 V60 independently
/// assembled the immediate choices N=16,17,24,25, exposing N at bits 24:20.
fn duplex_r0_assign_and_r1_minus_one(value:u8)->Result<u32,String>{
    if !(16..=31).contains(&value){return Err("unsupported compact duplex immediate".into());}
    Ok(0x2800_3a01|(u32::from(value)<<20))
}
/// Architectural supervisor trap (vector number 0 in this routine). The
/// immediate encoding is bounded independently from the local firmware.
fn trap_zero(vector:u8)->Result<u32,String>{
    if vector!=0{return Err("only the verified zero trap vector is supported".into());}
    Ok(0x5400_c000)
}

pub fn build_worker_fatal_path()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    for (pc,target) in [(0x96c0_1cb4,0x96c3_26d0),
                        (0x96c0_1cbc,0x96c0_7a44)] {
        let x=cdsp_early_controllers::encode_extended_call(pc,target)?;
        put(&mut out,x[0],P::ContinuationA);
        put(&mut out,x[1],P::Final);
    }
    put(&mut out,signed_minus_one(r(2)?),P::ContinuationA);
    put(&mut out,signed_minus_one(r(3)?),P::ContinuationA);
    // Preserve duplex as an encoded 32-bit word, not two scalar opcodes.
    out.extend_from_slice(&duplex_r0_assign_and_r1_minus_one(24)?.to_le_bytes());
    put(&mut out,trap_zero(0)?,P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP fatal path is not exactly nine words".into());}
    Ok(out)
}
pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_worker_state::verify_reference(reference)?;
    if cdsp_early_worker_state::REGION_END!=FILE_OFFSET {
        return Err("CDSP fatal path has an unimplemented preceding gap".into());
    }
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_worker_fatal_path()?.as_slice()) {
        return Err("CDSP fatal path differs from pinned reference".into());
    }
    Ok(())
}
pub fn build_supervisor_with_fatal_path()->Result<Vec<u8>,String>{
    let mut b=cdsp_early_worker_state::build_supervisor_with_worker_state()?;
    b.extend(build_worker_fatal_path()?);
    Ok(b)
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn fatal_trap_duplex_signed_immediates_match_independent_forms(){
        assert_eq!(FUNCTION_BYTES,36);
        assert_eq!(build_worker_fatal_path().unwrap().len(),36);
        assert_eq!(signed_minus_one(Gpr::new(2).unwrap()),0x78df_ffe2);
        assert_eq!(signed_minus_one(Gpr::new(3).unwrap()),0x78df_ffe3);
        assert_eq!(duplex_r0_assign_and_r1_minus_one(24).unwrap(),0x2980_3a01);
        assert_eq!(duplex_r0_assign_and_r1_minus_one(16).unwrap(),0x2900_3a01);
        assert!(duplex_r0_assign_and_r1_minus_one(2).is_err());
        assert!(trap_zero(1).is_err());
    }
}
