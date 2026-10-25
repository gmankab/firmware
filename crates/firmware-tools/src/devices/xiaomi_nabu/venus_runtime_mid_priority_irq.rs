// SPDX-License-Identifier: GPL-3.0-only
//! Codec controller interrupt queue prioritization and acknowledgment.
//! On a pending event, mask and clear the IRQ channel and call the queue
//! callback. Data words below are host MMIO pointers, not A32 instructions.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x296f0;
const END:usize=0x29850;
const CONTROLLER_HOST_DATA:[u32;2]=[
  0x0010_ceac, // handler registration table
  0x0010_40d4, // pending interrupt counter
];
fn acknowledge_prioritized_event(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x296f0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x296f4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x296f8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x296fc: mov r1, #3648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3648)?)?;
    // 0x29700: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x29704: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x29708: orr r1, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // 0x2970c: lsl r0, r0, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,15)?)?;
    // 0x29710: ldr r5, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    // 0x29714: str r5, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,16)?)?;
    // 0x29718: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // 0x2971c: tst r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,8)?)?;
    // 0x29720: bne 0x2977c <entry+0x4e0>   @ imm = #0x54
    c.b(C::Ne,0x2977c)?;
    // 0x29724: tst r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,1)?)?;
    // 0x29728: beq 0x29818 <entry+0x57c>   @ imm = #0xe8
    c.b(C::Eq,0x29818)?;
    // 0x2972c: ldr r2, [pc, #0x118]        @ 0x5b0 <entry+0x5b0>
    c.literal(R::R2,0x2984c)?;
    // 0x29730: orr r0, r0, #580
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,580)?)?;
    // 0x29734: orr r1, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,3072)?)?;
    // 0x29738: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // 0x2973c: str r5, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x29740: ldr r1, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // 0x29744: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x29748: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // 0x2974c: ldrb r2, [r4, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,8)?)?;
    // 0x29750: tst r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,1)?)?;
    // 0x29754: bne 0x2981c <entry+0x580>   @ imm = #0xc0
    c.b(C::Ne,0x2981c)?;
    // 0x29758: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x2975c: popne {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x29760: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x29764: mov r1, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28)?)?;
    // 0x29768: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2976c: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // 0x29770: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x29774: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x29778: b 0x29830 <entry+0x594>   @ imm = #0xb0
    c.b(C::Al,0x29830)?;
    // 0x2977c: and r2, r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(5)?,3)?)?;
    // 0x29780: and r6, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(5)?,1)?)?;
    // 0x29784: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x29788: bne 0x297bc <entry+0x520>   @ imm = #0x2c
    c.b(C::Ne,0x297bc)?;
    // 0x2978c: ands r1, r1, #32
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R1,32,true)?)?;
    // 0x29790: beq 0x297bc <entry+0x520>   @ imm = #0x24
    c.b(C::Eq,0x297bc)?;
    // 0x29794: orr r0, r0, #580
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,580)?)?;
    // 0x29798: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x2979c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x297a0: orr r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // 0x297a4: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x297a8: ldr r1, [pc, #0x98]         @ 0x5ac <entry+0x5ac>
    c.literal(R::R1,0x29848)?;
    // 0x297ac: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x297b0: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x297b4: add r0, r0, #52
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,52)?)?;
    // 0x297b8: bl 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x26e94
    c.bl(0x292c)?;
    // 0x297bc: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x297c0: beq 0x29818 <entry+0x57c>   @ imm = #0x50
    c.b(C::Eq,0x29818)?;
    // 0x297c4: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x297c8: mov r1, #580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,580)?)?;
    // 0x297cc: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    // 0x297d0: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // 0x297d4: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x297d8: str r5, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x297dc: ldr r1, [pc, #0x68]         @ 0x5b0 <entry+0x5b0>
    c.literal(R::R1,0x2984c)?;
    // 0x297e0: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x297e4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x297e8: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x297ec: ldrb r1, [r4, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // 0x297f0: tst r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1)?)?;
    // 0x297f4: bne 0x29800 <entry+0x564>   @ imm = #0x4
    c.b(C::Ne,0x29800)?;
    // 0x297f8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x297fc: bne 0x29818 <entry+0x57c>   @ imm = #0x14
    c.b(C::Ne,0x29818)?;
    // 0x29800: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x29804: ldr r1, [pc, #0x3c]         @ 0x5ac <entry+0x5ac>
    c.literal(R::R1,0x29848)?;
    // 0x29808: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x2980c: add r0, r0, #52
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,52)?)?;
    // 0x29810: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x29814: b 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x26ef0
    c.b(C::Al,0x292c)?;
    // 0x29818: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2981c: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x29820: mov r2, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // 0x29824: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // 0x29828: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2982c: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x29830: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x29834: ldr r1, [pc, #0xc]          @ 0x5ac <entry+0x5ac>
    c.literal(R::R1,0x29848)?;
    // 0x29838: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x2983c: add r0, r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,44)?)?;
    // 0x29840: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x29844: b 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x26f20
    c.b(C::Al,0x292c)?;
    c.check(0x29848)?;
    for value in CONTROLLER_HOST_DATA {c.emit(value)?;}
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{acknowledge_prioritized_event(c)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_irq_mask_has_correct_literal_pool(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        assert_eq!(&c.bytes[c.bytes.len()-8..], &CONTROLLER_HOST_DATA.map(u32::to_le_bytes).concat());
    }
}
