// SPDX-License-Identifier: GPL-3.0-only
//! Continuation of codec event dispatch. Three class-specific jump tables
//! select the descriptor paths and reuse handler targets in later routines.
//! Packed table words are *not* decoded as executable ARM instructions.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::Code;
const START:usize=0x2b774;
const END:usize=0x2ba68;

/// Update frame classes, allocate output buffers and dispatch by codec level.
fn route_frame_type(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2b774: ldr r1, [pc, #0xfec]        @ 0x2368 <entry+0x2368>
    c.literal(R::R1,0x2c768)?;
    // 0x2b778: b 0x2ba6c <entry+0x166c>  @ imm = #0x2ec
    c.b(C::Al,0x2ba6c)?;
    // 0x2b77c: ldr r1, [pc, #0xfe8]        @ 0x236c <entry+0x236c>
    c.literal(R::R1,0x2c76c)?;
    // 0x2b780: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b784: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3508
    c.bl(0x2ec94)?;
    // 0x2b788: ldr r1, [pc, #0xde0]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2b78c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b790: bl 0x2ec94 <entry+0x4894>  @ imm = #0x34fc
    c.bl(0x2ec94)?;
    // 0x2b794: ldr r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,192)?)?;
    // 0x2b798: mov r8, r6
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(6)?))?;
    // 0x2b79c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2b7a0: bne 0x2b7ac <entry+0x13ac>  @ imm = #0x4
    c.b(C::Ne,0x2b7ac)?;
    // 0x2b7a4: bl 0xa908 <entry+0xfffffffffffe0508> @ imm = #-0x20ea4
    c.bl(0xa908)?;
    // 0x2b7a8: str r0, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,196)?)?;
    // 0x2b7ac: bl 0xa908 <entry+0xfffffffffffe0508> @ imm = #-0x20eac
    c.bl(0xa908)?;
    // 0x2b7b0: ldr r3, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,192)?)?;
    // 0x2b7b4: ldr r1, [pc, #0xfe0]        @ 0x239c <entry+0x239c>
    c.literal(R::R1,0x2c79c)?;
    // 0x2b7b8: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x2b7bc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b7c0: bl 0x2ec94 <entry+0x4894>  @ imm = #0x34cc
    c.bl(0x2ec94)?;
    // 0x2b7c4: ldr r1, [pc, #0xda4]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2b7c8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b7cc: bl 0x2ec94 <entry+0x4894>  @ imm = #0x34c0
    c.bl(0x2ec94)?;
    // 0x2b7d0: ldr r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,192)?)?;
    // 0x2b7d4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2b7d8: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x2b7dc: str r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,192)?)?;
    // 0x2b7e0: add r0, r4, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,48)?)?;
    // 0x2b7e4: ldr r1, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x2b7e8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2b7ec: beq 0x2b83c <entry+0x143c>  @ imm = #0x48
    c.b(C::Eq,0x2b83c)?;
    // 0x2b7f0: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x2b7f4: cmp r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,16)?)?;
    // 0x2b7f8: blo 0x2b7e4 <entry+0x13e4>  @ imm = #-0x1c
    c.b(C::Cc,0x2b7e4)?;
    // 0x2b7fc: bne 0x2b83c <entry+0x143c>  @ imm = #0x38
    c.b(C::Ne,0x2b83c)?;
    // 0x2b800: ldr r1, [pc, #0xff0]        @ 0x23f8 <entry+0x23f8>
    c.literal(R::R1,0x2c7f8)?;
    // 0x2b804: ldr r2, [pc, #0xff0]        @ 0x23fc <entry+0x23fc>
    c.literal(R::R2,0x2c7fc)?;
    // 0x2b808: mov r3, #58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,58)?)?;
    // 0x2b80c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b810: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x2b814: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3478
    c.bl(0x2ec94)?;
    // 0x2b818: ldr r1, [pc, #0xd50]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2b81c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b820: bl 0x2ec94 <entry+0x4894>  @ imm = #0x346c
    c.bl(0x2ec94)?;
    // 0x2b824: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x2b828: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x2b82c: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2b830: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x2b834: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2b838: b 0x2bd60 <entry+0x1960>  @ imm = #0x520
    c.b(C::Al,0x2bd60)?;
    // 0x2b83c: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x2b840: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x2b844: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x2b848: bne 0x2b888 <entry+0x1488>  @ imm = #0x38
    c.b(C::Ne,0x2b888)?;
    // 0x2b84c: ldr r1, [pc, #0xfd8]        @ 0x242c <entry+0x242c>
    c.literal(R::R1,0x2c82c)?;
    // 0x2b850: ldr r2, [pc, #0xfa4]        @ 0x23fc <entry+0x23fc>
    c.literal(R::R2,0x2c7fc)?;
    // 0x2b854: mov r3, #78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,78)?)?;
    // 0x2b858: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b85c: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x2b860: bl 0x2ec94 <entry+0x4894>  @ imm = #0x342c
    c.bl(0x2ec94)?;
    // 0x2b864: ldr r1, [pc, #0xd04]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2b868: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b86c: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3420
    c.bl(0x2ec94)?;
    // 0x2b870: ldr r2, [pc, #0xfe4]        @ 0x245c <entry+0x245c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2b874: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2b878: mov r1, #592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,592)?)?;
    // 0x2b87c: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x2a0d4
    c.bl(0x17b0)?;
    // 0x2b880: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x2b884: mov r5, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,9)?)?;
    // 0x2b888: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x2b88c: beq 0x2bbb4 <entry+0x17b4>  @ imm = #0x320
    c.b(C::Eq,0x2bbb4)?;
    // 0x2b890: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,16)?)?;
    // 0x2b894: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x2b898: bge 0x2b9ac <entry+0x15ac>  @ imm = #0x10c
    c.b(C::Ge,0x2b9ac)?;
    // 0x2b89c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2b8a0: beq 0x2ba78 <entry+0x1678>  @ imm = #0x1d0
    c.b(C::Eq,0x2ba78)?;
    // 0x2b8a4: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x2b8a8: cmpne r0, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x2b8ac: bne 0x2bda4 <entry+0x19a4>  @ imm = #0x4f0
    c.b(C::Ne,0x2bda4)?;
    // 0x2b8b0: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x2b8b4: add r7, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(4)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x2b8b8: bl 0x2efc <entry+0xfffffffffffd8afc> @ imm = #-0x289c4
    c.bl(0x2efc)?;
    // 0x2b8bc: str r0, [r7, #0x30]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,48)?)?;
    // 0x2b8c0: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x2b8c4: ldr r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // 0x2b8c8: ldr r3, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,16)?)?;
    // 0x2b8cc: bl 0xb62d0 <entry+0x8bed0> @ imm = #0x8a9fc
    c.bl(0xb62d0)?;
    // 0x2b8d0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2b8d4: beq 0x2b910 <entry+0x1510>  @ imm = #0x34
    c.b(C::Eq,0x2b910)?;
    // 0x2b8d8: ldr r1, [pc, #0xfd8]        @ 0x24b8 <entry+0x24b8>
    c.literal(R::R1,0x2c8b8)?;
    // 0x2b8dc: ldr r2, [pc, #0xf18]        @ 0x23fc <entry+0x23fc>
    c.literal(R::R2,0x2c7fc)?;
    // 0x2b8e0: mov r3, #122
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,122)?)?;
    // 0x2b8e4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b8e8: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x2b8ec: bl 0x2ec94 <entry+0x4894>  @ imm = #0x33a0
    c.bl(0x2ec94)?;
    // 0x2b8f0: ldr r1, [pc, #0xc78]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2b8f4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b8f8: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3394
    c.bl(0x2ec94)?;
    // 0x2b8fc: ldr r2, [pc, #0xf58]        @ 0x245c <entry+0x245c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2b900: mov r1, #123
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,123)?)?;
    // 0x2b904: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2b908: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2b90c: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x2a164
    c.bl(0x17b0)?;
    // 0x2b910: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x2b914: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x2b918: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x2b91c: ldr r1, [pc, #0xff0]        @ 0x2514 <entry+0x2514>
    c.literal(R::R1,0x2c914)?;
    // 0x2b920: b 0x2bba0 <entry+0x17a0>  @ imm = #0x278
    c.b(C::Al,0x2bba0)?;
    // 0x2b924: ldr r0, [pc, #0xfec]        @ 0x2518 <entry+0x2518>
    c.literal(R::R0,0x2c918)?;
    // 0x2b928: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2b92c: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x2b930: bhi 0x2bde0 <entry+0x19e0>  @ imm = #0x4a8
    c.b(C::Hi,0x2bde0)?;
    // 0x2b934: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2b938: add r1, pc, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,4)?)?;
    // 0x2b93c: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x2b940: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2b944)?;
    const FORMAT_BUFFER_LAYOUTS:[u32;9]=[
        0x0002b968,0x0002c024,0x0002c02c,0x0002c034,0x0002c03c,
        0x0002c044,0x0002c04c,0x0002c054,0x0002c05c,
    ];
    for dest in FORMAT_BUFFER_LAYOUTS {c.emit(dest)?;}
    c.check(0x2b968)?;
    // 0x2b968: ldr r1, [pc, #0xfd8]        @ 0x2548 <entry+0x2548>
    c.literal(R::R1,0x2c948)?;
    // 0x2b96c: b 0x2c060 <entry+0x1c60>  @ imm = #0x6ec
    c.b(C::Al,0x2c060)?;
    // 0x2b970: sub r0, r7, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,1114112)?)?;
    // 0x2b974: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x2b978: bhi 0x2ba40 <entry+0x1640>  @ imm = #0xc0
    c.b(C::Hi,0x2ba40)?;
    // 0x2b97c: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2b980: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x2b984: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2b988)?;
    const LEVEL_CONTROLLER_ROUTES:[u32;7]=[
        0x0002b9a4,0x0002c174,0x0002c17c,0x0002c184,0x0002c18c,
        0x0002c0f4,0x0002c194,
    ];
    for dest in LEVEL_CONTROLLER_ROUTES {c.emit(dest)?;}
    c.check(0x2b9a4)?;
    // 0x2b9a4: ldr r1, [pc, #0xfa0]        @ 0x254c <entry+0x254c>
    c.literal(R::R1,0x2c94c)?;
    // 0x2b9a8: b 0x2c5b0 <entry+0x21b0>  @ imm = #0xc00
    c.b(C::Al,0x2c5b0)?;
    // 0x2b9ac: beq 0x2b9c0 <entry+0x15c0>  @ imm = #0xc
    c.b(C::Eq,0x2b9c0)?;
    // 0x2b9b0: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x2b9b4: beq 0x2bb10 <entry+0x1710>  @ imm = #0x154
    c.b(C::Eq,0x2bb10)?;
    // 0x2b9b8: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x2b9bc: bne 0x2bda4 <entry+0x19a4>  @ imm = #0x3e0
    c.b(C::Ne,0x2bda4)?;
    // 0x2b9c0: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x2b9c4: add r7, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(4)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x2b9c8: bl 0x2efc <entry+0xfffffffffffd8afc> @ imm = #-0x28ad4
    c.bl(0x2efc)?;
    // 0x2b9cc: str r0, [r7, #0x30]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,48)?)?;
    // 0x2b9d0: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x2b9d4: ldr r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // 0x2b9d8: ldr r3, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,16)?)?;
    // 0x2b9dc: bl 0xc8510 <entry+0x9e110> @ imm = #0x9cb2c
    c.bl(0xc8510)?;
    // 0x2b9e0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2b9e4: beq 0x2ba20 <entry+0x1620>  @ imm = #0x34
    c.b(C::Eq,0x2ba20)?;
    // 0x2b9e8: ldr r1, [pc, #0xfdc]        @ 0x25cc <entry+0x25cc>
    c.literal(R::R1,0x2c9cc)?;
    // 0x2b9ec: ldr r2, [pc, #0xe08]        @ 0x23fc <entry+0x23fc>
    c.literal(R::R2,0x2c7fc)?;
    // 0x2b9f0: mov r3, #142
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,142)?)?;
    // 0x2b9f4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b9f8: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x2b9fc: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3290
    c.bl(0x2ec94)?;
    // 0x2ba00: ldr r1, [pc, #0xb68]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2ba04: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ba08: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3284
    c.bl(0x2ec94)?;
    // 0x2ba0c: ldr r2, [pc, #0xe48]        @ 0x245c <entry+0x245c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2ba10: mov r1, #143
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,143)?)?;
    // 0x2ba14: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2ba18: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2ba1c: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x2a274
    c.bl(0x17b0)?;
    // 0x2ba20: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,16)?)?;
    // 0x2ba24: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x2ba28: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x2ba2c: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x2ba30: moveq r1, #5
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x2ba34: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x2ba38: ldr r1, [pc, #0xf90]        @ 0x25d0 <entry+0x25d0>
    c.literal(R::R1,0x2c9d0)?;
    // 0x2ba3c: b 0x2bba0 <entry+0x17a0>  @ imm = #0x15c
    c.b(C::Al,0x2bba0)?;
    // 0x2ba40: sub r0, r7, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,69632)?)?;
    // 0x2ba44: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x2ba48: bhi 0x2bf78 <entry+0x1b78>  @ imm = #0x528
    c.b(C::Hi,0x2bf78)?;
    // 0x2ba4c: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2ba50: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x2ba54: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2ba58)?;
    const SHORT_CODEC_ROUTES:[u32;4]=[
        0x0002ba68,0x0002c650,0x0002c67c,0x0002c6b0,
    ];
    for dest in SHORT_CODEC_ROUTES {c.emit(dest)?;}
    c.check(0x2ba68)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;route_frame_type(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_route_switch_tables_align(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|v:usize|u32::from_le_bytes(c.bytes[v-START..v-START+4].try_into().unwrap());
        assert_eq!(word(0x2b944),0x2b968);
        assert_eq!(word(0x2b964),0x2c05c);
        assert_eq!(word(0x2b988),0x2b9a4);
        assert_eq!(word(0x2b9a0),0x2c194);
        assert_eq!(word(0x2ba58),0x2ba68);
        assert_eq!(word(0x2ba64),0x2c6b0);
    }
}
