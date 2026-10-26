// SPDX-License-Identifier: GPL-3.0-only
//! Codec hardware controller selection and channel-buffer configuration;
//! follows with a short counter-update and descriptor-accessor routine.
//! Two large PC-relative diagnostic arrays are distinctly non-executable
//! literal data, not raw reference instructions.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x38a44;
const END:usize=0x38f70;
const INIT_DEVICE_DIAGNOSTICS:[u32;9]=[
    0x000e_db00,0x000e_db50,0x000e_dba0,0x000e_dc40,
    0x000e_dc90,0x000e_dcc4,0x000e_dcf8,0x000e_c0b3,
    0x000e_dbf0,
];
const REGISTER_FORMAT_DIAGNOSTICS:[u32;29]=[
    0x000e_c0b3,0x000e_df6c,0x000e_df84,0x000e_df9c,
    0x000e_dfb4,0x000e_de34,0x000e_de4c,0x000e_de64,
    0x000e_de7c,0x000e_de94,0x000e_deac,0x000e_dec4,
    0x000e_dedc,0x000e_df24,0x000e_df3c,0x000e_df54,
    0x000e_def4,0x000e_df0c,0x000e_dd2c,0x000e_dd44,
    0x000e_dd5c,0x000e_dd74,0x000e_dd8c,0x000e_dda4,
    0x000e_ddbc,0x000e_ddd4,0x000e_ddec,0x000e_de04,
    0x000e_de1c,
];
fn encode_halfword_register(c:C,load:bool,dst:R,base:R,index:R,wb:bool)->Result<u32,String>{
    if [dst,base,index].contains(&R::PC)||(load && wb && dst==base){
        return Err("invalid indexed halfword transfer".into());
    }
    Ok(((c as u32)<<28)|0x0180_00b0|(if load{1<<20}else{0})
       |(if wb{1<<21}else{0})|(base.number()<<16)
       |(dst.number()<<12)|index.number())
}
/// Publish codec format/resource pointers; the first function returns with
/// saved registers restored, then its nine-word debug pool follows.
fn initialize_codec_parameters(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x38a44: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x38a48: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x38a4c: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // 0x38a50: ldr r3, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,4)?)?;
    // 0x38a54: cmp r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4096)?)?;
    // 0x38a58: bne 0x38a68 <entry+0x35dc>  @ imm = #0x8
    c.b(C::Ne,0x38a68)?;
    // 0x38a5c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x38a60: str r4, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,12)?)?;
    // 0x38a64: b 0x38bb4 <entry+0x3728>  @ imm = #0x148
    c.b(C::Al,0x38bb4)?;
    // 0x38a68: ldr r7, [r1, #0x7dc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,2012)?)?;
    // 0x38a6c: ldr r6, [r1, #0x7e0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,2016)?)?;
    // 0x38a70: ldrb r5, [r1, #0x165]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R1,357)?)?;
    // 0x38a74: ldr r1, [r0, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,96)?)?;
    // 0x38a78: mul r10, r6, r7
    c.emit(arm32::multiply_register(C::Al,R::new(10)?,R::new(6)?,R::new(7)?)?)?;
    // 0x38a7c: ldr r7, [r0, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,104)?)?;
    // 0x38a80: mul r2, r10, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::new(10)?,R::R1)?)?;
    // 0x38a84: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x38a88: ldrne r0, [r0, #0x64]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::R0,100)?)?;
    // 0x38a8c: strne r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::SP,24)?)?;
    // 0x38a90: streq r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::SP,24)?)?;
    // 0x38a94: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // 0x38a98: bne 0x38ac0 <entry+0x3634>  @ imm = #0x20
    c.b(C::Ne,0x38ac0)?;
    // 0x38a9c: ldr r4, [pc, #0x11c]        @ 0x3734 <entry+0x3734>
    c.literal_cond(C::Al,R::new(4)?,0x38bc0)?;
    // 0x38aa0: ldr lr, [pc, #0x11c]        @ 0x3738 <entry+0x3738>
    c.literal_cond(C::Al,R::LR,0x38bc4)?;
    // 0x38aa4: ldr r6, [pc, #0x11c]        @ 0x373c <entry+0x373c>
    c.literal_cond(C::Al,R::new(6)?,0x38bc8)?;
    // 0x38aa8: ldr r7, [pc, #0x11c]        @ 0x3740 <entry+0x3740>
    c.literal_cond(C::Al,R::new(7)?,0x38bcc)?;
    // 0x38aac: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x38ab0: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x38ab4: mov r9, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,20)?)?;
    // 0x38ab8: mov r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,65536)?)?;
    // 0x38abc: b 0x38ae8 <entry+0x365c>  @ imm = #0x24
    c.b(C::Al,0x38ae8)?;
    // 0x38ac0: cmp r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8192)?)?;
    // 0x38ac4: bne 0x38b94 <entry+0x3708>  @ imm = #0xc8
    c.b(C::Ne,0x38b94)?;
    // 0x38ac8: ldr r4, [pc, #0x100]        @ 0x3744 <entry+0x3744>
    c.literal_cond(C::Al,R::new(4)?,0x38bd0)?;
    // 0x38acc: ldr r6, [pc, #0x100]        @ 0x3748 <entry+0x3748>
    c.literal_cond(C::Al,R::new(6)?,0x38bd4)?;
    // 0x38ad0: ldr r7, [pc, #0x100]        @ 0x374c <entry+0x374c>
    c.literal_cond(C::Al,R::new(7)?,0x38bd8)?;
    // 0x38ad4: mov r9, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,13)?)?;
    // 0x38ad8: mov lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // 0x38adc: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x38ae0: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x38ae4: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x38ae8: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x38aec: mul r0, r10, r5
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(10)?,R::new(5)?)?)?;
    // 0x38af0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x38af4: str lr, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,12)?)?;
    // 0x38af8: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x38afc: ldr r0, [pc, #0xdc]         @ 0x3754 <entry+0x3754>
    c.literal_cond(C::Al,R::R0,0x38be0)?;
    // 0x38b00: ldr r2, [r6], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R2,R::new(6)?,4)?)?;
    // 0x38b04: cmp r10, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x38b08: bhi 0x38b70 <entry+0x36e4>  @ imm = #0x60
    c.b(C::Hi,0x38b70)?;
    // 0x38b0c: ldr r2, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,0)?)?;
    // 0x38b10: mov r1, #1000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1000)?)?;
    // 0x38b14: mul r3, r2, r1
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R1)?)?;
    // 0x38b18: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // 0x38b1c: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x38b20: bhi 0x38b70 <entry+0x36e4>  @ imm = #0x48
    c.b(C::Hi,0x38b70)?;
    // 0x38b24: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x38b28: ldr r3, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // 0x38b2c: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x38b30: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x38b34: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x38b38: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x38b3c: movls r8, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x38b40: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x38b44: moveq r12, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x38b48: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x38b4c: andne r2, r12, r8
    c.emit(arm32::data_register(C::Ne,Op::And,R::R2,R::R12,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x38b50: cmpne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x38b54: bne 0x38b70 <entry+0x36e4>  @ imm = #0x14
    c.b(C::Ne,0x38b70)?;
    // 0x38b58: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // 0x38b5c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x38b60: ldrne r2, [lr]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::LR,0)?)?;
    // 0x38b64: ldrne r3, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::SP,8)?)?;
    // 0x38b68: cmpne r3, r2
    c.emit(arm32::data_register(C::Ne,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x38b6c: bls 0x38bb0 <entry+0x3724>  @ imm = #0x3c
    c.b(C::Ls,0x38bb0)?;
    // 0x38b70: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x38b74: add lr, lr, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::LR,4)?)?;
    // 0x38b78: add r7, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,4)?)?;
    // 0x38b7c: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x38b80: add r4, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4)?)?;
    // 0x38b84: cmp r5, r9
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x38b88: blo 0x38b00 <entry+0x3674>  @ imm = #-0x90
    c.b(C::Cc,0x38b00)?;
    // 0x38b8c: ldr r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,4)?)?;
    // 0x38b90: b 0x38bb4 <entry+0x3728>  @ imm = #0x1c
    c.b(C::Al,0x38bb4)?;
    // 0x38b94: ldr r2, [pc, #0x40]         @ 0x3750 <entry+0x3750>
    c.literal_cond(C::Al,R::R2,0x38bdc)?;
    // 0x38b98: mov r1, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,25)?)?;
    // 0x38b9c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38ba0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x38ba4: orr r1, r1, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    // 0x38ba8: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x37400
    c.bl(0x17b0)?;
    // 0x38bac: b 0x38bb4 <entry+0x3728>  @ imm = #0x0
    c.b(C::Al,0x38bb4)?;
    // 0x38bb0: ldr r4, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,0)?)?;
    // 0x38bb4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x38bb8: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x38bbc: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x38bc0)?;
    for word in INIT_DEVICE_DIAGNOSTICS{c.emit(word)?;}
    c.check(0x38be4)?;
    Ok(())
}

/// Populate three codec register sets, validating input controller class.
/// A 29-word diagnostic table follows the original return at 0x38e48.
fn load_codec_register_tables(c:&mut Code)->Result<(),String>{
    c.check(0x38be4)?;
    // 0x38be4: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x38be8: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x38bec: mov r7, r2
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // 0x38bf0: ldr r2, [pc, #0x254]        @ 0x39c0 <entry+0x39c0>
    c.literal_cond(C::Al,R::R2,0x38e4c)?;
    // 0x38bf4: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x38bf8: mov r1, #99
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,99)?)?;
    // 0x38bfc: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x38c00: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38c04: mov r5, r3
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // 0x38c08: cmp r6, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,6)?)?;
    // 0x38c0c: orr r1, r1, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    // 0x38c10: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x38c14: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x3746c
    c.bl(0x17b0)?;
    // 0x38c18: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x38c1c: beq 0x38cdc <entry+0x3850>  @ imm = #0xb8
    c.b(C::Eq,0x38cdc)?;
    // 0x38c20: cmp r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4096)?)?;
    // 0x38c24: beq 0x38ddc <entry+0x3950>  @ imm = #0x1b0
    c.b(C::Eq,0x38ddc)?;
    // 0x38c28: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x38c2c: bne 0x38e10 <entry+0x3984>  @ imm = #0x1dc
    c.b(C::Ne,0x38e10)?;
    // 0x38c30: ldr r0, [pc, #0x25c]        @ 0x3a08 <entry+0x3a08>
    c.literal_cond(C::Al,R::R0,0x38e94)?;
    // 0x38c34: ldr r1, [pc, #0x260]        @ 0x3a10 <entry+0x3a10>
    c.literal_cond(C::Al,R::R1,0x38e9c)?;
    // 0x38c38: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38c3c: ldr r1, [r1, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38c40: str r0, [r4, #0x340]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,832)?)?;
    // 0x38c44: ldr r0, [pc, #0x24c]        @ 0x3a0c <entry+0x3a0c>
    c.literal_cond(C::Al,R::R0,0x38e98)?;
    // 0x38c48: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38c4c: str r0, [r4, #0x344]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,836)?)?;
    // 0x38c50: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38c54: str r0, [r4, #0x348]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,840)?)?;
    // 0x38c58: str r1, [r4, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,848)?)?;
    // 0x38c5c: ldr r1, [pc, #0x23c]        @ 0x3a14 <entry+0x3a14>
    c.literal_cond(C::Al,R::R1,0x38ea0)?;
    // 0x38c60: ldr r1, [r1, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38c64: str r1, [r4, #0x354]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,852)?)?;
    // 0x38c68: str r0, [r4, #0x358]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,856)?)?;
    // 0x38c6c: ldr r0, [pc, #0x230]        @ 0x3a18 <entry+0x3a18>
    c.literal_cond(C::Al,R::R0,0x38ea4)?;
    // 0x38c70: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38c74: str r0, [r4, #0x360]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,864)?)?;
    // 0x38c78: ldr r0, [pc, #0x228]        @ 0x3a1c <entry+0x3a1c>
    c.literal_cond(C::Al,R::R0,0x38ea8)?;
    // 0x38c7c: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38c80: str r0, [r4, #0x364]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,868)?)?;
    // 0x38c84: ldr r0, [pc, #0x220]        @ 0x3a20 <entry+0x3a20>
    c.literal_cond(C::Al,R::R0,0x38eac)?;
    // 0x38c88: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38c8c: str r0, [r4, #0x368]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,872)?)?;
    // 0x38c90: ldr r0, [pc, #0x218]        @ 0x3a24 <entry+0x3a24>
    c.literal_cond(C::Al,R::R0,0x38eb0)?;
    // 0x38c94: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38c98: strb r0, [r4, #0x49e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1182)?)?;
    // 0x38c9c: ldr r0, [pc, #0x210]        @ 0x3a28 <entry+0x3a28>
    c.literal_cond(C::Al,R::R0,0x38eb4)?;
    // 0x38ca0: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38ca4: str r0, [r4, #0xc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,200)?)?;
    // 0x38ca8: ldr r0, [pc, #0x208]        @ 0x3a2c <entry+0x3a2c>
    c.literal_cond(C::Al,R::R0,0x38eb8)?;
    // 0x38cac: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38cb0: str r0, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,196)?)?;
    // 0x38cb4: ldr r0, [pc, #0x200]        @ 0x3a30 <entry+0x3a30>
    c.literal_cond(C::Al,R::R0,0x38ebc)?;
    // 0x38cb8: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38cbc: str r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,192)?)?;
    // 0x38cc0: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x38cc4: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x38cc8: cmpne r0, #64
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,64)?)?;
    // 0x38ccc: beq 0x38e0c <entry+0x3980>  @ imm = #0x138
    c.b(C::Eq,0x38e0c)?;
    // 0x38cd0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38cd4: str r0, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,196)?)?;
    // 0x38cd8: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x38cdc: ldr r0, [pc, #0x17c]        @ 0x39d4 <entry+0x39d4>
    c.literal_cond(C::Al,R::R0,0x38e60)?;
    // 0x38ce0: ldr r1, [pc, #0x17c]        @ 0x39d8 <entry+0x39d8>
    c.literal_cond(C::Al,R::R1,0x38e64)?;
    // 0x38ce4: ldr r2, [pc, #0x17c]        @ 0x39dc <entry+0x39dc>
    c.literal_cond(C::Al,R::R2,0x38e68)?;
    // 0x38ce8: ldr r3, [pc, #0x17c]        @ 0x39e0 <entry+0x39e0>
    c.literal_cond(C::Al,R::new(3)?,0x38e6c)?;
    // 0x38cec: add r12, r4, #864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(4)?,864)?)?;
    // 0x38cf0: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38cf4: ldr r1, [r1, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38cf8: ldr r2, [r2, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38cfc: ldr r3, [r3, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38d00: str r0, [r4, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,848)?)?;
    // 0x38d04: str r1, [r4, #0x354]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,852)?)?;
    // 0x38d08: str r2, [r4, #0x358]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,856)?)?;
    // 0x38d0c: str r3, [r4, #0x35c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,860)?)?;
    // 0x38d10: stm r12, {r0, r1, r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R0.bit() | R::R1.bit() | R::R2.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x38d14: ldr r0, [pc, #0x154]        @ 0x39e4 <entry+0x39e4>
    c.literal_cond(C::Al,R::R0,0x38e70)?;
    // 0x38d18: ldr r1, [pc, #0x158]        @ 0x39ec <entry+0x39ec>
    c.literal_cond(C::Al,R::R1,0x38e78)?;
    // 0x38d1c: ldr r2, [pc, #0x158]        @ 0x39f0 <entry+0x39f0>
    c.literal_cond(C::Al,R::R2,0x38e7c)?;
    // 0x38d20: add r3, r4, #900
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,900)?)?;
    // 0x38d24: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38d28: ldr r1, [r1, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38d2c: ldr r2, [r2, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38d30: str r0, [r4, #0x370]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,880)?)?;
    // 0x38d34: ldr r0, [pc, #0x138]        @ 0x39e8 <entry+0x39e8>
    c.literal_cond(C::Al,R::R0,0x38e74)?;
    // 0x38d38: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38d3c: str r0, [r4, #0x378]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,888)?)?;
    // 0x38d40: str r1, [r4, #0x37c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,892)?)?;
    // 0x38d44: str r2, [r4, #0x380]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,896)?)?;
    // 0x38d48: stm r3, {r0, r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit() | R::R1.bit() | R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x38d4c: add r3, r4, #912
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,912)?)?;
    // 0x38d50: stm r3, {r0, r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit() | R::R1.bit() | R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x38d54: ldr r0, [pc, #0x124]        @ 0x39f4 <entry+0x39f4>
    c.literal_cond(C::Al,R::R0,0x38e80)?;
    // 0x38d58: mov r1, #248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,248)?)?;
    // 0x38d5c: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x38d60: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38d64: str r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,92)?)?;
    // 0x38d68: ldr r0, [pc, #0x114]        @ 0x39f8 <entry+0x39f8>
    c.literal_cond(C::Al,R::R0,0x38e84)?;
    // 0x38d6c: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38d70: str r0, [r4, #0x4f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1268)?)?;
    // 0x38d74: ldr r0, [pc, #0x10c]        @ 0x39fc <entry+0x39fc>
    c.literal_cond(C::Al,R::R0,0x38e88)?;
    // 0x38d78: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38d7c: strh r0, [r4, r1]
    c.emit(encode_halfword_register(C::Al,false,R::R0,R::new(4)?,R::R1,false)?)?;
    // 0x38d80: ldr r0, [pc, #0x104]        @ 0x3a00 <entry+0x3a00>
    c.literal_cond(C::Al,R::R0,0x38e8c)?;
    // 0x38d84: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38d88: str r0, [r4, #0xc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,200)?)?;
    // 0x38d8c: ldr r0, [pc, #0xfc]         @ 0x3a04 <entry+0x3a04>
    c.literal_cond(C::Al,R::R0,0x38e90)?;
    // 0x38d90: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38d94: str r0, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,196)?)?;
    // 0x38d98: str r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,192)?)?;
    // 0x38d9c: str r0, [r4, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,188)?)?;
    // 0x38da0: ldrb r0, [r5, #0xca]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,202)?)?;
    // 0x38da4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x38da8: popeq {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x38dac: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // 0x38db0: ldr r1, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,36)?)?;
    // 0x38db4: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // 0x38db8: mov r0, #1040384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1040384)?)?;
    // 0x38dbc: orr r0, r0, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1048576)?)?;
    // 0x38dc0: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x38dc4: bhi 0x38e28 <entry+0x399c>  @ imm = #0x5c
    c.b(C::Hi,0x38e28)?;
    // 0x38dc8: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x38dcc: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x38dd0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x38dd4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x38dd8: b 0x38e38 <entry+0x39ac>  @ imm = #0x58
    c.b(C::Al,0x38e38)?;
    // 0x38ddc: ldr r0, [pc, #0x6c]         @ 0x39c4 <entry+0x39c4>
    c.literal_cond(C::Al,R::R0,0x38e50)?;
    // 0x38de0: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38de4: str r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // 0x38de8: ldr r0, [pc, #0x64]         @ 0x39c8 <entry+0x39c8>
    c.literal_cond(C::Al,R::R0,0x38e54)?;
    // 0x38dec: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38df0: str r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,36)?)?;
    // 0x38df4: ldr r0, [pc, #0x5c]         @ 0x39cc <entry+0x39cc>
    c.literal_cond(C::Al,R::R0,0x38e58)?;
    // 0x38df8: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38dfc: str r0, [r4, #0xc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,200)?)?;
    // 0x38e00: ldr r0, [pc, #0x54]         @ 0x39d0 <entry+0x39d0>
    c.literal_cond(C::Al,R::R0,0x38e5c)?;
    // 0x38e04: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x38e08: strb r0, [r4, #0x49c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1180)?)?;
    // 0x38e0c: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x38e10: ldr r2, [pc, #0x34]         @ 0x39c0 <entry+0x39c0>
    c.literal_cond(C::Al,R::R2,0x38e4c)?;
    // 0x38e14: mov r1, #195
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,195)?)?;
    // 0x38e18: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38e1c: orr r1, r1, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    // 0x38e20: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x38e24: b 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x3767c
    c.b(C::Al,0x17b0)?;
    // 0x38e28: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x38e2c: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x38e30: mov r3, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,127)?)?;
    // 0x38e34: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x38e38: str r3, [r4, #0x340]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,832)?)?;
    // 0x38e3c: str r1, [r4, #0x344]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,836)?)?;
    // 0x38e40: str r0, [r4, #0x348]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,840)?)?;
    // 0x38e44: str r2, [r4, #0x34c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,844)?)?;
    // 0x38e48: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x38e4c)?;
    for word in REGISTER_FORMAT_DIAGNOSTICS{c.emit(word)?;}
    c.check(0x38ec0)?;
    Ok(())
}

/// Call subordinate codec buffer helper and atomically update 64-bit
/// descriptor counter using explicit carry-propagating ARM instructions.
fn update_codec_counter(c:&mut Code)->Result<(),String>{
    c.check(0x38ec0)?;
    // 0x38ec0: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x38ec4: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x38ec8: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x38ecc: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // 0x38ed0: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x38ed4: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x38ed8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x38edc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x38ee0: bl 0x38f34 <entry+0x3aa8>  @ imm = #0x4c
    c.bl(0x38f34)?;
    // 0x38ee4: ldr r2, [r7, #0x6b8]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R2,R::new(7)?,1720)?)?;
    // 0x38ee8: ldr r1, [r6, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,420)?)?;
    // 0x38eec: ldr r3, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,4)?)?;
    // 0x38ef0: adds r2, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R1,Shift::Lsl,0,true)?)?;
    // 0x38ef4: adc r3, r3, #0
    c.emit(super::encode_adc_zero(C::Al,R::new(3)?,R::new(3)?)?)?;
    // 0x38ef8: subs r6, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::R0,R::R2,Shift::Lsl,0,true)?)?;
    // 0x38efc: rscs r6, r3, #0
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Rsc,R::new(6)?,R::new(3)?,0,true)?)?;
    // 0x38f00: blo 0x38f2c <entry+0x3aa0>  @ imm = #0x24
    c.b(C::Cc,0x38f2c)?;
    // 0x38f04: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x38f08: mov r5, r3
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // 0x38f0c: adds r2, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R1,Shift::Lsl,0,true)?)?;
    // 0x38f10: adc r3, r5, #0
    c.emit(super::encode_adc_zero(C::Al,R::new(3)?,R::new(5)?)?)?;
    // 0x38f14: subs r4, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::R0,R::R2,Shift::Lsl,0,true)?)?;
    // 0x38f18: rscs r4, r3, #0
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Rsc,R::new(4)?,R::new(3)?,0,true)?)?;
    // 0x38f1c: bhs 0x38f04 <entry+0x3a78>  @ imm = #-0x20
    c.b(C::Cs,0x38f04)?;
    // 0x38f20: str r6, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,0)?)?;
    // 0x38f24: str r5, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,4)?)?;
    // 0x38f28: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x38f2c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x38f30: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x38f34: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x38f38: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x38f3c: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x38f40: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x38f44: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x38f48: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x3635c
    c.bl(0x2bf4)?;
    // 0x38f4c: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x38f50: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x38f54: ldr r1, [r1, r4, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(4)?,Shift::Lsl,2)?)?;
    // 0x38f58: ldr r4, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,16)?)?;
    // 0x38f5c: ldr r5, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,20)?)?;
    // 0x38f60: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x36334
    c.bl(0x2c34)?;
    // 0x38f64: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x38f68: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x38f6c: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    initialize_codec_parameters(c)?;
    load_codec_register_tables(c)?;
    update_codec_counter(c)?;
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn three_routines_with_two_data_pools_are_contiguous(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let from=0x38bc0-START;let to=0x38be4-START;
        assert_eq!(&c.bytes[from..to],
            &INIT_DEVICE_DIAGNOSTICS.iter().flat_map(|w|w.to_le_bytes()).collect::<Vec<_>>());
        let from=0x38e4c-START;let to=0x38ec0-START;
        assert_eq!(&c.bytes[from..to],
            &REGISTER_FORMAT_DIAGNOSTICS.iter().flat_map(|w|w.to_le_bytes()).collect::<Vec<_>>());
    }
}
