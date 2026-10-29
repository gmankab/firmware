// SPDX-License-Identifier: GPL-3.0-only
//! Continuation of the CDSP codec-format bit-pattern detector. The repeated
//! tests share checked architecture encoders for shift/mask/equality/branch;
//! each original packet boundary and branch prediction bit remains explicit.
//! The surrounding function continues beyond this verified span.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P, ScalarPacket as H};
use super::cdsp_early_format_guard as previous;
use previous::{put,small_and,compare_p0_imm,p0_assign_small,p0new_jump,extended_compare_p0};

pub const FILE_OFFSET:usize=0x5538;
pub const REGION_END:usize=0x5680;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2538;

/// Detect one masked bit-field pattern. The same register sequence appears
/// for multiple codecs, with independent shift/width/format-ID values.
pub(super) fn codec_format_case(out:&mut Vec<u8>,right:u8,mask:u16,left:u8,
                     compare_to:u32,branch_start:u32,branch_target:u32,predicted_taken:bool)->Result<(),String>{
    let r=|n|Gpr::new(n);
    put(out,H::AssignSmallImmediate{dst:r(9)?,value:0}.encode()?,P::Final);
    put(out,H::AssignRegister{dst:r(11)?,src:r(0)?}.encode()?,P::Final);
    put(out,H::LogicalShiftRight{dst:r(11)?,src:r(11)?,amount:right}.encode()?,P::Final);
    if mask<=255 {
        put(out,small_and(r(11)?,r(11)?,mask)?,P::Final);
    }else if mask==2047 {
        let extender=((u32::from(mask)>>20)<<16)|((u32::from(mask)>>6)&0x3fff)|0x4000;
        put(out,extender,P::ContinuationA);
        put(out,small_and(r(11)?,r(11)?,(mask&63) as u16)?,P::Final);
    }else {return Err("unsupported format mask needs independently tested extender".into());}
    put(out,H::ArithmeticShiftLeft{dst:r(11)?,src:r(11)?,amount:left}.encode()?,P::Final);
    if compare_to==0 {
        put(out,H::PredicateP0EqualsZero{src:r(11)?}.encode()?,P::Final);
    }else {
        let pair=extended_compare_p0(r(11)?,compare_to);
        put(out,pair[0],P::ContinuationA);put(out,pair[1],P::Final);
    }
    put(out,p0_assign_small(r(9)?,1)?,P::Final);
    put(out,compare_p0_imm(r(9)?,1)?,P::ContinuationA);
    put(out,p0new_jump(branch_start,branch_target,predicted_taken,true)?,P::Final);
    Ok(())
}
pub(super) fn extended_negative_error(out:&mut Vec<u8>,value:i32,start:u32,target:u32)->Result<(),String>{
    let r=Gpr::new(0)?;
    let code=cdsp_boot::encode_extended_assignment(r,value,false);
    put(out,code[0],P::ContinuationA);
    put(out,code[1],P::ContinuationA);
    if target<=start||target-start>0x1ffe{return Err("codec error handler outside positive short jump".into());}
    put(out,H::JumpRelative{displacement_bytes:target-start}.encode()?,P::Final);
    Ok(())
}

pub fn build_format_guard_continuation()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    // Codec ID 0x5cxxxxxx, then 0x50xxxxxx.
    codec_format_case(&mut out,25,127,25,0x5c00_0000,0x96c0_2558,0x96c0_2564,false)?;
    put(&mut out,H::JumpRelative{displacement_bytes:0x80}.encode()?,P::Final);
    codec_format_case(&mut out,26,63,26,0x5000_0000,0x96c0_2584,0x96c0_25b8,false)?;
    codec_format_case(&mut out,24,1,31,0x8000_0000,0x96c0_25ac,0x96c0_25b8,false)?;
    put(&mut out,H::JumpRelative{displacement_bytes:0x2c}.encode()?,P::Final);
    codec_format_case(&mut out,24,255,24,0x6100_0000,0x96c0_25d8,0x96c0_25ec,false)?;
    extended_negative_error(&mut out,-0x1002,0x96c0_25e0,0x96c0_2878)?;
    codec_format_case(&mut out,21,2047,21,0x9600_0000,0x96c0_2610,0x96c0_2680,false)?;
    codec_format_case(&mut out,11,1,31,0x8000_0000,0x96c0_2638,0x96c0_264c,false)?;
    extended_negative_error(&mut out,-0x1021,0x96c0_2640,0x96c0_2878)?;
    codec_format_case(&mut out,11,3,30,0x8000_0000,0x96c0_266c,0x96c0_2680,false)?;
    extended_negative_error(&mut out,-0xa1,0x96c0_2674,0x96c0_2878)?;
    if out.len()!=FUNCTION_BYTES{return Err(format!("CDSP format guard continuation generated {} instead of {} bytes",out.len(),FUNCTION_BYTES));}
    Ok(out)
}
pub fn verify_reference(reference:&[u8])->Result<(),String>{
    previous::verify_reference(reference)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("format guard tail must join earlier prefix".into());}
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_format_guard_continuation()?.as_slice()){
        return Err("CDSP format guard continuation differs from pinned firmware".into());
    }
    Ok(())
}
#[cfg(test)] mod tests{
    use super::*;
    #[test] fn format_cases_and_error_targets_are_checked(){
        assert_eq!(FUNCTION_BYTES,328);
        assert_eq!(build_format_guard_continuation().unwrap().len(),FUNCTION_BYTES);
        let mut out=vec![];
        assert!(codec_format_case(&mut out,25,1023,25,0x5c00_0000,0x96c0_2558,0x96c0_2564,false).is_err());
        assert!(extended_negative_error(&mut vec![],-0x1002,0x96c0_25e0,0x96c0_25e0).is_err());
        assert!(extended_negative_error(&mut vec![],-0x1002,0x96c0_25e0,0x96c0_6000).is_err());
    }
}
