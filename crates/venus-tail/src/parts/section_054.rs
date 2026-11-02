
/// Populate every codec-controller plane address and associated DMA descriptor,
/// choose layout based on active stream status, and reconcile active slot
/// pointers. The original routine has several internal error routes which
/// converge on its return, plus a tail branch back into the same function.
/// The aligned diagnostic values after the function are not A32 opcodes.
fn reconcile_codec_dma_window(c:&mut Code)->Result<(),String>{
    c.check(0xba40c)?;
    // VMA 0xba40c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xba410: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xba414: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // VMA 0xba418: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // VMA 0xba41c: ldr r6, [r9, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,364)?)?;
    // VMA 0xba420: ldr r5, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,4)?)?;
    // VMA 0xba424: ldr r0, [r6, #0xd0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,208)?)?;
    // VMA 0xba428: ldr r4, [r6, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,260)?)?;
    // VMA 0xba42c: ldr r8, [r6, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(6)?,284)?)?;
    // VMA 0xba430: ldr r10, [r6, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(6)?,288)?)?;
    // VMA 0xba434: ldr r7, [r6, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,264)?)?;
    // VMA 0xba438: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xba43c: add r1, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba440: str r0, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,380)?)?;
    // VMA 0xba444: str r1, [r6, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,384)?)?;
    // VMA 0xba448: ldr r0, [r5, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,972)?)?;
    // VMA 0xba44c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba450: beq 0xba4b0 <entry+0x2a0>   @ imm = #0x58
    c.b(C::Eq,0xba4b0,false)?;
    // VMA 0xba454: ldr r0, [r6, #0xd4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,212)?)?;
    // VMA 0xba458: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba45c: beq 0xba478 <entry+0x268>   @ imm = #0x14
    c.b(C::Eq,0xba478,false)?;
    // VMA 0xba460: ldr r1, [r6, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,332)?)?;
    // VMA 0xba464: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xba468: add r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xba46c: str r0, [r6, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,428)?)?;
    // VMA 0xba470: str r1, [r6, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,432)?)?;
    // VMA 0xba474: b 0xba4b0 <entry+0x2a0>   @ imm = #0x34
    c.b(C::Al,0xba4b0,false)?;
    // VMA 0xba478: ldr r1, [pc, #0xb88]        @ 0xdf8 <entry+0xdf8>
    c.literal(R::R1,0xbb008)?;
    // VMA 0xba47c: ldr r2, [pc, #0xb88]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xba480: mov r3, #214
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,214)?)?;
    // VMA 0xba484: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xba488: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xba48c: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8b800
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba490: ldr r1, [pc, #0xb78]        @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xba494: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xba498: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8b80c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba49c: ldr r2, [pc, #0xb70]        @ 0xe04 <entry+0xe04>
    c.literal(R::R2,0xbb014)?;
    // VMA 0xba4a0: mov r1, #215
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,215)?)?;
    // VMA 0xba4a4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xba4a8: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xba4ac: bl 0x17b0 <entry+0xfffffffffff475a0> @ imm = #-0xb8d04
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xba4b0: ldr r0, [r5, #0x444]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1092)?)?;
    // VMA 0xba4b4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba4b8: beq 0xba528 <entry+0x318>   @ imm = #0x68
    c.b(C::Eq,0xba528,false)?;
    // VMA 0xba4bc: ldr r0, [r6, #0xd0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,208)?)?;
    // VMA 0xba4c0: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xba4c4: add r1, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba4c8: str r0, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,396)?)?;
    // VMA 0xba4cc: str r1, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,400)?)?;
    // VMA 0xba4d0: str r0, [r6, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,412)?)?;
    // VMA 0xba4d4: str r1, [r6, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,416)?)?;
    // VMA 0xba4d8: ldrb r0, [r6, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xba4dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba4e0: beq 0xba7e4 <entry+0x5d4>   @ imm = #0x2fc
    c.b(C::Eq,0xba7e4,false)?;
    // VMA 0xba4e4: ldrb r0, [r9, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,2140)?)?;
    // VMA 0xba4e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba4ec: bne 0xba7e4 <entry+0x5d4>   @ imm = #0x2f0
    c.b(C::Ne,0xba7e4,false)?;
    // VMA 0xba4f0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xba4f4: ldr r1, [pc, #0xb1c]        @ 0xe08 <entry+0xe08>
    c.literal(R::R1,0xbb018)?;
    // VMA 0xba4f8: ldr r2, [pc, #0xb0c]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xba4fc: mov r3, #668
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,668)?)?;
    // VMA 0xba500: str r0, [r6, #0x408]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1032)?)?;
    // VMA 0xba504: ldr r0, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,484)?)?;
    // VMA 0xba508: orr r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // VMA 0xba50c: str r0, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,484)?)?;
    // VMA 0xba510: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xba514: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8b888
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba518: ldr r1, [pc, #0xaf0]        @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xba51c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xba520: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8b894
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba524: b 0xba7e4 <entry+0x5d4>   @ imm = #0x2b8
    c.b(C::Al,0xba7e4,false)?;
    // VMA 0xba528: ldrb r0, [r6, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xba52c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xba530: beq 0xba580 <entry+0x370>   @ imm = #0x48
    c.b(C::Eq,0xba580,false)?;
    // VMA 0xba534: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xba538: beq 0xba5c8 <entry+0x3b8>   @ imm = #0x88
    c.b(C::Eq,0xba5c8,false)?;
    // VMA 0xba53c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba540: bne 0xba61c <entry+0x40c>   @ imm = #0xd4
    c.b(C::Ne,0xba61c,false)?;
    // VMA 0xba544: mov r0, #536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,536870912)?)?;
    // VMA 0xba548: str r0, [r6, #0x250]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,592)?)?;
    // VMA 0xba54c: ldr r0, [r9, #0x174]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,372)?)?;
    // VMA 0xba550: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba554: bne 0xba564 <entry+0x354>   @ imm = #0x8
    c.b(C::Ne,0xba564,false)?;
    // VMA 0xba558: ldr r0, [r9, #0x178]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,376)?)?;
    // VMA 0xba55c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba560: beq 0xba6b8 <entry+0x4a8>   @ imm = #0x150
    c.b(C::Eq,0xba6b8,false)?;
    // VMA 0xba564: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xba568: add r1, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba56c: str r0, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,396)?)?;
    // VMA 0xba570: str r1, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,400)?)?;
    // VMA 0xba574: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xba578: str r0, [r6, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,412)?)?;
    // VMA 0xba57c: b 0xba7e0 <entry+0x5d0>   @ imm = #0x25c
    c.b(C::Al,0xba7e0,false)?;
    // VMA 0xba580: mov r0, #134217728
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,134217728)?)?;
    // VMA 0xba584: str r0, [r6, #0x250]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,592)?)?;
    // VMA 0xba588: ldr r0, [r9, #0x174]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,372)?)?;
    // VMA 0xba58c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba590: beq 0xba658 <entry+0x448>   @ imm = #0xc0
    c.b(C::Eq,0xba658,false)?;
    // VMA 0xba594: ldr r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // VMA 0xba598: add r2, r1, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba59c: str r1, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,396)?)?;
    // VMA 0xba5a0: str r2, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,400)?)?;
    // VMA 0xba5a4: str r0, [r6, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,216)?)?;
    // VMA 0xba5a8: ldr r0, [r9, #0x874]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2164)?)?;
    // VMA 0xba5ac: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xba5b0: bne 0xba730 <entry+0x520>   @ imm = #0x178
    c.b(C::Ne,0xba730,false)?;
    // VMA 0xba5b4: ldr r0, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,484)?)?;
    // VMA 0xba5b8: ldr r1, [pc, #0xa5c]        @ 0xe0c <entry+0xe0c>
    c.literal(R::R1,0xbb01c)?;
    // VMA 0xba5bc: ldr r2, [pc, #0xa48]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xba5c0: mov r3, #568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,568)?)?;
    // VMA 0xba5c4: b 0xba714 <entry+0x504>   @ imm = #0x148
    c.b(C::Al,0xba714,false)?;
    // VMA 0xba5c8: mov r0, #134217728
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,134217728)?)?;
    // VMA 0xba5cc: str r0, [r6, #0x250]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,592)?)?;
    // VMA 0xba5d0: ldr r0, [r9, #0x178]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,376)?)?;
    // VMA 0xba5d4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba5d8: beq 0xba674 <entry+0x464>   @ imm = #0x94
    c.b(C::Eq,0xba674,false)?;
    // VMA 0xba5dc: ldr r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // VMA 0xba5e0: add r2, r1, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba5e4: str r1, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,396)?)?;
    // VMA 0xba5e8: str r2, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,400)?)?;
    // VMA 0xba5ec: str r0, [r6, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,216)?)?;
    // VMA 0xba5f0: ldr r0, [r9, #0x878]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2168)?)?;
    // VMA 0xba5f4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xba5f8: bne 0xba7d4 <entry+0x5c4>   @ imm = #0x1d4
    c.b(C::Ne,0xba7d4,false)?;
    // VMA 0xba5fc: ldr r0, [r6, #0x2c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,708)?)?;
    // VMA 0xba600: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xba604: beq 0xba7d4 <entry+0x5c4>   @ imm = #0x1c8
    c.b(C::Eq,0xba7d4,false)?;
    // VMA 0xba608: ldr r0, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,484)?)?;
    // VMA 0xba60c: ldr r1, [pc, #0xa08]        @ 0xe0c <entry+0xe0c>
    c.literal(R::R1,0xbb01c)?;
    // VMA 0xba610: ldr r2, [pc, #0x9f4]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xba614: mov r3, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,13)?)?;
    // VMA 0xba618: b 0xba7b4 <entry+0x5a4>   @ imm = #0x194
    c.b(C::Al,0xba7b4,false)?;
    // VMA 0xba61c: ldr r1, [pc, #0xa08]        @ 0xe1c <entry+0xe1c>
    c.literal(R::R1,0xbb02c)?;
    // VMA 0xba620: ldr r2, [pc, #0x9e4]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xba624: mov r3, #134
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,134)?)?;
    // VMA 0xba628: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xba62c: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xba630: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8b9a4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba634: ldr r1, [pc, #0x9d4]        @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xba638: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xba63c: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8b9b0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba640: ldr r2, [pc, #0x9cc]        @ 0xe04 <entry+0xe04>
    c.literal(R::R2,0xbb014)?;
    // VMA 0xba644: mov r1, #135
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,135)?)?;
    // VMA 0xba648: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xba64c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xba650: bl 0x17b0 <entry+0xfffffffffff475a0> @ imm = #-0xb8ea8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xba654: b 0xba7e4 <entry+0x5d4>   @ imm = #0x188
    c.b(C::Al,0xba7e4,false)?;
    // VMA 0xba658: ldr r0, [r9, #0x178]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,376)?)?;
    // VMA 0xba65c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba660: beq 0xba6d4 <entry+0x4c4>   @ imm = #0x6c
    c.b(C::Eq,0xba6d4,false)?;
    // VMA 0xba664: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xba668: str r0, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,396)?)?;
    // VMA 0xba66c: add r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba670: b 0xba6f0 <entry+0x4e0>   @ imm = #0x78
    c.b(C::Al,0xba6f0,false)?;
    // VMA 0xba674: ldr r0, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,380)?)?;
    // VMA 0xba678: str r0, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,396)?)?;
    // VMA 0xba67c: ldr r0, [r6, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,384)?)?;
    // VMA 0xba680: str r0, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,400)?)?;
    // VMA 0xba684: ldrb r0, [r9, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,2140)?)?;
    // VMA 0xba688: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba68c: beq 0xba79c <entry+0x58c>   @ imm = #0x108
    c.b(C::Eq,0xba79c,false)?;
    // VMA 0xba690: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xba694: str r0, [r6, #0x410]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1040)?)?;
    // VMA 0xba698: ldr r0, [r6, #0x2c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,708)?)?;
    // VMA 0xba69c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xba6a0: beq 0xba7d4 <entry+0x5c4>   @ imm = #0x12c
    c.b(C::Eq,0xba7d4,false)?;
    // VMA 0xba6a4: ldr r0, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,484)?)?;
    // VMA 0xba6a8: ldr r1, [pc, #0x974]        @ 0xe14 <entry+0xe14>
    c.literal(R::R1,0xbb024)?;
    // VMA 0xba6ac: ldr r2, [pc, #0x958]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xba6b0: mov r3, #35
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,35)?)?;
    // VMA 0xba6b4: b 0xba7b4 <entry+0x5a4>   @ imm = #0xf8
    c.b(C::Al,0xba7b4,false)?;
    // VMA 0xba6b8: ldr r0, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,380)?)?;
    // VMA 0xba6bc: str r0, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,396)?)?;
    // VMA 0xba6c0: ldr r0, [r6, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,384)?)?;
    // VMA 0xba6c4: str r0, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,400)?)?;
    // VMA 0xba6c8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xba6cc: str r0, [r6, #0x250]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,592)?)?;
    // VMA 0xba6d0: b 0xba574 <entry+0x364>   @ imm = #-0x164
    c.b(C::Al,0xba574,false)?;
    // VMA 0xba6d4: ldrb r0, [r9, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,2140)?)?;
    // VMA 0xba6d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba6dc: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xba6e0: streq r0, [r6, #0x408]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(6)?,1032)?)?;
    // VMA 0xba6e4: ldr r0, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,380)?)?;
    // VMA 0xba6e8: str r0, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,396)?)?;
    // VMA 0xba6ec: ldr r0, [r6, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,384)?)?;
    // VMA 0xba6f0: str r0, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,400)?)?;
    // VMA 0xba6f4: ldr r0, [r6, #0x310]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,784)?)?;
    // VMA 0xba6f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba6fc: bne 0xba730 <entry+0x520>   @ imm = #0x2c
    c.b(C::Ne,0xba730,false)?;
    // VMA 0xba700: ldr r0, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,484)?)?;
    // VMA 0xba704: ldr r1, [pc, #0x914]        @ 0xe10 <entry+0xe10>
    c.literal(R::R1,0xbb020)?;
    // VMA 0xba708: ldr r2, [pc, #0x8fc]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xba70c: mov r3, #82
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,82)?)?;
    // VMA 0xba710: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xba714: orr r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // VMA 0xba718: str r0, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,484)?)?;
    // VMA 0xba71c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xba720: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8ba94
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba724: ldr r1, [pc, #0x8e4]        @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xba728: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xba72c: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8baa0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba730: ldr r0, [r9, #0x178]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,376)?)?;
    // VMA 0xba734: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba738: beq 0xba774 <entry+0x564>   @ imm = #0x34
    c.b(C::Eq,0xba774,false)?;
    // VMA 0xba73c: ldr r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // VMA 0xba740: add r2, r1, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba744: str r1, [r6, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,412)?)?;
    // VMA 0xba748: str r2, [r6, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,416)?)?;
    // VMA 0xba74c: str r0, [r6, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,220)?)?;
    // VMA 0xba750: ldr r0, [r9, #0x878]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2168)?)?;
    // VMA 0xba754: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xba758: bne 0xba7e4 <entry+0x5d4>   @ imm = #0x84
    c.b(C::Ne,0xba7e4,false)?;
    // VMA 0xba75c: ldr r0, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,484)?)?;
    // VMA 0xba760: ldr r1, [pc, #0x8b4]        @ 0xe0c <entry+0xe0c>
    c.literal(R::R1,0xbb01c)?;
    // VMA 0xba764: ldr r2, [pc, #0x8a0]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xba768: mov r3, #94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,94)?)?;
    // VMA 0xba76c: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xba770: b 0xba508 <entry+0x2f8>   @ imm = #-0x270
    c.b(C::Al,0xba508,false)?;
    // VMA 0xba774: ldr r0, [r6, #0x310]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,784)?)?;
    // VMA 0xba778: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba77c: bne 0xba7d4 <entry+0x5c4>   @ imm = #0x50
    c.b(C::Ne,0xba7d4,false)?;
    // VMA 0xba780: ldr r0, [r9, #0x174]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,372)?)?;
    // VMA 0xba784: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba788: beq 0xbafd8 <entry+0xdc8>   @ imm = #0x848
    c.b(C::Eq,0xbafd8,false)?;
    // VMA 0xba78c: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xba790: str r0, [r6, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,412)?)?;
    // VMA 0xba794: add r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba798: b 0xbaff4 <entry+0xde4>   @ imm = #0x854
    c.b(C::Al,0xbaff4,false)?;
    // VMA 0xba79c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xba7a0: ldr r1, [pc, #0x880]        @ 0xe18 <entry+0xe18>
    c.literal(R::R1,0xbb028)?;
    // VMA 0xba7a4: ldr r2, [pc, #0x860]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xba7a8: mov r3, #27
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,27)?)?;
    // VMA 0xba7ac: str r0, [r6, #0x408]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1032)?)?;
    // VMA 0xba7b0: ldr r0, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,484)?)?;
    // VMA 0xba7b4: orr r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // VMA 0xba7b8: str r0, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,484)?)?;
    // VMA 0xba7bc: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xba7c0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xba7c4: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8bb38
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba7c8: ldr r1, [pc, #0x840]        @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xba7cc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xba7d0: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8bb44
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xba7d4: ldr r0, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,380)?)?;
    // VMA 0xba7d8: str r0, [r6, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,412)?)?;
    // VMA 0xba7dc: ldr r0, [r6, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,384)?)?;
    // VMA 0xba7e0: str r0, [r6, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,416)?)?;
    // VMA 0xba7e4: add r0, r7, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba7e8: str r0, [r6, #0x220]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,544)?)?;
    // VMA 0xba7ec: ldr r12, [r6, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(6)?,332)?)?;
    // VMA 0xba7f0: ldr r1, [r6, #0x150]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,336)?)?;
    // VMA 0xba7f4: add r1, r1, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xba7f8: str r1, [r6, #0x224]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,548)?)?;
    // VMA 0xba7fc: ldr r3, [r6, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,296)?)?;
    // VMA 0xba800: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xba804: beq 0xba8bc <entry+0x6ac>   @ imm = #0xb0
    c.b(C::Eq,0xba8bc,false)?;
    // VMA 0xba808: ldr r3, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,380)?)?;
    // VMA 0xba80c: add lr, r6, #416
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(6)?,416)?)?;
    // VMA 0xba810: str r3, [r6, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,388)?)?;
    // VMA 0xba814: add r3, r3, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba818: str r3, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,380)?)?;
    // VMA 0xba81c: add r3, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba820: str r3, [r6, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,392)?)?;
    // VMA 0xba824: add r3, r3, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba828: str r3, [r6, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,384)?)?;
    // VMA 0xba82c: ldr r3, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,396)?)?;
    // VMA 0xba830: str r3, [r6, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,404)?)?;
    // VMA 0xba834: add r3, r3, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba838: str r3, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,396)?)?;
    // VMA 0xba83c: add r3, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba840: str r3, [r6, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,408)?)?;
    // VMA 0xba844: add r3, r3, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba848: str r3, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,400)?)?;
    // VMA 0xba84c: ldr r3, [r6, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,412)?)?;
    // VMA 0xba850: add r7, r3, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(3)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba854: add r4, r7, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba858: str r7, [r6, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,412)?)?;
    // VMA 0xba85c: add r2, r4, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(4)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba860: stm lr, {r2, r3, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::LR,R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xba864: add r2, r10, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(10)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba868: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xba86c: str r0, [r6, #0x220]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,544)?)?;
    // VMA 0xba870: ldr r2, [r5, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,972)?)?;
    // VMA 0xba874: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xba878: bne 0xba8bc <entry+0x6ac>   @ imm = #0x3c
    c.b(C::Ne,0xba8bc,false)?;
    // VMA 0xba87c: ldr r2, [r6, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,368)?)?;
    // VMA 0xba880: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xba884: beq 0xba8bc <entry+0x6ac>   @ imm = #0x30
    c.b(C::Eq,0xba8bc,false)?;
    // VMA 0xba888: ldr r2, [r6, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,428)?)?;
    // VMA 0xba88c: str r2, [r6, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,436)?)?;
    // VMA 0xba890: ldr r3, [r6, #0x164]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,356)?)?;
    // VMA 0xba894: add r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xba898: str r2, [r6, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,428)?)?;
    // VMA 0xba89c: add r2, r12, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R12,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xba8a0: str r2, [r6, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,440)?)?;
    // VMA 0xba8a4: ldr r7, [r6, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,360)?)?;
    // VMA 0xba8a8: add r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xba8ac: str r2, [r6, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,432)?)?;
    // VMA 0xba8b0: add r2, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xba8b4: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xba8b8: str r1, [r6, #0x224]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,548)?)?;
    // VMA 0xba8bc: ldr r2, [r6, #0xc8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,200)?)?;
    // VMA 0xba8c0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xba8c4: bne 0xba8dc <entry+0x6cc>   @ imm = #0x10
    c.b(C::Ne,0xba8dc,false)?;
    // VMA 0xba8c8: ldr r2, [r6, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,368)?)?;
    // VMA 0xba8cc: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // VMA 0xba8d0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xba8d4: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xba8d8: lsl r1, r1, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsl,R::R2,false)?)?;
    // VMA 0xba8dc: str r0, [r6, #0x218]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,536)?)?;
    // VMA 0xba8e0: str r1, [r6, #0x21c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,540)?)?;
    // VMA 0xba8e4: ldr r0, [r6, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,564)?)?;
    // VMA 0xba8e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba8ec: bne 0xba8fc <entry+0x6ec>   @ imm = #0x8
    c.b(C::Ne,0xba8fc,false)?;
    // VMA 0xba8f0: ldr r0, [r6, #0x2c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,712)?)?;
    // VMA 0xba8f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xba8f8: beq 0xbafc4 <entry+0xdb4>   @ imm = #0x6c4
    c.b(C::Eq,0xbafc4,false)?;
    // VMA 0xba8fc: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xba900: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xba904: bl 0xb9c64 <entry+0xfffffffffffffa54> @ imm = #-0xca8
    c.b(C::Al,0xb9c64,true)?;
    // VMA 0xba908: ldr r2, [r5, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2308)?)?;
    // VMA 0xba90c: ldr r0, [pc, #0x71c]        @ 0xe20 <entry+0xe20>
    c.literal(R::R0,0xbb030)?;
    // VMA 0xba910: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xba914: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xba918: bne 0xba93c <entry+0x72c>   @ imm = #0x1c
    c.b(C::Ne,0xba93c,false)?;
    // VMA 0xba91c: ldr r2, [r5, #0xaa4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2724)?)?;
    // VMA 0xba920: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xba924: ldr r1, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,12)?)?;
    // VMA 0xba928: ldr r3, [r5, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,464)?)?;
    // VMA 0xba92c: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // VMA 0xba930: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // VMA 0xba934: bl 0x282b0 <entry+0xfffffffffff6e0a0> @ imm = #-0x9268c
    c.b(C::Al,0x282b0,true)?;
    // VMA 0xba938: ldr r2, [r5, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2308)?)?;
    // VMA 0xba93c: ldr r3, [r5, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,464)?)?;
    // VMA 0xba940: ldr r7, [r5, #0xaa4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,2724)?)?;
    // VMA 0xba944: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xba948: ldr r1, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,12)?)?;
    // VMA 0xba94c: add r4, r5, #104
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(5)?,104)?)?;
    // VMA 0xba950: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xba954: add r4, r4, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1024)?)?;
    // VMA 0xba958: movne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xba95c: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // VMA 0xba960: stmib sp, {r3, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xba964: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // VMA 0xba968: bl 0x278fc <entry+0xfffffffffff6d6ec> @ imm = #-0x93074
    c.b(C::Al,0x278fc,true)?;
    // VMA 0xba96c: ldrb r0, [r9, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,2140)?)?;
    // VMA 0xba970: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // VMA 0xba974: mov r1, #2384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2384)?)?;
    // VMA 0xba978: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xba97c: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xba980: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xba984: strb r0, [r6, #0x19]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,25)?)?;
    // VMA 0xba988: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // VMA 0xba98c: str r10, [r0, #0xa88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,2696)?)?;
    // VMA 0xba990: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xba994: ldr r3, [r6, #0x2d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,728)?)?;
    // VMA 0xba998: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xba99c: ldr r1, [r5, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2308)?)?;
    // VMA 0xba9a0: sub r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xba9a4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xba9a8: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0xe000_0000)?)?;
    // VMA 0xba9ac: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xba9b0: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // VMA 0xba9b4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xba9b8: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xba9bc: movlo r3, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xba9c0: ldr r1, [r5, #0xaa0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2720)?)?;
    // VMA 0xba9c4: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xba9c8: stm sp, {r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xba9cc: add r1, r5, #280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,280)?)?;
    // VMA 0xba9d0: add r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // VMA 0xba9d4: bl 0x3027c <entry+0xfffffffffff7606c> @ imm = #-0x8a760
    c.b(C::Al,0x3027c,true)?;
    // VMA 0xba9d8: ldr r0, [r6, #0x2d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,728)?)?;
    // VMA 0xba9dc: ldr r1, [pc, #0x650]        @ 0xe24 <entry+0xe24>
    c.literal(R::R1,0xbb034)?;
    // VMA 0xba9e0: ldr r2, [pc, #0x650]        @ 0xe28 <entry+0xe28>
    c.literal(R::R2,0xbb038)?;
    // VMA 0xba9e4: ldr r7, [pc, #0x658]        @ 0xe34 <entry+0xe34>
    c.literal(R::new(7)?,0xbb044)?;
    // VMA 0xba9e8: ldr r3, [pc, #0x658]        @ 0xe38 <entry+0xe38>
    c.literal(R::new(3)?,0xbb048)?;
    // VMA 0xba9ec: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xba9f0: moveq r2, r1
    c.emit(arm32::mov_register(C::Eq,R::R2,R::R1))?;
    // VMA 0xba9f4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xba9f8: ldr r1, [pc, #0x63c]        @ 0xe2c <entry+0xe2c>
    c.literal(R::R1,0xbb03c)?;
    // VMA 0xba9fc: ldrb r0, [r6, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xbaa00: movne r1, r2
    c.emit(arm32::mov_register(C::Ne,R::R1,R::R2))?;
    // VMA 0xbaa04: ldr r2, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,0)?)?;
    // VMA 0xbaa08: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xbaa0c: ldr r1, [pc, #0x62c]        @ 0xe30 <entry+0xe30>
    c.literal(R::R1,0xbb040)?;
    // VMA 0xbaa10: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbaa14: moveq r7, r1
    c.emit(arm32::mov_register(C::Eq,R::new(7)?,R::R1))?;
    // VMA 0xbaa18: ldr r1, [pc, #0x62c]        @ 0xe3c <entry+0xe3c>
    c.literal(R::R1,0xbb04c)?;
    // VMA 0xbaa1c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbaa20: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbaa24: movne r3, r7
    c.emit(arm32::mov_register(C::Ne,R::new(3)?,R::new(7)?))?;
    // VMA 0xbaa28: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8bd9c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbaa2c: ldr r7, [pc, #0x5dc]        @ 0xe00 <entry+0xe00>
    c.literal(R::new(7)?,0xbb010)?;
    // VMA 0xbaa30: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbaa34: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbaa38: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8bdac
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbaa3c: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbaa40: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xbaa44: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xbaa48: bl 0xbb6d8 <entry+0x14c8>  @ imm = #0xc88
    c.b(C::Al,0xbb6d8,true)?;
    // VMA 0xbaa4c: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbaa50: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xbaa54: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xbaa58: bl 0xbbcd4 <entry+0x1ac4>  @ imm = #0x1274
    c.b(C::Al,0xbbcd4,true)?;
    // VMA 0xbaa5c: ldr r0, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,52)?)?;
    // VMA 0xbaa60: bl 0x293c <entry+0xfffffffffff4872c> @ imm = #-0xb812c
    c.b(C::Al,0x293c,true)?;
    // VMA 0xbaa64: bl 0x26754 <entry+0xfffffffffff6c544> @ imm = #-0x94318
    c.b(C::Al,0x26754,true)?;
    // VMA 0xbaa68: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xbaa6c: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbaa70: ldr r4, [pc, #0x5d8]        @ 0xe40 <entry+0xe40>
    c.literal(R::new(4)?,0xbb050)?;
    // VMA 0xbaa74: ldr r1, [pc, #0x5d8]        @ 0xe44 <entry+0xe44>
    c.literal(R::R1,0xbb054)?;
    // VMA 0xbaa78: str r10, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(10)?,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbaa7c: str r10, [r5, #0xa84]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,2692)?)?;
    // VMA 0xbaa80: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbaa84: ldr r2, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,0)?)?;
    // VMA 0xbaa88: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8bdfc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbaa8c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbaa90: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbaa94: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8be08
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbaa98: ldr r0, [r5, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2308)?)?;
    // VMA 0xbaa9c: mov r10, r4
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(4)?))?;
    // VMA 0xbaaa0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbaaa4: bne 0xbab38 <entry+0x928>   @ imm = #0x8c
    c.b(C::Ne,0xbab38,false)?;
    // VMA 0xbaaa8: ldr r0, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,56)?)?;
    // VMA 0xbaaac: bl 0x293c <entry+0xfffffffffff4872c> @ imm = #-0xb8178
    c.b(C::Al,0x293c,true)?;
    // VMA 0xbaab0: bl 0x3070 <entry+0xfffffffffff48e60> @ imm = #-0xb7a48
    c.b(C::Al,0x3070,true)?;
    // VMA 0xbaab4: ldr r0, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,48)?)?;
    // VMA 0xbaab8: ldr r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,16)?)?;
    // VMA 0xbaabc: mov r7, #98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,98)?)?;
    // VMA 0xbaac0: orr r7, r7, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,512)?)?;
    // VMA 0xbaac4: mla r1, r0, r7, r4
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::R0,R::new(7)?,R::new(4)?)?)?;
    // VMA 0xbaac8: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // VMA 0xbaacc: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // VMA 0xbaad0: bl 0x266a8 <entry+0xfffffffffff6c498> @ imm = #-0x94430
    c.b(C::Al,0x266a8,true)?;
    // VMA 0xbaad4: ldr r1, [r6, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,480)?)?;
    // VMA 0xbaad8: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbaadc: bl 0xbc7a8 <entry+0x2598>  @ imm = #0x1cc4
    c.b(C::Al,0xbc7a8,true)?;
    // VMA 0xbaae0: bl 0x3074 <entry+0xfffffffffff48e64> @ imm = #-0xb7a74
    c.b(C::Al,0x3074,true)?;
    // VMA 0xbaae4: ldr r0, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,56)?)?;
    // VMA 0xbaae8: bl 0x294c <entry+0xfffffffffff4873c> @ imm = #-0xb81a4
    c.b(C::Al,0x294c,true)?;
    // VMA 0xbaaec: ldr r1, [pc, #0x564]        @ 0xe48 <entry+0xe48>
    c.literal(R::R1,0xbb058)?;
    // VMA 0xbaaf0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbaaf4: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8be68
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbaaf8: ldr r1, [pc, #0x510]        @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xbaafc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbab00: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8be74
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbab04: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbab08: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xbab0c: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xbab10: bl 0xbbabc <entry+0x18ac>  @ imm = #0xfa4
    c.b(C::Al,0xbbabc,true)?;
    // VMA 0xbab14: bl 0x3070 <entry+0xfffffffffff48e60> @ imm = #-0xb7aac
    c.b(C::Al,0x3070,true)?;
    // VMA 0xbab18: ldr r0, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,48)?)?;
    // VMA 0xbab1c: mla r1, r0, r7, r4
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::R0,R::new(7)?,R::new(4)?)?)?;
    // VMA 0xbab20: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // VMA 0xbab24: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // VMA 0xbab28: bl 0x266a8 <entry+0xfffffffffff6c498> @ imm = #-0x94488
    c.b(C::Al,0x266a8,true)?;
    // VMA 0xbab2c: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbab30: bl 0xbc7bc <entry+0x25ac>  @ imm = #0x1c84
    c.b(C::Al,0xbc7bc,true)?;
    // VMA 0xbab34: b 0xbab70 <entry+0x960>   @ imm = #0x34
    c.b(C::Al,0xbab70,false)?;
    // VMA 0xbab38: bl 0x3070 <entry+0xfffffffffff48e60> @ imm = #-0xb7ad0
    c.b(C::Al,0x3070,true)?;
    // VMA 0xbab3c: ldr r0, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,48)?)?;
    // VMA 0xbab40: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // VMA 0xbab44: mov r2, #98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,98)?)?;
    // VMA 0xbab48: orr r2, r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,512)?)?;
    // VMA 0xbab4c: mla r1, r0, r2, r3
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::R0,R::R2,R::new(3)?)?)?;
    // VMA 0xbab50: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // VMA 0xbab54: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // VMA 0xbab58: bl 0x266a8 <entry+0xfffffffffff6c498> @ imm = #-0x944b8
    c.b(C::Al,0x266a8,true)?;
    // VMA 0xbab5c: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbab60: bl 0xbc7bc <entry+0x25ac>  @ imm = #0x1c54
    c.b(C::Al,0xbc7bc,true)?;
    // VMA 0xbab64: ldr r1, [r6, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,480)?)?;
    // VMA 0xbab68: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbab6c: bl 0xbc7a8 <entry+0x2598>  @ imm = #0x1c34
    c.b(C::Al,0xbc7a8,true)?;
    // VMA 0xbab70: bl 0x3074 <entry+0xfffffffffff48e64> @ imm = #-0xb7b04
    c.b(C::Al,0x3074,true)?;
    // VMA 0xbab74: ldr r0, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,52)?)?;
    // VMA 0xbab78: bl 0x294c <entry+0xfffffffffff4873c> @ imm = #-0xb8234
    c.b(C::Al,0x294c,true)?;
    // VMA 0xbab7c: ldr r7, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,0)?)?;
    // VMA 0xbab80: ldr r4, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,4)?)?;
    // VMA 0xbab84: bl 0x26754 <entry+0xfffffffffff6c544> @ imm = #-0x94438
    c.b(C::Al,0x26754,true)?;
    // VMA 0xbab88: ldr r1, [pc, #0x4cc]        @ 0xe4c <entry+0xe4c>
    c.literal(R::R1,0xbb05c)?;
    // VMA 0xbab8c: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // VMA 0xbab90: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xbab94: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xbab98: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xbab9c: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // VMA 0xbaba0: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8bf14
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbaba4: ldr r1, [pc, #0x464]        @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xbaba8: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xbabac: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8bf20
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbabb0: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // VMA 0xbabb4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbabb8: str r1, [r0, #0xa88]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2696)?)?;
    // VMA 0xbabbc: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbabc0: ldr r1, [r10, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(10)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbabc4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xbabc8: bne 0xbabfc <entry+0x9ec>   @ imm = #0x2c
    c.b(C::Ne,0xbabfc,false)?;
    // VMA 0xbabcc: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xbabd0: ldr r1, [pc, #0x488]        @ 0xe50 <entry+0xe50>
    c.literal(R::R1,0xbb060)?;
    // VMA 0xbabd4: ldr r2, [pc, #0x430]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xbabd8: mov r3, #868
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,868)?)?;
    // VMA 0xbabdc: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbabe0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbabe4: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8bf58
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbabe8: ldr r1, [pc, #0x420]        @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xbabec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbabf0: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8bf64
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbabf4: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbabf8: b 0xbac38 <entry+0xa28>   @ imm = #0x38
    c.b(C::Al,0xbac38,false)?;
    // VMA 0xbabfc: mov r1, #3648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3648)?)?;
    // VMA 0xbac00: mov r2, #1808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1808)?)?;
    // VMA 0xbac04: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0xe000_0000)?)?;
    // VMA 0xbac08: orr r1, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbac0c: orr r2, r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    // VMA 0xbac10: ldr r7, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbac14: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xbac18: beq 0xbac28 <entry+0xa18>   @ imm = #0x8
    c.b(C::Eq,0xbac28,false)?;
    // VMA 0xbac1c: ands r7, r7, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(7)?,R::new(7)?,1,true)?)?;
    // VMA 0xbac20: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // VMA 0xbac24: beq 0xbac10 <entry+0xa00>   @ imm = #-0x1c
    c.b(C::Eq,0xbac10,false)?;
    // VMA 0xbac28: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0xe000_0000)?)?;
    // VMA 0xbac2c: ldr r1, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbac30: tst r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1)?)?;
    // VMA 0xbac34: bne 0xbac44 <entry+0xa34>   @ imm = #0x8
    c.b(C::Ne,0xbac44,false)?;
    // VMA 0xbac38: ldr r1, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,484)?)?;
    // VMA 0xbac3c: orr r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8)?)?;
    // VMA 0xbac40: str r1, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,484)?)?;
    // VMA 0xbac44: add r1, r6, #136
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,136)?)?;
    // VMA 0xbac48: ldrb r2, [r9, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,2140)?)?;
    // VMA 0xbac4c: add r3, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,1024)?)?;
    // VMA 0xbac50: ldr r1, [r5, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,972)?)?;
    // VMA 0xbac54: bl 0x2929c <entry+0xfffffffffff6f08c> @ imm = #-0x919c0
    c.b(C::Al,0x2929c,true)?;
    // VMA 0xbac58: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbac5c: bl 0x28520 <entry+0xfffffffffff6e310> @ imm = #-0x92744
    c.b(C::Al,0x28520,true)?;
    // VMA 0xbac60: str r0, [r6, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,720)?)?;
    // VMA 0xbac64: mov r2, #3616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3616)?)?;
    // VMA 0xbac68: mov r3, #548
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,548)?)?;
    // VMA 0xbac6c: ldr r1, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,8)?)?;
    // VMA 0xbac70: orr r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,3072)?)?;
    // VMA 0xbac74: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbac78: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0xe000_0000)?)?;
    // VMA 0xbac7c: ldr r1, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbac80: str r1, [r6, #0x3f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,1016)?)?;
    // VMA 0xbac84: ldr r1, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,8)?)?;
    // VMA 0xbac88: orr r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbac8c: mov r3, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,132)?)?;
    // VMA 0xbac90: ldr r1, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbac94: str r1, [r6, #0x3fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,1020)?)?;
    // VMA 0xbac98: ldr r1, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,8)?)?;
    // VMA 0xbac9c: orr r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbaca0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xbaca4: str r3, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbaca8: mov r3, #348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,348)?)?;
    // VMA 0xbacac: ldr r1, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,8)?)?;
    // VMA 0xbacb0: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    // VMA 0xbacb4: orr r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbacb8: mov r3, #1376
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1376)?)?;
    // VMA 0xbacbc: ldr r1, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbacc0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xbacc4: str r1, [r6, #0x3f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,1012)?)?;
    // VMA 0xbacc8: ldr r1, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,8)?)?;
    // VMA 0xbaccc: orr r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbacd0: ldr r1, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbacd4: str r1, [r6, #0x400]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,1024)?)?;
    // VMA 0xbacd8: ldr r1, [r6, #0xc8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,200)?)?;
    // VMA 0xbacdc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbace0: beq 0xbaeb4 <entry+0xca4>   @ imm = #0x1cc
    c.b(C::Eq,0xbaeb4,false)?;
    // VMA 0xbace4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbace8: beq 0xbad20 <entry+0xb10>   @ imm = #0x30
    c.b(C::Eq,0xbad20,false)?;
    // VMA 0xbacec: ldr r1, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,484)?)?;
    // VMA 0xbacf0: ldr r2, [pc, #0x314]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xbacf4: mov r3, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,153)?)?;
    // VMA 0xbacf8: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // VMA 0xbacfc: orr r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8)?)?;
    // VMA 0xbad00: str r1, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,484)?)?;
    // VMA 0xbad04: ldr r1, [pc, #0x35c]        @ 0xe58 <entry+0xe58>
    c.literal(R::R1,0xbb068)?;
    // VMA 0xbad08: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbad0c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbad10: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8c084
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbad14: ldr r1, [pc, #0x2f4]        @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xbad18: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbad1c: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8c090
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbad20: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbad24: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbad28: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0xe000_0000)?)?;
    // VMA 0xbad2c: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // VMA 0xbad30: orr r1, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbad34: ldr r1, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbad38: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // VMA 0xbad3c: ldrb r1, [r9, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,2140)?)?;
    // VMA 0xbad40: add r1, r1, r1, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,5,false)?)?;
    // VMA 0xbad44: add r1, r6, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xbad48: add r1, r1, #248
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,248)?)?;
    // VMA 0xbad4c: add r3, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,1024)?)?;
    // VMA 0xbad50: ldr r1, [r5, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2308)?)?;
    // VMA 0xbad54: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xbad58: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbad5c: bl 0x27ec0 <entry+0xfffffffffff6dcb0> @ imm = #-0x92ea4
    c.b(C::Al,0x27ec0,true)?;
    // VMA 0xbad60: ldr r0, [r5, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2308)?)?;
    // VMA 0xbad64: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbad68: bne 0xbad78 <entry+0xb68>   @ imm = #0x8
    c.b(C::Ne,0xbad78,false)?;
    // VMA 0xbad6c: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xbad70: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // VMA 0xbad74: bl 0x28478 <entry+0xfffffffffff6e268> @ imm = #-0x92904
    c.b(C::Al,0x28478,true)?;
    // VMA 0xbad78: ldr r0, [r6, #0xc8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,200)?)?;
    // VMA 0xbad7c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbad80: bne 0xbada0 <entry+0xb90>   @ imm = #0x18
    c.b(C::Ne,0xbada0,false)?;
    // VMA 0xbad84: ldr r0, [r5, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,972)?)?;
    // VMA 0xbad88: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbad8c: beq 0xbada0 <entry+0xb90>   @ imm = #0xc
    c.b(C::Eq,0xbada0,false)?;
    // VMA 0xbad90: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xbad94: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xbad98: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xbad9c: bl 0xbb078 <entry+0xe68>   @ imm = #0x2d4
    c.b(C::Al,0xbb078,true)?;
    // VMA 0xbada0: ldrb r4, [r6, #0x10]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,16)?)?;
    // VMA 0xbada4: cmp r4, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,3)?)?;
    // VMA 0xbada8: bhs 0xbadbc <entry+0xbac>   @ imm = #0xc
    c.b(C::Cs,0xbadbc,false)?;
    // VMA 0xbadac: ldr r1, [pc, #0x2bc]        @ 0xe60 <entry+0xe60>
    c.literal(R::R1,0xbb070)?;
    // VMA 0xbadb0: lsl r0, r4, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(4)?,Shift::Lsl,3)?)?;
    // VMA 0xbadb4: lsr r4, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(4)?,R::R0,R::R1,Shift::Lsr,R::R0,false)?)?;
    // VMA 0xbadb8: b 0xbadf0 <entry+0xbe0>   @ imm = #0x30
    c.b(C::Al,0xbadf0,false)?;
    // VMA 0xbadbc: ldr r1, [pc, #0x2a8]        @ 0xe5c <entry+0xe5c>
    c.literal(R::R1,0xbb06c)?;
    // VMA 0xbadc0: ldr r2, [pc, #0x244]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xbadc4: mov r3, #191
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,191)?)?;
    // VMA 0xbadc8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbadcc: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // VMA 0xbadd0: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8c144
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbadd4: ldr r1, [pc, #0x234]        @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xbadd8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbaddc: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8c150
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbade0: ldr r2, [pc, #0x22c]        @ 0xe04 <entry+0xe04>
    c.literal(R::R2,0xbb014)?;
    // VMA 0xbade4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbade8: mov r1, #960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,960)?)?;
    // VMA 0xbadec: bl 0x17b0 <entry+0xfffffffffff475a0> @ imm = #-0xb9644
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbadf0: ldrb r1, [r9, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,2140)?)?;
    // VMA 0xbadf4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbadf8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbadfc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbae00: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbae04: strb r1, [r9, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,2140)?)?;
    // VMA 0xbae08: ldrb r1, [r9, #0x85d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,2141)?)?;
    // VMA 0xbae0c: orr r1, r1, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbae10: strb r1, [r9, #0x85d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,2141)?)?;
    // VMA 0xbae14: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // VMA 0xbae18: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // VMA 0xbae1c: beq 0xbae2c <entry+0xc1c>   @ imm = #0x8
    c.b(C::Eq,0xbae2c,false)?;
    // VMA 0xbae20: ldr r1, [r6, #0x228]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,552)?)?;
    // VMA 0xbae24: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbae28: beq 0xbaf70 <entry+0xd60>   @ imm = #0x140
    c.b(C::Eq,0xbaf70,false)?;
    // VMA 0xbae2c: strb r0, [r9, #0x85d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,2141)?)?;
    // VMA 0xbae30: strb r0, [r9, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,2140)?)?;
    // VMA 0xbae34: str r0, [r9, #0x868]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,2152)?)?;
    // VMA 0xbae38: ldr r4, [r9, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(9)?,368)?)?;
    // VMA 0xbae3c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbae40: beq 0xbaee8 <entry+0xcd8>   @ imm = #0xa0
    c.b(C::Eq,0xbaee8,false)?;
    // VMA 0xbae44: ldr r0, [r4, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1000)?)?;
    // VMA 0xbae48: ldr r1, [r6, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,1000)?)?;
    // VMA 0xbae4c: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbae50: str r0, [r6, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1000)?)?;
    // VMA 0xbae54: ldr r0, [r4, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,996)?)?;
    // VMA 0xbae58: ldr r1, [r6, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,996)?)?;
    // VMA 0xbae5c: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbae60: str r0, [r6, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,996)?)?;
    // VMA 0xbae64: ldr r0, [r4, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1004)?)?;
    // VMA 0xbae68: ldr r1, [r6, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,1004)?)?;
    // VMA 0xbae6c: add r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbae70: str r1, [r6, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,1004)?)?;
    // VMA 0xbae74: ldr r0, [r4, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1008)?)?;
    // VMA 0xbae78: ldr r2, [r6, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,1008)?)?;
    // VMA 0xbae7c: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbae80: str r0, [r6, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1008)?)?;
    // VMA 0xbae84: ldr r2, [r4, #0x320]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,800)?)?;
    // VMA 0xbae88: ldr r3, [r6, #0x320]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,800)?)?;
    // VMA 0xbae8c: add r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbae90: str r2, [r6, #0x320]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,800)?)?;
    // VMA 0xbae94: blx 0x83c4c <entry+0xfffffffffffc9a3c> @ imm = #-0x37250
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // VMA 0xbae98: str r0, [r6, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1008)?)?;
    // VMA 0xbae9c: ldr r0, [r4, #0xcc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,204)?)?;
    // VMA 0xbaea0: bl 0x7fed0 <entry+0xfffffffffffc5cc0> @ imm = #-0x3afd8
    c.b(C::Al,0x7fed0,true)?;
    // VMA 0xbaea4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbaea8: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbaeac: streq r0, [r9, #0x170]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,368)?)?;
    // VMA 0xbaeb0: b 0xbaef8 <entry+0xce8>   @ imm = #0x40
    c.b(C::Al,0xbaef8,false)?;
    // VMA 0xbaeb4: ldrb r1, [r6, #0x2c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,44)?)?;
    // VMA 0xbaeb8: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // VMA 0xbaebc: beq 0xbace4 <entry+0xad4>   @ imm = #-0x1e0
    c.b(C::Eq,0xbace4,false)?;
    // VMA 0xbaec0: ldr r1, [r6, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,40)?)?;
    // VMA 0xbaec4: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // VMA 0xbaec8: cmp r0, r1, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsr,4,false)?)?;
    // VMA 0xbaecc: bne 0xbace4 <entry+0xad4>   @ imm = #-0x1f0
    c.b(C::Ne,0xbace4,false)?;
    // VMA 0xbaed0: ldr r1, [pc, #0x18c]        @ 0xe54 <entry+0xe54>
    c.literal(R::R1,0xbb064)?;
    // VMA 0xbaed4: ldr r2, [pc, #0x130]        @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xbaed8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbaedc: mov r3, #916
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,916)?)?;
    // VMA 0xbaee0: str r0, [r6, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,720)?)?;
    // VMA 0xbaee4: b 0xbad0c <entry+0xafc>   @ imm = #-0x1e0
    c.b(C::Al,0xbad0c,false)?;
    // VMA 0xbaee8: ldr r1, [r6, #0x3ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,1004)?)?;
    // VMA 0xbaeec: ldr r0, [r6, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1008)?)?;
    // VMA 0xbaef0: blx 0x83c4c <entry+0xfffffffffffc9a3c> @ imm = #-0x372ac
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // VMA 0xbaef4: str r0, [r6, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1008)?)?;
    // VMA 0xbaef8: ldr r0, [r5, #0x444]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1092)?)?;
    // VMA 0xbaefc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbaf00: bne 0xbaf50 <entry+0xd40>   @ imm = #0x48
    c.b(C::Ne,0xbaf50,false)?;
    // VMA 0xbaf04: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xbaf08: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xbaf0c: bl 0xba210 <entry>         @ imm = #-0xd04
    c.b(C::Al,0xba210,true)?;
    // VMA 0xbaf10: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbaf14: beq 0xbaf50 <entry+0xd40>   @ imm = #0x34
    c.b(C::Eq,0xbaf50,false)?;
    // VMA 0xbaf18: ldr r1, [pc, #0x154]        @ 0xe64 <entry+0xe64>
    c.literal(R::R1,0xbb074)?;
    // VMA 0xbaf1c: ldr r2, [pc, #0xe8]         @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xbaf20: mov r3, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,9)?)?;
    // VMA 0xbaf24: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbaf28: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // VMA 0xbaf2c: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8c2a0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbaf30: ldr r1, [pc, #0xd8]         @ 0xe00 <entry+0xe00>
    c.literal(R::R1,0xbb010)?;
    // VMA 0xbaf34: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbaf38: bl 0x2ec94 <entry+0xfffffffffff74a84> @ imm = #-0x8c2ac
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbaf3c: ldr r2, [pc, #0xd0]         @ 0xe04 <entry+0xe04>
    c.literal(R::R2,0xbb014)?;
    // VMA 0xbaf40: mov r1, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10)?)?;
    // VMA 0xbaf44: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbaf48: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xbaf4c: bl 0x17b0 <entry+0xfffffffffff475a0> @ imm = #-0xb97a4
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbaf50: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xbaf54: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xbaf58: bl 0xbb278 <entry+0x1068>  @ imm = #0x318
    c.b(C::Al,0xbb278,true)?;
    // VMA 0xbaf5c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbaf60: str r0, [r9, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,232)?)?;
    // VMA 0xbaf64: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbaf68: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbaf6c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xbaf70: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbaf74: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xbaf78: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xbaf7c: str r4, [r9, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,364)?)?;
    // VMA 0xbaf80: str r6, [r9, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,368)?)?;
    // VMA 0xbaf84: bl 0xbb278 <entry+0x1068>  @ imm = #0x2ec
    c.b(C::Al,0xbb278,true)?;
    // VMA 0xbaf88: ldr r1, [r9, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,368)?)?;
    // VMA 0xbaf8c: ldr r0, [r1, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,472)?)?;
    // VMA 0xbaf90: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbaf94: ldreq r0, [r9, #0x868]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(9)?,2152)?)?;
    // VMA 0xbaf98: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbaf9c: bne 0xbafb0 <entry+0xda0>   @ imm = #0xc
    c.b(C::Ne,0xbafb0,false)?;
    // VMA 0xbafa0: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xbafa4: bl 0xb7f94 <entry+0xffffffffffffdd84> @ imm = #-0x3018
    c.b(C::Al,0xb7f94,true)?;
    // VMA 0xbafa8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbafac: str r0, [r9, #0x868]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,2152)?)?;
    // VMA 0xbafb0: str r4, [r9, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,232)?)?;
    // VMA 0xbafb4: ldr r0, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,28)?)?;
    // VMA 0xbafb8: bl 0x292c <entry+0xfffffffffff4871c> @ imm = #-0xb8694
    c.b(C::Al,0x292c,true)?;
    // VMA 0xbafbc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbafc0: b 0xbaf68 <entry+0xd58>   @ imm = #-0x60
    c.b(C::Al,0xbaf68,false)?;
    // VMA 0xbafc4: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // VMA 0xbafc8: ldr r0, [r0, #0x50c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1292)?)?;
    // VMA 0xbafcc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbafd0: bne 0xba908 <entry+0x6f8>   @ imm = #-0x6d0
    c.b(C::Ne,0xba908,false)?;
    // VMA 0xbafd4: b 0xba8fc <entry+0x6ec>   @ imm = #-0x6e0
    c.b(C::Al,0xba8fc,false)?;
    // VMA 0xbafd8: ldrb r0, [r9, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,2140)?)?;
    // VMA 0xbafdc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbafe0: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbafe4: streq r0, [r6, #0x408]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(6)?,1032)?)?;
    // VMA 0xbafe8: ldr r0, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,380)?)?;
    // VMA 0xbafec: str r0, [r6, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,412)?)?;
    // VMA 0xbaff0: ldr r0, [r6, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,384)?)?;
    // VMA 0xbaff4: str r0, [r6, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,416)?)?;
    // VMA 0xbaff8: ldr r1, [pc, #0x20]         @ 0xe10 <entry+0xe10>
    c.literal(R::R1,0xbb020)?;
    // VMA 0xbaffc: ldr r2, [pc, #0x8]          @ 0xdfc <entry+0xdfc>
    c.literal(R::R2,0xbb00c)?;
    // VMA 0xbb000: mov r3, #632
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,632)?)?;
    // VMA 0xbb004: b 0xba504 <entry+0x2f4>   @ imm = #-0xb08
    c.b(C::Al,0xba504,false)?;
    c.check(0xbb008)
}
