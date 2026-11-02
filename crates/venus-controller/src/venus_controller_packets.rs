// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller packet classification and serialization with typed A32.
//! Runtime `[0x416c0,0x424c0)` contains one complex controller format
//! function with six jump-table words at `0x41cb4–0x41ccc`, followed by a
//! four-word diagnostic/bitmask literal pool `0x424b0–0x424c0`.
//!
//! Every executable instruction is assembled from public ARM register,
//! condition, shift, memory and DSP encodings. These are original instruction
//! *semantics* at the architecture level, not a proven high-level C rewrite.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::venus_controller_codec_controller::signed_long_multiply;

pub const START:usize=0x416c0;
pub const JUMP_TABLE_START:usize=0x41cb4;
pub const JUMP_TABLE_END:usize=0x41ccc;
pub const CODE_END:usize=0x424b0;
pub const END:usize=0x424c0;
pub const FILE_BASE:usize=0x4000;
const SCALER_CLASSES:[u32;6]=[
    0x0004_1ccc,0x0004_1ce8,0x0004_1d00,
    0x0004_1d18,0x0004_1d30,0x0004_1d48,
];
const CONTROLLER_LITERALS:[u32;4]=[
    0x000e_e460,0x1fff_ffe0,0x01ff_fffe,0x000e_e028,
];

/// ARM `UMULL`: unsigned 32x32 to 64-bit result, RdHi:RdLo.
fn unsigned_long_multiply(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
    if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("invalid UMULL operands".into());}
    Ok(((cond as u32)<<28)|0x0080_0090|(hi.number()<<16)|
       (lo.number()<<12)|(rs.number()<<8)|rm.number())
}

/// ARM register-offset LDRH, positive preindex, no base writeback.
pub(super) fn load_halfword_register(cond:C,rd:R,base:R,index:R)->Result<u32,String>{
    if [rd,base,index].contains(&R::PC){return Err("LDRH register offset forbids PC".into());}
    Ok(((cond as u32)<<28)|0x0190_00b0|
       (base.number()<<16)|(rd.number()<<12)|index.number())
}

/// ARM register-offset LDRSH, sign-extending the loaded halfword.
pub(super) fn load_signed_halfword_register(cond:C,rd:R,base:R,index:R)->Result<u32,String>{
    if [rd,base,index].contains(&R::PC){return Err("LDRSH register offset forbids PC".into());}
    Ok(((cond as u32)<<28)|0x0190_00f0|
       (base.number()<<16)|(rd.number()<<12)|index.number())
}

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at!=addr{return Err(format!("Venus controller code at 0x{:x} expected 0x{addr:x}",self.at));}
        Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<0x40000 || self.at>=0x68000 || self.at%4!=0{
            return Err(format!("outside Agent 20-04 reserved region at 0x{:x}",self.at));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,condition:C,dst:usize)->Result<(),String>{
        self.emit(arm32::branch(condition,self.at as u32,dst as u32,false)?)
    }
    fn call(&mut self,dst:usize)->Result<(),String>{
        self.emit(arm32::branch(C::Al,self.at as u32,dst as u32,true)?)
    }
    fn literal(&mut self,rd:R,target:usize)->Result<(),String>{
        let signed=i32::try_from(target as isize-self.at as isize-8)
            .map_err(|_|"controller literal offset does not fit signed word")?;
        self.emit(arm32::transfer_word(C::Al,true,rd,R::PC,signed)?)
    }
}

/// Begin a controller feature packet, normalize descriptor state, serialize
/// controller registers and choose one of six supported scale classes.
fn classify_scale_and_mode(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x416c0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x416c4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x416c8: sub sp, sp, #60
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,60)?)?;
    // 0x416cc: ldr r5, [pc, #0xddc]
    c.literal(R::new(5)?,0x424b0)?;
    // 0x416d0: ldr lr, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,44)?)?;
    // 0x416d4: mov r10, r1
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // 0x416d8: add r1, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,44)?)?;
    // 0x416dc: ldm r5, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::R2.bit() | R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x416e0: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit() | R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x416e4: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x416e8: mov r7, #57344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,57344)?)?;
    // 0x416ec: str r6, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,40)?)?;
    // 0x416f0: ldr r1, [r6, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,2744)?)?;
    // 0x416f4: ldrb r12, [r6, #0x495]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(6)?,1173)?)?;
    // 0x416f8: ldr r2, [r1, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,180)?)?;
    // 0x416fc: ldr r3, [r1, #0xb8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,184)?)?;
    // 0x41700: ldr r0, [r1, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,168)?)?;
    // 0x41704: add r2, r7, r2, lsl #13
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::R2,Shift::Lsl,13,false)?)?;
    // 0x41708: mov r7, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,7168)?)?;
    // 0x4170c: add r3, r7, r3, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,10,false)?)?;
    // 0x41710: ldr r7, [r6, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,644)?)?;
    // 0x41714: and r2, r2, #57344
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,57344)?)?;
    // 0x41718: and r3, r3, #7168
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,7168)?)?;
    // 0x4171c: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x41720: mov r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,512)?)?;
    // 0x41724: and r0, r3, r0, lsl #9
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,9,false)?)?;
    // 0x41728: ldrb r3, [r1, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,204)?)?;
    // 0x4172c: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x41730: orr r2, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x41734: ldreq r0, [r1, #0x60]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::R1,96)?)?;
    // 0x41738: moveq r6, #256
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,256)?)?;
    // 0x4173c: andeq r0, r6, r0, lsl #8
    c.emit(arm32::data_register(C::Eq,Op::And,R::R0,R::new(6)?,R::R0,Shift::Lsl,8,false)?)?;
    // 0x41740: orreq r2, r0, r2
    c.emit(arm32::data_register(C::Eq,Op::Orr,R::R2,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x41744: ldr r0, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,88)?)?;
    // 0x41748: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4174c: beq 0x417c0 <entry+0x18c0>
    c.branch(C::Eq,0x417c0)?;
    // 0x41750: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x41754: beq 0x4176c <entry+0x186c>
    c.branch(C::Eq,0x4176c)?;
    // 0x41758: ldrb r6, [r1, #0xde]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R1,222)?)?;
    // 0x4175c: ldrb r0, [lr, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,380)?)?;
    // 0x41760: add r6, r6, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,6)?)?;
    // 0x41764: cmp r6, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x41768: blo 0x417b8 <entry+0x18b8>
    c.branch(C::Cc,0x417b8)?;
    // 0x4176c: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x41770: beq 0x417a4 <entry+0x18a4>
    c.branch(C::Eq,0x417a4)?;
    // 0x41774: ldrb r6, [r1, #0xde]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R1,222)?)?;
    // 0x41778: ldrb r0, [lr, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,380)?)?;
    // 0x4177c: add r6, r6, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,6)?)?;
    // 0x41780: cmp r6, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x41784: bhs 0x417a4 <entry+0x18a4>
    c.branch(C::Cs,0x417a4)?;
    // 0x41788: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x4178c: bne 0x417b8 <entry+0x18b8>
    c.branch(C::Ne,0x417b8)?;
    // 0x41790: cmp r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,36)?)?;
    // 0x41794: blo 0x417b8 <entry+0x18b8>
    c.branch(C::Cc,0x417b8)?;
    // 0x41798: ldr r0, [lr, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,400)?)?;
    // 0x4179c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x417a0: bne 0x417b8 <entry+0x18b8>
    c.branch(C::Ne,0x417b8)?;
    // 0x417a4: orrs r0, r3, r12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R12,Shift::Lsl,0,true)?)?;
    // 0x417a8: bne 0x417c0 <entry+0x18c0>
    c.branch(C::Ne,0x417c0)?;
    // 0x417ac: ldrb r0, [lr, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,380)?)?;
    // 0x417b0: cmp r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,13)?)?;
    // 0x417b4: blo 0x417c0 <entry+0x18c0>
    c.branch(C::Cc,0x417c0)?;
    // 0x417b8: orr r0, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,2)?)?;
    // 0x417bc: b 0x417cc <entry+0x18cc>
    c.branch(C::Al,0x417cc)?;
    // 0x417c0: mov r0, #253
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,253)?)?;
    // 0x417c4: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x417c8: and r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x417cc: ldr r1, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,92)?)?;
    // 0x417d0: mov r2, #251
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,251)?)?;
    // 0x417d4: ldr r4, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,40)?)?;
    // 0x417d8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x417dc: orr r2, r2, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65280)?)?;
    // 0x417e0: and r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x417e4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x417e8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x417ec: movgt r1, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x417f0: orr r0, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x417f4: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,0)?)?;
    // 0x417f8: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x417fc: add r8, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R1,8)?)?;
    // 0x41800: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x41804: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x41808: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x4180c: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x41810: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x41814: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x41818: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x4181c: str r8, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(10)?,0)?)?;
    // 0x41820: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x41824: ldr r0, [lr, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,420)?)?;
    // 0x41828: ldr r6, [r2, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,2744)?)?;
    // 0x4182c: ldrb r7, [r6, #0xd8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,216)?)?;
    // 0x41830: ldr r0, [r0, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    // 0x41834: ldrb r5, [r6, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(6)?,204)?)?;
    // 0x41838: and r7, r7, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,15)?)?;
    // 0x4183c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x41840: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x41844: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x41848: lsl r7, r7, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Lsl,5)?)?;
    // 0x4184c: orr r9, r7, r0, lsl #9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(9)?,R::new(7)?,R::R0,Shift::Lsl,9,false)?)?;
    // 0x41850: ldrb r7, [r2, #0x47d]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R2,1149)?)?;
    // 0x41854: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x41858: and r5, r0, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x4185c: ldr r0, [r2, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,1244)?)?;
    // 0x41860: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x41864: orrne r9, r9, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(9)?,R::new(9)?,16)?)?;
    // 0x41868: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x4186c: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x41870: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x41874: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x41878: lsl r4, r3, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(3)?,Shift::Lsl,3)?)?;
    // 0x4187c: ldr r3, [r2, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,744)?)?;
    // 0x41880: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x41884: bne 0x41894 <entry+0x1994>
    c.branch(C::Ne,0x41894)?;
    // 0x41888: ldrb r0, [r2, #0x34a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R2,842)?)?;
    // 0x4188c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x41890: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x41894: ldrb r6, [r6, #0xe1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(6)?,225)?)?;
    // 0x41898: orr r5, r4, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4189c: tst r8, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,31)?)?;
    // 0x418a0: orr r5, r5, r9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(5)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x418a4: orr r0, r5, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,1,false)?)?;
    // 0x418a8: and r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x418ac: orr r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x418b0: and r6, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::R12,1)?)?;
    // 0x418b4: mov r12, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3)?)?;
    // 0x418b8: orr r0, r0, r6, lsl #11
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,11,false)?)?;
    // 0x418bc: ldrb r6, [r2, #0x496]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R2,1174)?)?;
    // 0x418c0: mov r2, #3104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3104)?)?;
    // 0x418c4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x418c8: and r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x418cc: orr r0, r0, r6, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,10,false)?)?;
    // 0x418d0: ldreq r6, [r8]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(8)?,0)?)?;
    // 0x418d4: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x418d8: streq r6, [r8]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(8)?,0)?)?;
    // 0x418dc: str r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x418e0: add r2, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,16)?)?;
    // 0x418e4: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x418e8: str r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x418ec: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,0)?)?;
    // 0x418f0: bne 0x4195c <entry+0x1a5c>
    c.branch(C::Ne,0x4195c)?;
    // 0x418f4: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x418f8: ldr r7, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,2744)?)?;
    // 0x418fc: ldrb r6, [r7, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(7)?,204)?)?;
    // 0x41900: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x41904: beq 0x4195c <entry+0x1a5c>
    c.branch(C::Eq,0x4195c)?;
    // 0x41908: ldrb r0, [r7, #0x51c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,1308)?)?;
    // 0x4190c: cmp r0, #33
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,33)?)?;
    // 0x41910: bne 0x41924 <entry+0x1a24>
    c.branch(C::Ne,0x41924)?;
    // 0x41914: mov r0, #292
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,292)?)?;
    // 0x41918: orr r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // 0x4191c: ldrh r0, [r7, r0]
    c.emit(load_halfword_register(C::Al,R::R0,R::new(7)?,R::R0)?)?;
    // 0x41920: b 0x41930 <entry+0x1a30>
    c.branch(C::Al,0x41930)?;
    // 0x41924: mov r0, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,106)?)?;
    // 0x41928: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // 0x4192c: ldrh r0, [lr, r0]
    c.emit(load_halfword_register(C::Al,R::R0,R::LR,R::R0)?)?;
    // 0x41930: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x41934: mov r12, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,4)?)?;
    // 0x41938: ldreq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,0)?)?;
    // 0x4193c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x41940: streq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R2,0)?)?;
    // 0x41944: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // 0x41948: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x4194c: str r2, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,16)?)?;
    // 0x41950: add r2, r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,24)?)?;
    // 0x41954: str r0, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,20)?)?;
    // 0x41958: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,0)?)?;
    // 0x4195c: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x41960: ldr r6, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,40)?)?;
    // 0x41964: ldrb r1, [lr, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::LR,380)?)?;
    // 0x41968: add r8, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R2,8)?)?;
    // 0x4196c: ldreq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,0)?)?;
    // 0x41970: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x41974: streq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R2,0)?)?;
    // 0x41978: mov r7, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,36)?)?;
    // 0x4197c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x41980: orr r7, r7, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,7168)?)?;
    // 0x41984: str r7, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,0)?)?;
    // 0x41988: str r1, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,4)?)?;
    // 0x4198c: str r8, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(10)?,0)?)?;
    // 0x41990: beq 0x41b6c <entry+0x1c6c>
    c.branch(C::Eq,0x41b6c)?;
    // 0x41994: ldr r3, [r6, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,644)?)?;
    // 0x41998: cmp r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4096)?)?;
    // 0x4199c: bne 0x41b78 <entry+0x1c78>
    c.branch(C::Ne,0x41b78)?;
    // 0x419a0: ldr r3, [lr, #0x210]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::LR,528)?)?;
    // 0x419a4: mov r7, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1792)?)?;
    // 0x419a8: add r12, r12, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R12,7)?)?;
    // 0x419ac: and r3, r7, r3, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x419b0: ldr r7, [r6, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,2744)?)?;
    // 0x419b4: add r6, r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,16)?)?;
    // 0x419b8: ldrb r7, [r7, #0x51c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(7)?,1308)?)?;
    // 0x419bc: cmp r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,255)?)?;
    // 0x419c0: ldrb r7, [lr, #0x249]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::LR,585)?)?;
    // 0x419c4: moveq r3, #768
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,768)?)?;
    // 0x419c8: tst r8, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,31)?)?;
    // 0x419cc: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x419d0: ldreq r7, [r8]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(8)?,0)?)?;
    // 0x419d4: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x419d8: streq r7, [r8]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(8)?,0)?)?;
    // 0x419dc: mov r7, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,40)?)?;
    // 0x419e0: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x419e4: add r8, r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R2,64)?)?;
    // 0x419e8: orr r7, r7, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,7168)?)?;
    // 0x419ec: str r7, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,8)?)?;
    // 0x419f0: str r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,12)?)?;
    // 0x419f4: str r6, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,0)?)?;
    // 0x419f8: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x419fc: ldr r7, [lr, #0x218]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::LR,536)?)?;
    // 0x41a00: ldr r5, [lr, #0x21c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::LR,540)?)?;
    // 0x41a04: orr r3, r3, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,3840)?)?;
    // 0x41a08: and r4, r7, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41a0c: mov r7, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16711680)?)?;
    // 0x41a10: orr r7, r7, #251658240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,251658240)?)?;
    // 0x41a14: and r5, r7, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(7)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x41a18: orr r5, r5, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x41a1c: ldreq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(6)?,0)?)?;
    // 0x41a20: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x41a24: streq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(6)?,0)?)?;
    // 0x41a28: mov r6, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,44)?)?;
    // 0x41a2c: orr r6, r6, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,7168)?)?;
    // 0x41a30: str r6, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,16)?)?;
    // 0x41a34: add r6, r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,24)?)?;
    // 0x41a38: str r5, [r2, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,20)?)?;
    // 0x41a3c: str r6, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,0)?)?;
    // 0x41a40: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x41a44: ldr r5, [lr, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::LR,544)?)?;
    // 0x41a48: ldr r4, [lr, #0x224]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::LR,548)?)?;
    // 0x41a4c: and r5, r5, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41a50: and r4, r7, r4, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(7)?,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x41a54: orr r5, r4, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x41a58: ldreq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(6)?,0)?)?;
    // 0x41a5c: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x41a60: streq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(6)?,0)?)?;
    // 0x41a64: mov r6, #3120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3120)?)?;
    // 0x41a68: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // 0x41a6c: str r6, [r2, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,24)?)?;
    // 0x41a70: add r6, r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,32)?)?;
    // 0x41a74: str r5, [r2, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,28)?)?;
    // 0x41a78: str r6, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,0)?)?;
    // 0x41a7c: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x41a80: ldr r5, [lr, #0x228]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::LR,552)?)?;
    // 0x41a84: ldr r4, [lr, #0x22c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::LR,556)?)?;
    // 0x41a88: and r5, r5, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41a8c: and r4, r7, r4, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(7)?,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x41a90: orr r5, r4, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x41a94: ldreq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(6)?,0)?)?;
    // 0x41a98: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x41a9c: streq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(6)?,0)?)?;
    // 0x41aa0: mov r6, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,52)?)?;
    // 0x41aa4: orr r6, r6, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,7168)?)?;
    // 0x41aa8: str r6, [r2, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,32)?)?;
    // 0x41aac: add r6, r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,40)?)?;
    // 0x41ab0: str r5, [r2, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,36)?)?;
    // 0x41ab4: str r6, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,0)?)?;
    // 0x41ab8: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x41abc: ldr r5, [lr, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::LR,560)?)?;
    // 0x41ac0: ldr r4, [lr, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::LR,564)?)?;
    // 0x41ac4: and r5, r5, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41ac8: and r4, r7, r4, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(7)?,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x41acc: orr r5, r4, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x41ad0: ldreq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(6)?,0)?)?;
    // 0x41ad4: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x41ad8: streq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(6)?,0)?)?;
    // 0x41adc: mov r6, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,56)?)?;
    // 0x41ae0: orr r6, r6, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,7168)?)?;
    // 0x41ae4: str r6, [r2, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,40)?)?;
    // 0x41ae8: add r6, r2, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,48)?)?;
    // 0x41aec: str r5, [r2, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,44)?)?;
    // 0x41af0: str r6, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,0)?)?;
    // 0x41af4: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x41af8: ldr r5, [lr, #0x238]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::LR,568)?)?;
    // 0x41afc: ldr r4, [lr, #0x23c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::LR,572)?)?;
    // 0x41b00: and r5, r5, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41b04: and r4, r7, r4, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(7)?,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x41b08: orr r5, r4, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x41b0c: ldreq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(6)?,0)?)?;
    // 0x41b10: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x41b14: streq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(6)?,0)?)?;
    // 0x41b18: mov r6, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,60)?)?;
    // 0x41b1c: orr r6, r6, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,7168)?)?;
    // 0x41b20: str r6, [r2, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,48)?)?;
    // 0x41b24: add r6, r2, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,56)?)?;
    // 0x41b28: str r5, [r2, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,52)?)?;
    // 0x41b2c: str r6, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,0)?)?;
    // 0x41b30: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x41b34: ldr r5, [lr, #0x240]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::LR,576)?)?;
    // 0x41b38: ldr r4, [lr, #0x244]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::LR,580)?)?;
    // 0x41b3c: and r3, r5, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41b40: and r7, r7, r4, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x41b44: orr r3, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41b48: ldreq r7, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(6)?,0)?)?;
    // 0x41b4c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x41b50: streq r7, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(6)?,0)?)?;
    // 0x41b54: mov r7, #7232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,7232)?)?;
    // 0x41b58: str r7, [r2, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,56)?)?;
    // 0x41b5c: str r3, [r2, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,60)?)?;
    // 0x41b60: str r8, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(10)?,0)?)?;
    // 0x41b64: ldr r6, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,40)?)?;
    // 0x41b68: b 0x41b78 <entry+0x1c78>
    c.branch(C::Al,0x41b78)?;
    // 0x41b6c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x41b70: str r2, [lr, #0x214]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,532)?)?;
    // 0x41b74: str r2, [lr, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,508)?)?;
    // 0x41b78: ldr r2, [r6, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,2744)?)?;
    // 0x41b7c: ldrb r3, [r2, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,224)?)?;
    // 0x41b80: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x41b84: bne 0x41b9c <entry+0x1c9c>
    c.branch(C::Ne,0x41b9c)?;
    // 0x41b88: ldrb r3, [r2, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,204)?)?;
    // 0x41b8c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x41b90: ldrne r3, [lr, #0x20]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::LR,32)?)?;
    // 0x41b94: ldrne r3, [r3, #0x7a0]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::new(3)?,1952)?)?;
    // 0x41b98: strbne r3, [r6, #0x490]
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(3)?,R::new(6)?,1168)?)?;
    // 0x41b9c: ldr r3, [lr, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::LR,400)?)?;
    // 0x41ba0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x41ba4: beq 0x41bb8 <entry+0x1cb8>
    c.branch(C::Eq,0x41bb8)?;
    // 0x41ba8: ldrb r7, [r6, #0x490]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,1168)?)?;
    // 0x41bac: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x41bb0: ldreq r7, [lr, #0x214]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::LR,532)?)?;
    // 0x41bb4: b 0x41bc8 <entry+0x1cc8>
    c.branch(C::Al,0x41bc8)?;
    // 0x41bb8: ldrb r6, [r6, #0x3dd]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(6)?,989)?)?;
    // 0x41bbc: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x41bc0: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x41bc4: moveq r7, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x41bc8: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41bcc: ldrb r6, [r2, #0x51c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R2,1308)?)?;
    // 0x41bd0: cmp r6, #23
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,23)?)?;
    // 0x41bd4: add r5, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,3)?)?;
    // 0x41bd8: add r6, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,5)?)?;
    // 0x41bdc: add r4, r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,6)?)?;
    // 0x41be0: ldreq r7, [r2, #0x524]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,1316)?)?;
    // 0x41be4: add r5, r5, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1024)?)?;
    // 0x41be8: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x41bec: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x41bf0: str lr, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,20)?)?;
    // 0x41bf4: add r4, r4, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1024)?)?;
    // 0x41bf8: add r9, r8, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(8)?,8)?)?;
    // 0x41bfc: addeq r5, r6, #1024
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(5)?,R::new(6)?,1024)?)?;
    // 0x41c00: add r6, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,4)?)?;
    // 0x41c04: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x41c08: addeq r5, r6, #1024
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(5)?,R::new(6)?,1024)?)?;
    // 0x41c0c: ldrb r6, [r2, #0xdd]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R2,221)?)?;
    // 0x41c10: ldrb r2, [r2, #0xde]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,222)?)?;
    // 0x41c14: ldrb r5, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(5)?,0)?)?;
    // 0x41c18: sub r2, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x41c1c: add r1, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x41c20: cmp r2, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x41c24: movgt r5, r2
    c.emit(arm32::mov_register(C::Gt,R::new(5)?,R::R2))?;
    // 0x41c28: mov r2, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,65280)?)?;
    // 0x41c2c: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x41c30: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x41c34: and r2, r2, r5, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(5)?,Shift::Lsl,8,false)?)?;
    // 0x41c38: add r5, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,8)?)?;
    // 0x41c3c: addeq r4, r5, #1024
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(4)?,R::new(5)?,1024)?)?;
    // 0x41c40: add r5, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,7)?)?;
    // 0x41c44: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x41c48: addeq r4, r5, #1024
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(4)?,R::new(5)?,1024)?)?;
    // 0x41c4c: ldrb r3, [r4]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,0)?)?;
    // 0x41c50: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41c54: movlo r3, r1
    c.emit(arm32::mov_register(C::Cc,R::new(3)?,R::R1))?;
    // 0x41c58: mov r1, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16711680)?)?;
    // 0x41c5c: tst r8, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,31)?)?;
    // 0x41c60: and r1, r1, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,16,false)?)?;
    // 0x41c64: orr r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41c68: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x41c6c: ldreq r2, [r8]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::new(8)?,0)?)?;
    // 0x41c70: orr r1, r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16777216)?)?;
    // 0x41c74: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x41c78: streq r2, [r8]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(8)?,0)?)?;
    // 0x41c7c: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // 0x41c80: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x41c84: str r2, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x41c88: str r1, [r8, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,4)?)?;
    // 0x41c8c: str r9, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(10)?,0)?)?;
    // 0x41c90: ldr r5, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,2744)?)?;
    // 0x41c94: ldr r1, [r5, #0x4bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1212)?)?;
    // 0x41c98: str r3, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,8)?)?;
    // 0x41c9c: str r8, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,36)?)?;
    // 0x41ca0: cmp r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // 0x41ca4: bhi 0x41d5c <entry+0x1e5c>
    c.branch(C::Hi,0x41d5c)?;
    // 0x41ca8: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // 0x41cac: add r2, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,0)?)?;
    // 0x41cb0: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
    c.check(JUMP_TABLE_START)?;
    for handler in SCALER_CLASSES{c.emit(handler)?;}
    c.check(JUMP_TABLE_END)
}

/// Process the selected codec scaling class, derive packed dimension and
/// feature flags, write the controller ring and prepare optional buffer words.
fn finish_controller_packet(c:&mut Code)->Result<(),String>{
    c.check(JUMP_TABLE_END)?;
    // 0x41ccc: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // 0x41cd0: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x41cd4: add r1, r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,512)?)?;
    // 0x41cd8: tst r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,16)?)?;
    // 0x41cdc: addne r1, r2, #508
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R2,508)?)?;
    // 0x41ce0: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x41ce4: b 0x41d5c <entry+0x1e5c>
    c.branch(C::Al,0x41d5c)?;
    // 0x41ce8: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41cec: ldr r1, [r0, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2656)?)?;
    // 0x41cf0: mul r2, r1, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R1)?)?;
    // 0x41cf4: ldr r1, [pc, #0x7bc]
    c.literal(R::R1,0x424b8)?;
    // 0x41cf8: and r1, r1, r2, lsr #7
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsr,7,false)?)?;
    // 0x41cfc: b 0x41d5c <entry+0x1e5c>
    c.branch(C::Al,0x41d5c)?;
    // 0x41d00: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41d04: ldr r1, [r0, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2656)?)?;
    // 0x41d08: mul r2, r1, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R1)?)?;
    // 0x41d0c: mvn r1, #-67108861
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,4227858435)?)?;
    // 0x41d10: and r1, r1, r2, lsr #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsr,6,false)?)?;
    // 0x41d14: b 0x41d5c <entry+0x1e5c>
    c.branch(C::Al,0x41d5c)?;
    // 0x41d18: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41d1c: ldr r1, [r0, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2656)?)?;
    // 0x41d20: mul r2, r1, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R1)?)?;
    // 0x41d24: mvn r1, #-268435441
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,4026531855)?)?;
    // 0x41d28: and r1, r1, r2, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsr,4,false)?)?;
    // 0x41d2c: b 0x41d5c <entry+0x1e5c>
    c.branch(C::Al,0x41d5c)?;
    // 0x41d30: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41d34: ldr r1, [r0, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2656)?)?;
    // 0x41d38: mul r2, r1, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R1)?)?;
    // 0x41d3c: ldr r1, [pc, #0x770]
    c.literal(R::R1,0x424b4)?;
    // 0x41d40: and r1, r1, r2, lsr #3
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsr,3,false)?)?;
    // 0x41d44: b 0x41d5c <entry+0x1e5c>
    c.branch(C::Al,0x41d5c)?;
    // 0x41d48: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41d4c: ldr r1, [r0, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2656)?)?;
    // 0x41d50: mul r2, r1, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R1)?)?;
    // 0x41d54: mvn r1, #-1073741761
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,3221225535)?)?;
    // 0x41d58: and r1, r1, r2, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsr,2,false)?)?;
    // 0x41d5c: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41d60: add r8, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R12,1)?)?;
    // 0x41d64: ldr r7, [r0, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,664)?)?;
    // 0x41d68: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x41d6c: str r10, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,24)?)?;
    // 0x41d70: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x41d74: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x41d78: blo 0x41f48 <entry+0x2048>
    c.branch(C::Cc,0x41f48)?;
    // 0x41d7c: ldr r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,40)?)?;
    // 0x41d80: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x41d84: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x41d88: mov r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x41d8c: mov r10, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,15)?)?;
    // 0x41d90: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x41d94: ldr r0, [r2, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,644)?)?;
    // 0x41d98: ldr r1, [r2, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,2656)?)?;
    // 0x41d9c: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x41da0: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x41da4: moveq r4, #5
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x41da8: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x41dac: moveq r10, #31
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(10)?,R::R0,31)?)?;
    // 0x41db0: lsr r6, r1, r4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(6)?,R::R0,R::R1,Shift::Lsr,R::new(4)?,false)?)?;
    // 0x41db4: tst r1, r10
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R1,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x41db8: addne r6, r3, r1, lsr r4
    c.emit(arm32::data_register_shifted_by_register(C::Ne,Op::Add,R::new(6)?,R::new(3)?,R::R1,Shift::Lsr,R::new(4)?,false)?)?;
    // 0x41dbc: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x41dc0: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x41dc4: blx 0x83c4c <entry+0x43d4c>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x41dc8: ldr r3, [r5, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,2652)?)?;
    // 0x41dcc: mul r1, r0, r7
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(7)?)?)?;
    // 0x41dd0: add lr, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::SP,44)?)?;
    // 0x41dd4: sub r12, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R12,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x41dd8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x41ddc: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x41de0: lsr r2, r3, r4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::new(3)?,Shift::Lsr,R::new(4)?,false)?)?;
    // 0x41de4: tst r10, r3
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41de8: addne r2, r1, r3, lsr r4
    c.emit(arm32::data_register_shifted_by_register(C::Ne,Op::Add,R::R2,R::R1,R::new(3)?,Shift::Lsr,R::new(4)?,false)?)?;
    // 0x41dec: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x41df0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x41df4: cmp r12, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41df8: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x41dfc: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x41e00: addhi r5, r5, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x41e04: mul r1, r5, r2
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(5)?,R::R2)?)?;
    // 0x41e08: str r1, [lr, r3, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::LR,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x41e0c: mla r1, r5, r2, r6
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::new(5)?,R::R2,R::new(6)?)?)?;
    // 0x41e10: and r3, r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,255)?)?;
    // 0x41e14: cmp r7, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x41e18: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x41e1c: bhi 0x41df4 <entry+0x1ef4>
    c.branch(C::Hi,0x41df4)?;
    // 0x41e20: ldr r0, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // 0x41e24: ldrb r0, [r0, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,224)?)?;
    // 0x41e28: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x41e2c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x41e30: bne 0x41f40 <entry+0x2040>
    c.branch(C::Ne,0x41f40)?;
    // 0x41e34: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x41e38: blo 0x41f40 <entry+0x2040>
    c.branch(C::Cc,0x41f40)?;
    // 0x41e3c: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x41e40: ldr r3, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,28)?)?;
    // 0x41e44: mov r6, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,76)?)?;
    // 0x41e48: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x41e4c: mov r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x41e50: mov lr, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,128)?)?;
    // 0x41e54: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x41e58: orr r6, r6, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,7168)?)?;
    // 0x41e5c: add r4, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,12)?)?;
    // 0x41e60: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x41e64: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x41e68: sub r0, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,1)?)?;
    // 0x41e6c: sub r5, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::new(4)?,4)?)?;
    // 0x41e70: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x41e74: bls 0x41eac <entry+0x1fac>
    c.branch(C::Ls,0x41eac)?;
    // 0x41e78: add r0, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,44)?)?;
    // 0x41e7c: ldr r2, [r0, r10, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(10)?,Shift::Lsl,2)?)?;
    // 0x41e80: umull r0, r1, r2, r3
    c.emit(unsigned_long_multiply(C::Al,R::R0,R::R1,R::R2,R::new(3)?)?)?;
    // 0x41e84: ldr r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // 0x41e88: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x41e8c: bl 0x8187c <entry+0x4197c>
    c.call(0x8187c)?;
    // 0x41e90: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // 0x41e94: ldr r3, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,28)?)?;
    // 0x41e98: mov lr, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,128)?)?;
    // 0x41e9c: mov r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x41ea0: add r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x41ea4: str r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x41ea8: b 0x41eb4 <entry+0x1fb4>
    c.branch(C::Al,0x41eb4)?;
    // 0x41eac: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x41eb0: sub r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x41eb4: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x41eb8: lsl r1, r12, r10
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R12,Shift::Lsl,R::new(10)?,false)?)?;
    // 0x41ebc: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // 0x41ec0: add r8, r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,2)?)?;
    // 0x41ec4: ldreq r2, [r4, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::new(4)?,-4)?)?;
    // 0x41ec8: and r10, r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(10)?,R::new(9)?,255)?)?;
    // 0x41ecc: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x41ed0: streq r2, [r4, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(4)?,-4)?)?;
    // 0x41ed4: str lr, [r4, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,-4)?)?;
    // 0x41ed8: str r1, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x41edc: add r1, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,4)?)?;
    // 0x41ee0: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x41ee4: ldreq r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(4)?,4)?)?;
    // 0x41ee8: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x41eec: streq r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,4)?)?;
    // 0x41ef0: str r6, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,4)?)?;
    // 0x41ef4: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x41ef8: add r4, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,16)?)?;
    // 0x41efc: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41f00: ldr r7, [r0, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,664)?)?;
    // 0x41f04: cmp r7, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x41f08: bhi 0x41e68 <entry+0x1f68>
    c.branch(C::Hi,0x41e68)?;
    // 0x41f0c: ldr r10, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,24)?)?;
    // 0x41f10: sub r9, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::new(4)?,4)?)?;
    // 0x41f14: mov r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x41f18: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x41f1c: moveq r1, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x41f20: tst r9, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(9)?,31)?)?;
    // 0x41f24: str r9, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(10)?,0)?)?;
    // 0x41f28: ldreq r0, [r4, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,-4)?)?;
    // 0x41f2c: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x41f30: streq r0, [r4, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,-4)?)?;
    // 0x41f34: mov r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,128)?)?;
    // 0x41f38: str r0, [r4, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,-4)?)?;
    // 0x41f3c: b 0x41f64 <entry+0x2064>
    c.branch(C::Al,0x41f64)?;
    // 0x41f40: ldr r10, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,24)?)?;
    // 0x41f44: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x41f48: tst r9, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(9)?,31)?)?;
    // 0x41f4c: ldreq r0, [r9]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(9)?,0)?)?;
    // 0x41f50: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x41f54: streq r0, [r9]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,0)?)?;
    // 0x41f58: mov r0, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,76)?)?;
    // 0x41f5c: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,7168)?)?;
    // 0x41f60: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x41f64: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x41f68: str r1, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,4)?)?;
    // 0x41f6c: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x41f70: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41f74: ldr r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,20)?)?;
    // 0x41f78: ldr r0, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2744)?)?;
    // 0x41f7c: ldr r1, [r9, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,480)?)?;
    // 0x41f80: ldr r2, [r0, #0x4b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,1208)?)?;
    // 0x41f84: sub r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x41f88: lsl r4, r1, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R1,Shift::Lsl,3)?)?;
    // 0x41f8c: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // 0x41f90: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x41f94: bne 0x41fbc <entry+0x20bc>
    c.branch(C::Ne,0x41fbc)?;
    // 0x41f98: ldr r2, [pc, #0x51c]
    c.literal(R::R2,0x424bc)?;
    // 0x41f9c: mov r1, #171
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,171)?)?;
    // 0x41fa0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x41fa4: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x41fa8: movgt r0, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x41fac: orr r1, r1, #6656
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6656)?)?;
    // 0x41fb0: bl 0x17b0 <entry+0xfffffffffffc18b0>
    c.call(0x17b0)?;
    // 0x41fb4: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41fb8: ldr r0, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2744)?)?;
    // 0x41fbc: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x41fc0: ands r1, r1, #16
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R1,16,true)?)?;
    // 0x41fc4: ldrbeq r0, [r0, #0xe0]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::R0,224)?)?;
    // 0x41fc8: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x41fcc: beq 0x41ff8 <entry+0x20f8>
    c.branch(C::Eq,0x41ff8)?;
    // 0x41fd0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x41fd4: bne 0x41ff0 <entry+0x20f0>
    c.branch(C::Ne,0x41ff0)?;
    // 0x41fd8: mov r1, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,153)?)?;
    // 0x41fdc: asr r0, r4, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(4)?,Shift::Asr,3)?)?;
    // 0x41fe0: orr r1, r1, #2304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    // 0x41fe4: smull r2, r3, r0, r1
    c.emit(signed_long_multiply(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x41fe8: lsr r0, r2, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsr,8)?)?;
    // 0x41fec: orr r4, r0, r3, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R0,R::new(3)?,Shift::Lsl,24,false)?)?;
    // 0x41ff0: bic r0, r4, #-268435456
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(4)?,4026531840)?)?;
    // 0x41ff4: b 0x4203c <entry+0x213c>
    c.branch(C::Al,0x4203c)?;
    // 0x41ff8: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x41ffc: ldr r0, [r0, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1244)?)?;
    // 0x42000: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x42004: beq 0x42030 <entry+0x2130>
    c.branch(C::Eq,0x42030)?;
    // 0x42008: ldr r0, [r9, #0x26c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,620)?)?;
    // 0x4200c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x42010: beq 0x42038 <entry+0x2138>
    c.branch(C::Eq,0x42038)?;
    // 0x42014: umull r1, r2, r0, r4
    c.emit(unsigned_long_multiply(C::Al,R::R1,R::R2,R::R0,R::new(4)?)?)?;
    // 0x42018: asr r3, r4, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(4)?,Shift::Asr,31)?)?;
    // 0x4201c: mla r7, r0, r3, r2
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::R0,R::new(3)?,R::R2)?)?;
    // 0x42020: lsr r0, r1, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Lsr,16)?)?;
    // 0x42024: orr r0, r0, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,16,false)?)?;
    // 0x42028: bic r0, r0, #-268435456
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,4026531840)?)?;
    // 0x4202c: b 0x4203c <entry+0x213c>
    c.branch(C::Al,0x4203c)?;
    // 0x42030: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x42034: b 0x4203c <entry+0x213c>
    c.branch(C::Al,0x4203c)?;
    // 0x42038: bic r0, r4, #-268435449
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(4)?,4026531847)?)?;
    // 0x4203c: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,0)?)?;
    // 0x42040: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x42044: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x42048: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4204c: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x42050: mov r2, #3152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3152)?)?;
    // 0x42054: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x42058: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x4205c: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x42060: add r0, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,8)?)?;
    // 0x42064: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x42068: ldr r2, [r9, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,412)?)?;
    // 0x4206c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x42070: beq 0x4208c <entry+0x218c>
    c.branch(C::Eq,0x4208c)?;
    // 0x42074: ldr r2, [r9, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,400)?)?;
    // 0x42078: add r3, r9, #980
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(9)?,980)?)?;
    // 0x4207c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x42080: addeq r3, r9, #984
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(3)?,R::new(9)?,984)?)?;
    // 0x42084: ldr r6, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,0)?)?;
    // 0x42088: b 0x420b4 <entry+0x21b4>
    c.branch(C::Al,0x420b4)?;
    // 0x4208c: ldr r7, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,40)?)?;
    // 0x42090: mov r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,5)?)?;
    // 0x42094: ldrb r2, [r7, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(7)?,2462)?)?;
    // 0x42098: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // 0x4209c: ldr r2, [r7, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,2652)?)?;
    // 0x420a0: ldr r7, [r7, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,2656)?)?;
    // 0x420a4: moveq r3, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x420a8: lsr r7, r7, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(7)?,R::R0,R::new(7)?,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x420ac: lsr r2, r2, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R2,Shift::Lsr,R::new(3)?,false)?)?;
    // 0x420b0: mul r6, r2, r7
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::R2,R::new(7)?)?)?;
    // 0x420b4: add r2, r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,2)?)?;
    // 0x420b8: str r2, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,36)?)?;
    // 0x420bc: ldr r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,40)?)?;
    // 0x420c0: ldr r2, [r2, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,2744)?)?;
    // 0x420c4: ldrb r2, [r2, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,224)?)?;
    // 0x420c8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x420cc: bne 0x421e0 <entry+0x22e0>
    c.branch(C::Ne,0x421e0)?;
    // 0x420d0: ldr r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,40)?)?;
    // 0x420d4: ldr r2, [r2, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,664)?)?;
    // 0x420d8: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x420dc: blo 0x421e0 <entry+0x22e0>
    c.branch(C::Cc,0x421e0)?;
    // 0x420e0: ldr r3, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,36)?)?;
    // 0x420e4: mov r9, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,84)?)?;
    // 0x420e8: add r5, r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R1,12)?)?;
    // 0x420ec: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x420f0: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x420f4: mov r12, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,128)?)?;
    // 0x420f8: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x420fc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x42100: orr r9, r9, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,7168)?)?;
    // 0x42104: sub r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R2,1)?)?;
    // 0x42108: sub r4, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(5)?,4)?)?;
    // 0x4210c: cmp r0, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x42110: bls 0x42148 <entry+0x2248>
    c.branch(C::Ls,0x42148)?;
    // 0x42114: add r0, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,44)?)?;
    // 0x42118: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // 0x4211c: str r3, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,36)?)?;
    // 0x42120: ldr r0, [r0, r10, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(10)?,Shift::Lsl,2)?)?;
    // 0x42124: lsl r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,4)?)?;
    // 0x42128: blx 0x83c4c <entry+0x43d4c>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x4212c: ldr r3, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,36)?)?;
    // 0x42130: mul r1, r0, r6
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(6)?)?)?;
    // 0x42134: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x42138: mov r12, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,128)?)?;
    // 0x4213c: add r7, r7, r1, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::R1,Shift::Lsr,4,false)?)?;
    // 0x42140: lsr r0, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Lsr,4)?)?;
    // 0x42144: b 0x4214c <entry+0x224c>
    c.branch(C::Al,0x4214c)?;
    // 0x42148: sub r0, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x4214c: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x42150: lsl r1, lr, r10
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::LR,Shift::Lsl,R::new(10)?,false)?)?;
    // 0x42154: add r8, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // 0x42158: add r3, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,2)?)?;
    // 0x4215c: ldreq r2, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::new(5)?,-4)?)?;
    // 0x42160: and r10, r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(10)?,R::new(8)?,255)?)?;
    // 0x42164: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x42168: streq r2, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(5)?,-4)?)?;
    // 0x4216c: str r12, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(5)?,-4)?)?;
    // 0x42170: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // 0x42174: add r1, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,4)?)?;
    // 0x42178: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4217c: ldreq r1, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,4)?)?;
    // 0x42180: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x42184: streq r1, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(5)?,4)?)?;
    // 0x42188: str r9, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,4)?)?;
    // 0x4218c: str r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,8)?)?;
    // 0x42190: add r5, r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,16)?)?;
    // 0x42194: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x42198: ldr r2, [r0, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,664)?)?;
    // 0x4219c: cmp r2, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x421a0: bhi 0x42104 <entry+0x2204>
    c.branch(C::Hi,0x42104)?;
    // 0x421a4: ldr r12, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,24)?)?;
    // 0x421a8: sub r0, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(5)?,4)?)?;
    // 0x421ac: mov r6, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,15)?)?;
    // 0x421b0: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x421b4: str r3, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,36)?)?;
    // 0x421b8: moveq r6, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // 0x421bc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x421c0: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x421c4: ldreq r1, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(5)?,-4)?)?;
    // 0x421c8: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x421cc: streq r1, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(5)?,-4)?)?;
    // 0x421d0: mov r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // 0x421d4: str r1, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,-4)?)?;
    // 0x421d8: ldr r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,20)?)?;
    // 0x421dc: b 0x42200 <entry+0x2300>
    c.branch(C::Al,0x42200)?;
    // 0x421e0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x421e4: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x421e8: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x421ec: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x421f0: mov r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // 0x421f4: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x421f8: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x421fc: ldr r12, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,24)?)?;
    // 0x42200: add lr, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R0,8)?)?;
    // 0x42204: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // 0x42208: str r6, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x4220c: str lr, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R12,0)?)?;
    // 0x42210: ldr r3, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,40)?)?;
    // 0x42214: ldrb r2, [r3, #0x495]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(3)?,1173)?)?;
    // 0x42218: mov r5, r3
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // 0x4221c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x42220: beq 0x42290 <entry+0x2390>
    c.branch(C::Eq,0x42290)?;
    // 0x42224: ldr r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,4)?)?;
    // 0x42228: and r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,255)?)?;
    // 0x4222c: sub r6, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42230: asr r3, r6, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(6)?,Shift::Asr,31)?)?;
    // 0x42234: add r3, r6, r3, lsr #30
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsr,30,false)?)?;
    // 0x42238: add r6, r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,3)?)?;
    // 0x4223c: cmp r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,7)?)?;
    // 0x42240: ldr r6, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,2744)?)?;
    // 0x42244: asr r3, r3, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Asr,2)?)?;
    // 0x42248: movlo r3, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x4224c: ldrb r6, [r6, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(6)?,224)?)?;
    // 0x42250: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x42254: ldrne r6, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(6)?,R::new(5)?,644)?)?;
    // 0x42258: cmpne r6, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(6)?,4096)?)?;
    // 0x4225c: bne 0x422c4 <entry+0x23c4>
    c.branch(C::Ne,0x422c4)?;
    // 0x42260: mov r2, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,388)?)?;
    // 0x42264: ldrb r3, [r9, #0x17e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,382)?)?;
    // 0x42268: ldrb r6, [r9, #0x182]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(9)?,386)?)?;
    // 0x4226c: ldrsh r1, [r9, r2]
    c.emit(load_signed_halfword_register(C::Al,R::R1,R::new(9)?,R::R2)?)?;
    // 0x42270: mov r2, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,384)?)?;
    // 0x42274: ldrh r2, [r9, r2]
    c.emit(load_halfword_register(C::Al,R::R2,R::new(9)?,R::R2)?)?;
    // 0x42278: orr r2, r3, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,8,false)?)?;
    // 0x4227c: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x42280: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // 0x42284: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x42288: orr r6, r2, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R2,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x4228c: b 0x4233c <entry+0x243c>
    c.branch(C::Al,0x4233c)?;
    // 0x42290: mov r3, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,384)?)?;
    // 0x42294: ldrb r6, [r9, #0x17e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(9)?,382)?)?;
    // 0x42298: mov r2, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,388)?)?;
    // 0x4229c: ldrb r5, [r9, #0x182]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(9)?,386)?)?;
    // 0x422a0: ldrh r3, [r9, r3]
    c.emit(load_halfword_register(C::Al,R::new(3)?,R::new(9)?,R::new(3)?)?)?;
    // 0x422a4: ldrh r2, [r9, r2]
    c.emit(load_halfword_register(C::Al,R::R2,R::new(9)?,R::R2)?)?;
    // 0x422a8: orr r3, r6, r3, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x422ac: mov r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,255)?)?;
    // 0x422b0: orr r6, r6, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,65280)?)?;
    // 0x422b4: and r3, r3, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x422b8: orr r3, r3, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x422bc: orr r2, r3, r2, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,24,false)?)?;
    // 0x422c0: b 0x42344 <entry+0x2444>
    c.branch(C::Al,0x42344)?;
    // 0x422c4: add r6, r3, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(3)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x422c8: mov r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,65280)?)?;
    // 0x422cc: mov r4, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,16711680)?)?;
    // 0x422d0: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x422d4: add r6, r6, r6, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(6)?,Shift::Lsr,31,false)?)?;
    // 0x422d8: add r6, r2, r6, asr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R2,R::new(6)?,Shift::Asr,1,false)?)?;
    // 0x422dc: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x422e0: movge r6, r1
    c.emit(arm32::mov_register(C::Ge,R::new(6)?,R::R1))?;
    // 0x422e4: and r6, r5, r6, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,8,false)?)?;
    // 0x422e8: add r5, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(3)?,1)?)?;
    // 0x422ec: add r5, r5, r5, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(5)?,R::new(5)?,Shift::Lsr,31,false)?)?;
    // 0x422f0: add r5, r2, r5, asr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::new(5)?,Shift::Asr,1,false)?)?;
    // 0x422f4: cmp r5, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x422f8: movge r5, r1
    c.emit(arm32::mov_register(C::Ge,R::new(5)?,R::R1))?;
    // 0x422fc: and r5, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,255)?)?;
    // 0x42300: orr r6, r6, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x42304: add r5, r3, r3, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(3)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x42308: rsb r3, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x4230c: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x42310: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x42314: add r5, r5, r5, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(5)?,R::new(5)?,Shift::Lsr,31,false)?)?;
    // 0x42318: add r3, r3, r3, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsr,31,false)?)?;
    // 0x4231c: add r5, r2, r5, asr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::new(5)?,Shift::Asr,1,false)?)?;
    // 0x42320: add r2, r2, r3, asr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Asr,1,false)?)?;
    // 0x42324: cmp r5, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x42328: movge r5, r1
    c.emit(arm32::mov_register(C::Ge,R::new(5)?,R::R1))?;
    // 0x4232c: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x42330: and r5, r4, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x42334: movlt r1, r2
    c.emit(arm32::mov_register(C::Lt,R::R1,R::R2))?;
    // 0x42338: orr r6, r6, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4233c: bic r2, r6, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::new(6)?,4278190080)?)?;
    // 0x42340: orr r2, r2, r1, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,24,false)?)?;
    // 0x42344: tst lr, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::LR,31)?)?;
    // 0x42348: mov r1, #92
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,92)?)?;
    // 0x4234c: ldr r6, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,40)?)?;
    // 0x42350: ldreq r3, [lr]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::LR,0)?)?;
    // 0x42354: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x42358: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x4235c: streq r3, [lr]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::LR,0)?)?;
    // 0x42360: str r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x42364: add r1, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,16)?)?;
    // 0x42368: str r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x4236c: str r1, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x42370: ldr r2, [r6, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,744)?)?;
    // 0x42374: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x42378: beq 0x4241c <entry+0x251c>
    c.branch(C::Eq,0x4241c)?;
    // 0x4237c: ldrb r2, [r9, #0x274]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,628)?)?;
    // 0x42380: mov r3, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,458752)?)?;
    // 0x42384: mov r6, #15728640
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,15728640)?)?;
    // 0x42388: ldrb r5, [r9, #0x272]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(9)?,626)?)?;
    // 0x4238c: ldrb r4, [r9, #0x273]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(9)?,627)?)?;
    // 0x42390: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x42394: and r2, r3, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,16,false)?)?;
    // 0x42398: ldrb r3, [r9, #0x275]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,629)?)?;
    // 0x4239c: and r3, r6, r3, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,20,false)?)?;
    // 0x423a0: ldrb r6, [r9, #0x271]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(9)?,625)?)?;
    // 0x423a4: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x423a8: ldrb r3, [r9, #0x277]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,631)?)?;
    // 0x423ac: orr r2, r2, r3, lsl #28
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,28,false)?)?;
    // 0x423b0: ldrb r3, [r9, #0x276]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,630)?)?;
    // 0x423b4: and r3, r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,7)?)?;
    // 0x423b8: orr r2, r2, r3, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,24,false)?)?;
    // 0x423bc: ldrb r3, [r9, #0x270]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,624)?)?;
    // 0x423c0: and r3, r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,15)?)?;
    // 0x423c4: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x423c8: mov r3, #240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,240)?)?;
    // 0x423cc: and r3, r3, r6, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,4,false)?)?;
    // 0x423d0: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x423d4: mov r3, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3840)?)?;
    // 0x423d8: and r3, r3, r5, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,8,false)?)?;
    // 0x423dc: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x423e0: and r3, r4, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,15)?)?;
    // 0x423e4: orr r2, r2, r3, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,12,false)?)?;
    // 0x423e8: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x423ec: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x423f0: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x423f4: mov r1, #3168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3168)?)?;
    // 0x423f8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x423fc: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x42400: add r1, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,24)?)?;
    // 0x42404: str r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,20)?)?;
    // 0x42408: str r1, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x4240c: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x42410: ldr r6, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,40)?)?;
    // 0x42414: add r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3)?)?;
    // 0x42418: b 0x42424 <entry+0x2524>
    c.branch(C::Al,0x42424)?;
    // 0x4241c: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x42420: add r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // 0x42424: ldrb r2, [r6, #0xb78]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,2936)?)?;
    // 0x42428: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4242c: beq 0x424a8 <entry+0x25a8>
    c.branch(C::Eq,0x424a8)?;
    // 0x42430: ldr r2, [r9, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,400)?)?;
    // 0x42434: ldr r7, [r6, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,2744)?)?;
    // 0x42438: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4243c: ldrb r7, [r7, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(7)?,224)?)?;
    // 0x42440: ldrne r2, [r9, #0x634]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(9)?,1588)?)?;
    // 0x42444: ldrbne r3, [r2, #0x28]
    c.emit(arm32::transfer_byte(C::Ne,true,R::new(3)?,R::R2,40)?)?;
    // 0x42448: ldrb r2, [r9, #0x5d9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,1497)?)?;
    // 0x4244c: moveq r3, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x42450: lsl r2, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,4)?)?;
    // 0x42454: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x42458: beq 0x42468 <entry+0x2568>
    c.branch(C::Eq,0x42468)?;
    // 0x4245c: ldrb r3, [r9, #0x5d8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,1496)?)?;
    // 0x42460: and r7, r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(3)?,15)?)?;
    // 0x42464: b 0x42474 <entry+0x2574>
    c.branch(C::Al,0x42474)?;
    // 0x42468: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x4246c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x42470: movne r7, #15
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,15)?)?;
    // 0x42474: and r3, r7, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(7)?,15)?)?;
    // 0x42478: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4247c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x42480: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x42484: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x42488: orr r2, r2, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2147483648)?)?;
    // 0x4248c: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x42490: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x42494: mov r3, #7296
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7296)?)?;
    // 0x42498: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // 0x4249c: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,4)?)?;
    // 0x424a0: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x424a4: str r1, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x424a8: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x424ac: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(CODE_END)?;
    for word in CONTROLLER_LITERALS{c.emit(word)?;}
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();classify_scale_and_mode(&mut c)?;finish_controller_packet(&mut c)?;Ok(c.bytes)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]
    fn controller_packets_cover_code_and_separate_jump_tables(){
        let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
        assert_eq!((JUMP_TABLE_START-START)+(CODE_END-JUMP_TABLE_END),886*4);
        for (i,x) in SCALER_CLASSES.iter().enumerate(){
            let at=JUMP_TABLE_START-START+i*4;
            assert_eq!(u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()),*x);
        }
        for (i,x) in CONTROLLER_LITERALS.iter().enumerate(){
            let at=CODE_END-START+i*4;
            assert_eq!(u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()),*x);
        }
    }
    #[test]
    fn long_multiply_and_halfword_register_loads_have_known_a32_encodings(){
        let r3=R::new(3).unwrap();let r4=R::new(4).unwrap();let r6=R::new(6).unwrap();let r9=R::new(9).unwrap();
        assert_eq!(unsigned_long_multiply(C::Al,R::R0,R::R1,R::R2,r3).unwrap(),0xe081_0392);
        assert_eq!(load_halfword_register(C::Al,R::R2,r9,R::R2).unwrap(),0xe199_20b2);
        assert_eq!(load_signed_halfword_register(C::Al,R::R1,r9,R::R2).unwrap(),0xe199_10f2);
        assert!(unsigned_long_multiply(C::Al,r4,r4,R::R2,r6).is_err());
        assert!(load_halfword_register(C::Al,R::PC,R::R0,R::R1).is_err());
    }
}
