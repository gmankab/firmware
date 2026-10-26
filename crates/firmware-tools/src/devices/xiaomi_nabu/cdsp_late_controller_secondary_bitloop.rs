// SPDX-License-Identifier: GPL-3.0-only
//! CDSP secondary-plane bit scanner, original executable 0x567b4..0x5680c.
//! The first plane scan at 0x566d8 uses R25/R26; this variant uses R26/R27
//! and stores low/high mask state in R19/R24. The source-built instruction
//! encodings, extended immediates and original V66 packet markers are kept
//! separate from any privately held reference file.
use super::cdsp_boot::{self,GeneralRegister as G,ScalarPacket as S,PacketMarker as M};
use super::cdsp_late_controller_mode_guard;

pub const FILE_OFFSET:usize=0x567b4;
pub const END_FILE_OFFSET:usize=0x5680c;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_87b4;
pub const RWE_START:usize=cdsp_late_controller_mode_guard::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_mode_guard::RWE_PREFIX_LAYOUT_LEN;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_mode_guard::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
const PROTOCOL_FLAG:u32=0x7700_0000;
const TOP_FLAG:u32=0xc000_0000;

fn r(i:u8)->Result<G,String>{G::new(i)}
fn emit(v:&mut Vec<u8>,word:u32){v.extend_from_slice(&word.to_le_bytes());}

pub fn encode_mask_pair_or(dst:G,left:G,right:G)->Result<u32,String>{
    if (dst.number(),left.number(),right.number())!=(2,19,24){
        return Err("secondary scan requires R2=or(R19,R24)".into());
    }
    Ok(0xf120_c000 | (u32::from(left.number())<<16) |
       (u32::from(right.number())<<8) | u32::from(dst.number()))
}

pub fn encode_plane_ct0(dst:G,src:G)->Result<u32,String>{
    if (dst.number(),src.number())!=(2,2){return Err("CT0 register must be R2".into());}
    Ok(0x8c40_c080 | (u32::from(src.number())<<16) | u32::from(dst.number()))
}

pub fn encode_minimum_unsigned(dst:G,left:G,right:G)->Result<u32,String>{
    if (dst.number(),left.number(),right.number())!=(4,3,2){
        return Err("secondary scan requires R4=minu(R3,R2)".into());
    }
    Ok(0xd5a0_c080 | (u32::from(right.number())<<16) |
       (u32::from(left.number())<<8) | u32::from(dst.number()))
}

/// `Rn ±= asl(R26,R27)`; original signed/subtract vs accumulate operations
/// use the same general register fields with a checked selector bit.
pub fn encode_plane_accumulator(dst:G,source:G,shift:G,subtract:bool)->Result<u32,String>{
    if (source.number(),shift.number())!=(26,27) ||
        ![(23,true),(24,false),(19,false)].contains(&(dst.number(),subtract)) {
        return Err("unverified secondary plane accumulator registers".into());
    }
    let base=if subtract{0xcc80_c080}else{0xccc0_c080};
    Ok(base | (u32::from(source.number())<<16) |
       (u32::from(shift.number())<<8) | u32::from(dst.number()))
}

/// `R2 |= or(R3,##0x77000000)` is a masked immediate producer followed by
/// an immext that preserves the full 32-bit protocol constant.
pub fn encode_protocol_or(dst:G,src:G,value:u32)->Result<u32,String>{
    if (dst.number(),src.number(),value)!=(2,3,PROTOCOL_FLAG){
        return Err("unsupported secondary protocol bit flag".into());
    }
    Ok(0xda80_c000|(u32::from(src.number())<<16)|u32::from(dst.number()))
}

pub fn encode_top_flag_or(dst:G,src:G,value:u32)->Result<u32,String>{
    if (dst.number(),src.number(),value)!=(3,24,TOP_FLAG){
        return Err("unsupported upper controller header flag".into());
    }
    Ok(0x7680_c000|(u32::from(src.number())<<16)|u32::from(dst.number()))
}

pub fn encode_controller_helper_call(from:u32,to:u32)->Result<u32,String>{
    if (from,to)!=(0x96cc_87f8,0x96dc_0aac){
        return Err("unsupported secondary plane helper physical call".into());
    }
    let delta=to.checked_sub(from).ok_or("helper call unexpectedly backward")?;
    if delta&1!=0 || delta>0x1fff_fe{return Err("helper call displacement outside Hexagon encoding".into());}
    Ok(0x5a00_4000 | (((delta>>15)&0x3f)<<16) | ((delta>>1)&0x3fff))
}

/// Backedge branch is relative to the two-instruction packet beginning at
/// physical 0x96cc8800 (not to the fused comparison word at 0x96cc8808).
pub fn encode_secondary_scan_retry(src:G,packet:u32,to:u32)->Result<u32,String>{
    if (src.number(),packet,to)!=(23,0x96cc_8800,0x96cc_87a8){
        return Err("unverified secondary plane retry packet/target".into());
    }
    let d=i64::from(to)-i64::from(packet);
    if d>=0 || d < -0x100 || d%2!=0{return Err("invalid signed bitloop backedge".into());}
    let low=((d/2) as i32 as u32)&0xff;
    Ok(0x1078_e000 | (u32::from(src.number()&15)<<16) | low)
}

/// Complete 22 original-order V66 instruction words for the second scan.
pub fn build_secondary_bitloop()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_mask_pair_or(r(2)?,r(19)?,r(24)?)?);
    emit(&mut out,M::ContinuationA.apply(S::SetBit{dst:r(2)?,src:r(2)?,position:12}.encode()?));
    emit(&mut out,S::ReverseSubtractImmediate{dst:r(3)?,src:r(3)?,value:31}.encode()?);
    emit(&mut out,encode_plane_ct0(r(2)?,r(2)?)?);
    emit(&mut out,encode_minimum_unsigned(r(4)?,r(3)?,r(2)?)?);
    emit(&mut out,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(2)?,src:r(4)?,amount:1}.encode()?));
    emit(&mut out,S::ClearBit{dst:r(27)?,src:r(4)?,position:0}.encode()?);
    emit(&mut out,M::ContinuationA.apply(encode_plane_accumulator(r(23)?,r(26)?,r(27)?,true)?));
    emit(&mut out,M::ContinuationA.apply(0xc680_c0e0 | (u32::from(r(2)?.number())<<8)));
    emit(&mut out,S::AssignUnsignedImmediate{dst:r(2)?,value:8}.encode()?);
    emit(&mut out,M::ContinuationA.apply(S::InsertBitField{dst:r(2)?,src:r(19)?,width:21,offset:1}.encode()?));
    emit(&mut out,M::ContinuationA.apply(0x7600_c000 | (u32::from(r(0)?.number())<<16) |
                  (255<<5) | u32::from(r(3)?.number())));
    emit(&mut out,0x7300_e000 | (u32::from(r(17)?.number())<<16) | (1<<5) | u32::from(r(0)?.number()));
    emit(&mut out,cdsp_boot::encode_extended_assignment(r(2)?,PROTOCOL_FLAG as i32,false)[0]);
    emit(&mut out,M::ContinuationA.apply(encode_protocol_or(r(2)?,r(3)?,PROTOCOL_FLAG)?));
    emit(&mut out,cdsp_boot::encode_extended_assignment(r(3)?,TOP_FLAG as i32,false)[0]);
    emit(&mut out,encode_top_flag_or(r(3)?,r(24)?,TOP_FLAG)?);
    emit(&mut out,encode_controller_helper_call(0x96cc_87f8,0x96dc_0aac)?);
    emit(&mut out,S::AddSmallImmediate{dst:r(17)?,src:r(17)?,value:1}.encode()?);
    emit(&mut out,M::ContinuationA.apply(encode_plane_accumulator(r(24)?,r(26)?,r(27)?,false)?));
    emit(&mut out,M::ContinuationA.apply(encode_plane_accumulator(r(19)?,r(26)?,r(27)?,false)?));
    emit(&mut out,encode_secondary_scan_retry(r(23)?,0x96cc_8800,0x96cc_87a8)?);
    if out.len()!=EXECUTABLE_LEN{return Err("secondary plane scan must emit exactly 88 bytes".into());}
    Ok(out)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_mode_guard::build_prefix()?;
    out.extend(build_secondary_bitloop()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN || out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN {
        return Err("secondary scan prefix length or code/fill accounting mismatch".into());
    }
    Ok(out)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_secondary_bitloop()?.as_slice()){
        return Err("secondary bitplane scan differs from pinned original".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("joined CDSP secondary scan prefix differs from pinned original".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn secondary_scan_fields_and_branches_are_checked(){
        let out=build_secondary_bitloop().unwrap();assert_eq!(out.len(),88);
        let w:Vec<u32>=out.chunks_exact(4).map(|q|u32::from_le_bytes(q.try_into().unwrap())).collect();
        assert_eq!(w[0],0xf133_d802);
        assert_eq!(w[7],0xcc9a_5b97);
        assert_eq!(w[13],0x0770_4000);
        assert_eq!(w[17],0x5a1f_415a);
        assert_eq!(w[21],0x107f_e0d4);
        assert!(encode_mask_pair_or(r(2).unwrap(),r(22).unwrap(),r(24).unwrap()).is_err());
        assert!(encode_protocol_or(r(2).unwrap(),r(3).unwrap(),0x7000_0000).is_err());
        assert!(encode_secondary_scan_retry(r(23).unwrap(),0x96cc_8800,0x96cc_87ac).is_err());
    }
    #[test]fn secondary_scan_prefix_is_contiguous(){
        assert_eq!(build_prefix().unwrap().len(),0x80c);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2016);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,44);
    }
}
