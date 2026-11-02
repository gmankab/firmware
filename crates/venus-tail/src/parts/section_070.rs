
/// Copy stride/plane offsets for input and optional output descriptors,
/// report a fixed diagnostic event, and restore the caller frame.
fn update_active_stream_plane_offsets(c:&mut Code)->Result<(),String>{
    c.check(0xbed64)?;
    // VMA 0xbed64: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xbed68: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xbed6c: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // VMA 0xbed70: ldr r0, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    // VMA 0xbed74: mov r4, r3
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // VMA 0xbed78: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // VMA 0xbed7c: str r0, [r4, #0x5c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1472)?)?;
    // VMA 0xbed80: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbed84: str r0, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,292)?)?;
    // VMA 0xbed88: ldr r1, [r4, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,176)?)?;
    // VMA 0xbed8c: add r2, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbed90: str r2, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,296)?)?;
    // VMA 0xbed94: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,152)?)?;
    // VMA 0xbed98: ldr r1, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,180)?)?;
    // VMA 0xbed9c: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbeda0: add r3, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbeda4: ldr r1, [pc, #0xe8]         @ 0x324 <entry+0x324>
    c.literal(R::R1,0xbee94)?;
    // VMA 0xbeda8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbedac: str r3, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,300)?)?;
    // VMA 0xbedb0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90124
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbedb4: ldr r1, [pc, #0xdc]         @ 0x328 <entry+0x328>
    c.literal(R::R1,0xbee98)?;
    // VMA 0xbedb8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbedbc: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90130
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbedc0: ldr r0, [r4, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,188)?)?;
    // VMA 0xbedc4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbedc8: beq 0xbee00 <entry+0x290>   @ imm = #0x30
    c.b(C::Eq,0xbee00,false)?;
    // VMA 0xbedcc: ldr r2, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,292)?)?;
    // VMA 0xbedd0: str r2, [r4, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,304)?)?;
    // VMA 0xbedd4: ldr r1, [r4, #0xb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,176)?)?;
    // VMA 0xbedd8: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,152)?)?;
    // VMA 0xbeddc: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbede0: add r3, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbede4: ldr r1, [pc, #0xb0]         @ 0x32c <entry+0x32c>
    c.literal(R::R1,0xbee9c)?;
    // VMA 0xbede8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbedec: str r3, [r4, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,308)?)?;
    // VMA 0xbedf0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90164
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbedf4: ldr r1, [pc, #0x9c]         @ 0x328 <entry+0x328>
    c.literal(R::R1,0xbee98)?;
    // VMA 0xbedf8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbedfc: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90170
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbee00: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xbee04: ldr r0, [r0, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,972)?)?;
    // VMA 0xbee08: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbee0c: popeq {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xbee10: str r5, [r4, #0x148]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,328)?)?;
    // VMA 0xbee14: ldr r0, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,248)?)?;
    // VMA 0xbee18: add r2, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbee1c: str r2, [r4, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,332)?)?;
    // VMA 0xbee20: ldr r0, [r4, #0xe0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,224)?)?;
    // VMA 0xbee24: ldr r1, [r4, #0xfc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,252)?)?;
    // VMA 0xbee28: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbee2c: add r3, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbee30: ldr r1, [pc, #0x5c]         @ 0x324 <entry+0x324>
    c.literal(R::R1,0xbee94)?;
    // VMA 0xbee34: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbee38: str r3, [r4, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,336)?)?;
    // VMA 0xbee3c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x901b0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbee40: ldr r1, [pc, #0x50]         @ 0x328 <entry+0x328>
    c.literal(R::R1,0xbee98)?;
    // VMA 0xbee44: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbee48: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x901bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbee4c: ldr r0, [r4, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,260)?)?;
    // VMA 0xbee50: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbee54: beq 0xbee90 <entry+0x320>   @ imm = #0x34
    c.b(C::Eq,0xbee90,false)?;
    // VMA 0xbee58: ldr r2, [r4, #0x148]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,328)?)?;
    // VMA 0xbee5c: str r2, [r4, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,340)?)?;
    // VMA 0xbee60: ldr r1, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,248)?)?;
    // VMA 0xbee64: ldr r0, [r4, #0xe0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,224)?)?;
    // VMA 0xbee68: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbee6c: add r3, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbee70: ldr r1, [pc, #0x24]         @ 0x32c <entry+0x32c>
    c.literal(R::R1,0xbee9c)?;
    // VMA 0xbee74: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbee78: str r3, [r4, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,344)?)?;
    // VMA 0xbee7c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x901f0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbee80: ldr r1, [pc, #0x10]         @ 0x328 <entry+0x328>
    c.literal(R::R1,0xbee98)?;
    // VMA 0xbee84: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbee88: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xbee8c: b 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x90200
    c.b(C::Al,0x2ec94,false)?;
    // VMA 0xbee90: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xbee94)?;
    const CODEC_PLANE_OFFSET_DIAGNOSTICS:[u32;3]=[0x000fa6a8,0x000e2788,0x000fa6c9];
    for value in CODEC_PLANE_OFFSET_DIAGNOSTICS {c.emit(value)?;}
    c.check(0xbeea0)?;
    Ok(())
}
