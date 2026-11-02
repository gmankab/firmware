
/// References to controller-global slots, resource lengths, and runtime
/// warnings emitted as a non-executable, PC-relative literal table.
const DMA_WINDOW_CONTROLLER_METADATA: [u32;28]=[
    0x000f9c35,0x000f9c64,0x000e2788,0x000f9ad3,
    0x000f9daa,0x000f9c7a,0x000f9d31,0x000f9cf9,
    0x000f9cb9,0x000f9d69,0x004c4b40,0x000f9e11,
    0x000f9e19,0x000f9e09,0x000f9e05,0x000f9e07,
    0x000f9e03,0x000f9deb,0x00104090,0x000f9e1d,
    0x000f9e38,0x000f9e57,0x000f9e91,0x000f9eb7,
    0x000f9eeb,0x000f9f22,0x00020103,0x000f9b5e,
];

/// Publish a codec frame into its assigned controller slot, update the
/// outstanding FIFO counters, and trigger the platform events that process
/// codec data and feedback. The original function returns via POP/PC.
fn push_stream_record_to_controller(c:&mut Code)->Result<(),String>{
    c.check(0xbb078)?;
    // VMA 0xbb078: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbb07c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xbb080: sub sp, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // VMA 0xbb084: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xbb088: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xbb08c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbb090: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // VMA 0xbb094: str r0, [r6, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1296)?)?;
    // VMA 0xbb098: ldr r1, [r6, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,972)?)?;
    // VMA 0xbb09c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbb0a0: beq 0xbb248 <entry+0x1d0>   @ imm = #0x1a0
    c.b(C::Eq,0xbb248,false)?;
    // VMA 0xbb0a4: ldr r7, [pc, #0x1ac]        @ 0x1e0 <entry+0x1e0>
    c.literal(R::new(7)?,0xbb258)?;
    // VMA 0xbb0a8: mov r9, #248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,248)?)?;
    // VMA 0xbb0ac: add r1, r6, #104
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,104)?)?;
    // VMA 0xbb0b0: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // VMA 0xbb0b4: add r8, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R1,1024)?)?;
    // VMA 0xbb0b8: orr r9, r9, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,1024)?)?;
    // VMA 0xbb0bc: b 0xbb0ec <entry+0x74>    @ imm = #0x28
    c.b(C::Al,0xbb0ec,false)?;
    // VMA 0xbb0c0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbb0c4: bne 0xbb240 <entry+0x1c8>   @ imm = #0x174
    c.b(C::Ne,0xbb240,false)?;
    // VMA 0xbb0c8: ldrb r2, [r4, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,2140)?)?;
    // VMA 0xbb0cc: ldr r1, [pc, #0x1a0]        @ 0x1fc <entry+0x1fc>
    c.literal(R::R1,0xbb274)?;
    // VMA 0xbb0d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbb0d4: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c448
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb0d8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbb0dc: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbb0e0: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c454
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb0e4: strb r10, [r4, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::new(4)?,2140)?)?;
    // VMA 0xbb0e8: ldr r0, [r6, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1296)?)?;
    // VMA 0xbb0ec: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xbb0f0: str r0, [r6, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,1296)?)?;
    // VMA 0xbb0f4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xbb0f8: ldr r1, [r4, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,364)?)?;
    // VMA 0xbb0fc: bl 0xb9c64 <entry+0xffffffffffffebec> @ imm = #-0x14a0
    c.b(C::Al,0xb9c64,true)?;
    // VMA 0xbb100: ldr r0, [r6, #0xaa4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2724)?)?;
    // VMA 0xbb104: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,8)?)?;
    // VMA 0xbb108: ldr r3, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,12)?)?;
    // VMA 0xbb10c: ldr r1, [pc, #0x140]        @ 0x1dc <entry+0x1dc>
    c.literal(R::R1,0xbb254)?;
    // VMA 0xbb110: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbb114: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbb118: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c48c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb11c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbb120: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbb124: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c498
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb128: ldr r3, [r6, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,464)?)?;
    // VMA 0xbb12c: ldr r2, [r6, #0xaa4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,2724)?)?;
    // VMA 0xbb130: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xbb134: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // VMA 0xbb138: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // VMA 0xbb13c: str r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,4)?)?;
    // VMA 0xbb140: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // VMA 0xbb144: bl 0x27ca4 <entry+0xfffffffffff6cc2c> @ imm = #-0x934a8
    c.b(C::Al,0x27ca4,true)?;
    // VMA 0xbb148: ldr r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    // VMA 0xbb14c: bl 0x293c <entry+0xfffffffffff478c4> @ imm = #-0xb8818
    c.b(C::Al,0x293c,true)?;
    // VMA 0xbb150: ldr r1, [pc, #0x104]        @ 0x1e4 <entry+0x1e4>
    c.literal(R::R1,0xbb25c)?;
    // VMA 0xbb154: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbb158: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c4cc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb15c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbb160: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbb164: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c4d8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb168: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xbb16c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xbb170: bl 0xbc7dc <entry+0x1764>  @ imm = #0x1664
    c.b(C::Al,0xbc7dc,true)?;
    // VMA 0xbb174: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xbb178: mov r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10240)?)?;
    // VMA 0xbb17c: mov r3, #98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,98)?)?;
    // VMA 0xbb180: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xbb184: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbb188: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbb18c: str r10, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(10)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbb190: ldr r0, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,48)?)?;
    // VMA 0xbb194: ldr r2, [pc, #0xc4]         @ 0x1e8 <entry+0x1e8>
    c.literal(R::R2,0xbb260)?;
    // VMA 0xbb198: mla r1, r0, r3, r2
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::R0,R::new(3)?,R::R2)?)?;
    // VMA 0xbb19c: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xbb1a0: bl 0x266a8 <entry+0xfffffffffff6b630> @ imm = #-0x94b00
    c.b(C::Al,0x266a8,true)?;
    // VMA 0xbb1a4: ldr r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    // VMA 0xbb1a8: bl 0x294c <entry+0xfffffffffff478d4> @ imm = #-0xb8864
    c.b(C::Al,0x294c,true)?;
    // VMA 0xbb1ac: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xbb1b0: ldr r1, [pc, #0xac]         @ 0x1ec <entry+0x1ec>
    c.literal(R::R1,0xbb264)?;
    // VMA 0xbb1b4: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbb1b8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbb1bc: bne 0xbb1f4 <entry+0x17c>   @ imm = #0x30
    c.b(C::Ne,0xbb1f4,false)?;
    // VMA 0xbb1c0: ldr r1, [pc, #0xa4]         @ 0x1f4 <entry+0x1f4>
    c.literal(R::R1,0xbb26c)?;
    // VMA 0xbb1c4: ldr r2, [pc, #0xa4]         @ 0x1f8 <entry+0x1f8>
    c.literal(R::R2,0xbb270)?;
    // VMA 0xbb1c8: mov r3, #165
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,165)?)?;
    // VMA 0xbb1cc: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbb1d0: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // VMA 0xbb1d4: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c548
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb1d8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbb1dc: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbb1e0: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c554
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb1e4: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xbb1e8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbb1ec: bl 0x276ac <entry+0xfffffffffff6c634> @ imm = #-0x93b48
    c.b(C::Al,0x276ac,true)?;
    // VMA 0xbb1f0: b 0xbb20c <entry+0x194>   @ imm = #0x14
    c.b(C::Al,0xbb20c,false)?;
    // VMA 0xbb1f4: ldr r1, [pc, #0x6c]         @ 0x1f0 <entry+0x1f0>
    c.literal(R::R1,0xbb268)?;
    // VMA 0xbb1f8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbb1fc: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c570
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb200: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbb204: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbb208: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8c57c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbb20c: ldrb r1, [r4, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2140)?)?;
    // VMA 0xbb210: ldr r0, [r4, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,364)?)?;
    // VMA 0xbb214: add r1, r1, r1, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,5,false)?)?;
    // VMA 0xbb218: add r0, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xbb21c: add r1, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb220: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xbb224: bl 0x280f0 <entry+0xfffffffffff6d078> @ imm = #-0x9313c
    c.b(C::Al,0x280f0,true)?;
    // VMA 0xbb228: ldr r1, [r5, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,732)?)?;
    // VMA 0xbb22c: ldr r0, [r6, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1296)?)?;
    // VMA 0xbb230: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // VMA 0xbb234: beq 0xbb0c0 <entry+0x48>    @ imm = #-0x17c
    c.b(C::Eq,0xbb0c0,false)?;
    // VMA 0xbb238: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbb23c: beq 0xbb248 <entry+0x1d0>   @ imm = #0x4
    c.b(C::Eq,0xbb248,false)?;
    // VMA 0xbb240: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbb244: strb r0, [r4, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2140)?)?;
    // VMA 0xbb248: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbb24c: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbb250: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xbb254)
}

/// Controller globals, logging identifiers and packet-class constants.
/// All nine words are literal/data, not ARM instructions.
const FRAME_CONTROLLER_RELEASE_LITERALS:[u32;9]=[
    0x000f4f76,0x000e2788,0x000f4f9e,
    0x004c4b40,0x00104090,0x000f9fc4,
    0x000f9f8f,0x000f9faa,0x000f9fd3,
];

/// Build hardware codec auxiliary NAL/plane records, including the
/// guarded reconstruction of active input surfaces, DMA copies and a
/// bounded list of frame descriptors. The original return is POP/PC.
fn synthesize_codec_auxiliary_records(c:&mut Code)->Result<(),String>{
    c.check(0xbb278)?;
    // VMA 0xbb278: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbb27c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xbb280: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // VMA 0xbb284: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // VMA 0xbb288: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // VMA 0xbb28c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbb290: ldr r0, [r9, #0xd0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,208)?)?;
    // VMA 0xbb294: ldr r4, [r9, #0x320]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(9)?,800)?)?;
    // VMA 0xbb298: ldr r7, [pc, #0x2cc]        @ 0x4f4 <entry+0x4f4>
    c.literal(R::new(7)?,0xbb56c)?;
    // VMA 0xbb29c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbb2a0: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // VMA 0xbb2a4: beq 0xbb3f0 <entry+0x378>   @ imm = #0x144
    c.b(C::Eq,0xbb3f0,false)?;
    // VMA 0xbb2a8: ldr r1, [r8, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,136)?)?;
    // VMA 0xbb2ac: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xbb2b0: bne 0xbb3f0 <entry+0x378>   @ imm = #0x138
    c.b(C::Ne,0xbb3f0,false)?;
    // VMA 0xbb2b4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbb2b8: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // VMA 0xbb2bc: add r2, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,12)?)?;
    // VMA 0xbb2c0: str r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // VMA 0xbb2c4: str r5, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    // VMA 0xbb2c8: bl 0x80134 <entry+0xfffffffffffc50bc> @ imm = #-0x3b19c
    c.b(C::Al,0x80134,true)?;
    // VMA 0xbb2cc: ldr r0, [r9, #0xd0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,208)?)?;
    // VMA 0xbb2d0: add r1, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,20)?)?;
    // VMA 0xbb2d4: add r2, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,24)?)?;
    // VMA 0xbb2d8: bl 0x801a8 <entry+0xfffffffffffc5130> @ imm = #-0x3b138
    c.b(C::Al,0x801a8,true)?;
    // VMA 0xbb2dc: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xbb2e0: sub r1, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb2e4: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xbb2e8: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // VMA 0xbb2ec: ldrb r2, [r8, #0x45d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(8)?,1117)?)?;
    // VMA 0xbb2f0: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // VMA 0xbb2f4: tst r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,64)?)?;
    // VMA 0xbb2f8: beq 0xbb3ec <entry+0x374>   @ imm = #0xec
    c.b(C::Eq,0xbb3ec,false)?;
    // VMA 0xbb2fc: ldr r2, [r9, #0x328]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,808)?)?;
    // VMA 0xbb300: add r2, r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,24)?)?;
    // VMA 0xbb304: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbb308: blo 0xbb3ec <entry+0x374>   @ imm = #0xdc
    c.b(C::Cc,0xbb3ec,false)?;
    // VMA 0xbb30c: add r6, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb310: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // VMA 0xbb314: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xbb318: bl 0x205c <entry+0xfffffffffff46fe4> @ imm = #-0xb92c4
    c.b(C::Al,0x205c,true)?;
    // VMA 0xbb31c: ldr r1, [r9, #0x324]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,804)?)?;
    // VMA 0xbb320: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbb324: beq 0xbb3ec <entry+0x374>   @ imm = #0xc0
    c.b(C::Eq,0xbb3ec,false)?;
    // VMA 0xbb328: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xbb32c: add r4, r9, #808
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(9)?,808)?)?;
    // VMA 0xbb330: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbb334: rsb r10, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(10)?,R::R0,4096)?)?;
    // VMA 0xbb338: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xbb33c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xbb340: str r7, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,4)?)?;
    // VMA 0xbb344: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xbb348: add r0, r10, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbb34c: str r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,8)?)?;
    // VMA 0xbb350: mov r2, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,14)?)?;
    // VMA 0xbb354: str r2, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,12)?)?;
    // VMA 0xbb358: ldr r2, [r9, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,732)?)?;
    // VMA 0xbb35c: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xbb360: moveq r3, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // VMA 0xbb364: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // VMA 0xbb368: moveq r3, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // VMA 0xbb36c: str r3, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,20)?)?;
    // VMA 0xbb370: ldr r2, [r8, #0xd0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,208)?)?;
    // VMA 0xbb374: ldr r7, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,0)?)?;
    // VMA 0xbb378: sub r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbb37c: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbb380: movhi r7, r0
    c.emit(arm32::mov_register(C::Hi,R::new(7)?,R::R0))?;
    // VMA 0xbb384: add r0, r6, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,24)?)?;
    // VMA 0xbb388: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xbb38c: bl 0x471c <entry+0xfffffffffff496a4> @ imm = #-0xb6c78
    c.b(C::Al,0x471c,true)?;
    // VMA 0xbb390: add r0, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,4)?)?;
    // VMA 0xbb394: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xbb398: str r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xbb39c: add r0, r7, #27
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,27)?)?;
    // VMA 0xbb3a0: bic r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,3)?)?;
    // VMA 0xbb3a4: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xbb3a8: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // VMA 0xbb3ac: sub r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbb3b0: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // VMA 0xbb3b4: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // VMA 0xbb3b8: ldr r2, [r9, #0x320]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,800)?)?;
    // VMA 0xbb3bc: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbb3c0: add r2, r6, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,20)?)?;
    // VMA 0xbb3c4: str r1, [r9, #0x320]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,800)?)?;
    // VMA 0xbb3c8: add r1, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,7)?)?;
    // VMA 0xbb3cc: bic r1, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,3)?)?;
    // VMA 0xbb3d0: ldr r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,8)?)?;
    // VMA 0xbb3d4: add r6, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbb3d8: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbb3dc: add r2, r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,12)?)?;
    // VMA 0xbb3e0: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // VMA 0xbb3e4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbb3e8: bne 0xbb33c <entry+0x2c4>   @ imm = #-0xb4
    c.b(C::Ne,0xbb33c,false)?;
    // VMA 0xbb3ec: ldr r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,4)?)?;
    // VMA 0xbb3f0: ldr r0, [r9, #0xd4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,212)?)?;
    // VMA 0xbb3f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbb3f8: beq 0xbb538 <entry+0x4c0>   @ imm = #0x138
    c.b(C::Eq,0xbb538,false)?;
    // VMA 0xbb3fc: ldr r1, [r8, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,972)?)?;
    // VMA 0xbb400: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xbb404: bne 0xbb538 <entry+0x4c0>   @ imm = #0x12c
    c.b(C::Ne,0xbb538,false)?;
    // VMA 0xbb408: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbb40c: add r2, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,12)?)?;
    // VMA 0xbb410: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // VMA 0xbb414: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // VMA 0xbb418: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // VMA 0xbb41c: bl 0x80134 <entry+0xfffffffffffc50bc> @ imm = #-0x3b2f0
    c.b(C::Al,0x80134,true)?;
    // VMA 0xbb420: ldr r0, [r9, #0xd4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,212)?)?;
    // VMA 0xbb424: add r1, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,20)?)?;
    // VMA 0xbb428: add r2, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,24)?)?;
    // VMA 0xbb42c: bl 0x801a8 <entry+0xfffffffffffc5130> @ imm = #-0x3b28c
    c.b(C::Al,0x801a8,true)?;
    // VMA 0xbb430: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xbb434: sub r1, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb438: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xbb43c: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // VMA 0xbb440: ldrb r2, [r8, #0x45d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(8)?,1117)?)?;
    // VMA 0xbb444: tst r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,64)?)?;
    // VMA 0xbb448: beq 0xbb538 <entry+0x4c0>   @ imm = #0xe8
    c.b(C::Eq,0xbb538,false)?;
    // VMA 0xbb44c: ldr r2, [r9, #0x328]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,808)?)?;
    // VMA 0xbb450: add r2, r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,24)?)?;
    // VMA 0xbb454: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbb458: blo 0xbb538 <entry+0x4c0>   @ imm = #0xd8
    c.b(C::Cc,0xbb538,false)?;
    // VMA 0xbb45c: add r6, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb460: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // VMA 0xbb464: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xbb468: bl 0x205c <entry+0xfffffffffff46fe4> @ imm = #-0xb9414
    c.b(C::Al,0x205c,true)?;
    // VMA 0xbb46c: ldr r1, [r9, #0x324]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,804)?)?;
    // VMA 0xbb470: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbb474: beq 0xbb538 <entry+0x4c0>   @ imm = #0xbc
    c.b(C::Eq,0xbb538,false)?;
    // VMA 0xbb478: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xbb47c: add r10, r9, #816
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(9)?,816)?)?;
    // VMA 0xbb480: rsb r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xbb484: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xbb488: str r7, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,4)?)?;
    // VMA 0xbb48c: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xbb490: add r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbb494: str r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,8)?)?;
    // VMA 0xbb498: mov r2, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,14)?)?;
    // VMA 0xbb49c: str r2, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,12)?)?;
    // VMA 0xbb4a0: ldr r2, [r9, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,732)?)?;
    // VMA 0xbb4a4: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xbb4a8: moveq r3, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // VMA 0xbb4ac: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // VMA 0xbb4b0: moveq r3, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // VMA 0xbb4b4: str r3, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,20)?)?;
    // VMA 0xbb4b8: ldr r2, [r8, #0xd0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,208)?)?;
    // VMA 0xbb4bc: ldr r7, [r10, #-0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,-8)?)?;
    // VMA 0xbb4c0: sub r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbb4c4: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbb4c8: movhi r7, r0
    c.emit(arm32::mov_register(C::Hi,R::new(7)?,R::R0))?;
    // VMA 0xbb4cc: add r0, r6, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,24)?)?;
    // VMA 0xbb4d0: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xbb4d4: bl 0x471c <entry+0xfffffffffff496a4> @ imm = #-0xb6dc0
    c.b(C::Al,0x471c,true)?;
    // VMA 0xbb4d8: add r0, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,4)?)?;
    // VMA 0xbb4dc: add r2, r6, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,20)?)?;
    // VMA 0xbb4e0: str r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xbb4e4: add r0, r7, #27
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,27)?)?;
    // VMA 0xbb4e8: bic r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,3)?)?;
    // VMA 0xbb4ec: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xbb4f0: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // VMA 0xbb4f4: sub r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbb4f8: add r1, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,7)?)?;
    // VMA 0xbb4fc: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // VMA 0xbb500: bic r1, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,3)?)?;
    // VMA 0xbb504: ldr r3, [r8, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,136)?)?;
    // VMA 0xbb508: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xbb50c: bne 0xbb524 <entry+0x4ac>   @ imm = #0x10
    c.b(C::Ne,0xbb524,false)?;
    // VMA 0xbb510: ldr r3, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xbb514: ldr r7, [r9, #0x320]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,800)?)?;
    // VMA 0xbb518: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xbb51c: add r3, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbb520: str r3, [r9, #0x320]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,800)?)?;
    // VMA 0xbb524: add r6, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbb528: ldr r1, [r10], #12
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R1,R::new(10)?,12)?)?;
    // VMA 0xbb52c: ldr r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,8)?)?;
    // VMA 0xbb530: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbb534: bne 0xbb484 <entry+0x40c>   @ imm = #-0xb8
    c.b(C::Ne,0xbb484,false)?;
    // VMA 0xbb538: add r0, r5, r5, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(5)?,Shift::Lsl,1,false)?)?;
    // VMA 0xbb53c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbb540: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xbb544: add r0, r0, #804
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,804)?)?;
    // VMA 0xbb548: sub r2, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(5)?,1)?)?;
    // VMA 0xbb54c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xbb550: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // VMA 0xbb554: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xbb558: sub r0, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,12)?)?;
    // VMA 0xbb55c: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // VMA 0xbb560: bne 0xbb548 <entry+0x4d0>   @ imm = #-0x20
    c.b(C::Ne,0xbb548,false)?;
    // VMA 0xbb564: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbb568: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xbb56c)
}

/// One packed firmware record-format code (non-executable constant).
const PACKED_CODEC_RECORD_FORMAT:u32=0x0101_0200;
