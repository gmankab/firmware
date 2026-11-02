
/// Validate the codec's shared format-plane input and pack encoder control
/// registers. The intra-function conditional ARM instruction at 0xae2f0 is
/// executable `ANDEQ`, not a literal; its high bits vary with flags.
fn program_live_codec_registers(c:&mut Code)->Result<(),String>{
    c.check(0xae108)?;
    // ARM 0xae108: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xae10c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xae110: sub sp, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // ARM 0xae114: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // ARM 0xae118: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // ARM 0xae11c: add r0, r8, #948
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,948)?)?;
    // ARM 0xae120: add r6, r8, #2880
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(8)?,2880)?)?;
    // ARM 0xae124: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // ARM 0xae128: ldr r0, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // ARM 0xae12c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae130: beq 0xae284 <entry+0x1b34>
    c.b(C::Eq,0xae284,false)?;
    // ARM 0xae134: mov r9, #696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,696)?)?;
    // ARM 0xae138: ldrsb r0, [r4, r9]
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(4)?,R::new(9)?)?)?;
    // ARM 0xae13c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xae140: blt 0xae4a4 <entry+0x1d54>
    c.b(C::Lt,0xae4a4,false)?;
    // ARM 0xae144: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM 0xae148: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // ARM 0xae14c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xae150: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xae154: bne 0xae174 <entry+0x1a24>
    c.b(C::Ne,0xae174,false)?;
    // ARM 0xae158: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // ARM 0xae15c: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM 0xae160: tst r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // ARM 0xae164: bne 0xae174 <entry+0x1a24>
    c.b(C::Ne,0xae174,false)?;
    // ARM 0xae168: tst r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,8)?)?;
    // ARM 0xae16c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xae170: movne r3, #2097152
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,2097152)?)?;
    // ARM 0xae174: mov r0, #185
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,185)?)?;
    // ARM 0xae178: add r5, r4, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(4)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // ARM 0xae17c: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // ARM 0xae180: ldrsb r7, [r5, r0]
    c.emit(arm32::load_signed_byte_register(C::Al,R::new(7)?,R::new(5)?,R::R0)?)?;
    // ARM 0xae184: ldrb r0, [r5, #0x2ca]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,714)?)?;
    // ARM 0xae188: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xae18c: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // ARM 0xae190: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xae194: str r0, [r8, #0x9d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,2516)?)?;
    // ARM 0xae198: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM 0xae19c: ldr r1, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,92)?)?;
    // ARM 0xae1a0: bl 0xae590 <entry+0x1e40>
    c.b(C::Al,0xae590,true)?;
    // ARM 0xae1a4: ldrb r0, [r5, #0x2ca]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,714)?)?;
    // ARM 0xae1a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae1ac: beq 0xae1f4 <entry+0x1aa4>
    c.b(C::Eq,0xae1f4,false)?;
    // ARM 0xae1b0: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // ARM 0xae1b4: orr r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4)?)?;
    // ARM 0xae1b8: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // ARM 0xae1bc: beq 0xae270 <entry+0x1b20>
    c.b(C::Eq,0xae270,false)?;
    // ARM 0xae1c0: ldr r0, [r8, #0xb44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2884)?)?;
    // ARM 0xae1c4: mov r1, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,360)?)?;
    // ARM 0xae1c8: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xae1cc: smlabb r0, r7, r1, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(7)?,R::R1,R::R0)?)?;
    // ARM 0xae1d0: ldr r0, [r0, #0x76c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1900)?)?;
    // ARM 0xae1d4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae1d8: bne 0xae270 <entry+0x1b20>
    c.b(C::Ne,0xae270,false)?;
    // ARM 0xae1dc: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xae1e0: mov r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // ARM 0xae1e4: smlabb r0, r7, r1, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(7)?,R::R1,R::R0)?)?;
    // ARM 0xae1e8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xae1ec: str r1, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,56)?)?;
    // ARM 0xae1f0: b 0xae270 <entry+0x1b20>
    c.b(C::Al,0xae270,false)?;
    // ARM 0xae1f4: mov r0, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,84)?)?;
    // ARM 0xae1f8: ldr r2, [pc, #0x38c]
    c.literal(R::R2,0xae58c)?;
    // ARM 0xae1fc: mov r1, #77
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,77)?)?;
    // ARM 0xae200: smulbb r9, r7, r0
    c.emit(arm32::signed_multiply_bottom_halves(C::Al,R::new(9)?,R::new(7)?,R::R0)?)?;
    // ARM 0xae204: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xae208: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // ARM 0xae20c: ldrb r0, [r0, r9]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // ARM 0xae210: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae214: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xae218: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xae21c: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // ARM 0xae220: orr r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4)?)?;
    // ARM 0xae224: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // ARM 0xae228: bne 0xae254 <entry+0x1b04>
    c.b(C::Ne,0xae254,false)?;
    // ARM 0xae22c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xae230: mov r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // ARM 0xae234: smlabb r0, r7, r1, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(7)?,R::R1,R::R0)?)?;
    // ARM 0xae238: ldr r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,20)?)?;
    // ARM 0xae23c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xae240: beq 0xae254 <entry+0x1b04>
    c.b(C::Eq,0xae254,false)?;
    // ARM 0xae244: add r2, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,40)?)?;
    // ARM 0xae248: ldr r1, [r8, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,984)?)?;
    // ARM 0xae24c: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // ARM 0xae250: bl 0xda564 <entry+0x2de14>
    c.b(C::Al,0xda564,true)?;
    // ARM 0xae254: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xae258: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xae25c: bl 0xa8fb4 <entry+0xffffffffffffc864>
    c.b(C::Al,0xa8fb4,true)?;
    // ARM 0xae260: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xae264: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xae268: strb r1, [r0, r9]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R1,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // ARM 0xae26c: mov r9, #696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,696)?)?;
    // ARM 0xae270: ldrsb r0, [r4, r9]
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(4)?,R::new(9)?)?)?;
    // ARM 0xae274: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // ARM 0xae278: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xae27c: blt 0xae148 <entry+0x19f8>
    c.b(C::Lt,0xae148,false)?;
    // ARM 0xae280: b 0xae4a4 <entry+0x1d54>
    c.b(C::Al,0xae4a4,false)?;
    // ARM 0xae284: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // ARM 0xae288: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // ARM 0xae28c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // ARM 0xae290: bls 0xae4a4 <entry+0x1d54>
    c.b(C::Ls,0xae4a4,false)?;
    // ARM 0xae294: ldr r2, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,8)?)?;
    // ARM 0xae298: ldr r5, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,92)?)?;
    // ARM 0xae29c: cmn r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R2,1)?)?;
    // ARM 0xae2a0: beq 0xae300 <entry+0x1bb0>
    c.b(C::Eq,0xae300,false)?;
    // ARM 0xae2a4: orr r1, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,4)?)?;
    // ARM 0xae2a8: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // ARM 0xae2ac: beq 0xae2e4 <entry+0x1b94>
    c.b(C::Eq,0xae2e4,false)?;
    // ARM 0xae2b0: ldr r1, [r8, #0xb44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,2884)?)?;
    // ARM 0xae2b4: mov r3, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,360)?)?;
    // ARM 0xae2b8: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // ARM 0xae2bc: mla r7, r2, r3, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::R2,R::new(3)?,R::R1)?)?;
    // ARM 0xae2c0: ldr r1, [r7, #0x76c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,1900)?)?;
    // ARM 0xae2c4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xae2c8: bne 0xae2e4 <entry+0x1b94>
    c.b(C::Ne,0xae2e4,false)?;
    // ARM 0xae2cc: mov r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    // ARM 0xae2d0: mov r7, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,84)?)?;
    // ARM 0xae2d4: str r12, [r8, #0x9d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(8)?,2516)?)?;
    // ARM 0xae2d8: ldr r3, [r8, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,2880)?)?;
    // ARM 0xae2dc: mla r1, r2, r7, r3
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::R2,R::new(7)?,R::new(3)?)?)?;
    // ARM 0xae2e0: str r12, [r1, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,56)?)?;
    // ARM 0xae2e4: mov r1, #2097152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2097152)?)?;
    // ARM 0xae2e8: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM 0xae2ec: tst r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // ARM 0xae2f0: andeq r3, r1, r0, lsl #18
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(3)?,R::R1,R::R0,Shift::Lsl,18,false)?)?;
    // ARM 0xae2f4: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM 0xae2f8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xae2fc: bl 0xae590 <entry+0x1e40>
    c.b(C::Al,0xae590,true)?;
    // ARM 0xae300: mov r7, #696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,696)?)?;
    // ARM 0xae304: ldrsb r0, [r4, r7]
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(4)?,R::new(7)?)?)?;
    // ARM 0xae308: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xae30c: blt 0xae4a4 <entry+0x1d54>
    c.b(C::Lt,0xae4a4,false)?;
    // ARM 0xae310: mov r1, #185
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,185)?)?;
    // ARM 0xae314: add r0, r8, #728
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,728)?)?;
    // ARM 0xae318: orr r10, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(5)?,4)?)?;
    // ARM 0xae31c: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // ARM 0xae320: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // ARM 0xae324: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM 0xae328: add r0, r4, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xae32c: ldrsb r5, [r0, r1]
    c.emit(arm32::load_signed_byte_register(C::Al,R::new(5)?,R::R0,R::R1)?)?;
    // ARM 0xae330: ldrb r0, [r0, #0x2ca]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,714)?)?;
    // ARM 0xae334: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae338: beq 0xae38c <entry+0x1c3c>
    c.b(C::Eq,0xae38c,false)?;
    // ARM 0xae33c: cmp r10, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,6)?)?;
    // ARM 0xae340: beq 0xae494 <entry+0x1d44>
    c.b(C::Eq,0xae494,false)?;
    // ARM 0xae344: ldr r0, [r8, #0xb44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2884)?)?;
    // ARM 0xae348: mov r2, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,360)?)?;
    // ARM 0xae34c: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // ARM 0xae350: smlabb r0, r5, r2, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(5)?,R::R2,R::R0)?)?;
    // ARM 0xae354: ldr r0, [r0, #0x76c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1900)?)?;
    // ARM 0xae358: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae35c: bne 0xae494 <entry+0x1d44>
    c.b(C::Ne,0xae494,false)?;
    // ARM 0xae360: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xae364: mov r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // ARM 0xae368: ldr r2, [pc, #0x21c]
    c.literal(R::R2,0xae58c)?;
    // ARM 0xae36c: smlabb r0, r5, r1, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(5)?,R::R1,R::R0)?)?;
    // ARM 0xae370: mov r1, #920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,920)?)?;
    // ARM 0xae374: ldr r0, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,56)?)?;
    // ARM 0xae378: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // ARM 0xae37c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xae380: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xae384: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xae388: b 0xae48c <entry+0x1d3c>
    c.b(C::Al,0xae48c,false)?;
    // ARM 0xae38c: mov r0, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,84)?)?;
    // ARM 0xae390: ldr r2, [pc, #0x1f4]
    c.literal(R::R2,0xae58c)?;
    // ARM 0xae394: mov r1, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,127)?)?;
    // ARM 0xae398: smulbb r7, r5, r0
    c.emit(arm32::signed_multiply_bottom_halves(C::Al,R::new(7)?,R::new(5)?,R::R0)?)?;
    // ARM 0xae39c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xae3a0: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // ARM 0xae3a4: ldrb r0, [r0, r7]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::R0,R::new(7)?,Shift::Lsl,0)?)?;
    // ARM 0xae3a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae3ac: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xae3b0: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xae3b4: cmp r10, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,6)?)?;
    // ARM 0xae3b8: beq 0xae3d8 <entry+0x1c88>
    c.b(C::Eq,0xae3d8,false)?;
    // ARM 0xae3bc: ldr r0, [r8, #0xb44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2884)?)?;
    // ARM 0xae3c0: mov r1, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,360)?)?;
    // ARM 0xae3c4: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xae3c8: smlabb r0, r5, r1, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(5)?,R::R1,R::R0)?)?;
    // ARM 0xae3cc: ldr r0, [r0, #0x76c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1900)?)?;
    // ARM 0xae3d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae3d4: beq 0xae414 <entry+0x1cc4>
    c.b(C::Eq,0xae414,false)?;
    // ARM 0xae3d8: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // ARM 0xae3dc: orr r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4)?)?;
    // ARM 0xae3e0: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // ARM 0xae3e4: bne 0xae470 <entry+0x1d20>
    c.b(C::Ne,0xae470,false)?;
    // ARM 0xae3e8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xae3ec: mov r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // ARM 0xae3f0: smlabb r0, r5, r1, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(5)?,R::R1,R::R0)?)?;
    // ARM 0xae3f4: ldr r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,20)?)?;
    // ARM 0xae3f8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xae3fc: beq 0xae470 <entry+0x1d20>
    c.b(C::Eq,0xae470,false)?;
    // ARM 0xae400: add r2, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,40)?)?;
    // ARM 0xae404: ldr r1, [r8, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,984)?)?;
    // ARM 0xae408: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // ARM 0xae40c: bl 0xda564 <entry+0x2de14>
    c.b(C::Al,0xda564,true)?;
    // ARM 0xae410: b 0xae470 <entry+0x1d20>
    c.b(C::Al,0xae470,false)?;
    // ARM 0xae414: ldr r0, [r8, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2880)?)?;
    // ARM 0xae418: mov r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // ARM 0xae41c: ldr r2, [pc, #0x168]
    c.literal(R::R2,0xae58c)?;
    // ARM 0xae420: smlabb r0, r5, r1, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(5)?,R::R1,R::R0)?)?;
    // ARM 0xae424: mov r1, #131
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,131)?)?;
    // ARM 0xae428: ldr r0, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,56)?)?;
    // ARM 0xae42c: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // ARM 0xae430: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xae434: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xae438: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xae43c: ldr r0, [r8, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2880)?)?;
    // ARM 0xae440: mov r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // ARM 0xae444: ldr r3, [r8, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,956)?)?;
    // ARM 0xae448: smlabb r0, r5, r1, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(5)?,R::R1,R::R0)?)?;
    // ARM 0xae44c: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // ARM 0xae450: add r2, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,40)?)?;
    // ARM 0xae454: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM 0xae458: bl 0xe0d28 <entry+0x345d8>
    c.b(C::Al,0xe0d28,true)?;
    // ARM 0xae45c: ldr r0, [r8, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2880)?)?;
    // ARM 0xae460: mov r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // ARM 0xae464: smlabb r0, r5, r1, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(5)?,R::R1,R::R0)?)?;
    // ARM 0xae468: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xae46c: str r1, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,56)?)?;
    // ARM 0xae470: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xae474: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xae478: bl 0xa8fb4 <entry+0xffffffffffffc864>
    c.b(C::Al,0xa8fb4,true)?;
    // ARM 0xae47c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xae480: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xae484: strb r1, [r0, r7]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R1,R::R0,R::new(7)?,Shift::Lsl,0)?)?;
    // ARM 0xae488: mov r7, #696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,696)?)?;
    // ARM 0xae48c: mov r1, #185
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,185)?)?;
    // ARM 0xae490: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // ARM 0xae494: ldrsb r0, [r4, r7]
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(4)?,R::new(7)?)?)?;
    // ARM 0xae498: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // ARM 0xae49c: cmp r9, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xae4a0: blt 0xae328 <entry+0x1bd8>
    c.b(C::Lt,0xae328,false)?;
    // ARM 0xae4a4: mov r0, #219
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,219)?)?;
    // ARM 0xae4a8: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // ARM 0xae4ac: ldrsb r0, [r4, r0]
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(4)?,R::R0)?)?;
    // ARM 0xae4b0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xae4b4: blt 0xae584 <entry+0x1e34>
    c.b(C::Lt,0xae584,false)?;
    // ARM 0xae4b8: add r0, r8, #728
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,728)?)?;
    // ARM 0xae4bc: add r5, r4, #732
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(4)?,732)?)?;
    // ARM 0xae4c0: mov r9, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,84)?)?;
    // ARM 0xae4c4: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM 0xae4c8: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM 0xae4cc: add r0, r5, r10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // ARM 0xae4d0: ldrsb r7, [r0]
    c.emit(arm32::load_signed_byte(C::Al,R::new(7)?,R::R0,0)?)?;
    // ARM 0xae4d4: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // ARM 0xae4d8: orr r1, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,4)?)?;
    // ARM 0xae4dc: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // ARM 0xae4e0: bne 0xae50c <entry+0x1dbc>
    c.b(C::Ne,0xae50c,false)?;
    // ARM 0xae4e4: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xae4e8: smlabb r0, r7, r9, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(7)?,R::new(9)?,R::R0)?)?;
    // ARM 0xae4ec: ldr r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,20)?)?;
    // ARM 0xae4f0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xae4f4: beq 0xae550 <entry+0x1e00>
    c.b(C::Eq,0xae550,false)?;
    // ARM 0xae4f8: add r2, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,40)?)?;
    // ARM 0xae4fc: ldr r1, [r8, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,984)?)?;
    // ARM 0xae500: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // ARM 0xae504: bl 0xda564 <entry+0x2de14>
    c.b(C::Al,0xda564,true)?;
    // ARM 0xae508: b 0xae550 <entry+0x1e00>
    c.b(C::Al,0xae550,false)?;
    // ARM 0xae50c: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // ARM 0xae510: smlabb r1, r7, r9, r1
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R1,R::new(7)?,R::new(9)?,R::R1)?)?;
    // ARM 0xae514: ldr r2, [r1, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,56)?)?;
    // ARM 0xae518: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // ARM 0xae51c: bne 0xae548 <entry+0x1df8>
    c.b(C::Ne,0xae548,false)?;
    // ARM 0xae520: add r2, r1, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,40)?)?;
    // ARM 0xae524: ldr r3, [r8, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,956)?)?;
    // ARM 0xae528: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // ARM 0xae52c: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM 0xae530: bl 0xe0d28 <entry+0x345d8>
    c.b(C::Al,0xe0d28,true)?;
    // ARM 0xae534: ldr r0, [r8, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2880)?)?;
    // ARM 0xae538: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xae53c: smlabb r0, r7, r9, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::new(7)?,R::new(9)?,R::R0)?)?;
    // ARM 0xae540: str r1, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,56)?)?;
    // ARM 0xae544: b 0xae550 <entry+0x1e00>
    c.b(C::Al,0xae550,false)?;
    // ARM 0xae548: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM 0xae54c: beq 0xae4e4 <entry+0x1d94>
    c.b(C::Eq,0xae4e4,false)?;
    // ARM 0xae550: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xae554: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xae558: bl 0xa8fb4 <entry+0xffffffffffffc864>
    c.b(C::Al,0xa8fb4,true)?;
    // ARM 0xae55c: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // ARM 0xae560: smulbb r0, r7, r9
    c.emit(arm32::signed_multiply_bottom_halves(C::Al,R::R0,R::new(7)?,R::new(9)?)?)?;
    // ARM 0xae564: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xae568: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // ARM 0xae56c: strb r2, [r1, r0]
    c.emit(arm32::transfer_byte_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xae570: mov r0, #219
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,219)?)?;
    // ARM 0xae574: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // ARM 0xae578: ldrsb r0, [r4, r0]
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(4)?,R::R0)?)?;
    // ARM 0xae57c: cmp r10, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xae580: blt 0xae4cc <entry+0x1d7c>
    c.b(C::Lt,0xae4cc,false)?;
    // ARM 0xae584: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xae588: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xae58c)?;
    c.emit(0x000f_8519)?; // Diagnostic message pointer, not executable.
    c.check(0xae590)
}

/// Create the next codec plane command pair from the metadata buffers.
fn submit_codec_plane_control_pair(c:&mut Code)->Result<(),String>{
    c.check(0xae590)?;
    // ARM 0xae590: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xae594: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM 0xae598: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // ARM 0xae59c: orr r7, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R1,4)?)?;
    // ARM 0xae5a0: mov r6, r3
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // ARM 0xae5a4: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // ARM 0xae5a8: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xae5ac: cmp r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,6)?)?;
    // ARM 0xae5b0: bne 0xae600 <entry+0x1eb0>
    c.b(C::Ne,0xae600,false)?;
    // ARM 0xae5b4: ldr r1, [r5, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,956)?)?;
    // ARM 0xae5b8: ldr r0, [r5, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2880)?)?;
    // ARM 0xae5bc: ldr r2, [r5, #0xb44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2884)?)?;
    // ARM 0xae5c0: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM 0xae5c4: mov r7, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,84)?)?;
    // ARM 0xae5c8: stm sp, {r1, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xae5cc: mov r3, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,360)?)?;
    // ARM 0xae5d0: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // ARM 0xae5d4: mla r1, r4, r3, r2
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::new(4)?,R::new(3)?,R::R2)?)?;
    // ARM 0xae5d8: mla r2, r4, r7, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::new(4)?,R::new(7)?,R::R0)?)?;
    // ARM 0xae5dc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xae5e0: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // ARM 0xae5e4: add r2, r2, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,60)?)?;
    // ARM 0xae5e8: bl 0xabbf8 <entry+0xfffffffffffff4a8>
    c.b(C::Al,0xabbf8,true)?;
    // ARM 0xae5ec: ldr r0, [r5, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2880)?)?;
    // ARM 0xae5f0: mla r1, r4, r7, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::new(4)?,R::new(7)?,R::R0)?)?;
    // ARM 0xae5f4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xae5f8: strb r0, [r1, #0x50]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,80)?)?;
    // ARM 0xae5fc: b 0xae640 <entry+0x1ef0>
    c.b(C::Al,0xae640,false)?;
    // ARM 0xae600: ldr r1, [r5, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,956)?)?;
    // ARM 0xae604: ldr r2, [r5, #0xb44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2884)?)?;
    // ARM 0xae608: ldr r0, [r5, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2880)?)?;
    // ARM 0xae60c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xae610: stm sp, {r1, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xae614: mov r3, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,360)?)?;
    // ARM 0xae618: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // ARM 0xae61c: mla r1, r4, r3, r2
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::new(4)?,R::new(3)?,R::R2)?)?;
    // ARM 0xae620: mov r2, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // ARM 0xae624: mla r3, r4, r2, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::new(4)?,R::R2,R::R0)?)?;
    // ARM 0xae628: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xae62c: add r2, r3, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,40)?)?;
    // ARM 0xae630: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // ARM 0xae634: bl 0xabbf8 <entry+0xfffffffffffff4a8>
    c.b(C::Al,0xabbf8,true)?;
    // ARM 0xae638: cmp r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,5)?)?;
    // ARM 0xae63c: bne 0xae5b4 <entry+0x1e64>
    c.b(C::Ne,0xae5b4,false)?;
    // ARM 0xae640: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // ARM 0xae644: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xae648)
}
