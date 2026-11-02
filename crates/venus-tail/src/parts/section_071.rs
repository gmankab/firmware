
/// Initialize one hardware codec output record using index-scaled plane
/// addresses, dispatch among four controller modes, and start emitting
/// descriptor sizes and auxiliary channel parameters. Continuation follows.
fn begin_codec_output_frame_descriptor(c:&mut Code)->Result<(),String>{
    c.check(0xbeea0)?;
    // VMA 0xbeea0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xbeea4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xbeea8: sub sp, sp, #68
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,68)?)?;
    // VMA 0xbeeac: ldr r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,8)?)?;
    // VMA 0xbeeb0: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // VMA 0xbeeb4: str r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,32)?)?;
    // VMA 0xbeeb8: add r2, r2, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xbeebc: add r0, r0, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,2,false)?)?;
    // VMA 0xbeec0: ldr r0, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,360)?)?;
    // VMA 0xbeec4: ldm r0, {r2, r10}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xbeec8: ldr r0, [r2, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,24)?)?;
    // VMA 0xbeecc: str r0, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-44)?)?;
    // VMA 0xbeed0: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbeed4: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // VMA 0xbeed8: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xbeedc: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xbeee0: bne 0xbef08 <entry+0x398>   @ imm = #0x20
    c.b(C::Ne,0xbef08,false)?;
    // VMA 0xbeee4: add r0, r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,12)?)?;
    // VMA 0xbeee8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbeeec: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xbeef0: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // VMA 0xbeef4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbeef8: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // VMA 0xbeefc: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xbef00: mov r0, #300
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,300)?)?;
    // VMA 0xbef04: b 0xbef2c <entry+0x3bc>   @ imm = #0x20
    c.b(C::Al,0xbef2c,false)?;
    // VMA 0xbef08: add r0, r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,12)?)?;
    // VMA 0xbef0c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbef10: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // VMA 0xbef14: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xbef18: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbef1c: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // VMA 0xbef20: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // VMA 0xbef24: mov r0, #232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,232)?)?;
    // VMA 0xbef28: orr r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // VMA 0xbef2c: ldr r9, [pc, #0xed0]        @ 0x1294 <entry+0x1294>
    c.literal(R::new(9)?,0xbfe04)?;
    // VMA 0xbef30: mov r4, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // VMA 0xbef34: add r2, r10, #676
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(10)?,676)?)?;
    // VMA 0xbef38: add r6, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbef3c: add r5, r10, #672
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(10)?,672)?)?;
    // VMA 0xbef40: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xbef44: orr r4, r4, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,1024)?)?;
    // VMA 0xbef48: str r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,40)?)?;
    // VMA 0xbef4c: b 0xbefc0 <entry+0x450>   @ imm = #0x6c
    c.b(C::Al,0xbefc0,false)?;
    // VMA 0xbef50: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xbef54: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // VMA 0xbef58: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbef5c: str r0, [r10, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,432)?)?;
    // VMA 0xbef60: ldr r1, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,8)?)?;
    // VMA 0xbef64: str r1, [r10, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,444)?)?;
    // VMA 0xbef68: str r1, [r10, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,412)?)?;
    // VMA 0xbef6c: str r0, [r10, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,420)?)?;
    // VMA 0xbef70: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // VMA 0xbef74: str r0, [r10, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,408)?)?;
    // VMA 0xbef78: str r8, [r10, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(10)?,348)?)?;
    // VMA 0xbef7c: ldr r0, [r10, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,364)?)?;
    // VMA 0xbef80: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xbef84: bl 0x7f974 <entry+0xfffffffffffc0e04> @ imm = #-0x3f618
    c.b(C::Al,0x7f974,true)?;
    // VMA 0xbef88: ldr r0, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,676)?)?;
    // VMA 0xbef8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbef90: bne 0xbf7b4 <entry+0xc44>   @ imm = #0x81c
    c.b(C::Ne,0xbf7b4,false)?;
    // VMA 0xbef94: ldr r1, [pc, #0xeb4]        @ 0x12e0 <entry+0x12e0>
    c.literal(R::R1,0xbfe50)?;
    // VMA 0xbef98: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbef9c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90310
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbefa0: ldr r1, [pc, #0xe54]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbefa4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbefa8: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9031c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbefac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbefb0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xbefb4: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // VMA 0xbefb8: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbd810
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbefbc: ldr r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,40)?)?;
    // VMA 0xbefc0: ldr r0, [r10, #0x15c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,348)?)?;
    // VMA 0xbefc4: ldr r7, [pc, #0xe28]        @ 0x1284 <entry+0x1284>
    c.literal(R::new(7)?,0xbfdf4)?;
    // VMA 0xbefc8: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xbefcc: bhi 0xbefc4 <entry+0x454>   @ imm = #-0x10
    c.b(C::Hi,0xbefc4,false)?;
    // VMA 0xbefd0: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbefd4: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // VMA 0xbefd8: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0xbefdc)?;
    const OUTPUT_PLANE_MODE_TARGETS:[u32;4]=[
        0x000bef50,0x000bef7c,0x000befec,0x000bf8c4,
    ];
    for target in OUTPUT_PLANE_MODE_TARGETS {c.emit(target)?;}
    c.check(0xbefec)?;
    // VMA 0xbefec: ldr r0, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,0)?)?;
    // VMA 0xbeff0: ldr r1, [pc, #0xe00]        @ 0x1288 <entry+0x1288>
    c.literal(R::R1,0xbfdf8)?;
    // VMA 0xbeff4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xbeff8: str r6, [r0, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,1272)?)?;
    // VMA 0xbeffc: str r6, [r0, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,1276)?)?;
    // VMA 0xbf000: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbf004: str r6, [r10, #0x240]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,576)?)?;
    // VMA 0xbf008: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9037c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf00c: ldr r1, [pc, #0xde8]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf010: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbf014: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90388
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf018: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xbf01c: ldr r1, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,676)?)?;
    // VMA 0xbf020: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // VMA 0xbf024: ldr r3, [r4, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,464)?)?;
    // VMA 0xbf028: ldmib r1, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xbf02c: bl 0x27718 <entry+0xfffffffffff68ba8> @ imm = #-0x9791c
    c.b(C::Al,0x27718,true)?;
    // VMA 0xbf030: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // VMA 0xbf034: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xbf038: beq 0xbf17c <entry+0x60c>   @ imm = #0x13c
    c.b(C::Eq,0xbf17c,false)?;
    // VMA 0xbf03c: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xbf040: bne 0xbf88c <entry+0xd1c>   @ imm = #0x844
    c.b(C::Ne,0xbf88c,false)?;
    // VMA 0xbf044: ldr r9, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(10)?,676)?)?;
    // VMA 0xbf048: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xbf04c: ldr r2, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,0)?)?;
    // VMA 0xbf050: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // VMA 0xbf054: str r6, [r9, #0x50c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,1292)?)?;
    // VMA 0xbf058: ldr r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,28)?)?;
    // VMA 0xbf05c: ldrb r0, [r4, #0xd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,13)?)?;
    // VMA 0xbf060: and r0, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // VMA 0xbf064: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbf068: strb r0, [r9, #0x508]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,1288)?)?;
    // VMA 0xbf06c: str r6, [r9, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,524)?)?;
    // VMA 0xbf070: ldrb r0, [r4, #0xd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,13)?)?;
    // VMA 0xbf074: and r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,1)?)?;
    // VMA 0xbf078: str r0, [r9, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1296)?)?;
    // VMA 0xbf07c: ldrb r0, [r4, #0xd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,13)?)?;
    // VMA 0xbf080: and r0, r1, r0, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsr,4,false)?)?;
    // VMA 0xbf084: str r0, [r9, #0x588]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1416)?)?;
    // VMA 0xbf088: strb r6, [r9, #0x522]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1314)?)?;
    // VMA 0xbf08c: strb r6, [r9, #0x546]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1350)?)?;
    // VMA 0xbf090: strb r6, [r9, #0x547]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1351)?)?;
    // VMA 0xbf094: strb r6, [r9, #0x548]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1352)?)?;
    // VMA 0xbf098: ldrb r0, [r4, #0x33]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,51)?)?;
    // VMA 0xbf09c: strb r0, [r9, #0x56f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,1391)?)?;
    // VMA 0xbf0a0: strb r6, [r9, #0x570]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1392)?)?;
    // VMA 0xbf0a4: strb r6, [r9, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1355)?)?;
    // VMA 0xbf0a8: strb r6, [r9, #0x571]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1393)?)?;
    // VMA 0xbf0ac: strb r6, [r9, #0x54c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1356)?)?;
    // VMA 0xbf0b0: strb r6, [r9, #0x56c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1388)?)?;
    // VMA 0xbf0b4: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbf0b8: str r0, [r9, #0x780]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1920)?)?;
    // VMA 0xbf0bc: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xbf0c0: str r0, [r2, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,476)?)?;
    // VMA 0xbf0c4: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbf0c8: str r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
    // VMA 0xbf0cc: str r1, [r2, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,480)?)?;
    // VMA 0xbf0d0: ands r2, r0, #15
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::R0,15,true)?)?;
    // VMA 0xbf0d4: rsb r2, r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R2,R::R2,16)?)?;
    // VMA 0xbf0d8: addne r0, r0, r2
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbf0dc: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // VMA 0xbf0e0: orr r2, r2, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65280)?)?;
    // VMA 0xbf0e4: and r8, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(8)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbf0e8: ands r0, r1, #15
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R1,15,true)?)?;
    // VMA 0xbf0ec: rsb r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,16)?)?;
    // VMA 0xbf0f0: addne r1, r1, r0
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbf0f4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf0f8: and r5, r1, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbf0fc: ldr r1, [pc, #0xd20]        @ 0x12b4 <entry+0x12b4>
    c.literal(R::R1,0xbfe24)?;
    // VMA 0xbf100: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xbf104: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xbf108: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9047c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf10c: ldr r1, [pc, #0xce8]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf110: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf114: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90488
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf118: str r5, [r9, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(9)?,108)?)?;
    // VMA 0xbf11c: str r8, [r9, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,112)?)?;
    // VMA 0xbf120: ldr r1, [pc, #0xcd8]        @ 0x1290 <entry+0x1290>
    c.literal(R::R1,0xbfe00)?;
    // VMA 0xbf124: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf128: ldmib r4, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(4)?,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xbf12c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x904a0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf130: ldr r1, [pc, #0xcc4]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf134: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf138: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x904ac
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf13c: strb r6, [r9, #0x56e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1390)?)?;
    // VMA 0xbf140: strb r6, [r9, #0x54e]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1358)?)?;
    // VMA 0xbf144: strb r6, [r9, #0x54f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(9)?,1359)?)?;
    // VMA 0xbf148: ldrb r0, [r4, #0x31]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,49)?)?;
    // VMA 0xbf14c: strb r0, [r9, #0x550]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,1360)?)?;
    // VMA 0xbf150: ldrb r0, [r4, #0x32]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,50)?)?;
    // VMA 0xbf154: strb r0, [r9, #0x551]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,1361)?)?;
    // VMA 0xbf158: ldr r0, [r9, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1296)?)?;
    // VMA 0xbf15c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbf160: beq 0xbf2c4 <entry+0x754>   @ imm = #0x15c
    c.b(C::Eq,0xbf2c4,false)?;
    // VMA 0xbf164: ldr r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,16)?)?;
    // VMA 0xbf168: ldr r1, [r9, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,108)?)?;
    // VMA 0xbf16c: ldr r2, [r9, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,112)?)?;
    // VMA 0xbf170: ldr r6, [r5, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,468)?)?;
    // VMA 0xbf174: ldr r3, [r5, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,472)?)?;
    // VMA 0xbf178: b 0xbf2f0 <entry+0x780>   @ imm = #0x170
    c.b(C::Al,0xbf2f0,false)?;
    // VMA 0xbf17c: ldr r5, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,48)?)?;
    // VMA 0xbf180: ldr r8, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(10)?,0)?)?;
    // VMA 0xbf184: ldr r4, [r10, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,676)?)?;
    // VMA 0xbf188: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xbf18c: ldrb r0, [r5, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,1)?)?;
    // VMA 0xbf190: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xbf194: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf198: str r0, [r4, #0x50c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1292)?)?;
    // VMA 0xbf19c: str r6, [r4, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,524)?)?;
    // VMA 0xbf1a0: str r6, [r8, #0x904]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,2308)?)?;
    // VMA 0xbf1a4: ldrh r0, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(5)?,2)?)?;
    // VMA 0xbf1a8: and r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,1)?)?;
    // VMA 0xbf1ac: str r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1296)?)?;
    // VMA 0xbf1b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf1b4: ldrh r1, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,2)?)?;
    // VMA 0xbf1b8: and r1, r0, r1, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsr,1,false)?)?;
    // VMA 0xbf1bc: str r1, [r4, #0x588]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1416)?)?;
    // VMA 0xbf1c0: ldrh r1, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,2)?)?;
    // VMA 0xbf1c4: and r1, r0, r1, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsr,2,false)?)?;
    // VMA 0xbf1c8: strb r1, [r4, #0x522]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1314)?)?;
    // VMA 0xbf1cc: ldrh r1, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,2)?)?;
    // VMA 0xbf1d0: and r1, r0, r1, lsr #3
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsr,3,false)?)?;
    // VMA 0xbf1d4: strb r1, [r4, #0x546]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1350)?)?;
    // VMA 0xbf1d8: ldrh r1, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,2)?)?;
    // VMA 0xbf1dc: and r1, r0, r1, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsr,4,false)?)?;
    // VMA 0xbf1e0: strb r1, [r4, #0x547]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1351)?)?;
    // VMA 0xbf1e4: ldrh r1, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,2)?)?;
    // VMA 0xbf1e8: and r1, r0, r1, lsr #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsr,5,false)?)?;
    // VMA 0xbf1ec: strb r1, [r4, #0x548]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1352)?)?;
    // VMA 0xbf1f0: ldrh r1, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,2)?)?;
    // VMA 0xbf1f4: and r1, r0, r1, lsr #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsr,6,false)?)?;
    // VMA 0xbf1f8: strb r1, [r4, #0x56f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1391)?)?;
    // VMA 0xbf1fc: ldrh r1, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,2)?)?;
    // VMA 0xbf200: and r1, r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,128)?)?;
    // VMA 0xbf204: lsr r1, r1, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,7)?)?;
    // VMA 0xbf208: strb r1, [r4, #0x570]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1392)?)?;
    // VMA 0xbf20c: ldrb r1, [r5, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,3)?)?;
    // VMA 0xbf210: and r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,1)?)?;
    // VMA 0xbf214: strb r1, [r4, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1355)?)?;
    // VMA 0xbf218: ldrh r1, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,2)?)?;
    // VMA 0xbf21c: and r1, r2, r1, lsr #9
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsr,9,false)?)?;
    // VMA 0xbf220: strb r1, [r4, #0x571]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1393)?)?;
    // VMA 0xbf224: ldrh r1, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,2)?)?;
    // VMA 0xbf228: and r1, r2, r1, lsr #11
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsr,11,false)?)?;
    // VMA 0xbf22c: strb r1, [r4, #0x54c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1356)?)?;
    // VMA 0xbf230: ldrh r1, [r5, #2]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,2)?)?;
    // VMA 0xbf234: and r0, r0, r1, lsr #13
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsr,13,false)?)?;
    // VMA 0xbf238: ldr r1, [pc, #0xbc0]        @ 0x1290 <entry+0x1290>
    c.literal(R::R1,0xbfe00)?;
    // VMA 0xbf23c: strb r0, [r4, #0x56c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1388)?)?;
    // VMA 0xbf240: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf244: ldr r3, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,8)?)?;
    // VMA 0xbf248: str r3, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,108)?)?;
    // VMA 0xbf24c: ldr r2, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,4)?)?;
    // VMA 0xbf250: str r2, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,112)?)?;
    // VMA 0xbf254: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x905c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf258: ldr r1, [pc, #0xb9c]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf25c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf260: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x905d4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf264: ldrb r12, [r5, #0xe]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(5)?,14)?)?;
    // VMA 0xbf268: strb r12, [r4, #0x56e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(4)?,1390)?)?;
    // VMA 0xbf26c: ldrb r0, [r5, #0x5b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,91)?)?;
    // VMA 0xbf270: strb r0, [r4, #0x54e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1358)?)?;
    // VMA 0xbf274: ldrb r0, [r5, #0x5c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,92)?)?;
    // VMA 0xbf278: strb r0, [r4, #0x54f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1359)?)?;
    // VMA 0xbf27c: ldrb r0, [r5, #0x5d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,93)?)?;
    // VMA 0xbf280: strb r0, [r4, #0x550]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1360)?)?;
    // VMA 0xbf284: ldrb r0, [r5, #0x5e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,94)?)?;
    // VMA 0xbf288: strb r0, [r4, #0x551]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1361)?)?;
    // VMA 0xbf28c: ldr r1, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,112)?)?;
    // VMA 0xbf290: lsr r0, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Lsr,4)?)?;
    // VMA 0xbf294: str r0, [r8, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,484)?)?;
    // VMA 0xbf298: ldr r2, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,108)?)?;
    // VMA 0xbf29c: lsr r0, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsr,4)?)?;
    // VMA 0xbf2a0: str r0, [r8, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,488)?)?;
    // VMA 0xbf2a4: str r1, [r8, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,476)?)?;
    // VMA 0xbf2a8: str r2, [r8, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,480)?)?;
    // VMA 0xbf2ac: ldr r3, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1296)?)?;
    // VMA 0xbf2b0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xbf2b4: beq 0xbf524 <entry+0x9b4>   @ imm = #0x268
    c.b(C::Eq,0xbf524,false)?;
    // VMA 0xbf2b8: ldr r0, [r8, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,468)?)?;
    // VMA 0xbf2bc: ldr r6, [r8, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(8)?,472)?)?;
    // VMA 0xbf2c0: b 0xbf534 <entry+0x9c4>   @ imm = #0x26c
    c.b(C::Al,0xbf534,false)?;
    // VMA 0xbf2c4: ldr r2, [r9, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,112)?)?;
    // VMA 0xbf2c8: ldr r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,16)?)?;
    // VMA 0xbf2cc: lsr r1, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R2,Shift::Lsr,4)?)?;
    // VMA 0xbf2d0: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // VMA 0xbf2d4: str r1, [r5, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,484)?)?;
    // VMA 0xbf2d8: ldr r1, [r9, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,108)?)?;
    // VMA 0xbf2dc: lsr r3, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R1,Shift::Lsr,4)?)?;
    // VMA 0xbf2e0: str r3, [r5, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,488)?)?;
    // VMA 0xbf2e4: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // VMA 0xbf2e8: str r2, [r5, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,468)?)?;
    // VMA 0xbf2ec: str r1, [r5, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,472)?)?;
    // VMA 0xbf2f0: str r6, [r9, #0x580]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,1408)?)?;
    // VMA 0xbf2f4: str r3, [r9, #0x584]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,1412)?)?;
    // VMA 0xbf2f8: mov r3, #556
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,556)?)?;
    // VMA 0xbf2fc: mov r6, r10
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(10)?))?;
    // VMA 0xbf300: mov r12, r5
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(5)?))?;
    // VMA 0xbf304: ldrh r5, [r6, r3]!
    c.emit(arm32::load_halfword_register_preindexed_writeback(C::Al,R::new(5)?,R::new(6)?,R::new(3)?)?)?;
    // VMA 0xbf308: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xbf30c: cmp r2, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbf310: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbf314: bne 0xbf330 <entry+0x7c0>   @ imm = #0x14
    c.b(C::Ne,0xbf330,false)?;
    // VMA 0xbf318: mov r5, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,46)?)?;
    // VMA 0xbf31c: orr r5, r5, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,512)?)?;
    // VMA 0xbf320: ldrh r4, [r10, r5]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(4)?,R::new(10)?,R::new(5)?)?)?;
    // VMA 0xbf324: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbf328: cmp r1, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbf32c: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xbf330: strb r5, [r9, #0x54a]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(9)?,1354)?)?;
    // VMA 0xbf334: strb r3, [r9, #0x549]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(9)?,1353)?)?;
    // VMA 0xbf338: strh r2, [r6]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::new(6)?,0)?)?;
    // VMA 0xbf33c: mov r2, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,46)?)?;
    // VMA 0xbf340: orr r2, r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,512)?)?;
    // VMA 0xbf344: strh r1, [r10, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(10)?,R::R2)?)?;
    // VMA 0xbf348: ldr r1, [r10, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,560)?)?;
    // VMA 0xbf34c: str r1, [r9, #0x514]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,1300)?)?;
    // VMA 0xbf350: str r0, [r10, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,560)?)?;
    // VMA 0xbf354: ldrb r1, [r10, #0x229]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,553)?)?;
    // VMA 0xbf358: str r1, [r9, #0x58c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,1420)?)?;
    // VMA 0xbf35c: ldrb r1, [r9, #0x588]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,1416)?)?;
    // VMA 0xbf360: strb r1, [r10, #0x229]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,553)?)?;
    // VMA 0xbf364: ldrb r2, [r9, #0x508]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,1288)?)?;
    // VMA 0xbf368: strb r2, [r10, #0x24c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,588)?)?;
    // VMA 0xbf36c: str r0, [r10, #0x23c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,572)?)?;
    // VMA 0xbf370: ldr r0, [r9, #0x780]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1920)?)?;
    // VMA 0xbf374: str r0, [r10, #0x254]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,596)?)?;
    // VMA 0xbf378: strb r1, [r10, #0x258]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,600)?)?;
    // VMA 0xbf37c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbf380: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbf384: str r3, [r12, #0x904]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,2308)?)?;
    // VMA 0xbf388: bl 0xc1b78 <entry+0x3008>  @ imm = #0x27e8
    c.b(C::Al,0xc1b78,true)?;
    // VMA 0xbf38c: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,0)?)?;
    // VMA 0xbf390: mov r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2048)?)?;
    // VMA 0xbf394: mov r3, #318767104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,318767104)?)?;
    // VMA 0xbf398: mov r4, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,120)?)?;
    // VMA 0xbf39c: orr r4, r4, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,2048)?)?;
    // VMA 0xbf3a0: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,8)?)?;
    // VMA 0xbf3a4: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbf3a8: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xbf3ac: str r3, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbf3b0: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,8)?)?;
    // VMA 0xbf3b4: orr r1, r4, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(4)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbf3b8: ldr r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbf3bc: tst r2, r7
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbf3c0: beq 0xbf3b8 <entry+0x848>   @ imm = #-0x10
    c.b(C::Eq,0xbf3b8,false)?;
    // VMA 0xbf3c4: ldr r1, [pc, #0xa5c]        @ 0x12b8 <entry+0x12b8>
    c.literal(R::R1,0xbfe28)?;
    // VMA 0xbf3c8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf3cc: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xbf3d0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90744
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf3d4: ldr r1, [pc, #0xa20]        @ 0x128c <entry+0x128c>
    c.literal(R::R1,0xbfdfc)?;
    // VMA 0xbf3d8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbf3dc: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90750
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbf3e0: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbf3e4: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // VMA 0xbf3e8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbf3ec: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xbf3f0: orr r2, r0, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbf3f4: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xbf3f8: lsl r1, r1, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,15)?)?;
    // VMA 0xbf3fc: ldr r2, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R0,Shift::Lsl,0)?)?;
    c.check(0xbf400)
}
