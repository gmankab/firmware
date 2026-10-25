// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller register getters/setters, indexed firmware table lookup
//! and callback initialization. Original per-routine literal words are kept
//! separate from typed ARM A32 instruction encoding.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const LITERAL_25718:[u32;1]=[0xe00d2050];
const LITERAL_25730:[u32;1]=[0xe00d204c];
const LITERAL_25748:[u32;1]=[0xe00d204c];
const LITERAL_2575C:[u32;1]=[0xe00d204c];
const LITERAL_2576C:[u32;1]=[0xe00d2054];
const LITERAL_2577C:[u32;1]=[0xe00d2034];
const LITERAL_2578C:[u32;1]=[0xe00d2060];
const LITERAL_257A8:[u32;1]=[0xe00e1080];
const LITERAL_257BC:[u32;1]=[0x004ff000];
const LITERAL_25878:[u32;4]=[0x0010400c,0x000e8274,0x000e82a4,0x000e2788];
const LITERAL_258E0:[u32;1]=[0x0010400c];
const LITERAL_25950:[u32;1]=[0x0010400c];
const LITERAL_25960:[u32;1]=[0x001040a4];
const LITERAL_25970:[u32;1]=[0x001040a4];
const LITERAL_25980:[u32;1]=[0x001040a4];
const LITERAL_25990:[u32;1]=[0x001040a4];
const LITERAL_259A0:[u32;1]=[0x001040a4];
const LITERAL_259B0:[u32;1]=[0x001040a4];
const LITERAL_259C0:[u32;1]=[0x001040a4];
const LITERAL_259D0:[u32;1]=[0x001040a4];
const LITERAL_259EC:[u32;2]=[0x0010409c,0xe0008014];
const LITERAL_25A00:[u32;1]=[0xe0008024];
const LITERAL_25A1C:[u32;2]=[0x0010409c,0xe0008014];
const LITERAL_25A3C:[u32;2]=[0x001040a0,0xe0008018];
const LITERAL_25A50:[u32;1]=[0xe0008028];
const LITERAL_25A6C:[u32;2]=[0x001040a0,0xe0008018];
const LITERAL_25AC0:[u32;3]=[0xe0008008,0x0010409c,0x0010ceac];
const LITERAL_25B10:[u32;3]=[0xe0008008,0x001040a0,0x0010ceac];

/// Checked original controller function-group entry at `0x25700`.
fn small_controller_registers(c:&mut Code)->Result<(),String>{
    c.check(0x25700)?;
    // runtime 0x25700: ldr r0, [pc, #0x10]
    c.literal(R::R0,0x25718)?;
    // runtime 0x25704: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // runtime 0x25708: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // runtime 0x2570c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // runtime 0x25710: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // runtime 0x25714: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x25718)?;
    for data_word in LITERAL_25718{c.emit(data_word)?;}
    c.check(0x2571c)?;
    // runtime 0x2571c: ldr r0, [pc, #0xc]
    c.literal(R::R0,0x25730)?;
    // runtime 0x25720: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // runtime 0x25724: orr r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1)?)?;
    // runtime 0x25728: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // runtime 0x2572c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x25730)?;
    for data_word in LITERAL_25730{c.emit(data_word)?;}
    c.check(0x25734)?;
    // runtime 0x25734: ldr r0, [pc, #0xc]
    c.literal(R::R0,0x25748)?;
    // runtime 0x25738: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // runtime 0x2573c: orr r1, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2)?)?;
    // runtime 0x25740: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // runtime 0x25744: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x25748)?;
    for data_word in LITERAL_25748{c.emit(data_word)?;}
    c.check(0x2574c)?;
    // runtime 0x2574c: ldr r0, [pc, #0x8]
    c.literal(R::R0,0x2575c)?;
    // runtime 0x25750: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // runtime 0x25754: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // runtime 0x25758: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2575c)?;
    for data_word in LITERAL_2575C{c.emit(data_word)?;}
    c.check(0x25760)?;
    // runtime 0x25760: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x2576c)?;
    // runtime 0x25764: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // runtime 0x25768: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2576c)?;
    for data_word in LITERAL_2576C{c.emit(data_word)?;}
    c.check(0x25770)?;
    // runtime 0x25770: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x2577c)?;
    // runtime 0x25774: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // runtime 0x25778: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2577c)?;
    for data_word in LITERAL_2577C{c.emit(data_word)?;}
    c.check(0x25780)?;
    // runtime 0x25780: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x2578c)?;
    // runtime 0x25784: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // runtime 0x25788: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2578c)?;
    for data_word in LITERAL_2578C{c.emit(data_word)?;}
    c.check(0x25790)?;
    // runtime 0x25790: ldr r0, [pc, #0x10]
    c.literal(R::R0,0x257a8)?;
    // runtime 0x25794: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // runtime 0x25798: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // runtime 0x2579c: tst r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,3)?)?;
    // runtime 0x257a0: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // runtime 0x257a4: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x257a8)?;
    for data_word in LITERAL_257A8{c.emit(data_word)?;}
    c.check(0x257ac)?;
    // runtime 0x257ac: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // runtime 0x257b0: ldr r1, [pc, #0x4]
    c.literal(R::R1,0x257bc)?;
    // runtime 0x257b4: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // runtime 0x257b8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x257bc)?;
    for data_word in LITERAL_257BC{c.emit(data_word)?;}
    c.check(0x257c0)?;
    c.check(0x257c0)
}

/// Checked original controller function-group entry at `0x257c0`.
fn compare_firmware_revision(c:&mut Code)->Result<(),String>{
    c.check(0x257c0)?;
    // runtime 0x257c0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // runtime 0x257c4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // runtime 0x257c8: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // runtime 0x257cc: ldr r0, [pc, #0xa4]
    c.literal(R::R0,0x25878)?;
    // runtime 0x257d0: add r6, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,4)?)?;
    // runtime 0x257d4: bl 0x8e88 <mid+0xfffffffffffe7210>
    c.bl(0x8e88)?;
    // runtime 0x257d8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // runtime 0x257dc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // runtime 0x257e0: add r5, r4, #84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(4)?,84)?)?;
    // runtime 0x257e4: ldr r0, [r6, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,-4)?)?;
    // runtime 0x257e8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // runtime 0x257ec: mov r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,11)?)?;
    // runtime 0x257f0: bl 0x84210 <mid+0x62598>
    c.bl(0x84210)?;
    // runtime 0x257f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // runtime 0x257f8: beq 0x25810 <mid+0x3b98>
    c.b(C::Eq,0x25810)?;
    // runtime 0x257fc: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // runtime 0x25800: add r6, r6, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,12)?)?;
    // runtime 0x25804: cmp r7, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,10)?)?;
    // runtime 0x25808: blo 0x257e4 <mid+0x3b6c>
    c.b(C::Cc,0x257e4)?;
    // runtime 0x2580c: b 0x25868 <mid+0x3bf0>
    c.b(C::Al,0x25868)?;
    // runtime 0x25810: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // runtime 0x25814: ldr r1, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,100)?)?;
    // runtime 0x25818: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // runtime 0x2581c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // runtime 0x25820: strh r1, [r6]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(6)?,0)?)?;
    // runtime 0x25824: beq 0x2586c <mid+0x3bf4>
    c.b(C::Eq,0x2586c)?;
    // runtime 0x25828: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // runtime 0x2582c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // runtime 0x25830: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // runtime 0x25834: bl 0x84e30 <mid+0x631b8>
    c.bl(0x84e30)?;
    // runtime 0x25838: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // runtime 0x2583c: beq 0x2586c <mid+0x3bf4>
    c.b(C::Eq,0x2586c)?;
    // runtime 0x25840: ldr r0, [r6, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,-4)?)?;
    // runtime 0x25844: ldr r1, [pc, #0x30]
    c.literal(R::R1,0x2587c)?;
    // runtime 0x25848: ldr r2, [pc, #0x30]
    c.literal(R::R2,0x25880)?;
    // runtime 0x2584c: mov r3, #92
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,92)?)?;
    // runtime 0x25850: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // runtime 0x25854: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // runtime 0x25858: bl 0x2ec94 <mid+0xd01c>
    c.bl(0x2ec94)?;
    // runtime 0x2585c: ldr r1, [pc, #0x20]
    c.literal(R::R1,0x25884)?;
    // runtime 0x25860: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // runtime 0x25864: bl 0x2ec94 <mid+0xd01c>
    c.bl(0x2ec94)?;
    // runtime 0x25868: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // runtime 0x2586c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // runtime 0x25870: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // runtime 0x25874: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x25878)?;
    for data_word in LITERAL_25878{c.emit(data_word)?;}
    c.check(0x25888)?;
    c.check(0x25888)
}

/// Checked original controller function-group entry at `0x25888`.
fn lookup_controller_stream(c:&mut Code)->Result<(),String>{
    c.check(0x25888)?;
    // runtime 0x25888: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // runtime 0x2588c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // runtime 0x25890: ldr r0, [pc, #0x48]
    c.literal(R::R0,0x258e0)?;
    // runtime 0x25894: add r4, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,4)?)?;
    // runtime 0x25898: bl 0x8e88 <mid+0xfffffffffffe7210>
    c.bl(0x8e88)?;
    // runtime 0x2589c: ldr r1, [r0, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,100)?)?;
    // runtime 0x258a0: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // runtime 0x258a4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // runtime 0x258a8: orr r2, r2, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65280)?)?;
    // runtime 0x258ac: and r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // runtime 0x258b0: ldrh r2, [r4]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(4)?,0)?)?;
    // runtime 0x258b4: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // runtime 0x258b8: beq 0x258d0 <mid+0x3c58>
    c.b(C::Eq,0x258d0)?;
    // runtime 0x258bc: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // runtime 0x258c0: add r4, r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,12)?)?;
    // runtime 0x258c4: cmp r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,10)?)?;
    // runtime 0x258c8: blo 0x258b0 <mid+0x3c38>
    c.b(C::Cc,0x258b0)?;
    // runtime 0x258cc: b 0x258d8 <mid+0x3c60>
    c.b(C::Al,0x258d8)?;
    // runtime 0x258d0: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // runtime 0x258d4: bl 0x84ea4 <mid+0x6322c>
    c.bl(0x84ea4)?;
    // runtime 0x258d8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // runtime 0x258dc: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x258e0)?;
    for data_word in LITERAL_258E0{c.emit(data_word)?;}
    c.check(0x258e4)?;
    c.check(0x258e4)
}

/// Checked original controller function-group entry at `0x258e4`.
fn submit_controller_command(c:&mut Code)->Result<(),String>{
    c.check(0x258e4)?;
    // runtime 0x258e4: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // runtime 0x258e8: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // runtime 0x258ec: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // runtime 0x258f0: ldr r0, [pc, #0x58]
    c.literal(R::R0,0x25950)?;
    // runtime 0x258f4: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // runtime 0x258f8: add r6, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,4)?)?;
    // runtime 0x258fc: bl 0x8e88 <mid+0xfffffffffffe7210>
    c.bl(0x8e88)?;
    // runtime 0x25900: ldr r1, [r0, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,100)?)?;
    // runtime 0x25904: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // runtime 0x25908: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // runtime 0x2590c: orr r2, r2, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65280)?)?;
    // runtime 0x25910: and r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // runtime 0x25914: b 0x25928 <mid+0x3cb0>
    c.b(C::Al,0x25928)?;
    // runtime 0x25918: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // runtime 0x2591c: add r6, r6, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,12)?)?;
    // runtime 0x25920: cmp r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,10)?)?;
    // runtime 0x25924: pophs {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Cs,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // runtime 0x25928: ldrh r2, [r6]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(6)?,0)?)?;
    // runtime 0x2592c: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // runtime 0x25930: bne 0x25918 <mid+0x3ca0>
    c.b(C::Ne,0x25918)?;
    // runtime 0x25934: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // runtime 0x25938: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // runtime 0x2593c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // runtime 0x25940: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // runtime 0x25944: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // runtime 0x25948: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // runtime 0x2594c: b 0x84ed4 <mid+0x6325c>
    c.b(C::Al,0x84ed4)?;
    c.check(0x25950)?;
    for data_word in LITERAL_25950{c.emit(data_word)?;}
    c.check(0x25954)?;
    c.check(0x25954)
}

/// Checked original controller function-group entry at `0x25954`.
fn access_peripheral_registers(c:&mut Code)->Result<(),String>{
    c.check(0x25954)?;
    // runtime 0x25954: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x25960)?;
    // runtime 0x25958: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // runtime 0x2595c: b 0x2bf4 <mid+0xfffffffffffe0f7c>
    c.b(C::Al,0x2bf4)?;
    c.check(0x25960)?;
    for data_word in LITERAL_25960{c.emit(data_word)?;}
    c.check(0x25964)?;
    // runtime 0x25964: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x25970)?;
    // runtime 0x25968: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // runtime 0x2596c: b 0x2c04 <mid+0xfffffffffffe0f8c>
    c.b(C::Al,0x2c04)?;
    c.check(0x25970)?;
    for data_word in LITERAL_25970{c.emit(data_word)?;}
    c.check(0x25974)?;
    // runtime 0x25974: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x25980)?;
    // runtime 0x25978: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // runtime 0x2597c: b 0x2c34 <mid+0xfffffffffffe0fbc>
    c.b(C::Al,0x2c34)?;
    c.check(0x25980)?;
    for data_word in LITERAL_25980{c.emit(data_word)?;}
    c.check(0x25984)?;
    // runtime 0x25984: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x25990)?;
    // runtime 0x25988: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // runtime 0x2598c: b 0x2bf4 <mid+0xfffffffffffe0f7c>
    c.b(C::Al,0x2bf4)?;
    c.check(0x25990)?;
    for data_word in LITERAL_25990{c.emit(data_word)?;}
    c.check(0x25994)?;
    // runtime 0x25994: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x259a0)?;
    // runtime 0x25998: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // runtime 0x2599c: b 0x2c04 <mid+0xfffffffffffe0f8c>
    c.b(C::Al,0x2c04)?;
    c.check(0x259a0)?;
    for data_word in LITERAL_259A0{c.emit(data_word)?;}
    c.check(0x259a4)?;
    // runtime 0x259a4: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x259b0)?;
    // runtime 0x259a8: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // runtime 0x259ac: b 0x2c34 <mid+0xfffffffffffe0fbc>
    c.b(C::Al,0x2c34)?;
    c.check(0x259b0)?;
    for data_word in LITERAL_259B0{c.emit(data_word)?;}
    c.check(0x259b4)?;
    // runtime 0x259b4: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x259c0)?;
    // runtime 0x259b8: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // runtime 0x259bc: b 0x2bf4 <mid+0xfffffffffffe0f7c>
    c.b(C::Al,0x2bf4)?;
    c.check(0x259c0)?;
    for data_word in LITERAL_259C0{c.emit(data_word)?;}
    c.check(0x259c4)?;
    // runtime 0x259c4: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x259d0)?;
    // runtime 0x259c8: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // runtime 0x259cc: b 0x2c34 <mid+0xfffffffffffe0fbc>
    c.b(C::Al,0x2c34)?;
    c.check(0x259d0)?;
    for data_word in LITERAL_259D0{c.emit(data_word)?;}
    c.check(0x259d4)?;
    // runtime 0x259d4: ldr r1, [pc, #0x10]
    c.literal(R::R1,0x259ec)?;
    // runtime 0x259d8: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // runtime 0x259dc: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // runtime 0x259e0: ldr r1, [pc, #0x8]
    c.literal(R::R1,0x259f0)?;
    // runtime 0x259e4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // runtime 0x259e8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x259ec)?;
    for data_word in LITERAL_259EC{c.emit(data_word)?;}
    c.check(0x259f4)?;
    // runtime 0x259f4: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x25a00)?;
    // runtime 0x259f8: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // runtime 0x259fc: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x25a00)?;
    for data_word in LITERAL_25A00{c.emit(data_word)?;}
    c.check(0x25a04)?;
    // runtime 0x25a04: ldr r0, [pc, #0x10]
    c.literal(R::R0,0x25a1c)?;
    // runtime 0x25a08: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // runtime 0x25a0c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // runtime 0x25a10: ldr r0, [pc, #0x8]
    c.literal(R::R0,0x25a20)?;
    // runtime 0x25a14: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // runtime 0x25a18: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x25a1c)?;
    for data_word in LITERAL_25A1C{c.emit(data_word)?;}
    c.check(0x25a24)?;
    // runtime 0x25a24: ldr r1, [pc, #0x10]
    c.literal(R::R1,0x25a3c)?;
    // runtime 0x25a28: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // runtime 0x25a2c: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // runtime 0x25a30: ldr r1, [pc, #0x8]
    c.literal(R::R1,0x25a40)?;
    // runtime 0x25a34: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // runtime 0x25a38: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x25a3c)?;
    for data_word in LITERAL_25A3C{c.emit(data_word)?;}
    c.check(0x25a44)?;
    // runtime 0x25a44: ldr r0, [pc, #0x4]
    c.literal(R::R0,0x25a50)?;
    // runtime 0x25a48: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // runtime 0x25a4c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x25a50)?;
    for data_word in LITERAL_25A50{c.emit(data_word)?;}
    c.check(0x25a54)?;
    // runtime 0x25a54: ldr r0, [pc, #0x10]
    c.literal(R::R0,0x25a6c)?;
    // runtime 0x25a58: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // runtime 0x25a5c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // runtime 0x25a60: ldr r0, [pc, #0x8]
    c.literal(R::R0,0x25a70)?;
    // runtime 0x25a64: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // runtime 0x25a68: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x25a6c)?;
    for data_word in LITERAL_25A6C{c.emit(data_word)?;}
    c.check(0x25a74)?;
    c.check(0x25a74)
}

/// Checked original controller function-group entry at `0x25a74`.
fn reset_peripheral_state(c:&mut Code)->Result<(),String>{
    c.check(0x25a74)?;
    // runtime 0x25a74: ldr r1, [pc, #0x44]
    c.literal(R::R1,0x25ac0)?;
    // runtime 0x25a78: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // runtime 0x25a7c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // runtime 0x25a80: str r0, [r1, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,52)?)?;
    // runtime 0x25a84: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // runtime 0x25a88: ldr r1, [pc, #0x34]
    c.literal(R::R1,0x25ac4)?;
    // runtime 0x25a8c: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // runtime 0x25a90: cmn r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R2,1)?)?;
    // runtime 0x25a94: streq r0, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::R1,0)?)?;
    // runtime 0x25a98: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // runtime 0x25a9c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // runtime 0x25aa0: ldr r4, [pc, #0x20]
    c.literal(R::new(4)?,0x25ac8)?;
    // runtime 0x25aa4: mov r1, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33554432)?)?;
    // runtime 0x25aa8: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,76)?)?;
    // runtime 0x25aac: bl 0x2d20 <mid+0xfffffffffffe10a8>
    c.bl(0x2d20)?;
    // runtime 0x25ab0: ldr r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // runtime 0x25ab4: add r0, r0, #156
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,156)?)?;
    // runtime 0x25ab8: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // runtime 0x25abc: b 0x292c <mid+0xfffffffffffe0cb4>
    c.b(C::Al,0x292c)?;
    c.check(0x25ac0)?;
    for data_word in LITERAL_25AC0{c.emit(data_word)?;}
    c.check(0x25acc)?;
    c.check(0x25acc)
}

/// Checked original controller function-group entry at `0x25acc`.
fn initialize_peripheral_state(c:&mut Code)->Result<(),String>{
    c.check(0x25acc)?;
    // runtime 0x25acc: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // runtime 0x25ad0: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // runtime 0x25ad4: ldr r0, [pc, #0x34]
    c.literal(R::R0,0x25b10)?;
    // runtime 0x25ad8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // runtime 0x25adc: ldr r4, [pc, #0x34]
    c.literal(R::new(4)?,0x25b18)?;
    // runtime 0x25ae0: str r1, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,52)?)?;
    // runtime 0x25ae4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // runtime 0x25ae8: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // runtime 0x25aec: ldr r0, [pc, #0x20]
    c.literal(R::R0,0x25b14)?;
    // runtime 0x25af0: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // runtime 0x25af4: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,76)?)?;
    // runtime 0x25af8: mov r1, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33554432)?)?;
    // runtime 0x25afc: bl 0x2d20 <mid+0xfffffffffffe10a8>
    c.bl(0x2d20)?;
    // runtime 0x25b00: ldr r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // runtime 0x25b04: add r0, r0, #164
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,164)?)?;
    // runtime 0x25b08: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // runtime 0x25b0c: b 0x292c <mid+0xfffffffffffe0cb4>
    c.b(C::Al,0x292c)?;
    c.check(0x25b10)?;
    for data_word in LITERAL_25B10{c.emit(data_word)?;}
    c.check(0x25b1c)?;
    c.check(0x25b1c)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    small_controller_registers(c)?;
    compare_firmware_revision(c)?;
    lookup_controller_stream(c)?;
    submit_controller_command(c)?;
    access_peripheral_registers(c)?;
    reset_peripheral_state(c)?;
    initialize_peripheral_state(c)?;
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn helper_groups_have_exact_size(){
        let mut c=Code::new(0x25700);
        emit(&mut c).unwrap();
        assert_eq!(c.at(),0x25b1c);
        assert_eq!(c.bytes.len(),0x41c);
    }
}
