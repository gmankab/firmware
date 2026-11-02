
/// Write controller timing fields, issue error callbacks and restore the original frame.
fn finish_hardware_codec_profile(c:&mut Code)->Result<(),String>{
    c.check(0xbcb88)?;
    // VMA 0xbcb88: lsl r0, r8, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(8)?,Shift::Lsl,4)?)?;
    // VMA 0xbcb8c: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xbcb90: ldr r0, [r9, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,732)?)?;
    // VMA 0xbcb94: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xbcb98: beq 0xbcbac <entry+0x3d0>   @ imm = #0xc
    c.b(C::Eq,0xbcbac,false)?;
    // VMA 0xbcb9c: ldr r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1296)?)?;
    // VMA 0xbcba0: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xbcba4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbcba8: bne 0xbcbe0 <entry+0x404>   @ imm = #0x30
    c.b(C::Ne,0xbcbe0,false)?;
    // VMA 0xbcbac: ldrb r0, [r10, #0x85c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,2140)?)?;
    // VMA 0xbcbb0: ldr r1, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,236)?)?;
    // VMA 0xbcbb4: ldr r6, [r9, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,544)?)?;
    // VMA 0xbcbb8: ldr r3, [r9, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,308)?)?;
    // VMA 0xbcbbc: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xbcbc0: ldr r1, [pc, #0x428]        @ 0x814 <entry+0x814>
    c.literal(R::R1,0xbcff0)?;
    // VMA 0xbcbc4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbcbc8: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xbcbcc: str r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-32)?)?;
    // VMA 0xbcbd0: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8df44
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbcbd4: ldr r1, [pc, #0x400]        @ 0x800 <entry+0x800>
    c.literal(R::R1,0xbcfdc)?;
    // VMA 0xbcbd8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbcbdc: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8df50
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbcbe0: ldr r1, [r4, #0x784]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1924)?)?;
    // VMA 0xbcbe4: orr r0, r5, #676
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,676)?)?;
    // VMA 0xbcbe8: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xbcbec: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xbcbf0: mov r8, r4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(4)?))?;
    // VMA 0xbcbf4: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbcbf8: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbcbfc: orr r0, r5, #680
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,680)?)?;
    // VMA 0xbcc00: ldr r1, [r4, #0x7a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1956)?)?;
    // VMA 0xbcc04: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbcc08: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbcc0c: orr r0, r5, #852
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,852)?)?;
    // VMA 0xbcc10: orr r1, r5, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,2896)?)?;
    // VMA 0xbcc14: orr r10, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::R0,14336)?)?;
    // VMA 0xbcc18: mov r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,72)?)?;
    // VMA 0xbcc1c: orr r5, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R1,12288)?)?;
    // VMA 0xbcc20: str r0, [r10, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbcc24: str r3, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,20)?)?;
    // VMA 0xbcc28: ldr r1, [r4, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2036)?)?;
    // VMA 0xbcc2c: mov r4, r9
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(9)?))?;
    // VMA 0xbcc30: str r1, [r5, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::new(5)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbcc34: orr r1, r3, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(3)?,3024)?)?;
    // VMA 0xbcc38: orr r7, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R1,12288)?)?;
    // VMA 0xbcc3c: str r0, [r7, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbcc40: orr r0, r3, #972
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,972)?)?;
    // VMA 0xbcc44: orr r9, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::R0,14336)?)?;
    // VMA 0xbcc48: ldr r0, [r8, #0x7f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2040)?)?;
    // VMA 0xbcc4c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbcc50: str r0, [r9, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(9)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbcc54: mov r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,72)?)?;
    // VMA 0xbcc58: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbcc5c: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbcc60: ldr r1, [r8, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,2036)?)?;
    // VMA 0xbcc64: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbcc68: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbcc6c: ldr r0, [r8, #0x7f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2040)?)?;
    // VMA 0xbcc70: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbcc74: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbcc78: mov r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,88)?)?;
    // VMA 0xbcc7c: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbcc80: ldr r1, [r8, #0x804]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,2052)?)?;
    // VMA 0xbcc84: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbcc88: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbcc8c: ldr r0, [r8, #0x808]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2056)?)?;
    // VMA 0xbcc90: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbcc94: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbcc98: mov r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,88)?)?;
    // VMA 0xbcc9c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbcca0: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbcca4: ldr r1, [r8, #0x804]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,2052)?)?;
    // VMA 0xbcca8: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbccac: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbccb0: ldr r0, [r8, #0x808]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2056)?)?;
    // VMA 0xbccb4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbccb8: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbccbc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbccc0: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbccc4: ldr r1, [r8, #0x774]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1908)?)?;
    // VMA 0xbccc8: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbcccc: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbccd0: ldr r0, [r8, #0x778]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1912)?)?;
    // VMA 0xbccd4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbccd8: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbccdc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbcce0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbcce4: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbcce8: ldr r1, [r8, #0x774]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1908)?)?;
    // VMA 0xbccec: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbccf0: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbccf4: ldr r0, [r8, #0x778]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1912)?)?;
    // VMA 0xbccf8: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbccfc: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbcd00: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // VMA 0xbcd04: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbcd08: ldr r1, [r8, #0x794]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1940)?)?;
    // VMA 0xbcd0c: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbcd10: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbcd14: ldr r0, [r8, #0x798]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1944)?)?;
    // VMA 0xbcd18: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbcd1c: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbcd20: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // VMA 0xbcd24: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbcd28: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbcd2c: ldr r1, [r8, #0x794]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1940)?)?;
    // VMA 0xbcd30: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbcd34: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbcd38: ldr r0, [r8, #0x798]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1944)?)?;
    // VMA 0xbcd3c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbcd40: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbcd44: ldr r1, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,388)?)?;
    // VMA 0xbcd48: ldr r0, [r4, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,380)?)?;
    // VMA 0xbcd4c: ldr r12, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(4)?,384)?)?;
    // VMA 0xbcd50: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // VMA 0xbcd54: add r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbcd58: ldr r1, [r4, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,392)?)?;
    // VMA 0xbcd5c: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // VMA 0xbcd60: ldr r1, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,288)?)?;
    // VMA 0xbcd64: ldr r3, [r4, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,260)?)?;
    // VMA 0xbcd68: ldr r2, [r4, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,264)?)?;
    // VMA 0xbcd6c: ldr lr, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(4)?,284)?)?;
    // VMA 0xbcd70: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // VMA 0xbcd74: mov r1, #102
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,102)?)?;
    // VMA 0xbcd78: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xbcd7c: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // VMA 0xbcd80: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xbcd84: add r0, r3, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbcd88: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xbcd8c: add r1, r12, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R12,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbcd90: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbcd94: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbcd98: mov r0, #118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,118)?)?;
    // VMA 0xbcd9c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbcda0: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbcda4: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbcda8: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbcdac: add r0, r2, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbcdb0: ldr r2, [pc, #0x23c]        @ 0x818 <entry+0x818>
    c.literal(R::R2,0xbcff4)?;
    // VMA 0xbcdb4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbcdb8: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbcdbc: mov r0, #103
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,103)?)?;
    // VMA 0xbcdc0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbcdc4: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbcdc8: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // VMA 0xbcdcc: add r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbcdd0: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbcdd4: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbcdd8: add r0, lr, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::LR,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbcddc: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbcde0: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbcde4: mov r0, #119
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,119)?)?;
    // VMA 0xbcde8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbcdec: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbcdf0: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // VMA 0xbcdf4: add r1, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbcdf8: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xbcdfc: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbce00: mov r1, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,18)?)?;
    // VMA 0xbce04: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xbce08: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // VMA 0xbce0c: add r0, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbce10: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbce14: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbce18: ldr r6, [r4, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,428)?)?;
    // VMA 0xbce1c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xbce20: bl 0x17b0 <entry+0xfffffffffff44fd4> @ imm = #-0xbb678
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbce24: mov r3, #38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,38)?)?;
    // VMA 0xbce28: ldr r0, [r4, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,332)?)?;
    // VMA 0xbce2c: ldr r1, [r4, #0x150]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,336)?)?;
    // VMA 0xbce30: ldr r2, [r4, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,432)?)?;
    // VMA 0xbce34: str r3, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,0)?)?;
    // VMA 0xbce38: ldr r4, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-32)?)?;
    // VMA 0xbce3c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbce40: add r6, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbce44: add r2, r2, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbce48: mov r4, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xbce4c: str r6, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,0)?)?;
    // VMA 0xbce50: str r3, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(7)?,0)?)?;
    // VMA 0xbce54: mov r3, #38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,38)?)?;
    // VMA 0xbce58: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbce5c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xbce60: str r3, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,0)?)?;
    // VMA 0xbce64: str r6, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,0)?)?;
    // VMA 0xbce68: str r3, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(7)?,0)?)?;
    // VMA 0xbce6c: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbce70: mov r0, #54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,54)?)?;
    // VMA 0xbce74: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xbce78: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xbce7c: str r2, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,0)?)?;
    // VMA 0xbce80: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbce84: bic r0, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R1,255)?)?;
    // VMA 0xbce88: mov r1, #54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,54)?)?;
    // VMA 0xbce8c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xbce90: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbce94: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // VMA 0xbce98: str r2, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,0)?)?;
    // VMA 0xbce9c: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xbcea0: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // VMA 0xbcea4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // VMA 0xbcea8: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xbceac: ldr r7, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,20)?)?;
    // VMA 0xbceb0: orr r0, r7, #2384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,2384)?)?;
    // VMA 0xbceb4: mov r6, r7
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(7)?))?;
    // VMA 0xbceb8: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xbcebc: str r1, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbcec0: ldr r1, [pc, #0x130]        @ 0x81c <entry+0x81c>
    c.literal(R::R1,0xbcff8)?;
    // VMA 0xbcec4: orr r0, r7, #2080
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,2080)?)?;
    // VMA 0xbcec8: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xbcecc: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xbced0: and r1, r2, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xbced4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbced8: orr r1, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16)?)?;
    // VMA 0xbcedc: orr r1, r1, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65536)?)?;
    // VMA 0xbcee0: str r1, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbcee4: orr r0, r7, #36
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,36)?)?;
    // VMA 0xbcee8: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbceec: str r2, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbcef0: orr r0, r7, #2064
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,2064)?)?;
    // VMA 0xbcef4: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xbcef8: str r1, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbcefc: orr r0, r7, #20
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,20)?)?;
    // VMA 0xbcf00: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbcf04: str r2, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbcf08: orr r0, r7, #14400
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,14400)?)?;
    // VMA 0xbcf0c: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // VMA 0xbcf10: str r1, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbcf14: orr r0, r7, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,2096)?)?;
    // VMA 0xbcf18: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xbcf1c: str r1, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbcf20: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xbcf24: orr r0, r7, #2160
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,2160)?)?;
    // VMA 0xbcf28: ldr r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // VMA 0xbcf2c: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // VMA 0xbcf30: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xbcf34: and r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbcf38: orr r5, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // VMA 0xbcf3c: ldr r1, [pc, #0xb8]         @ 0x820 <entry+0x820>
    c.literal(R::R1,0xbcffc)?;
    // VMA 0xbcf40: str r5, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbcf44: orr r0, r7, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,2144)?)?;
    // VMA 0xbcf48: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xbcf4c: str r5, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbcf50: ldr r2, [r8, #0x1d4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R2,R::new(8)?,468)?)?;
    // VMA 0xbcf54: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xbcf58: ldr r3, [r8, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,4)?)?;
    // VMA 0xbcf5c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbcf60: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xbcf64: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xbcf68: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbcf6c: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8e2e0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbcf70: ldr r1, [pc, #0x64]         @ 0x800 <entry+0x800>
    c.literal(R::R1,0xbcfdc)?;
    // VMA 0xbcf74: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbcf78: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8e2ec
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbcf7c: ldr r1, [r8, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,4)?)?;
    // VMA 0xbcf80: ldrh r2, [r8]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(8)?,0)?)?;
    // VMA 0xbcf84: orr r0, r6, #68
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,68)?)?;
    // VMA 0xbcf88: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbcf8c: orr r1, r2, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,16,false)?)?;
    // VMA 0xbcf90: str r1, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xbcf94: orr r0, r6, #52
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,52)?)?;
    // VMA 0xbcf98: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbcf9c: str r1, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xbcfa0: mov r1, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,254)?)?;
    // VMA 0xbcfa4: orr r0, r6, #116
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,116)?)?;
    // VMA 0xbcfa8: ldr r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,40)?)?;
    // VMA 0xbcfac: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // VMA 0xbcfb0: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbcfb4: and r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbcfb8: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // VMA 0xbcfbc: orr r1, r1, r2, lsl #17
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,17,false)?)?;
    // VMA 0xbcfc0: str r1, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xbcfc4: orr r0, r6, #100
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,100)?)?;
    // VMA 0xbcfc8: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbcfcc: str r5, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xbcfd0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbcfd4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xbcfd8)
}

/// Named controller mode addresses, packed format constants and codec
/// diagnostic pointers. None of these ten words is executable A32 code.
const HARDWARE_CODEC_PROFILE_LITERALS:[u32;10]=[
    0x000f_a09b,0x000e_2788,0x0303_0303,0x060a_0001,
    0x0001_0202,0x00d0_b034,0x000f_6752,0x000f_9ffa,
    0x0010_3070,0x000f_a0c9,
];
