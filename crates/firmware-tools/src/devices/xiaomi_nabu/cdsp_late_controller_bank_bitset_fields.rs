// SPDX-License-Identifier: GPL-3.0-only
//! V66 bank bitmap field extraction and the original compressed load/store
//! pair. LLVM6 reconstructs the operations but expands the duplex, so the
//! original pair is emitted as one checked Hexagon field-encoded word.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_bank_bitset_limits;
pub const FILE_OFFSET:usize=0x56bf0;
pub const END_FILE_OFFSET:usize=0x56c10;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8bf0;
pub const RWE_START:usize=cdsp_late_controller_bank_bitset_limits::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_bank_bitset_limits::RWE_PREFIX_LAYOUT_LEN;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_bank_bitset_limits::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
const TOP_BITMAP_FLAG:u32=0x0080_0000;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn put(b:&mut Vec<u8>,w:u32){b.extend_from_slice(&w.to_le_bytes());}
/// `R7:6=lsr(R1:0,#38)` uses a 64-bit pair shift, and its 38-bit amount
/// has distinct high-amount fields (not a 32-bit scalar LSR).
pub fn encode_long_bitmap_shift(dst_low:G,source_low:G,bits:u8)->Result<u32,String>{
    if (dst_low.number(),source_low.number(),bits)!=(6,0,38){
        return Err("unverified 64-bit bitmap shift tuple".into());
    }
    if dst_low.number()%2!=0||source_low.number()%2!=0{return Err("pair shift requires even low registers".into());}
    Ok(M::ContinuationA.apply(0x8000_c000 | (u32::from(bits&31)<<8)
      | if bits>=32 {0x2020}else{0} | u32::from(dst_low.number())))
}
/// Original single-word duplex combines `R5=memw(R20+#0)` and
/// `memw(FP+#0)=R17`. The original compressed fields are documented and
/// narrowly constrained to the observed source/destination register banks.
pub fn encode_controller_load_and_cursor_store(dst:G,base:G,frame:G,source:G,load_offset:u8,store_offset:u8)->Result<u32,String>{
    if (dst.number(),base.number(),frame.number(),source.number(),load_offset,store_offset)!=(5,20,29,17,0,0){
        return Err("unsupported controller bitmap duplex operands".into());
    }
    let bank_base=u32::from(base.number()-12); // V66 compact high-register bank
    let bank_src=u32::from(source.number()-8); // V66 compact store source bank
    Ok(0xc0c0_0000 | (u32::from(dst.number())<<16) |
       (bank_base<<8) | bank_src)
}
/// Unsigned mask of `R3=and(R4,R3)` with named register fields.
pub fn encode_bitmap_local_mask(dst:G,lhs:G,rhs:G)->Result<u32,String>{
    if (dst.number(),lhs.number(),rhs.number())!=(3,4,3){
        return Err("unverified CDSP bank local mask operands".into());
    }
    Ok(M::ContinuationA.apply(0xf100_c000 | (u32::from(lhs.number())<<16)
          | (u32::from(rhs.number())<<8) | u32::from(dst.number())))
}
/// `R18=or(R3,and(R18,##0x800000))`, a two-operand masked OR after the
/// explicitly emitted `immext` prefix. Independently assembled by LLVM6.
pub fn encode_top_bitmap_flag(dst:G,primary:G,masked:G,flag:u32)->Result<[u32;2],String>{
    if (dst.number(),primary.number(),masked.number(),flag)!=(18,3,18,TOP_BITMAP_FLAG){
        return Err("unverified controller upper-bit OR mask".into());
    }
    let ext=((flag>>20)<<16)|((flag>>6)&0x3fff)|0x4000;
    let consumer=M::ContinuationA.apply(0xda40_0000 | (u32::from(masked.number())<<16)
          | u32::from(primary.number()));
    Ok([ext,consumer])
}

pub fn build_bitmap_fields()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    put(&mut out,encode_long_bitmap_shift(r(6)?,r(0)?,38)?);
    put(&mut out,encode_controller_load_and_cursor_store(r(5)?,r(20)?,r(29)?,r(17)?,0,0)?);
    put(&mut out,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(8)?,src:r(0)?,amount:29}.encode()?));
    put(&mut out,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(9)?,src:r(0)?,amount:25}.encode()?));
    put(&mut out,encode_bitmap_local_mask(r(3)?,r(4)?,r(3)?)?);
    put(&mut out,S::AssignRegister{dst:r(18)?,src:r(6)?}.encode()?);
    for w in encode_top_bitmap_flag(r(18)?,r(3)?,r(18)?,TOP_BITMAP_FLAG)?{put(&mut out,w);}
    if out.len()!=EXECUTABLE_LEN{return Err("CDSP bitmap fields must emit eight words".into());}
    Ok(out)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_bank_bitset_limits::build_prefix()?;
    out.extend(build_bitmap_fields()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN || out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN {
        return Err("CDSP bitmap field prefix fill accounting wrong".into());
    }
    Ok(out)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bitmap_fields()?.as_slice()){
        return Err("CDSP bank bitfield source differs from reference".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("CDSP bank bitfield joined layout differs from reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn checked_bank_field_words_match_original_isa_forms(){
        let b=build_bitmap_fields().unwrap();assert_eq!(b.len(),32);
        let w:Vec<u32>=b.chunks_exact(4).map(|x|u32::from_le_bytes(x.try_into().unwrap())).collect();
        assert_eq!(w,[0x8000_6626,0xc0c5_0809,0x8c00_5d28,0x8c00_5929,
                     0xf104_4303,0x7066_c012,0x0008_4000,0xda52_4003]);
        assert!(encode_long_bitmap_shift(r(6).unwrap(),r(2).unwrap(),38).is_err());
        assert!(encode_controller_load_and_cursor_store(r(4).unwrap(),r(20).unwrap(),r(29).unwrap(),r(17).unwrap(),0,0).is_err());
        assert!(encode_bitmap_local_mask(r(3).unwrap(),r(4).unwrap(),r(2).unwrap()).is_err());
        assert!(encode_top_bitmap_flag(r(18).unwrap(),r(3).unwrap(),r(18).unwrap(),0x40).is_err());
    }
    #[test]fn layout_equals_executable_plus_separate_fill(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,3088);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,3028);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,60);
        assert_eq!(build_prefix().unwrap().len(),3088);
    }
}
