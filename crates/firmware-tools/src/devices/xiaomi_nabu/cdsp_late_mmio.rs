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
pub const CONFIG_MODE_100_OFFSET:usize=0x550a8;
pub const CONFIG_MODE_200_OFFSET:usize=0x55158;
pub const CONFIG_DEFAULT_OFFSET:usize=0x55208;
pub const CONFIG_COMMON_EXIT_OFFSET:usize=0x552b4;
pub const COMPLETE_CONFIG_PREFIX_LEN:usize=CONFIG_COMMON_EXIT_OFFSET-FILE_OFFSET;
pub const SHARED_MMIO_COMMIT_OFFSET:usize=CONFIG_COMMON_EXIT_OFFSET;
pub const SHARED_CLEAR_FLAGS_END:usize=0x5532c;
pub const FLAG_CLEAR_PREFIX_LEN:usize=SHARED_CLEAR_FLAGS_END-FILE_OFFSET;
pub const ENABLE_AND_WAIT_END:usize=0x5539c;
pub const ENABLE_AND_WAIT_LEN:usize=ENABLE_AND_WAIT_END-FILE_OFFSET;

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

/// Three mutually exclusive codec clock/control configuration paths.
/// The original code uses R14/R15/R16 as three control parameters, then
/// writes nine 32-bit MMIO slots and executes a memory barrier. The values
/// below are named hardware settings, not original instruction words.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ControllerMode {Feature100,Feature200,Neither}

/// One 176-byte (or final 172-byte) mode-config path, assembled from
/// structured MMIO value pairs. Source operands, extenders, parser markers,
/// and two jump displacements are reconstructed with architecture encoders.
pub fn build_controller_mode_program(mode:ControllerMode)->Result<Vec<u8>,String>{
    let r0=GeneralRegister::new(0)?;
    let mut out=Vec::with_capacity(176);
    let mode_code=if mode==ControllerMode::Feature100 {0x31} else {0x39};
    for (reg,value) in [(14,mode_code),(15,0x44),(16,1)]{
        for word in cdsp_boot::encode_extended_assignment(GeneralRegister::new(reg)?,value,true){push_word(&mut out,word);}
    }
    let option_228=if mode==ControllerMode::Feature100 {2} else {0};
    const DATA_BASE:u32=0x0830_0200;
    let settings=[
        (0x18,0x2048_5699), (0x1c,0x2267), (0x20,0x24),
        (0x0c,0), (0x10,0x805), (0x14,0xd0),
        (0x24,0), (0x28,option_228), (0x2c,0x20),
    ];
    for (offset,value) in settings{
        let register_address=DATA_BASE+offset;
        for word in cdsp_boot::encode_extended_assignment(r0,register_address as i32,true){push_word(&mut out,word);}
        for word in encode_extended_word_store_immediate(r0,0,value)?{push_word(&mut out,word);}
    }
    // Barrier is a checked documented Hexagon operation, LLVM6 assembled.
    push_word(&mut out,0xa800_c000);
    match mode {
        ControllerMode::Feature100 =>push_word(&mut out,ScalarPacket::JumpRelative{displacement_bytes:0x160}.encode()?),
        ControllerMode::Feature200 =>push_word(&mut out,ScalarPacket::JumpRelative{displacement_bytes:0xb0}.encode()?),
        ControllerMode::Neither=>{}, // falls through to 0x552b4
    }
    let wanted=if mode==ControllerMode::Neither {0xac}else{0xb0};
    if out.len()!=wanted {return Err("CDSP controller configuration branch length changed".into());}
    Ok(out)
}

pub fn build_all_controller_mode_programs()->Result<Vec<u8>,String>{
    let mut out=build_joined_with_mode_dispatch()?;
    out.extend(build_controller_mode_program(ControllerMode::Feature100)?);
    out.extend(build_controller_mode_program(ControllerMode::Feature200)?);
    out.extend(build_controller_mode_program(ControllerMode::Neither)?);
    if out.len()!=COMPLETE_CONFIG_PREFIX_LEN{
        return Err("CDSP all three controller paths have inconsistent coverage".into());
    }
    Ok(out)
}

pub fn verify_all_controller_mode_programs(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let generated=build_all_controller_mode_programs()?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+generated.len())!=Some(generated.as_slice()){
        return Err("CDSP controller MMIO configuration cases mismatch pinned original".into());
    }
    Ok(())
}

/// Hexagon scalar word store of a general register, restricted to exact
/// word-aligned zero offset as verified by LLVM6. Unlike an immediate-store
/// packet, this fits in one instruction word.
pub fn encode_word_store_register(base:GeneralRegister,src:GeneralRegister)->u32{
    0xa180_c000|(u32::from(base.number())<<16)|(u32::from(src.number())<<8)
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum RegisterLogic { And, Or }

/// Ordinary Hexagon three-register AND/OR operations. Field positions were
/// confirmed using independent LLVM6 test inputs with different registers.
pub fn encode_register_logic(op:RegisterLogic,dst:GeneralRegister,
                             left:GeneralRegister,right:GeneralRegister)->u32{
    (if op==RegisterLogic::And {0xf100_c000}else{0xf120_c000})
        |(u32::from(left.number())<<16)
        |(u32::from(right.number())<<8)
        |u32::from(dst.number())
}

/// Set one 16-bit half of a register to a verified unsigned short constant.
/// Used for the CDSP hardware bitmask register R5; not a generic V66 vpair.
pub fn encode_halfword_literal(reg:GeneralRegister,high:bool,value:u8)->u32{
    (if high {0x7220_c000}else{0x7120_c000})
       |(u32::from(reg.number())<<16)|u32::from(value)
}

/// Hexagon `Rdst=sub(#-1,Rsrc)` short (non-extended) form. The instruction
/// is placed as a packet continuation before a memw load. Verified by LLVM6
/// with distinct source/destination registers and packet-marker manipulation.
pub fn encode_not_via_sub_neg1(dst:GeneralRegister,src:GeneralRegister)->u32{
    0x7660_ffe0|(u32::from(src.number())<<16)|u32::from(dst.number())
}

/// Commit selected clock parameters R14/R15 to MMIO registers 0x8300204/208,
/// and clear any previous mode-selection bits in registers 0x8300200/238.
/// The two bit-clears are identical subroutines with different mask values.
pub fn build_commit_and_clear_mode_flags()->Result<Vec<u8>,String>{
    let r=|n|GeneralRegister::new(n);
    let mut out=Vec::with_capacity(0x78);
    for (address,source) in [(0x0830_0204,14),(0x0830_0208,15)]{
        let words=cdsp_boot::encode_extended_assignment(r(0)?,address as i32,true);
        // The first MMIO base assignment shares a three-word packet with
        // `R1=R14`, whereas the next base assignment terminates its packet.
        push_word(&mut out,words[0]);
        push_word(&mut out,if address==0x0830_0204 {
            PacketMarker::ContinuationA.apply(words[1])
        }else{words[1]});
        push_word(&mut out,ScalarPacket::AssignRegister{dst:r(1)?,src:r(source)?}.encode()?);
        push_word(&mut out,encode_word_store_register(r(0)?,r(1)?));
        push_word(&mut out,0xa800_c000); // hardware `barrier`
    }
    for (address,bitmask) in [(0x0830_0200,1),(0x0830_0238,7)]{
        for word in cdsp_boot::encode_extended_assignment(r(0)?,address as i32,true){push_word(&mut out,word);}
        push_word(&mut out,PacketMarker::ContinuationA.apply(
            ScalarPacket::AssignRegister{dst:r(7)?,src:r(0)?}.encode()?));
        push_word(&mut out,encode_halfword_literal(r(5)?,true,0));
        push_word(&mut out,encode_halfword_literal(r(5)?,false,bitmask));
        push_word(&mut out,PacketMarker::ContinuationA.apply(encode_not_via_sub_neg1(r(5)?,r(5)?)));
        push_word(&mut out,encode_word_load_immediate(r(4)?,r(0)?,0)?);
        push_word(&mut out,encode_register_logic(RegisterLogic::And,r(4)?,r(4)?,r(5)?));
        push_word(&mut out,encode_word_store_register(r(0)?,r(4)?));
        push_word(&mut out,0xa800_c000);
    }
    if out.len()!=0x78{return Err("CDSP MMIO commit/clear code has unexpected length".into());}
    Ok(out)
}

pub fn build_controller_through_flag_clear()->Result<Vec<u8>,String>{
    let mut out=build_all_controller_mode_programs()?;
    out.extend(build_commit_and_clear_mode_flags()?);
    if out.len()!=FLAG_CLEAR_PREFIX_LEN{return Err("CDSP MMIO flag-clear prefix length mismatch".into());}
    Ok(out)
}

pub fn verify_controller_through_flag_clear(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let out=build_controller_through_flag_clear()?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+out.len())!=Some(out.as_slice()){
        return Err("CDSP MMIO commit/clear prefix differs from reference".into());
    }
    Ok(())
}

/// A checked short Hexagon PC-relative link: `R13=add(PC,#8)`.
/// The exact scalar form is independently LLVM6-assembled. It is deliberately
/// restricted to this proven operand/offset pair, rather than accepting an
/// arbitrary PC-relative literal encoding without ISA evidence.
pub fn encode_pic_link(dst:GeneralRegister,forward_bytes:u8)->Result<u32,String>{
    if dst.number()!=13 || forward_bytes!=8 {
        return Err("Hexagon PIC link verified only for R13=PC+8".into());
    }
    Ok(0x6a49_c40d)
}

/// Set bit 2 in controller register 0x8300200 after a cache invalidate,
/// update another hardware status register, then branch to the shared
/// counter-wait code at physical 0x96cc7038. This is original control-flow
/// reconstruction, not a source-code substitute for hardware validation.
pub fn build_enable_feature_and_spin()->Result<Vec<u8>,String>{
    let r=|n|GeneralRegister::new(n);
    let mut out=Vec::with_capacity(0x70);
    for word in cdsp_boot::encode_extended_assignment(r(0)?,0x0830_0200,true){push_word(&mut out,word);}
    push_word(&mut out,PacketMarker::ContinuationA.apply(
        ScalarPacket::AssignRegister{dst:r(7)?,src:r(0)?}.encode()?));
    push_word(&mut out,encode_halfword_literal(r(5)?,true,0));
    push_word(&mut out,PacketMarker::ContinuationA.apply(encode_halfword_literal(r(5)?,false,4)));
    // V66 `dcinva(R7)`; narrow checked A32-like source constructor is
    // supported here after independent LLVM6 assembly and Rizin decode.
    push_word(&mut out,0xa020_c000|(u32::from(r(7)?.number())<<16));
    push_word(&mut out,PacketMarker::ContinuationA.apply(encode_not_via_sub_neg1(r(6)?,r(5)?)));
    push_word(&mut out,encode_word_load_immediate(r(4)?,r(0)?,0)?);
    push_word(&mut out,encode_register_logic(RegisterLogic::And,r(4)?,r(4)?,r(6)?));
    push_word(&mut out,encode_register_logic(RegisterLogic::Or,r(4)?,r(4)?,r(5)?));
    push_word(&mut out,encode_word_store_register(r(0)?,r(4)?));
    push_word(&mut out,0xa800_c000);
    for word in cdsp_boot::encode_extended_assignment(r(0)?,0x0830_0238,true){push_word(&mut out,word);}
    push_word(&mut out,encode_word_load_immediate(r(1)?,r(0)?,0)?);
    for word in cdsp_boot::encode_extended_assignment(r(2)?,1,true){push_word(&mut out,word);}
    push_word(&mut out,encode_register_logic(RegisterLogic::Or,r(1)?,r(1)?,r(2)?));
    push_word(&mut out,encode_word_store_register(r(0)?,r(1)?));
    push_word(&mut out,0xa800_c000);
    for (destination,value) in [(4,0x0830_0200_i32),(2,i32::MIN),(0,0xc0)]{
        for word in cdsp_boot::encode_extended_assignment(r(destination)?,value,true){push_word(&mut out,word);}
    }
    push_word(&mut out,encode_pic_link(r(13)?,8)?);
    push_word(&mut out,ScalarPacket::JumpBackward{displacement_bytes:-0x360}.encode()?);
    if out.len()!=0x70{return Err("CDSP feature-enable and spin loop must have 112 bytes".into());}
    Ok(out)
}

pub fn build_controller_through_wait_handoff()->Result<Vec<u8>,String>{
    let mut out=build_controller_through_flag_clear()?;
    out.extend(build_enable_feature_and_spin()?);
    if out.len()!=ENABLE_AND_WAIT_LEN{return Err("CDSP controller wait handoff length mismatch".into());}
    Ok(out)
}

pub fn verify_controller_through_wait_handoff(reference:&[u8])->Result<(),String>{
    cdsp_boot::validate_reference(reference)?;
    let out=build_controller_through_wait_handoff()?;
    if reference.get(FILE_OFFSET..FILE_OFFSET+out.len())!=Some(out.as_slice()){
        return Err("CDSP PIC wait/control source differs from pinned original".into());
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
    fn enabled_controller_feature_returns_through_pic_wait(){
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(encode_pic_link(r(13),8).unwrap(),0x6a49_c40d);
        assert!(encode_pic_link(r(0),8).is_err());
        assert!(encode_pic_link(r(13),12).is_err());
        let blob=build_controller_through_wait_handoff().unwrap();
        assert_eq!(blob.len(),ENABLE_AND_WAIT_LEN);
        assert_eq!(FILE_OFFSET+blob.len(),ENABLE_AND_WAIT_END);
        let read=|offset:usize|u32::from_le_bytes(blob[offset-FILE_OFFSET..offset-FILE_OFFSET+4].try_into().unwrap());
        assert_eq!(read(0x55340),0xa027_c000); // dcinva(R7)
        assert_eq!(read(0x55394),0x6a49_c40d); // R13=add(PC,#8)
        assert_eq!(read(0x55398),0x59ff_fe50); // jump to physical 0x96cc7038
    }
    #[test]
    fn reset_and_clear_mmio_source_encoding_is_bounded(){
        let r=|n|GeneralRegister::new(n).unwrap();
        assert_eq!(encode_word_store_register(r(0),r(1)),0xa180_c100);
        assert_eq!(encode_word_store_register(r(0),r(4)),0xa180_c400);
        assert_eq!(encode_register_logic(RegisterLogic::And,r(4),r(4),r(5)),0xf104_c504);
        assert_eq!(encode_register_logic(RegisterLogic::Or,r(4),r(4),r(5)),0xf124_c504);
        assert_eq!(encode_halfword_literal(r(5),true,0),0x7225_c000);
        assert_eq!(encode_halfword_literal(r(5),false,7),0x7125_c007);
        assert_eq!(encode_not_via_sub_neg1(r(5),r(5)),0x7665_ffe5);
        let bytes=build_commit_and_clear_mode_flags().unwrap();
        assert_eq!(bytes.len(),0x78);
        assert_eq!(build_controller_through_flag_clear().unwrap().len(),FLAG_CLEAR_PREFIX_LEN);
        assert_eq!(FILE_OFFSET+FLAG_CLEAR_PREFIX_LEN,SHARED_CLEAR_FLAGS_END);
    }
    #[test]
    fn all_hardware_mode_programs_are_exactly_partitioned(){
        assert_eq!(build_controller_mode_program(ControllerMode::Feature100).unwrap().len(),0xb0);
        assert_eq!(build_controller_mode_program(ControllerMode::Feature200).unwrap().len(),0xb0);
        assert_eq!(build_controller_mode_program(ControllerMode::Neither).unwrap().len(),0xac);
        let joined=build_all_controller_mode_programs().unwrap();
        assert_eq!(joined.len(),COMPLETE_CONFIG_PREFIX_LEN);
        assert_eq!(FILE_OFFSET+joined.len(),CONFIG_COMMON_EXIT_OFFSET);
        let get_word=|f:usize|u32::from_le_bytes(joined[f-FILE_OFFSET..f-FILE_OFFSET+4].try_into().unwrap());
        assert_eq!(get_word(0x55154),0x5800_c0b0); // first shared epilogue jump
        assert_eq!(get_word(0x55204),0x5800_c058); // second shared epilogue jump
        assert_eq!(get_word(0x552b0),0xa800_c000); // default falls through
        assert_eq!(get_word(0x5513c),0x3c40_c002); // feature100 register 0x228=2
        assert_eq!(get_word(0x551ec),0x3c40_c000); // feature200 register 0x228=0
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
