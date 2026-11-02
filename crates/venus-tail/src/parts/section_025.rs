
/// Handle codec buffer FIFO/plane reassignment and update controller interrupt-state diagnostics
/// Runtime slice 0xb0748..0xb0c64, with 5 classified trailing data words.
fn handle_frame_queue_and_controller_interrupts(c:&mut Code)->Result<(),String>{
    c.check(0xb0748)?;
    // ARM 0xb0748: add r0, r9, r5, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::new(5)?,Shift::Lsl,2,false)?)?;
    // ARM 0xb074c: ldr r7, [r0, #0xb8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,184)?)?;
    // ARM 0xb0750: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // ARM 0xb0754: beq 0xb0b6c <entry+0x441c>
    c.b(C::Eq,0xb0b6c,false)?;
    // ARM 0xb0758: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // ARM 0xb075c: str r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,24)?)?;
    // ARM 0xb0760: blt 0xb080c <entry+0x40bc>
    c.b(C::Lt,0xb080c,false)?;
    // ARM 0xb0764: mov r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048)?)?;
    // ARM 0xb0768: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // ARM 0xb076c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb0770: orr lr, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb0774: str r1, [lr, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::LR,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0778: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // ARM 0xb077c: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xb0780: orr r8, r1, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::R1,R::new(4)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb0784: ldr r2, [r8, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(8)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0788: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xb078c: beq 0xb080c <entry+0x40bc>
    c.b(C::Eq,0xb080c,false)?;
    // ARM 0xb0790: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xb0794: strb r2, [r11, #-0x1d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R11,-29)?)?;
    // ARM 0xb0798: strb r2, [r11, #-0x1e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R11,-30)?)?;
    // ARM 0xb079c: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // ARM 0xb07a0: ldrb r2, [r9, #0xb4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,180)?)?;
    // ARM 0xb07a4: cmp r2, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb07a8: bne 0xb07c8 <entry+0x4078>
    c.b(C::Ne,0xb07c8,false)?;
    // ARM 0xb07ac: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // ARM 0xb07b0: mov r3, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16777216)?)?;
    // ARM 0xb07b4: str r3, [lr, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::LR,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb07b8: ldr r2, [r8, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(8)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb07bc: cmp r2, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb07c0: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // ARM 0xb07c4: bhs 0xb0b80 <entry+0x4430>
    c.b(C::Cs,0xb0b80,false)?;
    // ARM 0xb07c8: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // ARM 0xb07cc: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM 0xb07d0: mov r0, lr
    c.emit(arm32::mov_register(C::Al,R::R0,R::LR))?;
    // ARM 0xb07d4: str r3, [r0, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb07d8: mov r3, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16777216)?)?;
    // ARM 0xb07dc: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // ARM 0xb07e0: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM 0xb07e4: ldr r3, [r0, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R0,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb07e8: strb r3, [r7, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(7)?,4)?)?;
    // ARM 0xb07ec: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // ARM 0xb07f0: cmp r2, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32768)?)?;
    // ARM 0xb07f4: str r2, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,8)?)?;
    // ARM 0xb07f8: blo 0xb0948 <entry+0x41f8>
    c.b(C::Cc,0xb0948,false)?;
    // ARM 0xb07fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb0800: str r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // ARM 0xb0804: mov r0, #61
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,61)?)?;
    // ARM 0xb0808: b 0xb0708 <entry+0x3fb8>
    c.b(C::Al,0xb0708,false)?;
    // ARM 0xb080c: mov r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048)?)?;
    // ARM 0xb0810: mov r8, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,3758096384)?)?;
    // ARM 0xb0814: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // ARM 0xb0818: mov r5, r7
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(7)?))?;
    // ARM 0xb081c: orr r10, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(10)?,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb0820: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // ARM 0xb0824: str r1, [r0, r8]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R0,R::new(8)?,Shift::Lsl,0)?)?;
    // ARM 0xb0828: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // ARM 0xb082c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb0830: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb0834: orr r4, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb0838: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xb083c: ldr r1, [r0, r8]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::new(8)?,Shift::Lsl,0)?)?;
    // ARM 0xb0840: strb r1, [r5, #0xc]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R1,R::new(5)?,12)?)?;
    // ARM 0xb0844: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // ARM 0xb0848: add r1, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,1)?)?;
    // ARM 0xb084c: strb r0, [r5, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,1)?)?;
    // ARM 0xb0850: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xb0854: ldrb r2, [r9, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,123)?)?;
    // ARM 0xb0858: bl 0xb0c64 <entry+0x4514>
    c.b(C::Al,0xb0c64,true)?;
    // ARM 0xb085c: ldrb r3, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // ARM 0xb0860: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xb0864: beq 0xb08c4 <entry+0x4174>
    c.b(C::Eq,0xb08c4,false)?;
    // ARM 0xb0868: add r1, r7, #84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,84)?)?;
    // ARM 0xb086c: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // ARM 0xb0870: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // ARM 0xb0874: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb0878: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xb087c: mov r9, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,16777216)?)?;
    // ARM 0xb0880: mov r3, r10
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // ARM 0xb0884: str r9, [r3, r8]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(9)?,R::new(3)?,R::new(8)?,Shift::Lsl,0)?)?;
    // ARM 0xb0888: str lr, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // ARM 0xb088c: ldr r3, [r4, r8]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(4)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0890: cmp r12, r3, lsr #15
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(3)?,Shift::Lsr,15,false)?)?;
    // ARM 0xb0894: bne 0xb093c <entry+0x41ec>
    c.b(C::Ne,0xb093c,false)?;
    // ARM 0xb0898: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // ARM 0xb089c: add r6, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // ARM 0xb08a0: sub r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb08a4: str r2, [r6, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,-64)?)?;
    // ARM 0xb08a8: ldr r3, [r4, r8]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(4)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb08ac: strb r3, [r1, r0]
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::new(3)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb08b0: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xb08b4: ldrb r3, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // ARM 0xb08b8: cmp r0, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb08bc: blt 0xb0880 <entry+0x4130>
    c.b(C::Lt,0xb0880,false)?;
    // ARM 0xb08c0: b 0xb08c8 <entry+0x4178>
    c.b(C::Al,0xb08c8,false)?;
    // ARM 0xb08c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb08c8: ldrb r1, [r7, #0xd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(7)?,13)?)?;
    // ARM 0xb08cc: add r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb08d0: strb r1, [r7, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(7)?,16)?)?;
    // ARM 0xb08d4: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // ARM 0xb08d8: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb08dc: bge 0xb0b9c <entry+0x444c>
    c.b(C::Ge,0xb0b9c,false)?;
    // ARM 0xb08e0: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // ARM 0xb08e4: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // ARM 0xb08e8: mov lr, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,16777216)?)?;
    // ARM 0xb08ec: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // ARM 0xb08f0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // ARM 0xb08f4: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // ARM 0xb08f8: str lr, [r1, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::LR,R::R1,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb08fc: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // ARM 0xb0900: ldr r1, [r4, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(4)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0904: cmp r12, r1, lsr #15
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::R1,Shift::Lsr,15,false)?)?;
    // ARM 0xb0908: bne 0xb0b74 <entry+0x4424>
    c.b(C::Ne,0xb0b74,false)?;
    // ARM 0xb090c: add r1, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb0910: add r3, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::R0,Shift::Lsl,2,false)?)?;
    // ARM 0xb0914: add r5, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R1,1)?)?;
    // ARM 0xb0918: add r1, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb091c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xb0920: str r5, [r3, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,20)?)?;
    // ARM 0xb0924: ldr r3, [r4, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(4)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0928: strb r3, [r1, #0x54]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R1,84)?)?;
    // ARM 0xb092c: ldrb r1, [r7, #0x10]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(7)?,16)?)?;
    // ARM 0xb0930: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb0934: blt 0xb08f4 <entry+0x41a4>
    c.b(C::Lt,0xb08f4,false)?;
    // ARM 0xb0938: b 0xb0b9c <entry+0x444c>
    c.b(C::Al,0xb0b9c,false)?;
    // ARM 0xb093c: ldr r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,24)?)?;
    // ARM 0xb0940: mov r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,63)?)?;
    // ARM 0xb0944: b 0xb0708 <entry+0x3fb8>
    c.b(C::Al,0xb0708,false)?;
    // ARM 0xb0948: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // ARM 0xb094c: sub r6, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(5)?,1)?)?;
    // ARM 0xb0950: sub r0, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0954: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // ARM 0xb0958: ldr r6, [r0, #0xb8]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(6)?,R::R0,184)?)?;
    // ARM 0xb095c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // ARM 0xb0960: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // ARM 0xb0964: beq 0xb0b6c <entry+0x441c>
    c.b(C::Eq,0xb0b6c,false)?;
    // ARM 0xb0968: lsl r3, r3, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsl,24)?)?;
    // ARM 0xb096c: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // ARM 0xb0970: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // ARM 0xb0974: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM 0xb0978: mov r5, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // ARM 0xb097c: sub r3, r4, r3, asr #23
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(4)?,R::new(3)?,Shift::Asr,23,false)?)?;
    // ARM 0xb0980: mul r0, r3, r2
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(3)?,R::R2)?)?;
    // ARM 0xb0984: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // ARM 0xb0988: ldrb r2, [r6, #0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,12)?)?;
    // ARM 0xb098c: ldrb r3, [r6, #0xd]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,13)?)?;
    // ARM 0xb0990: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // ARM 0xb0994: add r0, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0998: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // ARM 0xb099c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xb09a0: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM 0xb09a4: str r4, [lr, r5]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(4)?,R::LR,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb09a8: ldr r12, [r8, r5]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R12,R::new(8)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb09ac: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // ARM 0xb09b0: streq r4, [lr, r5]
    c.emit(arm32::transfer_word_register_offset(C::Eq,false,R::new(4)?,R::LR,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb09b4: ldreq r6, [r8, r5]
    c.emit(arm32::transfer_word_register_offset(C::Eq,true,R::new(6)?,R::new(8)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb09b8: lsleq r12, r6, #1
    c.emit(arm32::mov_shifted_register(C::Eq,R::R12,R::new(6)?,Shift::Lsl,1)?)?;
    // ARM 0xb09bc: sub r6, r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R2,5)?)?;
    // ARM 0xb09c0: sub r0, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R12,1)?)?;
    // ARM 0xb09c4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xb09c8: bhi 0xb0a30 <entry+0x42e0>
    c.b(C::Hi,0xb0a30,false)?;
    // ARM 0xb09cc: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // ARM 0xb09d0: mov r8, lr
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::LR))?;
    // ARM 0xb09d4: mov lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // ARM 0xb09d8: and r4, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(3)?,255)?)?;
    // ARM 0xb09dc: and r1, r10, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(10)?,255)?)?;
    // ARM 0xb09e0: mov r5, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // ARM 0xb09e4: add r1, r1, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb09e8: add r4, r7, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(7)?,R::R1,Shift::Lsl,2,false)?)?;
    // ARM 0xb09ec: add r1, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb09f0: cmp r6, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb09f4: ldrlt r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Lt,true,R::R0,R::SP,12)?)?;
    // ARM 0xb09f8: ldrlt r0, [r0]
    c.emit(arm32::transfer_word(C::Lt,true,R::R0,R::R0,0)?)?;
    // ARM 0xb09fc: ldrlt lr, [r0, r2, lsl #2]
    c.emit(arm32::transfer_word_register_offset(C::Lt,true,R::LR,R::R0,R::R2,Shift::Lsl,2,false)?)?;
    // ARM 0xb0a00: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // ARM 0xb0a04: cmp r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,1)?)?;
    // ARM 0xb0a08: movne r12, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R12,R::R0,0)?)?;
    // ARM 0xb0a0c: add r0, lr, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::LR,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0a10: mov lr, r8
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(8)?))?;
    // ARM 0xb0a14: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // ARM 0xb0a18: strb r12, [r1, #0x54]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R1,84)?)?;
    // ARM 0xb0a1c: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // ARM 0xb0a20: add r3, r3, r0, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::R0,Shift::Lsr,31,false)?)?;
    // ARM 0xb0a24: ldr r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,16)?)?;
    // ARM 0xb0a28: eor r0, r4, r0, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::new(4)?,R::R0,Shift::Lsr,31,false)?)?;
    // ARM 0xb0a2c: add r10, r10, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(10)?,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0a30: and r0, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(3)?,255)?)?;
    // ARM 0xb0a34: and r1, r10, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(10)?,255)?)?;
    // ARM 0xb0a38: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0a3c: cmp r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // ARM 0xb0a40: bhi 0xb0a54 <entry+0x4304>
    c.b(C::Hi,0xb0a54,false)?;
    // ARM 0xb0a44: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // ARM 0xb0a48: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // ARM 0xb0a4c: cmp r6, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0a50: blt 0xb09a4 <entry+0x4254>
    c.b(C::Lt,0xb09a4,false)?;
    // ARM 0xb0a54: strb r3, [r11, #-0x1d]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R11,-29)?)?;
    // ARM 0xb0a58: strb r10, [r11, #-0x1e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::R11,-30)?)?;
    // ARM 0xb0a5c: sub r0, r11, #29
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,29)?)?;
    // ARM 0xb0a60: sub r1, r11, #30
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,30)?)?;
    // ARM 0xb0a64: ldrb r2, [r9, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,123)?)?;
    // ARM 0xb0a68: bl 0xb0c64 <entry+0x4514>
    c.b(C::Al,0xb0c64,true)?;
    // ARM 0xb0a6c: ldrb r3, [r11, #-0x1d]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R11,-29)?)?;
    // ARM 0xb0a70: strb r3, [r7, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(7)?,12)?)?;
    // ARM 0xb0a74: ldrb r0, [r11, #-0x1e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R11,-30)?)?;
    // ARM 0xb0a78: strb r0, [r7, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,13)?)?;
    // ARM 0xb0a7c: add r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0a80: strb r0, [r7, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,16)?)?;
    // ARM 0xb0a84: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // ARM 0xb0a88: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM 0xb0a8c: blo 0xb0b00 <entry+0x43b0>
    c.b(C::Cc,0xb0b00,false)?;
    // ARM 0xb0a90: add r12, r7, #85
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(7)?,85)?)?;
    // ARM 0xb0a94: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb0a98: add r3, r12, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R12,R::R1,Shift::Lsl,2,false)?)?;
    // ARM 0xb0a9c: ldrb r2, [r12, r1]
    c.emit(arm32::transfer_byte_register_offset(C::Al,true,R::R2,R::R12,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb0aa0: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // ARM 0xb0aa4: ldr r3, [r3, #-0x3d]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,-61)?)?;
    // ARM 0xb0aa8: add r5, r7, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(7)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // ARM 0xb0aac: ldr r4, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,20)?)?;
    // ARM 0xb0ab0: cmp r3, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0ab4: bge 0xb0ad0 <entry+0x4380>
    c.b(C::Ge,0xb0ad0,false)?;
    // ARM 0xb0ab8: str r4, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,24)?)?;
    // ARM 0xb0abc: add r4, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0ac0: ldrb r0, [r4, #0x54]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,84)?)?;
    // ARM 0xb0ac4: strb r0, [r4, #0x55]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,85)?)?;
    // ARM 0xb0ac8: str r3, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,20)?)?;
    // ARM 0xb0acc: strb r2, [r4, #0x54]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,84)?)?;
    // ARM 0xb0ad0: sub r0, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(6)?,1)?)?;
    // ARM 0xb0ad4: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // ARM 0xb0ad8: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // ARM 0xb0adc: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // ARM 0xb0ae0: bgt 0xb0aa8 <entry+0x4358>
    c.b(C::Gt,0xb0aa8,false)?;
    // ARM 0xb0ae4: ldrb r2, [r7, #0x10]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(7)?,16)?)?;
    // ARM 0xb0ae8: add r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,1)?)?;
    // ARM 0xb0aec: add r1, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2)?)?;
    // ARM 0xb0af0: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0af4: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // ARM 0xb0af8: blt 0xb0a98 <entry+0x4348>
    c.b(C::Lt,0xb0a98,false)?;
    // ARM 0xb0afc: ldrb r3, [r7, #0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(7)?,12)?)?;
    // ARM 0xb0b00: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // ARM 0xb0b04: blo 0xb0b9c <entry+0x444c>
    c.b(C::Cc,0xb0b9c,false)?;
    // ARM 0xb0b08: add r0, r7, r3, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // ARM 0xb0b0c: add r1, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0b10: add r2, r7, #84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,84)?)?;
    // ARM 0xb0b14: sub r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?)?;
    // ARM 0xb0b18: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM 0xb0b1c: add r12, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R0,16)?)?;
    // ARM 0xb0b20: add lr, r1, #83
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R1,83)?)?;
    // ARM 0xb0b24: orr r5, r3, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0b28: cmp r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,16)?)?;
    // ARM 0xb0b2c: bhs 0xb0b8c <entry+0x443c>
    c.b(C::Cs,0xb0b8c,false)?;
    // ARM 0xb0b30: ldr r1, [r12, -r6, lsl #2]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::R12,R::new(6)?,Shift::Lsl,2,true)?)?;
    // ARM 0xb0b34: add r4, r2, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R2,R::new(6)?,Shift::Lsl,2,false)?)?;
    // ARM 0xb0b38: ldrb r5, [r2, r6]
    c.emit(arm32::transfer_byte_register_offset(C::Al,true,R::new(5)?,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0b3c: sub r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?)?;
    // ARM 0xb0b40: ldr r0, [r4, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,-64)?)?;
    // ARM 0xb0b44: str r1, [r4, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,-64)?)?;
    // ARM 0xb0b48: ldrb r1, [lr, -r6]
    c.emit(arm32::transfer_byte_register_offset(C::Al,true,R::R1,R::LR,R::new(6)?,Shift::Lsl,0,true)?)?;
    // ARM 0xb0b4c: strb r1, [r2, r6]
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::R1,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0b50: str r0, [r12, -r6, lsl #2]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::R12,R::new(6)?,Shift::Lsl,2,true)?)?;
    // ARM 0xb0b54: strb r5, [lr, -r6]
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::new(5)?,R::LR,R::new(6)?,Shift::Lsl,0,true)?)?;
    // ARM 0xb0b58: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // ARM 0xb0b5c: ldrb r0, [r7, #0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,12)?)?;
    // ARM 0xb0b60: cmp r6, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsr,1,false)?)?;
    // ARM 0xb0b64: blt 0xb0b24 <entry+0x43d4>
    c.b(C::Lt,0xb0b24,false)?;
    // ARM 0xb0b68: b 0xb0b9c <entry+0x444c>
    c.b(C::Al,0xb0b9c,false)?;
    // ARM 0xb0b6c: mov r0, #62
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,62)?)?;
    // ARM 0xb0b70: b 0xb0708 <entry+0x3fb8>
    c.b(C::Al,0xb0708,false)?;
    // ARM 0xb0b74: ldr r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,24)?)?;
    // ARM 0xb0b78: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // ARM 0xb0b7c: b 0xb0708 <entry+0x3fb8>
    c.b(C::Al,0xb0708,false)?;
    // ARM 0xb0b80: str r5, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,0)?)?;
    // ARM 0xb0b84: mov r0, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,60)?)?;
    // ARM 0xb0b88: b 0xb0708 <entry+0x3fb8>
    c.b(C::Al,0xb0708,false)?;
    // ARM 0xb0b8c: ldr r2, [pc, #0xbc]
    c.literal(R::R2,0xb0c50)?;
    // ARM 0xb0b90: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb0b94: mov r1, #174
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,174)?)?;
    // ARM 0xb0b98: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xb0b9c: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // ARM 0xb0ba0: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // ARM 0xb0ba4: mov r4, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // ARM 0xb0ba8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb0bac: orr r0, r0, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,15,false)?)?;
    // ARM 0xb0bb0: lsl r5, r1, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R1,Shift::Lsl,15)?)?;
    // ARM 0xb0bb4: ldr r0, [r0, r4]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0bb8: tst r0, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // ARM 0xb0bbc: beq 0xb0c00 <entry+0x44b0>
    c.b(C::Eq,0xb0c00,false)?;
    // ARM 0xb0bc0: orr r1, r5, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,124)?)?;
    // ARM 0xb0bc4: mov r2, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192)?)?;
    // ARM 0xb0bc8: mov r3, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,106)?)?;
    // ARM 0xb0bcc: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xb0bd0: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // ARM 0xb0bd4: str r2, [r1, r4]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0bd8: stm sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xb0bdc: ldr r1, [pc, #0x70]
    c.literal(R::R1,0xb0c54)?;
    // ARM 0xb0be0: ldr r2, [pc, #0x70]
    c.literal(R::R2,0xb0c58)?;
    // ARM 0xb0be4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0be8: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0bec: ldr r1, [pc, #0x68]
    c.literal(R::R1,0xb0c5c)?;
    // ARM 0xb0bf0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0bf4: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0bf8: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xb0bfc: b 0xb0c04 <entry+0x44b4>
    c.b(C::Al,0xb0c04,false)?;
    // ARM 0xb0c00: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb0c04: orr r1, r5, #116
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,116)?)?;
    // ARM 0xb0c08: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xb0c0c: ldr r1, [r1, r4]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0c10: ldr r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,24)?)?;
    // ARM 0xb0c14: and r2, r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,5)?)?;
    // ARM 0xb0c18: cmp r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // ARM 0xb0c1c: beq 0xb0708 <entry+0x3fb8>
    c.b(C::Eq,0xb0708,false)?;
    // ARM 0xb0c20: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // ARM 0xb0c24: ldr r1, [pc, #0x34]
    c.literal(R::R1,0xb0c60)?;
    // ARM 0xb0c28: ldr r2, [pc, #0x28]
    c.literal(R::R2,0xb0c58)?;
    // ARM 0xb0c2c: mov r3, #107
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,107)?)?;
    // ARM 0xb0c30: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0c34: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // ARM 0xb0c38: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0c3c: ldr r1, [pc, #0x18]
    c.literal(R::R1,0xb0c5c)?;
    // ARM 0xb0c40: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0c44: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0c48: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xb0c4c: b 0xb0708 <entry+0x3fb8>
    c.b(C::Al,0xb0708,false)?;
    c.check(0xb0c50)?;
    // ARM 0xb0c50: literal or constant data (NOT executable)
    c.emit(0x000f88c7)?;
    // ARM 0xb0c54: literal or constant data (NOT executable)
    c.emit(0x000f7058)?;
    // ARM 0xb0c58: literal or constant data (NOT executable)
    c.emit(0x000f8902)?;
    // ARM 0xb0c5c: literal or constant data (NOT executable)
    c.emit(0x000e2788)?;
    // ARM 0xb0c60: literal or constant data (NOT executable)
    c.emit(0x000f7095)?;
    c.check(0xb0c64)
}
