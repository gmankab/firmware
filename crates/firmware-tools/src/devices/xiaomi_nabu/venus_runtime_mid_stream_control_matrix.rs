// SPDX-License-Identifier: GPL-3.0-only
//! Codec controller stream configuration: copy typed descriptor/command
//! values into the large stack-backed controller message array. The format
//! and diagnostic pointers refer back to the preceding string-address pool.
//! This is part of a large function whose final epilogue is not yet emitted.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
const START:usize=0x3a2e4;
const END:usize=0x3aa00;

fn initialize_stream_parameter_matrix(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3a2e4: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x3a2e8: str r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-48)?)?;
    // 0x3a2ec: ldr r0, [pc, #-0x254]       @ 0x4d0 <entry+0x4d0>
    c.literal_cond(C::Al,R::R0,0x3a0a0)?;
    // 0x3a2f0: ldr r2, [pc, #-0x254]       @ 0x4d4 <entry+0x4d4>
    c.literal_cond(C::Al,R::R2,0x3a0a4)?;
    // 0x3a2f4: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x3a2f8: mov r4, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,10)?)?;
    // 0x3a2fc: add r3, r6, #412
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,412)?)?;
    // 0x3a300: ldr r7, [pc, #-0x260]       @ 0x4d8 <entry+0x4d8>
    c.literal_cond(C::Al,R::new(7)?,0x3a0a8)?;
    // 0x3a304: mov lr, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,6)?)?;
    // 0x3a308: add r5, r6, #568
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,568)?)?;
    // 0x3a30c: mov r8, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,12)?)?;
    // 0x3a310: mov r9, #244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,244)?)?;
    // 0x3a314: mov r12, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,7)?)?;
    // 0x3a318: add r10, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,24)?)?;
    // 0x3a31c: add r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,3072)?)?;
    // 0x3a320: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3a324: orr r8, r8, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,4096)?)?;
    // 0x3a328: orr r9, r9, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,4096)?)?;
    // 0x3a32c: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x3a330: add r0, r6, #312
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,312)?)?;
    // 0x3a334: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x3a338: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // 0x3a33c: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x3a340: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3a344: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x3a348: str r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,40)?)?;
    // 0x3a34c: add r2, r6, #324
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,324)?)?;
    // 0x3a350: str r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,44)?)?;
    // 0x3a354: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a358: str r2, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,48)?)?;
    // 0x3a35c: ldr r2, [pc, #-0x2b8]       @ 0x4dc <entry+0x4dc>
    c.literal_cond(C::Al,R::R2,0x3a0ac)?;
    // 0x3a360: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // 0x3a364: str r2, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,56)?)?;
    // 0x3a368: add r2, r6, #3392
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3392)?)?;
    // 0x3a36c: str r1, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,60)?)?;
    // 0x3a370: str r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,64)?)?;
    // 0x3a374: ldr r2, [pc, #-0x2cc]       @ 0x4e0 <entry+0x4e0>
    c.literal_cond(C::Al,R::R2,0x3a0b0)?;
    // 0x3a378: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // 0x3a37c: str r2, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,72)?)?;
    // 0x3a380: add r2, r6, #235
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,235)?)?;
    // 0x3a384: str r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,76)?)?;
    // 0x3a388: add r2, r2, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3840)?)?;
    // 0x3a38c: str r2, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,80)?)?;
    // 0x3a390: ldr r2, [pc, #-0x2e4]       @ 0x4e4 <entry+0x4e4>
    c.literal_cond(C::Al,R::R2,0x3a0b4)?;
    // 0x3a394: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x3a398: str r2, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,88)?)?;
    // 0x3a39c: add r2, r6, #129
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,129)?)?;
    // 0x3a3a0: str r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,92)?)?;
    // 0x3a3a4: add r2, r2, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3584)?)?;
    // 0x3a3a8: str r2, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,96)?)?;
    // 0x3a3ac: ldr r2, [pc, #-0x2fc]       @ 0x4e8 <entry+0x4e8>
    c.literal_cond(C::Al,R::R2,0x3a0b8)?;
    // 0x3a3b0: str r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,100)?)?;
    // 0x3a3b4: str r2, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,104)?)?;
    // 0x3a3b8: add r2, r6, #328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,328)?)?;
    // 0x3a3bc: str r1, [sp, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,108)?)?;
    // 0x3a3c0: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a3c4: str r2, [sp, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,112)?)?;
    // 0x3a3c8: ldr r2, [pc, #-0x314]       @ 0x4ec <entry+0x4ec>
    c.literal_cond(C::Al,R::R2,0x3a0bc)?;
    // 0x3a3cc: str r0, [sp, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,116)?)?;
    // 0x3a3d0: str r2, [sp, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,120)?)?;
    // 0x3a3d4: add r2, r6, #332
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,332)?)?;
    // 0x3a3d8: str r1, [sp, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,124)?)?;
    // 0x3a3dc: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a3e0: str r2, [sp, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,128)?)?;
    // 0x3a3e4: ldr r2, [pc, #-0x32c]       @ 0x4f0 <entry+0x4f0>
    c.literal_cond(C::Al,R::R2,0x3a0c0)?;
    // 0x3a3e8: str r0, [sp, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,132)?)?;
    // 0x3a3ec: str r2, [sp, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,136)?)?;
    // 0x3a3f0: add r2, r6, #372
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,372)?)?;
    // 0x3a3f4: str r1, [sp, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,140)?)?;
    // 0x3a3f8: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a3fc: str r2, [sp, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,144)?)?;
    // 0x3a400: ldr r2, [pc, #-0x344]       @ 0x4f4 <entry+0x4f4>
    c.literal_cond(C::Al,R::R2,0x3a0c4)?;
    // 0x3a404: str r0, [sp, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,148)?)?;
    // 0x3a408: str r2, [sp, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,152)?)?;
    // 0x3a40c: add r2, r6, #376
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,376)?)?;
    // 0x3a410: str r1, [sp, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,156)?)?;
    // 0x3a414: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a418: str r2, [sp, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,160)?)?;
    // 0x3a41c: ldr r2, [pc, #-0x35c]       @ 0x4f8 <entry+0x4f8>
    c.literal_cond(C::Al,R::R2,0x3a0c8)?;
    // 0x3a420: str r0, [sp, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,164)?)?;
    // 0x3a424: str r2, [sp, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,168)?)?;
    // 0x3a428: sub r2, r11, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,36)?)?;
    // 0x3a42c: str r1, [sp, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,172)?)?;
    // 0x3a430: str r2, [sp, #0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,176)?)?;
    // 0x3a434: ldr r2, [pc, #-0x370]       @ 0x4fc <entry+0x4fc>
    c.literal_cond(C::Al,R::R2,0x3a0cc)?;
    // 0x3a438: str r0, [sp, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,180)?)?;
    // 0x3a43c: str r2, [sp, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,184)?)?;
    // 0x3a440: sub r2, r11, #64
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,64)?)?;
    // 0x3a444: str r1, [sp, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,188)?)?;
    // 0x3a448: str r2, [sp, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,192)?)?;
    // 0x3a44c: ldr r2, [pc, #-0x384]       @ 0x500 <entry+0x500>
    c.literal_cond(C::Al,R::R2,0x3a0d0)?;
    // 0x3a450: str r0, [sp, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,196)?)?;
    // 0x3a454: str r2, [sp, #0xc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,200)?)?;
    // 0x3a458: add r2, r6, #3504
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3504)?)?;
    // 0x3a45c: str r1, [sp, #0xcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,204)?)?;
    // 0x3a460: str r2, [sp, #0xd0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,208)?)?;
    // 0x3a464: ldr r2, [pc, #-0x398]       @ 0x504 <entry+0x504>
    c.literal_cond(C::Al,R::R2,0x3a0d4)?;
    // 0x3a468: str r0, [sp, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,212)?)?;
    // 0x3a46c: str r2, [sp, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,216)?)?;
    // 0x3a470: sub r2, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,40)?)?;
    // 0x3a474: str r1, [sp, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,220)?)?;
    // 0x3a478: str r2, [sp, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,224)?)?;
    // 0x3a47c: ldr r2, [pc, #-0x3ac]       @ 0x508 <entry+0x508>
    c.literal_cond(C::Al,R::R2,0x3a0d8)?;
    // 0x3a480: str r0, [sp, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,228)?)?;
    // 0x3a484: str r2, [sp, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,232)?)?;
    // 0x3a488: sub r2, r11, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,44)?)?;
    // 0x3a48c: str r1, [sp, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,236)?)?;
    // 0x3a490: str r2, [sp, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,240)?)?;
    // 0x3a494: ldr r2, [pc, #-0x3c0]       @ 0x50c <entry+0x50c>
    c.literal_cond(C::Al,R::R2,0x3a0dc)?;
    // 0x3a498: str r0, [sp, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,244)?)?;
    // 0x3a49c: str r2, [sp, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,248)?)?;
    // 0x3a4a0: sub r2, r11, #48
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,48)?)?;
    // 0x3a4a4: str r1, [sp, #0xfc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,252)?)?;
    // 0x3a4a8: str r2, [sp, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,256)?)?;
    // 0x3a4ac: ldr r2, [pc, #-0x3d4]       @ 0x510 <entry+0x510>
    c.literal_cond(C::Al,R::R2,0x3a0e0)?;
    // 0x3a4b0: str r0, [sp, #0x104]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,260)?)?;
    // 0x3a4b4: str r2, [sp, #0x108]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,264)?)?;
    // 0x3a4b8: add r2, r6, #131
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,131)?)?;
    // 0x3a4bc: str r0, [sp, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,268)?)?;
    // 0x3a4c0: add r2, r2, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3584)?)?;
    // 0x3a4c4: str r2, [sp, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,272)?)?;
    // 0x3a4c8: ldr r2, [pc, #-0x3ec]       @ 0x514 <entry+0x514>
    c.literal_cond(C::Al,R::R2,0x3a0e4)?;
    // 0x3a4cc: str r0, [sp, #0x114]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,276)?)?;
    // 0x3a4d0: str r2, [sp, #0x118]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,280)?)?;
    // 0x3a4d4: add r2, r6, #644
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,644)?)?;
    // 0x3a4d8: str r0, [sp, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,284)?)?;
    // 0x3a4dc: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a4e0: str r2, [sp, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,288)?)?;
    // 0x3a4e4: ldr r2, [pc, #-0x404]       @ 0x518 <entry+0x518>
    c.literal_cond(C::Al,R::R2,0x3a0e8)?;
    // 0x3a4e8: str r0, [sp, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,292)?)?;
    // 0x3a4ec: str r2, [sp, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,296)?)?;
    // 0x3a4f0: sub r2, r11, #60
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,60)?)?;
    // 0x3a4f4: str r0, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,300)?)?;
    // 0x3a4f8: str r2, [sp, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,304)?)?;
    // 0x3a4fc: ldr r2, [pc, #-0x418]       @ 0x51c <entry+0x51c>
    c.literal_cond(C::Al,R::R2,0x3a0ec)?;
    // 0x3a500: str r0, [sp, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,308)?)?;
    // 0x3a504: str r2, [sp, #0x138]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,312)?)?;
    // 0x3a508: sub r2, r11, #52
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,52)?)?;
    // 0x3a50c: str r1, [sp, #0x13c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,316)?)?;
    // 0x3a510: str r2, [sp, #0x140]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,320)?)?;
    // 0x3a514: ldr r2, [pc, #-0x42c]       @ 0x520 <entry+0x520>
    c.literal_cond(C::Al,R::R2,0x3a0f0)?;
    // 0x3a518: str r0, [sp, #0x144]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,324)?)?;
    // 0x3a51c: str r2, [sp, #0x148]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,328)?)?;
    // 0x3a520: sub r2, r11, #56
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,56)?)?;
    // 0x3a524: str r1, [sp, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,332)?)?;
    // 0x3a528: str r2, [sp, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,336)?)?;
    // 0x3a52c: ldr r2, [pc, #-0x440]       @ 0x524 <entry+0x524>
    c.literal_cond(C::Al,R::R2,0x3a0f4)?;
    // 0x3a530: str r0, [sp, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,340)?)?;
    // 0x3a534: str r2, [sp, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,344)?)?;
    // 0x3a538: add r2, r6, #668
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,668)?)?;
    // 0x3a53c: str r1, [sp, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,348)?)?;
    // 0x3a540: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a544: str r2, [sp, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,352)?)?;
    // 0x3a548: ldr r2, [pc, #-0x458]       @ 0x528 <entry+0x528>
    c.literal_cond(C::Al,R::R2,0x3a0f8)?;
    // 0x3a54c: str r0, [sp, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,356)?)?;
    // 0x3a550: str r2, [sp, #0x168]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,360)?)?;
    // 0x3a554: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x3a558: str r2, [sp, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,364)?)?;
    // 0x3a55c: add r2, r6, #138
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,138)?)?;
    // 0x3a560: add r2, r2, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3584)?)?;
    // 0x3a564: str r2, [sp, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,368)?)?;
    // 0x3a568: ldr r2, [pc, #-0x474]       @ 0x52c <entry+0x52c>
    c.literal_cond(C::Al,R::R2,0x3a0fc)?;
    // 0x3a56c: str r0, [sp, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,372)?)?;
    // 0x3a570: str r2, [sp, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,376)?)?;
    // 0x3a574: add r2, r6, #580
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,580)?)?;
    // 0x3a578: str r1, [sp, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,380)?)?;
    // 0x3a57c: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a580: str r2, [sp, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,384)?)?;
    // 0x3a584: ldr r2, [pc, #-0x48c]       @ 0x530 <entry+0x530>
    c.literal_cond(C::Al,R::R2,0x3a100)?;
    // 0x3a588: str r0, [sp, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,388)?)?;
    // 0x3a58c: str r2, [sp, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,392)?)?;
    // 0x3a590: add r2, r6, #3536
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3536)?)?;
    // 0x3a594: str r4, [sp, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,396)?)?;
    // 0x3a598: str r2, [sp, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,400)?)?;
    // 0x3a59c: ldr r2, [pc, #-0x4a0]       @ 0x534 <entry+0x534>
    c.literal_cond(C::Al,R::R2,0x3a104)?;
    // 0x3a5a0: str r0, [sp, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,404)?)?;
    // 0x3a5a4: str r2, [sp, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,408)?)?;
    // 0x3a5a8: add r2, r6, #404
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,404)?)?;
    // 0x3a5ac: str r1, [sp, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,412)?)?;
    // 0x3a5b0: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a5b4: str r2, [sp, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,416)?)?;
    // 0x3a5b8: ldr r2, [pc, #-0x4b8]       @ 0x538 <entry+0x538>
    c.literal_cond(C::Al,R::R2,0x3a108)?;
    // 0x3a5bc: str r0, [sp, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,420)?)?;
    // 0x3a5c0: str r2, [sp, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,424)?)?;
    // 0x3a5c4: add r2, r6, #408
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,408)?)?;
    // 0x3a5c8: str r1, [sp, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,428)?)?;
    // 0x3a5cc: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a5d0: str r2, [sp, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,432)?)?;
    // 0x3a5d4: ldr r2, [pc, #-0x4d0]       @ 0x53c <entry+0x53c>
    c.literal_cond(C::Al,R::R2,0x3a10c)?;
    // 0x3a5d8: str r0, [sp, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,436)?)?;
    // 0x3a5dc: str r2, [sp, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,440)?)?;
    // 0x3a5e0: mov r2, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,60)?)?;
    // 0x3a5e4: str r1, [sp, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,444)?)?;
    // 0x3a5e8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3a5ec: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3a5f0: str r2, [sp, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,448)?)?;
    // 0x3a5f4: ldr r2, [pc, #-0x4ec]       @ 0x540 <entry+0x540>
    c.literal_cond(C::Al,R::R2,0x3a110)?;
    // 0x3a5f8: str r0, [sp, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,452)?)?;
    // 0x3a5fc: str r2, [sp, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,456)?)?;
    // 0x3a600: add r2, r6, #3408
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3408)?)?;
    // 0x3a604: str r0, [sp, #0x1cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,460)?)?;
    // 0x3a608: str r2, [sp, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,464)?)?;
    // 0x3a60c: ldr r2, [pc, #-0x500]       @ 0x544 <entry+0x544>
    c.literal_cond(C::Al,R::R2,0x3a114)?;
    // 0x3a610: str r0, [sp, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,468)?)?;
    // 0x3a614: str r2, [sp, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,472)?)?;
    // 0x3a618: add r2, r6, #98
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,98)?)?;
    // 0x3a61c: str r0, [sp, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,476)?)?;
    // 0x3a620: add r2, r2, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3584)?)?;
    // 0x3a624: str r2, [sp, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,480)?)?;
    // 0x3a628: ldr r2, [pc, #-0x518]       @ 0x548 <entry+0x548>
    c.literal_cond(C::Al,R::R2,0x3a118)?;
    // 0x3a62c: str r0, [sp, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,484)?)?;
    // 0x3a630: str r2, [sp, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,488)?)?;
    // 0x3a634: add r2, r6, #616
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,616)?)?;
    // 0x3a638: str r0, [sp, #0x1ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,492)?)?;
    // 0x3a63c: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a640: str r2, [sp, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,496)?)?;
    // 0x3a644: ldr r2, [pc, #-0x530]       @ 0x54c <entry+0x54c>
    c.literal_cond(C::Al,R::R2,0x3a11c)?;
    // 0x3a648: str r0, [sp, #0x1f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,500)?)?;
    // 0x3a64c: str r2, [sp, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,504)?)?;
    // 0x3a650: add r2, r6, #105
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,105)?)?;
    // 0x3a654: str r0, [sp, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,508)?)?;
    // 0x3a658: add r2, r2, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3584)?)?;
    // 0x3a65c: str r2, [sp, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,512)?)?;
    // 0x3a660: ldr r2, [pc, #-0x548]       @ 0x550 <entry+0x550>
    c.literal_cond(C::Al,R::R2,0x3a120)?;
    // 0x3a664: str r0, [sp, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,516)?)?;
    // 0x3a668: str r2, [sp, #0x208]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,520)?)?;
    // 0x3a66c: add r2, r6, #107
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,107)?)?;
    // 0x3a670: str r0, [sp, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,524)?)?;
    // 0x3a674: add r2, r2, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3584)?)?;
    // 0x3a678: str r2, [sp, #0x210]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,528)?)?;
    // 0x3a67c: ldr r2, [pc, #-0x560]       @ 0x554 <entry+0x554>
    c.literal_cond(C::Al,R::R2,0x3a124)?;
    // 0x3a680: str r0, [sp, #0x214]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,532)?)?;
    // 0x3a684: str r2, [sp, #0x218]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,536)?)?;
    // 0x3a688: add r2, r6, #620
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,620)?)?;
    // 0x3a68c: str r0, [sp, #0x21c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,540)?)?;
    // 0x3a690: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a694: str r2, [sp, #0x220]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,544)?)?;
    // 0x3a698: ldr r2, [pc, #-0x578]       @ 0x558 <entry+0x558>
    c.literal_cond(C::Al,R::R2,0x3a128)?;
    // 0x3a69c: str r0, [sp, #0x224]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,548)?)?;
    // 0x3a6a0: str r2, [sp, #0x228]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,552)?)?;
    // 0x3a6a4: add r2, r6, #109
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,109)?)?;
    // 0x3a6a8: str r0, [sp, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,556)?)?;
    // 0x3a6ac: add r2, r2, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3584)?)?;
    // 0x3a6b0: str r2, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,560)?)?;
    // 0x3a6b4: ldr r2, [pc, #-0x590]       @ 0x55c <entry+0x55c>
    c.literal_cond(C::Al,R::R2,0x3a12c)?;
    // 0x3a6b8: str r0, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,564)?)?;
    // 0x3a6bc: str r2, [sp, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,568)?)?;
    // 0x3a6c0: add r2, r6, #3792
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3792)?)?;
    // 0x3a6c4: str r1, [sp, #0x23c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,572)?)?;
    // 0x3a6c8: str r2, [sp, #0x240]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,576)?)?;
    // 0x3a6cc: ldr r2, [pc, #-0x5a4]       @ 0x560 <entry+0x560>
    c.literal_cond(C::Al,R::R2,0x3a130)?;
    // 0x3a6d0: str r0, [sp, #0x244]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,580)?)?;
    // 0x3a6d4: str r2, [sp, #0x248]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,584)?)?;
    // 0x3a6d8: add r2, r6, #708
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,708)?)?;
    // 0x3a6dc: str r1, [sp, #0x24c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,588)?)?;
    // 0x3a6e0: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a6e4: str r2, [sp, #0x250]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,592)?)?;
    // 0x3a6e8: ldr r2, [pc, #-0x5bc]       @ 0x564 <entry+0x564>
    c.literal_cond(C::Al,R::R2,0x3a134)?;
    // 0x3a6ec: str r0, [sp, #0x254]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,596)?)?;
    // 0x3a6f0: str r2, [sp, #0x258]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,600)?)?;
    // 0x3a6f4: mov r2, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,68)?)?;
    // 0x3a6f8: str r1, [sp, #0x25c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,604)?)?;
    // 0x3a6fc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3a700: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3a704: str r2, [sp, #0x260]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,608)?)?;
    // 0x3a708: ldr r2, [pc, #-0x5d8]       @ 0x568 <entry+0x568>
    c.literal_cond(C::Al,R::R2,0x3a138)?;
    // 0x3a70c: str r0, [sp, #0x264]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,612)?)?;
    // 0x3a710: str r2, [sp, #0x268]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,616)?)?;
    // 0x3a714: add r2, r6, #716
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,716)?)?;
    // 0x3a718: str r1, [sp, #0x26c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,620)?)?;
    // 0x3a71c: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a720: str r2, [sp, #0x270]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,624)?)?;
    // 0x3a724: ldr r2, [pc, #-0x5f0]       @ 0x56c <entry+0x56c>
    c.literal_cond(C::Al,R::R2,0x3a13c)?;
    // 0x3a728: str r0, [sp, #0x274]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,628)?)?;
    // 0x3a72c: str r2, [sp, #0x278]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,632)?)?;
    // 0x3a730: add r2, r6, #3968
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3968)?)?;
    // 0x3a734: str r0, [sp, #0x27c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,636)?)?;
    // 0x3a738: str r2, [sp, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,640)?)?;
    // 0x3a73c: ldr r2, [pc, #-0x604]       @ 0x570 <entry+0x570>
    c.literal_cond(C::Al,R::R2,0x3a140)?;
    // 0x3a740: str r0, [sp, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,644)?)?;
    // 0x3a744: str r2, [sp, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,648)?)?;
    // 0x3a748: add r2, r6, #872
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,872)?)?;
    // 0x3a74c: str r1, [sp, #0x28c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,652)?)?;
    // 0x3a750: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a754: str r2, [sp, #0x290]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,656)?)?;
    // 0x3a758: ldr r2, [pc, #-0x61c]       @ 0x574 <entry+0x574>
    c.literal_cond(C::Al,R::R2,0x3a144)?;
    // 0x3a75c: str r0, [sp, #0x294]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,660)?)?;
    // 0x3a760: str r2, [sp, #0x298]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,664)?)?;
    // 0x3a764: add r2, r6, #3904
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3904)?)?;
    // 0x3a768: str r1, [sp, #0x29c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,668)?)?;
    // 0x3a76c: str r2, [sp, #0x2a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,672)?)?;
    // 0x3a770: ldr r2, [pc, #-0x630]       @ 0x578 <entry+0x578>
    c.literal_cond(C::Al,R::R2,0x3a148)?;
    // 0x3a774: str r0, [sp, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,676)?)?;
    // 0x3a778: str r2, [sp, #0x2a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,680)?)?;
    // 0x3a77c: add r2, r6, #1000
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,1000)?)?;
    // 0x3a780: str r0, [sp, #0x2ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,684)?)?;
    // 0x3a784: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a788: str r2, [sp, #0x2b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,688)?)?;
    // 0x3a78c: ldr r2, [pc, #-0x648]       @ 0x57c <entry+0x57c>
    c.literal_cond(C::Al,R::R2,0x3a14c)?;
    // 0x3a790: str r0, [sp, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,692)?)?;
    // 0x3a794: str r2, [sp, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,696)?)?;
    // 0x3a798: add r2, r6, #3472
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3472)?)?;
    // 0x3a79c: str r1, [sp, #0x2bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,700)?)?;
    // 0x3a7a0: str r2, [sp, #0x2c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,704)?)?;
    // 0x3a7a4: ldr r2, [pc, #-0x65c]       @ 0x580 <entry+0x580>
    c.literal_cond(C::Al,R::R2,0x3a150)?;
    // 0x3a7a8: str r0, [sp, #0x2c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,708)?)?;
    // 0x3a7ac: str r2, [sp, #0x2c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,712)?)?;
    // 0x3a7b0: add r2, r6, #868
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,868)?)?;
    // 0x3a7b4: str r0, [sp, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,716)?)?;
    // 0x3a7b8: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a7bc: str r2, [sp, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,720)?)?;
    // 0x3a7c0: ldr r2, [pc, #-0x674]       @ 0x584 <entry+0x584>
    c.literal_cond(C::Al,R::R2,0x3a154)?;
    // 0x3a7c4: str r0, [sp, #0x2d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,724)?)?;
    // 0x3a7c8: str r2, [sp, #0x2d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,728)?)?;
    // 0x3a7cc: add r2, r6, #396
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,396)?)?;
    // 0x3a7d0: str r1, [sp, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,732)?)?;
    // 0x3a7d4: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3a7d8: str r2, [sp, #0x2e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,736)?)?;
    // 0x3a7dc: ldr r2, [pc, #-0x68c]       @ 0x588 <entry+0x588>
    c.literal_cond(C::Al,R::R2,0x3a158)?;
    // 0x3a7e0: str r0, [sp, #0x2e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,740)?)?;
    // 0x3a7e4: str r2, [sp, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,744)?)?;
    // 0x3a7e8: add r2, r6, #119
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,119)?)?;
    // 0x3a7ec: str r0, [sp, #0x2ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,748)?)?;
    // 0x3a7f0: add r2, r2, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3840)?)?;
    // 0x3a7f4: str r2, [sp, #0x2f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,752)?)?;
    // 0x3a7f8: ldr r2, [pc, #-0x6a4]       @ 0x58c <entry+0x58c>
    c.literal_cond(C::Al,R::R2,0x3a15c)?;
    // 0x3a7fc: str r0, [sp, #0x2f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,756)?)?;
    // 0x3a800: str r2, [sp, #0x2f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,760)?)?;
    // 0x3a804: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x3a808: str r2, [sp, #0x2fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,764)?)?;
    // 0x3a80c: str r3, [sp, #0x300]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,768)?)?;
    // 0x3a810: ldr r3, [pc, #-0x6b8]       @ 0x590 <entry+0x590>
    c.literal_cond(C::Al,R::new(3)?,0x3a160)?;
    // 0x3a814: str r0, [sp, #0x304]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,772)?)?;
    // 0x3a818: str r3, [sp, #0x308]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,776)?)?;
    // 0x3a81c: add r3, r6, #158
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,158)?)?;
    // 0x3a820: str r0, [sp, #0x30c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,780)?)?;
    // 0x3a824: add r3, r3, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,3328)?)?;
    // 0x3a828: str r3, [sp, #0x310]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,784)?)?;
    // 0x3a82c: ldr r3, [pc, #-0x6d0]       @ 0x594 <entry+0x594>
    c.literal_cond(C::Al,R::new(3)?,0x3a164)?;
    // 0x3a830: str r0, [sp, #0x314]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,788)?)?;
    // 0x3a834: str r3, [sp, #0x318]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,792)?)?;
    // 0x3a838: add r3, r6, #3488
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,3488)?)?;
    // 0x3a83c: str r0, [sp, #0x31c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,796)?)?;
    // 0x3a840: str r3, [sp, #0x320]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,800)?)?;
    // 0x3a844: ldr r3, [pc, #-0x6e4]       @ 0x598 <entry+0x598>
    c.literal_cond(C::Al,R::new(3)?,0x3a168)?;
    // 0x3a848: str r0, [sp, #0x324]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,804)?)?;
    // 0x3a84c: str r3, [sp, #0x328]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,808)?)?;
    // 0x3a850: add r3, r6, #492
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,492)?)?;
    // 0x3a854: str r1, [sp, #0x32c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,812)?)?;
    // 0x3a858: add r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,3072)?)?;
    // 0x3a85c: str r3, [sp, #0x330]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,816)?)?;
    // 0x3a860: ldr r3, [pc, #-0x6fc]       @ 0x59c <entry+0x59c>
    c.literal_cond(C::Al,R::new(3)?,0x3a16c)?;
    // 0x3a864: str r0, [sp, #0x334]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,820)?)?;
    // 0x3a868: str r3, [sp, #0x338]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,824)?)?;
    // 0x3a86c: add r3, r6, #250
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,250)?)?;
    // 0x3a870: str r0, [sp, #0x33c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,828)?)?;
    // 0x3a874: add r3, r3, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,3328)?)?;
    // 0x3a878: str r3, [sp, #0x340]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,832)?)?;
    // 0x3a87c: ldr r3, [pc, #-0x714]       @ 0x5a0 <entry+0x5a0>
    c.literal_cond(C::Al,R::new(3)?,0x3a170)?;
    // 0x3a880: str r0, [sp, #0x344]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,836)?)?;
    // 0x3a884: str r3, [sp, #0x348]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,840)?)?;
    // 0x3a888: add r3, r6, #251
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,251)?)?;
    // 0x3a88c: str r4, [sp, #0x34c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,844)?)?;
    // 0x3a890: add r3, r3, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,3328)?)?;
    // 0x3a894: str r3, [sp, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,848)?)?;
    // 0x3a898: ldr r3, [pc, #-0x72c]       @ 0x5a4 <entry+0x5a4>
    c.literal_cond(C::Al,R::new(3)?,0x3a174)?;
    // 0x3a89c: str r0, [sp, #0x354]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,852)?)?;
    // 0x3a8a0: str r3, [sp, #0x358]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,856)?)?;
    // 0x3a8a4: add r3, r6, #508
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,508)?)?;
    // 0x3a8a8: str r0, [sp, #0x35c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,860)?)?;
    // 0x3a8ac: add r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,3072)?)?;
    // 0x3a8b0: str r3, [sp, #0x360]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,864)?)?;
    // 0x3a8b4: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // 0x3a8b8: str r3, [sp, #0x364]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,868)?)?;
    // 0x3a8bc: ldr r3, [pc, #-0x74c]       @ 0x5a8 <entry+0x5a8>
    c.literal_cond(C::Al,R::new(3)?,0x3a178)?;
    // 0x3a8c0: str r3, [sp, #0x368]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,872)?)?;
    // 0x3a8c4: add r3, r6, #520
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,520)?)?;
    // 0x3a8c8: str r1, [sp, #0x36c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,876)?)?;
    // 0x3a8cc: add r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,3072)?)?;
    // 0x3a8d0: str r3, [sp, #0x370]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,880)?)?;
    // 0x3a8d4: str lr, [sp, #0x374]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,884)?)?;
    // 0x3a8d8: str r7, [sp, #0x378]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,888)?)?;
    // 0x3a8dc: add r7, r6, #564
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(6)?,564)?)?;
    // 0x3a8e0: str r4, [sp, #0x37c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,892)?)?;
    // 0x3a8e4: mov r3, #848
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,848)?)?;
    // 0x3a8e8: add r7, r7, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,3072)?)?;
    // 0x3a8ec: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3a8f0: str r7, [sp, #0x380]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,896)?)?;
    // 0x3a8f4: ldr r7, [pc, #-0x780]       @ 0x5ac <entry+0x5ac>
    c.literal_cond(C::Al,R::new(7)?,0x3a17c)?;
    // 0x3a8f8: str r0, [sp, #0x384]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,900)?)?;
    // 0x3a8fc: str r7, [sp, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,904)?)?;
    // 0x3a900: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x3a904: str r7, [sp, #0x38c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,908)?)?;
    // 0x3a908: str r5, [sp, #0x390]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,912)?)?;
    // 0x3a90c: ldr r5, [pc, #-0x794]       @ 0x5b0 <entry+0x5b0>
    c.literal_cond(C::Al,R::new(5)?,0x3a180)?;
    // 0x3a910: str r0, [sp, #0x394]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,916)?)?;
    // 0x3a914: str r5, [sp, #0x398]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,920)?)?;
    // 0x3a918: add r5, r6, #572
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,572)?)?;
    // 0x3a91c: str r7, [sp, #0x39c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,924)?)?;
    // 0x3a920: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3a924: str r5, [sp, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,928)?)?;
    // 0x3a928: ldr r5, [pc, #-0x7ac]       @ 0x5b4 <entry+0x5b4>
    c.literal_cond(C::Al,R::new(5)?,0x3a184)?;
    // 0x3a92c: str r0, [sp, #0x3a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,932)?)?;
    // 0x3a930: str r5, [sp, #0x3a8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,936)?)?;
    // 0x3a934: add r5, r6, #134
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,134)?)?;
    // 0x3a938: str r0, [sp, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,940)?)?;
    // 0x3a93c: add r5, r5, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3584)?)?;
    // 0x3a940: str r5, [sp, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,944)?)?;
    // 0x3a944: ldr r5, [pc, #-0x7c4]       @ 0x5b8 <entry+0x5b8>
    c.literal_cond(C::Al,R::new(5)?,0x3a188)?;
    // 0x3a948: str r0, [sp, #0x3b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,948)?)?;
    // 0x3a94c: str r5, [sp, #0x3b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,952)?)?;
    // 0x3a950: add r5, r6, #676
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,676)?)?;
    // 0x3a954: str r1, [sp, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,956)?)?;
    // 0x3a958: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3a95c: str r5, [sp, #0x3c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,960)?)?;
    // 0x3a960: ldr r5, [pc, #-0x7dc]       @ 0x5bc <entry+0x5bc>
    c.literal_cond(C::Al,R::new(5)?,0x3a18c)?;
    // 0x3a964: str r0, [sp, #0x3c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,964)?)?;
    // 0x3a968: str r5, [sp, #0x3c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,968)?)?;
    // 0x3a96c: add r5, r6, #584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,584)?)?;
    // 0x3a970: str r1, [sp, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,972)?)?;
    // 0x3a974: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3a978: str r5, [sp, #0x3d0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,976)?)?;
    // 0x3a97c: ldr r5, [pc, #-0x7f4]       @ 0x5c0 <entry+0x5c0>
    c.literal_cond(C::Al,R::new(5)?,0x3a190)?;
    // 0x3a980: str r0, [sp, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,980)?)?;
    // 0x3a984: str r5, [sp, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,984)?)?;
    // 0x3a988: add r5, r6, #3664
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,3664)?)?;
    // 0x3a98c: str r1, [sp, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,988)?)?;
    // 0x3a990: str r5, [sp, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,992)?)?;
    // 0x3a994: ldr r5, [pc, #-0x808]       @ 0x5c4 <entry+0x5c4>
    c.literal_cond(C::Al,R::new(5)?,0x3a194)?;
    // 0x3a998: str r0, [sp, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,996)?)?;
    // 0x3a99c: str r5, [sp, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1000)?)?;
    // 0x3a9a0: add r5, r6, #596
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,596)?)?;
    // 0x3a9a4: str r1, [sp, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1004)?)?;
    // 0x3a9a8: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3a9ac: str r5, [sp, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1008)?)?;
    // 0x3a9b0: ldr r5, [pc, #-0x820]       @ 0x5c8 <entry+0x5c8>
    c.literal_cond(C::Al,R::new(5)?,0x3a198)?;
    // 0x3a9b4: str r0, [sp, #0x3f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1012)?)?;
    // 0x3a9b8: str r5, [sp, #0x3f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1016)?)?;
    // 0x3a9bc: add r5, r6, #888
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,888)?)?;
    // 0x3a9c0: str r0, [sp, #0x3fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1020)?)?;
    // 0x3a9c4: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3a9c8: str r5, [sp, #0x400]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1024)?)?;
    // 0x3a9cc: ldr r5, [pc, #-0x838]       @ 0x5cc <entry+0x5cc>
    c.literal_cond(C::Al,R::new(5)?,0x3a19c)?;
    // 0x3a9d0: str r0, [sp, #0x404]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1028)?)?;
    // 0x3a9d4: str r5, [sp, #0x408]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1032)?)?;
    // 0x3a9d8: add r5, r6, #684
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,684)?)?;
    // 0x3a9dc: str r1, [sp, #0x40c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1036)?)?;
    // 0x3a9e0: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3a9e4: str r5, [sp, #0x410]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1040)?)?;
    // 0x3a9e8: ldr r5, [pc, #-0x850]       @ 0x5d0 <entry+0x5d0>
    c.literal_cond(C::Al,R::new(5)?,0x3a1a0)?;
    // 0x3a9ec: str r0, [sp, #0x414]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1044)?)?;
    // 0x3a9f0: str r5, [sp, #0x418]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1048)?)?;
    // 0x3a9f4: add r5, r6, #97
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,97)?)?;
    // 0x3a9f8: str r0, [sp, #0x41c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1052)?)?;
    // 0x3a9fc: add r5, r5, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3584)?)?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    initialize_stream_parameter_matrix(c)
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn matrix_writes_are_contiguous(){
       let mut c=Code::new(START);emit(&mut c).unwrap();
       assert_eq!(c.bytes.len(),END-START);
    }
}
