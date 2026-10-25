// SPDX-License-Identifier: GPL-3.0-only
//! Rebuild the Venus A32 controller packet-list constructor at runtime
//! `[0x43b50, 0x43ff0)`. This procedure populates command words for the
//! active codec plane and publishes a series of range command records.
//!
//! All executable words are encoded from typed public ARM32 operands.
//! There are no proprietary instruction constants or literal pools in this
//! region. Byte identity is checked using the privately pinned `venus.mbn`.
//! This is instruction-level reconstruction, not validated behavioral source.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};

pub const START:usize=0x43b50;
pub const END:usize=0x43ff0;
pub const FILE_BASE:usize=0x4000;

struct Code { at:usize, bytes:Vec<u8> }
impl Code {
    fn new()->Self {Self{at:START,bytes:Vec::with_capacity(END-START)}}
    fn check(&self,a:usize)->Result<(),String>{
        if self.at!=a {return Err(format!("Venus packet command at 0x{:x}, expected 0x{a:x}",self.at));}
        Ok(())
    }
    fn emit(&mut self,w:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at) || self.at%4!=0 {return Err(format!("outside owned range at 0x{:x}",self.at));}
        self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(cond,self.at as u32,target as u32,false)?)
    }
}

/// Initialize codec command header and choose the active plane layout.
fn prepare_packet_state(c:&mut Code)->Result<(),String>{
    c.check(0x43b50)?;
    // 0x43b50: push	{r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x43b54: add	r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x43b58: sub	sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // 0x43b5c: mov	r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x43b60: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43b64: mov	r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // 0x43b68: mov	r6, #14680064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,14680064)?)?;
    // 0x43b6c: str	r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x43b70: ldr	r1, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2744)?)?;
    // 0x43b74: ldrb	r4, [r5, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(5)?,2462)?)?;
    // 0x43b78: ldrb	r7, [r1, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R1,336)?)?;
    // 0x43b7c: ldrb	r2, [r1, #0x143]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,323)?)?;
    // 0x43b80: ldrb	r3, [r1, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,325)?)?;
    // 0x43b84: ldrb	r1, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,339)?)?;
    // 0x43b88: cmp	r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,16)?)?;
    // 0x43b8c: movne	r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x43b90: lsl	r0, r0, #14
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,14)?)?;
    // 0x43b94: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x43b98: and	r6, r6, r7, lsl #21
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(6)?,R::new(7)?,Shift::Lsl,21,false)?)?;
    // 0x43b9c: orr	r7, r2, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x43ba0: movne	r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x43ba4: cmp	r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x43ba8: orr	r0, r0, r1, lsl #11
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,11,false)?)?;
    // 0x43bac: movne	r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x43bb0: cmp	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x43bb4: orr	r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x43bb8: movne	r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x43bbc: tst	r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,255)?)?;
    // 0x43bc0: orr	r0, r0, r2, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,24,false)?)?;
    // 0x43bc4: orr	r1, r0, r3, lsl #20
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::new(3)?,Shift::Lsl,20,false)?)?;
    // 0x43bc8: ldr	r0, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x43bcc: bne	0x43be0 <entry+0x90>    @ imm = #0xc
    c.branch(C::Ne,0x43be0)?;
    // 0x43bd0: ldr	r2, [r5, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,692)?)?;
    // 0x43bd4: ldr	r3, [r5, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,716)?)?;
    // 0x43bd8: cmp	r2, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x43bdc: bls	0x43be8 <entry+0x98>    @ imm = #0x4
    c.branch(C::Ls,0x43be8)?;
    // 0x43be0: orr	r1, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16)?)?;
    // 0x43be4: b	0x43bf8 <entry+0xa8>    @ imm = #0xc
    c.branch(C::Al,0x43bf8)?;
    // 0x43be8: ldr	r2, [r5, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,696)?)?;
    // 0x43bec: ldr	r3, [r5, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,720)?)?;
    // 0x43bf0: cmp	r2, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x43bf4: orrhi	r1, r1, #16
    c.emit(arm32::data_immediate(C::Hi,Op::Orr,R::R1,R::R1,16)?)?;
    // 0x43bf8: ldr	r2, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,0)?)?;
    // 0x43bfc: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x43c00: ldreq	r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R2,0)?)?;
    // 0x43c04: andeq	r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x43c08: streq	r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R2,0)?)?;
    // 0x43c0c: mov	r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14336)?)?;
    // 0x43c10: str	r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // 0x43c14: str	r1, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,4)?)?;
    // 0x43c18: ldr	r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // 0x43c1c: add	r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x43c20: str	r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x43c24: add	r1, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,8)?)?;
    // 0x43c28: str	r1, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // 0x43c2c: ldrb	r1, [r5, #0xcc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,3272)?)?;
    // 0x43c30: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x43c34: beq	0x43c5c <entry+0x10c>   @ imm = #0x20
    c.branch(C::Eq,0x43c5c)?;
    // 0x43c38: ldr	r1, [r5, #0xcec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,3308)?)?;
    // 0x43c3c: ldr	r10, [r5, #0xce8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(5)?,3304)?)?;
    // 0x43c40: str	r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x43c44: ldr	r1, [r5, #0xcf4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,3316)?)?;
    // 0x43c48: ldr	r7, [r5, #0xcf0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,3312)?)?;
    // 0x43c4c: str	r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x43c50: add	r1, r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,20)?)?;
    // 0x43c54: str	r1, [r5, #0x950]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,2384)?)?;
    // 0x43c58: b	0x43c74 <entry+0x124>   @ imm = #0x14
    c.branch(C::Al,0x43c74)?;
    // 0x43c5c: ldr	r1, [r5, #0xad0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2768)?)?;
    // 0x43c60: ldr	r10, [r5, #0xacc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(5)?,2764)?)?;
    // 0x43c64: str	r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x43c68: ldr	r7, [r5, #0x52c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,1324)?)?;
    // 0x43c6c: ldr	r1, [r5, #0x530]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1328)?)?;
    // 0x43c70: str	r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    c.check(0x43c74)
}

/// Publish Y/UV plane regions through platform callback records.
fn publish_plane_descriptors(c:&mut Code)->Result<(),String>{
    c.check(0x43c74)?;
    // 0x43c74: ldr	r6, [r0, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,420)?)?;
    // 0x43c78: add	r8, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,32)?)?;
    // 0x43c7c: mov	r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x43c80: mov	r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x43c84: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43c88: ldr	r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    // 0x43c8c: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43c90: str	r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43c94: add	r4, r0, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x43c98: mov	r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x43c9c: stm	sp, {r0, r4, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(4)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x43ca0: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43ca4: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5c80
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43ca8: ldr	r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x43cac: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43cb0: str	r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43cb4: mov	r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x43cb8: mov	r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x43cbc: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43cc0: add	r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x43cc4: add	r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x43cc8: mov	r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x43ccc: str	r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x43cd0: str	r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x43cd4: ldr	r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x43cd8: str	r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x43cdc: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43ce0: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5cbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43ce4: ldrb	r0, [r5, #0x3ba]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,954)?)?;
    // 0x43ce8: cmp	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x43cec: beq	0x43d50 <entry+0x200>   @ imm = #0x5c
    c.branch(C::Eq,0x43d50)?;
    // 0x43cf0: ldr	r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    // 0x43cf4: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43cf8: str	r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43cfc: str	r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x43d00: mov	r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x43d04: mov	r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x43d08: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43d0c: stmib	sp, {r0, r10}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(10)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43d10: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43d14: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5cf0
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43d18: ldr	r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    // 0x43d1c: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43d20: str	r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43d24: str	r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x43d28: mov	r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x43d2c: mov	r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x43d30: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43d34: add	r0, r0, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x43d38: add	r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x43d3c: str	r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x43d40: ldr	r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x43d44: str	r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x43d48: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43d4c: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5d28
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43d50: ldrb	r0, [r5, #0xcc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,3272)?)?;
    // 0x43d54: cmp	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x43d58: beq	0x43fe4 <entry+0x494>   @ imm = #0x284
    c.branch(C::Eq,0x43fe4)?;
    // 0x43d5c: ldr	r0, [r5, #0xccc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,3276)?)?;
    // 0x43d60: mov	r4, r8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
    // 0x43d64: mov	r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x43d68: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43d6c: str	r0, [r5, #0x978]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2424)?)?;
    // 0x43d70: ldr	r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x43d74: add	r6, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x43d78: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43d7c: mov	r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x43d80: str	r6, [r5, #0x970]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,2416)?)?;
    // 0x43d84: ldr	r7, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,2800)?)?;
    // 0x43d88: ldr	r10, [r5, #0xaf4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(5)?,2804)?)?;
    // 0x43d8c: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43d90: str	r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43d94: stm	sp, {r0, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(6)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x43d98: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43d9c: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5d78
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43da0: ldr	r0, [r5, #0x970]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2416)?)?;
    // 0x43da4: ldr	r1, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2800)?)?;
    // 0x43da8: mov	r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x43dac: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43db0: str	r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43db4: mov	r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x43db8: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43dbc: str	r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x43dc0: stmib	sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43dc4: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43dc8: mov	r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x43dcc: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5da8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43dd0: ldr	r0, [r5, #0xaf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2808)?)?;
    // 0x43dd4: add	r6, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x43dd8: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43ddc: str	r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43de0: mov	r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x43de4: mov	r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x43de8: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43dec: add	r7, r6, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x43df0: mov	r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x43df4: str	r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // 0x43df8: str	r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x43dfc: str	r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x43e00: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43e04: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5de0
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43e08: ldr	r0, [r5, #0xaf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2808)?)?;
    // 0x43e0c: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43e10: str	r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43e14: str	r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x43e18: str	r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x43e1c: mov	r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x43e20: mov	r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x43e24: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43e28: str	r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x43e2c: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43e30: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5e0c
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43e34: ldrb	r0, [r5, #0x3b9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,953)?)?;
    // 0x43e38: cmp	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x43e3c: beq	0x43ef0 <entry+0x3a0>   @ imm = #0xac
    c.branch(C::Eq,0x43ef0)?;
    // 0x43e40: ldr	r0, [r5, #0x978]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2424)?)?;
    // 0x43e44: ldr	r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x43e48: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43e4c: str	r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43e50: str	r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // 0x43e54: mov	r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x43e58: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43e5c: stmib	sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43e60: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43e64: mov	r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x43e68: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5e44
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43e6c: ldr	r0, [r5, #0x978]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2424)?)?;
    // 0x43e70: ldr	r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x43e74: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43e78: str	r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43e7c: str	r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x43e80: mov	r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x43e84: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43e88: stmib	sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43e8c: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43e90: mov	r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x43e94: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5e70
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43e98: ldr	r0, [r5, #0xaf4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2804)?)?;
    // 0x43e9c: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43ea0: str	r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43ea4: str	r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // 0x43ea8: str	r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // 0x43eac: mov	r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x43eb0: mov	r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x43eb4: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43eb8: str	r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x43ebc: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43ec0: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5e9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    // 0x43ec4: ldr	r0, [r5, #0xaf4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2804)?)?;
    // 0x43ec8: str	r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43ecc: str	r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43ed0: str	r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x43ed4: str	r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // 0x43ed8: mov	r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x43edc: mov	r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x43ee0: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43ee4: str	r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x43ee8: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43eec: bl	0x3e02c <entry+0xffffffffffffa4dc> @ imm = #-0x5ec8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
    c.check(0x43ef0)
}

/// Append controller flush/control records, and return the packet count.
fn finish_packet_sequence(c:&mut Code)->Result<(),String>{
    c.check(0x43ef0)?;
    // 0x43ef0: ldr	r1, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x43ef4: mvn	r2, #1600
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,1600)?)?;
    // 0x43ef8: tst	r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x43efc: ldreq	r0, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::R1,0)?)?;
    // 0x43f00: andeq	r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x43f04: streq	r0, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::R1,0)?)?;
    // 0x43f08: mov	r0, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,96)?)?;
    // 0x43f0c: stm	r1, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit() | R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x43f10: ldr	r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x43f14: add	r3, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,1)?)?;
    // 0x43f18: str	r3, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,32)?)?;
    // 0x43f1c: add	r3, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,8)?)?;
    // 0x43f20: str	r3, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x43f24: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x43f28: ldreq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x43f2c: andeq	r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x43f30: streq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x43f34: mov	r3, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,100)?)?;
    // 0x43f38: str	r3, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,8)?)?;
    // 0x43f3c: str	r2, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,12)?)?;
    // 0x43f40: add	r2, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,2)?)?;
    // 0x43f44: str	r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,32)?)?;
    // 0x43f48: add	r2, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,16)?)?;
    // 0x43f4c: str	r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x43f50: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x43f54: ldreq	r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R2,0)?)?;
    // 0x43f58: andeq	r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x43f5c: streq	r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R2,0)?)?;
    // 0x43f60: mov	r3, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,104)?)?;
    // 0x43f64: mvn	r2, #1600
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,1600)?)?;
    // 0x43f68: str	r3, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,16)?)?;
    // 0x43f6c: add	r3, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,3)?)?;
    // 0x43f70: str	r2, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,20)?)?;
    // 0x43f74: str	r3, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,32)?)?;
    // 0x43f78: add	r3, r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,24)?)?;
    // 0x43f7c: str	r3, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x43f80: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x43f84: ldreq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x43f88: andeq	r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x43f8c: streq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x43f90: mov	r3, #108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,108)?)?;
    // 0x43f94: str	r3, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,24)?)?;
    // 0x43f98: str	r2, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,28)?)?;
    // 0x43f9c: add	r2, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,4)?)?;
    // 0x43fa0: add	r0, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5)?)?;
    // 0x43fa4: str	r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,32)?)?;
    // 0x43fa8: add	r2, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,32)?)?;
    // 0x43fac: str	r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x43fb0: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x43fb4: ldreq	r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R2,0)?)?;
    // 0x43fb8: andeq	r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x43fbc: streq	r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R2,0)?)?;
    // 0x43fc0: mov	r2, #2256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2256)?)?;
    // 0x43fc4: mov	r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,256)?)?;
    // 0x43fc8: orr	r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x43fcc: str	r2, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,32)?)?;
    // 0x43fd0: str	r3, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,36)?)?;
    // 0x43fd4: add	r1, r1, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,40)?)?;
    // 0x43fd8: str	r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x43fdc: str	r1, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // 0x43fe0: b	0x43fe8 <entry+0x498>   @ imm = #0x0
    c.branch(C::Al,0x43fe8)?;
    // 0x43fe4: ldr	r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x43fe8: sub	sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x43fec: pop	{r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x43ff0)
}

/// Emit only the exact original executable region; never pad unknown bytes.
pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();
    prepare_packet_state(&mut c)?;
    publish_plane_descriptors(&mut c)?;
    finish_packet_sequence(&mut c)?;
    Ok(c.bytes)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]
    fn original_controller_packet_structure_and_return(){
        let data=fragment().unwrap();
        assert_eq!(data.len(),1184);
        assert_eq!(START+data.len(),END);
        assert_eq!(u32::from_le_bytes(data[0..4].try_into().unwrap()),
                   arm32::push_registers(C::Al,
                       R::new(4).unwrap().bit()|R::new(5).unwrap().bit()|
                       R::new(6).unwrap().bit()|R::new(7).unwrap().bit()|
                       R::new(8).unwrap().bit()|R::new(9).unwrap().bit()|
                       R::new(10).unwrap().bit()|R::R11.bit()|R::LR.bit()).unwrap());
        assert_eq!(u32::from_le_bytes(data[1180..1184].try_into().unwrap()),
                   arm32::transfer_multiple(C::Al,true,R::SP,
                       R::new(4).unwrap().bit()|R::new(5).unwrap().bit()|
                       R::new(6).unwrap().bit()|R::new(7).unwrap().bit()|
                       R::new(8).unwrap().bit()|R::new(9).unwrap().bit()|
                       R::new(10).unwrap().bit()|R::R11.bit()|R::PC.bit(),
                       BlockMode::IncrementAfter,true).unwrap());
    }
}
