// SPDX-License-Identifier: GPL-3.0-only
//! Control-message publication helper with correctly paired LDRD/STRD and
//! carry-propagating 64-bit counter increment. Diagnostic and controller
//! addresses after the original return are literal data rather than opcodes.
use arm32::{Condition as C,DataOp as Op,Register as R,BlockMode};
use super::Code;
const START:usize=0x2ebbc;
const END:usize=0x2ec94;
fn publish_controller_event(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2ebbc: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2ebc0: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x2ebc4: sub sp, sp, #288
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,288)?)?;
    // 0x2ebc8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2ebcc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ebd0: add r5, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R1,1)?)?;
    // 0x2ebd4: add r1, r1, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,28)?)?;
    // 0x2ebd8: orr r0, r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,131072)?)?;
    // 0x2ebdc: cmp r1, #284
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,284)?)?;
    // 0x2ebe0: stmib sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x2ebe4: ldr r2, [pc, #0x98]         @ 0x1684 <entry+0x1684>
    c.literal(R::R2,0x2ec84)?;
    // 0x2ebe8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2ebec: str r5, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    // 0x2ebf0: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x2ebf4: mov r1, #163
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,163)?)?;
    // 0x2ebf8: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2ebfc: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2d454
    c.bl(0x17b0)?;
    // 0x2ec00: mov r6, sp
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::SP))?;
    // 0x2ec04: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x2ec08: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x2ec0c: add r0, r6, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,24)?)?;
    // 0x2ec10: bl 0x471c <entry+0xfffffffffffd711c> @ imm = #-0x2a4fc
    c.bl(0x471c)?;
    // 0x2ec14: ldr r4, [pc, #0x6c]         @ 0x1688 <entry+0x1688>
    c.literal(R::new(4)?,0x2ec88)?;
    // 0x2ec18: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2ec1c: bl 0x2bf4 <entry+0xfffffffffffd55f4> @ imm = #-0x2c030
    c.bl(0x2bf4)?;
    // 0x2ec20: ldr r0, [pc, #0x64]         @ 0x168c <entry+0x168c>
    c.literal(R::R0,0x2ec8c)?;
    // 0x2ec24: ldr r5, [pc, #0x64]         @ 0x1690 <entry+0x1690>
    c.literal(R::new(5)?,0x2ec90)?;
    // 0x2ec28: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2ec2c: ldrd r2, r3, [r0]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x2ec30: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x2ec34: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x2ec38: adds r2, r2, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::R2,R::R2,1,true)?)?;
    // 0x2ec3c: adc r3, r3, #0
    c.emit(super::encode_adc_zero(C::Al,R::new(3)?,R::new(3)?)?)?;
    // 0x2ec40: strd r2, r3, [r0]
    c.emit(arm32::transfer_doubleword(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x2ec44: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2ec48: sub r2, r11, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,20)?)?;
    // 0x2ec4c: bl 0x2f1e4 <entry+0x1be4>  @ imm = #0x590
    c.bl(0x2f1e4)?;
    // 0x2ec50: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ec54: bne 0x2ec60 <entry+0x1660>  @ imm = #0x4
    c.b(C::Ne,0x2ec60)?;
    // 0x2ec58: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2ec5c: bl 0x2f394 <entry+0x1d94>  @ imm = #0x730
    c.bl(0x2f394)?;
    // 0x2ec60: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2ec64: bl 0x2c34 <entry+0xfffffffffffd5634> @ imm = #-0x2c038
    c.bl(0x2c34)?;
    // 0x2ec68: ldr r0, [r11, #-0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-20)?)?;
    // 0x2ec6c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2ec70: bne 0x2ec78 <entry+0x1678>  @ imm = #0x0
    c.b(C::Ne,0x2ec78)?;
    // 0x2ec74: bl 0x2a158 <entry+0xffffffffffffcb58> @ imm = #-0x4b24
    c.bl(0x2a158)?;
    // 0x2ec78: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2ec7c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x2ec80: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2ec84)?;
    const EVENT_SOURCE_DATA_2EC84:[u32;4]=[0x000eac94,0x0010410c,0x0010e070,0x00104110];
    for word in EVENT_SOURCE_DATA_2EC84 {c.emit(word)?;}
    c.check(0x2ec94)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;publish_controller_event(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_events_preserve_literal_address_pool(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let data=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(data(0x2ec84),0x000eac94);
        assert_eq!(data(0x2ec88),0x0010410c);
        assert_eq!(data(0x2ec90),0x00104110);
    }
}
