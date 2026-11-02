// SPDX-License-Identifier: GPL-3.0-only
//! Ten codec event case paths. Each path logs category flags and routes a
//! populated descriptor to its type-specific platform handler. The small
//! literal pools between paths remain structured device metadata, not ARM32
//! executable opcodes.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::Code;
const START:usize=0x2c5ac;
const END:usize=0x2c950;
fn handle_codec_dispatch_cases(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2c5ac: ldr r1, [pc, #0xb6c]        @ 0x1520 <entry+0x1520>
    c.literal(R::R1,0x2d120)?;
    // 0x2c5b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c5b4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x26d8
    c.bl(0x2ec94)?;
    // 0x2c5b8: ldr r1, [pc, #0xae0]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c5bc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c5c0: bl 0x2ec94 <entry+0x3094>  @ imm = #0x26cc
    c.bl(0x2ec94)?;
    // 0x2c5c4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2c5c8: orr r2, r2, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,135168)?)?;
    // 0x2c5cc: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x2c5d0: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x2c5d4: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x2c5d8: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2c5dc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2c5e0: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x2c5e4: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x2c5e8: str r6, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,28)?)?;
    // 0x2c5ec: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x2c5f0: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2c5f4: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2c5f8: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2c5fc: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2c600: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2c604: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x2c608: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2c60c: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2c610: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x2480
    c.bl(0x2a198)?;
    // 0x2c614: ldr r1, [pc, #0xb7c]        @ 0x1598 <entry+0x1598>
    c.literal(R::R1,0x2d198)?;
    // 0x2c618: ldr r2, [pc, #0xb38]        @ 0x1558 <entry+0x1558>
    c.literal(R::R2,0x2d158)?;
    // 0x2c61c: mov r3, #146
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,146)?)?;
    // 0x2c620: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2c624: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x2c628: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2664
    c.bl(0x2ec94)?;
    // 0x2c62c: ldr r1, [pc, #0xa6c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c630: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2c634: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2658
    c.bl(0x2ec94)?;
    // 0x2c638: mov r1, #147
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,147)?)?;
    // 0x2c63c: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x2c640: ldr r2, [pc, #0xadc]        @ 0x1524 <entry+0x1524>
    c.literal(R::R2,0x2d124)?;
    // 0x2c644: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2c648: bl 0x17b0 <entry+0xfffffffffffd5bb0> @ imm = #-0x2aea0
    c.bl(0x17b0)?;
    // 0x2c64c: b 0x2b4ec <entry+0xfffffffffffff8ec> @ imm = #-0x1168
    c.b(C::Al,0x2b4ec)?;
    // 0x2c650: ldr r1, [pc, #0xaa4]        @ 0x14fc <entry+0x14fc>
    c.literal(R::R1,0x2d0fc)?;
    // 0x2c654: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c658: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2634
    c.bl(0x2ec94)?;
    // 0x2c65c: ldr r1, [pc, #0xa3c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c660: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c664: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2628
    c.bl(0x2ec94)?;
    // 0x2c668: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c66c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c670: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c674: bl 0x805e8 <entry+0x549e8> @ imm = #0x53f6c
    c.bl(0x805e8)?;
    // 0x2c678: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1198
    c.b(C::Al,0x2b4e8)?;
    // 0x2c67c: ldr r1, [pc, #0xa74]        @ 0x14f8 <entry+0x14f8>
    c.literal(R::R1,0x2d0f8)?;
    // 0x2c680: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c684: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2608
    c.bl(0x2ec94)?;
    // 0x2c688: ldr r1, [pc, #0xa10]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c68c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c690: bl 0x2ec94 <entry+0x3094>  @ imm = #0x25fc
    c.bl(0x2ec94)?;
    // 0x2c694: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c698: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c69c: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c6a0: bl 0x80638 <entry+0x54a38> @ imm = #0x53f90
    c.bl(0x80638)?;
    // 0x2c6a4: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x11c4
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c6a8)?;
    const CODEC_CASE_ADDRESSES_2C6A8:[u32;2]=[0x00110fff,0xffdef000];
    for word in CODEC_CASE_ADDRESSES_2C6A8 {c.emit(word)?;}
    c.check(0x2c6b0)?;
    // 0x2c6b0: ldr r1, [pc, #0xa3c]        @ 0x14f4 <entry+0x14f4>
    c.literal(R::R1,0x2d0f4)?;
    // 0x2c6b4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c6b8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x25d4
    c.bl(0x2ec94)?;
    // 0x2c6bc: ldr r1, [pc, #0x9dc]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c6c0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c6c4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x25c8
    c.bl(0x2ec94)?;
    // 0x2c6c8: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c6cc: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c6d0: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c6d4: bl 0x805fc <entry+0x549fc> @ imm = #0x53f20
    c.bl(0x805fc)?;
    // 0x2c6d8: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x11f8
    c.b(C::Al,0x2b4e8)?;
    // 0x2c6dc: ldr r1, [pc, #0xa0c]        @ 0x14f0 <entry+0x14f0>
    c.literal(R::R1,0x2d0f0)?;
    // 0x2c6e0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c6e4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x25a8
    c.bl(0x2ec94)?;
    // 0x2c6e8: ldr r1, [pc, #0x9b0]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c6ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c6f0: bl 0x2ec94 <entry+0x3094>  @ imm = #0x259c
    c.bl(0x2ec94)?;
    // 0x2c6f4: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c6f8: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c6fc: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c700: bl 0x8055c <entry+0x5495c> @ imm = #0x53e54
    c.bl(0x8055c)?;
    // 0x2c704: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1224
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c708)?;
    const CODEC_CASE_ADDRESSES_2C708:[u32;2]=[0x000e9e4e,0x00010101];
    for word in CODEC_CASE_ADDRESSES_2C708 {c.emit(word)?;}
    c.check(0x2c710)?;
    // 0x2c710: ldr r1, [pc, #0x9d4]        @ 0x14ec <entry+0x14ec>
    c.literal(R::R1,0x2d0ec)?;
    // 0x2c714: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c718: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2574
    c.bl(0x2ec94)?;
    // 0x2c71c: ldr r1, [pc, #0x97c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c720: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c724: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2568
    c.bl(0x2ec94)?;
    // 0x2c728: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c72c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c730: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c734: bl 0x80570 <entry+0x54970> @ imm = #0x53e34
    c.bl(0x80570)?;
    // 0x2c738: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1258
    c.b(C::Al,0x2b4e8)?;
    // 0x2c73c: ldr r1, [pc, #0x9a4]        @ 0x14e8 <entry+0x14e8>
    c.literal(R::R1,0x2d0e8)?;
    // 0x2c740: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c744: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2548
    c.bl(0x2ec94)?;
    // 0x2c748: ldr r1, [pc, #0x950]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c74c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c750: bl 0x2ec94 <entry+0x3094>  @ imm = #0x253c
    c.bl(0x2ec94)?;
    // 0x2c754: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c758: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c75c: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c760: bl 0x805c0 <entry+0x549c0> @ imm = #0x53e58
    c.bl(0x805c0)?;
    // 0x2c764: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1284
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c768)?;
    const CODEC_CASE_ADDRESSES_2C768:[u32;2]=[0x000e99b0,0x000e9aed];
    for word in CODEC_CASE_ADDRESSES_2C768 {c.emit(word)?;}
    c.check(0x2c770)?;
    // 0x2c770: ldr r1, [pc, #0x96c]        @ 0x14e4 <entry+0x14e4>
    c.literal(R::R1,0x2d0e4)?;
    // 0x2c774: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c778: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2514
    c.bl(0x2ec94)?;
    // 0x2c77c: ldr r1, [pc, #0x91c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c780: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c784: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2508
    c.bl(0x2ec94)?;
    // 0x2c788: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c78c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c790: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c794: bl 0x80660 <entry+0x54a60> @ imm = #0x53ec4
    c.bl(0x80660)?;
    // 0x2c798: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x12b8
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c79c)?;
    const CODEC_CASE_ADDRESSES_2C79C:[u32;1]=[0x000e8d2c];
    for word in CODEC_CASE_ADDRESSES_2C79C {c.emit(word)?;}
    c.check(0x2c7a0)?;
    // 0x2c7a0: ldr r1, [pc, #0x938]        @ 0x14e0 <entry+0x14e0>
    c.literal(R::R1,0x2d0e0)?;
    // 0x2c7a4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c7a8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x24e4
    c.bl(0x2ec94)?;
    // 0x2c7ac: ldr r1, [pc, #0x8ec]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c7b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c7b4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x24d8
    c.bl(0x2ec94)?;
    // 0x2c7b8: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c7bc: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c7c0: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c7c4: bl 0x80674 <entry+0x54a74> @ imm = #0x53ea8
    c.bl(0x80674)?;
    // 0x2c7c8: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x12e8
    c.b(C::Al,0x2b4e8)?;
    // 0x2c7cc: ldr r1, [pc, #0x908]        @ 0x14dc <entry+0x14dc>
    c.literal(R::R1,0x2d0dc)?;
    // 0x2c7d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c7d4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x24b8
    c.bl(0x2ec94)?;
    // 0x2c7d8: ldr r1, [pc, #0x8c0]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c7dc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c7e0: bl 0x2ec94 <entry+0x3094>  @ imm = #0x24ac
    c.bl(0x2ec94)?;
    // 0x2c7e4: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c7e8: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c7ec: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c7f0: bl 0x80584 <entry+0x54984> @ imm = #0x53d8c
    c.bl(0x80584)?;
    // 0x2c7f4: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1314
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c7f8)?;
    const CODEC_CASE_ADDRESSES_2C7F8:[u32;2]=[0x000ea0ee,0x000ea127];
    for word in CODEC_CASE_ADDRESSES_2C7F8 {c.emit(word)?;}
    c.check(0x2c800)?;
    // 0x2c800: ldr r1, [pc, #0x8d0]        @ 0x14d8 <entry+0x14d8>
    c.literal(R::R1,0x2d0d8)?;
    // 0x2c804: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c808: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2484
    c.bl(0x2ec94)?;
    // 0x2c80c: ldr r1, [pc, #0x88c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c810: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c814: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2478
    c.bl(0x2ec94)?;
    // 0x2c818: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c81c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c820: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c824: bl 0x80598 <entry+0x54998> @ imm = #0x53d6c
    c.bl(0x80598)?;
    // 0x2c828: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1348
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c82c)?;
    const CODEC_CASE_ADDRESSES_2C82C:[u32;1]=[0x000ea145];
    for word in CODEC_CASE_ADDRESSES_2C82C {c.emit(word)?;}
    c.check(0x2c830)?;
    // 0x2c830: ldr r1, [pc, #0x89c]        @ 0x14d4 <entry+0x14d4>
    c.literal(R::R1,0x2d0d4)?;
    // 0x2c834: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c838: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2454
    c.bl(0x2ec94)?;
    // 0x2c83c: ldr r1, [pc, #0x85c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c840: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c844: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2448
    c.bl(0x2ec94)?;
    // 0x2c848: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c84c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c850: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c854: bl 0x805ac <entry+0x549ac> @ imm = #0x53d50
    c.bl(0x805ac)?;
    // 0x2c858: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1378
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c85c)?;
    const CODEC_CASE_ADDRESSES_2C85C:[u32;1]=[0x000e884d];
    for word in CODEC_CASE_ADDRESSES_2C85C {c.emit(word)?;}
    c.check(0x2c860)?;
    // 0x2c860: ldr r1, [pc, #0x868]        @ 0x14d0 <entry+0x14d0>
    c.literal(R::R1,0x2d0d0)?;
    // 0x2c864: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c868: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2424
    c.bl(0x2ec94)?;
    // 0x2c86c: ldr r1, [pc, #0x82c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c870: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c874: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2418
    c.bl(0x2ec94)?;
    // 0x2c878: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c87c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c880: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c884: bl 0x80610 <entry+0x54a10> @ imm = #0x53d84
    c.bl(0x80610)?;
    // 0x2c888: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x13a8
    c.b(C::Al,0x2b4e8)?;
    // 0x2c88c: ldr r1, [pc, #0x838]        @ 0x14cc <entry+0x14cc>
    c.literal(R::R1,0x2d0cc)?;
    // 0x2c890: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c894: bl 0x2ec94 <entry+0x3094>  @ imm = #0x23f8
    c.bl(0x2ec94)?;
    // 0x2c898: ldr r1, [pc, #0x800]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c89c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c8a0: bl 0x2ec94 <entry+0x3094>  @ imm = #0x23ec
    c.bl(0x2ec94)?;
    // 0x2c8a4: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c8a8: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c8ac: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c8b0: bl 0x8064c <entry+0x54a4c> @ imm = #0x53d94
    c.bl(0x8064c)?;
    // 0x2c8b4: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x13d4
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c8b8)?;
    const CODEC_CASE_ADDRESSES_2C8B8:[u32;1]=[0x000ea1d9];
    for word in CODEC_CASE_ADDRESSES_2C8B8 {c.emit(word)?;}
    c.check(0x2c8bc)?;
    // 0x2c8bc: ldr r1, [pc, #0x804]        @ 0x14c8 <entry+0x14c8>
    c.literal(R::R1,0x2d0c8)?;
    // 0x2c8c0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c8c4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x23c8
    c.bl(0x2ec94)?;
    // 0x2c8c8: ldr r1, [pc, #0x7d0]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c8cc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c8d0: bl 0x2ec94 <entry+0x3094>  @ imm = #0x23bc
    c.bl(0x2ec94)?;
    // 0x2c8d4: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c8d8: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c8dc: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c8e0: bl 0x805d4 <entry+0x549d4> @ imm = #0x53cec
    c.bl(0x805d4)?;
    // 0x2c8e4: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1404
    c.b(C::Al,0x2b4e8)?;
    // 0x2c8e8: ldr r1, [pc, #0x7d4]        @ 0x14c4 <entry+0x14c4>
    c.literal(R::R1,0x2d0c4)?;
    // 0x2c8ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c8f0: bl 0x2ec94 <entry+0x3094>  @ imm = #0x239c
    c.bl(0x2ec94)?;
    // 0x2c8f4: ldr r1, [pc, #0x7a4]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c8f8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c8fc: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2390
    c.bl(0x2ec94)?;
    // 0x2c900: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c904: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c908: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c90c: bl 0x80624 <entry+0x54a24> @ imm = #0x53d10
    c.bl(0x80624)?;
    // 0x2c910: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1430
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c914)?;
    const CODEC_CASE_ADDRESSES_2C914:[u32;2]=[0x000ea20b,0xffeef000];
    for word in CODEC_CASE_ADDRESSES_2C914 {c.emit(word)?;}
    c.check(0x2c91c)?;
    // 0x2c91c: ldr r1, [pc, #0x79c]        @ 0x14c0 <entry+0x14c0>
    c.literal(R::R1,0x2d0c0)?;
    // 0x2c920: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c924: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2368
    c.bl(0x2ec94)?;
    // 0x2c928: ldr r1, [pc, #0x770]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c92c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c930: bl 0x2ec94 <entry+0x3094>  @ imm = #0x235c
    c.bl(0x2ec94)?;
    // 0x2c934: add r0, r4, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x2c938: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c93c: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2c940: bl 0x806c0 <entry+0x54ac0> @ imm = #0x53d78
    c.bl(0x806c0)?;
    // 0x2c944: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1464
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c948)?;
    const CODEC_CASE_ADDRESSES_2C948:[u32;2]=[0x000e9cb4,0x000e9bd5];
    for word in CODEC_CASE_ADDRESSES_2C948 {c.emit(word)?;}
    c.check(0x2c950)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;handle_codec_dispatch_cases(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_case_pools_are_interleaved_with_instructions(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let w=|v:usize|u32::from_le_bytes(c.bytes[v-START..v-START+4].try_into().unwrap());
        assert_eq!(w(0x2c6a8),0x00110fff);
        assert_eq!(w(0x2c6ac),0xffdef000);
        assert_eq!(w(0x2c708),0x000e9e4e);
        assert_eq!(w(0x2c914),0x000ea20b);
        assert_eq!(w(0x2c918),0xffeef000);
        assert_eq!(w(0x2c94c),0x000e9bd5);
    }
}
