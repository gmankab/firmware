
/// Compute controller flags and queue policy-specific records.
/// Part of the ongoing codec routine; no invented function return.
fn reconcile_codec_plane_policy(c:&mut Code)->Result<(),String>{
    c.check(0xbe45c)?;
    // VMA 0xbe45c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbe460: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // VMA 0xbe464: strb r0, [r5, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,16)?)?;
    // VMA 0xbe468: str r1, [r5, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,568)?)?;
    // VMA 0xbe46c: b 0xbeae4 <entry+0x2308>  @ imm = #0x670
    c.b(C::Al,0xbeae4,false)?;
    // VMA 0xbe470: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // VMA 0xbe474: mov r4, r5
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // VMA 0xbe478: str r0, [r5, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,568)?)?;
    // VMA 0xbe47c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbe480: strb r0, [r4, #0x8]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // VMA 0xbe484: ldr r0, [r9, #0x870]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2160)?)?;
    // VMA 0xbe488: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe48c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbe490: streq r0, [r5, #0x310]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(5)?,784)?)?;
    // VMA 0xbe494: b 0xbe4c8 <entry+0x1cec>  @ imm = #0x2c
    c.b(C::Al,0xbe4c8,false)?;
    // VMA 0xbe498: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe49c: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xbe4a0: b 0xbd744 <entry+0xf68>   @ imm = #-0xd64
    c.b(C::Al,0xbd744,false)?;
    // VMA 0xbe4a4: ldr r0, [r9, #0x870]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2160)?)?;
    // VMA 0xbe4a8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe4ac: bne 0xbe4c8 <entry+0x1cec>  @ imm = #0x14
    c.b(C::Ne,0xbe4c8,false)?;
    // VMA 0xbe4b0: ldr r0, [r9, #0x864]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2148)?)?;
    // VMA 0xbe4b4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbe4b8: bne 0xbe4c4 <entry+0x1ce8>  @ imm = #0x4
    c.b(C::Ne,0xbe4c4,false)?;
    // VMA 0xbe4bc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbe4c0: str r0, [r9, #0x870]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,2160)?)?;
    // VMA 0xbe4c4: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe4c8: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe4cc: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe4d0: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // VMA 0xbe4d4: bl 0xe1e60 <entry+0x25684> @ imm = #0x23984
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe4d8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe4dc: bne 0xbe5ac <entry+0x1dd0>  @ imm = #0xc8
    c.b(C::Ne,0xbe5ac,false)?;
    // VMA 0xbe4e0: ldrb r0, [r4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbe4e4: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // VMA 0xbe4e8: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // VMA 0xbe4ec: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe4f0: bhi 0xbe588 <entry+0x1dac>  @ imm = #0x90
    c.b(C::Hi,0xbe588,false)?;
    // VMA 0xbe4f4: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe4f8: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe4fc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe500: bl 0xe1e60 <entry+0x25684> @ imm = #0x23958
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe504: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe508: bne 0xbe5ac <entry+0x1dd0>  @ imm = #0x9c
    c.b(C::Ne,0xbe5ac,false)?;
    // VMA 0xbe50c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe510: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe514: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xbe518: strb r0, [r5, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,10)?)?;
    // VMA 0xbe51c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe520: bl 0xe1e60 <entry+0x25684> @ imm = #0x23938
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe524: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe528: bne 0xbe5ac <entry+0x1dd0>  @ imm = #0x7c
    c.b(C::Ne,0xbe5ac,false)?;
    // VMA 0xbe52c: ldrb r0, [r11, #-0x20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe530: strb r0, [r5, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,11)?)?;
    // VMA 0xbe534: strb r0, [r5, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,12)?)?;
    // VMA 0xbe538: ldrb r0, [r4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbe53c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xbe540: bne 0xbe588 <entry+0x1dac>  @ imm = #0x40
    c.b(C::Ne,0xbe588,false)?;
    // VMA 0xbe544: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe548: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe54c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe550: bl 0xe1e60 <entry+0x25684> @ imm = #0x23908
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe554: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe558: bne 0xbe5ac <entry+0x1dd0>  @ imm = #0x4c
    c.b(C::Ne,0xbe5ac,false)?;
    // VMA 0xbe55c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe560: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe564: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xbe568: strb r0, [r5, #0x9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,9)?)?;
    // VMA 0xbe56c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe570: bl 0xe1e60 <entry+0x25684> @ imm = #0x238e8
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe574: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe578: bne 0xbe5ac <entry+0x1dd0>  @ imm = #0x2c
    c.b(C::Ne,0xbe5ac,false)?;
    // VMA 0xbe57c: ldrb r0, [r11, #-0x20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe580: strb r0, [r5, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,13)?)?;
    // VMA 0xbe584: strb r0, [r5, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,14)?)?;
    // VMA 0xbe588: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xbe58c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbe590: strb r8, [r5, #0xf]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(5)?,15)?)?;
    // VMA 0xbe594: strb r8, [r5, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(5)?,16)?)?;
    // VMA 0xbe598: strb r0, [r5, #0x12]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,18)?)?;
    // VMA 0xbe59c: ldr r1, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,4)?)?;
    // VMA 0xbe5a0: ldr r1, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,20)?)?;
    // VMA 0xbe5a4: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // VMA 0xbe5a8: strbeq r0, [r5, #0x18]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R0,R::new(5)?,24)?)?;
    // VMA 0xbe5ac: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xbe5b0: str r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbe5b4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe5b8: b 0xbd744 <entry+0xf68>   @ imm = #-0xe7c
    c.b(C::Al,0xbd744,false)?;
    // VMA 0xbe5bc: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xbe5c0: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // VMA 0xbe5c4: str r2, [r9, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,156)?)?;
    // VMA 0xbe5c8: beq 0xbe60c <entry+0x1e30>  @ imm = #0x3c
    c.b(C::Eq,0xbe60c,false)?;
    // VMA 0xbe5cc: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // VMA 0xbe5d0: ldr r2, [pc, #0x67c]        @ 0x2478 <entry+0x2478>
    c.literal(R::R2,0xbec54)?;
    // VMA 0xbe5d4: mov r3, #79
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,79)?)?;
    // VMA 0xbe5d8: str r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbe5dc: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // VMA 0xbe5e0: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xbe5e4: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbe5e8: str r0, [r5, #0x23c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,572)?)?;
    // VMA 0xbe5ec: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xbe5f0: ldr r1, [pc, #0x658]        @ 0x2474 <entry+0x2474>
    c.literal(R::R1,0xbec50)?;
    // VMA 0xbe5f4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe5f8: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f96c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe5fc: ldr r1, [pc, #0x634]        @ 0x245c <entry+0x245c>
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbe600: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe604: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f978
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe608: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe60c: and r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,15)?)?;
    // VMA 0xbe610: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // VMA 0xbe614: beq 0xbe6ec <entry+0x1f10>  @ imm = #0xd0
    c.b(C::Eq,0xbe6ec,false)?;
    // VMA 0xbe618: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // VMA 0xbe61c: beq 0xbe71c <entry+0x1f40>  @ imm = #0xf8
    c.b(C::Eq,0xbe71c,false)?;
    // VMA 0xbe620: cmp r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,10)?)?;
    // VMA 0xbe624: bne 0xbe734 <entry+0x1f58>  @ imm = #0x108
    c.b(C::Ne,0xbe734,false)?;
    // VMA 0xbe628: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbe62c: str r0, [r9, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,160)?)?;
    // VMA 0xbe630: b 0xbe76c <entry+0x1f90>  @ imm = #0x134
    c.b(C::Al,0xbe76c,false)?;
    // VMA 0xbe634: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe638: ldr r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbe63c: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // VMA 0xbe640: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbe644: beq 0xbe670 <entry+0x1e94>  @ imm = #0x24
    c.b(C::Eq,0xbe670,false)?;
    // VMA 0xbe648: ldr r1, [pc, #0x5f4]        @ 0x2468 <entry+0x2468>
    c.literal(R::R1,0xbec44)?;
    // VMA 0xbe64c: ldr r2, [pc, #0x5f4]        @ 0x246c <entry+0x246c>
    c.literal(R::R2,0xbec48)?;
    // VMA 0xbe650: mov r3, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10)?)?;
    // VMA 0xbe654: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbe658: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe65c: orr r3, r3, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // VMA 0xbe660: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f9d4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe664: ldr r1, [pc, #0x5cc]        @ 0x245c <entry+0x245c>
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbe668: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe66c: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f9e0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe670: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xbe674: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe678: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xbe67c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe680: b 0xbea8c <entry+0x22b0>  @ imm = #0x404
    c.b(C::Al,0xbea8c,false)?;
    // VMA 0xbe684: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // VMA 0xbe688: b 0xbe6c8 <entry+0x1eec>  @ imm = #0x38
    c.b(C::Al,0xbe6c8,false)?;
    // VMA 0xbe68c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe690: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbe694: bne 0xbe6b0 <entry+0x1ed4>  @ imm = #0x14
    c.b(C::Ne,0xbe6b0,false)?;
    // VMA 0xbe698: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbe69c: beq 0xbe684 <entry+0x1ea8>  @ imm = #-0x20
    c.b(C::Eq,0xbe684,false)?;
    // VMA 0xbe6a0: ldr r0, [pc, #0x5c0]        @ 0x248c <entry+0x248c>
    c.literal(R::R0,0xbec68)?;
    // VMA 0xbe6a4: add r0, r0, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbe6a8: ldrb r0, [r0, #-0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,-1)?)?;
    // VMA 0xbe6ac: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbe6b0: ldr r1, [pc, #0x5b0]        @ 0x248c <entry+0x248c>
    c.literal(R::R1,0xbec68)?;
    // VMA 0xbe6b4: ldrb r1, [r1, r10]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbe6b8: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // VMA 0xbe6bc: cmp r10, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,64)?)?;
    // VMA 0xbe6c0: strb r0, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbe6c4: bhs 0xbe918 <entry+0x213c>  @ imm = #0x24c
    c.b(C::Cs,0xbe918,false)?;
    // VMA 0xbe6c8: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbe6cc: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbe6d0: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe6d4: bl 0xe1e60 <entry+0x25684> @ imm = #0x23784
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe6d8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe6dc: beq 0xbe68c <entry+0x1eb0>  @ imm = #-0x58
    c.b(C::Eq,0xbe68c,false)?;
    // VMA 0xbe6e0: b 0xbea00 <entry+0x2224>  @ imm = #0x318
    c.b(C::Al,0xbea00,false)?;
    // VMA 0xbe6e4: andeq r10, pc, r7, asr r2
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(10)?,R::PC,R::new(7)?,Shift::Asr,R::R2,false)?)?;
    // VMA 0xbe6e8: andeq r2, lr, r8, lsl #15
    c.emit(arm32::data_register(C::Eq,Op::And,R::R2,R::LR,R::new(8)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbe6ec: ldr r1, [pc, #0x564]        @ 0x247c <entry+0x247c>
    c.literal(R::R1,0xbec58)?;
    // VMA 0xbe6f0: ldr r2, [pc, #0x55c]        @ 0x2478 <entry+0x2478>
    c.literal(R::R2,0xbec54)?;
    // VMA 0xbe6f4: mov r3, #93
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,93)?)?;
    // VMA 0xbe6f8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe6fc: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xbe700: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8fa74
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe704: ldr r1, [pc, #0x52c]        @ 0x245c <entry+0x245c>
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbe708: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe70c: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8fa80
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe710: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbe714: str r0, [r9, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,160)?)?;
    // VMA 0xbe718: b 0xbe76c <entry+0x1f90>  @ imm = #0x4c
    c.b(C::Al,0xbe76c,false)?;
    // VMA 0xbe71c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbe720: str r0, [r9, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,160)?)?;
    // VMA 0xbe724: b 0xbe76c <entry+0x1f90>  @ imm = #0x40
    c.b(C::Al,0xbe76c,false)?;
    // VMA 0xbe728: andeq r10, pc, sp, asr #7
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(10)?,R::PC,R::SP,Shift::Asr,7,false)?)?;
    // VMA 0xbe72c: andeq r10, pc, r4, ror r2
    c.emit(arm32::data_register_shifted_by_register(C::Eq,Op::And,R::new(10)?,R::PC,R::new(4)?,Shift::Ror,R::R2,false)?)?;
    // VMA 0xbe730: andeq r10, pc, r10, ror #5
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(10)?,R::PC,R::new(10)?,Shift::Ror,5,false)?)?;
    // VMA 0xbe734: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbe738: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // VMA 0xbe73c: str r1, [r9, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,160)?)?;
    // VMA 0xbe740: beq 0xbe76c <entry+0x1f90>  @ imm = #0x24
    c.b(C::Eq,0xbe76c,false)?;
    // VMA 0xbe744: ldr r1, [pc, #0x510]        @ 0x2480 <entry+0x2480>
    c.literal(R::R1,0xbec5c)?;
    // VMA 0xbe748: ldr r2, [pc, #0x504]        @ 0x2478 <entry+0x2478>
    c.literal(R::R2,0xbec54)?;
    // VMA 0xbe74c: mov r3, #105
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,105)?)?;
    // VMA 0xbe750: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbe754: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe758: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xbe75c: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8fad0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe760: ldr r1, [pc, #0x4d0]        @ 0x245c <entry+0x245c>
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbe764: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe768: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8fadc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe76c: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe770: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe774: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe778: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xbe77c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe780: bl 0xe1e60 <entry+0x25684> @ imm = #0x236d8
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe784: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe788: bne 0xbe8c0 <entry+0x20e4>  @ imm = #0x130
    c.b(C::Ne,0xbe8c0,false)?;
    // VMA 0xbe78c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe790: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe794: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbe798: str r0, [r9, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,168)?)?;
    // VMA 0xbe79c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe7a0: bl 0xe1e60 <entry+0x25684> @ imm = #0x236b8
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe7a4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe7a8: bne 0xbe8b8 <entry+0x20dc>  @ imm = #0x108
    c.b(C::Ne,0xbe8b8,false)?;
    // VMA 0xbe7ac: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe7b0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe7b4: beq 0xbe7f4 <entry+0x2018>  @ imm = #0x38
    c.b(C::Eq,0xbe7f4,false)?;
    // VMA 0xbe7b8: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // VMA 0xbe7bc: ldr r2, [pc, #0x490]        @ 0x2478 <entry+0x2478>
    c.literal(R::R2,0xbec54)?;
    // VMA 0xbe7c0: mov r3, #131
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,131)?)?;
    // VMA 0xbe7c4: str r1, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,188)?)?;
    // VMA 0xbe7c8: mov r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // VMA 0xbe7cc: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xbe7d0: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xbe7d4: str r1, [r5, #0x23c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,572)?)?;
    // VMA 0xbe7d8: ldr r1, [pc, #0x480]        @ 0x2484 <entry+0x2484>
    c.literal(R::R1,0xbec60)?;
    c.check(0xbe7dc)
}

/// Four-way controller variant switch. The original code reads PC from
/// this table; these are handler addresses, not A32 instructions.
const CODEC_PLANE_POLICY_TARGETS:[u32;4]=[
    0x000b_e45c,0x000b_eac0,0x000b_eacc,0x000b_ead8,
];

/// The frame-controller reconciliation continuation extracts status words,
/// bounds queue entries and maps codec format bits into descriptor flags.
/// Two inline data words are callback/diagnostic pointers, not executable
/// instructions (a naive A32 decoder misleadingly prints ANDEQ for each).
const STREAM_PENDING_RECORD_MESSAGE:u32=0x000f_a36c;
const STREAM_ALTERNATE_RECORD_MESSAGE:u32=0x000f_a5a8;
