// SPDX-License-Identifier: GPL-3.0-only
//! Worker exception snapshot: supervisor-register sampling and cache-coherent
//! publication into the controller's absolute/GP-relative address tables.
//! Hexagon V66 privilege selectors require decoder evidence; ordinary memory
//! load/store field encodings were cross-checked with altered LLVM6 operands.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P};
use super::{cdsp_early_controllers,cdsp_early_worker_privileged_state as before,
            cdsp_early_worker_command as memory};
pub const FILE_OFFSET:usize=0x4f1c;
pub const REGION_END:usize=0x4f64;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1f1c;
fn put(out:&mut Vec<u8>,word:u32,mark:P){out.extend(mark.apply(word).to_le_bytes());}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum SystemState { Ssr, Elr, Diag, Syscfg }
fn read_privileged(dst:Gpr,reg:SystemState)->u32{
    let selector:u8=match reg{SystemState::Ssr=>6,SystemState::Elr=>3,
                              SystemState::Diag=>28,SystemState::Syscfg=>18};
    0x6e80_c000|(u32::from(selector)<<16)|u32::from(dst.number())
}
fn addr_ext(addr:u32)->u32{((addr>>20)<<16)|((addr>>6)&0x3fff)|0x4000}
/// Address form `memw(RassignBase=##address)=Rsource`.
fn absolute_store(base:Gpr,src:Gpr,address:u32)->Result<[u32;2],String>{
    if base.number()>7 || address%4!=0 {
        return Err("Hexagon absolute write base not in compact GPR set or address unaligned".into());
    }
    Ok([addr_ext(address),0xab80_c080|(u32::from(base.number())<<16)
        |(u32::from(src.number())<<8)|(address&0x3f)])
}
/// V66 memory store with GP-relative immediate, fixed four-byte alignment.
fn gp_store(src:Gpr,address:u32)->Result<[u32;2],String>{
    if address%4!=0{return Err("CDSP GP store target misaligned".into());}
    Ok([addr_ext(address),0x4880_c000|(u32::from(src.number())<<16)|(address&0x3f)])
}

pub fn build_worker_snapshot()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    let cb=cdsp_early_controllers::encode_extended_call(0x96c0_1f1c,0x96c3_26d0)?;
    put(&mut out,cb[0],P::ContinuationA);
    put(&mut out,cb[1],P::Final);
    put(&mut out,read_privileged(r(1)?,SystemState::Ssr),P::Final);
    put(&mut out,read_privileged(r(0)?,SystemState::Elr),P::ContinuationA);
    let saved_ssr=absolute_store(r(2)?,r(1)?,0xfe01_6e10)?;
    put(&mut out,saved_ssr[0],P::ContinuationA);
    put(&mut out,saved_ssr[1],P::Final);
    put(&mut out,read_privileged(r(1)?,SystemState::Diag),P::ContinuationA);
    let saved_elr=gp_store(r(0)?,0xfe01_6e14)?;
    put(&mut out,saved_elr[0],P::ContinuationA);
    put(&mut out,saved_elr[1],P::Final);
    put(&mut out,read_privileged(r(0)?,SystemState::Syscfg),P::ContinuationA);
    let saved_diag=absolute_store(r(3)?,r(1)?,0xfe03_7250)?;
    put(&mut out,saved_diag[0],P::ContinuationA);
    put(&mut out,saved_diag[1],P::Final);
    let status=cdsp_boot::encode_extended_gp_word_load(r(1)?,0xfe02_4d00);
    put(&mut out,status[0],P::ContinuationA);
    put(&mut out,status[1],P::ContinuationA);
    let saved_syscfg=gp_store(r(0)?,0xfe01_6e18)?;
    put(&mut out,saved_syscfg[0],P::ContinuationA);
    put(&mut out,saved_syscfg[1],P::Final);
    put(&mut out,memory::cache_clean(r(2)?),P::Final);
    put(&mut out,memory::cache_clean(r(3)?),P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP worker snapshot word count not 18".into());}
    Ok(out)
}
pub fn verify_reference(original:&[u8])->Result<(),String>{
    before::verify_reference(original)?;
    if before::REGION_END!=FILE_OFFSET{return Err("snapshot does not follow privilege-state write".into());}
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_worker_snapshot()?.as_slice()) {
        return Err("CDSP supervisor snapshot bytes differ from reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn snapshot_register_offsets_are_checked(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(build_worker_snapshot().unwrap().len(),72);
        assert_eq!(absolute_store(r(2),r(1),0xfe01_6e10).unwrap(),[0x0fe0_45b8,0xab82_c190]);
        assert_eq!(absolute_store(r(3),r(1),0xfe03_7250).unwrap(),[0x0fe0_4dc9,0xab83_c190]);
        assert_eq!(gp_store(r(0),0xfe01_6e14).unwrap(),[0x0fe0_45b8,0x4880_c014]);
        assert!(gp_store(r(0),0xfe01_6e15).is_err());
        assert_eq!(read_privileged(r(1),SystemState::Diag),0x6e9c_c001);
        assert_eq!(read_privileged(r(0),SystemState::Elr),0x6e83_c000);
    }
}
