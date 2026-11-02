
/// Retire a controller codec session, publish final frame status, and
/// release the registered codec slot through the platform callbacks.
fn retire_codec_controller_slot(c:&mut Code)->Result<(),String>{
    c.check(0xbb570)?;
    // VMA 0xbb570: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbb574: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xbb578: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xbb57c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xbb580: ldr r5, [r4, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,364)?)?;
    // VMA 0xbb584: ldr r6, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,4)?)?;
    // VMA 0xbb588: ldr r0, [r5, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,472)?)?;
    // VMA 0xbb58c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbb590: ldreq r0, [r5, #0x234]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(5)?,564)?)?;
    // VMA 0xbb594: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbb598: beq 0xbb5c0 <entry+0x548>   @ imm = #0x20
    c.b(C::Eq,0xbb5c0,false)?;
    // VMA 0xbb59c: ldr r0, [r5, #0x228]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,552)?)?;
    // VMA 0xbb5a0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbb5a4: bne 0xbb5cc <entry+0x554>   @ imm = #0x20
    c.b(C::Ne,0xbb5cc,false)?;
    // VMA 0xbb5a8: ldr r0, [r5, #0x214]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,532)?)?;
    // VMA 0xbb5ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbb5b0: beq 0xbb5cc <entry+0x554>   @ imm = #0x14
    c.b(C::Eq,0xbb5cc,false)?;
    // VMA 0xbb5b4: ldr r0, [r5, #0x208]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,520)?)?;
    // VMA 0xbb5b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbb5bc: bne 0xbb5cc <entry+0x554>   @ imm = #0x8
    c.b(C::Ne,0xbb5cc,false)?;
    // VMA 0xbb5c0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xbb5c4: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xbb5c8: bl 0xb7f94 <entry+0xffffffffffffcf1c> @ imm = #-0x363c
    c.b(C::Al,0xb7f94,true)?;
    // VMA 0xbb5cc: add r0, r4, #244
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,244)?)?;
    // VMA 0xbb5d0: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xbb5d4: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // VMA 0xbb5d8: bl 0x80928 <entry+0xfffffffffffc58b0> @ imm = #-0x3acb8
    c.b(C::Al,0x80928,true)?;
    // VMA 0xbb5dc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbb5e0: beq 0xbb61c <entry+0x5a4>   @ imm = #0x34
    c.b(C::Eq,0xbb61c,false)?;
    // VMA 0xbb5e4: ldr r1, [pc, #0x58]         @ 0x5cc <entry+0x5cc>
    c.literal(R::R1,0xbb644)?;
    // VMA 0xbb5e8: ldr r2, [pc, #0x58]         @ 0x5d0 <entry+0x5d0>
    c.literal(R::R2,0xbb648)?;
    // VMA 0xbb5ec: mov r3, #41
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,41)?)?;
    // VMA 0xbb5f0: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbb5f4: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // VMA 0xbb5f8: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c96c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb5fc: ldr r1, [pc, #0x48]         @ 0x5d4 <entry+0x5d4>
    c.literal(R::R1,0xbb64c)?;
    // VMA 0xbb600: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbb604: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c978
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb608: ldr r2, [pc, #0x40]         @ 0x5d8 <entry+0x5d8>
    c.literal(R::R2,0xbb650)?;
    // VMA 0xbb60c: mov r1, #42
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,42)?)?;
    // VMA 0xbb610: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbb614: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xbb618: bl 0x17b0 <entry+0xfffffffffff46738> @ imm = #-0xb9e70
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbb61c: ldr r0, [r6, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,36)?)?;
    // VMA 0xbb620: bl 0x292c <entry+0xfffffffffff478b4> @ imm = #-0xb8cfc
    c.b(C::Al,0x292c,true)?;
    // VMA 0xbb624: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbb628: str r0, [r4, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,364)?)?;
    // VMA 0xbb62c: str r0, [r4, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,232)?)?;
    // VMA 0xbb630: ldr r0, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,28)?)?;
    // VMA 0xbb634: bl 0x292c <entry+0xfffffffffff478b4> @ imm = #-0xb8d10
    c.b(C::Al,0x292c,true)?;
    // VMA 0xbb638: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbb63c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // VMA 0xbb640: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xbb644)
}

const CODEC_SESSION_RETIRE_DIAGNOSTICS:[u32;4]=[
    0x000f_9bab,0x000f_9f5c,0x000e_2788,0x000f_9ad3,
];

/// Switch codec external references and transport-ready mode while preserving
/// the original register-only fast path and terminal diagnostic thunk.
fn update_external_codec_reference(c:&mut Code)->Result<(),String>{
    c.check(0xbb654)?;
    // VMA 0xbb654: ldr r2, [r0, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,364)?)?;
    // VMA 0xbb658: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xbb65c: beq 0xbb66c <entry+0x5f4>   @ imm = #0x8
    c.b(C::Eq,0xbb66c,false)?;
    // VMA 0xbb660: ldr r3, [r0, #0x174]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,372)?)?;
    // VMA 0xbb664: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xbb668: strne r3, [r2, #0x1bc]
    c.emit(arm32::transfer_word(C::Ne,false,R::new(3)?,R::R2,444)?)?;
    // VMA 0xbb66c: ldr r3, [r0, #0x178]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,376)?)?;
    // VMA 0xbb670: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbb674: cmpne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xbb678: str r3, [r0, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,372)?)?;
    // VMA 0xbb67c: ldr r3, [r0, #0x878]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,2168)?)?;
    // VMA 0xbb680: str r3, [r0, #0x874]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,2164)?)?;
    // VMA 0xbb684: str r1, [r0, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,376)?)?;
    // VMA 0xbb688: beq 0xbb6b0 <entry+0x638>   @ imm = #0x20
    c.b(C::Eq,0xbb6b0,false)?;
    // VMA 0xbb68c: ldrb r3, [r2, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,8)?)?;
    // VMA 0xbb690: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xbb694: beq 0xbb6a8 <entry+0x630>   @ imm = #0xc
    c.b(C::Eq,0xbb6a8,false)?;
    // VMA 0xbb698: ldr r2, [r2, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,484)?)?;
    // VMA 0xbb69c: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xbb6a0: and r2, r3, r2, lsr #3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,3,false)?)?;
    // VMA 0xbb6a4: b 0xbb6ac <entry+0x634>   @ imm = #0x0
    c.b(C::Al,0xbb6ac,false)?;
    // VMA 0xbb6a8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbb6ac: str r2, [r0, #0x878]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,2168)?)?;
    // VMA 0xbb6b0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbb6b4: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // VMA 0xbb6b8: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // VMA 0xbb6bc: b 0x7ffc8 <entry+0xfffffffffffc4f50> @ imm = #-0x3b6fc
    c.b(C::Al,0x7ffc8,false)?;
    // VMA 0xbb6c0: ldr r2, [pc, #0xc]          @ 0x65c <entry+0x65c>
    c.literal(R::R2,0xbb6d4)?;
    // VMA 0xbb6c4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbb6c8: mov r1, #33
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33)?)?;
    // VMA 0xbb6cc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbb6d0: b 0x17b0 <entry+0xfffffffffff46738> @ imm = #-0xb9f28
    c.b(C::Al,0x17b0,false)?;
    c.check(0xbb6d4)
}
const EXTERNAL_CODEC_WARNING:u32=0x000f_9ffa;

/// Encode device pipeline setup for variable-width codec input frames,
/// serializing format flags and output-controller registers into the hardware
/// queue with explicit address calculation. The original routine returns
/// through its ARM saved-register epilogue at 0xbbaa8.
fn program_codec_packet_registers(c:&mut Code)->Result<(),String>{
    c.check(0xbb6d8)?;
    // VMA 0xbb6d8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbb6dc: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xbb6e0: sub sp, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // VMA 0xbb6e4: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // VMA 0xbb6e8: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xbb6ec: mov r10, r1
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // VMA 0xbb6f0: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // VMA 0xbb6f4: ldr r7, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,4)?)?;
    // VMA 0xbb6f8: ldr r2, [r4, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,456)?)?;
    // VMA 0xbb6fc: ldr r3, [r4, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,448)?)?;
    // VMA 0xbb700: ldr r12, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(10)?,4)?)?;
    // VMA 0xbb704: lsl r6, r0, #20
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::R0,Shift::Lsl,20)?)?;
    // VMA 0xbb708: ldrb r0, [r4, #0x10]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // VMA 0xbb70c: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // VMA 0xbb710: str r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,4)?)?;
    // VMA 0xbb714: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // VMA 0xbb718: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // VMA 0xbb71c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xbb720: bhs 0xbb734 <entry+0x6bc>   @ imm = #0xc
    c.b(C::Cs,0xbb734,false)?;
    // VMA 0xbb724: ldr r1, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,52)?)?;
    // VMA 0xbb728: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xbb72c: bic r6, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::R1,1)?)?;
    // VMA 0xbb730: b 0xbb788 <entry+0x710>   @ imm = #0x50
    c.b(C::Al,0xbb788,false)?;
    // VMA 0xbb734: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbb738: bne 0xbb748 <entry+0x6d0>   @ imm = #0x8
    c.b(C::Ne,0xbb748,false)?;
    // VMA 0xbb73c: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,52)?)?;
    // VMA 0xbb740: lsl r9, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::R0,Shift::Lsl,4)?)?;
    // VMA 0xbb744: b 0xbb7ec <entry+0x774>   @ imm = #0xa0
    c.b(C::Al,0xbb7ec,false)?;
    // VMA 0xbb748: ldr r1, [pc, #0x35c]        @ 0xa34 <entry+0xa34>
    c.literal(R::R1,0xbbaac)?;
    // VMA 0xbb74c: ldr r2, [pc, #0x35c]        @ 0xa38 <entry+0xa38>
    c.literal(R::R2,0xbbab0)?;
    // VMA 0xbb750: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbb754: mov r3, #73
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,73)?)?;
    // VMA 0xbb758: mov r5, r12
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R12))?;
    // VMA 0xbb75c: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8cad0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb760: ldr r1, [pc, #0x34c]        @ 0xa3c <entry+0xa3c>
    c.literal(R::R1,0xbbab4)?;
    // VMA 0xbb764: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbb768: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8cadc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb76c: ldr r2, [pc, #0x344]        @ 0xa40 <entry+0xa40>
    c.literal(R::R2,0xbbab8)?;
    // VMA 0xbb770: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbb774: mov r1, #74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,74)?)?;
    // VMA 0xbb778: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xbb77c: bl 0x17b0 <entry+0xfffffffffff46738> @ imm = #-0xb9fd4
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbb780: ldrb r0, [r4, #0x10]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // VMA 0xbb784: mov r12, r5
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(5)?))?;
    // VMA 0xbb788: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // VMA 0xbb78c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xbb790: bhs 0xbb7a0 <entry+0x728>   @ imm = #0x8
    c.b(C::Cs,0xbb7a0,false)?;
    // VMA 0xbb794: mvn r0, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,2147483663)?)?;
    // VMA 0xbb798: and r9, r0, r6, lsl #3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::R0,R::new(6)?,Shift::Lsl,3,false)?)?;
    // VMA 0xbb79c: b 0xbb7ec <entry+0x774>   @ imm = #0x48
    c.b(C::Al,0xbb7ec,false)?;
    // VMA 0xbb7a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbb7a4: bne 0xbb7b0 <entry+0x738>   @ imm = #0x4
    c.b(C::Ne,0xbb7b0,false)?;
    // VMA 0xbb7a8: lsl r9, r6, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::new(6)?,Shift::Lsl,4)?)?;
    // VMA 0xbb7ac: b 0xbb7ec <entry+0x774>   @ imm = #0x38
    c.b(C::Al,0xbb7ec,false)?;
    // VMA 0xbb7b0: ldr r1, [pc, #0x2f4]        @ 0xa34 <entry+0xa34>
    c.literal(R::R1,0xbbaac)?;
    // VMA 0xbb7b4: ldr r2, [pc, #0x2f4]        @ 0xa38 <entry+0xa38>
    c.literal(R::R2,0xbbab0)?;
    // VMA 0xbb7b8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbb7bc: mov r3, #97
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,97)?)?;
    // VMA 0xbb7c0: mov r5, r12
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R12))?;
    // VMA 0xbb7c4: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8cb38
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb7c8: ldr r1, [pc, #0x2e4]        @ 0xa3c <entry+0xa3c>
    c.literal(R::R1,0xbbab4)?;
    // VMA 0xbb7cc: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbb7d0: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8cb44
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb7d4: ldr r2, [pc, #0x2dc]        @ 0xa40 <entry+0xa40>
    c.literal(R::R2,0xbbab8)?;
    // VMA 0xbb7d8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbb7dc: mov r1, #98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,98)?)?;
    // VMA 0xbb7e0: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // VMA 0xbb7e4: bl 0x17b0 <entry+0xfffffffffff46738> @ imm = #-0xba03c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbb7e8: mov r12, r5
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(5)?))?;
    // VMA 0xbb7ec: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xbb7f0: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // VMA 0xbb7f4: cmp r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // VMA 0xbb7f8: mov lr, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,3758096384)?)?;
    // VMA 0xbb7fc: mov r7, #65
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,65)?)?;
    // VMA 0xbb800: mov r2, #856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,856)?)?;
    // VMA 0xbb804: mov r1, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4194304)?)?;
    // VMA 0xbb808: orr r0, r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16384)?)?;
    // VMA 0xbb80c: orr r6, r6, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,16384)?)?;
    // VMA 0xbb810: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // VMA 0xbb814: moveq r6, r0
    c.emit(arm32::mov_register(C::Eq,R::new(6)?,R::R0))?;
    // VMA 0xbb818: mov r0, #860
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,860)?)?;
    // VMA 0xbb81c: orr r2, r2, r8, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(8)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbb820: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbb824: orr r0, r0, r8, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(8)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbb828: str r7, [r0, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::R0,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbb82c: ldr r3, [r10, #0x888]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2184)?)?;
    // VMA 0xbb830: str r3, [r2, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R2,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbb834: mov r3, #984
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,984)?)?;
    // VMA 0xbb838: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // VMA 0xbb83c: orr r3, r3, r8, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(8)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbb840: str r7, [r3, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::new(3)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbb844: mov r7, #980
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,980)?)?;
    // VMA 0xbb848: ldr r5, [r10, #0x88c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,2188)?)?;
    // VMA 0xbb84c: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // VMA 0xbb850: orr r7, r7, r8, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(8)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbb854: bic r5, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,255)?)?;
    // VMA 0xbb858: str r5, [r7, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(5)?,R::new(7)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbb85c: ldr r5, [r12, #0x45c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R12,1116)?)?;
    // VMA 0xbb860: and r1, r1, r5, lsl #7
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(5)?,Shift::Lsl,7,false)?)?;
    // VMA 0xbb864: mov r5, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,48)?)?;
    // VMA 0xbb868: orr r1, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbb86c: ldr r6, [r4, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,732)?)?;
    // VMA 0xbb870: and r6, r5, r6, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,4,false)?)?;
    // VMA 0xbb874: mov r5, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,192)?)?;
    // VMA 0xbb878: orr r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb87c: ldrb r6, [r4, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,8)?)?;
    // VMA 0xbb880: and r6, r5, r6, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,6,false)?)?;
    // VMA 0xbb884: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbb888: orr r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb88c: ldr r6, [r12, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R12,2308)?)?;
    // VMA 0xbb890: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // VMA 0xbb894: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // VMA 0xbb898: movne r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xbb89c: orr r1, r1, r5, lsl #30
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,30,false)?)?;
    // VMA 0xbb8a0: ldr r5, [r10, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,156)?)?;
    // VMA 0xbb8a4: cmp r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // VMA 0xbb8a8: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbb8ac: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xbb8b0: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // VMA 0xbb8b4: orr r1, r1, r5, lsl #23
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,23,false)?)?;
    // VMA 0xbb8b8: mov r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3072)?)?;
    // VMA 0xbb8bc: orr r5, r5, r8, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(5)?,R::new(8)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbb8c0: str r1, [r5, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(5)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbb8c4: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbb8c8: ldr r5, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,0)?)?;
    // VMA 0xbb8cc: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    // VMA 0xbb8d0: orr r1, r1, r8, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(8)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbb8d4: orr r5, r9, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb8d8: str r5, [r1, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R1,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbb8dc: mov r1, #3504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3504)?)?;
    // VMA 0xbb8e0: mov r5, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,64)?)?;
    // VMA 0xbb8e4: orr r1, r1, r8, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(8)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbb8e8: str r5, [r1, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R1,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbb8ec: mov r1, #97
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,97)?)?;
    // VMA 0xbb8f0: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xbb8f4: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xbb8f8: ldr r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,4)?)?;
    // VMA 0xbb8fc: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xbb900: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbb904: ldr r1, [r4, #0x204]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,516)?)?;
    // VMA 0xbb908: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xbb90c: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xbb910: mov r1, #97
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,97)?)?;
    // VMA 0xbb914: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xbb918: ldr r5, [r10, #0x880]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,2176)?)?;
    // VMA 0xbb91c: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xbb920: mov r5, #16580608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16580608)?)?;
    // VMA 0xbb924: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbb928: mov r1, #113
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,113)?)?;
    // VMA 0xbb92c: orr r5, r5, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,33554432)?)?;
    // VMA 0xbb930: str r5, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,0)?)?;
    // VMA 0xbb934: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xbb938: ldr r0, [r10, #0x884]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2180)?)?;
    // VMA 0xbb93c: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbb940: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbb944: mov r3, #120832
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,120832)?)?;
    // VMA 0xbb948: lsl r0, r8, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(8)?,Shift::Lsl,15)?)?;
    // VMA 0xbb94c: orr r3, r3, #12320768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12320768)?)?;
    // VMA 0xbb950: str r3, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(7)?,0)?)?;
    // VMA 0xbb954: bne 0xbb968 <entry+0x8f0>   @ imm = #0xc
    c.b(C::Ne,0xbb968,false)?;
    // VMA 0xbb958: orr r1, r0, #3440
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,3440)?)?;
    // VMA 0xbb95c: str r5, [r1, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R1,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbb960: mov r1, #3456
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3456)?)?;
    // VMA 0xbb964: b 0xbb980 <entry+0x908>   @ imm = #0x14
    c.b(C::Al,0xbb980,false)?;
    // VMA 0xbb968: orr r1, r0, #364
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,364)?)?;
    // VMA 0xbb96c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xbb970: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbb974: str r12, [r1, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R12,R::R1,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbb978: mov r1, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,360)?)?;
    // VMA 0xbb97c: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbb980: orr r2, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbb984: mov r12, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xbb988: ldr r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,8)?)?;
    // VMA 0xbb98c: mov r1, #240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,240)?)?;
    // VMA 0xbb990: str r3, [r2, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xbb994: orr r2, r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,28)?)?;
    // VMA 0xbb998: ldr r3, [r4, #0x250]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,592)?)?;
    // VMA 0xbb99c: orr r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3072)?)?;
    // VMA 0xbb9a0: str r3, [r2, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xbb9a4: orr r2, r0, #436
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,436)?)?;
    // VMA 0xbb9a8: orr r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3072)?)?;
    // VMA 0xbb9ac: str r7, [r2, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xbb9b0: orr r2, r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,56)?)?;
    // VMA 0xbb9b4: orr r0, r0, #3696
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3696)?)?;
    // VMA 0xbb9b8: ldr r3, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,476)?)?;
    // VMA 0xbb9bc: orr r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3072)?)?;
    // VMA 0xbb9c0: str r3, [r2, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xbb9c4: mov r3, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3840)?)?;
    // VMA 0xbb9c8: ldrb r6, [r4, #0xb]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,11)?)?;
    // VMA 0xbb9cc: ldrb r5, [r4, #0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,12)?)?;
    // VMA 0xbb9d0: ldrb r2, [r4, #0xd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,13)?)?;
    // VMA 0xbb9d4: and r1, r1, r5, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(5)?,Shift::Lsl,4,false)?)?;
    // VMA 0xbb9d8: and r6, r6, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,15)?)?;
    // VMA 0xbb9dc: and r7, r3, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(3)?,R::R2,Shift::Lsl,8,false)?)?;
    // VMA 0xbb9e0: ldrb r3, [r4, #0x9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,9)?)?;
    // VMA 0xbb9e4: ldrb r2, [r4, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,10)?)?;
    // VMA 0xbb9e8: orr r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb9ec: mov r6, #61440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,61440)?)?;
    // VMA 0xbb9f0: orr r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb9f4: ldrb r7, [r4, #0xe]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,14)?)?;
    // VMA 0xbb9f8: and r7, r6, r7, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,12,false)?)?;
    // VMA 0xbb9fc: mov r6, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,65536)?)?;
    // VMA 0xbba00: orr r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbba04: ldrb r7, [r4, #0x12]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,18)?)?;
    // VMA 0xbba08: and r7, r6, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,16,false)?)?;
    // VMA 0xbba0c: mov r6, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,131072)?)?;
    // VMA 0xbba10: orr r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbba14: ldrb r7, [r4, #0x15]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,21)?)?;
    // VMA 0xbba18: and r7, r6, r7, lsl #17
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,17,false)?)?;
    // VMA 0xbba1c: mov r6, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,262144)?)?;
    // VMA 0xbba20: orr r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbba24: ldrb r7, [r4, #0x13]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,19)?)?;
    // VMA 0xbba28: and r7, r6, r7, lsl #18
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,18,false)?)?;
    // VMA 0xbba2c: mov r6, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,524288)?)?;
    // VMA 0xbba30: orr r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbba34: ldrb r7, [r4, #0x11]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,17)?)?;
    // VMA 0xbba38: and r7, r6, r7, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,19,false)?)?;
    // VMA 0xbba3c: mov r6, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1048576)?)?;
    // VMA 0xbba40: orr r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbba44: ldrb r7, [r4, #0x14]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,20)?)?;
    // VMA 0xbba48: and r7, r6, r7, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,20,false)?)?;
    // VMA 0xbba4c: orr r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbba50: mov r7, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16777216)?)?;
    // VMA 0xbba54: and r3, r7, r3, lsl #24
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,24,false)?)?;
    // VMA 0xbba58: orr r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbba5c: mov r3, #8388608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8388608)?)?;
    // VMA 0xbba60: and r2, r3, r2, lsl #23
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,23,false)?)?;
    // VMA 0xbba64: mov r3, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,33554432)?)?;
    // VMA 0xbba68: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbba6c: ldrb r2, [r4, #0x19]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,25)?)?;
    // VMA 0xbba70: and r2, r3, r2, lsl #25
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,25,false)?)?;
    // VMA 0xbba74: mov r3, #201326592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,201326592)?)?;
    // VMA 0xbba78: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbba7c: ldrb r2, [r4, #0xf]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,15)?)?;
    // VMA 0xbba80: and r2, r3, r2, lsl #26
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,26,false)?)?;
    // VMA 0xbba84: mov r3, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,268435456)?)?;
    // VMA 0xbba88: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbba8c: ldr r2, [r4, #0x410]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1040)?)?;
    // VMA 0xbba90: orr r1, r1, r2, lsl #29
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,29,false)?)?;
    // VMA 0xbba94: ldrb r2, [r4, #0x16]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,22)?)?;
    // VMA 0xbba98: and r2, r3, r2, lsl #28
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,28,false)?)?;
    // VMA 0xbba9c: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbbaa0: str r1, [r0, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xbbaa4: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbbaa8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xbbaac)
}
const CONTROLLER_PACKET_REG_DIAGNOSTICS:[u32;4]=[
    0x000f_a034,0x000f_a075,0x000e_2788,0x000f_9ffa,
];
