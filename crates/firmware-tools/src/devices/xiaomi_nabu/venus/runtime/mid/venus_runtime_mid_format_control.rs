// SPDX-License-Identifier: GPL-3.0-only
//! Controller codec format setup, plane size validation and frame-rate mode
//! choice. Preserves public A32 instruction conditions and branches, while
//! separating a 15-word controller diagnostic/scaling constant pool from code.
//! This is an instruction-level re-encoding, not a behavioral proof or OEM
//! signed firmware image.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x37be8;
const END:usize=0x38a44;
/// Diagnostics, controller register offsets, two integer bit-masks and a
/// 32-bit reciprocal multiply constant: all non-instruction data.
const FORMAT_CONTROL_LITERALS:[u32;15]=[
    0x000e_c261,0x000e_c2ad,0x000e_2788,0x000e_c2c3,
    0x000e_c2f8,0xffff_5555,0x000e_c328,0x000e_c37b,
    0x000e_c3d1,0xaaaa_aaab,0x000e_c402,0x000e_c144,
    0x000e_c1b3,0x000e_c180,0x000e_c0b3,
];

/// A32 indexed LDRSH with P=1 U=1 L=1 and optional base writeback.
/// Register bitfields are from public ARMv7 architecture specification.
fn encode_signed_halfword_register_writeback(c:C,dst:R,base:R,offset:R,wb:bool)->Result<u32,String>{
    if [dst,base,offset].contains(&R::PC) || (wb && dst==base){
        return Err("invalid signed-halfword indexed transfer".into());
    }
    Ok(((c as u32)<<28)|0x0190_00f0|(if wb{1<<21}else{0})
       |(base.number()<<16)|(dst.number()<<12)|offset.number())
}

/// A32 positive register-indexed LDRH/STRH and optionally base writeback.
/// Signed displacement is not used by these two controller formats.
fn encode_halfword_register(c:C,load:bool,dst:R,base:R,index:R,wb:bool)->Result<u32,String>{
    if [dst,base,index].contains(&R::PC) || (load && wb && dst==base){
        return Err("invalid indexed halfword transfer".into());
    }
    Ok(((c as u32)<<28)|0x0180_00b0|(if load{1<<20}else{0})
       |(if wb{1<<21}else{0})|(base.number()<<16)
       |(dst.number()<<12)|index.number())
}

/// Setup channel input codec format, bound accepted dimensions and report
/// unsupported plane layouts; includes all reachable alternate-case tails.
fn select_format_parameters(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x37be8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x37bec: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x37bf0: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // 0x37bf4: str r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,4)?)?;
    // 0x37bf8: mov r9, r2
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R2))?;
    // 0x37bfc: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x37c00: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x37c04: bl 0x2f8d0 <entry+0xffffffffffffa444> @ imm = #-0x833c
    c.bl(0x2f8d0)?;
    // 0x37c08: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37c0c: beq 0x37c40 <entry+0x27b4>  @ imm = #0x2c
    c.b(C::Eq,0x37c40)?;
    // 0x37c10: ldrb r0, [r4, #0x16b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,363)?)?;
    // 0x37c14: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37c18: bne 0x37c40 <entry+0x27b4>  @ imm = #0x20
    c.b(C::Ne,0x37c40)?;
    // 0x37c1c: ldrb r0, [r4, #0x2c0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,704)?)?;
    // 0x37c20: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x37c24: beq 0x37c40 <entry+0x27b4>  @ imm = #0x14
    c.b(C::Eq,0x37c40)?;
    // 0x37c28: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,76)?)?;
    // 0x37c2c: ldr r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,80)?)?;
    // 0x37c30: str r1, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,76)?)?;
    // 0x37c34: str r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,80)?)?;
    // 0x37c38: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37c3c: strb r0, [r4, #0x16b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,363)?)?;
    // 0x37c40: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x37c44: ldr r1, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,76)?)?;
    // 0x37c48: mov r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15)?)?;
    // 0x37c4c: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x37c50: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x37c54: moveq r3, #31
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,31)?)?;
    // 0x37c58: moveq r2, #5
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x37c5c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x37c60: add r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x37c64: lsr r1, r1, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsr,R::R2,false)?)?;
    // 0x37c68: lsl r8, r1, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(8)?,R::R0,R::R1,Shift::Lsl,R::R2,false)?)?;
    // 0x37c6c: lsr r5, r8, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(8)?,Shift::Lsr,4)?)?;
    // 0x37c70: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x37c74: lsreq r5, r8, #5
    c.emit(arm32::mov_shifted_register(C::Eq,R::new(5)?,R::new(8)?,Shift::Lsr,5)?)?;
    // 0x37c78: ldr r1, [r4, #0x790]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1936)?)?;
    // 0x37c7c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x37c80: beq 0x37c9c <entry+0x2810>  @ imm = #0x14
    c.b(C::Eq,0x37c9c)?;
    // 0x37c84: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x37c88: bne 0x37ca4 <entry+0x2818>  @ imm = #0x14
    c.b(C::Ne,0x37ca4)?;
    // 0x37c8c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x37c90: str r1, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,300)?)?;
    // 0x37c94: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x37c98: b 0x37ca0 <entry+0x2814>  @ imm = #0x0
    c.b(C::Al,0x37ca0)?;
    // 0x37c9c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x37ca0: strb r1, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,350)?)?;
    // 0x37ca4: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // 0x37ca8: bne 0x37d74 <entry+0x28e8>  @ imm = #0xc4
    c.b(C::Ne,0x37d74)?;
    // 0x37cac: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x37cb0: beq 0x37cc8 <entry+0x283c>  @ imm = #0x10
    c.b(C::Eq,0x37cc8)?;
    // 0x37cb4: ldr r0, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,284)?)?;
    // 0x37cb8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x37cbc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37cc0: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x37cc4: str r1, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,300)?)?;
    // 0x37cc8: ldr r0, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,108)?)?;
    // 0x37ccc: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x37cd0: bne 0x37d0c <entry+0x2880>  @ imm = #0x34
    c.b(C::Ne,0x37d0c)?;
    // 0x37cd4: ldr r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // 0x37cd8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x37cdc: beq 0x37d0c <entry+0x2880>  @ imm = #0x28
    c.b(C::Eq,0x37d0c)?;
    // 0x37ce0: ldr r1, [pc, #0xd20]        @ 0x357c <entry+0x357c>
    c.literal_cond(C::Al,R::R1,0x38a08)?;
    // 0x37ce4: ldr r2, [pc, #0xd20]        @ 0x3580 <entry+0x3580>
    c.literal_cond(C::Al,R::R2,0x38a0c)?;
    // 0x37ce8: mov r3, #142
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,142)?)?;
    // 0x37cec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x37cf0: orr r3, r3, #2304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x37cf4: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x9068
    c.bl(0x2ec94)?;
    // 0x37cf8: ldr r1, [pc, #0xd10]        @ 0x3584 <entry+0x3584>
    c.literal_cond(C::Al,R::R1,0x38a10)?;
    // 0x37cfc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x37d00: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x9074
    c.bl(0x2ec94)?;
    // 0x37d04: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37d08: str r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // 0x37d0c: ldrb r0, [r4, #0xc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,200)?)?;
    // 0x37d10: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x37d14: bne 0x37d74 <entry+0x28e8>  @ imm = #0x58
    c.b(C::Ne,0x37d74)?;
    // 0x37d18: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,76)?)?;
    // 0x37d1c: ldr r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,80)?)?;
    // 0x37d20: ldr r2, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,96)?)?;
    // 0x37d24: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,4)?)?;
    // 0x37d28: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // 0x37d2c: mul r3, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R0)?)?;
    // 0x37d30: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x37d34: mul r1, r3, r2
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(3)?,R::R2)?)?;
    // 0x37d38: mov r2, #14464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,14464)?)?;
    // 0x37d3c: orr r2, r2, #475136
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,475136)?)?;
    // 0x37d40: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x37d44: bhi 0x37d70 <entry+0x28e4>  @ imm = #0x24
    c.b(C::Hi,0x37d70)?;
    // 0x37d48: ldr r1, [pc, #0xcc4]        @ 0x3588 <entry+0x3588>
    c.literal_cond(C::Al,R::R1,0x38a14)?;
    // 0x37d4c: ldr r2, [pc, #0xcb8]        @ 0x3580 <entry+0x3580>
    c.literal_cond(C::Al,R::R2,0x38a0c)?;
    // 0x37d50: mov r3, #150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,150)?)?;
    // 0x37d54: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x37d58: orr r3, r3, #2304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2304)?)?;
    // 0x37d5c: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x90d0
    c.bl(0x2ec94)?;
    // 0x37d60: ldr r1, [pc, #0xca8]        @ 0x3584 <entry+0x3584>
    c.literal_cond(C::Al,R::R1,0x38a10)?;
    // 0x37d64: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x37d68: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x90dc
    c.bl(0x2ec94)?;
    // 0x37d6c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37d70: str r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // 0x37d74: ldrb r0, [r4, #0xc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,200)?)?;
    // 0x37d78: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x37d7c: bne 0x37da0 <entry+0x2914>  @ imm = #0x1c
    c.b(C::Ne,0x37da0)?;
    // 0x37d80: ldr r1, [pc, #0xc90]        @ 0x358c <entry+0x358c>
    c.literal_cond(C::Al,R::R1,0x38a18)?;
    // 0x37d84: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37d88: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x90fc
    c.bl(0x2ec94)?;
    // 0x37d8c: ldr r1, [pc, #0xc7c]        @ 0x3584 <entry+0x3584>
    c.literal_cond(C::Al,R::R1,0x38a10)?;
    // 0x37d90: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37d94: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x9108
    c.bl(0x2ec94)?;
    // 0x37d98: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37d9c: str r0, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,284)?)?;
    // 0x37da0: ldr r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // 0x37da4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37da8: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37dac: strbeq r0, [r4, #0x13a]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R0,R::new(4)?,314)?)?;
    // 0x37db0: ldrb r0, [r4, #0x68c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1676)?)?;
    // 0x37db4: cmp r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,9)?)?;
    // 0x37db8: bne 0x37dd4 <entry+0x2948>  @ imm = #0x14
    c.b(C::Ne,0x37dd4)?;
    // 0x37dbc: ldr r0, [pc, #0xc58]        @ 0x3590 <entry+0x3590>
    c.literal_cond(C::Al,R::R0,0x38a1c)?;
    // 0x37dc0: str r0, [r4, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1656)?)?;
    // 0x37dc4: str r0, [r4, #0x67c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1660)?)?;
    // 0x37dc8: str r0, [r4, #0x680]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1664)?)?;
    // 0x37dcc: str r0, [r4, #0x684]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1668)?)?;
    // 0x37dd0: str r0, [r4, #0x688]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1672)?)?;
    // 0x37dd4: ldrb r0, [r4, #0x60a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1546)?)?;
    // 0x37dd8: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x37ddc: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x37de0: strbeq r1, [r4, #0x17c]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R1,R::new(4)?,380)?)?;
    // 0x37de4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x37de8: moveq r0, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x37dec: strb r0, [r4, #0x60a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1546)?)?;
    // 0x37df0: ldrb r0, [r4, #0x159]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,345)?)?;
    // 0x37df4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37df8: beq 0x37e84 <entry+0x29f8>  @ imm = #0x84
    c.b(C::Eq,0x37e84)?;
    // 0x37dfc: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,76)?)?;
    // 0x37e00: cmp r0, #640
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,640)?)?;
    // 0x37e04: blo 0x37e18 <entry+0x298c>  @ imm = #0xc
    c.b(C::Cc,0x37e18)?;
    // 0x37e08: ldr r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,80)?)?;
    // 0x37e0c: lsr r0, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,5)?)?;
    // 0x37e10: cmp r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,14)?)?;
    // 0x37e14: bhi 0x37e34 <entry+0x29a8>  @ imm = #0x18
    c.b(C::Hi,0x37e34)?;
    // 0x37e18: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37e1c: mov r3, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,192)?)?;
    // 0x37e20: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x37e24: mov r7, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,192)?)?;
    // 0x37e28: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x37e2c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x37e30: b 0x37e4c <entry+0x29c0>  @ imm = #0x14
    c.b(C::Al,0x37e4c)?;
    // 0x37e34: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x37e38: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x37e3c: mov r0, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,23)?)?;
    // 0x37e40: mov r7, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,96)?)?;
    // 0x37e44: mov r3, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,48)?)?;
    // 0x37e48: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x37e4c: strb r6, [r4, #0x159]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,345)?)?;
    // 0x37e50: str r3, [r4, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,392)?)?;
    // 0x37e54: str r7, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,400)?)?;
    // 0x37e58: strb r0, [r4, #0x418]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1048)?)?;
    // 0x37e5c: strb r2, [r4, #0x419]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1049)?)?;
    // 0x37e60: strb r0, [r4, #0x42f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1071)?)?;
    // 0x37e64: strb r2, [r4, #0x430]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1072)?)?;
    // 0x37e68: strb r0, [r4, #0x446]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1094)?)?;
    // 0x37e6c: strb r2, [r4, #0x447]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1095)?)?;
    // 0x37e70: strb r0, [r4, #0x45d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1117)?)?;
    // 0x37e74: strb r2, [r4, #0x45e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1118)?)?;
    // 0x37e78: strb r0, [r4, #0x474]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1140)?)?;
    // 0x37e7c: strb r2, [r4, #0x475]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1141)?)?;
    // 0x37e80: str r1, [r4, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,496)?)?;
    // 0x37e84: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x37e88: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x37e8c: bne 0x37f1c <entry+0x2a90>  @ imm = #0x88
    c.b(C::Ne,0x37f1c)?;
    // 0x37e90: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x37e94: str r1, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,300)?)?;
    // 0x37e98: ldr r1, [r4, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,956)?)?;
    // 0x37e9c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x37ea0: bne 0x37eb0 <entry+0x2a24>  @ imm = #0x8
    c.b(C::Ne,0x37eb0)?;
    // 0x37ea4: ldr r1, [r4, #0x3c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,960)?)?;
    // 0x37ea8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x37eac: beq 0x37ec8 <entry+0x2a3c>  @ imm = #0x14
    c.b(C::Eq,0x37ec8)?;
    // 0x37eb0: ldr r1, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,972)?)?;
    // 0x37eb4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x37eb8: bne 0x37ed0 <entry+0x2a44>  @ imm = #0x10
    c.b(C::Ne,0x37ed0)?;
    // 0x37ebc: ldr r1, [r4, #0x3c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,968)?)?;
    // 0x37ec0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x37ec4: bne 0x37ed0 <entry+0x2a44>  @ imm = #0x4
    c.b(C::Ne,0x37ed0)?;
    // 0x37ec8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x37ecc: str r1, [r4, #0x3c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,964)?)?;
    // 0x37ed0: ldrb r2, [r4, #0x15b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,347)?)?;
    // 0x37ed4: add r1, r4, #760
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,760)?)?;
    // 0x37ed8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x37edc: beq 0x37f0c <entry+0x2a80>  @ imm = #0x28
    c.b(C::Eq,0x37f0c)?;
    // 0x37ee0: ldrsh r2, [r1]
    c.emit(arm32::load_signed_halfword(C::Al,R::R2,R::R1,0)?)?;
    // 0x37ee4: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x37ee8: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // 0x37eec: cmp r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,256)?)?;
    // 0x37ef0: movlt r2, r3
    c.emit(arm32::mov_register(C::Lt,R::R2,R::new(3)?))?;
    // 0x37ef4: strh r2, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R1,0)?)?;
    // 0x37ef8: mov r1, #764
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,764)?)?;
    // 0x37efc: ldrsh r2, [r4, r1]
    c.emit(encode_signed_halfword_register_writeback(C::Al,R::R2,R::new(4)?,R::R1,false)?)?;
    // 0x37f00: cmp r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,256)?)?;
    // 0x37f04: movlt r2, r3
    c.emit(arm32::mov_register(C::Lt,R::R2,R::new(3)?))?;
    // 0x37f08: b 0x37f14 <entry+0x2a88>  @ imm = #0x4
    c.b(C::Al,0x37f14)?;
    // 0x37f0c: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x37f10: strh r2, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R1,0)?)?;
    // 0x37f14: mov r1, #764
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,764)?)?;
    // 0x37f18: strh r2, [r4, r1]
    c.emit(encode_halfword_register(C::Al,false,R::R2,R::new(4)?,R::R1,false)?)?;
    // 0x37f1c: ldrb r1, [r4, #0x2b3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,691)?)?;
    // 0x37f20: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x37f24: ldrbeq r1, [r4, #0x15b]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R1,R::new(4)?,347)?)?;
    // 0x37f28: cmpeq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x37f2c: bne 0x37f54 <entry+0x2ac8>  @ imm = #0x20
    c.b(C::Ne,0x37f54)?;
    // 0x37f30: ldr r1, [pc, #0xae8]        @ 0x3594 <entry+0x3594>
    c.literal_cond(C::Al,R::R1,0x38a20)?;
    // 0x37f34: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37f38: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x37f3c: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x92b0
    c.bl(0x2ec94)?;
    // 0x37f40: ldr r1, [pc, #0xac8]        @ 0x3584 <entry+0x3584>
    c.literal_cond(C::Al,R::R1,0x38a10)?;
    // 0x37f44: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37f48: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x92bc
    c.bl(0x2ec94)?;
    // 0x37f4c: strb r6, [r4, #0x15b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(4)?,347)?)?;
    // 0x37f50: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x37f54: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x37f58: beq 0x37f80 <entry+0x2af4>  @ imm = #0x20
    c.b(C::Eq,0x37f80)?;
    // 0x37f5c: ldrb r0, [r4, #0x2c3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,707)?)?;
    // 0x37f60: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37f64: bne 0x37f80 <entry+0x2af4>  @ imm = #0x14
    c.b(C::Ne,0x37f80)?;
    // 0x37f68: ldrb r0, [r4, #0x2b3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,691)?)?;
    // 0x37f6c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x37f70: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37f74: strbne r0, [r4, #0x15b]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,347)?)?;
    // 0x37f78: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37f7c: strb r0, [r4, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,348)?)?;
    // 0x37f80: str r5, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    // 0x37f84: ldrb r0, [r4, #0x2b3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,691)?)?;
    // 0x37f88: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x37f8c: ldrbeq r0, [r4, #0x15c]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(4)?,348)?)?;
    // 0x37f90: cmpeq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x37f94: bne 0x37fc4 <entry+0x2b38>  @ imm = #0x28
    c.b(C::Ne,0x37fc4)?;
    // 0x37f98: ldr r1, [pc, #0xa84]        @ 0x3598 <entry+0x3598>
    c.literal_cond(C::Al,R::R1,0x38a24)?;
    // 0x37f9c: ldr r2, [pc, #0xa68]        @ 0x3580 <entry+0x3580>
    c.literal_cond(C::Al,R::R2,0x38a0c)?;
    // 0x37fa0: mov r3, #500
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,500)?)?;
    // 0x37fa4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x37fa8: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x37fac: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x9320
    c.bl(0x2ec94)?;
    // 0x37fb0: ldr r1, [pc, #0xa58]        @ 0x3584 <entry+0x3584>
    c.literal_cond(C::Al,R::R1,0x38a10)?;
    // 0x37fb4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x37fb8: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x932c
    c.bl(0x2ec94)?;
    // 0x37fbc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37fc0: strb r0, [r4, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,348)?)?;
    // 0x37fc4: mov r10, r4
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(4)?))?;
    // 0x37fc8: ldr r0, [r10, #0x194]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(10)?,404)?)?;
    // 0x37fcc: sub r1, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(10)?,8)?)?;
    // 0x37fd0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37fd4: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x37fd8: mov lr, r10
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(10)?))?;
    // 0x37fdc: sub r12, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R12,R::new(10)?,12)?)?;
    // 0x37fe0: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x37fe4: moveq r0, r1
    c.emit(arm32::mov_register(C::Eq,R::R0,R::R1))?;
    // 0x37fe8: ldr r6, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,0)?)?;
    // 0x37fec: ldr r0, [lr, #-0x4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::LR,-4)?)?;
    // 0x37ff0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37ff4: mov r0, lr
    c.emit(arm32::mov_register(C::Al,R::R0,R::LR))?;
    // 0x37ff8: moveq r0, r12
    c.emit(arm32::mov_register(C::Eq,R::R0,R::R12))?;
    // 0x37ffc: ldr r3, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // 0x38000: cmp r3, #192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,192)?)?;
    // 0x38004: mov r5, r3
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // 0x38008: movge r5, #192
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::new(5)?,R::R0,192)?)?;
    // 0x3800c: cmp r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,64)?)?;
    // 0x38010: movge r6, #64
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::new(6)?,R::R0,64)?)?;
    // 0x38014: str r5, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::LR,0)?)?;
    // 0x38018: str r6, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,0)?)?;
    // 0x3801c: ldr r2, [r10, #-0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,-12)?)?;
    // 0x38020: cmp r2, #96
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,96)?)?;
    // 0x38024: movge r2, #96
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R2,R::R0,96)?)?;
    // 0x38028: str r2, [r10, #-0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,-12)?)?;
    // 0x3802c: ldr r7, [r10, #-0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,-8)?)?;
    // 0x38030: cmp r7, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,32)?)?;
    // 0x38034: movge r7, #32
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::new(7)?,R::R0,32)?)?;
    // 0x38038: str r7, [r10, #-0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(10)?,-8)?)?;
    // 0x3803c: ldrb r0, [r10, #0x504]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1284)?)?;
    // 0x38040: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x38044: bne 0x38058 <entry+0x2bcc>  @ imm = #0xc
    c.b(C::Ne,0x38058)?;
    // 0x38048: cmp r5, #128
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,128)?)?;
    // 0x3804c: movge r3, #128
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // 0x38050: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::LR,0)?)?;
    // 0x38054: b 0x3805c <entry+0x2bd0>  @ imm = #0x0
    c.b(C::Al,0x3805c)?;
    // 0x38058: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x3805c: sub r1, r6, #33
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(6)?,33)?)?;
    // 0x38060: mov r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,48)?)?;
    // 0x38064: cmp r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // 0x38068: bhs 0x38074 <entry+0x2be8>  @ imm = #0x4
    c.b(C::Cs,0x38074)?;
    // 0x3806c: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // 0x38070: b 0x38084 <entry+0x2bf8>  @ imm = #0xc
    c.b(C::Al,0x38084)?;
    // 0x38074: sub r1, r6, #17
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(6)?,17)?)?;
    // 0x38078: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x3807c: cmp r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,14)?)?;
    // 0x38080: strls r0, [r10]
    c.emit(arm32::transfer_word(C::Ls,false,R::R0,R::new(10)?,0)?)?;
    // 0x38084: sub r0, r7, #17
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,17)?)?;
    // 0x38088: mov r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,32)?)?;
    // 0x3808c: cmp r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,14)?)?;
    // 0x38090: ldrls r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Ls,true,R::R1,R::SP,16)?)?;
    // 0x38094: movls r0, #32
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x38098: strls r0, [r1]
    c.emit(arm32::transfer_word(C::Ls,false,R::R0,R::R1,0)?)?;
    // 0x3809c: mov r1, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,192)?)?;
    // 0x380a0: ldrb r7, [r4, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,348)?)?;
    // 0x380a4: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x380a8: movne r6, #48
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,48)?)?;
    // 0x380ac: rsb r5, r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(5)?,R::new(6)?,0)?)?;
    // 0x380b0: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x380b4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x380b8: blt 0x380dc <entry+0x2c50>  @ imm = #0x1c
    c.b(C::Lt,0x380dc)?;
    // 0x380bc: sub r1, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x380c0: cmp r3, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x380c4: bge 0x380b0 <entry+0x2c24>  @ imm = #-0x1c
    c.b(C::Ge,0x380b0)?;
    // 0x380c8: add r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x380cc: cmp r3, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x380d0: ble 0x380b0 <entry+0x2c24>  @ imm = #-0x28
    c.b(C::Le,0x380b0)?;
    // 0x380d4: add r0, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x380d8: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,0)?)?;
    // 0x380dc: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x380e0: beq 0x380f4 <entry+0x2c68>  @ imm = #0xc
    c.b(C::Eq,0x380f4)?;
    // 0x380e4: sub r0, r2, #49
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R2,49)?)?;
    // 0x380e8: cmp r0, #46
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,46)?)?;
    // 0x380ec: bls 0x38100 <entry+0x2c74>  @ imm = #0xc
    c.b(C::Ls,0x38100)?;
    // 0x380f0: b 0x38108 <entry+0x2c7c>  @ imm = #0x10
    c.b(C::Al,0x38108)?;
    // 0x380f4: sub r0, r2, #65
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R2,65)?)?;
    // 0x380f8: cmp r0, #30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,30)?)?;
    // 0x380fc: bhi 0x38310 <entry+0x2e84>  @ imm = #0x20c
    c.b(C::Hi,0x38310)?;
    // 0x38100: mov r0, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,96)?)?;
    // 0x38104: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x38108: mov r1, #776
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,776)?)?;
    // 0x3810c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x38110: ldrh r1, [r0, r1]!
    c.emit(encode_halfword_register(C::Al,true,R::R1,R::R0,R::R1,true)?)?;
    // 0x38114: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x38118: beq 0x3813c <entry+0x2cb0>  @ imm = #0x1c
    c.b(C::Eq,0x3813c)?;
    // 0x3811c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x38120: strh r1, [r0]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::R0,0)?)?;
    // 0x38124: ldr r1, [pc, #0x8fc]        @ 0x359c <entry+0x359c>
    c.literal_cond(C::Al,R::R1,0x38a28)?;
    // 0x38128: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3812c: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x94a0
    c.bl(0x2ec94)?;
    // 0x38130: ldr r1, [pc, #0x8d8]        @ 0x3584 <entry+0x3584>
    c.literal_cond(C::Al,R::R1,0x38a10)?;
    // 0x38134: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38138: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x94ac
    c.bl(0x2ec94)?;
    // 0x3813c: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x38140: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x38144: orr r0, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
    // 0x38148: ldrsh r0, [r1, r0]!
    c.emit(encode_signed_halfword_register_writeback(C::Al,R::R0,R::R1,R::R0,true)?)?;
    // 0x3814c: cmn r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,2)?)?;
    // 0x38150: mvnle r0, #1
    c.emit(arm32::data_immediate(C::Le,Op::Mvn,R::R0,R::R0,1)?)?;
    // 0x38154: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x38158: movge r0, #2
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x3815c: strh r0, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,0)?)?;
    // 0x38160: ldr r0, [r1, #-0x29e]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,-670)?)?;
    // 0x38164: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38168: beq 0x38198 <entry+0x2d0c>  @ imm = #0x28
    c.b(C::Eq,0x38198)?;
    // 0x3816c: ldr r0, [r4, #0x314]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,788)?)?;
    // 0x38170: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38174: bne 0x381a0 <entry+0x2d14>  @ imm = #0x24
    c.b(C::Ne,0x381a0)?;
    // 0x38178: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x3817c: strb r0, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,350)?)?;
    // 0x38180: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38184: str r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // 0x38188: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // 0x3818c: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x38190: strne r0, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,300)?)?;
    // 0x38194: b 0x381a0 <entry+0x2d14>  @ imm = #0x4
    c.b(C::Al,0x381a0)?;
    // 0x38198: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3819c: str r0, [r4, #0x314]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,788)?)?;
    // 0x381a0: ldrb r0, [r4, #0x250]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,592)?)?;
    // 0x381a4: mov r1, #198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,198)?)?;
    // 0x381a8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x381ac: mov r3, #712
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,712)?)?;
    // 0x381b0: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x381b4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x381b8: ldrbne r0, [r4, #0x74]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(4)?,116)?)?;
    // 0x381bc: strbne r0, [r4, #0x24f]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,591)?)?;
    // 0x381c0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x381c4: strb r2, [r4, #0x2b4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,692)?)?;
    // 0x381c8: strb r2, [r4, #0x2b5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,693)?)?;
    // 0x381cc: strh r2, [r0, r1]!
    c.emit(encode_halfword_register(C::Al,false,R::R2,R::R0,R::R1,true)?)?;
    // 0x381d0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x381d4: strh r2, [r1, r3]!
    c.emit(encode_halfword_register(C::Al,false,R::R2,R::R1,R::new(3)?,true)?)?;
    // 0x381d8: ldrb r3, [r1, #-0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,-8)?)?;
    // 0x381dc: ldr r2, [r1, #-0x27c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,-636)?)?;
    // 0x381e0: tst r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,1)?)?;
    // 0x381e4: bne 0x38200 <entry+0x2d74>  @ imm = #0x14
    c.b(C::Ne,0x38200)?;
    // 0x381e8: ldr r3, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,52)?)?;
    // 0x381ec: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x381f0: bhs 0x38214 <entry+0x2d88>  @ imm = #0x1c
    c.b(C::Cs,0x38214)?;
    // 0x381f4: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x381f8: bne 0x38250 <entry+0x2dc4>  @ imm = #0x50
    c.b(C::Ne,0x38250)?;
    // 0x381fc: b 0x3822c <entry+0x2da0>  @ imm = #0x28
    c.b(C::Al,0x3822c)?;
    // 0x38200: ldr r3, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,56)?)?;
    // 0x38204: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x38208: blo 0x3822c <entry+0x2da0>  @ imm = #0x1c
    c.b(C::Cc,0x3822c)?;
    // 0x3820c: ldr r2, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,52)?)?;
    // 0x38210: b 0x38220 <entry+0x2d94>  @ imm = #0x8
    c.b(C::Al,0x38220)?;
    // 0x38214: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x38218: bne 0x38250 <entry+0x2dc4>  @ imm = #0x30
    c.b(C::Ne,0x38250)?;
    // 0x3821c: ldr r2, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,56)?)?;
    // 0x38220: ldr r3, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,80)?)?;
    // 0x38224: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x38228: bhs 0x38250 <entry+0x2dc4>  @ imm = #0x20
    c.b(C::Cs,0x38250)?;
    // 0x3822c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x38230: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x38234: strb r2, [r4, #0x2b4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,692)?)?;
    // 0x38238: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x3823c: strb r2, [r4, #0x2b5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,693)?)?;
    // 0x38240: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x38244: strh r2, [r0]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,0)?)?;
    // 0x38248: strh r2, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R1,0)?)?;
    // 0x3824c: b 0x3826c <entry+0x2de0>  @ imm = #0x18
    c.b(C::Al,0x3826c)?;
    // 0x38250: ldr r0, [r4, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,696)?)?;
    // 0x38254: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x38258: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3825c: bne 0x3826c <entry+0x2de0>  @ imm = #0x8
    c.b(C::Ne,0x3826c)?;
    // 0x38260: ldr r0, [r4, #0x2bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,700)?)?;
    // 0x38264: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38268: beq 0x38388 <entry+0x2efc>  @ imm = #0x118
    c.b(C::Eq,0x38388)?;
    // 0x3826c: cmp r8, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1920)?)?;
    // 0x38270: str r6, [r4, #0x7a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1952)?)?;
    // 0x38274: bhi 0x38298 <entry+0x2e0c>  @ imm = #0x1c
    c.b(C::Hi,0x38298)?;
    // 0x38278: ldrb r0, [r4, #0x159]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,345)?)?;
    // 0x3827c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38280: beq 0x38298 <entry+0x2e0c>  @ imm = #0x10
    c.b(C::Eq,0x38298)?;
    // 0x38284: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38288: str r8, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,288)?)?;
    // 0x3828c: mov r7, r8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // 0x38290: strb r0, [r4, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,332)?)?;
    // 0x38294: b 0x38558 <entry+0x30cc>  @ imm = #0x2bc
    c.b(C::Al,0x38558)?;
    // 0x38298: cmp r8, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,3840)?)?;
    // 0x3829c: bls 0x382e0 <entry+0x2e54>  @ imm = #0x3c
    c.b(C::Ls,0x382e0)?;
    // 0x382a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x382a4: str r0, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,284)?)?;
    // 0x382a8: strb r0, [r4, #0x162]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,354)?)?;
    // 0x382ac: ldr r0, [r4, #0x174]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,372)?)?;
    // 0x382b0: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x382b4: movne r0, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x382b8: strne r0, [r4, #0x174]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,372)?)?;
    // 0x382bc: ldrb r1, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,350)?)?;
    // 0x382c0: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x382c4: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x382c8: cmpeq r0, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x382cc: bne 0x38348 <entry+0x2ebc>  @ imm = #0x74
    c.b(C::Ne,0x38348)?;
    // 0x382d0: mov r7, #2112
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2112)?)?;
    // 0x382d4: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x382d8: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x382dc: b 0x3844c <entry+0x2fc0>  @ imm = #0x168
    c.b(C::Al,0x3844c)?;
    // 0x382e0: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x382e4: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x382e8: bne 0x38324 <entry+0x2e98>  @ imm = #0x34
    c.b(C::Ne,0x38324)?;
    // 0x382ec: ldr r1, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,284)?)?;
    // 0x382f0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x382f4: bne 0x38304 <entry+0x2e78>  @ imm = #0x8
    c.b(C::Ne,0x38304)?;
    // 0x382f8: ldrb r1, [r4, #0x162]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,354)?)?;
    // 0x382fc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x38300: beq 0x3896c <entry+0x34e0>  @ imm = #0x664
    c.b(C::Eq,0x3896c)?;
    // 0x38304: mov r7, #480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,480)?)?;
    // 0x38308: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x3830c: b 0x384e4 <entry+0x3058>  @ imm = #0x1d0
    c.b(C::Al,0x384e4)?;
    // 0x38310: sub r0, r2, #33
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R2,33)?)?;
    // 0x38314: cmp r0, #30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,30)?)?;
    // 0x38318: bhi 0x38108 <entry+0x2c7c>  @ imm = #-0x218
    c.b(C::Hi,0x38108)?;
    // 0x3831c: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // 0x38320: b 0x38104 <entry+0x2c78>  @ imm = #-0x224
    c.b(C::Al,0x38104)?;
    // 0x38324: ldr r1, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,300)?)?;
    // 0x38328: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3832c: beq 0x383e4 <entry+0x2f58>  @ imm = #0xb0
    c.b(C::Eq,0x383e4)?;
    // 0x38330: ldr r1, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,108)?)?;
    // 0x38334: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x38338: bne 0x38454 <entry+0x2fc8>  @ imm = #0x114
    c.b(C::Ne,0x38454)?;
    // 0x3833c: cmp r8, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1920)?)?;
    // 0x38340: bls 0x38468 <entry+0x2fdc>  @ imm = #0x120
    c.b(C::Ls,0x38468)?;
    // 0x38344: b 0x3847c <entry+0x2ff0>  @ imm = #0x130
    c.b(C::Al,0x3847c)?;
    // 0x38348: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x3834c: bne 0x38410 <entry+0x2f84>  @ imm = #0xbc
    c.b(C::Ne,0x38410)?;
    // 0x38350: ldrb r0, [r4, #0x250]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,592)?)?;
    // 0x38354: mov r7, #960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,960)?)?;
    // 0x38358: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3835c: bne 0x3841c <entry+0x2f90>  @ imm = #0xb8
    c.b(C::Ne,0x3841c)?;
    // 0x38360: ldrb r0, [r4, #0x2b5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,693)?)?;
    // 0x38364: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38368: bne 0x38380 <entry+0x2ef4>  @ imm = #0x10
    c.b(C::Ne,0x38380)?;
    // 0x3836c: ldr r0, [r4, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,696)?)?;
    // 0x38370: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38374: ldreq r0, [r4, #0x2bc]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,700)?)?;
    // 0x38378: orrseq r0, r0, r6
    c.emit(arm32::data_register(C::Eq,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,true)?)?;
    // 0x3837c: beq 0x3841c <entry+0x2f90>  @ imm = #0x98
    c.b(C::Eq,0x3841c)?;
    // 0x38380: mov r7, #768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,768)?)?;
    // 0x38384: b 0x3841c <entry+0x2f90>  @ imm = #0x90
    c.b(C::Al,0x3841c)?;
    // 0x38388: ldr r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // 0x3838c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38390: beq 0x3826c <entry+0x2de0>  @ imm = #-0x12c
    c.b(C::Eq,0x3826c)?;
    // 0x38394: bl 0x2f8d0 <entry+0xffffffffffffa444> @ imm = #-0x8acc
    c.bl(0x2f8d0)?;
    // 0x38398: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3839c: beq 0x383c4 <entry+0x2f38>  @ imm = #0x20
    c.b(C::Eq,0x383c4)?;
    // 0x383a0: ldrb r0, [r4, #0x2c0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,704)?)?;
    // 0x383a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x383a8: bne 0x3826c <entry+0x2de0>  @ imm = #-0x144
    c.b(C::Ne,0x3826c)?;
    // 0x383ac: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x383b0: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x383b4: beq 0x3826c <entry+0x2de0>  @ imm = #-0x150
    c.b(C::Eq,0x3826c)?;
    // 0x383b8: ldrb r0, [r4, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,348)?)?;
    // 0x383bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x383c0: bne 0x3826c <entry+0x2de0>  @ imm = #-0x15c
    c.b(C::Ne,0x3826c)?;
    // 0x383c4: bl 0x2f8d0 <entry+0xffffffffffffa444> @ imm = #-0x8afc
    c.bl(0x2f8d0)?;
    // 0x383c8: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x383cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x383d0: bne 0x3826c <entry+0x2de0>  @ imm = #-0x16c
    c.b(C::Ne,0x3826c)?;
    // 0x383d4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x383d8: bl 0x3701c <entry+0x1b90>  @ imm = #-0x13c4
    c.bl(0x3701c)?;
    // 0x383dc: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x383e0: b 0x3826c <entry+0x2de0>  @ imm = #-0x17c
    c.b(C::Al,0x3826c)?;
    // 0x383e4: ldr r1, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,284)?)?;
    // 0x383e8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x383ec: bne 0x383fc <entry+0x2f70>  @ imm = #0x8
    c.b(C::Ne,0x383fc)?;
    // 0x383f0: ldrb r1, [r4, #0x162]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,354)?)?;
    // 0x383f4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x383f8: beq 0x389f0 <entry+0x3564>  @ imm = #0x5f0
    c.b(C::Eq,0x389f0)?;
    // 0x383fc: mov r7, #480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,480)?)?;
    // 0x38400: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x38404: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x38408: str r1, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,300)?)?;
    // 0x3840c: b 0x384e4 <entry+0x3058>  @ imm = #0xd0
    c.b(C::Al,0x384e4)?;
    // 0x38410: mov r7, #1056
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1056)?)?;
    // 0x38414: cmp r8, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,4608)?)?;
    // 0x38418: movls r7, #1152
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(7)?,R::R0,1152)?)?;
    // 0x3841c: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x38420: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x38424: ldr r0, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,300)?)?;
    // 0x38428: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3842c: beq 0x3844c <entry+0x2fc0>  @ imm = #0x18
    c.b(C::Eq,0x3844c)?;
    // 0x38430: ldr r1, [pc, #0x5f8]        @ 0x35a4 <entry+0x35a4>
    c.literal_cond(C::Al,R::R1,0x38a30)?;
    // 0x38434: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38438: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x97ac
    c.bl(0x2ec94)?;
    // 0x3843c: ldr r1, [pc, #0x5cc]        @ 0x3584 <entry+0x3584>
    c.literal_cond(C::Al,R::R1,0x38a10)?;
    // 0x38440: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38444: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x97b8
    c.bl(0x2ec94)?;
    // 0x38448: ldr r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,288)?)?;
    // 0x3844c: str r6, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,300)?)?;
    // 0x38450: b 0x38558 <entry+0x30cc>  @ imm = #0x100
    c.b(C::Al,0x38558)?;
    // 0x38454: cmp r8, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1920)?)?;
    // 0x38458: bhi 0x3847c <entry+0x2ff0>  @ imm = #0x1c
    c.b(C::Hi,0x3847c)?;
    // 0x3845c: ldrb r1, [r4, #0xc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,200)?)?;
    // 0x38460: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x38464: bne 0x3847c <entry+0x2ff0>  @ imm = #0x10
    c.b(C::Ne,0x3847c)?;
    // 0x38468: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x3846c: str r8, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,288)?)?;
    // 0x38470: mov r7, r8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // 0x38474: strb r1, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,350)?)?;
    // 0x38478: b 0x384d4 <entry+0x3048>  @ imm = #0x54
    c.b(C::Al,0x384d4)?;
    // 0x3847c: ldrb r1, [r4, #0x162]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,354)?)?;
    // 0x38480: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x38484: bne 0x38494 <entry+0x3008>  @ imm = #0x8
    c.b(C::Ne,0x38494)?;
    // 0x38488: ldr r1, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,284)?)?;
    // 0x3848c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x38490: beq 0x384ac <entry+0x3020>  @ imm = #0x14
    c.b(C::Eq,0x384ac)?;
    // 0x38494: ldrb r1, [r4, #0xc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,200)?)?;
    // 0x38498: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x3849c: blo 0x384ac <entry+0x3020>  @ imm = #0x8
    c.b(C::Cc,0x384ac)?;
    // 0x384a0: str r8, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,288)?)?;
    // 0x384a4: mov r7, r8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // 0x384a8: b 0x384d4 <entry+0x3048>  @ imm = #0x24
    c.b(C::Al,0x384d4)?;
    // 0x384ac: ldr r2, [pc, #0x578]        @ 0x35a0 <entry+0x35a0>
    c.literal_cond(C::Al,R::R2,0x38a2c)?;
    // 0x384b0: mov r1, #95
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,95)?)?;
    // 0x384b4: cmp r8, #960
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,960)?)?;
    // 0x384b8: add r1, r1, r8, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(8)?,Shift::Lsr,1,false)?)?;
    // 0x384bc: umull r3, r7, r1, r2
    c.emit(super::encode_long_multiply(C::Al,false,R::new(3)?,R::new(7)?,R::R1,R::R2)?)?;
    // 0x384c0: lsr r1, r7, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(7)?,Shift::Lsr,6)?)?;
    // 0x384c4: mov r7, r8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // 0x384c8: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // 0x384cc: lslhi r7, r1, #5
    c.emit(arm32::mov_shifted_register(C::Hi,R::new(7)?,R::R1,Shift::Lsl,5)?)?;
    // 0x384d0: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x384d4: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x384d8: cmp r7, #960
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,960)?)?;
    // 0x384dc: ldrls r1, [r10]
    c.emit(arm32::transfer_word(C::Ls,true,R::R1,R::new(10)?,0)?)?;
    // 0x384e0: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // 0x384e4: cmp r7, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x384e8: mov r6, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,192)?)?;
    // 0x384ec: movhi r7, r8
    c.emit(arm32::mov_register(C::Hi,R::new(7)?,R::new(8)?))?;
    // 0x384f0: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x384f4: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x384f8: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x384fc: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x38500: moveq r6, #256
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,256)?)?;
    // 0x38504: bl 0x81850 <entry+0x4c3c4> @ imm = #0x49344
    c.bl(0x81850)?;
    // 0x38508: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3850c: beq 0x38558 <entry+0x30cc>  @ imm = #0x44
    c.b(C::Eq,0x38558)?;
    // 0x38510: cmp r1, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x38514: bhs 0x38558 <entry+0x30cc>  @ imm = #0x3c
    c.b(C::Cs,0x38558)?;
    // 0x38518: cmp r7, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,768)?)?;
    // 0x3851c: bls 0x3853c <entry+0x30b0>  @ imm = #0x18
    c.b(C::Ls,0x3853c)?;
    // 0x38520: sub r7, r7, #96
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::new(7)?,96)?)?;
    // 0x38524: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x38528: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x3852c: bl 0x81850 <entry+0x4c3c4> @ imm = #0x4931c
    c.bl(0x81850)?;
    // 0x38530: cmp r1, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x38534: blo 0x38520 <entry+0x3094>  @ imm = #-0x1c
    c.b(C::Cc,0x38520)?;
    // 0x38538: b 0x38554 <entry+0x30c8>  @ imm = #0x14
    c.b(C::Al,0x38554)?;
    // 0x3853c: add r7, r7, #96
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,96)?)?;
    // 0x38540: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x38544: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x38548: bl 0x81850 <entry+0x4c3c4> @ imm = #0x49300
    c.bl(0x81850)?;
    // 0x3854c: cmp r1, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x38550: blo 0x3853c <entry+0x30b0>  @ imm = #-0x1c
    c.b(C::Cc,0x3853c)?;
    // 0x38554: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x38558: cmp r7, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3855c: movhi r7, r8
    c.emit(arm32::mov_register(C::Hi,R::new(7)?,R::new(8)?))?;
    // 0x38560: cmp r8, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1920)?)?;
    // 0x38564: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x38568: ldr r6, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,80)?)?;
    // 0x3856c: str r6, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,296)?)?;
    // 0x38570: ldrb r0, [r4, #0xc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,200)?)?;
    // 0x38574: ldr r9, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,4)?)?;
    // 0x38578: bhi 0x385b8 <entry+0x312c>  @ imm = #0x38
    c.b(C::Hi,0x385b8)?;
    // 0x3857c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x38580: bne 0x385b8 <entry+0x312c>  @ imm = #0x30
    c.b(C::Ne,0x385b8)?;
    // 0x38584: cmp r9, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,8192)?)?;
    // 0x38588: beq 0x385b8 <entry+0x312c>  @ imm = #0x28
    c.b(C::Eq,0x385b8)?;
    // 0x3858c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38590: cmp r8, #960
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,960)?)?;
    // 0x38594: mov r7, r8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // 0x38598: strb r0, [r4, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,332)?)?;
    // 0x3859c: str r8, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,288)?)?;
    // 0x385a0: str r0, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,300)?)?;
    // 0x385a4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x385a8: strb r0, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,350)?)?;
    // 0x385ac: movhi r0, #32
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x385b0: strhi r0, [r10]
    c.emit(arm32::transfer_word(C::Hi,false,R::R0,R::new(10)?,0)?)?;
    // 0x385b4: b 0x385d8 <entry+0x314c>  @ imm = #0x1c
    c.b(C::Al,0x385d8)?;
    // 0x385b8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x385bc: bne 0x385d8 <entry+0x314c>  @ imm = #0x14
    c.b(C::Ne,0x385d8)?;
    // 0x385c0: ldrb r0, [r4, #0x69c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1692)?)?;
    // 0x385c4: ldr r10, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,12)?)?;
    // 0x385c8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x385cc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x385d0: strne r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,24)?)?;
    // 0x385d4: b 0x385dc <entry+0x3150>  @ imm = #0x0
    c.b(C::Al,0x385dc)?;
    // 0x385d8: ldr r10, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,12)?)?;
    // 0x385dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x385e0: cmp r7, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x385e4: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x385e8: cmp r7, #960
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,960)?)?;
    // 0x385ec: strb r0, [r4, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,332)?)?;
    // 0x385f0: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // 0x385f4: movhi r0, #32
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x385f8: str r0, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,404)?)?;
    // 0x385fc: ldrb r0, [r4, #0x13b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,315)?)?;
    // 0x38600: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38604: bne 0x38624 <entry+0x3198>  @ imm = #0x18
    c.b(C::Ne,0x38624)?;
    // 0x38608: ldrb r0, [r4, #0x13a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,314)?)?;
    // 0x3860c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x38610: ldrbne r0, [r4, #0x139]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(4)?,313)?)?;
    // 0x38614: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x38618: bne 0x38624 <entry+0x3198>  @ imm = #0x4
    c.b(C::Ne,0x38624)?;
    // 0x3861c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38620: strb r0, [r4, #0x13b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,315)?)?;
    // 0x38624: ldrb r0, [r4, #0x258]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,600)?)?;
    // 0x38628: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x3862c: bhi 0x38644 <entry+0x31b8>  @ imm = #0x10
    c.b(C::Hi,0x38644)?;
    // 0x38630: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x38634: mov r2, #194
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,194)?)?;
    // 0x38638: tst r2, r1, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::R0,false)?)?;
    // 0x3863c: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38640: strbne r0, [r4, #0x17c]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,380)?)?;
    // 0x38644: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,76)?)?;
    // 0x38648: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x3864c: add r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x38650: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x38654: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4b5f0
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x38658: str r0, [r4, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,304)?)?;
    // 0x3865c: add r0, r6, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x38660: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x38664: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x38668: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4b5dc
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x3866c: str r0, [r4, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,308)?)?;
    // 0x38670: ldrb r0, [r4, #0x159]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,345)?)?;
    // 0x38674: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38678: beq 0x38688 <entry+0x31fc>  @ imm = #0x8
    c.b(C::Eq,0x38688)?;
    // 0x3867c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38680: strb r0, [r4, #0x60b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1547)?)?;
    // 0x38684: b 0x38698 <entry+0x320c>  @ imm = #0xc
    c.b(C::Al,0x38698)?;
    // 0x38688: ldrb r1, [r4, #0x2c3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,707)?)?;
    // 0x3868c: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x38690: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // 0x38694: beq 0x386a8 <entry+0x321c>  @ imm = #0xc
    c.b(C::Eq,0x386a8)?;
    // 0x38698: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x3869c: strb r0, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,350)?)?;
    // 0x386a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x386a4: str r0, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,300)?)?;
    // 0x386a8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x386ac: cmp r7, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x386b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x386b4: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x386b8: strhs r1, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,300)?)?;
    // 0x386bc: cmp r9, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,4096)?)?;
    // 0x386c0: cmpne r9, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(9)?,2)?)?;
    // 0x386c4: bne 0x38704 <entry+0x3278>  @ imm = #0x38
    c.b(C::Ne,0x38704)?;
    // 0x386c8: cmp r7, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x386cc: bhs 0x38704 <entry+0x3278>  @ imm = #0x30
    c.b(C::Cs,0x38704)?;
    // 0x386d0: ldr r1, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,300)?)?;
    // 0x386d4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x386d8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x386dc: movne r2, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x386e0: strb r2, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,350)?)?;
    // 0x386e4: ldr r1, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,108)?)?;
    // 0x386e8: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x386ec: bne 0x38704 <entry+0x3278>  @ imm = #0x10
    c.b(C::Ne,0x38704)?;
    // 0x386f0: ldr r1, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,112)?)?;
    // 0x386f4: ldr r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    // 0x386f8: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x386fc: ldrlo r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Cc,true,R::R1,R::SP,8)?)?;
    // 0x38700: strlo r1, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Cc,false,R::R1,R::new(4)?,112)?)?;
    // 0x38704: ldrb r1, [r4, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,350)?)?;
    // 0x38708: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x3870c: ldreq r1, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(4)?,24)?)?;
    // 0x38710: cmpeq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x38714: beq 0x3893c <entry+0x34b0>  @ imm = #0x220
    c.b(C::Eq,0x3893c)?;
    // 0x38718: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3871c: strb r1, [r4, #0x626]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1574)?)?;
    // 0x38720: ldr r0, [r4, #0x22c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,556)?)?;
    // 0x38724: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38728: bne 0x38770 <entry+0x32e4>  @ imm = #0x40
    c.b(C::Ne,0x38770)?;
    // 0x3872c: ldr r0, [r4, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,560)?)?;
    // 0x38730: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38734: bne 0x38770 <entry+0x32e4>  @ imm = #0x34
    c.b(C::Ne,0x38770)?;
    // 0x38738: ldr r0, [r4, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,564)?)?;
    // 0x3873c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38740: bne 0x38770 <entry+0x32e4>  @ imm = #0x28
    c.b(C::Ne,0x38770)?;
    // 0x38744: ldr r0, [r4, #0x238]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,568)?)?;
    // 0x38748: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3874c: bne 0x38770 <entry+0x32e4>  @ imm = #0x1c
    c.b(C::Ne,0x38770)?;
    // 0x38750: ldr r1, [pc, #0x2dc]        @ 0x35a8 <entry+0x35a8>
    c.literal_cond(C::Al,R::R1,0x38a34)?;
    // 0x38754: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38758: str r0, [r4, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,560)?)?;
    // 0x3875c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38760: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x9ad4
    c.bl(0x2ec94)?;
    // 0x38764: ldr r1, [pc, #0x2a4]        @ 0x3584 <entry+0x3584>
    c.literal_cond(C::Al,R::R1,0x38a10)?;
    // 0x38768: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3876c: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x9ae0
    c.bl(0x2ec94)?;
    // 0x38770: ldrb r0, [r4, #0x660]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1632)?)?;
    // 0x38774: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38778: beq 0x387b4 <entry+0x3328>  @ imm = #0x34
    c.b(C::Eq,0x387b4)?;
    // 0x3877c: ldr r0, [r4, #0x22c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,556)?)?;
    // 0x38780: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38784: ldrne r0, [r4, #0x230]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,560)?)?;
    // 0x38788: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3878c: beq 0x387e0 <entry+0x3354>  @ imm = #0x4c
    c.b(C::Eq,0x387e0)?;
    // 0x38790: ldr r0, [r4, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,564)?)?;
    // 0x38794: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38798: ldrne r0, [r4, #0x238]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,568)?)?;
    // 0x3879c: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x387a0: beq 0x387e0 <entry+0x3354>  @ imm = #0x38
    c.b(C::Eq,0x387e0)?;
    // 0x387a4: ldr r1, [pc, #0x28c]        @ 0x35ac <entry+0x35ac>
    c.literal_cond(C::Al,R::R1,0x38a38)?;
    // 0x387a8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x387ac: str r0, [r4, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,568)?)?;
    // 0x387b0: b 0x387cc <entry+0x3340>  @ imm = #0x14
    c.b(C::Al,0x387cc)?;
    // 0x387b4: ldr r0, [r4, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,560)?)?;
    // 0x387b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x387bc: bne 0x387e0 <entry+0x3354>  @ imm = #0x1c
    c.b(C::Ne,0x387e0)?;
    // 0x387c0: ldr r1, [pc, #0x274]        @ 0x35b0 <entry+0x35b0>
    c.literal_cond(C::Al,R::R1,0x38a3c)?;
    // 0x387c4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x387c8: str r0, [r4, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,560)?)?;
    // 0x387cc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x387d0: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x9b44
    c.bl(0x2ec94)?;
    // 0x387d4: ldr r1, [pc, #0x234]        @ 0x3584 <entry+0x3584>
    c.literal_cond(C::Al,R::R1,0x38a10)?;
    // 0x387d8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x387dc: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x9b50
    c.bl(0x2ec94)?;
    // 0x387e0: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x387e4: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x387e8: bne 0x38808 <entry+0x337c>  @ imm = #0x18
    c.b(C::Ne,0x38808)?;
    // 0x387ec: ldr r1, [r4, #0x21c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,540)?)?;
    // 0x387f0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x387f4: bne 0x38840 <entry+0x33b4>  @ imm = #0x44
    c.b(C::Ne,0x38840)?;
    // 0x387f8: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x387fc: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38800: streq r0, [r4, #0x1a8]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,424)?)?;
    // 0x38804: b 0x38840 <entry+0x33b4>  @ imm = #0x34
    c.b(C::Al,0x38840)?;
    // 0x38808: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3880c: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x38810: str r1, [r4, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,556)?)?;
    // 0x38814: bne 0x38840 <entry+0x33b4>  @ imm = #0x24
    c.b(C::Ne,0x38840)?;
    // 0x38818: str r1, [r4, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,564)?)?;
    // 0x3881c: strb r1, [r4, #0x608]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1544)?)?;
    // 0x38820: ldrb r0, [r4, #0x23c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,572)?)?;
    // 0x38824: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38828: bne 0x38838 <entry+0x33ac>  @ imm = #0x8
    c.b(C::Ne,0x38838)?;
    // 0x3882c: ldrb r0, [r4, #0x68]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // 0x38830: tst r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,32)?)?;
    // 0x38834: beq 0x38840 <entry+0x33b4>  @ imm = #0x4
    c.b(C::Eq,0x38840)?;
    // 0x38838: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3883c: strb r0, [r4, #0x608]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1544)?)?;
    // 0x38840: ldr r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,80)?)?;
    // 0x38844: cmp r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,128)?)?;
    // 0x38848: bhi 0x38864 <entry+0x33d8>  @ imm = #0x14
    c.b(C::Hi,0x38864)?;
    // 0x3884c: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x38850: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x38854: cmp r0, #33
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,33)?)?;
    // 0x38858: ldrge r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Ge,true,R::R1,R::SP,16)?)?;
    // 0x3885c: movge r0, #32
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x38860: strge r0, [r1]
    c.emit(arm32::transfer_word(C::Ge,false,R::R0,R::R1,0)?)?;
    // 0x38864: ldr r2, [pc, #0x1d4]        @ 0x35b4 <entry+0x35b4>
    c.literal_cond(C::Al,R::R2,0x38a40)?;
    // 0x38868: mov r1, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,182)?)?;
    // 0x3886c: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // 0x38870: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38874: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x38878: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3887c: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2816)?)?;
    // 0x38880: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x370d8
    c.bl(0x17b0)?;
    // 0x38884: ldrb r0, [r4, #0x15d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,349)?)?;
    // 0x38888: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x3888c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x38890: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38894: streq r5, [r4, #0x318]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::new(4)?,792)?)?;
    // 0x38898: streq r5, [r4, #0x31c]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::new(4)?,796)?)?;
    // 0x3889c: strb r0, [r4, #0x169]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,361)?)?;
    // 0x388a0: strb r0, [r4, #0x14b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,331)?)?;
    // 0x388a4: strb r5, [r4, #0x15d]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,349)?)?;
    // 0x388a8: str r5, [r4, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,376)?)?;
    // 0x388ac: strb r1, [r4, #0x165]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,357)?)?;
    // 0x388b0: ldr r1, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,52)?)?;
    // 0x388b4: str r1, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,32)?)?;
    // 0x388b8: ldr r2, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,56)?)?;
    // 0x388bc: mul r3, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // 0x388c0: str r2, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,36)?)?;
    // 0x388c4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x388c8: cmp r3, #9437184
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,9437184)?)?;
    // 0x388cc: strbhi r0, [r4, #0x165]
    c.emit(arm32::transfer_byte(C::Hi,false,R::R0,R::new(4)?,357)?)?;
    // 0x388d0: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x388d4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x388d8: cmpne r0, #8192
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x388dc: beq 0x388fc <entry+0x3470>  @ imm = #0x18
    c.b(C::Eq,0x388fc)?;
    // 0x388e0: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x388e4: bne 0x38900 <entry+0x3474>  @ imm = #0x14
    c.b(C::Ne,0x38900)?;
    // 0x388e8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x388ec: str r1, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,284)?)?;
    // 0x388f0: ldrb r1, [r4, #0xb8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,184)?)?;
    // 0x388f4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x388f8: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x388fc: strb r1, [r4, #0xb8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,184)?)?;
    // 0x38900: ldr r1, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,300)?)?;
    // 0x38904: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x38908: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3890c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x38910: strb r1, [r4, #0x170]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,368)?)?;
    // 0x38914: ldrb r1, [r4, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,332)?)?;
    // 0x38918: strb r1, [r4, #0x14d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,333)?)?;
    // 0x3891c: beq 0x3892c <entry+0x34a0>  @ imm = #0x8
    c.b(C::Eq,0x3892c)?;
    // 0x38920: ldrb r0, [r4, #0x159]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,345)?)?;
    // 0x38924: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38928: beq 0x38934 <entry+0x34a8>  @ imm = #0x4
    c.b(C::Eq,0x38934)?;
    // 0x3892c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38930: strb r0, [r4, #0x170]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,368)?)?;
    // 0x38934: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x38938: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x3893c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x38940: cmp r9, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,8192)?)?;
    // 0x38944: bne 0x38954 <entry+0x34c8>  @ imm = #0x8
    c.b(C::Ne,0x38954)?;
    // 0x38948: cmp r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,12)?)?;
    // 0x3894c: bls 0x38718 <entry+0x328c>  @ imm = #-0x23c
    c.b(C::Ls,0x38718)?;
    // 0x38950: b 0x3871c <entry+0x3290>  @ imm = #-0x23c
    c.b(C::Al,0x3871c)?;
    // 0x38954: cmp r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,12)?)?;
    // 0x38958: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x3895c: movhi r2, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x38960: tst r2, r0
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x38964: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x38968: b 0x3871c <entry+0x3290>  @ imm = #-0x254
    c.b(C::Al,0x3871c)?;
    // 0x3896c: ldrb r1, [r4, #0x250]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,592)?)?;
    // 0x38970: mov r7, #960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,960)?)?;
    // 0x38974: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x38978: bne 0x389c8 <entry+0x353c>  @ imm = #0x48
    c.b(C::Ne,0x389c8)?;
    // 0x3897c: ldrb r1, [r4, #0x2b5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,693)?)?;
    // 0x38980: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x38984: bne 0x389c4 <entry+0x3538>  @ imm = #0x38
    c.b(C::Ne,0x389c4)?;
    // 0x38988: ldrb r1, [r4, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,348)?)?;
    // 0x3898c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x38990: beq 0x389a0 <entry+0x3514>  @ imm = #0x8
    c.b(C::Eq,0x389a0)?;
    // 0x38994: ldrb r1, [r4, #0xc8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,200)?)?;
    // 0x38998: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x3899c: blo 0x389c4 <entry+0x3538>  @ imm = #0x20
    c.b(C::Cc,0x389c4)?;
    // 0x389a0: ldr r1, [r4, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,696)?)?;
    // 0x389a4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x389a8: bne 0x389c4 <entry+0x3538>  @ imm = #0x14
    c.b(C::Ne,0x389c4)?;
    // 0x389ac: ldr r1, [r4, #0x2bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,700)?)?;
    // 0x389b0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x389b4: ldreq r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::SP,4)?)?;
    // 0x389b8: ldreq r1, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R1,40)?)?;
    // 0x389bc: orrseq r1, r1, r6
    c.emit(arm32::data_register(C::Eq,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,true)?)?;
    // 0x389c0: beq 0x389c8 <entry+0x353c>  @ imm = #0x0
    c.b(C::Eq,0x389c8)?;
    // 0x389c4: mov r7, #768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,768)?)?;
    // 0x389c8: str r7, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,288)?)?;
    // 0x389cc: ldr r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,80)?)?;
    // 0x389d0: cmp r1, #720
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,720)?)?;
    // 0x389d4: blo 0x384e4 <entry+0x3058>  @ imm = #-0x4f8
    c.b(C::Cc,0x384e4)?;
    // 0x389d8: sub r1, r8, #960
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(8)?,960)?)?;
    // 0x389dc: lsr r1, r1, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,6)?)?;
    // 0x389e0: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x389e4: bhi 0x389f8 <entry+0x356c>  @ imm = #0xc
    c.b(C::Hi,0x389f8)?;
    // 0x389e8: mov r7, #672
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,672)?)?;
    // 0x389ec: b 0x38308 <entry+0x2e7c>  @ imm = #-0x6ec
    c.b(C::Al,0x38308)?;
    // 0x389f0: mov r7, #960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,960)?)?;
    // 0x389f4: b 0x38308 <entry+0x2e7c>  @ imm = #-0x6f4
    c.b(C::Al,0x38308)?;
    // 0x389f8: sub r1, r8, #736
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(8)?,736)?)?;
    // 0x389fc: cmp r1, #223
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,223)?)?;
    // 0x38a00: bls 0x38304 <entry+0x2e78>  @ imm = #-0x704
    c.b(C::Ls,0x38304)?;
    // 0x38a04: b 0x384e4 <entry+0x3058>  @ imm = #-0x528
    c.b(C::Al,0x384e4)?;
    c.check(0x38a08)?;
    for value in FORMAT_CONTROL_LITERALS{c.emit(value)?;}
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{select_format_parameters(c)}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn typed_format_setup_exact_span_and_literal_pool(){
        let mut c=Code::new(START);
        emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        assert_eq!(&c.bytes[0x38a08-START..],
            &FORMAT_CONTROL_LITERALS.iter().flat_map(|w|w.to_le_bytes()).collect::<Vec<_>>());
    }
    #[test]fn checked_signed_halfword_indexed_modes(){
        let r1=R::R1;let r2=R::R2;let r4=R::new(4).unwrap();
        assert_eq!(encode_signed_halfword_register_writeback(C::Al,r2,r4,r1,false).unwrap(),0xe19420f1);
        assert_eq!(encode_signed_halfword_register_writeback(C::Al,R::R0,r1,R::R0,true).unwrap(),0xe1b100f0);
        assert!(encode_signed_halfword_register_writeback(C::Al,r1,r1,r2,true).is_err());
    }
}
