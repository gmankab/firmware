// SPDX-License-Identifier: GPL-3.0-only
//! Venus A32 controller format continuation `0x45000..0x45800`.
//! Writes controller plane registers, code types and input capabilities.
//! The single word at `0x4509c` is a non-executable inline literal,
//! skipped by predecessor branches and therefore not a coprocessor opcode.
//! All instructions are assembled from typed ARM operand encoders; the
//! enclosing controller function is not yet complete.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
pub const START:usize=0x45000;
pub const END:usize=0x45800;
const CONTROL_REFERENCE:u32=0xfe0f_83e0;
struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
    fn check(&self,a:usize)->Result<(),String>{
        if self.at!=a{return Err(format!("controller format extension at 0x{:x}; expected 0x{a:x}",self.at));}
        Ok(())
    }
    #[inline(never)]
    fn emit(&mut self,w:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at)||self.at%4!=0{return Err("Agent 04 ownership violation".into());}
        self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
    }
    #[inline(never)]
    fn branch(&mut self,cond:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(cond,self.at as u32,target as u32,false)?)
    }
}

/// Finish the first register packet and choose a controller mode.
#[inline(never)]
fn finish_main_packet(c:&mut Code)->Result<(),String>{
    c.check(0x45000)?;
    // 0x45000: orr	r2, r2, #1056964608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1056964608)?)?;
    // 0x45004: and	r1, r2, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,16,false)?)?;
    // 0x45008: orr	r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4500c: ldr	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x45010: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x45014: ldreq	r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x45018: andeq	r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4501c: streq	r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x45020: mov	r2, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6144)?)?;
    // 0x45024: str	r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x45028: str	r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x4502c: ldr	r8, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-36)?)?;
    // 0x45030: add	r2, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,1)?)?;
    // 0x45034: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x45038: add	r2, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,8)?)?;
    // 0x4503c: str	r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x45040: ldr	r7, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,644)?)?;
    // 0x45044: cmp	r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x45048: beq	0x45090 <entry+0x1540>  @ imm = #0x40
    c.branch(C::Eq,0x45090)?;
    // 0x4504c: cmp	r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x45050: beq	0x450a0 <entry+0x1550>  @ imm = #0x48
    c.branch(C::Eq,0x450a0)?;
    // 0x45054: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x45058: cmp	r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4096)?)?;
    // 0x4505c: bne	0x450b0 <entry+0x1560>  @ imm = #0x4c
    c.branch(C::Ne,0x450b0)?;
    // 0x45060: ldrb	r7, [r10, #0x338]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,824)?)?;
    // 0x45064: mov	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x45068: mov	r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x4506c: cmp	r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x45070: moveq	r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x45074: cmp	r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // 0x45078: lsl	r5, r3, #12
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(3)?,Shift::Lsl,12)?)?;
    // 0x4507c: moveq	r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x45080: orr	r3, r5, r3, lsl #19
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,19,false)?)?;
    // 0x45084: lsl	r7, r6, #9
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(6)?,Shift::Lsl,9)?)?;
    // 0x45088: orr	r3, r3, #112
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,112)?)?;
    // 0x4508c: b	0x450ac <entry+0x155c>  @ imm = #0x18
    c.branch(C::Al,0x450ac)?;
    // 0x45090: mov	r3, #3670016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3670016)?)?;
    // 0x45094: mov	r7, #28672
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,28672)?)?;
    // 0x45098: b	0x450ac <entry+0x155c>  @ imm = #0xc
    c.branch(C::Al,0x450ac)?;
    c.check(0x4509c)
}

/// Build the second codec parameter packet from selected descriptors.
#[inline(never)]
fn build_second_packet(c:&mut Code)->Result<(),String>{
    c.check(0x450a0)?;
    // 0x450a0: mov	r3, #144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,144)?)?;
    // 0x450a4: mov	r7, #24576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,24576)?)?;
    // 0x450a8: orr	r3, r3, #3145728
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,3145728)?)?;
    // 0x450ac: orr	r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x450b0: ldr	r7, [pc, #0xe5c]        @ 0x23c4 <entry+0x23c4>
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::PC,3676)?)?;
    // 0x450b4: mov	r5, #2013265920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2013265920)?)?;
    // 0x450b8: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x450bc: add	r4, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,16)?)?;
    // 0x450c0: mov	r12, #983040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,983040)?)?;
    // 0x450c4: and	r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x450c8: ldr	r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x450cc: ldr	r6, [r7, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,180)?)?;
    // 0x450d0: ldr	r7, [r7, #0xb8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,184)?)?;
    // 0x450d4: and	r6, r5, r6, lsl #27
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,27,false)?)?;
    // 0x450d8: orr	r3, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x450dc: mov	r6, #125829120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,125829120)?)?;
    // 0x450e0: and	r7, r6, r7, lsl #23
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,23,false)?)?;
    // 0x450e4: orr	r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x450e8: ldreq	r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,0)?)?;
    // 0x450ec: andeq	r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x450f0: streq	r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R2,0)?)?;
    // 0x450f4: mov	r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x450f8: tst	r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x450fc: orr	r2, r2, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,6144)?)?;
    // 0x45100: str	r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,8)?)?;
    // 0x45104: add	r2, r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,2)?)?;
    // 0x45108: str	r3, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,12)?)?;
    // 0x4510c: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x45110: str	r4, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x45114: ldr	r2, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2744)?)?;
    // 0x45118: ldr	r3, [r2, #0x468]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,1128)?)?;
    // 0x4511c: ldr	r6, [r2, #0x46c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,1132)?)?;
    // 0x45120: ldr	r5, [r2, #0x470]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,1136)?)?;
    // 0x45124: ldr	r7, [r2, #0x474]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,1140)?)?;
    // 0x45128: ldr	r1, [r2, #0x478]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,1144)?)?;
    // 0x4512c: ldr	r2, [r2, #0x47c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,1148)?)?;
    // 0x45130: and	r2, r12, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R12,R::R2,Shift::Lsl,16,false)?)?;
    // 0x45134: mov	r12, #15728640
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,15728640)?)?;
    // 0x45138: and	r1, r12, r1, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R12,R::R1,Shift::Lsl,20,false)?)?;
    // 0x4513c: orr	r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x45140: mov	r2, #251658240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,251658240)?)?;
    // 0x45144: and	r2, r2, r7, lsl #24
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,24,false)?)?;
    // 0x45148: orr	r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4514c: and	r2, r5, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(5)?,15)?)?;
    // 0x45150: mov	r5, #983040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,983040)?)?;
    // 0x45154: orr	r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x45158: mov	r2, #240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,240)?)?;
    // 0x4515c: and	r2, r2, r6, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsl,4,false)?)?;
    // 0x45160: mov	r6, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3840)?)?;
    // 0x45164: orr	r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x45168: and	r2, r6, r3, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x4516c: orr	r2, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x45170: ldreq	r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(4)?,0)?)?;
    // 0x45174: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x45178: streq	r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,0)?)?;
    // 0x4517c: mov	r1, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,44)?)?;
    // 0x45180: add	r4, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,24)?)?;
    // 0x45184: orr	r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x45188: tst	r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x4518c: str	r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x45190: add	r1, r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,3)?)?;
    // 0x45194: str	r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,20)?)?;
    // 0x45198: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4519c: str	r4, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x451a0: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x451a4: add	r12, r1, #1152
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R1,1152)?)?;
    // 0x451a8: ldm	r12, {r2, r3, r7, r9, r12}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit() | R::new(9)?.bit() | R::R12.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x451ac: ldr	r1, [r1, #0x494]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,1172)?)?;
    // 0x451b0: and	r1, r5, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(5)?,R::R1,Shift::Lsl,16,false)?)?;
    // 0x451b4: mov	r5, #15728640
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,15728640)?)?;
    // 0x451b8: and	r7, r7, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,15)?)?;
    // 0x451bc: and	r2, r6, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,8,false)?)?;
    // 0x451c0: and	r5, r5, r12, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::R12,Shift::Lsl,20,false)?)?;
    // 0x451c4: orr	r1, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x451c8: mov	r5, #251658240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,251658240)?)?;
    // 0x451cc: and	r5, r5, r9, lsl #24
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(9)?,Shift::Lsl,24,false)?)?;
    // 0x451d0: orr	r1, r1, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x451d4: orr	r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x451d8: mov	r7, #240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,240)?)?;
    // 0x451dc: and	r3, r7, r3, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,4,false)?)?;
    // 0x451e0: orr	r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x451e4: orr	r2, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x451e8: ldreq	r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(4)?,0)?)?;
    // 0x451ec: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x451f0: streq	r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,0)?)?;
    // 0x451f4: mov	r1, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2096)?)?;
    // 0x451f8: orr	r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x451fc: str	r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x45200: str	r2, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,28)?)?;
    // 0x45204: add	r1, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,4)?)?;
    // 0x45208: add	r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // 0x4520c: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x45210: str	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x45214: ldr	r1, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,644)?)?;
    // 0x45218: cmp	r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x4521c: beq	0x45288 <entry+0x1738>  @ imm = #0x64
    c.branch(C::Eq,0x45288)?;
    // 0x45220: cmp	r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x45224: beq	0x452f0 <entry+0x17a0>  @ imm = #0xc4
    c.branch(C::Eq,0x452f0)?;
    // 0x45228: ldr	r8, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,80)?)?;
    // 0x4522c: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45230: cmp	r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x45234: bne	0x4530c <entry+0x17bc>  @ imm = #0xd0
    c.branch(C::Ne,0x4530c)?;
    // 0x45238: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x4523c: ldr	r2, [pc, #0xe74]        @ 0x2568 <entry+0x2568>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,3700)?)?;
    // 0x45240: mov	r1, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,96)?)?;
    // 0x45244: orr	r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x45248: add	r7, r0, #188
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,188)?)?;
    // 0x4524c: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45250: ldm	r7, {r4, r5, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(7)?,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x45254: tst	r7, #224
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,224)?)?;
    // 0x45258: moveq	r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4525c: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x43ab4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45260: mov	r1, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16711680)?)?;
    // 0x45264: mov	r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,65280)?)?;
    // 0x45268: ldr	lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x4526c: and	r1, r1, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x45270: and	r0, r0, r6, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsl,8,false)?)?;
    // 0x45274: orr	r1, r1, r4, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(4)?,Shift::Lsl,24,false)?)?;
    // 0x45278: orr	r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4527c: and	r1, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(7)?,255)?)?;
    c.check(0x45280)
}

/// Collect device capability bitfields and expected format settings.
#[inline(never)]
fn derive_capability_bundles(c:&mut Code)->Result<(),String>{
    c.check(0x45280)?;
    // 0x45280: orr	r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x45284: b	0x4530c <entry+0x17bc>  @ imm = #0x80
    c.branch(C::Al,0x4530c)?;
    // 0x45288: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x4528c: ldr	r8, [pc, #0xe24]        @ 0x2568 <entry+0x2568>
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::PC,3620)?)?;
    // 0x45290: mov	r1, #102
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,102)?)?;
    // 0x45294: mov	r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x45298: orr	r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x4529c: add	r7, r0, #192
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,192)?)?;
    // 0x452a0: mov	r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x452a4: ldm	r7, {r5, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(7)?,R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x452a8: bic	r0, r4, r7, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R0,R::new(4)?,R::new(7)?,Shift::Lsr,1,false)?)?;
    // 0x452ac: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x43b04
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x452b0: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x452b4: mov	r1, #103
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,103)?)?;
    // 0x452b8: mov	r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x452bc: orr	r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x452c0: ldr	r0, [r0, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,196)?)?;
    // 0x452c4: bic	r0, r4, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R0,R::new(4)?,R::R0,Shift::Lsr,1,false)?)?;
    // 0x452c8: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x43b20
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x452cc: mov	r0, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16711680)?)?;
    // 0x452d0: mov	r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,65280)?)?;
    // 0x452d4: ldr	lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x452d8: and	r0, r0, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x452dc: and	r1, r1, r6, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsl,8,false)?)?;
    // 0x452e0: orr	r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x452e4: and	r1, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(7)?,255)?)?;
    // 0x452e8: orr	r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x452ec: b	0x45308 <entry+0x17b8>  @ imm = #0x14
    c.branch(C::Al,0x45308)?;
    // 0x452f0: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x452f4: mov	r2, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16711680)?)?;
    // 0x452f8: ldr	r1, [r0, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,192)?)?;
    // 0x452fc: ldrb	r0, [r0, #0xc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,200)?)?;
    // 0x45300: and	r1, r2, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,16,false)?)?;
    // 0x45304: orr	r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x45308: ldr	r8, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,80)?)?;
    // 0x4530c: ldr	r2, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::LR,0)?)?;
    // 0x45310: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x45314: ldreq	r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x45318: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4531c: streq	r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x45320: mov	r1, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,52)?)?;
    // 0x45324: orr	r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x45328: str	r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // 0x4532c: str	r0, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,4)?)?;
    // 0x45330: add	r0, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,8)?)?;
    // 0x45334: ldr	r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // 0x45338: add	r12, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(3)?,1)?)?;
    // 0x4533c: str	r12, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R11,-36)?)?;
    // 0x45340: str	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x45344: ldr	r7, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,644)?)?;
    // 0x45348: cmp	r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x4534c: cmpne	r7, #16384
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(7)?,16384)?)?;
    // 0x45350: beq	0x4535c <entry+0x180c>  @ imm = #0x4
    c.branch(C::Eq,0x4535c)?;
    // 0x45354: cmp	r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4096)?)?;
    // 0x45358: bne	0x453cc <entry+0x187c>  @ imm = #0x6c
    c.branch(C::Ne,0x453cc)?;
    // 0x4535c: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x45360: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x45364: add	r12, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(3)?,2)?)?;
    // 0x45368: ldrb	r6, [r1, #0x49d]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R1,1181)?)?;
    // 0x4536c: ldrb	r7, [r1, #0x49c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R1,1180)?)?;
    // 0x45370: ldrb	r5, [r1, #0x49e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R1,1182)?)?;
    // 0x45374: ldrb	r4, [r1, #0x49f]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R1,1183)?)?;
    // 0x45378: ldrb	r1, [r1, #0x4a0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,1184)?)?;
    // 0x4537c: and	r6, r6, #63
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,63)?)?;
    // 0x45380: and	r1, r1, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,127)?)?;
    // 0x45384: lsl	r6, r6, #21
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(6)?,Shift::Lsl,21)?)?;
    // 0x45388: orr	r7, r6, r7, lsl #27
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,27,false)?)?;
    // 0x4538c: and	r6, r5, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(5)?,127)?)?;
    // 0x45390: orr	r7, r7, r6, lsl #14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,14,false)?)?;
    // 0x45394: and	r6, r4, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(4)?,127)?)?;
    // 0x45398: orr	r7, r7, r6, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,7,false)?)?;
    // 0x4539c: orr	r1, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x453a0: ldreq	r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R0,0)?)?;
    // 0x453a4: andeq	r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x453a8: streq	r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R0,0)?)?;
    // 0x453ac: mov	r0, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,104)?)?;
    // 0x453b0: orr	r0, r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // 0x453b4: str	r0, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,8)?)?;
    // 0x453b8: add	r0, r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,16)?)?;
    // 0x453bc: str	r1, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,12)?)?;
    // 0x453c0: str	r12, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R11,-36)?)?;
    // 0x453c4: str	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x453c8: ldr	r7, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,644)?)?;
    // 0x453cc: cmp	r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x453d0: mov	r2, #300
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,300)?)?;
    // 0x453d4: cmpne	r7, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(7)?,4096)?)?;
    // 0x453d8: beq	0x453f4 <entry+0x18a4>  @ imm = #0x14
    c.branch(C::Eq,0x453f4)?;
    // 0x453dc: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x453e0: mov	r2, #1200
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1200)?)?;
    // 0x453e4: ldr	r1, [r1, #0x39c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,924)?)?;
    // 0x453e8: lsr	r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,2)?)?;
    // 0x453ec: mul	r3, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // 0x453f0: and	r2, r3, #4080
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(3)?,4080)?)?;
    // 0x453f4: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x453f8: add	r3, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,16)?)?;
    // 0x453fc: ldreq	r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x45400: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x45404: streq	r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x45408: mov	r1, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,132)?)?;
    // 0x4540c: orr	r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x45410: stm	r0, {r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit() | R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x45414: add	r1, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,1)?)?;
    // 0x45418: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4541c: add	r1, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,8)?)?;
    // 0x45420: str	r1, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::LR,0)?)?;
    // 0x45424: tst	r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x45428: ldreq	r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x4542c: andeq	r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x45430: streq	r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x45434: mov	r1, #2160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2160)?)?;
    // 0x45438: mov	r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4543c: orr	r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x45440: str	r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x45444: add	r1, r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,2)?)?;
    // 0x45448: str	r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x4544c: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x45450: str	r3, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::LR,0)?)?;
    // 0x45454: ldr	r2, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,400)?)?;
    // 0x45458: cmp	r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4545c: beq	0x45498 <entry+0x1948>  @ imm = #0x34
    c.branch(C::Eq,0x45498)?;
    // 0x45460: cmp	r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x45464: beq	0x454b0 <entry+0x1960>  @ imm = #0x44
    c.branch(C::Eq,0x454b0)?;
    // 0x45468: mov	r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x4546c: cmp	r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x45470: mov	r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x45474: mov	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x45478: mov	r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x4547c: bne	0x454d4 <entry+0x1984>  @ imm = #0x50
    c.branch(C::Ne,0x454d4)?;
    // 0x45480: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x45484: add	r7, r1, #876
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R1,876)?)?;
    // 0x45488: add	r6, r1, #872
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,872)?)?;
    // 0x4548c: add	r5, r1, #868
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R1,868)?)?;
    // 0x45490: add	r4, r1, #864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R1,864)?)?;
    // 0x45494: b	0x454c4 <entry+0x1974>  @ imm = #0x28
    c.branch(C::Al,0x454c4)?;
    // 0x45498: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x4549c: add	r7, r1, #844
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R1,844)?)?;
    // 0x454a0: add	r6, r1, #840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,840)?)?;
    // 0x454a4: add	r5, r1, #836
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R1,836)?)?;
    // 0x454a8: add	r4, r1, #832
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R1,832)?)?;
    // 0x454ac: b	0x454c4 <entry+0x1974>  @ imm = #0x10
    c.branch(C::Al,0x454c4)?;
    // 0x454b0: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x454b4: add	r7, r1, #860
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R1,860)?)?;
    // 0x454b8: add	r6, r1, #856
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,856)?)?;
    // 0x454bc: add	r5, r1, #852
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R1,852)?)?;
    // 0x454c0: add	r4, r1, #848
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R1,848)?)?;
    // 0x454c4: ldr	r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x454c8: ldr	r7, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,0)?)?;
    // 0x454cc: ldr	r6, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(6)?,0)?)?;
    // 0x454d0: ldrb	r5, [r4]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,0)?)?;
    // 0x454d4: mov	r4, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,65280)?)?;
    // 0x454d8: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x454dc: and	r1, r4, r1, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::R1,Shift::Lsl,8,false)?)?;
    // 0x454e0: mov	r4, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,16711680)?)?;
    // 0x454e4: and	r6, r4, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(4)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x454e8: orr	r7, r6, r7, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,24,false)?)?;
    // 0x454ec: orr	r1, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x454f0: ldreq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x454f4: orr	r1, r1, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x454f8: andeq	r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x454fc: streq	r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    c.check(0x45500)
}

/// Apply format-dependent register words and mode-specific flags.
#[inline(never)]
fn update_codec_fields(c:&mut Code)->Result<(),String>{
    c.check(0x45500)?;
    // 0x45500: mov	r3, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2144)?)?;
    // 0x45504: orr	r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x45508: str	r3, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,16)?)?;
    // 0x4550c: str	r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,20)?)?;
    // 0x45510: add	r1, r12, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,3)?)?;
    // 0x45514: add	r3, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,24)?)?;
    // 0x45518: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4551c: str	r3, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::LR,0)?)?;
    // 0x45520: ldr	r1, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,644)?)?;
    // 0x45524: cmp	r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // 0x45528: cmpne	r1, #8192
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x4552c: bne	0x455e0 <entry+0x1a90>  @ imm = #0xac
    c.branch(C::Ne,0x455e0)?;
    // 0x45530: cmp	r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x45534: beq	0x45564 <entry+0x1a14>  @ imm = #0x28
    c.branch(C::Eq,0x45564)?;
    // 0x45538: cmp	r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x4553c: beq	0x4557c <entry+0x1a2c>  @ imm = #0x38
    c.branch(C::Eq,0x4557c)?;
    // 0x45540: mov	r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x45544: cmp	r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x45548: bne	0x455b0 <entry+0x1a60>  @ imm = #0x60
    c.branch(C::Ne,0x455b0)?;
    // 0x4554c: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x45550: mov	r5, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,992)?)?;
    // 0x45554: ldr	r7, [r1, #0x378]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,888)?)?;
    // 0x45558: ldr	r6, [r1, #0x37c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,892)?)?;
    // 0x4555c: add	r1, r1, #896
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,896)?)?;
    // 0x45560: b	0x45590 <entry+0x1a40>  @ imm = #0x28
    c.branch(C::Al,0x45590)?;
    // 0x45564: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x45568: mov	r5, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,992)?)?;
    // 0x4556c: ldr	r7, [r1, #0x390]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,912)?)?;
    // 0x45570: ldr	r6, [r1, #0x394]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,916)?)?;
    // 0x45574: add	r1, r1, #920
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,920)?)?;
    // 0x45578: b	0x45590 <entry+0x1a40>  @ imm = #0x10
    c.branch(C::Al,0x45590)?;
    // 0x4557c: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x45580: mov	r5, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,992)?)?;
    // 0x45584: ldr	r7, [r1, #0x384]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,900)?)?;
    // 0x45588: ldr	r6, [r1, #0x388]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,904)?)?;
    // 0x4558c: add	r1, r1, #908
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,908)?)?;
    // 0x45590: and	r6, r5, r6, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,5,false)?)?;
    // 0x45594: and	r7, r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,31)?)?;
    // 0x45598: orr	r7, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x4559c: ldr	r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x455a0: mov	r6, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,31744)?)?;
    // 0x455a4: bic	r7, r7, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(7)?,31744)?)?;
    // 0x455a8: and	r1, r6, r1, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(6)?,R::R1,Shift::Lsl,10,false)?)?;
    // 0x455ac: orr	r7, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x455b0: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x455b4: ldreq	r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(3)?,0)?)?;
    // 0x455b8: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x455bc: streq	r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(3)?,0)?)?;
    // 0x455c0: mov	r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,100)?)?;
    // 0x455c4: orr	r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x455c8: str	r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x455cc: str	r7, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,28)?)?;
    // 0x455d0: add	r1, r12, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,4)?)?;
    // 0x455d4: add	r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // 0x455d8: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x455dc: str	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x455e0: cmp	r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // 0x455e4: bhs	0x455f0 <entry+0x1aa0>  @ imm = #0x4
    c.branch(C::Cs,0x455f0)?;
    // 0x455e8: lsl	r4, r2, #22
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R2,Shift::Lsl,22)?)?;
    // 0x455ec: b	0x4560c <entry+0x1abc>  @ imm = #0x18
    c.branch(C::Al,0x4560c)?;
    // 0x455f0: ldr	r2, [pc, #0xac0]        @ 0x2568 <entry+0x2568>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2752)?)?;
    // 0x455f4: mov	r1, #189
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,189)?)?;
    // 0x455f8: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x455fc: mov	r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x45600: orr	r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x45604: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x43e5c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45608: ldr	lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x4560c: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x45610: mov	r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // 0x45614: ldr	r2, [r1, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,176)?)?;
    // 0x45618: ldr	r0, [r1, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,172)?)?;
    // 0x4561c: and	r2, r3, r2, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,5,false)?)?;
    // 0x45620: mov	r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2048)?)?;
    // 0x45624: and	r0, r3, r0, lsl #11
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,11,false)?)?;
    // 0x45628: orr	r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4562c: orr	r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x45630: ldr	r2, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,644)?)?;
    // 0x45634: orr	r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // 0x45638: cmp	r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x4563c: beq	0x45660 <entry+0x1b10>  @ imm = #0x1c
    c.branch(C::Eq,0x45660)?;
    // 0x45640: ldr	r1, [r1, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,52)?)?;
    // 0x45644: mov	r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x45648: cmp	r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16384)?)?;
    // 0x4564c: and	r1, r3, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,2,false)?)?;
    // 0x45650: orr	r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x45654: bne	0x45660 <entry+0x1b10>  @ imm = #0x4
    c.branch(C::Ne,0x45660)?;
    // 0x45658: orr	r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3)?)?;
    // 0x4565c: b	0x45678 <entry+0x1b28>  @ imm = #0x14
    c.branch(C::Al,0x45678)?;
    // 0x45660: ldrb	r1, [r10, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,2462)?)?;
    // 0x45664: mov	r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x45668: bic	r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,3)?)?;
    // 0x4566c: cmp	r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,32)?)?;
    // 0x45670: moveq	r2, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x45674: orr	r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x45678: ldrb	r1, [r10, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,988)?)?;
    // 0x4567c: bic	r0, r0, #4128768
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,4128768)?)?;
    // 0x45680: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x45684: movne	r1, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x45688: orr	r0, r0, r1, lsl #19
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,19,false)?)?;
    // 0x4568c: orr	r0, r0, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,16,false)?)?;
    // 0x45690: ldr	r1, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::LR,0)?)?;
    // 0x45694: tst	r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x45698: ldreq	r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x4569c: andeq	r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x456a0: streq	r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x456a4: mov	r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x456a8: orr	r2, r2, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,6144)?)?;
    // 0x456ac: str	r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x456b0: str	r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x456b4: ldr	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x456b8: add	r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1)?)?;
    // 0x456bc: str	r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x456c0: add	r0, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,8)?)?;
    // 0x456c4: str	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x456c8: ldr	r3, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,644)?)?;
    // 0x456cc: cmp	r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4096)?)?;
    // 0x456d0: bne	0x45794 <entry+0x1c44>  @ imm = #0xbc
    c.branch(C::Ne,0x45794)?;
    // 0x456d4: ldr	r3, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,400)?)?;
    // 0x456d8: cmp	r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x456dc: beq	0x457a0 <entry+0x1c50>  @ imm = #0xbc
    c.branch(C::Eq,0x457a0)?;
    // 0x456e0: ldr	r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x456e4: mov	r6, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,56)?)?;
    // 0x456e8: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x456ec: ldrb	r7, [r3, #0x4a9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(3)?,1193)?)?;
    // 0x456f0: ldrb	r5, [r3, #0x4a2]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(3)?,1186)?)?;
    // 0x456f4: ldrb	r4, [r3, #0x4a3]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(3)?,1187)?)?;
    // 0x456f8: and	r12, r6, r7, lsl #3
    c.emit(arm32::data_register(C::Al,Op::And,R::R12,R::new(6)?,R::new(7)?,Shift::Lsl,3,false)?)?;
    // 0x456fc: ldrb	r6, [r3, #0x4a1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,1185)?)?;
    // 0x45700: ldrb	r7, [r3, #0x4a4]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(3)?,1188)?)?;
    // 0x45704: and	r5, r5, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,7)?)?;
    // 0x45708: lsl	r5, r5, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(5)?,Shift::Lsl,24)?)?;
    // 0x4570c: and	r6, r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,7)?)?;
    // 0x45710: and	r7, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,7)?)?;
    // 0x45714: orr	r6, r5, r6, lsl #27
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,27,false)?)?;
    // 0x45718: and	r5, r4, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(4)?,7)?)?;
    // 0x4571c: orr	r6, r6, r5, lsl #21
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,21,false)?)?;
    // 0x45720: orr	r7, r6, r7, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,18,false)?)?;
    // 0x45724: ldrb	r6, [r3, #0x4a5]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,1189)?)?;
    // 0x45728: and	r6, r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,7)?)?;
    // 0x4572c: orr	r7, r7, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,15,false)?)?;
    // 0x45730: ldrb	r6, [r3, #0x4a6]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,1190)?)?;
    // 0x45734: and	r6, r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,7)?)?;
    // 0x45738: orr	r7, r7, r6, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,12,false)?)?;
    // 0x4573c: ldrb	r6, [r3, #0x4a7]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,1191)?)?;
    // 0x45740: and	r6, r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,7)?)?;
    // 0x45744: orr	r7, r7, r6, lsl #9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,9,false)?)?;
    // 0x45748: ldrb	r6, [r3, #0x4a8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,1192)?)?;
    // 0x4574c: ldrb	r3, [r3, #0x4aa]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(3)?,1194)?)?;
    // 0x45750: and	r6, r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,7)?)?;
    // 0x45754: and	r3, r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,7)?)?;
    // 0x45758: orr	r7, r7, r6, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,6,false)?)?;
    // 0x4575c: orr	r7, r7, r12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x45760: orr	r3, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x45764: ldreq	r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R0,0)?)?;
    // 0x45768: andeq	r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4576c: streq	r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R0,0)?)?;
    // 0x45770: mov	r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // 0x45774: orr	r0, r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // 0x45778: str	r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,8)?)?;
    // 0x4577c: add	r0, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,2)?)?;
    // 0x45780: str	r3, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,12)?)?;
    // 0x45784: str	r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x45788: add	r0, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,16)?)?;
    // 0x4578c: str	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x45790: ldr	r3, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,644)?)?;
    // 0x45794: cmp	r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4096)?)?;
    // 0x45798: cmpne	r3, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x4579c: bne	0x4580c <entry+0x1cbc>  @ imm = #0x68
    c.branch(C::Ne,0x4580c)?;
    // 0x457a0: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x457a4: mov	r6, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3840)?)?;
    // 0x457a8: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x457ac: ldrb	r2, [r1, #0x4ab]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,1195)?)?;
    // 0x457b0: ldrb	r3, [r1, #0x4ac]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,1196)?)?;
    // 0x457b4: ldrb	r7, [r1, #0x4ad]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R1,1197)?)?;
    // 0x457b8: ldrb	r1, [r1, #0x4ae]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,1198)?)?;
    // 0x457bc: and	r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,15)?)?;
    // 0x457c0: and	r3, r6, r3, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x457c4: and	r2, r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,15)?)?;
    // 0x457c8: orr	r1, r1, r7, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,4,false)?)?;
    // 0x457cc: and	r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // 0x457d0: orr	r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x457d4: orr	r1, r1, r2, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,12,false)?)?;
    // 0x457d8: ldreq	r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x457dc: andeq	r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x457e0: streq	r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x457e4: mov	r2, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,60)?)?;
    // 0x457e8: orr	r2, r2, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,6144)?)?;
    // 0x457ec: str	r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x457f0: str	r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x457f4: add	r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x457f8: ldr	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x457fc: add	r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.check(0x45800)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();
    finish_main_packet(&mut c)?;
    c.check(0x4509c)?;
    c.emit(CONTROL_REFERENCE)?;
    build_second_packet(&mut c)?;
    derive_capability_bundles(&mut c)?;
    update_codec_fields(&mut c)?;
    Ok(c.bytes)
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn controller_mode_literal_is_separate_from_instructions(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        let ix=0x4509c-START;
        assert_eq!(u32::from_le_bytes(b[ix..ix+4].try_into().unwrap()),CONTROL_REFERENCE);
    }
}
