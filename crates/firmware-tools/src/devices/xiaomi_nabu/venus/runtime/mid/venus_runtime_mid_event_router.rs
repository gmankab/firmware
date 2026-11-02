// SPDX-License-Identifier: GPL-3.0-only
//! Controller event processing and two bounded indirect branch tables.
//! The new executable is byte-exact A32 re-encoding, but its entire handler
//! continues past END; target words are data, not executable instructions.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START:usize=0x2b474;
const END:usize=0x2b774;

/// Validate codec packet types and select handler from two bounded tables.
fn begin_codec_event_dispatch(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2b474: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2b478: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x2b47c: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // 0x2b480: sub sp, sp, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,2048)?)?;
    // 0x2b484: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2b488: ldr r1, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,120)?)?;
    // 0x2b48c: bl 0x2d19c <entry+0x2d9c>  @ imm = #0x1d08
    c.bl(0x2d19c)?;
    // 0x2b490: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x2b494: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x2b498: bne 0x2b4ec <entry+0x10ec>  @ imm = #0x4c
    c.b(C::Ne,0x2b4ec)?;
    // 0x2b49c: ldr r9, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,120)?)?;
    // 0x2b4a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2b4a4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2b4a8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2b4ac: bl 0x3084 <entry+0xfffffffffffd8c84> @ imm = #-0x28430
    c.bl(0x3084)?;
    // 0x2b4b0: ldr r7, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,4)?)?;
    // 0x2b4b4: mov r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,2)?)?;
    // 0x2b4b8: orr r10, r10, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,65536)?)?;
    // 0x2b4bc: cmp r7, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x2b4c0: bne 0x2b4cc <entry+0x10cc>  @ imm = #0x4
    c.b(C::Ne,0x2b4cc)?;
    // 0x2b4c4: bl 0x2b304 <entry+0xf04>   @ imm = #-0x1c8
    c.bl(0x2b304)?;
    // 0x2b4c8: b 0x2b4e8 <entry+0x10e8>  @ imm = #0x18
    c.b(C::Al,0x2b4e8)?;
    // 0x2b4cc: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // 0x2b4d0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2b4d4: bne 0x2b4f8 <entry+0x10f8>  @ imm = #0x1c
    c.b(C::Ne,0x2b4f8)?;
    // 0x2b4d8: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // 0x2b4dc: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2b4e0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2b4e4: bl 0x33bf0 <entry+0x97f0>  @ imm = #0x8704
    c.bl(0x33bf0)?;
    // 0x2b4e8: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2b4ec: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x2b4f0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x2b4f4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2b4f8: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x2b4fc: mov r6, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,6)?)?;
    // 0x2b500: orr r8, r8, #2162688
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2162688)?)?;
    // 0x2b504: orr r6, r6, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,131072)?)?;
    // 0x2b508: cmp r7, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x2b50c: movne r0, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b510: orrne r0, r0, #65536
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,65536)?)?;
    // 0x2b514: cmpne r7, r0
    c.emit(arm32::data_register(C::Ne,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2b518: beq 0x2b558 <entry+0x1158>  @ imm = #0x38
    c.b(C::Eq,0x2b558)?;
    // 0x2b51c: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x2b520: orr r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65536)?)?;
    // 0x2b524: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2b528: bne 0x2b580 <entry+0x1180>  @ imm = #0x50
    c.b(C::Ne,0x2b580)?;
    // 0x2b52c: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x2b530: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x2b534: bne 0x2b77c <entry+0x137c>  @ imm = #0x240
    c.b(C::Ne,0x2b77c)?;
    // 0x2b538: ldr r0, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,112)?)?;
    // 0x2b53c: ldr r1, [r0, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,188)?)?;
    // 0x2b540: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x2b544: str r1, [r0, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,188)?)?;
    // 0x2b548: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2b54c: ldr r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,20)?)?;
    // 0x2b550: bl 0x1c7e8 <entry+0xffffffffffff23e8> @ imm = #-0xed70
    c.bl(0x1c7e8)?;
    // 0x2b554: b 0x2b4e8 <entry+0x10e8>  @ imm = #-0x74
    c.b(C::Al,0x2b4e8)?;
    // 0x2b558: ldr r5, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,112)?)?;
    // 0x2b55c: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,8)?)?;
    // 0x2b560: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2b564: add r1, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x2b568: ldr r1, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,64)?)?;
    // 0x2b56c: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x2b570: beq 0x2b610 <entry+0x1210>  @ imm = #0x98
    c.b(C::Eq,0x2b610)?;
    // 0x2b574: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x2b578: cmp r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x2b57c: bls 0x2b564 <entry+0x1164>  @ imm = #-0x20
    c.b(C::Ls,0x2b564)?;
    // 0x2b580: ldr r5, [pc, #0xfe0]        @ 0x2168 <entry+0x2168>
    c.literal(R::new(5)?,0x2c568)?;
    // 0x2b584: cmp r7, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x2b588: bls 0x2b5d4 <entry+0x11d4>  @ imm = #0x44
    c.b(C::Ls,0x2b5d4)?;
    // 0x2b58c: ldr r1, [pc, #0xfd8]        @ 0x216c <entry+0x216c>
    c.literal(R::R1,0x2c56c)?;
    // 0x2b590: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2b594: bl 0x2ec94 <entry+0x4894>  @ imm = #0x36f8
    c.bl(0x2ec94)?;
    // 0x2b598: ldr r1, [pc, #0xfd0]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2b59c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2b5a0: bl 0x2ec94 <entry+0x4894>  @ imm = #0x36ec
    c.bl(0x2ec94)?;
    // 0x2b5a4: ldr r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,20)?)?;
    // 0x2b5a8: ldr r4, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,112)?)?;
    // 0x2b5ac: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2b5b0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2b5b4: bl 0x1c7e8 <entry+0xffffffffffff23e8> @ imm = #-0xedd4
    c.bl(0x1c7e8)?;
    // 0x2b5b8: add r0, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,4096)?)?;
    // 0x2b5bc: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2b5c0: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2b5c4: bls 0x2b4ec <entry+0x10ec>  @ imm = #-0xe0
    c.b(C::Ls,0x2b4ec)?;
    // 0x2b5c8: bl 0x26754 <entry+0xffffffffffffc354> @ imm = #-0x4e7c
    c.bl(0x26754)?;
    // 0x2b5cc: str r0, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,216)?)?;
    // 0x2b5d0: b 0x2b4ec <entry+0x10ec>  @ imm = #-0xec
    c.b(C::Al,0x2b4ec)?;
    // 0x2b5d4: cmp r7, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,69632)?)?;
    // 0x2b5d8: bls 0x2b6ac <entry+0x12ac>  @ imm = #0xcc
    c.b(C::Ls,0x2b6ac)?;
    // 0x2b5dc: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,8)?)?;
    // 0x2b5e0: add r1, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,12)?)?;
    // 0x2b5e4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2b5e8: bl 0x2d2b4 <entry+0x2eb4>  @ imm = #0x1cc4
    c.bl(0x2d2b4)?;
    // 0x2b5ec: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2b5f0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2b5f4: beq 0x2b4ec <entry+0x10ec>  @ imm = #-0x110
    c.b(C::Eq,0x2b4ec)?;
    // 0x2b5f8: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // 0x2b5fc: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // 0x2b600: blo 0x2b64c <entry+0x124c>  @ imm = #0x44
    c.b(C::Cc,0x2b64c)?;
    // 0x2b604: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x2b608: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // 0x2b60c: b 0x2c640 <entry+0x2240>  @ imm = #0x102c
    c.b(C::Al,0x2c640)?;
    // 0x2b610: ldr r0, [r5, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,188)?)?;
    // 0x2b614: ldr r1, [pc, #0xf8c]        @ 0x21a8 <entry+0x21a8>
    c.literal(R::R1,0x2c5a8)?;
    // 0x2b618: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2b61c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x2b620: str r0, [r5, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,188)?)?;
    // 0x2b624: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b628: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3664
    c.bl(0x2ec94)?;
    // 0x2b62c: ldr r1, [pc, #0xf3c]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2b630: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b634: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3658
    c.bl(0x2ec94)?;
    // 0x2b638: ldr r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,20)?)?;
    // 0x2b63c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2b640: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2b644: bl 0x1c7e8 <entry+0xffffffffffff23e8> @ imm = #-0xee64
    c.bl(0x1c7e8)?;
    // 0x2b648: b 0x2b4ec <entry+0x10ec>  @ imm = #-0x164
    c.b(C::Al,0x2b4ec)?;
    // 0x2b64c: add r6, r4, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(4)?,R::R2,Shift::Lsl,2,false)?)?;
    // 0x2b650: ldr r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // 0x2b654: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x2b658: ldr r0, [r6, #0x30]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(6)?,48)?)?;
    // 0x2b65c: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // 0x2b660: bl 0x3084 <entry+0xfffffffffffd8c84> @ imm = #-0x285e4
    c.bl(0x3084)?;
    // 0x2b664: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x2b668: ldr r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,24)?)?;
    // 0x2b66c: add r0, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x2b670: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // 0x2b674: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2b678: beq 0x2b6a4 <entry+0x12a4>  @ imm = #0x24
    c.b(C::Eq,0x2b6a4)?;
    // 0x2b67c: ldr r2, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,0)?)?;
    // 0x2b680: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2b684: cmp r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // 0x2b688: movhs r2, #32
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x2b68c: bl 0x471c <entry+0xfffffffffffda31c> @ imm = #-0x26f78
    c.bl(0x471c)?;
    // 0x2b690: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x2b694: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2b698: ldr r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,24)?)?;
    // 0x2b69c: bic r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2b6a0: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x2b6a4: mov r6, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,6)?)?;
    // 0x2b6a8: orr r6, r6, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,131072)?)?;
    // 0x2b6ac: ldr r0, [pc, #0xff4]        @ 0x22a8 <entry+0x22a8>
    c.literal(R::R0,0x2c6a8)?;
    // 0x2b6b0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2b6b4: orr r2, r2, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,135168)?)?;
    // 0x2b6b8: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2b6bc: ble 0x2b728 <entry+0x1328>  @ imm = #0x64
    c.b(C::Le,0x2b728)?;
    // 0x2b6c0: cmp r7, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x2b6c4: ble 0x2b924 <entry+0x1524>  @ imm = #0x258
    c.b(C::Le,0x2b924)?;
    // 0x2b6c8: ldr r0, [pc, #0xfdc]        @ 0x22ac <entry+0x22ac>
    c.literal(R::R0,0x2c6ac)?;
    // 0x2b6cc: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2b6d0: cmp r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,14)?)?;
    // 0x2b6d4: bhi 0x2c084 <entry+0x1c84>  @ imm = #0x9a8
    c.b(C::Hi,0x2c084)?;
    // 0x2b6d8: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2b6dc: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x2b6e0: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2b6e4)?;
    const CODEC_MESSAGE_DISPATCH:[u32;15]=[
        0x0002b720,0x0002c6dc,0x0002c710,0x0002c73c,0x0002c770,
        0x0002c7a0,0x0002c7cc,0x0002c800,0x0002c830,0x0002c860,
        0x0002c0f4,0x0002c88c,0x0002c8bc,0x0002c8e8,0x0002c91c,
    ];
    for dest in CODEC_MESSAGE_DISPATCH {c.emit(dest)?;}
    c.check(0x2b720)?;
    // 0x2b720: ldr r1, [pc, #0xfe0]        @ 0x2308 <entry+0x2308>
    c.literal(R::R1,0x2c708)?;
    // 0x2b724: b 0x2c5b0 <entry+0x21b0>  @ imm = #0xe84
    c.b(C::Al,0x2c5b0)?;
    // 0x2b728: ldr r0, [pc, #0xfdc]        @ 0x230c <entry+0x230c>
    c.literal(R::R0,0x2c70c)?;
    // 0x2b72c: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2b730: bgt 0x2b970 <entry+0x1570>  @ imm = #0x238
    c.b(C::Gt,0x2b970)?;
    // 0x2b734: sub r0, r7, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,65536)?)?;
    // 0x2b738: cmp r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,9)?)?;
    // 0x2b73c: bhi 0x2bed4 <entry+0x1ad4>  @ imm = #0x790
    c.b(C::Hi,0x2bed4)?;
    // 0x2b740: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2b744: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x2b748: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2b74c)?;
    const CODEC_REQUEST_DISPATCH:[u32;10]=[
        0x0002b774,0x0002c19c,0x0002c224,0x0002c270,0x0002c2cc,
        0x0002c314,0x0002c33c,0x0002b77c,0x0002c428,0x0002c574,
    ];
    for dest in CODEC_REQUEST_DISPATCH {c.emit(dest)?;}
    c.check(0x2b774)?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    begin_codec_event_dispatch(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn event_dispatch_tables_have_exact_targets(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|vma:usize|u32::from_le_bytes(c.bytes[vma-START..vma-START+4].try_into().unwrap());
        assert_eq!(word(0x2b6e4),0x0002b720);
        assert_eq!(word(0x2b71c),0x0002c91c);
        assert_eq!(word(0x2b74c),0x0002b774);
        assert_eq!(word(0x2b770),0x0002c574);
    }
}
