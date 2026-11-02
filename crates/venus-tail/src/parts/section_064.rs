
/// Scramble/normalize the 16 controller parameter words, then invoke a
/// small platform command submission helper. Both routines have their
/// own saved-register prologues and real POP/PC returns.
fn normalize_controller_command_words(c:&mut Code)->Result<(),String>{
    c.check(0xbd560)?;
    // VMA 0xbd560: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbd564: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xbd568: sub sp, sp, #64
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,64)?)?;
    // VMA 0xbd56c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbd570: mov r2, sp
    c.emit(arm32::mov_register(C::Al,R::R2,R::SP))?;
    // VMA 0xbd574: mov r12, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16711680)?)?;
    // VMA 0xbd578: mov r9, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,65280)?)?;
    // VMA 0xbd57c: mov lr, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,4278190080)?)?;
    // VMA 0xbd580: mov r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,255)?)?;
    // VMA 0xbd584: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xbd588: ldm r5, {r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xbd58c: add r1, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2)?)?;
    // VMA 0xbd590: cmp r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // VMA 0xbd594: and r3, r9, r6, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(9)?,R::new(6)?,Shift::Lsr,8,false)?)?;
    // VMA 0xbd598: and r4, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(6)?,255)?)?;
    // VMA 0xbd59c: and r10, r12, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(10)?,R::R12,R::new(7)?,Shift::Lsl,16,false)?)?;
    // VMA 0xbd5a0: orr r3, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd5a4: and r4, lr, r7, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::LR,R::new(7)?,Shift::Lsl,8,false)?)?;
    // VMA 0xbd5a8: orr r3, r3, r10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd5ac: orr r3, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd5b0: and r4, r8, r6, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(8)?,R::new(6)?,Shift::Lsr,8,false)?)?;
    // VMA 0xbd5b4: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // VMA 0xbd5b8: and r3, r9, r6, lsr #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(9)?,R::new(6)?,Shift::Lsr,16,false)?)?;
    // VMA 0xbd5bc: orr r3, r4, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd5c0: ldr r4, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,4)?)?;
    // VMA 0xbd5c4: add r5, r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,8)?)?;
    // VMA 0xbd5c8: and r6, r4, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(4)?,4278190080)?)?;
    // VMA 0xbd5cc: and r4, r12, r4, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R12,R::new(4)?,Shift::Lsl,8,false)?)?;
    // VMA 0xbd5d0: orr r3, r3, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd5d4: orr r3, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd5d8: str r3, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,4)?)?;
    // VMA 0xbd5dc: add r2, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,8)?)?;
    // VMA 0xbd5e0: blo 0xbd588 <entry+0xdac>   @ imm = #-0x60
    c.b(C::Cc,0xbd588,false)?;
    // VMA 0xbd5e4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbd5e8: mov r2, sp
    c.emit(arm32::mov_register(C::Al,R::R2,R::SP))?;
    // VMA 0xbd5ec: ldr r3, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::R2,4)?)?;
    // VMA 0xbd5f0: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xbd5f4: cmp r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // VMA 0xbd5f8: str r3, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // VMA 0xbd5fc: bne 0xbd5ec <entry+0xe10>   @ imm = #-0x18
    c.b(C::Ne,0xbd5ec,false)?;
    // VMA 0xbd600: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbd604: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xbd608: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbd60c: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xbd610: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // VMA 0xbd614: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xbd618: mov r3, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,13)?)?;
    // VMA 0xbd61c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbd620: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xbd624: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // VMA 0xbd628: ldr r0, [r0, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,128)?)?;
    // VMA 0xbd62c: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // VMA 0xbd630: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // VMA 0xbd634: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // VMA 0xbd638: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xbd63c: bl 0x2a990 <entry+0xfffffffffff6e1b4> @ imm = #-0x92cb4
    c.b(C::Al,0x2a990,true)?;
    // VMA 0xbd640: mov sp, r11
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // VMA 0xbd644: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xbd648)
}

// Seven-way switch table addressed by the original computed LDR PC.
// These are handler addresses, NOT decodable ARM32 instructions.
const CODEC_RECOVERY_DISPATCH_TARGETS:[u32;7]=[
    0x000b_d86c,0x000b_da9c,0x000b_d97c,0x000b_dc4c,
    0x000b_d97c,0x000b_dd24,0x000b_dd74,
];

/// Prepare codec buffer segments, validate queue availability and preserve frame metadata.
/// This remains part of the same long codec-buffer function.
fn begin_codec_buffer_reconciliation(c:&mut Code)->Result<(),String>{
    c.check(0xbd648)?;
    // VMA 0xbd648: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbd64c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xbd650: sub sp, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,44)?)?;
    // VMA 0xbd654: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // VMA 0xbd658: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd65c: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // VMA 0xbd660: ldr r6, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,4)?)?;
    // VMA 0xbd664: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // VMA 0xbd668: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // VMA 0xbd66c: ldr r7, [r5, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,456)?)?;
    // VMA 0xbd670: ldr r2, [r5, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,468)?)?;
    // VMA 0xbd674: ldr r3, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,4)?)?;
    // VMA 0xbd678: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xbd67c: ldr r1, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,44)?)?;
    // VMA 0xbd680: sub r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbd684: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xbd688: bl 0xe1ec4 <entry+0x256e8> @ imm = #0x24834
    c.b(C::Al,0xe1ec4,true)?;
    // VMA 0xbd68c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbd690: blt 0xbd6e0 <entry+0xf04>   @ imm = #0x48
    c.b(C::Lt,0xbd6e0,false)?;
    // VMA 0xbd694: ldr r1, [r5, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,468)?)?;
    // VMA 0xbd698: add r7, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd69c: sub r10, r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(10)?,R::new(7)?,3)?)?;
    // VMA 0xbd6a0: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbd6a4: str r10, [r5, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,464)?)?;
    // VMA 0xbd6a8: blt 0xbd6e8 <entry+0xf0c>   @ imm = #0x38
    c.b(C::Lt,0xbd6e8,false)?;
    // VMA 0xbd6ac: ldr r0, [r5, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,456)?)?;
    // VMA 0xbd6b0: cmp r0, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd6b4: bls 0xbd6e8 <entry+0xf0c>   @ imm = #0x2c
    c.b(C::Ls,0xbd6e8,false)?;
    // VMA 0xbd6b8: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbd6bc: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // VMA 0xbd6c0: bne 0xbd7c0 <entry+0xfe4>   @ imm = #0xf8
    c.b(C::Ne,0xbd7c0,false)?;
    // VMA 0xbd6c4: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xbd6c8: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xbd6cc: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbd6d0: str r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbd6d4: lsl r0, r10, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(10)?,Shift::Lsl,3)?)?;
    // VMA 0xbd6d8: str r0, [r5, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,996)?)?;
    // VMA 0xbd6dc: b 0xbd740 <entry+0xf64>   @ imm = #0x5c
    c.b(C::Al,0xbd740,false)?;
    // VMA 0xbd6e0: sub r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,3)?)?;
    // VMA 0xbd6e4: str r0, [r5, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,464)?)?;
    // VMA 0xbd6e8: ldr r1, [pc, #0xff4]        @ 0x1f08 <entry+0x1f08>
    c.literal(R::R1,0xbe6e4)?;
    // VMA 0xbd6ec: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbd6f0: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8ea64
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd6f4: ldr r1, [pc, #0xfec]        @ 0x1f0c <entry+0x1f0c>
    c.literal(R::R1,0xbe6e8)?;
    // VMA 0xbd6f8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbd6fc: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8ea70
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd700: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbd704: mvn r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,31)?)?;
    // VMA 0xbd708: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbd70c: mov r8, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,13)?)?;
    // VMA 0xbd710: str r0, [r9, #0x854]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,2132)?)?;
    // VMA 0xbd714: ldr r0, [r5, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,456)?)?;
    // VMA 0xbd718: ldr r2, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,240)?)?;
    // VMA 0xbd71c: lsl r3, r0, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R0,Shift::Lsl,3)?)?;
    // VMA 0xbd720: cmp r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,7)?)?;
    // VMA 0xbd724: addeq r3, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Eq,Op::Add,R::new(3)?,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // VMA 0xbd728: orr r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,2)?)?;
    // VMA 0xbd72c: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xbd730: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // VMA 0xbd734: str r3, [r5, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,996)?)?;
    // VMA 0xbd738: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd73c: str r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbd740: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbd744: mov r0, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    // VMA 0xbd748: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbd74c: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xbd750: orr r0, r0, r10, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(10)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbd754: ldr r7, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbd758: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbd75c: bne 0xbd770 <entry+0xf94>   @ imm = #0xc
    c.b(C::Ne,0xbd770,false)?;
    // VMA 0xbd760: mov r0, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2096)?)?;
    // VMA 0xbd764: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbd768: orr r0, r0, r10, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(10)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbd76c: ldr r4, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbd770: ldr r1, [r5, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,468)?)?;
    // VMA 0xbd774: add r0, r7, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd778: add r0, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7)?)?;
    // VMA 0xbd77c: add r2, r1, r0, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R0,Shift::Lsr,3,false)?)?;
    // VMA 0xbd780: str r2, [r5, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,468)?)?;
    // VMA 0xbd784: ldr r0, [r5, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,456)?)?;
    // VMA 0xbd788: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd78c: bls 0xbd7b4 <entry+0xfd8>   @ imm = #0x20
    c.b(C::Ls,0xbd7b4,false)?;
    // VMA 0xbd790: ldr r1, [pc, #0xf90]        @ 0x1f4c <entry+0x1f4c>
    c.literal(R::R1,0xbe728)?;
    // VMA 0xbd794: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbd798: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xbd79c: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8eb10
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd7a0: ldr r1, [pc, #0xf40]        @ 0x1f0c <entry+0x1f0c>
    c.literal(R::R1,0xbe6e8)?;
    // VMA 0xbd7a4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbd7a8: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8eb1c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd7ac: mov r8, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,13)?)?;
    // VMA 0xbd7b0: str r4, [r9, #0x854]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,2132)?)?;
    // VMA 0xbd7b4: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xbd7b8: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbd7bc: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xbd7c0: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbd7c4: add r2, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,36)?)?;
    // VMA 0xbd7c8: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbd7cc: bl 0xe1e60 <entry+0x25684> @ imm = #0x2468c
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbd7d0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbd7d4: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xbd7d8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd7dc: bne 0xbd740 <entry+0xf64>   @ imm = #-0xa4
    c.b(C::Ne,0xbd740,false)?;
    // VMA 0xbd7e0: ldr r0, [r5, #0x318]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,792)?)?;
    // VMA 0xbd7e4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd7e8: bne 0xbd834 <entry+0x1058>  @ imm = #0x44
    c.b(C::Ne,0xbd834,false)?;
    // VMA 0xbd7ec: ldr r0, [r5, #0x31c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,796)?)?;
    // VMA 0xbd7f0: ldr r2, [pc, #0xf34]        @ 0x1f50 <entry+0x1f50>
    c.literal(R::R2,0xbe72c)?;
    // VMA 0xbd7f4: mov r1, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,106)?)?;
    // VMA 0xbd7f8: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // VMA 0xbd7fc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbd800: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbd804: bl 0x17b0 <entry+0xfffffffffff44fd4> @ imm = #-0xbc05c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbd808: ldr r1, [r5, #0x31c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,796)?)?;
    // VMA 0xbd80c: ldr r0, [r5, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,448)?)?;
    // VMA 0xbd810: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xbd814: add r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd818: add r1, r5, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(5)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xbd81c: sub r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,3)?)?;
    // VMA 0xbd820: ldr r2, [r1, #0x318]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,792)?)?;
    // VMA 0xbd824: sub r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbd828: str r0, [r1, #0x31c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,796)?)?;
    // VMA 0xbd82c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd830: str r0, [r5, #0x318]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,792)?)?;
    // VMA 0xbd834: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xbd838: sub r1, r0, #178
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,178)?)?;
    // VMA 0xbd83c: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // VMA 0xbd840: bhi 0xbd8cc <entry+0x10f0>  @ imm = #0x84
    c.b(C::Hi,0xbd8cc,false)?;
    // VMA 0xbd844: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,2)?)?;
    // VMA 0xbd848: add r2, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,0)?)?;
    // VMA 0xbd84c: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R1,R::R2,Shift::Lsl,0)?)?;
    c.check(0xbd850)?;
    for destination in CODEC_RECOVERY_DISPATCH_TARGETS {c.emit(destination)?;}
    c.check(0xbd86c)?;
    // VMA 0xbd86c: ldr r1, [pc, #0xebc]        @ 0x1f54 <entry+0x1f54>
    c.literal(R::R1,0xbe730)?;
    // VMA 0xbd870: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbd874: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xbd878: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8ebec
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd87c: ldr r1, [pc, #0xe64]        @ 0x1f0c <entry+0x1f0c>
    c.literal(R::R1,0xbe6e8)?;
    // VMA 0xbd880: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbd884: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8ebf8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd888: ldr r0, [r5, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,448)?)?;
    // VMA 0xbd88c: ldr r1, [r5, #0x31c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,796)?)?;
    // VMA 0xbd890: add r2, r5, #804
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,804)?)?;
    // VMA 0xbd894: add r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd898: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xbd89c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xbd8a0: str r0, [r2, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,2)?)?;
    // VMA 0xbd8a4: ldr r1, [r5, #0x31c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,796)?)?;
    // VMA 0xbd8a8: ldr r0, [r5, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,732)?)?;
    // VMA 0xbd8ac: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xbd8b0: add r1, r2, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xbd8b4: str r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,8)?)?;
    // VMA 0xbd8b8: str r4, [r5, #0x318]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,792)?)?;
    // VMA 0xbd8bc: ldr r0, [r5, #0x31c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,796)?)?;
    c.check(0xbd8c0)
}

/// Check output-buffer categories and record status into controller memory.
/// This remains part of the same long codec-buffer function.
fn validate_controller_output_buffers(c:&mut Code)->Result<(),String>{
    c.check(0xbd8c0)?;
    // VMA 0xbd8c0: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xbd8c4: str r0, [r5, #0x31c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,796)?)?;
    // VMA 0xbd8c8: b 0xbde08 <entry+0x162c>  @ imm = #0x538
    c.b(C::Al,0xbde08,false)?;
    // VMA 0xbd8cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbd8d0: bne 0xbd97c <entry+0x11a0>  @ imm = #0xa4
    c.b(C::Ne,0xbd97c,false)?;
    // VMA 0xbd8d4: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbd8d8: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xbd8dc: bhs 0xbde34 <entry+0x1658>  @ imm = #0x550
    c.b(C::Cs,0xbde34,false)?;
    // VMA 0xbd8e0: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbd8e4: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbd8e8: mov r1, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10)?)?;
    // VMA 0xbd8ec: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbd8f0: bl 0xe1e60 <entry+0x25684> @ imm = #0x24568
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbd8f4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd8f8: bne 0xbe5ac <entry+0x1dd0>  @ imm = #0xcac
    c.b(C::Ne,0xbe5ac,false)?;
    // VMA 0xbd8fc: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbd900: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbd904: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xbd908: bl 0xe1e60 <entry+0x25684> @ imm = #0x24550
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbd90c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd910: bne 0xbe5ac <entry+0x1dd0>  @ imm = #0xc94
    c.b(C::Ne,0xbe5ac,false)?;
    // VMA 0xbd914: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbd918: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // VMA 0xbd91c: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // VMA 0xbd920: bhs 0xbe470 <entry+0x1c94>  @ imm = #0xb48
    c.b(C::Cs,0xbe470,false)?;
    // VMA 0xbd924: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // VMA 0xbd928: mov r4, r5
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // VMA 0xbd92c: strb r0, [r4, #0x8]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // VMA 0xbd930: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // VMA 0xbd934: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xbd938: beq 0xbe484 <entry+0x1ca8>  @ imm = #0xb44
    c.b(C::Eq,0xbe484,false)?;
    // VMA 0xbd93c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd940: beq 0xbe4a4 <entry+0x1cc8>  @ imm = #0xb5c
    c.b(C::Eq,0xbe4a4,false)?;
    // VMA 0xbd944: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbd948: bne 0xbe4c8 <entry+0x1cec>  @ imm = #0xb78
    c.b(C::Ne,0xbe4c8,false)?;
    // VMA 0xbd94c: ldr r0, [r9, #0x870]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2160)?)?;
    // VMA 0xbd950: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd954: bne 0xbd968 <entry+0x118c>  @ imm = #0xc
    c.b(C::Ne,0xbd968,false)?;
    // VMA 0xbd958: ldr r0, [r9, #0x864]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2148)?)?;
    // VMA 0xbd95c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbd960: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd964: streq r0, [r9, #0x870]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,2160)?)?;
    // VMA 0xbd968: ldrb r0, [r5, #0x2cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,716)?)?;
    // VMA 0xbd96c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd970: bne 0xbe4c4 <entry+0x1ce8>  @ imm = #0xb4c
    c.b(C::Ne,0xbe4c4,false)?;
    // VMA 0xbd974: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbd978: b 0xbe4c0 <entry+0x1ce4>  @ imm = #0xb40
    c.b(C::Al,0xbe4c0,false)?;
    // VMA 0xbd97c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // VMA 0xbd980: cmp r0, #174
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,174)?)?;
    // VMA 0xbd984: bhi 0xbddf0 <entry+0x1614>  @ imm = #0x464
    c.b(C::Hi,0xbddf0,false)?;
    // VMA 0xbd988: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbd98c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbd990: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbd994: sub r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,3)?)?;
    // VMA 0xbd998: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd99c: bhi 0xbda10 <entry+0x1234>  @ imm = #0x6c
    c.b(C::Hi,0xbda10,false)?;
    // VMA 0xbd9a0: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // VMA 0xbd9a4: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbd9a8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xbd9ac: str r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbd9b0: lsl r0, r10, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(10)?,Shift::Lsl,3)?)?;
    // VMA 0xbd9b4: str r10, [r5, #0x1cc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,460)?)?;
    // VMA 0xbd9b8: str r0, [r5, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1000)?)?;
    // VMA 0xbd9bc: mov r0, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    // VMA 0xbd9c0: ldr r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // VMA 0xbd9c4: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xbd9c8: orr r0, r0, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xbd9cc: ldr r7, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbd9d0: mov r0, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2096)?)?;
    // VMA 0xbd9d4: orr r0, r0, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xbd9d8: ldr r4, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbd9dc: ldr r1, [r5, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,468)?)?;
    // VMA 0xbd9e0: add r0, r4, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd9e4: add r0, r1, r0, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,3,false)?)?;
    // VMA 0xbd9e8: ldr r1, [pc, #0xff8]        @ 0x220c <entry+0x220c>
    c.literal(R::R1,0xbe9e8)?;
    // VMA 0xbd9ec: sub r2, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R0,4)?)?;
    // VMA 0xbd9f0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd9f4: str r0, [r5, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,476)?)?;
    // VMA 0xbd9f8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbd9fc: str r2, [r5, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,480)?)?;
    // VMA 0xbda00: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8ed74
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbda04: ldr r1, [pc, #0xcdc]        @ 0x1f0c <entry+0x1f0c>
    c.literal(R::R1,0xbe6e8)?;
    // VMA 0xbda08: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbda0c: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8ed80
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbda10: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // VMA 0xbda14: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbda18: cmp r0, #2800
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2800)?)?;
    // VMA 0xbda1c: bls 0xbda38 <entry+0x125c>  @ imm = #0x14
    c.b(C::Ls,0xbda38,false)?;
    // VMA 0xbda20: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbda24: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbda28: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xbda2c: bl 0xe1e60 <entry+0x25684> @ imm = #0x2442c
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbda30: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbda34: bne 0xbde64 <entry+0x1688>  @ imm = #0x428
    c.b(C::Ne,0xbde64,false)?;
    // VMA 0xbda38: ldr r0, [r9, #0xd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,216)?)?;
    // VMA 0xbda3c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbda40: ldreq r0, [r9, #0xdc]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(9)?,220)?)?;
    // VMA 0xbda44: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbda48: beq 0xbde4c <entry+0x1670>  @ imm = #0x3fc
    c.b(C::Eq,0xbde4c,false)?;
    // VMA 0xbda4c: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbda50: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbda54: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // VMA 0xbda58: bl 0xe1e60 <entry+0x25684> @ imm = #0x24400
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbda5c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbda60: bne 0xbde64 <entry+0x1688>  @ imm = #0x3fc
    c.b(C::Ne,0xbde64,false)?;
    // VMA 0xbda64: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbda68: ldr r2, [pc, #0xf88]        @ 0x221c <entry+0x221c>
    c.literal(R::R2,0xbe9f8)?;
    // VMA 0xbda6c: strb r0, [r5, #0x24]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,36)?)?;
    // VMA 0xbda70: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // VMA 0xbda74: ldr r1, [r5, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1004)?)?;
    // VMA 0xbda78: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xbda7c: str r1, [r5, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,1004)?)?;
    c.check(0xbda80)
}
