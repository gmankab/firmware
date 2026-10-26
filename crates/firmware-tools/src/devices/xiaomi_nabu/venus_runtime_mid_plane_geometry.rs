// SPDX-License-Identifier: GPL-3.0-only
//! Rebuild controller buffer geometry and stream-plane descriptor sizes.
//! A single A32 function selects packed bit-depth and format dimensions,
//! normalizes signed plane coordinates and computes allocation sizes.
//! A trailing address/bitmask pool is not executable source code.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x38f70;
const END:usize=0x39490;
const GEOMETRY_DIAGNOSTIC_AND_MASKS:[u32;3]=[
    0xfefe_fefe,0x000e_dfcc,0x1717_1717,
];

/// Select codec plane geometry and optional pack-format mapping. The main
/// function returns at 0x39368, with reachable alternate branches following;
/// a short bit-mask helper returns at 0x39480 before the literal words.
fn initialize_plane_geometry(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x38f70: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x38f74: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x38f78: sub sp, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // 0x38f7c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x38f80: ldr r6, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,32)?)?;
    // 0x38f84: ldr r9, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,36)?)?;
    // 0x38f88: ldr r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,80)?)?;
    // 0x38f8c: ldr r10, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(4)?,76)?)?;
    // 0x38f90: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x38f94: mov r7, r10
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(10)?))?;
    // 0x38f98: ldr r5, [r4, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,2104)?)?;
    // 0x38f9c: ldrb r0, [r5, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,339)?)?;
    // 0x38fa0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38fa4: bne 0x38fb4 <entry+0x3b28>  @ imm = #0x8
    c.b(C::Ne,0x38fb4)?;
    // 0x38fa8: ldr r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,288)?)?;
    // 0x38fac: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x38fb0: moveq r7, r10
    c.emit(arm32::mov_register(C::Eq,R::new(7)?,R::new(10)?))?;
    // 0x38fb4: add r0, r4, #472
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,472)?)?;
    // 0x38fb8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x38fbc: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x38fc0: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x38fc4: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x38fc8: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x35de4
    c.bl(0x31ec)?;
    // 0x38fcc: str r8, [r4, #0x484]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1156)?)?;
    // 0x38fd0: str r8, [r4, #0x488]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1160)?)?;
    // 0x38fd4: str r8, [r4, #0x48c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1164)?)?;
    // 0x38fd8: str r8, [r4, #0x490]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1168)?)?;
    // 0x38fdc: str r8, [r4, #0x700]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1792)?)?;
    // 0x38fe0: str r8, [r4, #0x704]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1796)?)?;
    // 0x38fe4: str r8, [r4, #0x708]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1800)?)?;
    // 0x38fe8: str r8, [r4, #0x70c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1804)?)?;
    // 0x38fec: ldrb r0, [r5, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,339)?)?;
    // 0x38ff0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38ff4: beq 0x39020 <entry+0x3b94>  @ imm = #0x24
    c.b(C::Eq,0x39020)?;
    // 0x38ff8: add r1, r10, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,15)?)?;
    // 0x38ffc: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // 0x39000: lsl r2, r1, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsl,7)?)?;
    // 0x39004: str r2, [r4, #0x484]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1156)?)?;
    // 0x39008: str r2, [r4, #0x488]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1160)?)?;
    // 0x3900c: ldrb r2, [r5, #0x154]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,340)?)?;
    // 0x39010: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x39014: lslhs r1, r1, #8
    c.emit(arm32::mov_shifted_register(C::Cs,R::R1,R::R1,Shift::Lsl,8)?)?;
    // 0x39018: strhs r1, [r4, #0x48c]
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,1164)?)?;
    // 0x3901c: strhs r1, [r4, #0x490]
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,1168)?)?;
    // 0x39020: str r8, [r4, #0x448]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1096)?)?;
    // 0x39024: str r8, [r4, #0x44c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1100)?)?;
    // 0x39028: str r8, [r4, #0x450]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1104)?)?;
    // 0x3902c: str r8, [r4, #0x454]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1108)?)?;
    // 0x39030: str r8, [r4, #0x474]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1140)?)?;
    // 0x39034: str r8, [r4, #0x478]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1144)?)?;
    // 0x39038: str r8, [r4, #0x47c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1148)?)?;
    // 0x3903c: str r8, [r4, #0x480]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1152)?)?;
    // 0x39040: str r8, [r4, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1236)?)?;
    // 0x39044: str r8, [r4, #0x4d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1240)?)?;
    // 0x39048: str r8, [r4, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1244)?)?;
    // 0x3904c: str r8, [r4, #0x4e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1248)?)?;
    // 0x39050: ldrb r1, [r4, #0x159]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,345)?)?;
    // 0x39054: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x39058: beq 0x390b8 <entry+0x3c2c>  @ imm = #0x58
    c.b(C::Eq,0x390b8)?;
    // 0x3905c: ldr r0, [r4, #0x7dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2012)?)?;
    // 0x39060: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x39064: lsr r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,2)?)?;
    // 0x39068: add r2, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x3906c: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x39070: bic r2, r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,15)?)?;
    // 0x39074: bic r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // 0x39078: add r2, r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,256)?)?;
    // 0x3907c: add r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // 0x39080: str r2, [r4, #0x47c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1148)?)?;
    // 0x39084: str r0, [r4, #0x474]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1140)?)?;
    // 0x39088: ldr r0, [r4, #0x7e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2016)?)?;
    // 0x3908c: add r1, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x39090: bic r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,15)?)?;
    // 0x39094: add r1, r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,64)?)?;
    // 0x39098: str r1, [r4, #0x478]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1144)?)?;
    // 0x3909c: mvn r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,7)?)?;
    // 0x390a0: and r0, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // 0x390a4: add r0, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,12)?)?;
    // 0x390a8: bic r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // 0x390ac: add r0, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,64)?)?;
    // 0x390b0: str r0, [r4, #0x480]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1152)?)?;
    // 0x390b4: b 0x39360 <entry+0x3ed4>  @ imm = #0x2a4
    c.b(C::Al,0x39360)?;
    // 0x390b8: ldrb r2, [r5, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,325)?)?;
    // 0x390bc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x390c0: beq 0x39138 <entry+0x3cac>  @ imm = #0x70
    c.b(C::Eq,0x39138)?;
    // 0x390c4: lsl lr, r10, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::LR,R::new(10)?,Shift::Lsl,16)?)?;
    // 0x390c8: lsl r0, r6, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(6)?,Shift::Lsl,16)?)?;
    // 0x390cc: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x390d0: and r10, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(10)?,R::R2,4)?)?;
    // 0x390d4: and r6, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::R2,8)?)?;
    // 0x390d8: asr r1, lr, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::LR,Shift::Asr,15)?)?;
    // 0x390dc: cmp r1, r0, asr #16
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Asr,16,false)?)?;
    // 0x390e0: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x390e4: movlt r3, #1
    c.emit(arm32::data_immediate(C::Lt,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x390e8: orr r3, r3, r10, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(10)?,Shift::Lsr,2,false)?)?;
    // 0x390ec: lsl r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,16)?)?;
    // 0x390f0: asr r2, r0, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Asr,15)?)?;
    // 0x390f4: lsl r0, r9, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(9)?,Shift::Lsl,16)?)?;
    // 0x390f8: asr r9, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::R0,Shift::Asr,16)?)?;
    // 0x390fc: cmp r2, r0, asr #16
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Asr,16,false)?)?;
    // 0x39100: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x39104: stm sp, {r1, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit() | R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x39108: blt 0x391e8 <entry+0x3d5c>  @ imm = #0xd8
    c.b(C::Lt,0x391e8)?;
    // 0x3910c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x39110: bne 0x391e8 <entry+0x3d5c>  @ imm = #0xd0
    c.b(C::Ne,0x391e8)?;
    // 0x39114: lsl r0, r7, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(7)?,Shift::Lsl,16)?)?;
    // 0x39118: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x3911c: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x39120: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x39124: asr r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
    // 0x39128: add r0, r2, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,3,false)?)?;
    // 0x3912c: bic r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,31)?)?;
    // 0x39130: add r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // 0x39134: b 0x39228 <entry+0x3d9c>  @ imm = #0xec
    c.b(C::Al,0x39228)?;
    // 0x39138: ldrb r1, [r5, #0x146]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,326)?)?;
    // 0x3913c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x39140: beq 0x39360 <entry+0x3ed4>  @ imm = #0x218
    c.b(C::Eq,0x39360)?;
    // 0x39144: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x39148: bne 0x39158 <entry+0x3ccc>  @ imm = #0x8
    c.b(C::Ne,0x39158)?;
    // 0x3914c: ldr r0, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,288)?)?;
    // 0x39150: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x39154: movne r7, r0
    c.emit(arm32::mov_register(C::Ne,R::new(7)?,R::R0))?;
    // 0x39158: lsl r0, r7, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(7)?,Shift::Lsl,16)?)?;
    // 0x3915c: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x39160: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x39164: cmp r6, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x39168: asr r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
    // 0x3916c: add r0, r2, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,3,false)?)?;
    // 0x39170: bic r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,31)?)?;
    // 0x39174: add r9, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R0,256)?)?;
    // 0x39178: lsl r0, r1, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Lsl,16)?)?;
    // 0x3917c: add r0, r2, r0, asr #13
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Asr,13,false)?)?;
    // 0x39180: str r9, [r4, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1276)?)?;
    // 0x39184: bic r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,31)?)?;
    // 0x39188: add r7, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,64)?)?;
    // 0x3918c: str r7, [r4, #0x500]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1280)?)?;
    // 0x39190: bne 0x391a8 <entry+0x3d1c>  @ imm = #0x10
    c.b(C::Ne,0x391a8)?;
    // 0x39194: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x39198: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3919c: ldrbeq r0, [r5, #0x150]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(5)?,336)?)?;
    // 0x391a0: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x391a4: beq 0x393b0 <entry+0x3f24>  @ imm = #0x204
    c.b(C::Eq,0x393b0)?;
    // 0x391a8: ldr r0, [r4, #0xa9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2716)?)?;
    // 0x391ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x391b0: beq 0x3936c <entry+0x3ee0>  @ imm = #0x1b4
    c.b(C::Eq,0x3936c)?;
    // 0x391b4: str r9, [r4, #0x474]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1140)?)?;
    // 0x391b8: str r7, [r4, #0x478]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1144)?)?;
    // 0x391bc: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x391c0: ldr r1, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,288)?)?;
    // 0x391c4: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4aa80
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x391c8: lsl r6, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::R0,Shift::Lsl,5)?)?;
    // 0x391cc: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x391d0: str r6, [r4, #0x50c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1292)?)?;
    // 0x391d4: ldr r1, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,56)?)?;
    // 0x391d8: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4aa6c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x391dc: lsl r0, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,5)?)?;
    // 0x391e0: str r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1296)?)?;
    // 0x391e4: b 0x39374 <entry+0x3ee8>  @ imm = #0x188
    c.b(C::Al,0x39374)?;
    // 0x391e8: lsl r0, r7, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(7)?,Shift::Lsl,16)?)?;
    // 0x391ec: mov r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x391f0: mov r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x391f4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x391f8: asr r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
    // 0x391fc: add r7, r2, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R2,R::R0,Shift::Lsl,2,false)?)?;
    // 0x39200: bic r7, r7, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(7)?,15)?)?;
    // 0x39204: add r8, r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(7)?,256)?)?;
    // 0x39208: str r8, [r4, #0x47c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1148)?)?;
    // 0x3920c: str r8, [r4, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1244)?)?;
    // 0x39210: beq 0x39224 <entry+0x3d98>  @ imm = #0xc
    c.b(C::Eq,0x39224)?;
    // 0x39214: add r0, r2, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,3,false)?)?;
    // 0x39218: bic r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // 0x3921c: add r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // 0x39220: b 0x39228 <entry+0x3d9c>  @ imm = #0x0
    c.b(C::Al,0x39228)?;
    // 0x39224: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x39228: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x3922c: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x39230: str r0, [r4, #0x474]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1140)?)?;
    // 0x39234: str r0, [r4, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1236)?)?;
    // 0x39238: add r6, r2, r9, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R2,R::new(9)?,Shift::Lsl,3,false)?)?;
    // 0x3923c: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x39240: bic r6, r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,31)?)?;
    // 0x39244: add r7, r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(6)?,64)?)?;
    // 0x39248: str r7, [r4, #0x478]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1144)?)?;
    // 0x3924c: str r7, [r4, #0x4d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1240)?)?;
    // 0x39250: bne 0x3926c <entry+0x3de0>  @ imm = #0x14
    c.b(C::Ne,0x3926c)?;
    // 0x39254: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // 0x39258: add r2, r2, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R1,Shift::Lsl,3,false)?)?;
    // 0x3925c: bic r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,31)?)?;
    // 0x39260: add r9, r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R2,64)?)?;
    // 0x39264: str r9, [r4, #0x480]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1152)?)?;
    // 0x39268: str r9, [r4, #0x4e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1248)?)?;
    // 0x3926c: ldrb r2, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x39270: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x39274: beq 0x3930c <entry+0x3e80>  @ imm = #0x90
    c.b(C::Eq,0x3930c)?;
    // 0x39278: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // 0x3927c: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x39280: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x39284: ldreq r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::SP,4)?)?;
    // 0x39288: cmpeq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3928c: beq 0x392c8 <entry+0x3e3c>  @ imm = #0x34
    c.b(C::Eq,0x392c8)?;
    // 0x39290: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x39294: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x39298: add r2, r1, lr, asr #14
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::LR,Shift::Asr,14,false)?)?;
    // 0x3929c: bic r2, r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,15)?)?;
    // 0x392a0: add r2, r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,256)?)?;
    // 0x392a4: str r2, [r4, #0x4b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1204)?)?;
    // 0x392a8: beq 0x39304 <entry+0x3e78>  @ imm = #0x54
    c.b(C::Eq,0x39304)?;
    // 0x392ac: ldr r2, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,0)?)?;
    // 0x392b0: add r1, r1, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,2,false)?)?;
    // 0x392b4: bic r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,15)?)?;
    // 0x392b8: add r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,256)?)?;
    // 0x392bc: str r1, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1196)?)?;
    // 0x392c0: str r7, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1200)?)?;
    // 0x392c4: b 0x392e8 <entry+0x3e5c>  @ imm = #0x1c
    c.b(C::Al,0x392e8)?;
    // 0x392c8: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x392cc: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x392d0: add r1, r1, lr, asr #13
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::LR,Shift::Asr,13,false)?)?;
    // 0x392d4: bic r1, r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,31)?)?;
    // 0x392d8: add r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,256)?)?;
    // 0x392dc: str r1, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1196)?)?;
    // 0x392e0: str r7, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1200)?)?;
    // 0x392e4: beq 0x3930c <entry+0x3e80>  @ imm = #0x20
    c.b(C::Eq,0x3930c)?;
    // 0x392e8: ldr r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    // 0x392ec: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x392f0: add r1, r1, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,3,false)?)?;
    // 0x392f4: bic r1, r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,31)?)?;
    // 0x392f8: add r1, r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,64)?)?;
    // 0x392fc: str r1, [r4, #0x4b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1208)?)?;
    // 0x39300: b 0x3930c <entry+0x3e80>  @ imm = #0x4
    c.b(C::Al,0x3930c)?;
    // 0x39304: str r2, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1196)?)?;
    // 0x39308: str r7, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1200)?)?;
    // 0x3930c: ldr r5, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,288)?)?;
    // 0x39310: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x39314: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4a930
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x39318: lsl r0, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,5)?)?;
    // 0x3931c: str r0, [r4, #0x448]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1096)?)?;
    // 0x39320: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x39324: ldr r6, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,56)?)?;
    // 0x39328: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x3932c: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4a918
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x39330: lsl r0, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,5)?)?;
    // 0x39334: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x39338: str r0, [r4, #0x44c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1100)?)?;
    // 0x3933c: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x39340: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4a904
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x39344: lsl r0, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,5)?)?;
    // 0x39348: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x3934c: str r0, [r4, #0x450]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1104)?)?;
    // 0x39350: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x39354: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4a8f0
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x39358: lsl r0, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,5)?)?;
    // 0x3935c: str r0, [r4, #0x454]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1108)?)?;
    // 0x39360: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x39364: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x39368: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x3936c: ldr r6, [r4, #0x50c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,1292)?)?;
    // 0x39370: ldr r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1296)?)?;
    // 0x39374: str r6, [r4, #0x448]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1096)?)?;
    // 0x39378: str r0, [r4, #0x44c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1100)?)?;
    // 0x3937c: ldrb r0, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x39380: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x39384: beq 0x39360 <entry+0x3ed4>  @ imm = #-0x2c
    c.b(C::Eq,0x39360)?;
    // 0x39388: lsl r0, r10, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(10)?,Shift::Lsl,16)?)?;
    // 0x3938c: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x39390: add r0, r1, r0, asr #13
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Asr,13,false)?)?;
    // 0x39394: bic r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,31)?)?;
    // 0x39398: add r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // 0x3939c: str r0, [r4, #0x504]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1284)?)?;
    // 0x393a0: str r7, [r4, #0x508]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1288)?)?;
    // 0x393a4: str r0, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1196)?)?;
    // 0x393a8: str r7, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1200)?)?;
    // 0x393ac: b 0x39360 <entry+0x3ed4>  @ imm = #-0x54
    c.b(C::Al,0x39360)?;
    // 0x393b0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x393b4: cmpne r6, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x393b8: beq 0x391a8 <entry+0x3d1c>  @ imm = #-0x218
    c.b(C::Eq,0x391a8)?;
    // 0x393bc: ldr r8, [r5, #0x148]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(5)?,328)?)?;
    // 0x393c0: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x393c4: ldrne r2, [r5, #0x14c]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(5)?,332)?)?;
    // 0x393c8: cmpne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x393cc: beq 0x391a8 <entry+0x3d1c>  @ imm = #-0x22c
    c.b(C::Eq,0x391a8)?;
    // 0x393d0: mov r3, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,100)?)?;
    // 0x393d4: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // 0x393d8: mov r7, r6
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // 0x393dc: mul r0, r2, r3
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::new(3)?)?)?;
    // 0x393e0: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4a864
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x393e4: mov r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,100)?)?;
    // 0x393e8: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x393ec: mul r0, r8, r1
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(8)?,R::R1)?)?;
    // 0x393f0: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x393f4: ldr r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,8)?)?;
    // 0x393f8: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4a84c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x393fc: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x39400: movlo r0, r6
    c.emit(arm32::mov_register(C::Cc,R::R0,R::new(6)?))?;
    // 0x39404: cmp r0, #80
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,80)?)?;
    // 0x39408: bhs 0x391a8 <entry+0x3d1c>  @ imm = #-0x268
    c.b(C::Cs,0x391a8)?;
    // 0x3940c: b 0x391b4 <entry+0x3d28>  @ imm = #-0x260
    c.b(C::Al,0x391b4)?;
    // 0x39410: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x39414: cmp r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // 0x39418: str r3, [r0, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,140)?)?;
    // 0x3941c: str r3, [r0, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,144)?)?;
    // 0x39420: blo 0x39480 <entry+0x3ff4>  @ imm = #0x58
    c.b(C::Cc,0x39480)?;
    // 0x39424: sub r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R2,1)?)?;
    // 0x39428: mov r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x3942c: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // 0x39430: add r2, r2, r12, lsl r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R2,R::R2,R::R12,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x39434: and r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x39438: ldr r2, [pc, #0x44]         @ 0x3ff8 <entry+0x3ff8>
    c.literal_cond(C::Al,R::R2,0x39484)?;
    // 0x3943c: tst r12, r2, lsr r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R12,R::R2,Shift::Lsr,R::R1,false)?)?;
    // 0x39440: ldreq r2, [pc, #0x40]         @ 0x3ffc <entry+0x3ffc>
    c.literal_cond(C::Eq,R::R2,0x39488)?;
    // 0x39444: streq r12, [r0, #0x8c]
    c.emit(arm32::transfer_word(C::Eq,false,R::R12,R::R0,140)?)?;
    // 0x39448: ldrbeq r2, [r2, r1, lsl #1]
    c.emit(arm32::transfer_byte_register(C::Eq,true,R::R2,R::R2,R::R1,Shift::Lsl,1)?)?;
    // 0x3944c: streq r2, [r0, #0x94]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,148)?)?;
    // 0x39450: ldr r2, [pc, #0x34]         @ 0x4000 <entry+0x4000>
    c.literal_cond(C::Al,R::R2,0x3948c)?;
    // 0x39454: tst r12, r2, lsr r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R12,R::R2,Shift::Lsr,R::R1,false)?)?;
    // 0x39458: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x3945c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x39460: str r2, [r0, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,144)?)?;
    // 0x39464: ldr r2, [pc, #0x1c]         @ 0x3ffc <entry+0x3ffc>
    c.literal_cond(C::Al,R::R2,0x39488)?;
    // 0x39468: add r1, r2, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,1,false)?)?;
    // 0x3946c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x39470: ldrb r1, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,1)?)?;
    // 0x39474: str r1, [r0, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,156)?)?;
    // 0x39478: str r2, [r0, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,160)?)?;
    // 0x3947c: str r2, [r0, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,164)?)?;
    // 0x39480: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x39484)?;
    for value in GEOMETRY_DIAGNOSTIC_AND_MASKS {c.emit(value)?;}
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{initialize_plane_geometry(c)}

#[cfg(test)] mod tests{
    use super::*;
    #[test]fn codec_plane_geometry_pool_and_instrs_are_contiguous(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        assert_eq!(&c.bytes[(0x39484-START)..],
           &GEOMETRY_DIAGNOSTIC_AND_MASKS.iter().flat_map(|w|w.to_le_bytes()).collect::<Vec<_>>());
    }
}
