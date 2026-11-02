
/// Publish the paired codec controller registers under the selected
/// descriptor state; preserve the original NULL/unused-slot initialization
/// as a separate short function and its original BX LR return.
fn restore_controller_output_registers(c:&mut Code)->Result<(),String>{
    c.check(0xc32cc)?;
    // VMA 0xc32cc: ldr r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // VMA 0xc32d0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xc32d4: bne 0xc3544 <entry+0x544>   @ imm = #0x268
    c.b(C::Ne,0xc3544,false)?;
    // VMA 0xc32d8: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc32dc: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xc32e0: ldr r4, [r0, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,400)?)?;
    // VMA 0xc32e4: mov r1, #2064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2064)?)?;
    // VMA 0xc32e8: lsl lr, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::LR,R::R2,Shift::Lsl,15)?)?;
    // VMA 0xc32ec: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // VMA 0xc32f0: orr r3, r1, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R1,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc32f4: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc32f8: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc32fc: mov r3, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,20)?)?;
    // VMA 0xc3300: ldr r4, [r0, #0x194]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,404)?)?;
    // VMA 0xc3304: orr r3, r3, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,10240)?)?;
    // VMA 0xc3308: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc330c: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3310: mov r3, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,28)?)?;
    // VMA 0xc3314: ldr r4, [r0, #0x198]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,408)?)?;
    // VMA 0xc3318: orr r3, r3, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,10240)?)?;
    // VMA 0xc331c: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc3320: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3324: mov r3, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,24)?)?;
    // VMA 0xc3328: ldr r4, [r0, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,412)?)?;
    // VMA 0xc332c: orr r3, r3, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,10240)?)?;
    // VMA 0xc3330: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc3334: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3338: mov r3, #10432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10432)?)?;
    // VMA 0xc333c: ldr r4, [r0, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,416)?)?;
    // VMA 0xc3340: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc3344: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3348: mov r3, #2080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2080)?)?;
    // VMA 0xc334c: ldr r4, [r0, #0x1ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,492)?)?;
    // VMA 0xc3350: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // VMA 0xc3354: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc3358: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc335c: mov r3, #152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,152)?)?;
    // VMA 0xc3360: ldr r4, [r0, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,496)?)?;
    // VMA 0xc3364: orr r3, r3, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,10240)?)?;
    // VMA 0xc3368: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc336c: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3370: mov r3, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2096)?)?;
    // VMA 0xc3374: ldr r4, [r0, #0x1f4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,500)?)?;
    // VMA 0xc3378: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // VMA 0xc337c: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc3380: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3384: mov r3, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,44)?)?;
    // VMA 0xc3388: ldr r4, [r0, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,504)?)?;
    // VMA 0xc338c: orr r3, r3, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,10240)?)?;
    // VMA 0xc3390: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc3394: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3398: mov r3, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,156)?)?;
    // VMA 0xc339c: ldr r4, [r0, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,508)?)?;
    // VMA 0xc33a0: orr r3, r3, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,10240)?)?;
    // VMA 0xc33a4: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc33a8: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc33ac: mov r3, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,52)?)?;
    // VMA 0xc33b0: ldr r4, [r0, #0x200]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,512)?)?;
    // VMA 0xc33b4: orr r3, r3, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,10240)?)?;
    // VMA 0xc33b8: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc33bc: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc33c0: mov r3, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,56)?)?;
    // VMA 0xc33c4: ldr r4, [r0, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,420)?)?;
    // VMA 0xc33c8: orr r3, r3, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,10240)?)?;
    // VMA 0xc33cc: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc33d0: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc33d4: mov r3, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,60)?)?;
    // VMA 0xc33d8: ldr r4, [r0, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,424)?)?;
    // VMA 0xc33dc: orr r3, r3, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,10240)?)?;
    // VMA 0xc33e0: orr r3, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc33e4: str r4, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc33e8: mov r1, #10304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10304)?)?;
    // VMA 0xc33ec: add r3, r0, #428
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,428)?)?;
    // VMA 0xc33f0: orr r1, r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3758096384)?)?;
    // VMA 0xc33f4: add r1, r1, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc33f8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc33fc: ldr r4, [r3], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(4)?,R::new(3)?,4)?)?;
    // VMA 0xc3400: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xc3404: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // VMA 0xc3408: str r4, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(4)?,R::R1,4)?)?;
    // VMA 0xc340c: bne 0xc33fc <entry+0x3fc>   @ imm = #-0x18
    c.b(C::Ne,0xc33fc,false)?;
    // VMA 0xc3410: orr r1, lr, #852
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::LR,852)?)?;
    // VMA 0xc3414: mov r12, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xc3418: mov r4, #38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,38)?)?;
    // VMA 0xc341c: orr r3, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R1,14336)?)?;
    // VMA 0xc3420: orr r1, lr, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::LR,2896)?)?;
    // VMA 0xc3424: str r4, [r3, r12]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(4)?,R::new(3)?,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xc3428: orr r2, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R1,12288)?)?;
    // VMA 0xc342c: orr r1, lr, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::LR,3024)?)?;
    // VMA 0xc3430: ldr r5, [r0, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,332)?)?;
    // VMA 0xc3434: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xc3438: str r5, [r2, r12]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(5)?,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xc343c: str r4, [r1, r12]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(4)?,R::R1,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xc3440: orr r4, lr, #972
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::LR,972)?)?;
    // VMA 0xc3444: ldr r6, [r0, #0xe0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,224)?)?;
    // VMA 0xc3448: orr r4, r4, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,14336)?)?;
    // VMA 0xc344c: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // VMA 0xc3450: str r6, [r4, r12]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::new(4)?,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xc3454: mov r6, #38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,38)?)?;
    // VMA 0xc3458: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // VMA 0xc345c: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // VMA 0xc3460: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xc3464: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // VMA 0xc3468: ldr r5, [r0, #0xe0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,224)?)?;
    // VMA 0xc346c: bic r5, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,255)?)?;
    // VMA 0xc3470: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xc3474: mov r5, #54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,54)?)?;
    // VMA 0xc3478: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xc347c: ldr r6, [r0, #0x150]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,336)?)?;
    // VMA 0xc3480: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3484: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc3488: ldr r5, [r0, #0xe4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,228)?)?;
    // VMA 0xc348c: bic r5, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,255)?)?;
    // VMA 0xc3490: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xc3494: mov r5, #54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,54)?)?;
    // VMA 0xc3498: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // VMA 0xc349c: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xc34a0: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc34a4: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc34a8: ldr r5, [r0, #0xe4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,228)?)?;
    // VMA 0xc34ac: bic r5, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,255)?)?;
    // VMA 0xc34b0: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xc34b4: mov r5, #39
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,39)?)?;
    // VMA 0xc34b8: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xc34bc: ldr r6, [r0, #0x154]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,340)?)?;
    // VMA 0xc34c0: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc34c4: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc34c8: ldr r5, [r0, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,248)?)?;
    // VMA 0xc34cc: bic r5, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,255)?)?;
    // VMA 0xc34d0: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xc34d4: mov r5, #39
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,39)?)?;
    // VMA 0xc34d8: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // VMA 0xc34dc: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xc34e0: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc34e4: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc34e8: ldr r5, [r0, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,248)?)?;
    // VMA 0xc34ec: bic r5, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,255)?)?;
    // VMA 0xc34f0: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xc34f4: mov r5, #55
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,55)?)?;
    // VMA 0xc34f8: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xc34fc: ldr r6, [r0, #0x158]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,344)?)?;
    // VMA 0xc3500: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3504: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc3508: ldr r5, [r0, #0xfc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,252)?)?;
    // VMA 0xc350c: bic r5, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,255)?)?;
    // VMA 0xc3510: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xc3514: mov r5, #55
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,55)?)?;
    // VMA 0xc3518: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // VMA 0xc351c: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xc3520: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xc3524: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc3528: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc352c: ldr r0, [r0, #0xfc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,252)?)?;
    // VMA 0xc3530: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc3534: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc3538: orr r0, lr, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::LR,10240)?)?;
    // VMA 0xc353c: str r1, [r0, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xc3540: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc3544: mov r0, #2064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2064)?)?;
    // VMA 0xc3548: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc354c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc3550: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // VMA 0xc3554: orr r0, r0, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc3558: str r3, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc355c: mov r0, #168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,168)?)?;
    // VMA 0xc3560: orr r0, r0, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    // VMA 0xc3564: orr r0, r0, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc3568: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // VMA 0xc356c: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc3570: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;

    c.check(0xc3574)
}
