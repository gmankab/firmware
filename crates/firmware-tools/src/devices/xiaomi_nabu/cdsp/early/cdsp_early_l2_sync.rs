// SPDX-License-Identifier: GPL-3.0-only
//! L2 global unlock/clean and SYSCFG wait helper. This routine begins
//! immediately after the independently emitted supervisor-context snapshot.
//! The two L2 controller opcodes are modeled as separate checked operations
//! and the P0.new backward branch is relative to its packet's first word.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H,SystemRegister};
use super::cdsp_early_context_snapshot;

pub const FILE_OFFSET:usize=0x4a08;
pub const REGION_END:usize=0x4a38;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1a08;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum GlobalL2Operation { Unlock, Clean }
impl GlobalL2Operation {
    fn encode(self)->u32 {
        // Hexagon L2 global cache control family. Operations occupy
        // bits 12:11; packet parse bits are supplied by PacketMarker.
        let opcode=match self {Self::Unlock=>1_u32,Self::Clean=>2_u32};
        0xa820_c000 | (opcode<<11)
    }
}

fn branch_while_syscfg_pending(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start%4!=0||target%4!=0||target>=packet_start{
        return Err("CDSP SYSCFG backward branch must be aligned and negative".into());
    }
    let delta=target as i64-packet_start as i64;
    if !(-256..=-4).contains(&delta)||delta%2!=0{
        return Err("CDSP SYSCFG backward branch out of P0 8-bit range".into());
    }
    // Hexagon P0.new taken-branch. The original packet begins with a
    // P0 producer at 0x4a2c and ends with branch at 0x4a30.
    Ok(0x5cdf_f800 | (((delta/2) as u32)&0xff))
}
fn put(out:&mut Vec<u8>, word:u32, marker:P){
    out.extend_from_slice(&marker.apply(word).to_le_bytes());
}

pub fn build_l2_sync()->Result<Vec<u8>,String>{
    let r=|n| Gpr::new(n);
    let mut output=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut output,H::ReadSyscfg{dst:r(0)?}.encode()?,P::Final);
    let value=cdsp_boot::encode_extended_word_load(r(1)?,r(23)?,-0x1fd9334)?;
    put(&mut output,value[0],P::ContinuationA);
    put(&mut output,value[1],P::Final);
    put(&mut output,H::InsertBitField{dst:r(0)?,src:r(1)?,width:3,offset:16}.encode()?,P::Final);
    put(&mut output,H::WriteSystem{system:SystemRegister::Syscfg,src:r(0)?}.encode()?,P::Final);
    put(&mut output,H::Isync.encode()?,P::Final);
    put(&mut output,GlobalL2Operation::Unlock.encode(),P::Final);
    put(&mut output,GlobalL2Operation::Clean.encode(),P::Final);
    put(&mut output,H::ReadSyscfg{dst:r(0)?}.encode()?,P::Final);
    put(&mut output,H::TestBitPredicate{predicate:0,src:r(0)?,bit:31}.encode()?,P::ContinuationA);
    put(&mut output,branch_while_syscfg_pending(0x96c0_1a2c,0x96c0_1a28)?,P::Final);
    put(&mut output,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if output.len()!=FUNCTION_BYTES{
        return Err(format!("CDSP L2 sync length {} != {FUNCTION_BYTES}",output.len()));
    }
    Ok(output)
}

pub fn build_supervisor_with_l2_sync()->Result<Vec<u8>,String>{
    if cdsp_early_context_snapshot::REGION_END!=FILE_OFFSET{
        return Err("CDSP supervisor and L2 sync functions have a gap".into());
    }
    let mut out=cdsp_early_context_snapshot::build_supervisor_with_snapshot()?;
    out.extend(build_l2_sync()?);
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_context_snapshot::verify_reference(original)?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_l2_sync()?.as_slice()){
        return Err("CDSP generated L2 sync differs from pinned reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn l2_unlock_clean_and_poll_are_typed(){
        let out=build_l2_sync().unwrap();
        assert_eq!(out.len(),48);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
        assert_eq!(GlobalL2Operation::Unlock.encode(),0xa820_c800);
        assert_eq!(GlobalL2Operation::Clean.encode(),0xa820_d000);
        assert_ne!(GlobalL2Operation::Clean.encode(),GlobalL2Operation::Unlock.encode());
        assert_eq!(branch_while_syscfg_pending(0x96c0_1a2c,0x96c0_1a28).unwrap(),0x5cdf_f8fe);
        assert!(branch_while_syscfg_pending(0x96c0_1a2c,0x96c0_1a2c).is_err());
        assert!(branch_while_syscfg_pending(0x96c0_1a2c,0x96c0_1a2e).is_err());
        assert!(branch_while_syscfg_pending(0x96c0_1a2c,0x96c0_162c).is_err());
    }
}
