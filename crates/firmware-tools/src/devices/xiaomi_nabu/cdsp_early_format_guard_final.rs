// SPDX-License-Identifier: GPL-3.0-only
//! Final 128 Hexagon instructions of the format-classification routine.
//! Each predicate test selects one documented bit-pattern of a codec format
//! mask, with signed error IDs and the original deallocframe + return.
//! Operations are generated from checked typed V66 instruction fields.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P, ScalarPacket as H};
use super::cdsp_early_format_guard_tail as previous;
use super::cdsp_early_format_guard::put;
use previous::{codec_format_case,extended_negative_error};

pub const FILE_OFFSET:usize=0x5680;
pub const REGION_END:usize=0x5880;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2680;

fn jump(out:&mut Vec<u8>,start:u32,target:u32)->Result<(),String>{
    if start%4!=0||target%4!=0||target<=start||target-start>0x1ffe {
        return Err("codec short jump target outside supported range".into());
    }
    put(out,H::JumpRelative{displacement_bytes:target-start}.encode()?,P::Final);
    Ok(())
}

/// Recognize codec format combinations through repeated, independently
/// validated shift+mask compare and .new predicate-branch packets, then
/// return a signed controller status at one common epilogue.
pub fn build_format_guard_final()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    codec_format_case(&mut out,29,7,29,0,0x96c0_269c,0x96c0_2720,false)?;
    codec_format_case(&mut out,8,255,24,0x3f00_0000,0x96c0_26c4,0x96c0_2720,false)?;
    codec_format_case(&mut out,6,1,31,0x8000_0000,0x96c0_26ec,0x96c0_2720,false)?;
    codec_format_case(&mut out,2,1,31,0x8000_0000,0x96c0_2714,0x96c0_2720,false)?;
    jump(&mut out,0x96c0_271c,0x96c0_2864)?;

    codec_format_case(&mut out,29,7,29,0x2000_0000,0x96c0_2740,0x96c0_27c4,false)?;
    codec_format_case(&mut out,8,255,24,0x1f00_0000,0x96c0_2768,0x96c0_27c4,false)?;
    codec_format_case(&mut out,6,1,31,0x8000_0000,0x96c0_2790,0x96c0_27c4,false)?;
    codec_format_case(&mut out,2,1,31,0x8000_0000,0x96c0_27b8,0x96c0_27c4,true)?;
    jump(&mut out,0x96c0_27c0,0x96c0_2864)?;

    codec_format_case(&mut out,29,7,29,0x4000_0000,0x96c0_27e4,0x96c0_2870,false)?;
    codec_format_case(&mut out,8,255,24,0x3f00_0000,0x96c0_280c,0x96c0_2870,false)?;
    codec_format_case(&mut out,6,1,31,0x8000_0000,0x96c0_2834,0x96c0_2870,false)?;
    codec_format_case(&mut out,2,1,31,0x8000_0000,0x96c0_285c,0x96c0_2870,true)?;
    extended_negative_error(&mut out,-25,0x96c0_2864,0x96c0_2878)?;
    let r=Gpr::new(0)?;
    let error=cdsp_boot::encode_extended_assignment(r,-1,true);
    put(&mut out,error[0],P::ContinuationA);put(&mut out,error[1],P::Final);
    // Independently assembled Hexagon6 `deallocframe` followed by `jumpr lr`.
    put(&mut out,0x901e_c01e,P::Final);
    put(&mut out,H::JumpRegister{source:Gpr::new(31)?}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES {return Err(format!("CDSP format classifier final generated {} instead of {} bytes",out.len(),FUNCTION_BYTES));}
    Ok(out)
}

/// Assemble the complete guarded codec-format classifier from its three
/// consecutive independently reconstructed typed instruction fragments.
pub fn build_full_format_classifier()->Result<Vec<u8>,String>{
    use super::cdsp_early_format_guard as first;
    if first::REGION_END!=previous::FILE_OFFSET || previous::REGION_END!=FILE_OFFSET {
        return Err("CDSP format classifier fragments do not form a contiguous range".into());
    }
    let mut result=first::build_format_guard_prefix()?;
    result.extend(previous::build_format_guard_continuation()?);
    result.extend(build_format_guard_final()?);
    if result.len()!=REGION_END-first::FILE_OFFSET {
        return Err("CDSP classifier generated a truncated combined region".into());
    }
    Ok(result)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    previous::verify_reference(reference)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("CDSP format guard final not adjacent to tail".into());}
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_format_guard_final()?.as_slice()){
        return Err("CDSP codec format classifier final differs from pinned reference".into());
    }
    if reference.get(super::cdsp_early_format_guard::FILE_OFFSET..REGION_END)
        !=Some(build_full_format_classifier()?.as_slice()) {
        return Err("Joined 1016-byte codec format classifier differs from pinned reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_format_classifier_completes_original_return(){
        assert_eq!(FUNCTION_BYTES,512);
        assert_eq!(build_full_format_classifier().unwrap().len(),0x3f8);
        let output=build_format_guard_final().unwrap();
        assert_eq!(output.len(),FUNCTION_BYTES);
        assert_eq!(&output[output.len()-8..], &[0x1e,0xc0,0x1e,0x90,0x00,0xc0,0x9f,0x52]);
        assert!(jump(&mut Vec::new(),0x96c0_271c,0x96c0_271c).is_err());
        assert!(jump(&mut Vec::new(),0x96c0_271c,0x96c1_0000).is_err());
    }
}
