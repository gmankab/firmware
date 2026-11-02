
/// Publish codec buffer geometry and per-plane synchronization registers.
fn write_hardware_codec_profile_plane_rows(c:&mut Code)->Result<(),String>{
    c.check(0xbc970)?;
    // VMA 0xbc970: mov r1, #428
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,428)?)?;
    // VMA 0xbc974: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbc978: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc97c: str r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc980: mov r1, #2480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2480)?)?;
    // VMA 0xbc984: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xbc988: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc98c: str r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc990: mov r1, #436
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,436)?)?;
    // VMA 0xbc994: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbc998: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc99c: str r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc9a0: mov r1, #244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,244)?)?;
    // VMA 0xbc9a4: ldr r2, [pc, #0x634]        @ 0x804 <entry+0x804>
    c.literal(R::R2,0xbcfe0)?;
    // VMA 0xbc9a8: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbc9ac: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc9b0: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc9b4: mov r1, #236
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,236)?)?;
    // VMA 0xbc9b8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbc9bc: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbc9c0: orr r2, r2, #33816576
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,33816576)?)?;
    // VMA 0xbc9c4: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc9c8: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc9cc: mov r1, #2288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2288)?)?;
    // VMA 0xbc9d0: ldr r2, [pc, #0x60c]        @ 0x808 <entry+0x808>
    c.literal(R::R2,0xbcfe4)?;
    // VMA 0xbc9d4: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xbc9d8: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc9dc: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc9e0: mov r1, #212
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,212)?)?;
    // VMA 0xbc9e4: ldr r2, [pc, #0x5fc]        @ 0x80c <entry+0x80c>
    c.literal(R::R2,0xbcfe8)?;
    // VMA 0xbc9e8: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbc9ec: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc9f0: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc9f4: mov r1, #216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,216)?)?;
    // VMA 0xbc9f8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbc9fc: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbca00: str r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-32)?)?;
    // VMA 0xbca04: mov r2, #352
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,352)?)?;
    // VMA 0xbca08: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbca0c: orr r2, r2, #5767168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5767168)?)?;
    // VMA 0xbca10: str r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbca14: mov r1, #220
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,220)?)?;
    // VMA 0xbca18: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbca1c: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbca20: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbca24: mov r1, #2272
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2272)?)?;
    // VMA 0xbca28: ldr r2, [pc, #0x5bc]        @ 0x810 <entry+0x810>
    c.literal(R::R2,0xbcfec)?;
    // VMA 0xbca2c: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xbca30: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbca34: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbca38: mov r1, #228
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,228)?)?;
    // VMA 0xbca3c: mov r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,11)?)?;
    // VMA 0xbca40: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbca44: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbca48: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbca4c: mov r1, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,56)?)?;
    // VMA 0xbca50: ldr r2, [r9, #0x428]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1064)?)?;
    // VMA 0xbca54: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // VMA 0xbca58: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbca5c: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbca60: mov r1, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,60)?)?;
    // VMA 0xbca64: ldr r2, [r9, #0x42c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1068)?)?;
    // VMA 0xbca68: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // VMA 0xbca6c: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbca70: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbca74: mov r1, #2080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2080)?)?;
    // VMA 0xbca78: mov r2, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,262144)?)?;
    // VMA 0xbca7c: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // VMA 0xbca80: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbca84: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbca88: mov r1, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,44)?)?;
    // VMA 0xbca8c: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // VMA 0xbca90: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbca94: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbca98: mov r1, #152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,152)?)?;
    // VMA 0xbca9c: mov r2, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4194304)?)?;
    // VMA 0xbcaa0: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // VMA 0xbcaa4: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbcaa8: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbcaac: mov r1, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,156)?)?;
    // VMA 0xbcab0: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // VMA 0xbcab4: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbcab8: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbcabc: mov r1, #188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,188)?)?;
    // VMA 0xbcac0: mov r2, #92274688
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,92274688)?)?;
    // VMA 0xbcac4: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // VMA 0xbcac8: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbcacc: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbcad0: mov r1, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,180)?)?;
    // VMA 0xbcad4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbcad8: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // VMA 0xbcadc: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbcae0: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbcae4: mov r1, #2064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2064)?)?;
    // VMA 0xbcae8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbcaec: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // VMA 0xbcaf0: orr r2, r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,16384)?)?;
    // VMA 0xbcaf4: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbcaf8: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbcafc: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // VMA 0xbcb00: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xbcb04: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // VMA 0xbcb08: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbcb0c: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbcb10: mov r1, #4080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4080)?)?;
    // VMA 0xbcb14: orr r1, r1, #61440
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,61440)?)?;
    // VMA 0xbcb18: and r1, r1, r8, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(8)?,Shift::Lsl,4,false)?)?;
    // VMA 0xbcb1c: orr r2, r1, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(5)?,Shift::Lsl,16,false)?)?;
    // VMA 0xbcb20: mov r1, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28)?)?;
    // VMA 0xbcb24: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // VMA 0xbcb28: str r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,28)?)?;
    // VMA 0xbcb2c: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbcb30: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbcb34: mov r2, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16711680)?)?;
    // VMA 0xbcb38: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // VMA 0xbcb3c: orr r2, r2, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,16777216)?)?;
    // VMA 0xbcb40: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // VMA 0xbcb44: and r2, r2, r8, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(8)?,Shift::Lsl,16,false)?)?;
    // VMA 0xbcb48: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbcb4c: orr r2, r2, #18
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,18)?)?;
    // VMA 0xbcb50: orr r2, r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,256)?)?;
    // VMA 0xbcb54: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbcb58: mov r1, #4080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4080)?)?;
    // VMA 0xbcb5c: mov r2, #66584576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,66584576)?)?;
    // VMA 0xbcb60: str r5, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,36)?)?;
    // VMA 0xbcb64: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xbcb68: orr r2, r2, #1006632960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1006632960)?)?;
    // VMA 0xbcb6c: and r2, r2, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(5)?,Shift::Lsl,16,false)?)?;
    // VMA 0xbcb70: and r1, r1, r8, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(8)?,Shift::Lsl,4,false)?)?;
    // VMA 0xbcb74: lsl r5, r7, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(7)?,Shift::Lsl,15)?)?;
    // VMA 0xbcb78: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbcb7c: mov r2, #10432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10432)?)?;
    // VMA 0xbcb80: orr r2, r2, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbcb84: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    c.check(0xbcb88)
}
