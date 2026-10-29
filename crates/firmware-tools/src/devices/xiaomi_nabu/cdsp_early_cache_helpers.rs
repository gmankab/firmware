// SPDX-License-Identifier: GPL-3.0-only
//! Four adjacent CDSP V66 cache/runtime helpers: copy/clean, CFGBASE physical
//! probe, dual-buffer initialization and physical-word snapshot. Every code
//! word is emitted from checked operands, not pasted OEM instruction arrays.
use super::cdsp_boot::{self, GeneralRegister as Gpr, PacketMarker as P, ScalarPacket as H, SystemRegister};
use super::{cdsp_early_icache_tags as previous, cdsp_early_worker_command as worker,
            cdsp_early_controllers as controllers};

pub const FILE_OFFSET:usize=0x53a4;
pub const REGION_END:usize=0x5488;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_23a4;

fn put(v:&mut Vec<u8>,word:u32,marker:P){v.extend_from_slice(&marker.apply(word).to_le_bytes());}

fn two_register_copies(dst1:Gpr,src1:Gpr,dst2:Gpr,src2:Gpr)->Result<u32,String>{
    // Checked against LLVM6 for both exact source/destination pairs seen in
    // the firmware; the compact duplex family is not assumed general.
    match (dst1.number(),src1.number(),dst2.number(),src2.number()) {
        (6,2,5,1)=>Ok(0x3026_3015),
        (1,5,0,6)=>Ok(0x3051_3060),
        _=>Err("unverified Hexagon register-move duplex layout".into()),
    }
}
fn load_postincrement(dst:Gpr,base:Gpr,increment:u8)->Result<u32,String>{
    if increment!=4||dst.number()>15||base.number()>7 {
        return Err("V66 source loader requires 4-byte postincrement and compact base".into());
    }
    Ok(0x9b80_c000 | (u32::from(base.number())<<16) |
       (u32::from(increment)<<3) | u32::from(dst.number()))
}
fn store_postincrement(src:Gpr,base:Gpr,increment:u8)->Result<u32,String>{
    if increment!=4||src.number()>15||base.number()>7 {
        return Err("V66 destination store requires 4-byte postincrement and compact source".into());
    }
    Ok(0xab80_c000 | (u32::from(base.number())<<16) |
       (u32::from(src.number())<<8) | (u32::from(increment)<<1))
}
fn physical_word(dst:Gpr,offset:Gpr,base:Gpr)->Result<u32,String>{
    if dst.number()>7||offset.number()>7||base.number()>7 {
        return Err("V66 privileged physical word reader restricts checked GP bank".into());
    }
    Ok(0x9200_e000 | (u32::from(offset.number())<<16) |
       (u32::from(base.number())<<8) | u32::from(dst.number()))
}
fn add_minus_256(dst:Gpr,src:Gpr)->Result<u32,String>{
    if dst.number()>7||src.number()>7 {
        return Err("signed minus-256 ADD restricted to independently validated registers".into());
    }
    let minus_256 = (-256_i32 as u32)&0x7ff;
    Ok(0xbfe0_0000 | (u32::from(src.number())<<16)
      | (minus_256<<5) | u32::from(dst.number()))
}
fn absolute_word_store(src:Gpr,base:Gpr,address:u32)->Result<[u32;2],String>{
    if src.number()>7 || base.number()>7 || address%4!=0 {
        return Err("V66 controller status pointer requires low registers and aligned address".into());
    }
    Ok([((address>>20)<<16)|((address>>6)&0x3fff)|0x4000,
        0xab80_c080|(u32::from(base.number())<<16)|
        (u32::from(src.number())<<8)|(address&0x3f)])
}
fn physical_word_mask(dst:Gpr,src:Gpr,width:u32)->Result<[u32;2],String>{
    // Independent LLVM6 assembly of `r0=and(r0,##0x7ff)` confirms exactly
    // this extended immediate layout and the split high/low fields.
    if dst.number()>7||src.number()>7||width!=0x7ff {
        return Err("V66 mask currently validated only for 11-bit physical addressing".into());
    }
    Ok([((width>>20)<<16)|((width>>6)&0x3fff)|0x4000,
        0x7600_c000|(u32::from(src.number())<<16)|
        ((width&0x3f)<<5)|u32::from(dst.number())])
}

/// Single hardware-looped word copy with a second loop cleaning the copied
/// D-cache lines, preserving both architectural loop-end parse markers.
fn copy_then_clean(out:&mut Vec<u8>)->Result<(),String>{
    let r=|n|Gpr::new(n);
    put(out,H::LogicalShiftRight{dst:r(2)?,src:r(2)?,amount:2}.encode()?,P::ContinuationA);
    out.extend_from_slice(&two_register_copies(r(6)?,r(2)?,r(5)?,r(1)?)?.to_le_bytes());
    put(out,H::HardwareLoop{loop_index:0,counter:r(2)?,displacement_bytes:8}.encode()?,P::ContinuationA);
    put(out,H::AssignSmallImmediate{dst:r(2)?,value:0}.encode()?,P::Final);
    put(out,load_postincrement(r(4)?,r(0)?,4)?,P::Final);
    put(out,H::Nop.encode()?,P::LoopEnd);
    put(out,store_postincrement(r(4)?,r(1)?,4)?,P::Final);
    out.extend_from_slice(&two_register_copies(r(1)?,r(5)?,r(0)?,r(6)?)?.to_le_bytes());
    put(out,H::LogicalShiftRight{dst:r(0)?,src:r(0)?,amount:5}.encode()?,P::Final);
    put(out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:1}.encode()?,P::Final);
    put(out,H::HardwareLoop{loop_index:0,counter:r(0)?,displacement_bytes:4}.encode()?,P::Final);
    put(out,H::AddSmallImmediate{dst:r(1)?,src:r(1)?,value:32}.encode()?,P::LoopEnd);
    put(out,worker::cache_clean(r(1)?),P::Final);
    put(out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    Ok(())
}

/// Read CFGBASE-selected physical controller words and publish a cache-clean
/// status record. Hardware physical reads are independently Rizin-decoded.
fn probe_cfgbase(out:&mut Vec<u8>)->Result<(),String>{
    let r=|n|Gpr::new(n);
    put(out,H::ReadSystem{system:SystemRegister::Cfgbase,dst:r(0)?}.encode()?,P::ContinuationA);
    put(out,H::AssignSmallImmediate{dst:r(1)?,value:0}.encode()?,P::Final);
    put(out,H::InsertBitField{dst:r(0)?,src:r(1)?,width:12,offset:20}.encode()?,P::Final);
    put(out,H::ArithmeticShiftLeft{dst:r(0)?,src:r(0)?,amount:5}.encode()?,P::ContinuationA);
    put(out,H::AssignSmallImmediate{dst:r(2)?,value:8}.encode()?,P::Final);
    put(out,physical_word(r(0)?,r(2)?,r(0)?)?,P::Final);
    put(out,H::InsertBitField{dst:r(0)?,src:r(1)?,width:12,offset:20}.encode()?,P::Final);
    put(out,H::ArithmeticShiftLeft{dst:r(0)?,src:r(0)?,amount:5}.encode()?,P::ContinuationA);
    put(out,H::AssignSmallImmediate{dst:r(2)?,value:68}.encode()?,P::Final);
    put(out,add_minus_256(r(0)?,r(0)?)?,P::Final);
    put(out,physical_word(r(0)?,r(2)?,r(0)?)?,P::Final);
    let status=absolute_word_store(r(0)?,r(1)?,0xfe01_6e1c)?;
    put(out,status[0],P::ContinuationA);put(out,status[1],P::Final);
    put(out,worker::cache_clean(r(1)?),P::Final);
    put(out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    Ok(())
}

/// Call the previous copy/clean helper for the controller's two 128-byte
/// source buffers. These are original Hexagon extended CALL instructions.
fn initialize_two_buffers(out:&mut Vec<u8>)->Result<(),String>{
    let r=|n|Gpr::new(n);
    // LLVM6 independently assembles `allocframe(#0)` to 0xa09dc000.
    put(out,0xa09d_c000,P::Final);
    put(out,H::AssignUnsignedImmediate{dst:r(2)?,value:128}.encode()?,P::ContinuationA);
    let left=cdsp_boot::encode_extended_assignment(r(1)?,0xfe01_6f60u32 as i32,true);
    put(out,left[0],P::ContinuationA);put(out,left[1],P::Final);
    let src=cdsp_boot::encode_extended_gp_word_load(r(0)?,0xfe02_6ddc);
    put(out,src[0],P::ContinuationA);put(out,src[1],P::Final);
    let call=controllers::encode_extended_call(0x96c0_2430,0x96c0_23a4)?;
    put(out,call[0],P::ContinuationA);put(out,call[1],P::Final);
    put(out,H::AssignUnsignedImmediate{dst:r(2)?,value:128}.encode()?,P::ContinuationA);
    let right=cdsp_boot::encode_extended_assignment(r(1)?,0xfe01_6fe0u32 as i32,true);
    put(out,right[0],P::ContinuationA);put(out,right[1],P::Final);
    let src2=cdsp_boot::encode_extended_gp_word_load(r(0)?,0xfe02_6de8);
    put(out,src2[0],P::ContinuationA);put(out,src2[1],P::Final);
    let call2=controllers::encode_extended_call(0x96c0_244c,0x96c0_23a4)?;
    put(out,call2[0],P::ContinuationA);put(out,call2[1],P::Final);
    // Independently assembled by LLVM6 `dealloc_return`.
    put(out,0x961e_c01e,P::Final);
    Ok(())
}

/// Mask the loaded physical word to 11 address bits; copy four physical words
/// through a zero-copy hardware loop and end with the original `jumpr LR`.
fn snapshot_physical_words(out:&mut Vec<u8>)->Result<(),String>{
    let r=|n|Gpr::new(n);
    let gp=cdsp_boot::encode_extended_gp_word_load(r(0)?,0xfe01_6d40);
    put(out,gp[0],P::ContinuationA);put(out,gp[1],P::Final);
    put(out,H::LogicalShiftRight{dst:r(2)?,src:r(0)?,amount:11}.encode()?,P::ContinuationA);
    let ptr=cdsp_boot::encode_extended_assignment(r(1)?,0xfe01_7060u32 as i32,true);
    put(out,ptr[0],P::ContinuationA);put(out,ptr[1],P::Final);
    put(out,H::HardwareLoopImmediate{loop_index:0,iterations:4,displacement_bytes:12}.encode()?,P::ContinuationA);
    let mask=physical_word_mask(r(0)?,r(0)?,0x7ff)?;
    put(out,mask[0],P::ContinuationA);put(out,mask[1],P::Final);
    put(out,physical_word(r(3)?,r(0)?,r(2)?)?,P::Final);
    put(out,H::AddSmallImmediate{dst:r(0)?,src:r(0)?,value:4}.encode()?,P::LoopEnd);
    put(out,store_postincrement(r(3)?,r(1)?,4)?,P::Final);
    put(out,H::JumpRegister{source:r(31)?}.encode()?,P::Final);
    Ok(())
}

pub fn build_cache_helper_group()->Result<Vec<u8>,String>{
    let mut data=Vec::with_capacity(FUNCTION_BYTES);
    copy_then_clean(&mut data)?;
    probe_cfgbase(&mut data)?;
    initialize_two_buffers(&mut data)?;
    snapshot_physical_words(&mut data)?;
    if data.len()!=FUNCTION_BYTES{
        return Err(format!("CDSP cache helper group generated {} bytes instead of {}",data.len(),FUNCTION_BYTES));
    }
    Ok(data)
}

pub fn verify_reference(reference:&[u8])->Result<(),String>{
    previous::verify_reference(reference)?;
    if previous::REGION_END!=FILE_OFFSET{
        return Err("CDSP cache helper group is not adjacent to I-cache tag sweep".into());
    }
    if reference.get(FILE_OFFSET..REGION_END)!=Some(build_cache_helper_group()?.as_slice()){
        return Err("CDSP cache helper group differs from original pinned reference".into());
    }
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn cache_copy_and_phys_snapshot_checks(){
        let r=|n|Gpr::new(n).unwrap();
        assert_eq!(FUNCTION_BYTES,228);
        assert_eq!(two_register_copies(r(6),r(2),r(5),r(1)).unwrap(),0x3026_3015);
        assert!(two_register_copies(r(6),r(1),r(5),r(2)).is_err());
        assert_eq!(load_postincrement(r(4),r(0),4).unwrap(),0x9b80_c024);
        assert_eq!(store_postincrement(r(4),r(1),4).unwrap(),0xab81_c408);
        assert!(load_postincrement(r(4),r(0),8).is_err());
        assert_eq!(physical_word(r(3),r(0),r(2)).unwrap(),0x9200_e203);
        assert!(physical_word(r(11),r(0),r(2)).is_err());
        assert_eq!(add_minus_256(r(0),r(0)).unwrap(),0xbfe0_e000);
        assert_eq!(absolute_word_store(r(0),r(1),0xfe01_6e1c).unwrap(),[0x0fe0_45b8,0xab81_c09c]);
        assert!(absolute_word_store(r(0),r(1),0xfe01_6e1d).is_err());
        assert_eq!(physical_word_mask(r(0),r(0),0x7ff).unwrap(),[0x0000_401f,0x7600_c7e0]);
        assert!(physical_word_mask(r(0),r(0),0xfff).is_err());
        assert_eq!(build_cache_helper_group().unwrap().len(),228);
    }
}
