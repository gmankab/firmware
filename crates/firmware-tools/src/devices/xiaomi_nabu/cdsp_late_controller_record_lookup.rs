// SPDX-License-Identifier: GPL-3.0-only
//! Source-built CDSP indexed controller-record lookup, file [0x56a30,0x56a74).
//! Hexagon V66 packet scheduling is preserved with explicit typed operands.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_status;
pub const FILE_OFFSET:usize=0x56a30;
pub const END_FILE_OFFSET:usize=0x56a74;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8a30;
pub const RWE_START:usize=cdsp_late_controller_status::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_status::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_status::RWE_PREFIX_LAYOUT_LEN;
const TRACE_ADDRESS:u32=0xc022_61f4;
const TRACE_HELPER:u32=0x96dc_0bac;
const RESOURCE_HELPER:u32=0x96cd_1230;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes())}
/// `Rd=memw(Rbase+Rindex<<0)`, for known V66 tuple at the index-table head.
pub fn encode_record_word_load(dst:G,base:G,index:G)->Result<u32,String>{
    if ![(3,17,23),(1,2,23)].contains(&(dst.number(),base.number(),index.number())){
        return Err("controller record index registers not verified".into());
    }
    let encoded=0x3a80_c000|(u32::from(base.number())<<16)|
        (u32::from(index.number())<<8)|u32::from(dst.number());
    Ok(if dst.number()==3{M::ContinuationA.apply(encoded)}else{M::Final.apply(encoded)})
}
/// V66 `if(cmp.eq(R3.new,-1)) jump:nt +0x74` uses the packet-relative
/// branch field and two-byte bias checked against the independent decoder.
pub fn encode_missing_record_branch(src:G,packet:u32,target:u32)->Result<u32,String>{
    if (src.number(),packet,target)!=(3,0x96cc_8a34,0x96cc_8aa8){
        return Err("unverified invalid-record guard".into());
    }
    let d=target.checked_sub(packet).ok_or("backwards record guard")?;
    if d&1!=0||d>0x1fe{return Err("invalid-record guard out of range".into());}
    Ok(M::Final.apply(0x2602_8000|((d+4)>>1)))
}
/// The trace call is an ordinary packet-relative Hexagon V66 direct CALL.
pub fn encode_record_call(packet:u32,target:u32)->Result<u32,String>{
    if ![(0x96cc_8a44,TRACE_HELPER),(0x96cc_8a64,RESOURCE_HELPER)].contains(&(packet,target)){
        return Err("unverified indexed record controller call".into());
    }
    let d=target.checked_sub(packet).ok_or("negative record service call")?;
    if d&1!=0||d>0x1ffffe{return Err("indexed record service displacement too large".into());}
    Ok(M::ContinuationA.apply(0x5a00_0000|(((d>>15)&0x3f)<<16)|((d>>1)&0x3fff)))
}
/// Duplex stores: memw(SP+8)=R2 and memw(SP+4)=R3.
pub fn encode_saved_record_duplex(stack:G,left:G,right:G,off1:u8,off2:u8)->Result<u32,String>{
    if (stack.number(),left.number(),right.number(),off1,off2)!=(29,2,3,8,4){
        return Err("record lookup expects pair of adjacent frame stores".into());
    }
    Ok(M::ContinuationB.apply(0xe822_0000|(u32::from(off1/4)<<10)|
       (u32::from(off2/4)<<4)|u32::from(right.number())))
}
/// V66 frame save `memw(SP+0)=R22`.
pub fn encode_saved_cursor(stack:G,src:G,offset:u8)->Result<u32,String>{
    if (stack.number(),src.number(),offset)!=(29,22,0){
        return Err("indexed lookup expects SP and saved R22".into());
    }
    Ok(M::Final.apply(0xa180_9000|(u32::from(stack.number())<<16)|
        (u32::from(src.number()&0xf)<<8)|u32::from(offset/4)))
}
/// ordinary aligned word read via an explicit base and immediate offset.
pub fn encode_record_base_word(dst:G,base:G,offset:u8,last:bool)->Result<u32,String>{
    if ![(2,16,16),(3,3,4),(2,18,0)].contains(&(dst.number(),base.number(),offset)){
        return Err("unverified indexed record base and offset".into());
    }
    let raw=0x9180_c000|(u32::from(base.number())<<16)|
        (u32::from(offset/4)<<5)|u32::from(dst.number());
    Ok(if last{M::Final.apply(raw)}else{M::ContinuationA.apply(raw)})
}
/// Move a register with the correct packet marker.
pub fn encode_pointer_move(dst:G,src:G)->Result<u32,String>{
    if (dst.number(),src.number())!=(0,17){return Err("unverified record pointer argument".into());}
    Ok(M::ContinuationA.apply(0x7060_c000|(u32::from(src.number())<<16)|u32::from(dst.number())))
}
/// Original compound if `R2.new==R16` then jump:nt error from file 0x56a70.
pub fn encode_same_handle_guard(src:G,expected:G,packet:u32,target:u32)->Result<u32,String>{
    if (src.number(),expected.number(),packet,target)!=(2,16,0x96cc_8a70,0x96cc_8aa8){
        return Err("unverified matching controller handle skip".into());
    }
    let d=target.checked_sub(packet).ok_or("handle guard backwards")?;
    if d&1!=0||d>0x1fe{return Err("handle guard out of range".into());}
    Ok(M::Final.apply(0x2002_9000|((d+4)>>1)))
}

pub fn build_indexed_record_lookup()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_record_word_load(r(3)?,r(17)?,r(23)?)?);
    emit(&mut out,encode_missing_record_branch(r(3)?,0x96cc_8a34,0x96cc_8aa8)?);
    for w in cdsp_boot::encode_extended_assignment(r(0)?,TRACE_ADDRESS as i32,false){emit(&mut out,w);}
    emit(&mut out,encode_saved_record_duplex(r(29)?,r(2)?,r(3)?,8,4)?);
    emit(&mut out,encode_record_call(0x96cc_8a44,TRACE_HELPER)?);
    emit(&mut out,encode_saved_cursor(r(29)?,r(22)?,0)?);
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(17)?,src:r(16)?,value:24}.encode()?));
    emit(&mut out,encode_record_base_word(r(2)?,r(16)?,16,true)?);
    emit(&mut out,M::ContinuationA.apply(S::AddRegisters{dst:r(3)?,left:r(2)?,right:r(23)?}.encode()?));
    emit(&mut out,encode_pointer_move(r(0)?,r(17)?)?);
    emit(&mut out,encode_record_word_load(r(1)?,r(2)?,r(23)?)?);
    emit(&mut out,encode_record_base_word(r(3)?,r(3)?,4,true)?);
    emit(&mut out,encode_record_call(0x96cc_8a64,RESOURCE_HELPER)?);
    emit(&mut out,M::Final.apply(S::LogicalShiftRight{dst:r(2)?,src:r(3)?,amount:12}.encode()?));
    emit(&mut out,encode_record_base_word(r(2)?,r(18)?,0,false)?);
    emit(&mut out,encode_same_handle_guard(r(2)?,r(16)?,0x96cc_8a70,0x96cc_8aa8)?);
    if out.len()!=EXECUTABLE_LEN{return Err("indexed controller lookup must contain 17 words".into());}
    Ok(out)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_status::build_prefix()?;
    out.extend(build_indexed_record_lookup()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN||out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("indexed record lookup/layout accounting failed".into());
    }
    Ok(out)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_indexed_record_lookup()?.as_slice()){
        return Err("CDSP indexed-record lookup does not match pinned reference".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("CDSP indexed lookup full prefix differs from original".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn indexed_record_words_match_expected_field_encodings(){
        let data=build_indexed_record_lookup().unwrap();assert_eq!(data.len(),68);
        let w:Vec<u32>=data.chunks_exact(4)
            .map(|c|u32::from_le_bytes(c.try_into().unwrap())).collect();
        assert_eq!(w,[0x3a91_5703,0x2602_c03c,0x0c02_4987,0x7800_4680,
          0xe822_0813,0x5a1f_40b4,0xa19d_d600,0xb010_4311,
          0x9190_c082,0xf302_5703,0x7071_4000,0x3a82_d701,
          0x9183_c023,0x5a01_43e6,0x8c03_cc22,0x9192_4002,0x2002_d01e]);
        assert!(encode_record_call(0x96cc8a40,TRACE_HELPER).is_err());
        assert!(encode_record_word_load(r(3).unwrap(),r(16).unwrap(),r(23).unwrap()).is_err());
        assert!(encode_same_handle_guard(r(2).unwrap(),r(16).unwrap(),0x96cc8a70,0x96cc8aa0).is_err());
    }
    #[test]fn indexed_lookup_keeps_prior_layout_accounting(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,2676);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2620);
        assert_eq!(build_prefix().unwrap().len(),FULL_RWE_LAYOUT_LEN);
    }
}
