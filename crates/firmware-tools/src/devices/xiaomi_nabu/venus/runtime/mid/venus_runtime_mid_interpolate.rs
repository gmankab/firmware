// SPDX-License-Identifier: GPL-3.0-only
//! Codec-format interpolation and scalar quantization helper. This routine
//! performs table lookups, multiplies, integer interpolation, bit-width
//! checks and Thumb-interworking BLX calls. The fractional scale/threshold
//! words and device data pointers are explicit non-executable A32 literals.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x3701c;
const END:usize=0x37180;
const INTERPOLATION_CONSTANTS:[u32;6]=[
    0x8888_8889,0x6666_6666,0x3fe6_6666,
    0x007d_0fff,0x000e_dad0,0x000e_dae8,
];
pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3701c: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x37020: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x37024: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x37028: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x3702c: ldr r0, [r6, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,96)?)?;
    // 0x37030: cmp r0, #1966080
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1966080)?)?;
    // 0x37034: cmpne r0, #30
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,30)?)?;
    // 0x37038: bne 0x37160 <entry+0x1cd4>  @ imm = #0x120
    c.b(C::Ne,0x37160)?;
    // 0x3703c: ldr r2, [r6, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,76)?)?;
    // 0x37040: ldr r3, [r6, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,80)?)?;
    // 0x37044: ldr r1, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,4)?)?;
    // 0x37048: ldr r0, [r6, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,100)?)?;
    // 0x3704c: mul r5, r3, r2
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::new(3)?,R::R2)?)?;
    // 0x37050: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x37054: lsr r4, r5, #12
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(5)?,Shift::Lsr,12)?)?;
    // 0x37058: bne 0x370a8 <entry+0x1c1c>  @ imm = #0x48
    c.b(C::Ne,0x370a8)?;
    // 0x3705c: blx 0x82ed0 <entry+0x4da44> @ imm = #0x4be6c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82ed0)?)?;
    // 0x37060: ldr r2, [pc, #0x104]        @ 0x1ce0 <entry+0x1ce0>
    c.literal_cond(C::Al,R::R2,0x3716c)?;
    // 0x37064: ldr r3, [pc, #0x104]        @ 0x1ce4 <entry+0x1ce4>
    c.literal_cond(C::Al,R::new(3)?,0x37170)?;
    // 0x37068: blx 0x823b0 <entry+0x4cf24> @ imm = #0x4b340
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x823b0)?)?;
    // 0x3706c: mov r3, #4063232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4063232)?)?;
    // 0x37070: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x37074: orr r3, r3, #1073741824
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1073741824)?)?;
    // 0x37078: blx 0x823b0 <entry+0x4cf24> @ imm = #0x4b330
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x823b0)?)?;
    // 0x3707c: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x37080: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x37084: mov r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // 0x37088: blx 0x82ed0 <entry+0x4da44> @ imm = #0x4be40
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82ed0)?)?;
    // 0x3708c: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x37090: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x37094: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x37098: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x3709c: blx 0x823b0 <entry+0x4cf24> @ imm = #0x4b30c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x823b0)?)?;
    // 0x370a0: blx 0x82d30 <entry+0x4d8a4> @ imm = #0x4bc88
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82d30)?)?;
    // 0x370a4: b 0x370bc <entry+0x1c30>  @ imm = #0x10
    c.b(C::Al,0x370bc)?;
    // 0x370a8: ldr r1, [pc, #0xb8]         @ 0x1cdc <entry+0x1cdc>
    c.literal_cond(C::Al,R::R1,0x37168)?;
    // 0x370ac: umull r2, r3, r0, r1
    c.emit(super::encode_long_multiply(C::Al,false,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x370b0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x370b4: lsr r0, r3, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(3)?,Shift::Lsr,4)?)?;
    // 0x370b8: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4cb8c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x370bc: ldr r1, [pc, #0xb0]         @ 0x1ce8 <entry+0x1ce8>
    c.literal_cond(C::Al,R::R1,0x37174)?;
    // 0x370c0: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x370c4: sub r0, r5, #102400
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(5)?,102400)?)?;
    // 0x370c8: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x370cc: bhi 0x37160 <entry+0x1cd4>  @ imm = #0x8c
    c.b(C::Hi,0x37160)?;
    // 0x370d0: ldrb r0, [r6, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,348)?)?;
    // 0x370d4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x370d8: bne 0x37160 <entry+0x1cd4>  @ imm = #0x80
    c.b(C::Ne,0x37160)?;
    // 0x370dc: ldr r0, [pc, #0x94]         @ 0x1cec <entry+0x1cec>
    c.literal_cond(C::Al,R::R0,0x37178)?;
    // 0x370e0: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x370e4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x370e8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x370ec: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x370f0: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // 0x370f4: and r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x370f8: ldr r5, [r0, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x370fc: cmp r5, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x37100: movhs r2, r6
    c.emit(arm32::mov_register(C::Cs,R::R2,R::new(6)?))?;
    // 0x37104: movlo r1, r6
    c.emit(arm32::mov_register(C::Cc,R::R1,R::new(6)?))?;
    // 0x37108: addlo r6, r6, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x3710c: andlo r5, r6, r3
    c.emit(arm32::data_register(C::Cc,Op::And,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x37110: cmplo r5, #6
    c.emit(arm32::data_immediate(C::Cc,Op::Cmp,R::R0,R::new(5)?,6)?)?;
    // 0x37114: blo 0x370f4 <entry+0x1c68>  @ imm = #-0x28
    c.b(C::Cc,0x370f4)?;
    // 0x37118: mov r3, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1020)?)?;
    // 0x3711c: orr r3, r3, #261120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,261120)?)?;
    // 0x37120: and r6, r3, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(3)?,R::R1,Shift::Lsl,2,false)?)?;
    // 0x37124: and r2, r3, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,2,false)?)?;
    // 0x37128: ldr r3, [pc, #0x4c]         @ 0x1cf0 <entry+0x1cf0>
    c.literal_cond(C::Al,R::new(3)?,0x3717c)?;
    // 0x3712c: ldr r1, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x37130: ldr r0, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x37134: ldr r5, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x37138: ldr r2, [r3, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(3)?,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x3713c: sub r3, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x37140: sub r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x37144: sub r2, r2, r5
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x37148: mul r0, r2, r3
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::new(3)?)?)?;
    // 0x3714c: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4caf8
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x37150: add r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x37154: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x37158: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3715c: movls r8, #2
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(8)?,R::R0,2)?)?;
    // 0x37160: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x37164: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x37168)?;
    for word in INTERPOLATION_CONSTANTS{c.emit(word)?;}
    c.check(END)
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn interpolation_scaling_literals_are_disjoint(){
    let mut c=Code::new(START);emit(&mut c).unwrap();assert_eq!(c.at(),END);
    for (i,word) in INTERPOLATION_CONSTANTS.iter().enumerate(){
      let offset=0x37168+i*4-START;
      assert_eq!(u32::from_le_bytes(c.bytes[offset..offset+4].try_into().unwrap()),*word);
    }
 }
}
