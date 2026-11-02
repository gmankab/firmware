// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 bank-count guards, source-built file [0x56b30,0x56b4c).
//! They consume freshly produced register values and use force-extended
//! packet-relative target displacements. Decoder/assembler corroboration
//! is independent; no original MBN instruction words enter these emitters.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_bank_entry;

pub const FILE_OFFSET:usize=0x56b30;
pub const END_FILE_OFFSET:usize=0x56b4c;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8b30;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const RWE_START:usize=cdsp_late_controller_bank_entry::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_bank_entry::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_bank_entry::RWE_PREFIX_LAYOUT_LEN;
const GP_BANK_COUNT:u32=0xc028_c0f8;
const NO_BANKS:u32=0x96cc_8e70;
const INSUFFICIENT_BANKS:u32=0x96cc_8e80;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn put(v:&mut Vec<u8>,w:u32){v.extend_from_slice(&w.to_le_bytes());}

/// Extend and branch when the freshly produced high bank limit R18 is zero.
/// Branch source is *packet* start, i.e. previous fragment's last LSR at
/// physical 0x96cc8b2c. The extender and the consumer split target bits.
pub fn encode_empty_bank_guard(new_limit:G,packet_start:u32,target:u32)->Result<[u32;2],String>{
    if new_limit.number()!=18||(packet_start,target)!=(0x96cc_8b2c,NO_BANKS){
        return Err("unverified controller zero bank count/packet target".into());
    }
    let delta=target.checked_sub(packet_start).ok_or("zero-bank branch must go forward")?;
    if delta%2!=0||delta>0x3fe {return Err("zero-bank branch displacement exceeds checked V66 span".into());}
    let ext=cdsp_boot::encode_extended_assignment(new_limit,delta as i32,false)[0];
    let body=0x2400_0000|(u32::from(new_limit.number()-16)<<16)|((delta&0x3f)<<1);
    Ok([M::ContinuationA.apply(ext),M::Final.apply(body)])
}
/// Original GP word load uses forced extended immediate, avoiding assembler
/// selection of a shorter address form. Absolute pointer revision is fixed.
pub fn encode_global_bank_count(dst:G,absolute:u32)->Result<[u32;2],String>{
    if dst.number()!=3||absolute!=GP_BANK_COUNT{return Err("unknown CDSP bank-count GP source".into());}
    let [ext,body]=cdsp_boot::encode_extended_gp_word_load(dst,absolute);
    Ok([M::ContinuationA.apply(ext),M::Final.apply(body)])
}
/// Newly produced R16 must be greater than R18 to enter the loop. A shared
/// packet from the preceding LSR is compared using this fused unsigned guard.
pub fn encode_insufficient_bank_guard(new_count:G,limit:G,packet_start:u32,target:u32)->Result<[u32;2],String>{
    if (new_count.number(),limit.number(),packet_start,target)!=(16,18,0x96cc_8b40,INSUFFICIENT_BANKS){
        return Err("unexpected bank count source/limit or branch site".into());
    }
    let delta=target.checked_sub(packet_start).ok_or("capacity branch backwards")?;
    if delta%2!=0||delta>0x3fe{return Err("capacity branch exceeds tested encoding width".into());}
    let ext=cdsp_boot::encode_extended_assignment(new_count,delta as i32,false)[0];
    // `.new` source 16 resides in the reduced register bank and the R18
    // comparison operand contributes to two disjoint consumer fields.
    let rhs=u32::from(limit.number()-16);
    let body=0x2102_1000|(rhs<<21)|(rhs<<8)|((delta&0x3f)>>1);
    Ok([M::ContinuationA.apply(ext),M::Final.apply(body)])
}
pub fn build_bank_guards()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    for w in encode_empty_bank_guard(r(18)?,0x96cc_8b2c,NO_BANKS)?{put(&mut out,w);}
    for w in encode_global_bank_count(r(3)?,GP_BANK_COUNT)?{put(&mut out,w);}
    put(&mut out,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(16)?,src:r(3)?,amount:12}.encode()?));
    for w in encode_insufficient_bank_guard(r(16)?,r(18)?,0x96cc_8b40,INSUFFICIENT_BANKS)?{put(&mut out,w);}
    if out.len()!=EXECUTABLE_LEN{return Err("controller bank guards must emit seven words".into());}
    Ok(out)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut b=cdsp_late_controller_bank_entry::build_prefix()?;
    b.extend(build_bank_guards()?);
    if b.len()!=FULL_RWE_LAYOUT_LEN||b.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("CDSP bank-guard prefix accounting failed".into());
    }
    Ok(b)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bank_guards()?.as_slice()){
        return Err("CDSP bank guard source differs from pinned MBN".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("CDSP bank guard RWE prefix differs from pinned MBN".into());
    }
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn guards_have_exact_checked_branch_fields(){
        let b=build_bank_guards().unwrap();assert_eq!(b.len(),28);
        let words:Vec<u32>=b.chunks_exact(4).map(|c|u32::from_le_bytes(c.try_into().unwrap())).collect();
        assert_eq!(words[0],0x0000_400d);assert_eq!(words[1],0x2402_c008);
        assert_eq!(words[2],0x0c02_6303);assert_eq!(words[3],0x4980_c703);
        assert_eq!(words[4],0x8c03_4c30);assert_eq!(words[5],0x0000_400d);
        assert_eq!(words[6],0x2142_d200);
        assert!(encode_empty_bank_guard(r(17).unwrap(),0x96cc8b2c,NO_BANKS).is_err());
        assert!(encode_global_bank_count(r(3).unwrap(),GP_BANK_COUNT+4).is_err());
        assert!(encode_insufficient_bank_guard(r(16).unwrap(),r(18).unwrap(),0x96cc8b40,INSUFFICIENT_BANKS+4).is_err());
    }
    #[test]fn bank_guard_span_preserves_rebuilt_prefix(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,2892);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2832);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,60);
        assert_eq!(build_prefix().unwrap().len(),2892);
    }
}
