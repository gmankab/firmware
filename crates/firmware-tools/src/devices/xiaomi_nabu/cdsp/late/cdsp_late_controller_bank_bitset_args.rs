// SPDX-License-Identifier: GPL-3.0-only
//! CDSP Hexagon controller bitmap command arguments: pair extractions,
//! controller-table read, original two-operation duplex, masked selectors.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_bank_bitset_fields;
pub const FILE_OFFSET:usize=0x56c10;
pub const END_FILE_OFFSET:usize=0x56c30;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8c10;
pub const RWE_START:usize=cdsp_late_controller_bank_bitset_fields::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_bank_bitset_fields::RWE_PREFIX_LAYOUT_LEN;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_bank_bitset_fields::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn put(v:&mut Vec<u8>,w:u32){v.extend_from_slice(&w.to_le_bytes());}
/// 64-bit `Rdest_odd:even=extractu(Rsrc_odd:even,#width,#offset)`.
/// Explicit 64-bit field mapping independently cross-checked by LLVM6.
pub fn encode_64bit_pair_extract(dst:G,src:G,width:u8,offset:u8)->Result<u32,String>{
    if !matches!((dst.number(),src.number(),width,offset),(24,6,9,17)|(22,6,6,20)) ||
        dst.number()%2!=0 || src.number()%2!=0 || u16::from(width)+u16::from(offset)>64 {
        return Err("unverified 64-bit bank descriptor extraction operands".into());
    }
    Ok(M::ContinuationA.apply(0x8100_c000 |
       (u32::from(offset&16)<<18) | (u32::from(src.number())<<16) |
       (u32::from(width)<<8) | (u32::from(offset&15)<<5) | u32::from(dst.number())))
}
/// `R1=memw(R21+#0)`, original independent LLVM load encoding.
pub fn encode_controller_word_load(dst:G,base:G,offset:u16)->Result<u32,String>{
    if (dst.number(),base.number(),offset)!=(1,21,0){return Err("unverified controller state word location".into());}
    Ok(0x9180_c000 | (u32::from(base.number())<<16) | u32::from(dst.number()))
}
/// The distinct signed/unsigned offset bits for `R2=extractu(R2,#4,#24)`
/// are independently assembled; the 24-bit offset occupies bits 21..22.
pub fn encode_upper_word_extract(dst:G,src:G,width:u8,offset:u8)->Result<u32,String>{
    if (dst.number(),src.number(),width,offset)!=(2,2,4,24){return Err("unverified upper descriptor extract tuple".into());}
    Ok(M::ContinuationA.apply(0x8d00_c000 | (u32::from(offset&24)<<18) |
        (u32::from(src.number())<<16) | (u32::from(width)<<8) |
        (u32::from(offset&7)<<5) | u32::from(dst.number())))
}
/// Original one-word V66 duplex: `R0=add(FP,#28); R3=R16`.
/// LLVM6 emits two separate instructions; this form is constrained to the
/// observed frame register, constant and compact source/destination fields.
pub fn encode_frame_pointer_and_register_copy(frame:G,frame_byte_offset:u8,dst:G,src:G)->Result<u32,String>{
    if (frame.number(),frame_byte_offset,dst.number(),src.number())!=(29,28,3,16){
        return Err("unverified compact frame-address and GPR copy duplex".into());
    }
    Ok(0x2c70_3000 | (u32::from(src.number()-8)<<4) | u32::from(dst.number()))
}
/// Hexagon masked-OR compound with a small flag constant. Destination
/// always equals the value being masked; the source is a separate GPR.
pub fn encode_small_masked_or(dst:G,primary:G,mask:u8)->Result<u32,String>{
    if !matches!((dst.number(),primary.number(),mask),(24,2,0x30)|(9,8,0x8)){
        return Err("unverified masked descriptor flag update".into());
    }
    Ok(M::ContinuationA.apply(0xda40_0000 | (u32::from(dst.number())<<16) |
         (u32::from(mask)<<5) | u32::from(primary.number())))
}
pub fn build_bitmap_args()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    put(&mut out,encode_64bit_pair_extract(r(24)?,r(6)?,9,17)?);
    put(&mut out,encode_controller_word_load(r(1)?,r(21)?,0)?);
    put(&mut out,encode_upper_word_extract(r(2)?,r(2)?,4,24)?);
    put(&mut out,encode_64bit_pair_extract(r(22)?,r(6)?,6,20)?);
    put(&mut out,encode_frame_pointer_and_register_copy(r(29)?,28,r(3)?,r(16)?)?);
    put(&mut out,encode_small_masked_or(r(24)?,r(2)?,0x30)?);
    put(&mut out,encode_small_masked_or(r(9)?,r(8)?,0x8)?);
    put(&mut out,M::ContinuationA.apply(S::AddSmallImmediate{dst:r(2)?,src:r(5)?,value:24}.encode()?));
    if out.len()!=EXECUTABLE_LEN{return Err("bitmap argument prefix must contain eight words".into());}
    Ok(out)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_bank_bitset_fields::build_prefix()?;
    out.extend(build_bitmap_args()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN || out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("CDSP bank argument prefix layout accounting wrong".into());
    }
    Ok(out)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bitmap_args()?.as_slice()){
        return Err("CDSP bitmap arguments do not match original".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("CDSP bitmap arguments joined prefix mismatch".into());
    }
    Ok(())
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn controller_bitmap_fields_encode_original_eight_words(){
        let b=build_bitmap_args().unwrap();assert_eq!(b.len(),32);
        let w:Vec<u32>=b.chunks_exact(4).map(|x|u32::from_le_bytes(x.try_into().unwrap())).collect();
        assert_eq!(w,[0x8146_4938,0x9195_c001,0x8d62_4402,0x8146_4696,
          0x2c70_3083,0xda58_4602,0xda49_4108,0xb005_4302]);
        assert!(encode_64bit_pair_extract(r(24).unwrap(),r(6).unwrap(),9,18).is_err());
        assert!(encode_controller_word_load(r(1).unwrap(),r(20).unwrap(),0).is_err());
        assert!(encode_upper_word_extract(r(2).unwrap(),r(2).unwrap(),4,23).is_err());
        assert!(encode_frame_pointer_and_register_copy(r(29).unwrap(),24,r(3).unwrap(),r(16).unwrap()).is_err());
        assert!(encode_small_masked_or(r(24).unwrap(),r(3).unwrap(),0x30).is_err());
    }
    #[test]fn joined_source_layout_has_exact_fill_separation(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,3120);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,3060);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,60);
        assert_eq!(build_prefix().unwrap().len(),3120);
    }
}
