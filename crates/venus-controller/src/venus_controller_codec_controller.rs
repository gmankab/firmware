// SPDX-License-Identifier: GPL-3.0-only
//! Format-specific codec controller packet constructor. This complete A32
//! routine serializes four kinds of pixel-plane descriptor, uses signed DSP
//! halfword arithmetic and calls public ARM-to-Thumb math routines.
//!
//! A32 instruction words are **recomputed** from typed registers, shifts,
//! conditions and architectural encodings. The three following literal words
//! are marked separately and are not misclassified as code.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::venus_controller_format_registers::signed_byte_register;

pub const START:usize=0x40c14;
pub const CODE_END:usize=0x412ac;
pub const END:usize=0x412b8;
pub const FILE_BASE:usize=0x4000;
const CONTROLLER_DATA:[u32;3]=[0x2620_1c16,0x8888_8889,0xffff_fdda];

/// A32 SMULL, signed multiply 32 x 32 into RdHi:RdLo. Register pairing and
/// prohibited PC operands are checked before emitting an instruction.
pub(super) fn signed_long_multiply(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
    if lo==hi || [lo,hi,rm,rs].contains(&R::PC) {
        return Err("invalid SMULL operand registers".into());
    }
    Ok(((cond as u32)<<28)|0x00c0_0090|(hi.number()<<16)|
        (lo.number()<<12)|(rs.number()<<8)|rm.number())
}

/// Public ARM DSP `SMULBB`: signed low-halfword multiply into a 32-bit word.
fn signed_multiply_bottom_halves(cond:C,rd:R,rm:R,rs:R)->Result<u32,String>{
    if [rd,rm,rs].iter().any(|x|x.number()>12) {
        return Err("invalid SMULBB register".into());
    }
    Ok(((cond as u32)<<28)|0x0160_0080|
        (rd.number()<<16)|(rs.number()<<8)|rm.number())
}

/// Public ARM DSP `SMLABB`: signed halfword multiply plus 32-bit accumulator.
fn signed_multiply_add_bottom_halves(cond:C,rd:R,rm:R,rs:R,acc:R)->Result<u32,String>{
    if [rd,rm,rs,acc].iter().any(|x|x.number()>12) {
        return Err("invalid SMLABB register".into());
    }
    Ok(((cond as u32)<<28)|0x0100_0080|
        (rd.number()<<16)|(acc.number()<<12)|(rs.number()<<8)|rm.number())
}

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at!=addr {return Err(format!("codec controller at 0x{:x} expected 0x{addr:x}",self.at));}
        Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<0x40000 || self.at>=0x68000 || self.at%4!=0{
            return Err(format!("outside exclusive Agent 04 Venus range at 0x{:x}",self.at));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(cond,self.at as u32,target as u32,false)?)
    }
    fn literal(&mut self,rd:R,target:usize)->Result<(),String>{
        let signed=i32::try_from(target as isize-self.at as isize-8)
            .map_err(|_|"A32 literal displacement out of range")?;
        self.emit(arm32::transfer_word(C::Al,true,rd,R::PC,signed)?)
    }
}

/// Serialize primary/secondary controller descriptors, convert signed input
/// bytes, compute fixed-point dimensions with SMULL/SMULBB/SMLABB and dispatch
/// helper math via BLX. The real return is `pop {...,pc}` at 0x412a8.
fn write_controller_packet(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x40c14: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x40c18: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x40c1c: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // 0x40c20: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x40c24: ldr lr, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,44)?)?;
    // 0x40c28: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x40c2c: orr r0, r0, #671088640
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,671088640)?)?;
    // 0x40c30: ldr r3, [lr, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::LR,400)?)?;
    // 0x40c34: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x40c38: beq 0x40c4c <entry+0xd4c>
    c.branch(C::Eq,0x40c4c)?;
    // 0x40c3c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x40c40: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x40c44: moveq r2, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x40c48: b 0x40c6c <entry+0xd6c>
    c.branch(C::Al,0x40c6c)?;
    // 0x40c4c: ldrb r3, [lr, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::LR,1005)?)?;
    // 0x40c50: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x40c54: beq 0x40c68 <entry+0xd68>
    c.branch(C::Eq,0x40c68)?;
    // 0x40c58: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x40c5c: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x40c60: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x40c64: b 0x40c6c <entry+0xd6c>
    c.branch(C::Al,0x40c6c)?;
    // 0x40c68: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x40c6c: ldr r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    // 0x40c70: bic r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,4)?)?;
    // 0x40c74: mov r6, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,31744)?)?;
    // 0x40c78: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x40c7c: ldrb r3, [r4, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,2462)?)?;
    // 0x40c80: ldr r7, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,644)?)?;
    // 0x40c84: ldrb r12, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(4)?,988)?)?;
    // 0x40c88: cmp r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,32)?)?;
    // 0x40c8c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x40c90: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x40c94: cmp r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4096)?)?;
    // 0x40c98: mov r7, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,992)?)?;
    // 0x40c9c: orr r2, r2, r3, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x40ca0: and r3, r2, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x40ca4: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x40ca8: orreq r3, r2, #16
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(3)?,R::R2,16)?)?;
    // 0x40cac: ldr r2, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2744)?)?;
    // 0x40cb0: and r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x40cb4: ldr r3, [r2, #0x248]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,584)?)?;
    // 0x40cb8: and r3, r7, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x40cbc: ldr r7, [lr, #0x36c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::LR,876)?)?;
    // 0x40cc0: orr r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x40cc4: ldr r3, [lr, #0x368]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::LR,872)?)?;
    // 0x40cc8: and r3, r6, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x40ccc: mov r6, #3670016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3670016)?)?;
    // 0x40cd0: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x40cd4: mov r3, #491520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,491520)?)?;
    // 0x40cd8: and r3, r3, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,15,false)?)?;
    // 0x40cdc: ldr r7, [r2, #0x4ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,1260)?)?;
    // 0x40ce0: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x40ce4: ldr r3, [r2, #0x4e8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,1256)?)?;
    // 0x40ce8: and r3, r6, r3, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,19,false)?)?;
    // 0x40cec: orr r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x40cf0: mov r3, #130023424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,130023424)?)?;
    // 0x40cf4: and r3, r3, r7, lsl #22
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,22,false)?)?;
    // 0x40cf8: mov r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // 0x40cfc: orr r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x40d00: ldrb r3, [r2, #0x4f0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,1264)?)?;
    // 0x40d04: and r3, r7, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x40d08: and r7, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R12,1)?)?;
    // 0x40d0c: orr r3, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x40d10: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x40d14: orr r3, r3, r7, lsl #28
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,28,false)?)?;
    // 0x40d18: mov r7, #154
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,154)?)?;
    // 0x40d1c: orr r7, r7, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,1024)?)?;
    // 0x40d20: ldrsb r2, [r2, r7]
    c.emit(signed_byte_register(C::Al,R::R2,R::R2,R::new(7)?)?)?;
    // 0x40d24: and r7, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R2,255)?)?;
    // 0x40d28: cmp r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // 0x40d2c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x40d30: moveq r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x40d34: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x40d38: movgt r5, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x40d3c: orr r3, r3, r7, lsl #30
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,30,false)?)?;
    // 0x40d40: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x40d44: orr r2, r3, r5, lsl #31
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,31,false)?)?;
    // 0x40d48: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x40d4c: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x40d50: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x40d54: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x40d58: orr r3, r3, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,9216)?)?;
    // 0x40d5c: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x40d60: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x40d64: add r2, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,8)?)?;
    // 0x40d68: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x40d6c: ldrb r3, [lr, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::LR,1005)?)?;
    // 0x40d70: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x40d74: ldrbeq r3, [lr, #0x3ee]
    c.emit(arm32::transfer_byte(C::Eq,true,R::new(3)?,R::LR,1006)?)?;
    // 0x40d78: cmpeq r3, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x40d7c: beq 0x40dc0 <entry+0xec0>
    c.branch(C::Eq,0x40dc0)?;
    // 0x40d80: ldr r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    // 0x40d84: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x40d88: mov r4, #65024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,65024)?)?;
    // 0x40d8c: ldr r3, [r3, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,2744)?)?;
    // 0x40d90: ldr r7, [r3, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,24)?)?;
    // 0x40d94: ldr r3, [r3, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,28)?)?;
    // 0x40d98: sub r6, r5, r7, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::new(5)?,R::new(7)?,Shift::Lsr,2,false)?)?;
    // 0x40d9c: lsr r7, r7, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Lsr,2)?)?;
    // 0x40da0: add r7, r4, r7, lsl #9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(4)?,R::new(7)?,Shift::Lsl,9,false)?)?;
    // 0x40da4: and r6, r6, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,127)?)?;
    // 0x40da8: and r7, r7, #65024
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,65024)?)?;
    // 0x40dac: orr r6, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x40db0: sub r7, r5, r3, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(5)?,R::new(3)?,Shift::Lsr,2,false)?)?;
    // 0x40db4: mov r5, #4128768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4128768)?)?;
    // 0x40db8: and r7, r5, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(5)?,R::new(7)?,Shift::Lsl,16,false)?)?;
    // 0x40dbc: b 0x40dfc <entry+0xefc>
    c.branch(C::Al,0x40dfc)?;
    // 0x40dc0: ldr r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    // 0x40dc4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x40dc8: mov r4, #65024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,65024)?)?;
    // 0x40dcc: ldr r3, [r3, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,2744)?)?;
    // 0x40dd0: ldr r7, [r3, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,32)?)?;
    // 0x40dd4: ldr r3, [r3, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,36)?)?;
    // 0x40dd8: sub r5, r6, r7, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(5)?,R::new(6)?,R::new(7)?,Shift::Lsr,2,false)?)?;
    // 0x40ddc: lsr r7, r7, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Lsr,2)?)?;
    // 0x40de0: sub r6, r6, r3, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::new(6)?,R::new(3)?,Shift::Lsr,2,false)?)?;
    // 0x40de4: add r7, r4, r7, lsl #9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(4)?,R::new(7)?,Shift::Lsl,9,false)?)?;
    // 0x40de8: and r5, r5, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,127)?)?;
    // 0x40dec: and r7, r7, #65024
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,65024)?)?;
    // 0x40df0: orr r7, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x40df4: mov r5, #4128768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4128768)?)?;
    // 0x40df8: and r6, r5, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x40dfc: orr r7, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x40e00: mov r6, #-67108864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4227858432)?)?;
    // 0x40e04: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x40e08: and r3, r6, r3, lsl #24
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,24,false)?)?;
    // 0x40e0c: bic r7, r7, #-67108864
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(7)?,4227858432)?)?;
    // 0x40e10: add r3, r3, #-67108864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4227858432)?)?;
    // 0x40e14: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x40e18: ldreq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,0)?)?;
    // 0x40e1c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x40e20: streq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R2,0)?)?;
    // 0x40e24: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x40e28: add r7, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,16)?)?;
    // 0x40e2c: orr r2, r2, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,9216)?)?;
    // 0x40e30: str r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,8)?)?;
    // 0x40e34: str r3, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,12)?)?;
    // 0x40e38: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // 0x40e3c: ldr r2, [pc, #0x468]
    c.literal(R::R2,0x412ac)?;
    // 0x40e40: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x40e44: str r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,28)?)?;
    // 0x40e48: ldr r2, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2744)?)?;
    // 0x40e4c: ldrb r2, [r2, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,224)?)?;
    // 0x40e50: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x40e54: beq 0x40e84 <entry+0xf84>
    c.branch(C::Eq,0x40e84)?;
    // 0x40e58: mov r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // 0x40e5c: mov r2, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,25)?)?;
    // 0x40e60: mov r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // 0x40e64: strb r0, [sp, #0x20]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x40e68: strb r2, [sp, #0x21]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::SP,33)?)?;
    // 0x40e6c: mov r2, #30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,30)?)?;
    // 0x40e70: strb r2, [sp, #0x22]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::SP,34)?)?;
    // 0x40e74: mov r2, #38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,38)?)?;
    // 0x40e78: strb r2, [sp, #0x23]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::SP,35)?)?;
    // 0x40e7c: mov r2, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // 0x40e80: b 0x40f6c <entry+0x106c>
    c.branch(C::Al,0x40f6c)?;
    // 0x40e84: mov r0, #380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,380)?)?;
    // 0x40e88: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x40e8c: mov r7, #250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,250)?)?;
    // 0x40e90: mov r4, #51
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,51)?)?;
    // 0x40e94: ldrsb r0, [lr, r0]
    c.emit(signed_byte_register(C::Al,R::R0,R::LR,R::R0)?)?;
    // 0x40e98: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // 0x40e9c: addne r2, r2, #244
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,244)?)?;
    // 0x40ea0: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x40ea4: lsl r3, r2, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsl,24)?)?;
    // 0x40ea8: movne r4, #63
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,63)?)?;
    // 0x40eac: add r7, r7, r3, asr #24
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(3)?,Shift::Asr,24,false)?)?;
    // 0x40eb0: strb r7, [sp, #0x20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::SP,32)?)?;
    // 0x40eb4: strb r2, [sp, #0x21]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::SP,33)?)?;
    // 0x40eb8: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x40ebc: add r7, r0, #250
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,250)?)?;
    // 0x40ec0: add r2, r2, r3, asr #24
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Asr,24,false)?)?;
    // 0x40ec4: strb r7, [sp, #0x1c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::SP,28)?)?;
    // 0x40ec8: strb r0, [sp, #0x1d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::SP,29)?)?;
    // 0x40ecc: strb r2, [sp, #0x22]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::SP,34)?)?;
    // 0x40ed0: add r2, r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,6)?)?;
    // 0x40ed4: add r0, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,12)?)?;
    // 0x40ed8: strb r2, [sp, #0x1e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::SP,30)?)?;
    // 0x40edc: mov r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x40ee0: strb r0, [sp, #0x1f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::SP,31)?)?;
    // 0x40ee4: add r0, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,28)?)?;
    // 0x40ee8: add r2, r2, r3, asr #24
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Asr,24,false)?)?;
    // 0x40eec: orr r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // 0x40ef0: add r3, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::SP,32)?)?;
    // 0x40ef4: strb r2, [sp, #0x23]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::SP,35)?)?;
    // 0x40ef8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x40efc: b 0x40f08 <entry+0x1008>
    c.branch(C::Al,0x40f08)?;
    // 0x40f00: ldrb r7, [r0], #1
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::new(7)?,R::R0,1)?)?;
    // 0x40f04: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x40f08: lsl r6, r7, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(7)?,Shift::Lsl,24)?)?;
    // 0x40f0c: asr r5, r6, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(6)?,Shift::Asr,24)?)?;
    // 0x40f10: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x40f14: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x40f18: blt 0x40f2c <entry+0x102c>
    c.branch(C::Lt,0x40f2c)?;
    // 0x40f1c: cmp r5, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x40f20: movgt r7, r4
    c.emit(arm32::mov_register(C::Gt,R::new(7)?,R::new(4)?))?;
    // 0x40f24: lsl r7, r7, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Lsl,24)?)?;
    // 0x40f28: asr r6, r7, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(7)?,Shift::Asr,24)?)?;
    // 0x40f2c: ldrsb r7, [r3]
    c.emit(arm32::load_signed_byte(C::Al,R::new(7)?,R::new(3)?,0)?)?;
    // 0x40f30: strb r6, [r0, #-0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R0,-1)?)?;
    // 0x40f34: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x40f38: blt 0x40f48 <entry+0x1048>
    c.branch(C::Lt,0x40f48)?;
    // 0x40f3c: cmp r7, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x40f40: movgt r7, r4
    c.emit(arm32::mov_register(C::Gt,R::new(7)?,R::new(4)?))?;
    // 0x40f44: b 0x40f4c <entry+0x104c>
    c.branch(C::Al,0x40f4c)?;
    // 0x40f48: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x40f4c: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x40f50: strb r7, [r3]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x40f54: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // 0x40f58: bne 0x40f00 <entry+0x1000>
    c.branch(C::Ne,0x40f00)?;
    // 0x40f5c: ldr r7, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    // 0x40f60: ldrb r0, [sp, #0x1c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x40f64: ldrb r2, [sp, #0x1d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::SP,29)?)?;
    // 0x40f68: ldrb r3, [sp, #0x1e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::SP,30)?)?;
    // 0x40f6c: str lr, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,4)?)?;
    // 0x40f70: tst r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,31)?)?;
    // 0x40f74: orr r0, r0, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,8,false)?)?;
    // 0x40f78: mov r8, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,28)?)?;
    // 0x40f7c: add r9, r7, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(7)?,12)?)?;
    // 0x40f80: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x40f84: add r10, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,32)?)?;
    // 0x40f88: ldreq r2, [r7]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::new(7)?,0)?)?;
    // 0x40f8c: orr r0, r0, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x40f90: orr r8, r8, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,9216)?)?;
    // 0x40f94: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x40f98: streq r2, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(7)?,0)?)?;
    // 0x40f9c: mov r2, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,44)?)?;
    // 0x40fa0: orr r2, r2, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,9216)?)?;
    // 0x40fa4: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // 0x40fa8: str r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x40fac: add r0, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,8)?)?;
    // 0x40fb0: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x40fb4: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x40fb8: ldrsb r0, [r10]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::new(10)?,0)?)?;
    // 0x40fbc: mvn r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,6)?)?;
    // 0x40fc0: cmp r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x40fc4: blt 0x40fd4 <entry+0x10d4>
    c.branch(C::Lt,0x40fd4)?;
    // 0x40fc8: sub r1, r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,22)?)?;
    // 0x40fcc: cmp r0, #37
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,37)?)?;
    // 0x40fd0: movgt r1, #15
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x40fd4: mov r2, #152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,152)?)?;
    // 0x40fd8: ldr r3, [pc, #0x2d0]
    c.literal(R::new(3)?,0x412b0)?;
    // 0x40fdc: sub r12, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R12,R::new(9)?,4)?)?;
    // 0x40fe0: cmp r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,22)?)?;
    // 0x40fe4: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x40fe8: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // 0x40fec: str r12, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,24)?)?;
    // 0x40ff0: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x40ff4: mul r2, r1, r5
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::new(5)?)?)?;
    // 0x40ff8: smull r7, r6, r2, r3
    c.emit(signed_long_multiply(C::Al,R::new(7)?,R::new(6)?,R::R2,R::new(3)?)?)?;
    // 0x40ffc: mla r2, r1, r5, r6
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R1,R::new(5)?,R::new(6)?)?)?;
    // 0x41000: lsr r1, r2, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R2,Shift::Lsr,3)?)?;
    // 0x41004: add r1, r1, r2, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsr,31,false)?)?;
    // 0x41008: add r1, r1, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4100c: bge 0x41024 <entry+0x1124>
    c.branch(C::Ge,0x41024)?;
    // 0x41010: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x41014: mov r7, #83
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,83)?)?;
    // 0x41018: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x4101c: mov r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,44)?)?;
    // 0x41020: b 0x4106c <entry+0x116c>
    c.branch(C::Al,0x4106c)?;
    // 0x41024: sub r6, r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R0,22)?)?;
    // 0x41028: mov r2, #108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,108)?)?;
    // 0x4102c: mov r3, #83
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,83)?)?;
    // 0x41030: cmp r0, #37
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,37)?)?;
    // 0x41034: smulbb r2, r6, r2
    c.emit(signed_multiply_bottom_halves(C::Al,R::R2,R::new(6)?,R::R2)?)?;
    // 0x41038: add r7, r3, r2, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(3)?,R::R2,Shift::Lsr,4,false)?)?;
    // 0x4103c: movgt r7, #184
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(7)?,R::R0,184)?)?;
    // 0x41040: ble 0x41054 <entry+0x1154>
    c.branch(C::Le,0x41054)?;
    // 0x41044: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x41048: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x4104c: mov r0, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,21)?)?;
    // 0x41050: b 0x4106c <entry+0x116c>
    c.branch(C::Al,0x4106c)?;
    // 0x41054: ldr r2, [pc, #0x258]
    c.literal(R::R2,0x412b4)?;
    // 0x41058: mov r3, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,25)?)?;
    // 0x4105c: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x41060: smlabb r0, r0, r3, r2
    c.emit(signed_multiply_add_bottom_halves(C::Al,R::R0,R::R0,R::new(3)?,R::R2)?)?;
    // 0x41064: mov r2, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,44)?)?;
    // 0x41068: sub r0, r2, r0, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R2,R::R0,Shift::Lsr,4,false)?)?;
    // 0x4106c: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x41070: orr r2, r2, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3840)?)?;
    // 0x41074: and r4, r2, r1, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R2,R::R1,Shift::Lsr,4,false)?)?;
    // 0x41078: blx 0x82ed0 <entry+0x42fd0>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x82ed0)?)?;
    // 0x4107c: mov r6, #266338304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,266338304)?)?;
    // 0x41080: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x41084: orr r6, r6, #805306368
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,805306368)?)?;
    // 0x41088: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x4108c: blx 0x818a0 <entry+0x419a0>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x818a0)?)?;
    // 0x41090: blx 0x82d30 <entry+0x42e30>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x82d30)?)?;
    // 0x41094: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x41098: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x4109c: blx 0x82ed0 <entry+0x42fd0>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x82ed0)?)?;
    // 0x410a0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x410a4: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x410a8: blx 0x818a0 <entry+0x419a0>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x818a0)?)?;
    // 0x410ac: blx 0x82d30 <entry+0x42e30>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x82d30)?)?;
    // 0x410b0: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x410b4: lsl r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,16)?)?;
    // 0x410b8: orr r5, r0, r5, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,24,false)?)?;
    // 0x410bc: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // 0x410c0: add r0, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x410c4: blx 0x82ed0 <entry+0x42fd0>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x82ed0)?)?;
    // 0x410c8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x410cc: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x410d0: blx 0x818a0 <entry+0x419a0>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x818a0)?)?;
    // 0x410d4: blx 0x82d30 <entry+0x42e30>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x82d30)?)?;
    // 0x410d8: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // 0x410dc: orr r0, r5, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,8,false)?)?;
    // 0x410e0: sub r1, r8, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(8)?,16)?)?;
    // 0x410e4: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x410e8: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // 0x410ec: orr r0, r0, #38
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,38)?)?;
    // 0x410f0: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // 0x410f4: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x410f8: ldreq r2, [r9, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::new(9)?,-4)?)?;
    // 0x410fc: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x41100: streq r2, [r9, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(9)?,-4)?)?;
    // 0x41104: str r1, [r9, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,-4)?)?;
    // 0x41108: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x4110c: add r1, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,4)?)?;
    // 0x41110: ldr r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,8)?)?;
    // 0x41114: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // 0x41118: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x4111c: ldr r0, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2744)?)?;
    // 0x41120: ldr r2, [r0, #0x334]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,820)?)?;
    // 0x41124: ldr r0, [r0, #0x338]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,824)?)?;
    // 0x41128: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4112c: lsreq r2, r4, #2
    c.emit(arm32::mov_shifted_register(C::Eq,R::R2,R::new(4)?,Shift::Lsr,2)?)?;
    // 0x41130: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x41134: moveq r0, r4
    c.emit(arm32::mov_register(C::Eq,R::R0,R::new(4)?))?;
    // 0x41138: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4113c: ldr r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,20)?)?;
    // 0x41140: ldreq r1, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(9)?,4)?)?;
    // 0x41144: and r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41148: orr r0, r0, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,16,false)?)?;
    // 0x4114c: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x41150: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x41154: streq r1, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(9)?,4)?)?;
    // 0x41158: str r8, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,4)?)?;
    // 0x4115c: str r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,8)?)?;
    // 0x41160: add r0, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,12)?)?;
    // 0x41164: add r8, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,4)?)?;
    // 0x41168: add r9, r9, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,16)?)?;
    // 0x4116c: cmp r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,4)?)?;
    // 0x41170: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x41174: bne 0x40fb8 <entry+0x10b8>
    c.branch(C::Ne,0x40fb8)?;
    // 0x41178: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // 0x4117c: sub lr, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::LR,R::new(9)?,4)?)?;
    // 0x41180: ldr r0, [r1, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,400)?)?;
    // 0x41184: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x41188: bne 0x4119c <entry+0x129c>
    c.branch(C::Ne,0x4119c)?;
    // 0x4118c: ldrb r0, [r1, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,1005)?)?;
    // 0x41190: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x41194: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x41198: beq 0x4127c <entry+0x137c>
    c.branch(C::Eq,0x4127c)?;
    // 0x4119c: ldr r7, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,12)?)?;
    // 0x411a0: mov r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,2)?)?;
    // 0x411a4: mov r8, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,48)?)?;
    // 0x411a8: mov r4, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,192)?)?;
    // 0x411ac: mov r12, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,12)?)?;
    // 0x411b0: ldr r1, [r7, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,2744)?)?;
    // 0x411b4: ldr r0, [r1, #0x28c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,652)?)?;
    // 0x411b8: ldr r3, [r1, #0x290]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,656)?)?;
    // 0x411bc: add r6, r1, #660
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,660)?)?;
    // 0x411c0: and r2, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,3)?)?;
    // 0x411c4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x411c8: ldr r0, [r7, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,644)?)?;
    // 0x411cc: movgt r2, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x411d0: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x411d4: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x411d8: movle r10, r3
    c.emit(arm32::mov_register(C::Le,R::new(10)?,R::new(3)?))?;
    // 0x411dc: ldm r6, {r0, r3, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(6)?,R::R0.bit() | R::new(3)?.bit() | R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x411e0: and r5, r8, r0, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(8)?,R::R0,Shift::Lsl,4,false)?)?;
    // 0x411e4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x411e8: and r0, r4, r3, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::new(3)?,Shift::Lsl,6,false)?)?;
    // 0x411ec: movgt r5, #32
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(5)?,R::R0,32)?)?;
    // 0x411f0: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x411f4: mov r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,768)?)?;
    // 0x411f8: movgt r0, #128
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R0,R::R0,128)?)?;
    // 0x411fc: and r7, r3, r6, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(3)?,R::new(6)?,Shift::Lsl,8,false)?)?;
    // 0x41200: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x41204: ldrb r6, [r1, #0x51c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R1,1308)?)?;
    // 0x41208: movgt r7, #512
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(7)?,R::R0,512)?)?;
    // 0x4120c: cmp r6, #19
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,19)?)?;
    // 0x41210: bne 0x41240 <entry+0x1340>
    c.branch(C::Ne,0x41240)?;
    // 0x41214: ldr r0, [r1, #0x524]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,1316)?)?;
    // 0x41218: and r2, r8, r0, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(8)?,R::R0,Shift::Lsl,4,false)?)?;
    // 0x4121c: and r5, r4, r0, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(4)?,R::R0,Shift::Lsl,6,false)?)?;
    // 0x41220: and r3, r3, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,8,false)?)?;
    // 0x41224: orr r2, r5, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x41228: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4122c: and r3, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R0,3)?)?;
    // 0x41230: and r0, r12, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R12,R::R0,Shift::Lsl,2,false)?)?;
    // 0x41234: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41238: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4123c: b 0x41254 <entry+0x1354>
    c.branch(C::Al,0x41254)?;
    // 0x41240: and r3, r12, r10, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R12,R::new(10)?,Shift::Lsl,2,false)?)?;
    // 0x41244: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41248: orr r2, r5, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4124c: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x41250: orr r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x41254: ldr r2, [r1, #0x2a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,672)?)?;
    // 0x41258: ldr r1, [r1, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,676)?)?;
    // 0x4125c: mov r3, #245760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,245760)?)?;
    // 0x41260: ldr r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,8)?)?;
    // 0x41264: bic r0, r0, #4177920
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,4177920)?)?;
    // 0x41268: and r1, r3, r1, lsl #14
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,14,false)?)?;
    // 0x4126c: mov r3, #3932160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3932160)?)?;
    // 0x41270: and r2, r3, r2, lsl #18
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,18,false)?)?;
    // 0x41274: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x41278: orr r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4127c: tst lr, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::LR,31)?)?;
    // 0x41280: ldreq r0, [r9, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(9)?,-4)?)?;
    // 0x41284: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x41288: streq r0, [r9, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,-4)?)?;
    // 0x4128c: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // 0x41290: orr r0, r0, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,9216)?)?;
    // 0x41294: stmda r9, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(9)?,R::R0.bit() | R::R1.bit(),BlockMode::DecrementAfter,false)?)?;
    // 0x41298: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x4129c: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x412a0: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x412a4: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x412a8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(CODE_END)?;
    for word in CONTROLLER_DATA {c.emit(word)?;}
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();write_controller_packet(&mut c)?;Ok(c.bytes)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]
    fn codec_controller_is_contiguous_with_named_data(){
        let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
        assert_eq!(CODE_END-START,422*4);
        for (i,&v) in CONTROLLER_DATA.iter().enumerate(){
            let at=CODE_END-START+i*4;
            assert_eq!(u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()),v);
        }
    }
    #[test]
    fn signed_dsp_encoders_match_independent_arm_assembler(){
        assert_eq!(signed_long_multiply(C::Al,R::new(7).unwrap(),R::new(6).unwrap(),R::R2,R::new(3).unwrap()).unwrap(),0xe0c6_7392);
        assert_eq!(signed_multiply_bottom_halves(C::Al,R::R2,R::new(6).unwrap(),R::R2).unwrap(),0xe162_0286);
        assert_eq!(signed_multiply_add_bottom_halves(C::Al,R::R0,R::R0,R::new(3).unwrap(),R::R2).unwrap(),0xe100_2380);
        assert!(signed_long_multiply(C::Al,R::R0,R::R0,R::R1,R::R2).is_err());
    }
}
