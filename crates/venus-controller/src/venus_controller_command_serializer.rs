// SPDX-License-Identifier: GPL-3.0-only
//! Complete A32 controller command serializer at runtime `[0x43188,0x43b50)`.
//! Program bytes are reconstructed from register/ISA-level expressions;
//! the last three literal words are named non-executable constants.
//! The routine's normal return is at `0x43b38`, with an additional branch
//! island `[0x43b3c,0x43b44)` preceding the literal pool.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};

pub const START:usize=0x43188;
pub const CODE_END:usize=0x43b44;
pub const END:usize=0x43b50;
const DATA_WORDS:[u32;3]=[
  0x000e_e028, // controller firmware diagnostic location
  0xfe0f_37ff, // codec bitfield gating mask
  0x0010_59e4, // controller runtime descriptor pointer
];

/// A32 signed halfword load with postindexed base update (`LDRSH Rt,[Rn],#imm`).
/// Refuse PC operands and destination==base writeback aliases.
fn load_signed_halfword_postindexed(cond:C,rd:R,base:R,signed_offset:i32)->Result<u32,String>{
    if rd==R::PC || base==R::PC || rd==base || signed_offset.unsigned_abs()>255 {
        return Err("invalid LDRSH postindexed address".into());
    }
    let magnitude=signed_offset.unsigned_abs();
    Ok(((cond as u32)<<28)|0x0050_00f0|if signed_offset>=0{1<<23}else{0}|
       (base.number()<<16)|(rd.number()<<12)|
       ((magnitude&0xf0)<<4)|(magnitude&0x0f))
}

struct Code {at:usize,bytes:Vec<u8>}
impl Code {
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,a:usize)->Result<(),String>{
        if self.at!=a {return Err(format!("codec command writer 0x{:x}, expected 0x{a:x}",self.at));}
        Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at) || self.at&3!=0{return Err("Agent 20-04 invalid address".into());}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(cond,self.at as u32,target as u32,false)?)
    }
    fn call(&mut self,target:usize)->Result<(),String>{
        self.emit(arm32::branch(C::Al,self.at as u32,target as u32,true)?)
    }
    fn literal(&mut self,rd:R,target:usize)->Result<(),String>{
        let offset=i32::try_from(target as isize-self.at as isize-8).map_err(|_|"literal distance overflow")?;
        self.emit(arm32::transfer_word(C::Al,true,rd,R::PC,offset)?)
    }
}

/// Emit serialized controller commands from bounded codec state values,
/// including platform callbacks, record-size checks, and status flags.
fn serialize_codec_commands(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x43188: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x4318c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x43190: sub sp, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,44)?)?;
    // 0x43194: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x43198: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4319c: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x431a0: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x431a4: ldr r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,644)?)?;
    // 0x431a8: ldr r10, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(5)?,44)?)?;
    // 0x431ac: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x431b0: beq 0x431ec <entry+0x32ec>
    c.branch(C::Eq,0x431ec)?;
    // 0x431b4: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x431b8: beq 0x43200 <entry+0x3300>
    c.branch(C::Eq,0x43200)?;
    // 0x431bc: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x431c0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x431c4: beq 0x43204 <entry+0x3304>
    c.branch(C::Eq,0x43204)?;
    // 0x431c8: ldr r2, [pc, #0x974]
    c.literal(R::R2,0x43b44)?;
    // 0x431cc: mov r1, #344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,344)?)?;
    // 0x431d0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x431d4: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x431d8: bl 0x17b0 <entry+0xfffffffffffc18b0>
    c.call(0x17b0)?;
    // 0x431dc: ldr r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,644)?)?;
    // 0x431e0: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x431e4: beq 0x431f0 <entry+0x32f0>
    c.branch(C::Eq,0x431f0)?;
    // 0x431e8: b 0x43204 <entry+0x3304>
    c.branch(C::Al,0x43204)?;
    // 0x431ec: mov r4, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,9)?)?;
    // 0x431f0: ldrb r0, [r10, #0x3e9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1001)?)?;
    // 0x431f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x431f8: orreq r4, r4, #64
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(4)?,R::new(4)?,64)?)?;
    // 0x431fc: b 0x43208 <entry+0x3308>
    c.branch(C::Al,0x43208)?;
    // 0x43200: mov r4, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,7)?)?;
    // 0x43204: orr r4, r4, #64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,64)?)?;
    // 0x43208: ldrb r0, [r5, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2462)?)?;
    // 0x4320c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x43210: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x43214: ldr r0, [pc, #0x92c]
    c.literal(R::R0,0x43b48)?;
    // 0x43218: ldr r2, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2744)?)?;
    // 0x4321c: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x43220: ldrb r3, [r2, #0x143]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,323)?)?;
    // 0x43224: ldrb r7, [r2, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R2,325)?)?;
    // 0x43228: ldrb r6, [r2, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R2,336)?)?;
    // 0x4322c: ldrb r2, [r2, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,339)?)?;
    // 0x43230: and r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x43234: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x43238: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4323c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x43240: orr r0, r0, r2, lsl #11
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,11,false)?)?;
    // 0x43244: orr r0, r0, r1, lsl #14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,14,false)?)?;
    // 0x43248: mov r1, #14680064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14680064)?)?;
    // 0x4324c: and r1, r1, r6, lsl #21
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsl,21,false)?)?;
    // 0x43250: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x43254: orr r1, r3, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x43258: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x4325c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x43260: movne r7, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x43264: orr r0, r0, r3, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,24,false)?)?;
    // 0x43268: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // 0x4326c: orr r0, r0, r7, lsl #20
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(7)?,Shift::Lsl,20,false)?)?;
    // 0x43270: bne 0x43294 <entry+0x3394>
    c.branch(C::Ne,0x43294)?;
    // 0x43274: ldr r1, [r5, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,692)?)?;
    // 0x43278: ldr r2, [r5, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,716)?)?;
    // 0x4327c: cmp r1, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,1,false)?)?;
    // 0x43280: bhi 0x43294 <entry+0x3394>
    c.branch(C::Hi,0x43294)?;
    // 0x43284: ldr r1, [r5, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,696)?)?;
    // 0x43288: ldr r2, [r5, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,720)?)?;
    // 0x4328c: cmp r1, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,1,false)?)?;
    // 0x43290: bls 0x43b3c <entry+0x3c3c>
    c.branch(C::Ls,0x43b3c)?;
    // 0x43294: orr r1, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,16)?)?;
    // 0x43298: ldrb r0, [r5, #0xb7a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2938)?)?;
    // 0x4329c: mov r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,255)?)?;
    // 0x432a0: mov r4, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,2)?)?;
    // 0x432a4: mov r9, r8
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(8)?))?;
    // 0x432a8: orr r6, r6, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,256)?)?;
    // 0x432ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x432b0: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x432b4: orrne r1, r1, #268435456
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R1,R::R1,268435456)?)?;
    // 0x432b8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x432bc: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x432c0: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x432c4: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x432c8: mov r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,14336)?)?;
    // 0x432cc: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x432d0: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x432d4: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // 0x432d8: add r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,1)?)?;
    // 0x432dc: add r1, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2)?)?;
    // 0x432e0: str r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-32)?)?;
    // 0x432e4: add r2, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,8)?)?;
    // 0x432e8: str r2, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x432ec: ldr r3, [r10, #0x368]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,872)?)?;
    // 0x432f0: ldr r7, [r10, #0x36c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,876)?)?;
    // 0x432f4: add r3, r3, r3, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsr,31,false)?)?;
    // 0x432f8: and r3, r6, r3, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsr,1,false)?)?;
    // 0x432fc: ldrb r6, [r5, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,988)?)?;
    // 0x43300: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x43304: movne r4, #4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x43308: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x4330c: mul r6, r7, r4
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::new(7)?,R::new(4)?)?)?;
    // 0x43310: mov r7, #16646144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16646144)?)?;
    // 0x43314: orr r7, r7, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,16777216)?)?;
    // 0x43318: and r7, r7, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x4331c: orr r3, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x43320: ldreq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R2,0)?)?;
    // 0x43324: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x43328: streq r7, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R2,0)?)?;
    // 0x4332c: mov r2, #216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,216)?)?;
    // 0x43330: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x43334: str r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,8)?)?;
    // 0x43338: str r3, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,12)?)?;
    // 0x4333c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x43340: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x43344: ldr r2, [pc, #0x7f8]
    c.literal(R::R2,0x43b44)?;
    // 0x43348: mov r1, #137
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,137)?)?;
    // 0x4334c: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x43350: orr r1, r1, #7424
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7424)?)?;
    // 0x43354: ldr r4, [r10, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,420)?)?;
    // 0x43358: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // 0x4335c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x43360: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x43364: bl 0x17b0 <entry+0xfffffffffffc18b0>
    c.call(0x17b0)?;
    // 0x43368: ldrb r0, [r5, #0xcc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,3272)?)?;
    // 0x4336c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x43370: bne 0x434b8 <entry+0x35b8>
    c.branch(C::Ne,0x434b8)?;
    // 0x43374: ldr r6, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,52)?)?;
    // 0x43378: ldr r4, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,60)?)?;
    // 0x4337c: ldr r7, [r5, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,280)?)?;
    // 0x43380: ldr r0, [r5, #0x524]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1316)?)?;
    // 0x43384: ldr r1, [r5, #0xad4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2772)?)?;
    // 0x43388: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x4338c: mov r8, r4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(4)?))?;
    // 0x43390: addne r8, r6, r0
    c.emit(arm32::data_register(C::Ne,Op::Add,R::new(8)?,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x43394: ldrb r0, [r5, #0x350]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,848)?)?;
    // 0x43398: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4339c: bne 0x43444 <entry+0x3544>
    c.branch(C::Ne,0x43444)?;
    // 0x433a0: ldr r0, [r5, #0xad0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2768)?)?;
    // 0x433a4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x433a8: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x433ac: ldr r2, [r5, #0x528]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,1320)?)?;
    // 0x433b0: ldr r0, [r5, #0x520]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1312)?)?;
    // 0x433b4: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x433b8: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // 0x433bc: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // 0x433c0: add r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x433c4: ldr r1, [r5, #0x52c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1324)?)?;
    // 0x433c8: str r10, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,36)?)?;
    // 0x433cc: mov r10, r9
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(9)?))?;
    // 0x433d0: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x433d4: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x433d8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x433dc: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x433e0: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x433e4: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x433e8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x433ec: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x433f0: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x433f4: sub r1, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,32)?)?;
    // 0x433f8: ldr r0, [r5, #0x530]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1328)?)?;
    // 0x433fc: str r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
    // 0x43400: ldr r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // 0x43404: ldr r10, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,36)?)?;
    // 0x43408: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x4340c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43410: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x43414: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x43418: add r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4341c: addeq r1, r2, r4
    c.emit(arm32::data_register(C::Eq,Op::Add,R::R1,R::R2,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x43420: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x43424: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x43428: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x4342c: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x43430: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43434: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x43438: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x4343c: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43440: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // 0x43444: ldrb r0, [r5, #0x3ba]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,954)?)?;
    // 0x43448: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4344c: beq 0x434b8 <entry+0x35b8>
    c.branch(C::Eq,0x434b8)?;
    // 0x43450: add r0, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x43454: ldr r1, [r5, #0xacc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2764)?)?;
    // 0x43458: mov r7, r9
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // 0x4345c: sub r4, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,32)?)?;
    // 0x43460: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x43464: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x43468: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4346c: str r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // 0x43470: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43474: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x43478: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4347c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43480: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x43484: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43488: ldr r0, [r5, #0x524]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1316)?)?;
    // 0x4348c: ldr r1, [r5, #0x528]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1320)?)?;
    // 0x43490: str r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // 0x43494: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43498: stm sp, {r6, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit() | R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4349c: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x434a0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x434a4: sub r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x434a8: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x434ac: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x434b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x434b4: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x434b8: ldr r0, [r5, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,664)?)?;
    // 0x434bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x434c0: beq 0x43568 <entry+0x3668>
    c.branch(C::Eq,0x43568)?;
    // 0x434c4: ldr r6, [r5, #0x764]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,1892)?)?;
    // 0x434c8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x434cc: sub r8, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,32)?)?;
    // 0x434d0: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x434d4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x434d8: ldr r1, [r5, #0x7bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1980)?)?;
    // 0x434dc: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x434e0: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x434e4: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x434e8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x434ec: mla r2, r0, r6, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R0,R::new(6)?,R::R1)?)?;
    // 0x434f0: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x434f4: stmib sp, {r2, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit() | R::new(6)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x434f8: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x434fc: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43500: ldr r1, [r5, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,664)?)?;
    // 0x43504: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x43508: and r0, r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(4)?,255)?)?;
    // 0x4350c: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x43510: bhi 0x434d8 <entry+0x35d8>
    c.branch(C::Hi,0x434d8)?;
    // 0x43514: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x43518: beq 0x43568 <entry+0x3668>
    c.branch(C::Eq,0x43568)?;
    // 0x4351c: ldr r6, [r5, #0x768]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,1896)?)?;
    // 0x43520: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43524: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x43528: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x4352c: ldr r1, [r5, #0x7c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1984)?)?;
    // 0x43530: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43534: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43538: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x4353c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43540: mla r2, r0, r6, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R0,R::new(6)?,R::R1)?)?;
    // 0x43544: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x43548: stmib sp, {r2, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit() | R::new(6)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4354c: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x43550: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43554: ldr r1, [r5, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,664)?)?;
    // 0x43558: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x4355c: and r0, r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(4)?,255)?)?;
    // 0x43560: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x43564: bhi 0x4352c <entry+0x362c>
    c.branch(C::Hi,0x4352c)?;
    // 0x43568: ldr r0, [r10, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,424)?)?;
    // 0x4356c: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x43570: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x43574: sub r8, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,32)?)?;
    // 0x43578: mov r4, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,2)?)?;
    // 0x4357c: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43580: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43584: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43588: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x4358c: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x43590: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x43594: stmib sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43598: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4359c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x435a0: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x435a4: ldr r0, [r10, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,424)?)?;
    // 0x435a8: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x435ac: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x435b0: ldr r3, [r5, #0xaf4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,2804)?)?;
    // 0x435b4: ldr r7, [r5, #0xaf8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,2808)?)?;
    // 0x435b8: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x435bc: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x435c0: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x435c4: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x435c8: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x435cc: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x435d0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x435d4: add r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x435d8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x435dc: stmib sp, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(7)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x435e0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x435e4: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x435e8: ldrb r0, [r5, #0x3b9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,953)?)?;
    // 0x435ec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x435f0: beq 0x4365c <entry+0x375c>
    c.branch(C::Eq,0x4365c)?;
    // 0x435f4: ldr r0, [r10, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,424)?)?;
    // 0x435f8: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x435fc: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43600: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43604: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x43608: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x4360c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43610: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43614: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43618: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x4361c: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43620: ldr r0, [r10, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,424)?)?;
    // 0x43624: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x43628: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x4362c: ldr r3, [r5, #0xaf4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,2804)?)?;
    // 0x43630: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43634: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43638: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x4363c: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x43640: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x43644: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x43648: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4364c: stmib sp, {r0, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(3)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43650: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43654: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43658: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x4365c: ldr r0, [r10, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,432)?)?;
    // 0x43660: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x43664: beq 0x437b4 <entry+0x38b4>
    c.branch(C::Eq,0x437b4)?;
    // 0x43668: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x4366c: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x43670: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x43674: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43678: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x4367c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43680: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x43684: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x43688: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x4368c: stmib sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43690: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43694: mov r2, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,14)?)?;
    // 0x43698: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x4369c: ldr r0, [r10, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,432)?)?;
    // 0x436a0: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x436a4: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x436a8: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x436ac: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x436b0: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x436b4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x436b8: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x436bc: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x436c0: stmib sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x436c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x436c8: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x436cc: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x436d0: ldr r0, [r10, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,432)?)?;
    // 0x436d4: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x436d8: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x436dc: ldr r3, [r5, #0xaf4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,2804)?)?;
    // 0x436e0: ldr r7, [r5, #0xaf8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,2808)?)?;
    // 0x436e4: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x436e8: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x436ec: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x436f0: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x436f4: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x436f8: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x436fc: mov r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,11)?)?;
    // 0x43700: add r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x43704: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43708: stmib sp, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(7)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4370c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43710: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43714: ldrb r0, [r5, #0x3b9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,953)?)?;
    // 0x43718: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4371c: beq 0x437b4 <entry+0x38b4>
    c.branch(C::Eq,0x437b4)?;
    // 0x43720: ldr r0, [r10, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,432)?)?;
    // 0x43724: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x43728: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x4372c: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43730: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x43734: mov r2, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,14)?)?;
    // 0x43738: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x4373c: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43740: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43744: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x43748: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x4374c: ldr r0, [r10, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,432)?)?;
    // 0x43750: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x43754: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43758: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x4375c: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x43760: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x43764: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43768: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4376c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43770: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x43774: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43778: ldr r0, [r10, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,432)?)?;
    // 0x4377c: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x43780: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x43784: ldr r3, [r5, #0xaf4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,2804)?)?;
    // 0x43788: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x4378c: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43790: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x43794: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x43798: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x4379c: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x437a0: mov r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,11)?)?;
    // 0x437a4: stmib sp, {r0, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(3)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x437a8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x437ac: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x437b0: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x437b4: ldr r0, [r10, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,436)?)?;
    // 0x437b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x437bc: beq 0x4390c <entry+0x3a0c>
    c.branch(C::Eq,0x4390c)?;
    // 0x437c0: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x437c4: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x437c8: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x437cc: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x437d0: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x437d4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x437d8: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x437dc: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x437e0: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x437e4: stmib sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x437e8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x437ec: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x437f0: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x437f4: ldr r0, [r10, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,436)?)?;
    // 0x437f8: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x437fc: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x43800: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43804: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43808: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x4380c: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x43810: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x43814: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x43818: stmib sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x4381c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43820: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x43824: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43828: ldr r0, [r10, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,436)?)?;
    // 0x4382c: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x43830: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x43834: ldr r3, [r5, #0xaf4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,2804)?)?;
    // 0x43838: ldr r7, [r5, #0xaf8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,2808)?)?;
    // 0x4383c: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43840: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43844: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x43848: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4384c: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x43850: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x43854: mov r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,11)?)?;
    // 0x43858: add r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x4385c: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x43860: stmib sp, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(7)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43864: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43868: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x4386c: ldrb r0, [r5, #0x3b9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,953)?)?;
    // 0x43870: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x43874: beq 0x4390c <entry+0x3a0c>
    c.branch(C::Eq,0x4390c)?;
    // 0x43878: ldr r0, [r10, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,436)?)?;
    // 0x4387c: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x43880: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43884: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x43888: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x4388c: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x43890: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43894: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43898: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4389c: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x438a0: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x438a4: ldr r0, [r10, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,436)?)?;
    // 0x438a8: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x438ac: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x438b0: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x438b4: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x438b8: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x438bc: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x438c0: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x438c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x438c8: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x438cc: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x438d0: ldr r0, [r10, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,436)?)?;
    // 0x438d4: ldr r1, [r5, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2796)?)?;
    // 0x438d8: ldr r2, [r5, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2800)?)?;
    // 0x438dc: ldr r3, [r5, #0xaf4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,2804)?)?;
    // 0x438e0: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x438e4: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x438e8: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x438ec: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x438f0: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x438f4: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x438f8: mov r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,11)?)?;
    // 0x438fc: stmib sp, {r0, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(3)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43900: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43904: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x43908: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x4390c: ldr r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,644)?)?;
    // 0x43910: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x43914: bne 0x4396c <entry+0x3a6c>
    c.branch(C::Ne,0x4396c)?;
    // 0x43918: ldr r2, [pc, #0x22c]
    c.literal(R::R2,0x43b4c)?;
    // 0x4391c: add r0, r10, #126
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,126)?)?;
    // 0x43920: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x43924: add r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // 0x43928: ldr r7, [r10, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,400)?)?;
    // 0x4392c: ldr r6, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,2744)?)?;
    // 0x43930: ldrsh r3, [r0], #2
    c.emit(load_signed_halfword_postindexed(C::Al,R::new(3)?,R::R0,2)?)?;
    // 0x43934: add r7, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x43938: ldrb r3, [r2, r3]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(3)?,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x4393c: ldrb r7, [r7, #0x4c0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(7)?,1216)?)?;
    // 0x43940: add r3, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x43944: ldr r7, [r10, #0x518]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,1304)?)?;
    // 0x43948: strb r3, [r7, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(3)?,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
    // 0x4394c: ldr r3, [r10, #0x518]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,1304)?)?;
    // 0x43950: ldrb r7, [r3, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(7)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x43954: cmp r7, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,127)?)?;
    // 0x43958: movhs r7, #127
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(7)?,R::R0,127)?)?;
    // 0x4395c: strb r7, [r3, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::new(7)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x43960: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x43964: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x43968: bne 0x43928 <entry+0x3a28>
    c.branch(C::Ne,0x43928)?;
    // 0x4396c: ldr r0, [r10, #0x518]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1304)?)?;
    // 0x43970: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x43974: mov r4, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,256)?)?;
    // 0x43978: bl 0x2014 <entry+0xfffffffffffc2114>
    c.call(0x2014)?;
    // 0x4397c: ldr r0, [r10, #0x518]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1304)?)?;
    // 0x43980: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x43984: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43988: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x4398c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x43990: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x43994: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43998: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x4399c: stmib sp, {r0, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(4)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x439a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x439a4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x439a8: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x439ac: ldr r0, [r10, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,444)?)?;
    // 0x439b0: ldr r1, [r5, #0x69c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1692)?)?;
    // 0x439b4: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x439b8: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x439bc: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x439c0: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x439c4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x439c8: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x439cc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x439d0: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x439d4: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x439d8: ldr r0, [r10, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,440)?)?;
    // 0x439dc: ldr r1, [r5, #0x69c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1692)?)?;
    // 0x439e0: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x439e4: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x439e8: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x439ec: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x439f0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x439f4: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x439f8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x439fc: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x43a00: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43a04: ldr r1, [r5, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,664)?)?;
    // 0x43a08: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x43a0c: beq 0x43a84 <entry+0x3b84>
    c.branch(C::Eq,0x43a84)?;
    // 0x43a10: ldr r7, [pc, #0x12c]
    c.literal(R::new(7)?,0x43b44)?;
    // 0x43a14: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x43a18: ldr r0, [r5, #0x688]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1672)?)?;
    // 0x43a1c: blx 0x83c4c <entry+0x43d4c>
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x43a20: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x43a24: ldr r8, [r10, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(10)?,456)?)?;
    // 0x43a28: mov r1, #246
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,246)?)?;
    // 0x43a2c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43a30: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x43a34: tst r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,255)?)?;
    // 0x43a38: orr r1, r1, #7424
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7424)?)?;
    // 0x43a3c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x43a40: bl 0x17b0 <entry+0xfffffffffffc18b0>
    c.call(0x17b0)?;
    // 0x43a44: sub r0, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,32)?)?;
    // 0x43a48: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43a4c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x43a50: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x43a54: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43a58: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x43a5c: mla r0, r6, r4, r8
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(6)?,R::new(4)?,R::new(8)?)?)?;
    // 0x43a60: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x43a64: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x43a68: stmib sp, {r0, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(6)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43a6c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x43a70: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43a74: ldr r1, [r5, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,664)?)?;
    // 0x43a78: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x43a7c: cmp r4, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x43a80: blo 0x43a18 <entry+0x3b18>
    c.branch(C::Cc,0x43a18)?;
    // 0x43a84: ldr r0, [r10, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,452)?)?;
    // 0x43a88: ldr r1, [r5, #0x684]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1668)?)?;
    // 0x43a8c: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // 0x43a90: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43a94: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43a98: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x43a9c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x43aa0: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43aa4: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x43aa8: mov r2, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,9)?)?;
    // 0x43aac: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43ab0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43ab4: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x43ab8: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43abc: ldr r0, [r10, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,448)?)?;
    // 0x43ac0: ldr r1, [r5, #0x684]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1668)?)?;
    // 0x43ac4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x43ac8: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43acc: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    // 0x43ad0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43ad4: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x43ad8: mov r2, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,9)?)?;
    // 0x43adc: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43ae0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43ae4: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x43ae8: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43aec: ldr r0, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2744)?)?;
    // 0x43af0: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x43af4: ldrb r0, [r0, #0xe8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,232)?)?;
    // 0x43af8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x43afc: beq 0x43b30 <entry+0x3c30>
    c.branch(C::Eq,0x43b30)?;
    // 0x43b00: ldr r0, [r10, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,464)?)?;
    // 0x43b04: ldr r1, [r5, #0x678]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1656)?)?;
    // 0x43b08: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // 0x43b0c: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x43b10: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x43b14: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x43b18: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x43b1c: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x43b20: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x43b24: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x43b28: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x43b2c: bl 0x3e02c <entry+0xffffffffffffe12c>
    c.call(0x3e02c)?;
    // 0x43b30: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x43b34: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x43b38: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x43b3c: bic r1, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R0,16)?)?;
    // 0x43b40: b 0x43298 <entry+0x3398>
    c.branch(C::Al,0x43298)?;
    c.check(CODE_END)?;
    for word in DATA_WORDS {c.emit(word)?;}
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();serialize_codec_commands(&mut c)?;Ok(c.bytes)
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]
    fn ldrsh_postindex_checked_against_a32_instruction(){
        assert_eq!(load_signed_halfword_postindexed(C::Al,R::new(3).unwrap(),R::R0,2).unwrap(),0xe0d0_30f2);
        assert!(load_signed_halfword_postindexed(C::Al,R::R0,R::R0,2).is_err());
        assert!(load_signed_halfword_postindexed(C::Al,R::R2,R::PC,1).is_err());
    }
    #[test] fn precise_instructions_and_data_layout(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        assert_eq!(CODE_END-START,623*4);
        for (i,x) in DATA_WORDS.iter().enumerate(){
            let at=CODE_END-START+i*4;
            assert_eq!(u32::from_le_bytes(b[at..at+4].try_into().unwrap()),*x);
        }
    }
}
