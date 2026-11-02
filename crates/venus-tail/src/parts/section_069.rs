fn reconcile_controller_pending_output(c:&mut Code)->Result<(),String>{
    c.check(0xbe7dc)?;
    // VMA 0xbe7dc: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbe7e0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe7e4: bl 0x2ec94 <entry+0xfffffffffff704b8>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe7e8: ldr r1, [pc, #0x448]
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbe7ec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe7f0: bl 0x2ec94 <entry+0xfffffffffff704b8>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe7f4: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbe7f8: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe7fc: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbe800: bl 0xe1e60 <entry+0x23684>
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe804: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe808: bne 0xbe8b8 <entry+0xdc>
    c.b(C::Ne,0xbe8b8,false)?;
    // VMA 0xbe80c: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // VMA 0xbe810: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xbe814: orr r1, r0, r1, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R1,Shift::Lsl,12,false)?)?;
    // VMA 0xbe818: mov r0, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1920)?)?;
    // VMA 0xbe81c: cmp r1, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // VMA 0xbe820: str r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,8)?)?;
    // VMA 0xbe824: bhi 0xbe834 <entry+0x58>
    c.b(C::Hi,0xbe834,false)?;
    // VMA 0xbe828: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbe82c: cmp r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // VMA 0xbe830: bhi 0xbe840 <entry+0x64>
    c.b(C::Hi,0xbe840,false)?;
    // VMA 0xbe834: str r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,8)?)?;
    // VMA 0xbe838: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbe83c: str r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbe840: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe844: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe848: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbe84c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe850: bl 0xe1e60 <entry+0x23684>
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe854: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe858: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xbe85c: bne 0xbe8c0 <entry+0xe4>
    c.b(C::Ne,0xbe8c0,false)?;
    // VMA 0xbe860: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // VMA 0xbe864: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xbe868: orr r1, r0, r1, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R1,Shift::Lsl,12,false)?)?;
    // VMA 0xbe86c: mov r0, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1920)?)?;
    // VMA 0xbe870: cmp r1, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // VMA 0xbe874: str r1, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,12)?)?;
    // VMA 0xbe878: bhi 0xbe888 <entry+0xac>
    c.b(C::Hi,0xbe888,false)?;
    // VMA 0xbe87c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbe880: cmp r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // VMA 0xbe884: bhi 0xbe894 <entry+0xb8>
    c.b(C::Hi,0xbe894,false)?;
    // VMA 0xbe888: str r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,12)?)?;
    // VMA 0xbe88c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbe890: str r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbe894: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe898: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe89c: mov r1, #29
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,29)?)?;
    // VMA 0xbe8a0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe8a4: bl 0xe1e60 <entry+0x23684>
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe8a8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe8ac: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe8b0: movne r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xbe8b4: b 0xbe8c0 <entry+0xe4>
    c.b(C::Al,0xbe8c0,false)?;
    // VMA 0xbe8b8: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe8bc: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xbe8c0: ldr r0, [r9, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,156)?)?;
    // VMA 0xbe8c4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xbe8c8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe8cc: str r0, [r6, #0x454]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1108)?)?;
    // VMA 0xbe8d0: ldr r0, [r9, #0xa0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,160)?)?;
    // VMA 0xbe8d4: str r0, [r6, #0x458]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1112)?)?;
    // VMA 0xbe8d8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbe8dc: str r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xbe8e0: ldr r0, [r9, #0xa4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,164)?)?;
    // VMA 0xbe8e4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbe8e8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbe8ec: moveq r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbe8f0: str r7, [r6, #0xa9c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,2716)?)?;
    // VMA 0xbe8f4: str r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbe8f8: ldr r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbe8fc: cmp r0, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbe900: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xbe904: movne r0, #13
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,13)?)?;
    // VMA 0xbe908: orrne r0, r0, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbe90c: strne r0, [r5, #0x23c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(5)?,572)?)?;
    // VMA 0xbe910: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xbe914: b 0xbea8c <entry+0x2b0>
    c.b(C::Al,0xbea8c,false)?;
    // VMA 0xbe918: ldrb r0, [r4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbe91c: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe920: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe924: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // VMA 0xbe928: movne r0, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe92c: strbne r0, [r4]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbe930: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbe934: bl 0xe1e60 <entry+0x23684>
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe938: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe93c: bne 0xbe9d8 <entry+0x1fc>
    c.b(C::Ne,0xbe9d8,false)?;
    // VMA 0xbe940: ldrb r0, [r11, #-0x20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe944: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbe948: strb r0, [r9, #0x99]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,153)?)?;
    // VMA 0xbe94c: beq 0xbe9fc <entry+0x220>
    c.b(C::Eq,0xbe9fc,false)?;
    // VMA 0xbe950: add r0, r9, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,88)?)?;
    // VMA 0xbe954: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // VMA 0xbe958: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbe95c: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xbe960: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbe964: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbe968: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe96c: bl 0xe1e60 <entry+0x23684>
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe970: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe974: bne 0xbe9d8 <entry+0x1fc>
    c.b(C::Ne,0xbe9d8,false)?;
    // VMA 0xbe978: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe97c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbe980: beq 0xbe98c <entry+0x1b0>
    c.b(C::Eq,0xbe98c,false)?;
    // VMA 0xbe984: ldr r1, [pc, #0x2dc]
    c.literal(R::R1,0xbec68)?;
    // VMA 0xbe988: b 0xbe9b4 <entry+0x1d8>
    c.b(C::Al,0xbe9b4,false)?;
    // VMA 0xbe98c: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbe990: beq 0xbe9ac <entry+0x1d0>
    c.b(C::Eq,0xbe9ac,false)?;
    // VMA 0xbe994: ldr r0, [pc, #0x2cc]
    c.literal(R::R0,0xbec68)?;
    // VMA 0xbe998: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // VMA 0xbe99c: add r0, r1, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbe9a0: ldrb r0, [r0, #-0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,-1)?)?;
    // VMA 0xbe9a4: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbe9a8: b 0xbe9b4 <entry+0x1d8>
    c.b(C::Al,0xbe9b4,false)?;
    // VMA 0xbe9ac: ldr r1, [pc, #0x2b8]
    c.literal(R::R1,0xbec6c)?;
    // VMA 0xbe9b0: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbe9b4: ldrb r1, [r1, r10]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbe9b8: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // VMA 0xbe9bc: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // VMA 0xbe9c0: cmp r10, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,64)?)?;
    // VMA 0xbe9c4: strb r0, [r2, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbe9c8: blo 0xbe960 <entry+0x184>
    c.b(C::Cc,0xbe960,false)?;
    // VMA 0xbe9cc: b 0xbea00 <entry+0x224>
    c.b(C::Al,0xbea00,false)?;
    // VMA 0xbe9d0: strb r0, [r5, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,11)?)?;
    // VMA 0xbe9d4: b 0xbe320 <entry+0xfffffffffffffb44>
    c.b(C::Al,0xbe320,false)?;
    // VMA 0xbe9d8: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbe9dc: b 0xbea00 <entry+0x224>
    c.b(C::Al,0xbea00,false)?;
    // VMA 0xbe9e0: strb r0, [r5, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,12)?)?;
    // VMA 0xbe9e4: b 0xbe368 <entry+0xfffffffffffffb8c>
    c.b(C::Al,0xbe368,false)?;
    // classified literal/data at 0xbe9e8 — caller must supply value
    c.emit(STREAM_PENDING_RECORD_MESSAGE)?;
    // VMA 0xbe9ec: cmp r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,9)?)?;
    // VMA 0xbe9f0: bhs 0xbdf68 <entry+0xfffffffffffff78c>
    c.b(C::Cs,0xbdf68,false)?;
    // VMA 0xbe9f4: b 0xbdf98 <entry+0xfffffffffffff7bc>
    c.b(C::Al,0xbdf98,false)?;
    // classified literal/data at 0xbe9f8 — caller must supply value
    c.emit(STREAM_ALTERNATE_RECORD_MESSAGE)?;
    // VMA 0xbe9fc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbea00: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbea04: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // VMA 0xbea08: add r1, r9, #196
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,196)?)?;
    // VMA 0xbea0c: str r4, [r9, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,192)?)?;
    // VMA 0xbea10: stm r1, {r0, r4, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(4)?.bit()|R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xbea14: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xbea18: str r8, [r9, #0xd0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,208)?)?;
    // VMA 0xbea1c: str r8, [r9, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,212)?)?;
    // VMA 0xbea20: str r4, [r9, #0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,176)?)?;
    // VMA 0xbea24: str r4, [r9, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,180)?)?;
    // VMA 0xbea28: str r8, [r9, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,224)?)?;
    // VMA 0xbea2c: str r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xbea30: str r8, [r9, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,164)?)?;
    // VMA 0xbea34: str r4, [r6, #0xa9c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,2716)?)?;
    // VMA 0xbea38: str r8, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,240)?)?;
    // VMA 0xbea3c: ldr r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbea40: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // VMA 0xbea44: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbea48: beq 0xbea80 <entry+0x2a4>
    c.b(C::Eq,0xbea80,false)?;
    // VMA 0xbea4c: mov r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // VMA 0xbea50: ldr r2, [pc, #0x1f0]
    c.literal(R::R2,0xbec48)?;
    // VMA 0xbea54: mov r3, #147
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,147)?)?;
    // VMA 0xbea58: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xbea5c: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // VMA 0xbea60: str r1, [r5, #0x23c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,572)?)?;
    // VMA 0xbea64: ldr r1, [pc, #0x204]
    c.literal(R::R1,0xbec70)?;
    // VMA 0xbea68: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbea6c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbea70: bl 0x2ec94 <entry+0xfffffffffff704b8>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbea74: ldr r1, [pc, #0x1bc]
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbea78: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbea7c: bl 0x2ec94 <entry+0xfffffffffff704b8>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbea80: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbea84: cmp r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1)?)?;
    // VMA 0xbea88: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xbea8c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbea90: beq 0xbd744 <entry+0xffffffffffffef68>
    c.b(C::Eq,0xbd744,false)?;
    // VMA 0xbea94: b 0xbde10 <entry+0xfffffffffffff634>
    c.b(C::Al,0xbde10,false)?;
    // VMA 0xbea98: strb r0, [r5, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,13)?)?;
    // VMA 0xbea9c: b 0xbe3b0 <entry+0xfffffffffffffbd4>
    c.b(C::Al,0xbe3b0,false)?;
    // VMA 0xbeaa0: cmp r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,11)?)?;
    // VMA 0xbeaa4: bhs 0xbdfc4 <entry+0xfffffffffffff7e8>
    c.b(C::Cs,0xbdfc4,false)?;
    // VMA 0xbeaa8: b 0xbdff0 <entry+0xfffffffffffff814>
    c.b(C::Al,0xbdff0,false)?;
    // VMA 0xbeaac: strb r0, [r5, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,14)?)?;
    // VMA 0xbeab0: b 0xbe3f8 <entry+0xfffffffffffffc1c>
    c.b(C::Al,0xbe3f8,false)?;
    // VMA 0xbeab4: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // VMA 0xbeab8: bhs 0xbe024 <entry+0xfffffffffffff848>
    c.b(C::Cs,0xbe024,false)?;
    // VMA 0xbeabc: b 0xbe054 <entry+0xfffffffffffff878>
    c.b(C::Al,0xbe054,false)?;
    // VMA 0xbeac0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbeac4: strb r0, [r5, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,16)?)?;
    // VMA 0xbeac8: b 0xbeae4 <entry+0x308>
    c.b(C::Al,0xbeae4,false)?;
    // VMA 0xbeacc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbead0: strb r0, [r5, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,16)?)?;
    // VMA 0xbead4: b 0xbeae4 <entry+0x308>
    c.b(C::Al,0xbeae4,false)?;
    // VMA 0xbead8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbeadc: strb r0, [r5, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,16)?)?;
    // VMA 0xbeae0: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xbeae4: str r0, [r5, #0x2d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,728)?)?;
    // VMA 0xbeae8: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbeaec: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbeaf0: mov r1, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10)?)?;
    // VMA 0xbeaf4: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbeaf8: bl 0xe1e60 <entry+0x23684>
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbeafc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbeb00: bne 0xbec28 <entry+0x44c>
    c.b(C::Ne,0xbec28,false)?;
    // VMA 0xbeb04: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbeb08: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbeb0c: and r2, r1, r0, lsr #9
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,9,false)?)?;
    // VMA 0xbeb10: strb r2, [r5, #0x11]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,17)?)?;
    // VMA 0xbeb14: and r2, r1, r0, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,8,false)?)?;
    // VMA 0xbeb18: strb r2, [r5, #0x12]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,18)?)?;
    // VMA 0xbeb1c: and r2, r1, r0, lsr #7
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,7,false)?)?;
    // VMA 0xbeb20: strb r2, [r5, #0x13]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,19)?)?;
    // VMA 0xbeb24: and r2, r1, r0, lsr #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,6,false)?)?;
    // VMA 0xbeb28: strb r2, [r5, #0x14]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,20)?)?;
    // VMA 0xbeb2c: and r2, r1, r0, lsr #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,5,false)?)?;
    // VMA 0xbeb30: strb r2, [r5, #0x15]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,21)?)?;
    // VMA 0xbeb34: and r2, r1, r0, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,4,false)?)?;
    // VMA 0xbeb38: strb r2, [r5, #0x16]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,22)?)?;
    // VMA 0xbeb3c: and r2, r1, r0, lsr #3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,3,false)?)?;
    // VMA 0xbeb40: and r1, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // VMA 0xbeb44: strb r2, [r5, #0x17]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,23)?)?;
    // VMA 0xbeb48: strb r1, [r5, #0x18]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,24)?)?;
    // VMA 0xbeb4c: ldr r1, [r5, #0x2d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,728)?)?;
    // VMA 0xbeb50: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // VMA 0xbeb54: beq 0xbeb78 <entry+0x39c>
    c.b(C::Eq,0xbeb78,false)?;
    // VMA 0xbeb58: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xbeb5c: beq 0xbeb9c <entry+0x3c0>
    c.b(C::Eq,0xbeb9c,false)?;
    // VMA 0xbeb60: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xbeb64: bne 0xbebcc <entry+0x3f0>
    c.b(C::Ne,0xbebcc,false)?;
    // VMA 0xbeb68: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbeb6c: b 0xbeba0 <entry+0x3c4>
    c.b(C::Al,0xbeba0,false)?;
    c.check(0xbeb70)
}

/// Finish the frame queue status path: classify whether output may be
/// queued, update its pointer/state slots, and preserve the two preceding
/// resource-data references that were misidentified as A32 ANDEQ.
fn finalize_stream_pending_queue(c:&mut Code)->Result<(),String>{
    c.check(0xbeb70)?;
    const PENDING_STREAM_MESSAGE_POINTERS:[u32;2]=[0x000fa40a,0x000fa438];
    for value in PENDING_STREAM_MESSAGE_POINTERS {c.emit(value)?;}
    c.check(0xbeb78)?;
    // VMA 0xbeb78: ldr r1, [r9, #0xa4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,164)?)?;
    // VMA 0xbeb7c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbeb80: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbeb84: moveq r2, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xbeb88: sub r1, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R2,1)?)?;
    // VMA 0xbeb8c: str r2, [r5, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,732)?)?;
    // VMA 0xbeb90: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xbeb94: blo 0xbeba4 <entry+0x34>    @ imm = #0x8
    c.b(C::Cc,0xbeba4,false)?;
    // VMA 0xbeb98: b 0xbebf8 <entry+0x88>    @ imm = #0x58
    c.b(C::Al,0xbebf8,false)?;
    // VMA 0xbeb9c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbeba0: str r1, [r5, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,732)?)?;
    // VMA 0xbeba4: ldr r1, [r9, #0x864]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2148)?)?;
    // VMA 0xbeba8: ldrb r2, [r5, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,8)?)?;
    // VMA 0xbebac: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbebb0: beq 0xbebd8 <entry+0x68>    @ imm = #0x20
    c.b(C::Eq,0xbebd8,false)?;
    // VMA 0xbebb4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xbebb8: bne 0xbebe8 <entry+0x78>    @ imm = #0x28
    c.b(C::Ne,0xbebe8,false)?;
    // VMA 0xbebbc: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xbebc0: ldreq r2, [r9, #0x860]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::new(9)?,2144)?)?;
    // VMA 0xbebc4: streq r2, [r5, #0x2c4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(5)?,708)?)?;
    // VMA 0xbebc8: b 0xbebe8 <entry+0x78>    @ imm = #0x18
    c.b(C::Al,0xbebe8,false)?;
    // VMA 0xbebcc: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xbebd0: str r1, [r5, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,732)?)?;
    // VMA 0xbebd4: b 0xbebf8 <entry+0x88>    @ imm = #0x1c
    c.b(C::Al,0xbebf8,false)?;
    // VMA 0xbebd8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xbebdc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xbebe0: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xbebe4: str r3, [r9, #0x860]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,2144)?)?;
    // VMA 0xbebe8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbebec: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbebf0: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbebf4: str r2, [r9, #0x864]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,2148)?)?;
    // VMA 0xbebf8: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbebfc: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // VMA 0xbec00: beq 0xbec1c <entry+0xac>    @ imm = #0x14
    c.b(C::Eq,0xbec1c,false)?;
    // VMA 0xbec04: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbec08: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbec0c: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // VMA 0xbec10: bl 0xe1e60 <entry+0x232f0> @ imm = #0x23248
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbec14: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbec18: bne 0xbec28 <entry+0xb8>    @ imm = #0x8
    c.b(C::Ne,0xbec28,false)?;
    // VMA 0xbec1c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbec20: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xbec24: b 0xbe5b0 <entry+0xfffffffffffffa40> @ imm = #-0x67c
    c.b(C::Al,0xbe5b0,false)?;
    // VMA 0xbec28: str r4, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,240)?)?;
    // VMA 0xbec2c: b 0xbe5b4 <entry+0xfffffffffffffa44> @ imm = #-0x680
    c.b(C::Al,0xbe5b4,false)?;
    c.check(0xbec30)?;
    const PENDING_CODEC_MESSAGE_AND_STATE_DATA:[u32;18]=[0x000fa520,0x000fa548,0x000e2788,0x000fa55e,0x000fa583,0x000fa308,0x000fa2d4,0x000fa44a,0x000fa463,0x000fa483,0x000fa495,0x000fa4cd,0x000fa4fa,0x000fa330,0x0010bb39,0x0010bb79,0x000fa2af,0x000fa3a8];
    for value in PENDING_CODEC_MESSAGE_AND_STATE_DATA {c.emit(value)?;}
    c.check(0xbec78)?;
    Ok(())
}

/// Compare the returned buffer records against a controller-selected
/// byte-index order table. Exit on fault without inventing a queue state.
fn remap_pending_stream_record_bytes(c:&mut Code)->Result<(),String>{
    c.check(0xbec78)?;
    // VMA 0xbec78: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xbec7c: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xbec80: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xbec84: mov r7, r2
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // VMA 0xbec88: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // VMA 0xbec8c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbec90: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xbec94: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // VMA 0xbec98: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbec9c: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xbeca0: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xbeca4: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbeca8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xbecac: bl 0xe1e60 <entry+0x232f0> @ imm = #0x231ac
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbecb0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbecb4: bne 0xbed54 <entry+0x1e4>   @ imm = #0x98
    c.b(C::Ne,0xbed54,false)?;
    // VMA 0xbecb8: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xbecbc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbecc0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbecc4: beq 0xbed54 <entry+0x1e4>   @ imm = #0x88
    c.b(C::Eq,0xbed54,false)?;
    // VMA 0xbecc8: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xbeccc: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // VMA 0xbecd0: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xbecd4: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbecd8: str r5, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,0)?)?;
    // VMA 0xbecdc: bl 0xe1e60 <entry+0x232f0> @ imm = #0x2317c
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbece0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbece4: bne 0xbed54 <entry+0x1e4>   @ imm = #0x68
    c.b(C::Ne,0xbed54,false)?;
    // VMA 0xbece8: ldr r9, [pc, #0x70]         @ 0x1f0 <entry+0x1f0>
    c.literal(R::new(9)?,0xbed60)?;
    // VMA 0xbecec: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbecf0: add r8, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,4)?)?;
    // VMA 0xbecf4: b 0xbed00 <entry+0x190>   @ imm = #0x4
    c.b(C::Al,0xbed00,false)?;
    // VMA 0xbecf8: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbecfc: b 0xbed34 <entry+0x1c4>   @ imm = #0x30
    c.b(C::Al,0xbed34,false)?;
    // VMA 0xbed00: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xbed04: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbed08: bne 0xbed20 <entry+0x1b0>   @ imm = #0x10
    c.b(C::Ne,0xbed20,false)?;
    // VMA 0xbed0c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbed10: beq 0xbecf8 <entry+0x188>   @ imm = #-0x20
    c.b(C::Eq,0xbecf8,false)?;
    // VMA 0xbed14: add r0, r9, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbed18: ldrb r0, [r0, #-0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,-1)?)?;
    // VMA 0xbed1c: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbed20: ldrb r1, [r9, r7]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(9)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xbed24: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xbed28: cmp r7, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,64)?)?;
    // VMA 0xbed2c: strb r0, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbed30: bhs 0xbed50 <entry+0x1e0>   @ imm = #0x18
    c.b(C::Cs,0xbed50,false)?;
    // VMA 0xbed34: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xbed38: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbed3c: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xbed40: bl 0xe1e60 <entry+0x232f0> @ imm = #0x23118
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbed44: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbed48: beq 0xbed00 <entry+0x190>   @ imm = #-0x50
    c.b(C::Eq,0xbed00,false)?;
    // VMA 0xbed4c: b 0xbed54 <entry+0x1e4>   @ imm = #0x0
    c.b(C::Al,0xbed54,false)?;
    // VMA 0xbed50: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbed54: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xbed58: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xbed5c: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xbed60)?;
    const CONTROLLER_RECORD_INDEX_ADDRESS:[u32;1]=[0x0010bb79];
    for value in CONTROLLER_RECORD_INDEX_ADDRESS {c.emit(value)?;}
    c.check(0xbed64)?;
    Ok(())
}
