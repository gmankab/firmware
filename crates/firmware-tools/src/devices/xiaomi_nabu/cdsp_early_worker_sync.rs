// SPDX-License-Identifier: GPL-3.0-only
//! CDSP worker post-guard controller synchronization and status publication.
//! Typed V66 encodings express GP/global pointers, thread-local MODECTL,
//! cache clean/invalidate, a completion callback and a ready-state return.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_worker_guard_exit as previous;
use super::cdsp_early_worker_command as memory;

pub const FILE_OFFSET:usize=0x5004;
pub const REGION_END:usize=0x5060;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2004;
fn put(out:&mut Vec<u8>,word:u32,mark:P){out.extend_from_slice(&mark.apply(word).to_le_bytes());}

/// Publish 31 into an absolute controller word. The original packet uses an
/// explicit immediate extender, retained as a typed zero extension.
fn write_constant_word(base:Gpr,value:u8)->Result<[u32;2],String>{
    if base.number()>7 || value>63 {return Err("V66 immediate store operand outside compact encoding".into());}
    Ok([0x4000,0x3c40_c000|(u32::from(base.number())<<16)|u32::from(value)])
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum SpecialRegister { HardwareThreadId, ModeCtl }
fn read_special(dst:Gpr,which:SpecialRegister)->u32{
    let selector=match which {SpecialRegister::HardwareThreadId=>8u32,SpecialRegister::ModeCtl=>17};
    0x6e80_c000|(selector<<16)|u32::from(dst.number())
}
fn multiply_immediate_add(dst:Gpr,src:Gpr,factor:u8)->Result<u32,String>{
    if factor>31{return Err("CDSP V66 MPYI factor must be a five-bit value".into());}
    Ok(0xe100_c000 | (u32::from(src.number())<<16)
       |(u32::from(factor)<<5)|u32::from(dst.number()))
}
fn store_new_reg(base:Gpr,producer:Gpr)->Result<u32,String>{
    if base.number()>7||producer.number()!=1 {return Err("CDSP mode store requires R1 new-value producer and compact base".into());}
    Ok(0xa1a0_d200|(u32::from(base.number())<<16))
}
fn cache_invalidate(base:Gpr)->u32{0xa020_c000|(u32::from(base.number())<<16)}
fn load_word(dst:Gpr,base:Gpr)->u32{0x9180_c000|(u32::from(base.number())<<16)|u32::from(dst.number())}

pub fn build_worker_sync()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    // Record status #31, then publish current thread MODECTL into the slot
    // and maintain controller cache coherency before the observer callback.
    let pointer=cdsp_boot::encode_extended_assignment(r(1)?,0xfe00_0044_u32 as i32,true);
    put(&mut out,pointer[0],P::ContinuationA);
    put(&mut out,pointer[1],P::Final);
    let status=write_constant_word(r(1)?,31)?;
    put(&mut out,status[0],P::ContinuationA);
    put(&mut out,status[1],P::Final);
    put(&mut out,memory::cache_clean(r(1)?),P::Final);
    put(&mut out,read_special(r(1)?,SpecialRegister::HardwareThreadId),P::ContinuationA);
    let slot=cdsp_boot::encode_extended_assignment(r(0)?,0xfe01_6e00_u32 as i32,true);
    put(&mut out,slot[0],P::ContinuationA);
    put(&mut out,slot[1],P::Final);
    put(&mut out,multiply_immediate_add(r(0)?,r(1)?,4)?,P::Final);
    put(&mut out,read_special(r(1)?,SpecialRegister::ModeCtl),P::ContinuationA);
    put(&mut out,store_new_reg(r(0)?,r(1)?)?,P::Final);
    put(&mut out,memory::cache_clean(r(0)?),P::Final);
    put(&mut out,cache_invalidate(r(0)?),P::Final);
    put(&mut out,load_word(r(1)?,r(0)?),P::Final);
    put(&mut out,memory::direct_call(0x96c0_203c,0x96c3_736c)?,P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0}.encode()?,P::Final);
    if out.len()!=0x40{return Err("CDSP MODECTL sync length not 64 bytes".into());}

    // Separate singleton callback publishes status #1 before returning via
    // the original LR. The producer-consumer write is a three-word packet.
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:1}.encode()?,P::ContinuationA);
    let updated=memory::publish_new_state(r(1)?,r(0)?,0xfe02_4d00)?;
    put(&mut out,updated[0],P::ContinuationA);
    put(&mut out,updated[1],P::Final);
    put(&mut out,memory::cache_clean(r(1)?),P::Final);
    put(&mut out,cache_invalidate(r(1)?),P::Final);
    put(&mut out,load_word(r(0)?,r(1)?),P::Final);
    put(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP worker sync/ready helper total length wrong".into());}
    Ok(out)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    previous::verify_reference(reference)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("CDSP controller sync must follow guard exit".into());}
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_worker_sync()?.as_slice()){
        return Err("CDSP controller sync/status not reference-identical".into());
    }
    Ok(())
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn mode_controller_slot_address_and_status_are_bounded(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,92);
        assert_eq!(build_worker_sync().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(write_constant_word(r(1),31).unwrap(),[0x4000,0x3c41_c01f]);
        assert_eq!(read_special(r(1),SpecialRegister::ModeCtl),0x6e91_c001);
        assert_eq!(multiply_immediate_add(r(0),r(1),4).unwrap(),0xe101_c080);
        assert_eq!(store_new_reg(r(0),r(1)).unwrap(),0xa1a0_d200);
        assert!(write_constant_word(r(8),31).is_err());
        assert!(write_constant_word(r(1),64).is_err());
        assert!(store_new_reg(r(0),r(2)).is_err());
        assert!(multiply_immediate_add(r(0),r(1),32).is_err());
    }
}
