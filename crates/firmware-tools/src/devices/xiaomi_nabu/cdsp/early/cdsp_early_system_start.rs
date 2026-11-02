// SPDX-License-Identifier: GPL-3.0-only
//! Reconstructed CDSP supervisor system-control start. This disjoint entry
//! follows 232 bytes of intentional ELF linker zero alignment (0x4418..4500),
//! NOT source-authored instructions. Branches and privileged accesses after
//! 0x4530 remain uncovered until later independent checkpoints.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H,SystemRegister};
use super::cdsp_early_cfgbase;

pub const FILE_OFFSET: usize=0x4500;
pub const FUNCTION_SIZE:usize=0x68;
pub const REGION_END:usize=FILE_OFFSET+FUNCTION_SIZE;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1500;

/// The system destination selector is an architectural privilege field.
/// `PMUCFG` uses selector 0x35 and can only be written from an integer GPR.
fn write_pmucfg(src:Gpr)->u32 { 0x6700_c035 | (u32::from(src.number())<<16) }

/// Register S54 controls scratch controller initialization. Do not generalize
/// this check to other system fields without decoding their architecture.
fn read_s54(dst:Gpr)->u32 { 0x6e80_c000 | (54<<16) |u32::from(dst.number()) }

fn put(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}
/// Limited V66 P0.new predicted-not-taken branch. The immediate is split
/// across bits 7:0 and 18:15 rather than contiguous within the word. The
/// displacement is packet-start relative, not relative to the branch word.
fn skip_if_new_p0_nt(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start%4!=0||target%4!=0||target<=packet_start{
        return Err("Hexagon P0.new forward jump requires aligned increasing addresses".into());
    }
    let span=target-packet_start;
    if span>0x7fc{return Err("P0.new branch exceeds checked 0x7fc-byte range".into());}
    let half=span/2;
    Ok(0x5c00_c800 | (half&0xff) | ((half>>8)<<15))
}
/// One-register basic word store; source and address are independent GPRs.
fn store_word(src:Gpr,address:Gpr)->u32{
    0xa180_c000 | (u32::from(address.number())<<16) | (u32::from(src.number())<<8)
}
/// Exact V66 new-value store; the new producer is the preceding member of
/// the same packet and the base remains a general-purpose register.
fn store_new_word_r24(address:Gpr)->u32{
    0xa1a0_d200 | (u32::from(address.number())<<16)
}

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
    // Compare against a forced extended immediate, and skip the controller
    // initialization when it is already configured for this revision.
    put(&mut out,0x4000,P::ContinuationA);
    put(&mut out,0x7500_c000 | (37u32<<5),P::ContinuationA);
    put(&mut out,skip_if_new_p0_nt(0x96c0_1530,0x96c0_1ad4)?,P::Final);
    let controller=cdsp_boot::encode_extended_add(r(0)?,r(23)?,-0x1fda71c,false);
    put(&mut out,controller[0],P::ContinuationA);
    put(&mut out,controller[1],P::ContinuationA);
    put(&mut out,store_word(r(24)?,r(25)?),P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(24)?,value:1}.encode()?,P::ContinuationA);
    put(&mut out,store_new_word_r24(r(0)?),P::Final);
    let reset=cdsp_boot::encode_extended_add(r(0)?,r(23)?,-0x1fdabe0,true);
    put(&mut out,reset[0],P::ContinuationA);
    put(&mut out,reset[1],P::Final);
    put(&mut out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:0}.encode()?,P::Final);
    put(&mut out,0x9180_c000 |u32::from(r(24)?.number()),P::Final);
    put(&mut out,H::SetBit{dst:r(24)?,src:r(24)?,position:29}.encode()?,P::ContinuationA);
    put(&mut out,store_new_word_r24(r(0)?),P::Final);
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
        assert_eq!(FUNCTION_SIZE,104);
        assert_eq!(build_system_start().unwrap().len(),104);
        assert_eq!(PHYSICAL_ADDRESS,0x96c00000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test]fn conditional_extended_branch_encodes_packet_relative(){
        assert_eq!(skip_if_new_p0_nt(0x96c0_1530,0x96c0_1ad4).unwrap(),0x5c01_c8d2);
        assert!(skip_if_new_p0_nt(0x96c0_1530,0x96c0_1600).is_ok());
        assert!(skip_if_new_p0_nt(0x96c0_1530,0x96c0_1530).is_err());
        assert!(skip_if_new_p0_nt(0x96c0_1531,0x96c0_1ad4).is_err());
    }
    #[test]fn system_register_encodings_change_with_operands(){
        assert_eq!(write_pmucfg(Gpr::new(23).unwrap()),0x6717_c035);
        assert_eq!(write_pmucfg(Gpr::new(4).unwrap()),0x6704_c035);
        assert_eq!(read_s54(Gpr::new(23).unwrap()),0x6eb6_c017);
        assert_eq!(read_s54(Gpr::new(7).unwrap()),0x6eb6_c007);
    }
}
