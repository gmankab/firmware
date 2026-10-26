// SPDX-License-Identifier: GPL-3.0-only
//! Controller event polling and codec buffer-state dispatch. Reconstructs
//! a complete iterative packet processor using checked ARM32 operations.
//! Data pointers and controller magic values below are never disassembled
//! as A32 code even when their bit patterns look like instructions.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x33488;
const END:usize=0x33abc;
const CONTROLLER_DISPATCH_DATA:[u32;19]=[
    0x0010ceac, // runtime 0x33a70
    0x000ebbd8, // runtime 0x33a74
    0x000e2788, // runtime 0x33a78
    0x00121003, // runtime 0x33a7c
    0x00111007, // runtime 0x33a80
    0x00121004, // runtime 0x33a84
    0x000ebca0, // runtime 0x33a88
    0x000ebc91, // runtime 0x33a8c
    0x000ebc55, // runtime 0x33a90
    0x000eb784, // runtime 0x33a94
    0x000ebd95, // runtime 0x33a98
    0x000ebdd3, // runtime 0x33a9c
    0x000ebc0b, // runtime 0x33aa0
    0x00121002, // runtime 0x33aa4
    0x00121001, // runtime 0x33aa8
    0x000ebd05, // runtime 0x33aac
    0x000ebce2, // runtime 0x33ab0
    0x000ebd28, // runtime 0x33ab4
    0x000ebd66, // runtime 0x33ab8
];

/// Iterate active codec request slots, decode frame classes, invoke resource
/// callbacks and propagate controller flags across queue entries.
fn publish_codec_state_events(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x33488: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3348c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x33490: sub sp, sp, #156
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,156)?)?;
    // 0x33494: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x33498: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3349c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x334a0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x334a4: bl 0x3084 <entry+0xfffffffffffd1304> @ imm = #-0x30428
    c.bl(0x3084)?;
    // 0x334a8: add r0, r6, #3472
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,3472)?)?;
    // 0x334ac: add r5, r6, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,200)?)?;
    // 0x334b0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x334b4: str r6, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,52)?)?;
    // 0x334b8: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x334bc: str r5, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,48)?)?;
    // 0x334c0: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x334c4: add r0, r6, #388
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,388)?)?;
    // 0x334c8: add r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7168)?)?;
    // 0x334cc: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x334d0: add r0, r6, #364
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,364)?)?;
    // 0x334d4: add r0, r0, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,9216)?)?;
    // 0x334d8: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x334dc: add r0, r6, #396
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,396)?)?;
    // 0x334e0: add r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7168)?)?;
    // 0x334e4: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x334e8: add r0, r6, #392
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,392)?)?;
    // 0x334ec: add r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7168)?)?;
    // 0x334f0: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x334f4: add r0, r6, #380
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,380)?)?;
    // 0x334f8: add r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7168)?)?;
    // 0x334fc: str r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // 0x33500: add r0, r6, #376
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,376)?)?;
    // 0x33504: add r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7168)?)?;
    // 0x33508: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // 0x3350c: add r0, r6, #372
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,372)?)?;
    // 0x33510: add r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7168)?)?;
    // 0x33514: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x33518: add r0, r6, #3440
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,3440)?)?;
    // 0x3351c: add r10, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::R0,4096)?)?;
    // 0x33520: add r0, r6, #364
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,364)?)?;
    // 0x33524: add r4, r0, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,7168)?)?;
    // 0x33528: add r0, r6, #7552
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,7552)?)?;
    // 0x3352c: str r10, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,60)?)?;
    // 0x33530: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x33534: add r0, r6, #360
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,360)?)?;
    // 0x33538: str r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,56)?)?;
    // 0x3353c: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x33540: ldr r0, [pc, #0x528]        @ 0x1cf0 <entry+0x1cf0>
    c.literal(R::R0,0x33a70)?;
    // 0x33544: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x33548: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x3354c: ldr r0, [r6, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,324)?)?;
    // 0x33550: bl 0x294c <entry+0xfffffffffffd0bcc> @ imm = #-0x30c0c
    c.bl(0x294c)?;
    // 0x33554: ldr r0, [r6, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,324)?)?;
    // 0x33558: bl 0x293c <entry+0xfffffffffffd0bbc> @ imm = #-0x30c24
    c.bl(0x293c)?;
    // 0x3355c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x33560: bl 0x80b90 <entry+0x4ee10> @ imm = #0x4d628
    c.bl(0x80b90)?;
    // 0x33564: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // 0x33568: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x3356c: beq 0x3354c <entry+0x17cc>  @ imm = #-0x28
    c.b(C::Eq,0x3354c)?;
    // 0x33570: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x33574: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x33578: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x3357c: bl 0x80a24 <entry+0x4eca4> @ imm = #0x4d4a0
    c.bl(0x80a24)?;
    // 0x33580: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x33584: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x33588: bne 0x33658 <entry+0x18d8>  @ imm = #0xc8
    c.b(C::Ne,0x33658)?;
    // 0x3358c: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x33590: ldr r1, [pc, #0x4dc]        @ 0x1cf4 <entry+0x1cf4>
    c.literal(R::R1,0x33a74)?;
    // 0x33594: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x33598: ldr r10, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,0)?)?;
    // 0x3359c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x335a0: str r3, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,72)?)?;
    // 0x335a4: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x335a8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x491c
    c.bl(0x2ec94)?;
    // 0x335ac: ldr r1, [pc, #0x4c4]        @ 0x1cf8 <entry+0x1cf8>
    c.literal(R::R1,0x33a78)?;
    // 0x335b0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x335b4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4928
    c.bl(0x2ec94)?;
    // 0x335b8: ldr r5, [pc, #0x4bc]        @ 0x1cfc <entry+0x1cfc>
    c.literal(R::new(5)?,0x33a7c)?;
    // 0x335bc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x335c0: orr r0, r0, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1114112)?)?;
    // 0x335c4: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x335c8: bgt 0x33668 <entry+0x18e8>  @ imm = #0x98
    c.b(C::Gt,0x33668)?;
    // 0x335cc: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x335d0: orr r0, r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,131072)?)?;
    // 0x335d4: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x335d8: beq 0x33a20 <entry+0x1ca0>  @ imm = #0x440
    c.b(C::Eq,0x33a20)?;
    // 0x335dc: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x335e0: orr r0, r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,131072)?)?;
    // 0x335e4: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x335e8: beq 0x3369c <entry+0x191c>  @ imm = #0xac
    c.b(C::Eq,0x3369c)?;
    // 0x335ec: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // 0x335f0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x335f4: orr r0, r0, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1114112)?)?;
    // 0x335f8: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x335fc: bne 0x33940 <entry+0x1bc0>  @ imm = #0x33c
    c.b(C::Ne,0x33940)?;
    // 0x33600: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x33604: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x33608: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x3360c: beq 0x3362c <entry+0x18ac>  @ imm = #0x18
    c.b(C::Eq,0x3362c)?;
    // 0x33610: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x33614: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // 0x33618: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x3361c: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x33620: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x33624: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x33628: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x3362c: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x33630: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x33634: ldrne r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::SP,24)?)?;
    // 0x33638: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x3363c: ldrne r1, [r1]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::R1,0)?)?;
    // 0x33640: strne r1, [r0]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,0)?)?;
    // 0x33644: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // 0x33648: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x3364c: str r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x33650: bl 0x2a158 <entry+0xffffffffffff83d8> @ imm = #-0x9500
    c.bl(0x2a158)?;
    // 0x33654: b 0x33a20 <entry+0x1ca0>  @ imm = #0x3c4
    c.b(C::Al,0x33a20)?;
    // 0x33658: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x3365c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x33660: bl 0x33d80 <entry+0x2000>  @ imm = #0x718
    c.bl(0x33d80)?;
    // 0x33664: b 0x33a60 <entry+0x1ce0>  @ imm = #0x3f4
    c.b(C::Al,0x33a60)?;
    // 0x33668: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // 0x3366c: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x33670: orr r0, r0, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1114112)?)?;
    // 0x33674: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x33678: beq 0x336dc <entry+0x195c>  @ imm = #0x5c
    c.b(C::Eq,0x336dc)?;
    // 0x3367c: ldr r0, [pc, #0x3fc]        @ 0x1d00 <entry+0x1d00>
    c.literal(R::R0,0x33a80)?;
    // 0x33680: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x33684: beq 0x33764 <entry+0x19e4>  @ imm = #0xd8
    c.b(C::Eq,0x33764)?;
    // 0x33688: ldr r0, [pc, #0x3f4]        @ 0x1d04 <entry+0x1d04>
    c.literal(R::R0,0x33a84)?;
    // 0x3368c: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x33690: bne 0x33940 <entry+0x1bc0>  @ imm = #0x2a8
    c.b(C::Ne,0x33940)?;
    // 0x33694: bl 0x20a0 <entry+0xfffffffffffd0320> @ imm = #-0x315fc
    c.bl(0x20a0)?;
    // 0x33698: b 0x33940 <entry+0x1bc0>  @ imm = #0x2a0
    c.b(C::Al,0x33940)?;
    // 0x3369c: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x336a0: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // 0x336a4: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x336a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x336ac: beq 0x33874 <entry+0x1af4>  @ imm = #0x1c0
    c.b(C::Eq,0x33874)?;
    // 0x336b0: ldr r1, [pc, #0x3d0]        @ 0x1d08 <entry+0x1d08>
    c.literal(R::R1,0x33a88)?;
    // 0x336b4: ldr r2, [pc, #0x3d0]        @ 0x1d0c <entry+0x1d0c>
    c.literal(R::R2,0x33a8c)?;
    // 0x336b8: mov r3, #154
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,154)?)?;
    // 0x336bc: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x336c0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x336c4: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x336c8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4a3c
    c.bl(0x2ec94)?;
    // 0x336cc: ldr r1, [pc, #0x3a4]        @ 0x1cf8 <entry+0x1cf8>
    c.literal(R::R1,0x33a78)?;
    // 0x336d0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x336d4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4a48
    c.bl(0x2ec94)?;
    // 0x336d8: b 0x33940 <entry+0x1bc0>  @ imm = #0x260
    c.b(C::Al,0x33940)?;
    // 0x336dc: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x336e0: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x336e4: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x336e8: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x336ec: tst r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1)?)?;
    // 0x336f0: beq 0x33710 <entry+0x1990>  @ imm = #0x18
    c.b(C::Eq,0x33710)?;
    // 0x336f4: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x336f8: ldr r2, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,36)?)?;
    // 0x336fc: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x33700: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x33704: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x33708: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,64)?)?;
    // 0x3370c: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x33710: tst r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,2)?)?;
    // 0x33714: ldrne r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::SP,24)?)?;
    // 0x33718: ldrne r1, [r1]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::R1,0)?)?;
    // 0x3371c: strne r1, [r0]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,0)?)?;
    // 0x33720: ldr r4, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,32)?)?;
    // 0x33724: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x33728: str r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x3372c: bl 0x2a158 <entry+0xffffffffffff83d8> @ imm = #-0x95dc
    c.bl(0x2a158)?;
    // 0x33730: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x33734: ldr r3, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,0)?)?;
    // 0x33738: ldr r1, [pc, #0x360]        @ 0x1d20 <entry+0x1d20>
    c.literal(R::R1,0x33aa0)?;
    // 0x3373c: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x33740: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x33744: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x33748: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x3374c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x33750: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4ac4
    c.bl(0x2ec94)?;
    // 0x33754: ldr r1, [pc, #0x31c]        @ 0x1cf8 <entry+0x1cf8>
    c.literal(R::R1,0x33a78)?;
    // 0x33758: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3375c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4ad0
    c.bl(0x2ec94)?;
    // 0x33760: b 0x33a20 <entry+0x1ca0>  @ imm = #0x2b8
    c.b(C::Al,0x33a20)?;
    // 0x33764: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x33768: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // 0x3376c: mov r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // 0x33770: ldr r4, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,0)?)?;
    // 0x33774: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x33778: ldr r7, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,0)?)?;
    // 0x3377c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x33780: bl 0x2038 <entry+0xfffffffffffd02b8> @ imm = #-0x31750
    c.bl(0x2038)?;
    // 0x33784: add r0, r4, r4, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(4)?,Shift::Lsl,2,false)?)?;
    // 0x33788: add r6, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x3378c: ldrd r0, r1, [r7]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x33790: ldr r3, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,12)?)?;
    // 0x33794: ldr r2, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,16)?)?;
    // 0x33798: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3379c: ldr r1, [pc, #0x2f4]        @ 0x1d18 <entry+0x1d18>
    c.literal(R::R1,0x33a98)?;
    // 0x337a0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x337a4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x337a8: mov r4, r3
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x337ac: str r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
    // 0x337b0: movne r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x337b4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x337b8: movne r4, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,2)?)?;
    // 0x337bc: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x337c0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4b34
    c.bl(0x2ec94)?;
    // 0x337c4: ldr r1, [pc, #0x2ac]        @ 0x1cf8 <entry+0x1cf8>
    c.literal(R::R1,0x33a78)?;
    // 0x337c8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x337cc: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4b40
    c.bl(0x2ec94)?;
    // 0x337d0: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // 0x337d4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x337d8: strd r0, r1, [r7, #8]
    c.emit(arm32::transfer_doubleword(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // 0x337dc: ldrd r0, r1, [r7]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x337e0: eor r2, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Eor,R::R2,R::R0,3)?)?;
    // 0x337e4: orrs r2, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,0,true)?)?;
    // 0x337e8: beq 0x33814 <entry+0x1a94>  @ imm = #0x24
    c.b(C::Eq,0x33814)?;
    // 0x337ec: orrs r2, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,true)?)?;
    // 0x337f0: beq 0x33808 <entry+0x1a88>  @ imm = #0x10
    c.b(C::Eq,0x33808)?;
    // 0x337f4: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // 0x337f8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x337fc: eoreq r0, r0, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Eor,R::R0,R::R0,4)?)?;
    // 0x33800: orrseq r0, r0, r1
    c.emit(arm32::data_register(C::Eq,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,true)?)?;
    // 0x33804: beq 0x33814 <entry+0x1a94>  @ imm = #0x8
    c.b(C::Eq,0x33814)?;
    // 0x33808: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3380c: str r4, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(7)?,0)?)?;
    // 0x33810: str r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x33814: ldr r0, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,16)?)?;
    // 0x33818: ldr r3, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,20)?)?;
    // 0x3381c: ldr r2, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,24)?)?;
    // 0x33820: ldr r1, [pc, #0x274]        @ 0x1d1c <entry+0x1d1c>
    c.literal(R::R1,0x33a9c)?;
    // 0x33824: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x33828: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3382c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4ba0
    c.bl(0x2ec94)?;
    // 0x33830: ldr r1, [pc, #0x240]        @ 0x1cf8 <entry+0x1cf8>
    c.literal(R::R1,0x33a78)?;
    // 0x33834: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x33838: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4bac
    c.bl(0x2ec94)?;
    // 0x3383c: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x33840: mov r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // 0x33844: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x33848: bl 0x2014 <entry+0xfffffffffffd0294> @ imm = #-0x3183c
    c.bl(0x2014)?;
    // 0x3384c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x33850: add r1, sp, #76
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,76)?)?;
    // 0x33854: str r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,76)?)?;
    // 0x33858: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x3385c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x33860: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x33864: add r0, r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,2)?)?;
    // 0x33868: str r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,80)?)?;
    // 0x3386c: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x33870: b 0x33a18 <entry+0x1c98>  @ imm = #0x1a0
    c.b(C::Al,0x33a18)?;
    // 0x33874: ldr r6, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,64)?)?;
    // 0x33878: str r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // 0x3387c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x33880: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x33884: add r0, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x33888: mov r2, r1
    c.emit(arm32::mov_register(C::Al,R::R2,R::R1))?;
    // 0x3388c: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33890: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x33894: bl 0x3084 <entry+0xfffffffffffd1304> @ imm = #-0x30818
    c.bl(0x3084)?;
    // 0x33898: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x3389c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x338a0: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x338a4: ldr r0, [r1, r0, lsl #2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x338a8: ldr r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,4)?)?;
    // 0x338ac: ldr r1, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,24)?)?;
    // 0x338b0: blx r1
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    // 0x338b4: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x338b8: str r7, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,0)?)?;
    // 0x338bc: bl 0x20a0 <entry+0xfffffffffffd0320> @ imm = #-0x31824
    c.bl(0x20a0)?;
    // 0x338c0: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x338c4: beq 0x3390c <entry+0x1b8c>  @ imm = #0x40
    c.b(C::Eq,0x3390c)?;
    // 0x338c8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x338cc: ldr r1, [pc, #0x1bc]        @ 0x1d10 <entry+0x1d10>
    c.literal(R::R1,0x33a90)?;
    // 0x338d0: ldr r2, [pc, #0x1b4]        @ 0x1d0c <entry+0x1d0c>
    c.literal(R::R2,0x33a8c)?;
    // 0x338d4: mov r3, #147
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,147)?)?;
    // 0x338d8: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x338dc: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x338e0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x338e4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4c58
    c.bl(0x2ec94)?;
    // 0x338e8: ldr r1, [pc, #0x188]        @ 0x1cf8 <entry+0x1cf8>
    c.literal(R::R1,0x33a78)?;
    // 0x338ec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x338f0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4c64
    c.bl(0x2ec94)?;
    // 0x338f4: ldr r2, [pc, #0x198]        @ 0x1d14 <entry+0x1d14>
    c.literal(R::R2,0x33a94)?;
    // 0x338f8: mov r1, #149
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,149)?)?;
    // 0x338fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33900: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x33904: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x3215c
    c.bl(0x17b0)?;
    // 0x33908: b 0x3393c <entry+0x1bbc>  @ imm = #0x2c
    c.b(C::Al,0x3393c)?;
    // 0x3390c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x33910: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x33914: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33918: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x3391c: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x33920: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x33924: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33928: str r1, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // 0x3392c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x33930: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33934: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33938: str r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x3393c: ldr r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,16)?)?;
    // 0x33940: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x33944: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x33948: ldr r1, [pc, #0x154]        @ 0x1d24 <entry+0x1d24>
    c.literal(R::R1,0x33aa4)?;
    // 0x3394c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x33950: beq 0x3399c <entry+0x1c1c>  @ imm = #0x44
    c.b(C::Eq,0x3399c)?;
    // 0x33954: ldr r1, [pc, #0x14c]        @ 0x1d28 <entry+0x1d28>
    c.literal(R::R1,0x33aa8)?;
    // 0x33958: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3395c: bne 0x339d4 <entry+0x1c54>  @ imm = #0x70
    c.b(C::Ne,0x339d4)?;
    // 0x33960: ldr r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,64)?)?;
    // 0x33964: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x33968: ldr r0, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,0)?)?;
    // 0x3396c: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x33970: mov r7, r2
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // 0x33974: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33978: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x3397c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x33980: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // 0x33984: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x33988: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,16)?)?;
    // 0x3398c: ldr r7, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,0)?)?;
    // 0x33990: bl 0x26754 <entry+0xffffffffffff49d4> @ imm = #-0xd244
    c.bl(0x26754)?;
    // 0x33994: ldr r1, [pc, #0x114]        @ 0x1d30 <entry+0x1d30>
    c.literal(R::R1,0x33ab0)?;
    // 0x33998: b 0x339ac <entry+0x1c2c>  @ imm = #0xc
    c.b(C::Al,0x339ac)?;
    // 0x3399c: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x339a0: ldr r7, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,0)?)?;
    // 0x339a4: bl 0x26754 <entry+0xffffffffffff49d4> @ imm = #-0xd258
    c.bl(0x26754)?;
    // 0x339a8: ldr r1, [pc, #0xfc]         @ 0x1d2c <entry+0x1d2c>
    c.literal(R::R1,0x33aac)?;
    // 0x339ac: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // 0x339b0: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x339b4: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x339b8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4d2c
    c.bl(0x2ec94)?;
    // 0x339bc: ldr r1, [pc, #0xb4]         @ 0x1cf8 <entry+0x1cf8>
    c.literal(R::R1,0x33a78)?;
    // 0x339c0: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x339c4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4d38
    c.bl(0x2ec94)?;
    // 0x339c8: bl 0x20a0 <entry+0xfffffffffffd0320> @ imm = #-0x31930
    c.bl(0x20a0)?;
    // 0x339cc: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x339d0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x339d4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x339d8: orr r1, r1, #1179648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1179648)?)?;
    // 0x339dc: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x339e0: bne 0x33a04 <entry+0x1c84>  @ imm = #0x1c
    c.b(C::Ne,0x33a04)?;
    // 0x339e4: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
    // 0x339e8: ldr r1, [pc, #0xc4]         @ 0x1d34 <entry+0x1d34>
    c.literal(R::R1,0x33ab4)?;
    // 0x339ec: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x339f0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x339f4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4d68
    c.bl(0x2ec94)?;
    // 0x339f8: ldr r1, [pc, #0x78]         @ 0x1cf8 <entry+0x1cf8>
    c.literal(R::R1,0x33a78)?;
    // 0x339fc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x33a00: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4d74
    c.bl(0x2ec94)?;
    // 0x33a04: cmp r10, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x33a08: bne 0x33a10 <entry+0x1c90>  @ imm = #0x0
    c.b(C::Ne,0x33a10)?;
    // 0x33a0c: bl 0x20a0 <entry+0xfffffffffffd0320> @ imm = #-0x31974
    c.bl(0x20a0)?;
    // 0x33a10: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x33a14: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x33a18: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x33a1c: bl 0x2a18c <entry+0xffffffffffff840c> @ imm = #-0x9898
    c.bl(0x2a18c)?;
    // 0x33a20: ldr r6, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,52)?)?;
    // 0x33a24: ldr r5, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,72)?)?;
    // 0x33a28: ldr r0, [r6, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,324)?)?;
    // 0x33a2c: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x33a30: bl 0x292c <entry+0xfffffffffffd0bac> @ imm = #-0x3110c
    c.bl(0x292c)?;
    // 0x33a34: ldr r1, [pc, #0x7c]         @ 0x1d38 <entry+0x1d38>
    c.literal(R::R1,0x33ab8)?;
    // 0x33a38: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33a3c: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x33a40: mov r7, r5
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(5)?))?;
    // 0x33a44: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4db8
    c.bl(0x2ec94)?;
    // 0x33a48: ldr r1, [pc, #0x28]         @ 0x1cf8 <entry+0x1cf8>
    c.literal(R::R1,0x33a78)?;
    // 0x33a4c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33a50: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4dc4
    c.bl(0x2ec94)?;
    // 0x33a54: ldr r10, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,60)?)?;
    // 0x33a58: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,56)?)?;
    // 0x33a5c: ldr r5, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,48)?)?;
    // 0x33a60: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // 0x33a64: cmp r9, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x33a68: blo 0x33574 <entry+0x17f4>  @ imm = #-0x4fc
    c.b(C::Cc,0x33574)?;
    // 0x33a6c: b 0x3354c <entry+0x17cc>  @ imm = #-0x528
    c.b(C::Al,0x3354c)?;
    c.check(0x33a70)?;
    for word in CONTROLLER_DISPATCH_DATA {c.emit(word)?;}
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    publish_codec_state_events(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_event_polling_has_named_literals(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let at=|a:usize|u32::from_le_bytes(c.bytes[a-START..a-START+4].try_into().unwrap());
        assert_eq!(at(0x33a70),0x0010ceac);
        assert_eq!(at(0x33a7c),0x00121003);
        assert_eq!(at(0x33aa8),0x00121001);
        assert_eq!(at(0x33ab8),CONTROLLER_DISPATCH_DATA[18]);
    }
}
