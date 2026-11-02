// SPDX-License-Identifier: GPL-3.0-only
//! CDSP RWE callback-registry cleanup, located in the later Hexagon V66
//! executable PT_LOAD at file [0x561a0,0x561e0). This is a separate
//! function-aligned source-built region; [0x56114,0x561a0) is not claimed.
//!
//! The original compiler intentionally used extended GP-relative loads
//! instead of LLVM6's shorter GP addressing alternatives. The two
//! producer/.new-consumer packet pairings are preserved, and branch offsets
//! are counted from the **start of each packet**, not the consumer address.
use super::cdsp_boot::{self,GeneralRegister,PacketMarker,ScalarPacket};
use super::cdsp_late_runtime::{encode_allocframe8,encode_callr};

pub const FILE_OFFSET:usize=0x561a0;
pub const ELF_VADDR:u32=0xc00c_81a0;
pub const PHYSICAL_ADDRESS:u32=0x96cc_81a0;
pub const EXECUTABLE_LEN:usize=0x40;
pub const END_FILE_OFFSET:usize=FILE_OFFSET+EXECUTABLE_LEN;
const FIRST_GLOBAL:u32=0xc036_3e00;
const CALLBACK_TABLE:u32=0xc028_c024;
const CALLBACK_REGISTRY:u32=0xc028_c014;

fn push(out:&mut Vec<u8>,word:u32){out.extend_from_slice(&word.to_le_bytes());}

/// PC packet starts at `packet_start`, then the compare's implicit R2.new
/// predicate branches to `target`. The new value is identified by packet
/// distance, not a register field, and the destination must be forward.
pub fn encode_r2_new_zero_branch(packet_start:u32,target:u32)->Result<u32,String>{
    if target<=packet_start || target-packet_start>0xfe || (target-packet_start)%2!=0{
        return Err("R2.new branch destination outside verified forward halfword range".into());
    }
    Ok(0x2402_c000 | ((target-packet_start)/2))
}

/// `P0=cmp.eq(R2,#0); if(!P0.new) jump:nt target`.
pub fn encode_p0_not_new_r2_zero(packet_start:u32,target:u32,reg:GeneralRegister)->Result<u32,String>{
    if reg.number()!=2 || target<=packet_start || target-packet_start>0xfe ||
       (target-packet_start)%2!=0 {
        return Err("unsupported combined P0 comparison and not-taken branch".into());
    }
    Ok(0x1040_4000 |(u32::from(reg.number())<<16)|((target-packet_start)/2))
}

/// A packet-end load into R2 whilst simultaneously setting the R16 base
/// register from an immediate extender. The specific memory-offset form
/// was independently observed in LLVM6 and Rizin V66.
pub fn encode_regbase_word_load(dst:GeneralRegister,base:GeneralRegister,
                                absolute:u32)->Result<u32,String>{
    if dst.number()!=2 || base.number()!=16 || absolute!=CALLBACK_TABLE {
        return Err("unsupported register-setting word load addressing".into());
    }
    Ok(0x9b90_5902)
}

/// Ordinary short signed register-pair frame store and 64-bit return duplex.
/// Only the exact observed register pairs are supported until verified forms
/// for other register combinations can be independently assembled.
pub fn encode_pair_frame_io(store:bool,low:GeneralRegister,
                           base:GeneralRegister,offset:u8)->Result<u32,String>{
    if low.number()!=16 || offset!=0 || (store && base.number()!=29) ||
       (!store && base.number()!=29) {
        return Err("unsupported Hexagon R17:16 frame register pair".into());
    }
    Ok(if store {0xa1dd_d000} else {0x3e04_1f40})
}

/// `R1:0=combine(#4,R16)` followed by the conditional indirect R2 call.
pub fn encode_pair_combine_literal(low:GeneralRegister,src:GeneralRegister,
                                   high:u8)->Result<u32,String>{
    if low.number()!=0 || src.number()!=16 || high!=4 {
        return Err("unsupported R1:0 combine immediate/register fields".into());
    }
    Ok(0x7330_6080)
}

/// V66 `.new` store with exactly one producer R2 in the preceding packet
/// slot; this is not a two-word forced-extension form.
pub fn encode_r2_new_store(base:GeneralRegister,offset:u16)->Result<u32,String>{
    if base.number()!=16 || offset!=0 {return Err("unsupported .new store offset/base".into());}
    Ok(0xa1a0_d200 |(u32::from(base.number())<<16)|u32::from(offset))
}

/// Source-build each of the 16 four-byte instruction words, including the
/// original parse-marker boundaries. The resulting 64 bytes are checked
/// independently against the reference but never read original bytes here.
pub fn build_runtime_registry_cleanup()->Result<Vec<u8>,String>{
    let r=|n|GeneralRegister::new(n);
    let mut out=Vec::with_capacity(EXECUTABLE_LEN);
    // First original packet: forced GP extender, load R2, frame allocation.
    for (i,word) in cdsp_boot::encode_extended_gp_word_load(r(2)?,FIRST_GLOBAL).into_iter().enumerate(){
        push(&mut out,if i==1{PacketMarker::ContinuationA.apply(word)}else{word});
    }
    push(&mut out,encode_allocframe8(8)?);
    // Guard the first registry pointer and save the callback register pair.
    push(&mut out,encode_p0_not_new_r2_zero(PHYSICAL_ADDRESS+0x0c,
                                            PHYSICAL_ADDRESS+0x3c,r(2)?)?);
    push(&mut out,encode_pair_frame_io(true,r(16)?,r(29)?,0)?);
    // Load a second callback-table pointer from an absolute register-setting load.
    push(&mut out,cdsp_boot::encode_extended_assignment(r(16)?,
                         CALLBACK_TABLE as i32,false)[0]);
    push(&mut out,encode_regbase_word_load(r(2)?,r(16)?,CALLBACK_TABLE)?);
    push(&mut out,encode_r2_new_zero_branch(PHYSICAL_ADDRESS+0x14,
                                            PHYSICAL_ADDRESS+0x3c)?);
    // Third packet: another forced GP extender+load followed by a .new check.
    for (i,word) in cdsp_boot::encode_extended_gp_word_load(r(2)?,CALLBACK_REGISTRY).into_iter().enumerate(){
        push(&mut out,if i==1{PacketMarker::ContinuationA.apply(word)}else{word});
    }
    push(&mut out,encode_r2_new_zero_branch(PHYSICAL_ADDRESS+0x20,
                                            PHYSICAL_ADDRESS+0x34)?);
    // Invoke callback with R1:0=(4,R16), if present.
    push(&mut out,encode_pair_combine_literal(r(0)?,r(16)?,4)?);
    push(&mut out,encode_callr(r(2)?));
    // Clear callback slot and restore paired frame state using original duplex.
    push(&mut out,PacketMarker::ContinuationA.apply(
        ScalarPacket::AssignSmallImmediate{dst:r(2)?,value:0}.encode()?));
    push(&mut out,encode_r2_new_store(r(16)?,0)?);
    push(&mut out,encode_pair_frame_io(false,r(16)?,r(29)?,0)?);
    if out.len()!=EXECUTABLE_LEN {
        return Err(format!("CDSP registry cleanup length {} instead of 64",out.len()));
    }
    Ok(out)
}

pub fn verify_runtime_registry_cleanup(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let generated=build_runtime_registry_cleanup()?;
    if reference.get(FILE_OFFSET..END_FILE_OFFSET)!=Some(generated.as_slice()){
        return Err("CDSP RWE callback-registry cleanup differs from pinned reference".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn guard_predicates_and_packet_relative_offsets(){
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(encode_p0_not_new_r2_zero(0x96cc81ac,0x96cc81dc,r(2)).unwrap(),0x1042_4018);
        assert_eq!(encode_r2_new_zero_branch(0x96cc81b4,0x96cc81dc).unwrap(),0x2402_c014);
        assert_eq!(encode_r2_new_zero_branch(0x96cc81c0,0x96cc81d4).unwrap(),0x2402_c00a);
        assert!(encode_r2_new_zero_branch(0x96cc81c0,0x96cc81a0).is_err());
        assert!(encode_p0_not_new_r2_zero(0x96cc81ac,0x96cc81dc,r(3)).is_err());
        assert_eq!(encode_pair_combine_literal(r(0),r(16),4).unwrap(),0x7330_6080);
        assert!(encode_pair_combine_literal(r(2),r(16),4).is_err());
        assert_eq!(encode_r2_new_store(r(16),0).unwrap(),0xa1b0_d200);
    }
    #[test]
    fn registry_16_words_preserve_instruction_and_packet_markers(){
        let bytes=build_runtime_registry_cleanup().unwrap();
        assert_eq!(bytes.len(),64);
        let w=|n:usize|u32::from_le_bytes(bytes[n*4..(n+1)*4].try_into().unwrap());
        assert_eq!(w(0),0x0c03_58f8);
        assert_eq!(w(1),0x4980_4002);
        assert_eq!(w(5),0x0c02_6300);
        assert_eq!(w(6),0x9b90_5902);
        assert_eq!(w(8),0x0c02_6300);
        assert_eq!(w(9),0x4980_4282);
        assert_eq!(w(14),0xa1b0_d200);
        assert_eq!(w(15),0x3e04_1f40);
        assert_eq!(FILE_OFFSET+bytes.len(),0x561e0);
    }
}
