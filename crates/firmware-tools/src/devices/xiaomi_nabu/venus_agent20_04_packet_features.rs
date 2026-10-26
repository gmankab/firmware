// SPDX-License-Identifier: GPL-3.0-only
//! Continue the same controller-configuration routine across 0x46800..0x47000.
//! This extension packs original packet command flags and controller format
//! metadata. Two original words are branch-skipped literal DATA, not bogus
//! SVC/halfword instructions inferred by a linear disassembler.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::venus_agent20_04_controller_packets::{load_halfword_register};

pub const START:usize=0x46800;
pub const END:usize=0x47000;
const SOURCE_MASK:u32=0x3ffc_0fc0;
const CONTROLLER_MASK:u32=0x007f_ffff;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at!=addr{return Err(format!("controller A32 runtime 0x{:x} expected 0x{addr:x}",self.at));}Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at)||self.at%4!=0{return Err("outside Agent 04 exclusive reservation".into());}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(cond,self.at as u32,target as u32,false)?)
    }
    fn literal(&mut self,cond:C,rd:R,target:usize)->Result<(),String>{
        let d=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"invalid ARM PC+8 literal span")?;
        self.emit(arm32::transfer_word(cond,true,rd,R::PC,d)?)
    }
}

/// Serialize per-codec block widths, packed stream masks and optional
/// component-table words. Earlier controller state is established in
/// `venus_agent20_04_feature_extension.rs`, not regenerated here.
fn encode_controller_packet_formats(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x46800: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46804: orr r3, r3, #32512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,32512)?)?;
    // 0x46808: and r1, r4, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::R1,Shift::Lsl,15,false)?)?;
    // 0x4680c: and r2, r3, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,1,false)?)?;
    // 0x46810: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x46814: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x46818: orr r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1)?)?;
    // 0x4681c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x46820: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x46824: mov r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // 0x46828: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x4682c: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x46830: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x46834: add r1, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,1)?)?;
    // 0x46838: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x4683c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x46840: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x46844: ldr r4, [r10, #0x3b4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,948)?)?;
    // 0x46848: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x4684c: beq 0x46910 <entry+0x910>   @ imm = #0xbc
    c.branch(C::Eq,0x46910)?;
    // 0x46850: ldr r5, [r10, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,944)?)?;
    // 0x46854: mov r8, #1056
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1056)?)?;
    // 0x46858: mov r7, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,36)?)?;
    // 0x4685c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x46860: mov lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // 0x46864: orr r8, r8, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,4096)?)?;
    // 0x46868: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x4686c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x46870: beq 0x468f8 <entry+0x8f8>   @ imm = #0x80
    c.branch(C::Eq,0x468f8)?;
    // 0x46874: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x46878: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x4687c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x46880: sub r5, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,8)?)?;
    // 0x46884: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x46888: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x4688c: ldreq r5, [r0, #-0x8]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::R0,-8)?)?;
    // 0x46890: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x46894: streq r5, [r0, #-0x8]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::R0,-8)?)?;
    // 0x46898: str r8, [r0, #-0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R0,-8)?)?;
    // 0x4689c: str r4, [r0, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,-4)?)?;
    // 0x468a0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x468a4: ldr r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-36)?)?;
    // 0x468a8: add r6, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(5)?,1)?)?;
    // 0x468ac: str r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-36)?)?;
    // 0x468b0: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x468b4: ldr r6, [r10, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,928)?)?;
    // 0x468b8: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x468bc: add r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x468c0: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x468c4: add r3, r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(5)?,2)?)?;
    // 0x468c8: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // 0x468cc: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x468d0: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x468d4: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x468d8: ldr r5, [r10, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,944)?)?;
    // 0x468dc: cmp r2, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x468e0: blo 0x46880 <entry+0x880>   @ imm = #-0x68
    c.branch(C::Cc,0x46880)?;
    // 0x468e4: ldr r2, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,104)?)?;
    // 0x468e8: sub r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,8)?)?;
    // 0x468ec: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x468f0: ldr r4, [r10, #0x3b4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,948)?)?;
    // 0x468f4: b 0x468fc <entry+0x8fc>   @ imm = #0x0
    c.branch(C::Al,0x468fc)?;
    // 0x468f8: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x468fc: ldr r2, [r10, #0x3a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,936)?)?;
    // 0x46900: add lr, lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::LR,1)?)?;
    // 0x46904: cmp lr, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x46908: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4690c: blo 0x4686c <entry+0x86c>   @ imm = #-0xa8
    c.branch(C::Cc,0x4686c)?;
    // 0x46910: ldr r2, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,644)?)?;
    // 0x46914: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x46918: beq 0x46938 <entry+0x938>   @ imm = #0x18
    c.branch(C::Eq,0x46938)?;
    // 0x4691c: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x46920: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x46924: beq 0x46944 <entry+0x944>   @ imm = #0x18
    c.branch(C::Eq,0x46944)?;
    // 0x46928: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x4692c: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x46930: moveq r1, #13
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,13)?)?;
    // 0x46934: b 0x46948 <entry+0x948>   @ imm = #0xc
    c.branch(C::Al,0x46948)?;
    // 0x46938: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4693c: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x46940: b 0x46948 <entry+0x948>   @ imm = #0x0
    c.branch(C::Al,0x46948)?;
    // 0x46944: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x46948: orr r2, r1, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R1,Shift::Lsl,16,false)?)?;
    // 0x4694c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46950: add r9, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R0,8)?)?;
    // 0x46954: orr r2, r2, r1, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,8,false)?)?;
    // 0x46958: orr r1, r2, r1, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,24,false)?)?;
    // 0x4695c: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x46960: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x46964: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x46968: mov r2, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,60)?)?;
    // 0x4696c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x46970: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x46974: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x46978: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x4697c: add r7, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R1,1)?)?;
    // 0x46980: str r7, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-36)?)?;
    // 0x46984: str r9, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(6)?,0)?)?;
    // 0x46988: ldr r0, [r10, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,944)?)?;
    // 0x4698c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x46990: beq 0x469e0 <entry+0x9e0>   @ imm = #0x48
    c.branch(C::Eq,0x469e0)?;
    // 0x46994: mov r1, #168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,168)?)?;
    // 0x46998: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4699c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x469a0: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x469a4: tst r9, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(9)?,31)?)?;
    // 0x469a8: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x469ac: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x469b0: ldreq r3, [r9]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(9)?,0)?)?;
    // 0x469b4: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x469b8: streq r3, [r9]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::new(9)?,0)?)?;
    // 0x469bc: str r1, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // 0x469c0: str r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,4)?)?;
    // 0x469c4: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // 0x469c8: add r9, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,8)?)?;
    // 0x469cc: ldr r3, [r10, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,944)?)?;
    // 0x469d0: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x469d4: blo 0x469a4 <entry+0x9a4>   @ imm = #-0x38
    c.branch(C::Cc,0x469a4)?;
    // 0x469d8: str r7, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-36)?)?;
    // 0x469dc: str r9, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(6)?,0)?)?;
    // 0x469e0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x469e4: ldr r4, [r10, #0x30c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,780)?)?;
    // 0x469e8: ldr r3, [r10, #0x310]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,784)?)?;
    // 0x469ec: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x469f0: str r0, [sp, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,524)?)?;
    // 0x469f4: str r0, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,300)?)?;
    // 0x469f8: ldrb r1, [r10, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,2462)?)?;
    // 0x469fc: cmp r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // 0x46a00: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,64)?)?;
    // 0x46a04: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x46a08: cmp r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // 0x46a0c: bne 0x46a1c <entry+0xa1c>   @ imm = #0x8
    c.branch(C::Ne,0x46a1c)?;
    // 0x46a10: str r0, [sp, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,524)?)?;
    // 0x46a14: b 0x46ab0 <entry+0xab0>   @ imm = #0x94
    c.branch(C::Al,0x46ab0)?;
    c.check(0x46a18)?;
    c.emit(SOURCE_MASK)?;
    c.check(0x46a1c)?;
    // 0x46a1c: ldr r0, [r10, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,940)?)?;
    // 0x46a20: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x46a24: beq 0x46a34 <entry+0xa34>   @ imm = #0x8
    c.branch(C::Eq,0x46a34)?;
    // 0x46a28: mov r0, #272
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,272)?)?;
    // 0x46a2c: b 0x46a44 <entry+0xa44>   @ imm = #0x10
    c.branch(C::Al,0x46a44)?;
    c.check(0x46a30)?;
    c.emit(CONTROLLER_MASK)?;
    c.check(0x46a34)?;
    // 0x46a34: ldr r0, [r10, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,664)?)?;
    // 0x46a38: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x46a3c: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x46a40: movhs r0, #256
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x46a44: str r0, [sp, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,524)?)?;
    // 0x46a48: tst r9, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(9)?,31)?)?;
    // 0x46a4c: ldreq r2, [r9]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::new(9)?,0)?)?;
    // 0x46a50: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x46a54: streq r2, [r9]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(9)?,0)?)?;
    // 0x46a58: mov r2, #2384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2384)?)?;
    // 0x46a5c: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x46a60: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x46a64: str r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,4)?)?;
    // 0x46a68: add r0, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,1)?)?;
    // 0x46a6c: ldr r1, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,104)?)?;
    // 0x46a70: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46a74: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x46a78: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46a7c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x46a80: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x46a84: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x46a88: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x46a8c: mov r0, #340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,340)?)?;
    // 0x46a90: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x46a94: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x46a98: str r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,8)?)?;
    // 0x46a9c: str r2, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,12)?)?;
    // 0x46aa0: add r0, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,2)?)?;
    // 0x46aa4: add r9, r9, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,16)?)?;
    // 0x46aa8: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46aac: str r9, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,0)?)?;
    // 0x46ab0: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x46ab4: cmp r4, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,15)?)?;
    // 0x46ab8: asrgt r3, r3, #4
    c.emit(arm32::mov_shifted_register(C::Gt,R::new(3)?,R::new(3)?,Shift::Asr,4)?)?;
    // 0x46abc: andgt r4, r4, #15
    c.emit(arm32::data_immediate(C::Gt,Op::And,R::new(4)?,R::new(4)?,15)?)?;
    // 0x46ac0: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x46ac4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x46ac8: beq 0x46e80 <entry+0xe80>   @ imm = #0x3b0
    c.branch(C::Eq,0x46e80)?;
    // 0x46acc: ldr r0, [pc, #0xf70]        @ 0x1a44 <entry+0x1a44>
    c.literal(C::Al,R::R0,0x47a44)?;
    // 0x46ad0: str r6, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,100)?)?;
    // 0x46ad4: str r4, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,92)?)?;
    // 0x46ad8: ldrb r7, [r10, #0x3ba]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,954)?)?;
    // 0x46adc: ldrb r0, [r0, r4]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x46ae0: and r6, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(7)?,1)?)?;
    // 0x46ae4: and r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,1)?)?;
    // 0x46ae8: lsl r0, r0, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,8)?)?;
    // 0x46aec: orr r0, r0, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x46af0: ldr r6, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,680)?)?;
    // 0x46af4: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x46af8: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x46afc: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x46b00: ldrb r7, [r10, #0x3db]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,987)?)?;
    // 0x46b04: lsl r6, r6, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(6)?,Shift::Lsl,4)?)?;
    // 0x46b08: movne r6, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,16)?)?;
    // 0x46b0c: tst r9, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(9)?,31)?)?;
    // 0x46b10: orr r0, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x46b14: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x46b18: and r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x46b1c: orr r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x46b20: ldrb r7, [r10, #0x3c4]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,964)?)?;
    // 0x46b24: and r7, r6, r7, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // 0x46b28: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x46b2c: orr r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x46b30: ldrb r7, [r10, #0x3c1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,961)?)?;
    // 0x46b34: and r8, r6, r7, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(8)?,R::new(6)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // 0x46b38: ldrb r7, [r10, #0x3c3]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,963)?)?;
    // 0x46b3c: mov r6, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,96)?)?;
    // 0x46b40: orr r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x46b44: and r7, r6, r7, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,5,false)?)?;
    // 0x46b48: mov r6, #672
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,672)?)?;
    // 0x46b4c: orr r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x46b50: str r0, [sp, #0x140]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,320)?)?;
    // 0x46b54: ldreq r7, [r9]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(9)?,0)?)?;
    // 0x46b58: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x46b5c: streq r7, [r9]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(9)?,0)?)?;
    // 0x46b60: mov r7, #2080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2080)?)?;
    // 0x46b64: orr r7, r7, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,12288)?)?;
    // 0x46b68: str r7, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(9)?,0)?)?;
    // 0x46b6c: str r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,4)?)?;
    // 0x46b70: ldr r7, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-36)?)?;
    // 0x46b74: ldr lr, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,104)?)?;
    // 0x46b78: add r0, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,1)?)?;
    // 0x46b7c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46b80: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x46b84: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x46b88: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46b8c: ldrh r6, [r10, r6]
    c.emit(load_halfword_register(C::Al,R::new(6)?,R::new(10)?,R::new(6)?)?)?;
    // 0x46b90: ldr r5, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,676)?)?;
    // 0x46b94: orr r6, r6, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x46b98: str r6, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,348)?)?;
    // 0x46b9c: ldreq r5, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::R0,0)?)?;
    // 0x46ba0: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x46ba4: streq r5, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::R0,0)?)?;
    // 0x46ba8: mov r0, #14400
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14400)?)?;
    // 0x46bac: str r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,8)?)?;
    // 0x46bb0: add r0, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,2)?)?;
    // 0x46bb4: str r6, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,12)?)?;
    // 0x46bb8: mov r6, #700
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,700)?)?;
    // 0x46bbc: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46bc0: add r0, r9, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,16)?)?;
    // 0x46bc4: orr r6, r6, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,2048)?)?;
    // 0x46bc8: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x46bcc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46bd0: ldrh r6, [r10, r6]
    c.emit(load_halfword_register(C::Al,R::new(6)?,R::new(10)?,R::new(6)?)?)?;
    // 0x46bd4: ldr r5, [r10, #0xac0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,2752)?)?;
    // 0x46bd8: orr r6, r6, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x46bdc: str r6, [sp, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,388)?)?;
    // 0x46be0: ldreq r5, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::R0,0)?)?;
    // 0x46be4: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x46be8: streq r5, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::R0,0)?)?;
    // 0x46bec: mov r0, #2160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2160)?)?;
    // 0x46bf0: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x46bf4: str r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,16)?)?;
    // 0x46bf8: add r0, r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,3)?)?;
    // 0x46bfc: str r6, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,20)?)?;
    // 0x46c00: mov r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,16)?)?;
    // 0x46c04: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46c08: add r0, r9, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,24)?)?;
    // 0x46c0c: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x46c10: ldr r2, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,680)?)?;
    // 0x46c14: ldrb r4, [r10, #0x3b9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(10)?,953)?)?;
    // 0x46c18: ldr r1, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,96)?)?;
    // 0x46c1c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x46c20: and r5, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R1,1)?)?;
    // 0x46c24: and r12, r6, r4, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R12,R::new(6)?,R::new(4)?,Shift::Lsl,4,false)?)?;
    // 0x46c28: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x46c2c: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x46c30: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x46c34: lsl r2, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,4)?)?;
    // 0x46c38: movne r2, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x46c3c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x46c40: ldrb r1, [r10, #0x3c8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,968)?)?;
    // 0x46c44: orr r2, r2, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x46c48: orrne r2, r12, r5
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::R2,R::R12,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x46c4c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46c50: and r1, r6, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(6)?,R::R1,Shift::Lsl,2,false)?)?;
    // 0x46c54: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x46c58: orr r1, r1, r8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x46c5c: mov r8, r4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(4)?))?;
    // 0x46c60: and r2, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(8)?,1)?)?;
    // 0x46c64: orr r1, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x46c68: ldr r2, [pc, #0xdd4]        @ 0x1a44 <entry+0x1a44>
    c.literal(C::Al,R::R2,0x47a44)?;
    // 0x46c6c: ldrb r2, [r2, r3]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R2,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x46c70: and r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,1)?)?;
    // 0x46c74: orr r2, r1, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R2,Shift::Lsl,8,false)?)?;
    // 0x46c78: str r2, [sp, #0x144]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,324)?)?;
    // 0x46c7c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x46c80: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x46c84: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x46c88: mov r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,36)?)?;
    // 0x46c8c: mov r1, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,716)?)?;
    // 0x46c90: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x46c94: str r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,24)?)?;
    // 0x46c98: add r0, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,4)?)?;
    // 0x46c9c: str r2, [r9, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,28)?)?;
    // 0x46ca0: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46ca4: add r0, r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,32)?)?;
    // 0x46ca8: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x46cac: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46cb0: ldrh r1, [r10, r1]
    c.emit(load_halfword_register(C::Al,R::R1,R::new(10)?,R::R1)?)?;
    // 0x46cb4: ldr r2, [r10, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,720)?)?;
    // 0x46cb8: orr r2, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x46cbc: str r2, [sp, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,352)?)?;
    // 0x46cc0: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x46cc4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x46cc8: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x46ccc: mov r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,68)?)?;
    // 0x46cd0: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x46cd4: str r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,32)?)?;
    // 0x46cd8: add r0, r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,5)?)?;
    // 0x46cdc: str r2, [r9, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,36)?)?;
    // 0x46ce0: add r2, r9, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(9)?,40)?)?;
    // 0x46ce4: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46ce8: mov r0, #284
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,284)?)?;
    // 0x46cec: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x46cf0: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x46cf4: orr r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // 0x46cf8: ldrh r0, [r10, r0]
    c.emit(load_halfword_register(C::Al,R::R0,R::new(10)?,R::R0)?)?;
    // 0x46cfc: orr r0, r0, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R0,Shift::Lsl,16,false)?)?;
    // 0x46d00: str r0, [sp, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,392)?)?;
    // 0x46d04: ldreq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x46d08: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x46d0c: streq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x46d10: mov r1, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,116)?)?;
    // 0x46d14: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x46d18: str r1, [r9, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,40)?)?;
    // 0x46d1c: str r0, [r9, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,44)?)?;
    // 0x46d20: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x46d24: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x46d28: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46d2c: add r0, r9, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,48)?)?;
    // 0x46d30: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x46d34: ldr r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x46d38: ldrb r1, [r1, #0x154]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,340)?)?;
    // 0x46d3c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x46d40: blo 0x46e88 <entry+0xe88>   @ imm = #0x140
    c.branch(C::Cc,0x46e88)?;
    // 0x46d44: ldr r1, [sp, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,324)?)?;
    // 0x46d48: ldr r2, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,680)?)?;
    // 0x46d4c: ldr r7, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,96)?)?;
    // 0x46d50: bic r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,1)?)?;
    // 0x46d54: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x46d58: orr r1, r1, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x46d5c: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x46d60: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x46d64: bic r3, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::R1,16)?)?;
    // 0x46d68: orr r2, r3, r2, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,4,false)?)?;
    // 0x46d6c: orrne r2, r1, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R2,R::R1,16)?)?;
    // 0x46d70: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x46d74: and r1, r12, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R12,255)?)?;
    // 0x46d78: mov r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,255)?)?;
    // 0x46d7c: orrne r2, r3, r1
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::R2,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x46d80: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46d84: orr r7, r7, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,65280)?)?;
    // 0x46d88: str r2, [sp, #0x144]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,324)?)?;
    // 0x46d8c: str r2, [sp, #0x148]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,328)?)?;
    // 0x46d90: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x46d94: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x46d98: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x46d9c: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x46da0: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x46da4: str r0, [r9, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,48)?)?;
    // 0x46da8: str r2, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,52)?)?;
    // 0x46dac: add r2, r9, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(9)?,56)?)?;
    // 0x46db0: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x46db4: ldr r5, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,104)?)?;
    // 0x46db8: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x46dbc: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46dc0: str r2, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,0)?)?;
    // 0x46dc4: mov r0, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,254)?)?;
    // 0x46dc8: ldr r6, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(10)?,2744)?)?;
    // 0x46dcc: ldr r1, [r10, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,716)?)?;
    // 0x46dd0: ldr r3, [r10, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,720)?)?;
    // 0x46dd4: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x46dd8: ldrb r6, [r6, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(6)?,325)?)?;
    // 0x46ddc: and r7, r1, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x46de0: tst r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,4)?)?;
    // 0x46de4: andne r7, r0, r1, lsl #1
    c.emit(arm32::data_register(C::Ne,Op::And,R::new(7)?,R::R0,R::R1,Shift::Lsl,1,false)?)?;
    // 0x46de8: and r1, r6, #8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(6)?,8)?)?;
    // 0x46dec: mov r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,16)?)?;
    // 0x46df0: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x46df4: orr r1, r6, r1, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(6)?,R::R1,Shift::Lsr,3,false)?)?;
    // 0x46df8: orr r3, r7, r3, lsl r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,R::R1,false)?)?;
    // 0x46dfc: str r3, [sp, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,352)?)?;
    // 0x46e00: ldreq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x46e04: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x46e08: streq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x46e0c: mov r1, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,72)?)?;
    // 0x46e10: add r2, r9, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(9)?,64)?)?;
    // 0x46e14: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x46e18: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x46e1c: str r1, [r9, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,56)?)?;
    // 0x46e20: str r3, [r9, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,60)?)?;
    // 0x46e24: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x46e28: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x46e2c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x46e30: str r2, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,0)?)?;
    // 0x46e34: ldr r1, [r10, #0x51c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1308)?)?;
    // 0x46e38: ldr r6, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,100)?)?;
    // 0x46e3c: ldr r4, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,92)?)?;
    // 0x46e40: and r0, r0, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,1,false)?)?;
    // 0x46e44: orr r0, r0, r1, lsl #17
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,17,false)?)?;
    // 0x46e48: str r0, [sp, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,392)?)?;
    // 0x46e4c: ldreq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x46e50: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x46e54: streq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x46e58: mov r1, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,120)?)?;
    // 0x46e5c: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x46e60: str r1, [r9, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,64)?)?;
    // 0x46e64: str r0, [r9, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,68)?)?;
    // 0x46e68: add r9, r9, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,72)?)?;
    // 0x46e6c: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x46e70: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x46e74: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46e78: str r9, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,0)?)?;
    // 0x46e7c: b 0x46e94 <entry+0xe94>   @ imm = #0x10
    c.branch(C::Al,0x46e94)?;
    // 0x46e80: ldrb r8, [r10, #0x3b9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(10)?,953)?)?;
    // 0x46e84: b 0x46e94 <entry+0xe94>   @ imm = #0x8
    c.branch(C::Al,0x46e94)?;
    // 0x46e88: ldr r6, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,100)?)?;
    // 0x46e8c: ldr r4, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,92)?)?;
    // 0x46e90: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x46e94: ldr r2, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,96)?)?;
    // 0x46e98: ldrb r0, [r10, #0x3c6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,966)?)?;
    // 0x46e9c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x46ea0: tst r9, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(9)?,31)?)?;
    // 0x46ea4: add r3, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(9)?,8)?)?;
    // 0x46ea8: and r0, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // 0x46eac: and r1, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(8)?,1)?)?;
    // 0x46eb0: and r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,1)?)?;
    // 0x46eb4: orr r1, r2, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,16,false)?)?;
    // 0x46eb8: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x46ebc: str r0, [sp, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,308)?)?;
    // 0x46ec0: ldreq r1, [r9]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(9)?,0)?)?;
    // 0x46ec4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x46ec8: streq r1, [r9]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(9)?,0)?)?;
    // 0x46ecc: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x46ed0: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x46ed4: str r1, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // 0x46ed8: str r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,4)?)?;
    // 0x46edc: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x46ee0: add r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1)?)?;
    // 0x46ee4: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46ee8: ldr r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,104)?)?;
    // 0x46eec: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x46ef0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x46ef4: str r0, [sp, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,304)?)?;
    // 0x46ef8: ldrb r1, [r10, #0xcc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,3272)?)?;
    // 0x46efc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x46f00: str r1, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,100)?)?;
    // 0x46f04: beq 0x46f18 <entry+0xf18>   @ imm = #0xc
    c.branch(C::Eq,0x46f18)?;
    // 0x46f08: ldr r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x46f0c: ldrb r1, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,339)?)?;
    // 0x46f10: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x46f14: beq 0x46f50 <entry+0xf50>   @ imm = #0x34
    c.branch(C::Eq,0x46f50)?;
    // 0x46f18: ldr r1, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,680)?)?;
    // 0x46f1c: ldrb r0, [r10, #0x3ba]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,954)?)?;
    // 0x46f20: mov r7, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,96)?)?;
    // 0x46f24: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x46f28: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x46f2c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x46f30: and r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,1)?)?;
    // 0x46f34: lsl r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,4)?)?;
    // 0x46f38: movne r1, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x46f3c: orr r0, r1, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x46f40: str r0, [sp, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,304)?)?;
    // 0x46f44: ldrb r1, [r10, #0x3c3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,963)?)?;
    // 0x46f48: and r7, r7, r1, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::R1,Shift::Lsl,5,false)?)?;
    // 0x46f4c: b 0x46f58 <entry+0xf58>   @ imm = #0x4
    c.branch(C::Al,0x46f58)?;
    // 0x46f50: mov r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16)?)?;
    // 0x46f54: orr r7, r7, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,65536)?)?;
    // 0x46f58: ldr r1, [pc, #0xee8]        @ 0x1e48 <entry+0x1e48>
    c.literal(C::Al,R::R1,0x47e48)?;
    // 0x46f5c: orr r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x46f60: ldrb r7, [r10, #0x3db]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,987)?)?;
    // 0x46f64: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x46f68: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x46f6c: ldr r1, [pc, #0xad0]        @ 0x1a44 <entry+0x1a44>
    c.literal(C::Al,R::R1,0x47a44)?;
    // 0x46f70: ldrb r1, [r1, r4]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x46f74: and r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,1)?)?;
    // 0x46f78: orr r0, r0, r1, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,8,false)?)?;
    // 0x46f7c: ldrb r1, [r10, #0x3c1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,961)?)?;
    // 0x46f80: str r7, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,92)?)?;
    // 0x46f84: and r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x46f88: orr r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x46f8c: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x46f90: and r1, r7, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(7)?,R::R1,Shift::Lsl,1,false)?)?;
    // 0x46f94: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x46f98: str r0, [sp, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,304)?)?;
    // 0x46f9c: ldreq r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(3)?,0)?)?;
    // 0x46fa0: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x46fa4: streq r1, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(3)?,0)?)?;
    // 0x46fa8: mov r1, #2064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2064)?)?;
    // 0x46fac: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x46fb0: str r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,8)?)?;
    // 0x46fb4: str r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,12)?)?;
    // 0x46fb8: add r0, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,2)?)?;
    // 0x46fbc: mov r1, #672
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,672)?)?;
    // 0x46fc0: ldr r7, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,104)?)?;
    // 0x46fc4: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x46fc8: add r0, r9, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,16)?)?;
    // 0x46fcc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x46fd0: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x46fd4: ldrh r1, [r10, r1]
    c.emit(load_halfword_register(C::Al,R::R1,R::new(10)?,R::R1)?)?;
    // 0x46fd8: ldr r3, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,676)?)?;
    // 0x46fdc: orr r1, r1, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x46fe0: str r1, [sp, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,332)?)?;
    // 0x46fe4: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x46fe8: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x46fec: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x46ff0: mov r0, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2096)?)?;
    // 0x46ff4: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x46ff8: str r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,16)?)?;
    // 0x46ffc: add r0, r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,3)?)?;
    c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();encode_controller_packet_formats(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn masked_parameter_literals_are_typed_data(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        for (at,value) in [(0x46a18,SOURCE_MASK),(0x46a30,CONTROLLER_MASK)]{
            let o=at-START;assert_eq!(u32::from_le_bytes(b[o..o+4].try_into().unwrap()),value);
        }
    }
}
