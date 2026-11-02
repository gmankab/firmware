// SPDX-License-Identifier: GPL-3.0-only
//! Three compact control-plane operations: codec status writeback, IRQ bank
//! initialization, and service command validation. Non-executable MMIO and
//! diagnostics are represented as data words separate from typed A32 code.
use arm32::{Condition as C, DataOp as Op, Register as R, BlockMode};
use super::Code;
const START:usize=0x2b13c;
const END:usize=0x2b304;

/// configure control status implemented from checked ARM A32 operands.
fn configure_control_status(c:&mut Code)->Result<(),String>{
    c.check(0x2b13c)?;
    // 0x2b13c: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2b140: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2b144: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2b148: ldr r0, [pc, #0x24]         @ 0xd74 <entry+0xd74>
    c.literal(R::R0,0x2b174)?;
    // 0x2b14c: mov r1, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32768)?)?;
    // 0x2b150: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2b154: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // 0x2b158: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x2b15c: bl 0x2d20 <entry+0xfffffffffffd8920> @ imm = #-0x28444
    c.bl(0x2d20)?;
    // 0x2b160: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x2b164: mov r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2147483648)?)?;
    // 0x2b168: bl 0x2d20 <entry+0xfffffffffffd8920> @ imm = #-0x28450
    c.bl(0x2d20)?;
    // 0x2b16c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2b170: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2b174)?;
    const STATUS_MMIO:[u32;1]=[0xe00df01c];
    for x in STATUS_MMIO {c.emit(x)?;}
    c.check(0x2b178)
}

/// initialize codec irq bank implemented from checked ARM A32 operands.
fn initialize_codec_irq_bank(c:&mut Code)->Result<(),String>{
    c.check(0x2b178)?;
    // 0x2b178: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2b17c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2b180: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2b184: ldr r0, [pc, #0x38]         @ 0xdc4 <entry+0xdc4>
    c.literal(R::R0,0x2b1c4)?;
    // 0x2b188: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2b18c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2b190: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2b194: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2b198: mov r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16384)?)?;
    // 0x2b19c: ldr r0, [pc, #0x24]         @ 0xdc8 <entry+0xdc8>
    c.literal(R::R0,0x2b1c8)?;
    // 0x2b1a0: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2b1a4: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // 0x2b1a8: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x2b1ac: bl 0x2d20 <entry+0xfffffffffffd8920> @ imm = #-0x28494
    c.bl(0x2d20)?;
    // 0x2b1b0: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x2b1b4: mov r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2147483648)?)?;
    // 0x2b1b8: bl 0x2d20 <entry+0xfffffffffffd8920> @ imm = #-0x284a0
    c.bl(0x2d20)?;
    // 0x2b1bc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2b1c0: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2b1c4)?;
    const IRQ_BANK_MMIO:[u32;2]=[0xe00d2130,0xe00df01c];
    for x in IRQ_BANK_MMIO {c.emit(x)?;}
    c.check(0x2b1cc)
}

/// validate controller service message implemented from checked ARM A32 operands.
fn validate_controller_service_message(c:&mut Code)->Result<(),String>{
    c.check(0x2b1cc)?;
    // 0x2b1cc: push {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2b1d0: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x2b1d4: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // 0x2b1d8: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x2b1dc: cmp r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,12)?)?;
    // 0x2b1e0: bne 0x2b21c <entry+0xe1c>   @ imm = #0x34
    c.b(C::Ne,0x2b21c)?;
    // 0x2b1e4: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x2b1e8: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x2b1ec: beq 0x2b254 <entry+0xe54>   @ imm = #0x60
    c.b(C::Eq,0x2b254)?;
    // 0x2b1f0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2b1f4: beq 0x2b294 <entry+0xe94>   @ imm = #0x98
    c.b(C::Eq,0x2b294)?;
    // 0x2b1f8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2b1fc: bne 0x2b2a8 <entry+0xea8>   @ imm = #0xa4
    c.b(C::Ne,0x2b2a8)?;
    // 0x2b200: ldr r2, [pc, #0xf4]         @ 0xefc <entry+0xefc>
    c.literal(R::R2,0x2b2fc)?;
    // 0x2b204: mov r1, #97
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,97)?)?;
    // 0x2b208: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2b20c: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x2b210: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x29a68
    c.bl(0x17b0)?;
    // 0x2b214: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b218: b 0x2b2e4 <entry+0xee4>   @ imm = #0xc4
    c.b(C::Al,0x2b2e4)?;
    // 0x2b21c: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x2b220: ldr r1, [pc, #0xc4]         @ 0xeec <entry+0xeec>
    c.literal(R::R1,0x2b2ec)?;
    // 0x2b224: ldr r2, [pc, #0xc4]         @ 0xef0 <entry+0xef0>
    c.literal(R::R2,0x2b2f0)?;
    // 0x2b228: mov r3, #344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,344)?)?;
    // 0x2b22c: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x2b230: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x2b234: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x2b238: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b23c: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3a50
    c.bl(0x2ec94)?;
    // 0x2b240: ldr r1, [pc, #0xac]         @ 0xef4 <entry+0xef4>
    c.literal(R::R1,0x2b2f4)?;
    // 0x2b244: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b248: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3a44
    c.bl(0x2ec94)?;
    // 0x2b24c: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x2b250: b 0x2b2e4 <entry+0xee4>   @ imm = #0x8c
    c.b(C::Al,0x2b2e4)?;
    // 0x2b254: ldr r1, [pc, #0x9c]         @ 0xef8 <entry+0xef8>
    c.literal(R::R1,0x2b2f8)?;
    // 0x2b258: ldr r2, [pc, #0x90]         @ 0xef0 <entry+0xef0>
    c.literal(R::R2,0x2b2f0)?;
    // 0x2b25c: mov r3, #109
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,109)?)?;
    // 0x2b260: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b264: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2b268: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3a24
    c.bl(0x2ec94)?;
    // 0x2b26c: ldr r1, [pc, #0x80]         @ 0xef4 <entry+0xef4>
    c.literal(R::R1,0x2b2f4)?;
    // 0x2b270: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b274: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3a18
    c.bl(0x2ec94)?;
    // 0x2b278: bl 0x8e98 <entry+0xfffffffffffdea98> @ imm = #-0x223e8
    c.bl(0x8e98)?;
    // 0x2b27c: bl 0x25c48 <entry+0xffffffffffffb848> @ imm = #-0x563c
    c.bl(0x25c48)?;
    // 0x2b280: bl 0x25c60 <entry+0xffffffffffffb860> @ imm = #-0x5628
    c.bl(0x25c60)?;
    // 0x2b284: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x2b288: bl 0x25c24 <entry+0xffffffffffffb824> @ imm = #-0x566c
    c.bl(0x25c24)?;
    // 0x2b28c: bl 0xe2248 <entry+0xb7e48> @ imm = #0xb6fb4
    c.bl(0xe2248)?;
    // 0x2b290: b 0x2b290 <entry+0xe90>   @ imm = #-0x8
    c.b(C::Al,0x2b290)?;
    // 0x2b294: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b298: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2b29c: str r0, [r11, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-4)?)?;
    // 0x2b2a0: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x2b2a4: b 0x2b2e4 <entry+0xee4>   @ imm = #0x38
    c.b(C::Al,0x2b2e4)?;
    // 0x2b2a8: ldr r1, [pc, #0x50]         @ 0xf00 <entry+0xf00>
    c.literal(R::R1,0x2b300)?;
    // 0x2b2ac: ldr r2, [pc, #0x3c]         @ 0xef0 <entry+0xef0>
    c.literal(R::R2,0x2b2f0)?;
    // 0x2b2b0: mov r3, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,130)?)?;
    // 0x2b2b4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b2b8: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2b2bc: bl 0x2ec94 <entry+0x4894>  @ imm = #0x39d0
    c.bl(0x2ec94)?;
    // 0x2b2c0: ldr r1, [pc, #0x2c]         @ 0xef4 <entry+0xef4>
    c.literal(R::R1,0x2b2f4)?;
    // 0x2b2c4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b2c8: bl 0x2ec94 <entry+0x4894>  @ imm = #0x39c4
    c.bl(0x2ec94)?;
    // 0x2b2cc: ldr r2, [pc, #0x28]         @ 0xefc <entry+0xefc>
    c.literal(R::R2,0x2b2fc)?;
    // 0x2b2d0: mov r1, #131
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,131)?)?;
    // 0x2b2d4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2b2d8: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x2b2dc: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x29b34
    c.bl(0x17b0)?;
    // 0x2b2e0: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x2b2e4: mov sp, r11
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // 0x2b2e8: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2b2ec)?;
    const SERVICE_MESSAGES:[u32;6]=[0x000e8bf7,0x000e8c45,0x000e2788,0x000e8c59,0x000e884d,0x000e8c74];
    for x in SERVICE_MESSAGES {c.emit(x)?;}
    c.check(0x2b304)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    configure_control_status(c)?;
    initialize_codec_irq_bank(c)?;
    validate_controller_service_message(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn irq_status_and_message_pools_have_exact_layout(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|vma:usize|u32::from_le_bytes(c.bytes[vma-START..vma-START+4].try_into().unwrap());
        assert_eq!(word(0x2b174),0xe00df01c);
        assert_eq!(word(0x2b1c4),0xe00d2130);
        assert_eq!(word(0x2b1c8),0xe00df01c);
        assert_eq!(word(0x2b2ec),0x000e8bf7);
        assert_eq!(word(0x2b300),0x000e8c74);
    }
}
