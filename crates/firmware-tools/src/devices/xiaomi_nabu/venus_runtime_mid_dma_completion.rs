// SPDX-License-Identifier: GPL-3.0-only
//! Controller DMA completion, error-mask readback, and capture-record setup.
//! Every executable instruction uses typed A32 operands and all status,
//! pointer and log words are modeled as non-executable data literals.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x29850;
const END:usize=0x29ae4;

/// Acknowledge completed DMA; notify controller once channel counter reaches the threshold.
fn notify_dma_completion(c:&mut Code)->Result<(),String>{
    c.check(0x29850)?;
    // 0x29850: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x29854: mov r1, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,56)?)?;
    // 0x29858: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x2985c: orr r3, r1, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R1,R::R2,Shift::Lsl,15,false)?)?;
    // 0x29860: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x29864: ldr r3, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x29868: tst r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,1)?)?;
    // 0x2986c: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x29870: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // 0x29874: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x29878: orr r2, r2, #60
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,60)?)?;
    // 0x2987c: orr r2, r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    // 0x29880: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x29884: ldr r2, [pc, #0x4c]         @ 0x63c <entry+0x63c>
    c.literal(R::R2,0x298d8)?;
    // 0x29888: ldr r1, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // 0x2988c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x29890: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // 0x29894: ldrb r2, [r0, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,8)?)?;
    // 0x29898: tst r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,1)?)?;
    // 0x2989c: bne 0x298a8 <entry+0x60c>   @ imm = #0x4
    c.b(C::Ne,0x298a8)?;
    // 0x298a0: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x298a4: bne 0x298d4 <entry+0x638>   @ imm = #0x28
    c.b(C::Ne,0x298d4)?;
    // 0x298a8: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x298ac: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x298b0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x298b4: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // 0x298b8: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x298bc: str r3, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    // 0x298c0: ldr r1, [pc, #0x14]         @ 0x640 <entry+0x640>
    c.literal(R::R1,0x298dc)?;
    // 0x298c4: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x298c8: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x298cc: add r0, r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,68)?)?;
    // 0x298d0: b 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x26fac
    c.b(C::Al,0x292c)?;
    // 0x298d4: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x298d8)?;
    const DATA:[u32;2]=[0x001040cc,0x0010ceac];
    for value in DATA {c.emit(value)?;}
    c.check(0x298e0)
}

/// Process stream/encoder error masks with writeback and callback routing.
fn acknowledge_encoder_error(c:&mut Code)->Result<(),String>{
    c.check(0x298e0)?;
    // 0x298e0: ldr r3, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // 0x298e4: mov r1, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,120)?)?;
    // 0x298e8: mov r12, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // 0x298ec: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x298f0: orr r2, r1, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x298f4: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x298f8: ldr r2, [r2, r12]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R12,Shift::Lsl,0)?)?;
    // 0x298fc: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x29900: ands r2, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R1,Shift::Lsl,0,true)?)?;
    // 0x29904: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x29908: lsl r1, r3, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(3)?,Shift::Lsl,15)?)?;
    // 0x2990c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x29910: orr r1, r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,20)?)?;
    // 0x29914: str r3, [r1, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R12,Shift::Lsl,0)?)?;
    // 0x29918: mov r3, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,124)?)?;
    // 0x2991c: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x29920: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x29924: orr r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // 0x29928: str r2, [r1, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R12,Shift::Lsl,0)?)?;
    // 0x2992c: ldr r1, [pc, #0xc]          @ 0x6a4 <entry+0x6a4>
    c.literal(R::R1,0x29940)?;
    // 0x29930: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x29934: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x29938: add r0, r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,36)?)?;
    // 0x2993c: b 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x27018
    c.b(C::Al,0x292c)?;
    c.check(0x29940)?;
    const DATA:[u32;1]=[0x0010ceac];
    for value in DATA {c.emit(value)?;}
    c.check(0x29944)
}

/// Initialize capture metadata, validate attached objects and allocate event buffer storage.
fn initialize_host_capture_record(c:&mut Code)->Result<(),String>{
    c.check(0x29944)?;
    // 0x29944: push {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x29948: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2994c: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x29950: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x29954: ldr r0, [pc, #0x170]        @ 0x830 <entry+0x830>
    c.literal(R::R0,0x29acc)?;
    // 0x29958: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x2995c: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x29960: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x29964: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x29968: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // 0x2996c: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // 0x29970: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x29974: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29978: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2997c: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x29980: add r0, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,4)?)?;
    // 0x29984: bl 0x29b4 <entry+0xfffffffffffd9718> @ imm = #-0x26fd8
    c.bl(0x29b4)?;
    // 0x29988: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2998c: beq 0x299c0 <entry+0x724>   @ imm = #0x2c
    c.b(C::Eq,0x299c0)?;
    // 0x29990: ldr r1, [pc, #0x138]        @ 0x834 <entry+0x834>
    c.literal(R::R1,0x29ad0)?;
    // 0x29994: ldr r2, [pc, #0x138]        @ 0x838 <entry+0x838>
    c.literal(R::R2,0x29ad4)?;
    // 0x29998: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2999c: mov r3, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,116)?)?;
    // 0x299a0: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x52ec
    c.bl(0x2ec94)?;
    // 0x299a4: ldr r1, [pc, #0x12c]        @ 0x83c <entry+0x83c>
    c.literal(R::R1,0x29ad8)?;
    // 0x299a8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x299ac: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x52e0
    c.bl(0x2ec94)?;
    // 0x299b0: ldr r2, [pc, #0x124]        @ 0x840 <entry+0x840>
    c.literal(R::R2,0x29adc)?;
    // 0x299b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x299b8: mov r1, #117
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,117)?)?;
    // 0x299bc: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28214
    c.bl(0x17b0)?;
    // 0x299c0: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x299c4: add r0, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    // 0x299c8: bl 0x2c44 <entry+0xfffffffffffd99a8> @ imm = #-0x26d8c
    c.bl(0x2c44)?;
    // 0x299cc: ldr r5, [pc, #0x108]        @ 0x840 <entry+0x840>
    c.literal(R::new(5)?,0x29adc)?;
    // 0x299d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x299d4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x299d8: mov r1, #121
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,121)?)?;
    // 0x299dc: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x299e0: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x299e4: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x2823c
    c.bl(0x17b0)?;
    // 0x299e8: add r0, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,8)?)?;
    // 0x299ec: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x299f0: bl 0x2c44 <entry+0xfffffffffffd99a8> @ imm = #-0x26db4
    c.bl(0x2c44)?;
    // 0x299f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x299f8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x299fc: mov r1, #125
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,125)?)?;
    // 0x29a00: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x29a04: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x29a08: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28260
    c.bl(0x17b0)?;
    // 0x29a0c: add r0, r4, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,40)?)?;
    // 0x29a10: bl 0x2b8c <entry+0xfffffffffffd98f0> @ imm = #-0x26e8c
    c.bl(0x2b8c)?;
    // 0x29a14: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29a18: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29a1c: mov r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // 0x29a20: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x29a24: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x29a28: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28280
    c.bl(0x17b0)?;
    // 0x29a2c: add r0, r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,12)?)?;
    // 0x29a30: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x29a34: bl 0x2c44 <entry+0xfffffffffffd99a8> @ imm = #-0x26df8
    c.bl(0x2c44)?;
    // 0x29a38: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29a3c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29a40: mov r1, #131
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,131)?)?;
    // 0x29a44: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x29a48: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x29a4c: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x282a4
    c.bl(0x17b0)?;
    // 0x29a50: add r0, r4, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,44)?)?;
    // 0x29a54: bl 0x2b8c <entry+0xfffffffffffd98f0> @ imm = #-0x26ed0
    c.bl(0x2b8c)?;
    // 0x29a58: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29a5c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29a60: mov r1, #134
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,134)?)?;
    // 0x29a64: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x29a68: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x29a6c: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x282c4
    c.bl(0x17b0)?;
    // 0x29a70: add r0, r4, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,48)?)?;
    // 0x29a74: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x29a78: mov r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // 0x29a7c: bl 0x31ec <entry+0xfffffffffffd9f50> @ imm = #-0x26898
    c.bl(0x31ec)?;
    // 0x29a80: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // 0x29a84: bl 0x2efc <entry+0xfffffffffffd9c60> @ imm = #-0x26b90
    c.bl(0x2efc)?;
    // 0x29a88: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29a8c: str r0, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,120)?)?;
    // 0x29a90: bne 0x29ac4 <entry+0x828>   @ imm = #0x2c
    c.b(C::Ne,0x29ac4)?;
    // 0x29a94: ldr r1, [pc, #0x44]         @ 0x844 <entry+0x844>
    c.literal(R::R1,0x29ae0)?;
    // 0x29a98: ldr r2, [pc, #0x34]         @ 0x838 <entry+0x838>
    c.literal(R::R2,0x29ad4)?;
    // 0x29a9c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x29aa0: mov r3, #145
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,145)?)?;
    // 0x29aa4: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x51e8
    c.bl(0x2ec94)?;
    // 0x29aa8: ldr r1, [pc, #0x28]         @ 0x83c <entry+0x83c>
    c.literal(R::R1,0x29ad8)?;
    // 0x29aac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x29ab0: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x51dc
    c.bl(0x2ec94)?;
    // 0x29ab4: ldr r2, [pc, #0x20]         @ 0x840 <entry+0x840>
    c.literal(R::R2,0x29adc)?;
    // 0x29ab8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29abc: mov r1, #146
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,146)?)?;
    // 0x29ac0: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28318
    c.bl(0x17b0)?;
    // 0x29ac4: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x29ac8: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x29acc)?;
    const DATA:[u32;6]=[0x00029ae4,0x000e87f8,0x000e8832,0x000e2788,0x000e884d,0x000e8881];
    for value in DATA {c.emit(value)?;}
    c.check(0x29ae4)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    notify_dma_completion(c)?;
    acknowledge_encoder_error(c)?;
    initialize_host_capture_record(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn completion_and_capture_literal_boundaries(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|addr:usize|u32::from_le_bytes(c.bytes[addr-START..addr-START+4].try_into().unwrap());
        assert_eq!(word(0x298d8),0x0010_40cc);
        assert_eq!(word(0x298dc),0x0010_ceac);
        assert_eq!(word(0x29940),0x0010_ceac);
        assert_eq!(word(0x29ad8),0x000e_2788);
    }
}
