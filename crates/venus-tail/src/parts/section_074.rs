
/// Initialize an active controller buffer, publish the initial row sizes
/// and report completion through the original event-notification helper.
fn prime_codec_output_buffer_dimensions(c:&mut Code)->Result<(),String>{
    c.check(0xc049c)?;
    // VMA 0xc049c: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc04a0: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xc04a4: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xc04a8: ldr r2, [pc, #0x130]        @ 0x1a70 <entry+0x1a70>
    c.literal(R::R2,0xc05e0)?;
    // VMA 0xc04ac: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xc04b0: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xc04b4: mov r1, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,106)?)?;
    // VMA 0xc04b8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc04bc: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // VMA 0xc04c0: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc04c4: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbed1c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc04c8: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xc04cc: ldr r9, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(5)?,0)?)?;
    // VMA 0xc04d0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xc04d4: ldr r5, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc04d8: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xc04dc: bl 0xd041c <entry+0x118ac> @ imm = #0xff38
    c.b(C::Al,0xd041c,true)?;
    // VMA 0xc04e0: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xc04e4: ldr r0, [r9, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,56)?)?;
    // VMA 0xc04e8: add r1, r5, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,1004)?)?;
    // VMA 0xc04ec: str r0, [r5, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,972)?)?;
    // VMA 0xc04f0: str r0, [r5, #0x910]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2320)?)?;
    // VMA 0xc04f4: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // VMA 0xc04f8: str r0, [r5, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,468)?)?;
    // VMA 0xc04fc: ldr r0, [r9, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,36)?)?;
    // VMA 0xc0500: str r0, [r5, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,472)?)?;
    // VMA 0xc0504: ldr r0, [r9, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,44)?)?;
    // VMA 0xc0508: str r0, [r5, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,480)?)?;
    // VMA 0xc050c: ldr r0, [r9, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,40)?)?;
    // VMA 0xc0510: str r0, [r5, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,476)?)?;
    // VMA 0xc0514: ldr r0, [r9, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,48)?)?;
    // VMA 0xc0518: str r0, [r5, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,484)?)?;
    // VMA 0xc051c: ldr r0, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,52)?)?;
    // VMA 0xc0520: str r0, [r5, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,488)?)?;
    // VMA 0xc0524: ldr r0, [r9, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xc0528: str r0, [r5, #0x3d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,976)?)?;
    // VMA 0xc052c: ldr r0, [r9, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,64)?)?;
    // VMA 0xc0530: str r0, [r5, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,980)?)?;
    // VMA 0xc0534: ldr r0, [r9, #0x13c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,316)?)?;
    // VMA 0xc0538: str r0, [r5, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,464)?)?;
    // VMA 0xc053c: add r0, r9, #508
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,508)?)?;
    // VMA 0xc0540: add r12, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R0,1024)?)?;
    // VMA 0xc0544: ldm r12, {r0, r2, r3, r4, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc0548: stm r1, {r0, r2, r3, r4, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc054c: add r7, r5, #508
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(5)?,508)?)?;
    // VMA 0xc0550: add r1, r9, #100
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,100)?)?;
    // VMA 0xc0554: ldr r0, [r9, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,68)?)?;
    // VMA 0xc0558: str r0, [r5, #0x3d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,976)?)?;
    // VMA 0xc055c: ldr r0, [r9, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,72)?)?;
    // VMA 0xc0560: str r0, [r5, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,980)?)?;
    // VMA 0xc0564: ldr r0, [r9, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,76)?)?;
    // VMA 0xc0568: str r0, [r5, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,984)?)?;
    // VMA 0xc056c: ldr r0, [r9, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,80)?)?;
    // VMA 0xc0570: str r0, [r5, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,988)?)?;
    // VMA 0xc0574: ldr r0, [r9, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,84)?)?;
    // VMA 0xc0578: str r0, [r5, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,992)?)?;
    // VMA 0xc057c: ldr r0, [r9, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,88)?)?;
    // VMA 0xc0580: str r0, [r5, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,996)?)?;
    // VMA 0xc0584: ldr r0, [r9, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,92)?)?;
    // VMA 0xc0588: str r0, [r5, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,508)?)?;
    // VMA 0xc058c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xc0590: bl 0xc05e4 <entry+0x1a74>  @ imm = #0x4c
    c.b(C::Al,0xc05e4,true)?;
    // VMA 0xc0594: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xc0598: mov r2, sp
    c.emit(arm32::mov_register(C::Al,R::R2,R::SP))?;
    // VMA 0xc059c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xc05a0: bl 0xd7cbc <entry+0x1914c> @ imm = #0x17714
    c.b(C::Al,0xd7cbc,true)?;
    // VMA 0xc05a4: ldr r0, [r5, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,972)?)?;
    // VMA 0xc05a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc05ac: beq 0xc05d4 <entry+0x1a64>  @ imm = #0x20
    c.b(C::Eq,0xc05d4,false)?;
    // VMA 0xc05b0: ldr r0, [r9, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,96)?)?;
    // VMA 0xc05b4: add r1, r9, #204
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,204)?)?;
    // VMA 0xc05b8: str r0, [r5, #0x27c]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(5)?,636)?)?;
    // VMA 0xc05bc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc05c0: bl 0xc05e4 <entry+0x1a74>  @ imm = #0x1c
    c.b(C::Al,0xc05e4,true)?;
    // VMA 0xc05c4: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // VMA 0xc05c8: mov r2, sp
    c.emit(arm32::mov_register(C::Al,R::R2,R::SP))?;
    // VMA 0xc05cc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc05d0: bl 0xd7cbc <entry+0x1914c> @ imm = #0x176e4
    c.b(C::Al,0xd7cbc,true)?;
    // VMA 0xc05d4: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc05d8: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xc05dc: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc05e0)?;
    const INITIAL_CONTROLLER_MESSAGE:u32=0x000fa71c;
    c.emit(INITIAL_CONTROLLER_MESSAGE)?;
    c.check(0xc05e4)
}

/// Populate multiple input and optional output buffer row mappings with
/// register-block copies, update command sizes and complete queued events.
fn publish_codec_controller_buffer_windows(c:&mut Code)->Result<(),String>{
    c.check(0xc05e4)?;
    // VMA 0xc05e4: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc05e8: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xc05ec: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xc05f0: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xc05f4: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xc05f8: ldr r2, [pc, #0x378]        @ 0x1e08 <entry+0x1e08>
    c.literal(R::R2,0xc0978)?;
    // VMA 0xc05fc: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc0600: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xc0604: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xc0608: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc060c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc0610: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc0614: and r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc0618: mov r1, #27
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,27)?)?;
    // VMA 0xc061c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xc0620: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbee78
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc0624: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc0628: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xc062c: orr r1, r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16384)?)?;
    // VMA 0xc0630: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc0634: bgt 0xc0658 <entry+0x1ae8>  @ imm = #0x1c
    c.b(C::Gt,0xc0658,false)?;
    // VMA 0xc0638: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xc063c: beq 0xc073c <entry+0x1bcc>  @ imm = #0xf8
    c.b(C::Eq,0xc073c,false)?;
    // VMA 0xc0640: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xc0644: beq 0xc0744 <entry+0x1bd4>  @ imm = #0xf8
    c.b(C::Eq,0xc0744,false)?;
    // VMA 0xc0648: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // VMA 0xc064c: bne 0xc0934 <entry+0x1dc4>  @ imm = #0x2e0
    c.b(C::Ne,0xc0934,false)?;
    // VMA 0xc0650: ldr r1, [pc, #0x334]        @ 0x1e1c <entry+0x1e1c>
    c.literal(R::R1,0xc098c)?;
    // VMA 0xc0654: b 0xc0748 <entry+0x1bd8>  @ imm = #0xec
    c.b(C::Al,0xc0748,false)?;
    // VMA 0xc0658: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xc065c: orr r1, r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16384)?)?;
    // VMA 0xc0660: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc0664: beq 0xc07d0 <entry+0x1c60>  @ imm = #0x164
    c.b(C::Eq,0xc07d0,false)?;
    // VMA 0xc0668: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xc066c: orr r1, r1, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,32768)?)?;
    // VMA 0xc0670: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc0674: beq 0xc0844 <entry+0x1cd4>  @ imm = #0x1c8
    c.b(C::Eq,0xc0844,false)?;
    // VMA 0xc0678: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xc067c: orr r1, r1, #49152
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,49152)?)?;
    // VMA 0xc0680: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc0684: bne 0xc0934 <entry+0x1dc4>  @ imm = #0x2a8
    c.b(C::Ne,0xc0934,false)?;
    // VMA 0xc0688: ldr r1, [pc, #0x2f4]        @ 0x1e14 <entry+0x1e14>
    c.literal(R::R1,0xc0984)?;
    // VMA 0xc068c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0690: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91a04
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0694: ldr r1, [pc, #0x2e4]        @ 0x1e10 <entry+0x1e10>
    c.literal(R::R1,0xc0980)?;
    // VMA 0xc0698: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc069c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91a10
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc06a0: ldr r0, [r6, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,100)?)?;
    // VMA 0xc06a4: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc06a8: add r12, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(4)?,4)?)?;
    // VMA 0xc06ac: str r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,104)?)?;
    // VMA 0xc06b0: ldr r0, [r6, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,96)?)?;
    // VMA 0xc06b4: str r0, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,100)?)?;
    // VMA 0xc06b8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc06bc: str r0, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,124)?)?;
    // VMA 0xc06c0: ldm r1, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc06c4: add r1, r4, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,20)?)?;
    // VMA 0xc06c8: stm r12, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc06cc: add r12, r6, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(6)?,24)?)?;
    // VMA 0xc06d0: ldm r12, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc06d4: add r12, r6, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(6)?,48)?)?;
    // VMA 0xc06d8: stm r1, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc06dc: add r1, r4, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,36)?)?;
    // VMA 0xc06e0: ldm r12, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc06e4: add r12, r6, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(6)?,72)?)?;
    // VMA 0xc06e8: stm r1, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc06ec: add r1, r4, #52
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,52)?)?;
    // VMA 0xc06f0: ldm r12, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc06f4: stm r1, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc06f8: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // VMA 0xc06fc: ldr r1, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,20)?)?;
    // VMA 0xc0700: str r1, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,72)?)?;
    // VMA 0xc0704: str r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,68)?)?;
    // VMA 0xc0708: ldr r0, [r6, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,40)?)?;
    // VMA 0xc070c: ldr r1, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,44)?)?;
    // VMA 0xc0710: str r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,80)?)?;
    // VMA 0xc0714: str r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,76)?)?;
    // VMA 0xc0718: ldr r0, [r6, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,64)?)?;
    // VMA 0xc071c: ldr r1, [r6, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,68)?)?;
    // VMA 0xc0720: str r1, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,88)?)?;
    // VMA 0xc0724: str r0, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,84)?)?;
    // VMA 0xc0728: ldr r0, [r6, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,88)?)?;
    // VMA 0xc072c: ldr r1, [r6, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,92)?)?;
    // VMA 0xc0730: str r1, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,96)?)?;
    // VMA 0xc0734: str r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,92)?)?;
    // VMA 0xc0738: b 0xc092c <entry+0x1dbc>  @ imm = #0x1ec
    c.b(C::Al,0xc092c,false)?;
    // VMA 0xc073c: ldr r1, [pc, #0x250]        @ 0x1e24 <entry+0x1e24>
    c.literal(R::R1,0xc0994)?;
    // VMA 0xc0740: b 0xc0748 <entry+0x1bd8>  @ imm = #0x0
    c.b(C::Al,0xc0748,false)?;
    // VMA 0xc0744: ldr r1, [pc, #0x244]        @ 0x1e20 <entry+0x1e20>
    c.literal(R::R1,0xc0990)?;
    // VMA 0xc0748: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc074c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91ac0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0750: ldr r1, [pc, #0x228]        @ 0x1e10 <entry+0x1e10>
    c.literal(R::R1,0xc0980)?;
    // VMA 0xc0754: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0758: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91acc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc075c: ldr r0, [r6, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,100)?)?;
    // VMA 0xc0760: str r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,104)?)?;
    // VMA 0xc0764: ldr r0, [r6, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,96)?)?;
    // VMA 0xc0768: str r0, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,100)?)?;
    // VMA 0xc076c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc0770: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // VMA 0xc0774: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // VMA 0xc0778: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // VMA 0xc077c: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xc0780: str r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // VMA 0xc0784: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // VMA 0xc0788: str r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // VMA 0xc078c: ldr r0, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,24)?)?;
    // VMA 0xc0790: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // VMA 0xc0794: ldr r0, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,28)?)?;
    // VMA 0xc0798: str r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // VMA 0xc079c: ldr r0, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // VMA 0xc07a0: str r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // VMA 0xc07a4: ldr r0, [r6, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,36)?)?;
    // VMA 0xc07a8: str r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // VMA 0xc07ac: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // VMA 0xc07b0: str r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,68)?)?;
    // VMA 0xc07b4: ldr r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,20)?)?;
    // VMA 0xc07b8: str r0, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,72)?)?;
    // VMA 0xc07bc: ldr r0, [r6, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,40)?)?;
    // VMA 0xc07c0: str r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,76)?)?;
    // VMA 0xc07c4: ldr r0, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,44)?)?;
    // VMA 0xc07c8: str r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,80)?)?;
    // VMA 0xc07cc: b 0xc092c <entry+0x1dbc>  @ imm = #0x158
    c.b(C::Al,0xc092c,false)?;
    // VMA 0xc07d0: ldr r1, [pc, #0x1a4]        @ 0x1e0c <entry+0x1e0c>
    c.literal(R::R1,0xc097c)?;
    // VMA 0xc07d4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc07d8: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91b4c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc07dc: ldr r1, [pc, #0x19c]        @ 0x1e10 <entry+0x1e10>
    c.literal(R::R1,0xc0980)?;
    // VMA 0xc07e0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc07e4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91b58
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc07e8: ldr r0, [r6, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,100)?)?;
    // VMA 0xc07ec: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc07f0: add r12, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(4)?,4)?)?;
    // VMA 0xc07f4: str r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,104)?)?;
    // VMA 0xc07f8: ldr r0, [r6, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,96)?)?;
    // VMA 0xc07fc: str r0, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,100)?)?;
    // VMA 0xc0800: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc0804: str r0, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,124)?)?;
    // VMA 0xc0808: ldm r1, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc080c: add r1, r4, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,20)?)?;
    // VMA 0xc0810: stm r12, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc0814: add r12, r6, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(6)?,24)?)?;
    // VMA 0xc0818: ldm r12, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R12,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc081c: stm r1, {r0, r2, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc0820: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // VMA 0xc0824: ldr r1, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,20)?)?;
    // VMA 0xc0828: str r1, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,72)?)?;
    // VMA 0xc082c: str r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,68)?)?;
    // VMA 0xc0830: ldr r0, [r6, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,40)?)?;
    // VMA 0xc0834: ldr r1, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,44)?)?;
    // VMA 0xc0838: str r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,80)?)?;
    // VMA 0xc083c: str r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,76)?)?;
    // VMA 0xc0840: b 0xc092c <entry+0x1dbc>  @ imm = #0xe4
    c.b(C::Al,0xc092c,false)?;
    // VMA 0xc0844: ldr r1, [pc, #0x13c]        @ 0x1e18 <entry+0x1e18>
    c.literal(R::R1,0xc0988)?;
    // VMA 0xc0848: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc084c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91bc0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0850: ldr r1, [pc, #0x128]        @ 0x1e10 <entry+0x1e10>
    c.literal(R::R1,0xc0980)?;
    // VMA 0xc0854: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0858: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91bcc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc085c: ldr r0, [r6, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,100)?)?;
    // VMA 0xc0860: str r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,104)?)?;
    // VMA 0xc0864: ldr r0, [r6, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,96)?)?;
    // VMA 0xc0868: str r0, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,100)?)?;
    // VMA 0xc086c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc0870: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // VMA 0xc0874: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // VMA 0xc0878: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // VMA 0xc087c: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xc0880: str r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // VMA 0xc0884: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // VMA 0xc0888: str r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // VMA 0xc088c: ldr r0, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,24)?)?;
    // VMA 0xc0890: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // VMA 0xc0894: ldr r0, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,28)?)?;
    // VMA 0xc0898: str r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // VMA 0xc089c: ldr r0, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // VMA 0xc08a0: str r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // VMA 0xc08a4: ldr r0, [r6, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,36)?)?;
    // VMA 0xc08a8: str r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // VMA 0xc08ac: ldr r0, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,48)?)?;
    // VMA 0xc08b0: str r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,36)?)?;
    // VMA 0xc08b4: ldr r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    // VMA 0xc08b8: str r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // VMA 0xc08bc: ldr r0, [r6, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,56)?)?;
    // VMA 0xc08c0: str r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,44)?)?;
    // VMA 0xc08c4: ldr r0, [r6, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,60)?)?;
    // VMA 0xc08c8: str r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,48)?)?;
    // VMA 0xc08cc: ldr r0, [r6, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,72)?)?;
    // VMA 0xc08d0: str r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,52)?)?;
    // VMA 0xc08d4: ldr r0, [r6, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,76)?)?;
    // VMA 0xc08d8: str r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,56)?)?;
    // VMA 0xc08dc: ldr r0, [r6, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,80)?)?;
    // VMA 0xc08e0: str r0, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,60)?)?;
    // VMA 0xc08e4: ldr r0, [r6, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,84)?)?;
    // VMA 0xc08e8: str r0, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,64)?)?;
    // VMA 0xc08ec: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // VMA 0xc08f0: str r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,68)?)?;
    // VMA 0xc08f4: ldr r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,20)?)?;
    // VMA 0xc08f8: str r0, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,72)?)?;
    // VMA 0xc08fc: ldr r0, [r6, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,40)?)?;
    // VMA 0xc0900: str r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,76)?)?;
    // VMA 0xc0904: ldr r0, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,44)?)?;
    // VMA 0xc0908: str r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,80)?)?;
    // VMA 0xc090c: ldr r0, [r6, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,64)?)?;
    // VMA 0xc0910: str r0, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,84)?)?;
    // VMA 0xc0914: ldr r0, [r6, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,68)?)?;
    // VMA 0xc0918: str r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,88)?)?;
    // VMA 0xc091c: ldr r0, [r6, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,88)?)?;
    // VMA 0xc0920: str r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,92)?)?;
    // VMA 0xc0924: ldr r0, [r6, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,92)?)?;
    // VMA 0xc0928: str r0, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,96)?)?;
    // VMA 0xc092c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // VMA 0xc0930: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc0934: ldr r1, [pc, #0x5c]         @ 0x1e28 <entry+0x1e28>
    c.literal(R::R1,0xc0998)?;
    // VMA 0xc0938: ldr r2, [pc, #0x5c]         @ 0x1e2c <entry+0x1e2c>
    c.literal(R::R2,0xc099c)?;
    // VMA 0xc093c: mov r3, #173
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,173)?)?;
    // VMA 0xc0940: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xc0944: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xc0948: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xc094c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91cc0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0950: ldr r1, [pc, #0x28]         @ 0x1e10 <entry+0x1e10>
    c.literal(R::R1,0xc0980)?;
    // VMA 0xc0954: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xc0958: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91ccc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc095c: ldr r2, [pc, #0x14]         @ 0x1e08 <entry+0x1e08>
    c.literal(R::R2,0xc0978)?;
    // VMA 0xc0960: mov r1, #174
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,174)?)?;
    // VMA 0xc0964: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc0968: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xc096c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // VMA 0xc0970: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc0974: b 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbf1cc
    c.b(C::Al,0x17b0,false)?;
    c.check(0xc0978)?;
    const CONTROLLER_BUFFER_WINDOW_DIAGNOSTICS:[u32;10]=[
      0x000fa71c,0x000fae63,0x000e2788,0x000fae32,0x000fae08,
      0x000e6c3e,0x000fade3,0x000fadbf,0x000fae88,0x000faeae,
    ];
    for datum in CONTROLLER_BUFFER_WINDOW_DIAGNOSTICS {c.emit(datum)?;}
    c.check(0xc09a0)
}
