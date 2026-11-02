
/// Initialize hardware codec frame state and its associated command
/// descriptors, including the conditional alternate configuration. This
/// routine's original tail is followed by 22 separately classified words.
fn initialize_hardware_frame_controller_profile(c:&mut Code)->Result<(),String>{
    c.check(0xc09a0)?;
    // VMA 0xc09a0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc09a4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc09a8: sub sp, sp, #444
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,444)?)?;
    // VMA 0xc09ac: ldr r4, [pc, #0x7b0]        @ 0x25f4 <entry+0x25f4>
    c.literal(R::new(4)?,0xc1164)?;
    // VMA 0xc09b0: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xc09b4: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xc09b8: mov r1, #171
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,171)?)?;
    // VMA 0xc09bc: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc09c0: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // VMA 0xc09c4: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc09c8: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xc09cc: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbf224
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc09d0: mov r1, #428
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,428)?)?;
    // VMA 0xc09d4: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc09d8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xc09dc: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xc09e0: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc09e4: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xc09e8: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbf240
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc09ec: bl 0x26754 <entry+0xfffffffffff67be4> @ imm = #-0x9a2a0
    c.b(C::Al,0x26754,true)?;
    // VMA 0xc09f0: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // VMA 0xc09f4: ldr r5, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,8)?)?;
    // VMA 0xc09f8: add r1, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xc09fc: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc0a00: add r1, r7, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xc0a04: ldr r2, [r1, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,360)?)?;
    // VMA 0xc0a08: ldm r2, {r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R2,R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc0a0c: ldr r9, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R2,0)?)?;
    // VMA 0xc0a10: str r5, [r2, #0x36c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,876)?)?;
    // VMA 0xc0a14: ldr r4, [pc, #0x74c]        @ 0x25f8 <entry+0x25f8>
    c.literal(R::new(4)?,0xc1168)?;
    // VMA 0xc0a18: beq 0xc0c04 <entry+0x2094>  @ imm = #0x1e4
    c.b(C::Eq,0xc0c04,false)?;
    // VMA 0xc0a1c: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // VMA 0xc0a20: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // VMA 0xc0a24: str r2, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,36)?)?;
    // VMA 0xc0a28: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // VMA 0xc0a2c: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // VMA 0xc0a30: mov r6, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1004)?)?;
    // VMA 0xc0a34: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // VMA 0xc0a38: ldr r0, [r7, #0x150]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,336)?)?;
    // VMA 0xc0a3c: orr r6, r6, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,1024)?)?;
    // VMA 0xc0a40: str r0, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,52)?)?;
    // VMA 0xc0a44: str r7, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,24)?)?;
    // VMA 0xc0a48: ldr r0, [r7, #0x158]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,344)?)?;
    // VMA 0xc0a4c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xc0a50: str r0, [r9, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,60)?)?;
    // VMA 0xc0a54: add r0, r5, #220
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,220)?)?;
    // VMA 0xc0a58: add r8, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R0,3072)?)?;
    // VMA 0xc0a5c: add r0, r5, #292
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,292)?)?;
    // VMA 0xc0a60: add r4, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,3072)?)?;
    // VMA 0xc0a64: add r1, r5, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0a68: ldr r0, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc0a6c: ldr r2, [r1, #0xcc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,3264)?)?;
    // VMA 0xc0a70: str r2, [r0, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,112)?)?;
    // VMA 0xc0a74: ldr r2, [r1, #0xcbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,3260)?)?;
    // VMA 0xc0a78: str r2, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,108)?)?;
    // VMA 0xc0a7c: ldr r2, [r1, #0xccc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,3276)?)?;
    // VMA 0xc0a80: str r2, [r0, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,296)?)?;
    // VMA 0xc0a84: ldr r0, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc0a88: ldr r2, [r1, #0xcd0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,3280)?)?;
    // VMA 0xc0a8c: str r2, [r0, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,300)?)?;
    // VMA 0xc0a90: ldr r0, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc0a94: ldr r2, [r1, #0xcd4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,3284)?)?;
    // VMA 0xc0a98: str r2, [r0, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,304)?)?;
    // VMA 0xc0a9c: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // VMA 0xc0aa0: ldr r0, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc0aa4: ldr r1, [r1, #0xcd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,3288)?)?;
    // VMA 0xc0aa8: str r1, [r0, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,308)?)?;
    // VMA 0xc0aac: add r1, r8, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(8)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0ab0: ldr r0, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc0ab4: add r0, r0, #124
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,124)?)?;
    // VMA 0xc0ab8: bl 0x471c <entry+0xfffffffffff45bac> @ imm = #-0xbc3a4
    c.b(C::Al,0x471c,true)?;
    // VMA 0xc0abc: ldr r0, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc0ac0: add r1, r4, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(4)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0ac4: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // VMA 0xc0ac8: add r0, r0, #196
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,196)?)?;
    // VMA 0xc0acc: bl 0x471c <entry+0xfffffffffff45bac> @ imm = #-0xbc3b8
    c.b(C::Al,0x471c,true)?;
    // VMA 0xc0ad0: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // VMA 0xc0ad4: add r6, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,4)?)?;
    // VMA 0xc0ad8: add r7, r7, #176
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,176)?)?;
    // VMA 0xc0adc: cmp r10, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,3)?)?;
    // VMA 0xc0ae0: bne 0xc0a64 <entry+0x1ef4>  @ imm = #-0x84
    c.b(C::Ne,0xc0a64,false)?;
    // VMA 0xc0ae4: ldr r8, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,36)?)?;
    // VMA 0xc0ae8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0aec: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc0af0: bl 0xc11bc <entry+0x264c>  @ imm = #0x6c4
    c.b(C::Al,0xc11bc,true)?;
    // VMA 0xc0af4: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc0af8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0afc: bl 0xcea18 <entry+0xfea8>  @ imm = #0xdf14
    c.b(C::Al,0xcea18,true)?;
    // VMA 0xc0b00: ldmib r5, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xc0b04: ldr r2, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,0)?)?;
    // VMA 0xc0b08: ldr r7, [r9, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,464)?)?;
    // VMA 0xc0b0c: ldr r6, [r9, #0xaa4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,2724)?)?;
    // VMA 0xc0b10: add r4, r5, #2080
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(5)?,2080)?)?;
    // VMA 0xc0b14: ldr r3, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,16)?)?;
    // VMA 0xc0b18: ldr r2, [r5, #0x20c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,524)?)?;
    // VMA 0xc0b1c: stm sp, {r4, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc0b20: str r6, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // VMA 0xc0b24: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xc0b28: movne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc0b2c: bl 0x278fc <entry+0xfffffffffff68d8c> @ imm = #-0x99238
    c.b(C::Al,0x278fc,true)?;
    // VMA 0xc0b30: ldr r1, [r5, #0x20c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,524)?)?;
    // VMA 0xc0b34: ldr r2, [r9, #0xaa0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,2720)?)?;
    // VMA 0xc0b38: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xc0b3c: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // VMA 0xc0b40: stm sp, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc0b44: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc0b48: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc0b4c: add r1, r5, #2256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,2256)?)?;
    // VMA 0xc0b50: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc0b54: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc0b58: bl 0x3027c <entry+0xfffffffffff7170c> @ imm = #-0x908e4
    c.b(C::Al,0x3027c,true)?;
    // VMA 0xc0b5c: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,16)?)?;
    // VMA 0xc0b60: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xc0b64: bne 0xc0b90 <entry+0x2020>  @ imm = #0x24
    c.b(C::Ne,0xc0b90,false)?;
    // VMA 0xc0b68: ldr r0, [r5, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1296)?)?;
    // VMA 0xc0b6c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc0b70: beq 0xc0b80 <entry+0x2010>  @ imm = #0x8
    c.b(C::Eq,0xc0b80,false)?;
    // VMA 0xc0b74: ldrb r0, [r5, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,1355)?)?;
    // VMA 0xc0b78: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc0b7c: beq 0xc1148 <entry+0x25d8>  @ imm = #0x5c4
    c.b(C::Eq,0xc1148,false)?;
    // VMA 0xc0b80: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc0b84: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0b88: bl 0xc7870 <entry+0x8d00>  @ imm = #0x6ce0
    c.b(C::Al,0xc7870,true)?;
    // VMA 0xc0b8c: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,16)?)?;
    // VMA 0xc0b90: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xc0b94: beq 0xc0cac <entry+0x213c>  @ imm = #0x110
    c.b(C::Eq,0xc0cac,false)?;
    // VMA 0xc0b98: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xc0b9c: bne 0xc0cc4 <entry+0x2154>  @ imm = #0x120
    c.b(C::Ne,0xc0cc4,false)?;
    // VMA 0xc0ba0: ldr r0, [r5, #0x818]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2072)?)?;
    // VMA 0xc0ba4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc0ba8: bne 0xc0cc4 <entry+0x2154>  @ imm = #0x114
    c.b(C::Ne,0xc0cc4,false)?;
    // VMA 0xc0bac: ldr r0, [r5, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1296)?)?;
    // VMA 0xc0bb0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc0bb4: ldrbeq r0, [r5, #0x54b]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(5)?,1355)?)?;
    // VMA 0xc0bb8: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc0bbc: bne 0xc0cc4 <entry+0x2154>  @ imm = #0x100
    c.b(C::Ne,0xc0cc4,false)?;
    // VMA 0xc0bc0: add r4, r8, #476
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,476)?)?;
    // VMA 0xc0bc4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc0bc8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc0bcc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc0bd0: beq 0xc0bd8 <entry+0x2068>  @ imm = #0x0
    c.b(C::Eq,0xc0bd8,false)?;
    // VMA 0xc0bd4: bl 0x7fed0 <entry+0xfffffffffffc1360> @ imm = #-0x40d0c
    c.b(C::Al,0x7fed0,true)?;
    // VMA 0xc0bd8: ldr r0, [r5, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1284)?)?;
    // VMA 0xc0bdc: bl 0x7ffc8 <entry+0xfffffffffffc1458> @ imm = #-0x40c1c
    c.b(C::Al,0x7ffc8,true)?;
    // VMA 0xc0be0: ldr r0, [r5, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1284)?)?;
    // VMA 0xc0be4: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xc0be8: cmp r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,4)?)?;
    // VMA 0xc0bec: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc0bf0: ldr r0, [r5, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1280)?)?;
    // VMA 0xc0bf4: str r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // VMA 0xc0bf8: add r4, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4)?)?;
    // VMA 0xc0bfc: bne 0xc0bc8 <entry+0x2058>  @ imm = #-0x3c
    c.b(C::Ne,0xc0bc8,false)?;
    // VMA 0xc0c00: b 0xc0cc4 <entry+0x2154>  @ imm = #0xbc
    c.b(C::Al,0xc0cc4,false)?;
    // VMA 0xc0c04: ldr r1, [pc, #0x5a4]        @ 0x2640 <entry+0x2640>
    c.literal(R::R1,0xc11b0)?;
    // VMA 0xc0c08: ldr r2, [pc, #0x58c]        @ 0x262c <entry+0x262c>
    c.literal(R::R2,0xc119c)?;
    // VMA 0xc0c0c: mov r3, #187
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,187)?)?;
    // VMA 0xc0c10: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc0c14: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // VMA 0xc0c18: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91f8c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0c1c: ldr r8, [pc, #0x54c]        @ 0x2600 <entry+0x2600>
    c.literal(R::new(8)?,0xc1170)?;
    // VMA 0xc0c20: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc0c24: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xc0c28: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91f9c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0c2c: ldr r5, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xc0c30: ldr r1, [pc, #0x57c]        @ 0x2644 <entry+0x2644>
    c.literal(R::R1,0xc11b4)?;
    // VMA 0xc0c34: ldr r9, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(9)?,0)?)?;
    // VMA 0xc0c38: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0c3c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xc0c40: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91fb4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0c44: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0c48: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xc0c4c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91fc0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0c50: mov r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,36)?)?;
    // VMA 0xc0c54: add r1, sp, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,48)?)?;
    // VMA 0xc0c58: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // VMA 0xc0c5c: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // VMA 0xc0c60: str r5, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,44)?)?;
    // VMA 0xc0c64: stm r1, {r0, r4, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(4)?.bit()|R::new(9)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc0c68: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc0c6c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc0c70: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,60)?)?;
    // VMA 0xc0c74: str r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,68)?)?;
    // VMA 0xc0c78: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // VMA 0xc0c7c: add r0, r7, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,200)?)?;
    // VMA 0xc0c80: add r1, sp, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,40)?)?;
    // VMA 0xc0c84: bl 0x80928 <entry+0xfffffffffffc1db8> @ imm = #-0x40364
    c.b(C::Al,0x80928,true)?;
    // VMA 0xc0c88: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc0c8c: beq 0xc1140 <entry+0x25d0>  @ imm = #0x4ac
    c.b(C::Eq,0xc1140,false)?;
    // VMA 0xc0c90: ldr r1, [pc, #0x520]        @ 0x2648 <entry+0x2648>
    c.literal(R::R1,0xc11b8)?;
    // VMA 0xc0c94: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0c98: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9200c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0c9c: ldr r1, [pc, #0x4cc]        @ 0x2600 <entry+0x2600>
    c.literal(R::R1,0xc1170)?;
    // VMA 0xc0ca0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0ca4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92018
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0ca8: b 0xc1140 <entry+0x25d0>  @ imm = #0x490
    c.b(C::Al,0xc1140,false)?;
    // VMA 0xc0cac: ldr r0, [r5, #0x818]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2072)?)?;
    // VMA 0xc0cb0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc0cb4: ldreq r0, [r5, #0x504]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(5)?,1284)?)?;
    // VMA 0xc0cb8: streq r0, [r8, #0x1cc]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(8)?,460)?)?;
    // VMA 0xc0cbc: ldreq r0, [r5, #0x500]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(5)?,1280)?)?;
    // VMA 0xc0cc0: streq r0, [r8, #0x1d4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(8)?,468)?)?;
    // VMA 0xc0cc4: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc0cc8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0ccc: bl 0xcf7a8 <entry+0x10c38> @ imm = #0xead4
    c.b(C::Al,0xcf7a8,true)?;
    // VMA 0xc0cd0: ldr r0, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,52)?)?;
    // VMA 0xc0cd4: bl 0x293c <entry+0xfffffffffff43dcc> @ imm = #-0xbe3a0
    c.b(C::Al,0x293c,true)?;
    // VMA 0xc0cd8: ldr r1, [pc, #0x48c]        @ 0x25fc <entry+0x25fc>
    c.literal(R::R1,0xc116c)?;
    // VMA 0xc0cdc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc0ce0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92054
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0ce4: ldr r1, [pc, #0x484]        @ 0x2600 <entry+0x2600>
    c.literal(R::R1,0xc1170)?;
    // VMA 0xc0ce8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc0cec: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92060
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0cf0: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xc0cf4: mov r1, #2384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2384)?)?;
    // VMA 0xc0cf8: mov r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,256)?)?;
    // VMA 0xc0cfc: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xc0d00: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xc0d04: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc0d08: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc0d0c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc0d10: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xc0d14: bl 0xc1f90 <entry+0x3420>  @ imm = #0x1274
    c.b(C::Al,0xc1f90,true)?;
    // VMA 0xc0d18: mov r1, #3440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3440)?)?;
    // VMA 0xc0d1c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc0d20: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc0d24: orr r1, r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3758096384)?)?;
    // VMA 0xc0d28: ldr r3, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,4)?)?;
    // VMA 0xc0d2c: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xc0d30: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // VMA 0xc0d34: str r0, [r1, r3, lsl #15]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(3)?,Shift::Lsl,15)?)?;
    // VMA 0xc0d38: ldr r3, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,4)?)?;
    // VMA 0xc0d3c: add r3, r1, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::new(3)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc0d40: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // VMA 0xc0d44: str r0, [r3, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,16)?)?;
    // VMA 0xc0d48: bne 0xc0d28 <entry+0x21b8>  @ imm = #-0x28
    c.b(C::Ne,0xc0d28,false)?;
    // VMA 0xc0d4c: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc0d50: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0d54: bl 0xc2874 <entry+0x3d04>  @ imm = #0x1b18
    c.b(C::Al,0xc2874,true)?;
    // VMA 0xc0d58: bl 0x26754 <entry+0xfffffffffff67be4> @ imm = #-0x9a60c
    c.b(C::Al,0x26754,true)?;
    // VMA 0xc0d5c: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xc0d60: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xc0d64: ldr r10, [pc, #0x408]       @ 0x2604 <entry+0x2604>
    c.literal(R::new(10)?,0xc1174)?;
    // VMA 0xc0d68: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc0d6c: str r1, [r10, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xc0d70: ldr r1, [pc, #0x400]        @ 0x2608 <entry+0x2608>
    c.literal(R::R1,0xc1178)?;
    // VMA 0xc0d74: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0d78: ldr r3, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,4)?)?;
    // VMA 0xc0d7c: ldr r2, [r5, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,392)?)?;
    // VMA 0xc0d80: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x920f4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0d84: ldr r6, [pc, #0x3e4]        @ 0x2600 <entry+0x2600>
    c.literal(R::new(6)?,0xc1170)?;
    // VMA 0xc0d88: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0d8c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc0d90: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92104
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0d94: bl 0x26754 <entry+0xfffffffffff67be4> @ imm = #-0x9a648
    c.b(C::Al,0x26754,true)?;
    // VMA 0xc0d98: ldr r7, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,32)?)?;
    // VMA 0xc0d9c: ldr r3, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,20)?)?;
    // VMA 0xc0da0: ldr r2, [r5, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,392)?)?;
    // VMA 0xc0da4: sub r1, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc0da8: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // VMA 0xc0dac: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xc0db0: ldr r1, [pc, #0x3c4]        @ 0x260c <entry+0x260c>
    c.literal(R::R1,0xc117c)?;
    // VMA 0xc0db4: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xc0db8: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9212c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0dbc: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xc0dc0: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc0dc4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92138
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0dc8: ldr r0, [r9, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,484)?)?;
    // VMA 0xc0dcc: ldr r1, [r9, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,488)?)?;
    // VMA 0xc0dd0: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // VMA 0xc0dd4: ldr r0, [pc, #0x3a4]        @ 0x2610 <entry+0x2610>
    c.literal(R::R0,0xc1180)?;
    // VMA 0xc0dd8: mov r1, #1424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1424)?)?;
    // VMA 0xc0ddc: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc0de0: mla r3, r2, r1, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R2,R::R1,R::R0)?)?;
    // VMA 0xc0de4: ldr r0, [r5, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,80)?)?;
    // VMA 0xc0de8: ldr r1, [pc, #0x394]        @ 0x2614 <entry+0x2614>
    c.literal(R::R1,0xc1184)?;
    // VMA 0xc0dec: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xc0df0: add r4, r1, r0, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R1,R::R0,Shift::Lsl,5,false)?)?;
    // VMA 0xc0df4: cmp r3, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0df8: movhi r4, r3
    c.emit(arm32::mov_register(C::Hi,R::new(4)?,R::new(3)?))?;
    // VMA 0xc0dfc: bl 0x3070 <entry+0xfffffffffff44500> @ imm = #-0xbdd94
    c.b(C::Al,0x3070,true)?;
    // VMA 0xc0e00: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // VMA 0xc0e04: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xc0e08: bl 0x266a8 <entry+0xfffffffffff67b38> @ imm = #-0x9a768
    c.b(C::Al,0x266a8,true)?;
    // VMA 0xc0e0c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc0e10: bl 0xc3bec <entry+0x507c>  @ imm = #0x2dd4
    c.b(C::Al,0xc3bec,true)?;
    // VMA 0xc0e14: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc0e18: bl 0xc1d08 <entry+0x3198>  @ imm = #0xee8
    c.b(C::Al,0xc1d08,true)?;
    // VMA 0xc0e1c: bl 0x3074 <entry+0xfffffffffff44504> @ imm = #-0xbddb0
    c.b(C::Al,0x3074,true)?;
    // VMA 0xc0e20: ldr r0, [r5, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,108)?)?;
    // VMA 0xc0e24: ldr r3, [r5, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,112)?)?;
    // VMA 0xc0e28: ldr r2, [r5, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,392)?)?;
    // VMA 0xc0e2c: ldr r1, [pc, #0x354]        @ 0x2618 <entry+0x2618>
    c.literal(R::R1,0xc1188)?;
    // VMA 0xc0e30: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xc0e34: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0e38: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x921ac
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0e3c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc0e40: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc0e44: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x921b8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0e48: ldr r2, [r5, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,392)?)?;
    // VMA 0xc0e4c: ldr r1, [pc, #0x338]        @ 0x261c <entry+0x261c>
    c.literal(R::R1,0xc118c)?;
    // VMA 0xc0e50: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc0e54: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x921c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0e58: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc0e5c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc0e60: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x921d4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0e64: ldr r0, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,52)?)?;
    // VMA 0xc0e68: bl 0x294c <entry+0xfffffffffff43ddc> @ imm = #-0xbe524
    c.b(C::Al,0x294c,true)?;
    // VMA 0xc0e6c: ldr r1, [pc, #0x31c]        @ 0x2620 <entry+0x2620>
    c.literal(R::R1,0xc1190)?;
    // VMA 0xc0e70: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc0e74: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x921e8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0e78: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc0e7c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc0e80: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x921f4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0e84: bl 0x26754 <entry+0xfffffffffff67be4> @ imm = #-0x9a738
    c.b(C::Al,0x26754,true)?;
    // VMA 0xc0e88: sub r1, r8, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc0e8c: ldr r3, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,20)?)?;
    // VMA 0xc0e90: ldr r2, [r5, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,392)?)?;
    // VMA 0xc0e94: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // VMA 0xc0e98: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xc0e9c: ldr r1, [pc, #0x2f0]        @ 0x2624 <entry+0x2624>
    c.literal(R::R1,0xc1194)?;
    // VMA 0xc0ea0: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xc0ea4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92218
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0ea8: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xc0eac: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc0eb0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92224
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0eb4: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xc0eb8: ldr r1, [r10, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(10)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xc0ebc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc0ec0: beq 0xc0f14 <entry+0x23a4>  @ imm = #0x4c
    c.b(C::Eq,0xc0f14,false)?;
    // VMA 0xc0ec4: bl 0x28520 <entry+0xfffffffffff699b0> @ imm = #-0x989ac
    c.b(C::Al,0x28520,true)?;
    // VMA 0xc0ec8: str r0, [r5, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,388)?)?;
    // VMA 0xc0ecc: ldr r2, [pc, #0x2c8]        @ 0x262c <entry+0x262c>
    c.literal(R::R2,0xc119c)?;
    // VMA 0xc0ed0: mov r3, #134
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,134)?)?;
    // VMA 0xc0ed4: ldr r1, [r5, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,60)?)?;
    // VMA 0xc0ed8: orr r3, r3, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // VMA 0xc0edc: orr r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8)?)?;
    // VMA 0xc0ee0: str r1, [r5, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,60)?)?;
    // VMA 0xc0ee4: ldr r1, [pc, #0x2ac]        @ 0x2628 <entry+0x2628>
    c.literal(R::R1,0xc1198)?;
    // VMA 0xc0ee8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xc0eec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc0ef0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92264
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0ef4: ldr r1, [pc, #0x274]        @ 0x2600 <entry+0x2600>
    c.literal(R::R1,0xc1170)?;
    // VMA 0xc0ef8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc0efc: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92270
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0f00: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,20)?)?;
    // VMA 0xc0f04: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xc0f08: ldr r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,16)?)?;
    // VMA 0xc0f0c: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xc0f10: b 0xc0ffc <entry+0x248c>  @ imm = #0xe4
    c.b(C::Al,0xc0ffc,false)?;
    // VMA 0xc0f14: mov r1, #3648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3648)?)?;
    // VMA 0xc0f18: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xc0f1c: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc0f20: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc0f24: tst r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,1)?)?;
    // VMA 0xc0f28: beq 0xc0f20 <entry+0x23b0>  @ imm = #-0x10
    c.b(C::Eq,0xc0f20,false)?;
    // VMA 0xc0f2c: ldr r0, [r5, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,392)?)?;
    // VMA 0xc0f30: ldr r1, [pc, #0x268]        @ 0x2630 <entry+0x2630>
    c.literal(R::R1,0xc11a0)?;
    // VMA 0xc0f34: ldr r2, [pc, #0x260]        @ 0x262c <entry+0x262c>
    c.literal(R::R2,0xc119c)?;
    // VMA 0xc0f38: mov r3, #94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,94)?)?;
    // VMA 0xc0f3c: orr r3, r3, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // VMA 0xc0f40: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xc0f44: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc0f48: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x922bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0f4c: ldr r1, [pc, #0x21c]        @ 0x2600 <entry+0x2600>
    c.literal(R::R1,0xc1170)?;
    // VMA 0xc0f50: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc0f54: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x922c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0f58: ldr r4, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,36)?)?;
    // VMA 0xc0f5c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0f60: add r2, r4, #688
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,688)?)?;
    // VMA 0xc0f64: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc0f68: bl 0xc8274 <entry+0x9704>  @ imm = #0x7304
    c.b(C::Al,0xc8274,true)?;
    // VMA 0xc0f6c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc0f70: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0f74: bl 0xc1d68 <entry+0x31f8>  @ imm = #0xdec
    c.b(C::Al,0xc1d68,true)?;
    // VMA 0xc0f78: ldrb r0, [r5, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,1312)?)?;
    // VMA 0xc0f7c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc0f80: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,20)?)?;
    // VMA 0xc0f84: ldr r10, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,16)?)?;
    // VMA 0xc0f88: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc0f8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc0f90: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc0f94: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc0f98: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xc0f9c: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xc0fa0: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc0fa4: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc0fa8: bne 0xc0fb4 <entry+0x2444>  @ imm = #0x4
    c.b(C::Ne,0xc0fb4,false)?;
    // VMA 0xc0fac: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xc0fb0: bne 0xc0fe8 <entry+0x2478>  @ imm = #0x30
    c.b(C::Ne,0xc0fe8,false)?;
    // VMA 0xc0fb4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc0fb8: bl 0xc3c0c <entry+0x509c>  @ imm = #0x2c4c
    c.b(C::Al,0xc3c0c,true)?;
    // VMA 0xc0fbc: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc0fc0: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xc0fc4: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xc0fc8: bne 0xc0fe8 <entry+0x2478>  @ imm = #0x18
    c.b(C::Ne,0xc0fe8,false)?;
    // VMA 0xc0fcc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc0fd0: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0fd4: bl 0xcfdb0 <entry+0x11240> @ imm = #0xedd4
    c.b(C::Al,0xcfdb0,true)?;
    // VMA 0xc0fd8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc0fdc: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0fe0: bl 0xc7314 <entry+0x87a4>  @ imm = #0x632c
    c.b(C::Al,0xc7314,true)?;
    // VMA 0xc0fe4: b 0xc0ff4 <entry+0x2484>  @ imm = #0x8
    c.b(C::Al,0xc0ff4,false)?;
    // VMA 0xc0fe8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc0fec: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0ff0: bl 0xc797c <entry+0x8e0c>  @ imm = #0x6984
    c.b(C::Al,0xc797c,true)?;
    // VMA 0xc0ff4: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xc0ff8: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xc0ffc: ldr r1, [r5, #0x20c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,524)?)?;
    // VMA 0xc1000: ldr r3, [r0, #0x5e4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,1508)?)?;
    // VMA 0xc1004: ldr r2, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,16)?)?;
    // VMA 0xc1008: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xc100c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xc1010: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc1014: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc1018: beq 0xc102c <entry+0x24bc>  @ imm = #0xc
    c.b(C::Eq,0xc102c,false)?;
    // VMA 0xc101c: ldr r7, [r5, #0x5c0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,1472)?)?;
    // VMA 0xc1020: add r7, r7, r7, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    // VMA 0xc1024: add r3, r3, r7, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,5,false)?)?;
    // VMA 0xc1028: b 0xc1030 <entry+0x24c0>  @ imm = #0x0
    c.b(C::Al,0xc1030,false)?;
    // VMA 0xc102c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc1030: ldr r7, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,24)?)?;
    // VMA 0xc1034: bl 0x27ec0 <entry+0xfffffffffff69350> @ imm = #-0x9917c
    c.b(C::Al,0x27ec0,true)?;
    // VMA 0xc1038: ldr r2, [r5, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,88)?)?;
    // VMA 0xc103c: ldr r1, [pc, #0x160]        @ 0x2634 <entry+0x2634>
    c.literal(R::R1,0xc11a4)?;
    // VMA 0xc1040: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1044: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x923b8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1048: ldr r4, [pc, #0x120]        @ 0x2600 <entry+0x2600>
    c.literal(R::new(4)?,0xc1170)?;
    // VMA 0xc104c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1050: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xc1054: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x923c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1058: ldr r3, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,4)?)?;
    // VMA 0xc105c: ldr r2, [r5, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,392)?)?;
    // VMA 0xc1060: ldr r1, [pc, #0x140]        @ 0x2638 <entry+0x2638>
    c.literal(R::R1,0xc11a8)?;
    // VMA 0xc1064: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc1068: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x923dc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc106c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc1070: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xc1074: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x923e8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1078: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,16)?)?;
    // VMA 0xc107c: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xc1080: bne 0xc1094 <entry+0x2524>  @ imm = #0xc
    c.b(C::Ne,0xc1094,false)?;
    // VMA 0xc1084: ldr r0, [r5, #0x504]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1284)?)?;
    // VMA 0xc1088: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc108c: beq 0xc1094 <entry+0x2524>  @ imm = #0x0
    c.b(C::Eq,0xc1094,false)?;
    // VMA 0xc1090: bl 0x7fed0 <entry+0xfffffffffffc1360> @ imm = #-0x411c8
    c.b(C::Al,0x7fed0,true)?;
    // VMA 0xc1094: ldr r0, [r5, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1276)?)?;
    // VMA 0xc1098: bl 0x7fed0 <entry+0xfffffffffffc1360> @ imm = #-0x411d0
    c.b(C::Al,0x7fed0,true)?;
    // VMA 0xc109c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc10a0: mov r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
    // VMA 0xc10a4: str r0, [r5, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1276)?)?;
    // VMA 0xc10a8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc10ac: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // VMA 0xc10b0: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // VMA 0xc10b4: mov r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,36)?)?;
    // VMA 0xc10b8: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // VMA 0xc10bc: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // VMA 0xc10c0: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // VMA 0xc10c4: str r10, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,52)?)?;
    // VMA 0xc10c8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc10cc: str r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // VMA 0xc10d0: ldr r0, [r5, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,392)?)?;
    // VMA 0xc10d4: str r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // VMA 0xc10d8: ldr r0, [r5, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,60)?)?;
    // VMA 0xc10dc: lsl r0, r0, #28
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,28)?)?;
    // VMA 0xc10e0: and r0, r1, r0, asr #31
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Asr,31,false)?)?;
    // VMA 0xc10e4: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,60)?)?;
    // VMA 0xc10e8: ldr r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // VMA 0xc10ec: bl 0x28520 <entry+0xfffffffffff699b0> @ imm = #-0x98bd4
    c.b(C::Al,0x28520,true)?;
    // VMA 0xc10f0: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // VMA 0xc10f4: add r0, r7, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,200)?)?;
    // VMA 0xc10f8: add r1, sp, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,40)?)?;
    // VMA 0xc10fc: str r8, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,72)?)?;
    // VMA 0xc1100: bl 0x80928 <entry+0xfffffffffffc1db8> @ imm = #-0x407e0
    c.b(C::Al,0x80928,true)?;
    // VMA 0xc1104: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1108: bne 0xc1138 <entry+0x25c8>  @ imm = #0x28
    c.b(C::Ne,0xc1138,false)?;
    // VMA 0xc110c: ldr r1, [pc, #0x98]         @ 0x263c <entry+0x263c>
    c.literal(R::R1,0xc11ac)?;
    // VMA 0xc1110: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc1114: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92488
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1118: ldr r1, [pc, #0x50]         @ 0x2600 <entry+0x2600>
    c.literal(R::R1,0xc1170)?;
    // VMA 0xc111c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc1120: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92494
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1124: ldr r2, [pc, #0x38]         @ 0x25f4 <entry+0x25f4>
    c.literal(R::R2,0xc1164)?;
    // VMA 0xc1128: mov r1, #213
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,213)?)?;
    // VMA 0xc112c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc1130: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // VMA 0xc1134: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbf98c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc1138: ldr r0, [r5, #0x4f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1268)?)?;
    // VMA 0xc113c: bl 0x7fed0 <entry+0xfffffffffffc1360> @ imm = #-0x41274
    c.b(C::Al,0x7fed0,true)?;
    // VMA 0xc1140: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc1144: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc1148: ldrb r0, [r5, #0x522]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,1314)?)?;
    // VMA 0xc114c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1150: beq 0xc0ba0 <entry+0x2030>  @ imm = #-0x5b8
    c.b(C::Eq,0xc0ba0,false)?;
    // VMA 0xc1154: ldrb r0, [r5, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,1312)?)?;
    // VMA 0xc1158: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc115c: bne 0xc0ba0 <entry+0x2030>  @ imm = #-0x5c4
    c.b(C::Ne,0xc0ba0,false)?;
    // VMA 0xc1160: b 0xc0b80 <entry+0x2010>  @ imm = #-0x5e8
    c.b(C::Al,0xc0b80,false)?;
    c.check(0xc1164)?;
    const CODEC_FRAME_PROFILE_DIAGNOSTIC_DATA:[u32;22]=[
      0x000fa71c,0x00121001,0x000fa8e9,0x000e2788,0x00104090,
      0x000fa909,0x000fa94c,0x00989680,0x044aa200,0x000fa979,
      0x000fa9bb,0x000fa9ec,0x000faa1a,0x000faa8b,0x000fa8d4,
      0x000faa50,0x000faabf,0x000faae1,0x000fab1f,0x000fa8ac,
      0x000f596c,0x000f5954,
    ];
    for word in CODEC_FRAME_PROFILE_DIAGNOSTIC_DATA {c.emit(word)?;}
    c.check(0xc11bc)
}
