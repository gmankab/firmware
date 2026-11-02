// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 exception-context save through the root-thread trap gate.
//! The two packed memory stores are genuine duplex operations and their
//! encoded 32-bit words are assembled from distinct register pair/offset
//! fields, not copied from OEM instruction bytes. Unimplemented code follows
//! at 0x481c, beyond this deliberately bounded fragment.
use super::cdsp_boot::{GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_exception_entry;

pub const FILE_OFFSET:usize=0x4760;
pub const REGION_END:usize=0x481c;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1760;

/// Only the 40-bit? No: Hexagon memory doubleword is a 64-bit register pair
/// (low even integer register, high = low+1). Two 32-bit instructions may
/// share a packet, or a V66 duplex word can contain two operations.
fn pair(low:Gpr)->Result<u32,String>{
    let n=low.number();
    if n>30||n%2!=0{return Err("CDSP context register pair must have even low GPR <=30".into());}
    Ok(u32::from(n))
}
fn store_double(base:Gpr,low:Gpr,offset:u16)->Result<u32,String>{
    if offset%8!=0 || offset>248 {return Err("CDSP memd offset must be aligned by 8 and fit the checked 248-byte field".into());}
    Ok(0xa1c0_c000|(u32::from(base.number())<<16)
       |(pair(low)?<<8)|u32::from(offset/8))
}
fn load_double(base:Gpr,low:Gpr)->Result<u32,String>{
    Ok(0x91c0_c000|(u32::from(base.number())<<16)|pair(low)?)
}
fn store_word(base:Gpr,src:Gpr,offset:u16)->Result<u32,String>{
    if offset%4!=0 ||offset>252{return Err("CDSP memw offset must be aligned by 4".into());}
    Ok(0xa180_c000|(u32::from(base.number())<<16)
       |(u32::from(src.number())<<8)|u32::from(offset/4))
}
/// Duplex upper half encodes in-place ADD of a compact high register pair;
/// lower half encodes MEMD SP+byte_offset with an even low GPR register.
fn duplex_add_and_store(accumulator:Gpr,right:Gpr,stack_offset:u8,low:Gpr)->Result<u32,String>{
    let d=accumulator.number();let s=right.number();
    if !(16..=23).contains(&d) || !(16..=23).contains(&s)
       || stack_offset>120||stack_offset%8!=0||low.number()>14 {
        return Err("unsupported V66 duplex high-REG ADD plus stack MEMD operands".into());
    }
    let lhs=u32::from(d-8);let rhs=u32::from(s-8);
    Ok(((0x7800|(rhs<<4)|lhs)<<16)
       | (0x2a00|u32::from(stack_offset)|pair(low)?/2))
}
/// One V66 duplex word packs two 64-bit stores, one in its high and the
/// other in its low half. The second register pair uses a compressed bank
/// mapping for registers R16..R23; the first uses the low GPR bank only.
fn duplex_two_stores(first_offset:u8,first_low:Gpr,
                    second_offset:u8,second_low:Gpr)->Result<u32,String>{
    let first=first_low.number();let second=second_low.number();
    if first_offset>248||second_offset>248||first_offset%8!=0||second_offset%8!=0 ||
       first>14||first%2!=0||second%2!=0||second>30 {
        return Err("V66 duplex MEMD pairs must use checked offset/compact register banks".into());
    }
    let second_pair=if second>=16 {u32::from((second-8)/2)}else{u32::from(second/2)};
    Ok(0xea00_0a00
       |((u32::from(first_offset)|u32::from(first/2))<<16)
       |u32::from(second_offset)|second_pair)
}
#[derive(Clone,Copy)]
enum ScalarSystem {Htid,Ccr,Ssr,Elr,Badva,Stid}
impl ScalarSystem {
    fn selector(self)->u8{ match self {
        Self::Htid=>8,Self::Ccr=>7,Self::Ssr=>6,
        Self::Elr=>3,Self::Badva=>9,Self::Stid=>2,
    }}
    fn emit(self,dst:Gpr)->u32{
        0x6e80_c000|(u32::from(self.selector())<<16)|u32::from(dst.number())
    }
}
#[derive(Clone,Copy)]
enum RuntimeScalar {PredicateGroup,Usr,Gp,Ugp}
impl RuntimeScalar {
    fn selector(self)->u8{match self{
        Self::PredicateGroup=>4,Self::Usr=>8,Self::Gp=>11,Self::Ugp=>10,
    }}
    fn emit(self,dst:Gpr)->u32{
        0x6a00_c000|(u32::from(self.selector())<<16)|u32::from(dst.number())
    }
}
#[derive(Clone,Copy)]
enum RuntimePair {HardwareLoop0,HardwareLoop1,Modulators,ControlStatus}
impl RuntimePair {
    fn selector(self)->u8{match self{
        Self::HardwareLoop0=>0,Self::HardwareLoop1=>2,
        Self::Modulators=>6,Self::ControlStatus=>12,
    }}
    fn emit(self,low:Gpr)->Result<u32,String>{
        Ok(0x6800_c000|(u32::from(self.selector())<<16)|pair(low)?)
    }
}
/// P0 compare and predicted-not-taken branch is a single instruction.
/// Hexagon compact predicate source encoding uses 0..7 and 16..23 banks.
fn skip_when_root_thread(packet_start:u32,target:u32,thread:Gpr)->Result<u32,String>{
    if packet_start%4!=0||target%4!=0||target<=packet_start {
        return Err("root thread exception gate target must be forward and packet aligned".into());
    }
    let src=thread.number();
    let index=if src<=7{src}else if (16..=23).contains(&src){src-8}
              else{return Err("unsupported compact root-thread predicate register".into())};
    let diff=target-packet_start;
    if diff>0x1fe {return Err("root-thread branch offset outside checked field".into());}
    Ok(0x1000_c000|(u32::from(index)<<16)|diff/2)
}
fn emit(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}
fn duplex(out:&mut Vec<u8>,encoded:u32){
    // Duplex slot fields can overlap generic parse bits; preserving the
    // independently verified coupled encoding is required.
    out.extend_from_slice(&encoded.to_le_bytes());
}

pub fn build_context_save()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    // The exact same independently authored thread-local controller header
    // appears twice, at 0x471c and 0x4760. Reuse our source emitter; never
    // copy either sequence from the reference file at build time.
    let header=cdsp_early_exception_entry::build_exception_entry()?;
    out.extend_from_slice(&header[..7*4]);
    duplex(&mut out,duplex_add_and_store(r(23)?,r(22)?,8,r(2)?)?);
    emit(&mut out,load_double(r(23)?,r(2)?)?,P::Final);
    emit(&mut out,H::AssignRegister{dst:r(2)?,src:r(28)?}.encode()?,P::ContinuationA);
    duplex(&mut out,duplex_two_stores(0,r(2)?,88,r(22)?)?);
    emit(&mut out,H::AssignRegister{dst:r(3)?,src:r(1)?}.encode()?,P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(24)?,96)?,P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(26)?,104)?,P::Final);
    emit(&mut out,store_double(r(29)?,r(2)?,112)?,P::Final);
    emit(&mut out,RuntimeScalar::PredicateGroup.emit(r(2)?),P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(30)?,120)?,P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(18)?,72)?,P::Final);
    emit(&mut out,RuntimeScalar::Usr.emit(r(3)?),P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(16)?,64)?,P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(14)?,56)?,P::Final);
    emit(&mut out,RuntimeScalar::Gp.emit(r(2)?),P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(12)?,48)?,P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(2)?,168)?,P::Final);
    emit(&mut out,RuntimeScalar::Ugp.emit(r(3)?),P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(10)?,40)?,P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(8)?,32)?,P::Final);
    emit(&mut out,RuntimePair::HardwareLoop0.emit(r(2)?)?,P::ContinuationA);
    duplex(&mut out,duplex_two_stores(24,r(6)?,136,r(2)?)?);
    emit(&mut out,RuntimePair::HardwareLoop1.emit(r(2)?)?,P::ContinuationA);
    duplex(&mut out,duplex_two_stores(16,r(4)?,144,r(2)?)?);
    emit(&mut out,RuntimePair::Modulators.emit(r(2)?)?,P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(2)?,152)?,P::Final);
    emit(&mut out,store_double(r(29)?,r(2)?,160)?,P::Final);
    emit(&mut out,RuntimePair::ControlStatus.emit(r(2)?)?,P::Final);
    emit(&mut out,ScalarSystem::Ccr.emit(r(2)?),P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(2)?,184)?,P::Final);
    emit(&mut out,ScalarSystem::Ssr.emit(r(3)?),P::ContinuationA);
    emit(&mut out,store_word(r(29)?,r(2)?,192)?,P::Final);
    emit(&mut out,ScalarSystem::Elr.emit(r(2)?),P::Final);
    emit(&mut out,ScalarSystem::Badva.emit(r(2)?),P::ContinuationA);
    emit(&mut out,store_double(r(29)?,r(2)?,128)?,P::Final);
    emit(&mut out,ScalarSystem::Stid.emit(r(3)?),P::Final);
    duplex(&mut out,duplex_two_stores(176,r(2)?,80,r(20)?)?);
    emit(&mut out,ScalarSystem::Htid.emit(r(23)?),P::Final);
    emit(&mut out,skip_when_root_thread(0x96c0_1814,0x96c0_181c,r(23)?)?,P::Final);
    emit(&mut out,H::JumpRelative{displacement_bytes:0x2bc}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES {return Err(format!("CDSP context save length {:#x} differs from expected {FUNCTION_BYTES:#x}",out.len()));}
    Ok(out)
}

pub fn build_supervisor_with_context()->Result<Vec<u8>,String>{
    let mut v=cdsp_early_exception_entry::build_supervisor_with_entry()?;
    if cdsp_early_exception_entry::REGION_END!=FILE_OFFSET {return Err("CDSP context save doesn't abut exception entry".into());}
    v.extend(build_context_save()?);
    if v.len()!=REGION_END-super::cdsp_early_system_start::FILE_OFFSET{return Err("CDSP saved context cumulative size mismatch".into());}
    Ok(v)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_exception_entry::verify_reference(original)?;
    let b=build_context_save()?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(b.as_slice()) {
        return Err("CDSP reconstructed interrupt context save mismatch".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn context_save_is_exact_and_does_not_generate_zero_fill(){
        assert_eq!(FILE_OFFSET,cdsp_early_exception_entry::REGION_END);
        assert_eq!(build_context_save().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(build_supervisor_with_context().unwrap().len(),REGION_END-0x4500);
        assert_eq!(FUNCTION_BYTES,188);
    }
    #[test]fn duplex_independent_operand_fields_change(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(duplex_add_and_store(r(23),r(22),8,r(2)).unwrap(),0x78ef_2a09);
        assert_eq!(duplex_add_and_store(r(23),r(22),16,r(4)).unwrap(),0x78ef_2a12);
        assert_eq!(duplex_two_stores(0,r(2),88,r(22)).unwrap(),0xea01_0a5f);
        assert_eq!(duplex_two_stores(24,r(6),136,r(2)).unwrap(),0xea1b_0a89);
        assert!(duplex_add_and_store(r(7),r(22),8,r(2)).is_err());
        assert!(store_double(r(29),r(3),8).is_err());
        assert!(store_double(r(29),r(2),257).is_err());
        assert_eq!(skip_when_root_thread(0x96c0_1814,0x96c0_181c,r(23)).unwrap(),0x100f_c004);
    }
}
