// SPDX-License-Identifier: GPL-3.0-only
//! A32 codec helper routines for buffer exponent, logarithmic bit width,
//! packed metadata accounting, aligned allocation and queue initialization.
//! A32 operations are written as named instruction encodings. Device string
//! pointers remain separate, non-executable literal data words.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x3548c;
const END:usize=0x35670;
const LOG_SCALE_MAGIC:u32=0x431b_de83; // [0x3553c,0x35540)
const BUFFER_LOG_DIAGNOSTIC:u32=0x000e_c0b3; // reused in both buffer helpers

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3548c: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x35490: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x35494: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x35498: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x3549c: cmp r2, r1, lsr r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsr,R::R0,false)?)?;
    // 0x354a0: bne 0x35498 <entry+0xc>     @ imm = #-0x10
    c.b(C::Ne,0x35498)?;
    // 0x354a4: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x354a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x354ac: beq 0x354d0 <entry+0x44>    @ imm = #0x1c
    c.b(C::Eq,0x354d0)?;
    // 0x354b0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x354b4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x354b8: lsr r3, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R0,Shift::Lsr,1)?)?;
    // 0x354bc: cmp r2, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsr,1,false)?)?;
    // 0x354c0: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x354c4: mov r0, r3
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
    // 0x354c8: bne 0x354b8 <entry+0x2c>    @ imm = #-0x18
    c.b(C::Ne,0x354b8)?;
    // 0x354cc: b 0x354d4 <entry+0x48>    @ imm = #0x0
    c.b(C::Al,0x354d4)?;
    // 0x354d0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x354d4: and r0, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R1,255)?)?;
    // 0x354d8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x354dc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x354e0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x354e4: blo 0x35504 <entry+0x78>    @ imm = #0x18
    c.b(C::Cc,0x35504)?;
    // 0x354e8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x354ec: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x354f0: lsr r3, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsr,1)?)?;
    // 0x354f4: cmp r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // 0x354f8: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x354fc: mov r2, r3
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // 0x35500: bhi 0x354f0 <entry+0x64>    @ imm = #-0x18
    c.b(C::Hi,0x354f0)?;
    // 0x35504: and r2, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,255)?)?;
    // 0x35508: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x3550c: lsl r2, r3, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // 0x35510: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x35514: addlo r1, r1, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Add,R::R1,R::R1,1)?)?;
    // 0x35518: and r0, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R1,255)?)?;
    // 0x3551c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x35520: mov r1, #288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,288)?)?;
    // 0x35524: orr r1, r1, #499712
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,499712)?)?;
    // 0x35528: mla r3, r2, r0, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R2,R::R0,R::R1)?)?;
    // 0x3552c: ldr r0, [pc, #0x8]          @ 0xb0 <entry+0xb0>
    c.literal_cond(C::Al,R::R0,0x3553c)?;
    // 0x35530: umull r1, r2, r3, r0
    c.emit(super::encode_long_multiply(C::Al,false,R::R1,R::R2,R::new(3)?,R::R0)?)?;
    // 0x35534: lsr r0, r2, #18
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsr,18)?)?;
    // 0x35538: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x3553c)?;
    c.emit(LOG_SCALE_MAGIC)?;
    c.check(0x35540)?;
    // 0x35540: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x35544: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35548: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x3554c: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit() | R::LR.bit())?)?;
    // 0x35550: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x35554: lsr r3, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R0,Shift::Lsr,16)?)?;
    // 0x35558: adds r0, r1, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,16,true)?)?;
    // 0x3555c: adc r12, r3, #0
    c.emit(super::encode_adc_zero(C::Al,R::R12,R::new(3)?)?)?;
    // 0x35560: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x35564: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x35568: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x3556c: lsl r2, r1, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R1,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x35570: eor r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Eor,R::new(3)?,R::new(3)?,31)?)?;
    // 0x35574: lsr r1, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,1)?)?;
    // 0x35578: lsr r3, r1, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(3)?,R::R0,R::R1,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x3557c: mov r1, r12
    c.emit(arm32::mov_register(C::Al,R::R1,R::R12))?;
    // 0x35580: bl 0x8187c <entry+0x4c3f0> @ imm = #0x4c2f4
    c.bl(0x8187c)?;
    // 0x35584: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x35588: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x3558c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x35590: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x35594: ldr r2, [pc, #0x3c]         @ 0x14c <entry+0x14c>
    c.literal_cond(C::Al,R::R2,0x355d8)?;
    // 0x35598: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x3559c: mov r1, #125
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,125)?)?;
    // 0x355a0: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // 0x355a4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x355a8: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x355ac: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x33e04
    c.bl(0x17b0)?;
    // 0x355b0: ldm r4, {r0, r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(4)?,R::R0.bit() | R::R1.bit() | R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x355b4: ldr r3, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // 0x355b8: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x355bc: rsb r4, r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(4)?,R::R2,0)?)?;
    // 0x355c0: add r3, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x355c4: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x355c8: and r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x355cc: mla r2, r0, r1, r3
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R0,R::R1,R::new(3)?)?)?;
    // 0x355d0: str r2, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,0)?)?;
    // 0x355d4: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x355d8)?;
    c.emit(BUFFER_LOG_DIAGNOSTIC)?;
    c.check(0x355dc)?;
    // 0x355dc: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x355e0: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x355e4: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x355e8: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x355ec: ldr r2, [pc, #0x54]         @ 0x1bc <entry+0x1bc>
    c.literal_cond(C::Al,R::R2,0x35648)?;
    // 0x355f0: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x355f4: mov r1, #142
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,142)?)?;
    // 0x355f8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x355fc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35600: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x35604: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x33e5c
    c.bl(0x17b0)?;
    // 0x35608: ldm r4, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(4)?,R::R2.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3560c: sub r0, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(6)?,1)?)?;
    // 0x35610: rsb r6, r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(6)?,R::new(6)?,0)?)?;
    // 0x35614: add r1, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x35618: and r5, r1, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3561c: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x35620: and r1, r0, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x35624: sub r0, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x35628: add r6, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3562c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35630: cmp r3, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x35634: addhs r0, r2, r6
    c.emit(arm32::data_register(C::Cs,Op::Add,R::R0,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x35638: subhs r2, r3, r6
    c.emit(arm32::data_register(C::Cs,Op::Sub,R::R2,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3563c: stmhs r4, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Cs,false,R::new(4)?,R::R0.bit() | R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x35640: movhs r0, r1
    c.emit(arm32::mov_register(C::Cs,R::R0,R::R1))?;
    // 0x35644: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x35648)?;
    c.emit(BUFFER_LOG_DIAGNOSTIC)?;
    c.check(0x3564c)?;
    // 0x3564c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x35650: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x35654: str r2, [r0, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,384)?)?;
    // 0x35658: str r2, [r0, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,388)?)?;
    // 0x3565c: str r2, [r0, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,392)?)?;
    // 0x35660: str r2, [r0, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,396)?)?;
    // 0x35664: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x35668: add r0, r0, #396
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,396)?)?;
    // 0x3566c: b 0x2b8c <entry+0xfffffffffffcd700> @ imm = #-0x32ae8
    c.b(C::Al,0x2b8c)?;
    c.check(END)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exponent_helpers_have_exact_data_pool_boundaries() {
        let mut c = Code::new(START);
        emit(&mut c).unwrap();
        assert_eq!(c.at(), END);
        let read = |at: usize| {
            let i=at-START;
            u32::from_le_bytes(c.bytes[i..i+4].try_into().unwrap())
        };
        assert_eq!(read(0x3553c),LOG_SCALE_MAGIC);
        assert_eq!(read(0x355d8),BUFFER_LOG_DIAGNOSTIC);
        assert_eq!(read(0x35648),BUFFER_LOG_DIAGNOSTIC);
    }
}
