// SPDX-License-Identifier: GPL-3.0-only
//! CDSP RWE link-state recorder and GP control reset, file 0x560d4..0x56114.
//! Three short function groups in the same `PT_LOAD`; earlier 0x56054..0x560d4
//! contains separate code and padding not yet source-generated and is NOT
//! counted as reconstructed executable bytes.
use super::cdsp_boot::{GeneralRegister,PacketMarker,ScalarPacket};
use super::cdsp_late_mmio::encode_word_store_register;

pub const FILE_OFFSET:usize=0x560d4;
pub const ELF_VADDR:u32=0xc00c_80d4;
pub const PHYSICAL_ADDRESS:u32=0x96cc_80d4;
pub const VERIFIED_SIZE:usize=0x40;

fn push(out:&mut Vec<u8>,v:u32){out.extend_from_slice(&v.to_le_bytes());}

/// Two-word Hexagon `R0=add(PC,##positive_disp)` with 32-bit immediate.
/// The extender carries high 26 bits and the PC-add field stores low six
/// immediate bits shifted left seven; both source operands are checked and
/// independently assembled by LLVM6.
pub fn encode_extended_pic_address(dst:GeneralRegister,relative:u32)->Result<[u32;2],String>{
    if dst.number()!=0 || ![0x001c_3f2c,0x001c_3f24].contains(&relative){
        return Err("Only independently confirmed CDSP R0=PC + 0x1c3f2c/0x1c3f24 supported".into());
    }
    let extender=((relative>>20)<<16)|((relative>>6)&0x3fff)|0x4000;
    let consumer=0x6a49_c000 |((relative&0x3f)<<7)|u32::from(dst.number());
    Ok([extender,consumer])
}

/// One-word signed immediate `R0=-1`; no fake extender is generated.
pub fn encode_one_word_negative_one(dst:GeneralRegister,value:i32)->Result<u32,String>{
    if dst.number()!=0 || value!=-1 {
        return Err("Only checked CDSP R0=#-1 non-extender Hexagon literal is accepted".into());
    }
    Ok(0x78df_ffe0)
}

/// Two-word extended jump from physical 0x96cc80f4 directly to the later
/// original code at 0x96dc3ec0. The displacement was LLVM6 independently
/// assembled from source `jump ##0xfbdcc`, no OEM instruction payload used.
pub fn encode_external_jump(displacement_bytes:u32)->Result<[u32;2],String>{
    if displacement_bytes!=0xfbdcc{
        return Err("CDSP register handoff jump only allows proven +0xfbdcc".into());
    }
    let extender=((displacement_bytes>>20)<<16)|((displacement_bytes>>6)&0x3fff)|0x4000;
    // The short low operand field is a separately LLVM6-confirmed selector;
    // this helper does not invent unproven farther target formats.
    Ok([extender,0x5800_c018])
}

/// V66 GP-relative `.new` scalar store of zero into one checked controller
/// runtime pointer. The source GP store is intentionally a four-word packet
/// with forced 32-bit offset, *not* the 3-word compressed LLVM6 variant.
pub fn encode_gp_zero_store(absolute:u32,producer:GeneralRegister)->Result<[u32;4],String>{
    if absolute!=0xc028_c014 || producer.number()!=2{
        return Err("CDSP GP zero store requires R2.new and the validated 0xc028c014 target".into());
    }
    let bits=absolute;
    let ext=((bits>>20)<<16)|((bits>>6)&0x3fff)|0x4000;
    let r2=ScalarPacket::AssignSmallImmediate{dst:producer,value:0}.encode()?;
    Ok([0x4000,PacketMarker::ContinuationA.apply(r2),ext,0x48a0_d200|(bits&0x3f)])
}

/// Restore via a leaf return stub, save PC-relative LR and SP state to
/// controller globals, jump into a late runtime helper, then clear a global
/// table entry using a V66 `.new` packet. The 0x560fc NOP is a real A32-like
/// Hexagon NOP instruction in the original source, not padding outside a
/// code region.
pub fn build_link_state_and_gp_clear()->Result<Vec<u8>,String>{
    let r0=GeneralRegister::new(0)?;
    let r31=GeneralRegister::new(31)?;
    let r29=GeneralRegister::new(29)?;
    let r2=GeneralRegister::new(2)?;
    let mut out=Vec::with_capacity(VERIFIED_SIZE);
    push(&mut out,ScalarPacket::JumpRegister{source:r31}.encode()?);
    for word in encode_extended_pic_address(r0,0x001c_3f2c)?{push(&mut out,word);}
    push(&mut out,encode_word_store_register(r0,r31));
    for word in encode_extended_pic_address(r0,0x001c_3f24)?{push(&mut out,word);}
    push(&mut out,encode_word_store_register(r0,r29));
    push(&mut out,encode_one_word_negative_one(r0,-1)?);
    for word in encode_external_jump(0xfbdcc)?{push(&mut out,word);}
    push(&mut out,ScalarPacket::Nop.encode()?);
    for word in encode_gp_zero_store(0xc028_c014,r2)?{push(&mut out,word);}
    push(&mut out,ScalarPacket::JumpRegister{source:r31}.encode()?);
    if out.len()!=VERIFIED_SIZE{return Err("CDSP GP controller bookkeeping must have exactly 64 bytes".into());}
    Ok(out)
}

pub fn verify_link_state_and_gp_clear(reference:&[u8])->Result<(),String>{
    super::cdsp_boot::validate_reference(reference)?;
    let bytes=build_link_state_and_gp_clear()?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+VERIFIED_SIZE)!=Some(bytes.as_slice()){
        return Err("CDSP GP+PIC controller sequence differs from pinned original".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn pc_relative_assembler_fields_and_gp_packet_are_bounded(){
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(encode_extended_pic_address(r(0),0x1c3f2c).unwrap(),[0x0001_70fc,0x6a49_d600]);
        assert_eq!(encode_extended_pic_address(r(0),0x1c3f24).unwrap(),[0x0001_70fc,0x6a49_d200]);
        assert!(encode_extended_pic_address(r(1),0x1c3f2c).is_err());
        assert_eq!(encode_one_word_negative_one(r(0),-1).unwrap(),0x78df_ffe0);
        assert!(encode_one_word_negative_one(r(0),0).is_err());
        assert_eq!(encode_external_jump(0xfbdcc).unwrap(),[0x0000_7ef7,0x5800_c018]);
        assert!(encode_external_jump(0xfb000).is_err());
        assert_eq!(encode_gp_zero_store(0xc028c014,r(2)).unwrap(),[0x0000_4000,0x7800_4002,0x0c02_6300,0x48a0_d214]);
        assert!(encode_gp_zero_store(0xc028c014,r(1)).is_err());
    }
    #[test]
    fn gp_controller_leaf_functions_do_not_cross_segment_boundaries(){
        let a=build_link_state_and_gp_clear().unwrap();
        assert_eq!(a.len(),64);
        assert_eq!(&a[..4],&0x529f_c000_u32.to_le_bytes());
        assert_eq!(&a[60..64],&0x529f_c000_u32.to_le_bytes());
        assert_eq!(FILE_OFFSET+a.len(),0x56114);
    }
}
