// SPDX-License-Identifier: GPL-3.0-only
//! Guard the first supervisor cache-tag recovery routine by CDSP revision.
//! This 48-byte V66 prefix contains a fast return, command flag publication,
//! a coupled absolute memory read and controller entry dispatch. The rest of
//! the cache-tag recovery function is reconstructed in following fragments.
use super::cdsp_boot::{GeneralRegister as Gpr, PacketMarker as P, ScalarPacket as H};
use super::{cdsp_early_worker_recovery as previous, cdsp_early_worker_command as op};

pub const FILE_OFFSET:usize=0x5184;
pub const REGION_END:usize=0x51b4;
pub const PHYSICAL_ADDRESS:u32=0x96c0_2184;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
fn emit(out:&mut Vec<u8>,word:u32,p:P){out.extend_from_slice(&p.apply(word).to_le_bytes());}

/// Only known V66 special register REV is supported by this encoding.
fn read_revision(dst:Gpr)->u32{0x6e80_c000|(29<<16)|u32::from(dst.number())}
fn and_eight_bits(dst:Gpr,src:Gpr,immediate:u8)->u32{
    0x7600_c000|(u32::from(src.number())<<16)|(u32::from(immediate)<<5)|u32::from(dst.number())
}
fn compare_gt_unsigned_p0(src:Gpr,immediate:u8)->u32{
    0x7540_c000|(u32::from(src.number())<<16)|(u32::from(immediate)<<5)
}
fn return_if_not_new_predicate(source:Gpr)->Result<u32,String>{
    if source.number()!=31 {return Err("CDSP conditional revision return must use LR".into());}
    Ok(0x5360_c800|(u32::from(source.number())<<16))
}
/// Scalar packet `Rdst = memw(Rassign=##addr)`: Hexagon mem address
/// stores bits [5:2] at body [11:8] and bits [31:6] in the extender.
fn absolute_memory_word_read(dst:Gpr,assign:Gpr,address:u32)->Result<[u32;2],String>{
    if dst.number()>7||assign.number()>7||address%4!=0 {
        return Err("absolute controller read needs compact registers and word alignment".into());
    }
    let ext=((address>>20)<<16)|((address>>6)&0x3fff)|0x4000;
    let body=0x9b80_d000|(u32::from(assign.number())<<16)
        |(((address&0x3f)>>2)<<8)|u32::from(dst.number());
    Ok([ext,body])
}
fn return_if_revision_initialized(src:Gpr)->Result<u32,String>{
    if src.number()>7 {return Err("revision guard predicate source must use compact GPR".into());}
    Ok(0x5900_3fc7|(u32::from(src.number())<<20))
}
fn store_and_create_context(src:Gpr,base:Gpr,frame:u16)->Result<u32,String>{
    if src.number()!=0||base.number()!=1||frame!=0 {
        return Err("coupled controller state store/allocframe only confirmed for r0,r1,zero frame".into());
    }
    Ok(0xa000_3c00|(u32::from(base.number())<<20))
}

pub fn build_revision_guard()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    emit(&mut out,read_revision(r(0)?),P::Final);
    emit(&mut out,and_eight_bits(r(0)?,r(0)?,255),P::Final);
    emit(&mut out,compare_gt_unsigned_p0(r(0)?,96),P::ContinuationA);
    emit(&mut out,return_if_not_new_predicate(r(31)?)?,P::Final);
    emit(&mut out,H::AssignSmallImmediate{dst:r(0)?,value:1}.encode()?,P::ContinuationA);
    emit(&mut out,H::AssignSmallImmediate{dst:r(12)?,value:10}.encode()?,P::ContinuationA);
    let memory=absolute_memory_word_read(r(2)?,r(1)?,0xfe01_6e24)?;
    emit(&mut out,memory[0],P::ContinuationA);
    emit(&mut out,memory[1],P::Final);
    // Compact compound predicate/return uses its own 0x0000 packet field.
    out.extend_from_slice(&return_if_revision_initialized(r(2)?)?.to_le_bytes());
    out.extend_from_slice(&store_and_create_context(r(0)?,r(1)?,0)?.to_le_bytes());
    emit(&mut out,op::direct_call(0x96c0_21ac,0x96c0_34e8)?,P::Final);
    emit(&mut out,H::JumpRelative{displacement_bytes:0xb0}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES {return Err("CDSP revision guard length mismatch".into());}
    Ok(out)
}

pub fn verify_reference(original:&[u8])->Result<(),String>{
    previous::verify_reference(original)?;
    if previous::REGION_END!=FILE_OFFSET{return Err("CDSP revision guard does not abut worker recovery".into());}
    let actual=original.get(FILE_OFFSET..REGION_END).ok_or("CDSP reference too short")?;
    if build_revision_guard()?.as_slice()!=actual{return Err("CDSP revision guard differs from pinned reference".into());}
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test] fn revision_guard_absolute_load_and_returns_are_bounded(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,48);
        assert_eq!(build_revision_guard().unwrap().len(),48);
        assert_eq!(read_revision(r(0)),0x6e9d_c000);
        assert_eq!(and_eight_bits(r(0),r(0),255),0x7600_dfe0);
        assert_eq!(compare_gt_unsigned_p0(r(0),96),0x7540_cc00);
        assert_eq!(absolute_memory_word_read(r(2),r(1),0xfe01_6e24).unwrap(),[0x0fe0_45b8,0x9b81_d902]);
        assert_eq!(absolute_memory_word_read(r(2),r(1),0xfe01_6e20).unwrap(),[0x0fe0_45b8,0x9b81_d802]);
        assert!(absolute_memory_word_read(r(2),r(1),0xfe01_6e21).is_err());
        assert_eq!(return_if_revision_initialized(r(2)).unwrap(),0x5920_3fc7);
        assert_eq!(store_and_create_context(r(0),r(1),0).unwrap(),0xa010_3c00);
        assert!(store_and_create_context(r(0),r(1),8).is_err());
        assert!(return_if_not_new_predicate(r(0)).is_err());
    }
}
