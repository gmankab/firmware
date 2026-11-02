
/// Publish aligned codec frame offsets and pixel-plane command metadata.
/// This is part of a larger codec function; the return remains outside this span.
fn write_frame_register_command_table(c:&mut Code)->Result<(),String>{
    c.check(0xbc200)?;
    // VMA 0xbc200: str r7, [r0, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc204: ldr r12, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,40)?)?;
    // VMA 0xbc208: str r12, [r1, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::R1,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc20c: str r7, [r2, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::R2,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc210: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // VMA 0xbc214: str r10, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,4)?)?;
    // VMA 0xbc218: bic r10, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(10)?,R::new(3)?,255)?)?;
    // VMA 0xbc21c: mov r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,6)?)?;
    // VMA 0xbc220: mov r5, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,22)?)?;
    // VMA 0xbc224: ldr r3, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,32)?)?;
    // VMA 0xbc228: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // VMA 0xbc22c: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // VMA 0xbc230: str r10, [r3, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(10)?,R::new(3)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc234: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbc238: str r12, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,40)?)?;
    // VMA 0xbc23c: str r12, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,0)?)?;
    // VMA 0xbc240: str r7, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,0)?)?;
    // VMA 0xbc244: mov r7, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,22)?)?;
    // VMA 0xbc248: str r10, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(3)?,0)?)?;
    // VMA 0xbc24c: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbc250: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,44)?)?;
    // VMA 0xbc254: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // VMA 0xbc258: str r7, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,0)?)?;
    // VMA 0xbc25c: str lr, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,0)?)?;
    // VMA 0xbc260: ldr r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,16)?)?;
    // VMA 0xbc264: bic r12, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R12,R::new(7)?,255)?)?;
    // VMA 0xbc268: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,0)?)?;
    // VMA 0xbc26c: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbc270: str r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,44)?)?;
    // VMA 0xbc274: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // VMA 0xbc278: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xbc27c: mov r5, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,7)?)?;
    // VMA 0xbc280: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,0)?)?;
    // VMA 0xbc284: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbc288: ldr r7, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-40)?)?;
    // VMA 0xbc28c: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xbc290: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xbc294: ldr r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xbc298: bic r9, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(9)?,R::new(5)?,255)?)?;
    // VMA 0xbc29c: mov r5, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,7)?)?;
    // VMA 0xbc2a0: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // VMA 0xbc2a4: str r9, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(3)?,0)?)?;
    // VMA 0xbc2a8: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbc2ac: str r7, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-40)?)?;
    // VMA 0xbc2b0: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xbc2b4: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xbc2b8: mov r5, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,23)?)?;
    // VMA 0xbc2bc: str r9, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(3)?,0)?)?;
    // VMA 0xbc2c0: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbc2c4: ldr r7, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,24)?)?;
    // VMA 0xbc2c8: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xbc2cc: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xbc2d0: ldr r5, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,12)?)?;
    // VMA 0xbc2d4: add r4, r5, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(5)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbc2d8: bic r8, r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(8)?,R::new(4)?,255)?)?;
    // VMA 0xbc2dc: mov r4, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,23)?)?;
    // VMA 0xbc2e0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xbc2e4: str r8, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(3)?,0)?)?;
    // VMA 0xbc2e8: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc2ec: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // VMA 0xbc2f0: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc2f4: mov r4, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,166)?)?;
    // VMA 0xbc2f8: str r8, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(3)?,0)?)?;
    // VMA 0xbc2fc: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xbc300: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc304: ldr r5, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,396)?)?;
    // VMA 0xbc308: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc30c: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc310: mov r4, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,182)?)?;
    // VMA 0xbc314: str r10, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(3)?,0)?)?;
    // VMA 0xbc318: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xbc31c: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc320: ldr r5, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,400)?)?;
    // VMA 0xbc324: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc328: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc32c: mov r4, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,167)?)?;
    // VMA 0xbc330: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,0)?)?;
    // VMA 0xbc334: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xbc338: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc33c: ldr r5, [r6, #0x194]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,404)?)?;
    // VMA 0xbc340: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc344: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc348: mov r4, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,183)?)?;
    // VMA 0xbc34c: str r9, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(3)?,0)?)?;
    // VMA 0xbc350: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xbc354: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc358: ldr r5, [r6, #0x198]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,408)?)?;
    // VMA 0xbc35c: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc360: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc364: mov r4, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,166)?)?;
    // VMA 0xbc368: str r8, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(3)?,0)?)?;
    // VMA 0xbc36c: orr r4, r4, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4352)?)?;
    // VMA 0xbc370: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc374: ldr r5, [r6, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,412)?)?;
    // VMA 0xbc378: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc37c: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc380: mov r4, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,182)?)?;
    // VMA 0xbc384: str r10, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(3)?,0)?)?;
    // VMA 0xbc388: orr r4, r4, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4352)?)?;
    // VMA 0xbc38c: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc390: ldr r5, [r6, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,416)?)?;
    // VMA 0xbc394: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc398: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc39c: mov r4, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,167)?)?;
    // VMA 0xbc3a0: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,0)?)?;
    // VMA 0xbc3a4: orr r4, r4, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4352)?)?;
    // VMA 0xbc3a8: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc3ac: ldr r5, [r6, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,420)?)?;
    // VMA 0xbc3b0: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc3b4: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc3b8: mov r4, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,183)?)?;
    // VMA 0xbc3bc: str r9, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(3)?,0)?)?;
    // VMA 0xbc3c0: orr r4, r4, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4352)?)?;
    // VMA 0xbc3c4: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc3c8: ldr r5, [r6, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,424)?)?;
    // VMA 0xbc3cc: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc3d0: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc3d4: mov r4, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,166)?)?;
    // VMA 0xbc3d8: str r8, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(3)?,0)?)?;
    // VMA 0xbc3dc: orr r4, r4, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4608)?)?;
    // VMA 0xbc3e0: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc3e4: ldr r5, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,380)?)?;
    // VMA 0xbc3e8: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc3ec: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc3f0: mov r4, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,182)?)?;
    // VMA 0xbc3f4: str r10, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(3)?,0)?)?;
    // VMA 0xbc3f8: orr r4, r4, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4608)?)?;
    // VMA 0xbc3fc: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc400: ldr r5, [r6, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,384)?)?;
    // VMA 0xbc404: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc408: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc40c: mov r4, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,167)?)?;
    // VMA 0xbc410: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,0)?)?;
    // VMA 0xbc414: orr r4, r4, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4608)?)?;
    // VMA 0xbc418: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc41c: ldr r5, [r6, #0x184]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,388)?)?;
    // VMA 0xbc420: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc424: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc428: mov r4, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,183)?)?;
    // VMA 0xbc42c: str r9, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(3)?,0)?)?;
    // VMA 0xbc430: orr r4, r4, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4608)?)?;
    // VMA 0xbc434: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc438: ldr r5, [r6, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,392)?)?;
    // VMA 0xbc43c: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc440: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc444: mov r4, #198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,198)?)?;
    // VMA 0xbc448: str r8, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(3)?,0)?)?;
    // VMA 0xbc44c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xbc450: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc454: ldr r5, [r6, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,396)?)?;
    // VMA 0xbc458: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc45c: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc460: mov r4, #214
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,214)?)?;
    // VMA 0xbc464: str r10, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(3)?,0)?)?;
    // VMA 0xbc468: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xbc46c: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc470: ldr r5, [r6, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,400)?)?;
    // VMA 0xbc474: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc478: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc47c: mov r4, #199
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,199)?)?;
    // VMA 0xbc480: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,0)?)?;
    // VMA 0xbc484: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xbc488: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc48c: ldr r5, [r6, #0x194]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,404)?)?;
    // VMA 0xbc490: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc494: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc498: mov r4, #215
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,215)?)?;
    // VMA 0xbc49c: str r9, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(3)?,0)?)?;
    // VMA 0xbc4a0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xbc4a4: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc4a8: ldr r5, [r6, #0x198]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,408)?)?;
    // VMA 0xbc4ac: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc4b0: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc4b4: mov r4, #198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,198)?)?;
    // VMA 0xbc4b8: str r8, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(3)?,0)?)?;
    // VMA 0xbc4bc: orr r4, r4, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4352)?)?;
    // VMA 0xbc4c0: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc4c4: ldr r5, [r6, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,412)?)?;
    // VMA 0xbc4c8: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc4cc: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc4d0: mov r4, #214
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,214)?)?;
    // VMA 0xbc4d4: str r10, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(3)?,0)?)?;
    // VMA 0xbc4d8: orr r4, r4, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4352)?)?;
    // VMA 0xbc4dc: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc4e0: ldr r5, [r6, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,416)?)?;
    // VMA 0xbc4e4: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc4e8: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc4ec: mov r4, #199
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,199)?)?;
    // VMA 0xbc4f0: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,0)?)?;
    // VMA 0xbc4f4: orr r4, r4, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4352)?)?;
    // VMA 0xbc4f8: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc4fc: ldr r5, [r6, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,420)?)?;
    // VMA 0xbc500: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc504: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc508: mov r4, #215
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,215)?)?;
    // VMA 0xbc50c: str r9, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(3)?,0)?)?;
    // VMA 0xbc510: orr r4, r4, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4352)?)?;
    // VMA 0xbc514: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc518: ldr r5, [r6, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,424)?)?;
    // VMA 0xbc51c: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc520: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc524: mov r4, #198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,198)?)?;
    // VMA 0xbc528: str r8, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(3)?,0)?)?;
    // VMA 0xbc52c: orr r4, r4, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4608)?)?;
    // VMA 0xbc530: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc534: ldr r5, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,380)?)?;
    // VMA 0xbc538: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc53c: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc540: mov r4, #214
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,214)?)?;
    // VMA 0xbc544: str r10, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(3)?,0)?)?;
    // VMA 0xbc548: orr r4, r4, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4608)?)?;
    // VMA 0xbc54c: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc550: ldr r5, [r6, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,384)?)?;
    // VMA 0xbc554: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc558: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc55c: mov r4, #199
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,199)?)?;
    // VMA 0xbc560: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,0)?)?;
    // VMA 0xbc564: orr r4, r4, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4608)?)?;
    // VMA 0xbc568: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc56c: ldr r5, [r6, #0x184]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,388)?)?;
    // VMA 0xbc570: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xbc574: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc578: mov r4, #215
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,215)?)?;
    // VMA 0xbc57c: str r9, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(3)?,0)?)?;
    // VMA 0xbc580: orr r4, r4, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4608)?)?;
    // VMA 0xbc584: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbc588: ldr r0, [r6, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,392)?)?;
    // VMA 0xbc58c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xbc590: str r4, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    // VMA 0xbc594: str r8, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(3)?,0)?)?;
    c.check(0xbc598)
}
