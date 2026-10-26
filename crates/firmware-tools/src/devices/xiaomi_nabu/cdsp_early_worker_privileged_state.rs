// SPDX-License-Identifier: GPL-3.0-only
//! CDSP worker privilege-state snapshot, MODECTL cache synchronization,
//! SYSCFG/SSR reset, and final controller status publish. Privileged V66
//! transfer selectors are encoded from checked register choices and corroborated
//! by the Hexagon decoder, not claimed independently LLVM6 assembled.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H,SystemRegister};
use super::{cdsp_early_worker_command as op,cdsp_early_worker_callbacks as before};
pub const FILE_OFFSET:usize=0x4ec8;
pub const REGION_END:usize=0x4f1c;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1ec8;
fn put(o:&mut Vec<u8>,word:u32,mark:P){o.extend(mark.apply(word).to_le_bytes());}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum PrivilegedRegister { HardwareThreadId, ModeControl, SystemConfig, Status }
fn read_privileged(dst:Gpr,reg:PrivilegedRegister)->u32{
    let selector:u8=match reg {
        PrivilegedRegister::HardwareThreadId=>8,
        PrivilegedRegister::ModeControl=>17,
        PrivilegedRegister::SystemConfig=>18,
        PrivilegedRegister::Status=>6,
    };
    0x6e80_c000|(u32::from(selector)<<16)|u32::from(dst.number())
}
/// `Rd+=mpyi(Rsrc,#count)` for the checked positive small multiplier range.
fn multiply_accumulate(dst:Gpr,src:Gpr,count:u8)->Result<u32,String>{
    if count>31 {return Err("CDSP V66 immediate multiplication limited to five bits".into());}
    Ok(0xe100_c000|(u32::from(src.number())<<16)|(u32::from(count)<<5)|u32::from(dst.number()))
}
/// V66 new-value memory write with producer R1 and base R0 in same packet.
fn store_new_mode_control(dst_base:Gpr,producer:Gpr)->Result<u32,String>{
    if producer.number()!=1||dst_base.number()>7 {
        return Err("MODECTL producer/store base requires V66 checked R1/R0 form".into());
    }
    Ok(0xa1a0_d200|(u32::from(dst_base.number())<<16))
}
fn cache_invalidate(base:Gpr)->u32{0xa020_c000|(u32::from(base.number())<<16)}
fn read_word(dst:Gpr,base:Gpr)->u32{0x9180_c000|(u32::from(base.number())<<16)|u32::from(dst.number())}

pub fn build_privileged_worker_state()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,read_privileged(r(1)?,PrivilegedRegister::HardwareThreadId),P::ContinuationA);
    let thread=cdsp_boot::encode_extended_assignment(r(0)?,-0x01fe_9200,true);
    put(&mut out,thread[0],P::ContinuationA);
    put(&mut out,thread[1],P::Final);
    put(&mut out,multiply_accumulate(r(0)?,r(1)?,4)?,P::Final);
    put(&mut out,read_privileged(r(1)?,PrivilegedRegister::ModeControl),P::ContinuationA);
    put(&mut out,store_new_mode_control(r(0)?,r(1)?)?,P::Final);
    put(&mut out,op::cache_clean(r(0)?),P::Final);
    put(&mut out,cache_invalidate(r(0)?),P::Final);
    put(&mut out,read_word(r(1)?,r(0)?),P::Final);
    put(&mut out,read_privileged(r(0)?,PrivilegedRegister::SystemConfig),P::Final);
    put(&mut out,H::ClearBit{dst:r(0)?,src:r(0)?,position:4}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:SystemRegister::Syscfg,src:r(0)?}.encode()?,P::Final);
    put(&mut out,read_privileged(r(0)?,PrivilegedRegister::Status),P::ContinuationA);
    put(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:0}.encode()?,P::Final);
    put(&mut out,H::InsertBitField{dst:r(0)?,src:r(1)?,width:3,offset:16}.encode()?,P::Final);
    put(&mut out,H::WriteSystem{system:SystemRegister::Ssr,src:r(0)?}.encode()?,P::Final);
    let value=cdsp_boot::encode_extended_assignment(r(0)?,36,false);
    put(&mut out,value[0],P::ContinuationA);
    put(&mut out,value[1],P::ContinuationA);
    let write=op::publish_new_state(r(1)?,r(0)?,0xfe00_0044)?;
    put(&mut out,write[0],P::ContinuationA);
    put(&mut out,write[1],P::Final);
    put(&mut out,op::cache_clean(r(1)?),P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP privileged worker instruction count mismatch".into());}
    Ok(out)
}
pub fn verify_reference(ref_image:&[u8])->Result<(),String>{
    before::verify_reference(ref_image)?;
    if before::REGION_END!=FILE_OFFSET{return Err("privileged worker state not adjacent to callback burst".into());}
    if ref_image.get(FILE_OFFSET..REGION_END)!=Some(build_privileged_worker_state()?.as_slice()) {
        return Err("CDSP privileged worker state differs from pinned reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn system_register_selector_and_state_are_bounded(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,84);
        assert_eq!(build_privileged_worker_state().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(read_privileged(r(1),PrivilegedRegister::HardwareThreadId),0x6e88_c001);
        assert_eq!(read_privileged(r(1),PrivilegedRegister::ModeControl),0x6e91_c001);
        assert_eq!(read_privileged(r(0),PrivilegedRegister::Status),0x6e86_c000);
        assert_eq!(multiply_accumulate(r(0),r(1),4).unwrap(),0xe101_c080);
        assert!(multiply_accumulate(r(0),r(1),32).is_err());
        assert!(store_new_mode_control(r(0),r(2)).is_err());
    }
}
