
/// Publish controller frame-plane memory, configure allocated resources,
/// copy codec plane tables to MMIO, and submit the callback. Its argument
/// setup and diagnostic constants are source-encoded A32 and explicit data.
fn publish_runtime_codec_frame_resources(c:&mut Code)->Result<(),String>{
    c.check(0xc2874)?;
    // VMA 0xc2874: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc2878: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc287c: sub sp, sp, #84
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,84)?)?;
    // VMA 0xc2880: mov r10, r1
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // VMA 0xc2884: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // VMA 0xc2888: ldr r0, [r10, #0x5d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1492)?)?;
    // VMA 0xc288c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc2890: bne 0xc28a0 <entry+0x3d30>  @ imm = #0x8
    c.b(C::Ne,0xc28a0,false)?;
    // VMA 0xc2894: ldr r0, [r10, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1496)?)?;
    // VMA 0xc2898: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc289c: beq 0xc2a5c <entry+0x3eec>  @ imm = #0x1b8
    c.b(C::Eq,0xc2a5c,false)?;
    // VMA 0xc28a0: ldrb r0, [r10, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1312)?)?;
    // VMA 0xc28a4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc28a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc28ac: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc28b0: ldr r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,48)?)?;
    // VMA 0xc28b4: ldr r5, [r10, #0x6f8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,1784)?)?;
    // VMA 0xc28b8: ldr r9, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(10)?,4)?)?;
    // VMA 0xc28bc: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // VMA 0xc28c0: ldr r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc28c4: ldr r7, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,112)?)?;
    // VMA 0xc28c8: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xc28cc: ldr r12, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R1,0)?)?;
    // VMA 0xc28d0: orr r0, r0, r9, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(9)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc28d4: add r3, r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,7)?)?;
    // VMA 0xc28d8: add r7, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,7)?)?;
    // VMA 0xc28dc: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xc28e0: ldr r6, [r12, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R12,16)?)?;
    // VMA 0xc28e4: cmp r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,4096)?)?;
    // VMA 0xc28e8: mov r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // VMA 0xc28ec: moveq r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // VMA 0xc28f0: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc28f4: movne r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xc28f8: orr r5, r6, r5, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(6)?,R::new(5)?,Shift::Lsl,8,false)?)?;
    // VMA 0xc28fc: mov r6, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // VMA 0xc2900: str r5, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc2904: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc2908: ldr r4, [r10, #0xb8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,184)?)?;
    // VMA 0xc290c: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // VMA 0xc2910: orr r5, r0, r9, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R0,R::new(9)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc2914: bic r0, r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(3)?,7)?)?;
    // VMA 0xc2918: bic r3, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(7)?,7)?)?;
    // VMA 0xc291c: mov r7, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,458752)?)?;
    // VMA 0xc2920: mov lr, r0
    c.emit(arm32::mov_register(C::Al,R::LR,R::R0))?;
    // VMA 0xc2924: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc2928: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xc292c: moveq r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xc2930: lsl r4, r4, #22
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(4)?,Shift::Lsl,22)?)?;
    // VMA 0xc2934: orr r2, r4, r2, lsl #20
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(4)?,R::R2,Shift::Lsl,20,false)?)?;
    // VMA 0xc2938: mov r4, #3670016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3670016)?)?;
    // VMA 0xc293c: str r2, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc2940: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // VMA 0xc2944: orr r5, r3, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(3)?,R::R0,Shift::Lsl,16,false)?)?;
    // VMA 0xc2948: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // VMA 0xc294c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc2950: orr r2, r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    // VMA 0xc2954: orr r2, r2, r9, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(9)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc2958: str r5, [r2, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc295c: ldr r2, [r10, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,192)?)?;
    // VMA 0xc2960: and r3, r4, r2, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(4)?,R::R2,Shift::Lsl,19,false)?)?;
    // VMA 0xc2964: and r2, r7, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(7)?,R::R2,Shift::Lsl,16,false)?)?;
    // VMA 0xc2968: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc296c: ldr r3, [r12, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R12,16)?)?;
    // VMA 0xc2970: cmp r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4096)?)?;
    // VMA 0xc2974: moveq r0, #7
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xc2978: orr r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc297c: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // VMA 0xc2980: orr r2, r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    // VMA 0xc2984: orr r0, r0, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4194304)?)?;
    // VMA 0xc2988: orr r2, r2, r9, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(9)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc298c: str r0, [r2, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc2990: mov r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6144)?)?;
    // VMA 0xc2994: orr r0, r0, r9, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(9)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc2998: str r5, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc299c: ldr r0, [r10, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,192)?)?;
    // VMA 0xc29a0: ldr r2, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1296)?)?;
    // VMA 0xc29a4: and r3, r7, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::R0,Shift::Lsl,16,false)?)?;
    // VMA 0xc29a8: and r0, r4, r0, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::R0,Shift::Lsl,19,false)?)?;
    // VMA 0xc29ac: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xc29b0: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // VMA 0xc29b4: movne r4, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xc29b8: orr r0, r0, r4, lsl #22
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,22,false)?)?;
    // VMA 0xc29bc: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc29c0: ldr r3, [r12, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R12,16)?)?;
    // VMA 0xc29c4: cmp r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4096)?)?;
    // VMA 0xc29c8: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc29cc: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc29d0: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xc29d4: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // VMA 0xc29d8: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xc29dc: orr r1, r1, r9, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(9)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc29e0: str r0, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc29e4: str r9, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,4)?)?;
    // VMA 0xc29e8: mov r6, r12
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R12))?;
    // VMA 0xc29ec: lsl r9, r9, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::new(9)?,Shift::Lsl,15)?)?;
    // VMA 0xc29f0: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // VMA 0xc29f4: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xc29f8: bne 0xc2a30 <entry+0x3ec0>  @ imm = #0x30
    c.b(C::Ne,0xc2a30,false)?;
    // VMA 0xc29fc: ldrb r0, [r10, #0x508]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1288)?)?;
    // VMA 0xc2a00: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc2a04: mov r12, lr
    c.emit(arm32::mov_register(C::Al,R::R12,R::LR))?;
    // VMA 0xc2a08: ldr lr, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-36)?)?;
    // VMA 0xc2a0c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc2a10: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc2a14: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xc2a18: lsl r3, r1, #19
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R1,Shift::Lsl,19)?)?;
    // VMA 0xc2a1c: moveq r8, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xc2a20: orr r1, r3, r1, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,12,false)?)?;
    // VMA 0xc2a24: orr r0, r1, r8, lsl #9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::new(8)?,Shift::Lsl,9,false)?)?;
    // VMA 0xc2a28: mov r1, #112
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,112)?)?;
    // VMA 0xc2a2c: b 0xc2aa0 <entry+0x3f30>  @ imm = #0x6c
    c.b(C::Al,0xc2aa0,false)?;
    // VMA 0xc2a30: ldrb r0, [r10, #0x56e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1390)?)?;
    // VMA 0xc2a34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc2a38: beq 0xc2a64 <entry+0x3ef4>  @ imm = #0x24
    c.b(C::Eq,0xc2a64,false)?;
    // VMA 0xc2a3c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc2a40: beq 0xc2a70 <entry+0x3f00>  @ imm = #0x28
    c.b(C::Eq,0xc2a70,false)?;
    // VMA 0xc2a44: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // VMA 0xc2a48: mov r12, lr
    c.emit(arm32::mov_register(C::Al,R::R12,R::LR))?;
    // VMA 0xc2a4c: bne 0xc2a80 <entry+0x3f10>  @ imm = #0x2c
    c.b(C::Ne,0xc2a80,false)?;
    // VMA 0xc2a50: mov r0, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20480)?)?;
    // VMA 0xc2a54: mov r1, #2621440
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2621440)?)?;
    // VMA 0xc2a58: b 0xc2a94 <entry+0x3f24>  @ imm = #0x34
    c.b(C::Al,0xc2a94,false)?;
    // VMA 0xc2a5c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc2a60: b 0xc28b0 <entry+0x3d40>  @ imm = #-0x1b8
    c.b(C::Al,0xc28b0,false)?;
    // VMA 0xc2a64: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // VMA 0xc2a68: mov r1, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1048576)?)?;
    // VMA 0xc2a6c: b 0xc2a78 <entry+0x3f08>  @ imm = #0x4
    c.b(C::Al,0xc2a78,false)?;
    // VMA 0xc2a70: mov r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16384)?)?;
    // VMA 0xc2a74: mov r1, #2097152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2097152)?)?;
    // VMA 0xc2a78: mov r12, lr
    c.emit(arm32::mov_register(C::Al,R::R12,R::LR))?;
    // VMA 0xc2a7c: b 0xc2a94 <entry+0x3f24>  @ imm = #0x10
    c.b(C::Al,0xc2a94,false)?;
    // VMA 0xc2a80: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xc2a84: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc2a88: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc2a8c: moveq r0, #12288
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,12288)?)?;
    // VMA 0xc2a90: moveq r1, #1572864
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1572864)?)?;
    // VMA 0xc2a94: ldr lr, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-36)?)?;
    // VMA 0xc2a98: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc2a9c: mov r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // VMA 0xc2aa0: orr r3, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(9)?,12)?)?;
    // VMA 0xc2aa4: orr r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc2aa8: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xc2aac: orr r7, r9, #6272
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(9)?,6272)?)?;
    // VMA 0xc2ab0: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,6144)?)?;
    // VMA 0xc2ab4: str r1, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2ab8: orr r3, r9, #2192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(9)?,2192)?)?;
    // VMA 0xc2abc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc2ac0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xc2ac4: str r1, [r7, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2ac8: str r1, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2acc: ldr r1, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,16)?)?;
    // VMA 0xc2ad0: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // VMA 0xc2ad4: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // VMA 0xc2ad8: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // VMA 0xc2adc: bne 0xc2b64 <entry+0x3ff4>  @ imm = #0x80
    c.b(C::Ne,0xc2b64,false)?;
    // VMA 0xc2ae0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc2ae4: beq 0xc2b64 <entry+0x3ff4>  @ imm = #0x78
    c.b(C::Eq,0xc2b64,false)?;
    // VMA 0xc2ae8: ldrb r3, [r10, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,1355)?)?;
    // VMA 0xc2aec: mov r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16384)?)?;
    // VMA 0xc2af0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc2af4: bne 0xc2b64 <entry+0x3ff4>  @ imm = #0x68
    c.b(C::Ne,0xc2b64,false)?;
    // VMA 0xc2af8: add r1, r10, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,1004)?)?;
    // VMA 0xc2afc: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xc2b00: add r6, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,1024)?)?;
    // VMA 0xc2b04: ldr r1, [pc, #0x7b0]        @ 0x474c <entry+0x474c>
    c.literal(R::R1,0xc32bc)?;
    // VMA 0xc2b08: add r5, r9, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(9)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc2b0c: ldr r3, [r6], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::new(6)?,4)?)?;
    // VMA 0xc2b10: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xc2b14: cmp r4, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,3)?)?;
    // VMA 0xc2b18: ldr r7, [r3, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,108)?)?;
    // VMA 0xc2b1c: ldr r3, [r3, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,112)?)?;
    // VMA 0xc2b20: orr r3, r3, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc2b24: str r3, [r5], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::new(5)?,4)?)?;
    // VMA 0xc2b28: bne 0xc2b0c <entry+0x3f9c>  @ imm = #-0x24
    c.b(C::Ne,0xc2b0c,false)?;
    // VMA 0xc2b2c: add r1, r9, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc2b30: add r3, r10, #972
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(10)?,972)?)?;
    // VMA 0xc2b34: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc2b38: add r6, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(3)?,1024)?)?;
    // VMA 0xc2b3c: add r1, r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,12)?)?;
    // VMA 0xc2b40: ldmda r6, {r3, r4}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(6)?,R::new(3)?.bit()|R::new(4)?.bit(),arm32::BlockMode::DecrementAfter,false)?)?;
    // VMA 0xc2b44: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xc2b48: add r6, r6, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,8)?)?;
    // VMA 0xc2b4c: cmp r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,3)?)?;
    // VMA 0xc2b50: orr r3, r3, r4, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc2b54: str r3, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // VMA 0xc2b58: bne 0xc2b40 <entry+0x3fd0>  @ imm = #-0x20
    c.b(C::Ne,0xc2b40,false)?;
    // VMA 0xc2b5c: ldr r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,16)?)?;
    // VMA 0xc2b60: ldr r1, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,16)?)?;
    // VMA 0xc2b64: ldr r3, [r10, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,192)?)?;
    // VMA 0xc2b68: mov r4, #3670016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3670016)?)?;
    // VMA 0xc2b6c: mov r5, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,458752)?)?;
    // VMA 0xc2b70: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // VMA 0xc2b74: and r4, r4, r3, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(4)?,R::new(3)?,Shift::Lsl,19,false)?)?;
    // VMA 0xc2b78: and r3, r5, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc2b7c: orr r3, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2b80: mov r4, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,32)?)?;
    // VMA 0xc2b84: moveq r4, #28
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,28)?)?;
    // VMA 0xc2b88: orr r1, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2b8c: orr r3, r9, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(9)?,7168)?)?;
    // VMA 0xc2b90: orr r1, r1, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4194304)?)?;
    // VMA 0xc2b94: str r1, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2b98: orr r1, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(9)?,12)?)?;
    // VMA 0xc2b9c: orr r3, r12, lr, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R12,R::LR,Shift::Lsl,16,false)?)?;
    // VMA 0xc2ba0: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // VMA 0xc2ba4: str r3, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2ba8: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // VMA 0xc2bac: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xc2bb0: bne 0xc2bbc <entry+0x404c>  @ imm = #0x4
    c.b(C::Ne,0xc2bbc,false)?;
    // VMA 0xc2bb4: ldrb r1, [r10, #0x508]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1288)?)?;
    // VMA 0xc2bb8: b 0xc2bc8 <entry+0x4058>  @ imm = #0x8
    c.b(C::Al,0xc2bc8,false)?;
    // VMA 0xc2bbc: ldr r1, [r10, #0x50c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1292)?)?;
    // VMA 0xc2bc0: ldrb r3, [r10, #0x508]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,1288)?)?;
    // VMA 0xc2bc4: orr r1, r3, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xc2bc8: mov r3, #25165824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,25165824)?)?;
    // VMA 0xc2bcc: mov r7, #3670016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3670016)?)?;
    // VMA 0xc2bd0: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xc2bd4: and r1, r3, r1, lsl #23
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,23,false)?)?;
    // VMA 0xc2bd8: ldr r3, [r10, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,192)?)?;
    // VMA 0xc2bdc: and r7, r7, r3, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::new(3)?,Shift::Lsl,19,false)?)?;
    // VMA 0xc2be0: orr r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2be4: mov r7, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,458752)?)?;
    // VMA 0xc2be8: and r3, r7, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc2bec: orr r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2bf0: mov r3, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,192)?)?;
    // VMA 0xc2bf4: and r2, r3, r2, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,6,false)?)?;
    // VMA 0xc2bf8: mov r3, #-1073741824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3221225472)?)?;
    // VMA 0xc2bfc: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2c00: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // VMA 0xc2c04: moveq r2, #7
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,7)?)?;
    // VMA 0xc2c08: orr r0, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2c0c: orr r1, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,256)?)?;
    // VMA 0xc2c10: orr r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,8)?)?;
    // VMA 0xc2c14: orr r2, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,5120)?)?;
    // VMA 0xc2c18: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xc2c1c: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2c20: orr r1, r9, #1440
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(9)?,1440)?)?;
    // VMA 0xc2c24: ldr r2, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,16)?)?;
    // VMA 0xc2c28: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc2c2c: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // VMA 0xc2c30: moveq r3, #1073741824
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1073741824)?)?;
    // VMA 0xc2c34: str r3, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2c38: ldr r1, [r10, #0x5d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1492)?)?;
    // VMA 0xc2c3c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc2c40: bne 0xc2c50 <entry+0x40e0>  @ imm = #0x8
    c.b(C::Ne,0xc2c50,false)?;
    // VMA 0xc2c44: ldr r1, [r10, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1496)?)?;
    // VMA 0xc2c48: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc2c4c: beq 0xc2cf8 <entry+0x4188>  @ imm = #0xa4
    c.b(C::Eq,0xc2cf8,false)?;
    // VMA 0xc2c50: ldr r2, [r10, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,1776)?)?;
    // VMA 0xc2c54: mov r3, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,254)?)?;
    // VMA 0xc2c58: ldr r1, [r10, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1772)?)?;
    // VMA 0xc2c5c: orr r3, r3, #32512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,32512)?)?;
    // VMA 0xc2c60: and r2, r3, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,1,false)?)?;
    // VMA 0xc2c64: ldr r3, [pc, #0x654]        @ 0x4750 <entry+0x4750>
    c.literal(R::new(3)?,0xc32c0)?;
    // VMA 0xc2c68: and r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xc2c6c: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2c70: orr r2, r9, #40
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(9)?,40)?)?;
    // VMA 0xc2c74: orr r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1)?)?;
    // VMA 0xc2c78: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // VMA 0xc2c7c: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2c80: ldr r0, [r10, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1772)?)?;
    // VMA 0xc2c84: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc2c88: beq 0xc2cbc <entry+0x414c>  @ imm = #0x2c
    c.b(C::Eq,0xc2cbc,false)?;
    // VMA 0xc2c8c: add r0, r10, #476
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,476)?)?;
    // VMA 0xc2c90: orr r1, r9, #1056
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(9)?,1056)?)?;
    // VMA 0xc2c94: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc2c98: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xc2c9c: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xc2ca0: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc2ca4: ldr r7, [r0, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::R2,Shift::Lsl,2)?)?;
    // VMA 0xc2ca8: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xc2cac: str r7, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc2cb0: ldr r7, [r10, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,1772)?)?;
    // VMA 0xc2cb4: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2cb8: blo 0xc2ca4 <entry+0x4134>  @ imm = #-0x1c
    c.b(C::Cc,0xc2ca4,false)?;
    // VMA 0xc2cbc: ldr r0, [r10, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1776)?)?;
    // VMA 0xc2cc0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc2cc4: beq 0xc2cf8 <entry+0x4188>  @ imm = #0x2c
    c.b(C::Eq,0xc2cf8,false)?;
    // VMA 0xc2cc8: add r0, r10, #732
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,732)?)?;
    // VMA 0xc2ccc: orr r1, r9, #36
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(9)?,36)?)?;
    // VMA 0xc2cd0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc2cd4: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xc2cd8: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xc2cdc: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // VMA 0xc2ce0: ldr r7, [r0, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::R2,Shift::Lsl,2)?)?;
    // VMA 0xc2ce4: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xc2ce8: str r7, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc2cec: ldr r7, [r10, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,1776)?)?;
    // VMA 0xc2cf0: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2cf4: blo 0xc2ce0 <entry+0x4170>  @ imm = #-0x1c
    c.b(C::Cc,0xc2ce0,false)?;
    // VMA 0xc2cf8: add r0, r10, #904
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,904)?)?;
    // VMA 0xc2cfc: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xc2d00: mov r7, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3758096384)?)?;
    // VMA 0xc2d04: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc2d08: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xc2d0c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc2d10: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xc2d14: str r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-52)?)?;
    // VMA 0xc2d18: orr r0, r9, #116
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,116)?)?;
    // VMA 0xc2d1c: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xc2d20: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xc2d24: orr r0, r9, #104
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,104)?)?;
    // VMA 0xc2d28: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xc2d2c: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xc2d30: orr r0, r9, #972
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,972)?)?;
    // VMA 0xc2d34: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc2d38: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // VMA 0xc2d3c: orr r0, r9, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,3024)?)?;
    // VMA 0xc2d40: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xc2d44: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // VMA 0xc2d48: orr r0, r9, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,2896)?)?;
    // VMA 0xc2d4c: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xc2d50: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // VMA 0xc2d54: orr r0, r9, #852
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,852)?)?;
    // VMA 0xc2d58: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc2d5c: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // VMA 0xc2d60: orr r0, r9, #2400
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,2400)?)?;
    // VMA 0xc2d64: orr r12, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R0,12288)?)?;
    // VMA 0xc2d68: orr r0, r9, #356
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(9)?,356)?)?;
    // VMA 0xc2d6c: orr r9, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::R0,14336)?)?;
    // VMA 0xc2d70: ldr r0, [r10, #0x204]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,516)?)?;
    // VMA 0xc2d74: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // VMA 0xc2d78: orr r1, r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,128)?)?;
    // VMA 0xc2d7c: str r10, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R11,-32)?)?;
    // VMA 0xc2d80: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // VMA 0xc2d84: mov r1, #178
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,178)?)?;
    // VMA 0xc2d88: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc2d8c: str r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,56)?)?;
    // VMA 0xc2d90: add r1, r10, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(10)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2d94: str r3, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,52)?)?;
    // VMA 0xc2d98: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // VMA 0xc2d9c: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // VMA 0xc2da0: add r3, r1, #1968
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,1968)?)?;
    // VMA 0xc2da4: str r1, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-40)?)?;
    // VMA 0xc2da8: add r1, r1, #1936
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1936)?)?;
    // VMA 0xc2dac: str r1, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-48)?)?;
    // VMA 0xc2db0: ldr r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,48)?)?;
    // VMA 0xc2db4: str r3, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-44)?)?;
    // VMA 0xc2db8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc2dbc: add r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2dc0: add r6, r1, #180
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,180)?)?;
    // VMA 0xc2dc4: orr lr, r3, r8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::new(3)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2dc8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc2dcc: orr r5, lr, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::LR,4096)?)?;
    // VMA 0xc2dd0: str r5, [r9, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::new(9)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2dd4: bne 0xc2df8 <entry+0x4288>  @ imm = #0x1c
    c.b(C::Ne,0xc2df8,false)?;
    // VMA 0xc2dd8: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // VMA 0xc2ddc: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xc2de0: addeq r2, r1, #172
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R2,R::R1,172)?)?;
    // VMA 0xc2de4: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xc2de8: ldr r5, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,12)?)?;
    // VMA 0xc2dec: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xc2df0: sub r5, r5, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(5)?,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2df4: b 0xc2e3c <entry+0x42cc>  @ imm = #0x40
    c.b(C::Al,0xc2e3c,false)?;
    // VMA 0xc2df8: ldr r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xc2dfc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc2e00: ldr r2, [r2, #0x20c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,524)?)?;
    // VMA 0xc2e04: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xc2e08: bne 0xc2e3c <entry+0x42cc>  @ imm = #0x2c
    c.b(C::Ne,0xc2e3c,false)?;
    // VMA 0xc2e0c: ldr r2, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-48)?)?;
    // VMA 0xc2e10: ldr r5, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-52)?)?;
    // VMA 0xc2e14: ldr r4, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // VMA 0xc2e18: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // VMA 0xc2e1c: addeq r2, r5, r4
    c.emit(arm32::data_register(C::Eq,Op::Add,R::R2,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2e20: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xc2e24: str r2, [r12, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R12,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2e28: str lr, [r9, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::LR,R::new(9)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2e2c: ldr r2, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // VMA 0xc2e30: ldr r5, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-40)?)?;
    // VMA 0xc2e34: addeq r2, r5, #1952
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R2,R::new(5)?,1952)?)?;
    // VMA 0xc2e38: ldr r5, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,0)?)?;
    // VMA 0xc2e3c: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // VMA 0xc2e40: add r3, r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,16)?)?;
    // VMA 0xc2e44: str r5, [r12, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R12,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2e48: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // VMA 0xc2e4c: bne 0xc2dc4 <entry+0x4254>  @ imm = #-0x90
    c.b(C::Ne,0xc2dc4,false)?;
    // VMA 0xc2e50: ldr r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // VMA 0xc2e54: ldr r5, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,56)?)?;
    // VMA 0xc2e58: add r8, r8, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,65536)?)?;
    // VMA 0xc2e5c: str r5, [r2, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(5)?,R::R2,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2e60: ldr r3, [r1, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,172)?)?;
    // VMA 0xc2e64: ldr r10, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,36)?)?;
    // VMA 0xc2e68: ldr r6, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc2e6c: str r6, [r10, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::new(10)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2e70: ldr r6, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,40)?)?;
    // VMA 0xc2e74: str r5, [r6, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(5)?,R::new(6)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2e78: ldr r3, [r3, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,16)?)?;
    // VMA 0xc2e7c: bic lr, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::LR,R::new(3)?,255)?)?;
    // VMA 0xc2e80: ldr r3, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,44)?)?;
    // VMA 0xc2e84: str lr, [r3, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::LR,R::new(3)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2e88: add lr, r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(5)?,16)?)?;
    // VMA 0xc2e8c: add r5, r5, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,65536)?)?;
    // VMA 0xc2e90: str lr, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R2,0)?)?;
    // VMA 0xc2e94: ldr r1, [r1, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,180)?)?;
    // VMA 0xc2e98: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xc2e9c: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,0)?)?;
    // VMA 0xc2ea0: str lr, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(6)?,0)?)?;
    // VMA 0xc2ea4: ldr r1, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,16)?)?;
    // VMA 0xc2ea8: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xc2eac: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xc2eb0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc2eb4: ldr r3, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,52)?)?;
    // VMA 0xc2eb8: ldr r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // VMA 0xc2ebc: lsl r1, r1, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsl,R::new(3)?,false)?)?;
    // VMA 0xc2ec0: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xc2ec4: str r1, [r2, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2ec8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc2ecc: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // VMA 0xc2ed0: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // VMA 0xc2ed4: str r2, [r1, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2ed8: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // VMA 0xc2edc: str r2, [r1, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2ee0: str r5, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,56)?)?;
    // VMA 0xc2ee4: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // VMA 0xc2ee8: ldr r10, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-32)?)?;
    // VMA 0xc2eec: add r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
    // VMA 0xc2ef0: bne 0xc2d90 <entry+0x4220>  @ imm = #-0x168
    c.b(C::Ne,0xc2d90,false)?;
    // VMA 0xc2ef4: ldr r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // VMA 0xc2ef8: mov r5, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xc2efc: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xc2f00: str r1, [r2, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc2f04: ldrb r1, [r10, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1312)?)?;
    // VMA 0xc2f08: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc2f0c: bne 0xc2f84 <entry+0x4414>  @ imm = #0x70
    c.b(C::Ne,0xc2f84,false)?;
    // VMA 0xc2f10: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // VMA 0xc2f14: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc2f18: bhi 0xc2f74 <entry+0x4404>  @ imm = #0x54
    c.b(C::Hi,0xc2f74,false)?;
    // VMA 0xc2f1c: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xc2f20: ldr r2, [r10, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,192)?)?;
    // VMA 0xc2f24: ldr r3, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,8)?)?;
    // VMA 0xc2f28: add r0, r1, #268
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,268)?)?;
    // VMA 0xc2f2c: add r1, r1, #264
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,264)?)?;
    // VMA 0xc2f30: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc2f34: add r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2048)?)?;
    // VMA 0xc2f38: addne r1, r0, #2048
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R0,2048)?)?;
    // VMA 0xc2f3c: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xc2f40: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc2f44: orr r0, r2, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,2147483648)?)?;
    // VMA 0xc2f48: str r0, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc2f4c: ldr r1, [pc, #0x370]        @ 0x4754 <entry+0x4754>
    c.literal(R::R1,0xc32c4)?;
    // VMA 0xc2f50: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc2f54: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x942c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc2f58: ldr r1, [pc, #0x368]        @ 0x4758 <entry+0x4758>
    c.literal(R::R1,0xc32c8)?;
    // VMA 0xc2f5c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc2f60: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x942d4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc2f64: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // VMA 0xc2f68: ldr r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    // VMA 0xc2f6c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xc2f70: b 0xc2f8c <entry+0x441c>  @ imm = #0x14
    c.b(C::Al,0xc2f8c,false)?;
    // VMA 0xc2f74: ldr r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    // VMA 0xc2f78: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xc2f7c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc2f80: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2f84: ldr r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    // VMA 0xc2f88: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // VMA 0xc2f8c: ldr r1, [r0, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,972)?)?;
    // VMA 0xc2f90: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xc2f94: bl 0xc32cc <entry+0x475c>  @ imm = #0x330
    c.b(C::Al,0xc32cc,true)?;
    // VMA 0xc2f98: ldr r6, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,32)?)?;
    // VMA 0xc2f9c: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xc2fa0: str r0, [r6, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc2fa4: ldr r1, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,296)?)?;
    // VMA 0xc2fa8: ldr r7, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,36)?)?;
    // VMA 0xc2fac: str r1, [r7, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc2fb0: ldr r3, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,40)?)?;
    // VMA 0xc2fb4: str r0, [r3, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc2fb8: ldr r0, [r10, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,152)?)?;
    // VMA 0xc2fbc: ldr r2, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // VMA 0xc2fc0: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc2fc4: str r0, [r2, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc2fc8: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xc2fcc: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xc2fd0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc2fd4: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xc2fd8: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc2fdc: ldr r0, [r10, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,152)?)?;
    // VMA 0xc2fe0: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc2fe4: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xc2fe8: mov r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // VMA 0xc2fec: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc2ff0: ldr r1, [r10, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,300)?)?;
    // VMA 0xc2ff4: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xc2ff8: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc2ffc: ldr r0, [r10, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,156)?)?;

    // VMA 0xc3000: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc3004: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xc3008: mov r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // VMA 0xc300c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xc3010: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc3014: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xc3018: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc301c: ldr r0, [r10, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,156)?)?;
    // VMA 0xc3020: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc3024: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xc3028: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xc302c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc3030: ldr r1, [r10, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,304)?)?;
    // VMA 0xc3034: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xc3038: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc303c: ldr r0, [r10, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,176)?)?;
    // VMA 0xc3040: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc3044: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xc3048: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xc304c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xc3050: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc3054: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xc3058: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc305c: ldr r0, [r10, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,176)?)?;
    // VMA 0xc3060: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc3064: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xc3068: mov r0, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,23)?)?;
    // VMA 0xc306c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc3070: ldr r1, [r10, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,308)?)?;
    // VMA 0xc3074: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xc3078: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc307c: ldr r0, [r10, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,180)?)?;
    // VMA 0xc3080: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc3084: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xc3088: mov r0, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,23)?)?;
    // VMA 0xc308c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xc3090: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc3094: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xc3098: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc309c: ldr r0, [r10, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,180)?)?;
    // VMA 0xc30a0: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc30a4: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xc30a8: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xc30ac: bl 0xc3574 <entry+0x574>   @ imm = #0x4c0
    c.b(C::Al,0xc3574,true)?;
    // VMA 0xc30b0: ldr r1, [r10, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,128)?)?;
    // VMA 0xc30b4: ldr r2, [r10, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,140)?)?;
    // VMA 0xc30b8: orr r0, r4, #108
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(4)?,108)?)?;
    // VMA 0xc30bc: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc30c0: orr r1, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // VMA 0xc30c4: str r1, [r0, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc30c8: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // VMA 0xc30cc: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xc30d0: ldr r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // VMA 0xc30d4: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // VMA 0xc30d8: beq 0xc3160 <entry+0x160>   @ imm = #0x80
    c.b(C::Eq,0xc3160,false)?;
    // VMA 0xc30dc: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // VMA 0xc30e0: bne 0xc32b0 <entry+0x2b0>   @ imm = #0x1c8
    c.b(C::Ne,0xc32b0,false)?;
    // VMA 0xc30e4: ldr r1, [r10, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,128)?)?;
    // VMA 0xc30e8: ldr r2, [r10, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,140)?)?;
    // VMA 0xc30ec: orr r0, r4, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(4)?,124)?)?;
    // VMA 0xc30f0: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc30f4: orr r1, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // VMA 0xc30f8: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xc30fc: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc3100: orr r0, r4, #14464
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(4)?,14464)?)?;
    // VMA 0xc3104: ldr r1, [r10, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,128)?)?;
    // VMA 0xc3108: ldr r3, [r10, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,140)?)?;
    // VMA 0xc310c: orr r1, r1, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc3110: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc3114: orr r0, r4, #132
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(4)?,132)?)?;
    // VMA 0xc3118: ldr r1, [r10, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,128)?)?;
    // VMA 0xc311c: ldr r3, [r10, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,140)?)?;
    // VMA 0xc3120: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc3124: orr r1, r1, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc3128: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc312c: orr r0, r4, #76
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(4)?,76)?)?;
    // VMA 0xc3130: ldr r1, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,108)?)?;
    // VMA 0xc3134: ldr r3, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,112)?)?;
    // VMA 0xc3138: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc313c: orr r1, r3, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,16,false)?)?;
    // VMA 0xc3140: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc3144: orr r0, r4, #2128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(4)?,2128)?)?;
    // VMA 0xc3148: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xc314c: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc3150: orr r0, r4, #84
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(4)?,84)?)?;
    // VMA 0xc3154: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc3158: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc315c: b 0xc32b0 <entry+0x2b0>   @ imm = #0x14c
    c.b(C::Al,0xc32b0,false)?;
    // VMA 0xc3160: ldr r1, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2028)?)?;
    // VMA 0xc3164: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc3168: beq 0xc3198 <entry+0x198>   @ imm = #0x28
    c.b(C::Eq,0xc3198,false)?;
    // VMA 0xc316c: ldr r3, [r1, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,128)?)?;
    // VMA 0xc3170: ldr r7, [r1, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,140)?)?;
    // VMA 0xc3174: orr r2, r4, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(4)?,124)?)?;
    // VMA 0xc3178: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // VMA 0xc317c: orr r3, r3, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc3180: mov r7, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3758096384)?)?;
    // VMA 0xc3184: str r3, [r2, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc3188: ldr r2, [r1, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,108)?)?;
    // VMA 0xc318c: ldr r1, [r1, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,112)?)?;
    // VMA 0xc3190: orr r2, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // VMA 0xc3194: b 0xc31ac <entry+0x1ac>   @ imm = #0x10
    c.b(C::Al,0xc31ac,false)?;
    // VMA 0xc3198: orr r1, r4, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,124)?)?;
    // VMA 0xc319c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc31a0: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xc31a4: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xc31a8: str r2, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc31ac: orr r1, r4, #76
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,76)?)?;
    // VMA 0xc31b0: orr r3, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R1,14336)?)?;
    // VMA 0xc31b4: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc31b8: str r2, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc31bc: orr r3, r4, #14464
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(4)?,14464)?)?;
    // VMA 0xc31c0: ldr r2, [r10, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2032)?)?;
    // VMA 0xc31c4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc31c8: beq 0xc31ec <entry+0x1ec>   @ imm = #0x1c
    c.b(C::Eq,0xc31ec,false)?;
    // VMA 0xc31cc: ldr r7, [r2, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,128)?)?;
    // VMA 0xc31d0: ldr r6, [r2, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,140)?)?;
    // VMA 0xc31d4: orr r7, r7, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc31d8: str r7, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc31dc: ldr r1, [r2, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,108)?)?;
    // VMA 0xc31e0: ldr r2, [r2, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,112)?)?;
    // VMA 0xc31e4: orr r2, r2, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,16,false)?)?;
    // VMA 0xc31e8: b 0xc31f4 <entry+0x1f4>   @ imm = #0x4
    c.b(C::Al,0xc31f4,false)?;
    // VMA 0xc31ec: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc31f0: str r2, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc31f4: orr r1, r4, #2128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,2128)?)?;
    // VMA 0xc31f8: orr r3, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R1,12288)?)?;
    // VMA 0xc31fc: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc3200: str r2, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3204: ldr r2, [r10, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,2036)?)?;
    // VMA 0xc3208: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc320c: beq 0xc3238 <entry+0x238>   @ imm = #0x24
    c.b(C::Eq,0xc3238,false)?;
    // VMA 0xc3210: ldr r7, [r2, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R2,128)?)?;
    // VMA 0xc3214: ldr r6, [r2, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,140)?)?;
    // VMA 0xc3218: orr r3, r4, #132
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(4)?,132)?)?;
    // VMA 0xc321c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // VMA 0xc3220: orr r7, r7, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc3224: str r7, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3228: ldr r1, [r2, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,108)?)?;
    // VMA 0xc322c: ldr r2, [r2, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,112)?)?;
    // VMA 0xc3230: orr r2, r2, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,16,false)?)?;
    // VMA 0xc3234: b 0xc3248 <entry+0x248>   @ imm = #0xc
    c.b(C::Al,0xc3248,false)?;
    // VMA 0xc3238: orr r2, r4, #132
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(4)?,132)?)?;
    // VMA 0xc323c: orr r3, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R2,14336)?)?;
    // VMA 0xc3240: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc3244: str r2, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3248: orr r1, r4, #84
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,84)?)?;
    // VMA 0xc324c: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xc3250: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xc3254: str r2, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc3258: ldr r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // VMA 0xc325c: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // VMA 0xc3260: bne 0xc32b0 <entry+0x2b0>   @ imm = #0x48
    c.b(C::Ne,0xc32b0,false)?;
    // VMA 0xc3264: ldr r1, [r0, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,972)?)?;
    // VMA 0xc3268: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc326c: beq 0xc32b0 <entry+0x2b0>   @ imm = #0x3c
    c.b(C::Eq,0xc32b0,false)?;
    // VMA 0xc3270: ldr r1, [r0, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2308)?)?;
    // VMA 0xc3274: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc3278: bne 0xc32b0 <entry+0x2b0>   @ imm = #0x30
    c.b(C::Ne,0xc32b0,false)?;
    // VMA 0xc327c: ldrb r1, [r0, #0x496]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,1174)?)?;
    // VMA 0xc3280: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc3284: bne 0xc32b0 <entry+0x2b0>   @ imm = #0x24
    c.b(C::Ne,0xc32b0,false)?;
    // VMA 0xc3288: orr r1, r4, #3424
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,3424)?)?;
    // VMA 0xc328c: mov r3, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,36)?)?;
    // VMA 0xc3290: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xc3294: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xc3298: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xc329c: str r3, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc32a0: mov r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // VMA 0xc32a4: ldr r2, [r0, #0xa90]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2704)?)?;
    // VMA 0xc32a8: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xc32ac: bl 0x31250 <entry+0xfffffffffff6e250> @ imm = #-0x92064
    c.b(C::Al,0x31250,true)?;
    // VMA 0xc32b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc32b4: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc32b8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;

    c.check(0xc32bc)?;
    const FRAME_RESOURCE_LITERALS:[u32;4]=[
        0xe000_1810, // MMIO frame register selector
        0x7fff_8000, // validated signed controller range bound
        0x000f_b226, // firmware diagnostic string pointer
        0x000e_2788, // shared debug line ending
    ];
    for value in FRAME_RESOURCE_LITERALS {c.emit(value)?;}
    c.check(0xc32cc)
}
