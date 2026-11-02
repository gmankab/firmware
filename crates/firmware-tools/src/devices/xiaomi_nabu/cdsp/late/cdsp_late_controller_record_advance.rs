// SPDX-License-Identifier: GPL-3.0-only
//! CDSP V66 controller record-scan cursor/return, file [0x56aa8,0x56adc).
//! The encoded branch targets are physically packet-relative. This completes
//! the active indexed-record function, but not the entire CDSP executable.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_record_bounds;
use super::cdsp_late_rng_update::encode_final_seed_return;

pub const FILE_OFFSET:usize=0x56aa8;
pub const END_FILE_OFFSET:usize=0x56adc;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8aa8;
pub const RWE_START:usize=cdsp_late_controller_record_bounds::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_record_bounds::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_record_bounds::RWE_PREFIX_LAYOUT_LEN;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn emit(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes())}

/// V66 duplex `R22=add(R22,#1);R17=memw(R16+16)`, exactly matched by LLVM6.
pub fn encode_advance_cursor_and_load(dst:G,source:G,ptr:G,base:G,offset:u8,step:u8)->Result<u32,String>{
    if (dst.number(),source.number(),ptr.number(),base.number(),offset,step)!=(22,22,17,16,16,1){
        return Err("record cursor advance expects R22++, R17=memw(R16+16)".into());
    }
    Ok(M::ContinuationB.apply(0x508e_0000|(u32::from(dst.number())<<20)|
       (u32::from(offset/4)<<8)|(u32::from(ptr.number()-16)<<7)|
       u32::from(ptr.number()-8)))
}
/// `R2=memw(R2+#12)` with scaled word offset, as produced by LLVM6.
pub fn encode_lookup_tail_word(dst:G,base:G,offset:u8)->Result<u32,String>{
    if (dst.number(),base.number(),offset)!=(2,2,12){
        return Err("indexed controller cursor expected record tail at R2+12".into());
    }
    Ok(M::ContinuationA.apply(0x9180_0000|(u32::from(base.number())<<16)|
       (u32::from(offset/4)<<5)|u32::from(dst.number())))
}
/// Conditional *nonzero* freshly loaded R2 loops to earlier record lookup.
pub fn encode_repeat_if_present(src:G,packet:u32,target:u32)->Result<u32,String>{
    if (src.number(),packet,target)!=(2,0x96cc_8ab4,0x96cc_8a30){
        return Err("unverified controller record traversal backedge".into());
    }
    let delta=i64::from(target)-i64::from(packet);
    if delta>=0 || delta < -256 || delta&1!=0{
        return Err("record traversal requires negative 8-bit halfword branch".into());
    }
    Ok(M::Final.apply(0x2472_2000|(((delta/2) as i32 as u32)&0xff)))
}
/// P0.new false increments the byte-scan index before checking a bounded
/// number of record candidates. LLVM6 confirms this operation independently.
pub fn encode_predicated_scan_increment(dst:G,predicate:u8,step:u8)->Result<u32,String>{
    if (dst.number(),predicate,step)!=(21,0,1){
        return Err("unverified P0 false scan increment".into());
    }
    Ok(M::ContinuationA.apply(0x7480_2000|(u32::from(dst.number())<<16)|
        0x20|u32::from(dst.number())))
}
/// Fused unsigned `P0 = R21 > 30; if(P0.new) jump:nt 0x96cc8ad0`.
/// Original branch fields include the 4-byte packet PC-relative adjustment.
pub fn encode_scan_limit_guard(src:G,limit:u8,packet:u32,target:u32)->Result<u32,String>{
    if (src.number(),limit,packet,target)!=(21,30,0x96cc_8ac0,0x96cc_8ad0){
        return Err("unverified scan count / branch guard".into());
    }
    let dist=target.checked_sub(packet).ok_or("scan limit points backwards")?;
    if dist&1!=0||dist>0x1fc{return Err("scan limit branch exceeds tested width".into());}
    Ok(M::Final.apply(0x1100_1e00|
       (u32::from(src.number()-8)<<16)|((dist+4)>>1)))
}
/// Short signed integer multiply R2=mpyi(R21,#160), V66 scalar DSP encoding.
pub fn encode_scan_stride(dst:G,src:G,mul:u16)->Result<u32,String>{
    if (dst.number(),src.number(),mul)!=(2,21,160){return Err("unverified scan slot size".into());}
    if mul%32!=0{return Err("scan slot multiplier requires 32-byte granularity".into());}
    Ok(M::Final.apply(0xe000_0000 | (u32::from(src.number())<<16)|
       (u32::from(mul/32)<<10)|u32::from(dst.number())))
}
/// V66 memory-byte read via base+index, no implicit unit conversion.
pub fn encode_indexed_scan_byte(dst:G,base:G,index:G)->Result<u32,String>{
    if (dst.number(),base.number(),index.number())!=(2,20,2){
        return Err("unverified controller byte table indexed load".into());
    }
    Ok(M::ContinuationA.apply(0x3a20_0000|
        (u32::from(base.number())<<16)|(u32::from(index.number())<<8)|u32::from(dst.number())))
}
/// R2.new nonzero branches to the earlier byte scan at physical 0x96cc8990.
pub fn encode_retry_scan_byte(src:G,packet:u32,target:u32)->Result<u32,String>{
    if (src.number(),packet,target)!=(2,0x96cc_8ac8,0x96cc_8990){
        return Err("unverified byte-scan loop backedge/producer".into());
    }
    let delta=i64::from(target)-i64::from(packet);
    if delta>=0 || delta < -512 || delta&1!=0{return Err("byte scan retry out of range".into());}
    Ok(M::Final.apply(0x2472_2000|(((delta/2) as i32 as u32)&0xff)))
}
/// Two simultaneously restored doubleword GPRs from the controller frame.
/// Hexagon frame offsets advance in 8-byte units; each source is an even pair.
pub fn encode_duplex_restored_pairs(first:G,second:G,fp:G,first_off:u8,second_off:u8)->Result<u32,String>{
    if fp.number()!=29 || ![(16,18,88,80),(20,22,72,64)].contains(&
         (first.number(),second.number(),first_off,second_off)){
        return Err("record-return paired frame offsets do not match verified layout".into());
    }
    let index=u32::from((first.number()-16)/2);
    let frame_inc=(u32::from(first_off)+8*index-40)/8;
    Ok(M::ContinuationB.apply(0x3e2c_1e25 - index*0x0007_0007 +
                              frame_inc*0x0008_0008))
}

pub fn build_record_advance()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    emit(&mut out,encode_advance_cursor_and_load(r(22)?,r(22)?,r(17)?,r(16)?,16,1)?);
    emit(&mut out,M::ContinuationA.apply(S::AddRegisters{dst:r(2)?,left:r(17)?,right:r(23)?}.encode()?));
    emit(&mut out,S::AddSmallImmediate{dst:r(23)?,src:r(23)?,value:8}.encode()?);
    emit(&mut out,encode_lookup_tail_word(r(2)?,r(2)?,12)?);
    emit(&mut out,encode_repeat_if_present(r(2)?,0x96cc_8ab4,0x96cc_8a30)?);
    emit(&mut out,encode_predicated_scan_increment(r(21)?,0,1)?);
    emit(&mut out,encode_scan_limit_guard(r(21)?,30,0x96cc_8ac0,0x96cc_8ad0)?);
    emit(&mut out,encode_scan_stride(r(2)?,r(21)?,160)?);
    emit(&mut out,encode_indexed_scan_byte(r(2)?,r(20)?,r(2)?)?);
    emit(&mut out,encode_retry_scan_byte(r(2)?,0x96cc_8ac8,0x96cc_8990)?);
    emit(&mut out,encode_duplex_restored_pairs(r(16)?,r(18)?,r(29)?,88,80)?);
    emit(&mut out,encode_duplex_restored_pairs(r(20)?,r(22)?,r(29)?,72,64)?);
    emit(&mut out,encode_final_seed_return(r(31)?,r(29)?)?);
    if out.len()!=EXECUTABLE_LEN{return Err("indexed record cursor/return length !=52 bytes".into());}
    Ok(out)
}

pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut b=cdsp_late_controller_record_bounds::build_prefix()?;
    b.extend(build_record_advance()?);
    if b.len()!=FULL_RWE_LAYOUT_LEN||b.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("record cursor RWE prefix accounting failed".into());
    }
    Ok(b)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_record_advance()?.as_slice()){
        return Err("source-built CDSP indexed record advance differs from reference".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("indexed record advance RWE prefix differs from reference".into());
    }
    Ok(())
}
#[cfg(test)] mod tests{
    use super::*;
    #[test]
    fn thirteen_record_cursor_words_match_original_bit_fields(){
        let b=build_record_advance().unwrap();assert_eq!(b.len(),52);
        let w:Vec<u32>=b.chunks_exact(4).map(|c|u32::from_le_bytes(c.try_into().unwrap())).collect();
        assert_eq!(w,[0x51ee_0489,0xf311_5702,0xb017_c117,0x9182_4062,
              0x2472_e0be,0x7495_6035,0x110d_de0a,0xe015_d402,
              0x3a34_4202,0x2472_e064,0x3e5c_1e55,0x3e4e_1e47,0x961e_c01e]);
        assert!(encode_scan_limit_guard(r(21).unwrap(),32,0x96cc8ac0,0x96cc8ad0).is_err());
        assert!(encode_duplex_restored_pairs(r(16).unwrap(),r(18).unwrap(),r(29).unwrap(),80,72).is_err());
        assert!(encode_retry_scan_byte(r(2).unwrap(),0x96cc8ac8,0x96cc8994).is_err());
    }
    #[test]
    fn prefix_matches_executable_layout_accounting(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,2780);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2724);
        assert_eq!(RWE_PREFIX_LAYOUT_LEN,56);
        assert_eq!(build_prefix().unwrap().len(),2780);
    }
}
