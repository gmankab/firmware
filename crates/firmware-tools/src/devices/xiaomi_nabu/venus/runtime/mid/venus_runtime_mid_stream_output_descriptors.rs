// SPDX-License-Identifier: GPL-3.0-only
//! Input/output stream descriptor publication through codec-controller
//! command records, conserving queue boundaries and original callbacks.
//! Two constant words after saved-register epilogues are separately modeled
//! hardware alignment masks; they are NOT ARM SVC instructions.
use super::Code;
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
const START:usize=0x3ee98;
const END:usize=0x3f34c;
const HARDWARE_VALID_BITS:u32=0x7fff_ff00;

fn transfer_packed_stream_descriptors(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3ee98: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3ee9c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x3eea0: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // 0x3eea4: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x3eea8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3eeac: mov r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // 0x3eeb0: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x3eeb4: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x3eeb8: add r8, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,32)?)?;
    // 0x3eebc: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x3eec0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3eec4: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x3eec8: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x3eecc: ldr r0, [r5, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,956)?)?;
    // 0x3eed0: sub r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,2048)?)?;
    // 0x3eed4: add r0, r1, r0, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,2,false)?)?;
    // 0x3eed8: ldr r1, [pc, #0x328]        @ 0x370 <entry+0x370>
    c.literal_cond(C::Al,R::R1,0x3f208)?;
    // 0x3eedc: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3eee0: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x3eee4: str r0, [r5, #0x418]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1048)?)?;
    // 0x3eee8: ldr r0, [r5, #0x548]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1352)?)?;
    // 0x3eeec: ldr r1, [r5, #0x414]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1044)?)?;
    // 0x3eef0: ldrb r7, [r5, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,350)?)?;
    // 0x3eef4: ldr r10, [r9, #0x110]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(9)?,272)?)?;
    // 0x3eef8: ldr r6, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,296)?)?;
    // 0x3eefc: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3ef00: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3ef04: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x3ef08: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x3ef0c: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3ef10: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3ef14: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3ef18: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0xef4
    c.bl(0x3e02c)?;
    // 0x3ef1c: ldr r0, [r5, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2064)?)?;
    // 0x3ef20: cmp r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // 0x3ef24: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x3ef28: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3ef2c: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3ef30: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3ef34: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x3ef38: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3ef3c: str r10, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,24)?)?;
    // 0x3ef40: movne r6, r10
    c.emit(arm32::mov_register(C::Ne,R::new(6)?,R::new(10)?))?;
    // 0x3ef44: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3ef48: str r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // 0x3ef4c: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3ef50: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3ef54: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0xf30
    c.bl(0x3e02c)?;
    // 0x3ef58: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // 0x3ef5c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x3ef60: bne 0x3ef94 <entry+0xfc>    @ imm = #0x2c
    c.b(C::Ne,0x3ef94)?;
    // 0x3ef64: ldr r0, [r5, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2064)?)?;
    // 0x3ef68: ldr r1, [r5, #0x3c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,960)?)?;
    // 0x3ef6c: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3ef70: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3ef74: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3ef78: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x3ef7c: sub r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3ef80: add r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3ef84: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3ef88: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3ef8c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3ef90: b 0x3f094 <entry+0x1fc>   @ imm = #0xfc
    c.b(C::Al,0x3f094)?;
    // 0x3ef94: mov r10, r9
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(9)?))?;
    // 0x3ef98: ldr r9, [r5, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(5)?,2064)?)?;
    // 0x3ef9c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3efa0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3efa4: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x3efa8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3efac: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3efb0: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3efb4: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3efb8: stmib sp, {r6, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(9)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3efbc: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0xf98
    c.bl(0x3e02c)?;
    // 0x3efc0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3efc4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3efc8: mov r2, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,9)?)?;
    // 0x3efcc: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3efd0: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3efd4: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3efd8: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3efdc: stmib sp, {r6, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(9)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3efe0: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0xfbc
    c.bl(0x3e02c)?;
    // 0x3efe4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3efe8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3efec: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x3eff0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3eff4: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3eff8: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3effc: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3f000: stmib sp, {r6, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(9)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f004: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0xfe0
    c.bl(0x3e02c)?;
    // 0x3f008: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f00c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f010: mov r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,11)?)?;
    // 0x3f014: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f018: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f01c: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f020: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3f024: stmib sp, {r6, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(9)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f028: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x1004
    c.bl(0x3e02c)?;
    // 0x3f02c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f030: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f034: mov r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x3f038: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f03c: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f040: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f044: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3f048: stmib sp, {r6, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(9)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f04c: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x1028
    c.bl(0x3e02c)?;
    // 0x3f050: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f054: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f058: mov r2, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,13)?)?;
    // 0x3f05c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f060: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f064: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f068: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3f06c: stmib sp, {r6, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(9)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f070: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x104c
    c.bl(0x3e02c)?;
    // 0x3f074: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f078: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f07c: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3f080: stmib sp, {r6, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit()|R::new(9)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f084: mov r9, r10
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(10)?))?;
    // 0x3f088: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f08c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f090: mov r2, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,14)?)?;
    // 0x3f094: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f098: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x1074
    c.bl(0x3e02c)?;
    // 0x3f09c: ldr r0, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,24)?)?;
    // 0x3f0a0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x3f0a4: bne 0x3f150 <entry+0x2b8>   @ imm = #0xa4
    c.b(C::Ne,0x3f150)?;
    // 0x3f0a8: ldr r0, [r5, #0x3c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,964)?)?;
    // 0x3f0ac: ldr r2, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,4)?)?;
    // 0x3f0b0: add r1, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3f0b4: ldr r0, [r5, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2064)?)?;
    // 0x3f0b8: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x3f0bc: bne 0x3f124 <entry+0x28c>   @ imm = #0x60
    c.b(C::Ne,0x3f124)?;
    // 0x3f0c0: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x3f0c4: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f0c8: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f0cc: mov r2, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,9)?)?;
    // 0x3f0d0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f0d4: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3f0d8: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x3f0dc: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3f0e0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f0e4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f0e8: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x10c4
    c.bl(0x3e02c)?;
    // 0x3f0ec: ldr r0, [r5, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2064)?)?;
    // 0x3f0f0: ldr r1, [r5, #0x3c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,964)?)?;
    // 0x3f0f4: ldr r2, [r5, #0x3c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,968)?)?;
    // 0x3f0f8: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f0fc: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f100: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3f104: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f108: add r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3f10c: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3f110: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f114: stmib sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f118: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f11c: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x3f120: b 0x3f14c <entry+0x2b4>   @ imm = #0x24
    c.b(C::Al,0x3f14c)?;
    // 0x3f124: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x3f128: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f12c: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f130: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x3f134: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x3f138: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x3f13c: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3f140: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f144: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f148: mov r2, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,9)?)?;
    // 0x3f14c: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x1128
    c.bl(0x3e02c)?;
    // 0x3f150: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // 0x3f154: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x3f158: cmpne r0, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x3f15c: bne 0x3f1cc <entry+0x334>   @ imm = #0x68
    c.b(C::Ne,0x3f1cc)?;
    // 0x3f160: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x3f164: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x3f168: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x3f16c: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x3f170: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f174: add r6, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3f178: ldr r0, [r9, #0x110]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,272)?)?;
    // 0x3f17c: ldr r1, [r5, #0x418]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1048)?)?;
    // 0x3f180: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f184: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f188: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3f18c: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f190: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f194: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f198: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x1174
    c.bl(0x3e02c)?;
    // 0x3f19c: ldr r0, [r5, #0x3b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,952)?)?;
    // 0x3f1a0: ldr r1, [r5, #0x418]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1048)?)?;
    // 0x3f1a4: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f1a8: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f1ac: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x3f1b0: str r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // 0x3f1b4: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x3f1b8: sub r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3f1bc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f1c0: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3f1c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f1c8: b 0x3f1f4 <entry+0x35c>   @ imm = #0x24
    c.b(C::Al,0x3f1f4)?;
    // 0x3f1cc: ldr r0, [r9, #0x110]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,272)?)?;
    // 0x3f1d0: ldr r1, [r5, #0x3b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,952)?)?;
    // 0x3f1d4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x3f1d8: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f1dc: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f1e0: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x3f1e4: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x3f1e8: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f1ec: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f1f0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f1f4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f1f8: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x11d4
    c.bl(0x3e02c)?;
    // 0x3f1fc: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x3f200: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x3f204: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x3f208)?;
    c.emit(HARDWARE_VALID_BITS)?;
    c.check(0x3f20c)
}

fn submit_codec_plane_descriptors(c:&mut Code)->Result<(),String>{
    c.check(0x3f20c)?;
    // 0x3f20c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3f210: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x3f214: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // 0x3f218: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x3f21c: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x3f220: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x3f224: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x3f228: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x3f22c: str r9, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,24)?)?;
    // 0x3f230: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x3f234: ldr r0, [r5, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,956)?)?;
    // 0x3f238: sub r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,2048)?)?;
    // 0x3f23c: add r0, r1, r0, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,2,false)?)?;
    // 0x3f240: ldr r1, [pc, #0x100]        @ 0x4b0 <entry+0x4b0>
    c.literal_cond(C::Al,R::R1,0x3f348)?;
    // 0x3f244: and r6, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3f248: str r6, [r5, #0x418]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,1048)?)?;
    // 0x3f24c: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // 0x3f250: ldr r7, [r8, #0x110]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(8)?,272)?)?;
    // 0x3f254: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x3f258: cmpne r0, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x3f25c: bne 0x3f2c0 <entry+0x428>   @ imm = #0x5c
    c.b(C::Ne,0x3f2c0)?;
    // 0x3f260: add r10, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,24)?)?;
    // 0x3f264: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f268: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f26c: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x3f270: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f274: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f278: str r10, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,20)?)?;
    // 0x3f27c: str r9, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    // 0x3f280: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x3f284: str r6, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // 0x3f288: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x1264
    c.bl(0x3e02c)?;
    // 0x3f28c: ldr r0, [r5, #0x3b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,952)?)?;
    // 0x3f290: ldr r1, [r5, #0x418]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1048)?)?;
    // 0x3f294: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f298: str r10, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,20)?)?;
    // 0x3f29c: str r9, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    // 0x3f2a0: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x3f2a4: sub r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3f2a8: add r1, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3f2ac: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x3f2b0: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3f2b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f2b8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f2bc: b 0x3f2f4 <entry+0x45c>   @ imm = #0x30
    c.b(C::Al,0x3f2f4)?;
    // 0x3f2c0: ldrb r1, [r5, #0x193]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,403)?)?;
    // 0x3f2c4: mov r0, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1879048192)?)?;
    // 0x3f2c8: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x3f2cc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3f2d0: add r1, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,24)?)?;
    // 0x3f2d4: ldreq r0, [r5, #0x3b8]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(5)?,952)?)?;
    // 0x3f2d8: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f2dc: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x3f2e0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3f2e4: stm sp, {r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3f2e8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f2ec: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3f2f0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f2f4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f2f8: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x12d4
    c.bl(0x3e02c)?;
    // 0x3f2fc: ldr r0, [r5, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,300)?)?;
    // 0x3f300: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3f304: beq 0x3f33c <entry+0x4a4>   @ imm = #0x30
    c.b(C::Eq,0x3f33c)?;
    // 0x3f308: ldr r0, [r8, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,296)?)?;
    // 0x3f30c: ldr r1, [r5, #0x3c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,960)?)?;
    // 0x3f310: add r2, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,24)?)?;
    // 0x3f314: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x3f318: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f31c: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x3f320: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x3f324: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x3f328: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x3f32c: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f330: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f334: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3f338: bl 0x3e02c <entry+0xfffffffffffff194> @ imm = #-0x1314
    c.bl(0x3e02c)?;
    // 0x3f33c: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x3f340: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x3f344: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x3f348)?;
    c.emit(HARDWARE_VALID_BITS)?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    transfer_packed_stream_descriptors(c)?;
    submit_codec_plane_descriptors(c)?;
    Ok(())
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn publishes_two_stream_classes_with_separate_data_masks(){
  let mut c=Code::new(START);emit(&mut c).unwrap();
  assert_eq!(c.at(),END);
  for a in [0x3f208,0x3f348]{
   let i=a-START;
   assert_eq!(u32::from_le_bytes(c.bytes[i..i+4].try_into().unwrap()),HARDWARE_VALID_BITS);
  }
 }
}
