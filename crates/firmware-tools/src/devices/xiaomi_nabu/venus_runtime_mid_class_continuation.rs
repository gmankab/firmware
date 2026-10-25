// SPDX-License-Identifier: GPL-3.0-only
//! ARM A32 codec class handlers, refcount management and event diagnostics.
//! Named diagnostic strings, capability fields and handler addresses are
//! separately represented as data; no reference executable opcodes imported.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
use super::Code;
const START:usize=0x2cc94;
const END:usize=0x2d19c;
fn handle_controller_format_requests(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2cc94: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2cc98: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2cc9c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x2cca0: add r0, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,1)?)?;
    // 0x2cca4: sub r1, r11, #48
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,48)?)?;
    // 0x2cca8: str r0, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-44)?)?;
    // 0x2ccac: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x2ccb0: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x2ccb4: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x2ccb8: str r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-48)?)?;
    // 0x2ccbc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2ccc0: bl 0x2a878 <entry+0xffffffffffffec78> @ imm = #-0x2450
    c.bl(0x2a878)?;
    // 0x2ccc4: ldr r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,192)?)?;
    // 0x2ccc8: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x2cccc: str r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,192)?)?;
    // 0x2ccd0: bl 0xa908 <entry+0xfffffffffffded08> @ imm = #-0x223d0
    c.bl(0xa908)?;
    // 0x2ccd4: ldr r3, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,192)?)?;
    // 0x2ccd8: ldr r1, [pc, #0x4b0]        @ 0x1590 <entry+0x1590>
    c.literal(R::R1,0x2d190)?;
    // 0x2ccdc: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x2cce0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2cce4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1fa8
    c.bl(0x2ec94)?;
    // 0x2cce8: ldr r1, [pc, #0x3b0]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2ccec: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ccf0: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1f9c
    c.bl(0x2ec94)?;
    // 0x2ccf4: ldr r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,192)?)?;
    // 0x2ccf8: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2ccfc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2cd00: bne 0x2b4ec <entry+0xfffffffffffff8ec> @ imm = #-0x181c
    c.b(C::Ne,0x2b4ec)?;
    // 0x2cd04: ldr r4, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,196)?)?;
    // 0x2cd08: bl 0xa908 <entry+0xfffffffffffded08> @ imm = #-0x22408
    c.bl(0xa908)?;
    // 0x2cd0c: cmp r4, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2cd10: beq 0x2b4ec <entry+0xfffffffffffff8ec> @ imm = #-0x182c
    c.b(C::Eq,0x2b4ec)?;
    // 0x2cd14: ldr r1, [pc, #0x478]        @ 0x1594 <entry+0x1594>
    c.literal(R::R1,0x2d194)?;
    // 0x2cd18: ldr r2, [pc, #0x438]        @ 0x1558 <entry+0x1558>
    c.literal(R::R2,0x2d158)?;
    // 0x2cd1c: mov r3, #162
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,162)?)?;
    // 0x2cd20: b 0x2c20c <entry+0x60c>   @ imm = #-0xb1c
    c.b(C::Al,0x2c20c)?;
    // 0x2cd24: cmp r7, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,69632)?)?;
    // 0x2cd28: ble 0x2d06c <entry+0x146c>  @ imm = #0x33c
    c.b(C::Le,0x2d06c)?;
    // 0x2cd2c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2cd30: orr r0, r0, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,69632)?)?;
    // 0x2cd34: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2cd38: beq 0x2c668 <entry+0xa68>   @ imm = #-0x6d8
    c.b(C::Eq,0x2c668)?;
    // 0x2cd3c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2cd40: orr r0, r0, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,69632)?)?;
    // 0x2cd44: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2cd48: beq 0x2c694 <entry+0xa94>   @ imm = #-0x6bc
    c.b(C::Eq,0x2c694)?;
    // 0x2cd4c: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2cd50: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2cd54: orr r0, r0, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,69632)?)?;
    // 0x2cd58: orr r2, r2, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,135168)?)?;
    // 0x2cd5c: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2cd60: beq 0x2c6c8 <entry+0xac8>   @ imm = #-0x6a0
    c.b(C::Eq,0x2c6c8)?;
    // 0x2cd64: b 0x2c5cc <entry+0x9cc>   @ imm = #-0x7a0
    c.b(C::Al,0x2c5cc)?;
    // 0x2cd68: ldr r2, [pc, #0x3b4]        @ 0x1524 <entry+0x1524>
    c.literal(R::R2,0x2d124)?;
    // 0x2cd6c: mov r1, #205
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,205)?)?;
    // 0x2cd70: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2cd74: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x2cd78: bl 0x17b0 <entry+0xfffffffffffd5bb0> @ imm = #-0x2b5d0
    c.bl(0x17b0)?;
    // 0x2cd7c: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x189c
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2cd80)?;
    const CLASS_DATA_2CD80:[u32;1]=[0x000e8d69];
    for word in CLASS_DATA_2CD80 {c.emit(word)?;}
    c.check(0x2cd84)?;
    // 0x2cd84: ldr r4, [pc, #0x3a0]        @ 0x152c <entry+0x152c>
    c.literal(R::new(4)?,0x2d12c)?;
    // 0x2cd88: ldr r1, [pc, #0x3b8]        @ 0x1548 <entry+0x1548>
    c.literal(R::R1,0x2d148)?;
    // 0x2cd8c: mov r3, #155
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,155)?)?;
    // 0x2cd90: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cd94: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2cd98: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x2cd9c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1ef0
    c.bl(0x2ec94)?;
    // 0x2cda0: ldr r5, [pc, #0x2f8]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::new(5)?,0x2d0a0)?;
    // 0x2cda4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cda8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x2cdac: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1ee0
    c.bl(0x2ec94)?;
    // 0x2cdb0: ldr r0, [pc, #0x394]        @ 0x154c <entry+0x154c>
    c.literal(R::R0,0x2d14c)?;
    // 0x2cdb4: bl 0x32190 <entry+0x6590>  @ imm = #0x53d4
    c.bl(0x32190)?;
    // 0x2cdb8: ldr r1, [pc, #0x390]        @ 0x1550 <entry+0x1550>
    c.literal(R::R1,0x2d150)?;
    // 0x2cdbc: mov r3, #157
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,157)?)?;
    // 0x2cdc0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cdc4: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x2cdc8: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2cdcc: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1ec0
    c.bl(0x2ec94)?;
    // 0x2cdd0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cdd4: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x2cdd8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1eb4
    c.bl(0x2ec94)?;
    // 0x2cddc: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x18fc
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2cde0)?;
    const CLASS_DATA_2CDE0:[u32;1]=[0x000e9e72];
    for word in CLASS_DATA_2CDE0 {c.emit(word)?;}
    c.check(0x2cde4)?;
    // 0x2cde4: ldr r1, [pc, #0x358]        @ 0x1544 <entry+0x1544>
    c.literal(R::R1,0x2d144)?;
    // 0x2cde8: ldr r2, [pc, #0x33c]        @ 0x152c <entry+0x152c>
    c.literal(R::R2,0x2d12c)?;
    // 0x2cdec: mov r3, #163
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,163)?)?;
    // 0x2cdf0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cdf4: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2cdf8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1e94
    c.bl(0x2ec94)?;
    // 0x2cdfc: ldr r1, [pc, #0x29c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2ce00: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ce04: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1e88
    c.bl(0x2ec94)?;
    // 0x2ce08: bl 0x32290 <entry+0x6690>  @ imm = #0x5480
    c.bl(0x32290)?;
    // 0x2ce0c: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x192c
    c.b(C::Al,0x2b4e8)?;
    // 0x2ce10: ldr r1, [pc, #0x328]        @ 0x1540 <entry+0x1540>
    c.literal(R::R1,0x2d140)?;
    // 0x2ce14: ldr r2, [pc, #0x310]        @ 0x152c <entry+0x152c>
    c.literal(R::R2,0x2d12c)?;
    // 0x2ce18: mov r3, #169
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,169)?)?;
    // 0x2ce1c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ce20: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2ce24: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1e68
    c.bl(0x2ec94)?;
    // 0x2ce28: ldr r1, [pc, #0x270]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2ce2c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ce30: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1e5c
    c.bl(0x2ec94)?;
    // 0x2ce34: bl 0x3238c <entry+0x678c>  @ imm = #0x5550
    c.bl(0x3238c)?;
    // 0x2ce38: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1958
    c.b(C::Al,0x2b4e8)?;
    // 0x2ce3c: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x2ce40: ldr r1, [pc, #0x2f4]        @ 0x153c <entry+0x153c>
    c.literal(R::R1,0x2d13c)?;
    // 0x2ce44: ldr r2, [pc, #0x2e0]        @ 0x152c <entry+0x152c>
    c.literal(R::R2,0x2d12c)?;
    // 0x2ce48: mov r3, #175
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,175)?)?;
    // 0x2ce4c: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2ce50: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2ce54: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ce58: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1e34
    c.bl(0x2ec94)?;
    // 0x2ce5c: ldr r1, [pc, #0x23c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2ce60: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ce64: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1e28
    c.bl(0x2ec94)?;
    // 0x2ce68: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x2ce6c: bl 0x32424 <entry+0x6824>  @ imm = #0x55b0
    c.bl(0x32424)?;
    // 0x2ce70: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1990
    c.b(C::Al,0x2b4e8)?;
    // 0x2ce74: ldr r1, [pc, #0x2bc]        @ 0x1538 <entry+0x1538>
    c.literal(R::R1,0x2d138)?;
    // 0x2ce78: ldr r2, [pc, #0x2ac]        @ 0x152c <entry+0x152c>
    c.literal(R::R2,0x2d12c)?;
    // 0x2ce7c: mov r3, #181
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,181)?)?;
    // 0x2ce80: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ce84: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2ce88: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1e04
    c.bl(0x2ec94)?;
    // 0x2ce8c: ldr r1, [pc, #0x20c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2ce90: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ce94: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1df8
    c.bl(0x2ec94)?;
    // 0x2ce98: bl 0x31e94 <entry+0x6294>  @ imm = #0x4ff4
    c.bl(0x31e94)?;
    // 0x2ce9c: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x19bc
    c.b(C::Al,0x2b4e8)?;
    // 0x2cea0: ldr r1, [pc, #0x28c]        @ 0x1534 <entry+0x1534>
    c.literal(R::R1,0x2d134)?;
    // 0x2cea4: ldr r2, [pc, #0x280]        @ 0x152c <entry+0x152c>
    c.literal(R::R2,0x2d12c)?;
    // 0x2cea8: mov r3, #187
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,187)?)?;
    // 0x2ceac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ceb0: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2ceb4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1dd8
    c.bl(0x2ec94)?;
    // 0x2ceb8: ldr r1, [pc, #0x1e0]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2cebc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cec0: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1dcc
    c.bl(0x2ec94)?;
    // 0x2cec4: bl 0x31dcc <entry+0x61cc>  @ imm = #0x4f00
    c.bl(0x31dcc)?;
    // 0x2cec8: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x19e8
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2cecc)?;
    const CLASS_DATA_2CECC:[u32;1]=[0x000ea097];
    for word in CLASS_DATA_2CECC {c.emit(word)?;}
    c.check(0x2ced0)?;
    // 0x2ced0: ldr r1, [pc, #0x258]        @ 0x1530 <entry+0x1530>
    c.literal(R::R1,0x2d130)?;
    // 0x2ced4: ldr r2, [pc, #0x250]        @ 0x152c <entry+0x152c>
    c.literal(R::R2,0x2d12c)?;
    // 0x2ced8: mov r3, #193
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,193)?)?;
    // 0x2cedc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cee0: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2cee4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1da8
    c.bl(0x2ec94)?;
    // 0x2cee8: ldr r1, [pc, #0x1b0]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2ceec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cef0: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1d9c
    c.bl(0x2ec94)?;
    // 0x2cef4: bl 0x31f74 <entry+0x6374>  @ imm = #0x5078
    c.bl(0x31f74)?;
    // 0x2cef8: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1a18
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2cefc)?;
    const CLASS_DATA_2CEFC:[u32;1]=[0x0010ceac];
    for word in CLASS_DATA_2CEFC {c.emit(word)?;}
    c.check(0x2cf00)?;
    // 0x2cf00: ldr r1, [pc, #0x220]        @ 0x1528 <entry+0x1528>
    c.literal(R::R1,0x2d128)?;
    // 0x2cf04: ldr r2, [pc, #0x220]        @ 0x152c <entry+0x152c>
    c.literal(R::R2,0x2d12c)?;
    // 0x2cf08: mov r3, #199
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,199)?)?;
    // 0x2cf0c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cf10: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2cf14: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1d78
    c.bl(0x2ec94)?;
    // 0x2cf18: ldr r1, [pc, #0x180]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2cf1c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cf20: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1d6c
    c.bl(0x2ec94)?;
    // 0x2cf24: bl 0x31ff0 <entry+0x63f0>  @ imm = #0x50c4
    c.bl(0x31ff0)?;
    // 0x2cf28: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0x1a48
    c.b(C::Al,0x2b4e8)?;
    c.check(0x2cf2c)?;
    const CLASS_DATA_2CF2C:[u32;2]=[0x00010102,0x000ea0b7];
    for word in CLASS_DATA_2CF2C {c.emit(word)?;}
    c.check(0x2cf34)?;
    // 0x2cf34: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x2cf38: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x2cf3c: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2cf40: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2cf44: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2cf48: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2cf4c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2cf50: str r6, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,28)?)?;
    // 0x2cf54: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x2cf58: add r0, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,1)?)?;
    // 0x2cf5c: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2cf60: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2cf64: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2cf68: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2cf6c: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x2cf70: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2cf74: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2cf78: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x2de8
    c.bl(0x2a198)?;
    // 0x2cf7c: ldr r1, [pc, #0x1dc]        @ 0x1560 <entry+0x1560>
    c.literal(R::R1,0x2d160)?;
    // 0x2cf80: ldr r2, [pc, #0x1d0]        @ 0x1558 <entry+0x1558>
    c.literal(R::R2,0x2d158)?;
    // 0x2cf84: mov r3, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,22)?)?;
    // 0x2cf88: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cf8c: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x2cf90: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1cfc
    c.bl(0x2ec94)?;
    // 0x2cf94: ldr r1, [pc, #0x104]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2cf98: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2cf9c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1cf0
    c.bl(0x2ec94)?;
    // 0x2cfa0: mov r1, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,23)?)?;
    // 0x2cfa4: b 0x2c63c <entry+0xa3c>   @ imm = #-0x970
    c.b(C::Al,0x2c63c)?;
    c.check(0x2cfa8)?;
    const CLASS_DATA_2CFA8:[u32;3]=[0x000ea4f0,0x000ea527,0x000ea548];
    for word in CLASS_DATA_2CFA8 {c.emit(word)?;}
    c.check(0x2cfb4)?;
    // 0x2cfb4: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x2cfb8: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x2cfbc: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2cfc0: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2cfc4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2cfc8: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2cfcc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2cfd0: str r6, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,28)?)?;
    // 0x2cfd4: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x2cfd8: add r0, r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,2)?)?;
    // 0x2cfdc: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2cfe0: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2cfe4: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2cfe8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2cfec: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x2cff0: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2cff4: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2cff8: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x2e68
    c.bl(0x2a198)?;
    // 0x2cffc: ldr r1, [pc, #0x15c]        @ 0x1560 <entry+0x1560>
    c.literal(R::R1,0x2d160)?;
    // 0x2d000: ldr r2, [pc, #0x150]        @ 0x1558 <entry+0x1558>
    c.literal(R::R2,0x2d158)?;
    // 0x2d004: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2d008: mov r3, #2128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2128)?)?;
    // 0x2d00c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1c80
    c.bl(0x2ec94)?;
    // 0x2d010: ldr r1, [pc, #0x88]         @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2d014: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2d018: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1c74
    c.bl(0x2ec94)?;
    // 0x2d01c: mov r1, #81
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,81)?)?;
    // 0x2d020: b 0x2c63c <entry+0xa3c>   @ imm = #-0x9ec
    c.b(C::Al,0x2c63c)?;
    c.check(0x2d024)?;
    const CLASS_DATA_2D024:[u32;8]=[0x000e9cd8,0x000e9d00,0x000e9d28,0x000e9d50,0x000e9d7f,0x000e9daa,0x000e9ddb,0x000e9e03];
    for word in CLASS_DATA_2D024 {c.emit(word)?;}
    c.check(0x2d044)?;
    // 0x2d044: cmp r7, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x2d048: beq 0x2be08 <entry+0x208>   @ imm = #-0x1248
    c.b(C::Eq,0x2be08)?;
    // 0x2d04c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2d050: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2d054: orr r0, r0, #2162688
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2162688)?)?;
    // 0x2d058: orr r2, r2, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,135168)?)?;
    // 0x2d05c: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2d060: beq 0x2c0ac <entry+0x4ac>   @ imm = #-0xfbc
    c.b(C::Eq,0x2c0ac)?;
    // 0x2d064: b 0x2c5cc <entry+0x9cc>   @ imm = #-0xaa0
    c.b(C::Al,0x2c5cc)?;
    c.check(0x2d068)?;
    const CLASS_DATA_2D068:[u32;1]=[0x000e9e2e];
    for word in CLASS_DATA_2D068 {c.emit(word)?;}
    c.check(0x2d06c)?;
    // 0x2d06c: ldr r0, [pc, #0x30]         @ 0x14a4 <entry+0x14a4>
    c.literal(R::R0,0x2d0a4)?;
    // 0x2d070: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2d074: beq 0x2bf08 <entry+0x308>   @ imm = #-0x1174
    c.b(C::Eq,0x2bf08)?;
    // 0x2d078: ldr r0, [pc, #0x28]         @ 0x14a8 <entry+0x14a8>
    c.literal(R::R0,0x2d0a8)?;
    // 0x2d07c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2d080: orr r2, r2, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,135168)?)?;
    // 0x2d084: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2d088: beq 0x2bf9c <entry+0x39c>   @ imm = #-0x10f4
    c.b(C::Eq,0x2bf9c)?;
    // 0x2d08c: b 0x2c5cc <entry+0x9cc>   @ imm = #-0xac8
    c.b(C::Al,0x2c5cc)?;
    c.check(0x2d090)?;
    const CLASS_DATA_2D090:[u32;67]=[0x000e9e97,0x000e8e8a,0x000ea0d7,0xffdeefff,0x000e2788,0x00010101,0x00010102,0x000e9c93,0x000e9c68,0x000e9c41,0x000e9c19,0x000e9bf5,0x000ea077,0x000ea053,0x000ea026,0x000e9ffb,0x000e9fd3,0x000e9fb2,0x000e9f90,0x000e9f6d,0x000e9f46,0x000e9f1e,0x000e9efe,0x000e9edd,0x000e9eb3,0x000e9ba6,0x000e9b7f,0x000e9b57,0x000e9b34,0x000e9b11,0x000e9ac9,0x000e9aa5,0x000e9a7d,0x000e9a59,0x000e9a3a,0x000e9a1e,0x000e99fc,0x000e884d,0x000ea72e,0x000ea527,0x000ea6f5,0x000ea6b9,0x000ea683,0x000ea63a,0x000ea601,0x000ea5ca,0x000ea55b,0x00029ae4,0x000ea590,0x000e8f9b,0x000e8e23,0x000e8f6d,0x000e8f30,0x000e8ee1,0x000e8eb0,0x000e8e42,0x000ea48e,0x000ea4d1,0x0010ded8,0x000ea41a,0x000ea471,0x000e8d8d,0x000ea3b3,0x000ea3fb,0x000e8db3,0x000e8df0,0x000e8ff3];
    for word in CLASS_DATA_2D090 {c.emit(word)?;}
    c.check(0x2d19c)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;handle_controller_format_requests(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_class_literals_are_identified(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|addr:usize|u32::from_le_bytes(c.bytes[addr-START..addr-START+4].try_into().unwrap());
        assert_eq!(word(0x2cd80),0x000e8d69);
        assert_eq!(word(0x2cecc),0x000ea097);
        assert_eq!(word(0x2d090),0x000e9e97);
        assert_eq!(word(0x2d09c),0xffdeefff);
        assert_eq!(word(0x2d14c),0x00029ae4);
        assert_eq!(word(0x2d198),0x000e8ff3);
    }
}
