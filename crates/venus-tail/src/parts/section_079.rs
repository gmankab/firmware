
/// Apply source/destination plane mode parameters and report the
/// original per-plane mismatch diagnostics, preserving the original return.
fn set_codec_plane_mode_parameters(c:&mut Code)->Result<(),String>{
    c.check(0xc1d68)?;
    // VMA 0xc1d68: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc1d6c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc1d70: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // VMA 0xc1d74: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xc1d78: mov r0, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1020)?)?;
    // VMA 0xc1d7c: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xc1d80: ldr r1, [pc, #0x1d8]        @ 0x33f0 <entry+0x33f0>
    c.literal(R::R1,0xc1f60)?;
    // VMA 0xc1d84: orr r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // VMA 0xc1d88: ldr r2, [r4, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,392)?)?;
    // VMA 0xc1d8c: ldrb r5, [r4, #0x7fa]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,2042)?)?;
    // VMA 0xc1d90: ldrb r7, [r4, #0x7f9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,2041)?)?;
    // VMA 0xc1d94: ldrb r10, [r4, #0x7f8]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(10)?,R::new(4)?,2040)?)?;
    // VMA 0xc1d98: ldrsb r9, [r4, r0]
    c.emit(arm32::load_signed_byte_register(C::Al,R::new(9)?,R::new(4)?,R::R0)?)?;
    // VMA 0xc1d9c: mov r0, #251
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,251)?)?;
    // VMA 0xc1da0: orr r0, r0, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1792)?)?;
    // VMA 0xc1da4: ldrsb r8, [r4, r0]
    c.emit(arm32::load_signed_byte_register(C::Al,R::new(8)?,R::new(4)?,R::R0)?)?;
    // VMA 0xc1da8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1dac: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x93120
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1db0: ldr r6, [pc, #0x1ac]        @ 0x33f4 <entry+0x33f4>
    c.literal(R::new(6)?,0xc1f64)?;
    // VMA 0xc1db4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1db8: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc1dbc: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x93130
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1dc0: ldr r1, [pc, #0x1a0]        @ 0x33f8 <entry+0x33f8>
    c.literal(R::R1,0xc1f68)?;
    // VMA 0xc1dc4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1dc8: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // VMA 0xc1dcc: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x93140
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1dd0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1dd4: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc1dd8: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9314c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1ddc: ldr r1, [pc, #0x188]        @ 0x33fc <entry+0x33fc>
    c.literal(R::R1,0xc1f6c)?;
    // VMA 0xc1de0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1de4: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xc1de8: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // VMA 0xc1dec: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x93160
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1df0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1df4: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc1df8: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9316c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1dfc: ldr r1, [pc, #0x16c]        @ 0x3400 <entry+0x3400>
    c.literal(R::R1,0xc1f70)?;
    // VMA 0xc1e00: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1e04: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xc1e08: str r5, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    // VMA 0xc1e0c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x93180
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1e10: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1e14: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc1e18: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9318c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1e1c: ldr r1, [pc, #0x150]        @ 0x3404 <entry+0x3404>
    c.literal(R::R1,0xc1f74)?;
    // VMA 0xc1e20: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc1e24: and r8, r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(8)?,R::new(8)?,255)?)?;
    // VMA 0xc1e28: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xc1e2c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1e30: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xc1e34: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x931a8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1e38: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1e3c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc1e40: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x931b4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1e44: ldr r1, [pc, #0x12c]        @ 0x3408 <entry+0x3408>
    c.literal(R::R1,0xc1f78)?;
    // VMA 0xc1e48: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xc1e4c: and r5, r9, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(9)?,255)?)?;
    // VMA 0xc1e50: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xc1e54: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1e58: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xc1e5c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x931d0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1e60: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1e64: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc1e68: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x931dc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1e6c: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // VMA 0xc1e70: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xc1e74: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xc1e78: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xc1e7c: bne 0xc1f54 <entry+0x33e4>  @ imm = #0xd0
    c.b(C::Ne,0xc1f54,false)?;
    // VMA 0xc1e80: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // VMA 0xc1e84: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc1e88: movne r10, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(10)?,R::R0,3)?)?;
    // VMA 0xc1e8c: str r10, [r4, #0x800]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,2048)?)?;
    // VMA 0xc1e90: str r0, [r4, #0x804]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2052)?)?;
    // VMA 0xc1e94: str r0, [r4, #0x808]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2056)?)?;
    // VMA 0xc1e98: ldr r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1296)?)?;
    // VMA 0xc1e9c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1ea0: beq 0xc1f54 <entry+0x33e4>  @ imm = #0xac
    c.b(C::Eq,0xc1f54,false)?;
    // VMA 0xc1ea4: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // VMA 0xc1ea8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc1eac: movne r0, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc1eb0: strne r0, [r4, #0x804]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,2052)?)?;
    // VMA 0xc1eb4: ldr r6, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,12)?)?;
    // VMA 0xc1eb8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc1ebc: movne r0, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc1ec0: strne r0, [r4, #0x808]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,2056)?)?;
    // VMA 0xc1ec4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc1ec8: bne 0xc1f0c <entry+0x339c>  @ imm = #0x3c
    c.b(C::Ne,0xc1f0c,false)?;
    // VMA 0xc1ecc: cmp r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,3)?)?;
    // VMA 0xc1ed0: bhs 0xc1ee4 <entry+0x3374>  @ imm = #0xc
    c.b(C::Cs,0xc1ee4,false)?;
    // VMA 0xc1ed4: ldr r0, [pc, #0xa8]         @ 0x3414 <entry+0x3414>
    c.literal(R::R0,0xc1f84)?;
    // VMA 0xc1ed8: ldr r1, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,0)?)?;
    // VMA 0xc1edc: ldr r0, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // VMA 0xc1ee0: b 0xc1f08 <entry+0x3398>  @ imm = #0x20
    c.b(C::Al,0xc1f08,false)?;
    // VMA 0xc1ee4: ldr r1, [pc, #0x90]         @ 0x340c <entry+0x340c>
    c.literal(R::R1,0xc1f7c)?;
    // VMA 0xc1ee8: ldr r2, [pc, #0x90]         @ 0x3410 <entry+0x3410>
    c.literal(R::R2,0xc1f80)?;
    // VMA 0xc1eec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1ef0: mov r3, #245
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,245)?)?;
    // VMA 0xc1ef4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x93268
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1ef8: ldr r1, [pc, #0x64]         @ 0x33f4 <entry+0x33f4>
    c.literal(R::R1,0xc1f64)?;
    // VMA 0xc1efc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1f00: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x93274
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1f04: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1f08: str r0, [r4, #0x804]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2052)?)?;
    // VMA 0xc1f0c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc1f10: bne 0xc1f54 <entry+0x33e4>  @ imm = #0x3c
    c.b(C::Ne,0xc1f54,false)?;
    // VMA 0xc1f14: cmp r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,3)?)?;
    // VMA 0xc1f18: bhs 0xc1f2c <entry+0x33bc>  @ imm = #0xc
    c.b(C::Cs,0xc1f2c,false)?;
    // VMA 0xc1f1c: ldr r0, [pc, #0x68]         @ 0x341c <entry+0x341c>
    c.literal(R::R0,0xc1f8c)?;
    // VMA 0xc1f20: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // VMA 0xc1f24: ldr r0, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // VMA 0xc1f28: b 0xc1f50 <entry+0x33e0>  @ imm = #0x20
    c.b(C::Al,0xc1f50,false)?;
    // VMA 0xc1f2c: ldr r1, [pc, #0x54]         @ 0x3418 <entry+0x3418>
    c.literal(R::R1,0xc1f88)?;
    // VMA 0xc1f30: ldr r2, [pc, #0x48]         @ 0x3410 <entry+0x3410>
    c.literal(R::R2,0xc1f80)?;
    // VMA 0xc1f34: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1f38: mov r3, #272
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,272)?)?;
    // VMA 0xc1f3c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x932b0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1f40: ldr r1, [pc, #0x1c]         @ 0x33f4 <entry+0x33f4>
    c.literal(R::R1,0xc1f64)?;
    // VMA 0xc1f44: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1f48: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x932bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1f4c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc1f50: str r0, [r4, #0x808]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2056)?)?;
    // VMA 0xc1f54: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1f58: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc1f5c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc1f60)?;
    const CODEC_PLANE_MODE_GLOBALS:[u32;12]=[
       0x000fb05d,0x000e2788,0x000fb071,0x000fb089,
       0x000fb0a3,0x000fb0be,0x000fb0d5,0x000fb0ed,
       0x000fb131,0x000fb2b8,0x000fb14c,0x000fb2c4,
    ];
    for word in CODEC_PLANE_MODE_GLOBALS {c.emit(word)?;}
    c.check(0xc1f90)
}
