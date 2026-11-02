
/// Emit buffer-format-specific size, alignment, and controller command
/// metadata for the dispatch cases, retaining all native branch targets.
/// This is still part of the long function begun at runtime 0xbd648.
fn emit_frame_mode_controller_configuration(c:&mut Code)->Result<(),String>{
    c.check(0xbdca4)?;
    // VMA 0xbdca4: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbdca8: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // VMA 0xbdcac: beq 0xbe288 <entry+0x1aac>  @ imm = #0x5d4
    c.b(C::Eq,0xbe288,false)?;
    // VMA 0xbdcb0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbdcb4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdcb8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbdcbc: bne 0xbde14 <entry+0x1638>  @ imm = #0x150
    c.b(C::Ne,0xbde14,false)?;
    // VMA 0xbdcc0: ldr r1, [pc, #0xf84]        @ 0x2470 <entry+0x2470>
    c.literal(R::R1,0xbec4c)?;
    // VMA 0xbdcc4: ldr r8, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(9)?,188)?)?;
    // VMA 0xbdcc8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbdccc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbdcd0: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xbdcd4: str r7, [r9, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(9)?,164)?)?;
    // VMA 0xbdcd8: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f04c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbdcdc: ldr r1, [pc, #0xf54]        @ 0x245c <entry+0x245c>
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbdce0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbdce4: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f058
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbdce8: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbdcec: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbdcf0: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbdcf4: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbdcf8: bl 0xe1e60 <entry+0x25684> @ imm = #0x24160
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdcfc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdd00: bne 0xbe8c0 <entry+0x20e4>  @ imm = #0xbb8
    c.b(C::Ne,0xbe8c0,false)?;
    // VMA 0xbdd04: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbdd08: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // VMA 0xbdd0c: and r1, r1, r0, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsr,4,false)?)?;
    // VMA 0xbdd10: cmp r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // VMA 0xbdd14: bne 0xbe5bc <entry+0x1de0>  @ imm = #0x8a0
    c.b(C::Ne,0xbe5bc,false)?;
    // VMA 0xbdd18: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbdd1c: str r1, [r9, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,156)?)?;
    // VMA 0xbdd20: b 0xbe60c <entry+0x1e30>  @ imm = #0x8e4
    c.b(C::Al,0xbe60c,false)?;
    // VMA 0xbdd24: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbdd28: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbdd2c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbdd30: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // VMA 0xbdd34: bne 0xbde14 <entry+0x1638>  @ imm = #0xd8
    c.b(C::Ne,0xbde14,false)?;
    // VMA 0xbdd38: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xbdd3c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbdd40: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbdd44: str r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbdd48: ldr r0, [r5, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,484)?)?;
    // VMA 0xbdd4c: orr r0, r0, #2097152
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2097152)?)?;
    // VMA 0xbdd50: str r0, [r5, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,484)?)?;
    // VMA 0xbdd54: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbdd58: str r0, [r5, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,556)?)?;
    // VMA 0xbdd5c: ldr r0, [r9, #0xe0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,224)?)?;
    // VMA 0xbdd60: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdd64: bne 0xbde10 <entry+0x1634>  @ imm = #0xa4
    c.b(C::Ne,0xbde10,false)?;
    // VMA 0xbdd68: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbdd6c: str r4, [r9, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(9)?,224)?)?;
    // VMA 0xbdd70: b 0xbde0c <entry+0x1630>  @ imm = #0x94
    c.b(C::Al,0xbde0c,false)?;
    // VMA 0xbdd74: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbdd78: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xbdd7c: bhs 0xbe278 <entry+0x1a9c>  @ imm = #0x4f4
    c.b(C::Cs,0xbe278,false)?;
    // VMA 0xbdd80: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbdd84: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbdd88: mov r1, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,25)?)?;
    // VMA 0xbdd8c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbdd90: bl 0xe1e60 <entry+0x25684> @ imm = #0x240c8
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdd94: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdd98: bne 0xbde28 <entry+0x164c>  @ imm = #0x88
    c.b(C::Ne,0xbde28,false)?;
    // VMA 0xbdd9c: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbdda0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbdda4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbdda8: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xbddac: bl 0xe1e60 <entry+0x25684> @ imm = #0x240ac
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbddb0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbddb4: bne 0xbde28 <entry+0x164c>  @ imm = #0x6c
    c.b(C::Ne,0xbde28,false)?;
    // VMA 0xbddb8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbddbc: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbddc0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbddc4: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xbddc8: strb r0, [r5, #0x2cc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,716)?)?;
    // VMA 0xbddcc: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbddd0: bl 0xe1e60 <entry+0x25684> @ imm = #0x24088
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbddd4: ldrb r1, [r5, #0x2cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,716)?)?;
    // VMA 0xbddd8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbdddc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdde0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbdde4: strb r1, [r9, #0x86c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(9)?,2156)?)?;
    // VMA 0xbdde8: bne 0xbd744 <entry+0xf68>   @ imm = #-0x6ac
    c.b(C::Ne,0xbd744,false)?;
    // VMA 0xbddec: b 0xbde10 <entry+0x1634>  @ imm = #0x1c
    c.b(C::Al,0xbde10,false)?;
    // VMA 0xbddf0: ldr r1, [pc, #0xe7c]        @ 0x2498 <entry+0x2498>
    c.literal(R::R1,0xbec74)?;
    // VMA 0xbddf4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbddf8: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f16c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbddfc: ldr r1, [pc, #0xe34]        @ 0x245c <entry+0x245c>
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbde00: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbde04: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f178
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbde08: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbde0c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbde10: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbde14: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbde18: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xbde1c: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // VMA 0xbde20: beq 0xbe29c <entry+0x1ac0>  @ imm = #0x474
    c.b(C::Eq,0xbe29c,false)?;
    // VMA 0xbde24: b 0xbe2ac <entry+0x1ad0>  @ imm = #0x480
    c.b(C::Al,0xbe2ac,false)?;
    // VMA 0xbde28: ldrb r0, [r5, #0x2cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,716)?)?;
    // VMA 0xbde2c: strb r0, [r9, #0x86c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,2156)?)?;
    // VMA 0xbde30: b 0xbe5b4 <entry+0x1dd8>  @ imm = #0x77c
    c.b(C::Al,0xbe5b4,false)?;
    // VMA 0xbde34: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbde38: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // VMA 0xbde3c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbde40: bne 0xbde14 <entry+0x1638>  @ imm = #-0x34
    c.b(C::Ne,0xbde14,false)?;
    // VMA 0xbde44: str r10, [r5, #0x1cc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,460)?)?;
    // VMA 0xbde48: b 0xbe288 <entry+0x1aac>  @ imm = #0x438
    c.b(C::Al,0xbe288,false)?;
    // VMA 0xbde4c: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbde50: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbde54: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // VMA 0xbde58: bl 0xe1e60 <entry+0x25684> @ imm = #0x24000
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbde5c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbde60: beq 0xbda4c <entry+0x1270>  @ imm = #-0x41c
    c.b(C::Eq,0xbda4c,false)?;
    // VMA 0xbde64: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xbde68: b 0xbe2ac <entry+0x1ad0>  @ imm = #0x43c
    c.b(C::Al,0xbe2ac,false)?;
    // VMA 0xbde6c: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbde70: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // VMA 0xbde74: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xbde78: bhs 0xbe278 <entry+0x1a9c>  @ imm = #0x3f8
    c.b(C::Cs,0xbe278,false)?;
    // VMA 0xbde7c: ldr r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbde80: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbde84: mov r6, r9
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // VMA 0xbde88: mov r4, r9
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(9)?))?;
    // VMA 0xbde8c: mov r8, r9
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(9)?))?;
    // VMA 0xbde90: mov r7, r9
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // VMA 0xbde94: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // VMA 0xbde98: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbde9c: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xbdea0: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // VMA 0xbdea4: str r2, [r9, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,192)?)?;
    // VMA 0xbdea8: str r0, [r6, #0xc4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(6)?,196)?)?;
    // VMA 0xbdeac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbdeb0: str r0, [r4, #0xc8]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::new(4)?,200)?)?;
    // VMA 0xbdeb4: str r2, [r8, #0xcc]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R2,R::new(8)?,204)?)?;
    // VMA 0xbdeb8: str r2, [r7, #0xd0]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R2,R::new(7)?,208)?)?;
    // VMA 0xbdebc: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xbdec0: str r2, [r3, #0xd4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,212)?)?;
    // VMA 0xbdec4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xbdec8: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xbdecc: str r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // VMA 0xbded0: str r0, [r1, #0xb0]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::R1,176)?)?;
    // VMA 0xbded4: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // VMA 0xbded8: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xbdedc: str r0, [r1, #0xb4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::R1,180)?)?;
    // VMA 0xbdee0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbdee4: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // VMA 0xbdee8: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xbdeec: bl 0xe1e60 <entry+0x25684> @ imm = #0x23f6c
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdef0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdef4: bne 0xbe638 <entry+0x1e5c>  @ imm = #0x73c
    c.b(C::Ne,0xbe638,false)?;
    // VMA 0xbdef8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xbdefc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbdf00: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xbdf04: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // VMA 0xbdf08: movhs r0, #8
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbdf0c: strhs r0, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(9)?,188)?)?;
    // VMA 0xbdf10: movhs r0, #5
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,5)?)?;
    // VMA 0xbdf14: strhs r0, [r9, #0xc4]
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(9)?,196)?)?;
    // VMA 0xbdf18: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbdf1c: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xbdf20: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbdf24: bl 0xe1e60 <entry+0x25684> @ imm = #0x23f34
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdf28: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdf2c: bne 0xbe638 <entry+0x1e5c>  @ imm = #0x704
    c.b(C::Ne,0xbe638,false)?;
    // VMA 0xbdf30: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbdf34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbdf38: beq 0xbe054 <entry+0x1878>  @ imm = #0x114
    c.b(C::Eq,0xbe054,false)?;
    // VMA 0xbdf3c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbdf40: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbdf44: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xbdf48: bl 0xe1e60 <entry+0x25684> @ imm = #0x23f10
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdf4c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdf50: bne 0xbe634 <entry+0x1e58>  @ imm = #0x6dc
    c.b(C::Ne,0xbe634,false)?;
    // VMA 0xbdf54: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // VMA 0xbdf58: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xbdf5c: bhi 0xbe9ec <entry+0x2210>  @ imm = #0xa88
    c.b(C::Hi,0xbe9ec,false)?;
    // VMA 0xbdf60: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdf64: beq 0xbe9ec <entry+0x2210>  @ imm = #0xa80
    c.b(C::Eq,0xbe9ec,false)?;
    // VMA 0xbdf68: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbdf6c: ldr r2, [pc, #0xcc0]        @ 0x2458 <entry+0x2458>
    c.literal(R::R2,0xbec34)?;
    // VMA 0xbdf70: mov r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,5)?)?;
    // VMA 0xbdf74: str r1, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,188)?)?;
    // VMA 0xbdf78: ldr r1, [pc, #0xcb0]        @ 0x2454 <entry+0x2454>
    c.literal(R::R1,0xbec30)?;
    // VMA 0xbdf7c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbdf80: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xbdf84: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbdf88: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f2fc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbdf8c: ldr r1, [pc, #0xca4]        @ 0x245c <entry+0x245c>
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbdf90: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbdf94: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f308
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbdf98: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbdf9c: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbdfa0: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xbdfa4: bl 0xe1e60 <entry+0x25684> @ imm = #0x23eb4
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbdfa8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdfac: bne 0xbe634 <entry+0x1e58>  @ imm = #0x680
    c.b(C::Ne,0xbe634,false)?;
    // VMA 0xbdfb0: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbdfb4: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xbdfb8: bhi 0xbeaa0 <entry+0x22c4>  @ imm = #0xae0
    c.b(C::Hi,0xbeaa0,false)?;
    // VMA 0xbdfbc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbdfc0: beq 0xbeaa0 <entry+0x22c4>  @ imm = #0xad8
    c.b(C::Eq,0xbeaa0,false)?;
    // VMA 0xbdfc4: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbdfc8: ldr r2, [pc, #0xc64]        @ 0x2458 <entry+0x2458>
    c.literal(R::R2,0xbec34)?;
    // VMA 0xbdfcc: mov r3, #532
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,532)?)?;
    // VMA 0xbdfd0: str r1, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,188)?)?;
    // VMA 0xbdfd4: ldr r1, [pc, #0xc60]        @ 0x2460 <entry+0x2460>
    c.literal(R::R1,0xbec3c)?;
    // VMA 0xbdfd8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbdfdc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbdfe0: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f354
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbdfe4: ldr r1, [pc, #0xc4c]        @ 0x245c <entry+0x245c>
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbdfe8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbdfec: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f360
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbdff0: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xbdff4: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // VMA 0xbdff8: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbdffc: bl 0xe1e60 <entry+0x25684> @ imm = #0x23e5c
    c.b(C::Al,0xe1e60,true)?;
    c.check(0xbe000)
}
