
/// Rebuild codec stream output register packets and per-plane descriptors,
/// retaining the original conditional diagnostic logging and MMIO writes.
/// Three tiny control stubs follow the first function's return. The four
/// 32-bit literal addresses between them are data rather than opcodes.
fn configure_codec_output_stream_mmio(c:&mut Code)->Result<(),String>{
    c.check(0xc3574)?;
    // VMA 0xc3574: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc3578: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc357c: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // VMA 0xc3580: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xc3584: ldr r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1296)?)?;
    // VMA 0xc3588: ldr r5, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,4)?)?;
    // VMA 0xc358c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc3590: beq 0xc35a0 <entry+0x5a0>   @ imm = #0x8
    c.b(C::Eq,0xc35a0,false)?;
    // VMA 0xc3594: ldr r0, [r4, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,276)?)?;
    // VMA 0xc3598: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc359c: beq 0xc3768 <entry+0x768>   @ imm = #0x1c4
    c.b(C::Eq,0xc3768,false)?;
    // VMA 0xc35a0: mov r0, #852
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,852)?)?;
    // VMA 0xc35a4: mov r3, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,166)?)?;
    // VMA 0xc35a8: mov r7, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3758096384)?)?;
    // VMA 0xc35ac: mov r1, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2896)?)?;
    // VMA 0xc35b0: mov r2, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3024)?)?;
    // VMA 0xc35b4: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc35b8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xc35bc: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xc35c0: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // VMA 0xc35c4: orr r0, r0, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc35c8: orr r1, r1, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc35cc: orr r2, r2, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc35d0: str r3, [r0, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R0,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc35d4: ldr r12, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(4)?,296)?)?;
    // VMA 0xc35d8: str r12, [r1, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::R1,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc35dc: str r3, [r2, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R2,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc35e0: mov r3, #972
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,972)?)?;
    // VMA 0xc35e4: ldr r6, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,152)?)?;
    // VMA 0xc35e8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // VMA 0xc35ec: orr r3, r3, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc35f0: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xc35f4: str r6, [r3, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::new(3)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc35f8: mov r7, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,182)?)?;
    // VMA 0xc35fc: mov r6, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,183)?)?;
    // VMA 0xc3600: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // VMA 0xc3604: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // VMA 0xc3608: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // VMA 0xc360c: ldr lr, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(4)?,300)?)?;
    // VMA 0xc3610: str lr, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,0)?)?;
    // VMA 0xc3614: str r7, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,0)?)?;
    // VMA 0xc3618: ldr r7, [r4, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,156)?)?;
    // VMA 0xc361c: bic r7, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(7)?,255)?)?;
    // VMA 0xc3620: str r7, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(3)?,0)?)?;
    // VMA 0xc3624: mov r7, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,167)?)?;
    // VMA 0xc3628: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // VMA 0xc362c: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // VMA 0xc3630: ldr r5, [r4, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,304)?)?;
    // VMA 0xc3634: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc3638: str r7, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,0)?)?;
    // VMA 0xc363c: ldr r7, [r4, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,176)?)?;
    // VMA 0xc3640: bic r7, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(7)?,255)?)?;
    // VMA 0xc3644: str r7, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(3)?,0)?)?;
    // VMA 0xc3648: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc364c: ldr r7, [r4, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,308)?)?;
    // VMA 0xc3650: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc3654: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3658: ldr r6, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,180)?)?;
    // VMA 0xc365c: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xc3660: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc3664: mov r6, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,166)?)?;
    // VMA 0xc3668: orr r6, r6, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4352)?)?;
    // VMA 0xc366c: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc3670: str r12, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,0)?)?;
    // VMA 0xc3674: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3678: ldr r6, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,152)?)?;
    // VMA 0xc367c: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xc3680: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc3684: mov r6, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,182)?)?;
    // VMA 0xc3688: orr r6, r6, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4352)?)?;
    // VMA 0xc368c: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc3690: str lr, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,0)?)?;
    // VMA 0xc3694: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3698: ldr r6, [r4, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,156)?)?;
    // VMA 0xc369c: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xc36a0: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc36a4: mov r6, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,167)?)?;
    // VMA 0xc36a8: orr r6, r6, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4352)?)?;
    // VMA 0xc36ac: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc36b0: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc36b4: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc36b8: ldr r6, [r4, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,176)?)?;
    // VMA 0xc36bc: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xc36c0: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc36c4: mov r6, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,183)?)?;
    // VMA 0xc36c8: orr r6, r6, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4352)?)?;
    // VMA 0xc36cc: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc36d0: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc36d4: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc36d8: ldr r6, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,180)?)?;
    // VMA 0xc36dc: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xc36e0: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc36e4: mov r6, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,166)?)?;
    // VMA 0xc36e8: orr r6, r6, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4608)?)?;
    // VMA 0xc36ec: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc36f0: str r12, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,0)?)?;
    // VMA 0xc36f4: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc36f8: ldr r6, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,152)?)?;
    // VMA 0xc36fc: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xc3700: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc3704: mov r6, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,182)?)?;
    // VMA 0xc3708: orr r6, r6, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4608)?)?;
    // VMA 0xc370c: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc3710: str lr, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,0)?)?;
    // VMA 0xc3714: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3718: ldr r6, [r4, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,156)?)?;
    // VMA 0xc371c: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xc3720: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc3724: mov r6, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,167)?)?;
    // VMA 0xc3728: orr r6, r6, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4608)?)?;
    // VMA 0xc372c: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc3730: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc3734: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3738: ldr r6, [r4, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,176)?)?;
    // VMA 0xc373c: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xc3740: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc3744: mov r6, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,183)?)?;
    // VMA 0xc3748: orr r6, r6, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4608)?)?;
    // VMA 0xc374c: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc3750: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc3754: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3758: ldr r0, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,180)?)?;
    // VMA 0xc375c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc3760: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc3764: b 0xc3bcc <entry+0xbcc>   @ imm = #0x460
    c.b(C::Al,0xc3bcc,false)?;
    // VMA 0xc3768: ldr r6, [r4, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,2028)?)?;
    // VMA 0xc376c: ldr r10, [r4, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(4)?,2032)?)?;
    // VMA 0xc3770: ldr r8, [r4, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,2036)?)?;
    // VMA 0xc3774: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc3778: ldrne r2, [r6, #0x128]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(6)?,296)?)?;
    // VMA 0xc377c: cmpne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc3780: beq 0xc37b4 <entry+0x7b4>   @ imm = #0x2c
    c.b(C::Eq,0xc37b4,false)?;
    // VMA 0xc3784: ldr r3, [r6, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,300)?)?;
    // VMA 0xc3788: ldr r1, [pc, #0x44c]        @ 0xbdc <entry+0xbdc>
    c.literal(R::R1,0xc3bdc)?;
    // VMA 0xc378c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3790: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94b04
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3794: ldr r1, [pc, #0x444]        @ 0xbe0 <entry+0xbe0>
    c.literal(R::R1,0xc3be0)?;
    // VMA 0xc3798: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc379c: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94b10
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc37a0: mov r7, r6
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // VMA 0xc37a4: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // VMA 0xc37a8: bne 0xc37e4 <entry+0x7e4>   @ imm = #0x34
    c.b(C::Ne,0xc37e4,false)?;
    // VMA 0xc37ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc37b0: b 0xc3820 <entry+0x820>   @ imm = #0x68
    c.b(C::Al,0xc3820,false)?;
    // VMA 0xc37b4: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // VMA 0xc37b8: ldrne r2, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(10)?,296)?)?;
    // VMA 0xc37bc: cmpne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc37c0: beq 0xc38c0 <entry+0x8c0>   @ imm = #0xf8
    c.b(C::Eq,0xc38c0,false)?;
    // VMA 0xc37c4: ldr r3, [r10, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,300)?)?;
    // VMA 0xc37c8: ldr r1, [pc, #0x414]        @ 0xbe4 <entry+0xbe4>
    c.literal(R::R1,0xc3be4)?;
    // VMA 0xc37cc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc37d0: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94b44
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc37d4: ldr r1, [pc, #0x404]        @ 0xbe0 <entry+0xbe0>
    c.literal(R::R1,0xc3be0)?;
    // VMA 0xc37d8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc37dc: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94b50
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc37e0: mov r7, r10
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(10)?))?;
    // VMA 0xc37e4: ldr r2, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,296)?)?;
    // VMA 0xc37e8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc37ec: beq 0xc381c <entry+0x81c>   @ imm = #0x28
    c.b(C::Eq,0xc381c,false)?;
    // VMA 0xc37f0: ldr r3, [r10, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,300)?)?;
    // VMA 0xc37f4: ldr r1, [pc, #0x3e8]        @ 0xbe4 <entry+0xbe4>
    c.literal(R::R1,0xc3be4)?;
    // VMA 0xc37f8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc37fc: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xc3800: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3804: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94b78
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3808: ldr r1, [pc, #0x3d0]        @ 0xbe0 <entry+0xbe0>
    c.literal(R::R1,0xc3be0)?;
    // VMA 0xc380c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3810: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94b84
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3814: mov r9, r10
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(10)?))?;
    // VMA 0xc3818: b 0xc3854 <entry+0x854>   @ imm = #0x34
    c.b(C::Al,0xc3854,false)?;
    // VMA 0xc381c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3820: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xc3824: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc3828: ldrne r2, [r6, #0x128]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(6)?,296)?)?;
    // VMA 0xc382c: cmpne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc3830: beq 0xc3888 <entry+0x888>   @ imm = #0x50
    c.b(C::Eq,0xc3888,false)?;
    // VMA 0xc3834: ldr r3, [r6, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,300)?)?;
    // VMA 0xc3838: ldr r1, [pc, #0x39c]        @ 0xbdc <entry+0xbdc>
    c.literal(R::R1,0xc3bdc)?;
    // VMA 0xc383c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3840: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94bb4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3844: ldr r1, [pc, #0x394]        @ 0xbe0 <entry+0xbe0>
    c.literal(R::R1,0xc3be0)?;
    // VMA 0xc3848: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc384c: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94bc0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3850: mov r9, r6
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(6)?))?;
    // VMA 0xc3854: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xc3858: beq 0xc3904 <entry+0x904>   @ imm = #0xa4
    c.b(C::Eq,0xc3904,false)?;
    // VMA 0xc385c: ldr r2, [r8, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,296)?)?;
    // VMA 0xc3860: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc3864: beq 0xc3904 <entry+0x904>   @ imm = #0x98
    c.b(C::Eq,0xc3904,false)?;
    // VMA 0xc3868: ldr r3, [r8, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,300)?)?;
    // VMA 0xc386c: ldr r1, [pc, #0x374]        @ 0xbe8 <entry+0xbe8>
    c.literal(R::R1,0xc3be8)?;
    // VMA 0xc3870: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3874: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94be8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3878: ldr r1, [pc, #0x360]        @ 0xbe0 <entry+0xbe0>
    c.literal(R::R1,0xc3be0)?;
    // VMA 0xc387c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3880: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94bf4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3884: b 0xc3978 <entry+0x978>   @ imm = #0xec
    c.b(C::Al,0xc3978,false)?;
    // VMA 0xc3888: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xc388c: beq 0xc3900 <entry+0x900>   @ imm = #0x6c
    c.b(C::Eq,0xc3900,false)?;
    // VMA 0xc3890: ldr r2, [r8, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,296)?)?;
    // VMA 0xc3894: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc3898: beq 0xc3bd4 <entry+0xbd4>   @ imm = #0x334
    c.b(C::Eq,0xc3bd4,false)?;
    // VMA 0xc389c: ldr r3, [r8, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,300)?)?;
    // VMA 0xc38a0: ldr r1, [pc, #0x340]        @ 0xbe8 <entry+0xbe8>
    c.literal(R::R1,0xc3be8)?;
    // VMA 0xc38a4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc38a8: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94c1c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc38ac: ldr r1, [pc, #0x32c]        @ 0xbe0 <entry+0xbe0>
    c.literal(R::R1,0xc3be0)?;
    // VMA 0xc38b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc38b4: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94c28
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc38b8: mov r9, r8
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(8)?))?;
    // VMA 0xc38bc: b 0xc385c <entry+0x85c>   @ imm = #-0x68
    c.b(C::Al,0xc385c,false)?;
    // VMA 0xc38c0: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xc38c4: mov r7, r4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(4)?))?;
    // VMA 0xc38c8: beq 0xc37a4 <entry+0x7a4>   @ imm = #-0x12c
    c.b(C::Eq,0xc37a4,false)?;
    // VMA 0xc38cc: ldr r2, [r8, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,296)?)?;
    // VMA 0xc38d0: mov r7, r4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(4)?))?;
    // VMA 0xc38d4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc38d8: beq 0xc37a4 <entry+0x7a4>   @ imm = #-0x13c
    c.b(C::Eq,0xc37a4,false)?;
    // VMA 0xc38dc: ldr r3, [r8, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,300)?)?;
    // VMA 0xc38e0: ldr r1, [pc, #0x300]        @ 0xbe8 <entry+0xbe8>
    c.literal(R::R1,0xc3be8)?;
    // VMA 0xc38e4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc38e8: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94c5c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc38ec: ldr r1, [pc, #0x2ec]        @ 0xbe0 <entry+0xbe0>
    c.literal(R::R1,0xc3be0)?;
    // VMA 0xc38f0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc38f4: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94c68
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc38f8: mov r7, r8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // VMA 0xc38fc: b 0xc37a4 <entry+0x7a4>   @ imm = #-0x160
    c.b(C::Al,0xc37a4,false)?;
    // VMA 0xc3900: mov r9, r4
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(4)?))?;
    // VMA 0xc3904: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc3908: ldrne r2, [r6, #0x128]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(6)?,296)?)?;
    // VMA 0xc390c: cmpne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc3910: beq 0xc3938 <entry+0x938>   @ imm = #0x20
    c.b(C::Eq,0xc3938,false)?;
    // VMA 0xc3914: ldr r3, [r6, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,300)?)?;
    // VMA 0xc3918: ldr r1, [pc, #0x2bc]        @ 0xbdc <entry+0xbdc>
    c.literal(R::R1,0xc3bdc)?;
    // VMA 0xc391c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3920: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94c94
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3924: ldr r1, [pc, #0x2b4]        @ 0xbe0 <entry+0xbe0>
    c.literal(R::R1,0xc3be0)?;
    // VMA 0xc3928: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc392c: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94ca0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3930: mov r8, r6
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(6)?))?;
    // VMA 0xc3934: b 0xc3978 <entry+0x978>   @ imm = #0x3c
    c.b(C::Al,0xc3978,false)?;
    // VMA 0xc3938: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // VMA 0xc393c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc3940: bne 0xc3974 <entry+0x974>   @ imm = #0x2c
    c.b(C::Ne,0xc3974,false)?;
    // VMA 0xc3944: ldr r2, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,296)?)?;
    // VMA 0xc3948: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc394c: beq 0xc3974 <entry+0x974>   @ imm = #0x20
    c.b(C::Eq,0xc3974,false)?;
    // VMA 0xc3950: ldr r3, [r10, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,300)?)?;
    // VMA 0xc3954: ldr r1, [pc, #0x288]        @ 0xbe4 <entry+0xbe4>
    c.literal(R::R1,0xc3be4)?;
    // VMA 0xc3958: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc395c: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94cd0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc3960: ldr r1, [pc, #0x278]        @ 0xbe0 <entry+0xbe0>
    c.literal(R::R1,0xc3be0)?;
    // VMA 0xc3964: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc3968: bl 0x2ec94 <entry+0xfffffffffff6bc94> @ imm = #-0x94cdc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc396c: mov r8, r10
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(10)?))?;
    // VMA 0xc3970: b 0xc3978 <entry+0x978>   @ imm = #0x0
    c.b(C::Al,0xc3978,false)?;
    // VMA 0xc3974: mov r8, r4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(4)?))?;
    // VMA 0xc3978: ldr r0, [r9, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,152)?)?;
    // VMA 0xc397c: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // VMA 0xc3980: ldr r0, [r7, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,152)?)?;
    // VMA 0xc3984: ldr r12, [r9, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(9)?,156)?)?;
    // VMA 0xc3988: ldr r2, [r9, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,176)?)?;
    // VMA 0xc398c: ldr r3, [r9, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,180)?)?;
    // VMA 0xc3990: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // VMA 0xc3994: ldr r4, [r7, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,180)?)?;
    // VMA 0xc3998: ldr r1, [r8, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,152)?)?;
    // VMA 0xc399c: ldr r6, [r7, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(7)?,156)?)?;
    // VMA 0xc39a0: ldr r0, [r7, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,176)?)?;
    // VMA 0xc39a4: cmp r4, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc39a8: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // VMA 0xc39ac: movhi r3, r7
    c.emit(arm32::mov_register(C::Hi,R::new(3)?,R::new(7)?))?;
    // VMA 0xc39b0: ldr r4, [r3, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(3)?,180)?)?;
    // VMA 0xc39b4: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // VMA 0xc39b8: ldr lr, [r8, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(8)?,180)?)?;
    // VMA 0xc39bc: ldr r1, [r8, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,156)?)?;
    // VMA 0xc39c0: ldr r10, [r8, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(8)?,176)?)?;
    // VMA 0xc39c4: cmp r4, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xc39c8: mov r4, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // VMA 0xc39cc: movls r3, r8
    c.emit(arm32::mov_register(C::Ls,R::new(3)?,R::new(8)?))?;
    // VMA 0xc39d0: cmp r0, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc39d4: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xc39d8: ldr r3, [r3, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,180)?)?;
    // VMA 0xc39dc: movhi r0, r7
    c.emit(arm32::mov_register(C::Hi,R::R0,R::new(7)?))?;
    // VMA 0xc39e0: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // VMA 0xc39e4: mov r3, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,166)?)?;
    // VMA 0xc39e8: ldr r2, [r0, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,176)?)?;
    // VMA 0xc39ec: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xc39f0: cmp r2, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc39f4: movls r0, r8
    c.emit(arm32::mov_register(C::Ls,R::R0,R::new(8)?))?;
    // VMA 0xc39f8: cmp r6, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xc39fc: mov r12, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,166)?)?;
    // VMA 0xc3a00: ldr r10, [r0, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,176)?)?;
    // VMA 0xc3a04: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xc3a08: orr r12, r12, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4352)?)?;
    // VMA 0xc3a0c: movhi r0, r7
    c.emit(arm32::mov_register(C::Hi,R::R0,R::new(7)?))?;
    // VMA 0xc3a10: ldr r2, [r0, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,156)?)?;
    // VMA 0xc3a14: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc3a18: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // VMA 0xc3a1c: ldr r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,4)?)?;
    // VMA 0xc3a20: movls r0, r8
    c.emit(arm32::mov_register(C::Ls,R::R0,R::new(8)?))?;
    // VMA 0xc3a24: ldr r6, [r0, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,156)?)?;
    // VMA 0xc3a28: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xc3a2c: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc3a30: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xc3a34: movhi r0, r7
    c.emit(arm32::mov_register(C::Hi,R::R0,R::new(7)?))?;
    // VMA 0xc3a38: ldr r1, [r0, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,152)?)?;
    // VMA 0xc3a3c: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc3a40: mov r1, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2896)?)?;
    // VMA 0xc3a44: movls r0, r8
    c.emit(arm32::mov_register(C::Ls,R::R0,R::new(8)?))?;
    // VMA 0xc3a48: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xc3a4c: ldr r0, [r0, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,152)?)?;
    // VMA 0xc3a50: orr r1, r1, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc3a54: bic lr, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::LR,R::R0,255)?)?;
    // VMA 0xc3a58: mov r0, #852
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,852)?)?;
    // VMA 0xc3a5c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc3a60: orr r0, r0, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc3a64: str r3, [r0, r4]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xc3a68: ldr r2, [r7, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,296)?)?;
    // VMA 0xc3a6c: str r2, [r1, r4]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xc3a70: mov r2, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3024)?)?;
    // VMA 0xc3a74: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // VMA 0xc3a78: orr r2, r2, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc3a7c: str r3, [r2, r4]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xc3a80: mov r3, #972
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,972)?)?;
    // VMA 0xc3a84: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // VMA 0xc3a88: orr r3, r3, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc3a8c: mov r5, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,182)?)?;
    // VMA 0xc3a90: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // VMA 0xc3a94: str lr, [r3, r4]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::LR,R::new(3)?,R::new(4)?,Shift::Lsl,0)?)?;
    // VMA 0xc3a98: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc3a9c: ldr r4, [r7, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,300)?)?;
    // VMA 0xc3aa0: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // VMA 0xc3aa4: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xc3aa8: bic r5, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(6)?,255)?)?;
    // VMA 0xc3aac: mov r6, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,167)?)?;
    // VMA 0xc3ab0: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // VMA 0xc3ab4: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xc3ab8: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc3abc: ldr r4, [r7, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(7)?,304)?)?;
    // VMA 0xc3ac0: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // VMA 0xc3ac4: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3ac8: mov r6, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,183)?)?;
    // VMA 0xc3acc: bic r4, r10, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(4)?,R::new(10)?,255)?)?;
    // VMA 0xc3ad0: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // VMA 0xc3ad4: str r4, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(3)?,0)?)?;
    // VMA 0xc3ad8: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc3adc: ldr r7, [r7, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,308)?)?;
    // VMA 0xc3ae0: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc3ae4: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3ae8: ldr r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,16)?)?;
    // VMA 0xc3aec: bic r6, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(7)?,255)?)?;
    // VMA 0xc3af0: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc3af4: str r12, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,0)?)?;
    // VMA 0xc3af8: ldr r7, [r9, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,296)?)?;
    // VMA 0xc3afc: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc3b00: str r12, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R2,0)?)?;
    // VMA 0xc3b04: mov r12, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,182)?)?;
    // VMA 0xc3b08: str lr, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // VMA 0xc3b0c: orr r12, r12, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4352)?)?;
    // VMA 0xc3b10: str r12, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,0)?)?;
    // VMA 0xc3b14: ldr r7, [r9, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,300)?)?;
    // VMA 0xc3b18: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc3b1c: str r12, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R2,0)?)?;
    // VMA 0xc3b20: mov r12, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,167)?)?;
    // VMA 0xc3b24: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xc3b28: orr r12, r12, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4352)?)?;
    // VMA 0xc3b2c: str r12, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,0)?)?;
    // VMA 0xc3b30: ldr r7, [r9, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,304)?)?;
    // VMA 0xc3b34: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc3b38: str r12, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R2,0)?)?;
    // VMA 0xc3b3c: mov r12, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,183)?)?;
    // VMA 0xc3b40: str r4, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(3)?,0)?)?;
    // VMA 0xc3b44: orr r12, r12, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4352)?)?;
    // VMA 0xc3b48: str r12, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,0)?)?;
    // VMA 0xc3b4c: ldr r7, [r9, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,308)?)?;
    // VMA 0xc3b50: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc3b54: str r12, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R2,0)?)?;
    // VMA 0xc3b58: mov r12, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,166)?)?;
    // VMA 0xc3b5c: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc3b60: orr r12, r12, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4608)?)?;
    // VMA 0xc3b64: str r12, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,0)?)?;
    // VMA 0xc3b68: ldr r7, [r8, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(8)?,296)?)?;
    // VMA 0xc3b6c: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc3b70: str r12, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R2,0)?)?;
    // VMA 0xc3b74: mov r12, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,182)?)?;
    // VMA 0xc3b78: str lr, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // VMA 0xc3b7c: orr r12, r12, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4608)?)?;
    // VMA 0xc3b80: str r12, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,0)?)?;
    // VMA 0xc3b84: ldr r7, [r8, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(8)?,300)?)?;
    // VMA 0xc3b88: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc3b8c: mov r7, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,167)?)?;
    // VMA 0xc3b90: str r12, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R2,0)?)?;
    // VMA 0xc3b94: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xc3b98: orr r7, r7, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4608)?)?;
    // VMA 0xc3b9c: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // VMA 0xc3ba0: ldr r5, [r8, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(8)?,304)?)?;
    // VMA 0xc3ba4: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc3ba8: str r7, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,0)?)?;
    // VMA 0xc3bac: mov r7, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,183)?)?;
    // VMA 0xc3bb0: str r4, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(3)?,0)?)?;
    // VMA 0xc3bb4: orr r7, r7, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4608)?)?;
    // VMA 0xc3bb8: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // VMA 0xc3bbc: ldr r0, [r8, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,308)?)?;
    // VMA 0xc3bc0: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xc3bc4: str r7, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,0)?)?;
    // VMA 0xc3bc8: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc3bcc: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc3bd0: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc3bd4: mov r9, r4
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(4)?))?;
    // VMA 0xc3bd8: b 0xc385c <entry+0x85c>   @ imm = #-0x384
    c.b(C::Al,0xc385c,false)?;
    c.check(0xc3bdc)?;
    const STREAM_OUTPUT_DIAGNOSTICS:[u32;4]=[
        0x000f_b239,0x000e_2788,0x000f_b262,0x000f_b28d,
    ];
    for addr in STREAM_OUTPUT_DIAGNOSTICS {c.emit(addr)?;}
    c.check(0xc3bec)?;
    // VMA 0xc3bec: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xc3bf0: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xc3bf4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc3bf8: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // VMA 0xc3bfc: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xc3c00: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc3c04: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3c08: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xc3c0c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xc3c10: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xc3c14)
}
