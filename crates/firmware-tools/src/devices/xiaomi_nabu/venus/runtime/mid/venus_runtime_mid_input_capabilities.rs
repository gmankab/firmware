// SPDX-License-Identifier: GPL-3.0-only
//! Initialize controller input capability byte fields and per-channel state.
//! This stage uses STRB preindexed writeback and checks queued device modes.
//! MMIO pointers and diagnostic strings are a separate literal pool.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x31b80;
const END:usize=0x31d80;
fn configure_input_capability_flags(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x31b80: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x31b84: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x31b88: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // 0x31b8c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x31b90: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31b94: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x31b98: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x31b9c: strb r0, [r1, #0x31]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::R1,49)?)?;
    // 0x31ba0: strb r0, [r1, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,3)?)?;
    // 0x31ba4: strb r0, [r1, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,2)?)?;
    // 0x31ba8: strb r0, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,1)?)?;
    // 0x31bac: strb r2, [r1, #0x11]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,17)?)?;
    // 0x31bb0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x31bb4: strb r2, [r1, #0x7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,7)?)?;
    // 0x31bb8: strb r0, [r1, #0x8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,8)?)?;
    // 0x31bbc: strb r2, [r1, #0x9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,9)?)?;
    // 0x31bc0: strh r0, [r1, #11]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,11)?)?;
    // 0x31bc4: strb r0, [r1, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,13)?)?;
    // 0x31bc8: strb r2, [r1, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,14)?)?;
    // 0x31bcc: strb r0, [r1, #0xf]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,15)?)?;
    // 0x31bd0: strb r0, [r1, #0x18]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,24)?)?;
    // 0x31bd4: ldrb r3, [r1, #0x4]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,4)?)?;
    // 0x31bd8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x31bdc: beq 0x31c14 <entry+0x2614>  @ imm = #0x30
    c.b(C::Eq,0x31c14)?;
    // 0x31be0: strb r2, [r4, #0x43]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,67)?)?;
    // 0x31be4: strb r2, [r4, #0x44]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,68)?)?;
    // 0x31be8: strb r0, [r4, #0x45]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,69)?)?;
    // 0x31bec: strb r0, [r4, #0x46]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,70)?)?;
    // 0x31bf0: ldr r0, [pc, #0x148]        @ 0x2740 <entry+0x2740>
    c.literal(R::R0,0x31d40)?;
    // 0x31bf4: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x31bf8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x31bfc: bne 0x31c34 <entry+0x2634>  @ imm = #0x30
    c.b(C::Ne,0x31c34)?;
    // 0x31c00: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x31c04: strb r0, [r4, #0x47]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,71)?)?;
    // 0x31c08: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31c0c: strb r0, [r4, #0x48]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,72)?)?;
    // 0x31c10: b 0x31c74 <entry+0x2674>  @ imm = #0x5c
    c.b(C::Al,0x31c74)?;
    // 0x31c14: add r1, r1, #18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,18)?)?;
    // 0x31c18: strb r0, [r1, #0x5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,5)?)?;
    // 0x31c1c: strb r0, [r1, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x31c20: strb r0, [r1, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,3)?)?;
    // 0x31c24: strb r0, [r1, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,2)?)?;
    // 0x31c28: strb r0, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,1)?)?;
    // 0x31c2c: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x31c30: b 0x31d30 <entry+0x2730>  @ imm = #0xf8
    c.b(C::Al,0x31d30)?;
    // 0x31c34: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31c38: strb r0, [r4, #0x48]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,72)?)?;
    // 0x31c3c: bl 0x2f8d0 <entry+0x2d0>   @ imm = #-0x2374
    c.bl(0x2f8d0)?;
    // 0x31c40: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x31c44: beq 0x31c54 <entry+0x2654>  @ imm = #0x8
    c.b(C::Eq,0x31c54)?;
    // 0x31c48: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31c4c: strb r0, [r4, #0x48]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,72)?)?;
    // 0x31c50: b 0x31c6c <entry+0x266c>  @ imm = #0x14
    c.b(C::Al,0x31c6c)?;
    // 0x31c54: bl 0x2f8ac <entry+0x2ac>   @ imm = #-0x23b0
    c.bl(0x2f8ac)?;
    // 0x31c58: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x31c5c: beq 0x31d38 <entry+0x2738>  @ imm = #0xd4
    c.b(C::Eq,0x31d38)?;
    // 0x31c60: ldr r0, [pc, #0xdc]         @ 0x2744 <entry+0x2744>
    c.literal(R::R0,0x31d44)?;
    // 0x31c64: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x31c68: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x31c6c: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x31c70: strb r0, [r4, #0x47]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,71)?)?;
    // 0x31c74: ldr r0, [pc, #0xcc]         @ 0x2748 <entry+0x2748>
    c.literal(R::R0,0x31d48)?;
    // 0x31c78: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // 0x31c7c: tst r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,4)?)?;
    // 0x31c80: ldrne r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::R0,20)?)?;
    // 0x31c84: strbne r2, [r4, #0x47]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R2,R::new(4)?,71)?)?;
    // 0x31c88: tst r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,2)?)?;
    // 0x31c8c: ldrne r2, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::R0,16)?)?;
    // 0x31c90: strbne r2, [r4, #0x49]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R2,R::new(4)?,73)?)?;
    // 0x31c94: tst r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1)?)?;
    // 0x31c98: ldrne r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::R0,12)?)?;
    // 0x31c9c: strbne r0, [r4, #0x48]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,72)?)?;
    // 0x31ca0: ldr r0, [pc, #0xa4]         @ 0x274c <entry+0x274c>
    c.literal(R::R0,0x31d4c)?;
    // 0x31ca4: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x31ca8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x31cac: beq 0x31d30 <entry+0x2730>  @ imm = #0x7c
    c.b(C::Eq,0x31d30)?;
    // 0x31cb0: ldr r1, [pc, #0x98]         @ 0x2750 <entry+0x2750>
    c.literal(R::R1,0x31d50)?;
    // 0x31cb4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x31cb8: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x302c
    c.bl(0x2ec94)?;
    // 0x31cbc: ldr r6, [pc, #0x90]         @ 0x2754 <entry+0x2754>
    c.literal(R::new(6)?,0x31d54)?;
    // 0x31cc0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x31cc4: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x31cc8: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x303c
    c.bl(0x2ec94)?;
    // 0x31ccc: ldr r5, [pc, #0x84]         @ 0x2758 <entry+0x2758>
    c.literal(R::new(5)?,0x31d58)?;
    // 0x31cd0: add r4, r5, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(5)?,12)?)?;
    // 0x31cd4: ldm r5, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::R2.bit()|R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x31cd8: ldm r4, {r0, r1, r4}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(4)?,R::R0.bit()|R::R1.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x31cdc: stm sp, {r0, r1, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x31ce0: ldr r1, [pc, #0x74]         @ 0x275c <entry+0x275c>
    c.literal(R::R1,0x31d5c)?;
    // 0x31ce4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x31ce8: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x305c
    c.bl(0x2ec94)?;
    // 0x31cec: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x31cf0: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x31cf4: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x3068
    c.bl(0x2ec94)?;
    // 0x31cf8: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x31cfc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x31d00: and r2, r1, r0, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,2,false)?)?;
    // 0x31d04: and r3, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // 0x31d08: ldr r1, [pc, #0x50]         @ 0x2760 <entry+0x2760>
    c.literal(R::R1,0x31d60)?;
    // 0x31d0c: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x31d10: and r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,1)?)?;
    // 0x31d14: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x31d18: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x308c
    c.bl(0x2ec94)?;
    // 0x31d1c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x31d20: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x31d24: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x31d28: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x31d2c: b 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x30a0
    c.b(C::Al,0x2ec94)?;
    // 0x31d30: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x31d34: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x31d38: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x31d3c: b 0x31c70 <entry+0x2670>  @ imm = #-0xd4
    c.b(C::Al,0x31c70)?;
    c.check(0x31d40)?;
    const CODEC_INPUT_CAPABILITY_DATA:[u32;9]=[0x0010df58,0x00103070,0x0010df5c,0x001040e4,0x000e53ee,0x000e2788,0x0010df7c,0x000e541f,0x000e5471];
    for word in CODEC_INPUT_CAPABILITY_DATA {c.emit(word)?;}
    c.check(0x31d64)?;
    // 0x31d64: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // 0x31d68: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x31d6c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x31d70: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x31d74: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // 0x31d78: ldr r0, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x31d7c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;configure_input_capability_flags(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn capability_flags_literal_and_register_offsets_are_explicit(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let at=|addr:usize|u32::from_le_bytes(c.bytes[addr-START..addr-START+4].try_into().unwrap());
        assert_eq!(at(0x31d40),0x0010df58);
        assert_eq!(at(0x31d44),0x00103070);
        assert_eq!(at(0x31d50),0x000e53ee);
        assert_eq!(at(0x31d60),0x000e5471);
    }
}
