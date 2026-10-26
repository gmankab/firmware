// SPDX-License-Identifier: GPL-3.0-only
//! Worker state command generation after the ownership predicate gate.
//! Uses typed V66 insert/setbit operations, a coupled word move, extended
//! memory-mapped status write, cache clean and packet-relative callbacks.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::{cdsp_early_controllers, cdsp_early_worker_decision};

pub const FILE_OFFSET:usize=0x4e28;
pub const REGION_END:usize=0x4e74;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1e28;
fn put(out:&mut Vec<u8>,word:u32,parse:P){out.extend_from_slice(&parse.apply(word).to_le_bytes());}

/// Compact coalesced `Rtmp=memw(RsrcBase+srcOffset);
/// memw(RdstBase+dstOffset)=Rtmp` (one four-byte Hexagon instruction).
/// Field positions are checked against varied LLVM6 register/offset samples.
fn relay_word(src_base:Gpr,src_offset:u8,dst_base:Gpr,dst_offset:u8,tmp:Gpr)->Result<u32,String>{
    if !(8..=23).contains(&src_base.number())||src_offset>4||src_offset%4!=0 ||
        dst_base.number()>7||dst_offset>4||dst_offset%4!=0||tmp.number()>7 {
        return Err("V66 coalesced relay addressing exceeds checked compact form".into());
    }
    Ok(0x8000_0000 | (u32::from(src_offset/4)<<24) |
       (u32::from(src_base.number()-8)<<20) | (u32::from(tmp.number())<<16)|
       (u32::from(dst_offset/4)<<8) | (u32::from(dst_base.number())<<4)|u32::from(tmp.number()))
}
pub(super) fn direct_call(start:u32,target:u32)->Result<u32,String>{
    if start%4!=0||target%4!=0{return Err("Hexagon short CALL address must align".into());}
    let delta=i64::from(target)-i64::from(start);
    if delta%2!=0 || !(-0x100000..=0xffffc).contains(&delta) {
        return Err("Hexagon CALL displacement outside verified signed range".into());
    }
    let half=(delta/2) as i32 as u32;
    Ok(0x5a00_c000|((half>>14&0xff)<<16)|(half&0x3fff))
}
/// Observed coupled producer/consumer: `memw(Rbase=##address)=R0.new`.
pub(super) fn publish_new_state(base:Gpr,producer:Gpr,address:u32)->Result<[u32;2],String>{
    if base.number()>7||producer.number()!=0||address%4!=0{
        return Err("V66 coupled state write requires compact base, R0 producer and aligned address".into());
    }
    let hi=((address>>20)<<16)|((address>>6)&0x3fff)|0x4000;
    let consumer=0xaba0_d280|(u32::from(base.number())<<16)|(address&0x3f);
    Ok([hi,consumer])
}
pub(super) fn cache_clean(addr:Gpr)->u32{0xa000_c000|(u32::from(addr.number())<<16)}

pub fn build_worker_command()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,H::AssignSmallImmediate{dst:r(2)?,value:2}.encode()?,P::Final);
    put(&mut out,H::InsertBitField{dst:r(1)?,src:r(2)?,width:8,offset:16}.encode()?,P::Final);
    for bit in [30,29] {
        put(&mut out,H::SetBit{dst:r(1)?,src:r(1)?,position:bit}.encode()?,P::Final);
    }
    put(&mut out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:8}.encode()?,P::ContinuationA);
    put(&mut out,relay_word(r(23)?,0,r(0)?,0,r(1)?)?,P::ContinuationB);
    put(&mut out,direct_call(0x96c0_1e40,0x96c3_2464)?,P::Final);
    let val=cdsp_boot::encode_extended_assignment(r(0)?,32,false);
    put(&mut out,val[0],P::ContinuationA);
    put(&mut out,val[1],P::ContinuationA);
    let state=publish_new_state(r(1)?,r(0)?,0xfe00_0044)?;
    put(&mut out,state[0],P::ContinuationA);
    put(&mut out,state[1],P::Final);
    put(&mut out,cache_clean(r(1)?),P::Final);
    for (here,target) in [
        (0x96c0_1e58,0x96c0_2184),
        (0x96c0_1e5c,0x96c0_2268),
        (0x96c0_1e60,0x96c0_29b0),
    ] {
        put(&mut out,direct_call(here,target)?,P::Final);
    }
    for (here,target) in [
        (0x96c0_1e64,0x96c0_1318),
        (0x96c0_1e6c,0x96c0_c004),
    ] {
        let call=cdsp_early_controllers::encode_extended_call(here,target)?;
        put(&mut out,call[0],P::ContinuationA);
        put(&mut out,call[1],P::Final);
    }
    if out.len()!=FUNCTION_BYTES {return Err(format!("V66 state writer emitted {} rather than {} bytes",out.len(),FUNCTION_BYTES));}
    Ok(out)
}
pub fn verify_reference(image:&[u8])->Result<(),String>{
    cdsp_early_worker_decision::verify_reference(image)?;
    if cdsp_early_worker_decision::REGION_END!=FILE_OFFSET{return Err("CDSP command not adjacent to decision".into());}
    let ours=build_worker_command()?;
    let ref_bytes=image.get(FILE_OFFSET..REGION_END).ok_or("short CDSP file")?;
    if ours!=ref_bytes{return Err("CDSP controller command differs from original reference".into());}
    Ok(())
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn checked_packed_relay_and_status(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(build_worker_command().unwrap().len(),76);
        assert_eq!(relay_word(r(23),0,r(0),0,r(1)).unwrap(),0x80f1_0001);
        assert_eq!(relay_word(r(22),4,r(2),4,r(2)).unwrap(),0x81e2_0122);
        assert!(relay_word(r(23),8,r(0),0,r(1)).is_err());
        assert!(relay_word(r(23),0,r(8),0,r(1)).is_err());
        assert_eq!(publish_new_state(r(1),r(0),0xfe00_0044).unwrap(),[0x0fe0_4001,0xaba1_d284]);
        assert!(publish_new_state(r(1),r(2),0xfe00_0044).is_err());
        assert_eq!(direct_call(0x96c0_1e40,0x96c3_2464).unwrap(),0x5a06_c312);
    }
}
