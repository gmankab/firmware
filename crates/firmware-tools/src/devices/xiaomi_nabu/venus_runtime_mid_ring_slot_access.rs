// SPDX-License-Identifier: GPL-3.0-only
//! Reference-count and stream-buffer slot helpers, plus a fast ring-buffer
//! completion predicate. Function boundaries and the six embedded literal
//! address words are explicit, while instructions are re-encoded as A32.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2f394;
const END:usize=0x2f5b0;
fn update_buffer_slots(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2f394: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2f398: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2f39c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2f3a0: beq 0x2f3b8 <entry+0x1db8>  @ imm = #0x10
    c.b(C::Eq,0x2f3b8)?;
    // 0x2f3a4: ldr r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,20)?)?;
    // 0x2f3a8: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x2f3ac: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x2f3b0: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,20)?)?;
    // 0x2f3b4: b 0x2f3d0 <entry+0x1dd0>  @ imm = #0x14
    c.b(C::Al,0x2f3d0)?;
    // 0x2f3b8: ldr r2, [pc, #0x18]         @ 0x1dd8 <entry+0x1dd8>
    c.literal(R::R2,0x2f3d8)?;
    // 0x2f3bc: mov r1, #149
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,149)?)?;
    // 0x2f3c0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f3c4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x2f3c8: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x2f3cc: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2dc24
    c.bl(0x17b0)?;
    // 0x2f3d0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2f3d4: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2f3d8)?;
    const RING_SLOT_DATA_2F3D8:[u32;1]=[0x000ead29];
    for word in RING_SLOT_DATA_2F3D8 {c.emit(word)?;}
    c.check(0x2f3dc)?;
    // 0x2f3dc: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2f3e0: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2f3e4: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x2f3e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2f3ec: beq 0x2f3f8 <entry+0x1df8>  @ imm = #0x4
    c.b(C::Eq,0x2f3f8)?;
    // 0x2f3f0: bl 0x25770 <entry+0xffffffffffff8170> @ imm = #-0x9c88
    c.bl(0x25770)?;
    // 0x2f3f4: b 0x2f3fc <entry+0x1dfc>  @ imm = #0x0
    c.b(C::Al,0x2f3fc)?;
    // 0x2f3f8: bl 0x25760 <entry+0xffffffffffff8160> @ imm = #-0x9ca0
    c.bl(0x25760)?;
    // 0x2f3fc: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // 0x2f400: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f404: rsb r1, r4, r4, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R1,R::new(4)?,R::new(4)?,Shift::Lsl,3,false)?)?;
    // 0x2f408: add r0, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x2f40c: ldr r0, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,36)?)?;
    // 0x2f410: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2f414: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f418: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2f41c: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2f420: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2f424: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x2f428: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2f42c: beq 0x2f438 <entry+0x1e38>  @ imm = #0x4
    c.b(C::Eq,0x2f438)?;
    // 0x2f430: bl 0x25770 <entry+0xffffffffffff8170> @ imm = #-0x9cc8
    c.bl(0x25770)?;
    // 0x2f434: b 0x2f43c <entry+0x1e3c>  @ imm = #0x0
    c.b(C::Al,0x2f43c)?;
    // 0x2f438: bl 0x25760 <entry+0xffffffffffff8160> @ imm = #-0x9ce0
    c.bl(0x25760)?;
    // 0x2f43c: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // 0x2f440: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f444: rsb r1, r4, r4, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R1,R::new(4)?,R::new(4)?,Shift::Lsl,3,false)?)?;
    // 0x2f448: add r0, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x2f44c: ldr r0, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,32)?)?;
    // 0x2f450: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2f454: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f458: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2f45c: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2f460: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x2f464: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x2f468: mov r8, r2
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R2))?;
    // 0x2f46c: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x2f470: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x2f474: beq 0x2f480 <entry+0x1e80>  @ imm = #0x4
    c.b(C::Eq,0x2f480)?;
    // 0x2f478: bl 0x25770 <entry+0xffffffffffff8170> @ imm = #-0x9d10
    c.bl(0x25770)?;
    // 0x2f47c: b 0x2f484 <entry+0x1e84>  @ imm = #0x0
    c.b(C::Al,0x2f484)?;
    // 0x2f480: bl 0x25760 <entry+0xffffffffffff8160> @ imm = #-0x9d28
    c.bl(0x25760)?;
    // 0x2f484: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x2f488: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x2f48c: beq 0x2f4dc <entry+0x1edc>  @ imm = #0x48
    c.b(C::Eq,0x2f4dc)?;
    // 0x2f490: ldr r0, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,16)?)?;
    // 0x2f494: ldr r2, [pc, #0xdc]         @ 0x1f78 <entry+0x1f78>
    c.literal(R::R2,0x2f578)?;
    // 0x2f498: ldr r5, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,8)?)?;
    // 0x2f49c: mov r1, #250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,250)?)?;
    // 0x2f4a0: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2f4a4: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x2f4a8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2f4ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f4b0: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f4b4: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2dd0c
    c.bl(0x17b0)?;
    // 0x2f4b8: add r0, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x2f4bc: ldrb r1, [r0, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,8)?)?;
    // 0x2f4c0: cmp r1, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x2f4c4: beq 0x2f4e4 <entry+0x1ee4>  @ imm = #0x18
    c.b(C::Eq,0x2f4e4)?;
    // 0x2f4c8: ldr r1, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,16)?)?;
    // 0x2f4cc: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x2f4d0: add r0, r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,56)?)?;
    // 0x2f4d4: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f4d8: blo 0x2f4bc <entry+0x1ebc>  @ imm = #-0x24
    c.b(C::Cc,0x2f4bc)?;
    // 0x2f4dc: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x2f4e0: b 0x2f570 <entry+0x1f70>  @ imm = #0x88
    c.b(C::Al,0x2f570)?;
    // 0x2f4e4: ldr r2, [pc, #0x90]         @ 0x1f7c <entry+0x1f7c>
    c.literal(R::R2,0x2f57c)?;
    // 0x2f4e8: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x2f4ec: ldr r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,4)?)?;
    // 0x2f4f0: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x2f4f4: ldr r3, [r2, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,136)?)?;
    // 0x2f4f8: cmp r3, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f4fc: cmpls r4, #2
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::new(4)?,2)?)?;
    // 0x2f500: bls 0x2f540 <entry+0x1f40>  @ imm = #0x38
    c.b(C::Ls,0x2f540)?;
    // 0x2f504: ldr r1, [pc, #0x74]         @ 0x1f80 <entry+0x1f80>
    c.literal(R::R1,0x2f580)?;
    // 0x2f508: ldr r2, [pc, #0x74]         @ 0x1f84 <entry+0x1f84>
    c.literal(R::R2,0x2f584)?;
    // 0x2f50c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f510: mov r3, #528
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,528)?)?;
    // 0x2f514: bl 0x2ec94 <entry+0x1694>  @ imm = #-0x888
    c.bl(0x2ec94)?;
    // 0x2f518: ldr r1, [pc, #0x68]         @ 0x1f88 <entry+0x1f88>
    c.literal(R::R1,0x2f588)?;
    // 0x2f51c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f520: bl 0x2ec94 <entry+0x1694>  @ imm = #-0x894
    c.bl(0x2ec94)?;
    // 0x2f524: ldr r2, [pc, #0x4c]         @ 0x1f78 <entry+0x1f78>
    c.literal(R::R2,0x2f578)?;
    // 0x2f528: mov r1, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,17)?)?;
    // 0x2f52c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f530: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x2f534: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2f538: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2dd90
    c.bl(0x17b0)?;
    // 0x2f53c: b 0x2f570 <entry+0x1f70>  @ imm = #0x2c
    c.b(C::Al,0x2f570)?;
    // 0x2f540: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // 0x2f544: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f548: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f54c: blo 0x2f504 <entry+0x1f04>  @ imm = #-0x50
    c.b(C::Cc,0x2f504)?;
    // 0x2f550: cmp r0, #-536870911
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3758096385)?)?;
    // 0x2f554: bhs 0x2f504 <entry+0x1f04>  @ imm = #-0x58
    c.b(C::Cs,0x2f504)?;
    // 0x2f558: add r3, r9, r9, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(9)?,R::new(9)?,Shift::Lsl,1,false)?)?;
    // 0x2f55c: add r2, r2, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x2f560: add r2, r2, r4, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(4)?,Shift::Lsl,3,false)?)?;
    // 0x2f564: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x2f568: str r1, [r2, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,144)?)?;
    // 0x2f56c: str r0, [r2, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,148)?)?;
    // 0x2f570: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2f574: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2f578)?;
    const RING_SLOT_DATA_2F578:[u32;5]=[0x000ead29,0x0010ceac,0x000eae34,0x000eae5f,0x000e2788];
    for word in RING_SLOT_DATA_2F578 {c.emit(word)?;}
    c.check(0x2f58c)?;
    // 0x2f58c: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x2f590: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f594: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2f598: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x2f59c: ldr r2, [r1, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,48)?)?;
    // 0x2f5a0: ldr r1, [r1, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,52)?)?;
    // 0x2f5a4: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f5a8: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f5ac: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;update_buffer_slots(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn ring_slot_literal_pool_and_count_are_exact(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let data=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(data(0x2f3d8),0x000ead29);
        assert_eq!(data(0x2f578),0x000ead29);
        assert_eq!(data(0x2f588),0x000e2788);
    }
}
