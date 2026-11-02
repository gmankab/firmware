
/// Begin the next controller descriptor builder: initialize the codec
/// event context, publish command-buffer pointers and write per-plane
/// state fields. The function continues beyond the current checkpoint;
/// its later literal pools are not silently synthesized.
fn begin_codec_controller_descriptor_builder(c:&mut Code)->Result<(),String>{
    c.check(0xc4b28)?;
    // VMA 0xc4b28: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc4b2c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc4b30: sub sp, sp, #164
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,164)?)?;
    // VMA 0xc4b34: ldr r2, [pc, #0xfb0]        @ 0x1aec <entry+0x1aec>
    c.literal(R::R2,0xc5aec)?;
    // VMA 0xc4b38: mov r1, #110
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,110)?)?;
    // VMA 0xc4b3c: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // VMA 0xc4b40: orr r1, r1, #2304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    // VMA 0xc4b44: bl 0x17b0 <entry+0xfffffffffff3d7b0> @ imm = #-0xc339c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc4b48: ldr r10, [r9, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(9)?,676)?)?;
    // VMA 0xc4b4c: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xc4b50: ldr r1, [r10, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,2064)?)?;
    // VMA 0xc4b54: bl 0xc1b78 <entry+0xffffffffffffdb78> @ imm = #-0x2fe4
    c.b(C::Al,0xc1b78,true)?;
    // VMA 0xc4b58: ldr r0, [r10, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2064)?)?;
    // VMA 0xc4b5c: ldr r2, [r9, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,432)?)?;
    // VMA 0xc4b60: ldr r3, [r9, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,444)?)?;
    // VMA 0xc4b64: ldr r1, [pc, #0xf84]        @ 0x1af0 <entry+0x1af0>
    c.literal(R::R1,0xc5af0)?;
    // VMA 0xc4b68: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xc4b6c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4b70: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95ee4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4b74: ldr r1, [pc, #0xf78]        @ 0x1af4 <entry+0x1af4>
    c.literal(R::R1,0xc5af4)?;
    // VMA 0xc4b78: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4b7c: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95ef0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4b80: ldr r4, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(9)?,0)?)?;
    // VMA 0xc4b84: mov r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2048)?)?;
    // VMA 0xc4b88: mov r2, #335544320
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,335544320)?)?;
    // VMA 0xc4b8c: mov r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,120)?)?;
    // VMA 0xc4b90: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xc4b94: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // VMA 0xc4b98: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xc4b9c: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc4ba0: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4ba4: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // VMA 0xc4ba8: orr r2, r8, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(8)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xc4bac: lsl r0, r0, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,15)?)?;
    // VMA 0xc4bb0: ldr r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4bb4: ldr r7, [pc, #0xf3c]        @ 0x1af8 <entry+0x1af8>
    c.literal(R::new(7)?,0xc5af8)?;
    // VMA 0xc4bb8: tst r3, r7
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc4bbc: beq 0xc4bb0 <entry+0xbb0>   @ imm = #-0x14
    c.b(C::Eq,0xc4bb0,false)?;
    // VMA 0xc4bc0: orr r1, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,4)?)?;
    // VMA 0xc4bc4: orr r3, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R1,2048)?)?;
    // VMA 0xc4bc8: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc4bcc: ldr r3, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4bd0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc4bd4: blt 0xc4d00 <entry+0xd00>   @ imm = #0x124
    c.b(C::Lt,0xc4d00,false)?;
    // VMA 0xc4bd8: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xc4bdc: mov r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2048)?)?;
    // VMA 0xc4be0: strb r2, [r9, #0x220]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(9)?,544)?)?;
    // VMA 0xc4be4: orr r2, r0, #2128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,2128)?)?;
    // VMA 0xc4be8: ldr r5, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4bec: orr r2, r0, #84
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,84)?)?;
    // VMA 0xc4bf0: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xc4bf4: ldr r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4bf8: orr r2, r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,88)?)?;
    // VMA 0xc4bfc: orr r0, r0, #92
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,92)?)?;
    // VMA 0xc4c00: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xc4c04: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xc4c08: ldr r2, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4c0c: str r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,40)?)?;
    // VMA 0xc4c10: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc4c14: ldr r6, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4c18: and r1, r2, r5, lsr #19
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::new(5)?,Shift::Lsr,19,false)?)?;
    // VMA 0xc4c1c: and r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(5)?,1)?)?;
    // VMA 0xc4c20: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc4c24: strb r0, [r10, #0x508]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1288)?)?;
    // VMA 0xc4c28: str r1, [r10, #0x50c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1292)?)?;
    // VMA 0xc4c2c: and r1, r2, r5, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::new(5)?,Shift::Lsr,1,false)?)?;
    // VMA 0xc4c30: movne r2, #4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xc4c34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc4c38: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc4c3c: str r2, [r4, #0x454]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1108)?)?;
    // VMA 0xc4c40: str r3, [r4, #0x458]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,1112)?)?;
    // VMA 0xc4c44: strb r1, [r10, #0x520]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,1312)?)?;
    // VMA 0xc4c48: beq 0xc4c8c <entry+0xc8c>   @ imm = #0x3c
    c.b(C::Eq,0xc4c8c,false)?;
    // VMA 0xc4c4c: ldr r1, [pc, #0xf78]        @ 0x1bcc <entry+0x1bcc>
    c.literal(R::R1,0xc5bcc)?;
    // VMA 0xc4c50: ldr r2, [pc, #0xf78]        @ 0x1bd0 <entry+0x1bd0>
    c.literal(R::R2,0xc5bd0)?;
    // VMA 0xc4c54: mov r3, #91
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,91)?)?;
    // VMA 0xc4c58: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc4c5c: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // VMA 0xc4c60: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95fd4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4c64: ldr r1, [pc, #0xe88]        @ 0x1af4 <entry+0x1af4>
    c.literal(R::R1,0xc5af4)?;
    // VMA 0xc4c68: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc4c6c: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95fe0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4c70: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // VMA 0xc4c74: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xc4c78: str r0, [r10, #0x81c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,2076)?)?;
    // VMA 0xc4c7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc4c80: str r0, [r10, #0x7bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1980)?)?;
    // VMA 0xc4c84: ldrb r1, [r10, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1312)?)?;
    // VMA 0xc4c88: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // VMA 0xc4c8c: ldr r2, [r0, #0xa90]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2704)?)?;
    // VMA 0xc4c90: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc4c94: str r2, [r0, #0xa94]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,2708)?)?;
    // VMA 0xc4c98: str r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,44)?)?;
    // VMA 0xc4c9c: beq 0xc4d24 <entry+0xd24>   @ imm = #0x80
    c.b(C::Eq,0xc4d24,false)?;
    // VMA 0xc4ca0: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xc4ca4: ldr r1, [pc, #0xf28]        @ 0x1bd4 <entry+0x1bd4>
    c.literal(R::R1,0xc5bd4)?;
    // VMA 0xc4ca8: and r3, r0, r5, lsr #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R0,R::new(5)?,Shift::Lsr,10,false)?)?;
    // VMA 0xc4cac: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc4cb0: strb r3, [r10, #0x521]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(10)?,1313)?)?;
    // VMA 0xc4cb4: ldr r2, [r10, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,392)?)?;
    // VMA 0xc4cb8: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x9602c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4cbc: ldr r1, [pc, #0xe30]        @ 0x1af4 <entry+0x1af4>
    c.literal(R::R1,0xc5af4)?;
    // VMA 0xc4cc0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc4cc4: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x96038
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4cc8: ldrb r0, [r10, #0x521]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1313)?)?;
    // VMA 0xc4ccc: add r1, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc4cd0: ldr r0, [r1, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,640)?)?;
    // VMA 0xc4cd4: str r0, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,2028)?)?;
    // VMA 0xc4cd8: ldr r2, [r1, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,640)?)?;
    // VMA 0xc4cdc: str r2, [r10, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,2032)?)?;
    // VMA 0xc4ce0: ldr r2, [r1, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,640)?)?;
    // VMA 0xc4ce4: str r2, [r10, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,2036)?)?;
    // VMA 0xc4ce8: ldr r1, [r1, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,640)?)?;
    // VMA 0xc4cec: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc4cf0: beq 0xc4dc8 <entry+0xdc8>   @ imm = #0xd0
    c.b(C::Eq,0xc4dc8,false)?;
    // VMA 0xc4cf4: add r1, r0, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,108)?)?;
    // VMA 0xc4cf8: add r2, r0, #112
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,112)?)?;
    // VMA 0xc4cfc: b 0xc4de0 <entry+0xde0>   @ imm = #0xdc
    c.b(C::Al,0xc4de0,false)?;
    // VMA 0xc4d00: ldr r2, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4d04: tst r2, #268435457
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,268435457)?)?;
    // VMA 0xc4d08: beq 0xc4bd8 <entry+0xbd8>   @ imm = #-0x138
    c.b(C::Eq,0xc4bd8,false)?;
    // VMA 0xc4d0c: ldr r1, [pc, #0xfbc]        @ 0x1cd0 <entry+0x1cd0>
    c.literal(R::R1,0xc5cd0)?;
    // VMA 0xc4d10: ldr r2, [pc, #0xeb8]        @ 0x1bd0 <entry+0x1bd0>
    c.literal(R::R2,0xc5bd0)?;
    // VMA 0xc4d14: mov r3, #49
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,49)?)?;
    // VMA 0xc4d18: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // VMA 0xc4d1c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc4d20: b 0xc5ed4 <entry+0x1ed4>  @ imm = #0x11ac
    c.b(C::Al,0xc5ed4,false)?;
    // VMA 0xc4d24: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc4d28: str r6, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,36)?)?;
    // VMA 0xc4d2c: and r6, r1, r5, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R1,R::new(5)?,Shift::Lsr,2,false)?)?;
    // VMA 0xc4d30: and r4, r1, r5, lsr #3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R1,R::new(5)?,Shift::Lsr,3,false)?)?;
    // VMA 0xc4d34: and r3, r1, r5, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R1,R::new(5)?,Shift::Lsr,4,false)?)?;
    // VMA 0xc4d38: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc4d3c: str r6, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,1296)?)?;
    // VMA 0xc4d40: str r4, [r10, #0x588]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(10)?,1416)?)?;
    // VMA 0xc4d44: strb r3, [r10, #0x522]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(10)?,1314)?)?;
    // VMA 0xc4d48: bne 0xc5020 <entry+0x1020>  @ imm = #0x2d0
    c.b(C::Ne,0xc5020,false)?;
    // VMA 0xc4d4c: str r1, [r9, #0x370]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,880)?)?;
    // VMA 0xc4d50: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc4d54: mov r8, r5
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(5)?))?;
    // VMA 0xc4d58: ldr r1, [r10, #0x50c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1292)?)?;
    // VMA 0xc4d5c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xc4d60: moveq r3, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // VMA 0xc4d64: str r3, [r0, #0xa90]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,2704)?)?;
    // VMA 0xc4d68: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc4d6c: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xc4d70: beq 0xc4d94 <entry+0xd94>   @ imm = #0x1c
    c.b(C::Eq,0xc4d94,false)?;
    // VMA 0xc4d74: ldr r1, [pc, #0xf58]        @ 0x1cd4 <entry+0x1cd4>
    c.literal(R::R1,0xc5cd4)?;
    // VMA 0xc4d78: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc4d7c: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x960f0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4d80: ldr r1, [pc, #0xd6c]        @ 0x1af4 <entry+0x1af4>
    c.literal(R::R1,0xc5af4)?;
    // VMA 0xc4d84: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc4d88: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x960fc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4d8c: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc4d90: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // VMA 0xc4d94: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xc4d98: mov r6, r8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // VMA 0xc4d9c: and r0, r0, r6, lsr #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsr,5,false)?)?;
    // VMA 0xc4da0: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // VMA 0xc4da4: strb r0, [r9, #0x224]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,548)?)?;
    // VMA 0xc4da8: bne 0xc515c <entry+0x115c>  @ imm = #0x3ac
    c.b(C::Ne,0xc515c,false)?;
    // VMA 0xc4dac: ldrb r0, [r9, #0x24c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,588)?)?;
    // VMA 0xc4db0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc4db4: bne 0xc5264 <entry+0x1264>  @ imm = #0x4a8
    c.b(C::Ne,0xc5264,false)?;
    // VMA 0xc4db8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc4dbc: strb r0, [r9, #0x226]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,550)?)?;
    // VMA 0xc4dc0: strb r0, [r9, #0x227]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,551)?)?;
    // VMA 0xc4dc4: b 0xc5210 <entry+0x1210>  @ imm = #0x444
    c.b(C::Al,0xc5210,false)?;
    // VMA 0xc4dc8: str r10, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2028)?)?;
    // VMA 0xc4dcc: str r10, [r10, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2032)?)?;
    // VMA 0xc4dd0: str r10, [r10, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2036)?)?;
    // VMA 0xc4dd4: add r1, r4, #472
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,472)?)?;
    // VMA 0xc4dd8: add r2, r4, #468
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,468)?)?;
    // VMA 0xc4ddc: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xc4de0: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xc4de4: ldr r5, [pc, #0xd00]        @ 0x1aec <entry+0x1aec>
    c.literal(R::new(5)?,0xc5aec)?;
    // VMA 0xc4de8: mov r6, #151
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,151)?)?;
    // VMA 0xc4dec: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xc4df0: add r8, r10, #2032
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(10)?,2032)?)?;
    // VMA 0xc4df4: orr r6, r6, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,768)?)?;
    // VMA 0xc4df8: str r2, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,112)?)?;
    // VMA 0xc4dfc: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xc4e00: str r1, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,108)?)?;
    // VMA 0xc4e04: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc4e08: str r1, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1296)?)?;
    // VMA 0xc4e0c: str r1, [r10, #0x588]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1416)?)?;
    // VMA 0xc4e10: add r1, r10, #972
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,972)?)?;
    // VMA 0xc4e14: strb r4, [r10, #0x54d]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(10)?,1357)?)?;
    // VMA 0xc4e18: add r7, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R1,1024)?)?;
    // VMA 0xc4e1c: b 0xc4e28 <entry+0xe28>   @ imm = #0x4
    c.b(C::Al,0xc4e28,false)?;
    // VMA 0xc4e20: ldr r0, [r8], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R0,R::new(8)?,4)?)?;
    // VMA 0xc4e24: add r7, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,8)?)?;
    // VMA 0xc4e28: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc4e2c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc4e30: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xc4e34: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4e38: bl 0x17b0 <entry+0xfffffffffff3d7b0> @ imm = #-0xc3690
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc4e3c: ldr r0, [r8, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,-4)?)?;
    // VMA 0xc4e40: ldr r1, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,112)?)?;
    // VMA 0xc4e44: ldr r3, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,108)?)?;
    // VMA 0xc4e48: ldr r2, [r0, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,112)?)?;
    // VMA 0xc4e4c: ldr r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,108)?)?;
    // VMA 0xc4e50: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xc4e54: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xc4e58: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xc4e5c: sub r0, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,4)?)?;
    // VMA 0xc4e60: bl 0xc420c <entry+0x20c>   @ imm = #-0xc5c
    c.b(C::Al,0xc420c,true)?;
    // VMA 0xc4e64: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xc4e68: cmp r4, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,3)?)?;
    // VMA 0xc4e6c: bne 0xc4e20 <entry+0xe20>   @ imm = #-0x54
    c.b(C::Ne,0xc4e20,false)?;
    // VMA 0xc4e70: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4e74: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc4e78: str r0, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1296)?)?;
    // VMA 0xc4e7c: strb r0, [r10, #0x522]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1314)?)?;
    // VMA 0xc4e80: strb r0, [r10, #0x546]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1350)?)?;
    // VMA 0xc4e84: strb r0, [r10, #0x547]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1351)?)?;
    // VMA 0xc4e88: strb r5, [r10, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(10)?,1355)?)?;
    // VMA 0xc4e8c: strb r5, [r10, #0x54c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(10)?,1356)?)?;
    // VMA 0xc4e90: ldrb r1, [r10, #0x521]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1313)?)?;
    // VMA 0xc4e94: str r1, [r10, #0x554]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1364)?)?;
    // VMA 0xc4e98: strb r5, [r10, #0x54f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(10)?,1359)?)?;
    // VMA 0xc4e9c: str r1, [r10, #0x558]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1368)?)?;
    // VMA 0xc4ea0: strb r5, [r10, #0x550]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(10)?,1360)?)?;
    // VMA 0xc4ea4: str r1, [r10, #0x55c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1372)?)?;
    // VMA 0xc4ea8: strb r5, [r10, #0x551]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(10)?,1361)?)?;
    // VMA 0xc4eac: str r5, [r10, #0x560]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,1376)?)?;
    // VMA 0xc4eb0: str r5, [r10, #0x564]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,1380)?)?;
    // VMA 0xc4eb4: str r5, [r10, #0x568]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,1384)?)?;
    // VMA 0xc4eb8: strb r5, [r10, #0x578]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(10)?,1400)?)?;
    // VMA 0xc4ebc: mov r1, #122
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,122)?)?;
    // VMA 0xc4ec0: ldr r2, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,112)?)?;
    // VMA 0xc4ec4: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // VMA 0xc4ec8: strh r2, [r10, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::new(10)?,R::R1)?)?;
    // VMA 0xc4ecc: mov r1, #380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,380)?)?;
    // VMA 0xc4ed0: ldr r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc4ed4: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xc4ed8: strh r3, [r10, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::new(3)?,R::new(10)?,R::R1)?)?;
    // VMA 0xc4edc: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xc4ee0: str r2, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,476)?)?;
    // VMA 0xc4ee4: str r3, [r4, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,480)?)?;
    // VMA 0xc4ee8: strb r5, [r9, #0x375]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(9)?,885)?)?;
    // VMA 0xc4eec: strb r5, [r10, #0x56d]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(10)?,1389)?)?;
    // VMA 0xc4ef0: strb r5, [r9, #0x374]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(9)?,884)?)?;
    // VMA 0xc4ef4: strb r5, [r10, #0x56f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(10)?,1391)?)?;
    // VMA 0xc4ef8: strb r0, [r10, #0x570]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1392)?)?;
    // VMA 0xc4efc: strb r5, [r10, #0x571]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(10)?,1393)?)?;
    // VMA 0xc4f00: ldr r0, [r9, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,560)?)?;
    // VMA 0xc4f04: str r0, [r10, #0x514]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1300)?)?;
    // VMA 0xc4f08: mov r0, #556
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,556)?)?;
    // VMA 0xc4f0c: ldrh r0, [r9, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xc4f10: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc4f14: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc4f18: bne 0xc4f34 <entry+0xf34>   @ imm = #0x14
    c.b(C::Ne,0xc4f34,false)?;
    // VMA 0xc4f1c: mov r0, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,46)?)?;
    // VMA 0xc4f20: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xc4f24: ldrh r1, [r9, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(9)?,R::R0)?)?;
    // VMA 0xc4f28: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc4f2c: cmp r3, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc4f30: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4f34: ldr r1, [pc, #0xff8]        @ 0x1f34 <entry+0x1f34>
    c.literal(R::R1,0xc5f34)?;
    // VMA 0xc4f38: strb r0, [r10, #0x54a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1354)?)?;
    // VMA 0xc4f3c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc4f40: strb r5, [r10, #0x549]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(10)?,1353)?)?;
    // VMA 0xc4f44: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x962b8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4f48: ldr r1, [pc, #0xba4]        @ 0x1af4 <entry+0x1af4>
    c.literal(R::R1,0xc5af4)?;
    // VMA 0xc4f4c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc4f50: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x962c4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4f54: str r5, [r10, #0x5d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,1492)?)?;
    // VMA 0xc4f58: str r5, [r10, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,1496)?)?;
    // VMA 0xc4f5c: mov r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,120)?)?;
    // VMA 0xc4f60: ldr r0, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,468)?)?;
    // VMA 0xc4f64: ldr r1, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,112)?)?;
    // VMA 0xc4f68: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xc4f6c: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc4f70: bhi 0xc4fc0 <entry+0xfc0>   @ imm = #0x48
    c.b(C::Hi,0xc4fc0,false)?;
    // VMA 0xc4f74: ldr r2, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,472)?)?;
    // VMA 0xc4f78: ldr r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc4f7c: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc4f80: bhi 0xc4fc0 <entry+0xfc0>   @ imm = #0x38
    c.b(C::Hi,0xc4fc0,false)?;
    // VMA 0xc4f84: ldr r7, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,192)?)?;
    // VMA 0xc4f88: ldr r6, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,196)?)?;
    // VMA 0xc4f8c: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc4f90: blo 0xc4fc0 <entry+0xfc0>   @ imm = #0x28
    c.b(C::Cc,0xc4fc0,false)?;
    // VMA 0xc4f94: ldr r7, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,972)?)?;
    // VMA 0xc4f98: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc4f9c: beq 0xc5404 <entry+0x1404>  @ imm = #0x460
    c.b(C::Eq,0xc5404,false)?;
    // VMA 0xc4fa0: ldr r7, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,248)?)?;
    // VMA 0xc4fa4: ldr r6, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,416)?)?;
    // VMA 0xc4fa8: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc4fac: blo 0xc4fc0 <entry+0xfc0>   @ imm = #0xc
    c.b(C::Cc,0xc4fc0,false)?;
    // VMA 0xc4fb0: ldr r7, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,236)?)?;
    // VMA 0xc4fb4: ldr r6, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,412)?)?;
    // VMA 0xc4fb8: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc4fbc: bhs 0xc5404 <entry+0x1404>  @ imm = #0x440
    c.b(C::Cs,0xc5404,false)?;
    // VMA 0xc4fc0: ldr r1, [pc, #0xfdc]        @ 0x1fa4 <entry+0x1fa4>
    c.literal(R::R1,0xc5fa4)?;
    // VMA 0xc4fc4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc4fc8: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x9633c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4fcc: ldr r1, [pc, #0xb20]        @ 0x1af4 <entry+0x1af4>
    c.literal(R::R1,0xc5af4)?;
    // VMA 0xc4fd0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc4fd4: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x96348
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4fd8: ldr r1, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,192)?)?;
    // VMA 0xc4fdc: ldr r2, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,196)?)?;
    // VMA 0xc4fe0: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc4fe4: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc4fe8: blo 0xc511c <entry+0x111c>  @ imm = #0x12c
    c.b(C::Cc,0xc511c,false)?;
    // VMA 0xc4fec: ldr r1, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,972)?)?;
    // VMA 0xc4ff0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc4ff4: beq 0xc5118 <entry+0x1118>  @ imm = #0x11c
    c.b(C::Eq,0xc5118,false)?;
    // VMA 0xc4ff8: ldr r1, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,248)?)?;
    // VMA 0xc4ffc: ldr r2, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,416)?)?;

    c.check(0xc5000)
}
