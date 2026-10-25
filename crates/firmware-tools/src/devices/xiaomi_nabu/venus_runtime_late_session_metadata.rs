// SPDX-License-Identifier: GPL-3.0-only
//! Register codec session diagnostics, hardware state and queue writing.
//! A32 instructions re-encoded from operands, literal pools are separate.
use arm32::{BlockMode, Condition as C, DataOp as Op, Register as R, Shift};
use super::Code;
const START:usize=0x1c7e8;
const END:usize=0x1c9c8;
const SESSION_LOGS:[u32;6]=[
    0x000e53c3,0x000e5623,0x000e560e,0x000e2788,0x000e55c5,0x000e55e5,
];
const RESOURCE_DESCRIPTORS:[u32;3]=[0x04021010,0x000e564a,0x000e2788];

/// Enqueue a pending codec session identifier and log success/error paths.
fn write_session_status(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1c7e8: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1c7ec: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    // 0x1c7f0: mov r5, r2
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R2))?;
    // 0x1c7f4: ldr r2, [pc, #0x11c]       
    c.literal(R::R2, 0x1c918)?;
    // 0x1c7f8: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x1c7fc: mov r1, #231
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xe7)?)?;
    // 0x1c800: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 0x1c804: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x100)?)?;
    // 0x1c808: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1c80c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 0)?)?;
    // 0x1c810: ldr r6, [r6, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(6)?, 144)?)?;
    // 0x1c814: bic r1, r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R1, R::R0, 0x3)?)?;
    // 0x1c818: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x4)?)?;
    // 0x1c81c: str r5, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al, false, R::new(5)?, R::new(4)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x1c820: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1c824: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1c828: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 4)?)?;
    // 0x1c82c: orr r1, r1, #2162688
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x210000)?)?;
    // 0x1c830: cmp r0, r1
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1c834: bne 0x1c8a8 <entry+0xc0>   
    c.b(C::Ne, 0x1c8a8)?;
    // 0x1c838: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 12)?)?;
    // 0x1c83c: ldr r2, [pc, #0xd4]        
    c.literal(R::R2, 0x1c918)?;
    // 0x1c840: mov r1, #508
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1fc)?)?;
    // 0x1c844: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1c848: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c84c: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1c850: ldr r1, [pc, #0xd0]        
    c.literal(R::R1, 0x1c928)?;
    // 0x1c854: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c858: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1c85c: ldr r1, [pc, #0xc0]        
    c.literal(R::R1, 0x1c924)?;
    // 0x1c860: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c864: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1c868: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 12)?)?;
    // 0x1c86c: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1c870: bl 0x80928 <entry+0x64140>
    c.bl(0x80928)?;
    // 0x1c874: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1c878: beq 0x1c90c <entry+0x124>  
    c.b(C::Eq, 0x1c90c)?;
    // 0x1c87c: ldr r1, [pc, #0xa8]        
    c.literal(R::R1, 0x1c92c)?;
    // 0x1c880: ldr r2, [pc, #0x98]        
    c.literal(R::R2, 0x1c920)?;
    // 0x1c884: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1c888: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1c88c: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x200)?)?;
    // 0x1c890: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1c894: ldr r1, [pc, #0x88]        
    c.literal(R::R1, 0x1c924)?;
    // 0x1c898: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1c89c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1c8a0: mov r1, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2)?)?;
    // 0x1c8a4: b 0x1c8fc <entry+0x114>  
    c.b(C::Al, 0x1c8fc)?;
    // 0x1c8a8: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1c8ac: ldr r2, [pc, #0x64]        
    c.literal(R::R2, 0x1c918)?;
    // 0x1c8b0: mov r1, #7
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x7)?)?;
    // 0x1c8b4: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1c8b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1c8bc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c8c0: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1c8c4: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1c8c8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1c8cc: bl 0x80928 <entry+0x64140>
    c.bl(0x80928)?;
    // 0x1c8d0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1c8d4: beq 0x1c90c <entry+0x124>  
    c.b(C::Eq, 0x1c90c)?;
    // 0x1c8d8: ldr r1, [pc, #0x3c]        
    c.literal(R::R1, 0x1c91c)?;
    // 0x1c8dc: ldr r2, [pc, #0x3c]        
    c.literal(R::R2, 0x1c920)?;
    // 0x1c8e0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1c8e4: mov r3, #524
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x20c)?)?;
    // 0x1c8e8: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1c8ec: ldr r1, [pc, #0x30]        
    c.literal(R::R1, 0x1c924)?;
    // 0x1c8f0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1c8f4: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1c8f8: mov r1, #13
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xd)?)?;
    // 0x1c8fc: ldr r2, [pc, #0x14]        
    c.literal(R::R2, 0x1c918)?;
    // 0x1c900: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1c904: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1c908: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1c90c: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 16)?)?;
    // 0x1c910: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1c914: b 0x292c <entry+0xfffffffffffe6144>
    c.b(C::Al, 0x292c)?;
    c.check(0x1c918)?;
    for word in SESSION_LOGS {c.emit(word)?;}
    c.check(0x1c930)
}
/// Copy three descriptor pointers and return from a register-only helper.
fn tiny_copy_helper(c:&mut Code)->Result<(),String>{
    c.check(0x1c930)?;
    // 0x1c930: ldm r1, {r2, r3, r12}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::R12.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c934: stm r0, {r2, r3, r12}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::R12.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c938: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x1c93c)
}
/// Send the controller's three-record metadata to the device interface.
fn send_session_metadata(c:&mut Code)->Result<(),String>{
    c.check(0x1c93c)?;
    // 0x1c93c: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1c940: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x18)?)?;
    // 0x1c944: sub sp, sp, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x800)?)?;
    // 0x1c948: mov r4, r3
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(3)?))?;
    // 0x1c94c: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x1c950: ldr r0, [pc, #0x64]        
    c.literal(R::R0, 0x1c9bc)?;
    // 0x1c954: ldr r3, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R11, 8)?)?;
    // 0x1c958: mov r8, sp
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::SP))?;
    // 0x1c95c: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 4)?)?;
    // 0x1c960: str r3, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::SP, 8)?)?;
    // 0x1c964: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 12)?)?;
    // 0x1c968: ldr r1, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R11, 12)?)?;
    // 0x1c96c: str r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 16)?)?;
    // 0x1c970: add r0, r8, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(8)?, 0x14)?)?;
    // 0x1c974: ldm r1, {r2, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(6)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c978: ldr r1, [pc, #0x40]        
    c.literal(R::R1, 0x1c9c0)?;
    // 0x1c97c: stm r0, {r2, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(6)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c980: mov r0, #32
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x20)?)?;
    // 0x1c984: mov r2, r3
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(3)?))?;
    // 0x1c988: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 0)?)?;
    // 0x1c98c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1c990: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1c994: ldr r1, [pc, #0x28]        
    c.literal(R::R1, 0x1c9c4)?;
    // 0x1c998: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1c99c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1c9a0: ldr r0, [r5, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 128)?)?;
    // 0x1c9a4: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 0x1c9a8: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 0x1c9ac: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1c9b0: bl 0x2a198 <entry+0xd9b0> 
    c.bl(0x2a198)?;
    // 0x1c9b4: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x18)?)?;
    // 0x1c9b8: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x1c9bc)?;
    for word in RESOURCE_DESCRIPTORS {c.emit(word)?;}
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    write_session_status(c)?;
    tiny_copy_helper(c)?;
    send_session_metadata(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn session_status_log_and_metadata_are_adjacent(){
        let mut c=Code::new(START); extend(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        for (i,v) in SESSION_LOGS.iter().enumerate(){
            let at=0x1c918-START+i*4;
            assert_eq!(&c.bytes[at..at+4],&v.to_le_bytes());
        }
        for (i,v) in RESOURCE_DESCRIPTORS.iter().enumerate(){
            let at=0x1c9bc-START+i*4;
            assert_eq!(&c.bytes[at..at+4],&v.to_le_bytes());
        }
    }
}
