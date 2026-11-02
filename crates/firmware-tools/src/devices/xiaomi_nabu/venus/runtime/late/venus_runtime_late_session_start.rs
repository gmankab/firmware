// SPDX-License-Identifier: GPL-3.0-only
//! Late Venus allocated codec-session start/stop setup. Uses typed ARM A32
//! operations and a checked diagnostics pool, not copied OEM opcodes.
use arm32::{BlockMode, Condition as C, DataOp as Op, Register as R, Shift};
use super::Code;

const START:usize=0x1c614;
const END:usize=0x1c7e8;
/// Diagnostic pointers and one source-identified sentinel value.
const SESSION_DIAGNOSTICS:[u32;9]=[
    0x000e550b, // diagnostic/sentinel 0
    0x000e2788, // diagnostic/sentinel 1
    0x000e5526, // diagnostic/sentinel 2
    0x000e554f, // diagnostic/sentinel 3
    0x000e5577, // diagnostic/sentinel 4
    0x000e559f, // diagnostic/sentinel 5
    0xdeadbeef, // diagnostic/sentinel 6
    0x000e54b4, // diagnostic/sentinel 7
    0x000e54ed, // diagnostic/sentinel 8
];

fn register_codec_session(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1c614: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1c618: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    // 0x1c61c: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x8)?)?;
    // 0x1c620: mov r7, r2
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x1c624: mov r5, r1
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R1))?;
    // 0x1c628: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1c62c: bl 0x23668 <entry+0x8368> 
    c.bl(0x23668)?;
    // 0x1c630: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 0x1c634: cmn r6, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmn, R::R0, R::new(6)?, 0x1)?)?;
    // 0x1c638: beq 0x1c768 <entry+0x1468> 
    c.b(C::Eq, 0x1c768)?;
    // 0x1c63c: ldr r1, [pc, #0x180]       
    c.literal(R::R1, 0x1c7c4)?;
    // 0x1c640: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c644: mov r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x1)?)?;
    // 0x1c648: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c64c: ldr r7, [pc, #0x174]       
    c.literal(R::new(7)?, 0x1c7c8)?;
    // 0x1c650: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c654: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1c658: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c65c: bl 0x25954 <entry+0xa654> 
    c.bl(0x25954)?;
    // 0x1c660: bl 0x25984 <entry+0xa684> 
    c.bl(0x25984)?;
    // 0x1c664: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1c668: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1c66c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1c670: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1c674: bl 0xd8b0 <entry+0xffffffffffff25b0>
    c.bl(0xd8b0)?;
    // 0x1c678: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1c67c: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1c680: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1c684: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1c688: bl 0x1b284 <entry+0xffffffffffffff84>
    c.bl(0x1b284)?;
    // 0x1c68c: ldr r1, [pc, #0x138]       
    c.literal(R::R1, 0x1c7cc)?;
    // 0x1c690: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c694: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1c698: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c69c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c6a0: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1c6a4: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c6a8: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1c6ac: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1c6b0: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1c6b4: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 8)?)?;
    // 0x1c6b8: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1c6bc: bl 0x25118 <entry+0x9e18> 
    c.bl(0x25118)?;
    // 0x1c6c0: ldr r1, [pc, #0x108]       
    c.literal(R::R1, 0x1c7d0)?;
    // 0x1c6c4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c6c8: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1c6cc: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c6d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c6d4: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1c6d8: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c6dc: ldr r0, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 140)?)?;
    // 0x1c6e0: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1c6e4: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1c6e8: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 8)?)?;
    // 0x1c6ec: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1c6f0: bl 0x25118 <entry+0x9e18> 
    c.bl(0x25118)?;
    // 0x1c6f4: ldr r1, [pc, #0xd8]        
    c.literal(R::R1, 0x1c7d4)?;
    // 0x1c6f8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c6fc: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1c700: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c704: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c708: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1c70c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c710: ldr r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 148)?)?;
    // 0x1c714: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1c718: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1c71c: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 8)?)?;
    // 0x1c720: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1c724: bl 0x25118 <entry+0x9e18> 
    c.bl(0x25118)?;
    // 0x1c728: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1c72c: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1c730: bl 0x21aa4 <entry+0x67a4> 
    c.bl(0x21aa4)?;
    // 0x1c734: ldr r1, [pc, #0x9c]        
    c.literal(R::R1, 0x1c7d8)?;
    // 0x1c738: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c73c: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1c740: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c744: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c748: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1c74c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c750: ldr r1, [pc, #0x84]        
    c.literal(R::R1, 0x1c7dc)?;
    // 0x1c754: add r0, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(4)?, R::new(6)?, Shift::Lsl, 2, false)?)?;
    // 0x1c758: str r1, [r0, #0x40]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 64)?)?;
    // 0x1c75c: bl 0x259a4 <entry+0xa6a4> 
    c.bl(0x259a4)?;
    // 0x1c760: bl 0x25974 <entry+0xa674> 
    c.bl(0x25974)?;
    // 0x1c764: b 0x1c7b8 <entry+0x14b8> 
    c.b(C::Al, 0x1c7b8)?;
    // 0x1c768: ldr r1, [pc, #0x70]        
    c.literal(R::R1, 0x1c7e0)?;
    // 0x1c76c: ldr r2, [pc, #0x70]        
    c.literal(R::R2, 0x1c7e4)?;
    // 0x1c770: mov r3, #182
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xb6)?)?;
    // 0x1c774: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1c778: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 0)?)?;
    // 0x1c77c: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x100)?)?;
    // 0x1c780: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c784: ldr r1, [pc, #0x3c]        
    c.literal(R::R1, 0x1c7c8)?;
    // 0x1c788: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1c78c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c790: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 128)?)?;
    // 0x1c794: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1c798: mov r3, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x4)?)?;
    // 0x1c79c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x2)?)?;
    // 0x1c7a0: orr r1, r1, #2162688
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x210000)?)?;
    // 0x1c7a4: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x1c7a8: stm sp, {r1, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R1.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c7ac: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1c7b0: bl 0x23ca8 <entry+0x89a8> 
    c.bl(0x23ca8)?;
    // 0x1c7b4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x1c7b8: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1c7bc: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x10)?)?;
    // 0x1c7c0: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    c.check(0x1c7c4)?;
    for word in SESSION_DIAGNOSTICS { c.emit(word)?; }
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    register_codec_session(c)?;
    c.check(END)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn session_diagnostics_have_exact_layout(){
        let mut c=Code::new(START);
        extend(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        for (i,word) in SESSION_DIAGNOSTICS.iter().enumerate(){
            let offset=0x1c7c4-START+i*4;
            assert_eq!(&c.bytes[offset..offset+4],&word.to_le_bytes());
        }
    }
}
