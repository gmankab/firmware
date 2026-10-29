// SPDX-License-Identifier: GPL-3.0-only
//! Hexagon V66 D-cache tag inspection and physical-word snapshot. This is
//! source-authored operand encoding of 45 instruction words, with two nested
//! hardware loops and an original loop-end marker. The privileged DCTAGR /
//! memw_phys selectors require independent Rizin decoding; LLVM6 validates
//! ordinary ALU, indexed-memory, loop and ABI instruction families.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::{cdsp_early_revision_guard as previous, cdsp_early_worker_command as op};

pub const FILE_OFFSET:usize=0x51b4;
pub const REGION_END:usize=0x5268;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_21b4;
fn put(out:&mut Vec<u8>,word:u32,marker:P){out.extend_from_slice(&marker.apply(word).to_le_bytes());}

fn register_multiply(dst:Gpr,left:Gpr,right:Gpr)->u32 {
    0xed00_c000 | (u32::from(left.number())<<16)
        | (u32::from(right.number())<<8) | u32::from(dst.number())
}
fn read_loop_count(dst:Gpr,loop_index:u8)->Result<u32,String>{
    if loop_index>1 {return Err("Hexagon LC selector must be LC0/LC1".into());}
    let selector=if loop_index==0 {1} else {3};
    Ok(0x6a00_c000 | (selector<<16) | u32::from(dst.number()))
}
fn read_tag(dst:Gpr,index:Gpr)->Result<u32,String>{
    if dst.number()>7 {return Err("DCTAGR verified only for low GPR destinations".into());}
    Ok(0xa420_c000|(u32::from(index.number())<<16)|u32::from(dst.number()))
}
fn indexed_store(src:Gpr,base:Gpr,index:Gpr,shift:u8)->Result<u32,String>{
    if shift!=0 {return Err("CDSP tag indexed store only verified with zero shift".into());}
    Ok(0x3b80_c000 | (u32::from(base.number())<<16)
       |(u32::from(index.number())<<8)|u32::from(src.number()))
}
fn and_registers(dst:Gpr,left:Gpr,right:Gpr)->u32 {
    0xf100_c000 | (u32::from(left.number())<<16)
       | (u32::from(right.number())<<8)|u32::from(dst.number())
}
fn physical_word_read(dst:Gpr,offset:Gpr,base:Gpr)->Result<u32,String>{
    if dst.number()>7||offset.number()>7||base.number()>7 {
        return Err("physical V66 word read restricts tested operand bank".into());
    }
    Ok(0x9200_e000|(u32::from(offset.number())<<16)
       |(u32::from(base.number())<<8)|u32::from(dst.number()))
}
fn duplex_increment_pair(first:Gpr,first_add:u8,second:Gpr,second_add:u8)->Result<u32,String>{
    // Independently LLVM-verified V66 compact duplex layout for this exact
    // pair. Do not extrapolate to other operand groups without ISA evidence.
    if first.number()!=5 || first_add!=4 || second.number()!=4 || second_add!=1 {
        return Err("only independently assembled R5+=#4; R4+=#1 duplex supported".into());
    }
    Ok(0x2000_3000|(u32::from(first_add)<<20)
       |(u32::from(first.number())<<16)
       |(u32::from(second_add)<<8)
       |(u32::from(second.number())<<4)|u32::from(second.number()))
}

fn branch_if_not_new_eq8(src:Gpr,here:u32,target:u32)->Result<u32,String>{
    if src.number()>7||here%4!=0||target%4!=0||target>=here || here-target>0x1fe {
        return Err("short backward V66 new-equality branch outside verified constraints".into());
    }
    let half=((i64::from(target)-i64::from(here))/2) as i32 as u32;
    Ok(0x1070_e800 | (u32::from(src.number())<<16) | (half&0xff))
}
fn absolute_word_store(src:Gpr,base:Gpr,address:u32)->Result<[u32;2],String>{
    if base.number()>7||src.number()>7||address%4!=0 {
        return Err("CDSP absolute cache state store requires compact registers and alignment".into());
    }
    let high=((address>>20)<<16)|((address>>6)&0x3fff)|0x4000;
    let body=0xab80_c080 | (u32::from(base.number())<<16)
       |(u32::from(src.number())<<8)|(address&0x3f);
    Ok([high,body])
}

pub fn build_dcache_tag_recovery()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    let count=cdsp_boot::encode_extended_gp_word_load(r(10)?,0xfe03_9204);
    put(&mut out,count[0],P::ContinuationA);
    put(&mut out,count[1],P::Final);
    put(&mut out,register_multiply(r(8)?,r(10)?,r(12)?),P::ContinuationA);
    let table=cdsp_boot::encode_extended_gp_word_load(r(14)?,0xfe02_6d38);
    put(&mut out,table[0],P::ContinuationA);
    put(&mut out,table[1],P::Final);
    put(&mut out,H::ArithmeticShiftLeft{dst:r(8)?,src:r(8)?,amount:5}.encode()?,P::ContinuationA);
    put(&mut out,H::PredicateP0EqualsZero{src:r(14)?}.encode()?,P::ContinuationA);
    put(&mut out,H::JumpIfNewP0NotTaken{displacement_bytes:0x98}.encode()?,P::Final);
    put(&mut out,H::HardwareLoop{loop_index:1,counter:r(10)?,displacement_bytes:4}.encode()?,P::Final);
    put(&mut out,read_loop_count(r(10)?,1)?,P::Final);
    put(&mut out,H::HardwareLoopImmediate{loop_index:0,iterations:8,displacement_bytes:8}.encode()?,P::ContinuationA);
    put(&mut out,H::AddSignedSmallImmediate{dst:r(12)?,src:r(10)?,value:-1}.encode()?,P::Final);
    put(&mut out,read_loop_count(r(10)?,0)?,P::ContinuationA);
    put(&mut out,H::AssignSmallImmediate{dst:r(7)?,value:40}.encode()?,P::Final);
    put(&mut out,H::AddSignedSmallImmediate{dst:r(10)?,src:r(10)?,value:-1}.encode()?,P::ContinuationA);
    put(&mut out,H::SubtractRegisters{dst:r(8)?,left:r(8)?,right:r(7)?}.encode()?,P::Final);
    put(&mut out,H::InsertBitField{dst:r(10)?,src:r(12)?,width:7,offset:5}.encode()?,P::ContinuationA);
    put(&mut out,H::AssignRegister{dst:r(5)?,src:r(8)?}.encode()?,P::Final);
    let mask=cdsp_boot::encode_extended_assignment(r(4)?,0x01ff_ffff,false);
    put(&mut out,mask[0],P::ContinuationA);
    put(&mut out,mask[1],P::ContinuationA);
    put(&mut out,read_tag(r(1)?,r(10)?)?,P::Final);
    put(&mut out,H::TestBitPredicate{predicate:0,src:r(1)?,bit:29}.encode()?,P::ContinuationA);
    put(&mut out,H::JumpIfNotNewP0NotTaken{displacement_bytes:0x3c}.encode()?,P::ContinuationA);
    put(&mut out,indexed_store(r(1)?,r(14)?,r(5)?,0)?,P::Final);
    put(&mut out,H::ArithmeticShiftLeft{dst:r(2)?,src:r(12)?,amount:5}.encode()?,P::ContinuationA);
    put(&mut out,and_registers(r(3)?,r(1)?,r(4)?),P::ContinuationA);
    put(&mut out,H::AssignSmallImmediate{dst:r(4)?,value:0}.encode()?,P::Final);
    put(&mut out,H::AssignRegister{dst:r(7)?,src:r(2)?}.encode()?,P::Final);
    put(&mut out,H::InsertBitField{dst:r(7)?,src:r(3)?,width:21,offset:11}.encode()?,P::ContinuationA);
    put(&mut out,H::AddSmallImmediate{dst:r(5)?,src:r(5)?,value:4}.encode()?,P::Final);
    put(&mut out,indexed_store(r(7)?,r(14)?,r(5)?,0)?,P::Final);
    put(&mut out,physical_word_read(r(6)?,r(2)?,r(3)?)?,P::Final);
    out.extend_from_slice(&duplex_increment_pair(r(5)?,4,r(4)?,1)?.to_le_bytes());
    put(&mut out,H::AddSmallImmediate{dst:r(2)?,src:r(2)?,value:4}.encode()?,P::ContinuationA);
    put(&mut out,indexed_store(r(6)?,r(14)?,r(5)?,0)?,P::Final);
    put(&mut out,branch_if_not_new_eq8(r(4)?,0x96c0_2240,0x96c0_2230)?,P::Final);
    // These architecturally defined no-op words *carry* the two hardware
    // loop-end parse markers and are executable, not file alignment padding.
    put(&mut out,H::Nop.encode()?,P::LoopEnd);
    put(&mut out,H::Nop.encode()?,P::LoopEnd);
    put(&mut out,H::Nop.encode()?,P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:2}.encode()?,P::Final);
    let done=absolute_word_store(r(1)?,r(0)?,0xfe01_6e24)?;
    put(&mut out,done[0],P::ContinuationA);
    put(&mut out,done[1],P::Final);
    put(&mut out,op::cache_clean(r(0)?),P::Final);
    // `deallocframe(FP)` is the unparameterized Hexagon V66 frame removal
    // verified with LLVM6, followed by `jumpr LR`.
    put(&mut out,0x901e_c01e,P::Final);
    put(&mut out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES {return Err(format!("CDSP D-cache recovery assembled {} instead of {} bytes",out.len(),FUNCTION_BYTES));}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    previous::verify_reference(original)?;
    if previous::REGION_END!=FILE_OFFSET {return Err("D-cache recovery must follow revision guard".into());}
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_dcache_tag_recovery()?.as_slice()) {
        return Err("source-built D-cache tag inspection differs from original CDSP".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn nested_loops_and_tag_capture_are_bounded(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,180);
        assert_eq!(build_dcache_tag_recovery().unwrap().len(),180);
        assert_eq!(read_loop_count(r(10),1).unwrap(),0x6a03_c00a);
        assert_eq!(read_loop_count(r(10),0).unwrap(),0x6a01_c00a);
        assert!(read_loop_count(r(10),2).is_err());
        assert_eq!(register_multiply(r(8),r(10),r(12)),0xed0a_cc08);
        assert_eq!(read_tag(r(1),r(10)).unwrap(),0xa42a_c001);
        assert_eq!(indexed_store(r(1),r(14),r(5),0).unwrap(),0x3b8e_c501);
        assert!(indexed_store(r(1),r(14),r(5),1).is_err());
        assert_eq!(physical_word_read(r(6),r(2),r(3)).unwrap(),0x9202_e306);
        assert!(physical_word_read(r(10),r(2),r(3)).is_err());
        assert_eq!(duplex_increment_pair(r(5),4,r(4),1).unwrap(),0x2045_3144);
        assert!(duplex_increment_pair(r(9),4,r(4),1).is_err());
        assert!(duplex_increment_pair(r(3),2,r(6),3).is_err());
        assert_eq!(branch_if_not_new_eq8(r(4),0x96c0_2240,0x96c0_2230).unwrap(),0x1074_e8f8);
        assert!(branch_if_not_new_eq8(r(4),0x96c0_2240,0x96c0_2244).is_err());
        assert_eq!(absolute_word_store(r(1),r(0),0xfe01_6e24).unwrap(),[0x0fe0_45b8,0xab80_c1a4]);
        assert!(absolute_word_store(r(1),r(0),0xfe01_6e25).is_err());
    }
}
