// SPDX-License-Identifier: GPL-3.0-only
//! Venus codec stream format setup, controller table copies and logging.
//! ARM A32 instructions are independently encoded from operands. Literal
//! address/diagnostic tables below are modeled as data, not OEM opcodes.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x32190;
const END:usize=0x32550;

/// ARM32 subroutine/handler at 0x32190; expected next boundary 0x32278.
fn verify_controller_start_sequence(c:&mut Code)->Result<(),String>{
    c.check(0x32190)?;
    // 0x32190: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x32194: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x32198: sub sp, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // 0x3219c: ldr r10, [pc, #0xd4]        @ 0x4f8 <entry+0x4f8>
    c.literal(R::new(10)?,0x32278)?;
    // 0x321a0: ldr r5, [pc, #0xd4]         @ 0x4fc <entry+0x4fc>
    c.literal(R::new(5)?,0x3227c)?;
    // 0x321a4: ldr r6, [pc, #0xd4]         @ 0x500 <entry+0x500>
    c.literal(R::new(6)?,0x32280)?;
    // 0x321a8: ldr r8, [pc, #0xd4]         @ 0x504 <entry+0x504>
    c.literal(R::new(8)?,0x32284)?;
    // 0x321ac: add r4, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::SP,8)?)?;
    // 0x321b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x321b4: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x321b8: add r7, r4, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,24)?)?;
    // 0x321bc: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x321c0: b 0x321cc <entry+0x44c>   @ imm = #0x4
    c.b(C::Al,0x321cc)?;
    // 0x321c4: ldrb r0, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1)?)?;
    // 0x321c8: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x321cc: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x321d0: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x321d4: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x321d8: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x321dc: mov r3, #141
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,141)?)?;
    // 0x321e0: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x321e4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x321e8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x355c
    c.bl(0x2ec94)?;
    // 0x321ec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x321f0: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x321f4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3568
    c.bl(0x2ec94)?;
    // 0x321f8: ldrb r0, [r4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x321fc: ldrb r1, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1)?)?;
    // 0x32200: ldrb r2, [r4, #0x2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,2)?)?;
    // 0x32204: ldrb r3, [r4, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,3)?)?;
    // 0x32208: orr r0, r0, r1, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,8,false)?)?;
    // 0x3220c: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x32210: orr r2, r2, r3, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x32214: orr r0, r0, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,16,false)?)?;
    // 0x32218: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3221c: bne 0x32260 <entry+0x4e0>   @ imm = #0x3c
    c.b(C::Ne,0x32260)?;
    // 0x32220: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x32224: ldr r1, [pc, #0x5c]         @ 0x508 <entry+0x508>
    c.literal(R::R1,0x32288)?;
    // 0x32228: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x3222c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x32230: mov r3, #144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,144)?)?;
    // 0x32234: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x32238: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3223c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x35b0
    c.bl(0x2ec94)?;
    // 0x32240: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32244: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x32248: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x35bc
    c.bl(0x2ec94)?;
    // 0x3224c: ldr r2, [pc, #0x38]         @ 0x50c <entry+0x50c>
    c.literal(R::R2,0x3228c)?;
    // 0x32250: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32254: mov r1, #145
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,145)?)?;
    // 0x32258: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x30ab0
    c.bl(0x17b0)?;
    // 0x3225c: mov r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // 0x32260: add r1, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,1)?)?;
    // 0x32264: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x32268: bne 0x321c4 <entry+0x444>   @ imm = #-0xac
    c.b(C::Ne,0x321c4)?;
    // 0x3226c: and r0, r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(9)?,255)?)?;
    // 0x32270: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x32274: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x32278)
}

/// ARM32 subroutine/handler at 0x32290; expected next boundary 0x32368.
fn prepare_stream_format_registration(c:&mut Code)->Result<(),String>{
    c.check(0x32290)?;
    // 0x32290: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x32294: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x32298: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x3229c: ldr r4, [pc, #0xcc]         @ 0x5f0 <entry+0x5f0>
    c.literal(R::new(4)?,0x32370)?;
    // 0x322a0: ldr r6, [pc, #0xc0]         @ 0x5e8 <entry+0x5e8>
    c.literal(R::new(6)?,0x32368)?;
    // 0x322a4: ldr r1, [pc, #0xc0]         @ 0x5ec <entry+0x5ec>
    c.literal(R::R1,0x3236c)?;
    // 0x322a8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x322ac: mov r3, #168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,168)?)?;
    // 0x322b0: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x322b4: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x322b8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x362c
    c.bl(0x2ec94)?;
    // 0x322bc: ldr r5, [pc, #0xb0]         @ 0x5f4 <entry+0x5f4>
    c.literal(R::new(5)?,0x32374)?;
    // 0x322c0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x322c4: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x322c8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x363c
    c.bl(0x2ec94)?;
    // 0x322cc: ldr r1, [pc, #0xa4]         @ 0x5f8 <entry+0x5f8>
    c.literal(R::R1,0x32378)?;
    // 0x322d0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x322d4: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x322d8: mov r3, #169
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,169)?)?;
    // 0x322dc: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x322e0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3654
    c.bl(0x2ec94)?;
    // 0x322e4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x322e8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x322ec: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3660
    c.bl(0x2ec94)?;
    // 0x322f0: ldr r1, [pc, #0x84]         @ 0x5fc <entry+0x5fc>
    c.literal(R::R1,0x3237c)?;
    // 0x322f4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x322f8: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x322fc: mov r3, #171
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,171)?)?;
    // 0x32300: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3674
    c.bl(0x2ec94)?;
    // 0x32304: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32308: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x3230c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3680
    c.bl(0x2ec94)?;
    // 0x32310: ldr r1, [pc, #0x68]         @ 0x600 <entry+0x600>
    c.literal(R::R1,0x32380)?;
    // 0x32314: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32318: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x3231c: mov r3, #172
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,172)?)?;
    // 0x32320: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3694
    c.bl(0x2ec94)?;
    // 0x32324: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32328: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x3232c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x36a0
    c.bl(0x2ec94)?;
    // 0x32330: ldr r0, [pc, #0x4c]         @ 0x604 <entry+0x604>
    c.literal(R::R0,0x32384)?;
    // 0x32334: ldr r1, [pc, #0x4c]         @ 0x608 <entry+0x608>
    c.literal(R::R1,0x32388)?;
    // 0x32338: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x3233c: mov r3, #176
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,176)?)?;
    // 0x32340: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x32344: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32348: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x36bc
    c.bl(0x2ec94)?;
    // 0x3234c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32350: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x32354: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x36c8
    c.bl(0x2ec94)?;
    // 0x32358: bl 0x324e0 <entry+0x760>   @ imm = #0x180
    c.bl(0x324e0)?;
    // 0x3235c: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x32360: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x32364: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x32368)
}

/// ARM32 subroutine/handler at 0x3238c; expected next boundary 0x323d8.
fn copy_active_controller_table(c:&mut Code)->Result<(),String>{
    c.check(0x3238c)?;
    // 0x3238c: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x32390: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x32394: ldr r1, [pc, #0x3c]         @ 0x658 <entry+0x658>
    c.literal(R::R1,0x323d8)?;
    // 0x32398: ldr r2, [pc, #0x3c]         @ 0x65c <entry+0x65c>
    c.literal(R::R2,0x323dc)?;
    // 0x3239c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x323a0: mov r3, #204
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,204)?)?;
    // 0x323a4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3718
    c.bl(0x2ec94)?;
    // 0x323a8: ldr r1, [pc, #0x30]         @ 0x660 <entry+0x660>
    c.literal(R::R1,0x323e0)?;
    // 0x323ac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x323b0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3724
    c.bl(0x2ec94)?;
    // 0x323b4: ldr r12, [pc, #0x28]        @ 0x664 <entry+0x664>
    c.literal(R::R12,0x323e4)?;
    // 0x323b8: ldr r1, [pc, #0x28]         @ 0x668 <entry+0x668>
    c.literal(R::R1,0x323e8)?;
    // 0x323bc: ldm r12!, {r0, r2, r3, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x323c0: stm r1!, {r0, r2, r3, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x323c4: ldm r12!, {r0, r2, r3, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x323c8: stm r1!, {r0, r2, r3, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x323cc: ldm r12, {r0, r2, r3, r4, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x323d0: stm r1, {r0, r2, r3, r4, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x323d4: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x323d8)
}

/// ARM32 subroutine/handler at 0x323ec; expected next boundary 0x3241c.
fn copy_inactive_controller_table(c:&mut Code)->Result<(),String>{
    c.check(0x323ec)?;
    // 0x323ec: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x323f0: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x323f4: ldr r12, [pc, #0x20]        @ 0x69c <entry+0x69c>
    c.literal(R::R12,0x3241c)?;
    // 0x323f8: ldr r1, [pc, #0x20]         @ 0x6a0 <entry+0x6a0>
    c.literal(R::R1,0x32420)?;
    // 0x323fc: ldm r12!, {r0, r2, r3, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x32400: stm r1!, {r0, r2, r3, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x32404: ldm r12!, {r0, r2, r3, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x32408: stm r1!, {r0, r2, r3, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x3240c: ldm r12, {r0, r2, r3, r4, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x32410: stm r1, {r0, r2, r3, r4, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x32414: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32418: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x3241c)
}

/// ARM32 subroutine/handler at 0x32424; expected next boundary 0x32464.
fn invoke_controller_table_copy(c:&mut Code)->Result<(),String>{
    c.check(0x32424)?;
    // 0x32424: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x32428: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x3242c: ldr r1, [pc, #0x30]         @ 0x6e4 <entry+0x6e4>
    c.literal(R::R1,0x32464)?;
    // 0x32430: ldr r2, [pc, #0x30]         @ 0x6e8 <entry+0x6e8>
    c.literal(R::R2,0x32468)?;
    // 0x32434: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x32438: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3243c: mov r3, #252
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,252)?)?;
    // 0x32440: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x37b4
    c.bl(0x2ec94)?;
    // 0x32444: ldr r1, [pc, #0x20]         @ 0x6ec <entry+0x6ec>
    c.literal(R::R1,0x3246c)?;
    // 0x32448: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3244c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x37c0
    c.bl(0x2ec94)?;
    // 0x32450: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x32454: bl 0x32470 <entry+0x6f0>   @ imm = #0x14
    c.bl(0x32470)?;
    // 0x32458: bl 0x324e0 <entry+0x760>   @ imm = #0x80
    c.bl(0x324e0)?;
    // 0x3245c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32460: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x32464)
}

/// ARM32 subroutine/handler at 0x32470; expected next boundary 0x324d0.
fn copy_controller_input_descriptor(c:&mut Code)->Result<(),String>{
    c.check(0x32470)?;
    // 0x32470: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x32474: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x32478: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x3247c: ldr r1, [pc, #0x4c]         @ 0x750 <entry+0x750>
    c.literal(R::R1,0x324d0)?;
    // 0x32480: ldr r2, [pc, #0x4c]         @ 0x754 <entry+0x754>
    c.literal(R::R2,0x324d4)?;
    // 0x32484: mov r3, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10)?)?;
    // 0x32488: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x3248c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32490: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x32494: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x32498: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x380c
    c.bl(0x2ec94)?;
    // 0x3249c: ldr r1, [pc, #0x34]         @ 0x758 <entry+0x758>
    c.literal(R::R1,0x324d8)?;
    // 0x324a0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x324a4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3818
    c.bl(0x2ec94)?;
    // 0x324a8: ldr r0, [pc, #0x2c]         @ 0x75c <entry+0x75c>
    c.literal(R::R0,0x324dc)?;
    // 0x324ac: ldm r0!, {r1, r2, r3, r4}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x324b0: stm r6!, {r1, r2, r3, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x324b4: ldm r0!, {r1, r2, r3, r4}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x324b8: stm r6!, {r1, r2, r3, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x324bc: ldm r0, {r1, r2, r3, r4, r5}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x324c0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x324c4: stm r6, {r1, r2, r3, r4, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R1.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x324c8: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x324cc: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x324d0)
}

/// ARM32 subroutine/handler at 0x324e0; expected next boundary 0x3250c.
fn log_controller_processing_start(c:&mut Code)->Result<(),String>{
    c.check(0x324e0)?;
    // 0x324e0: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x324e4: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x324e8: ldr r1, [pc, #0x1c]         @ 0x78c <entry+0x78c>
    c.literal(R::R1,0x3250c)?;
    // 0x324ec: ldr r2, [pc, #0x1c]         @ 0x790 <entry+0x790>
    c.literal(R::R2,0x32510)?;
    // 0x324f0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x324f4: mov r3, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,13)?)?;
    // 0x324f8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x386c
    c.bl(0x2ec94)?;
    // 0x324fc: ldr r1, [pc, #0x10]         @ 0x794 <entry+0x794>
    c.literal(R::R1,0x32514)?;
    // 0x32500: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32504: pop {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x32508: b 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x387c
    c.b(C::Al,0x2ec94)?;
    c.check(0x3250c)
}

/// ARM32 subroutine/handler at 0x32518; expected next boundary 0x32544.
fn log_controller_processing_failure(c:&mut Code)->Result<(),String>{
    c.check(0x32518)?;
    // 0x32518: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x3251c: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x32520: ldr r1, [pc, #0x1c]         @ 0x7c4 <entry+0x7c4>
    c.literal(R::R1,0x32544)?;
    // 0x32524: ldr r2, [pc, #0x1c]         @ 0x7c8 <entry+0x7c8>
    c.literal(R::R2,0x32548)?;
    // 0x32528: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3252c: mov r3, #197
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,197)?)?;
    // 0x32530: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x38a4
    c.bl(0x2ec94)?;
    // 0x32534: ldr r1, [pc, #0x10]         @ 0x7cc <entry+0x7cc>
    c.literal(R::R1,0x3254c)?;
    // 0x32538: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3253c: pop {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x32540: b 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x38b4
    c.b(C::Al,0x2ec94)?;
    c.check(0x32544)
}

// Original data pointers and constants at [0x32278,0x32290).
const STATE_START_MESSAGES:[u32;6]=[0x000eb466,0x000eb483,0x000e2788,0x00103068,0x000eb490,0x000eb4be];

// Original data pointers and constants at [0x32368,0x3238c).
const FORMAT_REGISTRATION_MESSAGES:[u32;9]=[0x000324e0,0x000eb4fb,0x000eb51a,0x000e2788,0x000eb52d,0x000eb53c,0x000eb579,0x41414141,0x000eb59d];

// Original data pointers and constants at [0x323d8,0x323ec).
const ACTIVE_TABLE_ADDRESSES:[u32;5]=[0x000eb5f3,0x000eb617,0x000e2788,0x00032518,0x0010dd9c];

// Original data pointers and constants at [0x3241c,0x32424).
const INACTIVE_TABLE_ADDRESSES:[u32;2]=[0x00032518,0x0010dd9c];

// Original data pointers and constants at [0x32464,0x32470).
const TABLE_COPY_MESSAGES:[u32;3]=[0x000eb638,0x000eb670,0x000e2788];

// Original data pointers and constants at [0x324d0,0x324e0).
const INPUT_DESCRIPTOR_ADDRESSES:[u32;4]=[0x000eb696,0x000eb6c8,0x000e2788,0x00032518];

// Original data pointers and constants at [0x3250c,0x32518).
const PROCESSING_START_MESSAGES:[u32;3]=[0x000eb6e6,0x000eb709,0x000e2788];

// Original data pointers and constants at [0x32544,0x32550).
const PROCESSING_FAILURE_MESSAGES:[u32;3]=[0x000eb71d,0x000eb74a,0x000e2788];

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    verify_controller_start_sequence(c)?;
    c.check(0x32278)?;
    for word in STATE_START_MESSAGES{c.emit(word)?;}
    c.check(0x32290)?;
    prepare_stream_format_registration(c)?;
    c.check(0x32368)?;
    for word in FORMAT_REGISTRATION_MESSAGES{c.emit(word)?;}
    c.check(0x3238c)?;
    copy_active_controller_table(c)?;
    c.check(0x323d8)?;
    for word in ACTIVE_TABLE_ADDRESSES{c.emit(word)?;}
    c.check(0x323ec)?;
    copy_inactive_controller_table(c)?;
    c.check(0x3241c)?;
    for word in INACTIVE_TABLE_ADDRESSES{c.emit(word)?;}
    c.check(0x32424)?;
    invoke_controller_table_copy(c)?;
    c.check(0x32464)?;
    for word in TABLE_COPY_MESSAGES{c.emit(word)?;}
    c.check(0x32470)?;
    copy_controller_input_descriptor(c)?;
    c.check(0x324d0)?;
    for word in INPUT_DESCRIPTOR_ADDRESSES{c.emit(word)?;}
    c.check(0x324e0)?;
    log_controller_processing_start(c)?;
    c.check(0x3250c)?;
    for word in PROCESSING_START_MESSAGES{c.emit(word)?;}
    c.check(0x32518)?;
    log_controller_processing_failure(c)?;
    c.check(0x32544)?;
    for word in PROCESSING_FAILURE_MESSAGES{c.emit(word)?;}
    c.check(0x32550)?;
    c.check(END)
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn stream_setup_and_table_copies_are_contiguous(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let at=|addr:usize|u32::from_le_bytes(c.bytes[addr-START..addr-START+4].try_into().unwrap());
        assert_eq!(at(0x32278),STATE_START_MESSAGES[0]);
        assert_eq!(at(0x32368),FORMAT_REGISTRATION_MESSAGES[0]);
        assert_eq!(at(0x323d8),ACTIVE_TABLE_ADDRESSES[0]);
        assert_eq!(at(0x324d0),INPUT_DESCRIPTOR_ADDRESSES[0]);
        assert_eq!(at(0x3254c),PROCESSING_FAILURE_MESSAGES[2]);
    }
}
