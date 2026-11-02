// SPDX-License-Identifier: GPL-3.0-only
//! Snapshot an exception context across user/supervisor registers, including
//! PMUCFG, MODECTL, UTIMER, EVB, diagnostics, thread flags, and interrupt
//! masks. Privileged register selectors are validated with Rizin Hexagon
//! V66; regular stores/loads and ABI frame instructions are LLVM6 checked.
use super::cdsp_boot::{GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_exception_finalize;

pub const FILE_OFFSET:usize=0x495c;
pub const REGION_END:usize=0x4a08;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_195c;
fn put(out:&mut Vec<u8>,word:u32,parse:P){out.extend_from_slice(&parse.apply(word).to_le_bytes());}
fn read_special(dst:Gpr,selector:u8)->Result<u32,String>{
    if selector>63{return Err("CDSP unsupported 6-bit privileged register ID".into());}
    Ok(0x6e80_c000|(u32::from(selector)<<16)|u32::from(dst.number()))
}
fn read_special_pair(low:Gpr,selector:u8)->Result<u32,String>{
    if low.number()%2!=0||selector>63{return Err("CDSP paired S register must target even destination register and valid ID".into());}
    Ok(0x6800_c000|(u32::from(selector)<<16)|u32::from(low.number()))
}
fn read_predicate_bits(dst:Gpr)->u32{0x6a04_c000|u32::from(dst.number())}
fn write_predicate_bits(src:Gpr)->u32{0x6220_c004|(u32::from(src.number())<<16)}
fn allocframe(local_bytes:u16)->Result<u32,String>{
    if local_bytes==0||local_bytes>0x1ff8||local_bytes%8!=0{
        return Err("CDSP allocframe requires nonzero, aligned frame in range".into());
    }
    Ok(0xa09d_c000|u32::from(local_bytes/8))
}
fn store_pair(low:Gpr,base:Gpr,offset:u16)->Result<u32,String>{
    if low.number()%2!=0||offset%8!=0||offset>0x3f8{return Err("CDSP paired store alignment or offset out of range".into());}
    Ok(0xa1c0_c000|(u32::from(base.number())<<16)
        |(u32::from(low.number())<<8)|u32::from(offset/8))
}
fn load_pair(low:Gpr,base:Gpr,offset:u16)->Result<u32,String>{
    if low.number()%2!=0||offset%8!=0||offset>0x3f8{return Err("CDSP paired load alignment or offset out of range".into());}
    Ok(0x91c0_c000|(u32::from(base.number())<<16)
        |(u32::from(offset/8)<<8)|u32::from(low.number()))
}
fn store_word(src:Gpr,base:Gpr,offset:u16)->Result<u32,String>{
    if offset%4!=0||offset>0x3fc{return Err("CDSP word-store offset must be in-range/aligned".into());}
    Ok(0xa180_c000|(u32::from(base.number())<<16)
        |(u32::from(src.number())<<8)|u32::from(offset/4))
}
fn get_imask(dst:Gpr,index:Gpr)->u32{
    0x6600_c000|(u32::from(index.number())<<16)|u32::from(dst.number())
}

pub fn build_context_snapshot()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    put(&mut out,read_predicate_bits(r(19)?),P::ContinuationA);
    put(&mut out,H::AssignRegister{dst:r(18)?,src:r(28)?}.encode()?,P::ContinuationA);
    put(&mut out,allocframe(8)?,P::Final);
    put(&mut out,store_pair(r(18)?,r(29)?,0)?,P::Final);
    put(&mut out,read_special(r(18)?,53)?,P::Final); // PMUCFG
    put(&mut out,read_special(r(19)?,17)?,P::Final); // MODECTL
    put(&mut out,store_pair(r(18)?,r(0)?,0)?,P::Final);
    put(&mut out,read_special_pair(r(18)?,30)?,P::Final); // UTIMER 64-bit
    put(&mut out,store_pair(r(18)?,r(0)?,8)?,P::Final);
    put(&mut out,read_special(r(18)?,16)?,P::Final); // EVB
    put(&mut out,read_special(r(19)?,28)?,P::Final); // DIAG
    put(&mut out,store_pair(r(18)?,r(0)?,16)?,P::Final);
    put(&mut out,read_special(r(18)?,20)?,P::Final); // S20
    put(&mut out,read_special(r(19)?,20)?,P::Final); // S20 second sample
    put(&mut out,store_pair(r(18)?,r(0)?,24)?,P::Final);
    put(&mut out,read_special(r(18)?,29)?,P::Final); // REV
    put(&mut out,read_special(r(19)?,0)?,P::Final); // SGP0
    put(&mut out,H::AssignRegister{dst:r(18)?,src:r(1)?}.encode()?,P::ContinuationA);
    put(&mut out,H::AssignSmallImmediate{dst:r(19)?,value:0}.encode()?,P::ContinuationA);
    put(&mut out,store_pair(r(18)?,r(0)?,40)?,P::Final);
    put(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:0}.encode()?,P::ContinuationA);
    put(&mut out,store_pair(r(18)?,r(0)?,64)?,P::Final);
    // Four distinct interrupt-mask levels selected by integer R1=0..3.
    for mask in 0..4 {
        put(&mut out,get_imask(r(28)?,r(1)?),P::Final);
        if mask<3 {
            put(&mut out,H::AssignSmallImmediate{dst:r(1)?,value:mask+1}.encode()?,P::ContinuationA);
            put(&mut out,store_word(r(28)?,r(0)?,48+u16::from(mask)*4)?,P::Final);
        } else {
            put(&mut out,load_pair(r(18)?,r(29)?,0)?,P::ContinuationA);
            put(&mut out,store_word(r(28)?,r(0)?,60)?,P::Final);
        }
    }
    for (selector,offset) in [(63,68),(60,72),(35,88)] {
        put(&mut out,read_special(r(28)?,selector)?,P::Final);
        put(&mut out,store_word(r(28)?,r(0)?,offset)?,P::Final);
    }
    put(&mut out,write_predicate_bits(r(19)?),P::ContinuationA);
    put(&mut out,H::AssignRegister{dst:r(28)?,src:r(18)?}.encode()?,P::ContinuationA);
    // Architectural Hexagon dealloc_return has no register operands, unlike
    // a forced packed OEM byte sequence; LLVM6 independently assembles it.
    put(&mut out,0x961e_c01e,P::Final);
    if out.len()!=FUNCTION_BYTES{return Err(format!("CDSP context snapshot emitted {} bytes instead of {}",out.len(),FUNCTION_BYTES));}
    Ok(out)
}

pub fn build_supervisor_with_snapshot()->Result<Vec<u8>,String>{
    if cdsp_early_exception_finalize::REGION_END!=FILE_OFFSET{return Err("CDSP snapshot first address does not follow exception return".into());}
    let mut out=cdsp_early_exception_finalize::build_supervisor_with_finalize()?;
    out.extend(build_context_snapshot()?);
    Ok(out)
}
pub fn verify_reference(original:&[u8])->Result<(),String>{
    cdsp_early_exception_finalize::verify_reference(original)?;
    if original.get(FILE_OFFSET..REGION_END)!=Some(build_context_snapshot()?.as_slice()){
        return Err("CDSP captured supervisor status differs from original pinned firmware".into());
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test]fn register_snapshot_has_bounded_frame_and_offset_rules(){
        assert_eq!(FUNCTION_BYTES,172);
        assert_eq!(build_context_snapshot().unwrap().len(),FUNCTION_BYTES);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_195c);
        assert!(allocframe(0).is_err());
        assert!(allocframe(9).is_err());
        assert!(store_pair(Gpr::new(19).unwrap(),Gpr::new(0).unwrap(),8).is_err());
        assert!(load_pair(Gpr::new(18).unwrap(),Gpr::new(0).unwrap(),9).is_err());
        assert!(store_word(Gpr::new(28).unwrap(),Gpr::new(0).unwrap(),3).is_err());
        assert_ne!(get_imask(Gpr::new(28).unwrap(),Gpr::new(1).unwrap()),
                   get_imask(Gpr::new(28).unwrap(),Gpr::new(2).unwrap()));
    }
}
