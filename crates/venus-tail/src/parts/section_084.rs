
/// Program codec stream control registers, validate paired plane resources
/// and deliver updated MMIO state to the downstream controller. The original
/// conditional logging paths and 19-word non-executable literal pool are
/// retained, followed by a bounded capability helper and a branch stub.
fn register_codec_controller_mode_policies(c:&mut Code)->Result<(),String>{
    c.check(0xc3c14)?;
    // VMA 0xc3c14: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc3c18: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc3c1c: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // VMA 0xc3c20: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc3c24: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xc3c28: bl 0xc1b78 <entry+0xffffffffffffeb78> @ imm = #-0x20b8
    c.b(C::Al,0xc1b78,true)?;
    // VMA 0xc3c2c: ldr r5, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xc3c30: mov r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2048)?)?;
    // VMA 0xc3c34: mov r2, #318767104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,318767104)?)?;
    // VMA 0xc3c38: mov r10, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,120)?)?;
    // VMA 0xc3c3c: orr r10, r10, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,2048)?)?;
    // VMA 0xc3c40: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xc3c44: orr r1, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xc3c48: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xc3c4c: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc3c50: ldr r1, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,8)?)?;
    // VMA 0xc3c54: orr r1, r10, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(10)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xc3c58: ldr r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc3c5c: ldr r3, [pc, #0x534]        @ 0x1198 <entry+0x1198>
    c.literal(R::new(3)?,0xc4198)?;
    // VMA 0xc3c60: tst r2, r3
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc3c64: beq 0xc3c58 <entry+0xc58>   @ imm = #-0x14
    c.b(C::Eq,0xc3c58,false)?;
    // VMA 0xc3c68: ldr r1, [pc, #0x52c]        @ 0x119c <entry+0x119c>
    c.literal(R::R1,0xc419c)?;
    // VMA 0xc3c6c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3c70: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xc3c74: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94fe8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3c78: ldr r1, [pc, #0x520]        @ 0x11a0 <entry+0x11a0>
    c.literal(R::R1,0xc41a0)?;
    // VMA 0xc3c7c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3c80: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94ff4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3c84: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc3c88: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // VMA 0xc3c8c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc3c90: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xc3c94: orr r2, r0, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xc3c98: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xc3c9c: lsl r1, r1, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,15)?)?;
    // VMA 0xc3ca0: ldr r2, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc3ca4: tst r2, #-2147483647
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,2147483649)?)?;
    // VMA 0xc3ca8: beq 0xc3ce4 <entry+0xce4>   @ imm = #0x34
    c.b(C::Eq,0xc3ce4,false)?;
    // VMA 0xc3cac: orr r2, r1, r10
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc3cb0: ldr r2, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc3cb4: tst r2, #268435457
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,268435457)?)?;
    // VMA 0xc3cb8: beq 0xc3ce4 <entry+0xce4>   @ imm = #0x24
    c.b(C::Eq,0xc3ce4,false)?;
    // VMA 0xc3cbc: ldr r1, [pc, #0x4e0]        @ 0x11a4 <entry+0x11a4>
    c.literal(R::R1,0xc41a4)?;
    // VMA 0xc3cc0: ldr r2, [pc, #0x4e0]        @ 0x11a8 <entry+0x11a8>
    c.literal(R::R2,0xc41a8)?;
    // VMA 0xc3cc4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc3cc8: mov r3, #71
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,71)?)?;
    // VMA 0xc3ccc: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x95040
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3cd0: ldr r1, [pc, #0x4c8]        @ 0x11a0 <entry+0x11a0>
    c.literal(R::R1,0xc41a0)?;
    // VMA 0xc3cd4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc3cd8: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x9504c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3cdc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc3ce0: b 0xc3f94 <entry+0xf94>   @ imm = #0x2ac
    c.b(C::Al,0xc3f94,false)?;
    // VMA 0xc3ce4: orr r1, r1, #72
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,72)?)?;
    // VMA 0xc3ce8: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xc3cec: ldr r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc3cf0: ldr r1, [pc, #0x4b4]        @ 0x11ac <entry+0x11ac>
    c.literal(R::R1,0xc41ac)?;
    // VMA 0xc3cf4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc3cf8: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xc3cfc: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x95070
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3d00: ldr r1, [pc, #0x498]        @ 0x11a0 <entry+0x11a0>
    c.literal(R::R1,0xc41a0)?;
    // VMA 0xc3d04: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc3d08: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x9507c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3d0c: and r0, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(6)?,1)?)?;
    // VMA 0xc3d10: and r1, r7, r6, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(7)?,R::new(6)?,Shift::Lsr,4,false)?)?;
    // VMA 0xc3d14: str r0, [r4, #0x23c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,572)?)?;
    // VMA 0xc3d18: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xc3d1c: and r0, r0, r6, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(6)?,Shift::Lsr,1,false)?)?;
    // VMA 0xc3d20: strb r0, [r4, #0x24c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,588)?)?;
    // VMA 0xc3d24: strb r1, [r4, #0x258]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,600)?)?;
    // VMA 0xc3d28: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // VMA 0xc3d2c: ldr r1, [pc, #0x47c]        @ 0x11b0 <entry+0x11b0>
    c.literal(R::R1,0xc41b0)?;
    // VMA 0xc3d30: and r1, r1, r6, lsr #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(6)?,Shift::Lsr,5,false)?)?;
    // VMA 0xc3d34: str r1, [r4, #0x254]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,596)?)?;
    // VMA 0xc3d38: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // VMA 0xc3d3c: str r7, [r1, #0x454]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,1108)?)?;
    // VMA 0xc3d40: bhs 0xc3df8 <entry+0xdf8>   @ imm = #0xb0
    c.b(C::Cs,0xc3df8,false)?;
    // VMA 0xc3d44: ldr r2, [pc, #0x46c]        @ 0x11b8 <entry+0x11b8>
    c.literal(R::R2,0xc41b8)?;
    // VMA 0xc3d48: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc3d4c: ldr r0, [r2, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R2,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xc3d50: ldr r2, [pc, #0x464]        @ 0x11bc <entry+0x11bc>
    c.literal(R::R2,0xc41bc)?;
    // VMA 0xc3d54: str r0, [r1, #0x458]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,1112)?)?;
    // VMA 0xc3d58: str r6, [r5, #0xa90]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,2704)?)?;
    // VMA 0xc3d5c: mov r1, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,124)?)?;
    // VMA 0xc3d60: ldr r0, [r5, #0x278]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,632)?)?;
    // VMA 0xc3d64: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc3d68: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc3d6c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3d70: bl 0x17b0 <entry+0xfffffffffff3e7b0> @ imm = #-0xc25c8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc3d74: ldr r0, [r5, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,972)?)?;
    // VMA 0xc3d78: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc3d7c: beq 0xc3d9c <entry+0xd9c>   @ imm = #0x18
    c.b(C::Eq,0xc3d9c,false)?;
    // VMA 0xc3d80: ldr r0, [r5, #0x278]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,632)?)?;
    // VMA 0xc3d84: ldr r2, [pc, #0x430]        @ 0x11bc <entry+0x11bc>
    c.literal(R::R2,0xc41bc)?;
    // VMA 0xc3d88: mov r1, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,127)?)?;
    // VMA 0xc3d8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc3d90: moveq r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // VMA 0xc3d94: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xc3d98: bl 0x17b0 <entry+0xfffffffffff3e7b0> @ imm = #-0xc25f0
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc3d9c: ldr r0, [r4, #0x23c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,572)?)?;
    // VMA 0xc3da0: mov r2, #556
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,556)?)?;
    // VMA 0xc3da4: str r0, [r4, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,560)?)?;
    // VMA 0xc3da8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc3dac: ldr r1, [r5, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,476)?)?;
    // VMA 0xc3db0: strh r1, [r4, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(4)?,R::R2)?)?;
    // VMA 0xc3db4: mov r2, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,46)?)?;
    // VMA 0xc3db8: ldr r1, [r5, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,480)?)?;
    // VMA 0xc3dbc: orr r2, r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,512)?)?;
    // VMA 0xc3dc0: strh r1, [r4, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(4)?,R::R2)?)?;
    // VMA 0xc3dc4: beq 0xc3e30 <entry+0xe30>   @ imm = #0x64
    c.b(C::Eq,0xc3e30,false)?;
    // VMA 0xc3dc8: ldr r0, [r5, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2700)?)?;
    // VMA 0xc3dcc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc3dd0: bne 0xc3f7c <entry+0xf7c>   @ imm = #0x1a4
    c.b(C::Ne,0xc3f7c,false)?;
    // VMA 0xc3dd4: ldr r1, [pc, #0x3e4]        @ 0x11c0 <entry+0x11c0>
    c.literal(R::R1,0xc41c0)?;
    // VMA 0xc3dd8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3ddc: str r0, [r5, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,504)?)?;
    // VMA 0xc3de0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3de4: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x95158
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3de8: ldr r1, [pc, #0x3b0]        @ 0x11a0 <entry+0x11a0>
    c.literal(R::R1,0xc41a0)?;
    // VMA 0xc3dec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3df0: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x95164
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3df4: b 0xc3f7c <entry+0xf7c>   @ imm = #0x180
    c.b(C::Al,0xc3f7c,false)?;
    // VMA 0xc3df8: ldr r1, [pc, #0x3b4]        @ 0x11b4 <entry+0x11b4>
    c.literal(R::R1,0xc41b4)?;
    // VMA 0xc3dfc: ldr r2, [pc, #0x3a4]        @ 0x11a8 <entry+0x11a8>
    c.literal(R::R2,0xc41a8)?;
    // VMA 0xc3e00: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xc3e04: mov r3, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,116)?)?;
    // VMA 0xc3e08: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x9517c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3e0c: ldr r1, [pc, #0x38c]        @ 0x11a0 <entry+0x11a0>
    c.literal(R::R1,0xc41a0)?;
    // VMA 0xc3e10: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xc3e14: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x95188
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3e18: ldr r0, [r4, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,676)?)?;
    // VMA 0xc3e1c: mov r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // VMA 0xc3e20: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc3e24: str r1, [r0, #0x81c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2076)?)?;
    // VMA 0xc3e28: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc3e2c: b 0xc3f94 <entry+0xf94>   @ imm = #0x160
    c.b(C::Al,0xc3f94,false)?;
    // VMA 0xc3e30: ldr r1, [pc, #0x38c]        @ 0x11c4 <entry+0x11c4>
    c.literal(R::R1,0xc41c4)?;
    // VMA 0xc3e34: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc3e38: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x951ac
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3e3c: ldr r8, [pc, #0x35c]        @ 0x11a0 <entry+0x11a0>
    c.literal(R::new(8)?,0xc41a0)?;
    // VMA 0xc3e40: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc3e44: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xc3e48: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x951bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3e4c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc3e50: mov r1, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,76)?)?;
    // VMA 0xc3e54: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xc3e58: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // VMA 0xc3e5c: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xc3e60: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc3e64: ldr r7, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3e68: ldr r1, [pc, #0x358]        @ 0x11c8 <entry+0x11c8>
    c.literal(R::R1,0xc41c8)?;
    // VMA 0xc3e6c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc3e70: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xc3e74: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x951e8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3e78: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc3e7c: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xc3e80: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x951f4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3e84: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // VMA 0xc3e88: lsr r2, r7, #30
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(7)?,Shift::Lsr,30)?)?;
    // VMA 0xc3e8c: and r0, r0, r7, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(7)?,Shift::Lsr,8,false)?)?;
    // VMA 0xc3e90: lsr r1, r0, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Lsr,6)?)?;
    // VMA 0xc3e94: str r1, [r4, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,564)?)?;
    // VMA 0xc3e98: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc3e9c: str r2, [r4, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,568)?)?;
    // VMA 0xc3ea0: strb r1, [r4, #0x224]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,548)?)?;
    // VMA 0xc3ea4: mov r1, #16128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16128)?)?;
    // VMA 0xc3ea8: and r0, r1, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,8,false)?)?;
    // VMA 0xc3eac: and r1, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(7)?,255)?)?;
    // VMA 0xc3eb0: orr r6, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc3eb4: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // VMA 0xc3eb8: orr r0, r0, #16128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16128)?)?;
    // VMA 0xc3ebc: str r6, [r5, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,476)?)?;
    // VMA 0xc3ec0: and r9, r0, r7, lsr #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::R0,R::new(7)?,Shift::Lsr,16,false)?)?;
    // VMA 0xc3ec4: ands r0, r7, #15
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::new(7)?,15,true)?)?;
    // VMA 0xc3ec8: rsb r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,16)?)?;
    // VMA 0xc3ecc: str r9, [r5, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,480)?)?;
    // VMA 0xc3ed0: addne r6, r6, r0
    c.emit(arm32::data_register(C::Ne,Op::Add,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc3ed4: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // VMA 0xc3ed8: and r0, r0, r7, lsr #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(7)?,Shift::Lsr,16,false)?)?;
    // VMA 0xc3edc: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xc3ee0: rsb r1, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R0,16)?)?;
    // VMA 0xc3ee4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc3ee8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc3eec: addne r9, r9, r1
    c.emit(arm32::data_register(C::Ne,Op::Add,R::new(9)?,R::new(9)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc3ef0: ldr r1, [pc, #0x2d4]        @ 0x11cc <entry+0x11cc>
    c.literal(R::R1,0xc41cc)?;
    // VMA 0xc3ef4: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // VMA 0xc3ef8: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x9526c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3efc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc3f00: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xc3f04: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x95278
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3f08: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc3f0c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc3f10: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // VMA 0xc3f14: bl 0xc46e4 <entry+0x16e4>  @ imm = #0x7c8
    c.b(C::Al,0xc46e4,true)?;
    // VMA 0xc3f18: ldr r0, [r5, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,484)?)?;
    // VMA 0xc3f1c: cmp r6, r0, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,4,false)?)?;
    // VMA 0xc3f20: ldrls r1, [r5, #0x1e8]
    c.emit(arm32::transfer_word(C::Ls,true,R::R1,R::new(5)?,488)?)?;
    // VMA 0xc3f24: cmpls r9, r1, lsl #4
    c.emit(arm32::data_register(C::Ls,Op::Cmp,R::R0,R::new(9)?,R::R1,Shift::Lsl,4,false)?)?;
    // VMA 0xc3f28: bls 0xc40bc <entry+0x10bc>  @ imm = #0x18c
    c.b(C::Ls,0xc40bc,false)?;
    // VMA 0xc3f2c: ldr r1, [pc, #0x2a0]        @ 0x11d4 <entry+0x11d4>
    c.literal(R::R1,0xc41d4)?;
    // VMA 0xc3f30: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3f34: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x952a8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3f38: ldr r1, [pc, #0x260]        @ 0x11a0 <entry+0x11a0>
    c.literal(R::R1,0xc41a0)?;
    // VMA 0xc3f3c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3f40: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x952b4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3f44: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc3f48: str r0, [r5, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,504)?)?;
    // VMA 0xc3f4c: lsr r0, r6, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(6)?,Shift::Lsr,4)?)?;
    // VMA 0xc3f50: lsr r1, r9, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(9)?,Shift::Lsr,4)?)?;
    // VMA 0xc3f54: str r0, [r5, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,484)?)?;
    // VMA 0xc3f58: str r1, [r5, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,488)?)?;
    // VMA 0xc3f5c: ldr r0, [r5, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,476)?)?;
    // VMA 0xc3f60: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // VMA 0xc3f64: bic r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // VMA 0xc3f68: str r0, [r5, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,468)?)?;
    // VMA 0xc3f6c: ldr r0, [r5, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,480)?)?;
    // VMA 0xc3f70: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // VMA 0xc3f74: bic r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // VMA 0xc3f78: str r0, [r5, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,472)?)?;
    // VMA 0xc3f7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc3f80: str r0, [r5, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2700)?)?;
    // VMA 0xc3f84: ldr r0, [r4, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,456)?)?;
    // VMA 0xc3f88: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xc3f8c: str r0, [r4, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,456)?)?;
    // VMA 0xc3f90: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3f94: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // VMA 0xc3f98: mov r7, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,52)?)?;
    // VMA 0xc3f9c: mov r3, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2096)?)?;
    // VMA 0xc3fa0: mov r5, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xc3fa4: mov r6, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,13)?)?;
    // VMA 0xc3fa8: orr r7, r7, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2048)?)?;
    // VMA 0xc3fac: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // VMA 0xc3fb0: ldr r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,8)?)?;
    // VMA 0xc3fb4: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc3fb8: orr r7, r7, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc3fbc: ldr r3, [r3, r5]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc3fc0: ldr r7, [r7, r5]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc3fc4: add r3, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc3fc8: lsr r7, r3, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(3)?,Shift::Lsr,3)?)?;
    // VMA 0xc3fcc: str r7, [r4, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,440)?)?;
    // VMA 0xc3fd0: ldr r7, [r4, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,676)?)?;
    // VMA 0xc3fd4: ldr r7, [r7, #0x81c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,2076)?)?;
    // VMA 0xc3fd8: cmp r7, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc3fdc: beq 0xc4020 <entry+0x1020>  @ imm = #0x3c
    c.b(C::Eq,0xc4020,false)?;
    // VMA 0xc3fe0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc3fe4: beq 0xc3ff0 <entry+0xff0>   @ imm = #0x4
    c.b(C::Eq,0xc3ff0,false)?;
    // VMA 0xc3fe8: ands r3, r3, #7
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(3)?,R::new(3)?,7,true)?)?;
    // VMA 0xc3fec: beq 0xc4020 <entry+0x1020>  @ imm = #0x2c
    c.b(C::Eq,0xc4020,false)?;
    // VMA 0xc3ff0: ldr r1, [pc, #0x1e0]        @ 0x11d8 <entry+0x11d8>
    c.literal(R::R1,0xc41d8)?;
    // VMA 0xc3ff4: ldr r2, [pc, #0x1e0]        @ 0x11dc <entry+0x11dc>
    c.literal(R::R2,0xc41dc)?;
    // VMA 0xc3ff8: mov r3, #169
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,169)?)?;
    // VMA 0xc3ffc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc4000: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xc4004: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95378
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4008: ldr r1, [pc, #0x190]        @ 0x1a0 <entry+0x1a0>
    c.literal(R::R1,0xc41a0)?;
    // VMA 0xc400c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc4010: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95384
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4014: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // VMA 0xc4018: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc401c: ldr r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,8)?)?;
    // VMA 0xc4020: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // VMA 0xc4024: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // VMA 0xc4028: orr r2, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc402c: ldr r3, [r2, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc4030: bic r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xc4034: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // VMA 0xc4038: ldr r3, [pc, #0x1a0]        @ 0x1e0 <entry+0x1e0>
    c.literal(R::new(3)?,0xc41e0)?;
    // VMA 0xc403c: ldr r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,8)?)?;
    // VMA 0xc4040: ldr r3, [r3, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
    // VMA 0xc4044: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // VMA 0xc4048: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc404c: beq 0xc4064 <entry+0x64>    @ imm = #0x10
    c.b(C::Eq,0xc4064,false)?;
    // VMA 0xc4050: orr r1, r2, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,1040)?)?;
    // VMA 0xc4054: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xc4058: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xc405c: str r3, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc4060: b 0xc40b4 <entry+0xb4>    @ imm = #0x4c
    c.b(C::Al,0xc40b4,false)?;
    // VMA 0xc4064: orr r2, r2, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,124)?)?;
    // VMA 0xc4068: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // VMA 0xc406c: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // VMA 0xc4070: orr r3, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R2,2048)?)?;
    // VMA 0xc4074: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xc4078: str r7, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc407c: mov r7, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,44)?)?;
    // VMA 0xc4080: ldr r3, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,8)?)?;
    // VMA 0xc4084: orr r7, r7, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2048)?)?;
    // VMA 0xc4088: orr r3, r7, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc408c: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc4090: ldr r3, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,8)?)?;
    // VMA 0xc4094: orr r3, r7, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc4098: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xc409c: str r7, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc40a0: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,8)?)?;
    // VMA 0xc40a4: orr r1, r10, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(10)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xc40a8: ldr r3, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc40ac: tst r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,2)?)?;
    // VMA 0xc40b0: beq 0xc40a8 <entry+0xa8>    @ imm = #-0x10
    c.b(C::Eq,0xc40a8,false)?;
    // VMA 0xc40b4: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc40b8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc40bc: ldr r2, [r5, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,192)?)?;
    // VMA 0xc40c0: ldr r3, [r5, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,196)?)?;
    // VMA 0xc40c4: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc40c8: blo 0xc3f2c <entry+0xffffffffffffff2c> @ imm = #-0x1a4
    c.b(C::Cc,0xc3f2c,false)?;
    // VMA 0xc40cc: ldr r2, [r5, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,972)?)?;
    // VMA 0xc40d0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc40d4: beq 0xc40f8 <entry+0xf8>    @ imm = #0x1c
    c.b(C::Eq,0xc40f8,false)?;
    // VMA 0xc40d8: ldr r2, [r5, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,248)?)?;
    // VMA 0xc40dc: ldr r3, [r5, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,416)?)?;
    // VMA 0xc40e0: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc40e4: blo 0xc3f2c <entry+0xffffffffffffff2c> @ imm = #-0x1c0
    c.b(C::Cc,0xc3f2c,false)?;
    // VMA 0xc40e8: ldr r2, [r5, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,236)?)?;
    // VMA 0xc40ec: ldr r3, [r5, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,412)?)?;
    // VMA 0xc40f0: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc40f4: blo 0xc3f2c <entry+0xffffffffffffff2c> @ imm = #-0x1d0
    c.b(C::Cc,0xc3f2c,false)?;
    // VMA 0xc40f8: lsl r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,4)?)?;
    // VMA 0xc40fc: cmp r6, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc4100: lslhs r0, r1, #4
    c.emit(arm32::mov_shifted_register(C::Cs,R::R0,R::R1,Shift::Lsl,4)?)?;
    // VMA 0xc4104: cmphs r9, r0
    c.emit(arm32::data_register(C::Cs,Op::Cmp,R::R0,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc4108: bhs 0xc415c <entry+0x15c>   @ imm = #0x4c
    c.b(C::Cs,0xc415c,false)?;
    // VMA 0xc410c: ldr r0, [r5, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,140)?)?;
    // VMA 0xc4110: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc4114: beq 0xc4124 <entry+0x124>   @ imm = #0x8
    c.b(C::Eq,0xc4124,false)?;
    // VMA 0xc4118: ldr r0, [r5, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,456)?)?;
    // VMA 0xc411c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc4120: beq 0xc418c <entry+0x18c>   @ imm = #0x64
    c.b(C::Eq,0xc418c,false)?;
    // VMA 0xc4124: ldr r0, [r5, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,144)?)?;
    // VMA 0xc4128: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc412c: beq 0xc413c <entry+0x13c>   @ imm = #0x8
    c.b(C::Eq,0xc413c,false)?;
    // VMA 0xc4130: ldr r0, [r5, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,452)?)?;
    // VMA 0xc4134: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc4138: beq 0xc418c <entry+0x18c>   @ imm = #0x4c
    c.b(C::Eq,0xc418c,false)?;
    // VMA 0xc413c: ldr r1, [pc, #0x8c]         @ 0x1d0 <entry+0x1d0>
    c.literal(R::R1,0xc41d0)?;
    // VMA 0xc4140: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4144: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xc4148: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x954bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc414c: ldr r1, [pc, #0x4c]         @ 0x1a0 <entry+0x1a0>
    c.literal(R::R1,0xc41a0)?;
    // VMA 0xc4150: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4154: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x954c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4158: b 0xc4190 <entry+0x190>   @ imm = #0x30
    c.b(C::Al,0xc4190,false)?;
    // VMA 0xc415c: ldr r0, [r5, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2700)?)?;
    // VMA 0xc4160: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc4164: bne 0xc3f4c <entry+0xffffffffffffff4c> @ imm = #-0x220
    c.b(C::Ne,0xc3f4c,false)?;
    // VMA 0xc4168: ldr r1, [pc, #0x50]         @ 0x1c0 <entry+0x1c0>
    c.literal(R::R1,0xc41c0)?;
    // VMA 0xc416c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4170: str r0, [r5, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,504)?)?;
    // VMA 0xc4174: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4178: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x954ec
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc417c: ldr r1, [pc, #0x1c]         @ 0x1a0 <entry+0x1a0>
    c.literal(R::R1,0xc41a0)?;
    // VMA 0xc4180: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4184: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x954f8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4188: b 0xc3f4c <entry+0xffffffffffffff4c> @ imm = #-0x244
    c.b(C::Al,0xc3f4c,false)?;
    // VMA 0xc418c: mov r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // VMA 0xc4190: str r7, [r5, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,504)?)?;
    // VMA 0xc4194: b 0xc3f4c <entry+0xffffffffffffff4c> @ imm = #-0x250
    c.b(C::Al,0xc3f4c,false)?;

    c.check(0xc4198)?;
    const CODEC_MODE_POLICY_DATA:[u32;19]=[
       0x10000401,0x000fac88,0x000e2788,0x000faca2,
       0x000fb50e,0x000face1,0x0007ffff,0x000fb528,
       0x000fbbf8,0x000fb3df,0x000fb604,0x000fb555,
       0x000fb56c,0x000fac1b,0x000fb5d0,0x000fb59a,
       0x000fb2d0,0x000fb304,0x0010408c,
    ];
    for word in CODEC_MODE_POLICY_DATA {c.emit(word)?;}
    c.check(0xc41e4)?;
    // VMA 0xc41e4: cmn r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // VMA 0xc41e8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc41ec: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc41f0: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc41f4: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // VMA 0xc41f8: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc41fc: and r0, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc4200: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xc4204: lsl r0, r0, #14
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,14)?)?;
    // VMA 0xc4208: b 0x1029e8 <entry+0x3e9e8> @ imm = #0x3e7d8
    c.b(C::Al,0x1029e8,false)?;

    c.check(0xc420c)
}
