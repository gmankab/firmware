// SPDX-License-Identifier: GPL-3.0-only
//! Reconstructed CDSP supervisor system-control start. This disjoint entry
//! follows 232 bytes of intentional ELF linker zero alignment (0x4418..4500),
//! NOT source-authored instructions. Branches and privileged accesses after
//! 0x4530 remain uncovered until later independent checkpoints.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H,SystemRegister};
use super::cdsp_early_cfgbase;

pub const FILE_OFFSET: usize=0x4500;
pub const FUNCTION_SIZE:usize=0x30;
pub const REGION_END:usize=FILE_OFFSET+FUNCTION_SIZE;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1500;

/// The system destination selector is an architectural privilege field.
/// `PMUCFG` uses selector 0x35 and can only be written from an integer GPR.
fn write_pmucfg(src:Gpr)->u32 { 0x6700_c035 | (u32::from(src.number())<<16) }

/// Register S54 controls scratch controller initialization. Do not generalize
/// this check to other system fields without decoding their architecture.
fn read_s54(dst:Gpr)->u32 { 0x6e80_c000 | (54<<16) |u32::from(dst.number()) }

fn put(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}

pub fn build_system_start()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_SIZE);
    put(&mut out,H::ReadSyscfg{dst:r(23)?}.encode()?,P::Final);
    put(&mut out,write_pmucfg(r(23)?),P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:40}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:SystemRegister::Syscfg,src:r(0)?}.encode()?,P::Final);
    put(&mut out,H::Isync.encode()?,P::Final);
    put(&mut out,read_s54(r(23)?),P::Final);
    let relocated=cdsp_boot::encode_extended_add(r(0)?,r(23)?,-0x1fdabe0,true);
    put(&mut out,relocated[0],P::ContinuationA);
    put(&mut out,relocated[1],P::Final);
    put(&mut out,H::AddSmallImmediate{dst:r(25)?,src:r(0)?,value:4}.encode()?,P::ContinuationA);
    let initial=cdsp_boot::encode_extended_assignment(r(24)?,37,true);
    put(&mut out,initial[0],P::ContinuationA);
    put(&mut out,initial[1],P::Final);
    // R0 = memw(R25 + #0): indexed form separately corroborated with LLVM.
    put(&mut out,0x9180_c000 | (u32::from(r(25)?.number())<<16)|u32::from(r(0)?.number()),P::Final);
    if out.len()!=FUNCTION_SIZE {return Err("CDSP supervisor-start length mismatch".into());}
    Ok(out)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_cfgbase::verify_reference(reference)?;
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_system_start()?.as_slice()){
        return Err("CDSP supervisor entry does not match the pinned reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn system_start_after_alignment_is_independently_encoded(){
        assert_eq!(FILE_OFFSET,cdsp_early_cfgbase::ZERO_FILL_END);
        assert_eq!(FUNCTION_SIZE,48);
        assert_eq!(build_system_start().unwrap().len(),48);
        assert_eq!(PHYSICAL_ADDRESS,0x96c00000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test]fn system_register_encodings_change_with_operands(){
        assert_eq!(write_pmucfg(Gpr::new(23).unwrap()),0x6717_c035);
        assert_eq!(write_pmucfg(Gpr::new(4).unwrap()),0x6704_c035);
        assert_eq!(read_s54(Gpr::new(23).unwrap()),0x6eb6_c017);
        assert_eq!(read_s54(Gpr::new(7).unwrap()),0x6eb6_c007);
    }
}
