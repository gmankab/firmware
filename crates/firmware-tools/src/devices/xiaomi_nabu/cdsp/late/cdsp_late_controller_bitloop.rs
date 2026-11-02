// SPDX-License-Identifier: GPL-3.0-only
//! Start of the large-controller bit-plane iterator, CDSP file 0x566c0.
//! Source-encoded Hexagon V66 packets compute the current word-plane size,
//! terminate if the active stream is empty and guard a later MMIO loop.
//! The complete loop and function epilogue remain to be reconstructed.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_transfer;

pub const FILE_OFFSET:usize=0x566c0;
pub const END_FILE_OFFSET:usize=0x56734;
pub const ENTRY_EXECUTABLE_LEN:usize=0x18;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_86c0;
pub const RWE_START:usize=0x56000;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_transfer::RWE_PREFIX_CODE_LEN+EXECUTABLE_LEN;
pub const RWE_LAYOUT_NOP_LEN:usize=cdsp_late_controller_transfer::RWE_PREFIX_LAYOUT_LEN;

fn r(i:u8)->Result<G,String>{G::new(i)}
fn emit(v:&mut Vec<u8>,w:u32){v.extend_from_slice(&w.to_le_bytes());}

/// Original low-register duplex pair `R25:24=combine(#1,#7)`. The checked
/// operands are assembled from two signed halfword values, not copied data.
pub fn encode_plane_size_pair(low:G,high:i8,low_value:i8)->Result<u32,String>{
    if low.number()!=24||high!=1||low_value!=7{
        return Err("controller uses R25:24=combine(1,7)".into());
    }
    Ok(0x7c03_6000 | (u32::from(low_value as u8)<<3) | u32::from(low.number()))
}

/// `P0=cmp.eq(R17,#0); if(P0.new) jump:nt skip` in a single fused V66
/// instruction, with immediate branch displacement including the 8-byte
/// advance implicit in this original decode family. Physical targets were
/// verified with Rizin against the complete original stream.
pub fn encode_skip_if_empty(src:G,packet:u32,target:u32)->Result<u32,String>{
    if src.number()!=17 || packet!=PHYSICAL_ADDRESS+8 || target!=0x96cc_8734{
        return Err("unsupported bitloop empty-stream predicate/target".into());
    }
    let d=target.checked_sub(packet).ok_or("empty stream branch must be forward")?;
    if d==0||d>0x1f8||d%2!=0{return Err("empty stream branch displacement too large".into());}
    Ok(0x1009_c000 | ((d+8)>>1))
}

/// `R3=cl0(R17)`; only the witnessed general-register form is accepted.
pub fn encode_count_leading_zeros(dst:G,src:G)->Result<u32,String>{
    if dst.number()!=3||src.number()!=17{return Err("bitloop CL0 requires R3,R17".into());}
    Ok(0x8c00_40a0 | (u32::from(src.number())<<16) | u32::from(dst.number()))
}

/// Reject too-small output bitplane: `if(!cmp.gtu(R24,R2.new)) jump:nt`.
/// The branch consumes R2.new immediately after the negative add.
pub fn encode_if_insufficient_bits(reg:G,packet:u32,target:u32)->Result<u32,String>{
    if reg.number()!=24||packet!=PHYSICAL_ADDRESS+0x14||target!=0x96cc_8890{
        return Err("controller bit capacity branch has unsupported source/target".into());
    }
    let d=target.checked_sub(packet).ok_or("bit capacity branch must be forward")?;
    if d==0||d>0x3f8||d%2!=0{return Err("bit capacity branch out of range".into());}
    Ok(0x2242_d800 | ((d+8)>>1))
}

pub fn build_bitloop_prefix()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_plane_size_pair(r(24)?,1,7)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignUnsignedImmediate{dst:r(18)?,value:16}.encode()?));
    emit(&mut out,encode_skip_if_empty(r(17)?,PHYSICAL_ADDRESS+8,0x96cc_8734)?);
    emit(&mut out,encode_count_leading_zeros(r(3)?,r(17)?)?);
    emit(&mut out,M::ContinuationA.apply(S::AddSignedSmallImmediate{dst:r(2)?,src:r(18)?,value:-16}.encode()?));
    emit(&mut out,encode_if_insufficient_bits(r(24)?,PHYSICAL_ADDRESS+0x14,0x96cc_8890)?);
    if out.len()!=ENTRY_EXECUTABLE_LEN{return Err("bitloop entry must contain six V66 instructions".into());}
    Ok(out)
}

/// Controller `Rn=or(Rleft,Rright)` as independently LLVM6-assembled
/// V66 scalar register operation. Only the verified mask-scan operands work.
pub fn encode_register_or(dst:G,left:G,right:G)->Result<u32,String>{
    if (dst.number(),left.number(),right.number())!=(2,22,23){
        return Err("bitplane mask scan expects R2=or(R22,R23)".into());
    }
    Ok(0xf120_c000 | (u32::from(left.number())<<16) |
       (u32::from(right.number())<<8) | u32::from(dst.number()))
}

pub fn encode_ct0(dst:G,src:G)->Result<u32,String>{
    if (dst.number(),src.number())!=(2,2){return Err("bitplane trailing zero counter uses R2".into());}
    Ok(0x8c40_c080 | (u32::from(src.number())<<16) | u32::from(dst.number()))
}

pub fn encode_minimum_unsigned(dst:G,left:G,right:G)->Result<u32,String>{
    if (dst.number(),left.number(),right.number())!=(4,3,2){
        return Err("controller minimum must be unsigned R4=minu(R3,R2)".into());
    }
    Ok(0xd5a0_c080 | (u32::from(right.number())<<16) |
       (u32::from(left.number())<<8) | u32::from(dst.number()))
}

/// `R17 -= asl(R25,R26)` and `R23/R22 += asl(R25,R26)`. This
/// arithmetic-shift register family is independently emitted by LLVM6.
pub fn encode_shifted_accumulator(dst:G,left:G,shift:G,subtract:bool)->Result<u32,String>{
    if (left.number(),shift.number())!=(25,26) ||
       ![(17,true),(23,false),(22,false)].contains(&(dst.number(),subtract)){
        return Err("unsupported shifted bitplane accumulator registers or sign".into());
    }
    let base=if subtract {0xcc80_c080} else {0xccc0_c080};
    Ok(base | (u32::from(left.number())<<16) |
       (u32::from(shift.number())<<8) | u32::from(dst.number()))
}

pub fn encode_left_shift_constant(dst:G,value:u8,shift:G)->Result<u32,String>{
    if (dst.number(),value,shift.number())!=(0,1,2){
        return Err("controller requires R0=lsl(#1,R2)".into());
    }
    Ok(0xc680_c0e0 | (u32::from(shift.number())<<8))
}

/// High-precision producer `R2=and(##0x3ffffe,asl(R2,#1))`.
/// Checked narrow fields correspond to an independently assembled LLVM6
/// instruction; its separate `immext` is constructed from the named value.
pub fn encode_and_shifted_extended(dst:G,src:G,amount:u8,mask:u32)->Result<u32,String>{
    if (dst.number(),src.number(),amount,mask)!=(2,2,1,0x003f_fffe){
        return Err("bitplane mask requires R2=and(0x3ffffe,asl(R2,1))".into());
    }
    Ok(0xde22_e1e0)
}

pub fn encode_byte_mask(dst:G,src:G,mask:u16)->Result<u32,String>{
    if (dst.number(),src.number(),mask)!=(3,0,255){
        return Err("bitplane flag byte mask requires R3=and(R0,255)".into());
    }
    Ok(0x7600_c000 | (u32::from(src.number())<<16) |
       (u32::from(mask)<<5) | u32::from(dst.number()))
}

pub fn encode_combine_with_one(low:G,source:G,value:u8)->Result<u32,String>{
    if (low.number(),source.number(),value)!=(0,18,1){
        return Err("bitplane protocol combines R18 and #1 into R1:0".into());
    }
    Ok(0x7300_e000 | (u32::from(source.number())<<16) |
       (u32::from(value)<<5) | u32::from(low.number()))
}

/// `R2 |= or(R3,##0x70000000)` / `R3=or(R23,##0xc0000000)`.
/// Both use independent LLVM-validated body fields plus an immext packet.
pub fn encode_extended_or(dst:G,src:G,constant:u32,accumulate:bool)->Result<u32,String>{
    match (dst.number(),src.number(),constant,accumulate){
        (2,3,0x7000_0000,true) =>
            Ok(0xda80_c000 | (u32::from(src.number())<<16) | u32::from(dst.number())),
        (3,23,0xc000_0000,false) =>
            Ok(0x7680_c000 | (u32::from(src.number())<<16) | u32::from(dst.number())),
        _=>Err("unsupported bitplane protocol header operand combination".into())
    }
}

/// Original direct call to hardware bitplane helper `0x96dc0aac`.
pub fn encode_bitplane_service_call(from:u32,target:u32)->Result<u32,String>{
    if (from,target)!=(0x96cc_8720,0x96dc_0aac){
        return Err("unverified bitplane MMIO helper source or target".into());
    }
    let bytes=target.checked_sub(from).ok_or("bitplane call is backwards")?;
    if bytes&1!=0 || bytes>0x1fff_fe {return Err("bitplane call displacement invalid".into());}
    Ok(0x5a00_4000 | (((bytes>>15)&0x3f)<<16) | ((bytes/2)&0x3fff))
}

/// Fused `P0=cmp.eq(R17,#0); if(!P0.new) jump:t` to controller loop.
/// Hexagon branch distance is relative to *packet start* `0x96cc8728`,
/// not the final fused comparator word at `0x96cc8730`.
pub fn encode_repeat_until_drained(src:G,packet:u32,target:u32)->Result<u32,String>{
    if src.number()!=17||packet!=0x96cc_8728||target!=0x96cc_86cc{
        return Err("bitplane retry source or destination differs".into());
    }
    let bytes=i64::from(target)-i64::from(packet);
    if bytes>=0 || bytes < -0x100 || bytes%2!=0 {
        return Err("bitplane short retry displacement outside signed width".into());
    }
    let half=(bytes/2) as i32 as u32;
    Ok(0x1079_e000 | (half&0xff))
}

/// The 23 source-encoded instruction words of the first bit-plane scan loop.
/// No private reference bytes are read by this emitter. Packet parse markers
/// are preserved explicitly rather than inferred from LLVM6's scheduler.
pub fn build_bitloop_body()->Result<Vec<u8>,String>{
    let mut v=Vec::with_capacity(0x5c);
    emit(&mut v,encode_register_or(r(2)?,r(22)?,r(23)?)?);
    emit(&mut v,M::ContinuationA.apply(S::SetBit{dst:r(2)?,src:r(2)?,position:12}.encode()?));
    emit(&mut v,S::ReverseSubtractImmediate{dst:r(3)?,src:r(3)?,value:31}.encode()?);
    emit(&mut v,encode_ct0(r(2)?,r(2)?)?);
    emit(&mut v,encode_minimum_unsigned(r(4)?,r(3)?,r(2)?)?);
    emit(&mut v,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(2)?,src:r(4)?,amount:1}.encode()?));
    emit(&mut v,S::ClearBit{dst:r(26)?,src:r(4)?,position:0}.encode()?);
    emit(&mut v,M::ContinuationA.apply(encode_shifted_accumulator(r(17)?,r(25)?,r(26)?,true)?));
    emit(&mut v,M::ContinuationA.apply(encode_left_shift_constant(r(0)?,1,r(2)?)?));
    emit(&mut v,S::AssignRegister{dst:r(2)?,src:r(22)?}.encode()?);
    let mask=0x003f_fffe;
    emit(&mut v,cdsp_boot::encode_extended_assignment(r(2)?,mask as i32,false)[0]);
    emit(&mut v,M::ContinuationA.apply(encode_and_shifted_extended(r(2)?,r(2)?,1,mask)?));
    emit(&mut v,M::ContinuationA.apply(encode_byte_mask(r(3)?,r(0)?,255)?));
    emit(&mut v,encode_combine_with_one(r(0)?,r(18)?,1)?);
    let flag=0x7000_0000;
    emit(&mut v,cdsp_boot::encode_extended_assignment(r(2)?,flag as i32,false)[0]);
    emit(&mut v,M::ContinuationA.apply(encode_extended_or(r(2)?,r(3)?,flag,true)?));
    let upper=0xc000_0000;
    emit(&mut v,cdsp_boot::encode_extended_assignment(r(3)?,upper as i32,false)[0]);
    emit(&mut v,encode_extended_or(r(3)?,r(23)?,upper,false)?);
    emit(&mut v,encode_bitplane_service_call(0x96cc_8720,0x96dc_0aac)?);
    emit(&mut v,S::AddSmallImmediate{dst:r(18)?,src:r(18)?,value:1}.encode()?);
    emit(&mut v,M::ContinuationA.apply(encode_shifted_accumulator(r(23)?,r(25)?,r(26)?,false)?));
    emit(&mut v,M::ContinuationA.apply(encode_shifted_accumulator(r(22)?,r(25)?,r(26)?,false)?));
    emit(&mut v,encode_repeat_until_drained(r(17)?,0x96cc_8728,0x96cc_86cc)?);
    if v.len()!=0x5c{return Err("bitplane main loop must reconstruct 92 executable bytes".into());}
    Ok(v)
}

/// Source-only fragment for all decoded bitplane entry and primary loop.
pub fn build_bitloop_full()->Result<Vec<u8>,String>{
    let mut out=build_bitloop_prefix()?;
    out.extend(build_bitloop_body()?);
    if out.len()!=EXECUTABLE_LEN {return Err("bitloop full fragment length mismatch".into());}
    Ok(out)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_transfer::build_prefix()?;
    out.extend(build_bitloop_full()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN || out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_LAYOUT_NOP_LEN{
        return Err("bitloop source code/fill bookkeeping mismatch".into());
    }
    Ok(out)
}

pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_bitloop_full()?.as_slice()){
        return Err("bitloop prefix bytes differ from pinned CDSP original".into());
    }
    Ok(())
}

pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("joined RWE bitloop layout differs from pinned original".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn bitloop_fields_and_branches_are_checked(){
        let bytes=build_bitloop_prefix().unwrap();
        assert_eq!(bytes.len(),ENTRY_EXECUTABLE_LEN);
        let words:Vec<u32>=bytes.chunks_exact(4).map(|w|u32::from_le_bytes(w.try_into().unwrap())).collect();
        assert_eq!(words,[0x7c03_6038,0x7800_4212,0x1009_c03a,0x8c11_40a3,0xbff2_7e02,0x2242_d8e2]);
        assert!(encode_plane_size_pair(r(24).unwrap(),2,7).is_err());
        assert!(encode_skip_if_empty(r(17).unwrap(),PHYSICAL_ADDRESS+8,0x96cc_8738).is_err());
        assert!(encode_count_leading_zeros(r(2).unwrap(),r(17).unwrap()).is_err());
        assert!(encode_if_insufficient_bits(r(24).unwrap(),PHYSICAL_ADDRESS+0x14,0x96cc_8894).is_err());
    }
    #[test]
    fn bitloop_body_enforces_instructions_and_original_packet_boundaries(){
        let body=build_bitloop_body().unwrap();
        assert_eq!(body.len(),0x5c);
        let w:Vec<u32>=body.chunks_exact(4).map(|q|u32::from_le_bytes(q.try_into().unwrap())).collect();
        assert_eq!(w[0],0xf136_d702);
        assert_eq!(w[1],0x8cc2_4c02);
        assert_eq!(w[7],0xcc99_5a91);
        assert_eq!(w[10],0x0003_7fff);
        assert_eq!(w[14],0x0700_4000);
        assert_eq!(w[16],0x0c00_4000);
        assert_eq!(w[18],0x5a1f_41c6);
        assert_eq!(w[22],0x1079_e0d2);
        assert!(encode_register_or(r(2).unwrap(),r(21).unwrap(),r(23).unwrap()).is_err());
        assert!(encode_bitplane_service_call(0x96cc_8720,0x96dc_0ab0).is_err());
        assert!(encode_repeat_until_drained(r(17).unwrap(),0x96cc_8728,0x96cc_86d0).is_err());
        assert!(encode_extended_or(r(3).unwrap(),r(23).unwrap(),0xc000_0001,false).is_err());
    }
    #[test]fn prefix_counts_layout_fill_separately(){
        assert_eq!(build_prefix().unwrap().len(),0x734);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,1800);
        assert_eq!(RWE_LAYOUT_NOP_LEN,44);
    }
}
