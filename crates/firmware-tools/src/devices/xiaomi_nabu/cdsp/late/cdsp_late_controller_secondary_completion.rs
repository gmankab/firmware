// SPDX-License-Identifier: GPL-3.0-only
//! Source-encoded V66 controller bit-plane capacity test and command header,
//! CDSP file 0x5680c..0x56848 (60 bytes). The physical jump predicates are
//! built with checked packet-relative offsets and independently assemble
//! byte-for-byte using LLVM6 from an authored Hexagon source fixture.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_secondary_bitloop;

pub const FILE_OFFSET:usize=0x5680c;
pub const END_FILE_OFFSET:usize=0x56848;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_880c;
pub const RWE_START:usize=cdsp_late_controller_secondary_bitloop::RWE_START;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_secondary_bitloop::RWE_PREFIX_LAYOUT_LEN;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_secondary_bitloop::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;

fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// P0 = cmp.gtu(R22,R20); if(P0.new) jump:nt 0x96cc881c.
/// The signed/unsigned predicate shares one original four-word packet with
/// the subsequent `.new` comparison branch at 0x96cc8818.
pub fn encode_mask_gt_branch(left:G,right:G,packet:u32,target:u32)->Result<u32,String>{
    if (left.number(),right.number(),packet,target)!=(22,20,PHYSICAL_ADDRESS,0x96cc_881c){
        return Err("secondary mask predicate operands or target unverified".into());
    }
    let d=target.checked_sub(packet).ok_or("mask predicate must branch forward")?;
    if d<4||d>0x3e ||d&1!=0{return Err("mask predicate displacement invalid".into());}
    Ok(0x150e_cc00|((d-4)>>1))
}

/// Fused comparison against `R3.new`, with target relative to the producer's
/// physical instruction address, and original V66 packet parse marker.
pub fn encode_new_sum_exceeds(dst:G,capacity:G,from:u32,target:u32)->Result<u32,String>{
    if (dst.number(),capacity.number(),from,target)!=(3,20,0x96cc_8814,0x96cc_884c){
        return Err("unsupported summed-size comparator or destination".into());
    }
    let d=target.checked_sub(from).ok_or("sum-bound branch must be forward")?;
    if d>0x1fe ||d&1!=0{return Err("sum-bound displacement out of range".into());}
    Ok(0x2102_f400 | (d>>1))
}

/// Compare fresh R3=#8 against the scan bit capacity and branch to the shared
/// controller-insufficient-capacity diagnostic at physical 0x96cc88c8.
pub fn encode_insufficient_capacity(remaining:G,min_bits:G,from:u32,target:u32)->Result<u32,String>{
    if (remaining.number(),min_bits.number(),from,target)!=(3,2,0x96cc_881c,0x96cc_88c8){
        return Err("secondary bit capacity branch has unsupported operands".into());
    }
    let d=target.checked_sub(from).ok_or("capacity branch must be forward")?;
    if d>0x3fe ||d&1!=0{return Err("capacity branch displacement invalid".into());}
    Ok(0x2142_c200 | (d>>1))
}

pub fn encode_pack_upper_bank(dst:G,source:G,shift:u8)->Result<u32,String>{
    if (dst.number(),source.number(),shift)!=(3,20,12){
        return Err("upper controller field uses R3 |= lsr(R20,12)".into());
    }
    Ok(0x8e40_40a0 | (u32::from(source.number())<<16) |
       (u32::from(shift)<<8) | u32::from(dst.number()))
}

/// Derived from the independently assembled V60 `or(R4,and(R2,imm))`
/// family. These particular registers and mask are checked rather than
/// pretending to implement every V66 nested logical opcode.
pub fn encode_nested_plane_mask(dst:G,mask_source:G,size_source:G,mask:u32)->Result<u32,String>{
    if (dst.number(),mask_source.number(),size_source.number(),mask)!=(2,4,2,0x1f_fffe){
        return Err("unsupported nested per-plane mask/operand tuple".into());
    }
    Ok(0xda40_c3c0 | (u32::from(size_source.number())<<16) |
       (u32::from(mask_source.number())<<8) |u32::from(mask_source.number()))
}

/// Exactly 15 original V66 words for the second scan's guard and command
/// prologue. All immediate and branch destinations are derived from named
/// parameter values; no original-reference file access is used here.
pub fn build_secondary_completion()->Result<Vec<u8>,String>{
    let mut b=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut b,S::AddSignedSmallImmediate{dst:r(2)?,src:r(17)?,value:-31}.encode()?);
    emit(&mut b,encode_mask_gt_branch(r(22)?,r(20)?,PHYSICAL_ADDRESS,0x96cc_881c)?);
    emit(&mut b,M::ContinuationA.apply(S::AddRegisters{dst:r(3)?,left:r(21)?,right:r(22)?}.encode()?));
    emit(&mut b,encode_new_sum_exceeds(r(3)?,r(20)?,0x96cc_8814,0x96cc_884c)?);
    emit(&mut b,M::ContinuationA.apply(S::AssignUnsignedImmediate{dst:r(3)?,value:8}.encode()?));
    emit(&mut b,encode_insufficient_capacity(r(3)?,r(2)?,0x96cc_881c,0x96cc_88c8)?);
    emit(&mut b,M::ContinuationA.apply(S::ArithmeticShiftLeft{dst:r(2)?,src:r(18)?,amount:1}.encode()?));
    let top=cdsp_boot::encode_extended_assignment(r(3)?,0xc000_0000u32 as i32,false);
    emit(&mut b,top[0]);
    emit(&mut b,top[1]);
    emit(&mut b,0x7300_e000|(u32::from(r(17)?.number())<<16)|(1<<5)|u32::from(r(0)?.number()));
    emit(&mut b,encode_pack_upper_bank(r(3)?,r(20)?,12)?);
    let prefix=cdsp_boot::encode_extended_assignment(r(4)?,0x7700_0001,false);
    emit(&mut b,prefix[0]);
    emit(&mut b,prefix[1]|0x8000); // last in packet
    emit(&mut b,cdsp_boot::encode_extended_assignment(r(2)?,0x001f_fffe,false)[0]);
    emit(&mut b,encode_nested_plane_mask(r(2)?,r(4)?,r(2)?,0x1f_fffe)?);
    if b.len()!=EXECUTABLE_LEN{return Err("secondary controller completion must contain 15 instruction words".into());}
    Ok(b)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_secondary_bitloop::build_prefix()?;
    out.extend(build_secondary_completion()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN||out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("secondary completion executable/layout accounting mismatch".into());
    }
    Ok(out)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_secondary_completion()?.as_slice()){
        return Err("secondary controller completion differs from pinned CDSP".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("joined controller completion RWE prefix differs from CDSP".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn completion_checks_original_15_words_and_rejects_invalid_operands(){
        let b=build_secondary_completion().unwrap();assert_eq!(b.len(),60);
        let words:Vec<u32>=b.chunks_exact(4).map(|s|u32::from_le_bytes(s.try_into().unwrap())).collect();
        assert_eq!(words[0],0xbff1_fc22);
        assert_eq!(words[3],0x2102_f41c);
        assert_eq!(words[5],0x2142_c256);
        assert_eq!(words[14],0xda42_c7c4);
        assert!(encode_mask_gt_branch(r(22).unwrap(),r(20).unwrap(),PHYSICAL_ADDRESS,0x96cc_8820).is_err());
        assert!(encode_insufficient_capacity(r(4).unwrap(),r(2).unwrap(),0x96cc_881c,0x96cc_88c8).is_err());
        assert!(encode_nested_plane_mask(r(2).unwrap(),r(4).unwrap(),r(2).unwrap(),0x1f_fffc).is_err());
    }
    #[test]fn completion_appends_to_contiguous_rwe_without_relabeling_nops(){
        assert_eq!(build_prefix().unwrap().len(),0x848);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2076);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,44);
    }
}
