// SPDX-License-Identifier: GPL-3.0-only
//! Restore the supervisor context after the worker handoff. This 140-byte
//! fragment consists of 35 V66 instruction words, preserving packet markers,
//! privileged system-register selectors and the original callback addresses.
//! The OEM executable is not copied into source arrays; the assembled bytes
//! are derived from narrow checked operand encoders.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P, ScalarPacket as H};
use super::{cdsp_early_worker_handoff as previous, cdsp_early_worker_command as op};

pub const FILE_OFFSET: usize = 0x50f8;
pub const REGION_END: usize = 0x5184;
pub const PHYSICAL_ADDRESS: u32 = 0x96c0_20f8;
pub const FUNCTION_BYTES: usize = REGION_END - FILE_OFFSET;

fn put(out: &mut Vec<u8>, opcode: u32, marker: P) {
    out.extend_from_slice(&marker.apply(opcode).to_le_bytes());
}

/// A 16-byte, 8-byte-aligned stack allocation.
fn allocframe(bytes: u16) -> Result<u32,String> {
    if bytes == 0 || bytes > 0x1ff8 || bytes % 8 != 0 {
        return Err("V66 allocframe requires nonzero, 8-byte-aligned size <= 8184".into());
    }
    Ok(0xa09d_c000 | u32::from(bytes/8))
}
/// `memd(Rbase+#offset) = Rodd:Reven` with compact indexed displacement.
fn store_pair(low: Gpr, base: Gpr, offset: u16) -> Result<u32,String> {
    if low.number()%2 != 0 || offset%8 != 0 || offset>0x3f8 {
        return Err("Hexagon MEMD store requires even register pair and 8-byte offset".into());
    }
    Ok(0xa1c0_c000 | (u32::from(base.number())<<16)
        | (u32::from(low.number())<<8) | u32::from(offset/8))
}
/// `Rodd:Reven = memd(Rbase+#offset)`, with the offset encoded in steps of 8.
fn load_pair(low: Gpr, base: Gpr, offset: u16) -> Result<u32,String> {
    if low.number()%2!=0 || offset%8!=0 || offset>0x3f8 {
        return Err("Hexagon MEMD load requires even register pair and 8-byte offset".into());
    }
    Ok(0x91c0_c000 | (u32::from(base.number())<<16)
       | (u32::from(offset/8)<<5) | u32::from(low.number()))
}
/// One ordinary word store/load at positive word-aligned base displacement.
fn word_access(load: bool, item: Gpr, base: Gpr, offset: u16) -> Result<u32,String> {
    if offset%4 != 0 || offset>0x3fc {return Err("CDSP word offset unaligned/out of range".into());}
    let family=if load {0x9180_c000} else {0xa180_c000};
    Ok(family | (u32::from(base.number())<<16)
       | if load {u32::from(item.number()) | (u32::from(offset/4)<<5)}
         else {(u32::from(item.number())<<8) | u32::from(offset/4)})
}
/// One V66 privileged general-to-system move; only identified register IDs.
#[derive(Clone,Copy,Debug)]
enum SpecialRegister { Sgp0, Htid, Elr }
fn transfer_special(dst_or_src:Gpr, reg:SpecialRegister, read:bool)->Result<u32,String> {
    let selector=match reg {SpecialRegister::Sgp0=>0, SpecialRegister::Htid=>8,
                            SpecialRegister::Elr=>3};
    if read {
        if matches!(reg, SpecialRegister::Elr) {return Err("ELR read not identified in this function".into());}
        Ok(0x6e80_c000|(selector<<16)|u32::from(dst_or_src.number()))
    } else {
        if !matches!(reg, SpecialRegister::Elr) {return Err("unverified system-register write".into());}
        Ok(0x6700_c000 | (u32::from(dst_or_src.number())<<16)|selector)
    }
}
/// The verified one-word compound: `P0=(R0==#0); R1=memw(R1+#0)`.
/// The R1 register pair is encoded in separate compact 4-bit fields.
fn guarded_load_if_zero(test:Gpr, dst:Gpr, base:Gpr) -> Result<u32,String> {
    if test.number()!=0 || dst.number()>7 || base.number()>7 {
        return Err("only observed R0 zero test and compact memory registers supported".into());
    }
    Ok(0x5900_0000 | (u32::from(base.number())<<4) | u32::from(dst.number()))
}
/// Checked Hexagon conditional `if(P0) call target` with forced extender.
/// PC-relative offset is measured from the *start of the entire packet*,
/// even when a non-extender arithmetic producer begins the packet.
fn conditional_extended_call(packet_start:u32,target:u32)->Result<[u32;2],String>{
    if packet_start%4!=0 || target%4!=0 {return Err("unwordaligned conditional extended call".into());}
    let delta=i64::from(target)-i64::from(packet_start);
    if delta<i64::from(i32::MIN) || delta>i64::from(i32::MAX) {
        return Err("conditional extended call outside 32-bit signed distance".into());
    }
    let bits=delta as i32 as u32;
    Ok([((bits>>20)<<16)|((bits>>6)&0x3fff)|0x4000,
        0x5d00_c000|((bits&0x3f)<<1)])
}
/// `Rdst += mpyi(Rsrc,#positive8bit)`; encoded in the unsigned immediate
/// field and restricted to the checked V66 range.
fn add_mpyi(dst:Gpr,src:Gpr,factor:u8)->u32 {
    0xe100_c000 | (u32::from(src.number())<<16)
        | (u32::from(factor)<<5) | u32::from(dst.number())
}
/// One compound `P0=(R0==0); if(!P0.new) jump:nt target`.
fn branch_if_not_new_zero(src:Gpr,packet:u32,target:u32)->Result<u32,String>{
    if src.number()!=0 || packet%4!=0 || target%4!=0 || target<=packet || target-packet>0x1fe {
        return Err("V66 compound zero/branch bounded forward interval only".into());
    }
    Ok(0x1040_c000|((target-packet)/2))
}

pub fn build_worker_recovery()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,allocframe(16)?,P::Final);
    put(&mut out,store_pair(r(18)?,r(29)?,8)?,P::Final);
    put(&mut out,op::direct_call(0x96c0_2100,0x96c3_2430)?,P::ContinuationA);
    put(&mut out,H::SetBit{dst:r(1)?,src:r(1)?,position:29}.encode()?,P::ContinuationA);
    put(&mut out,H::AssignRegister{dst:r(28)?,src:r(0)?}.encode()?,P::Final);
    put(&mut out,word_access(false,r(0)?,r(29)?,0)?,P::Final);
    put(&mut out,transfer_special(r(1)?,SpecialRegister::Sgp0,true)?,P::Final);
    put(&mut out,H::AssignRegister{dst:r(0)?,src:r(28)?}.encode()?,P::ContinuationA);
    put(&mut out,guarded_load_if_zero(r(0)?,r(1)?,r(1)?)?,P::ContinuationB);
    let skip=conditional_extended_call(0x96c0_211c,0x96c3_2464)?;
    put(&mut out,skip[0],P::ContinuationA);
    put(&mut out,skip[1],P::ContinuationA);
    put(&mut out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:8}.encode()?,P::Final);
    put(&mut out,transfer_special(r(9)?,SpecialRegister::Sgp0,true)?,P::Final);
    put(&mut out,word_access(true,r(2)?,r(9)?,0)?,P::Final);
    put(&mut out,load_pair(r(4)?,r(2)?,24)?,P::ContinuationA);
    put(&mut out,load_pair(r(6)?,r(2)?,32)?,P::Final);
    put(&mut out,transfer_special(r(4)?,SpecialRegister::Elr,false)?,P::Final);
    put(&mut out,load_pair(r(6)?,r(2)?,40)?,P::ContinuationA);
    put(&mut out,store_pair(r(6)?,r(9)?,80)?,P::Final);
    let assign=cdsp_boot::encode_extended_assignment(r(0)?,-0x01f5_4408,false);
    put(&mut out,assign[0],P::ContinuationA);
    put(&mut out,assign[1],P::ContinuationA);
    put(&mut out,store_pair(r(6)?,r(9)?,72)?,P::Final);
    put(&mut out,transfer_special(r(3)?,SpecialRegister::Htid,true)?,P::ContinuationA);
    put(&mut out,H::AddSmallImmediate{dst:r(1)?,src:r(0)?,value:104}.encode()?,P::ContinuationA);
    put(&mut out,H::AssignRegister{dst:r(0)?,src:r(9)?}.encode()?,P::Final);
    put(&mut out,add_mpyi(r(1)?,r(3)?,224),P::ContinuationA);
    let cb1=conditional_extended_call(0x96c0_215c,0x96c3_251c)?;
    put(&mut out,cb1[0],P::ContinuationA);
    put(&mut out,cb1[1],P::Final);
    let cb2=conditional_extended_call(0x96c0_2168,0x96c3_736c)?;
    put(&mut out,cb2[0],P::ContinuationA);
    put(&mut out,cb2[1],P::Final);
    put(&mut out,word_access(true,r(0)?,r(29)?,0)?,P::Final);
    put(&mut out,branch_if_not_new_zero(r(0)?,0x96c0_2174,0x96c0_2180)?,P::Final);
    put(&mut out,load_pair(r(18)?,r(29)?,8)?,P::Final);
    // Architectural `LR:FP=dealloc_return(FP)` is an unparameterized ABI
    // operation; independently assembled as a singleton using LLVM6.
    put(&mut out,0x961e_c01e,P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES {return Err(format!("CDSP recovery emitted {} bytes, expected {}",out.len(),FUNCTION_BYTES));}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    previous::verify_reference(original)?;
    if previous::REGION_END!=FILE_OFFSET {return Err("CDSP worker recovery not contiguous after worker handoff".into());}
    let ours=build_worker_recovery()?;
    let theirs=original.get(FILE_OFFSET..REGION_END).ok_or("CDSP reference truncated in recovery")?;
    if let Some((idx,(a,b)))=ours.iter().zip(theirs).enumerate().find(|(_, (a,b))|a!=b) {
        return Err(format!("CDSP worker recovery mismatch at file {:#x}: generated {a:02x} vs reference {b:02x}",FILE_OFFSET+idx));
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test]
    fn save_restore_and_packet_offsets_are_checked(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,140);
        assert_eq!(build_worker_recovery().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(allocframe(16).unwrap(),0xa09d_c002);
        assert!(allocframe(9).is_err());
        assert_eq!(store_pair(r(18),r(29),8).unwrap(),0xa1dd_d201);
        assert_eq!(load_pair(r(4),r(2),24).unwrap(),0x91c2_c064);
        assert!(load_pair(r(3),r(2),24).is_err());
        assert_eq!(guarded_load_if_zero(r(0),r(1),r(1)).unwrap(),0x5900_0011);
        assert!(guarded_load_if_zero(r(2),r(1),r(1)).is_err());
        assert_eq!(conditional_extended_call(0x96c0_211c,0x96c3_2464).unwrap(),[0x0000_4c0d,0x5d00_c010]);
        assert_eq!(conditional_extended_call(0x96c0_215c,0x96c3_251c).unwrap(),[0x0000_4c0f,0x5d00_c000]);
        assert_eq!(conditional_extended_call(0x96c0_2168,0x96c3_736c).unwrap(),[0x0000_4d48,0x5d00_c008]);
        assert!(conditional_extended_call(0x96c0_211d,0x96c3_2464).is_err());
        assert_eq!(add_mpyi(r(1),r(3),224),0xe103_dc01);
        assert_eq!(branch_if_not_new_zero(r(0),0x96c0_2174,0x96c0_2180).unwrap(),0x1040_c006);
    }
}
