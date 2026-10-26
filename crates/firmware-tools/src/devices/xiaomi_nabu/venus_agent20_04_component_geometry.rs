// SPDX-License-Identifier: GPL-3.0-only
//! Continue codec/controller flag serialization and plane geometry through
//! 0x47000–0x47800. Two branch-skipped mask words are not ARM instructions.
//! The larger controller routine continues into the following interval.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::venus_agent20_04_controller_packets::load_halfword_register;

pub const START:usize=0x47000;
pub const END:usize=0x47800;
const MAX_COMPONENT_PAIR:u32=0x2dc0_ffff;
const SIGNED_COMPONENT_LIMIT:u32=0x7fff_8000;
struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at!=addr{return Err(format!("A32 source location 0x{:x} expected 0x{addr:x}",self.at));} Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at)||self.at%4!=0{return Err("outside Agent 04 exclusive reservation".into());}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,c:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(c,self.at as u32,target as u32,false)?)
    }
    fn literal(&mut self,c:C,rd:R,target:usize)->Result<(),String>{
        let displacement=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"invalid literal displacement")?;
        self.emit(arm32::transfer_word(c,true,rd,R::PC,displacement)?)
    }
}

/// Calculate and serialize codec component sizes, reserved-frame flags and
/// packet output words, retaining signed overflow controls.
fn write_component_dimensions(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x47000: str r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,20)?)?;
    // 0x47004: mov r1, #604
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,604)?)?;
    // 0x47008: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x4700c: add r0, r9, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,24)?)?;
    // 0x47010: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x47014: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x47018: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4701c: ldrh r1, [r10, r1]
    c.emit(load_halfword_register(C::Al,R::R1,R::new(10)?,R::R1)?)?;
    // 0x47020: ldr r3, [r10, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2656)?)?;
    // 0x47024: orr r1, r1, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x47028: str r1, [sp, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,336)?)?;
    // 0x4702c: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x47030: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x47034: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x47038: mov r0, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    // 0x4703c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x47040: str r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,24)?)?;
    // 0x47044: add r0, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,4)?)?;
    // 0x47048: str r1, [r9, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,28)?)?;
    // 0x4704c: mov r1, #700
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,700)?)?;
    // 0x47050: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x47054: add r0, r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,32)?)?;
    // 0x47058: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x4705c: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x47060: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x47064: ldrh r1, [r10, r1]
    c.emit(load_halfword_register(C::Al,R::R1,R::new(10)?,R::R1)?)?;
    // 0x47068: ldr r3, [r10, #0xac0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2752)?)?;
    // 0x4706c: orr r1, r1, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x47070: str r1, [sp, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,372)?)?;
    // 0x47074: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x47078: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x4707c: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x47080: mov r3, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2144)?)?;
    // 0x47084: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x47088: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // 0x4708c: str r3, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,32)?)?;
    // 0x47090: str r1, [r9, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,36)?)?;
    // 0x47094: add r1, r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,5)?)?;
    // 0x47098: add r2, r9, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(9)?,40)?)?;
    // 0x4709c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x470a0: mov r1, #284
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,284)?)?;
    // 0x470a4: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // 0x470a8: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x470ac: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x470b0: ldrh r1, [r10, r1]
    c.emit(load_halfword_register(C::Al,R::R1,R::new(10)?,R::R1)?)?;
    // 0x470b4: orr r1, r1, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R1,Shift::Lsl,16,false)?)?;
    // 0x470b8: str r1, [sp, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,376)?)?;
    // 0x470bc: ldreq r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R2,0)?)?;
    // 0x470c0: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x470c4: streq r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R2,0)?)?;
    // 0x470c8: lsl r2, r0, r6
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R0,Shift::Lsl,R::new(6)?,false)?)?;
    // 0x470cc: mov r0, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,100)?)?;
    // 0x470d0: mov r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,32)?)?;
    // 0x470d4: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x470d8: str r0, [r9, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,40)?)?;
    // 0x470dc: str r1, [r9, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,44)?)?;
    // 0x470e0: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x470e4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x470e8: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x470ec: add r0, r9, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,48)?)?;
    // 0x470f0: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x470f4: mov r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // 0x470f8: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,60)?)?;
    // 0x470fc: ldr r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
    // 0x47100: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x47104: ldr r1, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,100)?)?;
    // 0x47108: moveq r6, #48
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,48)?)?;
    // 0x4710c: moveq r7, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x47110: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x47114: beq 0x47128 <entry+0x1128>  @ imm = #0xc
    c.branch(C::Eq,0x47128)?;
    // 0x47118: ldr r0, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2744)?)?;
    // 0x4711c: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x47120: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x47124: beq 0x47138 <entry+0x1138>  @ imm = #0xc
    c.branch(C::Eq,0x47138)?;
    // 0x47128: ldr r0, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,680)?)?;
    // 0x4712c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x47130: moveq r6, #128
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,128)?)?;
    // 0x47134: moveq r7, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x47138: ldr r4, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,2744)?)?;
    // 0x4713c: mov r12, r2
    c.emit(arm32::mov_register(C::Al,R::R12,R::R2))?;
    // 0x47140: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x47144: ldrb r5, [r4, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,336)?)?;
    // 0x47148: str r2, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,76)?)?;
    // 0x4714c: str r7, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,44)?)?;
    // 0x47150: str r4, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,84)?)?;
    // 0x47154: str r8, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,32)?)?;
    // 0x47158: ands r0, r5, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::new(5)?,1,true)?)?;
    // 0x4715c: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // 0x47160: bne 0x47208 <entry+0x1208>  @ imm = #0xa0
    c.branch(C::Ne,0x47208)?;
    // 0x47164: ldr r2, [r10, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,692)?)?;
    // 0x47168: ldr r0, [r10, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,720)?)?;
    // 0x4716c: ldr r8, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,76)?)?;
    // 0x47170: ldr r1, [r10, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,716)?)?;
    // 0x47174: ldr r4, [r10, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,696)?)?;
    // 0x47178: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // 0x4717c: mul r0, r2, r8
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::new(8)?)?)?;
    // 0x47180: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3cac4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x47184: ldr r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // 0x47188: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x4718c: mul r0, r4, r8
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(4)?,R::new(8)?)?)?;
    // 0x47190: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3cab4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x47194: ldr r4, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,84)?)?;
    // 0x47198: ldr r3, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,100)?)?;
    // 0x4719c: mov lr, r7
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(7)?))?;
    // 0x471a0: ldrb r2, [r4, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,325)?)?;
    // 0x471a4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x471a8: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x471ac: movne r1, #24
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x471b0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x471b4: add r12, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x471b8: add r1, r1, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // 0x471bc: beq 0x471e0 <entry+0x11e0>  @ imm = #0x1c
    c.branch(C::Eq,0x471e0)?;
    // 0x471c0: ldrb r3, [r4, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,339)?)?;
    // 0x471c4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x471c8: beq 0x47204 <entry+0x1204>  @ imm = #0x34
    c.branch(C::Eq,0x47204)?;
    // 0x471cc: ldr r3, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,680)?)?;
    // 0x471d0: ldr r7, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,44)?)?;
    // 0x471d4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x471d8: beq 0x471f0 <entry+0x11f0>  @ imm = #0x10
    c.branch(C::Eq,0x471f0)?;
    // 0x471dc: b 0x47208 <entry+0x1208>  @ imm = #0x24
    c.branch(C::Al,0x47208)?;
    // 0x471e0: ldr r3, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,680)?)?;
    // 0x471e4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x471e8: bne 0x47204 <entry+0x1204>  @ imm = #0x14
    c.branch(C::Ne,0x47204)?;
    // 0x471ec: ldr r7, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,44)?)?;
    // 0x471f0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x471f4: beq 0x47208 <entry+0x1208>  @ imm = #0xc
    c.branch(C::Eq,0x47208)?;
    // 0x471f8: add r12, r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R0,9)?)?;
    // 0x471fc: add r1, lr, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::LR,24)?)?;
    // 0x47200: b 0x47208 <entry+0x1208>  @ imm = #0x0
    c.branch(C::Al,0x47208)?;
    // 0x47204: ldr r7, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,44)?)?;
    // 0x47208: ldrb r0, [r4, #0x143]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,323)?)?;
    // 0x4720c: orr r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x47210: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // 0x47214: bne 0x47238 <entry+0x1238>  @ imm = #0x1c
    c.branch(C::Ne,0x47238)?;
    // 0x47218: ldr r0, [r10, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,692)?)?;
    // 0x4721c: ldr r2, [r10, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,716)?)?;
    // 0x47220: cmp r0, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,1,false)?)?;
    // 0x47224: bhi 0x47238 <entry+0x1238>  @ imm = #0xc
    c.branch(C::Hi,0x47238)?;
    // 0x47228: ldr r0, [r10, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,696)?)?;
    // 0x4722c: ldr r2, [r10, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,720)?)?;
    // 0x47230: cmp r0, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,1,false)?)?;
    // 0x47234: bls 0x479f8 <entry+0x19f8>  @ imm = #0x7bc
    c.branch(C::Ls,0x479f8)?;
    // 0x47238: ldr r0, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,300)?)?;
    // 0x4723c: mov r5, r12
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R12))?;
    // 0x47240: orr r4, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::R0,16)?)?;
    // 0x47244: add r0, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x47248: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x4724c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x47250: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3c9f4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x47254: add r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,1)?)?;
    // 0x47258: ands r0, r4, #16
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::new(4)?,16,true)?)?;
    // 0x4725c: str r4, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,300)?)?;
    // 0x47260: str r5, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,56)?)?;
    // 0x47264: str r4, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,52)?)?;
    // 0x47268: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // 0x4726c: str r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,28)?)?;
    // 0x47270: bne 0x472d8 <entry+0x12d8>  @ imm = #0x60
    c.branch(C::Ne,0x472d8)?;
    // 0x47274: ldr r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,100)?)?;
    // 0x47278: cmp r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // 0x4727c: mov r8, r2
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R2))?;
    // 0x47280: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x47284: movhs r8, #5
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(8)?,R::R0,5)?)?;
    // 0x47288: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4728c: beq 0x472a0 <entry+0x12a0>  @ imm = #0xc
    c.branch(C::Eq,0x472a0)?;
    // 0x47290: ldr r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,84)?)?;
    // 0x47294: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x47298: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4729c: beq 0x472ac <entry+0x12ac>  @ imm = #0x8
    c.branch(C::Eq,0x472ac)?;
    // 0x472a0: ldr r0, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,680)?)?;
    // 0x472a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x472a8: beq 0x472f8 <entry+0x12f8>  @ imm = #0x48
    c.branch(C::Eq,0x472f8)?;
    // 0x472ac: add r0, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x472b0: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x472b4: mov r4, r3
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x472b8: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x472bc: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3c988
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x472c0: ldr r1, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,84)?)?;
    // 0x472c4: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x472c8: ldrb r4, [r1, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R1,325)?)?;
    // 0x472cc: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x472d0: addne r0, r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R0,1)?)?;
    // 0x472d4: b 0x47318 <entry+0x1318>  @ imm = #0x3c
    c.branch(C::Al,0x47318)?;
    // 0x472d8: ldr r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,92)?)?;
    // 0x472dc: mov r4, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3)?)?;
    // 0x472e0: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x472e4: mov r8, r2
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R2))?;
    // 0x472e8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x472ec: moveq r4, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x472f0: moveq r7, #8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // 0x472f4: b 0x4737c <entry+0x137c>  @ imm = #0x80
    c.branch(C::Al,0x4737c)?;
    // 0x472f8: add r0, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x472fc: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x47300: mov r4, r3
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x47304: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x47308: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3c93c
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x4730c: ldr r1, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,84)?)?;
    // 0x47310: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x47314: ldrb r4, [r1, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R1,325)?)?;
    // 0x47318: ldr r6, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,92)?)?;
    // 0x4731c: ldr r5, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,48)?)?;
    // 0x47320: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x47324: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x47328: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x4732c: mov r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // 0x47330: movls r0, r3
    c.emit(arm32::mov_register(C::Ls,R::R0,R::new(3)?))?;
    // 0x47334: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x47338: moveq r7, #12
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,12)?)?;
    // 0x4733c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x47340: moveq r7, r0
    c.emit(arm32::mov_register(C::Eq,R::new(7)?,R::R0))?;
    // 0x47344: add r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1)?)?;
    // 0x47348: add r0, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // 0x4734c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x47350: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3c8f4
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x47354: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x47358: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x4735c: mov r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x47360: addne r0, r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R0,1)?)?;
    // 0x47364: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x47368: movls r0, r1
    c.emit(arm32::mov_register(C::Ls,R::R0,R::R1))?;
    // 0x4736c: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x47370: moveq r4, #6
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,6)?)?;
    // 0x47374: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x47378: moveq r4, r0
    c.emit(arm32::mov_register(C::Eq,R::new(4)?,R::R0))?;
    // 0x4737c: ldr r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,100)?)?;
    // 0x47380: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x47384: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x47388: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x4738c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x47390: beq 0x473c4 <entry+0x13c4>  @ imm = #0x2c
    c.branch(C::Eq,0x473c4)?;
    // 0x47394: ldr r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,84)?)?;
    // 0x47398: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x4739c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x473a0: ldrne r0, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(10)?,680)?)?;
    // 0x473a4: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x473a8: beq 0x473d0 <entry+0x13d0>  @ imm = #0x20
    c.branch(C::Eq,0x473d0)?;
    // 0x473ac: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // 0x473b0: ldr r2, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,56)?)?;
    // 0x473b4: ldr r4, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,52)?)?;
    // 0x473b8: mov r5, r8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // 0x473bc: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x473c0: b 0x4747c <entry+0x147c>  @ imm = #0xb4
    c.branch(C::Al,0x4747c)?;
    // 0x473c4: ldr r0, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,680)?)?;
    // 0x473c8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x473cc: bne 0x47494 <entry+0x1494>  @ imm = #0xc0
    c.branch(C::Ne,0x47494)?;
    // 0x473d0: mov r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3)?)?;
    // 0x473d4: cmp r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,3)?)?;
    // 0x473d8: movls r8, r5
    c.emit(arm32::mov_register(C::Ls,R::new(8)?,R::new(5)?))?;
    // 0x473dc: mul r0, r8, r7
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(8)?,R::new(7)?)?)?;
    // 0x473e0: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x473e4: cmp r0, #129
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,129)?)?;
    // 0x473e8: blo 0x4741c <entry+0x141c>  @ imm = #0x2c
    c.branch(C::Cc,0x4741c)?;
    // 0x473ec: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // 0x473f0: mov r6, r8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // 0x473f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x473f8: mov r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,128)?)?;
    // 0x473fc: moveq r6, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // 0x47400: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x47404: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3c840
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x47408: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x4740c: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x47410: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x47414: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x47418: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x4741c: mul r0, r8, r4
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(8)?,R::new(4)?)?)?;
    // 0x47420: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // 0x47424: cmp r0, #129
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,129)?)?;
    // 0x47428: movlo r5, r8
    c.emit(arm32::mov_register(C::Cc,R::new(5)?,R::new(8)?))?;
    // 0x4742c: movhs r6, #42
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(6)?,R::R0,42)?)?;
    // 0x47430: blo 0x47468 <entry+0x1468>  @ imm = #0x30
    c.branch(C::Cc,0x47468)?;
    // 0x47434: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // 0x47438: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4743c: beq 0x47468 <entry+0x1468>  @ imm = #0x24
    c.branch(C::Eq,0x47468)?;
    // 0x47440: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x47444: mov r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,128)?)?;
    // 0x47448: mov r4, r3
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x4744c: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3c7f8
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x47450: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x47454: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x47458: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x4745c: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x47460: mov r5, r8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // 0x47464: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x47468: ldr r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,100)?)?;
    // 0x4746c: ldr r2, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,56)?)?;
    // 0x47470: ldr r4, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,52)?)?;
    // 0x47474: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x47478: beq 0x474a8 <entry+0x14a8>  @ imm = #0x28
    c.branch(C::Eq,0x474a8)?;
    // 0x4747c: ldr r1, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,84)?)?;
    // 0x47480: ldr r0, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,680)?)?;
    // 0x47484: ldrb r1, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,339)?)?;
    // 0x47488: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4748c: bne 0x474ac <entry+0x14ac>  @ imm = #0x18
    c.branch(C::Ne,0x474ac)?;
    // 0x47490: b 0x47500 <entry+0x1500>  @ imm = #0x68
    c.branch(C::Al,0x47500)?;
    // 0x47494: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // 0x47498: ldr r2, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,56)?)?;
    // 0x4749c: ldr r4, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,52)?)?;
    // 0x474a0: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x474a4: mov r5, r8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // 0x474a8: ldr r0, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,680)?)?;
    // 0x474ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x474b0: bne 0x47500 <entry+0x1500>  @ imm = #0x48
    c.branch(C::Ne,0x47500)?;
    // 0x474b4: ldr r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // 0x474b8: add r0, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x474bc: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x474c0: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3c784
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x474c4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x474c8: and r1, r1, r4, lsr #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(4)?,Shift::Lsr,20,false)?)?;
    // 0x474cc: add r3, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x474d0: add r0, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,1)?)?;
    // 0x474d4: add r5, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // 0x474d8: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // 0x474dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x474e0: bne 0x47538 <entry+0x1538>  @ imm = #0x50
    c.branch(C::Ne,0x47538)?;
    // 0x474e4: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // 0x474e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x474ec: bne 0x47560 <entry+0x1560>  @ imm = #0x6c
    c.branch(C::Ne,0x47560)?;
    // 0x474f0: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x474f4: mov r6, r5
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(5)?))?;
    // 0x474f8: mov r7, r3
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(3)?))?;
    // 0x474fc: b 0x47550 <entry+0x1550>  @ imm = #0x4c
    c.branch(C::Al,0x47550)?;
    // 0x47500: ldr r2, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,92)?)?;
    // 0x47504: mul r1, r5, r6
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(5)?,R::new(6)?)?)?;
    // 0x47508: lsl r4, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R1,Shift::Lsl,1)?)?;
    // 0x4750c: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // 0x47510: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x47514: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x47518: lsreq r4, r1, #1
    c.emit(arm32::mov_shifted_register(C::Eq,R::new(4)?,R::R1,Shift::Lsr,1)?)?;
    // 0x4751c: mul r1, r3, r7
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(3)?,R::new(7)?)?)?;
    // 0x47520: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x47524: lsl r8, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::R1,Shift::Lsl,1)?)?;
    // 0x47528: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // 0x4752c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x47530: lsreq r8, r1, #1
    c.emit(arm32::mov_shifted_register(C::Eq,R::new(8)?,R::R1,Shift::Lsr,1)?)?;
    // 0x47534: b 0x47588 <entry+0x1588>  @ imm = #0x4c
    c.branch(C::Al,0x47588)?;
    // 0x47538: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // 0x4753c: mov r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // 0x47540: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x47544: bne 0x47564 <entry+0x1564>  @ imm = #0x18
    c.branch(C::Ne,0x47564)?;
    // 0x47548: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x4754c: mov r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x47550: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x47554: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // 0x47558: b 0x47568 <entry+0x1568>  @ imm = #0x8
    c.branch(C::Al,0x47568)?;
    c.check(0x4755c)?;
    c.emit(MAX_COMPONENT_PAIR)?;
    c.check(0x47560)?;
    // 0x47560: mov r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // 0x47564: mov r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x47568: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x4756c: mul r0, r6, r5
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::new(5)?)?)?;
    // 0x47570: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x47574: lsl r4, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x47578: mul r0, r7, r3
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(7)?,R::new(3)?)?)?;
    // 0x4757c: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x47580: lsl r8, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::R0,Shift::Lsl,2)?)?;
    // 0x47584: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x47588: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4758c: bne 0x47614 <entry+0x1614>  @ imm = #0x80
    c.branch(C::Ne,0x47614)?;
    // 0x47590: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x47594: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x47598: beq 0x47614 <entry+0x1614>  @ imm = #0x74
    c.branch(C::Eq,0x47614)?;
    // 0x4759c: cmp r8, #153
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,153)?)?;
    // 0x475a0: blo 0x475dc <entry+0x15dc>  @ imm = #0x34
    c.branch(C::Cc,0x475dc)?;
    // 0x475a4: mov r8, r4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(4)?))?;
    // 0x475a8: lsl r4, r3, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x475ac: mov r0, #152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,152)?)?;
    // 0x475b0: mov r7, r3
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(3)?))?;
    // 0x475b4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x475b8: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3c68c
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x475bc: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x475c0: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x475c4: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x475c8: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x475cc: mul r0, r4, r7
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(4)?,R::new(7)?)?)?;
    // 0x475d0: mov r4, r8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
    // 0x475d4: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x475d8: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // 0x475dc: cmp r4, #89
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,89)?)?;
    // 0x475e0: blo 0x47614 <entry+0x1614>  @ imm = #0x2c
    c.branch(C::Cc,0x47614)?;
    // 0x475e4: lsl r4, r5, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x475e8: mov r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,88)?)?;
    // 0x475ec: mov r6, r3
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // 0x475f0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x475f4: blx 0x83c4c <entry+0x3dc4c> @ imm = #0x3c650
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x475f8: ldr r12, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,104)?)?;
    // 0x475fc: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x47600: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x47604: mov lr, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,255)?)?;
    // 0x47608: mul r0, r4, r6
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(4)?,R::new(6)?)?)?;
    // 0x4760c: orr lr, lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    // 0x47610: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x47614: mov r1, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16711680)?)?;
    // 0x47618: and r2, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(3)?,255)?)?;
    // 0x4761c: mov r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,65280)?)?;
    // 0x47620: and r1, r1, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x47624: and r0, r0, r7, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(7)?,Shift::Lsl,8,false)?)?;
    // 0x47628: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4762c: ldr r2, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,60)?)?;
    // 0x47630: orr r1, r1, r6, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,24,false)?)?;
    // 0x47634: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x47638: str r0, [sp, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,448)?)?;
    // 0x4763c: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x47640: ldreq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R2,0)?)?;
    // 0x47644: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x47648: streq r1, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    // 0x4764c: mov r1, #244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,244)?)?;
    // 0x47650: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x47654: str r1, [r9, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,48)?)?;
    // 0x47658: str r0, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,52)?)?;
    // 0x4765c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x47660: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x47664: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x47668: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x4766c: add r0, r9, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,56)?)?;
    // 0x47670: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x47674: str r1, [sp, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,436)?)?;
    // 0x47678: ldr r1, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2744)?)?;
    // 0x4767c: ldr r6, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,80)?)?;
    // 0x47680: ldrb r2, [r1, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,339)?)?;
    // 0x47684: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x47688: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x4768c: bne 0x476b8 <entry+0x16b8>  @ imm = #0x24
    c.branch(C::Ne,0x476b8)?;
    // 0x47690: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // 0x47694: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x47698: bne 0x476a8 <entry+0x16a8>  @ imm = #0x8
    c.branch(C::Ne,0x476a8)?;
    // 0x4769c: ldr r1, [r10, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,680)?)?;
    // 0x476a0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x476a4: beq 0x4cb68 <entry+0x6b68>  @ imm = #0x54bc
    c.branch(C::Eq,0x4cb68)?;
    // 0x476a8: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x476ac: ldr r2, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,96)?)?;
    // 0x476b0: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x476b4: moveq r1, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x476b8: str r1, [sp, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,436)?)?;
    // 0x476bc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x476c0: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x476c4: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x476c8: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x476cc: mov r0, #232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,232)?)?;
    // 0x476d0: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x476d4: str r0, [r9, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,56)?)?;
    // 0x476d8: str r1, [r9, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,60)?)?;
    // 0x476dc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x476e0: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x476e4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x476e8: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x476ec: add r0, r9, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,64)?)?;
    // 0x476f0: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x476f4: str r1, [sp, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,440)?)?;
    // 0x476f8: str r1, [sp, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,444)?)?;
    // 0x476fc: ldr r2, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2744)?)?;
    // 0x47700: ldrb r2, [r2, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,339)?)?;
    // 0x47704: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x47708: bne 0x47750 <entry+0x1750>  @ imm = #0x40
    c.branch(C::Ne,0x47750)?;
    // 0x4770c: mov r1, #33816576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33816576)?)?;
    // 0x47710: ldr r3, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,96)?)?;
    // 0x47714: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x47718: str r1, [sp, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,440)?)?;
    // 0x4771c: mov r1, #655360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,655360)?)?;
    // 0x47720: orr r1, r1, #100663296
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,100663296)?)?;
    // 0x47724: str r1, [sp, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,444)?)?;
    // 0x47728: ldrb r7, [sp, #0x144]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::SP,324)?)?;
    // 0x4772c: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x47730: mov r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,5)?)?;
    // 0x47734: moveq r2, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x47738: moveq r3, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x4773c: tst r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,16)?)?;
    // 0x47740: moveq r3, r2
    c.emit(arm32::mov_register(C::Eq,R::new(3)?,R::R2))?;
    // 0x47744: orr r2, r3, #33816576
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(3)?,33816576)?)?;
    // 0x47748: b 0x47764 <entry+0x1764>  @ imm = #0x14
    c.branch(C::Al,0x47764)?;
    c.check(0x4774c)?;
    c.emit(SIGNED_COMPONENT_LIMIT)?;
    c.check(0x47750)?;
    // 0x47750: ldr r3, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,96)?)?;
    // 0x47754: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x47758: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x4775c: moveq r2, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x47760: mov r3, r2
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // 0x47764: orr r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x47768: str r2, [sp, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,440)?)?;
    // 0x4776c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x47770: str r1, [sp, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,444)?)?;
    // 0x47774: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x47778: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4777c: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x47780: mov r0, #236
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,236)?)?;
    // 0x47784: add r1, r9, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,72)?)?;
    // 0x47788: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x4778c: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x47790: str r0, [r9, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,64)?)?;
    // 0x47794: str r2, [r9, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,68)?)?;
    // 0x47798: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x4779c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x477a0: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x477a4: str r1, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x477a8: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x477ac: ldr r0, [sp, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,444)?)?;
    // 0x477b0: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x477b4: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x477b8: mov r1, #2288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2288)?)?;
    // 0x477bc: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x477c0: str r1, [r9, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,72)?)?;
    // 0x477c4: str r0, [r9, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,76)?)?;
    // 0x477c8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x477cc: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x477d0: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x477d4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x477d8: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x477dc: add r0, r9, #80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,80)?)?;
    // 0x477e0: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x477e4: str r1, [sp, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,452)?)?;
    // 0x477e8: ldr r2, [r10, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2744)?)?;
    // 0x477ec: ldrb r3, [r2, #0x51c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,1308)?)?;
    // 0x477f0: ldr r2, [pc, #0xeec]        @ 0x26e4 <entry+0x26e4>
    c.literal(C::Al,R::R2,0x486e4)?;
    // 0x477f4: cmp r3, #29
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,29)?)?;
    // 0x477f8: beq 0x4780c <entry+0x180c>  @ imm = #0xc
    c.branch(C::Eq,0x4780c)?;
    // 0x477fc: cmp r3, #21
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,21)?)?;
    c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();write_component_dimensions(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn component_masks_are_not_compiled_as_instructions(){
        let b=fragment().unwrap(); assert_eq!(b.len(),END-START);
        for (at,val) in [(0x4755c,MAX_COMPONENT_PAIR),(0x4774c,SIGNED_COMPONENT_LIMIT)]{
            let off=at-START; assert_eq!(u32::from_le_bytes(b[off..off+4].try_into().unwrap()),val);
        }
    }
}
