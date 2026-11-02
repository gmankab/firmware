
/// Normalize controller input sizes and copy/check the codec's per-plane
/// resource list, dispatching fault diagnostics when bytes/slots disagree.
/// Three literal pools are distinguished from executable A32 instructions.
fn validate_codec_plane_lengths_and_descriptor_arrays(c:&mut Code)->Result<(),String>{
    c.check(0xb0c64)?;
    // ARM 0xb0c64: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb0c68: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM 0xb0c6c: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // ARM 0xb0c70: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // ARM 0xb0c74: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // ARM 0xb0c78: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // ARM 0xb0c7c: ldrb r2, [r6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,0)?)?;
    // ARM 0xb0c80: cmp r2, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0c84: bls 0xb0cc0 <entry+0x4570>
    c.b(C::Ls,0xb0cc0,false)?;
    // ARM 0xb0c88: ldrb r0, [r7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // ARM 0xb0c8c: ldr r1, [pc, #0x6c]
    c.literal(R::R1,0xb0d00)?;
    // ARM 0xb0c90: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // ARM 0xb0c94: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xb0c98: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb0c9c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0ca0: ldr r1, [pc, #0x5c]
    c.literal(R::R1,0xb0d04)?;
    // ARM 0xb0ca4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb0ca8: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0cac: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM 0xb0cb0: strb r5, [r6]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(6)?,0)?)?;
    // ARM 0xb0cb4: strb r4, [r7]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(7)?,0)?)?;
    // ARM 0xb0cb8: ldrb r2, [r6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,0)?)?;
    // ARM 0xb0cbc: b 0xb0cc4 <entry+0x4574>
    c.b(C::Al,0xb0cc4,false)?;
    // ARM 0xb0cc0: ldrb r4, [r7]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(7)?,0)?)?;
    // ARM 0xb0cc4: sub r3, r5, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0cc8: cmp r3, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0ccc: bge 0xb0cf8 <entry+0x45a8>
    c.b(C::Ge,0xb0cf8,false)?;
    // ARM 0xb0cd0: ldr r1, [pc, #0x30]
    c.literal(R::R1,0xb0d08)?;
    // ARM 0xb0cd4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb0cd8: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // ARM 0xb0cdc: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0ce0: ldr r1, [pc, #0x1c]
    c.literal(R::R1,0xb0d04)?;
    // ARM 0xb0ce4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb0ce8: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0cec: ldrb r0, [r6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xb0cf0: sub r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0cf4: strb r0, [r7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // ARM 0xb0cf8: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // ARM 0xb0cfc: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb0d00)?;
    // ARM 0xb0d00: controller diagnostic/format data, not executable
    c.emit(0x000f8a8e)?;
    // ARM 0xb0d04: controller diagnostic/format data, not executable
    c.emit(0x000e2788)?;
    // ARM 0xb0d08: controller diagnostic/format data, not executable
    c.emit(0x000f8ae6)?;
    c.check(0xb0d0c)?;
    // ARM 0xb0d0c: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb0d10: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // ARM 0xb0d14: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // ARM 0xb0d18: ldr r12, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,36)?)?;
    // ARM 0xb0d1c: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM 0xb0d20: blt 0xb0db8 <entry+0x4668>
    c.b(C::Lt,0xb0db8,false)?;
    // ARM 0xb0d24: mov r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048)?)?;
    // ARM 0xb0d28: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // ARM 0xb0d2c: orr r3, r0, r12, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::R12,Shift::Lsl,15,false)?)?;
    // ARM 0xb0d30: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb0d34: str r6, [r3, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(6)?,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0d38: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // ARM 0xb0d3c: orr r6, r6, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,2048)?)?;
    // ARM 0xb0d40: orr r5, r6, r12, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(6)?,R::R12,Shift::Lsl,15,false)?)?;
    // ARM 0xb0d44: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0d48: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb0d4c: beq 0xb0db8 <entry+0x4668>
    c.b(C::Eq,0xb0db8,false)?;
    // ARM 0xb0d50: ldrb r0, [r1, #0xb4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R1,180)?)?;
    // ARM 0xb0d54: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // ARM 0xb0d58: mov r4, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,16777216)?)?;
    // ARM 0xb0d5c: cmp r0, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0d60: moveq r0, #-536870912
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb0d64: moveq r2, #16777216
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,16777216)?)?;
    // ARM 0xb0d68: streq r2, [r3, r0]
    c.emit(arm32::transfer_word_register_offset(C::Eq,false,R::R2,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0d6c: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // ARM 0xb0d70: ldreq r0, [r5, r0]
    c.emit(arm32::transfer_word_register_offset(C::Eq,true,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0d74: mov r0, r3
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
    // ARM 0xb0d78: str r6, [r0, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::R0,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb0d7c: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // ARM 0xb0d80: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xb0d84: ldr r4, [r0, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(4)?,R::R0,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb0d88: mvn r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(4)?,R::R0,0)?)?;
    // ARM 0xb0d8c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // ARM 0xb0d90: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0d94: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xb0d98: ldr r0, [r5, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0d9c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb0da0: streq r6, [r3, r2]
    c.emit(arm32::transfer_word_register_offset(C::Eq,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0da4: ldreq r0, [r5, r2]
    c.emit(arm32::transfer_word_register_offset(C::Eq,true,R::R0,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb0da8: ldr r0, [r1, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,640)?)?;
    // ARM 0xb0dac: cmp r4, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0db0: blo 0xb0d90 <entry+0x4640>
    c.b(C::Cc,0xb0d90,false)?;
    // ARM 0xb0db4: b 0xb0e68 <entry+0x4718>
    c.b(C::Al,0xb0e68,false)?;
    // ARM 0xb0db8: mov r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048)?)?;
    // ARM 0xb0dbc: mov r5, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // ARM 0xb0dc0: mov r8, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,16777216)?)?;
    // ARM 0xb0dc4: orr r6, r0, r12, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R0,R::R12,Shift::Lsl,15,false)?)?;
    // ARM 0xb0dc8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xb0dcc: str r8, [r0, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(8)?,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb0dd0: str r8, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R0,0)?)?;
    // ARM 0xb0dd4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb0dd8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb0ddc: orr r2, r0, r12, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R12,Shift::Lsl,15,false)?)?;
    // ARM 0xb0de0: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // ARM 0xb0de4: ldr r4, [r0, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(4)?,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb0de8: ldr lr, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,0)?)?;
    // ARM 0xb0dec: cmp r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,1)?)?;
    // ARM 0xb0df0: blt 0xb0e28 <entry+0x46d8>
    c.b(C::Lt,0xb0e28,false)?;
    // ARM 0xb0df4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb0df8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb0dfc: mov r7, r6
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // ARM 0xb0e00: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xb0e04: str r8, [r7, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(8)?,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb0e08: cmp r0, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0e0c: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // ARM 0xb0e10: mov r7, r2
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // ARM 0xb0e14: ldr r3, [r7, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb0e18: ldr r3, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,0)?)?;
    // ARM 0xb0e1c: bne 0xb0dfc <entry+0x46ac>
    c.b(C::Ne,0xb0dfc,false)?;
    // ARM 0xb0e20: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // ARM 0xb0e24: b 0xb0e2c <entry+0x46dc>
    c.b(C::Al,0xb0e2c,false)?;
    // ARM 0xb0e28: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xb0e2c: add r7, lr, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::LR,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0e30: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0e34: bge 0xb0e68 <entry+0x4718>
    c.b(C::Ge,0xb0e68,false)?;
    // ARM 0xb0e38: mov r5, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // ARM 0xb0e3c: mov lr, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,16777216)?)?;
    // ARM 0xb0e40: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb0e44: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM 0xb0e48: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // ARM 0xb0e4c: str lr, [r1, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::LR,R::R1,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb0e50: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0e54: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb0e58: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // ARM 0xb0e5c: ldr r4, [r1, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(4)?,R::R1,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb0e60: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // ARM 0xb0e64: bne 0xb0e44 <entry+0x46f4>
    c.b(C::Ne,0xb0e44,false)?;
    // ARM 0xb0e68: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // ARM 0xb0e6c: mov r6, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // ARM 0xb0e70: lsl r4, r12, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R12,Shift::Lsl,15)?)?;
    // ARM 0xb0e74: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb0e78: orr r0, r0, r12, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R12,Shift::Lsl,15,false)?)?;
    // ARM 0xb0e7c: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0e80: tst r0, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // ARM 0xb0e84: beq 0xb0ec8 <entry+0x4778>
    c.b(C::Eq,0xb0ec8,false)?;
    // ARM 0xb0e88: orr r1, r4, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,124)?)?;
    // ARM 0xb0e8c: mov r2, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192)?)?;
    // ARM 0xb0e90: mov r3, #186
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,186)?)?;
    // ARM 0xb0e94: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xb0e98: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // ARM 0xb0e9c: str r2, [r1, r6]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0ea0: stm sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xb0ea4: ldr r1, [pc, #0x94]
    c.literal(R::R1,0xb0f40)?;
    // ARM 0xb0ea8: ldr r2, [pc, #0x94]
    c.literal(R::R2,0xb0f44)?;
    // ARM 0xb0eac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0eb0: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0eb4: ldr r1, [pc, #0x8c]
    c.literal(R::R1,0xb0f48)?;
    // ARM 0xb0eb8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0ebc: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0ec0: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xb0ec4: b 0xb0ecc <entry+0x477c>
    c.b(C::Al,0xb0ecc,false)?;
    // ARM 0xb0ec8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb0ecc: orr r1, r4, #116
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,116)?)?;
    // ARM 0xb0ed0: orr r5, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R1,2048)?)?;
    // ARM 0xb0ed4: ldr r1, [r5, r6]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0ed8: and r2, r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,5)?)?;
    // ARM 0xb0edc: cmp r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // ARM 0xb0ee0: beq 0xb0f10 <entry+0x47c0>
    c.b(C::Eq,0xb0f10,false)?;
    // ARM 0xb0ee4: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // ARM 0xb0ee8: ldr r1, [pc, #0x5c]
    c.literal(R::R1,0xb0f4c)?;
    // ARM 0xb0eec: ldr r2, [pc, #0x50]
    c.literal(R::R2,0xb0f44)?;
    // ARM 0xb0ef0: mov r3, #187
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,187)?)?;
    // ARM 0xb0ef4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0ef8: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // ARM 0xb0efc: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0f00: ldr r1, [pc, #0x40]
    c.literal(R::R1,0xb0f48)?;
    // ARM 0xb0f04: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0f08: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0f0c: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xb0f10: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb0f14: ldr r2, [r5, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb0f18: tst r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,4)?)?;
    // ARM 0xb0f1c: bne 0xb0f38 <entry+0x47e8>
    c.b(C::Ne,0xb0f38,false)?;
    // ARM 0xb0f20: orr r2, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(4)?,4)?)?;
    // ARM 0xb0f24: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // ARM 0xb0f28: ldr r3, [r2, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb0f2c: ldr r3, [r5, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb0f30: tst r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,4)?)?;
    // ARM 0xb0f34: beq 0xb0f28 <entry+0x47d8>
    c.b(C::Eq,0xb0f28,false)?;
    // ARM 0xb0f38: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // ARM 0xb0f3c: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb0f40)?;
    // ARM 0xb0f40: controller diagnostic/format data, not executable
    c.emit(0x000f7058)?;
    // ARM 0xb0f44: controller diagnostic/format data, not executable
    c.emit(0x000f8915)?;
    // ARM 0xb0f48: controller diagnostic/format data, not executable
    c.emit(0x000e2788)?;
    // ARM 0xb0f4c: controller diagnostic/format data, not executable
    c.emit(0x000f7095)?;
    c.check(0xb0f50)?;
    // ARM 0xb0f50: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb0f54: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xb0f58: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // ARM 0xb0f5c: ldr r4, [r0, #0x990]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,2448)?)?;
    // ARM 0xb0f60: ldrb r1, [r4, #0x13]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,19)?)?;
    // ARM 0xb0f64: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xb0f68: beq 0xb115c <entry+0x4a0c>
    c.b(C::Eq,0xb115c,false)?;
    // ARM 0xb0f6c: ldrb r1, [r4, #0x15]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,21)?)?;
    // ARM 0xb0f70: ldr lr, [r0, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,2444)?)?;
    // ARM 0xb0f74: ldrh r5, [r4, #22]
    c.emit(arm32::load_halfword(C::Al,R::new(5)?,R::new(4)?,22)?)?;
    // ARM 0xb0f78: ldr r7, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,28)?)?;
    // ARM 0xb0f7c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xb0f80: beq 0xb1004 <entry+0x48b4>
    c.b(C::Eq,0xb1004,false)?;
    // ARM 0xb0f84: cmp r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // ARM 0xb0f88: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM 0xb0f8c: str r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,16)?)?;
    // ARM 0xb0f90: str lr, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,8)?)?;
    // ARM 0xb0f94: blo 0xb1048 <entry+0x48f8>
    c.b(C::Cc,0xb1048,false)?;
    // ARM 0xb0f98: add r0, lr, #616
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::LR,616)?)?;
    // ARM 0xb0f9c: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM 0xb0fa0: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // ARM 0xb0fa4: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // ARM 0xb0fa8: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // ARM 0xb0fac: add r8, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(10)?,1)?)?;
    // ARM 0xb0fb0: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xb0fb4: ldrh r4, [r0]
    c.emit(arm32::load_halfword(C::Al,R::new(4)?,R::R0,0)?)?;
    // ARM 0xb0fb8: mul r0, r8, r4
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(8)?,R::new(4)?)?)?;
    // ARM 0xb0fbc: blx 0x82be4 <entry+0xfffffffffffd6494>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // ARM 0xb0fc0: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // ARM 0xb0fc4: mul r0, r10, r4
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(10)?,R::new(4)?)?)?;
    // ARM 0xb0fc8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xb0fcc: blx 0x82be4 <entry+0xfffffffffffd6494>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // ARM 0xb0fd0: ldr r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,16)?)?;
    // ARM 0xb0fd4: sub r0, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0fd8: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // ARM 0xb0fdc: mov r10, r8
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(8)?))?;
    // ARM 0xb0fe0: strh r0, [r7], #2
    c.emit(arm32::store_halfword_postindexed(C::Al,R::R0,R::new(7)?,2)?)?;
    // ARM 0xb0fe4: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // ARM 0xb0fe8: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb0fec: add r9, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0ff0: ldrh r5, [r4, #22]
    c.emit(arm32::load_halfword(C::Al,R::new(5)?,R::new(4)?,22)?)?;
    // ARM 0xb0ff4: sub r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(5)?,1)?)?;
    // ARM 0xb0ff8: cmp r8, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0ffc: blt 0xb0fa8 <entry+0x4858>
    c.b(C::Lt,0xb0fa8,false)?;
    // ARM 0xb1000: b 0xb104c <entry+0x48fc>
    c.b(C::Al,0xb104c,false)?;
    // ARM 0xb1004: mov r1, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,616)?)?;
    // ARM 0xb1008: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // ARM 0xb100c: cmp r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // ARM 0xb1010: ldrh r1, [lr, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::LR,R::R1)?)?;
    // ARM 0xb1014: blo 0xb1104 <entry+0x49b4>
    c.b(C::Cc,0xb1104,false)?;
    // ARM 0xb1018: sub r3, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(5)?,1)?)?;
    // ARM 0xb101c: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM 0xb1020: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xb1024: ldrh r5, [r7]
    c.emit(arm32::load_halfword(C::Al,R::new(5)?,R::new(7)?,0)?)?;
    // ARM 0xb1028: add r2, r2, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb102c: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1030: bgt 0xb1174 <entry+0x4a24>
    c.b(C::Gt,0xb1174,false)?;
    // ARM 0xb1034: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // ARM 0xb1038: add r7, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,2)?)?;
    // ARM 0xb103c: cmp r6, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1040: blt 0xb1024 <entry+0x48d4>
    c.b(C::Lt,0xb1024,false)?;
    // ARM 0xb1044: b 0xb1108 <entry+0x49b8>
    c.b(C::Al,0xb1108,false)?;
    // ARM 0xb1048: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // ARM 0xb104c: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // ARM 0xb1050: mov r0, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,616)?)?;
    // ARM 0xb1054: ldrh r0, [r1, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::R1,R::R0)?)?;
    // ARM 0xb1058: sub r0, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb105c: strh r0, [r7], #2
    c.emit(arm32::store_halfword_postindexed(C::Al,R::R0,R::new(7)?,2)?)?;
    // ARM 0xb1060: ldrh r5, [r4, #24]
    c.emit(arm32::load_halfword(C::Al,R::new(5)?,R::new(4)?,24)?)?;
    // ARM 0xb1064: cmp r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // ARM 0xb1068: blo 0xb10dc <entry+0x498c>
    c.b(C::Cc,0xb10dc,false)?;
    // ARM 0xb106c: add r0, r1, #106
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,106)?)?;
    // ARM 0xb1070: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM 0xb1074: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM 0xb1078: add r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,512)?)?;
    // ARM 0xb107c: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // ARM 0xb1080: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // ARM 0xb1084: add r9, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(4)?,1)?)?;
    // ARM 0xb1088: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xb108c: ldrh r6, [r0]
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::R0,0)?)?;
    // ARM 0xb1090: mul r0, r9, r6
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(9)?,R::new(6)?)?)?;
    // ARM 0xb1094: blx 0x82be4 <entry+0xfffffffffffd6494>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // ARM 0xb1098: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // ARM 0xb109c: mul r0, r4, r6
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(4)?,R::new(6)?)?)?;
    // ARM 0xb10a0: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xb10a4: blx 0x82be4 <entry+0xfffffffffffd6494>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // ARM 0xb10a8: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // ARM 0xb10ac: sub r0, r8, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb10b0: mov r4, r9
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(9)?))?;
    // ARM 0xb10b4: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // ARM 0xb10b8: strh r0, [r7], #2
    c.emit(arm32::store_halfword_postindexed(C::Al,R::R0,R::new(7)?,2)?)?;
    // ARM 0xb10bc: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb10c0: add r10, r0, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(10)?,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb10c4: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // ARM 0xb10c8: ldrh r5, [r0, #24]
    c.emit(arm32::load_halfword(C::Al,R::new(5)?,R::R0,24)?)?;
    // ARM 0xb10cc: sub r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(5)?,1)?)?;
    // ARM 0xb10d0: cmp r9, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb10d4: blt 0xb1080 <entry+0x4930>
    c.b(C::Lt,0xb1080,false)?;
    // ARM 0xb10d8: b 0xb10e0 <entry+0x4990>
    c.b(C::Al,0xb10e0,false)?;
    // ARM 0xb10dc: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM 0xb10e0: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // ARM 0xb10e4: mov r0, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,106)?)?;
    // ARM 0xb10e8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xb10ec: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // ARM 0xb10f0: ldrh r0, [r1, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::R1,R::R0)?)?;
    // ARM 0xb10f4: sub r0, r0, r10
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb10f8: strh r0, [r7]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(7)?,0)?)?;
    // ARM 0xb10fc: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // ARM 0xb1100: b 0xb1160 <entry+0x4a10>
    c.b(C::Al,0xb1160,false)?;
    // ARM 0xb1104: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xb1108: sub r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb110c: strh r1, [r7], #2
    c.emit(arm32::store_halfword_postindexed(C::Al,R::R1,R::new(7)?,2)?)?;
    // ARM 0xb1110: mov r1, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,106)?)?;
    // ARM 0xb1114: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // ARM 0xb1118: ldrh r2, [r4, #24]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(4)?,24)?)?;
    // ARM 0xb111c: ldrh r1, [lr, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::LR,R::R1)?)?;
    // ARM 0xb1120: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // ARM 0xb1124: blo 0xb1154 <entry+0x4a04>
    c.b(C::Cc,0xb1154,false)?;
    // ARM 0xb1128: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // ARM 0xb112c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xb1130: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // ARM 0xb1134: ldrh r6, [r7]
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::new(7)?,0)?)?;
    // ARM 0xb1138: add r12, r12, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R12,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb113c: cmp r12, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1140: bgt 0xb1174 <entry+0x4a24>
    c.b(C::Gt,0xb1174,false)?;
    // ARM 0xb1144: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // ARM 0xb1148: add r7, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,2)?)?;
    // ARM 0xb114c: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb1150: blt 0xb1134 <entry+0x49e4>
    c.b(C::Lt,0xb1134,false)?;
    // ARM 0xb1154: sub r1, r1, r12
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // ARM 0xb1158: strh r1, [r7]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(7)?,0)?)?;
    // ARM 0xb115c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xb1160: ldr r1, [r0, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2432)?)?;
    // ARM 0xb1164: add r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb1168: str r1, [r0, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2432)?)?;
    // ARM 0xb116c: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xb1170: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xb1174: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb1178: mov r2, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,127)?)?;
    // ARM 0xb117c: strb r1, [r4, #0x13]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,19)?)?;
    // ARM 0xb1180: b 0xb1160 <entry+0x4a10>
    c.b(C::Al,0xb1160,false)?;
    c.check(0xb1184)?;
    // ARM 0xb1184: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb1188: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // ARM 0xb118c: ldr r1, [pc, #0x18]
    c.literal(R::R1,0xb11ac)?;
    // ARM 0xb1190: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb1194: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1198: ldr r1, [pc, #0x10]
    c.literal(R::R1,0xb11b0)?;
    // ARM 0xb119c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb11a0: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb11a4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb11a8: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb11ac)?;
    // ARM 0xb11ac: controller diagnostic/format data, not executable
    c.emit(0x000f7197)?;
    // ARM 0xb11b0: controller diagnostic/format data, not executable
    c.emit(0x000e2788)?;
    c.check(0xb11b4)
}
