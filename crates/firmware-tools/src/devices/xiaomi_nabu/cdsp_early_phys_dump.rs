// SPDX-License-Identifier: GPL-3.0-only
//! CDSP root physical-state snapshot and flush of the controller's L1 data
//! cache state. The first helper copies privileged physical words through a
//! bounded hardware loop; the second publishes a controller flag and cleans
//! its data cache line. Hexagon packet markers and GPR addressing are typed.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P, ScalarPacket as H};
use super::cdsp_early_l2_sync;

pub const FILE_OFFSET:usize=0x4a38;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1a38;
pub const COPY_END:usize=0x4a84;
pub const REGION_END:usize=0x4ab0;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;

#[derive(Copy,Clone,Debug)]
enum PhysicalCopyOp {
    ShiftRightRegister { dst:Gpr, value:Gpr, shift:Gpr },
    LoadWord { dst:Gpr, physical_address:Gpr, selector:Gpr },
    StoreWordPostIncrement { src:Gpr, destination:Gpr, step:u8 },
}
impl PhysicalCopyOp {
    fn encode(self)->Result<u32,String>{
        match self {
            Self::ShiftRightRegister{dst,value,shift}=>
                Ok(0xc640_c040|(u32::from(value.number())<<16)
                    |(u32::from(shift.number())<<8)|u32::from(dst.number())),
            Self::LoadWord{dst,physical_address,selector}=>
                Ok(0x9200_e000|(u32::from(physical_address.number())<<16)
                    |(u32::from(selector.number())<<8)|u32::from(dst.number())),
            Self::StoreWordPostIncrement{src,destination,step}=>{
                if ![4,8,12,16,20,24,28].contains(&step){
                    return Err("CDSP physical snapshot requires 4-byte aligned valid postincrement".into());
                }
                Ok(0xab80_c000|(u32::from(destination.number())<<16)
                   |(u32::from(src.number())<<8)|u32::from(step)*2)
            }
        }
    }
}
fn special_s54(dst:Gpr)->u32{0x6e80_c000|(54<<16)|u32::from(dst.number())}
fn memory_word(dst:Gpr,base:Gpr)->u32{0x9180_c000|(u32::from(base.number())<<16)|u32::from(dst.number())}
fn new_word_store(base:Gpr)->u32{0xa1a0_d200|(u32::from(base.number())<<16)}
#[derive(Copy,Clone,Debug)] enum CacheLineOp { Clean, Invalidate }
fn cache_line_op(op:CacheLineOp,address:Gpr)->u32{
    let opcode=match op {CacheLineOp::Clean=>0xa000_c000,CacheLineOp::Invalidate=>0xa020_c000};
    opcode|(u32::from(address.number())<<16)
}
fn put(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}

pub fn build_physical_dump()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(COPY_END-FILE_OFFSET);
    put(&mut out,special_s54(r(23)?),P::Final);
    let relocated=cdsp_boot::encode_extended_word_load(r(1)?,r(23)?,-0x3f98cda0)?;
    put(&mut out,relocated[0],P::ContinuationA);
    put(&mut out,relocated[1],P::ContinuationA);
    let size=cdsp_boot::encode_extended_word_load(r(3)?,r(23)?,-0x1f39980)?;
    put(&mut out,size[0],P::ContinuationA);
    put(&mut out,size[1],P::Final);
    put(&mut out,H::PredicateP0GreaterThan32{src:r(3)?}.encode()?,P::ContinuationA);
    put(&mut out,H::JumpIfNotNewP0NotTaken{displacement_bytes:0x34}.encode()?,P::Final);
    put(&mut out,H::AddRegisters{dst:r(1)?,left:r(1)?,right:r(23)?}.encode()?,P::ContinuationA);
    let address=cdsp_boot::encode_extended_word_load(r(0)?,r(23)?,-0x1f398cc)?;
    put(&mut out,address[0],P::ContinuationA);
    put(&mut out,address[1],P::Final);
    put(&mut out,H::LogicalShiftRight{dst:r(2)?,src:r(3)?,amount:2}.encode()?,P::ContinuationA);
    put(&mut out,H::SkipIfSourceZeroNotTaken{src:r(0)?,displacement_bytes:0x20}.encode()?,P::Final);
    put(&mut out,H::HardwareLoop{loop_index:0,counter:r(2)?,displacement_bytes:8}.encode()?,P::ContinuationA);
    put(&mut out,H::AssignSmallImmediate{dst:r(2)?,value:11}.encode()?,P::Final);
    put(&mut out,PhysicalCopyOp::ShiftRightRegister{dst:r(4)?,value:r(0)?,shift:r(2)?}.encode()?,P::Final);
    put(&mut out,PhysicalCopyOp::LoadWord{dst:r(5)?,physical_address:r(0)?,selector:r(4)?}.encode()?,P::Final);
    put(&mut out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:4}.encode()?,P::LoopEnd);
    put(&mut out,PhysicalCopyOp::StoreWordPostIncrement{src:r(5)?,destination:r(1)?,step:4}.encode()?,P::Final);
    put(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=COPY_END-FILE_OFFSET{return Err("Physical snapshot instruction count incorrect".into());}
    Ok(out)
}

pub fn build_cache_publish()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(REGION_END-COPY_END);
    put(&mut out,special_s54(r(23)?),P::Final);
    let pointer=cdsp_boot::encode_extended_add(r(22)?,r(23)?,-0x1fc6e38,true);
    put(&mut out,pointer[0],P::ContinuationA);
    put(&mut out,pointer[1],P::Final);
    put(&mut out,memory_word(r(22)?,r(22)?),P::Final);
    let status=cdsp_boot::encode_extended_add(r(23)?,r(22)?,-0x1fdb300,true);
    put(&mut out,status[0],P::ContinuationA);
    put(&mut out,status[1],P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(22)?,value:4}.encode()?,P::ContinuationA);
    put(&mut out,new_word_store(r(23)?),P::Final);
    put(&mut out,cache_line_op(CacheLineOp::Clean,r(23)?),P::Final);
    put(&mut out,cache_line_op(CacheLineOp::Invalidate,r(23)?),P::Final);
    put(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=REGION_END-COPY_END{return Err("CDSP cache-line publish instruction count incorrect".into());}
    Ok(out)
}

pub fn build_physical_dump_and_cache_publish()->Result<Vec<u8>,String>{
    let mut b=build_physical_dump()?;b.extend(build_cache_publish()?);
    if b.len()!=FUNCTION_BYTES{return Err("CDSP physical dump/caching has a gap".into());}
    Ok(b)
}

pub fn build_supervisor_with_physical_dump()->Result<Vec<u8>,String>{
    if cdsp_early_l2_sync::REGION_END!=FILE_OFFSET {
        return Err("CDSP physical dump does not abut L2 sync".into());
    }
    let mut b=cdsp_early_l2_sync::build_supervisor_with_l2_sync()?;
    b.extend(build_physical_dump_and_cache_publish()?);
    Ok(b)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_l2_sync::verify_reference(original)?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_physical_dump_and_cache_publish()?.as_slice()){
        return Err("CDSP physical dump/cache publish differs from original pinned firmware".into());
    }
    Ok(())
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn physdump_and_publish_are_adjacent(){
        assert_eq!(FUNCTION_BYTES,120);
        assert_eq!(build_physical_dump().unwrap().len(),76);
        assert_eq!(build_cache_publish().unwrap().len(),44);
        assert_eq!(build_physical_dump_and_cache_publish().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+(FILE_OFFSET-0x3000) as u32);
    }
    #[test]fn memory_and_cache_encodings_check_operands(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(PhysicalCopyOp::ShiftRightRegister{dst:r(4),value:r(0),shift:r(2)}.encode().unwrap(),0xc640_c244);
        assert_eq!(PhysicalCopyOp::LoadWord{dst:r(5),physical_address:r(0),selector:r(4)}.encode().unwrap(),0x9200_e405);
        assert_eq!(PhysicalCopyOp::StoreWordPostIncrement{src:r(5),destination:r(1),step:4}.encode().unwrap(),0xab81_c508);
        assert!(PhysicalCopyOp::StoreWordPostIncrement{src:r(5),destination:r(1),step:7}.encode().is_err());
        assert_eq!(new_word_store(r(23)),0xa1b7_d200);
        assert_eq!(cache_line_op(CacheLineOp::Clean,r(23)),0xa017_c000);
        assert_eq!(cache_line_op(CacheLineOp::Invalidate,r(23)),0xa037_c000);
    }
}
