// SPDX-License-Identifier: GPL-3.0-only
//! Source-encoded codec controller command words and stride tables across
//! 0x48000–0x49000. The four inline values at 0x48a20..0x48a30 are
//! branch-skipped controller masks/data, never treated as instructions.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};

pub const START:usize=0x48000;
pub const END:usize=0x49000;
const CODEC_CONTROLLER_MASKS:[u32;4]=[
    0xffff_c000, 0x8000_3fff, 0x000e_e028, 0xaaaa_aaab,
];
/// Checked public ARM SMULBB signed low-halfword multiply.
fn signed_multiply_bottom_halves(c:C,rd:R,rm:R,rs:R)->Result<u32,String>{
    if [rd,rm,rs].iter().any(|x|matches!(x.number(),13|15)){return Err("unsafe SMULBB register".into());}
    Ok(((c as u32)<<28)|0x0160_0080|(rd.number()<<16)|(rs.number()<<8)|rm.number())
}
struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
    fn check(&self,a:usize)->Result<(),String>{
        if self.at!=a{return Err(format!("A32 packet writer at 0x{:x}, expected 0x{a:x}",self.at));}Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at)||self.at%4!=0{return Err("outside Agent 04 exclusive code reservation".into());}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,c:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(c,self.at as u32,target as u32,false)?)
    }
    fn literal(&mut self,c:C,rd:R,target:usize)->Result<(),String>{
        let delta=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"invalid PC literal span")?;
        self.emit(arm32::transfer_word(c,true,rd,R::PC,delta)?)
    }
}

/// Continue codec controller-register commands and format-specific metadata
/// packing for the active frame. No function return is fabricated here.
fn pack_additional_codec_registers(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x48000: stm r0, {r1, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x48004: ldr r12, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-36)?)?;
    // 0x48008: add r2, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R12,1)?)?;
    // 0x4800c: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x48010: add r2, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,8)?)?;
    // 0x48014: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x48018: ldr r3, [sp, #0x224]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,548)?)?;
    // 0x4801c: and r3, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,3)?)?;
    // 0x48020: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x48024: ldreq r3, [r10, #0x694]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(10)?,1684)?)?;
    // 0x48028: streq r3, [sp, #0xcc]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::SP,204)?)?;
    // 0x4802c: ldr r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x48030: ldr r3, [r3, #0x514]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,1300)?)?;
    // 0x48034: str r7, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,76)?)?;
    // 0x48038: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x4803c: beq 0x48060 <entry+0x2060>  @ imm = #0x1c
    c.branch(C::Eq,0x48060)?;
    // 0x48040: and r3, r3, #12
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,12)?)?;
    // 0x48044: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x48048: cmp r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4)?)?;
    // 0x4804c: ldr r3, [r10, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1752)?)?;
    // 0x48050: moveq r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x48054: cmp r3, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,384)?)?;
    // 0x48058: movls r7, #3
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x4805c: b 0x48070 <entry+0x2070>  @ imm = #0xc
    c.branch(C::Al,0x48070)?;
    // 0x48060: ldr r3, [r10, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1752)?)?;
    // 0x48064: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x48068: cmp r3, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,384)?)?;
    // 0x4806c: movls r7, #3
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x48070: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x48074: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x48078: orr r3, r3, #17408
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,17408)?)?;
    // 0x4807c: orr r3, r3, r7, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,4,false)?)?;
    // 0x48080: str r3, [sp, #0x228]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,552)?)?;
    // 0x48084: ldreq r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R2,0)?)?;
    // 0x48088: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x4808c: streq r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,0)?)?;
    // 0x48090: mov r2, #420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,420)?)?;
    // 0x48094: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x48098: str r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,8)?)?;
    // 0x4809c: add r2, r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R12,2)?)?;
    // 0x480a0: str r3, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,12)?)?;
    // 0x480a4: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x480a8: add r2, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,16)?)?;
    // 0x480ac: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x480b0: ldr r3, [r10, #0x6a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1696)?)?;
    // 0x480b4: str r3, [sp, #0xd0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,208)?)?;
    // 0x480b8: str r3, [sp, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,212)?)?;
    // 0x480bc: ldr r3, [r10, #0x6a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1704)?)?;
    // 0x480c0: str r3, [sp, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,216)?)?;
    // 0x480c4: ldr r3, [r10, #0x6b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1712)?)?;
    // 0x480c8: str r3, [sp, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,112)?)?;
    // 0x480cc: str r3, [sp, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,116)?)?;
    // 0x480d0: ldr r3, [r10, #0x6b4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1716)?)?;
    // 0x480d4: str r3, [sp, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,120)?)?;
    // 0x480d8: ldr r3, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2744)?)?;
    // 0x480dc: ldr r5, [r3, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,1296)?)?;
    // 0x480e0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x480e4: str r3, [sp, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,556)?)?;
    // 0x480e8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x480ec: beq 0x48158 <entry+0x2158>  @ imm = #0x64
    c.branch(C::Eq,0x48158)?;
    // 0x480f0: and r6, r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(5)?,3)?)?;
    // 0x480f4: cmp r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,3)?)?;
    // 0x480f8: str r6, [sp, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,556)?)?;
    // 0x480fc: bne 0x48114 <entry+0x2114>  @ imm = #0x10
    c.branch(C::Ne,0x48114)?;
    // 0x48100: ldr r3, [r10, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1772)?)?;
    // 0x48104: mov r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // 0x48108: cmp r3, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,384)?)?;
    // 0x4810c: movhi r6, #0
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x48110: strhi r6, [sp, #0x22c]
    c.emit(arm32::transfer_word(C::Hi,false,R::new(6)?,R::SP,556)?)?;
    // 0x48114: ldr r4, [r10, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,1776)?)?;
    // 0x48118: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x4811c: and r3, r3, r5, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsr,2,false)?)?;
    // 0x48120: lsl r5, r3, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(3)?,Shift::Lsl,4)?)?;
    // 0x48124: cmp r4, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,384)?)?;
    // 0x48128: movhi r5, #0
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x4812c: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x48130: moveq r5, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x48134: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // 0x48138: orr r3, r5, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4813c: str r3, [sp, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,556)?)?;
    // 0x48140: bne 0x48158 <entry+0x2158>  @ imm = #0x10
    c.branch(C::Ne,0x48158)?;
    // 0x48144: orr r3, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(5)?,4)?)?;
    // 0x48148: str r3, [sp, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,556)?)?;
    // 0x4814c: ldr r6, [r10, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,1772)?)?;
    // 0x48150: str r6, [sp, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,220)?)?;
    // 0x48154: b 0x4815c <entry+0x215c>  @ imm = #0x0
    c.branch(C::Al,0x4815c)?;
    // 0x48158: str r3, [sp, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,556)?)?;
    // 0x4815c: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x48160: ldreq r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R2,0)?)?;
    // 0x48164: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x48168: streq r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,0)?)?;
    // 0x4816c: mov r2, #424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,424)?)?;
    // 0x48170: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x48174: str r2, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,16)?)?;
    // 0x48178: str r3, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,20)?)?;
    // 0x4817c: add r2, r12, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R12,3)?)?;
    // 0x48180: add r3, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,24)?)?;
    // 0x48184: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x48188: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::LR,0)?)?;
    // 0x4818c: ldr r2, [r10, #0x6b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1720)?)?;
    // 0x48190: ldr r6, [sp, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,124)?)?;
    // 0x48194: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x48198: str r2, [sp, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,124)?)?;
    // 0x4819c: ldr r6, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,2744)?)?;
    // 0x481a0: ldrb lr, [r10, #0xcc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::new(10)?,3272)?)?;
    // 0x481a4: ldr r5, [r6, #0x50c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,1292)?)?;
    // 0x481a8: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // 0x481ac: str r7, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,84)?)?;
    // 0x481b0: beq 0x481c0 <entry+0x21c0>  @ imm = #0x8
    c.branch(C::Eq,0x481c0)?;
    // 0x481b4: ldrb r4, [r6, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,339)?)?;
    // 0x481b8: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x481bc: beq 0x481cc <entry+0x21cc>  @ imm = #0x8
    c.branch(C::Eq,0x481cc)?;
    // 0x481c0: ldr r4, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,680)?)?;
    // 0x481c4: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x481c8: beq 0x482c8 <entry+0x22c8>  @ imm = #0xf8
    c.branch(C::Eq,0x482c8)?;
    // 0x481cc: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x481d0: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x481d4: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x481d8: beq 0x482dc <entry+0x22dc>  @ imm = #0xfc
    c.branch(C::Eq,0x482dc)?;
    // 0x481dc: mov r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,768)?)?;
    // 0x481e0: mov r6, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,12288)?)?;
    // 0x481e4: and r1, r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(5)?,3)?)?;
    // 0x481e8: ldr r2, [r10, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1760)?)?;
    // 0x481ec: and r4, r4, r5, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(4)?,R::new(5)?,Shift::Lsl,4,false)?)?;
    // 0x481f0: and r6, r6, r5, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,6,false)?)?;
    // 0x481f4: orr r4, r4, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x481f8: orr r6, r4, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x481fc: mov r4, #196608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,196608)?)?;
    // 0x48200: and r4, r4, r5, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(4)?,R::new(5)?,Shift::Lsl,8,false)?)?;
    // 0x48204: orr r6, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x48208: mov r4, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3)?)?;
    // 0x4820c: and r5, r4, r5, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsr,2,false)?)?;
    // 0x48210: cmp r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // 0x48214: moveq r5, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x48218: cmp r2, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,384)?)?;
    // 0x4821c: orr r4, r6, r5, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(6)?,R::new(5)?,Shift::Lsl,4,false)?)?;
    // 0x48220: movls r6, r4
    c.emit(arm32::mov_register(C::Ls,R::new(6)?,R::new(4)?))?;
    // 0x48224: cmp r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,3)?)?;
    // 0x48228: movne r6, r4
    c.emit(arm32::mov_register(C::Ne,R::new(6)?,R::new(4)?))?;
    // 0x4822c: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x48230: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x48234: bne 0x48250 <entry+0x2250>  @ imm = #0x14
    c.branch(C::Ne,0x48250)?;
    // 0x48238: ldr r1, [r10, #0x774]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1908)?)?;
    // 0x4823c: cmp r1, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,384)?)?;
    // 0x48240: movgt r1, #1008
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R1,R::R0,1008)?)?;
    // 0x48244: orrgt r1, r1, #208896
    c.emit(arm32::data_immediate(C::Gt,Op::Orr,R::R1,R::R1,208896)?)?;
    // 0x48248: andgt r6, r6, r1
    c.emit(arm32::data_register(C::Gt,Op::And,R::new(6)?,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4824c: strgt r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Gt,false,R::new(6)?,R::SP,560)?)?;
    // 0x48250: and r1, r6, #768
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,768)?)?;
    // 0x48254: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x48258: bne 0x4826c <entry+0x226c>  @ imm = #0xc
    c.branch(C::Ne,0x4826c)?;
    // 0x4825c: ldr r1, [r10, #0x76c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1900)?)?;
    // 0x48260: cmp r1, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,384)?)?;
    // 0x48264: bicgt r6, r6, #768
    c.emit(arm32::data_immediate(C::Gt,Op::Bic,R::new(6)?,R::new(6)?,768)?)?;
    // 0x48268: strgt r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Gt,false,R::new(6)?,R::SP,560)?)?;
    // 0x4826c: and r1, r6, #12288
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,12288)?)?;
    // 0x48270: cmp r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,12288)?)?;
    // 0x48274: bne 0x48288 <entry+0x2288>  @ imm = #0xc
    c.branch(C::Ne,0x48288)?;
    // 0x48278: ldr r1, [r10, #0x770]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1904)?)?;
    // 0x4827c: cmp r1, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,384)?)?;
    // 0x48280: bicgt r6, r6, #12288
    c.emit(arm32::data_immediate(C::Gt,Op::Bic,R::new(6)?,R::new(6)?,12288)?)?;
    // 0x48284: strgt r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Gt,false,R::new(6)?,R::SP,560)?)?;
    // 0x48288: and r1, r6, #196608
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,196608)?)?;
    // 0x4828c: cmp r1, #196608
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,196608)?)?;
    // 0x48290: bne 0x482a4 <entry+0x22a4>  @ imm = #0xc
    c.branch(C::Ne,0x482a4)?;
    // 0x48294: ldr r1, [r10, #0x770]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1904)?)?;
    // 0x48298: cmp r1, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,384)?)?;
    // 0x4829c: bicgt r6, r6, #196608
    c.emit(arm32::data_immediate(C::Gt,Op::Bic,R::new(6)?,R::new(6)?,196608)?)?;
    // 0x482a0: strgt r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Gt,false,R::new(6)?,R::SP,560)?)?;
    // 0x482a4: and r1, r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,3)?)?;
    // 0x482a8: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x482ac: bne 0x482e4 <entry+0x22e4>  @ imm = #0x30
    c.branch(C::Ne,0x482e4)?;
    // 0x482b0: bic r1, r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(6)?,7)?)?;
    // 0x482b4: orr r6, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::R1,4)?)?;
    // 0x482b8: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x482bc: ldr r1, [r10, #0x774]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1908)?)?;
    // 0x482c0: str r1, [sp, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,224)?)?;
    // 0x482c4: b 0x482ec <entry+0x22ec>  @ imm = #0x20
    c.branch(C::Al,0x482ec)?;
    // 0x482c8: ldrb r6, [r6, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(6)?,325)?)?;
    // 0x482cc: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x482d0: beq 0x481cc <entry+0x21cc>  @ imm = #-0x10c
    c.branch(C::Eq,0x481cc)?;
    // 0x482d4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x482d8: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x482dc: mov r6, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4194304)?)?;
    // 0x482e0: b 0x48378 <entry+0x2378>  @ imm = #0x90
    c.branch(C::Al,0x48378)?;
    // 0x482e4: bic r6, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,4)?)?;
    // 0x482e8: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x482ec: and r1, r6, #768
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,768)?)?;
    // 0x482f0: cmp r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,512)?)?;
    // 0x482f4: bne 0x48310 <entry+0x2310>  @ imm = #0x14
    c.branch(C::Ne,0x48310)?;
    // 0x482f8: bic r1, r6, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(6)?,1792)?)?;
    // 0x482fc: orr r6, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::R1,1024)?)?;
    // 0x48300: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x48304: ldr r1, [r10, #0x76c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1900)?)?;
    // 0x48308: str r1, [sp, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,228)?)?;
    // 0x4830c: b 0x48318 <entry+0x2318>  @ imm = #0x4
    c.branch(C::Al,0x48318)?;
    // 0x48310: bic r6, r6, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,1024)?)?;
    // 0x48314: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x48318: and r1, r6, #12288
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,12288)?)?;
    // 0x4831c: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x48320: bne 0x4833c <entry+0x233c>  @ imm = #0x14
    c.branch(C::Ne,0x4833c)?;
    // 0x48324: bic r1, r6, #28672
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(6)?,28672)?)?;
    // 0x48328: orr r6, r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::R1,16384)?)?;
    // 0x4832c: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x48330: ldr r1, [r10, #0x770]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1904)?)?;
    // 0x48334: str r1, [sp, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,232)?)?;
    // 0x48338: b 0x48344 <entry+0x2344>  @ imm = #0x4
    c.branch(C::Al,0x48344)?;
    // 0x4833c: bic r6, r6, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,16384)?)?;
    // 0x48340: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x48344: and r1, r6, #196608
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,196608)?)?;
    // 0x48348: cmp r1, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,131072)?)?;
    // 0x4834c: bne 0x48368 <entry+0x2368>  @ imm = #0x14
    c.branch(C::Ne,0x48368)?;
    // 0x48350: bic r1, r6, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(6)?,458752)?)?;
    // 0x48354: orr r6, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::R1,262144)?)?;
    // 0x48358: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x4835c: ldr r1, [r10, #0x770]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1904)?)?;
    // 0x48360: str r1, [sp, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,236)?)?;
    // 0x48364: b 0x48370 <entry+0x2370>  @ imm = #0x4
    c.branch(C::Al,0x48370)?;
    // 0x48368: bic r6, r6, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,262144)?)?;
    // 0x4836c: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x48370: bic r1, r6, #7340032
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(6)?,7340032)?)?;
    // 0x48374: orr r6, r1, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::R1,4194304)?)?;
    // 0x48378: str r6, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,560)?)?;
    // 0x4837c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x48380: add r7, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,32)?)?;
    // 0x48384: ldreq r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(3)?,0)?)?;
    // 0x48388: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4838c: streq r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(3)?,0)?)?;
    // 0x48390: mov r1, #428
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,428)?)?;
    // 0x48394: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x48398: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x4839c: str r6, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,28)?)?;
    // 0x483a0: add r1, r12, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,4)?)?;
    // 0x483a4: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x483a8: ldr r1, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,104)?)?;
    // 0x483ac: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // 0x483b0: ldr r1, [r10, #0x6bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1724)?)?;
    // 0x483b4: ldr r2, [sp, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,128)?)?;
    // 0x483b8: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x483bc: str r1, [sp, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,128)?)?;
    // 0x483c0: ldr r1, [r10, #0x6c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1728)?)?;
    // 0x483c4: ldr r2, [sp, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,132)?)?;
    // 0x483c8: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x483cc: str r1, [sp, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,132)?)?;
    // 0x483d0: ldr r1, [r10, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1732)?)?;
    // 0x483d4: ldr r2, [sp, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,136)?)?;
    // 0x483d8: add r2, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x483dc: str r2, [sp, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,136)?)?;
    // 0x483e0: ldr r2, [sp, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,140)?)?;
    // 0x483e4: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x483e8: str r1, [sp, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,140)?)?;
    // 0x483ec: ldrb r1, [sp, #0x232]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::SP,562)?)?;
    // 0x483f0: tst r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,64)?)?;
    // 0x483f4: ldrne r1, [r10, #0xb80]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(10)?,2944)?)?;
    // 0x483f8: strne r1, [sp, #0x118]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::SP,280)?)?;
    // 0x483fc: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // 0x48400: ldr r1, [r10, #0xb7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2940)?)?;
    // 0x48404: str r1, [sp, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,184)?)?;
    // 0x48408: ldr r5, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,2744)?)?;
    // 0x4840c: ldr r6, [r5, #0x508]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,1288)?)?;
    // 0x48410: beq 0x48420 <entry+0x2420>  @ imm = #0x8
    c.branch(C::Eq,0x48420)?;
    // 0x48414: ldrb r1, [r5, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,339)?)?;
    // 0x48418: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4841c: beq 0x4842c <entry+0x242c>  @ imm = #0x8
    c.branch(C::Eq,0x4842c)?;
    // 0x48420: ldr r1, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,680)?)?;
    // 0x48424: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x48428: beq 0x484f4 <entry+0x24f4>  @ imm = #0xc4
    c.branch(C::Eq,0x484f4)?;
    // 0x4842c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x48430: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x48434: str r5, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,564)?)?;
    // 0x48438: beq 0x4858c <entry+0x258c>  @ imm = #0x14c
    c.branch(C::Eq,0x4858c)?;
    // 0x4843c: mov r2, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,48)?)?;
    // 0x48440: mov r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12288)?)?;
    // 0x48444: and r9, r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(9)?,R::new(6)?,3)?)?;
    // 0x48448: ldr r3, [r10, #0x6f8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1784)?)?;
    // 0x4844c: mov r4, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3)?)?;
    // 0x48450: ldr lr, [r10, #0x700]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(10)?,1792)?)?;
    // 0x48454: and r2, r2, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x48458: and r1, r1, r6, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsl,4,false)?)?;
    // 0x4845c: orr r2, r2, r9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x48460: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x48464: mov r2, #196608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,196608)?)?;
    // 0x48468: and r2, r2, r6, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsl,6,false)?)?;
    // 0x4846c: cmp r3, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,384)?)?;
    // 0x48470: orr r5, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x48474: and r2, r4, r6, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(4)?,R::new(6)?,Shift::Lsr,4,false)?)?;
    // 0x48478: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x4847c: orrls r1, r1, r2, lsl #8
    c.emit(arm32::data_register(C::Ls,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,8,false)?)?;
    // 0x48480: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x48484: and r2, r4, r6, lsr #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(4)?,R::new(6)?,Shift::Lsr,6,false)?)?;
    // 0x48488: moveq r1, r5
    c.emit(arm32::mov_register(C::Eq,R::R1,R::new(5)?))?;
    // 0x4848c: cmp r3, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,384)?)?;
    // 0x48490: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x48494: orrls r3, r3, r2, lsl #10
    c.emit(arm32::data_register(C::Ls,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,10,false)?)?;
    // 0x48498: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x4849c: moveq r3, r1
    c.emit(arm32::mov_register(C::Eq,R::new(3)?,R::R1))?;
    // 0x484a0: and r1, r4, r6, lsr #12
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::new(6)?,Shift::Lsr,12,false)?)?;
    // 0x484a4: cmp lr, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,384)?)?;
    // 0x484a8: mov r2, r3
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // 0x484ac: orrls r2, r2, r1, lsl #20
    c.emit(arm32::data_register(C::Ls,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,20,false)?)?;
    // 0x484b0: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x484b4: and r1, r4, r6, lsr #14
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::new(6)?,Shift::Lsr,14,false)?)?;
    // 0x484b8: moveq r2, r3
    c.emit(arm32::mov_register(C::Eq,R::R2,R::new(3)?))?;
    // 0x484bc: cmp lr, #384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,384)?)?;
    // 0x484c0: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x484c4: orrls r6, r6, r1, lsl #22
    c.emit(arm32::data_register(C::Ls,Op::Orr,R::new(6)?,R::new(6)?,R::R1,Shift::Lsl,22,false)?)?;
    // 0x484c8: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x484cc: moveq r6, r2
    c.emit(arm32::mov_register(C::Eq,R::new(6)?,R::R2))?;
    // 0x484d0: cmp r9, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,2)?)?;
    // 0x484d4: str r6, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,564)?)?;
    // 0x484d8: bne 0x48508 <entry+0x2508>  @ imm = #0x28
    c.branch(C::Ne,0x48508)?;
    // 0x484dc: bic r1, r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(6)?,7)?)?;
    // 0x484e0: orr r6, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::R1,4)?)?;
    // 0x484e4: str r6, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,564)?)?;
    // 0x484e8: ldr r1, [r10, #0x6f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1780)?)?;
    // 0x484ec: str r1, [sp, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,240)?)?;
    // 0x484f0: b 0x48510 <entry+0x2510>  @ imm = #0x18
    c.branch(C::Al,0x48510)?;
    // 0x484f4: ldrb r1, [r5, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,325)?)?;
    // 0x484f8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x484fc: beq 0x4842c <entry+0x242c>  @ imm = #-0xd8
    c.branch(C::Eq,0x4842c)?;
    // 0x48500: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x48504: b 0x48588 <entry+0x2588>  @ imm = #0x7c
    c.branch(C::Al,0x48588)?;
    // 0x48508: bic r6, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,4)?)?;
    // 0x4850c: str r6, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,564)?)?;
    // 0x48510: and r1, r6, #48
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,48)?)?;
    // 0x48514: cmp r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,32)?)?;
    // 0x48518: bne 0x48534 <entry+0x2534>  @ imm = #0x14
    c.branch(C::Ne,0x48534)?;
    // 0x4851c: bic r1, r6, #112
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(6)?,112)?)?;
    // 0x48520: orr r6, r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::R1,64)?)?;
    // 0x48524: str r6, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,564)?)?;
    // 0x48528: ldr r1, [r10, #0x6f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1780)?)?;
    // 0x4852c: str r1, [sp, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,244)?)?;
    // 0x48530: b 0x4853c <entry+0x253c>  @ imm = #0x4
    c.branch(C::Al,0x4853c)?;
    // 0x48534: bic r6, r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,64)?)?;
    // 0x48538: str r6, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,564)?)?;
    // 0x4853c: and r1, r6, #12288
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,12288)?)?;
    // 0x48540: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x48544: bne 0x48560 <entry+0x2560>  @ imm = #0x14
    c.branch(C::Ne,0x48560)?;
    // 0x48548: bic r1, r6, #28672
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(6)?,28672)?)?;
    // 0x4854c: orr r6, r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::R1,16384)?)?;
    // 0x48550: str r6, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,564)?)?;
    // 0x48554: ldr r1, [r10, #0x6fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1788)?)?;
    // 0x48558: str r1, [sp, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,248)?)?;
    // 0x4855c: b 0x48568 <entry+0x2568>  @ imm = #0x4
    c.branch(C::Al,0x48568)?;
    // 0x48560: bic r6, r6, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,16384)?)?;
    // 0x48564: str r6, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,564)?)?;
    // 0x48568: and r1, r6, #196608
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,196608)?)?;
    // 0x4856c: cmp r1, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,131072)?)?;
    // 0x48570: bne 0x48584 <entry+0x2584>  @ imm = #0xc
    c.branch(C::Ne,0x48584)?;
    // 0x48574: bic r1, r6, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::new(6)?,458752)?)?;
    // 0x48578: str lr, [sp, #0xfc]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,252)?)?;
    // 0x4857c: orr r5, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R1,262144)?)?;
    // 0x48580: b 0x48588 <entry+0x2588>  @ imm = #0x0
    c.branch(C::Al,0x48588)?;
    // 0x48584: bic r5, r6, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(6)?,262144)?)?;
    // 0x48588: str r5, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,564)?)?;
    // 0x4858c: tst r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,31)?)?;
    // 0x48590: ldreq r1, [r7]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(7)?,0)?)?;
    // 0x48594: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48598: streq r1, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(7)?,0)?)?;
    // 0x4859c: mov r1, #2480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2480)?)?;
    // 0x485a0: add r7, sp, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::SP,108)?)?;
    // 0x485a4: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x485a8: str r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // 0x485ac: str r5, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,36)?)?;
    // 0x485b0: add r1, r12, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,5)?)?;
    // 0x485b4: add r0, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,40)?)?;
    // 0x485b8: ldr lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x485bc: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x485c0: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x485c4: ldr r0, [r10, #0x6c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1736)?)?;
    // 0x485c8: str r0, [sp, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,144)?)?;
    // 0x485cc: str r0, [sp, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,148)?)?;
    // 0x485d0: ldr r0, [r10, #0x6d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1744)?)?;
    // 0x485d4: str r0, [sp, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,152)?)?;
    // 0x485d8: str r0, [sp, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,156)?)?;
    // 0x485dc: ldr r0, [r10, #0x6d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1748)?)?;
    // 0x485e0: str r0, [sp, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,160)?)?;
    // 0x485e4: str r0, [sp, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,164)?)?;
    // 0x485e8: ldr r0, [r10, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,664)?)?;
    // 0x485ec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x485f0: beq 0x48638 <entry+0x2638>  @ imm = #0x40
    c.branch(C::Eq,0x48638)?;
    // 0x485f4: ldr r2, [r10, #0xb8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2956)?)?;
    // 0x485f8: add r1, r10, #916
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,916)?)?;
    // 0x485fc: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x48600: mov r3, #80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,80)?)?;
    // 0x48604: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x48608: add r4, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R1,2048)?)?;
    // 0x4860c: tst r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,255)?)?;
    // 0x48610: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x48614: add r6, r3, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(3)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x48618: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4861c: addeq r1, r10, #2960
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::new(10)?,2960)?)?;
    // 0x48620: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x48624: str r2, [r7, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(7)?,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x48628: str r1, [r8, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(8)?,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x4862c: and r6, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(5)?,255)?)?;
    // 0x48630: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x48634: bhi 0x4860c <entry+0x260c>  @ imm = #-0x30
    c.branch(C::Hi,0x4860c)?;
    // 0x48638: ldr r6, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,300)?)?;
    // 0x4863c: ldr r1, [sp, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,436)?)?;
    // 0x48640: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x48644: and r0, r0, r6, lsr #14
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsr,14,false)?)?;
    // 0x48648: and r4, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::R1,7)?)?;
    // 0x4864c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x48650: beq 0x48670 <entry+0x2670>  @ imm = #0x18
    c.branch(C::Eq,0x48670)?;
    // 0x48654: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x48658: beq 0x4867c <entry+0x267c>  @ imm = #0x1c
    c.branch(C::Eq,0x4867c)?;
    // 0x4865c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x48660: bne 0x48688 <entry+0x2688>  @ imm = #0x20
    c.branch(C::Ne,0x48688)?;
    // 0x48664: mov r5, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,18)?)?;
    // 0x48668: mov r7, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,10)?)?;
    // 0x4866c: b 0x486a8 <entry+0x26a8>  @ imm = #0x34
    c.branch(C::Al,0x486a8)?;
    // 0x48670: mov r5, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x48674: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x48678: b 0x486a8 <entry+0x26a8>  @ imm = #0x28
    c.branch(C::Al,0x486a8)?;
    // 0x4867c: mov r5, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,10)?)?;
    // 0x48680: mov r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,6)?)?;
    // 0x48684: b 0x486a8 <entry+0x26a8>  @ imm = #0x1c
    c.branch(C::Al,0x486a8)?;
    // 0x48688: ldr r2, [pc, #0x398]        @ 0x2a28 <entry+0x2a28>
    c.literal(C::Al,R::R2,0x48a28)?;
    // 0x4868c: mov r1, #1872
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1872)?)?;
    // 0x48690: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x48694: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x48698: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x4869c: bl 0x17b0 <entry+0xfffffffffffbb7b0> @ imm = #-0x46ef4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x486a0: ldr lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x486a4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x486a8: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x486ac: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x486b0: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x486b4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x486b8: bne 0x48708 <entry+0x2708>  @ imm = #0x48
    c.branch(C::Ne,0x48708)?;
    // 0x486bc: ldrb r1, [r10, #0x3b9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,953)?)?;
    // 0x486c0: lsr r0, r6, #14
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(6)?,Shift::Lsr,14)?)?;
    // 0x486c4: and r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,3)?)?;
    // 0x486c8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x486cc: beq 0x486e8 <entry+0x26e8>  @ imm = #0x14
    c.branch(C::Eq,0x486e8)?;
    // 0x486d0: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x486d4: bne 0x48708 <entry+0x2708>  @ imm = #0x2c
    c.branch(C::Ne,0x48708)?;
    // 0x486d8: mov r1, #904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,904)?)?;
    // 0x486dc: orr r1, r1, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,9216)?)?;
    // 0x486e0: b 0x486f8 <entry+0x26f8>  @ imm = #0x10
    c.branch(C::Al,0x486f8)?;
    // 0x486e4: andeq r0, r1, r1, lsl #4
    c.emit(arm32::data_register(C::Eq,Op::And,R::R0,R::R1,R::R1,Shift::Lsl,4,false)?)?;
    // 0x486e8: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x486ec: bne 0x48708 <entry+0x2708>  @ imm = #0x14
    c.branch(C::Ne,0x48708)?;
    // 0x486f0: mov r1, #101
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,101)?)?;
    // 0x486f4: orr r1, r1, #9984
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,9984)?)?;
    // 0x486f8: ldr r2, [pc, #0x328]        @ 0x2a28 <entry+0x2a28>
    c.literal(C::Al,R::R2,0x48a28)?;
    // 0x486fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x48700: bl 0x17b0 <entry+0xfffffffffffbb7b0> @ imm = #-0x46f58
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x48704: ldr lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x48708: ldrb r0, [sp, #0x134]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::SP,308)?)?;
    // 0x4870c: smulbb r2, r7, r4
    c.emit(signed_multiply_bottom_halves(C::Al,R::R2,R::new(7)?,R::new(4)?)?)?;
    // 0x48710: smulbb r3, r5, r4
    c.emit(signed_multiply_bottom_halves(C::Al,R::new(3)?,R::new(5)?,R::new(4)?)?)?;
    // 0x48714: mov r5, #14848
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,14848)?)?;
    // 0x48718: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x4871c: addne r0, r2, r2, lsl #1
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    // 0x48720: lsrne r2, r0, #1
    c.emit(arm32::mov_shifted_register(C::Ne,R::R2,R::R0,Shift::Lsr,1)?)?;
    // 0x48724: addne r0, r3, r3, lsl #1
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::new(3)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x48728: lsrne r3, r0, #1
    c.emit(arm32::mov_shifted_register(C::Ne,R::new(3)?,R::R0,Shift::Lsr,1)?)?;
    // 0x4872c: ldr r0, [lr]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,0)?)?;
    // 0x48730: lsl r6, r3, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(3)?,Shift::Lsl,16)?)?;
    // 0x48734: add r12, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x48738: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4873c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x48740: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48744: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x48748: stm r0, {r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::new(5)?.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4874c: ldr r7, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-36)?)?;
    // 0x48750: add r4, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(7)?,1)?)?;
    // 0x48754: str r4, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-36)?)?;
    // 0x48758: add r4, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,8)?)?;
    // 0x4875c: str r4, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x48760: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x48764: ldreq r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(4)?,0)?)?;
    // 0x48768: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4876c: streq r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,0)?)?;
    // 0x48770: str r5, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,8)?)?;
    // 0x48774: add r5, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,16)?)?;
    // 0x48778: add r1, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,2)?)?;
    // 0x4877c: str r6, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,12)?)?;
    // 0x48780: orr r6, r3, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,16,false)?)?;
    // 0x48784: add r4, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,24)?)?;
    // 0x48788: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4878c: str r5, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::LR,0)?)?;
    // 0x48790: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x48794: ldreq r1, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,0)?)?;
    // 0x48798: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4879c: streq r1, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(5)?,0)?)?;
    // 0x487a0: mov r5, #516
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,516)?)?;
    // 0x487a4: add r1, r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,3)?)?;
    // 0x487a8: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x487ac: orr r5, r5, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,14336)?)?;
    // 0x487b0: str r5, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,16)?)?;
    // 0x487b4: str r6, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,20)?)?;
    // 0x487b8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x487bc: str r4, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x487c0: ldreq r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(4)?,0)?)?;
    // 0x487c4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x487c8: streq r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,0)?)?;
    // 0x487cc: str r5, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,24)?)?;
    // 0x487d0: str r6, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,28)?)?;
    // 0x487d4: add r1, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,4)?)?;
    // 0x487d8: add r6, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,32)?)?;
    // 0x487dc: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x487e0: str r6, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::LR,0)?)?;
    // 0x487e4: ldr r1, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,88)?)?;
    // 0x487e8: ldr r1, [r1, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,2744)?)?;
    // 0x487ec: ldrb r1, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,339)?)?;
    // 0x487f0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x487f4: bne 0x48a30 <entry+0x2a30>  @ imm = #0x234
    c.branch(C::Ne,0x48a30)?;
    // 0x487f8: ldr r1, [sp, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,324)?)?;
    // 0x487fc: ldrb r2, [sp, #0x134]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::SP,308)?)?;
    // 0x48800: mov r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // 0x48804: mov r9, #31
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,31)?)?;
    // 0x48808: str r12, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,100)?)?;
    // 0x4880c: ldr r4, [sp, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,444)?)?;
    // 0x48810: ands r1, r1, #16
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R1,16,true)?)?;
    // 0x48814: moveq r3, #128
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // 0x48818: tst r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,1)?)?;
    // 0x4881c: ldr r2, [sp, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,440)?)?;
    // 0x48820: eor r1, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Eor,R::R1,R::R1,16)?)?;
    // 0x48824: addne r3, r1, #48
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(3)?,R::R1,48)?)?;
    // 0x48828: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x4882c: and r5, r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R2,7)?)?;
    // 0x48830: and r1, r9, r2, lsr #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::R2,Shift::Lsr,16,false)?)?;
    // 0x48834: and r2, r9, r2, lsr #24
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(9)?,R::R2,Shift::Lsr,24,false)?)?;
    // 0x48838: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4883c: smulbb r1, r5, r1
    c.emit(signed_multiply_bottom_halves(C::Al,R::R1,R::new(5)?,R::R1)?)?;
    // 0x48840: smulbb r2, r5, r2
    c.emit(signed_multiply_bottom_halves(C::Al,R::R2,R::new(5)?,R::R2)?)?;
    // 0x48844: and r5, r4, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(4)?,7)?)?;
    // 0x48848: smulbb r1, r1, r3
    c.emit(signed_multiply_bottom_halves(C::Al,R::R1,R::R1,R::new(3)?)?)?;
    // 0x4884c: smulbb lr, r2, r3
    c.emit(signed_multiply_bottom_halves(C::Al,R::LR,R::R2,R::new(3)?)?)?;
    // 0x48850: and r2, r9, r4, lsr #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(9)?,R::new(4)?,Shift::Lsr,16,false)?)?;
    // 0x48854: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x48858: lsr r1, r1, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,5)?)?;
    // 0x4885c: lsr r9, lr, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::LR,Shift::Lsr,5)?)?;
    // 0x48860: str r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,56)?)?;
    // 0x48864: orr r10, r12, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(10)?,R::R12,R::R1,Shift::Lsl,16,false)?)?;
    // 0x48868: smulbb r12, r2, r3
    c.emit(signed_multiply_bottom_halves(C::Al,R::R12,R::R2,R::new(3)?)?)?;
    // 0x4886c: add r2, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,40)?)?;
    // 0x48870: ldreq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,0)?)?;
    // 0x48874: smulbb r12, r12, r5
    c.emit(signed_multiply_bottom_halves(C::Al,R::R12,R::R12,R::new(5)?)?)?;
    // 0x48878: lsr r12, r12, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R12,R::R12,Shift::Lsr,5)?)?;
    // 0x4887c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48880: streq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,0)?)?;
    // 0x48884: mov r1, #520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,520)?)?;
    // 0x48888: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x4888c: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x48890: str r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // 0x48894: str r10, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,36)?)?;
    // 0x48898: add r1, r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,5)?)?;
    // 0x4889c: ldr lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x488a0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x488a4: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x488a8: ldreq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x488ac: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x488b0: streq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x488b4: mov r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,31)?)?;
    // 0x488b8: add r2, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,48)?)?;
    // 0x488bc: str r5, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,60)?)?;
    // 0x488c0: and r6, r1, r4, lsr #24
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R1,R::new(4)?,Shift::Lsr,24,false)?)?;
    // 0x488c4: mov r1, #568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,568)?)?;
    // 0x488c8: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x488cc: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x488d0: smulbb r3, r6, r3
    c.emit(signed_multiply_bottom_halves(C::Al,R::new(3)?,R::new(6)?,R::new(3)?)?)?;
    // 0x488d4: add r6, r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,56)?)?;
    // 0x488d8: str r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,40)?)?;
    // 0x488dc: add r1, r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,6)?)?;
    // 0x488e0: str r10, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,44)?)?;
    // 0x488e4: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x488e8: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x488ec: ldr r1, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,100)?)?;
    // 0x488f0: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,56)?)?;
    // 0x488f4: add r5, r4, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x488f8: ldreq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x488fc: orr r4, r5, r9, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(5)?,R::new(9)?,Shift::Lsl,16,false)?)?;
    // 0x48900: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48904: streq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x48908: mov r1, #524
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,524)?)?;
    // 0x4890c: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x48910: add r2, r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,72)?)?;
    // 0x48914: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x48918: str r1, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,48)?)?;
    // 0x4891c: add r1, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,7)?)?;
    // 0x48920: str r4, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,52)?)?;
    // 0x48924: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x48928: str r6, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::LR,0)?)?;
    // 0x4892c: ldreq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,0)?)?;
    // 0x48930: ldr r10, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,88)?)?;
    // 0x48934: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48938: streq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,0)?)?;
    // 0x4893c: add r6, r5, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(5)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x48940: ldr r1, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,60)?)?;
    // 0x48944: orr r5, r6, r12, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(6)?,R::R12,Shift::Lsl,16,false)?)?;
    // 0x48948: add r6, r6, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x4894c: smulbb r3, r3, r1
    c.emit(signed_multiply_bottom_halves(C::Al,R::new(3)?,R::new(3)?,R::R1)?)?;
    // 0x48950: mov r1, #572
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,572)?)?;
    // 0x48954: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x48958: lsr r3, r3, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsr,5)?)?;
    // 0x4895c: str r1, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,56)?)?;
    // 0x48960: str r4, [r0, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,60)?)?;
    // 0x48964: add r1, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,8)?)?;
    // 0x48968: add r4, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,64)?)?;
    // 0x4896c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x48970: str r4, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x48974: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x48978: ldreq r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(4)?,0)?)?;
    // 0x4897c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48980: streq r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,0)?)?;
    // 0x48984: mov r4, #2576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,2576)?)?;
    // 0x48988: add r1, r7, #9
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,9)?)?;
    // 0x4898c: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x48990: orr r4, r4, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,12288)?)?;
    // 0x48994: str r4, [r0, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,64)?)?;
    // 0x48998: str r5, [r0, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,68)?)?;
    // 0x4899c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x489a0: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x489a4: ldreq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x489a8: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x489ac: streq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x489b0: str r4, [r0, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,72)?)?;
    // 0x489b4: str r5, [r0, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,76)?)?;
    // 0x489b8: add r1, r7, #10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,10)?)?;
    // 0x489bc: add r5, r0, #80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,80)?)?;
    // 0x489c0: orr r2, r6, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(6)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x489c4: add r4, r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,88)?)?;
    // 0x489c8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x489cc: str r5, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::LR,0)?)?;
    // 0x489d0: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x489d4: ldreq r1, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,0)?)?;
    // 0x489d8: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x489dc: streq r1, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(5)?,0)?)?;
    // 0x489e0: mov r5, #532
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,532)?)?;
    // 0x489e4: add r1, r7, #11
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,11)?)?;
    // 0x489e8: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x489ec: orr r5, r5, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,14336)?)?;
    // 0x489f0: str r5, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,80)?)?;
    // 0x489f4: str r2, [r0, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,84)?)?;
    // 0x489f8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x489fc: str r4, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x48a00: ldreq r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(4)?,0)?)?;
    // 0x48a04: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48a08: streq r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,0)?)?;
    // 0x48a0c: add r1, r7, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,12)?)?;
    // 0x48a10: str r5, [r0, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,88)?)?;
    // 0x48a14: str r2, [r0, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,92)?)?;
    // 0x48a18: add r2, r0, #96
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,96)?)?;
    // 0x48a1c: b 0x48b14 <entry+0x2b14>  @ imm = #0xf0
    c.branch(C::Al,0x48b14)?;
    c.check(0x48a20)?;
    for data in CODEC_CONTROLLER_MASKS {c.emit(data)?;}
    c.check(0x48a30)?;
    // 0x48a30: ldr r1, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,300)?)?;
    // 0x48a34: mov r3, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,48)?)?;
    // 0x48a38: add r2, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,40)?)?;
    // 0x48a3c: ands r5, r1, #49152
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(5)?,R::R1,49152,true)?)?;
    // 0x48a40: moveq r3, #16
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // 0x48a44: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x48a48: ldreq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(6)?,0)?)?;
    // 0x48a4c: orr r4, r12, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R12,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x48a50: add r3, r3, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x48a54: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48a58: streq r1, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,0)?)?;
    // 0x48a5c: mov r1, #520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,520)?)?;
    // 0x48a60: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x48a64: mov r6, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,24)?)?;
    // 0x48a68: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x48a6c: str r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // 0x48a70: add r1, r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,5)?)?;
    // 0x48a74: str r4, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,36)?)?;
    // 0x48a78: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x48a7c: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x48a80: ldreq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x48a84: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48a88: streq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x48a8c: mov r1, #568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,568)?)?;
    // 0x48a90: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x48a94: add r5, r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,56)?)?;
    // 0x48a98: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x48a9c: moveq r6, #8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,8)?)?;
    // 0x48aa0: str r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,40)?)?;
    // 0x48aa4: str r4, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,44)?)?;
    // 0x48aa8: add r1, r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,6)?)?;
    // 0x48aac: add r4, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,48)?)?;
    // 0x48ab0: orr r2, r3, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x48ab4: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x48ab8: str r4, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x48abc: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x48ac0: ldreq r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(4)?,0)?)?;
    // 0x48ac4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48ac8: streq r1, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,0)?)?;
    // 0x48acc: mov r1, #524
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,524)?)?;
    // 0x48ad0: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x48ad4: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x48ad8: str r1, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,48)?)?;
    // 0x48adc: add r1, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,7)?)?;
    // 0x48ae0: str r2, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,52)?)?;
    // 0x48ae4: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x48ae8: str r5, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::LR,0)?)?;
    // 0x48aec: ldreq r1, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,0)?)?;
    // 0x48af0: ldr r10, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,88)?)?;
    // 0x48af4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x48af8: streq r1, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(5)?,0)?)?;
    // 0x48afc: mov r1, #572
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,572)?)?;
    // 0x48b00: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x48b04: str r1, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,56)?)?;
    // 0x48b08: str r2, [r0, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,60)?)?;
    // 0x48b0c: add r1, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,8)?)?;
    // 0x48b10: add r2, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,64)?)?;
    // 0x48b14: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x48b18: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x48b1c: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x48b20: add r0, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x48b24: add r7, sp, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::SP,108)?)?;
    // 0x48b28: ldreq r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R2,0)?)?;
    // 0x48b2c: ldr r5, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,80)?)?;
    // 0x48b30: ldr r4, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,92)?)?;
    // 0x48b34: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x48b38: orr r3, r0, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x48b3c: add r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x48b40: streq r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,0)?)?;
    // 0x48b44: mov r6, #2608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2608)?)?;
    // 0x48b48: add r9, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x48b4c: orr r6, r6, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,12288)?)?;
    // 0x48b50: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // 0x48b54: str r3, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,4)?)?;
    // 0x48b58: add r3, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,1)?)?;
    // 0x48b5c: add r6, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,8)?)?;
    // 0x48b60: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x48b64: str r6, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::LR,0)?)?;
    // 0x48b68: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x48b6c: orr r3, r0, r4, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x48b70: ldreq r5, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::new(6)?,0)?)?;
    // 0x48b74: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x48b78: streq r5, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::new(6)?,0)?)?;
    // 0x48b7c: mov r6, #564
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,564)?)?;
    // 0x48b80: add r5, r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R2,16)?)?;
    // 0x48b84: orr r6, r6, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,14336)?)?;
    // 0x48b88: str r6, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,8)?)?;
    // 0x48b8c: str r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,12)?)?;
    // 0x48b90: add r3, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,2)?)?;
    // 0x48b94: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x48b98: str r5, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::LR,0)?)?;
    // 0x48b9c: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x48ba0: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x48ba4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x48ba8: bne 0x48cf0 <entry+0x2cf0>  @ imm = #0x140
    c.branch(C::Ne,0x48cf0)?;
    // 0x48bac: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x48bb0: ldr r3, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,68)?)?;
    // 0x48bb4: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x48bb8: mul r6, r3, r0
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::new(3)?,R::R0)?)?;
    // 0x48bbc: ldreq r3, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(5)?,0)?)?;
    // 0x48bc0: orr r0, r9, #6291456
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,6291456)?)?;
    // 0x48bc4: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x48bc8: streq r3, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::new(5)?,0)?)?;
    // 0x48bcc: mov r5, #580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,580)?)?;
    // 0x48bd0: ldr r3, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,76)?)?;
    // 0x48bd4: ldr r4, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,72)?)?;
    // 0x48bd8: orr r5, r5, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,14336)?)?;
    // 0x48bdc: str r5, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,16)?)?;
    // 0x48be0: str r0, [r2, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,20)?)?;
    // 0x48be4: add r0, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,3)?)?;
    // 0x48be8: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x48bec: add r0, r9, #96
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,96)?)?;
    // 0x48bf0: orr r5, r0, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R0,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x48bf4: add r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x48bf8: mul r12, r4, r3
    c.emit(arm32::multiply_register(C::Al,R::R12,R::new(4)?,R::new(3)?)?)?;
    // 0x48bfc: add r4, r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R2,24)?)?;
    // 0x48c00: str r4, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x48c04: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x48c08: add r9, r0, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // 0x48c0c: ldreq r3, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(4)?,0)?)?;
    // 0x48c10: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x48c14: streq r3, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::new(4)?,0)?)?;
    // 0x48c18: mov r3, #2640
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2640)?)?;
    // 0x48c1c: add r4, r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R2,32)?)?;
    // 0x48c20: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // 0x48c24: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x48c28: str r3, [r2, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,24)?)?;
    // 0x48c2c: add r3, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,4)?)?;
    // 0x48c30: str r5, [r2, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,28)?)?;
    // 0x48c34: orr r5, r0, r12, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R0,R::R12,Shift::Lsl,16,false)?)?;
    // 0x48c38: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x48c3c: str r4, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::LR,0)?)?;
    // 0x48c40: ldreq r3, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(4)?,0)?)?;
    // 0x48c44: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x48c48: streq r3, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::new(4)?,0)?)?;
    // 0x48c4c: mov r3, #600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,600)?)?;
    // 0x48c50: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x48c54: str r3, [r2, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,32)?)?;
    // 0x48c58: str r5, [r2, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,36)?)?;
    // 0x48c5c: add r3, r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,5)?)?;
    // 0x48c60: add r5, r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R2,40)?)?;
    // 0x48c64: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x48c68: str r5, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::LR,0)?)?;
    // 0x48c6c: ldr r0, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,96)?)?;
    // 0x48c70: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x48c74: blo 0x48cf0 <entry+0x2cf0>  @ imm = #0x74
    c.branch(C::Cc,0x48cf0)?;
    // 0x48c78: lsl r3, r6, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(6)?,Shift::Lsl,16)?)?;
    // 0x48c7c: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x48c80: lsl r0, r12, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R12,Shift::Lsl,16)?)?;
    // 0x48c84: add r6, r9, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(9)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x48c88: orr r4, r9, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(9)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x48c8c: ldreq r3, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(5)?,0)?)?;
    // 0x48c90: orr r0, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x48c94: add r9, r6, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x48c98: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x48c9c: streq r3, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::new(5)?,0)?)?;
    // 0x48ca0: mov r3, #596
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,596)?)?;
    // 0x48ca4: add r5, r2, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R2,48)?)?;
    // 0x48ca8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x48cac: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x48cb0: str r3, [r2, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,40)?)?;
    // 0x48cb4: add r3, r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,6)?)?;
    // 0x48cb8: str r4, [r2, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,44)?)?;
    // 0x48cbc: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x48cc0: str r5, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::LR,0)?)?;
    // 0x48cc4: ldreq r3, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(5)?,0)?)?;
    // 0x48cc8: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x48ccc: streq r3, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::new(5)?,0)?)?;
    // 0x48cd0: mov r3, #604
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,604)?)?;
    // 0x48cd4: add r5, r2, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R2,56)?)?;
    // 0x48cd8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x48cdc: str r3, [r2, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,48)?)?;
    // 0x48ce0: str r0, [r2, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,52)?)?;
    // 0x48ce4: add r0, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,7)?)?;
    // 0x48ce8: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x48cec: str r5, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::LR,0)?)?;
    // 0x48cf0: ldr r0, [r10, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,664)?)?;
    // 0x48cf4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x48cf8: blo 0x48da0 <entry+0x2da0>  @ imm = #0xa0
    c.branch(C::Cc,0x48da0)?;
    // 0x48cfc: mov r2, #2768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2768)?)?;
    // 0x48d00: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x48d04: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x48d08: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x48d0c: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x48d10: ldr r6, [r3], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(6)?,R::new(3)?,4)?)?;
    // 0x48d14: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x48d18: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x48d1c: ldreq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::R0,0)?)?;
    // 0x48d20: add r6, r6, r6, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x48d24: add r6, r6, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,15)?)?;
    // 0x48d28: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x48d2c: lsr r6, r6, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(6)?,Shift::Lsr,4)?)?;
    // 0x48d30: streq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::R0,0)?)?;
    // 0x48d34: cmp r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,13)?)?;
    // 0x48d38: stm r0, {r2, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x48d3c: add r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
    // 0x48d40: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x48d44: ldr r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-36)?)?;
    // 0x48d48: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x48d4c: str r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-36)?)?;
    // 0x48d50: bne 0x48d10 <entry+0x2d10>  @ imm = #-0x48
    c.branch(C::Ne,0x48d10)?;
    // 0x48d54: add r0, r5, #104
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,104)?)?;
    // 0x48d58: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x48d5c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x48d60: ldr r1, [sp, #0xb8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,184)?)?;
    // 0x48d64: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x48d68: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // 0x48d6c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x48d70: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // 0x48d74: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x48d78: mov r0, #780
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,780)?)?;
    // 0x48d7c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x48d80: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // 0x48d84: str r0, [r5, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,104)?)?;
    // 0x48d88: str r1, [r5, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,108)?)?;
    // 0x48d8c: add r5, r5, #112
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,112)?)?;
    // 0x48d90: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x48d94: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x48d98: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x48d9c: str r5, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::LR,0)?)?;
    // 0x48da0: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x48da4: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x48da8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x48dac: beq 0x48e14 <entry+0x2e14>  @ imm = #0x60
    c.branch(C::Eq,0x48e14)?;
    // 0x48db0: mov r3, #772
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,772)?)?;
    // 0x48db4: add r1, r7, #52
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,52)?)?;
    // 0x48db8: mov r2, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,13)?)?;
    // 0x48dbc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x48dc0: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x48dc4: ldr r6, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,0)?)?;
    // 0x48dc8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x48dcc: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x48dd0: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // 0x48dd4: ldreq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::R0,0)?)?;
    // 0x48dd8: add r6, r6, r6, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x48ddc: add r6, r6, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,15)?)?;
    // 0x48de0: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x48de4: lsr r6, r6, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(6)?,Shift::Lsr,4)?)?;
    // 0x48de8: streq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::R0,0)?)?;
    // 0x48dec: cmp r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,15)?)?;
    // 0x48df0: stm r0, {r3, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::new(3)?.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x48df4: add r3, r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4)?)?;
    // 0x48df8: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x48dfc: ldr r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-36)?)?;
    // 0x48e00: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x48e04: str r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-36)?)?;
    // 0x48e08: bne 0x48dc4 <entry+0x2dc4>  @ imm = #-0x4c
    c.branch(C::Ne,0x48dc4)?;
    // 0x48e0c: add r0, r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,16)?)?;
    // 0x48e10: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x48e14: ldr r0, [r10, #0x29c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,668)?)?;
    // 0x48e18: and r7, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R0,255)?)?;
    // 0x48e1c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x48e20: lsl r1, r7, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(7)?,Shift::Lsl,7)?)?;
    // 0x48e24: str r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,92)?)?;
    // 0x48e28: beq 0x48e50 <entry+0x2e50>  @ imm = #0x20
    c.branch(C::Eq,0x48e50)?;
    // 0x48e2c: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x48e30: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x48e34: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // 0x48e38: beq 0x48e58 <entry+0x2e58>  @ imm = #0x18
    c.branch(C::Eq,0x48e58)?;
    // 0x48e3c: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x48e40: bne 0x48e60 <entry+0x2e60>  @ imm = #0x18
    c.branch(C::Ne,0x48e60)?;
    // 0x48e44: mov r4, #476
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,476)?)?;
    // 0x48e48: orr r4, r4, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,1024)?)?;
    // 0x48e4c: b 0x48e80 <entry+0x2e80>  @ imm = #0x2c
    c.branch(C::Al,0x48e80)?;
    // 0x48e50: mov r4, #1872
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1872)?)?;
    // 0x48e54: b 0x48e78 <entry+0x2e78>  @ imm = #0x1c
    c.branch(C::Al,0x48e78)?;
    // 0x48e58: mov r4, #1696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1696)?)?;
    // 0x48e5c: b 0x48e80 <entry+0x2e80>  @ imm = #0x1c
    c.branch(C::Al,0x48e80)?;
    // 0x48e60: ldr r2, [pc, #-0x440]       @ 0x2a28 <entry+0x2a28>
    c.literal(C::Al,R::R2,0x48a28)?;
    // 0x48e64: mov r1, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    // 0x48e68: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x48e6c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x48e70: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x48e74: bl 0x17b0 <entry+0xfffffffffffbb7b0> @ imm = #-0x476cc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x48e78: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x48e7c: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // 0x48e80: lsl r0, r7, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(7)?,Shift::Lsl,16)?)?;
    // 0x48e84: str r4, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,96)?)?;
    // 0x48e88: str r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,100)?)?;
    // 0x48e8c: ldr r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,92)?)?;
    // 0x48e90: sub r10, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(10)?,R::R0,1)?)?;
    // 0x48e94: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x48e98: ldr r1, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,88)?)?;
    // 0x48e9c: add r2, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,4)?)?;
    // 0x48ea0: ldr r1, [r1, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,2744)?)?;
    // 0x48ea4: ldrb r1, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,339)?)?;
    // 0x48ea8: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x48eac: mov r8, r2
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R2))?;
    // 0x48eb0: cmp r6, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,13)?)?;
    // 0x48eb4: blo 0x48ec0 <entry+0x2ec0>  @ imm = #0x4
    c.branch(C::Cc,0x48ec0)?;
    // 0x48eb8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x48ebc: beq 0x4900c <entry+0x300c>  @ imm = #0x148
    c.branch(C::Eq,0x4900c)?;
    // 0x48ec0: ldr r0, [r8, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,-4)?)?;
    // 0x48ec4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x48ec8: bne 0x48f7c <entry+0x2f7c>  @ imm = #0xac
    c.branch(C::Ne,0x48f7c)?;
    // 0x48ecc: add r2, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,4)?)?;
    // 0x48ed0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x48ed4: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x48ed8: beq 0x48ea8 <entry+0x2ea8>  @ imm = #-0x38
    c.branch(C::Eq,0x48ea8)?;
    // 0x48edc: add r0, sp, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,108)?)?;
    // 0x48ee0: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x48ee4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x48ee8: beq 0x49000 <entry+0x3000>  @ imm = #0x110
    c.branch(C::Eq,0x49000)?;
    // 0x48eec: ldr r1, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,88)?)?;
    // 0x48ef0: ldr r1, [r1, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,664)?)?;
    // 0x48ef4: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x48ef8: blo 0x49000 <entry+0x3000>  @ imm = #0x100
    c.branch(C::Cc,0x49000)?;
    // 0x48efc: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x48f00: mul r2, r0, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // 0x48f04: ldr r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
    // 0x48f08: add r0, r2, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    // 0x48f0c: add r0, r10, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x48f10: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3ad34
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x48f14: ldr r5, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,100)?)?;
    // 0x48f18: ldr r3, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,104)?)?;
    // 0x48f1c: mov r1, #2672
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2672)?)?;
    // 0x48f20: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x48f24: add r2, r1, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x48f28: mul r1, r5, r0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(5)?,R::R0)?)?;
    // 0x48f2c: orr r5, r1, r9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x48f30: ldr r1, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,0)?)?;
    // 0x48f34: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x48f38: bne 0x48f4c <entry+0x2f4c>  @ imm = #0xc
    c.branch(C::Ne,0x48f4c)?;
    // 0x48f3c: ldr r4, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,0)?)?;
    // 0x48f40: and r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x48f44: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // 0x48f48: ldr r4, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,96)?)?;
    // 0x48f4c: stm r1, {r2, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x48f50: add r9, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x48f54: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x48f58: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x48f5c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x48f60: cmp r9, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x48f64: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x48f68: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x48f6c: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x48f70: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // 0x48f74: mov r1, #41
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,41)?)?;
    // 0x48f78: b 0x48fec <entry+0x2fec>  @ imm = #0x6c
    c.branch(C::Al,0x48fec)?;
    // 0x48f7c: ldr r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
    // 0x48f80: add r0, r10, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x48f84: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3acc0
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x48f88: ldr r5, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,100)?)?;
    // 0x48f8c: ldr r3, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,104)?)?;
    // 0x48f90: mov r1, #2672
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2672)?)?;
    // 0x48f94: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x48f98: add r2, r1, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x48f9c: mul r1, r5, r0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(5)?,R::R0)?)?;
    // 0x48fa0: orr r5, r1, r9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x48fa4: ldr r1, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,0)?)?;
    // 0x48fa8: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x48fac: bne 0x48fc0 <entry+0x2fc0>  @ imm = #0xc
    c.branch(C::Ne,0x48fc0)?;
    // 0x48fb0: ldr r4, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,0)?)?;
    // 0x48fb4: and r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x48fb8: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // 0x48fbc: ldr r4, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,96)?)?;
    // 0x48fc0: stm r1, {r2, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x48fc4: add r9, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x48fc8: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x48fcc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x48fd0: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x48fd4: cmp r9, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x48fd8: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x48fdc: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x48fe0: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x48fe4: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // 0x48fe8: mov r1, #33
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33)?)?;
    // 0x48fec: ldr r2, [pc, #-0x5cc]       @ 0x2a28 <entry+0x2a28>
    c.literal(C::Al,R::R2,0x48a28)?;
    // 0x48ff0: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x48ff4: bl 0x17b0 <entry+0xfffffffffffbb7b0> @ imm = #-0x4784c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x48ff8: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x48ffc: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();pack_additional_codec_registers(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn mask_data_is_separate_from_arm_instructions(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        for (i,word) in CODEC_CONTROLLER_MASKS.iter().enumerate(){
            let at=0x48a20-START+4*i;
            assert_eq!(u32::from_le_bytes(b[at..at+4].try_into().unwrap()),*word);
        }
        assert_eq!(signed_multiply_bottom_halves(C::Al,R::R0,R::R0,R::new(6).unwrap()).unwrap(),0xe160_0680);
        assert_eq!(signed_multiply_bottom_halves(C::Al,R::LR,R::R2,R::new(3).unwrap()).unwrap(),0xe16e_0382);
    }
}
