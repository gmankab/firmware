
/// Process codec input plane stream commands and validate optional buffer state; function continues
/// Runtime slice 0xb1b44..0xb2000, with 0 classified trailing data words.
fn process_codec_plane_command_queue(c:&mut Code)->Result<(),String>{
    c.check(0xb1b44)?;
    // ARM 0xb1b44: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb1b48: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xb1b4c: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // ARM 0xb1b50: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xb1b54: ldr r5, [r4, #0xe70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,3696)?)?;
    // ARM 0xb1b58: ldr r10, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(4)?,36)?)?;
    // ARM 0xb1b5c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // ARM 0xb1b60: mov r6, r10
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(10)?))?;
    // ARM 0xb1b64: bne 0xb1b94 <entry+0x5444>
    c.b(C::Ne,0xb1b94,false)?;
    // ARM 0xb1b68: mov r0, #1824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1824)?)?;
    // ARM 0xb1b6c: bl 0x2f28 <entry+0xfffffffffff567d8>
    c.b(C::Al,0x2f28,true)?;
    // ARM 0xb1b70: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xb1b74: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // ARM 0xb1b78: beq 0xb1c38 <entry+0x54e8>
    c.b(C::Eq,0xb1c38,false)?;
    // ARM 0xb1b7c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xb1b80: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb1b84: mov r2, #1824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1824)?)?;
    // ARM 0xb1b88: bl 0x31ec <entry+0xfffffffffff56a9c>
    c.b(C::Al,0x31ec,true)?;
    // ARM 0xb1b8c: str r5, [r4, #0xe70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,3696)?)?;
    // ARM 0xb1b90: ldr r6, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,36)?)?;
    // ARM 0xb1b94: mov r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048)?)?;
    // ARM 0xb1b98: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // ARM 0xb1b9c: lsl r2, r6, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(6)?,Shift::Lsl,15)?)?;
    // ARM 0xb1ba0: mov r8, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,18)?)?;
    // ARM 0xb1ba4: orr r9, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(9)?,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb1ba8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb1bac: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xb1bb0: str r0, [r1, r3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // ARM 0xb1bb4: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM 0xb1bb8: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb1bbc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb1bc0: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb1bc4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb1bc8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb1bcc: orr r7, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb1bd0: ldr r0, [r7, r3]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1bd4: strb r0, [r5, #0x5c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,92)?)?;
    // ARM 0xb1bd8: and r0, r0, #240
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,240)?)?;
    // ARM 0xb1bdc: cmp r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // ARM 0xb1be0: bhi 0xb1c9c <entry+0x554c>
    c.b(C::Hi,0xb1c9c,false)?;
    // ARM 0xb1be4: ldr r0, [r7, r3]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1be8: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xb1bec: and r1, r0, #248
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,248)?)?;
    // ARM 0xb1bf0: strb r0, [r5, #0x5e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,94)?)?;
    // ARM 0xb1bf4: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // ARM 0xb1bf8: bhi 0xb1c9c <entry+0x554c>
    c.b(C::Hi,0xb1c9c,false)?;
    // ARM 0xb1bfc: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // ARM 0xb1c00: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb1c04: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // ARM 0xb1c08: ldr r1, [r7, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1c0c: strb r1, [r5, #0x5d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,93)?)?;
    // ARM 0xb1c10: and r1, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,255)?)?;
    // ARM 0xb1c14: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xb1c18: bl 0xb2d48 <entry+0x65f8>
    c.b(C::Al,0xb2d48,true)?;
    // ARM 0xb1c1c: mov r0, #77
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,77)?)?;
    // ARM 0xb1c20: str r0, [r5, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,88)?)?;
    // ARM 0xb1c24: ldrb r1, [r5, #0x2]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2)?)?;
    // ARM 0xb1c28: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // ARM 0xb1c2c: bne 0xb1c44 <entry+0x54f4>
    c.b(C::Ne,0xb1c44,false)?;
    // ARM 0xb1c30: str r0, [r5, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,88)?)?;
    // ARM 0xb1c34: b 0xb1c50 <entry+0x5500>
    c.b(C::Al,0xb1c50,false)?;
    // ARM 0xb1c38: mov r0, #45
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,45)?)?;
    // ARM 0xb1c3c: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xb1c40: b 0xb1ddc <entry+0x568c>
    c.b(C::Al,0xb1ddc,false)?;
    // ARM 0xb1c44: ldrb r2, [r5, #0x7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,7)?)?;
    // ARM 0xb1c48: tst r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,64)?)?;
    // ARM 0xb1c4c: strne r0, [r5, #0x58]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(5)?,88)?)?;
    // ARM 0xb1c50: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // ARM 0xb1c54: beq 0xb1c64 <entry+0x5514>
    c.b(C::Eq,0xb1c64,false)?;
    // ARM 0xb1c58: ldrb r0, [r5, #0x7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,7)?)?;
    // ARM 0xb1c5c: tst r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,32)?)?;
    // ARM 0xb1c60: beq 0xb1c6c <entry+0x551c>
    c.b(C::Eq,0xb1c6c,false)?;
    // ARM 0xb1c64: mov r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,88)?)?;
    // ARM 0xb1c68: str r0, [r5, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,88)?)?;
    // ARM 0xb1c6c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb1c70: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // ARM 0xb1c74: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // ARM 0xb1c78: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // ARM 0xb1c7c: str r1, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb1c80: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb1c84: ldr r1, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb1c88: strb r1, [r5, #0x5f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,95)?)?;
    // ARM 0xb1c8c: and r1, r1, #240
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,240)?)?;
    // ARM 0xb1c90: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // ARM 0xb1c94: cmp r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // ARM 0xb1c98: bls 0xb1de8 <entry+0x5698>
    c.b(C::Ls,0xb1de8,false)?;
    // ARM 0xb1c9c: orr r0, r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,116)?)?;
    // ARM 0xb1ca0: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb1ca4: ldr r1, [r0, r3]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1ca8: tst r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,4)?)?;
    // ARM 0xb1cac: bne 0xb1cc4 <entry+0x5574>
    c.b(C::Ne,0xb1cc4,false)?;
    // ARM 0xb1cb0: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb1cb4: ldr r2, [r7, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1cb8: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1cbc: tst r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,4)?)?;
    // ARM 0xb1cc0: beq 0xb1cb4 <entry+0x5564>
    c.b(C::Eq,0xb1cb4,false)?;
    // ARM 0xb1cc4: str r8, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,2432)?)?;
    // ARM 0xb1cc8: ldr r0, [pc, #0xda0]
    c.literal(R::R0,0xb2a70)?;
    // ARM 0xb1ccc: mov r6, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // ARM 0xb1cd0: lsl r7, r10, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(10)?,Shift::Lsl,15)?)?;
    // ARM 0xb1cd4: str r0, [r5, #0x26c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,620)?)?;
    // ARM 0xb1cd8: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // ARM 0xb1cdc: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb1ce0: orr r0, r0, r10, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(10)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb1ce4: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1ce8: tst r0, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // ARM 0xb1cec: beq 0xb1d30 <entry+0x55e0>
    c.b(C::Eq,0xb1d30,false)?;
    // ARM 0xb1cf0: orr r1, r7, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(7)?,124)?)?;
    // ARM 0xb1cf4: mov r2, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192)?)?;
    // ARM 0xb1cf8: mov r3, #33
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,33)?)?;
    // ARM 0xb1cfc: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xb1d00: orr r3, r3, #2816
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // ARM 0xb1d04: str r2, [r1, r6]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1d08: stm sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xb1d0c: ldr r1, [pc, #0xd4c]
    c.literal(R::R1,0xb2a60)?;
    // ARM 0xb1d10: ldr r2, [pc, #0xd5c]
    c.literal(R::R2,0xb2a74)?;
    // ARM 0xb1d14: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb1d18: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1d1c: ldr r1, [pc, #0xd28]
    c.literal(R::R1,0xb2a4c)?;
    // ARM 0xb1d20: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb1d24: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1d28: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xb1d2c: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xb1d30: orr r0, r7, #116
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,116)?)?;
    // ARM 0xb1d34: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb1d38: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1d3c: and r1, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,5)?)?;
    // ARM 0xb1d40: cmp r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // ARM 0xb1d44: beq 0xb1d78 <entry+0x5628>
    c.b(C::Eq,0xb1d78,false)?;
    // ARM 0xb1d48: ldr r1, [pc, #0xd28]
    c.literal(R::R1,0xb2a78)?;
    // ARM 0xb1d4c: ldr r2, [pc, #0xd20]
    c.literal(R::R2,0xb2a74)?;
    // ARM 0xb1d50: mov r3, #34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,34)?)?;
    // ARM 0xb1d54: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xb1d58: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb1d5c: orr r3, r3, #2816
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // ARM 0xb1d60: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1d64: ldr r1, [pc, #0xce0]
    c.literal(R::R1,0xb2a4c)?;
    // ARM 0xb1d68: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb1d6c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1d70: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xb1d74: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xb1d78: ldr r0, [r4, #0xfac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4012)?)?;
    // ARM 0xb1d7c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb1d80: beq 0xb1dbc <entry+0x566c>
    c.b(C::Eq,0xb1dbc,false)?;
    // ARM 0xb1d84: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM 0xb1d88: str r6, [r4, #0xfac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,4012)?)?;
    // ARM 0xb1d8c: ldrb r2, [r5, #0x5f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,95)?)?;
    // ARM 0xb1d90: add r0, r4, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R2,Shift::Lsl,2,false)?)?;
    // ARM 0xb1d94: ldr r0, [r0, #0xe30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,3632)?)?;
    // ARM 0xb1d98: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb1d9c: bne 0xb1dbc <entry+0x566c>
    c.b(C::Ne,0xb1dbc,false)?;
    // ARM 0xb1da0: ldr r1, [pc, #0xcd4]
    c.literal(R::R1,0xb2a7c)?;
    // ARM 0xb1da4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb1da8: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1dac: ldr r1, [pc, #0xc98]
    c.literal(R::R1,0xb2a4c)?;
    // ARM 0xb1db0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb1db4: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1db8: str r6, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,2432)?)?;
    // ARM 0xb1dbc: ldr r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xb1dc0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb1dc4: bne 0xb1ddc <entry+0x568c>
    c.b(C::Ne,0xb1ddc,false)?;
    // ARM 0xb1dc8: ldrb r0, [r5, #0x5f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,95)?)?;
    // ARM 0xb1dcc: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // ARM 0xb1dd0: ldr r1, [r0, #0xe30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,3632)?)?;
    // ARM 0xb1dd4: str r1, [r4, #0xe70]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,3696)?)?;
    // ARM 0xb1dd8: str r5, [r0, #0xe30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,3632)?)?;
    // ARM 0xb1ddc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb1de0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xb1de4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xb1de8: ldr r0, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb1dec: strb r0, [r5, #0x60]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,96)?)?;
    // ARM 0xb1df0: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // ARM 0xb1df4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xb1df8: bne 0xb1ef0 <entry+0x57a0>
    c.b(C::Ne,0xb1ef0,false)?;
    // ARM 0xb1dfc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb1e00: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // ARM 0xb1e04: strb r1, [r5, #0x61]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,97)?)?;
    // ARM 0xb1e08: bne 0xb1e20 <entry+0x56d0>
    c.b(C::Ne,0xb1e20,false)?;
    // ARM 0xb1e0c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb1e10: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb1e14: str r1, [r9, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb1e18: ldr r0, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb1e1c: strb r0, [r5, #0x61]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,97)?)?;
    // ARM 0xb1e20: mov r8, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,3758096384)?)?;
    // ARM 0xb1e24: mov r0, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16777216)?)?;
    // ARM 0xb1e28: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xb1e2c: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // ARM 0xb1e30: str r0, [r1, r8]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(8)?,Shift::Lsl,0)?)?;
    // ARM 0xb1e34: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // ARM 0xb1e38: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb1e3c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb1e40: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb1e44: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM 0xb1e48: ldr r1, [r0, r8]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::new(8)?,Shift::Lsl,0)?)?;
    // ARM 0xb1e4c: strh r1, [r5, #98]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(5)?,98)?)?;
    // ARM 0xb1e50: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // ARM 0xb1e54: and r0, r1, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1e58: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // ARM 0xb1e5c: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,4)?)?;
    // ARM 0xb1e60: and r1, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1e64: strh r2, [r5, #100]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::new(5)?,100)?)?;
    // ARM 0xb1e68: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // ARM 0xb1e6c: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // ARM 0xb1e70: bl 0x2d394 <entry+0xfffffffffff80c44>
    c.b(C::Al,0x2d394,true)?;
    // ARM 0xb1e74: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb1e78: beq 0xb1f38 <entry+0x57e8>
    c.b(C::Eq,0xb1f38,false)?;
    // ARM 0xb1e7c: ldr r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,644)?)?;
    // ARM 0xb1e80: ldr r2, [pc, #0xbc0]
    c.literal(R::R2,0xb2a48)?;
    // ARM 0xb1e84: mov r3, #376
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,376)?)?;
    // ARM 0xb1e88: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // ARM 0xb1e8c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xb1e90: str r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,644)?)?;
    // ARM 0xb1e94: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb1e98: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xb1e9c: ldrh r0, [r5, #98]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(5)?,98)?)?;
    // ARM 0xb1ea0: ldrh r1, [r5, #100]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,100)?)?;
    // ARM 0xb1ea4: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xb1ea8: ldr r1, [pc, #0xb94]
    c.literal(R::R1,0xb2a44)?;
    // ARM 0xb1eac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb1eb0: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1eb4: ldr r1, [pc, #0xb90]
    c.literal(R::R1,0xb2a4c)?;
    // ARM 0xb1eb8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb1ebc: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1ec0: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // ARM 0xb1ec4: orr r0, r0, #116
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,116)?)?;
    // ARM 0xb1ec8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb1ecc: ldr r1, [r0, r8]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1ed0: tst r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,4)?)?;
    // ARM 0xb1ed4: bne 0xb1cc8 <entry+0x5578>
    c.b(C::Ne,0xb1cc8,false)?;
    // ARM 0xb1ed8: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb1edc: ldr r2, [r7, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1ee0: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1ee4: tst r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,4)?)?;
    // ARM 0xb1ee8: beq 0xb1edc <entry+0x578c>
    c.b(C::Eq,0xb1edc,false)?;
    // ARM 0xb1eec: b 0xb1cc8 <entry+0x5578>
    c.b(C::Al,0xb1cc8,false)?;
    // ARM 0xb1ef0: ldr r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,644)?)?;
    // ARM 0xb1ef4: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb1ef8: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xb1efc: str r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,644)?)?;
    // ARM 0xb1f00: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb1f04: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xb1f08: mov r0, #176
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,176)?)?;
    // ARM 0xb1f0c: strh r0, [r5, #98]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,98)?)?;
    // ARM 0xb1f10: mov r0, #144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,144)?)?;
    // ARM 0xb1f14: strh r0, [r5, #100]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,100)?)?;
    // ARM 0xb1f18: orr r0, r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,116)?)?;
    // ARM 0xb1f1c: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb1f20: b 0xb1f28 <entry+0x57d8>
    c.b(C::Al,0xb1f28,false)?;
    // ARM 0xb1f24: ldr r2, [r7, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1f28: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1f2c: tst r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,4)?)?;
    // ARM 0xb1f30: beq 0xb1f24 <entry+0x57d4>
    c.b(C::Eq,0xb1f24,false)?;
    // ARM 0xb1f34: b 0xb1cc8 <entry+0x5578>
    c.b(C::Al,0xb1cc8,false)?;
    // ARM 0xb1f38: ldr r0, [r7, r8]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1f3c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb1f40: beq 0xb1fdc <entry+0x588c>
    c.b(C::Eq,0xb1fdc,false)?;
    // ARM 0xb1f44: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb1f48: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // ARM 0xb1f4c: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // ARM 0xb1f50: mov r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,255)?)?;
    // ARM 0xb1f54: mov r12, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16711680)?)?;
    // ARM 0xb1f58: str r1, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb1f5c: orr r8, r8, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,65280)?)?;
    // ARM 0xb1f60: orr r12, r12, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4278190080)?)?;
    // ARM 0xb1f64: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb1f68: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb1f6c: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb1f70: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xb1f74: ldr r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb1f78: strh r2, [r5, #102]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::new(5)?,102)?)?;
    // ARM 0xb1f7c: and lr, r2, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::LR,R::R2,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1f80: ldr r3, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,0)?)?;
    // ARM 0xb1f84: strh r3, [r5, #104]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::new(5)?,104)?)?;
    // ARM 0xb1f88: orr r3, r3, r12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R12,Shift::Lsl,0,false)?)?;
    // ARM 0xb1f8c: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb1f90: eor r3, r3, r8
    c.emit(arm32::data_register(C::Al,Op::Eor,R::new(3)?,R::new(3)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1f94: strh r0, [r5, #106]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,106)?)?;
    // ARM 0xb1f98: and r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1f9c: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // ARM 0xb1fa0: strh r1, [r5, #108]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(5)?,108)?)?;
    // ARM 0xb1fa4: orr r1, r1, r12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // ARM 0xb1fa8: ldrh r2, [r5, #98]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(5)?,98)?)?;
    // ARM 0xb1fac: eor r1, r1, r8
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R1,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1fb0: add r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb1fb4: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // ARM 0xb1fb8: cmp r2, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xb1fbc: movlt r2, #0
    c.emit(arm32::data_immediate(C::Lt,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xb1fc0: strhlt r2, [r5, #102]
    c.emit(arm32::store_halfword(C::Lt,R::R2,R::new(5)?,102)?)?;
    // ARM 0xb1fc4: strhlt r2, [r5, #104]
    c.emit(arm32::store_halfword(C::Lt,R::R2,R::new(5)?,104)?)?;
    // ARM 0xb1fc8: ldrh r2, [r5, #100]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(5)?,100)?)?;
    // ARM 0xb1fcc: add r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb1fd0: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb1fd4: blt 0xb1fec <entry+0x589c>
    c.b(C::Lt,0xb1fec,false)?;
    // ARM 0xb1fd8: b 0xb1ff8 <entry+0x58a8>
    c.b(C::Al,0xb1ff8,false)?;
    // ARM 0xb1fdc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb1fe0: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // ARM 0xb1fe4: strh r0, [r5, #102]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,102)?)?;
    // ARM 0xb1fe8: strh r0, [r5, #104]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,104)?)?;
    // ARM 0xb1fec: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb1ff0: strh r0, [r5, #106]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,106)?)?;
    // ARM 0xb1ff4: strh r0, [r5, #108]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,108)?)?;
    // ARM 0xb1ff8: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb1ffc: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    c.check(0xb2000)
}
