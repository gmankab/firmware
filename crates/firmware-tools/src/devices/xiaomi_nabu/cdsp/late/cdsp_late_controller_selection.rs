// SPDX-License-Identifier: GPL-3.0-only
//! Controller candidate-buffer selection, file `[0x56990,0x569c0)`.
//!
//! Original V66 code computes a clamp/index, locates the buffer and guards
//! against a missing controller descriptor. No reference bytes are embedded
//! in the generated Rust output; all operations use checked Hexagon operands.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M};
use super::cdsp_late_controller_init_dispatch;

pub const FILE_OFFSET:usize=0x56990;
pub const END_FILE_OFFSET:usize=0x569c0;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8990;
pub const RWE_START:usize=cdsp_late_controller_init_dispatch::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_init_dispatch::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_init_dispatch::RWE_PREFIX_LAYOUT_LEN;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(out:&mut Vec<u8>,w:u32){out.extend_from_slice(&w.to_le_bytes());}

/// P0 = !(R21 > R19), confirmed from LLVM6 nonnegated compare fields.
pub fn encode_predicate_not_greater(left:G,right:G)->Result<u32,String>{
    if (left.number(),right.number())!=(21,19){return Err("unsupported candidate-count operands".into());}
    Ok(M::ContinuationA.apply(0xf240_c000|(u32::from(left.number())<<16)
        |(u32::from(right.number())<<8)|0x0010))
}
/// P1 = (R21 == 0), packet-continuation form.
pub fn encode_zero_count(src:G,pred:u8)->Result<u32,String>{
    if (src.number(),pred)!=(21,1){return Err("P1 must compare R21 with zero".into());}
    Ok(M::ContinuationA.apply(0x7500_c000|(u32::from(src.number())<<16)|u32::from(pred)))
}
/// R1:0 = combine(1,0), a checked duplex immediate pair.
pub fn encode_selector_default(low:G,high:u8,lower:u8)->Result<u32,String>{
    if (low.number(),high,lower)!=(0,1,0){return Err("selector requires R1:0=(1,0)".into());}
    Ok(M::ContinuationA.apply(0x7c00_c000 | (u32::from(high)<<5)|u32::from(low.number())))
}
/// `if(P1.new) R3=R19`; used to set the first candidate.
pub fn encode_if_zero_use_first(dst:G,src:G,pred:u8)->Result<u32,String>{
    if (dst.number(),src.number(),pred)!=(3,19,1){return Err("only P1.new may select R19 into R3".into());}
    Ok(M::Final.apply(0x7400_e000|(u32::from(pred)<<21)
        |(u32::from(src.number())<<16)|u32::from(dst.number())))
}
/// R2 = mux(P0,-1,0); restrict signed immediate and predicate bit.
pub fn encode_clamp_mux(dst:G,negative:i8,positive:i8)->Result<u32,String>{
    if (dst.number(),negative,positive)!=(2,-1,0){return Err("clamp mux expects P0 ? -1 : 0".into());}
    Ok(M::Final.apply(0x7a00_dfe0|u32::from(dst.number())))
}
/// If !P1, select the offset `R3=R21+R2`.
pub fn encode_adjusted_candidate(dst:G,left:G,right:G,pred:u8)->Result<u32,String>{
    if (dst.number(),left.number(),right.number(),pred)!=(3,21,2,1){
        return Err("candidate index must be R21+R2 under !P1".into());
    }
    Ok(M::Final.apply(0xfb00_c0a0|(u32::from(left.number())<<16)
        |(u32::from(right.number())<<8)|u32::from(dst.number())))
}
/// R2 = R20 + 160 * R3. LLVM6 independently encodes this multiply-add
/// word, with the original non-final parse marker restored.
pub fn encode_indexed_pointer(dst:G,base:G,index:G,stride:u16)->Result<u32,String>{
    if (dst.number(),base.number(),index.number(),stride)!=(2,20,3,160){
        return Err("indexed controller pointer must be R20 + 160*R3".into());
    }
    Ok(M::ContinuationA.apply(0xdf43_e214))
}
/// 0x569ac's direct CALL is measured from packet start at physical
/// 0x96cc89a8, one instruction before the CALL word.
pub fn encode_lookup_call(packet:u32,target:u32)->Result<u32,String>{
    if (packet,target)!=(0x96cc_89a8,0x96cc_fd00){
        return Err("controller lookup callback packet/target not independently verified".into());
    }
    let delta=target.checked_sub(packet).ok_or("lookup target before packet")?;
    if delta&1!=0||delta>0x1fff_fe{return Err("lookup displacement not in checked branch range".into());}
    Ok(M::Final.apply(0x5a00_4000|(((delta>>15)&0x3f)<<16)|((delta>>1)&0x3fff)))
}
/// R16 = R0, in the same original packet as following new-value check.
pub fn encode_lookup_result(dst:G,src:G)->Result<u32,String>{
    if (dst.number(),src.number())!=(16,0){return Err("lookup result must move R0 into R16".into());}
    Ok(M::ContinuationA.apply(0x7060_c000|u32::from(dst.number())))
}
/// If the newly loaded buffer pointer is zero, branch to the documented
/// controller error sequence; two checked packet starts have distinct targets.
pub fn encode_missing_handle_jump(packet:u32,target:u32)->Result<u32,String>{
    if ![(0x96cc_89b0,0x96cc_8adc),(0x96cc_89b8,0x96cc_8aec)].contains(&(packet,target)){
        return Err("unverified missing-buffer or missing-handle jump".into());
    }
    let d=target.checked_sub(packet).ok_or("error target behind packet")?;
    if d&1!=0||d>0x1fe{return Err("error target exceeds checked branch displacement".into());}
    Ok(M::Final.apply(0x2402_c000|((d>>1)&0xff)))
}
/// R2 = memw(R18+#0). Resource reference is stored at an aligned word.
pub fn encode_controller_handle(dst:G,base:G,offset:u8)->Result<u32,String>{
    if (dst.number(),base.number(),offset)!=(2,18,0){return Err("controller handle needs R2=memw(R18)".into());}
    Ok(M::ContinuationA.apply(0x9180_c000|(u32::from(base.number())<<16)|u32::from(dst.number())))
}

pub fn build_candidate_selection()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_predicate_not_greater(r(21)?,r(19)?)?);
    emit(&mut out,encode_zero_count(r(21)?,1)?);
    emit(&mut out,encode_selector_default(r(0)?,1,0)?);
    emit(&mut out,encode_if_zero_use_first(r(3)?,r(19)?,1)?);
    emit(&mut out,encode_clamp_mux(r(2)?,-1,0)?);
    emit(&mut out,encode_adjusted_candidate(r(3)?,r(21)?,r(2)?,1)?);
    emit(&mut out,encode_indexed_pointer(r(2)?,r(20)?,r(3)?,160)?);
    emit(&mut out,encode_lookup_call(0x96cc_89a8,0x96cc_fd00)?);
    emit(&mut out,encode_lookup_result(r(16)?,r(0)?)?);
    emit(&mut out,encode_missing_handle_jump(0x96cc_89b0,0x96cc_8adc)?);
    emit(&mut out,encode_controller_handle(r(2)?,r(18)?,0)?);
    emit(&mut out,encode_missing_handle_jump(0x96cc_89b8,0x96cc_8aec)?);
    if out.len()!=EXECUTABLE_LEN{return Err("controller candidate selector must be 48 bytes".into());}
    Ok(out)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_init_dispatch::build_prefix()?;
    out.extend(build_candidate_selection()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN||out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("controller buffer selection code/fill accounting mismatch".into());
    }
    Ok(out)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_candidate_selection()?.as_slice()){
        return Err("source-built controller buffer selector differs from original CDSP".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("joined controller-selection RWE prefix differs from original CDSP".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_candidate_fields_and_displacements_are_checked(){
        let b=build_candidate_selection().unwrap();assert_eq!(b.len(),48);
        let w:Vec<u32>=b.chunks_exact(4).map(|b|u32::from_le_bytes(b.try_into().unwrap())).collect();
        assert_eq!(w,[0xf255_5310,0x7515_4001,0x7c00_4020,0x7433_e003,
          0x7a00_dfe2,0xfb15_c2a3,0xdf43_6214,0x5a00_f9ac,
          0x7060_4010,0x2402_c096,0x9192_4002,0x2402_c09a]);
        assert!(encode_zero_count(r(21).unwrap(),0).is_err());
        assert!(encode_adjusted_candidate(r(3).unwrap(),r(20).unwrap(),r(2).unwrap(),1).is_err());
        assert!(encode_indexed_pointer(r(2).unwrap(),r(20).unwrap(),r(3).unwrap(),128).is_err());
        assert!(encode_lookup_call(0x96cc89ac,0x96ccfd00).is_err());
        assert!(encode_missing_handle_jump(0x96cc89b4,0x96cc8adc).is_err());
    }
    #[test]fn selection_preserves_layout_and_file_boundaries(){
        assert_eq!(build_prefix().unwrap().len(),0x9c0);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2440);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,56);
        assert_eq!(FILE_OFFSET+EXECUTABLE_LEN,END_FILE_OFFSET);
    }
}
