// SPDX-License-Identifier: GPL-3.0-only
//! First verified slice of the long Venus controller format-parameter setup
//! routine. The function begins at 0x43ff0, and its branches continue past
//! 0x45000; this module does not claim to provide its complete return.
//! All words are generated from named ARM A32 operands, condition codes,
//! signed-byte loads and block-transfer addressing modes; no OEM instruction
//! data or unknown bytes are embedded in the generator.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
pub const START:usize=0x43ff0;
pub const END:usize=0x45000;
struct Code {at:usize, bytes:Vec<u8>}
impl Code {
    fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
    fn check(&self,a:usize)->Result<(),String>{
        if self.at!=a{return Err(format!("codec format parameters 0x{:x}, expected 0x{a:x}",self.at));} Ok(())
    }
    #[inline(never)]
    fn emit(&mut self,w:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at) || self.at&3!=0{return Err(format!("Agent 04 invalid 0x{:x}",self.at));}
        self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
    }
    #[inline(never)]
    fn branch(&mut self,c:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(c,self.at as u32,target as u32,false)?)
    }
}

/// Initialize control format state, registers, and source descriptors.
#[inline(never)]
fn initialize_controller_format(c:&mut Code)->Result<(),String>{
    c.check(0x43ff0)?;
    // 0x43ff0: push	{r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x43ff4: add	r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x43ff8: sub	sp, sp, #268
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,268)?)?;
    // 0x43ffc: sub	sp, sp, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,1024)?)?;
    // 0x44000: mov	r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // 0x44004: mov	r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x44008: ldr	r1, [pc, #0xeb0]        @ 0x1370 <entry+0x1370>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3760)?)?;
    // 0x4400c: mov	r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x44010: ldr	r8, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(10)?,44)?)?;
    // 0x44014: bl	0x2ec94 <entry+0xfffffffffffeb144> @ imm = #-0x15388
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x44018: ldr	r1, [pc, #0xea4]        @ 0x1374 <entry+0x1374>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3748)?)?;
    // 0x4401c: mov	r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x44020: bl	0x2ec94 <entry+0xfffffffffffeb144> @ imm = #-0x15394
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x44024: mov	r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x44028: mov	r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x4402c: bl	0x3f834 <entry+0xffffffffffffbce4> @ imm = #-0x4800
    c.emit(arm32::branch(C::Al,c.at as u32,0x3f834,true)?)?;
    // 0x44030: str	r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x44034: mov	lr, r4
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(4)?))?;
    // 0x44038: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x4403c: ldrb	r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x44040: str	lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,104)?)?;
    // 0x44044: str	r10, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,88)?)?;
    // 0x44048: str	r8, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,80)?)?;
    // 0x4404c: cmp	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x44050: bne	0x46568 <entry+0x2a18>  @ imm = #0x2510
    c.branch(C::Ne,0x46568)?;
    // 0x44054: ldr	r0, [r10, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2652)?)?;
    // 0x44058: mov	r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x4405c: ldr	r1, [r10, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2656)?)?;
    // 0x44060: mov	r3, lr
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::LR))?;
    // 0x44064: mov	r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x44068: mov	r12, #154
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,154)?)?;
    // 0x4406c: mov	r5, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,268435456)?)?;
    // 0x44070: orr	r2, r2, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1792)?)?;
    // 0x44074: orr	r12, r12, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,1024)?)?;
    // 0x44078: and	r0, r2, r0, lsr #3
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsr,3,false)?)?;
    // 0x4407c: mov	r2, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16711680)?)?;
    // 0x44080: orr	r2, r2, #117440512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,117440512)?)?;
    // 0x44084: and	r1, r2, r1, lsl #13
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,13,false)?)?;
    // 0x44088: orr	r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4408c: str	r1, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,300)?)?;
    // 0x44090: ldr	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x44094: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x44098: add	r8, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R0,8)?)?;
    // 0x4409c: ldreq	r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x440a0: andeq	r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x440a4: streq	r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x440a8: mov	r2, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,9216)?)?;
    // 0x440ac: str	r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x440b0: str	r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x440b4: ldr	lr, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-36)?)?;
    // 0x440b8: add	r2, lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::LR,1)?)?;
    // 0x440bc: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x440c0: str	r8, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(3)?,0)?)?;
    // 0x440c4: str	r6, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,348)?)?;
    // 0x440c8: ldr	r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x440cc: ldrsb	r9, [r7, r12]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::new(9)?,R::new(7)?,R::R12)?)?;
    // 0x440d0: cmp	r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,1)?)?;
    // 0x440d4: movge	r6, #1073741824
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::new(6)?,R::R0,1073741824)?)?;
    // 0x440d8: strge	r6, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Ge,false,R::new(6)?,R::SP,348)?)?;
    // 0x440dc: cmp	r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,1)?)?;
    // 0x440e0: ldr	r2, [r7, #0x250]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,592)?)?;
    // 0x440e4: ldr	r4, [r7, #0x24c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,588)?)?;
    // 0x440e8: ldr	r1, [r7, #0x254]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,596)?)?;
    // 0x440ec: ldr	r3, [r7, #0x258]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,600)?)?;
    // 0x440f0: and	r2, r5, r2, lsl #28
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(5)?,R::R2,Shift::Lsl,28,false)?)?;
    // 0x440f4: mov	r5, #536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,536870912)?)?;
    // 0x440f8: and	r4, r5, r4, lsl #29
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(5)?,R::new(4)?,Shift::Lsl,29,false)?)?;
    // 0x440fc: orr	r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x44100: orr	r2, r2, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x44104: mov	r4, #134217728
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,134217728)?)?;
    // 0x44108: and	r1, r4, r1, lsl #27
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::R1,Shift::Lsl,27,false)?)?;
    // 0x4410c: orr	r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x44110: mov	r2, #67108864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,67108864)?)?;
    // 0x44114: and	r2, r2, r3, lsl #26
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,26,false)?)?;
    // 0x44118: mov	r3, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,33554432)?)?;
    // 0x4411c: orr	r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x44120: ldr	r2, [r7, #0x25c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,604)?)?;
    // 0x44124: and	r2, r3, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,25,false)?)?;
    // 0x44128: orr	r6, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4412c: str	r6, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,348)?)?;
    // 0x44130: blt	0x4414c <entry+0x5fc>   @ imm = #0x14
    c.branch(C::Lt,0x4414c)?;
    // 0x44134: ldrb	r1, [r10, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,2462)?)?;
    // 0x44138: cmp	r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,32)?)?;
    // 0x4413c: orreq	r6, r6, #805306368
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(6)?,R::new(6)?,805306368)?)?;
    // 0x44140: streq	r6, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::SP,348)?)?;
    // 0x44144: orr	r6, r6, #234881024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,234881024)?)?;
    // 0x44148: str	r6, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,348)?)?;
    // 0x4414c: ldr	r5, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,644)?)?;
    // 0x44150: cmp	r5, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8192)?)?;
    // 0x44154: bne	0x4418c <entry+0x63c>   @ imm = #0x30
    c.branch(C::Ne,0x4418c)?;
    // 0x44158: add	r3, r7, #616
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(7)?,616)?)?;
    // 0x4415c: mov	r4, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,31744)?)?;
    // 0x44160: ldm	r3, {r1, r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(3)?,R::R1.bit() | R::R2.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x44164: and	r2, r4, r2, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(4)?,R::R2,Shift::Lsl,10,false)?)?;
    // 0x44168: ldr	r4, [pc, #0xf2c]        @ 0x154c <entry+0x154c>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,3884)?)?;
    // 0x4416c: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x44170: and	r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x44174: orr	r3, r4, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x44178: orr	r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4417c: mov	r3, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32505856)?)?;
    // 0x44180: and	r1, r3, r1, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,20,false)?)?;
    // 0x44184: orr	r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x44188: str	r1, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,348)?)?;
    // 0x4418c: ldr	r1, [r7, #0x274]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,628)?)?;
    // 0x44190: mov	r2, #1040187392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1040187392)?)?;
    // 0x44194: cmp	r5, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8192)?)?;
    // 0x44198: and	r6, r2, r1, lsl #25
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R2,R::R1,Shift::Lsl,25,false)?)?;
    // 0x4419c: str	r6, [sp, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,352)?)?;
    // 0x441a0: bne	0x441ec <entry+0x69c>   @ imm = #0x44
    c.branch(C::Ne,0x441ec)?;
    // 0x441a4: add	r4, r7, #632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(7)?,632)?)?;
    // 0x441a8: mov	r5, #1015808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1015808)?)?;
    // 0x441ac: ldm	r4, {r1, r2, r3, r4}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(4)?,R::R1.bit() | R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x441b0: and	r2, r5, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(5)?,R::R2,Shift::Lsl,15,false)?)?;
    // 0x441b4: mov	r5, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,32505856)?)?;
    // 0x441b8: and	r1, r5, r1, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(5)?,R::R1,Shift::Lsl,20,false)?)?;
    // 0x441bc: orr	r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x441c0: orr	r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x441c4: mov	r2, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,31744)?)?;
    // 0x441c8: and	r2, r2, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x441cc: orr	r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x441d0: mov	r2, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,992)?)?;
    // 0x441d4: and	r2, r2, r4, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(4)?,Shift::Lsl,5,false)?)?;
    // 0x441d8: orr	r6, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x441dc: str	r6, [sp, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,352)?)?;
    // 0x441e0: ldr	r1, [r7, #0x288]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,648)?)?;
    // 0x441e4: and	r7, r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R1,31)?)?;
    // 0x441e8: b	0x4420c <entry+0x6bc>   @ imm = #0x1c
    c.branch(C::Al,0x4420c)?;
    // 0x441ec: ldr	r1, [r7, #0x27c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,636)?)?;
    // 0x441f0: mov	r2, #1015808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1015808)?)?;
    // 0x441f4: and	r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // 0x441f8: mov	r2, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,992)?)?;
    // 0x441fc: orr	r6, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x44200: str	r6, [sp, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,352)?)?;
    // 0x44204: ldr	r1, [r7, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,644)?)?;
    // 0x44208: and	r7, r2, r1, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R2,R::R1,Shift::Lsl,5,false)?)?;
    // 0x4420c: orr	r7, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x44210: tst	r8, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,31)?)?;
    // 0x44214: mov	r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x44218: mov	r9, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,6)?)?;
    // 0x4421c: str	r7, [sp, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,352)?)?;
    // 0x44220: ldreq	r1, [r8]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(8)?,0)?)?;
    // 0x44224: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x44228: streq	r1, [r8]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(8)?,0)?)?;
    // 0x4422c: mov	r1, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,52)?)?;
    // 0x44230: add	r8, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R0,16)?)?;
    // 0x44234: orr	r1, r1, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,9216)?)?;
    // 0x44238: str	r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x4423c: str	r7, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,12)?)?;
    // 0x44240: add	r1, lr, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::LR,2)?)?;
    // 0x44244: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x44248: ldr	r1, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,104)?)?;
    // 0x4424c: str	r8, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,0)?)?;
    // 0x44250: ldr	r1, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,80)?)?;
    // 0x44254: ldr	r4, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,644)?)?;
    // 0x44258: str	r6, [sp, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,304)?)?;
    // 0x4425c: ldr	r5, [r1, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,400)?)?;
    // 0x44260: cmp	r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,4096)?)?;
    // 0x44264: moveq	r9, #8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(9)?,R::R0,8)?)?;
    // 0x44268: cmp	r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // 0x4426c: beq	0x4427c <entry+0x72c>   @ imm = #0x8
    c.branch(C::Eq,0x4427c)?;
    // 0x44270: cmp	r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // 0x44274: moveq	r6, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // 0x44278: b	0x4428c <entry+0x73c>   @ imm = #0xc
    c.branch(C::Al,0x4428c)?;
    // 0x4427c: ldr	r1, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,80)?)?;
    // 0x44280: ldrb	r6, [r1, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R1,1005)?)?;
    // 0x44284: cmp	r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // 0x44288: movne	r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x4428c: ldrb	r1, [r10, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,2462)?)?;
    // 0x44290: ldr	r3, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,80)?)?;
    // 0x44294: mov	r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x44298: cmp	r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,32)?)?;
    // 0x4429c: mov	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x442a0: ldr	r7, [r3, #0x36c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,876)?)?;
    // 0x442a4: moveq	r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x442a8: cmp	r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,4096)?)?;
    // 0x442ac: mov	r4, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,992)?)?;
    // 0x442b0: orr	r6, r6, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::R1,Shift::Lsl,2,false)?)?;
    // 0x442b4: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x442b8: orreq	r6, r6, #16
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(6)?,R::new(6)?,16)?)?;
    // 0x442bc: ldr	r2, [r1, #0x248]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,584)?)?;
    // 0x442c0: and	r2, r4, r2, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(4)?,R::R2,Shift::Lsl,5,false)?)?;
    // 0x442c4: ldr	r4, [r3, #0x368]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(3)?,872)?)?;
    // 0x442c8: mov	r3, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,31744)?)?;
    // 0x442cc: orr	r2, r2, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x442d0: and	r3, r3, r4, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,10,false)?)?;
    // 0x442d4: ldr	r4, [r1, #0x4ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,1260)?)?;
    // 0x442d8: orr	r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x442dc: mov	r3, #491520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,491520)?)?;
    // 0x442e0: and	r3, r3, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,15,false)?)?;
    // 0x442e4: mov	r7, #3670016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3670016)?)?;
    // 0x442e8: orr	r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x442ec: ldr	r3, [r1, #0x4e8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,1256)?)?;
    // 0x442f0: and	r3, r7, r3, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,19,false)?)?;
    // 0x442f4: orr	r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x442f8: mov	r3, #130023424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,130023424)?)?;
    // 0x442fc: and	r3, r3, r4, lsl #22
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,22,false)?)?;
    c.check(0x44300)
}

/// Compute per-plane sizes, feature modes, and byte-scaled controller fields.
#[inline(never)]
fn derive_stream_metrics(c:&mut Code)->Result<(),String>{
    c.check(0x44300)?;
    // 0x44300: mov	r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,8)?)?;
    // 0x44304: orr	r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x44308: ldrb	r3, [r1, #0x4f0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,1264)?)?;
    // 0x4430c: ldrsb	r1, [r1, r12]
    c.emit(super::venus_controller_format_registers::signed_byte_register(C::Al,R::R1,R::R1,R::R12)?)?;
    // 0x44310: mov	r12, #16646144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16646144)?)?;
    // 0x44314: orr	r12, r12, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4278190080)?)?;
    // 0x44318: and	r3, r4, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(4)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x4431c: add	r4, r9, #254
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(9)?,254)?)?;
    // 0x44320: orr	r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x44324: ldrb	r3, [r10, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,988)?)?;
    // 0x44328: add	r4, r4, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,65280)?)?;
    // 0x4432c: and	r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x44330: orr	r2, r2, r3, lsl #28
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,28,false)?)?;
    // 0x44334: and	r3, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R1,255)?)?;
    // 0x44338: cmp	r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,3)?)?;
    // 0x4433c: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x44340: moveq	r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x44344: cmp	r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x44348: movgt	r5, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x4434c: orr	r2, r2, r3, lsl #30
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,30,false)?)?;
    // 0x44350: tst	r8, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,31)?)?;
    // 0x44354: orr	r2, r2, r5, lsl #31
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(5)?,Shift::Lsl,31,false)?)?;
    // 0x44358: str	r2, [sp, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,304)?)?;
    // 0x4435c: ldreq	r1, [r8]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(8)?,0)?)?;
    // 0x44360: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x44364: streq	r1, [r8]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(8)?,0)?)?;
    // 0x44368: mov	r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x4436c: orr	r1, r1, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,9216)?)?;
    // 0x44370: str	r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x44374: str	r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,20)?)?;
    // 0x44378: add	r1, lr, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::LR,3)?)?;
    // 0x4437c: add	r2, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,24)?)?;
    // 0x44380: ldr	r8, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
    // 0x44384: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x44388: ands	r1, r6, #2
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::new(6)?,2,true)?)?;
    // 0x4438c: str	r2, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x44390: ldr	r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x44394: add	r7, r3, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(3)?,24)?)?;
    // 0x44398: addeq	r7, r3, #32
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(7)?,R::new(3)?,32)?)?;
    // 0x4439c: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x443a0: ldr	r6, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,0)?)?;
    // 0x443a4: sub	r5, r6, r9
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(5)?,R::new(6)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x443a8: sub	r7, r4, r6
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x443ac: mov	r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,255)?)?;
    // 0x443b0: orr	r6, r6, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,65280)?)?;
    // 0x443b4: add	r5, r12, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R12,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x443b8: and	r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x443bc: orr	r5, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x443c0: add	r7, r3, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(3)?,28)?)?;
    // 0x443c4: addeq	r7, r3, #36
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(7)?,R::new(3)?,36)?)?;
    // 0x443c8: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x443cc: ldr	r1, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,0)?)?;
    // 0x443d0: str	r5, [sp, #0x168]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,360)?)?;
    // 0x443d4: sub	r4, r4, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x443d8: sub	r3, r1, r9
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x443dc: ldreq	r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x443e0: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x443e4: streq	r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x443e8: mov	r1, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,60)?)?;
    // 0x443ec: add	r2, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,32)?)?;
    // 0x443f0: orr	r1, r1, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,9216)?)?;
    // 0x443f4: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x443f8: str	r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x443fc: add	r1, lr, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::LR,4)?)?;
    // 0x44400: str	r5, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,28)?)?;
    // 0x44404: mov	r5, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16711680)?)?;
    // 0x44408: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4440c: add	r1, r12, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R12,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x44410: and	r3, r4, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x44414: str	r2, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x44418: mov	r6, #240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,240)?)?;
    // 0x4441c: orr	r3, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x44420: str	r3, [sp, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,364)?)?;
    // 0x44424: ldreq	r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x44428: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4442c: streq	r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x44430: mov	r1, #9280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,9280)?)?;
    // 0x44434: mov	r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x44438: str	r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // 0x4443c: add	r1, lr, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::LR,5)?)?;
    // 0x44440: str	r3, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,36)?)?;
    // 0x44444: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x44448: add	r1, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,40)?)?;
    // 0x4444c: str	r1, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x44450: str	r2, [sp, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,368)?)?;
    // 0x44454: ldr	r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x44458: ldr	r2, [r3, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,64)?)?;
    // 0x4445c: ldr	r7, [r3, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,72)?)?;
    // 0x44460: and	r7, r6, r7, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,4,false)?)?;
    // 0x44464: and	r2, r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,15)?)?;
    // 0x44468: orr	r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4446c: str	r2, [sp, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,368)?)?;
    // 0x44470: ldr	r7, [r3, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,68)?)?;
    // 0x44474: ldr	r6, [r3, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,76)?)?;
    // 0x44478: and	r7, r5, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(5)?,R::new(7)?,Shift::Lsl,16,false)?)?;
    // 0x4447c: orr	r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x44480: mov	r7, #2130706432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2130706432)?)?;
    // 0x44484: and	r7, r7, r6, lsl #24
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,24,false)?)?;
    // 0x44488: orr	r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4448c: str	r2, [sp, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,368)?)?;
    // 0x44490: ldr	r7, [r3, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,80)?)?;
    // 0x44494: ldr	r3, [r3, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,84)?)?;
    // 0x44498: orrs	r6, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,true)?)?;
    // 0x4449c: beq	0x444bc <entry+0x96c>   @ imm = #0x18
    c.branch(C::Eq,0x444bc)?;
    // 0x444a0: mov	r6, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3840)?)?;
    // 0x444a4: and	r7, r6, r7, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,8,false)?)?;
    // 0x444a8: orr	r2, r2, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x444ac: mov	r7, #61440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,61440)?)?;
    // 0x444b0: and	r3, r7, r3, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,12,false)?)?;
    // 0x444b4: str	r2, [sp, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,368)?)?;
    // 0x444b8: b	0x444c4 <entry+0x974>   @ imm = #0x4
    c.branch(C::Al,0x444c4)?;
    // 0x444bc: str	r2, [sp, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,368)?)?;
    // 0x444c0: mov	r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4096)?)?;
    // 0x444c4: bic	r2, r2, #61440
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,61440)?)?;
    // 0x444c8: tst	r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x444cc: mov	r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x444d0: orr	r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x444d4: str	r2, [sp, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,368)?)?;
    // 0x444d8: ldreq	r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x444dc: andeq	r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x444e0: streq	r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x444e4: mov	r1, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,68)?)?;
    // 0x444e8: orr	r1, r1, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,9216)?)?;
    // 0x444ec: str	r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,40)?)?;
    // 0x444f0: str	r2, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,44)?)?;
    // 0x444f4: add	r0, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,48)?)?;
    // 0x444f8: mov	r2, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,60)?)?;
    // 0x444fc: ldr	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x44500: add	r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x44504: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x44508: str	r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x4450c: add	r0, sp, #300
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,300)?)?;
    // 0x44510: mov	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x44514: add	r0, r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,72)?)?;
    // 0x44518: bl	0x31ec <entry+0xfffffffffffbf69c> @ imm = #-0x41334
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x4451c: ldrb	r0, [r10, #0x3d9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,985)?)?;
    // 0x44520: cmp	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x44524: beq	0x445d4 <entry+0xa84>   @ imm = #0xa8
    c.branch(C::Eq,0x445d4)?;
    // 0x44528: mov	r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x4452c: mov	r0, #772
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,772)?)?;
    // 0x44530: mov	r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x44534: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x44538: orr	r2, r2, #8257536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8257536)?)?;
    // 0x4453c: ldr	r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x44540: add	r6, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44544: ldrb	r4, [r6, #-0x5c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,-92)?)?;
    // 0x44548: cmp	r4, #22
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,22)?)?;
    // 0x4454c: bhi	0x4455c <entry+0xa0c>   @ imm = #0x8
    c.branch(C::Hi,0x4455c)?;
    // 0x44550: tst	r2, r1, lsl r4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::new(4)?,false)?)?;
    // 0x44554: strbne	r5, [r6, #-0x5c]
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(5)?,R::new(6)?,-92)?)?;
    // 0x44558: ldrne	r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x4455c: add	r6, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44560: ldrb	r4, [r6, #-0x45]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,-69)?)?;
    // 0x44564: cmp	r4, #22
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,22)?)?;
    // 0x44568: bhi	0x44578 <entry+0xa28>   @ imm = #0x8
    c.branch(C::Hi,0x44578)?;
    // 0x4456c: tst	r2, r1, lsl r4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::new(4)?,false)?)?;
    // 0x44570: strbne	r5, [r6, #-0x45]
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(5)?,R::new(6)?,-69)?)?;
    // 0x44574: ldrne	r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x44578: add	r6, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4457c: ldrb	r4, [r6, #-0x2e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,-46)?)?;
    // 0x44580: cmp	r4, #22
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,22)?)?;
    // 0x44584: bhi	0x44594 <entry+0xa44>   @ imm = #0x8
    c.branch(C::Hi,0x44594)?;
    // 0x44588: tst	r2, r1, lsl r4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::new(4)?,false)?)?;
    // 0x4458c: strbne	r5, [r6, #-0x2e]
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(5)?,R::new(6)?,-46)?)?;
    // 0x44590: ldrne	r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x44594: add	r6, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44598: ldrb	r4, [r6, #-0x17]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,-23)?)?;
    // 0x4459c: cmp	r4, #22
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,22)?)?;
    // 0x445a0: bhi	0x445b0 <entry+0xa60>   @ imm = #0x8
    c.branch(C::Hi,0x445b0)?;
    // 0x445a4: tst	r2, r1, lsl r4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::new(4)?,false)?)?;
    // 0x445a8: strbne	r5, [r6, #-0x17]
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(5)?,R::new(6)?,-23)?)?;
    // 0x445ac: ldrne	r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x445b0: ldrb	r6, [r7, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(6)?,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    // 0x445b4: cmp	r6, #22
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,22)?)?;
    // 0x445b8: bhi	0x445c4 <entry+0xa74>   @ imm = #0x4
    c.branch(C::Hi,0x445c4)?;
    // 0x445bc: tst	r2, r1, lsl r6
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::new(6)?,false)?)?;
    // 0x445c0: strbne	r5, [r7, r0]
    c.emit(arm32::transfer_byte_register(C::Ne,false,R::new(5)?,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    // 0x445c4: add	r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x445c8: add	r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x445cc: cmp	r3, #23
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,23)?)?;
    // 0x445d0: bne	0x4453c <entry+0x9ec>   @ imm = #-0x9c
    c.branch(C::Ne,0x4453c)?;
    // 0x445d4: mov	r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x445d8: mov	r1, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32505856)?)?;
    // 0x445dc: mov	lr, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,992)?)?;
    // 0x445e0: mov	r8, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,31744)?)?;
    // 0x445e4: mov	r9, #1015808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1015808)?)?;
    // 0x445e8: str	r12, [sp, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,432)?)?;
    // 0x445ec: ldr	r2, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2744)?)?;
    // 0x445f0: ldrb	r3, [r2, #0x2a8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,680)?)?;
    // 0x445f4: ldrb	r7, [r2, #0x2a9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R2,681)?)?;
    // 0x445f8: ldrb	r6, [r2, #0x2aa]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R2,682)?)?;
    // 0x445fc: ldrb	r5, [r2, #0x2ab]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R2,683)?)?;
    c.check(0x44600)
}

/// Populate capability fields for each input channel and device format.
#[inline(never)]
fn populate_controller_fields(c:&mut Code)->Result<(),String>{
    c.check(0x44600)?;
    // 0x44600: ldrb	r0, [r2, #0x2ac]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R2,684)?)?;
    // 0x44604: and	r7, r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,31)?)?;
    // 0x44608: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x4460c: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x44610: orr	r3, r3, r7, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,5,false)?)?;
    // 0x44614: and	r7, r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(6)?,31)?)?;
    // 0x44618: orr	r3, r3, r7, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,10,false)?)?;
    // 0x4461c: and	r7, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(5)?,31)?)?;
    // 0x44620: orr	r3, r3, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,15,false)?)?;
    // 0x44624: orr	r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x44628: ldrb	r3, [r2, #0x2ad]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,685)?)?;
    // 0x4462c: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x44630: orr	r0, r0, r3, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,25,false)?)?;
    // 0x44634: str	r0, [sp, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,372)?)?;
    // 0x44638: ldrb	r3, [r2, #0x2ae]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,686)?)?;
    // 0x4463c: ldrb	r7, [r2, #0x2af]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R2,687)?)?;
    // 0x44640: ldrb	r6, [r2, #0x2b0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R2,688)?)?;
    // 0x44644: ldrb	r5, [r2, #0x2b1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R2,689)?)?;
    // 0x44648: ldrb	r0, [r2, #0x2b2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R2,690)?)?;
    // 0x4464c: and	r7, r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,31)?)?;
    // 0x44650: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x44654: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x44658: orr	r3, r3, r7, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,5,false)?)?;
    // 0x4465c: and	r7, r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(6)?,31)?)?;
    // 0x44660: orr	r3, r3, r7, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,10,false)?)?;
    // 0x44664: and	r7, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(5)?,31)?)?;
    // 0x44668: orr	r3, r3, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,15,false)?)?;
    // 0x4466c: orr	r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x44670: ldrb	r3, [r2, #0x2b3]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,691)?)?;
    // 0x44674: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x44678: orr	r0, r0, r3, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,25,false)?)?;
    // 0x4467c: str	r0, [sp, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,392)?)?;
    // 0x44680: ldr	r6, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,2744)?)?;
    // 0x44684: ldrb	r4, [r2, #0x2b4]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R2,692)?)?;
    // 0x44688: ldrb	r2, [r2, #0x2b5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,693)?)?;
    // 0x4468c: ldrb	r0, [r6, #0x2b6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,694)?)?;
    // 0x44690: ldrb	r3, [r6, #0x2b7]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,695)?)?;
    // 0x44694: ldrb	r7, [r6, #0x2b8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,696)?)?;
    // 0x44698: ldrb	r5, [r6, #0x2b9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,697)?)?;
    // 0x4469c: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x446a0: and	r4, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(4)?,31)?)?;
    // 0x446a4: orr	r2, r4, r2, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(4)?,R::R2,Shift::Lsl,5,false)?)?;
    // 0x446a8: and	r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // 0x446ac: and	r7, r1, r7, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R1,R::new(7)?,Shift::Lsl,20,false)?)?;
    // 0x446b0: orr	r0, r2, r0, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,10,false)?)?;
    // 0x446b4: and	r2, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(3)?,31)?)?;
    // 0x446b8: orr	r0, r0, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,15,false)?)?;
    // 0x446bc: and	r2, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(5)?,31)?)?;
    // 0x446c0: orr	r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x446c4: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x446c8: str	r0, [sp, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,412)?)?;
    // 0x446cc: ldrb	r0, [r6, #0x2ba]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,698)?)?;
    // 0x446d0: ldr	r4, [sp, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,432)?)?;
    // 0x446d4: ldrb	r3, [r6, #0x2bb]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,699)?)?;
    // 0x446d8: ldrb	r7, [r6, #0x2bc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,700)?)?;
    // 0x446dc: ldrb	r5, [r6, #0x2bd]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,701)?)?;
    // 0x446e0: and	r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // 0x446e4: and	r4, r4, #-33554432
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(4)?,4261412864)?)?;
    // 0x446e8: and	r3, lr, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::LR,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x446ec: and	r7, r8, r7, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(8)?,R::new(7)?,Shift::Lsl,10,false)?)?;
    // 0x446f0: and	r5, r9, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(9)?,R::new(5)?,Shift::Lsl,15,false)?)?;
    // 0x446f4: orr	r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x446f8: orr	r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x446fc: orr	r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44700: orr	r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44704: ldrb	r5, [r6, #0x2be]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,702)?)?;
    // 0x44708: and	r5, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,31)?)?;
    // 0x4470c: orr	r0, r0, r5, lsl #20
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(5)?,Shift::Lsl,20,false)?)?;
    // 0x44710: str	r0, [sp, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,432)?)?;
    // 0x44714: str	r12, [sp, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,436)?)?;
    // 0x44718: ldr	r5, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,2744)?)?;
    // 0x4471c: ldrb	r3, [r6, #0x2bf]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,703)?)?;
    // 0x44720: ldrb	r7, [r6, #0x2c0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,704)?)?;
    // 0x44724: ldrb	r6, [r6, #0x2c1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(6)?,705)?)?;
    // 0x44728: ldrb	r0, [r5, #0x2c2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,706)?)?;
    // 0x4472c: ldrb	r4, [r5, #0x2c3]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,707)?)?;
    // 0x44730: ldrb	r2, [r5, #0x2c4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,708)?)?;
    // 0x44734: and	r7, r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,31)?)?;
    // 0x44738: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x4473c: and	r6, r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,31)?)?;
    // 0x44740: orr	r3, r3, r7, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,5,false)?)?;
    // 0x44744: orr	r3, r3, r6, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,10,false)?)?;
    // 0x44748: and	r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // 0x4474c: and	r4, r1, r4, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R1,R::new(4)?,Shift::Lsl,20,false)?)?;
    // 0x44750: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44754: orr	r0, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x44758: orr	r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4475c: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x44760: str	r0, [sp, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,376)?)?;
    // 0x44764: ldrb	r2, [r5, #0x2c5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,709)?)?;
    // 0x44768: ldrb	r3, [r5, #0x2c6]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,710)?)?;
    // 0x4476c: ldrb	r4, [r5, #0x2c7]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,711)?)?;
    // 0x44770: ldrb	r6, [r5, #0x2c8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,712)?)?;
    // 0x44774: ldrb	r0, [r5, #0x2c9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,713)?)?;
    // 0x44778: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x4477c: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44780: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x44784: orr	r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x44788: and	r3, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,31)?)?;
    // 0x4478c: orr	r2, r2, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x44790: and	r3, r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(6)?,31)?)?;
    // 0x44794: orr	r2, r2, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x44798: orr	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4479c: ldrb	r2, [r5, #0x2ca]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,714)?)?;
    // 0x447a0: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x447a4: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x447a8: str	r0, [sp, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,396)?)?;
    // 0x447ac: ldrb	r2, [r5, #0x2cb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,715)?)?;
    // 0x447b0: ldrb	r3, [r5, #0x2cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,716)?)?;
    // 0x447b4: ldrb	r4, [r5, #0x2cd]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,717)?)?;
    // 0x447b8: ldrb	r0, [r5, #0x2cf]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,719)?)?;
    // 0x447bc: ldrb	r5, [r5, #0x2ce]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(5)?,718)?)?;
    // 0x447c0: ldr	r6, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,2744)?)?;
    // 0x447c4: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x447c8: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x447cc: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x447d0: orr	r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x447d4: and	r3, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,31)?)?;
    // 0x447d8: orr	r2, r2, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x447dc: and	r3, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(5)?,31)?)?;
    // 0x447e0: orr	r2, r2, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x447e4: orr	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x447e8: ldrb	r2, [r6, #0x2d0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,720)?)?;
    // 0x447ec: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x447f0: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x447f4: str	r0, [sp, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,416)?)?;
    // 0x447f8: ldrb	r0, [r6, #0x2d1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,721)?)?;
    // 0x447fc: ldr	r5, [sp, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,436)?)?;
    // 0x44800: ldrb	r2, [r6, #0x2d2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,722)?)?;
    // 0x44804: ldrb	r3, [r6, #0x2d3]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,723)?)?;
    // 0x44808: ldrb	r4, [r6, #0x2d4]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,724)?)?;
    // 0x4480c: and	r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // 0x44810: and	r5, r5, #-33554432
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,4261412864)?)?;
    // 0x44814: and	r2, lr, r2, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::LR,R::R2,Shift::Lsl,5,false)?)?;
    // 0x44818: orr	r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4481c: orr	r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44820: and	r2, r8, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(8)?,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x44824: orr	r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44828: and	r2, r9, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(9)?,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x4482c: orr	r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44830: ldrb	r2, [r6, #0x2d5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,725)?)?;
    // 0x44834: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44838: orr	r0, r0, r2, lsl #20
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,20,false)?)?;
    // 0x4483c: str	r0, [sp, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,436)?)?;
    // 0x44840: str	r12, [sp, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,440)?)?;
    // 0x44844: ldrb	r2, [r6, #0x2d6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,726)?)?;
    // 0x44848: ldrb	r3, [r6, #0x2d7]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,727)?)?;
    // 0x4484c: ldrb	r4, [r6, #0x2d8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,728)?)?;
    // 0x44850: ldrb	r5, [r6, #0x2d9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,729)?)?;
    // 0x44854: ldrb	r0, [r6, #0x2da]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,730)?)?;
    // 0x44858: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x4485c: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44860: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x44864: orr	r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x44868: and	r3, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,31)?)?;
    // 0x4486c: orr	r2, r2, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x44870: and	r3, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(5)?,31)?)?;
    // 0x44874: orr	r2, r2, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x44878: orr	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4487c: ldrb	r2, [r6, #0x2db]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,731)?)?;
    // 0x44880: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44884: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x44888: str	r0, [sp, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,380)?)?;
    // 0x4488c: ldr	r6, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,2744)?)?;
    // 0x44890: ldrb	r2, [r6, #0x2dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,732)?)?;
    // 0x44894: ldrb	r3, [r6, #0x2dd]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,733)?)?;
    // 0x44898: ldrb	r4, [r6, #0x2de]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,734)?)?;
    // 0x4489c: ldrb	r5, [r6, #0x2df]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,735)?)?;
    // 0x448a0: ldrb	r0, [r6, #0x2e0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,736)?)?;
    // 0x448a4: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x448a8: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x448ac: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x448b0: orr	r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x448b4: and	r3, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,31)?)?;
    // 0x448b8: orr	r2, r2, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x448bc: and	r3, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(5)?,31)?)?;
    // 0x448c0: orr	r2, r2, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x448c4: orr	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x448c8: ldrb	r2, [r6, #0x2e1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,737)?)?;
    // 0x448cc: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x448d0: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x448d4: str	r0, [sp, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,400)?)?;
    // 0x448d8: ldrb	r2, [r6, #0x2e2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,738)?)?;
    // 0x448dc: ldrb	r3, [r6, #0x2e3]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,739)?)?;
    // 0x448e0: ldrb	r4, [r6, #0x2e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,740)?)?;
    // 0x448e4: ldrb	r5, [r6, #0x2e5]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,741)?)?;
    // 0x448e8: ldrb	r0, [r6, #0x2e6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,742)?)?;
    // 0x448ec: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x448f0: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x448f4: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x448f8: orr	r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x448fc: and	r3, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,31)?)?;
    // 0x44900: orr	r2, r2, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x44904: and	r3, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(5)?,31)?)?;
    // 0x44908: orr	r2, r2, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x4490c: orr	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x44910: ldrb	r2, [r6, #0x2e7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,743)?)?;
    // 0x44914: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44918: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x4491c: str	r0, [sp, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,420)?)?;
    // 0x44920: ldrb	r0, [r6, #0x2e8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,744)?)?;
    // 0x44924: ldr	r3, [sp, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,440)?)?;
    // 0x44928: ldrb	r2, [r6, #0x2e9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,745)?)?;
    // 0x4492c: ldr	r5, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,2744)?)?;
    // 0x44930: and	r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // 0x44934: and	r3, r3, #-33554432
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,4261412864)?)?;
    // 0x44938: and	r2, lr, r2, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::LR,R::R2,Shift::Lsl,5,false)?)?;
    // 0x4493c: ldrb	r4, [r5, #0x2ec]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,748)?)?;
    // 0x44940: orr	r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44944: ldrb	r3, [r5, #0x2eb]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,747)?)?;
    // 0x44948: orr	r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4494c: ldrb	r2, [r5, #0x2ea]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,746)?)?;
    // 0x44950: and	r2, r8, r2, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(8)?,R::R2,Shift::Lsl,10,false)?)?;
    // 0x44954: orr	r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44958: and	r2, r9, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(9)?,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x4495c: orr	r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44960: and	r2, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(4)?,31)?)?;
    // 0x44964: orr	r0, r0, r2, lsl #20
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,20,false)?)?;
    // 0x44968: str	r0, [sp, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,440)?)?;
    // 0x4496c: str	r12, [sp, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,444)?)?;
    // 0x44970: ldrb	r2, [r5, #0x2ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,749)?)?;
    // 0x44974: ldrb	r3, [r5, #0x2ee]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,750)?)?;
    // 0x44978: ldrb	r4, [r5, #0x2ef]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,751)?)?;
    // 0x4497c: ldrb	r6, [r5, #0x2f0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,752)?)?;
    // 0x44980: ldrb	r0, [r5, #0x2f1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,753)?)?;
    // 0x44984: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x44988: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x4498c: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x44990: orr	r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x44994: and	r3, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,31)?)?;
    // 0x44998: orr	r2, r2, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x4499c: and	r3, r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(6)?,31)?)?;
    // 0x449a0: orr	r2, r2, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x449a4: orr	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x449a8: ldrb	r2, [r5, #0x2f2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,754)?)?;
    // 0x449ac: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x449b0: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x449b4: str	r0, [sp, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,384)?)?;
    // 0x449b8: ldr	r6, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,2744)?)?;
    // 0x449bc: ldrb	r3, [r5, #0x2f3]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,755)?)?;
    // 0x449c0: ldrb	r4, [r5, #0x2f4]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,756)?)?;
    // 0x449c4: ldrb	r7, [r5, #0x2f5]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,757)?)?;
    // 0x449c8: ldrb	r5, [r5, #0x2f6]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(5)?,758)?)?;
    // 0x449cc: ldrb	r0, [r6, #0x2f7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,759)?)?;
    // 0x449d0: ldrb	r2, [r6, #0x2f8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,760)?)?;
    // 0x449d4: and	r4, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(4)?,31)?)?;
    // 0x449d8: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x449dc: orr	r3, r3, r4, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,5,false)?)?;
    // 0x449e0: and	r4, r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(7)?,31)?)?;
    // 0x449e4: orr	r3, r3, r4, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,10,false)?)?;
    // 0x449e8: and	r4, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(5)?,31)?)?;
    // 0x449ec: orr	r3, r3, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x449f0: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x449f4: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x449f8: orr	r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x449fc: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    c.check(0x44a00)
}

/// Construct remaining format-word bitfields and outgoing controller flags.
#[inline(never)]
fn stage_output_layout(c:&mut Code)->Result<(),String>{
    c.check(0x44a00)?;
    // 0x44a00: str	r0, [sp, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,404)?)?;
    // 0x44a04: ldrb	r2, [r6, #0x2f9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,761)?)?;
    // 0x44a08: ldrb	r3, [r6, #0x2fa]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,762)?)?;
    // 0x44a0c: ldrb	r4, [r6, #0x2fb]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,763)?)?;
    // 0x44a10: ldrb	r5, [r6, #0x2fc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,764)?)?;
    // 0x44a14: ldrb	r0, [r6, #0x2fd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,765)?)?;
    // 0x44a18: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x44a1c: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44a20: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x44a24: orr	r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x44a28: and	r3, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,31)?)?;
    // 0x44a2c: orr	r2, r2, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x44a30: and	r3, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(5)?,31)?)?;
    // 0x44a34: orr	r2, r2, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x44a38: orr	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x44a3c: ldrb	r2, [r6, #0x2fe]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,766)?)?;
    // 0x44a40: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44a44: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x44a48: str	r0, [sp, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,424)?)?;
    // 0x44a4c: ldrb	r0, [r6, #0x2ff]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,767)?)?;
    // 0x44a50: ldr	r5, [sp, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,444)?)?;
    // 0x44a54: ldrb	r2, [r6, #0x300]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,768)?)?;
    // 0x44a58: ldrb	r3, [r6, #0x301]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,769)?)?;
    // 0x44a5c: ldrb	r4, [r6, #0x302]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,770)?)?;
    // 0x44a60: and	r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // 0x44a64: and	r5, r5, #-33554432
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,4261412864)?)?;
    // 0x44a68: and	r2, lr, r2, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::LR,R::R2,Shift::Lsl,5,false)?)?;
    // 0x44a6c: orr	r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44a70: orr	r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44a74: and	r2, r8, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(8)?,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x44a78: orr	r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44a7c: and	r2, r9, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(9)?,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x44a80: orr	r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44a84: ldrb	r2, [r6, #0x303]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,771)?)?;
    // 0x44a88: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44a8c: orr	r0, r0, r2, lsl #20
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,20,false)?)?;
    // 0x44a90: str	r0, [sp, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,444)?)?;
    // 0x44a94: str	r12, [sp, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,448)?)?;
    // 0x44a98: ldr	r6, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,2744)?)?;
    // 0x44a9c: ldrb	r2, [r6, #0x304]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,772)?)?;
    // 0x44aa0: ldrb	r3, [r6, #0x305]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,773)?)?;
    // 0x44aa4: ldrb	r4, [r6, #0x306]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,774)?)?;
    // 0x44aa8: ldrb	r5, [r6, #0x307]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,775)?)?;
    // 0x44aac: ldrb	r0, [r6, #0x308]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,776)?)?;
    // 0x44ab0: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x44ab4: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44ab8: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x44abc: orr	r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x44ac0: and	r3, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,31)?)?;
    // 0x44ac4: orr	r2, r2, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x44ac8: and	r3, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(5)?,31)?)?;
    // 0x44acc: orr	r2, r2, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x44ad0: orr	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x44ad4: ldrb	r2, [r6, #0x309]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,777)?)?;
    // 0x44ad8: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44adc: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x44ae0: str	r0, [sp, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,388)?)?;
    // 0x44ae4: ldrb	r2, [r6, #0x30a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,778)?)?;
    // 0x44ae8: ldrb	r3, [r6, #0x30b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,779)?)?;
    // 0x44aec: ldrb	r4, [r6, #0x30c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,780)?)?;
    // 0x44af0: ldrb	r5, [r6, #0x30d]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,781)?)?;
    // 0x44af4: ldrb	r0, [r6, #0x30e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,782)?)?;
    // 0x44af8: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x44afc: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44b00: and	r0, r1, r0, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,20,false)?)?;
    // 0x44b04: orr	r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x44b08: and	r3, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,31)?)?;
    // 0x44b0c: orr	r2, r2, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x44b10: and	r3, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(5)?,31)?)?;
    // 0x44b14: orr	r2, r2, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x44b18: orr	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x44b1c: ldrb	r2, [r6, #0x30f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,783)?)?;
    // 0x44b20: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44b24: orr	r0, r0, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,25,false)?)?;
    // 0x44b28: str	r0, [sp, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,408)?)?;
    // 0x44b2c: ldr	r5, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,2744)?)?;
    // 0x44b30: ldrb	r3, [r5, #0x314]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,788)?)?;
    // 0x44b34: ldrb	r0, [r5, #0x312]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,786)?)?;
    // 0x44b38: ldrb	r2, [r5, #0x313]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,787)?)?;
    // 0x44b3c: ldrb	r4, [r5, #0x315]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,789)?)?;
    // 0x44b40: and	r1, r1, r3, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,20,false)?)?;
    // 0x44b44: ldrb	r3, [r6, #0x310]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,784)?)?;
    // 0x44b48: ldrb	r6, [r6, #0x311]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(6)?,785)?)?;
    // 0x44b4c: and	r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // 0x44b50: and	r2, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,31)?)?;
    // 0x44b54: and	r6, r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,31)?)?;
    // 0x44b58: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x44b5c: orr	r3, r3, r6, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,5,false)?)?;
    // 0x44b60: orr	r0, r3, r0, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R0,Shift::Lsl,10,false)?)?;
    // 0x44b64: orr	r0, r0, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,15,false)?)?;
    // 0x44b68: orr	r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44b6c: and	r1, r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(4)?,31)?)?;
    // 0x44b70: orr	r0, r0, r1, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,25,false)?)?;
    // 0x44b74: str	r0, [sp, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,428)?)?;
    // 0x44b78: ldrb	r0, [r5, #0x316]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,790)?)?;
    // 0x44b7c: ldr	r7, [sp, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,448)?)?;
    // 0x44b80: ldrb	r1, [r5, #0x317]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,791)?)?;
    // 0x44b84: ldrb	r2, [r5, #0x318]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,792)?)?;
    // 0x44b88: ldrb	r3, [r5, #0x319]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,793)?)?;
    // 0x44b8c: and	r0, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    // 0x44b90: and	r7, r7, #-33554432
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,4261412864)?)?;
    // 0x44b94: and	r1, lr, r1, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::LR,R::R1,Shift::Lsl,5,false)?)?;
    // 0x44b98: orr	r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44b9c: orr	r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44ba0: and	r1, r8, r2, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(8)?,R::R2,Shift::Lsl,10,false)?)?;
    // 0x44ba4: orr	r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44ba8: and	r1, r9, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x44bac: orr	r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44bb0: ldrb	r1, [r5, #0x31a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,794)?)?;
    // 0x44bb4: and	r1, r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,31)?)?;
    // 0x44bb8: orr	r0, r0, r1, lsl #20
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,20,false)?)?;
    // 0x44bbc: str	r0, [sp, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,448)?)?;
    // 0x44bc0: ldrb	r0, [r10, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,2462)?)?;
    // 0x44bc4: cmp	r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32)?)?;
    // 0x44bc8: bne	0x44ce4 <entry+0x1194>  @ imm = #0x114
    c.branch(C::Ne,0x44ce4)?;
    // 0x44bcc: ldr	r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,104)?)?;
    // 0x44bd0: mov	r9, #1136
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1136)?)?;
    // 0x44bd4: mov	r10, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,132)?)?;
    // 0x44bd8: mov	r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x44bdc: mov	r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x44be0: orr	r9, r9, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,8192)?)?;
    // 0x44be4: orr	r10, r10, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,9216)?)?;
    // 0x44be8: ldr	lr, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,0)?)?;
    // 0x44bec: mov	r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,72)?)?;
    // 0x44bf0: add	r2, lr, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::LR,R::R12,Shift::Lsl,0,false)?)?;
    // 0x44bf4: add	r7, lr, r5, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::LR,R::new(5)?,Shift::Lsl,3,false)?)?;
    // 0x44bf8: add	r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x44bfc: add	r12, r12, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R12,32)?)?;
    // 0x44c00: orr	r0, r0, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,9216)?)?;
    // 0x44c04: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x44c08: add	r6, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44c0c: add	r0, sp, #300
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,300)?)?;
    // 0x44c10: add	r3, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x44c14: ldreq	r0, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::R2,0)?)?;
    // 0x44c18: ldr	r8, [r3, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(3)?,72)?)?;
    // 0x44c1c: andeq	r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x44c20: streq	r0, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::R2,0)?)?;
    // 0x44c24: str	r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // 0x44c28: str	r8, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(7)?,4)?)?;
    // 0x44c2c: ldr	r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-36)?)?;
    // 0x44c30: ldr	r1, [r3, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,92)?)?;
    // 0x44c34: add	r0, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,1)?)?;
    // 0x44c38: str	r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x44c3c: mov	r0, #92
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,92)?)?;
    // 0x44c40: orr	r0, r0, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,9216)?)?;
    // 0x44c44: add	r8, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x44c48: add	r0, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,8)?)?;
    // 0x44c4c: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x44c50: ldreq	r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(7)?,8)?)?;
    // 0x44c54: andeq	r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x44c58: streq	r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(7)?,8)?)?;
    // 0x44c5c: add	r0, r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,2)?)?;
    // 0x44c60: str	r8, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(7)?,8)?)?;
    // 0x44c64: str	r1, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,12)?)?;
    // 0x44c68: add	r8, r5, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::new(5)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x44c6c: str	r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x44c70: add	r0, r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,16)?)?;
    // 0x44c74: ldr	r1, [r3, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,112)?)?;
    // 0x44c78: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x44c7c: ldreq	r0, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(7)?,16)?)?;
    // 0x44c80: andeq	r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x44c84: streq	r0, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(7)?,16)?)?;
    // 0x44c88: str	r8, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(7)?,16)?)?;
    // 0x44c8c: str	r1, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,20)?)?;
    // 0x44c90: add	r0, r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,3)?)?;
    // 0x44c94: ldr	r1, [r3, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,132)?)?;
    // 0x44c98: add	r3, r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,24)?)?;
    // 0x44c9c: str	r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x44ca0: add	r0, r5, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x44ca4: add	r5, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4)?)?;
    // 0x44ca8: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x44cac: ldreq	r3, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(7)?,24)?)?;
    // 0x44cb0: andeq	r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x44cb4: streq	r3, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::new(7)?,24)?)?;
    // 0x44cb8: str	r0, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,24)?)?;
    // 0x44cbc: cmp	r4, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,5)?)?;
    // 0x44cc0: add	r0, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,4)?)?;
    // 0x44cc4: str	r1, [r2, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,28)?)?;
    // 0x44cc8: str	r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x44ccc: bne	0x44bec <entry+0x109c>  @ imm = #-0xe8
    c.branch(C::Ne,0x44bec)?;
    // 0x44cd0: ldr	r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x44cd4: ldr	r10, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,88)?)?;
    // 0x44cd8: ldr	r8, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,80)?)?;
    // 0x44cdc: add	r3, lr, #160
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::LR,160)?)?;
    // 0x44ce0: b	0x44dc0 <entry+0x1270>  @ imm = #0xd8
    c.branch(C::Al,0x44dc0)?;
    // 0x44ce4: ldr	r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x44ce8: ldr	r1, [sp, #0x178]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,376)?)?;
    // 0x44cec: ldr	r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x44cf0: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x44cf4: add	r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x44cf8: ldreq	r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x44cfc: andeq	r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x44d00: streq	r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x44d04: mov	r2, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,76)?)?;
    // 0x44d08: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x44d0c: orr	r2, r2, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,9216)?)?;
    // 0x44d10: str	r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x44d14: str	r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x44d18: ldr	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x44d1c: add	r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,1)?)?;
    // 0x44d20: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x44d24: str	r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x44d28: ldreq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x44d2c: ldr	r2, [sp, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,396)?)?;
    // 0x44d30: andeq	r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x44d34: streq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x44d38: mov	r3, #1120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1120)?)?;
    // 0x44d3c: orr	r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x44d40: str	r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x44d44: str	r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x44d48: add	r2, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,2)?)?;
    // 0x44d4c: add	r3, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,16)?)?;
    // 0x44d50: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x44d54: str	r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x44d58: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x44d5c: ldreq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x44d60: ldr	r2, [sp, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,416)?)?;
    // 0x44d64: ldr	r8, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,80)?)?;
    // 0x44d68: andeq	r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x44d6c: streq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x44d70: mov	r3, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,116)?)?;
    // 0x44d74: orr	r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x44d78: str	r3, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,16)?)?;
    // 0x44d7c: str	r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,20)?)?;
    // 0x44d80: add	r2, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,3)?)?;
    // 0x44d84: add	r3, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,24)?)?;
    // 0x44d88: add	r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // 0x44d8c: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x44d90: str	r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x44d94: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x44d98: ldreq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x44d9c: ldr	r2, [sp, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,436)?)?;
    // 0x44da0: andeq	r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x44da4: streq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x44da8: mov	r3, #136
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,136)?)?;
    // 0x44dac: orr	r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x44db0: str	r3, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,24)?)?;
    // 0x44db4: add	r3, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,32)?)?;
    // 0x44db8: str	r2, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,28)?)?;
    // 0x44dbc: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x44dc0: str	r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x44dc4: mov	r1, #248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,248)?)?;
    // 0x44dc8: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x44dcc: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x44dd0: orr	r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x44dd4: ldrh	r1, [r0, r1]
    c.emit(super::venus_controller_packets::load_halfword_register(C::Al,R::R1,R::R0,R::R1)?)?;
    // 0x44dd8: ldr	r0, [r0, #0x4f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1268)?)?;
    // 0x44ddc: orr	r0, r1, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x44de0: ldr	r1, [pc, #0xe98]        @ 0x2130 <entry+0x2130>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3736)?)?;
    // 0x44de4: and	r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x44de8: str	r0, [sp, #0x1cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,460)?)?;
    // 0x44dec: ldreq	r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(3)?,0)?)?;
    // 0x44df0: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x44df4: streq	r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(3)?,0)?)?;
    // 0x44df8: mov	r1, #9472
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,9472)?)?;
    // 0x44dfc: str	r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // 0x44e00: str	r0, [r3, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,4)?)?;
    // 0x44e04: add	r0, r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,8)?)?;
    // 0x44e08: ldr	r7, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-36)?)?;
    // 0x44e0c: add	lr, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(7)?,1)?)?;
    // 0x44e10: str	lr, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R11,-36)?)?;
    // 0x44e14: str	r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x44e18: ldrb	r12, [r10, #0x3d9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(10)?,985)?)?;
    // 0x44e1c: cmp	r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x44e20: beq	0x44e9c <entry+0x134c>  @ imm = #0x74
    c.branch(C::Eq,0x44e9c)?;
    // 0x44e24: ldr	r2, [pc, #0xe58]        @ 0x2134 <entry+0x2134>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3672)?)?;
    // 0x44e28: mov	r6, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,100)?)?;
    // 0x44e2c: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x44e30: add	lr, r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(7)?,3)?)?;
    // 0x44e34: str	r2, [sp, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,464)?)?;
    // 0x44e38: str	r6, [sp, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,468)?)?;
    // 0x44e3c: ldreq	r6, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R0,0)?)?;
    // 0x44e40: andeq	r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x44e44: streq	r6, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R0,0)?)?;
    // 0x44e48: mov	r0, #260
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,260)?)?;
    // 0x44e4c: orr	r0, r0, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,9216)?)?;
    // 0x44e50: str	r0, [r3, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,8)?)?;
    // 0x44e54: str	r2, [r3, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,12)?)?;
    // 0x44e58: add	r0, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,2)?)?;
    // 0x44e5c: add	r2, r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,16)?)?;
    // 0x44e60: ldr	r1, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,104)?)?;
    // 0x44e64: str	r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x44e68: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x44e6c: str	r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x44e70: ldreq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R2,0)?)?;
    // 0x44e74: ldr	r0, [sp, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,468)?)?;
    // 0x44e78: andeq	r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x44e7c: streq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,0)?)?;
    // 0x44e80: mov	r2, #264
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,264)?)?;
    // 0x44e84: orr	r2, r2, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,9216)?)?;
    // 0x44e88: str	r2, [r3, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,16)?)?;
    // 0x44e8c: str	r0, [r3, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,20)?)?;
    // 0x44e90: add	r0, r3, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,24)?)?;
    // 0x44e94: str	lr, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R11,-36)?)?;
    // 0x44e98: str	r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x44e9c: ldr	r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x44ea0: ldrb	r3, [r7, #0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(7)?,12)?)?;
    // 0x44ea4: cmp	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x44ea8: ldrne	r3, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::new(8)?,400)?)?;
    // 0x44eac: cmpne	r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x44eb0: beq	0x44ec8 <entry+0x1378>  @ imm = #0x10
    c.branch(C::Eq,0x44ec8)?;
    // 0x44eb4: ldr	r3, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,348)?)?;
    // 0x44eb8: bic	r3, r3, #983040
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,983040)?)?;
    // 0x44ebc: b	0x44f54 <entry+0x1404>  @ imm = #0x90
    c.branch(C::Al,0x44f54)?;
    // 0x44ec0: andeq	lr, lr, lr, lsl #2
    c.emit(arm32::data_register(C::Eq,Op::And,R::LR,R::LR,R::LR,Shift::Lsl,2,false)?)?;
    // 0x44ec4: andeq	r2, lr, r8, lsl #15
    c.emit(arm32::data_register(C::Eq,Op::And,R::R2,R::LR,R::new(8)?,Shift::Lsl,15,false)?)?;
    // 0x44ec8: ldr	r6, [r7, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,192)?)?;
    // 0x44ecc: ldr	r3, [r7, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,188)?)?;
    // 0x44ed0: ldr	r2, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,348)?)?;
    // 0x44ed4: ldr	r5, [r7, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,196)?)?;
    // 0x44ed8: ldr	r4, [r7, #0xc8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,200)?)?;
    // 0x44edc: mov	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x44ee0: cmp	r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x44ee4: mov	r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x44ee8: bic	r2, r2, #196608
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,196608)?)?;
    // 0x44eec: movgt	r6, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x44ef0: cmp	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x44ef4: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x44ef8: movgt	r3, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x44efc: cmp	r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x44f00: mov	r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x44f04: orr	r2, r2, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x44f08: movgt	r5, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x44f0c: orr	r2, r2, r6, lsl #17
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(6)?,Shift::Lsl,17,false)?)?;
    // 0x44f10: bic	r3, r2, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::R2,262144)?)?;
    // 0x44f14: orr	r5, r3, r5, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(3)?,R::new(5)?,Shift::Lsl,18,false)?)?;
    // 0x44f18: ldrb	r3, [r7, #0x49a]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(7)?,1178)?)?;
    // 0x44f1c: cmp	r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,3)?)?;
    // 0x44f20: orreq	r5, r2, #262144
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(5)?,R::R2,262144)?)?;
    // 0x44f24: cmp	r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x44f28: bic	r2, r5, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::new(5)?,524288)?)?;
    // 0x44f2c: movgt	r1, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x44f30: orr	r3, r2, r1, lsl #19
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R2,R::R1,Shift::Lsl,19,false)?)?;
    // 0x44f34: and	r1, r5, #458752
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(5)?,458752)?)?;
    // 0x44f38: cmp	r1, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,458752)?)?;
    // 0x44f3c: str	r3, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,348)?)?;
    // 0x44f40: bne	0x44f58 <entry+0x1408>  @ imm = #0x10
    c.branch(C::Ne,0x44f58)?;
    // 0x44f44: ldrb	r1, [r7, #0x4f0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(7)?,1264)?)?;
    // 0x44f48: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x44f4c: beq	0x44f58 <entry+0x1408>  @ imm = #0x4
    c.branch(C::Eq,0x44f58)?;
    // 0x44f50: bic	r3, r3, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,262144)?)?;
    // 0x44f54: str	r3, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,348)?)?;
    // 0x44f58: ldrb	r1, [r7, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(7)?,204)?)?;
    // 0x44f5c: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x44f60: bne	0x44f7c <entry+0x142c>  @ imm = #0x14
    c.branch(C::Ne,0x44f7c)?;
    // 0x44f64: ldrb	r1, [r7, #0xe8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(7)?,232)?)?;
    // 0x44f68: cmp	r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x44f6c: beq	0x44f7c <entry+0x142c>  @ imm = #0x8
    c.branch(C::Eq,0x44f7c)?;
    // 0x44f70: ldrb	r1, [r10, #0x495]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1173)?)?;
    // 0x44f74: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x44f78: beq	0x44f98 <entry+0x1448>  @ imm = #0x18
    c.branch(C::Eq,0x44f98)?;
    // 0x44f7c: ldrb	r1, [r10, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,972)?)?;
    // 0x44f80: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x44f84: beq	0x44f98 <entry+0x1448>  @ imm = #0xc
    c.branch(C::Eq,0x44f98)?;
    // 0x44f88: ldr	r1, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,644)?)?;
    // 0x44f8c: cmp	r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x44f90: orreq	r3, r3, #131072
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(3)?,R::new(3)?,131072)?)?;
    // 0x44f94: streq	r3, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::SP,348)?)?;
    // 0x44f98: cmp	r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x44f9c: bicne	r1, r3, #1040187392
    c.emit(arm32::data_immediate(C::Ne,Op::Bic,R::R1,R::new(3)?,1040187392)?)?;
    // 0x44fa0: orrne	r3, r1, #33554432
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(3)?,R::R1,33554432)?)?;
    // 0x44fa4: strne	r3, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Ne,false,R::new(3)?,R::SP,348)?)?;
    // 0x44fa8: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x44fac: ldreq	r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x44fb0: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x44fb4: streq	r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x44fb8: mov	r1, #1072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1072)?)?;
    // 0x44fbc: orr	r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x44fc0: stm	r0, {r1, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x44fc4: add	r1, lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::LR,1)?)?;
    // 0x44fc8: add	r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x44fcc: ldr	lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x44fd0: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x44fd4: str	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x44fd8: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x44fdc: ldrb	r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x44fe0: cmp	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x44fe4: bne	0x46568 <entry+0x2a18>  @ imm = #0x157c
    c.branch(C::Ne,0x46568)?;
    // 0x44fe8: ldr	r0, [r10, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2652)?)?;
    // 0x44fec: mov	r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x44ff0: ldr	r1, [r10, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2656)?)?;
    // 0x44ff4: orr	r2, r2, #16128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,16128)?)?;
    // 0x44ff8: and	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x44ffc: mov	r2, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16711680)?)?;
    c.check(0x45000)
}

/// Standalone partial function slice (never filled with missing source bytes).
pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();
    initialize_controller_format(&mut c)?;
    derive_stream_metrics(&mut c)?;
    populate_controller_fields(&mut c)?;
    stage_output_layout(&mut c)?;
    Ok(c.bytes)
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn format_prefix_instructions_are_contiguous(){
        assert_eq!(fragment().unwrap().len(),END-START);
        assert_eq!(END,0x45000);
    }
}
