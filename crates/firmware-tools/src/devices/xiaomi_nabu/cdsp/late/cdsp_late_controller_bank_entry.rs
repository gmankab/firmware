// SPDX-License-Identifier: GPL-3.0-only
//! V66 controller memory-bank enumeration prologue, CDSP file 0x56b00..0x56b30.
//! The function continues beyond this verified 48-byte prefix. All words are
//! independently encoded from checked instruction fields; reference bytes
//! are consumed only by the optional verifier, never by the builder.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_record_diagnostics;

pub const FILE_OFFSET:usize=0x56b00;
pub const END_FILE_OFFSET:usize=0x56b30;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8b00;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const RWE_START:usize=cdsp_late_controller_record_diagnostics::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_record_diagnostics::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_record_diagnostics::RWE_PREFIX_LAYOUT_LEN;
const CONTROLLER_RANGE_FAIL:u32=0x96cc_8e1c;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn put(v:&mut Vec<u8>,w:u32){v.extend_from_slice(&w.to_le_bytes());}

/// Hexagon V66 `memd(FP-16)=R17:16; allocframe(#80)`; a two-operation
/// duplex checked independently against LLVM 6. The frame size is /8 coded.
pub fn encode_frame_pair_and_allocation(fp:G,lo:G,bytes:u8)->Result<u32,String>{
    if (fp.number(),lo.number(),bytes)!=(29,16,80){return Err("bank enumerator requires R17:16 and an 80-byte frame".into());}
    if bytes%8!=0 {return Err("frame allocation not eight-byte aligned".into());}
    Ok(0xebf4_1c00|(u32::from(bytes/8)<<4))
}
/// V66 paired `memd(FP+64)=R19:18; memd(FP+56)=R21:20` duplex.
/// The original size/offset combination has been assembled independently.
pub fn encode_frame_save_duplex(fp:G,first:G,second:G,first_off:u8,second_off:u8)->Result<u32,String>{
    if (fp.number(),first.number(),second.number(),first_off,second_off)!=(29,18,20,64,56){
        return Err("unverified controller paired FP saves".into());
    }
    if first_off%8!=0||second_off%8!=0{return Err("frame offsets must be doubleword aligned".into());}
    Ok(0xea05_0a06|(u32::from(first_off/8)<<19)|(u32::from(second_off/8)<<3))
}
/// Predicated large negative immediate assignment: `if(P0) R2=##-0x40000000`.
/// The high constant is encoded into an architecture immext prefix; the
/// consumer is a checked conditional immediate, not an opaque opcode.
pub fn encode_predicated_controller_mask(dst:G,predicate:u8,mask:i32)->Result<[u32;2],String>{
    if (dst.number(),predicate,mask)!=(2,0,-0x4000_0000){return Err("unverified guarded bank mask".into());}
    let ext=cdsp_boot::encode_extended_assignment(dst,mask,false)[0];
    Ok([M::ContinuationA.apply(ext),M::ContinuationA.apply(0x7e00_0000|u32::from(dst.number()))])
}
/// One checked stack doubleword save of an even/odd GPR pair. The first save
/// is packet-internal; the second closes it; the final closes the branch packet.
pub fn encode_stack_bank_pair(base:G,low:G,offset:u8,last:bool)->Result<u32,String>{
    if base.number()!=29||![(22,48),(24,40),(26,32)].contains(&(low.number(),offset)){
        return Err("invalid bank-controller callee-saved register pair/stack offset".into());
    }
    if offset%8!=0{return Err("stack pair offset must be 8-byte aligned".into());}
    let raw=0xa1dd_0000|(u32::from(low.number())<<8)|u32::from(offset/8);
    Ok((if last{M::Final}else{M::ContinuationA}).apply(raw))
}
/// `if(!P0) jump:nt` to the first bank-controller fallback. The 9th branch
/// displacement halfword bit is encoded at bit 13 in this Hexagon family.
pub fn encode_skip_absent_bank(pred:u8,site:u32,target:u32)->Result<u32,String>{
    if pred!=0||(site,target)!=(PHYSICAL_ADDRESS+0x24,CONTROLLER_RANGE_FAIL){
        return Err("unverified controller bank fallback branch/target".into());
    }
    let delta=target.checked_sub(site).ok_or("controller branch must be forward")?;
    if delta%2!=0||delta>0x3fe{return Err("controller bank branch displacement outside V66 fields".into());}
    let hw=delta/2;
    Ok(M::ContinuationA.apply(0x5c20_0000|((hw&0x100)<<5)|(hw&0xff)))
}
/// First 48 bytes of the controller bank enumeration. No reference access.
pub fn build_bank_entry()->Result<Vec<u8>,String>{
    let mut v=Vec::with_capacity(EXECUTABLE_LEN);
    let [ext,zero]=cdsp_boot::encode_extended_assignment(r(2)?,0,false);
    put(&mut v,M::ContinuationA.apply(ext));put(&mut v,M::ContinuationA.apply(zero));
    put(&mut v,encode_frame_pair_and_allocation(r(29)?,r(16)?,80)?);
    put(&mut v,M::ContinuationA.apply(S::PredicateP0EqualsZero{src:r(2)?}.encode()?));
    put(&mut v,encode_frame_save_duplex(r(29)?,r(18)?,r(20)?,64,56)?);
    for w in encode_predicated_controller_mask(r(2)?,0,-0x4000_0000)?{put(&mut v,w);}
    put(&mut v,encode_stack_bank_pair(r(29)?,r(22)?,48,false)?);
    put(&mut v,encode_stack_bank_pair(r(29)?,r(24)?,40,true)?);
    put(&mut v,encode_skip_absent_bank(0,PHYSICAL_ADDRESS+0x24,CONTROLLER_RANGE_FAIL)?);
    put(&mut v,encode_stack_bank_pair(r(29)?,r(26)?,32,true)?);
    put(&mut v,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(18)?,src:r(2)?,amount:12}.encode()?));
    if v.len()!=EXECUTABLE_LEN {return Err("bank entry must emit exactly 12 Hexagon words".into());}
    Ok(v)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut b=cdsp_late_controller_record_diagnostics::build_prefix()?;
    b.extend(build_bank_entry()?);
    if b.len()!=FULL_RWE_LAYOUT_LEN||b.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("CDSP bank entry layout and executable accounting differ".into());
    }
    Ok(b)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bank_entry()?.as_slice()){
        return Err("source-generated controller bank entry differs from pinned CDSP reference".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("controller bank entry joined RWE prefix differs from pinned CDSP".into());
    }
    Ok(())
}
#[cfg(test)] mod tests{
    use super::*;
    #[test]fn checked_bank_controller_prologue(){
        let b=build_bank_entry().unwrap();assert_eq!(b.len(),48);
        let w:Vec<u32>=b.chunks_exact(4).map(|x|u32::from_le_bytes(x.try_into().unwrap())).collect();
        assert_eq!(w[0],0x0000_4000);assert_eq!(w[1],0x7800_4002);
        assert_eq!(w[2],0xebf4_1ca0);assert_eq!(w[4],0xea45_0a3e);
        assert_eq!(w[9],0x5c20_607c);assert_eq!(w[11],0x8c02_4c32);
        assert!(encode_frame_pair_and_allocation(r(29).unwrap(),r(16).unwrap(),72).is_err());
        assert!(encode_frame_save_duplex(r(29).unwrap(),r(18).unwrap(),r(22).unwrap(),64,56).is_err());
        assert!(encode_predicated_controller_mask(r(2).unwrap(),1,-0x4000_0000).is_err());
        assert!(encode_stack_bank_pair(r(29).unwrap(),r(22).unwrap(),40,false).is_err());
        assert!(encode_skip_absent_bank(0,PHYSICAL_ADDRESS+0x24,CONTROLLER_RANGE_FAIL+4).is_err());
    }
    #[test]fn bank_entry_is_contiguous_and_fill_is_not_executable(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,2864);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2804);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,60);
        assert_eq!(build_prefix().unwrap().len(),2864);
    }
}
