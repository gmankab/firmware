
/// Publish the codec-buffer DMA state and update per-channel accounting;
/// the inline global callback pointer following the return is data.
fn publish_codec_buffer_registers(c:&mut Code)->Result<(),String>{
    c.check(0xae648)?;
    // ARM 0xae648: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xae64c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM 0xae650: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xae654: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // ARM 0xae658: ldr r0, [r5, #0x994]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2452)?)?;
    // ARM 0xae65c: add r6, r5, #2816
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(5)?,2816)?)?;
    // ARM 0xae660: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae664: bne 0xae694 <entry+0x1f44>
    c.b(C::Ne,0xae694,false)?;
    // ARM 0xae668: ldr r0, [pc, #0xd4]
    c.literal(R::R0,0xae744)?;
    // ARM 0xae66c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM 0xae670: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // ARM 0xae674: add r0, r5, #2880
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,2880)?)?;
    // ARM 0xae678: blx r2
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // ARM 0xae67c: ldr r2, [r5, #0xbf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,3056)?)?;
    // ARM 0xae680: add r0, r4, #84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,84)?)?;
    // ARM 0xae684: add r1, r5, #3040
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,3040)?)?;
    // ARM 0xae688: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // ARM 0xae68c: str r2, [r4, #0x464]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1124)?)?;
    // ARM 0xae690: bl 0x471c <entry+0xfffffffffff57fcc>
    c.b(C::Al,0x471c,true)?;
    // ARM 0xae694: add r0, r5, #2880
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,2880)?)?;
    // ARM 0xae698: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // ARM 0xae69c: bl 0xb3d04 <entry+0x75b4>
    c.b(C::Al,0xb3d04,true)?;
    // ARM 0xae6a0: ldr r0, [r5, #0x994]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2452)?)?;
    // ARM 0xae6a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae6a8: bne 0xae6b8 <entry+0x1f68>
    c.b(C::Ne,0xae6b8,false)?;
    // ARM 0xae6ac: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xae6b0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // ARM 0xae6b4: bl 0xa9578 <entry+0xffffffffffffce28>
    c.b(C::Al,0xa9578,true)?;
    // ARM 0xae6b8: ldr r0, [r5, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2432)?)?;
    // ARM 0xae6bc: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // ARM 0xae6c0: str r0, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,116)?)?;
    // ARM 0xae6c4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xae6c8: bl 0xaa22c <entry+0xffffffffffffdadc>
    c.b(C::Al,0xaa22c,true)?;
    // ARM 0xae6cc: ldr r0, [r5, #0x488]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1160)?)?;
    // ARM 0xae6d0: mov r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // ARM 0xae6d4: bl 0xd9f0c <entry+0x2d7bc>
    c.b(C::Al,0xd9f0c,true)?;
    // ARM 0xae6d8: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,144)?)?;
    // ARM 0xae6dc: add r0, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,64)?)?;
    // ARM 0xae6e0: str r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,144)?)?;
    // ARM 0xae6e4: ldr r0, [r5, #0x46c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1132)?)?;
    // ARM 0xae6e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae6ec: beq 0xae708 <entry+0x1fb8>
    c.b(C::Eq,0xae708,false)?;
    // ARM 0xae6f0: ldr r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,148)?)?;
    // ARM 0xae6f4: mov r1, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,192)?)?;
    // ARM 0xae6f8: add r0, r0, #192
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,192)?)?;
    // ARM 0xae6fc: str r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,148)?)?;
    // ARM 0xae700: ldr r0, [r5, #0x48c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1164)?)?;
    // ARM 0xae704: bl 0xd9f0c <entry+0x2d7bc>
    c.b(C::Al,0xd9f0c,true)?;
    // ARM 0xae708: ldr r0, [r5, #0x994]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2452)?)?;
    // ARM 0xae70c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xae710: str r0, [r5, #0x994]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2452)?)?;
    // ARM 0xae714: ldr r1, [r4, #0x448]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1096)?)?;
    // ARM 0xae718: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xae71c: beq 0xae73c <entry+0x1fec>
    c.b(C::Eq,0xae73c,false)?;
    // ARM 0xae720: mov r0, #680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,680)?)?;
    // ARM 0xae724: ldr r2, [r1, #0x778]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,1912)?)?;
    // ARM 0xae728: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xae72c: ldrsb r0, [r5, r0]
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::new(5)?,R::R0)?)?;
    // ARM 0xae730: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xae734: str r0, [r1, #0x778]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,1912)?)?;
    // ARM 0xae738: ldr r0, [r5, #0x994]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2452)?)?;
    // ARM 0xae73c: str r0, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,124)?)?;
    // ARM 0xae740: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xae744)?;
    c.emit(0x0010_dd90)?; // Controller callback-table address, not instruction.
    c.check(0xae748)
}
