// SPDX-License-Identifier: GPL-3.0-only
//! Venus A32 codec register and source capability checks from 0x45800 to
//! 0x46000. The complete 4 KB slices continue a larger controller function;
//! do not infer a return from this address boundary.
//!
//! Two inline constant pool sites are separated from executable A32:
//! 0x45c80–0x45c88 and 0x45f14–0x45f18. The disassembler gave implausible
//! LDRSHT PC/PC mnemonics for these words; the earlier unconditional
//! branches jump around them. They are data, not coprocessor instructions.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
pub const START:usize=0x45800;
pub const END:usize=0x46000;
// Exact operand/index constants in non-executable ARM pools.
const CODEC_INDEX_MASK:u32=0x00ff_03ff;
const CODEC_COMPARISON_WORD:u32=0x4082_0826;
const CODEC_LIMIT_LITERAL:u32=0x807f_ffff;
struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
    fn check(&self,a:usize)->Result<(),String>{
        if self.at!=a{return Err(format!("controller capability 0x{:x}, expected 0x{a:x}",self.at));} Ok(())
    }
    #[inline(never)]
    fn emit(&mut self,w:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at)||self.at&3!=0 {return Err("invalid owned runtime location".into());}
        self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
    }
    #[inline(never)]
    fn branch(&mut self,c:C,t:usize)->Result<(),String>{
        self.emit(arm32::branch(c,self.at as u32,t as u32,false)?)
    }
}

/// Pack channel capability words and process active-mode status.
#[inline(never)]
fn encode_primary_capabilities(c:&mut Code)->Result<(),String>{
    c.check(0x45800)?;
    // 0x45800: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x45804: str	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x45808: ldr	r3, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,644)?)?;
    // 0x4580c: cmp	r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8192)?)?;
    // 0x45810: cmpne	r3, #16384
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(3)?,16384)?)?;
    // 0x45814: bne	0x459cc <entry+0x1e7c>  @ imm = #0x1b0
    c.branch(C::Ne,0x459cc)?;
    // 0x45818: ldr	lr, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(8)?,400)?)?;
    // 0x4581c: cmp	lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // 0x45820: beq	0x459d4 <entry+0x1e84>  @ imm = #0x1ac
    c.branch(C::Eq,0x459d4)?;
    // 0x45824: ldr	r2, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2744)?)?;
    // 0x45828: mov	r1, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,31744)?)?;
    // 0x4582c: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x45830: add	r7, r2, #936
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R2,936)?)?;
    // 0x45834: ldm	r7, {r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(7)?,R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x45838: ldr	r4, [r2, #0x3b4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R2,948)?)?;
    // 0x4583c: and	r5, r1, r7, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R1,R::new(7)?,Shift::Lsl,10,false)?)?;
    // 0x45840: mov	r1, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,992)?)?;
    // 0x45844: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x45848: and	r6, r1, r6, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R1,R::new(6)?,Shift::Lsl,5,false)?)?;
    // 0x4584c: mov	r1, #1015808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1015808)?)?;
    // 0x45850: orr	r3, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x45854: and	r6, r1, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R1,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x45858: mov	r1, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32505856)?)?;
    // 0x4585c: orr	r3, r3, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x45860: orr	r3, r3, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x45864: ldr	r6, [r2, #0x3b8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,952)?)?;
    // 0x45868: ldr	r2, [r2, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,956)?)?;
    // 0x4586c: and	r6, r1, r6, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R1,R::new(6)?,Shift::Lsl,20,false)?)?;
    // 0x45870: mov	r1, #1040187392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1040187392)?)?;
    // 0x45874: orr	r3, r3, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x45878: and	r2, r1, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R2,Shift::Lsl,25,false)?)?;
    // 0x4587c: orr	r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x45880: ldreq	r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x45884: andeq	r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x45888: streq	r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x4588c: mov	r3, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,72)?)?;
    // 0x45890: orr	r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x45894: str	r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x45898: str	r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4589c: mov	r2, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,31744)?)?;
    // 0x458a0: ldr	r8, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-36)?)?;
    // 0x458a4: ldr	r9, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,104)?)?;
    // 0x458a8: add	r3, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(8)?,1)?)?;
    // 0x458ac: str	r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x458b0: add	r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x458b4: str	r3, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x458b8: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x458bc: ldr	r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x458c0: add	r12, r7, #964
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(7)?,964)?)?;
    // 0x458c4: ldr	r5, [r7, #0x3c0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,960)?)?;
    // 0x458c8: ldm	r12, {r4, r6, r12}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::new(4)?.bit() | R::new(6)?.bit() | R::R12.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x458cc: and	r5, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,31)?)?;
    // 0x458d0: and	r6, r2, r6, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R2,R::new(6)?,Shift::Lsl,10,false)?)?;
    // 0x458d4: mov	r2, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,992)?)?;
    // 0x458d8: and	r4, r2, r4, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R2,R::new(4)?,Shift::Lsl,5,false)?)?;
    // 0x458dc: mov	r2, #1015808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1015808)?)?;
    // 0x458e0: orr	r5, r4, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x458e4: orr	r6, r5, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x458e8: and	r5, r2, r12, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R2,R::R12,Shift::Lsl,15,false)?)?;
    // 0x458ec: mov	r2, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32505856)?)?;
    // 0x458f0: orr	r6, r6, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x458f4: ldr	r5, [r7, #0x3d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,976)?)?;
    // 0x458f8: ldr	r7, [r7, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,980)?)?;
    // 0x458fc: and	r5, r2, r5, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R2,R::new(5)?,Shift::Lsl,20,false)?)?;
    // 0x45900: and	r7, r1, r7, lsl #25
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R1,R::new(7)?,Shift::Lsl,25,false)?)?;
    // 0x45904: mov	r2, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,31744)?)?;
    // 0x45908: orr	r6, r6, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4590c: orr	r7, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x45910: ldreq	r6, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(3)?,0)?)?;
    // 0x45914: andeq	r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x45918: streq	r6, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(3)?,0)?)?;
    // 0x4591c: mov	r3, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,76)?)?;
    // 0x45920: orr	r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // 0x45924: str	r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x45928: add	r3, r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(8)?,2)?)?;
    // 0x4592c: str	r7, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,12)?)?;
    // 0x45930: str	r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x45934: add	r3, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,16)?)?;
    // 0x45938: str	r3, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x4593c: tst	r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x45940: ldr	r7, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,2744)?)?;
    // 0x45944: ldr	r4, [r7, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,992)?)?;
    // 0x45948: ldr	r6, [r7, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,984)?)?;
    // 0x4594c: ldr	r5, [r7, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,988)?)?;
    // 0x45950: ldr	r1, [r7, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,996)?)?;
    // 0x45954: and	r2, r2, r4, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(4)?,Shift::Lsl,10,false)?)?;
    // 0x45958: mov	r4, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,992)?)?;
    // 0x4595c: and	r6, r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,31)?)?;
    // 0x45960: and	r5, r4, r5, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,5,false)?)?;
    // 0x45964: orr	r6, r5, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x45968: orr	r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x4596c: mov	r6, #1015808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1015808)?)?;
    // 0x45970: and	r1, r6, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(6)?,R::R1,Shift::Lsl,15,false)?)?;
    // 0x45974: mov	r6, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,32505856)?)?;
    // 0x45978: orr	r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4597c: ldr	r2, [r7, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,1000)?)?;
    // 0x45980: and	r2, r6, r2, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,20,false)?)?;
    // 0x45984: orr	r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x45988: ldr	r2, [r7, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,1004)?)?;
    // 0x4598c: mov	r7, #1040187392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1040187392)?)?;
    // 0x45990: and	r2, r7, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(7)?,R::R2,Shift::Lsl,25,false)?)?;
    // 0x45994: orr	r7, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x45998: ldreq	r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(3)?,0)?)?;
    // 0x4599c: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x459a0: streq	r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(3)?,0)?)?;
    // 0x459a4: mov	r1, #2128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2128)?)?;
    // 0x459a8: orr	r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x459ac: str	r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x459b0: str	r7, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,20)?)?;
    // 0x459b4: add	r1, r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,3)?)?;
    // 0x459b8: add	r0, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,24)?)?;
    // 0x459bc: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x459c0: str	r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x459c4: ldr	r8, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,80)?)?;
    // 0x459c8: b	0x459d8 <entry+0x1e88>  @ imm = #0x8
    c.branch(C::Al,0x459d8)?;
    // 0x459cc: ldr	lr, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(8)?,400)?)?;
    // 0x459d0: b	0x459d8 <entry+0x1e88>  @ imm = #0x0
    c.branch(C::Al,0x459d8)?;
    // 0x459d4: mov	lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // 0x459d8: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x459dc: mov	r5, #258048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,258048)?)?;
    // 0x459e0: cmp	lr, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,2)?)?;
    // 0x459e4: ldr	r7, [r1, #0x41c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,1052)?)?;
    // 0x459e8: ldr	r2, [r1, #0x414]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,1044)?)?;
    // 0x459ec: ldr	r3, [r1, #0x418]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,1048)?)?;
    // 0x459f0: ldr	r6, [r1, #0x420]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,1056)?)?;
    // 0x459f4: ldr	r1, [r1, #0x424]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,1060)?)?;
    // 0x459f8: and	r7, r5, r7, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(5)?,R::new(7)?,Shift::Lsl,12,false)?)?;
    // 0x459fc: mov	r5, #4032
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4032)?)?;
    // 0x45a00: and	r1, r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,63)?)?;
    // 0x45a04: and	r6, r5, r6, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,6,false)?)?;
    // 0x45a08: orr	r1, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x45a0c: orr	r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x45a10: mov	r7, #16515072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16515072)?)?;
    // 0x45a14: and	r3, r7, r3, lsl #18
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,18,false)?)?;
    // 0x45a18: orr	r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x45a1c: mov	r3, #1056964608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1056964608)?)?;
    // 0x45a20: and	r2, r3, r2, lsl #24
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,24,false)?)?;
    // 0x45a24: orr	r2, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x45a28: ldr	r1, [pc, #0xfe8]        @ 0x2ec8 <entry+0x2ec8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,4072)?)?;
    // 0x45a2c: and	r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x45a30: orr	r1, r1, #58
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,58)?)?;
    // 0x45a34: orreq	r2, r1, #249856
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R2,R::R1,249856)?)?;
    // 0x45a38: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x45a3c: ldreq	r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x45a40: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x45a44: streq	r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x45a48: mov	r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // 0x45a4c: orr	r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x45a50: stm	r0, {r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit() | R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x45a54: mov	r1, #469762048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,469762048)?)?;
    // 0x45a58: ldr	r12, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-36)?)?;
    // 0x45a5c: ldr	lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x45a60: add	r2, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R12,1)?)?;
    // 0x45a64: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x45a68: add	r2, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,8)?)?;
    // 0x45a6c: str	r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x45a70: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x45a74: ldr	r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x45a78: ldr	r7, [r3, #0x444]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,1092)?)?;
    // 0x45a7c: ldr	r6, [r3, #0x448]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,1096)?)?;
    c.check(0x45a80)
}

/// Validate codec mode identifiers and conditional register behavior.
#[inline(never)]
fn validate_codec_descriptor_modes(c:&mut Code)->Result<(),String>{
    c.check(0x45a80)?;
    // 0x45a80: ldr	r5, [r3, #0x450]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,1104)?)?;
    // 0x45a84: ldr	r4, [r3, #0x454]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(3)?,1108)?)?;
    // 0x45a88: and	r1, r1, r7, lsl #26
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,26,false)?)?;
    // 0x45a8c: mov	r7, #58720256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,58720256)?)?;
    // 0x45a90: and	r7, r7, r6, lsl #23
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,23,false)?)?;
    // 0x45a94: mov	r6, #448
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,448)?)?;
    // 0x45a98: orr	r1, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x45a9c: mov	r7, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,458752)?)?;
    // 0x45aa0: and	r7, r7, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x45aa4: orr	r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x45aa8: mov	r7, #57344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,57344)?)?;
    // 0x45aac: and	r7, r7, r4, lsl #13
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(4)?,Shift::Lsl,13,false)?)?;
    // 0x45ab0: mov	r4, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,32505856)?)?;
    // 0x45ab4: orr	r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x45ab8: ldr	r7, [r3, #0x45c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,1116)?)?;
    // 0x45abc: ldr	r3, [r3, #0x460]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,1120)?)?;
    // 0x45ac0: and	r7, r6, r7, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,6,false)?)?;
    // 0x45ac4: orr	r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x45ac8: mov	r7, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,56)?)?;
    // 0x45acc: and	r3, r7, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x45ad0: orr	r3, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x45ad4: ldreq	r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x45ad8: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x45adc: streq	r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x45ae0: mov	r1, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,88)?)?;
    // 0x45ae4: add	r2, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,16)?)?;
    // 0x45ae8: orr	r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x45aec: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x45af0: str	r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x45af4: add	r1, r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,2)?)?;
    // 0x45af8: str	r3, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,12)?)?;
    // 0x45afc: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x45b00: str	r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x45b04: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x45b08: ldr	r3, [r1, #0x430]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,1072)?)?;
    // 0x45b0c: ldr	r7, [r1, #0x434]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,1076)?)?;
    // 0x45b10: ldr	r6, [r1, #0x438]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,1080)?)?;
    // 0x45b14: ldr	r5, [r1, #0x43c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,1084)?)?;
    // 0x45b18: ldr	r1, [r1, #0x440]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,1088)?)?;
    // 0x45b1c: and	r1, r4, r1, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::R1,Shift::Lsl,20,false)?)?;
    // 0x45b20: mov	r4, #1015808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1015808)?)?;
    // 0x45b24: and	r3, r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,31)?)?;
    // 0x45b28: and	r5, r4, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,15,false)?)?;
    // 0x45b2c: orr	r1, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x45b30: mov	r5, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,31744)?)?;
    // 0x45b34: and	r6, r5, r6, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,10,false)?)?;
    // 0x45b38: orr	r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x45b3c: mov	r6, #992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,992)?)?;
    // 0x45b40: and	r7, r6, r7, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,5,false)?)?;
    // 0x45b44: orr	r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x45b48: orr	r3, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x45b4c: ldreq	r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x45b50: andeq	r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x45b54: streq	r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x45b58: mov	r1, #92
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,92)?)?;
    // 0x45b5c: orr	r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // 0x45b60: str	r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x45b64: str	r3, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,20)?)?;
    // 0x45b68: add	r1, r12, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,3)?)?;
    // 0x45b6c: add	r0, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,24)?)?;
    // 0x45b70: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x45b74: str	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x45b78: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45b7c: ldrb	r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x45b80: cmp	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x45b84: bne	0x46568 <entry+0x2a18>  @ imm = #0x9dc
    c.branch(C::Ne,0x46568)?;
    // 0x45b88: ldrb	r0, [r10, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,988)?)?;
    // 0x45b8c: mov	r9, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,9)?)?;
    // 0x45b90: orr	r9, r9, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,4194304)?)?;
    // 0x45b94: cmp	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x45b98: movne	r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x45b9c: lsl	r1, r0, #19
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Lsl,19)?)?;
    // 0x45ba0: orr	r0, r1, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x45ba4: ldr	r1, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,400)?)?;
    // 0x45ba8: orr	r4, r0, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::R0,4194304)?)?;
    // 0x45bac: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x45bb0: beq	0x45bf0 <entry+0x20a0>  @ imm = #0x38
    c.branch(C::Eq,0x45bf0)?;
    // 0x45bb4: cmp	r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x45bb8: beq	0x45bcc <entry+0x207c>  @ imm = #0xc
    c.branch(C::Eq,0x45bcc)?;
    // 0x45bbc: cmp	r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x45bc0: bne	0x45bd8 <entry+0x2088>  @ imm = #0x10
    c.branch(C::Ne,0x45bd8)?;
    // 0x45bc4: add	r1, r9, #119
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,119)?)?;
    // 0x45bc8: b	0x45bd0 <entry+0x2080>  @ imm = #0x0
    c.branch(C::Al,0x45bd0)?;
    // 0x45bcc: add	r1, r9, #247
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,247)?)?;
    // 0x45bd0: orr	r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x45bd4: b	0x45bf0 <entry+0x20a0>  @ imm = #0x14
    c.branch(C::Al,0x45bf0)?;
    // 0x45bd8: ldr	r2, [pc, #0x4d8]        @ 0x2568 <entry+0x2568>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1240)?)?;
    // 0x45bdc: mov	r1, #65
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,65)?)?;
    // 0x45be0: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45be4: orr	r1, r1, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8448)?)?;
    // 0x45be8: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x44440
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45bec: ldr	lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x45bf0: ldr	r0, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,644)?)?;
    // 0x45bf4: mov	r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x45bf8: mov	r7, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16711680)?)?;
    // 0x45bfc: orr	r5, r5, #16128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,16128)?)?;
    // 0x45c00: orr	r7, r7, #1056964608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,1056964608)?)?;
    // 0x45c04: cmp	r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x45c08: bge	0x45c40 <entry+0x20f0>  @ imm = #0x30
    c.branch(C::Ge,0x45c40)?;
    // 0x45c0c: cmp	r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x45c10: beq	0x45c60 <entry+0x2110>  @ imm = #0x48
    c.branch(C::Eq,0x45c60)?;
    // 0x45c14: cmp	r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x45c18: beq	0x45c68 <entry+0x2118>  @ imm = #0x48
    c.branch(C::Eq,0x45c68)?;
    // 0x45c1c: cmp	r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x45c20: bne	0x45c90 <entry+0x2140>  @ imm = #0x68
    c.branch(C::Ne,0x45c90)?;
    // 0x45c24: ldr	r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x45c28: bic	r0, r4, #61
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(4)?,61)?)?;
    // 0x45c2c: ldr	r1, [r1, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,172)?)?;
    // 0x45c30: and	r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,1)?)?;
    // 0x45c34: orr	r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x45c38: orr	r1, r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,36)?)?;
    // 0x45c3c: b	0x45c94 <entry+0x2144>  @ imm = #0x50
    c.branch(C::Al,0x45c94)?;
    // 0x45c40: beq	0x45c74 <entry+0x2124>  @ imm = #0x2c
    c.branch(C::Eq,0x45c74)?;
    // 0x45c44: ldr	r1, [pc, #0xde4]        @ 0x2ee0 <entry+0x2ee0>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,3556)?)?;
    // 0x45c48: cmp	r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x45c4c: beq	0x45c88 <entry+0x2138>  @ imm = #0x34
    c.branch(C::Eq,0x45c88)?;
    // 0x45c50: cmn	r0, #-134217727
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,4160749569)?)?;
    // 0x45c54: biceq	r0, r4, #60
    c.emit(arm32::data_immediate(C::Eq,Op::Bic,R::R0,R::new(4)?,60)?)?;
    // 0x45c58: orreq	r4, r0, #20
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(4)?,R::R0,20)?)?;
    // 0x45c5c: b	0x45c90 <entry+0x2140>  @ imm = #0x2c
    c.branch(C::Al,0x45c90)?;
    // 0x45c60: bic	r4, r4, #60
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(4)?,R::new(4)?,60)?)?;
    // 0x45c64: b	0x45c90 <entry+0x2140>  @ imm = #0x24
    c.branch(C::Al,0x45c90)?;
    // 0x45c68: bic	r0, r4, #60
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(4)?,60)?)?;
    // 0x45c6c: orr	r4, r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::R0,28)?)?;
    // 0x45c70: b	0x45c90 <entry+0x2140>  @ imm = #0x18
    c.branch(C::Al,0x45c90)?;
    // 0x45c74: bic	r0, r4, #60
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(4)?,60)?)?;
    // 0x45c78: orr	r4, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::R0,32)?)?;
    // 0x45c7c: b	0x45c90 <entry+0x2140>  @ imm = #0xc
    c.branch(C::Al,0x45c90)?;
    c.check(0x45c80)
}

/// Write selected mode and stride parameters to controller buffers.
#[inline(never)]
fn publish_mode_parameters(c:&mut Code)->Result<(),String>{
    c.check(0x45c88)?;
    // 0x45c88: bic	r0, r4, #60
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(4)?,60)?)?;
    // 0x45c8c: orr	r4, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::R0,4)?)?;
    // 0x45c90: bic	r1, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(4)?,1)?)?;
    // 0x45c94: ldr	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x45c98: tst	r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x45c9c: ldreq	r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x45ca0: andeq	r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x45ca4: streq	r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x45ca8: mov	r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7168)?)?;
    // 0x45cac: str	r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x45cb0: str	r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x45cb4: ldr	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x45cb8: add	r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,1)?)?;
    // 0x45cbc: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x45cc0: add	r2, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,8)?)?;
    // 0x45cc4: str	r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x45cc8: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x45ccc: ldr	r3, [r10, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2652)?)?;
    // 0x45cd0: ldr	r6, [r10, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,2656)?)?;
    // 0x45cd4: and	r3, r3, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x45cd8: and	r6, r7, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(7)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x45cdc: orr	r3, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x45ce0: ldreq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R2,0)?)?;
    // 0x45ce4: andeq	r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x45ce8: streq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,0)?)?;
    // 0x45cec: mov	r2, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,88)?)?;
    // 0x45cf0: orr	r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x45cf4: str	r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,8)?)?;
    // 0x45cf8: add	r2, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,2)?)?;
    // 0x45cfc: str	r3, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,12)?)?;
    // 0x45d00: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x45d04: add	r2, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,16)?)?;
    // 0x45d08: str	r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x45d0c: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x45d10: ldr	r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x45d14: ldr	r6, [r3, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,136)?)?;
    // 0x45d18: ldrh	r3, [r3, #140]
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::new(3)?,140)?)?;
    // 0x45d1c: orr	r3, r3, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x45d20: ldreq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R2,0)?)?;
    // 0x45d24: andeq	r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x45d28: streq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,0)?)?;
    // 0x45d2c: mov	r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x45d30: orr	r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x45d34: str	r2, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,16)?)?;
    // 0x45d38: add	r2, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,3)?)?;
    // 0x45d3c: str	r3, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,20)?)?;
    // 0x45d40: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x45d44: add	r2, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,24)?)?;
    // 0x45d48: str	r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x45d4c: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x45d50: ldr	r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x45d54: ldr	r6, [r3, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,144)?)?;
    // 0x45d58: ldrh	r3, [r3, #148]
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::new(3)?,148)?)?;
    // 0x45d5c: orr	r3, r3, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x45d60: ldreq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R2,0)?)?;
    // 0x45d64: andeq	r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x45d68: streq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,0)?)?;
    // 0x45d6c: mov	r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x45d70: orr	r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x45d74: str	r2, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,24)?)?;
    // 0x45d78: add	r2, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,4)?)?;
    // 0x45d7c: str	r3, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,28)?)?;
    c.check(0x45d80)
}

/// Evaluate device feature statuses and invoke diagnostic callbacks.
#[inline(never)]
fn check_device_feature_states(c:&mut Code)->Result<(),String>{
    c.check(0x45d80)?;
    // 0x45d80: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x45d84: add	r2, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,32)?)?;
    // 0x45d88: str	r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x45d8c: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x45d90: ldr	r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x45d94: ldr	r6, [r3, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,152)?)?;
    // 0x45d98: ldrh	r3, [r3, #156]
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::new(3)?,156)?)?;
    // 0x45d9c: orr	r3, r3, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x45da0: ldreq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R2,0)?)?;
    // 0x45da4: andeq	r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x45da8: streq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,0)?)?;
    // 0x45dac: mov	r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x45db0: orr	r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x45db4: str	r2, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,32)?)?;
    // 0x45db8: add	r2, r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,5)?)?;
    // 0x45dbc: str	r3, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,36)?)?;
    // 0x45dc0: add	r1, r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,6)?)?;
    // 0x45dc4: str	r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x45dc8: add	r2, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,40)?)?;
    // 0x45dcc: str	r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x45dd0: tst	r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x45dd4: ldr	r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x45dd8: ldr	r6, [r3, #0xa0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,160)?)?;
    // 0x45ddc: ldrh	r3, [r3, #164]
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::new(3)?,164)?)?;
    // 0x45de0: orr	r3, r3, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x45de4: ldreq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R2,0)?)?;
    // 0x45de8: andeq	r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x45dec: streq	r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,0)?)?;
    // 0x45df0: mov	r2, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // 0x45df4: orr	r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x45df8: str	r2, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,40)?)?;
    // 0x45dfc: str	r3, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,44)?)?;
    // 0x45e00: add	r0, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,48)?)?;
    // 0x45e04: str	r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x45e08: str	r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x45e0c: ldr	r0, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,644)?)?;
    // 0x45e10: cmp	r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // 0x45e14: beq	0x45f18 <entry+0x23c8>  @ imm = #0xfc
    c.branch(C::Eq,0x45f18)?;
    // 0x45e18: cmp	r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x45e1c: bne	0x46024 <entry+0x24d4>  @ imm = #0x200
    c.branch(C::Ne,0x46024)?;
    // 0x45e20: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45e24: ldrb	r1, [r0, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,204)?)?;
    // 0x45e28: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x45e2c: beq	0x46024 <entry+0x24d4>  @ imm = #0x1f0
    c.branch(C::Eq,0x46024)?;
    // 0x45e30: ldrb	r0, [r0, #0x498]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1176)?)?;
    // 0x45e34: ldr	r4, [pc, #0x27c]        @ 0x2568 <entry+0x2568>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,636)?)?;
    // 0x45e38: mov	r1, #137
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,137)?)?;
    // 0x45e3c: orr	r1, r1, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8448)?)?;
    // 0x45e40: cmp	r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x45e44: mov	r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x45e48: movne	r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45e4c: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x446a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45e50: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45e54: mov	r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x45e58: mov	r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x45e5c: ldrb	r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x45e60: ldrb	r0, [r0, #0xda]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,218)?)?;
    // 0x45e64: cmp	r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x45e68: mov	r1, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,138)?)?;
    // 0x45e6c: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45e70: moveq	r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x45e74: orr	r1, r1, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8448)?)?;
    // 0x45e78: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x446d0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45e7c: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45e80: ldrb	r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x45e84: ldrb	r0, [r0, #0xdc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,220)?)?;
    // 0x45e88: bl	0x81850 <entry+0x3dd00> @ imm = #0x3b9c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x45e8c: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x45e90: mov	r1, #139
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,139)?)?;
    // 0x45e94: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45e98: mov	r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x45e9c: moveq	r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x45ea0: orr	r1, r1, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8448)?)?;
    // 0x45ea4: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x446fc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45ea8: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45eac: ldrb	r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x45eb0: ldrb	r0, [r0, #0xdb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,219)?)?;
    // 0x45eb4: bl	0x81850 <entry+0x3dd00> @ imm = #0x3b994
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x45eb8: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x45ebc: mov	r1, #396
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,396)?)?;
    // 0x45ec0: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45ec4: mov	r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x45ec8: moveq	r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x45ecc: orr	r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x45ed0: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x44728
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45ed4: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45ed8: mov	r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x45edc: ldrb	r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x45ee0: ldrb	r0, [r0, #0xdd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,221)?)?;
    // 0x45ee4: cmp	r0, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,1,false)?)?;
    // 0x45ee8: mov	r1, #141
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,141)?)?;
    // 0x45eec: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45ef0: moveq	r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x45ef4: orr	r1, r1, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8448)?)?;
    // 0x45ef8: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x44750
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45efc: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45f00: ldrb	r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x45f04: ldrb	r0, [r0, #0xde]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,222)?)?;
    // 0x45f08: cmp	r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x45f0c: mov	r1, #142
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,142)?)?;
    // 0x45f10: b	0x4600c <entry+0x24bc>  @ imm = #0xf4
    c.branch(C::Al,0x4600c)?;
    c.check(0x45f14)
}

/// Validate the remaining source capability fields and record flags.
#[inline(never)]
fn check_followup_source_capabilities(c:&mut Code)->Result<(),String>{
    c.check(0x45f18)?;
    // 0x45f18: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45f1c: ldrb	r1, [r0, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,204)?)?;
    // 0x45f20: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x45f24: beq	0x46024 <entry+0x24d4>  @ imm = #0xf8
    c.branch(C::Eq,0x46024)?;
    // 0x45f28: ldrb	r0, [r0, #0x498]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1176)?)?;
    // 0x45f2c: ldr	r4, [pc, #0x184]        @ 0x2568 <entry+0x2568>
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::PC,388)?)?;
    // 0x45f30: mov	r1, #145
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,145)?)?;
    // 0x45f34: orr	r1, r1, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8448)?)?;
    // 0x45f38: cmp	r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x45f3c: mov	r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x45f40: movne	r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45f44: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x4479c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45f48: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45f4c: mov	r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x45f50: mov	r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x45f54: ldrb	r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x45f58: ldrb	r0, [r0, #0xda]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,218)?)?;
    // 0x45f5c: cmp	r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x45f60: mov	r1, #146
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,146)?)?;
    // 0x45f64: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45f68: moveq	r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x45f6c: orr	r1, r1, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8448)?)?;
    // 0x45f70: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x447c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45f74: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45f78: ldrb	r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x45f7c: ldrb	r0, [r0, #0xdc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,220)?)?;
    // 0x45f80: bl	0x81850 <entry+0x3dd00> @ imm = #0x3b8c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x45f84: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x45f88: mov	r1, #147
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,147)?)?;
    // 0x45f8c: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45f90: mov	r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x45f94: moveq	r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x45f98: orr	r1, r1, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8448)?)?;
    // 0x45f9c: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x447f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45fa0: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45fa4: ldrb	r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x45fa8: ldrb	r0, [r0, #0xdb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,219)?)?;
    // 0x45fac: bl	0x81850 <entry+0x3dd00> @ imm = #0x3b89c
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x45fb0: cmp	r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x45fb4: mov	r1, #404
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,404)?)?;
    // 0x45fb8: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45fbc: mov	r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x45fc0: moveq	r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x45fc4: orr	r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x45fc8: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x44820
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45fcc: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45fd0: mov	r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x45fd4: ldrb	r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x45fd8: ldrb	r0, [r0, #0xdd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,221)?)?;
    // 0x45fdc: cmp	r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x45fe0: mov	r1, #149
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,149)?)?;
    // 0x45fe4: mov	r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x45fe8: moveq	r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x45fec: orr	r1, r1, #8448
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8448)?)?;
    // 0x45ff0: bl	0x17b0 <entry+0xfffffffffffbdc60> @ imm = #-0x44848
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x45ff4: ldr	r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x45ff8: ldrb	r1, [r0, #0xd9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,217)?)?;
    // 0x45ffc: ldrb	r0, [r0, #0xde]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,222)?)?;
    c.check(0x46000)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();
    encode_primary_capabilities(&mut c)?;
    validate_codec_descriptor_modes(&mut c)?;
    c.check(0x45c80)?;
    c.emit(CODEC_INDEX_MASK)?;
    c.emit(CODEC_COMPARISON_WORD)?;
    publish_mode_parameters(&mut c)?;
    check_device_feature_states(&mut c)?;
    c.check(0x45f14)?;
    c.emit(CODEC_LIMIT_LITERAL)?;
    check_followup_source_capabilities(&mut c)?;
    Ok(c.bytes)
}
#[cfg(test)] mod tests{
    use super::*;
    #[test]fn codec_capability_words_are_not_misidentified_as_code(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        for (at,word) in [(0x45c80,CODEC_INDEX_MASK),(0x45c84,CODEC_COMPARISON_WORD),
                          (0x45f14,CODEC_LIMIT_LITERAL)]{
            let offset=at-START;
            assert_eq!(u32::from_le_bytes(b[offset..offset+4].try_into().unwrap()),word);
        }
    }
}
