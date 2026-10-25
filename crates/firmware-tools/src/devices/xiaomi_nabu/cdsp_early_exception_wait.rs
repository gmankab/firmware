// SPDX-License-Identifier: GPL-3.0-only
//! MODECTL hardware retry loop following the exception-controller handoff.
//! All conditional branches are relative to their packet start, not the
//! address of the branch word. The loop body is source-encoded V66 packets.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_exception_transition;

pub const FILE_OFFSET:usize=0x48a4;
pub const REGION_END:usize=0x48e8;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_18a4;

/// A16 immediate assignment in the unsigned Hexagon A32 *instruction*
/// sense (12 low/mid bits plus nibble in [20:16]); LLVM6 independently
/// assembles 0x2710 and other values with the same register and immediate
/// field layout without an extender.
fn assign_unsigned_16(dst:Gpr,value:u16)->u32{
    0x7800_c000 | ((u32::from(value)>>9)<<16)
        | ((u32::from(value)&0x1ff)<<5) | u32::from(dst.number())
}
fn read_modectl(dst:Gpr)->u32{0x6e80_c000 | (17<<16) |u32::from(dst.number())}
fn xor_regs(dst:Gpr,left:Gpr,right:Gpr)->u32{
    0xf160_c000 | (u32::from(left.number())<<16)
        | (u32::from(right.number())<<8) |u32::from(dst.number())
}
fn clearbit_by_register(dst:Gpr,src:Gpr,position:Gpr)->u32{
    0xc680_c040 | (u32::from(src.number())<<16)
        | (u32::from(position.number())<<8)|u32::from(dst.number())
}
/// Compound `P0 = (Rs == #0); if (!P0.new) jump:nt` within a short
/// forward interval. This is different from the single-predicate branch.
fn zero_compare_then_skip_nt(src:Gpr,packet_start:u32,target:u32)->Result<u32,String>{
    if src.number()>7 || packet_start%4!=0 || target%4!=0 || target<=packet_start{
        return Err("CDSP zero compare branch register/alignment invalid".into());
    }
    let delta=target-packet_start;
    if delta>0x1fe || delta%2!=0{return Err("CDSP compare branch range exceeded".into());}
    Ok(0x1040_c000 | (u32::from(src.number())<<16) | (delta/2))
}
/// Negative `.new` register compare branch, fixed distance of producer in
/// current packet. Encodes a signed eight-bit halfword displacement.
fn branch_if_new_nonzero_backward(packet_start:u32,target:u32)->Result<u32,String>{
    if packet_start%4!=0 || target%4!=0 || target>=packet_start {
        return Err("CDSP new-value loop branch requires backward aligned target".into());
    }
    let delta=i64::from(target)-i64::from(packet_start);
    if delta< -256 || delta%2!=0{return Err("CDSP new-value loop range exceeded".into());}
    Ok(0x2472_e000|(((delta/2) as i32 as u32)&0xff))
}
fn store_word(src:Gpr,base:Gpr)->u32{
    0xa180_c000 | (u32::from(base.number())<<16) | (u32::from(src.number())<<8)
}
fn store_new_word(base:Gpr)->u32{
    0xa1a0_d200 | (u32::from(base.number())<<16)
}
fn put(out:&mut Vec<u8>,word:u32,parse:P){out.extend_from_slice(&parse.apply(word).to_le_bytes());}

pub fn build_exception_wait()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,0x6e80_c000 | (54<<16) |u32::from(r(23)?.number()),P::ContinuationA); // S54
    put(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:0}.encode()?,P::Final);
    let scratch=cdsp_boot::encode_extended_add(r(5)?,r(23)?,-0x1fda720,false);
    put(&mut out,scratch[0],P::ContinuationA);
    put(&mut out,scratch[1],P::ContinuationA);
    put(&mut out,assign_unsigned_16(r(4)?,10_000),P::Final);
    put(&mut out,H::AddSignedSmallImmediate{dst:r(4)?,src:r(4)?,value:-1}.encode()?,P::ContinuationA);
    put(&mut out,zero_compare_then_skip_nt(r(4)?,0x96c0_18b8,0x96c0_18c8)?,P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0x28}.encode()?,P::ContinuationA);
    put(&mut out,store_word(r(1)?,r(5)?),P::Final);
    put(&mut out,read_modectl(r(1)?),P::Final);
    put(&mut out,H::ExtractUnsigned{dst:r(2)?,src:r(1)?,width:4,offset:0}.encode()?,P::ContinuationA);
    put(&mut out,H::ExtractUnsigned{dst:r(3)?,src:r(1)?,width:4,offset:16}.encode()?,P::Final);
    put(&mut out,xor_regs(r(2)?,r(2)?,r(3)?),P::Final);
    put(&mut out,clearbit_by_register(r(2)?,r(2)?,r(0)?),P::ContinuationA);
    put(&mut out,branch_if_new_nonzero_backward(0x96c0_18d8,0x96c0_18b8)?,P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(4)?,value:0}.encode()?,P::ContinuationA);
    put(&mut out,store_new_word(r(5)?),P::Final);
    if out.len()!=FUNCTION_BYTES{return Err("CDSP MODECTL wait routine emitted incorrect length".into());}
    Ok(out)
}

pub fn build_supervisor_with_wait()->Result<Vec<u8>,String>{
    if cdsp_early_exception_transition::REGION_END!=FILE_OFFSET {
        return Err("CDSP wait loop has uncovered bytes after transition".into());
    }
    let mut all=cdsp_early_exception_transition::build_supervisor_with_transition()?;
    all.extend(build_exception_wait()?);
    Ok(all)
}
pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_exception_transition::verify_reference(original)?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_exception_wait()?.as_slice()){
        return Err("CDSP MODECTL wait loop differs from pinned reference".into());
    }
    Ok(())
}
#[cfg(test)] mod tests{
    use super::*;
    #[test]fn modectl_retry_uses_checked_branch_and_field_encoding(){
        assert_eq!(FUNCTION_BYTES,68);
        assert_eq!(build_exception_wait().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(assign_unsigned_16(Gpr::new(4).unwrap(),10_000),0x7813_e204);
        assert_eq!(zero_compare_then_skip_nt(Gpr::new(4).unwrap(),0x96c0_18b8,0x96c0_18c8).unwrap(),0x1044_c008);
        assert_eq!(branch_if_new_nonzero_backward(0x96c0_18d8,0x96c0_18b8).unwrap(),0x2472_e0f0);
        assert!(branch_if_new_nonzero_backward(0x96c0_18d8,0x96c0_18e0).is_err());
    }
}
