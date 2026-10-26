// SPDX-License-Identifier: GPL-3.0-only
//! Final controller register-bank and stream-command descriptor array writes.
//! The main function's completion logic follows this block. Every output word
//! here is an ARM32 instruction generated from explicit public ISA operands;
//! diagnostic pointers loaded through PC refer to preceding named pools.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
const START:usize=0x3c53c;
const END:usize=0x3dc00;

fn populate_stream_controller_registers(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3c53c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c540: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c544: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c548: ldr r3, [pc, #-0x9c]        @ 0x28e4 <entry+0x28e4>
    c.literal_cond(C::Al,R::new(3)?,0x3c4b4)?;
    // 0x3c54c: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c550: mov r3, #164
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,164)?)?;
    // 0x3c554: mov r4, #168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,168)?)?;
    // 0x3c558: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c55c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c560: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c564: mov r3, #836
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,836)?)?;
    // 0x3c568: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c56c: add r3, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3c570: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c574: mov r3, #172
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,172)?)?;
    // 0x3c578: mov r4, #176
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,176)?)?;
    // 0x3c57c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c580: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c584: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c588: ldr r3, [pc, #-0xd8]        @ 0x28e8 <entry+0x28e8>
    c.literal_cond(C::Al,R::new(3)?,0x3c4b8)?;
    // 0x3c58c: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c590: mov r3, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,180)?)?;
    // 0x3c594: mov r4, #184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,184)?)?;
    // 0x3c598: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c59c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c5a0: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c5a4: add r3, r6, #69
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,69)?)?;
    // 0x3c5a8: add r3, r3, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4864)?)?;
    // 0x3c5ac: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c5b0: ldr r4, [pc, #-0xfc]        @ 0x28ec <entry+0x28ec>
    c.literal_cond(C::Al,R::new(4)?,0x3c4bc)?;
    // 0x3c5b4: mov r3, #188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,188)?)?;
    // 0x3c5b8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c5bc: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c5c0: mov r3, #4288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4288)?)?;
    // 0x3c5c4: str r4, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c5c8: mov r3, #196
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,196)?)?;
    // 0x3c5cc: mov r4, #200
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,200)?)?;
    // 0x3c5d0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c5d4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c5d8: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c5dc: add r3, r6, #70
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,70)?)?;
    // 0x3c5e0: add r3, r3, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4864)?)?;
    // 0x3c5e4: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c5e8: mov r3, #204
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,204)?)?;
    // 0x3c5ec: mov r4, #208
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,208)?)?;
    // 0x3c5f0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c5f4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c5f8: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c5fc: ldr r3, [pc, #-0x144]       @ 0x28f0 <entry+0x28f0>
    c.literal_cond(C::Al,R::new(3)?,0x3c4c0)?;
    // 0x3c600: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c604: mov r3, #212
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,212)?)?;
    // 0x3c608: mov r4, #216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,216)?)?;
    // 0x3c60c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c610: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c614: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c618: add r3, r6, #71
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,71)?)?;
    // 0x3c61c: add r3, r3, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4864)?)?;
    // 0x3c620: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c624: mov r3, #220
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,220)?)?;
    // 0x3c628: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c62c: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c630: ldr r3, [pc, #-0x174]       @ 0x28f4 <entry+0x28f4>
    c.literal_cond(C::Al,R::new(3)?,0x3c4c4)?;
    // 0x3c634: str r3, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c638: mov r3, #228
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,228)?)?;
    // 0x3c63c: mov r7, #232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,232)?)?;
    // 0x3c640: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c644: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c648: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c64c: mov r3, #840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,840)?)?;
    // 0x3c650: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c654: add r4, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3c658: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c65c: mov r4, #236
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,236)?)?;
    // 0x3c660: mov r7, #240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,240)?)?;
    // 0x3c664: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c668: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c66c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c670: ldr r4, [pc, #-0x1b0]       @ 0x28f8 <entry+0x28f8>
    c.literal_cond(C::Al,R::new(4)?,0x3c4c8)?;
    // 0x3c674: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c678: add r4, r6, #73
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,73)?)?;
    // 0x3c67c: str r0, [r10, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(9)?,Shift::Lsl,0)?)?;
    // 0x3c680: mov r9, #248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,248)?)?;
    // 0x3c684: ldr r7, [pc, #-0x1c0]       @ 0x28fc <entry+0x28fc>
    c.literal_cond(C::Al,R::new(7)?,0x3c4cc)?;
    // 0x3c688: add r4, r4, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4864)?)?;
    // 0x3c68c: orr r9, r9, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,4096)?)?;
    // 0x3c690: str r4, [r10, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(9)?,Shift::Lsl,0)?)?;
    // 0x3c694: mov r4, #252
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,252)?)?;
    // 0x3c698: ldr r9, [pc, #-0x1d0]       @ 0x2900 <entry+0x2900>
    c.literal_cond(C::Al,R::new(9)?,0x3c4d0)?;
    // 0x3c69c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c6a0: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c6a4: mov r4, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4352)?)?;
    // 0x3c6a8: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c6ac: mov r4, #260
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,260)?)?;
    // 0x3c6b0: mov r7, #264
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,264)?)?;
    // 0x3c6b4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c6b8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c6bc: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c6c0: add r4, r6, #74
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,74)?)?;
    // 0x3c6c4: add r4, r4, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4864)?)?;
    // 0x3c6c8: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c6cc: mov r4, #268
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,268)?)?;
    // 0x3c6d0: mov r7, #272
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,272)?)?;
    // 0x3c6d4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c6d8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c6dc: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c6e0: ldr r4, [pc, #-0x214]       @ 0x2904 <entry+0x2904>
    c.literal_cond(C::Al,R::new(4)?,0x3c4d4)?;
    // 0x3c6e4: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c6e8: mov r4, #276
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,276)?)?;
    // 0x3c6ec: mov r7, #280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,280)?)?;
    // 0x3c6f0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c6f4: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c6f8: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c6fc: add r4, r6, #75
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,75)?)?;
    // 0x3c700: add r4, r4, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4864)?)?;
    // 0x3c704: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c708: mov r4, #284
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,284)?)?;
    // 0x3c70c: mov r7, #288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,288)?)?;
    // 0x3c710: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c714: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c718: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c71c: ldr r4, [pc, #-0x24c]       @ 0x2908 <entry+0x2908>
    c.literal_cond(C::Al,R::new(4)?,0x3c4d8)?;
    // 0x3c720: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c724: mov r4, #292
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,292)?)?;
    // 0x3c728: mov r7, #296
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,296)?)?;
    // 0x3c72c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c730: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c734: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c738: add r4, r6, #3824
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,3824)?)?;
    // 0x3c73c: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c740: mov r4, #300
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,300)?)?;
    // 0x3c744: mov r7, #304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,304)?)?;
    // 0x3c748: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c74c: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c750: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c754: ldr r4, [pc, #-0x280]       @ 0x290c <entry+0x290c>
    c.literal_cond(C::Al,R::new(4)?,0x3c4dc)?;
    // 0x3c758: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c75c: mov r4, #308
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,308)?)?;
    // 0x3c760: mov r7, #312
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,312)?)?;
    // 0x3c764: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c768: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c76c: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c770: add r4, r6, #760
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,760)?)?;
    // 0x3c774: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3c778: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c77c: ldr r7, [pc, #-0x2a4]       @ 0x2910 <entry+0x2910>
    c.literal_cond(C::Al,R::new(7)?,0x3c4e0)?;
    // 0x3c780: mov r4, #316
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,316)?)?;
    // 0x3c784: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c788: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c78c: mov r4, #4416
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4416)?)?;
    // 0x3c790: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c794: mov r4, #324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,324)?)?;
    // 0x3c798: mov r7, #328
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,328)?)?;
    // 0x3c79c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c7a0: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c7a4: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c7a8: add r4, r6, #3872
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,3872)?)?;
    // 0x3c7ac: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c7b0: mov r4, #332
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,332)?)?;
    // 0x3c7b4: ldr r7, [pc, #-0x2d8]       @ 0x2914 <entry+0x2914>
    c.literal_cond(C::Al,R::new(7)?,0x3c4e4)?;
    // 0x3c7b8: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c7bc: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c7c0: mov r4, #336
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,336)?)?;
    // 0x3c7c4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c7c8: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c7cc: mov r4, #340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,340)?)?;
    // 0x3c7d0: mov r7, #344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,344)?)?;
    // 0x3c7d4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c7d8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c7dc: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c7e0: add r4, r6, #804
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,804)?)?;
    // 0x3c7e4: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3c7e8: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c7ec: mov r4, #348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,348)?)?;
    // 0x3c7f0: ldr r7, [pc, #-0x310]       @ 0x2918 <entry+0x2918>
    c.literal_cond(C::Al,R::new(7)?,0x3c4e8)?;
    // 0x3c7f4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c7f8: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c7fc: mov r4, #352
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,352)?)?;
    // 0x3c800: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c804: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c808: mov r4, #356
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,356)?)?;
    // 0x3c80c: mov r7, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,360)?)?;
    // 0x3c810: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c814: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c818: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c81c: add r4, r6, #808
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,808)?)?;
    // 0x3c820: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3c824: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c828: mov r4, #364
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,364)?)?;
    // 0x3c82c: ldr r7, [pc, #-0x348]       @ 0x291c <entry+0x291c>
    c.literal_cond(C::Al,R::new(7)?,0x3c4ec)?;
    // 0x3c830: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c834: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c838: mov r4, #368
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,368)?)?;
    // 0x3c83c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c840: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c844: mov r4, #372
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,372)?)?;
    // 0x3c848: mov r7, #376
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,376)?)?;
    // 0x3c84c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c850: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c854: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c858: add r4, r6, #812
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,812)?)?;
    // 0x3c85c: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3c860: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c864: ldr r7, [pc, #-0x37c]       @ 0x2920 <entry+0x2920>
    c.literal_cond(C::Al,R::new(7)?,0x3c4f0)?;
    // 0x3c868: mov r4, #380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,380)?)?;
    // 0x3c86c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c870: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c874: mov r4, #4480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4480)?)?;
    // 0x3c878: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c87c: mov r4, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,388)?)?;
    // 0x3c880: mov r7, #392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,392)?)?;
    // 0x3c884: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c888: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c88c: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c890: add r4, r6, #3888
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,3888)?)?;
    // 0x3c894: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c898: mov r4, #396
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,396)?)?;
    // 0x3c89c: ldr r7, [pc, #-0x3b0]       @ 0x2924 <entry+0x2924>
    c.literal_cond(C::Al,R::new(7)?,0x3c4f4)?;
    // 0x3c8a0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c8a4: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c8a8: mov r4, #400
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,400)?)?;
    // 0x3c8ac: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c8b0: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c8b4: mov r4, #404
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,404)?)?;
    // 0x3c8b8: mov r7, #408
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,408)?)?;
    // 0x3c8bc: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c8c0: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c8c4: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c8c8: add r4, r6, #820
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,820)?)?;
    // 0x3c8cc: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3c8d0: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c8d4: mov r4, #412
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,412)?)?;
    // 0x3c8d8: ldr r7, [pc, #-0x3e8]       @ 0x2928 <entry+0x2928>
    c.literal_cond(C::Al,R::new(7)?,0x3c4f8)?;
    // 0x3c8dc: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c8e0: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c8e4: mov r4, #416
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,416)?)?;
    // 0x3c8e8: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c8ec: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c8f0: mov r4, #420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,420)?)?;
    // 0x3c8f4: mov r7, #424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,424)?)?;
    // 0x3c8f8: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c8fc: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c900: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c904: add r4, r6, #824
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,824)?)?;
    // 0x3c908: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3c90c: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c910: mov r4, #428
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,428)?)?;
    // 0x3c914: ldr r7, [pc, #-0x420]       @ 0x292c <entry+0x292c>
    c.literal_cond(C::Al,R::new(7)?,0x3c4fc)?;
    // 0x3c918: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c91c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c920: mov r4, #432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,432)?)?;
    // 0x3c924: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c928: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c92c: mov r4, #436
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,436)?)?;
    // 0x3c930: mov r7, #440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,440)?)?;
    // 0x3c934: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c938: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c93c: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c940: add r4, r6, #828
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,828)?)?;
    // 0x3c944: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3c948: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c94c: ldr r7, [pc, #-0x454]       @ 0x2930 <entry+0x2930>
    c.literal_cond(C::Al,R::new(7)?,0x3c500)?;
    // 0x3c950: mov r4, #444
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,444)?)?;
    // 0x3c954: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c958: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c95c: mov r4, #4544
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4544)?)?;
    // 0x3c960: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c964: mov r4, #452
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,452)?)?;
    // 0x3c968: mov r7, #456
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,456)?)?;
    // 0x3c96c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c970: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c974: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c978: add r4, r6, #892
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,892)?)?;
    // 0x3c97c: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3c980: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c984: mov r4, #460
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,460)?)?;
    // 0x3c988: mov r7, #464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,464)?)?;
    // 0x3c98c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c990: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c994: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c998: ldr r4, [pc, #-0x49c]       @ 0x2934 <entry+0x2934>
    c.literal_cond(C::Al,R::new(4)?,0x3c504)?;
    // 0x3c99c: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c9a0: mov r4, #468
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,468)?)?;
    // 0x3c9a4: mov r7, #472
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,472)?)?;
    // 0x3c9a8: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c9ac: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c9b0: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c9b4: add r4, r6, #121
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,121)?)?;
    // 0x3c9b8: add r4, r4, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3840)?)?;
    // 0x3c9bc: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c9c0: mov r4, #476
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,476)?)?;
    // 0x3c9c4: mov r7, #480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,480)?)?;
    // 0x3c9c8: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c9cc: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c9d0: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c9d4: ldr r4, [pc, #-0x4d4]       @ 0x2938 <entry+0x2938>
    c.literal_cond(C::Al,R::new(4)?,0x3c508)?;
    // 0x3c9d8: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c9dc: mov r4, #484
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,484)?)?;
    // 0x3c9e0: mov r7, #488
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,488)?)?;
    // 0x3c9e4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c9e8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3c9ec: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c9f0: add r4, r6, #113
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,113)?)?;
    // 0x3c9f4: add r4, r4, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3840)?)?;
    // 0x3c9f8: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3c9fc: mov r4, #492
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,492)?)?;
    // 0x3ca00: mov r7, #496
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,496)?)?;
    // 0x3ca04: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3ca08: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ca0c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3ca10: ldr r4, [pc, #-0x50c]       @ 0x293c <entry+0x293c>
    c.literal_cond(C::Al,R::new(4)?,0x3c50c)?;
    // 0x3ca14: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ca18: mov r4, #500
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,500)?)?;
    // 0x3ca1c: mov r7, #504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,504)?)?;
    // 0x3ca20: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3ca24: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ca28: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3ca2c: add r4, r6, #114
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,114)?)?;
    // 0x3ca30: add r4, r4, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3840)?)?;
    // 0x3ca34: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ca38: ldr r7, [pc, #-0x530]       @ 0x2940 <entry+0x2940>
    c.literal_cond(C::Al,R::new(7)?,0x3c510)?;
    // 0x3ca3c: mov r4, #508
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,508)?)?;
    // 0x3ca40: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3ca44: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3ca48: mov r4, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4608)?)?;
    // 0x3ca4c: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3ca50: mov r4, #516
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,516)?)?;
    // 0x3ca54: mov r7, #520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,520)?)?;
    // 0x3ca58: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3ca5c: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ca60: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3ca64: add r4, r6, #115
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,115)?)?;
    // 0x3ca68: add r4, r4, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3840)?)?;
    // 0x3ca6c: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ca70: mov r4, #524
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,524)?)?;
    // 0x3ca74: mov r7, #528
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,528)?)?;
    // 0x3ca78: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3ca7c: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ca80: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3ca84: ldr r4, [pc, #-0x578]       @ 0x2944 <entry+0x2944>
    c.literal_cond(C::Al,R::new(4)?,0x3c514)?;
    // 0x3ca88: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ca8c: mov r4, #532
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,532)?)?;
    // 0x3ca90: mov r7, #536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,536)?)?;
    // 0x3ca94: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3ca98: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ca9c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3caa0: add r4, r6, #884
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,884)?)?;
    // 0x3caa4: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3caa8: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3caac: mov r4, #540
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,540)?)?;
    // 0x3cab0: mov r7, #544
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,544)?)?;
    // 0x3cab4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3cab8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cabc: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cac0: ldr r4, [pc, #-0x5b0]       @ 0x2948 <entry+0x2948>
    c.literal_cond(C::Al,R::new(4)?,0x3c518)?;
    // 0x3cac4: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cac8: mov r4, #548
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,548)?)?;
    // 0x3cacc: mov r7, #552
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,552)?)?;
    // 0x3cad0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3cad4: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cad8: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cadc: add r4, r6, #964
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,964)?)?;
    // 0x3cae0: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3cae4: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cae8: mov r4, #556
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,556)?)?;
    // 0x3caec: mov r7, #560
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,560)?)?;
    // 0x3caf0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3caf4: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3caf8: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cafc: ldr r4, [pc, #-0x5e8]       @ 0x294c <entry+0x294c>
    c.literal_cond(C::Al,R::new(4)?,0x3c51c)?;
    // 0x3cb00: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cb04: mov r4, #564
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,564)?)?;
    // 0x3cb08: mov r7, #568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,568)?)?;
    // 0x3cb0c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3cb10: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cb14: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cb18: add r4, r6, #968
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,968)?)?;
    // 0x3cb1c: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3cb20: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cb24: ldr r7, [pc, #-0x60c]       @ 0x2950 <entry+0x2950>
    c.literal_cond(C::Al,R::new(7)?,0x3c520)?;
    // 0x3cb28: mov r4, #572
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,572)?)?;
    // 0x3cb2c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3cb30: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cb34: mov r4, #4672
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4672)?)?;
    // 0x3cb38: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cb3c: mov r4, #580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,580)?)?;
    // 0x3cb40: mov r7, #584
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,584)?)?;
    // 0x3cb44: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3cb48: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cb4c: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cb50: add r4, r6, #972
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,972)?)?;
    // 0x3cb54: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3cb58: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cb5c: mov r4, #588
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,588)?)?;
    // 0x3cb60: mov r7, #592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,592)?)?;
    // 0x3cb64: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3cb68: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cb6c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cb70: ldr r4, [pc, #-0x654]       @ 0x2954 <entry+0x2954>
    c.literal_cond(C::Al,R::new(4)?,0x3c524)?;
    // 0x3cb74: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cb78: mov r4, #596
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,596)?)?;
    // 0x3cb7c: mov r7, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,616)?)?;
    // 0x3cb80: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3cb84: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cb88: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cb8c: mov r4, #600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,600)?)?;
    // 0x3cb90: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3cb94: str r2, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cb98: mov r2, #604
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,604)?)?;
    // 0x3cb9c: mov r4, #608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,608)?)?;
    // 0x3cba0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cba4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3cba8: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cbac: ldr r2, [pc, #-0x68c]       @ 0x2958 <entry+0x2958>
    c.literal_cond(C::Al,R::R2,0x3c528)?;
    // 0x3cbb0: str r2, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3cbb4: mov r2, #612
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,612)?)?;
    // 0x3cbb8: ldr r4, [pc, #-0x694]       @ 0x295c <entry+0x295c>
    c.literal_cond(C::Al,R::new(4)?,0x3c52c)?;
    // 0x3cbbc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cbc0: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cbc4: add r2, r6, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x3cbc8: mov r12, #856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,856)?)?;
    // 0x3cbcc: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cbd0: mov r2, #620
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,620)?)?;
    // 0x3cbd4: mov r7, #624
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,624)?)?;
    // 0x3cbd8: orr r12, r12, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4096)?)?;
    // 0x3cbdc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cbe0: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cbe4: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cbe8: ldr r2, [pc, #-0x6c0]       @ 0x2960 <entry+0x2960>
    c.literal_cond(C::Al,R::R2,0x3c530)?;
    // 0x3cbec: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cbf0: mov r2, #628
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,628)?)?;
    // 0x3cbf4: mov r7, #632
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,632)?)?;
    // 0x3cbf8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cbfc: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cc00: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cc04: add r2, r6, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x3cc08: mov lr, #920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,920)?)?;
    // 0x3cc0c: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cc10: ldr r7, [pc, #-0x6e4]       @ 0x2964 <entry+0x2964>
    c.literal_cond(C::Al,R::new(7)?,0x3c534)?;
    // 0x3cc14: mov r2, #636
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,636)?)?;
    // 0x3cc18: orr lr, lr, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,4096)?)?;
    // 0x3cc1c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cc20: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cc24: mov r2, #4736
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4736)?)?;
    // 0x3cc28: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cc2c: mov r2, #644
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,644)?)?;
    // 0x3cc30: mov r7, #648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,648)?)?;
    // 0x3cc34: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cc38: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cc3c: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cc40: add r2, r6, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3cc44: mov r5, #928
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,928)?)?;
    // 0x3cc48: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cc4c: mov r2, #652
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,652)?)?;
    // 0x3cc50: mov r7, #656
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,656)?)?;
    // 0x3cc54: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x3cc58: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cc5c: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cc60: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cc64: ldr r2, [pc, #-0x734]       @ 0x2968 <entry+0x2968>
    c.literal_cond(C::Al,R::R2,0x3c538)?;
    // 0x3cc68: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cc6c: mov r2, #660
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,660)?)?;
    // 0x3cc70: mov r7, #664
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,664)?)?;
    // 0x3cc74: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cc78: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cc7c: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cc80: add r2, r6, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3cc84: mov r8, #940
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,940)?)?;
    // 0x3cc88: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cc8c: mov r2, #668
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,668)?)?;
    // 0x3cc90: mov r7, #672
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,672)?)?;
    // 0x3cc94: orr r8, r8, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,4096)?)?;
    // 0x3cc98: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cc9c: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cca0: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cca4: ldr r2, [pc, #0xfdc]        @ 0x40b8 <entry+0x40b8>
    c.literal_cond(C::Al,R::R2,0x3dc88)?;
    // 0x3cca8: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ccac: mov r2, #676
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,676)?)?;
    // 0x3ccb0: mov r7, #680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,680)?)?;
    // 0x3ccb4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ccb8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ccbc: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3ccc0: mov r2, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,104)?)?;
    // 0x3ccc4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ccc8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3cccc: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ccd0: mov r2, #684
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,684)?)?;
    // 0x3ccd4: mov r7, #688
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,688)?)?;
    // 0x3ccd8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ccdc: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cce0: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cce4: ldr r2, [pc, #0xff4]        @ 0x4110 <entry+0x4110>
    c.literal_cond(C::Al,R::R2,0x3dce0)?;
    // 0x3cce8: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ccec: mov r2, #692
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,692)?)?;
    // 0x3ccf0: mov r7, #696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,696)?)?;
    // 0x3ccf4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ccf8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ccfc: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cd00: mov r2, #108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,108)?)?;
    // 0x3cd04: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cd08: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3cd0c: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cd10: ldr r7, [pc, #0xfec]        @ 0x4134 <entry+0x4134>
    c.literal_cond(C::Al,R::new(7)?,0x3dd04)?;
    // 0x3cd14: mov r2, #700
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,700)?)?;
    // 0x3cd18: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cd1c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cd20: mov r2, #4800
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4800)?)?;
    // 0x3cd24: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cd28: mov r2, #708
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,708)?)?;
    // 0x3cd2c: mov r7, #712
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,712)?)?;
    // 0x3cd30: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cd34: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cd38: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cd3c: mov r2, #112
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,112)?)?;
    // 0x3cd40: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cd44: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3cd48: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cd4c: mov r2, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,716)?)?;
    // 0x3cd50: mov r7, #720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,720)?)?;
    // 0x3cd54: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cd58: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cd5c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cd60: ldr r2, [pc, #0xff4]        @ 0x418c <entry+0x418c>
    c.literal_cond(C::Al,R::R2,0x3dd5c)?;
    // 0x3cd64: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cd68: mov r2, #724
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,724)?)?;
    // 0x3cd6c: mov r7, #728
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,728)?)?;
    // 0x3cd70: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cd74: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cd78: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cd7c: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // 0x3cd80: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cd84: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3cd88: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cd8c: mov r2, #732
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,732)?)?;
    // 0x3cd90: mov r7, #736
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,736)?)?;
    // 0x3cd94: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cd98: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cd9c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cda0: ldr r2, [pc, #0xfb8]        @ 0x4190 <entry+0x4190>
    c.literal_cond(C::Al,R::R2,0x3dd60)?;
    // 0x3cda4: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cda8: mov r2, #740
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,740)?)?;
    // 0x3cdac: mov r7, #744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,744)?)?;
    // 0x3cdb0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cdb4: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cdb8: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cdbc: add r2, r6, #4224
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,4224)?)?;
    // 0x3cdc0: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cdc4: mov r2, #748
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,748)?)?;
    // 0x3cdc8: ldr r7, [pc, #0xff8]        @ 0x41f8 <entry+0x41f8>
    c.literal_cond(C::Al,R::new(7)?,0x3ddc8)?;
    // 0x3cdcc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cdd0: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cdd4: mov r2, #752
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,752)?)?;
    // 0x3cdd8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cddc: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cde0: mov r2, #756
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,756)?)?;
    // 0x3cde4: mov r7, #760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,760)?)?;
    // 0x3cde8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cdec: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cdf0: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cdf4: mov r2, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // 0x3cdf8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cdfc: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3ce00: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ce04: ldr r7, [pc, #0xfec]        @ 0x4228 <entry+0x4228>
    c.literal_cond(C::Al,R::new(7)?,0x3ddf8)?;
    // 0x3ce08: mov r2, #764
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,764)?)?;
    // 0x3ce0c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ce10: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3ce14: mov r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4864)?)?;
    // 0x3ce18: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3ce1c: mov r2, #772
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,772)?)?;
    // 0x3ce20: mov r7, #776
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,776)?)?;
    // 0x3ce24: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ce28: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ce2c: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3ce30: mov r2, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,120)?)?;
    // 0x3ce34: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ce38: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3ce3c: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ce40: mov r2, #780
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,780)?)?;
    // 0x3ce44: mov r7, #784
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,784)?)?;
    // 0x3ce48: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ce4c: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ce50: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3ce54: ldr r2, [pc, #0xfec]        @ 0x4278 <entry+0x4278>
    c.literal_cond(C::Al,R::R2,0x3de48)?;
    // 0x3ce58: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ce5c: mov r2, #788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,788)?)?;
    // 0x3ce60: mov r7, #792
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,792)?)?;
    // 0x3ce64: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ce68: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ce6c: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3ce70: mov r2, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,124)?)?;
    // 0x3ce74: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ce78: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3ce7c: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ce80: mov r2, #796
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,796)?)?;
    // 0x3ce84: mov r7, #800
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,800)?)?;
    // 0x3ce88: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ce8c: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ce90: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3ce94: ldr r2, [pc, #0xff0]        @ 0x42bc <entry+0x42bc>
    c.literal_cond(C::Al,R::R2,0x3de8c)?;
    // 0x3ce98: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3ce9c: mov r2, #804
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,804)?)?;
    // 0x3cea0: mov r7, #808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,808)?)?;
    // 0x3cea4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cea8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ceac: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3ceb0: mov r2, #144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,144)?)?;
    // 0x3ceb4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ceb8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3cebc: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cec0: mov r2, #812
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,812)?)?;
    // 0x3cec4: mov r7, #816
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,816)?)?;
    // 0x3cec8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cecc: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ced0: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3ced4: ldr r2, [pc, #0xfb4]        @ 0x42c0 <entry+0x42c0>
    c.literal_cond(C::Al,R::R2,0x3de90)?;
    // 0x3ced8: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cedc: mov r2, #820
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,820)?)?;
    // 0x3cee0: mov r7, #824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,824)?)?;
    // 0x3cee4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cee8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3ceec: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cef0: mov r2, #148
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,148)?)?;
    // 0x3cef4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cef8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3cefc: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3cf00: ldr r7, [pc, #0xff4]        @ 0x432c <entry+0x432c>
    c.literal_cond(C::Al,R::new(7)?,0x3defc)?;
    // 0x3cf04: mov r2, #828
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,828)?)?;
    // 0x3cf08: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cf0c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cf10: mov r2, #4928
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4928)?)?;
    // 0x3cf14: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cf18: mov r2, #836
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,836)?)?;
    // 0x3cf1c: mov r7, #952
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,952)?)?;
    // 0x3cf20: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cf24: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3cf28: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cf2c: mov r2, #136
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,136)?)?;
    // 0x3cf30: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cf34: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3cf38: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3cf3c: mov r2, #844
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,844)?)?;
    // 0x3cf40: mov r3, #848
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,848)?)?;
    // 0x3cf44: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cf48: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3cf4c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cf50: ldr r2, [pc, #0xfa8]        @ 0x4330 <entry+0x4330>
    c.literal_cond(C::Al,R::R2,0x3df00)?;
    // 0x3cf54: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3cf58: mov r2, #852
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,852)?)?;
    // 0x3cf5c: mov r3, #864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,864)?)?;
    // 0x3cf60: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cf64: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3cf68: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cf6c: mov r2, #140
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,140)?)?;
    // 0x3cf70: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cf74: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3cf78: str r2, [r10, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::R12,Shift::Lsl,0)?)?;
    // 0x3cf7c: mov r2, #860
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,860)?)?;
    // 0x3cf80: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cf84: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cf88: ldr r2, [pc, #0xf74]        @ 0x4334 <entry+0x4334>
    c.literal_cond(C::Al,R::R2,0x3df04)?;
    // 0x3cf8c: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3cf90: mov r2, #868
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,868)?)?;
    // 0x3cf94: mov r3, #872
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,872)?)?;
    // 0x3cf98: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cf9c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3cfa0: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cfa4: mov r2, #152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,152)?)?;
    // 0x3cfa8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cfac: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3cfb0: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3cfb4: mov r2, #876
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,876)?)?;
    // 0x3cfb8: ldr r3, [pc, #0xf48]        @ 0x4338 <entry+0x4338>
    c.literal_cond(C::Al,R::new(3)?,0x3df08)?;
    // 0x3cfbc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cfc0: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cfc4: mov r2, #880
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,880)?)?;
    // 0x3cfc8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cfcc: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cfd0: mov r2, #884
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,884)?)?;
    // 0x3cfd4: mov r3, #888
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,888)?)?;
    // 0x3cfd8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cfdc: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3cfe0: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3cfe4: mov r2, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,156)?)?;
    // 0x3cfe8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3cfec: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3cff0: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3cff4: ldr r3, [pc, #0xf10]        @ 0x433c <entry+0x433c>
    c.literal_cond(C::Al,R::new(3)?,0x3df0c)?;
    // 0x3cff8: mov r2, #892
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,892)?)?;
    // 0x3cffc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d000: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d004: mov r2, #4992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4992)?)?;
    // 0x3d008: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d00c: mov r2, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,900)?)?;
    // 0x3d010: mov r3, #904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,904)?)?;
    // 0x3d014: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d018: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3d01c: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d020: mov r2, #160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,160)?)?;
    // 0x3d024: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d028: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3d02c: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d030: mov r2, #908
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,908)?)?;
    // 0x3d034: mov r3, #912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,912)?)?;
    // 0x3d038: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d03c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3d040: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d044: ldr r2, [pc, #0xec4]        @ 0x4340 <entry+0x4340>
    c.literal_cond(C::Al,R::R2,0x3df10)?;
    // 0x3d048: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d04c: mov r3, #164
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,164)?)?;
    // 0x3d050: mov r2, #916
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,916)?)?;
    // 0x3d054: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3d058: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d05c: add r3, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3d060: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d064: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3d068: str r3, [r10, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::LR,Shift::Lsl,0)?)?;
    // 0x3d06c: mov r3, #924
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,924)?)?;
    // 0x3d070: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3d074: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d078: str r4, [r10, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x3d07c: mov r4, #932
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,932)?)?;
    // 0x3d080: mov r5, #936
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,936)?)?;
    // 0x3d084: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d088: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x3d08c: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d090: mov r4, #168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,168)?)?;
    // 0x3d094: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d098: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d09c: str r4, [r10, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x3d0a0: ldr r4, [pc, #0xe6c]        @ 0x4344 <entry+0x4344>
    c.literal_cond(C::Al,R::new(4)?,0x3df14)?;
    // 0x3d0a4: mov r5, #944
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,944)?)?;
    // 0x3d0a8: str r0, [r10, r8]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x3d0ac: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x3d0b0: str r4, [r10, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x3d0b4: mov r4, #948
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,948)?)?;
    // 0x3d0b8: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d0bc: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d0c0: mov r4, #172
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,172)?)?;
    // 0x3d0c4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d0c8: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d0cc: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d0d0: mov r4, #956
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,956)?)?;
    // 0x3d0d4: mov r7, #968
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,968)?)?;
    // 0x3d0d8: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d0dc: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3d0e0: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d0e4: mov r4, #5056
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5056)?)?;
    // 0x3d0e8: str r9, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(9)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d0ec: mov r4, #964
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,964)?)?;
    // 0x3d0f0: mov r9, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,10)?)?;
    // 0x3d0f4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d0f8: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d0fc: mov r4, #176
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,176)?)?;
    // 0x3d100: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d104: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d108: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d10c: mov r4, #972
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,972)?)?;
    // 0x3d110: ldr r7, [pc, #0xe00]        @ 0x4348 <entry+0x4348>
    c.literal_cond(C::Al,R::new(7)?,0x3df18)?;
    // 0x3d114: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d118: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d11c: mov r4, #976
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,976)?)?;
    // 0x3d120: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d124: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d128: mov r4, #980
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,980)?)?;
    // 0x3d12c: mov r7, #984
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,984)?)?;
    // 0x3d130: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d134: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3d138: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d13c: mov r4, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,180)?)?;
    // 0x3d140: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d144: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d148: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d14c: mov r4, #988
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,988)?)?;
    // 0x3d150: ldr r7, [pc, #0xdc4]        @ 0x434c <entry+0x434c>
    c.literal_cond(C::Al,R::new(7)?,0x3df1c)?;
    // 0x3d154: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d158: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d15c: mov r4, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,992)?)?;
    // 0x3d160: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d164: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d168: mov r4, #996
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,996)?)?;
    // 0x3d16c: mov r7, #1000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1000)?)?;
    // 0x3d170: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d174: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3d178: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d17c: mov r4, #184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,184)?)?;
    // 0x3d180: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d184: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d188: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d18c: mov r4, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1004)?)?;
    // 0x3d190: ldr r7, [pc, #0xd88]        @ 0x4350 <entry+0x4350>
    c.literal_cond(C::Al,R::new(7)?,0x3df20)?;
    // 0x3d194: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d198: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d19c: mov r4, #1008
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1008)?)?;
    // 0x3d1a0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d1a4: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d1a8: mov r4, #1012
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1012)?)?;
    // 0x3d1ac: mov r7, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1016)?)?;
    // 0x3d1b0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d1b4: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3d1b8: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d1bc: mov r4, #188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,188)?)?;
    // 0x3d1c0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d1c4: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d1c8: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d1cc: ldr r7, [pc, #0xd50]        @ 0x4354 <entry+0x4354>
    c.literal_cond(C::Al,R::new(7)?,0x3df24)?;
    // 0x3d1d0: mov r4, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1020)?)?;
    // 0x3d1d4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d1d8: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d1dc: mov r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5120)?)?;
    // 0x3d1e0: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d1e4: mov r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x3d1e8: mov r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // 0x3d1ec: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d1f0: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d1f4: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d1f8: add r4, r6, #4288
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,4288)?)?;
    // 0x3d1fc: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d200: mov r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,12)?)?;
    // 0x3d204: ldr r7, [pc, #0xd1c]        @ 0x4358 <entry+0x4358>
    c.literal_cond(C::Al,R::new(7)?,0x3df28)?;
    // 0x3d208: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d20c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d210: mov r4, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1040)?)?;
    // 0x3d214: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d218: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d21c: mov r4, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,20)?)?;
    // 0x3d220: mov r7, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,24)?)?;
    // 0x3d224: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d228: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d22c: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d230: mov r4, #196
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,196)?)?;
    // 0x3d234: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d238: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d23c: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d240: mov r4, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,28)?)?;
    // 0x3d244: ldr r7, [pc, #0xce0]        @ 0x435c <entry+0x435c>
    c.literal_cond(C::Al,R::new(7)?,0x3df2c)?;
    // 0x3d248: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d24c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d250: mov r4, #1056
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1056)?)?;
    // 0x3d254: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d258: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d25c: mov r4, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,36)?)?;
    // 0x3d260: mov r7, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,40)?)?;
    // 0x3d264: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d268: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d26c: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d270: mov r4, #200
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,200)?)?;
    // 0x3d274: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d278: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d27c: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d280: mov r4, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,44)?)?;
    // 0x3d284: ldr r7, [pc, #0xca4]        @ 0x4360 <entry+0x4360>
    c.literal_cond(C::Al,R::new(7)?,0x3df30)?;
    // 0x3d288: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d28c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d290: mov r4, #1072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1072)?)?;
    // 0x3d294: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d298: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d29c: mov r4, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,52)?)?;
    // 0x3d2a0: mov r7, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,56)?)?;
    // 0x3d2a4: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d2a8: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d2ac: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d2b0: mov r4, #204
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,204)?)?;
    // 0x3d2b4: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d2b8: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d2bc: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d2c0: ldr r7, [pc, #0xc6c]        @ 0x4364 <entry+0x4364>
    c.literal_cond(C::Al,R::new(7)?,0x3df34)?;
    // 0x3d2c4: mov r4, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,60)?)?;
    // 0x3d2c8: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d2cc: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d2d0: mov r4, #5184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5184)?)?;
    // 0x3d2d4: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d2d8: mov r4, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,68)?)?;
    // 0x3d2dc: mov r7, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,72)?)?;
    // 0x3d2e0: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d2e4: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d2e8: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d2ec: mov r4, #208
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,208)?)?;
    // 0x3d2f0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d2f4: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d2f8: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d2fc: mov r4, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,76)?)?;
    // 0x3d300: ldr r7, [pc, #0xc30]        @ 0x4368 <entry+0x4368>
    c.literal_cond(C::Al,R::new(7)?,0x3df38)?;
    // 0x3d304: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d308: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d30c: mov r4, #1104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1104)?)?;
    // 0x3d310: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d314: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d318: mov r4, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,84)?)?;
    // 0x3d31c: mov r7, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,88)?)?;
    // 0x3d320: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d324: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d328: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d32c: mov r4, #212
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,212)?)?;
    // 0x3d330: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d334: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d338: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d33c: mov r4, #92
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,92)?)?;
    // 0x3d340: ldr r7, [pc, #0xbf4]        @ 0x436c <entry+0x436c>
    c.literal_cond(C::Al,R::new(7)?,0x3df3c)?;
    // 0x3d344: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d348: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d34c: mov r4, #1120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1120)?)?;
    // 0x3d350: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d354: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d358: mov r4, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,100)?)?;
    // 0x3d35c: mov r7, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,104)?)?;
    // 0x3d360: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d364: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d368: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d36c: mov r4, #220
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,220)?)?;
    // 0x3d370: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d374: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d378: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d37c: mov r4, #108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,108)?)?;
    // 0x3d380: ldr r7, [pc, #0xbb8]        @ 0x4370 <entry+0x4370>
    c.literal_cond(C::Al,R::new(7)?,0x3df40)?;
    // 0x3d384: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d388: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d38c: mov r4, #1136
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1136)?)?;
    // 0x3d390: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d394: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d398: mov r4, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,116)?)?;
    // 0x3d39c: mov r7, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,120)?)?;
    // 0x3d3a0: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d3a4: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d3a8: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d3ac: mov r4, #216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,216)?)?;
    // 0x3d3b0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d3b4: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d3b8: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d3bc: ldr r7, [pc, #0xb80]        @ 0x4374 <entry+0x4374>
    c.literal_cond(C::Al,R::new(7)?,0x3df44)?;
    // 0x3d3c0: mov r4, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,124)?)?;
    // 0x3d3c4: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d3c8: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d3cc: mov r4, #5248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5248)?)?;
    // 0x3d3d0: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d3d4: mov r4, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,132)?)?;
    // 0x3d3d8: mov r7, #136
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,136)?)?;
    // 0x3d3dc: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d3e0: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d3e4: str r1, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d3e8: mov r4, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,56)?)?;
    // 0x3d3ec: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d3f0: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3d3f4: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d3f8: mov r4, #140
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,140)?)?;
    // 0x3d3fc: ldr r7, [pc, #0xb44]        @ 0x4378 <entry+0x4378>
    c.literal_cond(C::Al,R::new(7)?,0x3df48)?;
    // 0x3d400: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d404: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d408: mov r4, #1168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1168)?)?;
    // 0x3d40c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d410: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d414: mov r4, #148
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,148)?)?;
    // 0x3d418: mov r7, #152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,152)?)?;
    // 0x3d41c: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d420: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d424: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d428: add r4, r6, #3728
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,3728)?)?;
    // 0x3d42c: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d430: mov r4, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,156)?)?;
    // 0x3d434: ldr r7, [pc, #0xb10]        @ 0x437c <entry+0x437c>
    c.literal_cond(C::Al,R::new(7)?,0x3df4c)?;
    // 0x3d438: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d43c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d440: mov r4, #1184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1184)?)?;
    // 0x3d444: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d448: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d44c: mov r4, #164
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,164)?)?;
    // 0x3d450: mov r7, #168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,168)?)?;
    // 0x3d454: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d458: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d45c: str r9, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(9)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d460: add r4, r6, #135
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,135)?)?;
    // 0x3d464: add r4, r4, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3584)?)?;
    // 0x3d468: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d46c: mov r4, #172
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,172)?)?;
    // 0x3d470: ldr r7, [pc, #0xad8]        @ 0x4380 <entry+0x4380>
    c.literal_cond(C::Al,R::new(7)?,0x3df50)?;
    // 0x3d474: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d478: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d47c: mov r4, #1200
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1200)?)?;
    // 0x3d480: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d484: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d488: mov r4, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,180)?)?;
    // 0x3d48c: mov r7, #184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,184)?)?;
    // 0x3d490: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d494: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d498: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d49c: add r4, r6, #648
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,648)?)?;
    // 0x3d4a0: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3d4a4: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d4a8: ldr r7, [pc, #0xaa4]        @ 0x4384 <entry+0x4384>
    c.literal_cond(C::Al,R::new(7)?,0x3df54)?;
    // 0x3d4ac: mov r4, #188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,188)?)?;
    // 0x3d4b0: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d4b4: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d4b8: mov r4, #5312
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5312)?)?;
    // 0x3d4bc: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d4c0: mov r4, #196
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,196)?)?;
    // 0x3d4c4: mov r7, #200
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,200)?)?;
    // 0x3d4c8: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d4cc: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d4d0: str r9, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(9)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d4d4: add r4, r6, #137
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,137)?)?;
    // 0x3d4d8: mov r9, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,10)?)?;
    // 0x3d4dc: add r4, r4, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3584)?)?;
    // 0x3d4e0: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d4e4: mov r4, #204
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,204)?)?;
    // 0x3d4e8: ldr r7, [pc, #0xa68]        @ 0x4388 <entry+0x4388>
    c.literal_cond(C::Al,R::new(7)?,0x3df58)?;
    // 0x3d4ec: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d4f0: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d4f4: mov r4, #1232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1232)?)?;
    // 0x3d4f8: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d4fc: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d500: mov r4, #212
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,212)?)?;
    // 0x3d504: mov r7, #216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,216)?)?;
    // 0x3d508: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d50c: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d510: str r9, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(9)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d514: add r4, r6, #484
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,484)?)?;
    // 0x3d518: add r4, r4, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3072)?)?;
    // 0x3d51c: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d520: mov r4, #220
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,220)?)?;
    // 0x3d524: ldr r7, [pc, #0xa30]        @ 0x438c <entry+0x438c>
    c.literal_cond(C::Al,R::new(7)?,0x3df5c)?;
    // 0x3d528: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d52c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d530: mov r4, #1248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1248)?)?;
    // 0x3d534: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d538: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d53c: mov r4, #228
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,228)?)?;
    // 0x3d540: mov r7, #232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,232)?)?;
    // 0x3d544: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d548: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d54c: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d550: add r4, r6, #3568
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,3568)?)?;
    // 0x3d554: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d558: mov r4, #236
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,236)?)?;
    // 0x3d55c: ldr r7, [pc, #0x9fc]        @ 0x4390 <entry+0x4390>
    c.literal_cond(C::Al,R::new(7)?,0x3df60)?;
    // 0x3d560: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d564: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d568: mov r4, #1264
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1264)?)?;
    // 0x3d56c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3d570: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d574: mov r4, #244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,244)?)?;
    // 0x3d578: mov r7, #248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,248)?)?;
    // 0x3d57c: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d580: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d584: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d588: add r4, r6, #241
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(6)?,241)?)?;
    // 0x3d58c: add r4, r4, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,3328)?)?;
    // 0x3d590: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d594: ldr r7, [pc, #0x9c8]        @ 0x4394 <entry+0x4394>
    c.literal_cond(C::Al,R::new(7)?,0x3df64)?;
    // 0x3d598: mov r4, #252
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,252)?)?;
    // 0x3d59c: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d5a0: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d5a4: mov r4, #5376
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5376)?)?;
    // 0x3d5a8: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d5ac: mov r4, #260
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,260)?)?;
    // 0x3d5b0: add r7, r6, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x3d5b4: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d5b8: str r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d5bc: mov r4, #264
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,264)?)?;
    // 0x3d5c0: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d5c4: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d5c8: mov r7, #268
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,268)?)?;
    // 0x3d5cc: ldr r4, [pc, #0x994]        @ 0x4398 <entry+0x4398>
    c.literal_cond(C::Al,R::new(4)?,0x3df68)?;
    // 0x3d5d0: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d5d4: str r1, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d5d8: mov r7, #1296
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1296)?)?;
    // 0x3d5dc: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3d5e0: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d5e4: mov r7, #276
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,276)?)?;
    // 0x3d5e8: mov r4, #280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,280)?)?;
    // 0x3d5ec: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d5f0: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x3d5f4: str r0, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d5f8: add r7, r6, #147
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(6)?,147)?)?;
    // 0x3d5fc: add r7, r7, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,4864)?)?;
    // 0x3d600: str r7, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3d604: mov r7, #284
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,284)?)?;
    // 0x3d608: ldr r4, [pc, #0x95c]        @ 0x439c <entry+0x439c>
    c.literal_cond(C::Al,R::new(4)?,0x3df6c)?;
    // 0x3d60c: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d610: str r0, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d614: mov r7, #1312
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1312)?)?;
    // 0x3d618: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3d61c: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d620: mov r7, #292
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,292)?)?;
    // 0x3d624: mov r4, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,7)?)?;
    // 0x3d628: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d62c: str r4, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d630: mov r7, #296
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,296)?)?;
    // 0x3d634: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x3d638: str r2, [r10, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x3d63c: mov r2, #300
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,300)?)?;
    // 0x3d640: ldr r7, [pc, #0x928]        @ 0x43a0 <entry+0x43a0>
    c.literal_cond(C::Al,R::new(7)?,0x3df70)?;
    // 0x3d644: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d648: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d64c: mov r2, #1328
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1328)?)?;
    // 0x3d650: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d654: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d658: mov r2, #308
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,308)?)?;
    // 0x3d65c: mov r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,7)?)?;
    // 0x3d660: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d664: str r4, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d668: add r2, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3d66c: mov r3, #312
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,312)?)?;
    // 0x3d670: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d674: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d678: ldr r3, [pc, #0x8f4]        @ 0x43a4 <entry+0x43a4>
    c.literal_cond(C::Al,R::new(3)?,0x3df74)?;
    // 0x3d67c: mov r2, #316
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,316)?)?;
    // 0x3d680: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d684: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d688: mov r2, #5440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5440)?)?;
    // 0x3d68c: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d690: mov r2, #324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,324)?)?;
    // 0x3d694: mov r3, #328
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,328)?)?;
    // 0x3d698: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d69c: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d6a0: str r4, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d6a4: add r2, r6, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x3d6a8: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d6ac: mov r2, #332
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,332)?)?;
    // 0x3d6b0: ldr r3, [pc, #0x8c0]        @ 0x43a8 <entry+0x43a8>
    c.literal_cond(C::Al,R::new(3)?,0x3df78)?;
    // 0x3d6b4: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d6b8: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d6bc: mov r2, #1360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1360)?)?;
    // 0x3d6c0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d6c4: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d6c8: mov r2, #340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,340)?)?;
    // 0x3d6cc: mov r3, #344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,344)?)?;
    // 0x3d6d0: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d6d4: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d6d8: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d6dc: mov r2, #852
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,852)?)?;
    // 0x3d6e0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d6e4: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3d6e8: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d6ec: mov r2, #348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,348)?)?;
    // 0x3d6f0: ldr r3, [pc, #0x884]        @ 0x43ac <entry+0x43ac>
    c.literal_cond(C::Al,R::new(3)?,0x3df7c)?;
    // 0x3d6f4: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d6f8: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d6fc: mov r2, #1376
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1376)?)?;
    // 0x3d700: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d704: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d708: mov r2, #356
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,356)?)?;
    // 0x3d70c: mov r3, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,360)?)?;
    // 0x3d710: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d714: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d718: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d71c: add r2, r6, #712
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,712)?)?;
    // 0x3d720: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3d724: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d728: mov r2, #364
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,364)?)?;
    // 0x3d72c: ldr r3, [pc, #0x84c]        @ 0x43b0 <entry+0x43b0>
    c.literal_cond(C::Al,R::new(3)?,0x3df80)?;
    // 0x3d730: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d734: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d738: mov r2, #1392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1392)?)?;
    // 0x3d73c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d740: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d744: mov r2, #372
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,372)?)?;
    // 0x3d748: mov r3, #376
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,376)?)?;
    // 0x3d74c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d750: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d754: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d758: add r2, r6, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3d75c: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d760: ldr r3, [pc, #0x81c]        @ 0x43b4 <entry+0x43b4>
    c.literal_cond(C::Al,R::new(3)?,0x3df84)?;
    // 0x3d764: mov r2, #380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,380)?)?;
    // 0x3d768: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d76c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d770: mov r2, #5504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5504)?)?;
    // 0x3d774: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d778: mov r2, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,388)?)?;
    // 0x3d77c: mov r3, #392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,392)?)?;
    // 0x3d780: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d784: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d788: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d78c: add r2, r6, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3d790: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d794: mov r2, #396
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,396)?)?;
    // 0x3d798: ldr r3, [pc, #0x7e8]        @ 0x43b8 <entry+0x43b8>
    c.literal_cond(C::Al,R::new(3)?,0x3df88)?;
    // 0x3d79c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d7a0: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d7a4: mov r2, #1424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1424)?)?;
    // 0x3d7a8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d7ac: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d7b0: mov r2, #404
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,404)?)?;
    // 0x3d7b4: mov r3, #408
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,408)?)?;
    // 0x3d7b8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d7bc: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d7c0: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d7c4: mov r2, #936
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,936)?)?;
    // 0x3d7c8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d7cc: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3d7d0: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d7d4: mov r2, #412
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,412)?)?;
    // 0x3d7d8: ldr r3, [pc, #0x7ac]        @ 0x43bc <entry+0x43bc>
    c.literal_cond(C::Al,R::new(3)?,0x3df8c)?;
    // 0x3d7dc: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d7e0: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d7e4: mov r2, #1440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1440)?)?;
    // 0x3d7e8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d7ec: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d7f0: mov r2, #420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,420)?)?;
    // 0x3d7f4: mov r3, #424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,424)?)?;
    // 0x3d7f8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d7fc: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d800: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d804: mov r2, #932
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,932)?)?;
    // 0x3d808: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d80c: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3d810: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d814: mov r2, #428
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,428)?)?;
    // 0x3d818: ldr r3, [pc, #0x770]        @ 0x43c0 <entry+0x43c0>
    c.literal_cond(C::Al,R::new(3)?,0x3df90)?;
    // 0x3d81c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d820: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d824: mov r2, #1456
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1456)?)?;
    // 0x3d828: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d82c: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d830: mov r2, #436
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,436)?)?;
    // 0x3d834: mov r3, #440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,440)?)?;
    // 0x3d838: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d83c: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d840: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d844: mov r2, #928
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,928)?)?;
    // 0x3d848: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d84c: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3d850: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d854: ldr r3, [pc, #0x738]        @ 0x43c4 <entry+0x43c4>
    c.literal_cond(C::Al,R::new(3)?,0x3df94)?;
    // 0x3d858: mov r2, #444
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,444)?)?;
    // 0x3d85c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d860: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d864: mov r2, #5568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5568)?)?;
    // 0x3d868: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d86c: mov r2, #452
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,452)?)?;
    // 0x3d870: mov r3, #456
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,456)?)?;
    // 0x3d874: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d878: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d87c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d880: mov r2, #948
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,948)?)?;
    // 0x3d884: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d888: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3d88c: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d890: mov r2, #460
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,460)?)?;
    // 0x3d894: ldr r3, [pc, #0x6fc]        @ 0x43c8 <entry+0x43c8>
    c.literal_cond(C::Al,R::new(3)?,0x3df98)?;
    // 0x3d898: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d89c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d8a0: mov r2, #1488
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1488)?)?;
    // 0x3d8a4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d8a8: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d8ac: mov r2, #468
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,468)?)?;
    // 0x3d8b0: mov r3, #472
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,472)?)?;
    // 0x3d8b4: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d8b8: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d8bc: str r7, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d8c0: mov r2, #952
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,952)?)?;
    // 0x3d8c4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d8c8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3d8cc: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d8d0: mov r2, #476
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,476)?)?;
    // 0x3d8d4: ldr r3, [pc, #0x6c0]        @ 0x43cc <entry+0x43cc>
    c.literal_cond(C::Al,R::new(3)?,0x3df9c)?;
    // 0x3d8d8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d8dc: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d8e0: mov r2, #1504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1504)?)?;
    // 0x3d8e4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d8e8: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d8ec: mov r2, #484
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,484)?)?;
    // 0x3d8f0: mov r3, #488
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,488)?)?;
    // 0x3d8f4: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d8f8: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d8fc: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d900: mov r2, #956
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,956)?)?;
    // 0x3d904: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d908: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3d90c: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d910: mov r2, #492
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,492)?)?;
    // 0x3d914: ldr r3, [pc, #0x684]        @ 0x43d0 <entry+0x43d0>
    c.literal_cond(C::Al,R::new(3)?,0x3dfa0)?;
    // 0x3d918: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d91c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d920: mov r2, #1520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1520)?)?;
    // 0x3d924: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d928: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d92c: mov r2, #500
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,500)?)?;
    // 0x3d930: mov r3, #504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,504)?)?;
    // 0x3d934: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d938: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d93c: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d940: mov r2, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,900)?)?;
    // 0x3d944: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d948: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3d94c: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d950: ldr r3, [pc, #0x64c]        @ 0x43d4 <entry+0x43d4>
    c.literal_cond(C::Al,R::new(3)?,0x3dfa4)?;
    // 0x3d954: mov r2, #508
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,508)?)?;
    // 0x3d958: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d95c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d960: mov r2, #5632
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5632)?)?;
    // 0x3d964: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d968: mov r2, #516
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,516)?)?;
    // 0x3d96c: mov r3, #520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,520)?)?;
    // 0x3d970: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d974: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d978: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d97c: add r2, r6, #4992
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,4992)?)?;
    // 0x3d980: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d984: mov r2, #524
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,524)?)?;
    // 0x3d988: ldr r3, [pc, #0x618]        @ 0x43d8 <entry+0x43d8>
    c.literal_cond(C::Al,R::new(3)?,0x3dfa8)?;
    // 0x3d98c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d990: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d994: mov r2, #1552
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1552)?)?;
    // 0x3d998: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d99c: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d9a0: mov r2, #532
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,532)?)?;
    // 0x3d9a4: mov r3, #536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,536)?)?;
    // 0x3d9a8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d9ac: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d9b0: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d9b4: add r2, r6, #764
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,764)?)?;
    // 0x3d9b8: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3d9bc: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d9c0: mov r2, #540
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,540)?)?;
    // 0x3d9c4: ldr r3, [pc, #0x5e0]        @ 0x43dc <entry+0x43dc>
    c.literal_cond(C::Al,R::new(3)?,0x3dfac)?;
    // 0x3d9c8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d9cc: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d9d0: mov r2, #1568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1568)?)?;
    // 0x3d9d4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3d9d8: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d9dc: mov r2, #548
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,548)?)?;
    // 0x3d9e0: mov r3, #552
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,552)?)?;
    // 0x3d9e4: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3d9e8: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3d9ec: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3d9f0: add r2, r6, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3840)?)?;
    // 0x3d9f4: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3d9f8: mov r2, #556
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,556)?)?;
    // 0x3d9fc: ldr r3, [pc, #0x5ac]        @ 0x43e0 <entry+0x43e0>
    c.literal_cond(C::Al,R::new(3)?,0x3dfb0)?;
    // 0x3da00: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3da04: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3da08: mov r2, #1584
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1584)?)?;
    // 0x3da0c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3da10: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3da14: mov r2, #564
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,564)?)?;
    // 0x3da18: mov r3, #568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,568)?)?;
    // 0x3da1c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3da20: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3da24: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3da28: add r2, r6, #772
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,772)?)?;
    // 0x3da2c: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3da30: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3da34: ldr r3, [pc, #0x578]        @ 0x43e4 <entry+0x43e4>
    c.literal_cond(C::Al,R::new(3)?,0x3dfb4)?;
    // 0x3da38: mov r2, #572
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,572)?)?;
    // 0x3da3c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3da40: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3da44: mov r2, #5696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5696)?)?;
    // 0x3da48: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3da4c: mov r2, #580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,580)?)?;
    // 0x3da50: mov r3, #584
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,584)?)?;
    // 0x3da54: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3da58: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3da5c: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3da60: add r2, r6, #776
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,776)?)?;
    // 0x3da64: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3da68: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3da6c: mov r2, #588
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,588)?)?;
    // 0x3da70: ldr r3, [pc, #0x540]        @ 0x43e8 <entry+0x43e8>
    c.literal_cond(C::Al,R::new(3)?,0x3dfb8)?;
    // 0x3da74: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3da78: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3da7c: mov r2, #1616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1616)?)?;
    // 0x3da80: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3da84: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3da88: mov r2, #596
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,596)?)?;
    // 0x3da8c: mov r3, #600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,600)?)?;
    // 0x3da90: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3da94: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3da98: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3da9c: add r2, r6, #780
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,780)?)?;
    // 0x3daa0: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3daa4: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3daa8: mov r2, #604
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,604)?)?;
    // 0x3daac: ldr r3, [pc, #0x508]        @ 0x43ec <entry+0x43ec>
    c.literal_cond(C::Al,R::new(3)?,0x3dfbc)?;
    // 0x3dab0: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3dab4: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dab8: mov r2, #1632
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1632)?)?;
    // 0x3dabc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3dac0: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dac4: mov r2, #612
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,612)?)?;
    // 0x3dac8: mov r3, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,616)?)?;
    // 0x3dacc: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3dad0: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3dad4: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dad8: add r2, r6, #3856
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3856)?)?;
    // 0x3dadc: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3dae0: mov r2, #620
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,620)?)?;
    // 0x3dae4: ldr r3, [pc, #0x4d4]        @ 0x43f0 <entry+0x43f0>
    c.literal_cond(C::Al,R::new(3)?,0x3dfc0)?;
    // 0x3dae8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3daec: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3daf0: mov r2, #1648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1648)?)?;
    // 0x3daf4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3daf8: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dafc: mov r2, #628
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,628)?)?;
    // 0x3db00: mov r3, #632
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,632)?)?;
    // 0x3db04: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3db08: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3db0c: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3db10: add r2, r6, #788
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,788)?)?;
    // 0x3db14: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3db18: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3db1c: ldr r3, [pc, #0x4a0]        @ 0x43f4 <entry+0x43f4>
    c.literal_cond(C::Al,R::new(3)?,0x3dfc4)?;
    // 0x3db20: mov r2, #636
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,636)?)?;
    // 0x3db24: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3db28: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3db2c: mov r2, #5760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5760)?)?;
    // 0x3db30: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3db34: mov r2, #644
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,644)?)?;
    // 0x3db38: mov r3, #648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,648)?)?;
    // 0x3db3c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3db40: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3db44: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3db48: add r2, r6, #792
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,792)?)?;
    // 0x3db4c: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3db50: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3db54: mov r2, #652
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,652)?)?;
    // 0x3db58: ldr r3, [pc, #0x468]        @ 0x43f8 <entry+0x43f8>
    c.literal_cond(C::Al,R::new(3)?,0x3dfc8)?;
    // 0x3db5c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3db60: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3db64: mov r2, #1680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1680)?)?;
    // 0x3db68: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3db6c: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3db70: mov r2, #660
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,660)?)?;
    // 0x3db74: mov r3, #664
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,664)?)?;
    // 0x3db78: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3db7c: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // 0x3db80: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3db84: add r2, r6, #796
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,796)?)?;
    // 0x3db88: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3db8c: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3db90: mov r2, #668
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,668)?)?;
    // 0x3db94: ldr r3, [pc, #0x430]        @ 0x43fc <entry+0x43fc>
    c.literal_cond(C::Al,R::new(3)?,0x3dfcc)?;
    // 0x3db98: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3db9c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dba0: mov r2, #1696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1696)?)?;
    // 0x3dba4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3dba8: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dbac: mov r2, #676
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,676)?)?;
    // 0x3dbb0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3dbb4: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3dbb8: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dbbc: add r1, r6, #130
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,130)?)?;
    // 0x3dbc0: mov r2, #680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,680)?)?;
    // 0x3dbc4: add r1, r1, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,3584)?)?;
    // 0x3dbc8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3dbcc: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dbd0: mov r1, #684
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,684)?)?;
    // 0x3dbd4: ldr r2, [pc, #0x3f4]        @ 0x4400 <entry+0x4400>
    c.literal_cond(C::Al,R::R2,0x3dfd0)?;
    // 0x3dbd8: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x3dbdc: str r0, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R1,Shift::Lsl,0)?)?;
    // 0x3dbe0: mov r1, #1712
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1712)?)?;
    // 0x3dbe4: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x3dbe8: str r2, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::R1,Shift::Lsl,0)?)?;
    // 0x3dbec: mov r1, #692
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,692)?)?;
    // 0x3dbf0: mov r2, #696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,696)?)?;
    // 0x3dbf4: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x3dbf8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3dbfc: str r0, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R1,Shift::Lsl,0)?)?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{populate_stream_controller_registers(c)}

#[cfg(test)]mod tests{
 use super::*;
 #[test]fn stream_controller_registers_are_adjacent(){
    let mut c=Code::new(START);emit(&mut c).unwrap();
    assert_eq!(c.bytes.len(),END-START);
 }
}
