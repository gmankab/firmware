// SPDX-License-Identifier: GPL-3.0-only
//! Codec output-frame controller matrix completion to Agent 20-03's reserved
//! upper boundary 0x40000. Emitted A32 operations preserve conditional flags,
//! optional halfword register-indexed loads, carry-aware 64-bit arithmetic
//! and indirect control paths; the original function continues at 0x40000.
//! No trailing return, opaque OEM opcode bytes or invented padding are used.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
const START:usize=0x3fc00;
const END:usize=0x40000;

/// A32 LDRH Rt,[Rn,Rm] register-offset, positive preindexed no writeback.
/// Fields: P=1,U=1,I=0,L=1,S=0,H=1, and 0xb in bits [7:4].
fn halfword_register(cond:C,rt:R,rn:R,rm:R)->Result<u32,String>{
    if [rt,rn,rm].iter().any(|r|*r==R::PC){
        return Err("A32 register-indexed LDRH cannot use PC".into());
    }
    Ok(((cond as u32)<<28)|0x0190_00b0|
       (rn.number()<<16)|(rt.number()<<12)|rm.number())
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3fc00: orr r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3fc04: mov r0, #1044480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1044480)?)?;
    // 0x3fc08: movne r9, r4
    c.emit(arm32::mov_register(C::Ne,R::new(9)?,R::new(4)?))?;
    // 0x3fc0c: orr r0, r0, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32505856)?)?;
    // 0x3fc10: and r0, r0, r9, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(9)?,Shift::Lsl,12,false)?)?;
    // 0x3fc14: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3fc18: str r8, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,88)?)?;
    // 0x3fc1c: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x3fc20: add r4, r7, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(7)?,32)?)?;
    // 0x3fc24: ldreq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,0)?)?;
    // 0x3fc28: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x3fc2c: streq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,0)?)?;
    // 0x3fc30: ldr r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
    // 0x3fc34: rsb r3, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(3)?,R::R1,0)?)?;
    // 0x3fc38: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x3fc3c: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x3fc40: str r1, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,16)?)?;
    // 0x3fc44: str r0, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,20)?)?;
    // 0x3fc48: add r1, r7, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,24)?)?;
    // 0x3fc4c: ldr r8, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,68)?)?;
    // 0x3fc50: ldr r9, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,100)?)?;
    // 0x3fc54: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x3fc58: add r0, r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,3)?)?;
    // 0x3fc5c: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x3fc60: str r1, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // 0x3fc64: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x3fc68: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x3fc6c: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x3fc70: and r0, lr, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::LR,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3fc74: orr r0, r0, r12, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R12,Shift::Lsl,16,false)?)?;
    // 0x3fc78: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x3fc7c: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x3fc80: ldr r1, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,88)?)?;
    // 0x3fc84: rsb r1, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R1,0)?)?;
    // 0x3fc88: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,64)?)?;
    // 0x3fc8c: mov r1, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28)?)?;
    // 0x3fc90: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x3fc94: str r1, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,24)?)?;
    // 0x3fc98: str r0, [r7, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,28)?)?;
    // 0x3fc9c: add r0, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,4)?)?;
    // 0x3fca0: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x3fca4: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x3fca8: str r4, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,0)?)?;
    // 0x3fcac: str r3, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,28)?)?;
    // 0x3fcb0: str r10, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,40)?)?;
    // 0x3fcb4: ldr r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,80)?)?;
    // 0x3fcb8: rsb r0, r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,18)?)?;
    // 0x3fcbc: add r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3fcc0: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x3fcc4: lsl r0, lr, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::LR,Shift::Lsl,R::R0,false)?)?;
    // 0x3fcc8: blx 0x83c4c <entry+0x44900> @ imm = #0x43f7c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x3fccc: ldr r5, [pc, #0xd48]        @ 0x16d0 <entry+0x16d0>
    c.literal_cond(C::Al,R::new(5)?,0x40a1c)?;
    // 0x3fcd0: mov r10, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1048576)?)?;
    // 0x3fcd4: tst r0, r5
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3fcd8: add r6, r5, #1835008
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(5)?,1835008)?)?;
    // 0x3fcdc: addne r0, r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R0,1)?)?;
    // 0x3fce0: and r1, r0, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3fce4: cmp r1, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1048576)?)?;
    // 0x3fce8: movhs r1, r10
    c.emit(arm32::mov_register(C::Cs,R::R1,R::new(10)?))?;
    // 0x3fcec: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x3fcf0: ldreq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,0)?)?;
    // 0x3fcf4: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x3fcf8: streq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,0)?)?;
    // 0x3fcfc: mov r0, #2080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2080)?)?;
    // 0x3fd00: add r4, r7, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(7)?,40)?)?;
    // 0x3fd04: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x3fd08: str r0, [r7, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,32)?)?;
    // 0x3fd0c: add r0, r8, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,5)?)?;
    // 0x3fd10: str r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x3fd14: str r1, [r7, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,36)?)?;
    // 0x3fd18: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x3fd1c: str r4, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,0)?)?;
    // 0x3fd20: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x3fd24: ldr r1, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,84)?)?;
    // 0x3fd28: add r0, r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,18)?)?;
    // 0x3fd2c: sub r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3fd30: ldr r1, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,96)?)?;
    // 0x3fd34: lsl r0, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R1,Shift::Lsl,R::R0,false)?)?;
    // 0x3fd38: ldr r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // 0x3fd3c: blx 0x83c4c <entry+0x44900> @ imm = #0x43f08
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x3fd40: tst r0, r5
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3fd44: mov r3, #134
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,134)?)?;
    // 0x3fd48: mov r2, #392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,392)?)?;
    // 0x3fd4c: addne r0, r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R0,1)?)?;
    // 0x3fd50: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x3fd54: and r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3fd58: mov r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,255)?)?;
    // 0x3fd5c: cmp r0, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1048576)?)?;
    // 0x3fd60: orr r6, r6, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,768)?)?;
    // 0x3fd64: movlo r10, r0
    c.emit(arm32::mov_register(C::Cc,R::new(10)?,R::R0))?;
    // 0x3fd68: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x3fd6c: ldreq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,0)?)?;
    // 0x3fd70: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x3fd74: streq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,0)?)?;
    // 0x3fd78: mov r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,44)?)?;
    // 0x3fd7c: orr r0, r0, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    // 0x3fd80: str r0, [r7, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,40)?)?;
    // 0x3fd84: add r0, r8, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,6)?)?;
    // 0x3fd88: str r10, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,44)?)?;
    // 0x3fd8c: str r10, [r7, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(7)?,44)?)?;
    // 0x3fd90: add r10, r7, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(7)?,64)?)?;
    // 0x3fd94: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x3fd98: add r0, r7, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,48)?)?;
    // 0x3fd9c: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x3fda0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3fda4: ldr r5, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,104)?)?;
    // 0x3fda8: ldr r4, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,76)?)?;
    // 0x3fdac: ldr r1, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2744)?)?;
    // 0x3fdb0: ldrh r2, [r1, r2]
    c.emit(halfword_register(C::Al,R::R2,R::R1,R::R2)?)?;
    // 0x3fdb4: ldrh r1, [r1, r3]
    c.emit(halfword_register(C::Al,R::R1,R::R1,R::new(3)?)?)?;
    // 0x3fdb8: mov r3, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,138)?)?;
    // 0x3fdbc: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x3fdc0: and r2, r2, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3fdc4: and r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3fdc8: orr r1, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x3fdcc: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x3fdd0: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x3fdd4: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x3fdd8: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // 0x3fddc: mov r2, #396
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,396)?)?;
    // 0x3fde0: orr r0, r0, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    // 0x3fde4: str r0, [r7, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,48)?)?;
    // 0x3fde8: add r0, r8, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,7)?)?;
    // 0x3fdec: str r1, [r7, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,52)?)?;
    // 0x3fdf0: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x3fdf4: add r0, r7, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,56)?)?;
    // 0x3fdf8: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x3fdfc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3fe00: ldr r1, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2744)?)?;
    // 0x3fe04: ldrh r2, [r1, r2]
    c.emit(halfword_register(C::Al,R::R2,R::R1,R::R2)?)?;
    // 0x3fe08: ldrh r1, [r1, r3]
    c.emit(halfword_register(C::Al,R::R1,R::R1,R::new(3)?)?)?;
    // 0x3fe0c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3fe10: and r2, r2, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3fe14: and r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3fe18: ldr r6, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,40)?)?;
    // 0x3fe1c: orr r1, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x3fe20: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x3fe24: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x3fe28: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x3fe2c: mov r0, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,60)?)?;
    // 0x3fe30: orr r0, r0, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    // 0x3fe34: str r0, [r7, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,56)?)?;
    // 0x3fe38: add r0, r8, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,8)?)?;
    // 0x3fe3c: str r1, [r7, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,60)?)?;
    // 0x3fe40: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x3fe44: str r10, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(9)?,0)?)?;
    // 0x3fe48: ldr r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,80)?)?;
    // 0x3fe4c: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x3fe50: sub r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3fe54: ldr r1, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,644)?)?;
    // 0x3fe58: mov r5, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,22)?)?;
    // 0x3fe5c: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x3fe60: moveq r5, #23
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,23)?)?;
    // 0x3fe64: add r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3fe68: sub r2, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R0,32)?)?;
    // 0x3fe6c: rsb r1, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R0,32)?)?;
    // 0x3fe70: lsl r0, r4, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(4)?,Shift::Lsl,R::R0,false)?)?;
    // 0x3fe74: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x3fe78: lsr r1, r4, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::new(4)?,Shift::Lsr,R::R1,false)?)?;
    // 0x3fe7c: movge r0, #0
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3fe80: lslge r1, r4, r2
    c.emit(arm32::data_register_shifted_by_register(C::Ge,Op::Mov,R::R1,R::R0,R::new(4)?,Shift::Lsl,R::R2,false)?)?;
    // 0x3fe84: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x3fe88: adds r0, r0, r6, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsr,1,true)?)?;
    // 0x3fe8c: adc r1, r1, #0
    c.emit(super::encode_adc_zero(C::Al,R::R1,R::R1)?)?;
    // 0x3fe90: bl 0x8187c <entry+0x42530> @ imm = #0x419e4
    c.bl(0x8187c)?;
    // 0x3fe94: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x3fe98: ldr r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,84)?)?;
    // 0x3fe9c: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,64)?)?;
    // 0x3fea0: ldr r3, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,96)?)?;
    // 0x3fea4: sub r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3fea8: add r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3feac: rsb r1, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R0,32)?)?;
    // 0x3feb0: sub r2, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R0,32)?)?;
    // 0x3feb4: lsl r0, r3, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(3)?,Shift::Lsl,R::R0,false)?)?;
    // 0x3feb8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x3febc: lsr r1, r3, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::new(3)?,Shift::Lsr,R::R1,false)?)?;
    // 0x3fec0: lslge r1, r3, r2
    c.emit(arm32::data_register_shifted_by_register(C::Ge,Op::Mov,R::R1,R::R0,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // 0x3fec4: ldr r2, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,72)?)?;
    // 0x3fec8: movge r0, #0
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3fecc: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3fed0: adds r0, r0, r2, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsr,1,true)?)?;
    // 0x3fed4: adc r1, r1, #0
    c.emit(super::encode_adc_zero(C::Al,R::R1,R::R1)?)?;
    // 0x3fed8: bl 0x8187c <entry+0x42530> @ imm = #0x4199c
    c.bl(0x8187c)?;
    // 0x3fedc: tst r10, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(10)?,31)?)?;
    // 0x3fee0: bic r2, r4, #-134217728
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::new(4)?,4160749568)?)?;
    // 0x3fee4: mov r4, r8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
    // 0x3fee8: bic r0, r0, #-134217728
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,4160749568)?)?;
    // 0x3feec: add r5, r7, #80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(7)?,80)?)?;
    // 0x3fef0: lsr r12, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R12,R::new(6)?,Shift::Lsr,1)?)?;
    // 0x3fef4: ldreq r1, [r10]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(10)?,0)?)?;
    // 0x3fef8: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x3fefc: streq r1, [r10]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(10)?,0)?)?;
    // 0x3ff00: mov r1, #152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,152)?)?;
    // 0x3ff04: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x3ff08: str r1, [r7, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,64)?)?;
    // 0x3ff0c: add r1, r8, #9
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,9)?)?;
    // 0x3ff10: str r2, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,72)?)?;
    // 0x3ff14: str r2, [r7, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,68)?)?;
    // 0x3ff18: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x3ff1c: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x3ff20: add r1, r7, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,72)?)?;
    // 0x3ff24: str r1, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // 0x3ff28: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x3ff2c: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x3ff30: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x3ff34: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x3ff38: mov r1, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,156)?)?;
    // 0x3ff3c: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x3ff40: str r1, [r7, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,72)?)?;
    // 0x3ff44: add r1, r4, #10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,10)?)?;
    // 0x3ff48: str r0, [r7, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,76)?)?;
    // 0x3ff4c: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x3ff50: str r5, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(9)?,0)?)?;
    // 0x3ff54: ldr r8, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
    // 0x3ff58: ldr r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,76)?)?;
    // 0x3ff5c: ldr r1, [r8, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,928)?)?;
    // 0x3ff60: str r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // 0x3ff64: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x3ff68: mul r6, r1, r10
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::R1,R::new(10)?)?)?;
    // 0x3ff6c: rsb r1, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R0,32)?)?;
    // 0x3ff70: sub r3, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R0,32)?)?;
    // 0x3ff74: lsr r1, r6, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::new(6)?,Shift::Lsr,R::R1,false)?)?;
    // 0x3ff78: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x3ff7c: lslge r1, r6, r3
    c.emit(arm32::data_register_shifted_by_register(C::Ge,Op::Mov,R::R1,R::R0,R::new(6)?,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x3ff80: lsl r3, r6, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(3)?,R::R0,R::new(6)?,Shift::Lsl,R::R0,false)?)?;
    // 0x3ff84: movge r3, #0
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3ff88: adds r0, r3, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R12,Shift::Lsl,0,true)?)?;
    // 0x3ff8c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3ff90: adc r1, r1, #0
    c.emit(super::encode_adc_zero(C::Al,R::R1,R::R1)?)?;
    // 0x3ff94: bl 0x8187c <entry+0x42530> @ imm = #0x418e0
    c.bl(0x8187c)?;
    // 0x3ff98: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x3ff9c: bic r0, r0, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,2147483648)?)?;
    // 0x3ffa0: ldreq r1, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,0)?)?;
    // 0x3ffa4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x3ffa8: streq r1, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(5)?,0)?)?;
    // 0x3ffac: mov r1, #188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,188)?)?;
    // 0x3ffb0: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x3ffb4: str r1, [r7, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,80)?)?;
    // 0x3ffb8: str r0, [r7, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,84)?)?;
    // 0x3ffbc: add r0, r4, #11
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,11)?)?;
    // 0x3ffc0: mov r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,16)?)?;
    // 0x3ffc4: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x3ffc8: add r0, r7, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,88)?)?;
    // 0x3ffcc: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x3ffd0: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // 0x3ffd4: ldr r2, [r8, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,644)?)?;
    // 0x3ffd8: ldr r1, [r8, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,716)?)?;
    // 0x3ffdc: ldr r3, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,92)?)?;
    // 0x3ffe0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x3ffe4: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // 0x3ffe8: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x3ffec: moveq r4, #32
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,32)?)?;
    // 0x3fff0: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x3fff4: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x3fff8: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x3fffc: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    c.check(END)
}

#[cfg(test)] mod tests {
 use super::*;
 #[test] fn checked_halfword_indexed_encoding(){
   assert_eq!(halfword_register(C::Al,R::R2,R::R1,R::R2).unwrap(),0xe191_20b2);
   assert_eq!(halfword_register(C::Al,R::R1,R::R1,R::new(3).unwrap()).unwrap(),0xe191_10b3);
   assert!(halfword_register(C::Al,R::PC,R::R1,R::R2).is_err());
   assert!(halfword_register(C::Al,R::R0,R::PC,R::R2).is_err());
 }
}
