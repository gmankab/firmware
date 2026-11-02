
/// Complete codec output plane sizing, frame flags, weighted selection and
/// the fallback register commits. The original return and 23-entry literal
/// table are kept separate from the executable A32 instruction stream.
fn finish_codec_output_frame_descriptor(c:&mut Code)->Result<(),String>{
    c.check(0xbf400)?;
    // VMA 0xbf400: tst r2, #-2147483647
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,2147483649)?)?;
    // VMA 0xbf404: beq 0xbf444 <entry+0x8d4>   @ imm = #0x38
    c.b(C::Eq,0xbf444,false)?;
    // VMA 0xbf408: orr r2, r1, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbf40c: ldr r2, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbf410: tst r2, #268435457
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,268435457)?)?;
    // VMA 0xbf414: beq 0xbf444 <entry+0x8d4>   @ imm = #0x28
    c.b(C::Eq,0xbf444,false)?;
    // VMA 0xbf418: ldr r1, [pc, #0xa0c]        @ 0x12bc <entry+0x12bc>
    c.literal(R::R1,0xbfe2c)?;
    // VMA 0xbf41c: ldr r2, [pc, #0xa0c]        @ 0x12c0 <entry+0x12c0>
    c.literal(R::R2,0xbfe30)?;
    // VMA 0xbf420: mov r3, #117
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,117)?)?;
    // VMA 0xbf424: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbf428: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // VMA 0xbf42c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x907a0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf430: ldr r1, [pc, #0x9c4]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf434: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbf438: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x907ac
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf43c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbf440: b 0xbf474 <entry+0x904>   @ imm = #0x2c
    c.b(C::Al,0xbf474,false)?;
    // VMA 0xbf444: orr r1, r1, #72
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,72)?)?;
    // VMA 0xbf448: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xbf44c: ldr r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbf450: ldr r1, [pc, #0x9dc]        @ 0x12c4 <entry+0x12c4>
    c.literal(R::R1,0xbfe34)?;
    // VMA 0xbf454: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbf458: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x907cc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf45c: ldr r1, [pc, #0x998]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf460: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbf464: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x907d8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf468: ldr r0, [r10, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,456)?)?;
    // VMA 0xbf46c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xbf470: str r0, [r10, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,456)?)?;
    // VMA 0xbf474: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbf478: mov r7, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,52)?)?;
    // VMA 0xbf47c: mov r2, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2096)?)?;
    // VMA 0xbf480: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xbf484: orr r7, r7, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2048)?)?;
    // VMA 0xbf488: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // VMA 0xbf48c: orr r2, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbf490: orr r7, r7, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbf494: ldr r2, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbf498: ldr r3, [r7, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbf49c: adds r3, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,0,true)?)?;
    // VMA 0xbf4a0: beq 0xbf4dc <entry+0x96c>   @ imm = #0x34
    c.b(C::Eq,0xbf4dc,false)?;
    // VMA 0xbf4a4: ands r2, r3, #7
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::new(3)?,7,true)?)?;
    // VMA 0xbf4a8: bne 0xbf4dc <entry+0x96c>   @ imm = #0x2c
    c.b(C::Ne,0xbf4dc,false)?;
    // VMA 0xbf4ac: lsr r3, r3, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsr,3)?)?;
    // VMA 0xbf4b0: lsl r2, r1, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsl,15)?)?;
    // VMA 0xbf4b4: str r3, [r10, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,440)?)?;
    // VMA 0xbf4b8: ldr r3, [pc, #0x958]        @ 0x12a8 <entry+0x12a8>
    c.literal(R::new(3)?,0xbfe18)?;
    // VMA 0xbf4bc: ldr r1, [r3, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(3)?,R::R1,Shift::Lsl,2)?)?;
    // VMA 0xbf4c0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbf4c4: beq 0xbf7ec <entry+0xc7c>   @ imm = #0x320
    c.b(C::Eq,0xbf7ec,false)?;
    // VMA 0xbf4c8: orr r0, r2, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,1040)?)?;
    // VMA 0xbf4cc: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbf4d0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbf4d4: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbf4d8: b 0xbf83c <entry+0xccc>   @ imm = #0x35c
    c.b(C::Al,0xbf83c,false)?;
    // VMA 0xbf4dc: ldr r1, [pc, #0x954]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::R1,0xbfe38)?;
    // VMA 0xbf4e0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf4e4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90858
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf4e8: ldr r1, [pc, #0x90c]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf4ec: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf4f0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90864
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf4f4: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbf4f8: mov r3, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,11)?)?;
    // VMA 0xbf4fc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbf500: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xbf504: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // VMA 0xbf508: ldr r0, [r0, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,128)?)?;
    // VMA 0xbf50c: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // VMA 0xbf510: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // VMA 0xbf514: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // VMA 0xbf518: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xbf51c: bl 0x2a990 <entry+0xfffffffffff6be20> @ imm = #-0x94b94
    c.b(C::Al,0x2a990,true)?;
    // VMA 0xbf520: b 0xbf844 <entry+0xcd4>   @ imm = #0x31c
    c.b(C::Al,0xbf844,false)?;
    // VMA 0xbf524: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // VMA 0xbf528: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // VMA 0xbf52c: str r1, [r8, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,468)?)?;
    // VMA 0xbf530: str r2, [r8, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,472)?)?;
    // VMA 0xbf534: str r0, [r4, #0x580]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1408)?)?;
    // VMA 0xbf538: str r6, [r4, #0x584]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,1412)?)?;
    // VMA 0xbf53c: mov r0, #556
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,556)?)?;
    // VMA 0xbf540: mov r6, r10
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(10)?))?;
    // VMA 0xbf544: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbf548: ldrh r0, [r6, r0]!
    c.emit(arm32::load_halfword_register_preindexed_writeback(C::Al,R::R0,R::new(6)?,R::R0)?)?;
    // VMA 0xbf54c: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbf550: bne 0xbf56c <entry+0x9fc>   @ imm = #0x14
    c.b(C::Ne,0xbf56c,false)?;
    // VMA 0xbf554: mov r0, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,46)?)?;
    // VMA 0xbf558: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbf55c: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xbf560: ldrh r0, [r10, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(10)?,R::R0)?)?;
    // VMA 0xbf564: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbf568: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xbf56c: strb r5, [r4, #0x54a]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1354)?)?;
    // VMA 0xbf570: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xbf574: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // VMA 0xbf578: ldrb r0, [r0, #0x61]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,97)?)?;
    // VMA 0xbf57c: and r0, r5, r0, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsr,2,false)?)?;
    // VMA 0xbf580: strb r0, [r4, #0x549]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1353)?)?;
    // VMA 0xbf584: mov r0, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,46)?)?;
    // VMA 0xbf588: strh r1, [r6]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(6)?,0)?)?;
    // VMA 0xbf58c: mov r1, #231
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,231)?)?;
    // VMA 0xbf590: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xbf594: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xbf598: strh r2, [r10, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::new(10)?,R::R0)?)?;
    // VMA 0xbf59c: ldr r2, [pc, #0x860]        @ 0x1294 <entry+0x1294>
    c.literal(R::R2,0xbfe04)?;
    // VMA 0xbf5a0: ldr r0, [r10, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,560)?)?;
    // VMA 0xbf5a4: str r0, [r4, #0x514]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1300)?)?;
    // VMA 0xbf5a8: str r3, [r10, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,560)?)?;
    // VMA 0xbf5ac: ldrb r0, [r10, #0x229]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,553)?)?;
    // VMA 0xbf5b0: str r0, [r4, #0x58c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1420)?)?;
    // VMA 0xbf5b4: ldr r0, [r4, #0x588]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1416)?)?;
    // VMA 0xbf5b8: strb r0, [r10, #0x229]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,553)?)?;
    // VMA 0xbf5bc: strb r12, [r10, #0x374]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(10)?,884)?)?;
    // VMA 0xbf5c0: ldrb r0, [r4, #0x56c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1388)?)?;
    // VMA 0xbf5c4: strb r0, [r10, #0x375]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,885)?)?;
    // VMA 0xbf5c8: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbf5cc: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbde24
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbf5d0: ldr r4, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,676)?)?;
    // VMA 0xbf5d4: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbf5d8: ldr r1, [r4, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2064)?)?;
    // VMA 0xbf5dc: bl 0xc1b78 <entry+0x3008>  @ imm = #0x2594
    c.b(C::Al,0xc1b78,true)?;
    // VMA 0xbf5e0: ldr r0, [r4, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2064)?)?;
    // VMA 0xbf5e4: ldr r2, [r10, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,432)?)?;
    // VMA 0xbf5e8: ldr r3, [r10, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,444)?)?;
    // VMA 0xbf5ec: ldr r1, [pc, #0x814]        @ 0x1298 <entry+0x1298>
    c.literal(R::R1,0xbfe08)?;
    // VMA 0xbf5f0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbf5f4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf5f8: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9096c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf5fc: ldr r1, [pc, #0x7f8]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf600: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf604: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90978
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf608: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbf60c: mov r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2048)?)?;
    // VMA 0xbf610: mov r3, #335544320
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,335544320)?)?;
    // VMA 0xbf614: mov r5, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,120)?)?;
    // VMA 0xbf618: orr r5, r5, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,2048)?)?;
    // VMA 0xbf61c: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // VMA 0xbf620: orr r2, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbf624: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbf628: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbf62c: add r3, r7, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(7)?,1024)?)?;
    // VMA 0xbf630: ldr r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,8)?)?;
    // VMA 0xbf634: orr r0, r5, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xbf638: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // VMA 0xbf63c: ldr r7, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbf640: tst r7, r3
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbf644: beq 0xbf63c <entry+0xacc>   @ imm = #-0x10
    c.b(C::Eq,0xbf63c,false)?;
    // VMA 0xbf648: orr r1, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,4)?)?;
    // VMA 0xbf64c: orr r2, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R1,2048)?)?;
    // VMA 0xbf650: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbf654: ldr r2, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbf658: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xbf65c: blt 0xbf730 <entry+0xbc0>   @ imm = #0xcc
    c.b(C::Lt,0xbf730,false)?;
    // VMA 0xbf660: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbf664: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xbf668: bl 0xc47c8 <entry+0x5c58>  @ imm = #0x5158
    c.b(C::Al,0xc47c8,true)?;
    // VMA 0xbf66c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbf670: strb r0, [r10, #0x220]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,544)?)?;
    // VMA 0xbf674: ldr r2, [r10, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,456)?)?;
    // VMA 0xbf678: add r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1)?)?;
    // VMA 0xbf67c: str r0, [r10, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,456)?)?;
    // VMA 0xbf680: ldr r0, [r4, #0x588]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1416)?)?;
    // VMA 0xbf684: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbf688: beq 0xbf6c0 <entry+0xb50>   @ imm = #0x30
    c.b(C::Eq,0xbf6c0,false)?;
    // VMA 0xbf68c: ldr r1, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,108)?)?;
    // VMA 0xbf690: ldr r3, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,112)?)?;
    // VMA 0xbf694: ldr r0, [r10, #0x378]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,888)?)?;
    // VMA 0xbf698: mul r7, r3, r1
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::new(3)?,R::R1)?)?;
    // VMA 0xbf69c: add r1, r7, r7, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // VMA 0xbf6a0: add r3, r0, r1, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::R1,Shift::Lsr,1,false)?)?;
    // VMA 0xbf6a4: ldr r1, [pc, #0x768]        @ 0x12a4 <entry+0x12a4>
    c.literal(R::R1,0xbfe14)?;
    // VMA 0xbf6a8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf6ac: str r3, [r10, #0x378]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,888)?)?;
    // VMA 0xbf6b0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90a24
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf6b4: ldr r1, [pc, #0x740]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf6b8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf6bc: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90a30
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf6c0: ldrb r1, [r4, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1312)?)?;
    // VMA 0xbf6c4: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xbf6c8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbf6cc: bne 0xbf708 <entry+0xb98>   @ imm = #0x34
    c.b(C::Ne,0xbf708,false)?;
    // VMA 0xbf6d0: mov r3, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,52)?)?;
    // VMA 0xbf6d4: mov r1, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2096)?)?;
    // VMA 0xbf6d8: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xbf6dc: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xbf6e0: orr r1, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbf6e4: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbf6e8: ldr r1, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbf6ec: ldr r2, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbf6f0: adds r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,true)?)?;
    // VMA 0xbf6f4: beq 0xbfdec <entry+0x127c>  @ imm = #0x6f0
    c.b(C::Eq,0xbfdec,false)?;
    // VMA 0xbf6f8: lsr r2, r1, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsr,3)?)?;
    // VMA 0xbf6fc: and r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,7)?)?;
    // VMA 0xbf700: str r2, [r10, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,440)?)?;
    // VMA 0xbf704: str r1, [r4, #0x574]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1396)?)?;
    // VMA 0xbf708: ldr r1, [pc, #0x708]        @ 0x12a8 <entry+0x12a8>
    c.literal(R::R1,0xbfe18)?;
    // VMA 0xbf70c: ldr r1, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbf710: lsl r0, r0, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,15)?)?;
    // VMA 0xbf714: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbf718: beq 0xbf760 <entry+0xbf0>   @ imm = #0x40
    c.b(C::Eq,0xbf760,false)?;
    // VMA 0xbf71c: orr r0, r0, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1040)?)?;
    // VMA 0xbf720: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbf724: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbf728: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbf72c: b 0xbf88c <entry+0xd1c>   @ imm = #0x158
    c.b(C::Al,0xbf88c,false)?;
    // VMA 0xbf730: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbf734: tst r0, #268435457
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,268435457)?)?;
    // VMA 0xbf738: beq 0xbf660 <entry+0xaf0>   @ imm = #-0xe0
    c.b(C::Eq,0xbf660,false)?;
    // VMA 0xbf73c: ldr r1, [pc, #0x6c8]        @ 0x129c <entry+0x129c>
    c.literal(R::R1,0xbfe0c)?;
    // VMA 0xbf740: ldr r2, [pc, #0x6c8]        @ 0x12a0 <entry+0x12a0>
    c.literal(R::R2,0xbfe10)?;
    // VMA 0xbf744: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbf748: mov r3, #1856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1856)?)?;
    // VMA 0xbf74c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90ac0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf750: ldr r1, [pc, #0x6a4]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf754: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbf758: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90acc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf75c: b 0xbf844 <entry+0xcd4>   @ imm = #0xe0
    c.b(C::Al,0xbf844,false)?;
    // VMA 0xbf760: orr r0, r0, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,124)?)?;
    // VMA 0xbf764: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xbf768: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xbf76c: orr r1, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,2048)?)?;
    // VMA 0xbf770: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xbf774: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbf778: mov r2, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,44)?)?;
    // VMA 0xbf77c: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // VMA 0xbf780: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xbf784: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbf788: str r3, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbf78c: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // VMA 0xbf790: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbf794: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbf798: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbf79c: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // VMA 0xbf7a0: orr r1, r5, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(5)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbf7a4: ldr r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbf7a8: tst r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,2)?)?;
    // VMA 0xbf7ac: beq 0xbf7a4 <entry+0xc34>   @ imm = #-0x10
    c.b(C::Eq,0xbf7a4,false)?;
    // VMA 0xbf7b0: b 0xbf88c <entry+0xd1c>   @ imm = #0xd4
    c.b(C::Al,0xbf88c,false)?;
    // VMA 0xbf7b4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbf7b8: mov r2, #3792
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3792)?)?;
    // VMA 0xbf7bc: bl 0x31ec <entry+0xfffffffffff4467c> @ imm = #-0xbc5d8
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xbf7c0: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,0)?)?;
    // VMA 0xbf7c4: ldr r0, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,676)?)?;
    // VMA 0xbf7c8: ldr r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,8)?)?;
    // VMA 0xbf7cc: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // VMA 0xbf7d0: ldr r1, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,12)?)?;
    // VMA 0xbf7d4: str r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // VMA 0xbf7d8: ldr r1, [r10, #0x2a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,672)?)?;
    // VMA 0xbf7dc: str r1, [r0, #0x4f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1268)?)?;
    // VMA 0xbf7e0: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbf7e4: str r1, [r10, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,348)?)?;
    // VMA 0xbf7e8: b 0xbeff0 <entry+0x480>   @ imm = #-0x800
    c.b(C::Al,0xbeff0,false)?;
    // VMA 0xbf7ec: orr r1, r2, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,124)?)?;
    // VMA 0xbf7f0: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // VMA 0xbf7f4: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbf7f8: orr r2, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R1,2048)?)?;
    // VMA 0xbf7fc: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbf800: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbf804: mov r3, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,44)?)?;
    // VMA 0xbf808: ldr r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,8)?)?;
    // VMA 0xbf80c: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xbf810: orr r2, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xbf814: str r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbf818: ldr r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,8)?)?;
    // VMA 0xbf81c: orr r2, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xbf820: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xbf824: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbf828: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // VMA 0xbf82c: orr r0, r4, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(4)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbf830: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbf834: tst r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,2)?)?;
    // VMA 0xbf838: beq 0xbf830 <entry+0xcc0>   @ imm = #-0x10
    c.b(C::Eq,0xbf830,false)?;
    // VMA 0xbf83c: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // VMA 0xbf840: beq 0xbf88c <entry+0xd1c>   @ imm = #0x44
    c.b(C::Eq,0xbf88c,false)?;
    // VMA 0xbf844: ldr r1, [pc, #0x5d4]        @ 0x12b0 <entry+0x12b0>
    c.literal(R::R1,0xbfe20)?;
    // VMA 0xbf848: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf84c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90bc0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf850: ldr r1, [pc, #0x5a4]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf854: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf858: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90bcc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf85c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf860: str r0, [r10, #0x240]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,576)?)?;
    // VMA 0xbf864: ldr r1, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,676)?)?;
    // VMA 0xbf868: ldr r0, [r1, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,1276)?)?;
    // VMA 0xbf86c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbf870: beq 0xbf880 <entry+0xd10>   @ imm = #0x8
    c.b(C::Eq,0xbf880,false)?;
    // VMA 0xbf874: bl 0x7fed0 <entry+0xfffffffffffc1360> @ imm = #-0x3f9ac
    c.b(C::Al,0x7fed0,true)?;
    // VMA 0xbf878: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // VMA 0xbf87c: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // VMA 0xbf880: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbf884: str r0, [r1, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,1272)?)?;
    // VMA 0xbf888: str r0, [r1, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,1276)?)?;
    // VMA 0xbf88c: ldr r2, [r10, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,456)?)?;
    // VMA 0xbf890: ldr r1, [pc, #0x5a4]        @ 0x12cc <entry+0x12cc>
    c.literal(R::R1,0xbfe3c)?;
    // VMA 0xbf894: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf898: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90c0c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf89c: ldr r1, [pc, #0x558]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf8a0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf8a4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90c18
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf8a8: ldr r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // VMA 0xbf8ac: ldr r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // VMA 0xbf8b0: ldr r0, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,676)?)?;
    // VMA 0xbf8b4: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xbf8b8: bl 0x27840 <entry+0xfffffffffff68cd0> @ imm = #-0x98080
    c.b(C::Al,0x27840,true)?;
    // VMA 0xbf8bc: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xbf8c0: str r0, [r10, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,348)?)?;
    // VMA 0xbf8c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbf8c8: str r0, [r10, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,464)?)?;
    // VMA 0xbf8cc: str r0, [r10, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,472)?)?;
    // VMA 0xbf8d0: ldr r3, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,44)?)?;
    // VMA 0xbf8d4: ldr r1, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,676)?)?;
    // VMA 0xbf8d8: ldr r2, [r3, #0xa98]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,2712)?)?;
    // VMA 0xbf8dc: str r2, [r1, #0x818]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,2072)?)?;
    // VMA 0xbf8e0: str r0, [r3, #0xa98]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,2712)?)?;
    // VMA 0xbf8e4: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbf8e8: bl 0xc6e54 <entry+0x82e4>  @ imm = #0x7564
    c.b(C::Al,0xc6e54,true)?;
    // VMA 0xbf8ec: ldr r7, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,676)?)?;
    // VMA 0xbf8f0: ldr r0, [r7, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,80)?)?;
    // VMA 0xbf8f4: lsl r0, r0, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,3)?)?;
    // VMA 0xbf8f8: str r0, [r7, #0x370]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,880)?)?;
    // VMA 0xbf8fc: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbf900: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xbf904: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xbf908: bne 0xbf920 <entry+0xdb0>   @ imm = #0x10
    c.b(C::Ne,0xbf920,false)?;
    // VMA 0xbf90c: ldrb r0, [r7, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,1312)?)?;
    // VMA 0xbf910: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbf914: ldrne r0, [r7, #0x20]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(7)?,32)?)?;
    // VMA 0xbf918: lslne r0, r0, #3
    c.emit(arm32::mov_shifted_register(C::Ne,R::R0,R::R0,Shift::Lsl,3)?)?;
    // VMA 0xbf91c: strne r0, [r7, #0x370]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(7)?,880)?)?;
    // VMA 0xbf920: ldr r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // VMA 0xbf924: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xbf928: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xbf92c: bne 0xbfbb0 <entry+0x1040>  @ imm = #0x27c
    c.b(C::Ne,0xbfbb0,false)?;
    // VMA 0xbf930: ldr r4, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,48)?)?;
    // VMA 0xbf934: ldr r7, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-44)?)?;
    // VMA 0xbf938: ldr r2, [pc, #0x4c4]        @ 0x1294 <entry+0x1294>
    c.literal(R::R2,0xbfe04)?;
    // VMA 0xbf93c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbf940: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xbf944: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbf948: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf94c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbf950: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbf954: and r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbf958: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // VMA 0xbf95c: ldr r8, [r1, #0x13c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R1,316)?)?;
    // VMA 0xbf960: mov r1, #238
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,238)?)?;
    // VMA 0xbf964: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xbf968: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbe1c0
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbf96c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbf970: ldr r9, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(10)?,676)?)?;
    // VMA 0xbf974: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // VMA 0xbf978: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbf97c: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xbf980: str r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-32)?)?;
    // VMA 0xbf984: str r5, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-36)?)?;
    // VMA 0xbf988: str r5, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-40)?)?;
    // VMA 0xbf98c: ldrb r0, [r6], #88
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::R0,R::new(6)?,88)?)?;
    // VMA 0xbf990: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // VMA 0xbf994: and r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,127)?)?;
    // VMA 0xbf998: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbf99c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbf9a0: bl 0xbed64 <entry+0x1f4>   @ imm = #-0xc44
    c.b(C::Al,0xbed64,true)?;
    // VMA 0xbf9a4: add r0, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,16)?)?;
    // VMA 0xbf9a8: sub r1, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,40)?)?;
    // VMA 0xbf9ac: ldrsb r2, [r6]
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::new(6)?,0)?)?;
    // VMA 0xbf9b0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xbf9b4: blt 0xbf9e8 <entry+0xe78>   @ imm = #0x2c
    c.b(C::Lt,0xbf9e8,false)?;
    // VMA 0xbf9b8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xbf9bc: ldrb r7, [r0, r3]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::new(7)?,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbf9c0: cmp r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,255)?)?;
    // VMA 0xbf9c4: beq 0xbf9d4 <entry+0xe64>   @ imm = #0x8
    c.b(C::Eq,0xbf9d4,false)?;
    // VMA 0xbf9c8: eor r7, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Eor,R::new(7)?,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbf9cc: ands r7, r7, #127
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(7)?,R::new(7)?,127,true)?)?;
    // VMA 0xbf9d0: beq 0xbf9e4 <entry+0xe74>   @ imm = #0xc
    c.b(C::Eq,0xbf9e4,false)?;
    // VMA 0xbf9d4: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xbf9d8: cmp r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8)?)?;
    // VMA 0xbf9dc: blo 0xbf9bc <entry+0xe4c>   @ imm = #-0x28
    c.b(C::Cc,0xbf9bc,false)?;
    // VMA 0xbf9e0: b 0xbf9e8 <entry+0xe78>   @ imm = #0x0
    c.b(C::Al,0xbf9e8,false)?;
    // VMA 0xbf9e4: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // VMA 0xbf9e8: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xbf9ec: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xbf9f0: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // VMA 0xbf9f4: cmp r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,3)?)?;
    // VMA 0xbf9f8: bne 0xbf9ac <entry+0xe3c>   @ imm = #-0x54
    c.b(C::Ne,0xbf9ac,false)?;
    // VMA 0xbf9fc: add r0, r9, #188
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbfa00: str r10, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,40)?)?;
    // VMA 0xbfa04: add r7, r9, #196
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(9)?,196)?)?;
    // VMA 0xbfa08: add r4, r9, #124
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(9)?,124)?)?;
    // VMA 0xbfa0c: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // VMA 0xbfa10: sub r8, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,40)?)?;
    // VMA 0xbfa14: add r6, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,3072)?)?;
    // VMA 0xbfa18: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // VMA 0xbfa1c: add r5, r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,88)?)?;
    // VMA 0xbfa20: ldrsb r0, [r5]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::new(5)?,0)?)?;
    // VMA 0xbfa24: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbfa28: blt 0xbfadc <entry+0xf6c>   @ imm = #0xac
    c.b(C::Lt,0xbfadc,false)?;
    // VMA 0xbfa2c: ldr r0, [r9, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1296)?)?;
    // VMA 0xbfa30: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbfa34: beq 0xbfadc <entry+0xf6c>   @ imm = #0xa0
    c.b(C::Eq,0xbfadc,false)?;
    // VMA 0xbfa38: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // VMA 0xbfa3c: ldr r3, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,48)?)?;
    // VMA 0xbfa40: add r0, r3, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xbfa44: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // VMA 0xbfa48: ldr r2, [r1, #0x18]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R2,R::R1,24)?)?;
    // VMA 0xbfa4c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xbfa50: addeq r1, r3, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::new(3)?,4)?)?;
    // VMA 0xbfa54: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xbfa58: str r1, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,4)?)?;
    // VMA 0xbfa5c: ldr r1, [r0, #0x38]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,56)?)?;
    // VMA 0xbfa60: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbfa64: addeq r0, r3, #8
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::new(3)?,8)?)?;
    // VMA 0xbfa68: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xbfa6c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xbfa70: ldrb r0, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xbfa74: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // VMA 0xbfa78: and r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,127)?)?;
    // VMA 0xbfa7c: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbfa80: ldr r1, [r9, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,176)?)?;
    // VMA 0xbfa84: add r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbfa88: str r1, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,16)?)?;
    // VMA 0xbfa8c: ldr r0, [r9, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,152)?)?;
    // VMA 0xbfa90: ldr r2, [r9, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,180)?)?;
    // VMA 0xbfa94: add r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbfa98: add r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbfa9c: str r1, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,20)?)?;
    // VMA 0xbfaa0: ldr r1, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,188)?)?;
    // VMA 0xbfaa4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbfaa8: beq 0xbfb18 <entry+0xfa8>   @ imm = #0x68
    c.b(C::Eq,0xbfb18,false)?;
    // VMA 0xbfaac: ldrb r1, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbfab0: ldr r2, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // VMA 0xbfab4: and r1, r1, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,127)?)?;
    // VMA 0xbfab8: ldr r1, [r2, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,2)?)?;
    // VMA 0xbfabc: str r1, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,24)?)?;
    // VMA 0xbfac0: ldrb r1, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbfac4: and r1, r1, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,127)?)?;
    // VMA 0xbfac8: ldr r1, [r2, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,2)?)?;
    // VMA 0xbfacc: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbfad0: ldr r1, [r9, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,176)?)?;
    // VMA 0xbfad4: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbfad8: b 0xbfb14 <entry+0xfa4>   @ imm = #0x34
    c.b(C::Al,0xbfb14,false)?;
    // VMA 0xbfadc: ldr r0, [r9, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,112)?)?;
    // VMA 0xbfae0: str r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // VMA 0xbfae4: ldr r0, [r9, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,108)?)?;
    // VMA 0xbfae8: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xbfaec: ldr r0, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,296)?)?;
    // VMA 0xbfaf0: str r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xbfaf4: ldr r0, [r9, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,300)?)?;
    // VMA 0xbfaf8: str r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xbfafc: ldr r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbfb00: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbfb04: beq 0xbfb18 <entry+0xfa8>   @ imm = #0xc
    c.b(C::Eq,0xbfb18,false)?;
    // VMA 0xbfb08: ldr r0, [r9, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,304)?)?;
    // VMA 0xbfb0c: str r0, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,24)?)?;
    // VMA 0xbfb10: ldr r0, [r9, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,308)?)?;
    // VMA 0xbfb14: str r0, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,28)?)?;
    // VMA 0xbfb18: add r0, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,32)?)?;
    // VMA 0xbfb1c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xbfb20: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // VMA 0xbfb24: bl 0x471c <entry+0xfffffffffff45bac> @ imm = #-0xbb410
    c.b(C::Al,0x471c,true)?;
    // VMA 0xbfb28: add r0, r6, #104
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,104)?)?;
    // VMA 0xbfb2c: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbfb30: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // VMA 0xbfb34: bl 0x471c <entry+0xfffffffffff45bac> @ imm = #-0xbb420
    c.b(C::Al,0x471c,true)?;
    // VMA 0xbfb38: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // VMA 0xbfb3c: add r8, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,4)?)?;
    // VMA 0xbfb40: add r6, r6, #176
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,176)?)?;
    // VMA 0xbfb44: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xbfb48: cmp r10, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,3)?)?;
    // VMA 0xbfb4c: bne 0xbfa20 <entry+0xeb0>   @ imm = #-0x134
    c.b(C::Ne,0xbfa20,false)?;
    // VMA 0xbfb50: add r0, r9, #972
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,972)?)?;
    // VMA 0xbfb54: add r5, r9, #3264
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(9)?,3264)?)?;
    // VMA 0xbfb58: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xbfb5c: add r7, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,1024)?)?;
    // VMA 0xbfb60: mov r4, r7
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(7)?))?;
    // VMA 0xbfb64: ldr r0, [r9, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,108)?)?;
    // VMA 0xbfb68: ldr r1, [r9, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,112)?)?;
    // VMA 0xbfb6c: ldr r3, [r5, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,-4)?)?;
    // VMA 0xbfb70: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // VMA 0xbfb74: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xbfb78: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xbfb7c: sub r0, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,4)?)?;
    // VMA 0xbfb80: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbfb84: bl 0xc420c <entry+0x569c>  @ imm = #0x4680
    c.b(C::Al,0xc420c,true)?;
    // VMA 0xbfb88: ldr r1, [r4], #8
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbfb8c: ldr r0, [r7, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,-4)?)?;
    // VMA 0xbfb90: bl 0xc41e4 <entry+0x5674>  @ imm = #0x464c
    c.b(C::Al,0xc41e4,true)?;
    // VMA 0xbfb94: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xbfb98: add r5, r5, #176
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,176)?)?;
    // VMA 0xbfb9c: mov r7, r4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(4)?))?;
    // VMA 0xbfba0: cmp r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,3)?)?;
    // VMA 0xbfba4: bne 0xbfb64 <entry+0xff4>   @ imm = #-0x48
    c.b(C::Ne,0xbfb64,false)?;
    // VMA 0xbfba8: ldr r10, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,40)?)?;
    // VMA 0xbfbac: b 0xbfce8 <entry+0x1178>  @ imm = #0x134
    c.b(C::Al,0xbfce8,false)?;
    // VMA 0xbfbb0: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // VMA 0xbfbb4: ldr r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,28)?)?;
    // VMA 0xbfbb8: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // VMA 0xbfbbc: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // VMA 0xbfbc0: ldr r2, [r0, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,1272)?)?;
    // VMA 0xbfbc4: ldrb r0, [r4, #0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // VMA 0xbfbc8: and r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,127)?)?;
    // VMA 0xbfbcc: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbfbd0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbfbd4: bl 0xbed64 <entry+0x1f4>   @ imm = #-0xe78
    c.b(C::Al,0xbed64,true)?;
    // VMA 0xbfbd8: ldrb r0, [r4, #0x30]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // VMA 0xbfbdc: add r8, r7, #196
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(7)?,196)?)?;
    // VMA 0xbfbe0: add r9, r7, #124
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(7)?,124)?)?;
    // VMA 0xbfbe4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbfbe8: sub r6, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,40)?)?;
    // VMA 0xbfbec: and r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,127)?)?;
    // VMA 0xbfbf0: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // VMA 0xbfbf4: ldrb r0, [r4, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,47)?)?;
    // VMA 0xbfbf8: and r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,127)?)?;
    // VMA 0xbfbfc: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // VMA 0xbfc00: ldrb r0, [r4, #0x2e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,46)?)?;
    // VMA 0xbfc04: and r0, r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,127)?)?;
    // VMA 0xbfc08: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xbfc0c: add r0, r7, #292
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,292)?)?;
    // VMA 0xbfc10: add r4, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,3072)?)?;
    // VMA 0xbfc14: ldr r1, [r7, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,112)?)?;
    // VMA 0xbfc18: ldr r0, [r7, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,1296)?)?;
    // VMA 0xbfc1c: str r1, [r4, #-0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,-100)?)?;
    // VMA 0xbfc20: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbfc24: ldr r1, [r7, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,108)?)?;
    // VMA 0xbfc28: str r1, [r4, #-0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,-104)?)?;
    // VMA 0xbfc2c: beq 0xbfc88 <entry+0x1118>  @ imm = #0x54
    c.b(C::Eq,0xbfc88,false)?;
    // VMA 0xbfc30: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xbfc34: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // VMA 0xbfc38: ldr r2, [r7, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,176)?)?;
    // VMA 0xbfc3c: ldr r1, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbfc40: add r2, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbfc44: str r2, [r4, #-0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,-88)?)?;
    // VMA 0xbfc48: ldr r1, [r7, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,152)?)?;
    // VMA 0xbfc4c: ldr r3, [r7, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,180)?)?;
    // VMA 0xbfc50: add r2, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbfc54: add r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbfc58: str r2, [r4, #-0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,-84)?)?;
    // VMA 0xbfc5c: ldr r2, [r7, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,188)?)?;
    // VMA 0xbfc60: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xbfc64: beq 0xbfcb4 <entry+0x1144>  @ imm = #0x48
    c.b(C::Eq,0xbfcb4,false)?;
    // VMA 0xbfc68: ldr r3, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-44)?)?;
    // VMA 0xbfc6c: ldr r2, [r3, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(3)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbfc70: str r2, [r4, #-0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,-80)?)?;
    // VMA 0xbfc74: ldr r0, [r3, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(3)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbfc78: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbfc7c: ldr r1, [r7, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,176)?)?;
    // VMA 0xbfc80: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbfc84: b 0xbfcb0 <entry+0x1140>  @ imm = #0x24
    c.b(C::Al,0xbfcb0,false)?;
    // VMA 0xbfc88: ldr r0, [r7, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,296)?)?;
    // VMA 0xbfc8c: str r0, [r4, #-0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,-88)?)?;
    // VMA 0xbfc90: ldr r0, [r7, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,300)?)?;
    // VMA 0xbfc94: str r0, [r4, #-0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,-84)?)?;
    // VMA 0xbfc98: ldr r0, [r7, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,188)?)?;
    // VMA 0xbfc9c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbfca0: beq 0xbfcb4 <entry+0x1144>  @ imm = #0xc
    c.b(C::Eq,0xbfcb4,false)?;
    // VMA 0xbfca4: ldr r0, [r7, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,304)?)?;
    // VMA 0xbfca8: str r0, [r4, #-0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,-80)?)?;
    // VMA 0xbfcac: ldr r0, [r7, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,308)?)?;
    // VMA 0xbfcb0: str r0, [r4, #-0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,-76)?)?;
    // VMA 0xbfcb4: sub r0, r4, #72
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(4)?,72)?)?;
    // VMA 0xbfcb8: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xbfcbc: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // VMA 0xbfcc0: bl 0x471c <entry+0xfffffffffff45bac> @ imm = #-0xbb5ac
    c.b(C::Al,0x471c,true)?;
    // VMA 0xbfcc4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xbfcc8: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xbfccc: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // VMA 0xbfcd0: bl 0x471c <entry+0xfffffffffff45bac> @ imm = #-0xbb5bc
    c.b(C::Al,0x471c,true)?;
    // VMA 0xbfcd4: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xbfcd8: add r6, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,4)?)?;
    // VMA 0xbfcdc: add r4, r4, #176
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,176)?)?;
    // VMA 0xbfce0: cmp r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,3)?)?;
    // VMA 0xbfce4: bne 0xbfc14 <entry+0x10a4>  @ imm = #-0xd8
    c.b(C::Ne,0xbfc14,false)?;
    // VMA 0xbfce8: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xbfcec: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // VMA 0xbfcf0: add r0, r4, #280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,280)?)?;
    // VMA 0xbfcf4: add r1, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,2048)?)?;
    // VMA 0xbfcf8: ldr r0, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,676)?)?;
    // VMA 0xbfcfc: add r0, r0, #400
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,400)?)?;
    // VMA 0xbfd00: bl 0x471c <entry+0xfffffffffff45bac> @ imm = #-0xbb5ec
    c.b(C::Al,0x471c,true)?;
    // VMA 0xbfd04: ldr r0, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,676)?)?;
    // VMA 0xbfd08: ldr r1, [r4, #0x910]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2320)?)?;
    // VMA 0xbfd0c: str r1, [r0, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,396)?)?;
    // VMA 0xbfd10: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,0)?)?;
    // VMA 0xbfd14: ldr r2, [r1, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,992)?)?;
    // VMA 0xbfd18: str r2, [r0, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,268)?)?;
    // VMA 0xbfd1c: ldr r1, [r1, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,996)?)?;
    // VMA 0xbfd20: str r1, [r0, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,272)?)?;
    // VMA 0xbfd24: ldr r1, [r10, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,640)?)?;
    // VMA 0xbfd28: str r1, [r0, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2028)?)?;
    // VMA 0xbfd2c: ldr r0, [r10, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,644)?)?;
    // VMA 0xbfd30: ldr r1, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,676)?)?;
    // VMA 0xbfd34: str r0, [r1, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2032)?)?;
    // VMA 0xbfd38: ldr r0, [r10, #0x288]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,648)?)?;
    // VMA 0xbfd3c: ldr r1, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,676)?)?;
    // VMA 0xbfd40: str r0, [r1, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2036)?)?;
    // VMA 0xbfd44: ldr r1, [pc, #0xf4]         @ 0x12d0 <entry+0x12d0>
    c.literal(R::R1,0xbfe40)?;
    // VMA 0xbfd48: ldr r0, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,676)?)?;
    // VMA 0xbfd4c: ldr r2, [r0, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,88)?)?;
    // VMA 0xbfd50: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbfd54: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x910c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbfd58: ldr r1, [pc, #0x9c]         @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbfd5c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbfd60: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x910d4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbfd64: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // VMA 0xbfd68: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // VMA 0xbfd6c: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // VMA 0xbfd70: ldr r0, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,676)?)?;
    // VMA 0xbfd74: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xbfd78: mov r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // VMA 0xbfd7c: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xbfd80: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbfd84: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // VMA 0xbfd88: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xbfd8c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xbfd90: ldr r0, [pc, #0xac]         @ 0x12d4 <entry+0x12d4>
    c.literal(R::R0,0xbfe44)?;
    // VMA 0xbfd94: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbfd98: sub r1, r11, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,40)?)?;
    // VMA 0xbfd9c: bl 0x80928 <entry+0xfffffffffffc1db8> @ imm = #-0x3f47c
    c.b(C::Al,0x80928,true)?;
    // VMA 0xbfda0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbfda4: beq 0xbfddc <entry+0x126c>  @ imm = #0x30
    c.b(C::Eq,0xbfddc,false)?;
    // VMA 0xbfda8: ldr r1, [pc, #0x98]         @ 0x12d8 <entry+0x12d8>
    c.literal(R::R1,0xbfe48)?;
    // VMA 0xbfdac: ldr r2, [pc, #0x98]         @ 0x12dc <entry+0x12dc>
    c.literal(R::R2,0xbfe4c)?;
    // VMA 0xbfdb0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbfdb4: mov r3, #1168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1168)?)?;
    // VMA 0xbfdb8: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9112c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbfdbc: ldr r1, [pc, #0x38]         @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbfdc0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbfdc4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91138
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbfdc8: ldr r2, [pc, #0x34]         @ 0x1294 <entry+0x1294>
    c.literal(R::R2,0xbfe04)?;
    // VMA 0xbfdcc: mov r1, #145
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,145)?)?;
    // VMA 0xbfdd0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbfdd4: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xbfdd8: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbe630
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbfddc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbfde0: str r0, [r10, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,348)?)?;
    // VMA 0xbfde4: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbfde8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xbfdec: ldr r1, [pc, #0x28]         @ 0x12ac <entry+0x12ac>
    c.literal(R::R1,0xbfe1c)?;
    // VMA 0xbfdf0: b 0xbf4e0 <entry+0x970>   @ imm = #-0x918
    c.b(C::Al,0xbf4e0,false)?;
    // VMA 0xbfdf4: andne r0, r0, r1, lsl #8
    c.emit(arm32::data_register(C::Ne,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,8,false)?)?;
    c.check(0xbfdf8)?;
    const CODEC_OUTPUT_DIAGNOSTIC_REFERENCES:[u32;23]=[
        0x000fa754,0x000e2788,0x000fac3a,0x000fa71c,
        0x000fad07,0x000fad7c,0x000fada4,0x000fad42,
        0x0010408c,0x000fad5e,0x000fa76f,0x000fac1b,
        0x000fac88,0x000faca2,0x000facc6,0x000face1,
        0x000fac5c,0x000fa79a,0x000fa7be,0xffffeb28,
        0x000f539b,0x000fa7e9,0x000fa6f2,
    ];
    for data in CODEC_OUTPUT_DIAGNOSTIC_REFERENCES { c.emit(data)?; }
    c.check(0xbfe54)
}
