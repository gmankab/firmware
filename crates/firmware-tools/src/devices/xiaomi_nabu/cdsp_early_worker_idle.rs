// SPDX-License-Identifier: GPL-3.0-only
//! CDSP worker idle/cleanup on controller mode 1. Three GP pointer guards
//! precede typed 32-bit and duplex zero stores; the path ends in a self-jump.
//! Hexagon V66 packet markers are emitted separately from instruction fields.
use super::cdsp_boot::{self,GeneralRegister as Gpr,PacketMarker as P,ScalarPacket as H};
use super::cdsp_early_worker_trap;

pub const FILE_OFFSET:usize=0x4cd8;
pub const REGION_END:usize=0x4d24;
pub const FUNCTION_BYTES:usize=REGION_END-FILE_OFFSET;
pub const PHYSICAL_ADDRESS:u32=0x96c0_1cd8;
fn put(out:&mut Vec<u8>,word:u32,parse:P){out.extend_from_slice(&parse.apply(word).to_le_bytes());}

/// Coupled V66 GP-relative load + new-result predicate branch; the branch
/// displacement is a nine-bit packet-relative halfword count with checked
/// forward alignment. Only the evidenced R0 source is supported.
fn skip_on_gp_zero(packet_start:u32,target:u32,taken:bool)->Result<u32,String>{
    if packet_start%4!=0 || target%4!=0 || target<=packet_start {
        return Err("GP new-value conditional branch requires aligned forward target".into());
    }
    let distance=target-packet_start;
    if distance>0x1fe || distance%2!=0 {
        return Err("GP new-value branch displacement outside V66 short range".into());
    }
    Ok((if taken {0x2402_e000} else {0x2402_c000})|(distance/2))
}
/// Compact two zero stores in a V66 duplex word. Both source memory offsets
/// must be 4-byte aligned; the source base is the observed `R0` only.
fn duplex_zero_two_words(first:u8,second:u8)->Result<u32,String>{
    if first%4!=0 || second%4!=0 || first>124 || second>124 {
        return Err("CDSP duplex MEMW-zero requires aligned bounded offsets".into());
    }
    Ok(0xf000_1000 | (u32::from(first/4)<<16)|u32::from(second/4))
}
/// Word zeroing with a single 32-bit instruction. The byte offset is scaled
/// by four and stored in bits 13:7; packet end is supplied by the caller.
fn zero_word_at_r0(byte_offset:u8)->Result<u32,String>{
    if byte_offset>124 || byte_offset%4!=0 {return Err("CDSP word zero offset out of bounds".into());}
    Ok(0x3c40_c000|(u32::from(byte_offset/4)<<7))
}
/// Atomic bit clear within `memw(R0+#4)`; the observed family encodes the
/// bit position in the low five operand bits, not as an opaque vendor word.
fn clear_word_bit_at_r0_plus4(bit:u8)->Result<u32,String>{
    if bit>=32{return Err("memory clrbit only admits bit indices 0..31".into());}
    Ok(0x3f40_c0c0|u32::from(bit))
}

pub fn build_worker_idle()->Result<Vec<u8>,String>{
    let r=|n|Gpr::new(n);
    let mut out=Vec::with_capacity(FUNCTION_BYTES);
    for (where_,address,taken) in [(0x96c0_1cd8,0xfe0a_c0a0,true),
                                    (0x96c0_1ce4,0xfe0a_5ddc,true),
                                    (0x96c0_1cf0,0xfe03_7ad0,false)] {
        let gp=cdsp_boot::encode_extended_gp_word_load(r(0)?,address);
        put(&mut out,gp[0],P::ContinuationA);
        put(&mut out,gp[1],P::ContinuationA);
        put(&mut out,skip_on_gp_zero(where_,0x96c0_1d20,taken)?,P::Final);
    }
    // The two duplex words contain four stores; the next six instructions
    // clear additional pointer pairs with explicit two-instruction packets.
    out.extend_from_slice(&duplex_zero_two_words(36,32)?.to_le_bytes());
    out.extend_from_slice(&duplex_zero_two_words(16,20)?.to_le_bytes());
    for (first,second) in [(64,60),(92,88),(120,116)] {
        put(&mut out,zero_word_at_r0(first)?,P::ContinuationA);
        put(&mut out,zero_word_at_r0(second)?,P::Final);
    }
    put(&mut out,clear_word_bit_at_r0_plus4(21)?,P::Final);
    put(&mut out,H::JumpRelative{displacement_bytes:0}.encode()?,P::Final);
    if out.len()!=FUNCTION_BYTES {return Err("CDSP worker idle path has wrong instruction count".into());}
    Ok(out)
}
pub fn verify_reference(reference:&[u8])->Result<(),String>{
    cdsp_early_worker_trap::verify_reference(reference)?;
    if cdsp_early_worker_trap::REGION_END!=FILE_OFFSET {
        return Err("CDSP worker idle region does not immediately follow fatal mode".into());
    }
    let generated=build_worker_idle()?;
    if reference.get(FILE_OFFSET..REGION_END)!=Some(generated.as_slice()){
        return Err("CDSP worker idle/cleanup instructions differ from pinned reference".into());
    }
    Ok(())
}
pub fn build_supervisor_with_idle()->Result<Vec<u8>,String>{
    let mut out=cdsp_early_worker_trap::build_supervisor_with_fatal_path()?;
    out.extend(build_worker_idle()?);
    Ok(out)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn worker_idle_checked_offsets_and_packet_boundaries(){
        assert_eq!(FUNCTION_BYTES,76);
        assert_eq!(build_worker_idle().unwrap().len(),76);
        assert_eq!(skip_on_gp_zero(0x96c0_1cd8,0x96c0_1d20,true).unwrap(),0x2402_e024);
        assert_eq!(skip_on_gp_zero(0x96c0_1cf0,0x96c0_1d20,false).unwrap(),0x2402_c018);
        assert!(skip_on_gp_zero(0x96c0_1cf0,0x96c0_1cf0,false).is_err());
        assert_eq!(duplex_zero_two_words(36,32).unwrap(),0xf009_1008);
        assert_eq!(duplex_zero_two_words(16,20).unwrap(),0xf004_1005);
        assert!(duplex_zero_two_words(3,20).is_err());
        assert_eq!(zero_word_at_r0(92).unwrap(),0x3c40_cb80);
        assert!(zero_word_at_r0(125).is_err());
        assert_eq!(clear_word_bit_at_r0_plus4(21).unwrap(),0x3f40_c0d5);
        assert!(clear_word_bit_at_r0_plus4(32).is_err());
    }
}
