// SPDX-License-Identifier: GPL-3.0-only
//! Record completion and validation callbacks for Venus codec ring queues.
//! Hardware register operations use typed A32 operands; table entries and
//! diagnostic strings are explicitly distinguished from executable code.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x39bd0;
const END:usize=0x39f68;
const DEFAULT_RECORD_TAG:u32=0x0022_1007;
const UNSUPPORTED_CODEC_LOG:u32=0x000e_c0b3;

/// Flush each codec record, optionally reporting retained payload metadata,
/// and reset the counters after dispatching all queued entries.
fn flush_codec_records(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x39bd0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x39bd4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x39bd8: sub sp, sp, #84
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,84)?)?;
    // 0x39bdc: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x39be0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x39be4: ldr r0, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // 0x39be8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x39bec: beq 0x39cac <entry+0xdc>    @ imm = #0xb8
    c.b(C::Eq,0x39cac)?;
    // 0x39bf0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x39bf4: add r9, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::SP,4)?)?;
    // 0x39bf8: mov r10, #80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,80)?)?;
    // 0x39bfc: mvn r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(6)?,R::R0,0)?)?;
    // 0x39c00: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x39c04: ldr r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // 0x39c08: bl 0x2bf4 <entry+0xfffffffffffc9024> @ imm = #-0x3701c
    c.bl(0x2bf4)?;
    // 0x39c0c: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // 0x39c10: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x39c14: ldr r0, [r0, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x39c18: ldr r1, [r0, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,168)?)?;
    // 0x39c1c: cmn r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // 0x39c20: beq 0x39c68 <entry+0x98>    @ imm = #0x40
    c.b(C::Eq,0x39c68)?;
    // 0x39c24: str r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // 0x39c28: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x39c2c: ldr r1, [pc, #0x94]         @ 0xf8 <entry+0xf8>
    c.literal_cond(C::Al,R::R1,0x39cc8)?;
    // 0x39c30: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x39c34: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x39c38: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x39c3c: ldr r1, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,52)?)?;
    // 0x39c40: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x39c44: str r10, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,4)?)?;
    // 0x39c48: ldr r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,40)?)?;
    // 0x39c4c: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x39c50: ldr r1, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,44)?)?;
    // 0x39c54: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x39c58: ldr r1, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,48)?)?;
    // 0x39c5c: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x39c60: str r7, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,80)?)?;
    // 0x39c64: str r6, [r0, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,168)?)?;
    // 0x39c68: ldr r0, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,100)?)?;
    // 0x39c6c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x39c70: strne r7, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Ne,false,R::new(7)?,R::new(4)?,100)?)?;
    // 0x39c74: ldr r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // 0x39c78: bl 0x2c34 <entry+0xfffffffffffc9064> @ imm = #-0x3704c
    c.bl(0x2c34)?;
    // 0x39c7c: cmp r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1)?)?;
    // 0x39c80: bne 0x39cac <entry+0xdc>    @ imm = #0x24
    c.b(C::Ne,0x39cac)?;
    // 0x39c84: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x39c88: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x39c8c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x39c90: bne 0x39ca0 <entry+0xd0>    @ imm = #0x8
    c.b(C::Ne,0x39ca0)?;
    // 0x39c94: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x39c98: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x39c9c: bl 0x2a878 <entry+0xffffffffffff0ca8> @ imm = #-0xf42c
    c.bl(0x2a878)?;
    // 0x39ca0: ldr r0, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // 0x39ca4: cmp r5, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x39ca8: blo 0x39c04 <entry+0x34>    @ imm = #-0xac
    c.b(C::Cc,0x39c04)?;
    // 0x39cac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x39cb0: str r0, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,108)?)?;
    // 0x39cb4: str r0, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,112)?)?;
    // 0x39cb8: str r0, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,116)?)?;
    // 0x39cbc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x39cc0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x39cc4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x39cc8)?;
    c.emit(DEFAULT_RECORD_TAG)?;
    c.check(0x39ccc)
}

/// Normalize inbound and outbound codec data type identifiers. These two
/// eight-way dispatch tables contain runtime branch destinations, NOT A32.
fn translate_codec_record_types(c:&mut Code)->Result<(),String>{
    c.check(0x39ccc)?;
    // 0x39ccc: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x39cd0: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x39cd4: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x39cd8: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x39cdc: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x39ce0: cmp r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x39ce4: bgt 0x39d2c <entry+0x15c>   @ imm = #0x40
    c.b(C::Gt,0x39d2c)?;
    // 0x39ce8: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x39cec: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x39cf0: bhi 0x39d8c <entry+0x1bc>   @ imm = #0x94
    c.b(C::Hi,0x39d8c)?;
    // 0x39cf4: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x39cf8: add r2, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,0)?)?;
    // 0x39cfc: ldr pc, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R2,Shift::Lsl,0)?)?;
    c.check(0x39d00)?;
    const INPUT_TYPE_TARGETS:[u32;8]=[
      0x39d20,0x39d50,0x39d8c,0x39d5c,
      0x39d8c,0x39d8c,0x39d8c,0x39d68,
    ];
    for destination in INPUT_TYPE_TARGETS {c.emit(destination)?;}
    c.check(0x39d20)?;
    // 0x39d20: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x39d24: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x39d28: b 0x39da0 <entry+0x1d0>   @ imm = #0x70
    c.b(C::Al,0x39da0)?;
    // 0x39d2c: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x39d30: beq 0x39d74 <entry+0x1a4>   @ imm = #0x3c
    c.b(C::Eq,0x39d74)?;
    // 0x39d34: cmp r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32)?)?;
    // 0x39d38: beq 0x39d80 <entry+0x1b0>   @ imm = #0x40
    c.b(C::Eq,0x39d80)?;
    // 0x39d3c: cmp r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,64)?)?;
    // 0x39d40: bne 0x39d8c <entry+0x1bc>   @ imm = #0x44
    c.b(C::Ne,0x39d8c)?;
    // 0x39d44: mov r0, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,100)?)?;
    // 0x39d48: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x39d4c: b 0x39da0 <entry+0x1d0>   @ imm = #0x4c
    c.b(C::Al,0x39da0)?;
    // 0x39d50: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x39d54: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x39d58: b 0x39da0 <entry+0x1d0>   @ imm = #0x40
    c.b(C::Al,0x39da0)?;
    // 0x39d5c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x39d60: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x39d64: b 0x39da0 <entry+0x1d0>   @ imm = #0x34
    c.b(C::Al,0x39da0)?;
    // 0x39d68: mov r0, #118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,118)?)?;
    // 0x39d6c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x39d70: b 0x39da0 <entry+0x1d0>   @ imm = #0x28
    c.b(C::Al,0x39da0)?;
    // 0x39d74: mov r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,128)?)?;
    // 0x39d78: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x39d7c: b 0x39da0 <entry+0x1d0>   @ imm = #0x1c
    c.b(C::Al,0x39da0)?;
    // 0x39d80: mov r0, #66
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,66)?)?;
    // 0x39d84: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x39d88: b 0x39da0 <entry+0x1d0>   @ imm = #0x10
    c.b(C::Al,0x39da0)?;
    // 0x39d8c: ldr r2, [pc, #0x158]        @ 0x31c <entry+0x31c>
    c.literal_cond(C::Al,R::R2,0x39eec)?;
    // 0x39d90: mov r1, #232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,232)?)?;
    // 0x39d94: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x39d98: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x39d9c: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x385f4
    c.bl(0x17b0)?;
    // 0x39da0: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x39da4: cmp r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,255)?)?;
    // 0x39da8: bgt 0x39df8 <entry+0x228>   @ imm = #0x48
    c.b(C::Gt,0x39df8)?;
    // 0x39dac: cmp r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x39db0: bgt 0x39e20 <entry+0x250>   @ imm = #0x68
    c.b(C::Gt,0x39e20)?;
    // 0x39db4: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // 0x39db8: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // 0x39dbc: bhi 0x39ee0 <entry+0x310>   @ imm = #0x11c
    c.b(C::Hi,0x39ee0)?;
    // 0x39dc0: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // 0x39dc4: add r2, pc, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,4)?)?;
    // 0x39dc8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x39dcc: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
    c.check(0x39dd0)?;
    const OUTPUT_TYPE_TARGETS:[u32;8]=[
      0x39df0,0x39ea0,0x39ee0,0x39ee4,
      0x39ee0,0x39ee0,0x39ee0,0x39ea8,
    ];
    for destination in OUTPUT_TYPE_TARGETS {c.emit(destination)?;}
    c.check(0x39df0)?;
    // 0x39df0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x39df4: b 0x39ee4 <entry+0x314>   @ imm = #0xe8
    c.b(C::Al,0x39ee4)?;
    // 0x39df8: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x39dfc: bge 0x39e40 <entry+0x270>   @ imm = #0x3c
    c.b(C::Ge,0x39e40)?;
    // 0x39e00: cmp r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1024)?)?;
    // 0x39e04: bge 0x39e60 <entry+0x290>   @ imm = #0x54
    c.b(C::Ge,0x39e60)?;
    // 0x39e08: cmp r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,256)?)?;
    // 0x39e0c: beq 0x39eb0 <entry+0x2e0>   @ imm = #0x9c
    c.b(C::Eq,0x39eb0)?;
    // 0x39e10: cmp r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,512)?)?;
    // 0x39e14: bne 0x39ee0 <entry+0x310>   @ imm = #0xc4
    c.b(C::Ne,0x39ee0)?;
    // 0x39e18: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x39e1c: b 0x39ee4 <entry+0x314>   @ imm = #0xc0
    c.b(C::Al,0x39ee4)?;
    // 0x39e20: cmp r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,63)?)?;
    // 0x39e24: bgt 0x39e74 <entry+0x2a4>   @ imm = #0x48
    c.b(C::Gt,0x39e74)?;
    // 0x39e28: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x39e2c: beq 0x39eb8 <entry+0x2e8>   @ imm = #0x84
    c.b(C::Eq,0x39eb8)?;
    // 0x39e30: cmp r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32)?)?;
    // 0x39e34: bne 0x39ee0 <entry+0x310>   @ imm = #0xa4
    c.b(C::Ne,0x39ee0)?;
    // 0x39e38: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x39e3c: b 0x39ee4 <entry+0x314>   @ imm = #0xa0
    c.b(C::Al,0x39ee4)?;
    // 0x39e40: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x39e44: bge 0x39e8c <entry+0x2bc>   @ imm = #0x40
    c.b(C::Ge,0x39e8c)?;
    // 0x39e48: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x39e4c: beq 0x39ec0 <entry+0x2f0>   @ imm = #0x6c
    c.b(C::Eq,0x39ec0)?;
    // 0x39e50: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x39e54: bne 0x39ee0 <entry+0x310>   @ imm = #0x84
    c.b(C::Ne,0x39ee0)?;
    // 0x39e58: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x39e5c: b 0x39ee4 <entry+0x314>   @ imm = #0x80
    c.b(C::Al,0x39ee4)?;
    // 0x39e60: beq 0x39ec8 <entry+0x2f8>   @ imm = #0x60
    c.b(C::Eq,0x39ec8)?;
    // 0x39e64: cmp r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2048)?)?;
    // 0x39e68: bne 0x39ee0 <entry+0x310>   @ imm = #0x70
    c.b(C::Ne,0x39ee0)?;
    // 0x39e6c: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x39e70: b 0x39ee4 <entry+0x314>   @ imm = #0x6c
    c.b(C::Al,0x39ee4)?;
    // 0x39e74: cmp r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,64)?)?;
    // 0x39e78: beq 0x39ed0 <entry+0x300>   @ imm = #0x50
    c.b(C::Eq,0x39ed0)?;
    // 0x39e7c: cmp r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,128)?)?;
    // 0x39e80: bne 0x39ee0 <entry+0x310>   @ imm = #0x58
    c.b(C::Ne,0x39ee0)?;
    // 0x39e84: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x39e88: b 0x39ee4 <entry+0x314>   @ imm = #0x54
    c.b(C::Al,0x39ee4)?;
    // 0x39e8c: beq 0x39ed8 <entry+0x308>   @ imm = #0x44
    c.b(C::Eq,0x39ed8)?;
    // 0x39e90: cmp r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32768)?)?;
    // 0x39e94: bne 0x39ee0 <entry+0x310>   @ imm = #0x44
    c.b(C::Ne,0x39ee0)?;
    // 0x39e98: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x39e9c: b 0x39ee4 <entry+0x314>   @ imm = #0x40
    c.b(C::Al,0x39ee4)?;
    // 0x39ea0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x39ea4: b 0x39ee4 <entry+0x314>   @ imm = #0x38
    c.b(C::Al,0x39ee4)?;
    // 0x39ea8: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x39eac: b 0x39ee4 <entry+0x314>   @ imm = #0x30
    c.b(C::Al,0x39ee4)?;
    // 0x39eb0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x39eb4: b 0x39ee4 <entry+0x314>   @ imm = #0x28
    c.b(C::Al,0x39ee4)?;
    // 0x39eb8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x39ebc: b 0x39ee4 <entry+0x314>   @ imm = #0x20
    c.b(C::Al,0x39ee4)?;
    // 0x39ec0: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x39ec4: b 0x39ee4 <entry+0x314>   @ imm = #0x18
    c.b(C::Al,0x39ee4)?;
    // 0x39ec8: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x39ecc: b 0x39ee4 <entry+0x314>   @ imm = #0x10
    c.b(C::Al,0x39ee4)?;
    // 0x39ed0: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x39ed4: b 0x39ee4 <entry+0x314>   @ imm = #0x8
    c.b(C::Al,0x39ee4)?;
    // 0x39ed8: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x39edc: b 0x39ee4 <entry+0x314>   @ imm = #0x0
    c.b(C::Al,0x39ee4)?;
    // 0x39ee0: mov r0, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    // 0x39ee4: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x39ee8: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x39eec)?;
    c.emit(UNSUPPORTED_CODEC_LOG)?;
    c.check(0x39ef0)
}

/// Small codec-ID conversion callback shared with the normalizer above.
fn normalize_codec_subtype(c:&mut Code)->Result<(),String>{
    c.check(0x39ef0)?;
    // 0x39ef0: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // 0x39ef4: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x39ef8: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x39efc: bhi 0x39f3c <entry+0x36c>   @ imm = #0x38
    c.b(C::Hi,0x39f3c)?;
    // 0x39f00: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x39f04: add r2, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,0)?)?;
    // 0x39f08: ldr pc, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R2,Shift::Lsl,0)?)?;
    c.check(0x39f0c)?;
    const SMALL_CODEC_TARGETS:[u32;8]=[
      0x39f2c,0x39f34,0x39f3c,0x39f50,
      0x39f3c,0x39f3c,0x39f3c,0x39f58,
    ];
    for destination in SMALL_CODEC_TARGETS {c.emit(destination)?;}
    c.check(0x39f2c)?;
    // 0x39f2c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x39f30: b 0x39f5c <entry+0x38c>   @ imm = #0x24
    c.b(C::Al,0x39f5c)?;
    // 0x39f34: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x39f38: b 0x39f5c <entry+0x38c>   @ imm = #0x1c
    c.b(C::Al,0x39f5c)?;
    // 0x39f3c: ldr r2, [pc, #0x20]         @ 0x394 <entry+0x394>
    c.literal_cond(C::Al,R::R2,0x39f64)?;
    // 0x39f40: mov r1, #94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,94)?)?;
    // 0x39f44: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x39f48: orr r1, r1, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4352)?)?;
    // 0x39f4c: b 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x387a4
    c.b(C::Al,0x17b0)?;
    // 0x39f50: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x39f54: b 0x39f5c <entry+0x38c>   @ imm = #0x0
    c.b(C::Al,0x39f5c)?;
    // 0x39f58: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x39f5c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x39f60: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x39f64)?;
    c.emit(UNSUPPORTED_CODEC_LOG)?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    flush_codec_records(c)?;
    translate_codec_record_types(c)?;
    normalize_codec_subtype(c)?;
    Ok(())
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn codec_callback_records_contiguous(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
    }
    #[test]fn codec_tables_and_two_diagnostics_are_exact(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        let get=|a|u32::from_le_bytes(c.bytes[a-START..a-START+4].try_into().unwrap());
        assert_eq!(get(0x39cc8),DEFAULT_RECORD_TAG);
        assert_eq!(get(0x39eec),UNSUPPORTED_CODEC_LOG);
        assert_eq!(get(0x39f64),UNSUPPORTED_CODEC_LOG);
        assert_eq!(get(0x39d00),0x39d20);
        assert_eq!(get(0x39dd0),0x39df0);
        assert_eq!(get(0x39f0c),0x39f2c);
    }
}
