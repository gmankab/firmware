// SPDX-License-Identifier: GPL-3.0-only
//! ARM32 packed input-bitstream helper and codec callback-register bootstrap.
//! Input bit width is adjusted using ARM register shifts and halfword offsets.
//! The five words following the callback epilogue are controller addresses,
//! not ARM instruction words.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2ea50;
const END:usize=0x2ebbc;
fn pack_bits_and_register_callbacks(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2ea50: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2ea54: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x2ea58: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x2ea5c: str r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,4)?)?;
    // 0x2ea60: ldrh r7, [r3]
    c.emit(arm32::load_halfword(C::Al,R::new(7)?,R::new(3)?,0)?)?;
    // 0x2ea64: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x2ea68: ldrh r12, [r2]
    c.emit(arm32::load_halfword(C::Al,R::R12,R::R2,0)?)?;
    // 0x2ea6c: ldr r2, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,12)?)?;
    // 0x2ea70: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x2ea74: blt 0x2eb18 <entry+0x1518>  @ imm = #0x9c
    c.b(C::Lt,0x2eb18)?;
    // 0x2ea78: ldr lr, [r11, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,16)?)?;
    // 0x2ea7c: ldr r4, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,8)?)?;
    // 0x2ea80: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x2ea84: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x2ea88: mov r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,32)?)?;
    // 0x2ea8c: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // 0x2ea90: and r10, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(10)?,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x2ea94: ldr r2, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R2,R::R1,4)?)?;
    // 0x2ea98: lsl r7, r7, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Lsl,16)?)?;
    // 0x2ea9c: sub r3, r9, r7, asr #16
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(9)?,R::new(7)?,Shift::Asr,16,false)?)?;
    // 0x2eaa0: cmp r3, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x2eaa4: and r6, r2, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R2,R::LR,Shift::Lsl,0,false)?)?;
    // 0x2eaa8: lsl r2, r12, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R12,Shift::Lsl,16)?)?;
    // 0x2eaac: asr r8, r2, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::R2,Shift::Asr,16)?)?;
    // 0x2eab0: ldr r2, [r0, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x2eab4: bge 0x2eae4 <entry+0x14e4>  @ imm = #0x28
    c.b(C::Ge,0x2eae4)?;
    // 0x2eab8: lsl r3, r3, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsl,16)?)?;
    // 0x2eabc: add r12, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R12,1)?)?;
    // 0x2eac0: sub r7, r4, r3, asr #16
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(4)?,R::new(3)?,Shift::Asr,16,false)?)?;
    // 0x2eac4: asr r3, r3, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Asr,16)?)?;
    // 0x2eac8: lsl r2, r2, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R2,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x2eacc: add r2, r2, r6, asr r7
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R2,R::R2,R::new(6)?,Shift::Asr,R::new(7)?,false)?)?;
    // 0x2ead0: str r2, [r0, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x2ead4: lsl r2, r12, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R12,Shift::Lsl,16)?)?;
    // 0x2ead8: asr r2, r2, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,16)?)?;
    // 0x2eadc: str r6, [r0, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x2eae0: b 0x2eb0c <entry+0x150c>  @ imm = #0x24
    c.b(C::Al,0x2eb0c)?;
    // 0x2eae4: add r2, r6, r2, lsl r4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,R::new(4)?,false)?)?;
    // 0x2eae8: asr r3, r7, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(7)?,Shift::Asr,16)?)?;
    // 0x2eaec: str r2, [r0, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x2eaf0: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x2eaf4: add r7, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x2eaf8: orr r2, r2, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65280)?)?;
    // 0x2eafc: and r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x2eb00: cmp r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // 0x2eb04: moveq r7, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2eb08: addeq r12, r12, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R12,R::R12,1)?)?;
    // 0x2eb0c: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x2eb10: cmp r5, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x2eb14: bne 0x2ea94 <entry+0x1494>  @ imm = #-0x88
    c.b(C::Ne,0x2ea94)?;
    // 0x2eb18: ldr r0, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // 0x2eb1c: strh r12, [r0]
    c.emit(arm32::store_halfword(C::Al,R::R12,R::R0,0)?)?;
    // 0x2eb20: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x2eb24: strh r7, [r0]
    c.emit(arm32::store_halfword(C::Al,R::new(7)?,R::R0,0)?)?;
    // 0x2eb28: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x2eb2c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2eb30: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2eb34: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2eb38: ldr r0, [pc, #0x68]         @ 0x15a8 <entry+0x15a8>
    c.literal(R::R0,0x2eba8)?;
    // 0x2eb3c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2eb40: ldr r4, [pc, #0x68]         @ 0x15b0 <entry+0x15b0>
    c.literal(R::new(4)?,0x2ebb0)?;
    // 0x2eb44: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2eb48: ldr r0, [pc, #0x5c]         @ 0x15ac <entry+0x15ac>
    c.literal(R::R0,0x2ebac)?;
    // 0x2eb4c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2eb50: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x2eb54: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2eb58: bl 0x2b8c <entry+0xfffffffffffd558c> @ imm = #-0x2bfd4
    c.bl(0x2b8c)?;
    // 0x2eb5c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2eb60: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2eb64: bne 0x2eb78 <entry+0x1578>  @ imm = #0xc
    c.b(C::Ne,0x2eb78)?;
    // 0x2eb68: ldr r2, [pc, #0x44]         @ 0x15b4 <entry+0x15b4>
    c.literal(R::R2,0x2ebb4)?;
    // 0x2eb6c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2eb70: mov r1, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,132)?)?;
    // 0x2eb74: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2d3cc
    c.bl(0x17b0)?;
    // 0x2eb78: ldr r2, [pc, #0x38]         @ 0x15b8 <entry+0x15b8>
    c.literal(R::R2,0x2ebb8)?;
    // 0x2eb7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2eb80: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x2eb84: bl 0x2f45c <entry+0x1e5c>  @ imm = #0x8d0
    c.bl(0x2f45c)?;
    // 0x2eb88: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2eb8c: bne 0x2eba0 <entry+0x15a0>  @ imm = #0xc
    c.b(C::Ne,0x2eba0)?;
    // 0x2eb90: ldr r2, [pc, #0x1c]         @ 0x15b4 <entry+0x15b4>
    c.literal(R::R2,0x2ebb4)?;
    // 0x2eb94: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2eb98: mov r1, #139
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,139)?)?;
    // 0x2eb9c: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2d3f4
    c.bl(0x17b0)?;
    // 0x2eba0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2eba4: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2eba8)?;
    const CALLBACK_ADDRESS_DATA_2EBA8:[u32;5]=[0x00104104,0x0010e070,0x0010410c,0x000eac94,0x00104110];
    for word in CALLBACK_ADDRESS_DATA_2EBA8 {c.emit(word)?;}
    c.check(0x2ebbc)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;pack_bits_and_register_callbacks(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn packed_bits_and_callbacks_verify_pool(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let data=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(data(0x2eba8),0x00104104);
        assert_eq!(data(0x2ebac),0x0010e070);
        assert_eq!(data(0x2ebb8),0x00104110);
    }
}
