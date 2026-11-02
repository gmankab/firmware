// SPDX-License-Identifier: GPL-3.0-only
//! CDSP RWE runtime callback initializer, starting at source MBN file 0x56000.
//! This is genuinely executable Hexagon V66 code: unlike metadata-heavy
//! earlier `R E` PT_LOAD, the mapped RWE segment contains instructions.
//! Its virtual address and physical mapped address are different. Only
//! proven architecture operand encodings are used; no original firmware
//! instruction bytes are copied in from a reference image.
use super::cdsp_boot::{GeneralRegister,PacketMarker,ScalarPacket};
use super::cdsp_late_mmio::{encode_word_load_immediate,encode_word_store_register};

pub const FILE_OFFSET:usize=0x56000;
pub const ELF_VADDR:u32=0xc00c_8000;
pub const PHYSICAL_ADDRESS:u32=0x96cc_8000;
pub const FIRST_FUNCTION_SIZE:usize=0x54;
// Exactly twelve source-image padding bytes follow the return at file 0x56054.
// They are not part of the typed executable emitter or reconstructed code count.
pub const FOLLOWING_ZERO_PADDING:usize=0x0c;

fn push_u32(out:&mut Vec<u8>,instruction:u32){out.extend_from_slice(&instruction.to_le_bytes());}

/// Hexagon `allocframe(SP,#8)`. Small frame sizes are a noncontinuous
/// encoding family, so only the exact 8-byte form that LLVM6 verified is
/// supported here rather than guessing a wider operand range.
pub fn encode_allocframe8(frame_bytes:u16)->Result<u32,String>{
    if frame_bytes!=8 {return Err("Only LLVM6-confirmed 8-byte CDSP frame allocation is supported".into());}
    Ok(0xa09d_c000|(u32::from(frame_bytes)/8))
}

/// Hexagon `Rreg.h/l=#immediate16`. Its signed 16-bit source value is split
/// into low 14-bit field and upper two bits at [23:22], as separately tested
/// with LLVM6 high/low-halfword literal fixtures.
pub fn encode_halfword_literal16(dst:GeneralRegister,high:bool,value:u16)->u32{
    let base=if high {0x7220_c000}else{0x7120_c000};
    base|(u32::from(dst.number())<<16)
        |(u32::from(value)&0x3fff)
        |((u32::from(value)&0xc000)<<8)
}

/// Hexagon `callr Rreg` stores a return link into LR and invokes the named
/// register. LLVM6 independently confirms R0 and R28 encoder fields.
pub fn encode_callr(target:GeneralRegister)->u32{
    0x50a0_c000|(u32::from(target.number())<<16)
}

/// Low-register duplex: `Rleft=#0; Rright=#0`. This is an actual one-word
/// Hexagon duplex pair with explicit register fields, *not* two 32-bit words.
/// Only GPRs within the independently verified low-index bank are accepted.
pub fn encode_duplex_zero_regs(left:GeneralRegister,right:GeneralRegister)->Result<u32,String>{
    if left.number()>7 || right.number()>7 || left==right{
        return Err("CDSP zero duplex requires two distinct GPRs in R0..R7".into());
    }
    Ok(0x2800_2800|(u32::from(left.number())<<16)|u32::from(right.number()))
}

/// CDSP `deallocframe FP`, the independently LLVM6-assembled scalar form.
pub fn encode_deallocframe()->u32{0x901e_c01e}

/// Initialize the CDSP RWE callback table, execute the optional startup hook,
/// run any registered callbacks, then restore caller frame and return. Packet
/// ordering is explicit to preserve the original source's duplex and
/// two-operation packet boundaries.
pub fn build_runtime_callback_initializer()->Result<Vec<u8>,String>{
    let r=|n|GeneralRegister::new(n);
    let mut out=Vec::with_capacity(FIRST_FUNCTION_SIZE);
    push_u32(&mut out,encode_allocframe8(8)?);
    push_u32(&mut out,encode_word_store_register(r(29)?,r(27)?));
    push_u32(&mut out,encode_halfword_literal16(r(0)?,true,0xc028));
    push_u32(&mut out,encode_halfword_literal16(r(0)?,false,0x9d20));
    push_u32(&mut out,encode_word_load_immediate(r(1)?,r(0)?,0)?);
    push_u32(&mut out,ScalarPacket::SkipIfSourceZeroNotTaken{src:r(1)?,displacement_bytes:24}.encode()?);
    // A packet containing one ordinary instruction and one two-way duplex.
    // The field order, rather than LLVM's source-statement order, encodes R2
    // as a standalone continuation and R3/R1 together in a single word.
    push_u32(&mut out,PacketMarker::ContinuationA.apply(
        ScalarPacket::AssignSmallImmediate{dst:r(2)?,value:0}.encode()?));
    push_u32(&mut out,encode_duplex_zero_regs(r(3)?,r(1)?)?);
    push_u32(&mut out,encode_halfword_literal16(r(28)?,true,0xc01e));
    push_u32(&mut out,encode_halfword_literal16(r(28)?,false,0x0bfc));
    push_u32(&mut out,encode_callr(r(28)?));
    push_u32(&mut out,encode_halfword_literal16(r(27)?,true,0xc028));
    push_u32(&mut out,encode_halfword_literal16(r(27)?,false,0xb008));
    push_u32(&mut out,ScalarPacket::AddSignedSmallImmediate{dst:r(27)?,src:r(27)?,value:-4}.encode()?);
    push_u32(&mut out,encode_word_load_immediate(r(0)?,r(27)?,0)?);
    push_u32(&mut out,ScalarPacket::SkipIfSourceZeroNotTaken{src:r(0)?,displacement_bytes:12}.encode()?);
    push_u32(&mut out,encode_callr(r(0)?));
    push_u32(&mut out,ScalarPacket::JumpBackward{displacement_bytes:-16}.encode()?);
    push_u32(&mut out,PacketMarker::ContinuationA.apply(encode_word_load_immediate(r(27)?,r(29)?,0)?));
    push_u32(&mut out,encode_deallocframe());
    push_u32(&mut out,ScalarPacket::JumpRegister{source:r(31)?}.encode()?);
    if out.len()!=FIRST_FUNCTION_SIZE{
        return Err("CDSP RWE callback initializer size deviates from 84 executable bytes".into());
    }
    Ok(out)
}

pub fn verify_runtime_callback_initializer(reference:&[u8])->Result<(),String>{
    super::cdsp_boot::validate_reference(reference)?;
    let generated=build_runtime_callback_initializer()?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+FIRST_FUNCTION_SIZE)!=Some(generated.as_slice()){
        return Err("CDSP RWE callback startup differs from pinned original bytes".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn runtime_duplex_and_halfwords_use_checked_fields(){
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(encode_allocframe8(8).unwrap(),0xa09d_c001);
        assert!(encode_allocframe8(16).is_err());
        assert_eq!(encode_halfword_literal16(r(0),true,0xc028),0x72e0_c028);
        assert_eq!(encode_halfword_literal16(r(0),false,0x9d20),0x71a0_dd20);
        assert_eq!(encode_halfword_literal16(r(28),true,0xc01e),0x72fc_c01e);
        assert_eq!(encode_duplex_zero_regs(r(3),r(1)).unwrap(),0x2803_2801);
        assert!(encode_duplex_zero_regs(r(8),r(1)).is_err());
        assert_eq!(encode_callr(r(28)),0x50bc_c000);
        assert_eq!(encode_deallocframe(),0x901e_c01e);
    }

    #[test]
    fn source_built_runtime_callback_has_exact_84_byte_function_boundary(){
        let bytes=build_runtime_callback_initializer().unwrap();
        assert_eq!(bytes.len(),FIRST_FUNCTION_SIZE);
        let word=|idx:usize|u32::from_le_bytes(bytes[idx*4..idx*4+4].try_into().unwrap());
        assert_eq!(word(7),encode_duplex_zero_regs(GeneralRegister::new(3).unwrap(),GeneralRegister::new(1).unwrap()).unwrap());
        assert_eq!(word(17),ScalarPacket::JumpBackward{displacement_bytes:-16}.encode().unwrap());
        assert_eq!(word(19),encode_deallocframe());
        assert_eq!(word(20),ScalarPacket::JumpRegister{source:GeneralRegister::new(31).unwrap()}.encode().unwrap());
        assert_eq!(FILE_OFFSET+bytes.len(),0x56054);
    }
}
