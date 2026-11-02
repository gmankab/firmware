
/// Initialize codec stream packet pointers, select the indexed frame
/// event handler through a checked 41-entry jump table and update the
/// stream feedback/error state before returning to its original caller.
fn prepare_codec_stream_dispatch(c:&mut Code)->Result<(),String>{
    c.check(0xae748)?;
    // ARM 0xae748: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xae74c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xae750: sub sp, sp, #132
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,132)?)?;
    // ARM 0xae754: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xae758: mov r7, r2
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R2))?;
    // ARM 0xae75c: ldr r0, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2464)?)?;
    // ARM 0xae760: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae764: beq 0xaef40 <entry+0x27f0>
    c.b(C::Eq,0xaef40,false)?;
    // ARM 0xae768: ldr r6, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,36)?)?;
    // ARM 0xae76c: add r0, r5, #2816
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,2816)?)?;
    // ARM 0xae770: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xae774: str r3, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-44)?)?;
    // ARM 0xae778: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,64)?)?;
    // ARM 0xae77c: str r7, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-60)?)?;
    // ARM 0xae780: add r8, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xae784: ldr r9, [pc, #0xac8]
    c.literal(R::new(9)?,0xaf254)?;
    // ARM 0xae788: mvn r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,3758096384)?)?;
    // ARM 0xae78c: mov r4, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // ARM 0xae790: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // ARM 0xae794: mov r0, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1040)?)?;
    // ARM 0xae798: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae79c: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // ARM 0xae7a0: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // ARM 0xae7a4: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // ARM 0xae7a8: str r2, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-48)?)?;
    // ARM 0xae7ac: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae7b0: orr r10, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(10)?,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae7b4: mov r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,44)?)?;
    // ARM 0xae7b8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae7bc: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae7c0: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // ARM 0xae7c4: mov r0, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,116)?)?;
    // ARM 0xae7c8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae7cc: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae7d0: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // ARM 0xae7d4: mov r0, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    // ARM 0xae7d8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae7dc: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae7e0: str r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,80)?)?;
    // ARM 0xae7e4: mov r0, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2096)?)?;
    // ARM 0xae7e8: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae7ec: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // ARM 0xae7f0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xae7f4: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae7f8: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae7fc: str r0, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-64)?)?;
    // ARM 0xae800: mov r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048)?)?;
    // ARM 0xae804: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae808: str r0, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-68)?)?;
    // ARM 0xae80c: mov r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,36)?)?;
    // ARM 0xae810: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae814: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae818: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,60)?)?;
    // ARM 0xae81c: mov r0, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2144)?)?;
    // ARM 0xae820: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae824: str r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // ARM 0xae828: mov r0, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,124)?)?;
    // ARM 0xae82c: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae830: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae834: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // ARM 0xae838: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // ARM 0xae83c: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae840: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae844: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // ARM 0xae848: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // ARM 0xae84c: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae850: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae854: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // ARM 0xae858: mov r0, #988
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,988)?)?;
    // ARM 0xae85c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // ARM 0xae860: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae864: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // ARM 0xae868: mov r0, #3040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3040)?)?;
    // ARM 0xae86c: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // ARM 0xae870: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae874: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // ARM 0xae878: mov r0, #2912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2912)?)?;
    // ARM 0xae87c: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // ARM 0xae880: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae884: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // ARM 0xae888: mov r0, #868
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,868)?)?;
    // ARM 0xae88c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // ARM 0xae890: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae894: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // ARM 0xae898: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // ARM 0xae89c: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae8a0: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // ARM 0xae8a4: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM 0xae8a8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xae8ac: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xae8b0: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM 0xae8b4: rsb r0, r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(7)?,0)?)?;
    // ARM 0xae8b8: str r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,76)?)?;
    // ARM 0xae8bc: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xae8c0: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // ARM 0xae8c4: bic r0, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::new(7)?,255)?)?;
    // ARM 0xae8c8: ldr r7, [pc, #0x990]
    c.literal(R::new(7)?,0xaf260)?;
    // ARM 0xae8cc: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // ARM 0xae8d0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xae8d4: ldr r6, [r5, #0x99c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,2460)?)?;
    // ARM 0xae8d8: str r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-56)?)?;
    // ARM 0xae8dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xae8e0: str r8, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-76)?)?;
    // ARM 0xae8e4: str r0, [r11, #-0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-72)?)?;
    // ARM 0xae8e8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xae8ec: str r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-52)?)?;
    // ARM 0xae8f0: b 0xaed58 <entry+0x2608>
    c.b(C::Al,0xaed58,false)?;
    // ARM 0xae8f4: ldr r0, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-44)?)?;
    // ARM 0xae8f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae8fc: beq 0xae948 <entry+0x21f8>
    c.b(C::Eq,0xae948,false)?;
    // ARM 0xae900: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // ARM 0xae904: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae908: bne 0xae948 <entry+0x21f8>
    c.b(C::Ne,0xae948,false)?;
    // ARM 0xae90c: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // ARM 0xae910: ldr r2, [r5, #0x9cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2508)?)?;
    // ARM 0xae914: ldr r1, [r5, #0x9c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2504)?)?;
    // ARM 0xae918: add r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xae91c: sub r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xae920: ldr r2, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-60)?)?;
    // ARM 0xae924: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae928: addlt r0, r0, r2
    c.emit(arm32::data_register(C::Lt,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xae92c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xae930: ldr r2, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // ARM 0xae934: ldr r1, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,16)?)?;
    // ARM 0xae938: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xae93c: str r0, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,16)?)?;
    // ARM 0xae940: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xae944: str r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-56)?)?;
    // ARM 0xae948: ldr r1, [pc, #0x918]
    c.literal(R::R1,0xaf268)?;
    // ARM 0xae94c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xae950: mov r2, r3
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // ARM 0xae954: str r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-32)?)?;
    // ARM 0xae958: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // ARM 0xae95c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xae960: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xae964: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xae968: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xae96c: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // ARM 0xae970: cmp r1, #40
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,40)?)?;
    // ARM 0xae974: bhi 0xaec3c <entry+0x24ec>
    c.b(C::Hi,0xaec3c,false)?;
    // ARM 0xae978: lsl r0, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Lsl,2)?)?;
    // ARM 0xae97c: add r2, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::PC,0)?)?;
    // ARM 0xae980: ldr pc, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R2,Shift::Lsl,0)?)?;
    c.check(0xae984)?;
    // Forty-one original dispatch *addresses*, explicitly data not opcodes.
    const CODEC_FRAME_EVENT_JUMPS:[u32;41]=[
        0x000aea28, 0x000aea28, 0x000aea28, 0x000aea28, 0x000aea28, 0x000aea28,
        0x000aea28, 0x000aea28, 0x000aea28, 0x000aea28, 0x000aec3c, 0x000aec3c,
        0x000aec3c, 0x000aec3c, 0x000aec3c, 0x000aec3c, 0x000aea28, 0x000aea28,
        0x000aea28, 0x000aea28, 0x000aea28, 0x000aea28, 0x000aec3c, 0x000aec3c,
        0x000aec3c, 0x000aec3c, 0x000aec3c, 0x000aec3c, 0x000aec3c, 0x000aec3c,
        0x000aec3c, 0x000aec3c, 0x000aeb98, 0x000aebac, 0x000aebe0, 0x000aec3c,
        0x000aec28, 0x000aec3c, 0x000aec3c, 0x000aeb74, 0x000aeb74,
    ];
    for address in CODEC_FRAME_EVENT_JUMPS {c.emit(address)?;}
    c.check(0xaea28)?;
    // ARM 0xaea28: mov r9, r7
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(7)?))?;
    // ARM 0xaea2c: ldr r7, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-40)?)?;
    // ARM 0xaea30: ldr r0, [r7, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaea34: ldr r2, [r6, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(6)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaea38: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaea3c: str r0, [r5, #0x9a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2472)?)?;
    // ARM 0xaea40: ldr r0, [r11, #-0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-72)?)?;
    // ARM 0xaea44: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // ARM 0xaea48: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xaea4c: bl 0xb5008 <entry+0x88b8>
    c.b(C::Al,0xb5008,true)?;
    // ARM 0xaea50: ldr r0, [r5, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2432)?)?;
    // ARM 0xaea54: cmp r0, #19
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,19)?)?;
    // ARM 0xaea58: beq 0xaef48 <entry+0x27f8>
    c.b(C::Eq,0xaef48,false)?;
    // ARM 0xaea5c: ldr r3, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-68)?)?;
    // ARM 0xaea60: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xaea64: str r0, [r3, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaea68: ldr r2, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-64)?)?;
    // ARM 0xaea6c: ldr r0, [r2, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaea70: ldr r0, [r7, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaea74: ldr r1, [r6, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaea78: mov r7, r9
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // ARM 0xaea7c: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaea80: ldr r1, [r5, #0x9a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2472)?)?;
    // ARM 0xaea84: sub r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaea88: ands r1, r0, #7
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R0,7,true)?)?;
    // ARM 0xaea8c: str r0, [r5, #0x9a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2472)?)?;
    // ARM 0xaea90: rsbne r0, r1, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Rsb,R::R0,R::R1,8)?)?;
    // ARM 0xaea94: strne r0, [r3, r4]
    c.emit(arm32::transfer_word_register(C::Ne,false,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaea98: ldrne r0, [r2, r4]
    c.emit(arm32::transfer_word_register(C::Ne,true,R::R0,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaea9c: ldr r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-36)?)?;
    // ARM 0xaeaa0: ldr r3, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-60)?)?;
    // ARM 0xaeaa4: ldrne r0, [r5, #0x9a8]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,2472)?)?;
    // ARM 0xaeaa8: add r0, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7)?)?;
    // ARM 0xaeaac: bic r1, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R0,7)?)?;
    // ARM 0xaeab0: add r0, r8, r0, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R0,Shift::Lsr,3,false)?)?;
    // ARM 0xaeab4: ldr r8, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-76)?)?;
    // ARM 0xaeab8: add r2, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaeabc: cmp r2, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaeac0: subhs r2, r2, r3
    c.emit(arm32::data_register(C::Cs,Op::Sub,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaeac4: str r2, [r5, #0x9a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,2468)?)?;
    // ARM 0xaeac8: str r1, [r5, #0x9a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,2472)?)?;
    // ARM 0xaeacc: ldr r1, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,24)?)?;
    // ARM 0xaead0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xaead4: ldr r1, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-52)?)?;
    // ARM 0xaead8: movne r0, r1
    c.emit(arm32::mov_register(C::Ne,R::R0,R::R1))?;
    // ARM 0xaeadc: ldr r1, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2464)?)?;
    // ARM 0xaeae0: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaeae4: blo 0xaf088 <entry+0x2938>
    c.b(C::Cc,0xaf088,false)?;
    // ARM 0xaeae8: ldr r3, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,76)?)?;
    // ARM 0xaeaec: add r2, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaeaf0: cmp r8, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xaeaf4: addls r2, r2, r3
    c.emit(arm32::data_register(C::Ls,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaeaf8: subs r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,true)?)?;
    // ARM 0xaeafc: mvneq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mvn,R::R0,R::R0,0)?)?;
    // ARM 0xaeb00: str r2, [r5, #0x99c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,2460)?)?;
    // ARM 0xaeb04: str r0, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2464)?)?;
    // ARM 0xaeb08: ldr r2, [r5, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2432)?)?;
    // ARM 0xaeb0c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaeb10: beq 0xaef58 <entry+0x2808>
    c.b(C::Eq,0xaef58,false)?;
    // ARM 0xaeb14: ldr r1, [pc, #0x768]
    c.literal(R::R1,0xaf284)?;
    // ARM 0xaeb18: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaeb1c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaeb20: ldr r9, [pc, #0x72c]
    c.literal(R::new(9)?,0xaf254)?;
    // ARM 0xaeb24: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaeb28: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xaeb2c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaeb30: ldr r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-48)?)?;
    // ARM 0xaeb34: ldr r6, [r5, #0x99c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,2460)?)?;
    // ARM 0xaeb38: ldr r0, [r7, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,2)?)?;
    // ARM 0xaeb3c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaeb40: bne 0xaed14 <entry+0x25c4>
    c.b(C::Ne,0xaed14,false)?;
    // ARM 0xaeb44: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // ARM 0xaeb48: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM 0xaeb4c: str r1, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaeb50: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xaeb54: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // ARM 0xaeb58: str r1, [r0, r4]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaeb5c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xaeb60: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // ARM 0xaeb64: ldr r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaeb68: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // ARM 0xaeb6c: beq 0xaeb64 <entry+0x2414>
    c.b(C::Eq,0xaeb64,false)?;
    // ARM 0xaeb70: b 0xaed20 <entry+0x25d0>
    c.b(C::Al,0xaed20,false)?;
    // ARM 0xaeb74: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // ARM 0xaeb78: str r0, [r5, #0x99c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2460)?)?;
    // ARM 0xaeb7c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xaeb80: bl 0xaf6e0 <entry+0x2f90>
    c.b(C::Al,0xaf6e0,true)?;
    // ARM 0xaeb84: ldr r2, [r5, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2432)?)?;
    // ARM 0xaeb88: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaeb8c: beq 0xaec3c <entry+0x24ec>
    c.b(C::Eq,0xaec3c,false)?;
    // ARM 0xaeb90: ldr r1, [pc, #0x6d4]
    c.literal(R::R1,0xaf26c)?;
    // ARM 0xaeb94: b 0xaec10 <entry+0x24c0>
    c.b(C::Al,0xaec10,false)?;
    // ARM 0xaeb98: ldr r2, [r5, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2432)?)?;
    // ARM 0xaeb9c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaeba0: beq 0xaec3c <entry+0x24ec>
    c.b(C::Eq,0xaec3c,false)?;
    // ARM 0xaeba4: ldr r1, [pc, #0x6d4]
    c.literal(R::R1,0xaf280)?;
    // ARM 0xaeba8: b 0xaec10 <entry+0x24c0>
    c.b(C::Al,0xaec10,false)?;
    // ARM 0xaebac: ldr r0, [pc, #0x6c4]
    c.literal(R::R0,0xaf278)?;
    // ARM 0xaebb0: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // ARM 0xaebb4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xaebb8: blx r1
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    // ARM 0xaebbc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaebc0: bne 0xaec3c <entry+0x24ec>
    c.b(C::Ne,0xaec3c,false)?;
    // ARM 0xaebc4: ldr r2, [r5, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2432)?)?;
    // ARM 0xaebc8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaebcc: beq 0xaec3c <entry+0x24ec>
    c.b(C::Eq,0xaec3c,false)?;
    // ARM 0xaebd0: cmp r2, #56
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,56)?)?;
    // ARM 0xaebd4: beq 0xaf064 <entry+0x2914>
    c.b(C::Eq,0xaf064,false)?;
    // ARM 0xaebd8: ldr r1, [pc, #0x69c]
    c.literal(R::R1,0xaf27c)?;
    // ARM 0xaebdc: b 0xaec10 <entry+0x24c0>
    c.b(C::Al,0xaec10,false)?;
    // ARM 0xaebe0: ldr r0, [pc, #0x688]
    c.literal(R::R0,0xaf270)?;
    // ARM 0xaebe4: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // ARM 0xaebe8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xaebec: blx r1
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    // ARM 0xaebf0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaebf4: bne 0xaec3c <entry+0x24ec>
    c.b(C::Ne,0xaec3c,false)?;
    // ARM 0xaebf8: ldr r2, [r5, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2432)?)?;
    // ARM 0xaebfc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaec00: beq 0xaec3c <entry+0x24ec>
    c.b(C::Eq,0xaec3c,false)?;
    // ARM 0xaec04: cmp r2, #56
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,56)?)?;
    // ARM 0xaec08: beq 0xaf064 <entry+0x2914>
    c.b(C::Eq,0xaf064,false)?;
    // ARM 0xaec0c: ldr r1, [pc, #0x660]
    c.literal(R::R1,0xaf274)?;
    // ARM 0xaec10: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaec14: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaec18: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaec1c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xaec20: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaec24: b 0xaec3c <entry+0x24ec>
    c.b(C::Al,0xaec3c,false)?;
    // ARM 0xaec28: ldr r0, [r5, #0x308]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,776)?)?;
    // ARM 0xaec2c: orr r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // ARM 0xaec30: str r0, [r5, #0x308]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,776)?)?;
    // ARM 0xaec34: mvn r0, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,2147483648)?)?;
    // ARM 0xaec38: str r0, [r5, #0x9b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2488)?)?;
    // ARM 0xaec3c: ldr r0, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,24)?)?;
    // ARM 0xaec40: ldr r7, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-52)?)?;
    // ARM 0xaec44: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaec48: bne 0xaecb4 <entry+0x2564>
    c.b(C::Ne,0xaecb4,false)?;
    // ARM 0xaec4c: ldr r2, [r5, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2432)?)?;
    // ARM 0xaec50: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaec54: beq 0xaec9c <entry+0x254c>
    c.b(C::Eq,0xaec9c,false)?;
    // ARM 0xaec58: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // ARM 0xaec5c: ldr r0, [r1, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaec60: ldr r3, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-64)?)?;
    // ARM 0xaec64: tst r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // ARM 0xaec68: bne 0xaec7c <entry+0x252c>
    c.b(C::Ne,0xaec7c,false)?;
    // ARM 0xaec6c: ldr r0, [r3, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaec70: ldr r0, [r1, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaec74: tst r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,4)?)?;
    // ARM 0xaec78: beq 0xaec6c <entry+0x251c>
    c.b(C::Eq,0xaec6c,false)?;
    // ARM 0xaec7c: ldr r1, [pc, #0x604]
    c.literal(R::R1,0xaf288)?;
    // ARM 0xaec80: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaec84: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaec88: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaec8c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xaec90: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaec94: mov r7, r8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // ARM 0xaec98: b 0xaecb4 <entry+0x2564>
    c.b(C::Al,0xaecb4,false)?;
    // ARM 0xaec9c: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // ARM 0xaeca0: ldr r0, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaeca4: ldr r1, [r6, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaeca8: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaecac: add r0, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7)?)?;
    // ARM 0xaecb0: asr r7, r0, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R0,Shift::Asr,3)?)?;
    // ARM 0xaecb4: ldr r0, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2464)?)?;
    // ARM 0xaecb8: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaecbc: bhi 0xaf07c <entry+0x292c>
    c.b(C::Hi,0xaf07c,false)?;
    // ARM 0xaecc0: ldr r1, [pc, #0x5c4]
    c.literal(R::R1,0xaf28c)?;
    // ARM 0xaecc4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xaecc8: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // ARM 0xaeccc: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaecd0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xaecd4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xaecd8: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaecdc: ldr r0, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2464)?)?;
    // ARM 0xaece0: sub r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaece4: str r0, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2464)?)?;
    // ARM 0xaece8: ldr r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-36)?)?;
    // ARM 0xaecec: ldr r8, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-76)?)?;
    // ARM 0xaecf0: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,76)?)?;
    // ARM 0xaecf4: add r6, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaecf8: ldr r7, [pc, #0x560]
    c.literal(R::new(7)?,0xaf260)?;
    // ARM 0xaecfc: cmp r8, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaed00: addls r6, r6, r0
    c.emit(arm32::data_register(C::Ls,Op::Add,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaed04: ldr r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-48)?)?;
    // ARM 0xaed08: ldr r0, [r7, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,2)?)?;
    // ARM 0xaed0c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaed10: beq 0xaed28 <entry+0x25d8>
    c.b(C::Eq,0xaed28,false)?;
    // ARM 0xaed14: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // ARM 0xaed18: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xaed1c: str r1, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaed20: ldr r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // ARM 0xaed24: b 0xaed58 <entry+0x2608>
    c.b(C::Al,0xaed58,false)?;
    // ARM 0xaed28: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,72)?)?;
    // ARM 0xaed2c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM 0xaed30: str r1, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaed34: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xaed38: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // ARM 0xaed3c: str r1, [r0, r4]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaed40: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xaed44: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // ARM 0xaed48: ldr r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // ARM 0xaed4c: ldr r0, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaed50: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // ARM 0xaed54: beq 0xaed4c <entry+0x25fc>
    c.b(C::Eq,0xaed4c,false)?;
    // ARM 0xaed58: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xaed5c: str r0, [r5, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2432)?)?;
    // ARM 0xaed60: ldr r0, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,24)?)?;
    // ARM 0xaed64: str r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-36)?)?;
    // ARM 0xaed68: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaed6c: beq 0xaedb4 <entry+0x2664>
    c.b(C::Eq,0xaedb4,false)?;
    // ARM 0xaed70: ldr r1, [pc, #0x4d4]
    c.literal(R::R1,0xaf24c)?;
    // ARM 0xaed74: ldr r2, [pc, #0x4d4]
    c.literal(R::R2,0xaf250)?;
    // ARM 0xaed78: mov r6, r3
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // ARM 0xaed7c: mov r3, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,166)?)?;
    // ARM 0xaed80: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xaed84: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // ARM 0xaed88: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaed8c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xaed90: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xaed94: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaed98: ldr r2, [pc, #0x4b8]
    c.literal(R::R2,0xaf258)?;
    // ARM 0xaed9c: mov r1, #167
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,167)?)?;
    // ARM 0xaeda0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xaeda4: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xaeda8: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xaedac: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // ARM 0xaedb0: b 0xaef14 <entry+0x27c4>
    c.b(C::Al,0xaef14,false)?;
    // ARM 0xaedb4: ldr r0, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2464)?)?;
    // ARM 0xaedb8: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // ARM 0xaedbc: blo 0xaf088 <entry+0x2938>
    c.b(C::Cc,0xaf088,false)?;
    // ARM 0xaedc0: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // ARM 0xaedc4: beq 0xaf1e8 <entry+0x2a98>
    c.b(C::Eq,0xaf1e8,false)?;
    // ARM 0xaedc8: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // ARM 0xaedcc: mov r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3072)?)?;
    // ARM 0xaedd0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM 0xaedd4: str r1, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaedd8: mov r1, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,48)?)?;
    // ARM 0xaeddc: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // ARM 0xaede0: ldr r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // ARM 0xaede4: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // ARM 0xaede8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xaedec: moveq r1, #4224
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,4224)?)?;
    // ARM 0xaedf0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xaedf4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xaedf8: str r1, [r2, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaedfc: moveq r0, #9
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,9)?)?;
    // ARM 0xaee00: ldr r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,64)?)?;
    // ARM 0xaee04: ldr r3, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,36)?)?;
    // ARM 0xaee08: str r2, [r3, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaee0c: ldr r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,40)?)?;
    // ARM 0xaee10: str r1, [r2, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaee14: ldr r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // ARM 0xaee18: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // ARM 0xaee1c: str r2, [r1, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaee20: ldr r1, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2464)?)?;
    // ARM 0xaee24: ldr r2, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,48)?)?;
    // ARM 0xaee28: str r1, [r2, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaee2c: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // ARM 0xaee30: ldr r2, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,52)?)?;
    // ARM 0xaee34: add r1, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaee38: str r1, [r2, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaee3c: mvn r2, #-1946157056
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,2348810240)?)?;
    // ARM 0xaee40: ldr r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // ARM 0xaee44: str r2, [r1, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaee48: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // ARM 0xaee4c: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,56)?)?;
    // ARM 0xaee50: str r0, [r1, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaee54: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xaee58: ldr r2, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,60)?)?;
    // ARM 0xaee5c: moveq r0, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM 0xaee60: orr r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,1)?)?;
    // ARM 0xaee64: str r1, [r2, r4]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaee68: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // ARM 0xaee6c: ldr r0, [pc, #0x3e8]
    c.literal(R::R0,0xaf25c)?;
    // ARM 0xaee70: str r0, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,56)?)?;
    // ARM 0xaee74: ldr r0, [r5, #0x354]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,852)?)?;
    // ARM 0xaee78: bl 0x293c <entry+0xfffffffffff561ec>
    c.b(C::Al,0x293c,true)?;
    // ARM 0xaee7c: bl 0x3070 <entry+0xfffffffffff56920>
    c.b(C::Al,0x3070,true)?;
    // ARM 0xaee80: ldr r9, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-68)?)?;
    // ARM 0xaee84: mov r0, #117440512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,117440512)?)?;
    // ARM 0xaee88: str r0, [r9, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(9)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaee8c: ldr r0, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,48)?)?;
    // ARM 0xaee90: ldr r6, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-48)?)?;
    // ARM 0xaee94: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaee98: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xaee9c: ldrne r7, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(7)?,R::new(5)?,56)?)?;
    // ARM 0xaeea0: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xaeea4: bl 0x26658 <entry+0xfffffffffff79f08>
    c.b(C::Al,0x26658,true)?;
    // ARM 0xaeea8: bl 0x3074 <entry+0xfffffffffff56924>
    c.b(C::Al,0x3074,true)?;
    // ARM 0xaeeac: ldr r0, [r5, #0x354]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,852)?)?;
    // ARM 0xaeeb0: bl 0x294c <entry+0xfffffffffff561fc>
    c.b(C::Al,0x294c,true)?;
    // ARM 0xaeeb4: ldr r7, [pc, #0x3a4]
    c.literal(R::new(7)?,0xaf260)?;
    // ARM 0xaeeb8: ldr r0, [r7, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::new(6)?,Shift::Lsl,2)?)?;
    // ARM 0xaeebc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaeec0: bne 0xaf030 <entry+0x28e0>
    c.b(C::Ne,0xaf030,false)?;
    // ARM 0xaeec4: ldr r1, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-64)?)?;
    // ARM 0xaeec8: ldr r0, [r1, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaeecc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaeed0: str r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-52)?)?;
    // ARM 0xaeed4: blt 0xaf06c <entry+0x291c>
    c.b(C::Lt,0xaf06c,false)?;
    // ARM 0xaeed8: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // ARM 0xaeedc: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // ARM 0xaeee0: ldr r9, [pc, #0x36c]
    c.literal(R::new(9)?,0xaf254)?;
    // ARM 0xaeee4: str r2, [r0, r4]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaeee8: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // ARM 0xaeeec: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // ARM 0xaeef0: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // ARM 0xaeef4: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // ARM 0xaeef8: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // ARM 0xaeefc: ldr r1, [r0, r4]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaef00: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // ARM 0xaef04: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // ARM 0xaef08: and r3, r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R1,63)?)?;
    // ARM 0xaef0c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // ARM 0xaef10: str r0, [r11, #-0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-72)?)?;
    // ARM 0xaef14: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // ARM 0xaef18: ldr r0, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaef1c: ldr r6, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,80)?)?;
    // ARM 0xaef20: ldr r1, [r6, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::new(4)?,Shift::Lsl,0)?)?;
    // ARM 0xaef24: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaef28: add r0, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,7)?)?;
    // ARM 0xaef2c: asr r8, r0, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::R0,Shift::Asr,3)?)?;
    // ARM 0xaef30: ldr r0, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2464)?)?;
    // ARM 0xaef34: cmp r8, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaef38: bls 0xae8f4 <entry+0x21a4>
    c.b(C::Ls,0xae8f4,false)?;
    // ARM 0xaef3c: b 0xaf080 <entry+0x2930>
    c.b(C::Al,0xaf080,false)?;
    // ARM 0xaef40: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xaef44: b 0xaf1c8 <entry+0x2a78>
    c.b(C::Al,0xaf1c8,false)?;
    // ARM 0xaef48: ldr r8, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-76)?)?;
    // ARM 0xaef4c: ldr r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-36)?)?;
    // ARM 0xaef50: mov r7, r9
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // ARM 0xaef54: b 0xaf088 <entry+0x2938>
    c.b(C::Al,0xaf088,false)?;
    // ARM 0xaef58: ldrb r1, [r5, #0xb06]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2822)?)?;
    // ARM 0xaef5c: ldrb r2, [r5, #0xa20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2592)?)?;
    // ARM 0xaef60: ldrb r0, [r5, #0xa26]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,2598)?)?;
    // ARM 0xaef64: ldrb r3, [r5, #0xa21]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(5)?,2593)?)?;
    // ARM 0xaef68: ldrb r7, [r5, #0xa24]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(5)?,2596)?)?;
    // ARM 0xaef6c: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xaef70: ldrb r1, [r5, #0xb08]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2824)?)?;
    // ARM 0xaef74: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xaef78: ldrb r2, [r5, #0xb0a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2826)?)?;
    // ARM 0xaef7c: orrne r4, r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xaef80: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaef84: ldrb r1, [r5, #0xb09]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2825)?)?;
    // ARM 0xaef88: orrne r4, r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xaef8c: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaef90: ldrb r1, [r5, #0xa28]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2600)?)?;
    // ARM 0xaef94: orrne r4, r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xaef98: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaef9c: ldrb r1, [r5, #0xa31]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2609)?)?;
    // ARM 0xaefa0: ldrb r2, [r5, #0xb0c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2828)?)?;
    // ARM 0xaefa4: orrne r4, r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xaefa8: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaefac: ldr r1, [r5, #0xa34]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2612)?)?;
    // ARM 0xaefb0: ldr r2, [r5, #0xb10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2832)?)?;
    // ARM 0xaefb4: orrne r4, r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xaefb8: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaefbc: ldrb r1, [r5, #0xa38]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2616)?)?;
    // ARM 0xaefc0: ldrb r2, [r5, #0xb14]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2836)?)?;
    // ARM 0xaefc4: orrne r4, r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xaefc8: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaefcc: ldrb r1, [r5, #0xa7e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,2686)?)?;
    // ARM 0xaefd0: ldrb r2, [r5, #0xb17]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2839)?)?;
    // ARM 0xaefd4: orrne r4, r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xaefd8: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaefdc: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // ARM 0xaefe0: orrne r4, r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xaefe4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xaefe8: beq 0xaf1d0 <entry+0x2a80>
    c.b(C::Eq,0xaf1d0,false)?;
    // ARM 0xaefec: ldr r7, [pc, #0x26c]
    c.literal(R::new(7)?,0xaf260)?;
    // ARM 0xaeff0: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // ARM 0xaeff4: bne 0xaf088 <entry+0x2938>
    c.b(C::Ne,0xaf088,false)?;
    // ARM 0xaeff8: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // ARM 0xaeffc: ldrb r2, [r5, #0xb05]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2821)?)?;
    // ARM 0xaf000: ldrh r1, [r0]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::R0,0)?)?;
    // ARM 0xaf004: ldr r0, [r5, #0xa2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2604)?)?;
    // ARM 0xaf008: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaf00c: beq 0xaf028 <entry+0x28d8>
    c.b(C::Eq,0xaf028,false)?;
    // ARM 0xaf010: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // ARM 0xaf014: orr r2, r2, #2816
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2816)?)?;
    // ARM 0xaf018: ldrh r2, [r5, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R2,R::new(5)?,R::R2)?)?;
    // ARM 0xaf01c: mul r3, r2, r1
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R1)?)?;
    // ARM 0xaf020: cmp r0, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaf024: bne 0xaf1f4 <entry+0x2aa4>
    c.b(C::Ne,0xaf1f4,false)?;
    // ARM 0xaf028: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // ARM 0xaf02c: b 0xaf20c <entry+0x2abc>
    c.b(C::Al,0xaf20c,false)?;
    // ARM 0xaf030: ldr r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-48)?)?;
    // ARM 0xaf034: ldr r1, [pc, #0x228]
    c.literal(R::R1,0xaf264)?;
    // ARM 0xaf038: ldr r2, [pc, #0x210]
    c.literal(R::R2,0xaf250)?;
    // ARM 0xaf03c: mov r3, #97
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,97)?)?;
    // ARM 0xaf040: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // ARM 0xaf044: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xaf048: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM 0xaf04c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaf050: ldr r1, [pc, #0x1fc]
    c.literal(R::R1,0xaf254)?;
    // ARM 0xaf054: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM 0xaf058: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaf05c: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // ARM 0xaf060: str r0, [r5, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2432)?)?;
    // ARM 0xaf064: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xaf068: b 0xaf174 <entry+0x2a24>
    c.b(C::Al,0xaf174,false)?;
    // ARM 0xaf06c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xaf070: mov r1, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,268435456)?)?;
    // ARM 0xaf074: str r1, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(8)?,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xaf078: b 0xaf080 <entry+0x2930>
    c.b(C::Al,0xaf080,false)?;
    // ARM 0xaf07c: ldr r7, [pc, #0x1dc]
    c.literal(R::new(7)?,0xaf260)?;
    // ARM 0xaf080: ldr r8, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,-76)?)?;
    // ARM 0xaf084: ldr r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-36)?)?;
    // ARM 0xaf088: ldr r1, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2464)?)?;
    // ARM 0xaf08c: ldr r4, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-60)?)?;
    // ARM 0xaf090: add r0, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaf094: cmp r8, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaf098: subls r0, r0, r4
    c.emit(arm32::data_register(C::Ls,Op::Sub,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaf09c: str r0, [r5, #0x9a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2468)?)?;
    // ARM 0xaf0a0: ldr r3, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-44)?)?;
    // ARM 0xaf0a4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xaf0a8: beq 0xaf164 <entry+0x2a14>
    c.b(C::Eq,0xaf164,false)?;
    // ARM 0xaf0ac: ldr r1, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-56)?)?;
    // ARM 0xaf0b0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xaf0b4: bne 0xaf0dc <entry+0x298c>
    c.b(C::Ne,0xaf0dc,false)?;
    // ARM 0xaf0b8: ldr r2, [r5, #0x9cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,2508)?)?;
    // ARM 0xaf0bc: ldr r1, [r5, #0x9c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2504)?)?;
    // ARM 0xaf0c0: sub r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xaf0c4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaf0c8: addlt r0, r0, r4
    c.emit(arm32::data_register(C::Lt,Op::Add,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaf0cc: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xaf0d0: ldr r1, [r3, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,16)?)?;
    // ARM 0xaf0d4: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaf0d8: str r0, [r3, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,16)?)?;
    // ARM 0xaf0dc: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // ARM 0xaf0e0: ldrb r2, [r5, #0xb05]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2821)?)?;
    // ARM 0xaf0e4: ldrh r1, [r0]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::R0,0)?)?;
    // ARM 0xaf0e8: ldr r0, [r3, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,128)?)?;
    // ARM 0xaf0ec: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaf0f0: beq 0xaf10c <entry+0x29bc>
    c.b(C::Eq,0xaf10c,false)?;
    // ARM 0xaf0f4: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // ARM 0xaf0f8: orr r2, r2, #2816
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2816)?)?;
    // ARM 0xaf0fc: ldrh r2, [r5, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R2,R::new(5)?,R::R2)?)?;
    // ARM 0xaf100: mul r3, r2, r1
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R1)?)?;
    // ARM 0xaf104: cmp r0, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaf108: bne 0xaf114 <entry+0x29c4>
    c.b(C::Ne,0xaf114,false)?;
    // ARM 0xaf10c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // ARM 0xaf110: b 0xaf12c <entry+0x29dc>
    c.b(C::Al,0xaf12c,false)?;
    // ARM 0xaf114: ldr r2, [r5, #0x4a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,1192)?)?;
    // ARM 0xaf118: ldr r3, [r5, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,1196)?)?;
    // ARM 0xaf11c: add r0, r2, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xaf120: ldrh r0, [r0]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::R0,0)?)?;
    // ARM 0xaf124: add r0, r3, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xaf128: ldrh r0, [r0, #-2]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::R0,-2)?)?;
    // ARM 0xaf12c: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xaf130: ldrb r2, [r5, #0xb04]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2820)?)?;
    // ARM 0xaf134: mul r3, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // ARM 0xaf138: mov r1, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1020)?)?;
    // ARM 0xaf13c: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    // ARM 0xaf140: and r1, r1, r3, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsr,1,false)?)?;
    // ARM 0xaf144: mul r3, r0, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R0,R::R2)?)?;
    // ARM 0xaf148: mov r0, #1044480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1044480)?)?;
    // ARM 0xaf14c: orr r0, r0, #3145728
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3145728)?)?;
    // ARM 0xaf150: and r0, r0, r3, lsl #9
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,9,false)?)?;
    // ARM 0xaf154: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaf158: ldr r1, [r5, #0x9d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2512)?)?;
    // ARM 0xaf15c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xaf160: ldr r1, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2464)?)?;
    // ARM 0xaf164: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xaf168: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // ARM 0xaf16c: str r0, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2464)?)?;
    // ARM 0xaf170: blo 0xaf1c8 <entry+0x2a78>
    c.b(C::Cc,0xaf1c8,false)?;
    // ARM 0xaf174: ldr r1, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // ARM 0xaf178: ldr r1, [r7, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(7)?,R::R1,Shift::Lsl,2)?)?;
    // ARM 0xaf17c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xaf180: beq 0xaf198 <entry+0x2a48>
    c.b(C::Eq,0xaf198,false)?;
    // ARM 0xaf184: ldr r3, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,68)?)?;
    // ARM 0xaf188: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xaf18c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM 0xaf190: str r2, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xaf194: b 0xaf1c8 <entry+0x2a78>
    c.b(C::Al,0xaf1c8,false)?;
    // ARM 0xaf198: ldr r3, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,72)?)?;
    // ARM 0xaf19c: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xaf1a0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // ARM 0xaf1a4: str r2, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xaf1a8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM 0xaf1ac: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // ARM 0xaf1b0: str r2, [r3, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xaf1b4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xaf1b8: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // ARM 0xaf1bc: ldr r2, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(10)?,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xaf1c0: tst r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,2)?)?;
    // ARM 0xaf1c4: beq 0xaf1bc <entry+0x2a6c>
    c.b(C::Eq,0xaf1bc,false)?;
    // ARM 0xaf1c8: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xaf1cc: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xaf1d0: ldr r7, [pc, #0x88]
    c.literal(R::new(7)?,0xaf260)?;
    // ARM 0xaf1d4: ldr r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,4)?)?;
    // ARM 0xaf1d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaf1dc: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xaf1e0: streq r0, [r5, #0xa2c]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(5)?,2604)?)?;
    // ARM 0xaf1e4: b 0xaf244 <entry+0x2af4>
    c.b(C::Al,0xaf244,false)?;
    // ARM 0xaf1e8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xaf1ec: str r0, [r5, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2464)?)?;
    // ARM 0xaf1f0: b 0xaf088 <entry+0x2938>
    c.b(C::Al,0xaf088,false)?;
    // ARM 0xaf1f4: ldr r2, [r5, #0x4a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,1192)?)?;
    // ARM 0xaf1f8: ldr r3, [r5, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,1196)?)?;
    // ARM 0xaf1fc: add r0, r2, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xaf200: ldrh r0, [r0]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::R0,0)?)?;
    // ARM 0xaf204: add r0, r3, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xaf208: ldrh r0, [r0, #-2]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::R0,-2)?)?;
    // ARM 0xaf20c: ldr r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,4)?)?;
    // ARM 0xaf210: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xaf214: ldrb r2, [r5, #0xb04]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,2820)?)?;
    // ARM 0xaf218: mul r3, r1, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::R2)?)?;
    // ARM 0xaf21c: mov r1, #1020
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1020)?)?;
    // ARM 0xaf220: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    // ARM 0xaf224: and r1, r1, r3, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsr,1,false)?)?;
    // ARM 0xaf228: mul r3, r0, r2
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R0,R::R2)?)?;
    // ARM 0xaf22c: mov r0, #1044480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1044480)?)?;
    // ARM 0xaf230: orr r0, r0, #3145728
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3145728)?)?;
    // ARM 0xaf234: and r0, r0, r3, lsl #9
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,9,false)?)?;
    // ARM 0xaf238: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaf23c: ldr r1, [r5, #0x9d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2512)?)?;
    // ARM 0xaf240: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xaf244: orr r0, r4, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaf248: b 0xaf174 <entry+0x2a24>
    c.b(C::Al,0xaf174,false)?;
    c.check(0xaf24c)?;
    const STREAM_DISPATCH_DIAGNOSTICS:[u32;17]=[
        0x000f6cb9, 0x000f868a, 0x000e2788, 0x000f8519, 0x05f5e100, 0x0010408c,
        0x000f866c, 0x000f869c, 0x000f6e04, 0x0010dd84, 0x000f6df1, 0x0010dd8c,
        0x000f6dde, 0x000f86c0, 0x000f6d30, 0x000f86d3, 0x000f870c,
    ];
    for word in STREAM_DISPATCH_DIAGNOSTICS {c.emit(word)?;}
    c.check(0xaf290)
}
