// SPDX-License-Identifier: GPL-3.0-only
//! A32 MMIO controller-state getters and per-channel reference counters.
//! These short helpers alternate executable instructions with literal MMIO
//! addresses, so their instruction/data classification is explicit.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2f7d0;
const END:usize=0x2f96c;
fn report_mmio_state_and_counters(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2f7d0: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2f7d4: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2f7d8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2f7dc: bl 0x2b18 <entry+0xfffffffffffd3518> @ imm = #-0x2cccc
    c.bl(0x2b18)?;
    // 0x2f7e0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2f7e4: beq 0x2f7f8 <entry+0x1f8>   @ imm = #0xc
    c.b(C::Eq,0x2f7f8)?;
    // 0x2f7e8: ldr r2, [pc, #0x14]         @ 0x204 <entry+0x204>
    c.literal(R::R2,0x2f804)?;
    // 0x2f7ec: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f7f0: mov r1, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,68)?)?;
    // 0x2f7f4: bl 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x2e04c
    c.bl(0x17b0)?;
    // 0x2f7f8: add r0, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    // 0x2f7fc: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2f800: b 0x2ce0 <entry+0xfffffffffffd36e0> @ imm = #-0x2cb28
    c.b(C::Al,0x2ce0)?;
    c.check(0x2f804)?;
    const MMIO_POLL_WORDS_2F804:[u32;1]=[0x000eaec8];
    for word in MMIO_POLL_WORDS_2F804 {c.emit(word)?;}
    c.check(0x2f808)?;
    // 0x2f808: ldr r0, [pc, #0x18]         @ 0x228 <entry+0x228>
    c.literal(R::R0,0x2f828)?;
    // 0x2f80c: mov r2, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1048576)?)?;
    // 0x2f810: orr r2, r2, #1342177280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1342177280)?)?;
    // 0x2f814: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x2f818: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f81c: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x2f820: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f824: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2f828)?;
    const MMIO_POLL_WORDS_2F828:[u32;1]=[0x0010e07c];
    for word in MMIO_POLL_WORDS_2F828 {c.emit(word)?;}
    c.check(0x2f82c)?;
    // 0x2f82c: ldr r0, [pc, #0x30]         @ 0x264 <entry+0x264>
    c.literal(R::R0,0x2f864)?;
    // 0x2f830: mov r1, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1048576)?)?;
    // 0x2f834: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2f838: orr r1, r1, #1342177280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1342177280)?)?;
    // 0x2f83c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2f840: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f844: ldr r1, [pc, #0x1c]         @ 0x268 <entry+0x268>
    c.literal(R::R1,0x2f868)?;
    // 0x2f848: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f84c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f850: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x2f854: cmp r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
    // 0x2f858: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2f85c: and r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x2f860: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2f864)?;
    const MMIO_POLL_WORDS_2F864:[u32;2]=[0x0010e07c,0x0010e078];
    for word in MMIO_POLL_WORDS_2F864 {c.emit(word)?;}
    c.check(0x2f86c)?;
    // 0x2f86c: ldr r0, [pc, #0x30]         @ 0x2a4 <entry+0x2a4>
    c.literal(R::R0,0x2f8a4)?;
    // 0x2f870: mov r1, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1048576)?)?;
    // 0x2f874: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2f878: orr r1, r1, #1342177280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1342177280)?)?;
    // 0x2f87c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2f880: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f884: ldr r1, [pc, #0x1c]         @ 0x2a8 <entry+0x2a8>
    c.literal(R::R1,0x2f8a8)?;
    // 0x2f888: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f88c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f890: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x2f894: cmp r1, #47
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,47)?)?;
    // 0x2f898: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2f89c: and r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x2f8a0: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2f8a4)?;
    const MMIO_POLL_WORDS_2F8A4:[u32;2]=[0x0010e07c,0x0010e078];
    for word in MMIO_POLL_WORDS_2F8A4 {c.emit(word)?;}
    c.check(0x2f8ac)?;
    // 0x2f8ac: ldr r0, [pc, #0x18]         @ 0x2cc <entry+0x2cc>
    c.literal(R::R0,0x2f8cc)?;
    // 0x2f8b0: mov r2, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1114112)?)?;
    // 0x2f8b4: orr r2, r2, #1342177280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1342177280)?)?;
    // 0x2f8b8: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x2f8bc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f8c0: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x2f8c4: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f8c8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2f8cc)?;
    const MMIO_POLL_WORDS_2F8CC:[u32;1]=[0x0010e07c];
    for word in MMIO_POLL_WORDS_2F8CC {c.emit(word)?;}
    c.check(0x2f8d0)?;
    // 0x2f8d0: ldr r0, [pc, #0x38]         @ 0x310 <entry+0x310>
    c.literal(R::R0,0x2f910)?;
    // 0x2f8d4: mov r1, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10)?)?;
    // 0x2f8d8: mov r3, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1114112)?)?;
    // 0x2f8dc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2f8e0: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x2f8e4: orr r3, r3, #1342177280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1342177280)?)?;
    // 0x2f8e8: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2f8ec: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2f8f0: ldr r1, [pc, #0x1c]         @ 0x314 <entry+0x314>
    c.literal(R::R1,0x2f914)?;
    // 0x2f8f4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f8f8: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2f8fc: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x2f900: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x2f904: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2f908: and r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2f90c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2f910)?;
    const MMIO_POLL_WORDS_2F910:[u32;2]=[0x0010e078,0x0010e07c];
    for word in MMIO_POLL_WORDS_2F910 {c.emit(word)?;}
    c.check(0x2f918)?;
    // 0x2f918: ldr r0, [pc, #0x4]          @ 0x324 <entry+0x324>
    c.literal(R::R0,0x2f924)?;
    // 0x2f91c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2f920: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2f924)?;
    const MMIO_POLL_WORDS_2F924:[u32;1]=[0x0010e07c];
    for word in MMIO_POLL_WORDS_2F924 {c.emit(word)?;}
    c.check(0x2f928)?;
    // 0x2f928: ldr r1, [pc, #0xc]          @ 0x33c <entry+0x33c>
    c.literal(R::R1,0x2f93c)?;
    // 0x2f92c: ldr r2, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x2f930: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x2f934: str r2, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x2f938: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2f93c)?;
    const MMIO_POLL_WORDS_2F93C:[u32;1]=[0x00104118];
    for word in MMIO_POLL_WORDS_2F93C {c.emit(word)?;}
    c.check(0x2f940)?;
    // 0x2f940: ldr r1, [pc, #0x10]         @ 0x358 <entry+0x358>
    c.literal(R::R1,0x2f958)?;
    // 0x2f944: ldr r2, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x2f948: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x2f94c: subne r2, r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::R2,R::R2,1)?)?;
    // 0x2f950: strne r2, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Ne,false,R::R2,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x2f954: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2f958)?;
    const MMIO_POLL_WORDS_2F958:[u32;1]=[0x00104118];
    for word in MMIO_POLL_WORDS_2F958 {c.emit(word)?;}
    c.check(0x2f95c)?;
    // 0x2f95c: ldr r1, [pc, #0x4]          @ 0x368 <entry+0x368>
    c.literal(R::R1,0x2f968)?;
    // 0x2f960: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x2f964: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2f968)?;
    const MMIO_POLL_WORDS_2F968:[u32;1]=[0x00104118];
    for word in MMIO_POLL_WORDS_2F968 {c.emit(word)?;}
    c.check(0x2f96c)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;report_mmio_state_and_counters(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn mmio_pool_offsets_are_preserved(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let data=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(data(0x2f804),0x000eaec8);
        assert_eq!(data(0x2f828),0x0010e07c);
        assert_eq!(data(0x2f93c),0x00104118);
        assert_eq!(data(0x2f968),0x00104118);
    }
}
