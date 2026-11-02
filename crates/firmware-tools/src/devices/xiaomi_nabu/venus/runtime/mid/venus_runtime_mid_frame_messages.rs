// SPDX-License-Identifier: GPL-3.0-only
//! Process codec event packet and associated frame buffer allocations.
//! Validation, diagnostic and queue transitions are generated entirely
//! from typed ARM32 instructions. This remains a partial larger function.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::Code;
const START:usize=0x2ba68;
const END:usize=0x2be44;

/// Dispatch buffer-type events, report errors, and prepare new allocated
/// codec frame descriptors before invoking platform callbacks.
fn process_codec_buffer_events(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2ba68: ldr r1, [pc, #0xff4]        @ 0x2664 <entry+0x2664>
    c.literal(R::R1,0x2ca64)?;
    // 0x2ba6c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2ba70: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x2ba74: b 0x2c064 <entry+0x1c64>  @ imm = #0x5e8
    c.b(C::Al,0x2c064)?;
    // 0x2ba78: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x2ba7c: add r7, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(4)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x2ba80: bl 0x2efc <entry+0xfffffffffffd8afc> @ imm = #-0x28b8c
    c.bl(0x2efc)?;
    // 0x2ba84: str r0, [r7, #0x30]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,48)?)?;
    // 0x2ba88: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ba8c: bne 0x2baa8 <entry+0x16a8>  @ imm = #0x14
    c.b(C::Ne,0x2baa8)?;
    // 0x2ba90: ldr r2, [pc, #0xdc4]        @ 0x245c <entry+0x245c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2ba94: mov r1, #94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,94)?)?;
    // 0x2ba98: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2ba9c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2baa0: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x2a2f8
    c.bl(0x17b0)?;
    // 0x2baa4: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x2baa8: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,8)?)?;
    // 0x2baac: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2bab0: bl 0x8b1b8 <entry+0x60db8> @ imm = #0x5f700
    c.bl(0x8b1b8)?;
    // 0x2bab4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2bab8: beq 0x2bafc <entry+0x16fc>  @ imm = #0x3c
    c.b(C::Eq,0x2bafc)?;
    // 0x2babc: ldr r1, [pc, #0xfa4]        @ 0x2668 <entry+0x2668>
    c.literal(R::R1,0x2ca68)?;
    // 0x2bac0: ldr r2, [pc, #0xd34]        @ 0x23fc <entry+0x23fc>
    c.literal(R::R2,0x2c7fc)?;
    // 0x2bac4: mov r3, #101
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,101)?)?;
    // 0x2bac8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2bacc: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x2bad0: bl 0x2ec94 <entry+0x4894>  @ imm = #0x31bc
    c.bl(0x2ec94)?;
    // 0x2bad4: ldr r1, [pc, #0xa94]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2bad8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2badc: bl 0x2ec94 <entry+0x4894>  @ imm = #0x31b0
    c.bl(0x2ec94)?;
    // 0x2bae0: ldr r2, [pc, #0xd74]        @ 0x245c <entry+0x245c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2bae4: mov r1, #103
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,103)?)?;
    // 0x2bae8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2baec: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2baf0: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x2a348
    c.bl(0x17b0)?;
    // 0x2baf4: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x2baf8: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x2bafc: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x2bb00: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x2bb04: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x2bb08: ldr r1, [pc, #0xfd8]        @ 0x26e8 <entry+0x26e8>
    c.literal(R::R1,0x2cae8)?;
    // 0x2bb0c: b 0x2bba0 <entry+0x17a0>  @ imm = #0x8c
    c.b(C::Al,0x2bba0)?;
    // 0x2bb10: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x2bb14: add r7, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(4)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x2bb18: bl 0x2efc <entry+0xfffffffffffd8afc> @ imm = #-0x28c24
    c.bl(0x2efc)?;
    // 0x2bb1c: str r0, [r7, #0x30]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,48)?)?;
    // 0x2bb20: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2bb24: bne 0x2bb40 <entry+0x1740>  @ imm = #0x14
    c.b(C::Ne,0x2bb40)?;
    // 0x2bb28: ldr r2, [pc, #0xd2c]        @ 0x245c <entry+0x245c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2bb2c: mov r1, #165
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,165)?)?;
    // 0x2bb30: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2bb34: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2bb38: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x2a390
    c.bl(0x17b0)?;
    // 0x2bb3c: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x2bb40: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,8)?)?;
    // 0x2bb44: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2bb48: bl 0xa83f8 <entry+0x7dff8> @ imm = #0x7c8a8
    c.bl(0xa83f8)?;
    // 0x2bb4c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2bb50: beq 0x2bb90 <entry+0x1790>  @ imm = #0x38
    c.b(C::Eq,0x2bb90)?;
    // 0x2bb54: ldr r1, [pc, #0xfec]        @ 0x2748 <entry+0x2748>
    c.literal(R::R1,0x2cb48)?;
    // 0x2bb58: ldr r2, [pc, #0xc9c]        @ 0x23fc <entry+0x23fc>
    c.literal(R::R2,0x2c7fc)?;
    // 0x2bb5c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2bb60: mov r3, #684
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,684)?)?;
    // 0x2bb64: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3128
    c.bl(0x2ec94)?;
    // 0x2bb68: ldr r1, [pc, #0xa00]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2bb6c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2bb70: bl 0x2ec94 <entry+0x4894>  @ imm = #0x311c
    c.bl(0x2ec94)?;
    // 0x2bb74: ldr r2, [pc, #0xce0]        @ 0x245c <entry+0x245c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2bb78: mov r1, #174
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,174)?)?;
    // 0x2bb7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2bb80: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2bb84: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x2a3dc
    c.bl(0x17b0)?;
    // 0x2bb88: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x2bb8c: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x2bb90: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x2bb94: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x2bb98: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x2bb9c: ldr r1, [pc, #0xfa8]        @ 0x274c <entry+0x274c>
    c.literal(R::R1,0x2cb4c)?;
    // 0x2bba0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2bba4: bl 0x2ec94 <entry+0x4894>  @ imm = #0x30e8
    c.bl(0x2ec94)?;
    // 0x2bba8: ldr r1, [pc, #0x9c0]        @ 0x2170 <entry+0x2170>
    c.literal(R::R1,0x2c570)?;
    // 0x2bbac: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2bbb0: bl 0x2ec94 <entry+0x4894>  @ imm = #0x30dc
    c.bl(0x2ec94)?;
    // 0x2bbb4: ldrb r0, [r9, #0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x2bbb8: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x2bbbc: beq 0x2bd0c <entry+0x190c>  @ imm = #0x148
    c.b(C::Eq,0x2bd0c)?;
    // 0x2bbc0: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,16)?)?;
    // 0x2bbc4: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x2bbc8: blt 0x2bc2c <entry+0x182c>  @ imm = #0x5c
    c.b(C::Lt,0x2bc2c)?;
    // 0x2bbcc: cmpne r0, #32768
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,32768)?)?;
    // 0x2bbd0: beq 0x2bc38 <entry+0x1838>  @ imm = #0x60
    c.b(C::Eq,0x2bc38)?;
    // 0x2bbd4: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x2bbd8: beq 0x2bc38 <entry+0x1838>  @ imm = #0x58
    c.b(C::Eq,0x2bc38)?;
    // 0x2bbdc: add r0, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x2bbe0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2bbe4: ldr r2, [pc, #0xc10]        @ 0x23fc <entry+0x23fc>
    c.literal(R::R2,0x2c7fc)?;
    // 0x2bbe8: mov r3, #222
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,222)?)?;
    // 0x2bbec: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2bbf0: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x2bbf4: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x2bbf8: ldr r1, [pc, #0xfc8]        @ 0x27c8 <entry+0x27c8>
    c.literal(R::R1,0x2cbc8)?;
    // 0x2bbfc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    c.check(0x2bc00)?;
    // 0x2bc00: bl 0x2ec94 <entry+0x3094>  @ imm = #0x308c
    c.bl(0x2ec94)?;
    // 0x2bc04: ldr r1, [pc, #0x964]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2bc08: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2bc0c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x3080
    c.bl(0x2ec94)?;
    // 0x2bc10: ldr r2, [pc, #0xc44]        @ 0xc5c <entry+0xc5c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2bc14: mov r1, #223
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,223)?)?;
    // 0x2bc18: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2bc1c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2bc20: bl 0x17b0 <entry+0xfffffffffffd5bb0> @ imm = #-0x2a478
    c.bl(0x17b0)?;
    // 0x2bc24: mov r5, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x2bc28: b 0x2bd0c <entry+0x10c>   @ imm = #0xdc
    c.b(C::Al,0x2bd0c)?;
    // 0x2bc2c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2bc30: cmpne r0, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x2bc34: bne 0x2bbdc <entry+0xffffffffffffffdc> @ imm = #-0x60
    c.b(C::Ne,0x2bbdc)?;
    // 0x2bc38: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x2bc3c: add r7, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(4)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x2bc40: bl 0x2efc <entry+0xfffffffffffd72fc> @ imm = #-0x28d4c
    c.bl(0x2efc)?;
    // 0x2bc44: str r0, [r7, #0x30]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,48)?)?;
    // 0x2bc48: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x2bc4c: ldr r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // 0x2bc50: ldr r3, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,16)?)?;
    // 0x2bc54: bl 0x5a64c <entry+0x2ea4c> @ imm = #0x2e9f0
    c.bl(0x5a64c)?;
    // 0x2bc58: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2bc5c: beq 0x2bc98 <entry+0x98>    @ imm = #0x34
    c.b(C::Eq,0x2bc98)?;
    // 0x2bc60: ldr r1, [pc, #0xfe0]        @ 0x1048 <entry+0x1048>
    c.literal(R::R1,0x2cc48)?;
    // 0x2bc64: ldr r2, [pc, #0xb90]        @ 0xbfc <entry+0xbfc>
    c.literal(R::R2,0x2c7fc)?;
    // 0x2bc68: mov r3, #209
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,209)?)?;
    // 0x2bc6c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2bc70: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x2bc74: bl 0x2ec94 <entry+0x3094>  @ imm = #0x3018
    c.bl(0x2ec94)?;
    // 0x2bc78: ldr r1, [pc, #0x8f0]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2bc7c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2bc80: bl 0x2ec94 <entry+0x3094>  @ imm = #0x300c
    c.bl(0x2ec94)?;
    // 0x2bc84: ldr r2, [pc, #0xbd0]        @ 0xc5c <entry+0xc5c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2bc88: mov r1, #210
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,210)?)?;
    // 0x2bc8c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2bc90: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2bc94: bl 0x17b0 <entry+0xfffffffffffd5bb0> @ imm = #-0x2a4ec
    c.bl(0x17b0)?;
    // 0x2bc98: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,16)?)?;
    // 0x2bc9c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x2bca0: bge 0x2bcbc <entry+0xbc>    @ imm = #0x14
    c.b(C::Ge,0x2bcbc)?;
    // 0x2bca4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2bca8: beq 0x2bcd0 <entry+0xd0>    @ imm = #0x20
    c.b(C::Eq,0x2bcd0)?;
    // 0x2bcac: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x2bcb0: bne 0x2bce0 <entry+0xe0>    @ imm = #0x28
    c.b(C::Ne,0x2bce0)?;
    // 0x2bcb4: mov r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // 0x2bcb8: b 0x2bcec <entry+0xec>    @ imm = #0x2c
    c.b(C::Al,0x2bcec)?;
    // 0x2bcbc: beq 0x2bcd8 <entry+0xd8>    @ imm = #0x14
    c.b(C::Eq,0x2bcd8)?;
    // 0x2bcc0: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x2bcc4: bne 0x2bce0 <entry+0xe0>    @ imm = #0x14
    c.b(C::Ne,0x2bce0)?;
    // 0x2bcc8: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x2bccc: b 0x2bcec <entry+0xec>    @ imm = #0x18
    c.b(C::Al,0x2bcec)?;
    // 0x2bcd0: mov r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
    // 0x2bcd4: b 0x2bcec <entry+0xec>    @ imm = #0x10
    c.b(C::Al,0x2bcec)?;
    // 0x2bcd8: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x2bcdc: b 0x2bcec <entry+0xec>    @ imm = #0x8
    c.b(C::Al,0x2bcec)?;
    // 0x2bce0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2bce4: cmp r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32768)?)?;
    // 0x2bce8: moveq r1, #12
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x2bcec: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x2bcf0: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x2bcf4: ldr r1, [pc, #0xf90]        @ 0x108c <entry+0x108c>
    c.literal(R::R1,0x2cc8c)?;
    // 0x2bcf8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2bcfc: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2f90
    c.bl(0x2ec94)?;
    // 0x2bd00: ldr r1, [pc, #0x868]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2bd04: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2bd08: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2f84
    c.bl(0x2ec94)?;
    // 0x2bd0c: ldrb r0, [r9, #0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x2bd10: tst r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // 0x2bd14: beq 0x2bd50 <entry+0x150>   @ imm = #0x34
    c.b(C::Eq,0x2bd50)?;
    // 0x2bd18: ldr r1, [pc, #0xf70]        @ 0x1090 <entry+0x1090>
    c.literal(R::R1,0x2cc90)?;
    // 0x2bd1c: ldr r2, [pc, #0xad8]        @ 0xbfc <entry+0xbfc>
    c.literal(R::R2,0x2c7fc)?;
    // 0x2bd20: mov r3, #233
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,233)?)?;
    // 0x2bd24: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2bd28: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x2bd2c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2f60
    c.bl(0x2ec94)?;
    // 0x2bd30: ldr r1, [pc, #0x838]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2bd34: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2bd38: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2f54
    c.bl(0x2ec94)?;
    // 0x2bd3c: ldr r2, [pc, #0xb18]        @ 0xc5c <entry+0xc5c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2bd40: mov r1, #234
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,234)?)?;
    // 0x2bd44: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2bd48: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2bd4c: bl 0x17b0 <entry+0xfffffffffffd5bb0> @ imm = #-0x2a5a4
    c.bl(0x17b0)?;
    // 0x2bd50: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x2bd54: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x2bd58: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2bd5c: str r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,28)?)?;
    // 0x2bd60: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2bd64: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2bd68: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x2bd6c: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // 0x2bd70: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2bd74: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2bd78: bl 0x2a878 <entry+0xffffffffffffec78> @ imm = #-0x1508
    c.bl(0x2a878)?;
    // 0x2bd7c: bl 0xa908 <entry+0xfffffffffffded08> @ imm = #-0x2147c
    c.bl(0xa908)?;
    // 0x2bd80: ldr r1, [pc, #0xff8]        @ 0x1180 <entry+0x1180>
    c.literal(R::R1,0x2cd80)?;
    // 0x2bd84: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x2bd88: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2bd8c: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2bd90: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2efc
    c.bl(0x2ec94)?;
    // 0x2bd94: ldr r1, [pc, #0x7d4]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2bd98: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2bd9c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2ef0
    c.bl(0x2ec94)?;
    // 0x2bda0: b 0x2b4ec <entry+0xfffffffffffff8ec> @ imm = #-0x8bc
    c.b(C::Al,0x2b4ec)?;
    // 0x2bda4: add r0, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x2bda8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2bdac: ldr r2, [pc, #0xa48]        @ 0xbfc <entry+0xbfc>
    c.literal(R::R2,0x2c7fc)?;
    // 0x2bdb0: mov r3, #185
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,185)?)?;
    // 0x2bdb4: ldr r0, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,48)?)?;
    // 0x2bdb8: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x2bdbc: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x2bdc0: ldr r1, [pc, #0xe00]        @ 0xfc8 <entry+0xfc8>
    c.literal(R::R1,0x2cbc8)?;
    // 0x2bdc4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2bdc8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2ec4
    c.bl(0x2ec94)?;
    // 0x2bdcc: ldr r1, [pc, #0x79c]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2bdd0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2bdd4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2eb8
    c.bl(0x2ec94)?;
    // 0x2bdd8: mov r5, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x2bddc: b 0x2bbb4 <entry+0xffffffffffffffb4> @ imm = #-0x230
    c.b(C::Al,0x2bbb4)?;
    // 0x2bde0: cmp r7, #2162688
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2162688)?)?;
    // 0x2bde4: beq 0x2c07c <entry+0x47c>   @ imm = #0x290
    c.b(C::Eq,0x2c07c)?;
    // 0x2bde8: cmp r7, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x2bdec: bne 0x2c0f4 <entry+0x4f4>   @ imm = #0x300
    c.b(C::Ne,0x2c0f4)?;
    // 0x2bdf0: ldr r1, [pc, #0xfe8]        @ 0x11e0 <entry+0x11e0>
    c.literal(R::R1,0x2cde0)?;
    // 0x2bdf4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2bdf8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2e94
    c.bl(0x2ec94)?;
    // 0x2bdfc: ldr r1, [pc, #0x76c]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2be00: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2be04: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2e88
    c.bl(0x2ec94)?;
    // 0x2be08: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x2be0c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2be10: orr r1, r1, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,135168)?)?;
    // 0x2be14: cmp r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,12)?)?;
    // 0x2be18: bne 0x2c950 <entry+0xd50>   @ imm = #0xb30
    c.b(C::Ne,0x2c950)?;
    // 0x2be1c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2be20: sub r1, r11, #48
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,48)?)?;
    // 0x2be24: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2be28: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2be2c: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,8)?)?;
    // 0x2be30: bl 0x2d2b4 <entry+0x16b4>  @ imm = #0x147c
    c.bl(0x2d2b4)?;
    // 0x2be34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2be38: beq 0x2cc4c <entry+0x104c>  @ imm = #0xe0c
    c.b(C::Eq,0x2cc4c)?;
    // 0x2be3c: ldr r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-48)?)?;
    // 0x2be40: add r8, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    c.check(0x2be44)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;process_codec_buffer_events(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_buffer_event_instructions_are_contiguous(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let w=|v:usize|u32::from_le_bytes(c.bytes[v-START..v-START+4].try_into().unwrap());
        assert_eq!(w(0x2ba68),arm32::transfer_word(C::Al,true,R::R1,R::PC,0xff4).unwrap());
        assert_eq!(w(0x2be40),arm32::data_register(C::Al,Op::Add,R::new(8).unwrap(),R::new(4).unwrap(),R::R0,Shift::Lsl,2,false).unwrap());
    }
}
