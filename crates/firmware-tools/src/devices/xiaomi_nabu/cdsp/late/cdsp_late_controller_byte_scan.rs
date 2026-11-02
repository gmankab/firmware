// SPDX-License-Identifier: GPL-3.0-only
//! CDSP controller's descriptor lookup and bounded byte-scan, original file
//! [0x569c0, 0x569fc), physical [0x96cc89c0, 0x96cc89fc).
//! Checked Hexagon operand encodings preserve original V66 duplex and packet
//! scheduling; the independent LLVM6 fixture reorders selected operations.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_selection;

pub const FILE_OFFSET:usize=0x569c0;
pub const END_FILE_OFFSET:usize=0x569fc;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_89c0;
pub const RWE_START:usize=cdsp_late_controller_selection::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=
    cdsp_late_controller_selection::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_selection::RWE_PREFIX_LAYOUT_LEN;
const TRACE_MESSAGE:u32=0xc022_61cd;
const TRACE_HELPER:u32=0x96dc_0bac;

fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// Packet begins with a signed address extender and passes the diagnostic
/// pointer in R0. Lower six bits remain inside the ordinary MOV instruction.
pub fn encode_trace_pointer(dst:G,address:u32)->Result<[u32;2],String>{
    if dst.number()!=0||address!=TRACE_MESSAGE {
        return Err("unverified controller descriptor trace pointer".into());
    }
    Ok(cdsp_boot::encode_extended_assignment(dst,address as i32,false))
}
/// V66 duplex `R2=memw(R16+16); memw(SP+8)=R16`; the original LLVM6
/// fixture confirms this packed 32-bit word without a separate extender.
pub fn encode_descriptor_and_stack_duplex(dst:G,base:G,stack:G,
        address_offset:u8,stack_offset:u8)->Result<u32,String>{
    if (dst.number(),base.number(),stack.number(),address_offset,stack_offset)!=(2,16,29,16,8){
        return Err("unsupported controller descriptor/load duplex".into());
    }
    Ok(0xc482_0000|(u32::from(stack_offset/4)<<10)|
       (u32::from(dst.number())<<4)|u32::from(address_offset/2))
}
/// Packet-relative call uses the controller's absolute physical service.
pub fn encode_trace_call(packet_start:u32,target:u32)->Result<u32,String>{
    if (packet_start,target)!=(0x96cc_89cc,TRACE_HELPER){
        return Err("trace helper call has unverified packet/target".into());
    }
    let d=target.checked_sub(packet_start).ok_or("backwards trace call")?;
    if d&1!=0||d>0x1ffffe{return Err("trace call exceeds checked relative range".into());}
    Ok(M::ContinuationA.apply(0x5a00_0000|(((d>>15)&0x3f)<<16)|((d>>1)&0x3fff)))
}
/// Packed stack pair stores saved byte cursor R21 and resource count R2.
pub fn encode_saved_scan_duplex(stack:G,cursor:G,handle:G,
                               first_offset:u8,second_offset:u8)->Result<u32,String>{
    if (stack.number(),cursor.number(),handle.number(),first_offset,second_offset)!=(29,21,2,0,4){
        return Err("unsupported saved controller scan state duplex".into());
    }
    Ok(0xe812_0000|(u32::from(second_offset/4)<<11)|
       (u32::from(stack.number()-16)&0xf))
}
/// Packed initialization of zero counter and controller input pointer.
pub fn encode_zero_counter_and_pointer(counter:G,pointer:G,source:G,offset:u8)->Result<u32,String>{
    if (counter.number(),pointer.number(),source.number(),offset)!=(2,17,16,16){
        return Err("unverified controller byte scan initialization".into());
    }
    Ok(0x4802_0400|(u32::from(counter.number())<<6)|u32::from(pointer.number()-8))
}
/// Combined unsigned compare and `P0.new` early jump (limit 31 bytes).
pub fn encode_bounded_byte_count(src:G,limit:u8,packet_start:u32,target:u32)->Result<u32,String>{
    if (src.number(),limit,packet_start,target)!=(2,31,0x96cc_89dc,0x96cc_89fc){
        return Err("controller scan bounded-count comparison not verified".into());
    }
    let d=target.checked_sub(packet_start).ok_or("unverified backwards bounded branch")?;
    if d&1!=0||d>0x1fe{return Err("bounded branch displacement exceeds limit".into());}
    Ok(M::Final.apply(0x1100_df00|(u32::from(src.number())<<16)|(d>>1)))
}
/// Signed byte load of the next candidate value. Operand fields are
/// checked against the independently assembled V66 byte-addressing form.
pub fn encode_current_byte(dst:G,base:G,byte_offset:u8)->Result<u32,String>{
    if (dst.number(),base.number(),byte_offset)!=(3,22,0){
        return Err("controller scan requires R3=memb(R22+0)".into());
    }
    Ok(M::ContinuationA.apply(0x9100_c000|
        (u32::from(base.number())<<16)|u32::from(dst.number())))
}
/// The fused consumer tests the newly read R3 byte for termination and
/// jumps over the remainder of the scan when it is zero.
pub fn encode_zero_byte_guard(src:G,packet:u32,target:u32)->Result<u32,String>{
    if (src.number(),packet,target)!=(3,0x96cc_89e0,0x96cc_89fc){
        return Err("controller byte-scan zero guard site/operand not verified".into());
    }
    let d=target.checked_sub(packet).ok_or("negative byte-scan guard")?;
    if d&1!=0||d>0x1fe{return Err("byte-scan zero guard not encodable".into());}
    Ok(M::Final.apply(0x2402_8000|(d>>1)))
}
/// Raw V66 signed byte, register+index operand form, from independent Hexagon.
pub fn encode_indexed_byte(dst:G,base:G,index:G)->Result<u32,String>{
    if (dst.number(),base.number(),index.number())!=(4,17,2){
        return Err("unverified signed indexed-byte operands".into());
    }
    Ok(M::Final.apply(0x3a00_0000|(u32::from(base.number())<<16)
        |(u32::from(index.number())<<8)|u32::from(dst.number())))
}
/// `if(P0.new) jump:t 0x96cc89dc` compares the loaded R4 and expected R3.
pub fn encode_matching_byte_backedge(left:G,right:G,packet:u32,target:u32)->Result<u32,String>{
    if (left.number(),right.number(),packet,target)!=(4,3,0x96cc_89f4,0x96cc_89dc){
        return Err("controller byte-compare retry site/operands not verified".into());
    }
    let delta=i64::from(target)-i64::from(packet);
    if delta>=0||delta < -0x100||delta&1!=0{return Err("backward match branch out of range".into());}
    Ok(M::Final.apply(0x1400_e000|(u32::from(left.number())<<16)
        |(u32::from(right.number())<<20)|(u32::from(right.number())<<8)
        |(((delta/2) as i32 as u32)&0xff)))
}
/// Unconditional fallthrough skip if a byte mismatches.
pub fn encode_skip_to_result(packet:u32,target:u32)->Result<u32,String>{
    if (packet,target)!=(0x96cc_89f8,0x96cc_8a0c){
        return Err("unsupported controller scan skip target".into());
    }
    let delta=target.checked_sub(packet).ok_or("scan skip backwards")?;
    if delta&1!=0||delta>0x1ffe{return Err("scan jump out of narrow range".into());}
    Ok(M::Final.apply(0x5800_8000|((delta>>1)&0x3fff)))
}

/// Checked construction of 15 original V66 instruction words. Duplex tuples
/// are deliberately narrow: generalization requires an independent assembler.
pub fn build_controller_byte_scan()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    for word in encode_trace_pointer(r(0)?,TRACE_MESSAGE)?{emit(&mut out,word);}
    emit(&mut out,encode_descriptor_and_stack_duplex(r(2)?,r(16)?,r(29)?,16,8)?);
    emit(&mut out,encode_trace_call(0x96cc_89cc,TRACE_HELPER)?);
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(22)?,src:r(29)?,value:24}.encode()?));
    emit(&mut out,encode_saved_scan_duplex(r(29)?,r(21)?,r(2)?,0,4)?);
    emit(&mut out,encode_zero_counter_and_pointer(r(2)?,r(17)?,r(16)?,16)?);
    emit(&mut out,encode_bounded_byte_count(r(2)?,31,0x96cc_89dc,0x96cc_89fc)?);
    emit(&mut out,encode_current_byte(r(3)?,r(22)?,0)?);
    emit(&mut out,encode_zero_byte_guard(r(3)?,0x96cc_89e0,0x96cc_89fc)?);
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(2)?,src:r(2)?,value:1}.encode()?));
    emit(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(22)?,src:r(22)?,value:1}.encode()?));
    emit(&mut out,encode_indexed_byte(r(4)?,r(17)?,r(2)?)?);
    emit(&mut out,encode_matching_byte_backedge(r(4)?,r(3)?,0x96cc_89f4,0x96cc_89dc)?);
    emit(&mut out,encode_skip_to_result(0x96cc_89f8,0x96cc_8a0c)?);
    if out.len()!=EXECUTABLE_LEN{return Err("CDSP controller scan must have 15 words".into());}
    Ok(out)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_selection::build_prefix()?;
    out.extend(build_controller_byte_scan()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN||out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("controller scan code/layout coverage does not match prefix".into());
    }
    Ok(out)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_controller_byte_scan()?.as_slice()){
        return Err("typed CDSP byte scan mismatches pinned reference".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("typed CDSP byte scan RWE prefix differs from reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn byte_scan_typed_words_and_bounds(){
        let data=build_controller_byte_scan().unwrap();
        assert_eq!(data.len(),60);
        let words:Vec<u32>=data.chunks_exact(4)
            .map(|b|u32::from_le_bytes(b.try_into().unwrap())).collect();
        assert_eq!(words,[0x0c02_4987,0x7800_41a0,0xc482_0828,0x5a1f_40f0,
          0xb01d_4316,0xe812_080d,0x4802_0489,0x1102_df10,
          0x9116_4003,0x2402_c00e,0xb002_4022,0xb016_4036,
          0x3a11_c204,0x1434_e3f4,0x5800_c00a]);
        assert!(encode_trace_call(0x96cc_89c8,TRACE_HELPER).is_err());
        assert!(encode_bounded_byte_count(r(2).unwrap(),32,0x96cc89dc,0x96cc89fc).is_err());
        assert!(encode_current_byte(r(4).unwrap(),r(22).unwrap(),0).is_err());
        assert!(encode_zero_byte_guard(r(4).unwrap(),0x96cc89e0,0x96cc89fc).is_err());
        assert!(encode_indexed_byte(r(4).unwrap(),r(17).unwrap(),r(3).unwrap()).is_err());
        assert!(encode_skip_to_result(0x96cc89f8,0x96cc89f0).is_err());
    }
    #[test]fn byte_scan_prefix_count(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,2556);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2500);
        assert_eq!(build_prefix().unwrap().len(),FULL_RWE_LAYOUT_LEN);
    }
}
