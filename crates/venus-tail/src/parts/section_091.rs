

/// Restore controller frame buffers and reset codec status flags before
/// returning through the original A32 register-list epilogue. An adjacent
/// helper then copies small fixed-width tables into mapped hardware scratch
/// memory with byte-precise pre/post-indexed register operations.
fn restore_controller_frame_buffer_state(c:&mut Code)->Result<(),String>{
    c.check(0xc6334)?;
    // VMA 0xc6334: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xc6338: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xc633c: ldr r6, [pc, #0xf4]
    c.literal(R::new(6)?,0xc6438)?;
    // VMA 0xc6340: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // VMA 0xc6344: mov r1, #171
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,171)?)?;
    // VMA 0xc6348: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xc634c: orr r1, r1, #2304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    // VMA 0xc6350: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xc6354: bl 0x17b0 <entry+0xfffffffffff3bad8>
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc6358: mov r1, #428
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,428)?)?;
    // VMA 0xc635c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc6360: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xc6364: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xc6368: bl 0x17b0 <entry+0xfffffffffff3bad8>
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc636c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc6370: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xc6374: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc6378: ldr r2, [r0, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,468)?)?;
    // VMA 0xc637c: str r2, [r4, #0x580]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1408)?)?;
    // VMA 0xc6380: ldr r0, [r0, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,472)?)?;
    // VMA 0xc6384: str r0, [r4, #0x584]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1412)?)?;
    // VMA 0xc6388: str r0, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,108)?)?;
    // VMA 0xc638c: str r2, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,112)?)?;
    // VMA 0xc6390: str r3, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1296)?)?;
    // VMA 0xc6394: strb r3, [r4, #0x522]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1314)?)?;
    // VMA 0xc6398: strb r3, [r4, #0x546]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1350)?)?;
    // VMA 0xc639c: strb r3, [r4, #0x547]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1351)?)?;
    // VMA 0xc63a0: strb r1, [r4, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1355)?)?;
    // VMA 0xc63a4: strb r1, [r4, #0x54c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1356)?)?;
    // VMA 0xc63a8: ldrb r6, [r4, #0x521]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,1313)?)?;
    // VMA 0xc63ac: str r6, [r4, #0x554]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1364)?)?;
    // VMA 0xc63b0: strb r1, [r4, #0x54f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1359)?)?;
    // VMA 0xc63b4: str r6, [r4, #0x558]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1368)?)?;
    // VMA 0xc63b8: strb r1, [r4, #0x550]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1360)?)?;
    // VMA 0xc63bc: str r6, [r4, #0x55c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1372)?)?;
    // VMA 0xc63c0: mov r6, #122
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,122)?)?;
    // VMA 0xc63c4: strb r1, [r4, #0x551]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1361)?)?;
    // VMA 0xc63c8: str r1, [r4, #0x560]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1376)?)?;
    // VMA 0xc63cc: str r1, [r4, #0x564]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1380)?)?;
    // VMA 0xc63d0: str r1, [r4, #0x568]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1384)?)?;
    // VMA 0xc63d4: strb r1, [r4, #0x578]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1400)?)?;
    // VMA 0xc63d8: orr r6, r6, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,1280)?)?;
    // VMA 0xc63dc: strh r2, [r4, r6]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::new(4)?,R::new(6)?)?)?;
    // VMA 0xc63e0: mov r6, #380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,380)?)?;
    // VMA 0xc63e4: orr r6, r6, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,1024)?)?;
    // VMA 0xc63e8: strh r0, [r4, r6]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(4)?,R::new(6)?)?)?;
    // VMA 0xc63ec: strb r1, [r4, #0x56f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1391)?)?;
    // VMA 0xc63f0: strb r3, [r4, #0x570]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1392)?)?;
    // VMA 0xc63f4: strb r1, [r4, #0x571]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1393)?)?;
    // VMA 0xc63f8: ldr r3, [r5, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,560)?)?;
    // VMA 0xc63fc: str r3, [r4, #0x514]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1300)?)?;
    // VMA 0xc6400: mov r3, #556
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,556)?)?;
    // VMA 0xc6404: ldrh r3, [r5, r3]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(3)?,R::new(5)?,R::new(3)?)?)?;
    // VMA 0xc6408: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc640c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc6410: bne 0xc642c <entry+0x754>
    c.b(C::Ne,0xc642c,false)?;
    // VMA 0xc6414: mov r2, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,46)?)?;
    // VMA 0xc6418: orr r2, r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,512)?)?;
    // VMA 0xc641c: ldrh r3, [r5, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(3)?,R::new(5)?,R::R2)?)?;
    // VMA 0xc6420: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc6424: cmp r0, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc6428: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc642c: strb r2, [r4, #0x54a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,1354)?)?;
    // VMA 0xc6430: strb r1, [r4, #0x549]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1353)?)?;
    // VMA 0xc6434: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc6438)?;
    const CONTROLLER_RESTORE_WARNING:u32=0x000f_b3df;
    c.emit(CONTROLLER_RESTORE_WARNING)?;
    c.check(0xc643c)?;
    // VMA 0xc643c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xc6440: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc6444: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // VMA 0xc6448: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc644c: mov r2, #3616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3616)?)?;
    // VMA 0xc6450: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xc6454: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // VMA 0xc6458: bl 0x31ec <entry+0xfffffffffff3d514>
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xc645c: ldr r12, [pc, #0x138]
    c.literal(R::R12,0xc659c)?;
    // VMA 0xc6460: mov lr, r4
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(4)?))?;
    // VMA 0xc6464: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // VMA 0xc6468: mov r10, lr
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::LR))?;
    // VMA 0xc646c: mov r3, r12
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R12))?;
    // VMA 0xc6470: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xc6474: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc6478: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xc647c: mov r1, r3
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(3)?))?;
    // VMA 0xc6480: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xc6484: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // VMA 0xc6488: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc648c: ldrb r8, [r5], #1
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::new(8)?,R::new(5)?,1)?)?;
    // VMA 0xc6490: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xc6494: cmp r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,11)?)?;
    // VMA 0xc6498: strb r8, [r7], #1
    c.emit(arm32::transfer_byte_postindexed(C::Al,false,R::new(8)?,R::new(7)?,1)?)?;
    // VMA 0xc649c: bne 0xc648c <entry+0x7b4>
    c.b(C::Ne,0xc648c,false)?;
    // VMA 0xc64a0: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xc64a4: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // VMA 0xc64a8: add r1, r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,11)?)?;
    // VMA 0xc64ac: cmp r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,3)?)?;
    // VMA 0xc64b0: bne 0xc6480 <entry+0x7a8>
    c.b(C::Ne,0xc6480,false)?;
    // VMA 0xc64b4: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xc64b8: add r10, r10, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,48)?)?;
    // VMA 0xc64bc: add r3, r3, #33
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,33)?)?;
    // VMA 0xc64c0: cmp r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,8)?)?;
    // VMA 0xc64c4: bne 0xc6474 <entry+0x79c>
    c.b(C::Ne,0xc6474,false)?;
    // VMA 0xc64c8: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // VMA 0xc64cc: add lr, lr, #384
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::LR,384)?)?;
    // VMA 0xc64d0: add r12, r12, #264
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R12,264)?)?;
    // VMA 0xc64d4: cmp r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,4)?)?;
    // VMA 0xc64d8: bne 0xc6468 <entry+0x790>
    c.b(C::Ne,0xc6468,false)?;
    // VMA 0xc64dc: ldr r0, [pc, #0xbc]
    c.literal(R::R0,0xc65a0)?;
    // VMA 0xc64e0: ldr r6, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,0)?)?;
    // VMA 0xc64e4: ldrb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,0)?)?;
    // VMA 0xc64e8: ldrb r2, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,1)?)?;
    // VMA 0xc64ec: ldrb r3, [r0, #0x2]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,2)?)?;
    // VMA 0xc64f0: ldrb r0, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,3)?)?;
    // VMA 0xc64f4: orr r7, r3, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(3)?,R::R0,Shift::Lsl,8,false)?)?;
    // VMA 0xc64f8: orr r1, r1, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,8,false)?)?;
    // VMA 0xc64fc: orr r1, r1, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc6500: mov r7, r6
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // VMA 0xc6504: strb r1, [r7, #0x606]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R1,R::new(7)?,1542)?)?;
    // VMA 0xc6508: strb r0, [r7, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,3)?)?;
    // VMA 0xc650c: ldr r0, [pc, #0x90]
    c.literal(R::R0,0xc65a4)?;
    // VMA 0xc6510: strb r3, [r7, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(7)?,2)?)?;
    // VMA 0xc6514: strb r2, [r7, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(7)?,1)?)?;
    // VMA 0xc6518: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc651c: ldrb r2, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,1)?)?;
    // VMA 0xc6520: ldrb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,0)?)?;
    // VMA 0xc6524: ldrb r0, [r0, #0x2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,2)?)?;
    // VMA 0xc6528: strb r2, [r7, #-0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(7)?,-2)?)?;
    // VMA 0xc652c: strb r1, [r7, #-0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(7)?,-3)?)?;
    // VMA 0xc6530: strb r0, [r7, #-0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,-1)?)?;
    // VMA 0xc6534: ldr r0, [pc, #0x6c]
    c.literal(R::R0,0xc65a8)?;
    // VMA 0xc6538: add r1, r7, #10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,10)?)?;
    // VMA 0xc653c: add r2, r0, #19
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,19)?)?;
    // VMA 0xc6540: ldrb r7, [r2, #-0x13]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R2,-19)?)?;
    // VMA 0xc6544: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xc6548: cmp r3, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,9)?)?;
    // VMA 0xc654c: strb r7, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc6550: ldrb r7, [r2]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R2,0)?)?;
    // VMA 0xc6554: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xc6558: strb r7, [r1, #0x20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::R1,32)?)?;
    // VMA 0xc655c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xc6560: bne 0xc6540 <entry+0x868>
    c.b(C::Ne,0xc6540,false)?;
    // VMA 0xc6564: add r0, r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,28)?)?;
    // VMA 0xc6568: add r1, r6, #1568
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,1568)?)?;
    // VMA 0xc656c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc6570: ldrb r3, [r0, #-0x13]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,-19)?)?;
    // VMA 0xc6574: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xc6578: cmp r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,10)?)?;
    // VMA 0xc657c: strb r3, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // VMA 0xc6580: ldrb r3, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc6584: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xc6588: strb r3, [r1, #0x20]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R1,32)?)?;
    // VMA 0xc658c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xc6590: bne 0xc6570 <entry+0x898>
    c.b(C::Ne,0xc6570,false)?;
    // VMA 0xc6594: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc6598: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc659c)?;
    const CONTROLLER_TABLE_BASES:[u32;4]=[
        0x000f_dc9f,0x000f_dc98,0x000f_dc9c,0x000f_e0bf,
    ];
    for base in CONTROLLER_TABLE_BASES {c.emit(base)?;}
    c.check(END)
}

pub fn fragment() -> Result<Vec<u8>,String> {
    let mut c=Code::new();
    continue_prior_codec_release(&mut c)?;
    for value in CONTROLLER_STATUS_DIAGNOSTICS { c.emit(value)?; }
    account_codec_state_and_notify(&mut c)?;
    c.check(0xa835c)?;
    c.emit(CODEC_STATUS_MESSAGE)?;
    notify_on_codec_status(&mut c)?;
    c.check(0xa838c)?;
    c.emit(CONTROLLER_ERROR_CLASS)?;
    final_codec_recovery_branch(&mut c)?;
    c.check(0xa8398)?;
    for value in CONTROLLER_MESSAGE_CONSTANTS { c.emit(value)?; }
    initialize_codec_resource_layout(&mut c)?;
    for word in RESOURCE_INIT_LOGGING {c.emit(word)?;}
    for word in RESOURCE_INITIALIZER_CALLBACKS {c.emit(word)?;}
    for word in RESOURCE_ERROR_MESSAGES {c.emit(word)?;}
    clear_multichannel_codec_slots(&mut c)?;
    clear_single_codec_slots(&mut c)?;
    select_next_codec_slot(&mut c)?;
    find_codec_record_by_tag(&mut c)?;
    for addr in MISSING_CODEC_SLOT_MESSAGES {c.emit(addr)?;}
    validate_slot_substate(&mut c)?;
    c.check(0xa9034)?;
    c.emit(CODEC_SLOT_CHECK_MESSAGE)?;
    publish_slot_allocator_pointer(&mut c)?;
    c.check(0xa90d8)?;
    c.emit(CODEC_SLOT_CHECK_MESSAGE)?;
    confirm_slot_visibility(&mut c)?;
    c.check(0xa911c)?;
    c.emit(CODEC_SLOT_CHECK_MESSAGE)?;
    read_write_slot_fields(&mut c)?;
    encode_slot_mapping_vectors(&mut c)?;
    c.emit(SLOT_MANAGER_GLOBAL)?;
    resolve_codec_slot_hardware(&mut c)?;
    begin_codec_controller_format(&mut c)?;
    write_codec_plane_blending(&mut c)?;
    handle_codec_plane_blending_failures(&mut c)?;
    for data in CODEC_BLEND_LITERALS {c.emit(data)?;}
    encode_codec_controller_output_record(&mut c)?;
    for global in CODEC_OUTPUT_GLOBALS {c.emit(global)?;}
    encode_codec_channel_command_queue(&mut c)?;
    for data in CODEC_CHANNEL_LITERALS {c.emit(data)?;}
    configure_codec_register_fifo(&mut c)?;
    for data in CODEC_FIFO_LITERALS {c.emit(data)?;}
    remap_codec_channel_slots(&mut c)?;
    for addr in SLOT_REMAP_GLOBALS {c.emit(addr)?;}
    link_codec_slot_runtime(&mut c)?;
    publish_codec_parameter_modes(&mut c)?;
    dispatch_controller_codec_events(&mut c)?;
    for word in CODEC_RELAY_DIAGNOSTICS {c.emit(word)?;}
    program_codec_register_packets(&mut c)?;
    for data in CODEC_REGISTER_PACKET_LITERALS {c.emit(data)?;}
    serialize_codec_resource_release(&mut c)?;
    begin_codec_allocation_and_scaled_dimensions(&mut c)?;
    finish_codec_allocator_and_callback_getter(&mut c)?;
    construct_codec_configuration_record(&mut c)?;
    bind_surface_state_and_reset_slots(&mut c)?;
    activate_codec_channel_buffers(&mut c)?;
    deactivate_codec_channel_buffers(&mut c)?;
    reset_active_codec_output_buffers(&mut c)?;
    program_live_codec_registers(&mut c)?;
    submit_codec_plane_control_pair(&mut c)?;
    publish_codec_buffer_registers(&mut c)?;
    prepare_codec_stream_dispatch(&mut c)?;
    create_codec_format_accessors(&mut c)?;
    assemble_codec_command_and_report(&mut c)?;
    prepare_codec_record_and_guard_callbacks(&mut c)?;
    handle_frame_queue_and_controller_interrupts(&mut c)?;
    validate_codec_plane_lengths_and_descriptor_arrays(&mut c)?;
    synchronize_frame_planes_and_dispatch_notifications(&mut c)?;
    process_codec_plane_command_queue(&mut c)?;
    finish_frame_command_queue_and_output_registers(&mut c)?;
    enqueue_codec_output_and_status(&mut c)?;
    for word in CODEC_OUTPUT_DIAGNOSTICS { c.emit(word)?; }
    read_codec_output_plane_records(&mut c)?;
    reconcile_codec_frame_queue(&mut c)?;
    for addr in CODEC_FRAME_QUEUE_DIAGNOSTICS {c.emit(addr)?;}
    collapse_buffer_response_slots(&mut c)?;
    initialize_live_codec_buffers(&mut c)?;
    for word in LIVE_CODEC_BUFFER_GLOBALS {c.emit(word)?;}
    drain_controller_frame_slots(&mut c)?;
    c.emit(RESET_RECORD_WARNING)?;
    compact_controller_frame_records(&mut c)?;
    update_remaining_codec_records(&mut c)?;
    insert_codec_frame_record(&mut c)?;
    set_codec_channel_state(&mut c)?;
    for word in CHANNEL_STATE_REPORTS {c.emit(word)?;}
    shift_codec_record_indices(&mut c)?;
    for word in CODEC_RECORD_SHIFT_DIAGNOSTICS {c.emit(word)?;}
    register_stream_record(&mut c)?;
    sum_codec_plane_sizes(&mut c)?;
    sum_codec_line_offsets(&mut c)?;
    flush_controller_stream_buffer(&mut c)?;
    for word in STREAM_FLUSH_DIAGNOSTICS {c.emit(word)?;}
    update_codec_stream_feedback(&mut c)?;
    for word in STREAM_FEEDBACK_DIAGNOSTICS {c.emit(word)?;}
    begin_controller_stream_classification(&mut c)?;
    finish_controller_stream_classification(&mut c)?;
    initialize_codec_plane_status(&mut c)?;
    create_controller_codec_descriptor(&mut c)?;
    forward_codec_context_calls(&mut c)?;
    register_controller_descriptor_and_planes(&mut c)?;
    build_controller_plane_layout_and_event_dispatch(&mut c)?;
    release_codec_context_and_reset_queue(&mut c)?;
    drain_codec_slot_contexts_and_close(&mut c)?;
    activate_codec_runtime_and_recheck_buffers(&mut c)?;
    issue_codec_stream_commands(&mut c)?;
    begin_codec_stream_record(&mut c)?;
    stop_codec_stream_record(&mut c)?;
    select_codec_stream_state(&mut c)?;
    calculate_codec_buffer_crc32(&mut c)?;
    publish_codec_active_record(&mut c)?;
    initialize_codec_output_message_queue(&mut c)?;
    serialize_active_codec_output_record(&mut c)?;
    configure_codec_input_plane_dimensions(&mut c)?;
    prepare_codec_input_records_and_feedback(&mut c)?;
    initialize_buffer_subscription(&mut c)?;
    inspect_codec_fallback_mode(&mut c)?;
    begin_frame_descriptor_dispatch(&mut c)?;
    write_frame_descriptor_command_records(&mut c)?;
    finish_frame_descriptor_dispatch(&mut c)?;
    reconcile_controller_frame_status(&mut c)?;
    clear_optional_codec_sources(&mut c)?;
    initialize_frame_registers(&mut c)?;
    attach_optional_input_source(&mut c)?;
    clone_optional_codec_source(&mut c)?;
    select_codec_stream_end_state(&mut c)?;
    for word in STREAM_END_DIAGNOSTICS {c.emit(word)?;}
    settle_codec_dma_status(&mut c)?;
    reset_codec_command_session(&mut c)?;
    for word in CODEC_RESET_GLOBALS {c.emit(word)?;}
    reconcile_codec_dma_window(&mut c)?;
    for word in DMA_WINDOW_CONTROLLER_METADATA {c.emit(word)?;}
    push_stream_record_to_controller(&mut c)?;
    for word in FRAME_CONTROLLER_RELEASE_LITERALS {c.emit(word)?;}
    synthesize_codec_auxiliary_records(&mut c)?;
    c.emit(PACKED_CODEC_RECORD_FORMAT)?;
    retire_codec_controller_slot(&mut c)?;
    for word in CODEC_SESSION_RETIRE_DIAGNOSTICS {c.emit(word)?;}
    update_external_codec_reference(&mut c)?;
    c.emit(EXTERNAL_CODEC_WARNING)?;
    program_codec_packet_registers(&mut c)?;
    for value in CONTROLLER_PACKET_REG_DIAGNOSTICS {c.emit(value)?;}
    check_codec_hardware_readback(&mut c)?;
    program_codec_status_registers(&mut c)?;
    program_codec_stream_aux_registers(&mut c)?;
    setup_stream_controller_mmio(&mut c)?;
    program_stream_base_descriptors(&mut c)?;
    initialize_codec_tuning_buffers(&mut c)?;
    write_frame_register_command_table(&mut c)?;
    finish_stream_controller_command_buffer(&mut c)?;
    for literal in STREAM_CONTROLLER_COMMAND_LITERALS {c.emit(literal)?;}
    controller_command_register_helpers(&mut c)?;
    begin_hardware_codec_profile(&mut c)?;
    write_hardware_codec_profile_plane_rows(&mut c)?;
    finish_hardware_codec_profile(&mut c)?;
    for datum in HARDWARE_CODEC_PROFILE_LITERALS {c.emit(datum)?;}
    initialize_controller_stream_record_state(&mut c)?;
    for word in CONTROLLER_RECORD_INIT_LITERALS {c.emit(word)?;}
    normalize_controller_command_words(&mut c)?;
    begin_codec_buffer_reconciliation(&mut c)?;
    validate_controller_output_buffers(&mut c)?;
    stage_controller_codec_packet_sequence(&mut c)?;
    classify_frame_controller_mode(&mut c)?;
    for target in CONTROLLER_FRAME_MODE_BRANCHES {c.emit(target)?;}
    emit_frame_mode_controller_configuration(&mut c)?;
    prepare_codec_mode_output_buffers(&mut c)?;
    select_codec_plane_policy(&mut c)?;
    for addr in CODEC_PLANE_POLICY_TARGETS {c.emit(addr)?;}
    reconcile_codec_plane_policy(&mut c)?;
    reconcile_controller_pending_output(&mut c)?;
    finalize_stream_pending_queue(&mut c)?;
    remap_pending_stream_record_bytes(&mut c)?;
    update_active_stream_plane_offsets(&mut c)?;
    begin_codec_output_frame_descriptor(&mut c)?;
    finish_codec_output_frame_descriptor(&mut c)?;
    initialize_codec_profile_allocator(&mut c)?;
    prime_codec_output_buffer_dimensions(&mut c)?;
    publish_codec_controller_buffer_windows(&mut c)?;
    initialize_hardware_frame_controller_profile(&mut c)?;
    validate_controller_profile_buffer_availability(&mut c)?;
    release_controller_profile_buffer_handles(&mut c)?;
    initialize_codec_control_channel_object(&mut c)?;
    program_codec_profile_controller_registers(&mut c)?;
    finalize_codec_output_descriptor_resources(&mut c)?;
    pulse_codec_controller_status(&mut c)?;
    set_codec_plane_mode_parameters(&mut c)?;
    configure_runtime_codec_frame_plane_tables(&mut c)?;
    publish_runtime_codec_frame_resources(&mut c)?;
    restore_controller_output_registers(&mut c)?;
    configure_codec_output_stream_mmio(&mut c)?;
    register_codec_controller_mode_policies(&mut c)?;
    decode_packed_controller_plane_parameters(&mut c)?;
    calculate_frame_scaling_and_policy(&mut c)?;
    measure_frame_plane_allocation_span(&mut c)?;
    begin_codec_controller_descriptor_builder(&mut c)?;
    continue_codec_descriptor_command_state(&mut c)?;
    initialize_controller_record_suffix(&mut c)?;
    process_live_frame_registration(&mut c)?;
    finish_controller_error_routing(&mut c)?;
    restore_controller_frame_buffer_state(&mut c)?;
    c.check(END)?;
    Ok(c.bytes)
}
