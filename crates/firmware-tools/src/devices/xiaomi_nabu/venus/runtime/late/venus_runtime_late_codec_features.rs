// SPDX-License-Identifier: GPL-3.0-only
//! Late Venus codec feature and buffer-safety flag derivation. This function
//! interprets hardware capability register bits and codec plane bounds,
//! updating flags; it is A32 operand-level reconstruction, not OEM source.
use arm32::{BlockMode, Condition as C, DataOp as Op, Register as R, Shift};
use super::Code;
const START:usize=0x1e5c8;
const END:usize=0x1e704;
const FEATURE_WARNING:u32=0x000e53c3;

fn populate_codec_features(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1e5c8: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1e5cc: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x18)?)?;
    // 0x1e5d0: ldr r7, [r1, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::R1, 2648)?)?;
    // 0x1e5d4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1e5d8: ldr r9, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::new(9)?, R::R1, 88)?)?;
    // 0x1e5dc: ldr lr, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::R1, 92)?)?;
    // 0x1e5e0: ldr r8, [r1, #0x170]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::R1, 368)?)?;
    // 0x1e5e4: ldr r12, [r1, #0x1e8]
    c.emit(arm32::transfer_word(C::Al, true, R::R12, R::R1, 488)?)?;
    // 0x1e5e8: mov r6, r1
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R1))?;
    // 0x1e5ec: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x1e5f0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x0)?)?;
    // 0x1e5f4: ldr r0, [r7, #0x324]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 804)?)?;
    // 0x1e5f8: str r2, [r1, #0x924]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 2340)?)?;
    // 0x1e5fc: str r2, [r1, #0x928]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 2344)?)?;
    // 0x1e600: tst r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x1)?)?;
    // 0x1e604: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1e608: strne r2, [r1, #0x924]
    c.emit(arm32::transfer_word(C::Ne, false, R::R2, R::R1, 2340)?)?;
    // 0x1e60c: tst r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    // 0x1e610: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1e614: strne r0, [r1, #0x928]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::R1, 2344)?)?;
    // 0x1e618: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1e61c: str r0, [r6, #0x914]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, false, R::R0, R::new(6)?, 2324)?)?;
    // 0x1e620: str r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 4)?)?;
    // 0x1e624: str r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 8)?)?;
    // 0x1e628: str r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 12)?)?;
    // 0x1e62c: ldr r2, [r7, #0x334]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(7)?, 820)?)?;
    // 0x1e630: tst r2, #4
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R2, 0x4)?)?;
    // 0x1e634: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1e638: strne r3, [r6]
    c.emit(arm32::transfer_word(C::Ne, false, R::new(3)?, R::new(6)?, 0)?)?;
    // 0x1e63c: tst r2, #8
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R2, 0x8)?)?;
    // 0x1e640: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1e644: strne r0, [r1, #0x918]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::R1, 2328)?)?;
    // 0x1e648: ldr r5, [r7, #0x340]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(7)?, 832)?)?;
    // 0x1e64c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x1e650: tst r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::new(5)?, 0x1)?)?;
    // 0x1e654: movne r4, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(4)?, R::R0, 0x1)?)?;
    // 0x1e658: strne r4, [r1, #0x91c]
    c.emit(arm32::transfer_word(C::Ne, false, R::new(4)?, R::R1, 2332)?)?;
    // 0x1e65c: tst r5, #2
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::new(5)?, 0x2)?)?;
    // 0x1e660: and r5, r2, #3
    c.emit(arm32::data_immediate(C::Al, Op::And, R::new(5)?, R::R2, 0x3)?)?;
    // 0x1e664: movne r7, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x1e668: strne r7, [r1, #0x920]
    c.emit(arm32::transfer_word(C::Ne, false, R::new(7)?, R::R1, 2336)?)?;
    // 0x1e66c: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x1)?)?;
    // 0x1e670: bne 0x1e6cc <entry+0x3e4>  
    c.b(C::Ne, 0x1e6cc)?;
    // 0x1e674: lsr r5, r9, r4
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Mov, R::new(5)?, R::R0, R::new(9)?, Shift::Lsr, R::new(4)?, false)?)?;
    // 0x1e678: lsl r5, r5, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x1e67c: add r4, r5, r9, lsr r4
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Add, R::new(4)?, R::new(5)?, R::new(9)?, Shift::Lsr, R::new(4)?, false)?)?;
    // 0x1e680: add r5, r8, r8, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(5)?, R::new(8)?, R::new(8)?, Shift::Lsl, 1, false)?)?;
    // 0x1e684: lsl r4, r4, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::new(4)?, R::new(4)?, Shift::Lsl, 1)?)?;
    // 0x1e688: cmp r4, r5, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(4)?, R::new(5)?, Shift::Lsl, 3, false)?)?;
    // 0x1e68c: lsr r4, lr, r7
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Mov, R::new(4)?, R::R0, R::LR, Shift::Lsr, R::new(7)?, false)?)?;
    // 0x1e690: lsl r4, r4, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::new(4)?, R::new(4)?, Shift::Lsl, 2)?)?;
    // 0x1e694: movhi r3, #1
    c.emit(arm32::data_immediate(C::Hi, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1e698: add r7, r4, lr, lsr r7
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Add, R::new(7)?, R::new(4)?, R::LR, Shift::Lsr, R::new(7)?, false)?)?;
    // 0x1e69c: strhi r3, [r6]
    c.emit(arm32::transfer_word(C::Hi, false, R::new(3)?, R::new(6)?, 0)?)?;
    // 0x1e6a0: add r6, r12, r12, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(6)?, R::R12, R::R12, Shift::Lsl, 1, false)?)?;
    // 0x1e6a4: lsl r7, r7, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::new(7)?, Shift::Lsl, 1)?)?;
    // 0x1e6a8: cmp r7, r6, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::new(6)?, Shift::Lsl, 3, false)?)?;
    // 0x1e6ac: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1e6b0: strhi r0, [r1, #0x918]
    c.emit(arm32::transfer_word(C::Hi, false, R::R0, R::R1, 2328)?)?;
    // 0x1e6b4: orrs r7, r0, r3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(7)?, R::R0, R::new(3)?, Shift::Lsl, 0, true)?)?;
    // 0x1e6b8: movne r7, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x1e6bc: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(6)?, R::R0, 0x1)?)?;
    // 0x1e6c0: strne r7, [r1, #0x908]
    c.emit(arm32::transfer_word(C::Ne, false, R::new(7)?, R::R1, 2312)?)?;
    // 0x1e6c4: strne r6, [r1, #0x90c]
    c.emit(arm32::transfer_word(C::Ne, false, R::new(6)?, R::R1, 2316)?)?;
    // 0x1e6c8: b 0x1e6e0 <entry+0x3f8>  
    c.b(C::Al, 0x1e6e0)?;
    // 0x1e6cc: tst r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R2, 0x2)?)?;
    // 0x1e6d0: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1e6d4: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1e6d8: strne r0, [r1, #0x914]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::R1, 2324)?)?;
    // 0x1e6dc: strne r0, [r1, #0x918]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::R1, 2328)?)?;
    // 0x1e6e0: tst r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R2, 0x2)?)?;
    // 0x1e6e4: popeq {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1e6e8: ldr r2, [pc, #0x10]        
    c.literal(R::R2, 0x1e700)?;
    // 0x1e6ec: mov r1, #692
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2b4)?)?;
    // 0x1e6f0: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::R0, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 0x1e6f4: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    // 0x1e6f8: pop {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1e6fc: b 0x17b0 <entry+0xfffffffffffe34c8>
    c.b(C::Al, 0x17b0)?;

    c.check(0x1e700)?;
    c.emit(FEATURE_WARNING)?;
    c.check(END)
}
pub(super) fn extend(c:&mut Code)->Result<(),String>{
    populate_codec_features(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn capability_bitfield_output_has_exact_size(){
        let mut c=Code::new(START);extend(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        assert_eq!(&c.bytes[c.bytes.len()-4..],&FEATURE_WARNING.to_le_bytes());
    }
}
