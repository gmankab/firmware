// SPDX-License-Identifier: GPL-3.0-only
//! Codec controller allocation and callback resource accessor helpers.
//! Public ARM A32 operations are encoded with typed registers and separate
//! PC-relative diagnostic pointer words. This reproduces machine bytes, not
//! newly authored high-level algorithms or authenticated firmware.
use super::Code;
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
const START:usize=0x39490;
const END:usize=0x39bd0;
const CONTROLLER_RESOURCE_DIAGNOSTIC:u32=0x000e_c0b3;

/// Checked indexed signed-halfword load for signed codec plane coefficients.
fn encode_signed_halfword_register_writeback(c:C,dst:R,base:R,offset:R,wb:bool)->Result<u32,String>{
    if [dst,base,offset].contains(&R::PC)||(wb && dst==base){
        return Err("unsafe signed halfword index".into());
    }
    Ok(((c as u32)<<28)|0x0190_00f0|(if wb{1<<21}else{0})
       |(base.number()<<16)|(dst.number()<<12)|offset.number())
}

/// Compute codec controller frame state and size, with hardware coefficient
/// conversion and explicit saved-register return at 0x39968.
fn configure_codec_capture(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x39490: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x39494: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x39498: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x3949c: mov r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // 0x394a0: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // 0x394a4: mov r7, r2
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // 0x394a8: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x394ac: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x368c0
    c.bl(0x2bf4)?;
    // 0x394b0: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // 0x394b4: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x394b8: ldr r5, [pc, #0x4ac]        @ 0x44e0 <entry+0x44e0>
    c.literal_cond(C::Al,R::new(5)?,0x3996c)?;
    // 0x394bc: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x394c0: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x394c4: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x394c8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x394cc: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x394d0: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x394d4: orr r1, r1, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3840)?)?;
    // 0x394d8: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x37d30
    c.bl(0x17b0)?;
    // 0x394dc: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x394e0: ldr r1, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,20)?)?;
    // 0x394e4: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x394e8: ldr r6, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x394ec: mov r1, #776
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,776)?)?;
    // 0x394f0: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    // 0x394f4: ldr r0, [r6, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,168)?)?;
    // 0x394f8: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // 0x394fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x39500: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x39504: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x37d5c
    c.bl(0x17b0)?;
    // 0x39508: str r8, [r6, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,84)?)?;
    // 0x3950c: ldr r0, [r9, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,28)?)?;
    // 0x39510: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x39514: bne 0x39624 <entry+0x4198>  @ imm = #0x108
    c.b(C::Ne,0x39624)?;
    // 0x39518: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x3951c: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x39520: mov r2, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,60)?)?;
    // 0x39524: bl 0x471c <entry+0xfffffffffffcf290> @ imm = #-0x34e10
    c.bl(0x471c)?;
    // 0x39528: ldr r3, [r7, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,44)?)?;
    // 0x3952c: ldr r5, [r7, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,52)?)?;
    // 0x39530: ldr r0, [r10, #0x4f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1268)?)?;
    // 0x39534: ldr r1, [r10, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1272)?)?;
    // 0x39538: ldr r2, [r10, #0x508]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1288)?)?;
    // 0x3953c: add r3, r5, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x39540: mla r5, r1, r0, r3
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::R1,R::R0,R::new(3)?)?)?;
    // 0x39544: rsb r1, r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R2,0)?)?;
    // 0x39548: add r0, r2, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3954c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x39550: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x39554: str r0, [r6, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,60)?)?;
    // 0x39558: ldr r0, [r10, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,716)?)?;
    // 0x3955c: str r0, [r6, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,72)?)?;
    // 0x39560: ldr r0, [r10, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,720)?)?;
    // 0x39564: str r0, [r6, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,76)?)?;
    // 0x39568: str r8, [r6, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,64)?)?;
    // 0x3956c: str r8, [r6, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,68)?)?;
    // 0x39570: str r8, [r6, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,80)?)?;
    // 0x39574: str r8, [r6, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,88)?)?;
    // 0x39578: str r8, [r6, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,104)?)?;
    // 0x3957c: str r8, [r6, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,100)?)?;
    // 0x39580: add r0, r10, #1232
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,1232)?)?;
    // 0x39584: ldr r5, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,16)?)?;
    // 0x39588: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x3958c: str r5, [r6, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,92)?)?;
    // 0x39590: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x39594: ldr r4, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,20)?)?;
    // 0x39598: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x3959c: str r4, [r6, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,96)?)?;
    // 0x395a0: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x395a4: bl 0x35670 <entry+0x1e4>   @ imm = #-0x3f3c
    c.bl(0x35670)?;
    // 0x395a8: ldr r0, [r10, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1244)?)?;
    // 0x395ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x395b0: beq 0x395f0 <entry+0x4164>  @ imm = #0x38
    c.b(C::Eq,0x395f0)?;
    // 0x395b4: ldr r7, [r10, #0xd98]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,3480)?)?;
    // 0x395b8: str r7, [r6, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,84)?)?;
    // 0x395bc: ldrb r0, [r10, #0x3dd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,989)?)?;
    // 0x395c0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x395c4: bne 0x395f0 <entry+0x4164>  @ imm = #0x24
    c.b(C::Ne,0x395f0)?;
    // 0x395c8: mov r8, r5
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(5)?))?;
    // 0x395cc: ldr r5, [r10, #0xa64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,2660)?)?;
    // 0x395d0: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x395d4: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x395d8: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4a66c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x395dc: cmp r5, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x395e0: str r0, [r6, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,84)?)?;
    // 0x395e4: mov r5, r8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // 0x395e8: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x395ec: strhi r0, [r6, #0x54]
    c.emit(arm32::transfer_word(C::Hi,false,R::R0,R::new(6)?,84)?)?;
    // 0x395f0: ldr r0, [r10, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,744)?)?;
    // 0x395f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x395f8: bne 0x39834 <entry+0x43a8>  @ imm = #0x234
    c.b(C::Ne,0x39834)?;
    // 0x395fc: ldr r1, [r10, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,436)?)?;
    // 0x39600: cmn r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // 0x39604: beq 0x3978c <entry+0x4300>  @ imm = #0x180
    c.b(C::Eq,0x3978c)?;
    // 0x39608: cmp r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,255)?)?;
    // 0x3960c: bhi 0x397b4 <entry+0x4328>  @ imm = #0x1a0
    c.b(C::Hi,0x397b4)?;
    // 0x39610: str r1, [r6, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,108)?)?;
    // 0x39614: ldr r2, [r10, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,436)?)?;
    // 0x39618: str r2, [r6, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,112)?)?;
    // 0x3961c: ldr r0, [r10, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,436)?)?;
    // 0x39620: b 0x397d0 <entry+0x4344>  @ imm = #0x1a8
    c.b(C::Al,0x397d0)?;
    // 0x39624: ldr r0, [r7, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,28)?)?;
    // 0x39628: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3962c: bne 0x39640 <entry+0x41b4>  @ imm = #0xc
    c.b(C::Ne,0x39640)?;
    // 0x39630: ldr r0, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,44)?)?;
    // 0x39634: ldrb r0, [r0, #0x6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,6)?)?;
    // 0x39638: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x3963c: bne 0x3977c <entry+0x42f0>  @ imm = #0x138
    c.b(C::Ne,0x3977c)?;
    // 0x39640: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x39644: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x39648: mov r2, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,360)?)?;
    // 0x3964c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x39650: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x39654: ldr r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,4)?)?;
    // 0x39658: str r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // 0x3965c: ldr r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,8)?)?;
    // 0x39660: str r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // 0x39664: str r12, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(6)?,12)?)?;
    // 0x39668: ldr r1, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,12)?)?;
    // 0x3966c: str r1, [r6, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,92)?)?;
    // 0x39670: ldr r1, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,16)?)?;
    // 0x39674: str r1, [r6, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,96)?)?;
    // 0x39678: ldr r1, [r7, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,60)?)?;
    // 0x3967c: str r1, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,24)?)?;
    // 0x39680: str r12, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(6)?,28)?)?;
    // 0x39684: str r12, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(6)?,32)?)?;
    // 0x39688: ldr r1, [r7, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,44)?)?;
    // 0x3968c: str r1, [r6, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,36)?)?;
    // 0x39690: ldr r1, [r7, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,28)?)?;
    // 0x39694: str r1, [r6, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,40)?)?;
    // 0x39698: str r12, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(6)?,44)?)?;
    // 0x3969c: ldr r1, [r7, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,56)?)?;
    // 0x396a0: str r1, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,48)?)?;
    // 0x396a4: ldr r1, [r7, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,28)?)?;
    // 0x396a8: str r1, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,52)?)?;
    // 0x396ac: str r12, [r6, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(6)?,56)?)?;
    // 0x396b0: ldr r1, [r7, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,32)?)?;
    // 0x396b4: str r1, [r6, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,60)?)?;
    // 0x396b8: ldr r1, [r7, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,48)?)?;
    // 0x396bc: str r1, [r6, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,64)?)?;
    // 0x396c0: ldr r1, [r7, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,36)?)?;
    // 0x396c4: str r1, [r6, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,100)?)?;
    // 0x396c8: ldr r1, [r7, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,40)?)?;
    // 0x396cc: str r1, [r6, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,104)?)?;
    // 0x396d0: ldr r1, [r7, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,52)?)?;
    // 0x396d4: str r1, [r6, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,68)?)?;
    // 0x396d8: ldr r1, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,20)?)?;
    // 0x396dc: str r1, [r6, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,72)?)?;
    // 0x396e0: ldr r1, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,24)?)?;
    // 0x396e4: str r1, [r6, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,76)?)?;
    // 0x396e8: ldr r1, [r7, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,100)?)?;
    // 0x396ec: str r1, [r6, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,80)?)?;
    // 0x396f0: ldr r1, [r10, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,276)?)?;
    // 0x396f4: ldr r1, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // 0x396f8: str r1, [r6, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,88)?)?;
    // 0x396fc: ldr r1, [r10, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1244)?)?;
    // 0x39700: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x39704: beq 0x3971c <entry+0x4290>  @ imm = #0x10
    c.b(C::Eq,0x3971c)?;
    // 0x39708: ldr r1, [r7, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,96)?)?;
    // 0x3970c: str r1, [r6, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,84)?)?;
    // 0x39710: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x39714: ldreq r1, [r10, #0xd98]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(10)?,3480)?)?;
    // 0x39718: streq r1, [r6, #0x54]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(6)?,84)?)?;
    // 0x3971c: add r1, r7, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,64)?)?;
    // 0x39720: add r2, r6, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,108)?)?;
    // 0x39724: ldm r1!, {r0, r3, r4, r5}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R0.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x39728: stm r2!, {r0, r3, r4, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x3972c: ldm r1, {r0, r3, r4, r5}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R0.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x39730: stm r2, {r0, r3, r4, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x39734: add r0, r10, #1232
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,1232)?)?;
    // 0x39738: ldr r5, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(7)?,12)?)?;
    // 0x3973c: ldr r7, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,16)?)?;
    // 0x39740: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x39744: str r12, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,0)?)?;
    // 0x39748: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x3974c: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x39750: bl 0x35670 <entry+0x1e4>   @ imm = #-0x40e8
    c.bl(0x35670)?;
    // 0x39754: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x39758: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x3975c: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x39760: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x39764: bl 0x8187c <entry+0x4c3f0> @ imm = #0x48110
    c.bl(0x8187c)?;
    // 0x39768: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x3976c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x39770: str r5, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,16)?)?;
    // 0x39774: str r4, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,20)?)?;
    // 0x39778: b 0x39834 <entry+0x43a8>  @ imm = #0xb4
    c.b(C::Al,0x39834)?;
    // 0x3977c: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x39780: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x36b54
    c.bl(0x2c34)?;
    // 0x39784: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x39788: b 0x39960 <entry+0x44d4>  @ imm = #0x1d0
    c.b(C::Al,0x39960)?;
    // 0x3978c: mov r0, #756
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,756)?)?;
    // 0x39790: ldrsh r1, [r10, r0]
    c.emit(encode_signed_halfword_register_writeback(C::Al,R::R1,R::new(10)?,R::R0,false)?)?;
    // 0x39794: mov r0, #246
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,246)?)?;
    // 0x39798: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // 0x3979c: str r1, [r6, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,108)?)?;
    // 0x397a0: ldrsh r2, [r10, r0]
    c.emit(encode_signed_halfword_register_writeback(C::Al,R::R2,R::new(10)?,R::R0,false)?)?;
    // 0x397a4: mov r0, #760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,760)?)?;
    // 0x397a8: str r2, [r6, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,112)?)?;
    // 0x397ac: ldrsh r0, [r10, r0]
    c.emit(encode_signed_halfword_register_writeback(C::Al,R::R0,R::new(10)?,R::R0,false)?)?;
    // 0x397b0: b 0x397d0 <entry+0x4344>  @ imm = #0x18
    c.b(C::Al,0x397d0)?;
    // 0x397b4: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // 0x397b8: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // 0x397bc: str r1, [r6, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,108)?)?;
    // 0x397c0: ldr r0, [r10, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,436)?)?;
    // 0x397c4: and r2, r2, r0, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R0,Shift::Lsr,8,false)?)?;
    // 0x397c8: str r2, [r6, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,112)?)?;
    // 0x397cc: ldrb r0, [r10, #0x1b6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,438)?)?;
    // 0x397d0: str r0, [r6, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,116)?)?;
    // 0x397d4: ldrb r3, [r10, #0x403]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,1027)?)?;
    // 0x397d8: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x397dc: blo 0x397f0 <entry+0x4364>  @ imm = #0xc
    c.b(C::Cc,0x397f0)?;
    // 0x397e0: ldrb r3, [r10, #0x406]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,1030)?)?;
    // 0x397e4: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x397e8: movhi r1, r3
    c.emit(arm32::mov_register(C::Hi,R::R1,R::new(3)?))?;
    // 0x397ec: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x397f0: str r3, [r6, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,108)?)?;
    // 0x397f4: ldrb r1, [r10, #0x404]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1028)?)?;
    // 0x397f8: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x397fc: blo 0x39810 <entry+0x4384>  @ imm = #0xc
    c.b(C::Cc,0x39810)?;
    // 0x39800: ldrb r1, [r10, #0x407]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1031)?)?;
    // 0x39804: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x39808: movhi r2, r1
    c.emit(arm32::mov_register(C::Hi,R::R2,R::R1))?;
    // 0x3980c: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x39810: str r1, [r6, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,112)?)?;
    // 0x39814: ldrb r1, [r10, #0x405]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1029)?)?;
    // 0x39818: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3981c: blo 0x39830 <entry+0x43a4>  @ imm = #0xc
    c.b(C::Cc,0x39830)?;
    // 0x39820: ldrb r1, [r10, #0x408]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1032)?)?;
    // 0x39824: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x39828: movhi r0, r1
    c.emit(arm32::mov_register(C::Hi,R::R0,R::R1))?;
    // 0x3982c: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x39830: str r1, [r6, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,116)?)?;
    // 0x39834: ldr r8, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(9)?,8)?)?;
    // 0x39838: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x3983c: bne 0x39874 <entry+0x43e8>  @ imm = #0x30
    c.b(C::Ne,0x39874)?;
    // 0x39840: ldr r2, [r10, #0x424]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1060)?)?;
    // 0x39844: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x39848: beq 0x39874 <entry+0x43e8>  @ imm = #0x24
    c.b(C::Eq,0x39874)?;
    // 0x3984c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x39850: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x39854: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x39858: bl 0x8187c <entry+0x4c3f0> @ imm = #0x4801c
    c.bl(0x8187c)?;
    // 0x3985c: ldr r0, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,44)?)?;
    // 0x39860: subs r1, r4, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(4)?,R::R2,Shift::Lsl,0,true)?)?;
    // 0x39864: sbc r2, r5, r3
    c.emit(arm32::data_register(C::Al,Op::Sbc,R::R2,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x39868: ldr r0, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,36)?)?;
    // 0x3986c: str r1, [r0, #0x6b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1720)?)?;
    // 0x39870: str r2, [r0, #0x6bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1724)?)?;
    // 0x39874: add r0, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,1)?)?;
    // 0x39878: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3987c: str r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,8)?)?;
    // 0x39880: str r8, [r6, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,168)?)?;
    // 0x39884: str r1, [r6, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,172)?)?;
    // 0x39888: ldrb r0, [r10, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,3104)?)?;
    // 0x3988c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x39890: beq 0x398c4 <entry+0x4438>  @ imm = #0x2c
    c.b(C::Eq,0x398c4)?;
    // 0x39894: ldr r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    // 0x39898: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3989c: ldrne r0, [r6, #0x28]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,40)?)?;
    // 0x398a0: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x398a4: beq 0x398bc <entry+0x4430>  @ imm = #0x10
    c.b(C::Eq,0x398bc)?;
    // 0x398a8: ldr r1, [r10, #0xcb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,3252)?)?;
    // 0x398ac: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x398b0: bl 0x81850 <entry+0x4c3c4> @ imm = #0x47f98
    c.bl(0x81850)?;
    // 0x398b4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x398b8: beq 0x398c4 <entry+0x4438>  @ imm = #0x4
    c.b(C::Eq,0x398c4)?;
    // 0x398bc: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x398c0: str r0, [r6, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,172)?)?;
    // 0x398c4: ldr r0, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,20)?)?;
    // 0x398c8: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x398cc: str r0, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,20)?)?;
    // 0x398d0: ldr r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,24)?)?;
    // 0x398d4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x398d8: ldreq r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(9)?,16)?)?;
    // 0x398dc: addeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::R0,1)?)?;
    // 0x398e0: streq r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,16)?)?;
    // 0x398e4: ldrb r0, [r6, #0x18]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,24)?)?;
    // 0x398e8: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x398ec: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x398f0: strne r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(9)?,24)?)?;
    // 0x398f4: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x398f8: strne r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(9)?,8)?)?;
    // 0x398fc: ldrb r0, [r10, #0x399]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,921)?)?;
    // 0x39900: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x39904: bne 0x39958 <entry+0x44cc>  @ imm = #0x4c
    c.b(C::Ne,0x39958)?;
    // 0x39908: ldr r1, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,296)?)?;
    // 0x3990c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x39910: and r1, r0, r1, lsr #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsr,6,false)?)?;
    // 0x39914: str r1, [r6, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,140)?)?;
    // 0x39918: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3991c: ldrne r1, [r10, #0x194]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(10)?,404)?)?;
    // 0x39920: strne r1, [r6, #0x94]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::new(6)?,148)?)?;
    // 0x39924: ldr r1, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,296)?)?;
    // 0x39928: and r0, r0, r1, lsr #7
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsr,7,false)?)?;
    // 0x3992c: bic r2, r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R1,64)?)?;
    // 0x39930: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x39934: str r2, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,296)?)?;
    // 0x39938: str r0, [r6, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,144)?)?;
    // 0x3993c: addne r0, r10, #408
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::new(10)?,408)?)?;
    // 0x39940: addne r1, r6, #156
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::new(6)?,156)?)?;
    // 0x39944: ldmne r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Ne,true,R::R0,R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x39948: stmne r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Ne,false,R::R1,R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3994c: ldr r0, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,296)?)?;
    // 0x39950: bic r0, r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,128)?)?;
    // 0x39954: str r0, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,296)?)?;
    // 0x39958: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x3995c: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x36d30
    c.bl(0x2c34)?;
    // 0x39960: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x39964: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x39968: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x3996c)?;
    c.emit(CONTROLLER_RESOURCE_DIAGNOSTIC)?;
    c.check(0x39970)?;
    Ok(())
}

/// Check stream indices and selected buffer flags through an ordered series
/// of checked pointers/callbacks. Original POP/PC at 0x39abc.
fn update_codec_resource_flags(c:&mut Code)->Result<(),String>{
    c.check(0x39970)?;
    // 0x39970: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x39974: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x39978: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x3997c: mov r7, r2
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // 0x39980: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x39984: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x39988: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x36d9c
    c.bl(0x2bf4)?;
    // 0x3998c: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x39990: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x39994: cmp r0, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x39998: bls 0x39a28 <entry+0x459c>  @ imm = #0x88
    c.b(C::Ls,0x39a28)?;
    // 0x3999c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x399a0: mov r2, #216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,216)?)?;
    // 0x399a4: ldr r6, [r0, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::R0,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x399a8: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x399ac: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x399b0: bl 0x471c <entry+0xfffffffffffcf290> @ imm = #-0x3529c
    c.bl(0x471c)?;
    // 0x399b4: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x399b8: add r1, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,1)?)?;
    // 0x399bc: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x399c0: bhs 0x399f8 <entry+0x456c>  @ imm = #0x30
    c.b(C::Cs,0x399f8)?;
    // 0x399c4: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x399c8: add r2, r1, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::new(5)?,Shift::Lsl,2,false)?)?;
    // 0x399cc: ldr r2, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,4)?)?;
    // 0x399d0: ldr r3, [r2, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,168)?)?;
    // 0x399d4: cmn r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(3)?,1)?)?;
    // 0x399d8: beq 0x399f8 <entry+0x456c>  @ imm = #0x18
    c.b(C::Eq,0x399f8)?;
    // 0x399dc: str r2, [r1, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x399e0: add r2, r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,2)?)?;
    // 0x399e4: add r1, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,1)?)?;
    // 0x399e8: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x399ec: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x399f0: blo 0x399c4 <entry+0x4538>  @ imm = #-0x34
    c.b(C::Cc,0x399c4)?;
    // 0x399f4: b 0x399fc <entry+0x4570>  @ imm = #0x0
    c.b(C::Al,0x399fc)?;
    // 0x399f8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x399fc: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x39a00: str r0, [r6, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,168)?)?;
    // 0x39a04: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x39a08: str r6, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x39a0c: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x39a10: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // 0x39a14: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x39a18: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // 0x39a1c: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x39a20: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x39a24: str r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // 0x39a28: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x39a2c: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x36e00
    c.bl(0x2c34)?;
    // 0x39a30: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x39a34: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x39a38: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x39a3c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x39a40: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x39a44: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x39a48: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x39a4c: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x39a50: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x36e64
    c.bl(0x2bf4)?;
    // 0x39a54: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x39a58: ldr r2, [pc, #0x60]         @ 0x4634 <entry+0x4634>
    c.literal_cond(C::Al,R::R2,0x39ac0)?;
    // 0x39a5c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x39a60: cmp r1, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x39a64: mov r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x39a68: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x39a6c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x39a70: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x382c8
    c.bl(0x17b0)?;
    // 0x39a74: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x39a78: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x39a7c: ldr r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,24)?)?;
    // 0x39a80: ands r6, r1, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(6)?,R::R1,1,true)?)?;
    // 0x39a84: beq 0x39ab0 <entry+0x4624>  @ imm = #0x24
    c.b(C::Eq,0x39ab0)?;
    // 0x39a88: ldr r1, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,52)?)?;
    // 0x39a8c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x39a90: beq 0x39aa8 <entry+0x461c>  @ imm = #0x10
    c.b(C::Eq,0x39aa8)?;
    // 0x39a94: ldr r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,40)?)?;
    // 0x39a98: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x39a9c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x39aa0: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x39aa4: b 0x39aac <entry+0x4620>  @ imm = #0x0
    c.b(C::Al,0x39aac)?;
    // 0x39aa8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x39aac: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x39ab0: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x39ab4: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x36e88
    c.bl(0x2c34)?;
    // 0x39ab8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x39abc: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x39ac0)?;
    c.emit(CONTROLLER_RESOURCE_DIAGNOSTIC)?;
    c.check(0x39ac4)?;
    Ok(())
}

/// Small descriptor accessor, masked field selection and protected pointer
/// update routines. Terminate immediately before next function at 0x39bd0.
fn controller_descriptor_accessors(c:&mut Code)->Result<(),String>{
    c.check(0x39ac4)?;
    // 0x39ac4: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x39ac8: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x39acc: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x39ad0: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x39ad4: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x39ad8: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x36eec
    c.bl(0x2bf4)?;
    // 0x39adc: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x39ae0: cmp r0, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x39ae4: bls 0x39af8 <entry+0x466c>  @ imm = #0xc
    c.b(C::Ls,0x39af8)?;
    // 0x39ae8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x39aec: ldr r0, [r0, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x39af0: add r0, r0, #168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,168)?)?;
    // 0x39af4: b 0x39afc <entry+0x4670>  @ imm = #0x0
    c.b(C::Al,0x39afc)?;
    // 0x39af8: add r0, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,8)?)?;
    // 0x39afc: ldr r5, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,0)?)?;
    // 0x39b00: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x39b04: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x36ed8
    c.bl(0x2c34)?;
    // 0x39b08: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x39b0c: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x39b10: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x39b14: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x39b18: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x39b1c: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x39b20: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x39b24: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x39b28: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x36f3c
    c.bl(0x2bf4)?;
    // 0x39b2c: ldr r1, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,16)?)?;
    // 0x39b30: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x39b34: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x39b38: cmp r1, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x39b3c: ldrhi r0, [r5]
    c.emit(arm32::transfer_word(C::Hi,true,R::R0,R::new(5)?,0)?)?;
    // 0x39b40: ldrhi r7, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Hi,true,R::new(7)?,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // 0x39b44: ldrhi r0, [r7, #0xac]
    c.emit(arm32::transfer_word(C::Hi,true,R::R0,R::new(7)?,172)?)?;
    // 0x39b48: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x39b4c: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x39b50: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x36f24
    c.bl(0x2c34)?;
    // 0x39b54: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x39b58: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x39b5c: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x39b60: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x39b64: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x39b68: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x39b6c: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x39b70: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x36f84
    c.bl(0x2bf4)?;
    // 0x39b74: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x39b78: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x39b7c: cmp r0, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x39b80: ldrhi r0, [r4]
    c.emit(arm32::transfer_word(C::Hi,true,R::R0,R::new(4)?,0)?)?;
    // 0x39b84: ldrhi r0, [r0, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Hi,true,R::R0,R::R0,R::new(5)?,Shift::Lsl,2)?)?;
    // 0x39b88: ldrhi r6, [r0, #0xac]
    c.emit(arm32::transfer_word(C::Hi,true,R::new(6)?,R::R0,172)?)?;
    // 0x39b8c: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x39b90: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x36f64
    c.bl(0x2c34)?;
    // 0x39b94: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x39b98: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x39b9c: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x39ba0: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x39ba4: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x39ba8: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x39bac: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x39bb0: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // 0x39bb4: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x36fc8
    c.bl(0x2bf4)?;
    // 0x39bb8: ldr r0, [r5, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,172)?)?;
    // 0x39bbc: orr r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x39bc0: str r0, [r5, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,172)?)?;
    // 0x39bc4: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // 0x39bc8: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x39bcc: b 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x36fa0
    c.b(C::Al,0x2c34)?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    configure_codec_capture(c)?;
    update_codec_resource_flags(c)?;
    controller_descriptor_accessors(c)?;
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn resource_accessors_are_contiguous_with_two_literals(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        for addr in [0x3996c,0x39ac0]{
            let i=addr-START;
            assert_eq!(&c.bytes[i..i+4],&CONTROLLER_RESOURCE_DIAGNOSTIC.to_le_bytes());
        }
    }
}
