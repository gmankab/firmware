// SPDX-License-Identifier: GPL-3.0-only
//! Initialize codec input-mode configuration and controller command defaults:
//! per-plane format/strides, channel enable masks, callback/status arrays,
//! short packed command tables and baseline MMIO data. This is instruction-
//! exact typed A32 re-encoding, not independent high-level codec algorithms.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x36048;
const END:usize=0x3701c;

/// Encode pre-indexed positive/negative shifted-register STRH/LDRH. Public
/// A32 architecture bitfields, independently tested against LLVM; never raw
/// reference instruction words. Writeback allowed only when safe.
fn encode_halfword_register(cond:C,load:bool,dest:R,base:R,index:R,writeback:bool)->Result<u32,String>{
    if base==R::PC || index==R::PC || dest==R::PC || (load && dest==base && writeback) {
        return Err("invalid indexed halfword register addressing".into());
    }
    Ok(((cond as u32)<<28)|0x0180_00b0
       |(if load {1<<20} else {0})|(if writeback {1<<21} else {0})
       |(base.number()<<16)|(dest.number()<<12)|index.number())
}

/// A32 LDRB/STRB with pre-indexed signed imm12 offset and base writeback.
fn encode_byte_preindexed_writeback(cond:C,load:bool,dest:R,base:R,offset:i32)->Result<u32,String>{
    if base==R::PC || (load && dest==base) || offset.unsigned_abs()>4095 {
        return Err("invalid indexed byte writeback addressing".into());
    }
    Ok(arm32::transfer_word_preindexed_writeback(cond,load,dest,base,offset)? | (1<<22))
}

const CONTROLLER_INITIALIZATION_LITERALS:[u32;17]=[
    0x000ec0b3,
    0x0010df5c,
    0x004c4b40,
    0x001040e4,
    0x000e53ee,
    0x000e2788,
    0x0010df7c,
    0x000e541f,
    0x000e5471,
    0x15020100,
    0x080b0716,
    0x0d0c0a09,
    0x11100f0e,
    0x03141312,
    0x00060504,
    0xaaaaaaaa,
    0x01c9c380,
];

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // Controller setup block: controller_mode, runtime [0x36048, 0x36400).
    // 0x36048: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x3604c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x36050: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // 0x36054: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x36058: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x3605c: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x36060: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x36064: beq 0x36084 <entry+0xbf8>   @ imm = #0x18
    c.b(C::Eq,0x36084)?;
    // 0x36068: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x3606c: beq 0x360a0 <entry+0xc14>   @ imm = #0x2c
    c.b(C::Eq,0x360a0)?;
    // 0x36070: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x36074: bne 0x360b0 <entry+0xc24>   @ imm = #0x34
    c.b(C::Ne,0x360b0)?;
    // 0x36078: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x3607c: mov r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16384)?)?;
    // 0x36080: b 0x36094 <entry+0xc08>   @ imm = #0xc
    c.b(C::Al,0x36094)?;
    // 0x36084: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x36088: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x3608c: mov r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // 0x36090: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x36094: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x36098: str r1, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,12)?)?;
    // 0x3609c: b 0x360c0 <entry+0xc34>   @ imm = #0x1c
    c.b(C::Al,0x360c0)?;
    // 0x360a0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x360a4: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x360a8: str r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // 0x360ac: b 0x360c0 <entry+0xc34>   @ imm = #0xc
    c.b(C::Al,0x360c0)?;
    // 0x360b0: ldr r2, [pc, #0xf20]        @ 0x1b4c <entry+0x1b4c>
    c.literal_cond(C::Al,R::R2,0x36fd8)?;
    // 0x360b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x360b8: mov r1, #984
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,984)?)?;
    // 0x360bc: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x34914
    c.bl(0x17b0)?;
    // 0x360c0: ldr r5, [pc, #0xf14]        @ 0x1b50 <entry+0x1b50>
    c.literal_cond(C::Al,R::new(5)?,0x36fdc)?;
    // 0x360c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x360c8: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x360cc: mov r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1280)?)?;
    // 0x360d0: mov r2, #720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,720)?)?;
    // 0x360d4: str r0, [r4, #0x794]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1940)?)?;
    // 0x360d8: str r7, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,40)?)?;
    // 0x360dc: str r1, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,32)?)?;
    // 0x360e0: str r2, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,36)?)?;
    // 0x360e4: str r1, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,76)?)?;
    // 0x360e8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x360ec: str r2, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,80)?)?;
    // 0x360f0: str r0, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,288)?)?;
    // 0x360f4: str r0, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,296)?)?;
    // 0x360f8: str r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // 0x360fc: str r1, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,28)?)?;
    // 0x36100: str r1, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,24)?)?;
    // 0x36104: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // 0x36108: str r0, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,196)?)?;
    // 0x3610c: strh r0, [r4, #200]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(4)?,200)?)?;
    // 0x36110: strb r7, [r4, #0x13b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,315)?)?;
    // 0x36114: strb r7, [r4, #0x13d]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,317)?)?;
    // 0x36118: strb r0, [r4, #0x13c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,316)?)?;
    // 0x3611c: strb r0, [r4, #0x13f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,319)?)?;
    // 0x36120: ldrb r0, [r5, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x36124: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x36128: ldrne r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,16)?)?;
    // 0x3612c: strbne r0, [r4, #0x13f]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,319)?)?;
    // 0x36130: bl 0x2f8ac <entry+0xffffffffffffa420> @ imm = #-0x688c
    c.bl(0x2f8ac)?;
    // 0x36134: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x36138: beq 0x36148 <entry+0xcbc>   @ imm = #0x8
    c.b(C::Eq,0x36148)?;
    // 0x3613c: bl 0x2f8d0 <entry+0xffffffffffffa444> @ imm = #-0x6874
    c.bl(0x2f8d0)?;
    // 0x36140: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x36144: moveq r7, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x36148: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3614c: strb r7, [r4, #0x141]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,321)?)?;
    // 0x36150: strb r7, [r4, #0x145]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,325)?)?;
    // 0x36154: strb r7, [r4, #0x146]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,326)?)?;
    // 0x36158: strb r7, [r4, #0x147]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,327)?)?;
    // 0x3615c: strb r7, [r4, #0x149]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,329)?)?;
    // 0x36160: mov r3, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,22)?)?;
    // 0x36164: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x36168: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // 0x3616c: strb r0, [r4, #0x140]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,320)?)?;
    // 0x36170: strb r0, [r4, #0x143]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,323)?)?;
    // 0x36174: strb r0, [r4, #0x144]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,324)?)?;
    // 0x36178: strb r0, [r4, #0x148]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,328)?)?;
    // 0x3617c: strb r0, [r4, #0x14a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,330)?)?;
    // 0x36180: str r0, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,288)?)?;
    // 0x36184: str r0, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,296)?)?;
    // 0x36188: str r0, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,300)?)?;
    // 0x3618c: str r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,104)?)?;
    // 0x36190: ldr r1, [pc, #0xe48]        @ 0x1b54 <entry+0x1b54>
    c.literal_cond(C::Al,R::R1,0x36fe0)?;
    // 0x36194: str r1, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,100)?)?;
    // 0x36198: mov r1, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,23)?)?;
    // 0x3619c: strh r3, [r4, #116]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::new(4)?,116)?)?;
    // 0x361a0: strh r1, [r4, #118]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(4)?,118)?)?;
    // 0x361a4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x361a8: str r0, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,108)?)?;
    // 0x361ac: str r0, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,112)?)?;
    // 0x361b0: strb r0, [r4, #0x15d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,349)?)?;
    // 0x361b4: strb r0, [r4, #0x169]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,361)?)?;
    // 0x361b8: strb r0, [r4, #0xbc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,188)?)?;
    // 0x361bc: strb r1, [r4, #0xd1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,209)?)?;
    // 0x361c0: str r2, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,4)?)?;
    // 0x361c4: str r0, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,136)?)?;
    // 0x361c8: strh r3, [r4, #120]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::new(4)?,120)?)?;
    // 0x361cc: str r0, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,284)?)?;
    // 0x361d0: strb r0, [r4, #0xad]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,173)?)?;
    // 0x361d4: strb r0, [r4, #0xae]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,174)?)?;
    // 0x361d8: str r0, [r4, #0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,176)?)?;
    // 0x361dc: strb r0, [r4, #0xaa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,170)?)?;
    // 0x361e0: strb r0, [r4, #0xac]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,172)?)?;
    // 0x361e4: str r0, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,180)?)?;
    // 0x361e8: strb r1, [r4, #0x164]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,356)?)?;
    // 0x361ec: strb r0, [r4, #0x9c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,156)?)?;
    // 0x361f0: str r0, [r4, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,160)?)?;
    // 0x361f4: str r0, [r4, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,164)?)?;
    // 0x361f8: strb r1, [r4, #0xa8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,168)?)?;
    // 0x361fc: strb r0, [r4, #0x168]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,360)?)?;
    // 0x36200: strb r0, [r4, #0x162]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,354)?)?;
    // 0x36204: strb r0, [r4, #0x166]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,358)?)?;
    // 0x36208: str r0, [r4, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,184)?)?;
    // 0x3620c: bne 0x36224 <entry+0xd98>   @ imm = #0x10
    c.b(C::Ne,0x36224)?;
    // 0x36210: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x36214: strb r2, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,350)?)?;
    // 0x36218: strb r1, [r4, #0x142]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,322)?)?;
    // 0x3621c: strb r0, [r4, #0x13e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,318)?)?;
    // 0x36220: b 0x36250 <entry+0xdc4>   @ imm = #0x28
    c.b(C::Al,0x36250)?;
    // 0x36224: strb r1, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,350)?)?;
    // 0x36228: strb r0, [r4, #0x142]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,322)?)?;
    // 0x3622c: bl 0x2f8d0 <entry+0xffffffffffffa444> @ imm = #-0x6964
    c.bl(0x2f8d0)?;
    // 0x36230: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x36234: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x36238: strbne r0, [r4, #0x142]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,322)?)?;
    // 0x3623c: bl 0x2f8ac <entry+0xffffffffffffa420> @ imm = #-0x6998
    c.bl(0x2f8ac)?;
    // 0x36240: mov r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x36244: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x36248: movne r1, #14
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x3624c: strb r1, [r4, #0x13e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,318)?)?;
    // 0x36250: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x36254: tst r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // 0x36258: ldrne r1, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(5)?,20)?)?;
    // 0x3625c: strbne r1, [r4, #0x13e]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(4)?,318)?)?;
    // 0x36260: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x36264: ldrne r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,12)?)?;
    // 0x36268: strbne r0, [r4, #0x142]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,322)?)?;
    // 0x3626c: ldr r0, [pc, #0xd70]        @ 0x1b58 <entry+0x1b58>
    c.literal_cond(C::Al,R::R0,0x36fe4)?;
    // 0x36270: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x36274: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x36278: beq 0x362f4 <entry+0xe68>   @ imm = #0x74
    c.b(C::Eq,0x362f4)?;
    // 0x3627c: ldr r1, [pc, #0xd64]        @ 0x1b5c <entry+0x1b5c>
    c.literal_cond(C::Al,R::R1,0x36fe8)?;
    // 0x36280: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x36284: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x75f8
    c.bl(0x2ec94)?;
    // 0x36288: ldr r6, [pc, #0xd5c]        @ 0x1b60 <entry+0x1b60>
    c.literal_cond(C::Al,R::new(6)?,0x36fec)?;
    // 0x3628c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x36290: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x36294: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x7608
    c.bl(0x2ec94)?;
    // 0x36298: ldr r5, [pc, #0xd50]        @ 0x1b64 <entry+0x1b64>
    c.literal_cond(C::Al,R::new(5)?,0x36ff0)?;
    // 0x3629c: add r7, r5, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(5)?,12)?)?;
    // 0x362a0: ldm r5, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::R2.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x362a4: ldm r7, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(7)?,R::R0.bit() | R::R1.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x362a8: stm sp, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x362ac: ldr r1, [pc, #0xd40]        @ 0x1b68 <entry+0x1b68>
    c.literal_cond(C::Al,R::R1,0x36ff4)?;
    // 0x362b0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x362b4: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x7628
    c.bl(0x2ec94)?;
    // 0x362b8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x362bc: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x362c0: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x7634
    c.bl(0x2ec94)?;
    // 0x362c4: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x362c8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x362cc: and r2, r1, r0, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,2,false)?)?;
    // 0x362d0: and r3, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // 0x362d4: ldr r1, [pc, #0xd1c]        @ 0x1b6c <entry+0x1b6c>
    c.literal_cond(C::Al,R::R1,0x36ff8)?;
    // 0x362d8: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x362dc: and r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,1)?)?;
    // 0x362e0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x362e4: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x7658
    c.bl(0x2ec94)?;
    // 0x362e8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x362ec: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x362f0: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x7664
    c.bl(0x2ec94)?;
    // 0x362f4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x362f8: mov r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // 0x362fc: add r0, r4, #224
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,224)?)?;
    // 0x36300: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x36304: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x36308: strb r6, [r4, #0x159]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,345)?)?;
    // 0x3630c: strb r9, [r4, #0x15f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,351)?)?;
    // 0x36310: strb r9, [r4, #0x160]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,352)?)?;
    // 0x36314: strb r6, [r4, #0x161]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,353)?)?;
    // 0x36318: strb r6, [r4, #0x15b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,347)?)?;
    // 0x3631c: strb r6, [r4, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,348)?)?;
    // 0x36320: strh r6, [r4, #216]
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::new(4)?,216)?)?;
    // 0x36324: strh r6, [r4, #214]
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::new(4)?,214)?)?;
    // 0x36328: strh r6, [r4, #212]
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::new(4)?,212)?)?;
    // 0x3632c: strh r6, [r4, #210]
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::new(4)?,210)?)?;
    // 0x36330: strb r6, [r4, #0xda]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,218)?)?;
    // 0x36334: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x33150
    c.bl(0x31ec)?;
    // 0x36338: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x3633c: strb r9, [r4, #0x626]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1574)?)?;
    // 0x36340: mov r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,2)?)?;
    // 0x36344: mov r1, #245
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,245)?)?;
    // 0x36348: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x3634c: mov r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x36350: mov r5, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,15)?)?;
    // 0x36354: str r0, [r4, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,372)?)?;
    // 0x36358: strb r6, [r4, #0x23c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,572)?)?;
    // 0x3635c: strb r0, [r4, #0x24d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,589)?)?;
    // 0x36360: mov r0, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,23)?)?;
    // 0x36364: strb r8, [r4, #0x24e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(4)?,590)?)?;
    // 0x36368: strb r6, [r4, #0x24f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,591)?)?;
    // 0x3636c: strb r6, [r4, #0x250]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,592)?)?;
    // 0x36370: strb r6, [r4, #0x258]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,600)?)?;
    // 0x36374: orr r1, r1, #11776
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,11776)?)?;
    // 0x36378: str r0, [r4, #0x25c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,604)?)?;
    // 0x3637c: str r0, [r4, #0x260]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,608)?)?;
    // 0x36380: str r0, [r4, #0x264]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,612)?)?;
    // 0x36384: str r0, [r4, #0x268]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,616)?)?;
    // 0x36388: str r0, [r4, #0x26c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,620)?)?;
    // 0x3638c: str r0, [r4, #0x270]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,624)?)?;
    // 0x36390: str r0, [r4, #0x274]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,628)?)?;
    // 0x36394: str r0, [r4, #0x278]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,632)?)?;
    // 0x36398: mov r0, #51
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,51)?)?;
    // 0x3639c: str r6, [r4, #0x27c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,636)?)?;
    // 0x363a0: str r6, [r4, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,640)?)?;
    // 0x363a4: str r6, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,644)?)?;
    // 0x363a8: str r0, [r4, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,648)?)?;
    // 0x363ac: str r0, [r4, #0x28c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,652)?)?;
    // 0x363b0: str r0, [r4, #0x290]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,656)?)?;
    // 0x363b4: mov r0, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,96)?)?;
    // 0x363b8: str r6, [r4, #0x240]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,576)?)?;
    // 0x363bc: str r6, [r4, #0x224]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,548)?)?;
    // 0x363c0: str r6, [r4, #0x228]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,552)?)?;
    // 0x363c4: str r9, [r4, #0x294]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,660)?)?;
    // 0x363c8: str r9, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,664)?)?;
    // 0x363cc: str r6, [r4, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,424)?)?;
    // 0x363d0: str r6, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,412)?)?;
    // 0x363d4: str r9, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,416)?)?;
    // 0x363d8: str r6, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,388)?)?;
    // 0x363dc: str r6, [r4, #0x218]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,536)?)?;
    // 0x363e0: str r0, [r4, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,392)?)?;
    // 0x363e4: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x363e8: str r0, [r4, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,396)?)?;
    // 0x363ec: mov r0, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,192)?)?;
    // 0x363f0: str r0, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,400)?)?;
    // 0x363f4: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // 0x363f8: str r0, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,404)?)?;
    // 0x363fc: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    c.check(0x36400)?;
    // Controller setup block: format_table, runtime [0x36400, 0x36800).
    // 0x36400: strb r6, [r4, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,380)?)?;
    // 0x36404: str r6, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,420)?)?;
    // 0x36408: str r6, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,384)?)?;
    // 0x3640c: str r6, [r4, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,428)?)?;
    // 0x36410: str r9, [r4, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,456)?)?;
    // 0x36414: str r6, [r4, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,464)?)?;
    // 0x36418: orr r0, r0, #46848
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,46848)?)?;
    // 0x3641c: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // 0x36420: str r1, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,508)?)?;
    // 0x36424: str r0, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,512)?)?;
    // 0x36428: str r1, [r4, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,516)?)?;
    // 0x3642c: str r0, [r4, #0x208]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,520)?)?;
    // 0x36430: str r1, [r4, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,524)?)?;
    // 0x36434: str r0, [r4, #0x210]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,528)?)?;
    // 0x36438: mov r0, #109
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,109)?)?;
    // 0x3643c: str r1, [r4, #0x214]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,532)?)?;
    // 0x36440: str r6, [r4, #0x21c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,540)?)?;
    // 0x36444: str r6, [r4, #0x220]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,544)?)?;
    // 0x36448: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3644c: str r0, [r4, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,568)?)?;
    // 0x36450: str r0, [r4, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,564)?)?;
    // 0x36454: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x36458: str r0, [r4, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,560)?)?;
    // 0x3645c: add r0, r4, #792
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,792)?)?;
    // 0x36460: str r6, [r4, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,556)?)?;
    // 0x36464: str r6, [r4, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,408)?)?;
    // 0x36468: str r6, [r4, #0x310]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,784)?)?;
    // 0x3646c: str r9, [r4, #0x314]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,788)?)?;
    // 0x36470: strb r6, [r4, #0x251]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,593)?)?;
    // 0x36474: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x33290
    c.bl(0x31ec)?;
    // 0x36478: mov r0, #800
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,800)?)?;
    // 0x3647c: strb r7, [r4, #0x248]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,584)?)?;
    // 0x36480: strb r9, [r4, #0x249]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,585)?)?;
    // 0x36484: strb r9, [r4, #0x24a]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,586)?)?;
    // 0x36488: strb r8, [r4, #0x24b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(4)?,587)?)?;
    // 0x3648c: strb r9, [r4, #0x24c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,588)?)?;
    // 0x36490: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x36494: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x36498: str r0, [r4, #0x254]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,596)?)?;
    // 0x3649c: mov r0, #198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,198)?)?;
    // 0x364a0: str r6, [r4, #0x29c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,668)?)?;
    // 0x364a4: str r6, [r4, #0x2a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,672)?)?;
    // 0x364a8: str r6, [r4, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,676)?)?;
    // 0x364ac: strb r6, [r4, #0x2af]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,687)?)?;
    // 0x364b0: strb r6, [r4, #0x15b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,347)?)?;
    // 0x364b4: strb r6, [r4, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,348)?)?;
    // 0x364b8: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // 0x364bc: strh r6, [r4, r0]
    c.emit(encode_halfword_register(C::Al,false,R::new(6)?,R::new(4)?,R::R0,false)?)?;
    // 0x364c0: mov r0, #712
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,712)?)?;
    // 0x364c4: strh r6, [r4, r0]
    c.emit(encode_halfword_register(C::Al,false,R::new(6)?,R::new(4)?,R::R0,false)?)?;
    // 0x364c8: mov r0, #246
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,246)?)?;
    // 0x364cc: strb r6, [r4, #0x2c3]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,707)?)?;
    // 0x364d0: strb r6, [r4, #0x2c4]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,708)?)?;
    // 0x364d4: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // 0x364d8: strh r6, [r4, r0]
    c.emit(encode_halfword_register(C::Al,false,R::new(6)?,R::new(4)?,R::R0,false)?)?;
    // 0x364dc: mov r0, #760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,760)?)?;
    // 0x364e0: strh r2, [r4, r0]
    c.emit(encode_halfword_register(C::Al,false,R::R2,R::new(4)?,R::R0,false)?)?;
    // 0x364e4: mov r0, #250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,250)?)?;
    // 0x364e8: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // 0x364ec: strh r6, [r4, r0]
    c.emit(encode_halfword_register(C::Al,false,R::new(6)?,R::new(4)?,R::R0,false)?)?;
    // 0x364f0: mov r0, #764
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,764)?)?;
    // 0x364f4: strh r2, [r1, r0]!
    c.emit(encode_halfword_register(C::Al,false,R::R2,R::R1,R::R0,true)?)?;
    // 0x364f8: add r0, r4, #210
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,210)?)?;
    // 0x364fc: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x36500: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x36504: add r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,512)?)?;
    // 0x36508: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3650c: strb r6, [r4, #0x2c0]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,704)?)?;
    // 0x36510: strb r6, [r4, #0x2c1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,705)?)?;
    // 0x36514: strb r6, [r4, #0x2c2]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,706)?)?;
    // 0x36518: strb r6, [r4, #0x2b3]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,691)?)?;
    // 0x3651c: strb r5, [r4, #0x2ae]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,686)?)?;
    // 0x36520: strb r6, [r4, #0x2b0]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,688)?)?;
    // 0x36524: strb r6, [r4, #0x2b1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,689)?)?;
    // 0x36528: strb r6, [r4, #0x2b2]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,690)?)?;
    // 0x3652c: strb r6, [r4, #0x2b4]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,692)?)?;
    // 0x36530: strb r6, [r4, #0x2b5]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,693)?)?;
    // 0x36534: str r6, [r4, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,696)?)?;
    // 0x36538: str r6, [r4, #0x2bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,700)?)?;
    // 0x3653c: str r6, [r4, #0x308]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,776)?)?;
    // 0x36540: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x3335c
    c.bl(0x31ec)?;
    // 0x36544: add r0, r4, #812
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,812)?)?;
    // 0x36548: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3654c: mov r2, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // 0x36550: str r8, [r4, #0x244]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,580)?)?;
    // 0x36554: str r6, [r4, #0x634]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1588)?)?;
    // 0x36558: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x33374
    c.bl(0x31ec)?;
    // 0x3655c: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x36560: mov r5, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,5)?)?;
    // 0x36564: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x36568: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x3656c: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x36570: mov r8, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,13)?)?;
    // 0x36574: mov r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,2)?)?;
    // 0x36578: mov r10, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,7)?)?;
    // 0x3657c: mov r7, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,10)?)?;
    // 0x36580: mov lr, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,17)?)?;
    // 0x36584: str r0, [r4, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,944)?)?;
    // 0x36588: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x3658c: str r6, [r4, #0x3b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,948)?)?;
    // 0x36590: str r6, [r4, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,432)?)?;
    // 0x36594: str r6, [r4, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,436)?)?;
    // 0x36598: str r0, [r4, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,440)?)?;
    // 0x3659c: str r0, [r4, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,444)?)?;
    // 0x365a0: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x365a4: str r3, [r4, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,448)?)?;
    // 0x365a8: str r0, [r4, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,452)?)?;
    // 0x365ac: str r3, [r4, #0x3b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,952)?)?;
    // 0x365b0: str r6, [r4, #0x4a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1188)?)?;
    // 0x365b4: str r6, [r4, #0x4a8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1192)?)?;
    // 0x365b8: str r6, [r4, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,956)?)?;
    // 0x365bc: str r6, [r4, #0x3c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,960)?)?;
    // 0x365c0: str r9, [r4, #0x3c4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,964)?)?;
    // 0x365c4: str r9, [r4, #0x3c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,968)?)?;
    // 0x365c8: str r9, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,972)?)?;
    // 0x365cc: str r6, [r4, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,984)?)?;
    // 0x365d0: str r6, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,988)?)?;
    // 0x365d4: str r6, [r4, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,992)?)?;
    // 0x365d8: str r9, [r4, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,996)?)?;
    // 0x365dc: str r0, [r4, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1000)?)?;
    // 0x365e0: str r5, [r4, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1004)?)?;
    // 0x365e4: str r6, [r4, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1008)?)?;
    // 0x365e8: str r6, [r4, #0x3f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1012)?)?;
    // 0x365ec: str r6, [r4, #0x3f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1016)?)?;
    // 0x365f0: str r3, [r4, #0x3fc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1020)?)?;
    // 0x365f4: str r3, [r4, #0x400]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1024)?)?;
    // 0x365f8: str r3, [r4, #0x404]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1028)?)?;
    // 0x365fc: str r3, [r4, #0x408]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1032)?)?;
    // 0x36600: str r3, [r4, #0x40c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1036)?)?;
    // 0x36604: str r6, [r4, #0x410]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1040)?)?;
    // 0x36608: str r6, [r4, #0x414]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1044)?)?;
    // 0x3660c: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x36610: ldr r0, [pc, #0x9e4]        @ 0x1b70 <entry+0x1b70>
    c.literal_cond(C::Al,R::R0,0x36ffc)?;
    // 0x36614: str r0, [r4, #0x418]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1048)?)?;
    // 0x36618: ldr r0, [pc, #0x9e0]        @ 0x1b74 <entry+0x1b74>
    c.literal_cond(C::Al,R::R0,0x37000)?;
    // 0x3661c: str r0, [r4, #0x41c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1052)?)?;
    // 0x36620: ldr r0, [pc, #0x9dc]        @ 0x1b78 <entry+0x1b78>
    c.literal_cond(C::Al,R::R0,0x37004)?;
    // 0x36624: str r0, [r4, #0x420]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1056)?)?;
    // 0x36628: ldr r0, [pc, #0x9d8]        @ 0x1b7c <entry+0x1b7c>
    c.literal_cond(C::Al,R::R0,0x37008)?;
    // 0x3662c: str r0, [r4, #0x424]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1060)?)?;
    // 0x36630: ldr r0, [pc, #0x9d4]        @ 0x1b80 <entry+0x1b80>
    c.literal_cond(C::Al,R::R0,0x3700c)?;
    // 0x36634: str r0, [r4, #0x428]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1064)?)?;
    // 0x36638: ldr r0, [pc, #0x9d0]        @ 0x1b84 <entry+0x1b84>
    c.literal_cond(C::Al,R::R0,0x37010)?;
    // 0x3663c: str r0, [r4, #0x42c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1068)?)?;
    // 0x36640: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x36644: strb r9, [r4, #0x430]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1072)?)?;
    // 0x36648: strb r0, [r4, #0x431]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1073)?)?;
    // 0x3664c: strb r3, [r4, #0x432]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1074)?)?;
    // 0x36650: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x36654: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x36658: strb r2, [r4, #0x433]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1075)?)?;
    // 0x3665c: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x36660: strb r3, [r4, #0x434]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1076)?)?;
    // 0x36664: strb r5, [r4, #0x435]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1077)?)?;
    // 0x36668: strb r1, [r4, #0x436]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1078)?)?;
    // 0x3666c: strb r0, [r4, #0x437]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1079)?)?;
    // 0x36670: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x36674: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x36678: mov r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16)?)?;
    // 0x3667c: strb r0, [r4, #0x438]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1080)?)?;
    // 0x36680: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x36684: strb r0, [r4, #0x439]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1081)?)?;
    // 0x36688: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x3668c: strb r0, [r4, #0x43a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1082)?)?;
    // 0x36690: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x36694: strb r0, [r4, #0x43b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1083)?)?;
    // 0x36698: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x3669c: strb r0, [r4, #0x43c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1084)?)?;
    // 0x366a0: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x366a4: strb r0, [r4, #0x43d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1085)?)?;
    // 0x366a8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x366ac: strb r2, [r4, #0x43e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1086)?)?;
    // 0x366b0: strb r0, [r4, #0x43f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1087)?)?;
    // 0x366b4: mov r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,17)?)?;
    // 0x366b8: strb r0, [r4, #0x440]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1088)?)?;
    // 0x366bc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x366c0: strb r6, [r4, #0x446]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1094)?)?;
    // 0x366c4: strb r6, [r4, #0x445]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1093)?)?;
    // 0x366c8: strb r6, [r4, #0x442]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1090)?)?;
    // 0x366cc: strb r6, [r0, #0x441]!
    c.emit(encode_byte_preindexed_writeback(C::Al,false,R::new(6)?,R::R0,1089)?)?;
    // 0x366d0: strb r6, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R0,3)?)?;
    // 0x366d4: strb r6, [r0, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R0,2)?)?;
    // 0x366d8: mov r0, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,21)?)?;
    // 0x366dc: strb r9, [r4, #0x447]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1095)?)?;
    // 0x366e0: strb r3, [r4, #0x448]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1096)?)?;
    // 0x366e4: mov r3, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,11)?)?;
    // 0x366e8: strb r0, [r4, #0x449]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1097)?)?;
    // 0x366ec: mov r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // 0x366f0: strb r12, [r4, #0x44a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(4)?,1098)?)?;
    // 0x366f4: mov r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,2)?)?;
    // 0x366f8: strb r0, [r4, #0x44b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1099)?)?;
    // 0x366fc: strb r8, [r4, #0x44c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(4)?,1100)?)?;
    // 0x36700: strb r1, [r4, #0x44d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1101)?)?;
    // 0x36704: strb r2, [r4, #0x44e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1102)?)?;
    // 0x36708: strb r5, [r4, #0x44f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1103)?)?;
    // 0x3670c: mov r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,8)?)?;
    // 0x36710: mov r1, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,9)?)?;
    // 0x36714: mov r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,18)?)?;
    // 0x36718: strb r10, [r4, #0x450]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(4)?,1104)?)?;
    // 0x3671c: mov r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,12)?)?;
    // 0x36720: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x36724: mov r8, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,19)?)?;
    // 0x36728: strb r5, [r4, #0x451]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1105)?)?;
    // 0x3672c: strb r1, [r4, #0x452]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1106)?)?;
    // 0x36730: strb r7, [r4, #0x453]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1107)?)?;
    // 0x36734: strb lr, [r4, #0x454]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(4)?,1108)?)?;
    // 0x36738: strb r0, [r4, #0x455]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1109)?)?;
    // 0x3673c: mov r0, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,19)?)?;
    // 0x36740: strb r3, [r4, #0x456]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1110)?)?;
    // 0x36744: strb r10, [r4, #0x457]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(4)?,1111)?)?;
    // 0x36748: mov lr, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,3)?)?;
    // 0x3674c: mov r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,5)?)?;
    // 0x36750: strb r0, [r4, #0x458]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1112)?)?;
    // 0x36754: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x36758: strb r0, [r4, #0x459]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1113)?)?;
    // 0x3675c: strb lr, [r4, #0x45a]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(4)?,1114)?)?;
    // 0x36760: strb r7, [r4, #0x45b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1115)?)?;
    // 0x36764: strb r2, [r4, #0x45c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1116)?)?;
    // 0x36768: strb r6, [r4, #0x45d]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1117)?)?;
    // 0x3676c: strb r9, [r4, #0x45e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1118)?)?;
    // 0x36770: strb r7, [r4, #0x45f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1119)?)?;
    // 0x36774: mov r7, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,21)?)?;
    // 0x36778: strb r7, [r4, #0x460]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1120)?)?;
    // 0x3677c: mov r7, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,22)?)?;
    // 0x36780: strb r12, [r4, #0x461]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(4)?,1121)?)?;
    // 0x36784: strb r7, [r4, #0x462]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1122)?)?;
    // 0x36788: strb r5, [r4, #0x463]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1123)?)?;
    // 0x3678c: strb r1, [r4, #0x464]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1124)?)?;
    // 0x36790: mov r1, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10)?)?;
    // 0x36794: mov r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,7)?)?;
    // 0x36798: mov r5, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,11)?)?;
    // 0x3679c: strb r1, [r4, #0x465]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1125)?)?;
    // 0x367a0: mov r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // 0x367a4: strb r7, [r4, #0x466]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1126)?)?;
    // 0x367a8: strb r3, [r4, #0x467]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1127)?)?;
    // 0x367ac: strb r10, [r4, #0x468]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(4)?,1128)?)?;
    // 0x367b0: mov r10, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,4)?)?;
    // 0x367b4: mov r3, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,18)?)?;
    // 0x367b8: strb r1, [r4, #0x469]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1129)?)?;
    // 0x367bc: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x367c0: strb r1, [r4, #0x46a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1130)?)?;
    // 0x367c4: mov r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x367c8: strb r1, [r4, #0x46b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1131)?)?;
    // 0x367cc: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x367d0: strb r1, [r4, #0x46c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1132)?)?;
    // 0x367d4: mov r1, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,17)?)?;
    // 0x367d8: strb r1, [r4, #0x46d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1133)?)?;
    // 0x367dc: strb r3, [r4, #0x46e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1134)?)?;
    // 0x367e0: strb r8, [r4, #0x46f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(4)?,1135)?)?;
    // 0x367e4: strb r0, [r4, #0x470]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1136)?)?;
    // 0x367e8: strb lr, [r4, #0x471]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(4)?,1137)?)?;
    // 0x367ec: strb r10, [r4, #0x472]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(4)?,1138)?)?;
    // 0x367f0: strb r2, [r4, #0x473]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1139)?)?;
    // 0x367f4: strb r6, [r4, #0x474]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1140)?)?;
    // 0x367f8: strb r9, [r4, #0x475]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1141)?)?;
    // 0x367fc: strb r2, [r4, #0x476]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1142)?)?;
    c.check(0x36800)?;
    // Controller setup block: codec_defaults, runtime [0x36800, 0x36c00).
    // 0x36800: mov r2, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,21)?)?;
    // 0x36804: mov r3, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10)?)?;
    // 0x36808: mov lr, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,3)?)?;
    // 0x3680c: strb r2, [r4, #0x477]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1143)?)?;
    // 0x36810: mov r2, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,22)?)?;
    // 0x36814: strb r12, [r4, #0x478]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(4)?,1144)?)?;
    // 0x36818: mov r12, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,20)?)?;
    // 0x3681c: strb r2, [r4, #0x479]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1145)?)?;
    // 0x36820: mov r2, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,18)?)?;
    // 0x36824: strb r2, [r4, #0x47a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1146)?)?;
    // 0x36828: strb r8, [r4, #0x47b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(4)?,1147)?)?;
    // 0x3682c: strb r0, [r4, #0x47c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1148)?)?;
    // 0x36830: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x36834: strb r1, [r4, #0x47d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1149)?)?;
    // 0x36838: strb r7, [r4, #0x47e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1150)?)?;
    // 0x3683c: mov r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x36840: add r2, r4, #1200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,1200)?)?;
    // 0x36844: mov r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,3)?)?;
    // 0x36848: mov r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,5)?)?;
    // 0x3684c: strb r0, [r4, #0x47f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1151)?)?;
    // 0x36850: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x36854: strb r0, [r4, #0x480]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1152)?)?;
    // 0x36858: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x3685c: strb r3, [r4, #0x481]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1153)?)?;
    // 0x36860: strb r5, [r4, #0x482]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1154)?)?;
    // 0x36864: mov r5, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,20)?)?;
    // 0x36868: strb r0, [r4, #0x483]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1155)?)?;
    // 0x3686c: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x36870: strb r0, [r4, #0x484]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1156)?)?;
    // 0x36874: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x36878: strb r0, [r4, #0x485]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1157)?)?;
    // 0x3687c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x36880: strb r1, [r4, #0x486]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1158)?)?;
    // 0x36884: strb r0, [r4, #0x487]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1159)?)?;
    // 0x36888: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x3688c: strb lr, [r4, #0x488]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(4)?,1160)?)?;
    // 0x36890: strb r10, [r4, #0x489]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(4)?,1161)?)?;
    // 0x36894: strb r0, [r4, #0x48a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1162)?)?;
    // 0x36898: mov r0, #117
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,117)?)?;
    // 0x3689c: str r9, [r4, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1196)?)?;
    // 0x368a0: stm r2, {r0, r1, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit() | R::R1.bit() | R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x368a4: mov r0, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,100)?)?;
    // 0x368a8: str r6, [r4, #0x4bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1212)?)?;
    // 0x368ac: mov r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // 0x368b0: add r1, r4, #1232
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,1232)?)?;
    // 0x368b4: str r0, [r4, #0x4c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1216)?)?;
    // 0x368b8: mov r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,127)?)?;
    // 0x368bc: str r12, [r4, #0x4c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,1220)?)?;
    // 0x368c0: str r6, [r4, #0x4c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1224)?)?;
    // 0x368c4: str r6, [r4, #0x4cc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1228)?)?;
    // 0x368c8: stm r1, {r0, r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit() | R::R2.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x368cc: add r0, r4, #220
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,220)?)?;
    // 0x368d0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x368d4: mov r2, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,52)?)?;
    // 0x368d8: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x368dc: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x336f8
    c.bl(0x31ec)?;
    // 0x368e0: mvn r0, #-67108864
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,4227858432)?)?;
    // 0x368e4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x368e8: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x368ec: str r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1296)?)?;
    // 0x368f0: add r0, r4, #388
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,388)?)?;
    // 0x368f4: str r7, [r4, #0x514]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1300)?)?;
    // 0x368f8: str r6, [r4, #0x1cc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,460)?)?;
    // 0x368fc: str r9, [r4, #0x580]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1408)?)?;
    // 0x36900: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x36904: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x33720
    c.bl(0x31ec)?;
    // 0x36908: add r0, r4, #428
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,428)?)?;
    // 0x3690c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x36910: mov r2, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,44)?)?;
    // 0x36914: str r7, [r4, #0x5a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1440)?)?;
    // 0x36918: str r8, [r4, #0x5a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1444)?)?;
    // 0x3691c: str r9, [r4, #0x5a8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1448)?)?;
    // 0x36920: mov r7, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,10)?)?;
    // 0x36924: str r6, [r4, #0x518]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1304)?)?;
    // 0x36928: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x3692c: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x33748
    c.bl(0x31ec)?;
    // 0x36930: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x36934: str r9, [r4, #0x51c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1308)?)?;
    // 0x36938: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3693c: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x36940: str r0, [r4, #0x520]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1312)?)?;
    // 0x36944: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x36948: str r8, [r4, #0x524]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1316)?)?;
    // 0x3694c: str r10, [r4, #0x528]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,1320)?)?;
    // 0x36950: str r0, [r4, #0x52c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1324)?)?;
    // 0x36954: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x36958: str r0, [r4, #0x530]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1328)?)?;
    // 0x3695c: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x36960: str r0, [r4, #0x534]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1332)?)?;
    // 0x36964: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x36968: str r0, [r4, #0x538]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1336)?)?;
    // 0x3696c: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x36970: str r0, [r4, #0x53c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1340)?)?;
    // 0x36974: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x36978: str r7, [r4, #0x540]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1344)?)?;
    // 0x3697c: str r0, [r4, #0x544]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1348)?)?;
    // 0x36980: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x36984: str r0, [r4, #0x548]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1352)?)?;
    // 0x36988: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x3698c: str r0, [r4, #0x54c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1356)?)?;
    // 0x36990: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x36994: str r0, [r4, #0x550]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1360)?)?;
    // 0x36998: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x3699c: str r0, [r4, #0x554]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1364)?)?;
    // 0x369a0: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x369a4: str r0, [r4, #0x558]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1368)?)?;
    // 0x369a8: mov r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,17)?)?;
    // 0x369ac: str r0, [r4, #0x55c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1372)?)?;
    // 0x369b0: mov r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,18)?)?;
    // 0x369b4: str r0, [r4, #0x560]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1376)?)?;
    // 0x369b8: mov r0, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,19)?)?;
    // 0x369bc: str r0, [r4, #0x564]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1380)?)?;
    // 0x369c0: mov r0, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,21)?)?;
    // 0x369c4: str r5, [r4, #0x568]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1384)?)?;
    // 0x369c8: str r0, [r4, #0x56c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1388)?)?;
    // 0x369cc: mov r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // 0x369d0: str r0, [r4, #0x570]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1392)?)?;
    // 0x369d4: mov r0, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,23)?)?;
    // 0x369d8: str r0, [r4, #0x574]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1396)?)?;
    // 0x369dc: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // 0x369e0: str r0, [r4, #0x578]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1400)?)?;
    // 0x369e4: mov r0, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,25)?)?;
    // 0x369e8: str r0, [r4, #0x57c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1404)?)?;
    // 0x369ec: add r0, r4, #472
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,472)?)?;
    // 0x369f0: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x369f4: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x33810
    c.bl(0x31ec)?;
    // 0x369f8: mov r0, #134217728
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,134217728)?)?;
    // 0x369fc: str r10, [r4, #0x5f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,1520)?)?;
    // 0x36a00: str r10, [r4, #0x5f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,1524)?)?;
    // 0x36a04: str r10, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,1528)?)?;
    // 0x36a08: str r10, [r4, #0x5fc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,1532)?)?;
    // 0x36a0c: str r10, [r4, #0x600]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,1536)?)?;
    // 0x36a10: str r10, [r4, #0x604]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,1540)?)?;
    // 0x36a14: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x36a18: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x36a1c: str r0, [r4, #0x60c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1548)?)?;
    // 0x36a20: add r0, r4, #1552
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,1552)?)?;
    // 0x36a24: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x33840
    c.bl(0x31ec)?;
    // 0x36a28: mov r0, #1584
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1584)?)?;
    // 0x36a2c: strb r9, [r4, #0x61f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1567)?)?;
    // 0x36a30: strb r9, [r4, #0x620]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1568)?)?;
    // 0x36a34: strb r9, [r4, #0x621]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1569)?)?;
    // 0x36a38: strb r6, [r4, #0x622]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1570)?)?;
    // 0x36a3c: strb r6, [r4, #0x623]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1571)?)?;
    // 0x36a40: str r6, [r4, #0x30c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,780)?)?;
    // 0x36a44: strb r9, [r4, #0x624]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1572)?)?;
    // 0x36a48: str r6, [r4, #0x608]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1544)?)?;
    // 0x36a4c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x36a50: mov r2, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,25)?)?;
    // 0x36a54: strh r6, [r4, r0]
    c.emit(encode_halfword_register(C::Al,false,R::new(6)?,R::new(4)?,R::R0,false)?)?;
    // 0x36a58: add r0, r4, #568
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,568)?)?;
    // 0x36a5c: strb r6, [r4, #0x632]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1586)?)?;
    // 0x36a60: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // 0x36a64: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x33880
    c.bl(0x31ec)?;
    // 0x36a68: mov r0, #82
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,82)?)?;
    // 0x36a6c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x36a70: mov lr, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,2)?)?;
    // 0x36a74: mov r2, #230
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,230)?)?;
    // 0x36a78: mov r12, r4
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(4)?))?;
    // 0x36a7c: mov r3, #85
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,85)?)?;
    // 0x36a80: orr r0, r0, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1536)?)?;
    // 0x36a84: orr r3, r3, #21760
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,21760)?)?;
    // 0x36a88: strh r1, [r4, r0]
    c.emit(encode_halfword_register(C::Al,false,R::R1,R::new(4)?,R::R0,false)?)?;
    // 0x36a8c: mov r1, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,616)?)?;
    // 0x36a90: mov r0, #220
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,220)?)?;
    // 0x36a94: strb r6, [r4, #0x654]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1620)?)?;
    // 0x36a98: strb r9, [r4, #0x655]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1621)?)?;
    // 0x36a9c: strb lr, [r4, #0x656]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(4)?,1622)?)?;
    // 0x36aa0: str r6, [r4, #0x628]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1576)?)?;
    // 0x36aa4: str r6, [r4, #0x62c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1580)?)?;
    // 0x36aa8: str r6, [r4, #0x658]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1624)?)?;
    // 0x36aac: str r6, [r4, #0x65c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1628)?)?;
    // 0x36ab0: strb r6, [r4, #0x660]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1632)?)?;
    // 0x36ab4: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x36ab8: str r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1636)?)?;
    // 0x36abc: mov r0, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,100)?)?;
    // 0x36ac0: strh r2, [r12, r1]!
    c.emit(encode_halfword_register(C::Al,false,R::R2,R::R12,R::R1,true)?)?;
    // 0x36ac4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x36ac8: strb r6, [r4, #0x625]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1573)?)?;
    // 0x36acc: strb r6, [r4, #0x698]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1688)?)?;
    // 0x36ad0: strb r6, [r4, #0x170]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,368)?)?;
    // 0x36ad4: strb r6, [r1, #0x673]!
    c.emit(encode_byte_preindexed_writeback(C::Al,false,R::new(6)?,R::R1,1651)?)?;
    // 0x36ad8: strb r6, [r1, #-0x4]!
    c.emit(encode_byte_preindexed_writeback(C::Al,false,R::new(6)?,R::R1,-4)?)?;
    // 0x36adc: strb r6, [r1, #-0x4]!
    c.emit(encode_byte_preindexed_writeback(C::Al,false,R::new(6)?,R::R1,-4)?)?;
    // 0x36ae0: strb r6, [r1, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,11)?)?;
    // 0x36ae4: strb r6, [r1, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,10)?)?;
    // 0x36ae8: strb r6, [r1, #0x9]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,9)?)?;
    // 0x36aec: strb r6, [r1, #0x7]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,7)?)?;
    // 0x36af0: strb r6, [r1, #0x6]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,6)?)?;
    // 0x36af4: strb r6, [r1, #0x5]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,5)?)?;
    // 0x36af8: strb r6, [r1, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,3)?)?;
    // 0x36afc: strb r6, [r1, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,2)?)?;
    // 0x36b00: strb r6, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,1)?)?;
    // 0x36b04: strb r6, [r1, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,12)?)?;
    // 0x36b08: ldr r2, [pc, #0x504]        @ 0x1b88 <entry+0x1b88>
    c.literal_cond(C::Al,R::R2,0x37014)?;
    // 0x36b0c: str r2, [r1, #0xd]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,13)?)?;
    // 0x36b10: str r2, [r1, #0x11]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,17)?)?;
    // 0x36b14: str r2, [r1, #0x15]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,21)?)?;
    // 0x36b18: str r2, [r1, #0x19]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,25)?)?;
    // 0x36b1c: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x36b20: str r3, [r1, #0x1d]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,29)?)?;
    // 0x36b24: strb r6, [r1, #0x21]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,33)?)?;
    // 0x36b28: str r6, [r1, #0x25]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,37)?)?;
    // 0x36b2c: str r6, [r1, #0x29]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,41)?)?;
    // 0x36b30: str r6, [r1, #-0x497]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,-1175)?)?;
    // 0x36b34: str r9, [r1, #-0x493]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,-1171)?)?;
    // 0x36b38: str r9, [r1, #-0x48f]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,-1167)?)?;
    // 0x36b3c: str r9, [r1, #-0x48b]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,-1163)?)?;
    // 0x36b40: str r9, [r1, #-0x487]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,-1159)?)?;
    // 0x36b44: str r9, [r1, #-0x483]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,-1155)?)?;
    // 0x36b48: str r9, [r1, #-0x47f]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,-1151)?)?;
    // 0x36b4c: str r9, [r1, #-0x47b]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,-1147)?)?;
    // 0x36b50: str r0, [r1, #-0x477]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,-1143)?)?;
    // 0x36b54: strb r6, [r1, #-0x500]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,-1280)?)?;
    // 0x36b58: str r6, [r1, #0x135]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,309)?)?;
    // 0x36b5c: strb r6, [r4, #0x24f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,591)?)?;
    // 0x36b60: strb r6, [r1, #-0x552]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,-1362)?)?;
    // 0x36b64: strb r2, [r1, #-0x553]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,-1363)?)?;
    // 0x36b68: str r9, [r1, #-0x5df]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,-1503)?)?;
    // 0x36b6c: str r9, [r1, #-0x5db]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,-1499)?)?;
    // 0x36b70: strb r9, [r1, #-0x533]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,-1331)?)?;
    // 0x36b74: strb r6, [r1, #-0x531]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,-1329)?)?;
    // 0x36b78: strb r9, [r1, #-0x532]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::R1,-1330)?)?;
    // 0x36b7c: str r6, [r1, #-0x643]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,-1603)?)?;
    // 0x36b80: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x36b84: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x36b88: beq 0x36e54 <entry+0x19c8>  @ imm = #0x2c4
    c.b(C::Eq,0x36e54)?;
    // 0x36b8c: ldr r6, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,12)?)?;
    // 0x36b90: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x36b94: bne 0x36f94 <entry+0x1b08>  @ imm = #0x3f8
    c.b(C::Ne,0x36f94)?;
    // 0x36b98: ldr r1, [pc, #0x478]        @ 0x1b8c <entry+0x1b8c>
    c.literal_cond(C::Al,R::R1,0x37018)?;
    // 0x36b9c: mov r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8192)?)?;
    // 0x36ba0: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x36ba4: mov lr, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,5)?)?;
    // 0x36ba8: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x36bac: mov r5, #47
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,47)?)?;
    // 0x36bb0: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x36bb4: mov r8, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,6)?)?;
    // 0x36bb8: mov r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x36bbc: mov r10, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,10)?)?;
    // 0x36bc0: str r1, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,100)?)?;
    // 0x36bc4: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x36bc8: strh r1, [r4, #116]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(4)?,116)?)?;
    // 0x36bcc: str r2, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,4)?)?;
    // 0x36bd0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x36bd4: strh r1, [r4, #120]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(4)?,120)?)?;
    // 0x36bd8: strh r1, [r4, #118]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(4)?,118)?)?;
    // 0x36bdc: strb r9, [r4, #0x164]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,356)?)?;
    // 0x36be0: strb r9, [r4, #0x9c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,156)?)?;
    // 0x36be4: strb r9, [r4, #0xa8]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,168)?)?;
    // 0x36be8: mov r1, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,21)?)?;
    // 0x36bec: strb r2, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,350)?)?;
    // 0x36bf0: strb lr, [r4, #0x24d]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(4)?,589)?)?;
    // 0x36bf4: strb lr, [r4, #0x24e]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(4)?,590)?)?;
    // 0x36bf8: strb r9, [r4, #0x24f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,591)?)?;
    // 0x36bfc: str r3, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,412)?)?;
    c.check(0x36c00)?;
    // Controller setup block: hardware_fields, runtime [0x36c00, 0x36fd8).
    // 0x36c00: str r3, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,388)?)?;
    // 0x36c04: str r5, [r4, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,556)?)?;
    // 0x36c08: str r5, [r4, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,560)?)?;
    // 0x36c0c: str r5, [r4, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,564)?)?;
    // 0x36c10: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x36c14: str r0, [r4, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,568)?)?;
    // 0x36c18: add r0, r4, #984
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,984)?)?;
    // 0x36c1c: strh r5, [r6]
    c.emit(arm32::store_halfword(C::Al,R::new(5)?,R::new(6)?,0)?)?;
    // 0x36c20: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x36c24: str r2, [r4, #0x330]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,816)?)?;
    // 0x36c28: str r3, [r4, #0x334]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,820)?)?;
    // 0x36c2c: str r2, [r4, #0x338]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,824)?)?;
    // 0x36c30: str r3, [r4, #0x33c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,828)?)?;
    // 0x36c34: str r2, [r4, #0x340]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,832)?)?;
    // 0x36c38: str r3, [r4, #0x344]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,836)?)?;
    // 0x36c3c: str r2, [r4, #0x348]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,840)?)?;
    // 0x36c40: str r3, [r4, #0x34c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,844)?)?;
    // 0x36c44: str r2, [r4, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,848)?)?;
    // 0x36c48: str r3, [r4, #0x354]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,852)?)?;
    // 0x36c4c: str r2, [r4, #0x358]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,856)?)?;
    // 0x36c50: str r3, [r4, #0x35c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,860)?)?;
    // 0x36c54: str r2, [r4, #0x360]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,864)?)?;
    // 0x36c58: str r3, [r4, #0x364]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,868)?)?;
    // 0x36c5c: str r2, [r4, #0x368]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,872)?)?;
    // 0x36c60: str r3, [r4, #0x36c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,876)?)?;
    // 0x36c64: str r2, [r4, #0x370]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,880)?)?;
    // 0x36c68: str r3, [r4, #0x374]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,884)?)?;
    // 0x36c6c: str r2, [r4, #0x378]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,888)?)?;
    // 0x36c70: str r3, [r4, #0x37c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,892)?)?;
    // 0x36c74: str r2, [r4, #0x380]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,896)?)?;
    // 0x36c78: str r3, [r4, #0x384]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,900)?)?;
    // 0x36c7c: str r2, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,904)?)?;
    // 0x36c80: str r3, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,908)?)?;
    // 0x36c84: str r2, [r4, #0x390]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,912)?)?;
    // 0x36c88: str r3, [r4, #0x394]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,916)?)?;
    // 0x36c8c: str r2, [r4, #0x398]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,920)?)?;
    // 0x36c90: str r3, [r4, #0x39c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,924)?)?;
    // 0x36c94: str r2, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,928)?)?;
    // 0x36c98: str r3, [r4, #0x3a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,932)?)?;
    // 0x36c9c: str r2, [r4, #0x3a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,936)?)?;
    // 0x36ca0: str r3, [r4, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,940)?)?;
    // 0x36ca4: mov r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,8)?)?;
    // 0x36ca8: str r6, [r4, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,432)?)?;
    // 0x36cac: str r6, [r4, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,436)?)?;
    // 0x36cb0: str r6, [r4, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,440)?)?;
    // 0x36cb4: str r6, [r4, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,444)?)?;
    // 0x36cb8: str r3, [r4, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,448)?)?;
    // 0x36cbc: str r3, [r4, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,956)?)?;
    // 0x36cc0: str r3, [r4, #0x3c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,960)?)?;
    // 0x36cc4: stm r0, {r3, r6, r8, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::new(3)?.bit() | R::new(6)?.bit() | R::new(8)?.bit() | R::new(9)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x36cc8: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x36ccc: str r5, [r4, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1000)?)?;
    // 0x36cd0: str r0, [r4, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1004)?)?;
    // 0x36cd4: str r0, [r4, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1008)?)?;
    // 0x36cd8: str r5, [r4, #0x3f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1012)?)?;
    // 0x36cdc: strb r7, [r4, #0x431]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1073)?)?;
    // 0x36ce0: strb r1, [r4, #0x432]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1074)?)?;
    // 0x36ce4: mov r1, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    // 0x36ce8: strb r2, [r4, #0x433]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1075)?)?;
    // 0x36cec: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x36cf0: strb r1, [r4, #0x434]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1076)?)?;
    // 0x36cf4: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x36cf8: strb r1, [r4, #0x435]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1077)?)?;
    // 0x36cfc: mov r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
    // 0x36d00: strb r5, [r4, #0x436]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1078)?)?;
    // 0x36d04: mov r5, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,9)?)?;
    // 0x36d08: strb r1, [r4, #0x437]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1079)?)?;
    // 0x36d0c: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // 0x36d10: strb r1, [r4, #0x438]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1080)?)?;
    // 0x36d14: mov r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // 0x36d18: strb r1, [r4, #0x439]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1081)?)?;
    // 0x36d1c: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x36d20: strb r5, [r4, #0x43a]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1082)?)?;
    // 0x36d24: strb r10, [r4, #0x43b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(4)?,1083)?)?;
    // 0x36d28: strb r1, [r4, #0x43c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1084)?)?;
    // 0x36d2c: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x36d30: strb r0, [r4, #0x43d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1085)?)?;
    // 0x36d34: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x36d38: strb r1, [r4, #0x43e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1086)?)?;
    // 0x36d3c: mov r1, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,17)?)?;
    // 0x36d40: strb r1, [r4, #0x43f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1087)?)?;
    // 0x36d44: mov r1, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,18)?)?;
    // 0x36d48: strb r1, [r4, #0x440]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1088)?)?;
    // 0x36d4c: mov r1, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,19)?)?;
    // 0x36d50: strb r1, [r4, #0x441]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1089)?)?;
    // 0x36d54: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x36d58: strb r1, [r4, #0x442]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1090)?)?;
    // 0x36d5c: strb r6, [r4, #0x443]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1091)?)?;
    // 0x36d60: strb lr, [r4, #0x444]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(4)?,1092)?)?;
    // 0x36d64: strb r8, [r4, #0x445]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(4)?,1093)?)?;
    // 0x36d68: str r9, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,1200)?)?;
    // 0x36d6c: str r3, [r4, #0x4b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1204)?)?;
    // 0x36d70: str lr, [r4, #0x4b8]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,1208)?)?;
    // 0x36d74: str r10, [r4, #0x4bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,1212)?)?;
    // 0x36d78: mov r1, #30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,30)?)?;
    // 0x36d7c: mov r10, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,15)?)?;
    // 0x36d80: str lr, [r4, #0x4c0]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,1216)?)?;
    // 0x36d84: str r10, [r4, #0x4c4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,1220)?)?;
    // 0x36d88: str r1, [r4, #0x4c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1224)?)?;
    // 0x36d8c: str r0, [r4, #0x4cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1228)?)?;
    // 0x36d90: str lr, [r4, #0x4d0]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,1232)?)?;
    // 0x36d94: str r10, [r4, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,1236)?)?;
    // 0x36d98: str r1, [r4, #0x4d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1240)?)?;
    // 0x36d9c: str r0, [r4, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1244)?)?;
    // 0x36da0: add r0, r4, #1264
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,1264)?)?;
    // 0x36da4: str lr, [r4, #0x4e0]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,1248)?)?;
    // 0x36da8: str r5, [r4, #0x4e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1256)?)?;
    // 0x36dac: str r7, [r4, #0x4ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1260)?)?;
    // 0x36db0: stm r0, {r2, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit() | R::new(5)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x36db4: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x36db8: str r2, [r4, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1276)?)?;
    // 0x36dbc: str r5, [r4, #0x500]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1280)?)?;
    // 0x36dc0: str r7, [r4, #0x504]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1284)?)?;
    // 0x36dc4: str r2, [r4, #0x508]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1288)?)?;
    // 0x36dc8: str r6, [r4, #0x50c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1292)?)?;
    // 0x36dcc: str r7, [r4, #0x514]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1300)?)?;
    // 0x36dd0: str r0, [r4, #0x58c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1420)?)?;
    // 0x36dd4: str r0, [r4, #0x594]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1428)?)?;
    // 0x36dd8: add r0, r4, #1456
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,1456)?)?;
    // 0x36ddc: str r3, [r4, #0x5a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1440)?)?;
    // 0x36de0: str r6, [r4, #0x5a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1444)?)?;
    // 0x36de4: str r6, [r4, #0x5ac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1452)?)?;
    // 0x36de8: stm r0, {r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::new(3)?.bit() | R::new(6)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x36dec: mov r0, #250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,250)?)?;
    // 0x36df0: str r6, [r4, #0x5c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1472)?)?;
    // 0x36df4: str r7, [r4, #0x5c4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1476)?)?;
    // 0x36df8: str r6, [r4, #0x5cc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1484)?)?;
    // 0x36dfc: str r7, [r4, #0x5d0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1488)?)?;
    // 0x36e00: str r8, [r4, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1496)?)?;
    // 0x36e04: str r7, [r4, #0x5dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1500)?)?;
    // 0x36e08: str r2, [r4, #0x5e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1504)?)?;
    // 0x36e0c: str r8, [r4, #0x5e4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1508)?)?;
    // 0x36e10: str r7, [r4, #0x5e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1512)?)?;
    // 0x36e14: str r2, [r4, #0x5ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1516)?)?;
    // 0x36e18: str r8, [r4, #0x5f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1520)?)?;
    // 0x36e1c: str r7, [r4, #0x5f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1524)?)?;
    // 0x36e20: str r2, [r4, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1528)?)?;
    // 0x36e24: str r8, [r4, #0x5fc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,1532)?)?;
    // 0x36e28: str r7, [r4, #0x600]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,1536)?)?;
    // 0x36e2c: str r2, [r4, #0x604]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1540)?)?;
    // 0x36e30: strb r9, [r4, #0x60f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1551)?)?;
    // 0x36e34: strb r9, [r4, #0x61f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1567)?)?;
    // 0x36e38: strb r9, [r4, #0x620]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1568)?)?;
    // 0x36e3c: strb r9, [r4, #0x621]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1569)?)?;
    // 0x36e40: str r0, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1636)?)?;
    // 0x36e44: mov r0, #260
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,260)?)?;
    // 0x36e48: strh r0, [r12]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R12,0)?)?;
    // 0x36e4c: strb r9, [r4, #0x24f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,591)?)?;
    // 0x36e50: b 0x36f94 <entry+0x1b08>  @ imm = #0x13c
    c.b(C::Al,0x36f94)?;
    // 0x36e54: mov r0, #576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,576)?)?;
    // 0x36e58: str r9, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,24)?)?;
    // 0x36e5c: strb r9, [r4, #0x140]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,320)?)?;
    // 0x36e60: strb r9, [r4, #0x144]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,324)?)?;
    // 0x36e64: str r6, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,300)?)?;
    // 0x36e68: mov r1, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,18)?)?;
    // 0x36e6c: mov r3, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,127)?)?;
    // 0x36e70: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x36e74: mov r5, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,5)?)?;
    // 0x36e78: orr r0, r0, #999424
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,999424)?)?;
    // 0x36e7c: str r0, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,100)?)?;
    // 0x36e80: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x36e84: strh r1, [r4, #116]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(4)?,116)?)?;
    // 0x36e88: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x36e8c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x36e90: str r6, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,12)?)?;
    // 0x36e94: strh r1, [r4, #120]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(4)?,120)?)?;
    // 0x36e98: strh r1, [r4, #118]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(4)?,118)?)?;
    // 0x36e9c: strb r6, [r4, #0x164]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,356)?)?;
    // 0x36ea0: strb r6, [r4, #0x9c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,156)?)?;
    // 0x36ea4: str r9, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,412)?)?;
    // 0x36ea8: mov r1, #117
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,117)?)?;
    // 0x36eac: str r0, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,404)?)?;
    // 0x36eb0: str r6, [r4, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,564)?)?;
    // 0x36eb4: str r3, [r4, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,568)?)?;
    // 0x36eb8: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x36ebc: strh r2, [r0]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,0)?)?;
    // 0x36ec0: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x36ec4: strb r9, [r4, #0x2ae]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,686)?)?;
    // 0x36ec8: str r7, [r4, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,432)?)?;
    // 0x36ecc: str r5, [r4, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,436)?)?;
    // 0x36ed0: str r9, [r4, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,452)?)?;
    // 0x36ed4: str r6, [r4, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,996)?)?;
    // 0x36ed8: str r6, [r4, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1000)?)?;
    // 0x36edc: str r6, [r4, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1004)?)?;
    // 0x36ee0: str r6, [r4, #0x3fc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1020)?)?;
    // 0x36ee4: str r6, [r4, #0x400]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1024)?)?;
    // 0x36ee8: str r0, [r4, #0x404]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1028)?)?;
    // 0x36eec: str r0, [r4, #0x408]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1032)?)?;
    // 0x36ef0: str r0, [r4, #0x40c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1036)?)?;
    // 0x36ef4: str r0, [r4, #0x410]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1040)?)?;
    // 0x36ef8: str r0, [r4, #0x414]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1044)?)?;
    // 0x36efc: str r1, [r4, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1200)?)?;
    // 0x36f00: mov r1, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,40)?)?;
    // 0x36f04: str r3, [r4, #0x4c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1216)?)?;
    // 0x36f08: str r1, [r4, #0x4c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1220)?)?;
    // 0x36f0c: mov r1, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10)?)?;
    // 0x36f10: str r1, [r4, #0x4c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1224)?)?;
    // 0x36f14: str r6, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1296)?)?;
    // 0x36f18: str r6, [r4, #0x580]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1408)?)?;
    // 0x36f1c: str r5, [r4, #0x5ac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1452)?)?;
    // 0x36f20: str r0, [r4, #0x5b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1456)?)?;
    // 0x36f24: strb lr, [r4, #0x60c]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(4)?,1548)?)?;
    // 0x36f28: strb r2, [r4, #0x60f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1551)?)?;
    // 0x36f2c: strb r7, [r4, #0x610]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1552)?)?;
    // 0x36f30: strb r5, [r4, #0x611]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1553)?)?;
    // 0x36f34: strb r5, [r4, #0x612]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1554)?)?;
    // 0x36f38: strb r0, [r4, #0x613]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1555)?)?;
    // 0x36f3c: strb r5, [r4, #0x614]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1556)?)?;
    // 0x36f40: strb r5, [r4, #0x615]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1557)?)?;
    // 0x36f44: strb r0, [r4, #0x616]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1558)?)?;
    // 0x36f48: strb r7, [r4, #0x617]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1559)?)?;
    // 0x36f4c: strb r0, [r4, #0x618]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1560)?)?;
    // 0x36f50: strb lr, [r4, #0x619]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::new(4)?,1561)?)?;
    // 0x36f54: strb r6, [r4, #0x61a]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1562)?)?;
    // 0x36f58: strb r5, [r4, #0x61b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1563)?)?;
    // 0x36f5c: strb r0, [r4, #0x61c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1564)?)?;
    // 0x36f60: strb r0, [r4, #0x61d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1565)?)?;
    // 0x36f64: strb r6, [r4, #0x61e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1566)?)?;
    // 0x36f68: strb r6, [r4, #0x61f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1567)?)?;
    // 0x36f6c: strb r6, [r4, #0x620]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,1568)?)?;
    // 0x36f70: strb r9, [r4, #0x622]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1570)?)?;
    // 0x36f74: strb r9, [r4, #0x623]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1571)?)?;
    // 0x36f78: strb r9, [r4, #0x650]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1616)?)?;
    // 0x36f7c: str r6, [r4, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1636)?)?;
    // 0x36f80: strh r6, [r12]
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::R12,0)?)?;
    // 0x36f84: strb r9, [r4, #0x698]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,1688)?)?;
    // 0x36f88: strb r6, [r4, #0x24f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,591)?)?;
    // 0x36f8c: strb r9, [r4, #0x13a]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(9)?,R::new(4)?,314)?)?;
    // 0x36f90: str r9, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,40)?)?;
    // 0x36f94: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x36f98: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x36f9c: strb r0, [r4, #0x6c9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1737)?)?;
    // 0x36fa0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x36fa4: strb r0, [r4, #0x69d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1693)?)?;
    // 0x36fa8: strb r0, [r4, #0x6ad]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1709)?)?;
    // 0x36fac: strb r0, [r4, #0x6ae]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1710)?)?;
    // 0x36fb0: strb r1, [r4, #0x2b6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,694)?)?;
    // 0x36fb4: str r0, [r4, #0x790]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1936)?)?;
    // 0x36fb8: str r0, [r4, #0x79c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1948)?)?;
    // 0x36fbc: strb r0, [r4, #0x699]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1689)?)?;
    // 0x36fc0: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x36fc4: strb r0, [r4, #0x69a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1690)?)?;
    // 0x36fc8: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x36fcc: strb r0, [r4, #0x69b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1691)?)?;
    // 0x36fd0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x36fd4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x36fd8)?;
    for word in CONTROLLER_INITIALIZATION_LITERALS {c.emit(word)?;}
    c.check(END)
}

#[cfg(test)]mod tests {
 use super::*;
 #[test]fn controller_defaults_exact_layout_and_literal_array(){
    let mut c=Code::new(START);emit(&mut c).unwrap();assert_eq!(c.at(),END);
    for (i,expect) in CONTROLLER_INITIALIZATION_LITERALS.iter().enumerate(){
       let offset=0x36fd8+i*4-START;
       assert_eq!(u32::from_le_bytes(c.bytes[offset..offset+4].try_into().unwrap()),*expect);
    }
    assert_eq!(encode_halfword_register(C::Al,false,R::new(6).unwrap(),R::new(4).unwrap(),R::R0,false).unwrap(),0xe184_60b0);
    assert_eq!(encode_halfword_register(C::Al,false,R::R2,R::R1,R::R0,true).unwrap(),0xe1a1_20b0);
    assert_eq!(encode_byte_preindexed_writeback(C::Al,false,R::new(6).unwrap(),R::R0,0x441).unwrap(),0xe5e0_6441);
    assert!(encode_byte_preindexed_writeback(C::Al,true,R::PC,R::R0,4096).is_err());
 }
}
