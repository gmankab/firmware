// SPDX-License-Identifier: GPL-3.0-only
//! Select the closest controller record using sampled occupancy differences,
//! scale the selected 64-bit index, and update the modulo-five descriptor ring.
//! The function performs a ten-channel comparison over a nine-entry search
//! table, checks unsigned carry across 64-bit accumulations and emits five
//! separate 32-bit diagnostic/literal words after its return.
use arm32::{branch, data_immediate, data_immediate_with_flags, data_register,
    mov_register, mov_shifted_register, multiply_accumulate,
    push_registers, transfer_byte,
    transfer_byte_register, transfer_multiple,
    transfer_word, transfer_word_postindexed,
    transfer_word_register_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START:usize=0x71978;
pub const END:usize=0x71e14;
pub const FILE_BASE:usize=0x4000;
const LOG_RECORD:u32=0x000f_2e4c;
const DIVIDE_BY_FIVE:u32=0xcccc_cccd;
const LOG_PROFILE:u32=0x000f_31e7;
const LOG_FOOTER:u32=0x000e_2788;
const LOG_RESULT:u32=0x000f_3235;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at==addr {Ok(())}else{Err(format!("codec record selector 0x{:x} != 0x{addr:x}",self.at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START||self.at>=END||self.at%4!=0{return Err(format!("invalid selector A32 address 0x{:x}",self.at));}
        self.bytes.extend(word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize,link:bool)->Result<(),String>{
        self.emit(branch(cond,self.at as u32,target as u32,link)?)
    }
    fn literal(&mut self,dst:R,target:usize)->Result<(),String>{
        self.emit(transfer_word(C::Al,true,dst,R::PC,target as i32-self.at as i32-8)?)
    }
}
fn r(n:u8)->R{R::new(n).expect("constant register number in 0..15")}
fn umull(c:&mut Code,low:R,high:R,left:R,right:R)->Result<(),String>{
    if low==high||[low,high,left,right].iter().any(|r|r.number()>=13) {return Err("invalid A32 UMULL register operands".into());}
    c.emit(0xe080_0090 | (high.number()<<16)|(low.number()<<12)|(right.number()<<8)|left.number())
}

fn select_closest_record(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x71978: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(push_registers(C::Al,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::LR.bit())?)?;
    // 0x7197c: add r11, sp, #28
    c.emit(data_immediate(C::Al,Op::Add,r(11),R::SP,28)?)?;
    // 0x71980: sub sp, sp, #76
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,R::SP,76)?)?;
    // 0x71984: mov r5, r0
    c.emit(mov_register(C::Al,r(5),r(0)))?;
    // 0x71988: mov r6, r1
    c.emit(mov_register(C::Al,r(6),r(1)))?;
    // 0x7198c: cmp r5, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),0)?)?;
    // 0x71990: beq 0x719a4 <entry+0x3b8>   @ imm = #0xc
    c.branch(C::Eq,0x719a4,false)?;
    // 0x71994: ldr r0, [r5, #0x168]
    c.emit(transfer_word(C::Al,true,r(0),r(5),360)?)?;
    // 0x71998: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x7199c: movne r0, #1
    c.emit(data_immediate(C::Ne,Op::Mov,r(0),R::R0,1)?)?;
    // 0x719a0: b 0x719a8 <entry+0x3bc>   @ imm = #0x0
    c.branch(C::Al,0x719a8,false)?;
    // 0x719a4: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x719a8: ldr r2, [pc, #0x450]        @ 0x814 <entry+0x814>
    c.literal(r(2),0x71e00)?;
    // 0x719ac: mov r1, #736
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,736)?)?;
    // 0x719b0: orr r1, r1, #8192
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),8192)?)?;
    // 0x719b4: bl 0x17b0 <entry+0xfffffffffff901c4> @ imm = #-0x7020c
    c.branch(C::Al,0x17b0,true)?;
    // 0x719b8: ldr r1, [r6, #0x20]
    c.emit(transfer_word(C::Al,true,r(1),r(6),32)?)?;
    // 0x719bc: ldr r12, [r5, #0x164]
    c.emit(transfer_word(C::Al,true,r(12),r(5),356)?)?;
    // 0x719c0: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x719c4: beq 0x71a14 <entry+0x428>   @ imm = #0x48
    c.branch(C::Eq,0x71a14,false)?;
    // 0x719c8: cmp r12, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(12),0)?)?;
    // 0x719cc: beq 0x71a34 <entry+0x448>   @ imm = #0x60
    c.branch(C::Eq,0x71a34,false)?;
    // 0x719d0: ldr r2, [pc, #0x42c]        @ 0x818 <entry+0x818>
    c.literal(r(2),0x71e04)?;
    // 0x719d4: subs r1, r12, #4
    c.emit(data_immediate_with_flags(C::Al,Op::Sub,r(1),r(12),4,true)?)?;
    // 0x719d8: mov r9, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(9),R::R0,0)?)?;
    // 0x719dc: movls r1, r9
    c.emit(mov_register(C::Ls,r(1),r(9)))?;
    // 0x719e0: cmp r1, r12
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(1),r(12),Shift::Lsl,0,false)?)?;
    // 0x719e4: umull r3, r7, r12, r2
    umull(c,r(3),r(7),r(12),r(2))?;
    // 0x719e8: lsr r2, r7, #2
    c.emit(mov_shifted_register(C::Al,r(2),r(7),Shift::Lsr,2)?)?;
    // 0x719ec: add r2, r2, r2, lsl #2
    c.emit(data_register(C::Al,Op::Add,r(2),r(2),r(2),Shift::Lsl,2,false)?)?;
    // 0x719f0: sub r2, r12, r2
    c.emit(data_register(C::Al,Op::Sub,r(2),r(12),r(2),Shift::Lsl,0,false)?)?;
    // 0x719f4: add r2, r5, r2, lsl #6
    c.emit(data_register(C::Al,Op::Add,r(2),r(5),r(2),Shift::Lsl,6,false)?)?;
    // 0x719f8: ldrb r8, [r2, #0x39]
    c.emit(transfer_byte(C::Al,true,r(8),r(2),57)?)?;
    // 0x719fc: str r5, [sp, #0x2c]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,44)?)?;
    // 0x71a00: bls 0x71a40 <entry+0x454>   @ imm = #0x38
    c.branch(C::Ls,0x71a40,false)?;
    // 0x71a04: str r0, [r11, #-0x2c]
    c.emit(transfer_word(C::Al,false,r(0),r(11),-44)?)?;
    // 0x71a08: str r0, [sp, #0x30]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,48)?)?;
    // 0x71a0c: str r0, [sp, #0x34]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,52)?)?;
    // 0x71a10: b 0x71c04 <entry+0x618>   @ imm = #0x1ec
    c.branch(C::Al,0x71c04,false)?;
    // 0x71a14: ldr r1, [pc, #0x3e8]        @ 0x818 <entry+0x818>
    c.literal(r(1),0x71e04)?;
    // 0x71a18: umull r2, r3, r12, r1
    umull(c,r(2),r(3),r(12),r(1))?;
    // 0x71a1c: lsr r1, r3, #2
    c.emit(mov_shifted_register(C::Al,r(1),r(3),Shift::Lsr,2)?)?;
    // 0x71a20: add r1, r1, r1, lsl #2
    c.emit(data_register(C::Al,Op::Add,r(1),r(1),r(1),Shift::Lsl,2,false)?)?;
    // 0x71a24: sub r1, r12, r1
    c.emit(data_register(C::Al,Op::Sub,r(1),r(12),r(1),Shift::Lsl,0,false)?)?;
    // 0x71a28: add r1, r5, r1, lsl #6
    c.emit(data_register(C::Al,Op::Add,r(1),r(5),r(1),Shift::Lsl,6,false)?)?;
    // 0x71a2c: ldrb r1, [r1, #0x39]
    c.emit(transfer_byte(C::Al,true,r(1),r(1),57)?)?;
    // 0x71a30: b 0x71dd0 <entry+0x7e4>   @ imm = #0x398
    c.branch(C::Al,0x71dd0,false)?;
    // 0x71a34: mov r12, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(12),R::R0,0)?)?;
    // 0x71a38: mov r9, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(9),R::R0,0)?)?;
    // 0x71a3c: b 0x71dac <entry+0x7c0>   @ imm = #0x368
    c.branch(C::Al,0x71dac,false)?;
    // 0x71a40: mov r2, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,0)?)?;
    // 0x71a44: str r8, [sp, #0x28]
    c.emit(transfer_word(C::Al,false,r(8),R::SP,40)?)?;
    // 0x71a48: mov r10, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(10),R::R0,0)?)?;
    // 0x71a4c: mov r4, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(4),R::R0,0)?)?;
    // 0x71a50: mov r6, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,0)?)?;
    // 0x71a54: mov r8, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(8),R::R0,0)?)?;
    // 0x71a58: str r12, [sp, #0x30]
    c.emit(transfer_word(C::Al,false,r(12),R::SP,48)?)?;
    // 0x71a5c: str r2, [r11, #-0x24]
    c.emit(transfer_word(C::Al,false,r(2),r(11),-36)?)?;
    // 0x71a60: mov r2, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,0)?)?;
    // 0x71a64: str r2, [r11, #-0x20]
    c.emit(transfer_word(C::Al,false,r(2),r(11),-32)?)?;
    // 0x71a68: mov r2, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,0)?)?;
    // 0x71a6c: str r2, [r11, #-0x28]
    c.emit(transfer_word(C::Al,false,r(2),r(11),-40)?)?;
    // 0x71a70: mov r2, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,0)?)?;
    // 0x71a74: str r10, [r11, #-0x2c]
    c.emit(transfer_word(C::Al,false,r(10),r(11),-44)?)?;
    // 0x71a78: mov r12, r5
    c.emit(mov_register(C::Al,r(12),r(5)))?;
    // 0x71a7c: add r2, r2, #1
    c.emit(data_immediate(C::Al,Op::Add,r(2),r(2),1)?)?;
    // 0x71a80: ldr r7, [pc, #0x37c]        @ 0x818 <entry+0x818>
    c.literal(r(7),0x71e04)?;
    // 0x71a84: umull r3, r0, r1, r7
    umull(c,r(3),r(0),r(1),r(7))?;
    // 0x71a88: lsr r0, r0, #2
    c.emit(mov_shifted_register(C::Al,r(0),r(0),Shift::Lsr,2)?)?;
    // 0x71a8c: add r0, r0, r0, lsl #2
    c.emit(data_register(C::Al,Op::Add,r(0),r(0),r(0),Shift::Lsl,2,false)?)?;
    // 0x71a90: sub r0, r1, r0
    c.emit(data_register(C::Al,Op::Sub,r(0),r(1),r(0),Shift::Lsl,0,false)?)?;
    // 0x71a94: add r1, r1, #1
    c.emit(data_immediate(C::Al,Op::Add,r(1),r(1),1)?)?;
    // 0x71a98: ldr r0, [r12, r0, lsl #6]!
    c.emit(transfer_word_register_preindexed_writeback(C::Al,true,r(0),r(12),r(0),Shift::Lsl,6)?)?;
    // 0x71a9c: ldmib r12, {r3, r7}
    c.emit(transfer_multiple(C::Al,true,r(12),r(3).bit()|r(7).bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x71aa0: adds r8, r0, r8
    c.emit(data_register(C::Al,Op::Add,r(8),r(0),r(8),Shift::Lsl,0,true)?)?;
    // 0x71aa4: ldr r0, [r11, #-0x28]
    c.emit(transfer_word(C::Al,true,r(0),r(11),-40)?)?;
    // 0x71aa8: str r7, [sp, #0x34]
    c.emit(transfer_word(C::Al,false,r(7),R::SP,52)?)?;
    // 0x71aac: adc r0, r3, r0
    c.emit(data_register(C::Al,Op::Add,r(0),r(3),r(0),Shift::Lsl,0,false)? | (1<<21))?;
    // 0x71ab0: ldr r7, [r12, #0xc]
    c.emit(transfer_word(C::Al,true,r(7),r(12),12)?)?;
    // 0x71ab4: str r7, [r11, #-0x30]
    c.emit(transfer_word(C::Al,false,r(7),r(11),-48)?)?;
    // 0x71ab8: ldr r10, [r12, #0x10]
    c.emit(transfer_word(C::Al,true,r(10),r(12),16)?)?;
    // 0x71abc: ldr lr, [r12, #0x14]
    c.emit(transfer_word(C::Al,true,R::LR,r(12),20)?)?;
    // 0x71ac0: str r0, [r11, #-0x28]
    c.emit(transfer_word(C::Al,false,r(0),r(11),-40)?)?;
    // 0x71ac4: ldr r0, [r11, #-0x24]
    c.emit(transfer_word(C::Al,true,r(0),r(11),-36)?)?;
    // 0x71ac8: adds r0, r10, r0
    c.emit(data_register(C::Al,Op::Add,r(0),r(10),r(0),Shift::Lsl,0,true)?)?;
    // 0x71acc: str r0, [r11, #-0x24]
    c.emit(transfer_word(C::Al,false,r(0),r(11),-36)?)?;
    // 0x71ad0: ldr r0, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(0),r(11),-32)?)?;
    // 0x71ad4: adc r0, lr, r0
    c.emit(data_register(C::Al,Op::Add,r(0),R::LR,r(0),Shift::Lsl,0,false)? | (1<<21))?;
    // 0x71ad8: str r0, [r11, #-0x20]
    c.emit(transfer_word(C::Al,false,r(0),r(11),-32)?)?;
    // 0x71adc: ldr r0, [sp, #0x34]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,52)?)?;
    // 0x71ae0: adds r4, r0, r4
    c.emit(data_register(C::Al,Op::Add,r(4),r(0),r(4),Shift::Lsl,0,true)?)?;
    // 0x71ae4: ldr r0, [r11, #-0x30]
    c.emit(transfer_word(C::Al,true,r(0),r(11),-48)?)?;
    // 0x71ae8: adc r6, r0, r6
    c.emit(data_register(C::Al,Op::Add,r(6),r(0),r(6),Shift::Lsl,0,false)? | (1<<21))?;
    // 0x71aec: ldr r0, [r12, #0x20]
    c.emit(transfer_word(C::Al,true,r(0),r(12),32)?)?;
    // 0x71af0: ldr r3, [r12, #0x24]
    c.emit(transfer_word(C::Al,true,r(3),r(12),36)?)?;
    // 0x71af4: ldr r12, [sp, #0x30]
    c.emit(transfer_word(C::Al,true,r(12),R::SP,48)?)?;
    // 0x71af8: ldr r10, [r11, #-0x2c]
    c.emit(transfer_word(C::Al,true,r(10),r(11),-44)?)?;
    // 0x71afc: adds r9, r0, r9
    c.emit(data_register(C::Al,Op::Add,r(9),r(0),r(9),Shift::Lsl,0,true)?)?;
    // 0x71b00: adc r10, r3, r10
    c.emit(data_register(C::Al,Op::Add,r(10),r(3),r(10),Shift::Lsl,0,false)? | (1<<21))?;
    // 0x71b04: cmp r1, r12
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(1),r(12),Shift::Lsl,0,false)?)?;
    // 0x71b08: bls 0x71a74 <entry+0x488>   @ imm = #-0x9c
    c.branch(C::Ls,0x71a74,false)?;
    // 0x71b0c: mov r0, r9
    c.emit(mov_register(C::Al,r(0),r(9)))?;
    // 0x71b10: mov r1, r10
    c.emit(mov_register(C::Al,r(1),r(10)))?;
    // 0x71b14: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x71b18: mov r7, r2
    c.emit(mov_register(C::Al,r(7),r(2)))?;
    // 0x71b1c: bl 0x8187c <entry+0x10290> @ imm = #0xfd58
    c.branch(C::Al,0x8187c,true)?;
    // 0x71b20: str r0, [sp, #0x34]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,52)?)?;
    // 0x71b24: str r8, [sp, #0x20]
    c.emit(transfer_word(C::Al,false,r(8),R::SP,32)?)?;
    // 0x71b28: mov r0, r8
    c.emit(mov_register(C::Al,r(0),r(8)))?;
    // 0x71b2c: ldr r8, [r11, #-0x28]
    c.emit(transfer_word(C::Al,true,r(8),r(11),-40)?)?;
    // 0x71b30: str r1, [sp, #0x1c]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,28)?)?;
    // 0x71b34: mov r2, r7
    c.emit(mov_register(C::Al,r(2),r(7)))?;
    // 0x71b38: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x71b3c: mov r1, r8
    c.emit(mov_register(C::Al,r(1),r(8)))?;
    // 0x71b40: bl 0x8187c <entry+0x10290> @ imm = #0xfd34
    c.branch(C::Al,0x8187c,true)?;
    // 0x71b44: str r0, [sp, #0x30]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,48)?)?;
    // 0x71b48: str r1, [r11, #-0x30]
    c.emit(transfer_word(C::Al,false,r(1),r(11),-48)?)?;
    // 0x71b4c: mov r0, r4
    c.emit(mov_register(C::Al,r(0),r(4)))?;
    // 0x71b50: mov r1, r6
    c.emit(mov_register(C::Al,r(1),r(6)))?;
    // 0x71b54: mov r2, r7
    c.emit(mov_register(C::Al,r(2),r(7)))?;
    // 0x71b58: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x71b5c: bl 0x8187c <entry+0x10290> @ imm = #0xfd18
    c.branch(C::Al,0x8187c,true)?;
    // 0x71b60: ldr r5, [r11, #-0x24]
    c.emit(transfer_word(C::Al,true,r(5),r(11),-36)?)?;
    // 0x71b64: str r1, [r11, #-0x2c]
    c.emit(transfer_word(C::Al,false,r(1),r(11),-44)?)?;
    // 0x71b68: ldr r1, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(1),r(11),-32)?)?;
    // 0x71b6c: str r0, [sp, #0x24]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,36)?)?;
    // 0x71b70: mov r2, r7
    c.emit(mov_register(C::Al,r(2),r(7)))?;
    // 0x71b74: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x71b78: mov r0, r5
    c.emit(mov_register(C::Al,r(0),r(5)))?;
    // 0x71b7c: bl 0x8187c <entry+0x10290> @ imm = #0xfcf8
    c.branch(C::Al,0x8187c,true)?;
    // 0x71b80: subs r2, r9, r7
    c.emit(data_register(C::Al,Op::Sub,r(2),r(9),r(7),Shift::Lsl,0,true)?)?;
    // 0x71b84: mov r12, r5
    c.emit(mov_register(C::Al,r(12),r(5)))?;
    // 0x71b88: sbcs r2, r10, #0
    c.emit(data_immediate_with_flags(C::Al,Op::Sbc,r(2),r(10),0,true)?)?;
    // 0x71b8c: mov r10, r4
    c.emit(mov_register(C::Al,r(10),r(4)))?;
    // 0x71b90: bhs 0x71ba8 <entry+0x5bc>   @ imm = #0x10
    c.branch(C::Cs,0x71ba8,false)?;
    // 0x71b94: ldr r4, [sp, #0x2c]
    c.emit(transfer_word(C::Al,true,r(4),R::SP,44)?)?;
    // 0x71b98: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x71b9c: ldr r2, [r4, #0x150]
    c.emit(transfer_word(C::Al,true,r(2),r(4),336)?)?;
    // 0x71ba0: str r2, [sp, #0x34]
    c.emit(transfer_word(C::Al,false,r(2),R::SP,52)?)?;
    // 0x71ba4: b 0x71bb0 <entry+0x5c4>   @ imm = #0x4
    c.branch(C::Al,0x71bb0,false)?;
    // 0x71ba8: ldr r4, [sp, #0x2c]
    c.emit(transfer_word(C::Al,true,r(4),R::SP,44)?)?;
    // 0x71bac: ldr r3, [sp, #0x1c]
    c.emit(transfer_word(C::Al,true,r(3),R::SP,28)?)?;
    // 0x71bb0: ldr r2, [sp, #0x20]
    c.emit(transfer_word(C::Al,true,r(2),R::SP,32)?)?;
    // 0x71bb4: ldr lr, [sp, #0x24]
    c.emit(transfer_word(C::Al,true,R::LR,R::SP,36)?)?;
    // 0x71bb8: mov r5, r1
    c.emit(mov_register(C::Al,r(5),r(1)))?;
    // 0x71bbc: ldr r1, [r11, #-0x30]
    c.emit(transfer_word(C::Al,true,r(1),r(11),-48)?)?;
    // 0x71bc0: subs r2, r2, r7
    c.emit(data_register(C::Al,Op::Sub,r(2),r(2),r(7),Shift::Lsl,0,true)?)?;
    // 0x71bc4: sbcs r2, r8, #0
    c.emit(data_immediate_with_flags(C::Al,Op::Sbc,r(2),r(8),0,true)?)?;
    // 0x71bc8: ldr r8, [sp, #0x28]
    c.emit(transfer_word(C::Al,true,r(8),R::SP,40)?)?;
    // 0x71bcc: mov r2, r0
    c.emit(mov_register(C::Al,r(2),r(0)))?;
    // 0x71bd0: ldrlo r0, [r4, #0x140]
    c.emit(transfer_word(C::Cc,true,r(0),r(4),320)?)?;
    // 0x71bd4: movlo r3, #0
    c.emit(data_immediate(C::Cc,Op::Mov,r(3),R::R0,0)?)?;
    // 0x71bd8: strlo r0, [sp, #0x34]
    c.emit(transfer_word(C::Cc,false,r(0),R::SP,52)?)?;
    // 0x71bdc: subs r0, r10, r7
    c.emit(data_register(C::Al,Op::Sub,r(0),r(10),r(7),Shift::Lsl,0,true)?)?;
    // 0x71be0: sbcs r0, r6, #0
    c.emit(data_immediate_with_flags(C::Al,Op::Sbc,r(0),r(6),0,true)?)?;
    // 0x71be4: movlo r0, #0
    c.emit(data_immediate(C::Cc,Op::Mov,r(0),R::R0,0)?)?;
    // 0x71be8: ldrlo lr, [r4, #0x144]
    c.emit(transfer_word(C::Cc,true,R::LR,r(4),324)?)?;
    // 0x71bec: strlo r0, [r11, #-0x2c]
    c.emit(transfer_word(C::Cc,false,r(0),r(11),-44)?)?;
    // 0x71bf0: subs r0, r12, r7
    c.emit(data_register(C::Al,Op::Sub,r(0),r(12),r(7),Shift::Lsl,0,true)?)?;
    // 0x71bf4: ldr r0, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(0),r(11),-32)?)?;
    // 0x71bf8: sbcs r0, r0, #0
    c.emit(data_immediate_with_flags(C::Al,Op::Sbc,r(0),r(0),0,true)?)?;
    // 0x71bfc: ldrlo r2, [r4, #0x148]
    c.emit(transfer_word(C::Cc,true,r(2),r(4),328)?)?;
    // 0x71c00: movlo r5, #0
    c.emit(data_immediate(C::Cc,Op::Mov,r(5),R::R0,0)?)?;
    // 0x71c04: ldr r0, [sp, #0x30]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,48)?)?;
    // 0x71c08: str r1, [r11, #-0x30]
    c.emit(transfer_word(C::Al,false,r(1),r(11),-48)?)?;
    // 0x71c0c: mov r4, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(4),R::R0,1)?)?;
    // 0x71c10: mov r9, r3
    c.emit(mov_register(C::Al,r(9),r(3)))?;
    // 0x71c14: mov r10, lr
    c.emit(mov_register(C::Al,r(10),R::LR))?;
    // 0x71c18: stm sp, {r0, r1, lr}
    c.emit(transfer_multiple(C::Al,false,R::SP,r(0).bit()|r(1).bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x71c1c: str r2, [r11, #-0x20]
    c.emit(transfer_word(C::Al,false,r(2),r(11),-32)?)?;
    // 0x71c20: mov r6, r0
    c.emit(mov_register(C::Al,r(6),r(0)))?;
    // 0x71c24: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x71c28: ldr r1, [r11, #-0x2c]
    c.emit(transfer_word(C::Al,true,r(1),r(11),-44)?)?;
    // 0x71c2c: str r1, [sp, #0xc]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,12)?)?;
    // 0x71c30: str r2, [sp, #0x10]
    c.emit(transfer_word(C::Al,false,r(2),R::SP,16)?)?;
    // 0x71c34: mov r2, r5
    c.emit(mov_register(C::Al,r(2),r(5)))?;
    // 0x71c38: str r5, [sp, #0x14]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,20)?)?;
    // 0x71c3c: ldr r5, [sp, #0x34]
    c.emit(transfer_word(C::Al,true,r(5),R::SP,52)?)?;
    // 0x71c40: ldr r1, [pc, #0x1c0]        @ 0x81c <entry+0x81c>
    c.literal(r(1),0x71e08)?;
    // 0x71c44: str r2, [r11, #-0x24]
    c.emit(transfer_word(C::Al,false,r(2),r(11),-36)?)?;
    // 0x71c48: mov r2, r5
    c.emit(mov_register(C::Al,r(2),r(5)))?;
    // 0x71c4c: bl 0x2ec94 <entry+0xfffffffffffbd6a8> @ imm = #-0x42fc0
    c.branch(C::Al,0x2ec94,true)?;
    // 0x71c50: ldr r1, [pc, #0x1b4]        @ 0x820 <entry+0x820>
    c.literal(r(1),0x71e0c)?;
    // 0x71c54: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x71c58: bl 0x2ec94 <entry+0xfffffffffffbd6a8> @ imm = #-0x42fcc
    c.branch(C::Al,0x2ec94,true)?;
    // 0x71c5c: umull r1, r2, r6, r10
    umull(c,r(1),r(2),r(6),r(10))?;
    // 0x71c60: umull r0, r3, r1, r5
    umull(c,r(0),r(3),r(1),r(5))?;
    // 0x71c64: mla r7, r1, r9, r3
    c.emit(multiply_accumulate(C::Al,r(7),r(1),r(9),r(3))?)?;
    // 0x71c68: ldr r3, [r11, #-0x2c]
    c.emit(transfer_word(C::Al,true,r(3),r(11),-44)?)?;
    // 0x71c6c: mla r1, r6, r3, r2
    c.emit(multiply_accumulate(C::Al,r(1),r(6),r(3),r(2))?)?;
    // 0x71c70: ldr r2, [r11, #-0x30]
    c.emit(transfer_word(C::Al,true,r(2),r(11),-48)?)?;
    // 0x71c74: mla r3, r2, r10, r1
    c.emit(multiply_accumulate(C::Al,r(3),r(2),r(10),r(1))?)?;
    // 0x71c78: ldr r2, [sp, #0x2c]
    c.emit(transfer_word(C::Al,true,r(2),R::SP,44)?)?;
    // 0x71c7c: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x71c80: mla r12, r3, r5, r7
    c.emit(multiply_accumulate(C::Al,r(12),r(3),r(5),r(7))?)?;
    // 0x71c84: ldr r3, [r2, #0x15c]
    c.emit(transfer_word(C::Al,true,r(3),r(2),348)?)?;
    // 0x71c88: ldrb r7, [r3, r4]
    c.emit(transfer_byte_register(C::Al,true,r(7),r(3),r(4),Shift::Lsl,0)?)?;
    // 0x71c8c: cmp r7, r8
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(7),r(8),Shift::Lsl,0,false)?)?;
    // 0x71c90: mov r6, r7
    c.emit(mov_register(C::Al,r(6),r(7)))?;
    // 0x71c94: movhi r6, r8
    c.emit(mov_register(C::Hi,r(6),r(8)))?;
    // 0x71c98: movls r7, r8
    c.emit(mov_register(C::Ls,r(7),r(8)))?;
    // 0x71c9c: sub r7, r7, r6
    c.emit(data_register(C::Al,Op::Sub,r(7),r(7),r(6),Shift::Lsl,0,false)?)?;
    // 0x71ca0: and r6, r1, #255
    c.emit(data_immediate(C::Al,Op::And,r(6),r(1),255)?)?;
    // 0x71ca4: ldrb r6, [r3, r6]
    c.emit(transfer_byte_register(C::Al,true,r(6),r(3),r(6),Shift::Lsl,0)?)?;
    // 0x71ca8: cmp r6, r8
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(6),r(8),Shift::Lsl,0,false)?)?;
    // 0x71cac: mov r5, r6
    c.emit(mov_register(C::Al,r(5),r(6)))?;
    // 0x71cb0: movhi r5, r8
    c.emit(mov_register(C::Hi,r(5),r(8)))?;
    // 0x71cb4: movls r6, r8
    c.emit(mov_register(C::Ls,r(6),r(8)))?;
    // 0x71cb8: sub r6, r6, r5
    c.emit(data_register(C::Al,Op::Sub,r(6),r(6),r(5),Shift::Lsl,0,false)?)?;
    // 0x71cbc: cmp r7, r6
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(7),r(6),Shift::Lsl,0,false)?)?;
    // 0x71cc0: movlo r1, r4
    c.emit(mov_register(C::Cc,r(1),r(4)))?;
    // 0x71cc4: add r4, r4, #1
    c.emit(data_immediate(C::Al,Op::Add,r(4),r(4),1)?)?;
    // 0x71cc8: cmp r4, #10
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(4),10)?)?;
    // 0x71ccc: bne 0x71c88 <entry+0x69c>   @ imm = #-0x4c
    c.branch(C::Ne,0x71c88,false)?;
    // 0x71cd0: ldr r5, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(5),r(11),-32)?)?;
    // 0x71cd4: ldr r4, [r11, #-0x24]
    c.emit(transfer_word(C::Al,true,r(4),r(11),-36)?)?;
    // 0x71cd8: and r1, r1, #255
    c.emit(data_immediate(C::Al,Op::And,r(1),r(1),255)?)?;
    // 0x71cdc: mov r9, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(9),R::R0,0)?)?;
    // 0x71ce0: add r1, r1, r1, lsl #3
    c.emit(data_register(C::Al,Op::Add,r(1),r(1),r(1),Shift::Lsl,3,false)?)?;
    // 0x71ce4: umull r3, r7, r0, r5
    umull(c,r(3),r(7),r(0),r(5))?;
    // 0x71ce8: mla r6, r0, r4, r7
    c.emit(multiply_accumulate(C::Al,r(6),r(0),r(4),r(7))?)?;
    // 0x71cec: lsr r0, r3, #16
    c.emit(mov_shifted_register(C::Al,r(0),r(3),Shift::Lsr,16)?)?;
    // 0x71cf0: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x71cf4: mla r7, r12, r5, r6
    c.emit(multiply_accumulate(C::Al,r(7),r(12),r(5),r(6))?)?;
    // 0x71cf8: mov r5, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(5),R::R0,0)?)?;
    // 0x71cfc: mov r6, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,0)?)?;
    // 0x71d00: orr r0, r0, r7, lsl #16
    c.emit(data_register(C::Al,Op::Orr,r(0),r(0),r(7),Shift::Lsl,16,false)?)?;
    // 0x71d04: str r0, [r11, #-0x20]
    c.emit(transfer_word(C::Al,false,r(0),r(11),-32)?)?;
    // 0x71d08: lsr r0, r7, #16
    c.emit(mov_shifted_register(C::Al,r(0),r(7),Shift::Lsr,16)?)?;
    // 0x71d0c: mov r7, #1073741824
    c.emit(data_immediate(C::Al,Op::Mov,r(7),R::R0,1073741824)?)?;
    // 0x71d10: ldr r2, [r2, #0x154]
    c.emit(transfer_word(C::Al,true,r(2),r(2),340)?)?;
    // 0x71d14: str r0, [r11, #-0x24]
    c.emit(transfer_word(C::Al,false,r(0),r(11),-36)?)?;
    // 0x71d18: add lr, r2, r1, lsl #2
    c.emit(data_register(C::Al,Op::Add,R::LR,r(2),r(1),Shift::Lsl,2,false)?)?;
    // 0x71d1c: ldr r0, [lr], #4
    c.emit(transfer_word_postindexed(C::Al,true,r(0),R::LR,4)?)?;
    // 0x71d20: ldr r1, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(1),r(11),-32)?)?;
    // 0x71d24: ldr r2, [r11, #-0x24]
    c.emit(transfer_word(C::Al,true,r(2),r(11),-36)?)?;
    // 0x71d28: mov r12, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(12),R::R0,0)?)?;
    // 0x71d2c: subs r8, r1, r0
    c.emit(data_register(C::Al,Op::Sub,r(8),r(1),r(0),Shift::Lsl,0,true)?)?;
    // 0x71d30: sbc r10, r2, #0
    c.emit(data_immediate(C::Al,Op::Sbc,r(10),r(2),0)?)?;
    // 0x71d34: subs r4, r0, r1
    c.emit(data_register(C::Al,Op::Sub,r(4),r(0),r(1),Shift::Lsl,0,true)?)?;
    // 0x71d38: rscs r2, r2, #0
    c.emit(data_immediate_with_flags(C::Al,Op::Rsc,r(2),r(2),0,true)?)?;
    // 0x71d3c: movlo r12, #1
    c.emit(data_immediate(C::Cc,Op::Mov,r(12),R::R0,1)?)?;
    // 0x71d40: cmp r12, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(12),0)?)?;
    // 0x71d44: movne r4, r8
    c.emit(mov_register(C::Ne,r(4),r(8)))?;
    // 0x71d48: movne r2, r10
    c.emit(mov_register(C::Ne,r(2),r(10)))?;
    // 0x71d4c: subs r1, r4, r7
    c.emit(data_register(C::Al,Op::Sub,r(1),r(4),r(7),Shift::Lsl,0,true)?)?;
    // 0x71d50: sbcs r1, r2, r3
    c.emit(data_register(C::Al,Op::Sbc,r(1),r(2),r(3),Shift::Lsl,0,true)?)?;
    // 0x71d54: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x71d58: movhs r1, #1
    c.emit(data_immediate(C::Cs,Op::Mov,r(1),R::R0,1)?)?;
    // 0x71d5c: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x71d60: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x71d64: moveq r0, #1
    c.emit(data_immediate(C::Eq,Op::Mov,r(0),R::R0,1)?)?;
    // 0x71d68: orrs r0, r0, r1
    c.emit(data_register(C::Al,Op::Orr,r(0),r(0),r(1),Shift::Lsl,0,true)?)?;
    // 0x71d6c: moveq r6, r5
    c.emit(mov_register(C::Eq,r(6),r(5)))?;
    // 0x71d70: moveq r9, r5
    c.emit(mov_register(C::Eq,r(9),r(5)))?;
    // 0x71d74: add r5, r5, #1
    c.emit(data_immediate(C::Al,Op::Add,r(5),r(5),1)?)?;
    // 0x71d78: moveq r3, r2
    c.emit(mov_register(C::Eq,r(3),r(2)))?;
    // 0x71d7c: moveq r7, r4
    c.emit(mov_register(C::Eq,r(7),r(4)))?;
    // 0x71d80: cmp r5, #9
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),9)?)?;
    // 0x71d84: bne 0x71d1c <entry+0x730>   @ imm = #-0x70
    c.branch(C::Ne,0x71d1c,false)?;
    // 0x71d88: ldr r1, [pc, #0x80]         @ 0x824 <entry+0x824>
    c.literal(r(1),0x71e10)?;
    // 0x71d8c: and r2, r6, #255
    c.emit(data_immediate(C::Al,Op::And,r(2),r(6),255)?)?;
    // 0x71d90: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x71d94: bl 0x2ec94 <entry+0xfffffffffffbd6a8> @ imm = #-0x43108
    c.branch(C::Al,0x2ec94,true)?;
    // 0x71d98: ldr r1, [pc, #0x6c]         @ 0x820 <entry+0x820>
    c.literal(r(1),0x71e0c)?;
    // 0x71d9c: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x71da0: bl 0x2ec94 <entry+0xfffffffffffbd6a8> @ imm = #-0x43114
    c.branch(C::Al,0x2ec94,true)?;
    // 0x71da4: ldr r5, [sp, #0x2c]
    c.emit(transfer_word(C::Al,true,r(5),R::SP,44)?)?;
    // 0x71da8: ldr r12, [r5, #0x164]
    c.emit(transfer_word(C::Al,true,r(12),r(5),356)?)?;
    // 0x71dac: ldr r0, [pc, #0x50]         @ 0x818 <entry+0x818>
    c.literal(r(0),0x71e04)?;
    // 0x71db0: umull r1, r2, r12, r0
    umull(c,r(1),r(2),r(12),r(0))?;
    // 0x71db4: lsr r0, r2, #2
    c.emit(mov_shifted_register(C::Al,r(0),r(2),Shift::Lsr,2)?)?;
    // 0x71db8: and r1, r9, #255
    c.emit(data_immediate(C::Al,Op::And,r(1),r(9),255)?)?;
    // 0x71dbc: add r0, r0, r0, lsl #2
    c.emit(data_register(C::Al,Op::Add,r(0),r(0),r(0),Shift::Lsl,2,false)?)?;
    // 0x71dc0: sub r0, r12, r0
    c.emit(data_register(C::Al,Op::Sub,r(0),r(12),r(0),Shift::Lsl,0,false)?)?;
    // 0x71dc4: add r0, r5, r0, lsl #6
    c.emit(data_register(C::Al,Op::Add,r(0),r(5),r(0),Shift::Lsl,6,false)?)?;
    // 0x71dc8: ldrb r0, [r0, #0x39]
    c.emit(transfer_byte(C::Al,true,r(0),r(0),57)?)?;
    // 0x71dcc: add r1, r0, r1
    c.emit(data_register(C::Al,Op::Add,r(1),r(0),r(1),Shift::Lsl,0,false)?)?;
    // 0x71dd0: ldr r0, [pc, #0x2c]         @ 0x818 <entry+0x818>
    c.literal(r(0),0x71e04)?;
    // 0x71dd4: umull r2, r3, r12, r0
    umull(c,r(2),r(3),r(12),r(0))?;
    // 0x71dd8: lsr r0, r3, #2
    c.emit(mov_shifted_register(C::Al,r(0),r(3),Shift::Lsr,2)?)?;
    // 0x71ddc: add r0, r0, r0, lsl #2
    c.emit(data_register(C::Al,Op::Add,r(0),r(0),r(0),Shift::Lsl,2,false)?)?;
    // 0x71de0: sub r0, r12, r0
    c.emit(data_register(C::Al,Op::Sub,r(0),r(12),r(0),Shift::Lsl,0,false)?)?;
    // 0x71de4: add r0, r5, r0, lsl #6
    c.emit(data_register(C::Al,Op::Add,r(0),r(5),r(0),Shift::Lsl,6,false)?)?;
    // 0x71de8: strb r1, [r0, #0x38]
    c.emit(transfer_byte(C::Al,false,r(1),r(0),56)?)?;
    // 0x71dec: add r0, r12, #1
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(12),1)?)?;
    // 0x71df0: str r0, [r5, #0x164]
    c.emit(transfer_word(C::Al,false,r(0),r(5),356)?)?;
    // 0x71df4: and r0, r1, #255
    c.emit(data_immediate(C::Al,Op::And,r(0),r(1),255)?)?;
    // 0x71df8: sub sp, r11, #28
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,r(11),28)?)?;
    // 0x71dfc: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;

    c.check(0x71e00)?;
    for word in [LOG_RECORD,DIVIDE_BY_FIVE,LOG_PROFILE,LOG_FOOTER,LOG_RESULT]{c.emit(word)?;}
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();select_closest_record(&mut c)?;Ok(c.bytes)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn five_named_literal_words_after_return(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        for (i,word) in [LOG_RECORD,DIVIDE_BY_FIVE,LOG_PROFILE,LOG_FOOTER,LOG_RESULT].into_iter().enumerate(){
            let at=b.len()-20+i*4;
            assert_eq!(&b[at..at+4],&word.to_le_bytes());
        }
    }
}
