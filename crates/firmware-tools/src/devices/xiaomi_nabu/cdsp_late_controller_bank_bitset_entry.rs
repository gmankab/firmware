// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 controller-bank enumeration bitset entry: initialize the
//! global allocation bitmap and caller's frame before reading its first
//! 64-bit word. Original executable CDSP file [0x56b98,0x56bb4).
//! Constants are typed address operands, not imported reference opcodes.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_bank_predicates;
pub const FILE_OFFSET:usize=0x56b98;
pub const END_FILE_OFFSET:usize=0x56bb4;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8b98;
pub const RWE_START:usize=cdsp_late_controller_bank_predicates::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=
    cdsp_late_controller_bank_predicates::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_bank_predicates::RWE_PREFIX_LAYOUT_LEN;
const GLOBAL_BANK_BITMAP:u32=0xc028_c100;
const CONTROLLER_TABLE:u32=0xc036_4ae0;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(b:&mut Vec<u8>,w:u32){b.extend_from_slice(&w.to_le_bytes());}

/// `R27:26=combine(#0,##0xc028c100)` splits the 32-bit bitmap address
/// into an extended constant and the original low-register-pair consumer.
pub fn encode_bank_bitmap_pointer(low:G,high_immediate:u8,address:u32)->Result<[u32;2],String>{
    if (low.number(),high_immediate,address)!=(26,0,GLOBAL_BANK_BITMAP){
        return Err("unverified CDSP bank bitmap address/register pair".into());
    }
    let [prefix,_]=cdsp_boot::encode_extended_assignment(low,address as i32,false);
    let body=0x7c80_c000|u32::from(low.number());
    Ok([M::ContinuationA.apply(prefix),M::ContinuationA.apply(body)])
}
/// `R20=##0xc0364ae0`, the controller descriptor table pointer. Its
/// extended form and low-six-bit immediate are encoded by cdsp_boot.
pub fn encode_controller_table_pointer(dst:G,address:u32)->Result<[u32;2],String>{
    if (dst.number(),address)!=(20,CONTROLLER_TABLE){
        return Err("unverified CDSP controller table address or destination".into());
    }
    let [ext,body]=cdsp_boot::encode_extended_assignment(dst,address as i32,false);
    Ok([M::ContinuationA.apply(ext),M::Final.apply(body)])
}
/// `R23:22=combine(#0,#0)` allocates a zero-width sentinel pair.
pub fn encode_zero_sentinel(low:G)->Result<u32,String>{
    if low.number()!=22{return Err("zero sentinel must use R23:22".into());}
    Ok(S::CombineZeroPair{low}.encode()?)
}
/// The original one-word duplex saves R18 at FP+24 and R19 at FP+20.
/// LLVM6 expands it into two semantic stores; Rizin confirms this original
/// V66 compressed form. The tuple is explicitly restricted rather than
/// guessing other duplex address fields.
pub fn encode_two_bank_cursor_saves(frame:G,first:G,second:G,off1:u8,off2:u8)->Result<u32,String>{
    if (frame.number(),first.number(),second.number(),off1,off2)!=(29,18,19,24,20){
        return Err("unverified bank cursor save duplex tuple".into());
    }
    if off1%4!=0||off2%4!=0{return Err("bank cursor saves must be word aligned".into());}
    let base=0xe800_085b | (u32::from(off1/4)<<20) | (u32::from(off2/4)<<17);
    Ok(M::ContinuationB.apply(base))
}
/// One aligned 64-bit bitmap read `R1:0=memd(R26+#0)`.
pub fn encode_bitmap_pair_load(low:G,base:G,offset:u8)->Result<u32,String>{
    if (low.number(),base.number(),offset)!=(0,26,0){
        return Err("unverified bank bitmap 64-bit word load".into());
    }
    Ok(M::Final.apply(0x91c0_c000|(u32::from(base.number())<<16)|u32::from(low.number())))
}

pub fn build_bank_bitset_entry()->Result<Vec<u8>,String>{
    let mut result=Vec::with_capacity(EXECUTABLE_LEN);
    for word in encode_bank_bitmap_pointer(r(26)?,0,GLOBAL_BANK_BITMAP)?{emit(&mut result,word);}
    for word in encode_controller_table_pointer(r(20)?,CONTROLLER_TABLE)?{emit(&mut result,word);}
    emit(&mut result,encode_zero_sentinel(r(22)?)?);
    emit(&mut result,encode_two_bank_cursor_saves(r(29)?,r(18)?,r(19)?,24,20)?);
    emit(&mut result,encode_bitmap_pair_load(r(0)?,r(26)?,0)?);
    if result.len()!=EXECUTABLE_LEN{return Err("CDSP bank bitset entry must be exactly seven words".into());}
    Ok(result)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut prefix=cdsp_late_controller_bank_predicates::build_prefix()?;
    prefix.extend(build_bank_bitset_entry()?);
    if prefix.len()!=FULL_RWE_LAYOUT_LEN||prefix.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("bank bitset prefix accounting wrong".into());
    }
    Ok(prefix)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bank_bitset_entry()?.as_slice()){
        return Err("CDSP bank bitset entry mismatches original bytes".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("CDSP bank bitmap RWE prefix differs from reference".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn bitmap_entry_emits_checked_field_encodings(){
        let bytes=build_bank_bitset_entry().unwrap();assert_eq!(bytes.len(),28);
        let w:Vec<u32>=bytes.chunks_exact(4).map(|c|u32::from_le_bytes(c.try_into().unwrap())).collect();
        assert_eq!(w,[0x0c02_6304,0x7c80_401a,0x0c03_592b,
           0x7800_c414,0x7c00_c016,0xe86a_085b,0x91da_c000]);
        assert!(encode_bank_bitmap_pointer(r(26).unwrap(),1,GLOBAL_BANK_BITMAP).is_err());
        assert!(encode_controller_table_pointer(r(19).unwrap(),CONTROLLER_TABLE).is_err());
        assert!(encode_zero_sentinel(r(20).unwrap()).is_err());
        assert!(encode_two_bank_cursor_saves(r(29).unwrap(),r(18).unwrap(),r(19).unwrap(),24,16).is_err());
        assert!(encode_bitmap_pair_load(r(2).unwrap(),r(26).unwrap(),0).is_err());
    }
    #[test]fn bitmap_entry_preserves_exact_file_and_code_layout(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,2996);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2936);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,60);
        assert_eq!(build_prefix().unwrap().len(),2996);
    }
}
