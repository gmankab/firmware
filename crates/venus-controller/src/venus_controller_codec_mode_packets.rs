// SPDX-License-Identifier: GPL-3.0-only
//! Reconstruct the codec/controller state writer at A32 runtime
//! `[0x4272c,0x43188)`, followed by five explicitly separated literal
//! words `[0x43174,0x43188)`. This generator uses typed ARM A32 operands
//! rather than copying original opcode bytes. The function returns using
//! original `pop {...,pc}` at `0x43170`.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::venus_controller_format_registers::signed_byte_register;
use super::venus_controller_packets::{load_halfword_register,load_signed_halfword_register};

pub const START:usize=0x4272c;
pub const CODE_END:usize=0x43174;
pub const END:usize=0x43188;
const LITERAL_WORDS:[u32;5]=[
    0x2dc0_ffff, // firmware-compatible descriptor index/bit mask
    0x0010_59e4, // controller data pointer
    0xfffd_eff9, // signed controller-index offset
    0x0001_ffff, // byte/word capability mask
    0xffff_801f, // signed controller mask
];

struct Code {at:usize,bytes:Vec<u8>}
impl Code {
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at!=addr{return Err(format!("controller writer addr 0x{:x} expected 0x{addr:x}",self.at))}
        Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at)||self.at%4!=0{return Err("outside Agent 20-04 reserved interval".into());}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,condition:C,addr:usize)->Result<(),String>{
        self.emit(arm32::branch(condition,self.at as u32,addr as u32,false)?)
    }
    fn call(&mut self,addr:usize)->Result<(),String>{
        self.emit(arm32::branch(C::Al,self.at as u32,addr as u32,true)?)
    }
    fn literal(&mut self,rd:R,addr:usize)->Result<(),String>{
        let disp=i32::try_from(addr as isize-self.at as isize-8).map_err(|_|"invalid A32 literal offset")?;
        self.emit(arm32::transfer_word(C::Al,true,rd,R::PC,disp)?)
    }
}

/// Validate codec-frame state and controller feature bits, then update the
/// outgoing command records. The long branch lattice preserves conditions
/// on C/Z/N/V tests, signed narrow loads and original register lifetimes.
fn write_codec_mode_packets(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x4272c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x42730: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x42734: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // 0x42738: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x4273c: mov r10, r1
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // 0x42740: ldr r8, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(5)?,44)?)?;
    // 0x42744: ldr r1, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,644)?)?;
    // 0x42748: ldr r7, [r8, #0x368]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(8)?,872)?)?;
    // 0x4274c: ldr r6, [r8, #0x36c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(8)?,876)?)?;
    // 0x42750: ldr r3, [r8, #0x370]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,880)?)?;
    // 0x42754: ldr r0, [r8, #0x374]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,884)?)?;
    // 0x42758: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x4275c: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x42760: beq 0x42798 <entry+0x2898>
    c.branch(C::Eq,0x42798)?;
    // 0x42764: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x42768: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x4276c: orr r2, r2, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1048576)?)?;
    // 0x42770: beq 0x427b4 <entry+0x28b4>
    c.branch(C::Eq,0x427b4)?;
    // 0x42774: mov r0, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1048576)?)?;
    // 0x42778: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x4277c: bne 0x427c0 <entry+0x28c0>
    c.branch(C::Ne,0x427c0)?;
    // 0x42780: ldrb r1, [r5, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,988)?)?;
    // 0x42784: add r0, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,2)?)?;
    // 0x42788: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x4278c: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x42790: orr r0, r0, r1, lsl #23
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,23,false)?)?;
    // 0x42794: b 0x427c0 <entry+0x28c0>
    c.branch(C::Al,0x427c0)?;
    // 0x42798: ldr r1, [r5, #0x288]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,648)?)?;
    // 0x4279c: mov r0, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1048576)?)?;
    // 0x427a0: cmp r1, #77
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,77)?)?;
    // 0x427a4: moveq r0, #9437184
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,9437184)?)?;
    // 0x427a8: cmp r1, #100
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,100)?)?;
    // 0x427ac: moveq r0, #17825792
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,17825792)?)?;
    // 0x427b0: b 0x427c0 <entry+0x28c0>
    c.branch(C::Al,0x427c0)?;
    // 0x427b4: ldrb r0, [r5, #0x338]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,824)?)?;
    // 0x427b8: and r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,3)?)?;
    // 0x427bc: orr r0, r2, r0, lsl #23
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,23,false)?)?;
    // 0x427c0: ldr r2, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,400)?)?;
    // 0x427c4: str r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // 0x427c8: stm sp, {r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(6)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x427cc: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x427d0: beq 0x429b0 <entry+0x2ab0>
    c.branch(C::Eq,0x429b0)?;
    // 0x427d4: bic r0, r0, #240
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,240)?)?;
    // 0x427d8: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x427dc: orreq r0, r0, #192
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R0,R::R0,192)?)?;
    // 0x427e0: str r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
    // 0x427e4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x427e8: and r6, r0, #192
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::R0,192)?)?;
    // 0x427ec: mov r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,4)?)?;
    // 0x427f0: ldr r1, [pc, #0x97c]
    c.literal(R::R1,0x43174)?;
    // 0x427f4: ldrb r2, [r5, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,990)?)?;
    // 0x427f8: ldrb r12, [r5, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(5)?,988)?)?;
    // 0x427fc: ldr r4, [r5, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,940)?)?;
    // 0x42800: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x42804: ldr r2, [r5, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,664)?)?;
    // 0x42808: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4280c: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x42810: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x42814: mov r2, r12
    c.emit(arm32::mov_register(C::Al,R::R2,R::R12))?;
    // 0x42818: orreq r1, r1, r3, lsl #30
    c.emit(arm32::data_register(C::Eq,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,30,false)?)?;
    // 0x4281c: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x42820: movne r2, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x42824: orr r1, r1, r2, lsl #19
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,19,false)?)?;
    // 0x42828: orr r1, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x4282c: mov r2, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,33554432)?)?;
    // 0x42830: and r2, r2, r4, lsl #25
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(4)?,Shift::Lsl,25,false)?)?;
    // 0x42834: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42838: orr r2, r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R1,2147483648)?)?;
    // 0x4283c: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,0)?)?;
    // 0x42840: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x42844: ldreq r4, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::R1,0)?)?;
    // 0x42848: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x4284c: streq r4, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::R1,0)?)?;
    // 0x42850: mov r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,8)?)?;
    // 0x42854: cmp r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,64)?)?;
    // 0x42858: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x4285c: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // 0x42860: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,4)?)?;
    // 0x42864: add r2, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,8)?)?;
    // 0x42868: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,0)?)?;
    // 0x4286c: ldr r7, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,2744)?)?;
    // 0x42870: ldr r4, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,644)?)?;
    // 0x42874: add r0, r7, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,24)?)?;
    // 0x42878: addeq r0, r7, #32
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::new(7)?,32)?)?;
    // 0x4287c: cmp r4, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,16384)?)?;
    // 0x42880: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x42884: moveq r9, #8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(9)?,R::R0,8)?)?;
    // 0x42888: cmp r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,4096)?)?;
    // 0x4288c: moveq r9, #6
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(9)?,R::R0,6)?)?;
    // 0x42890: cmp r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,64)?)?;
    // 0x42894: sub r4, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x42898: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4289c: rsb lr, r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::LR,R::new(4)?,0)?)?;
    // 0x428a0: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x428a4: and r3, lr, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::LR,R::R0,Shift::Lsl,0,false)?)?;
    // 0x428a8: orr r4, r3, r4, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(3)?,R::new(4)?,Shift::Lsl,16,false)?)?;
    // 0x428ac: add r3, r7, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(7)?,28)?)?;
    // 0x428b0: addeq r3, r7, #36
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::new(3)?,R::new(7)?,36)?)?;
    // 0x428b4: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x428b8: ldr r3, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // 0x428bc: sub r6, r3, r9
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::new(3)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x428c0: ldreq r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R2,0)?)?;
    // 0x428c4: rsb r7, r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(7)?,R::new(6)?,0)?)?;
    // 0x428c8: and r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x428cc: orr r0, r0, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x428d0: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x428d4: streq r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R2,0)?)?;
    // 0x428d8: mov r2, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,52)?)?;
    // 0x428dc: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x428e0: str r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x428e4: add r2, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,16)?)?;
    // 0x428e8: str r4, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,12)?)?;
    // 0x428ec: mov r4, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,16777216)?)?;
    // 0x428f0: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,0)?)?;
    // 0x428f4: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x428f8: ldreq r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R2,0)?)?;
    // 0x428fc: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x42900: streq r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R2,0)?)?;
    // 0x42904: mov r2, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // 0x42908: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x4290c: str r2, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,16)?)?;
    // 0x42910: str r0, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,20)?)?;
    // 0x42914: add r0, r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,24)?)?;
    // 0x42918: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x4291c: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x42920: ldr r6, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,2744)?)?;
    // 0x42924: ldrb r3, [r5, #0x47d]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,1149)?)?;
    // 0x42928: ldr lr, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,16)?)?;
    // 0x4292c: ldr r7, [r6, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,168)?)?;
    // 0x42930: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x42934: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x42938: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x4293c: and r7, r4, r7, lsl #24
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(4)?,R::new(7)?,Shift::Lsl,24,false)?)?;
    // 0x42940: mov r4, #14680064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,14680064)?)?;
    // 0x42944: orr r3, r7, r3, lsl #26
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,26,false)?)?;
    // 0x42948: ldr r7, [r6, #0x3a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,932)?)?;
    // 0x4294c: and r7, r4, r7, lsl #21
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(4)?,R::new(7)?,Shift::Lsl,21,false)?)?;
    // 0x42950: ldr r4, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,644)?)?;
    // 0x42954: orr r7, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x42958: mov r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2048)?)?;
    // 0x4295c: cmp r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,4096)?)?;
    // 0x42960: ldrne r2, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(6)?,52)?)?;
    // 0x42964: and r2, r3, r2, lsl #11
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,11,false)?)?;
    // 0x42968: ldrb r3, [r6, #0x51c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,1308)?)?;
    // 0x4296c: orr r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42970: cmp r3, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,17)?)?;
    // 0x42974: bne 0x42980 <entry+0x2a80>
    c.branch(C::Ne,0x42980)?;
    // 0x42978: ldr r3, [r6, #0x524]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,1316)?)?;
    // 0x4297c: b 0x42a0c <entry+0x2b0c>
    c.branch(C::Al,0x42a0c)?;
    // 0x42980: ldr r7, [r8, #0x518]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(8)?,1304)?)?;
    // 0x42984: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x42988: beq 0x42a1c <entry+0x2b1c>
    c.branch(C::Eq,0x42a1c)?;
    // 0x4298c: cmp r4, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,8192)?)?;
    // 0x42990: bne 0x429d0 <entry+0x2ad0>
    c.branch(C::Ne,0x429d0)?;
    // 0x42994: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x42998: beq 0x42a04 <entry+0x2b04>
    c.branch(C::Eq,0x42a04)?;
    // 0x4299c: ldrb r3, [r8, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(8)?,380)?)?;
    // 0x429a0: cmp r3, #52
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,52)?)?;
    // 0x429a4: blo 0x429d0 <entry+0x2ad0>
    c.branch(C::Cc,0x429d0)?;
    // 0x429a8: ldrb r3, [r7, #0x33]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(7)?,51)?)?;
    // 0x429ac: b 0x42a0c <entry+0x2b0c>
    c.branch(C::Al,0x42a0c)?;
    // 0x429b0: ldrb r1, [r8, #0x3ed]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,1005)?)?;
    // 0x429b4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x429b8: beq 0x42d84 <entry+0x2e84>
    c.branch(C::Eq,0x42d84)?;
    // 0x429bc: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x429c0: bne 0x42d90 <entry+0x2e90>
    c.branch(C::Ne,0x42d90)?;
    // 0x429c4: bic r0, r0, #240
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,240)?)?;
    // 0x429c8: orr r0, r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,128)?)?;
    // 0x429cc: b 0x427e0 <entry+0x28e0>
    c.branch(C::Al,0x427e0)?;
    // 0x429d0: cmp r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,4096)?)?;
    // 0x429d4: bne 0x42a04 <entry+0x2b04>
    c.branch(C::Ne,0x42a04)?;
    // 0x429d8: ldrb r3, [r8, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(8)?,380)?)?;
    // 0x429dc: ldr r7, [pc, #0x794]
    c.literal(R::new(7)?,0x43178)?;
    // 0x429e0: mov r4, #1216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1216)?)?;
    // 0x429e4: ldrb r3, [r7, r3]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x429e8: add r7, r6, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(6)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x429ec: ldrsb r7, [r7, r4]
    c.emit(signed_byte_register(C::Al,R::new(7)?,R::new(7)?,R::new(4)?)?)?;
    // 0x429f0: add r4, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x429f4: mov r3, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,127)?)?;
    // 0x429f8: cmp r4, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,127)?)?;
    // 0x429fc: movle r3, r4
    c.emit(arm32::mov_register(C::Le,R::new(3)?,R::new(4)?))?;
    // 0x42a00: b 0x42a0c <entry+0x2b0c>
    c.branch(C::Al,0x42a0c)?;
    // 0x42a04: ldrb r3, [r8, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(8)?,380)?)?;
    // 0x42a08: ldrb r3, [r7, r3]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x42a0c: mov r7, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1016)?)?;
    // 0x42a10: orr r7, r7, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,1024)?)?;
    // 0x42a14: and r3, r7, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x42a18: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42a1c: ldr r3, [pc, #0x758]
    c.literal(R::new(3)?,0x4317c)?;
    // 0x42a20: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x42a24: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x42a28: ldrb r3, [r5, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,2462)?)?;
    // 0x42a2c: cmp r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,32)?)?;
    // 0x42a30: ldr r3, [r6, #0x33c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,828)?)?;
    // 0x42a34: moveq r7, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x42a38: orr r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42a3c: mov r7, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,131072)?)?;
    // 0x42a40: and r3, r7, r3, lsl #17
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,17,false)?)?;
    // 0x42a44: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x42a48: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x42a4c: ldr r3, [r6, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,92)?)?;
    // 0x42a50: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x42a54: moveq r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x42a58: cmp lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,1)?)?;
    // 0x42a5c: orr r2, r2, r7, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,12,false)?)?;
    // 0x42a60: bne 0x42a74 <entry+0x2b74>
    c.branch(C::Ne,0x42a74)?;
    // 0x42a64: ldr r3, [r8, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,420)?)?;
    // 0x42a68: ldr r3, [r3, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,168)?)?;
    // 0x42a6c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x42a70: orrne r2, r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R2,R::R2,1)?)?;
    // 0x42a74: ldrb r3, [r5, #0xb78]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,2936)?)?;
    // 0x42a78: ldrb r7, [r5, #0x495]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,1173)?)?;
    // 0x42a7c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x42a80: ldrb r3, [r5, #0x497]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,1175)?)?;
    // 0x42a84: and r6, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(7)?,1)?)?;
    // 0x42a88: orrne r2, r2, #8192
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R2,R::R2,8192)?)?;
    // 0x42a8c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x42a90: bic r2, r2, #167772160
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,167772160)?)?;
    // 0x42a94: orr r2, r2, r6, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(6)?,Shift::Lsl,25,false)?)?;
    // 0x42a98: and r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x42a9c: beq 0x42ae4 <entry+0x2be4>
    c.branch(C::Eq,0x42ae4)?;
    // 0x42aa0: ldr r7, [r5, #0x49c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,1180)?)?;
    // 0x42aa4: ldr r6, [pc, #0x6d4]
    c.literal(R::new(6)?,0x43180)?;
    // 0x42aa8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x42aac: mov r9, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,7)?)?;
    // 0x42ab0: and r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x42ab4: ldrb r6, [r5, #0x498]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(5)?,1176)?)?;
    // 0x42ab8: orr r4, r7, r6, lsl #17
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(7)?,R::new(6)?,Shift::Lsl,17,false)?)?;
    // 0x42abc: ldreq r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R0,0)?)?;
    // 0x42ac0: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x42ac4: streq r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R0,0)?)?;
    // 0x42ac8: mov r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,88)?)?;
    // 0x42acc: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x42ad0: str r0, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,24)?)?;
    // 0x42ad4: add r0, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,32)?)?;
    // 0x42ad8: str r4, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,28)?)?;
    // 0x42adc: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x42ae0: b 0x42ae8 <entry+0x2be8>
    c.branch(C::Al,0x42ae8)?;
    // 0x42ae4: mov r9, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,6)?)?;
    // 0x42ae8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x42aec: orr r1, r2, r3, lsl #27
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::new(3)?,Shift::Lsl,27,false)?)?;
    // 0x42af0: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x42af4: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x42af8: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x42afc: mov r2, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1040)?)?;
    // 0x42b00: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x42b04: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x42b08: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x42b0c: add r1, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,8)?)?;
    // 0x42b10: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x42b14: ldr r2, [r5, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,1244)?)?;
    // 0x42b18: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x42b1c: beq 0x42b44 <entry+0x2c44>
    c.branch(C::Eq,0x42b44)?;
    // 0x42b20: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x42b24: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x42b28: bne 0x42b4c <entry+0x2c4c>
    c.branch(C::Ne,0x42b4c)?;
    // 0x42b2c: ldr r2, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2744)?)?;
    // 0x42b30: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // 0x42b34: ldrb r2, [r2, #0xe8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,232)?)?;
    // 0x42b38: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // 0x42b3c: moveq r3, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x42b40: b 0x42b4c <entry+0x2c4c>
    c.branch(C::Al,0x42b4c)?;
    // 0x42b44: ldr r2, [r8, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,480)?)?;
    // 0x42b48: lsl r3, r2, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsl,16)?)?;
    // 0x42b4c: ldr r2, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,644)?)?;
    // 0x42b50: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x42b54: bge 0x42b6c <entry+0x2c6c>
    c.branch(C::Ge,0x42b6c)?;
    // 0x42b58: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x42b5c: beq 0x42b9c <entry+0x2c9c>
    c.branch(C::Eq,0x42b9c)?;
    // 0x42b60: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x42b64: beq 0x42b94 <entry+0x2c94>
    c.branch(C::Eq,0x42b94)?;
    // 0x42b68: b 0x42bb8 <entry+0x2cb8>
    c.branch(C::Al,0x42bb8)?;
    // 0x42b6c: cmp r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16384)?)?;
    // 0x42b70: beq 0x42b94 <entry+0x2c94>
    c.branch(C::Eq,0x42b94)?;
    // 0x42b74: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x42b78: bne 0x42bb8 <entry+0x2cb8>
    c.branch(C::Ne,0x42bb8)?;
    // 0x42b7c: ldr r2, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2744)?)?;
    // 0x42b80: ldr r7, [r2, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,56)?)?;
    // 0x42b84: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x42b88: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x42b8c: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x42b90: b 0x42bac <entry+0x2cac>
    c.branch(C::Al,0x42bac)?;
    // 0x42b94: ldrb r2, [r5, #0x339]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,825)?)?;
    // 0x42b98: b 0x42bac <entry+0x2cac>
    c.branch(C::Al,0x42bac)?;
    // 0x42b9c: ldrb r7, [r5, #0x544]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,1348)?)?;
    // 0x42ba0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x42ba4: cmp r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // 0x42ba8: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x42bac: mov r7, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,32768)?)?;
    // 0x42bb0: and r2, r7, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(7)?,R::R2,Shift::Lsl,15,false)?)?;
    // 0x42bb4: orr r3, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x42bb8: ldr r2, [pc, #0x5c4]
    c.literal(R::R2,0x43184)?;
    // 0x42bbc: mov r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8192)?)?;
    // 0x42bc0: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x42bc4: and r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42bc8: ldrb r3, [r8, #0x3ea]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(8)?,1002)?)?;
    // 0x42bcc: and r3, r7, r3, lsl #13
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,13,false)?)?;
    // 0x42bd0: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42bd4: ldrb r3, [r8, #0x3e9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(8)?,1001)?)?;
    // 0x42bd8: and r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x42bdc: orr r2, r2, r3, lsl #14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,14,false)?)?;
    // 0x42be0: ldrb r3, [r8, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(8)?,380)?)?;
    // 0x42be4: orr r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // 0x42be8: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x42bec: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x42bf0: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x42bf4: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x42bf8: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x42bfc: str r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x42c00: add r1, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,16)?)?;
    // 0x42c04: str r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x42c08: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x42c0c: ldr r2, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2744)?)?;
    // 0x42c10: ldrb r7, [r8, #0x3e7]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(8)?,999)?)?;
    // 0x42c14: ldr r6, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,644)?)?;
    // 0x42c18: ldrb r3, [r8, #0x3e6]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(8)?,998)?)?;
    // 0x42c1c: ldrb r2, [r2, #0xd8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,216)?)?;
    // 0x42c20: and r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x42c24: cmp r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,8192)?)?;
    // 0x42c28: and r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x42c2c: lsl r4, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R2,Shift::Lsl,4)?)?;
    // 0x42c30: orreq r4, r7, r2, lsl #4
    c.emit(arm32::data_register(C::Eq,Op::Orr,R::new(4)?,R::new(7)?,R::R2,Shift::Lsl,4,false)?)?;
    // 0x42c34: ldrb r7, [r5, #0xb7a]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,2938)?)?;
    // 0x42c38: and r2, r4, #241
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(4)?,241)?)?;
    // 0x42c3c: orreq r2, r2, r3, lsl #8
    c.emit(arm32::data_register(C::Eq,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x42c40: orr r3, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R2,2)?)?;
    // 0x42c44: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x42c48: orrne r2, r3, #57344
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R2,R::new(3)?,57344)?)?;
    // 0x42c4c: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x42c50: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x42c54: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x42c58: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x42c5c: ldr r1, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,0)?)?;
    // 0x42c60: rsb r6, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(6)?,R::R1,0)?)?;
    // 0x42c64: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // 0x42c68: rsb r3, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(3)?,R::R1,0)?)?;
    // 0x42c6c: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x42c70: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x42c74: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x42c78: add r1, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,24)?)?;
    // 0x42c7c: str r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,20)?)?;
    // 0x42c80: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x42c84: ldr r2, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,644)?)?;
    // 0x42c88: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x42c8c: bne 0x42de0 <entry+0x2ee0>
    c.branch(C::Ne,0x42de0)?;
    // 0x42c90: ldr r2, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2744)?)?;
    // 0x42c94: ldrb r2, [r2, #0x4e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,1252)?)?;
    // 0x42c98: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x42c9c: bne 0x42cb4 <entry+0x2db4>
    c.branch(C::Ne,0x42cb4)?;
    // 0x42ca0: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // 0x42ca4: bne 0x42ccc <entry+0x2dcc>
    c.branch(C::Ne,0x42ccc)?;
    // 0x42ca8: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x42cac: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x42cb0: b 0x42cdc <entry+0x2ddc>
    c.branch(C::Al,0x42cdc)?;
    // 0x42cb4: ldr r2, [r5, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,908)?)?;
    // 0x42cb8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x42cbc: bne 0x42ccc <entry+0x2dcc>
    c.branch(C::Ne,0x42ccc)?;
    // 0x42cc0: ldr r2, [r5, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,1236)?)?;
    // 0x42cc4: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x42cc8: bne 0x42d1c <entry+0x2e1c>
    c.branch(C::Ne,0x42d1c)?;
    // 0x42ccc: ldrb r2, [r8, #0x51e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(8)?,1310)?)?;
    // 0x42cd0: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x42cd4: and r7, r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R2,3)?)?;
    // 0x42cd8: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x42cdc: orr r7, r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,16)?)?;
    // 0x42ce0: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x42ce4: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // 0x42ce8: and r7, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,255)?)?;
    // 0x42cec: orr r2, r2, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x42cf0: ldreq r7, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R1,0)?)?;
    // 0x42cf4: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x42cf8: streq r7, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R1,0)?)?;
    // 0x42cfc: mov r1, #5248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5248)?)?;
    // 0x42d00: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x42d04: add r1, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,32)?)?;
    // 0x42d08: str r2, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,28)?)?;
    // 0x42d0c: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x42d10: ldr r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,644)?)?;
    // 0x42d14: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x42d18: bne 0x42de0 <entry+0x2ee0>
    c.branch(C::Ne,0x42de0)?;
    // 0x42d1c: ldrb r2, [r8, #0x342]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(8)?,834)?)?;
    // 0x42d20: ldrb r0, [r5, #0x328]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,808)?)?;
    // 0x42d24: and r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,1)?)?;
    // 0x42d28: lsl r2, r2, #30
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,30)?)?;
    // 0x42d2c: orr r0, r2, r0, lsl #31
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,31,false)?)?;
    // 0x42d30: ldr r2, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2744)?)?;
    // 0x42d34: ldr r7, [r2, #0x4c8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,1224)?)?;
    // 0x42d38: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x42d3c: beq 0x42d98 <entry+0x2e98>
    c.branch(C::Eq,0x42d98)?;
    // 0x42d40: add lr, r2, #1232
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R2,1232)?)?;
    // 0x42d44: ldr r7, [r2, #0x4cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,1228)?)?;
    // 0x42d48: mov r2, #4032
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4032)?)?;
    // 0x42d4c: ldm lr, {r4, r12, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::LR,R::new(4)?.bit() | R::R12.bit() | R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x42d50: and r2, r2, r4, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(4)?,Shift::Lsl,6,false)?)?;
    // 0x42d54: and r4, r7, #63
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(7)?,63)?)?;
    // 0x42d58: orr r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x42d5c: orr r0, r0, r4, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,24,false)?)?;
    // 0x42d60: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42d64: mov r2, #258048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,258048)?)?;
    // 0x42d68: and r2, r2, r12, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R12,Shift::Lsl,12,false)?)?;
    // 0x42d6c: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42d70: mov r2, #16515072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16515072)?)?;
    // 0x42d74: and r2, r2, lr, lsl #18
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::LR,Shift::Lsl,18,false)?)?;
    // 0x42d78: ldr lr, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,16)?)?;
    // 0x42d7c: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42d80: b 0x42db4 <entry+0x2eb4>
    c.branch(C::Al,0x42db4)?;
    // 0x42d84: bic r0, r0, #240
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,240)?)?;
    // 0x42d88: orr r0, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,64)?)?;
    // 0x42d8c: b 0x427e0 <entry+0x28e0>
    c.branch(C::Al,0x427e0)?;
    // 0x42d90: bic r0, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,48)?)?;
    // 0x42d94: b 0x427e0 <entry+0x28e0>
    c.branch(C::Al,0x427e0)?;
    // 0x42d98: ldrb r2, [r8, #0x33f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(8)?,831)?)?;
    // 0x42d9c: and r2, r2, #63
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,63)?)?;
    // 0x42da0: orr r7, r2, r2, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R2,R::R2,Shift::Lsl,24,false)?)?;
    // 0x42da4: orr r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x42da8: orr r0, r0, r2, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,6,false)?)?;
    // 0x42dac: orr r0, r0, r2, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,12,false)?)?;
    // 0x42db0: orr r0, r0, r2, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,18,false)?)?;
    // 0x42db4: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x42db8: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // 0x42dbc: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x42dc0: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x42dc4: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x42dc8: mov r2, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // 0x42dcc: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x42dd0: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x42dd4: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x42dd8: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x42ddc: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x42de0: mov r7, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16711680)?)?;
    // 0x42de4: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x42de8: and r3, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,255)?)?;
    // 0x42dec: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x42df0: and r6, r7, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(7)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x42df4: orr r3, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x42df8: ldreq r6, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R1,0)?)?;
    // 0x42dfc: rsb r2, r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R2,R::R0,0)?)?;
    // 0x42e00: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x42e04: and r2, r7, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(7)?,R::R2,Shift::Lsl,16,false)?)?;
    // 0x42e08: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x42e0c: streq r6, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R1,0)?)?;
    // 0x42e10: mov r6, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,44)?)?;
    // 0x42e14: orr r6, r6, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,5120)?)?;
    // 0x42e18: rsb r0, r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,0)?)?;
    // 0x42e1c: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x42e20: str r3, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x42e24: add r3, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,8)?)?;
    // 0x42e28: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // 0x42e2c: str r3, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,0)?)?;
    // 0x42e30: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x42e34: orr r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x42e38: ldreq r2, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::new(3)?,0)?)?;
    // 0x42e3c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x42e40: streq r2, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(3)?,0)?)?;
    // 0x42e44: mov r2, #1072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1072)?)?;
    // 0x42e48: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x42e4c: str r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    // 0x42e50: str r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // 0x42e54: add r0, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,16)?)?;
    // 0x42e58: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x42e5c: ldr r1, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,644)?)?;
    // 0x42e60: str r9, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,4)?)?;
    // 0x42e64: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x42e68: bne 0x42e98 <entry+0x2f98>
    c.branch(C::Ne,0x42e98)?;
    // 0x42e6c: ldr r1, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2744)?)?;
    // 0x42e70: ldrb r2, [r1, #0x51c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,1308)?)?;
    // 0x42e74: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // 0x42e78: bne 0x42f14 <entry+0x3014>
    c.branch(C::Ne,0x42f14)?;
    // 0x42e7c: ldr r6, [r1, #0x524]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,1316)?)?;
    // 0x42e80: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x42e84: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x42e88: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x42e8c: mov lr, r6
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(6)?))?;
    // 0x42e90: mov r12, r6
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(6)?))?;
    // 0x42e94: b 0x42f78 <entry+0x3078>
    c.branch(C::Al,0x42f78)?;
    // 0x42e98: mvn r6, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(6)?,R::R0,2147483648)?)?;
    // 0x42e9c: cmp lr, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,2)?)?;
    // 0x42ea0: bne 0x42f64 <entry+0x3064>
    c.branch(C::Ne,0x42f64)?;
    // 0x42ea4: ldr r0, [r8, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,420)?)?;
    // 0x42ea8: ldr r1, [r8, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,640)?)?;
    // 0x42eac: ldr r2, [r8, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,644)?)?;
    // 0x42eb0: ldr r4, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,16)?)?;
    // 0x42eb4: ldr r3, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,20)?)?;
    // 0x42eb8: subs r0, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,true)?)?;
    // 0x42ebc: sbc r1, r4, r2
    c.emit(arm32::data_register(C::Al,Op::Sbc,R::R1,R::new(4)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x42ec0: ldr r2, [r5, #0x4d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,1240)?)?;
    // 0x42ec4: bl 0x35520 <entry+0xffffffffffff5620>
    c.call(0x35520)?;
    // 0x42ec8: add r3, r8, #640
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(8)?,640)?)?;
    // 0x42ecc: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x42ed0: ldm r3, {r0, r1, r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(3)?,R::R0.bit() | R::R1.bit() | R::R2.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x42ed4: subs r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R2,R::R0,Shift::Lsl,0,true)?)?;
    // 0x42ed8: ldr r2, [r5, #0x4d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,1240)?)?;
    // 0x42edc: sbc r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Sbc,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x42ee0: bl 0x35520 <entry+0xffffffffffff5620>
    c.call(0x35520)?;
    // 0x42ee4: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x42ee8: lsl r0, r9, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(9)?,Shift::Lsl,31)?)?;
    // 0x42eec: lsr r1, r9, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(9)?,Shift::Lsr,1)?)?;
    // 0x42ef0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x42ef4: bl 0x8187c <entry+0x4197c>
    c.call(0x8187c)?;
    // 0x42ef8: add r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1)?)?;
    // 0x42efc: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x42f00: mvn r2, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,2147483648)?)?;
    // 0x42f04: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x42f08: mov lr, r1
    c.emit(arm32::mov_register(C::Al,R::LR,R::R1))?;
    // 0x42f0c: mov r12, r1
    c.emit(arm32::mov_register(C::Al,R::R12,R::R1))?;
    // 0x42f10: b 0x42f78 <entry+0x3078>
    c.branch(C::Al,0x42f78)?;
    // 0x42f14: mov r1, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,254)?)?;
    // 0x42f18: mov r2, #246
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,246)?)?;
    // 0x42f1c: mov r6, #1012
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1012)?)?;
    // 0x42f20: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x42f24: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x42f28: orr r2, r2, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,768)?)?;
    // 0x42f2c: ldrh r6, [r8, r6]
    c.emit(load_halfword_register(C::Al,R::new(6)?,R::new(8)?,R::new(6)?)?)?;
    // 0x42f30: orr r4, r4, #7936
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,7936)?)?;
    // 0x42f34: ldrsh r12, [r8, r1]
    c.emit(load_signed_halfword_register(C::Al,R::R12,R::new(8)?,R::R1)?)?;
    // 0x42f38: mov r1, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1020)?)?;
    // 0x42f3c: ldrh r2, [r8, r2]
    c.emit(load_halfword_register(C::Al,R::R2,R::new(8)?,R::R2)?)?;
    // 0x42f40: ldrsh lr, [r8, r1]
    c.emit(load_signed_halfword_register(C::Al,R::LR,R::new(8)?,R::R1)?)?;
    // 0x42f44: mov r1, #250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,250)?)?;
    // 0x42f48: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x42f4c: ldrsh r3, [r8, r1]
    c.emit(load_signed_halfword_register(C::Al,R::new(3)?,R::new(8)?,R::R1)?)?;
    // 0x42f50: mov r1, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1016)?)?;
    // 0x42f54: and r6, r6, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x42f58: ldrsh r1, [r8, r1]
    c.emit(load_signed_halfword_register(C::Al,R::R1,R::new(8)?,R::R1)?)?;
    // 0x42f5c: and r2, r2, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x42f60: b 0x42f78 <entry+0x3078>
    c.branch(C::Al,0x42f78)?;
    // 0x42f64: mvn r2, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,2147483648)?)?;
    // 0x42f68: mvn r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,2147483648)?)?;
    // 0x42f6c: mvn r3, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,2147483648)?)?;
    // 0x42f70: mvn lr, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::LR,R::R0,2147483648)?)?;
    // 0x42f74: mvn r12, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R12,R::R0,2147483648)?)?;
    // 0x42f78: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x42f7c: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x42f80: ldreq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::R0,0)?)?;
    // 0x42f84: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x42f88: streq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::R0,0)?)?;
    // 0x42f8c: mov r4, #5184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5184)?)?;
    // 0x42f90: stm r0, {r4, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::new(4)?.bit() | R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x42f94: add r6, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,8)?)?;
    // 0x42f98: str r6, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,0)?)?;
    // 0x42f9c: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x42fa0: ldreq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(6)?,0)?)?;
    // 0x42fa4: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x42fa8: streq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(6)?,0)?)?;
    // 0x42fac: mov r4, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,68)?)?;
    // 0x42fb0: orr r4, r4, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,5120)?)?;
    // 0x42fb4: str r4, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,8)?)?;
    // 0x42fb8: str r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x42fbc: add r2, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,16)?)?;
    // 0x42fc0: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,0)?)?;
    // 0x42fc4: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x42fc8: ldreq r4, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::R2,0)?)?;
    // 0x42fcc: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x42fd0: streq r4, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::R2,0)?)?;
    // 0x42fd4: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // 0x42fd8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x42fdc: str r2, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,16)?)?;
    // 0x42fe0: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,20)?)?;
    // 0x42fe4: add r1, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,24)?)?;
    // 0x42fe8: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x42fec: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x42ff0: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x42ff4: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x42ff8: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x42ffc: mov r1, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,76)?)?;
    // 0x43000: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x43004: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x43008: add r1, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,32)?)?;
    // 0x4300c: str r3, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,28)?)?;
    // 0x43010: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x43014: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x43018: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x4301c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x43020: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x43024: mov r1, #1104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1104)?)?;
    // 0x43028: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x4302c: str r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,32)?)?;
    // 0x43030: add r1, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,40)?)?;
    // 0x43034: str lr, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R0,36)?)?;
    // 0x43038: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x4303c: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x43040: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x43044: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x43048: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x4304c: mov r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // 0x43050: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x43054: str r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,40)?)?;
    // 0x43058: add r1, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,48)?)?;
    // 0x4305c: str r12, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,44)?)?;
    // 0x43060: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x43064: ldr r2, [r5, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,744)?)?;
    // 0x43068: ldr r3, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,400)?)?;
    // 0x4306c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x43070: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x43074: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x43078: lsl r6, r2, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::R2,Shift::Lsl,5)?)?;
    // 0x4307c: orrne r6, r7, r2, lsl #5
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::new(6)?,R::new(7)?,R::R2,Shift::Lsl,5,false)?)?;
    // 0x43080: ldr r2, [r5, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2744)?)?;
    // 0x43084: mov r7, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,64)?)?;
    // 0x43088: ldrb r4, [r2, #0xcc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R2,204)?)?;
    // 0x4308c: and r7, r7, r4, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(4)?,Shift::Lsl,6,false)?)?;
    // 0x43090: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x43094: orr r7, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x43098: orr r6, r7, #20
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(7)?,20)?)?;
    // 0x4309c: beq 0x430bc <entry+0x31bc>
    c.branch(C::Eq,0x430bc)?;
    // 0x430a0: ldr r5, [r8, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(8)?,420)?)?;
    // 0x430a4: ldr r5, [r5, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,168)?)?;
    // 0x430a8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x430ac: ldr r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,4)?)?;
    // 0x430b0: orrne r6, r7, #22
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(6)?,R::new(7)?,22)?)?;
    // 0x430b4: orr r6, r6, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,8)?)?;
    // 0x430b8: b 0x430cc <entry+0x31cc>
    c.branch(C::Al,0x430cc)?;
    // 0x430bc: ldr r7, [r5, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,1244)?)?;
    // 0x430c0: ldr r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,4)?)?;
    // 0x430c4: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x430c8: orreq r6, r6, #8
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(6)?,R::new(6)?,8)?)?;
    // 0x430cc: bic r7, r6, #896
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(6)?,896)?)?;
    // 0x430d0: ldrb r6, [r2, #0xe8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R2,232)?)?;
    // 0x430d4: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x430d8: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x430dc: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x430e0: orr r7, r7, r6, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,7,false)?)?;
    // 0x430e4: beq 0x43110 <entry+0x3210>
    c.branch(C::Eq,0x43110)?;
    // 0x430e8: ldrb r6, [r2, #0x49a]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R2,1178)?)?;
    // 0x430ec: bic r7, r7, #6912
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(7)?,6912)?)?;
    // 0x430f0: lsl r3, r6, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(6)?,Shift::Lsl,24)?)?;
    // 0x430f4: asr r3, r3, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Asr,24)?)?;
    // 0x430f8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x430fc: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x43100: orrgt r3, r3, #2048
    c.emit(arm32::data_immediate(C::Gt,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x43104: cmp r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,3)?)?;
    // 0x43108: orreq r3, r7, #4096
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(3)?,R::new(7)?,4096)?)?;
    // 0x4310c: b 0x43114 <entry+0x3214>
    c.branch(C::Al,0x43114)?;
    // 0x43110: bic r3, r7, #6912
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(7)?,6912)?)?;
    // 0x43114: mov r7, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,153)?)?;
    // 0x43118: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x4311c: bic r3, r3, #58368
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,58368)?)?;
    // 0x43120: orr r7, r7, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,1024)?)?;
    // 0x43124: ldrsb r7, [r2, r7]
    c.emit(signed_byte_register(C::Al,R::new(7)?,R::R2,R::new(7)?)?)?;
    // 0x43128: ldrb r2, [r2, #0x49b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,1179)?)?;
    // 0x4312c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x43130: and r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,1)?)?;
    // 0x43134: movgt r6, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x43138: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4313c: orr r3, r3, r6, lsl #13
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,13,false)?)?;
    // 0x43140: orr r2, r3, r2, lsl #10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,10,false)?)?;
    // 0x43144: ldreq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R1,0)?)?;
    // 0x43148: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x4314c: streq r3, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R1,0)?)?;
    // 0x43150: mov r1, #1120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1120)?)?;
    // 0x43154: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x43158: str r1, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,48)?)?;
    // 0x4315c: str r2, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,52)?)?;
    // 0x43160: add r0, r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,56)?)?;
    // 0x43164: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x43168: add r0, r5, #9
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,9)?)?;
    // 0x4316c: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x43170: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(CODE_END)?;
    for value in LITERAL_WORDS {c.emit(value)?;}
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();write_codec_mode_packets(&mut c)?;Ok(c.bytes)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test] fn code_and_literal_pool_have_exact_coverage(){
        let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
        assert_eq!(CODE_END-START,658*4);
        for (i,value) in LITERAL_WORDS.iter().enumerate(){
            let at=CODE_END-START+i*4;
            assert_eq!(u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()),*value);
        }
    }
}
