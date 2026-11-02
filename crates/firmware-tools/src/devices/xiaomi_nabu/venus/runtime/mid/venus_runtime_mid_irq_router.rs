// SPDX-License-Identifier: GPL-3.0-only
//! Controller IRQ routing flags and MMIO dispatch. The four trailing
//! register/diagnostic pointers are data, never decoded as A32 instructions.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2fbc4;
const END:usize=0x2fcbc;
fn update_mmio_irq_route(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2fbc4: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2fbc8: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x2fbcc: ldr r6, [pc, #0xd8]         @ 0x6ac <entry+0x6ac>
    c.literal(R::new(6)?,0x2fcac)?;
    // 0x2fbd0: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x2fbd4: ldr r1, [pc, #0xd4]         @ 0x6b0 <entry+0x6b0>
    c.literal(R::R1,0x2fcb0)?;
    // 0x2fbd8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2fbdc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2fbe0: ldr r2, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,0)?)?;
    // 0x2fbe4: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xf58
    c.bl(0x2ec94)?;
    // 0x2fbe8: ldr r1, [pc, #0xc4]         @ 0x6b4 <entry+0x6b4>
    c.literal(R::R1,0x2fcb4)?;
    // 0x2fbec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2fbf0: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xf64
    c.bl(0x2ec94)?;
    // 0x2fbf4: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x2fbf8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2fbfc: popeq {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2fc00: ldr r3, [pc, #0xb0]         @ 0x6b8 <entry+0x6b8>
    c.literal(R::new(3)?,0x2fcb8)?;
    // 0x2fc04: mov lr, #7936
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,7936)?)?;
    // 0x2fc08: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x2fc0c: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x2fc10: tst r5, #512
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,512)?)?;
    // 0x2fc14: ldr r0, [r3, r4, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,2)?)?;
    // 0x2fc18: and r0, lr, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::LR,R::R0,Shift::Lsl,8,false)?)?;
    // 0x2fc1c: bne 0x2fc28 <entry+0x628>   @ imm = #0x4
    c.b(C::Ne,0x2fc28)?;
    // 0x2fc20: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x2fc24: b 0x2fc54 <entry+0x654>   @ imm = #0x28
    c.b(C::Al,0x2fc54)?;
    // 0x2fc28: rsb r2, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R2,R::new(4)?,1)?)?;
    // 0x2fc2c: mov r5, #520093696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,520093696)?)?;
    // 0x2fc30: mov r6, #2031616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2031616)?)?;
    // 0x2fc34: ldr r2, [r3, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
    // 0x2fc38: and r5, r5, r2, lsl #24
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::R2,Shift::Lsl,24,false)?)?;
    // 0x2fc3c: and r6, r6, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(6)?,R::R2,Shift::Lsl,16,false)?)?;
    // 0x2fc40: orr r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2fc44: and r5, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R2,31)?)?;
    // 0x2fc48: mov r2, #272
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,272)?)?;
    // 0x2fc4c: orr r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x2fc50: orr r2, r2, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,69632)?)?;
    // 0x2fc54: rsb r6, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(6)?,R::new(4)?,1)?)?;
    // 0x2fc58: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x2fc5c: ldr r3, [r3, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x2fc60: mov r6, #820
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,820)?)?;
    // 0x2fc64: orr r6, r6, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,14336)?)?;
    // 0x2fc68: orr r6, r6, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x2fc6c: str r0, [r6, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // 0x2fc70: mov r0, #824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,824)?)?;
    // 0x2fc74: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x2fc78: and r3, lr, r3, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::LR,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x2fc7c: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x2fc80: orr r3, r3, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x2fc84: str r3, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x2fc88: mov r0, #828
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,828)?)?;
    // 0x2fc8c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x2fc90: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x2fc94: str r12, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R12,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x2fc98: mov r0, #2864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2864)?)?;
    // 0x2fc9c: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x2fca0: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x2fca4: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x2fca8: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2fcac)?;
    const ROUTER_MMIO_DATA:[u32;4]=[0x00104124,0x000eb021,0x000e2788,0x0010411c];
    for word in ROUTER_MMIO_DATA {c.emit(word)?;}
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;update_mmio_irq_route(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn irq_router_literals_match_bounded_span(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let tail=&c.bytes[c.bytes.len()-16..];
        for (i,v) in [0x00104124u32,0x000eb021,0x000e2788,0x0010411c].iter().enumerate(){
            assert_eq!(&tail[4*i..4*i+4],&v.to_le_bytes());
        }
    }
}
