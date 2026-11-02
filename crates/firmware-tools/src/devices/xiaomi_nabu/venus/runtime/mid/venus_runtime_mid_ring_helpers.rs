// SPDX-License-Identifier: GPL-3.0-only
//! Two independent controller ring-buffer helper routines. They verify
//! copied offsets/word counts and move packed record data into/out of the
//! shared ring. The two trailing literal pools identify diagnostic strings
//! and memory-map addresses, not executable ARM instructions.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2efc0;
const END:usize=0x2f394;
fn inspect_ring_and_copy_record(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2efc0: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2efc4: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x2efc8: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x2efcc: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x2efd0: mov r9, r2
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R2))?;
    // 0x2efd4: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x2efd8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x2efdc: beq 0x2f038 <entry+0x1a38>  @ imm = #0x54
    c.b(C::Eq,0x2f038)?;
    // 0x2efe0: ldrb r0, [r5, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x2efe4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2efe8: bhi 0x2f054 <entry+0x1a54>  @ imm = #0x64
    c.b(C::Hi,0x2f054)?;
    // 0x2efec: ldr r1, [pc, #0x1d4]        @ 0x1bc8 <entry+0x1bc8>
    c.literal(R::R1,0x2f1c8)?;
    // 0x2eff0: mvn r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(7)?,R::R0,0)?)?;
    // 0x2eff4: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x2eff8: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x2effc: ldr r1, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,4)?)?;
    // 0x2f000: add r2, r0, #148
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,148)?)?;
    // 0x2f004: ldr r0, [r2, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,-4)?)?;
    // 0x2f008: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2f00c: bne 0x2f024 <entry+0x1a24>  @ imm = #0x10
    c.b(C::Ne,0x2f024)?;
    // 0x2f010: ldr r0, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,0)?)?;
    // 0x2f014: sub r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f018: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x2f01c: cmp r0, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x2f020: beq 0x2f048 <entry+0x1a48>  @ imm = #0x20
    c.b(C::Eq,0x2f048)?;
    // 0x2f024: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x2f028: add r2, r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,24)?)?;
    // 0x2f02c: cmp r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // 0x2f030: blt 0x2f004 <entry+0x1a04>  @ imm = #-0x34
    c.b(C::Lt,0x2f004)?;
    // 0x2f034: b 0x2f054 <entry+0x1a54>  @ imm = #0x18
    c.b(C::Al,0x2f054)?;
    // 0x2f038: ldr r2, [pc, #0x1a0]        @ 0x1be0 <entry+0x1be0>
    c.literal(R::R2,0x2f1e0)?;
    // 0x2f03c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f040: mov r1, #160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,160)?)?;
    // 0x2f044: b 0x2f080 <entry+0x1a80>  @ imm = #0x34
    c.b(C::Al,0x2f080)?;
    // 0x2f048: ldr r2, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,48)?)?;
    // 0x2f04c: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2f050: bls 0x2f094 <entry+0x1a94>  @ imm = #0x3c
    c.b(C::Ls,0x2f094)?;
    // 0x2f054: ldr r1, [pc, #0x180]        @ 0x1bdc <entry+0x1bdc>
    c.literal(R::R1,0x2f1dc)?;
    // 0x2f058: ldr r2, [pc, #0x174]        @ 0x1bd4 <entry+0x1bd4>
    c.literal(R::R2,0x2f1d4)?;
    // 0x2f05c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f060: mov r3, #184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,184)?)?;
    // 0x2f064: bl 0x2ec94 <entry+0x1694>  @ imm = #-0x3d8
    c.bl(0x2ec94)?;
    // 0x2f068: ldr r1, [pc, #0x168]        @ 0x1bd8 <entry+0x1bd8>
    c.literal(R::R1,0x2f1d8)?;
    // 0x2f06c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f070: bl 0x2ec94 <entry+0x1694>  @ imm = #-0x3e4
    c.bl(0x2ec94)?;
    // 0x2f074: ldr r2, [pc, #0x164]        @ 0x1be0 <entry+0x1be0>
    c.literal(R::R2,0x2f1e0)?;
    // 0x2f078: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f07c: mov r1, #185
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,185)?)?;
    // 0x2f080: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x2f084: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2d8dc
    c.bl(0x17b0)?;
    // 0x2f088: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2f08c: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // 0x2f090: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2f094: ldr r7, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,52)?)?;
    // 0x2f098: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x2f09c: bne 0x2f0b8 <entry+0x1ab8>  @ imm = #0x14
    c.b(C::Ne,0x2f0b8)?;
    // 0x2f0a0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f0a4: str r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,32)?)?;
    // 0x2f0a8: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x2f0ac: str r5, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(9)?,0)?)?;
    // 0x2f0b0: bl 0x30cc <entry+0xfffffffffffd5acc> @ imm = #-0x2bfec
    c.bl(0x30cc)?;
    // 0x2f0b4: b 0x2f088 <entry+0x1a88>  @ imm = #-0x34
    c.b(C::Al,0x2f088)?;
    // 0x2f0b8: ldr r8, [pc, #0x10c]        @ 0x1bcc <entry+0x1bcc>
    c.literal(R::new(8)?,0x2f1cc)?;
    // 0x2f0bc: add r1, r1, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,2,false)?)?;
    // 0x2f0c0: ldr r4, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,0)?)?;
    // 0x2f0c4: ldr r12, [r1, r4]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R12,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x2f0c8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x2f0cc: cmp r4, r12, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R12,Shift::Lsr,2,false)?)?;
    // 0x2f0d0: beq 0x2f120 <entry+0x1b20>  @ imm = #0x48
    c.b(C::Eq,0x2f120)?;
    // 0x2f0d4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x2f0d8: beq 0x2f110 <entry+0x1b10>  @ imm = #0x30
    c.b(C::Eq,0x2f110)?;
    // 0x2f0dc: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x2f0e0: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x2f0e4: cmp r12, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x2f0e8: bhi 0x2f120 <entry+0x1b20>  @ imm = #0x30
    c.b(C::Hi,0x2f120)?;
    // 0x2f0ec: ands r3, r12, #3
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(3)?,R::R12,3,true)?)?;
    // 0x2f0f0: bne 0x2f120 <entry+0x1b20>  @ imm = #0x28
    c.b(C::Ne,0x2f120)?;
    // 0x2f0f4: lsr r3, r12, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R12,Shift::Lsr,2)?)?;
    // 0x2f0f8: add r7, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x2f0fc: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2f100: bhs 0x2f16c <entry+0x1b6c>  @ imm = #0x64
    c.b(C::Cs,0x2f16c)?;
    // 0x2f104: bic r2, r12, #3
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R12,3)?)?;
    // 0x2f108: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x2f10c: b 0x2f194 <entry+0x1b94>  @ imm = #0x80
    c.b(C::Al,0x2f194)?;
    // 0x2f110: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x2f114: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x2f118: cmp r12, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x2f11c: blo 0x2f0f4 <entry+0x1af4>  @ imm = #-0x30
    c.b(C::Cc,0x2f0f4)?;
    // 0x2f120: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f124: str r7, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,48)?)?;
    // 0x2f128: ldr r2, [pc, #0xa4]         @ 0x1bd4 <entry+0x1bd4>
    c.literal(R::R2,0x2f1d4)?;
    // 0x2f12c: mov r3, #213
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,213)?)?;
    // 0x2f130: str r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,32)?)?;
    // 0x2f134: ldr r0, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,36)?)?;
    // 0x2f138: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2f13c: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f140: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x2f144: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x2f148: ldr r1, [pc, #0x80]         @ 0x1bd0 <entry+0x1bd0>
    c.literal(R::R1,0x2f1d0)?;
    // 0x2f14c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2f150: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f154: bl 0x2ec94 <entry+0x1694>  @ imm = #-0x4c8
    c.bl(0x2ec94)?;
    // 0x2f158: ldr r1, [pc, #0x78]         @ 0x1bd8 <entry+0x1bd8>
    c.literal(R::R1,0x2f1d8)?;
    // 0x2f15c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f160: bl 0x2ec94 <entry+0x1694>  @ imm = #-0x4d4
    c.bl(0x2ec94)?;
    // 0x2f164: mvn r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(5)?,R::R0,0)?)?;
    // 0x2f168: b 0x2f088 <entry+0x1a88>  @ imm = #-0xe8
    c.b(C::Al,0x2f088)?;
    // 0x2f16c: sub r7, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2f170: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x2f174: sub r4, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x2f178: lsl r2, r4, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(4)?,Shift::Lsl,2)?)?;
    // 0x2f17c: bl 0x471c <entry+0xfffffffffffd711c> @ imm = #-0x2aa68
    c.bl(0x471c)?;
    // 0x2f180: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x2f184: ldr r2, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,4)?)?;
    // 0x2f188: add r0, r6, r4, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(4)?,Shift::Lsl,2,false)?)?;
    // 0x2f18c: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f190: lsl r2, r7, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x2f194: bl 0x471c <entry+0xfffffffffffd711c> @ imm = #-0x2aa80
    c.bl(0x471c)?;
    // 0x2f198: str r7, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,48)?)?;
    // 0x2f19c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2f1a0: ldr r0, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,52)?)?;
    // 0x2f1a4: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2f1a8: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2f1ac: str r1, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,32)?)?;
    // 0x2f1b0: ldr r0, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,36)?)?;
    // 0x2f1b4: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x2f1b8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2f1bc: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f1c0: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x2f1c4: b 0x2f088 <entry+0x1a88>  @ imm = #-0x144
    c.b(C::Al,0x2f088)?;
    c.check(0x2f1c8)?;
    const RING_HELPER_DATA_2F1C8:[u32;7]=[0x0010ceac,0x00104114,0x000eadac,0x000ead98,0x000e2788,0x000ead5d,0x000ead29];
    for word in RING_HELPER_DATA_2F1C8 {c.emit(word)?;}
    c.check(0x2f1e4)?;
    // 0x2f1e4: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2f1e8: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x2f1ec: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x2f1f0: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x2f1f4: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x2f1f8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x2f1fc: beq 0x2f258 <entry+0x1c58>  @ imm = #0x54
    c.b(C::Eq,0x2f258)?;
    // 0x2f200: ldrb r0, [r5, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x2f204: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2f208: bhi 0x2f280 <entry+0x1c80>  @ imm = #0x70
    c.b(C::Hi,0x2f280)?;
    // 0x2f20c: ldr r1, [pc, #0x168]        @ 0x1d7c <entry+0x1d7c>
    c.literal(R::R1,0x2f37c)?;
    // 0x2f210: mvn r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,0)?)?;
    // 0x2f214: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x2f218: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x2f21c: add r2, r0, #148
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,148)?)?;
    // 0x2f220: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // 0x2f224: ldr r1, [r2, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,-4)?)?;
    // 0x2f228: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f22c: bne 0x2f244 <entry+0x1c44>  @ imm = #0x10
    c.b(C::Ne,0x2f244)?;
    // 0x2f230: ldr r1, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // 0x2f234: sub r7, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2f238: ldr r1, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,12)?)?;
    // 0x2f23c: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x2f240: beq 0x2f26c <entry+0x1c6c>  @ imm = #0x24
    c.b(C::Eq,0x2f26c)?;
    // 0x2f244: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x2f248: add r2, r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,24)?)?;
    // 0x2f24c: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2f250: blt 0x2f224 <entry+0x1c24>  @ imm = #-0x34
    c.b(C::Lt,0x2f224)?;
    // 0x2f254: b 0x2f280 <entry+0x1c80>  @ imm = #0x24
    c.b(C::Al,0x2f280)?;
    // 0x2f258: ldr r2, [pc, #0x124]        @ 0x1d84 <entry+0x1d84>
    c.literal(R::R2,0x2f384)?;
    // 0x2f25c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f260: mov r1, #288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,288)?)?;
    // 0x2f264: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2f268: b 0x2f2b8 <entry+0x1cb8>  @ imm = #0x48
    c.b(C::Al,0x2f2b8)?;
    // 0x2f26c: ldr r2, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,52)?)?;
    // 0x2f270: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f274: ldrls r3, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Ls,true,R::new(3)?,R::new(5)?,48)?)?;
    // 0x2f278: cmpls r3, r1
    c.emit(arm32::data_register(C::Ls,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f27c: bls 0x2f2c4 <entry+0x1cc4>  @ imm = #0x40
    c.b(C::Ls,0x2f2c4)?;
    // 0x2f280: ldr r1, [pc, #0x100]        @ 0x1d88 <entry+0x1d88>
    c.literal(R::R1,0x2f388)?;
    // 0x2f284: ldr r2, [pc, #0x100]        @ 0x1d8c <entry+0x1d8c>
    c.literal(R::R2,0x2f38c)?;
    // 0x2f288: mov r3, #58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,58)?)?;
    // 0x2f28c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f290: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x2f294: bl 0x2ec94 <entry+0x1694>  @ imm = #-0x608
    c.bl(0x2ec94)?;
    // 0x2f298: ldr r1, [pc, #0xf0]         @ 0x1d90 <entry+0x1d90>
    c.literal(R::R1,0x2f390)?;
    // 0x2f29c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f2a0: bl 0x2ec94 <entry+0x1694>  @ imm = #-0x614
    c.bl(0x2ec94)?;
    // 0x2f2a4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2f2a8: mov r1, #59
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,59)?)?;
    // 0x2f2ac: ldr r2, [pc, #0xd0]         @ 0x1d84 <entry+0x1d84>
    c.literal(R::R2,0x2f384)?;
    // 0x2f2b0: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x2f2b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f2b8: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2db10
    c.bl(0x17b0)?;
    // 0x2f2bc: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x2f2c0: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2f2c4: ldr r12, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(6)?,0)?)?;
    // 0x2f2c8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2f2cc: cmp r7, r12, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R12,Shift::Lsr,2,false)?)?;
    // 0x2f2d0: beq 0x2f318 <entry+0x1d18>  @ imm = #0x40
    c.b(C::Eq,0x2f318)?;
    // 0x2f2d4: subs r3, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,0,true)?)?;
    // 0x2f2d8: lsr r7, r12, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R12,Shift::Lsr,2)?)?;
    // 0x2f2dc: addls r3, r3, r1
    c.emit(arm32::data_register(C::Ls,Op::Add,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f2e0: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x2f2e4: bls 0x2f320 <entry+0x1d20>  @ imm = #0x34
    c.b(C::Ls,0x2f320)?;
    // 0x2f2e8: ldr r9, [pc, #0x90]         @ 0x1d80 <entry+0x1d80>
    c.literal(R::new(9)?,0x2f380)?;
    // 0x2f2ec: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2f2f0: add r8, r2, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x2f2f4: str r3, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,36)?)?;
    // 0x2f2f8: cmp r8, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f2fc: ldr r3, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,0)?)?;
    // 0x2f300: add r2, r3, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,2,false)?)?;
    // 0x2f304: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2f308: bhs 0x2f330 <entry+0x1d30>  @ imm = #0x20
    c.b(C::Cs,0x2f330)?;
    // 0x2f30c: bic r2, r12, #3
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R12,3)?)?;
    // 0x2f310: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2f314: b 0x2f358 <entry+0x1d58>  @ imm = #0x3c
    c.b(C::Al,0x2f358)?;
    // 0x2f318: mov r1, #70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,70)?)?;
    // 0x2f31c: b 0x2f2ac <entry+0x1cac>  @ imm = #-0x78
    c.b(C::Al,0x2f2ac)?;
    // 0x2f320: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f324: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2f328: str r0, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,36)?)?;
    // 0x2f32c: b 0x2f2bc <entry+0x1cbc>  @ imm = #-0x78
    c.b(C::Al,0x2f2bc)?;
    // 0x2f330: sub r8, r8, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(8)?,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f334: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2f338: sub r7, r7, r8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x2f33c: lsl r2, r7, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x2f340: bl 0x471c <entry+0xfffffffffffd711c> @ imm = #-0x2ac2c
    c.bl(0x471c)?;
    // 0x2f344: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x2f348: ldr r2, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,4)?)?;
    // 0x2f34c: add r1, r6, r7, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x2f350: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2f354: lsl r2, r8, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(8)?,Shift::Lsl,2)?)?;
    // 0x2f358: bl 0x471c <entry+0xfffffffffffd711c> @ imm = #-0x2ac44
    c.bl(0x471c)?;
    // 0x2f35c: bl 0x30cc <entry+0xfffffffffffd5acc> @ imm = #-0x2c298
    c.bl(0x30cc)?;
    // 0x2f360: str r8, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,52)?)?;
    // 0x2f364: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x2f368: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // 0x2f36c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2f370: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f374: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x2f378: b 0x2f2bc <entry+0x1cbc>  @ imm = #-0xc4
    c.b(C::Al,0x2f2bc)?;
    c.check(0x2f37c)?;
    const RING_HELPER_DATA_2F37C:[u32;6]=[0x0010ceac,0x00104114,0x000ead29,0x000eade9,0x000eae22,0x000e2788];
    for word in RING_HELPER_DATA_2F37C {c.emit(word)?;}
    c.check(0x2f394)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;inspect_ring_and_copy_record(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn ring_helpers_pool_boundaries_are_explicit(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let data=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(data(0x2f1c8),0x0010ceac);
        assert_eq!(data(0x2f1d8),0x000e2788);
        assert_eq!(data(0x2f37c),0x0010ceac);
        assert_eq!(data(0x2f390),0x000e2788);
    }
}
