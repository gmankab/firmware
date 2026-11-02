
/// Initialize the codec buffer lifecycle and update all controller
/// metadata registers for the active input layout, including error paths.
/// The first function returns at 0xbd518 and a diagnostic tail branches
/// back to its cleanup epilogue.
fn initialize_controller_stream_record_state(c:&mut Code)->Result<(),String>{
    c.check(0xbd000)?;
    // VMA 0xbd000: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbd004: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xbd008: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // VMA 0xbd00c: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // VMA 0xbd010: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // VMA 0xbd014: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbd018: ldr r0, [r6, #0xa4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,164)?)?;
    // VMA 0xbd01c: ldr r4, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,4)?)?;
    // VMA 0xbd020: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // VMA 0xbd024: ldr r0, [r6, #0xe0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,224)?)?;
    // VMA 0xbd028: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xbd02c: ldr r0, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,468)?)?;
    // VMA 0xbd030: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xbd034: ldr r0, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,472)?)?;
    // VMA 0xbd038: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // VMA 0xbd03c: str r5, [r7, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,732)?)?;
    // VMA 0xbd040: str r5, [r6, #0x854]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,2132)?)?;
    // VMA 0xbd044: bl 0x26754 <entry+0xfffffffffff69f78> @ imm = #-0x968f8
    c.b(C::Al,0x26754,true)?;
    // VMA 0xbd048: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xbd04c: str r5, [r7, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,468)?)?;
    // VMA 0xbd050: ldr r1, [r7, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,448)?)?;
    // VMA 0xbd054: ands r0, r1, #255
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R1,255,true)?)?;
    // VMA 0xbd058: beq 0xbd074 <entry+0x898>   @ imm = #0x14
    c.b(C::Eq,0xbd074,false)?;
    // VMA 0xbd05c: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xbd060: str r0, [r7, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,468)?)?;
    // VMA 0xbd064: str r1, [r7, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,448)?)?;
    // VMA 0xbd068: ldr r1, [r7, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,456)?)?;
    // VMA 0xbd06c: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd070: str r0, [r7, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,456)?)?;
    // VMA 0xbd074: mov r8, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,124)?)?;
    // VMA 0xbd078: mov r9, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,36)?)?;
    // VMA 0xbd07c: mov r10, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,3758096384)?)?;
    // VMA 0xbd080: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xbd084: orr r9, r9, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,2048)?)?;
    // VMA 0xbd088: ldr r0, [r6, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,240)?)?;
    // VMA 0xbd08c: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // VMA 0xbd090: beq 0xbd23c <entry+0xa60>   @ imm = #0x1a4
    c.b(C::Eq,0xbd23c,false)?;
    // VMA 0xbd094: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // VMA 0xbd098: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // VMA 0xbd09c: mov r2, #868
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,868)?)?;
    // VMA 0xbd0a0: mov r3, #2912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2912)?)?;
    // VMA 0xbd0a4: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xbd0a8: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // VMA 0xbd0ac: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // VMA 0xbd0b0: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbd0b4: mov r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2048)?)?;
    // VMA 0xbd0b8: str r1, [r0, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd0bc: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbd0c0: ldr r0, [r4, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,464)?)?;
    // VMA 0xbd0c4: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbd0c8: mov r2, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,48)?)?;
    // VMA 0xbd0cc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd0d0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // VMA 0xbd0d4: moveq r2, #4224
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,4224)?)?;
    // VMA 0xbd0d8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd0dc: str r2, [r1, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd0e0: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbd0e4: orr r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbd0e8: ldr r3, [r7, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,448)?)?;
    // VMA 0xbd0ec: str r3, [r1, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd0f0: mov r3, #3040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3040)?)?;
    // VMA 0xbd0f4: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbd0f8: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // VMA 0xbd0fc: orr r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbd100: str r2, [r1, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd104: mov r2, #988
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,988)?)?;
    // VMA 0xbd108: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbd10c: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // VMA 0xbd110: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbd114: ldr r2, [r7, #0x204]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,516)?)?;
    // VMA 0xbd118: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // VMA 0xbd11c: str r2, [r1, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd120: mov r2, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // VMA 0xbd124: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbd128: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xbd12c: ldr r3, [r7, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,468)?)?;
    // VMA 0xbd130: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbd134: ldr r2, [r7, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,456)?)?;
    // VMA 0xbd138: sub r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd13c: mov r3, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2144)?)?;
    // VMA 0xbd140: str r2, [r1, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd144: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // VMA 0xbd148: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbd14c: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xbd150: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbd154: ldr r2, [r7, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,468)?)?;
    // VMA 0xbd158: str r2, [r1, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd15c: mvn r2, #-1946157056
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,2348810240)?)?;
    // VMA 0xbd160: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbd164: orr r1, r8, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(8)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbd168: str r2, [r1, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd16c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbd170: ldr r2, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,8)?)?;
    // VMA 0xbd174: moveq r1, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbd178: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd17c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd180: moveq r0, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xbd184: orr r2, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xbd188: str r1, [r2, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd18c: orr r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,1)?)?;
    // VMA 0xbd190: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbd194: orr r1, r9, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(9)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbd198: str r2, [r1, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd19c: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xbd1a0: orr r1, r9, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(9)?,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xbd1a4: str r0, [r1, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd1a8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xbd1ac: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xbd1b0: bl 0xbd648 <entry+0xe6c>   @ imm = #0x490
    c.b(C::Al,0xbd648,true)?;
    // VMA 0xbd1b4: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xbd1b8: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // VMA 0xbd1bc: ldr r1, [pc, #0x37c]        @ 0xd64 <entry+0xd64>
    c.literal(R::R1,0xbd540)?;
    // VMA 0xbd1c0: ldr r1, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbd1c4: lsl r0, r0, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,15)?)?;
    // VMA 0xbd1c8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbd1cc: beq 0xbd1e0 <entry+0xa04>   @ imm = #0xc
    c.b(C::Eq,0xbd1e0,false)?;
    // VMA 0xbd1d0: orr r0, r0, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1040)?)?;
    // VMA 0xbd1d4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbd1d8: str r1, [r0, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd1dc: b 0xbd230 <entry+0xa54>   @ imm = #0x4c
    c.b(C::Al,0xbd230,false)?;
    // VMA 0xbd1e0: orr r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd1e4: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbd1e8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbd1ec: str r1, [r0, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd1f0: mov r1, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,44)?)?;
    // VMA 0xbd1f4: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // VMA 0xbd1f8: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xbd1fc: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbd200: str r2, [r0, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd204: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // VMA 0xbd208: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbd20c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbd210: str r1, [r0, r10]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd214: mov r1, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,120)?)?;
    // VMA 0xbd218: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // VMA 0xbd21c: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xbd220: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbd224: ldr r1, [r0, r10]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::new(10)?,Shift::Lsl,0)?)?;
    // VMA 0xbd228: tst r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,2)?)?;
    // VMA 0xbd22c: beq 0xbd224 <entry+0xa48>   @ imm = #-0x10
    c.b(C::Eq,0xbd224,false)?;
    // VMA 0xbd230: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xbd234: beq 0xbd088 <entry+0x8ac>   @ imm = #-0x1b4
    c.b(C::Eq,0xbd088,false)?;
    // VMA 0xbd238: b 0xbd240 <entry+0xa64>   @ imm = #0x0
    c.b(C::Al,0xbd240,false)?;
    // VMA 0xbd23c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbd240: ldr r0, [r6, #0xa4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,164)?)?;
    // VMA 0xbd244: str r0, [r7, #0xc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,200)?)?;
    // VMA 0xbd248: ldr r9, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(7)?,0)?)?;
    // VMA 0xbd24c: bl 0x26754 <entry+0xfffffffffff69f78> @ imm = #-0x96b00
    c.b(C::Al,0x26754,true)?;
    // VMA 0xbd250: ldr r1, [pc, #0x2ec]        @ 0xd68 <entry+0xd68>
    c.literal(R::R1,0xbd544)?;
    // VMA 0xbd254: ldr r3, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // VMA 0xbd258: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbd25c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xbd260: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // VMA 0xbd264: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8e5d8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd268: ldr r1, [pc, #0x2d8]        @ 0xd6c <entry+0xd6c>
    c.literal(R::R1,0xbd548)?;
    // VMA 0xbd26c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xbd270: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8e5e4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd274: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // VMA 0xbd278: ldr r0, [r6, #0xe0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,224)?)?;
    // VMA 0xbd27c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xbd280: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbd284: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd288: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd28c: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // VMA 0xbd290: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd294: str r0, [r7, #0x2c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,712)?)?;
    // VMA 0xbd298: bne 0xbd2bc <entry+0xae0>   @ imm = #0x1c
    c.b(C::Ne,0xbd2bc,false)?;
    // VMA 0xbd29c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbd2a0: str r0, [r6, #0x854]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2132)?)?;
    // VMA 0xbd2a4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd2a8: ldr r1, [r6, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,240)?)?;
    // VMA 0xbd2ac: cmp r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // VMA 0xbd2b0: moveq r0, #6
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xbd2b4: str r0, [r6, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,240)?)?;
    // VMA 0xbd2b8: b 0xbd2c0 <entry+0xae4>   @ imm = #0x0
    c.b(C::Al,0xbd2c0,false)?;
    // VMA 0xbd2bc: ldr r0, [r6, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,240)?)?;
    // VMA 0xbd2c0: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // VMA 0xbd2c4: bne 0xbd300 <entry+0xb24>   @ imm = #0x34
    c.b(C::Ne,0xbd300,false)?;
    // VMA 0xbd2c8: ldr r0, [r6, #0x854]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2132)?)?;
    // VMA 0xbd2cc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbd2d0: str r1, [r6, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,240)?)?;
    // VMA 0xbd2d4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbd2d8: bne 0xbd314 <entry+0xb38>   @ imm = #0x34
    c.b(C::Ne,0xbd314,false)?;
    // VMA 0xbd2dc: ldr r0, [r7, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,464)?)?;
    // VMA 0xbd2e0: ldr r1, [r6, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,2104)?)?;
    // VMA 0xbd2e4: add r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd2e8: str r1, [r6, #0x838]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,2104)?)?;
    // VMA 0xbd2ec: ldr r1, [r6, #0x840]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,2112)?)?;
    // VMA 0xbd2f0: sub r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd2f4: str r0, [r6, #0x840]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2112)?)?;
    // VMA 0xbd2f8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd2fc: b 0xbd314 <entry+0xb38>   @ imm = #0x10
    c.b(C::Al,0xbd314,false)?;
    // VMA 0xbd300: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbd304: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbd308: str r1, [r7, #0x208]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,520)?)?;
    // VMA 0xbd30c: str r0, [r6, #0x854]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,2132)?)?;
    // VMA 0xbd310: str r1, [r6, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,240)?)?;
    // VMA 0xbd314: str r0, [r7, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,472)?)?;
    // VMA 0xbd318: ldr r0, [r7, #0x23c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,572)?)?;
    // VMA 0xbd31c: ldr r1, [pc, #0x228]        @ 0xd70 <entry+0xd70>
    c.literal(R::R1,0xbd54c)?;
    // VMA 0xbd320: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbd324: beq 0xbd510 <entry+0xd34>   @ imm = #0x1e4
    c.b(C::Eq,0xbd510,false)?;
    // VMA 0xbd328: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xbd32c: mvn r3, #-268435455
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,4026531841)?)?;
    // VMA 0xbd330: str r0, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,476)?)?;
    // VMA 0xbd334: add r2, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,15)?)?;
    // VMA 0xbd338: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // VMA 0xbd33c: lsr r0, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsr,4)?)?;
    // VMA 0xbd340: bic r2, r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,15)?)?;
    // VMA 0xbd344: str r1, [r4, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,480)?)?;
    // VMA 0xbd348: str r0, [r4, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,484)?)?;
    // VMA 0xbd34c: add r0, r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,31)?)?;
    // VMA 0xbd350: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // VMA 0xbd354: and r3, r3, r0, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R0,Shift::Lsr,4,false)?)?;
    // VMA 0xbd358: ldr r0, [r6, #0xa4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,164)?)?;
    // VMA 0xbd35c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbd360: lsreq r3, r1, #4
    c.emit(arm32::mov_shifted_register(C::Eq,R::new(3)?,R::R1,Shift::Lsr,4)?)?;
    // VMA 0xbd364: lsl r1, r3, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(3)?,Shift::Lsl,4)?)?;
    // VMA 0xbd368: str r3, [r4, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,488)?)?;
    // VMA 0xbd36c: str r2, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,468)?)?;
    // VMA 0xbd370: str r1, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,472)?)?;
    // VMA 0xbd374: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // VMA 0xbd378: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbd37c: bne 0xbd3fc <entry+0xc20>   @ imm = #0x78
    c.b(C::Ne,0xbd3fc,false)?;
    // VMA 0xbd380: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // VMA 0xbd384: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbd388: ldreq r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::SP,8)?)?;
    // VMA 0xbd38c: cmpeq r1, r0
    c.emit(arm32::data_register(C::Eq,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd390: bne 0xbd3fc <entry+0xc20>   @ imm = #0x64
    c.b(C::Ne,0xbd3fc,false)?;
    // VMA 0xbd394: ldr r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,192)?)?;
    // VMA 0xbd398: ldr r1, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,404)?)?;
    // VMA 0xbd39c: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd3a0: blo 0xbd3fc <entry+0xc20>   @ imm = #0x54
    c.b(C::Cc,0xbd3fc,false)?;
    // VMA 0xbd3a4: ldr r0, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,180)?)?;
    // VMA 0xbd3a8: ldr r1, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,400)?)?;
    // VMA 0xbd3ac: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd3b0: blo 0xbd3fc <entry+0xc20>   @ imm = #0x44
    c.b(C::Cc,0xbd3fc,false)?;
    // VMA 0xbd3b4: ldr r0, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xbd3b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbd3bc: beq 0xbd3e0 <entry+0xc04>   @ imm = #0x1c
    c.b(C::Eq,0xbd3e0,false)?;
    // VMA 0xbd3c0: ldr r0, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,248)?)?;
    // VMA 0xbd3c4: ldr r1, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,416)?)?;
    // VMA 0xbd3c8: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd3cc: blo 0xbd3fc <entry+0xc20>   @ imm = #0x28
    c.b(C::Cc,0xbd3fc,false)?;
    // VMA 0xbd3d0: ldr r0, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,236)?)?;
    // VMA 0xbd3d4: ldr r1, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,412)?)?;
    // VMA 0xbd3d8: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd3dc: blo 0xbd3fc <entry+0xc20>   @ imm = #0x18
    c.b(C::Cc,0xbd3fc,false)?;
    // VMA 0xbd3e0: ldr r0, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,348)?)?;
    // VMA 0xbd3e4: ldr r1, [r4, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,432)?)?;
    // VMA 0xbd3e8: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd3ec: blo 0xbd3fc <entry+0xc20>   @ imm = #0x8
    c.b(C::Cc,0xbd3fc,false)?;
    // VMA 0xbd3f0: ldr r0, [r4, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2700)?)?;
    // VMA 0xbd3f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbd3f8: beq 0xbd510 <entry+0xd34>   @ imm = #0x110
    c.b(C::Eq,0xbd510,false)?;
    // VMA 0xbd3fc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xbd400: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbd404: bl 0xd176c <entry+0x14f90> @ imm = #0x14360
    c.b(C::Al,0xd176c,true)?;
    // VMA 0xbd408: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd40c: str r0, [r4, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2700)?)?;
    // VMA 0xbd410: ldr r0, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xbd414: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbd418: beq 0xbd458 <entry+0xc7c>   @ imm = #0x38
    c.b(C::Eq,0xbd458,false)?;
    // VMA 0xbd41c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xbd420: bl 0xd2adc <entry+0x16300> @ imm = #0x156b4
    c.b(C::Al,0xd2adc,true)?;
    // VMA 0xbd424: mov r1, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,19)?)?;
    // VMA 0xbd428: orr r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // VMA 0xbd42c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xbd430: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbd434: bne 0xbd458 <entry+0xc7c>   @ imm = #0x1c
    c.b(C::Ne,0xbd458,false)?;
    // VMA 0xbd438: ldr r1, [pc, #0x110]        @ 0xd74 <entry+0xd74>
    c.literal(R::R1,0xbd550)?;
    // VMA 0xbd43c: ldr r2, [pc, #0x110]        @ 0xd78 <entry+0xd78>
    c.literal(R::R2,0xbd554)?;
    // VMA 0xbd440: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbd444: mov r3, #160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,160)?)?;
    // VMA 0xbd448: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8e7bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd44c: ldr r1, [pc, #0xf4]         @ 0xd6c <entry+0xd6c>
    c.literal(R::R1,0xbd548)?;
    // VMA 0xbd450: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbd454: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8e7c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd458: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xbd45c: bl 0xb60ec <entry+0xffffffffffff9910> @ imm = #-0x7378
    c.b(C::Al,0xb60ec,true)?;
    // VMA 0xbd460: ldr r0, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,192)?)?;
    // VMA 0xbd464: ldr r1, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,404)?)?;
    // VMA 0xbd468: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd46c: blo 0xbd4cc <entry+0xcf0>   @ imm = #0x58
    c.b(C::Cc,0xbd4cc,false)?;
    // VMA 0xbd470: ldr r0, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,180)?)?;
    // VMA 0xbd474: ldr r1, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,400)?)?;
    // VMA 0xbd478: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd47c: blo 0xbd4cc <entry+0xcf0>   @ imm = #0x48
    c.b(C::Cc,0xbd4cc,false)?;
    // VMA 0xbd480: ldr r0, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,292)?)?;
    // VMA 0xbd484: ldr r1, [r4, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,424)?)?;
    // VMA 0xbd488: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd48c: blo 0xbd4cc <entry+0xcf0>   @ imm = #0x38
    c.b(C::Cc,0xbd4cc,false)?;
    // VMA 0xbd490: ldr r0, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,348)?)?;
    // VMA 0xbd494: ldr r1, [r4, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,432)?)?;
    // VMA 0xbd498: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd49c: blo 0xbd4cc <entry+0xcf0>   @ imm = #0x28
    c.b(C::Cc,0xbd4cc,false)?;
    // VMA 0xbd4a0: ldr r0, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,972)?)?;
    // VMA 0xbd4a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbd4a8: beq 0xbd51c <entry+0xd40>   @ imm = #0x6c
    c.b(C::Eq,0xbd51c,false)?;
    // VMA 0xbd4ac: ldr r0, [r4, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,248)?)?;
    // VMA 0xbd4b0: ldr r1, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,416)?)?;
    // VMA 0xbd4b4: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd4b8: blo 0xbd4cc <entry+0xcf0>   @ imm = #0xc
    c.b(C::Cc,0xbd4cc,false)?;
    // VMA 0xbd4bc: ldr r0, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,236)?)?;
    // VMA 0xbd4c0: ldr r1, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,412)?)?;
    // VMA 0xbd4c4: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbd4c8: bhs 0xbd51c <entry+0xd40>   @ imm = #0x4c
    c.b(C::Cs,0xbd51c,false)?;
    // VMA 0xbd4cc: ldr r1, [pc, #0x88]         @ 0xd80 <entry+0xd80>
    c.literal(R::R1,0xbd55c)?;
    // VMA 0xbd4d0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbd4d4: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8e848
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd4d8: ldr r1, [pc, #0x68]         @ 0xd6c <entry+0xd6c>
    c.literal(R::R1,0xbd548)?;
    // VMA 0xbd4dc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbd4e0: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8e854
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd4e4: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xbd4e8: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xbd4ec: str r0, [r4, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,376)?)?;
    // VMA 0xbd4f0: str r1, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,504)?)?;
    // VMA 0xbd4f4: ldr r1, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,972)?)?;
    // VMA 0xbd4f8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbd4fc: strne r0, [r4, #0x17c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,380)?)?;
    // VMA 0xbd500: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbd504: str r0, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,400)?)?;
    // VMA 0xbd508: str r0, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,412)?)?;
    // VMA 0xbd50c: str r0, [r4, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,424)?)?;
    // VMA 0xbd510: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xbd514: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbd518: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xbd51c: ldr r1, [pc, #0x34]         @ 0xd7c <entry+0xd7c>
    c.literal(R::R1,0xbd558)?;
    // VMA 0xbd520: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbd524: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8e898
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd528: ldr r1, [pc, #0x18]         @ 0xd6c <entry+0xd6c>
    c.literal(R::R1,0xbd548)?;
    // VMA 0xbd52c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbd530: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8e8a4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbd534: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbd538: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // VMA 0xbd53c: b 0xbd510 <entry+0xd34>   @ imm = #-0x34
    c.b(C::Al,0xbd510,false)?;
    c.check(0xbd540)
}

/// Global addresses, diagnostic pointers and descriptor layout constants.
/// These eight words are data, not executable A32 instructions.
const CONTROLLER_RECORD_INIT_LITERALS:[u32;8]=[
    0x0010_408c,0x000f_a170,0x000e_2788,0x0100_1004,
    0x000f_a1ac,0x000f_a1da,0x000f_a1f9,0x000f_a227,
];
