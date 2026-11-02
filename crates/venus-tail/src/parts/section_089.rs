

/// Extend the active controller record initialization through its three
/// original inline data pools. Those nine words are controller diagnostics
/// and a global mapping address, *not* executable A32 instructions.
fn initialize_controller_record_suffix(c: &mut Code) -> Result<(),String> {
    c.check(0xc5aec)?;
    const INITIALIZATION_DIAGNOSTICS:[u32;4]=[
        0x000f_b3df,0x000f_ad07,0x000e_2788,0x1000_0801,
    ];
    for addr in INITIALIZATION_DIAGNOSTICS { c.emit(addr)?; }
    c.check(0xc5afc)?;
    // VMA 0xc5afc: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xc5b00: ldr r1, [pc, #0x7f8]
    c.literal(R::R1,0xc6300)?;
    // VMA 0xc5b04: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc5b08: str r2, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,504)?)?;
    // VMA 0xc5b0c: bl 0x2ec94 <entry+0xfffffffffff691a8>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5b10: ldr r1, [pc, #0x7b0]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc5b14: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc5b18: bl 0x2ec94 <entry+0xfffffffffff691a8>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5b1c: ldr r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,112)?)?;
    // VMA 0xc5b20: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // VMA 0xc5b24: str r0, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,468)?)?;
    // VMA 0xc5b28: ldr r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,108)?)?;
    // VMA 0xc5b2c: str r0, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,472)?)?;
    // VMA 0xc5b30: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc5b34: mov r4, r6
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(6)?))?;
    // VMA 0xc5b38: str r0, [r6, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2700)?)?;
    // VMA 0xc5b3c: ldr r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,112)?)?;
    // VMA 0xc5b40: lsr r1, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Lsr,4)?)?;
    // VMA 0xc5b44: str r1, [r6, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,484)?)?;
    // VMA 0xc5b48: ldr r1, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,108)?)?;
    // VMA 0xc5b4c: lsr r2, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsr,4)?)?;
    // VMA 0xc5b50: str r2, [r6, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,488)?)?;
    // VMA 0xc5b54: str r0, [r6, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,476)?)?;
    // VMA 0xc5b58: str r1, [r6, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,480)?)?;
    // VMA 0xc5b5c: ldr r0, [r6, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,468)?)?;
    // VMA 0xc5b60: str r0, [r10, #0x580]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1408)?)?;
    // VMA 0xc5b64: ldr r0, [r6, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,472)?)?;
    // VMA 0xc5b68: str r0, [r10, #0x584]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1412)?)?;
    // VMA 0xc5b6c: ldr r6, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,32)?)?;
    // VMA 0xc5b70: ldr r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,504)?)?;
    // VMA 0xc5b74: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5b78: beq 0xc5bb4 <entry+0xc8>
    c.b(C::Eq,0xc5bb4,false)?;
    // VMA 0xc5b7c: ldr r0, [r4, #0x910]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2320)?)?;
    // VMA 0xc5b80: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5b84: ldrne r0, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xc5b88: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5b8c: beq 0xc5bb4 <entry+0xc8>
    c.b(C::Eq,0xc5bb4,false)?;
    // VMA 0xc5b90: ldr r0, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1296)?)?;
    // VMA 0xc5b94: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc5b98: bne 0xc5bb4 <entry+0xc8>
    c.b(C::Ne,0xc5bb4,false)?;
    // VMA 0xc5b9c: ldr r1, [pc, #0x770]
    c.literal(R::R1,0xc6314)?;
    // VMA 0xc5ba0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc5ba4: bl 0x2ec94 <entry+0xfffffffffff691a8>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5ba8: ldr r1, [pc, #0x718]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc5bac: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc5bb0: bl 0x2ec94 <entry+0xfffffffffff691a8>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5bb4: ldrb r0, [r10, #0x522]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1314)?)?;
    // VMA 0xc5bb8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5bbc: beq 0xc5bd8 <entry+0xec>
    c.b(C::Eq,0xc5bd8,false)?;
    // VMA 0xc5bc0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc5bc4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc5bc8: b 0xc5be4 <entry+0xf8>
    c.b(C::Al,0xc5be4,false)?;
    c.check(0xc5bcc)?;
    const CHANNEL_MODE_MESSAGES:[u32;3]=[
        0x000f_b65b,0x000f_b641,0x000f_b679,
    ];
    for addr in CHANNEL_MODE_MESSAGES { c.emit(addr)?; }
    c.check(0xc5bd8)?;
    // VMA 0xc5bd8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc5bdc: and r1, r2, r6, lsr #29
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::new(6)?,Shift::Lsr,29,false)?)?;
    // VMA 0xc5be0: and r2, r2, r6, lsr #28
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsr,28,false)?)?;
    // VMA 0xc5be4: strb r2, [r10, #0x56f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,1391)?)?;
    // VMA 0xc5be8: strb r1, [r10, #0x570]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,1392)?)?;
    // VMA 0xc5bec: lsr r1, r6, #30
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(6)?,Shift::Lsr,30)?)?;
    // VMA 0xc5bf0: strb r1, [r10, #0x571]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,1393)?)?;
    // VMA 0xc5bf4: ldr r1, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1296)?)?;
    // VMA 0xc5bf8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc5bfc: beq 0xc5c0c <entry+0x120>
    c.b(C::Eq,0xc5c0c,false)?;
    // VMA 0xc5c00: ldrb r1, [r10, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1355)?)?;
    // VMA 0xc5c04: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc5c08: beq 0xc5f8c <entry+0x4a0>
    c.b(C::Eq,0xc5f8c,false)?;
    // VMA 0xc5c0c: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // VMA 0xc5c10: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xc5c14: bne 0xc5c60 <entry+0x174>
    c.b(C::Ne,0xc5c60,false)?;
    // VMA 0xc5c18: add r0, r9, #748
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,748)?)?;
    // VMA 0xc5c1c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc5c20: mov r2, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,96)?)?;
    // VMA 0xc5c24: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xc5c28: bl 0x31ec <entry+0xfffffffffff3d700>
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xc5c2c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc5c30: mov r0, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,716)?)?;
    // VMA 0xc5c34: strb r4, [r9, #0x2df]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(9)?,735)?)?;
    // VMA 0xc5c38: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xc5c3c: strh r1, [r9, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(9)?,R::R0)?)?;
    // VMA 0xc5c40: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5c44: strb r0, [r9, #0x2d2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,722)?)?;
    // VMA 0xc5c48: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // VMA 0xc5c4c: strb r4, [r9, #0x2d3]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(9)?,723)?)?;
    // VMA 0xc5c50: strb r0, [r9, #0x2d4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,724)?)?;
    // VMA 0xc5c54: strb r0, [r9, #0x2d5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,725)?)?;
    // VMA 0xc5c58: strb r4, [r9, #0x2d8]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(9)?,728)?)?;
    // VMA 0xc5c5c: strb r4, [r9, #0x2d9]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::new(9)?,729)?)?;
    // VMA 0xc5c60: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xc5c64: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xc5c68: bl 0xc47c8 <entry+0xffffffffffffecdc>
    c.b(C::Al,0xc47c8,true)?;
    // VMA 0xc5c6c: mov r0, #556
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,556)?)?;
    // VMA 0xc5c70: mov r5, r9
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // VMA 0xc5c74: ldrh r1, [r5, r0]!
    c.emit(arm32::load_halfword_register_preindexed_writeback(C::Al,R::R1,R::new(5)?,R::R0)?)?;
    // VMA 0xc5c78: ldr r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,112)?)?;
    // VMA 0xc5c7c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc5c80: bne 0xc5cd8 <entry+0x1ec>
    c.b(C::Ne,0xc5cd8,false)?;
    // VMA 0xc5c84: mov r1, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,46)?)?;
    // VMA 0xc5c88: ldr r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc5c8c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // VMA 0xc5c90: ldrh r2, [r9, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R2,R::new(9)?,R::R1)?)?;
    // VMA 0xc5c94: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc5c98: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc5c9c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc5ca0: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc5ca4: strb r2, [r10, #0x54a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,1354)?)?;
    // VMA 0xc5ca8: ldrbeq r2, [r10, #0x522]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R2,R::new(10)?,1314)?)?;
    // VMA 0xc5cac: cmpeq r2, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc5cb0: bne 0xc5ce0 <entry+0x1f4>
    c.b(C::Ne,0xc5ce0,false)?;
    // VMA 0xc5cb4: ldrb r2, [r10, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,1355)?)?;
    // VMA 0xc5cb8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc5cbc: bne 0xc5ce0 <entry+0x1f4>
    c.b(C::Ne,0xc5ce0,false)?;
    // VMA 0xc5cc0: ldrb r1, [r9, #0x229]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,553)?)?;
    // VMA 0xc5cc4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc5cc8: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc5ccc: b 0xc5ce0 <entry+0x1f4>
    c.b(C::Al,0xc5ce0,false)?;
    c.check(0xc5cd0)?;
    const BUFFER_STATE_WARNINGS:[u32;2]=[0x000f_ad7c,0x000f_b7bb];
    for addr in BUFFER_STATE_WARNINGS { c.emit(addr)?; }
    c.check(0xc5cd8)
}


/// Record live codec frame attributes and publish updated output fields.
/// The processor's conditional branches retain their original conditions.
/// Small intervening diagnostic pointers are data, not A32 instructions.
fn process_live_frame_registration(c:&mut Code) -> Result<(),String> {
    c.check(0xc5cd8)?;
    // VMA 0xc5cd8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc5cdc: strb r1, [r10, #0x54a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,1354)?)?;
    // VMA 0xc5ce0: strb r1, [r10, #0x549]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,1353)?)?;
    // VMA 0xc5ce4: ldr r1, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1296)?)?;
    // VMA 0xc5ce8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc5cec: beq 0xc5d08 <entry+0x30>
    c.b(C::Eq,0xc5d08,false)?;
    // VMA 0xc5cf0: ldrb r2, [r10, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,1355)?)?;
    // VMA 0xc5cf4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc5cf8: bne 0xc5d08 <entry+0x30>
    c.b(C::Ne,0xc5d08,false)?;
    // VMA 0xc5cfc: ldrb r2, [r10, #0x522]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,1314)?)?;
    // VMA 0xc5d00: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc5d04: beq 0xc5d40 <entry+0x68>
    c.b(C::Eq,0xc5d40,false)?;
    // VMA 0xc5d08: ldr r1, [pc, #0x608]
    c.literal(R::R1,0xc6318)?;
    // VMA 0xc5d0c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5d10: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5d14: ldr r1, [pc, #0x5ac]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc5d18: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5d1c: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5d20: mov r0, #1360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1360)?)?;
    // VMA 0xc5d24: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc5d28: strh r1, [r10, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(10)?,R::R0)?)?;
    // VMA 0xc5d2c: mov r0, #78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,78)?)?;
    // VMA 0xc5d30: orr r0, r0, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1280)?)?;
    // VMA 0xc5d34: strh r1, [r10, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(10)?,R::R0)?)?;
    // VMA 0xc5d38: ldr r1, [r10, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,1296)?)?;
    // VMA 0xc5d3c: ldr r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,112)?)?;
    // VMA 0xc5d40: ldrb r2, [r9, #0x224]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,548)?)?;
    // VMA 0xc5d44: strb r2, [r10, #0x544]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,1348)?)?;
    // VMA 0xc5d48: ldrb r2, [r9, #0x225]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,549)?)?;
    // VMA 0xc5d4c: strb r2, [r10, #0x545]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,1349)?)?;
    // VMA 0xc5d50: ldrb r2, [r9, #0x226]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,550)?)?;
    // VMA 0xc5d54: strb r2, [r10, #0x546]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,1350)?)?;
    // VMA 0xc5d58: ldrb r2, [r9, #0x227]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,551)?)?;
    // VMA 0xc5d5c: strb r2, [r10, #0x547]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,1351)?)?;
    // VMA 0xc5d60: ldrb r2, [r9, #0x228]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,552)?)?;
    // VMA 0xc5d64: strb r2, [r10, #0x548]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,1352)?)?;
    // VMA 0xc5d68: strh r0, [r5]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc5d6c: mov r2, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,46)?)?;
    // VMA 0xc5d70: ldr r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,108)?)?;
    // VMA 0xc5d74: orr r2, r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,512)?)?;
    // VMA 0xc5d78: strh r3, [r9, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::new(3)?,R::new(9)?,R::R2)?)?;
    // VMA 0xc5d7c: ldr r2, [r9, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,560)?)?;
    // VMA 0xc5d80: str r2, [r10, #0x514]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,1300)?)?;
    // VMA 0xc5d84: str r1, [r9, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,560)?)?;
    // VMA 0xc5d88: ldrb r2, [r9, #0x229]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,553)?)?;
    // VMA 0xc5d8c: str r2, [r10, #0x58c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,1420)?)?;
    // VMA 0xc5d90: ldr r7, [r10, #0x588]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,1416)?)?;
    // VMA 0xc5d94: strb r7, [r9, #0x229]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(9)?,553)?)?;
    // VMA 0xc5d98: ldr r2, [r9, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,456)?)?;
    // VMA 0xc5d9c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc5da0: ldr r1, [pc, #0x574]
    c.literal(R::R1,0xc631c)?;
    // VMA 0xc5da4: add r6, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,1)?)?;
    // VMA 0xc5da8: str r6, [r9, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(9)?,456)?)?;
    // VMA 0xc5dac: ldrb r6, [r10, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(10)?,1312)?)?;
    // VMA 0xc5db0: ldrb r5, [r10, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(10)?,1355)?)?;
    // VMA 0xc5db4: stm sp, {r0, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc5db8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc5dbc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc5dc0: str r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // VMA 0xc5dc4: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xc5dc8: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5dcc: ldr r1, [pc, #0x4f4]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc5dd0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc5dd4: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5dd8: ldr r2, [r9, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,456)?)?;
    // VMA 0xc5ddc: ldrb r6, [r10, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(10)?,1312)?)?;
    // VMA 0xc5de0: ldrb r0, [r10, #0x522]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1314)?)?;
    // VMA 0xc5de4: ldrb r1, [r10, #0x54c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1356)?)?;
    // VMA 0xc5de8: ldrb r7, [r10, #0x56f]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(10)?,1391)?)?;
    // VMA 0xc5dec: ldrb r3, [r10, #0x571]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,1393)?)?;
    // VMA 0xc5df0: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // VMA 0xc5df4: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc5df8: beq 0xc5e04 <entry+0x12c>
    c.b(C::Eq,0xc5e04,false)?;
    // VMA 0xc5dfc: ldrb r6, [r10, #0x521]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(10)?,1313)?)?;
    // VMA 0xc5e00: b 0xc5e0c <entry+0x134>
    c.b(C::Al,0xc5e0c,false)?;
    // VMA 0xc5e04: mov r6, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,15)?)?;
    // VMA 0xc5e08: orr r6, r6, #9984
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,9984)?)?;
    // VMA 0xc5e0c: stm sp, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xc5e10: ldr r1, [pc, #0x508]
    c.literal(R::R1,0xc6320)?;
    // VMA 0xc5e14: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc5e18: str r6, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    // VMA 0xc5e1c: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5e20: ldr r1, [pc, #0x4a0]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc5e24: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc5e28: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5e2c: ldr r0, [r10, #0x588]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1416)?)?;
    // VMA 0xc5e30: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5e34: beq 0xc5e74 <entry+0x19c>
    c.b(C::Eq,0xc5e74,false)?;
    // VMA 0xc5e38: ldr r1, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,108)?)?;
    // VMA 0xc5e3c: ldr r2, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,112)?)?;
    // VMA 0xc5e40: ldr r0, [r9, #0x378]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,888)?)?;
    // VMA 0xc5e44: mul r3, r2, r1
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R1)?)?;
    // VMA 0xc5e48: add r1, r3, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // VMA 0xc5e4c: add r3, r0, r1, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::R1,Shift::Lsr,1,false)?)?;
    // VMA 0xc5e50: ldr r1, [pc, #0x4cc]
    c.literal(R::R1,0xc6324)?;
    // VMA 0xc5e54: str r3, [r9, #0x378]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(9)?,888)?)?;
    // VMA 0xc5e58: ldr r0, [r9, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,456)?)?;
    // VMA 0xc5e5c: sub r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R0,1)?)?;
    // VMA 0xc5e60: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5e64: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5e68: ldr r1, [pc, #0x458]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc5e6c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5e70: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5e74: ldrb r1, [r10, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1312)?)?;
    // VMA 0xc5e78: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5e7c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc5e80: bne 0xc5ee8 <entry+0x210>
    c.b(C::Ne,0xc5ee8,false)?;
    // VMA 0xc5e84: ldr r1, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,4)?)?;
    // VMA 0xc5e88: mov r7, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,52)?)?;
    // VMA 0xc5e8c: mov r2, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2096)?)?;
    // VMA 0xc5e90: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xc5e94: orr r7, r7, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2048)?)?;
    // VMA 0xc5e98: orr r2, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xc5e9c: orr r1, r7, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xc5ea0: ldr r2, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc5ea4: ldr r1, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc5ea8: adds r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,true)?)?;
    // VMA 0xc5eac: beq 0xc5ec4 <entry+0x1ec>
    c.b(C::Eq,0xc5ec4,false)?;
    // VMA 0xc5eb0: lsr r2, r1, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsr,3)?)?;
    // VMA 0xc5eb4: and r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,7)?)?;
    // VMA 0xc5eb8: str r2, [r9, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,440)?)?;
    // VMA 0xc5ebc: str r1, [r10, #0x574]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,1396)?)?;
    // VMA 0xc5ec0: b 0xc5ee8 <entry+0x210>
    c.b(C::Al,0xc5ee8,false)?;
    // VMA 0xc5ec4: ldr r1, [pc, #0x45c]
    c.literal(R::R1,0xc6328)?;
    // VMA 0xc5ec8: ldr r2, [pc, #0x45c]
    c.literal(R::R2,0xc632c)?;
    // VMA 0xc5ecc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc5ed0: mov r3, #2448
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2448)?)?;
    // VMA 0xc5ed4: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5ed8: ldr r1, [pc, #0x3e8]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc5edc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc5ee0: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc5ee4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc5ee8: ldr r1, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // VMA 0xc5eec: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xc5ef0: orr r2, r2, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1024)?)?;
    // VMA 0xc5ef4: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,8)?)?;
    // VMA 0xc5ef8: orr r2, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xc5efc: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc5f00: ldr r3, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc5f04: bic r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xc5f08: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // VMA 0xc5f0c: ldr r3, [pc, #0x41c]
    c.literal(R::new(3)?,0xc6330)?;
    // VMA 0xc5f10: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // VMA 0xc5f14: ldr r3, [r3, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
    // VMA 0xc5f18: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // VMA 0xc5f1c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc5f20: beq 0xc5f38 <entry+0x260>
    c.b(C::Eq,0xc5f38,false)?;
    // VMA 0xc5f24: orr r2, r2, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1040)?)?;
    // VMA 0xc5f28: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xc5f2c: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc5f30: b 0xc5f84 <entry+0x2ac>
    c.b(C::Al,0xc5f84,false)?;
    c.check(0xc5f34)?;
    const FRAME_RECORD_WARNING:u32=0x000f_b6b3;
    c.emit(FRAME_RECORD_WARNING)?;
    c.check(0xc5f38)?;
    // VMA 0xc5f38: orr r2, r2, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,124)?)?;
    // VMA 0xc5f3c: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // VMA 0xc5f40: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xc5f44: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xc5f48: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc5f4c: mov r3, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,44)?)?;
    // VMA 0xc5f50: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // VMA 0xc5f54: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xc5f58: orr r2, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc5f5c: str r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc5f60: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // VMA 0xc5f64: orr r2, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc5f68: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc5f6c: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc5f70: ldr r2, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,4)?)?;
    // VMA 0xc5f74: orr r2, r8, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(8)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc5f78: ldr r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc5f7c: tst r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,2)?)?;
    // VMA 0xc5f80: beq 0xc5f78 <entry+0x2a0>
    c.b(C::Eq,0xc5f78,false)?;
    // VMA 0xc5f84: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc5f88: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc5f8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5f90: beq 0xc5c60 <entry+0xffffffffffffff88>
    c.b(C::Eq,0xc5c60,false)?;
    // VMA 0xc5f94: ldrb r0, [r10, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1312)?)?;
    // VMA 0xc5f98: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc5f9c: bne 0xc5c60 <entry+0xffffffffffffff88>
    c.b(C::Ne,0xc5c60,false)?;
    // VMA 0xc5fa0: b 0xc5c0c <entry+0xffffffffffffff34>
    c.b(C::Al,0xc5c0c,false)?;
    c.check(0xc5fa4)?;
    const FRAME_RELEASE_WARNINGS:[u32;2]=[0x000f_b6cf,0x000f_b958];
    for addr in FRAME_RELEASE_WARNINGS {c.emit(addr)?;}
    c.check(0xc5fac)
}
