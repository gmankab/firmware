
/// Stage validated codec packet fields for later controller MMIO submission.
/// This remains part of the same long codec-buffer function.
fn stage_controller_codec_packet_sequence(c:&mut Code)->Result<(),String>{
    c.check(0xbda80)?;
    // VMA 0xbda80: ldrb r1, [r5, #0x14]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,20)?)?;
    // VMA 0xbda84: add r1, r2, r1, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,7,false)?)?;
    // VMA 0xbda88: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbda8c: ldr r1, [r5, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,1008)?)?;
    // VMA 0xbda90: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbda94: str r0, [r5, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1008)?)?;
    // VMA 0xbda98: b 0xbde10 <entry+0x1634>  @ imm = #0x370
    c.b(C::Al,0xbde10,false)?;
    // VMA 0xbda9c: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbdaa0: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xbdaa4: bhs 0xbe278 <entry+0x1a9c>  @ imm = #0x7cc
    c.b(C::Cs,0xbe278,false)?;
    // VMA 0xbdaa8: ldr r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbdaac: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xbdab0: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbdab4: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // VMA 0xbdab8: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xbdabc: str r8, [r9, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,164)?)?;
    // VMA 0xbdac0: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbdac4: bl 0xe1e60 <entry+0x25684> @ imm = #0x24394
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdac8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdacc: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbdad0: bne 0xbea00 <entry+0x2224>  @ imm = #0xf28
    c.b(C::Ne,0xbea00,false)?;
    // VMA 0xbdad4: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xbdad8: mov r0, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1920)?)?;
    // VMA 0xbdadc: cmp r1, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // VMA 0xbdae0: str r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,8)?)?;
    // VMA 0xbdae4: bhi 0xbdaf4 <entry+0x1318>  @ imm = #0x8
    c.b(C::Hi,0xbdaf4,false)?;
    // VMA 0xbdae8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbdaec: cmp r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // VMA 0xbdaf0: bhi 0xbdb00 <entry+0x1324>  @ imm = #0x8
    c.b(C::Hi,0xbdb00,false)?;
    // VMA 0xbdaf4: str r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,8)?)?;
    // VMA 0xbdaf8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbdafc: str r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbdb00: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbdb04: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbdb08: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // VMA 0xbdb0c: bl 0xe1e60 <entry+0x25684> @ imm = #0x2434c
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdb10: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdb14: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbdb18: bne 0xbea00 <entry+0x2224>  @ imm = #0xee0
    c.b(C::Ne,0xbea00,false)?;
    // VMA 0xbdb1c: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xbdb20: mov r0, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1920)?)?;
    // VMA 0xbdb24: cmp r1, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // VMA 0xbdb28: str r1, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,12)?)?;
    // VMA 0xbdb2c: bhi 0xbdb3c <entry+0x1360>  @ imm = #0x8
    c.b(C::Hi,0xbdb3c,false)?;
    // VMA 0xbdb30: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbdb34: cmp r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // VMA 0xbdb38: bhi 0xbdb48 <entry+0x136c>  @ imm = #0x8
    c.b(C::Hi,0xbdb48,false)?;
    // VMA 0xbdb3c: str r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,12)?)?;
    // VMA 0xbdb40: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbdb44: str r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbdb48: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbdb4c: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbdb50: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbdb54: bl 0xe1e60 <entry+0x25684> @ imm = #0x24304
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdb58: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdb5c: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbdb60: bne 0xbea00 <entry+0x2224>  @ imm = #0xe98
    c.b(C::Ne,0xbea00,false)?;
    // VMA 0xbdb64: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbdb68: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // VMA 0xbdb6c: str r0, [r9, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,184)?)?;
    // VMA 0xbdb70: cmp r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,14)?)?;
    // VMA 0xbdb74: blo 0xbdba4 <entry+0x13c8>  @ imm = #0x28
    c.b(C::Cc,0xbdba4,false)?;
    // VMA 0xbdb78: ldr r1, [pc, #0xff0]        @ 0x2394 <entry+0x2394>
    c.literal(R::R1,0xbeb70)?;
    // VMA 0xbdb7c: ldr r2, [pc, #0xff0]        @ 0x2398 <entry+0x2398>
    c.literal(R::R2,0xbeb74)?;
    // VMA 0xbdb80: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbdb84: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbdb88: mov r3, #155
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,155)?)?;
    // VMA 0xbdb8c: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8ef00
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbdb90: ldr r1, [pc, #0xb50]        @ 0x1f0c <entry+0x1f0c>
    c.literal(R::R1,0xbe6e8)?;
    // VMA 0xbdb94: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbdb98: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8ef0c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbdb9c: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xbdba0: str r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbdba4: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbdba8: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbdbac: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbdbb0: bl 0xe1e60 <entry+0x25684> @ imm = #0x242a8
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdbb4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdbb8: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbdbbc: bne 0xbea00 <entry+0x2224>  @ imm = #0xe3c
    c.b(C::Ne,0xbea00,false)?;
    // VMA 0xbdbc0: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbdbc4: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbdbc8: mov r1, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,18)?)?;
    // VMA 0xbdbcc: bl 0xe1e60 <entry+0x25684> @ imm = #0x2428c
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdbd0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdbd4: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbdbd8: bne 0xbea00 <entry+0x2224>  @ imm = #0xe20
    c.b(C::Ne,0xbea00,false)?;
    // VMA 0xbdbdc: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbdbe0: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbdbe4: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // VMA 0xbdbe8: bl 0xe1e60 <entry+0x25684> @ imm = #0x24270
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdbec: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdbf0: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbdbf4: bne 0xbea00 <entry+0x2224>  @ imm = #0xe04
    c.b(C::Ne,0xbea00,false)?;
    // VMA 0xbdbf8: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbdbfc: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    c.check(0xbdc00)
}

/// Resolve the selected codec mode's control-buffer metadata and enter the
/// eight-way hardware-format dispatch. The following eight pointer words
/// are a branch table, not executable ARM32 instructions.
fn classify_frame_controller_mode(c:&mut Code)->Result<(),String>{
    c.check(0xbdc00)?;
    // VMA 0xbdc00: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbdc04: strb r7, [r9, #0xac]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(9)?,172)?)?;
    // VMA 0xbdc08: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbdc0c: bl 0xe1e60 <entry+0x25684> @ imm = #0x2424c
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdc10: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdc14: bne 0xbea00 <entry+0x2224>  @ imm = #0xde4
    c.b(C::Ne,0xbea00,false)?;
    // VMA 0xbdc18: ldrb r0, [r11, #-0x20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbdc1c: add r4, r9, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(9)?,24)?)?;
    // VMA 0xbdc20: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbdc24: strb r0, [r9, #0x98]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,152)?)?;
    // VMA 0xbdc28: beq 0xbe918 <entry+0x213c>  @ imm = #0xce8
    c.b(C::Eq,0xbe918,false)?;
    // VMA 0xbdc2c: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbdc30: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbdc34: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbdc38: bl 0xe1e60 <entry+0x25684> @ imm = #0x24220
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdc3c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdc40: bne 0xbea00 <entry+0x2224>  @ imm = #0xdb8
    c.b(C::Ne,0xbea00,false)?;
    // VMA 0xbdc44: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // VMA 0xbdc48: b 0xbe68c <entry+0x1eb0>  @ imm = #0xa3c
    c.b(C::Al,0xbe68c,false)?;
    // VMA 0xbdc4c: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbdc50: add r2, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,32)?)?;
    // VMA 0xbdc54: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbdc58: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbdc5c: bl 0xe1e60 <entry+0x25684> @ imm = #0x241fc
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdc60: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdc64: bne 0xbd744 <entry+0xf68>   @ imm = #-0x528
    c.b(C::Ne,0xbd744,false)?;
    // VMA 0xbdc68: ldr r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // VMA 0xbdc6c: sub r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R2,1)?)?;
    // VMA 0xbdc70: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // VMA 0xbdc74: bhi 0xbe13c <entry+0x1960>  @ imm = #0x4c0
    c.b(C::Hi,0xbe13c,false)?;
    // VMA 0xbdc78: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbdc7c: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // VMA 0xbdc80: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0xbdc84)
}

const CONTROLLER_FRAME_MODE_BRANCHES:[u32;8]=[
    0x000b_dca4,0x000b_de6c,0x000b_e0b0,0x000b_e13c,
    0x000b_e154,0x000b_e13c,0x000b_e1a8,0x000b_e2b8,
];
