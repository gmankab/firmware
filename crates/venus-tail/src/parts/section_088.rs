
/// Continue the codec frame-descriptor state machine through the command
/// scheduling, mode validation and indexed packet handling branches. This
/// is deliberately a partial function: the next literal pool and epilogue
/// follow the checkpoint, so no unknown bytes or fabricated returns appear.
fn continue_codec_descriptor_command_state(c:&mut Code)->Result<(),String>{
    c.check(0xc5000)?;
    // VMA 0xc5000: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc5004: blo 0xc511c <entry+0x11c>   @ imm = #0x110
    c.b(C::Cc,0xc511c,false)?;
    // VMA 0xc5008: ldr r1, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,236)?)?;
    // VMA 0xc500c: ldr r2, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,412)?)?;
    // VMA 0xc5010: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc5014: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc5018: movlo r0, #3
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc501c: b 0xc511c <entry+0x11c>   @ imm = #0xf8
    c.b(C::Al,0xc511c,false)?;
    // VMA 0xc5020: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc5024: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc5028: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc502c: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc5030: andeq r0, r1, r5, lsr #9
    c.emit(arm32::data_register(C::Eq,Op::And,R::R0,R::R1,R::new(5)?,Shift::Lsr,9,false)?)?;
    // VMA 0xc5034: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xc5038: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc503c: andeq r2, r1, r5, lsr #17
    c.emit(arm32::data_register(C::Eq,Op::And,R::R2,R::R1,R::new(5)?,Shift::Lsr,17,false)?)?;
    // VMA 0xc5040: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5044: strb r0, [r10, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1355)?)?;
    // VMA 0xc5048: strb r2, [r10, #0x54c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,1356)?)?;
    // VMA 0xc504c: str r5, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,32)?)?;
    // VMA 0xc5050: beq 0xc519c <entry+0x19c>   @ imm = #0x144
    c.b(C::Eq,0xc519c,false)?;
    // VMA 0xc5054: ldr r2, [r10, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,392)?)?;
    // VMA 0xc5058: ldr r1, [pc, #0xf48]        @ 0xfa8 <entry+0xfa8>
    c.literal(R::R1,0xc5fa8)?;
    // VMA 0xc505c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc5060: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x963d4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5064: ldr r1, [pc, #0xa88]        @ 0xaf4 <entry+0xaf4>
    c.literal(R::R1,0xc5af4)?;
    // VMA 0xc5068: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc506c: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x963e0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5070: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5074: str r0, [r9, #0x370]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,880)?)?;
    // VMA 0xc5078: ldr r0, [r10, #0x50c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1292)?)?;
    // VMA 0xc507c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc5080: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // VMA 0xc5084: moveq r6, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // VMA 0xc5088: str r6, [r0, #0xa90]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,2704)?)?;
    // VMA 0xc508c: ldr r2, [r0, #0xa94]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2708)?)?;
    // VMA 0xc5090: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xc5094: cmp r6, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc5098: beq 0xc50c0 <entry+0xc0>    @ imm = #0x20
    c.b(C::Eq,0xc50c0,false)?;
    // VMA 0xc509c: ldr r1, [pc, #0xc30]        @ 0xcd4 <entry+0xcd4>
    c.literal(R::R1,0xc5cd4)?;
    // VMA 0xc50a0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc50a4: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // VMA 0xc50a8: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x9641c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc50ac: ldr r1, [pc, #0xa40]        @ 0xaf4 <entry+0xaf4>
    c.literal(R::R1,0xc5af4)?;
    // VMA 0xc50b0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc50b4: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96428
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc50b8: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc50bc: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // VMA 0xc50c0: ldr r0, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1296)?)?;
    // VMA 0xc50c4: str r0, [r9, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,560)?)?;
    // VMA 0xc50c8: ldrb r0, [r10, #0x508]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1288)?)?;
    // VMA 0xc50cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc50d0: bne 0xc50e0 <entry+0xe0>    @ imm = #0x8
    c.b(C::Ne,0xc50e0,false)?;
    // VMA 0xc50d4: ldr r0, [r10, #0x50c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1292)?)?;
    // VMA 0xc50d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc50dc: beq 0xc5960 <entry+0x960>   @ imm = #0x87c
    c.b(C::Eq,0xc5960,false)?;
    // VMA 0xc50e0: ldr r5, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,32)?)?;
    // VMA 0xc50e4: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xc50e8: and r0, r0, r5, lsr #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(5)?,Shift::Lsr,5,false)?)?;
    // VMA 0xc50ec: strb r0, [r9, #0x224]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,548)?)?;
    // VMA 0xc50f0: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // VMA 0xc50f4: ldr r6, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,36)?)?;
    // VMA 0xc50f8: bne 0xc5220 <entry+0x220>   @ imm = #0x120
    c.b(C::Ne,0xc5220,false)?;
    // VMA 0xc50fc: ldrb r0, [r9, #0x24c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,588)?)?;
    // VMA 0xc5100: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc5104: bne 0xc5648 <entry+0x648>   @ imm = #0x53c
    c.b(C::Ne,0xc5648,false)?;
    // VMA 0xc5108: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc510c: strb r0, [r9, #0x226]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,550)?)?;
    // VMA 0xc5110: strb r0, [r9, #0x227]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,551)?)?;
    // VMA 0xc5114: b 0xc5584 <entry+0x584>   @ imm = #0x468
    c.b(C::Al,0xc5584,false)?;
    // VMA 0xc5118: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc511c: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // VMA 0xc5120: ldr r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,112)?)?;
    // VMA 0xc5124: str r0, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,468)?)?;
    // VMA 0xc5128: ldr r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,108)?)?;
    // VMA 0xc512c: str r0, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,472)?)?;
    // VMA 0xc5130: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc5134: str r0, [r4, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2700)?)?;
    // VMA 0xc5138: ldr r0, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,468)?)?;
    // VMA 0xc513c: str r0, [r10, #0x580]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1408)?)?;
    // VMA 0xc5140: ldr r0, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,472)?)?;
    // VMA 0xc5144: str r0, [r10, #0x584]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1412)?)?;
    // VMA 0xc5148: ldr r7, [r10, #0x588]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,1416)?)?;
    // VMA 0xc514c: ldr r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc5150: ldr r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,112)?)?;
    // VMA 0xc5154: ldr r1, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1296)?)?;
    // VMA 0xc5158: b 0xc5d98 <entry+0xd98>   @ imm = #0xc38
    c.b(C::Al,0xc5d98,false)?;
    // VMA 0xc515c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5160: and r1, r0, r6, lsr #13
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(6)?,Shift::Lsr,13,false)?)?;
    // VMA 0xc5164: strb r1, [r9, #0x225]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,549)?)?;
    // VMA 0xc5168: ldrb r1, [r9, #0x24c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,588)?)?;
    // VMA 0xc516c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xc5170: bne 0xc5298 <entry+0x298>   @ imm = #0x120
    c.b(C::Ne,0xc5298,false)?;
    // VMA 0xc5174: and r1, r0, r6, lsr #14
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(6)?,Shift::Lsr,14,false)?)?;
    // VMA 0xc5178: and r0, r0, r6, lsr #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsr,15,false)?)?;
    // VMA 0xc517c: tst r0, r6, lsr #14
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(6)?,Shift::Lsr,14,false)?)?;
    // VMA 0xc5180: strb r1, [r9, #0x226]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,550)?)?;
    // VMA 0xc5184: strb r0, [r9, #0x227]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,551)?)?;
    // VMA 0xc5188: beq 0xc5210 <entry+0x210>   @ imm = #0x80
    c.b(C::Eq,0xc5210,false)?;
    // VMA 0xc518c: ldr r1, [pc, #0xfbc]        @ 0x1150 <entry+0x1150>
    c.literal(R::R1,0xc6150)?;
    // VMA 0xc5190: ldr r2, [pc, #0xa38]        @ 0xbd0 <entry+0xbd0>
    c.literal(R::R2,0xc5bd0)?;
    // VMA 0xc5194: mov r3, #147
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,147)?)?;
    // VMA 0xc5198: b 0xc5270 <entry+0x270>   @ imm = #0xd0
    c.b(C::Al,0xc5270,false)?;
    // VMA 0xc519c: lsr r0, r5, #20
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsr,20)?)?;
    // VMA 0xc51a0: and r3, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(7)?,7)?)?;
    // VMA 0xc51a4: strb r0, [r10, #0x54d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1357)?)?;
    // VMA 0xc51a8: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xc51ac: str r3, [r10, #0x554]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,1364)?)?;
    // VMA 0xc51b0: and r1, r0, r7, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(7)?,Shift::Lsr,4,false)?)?;
    // VMA 0xc51b4: and r0, r0, r7, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(7)?,Shift::Lsr,8,false)?)?;
    // VMA 0xc51b8: str r1, [r10, #0x558]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1368)?)?;
    // VMA 0xc51bc: str r0, [r10, #0x55c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1372)?)?;
    // VMA 0xc51c0: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xc51c4: ldr r1, [pc, #0xff0]        @ 0x11bc <entry+0x11bc>
    c.literal(R::R1,0xc61bc)?;
    // VMA 0xc51c8: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xc51cc: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // VMA 0xc51d0: and r2, r0, r5, lsr #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::new(5)?,Shift::Lsr,20,false)?)?;
    // VMA 0xc51d4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc51d8: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x9654c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc51dc: ldr r1, [pc, #0x910]        @ 0xaf4 <entry+0xaf4>
    c.literal(R::R1,0xc5af4)?;
    // VMA 0xc51e0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc51e4: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96558
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc51e8: ldrb r0, [r10, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1355)?)?;
    // VMA 0xc51ec: mov r6, r5
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(5)?))?;
    // VMA 0xc51f0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc51f4: beq 0xc5594 <entry+0x594>   @ imm = #0x398
    c.b(C::Eq,0xc5594,false)?;
    // VMA 0xc51f8: str r10, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2028)?)?;
    // VMA 0xc51fc: str r10, [r10, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2032)?)?;
    // VMA 0xc5200: str r10, [r10, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2036)?)?;
    // VMA 0xc5204: mov r3, r10
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // VMA 0xc5208: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xc520c: b 0xc56d0 <entry+0x6d0>   @ imm = #0x4bc
    c.b(C::Al,0xc56d0,false)?;
    // VMA 0xc5210: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5214: and r0, r0, r6, lsr #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsr,16,false)?)?;
    // VMA 0xc5218: strb r0, [r9, #0x228]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,552)?)?;
    // VMA 0xc521c: b 0xc52a0 <entry+0x2a0>   @ imm = #0x7c
    c.b(C::Al,0xc52a0,false)?;
    // VMA 0xc5220: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5224: and r1, r0, r5, lsr #13
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(5)?,Shift::Lsr,13,false)?)?;
    // VMA 0xc5228: strb r1, [r9, #0x225]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,549)?)?;
    // VMA 0xc522c: ldrb r1, [r9, #0x24c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,588)?)?;
    // VMA 0xc5230: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xc5234: bne 0xc565c <entry+0x65c>   @ imm = #0x420
    c.b(C::Ne,0xc565c,false)?;
    // VMA 0xc5238: and r1, r0, r5, lsr #14
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(5)?,Shift::Lsr,14,false)?)?;
    // VMA 0xc523c: and r0, r0, r5, lsr #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(5)?,Shift::Lsr,15,false)?)?;
    // VMA 0xc5240: tst r0, r5, lsr #14
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(5)?,Shift::Lsr,14,false)?)?;
    // VMA 0xc5244: strb r1, [r9, #0x226]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,550)?)?;
    // VMA 0xc5248: strb r0, [r9, #0x227]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,551)?)?;
    // VMA 0xc524c: beq 0xc5584 <entry+0x584>   @ imm = #0x330
    c.b(C::Eq,0xc5584,false)?;
    // VMA 0xc5250: ldr r1, [pc, #0xef8]        @ 0x1150 <entry+0x1150>
    c.literal(R::R1,0xc6150)?;
    // VMA 0xc5254: ldr r2, [pc, #0x974]        @ 0xbd0 <entry+0xbd0>
    c.literal(R::R2,0xc5bd0)?;
    // VMA 0xc5258: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc525c: mov r3, #1376
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1376)?)?;
    // VMA 0xc5260: b 0xc5ed4 <entry+0xed4>   @ imm = #0xc6c
    c.b(C::Al,0xc5ed4,false)?;
    // VMA 0xc5264: ldr r1, [pc, #0xfe0]        @ 0x124c <entry+0x124c>
    c.literal(R::R1,0xc624c)?;
    // VMA 0xc5268: ldr r2, [pc, #0x960]        @ 0xbd0 <entry+0xbd0>
    c.literal(R::R2,0xc5bd0)?;
    // VMA 0xc526c: mov r3, #171
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,171)?)?;
    // VMA 0xc5270: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // VMA 0xc5274: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc5278: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x965ec
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc527c: ldr r1, [pc, #0x870]        @ 0xaf4 <entry+0xaf4>
    c.literal(R::R1,0xc5af4)?;
    // VMA 0xc5280: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc5284: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x965f8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5288: mov r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,120)?)?;
    // VMA 0xc528c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc5290: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xc5294: b 0xc5ee8 <entry+0xee8>   @ imm = #0xc4c
    c.b(C::Al,0xc5ee8,false)?;
    // VMA 0xc5298: strb r0, [r9, #0x226]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,550)?)?;
    // VMA 0xc529c: strb r0, [r9, #0x227]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,551)?)?;
    // VMA 0xc52a0: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // VMA 0xc52a4: mov r7, #122
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,122)?)?;
    // VMA 0xc52a8: strb r0, [r10, #0x54d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1357)?)?;
    // VMA 0xc52ac: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // VMA 0xc52b0: orr r7, r7, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,1280)?)?;
    // VMA 0xc52b4: ldr r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xc52b8: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // VMA 0xc52bc: and r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc52c0: add r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,1)?)?;
    // VMA 0xc52c4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc52c8: add r3, r0, r1, lsr #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::R1,Shift::Lsr,16,false)?)?;
    // VMA 0xc52cc: and r1, r0, r6, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(6)?,Shift::Lsr,8,false)?)?;
    // VMA 0xc52d0: str r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc52d4: str r2, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,112)?)?;
    // VMA 0xc52d8: strb r1, [r10, #0x578]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,1400)?)?;
    // VMA 0xc52dc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc52e0: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // VMA 0xc52e4: ldr r5, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,36)?)?;
    // VMA 0xc52e8: addne r1, r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::new(5)?,1)?)?;
    // VMA 0xc52ec: strh r1, [r10, r7]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(10)?,R::new(7)?)?)?;
    // VMA 0xc52f0: mov r1, r3
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(3)?))?;
    // VMA 0xc52f4: addne r1, r0, r5, lsr #16
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R1,R::R0,R::new(5)?,Shift::Lsr,16,false)?)?;
    // VMA 0xc52f8: mov r0, #380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,380)?)?;
    // VMA 0xc52fc: orr r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // VMA 0xc5300: strh r1, [r10, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(10)?,R::R0)?)?;
    // VMA 0xc5304: ldr r1, [pc, #0xff8]        @ 0x1304 <entry+0x1304>
    c.literal(R::R1,0xc6304)?;
    // VMA 0xc5308: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc530c: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96680
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5310: ldr r1, [pc, #0xfb0]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc5314: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc5318: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x9668c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc531c: ldr r2, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,108)?)?;
    // VMA 0xc5320: ldr r1, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,112)?)?;
    // VMA 0xc5324: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc5328: bl 0xc46e4 <entry+0xfffffffffffff6e4> @ imm = #-0xc4c
    c.b(C::Al,0xc46e4,true)?;
    // VMA 0xc532c: ldr r0, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,468)?)?;
    // VMA 0xc5330: ldr r1, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,112)?)?;
    // VMA 0xc5334: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc5338: bhi 0xc5394 <entry+0x394>   @ imm = #0x54
    c.b(C::Hi,0xc5394,false)?;
    // VMA 0xc533c: ldr r2, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,472)?)?;
    // VMA 0xc5340: ldr r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc5344: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc5348: bhi 0xc5394 <entry+0x394>   @ imm = #0x44
    c.b(C::Hi,0xc5394,false)?;
    // VMA 0xc534c: ldr r7, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,192)?)?;
    // VMA 0xc5350: ldr r6, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,196)?)?;
    // VMA 0xc5354: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc5358: mov r6, r8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // VMA 0xc535c: blo 0xc5394 <entry+0x394>   @ imm = #0x30
    c.b(C::Cc,0xc5394,false)?;
    // VMA 0xc5360: ldr r7, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,972)?)?;
    // VMA 0xc5364: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc5368: beq 0xc6154 <entry+0x1154>  @ imm = #0xde4
    c.b(C::Eq,0xc6154,false)?;
    // VMA 0xc536c: ldr r7, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,248)?)?;
    // VMA 0xc5370: ldr r6, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,416)?)?;
    // VMA 0xc5374: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc5378: mov r6, r8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // VMA 0xc537c: blo 0xc5394 <entry+0x394>   @ imm = #0x10
    c.b(C::Cc,0xc5394,false)?;
    // VMA 0xc5380: ldr r7, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,236)?)?;
    // VMA 0xc5384: ldr r6, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,412)?)?;
    // VMA 0xc5388: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc538c: mov r6, r8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // VMA 0xc5390: bhs 0xc6154 <entry+0x1154>  @ imm = #0xdbc
    c.b(C::Cs,0xc6154,false)?;
    // VMA 0xc5394: ldr r1, [pc, #0xf74]        @ 0x1310 <entry+0x1310>
    c.literal(R::R1,0xc6310)?;
    // VMA 0xc5398: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc539c: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96710
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc53a0: ldr r1, [pc, #0xf20]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc53a4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc53a8: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x9671c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc53ac: mov r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,120)?)?;
    // VMA 0xc53b0: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc53b4: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // VMA 0xc53b8: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xc53bc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc53c0: str r0, [r4, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2700)?)?;
    // VMA 0xc53c4: ldr r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,112)?)?;
    // VMA 0xc53c8: lsr r1, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Lsr,4)?)?;
    // VMA 0xc53cc: str r1, [r4, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,484)?)?;
    // VMA 0xc53d0: ldr r1, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,108)?)?;
    // VMA 0xc53d4: lsr r2, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsr,4)?)?;
    // VMA 0xc53d8: str r2, [r4, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,488)?)?;
    // VMA 0xc53dc: str r0, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,476)?)?;
    // VMA 0xc53e0: str r1, [r4, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,480)?)?;
    // VMA 0xc53e4: str r0, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,468)?)?;
    // VMA 0xc53e8: str r1, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,472)?)?;
    // VMA 0xc53ec: str r0, [r10, #0x580]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1408)?)?;
    // VMA 0xc53f0: str r1, [r10, #0x584]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1412)?)?;
    // VMA 0xc53f4: str r10, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2028)?)?;
    // VMA 0xc53f8: str r10, [r10, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2032)?)?;
    // VMA 0xc53fc: str r10, [r10, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2036)?)?;
    // VMA 0xc5400: b 0xc5b70 <entry+0xb70>   @ imm = #0x768
    c.b(C::Al,0xc5b70,false)?;
    // VMA 0xc5404: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc5408: cmphs r3, r2
    c.emit(arm32::data_register(C::Cs,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc540c: bhs 0xc5790 <entry+0x790>   @ imm = #0x37c
    c.b(C::Cs,0xc5790,false)?;
    // VMA 0xc5410: ldr r0, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,140)?)?;
    // VMA 0xc5414: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5418: beq 0xc5428 <entry+0x428>   @ imm = #0x8
    c.b(C::Eq,0xc5428,false)?;
    // VMA 0xc541c: ldr r0, [r4, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,456)?)?;
    // VMA 0xc5420: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5424: beq 0xc6130 <entry+0x1130>  @ imm = #0xd04
    c.b(C::Eq,0xc6130,false)?;
    // VMA 0xc5428: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,144)?)?;
    // VMA 0xc542c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5430: beq 0xc5440 <entry+0x440>   @ imm = #0x8
    c.b(C::Eq,0xc5440,false)?;
    // VMA 0xc5434: ldr r0, [r4, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,452)?)?;
    // VMA 0xc5438: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc543c: beq 0xc6130 <entry+0x1130>  @ imm = #0xcec
    c.b(C::Eq,0xc6130,false)?;
    // VMA 0xc5440: ldr r1, [pc, #0xe8c]        @ 0x12d4 <entry+0x12d4>
    c.literal(R::R1,0xc62d4)?;
    // VMA 0xc5444: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc5448: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x967bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc544c: ldr r1, [pc, #0xe74]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc5450: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc5454: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x967c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5458: ldr r0, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xc545c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5460: beq 0xc5130 <entry+0x130>   @ imm = #-0x338
    c.b(C::Eq,0xc5130,false)?;
    // VMA 0xc5464: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5468: add r5, sp, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,48)?)?;
    // VMA 0xc546c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc5470: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // VMA 0xc5474: str r0, [r10, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,384)?)?;
    // VMA 0xc5478: ldr r0, [r4, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,992)?)?;
    // VMA 0xc547c: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // VMA 0xc5480: ldr r0, [r4, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,996)?)?;
    // VMA 0xc5484: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // VMA 0xc5488: ldr r0, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,472)?)?;
    // VMA 0xc548c: ldr r6, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,468)?)?;
    // VMA 0xc5490: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // VMA 0xc5494: ldr r0, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,476)?)?;
    // VMA 0xc5498: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xc549c: ldr r0, [r4, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,480)?)?;
    // VMA 0xc54a0: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xc54a4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc54a8: bl 0x31ec <entry+0xfffffffffff3e1ec> @ imm = #-0xc22c4
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xc54ac: ldr r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,112)?)?;
    // VMA 0xc54b0: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // VMA 0xc54b4: str r0, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,468)?)?;
    // VMA 0xc54b8: ldr r1, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,108)?)?;
    // VMA 0xc54bc: str r1, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,472)?)?;
    // VMA 0xc54c0: str r0, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,476)?)?;
    // VMA 0xc54c4: add r0, r4, #280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,280)?)?;
    // VMA 0xc54c8: str r1, [r4, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,480)?)?;
    // VMA 0xc54cc: add r7, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,2048)?)?;
    // VMA 0xc54d0: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc54d4: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xc54d8: bl 0x471c <entry+0xfffffffffff3f71c> @ imm = #-0xc0dc4
    c.b(C::Al,0x471c,true)?;
    // VMA 0xc54dc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc54e0: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // VMA 0xc54e4: bl 0xd2adc <entry+0xdadc>  @ imm = #0xd5f0
    c.b(C::Al,0xd2adc,true)?;
    // VMA 0xc54e8: mov r1, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,19)?)?;
    // VMA 0xc54ec: orr r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // VMA 0xc54f0: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc54f4: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc54f8: bne 0xc5520 <entry+0x520>   @ imm = #0x20
    c.b(C::Ne,0xc5520,false)?;
    // VMA 0xc54fc: ldr r1, [pc, #0xdd4]        @ 0x12d8 <entry+0x12d8>
    c.literal(R::R1,0xc62d8)?;
    // VMA 0xc5500: ldr r2, [pc, #0xdc4]        @ 0x12cc <entry+0x12cc>
    c.literal(R::R2,0xc62cc)?;
    // VMA 0xc5504: mov r3, #42
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,42)?)?;
    // VMA 0xc5508: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc550c: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // VMA 0xc5510: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96884
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5514: ldr r1, [pc, #0xdac]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc5518: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc551c: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96890
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5520: add r0, r10, #400
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,400)?)?;
    // VMA 0xc5524: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xc5528: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // VMA 0xc552c: bl 0x471c <entry+0xfffffffffff3f71c> @ imm = #-0xc0e18
    c.b(C::Al,0x471c,true)?;
    // VMA 0xc5530: mov r4, r5
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // VMA 0xc5534: add r1, sp, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,48)?)?;
    // VMA 0xc5538: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // VMA 0xc553c: ldr r0, [r4, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,992)?)?;
    // VMA 0xc5540: str r0, [r10, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,268)?)?;
    // VMA 0xc5544: ldr r0, [r4, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,996)?)?;
    // VMA 0xc5548: str r0, [r10, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,272)?)?;
    // VMA 0xc554c: str r6, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,468)?)?;
    // VMA 0xc5550: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // VMA 0xc5554: str r0, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,472)?)?;
    // VMA 0xc5558: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xc555c: str r0, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,476)?)?;
    // VMA 0xc5560: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xc5564: str r0, [r4, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,480)?)?;
    // VMA 0xc5568: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // VMA 0xc556c: str r0, [r4, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,992)?)?;
    // VMA 0xc5570: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xc5574: str r0, [r4, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,996)?)?;
    // VMA 0xc5578: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xc557c: bl 0x471c <entry+0xfffffffffff3f71c> @ imm = #-0xc0e68
    c.b(C::Al,0x471c,true)?;
    // VMA 0xc5580: b 0xc5130 <entry+0x130>   @ imm = #-0x458
    c.b(C::Al,0xc5130,false)?;
    // VMA 0xc5584: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5588: and r0, r0, r5, lsr #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(5)?,Shift::Lsr,16,false)?)?;
    // VMA 0xc558c: strb r0, [r9, #0x228]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,552)?)?;
    // VMA 0xc5590: b 0xc5978 <entry+0x978>   @ imm = #0x3e0
    c.b(C::Al,0xc5978,false)?;
    // VMA 0xc5594: ldr r0, [r10, #0x554]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1364)?)?;
    // VMA 0xc5598: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc559c: ldr r0, [r0, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,640)?)?;
    // VMA 0xc55a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc55a4: bne 0xc55d0 <entry+0x5d0>   @ imm = #0x24
    c.b(C::Ne,0xc55d0,false)?;
    // VMA 0xc55a8: ldr r0, [r10, #0x558]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1368)?)?;
    // VMA 0xc55ac: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc55b0: ldr r0, [r0, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,640)?)?;
    // VMA 0xc55b4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc55b8: bne 0xc55d0 <entry+0x5d0>   @ imm = #0x10
    c.b(C::Ne,0xc55d0,false)?;
    // VMA 0xc55bc: ldr r0, [r10, #0x55c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1372)?)?;
    // VMA 0xc55c0: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc55c4: ldr r0, [r0, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,640)?)?;
    // VMA 0xc55c8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc55cc: beq 0xc6274 <entry+0x1274>  @ imm = #0xca0
    c.b(C::Eq,0xc6274,false)?;
    // VMA 0xc55d0: str r0, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,2028)?)?;
    // VMA 0xc55d4: ldr r0, [r10, #0x558]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1368)?)?;
    // VMA 0xc55d8: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc55dc: ldr r3, [r0, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,640)?)?;
    // VMA 0xc55e0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc55e4: bne 0xc5668 <entry+0x668>   @ imm = #0x7c
    c.b(C::Ne,0xc5668,false)?;
    // VMA 0xc55e8: ldr r0, [r10, #0x55c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1372)?)?;
    // VMA 0xc55ec: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc55f0: ldr r3, [r0, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,640)?)?;
    // VMA 0xc55f4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc55f8: bne 0xc5668 <entry+0x668>   @ imm = #0x68
    c.b(C::Ne,0xc5668,false)?;
    // VMA 0xc55fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc5600: add r1, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc5604: ldr r3, [r1, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,640)?)?;
    // VMA 0xc5608: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc560c: bne 0xc5668 <entry+0x668>   @ imm = #0x54
    c.b(C::Ne,0xc5668,false)?;
    // VMA 0xc5610: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xc5614: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // VMA 0xc5618: bls 0xc5600 <entry+0x600>   @ imm = #-0x20
    c.b(C::Ls,0xc5600,false)?;
    // VMA 0xc561c: ldr r3, [r10, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2032)?)?;
    // VMA 0xc5620: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc5624: bne 0xc566c <entry+0x66c>   @ imm = #0x40
    c.b(C::Ne,0xc566c,false)?;
    // VMA 0xc5628: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xc562c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xc5630: bl 0xc6334 <entry+0x1334>  @ imm = #0xcfc
    c.b(C::Al,0xc6334,true)?;
    // VMA 0xc5634: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5638: mov r3, r10
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // VMA 0xc563c: str r0, [r10, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,516)?)?;
    // VMA 0xc5640: str r10, [r10, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2032)?)?;
    // VMA 0xc5644: b 0xc566c <entry+0x66c>   @ imm = #0x20
    c.b(C::Al,0xc566c,false)?;
    // VMA 0xc5648: ldr r1, [pc, #0xc8c]        @ 0x12dc <entry+0x12dc>
    c.literal(R::R1,0xc62dc)?;
    // VMA 0xc564c: ldr r2, [pc, #0xc78]        @ 0x12cc <entry+0x12cc>
    c.literal(R::R2,0xc62cc)?;
    // VMA 0xc5650: mov r3, #376
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,376)?)?;
    // VMA 0xc5654: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // VMA 0xc5658: b 0xc4d1c <entry+0xfffffffffffffd1c> @ imm = #-0x944
    c.b(C::Al,0xc4d1c,false)?;
    // VMA 0xc565c: strb r0, [r9, #0x226]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,550)?)?;
    // VMA 0xc5660: strb r0, [r9, #0x227]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,551)?)?;
    // VMA 0xc5664: b 0xc5978 <entry+0x978>   @ imm = #0x30c
    c.b(C::Al,0xc5978,false)?;
    // VMA 0xc5668: str r3, [r10, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,2032)?)?;
    // VMA 0xc566c: ldr r0, [r10, #0x55c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1372)?)?;
    // VMA 0xc5670: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc5674: ldr r0, [r0, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,640)?)?;
    // VMA 0xc5678: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc567c: bne 0xc56cc <entry+0x6cc>   @ imm = #0x48
    c.b(C::Ne,0xc56cc,false)?;
    // VMA 0xc5680: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc5684: add r0, r9, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xc5688: ldr r0, [r0, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,640)?)?;
    // VMA 0xc568c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5690: bne 0xc56cc <entry+0x6cc>   @ imm = #0x34
    c.b(C::Ne,0xc56cc,false)?;
    // VMA 0xc5694: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xc5698: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // VMA 0xc569c: bls 0xc5684 <entry+0x684>   @ imm = #-0x20
    c.b(C::Ls,0xc5684,false)?;
    // VMA 0xc56a0: ldr r0, [r10, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2036)?)?;
    // VMA 0xc56a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc56a8: bne 0xc56d0 <entry+0x6d0>   @ imm = #0x20
    c.b(C::Ne,0xc56d0,false)?;
    // VMA 0xc56ac: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xc56b0: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xc56b4: bl 0xc6334 <entry+0x1334>  @ imm = #0xc78
    c.b(C::Al,0xc6334,true)?;
    // VMA 0xc56b8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc56bc: str r0, [r10, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,516)?)?;
    // VMA 0xc56c0: str r10, [r10, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2036)?)?;
    // VMA 0xc56c4: ldr r3, [r10, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,2032)?)?;
    // VMA 0xc56c8: b 0xc5208 <entry+0x208>   @ imm = #-0x4c8
    c.b(C::Al,0xc5208,false)?;
    // VMA 0xc56cc: str r0, [r10, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,2036)?)?;
    // VMA 0xc56d0: ldr r2, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2028)?)?;
    // VMA 0xc56d4: ldr r1, [pc, #0xc08]        @ 0x12e4 <entry+0x12e4>
    c.literal(R::R1,0xc62e4)?;
    // VMA 0xc56d8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xc56dc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc56e0: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xc56e4: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96a58
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc56e8: ldr r1, [pc, #0xbd8]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc56ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc56f0: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96a64
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc56f4: and r0, r4, r7, lsr #3
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::new(7)?,Shift::Lsr,3,false)?)?;
    // VMA 0xc56f8: and r1, r4, r7, lsr #12
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::new(7)?,Shift::Lsr,12,false)?)?;
    // VMA 0xc56fc: and r2, r4, r7, lsr #14
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(4)?,R::new(7)?,Shift::Lsr,14,false)?)?;
    // VMA 0xc5700: strb r0, [r10, #0x54f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1359)?)?;
    // VMA 0xc5704: and r0, r4, r7, lsr #7
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::new(7)?,Shift::Lsr,7,false)?)?;
    // VMA 0xc5708: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc570c: strb r0, [r10, #0x550]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1360)?)?;
    // VMA 0xc5710: and r0, r4, r7, lsr #11
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::new(7)?,Shift::Lsr,11,false)?)?;
    // VMA 0xc5714: strb r0, [r10, #0x551]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1361)?)?;
    // VMA 0xc5718: and r0, r4, r7, lsr #13
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::new(7)?,Shift::Lsr,13,false)?)?;
    // VMA 0xc571c: str r1, [r10, #0x560]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1376)?)?;
    // VMA 0xc5720: str r0, [r10, #0x564]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1380)?)?;
    // VMA 0xc5724: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc5728: str r2, [r10, #0x568]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,1384)?)?;
    // VMA 0xc572c: bne 0xc5758 <entry+0x758>   @ imm = #0x24
    c.b(C::Ne,0xc5758,false)?;
    // VMA 0xc5730: add r0, r10, #356
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,356)?)?;
    // VMA 0xc5734: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc5738: add r1, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1024)?)?;
    // VMA 0xc573c: add r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1)?)?;
    // VMA 0xc5740: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xc5744: bhi 0xc5814 <entry+0x814>   @ imm = #0xc8
    c.b(C::Hi,0xc5814,false)?;
    // VMA 0xc5748: ldr r2, [r1, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R1,R::R2,Shift::Lsl,2)?)?;
    // VMA 0xc574c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc5750: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // VMA 0xc5754: beq 0xc573c <entry+0x73c>   @ imm = #-0x20
    c.b(C::Eq,0xc573c,false)?;
    // VMA 0xc5758: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc575c: ldr r1, [r0, #0x554]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,1364)?)?;
    // VMA 0xc5760: add r2, r9, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(9)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xc5764: ldr r2, [r2, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,640)?)?;
    // VMA 0xc5768: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc576c: beq 0xc57c0 <entry+0x7c0>   @ imm = #0x4c
    c.b(C::Eq,0xc57c0,false)?;
    // VMA 0xc5770: ldr r2, [r2, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,112)?)?;
    // VMA 0xc5774: str r2, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,112)?)?;
    // VMA 0xc5778: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xc577c: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc5780: ldr r0, [r0, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,640)?)?;
    // VMA 0xc5784: ldr r3, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,108)?)?;
    // VMA 0xc5788: str r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc578c: b 0xc57fc <entry+0x7fc>   @ imm = #0x68
    c.b(C::Al,0xc57fc,false)?;
    // VMA 0xc5790: ldr r0, [r4, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2700)?)?;
    // VMA 0xc5794: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc5798: bne 0xc5130 <entry+0x130>   @ imm = #-0x670
    c.b(C::Ne,0xc5130,false)?;
    // VMA 0xc579c: ldr r1, [pc, #0xb2c]        @ 0x12d0 <entry+0x12d0>
    c.literal(R::R1,0xc62d0)?;
    // VMA 0xc57a0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc57a4: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // VMA 0xc57a8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc57ac: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96b20
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc57b0: ldr r1, [pc, #0xb10]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc57b4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc57b8: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96b2c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc57bc: b 0xc5130 <entry+0x130>   @ imm = #-0x694
    c.b(C::Al,0xc5130,false)?;
    // VMA 0xc57c0: add r0, r10, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xc57c4: str r10, [r0, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,2028)?)?;
    // VMA 0xc57c8: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // VMA 0xc57cc: ldr r1, [r0, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,468)?)?;
    // VMA 0xc57d0: str r1, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,112)?)?;
    // VMA 0xc57d4: ldr r1, [pc, #0xb14]        @ 0x12f0 <entry+0x12f0>
    c.literal(R::R1,0xc62f0)?;
    // VMA 0xc57d8: ldr r0, [r0, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,472)?)?;
    // VMA 0xc57dc: str r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,108)?)?;
    // VMA 0xc57e0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc57e4: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96b58
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc57e8: ldr r1, [pc, #0xad8]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc57ec: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc57f0: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96b64
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc57f4: ldr r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc57f8: ldr r2, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,112)?)?;
    // VMA 0xc57fc: ldr r1, [pc, #0xaf0]        @ 0x12f4 <entry+0x12f4>
    c.literal(R::R1,0xc62f4)?;
    // VMA 0xc5800: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc5804: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96b78
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5808: ldr r1, [pc, #0xab8]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc580c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc5810: b 0xc5870 <entry+0x870>   @ imm = #0x58
    c.b(C::Al,0xc5870,false)?;
    // VMA 0xc5814: ldr r1, [pc, #0xacc]        @ 0x12e8 <entry+0x12e8>
    c.literal(R::R1,0xc62e8)?;
    // VMA 0xc5818: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc581c: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96b90
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5820: mov r4, r6
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(6)?))?;
    // VMA 0xc5824: ldr r6, [pc, #0xa9c]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::new(6)?,0xc62c8)?;
    // VMA 0xc5828: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc582c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc5830: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96ba4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5834: ldr r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xc5838: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // VMA 0xc583c: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // VMA 0xc5840: and r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc5844: add r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,1)?)?;
    // VMA 0xc5848: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc584c: add r3, r0, r1, lsr #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::R1,Shift::Lsr,16,false)?)?;
    // VMA 0xc5850: ldr r1, [pc, #0xa94]        @ 0x12ec <entry+0x12ec>
    c.literal(R::R1,0xc62ec)?;
    // VMA 0xc5854: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc5858: str r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc585c: str r2, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,112)?)?;
    // VMA 0xc5860: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96bd4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5864: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc5868: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc586c: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // VMA 0xc5870: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96be4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5874: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5878: and r1, r0, r6, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(6)?,Shift::Lsr,8,false)?)?;
    // VMA 0xc587c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc5880: strb r1, [r10, #0x578]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,1400)?)?;
    // VMA 0xc5884: beq 0xc5898 <entry+0x898>   @ imm = #0xc
    c.b(C::Eq,0xc5898,false)?;
    // VMA 0xc5888: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // VMA 0xc588c: add r0, r0, r1, lsr #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsr,16,false)?)?;
    // VMA 0xc5890: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xc5894: b 0xc58a0 <entry+0x8a0>   @ imm = #0x4
    c.b(C::Al,0xc58a0,false)?;
    // VMA 0xc5898: ldr r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,108)?)?;
    // VMA 0xc589c: ldr r1, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,112)?)?;
    // VMA 0xc58a0: mov r2, #122
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,122)?)?;
    // VMA 0xc58a4: orr r2, r2, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1280)?)?;
    // VMA 0xc58a8: strh r1, [r10, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(10)?,R::R2)?)?;
    // VMA 0xc58ac: mov r1, #380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,380)?)?;
    // VMA 0xc58b0: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xc58b4: strh r0, [r10, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(10)?,R::R1)?)?;
    // VMA 0xc58b8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc58bc: and r1, r0, r7, lsr #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(7)?,Shift::Lsr,15,false)?)?;
    // VMA 0xc58c0: strb r1, [r9, #0x375]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,885)?)?;
    // VMA 0xc58c4: and r1, r0, r7, lsr #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(7)?,Shift::Lsr,16,false)?)?;
    // VMA 0xc58c8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc58cc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc58d0: strb r1, [r10, #0x56d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,1389)?)?;
    // VMA 0xc58d4: bne 0xc58e8 <entry+0x8e8>   @ imm = #0xc
    c.b(C::Ne,0xc58e8,false)?;
    // VMA 0xc58d8: ldr r1, [pc, #0xa18]        @ 0x12f8 <entry+0x12f8>
    c.literal(R::R1,0xc62f8)?;
    // VMA 0xc58dc: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // VMA 0xc58e0: and r0, r0, r7, lsr #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(7)?,Shift::Lsr,15,false)?)?;
    // VMA 0xc58e4: ldr r0, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc58e8: ldr r7, [pc, #0x9d4]        @ 0x12c4 <entry+0x12c4>
    c.literal(R::new(7)?,0xc62c4)?;
    // VMA 0xc58ec: strb r0, [r9, #0x374]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,884)?)?;
    // VMA 0xc58f0: add r0, r10, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,1004)?)?;
    // VMA 0xc58f4: mov r6, #202
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,202)?)?;
    // VMA 0xc58f8: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xc58fc: add r4, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,1024)?)?;
    // VMA 0xc5900: add r0, r10, #972
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,972)?)?;
    // VMA 0xc5904: orr r6, r6, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,1536)?)?;
    // VMA 0xc5908: add r5, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,1024)?)?;
    // VMA 0xc590c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc5910: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc5914: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xc5918: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc591c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5920: bl 0x17b0 <entry+0xfffffffffff3c7b0> @ imm = #-0xc4178
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc5924: ldr r0, [r4], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xc5928: ldr r1, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,112)?)?;
    // VMA 0xc592c: ldr r3, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,108)?)?;
    // VMA 0xc5930: ldr r2, [r0, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,112)?)?;
    // VMA 0xc5934: ldr r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,108)?)?;
    // VMA 0xc5938: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xc593c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc5940: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xc5944: sub r0, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(5)?,4)?)?;
    // VMA 0xc5948: bl 0xc420c <entry+0xfffffffffffff20c> @ imm = #-0x1744
    c.b(C::Al,0xc420c,true)?;
    // VMA 0xc594c: add r8, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // VMA 0xc5950: add r5, r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,8)?)?;
    // VMA 0xc5954: cmp r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,3)?)?;
    // VMA 0xc5958: bne 0xc590c <entry+0x90c>   @ imm = #-0x54
    c.b(C::Ne,0xc590c,false)?;
    // VMA 0xc595c: b 0xc5a04 <entry+0xa04>   @ imm = #0xa0
    c.b(C::Al,0xc5a04,false)?;
    // VMA 0xc5960: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5964: strb r0, [r9, #0x224]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,548)?)?;
    // VMA 0xc5968: strb r0, [r9, #0x226]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,550)?)?;
    // VMA 0xc596c: strb r0, [r9, #0x227]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,551)?)?;
    // VMA 0xc5970: ldr r5, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,32)?)?;
    // VMA 0xc5974: ldr r6, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,36)?)?;
    // VMA 0xc5978: lsr r0, r5, #20
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsr,20)?)?;
    // VMA 0xc597c: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xc5980: strb r0, [r10, #0x54d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1357)?)?;
    // VMA 0xc5984: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // VMA 0xc5988: ldr r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // VMA 0xc598c: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // VMA 0xc5990: and r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc5994: add r3, r4, r1, lsr #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(4)?,R::R1,Shift::Lsr,16,false)?)?;
    // VMA 0xc5998: ldr r1, [pc, #0x940]        @ 0x12e0 <entry+0x12e0>
    c.literal(R::R1,0xc62e0)?;
    // VMA 0xc599c: add r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,1)?)?;
    // VMA 0xc59a0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc59a4: str r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc59a8: str r2, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,112)?)?;
    // VMA 0xc59ac: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96d20
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc59b0: ldr r1, [pc, #0x910]        @ 0x12c8 <entry+0x12c8>
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc59b4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc59b8: bl 0x2ec94 <entry+0xfffffffffff69c94> @ imm = #-0x96d2c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc59bc: and r0, r4, r5, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::new(5)?,Shift::Lsr,8,false)?)?;
    // VMA 0xc59c0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc59c4: strb r0, [r10, #0x578]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1400)?)?;
    // VMA 0xc59c8: beq 0xc59d8 <entry+0x9d8>   @ imm = #0x8
    c.b(C::Eq,0xc59d8,false)?;
    // VMA 0xc59cc: add r0, r4, r6, lsr #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(6)?,Shift::Lsr,16,false)?)?;
    // VMA 0xc59d0: add r1, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,1)?)?;
    // VMA 0xc59d4: b 0xc59e0 <entry+0x9e0>   @ imm = #0x4
    c.b(C::Al,0xc59e0,false)?;
    // VMA 0xc59d8: ldr r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,108)?)?;
    // VMA 0xc59dc: ldr r1, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,112)?)?;
    // VMA 0xc59e0: mov r2, #122
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,122)?)?;
    // VMA 0xc59e4: orr r2, r2, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1280)?)?;
    // VMA 0xc59e8: strh r1, [r10, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(10)?,R::R2)?)?;
    // VMA 0xc59ec: mov r1, #380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,380)?)?;
    // VMA 0xc59f0: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xc59f4: strh r0, [r10, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(10)?,R::R1)?)?;
    // VMA 0xc59f8: str r10, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2028)?)?;
    // VMA 0xc59fc: str r10, [r10, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2032)?)?;
    // VMA 0xc5a00: str r10, [r10, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2036)?)?;
    // VMA 0xc5a04: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xc5a08: bl 0xc7a74 <entry+0x2a74>  @ imm = #0x2064
    c.b(C::Al,0xc7a74,true)?;
    // VMA 0xc5a0c: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xc5a10: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5a14: bne 0xc5a34 <entry+0xa34>   @ imm = #0x18
    c.b(C::Ne,0xc5a34,false)?;
    // VMA 0xc5a18: ldrb r0, [r10, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1355)?)?;
    // VMA 0xc5a1c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5a20: beq 0xc5a34 <entry+0xa34>   @ imm = #0xc
    c.b(C::Eq,0xc5a34,false)?;
    // VMA 0xc5a24: ldr r2, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,108)?)?;
    // VMA 0xc5a28: ldr r1, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,112)?)?;
    // VMA 0xc5a2c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc5a30: bl 0xc46e4 <entry+0xfffffffffffff6e4> @ imm = #-0x1354
    c.b(C::Al,0xc46e4,true)?;
    // VMA 0xc5a34: ldr r2, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,468)?)?;
    // VMA 0xc5a38: ldr r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,112)?)?;
    // VMA 0xc5a3c: cmp r0, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc5a40: bhi 0xc5a90 <entry+0xa90>   @ imm = #0x48
    c.b(C::Hi,0xc5a90,false)?;
    // VMA 0xc5a44: ldr r3, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,472)?)?;
    // VMA 0xc5a48: ldr r1, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,108)?)?;
    // VMA 0xc5a4c: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc5a50: bhi 0xc5a90 <entry+0xa90>   @ imm = #0x38
    c.b(C::Hi,0xc5a90,false)?;
    // VMA 0xc5a54: ldr r7, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,192)?)?;
    // VMA 0xc5a58: ldr r6, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,196)?)?;
    // VMA 0xc5a5c: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc5a60: blo 0xc5a90 <entry+0xa90>   @ imm = #0x28
    c.b(C::Cc,0xc5a90,false)?;
    // VMA 0xc5a64: ldr r7, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,972)?)?;
    // VMA 0xc5a68: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc5a6c: beq 0xc5fac <entry+0xfac>   @ imm = #0x538
    c.b(C::Eq,0xc5fac,false)?;
    // VMA 0xc5a70: ldr r7, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,248)?)?;
    // VMA 0xc5a74: ldr r6, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,416)?)?;
    // VMA 0xc5a78: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc5a7c: blo 0xc5a90 <entry+0xa90>   @ imm = #0xc
    c.b(C::Cc,0xc5a90,false)?;
    // VMA 0xc5a80: ldr r7, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,236)?)?;
    // VMA 0xc5a84: ldr r6, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,412)?)?;
    // VMA 0xc5a88: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc5a8c: bhs 0xc5fac <entry+0xfac>   @ imm = #0x518
    c.b(C::Cs,0xc5fac,false)?;
    // VMA 0xc5a90: ldr r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,192)?)?;
    // VMA 0xc5a94: ldr r1, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,196)?)?;
    // VMA 0xc5a98: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xc5a9c: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc5aa0: bhs 0xc5ab0 <entry+0xab0>   @ imm = #0x8
    c.b(C::Cs,0xc5ab0,false)?;
    // VMA 0xc5aa4: mov r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,120)?)?;
    // VMA 0xc5aa8: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xc5aac: b 0xc5b00 <entry+0xb00>   @ imm = #0x4c
    c.b(C::Al,0xc5b00,false)?;
    // VMA 0xc5ab0: ldr r0, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xc5ab4: mov r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,120)?)?;
    // VMA 0xc5ab8: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xc5abc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5ac0: beq 0xc5afc <entry+0xafc>   @ imm = #0x34
    c.b(C::Eq,0xc5afc,false)?;
    // VMA 0xc5ac4: ldr r0, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,248)?)?;
    // VMA 0xc5ac8: ldr r1, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,416)?)?;
    // VMA 0xc5acc: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc5ad0: blo 0xc5b00 <entry+0xb00>   @ imm = #0x28
    c.b(C::Cc,0xc5b00,false)?;
    // VMA 0xc5ad4: ldr r0, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,236)?)?;
    // VMA 0xc5ad8: ldr r1, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,412)?)?;
    // VMA 0xc5adc: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xc5ae0: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc5ae4: movlo r2, #3
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xc5ae8: b 0xc5b00 <entry+0xb00>   @ imm = #0x10
    c.b(C::Al,0xc5b00,false)?;

    c.check(0xc5aec)
}
