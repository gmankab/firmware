// SPDX-License-Identifier: GPL-3.0-only
//! CDSP Hexagon V66 indexed record-size guards, file [0x56a74,0x56aa8).
//! No original firmware instruction arrays are embedded in this implementation.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_record_lookup;

pub const FILE_OFFSET:usize=0x56a74;
pub const END_FILE_OFFSET:usize=0x56aa8;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8a74;
pub const RWE_START:usize=cdsp_late_controller_record_lookup::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_record_lookup::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_record_lookup::RWE_PREFIX_LAYOUT_LEN;
const SIZE_HELPER:u32=0x96cd_0480;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes())}
/// The V66 two-word-load duplex reads both packed record metadata handles.
/// LLVM6 independently reproduces this exact duplex form. The narrow tuple
/// prevents accidentally applying its bit layout to unrelated registers.
pub fn encode_record_header_duplex(dst_a:G,base_a:G,dst_b:G,base_b:G,offset:u8)->Result<u32,String>{
    if (dst_a.number(),base_a.number(),dst_b.number(),base_b.number(),offset)!=(4,2,5,16,16){
        return Err("record bounds require R4=memw(R2+16), R5=memw(R16+16)".into());
    }
    Ok(M::ContinuationB.apply(0x0400_0000 | (u32::from(base_a.number())<<20)
       | (u32::from(dst_a.number())<<16) | (u32::from(offset/4)<<8)
       | (u32::from(base_b.number()/2)<<4) | u32::from(dst_b.number())))
}
/// Indexed record-size word, verified V66 register-indexed addressing.
pub fn encode_indexed_size(dst:G,base:G,index:G)->Result<u32,String>{
    if (dst.number(),base.number(),index.number())!=(3,5,23){
        return Err("indexed record bounds expect R3 from R5+R23".into());
    }
    Ok(M::Final.apply(0x3a80_0000 | (u32::from(base.number())<<16)
       | (u32::from(index.number())<<8) | u32::from(dst.number())))
}
/// Word load with an aligned scaled offset. Checked for the two concrete
/// record-bound accesses, separately checked against the independent fixture.
pub fn encode_metadata_load(dst:G,base:G,offset:u8,last:bool)->Result<u32,String>{
    if ![(2,4,32),(5,5,4)].contains(&(dst.number(),base.number(),offset)){
        return Err("unexpected record metadata offset or register".into());
    }
    let raw=0x9180_0000 | (u32::from(base.number())<<16)
        | (u32::from(offset/4)<<5) | u32::from(dst.number());
    Ok((if last{M::Final}else{M::ContinuationA}).apply(raw))
}
/// Fused new-value unsigned greater-than guard, measured from first word
/// of the *producer packet*, not the consumer's own instruction address.
pub fn encode_new_unsigned_overflow(lhs:G,new_rhs:G,packet:u32,target:u32)->Result<u32,String>{
    if ![(3,2,0x96cc_8a7c),(6,3,0x96cc_8a94)]
        .contains(&(lhs.number(),new_rhs.number(),packet)) || target!=0x96cc_8aa8{
        return Err("unverified fused record-size new-value guard".into());
    }
    let delta=target.checked_sub(packet).ok_or("record bounds branch goes backwards")?;
    if delta&1!=0||delta>0x1fe{return Err("new-value branch displacement too large".into());}
    Ok(M::Final.apply(0x2202_2000|(u32::from(lhs.number())<<8)|(delta>>1)))
}
/// The original duplex performs `R5=add(R5,R23)` and `R4=memw(R4+36)`.
pub fn encode_move_cursor_and_load(cursor:G,index:G,ptr:G,offset:u8)->Result<u32,String>{
    if (cursor.number(),index.number(),ptr.number(),offset)!=(5,23,4,36){
        return Err("record cursor duplex expects (R5,R23,R4,+36)".into());
    }
    Ok(M::ContinuationB.apply(0x5800_0000|
        (u32::from(index.number()-8)<<20)|(u32::from(cursor.number())<<16)|
        (u32::from(offset/4)<<8)|(u32::from(ptr.number())<<4)|u32::from(ptr.number())))
}
/// `Rd += lsr(Rsrc,#12)` uses the scalar DSP-extension accumulation field.
pub fn encode_add_shifted_record_size(dst:G,src:G,bits:u8)->Result<u32,String>{
    if ![(6,4,12),(3,5,12)].contains(&(dst.number(),src.number(),bits)){
        return Err("record size add-shift must use original registers/shift".into());
    }
    Ok(M::ContinuationA.apply(0x8e00_40a0 |
       (u32::from(src.number())<<16) | (u32::from(bits)<<8) | u32::from(dst.number())))
}
/// Packet-relative Hexagon call to the shared controller size helper.
pub fn encode_record_size_call(packet:u32,target:u32)->Result<u32,String>{
    if (packet,target)!=(0x96cc_8a9c,SIZE_HELPER){return Err("unexpected size-helper call".into());}
    let delta=target.checked_sub(packet).ok_or("size helper behind packet")?;
    if delta&1!=0||delta>0x1fff_fe{return Err("size-helper displacement unsupported".into());}
    Ok(M::ContinuationA.apply(0x5a00_0000|(((delta>>15)&0x3f)<<16)|((delta>>1)&0x3fff)))
}
/// Final V66 duplex `R3=#8; R0=R17`, confirmed exactly by LLVM6 fixture.
pub fn encode_error_argument_duplex(code:G,code_val:u8,dst:G,src:G)->Result<u32,String>{
    if (code.number(),code_val,dst.number(),src.number())!=(3,8,0,17){
        return Err("invalid record-size fallback argument tuple".into());
    }
    Ok(M::ContinuationB.apply(0x2880_0000|
       (u32::from(code.number())<<16)|(u32::from(code_val/8)<<12)|
       (u32::from(src.number()-16)<<13)|(u32::from(src.number()-8)<<4)))
}

pub fn build_record_bounds()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_record_header_duplex(r(4)?,r(2)?,r(5)?,r(16)?,16)?);
    emit(&mut out,encode_indexed_size(r(3)?,r(5)?,r(23)?)?);
    emit(&mut out,encode_metadata_load(r(2)?,r(4)?,32,false)?);
    emit(&mut out,encode_new_unsigned_overflow(r(3)?,r(2)?,0x96cc_8a7c,0x96cc_8aa8)?);
    emit(&mut out,M::ContinuationA.apply(S::AssignRegister{dst:r(6)?,src:r(2)?}.encode()?));
    emit(&mut out,encode_move_cursor_and_load(r(5)?,r(23)?,r(4)?,36)?);
    emit(&mut out,encode_add_shifted_record_size(r(6)?,r(4)?,12)?);
    emit(&mut out,encode_metadata_load(r(5)?,r(5)?,4,true)?);
    emit(&mut out,encode_add_shifted_record_size(r(3)?,r(5)?,12)?);
    emit(&mut out,encode_new_unsigned_overflow(r(6)?,r(3)?,0x96cc_8a94,0x96cc_8aa8)?);
    emit(&mut out,encode_record_size_call(0x96cc_8a9c,SIZE_HELPER)?);
    emit(&mut out,M::ContinuationA.apply(S::LogicalShiftRight{dst:r(1)?,src:r(4)?,amount:12}.encode()?));
    emit(&mut out,encode_error_argument_duplex(r(3)?,8,r(0)?,r(17)?)?);
    if out.len()!=EXECUTABLE_LEN{return Err("indexed record size guards are not 13 Hexagon words".into());}
    Ok(out)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut b=cdsp_late_controller_record_lookup::build_prefix()?;
    b.extend(build_record_bounds()?);
    if b.len()!=FULL_RWE_LAYOUT_LEN||b.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("record size verifier accounting differs from prefix".into());
    }
    Ok(b)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_record_bounds()?.as_slice()){
        return Err("CDSP indexed record bounds do not match reference".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("CDSP indexed record bounds: prefix differs from pinned image".into());
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn record_bounds_encode_checked_packets(){
        let b=build_record_bounds().unwrap();assert_eq!(b.len(),52);
        let words:Vec<u32>=b.chunks_exact(4).map(|c|u32::from_le_bytes(c.try_into().unwrap())).collect();
        assert_eq!(words,[0x0424_0485,0x3a85_d703,0x9184_4102,0x2202_e316,
            0x7062_4006,0x58f5_0944,0x8e04_4ca6,0x9185_c025,
            0x8e05_4ca3,0x2202_e60a,0x5a00_7cf2,0x8c04_4c21,0x2883_3090]);
        assert!(encode_metadata_load(r(2).unwrap(),r(4).unwrap(),24,false).is_err());
        assert!(encode_new_unsigned_overflow(r(6).unwrap(),r(3).unwrap(),
                    0x96cc8a94,0x96cc8aac).is_err());
        assert!(encode_record_size_call(0x96cc8a98,SIZE_HELPER).is_err());
    }
    #[test] fn prefix_tracks_source_executable_not_alignment(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,2728);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2672);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,56);
        assert_eq!(build_prefix().unwrap().len(),2728);
    }
}
