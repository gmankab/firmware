// SPDX-License-Identifier: GPL-3.0-only
//! Codec request packet creation, response dispatch and error reporting.
//! Literal pools (including a nine-way handler target table) are treated as
//! data addresses; all executable words use public A32 typed encoders.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2c950;
const END:usize=0x2cc94;
fn dispatch_controller_requests(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2c950: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x2c954: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x2c958: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2c95c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2c960: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2c964: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2c968: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2c96c: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x2c970: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2c974: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x2c978: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2c97c: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2c980: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2c984: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2c988: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2c98c: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x2c990: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2c994: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2c998: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x2808
    c.bl(0x2a198)?;
    // 0x2c99c: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x2c9a0: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x2c9a4: ldr r2, [pc, #0x7d4]        @ 0x1580 <entry+0x1580>
    c.literal(R::R2,0x2d180)?;
    // 0x2c9a8: mov r3, #876
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,876)?)?;
    // 0x2c9ac: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2c9b0: ldr r1, [pc, #0x7c4]        @ 0x157c <entry+0x157c>
    c.literal(R::R1,0x2d17c)?;
    // 0x2c9b4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2c9b8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x22d4
    c.bl(0x2ec94)?;
    // 0x2c9bc: ldr r1, [pc, #0x6dc]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c9c0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2c9c4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x22c8
    c.bl(0x2ec94)?;
    // 0x2c9c8: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x14e8
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2c9cc)?;
    const REQUEST_DATA_2C9CC:[u32;2]=[0x000ea235,0x000ea265];
    for word in REQUEST_DATA_2C9CC {c.emit(word)?;}
    c.check(0x2c9d4)?;
    // 0x2c9d4: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x2c9d8: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x2c9dc: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x2c9e0: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2c9e4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2c9e8: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x2c9ec: mov r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
    // 0x2c9f0: str r6, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,28)?)?;
    // 0x2c9f4: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x2c9f8: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2c9fc: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2ca00: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2ca04: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2ca08: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2ca0c: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x2ca10: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2ca14: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2ca18: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x2888
    c.bl(0x2a198)?;
    // 0x2ca1c: ldr r1, [pc, #0x738]        @ 0x155c <entry+0x155c>
    c.literal(R::R1,0x2d15c)?;
    // 0x2ca20: ldr r2, [pc, #0x730]        @ 0x1558 <entry+0x1558>
    c.literal(R::R2,0x2d158)?;
    // 0x2ca24: mov r3, #37
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,37)?)?;
    // 0x2ca28: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ca2c: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x2ca30: bl 0x2ec94 <entry+0x3094>  @ imm = #0x225c
    c.bl(0x2ec94)?;
    // 0x2ca34: ldr r1, [pc, #0x664]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2ca38: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ca3c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2250
    c.bl(0x2ec94)?;
    // 0x2ca40: mov r1, #38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,38)?)?;
    // 0x2ca44: b 0x2c63c <entry+0xa3c>   @ imm = #-0x410
    c.b(C::Al,0x2c63c)?;
    // 0x2ca48: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x2ca4c: ldr r2, [pc, #0x704]        @ 0x1558 <entry+0x1558>
    c.literal(R::R2,0x2d158)?;
    // 0x2ca50: mov r3, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,52)?)?;
    // 0x2ca54: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2ca58: ldr r1, [pc, #0x6f4]        @ 0x1554 <entry+0x1554>
    c.literal(R::R1,0x2d154)?;
    // 0x2ca5c: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x2ca60: b 0x2cb68 <entry+0xf68>   @ imm = #0x100
    c.b(C::Al,0x2cb68)?;
    c.check(0x2ca64)?;
    const REQUEST_DATA_2CA64:[u32;2]=[0x000e99d4,0x000ea175];
    for word in REQUEST_DATA_2CA64 {c.emit(word)?;}
    c.check(0x2ca6c)?;
    // 0x2ca6c: ldr r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // 0x2ca70: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x2ca74: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2ca78: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x2ca7c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x2ca80: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x2ca84: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x2ca88: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x2ca8c: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x2ca90: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2ca94: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2ca98: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2ca9c: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2caa0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2caa4: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x2caa8: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2caac: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2cab0: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x2920
    c.bl(0x2a198)?;
    // 0x2cab4: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x2cab8: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x2cabc: ldr r2, [pc, #0x6c8]        @ 0x158c <entry+0x158c>
    c.literal(R::R2,0x2d18c)?;
    // 0x2cac0: mov r3, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,46)?)?;
    // 0x2cac4: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // 0x2cac8: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2cacc: ldr r1, [pc, #0x6b4]        @ 0x1588 <entry+0x1588>
    c.literal(R::R1,0x2d188)?;
    // 0x2cad0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cad4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x21b8
    c.bl(0x2ec94)?;
    // 0x2cad8: ldr r1, [pc, #0x5c0]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2cadc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cae0: bl 0x2ec94 <entry+0x3094>  @ imm = #0x21ac
    c.bl(0x2ec94)?;
    // 0x2cae4: b 0x2ccc4 <entry+0x10c4>  @ imm = #0x1d8
    c.b(C::Al,0x2ccc4)?;
    c.check(0x2cae8)?;
    const REQUEST_DATA_2CAE8:[u32;1]=[0x000ea1af];
    for word in REQUEST_DATA_2CAE8 {c.emit(word)?;}
    c.check(0x2caec)?;
    // 0x2caec: mov r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x2caf0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2caf4: orr r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65536)?)?;
    // 0x2caf8: orr r2, r2, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,135168)?)?;
    // 0x2cafc: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2cb00: bgt 0x2cd24 <entry+0x1124>  @ imm = #0x21c
    c.b(C::Gt,0x2cd24)?;
    // 0x2cb04: mvn r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,65536)?)?;
    // 0x2cb08: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2cb0c: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x2cb10: bhi 0x2c5cc <entry+0x9cc>   @ imm = #-0x54c
    c.b(C::Hi,0x2c5cc)?;
    // 0x2cb14: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2cb18: add r1, pc, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,4)?)?;
    // 0x2cb1c: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x2cb20: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2cb24)?;
    const REQUEST_DATA_2CB24:[u32;11]=[0x0002c1b8,0x0002c23c,0x0002c28c,0x0002c2e8,0x0002c32c,0x0002c354,0x0002b794,0x0002c444,0x0002c58c,0x000ea28d,0x000ea2c7];
    for word in REQUEST_DATA_2CB24 {c.emit(word)?;}
    c.check(0x2cb50)?;
    // 0x2cb50: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x2cb54: ldr r1, [pc, #0x608]        @ 0x1564 <entry+0x1564>
    c.literal(R::R1,0x2d164)?;
    // 0x2cb58: ldr r2, [pc, #0x5f8]        @ 0x1558 <entry+0x1558>
    c.literal(R::R2,0x2d158)?;
    // 0x2cb5c: mov r3, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,254)?)?;
    // 0x2cb60: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x2cb64: orr r3, r3, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // 0x2cb68: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cb6c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2120
    c.bl(0x2ec94)?;
    // 0x2cb70: ldr r1, [pc, #0x528]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2cb74: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cb78: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2114
    c.bl(0x2ec94)?;
    // 0x2cb7c: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x2cb80: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x2cb84: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x2cb88: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2cb8c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2cb90: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x2cb94: mov r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // 0x2cb98: str r6, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,28)?)?;
    // 0x2cb9c: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x2cba0: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2cba4: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2cba8: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2cbac: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2cbb0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2cbb4: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x2cbb8: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2cbbc: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2cbc0: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x2a30
    c.bl(0x2a198)?;
    // 0x2cbc4: b 0x2b4ec <entry+0xfffffffffffff8ec> @ imm = #-0x16e0
    c.b(C::Al,0x2b4ec)?;
    c.check(0x2cbc8)?;
    const REQUEST_DATA_2CBC8:[u32;1]=[0x000ea2f1];
    for word in REQUEST_DATA_2CBC8 {c.emit(word)?;}
    c.check(0x2cbcc)?;
    // 0x2cbcc: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x2cbd0: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x2cbd4: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2cbd8: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2cbdc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2cbe0: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2cbe4: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x2cbe8: str r6, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,28)?)?;
    // 0x2cbec: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x2cbf0: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x2cbf4: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2cbf8: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2cbfc: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2cc00: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2cc04: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x2cc08: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2cc0c: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2cc10: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x2a80
    c.bl(0x2a198)?;
    // 0x2cc14: ldr r1, [pc, #0x540]        @ 0x155c <entry+0x155c>
    c.literal(R::R1,0x2d15c)?;
    // 0x2cc18: ldr r2, [pc, #0x538]        @ 0x1558 <entry+0x1558>
    c.literal(R::R2,0x2d158)?;
    // 0x2cc1c: mov r3, #95
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,95)?)?;
    // 0x2cc20: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cc24: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x2cc28: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2064
    c.bl(0x2ec94)?;
    // 0x2cc2c: ldr r1, [pc, #0x46c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2cc30: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cc34: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2058
    c.bl(0x2ec94)?;
    // 0x2cc38: ldr r2, [pc, #0x4e4]        @ 0x1524 <entry+0x1524>
    c.literal(R::R2,0x2d124)?;
    // 0x2cc3c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2cc40: mov r1, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2144)?)?;
    // 0x2cc44: b 0x2c648 <entry+0xa48>   @ imm = #-0x604
    c.b(C::Al,0x2c648)?;
    c.check(0x2cc48)?;
    const REQUEST_DATA_2CC48:[u32;1]=[0x000ea320];
    for word in REQUEST_DATA_2CC48 {c.emit(word)?;}
    c.check(0x2cc4c)?;
    // 0x2cc4c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2cc50: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2cc54: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2cc58: bl 0x20a0 <entry+0xfffffffffffd64a0> @ imm = #-0x2abc0
    c.bl(0x20a0)?;
    // 0x2cc5c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2cc60: orr r0, r0, #2228224
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2228224)?)?;
    // 0x2cc64: add r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // 0x2cc68: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x2cc6c: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x2cc70: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2cc74: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x2cc78: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2cc7c: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2cc80: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2cc84: bl 0x2a878 <entry+0xffffffffffffec78> @ imm = #-0x2414
    c.bl(0x2a878)?;
    // 0x2cc88: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x17a8
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2cc8c)?;
    const REQUEST_DATA_2CC8C:[u32;2]=[0x000ea35a,0x000ea385];
    for word in REQUEST_DATA_2CC8C {c.emit(word)?;}
    c.check(0x2cc94)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;dispatch_controller_requests(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_request_data_is_explicit(){
        let mut code=Code::new(START);emit(&mut code).unwrap();
        assert_eq!(code.bytes.len(),END-START);
        let at=|address:usize| -> u32 {
            let i=address-START;
            u32::from_le_bytes(code.bytes[i..i+4].try_into().unwrap())
        };
        assert_eq!(at(0x2c9cc),0x000ea235);
        assert_eq!(at(0x2ca64),0x000e99d4);
        assert_eq!(at(0x2cb24),0x0002c1b8);
        assert_eq!(at(0x2cb44),0x0002c58c);
        assert_eq!(at(0x2cc90),0x000ea385);
    }
}
