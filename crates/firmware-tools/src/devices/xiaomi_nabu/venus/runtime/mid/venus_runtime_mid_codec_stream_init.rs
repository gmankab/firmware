// SPDX-License-Identifier: GPL-3.0-only
//! Initialize a codec streaming session, collect plane-format and message
//! metadata, verify resource-ready flags and publish command-buffer state.
//! The six trailing codec/controller diagnostics remain structured data.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x34fac;
const END:usize=0x3548c;
const STREAM_INITIALIZATION_LITERALS:[u32;6]=[
  0x00121002,0x0010ceac,0x000ebf55,
  0x000ec086,0x000e2788,0x000ebe82,
];
pub(super) fn emit(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x34fac: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x34fb0: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x34fb4: sub sp, sp, #548
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,548)?)?;
    // 0x34fb8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x34fbc: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x34fc0: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x34fc4: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // 0x34fc8: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x34fcc: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x34fd0: ldr r1, [pc, #0x49c]        @ 0x36f4 <entry+0x36f4>
    c.literal(R::R1,0x35474)?;
    // 0x34fd4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34fd8: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x34fdc: beq 0x3516c <entry+0x33ec>  @ imm = #0x188
    c.b(C::Eq,0x3516c)?;
    // 0x34fe0: ldr r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // 0x34fe4: bl 0x2bf4 <entry+0xfffffffffffd0e74> @ imm = #-0x323f8
    c.bl(0x2bf4)?;
    // 0x34fe8: mov r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,68)?)?;
    // 0x34fec: mov r10, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,255)?)?;
    // 0x34ff0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x34ff4: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x34ff8: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x34ffc: orr r10, r10, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,65280)?)?;
    // 0x35000: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x35004: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x35008: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x3500c: str r7, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,44)?)?;
    // 0x35010: str r7, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,48)?)?;
    // 0x35014: str r7, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,52)?)?;
    // 0x35018: str r10, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,96)?)?;
    // 0x3501c: ldr r0, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // 0x35020: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35024: beq 0x350cc <entry+0x334c>  @ imm = #0xa0
    c.b(C::Eq,0x350cc)?;
    // 0x35028: add r1, r4, #1632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,1632)?)?;
    // 0x3502c: mvn r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(8)?,R::R0,0)?)?;
    // 0x35030: add r5, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R1,4096)?)?;
    // 0x35034: ldr r1, [pc, #0x43c]        @ 0x36f8 <entry+0x36f8>
    c.literal(R::R1,0x35478)?;
    // 0x35038: ldr r6, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,0)?)?;
    // 0x3503c: ldr r1, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,92)?)?;
    // 0x35040: ldr r9, [r1, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(9)?,R::R1,R::new(7)?,Shift::Lsl,2)?)?;
    // 0x35044: ldr r1, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,52)?)?;
    // 0x35048: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3504c: ldrne r1, [r9, #0xa8]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(9)?,168)?)?;
    // 0x35050: cmnne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmn,R::R0,R::R1,1)?)?;
    // 0x35054: beq 0x350c0 <entry+0x3340>  @ imm = #0x64
    c.b(C::Eq,0x350c0)?;
    // 0x35058: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3505c: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x35060: add r2, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,28)?)?;
    // 0x35064: bl 0x356e8 <entry+0x3968>  @ imm = #0x67c
    c.bl(0x356e8)?;
    // 0x35068: ldr r0, [r9, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,92)?)?;
    // 0x3506c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x35070: str r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // 0x35074: ldr r1, [r9, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,96)?)?;
    // 0x35078: str r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,68)?)?;
    // 0x3507c: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // 0x35080: str r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,76)?)?;
    // 0x35084: add r1, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,32)?)?;
    // 0x35088: ldr r0, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,52)?)?;
    // 0x3508c: str r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,80)?)?;
    // 0x35090: ldr r0, [r9, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,60)?)?;
    // 0x35094: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x35098: ldr r0, [r9, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,48)?)?;
    // 0x3509c: str r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,88)?)?;
    // 0x350a0: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x350a4: str r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // 0x350a8: str r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,92)?)?;
    // 0x350ac: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x350b0: str r10, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,96)?)?;
    // 0x350b4: bl 0x2a18c <entry+0xffffffffffff840c> @ imm = #-0xaf30
    c.bl(0x2a18c)?;
    // 0x350b8: str r8, [r9, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,168)?)?;
    // 0x350bc: ldr r0, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // 0x350c0: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x350c4: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x350c8: blo 0x3503c <entry+0x32bc>  @ imm = #-0x94
    c.b(C::Cc,0x3503c)?;
    // 0x350cc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x350d0: mov r8, r10
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(10)?))?;
    // 0x350d4: str r0, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,108)?)?;
    // 0x350d8: str r0, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,112)?)?;
    // 0x350dc: str r0, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,116)?)?;
    // 0x350e0: str r0, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,100)?)?;
    // 0x350e4: ldr r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // 0x350e8: bl 0x2c34 <entry+0xfffffffffffd0eb4> @ imm = #-0x324bc
    c.bl(0x2c34)?;
    // 0x350ec: add r0, r4, #1632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,1632)?)?;
    // 0x350f0: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x350f4: add r2, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,28)?)?;
    // 0x350f8: add r5, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,4096)?)?;
    // 0x350fc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x35100: bl 0x356e8 <entry+0x3968>  @ imm = #0x5e0
    c.bl(0x356e8)?;
    // 0x35104: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x35108: bne 0x3516c <entry+0x33ec>  @ imm = #0x5c
    c.b(C::Ne,0x3516c)?;
    // 0x3510c: ldr r0, [pc, #0x364]        @ 0x36f8 <entry+0x36f8>
    c.literal(R::R0,0x35478)?;
    // 0x35110: add r9, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::SP,16)?)?;
    // 0x35114: add r10, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,28)?)?;
    // 0x35118: ldr r6, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,0)?)?;
    // 0x3511c: add r0, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,32)?)?;
    // 0x35120: add r7, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,32)?)?;
    // 0x35124: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x35128: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3512c: mov r2, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // 0x35130: bl 0x31ec <entry+0xfffffffffffd146c> @ imm = #-0x31f4c
    c.bl(0x31ec)?;
    // 0x35134: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x35138: add r1, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,32)?)?;
    // 0x3513c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x35140: str r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // 0x35144: str r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,92)?)?;
    // 0x35148: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x3514c: str r8, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,96)?)?;
    // 0x35150: bl 0x2a18c <entry+0xffffffffffff840c> @ imm = #-0xafcc
    c.bl(0x2a18c)?;
    // 0x35154: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x35158: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x3515c: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x35160: bl 0x356e8 <entry+0x3968>  @ imm = #0x580
    c.bl(0x356e8)?;
    // 0x35164: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x35168: beq 0x35124 <entry+0x33a4>  @ imm = #-0x4c
    c.b(C::Eq,0x35124)?;
    // 0x3516c: mov r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,68)?)?;
    // 0x35170: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x35174: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x35178: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x3517c: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x35180: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x35184: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x35188: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x3518c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35190: str r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // 0x35194: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // 0x35198: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // 0x3519c: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x351a0: str r1, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,96)?)?;
    // 0x351a4: str r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,92)?)?;
    // 0x351a8: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x351ac: add r0, r0, #1600
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1600)?)?;
    // 0x351b0: bl 0x35d68 <entry+0x3fe8>  @ imm = #0xbb0
    c.bl(0x35d68)?;
    // 0x351b4: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x351b8: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x351bc: beq 0x35234 <entry+0x34b4>  @ imm = #0x70
    c.b(C::Eq,0x35234)?;
    // 0x351c0: ldr r0, [pc, #0x2b0]        @ 0x36f8 <entry+0x36f8>
    c.literal(R::R0,0x35478)?;
    // 0x351c4: add r6, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::SP,32)?)?;
    // 0x351c8: ldr r5, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,0)?)?;
    // 0x351cc: ldrd r0, r1, [r7, #192]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R0,R::new(7)?,192)?)?;
    // 0x351d0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x351d4: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,64)?)?;
    // 0x351d8: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // 0x351dc: ldrd r0, r1, [r7, #56]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R0,R::new(7)?,56)?)?;
    // 0x351e0: str r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // 0x351e4: str r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,76)?)?;
    // 0x351e8: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x351ec: ldr r0, [r7, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,68)?)?;
    // 0x351f0: str r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,80)?)?;
    // 0x351f4: ldr r0, [r7, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,292)?)?;
    // 0x351f8: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x351fc: ldr r0, [r7, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,44)?)?;
    // 0x35200: str r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,88)?)?;
    // 0x35204: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x35208: bl 0x2a18c <entry+0xffffffffffff840c> @ imm = #-0xb084
    c.bl(0x2a18c)?;
    // 0x3520c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x35210: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x35214: mov r2, #448
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,448)?)?;
    // 0x35218: bl 0x31ec <entry+0xfffffffffffd146c> @ imm = #-0x32034
    c.bl(0x31ec)?;
    // 0x3521c: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x35220: add r0, r0, #1600
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1600)?)?;
    // 0x35224: bl 0x35d68 <entry+0x3fe8>  @ imm = #0xb3c
    c.bl(0x35d68)?;
    // 0x35228: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x3522c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x35230: bne 0x351cc <entry+0x344c>  @ imm = #-0x6c
    c.b(C::Ne,0x351cc)?;
    // 0x35234: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x35238: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x3523c: bne 0x35390 <entry+0x3610>  @ imm = #0x14c
    c.b(C::Ne,0x35390)?;
    // 0x35240: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x35244: add r5, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,64)?)?;
    // 0x35248: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3524c: bl 0x35d68 <entry+0x3fe8>  @ imm = #0xb14
    c.bl(0x35d68)?;
    // 0x35250: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x35254: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x35258: beq 0x35388 <entry+0x3608>  @ imm = #0x128
    c.b(C::Eq,0x35388)?;
    // 0x3525c: mov r10, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,76)?)?;
    // 0x35260: mov r8, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,68)?)?;
    // 0x35264: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x35268: add r6, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::SP,32)?)?;
    // 0x3526c: ldr r0, [r7, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,368)?)?;
    // 0x35270: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35274: bne 0x35364 <entry+0x35e4>  @ imm = #0xe8
    c.b(C::Ne,0x35364)?;
    // 0x35278: ldr r0, [r7, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,384)?)?;
    // 0x3527c: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x35280: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x35284: str r10, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,32)?)?;
    // 0x35288: str r8, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,40)?)?;
    // 0x3528c: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x35290: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x35294: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // 0x35298: str r9, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,52)?)?;
    // 0x3529c: ldr r0, [r7, #0x178]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,376)?)?;
    // 0x352a0: ldr r1, [r7, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,380)?)?;
    // 0x352a4: str r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // 0x352a8: str r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,76)?)?;
    // 0x352ac: ldrd r0, r1, [r7, #56]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R0,R::new(7)?,56)?)?;
    // 0x352b0: str r1, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,80)?)?;
    // 0x352b4: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x352b8: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x352bc: ldr r0, [r7, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,68)?)?;
    // 0x352c0: str r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,88)?)?;
    // 0x352c4: ldr r0, [r7, #0x184]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,388)?)?;
    // 0x352c8: str r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,92)?)?;
    // 0x352cc: ldr r0, [r7, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,44)?)?;
    // 0x352d0: str r0, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,96)?)?;
    // 0x352d4: ldr r0, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,20)?)?;
    // 0x352d8: str r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,104)?)?;
    // 0x352dc: str r9, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,60)?)?;
    // 0x352e0: ldr r0, [r7, #0x164]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,356)?)?;
    // 0x352e4: str r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // 0x352e8: str r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,100)?)?;
    // 0x352ec: ldr r0, [r4, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,276)?)?;
    // 0x352f0: add r0, r0, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,200)?)?;
    // 0x352f4: bl 0x80928 <entry+0x4eba8> @ imm = #0x4b62c
    c.bl(0x80928)?;
    // 0x352f8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x352fc: beq 0x35334 <entry+0x35b4>  @ imm = #0x30
    c.b(C::Eq,0x35334)?;
    // 0x35300: ldr r1, [pc, #0x174]        @ 0x36fc <entry+0x36fc>
    c.literal(R::R1,0x3547c)?;
    // 0x35304: ldr r2, [pc, #0x174]        @ 0x3700 <entry+0x3700>
    c.literal(R::R2,0x35480)?;
    // 0x35308: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x3530c: mov r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,768)?)?;
    // 0x35310: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x6684
    c.bl(0x2ec94)?;
    // 0x35314: ldr r1, [pc, #0x168]        @ 0x3704 <entry+0x3704>
    c.literal(R::R1,0x35484)?;
    // 0x35318: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x3531c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x6690
    c.bl(0x2ec94)?;
    // 0x35320: ldr r2, [pc, #0x160]        @ 0x3708 <entry+0x3708>
    c.literal(R::R2,0x35488)?;
    // 0x35324: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x35328: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3532c: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x35330: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x33b88
    c.bl(0x17b0)?;
    // 0x35334: ldr r0, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,124)?)?;
    // 0x35338: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3533c: ldrne r1, [r7, #0x164]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(7)?,356)?)?;
    // 0x35340: cmpne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x35344: beq 0x3534c <entry+0x35cc>  @ imm = #0x0
    c.b(C::Eq,0x3534c)?;
    // 0x35348: bl 0x7fd1c <entry+0x4df9c> @ imm = #0x4a9cc
    c.bl(0x7fd1c)?;
    // 0x3534c: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // 0x35350: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35354: ldrne r1, [r7, #0x160]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(7)?,352)?)?;
    // 0x35358: cmpne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3535c: beq 0x35364 <entry+0x35e4>  @ imm = #0x0
    c.b(C::Eq,0x35364)?;
    // 0x35360: bl 0x7fd1c <entry+0x4df9c> @ imm = #0x4a9b4
    c.bl(0x7fd1c)?;
    // 0x35364: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x35368: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3536c: mov r2, #416
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,416)?)?;
    // 0x35370: bl 0x31ec <entry+0xfffffffffffd146c> @ imm = #-0x3218c
    c.bl(0x31ec)?;
    // 0x35374: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x35378: bl 0x35d68 <entry+0x3fe8>  @ imm = #0x9e8
    c.bl(0x35d68)?;
    // 0x3537c: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x35380: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x35384: bne 0x3526c <entry+0x34ec>  @ imm = #-0x120
    c.b(C::Ne,0x3526c)?;
    // 0x35388: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x3538c: bl 0x761bc <entry+0x4443c> @ imm = #0x40e28
    c.bl(0x761bc)?;
    // 0x35390: ldr r0, [pc, #0xe0]         @ 0x36f8 <entry+0x36f8>
    c.literal(R::R0,0x35478)?;
    // 0x35394: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x35398: ldr r0, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
    // 0x3539c: bl 0x252a8 <entry+0xffffffffffff3528> @ imm = #-0x100fc
    c.bl(0x252a8)?;
    // 0x353a0: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x353a4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x353a8: ldr r0, [r0, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,420)?)?;
    // 0x353ac: str r1, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,52)?)?;
    // 0x353b0: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x353b4: add r5, r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,28)?)?;
    // 0x353b8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x353bc: bl 0x35df4 <entry+0x4074>  @ imm = #0xa30
    c.bl(0x35df4)?;
    // 0x353c0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x353c4: bne 0x353b8 <entry+0x3638>  @ imm = #-0x14
    c.b(C::Ne,0x353b8)?;
    // 0x353c8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x353cc: bl 0x59604 <entry+0x27884> @ imm = #0x24230
    c.bl(0x59604)?;
    // 0x353d0: ldr r0, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,284)?)?;
    // 0x353d4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x353d8: add r1, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,32)?)?;
    // 0x353dc: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x353e0: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // 0x353e4: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x353e8: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x353ec: str r5, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,52)?)?;
    // 0x353f0: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x353f4: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x353f8: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x353fc: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // 0x35400: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x35404: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x35408: ldr r0, [r4, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,276)?)?;
    // 0x3540c: add r0, r0, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,200)?)?;
    // 0x35410: bl 0x80928 <entry+0x4eba8> @ imm = #0x4b510
    c.bl(0x80928)?;
    // 0x35414: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x35418: beq 0x35450 <entry+0x36d0>  @ imm = #0x30
    c.b(C::Eq,0x35450)?;
    // 0x3541c: ldr r1, [pc, #0x58]         @ 0x36fc <entry+0x36fc>
    c.literal(R::R1,0x3547c)?;
    // 0x35420: ldr r2, [pc, #0x58]         @ 0x3700 <entry+0x3700>
    c.literal(R::R2,0x35480)?;
    // 0x35424: mov r3, #47
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,47)?)?;
    // 0x35428: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x3542c: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // 0x35430: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x67a4
    c.bl(0x2ec94)?;
    // 0x35434: ldr r1, [pc, #0x48]         @ 0x3704 <entry+0x3704>
    c.literal(R::R1,0x35484)?;
    // 0x35438: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x3543c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x67b0
    c.bl(0x2ec94)?;
    // 0x35440: ldr r2, [pc, #0x40]         @ 0x3708 <entry+0x3708>
    c.literal(R::R2,0x35488)?;
    // 0x35444: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35448: mov r1, #816
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,816)?)?;
    // 0x3544c: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x33ca4
    c.bl(0x17b0)?;
    // 0x35450: strb r5, [r4, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,12)?)?;
    // 0x35454: ldrb r0, [r4, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,990)?)?;
    // 0x35458: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x3545c: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35460: strbeq r0, [r4, #0xd]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R0,R::new(4)?,13)?)?;
    // 0x35464: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x35468: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x3546c: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x35470: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
 c.check(0x35474)?;
 for word in STREAM_INITIALIZATION_LITERALS {c.emit(word)?;}
 c.check(END)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn codec_stream_initialization_preserves_the_six_literal_words(){
  assert_eq!(END-START,306*4+6*4);
  let mut c=Code::new(START);emit(&mut c).unwrap();assert_eq!(c.at(),END);
  assert_eq!(STREAM_INITIALIZATION_LITERALS[1],0x0010ceac);
 }
}
