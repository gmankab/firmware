// SPDX-License-Identifier: GPL-3.0-only
//! CDSP later `R E` code segment: initialization of a three-word controller
//! MMIO register bank. This is a Hexagon V66 **code** region, unlike the
//! preceding `R E` ELF segment that contains mostly data and structural fill.
//!
//! Address spaces are distinct: CDSP MBN file [0x55000,0x55038), ELF VMA
//! [0xc00c7000,0xc00c7038), physical [0x96cc7000,0x96cc7038).
//! The direct jump to physical 0x96cc7070 is external to this short fragment.
//! No original firmware opcode sequences are embedded in this source.
use super::cdsp_boot::{self,GeneralRegister,ScalarPacket,PacketMarker};

pub const FILE_OFFSET:usize=0x55000;
pub const ELF_VADDR:u32=0xc00c7000;
pub const PHYSICAL_ADDRESS:u32=0x96cc7000;
pub const FIRST_ENTRY_LEN:usize=0x38;
pub const WAIT_FILE_OFFSET:usize=0x55038;
pub const WAIT_LEN:usize=0x38;
pub const JOINED_LEN:usize=FIRST_ENTRY_LEN+WAIT_LEN;
pub const DETECT_OFFSET:usize=0x55070;
pub const DETECT_LEN:usize=0x38;
pub const THREE_STAGE_LEN:usize=JOINED_LEN+DETECT_LEN;

/// Checked Hexagon `memw(Rbase + #offset) = ##value` with an immediate extender.
/// Only the small verified 32-bit, word-aligned addressing form is admitted;
/// it is independently field-tested with LLVM6 and decoded with Rizin V66.
/// The two words constitute one packet, extender followed by the consuming
/// immediate store. No generic memory-store support is implied.
pub fn encode_extended_word_store_immediate(base:GeneralRegister,
                                            offset_bytes:u8,value:u32)->Result<[u32;2],String>{
    if offset_bytes%4!=0 || offset_bytes>12 {
        return Err("Hexagon immediate word store: offset must be 0,4,8,12".into());
    }
    let extender=((value>>20)<<16)|((value>>6)&0x3fff)|0x4000;
    let store=0x3c40_c000 | (u32::from(base.number())<<16)
        | (u32::from(offset_bytes)<<5) | (value&0x3f);
    Ok([extender,store])
}

fn push_word(out:&mut Vec<u8>,value:u32){out.extend_from_slice(&value.to_le_bytes());}

/// Three MMIO control writes: addresses 0x83a0040, +4 and +8 each receive 7.
/// A Hexagon barrier orders the writes before control transfers to the shared
/// state routine at physical address 0x96cc7070. The emitter reads *no*
/// private reference bytes; the OEM firmware is solely an external verifier.
pub fn build_mmio_setup()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(FIRST_ENTRY_LEN);
    let r0=GeneralRegister::new(0)?;
    for register_address in [0x083a_0040u32,0x083a_0044,0x083a_0048] {
        for word in cdsp_boot::encode_extended_assignment(r0,register_address as i32,true){
            push_word(&mut out,word);
        }
        for word in encode_extended_word_store_immediate(r0,0,7)? {
            push_word(&mut out,word);
        }
    }
    // Hexagon `barrier`, independently encoded by LLVM6 and decoded by Rizin.
    push_word(&mut out,0xa800_c000);
    // Hexagon `jump +0x3c`: from physical 0x96cc7034 to 0x96cc7070.
    push_word(&mut out,PacketMarker::Final.apply(
        ScalarPacket::JumpRelative{displacement_bytes:0x3c}.encode()?));
    if out.len()!=FIRST_ENTRY_LEN{return Err("CDSP later MMIO setup length != 56".into());}
    Ok(out)
}

/// Checked Hexagon `Rdest = memw(Rbase + #word_offset)` scalar. LLVM6
/// independently established base and destination fields for the immediate
/// zero-offset form; the narrow byte-offset range prevents field overlap.
pub fn encode_word_load_immediate(dst:GeneralRegister,base:GeneralRegister,
                                  offset_bytes:u8)->Result<u32,String>{
    if offset_bytes%4!=0 || offset_bytes>12{
        return Err("Hexagon word-load offset must be aligned <= 12".into());
    }
    Ok(0x9180_c000 | (u32::from(base.number())<<16)
       | (u32::from(offset_bytes)<<5) | u32::from(dst.number()))
}

/// Single packed packet: `P0=cmp.gtu(Rleft,Rright)` followed by
/// `if (!P0.new) jump:t PC+rel`; only the independently evidenced negative
/// short-backedge form is implemented. Source and destination fields were
/// varied through LLVM6 fixtures; no OEM opcode is copied as input.
pub fn encode_wait_if_unsigned_delta_small(left:GeneralRegister,
                                            right:GeneralRegister,
                                            displacement_bytes:i16)->Result<u32,String>{
    if displacement_bytes>=0 || displacement_bytes < -32 || displacement_bytes%2!=0{
        return Err("Hexagon counter-spin branch must be negative, even and <=32 bytes".into());
    }
    Ok(0x1570_e000 | (u32::from(left.number())<<16)
        | (u32::from(right.number())<<8)
        | (((i32::from(displacement_bytes)/2) as u32)&0xff))
}

/// Shared monotonic counter wait: save three argument GPRs, snapshot an MMIO
/// counter, reread it until `R3-R1 > R2`, restore arguments and jump to R13.
/// The first two assignments of each three-instruction Hexagon packet have
/// explicit continuation parse markers; the third closes the packet.
pub fn build_shared_wait()->Result<Vec<u8>,String>{
    let r=|n|GeneralRegister::new(n);
    let mut out=Vec::with_capacity(WAIT_LEN);
    for (dst,src,last) in [(8,1,false),(9,2,false),(10,3,true)]{
        let instr=ScalarPacket::AssignRegister{dst:r(dst)?,src:r(src)?}.encode()?;
        push_word(&mut out, (if last {PacketMarker::Final}else{PacketMarker::ContinuationA}).apply(instr));
    }
    push_word(&mut out,ScalarPacket::AssignRegister{dst:r(2)?,src:r(0)?}.encode()?);
    for word in cdsp_boot::encode_extended_assignment(r(0)?,0x083a_2000,true){push_word(&mut out,word);}
    push_word(&mut out,encode_word_load_immediate(r(1)?,r(0)?,0)?);
    push_word(&mut out,encode_word_load_immediate(r(3)?,r(0)?,0)?);
    push_word(&mut out,ScalarPacket::SubtractRegisters{dst:r(3)?,left:r(3)?,right:r(1)?}.encode()?);
    // Branch PC at 0x96cc705c to the second MMIO load at 0x96cc7054.
    push_word(&mut out,encode_wait_if_unsigned_delta_small(r(3)?,r(2)?,-8)?);
    for (dst,src,last) in [(1,8,false),(2,9,false),(3,10,true)]{
        let instr=ScalarPacket::AssignRegister{dst:r(dst)?,src:r(src)?}.encode()?;
        push_word(&mut out,(if last{PacketMarker::Final}else{PacketMarker::ContinuationA}).apply(instr));
    }
    push_word(&mut out,ScalarPacket::JumpRegister{source:r(13)?}.encode()?);
    if out.len()!=WAIT_LEN{return Err("Hexagon shared wait length !=56".into());}
    Ok(out)
}

pub fn build_joined_initial_controller()->Result<Vec<u8>,String>{
    let mut out=build_mmio_setup()?;
    out.extend(build_shared_wait()?);
    if out.len()!=JOINED_LEN{return Err("Hexagon joined CDSP code length mismatch".into());}
    Ok(out)
}

pub fn verify_joined_controller(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let actual=build_joined_initial_controller()?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+JOINED_LEN)!=Some(actual.as_slice()){
        return Err("CDSP later controller entry+wait does not match OEM reference".into());
    }
    Ok(())
}

/// Hexagon two-word `Rd = and(Rsrc,##value)` with the immediate extender.
/// Validated against multiple LLVM6 R2/R1 masks and Rizin V66 decoding.
pub fn encode_extended_and(dst:GeneralRegister,src:GeneralRegister,
                           value:u32)->[u32;2]{
    let ext=((value>>20)<<16)|((value>>6)&0x3fff)|0x4000;
    let op=0x7600_c000 | (u32::from(src.number())<<16)
        | ((value&0x3f)<<5) | u32::from(dst.number());
    [ext,op]
}

/// Hexagon triple-word packet: `P0=cmp.eq(Rsrc,##value); if(P0.new) jump:t`.
/// No branch to arbitrary addresses is implied: caller supplies one short,
/// strictly positive, halfword-aligned packet-relative destination.
pub fn encode_extended_new_p0_eq_branch(src:GeneralRegister,value:u32,
                                       forward_bytes:u16)->Result<[u32;3],String>{
    if forward_bytes==0 || forward_bytes>0x1fe || forward_bytes%2!=0{
        return Err("Hexagon new-predicate branch requires a short positive even offset".into());
    }
    let ext=((value>>20)<<16)|((value>>6)&0x3fff)|0x4000;
    let test=0x7500_4000 | (u32::from(src.number())<<16) | ((value&0x3f)<<5);
    let branch=0x5c00_d800 | u32::from(forward_bytes/2);
    Ok([ext,test,branch])
}

/// Read the controller configuration word at 0x1fc8000 and dispatch to one
/// of the two observed MMIO programming cases depending on the 0x100/0x200
/// feature bits. Later case bodies begin at 0x550a8 and follow separately.
pub fn build_detect_controller_mode()->Result<Vec<u8>,String>{
    let mut out=Vec::with_capacity(DETECT_LEN);
    let r=|n|GeneralRegister::new(n);
    for word in cdsp_boot::encode_extended_assignment(r(0)?,0x01fc_8000,true){push_word(&mut out,word);}
    push_word(&mut out,encode_word_load_immediate(r(1)?,r(0)?,0)?);
    for (mask,branch_distance) in [(0x100,0x24),(0x200,0xc0)] {
        for word in encode_extended_and(r(2)?,r(1)?,mask){push_word(&mut out,word);}
        for word in encode_extended_new_p0_eq_branch(r(2)?,mask,branch_distance)?{push_word(&mut out,word);}
    }
    // From physical 0x96cc70a4 to the default case at 0x96cc7208.
    push_word(&mut out,ScalarPacket::JumpRelative{displacement_bytes:0x164}.encode()?);
    if out.len()!=DETECT_LEN{return Err("CDSP detect-mode prefix length !=56".into());}
    Ok(out)
}

pub fn build_joined_with_mode_dispatch()->Result<Vec<u8>,String>{
    let mut out=build_joined_initial_controller()?;
    out.extend(build_detect_controller_mode()?);
    if out.len()!=THREE_STAGE_LEN{return Err("Hexagon CDSP mode dispatch span length mismatch".into());}
    Ok(out)
}

pub fn verify_joined_with_mode_dispatch(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let actual=build_joined_with_mode_dispatch()?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+THREE_STAGE_LEN)!=Some(actual.as_slice()){
        return Err("CDSP controller mode detection differs from original".into());
    }
    Ok(())
}

pub fn verify_mmio_setup(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let actual=build_mmio_setup()?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+FIRST_ENTRY_LEN)!=Some(actual.as_slice()){
        return Err("CDSP later MMIO setup differs from pinned reference".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mmio_store_immediate_fields_are_checked(){
        let r0=GeneralRegister::new(0).unwrap();
        let r1=GeneralRegister::new(1).unwrap();
        assert_eq!(encode_extended_word_store_immediate(r0,0,7).unwrap(),[0x0000_4000,0x3c40_c007]);
        assert_eq!(encode_extended_word_store_immediate(r1,4,11).unwrap(),[0x0000_4000,0x3c41_c08b]);
        assert_eq!(encode_extended_word_store_immediate(r0,0,0x1234).unwrap(),[0x0000_4048,0x3c40_c034]);
        assert!(encode_extended_word_store_immediate(r0,2,7).is_err());
        assert!(encode_extended_word_store_immediate(r0,16,7).is_err());
    }
    #[test]
    fn controller_mode_detection_packet_operands_are_validated(){
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(encode_extended_and(r(2),r(1),0x100),[0x0000_4004,0x7601_c002]);
        assert_eq!(encode_extended_and(r(2),r(1),0x200),[0x0000_4008,0x7601_c002]);
        assert_eq!(encode_extended_new_p0_eq_branch(r(2),0x100,0x24).unwrap(),
                   [0x0000_4004,0x7502_4000,0x5c00_d812]);
        assert_eq!(encode_extended_new_p0_eq_branch(r(2),0x200,0xc0).unwrap(),
                   [0x0000_4008,0x7502_4000,0x5c00_d860]);
        assert!(encode_extended_new_p0_eq_branch(r(2),0x100,0).is_err());
        assert!(encode_extended_new_p0_eq_branch(r(2),0x100,3).is_err());
        let combined=build_joined_with_mode_dispatch().unwrap();
        assert_eq!(combined.len(),THREE_STAGE_LEN);
        assert_eq!(combined[0x70..0x74],[0x00,0x72,0x1f,0x00]);
        assert_eq!(combined[0xa4..0xa8],[0xb2,0xc0,0x00,0x58]);
        assert_eq!(PHYSICAL_ADDRESS+THREE_STAGE_LEN as u32,0x96cc70a8);
    }
    #[test]
    fn late_counter_wait_encoding_matches_llvm_checked_fields(){
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(encode_word_load_immediate(r(1),r(0),0).unwrap(),0x9180_c001);
        assert_eq!(encode_word_load_immediate(r(3),r(0),0).unwrap(),0x9180_c003);
        assert_eq!(encode_wait_if_unsigned_delta_small(r(3),r(2),-8).unwrap(),0x1573_e2fc);
        assert!(encode_wait_if_unsigned_delta_small(r(3),r(2),0).is_err());
        assert!(encode_wait_if_unsigned_delta_small(r(3),r(2),-33).is_err());
        let joined=build_joined_initial_controller().unwrap();
        assert_eq!(joined.len(),JOINED_LEN);
        assert_eq!(joined[0x38..0x3c],[0x08,0x40,0x61,0x70]);
        assert_eq!(joined[0x5c..0x60],[0xfc,0xe2,0x73,0x15]);
        assert_eq!(PHYSICAL_ADDRESS+JOINED_LEN as u32,0x96cc7070);
    }
    #[test]
    fn initial_mmio_setup_typed_packets_are_contiguous(){
        let bytes=build_mmio_setup().unwrap();
        assert_eq!(bytes.len(),FIRST_ENTRY_LEN);
        let words:Vec<_>=bytes.chunks_exact(4).map(|b|u32::from_le_bytes(b.try_into().unwrap())).collect();
        assert_eq!(words[0],cdsp_boot::encode_extended_assignment(GeneralRegister::new(0).unwrap(),0x083a0040,true)[0]);
        assert_eq!(words[12],0xa800_c000);
        assert_eq!(words[13],ScalarPacket::JumpRelative{displacement_bytes:0x3c}.encode().unwrap());
        assert_eq!(ELF_VADDR+FIRST_ENTRY_LEN as u32,0xc00c7038);
        assert_eq!(PHYSICAL_ADDRESS+FIRST_ENTRY_LEN as u32,0x96cc7038);
    }
}
