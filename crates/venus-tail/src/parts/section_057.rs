
/// Check whether an encoded stream mode requests an external controller
/// readback. The original helper falls through to the next routine rather
/// than synthesizing any return instruction.
fn check_codec_hardware_readback(c:&mut Code)->Result<(),String>{
    c.check(0xbbabc)?;
    // VMA 0xbbabc: ldr r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,4)?)?;
    // VMA 0xbbac0: ldr r2, [r2, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,2308)?)?;
    // VMA 0xbbac4: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xbbac8: bne 0xbbae8 <entry+0xa70>   @ imm = #0x18
    c.b(C::Ne,0xbbae8,false)?;
    // VMA 0xbbacc: mov r2, #3472
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3472)?)?;
    // VMA 0xbbad0: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xbbad4: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbbad8: ldr r2, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbbadc: mov r2, #3488
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3488)?)?;
    // VMA 0xbbae0: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbbae4: ldr r2, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    c.check(0xbbae8)
}

/// Program the controller's stream-status and packet-class register fields,
/// selecting the format-specific hardware memory windows.
fn program_codec_status_registers(c:&mut Code)->Result<(),String>{
    c.check(0xbbae8)?;
    // VMA 0xbbae8: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbbaec: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xbbaf0: mov r2, #852
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,852)?)?;
    // VMA 0xbbaf4: mov r12, #178
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,178)?)?;
    // VMA 0xbbaf8: mov lr, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,3758096384)?)?;
    // VMA 0xbbafc: mov r3, #972
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,972)?)?;
    // VMA 0xbbb00: mov r5, #16580608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16580608)?)?;
    // VMA 0xbbb04: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // VMA 0xbbb08: orr r12, r12, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4096)?)?;
    // VMA 0xbbb0c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // VMA 0xbbb10: orr r5, r5, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,33554432)?)?;
    // VMA 0xbbb14: orr r6, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbbb18: mov r2, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2896)?)?;
    // VMA 0xbbb1c: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbbb20: str r12, [r6, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::new(6)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbb24: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // VMA 0xbbb28: ldr r4, [r1, #0x880]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,2176)?)?;
    // VMA 0xbbb2c: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbbb30: str r4, [r2, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(4)?,R::R2,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbb34: mov r4, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3024)?)?;
    // VMA 0xbbb38: orr r4, r4, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,12288)?)?;
    // VMA 0xbbb3c: orr r4, r4, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(4)?,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbbb40: str r12, [r4, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::new(4)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbb44: str r5, [r3, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(5)?,R::new(3)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbb48: mov r5, #194
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,194)?)?;
    // VMA 0xbbb4c: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // VMA 0xbbb50: str r5, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,0)?)?;
    // VMA 0xbbb54: ldr r1, [r1, #0x884]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,2180)?)?;
    // VMA 0xbbb58: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xbbb5c: mov r1, #120832
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,120832)?)?;
    // VMA 0xbbb60: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xbbb64: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbbb68: orr r1, r1, #12320768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12320768)?)?;
    // VMA 0xbbb6c: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbbb70: mov r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // VMA 0xbbb74: orr r1, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbbb78: str r2, [r1, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbb7c: mov r1, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,104)?)?;
    // VMA 0xbbb80: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbbb84: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // VMA 0xbbb88: orr r1, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbbb8c: str r2, [r1, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbb90: mov r1, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,116)?)?;
    // VMA 0xbbb94: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // VMA 0xbbb98: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbbb9c: str r2, [r0, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbba0: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xbbba4)
}

/// Commit the companion stream parameters including fixed-point input-plane
/// offsets and buffering flags; the two original return sequences are kept.
fn program_codec_stream_aux_registers(c:&mut Code)->Result<(),String>{
    c.check(0xbbba4)?;
    // VMA 0xbbba4: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbbba8: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xbbbac: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xbbbb0: ldr r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,4)?)?;
    // VMA 0xbbbb4: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // VMA 0xbbbb8: ldrb r0, [r0, #0x494]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1172)?)?;
    // VMA 0xbbbbc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbbbc0: beq 0xbbca4 <entry+0xc2c>   @ imm = #0xdc
    c.b(C::Eq,0xbbca4,false)?;
    // VMA 0xbbbc4: add r0, r5, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,20)?)?;
    // VMA 0xbbbc8: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xbbbcc: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xbbbd0: bl 0xd9604 <entry+0x1e58c> @ imm = #0x1da2c
    c.b(C::Al,0xd9604,true)?;
    // VMA 0xbbbd4: mov r0, #852
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,852)?)?;
    // VMA 0xbbbd8: mov lr, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,3758096384)?)?;
    // VMA 0xbbbdc: mov r12, #38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,38)?)?;
    // VMA 0xbbbe0: mov r2, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2896)?)?;
    // VMA 0xbbbe4: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbbbe8: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // VMA 0xbbbec: orr r6, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbbf0: orr r2, r2, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbbf4: mov r0, #972
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,972)?)?;
    // VMA 0xbbbf8: str r12, [r6, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::new(6)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbbfc: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbbc00: ldr r3, [r5, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,428)?)?;
    // VMA 0xbbc04: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbc08: str r3, [r2, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R2,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbc0c: mov r3, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3024)?)?;
    // VMA 0xbbc10: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // VMA 0xbbc14: orr r3, r3, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbc18: str r12, [r3, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::new(3)?,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbc1c: mov r12, #38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,38)?)?;
    // VMA 0xbbc20: ldr r1, [r5, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,332)?)?;
    // VMA 0xbbc24: orr r12, r12, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4096)?)?;
    // VMA 0xbbc28: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xbbc2c: str r1, [r0, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R0,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbc30: str r12, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(6)?,0)?)?;
    // VMA 0xbbc34: ldr r1, [r5, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,428)?)?;
    // VMA 0xbbc38: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xbbc3c: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,0)?)?;
    // VMA 0xbbc40: mov r12, #54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,54)?)?;
    // VMA 0xbbc44: ldr r1, [r5, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,332)?)?;
    // VMA 0xbbc48: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xbbc4c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xbbc50: str r12, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(6)?,0)?)?;
    // VMA 0xbbc54: ldr r1, [r5, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,432)?)?;
    // VMA 0xbbc58: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xbbc5c: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(3)?,0)?)?;
    // VMA 0xbbc60: ldr r1, [r5, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,360)?)?;
    // VMA 0xbbc64: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xbbc68: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xbbc6c: mov r1, #54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,54)?)?;
    // VMA 0xbbc70: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xbbc74: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // VMA 0xbbc78: ldr r6, [r5, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,432)?)?;
    // VMA 0xbbc7c: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // VMA 0xbbc80: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbbc84: ldr r1, [r5, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,360)?)?;
    // VMA 0xbbc88: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xbbc8c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // VMA 0xbbc90: mov r0, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10240)?)?;
    // VMA 0xbbc94: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbbc98: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbc9c: str r1, [r0, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::LR,Shift::Lsl,0)?)?;
    // VMA 0xbbca0: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xbbca4: mov r0, #2064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2064)?)?;
    // VMA 0xbbca8: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbbcac: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbbcb0: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // VMA 0xbbcb4: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbcb8: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbbcbc: mov r0, #168
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,168)?)?;
    // VMA 0xbbcc0: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // VMA 0xbbcc4: orr r0, r0, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,10240)?)?;
    // VMA 0xbbcc8: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbccc: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbbcd0: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xbbcd4)
}


/// Validate codec mode, prepare controller MMIO windows and compact stream sizes.
/// This is part of a larger codec function; the return remains outside this span.
fn setup_stream_controller_mmio(c:&mut Code)->Result<(),String>{
    c.check(0xbbcd4)?;
    // VMA 0xbbcd4: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbbcd8: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xbbcdc: sub sp, sp, #60
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,60)?)?;
    // VMA 0xbbce0: str r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // VMA 0xbbce4: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xbbce8: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // VMA 0xbbcec: ldr r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,4)?)?;
    // VMA 0xbbcf0: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xbbcf4: ldr r0, [r6, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,56)?)?;
    // VMA 0xbbcf8: ldr r8, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(6)?,4)?)?;
    // VMA 0xbbcfc: lsl r4, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R0,Shift::Lsl,4)?)?;
    // VMA 0xbbd00: ldrb r0, [r6, #0x10]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // VMA 0xbbd04: cmp r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,3)?)?;
    // VMA 0xbbd08: movne r8, #4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(8)?,R::R0,4)?)?;
    // VMA 0xbbd0c: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // VMA 0xbbd10: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xbbd14: bhs 0xbbd28 <entry+0xcb0>   @ imm = #0xc
    c.b(C::Cs,0xbbd28,false)?;
    // VMA 0xbbd18: ldr r1, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,52)?)?;
    // VMA 0xbbd1c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xbbd20: bic r7, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::R1,1)?)?;
    // VMA 0xbbd24: b 0xbbd78 <entry+0xd00>   @ imm = #0x4c
    c.b(C::Al,0xbbd78,false)?;
    // VMA 0xbbd28: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbbd2c: bne 0xbbd3c <entry+0xcc4>   @ imm = #0x8
    c.b(C::Ne,0xbbd3c,false)?;
    // VMA 0xbbd30: ldr r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    // VMA 0xbbd34: lsl r7, r0, #20
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R0,Shift::Lsl,20)?)?;
    // VMA 0xbbd38: b 0xbbdd8 <entry+0xd60>   @ imm = #0x98
    c.b(C::Al,0xbbdd8,false)?;
    // VMA 0xbbd3c: ldr r1, [pc, #0xa48]        @ 0x1714 <entry+0x1714>
    c.literal(R::R1,0xbc78c)?;
    // VMA 0xbbd40: ldr r2, [pc, #0xa48]        @ 0x1718 <entry+0x1718>
    c.literal(R::R2,0xbc790)?;
    // VMA 0xbbd44: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbbd48: mov r3, #308
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,308)?)?;
    // VMA 0xbbd4c: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8d0c0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbbd50: ldr r1, [pc, #0xa3c]        @ 0x171c <entry+0x171c>
    c.literal(R::R1,0xbc794)?;
    // VMA 0xbbd54: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbbd58: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8d0cc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbbd5c: ldr r2, [pc, #0xa34]        @ 0x1720 <entry+0x1720>
    c.literal(R::R2,0xbc798)?;
    // VMA 0xbbd60: mov r1, #53
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,53)?)?;
    // VMA 0xbbd64: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbbd68: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbbd6c: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xbbd70: bl 0x17b0 <entry+0xfffffffffff46738> @ imm = #-0xba5c8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbbd74: ldrb r0, [r6, #0x10]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // VMA 0xbbd78: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,1)?)?;
    // VMA 0xbbd7c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xbbd80: bhs 0xbbd8c <entry+0xd14>   @ imm = #0x4
    c.b(C::Cs,0xbbd8c,false)?;
    // VMA 0xbbd84: lsl r7, r7, #19
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Lsl,19)?)?;
    // VMA 0xbbd88: b 0xbbdd8 <entry+0xd60>   @ imm = #0x48
    c.b(C::Al,0xbbdd8,false)?;
    // VMA 0xbbd8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbbd90: bne 0xbbd9c <entry+0xd24>   @ imm = #0x4
    c.b(C::Ne,0xbbd9c,false)?;
    // VMA 0xbbd94: lsl r7, r7, #20
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(7)?,Shift::Lsl,20)?)?;
    // VMA 0xbbd98: b 0xbbdd8 <entry+0xd60>   @ imm = #0x38
    c.b(C::Al,0xbbdd8,false)?;
    // VMA 0xbbd9c: ldr r1, [pc, #0x9e8]        @ 0x1714 <entry+0x1714>
    c.literal(R::R1,0xbc78c)?;
    // VMA 0xbbda0: ldr r2, [pc, #0x9e8]        @ 0x1718 <entry+0x1718>
    c.literal(R::R2,0xbc790)?;
    // VMA 0xbbda4: mov r3, #78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,78)?)?;
    // VMA 0xbbda8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbbdac: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // VMA 0xbbdb0: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8d124
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbbdb4: ldr r1, [pc, #0x9d8]        @ 0x171c <entry+0x171c>
    c.literal(R::R1,0xbc794)?;
    // VMA 0xbbdb8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xbbdbc: bl 0x2ec94 <entry+0xfffffffffff73c1c> @ imm = #-0x8d130
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbbdc0: ldr r2, [pc, #0x9d0]        @ 0x1720 <entry+0x1720>
    c.literal(R::R2,0xbc798)?;
    // VMA 0xbbdc4: mov r1, #79
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,79)?)?;
    // VMA 0xbbdc8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbbdcc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbbdd0: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xbbdd4: bl 0x17b0 <entry+0xfffffffffff46738> @ imm = #-0xba62c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbbdd8: ldr r12, [r6, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(6)?,732)?)?;
    // VMA 0xbbddc: mov r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // VMA 0xbbde0: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // VMA 0xbbde4: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xbbde8: mov lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // VMA 0xbbdec: orr r1, r0, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbdf0: lsl r0, r5, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsl,15)?)?;
    // VMA 0xbbdf4: and r2, r2, r12, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R12,Shift::Lsl,2,false)?)?;
    // VMA 0xbbdf8: orr r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1)?)?;
    // VMA 0xbbdfc: str r2, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    c.check(0xbbe00)
}
