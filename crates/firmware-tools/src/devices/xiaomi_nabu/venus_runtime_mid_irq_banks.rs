// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller IRQ register-bank logging, IRQ shutdown, interrupt-mask
//! set/clear, and indexed resource readiness probes. One instruction block
//! loads adjacent registers via LDMIB. Diagnostic/MMIO words are data pools.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2f96c;
const END:usize=0x2fbc4;
fn update_irq_state_and_registers(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2f96c: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2f970: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x2f974: ldr r1, [pc, #0xa0]         @ 0x41c <entry+0x41c>
    c.literal(R::R1,0x2fa1c)?;
    // 0x2f978: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2f97c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2f980: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xcf4
    c.bl(0x2ec94)?;
    // 0x2f984: ldr r1, [pc, #0x94]         @ 0x420 <entry+0x420>
    c.literal(R::R1,0x2fa20)?;
    // 0x2f988: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2f98c: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xd00
    c.bl(0x2ec94)?;
    // 0x2f990: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2f994: ldr r2, [pc, #0x88]         @ 0x424 <entry+0x424>
    c.literal(R::R2,0x2fa24)?;
    // 0x2f998: mov r1, #51
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,51)?)?;
    // 0x2f99c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x2f9a0: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x2f9a4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f9a8: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f9ac: bl 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x2e204
    c.bl(0x17b0)?;
    // 0x2f9b0: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2f9b4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2f9b8: beq 0x2fa10 <entry+0x410>   @ imm = #0x50
    c.b(C::Eq,0x2fa10)?;
    // 0x2f9bc: ldr r5, [pc, #0x64]         @ 0x428 <entry+0x428>
    c.literal(R::new(5)?,0x2fa28)?;
    // 0x2f9c0: ldr r8, [pc, #0x64]         @ 0x42c <entry+0x42c>
    c.literal(R::new(8)?,0x2fa2c)?;
    // 0x2f9c4: ldr r9, [pc, #0x54]         @ 0x420 <entry+0x420>
    c.literal(R::new(9)?,0x2fa20)?;
    // 0x2f9c8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2f9cc: add r6, r4, r7, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(4)?,R::new(7)?,Shift::Lsl,3,false)?)?;
    // 0x2f9d0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2f9d4: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x2f9d8: ldmib r6, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(6)?,R::R2.bit()|R::new(3)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x2f9dc: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xd50
    c.bl(0x2ec94)?;
    // 0x2f9e0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2f9e4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2f9e8: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xd5c
    c.bl(0x2ec94)?;
    // 0x2f9ec: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // 0x2f9f0: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x2f9f4: str r0, [r5], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R0,R::new(5)?,4)?)?;
    // 0x2f9f8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2f9fc: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2fa00: blo 0x2f9cc <entry+0x3cc>   @ imm = #-0x3c
    c.b(C::Cc,0x2f9cc)?;
    // 0x2fa04: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x2fa08: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2fa0c: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x2fa10: ldr r0, [pc, #0x18]         @ 0x430 <entry+0x430>
    c.literal(R::R0,0x2fa30)?;
    // 0x2fa14: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x2fa18: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2fa1c)?;
    const IRQ_MMIO_AND_LOG_DATA_2FA1C:[u32;6]=[0x000eafa7,0x000e2788,0x000eafbb,0x0010411c,0x000eafeb,0x00104124];
    for value in IRQ_MMIO_AND_LOG_DATA_2FA1C {c.emit(value)?;}
    c.check(0x2fa34)?;
    // 0x2fa34: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2fa38: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2fa3c: ldr r4, [pc, #0x5c]         @ 0x4a0 <entry+0x4a0>
    c.literal(R::new(4)?,0x2faa0)?;
    // 0x2fa40: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2fa44: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2fa48: beq 0x2fa70 <entry+0x470>   @ imm = #0x20
    c.b(C::Eq,0x2fa70)?;
    // 0x2fa4c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2fa50: bl 0x27180 <entry+0xffffffffffff7b80> @ imm = #-0x88d8
    c.bl(0x27180)?;
    // 0x2fa54: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2fa58: bne 0x2fa70 <entry+0x470>   @ imm = #0x10
    c.b(C::Ne,0x2fa70)?;
    // 0x2fa5c: ldr r0, [pc, #0x40]         @ 0x4a4 <entry+0x4a4>
    c.literal(R::R0,0x2faa4)?;
    // 0x2fa60: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x2fa64: ldr r2, [pc, #0x3c]         @ 0x4a8 <entry+0x4a8>
    c.literal(R::R2,0x2faa8)?;
    // 0x2fa68: and r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x2fa6c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2fa70: ldr r0, [pc, #0x34]         @ 0x4ac <entry+0x4ac>
    c.literal(R::R0,0x2faac)?;
    // 0x2fa74: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x2fa78: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2fa7c: ldr r1, [pc, #0x2c]         @ 0x4b0 <entry+0x4b0>
    c.literal(R::R1,0x2fab0)?;
    // 0x2fa80: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2fa84: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x2fa88: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2fa8c: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xe00
    c.bl(0x2ec94)?;
    // 0x2fa90: ldr r1, [pc, #0x1c]         @ 0x4b4 <entry+0x4b4>
    c.literal(R::R1,0x2fab4)?;
    // 0x2fa94: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2fa98: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2fa9c: b 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xe10
    c.b(C::Al,0x2ec94)?;
    c.check(0x2faa0)?;
    const IRQ_MMIO_AND_LOG_DATA_2FAA0:[u32;6]=[0x00104124,0xe0003b30,0xfffeeeee,0x0010411c,0x000eb00b,0x000e2788];
    for value in IRQ_MMIO_AND_LOG_DATA_2FAA0 {c.emit(value)?;}
    c.check(0x2fab8)?;
    // 0x2fab8: ldr r0, [pc, #0x4]          @ 0x4c4 <entry+0x4c4>
    c.literal(R::R0,0x2fac4)?;
    // 0x2fabc: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2fac0: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2fac4)?;
    const IRQ_MMIO_AND_LOG_DATA_2FAC4:[u32;1]=[0x00104124];
    for value in IRQ_MMIO_AND_LOG_DATA_2FAC4 {c.emit(value)?;}
    c.check(0x2fac8)?;
    // 0x2fac8: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x2facc: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x2fad0: bne 0x2fb04 <entry+0x504>   @ imm = #0x2c
    c.b(C::Ne,0x2fb04)?;
    // 0x2fad4: ldr r1, [pc, #0x54]         @ 0x530 <entry+0x530>
    c.literal(R::R1,0x2fb30)?;
    // 0x2fad8: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x2fadc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2fae0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2fae4: beq 0x2fb08 <entry+0x508>   @ imm = #0x1c
    c.b(C::Eq,0x2fb08)?;
    // 0x2fae8: ldr r2, [pc, #0x44]         @ 0x534 <entry+0x534>
    c.literal(R::R2,0x2fb34)?;
    // 0x2faec: rsb r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R0,1)?)?;
    // 0x2faf0: mov r12, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,17)?)?;
    // 0x2faf4: ldr r1, [r2, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,2)?)?;
    // 0x2faf8: mov r2, #7936
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7936)?)?;
    // 0x2fafc: and r1, r2, r1, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,8,false)?)?;
    // 0x2fb00: b 0x2fb08 <entry+0x508>   @ imm = #0x0
    c.b(C::Al,0x2fb08)?;
    // 0x2fb04: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2fb08: mov r3, #824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,824)?)?;
    // 0x2fb0c: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x2fb10: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x2fb14: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x2fb18: str r1, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x2fb1c: mov r1, #2464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2464)?)?;
    // 0x2fb20: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x2fb24: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // 0x2fb28: str r12, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R12,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x2fb2c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2fb30)?;
    const IRQ_MMIO_AND_LOG_DATA_2FB30:[u32;2]=[0x00104124,0x0010411c];
    for value in IRQ_MMIO_AND_LOG_DATA_2FB30 {c.emit(value)?;}
    c.check(0x2fb38)?;
    // 0x2fb38: push {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2fb3c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2fb40: ldr r5, [pc, #0x6c]         @ 0x5b4 <entry+0x5b4>
    c.literal(R::new(5)?,0x2fbb4)?;
    // 0x2fb44: ldr r1, [pc, #0x6c]         @ 0x5b8 <entry+0x5b8>
    c.literal(R::R1,0x2fbb8)?;
    // 0x2fb48: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2fb4c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2fb50: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x2fb54: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xec8
    c.bl(0x2ec94)?;
    // 0x2fb58: ldr r1, [pc, #0x5c]         @ 0x5bc <entry+0x5bc>
    c.literal(R::R1,0x2fbbc)?;
    // 0x2fb5c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2fb60: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xed4
    c.bl(0x2ec94)?;
    // 0x2fb64: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2fb68: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2fb6c: popeq {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2fb70: ldr r0, [pc, #0x48]         @ 0x5c0 <entry+0x5c0>
    c.literal(R::R0,0x2fbc0)?;
    // 0x2fb74: mov r1, #7936
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7936)?)?;
    // 0x2fb78: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x2fb7c: ldr r0, [r0, r4, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,2)?)?;
    // 0x2fb80: and r1, r1, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsl,8,false)?)?;
    // 0x2fb84: and r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // 0x2fb88: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2fb8c: mov r1, #820
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,820)?)?;
    // 0x2fb90: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x2fb94: orr r1, r1, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x2fb98: str r0, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::R2,Shift::Lsl,0)?)?;
    // 0x2fb9c: mov r0, #2864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2864)?)?;
    // 0x2fba0: mov r1, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,17)?)?;
    // 0x2fba4: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x2fba8: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x2fbac: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x2fbb0: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2fbb4)?;
    const IRQ_MMIO_AND_LOG_DATA_2FBB4:[u32;4]=[0x00104124,0x000eb021,0x000e2788,0x0010411c];
    for value in IRQ_MMIO_AND_LOG_DATA_2FBB4 {c.emit(value)?;}
    c.check(0x2fbc4)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;update_irq_state_and_registers(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn irq_register_bank_data_is_explicit(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let read=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(read(0x2fa28),0x0010411c);
        assert_eq!(read(0x2fac4),0x00104124);
        assert_eq!(read(0x2fb30),0x00104124);
        assert_eq!(read(0x2fbc0),0x0010411c);
    }
}
