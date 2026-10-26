// SPDX-License-Identifier: GPL-3.0-only
//! Venus codec stream transaction setup and failure diagnostics. The function
//! continues after the 147-entry message-address table; this partial emitted
//! segment intentionally stops at its end and never inserts dummy opcodes.
//! The diagnostic pointers are identified as non-executable data, not a
//! substitute for independently reconstructed proprietary DSP instructions.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
const START:usize=0x39f68;
const END:usize=0x3a2e4;

/// Checked indexed A32 LDRH/STRH, with public ISA bitfields (register index).
fn encode_halfword_register(c:C,load:bool,dst:R,base:R,index:R,wb:bool)->Result<u32,String>{
    if [dst,base,index].contains(&R::PC)||(load && wb && dst==base){
        return Err("invalid indexed halfword transfer".into());
    }
    Ok(((c as u32)<<28)|0x0180_00b0|(if load{1<<20}else{0})
       |(if wb{1<<21}else{0})|(base.number()<<16)
       |(dst.number()<<12)|index.number())
}

/// Diagnostic text references, _not_ executable ARM instructions.
/// Each entry is an address in the original read-only firmware segment.
const STREAM_CONTROL_MESSAGES:[u32;147]=[
    0x000ee010,0x000ec0b3,0x000ec44a,0x000ec44f,0x000ec770,0x000ec465,0x000ec473,
    0x000ec47c,0x000ec482,0x000ec494,0x000ec4a7,0x000ec4b3,0x000ec4c0,0x000ec4cb,
    0x000ec4d8,0x000ec4e3,0x000ec4ef,0x000ec4f9,0x000ec504,0x000ec512,0x000ec523,
    0x000ec52f,0x000ec537,0x000ec53d,0x000ec548,0x000ec556,0x000ec563,0x000ec571,
    0x000ec57b,0x000ec58e,0x000ec59b,0x000ec5a9,0x000ec5b5,0x000ec5ce,0x000ec5e7,
    0x000ec607,0x000ec625,0x000ec643,0x000ec655,0x000ec66d,0x000ec680,0x000ec697,
    0x000ec6a4,0x000ec6ab,0x000ec6c2,0x000ec6cd,0x000ec6dd,0x000ec6e9,0x000ec6f7,
    0x000ec704,0x000ec710,0x000ec71a,0x000ec724,0x000ec73c,0x000ec748,0x000ec757,
    0x000ec761,0x000ec777,0x000ec781,0x000ec78c,0x000ec794,0x000ec7a1,0x000ec7a9,
    0x000ec7b1,0x000ec7bc,0x000ec7c9,0x000ec7d0,0x000ec7dc,0x000ec7f5,0x000ec813,
    0x000ec838,0x000ec857,0x000ec87d,0x000ec88d,0x000ec89c,0x000ec8a7,0x000ec8b8,
    0x000ec8c6,0x000ec8d8,0x000ec8e9,0x000ec8fd,0x000ec905,0x000ec91c,0x000ec934,
    0x000ec93f,0x000ec954,0x000ec968,0x000ec97e,0x000ec98a,0x000ec995,0x000ec9a3,
    0x000ec9ad,0x000ec9b7,0x000ec9c8,0x000ec9d9,0x000ec9e0,0x000ec9ec,0x000ec9f8,
    0x000eca0a,0x000eca17,0x000eca32,0x000eca50,0x000eca70,0x000eca8c,0x000ecaa0,
    0x000ecab0,0x000ecabf,0x000ecad3,0x000ecadf,0x000ecae8,0x000ecaf6,0x000ecafe,
    0x000ecbda,0x000ecb07,0x000ecb1e,0x000ecb34,0x000ecb45,0x000ecb57,0x000ecb5c,
    0x000ecb6c,0x000ecb77,0x000ecb8a,0x000ecb9d,0x000ecba9,0x000ecbb6,0x000ecbc8,
    0x000ece46,0x000ecbe6,0x000ecbf3,0x000ecbfd,0x000ecc0a,0x000ecc1b,0x000ecc2c,
    0x000ecc3d,0x000ecc4e,0x000ecc61,0x000ecc74,0x000ecc88,0x000ecc97,0x000ecca6,
    0x000eccb7,0x000ecccc,0x000eccdd,0x000eccf0,0x000eccff,0x000ecd0e,0x000ecd23,
];

/// Select the controller source for stream configuration, check format IDs,
/// and prepare temporary register accounting. The function's main body
/// resumes at address `0x3a2e4`, outside this first partial checkpoint.
fn initialize_stream_control(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x39f68: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x39f6c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x39f70: sub sp, sp, #780
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,780)?)?;
    // 0x39f74: sub sp, sp, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,5120)?)?;
    // 0x39f78: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x39f7c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x39f80: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x39f84: mov r0, #296
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,296)?)?;
    // 0x39f88: str r1, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-56)?)?;
    // 0x39f8c: str r1, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-60)?)?;
    // 0x39f90: str r1, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-64)?)?;
    // 0x39f94: orr r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // 0x39f98: ldr r7, [r6, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,2732)?)?;
    // 0x39f9c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x39fa0: beq 0x39fbc <entry+0x3ec>   @ imm = #0x14
    c.b(C::Eq,0x39fbc)?;
    // 0x39fa4: mov r1, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,138)?)?;
    // 0x39fa8: orr r1, r1, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    // 0x39fac: ldrh r1, [r6, r1]
    c.emit(encode_halfword_register(C::Al,true,R::R1,R::new(6)?,R::R1,false)?)?;
    // 0x39fb0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x39fb4: ldr r1, [r6, #0x304]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,772)?)?;
    // 0x39fb8: b 0x39ff8 <entry+0x428>   @ imm = #0x38
    c.b(C::Al,0x39ff8)?;
    // 0x39fbc: ldr r7, [r6, #0xd90]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,3472)?)?;
    // 0x39fc0: add r7, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,2)?)?;
    // 0x39fc4: and r7, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,7)?)?;
    // 0x39fc8: cmp r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,5)?)?;
    // 0x39fcc: ldrls r1, [pc, #0xc4]         @ 0x4c8 <entry+0x4c8>
    c.literal_cond(C::Ls,R::R1,0x3a098)?;
    // 0x39fd0: ldrls r1, [r1, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Ls,true,R::R1,R::R1,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x39fd4: str r1, [r6, #0xd90]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,3472)?)?;
    // 0x39fd8: mov r1, #78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,78)?)?;
    // 0x39fdc: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x39fe0: ldrh r1, [r6, r1]
    c.emit(encode_halfword_register(C::Al,true,R::R1,R::new(6)?,R::R1,false)?)?;
    // 0x39fe4: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x39fe8: ldr r1, [r6, #0x150]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,336)?)?;
    // 0x39fec: ldr r7, [r6, #0x154]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,340)?)?;
    // 0x39ff0: add r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x39ff4: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x39ff8: str r1, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-64)?)?;
    // 0x39ffc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3a000: add r0, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3a004: str r1, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-40)?)?;
    // 0x3a008: str r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-44)?)?;
    // 0x3a00c: ldr r1, [r6, #0xd30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,3376)?)?;
    // 0x3a010: str r1, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-52)?)?;
    // 0x3a014: ldr r1, [r6, #0xd34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,3380)?)?;
    // 0x3a018: str r1, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-56)?)?;
    // 0x3a01c: ldr r1, [r6, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,3372)?)?;
    // 0x3a020: str r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // 0x3a024: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x3a028: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x3a02c: bge 0x3a048 <entry+0x478>   @ imm = #0x14
    c.b(C::Ge,0x3a048)?;
    // 0x3a030: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x3a034: beq 0x3a080 <entry+0x4b0>   @ imm = #0x44
    c.b(C::Eq,0x3a080)?;
    // 0x3a038: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x3a03c: bne 0x3a068 <entry+0x498>   @ imm = #0x24
    c.b(C::Ne,0x3a068)?;
    // 0x3a040: mov r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // 0x3a044: b 0x3a058 <entry+0x488>   @ imm = #0xc
    c.b(C::Al,0x3a058)?;
    // 0x3a048: beq 0x3a2e4 <entry+0x714>   @ imm = #0x294
    c.b(C::Eq,0x3a2e4)?;
    // 0x3a04c: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // 0x3a050: bne 0x3a068 <entry+0x498>   @ imm = #0x10
    c.b(C::Ne,0x3a068)?;
    // 0x3a054: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x3a058: str r1, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-48)?)?;
    // 0x3a05c: sub r1, r11, #60
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,60)?)?;
    // 0x3a060: bl 0x39ef0 <entry+0x320>   @ imm = #-0x178
    c.bl(0x39ef0)?;
    // 0x3a064: b 0x3a2ec <entry+0x71c>   @ imm = #0x280
    c.b(C::Al,0x3a2ec)?;
    // 0x3a068: ldr r2, [pc, #0x2c]         @ 0x4cc <entry+0x4cc>
    c.literal_cond(C::Al,R::R2,0x3a09c)?;
    // 0x3a06c: mov r1, #424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,424)?)?;
    // 0x3a070: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3a074: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x3a078: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x388d0
    c.bl(0x17b0)?;
    // 0x3a07c: b 0x3a2ec <entry+0x71c>   @ imm = #0x268
    c.b(C::Al,0x3a2ec)?;
    // 0x3a080: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3a084: sub r2, r11, #56
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,56)?)?;
    // 0x3a088: str r1, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-48)?)?;
    // 0x3a08c: sub r1, r11, #52
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,52)?)?;
    // 0x3a090: bl 0x39ccc <entry+0xfc>    @ imm = #-0x3cc
    c.bl(0x39ccc)?;
    // 0x3a094: b 0x3a2ec <entry+0x71c>   @ imm = #0x250
    c.b(C::Al,0x3a2ec)?;
    c.check(0x3a098)?;
    for msg in STREAM_CONTROL_MESSAGES {c.emit(msg)?;}
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    initialize_stream_control(c)
}

#[cfg(test)]mod tests {
 use super::*;
 #[test]fn format_string_addresses_have_original_length(){
    assert_eq!(STREAM_CONTROL_MESSAGES.len(),147);
    assert_eq!(STREAM_CONTROL_MESSAGES[0],0x000e_e010);
    assert_eq!(STREAM_CONTROL_MESSAGES[146],0x000e_cd23);
    let mut c=Code::new(START);emit(&mut c).unwrap();
    assert_eq!(c.bytes.len(),END-START);
 }
 #[test]fn halfword_register_encoding_is_checked(){
    assert_eq!(encode_halfword_register(C::Al,true,R::R1,R::new(6).unwrap(),R::R1,false).unwrap(),0xe19610b1);
    assert!(encode_halfword_register(C::Al,true,R::PC,R::R1,R::R2,false).is_err());
 }
}
