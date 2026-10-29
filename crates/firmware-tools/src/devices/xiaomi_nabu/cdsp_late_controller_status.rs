// SPDX-License-Identifier: GPL-3.0-only
//! Source-built V66 controller status forwarding and loop setup, CDSP file
//! `[0x569fc,0x56a30)`; byte-identical checking uses the original privately.
use super::cdsp_boot::{self,GeneralRegister as G,PacketMarker as M,ScalarPacket as S};
use super::cdsp_late_controller_byte_scan;
pub const FILE_OFFSET:usize=0x569fc;
pub const END_FILE_OFFSET:usize=0x56a30;
pub const EXECUTABLE_LEN:usize=END_FILE_OFFSET-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96cc_89fc;
pub const RWE_START:usize=cdsp_late_controller_byte_scan::RWE_START;
pub const FULL_RWE_LAYOUT_LEN:usize=END_FILE_OFFSET-RWE_START;
pub const FULL_RWE_EXECUTABLE_LEN:usize=cdsp_late_controller_byte_scan::FULL_RWE_EXECUTABLE_LEN+EXECUTABLE_LEN;
pub const RWE_PREFIX_LAYOUT_LEN:usize=cdsp_late_controller_byte_scan::RWE_PREFIX_LAYOUT_LEN;
const GP_STATUS:u32=0xc028_c0f0;
const TRACE_LITERAL:u32=0xc022_61e1;
const TRACE_CALLBACK:u32=0x96dd_f2c0;
const FLAG_SLOT:u32=0xc036_415c;
fn r(n:u8)->Result<G,String>{G::new(n)}
fn put(b:&mut Vec<u8>,w:u32){b.extend_from_slice(&w.to_le_bytes())}
/// `R2=lsr(R2,#12)`, checked through public scalar architecture operation.
pub fn encode_shift_status(dst:G,src:G,shift:u8)->Result<u32,String>{
    if (dst.number(),src.number(),shift)!=(2,2,12){return Err("unverified controller status normalization".into());}
    S::LogicalShiftRight{dst,src,amount:shift}.encode()
}
/// `memw(R17+32)=R2.new`: new-value slot is a packet distance, not R2.
pub fn encode_status_new_store(base:G,src:G,offset:u8,distance:u8)->Result<u32,String>{
    if (base.number(),src.number(),offset,distance)!=(17,2,32,2){
        return Err("status new-value slot operands not confirmed".into());
    }
    Ok(M::Final.apply(0xa1a0_d000|(u32::from(base.number())<<16)
        |(u32::from(distance)<<8)|u32::from(offset/4)))
}
/// Physical call from a single-instruction packet. CALL-relative halfwords
/// are distributed between low fourteen and extended high six bits.
pub fn encode_scan_finish_call(packet:u32,target:u32)->Result<u32,String>{
    if (packet,target)!=(0x96cc_8a0c,TRACE_CALLBACK){
        return Err("unverified status forwarding call/target".into());
    }
    let d=target.checked_sub(packet).ok_or("negative service displacement")?;
    if d&1!=0||d>0x1fff_fe{return Err("service displacement out of range".into());}
    Ok(M::ContinuationA.apply(0x5a00_0000|(((d>>15)&0x3f)<<16)|((d>>1)&0x3fff)))
}
/// Extended `R1:0=combine(##0xc02261e1,R17)`; the source literal address
/// is split across the extender and consumer encoding.
pub fn encode_combined_trace(low:G,high:G,literal:u32)->Result<[u32;2],String>{
    if (low.number(),high.number(),literal)!=(0,17,TRACE_LITERAL){
        return Err("unverified controller trace combine/word".into());
    }
    let [ext,_]=cdsp_boot::encode_extended_assignment(low,literal as i32,false);
    let body=M::Final.apply(0x7320_2000|(u32::from(high.number())<<16)
        |((literal&0x3f)<<5)|u32::from(low.number()));
    Ok([M::ContinuationA.apply(ext),body])
}
/// V66 duplex: compare R0 with zero, and read R17+36 into R2.
pub fn encode_cursor_flag_duplex(pred:u8,source:G,dst:G,base:G,offset:u8)->Result<u32,String>{
    if (pred,source.number(),dst.number(),base.number(),offset)!=(0,0,2,17,36){
        return Err("unverified cursor status duplex inputs".into());
    }
    Ok(M::ContinuationB.apply(0x5900_0900|(u32::from(offset/4)<<4)|u32::from(dst.number())))
}
/// Compound P1.new jump when the second descriptor is empty.
pub fn encode_guarded_second_skip(value:G,pred:u8,packet:u32,target:u32)->Result<u32,String>{
    if (value.number(),pred,packet,target)!=(2,1,0x96cc_8a1c,0x96cc_8abc){
        return Err("unverified second-descriptor skip".into());
    }
    let d=target.checked_sub(packet).ok_or("guarded skip negative")?;
    if d&1!=0||d>0x1fe{return Err("descriptor jump exceeds checked width".into());}
    Ok(M::ContinuationA.apply(0x1202_0000|(d>>1)))
}
/// P0-guarded absolute word store at controller flag slot. The low bits of
/// the address are represented as an aligned word offset in the consumer.
pub fn encode_conditional_global_store(src:G,pred:u8,address:u32)->Result<[u32;2],String>{
    if (src.number(),pred,address)!=(16,0,FLAG_SLOT){
        return Err("unverified controller status global store".into());
    }
    let [ext,_]=cdsp_boot::encode_extended_assignment(src,address as i32,false);
    let body=M::Final.apply(0xaf80_9000|(u32::from(src.number())<<12)|(((address&0x3f)/4)<<5));
    Ok([M::ContinuationA.apply(ext),body])
}
/// Duplex pair R23:22=combine(32,0), validated by LLVM6 V60 encoding.
pub fn encode_clear_scan_pair(low:G,high:u8,lower:u8)->Result<u32,String>{
    if (low.number(),high,lower)!=(22,32,0){return Err("unverified state loop pair initialization".into());}
    Ok(M::ContinuationA.apply(0x7c00_0000|(u32::from(high)<<5)|u32::from(low.number())))
}

pub fn build_controller_status()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    for (i,w) in cdsp_boot::encode_extended_gp_word_load(r(2)?,GP_STATUS).into_iter().enumerate(){
        put(&mut out,if i==0{M::ContinuationA.apply(w)}else{M::Final.apply(w)});
    }
    put(&mut out,M::ContinuationA.apply(encode_shift_status(r(2)?,r(2)?,12)?));
    put(&mut out,encode_status_new_store(r(17)?,r(2)?,32,2)?);
    put(&mut out,encode_scan_finish_call(0x96cc_8a0c,TRACE_CALLBACK)?);
    for w in encode_combined_trace(r(0)?,r(17)?,TRACE_LITERAL)?{put(&mut out,w);}
    put(&mut out,encode_cursor_flag_duplex(0,r(0)?,r(2)?,r(17)?,36)?);
    put(&mut out,encode_guarded_second_skip(r(2)?,1,0x96cc_8a1c,0x96cc_8abc)?);
    for w in encode_conditional_global_store(r(16)?,0,FLAG_SLOT)?{put(&mut out,w);}
    put(&mut out,encode_clear_scan_pair(r(22)?,32,0)?);
    put(&mut out,S::Nop.encode()?);
    if out.len()!=EXECUTABLE_LEN{return Err("controller status region does not contain exactly 13 words".into());}
    Ok(out)
}
pub fn build_prefix()->Result<Vec<u8>,String>{
    let mut out=cdsp_late_controller_byte_scan::build_prefix()?;
    out.extend(build_controller_status()?);
    if out.len()!=FULL_RWE_LAYOUT_LEN||out.len()!=FULL_RWE_EXECUTABLE_LEN+RWE_PREFIX_LAYOUT_LEN{
        return Err("CDSP controller status and layout accounting failed".into());
    }
    Ok(out)
}
pub fn verify(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(build_controller_status()?.as_slice()){
        return Err("CDSP status output differs from original reference".into());
    }
    Ok(())
}
pub fn verify_prefix(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    if reference.get(RWE_START..END_FILE_OFFSET)!=Some(build_prefix()?.as_slice()){
        return Err("CDSP status prefix differs from original reference".into());
    }
    Ok(())
}
#[cfg(test)] mod tests{
    use super::*;
    #[test]fn controller_status_data_fields_are_typed(){
        let b=build_controller_status().unwrap();assert_eq!(b.len(),52);
        let w:Vec<u32>=b.chunks_exact(4).map(|w|u32::from_le_bytes(w.try_into().unwrap())).collect();
        assert_eq!(w,[0x0c02_6303,0x4980_c602,0x8c02_4c22,0xa1b1_d208,
           0x5a22_745a,0x0c02_4987,0x7331_e420,0x5900_0992,
           0x1202_4050,0x0c03_5905,0xaf81_d0e0,0x7c00_4416,0x7f00_c000]);
        assert!(encode_combined_trace(r(0).unwrap(),r(16).unwrap(),TRACE_LITERAL).is_err());
        assert!(encode_scan_finish_call(0x96cc8a10,TRACE_CALLBACK).is_err());
        assert!(encode_guarded_second_skip(r(2).unwrap(),1,0x96cc8a1c,0x96cc8aa8).is_err());
    }
    #[test]fn controller_status_prefix_has_exact_accounting(){
        assert_eq!(FULL_RWE_LAYOUT_LEN,2608);
        assert_eq!(FULL_RWE_EXECUTABLE_LEN,2552);
        assert_eq!(build_prefix().unwrap().len(),2608);
    }
}
