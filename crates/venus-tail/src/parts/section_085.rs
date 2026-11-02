
/// Check codec plane sizes before packet publication, then unpack the
/// bit-packed 64-bit controller capability fields into bounded byte and
/// halfword descriptors. The original fast-path branch and caller return
/// instructions remain present rather than being replaced with placeholders.
fn decode_packed_controller_plane_parameters(c:&mut Code)->Result<(),String>{
    c.check(0xc420c)?;
    // VMA 0xc420c: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc4210: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xc4214: ldr r6, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,12)?)?;
    // VMA 0xc4218: mov r5, r3
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // VMA 0xc421c: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xc4220: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xc4224: lsl r0, r5, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsl,4)?)?;
    // VMA 0xc4228: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc422c: blt 0xc4270 <entry+0x270>   @ imm = #0x3c
    c.b(C::Lt,0xc4270,false)?;
    // VMA 0xc4230: ldr r1, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,8)?)?;
    // VMA 0xc4234: lsl r0, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsl,4)?)?;
    // VMA 0xc4238: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc423c: lslge r0, r1, #1
    c.emit(arm32::mov_shifted_register(C::Ge,R::R0,R::R1,Shift::Lsl,1)?)?;
    // VMA 0xc4240: cmpge r0, r2
    c.emit(arm32::data_register(C::Ge,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc4244: blt 0xc4270 <entry+0x270>   @ imm = #0x24
    c.b(C::Lt,0xc4270,false)?;
    // VMA 0xc4248: lsl r0, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(6)?,Shift::Lsl,1)?)?;
    // VMA 0xc424c: cmp r0, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc4250: blt 0xc4270 <entry+0x270>   @ imm = #0x18
    c.b(C::Lt,0xc4270,false)?;
    // VMA 0xc4254: lsl r0, r2, #14
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsl,14)?)?;
    // VMA 0xc4258: blx 0x82be4 <entry+0xfffffffffffbebe4> @ imm = #-0x4167c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xc425c: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc4260: lsl r0, r5, #14
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsl,14)?)?;
    // VMA 0xc4264: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc4268: blx 0x82be4 <entry+0xfffffffffffbebe4> @ imm = #-0x4168c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xc426c: b 0xc4278 <entry+0x278>   @ imm = #0x4
    c.b(C::Al,0xc4278,false)?;
    // VMA 0xc4270: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // VMA 0xc4274: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc4278: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc427c: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc4280: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xc4284: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // VMA 0xc4288: and r2, r2, #63
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,63)?)?;
    // VMA 0xc428c: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // VMA 0xc4290: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xc4294: and r2, r3, r2, lsr #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,6,false)?)?;
    // VMA 0xc4298: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // VMA 0xc429c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc42a0: ldr r3, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,44)?)?;
    // VMA 0xc42a4: and r3, r2, r3, lsr #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,5,false)?)?;
    // VMA 0xc42a8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc42ac: strb r3, [r0, #0x8]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // VMA 0xc42b0: beq 0xc43a4 <entry+0x3a4>   @ imm = #0xec
    c.b(C::Eq,0xc43a4,false)?;
    // VMA 0xc42b4: ldr r3, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,44)?)?;
    // VMA 0xc42b8: and r2, r2, r3, lsr #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsr,6,false)?)?;
    // VMA 0xc42bc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc42c0: strb r2, [r0, #0x9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,9)?)?;
    // VMA 0xc42c4: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // VMA 0xc42c8: push {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc42cc: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xc42d0: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xc42d4: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xc42d8: and r4, r3, r2, lsr #9
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(3)?,R::R2,Shift::Lsr,9,false)?)?;
    // VMA 0xc42dc: strb r4, [r0, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::R0,10)?)?;
    // VMA 0xc42e0: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc42e4: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xc42e8: and r5, r3, r2, lsr #17
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(3)?,R::R2,Shift::Lsr,17,false)?)?;
    // VMA 0xc42ec: strb r5, [r0, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R0,11)?)?;
    // VMA 0xc42f0: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xc42f4: and lr, r3, r2, lsr #25
    c.emit(arm32::data_register(C::Al,Op::And,R::LR,R::new(3)?,R::R2,Shift::Lsr,25,false)?)?;
    // VMA 0xc42f8: strb lr, [r0, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::R0,12)?)?;
    // VMA 0xc42fc: ldr r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,4)?)?;
    // VMA 0xc4300: and r12, r3, r2, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R12,R::new(3)?,R::R2,Shift::Lsr,1,false)?)?;
    // VMA 0xc4304: movne r3, #127
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,127)?)?;
    // VMA 0xc4308: strb r12, [r0, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R0,13)?)?;
    // VMA 0xc430c: ldrne r2, [r1]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::R1,0)?)?;
    // VMA 0xc4310: andne r2, r3, r2, lsr #10
    c.emit(arm32::data_register(C::Ne,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,10,false)?)?;
    // VMA 0xc4314: strbne r2, [r0, #0xe]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R2,R::R0,14)?)?;
    // VMA 0xc4318: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc431c: ldrne r2, [r1]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::R1,0)?)?;
    // VMA 0xc4320: movne r3, #127
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,127)?)?;
    // VMA 0xc4324: andne r2, r3, r2, lsr #18
    c.emit(arm32::data_register(C::Ne,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,18,false)?)?;
    // VMA 0xc4328: strbne r2, [r0, #0xf]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R2,R::R0,15)?)?;
    // VMA 0xc432c: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // VMA 0xc4330: beq 0xc4348 <entry+0x348>   @ imm = #0x10
    c.b(C::Eq,0xc4348,false)?;
    // VMA 0xc4334: ldrd r2, r3, [r1]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xc4338: lsr r2, r2, #26
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsr,26)?)?;
    // VMA 0xc433c: orr r2, r2, r3, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,6,false)?)?;
    // VMA 0xc4340: and r2, r2, #127
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,127)?)?;
    // VMA 0xc4344: strb r2, [r0, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,16)?)?;
    // VMA 0xc4348: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // VMA 0xc434c: ldrne r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::R1,4)?)?;
    // VMA 0xc4350: movne r3, #127
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,127)?)?;
    // VMA 0xc4354: andne r2, r3, r2, lsr #2
    c.emit(arm32::data_register(C::Ne,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,2,false)?)?;
    // VMA 0xc4358: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xc435c: strbne r2, [r0, #0x11]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R2,R::R0,17)?)?;
    // VMA 0xc4360: ldr r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,4)?)?;
    // VMA 0xc4364: and r5, r3, r2, lsr #9
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(3)?,R::R2,Shift::Lsr,9,false)?)?;
    // VMA 0xc4368: strb r5, [r0, #0x12]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R0,18)?)?;
    // VMA 0xc436c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc4370: ldr r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,4)?)?;
    // VMA 0xc4374: movne r5, #127
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,127)?)?;
    // VMA 0xc4378: and r2, r3, r2, lsr #17
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,17,false)?)?;
    // VMA 0xc437c: strb r2, [r0, #0x13]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,19)?)?;
    // VMA 0xc4380: ldrne r3, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::R1,4)?)?;
    // VMA 0xc4384: andne r3, r5, r3, lsr #10
    c.emit(arm32::data_register(C::Ne,Op::And,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsr,10,false)?)?;
    // VMA 0xc4388: strbne r3, [r0, #0x14]
    c.emit(arm32::transfer_byte(C::Ne,false,R::new(3)?,R::R0,20)?)?;
    // VMA 0xc438c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc4390: pop {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc4394: ldrne r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::R1,4)?)?;
    // VMA 0xc4398: movne r2, #127
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,127)?)?;
    // VMA 0xc439c: andne r1, r2, r1, lsr #18
    c.emit(arm32::data_register(C::Ne,Op::And,R::R1,R::R2,R::R1,Shift::Lsr,18,false)?)?;
    // VMA 0xc43a0: strbne r1, [r0, #0x15]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::R0,21)?)?;
    // VMA 0xc43a4: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xc43a8: ldr r3, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,44)?)?;
    // VMA 0xc43ac: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc43b0: and r3, r2, r3, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,2,false)?)?;
    // VMA 0xc43b4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc43b8: strb r3, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc43bc: beq 0xc4690 <entry+0x690>   @ imm = #0x2cc
    c.b(C::Eq,0xc4690,false)?;
    // VMA 0xc43c0: ldr r3, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,44)?)?;
    // VMA 0xc43c4: and r2, r2, r3, lsr #3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsr,3,false)?)?;
    // VMA 0xc43c8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc43cc: strb r2, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1)?)?;
    // VMA 0xc43d0: beq 0xc4430 <entry+0x430>   @ imm = #0x58
    c.b(C::Eq,0xc4430,false)?;
    // VMA 0xc43d4: ldrb r2, [r1, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,10)?)?;
    // VMA 0xc43d8: strb r2, [r0, #0x5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,5)?)?;
    // VMA 0xc43dc: ldrb r2, [r1, #0xb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,11)?)?;
    // VMA 0xc43e0: strb r2, [r0, #0x6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,6)?)?;
    // VMA 0xc43e4: ldrb r2, [r1, #0xc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,12)?)?;
    // VMA 0xc43e8: strb r2, [r0, #0x7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,7)?)?;
    // VMA 0xc43ec: ldrb r2, [r1, #0xd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,13)?)?;
    // VMA 0xc43f0: strb r2, [r0, #0x8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,8)?)?;
    // VMA 0xc43f4: ldrb r2, [r1, #0xe]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,14)?)?;
    // VMA 0xc43f8: strb r2, [r0, #0x9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,9)?)?;
    // VMA 0xc43fc: ldrb r2, [r1, #0xf]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,15)?)?;
    // VMA 0xc4400: strb r2, [r0, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,10)?)?;
    // VMA 0xc4404: ldrb r2, [r1, #0x10]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,16)?)?;
    // VMA 0xc4408: strb r2, [r0, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,11)?)?;
    // VMA 0xc440c: ldrb r2, [r1, #0x2c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,44)?)?;
    // VMA 0xc4410: and r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,1)?)?;
    // VMA 0xc4414: strb r2, [r0, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,4)?)?;
    // VMA 0xc4418: ldrb r2, [r1, #0x11]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,17)?)?;
    // VMA 0xc441c: strb r2, [r0, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,12)?)?;
    // VMA 0xc4420: ldrb r2, [r1, #0x12]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,18)?)?;
    // VMA 0xc4424: strb r2, [r0, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,13)?)?;
    // VMA 0xc4428: ldrb r2, [r1, #0x13]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,19)?)?;
    // VMA 0xc442c: strb r2, [r0, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,14)?)?;
    // VMA 0xc4430: ldr r3, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,44)?)?;
    // VMA 0xc4434: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc4438: and r3, r2, r3, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,4,false)?)?;
    // VMA 0xc443c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc4440: strb r3, [r0, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,2)?)?;
    // VMA 0xc4444: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // VMA 0xc4448: ldr r3, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,44)?)?;
    // VMA 0xc444c: and r2, r2, r3, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsr,1,false)?)?;
    // VMA 0xc4450: strb r2, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,3)?)?;
    // VMA 0xc4454: ldr r2, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,20)?)?;
    // VMA 0xc4458: lsl r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,23)?)?;
    // VMA 0xc445c: asr r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,23)?)?;
    // VMA 0xc4460: strh r2, [r0, #16]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,16)?)?;
    // VMA 0xc4464: ldr r2, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,20)?)?;
    // VMA 0xc4468: ldr r3, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,24)?)?;
    // VMA 0xc446c: lsr r2, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsr,1)?)?;
    // VMA 0xc4470: orr r2, r2, r3, lsl #31
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,31,false)?)?;
    // VMA 0xc4474: asr r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,23)?)?;
    // VMA 0xc4478: strh r2, [r0, #24]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,24)?)?;
    // VMA 0xc447c: ldrh r2, [r1, #26]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::R1,26)?)?;
    // VMA 0xc4480: lsl r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,23)?)?;
    // VMA 0xc4484: asr r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,23)?)?;
    // VMA 0xc4488: strh r2, [r0, #32]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,32)?)?;
    // VMA 0xc448c: ldr r2, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,28)?)?;
    // VMA 0xc4490: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // VMA 0xc4494: asr r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,23)?)?;
    // VMA 0xc4498: strh r2, [r0, #40]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,40)?)?;
    // VMA 0xc449c: ldr r2, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,32)?)?;
    // VMA 0xc44a0: lsl r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,23)?)?;
    // VMA 0xc44a4: asr r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,23)?)?;
    // VMA 0xc44a8: strh r2, [r0, #48]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,48)?)?;
    // VMA 0xc44ac: ldr r2, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,36)?)?;
    // VMA 0xc44b0: ldrb r3, [r1, #0x23]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,35)?)?;
    // VMA 0xc44b4: orr r2, r3, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,8,false)?)?;
    // VMA 0xc44b8: lsl r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,23)?)?;
    // VMA 0xc44bc: asr r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,23)?)?;
    // VMA 0xc44c0: strh r2, [r0, #56]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,56)?)?;
    // VMA 0xc44c4: ldrh r2, [r1, #38]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::R1,38)?)?;
    // VMA 0xc44c8: lsl r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,23)?)?;
    // VMA 0xc44cc: asr r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,23)?)?;
    // VMA 0xc44d0: strh r2, [r0, #64]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,64)?)?;
    // VMA 0xc44d4: ldr r2, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,40)?)?;
    // VMA 0xc44d8: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // VMA 0xc44dc: asr r2, r2, #23
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,23)?)?;
    // VMA 0xc44e0: strh r2, [r0, #72]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,72)?)?;
    // VMA 0xc44e4: ldr r2, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,20)?)?;
    // VMA 0xc44e8: lsl r2, r2, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,16)?)?;
    // VMA 0xc44ec: asr r2, r2, #25
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,25)?)?;
    // VMA 0xc44f0: strh r2, [r0, #18]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,18)?)?;
    // VMA 0xc44f4: ldr r3, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,28)?)?;
    // VMA 0xc44f8: ldr r2, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,24)?)?;
    // VMA 0xc44fc: lsrs r3, r3, #1
    c.emit(arm32::data_register(C::Al,Op::Mov,R::new(3)?,R::R0,R::new(3)?,Shift::Lsr,1,true)?)?;
    // VMA 0xc4500: rrx r2, r2
    c.emit(arm32::rotate_right_with_extend(C::Al,R::R2,R::R2,false)?)?;
    // VMA 0xc4504: lsl r2, r2, #25
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,25)?)?;
    // VMA 0xc4508: asr r2, r2, #25
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,25)?)?;
    // VMA 0xc450c: strh r2, [r0, #26]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,26)?)?;
    // VMA 0xc4510: ldr r2, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,24)?)?;
    // VMA 0xc4514: asr r2, r2, #25
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,25)?)?;
    // VMA 0xc4518: strh r2, [r0, #34]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,34)?)?;
    // VMA 0xc451c: ldr r2, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,28)?)?;
    // VMA 0xc4520: lsl r2, r2, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,8)?)?;
    // VMA 0xc4524: asr r2, r2, #25
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,25)?)?;
    // VMA 0xc4528: strh r2, [r0, #42]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,42)?)?;
    // VMA 0xc452c: ldr r2, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,32)?)?;
    // VMA 0xc4530: lsl r2, r2, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,16)?)?;
    // VMA 0xc4534: asr r2, r2, #25
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,25)?)?;
    // VMA 0xc4538: strh r2, [r0, #50]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,50)?)?;
    // VMA 0xc453c: ldr r2, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,36)?)?;
    // VMA 0xc4540: lsl r2, r2, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,24)?)?;
    // VMA 0xc4544: asr r2, r2, #25
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,25)?)?;
    // VMA 0xc4548: strh r2, [r0, #58]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,58)?)?;
    // VMA 0xc454c: ldr r2, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,36)?)?;
    // VMA 0xc4550: asr r2, r2, #25
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,25)?)?;
    // VMA 0xc4554: strh r2, [r0, #66]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,66)?)?;
    // VMA 0xc4558: ldr r2, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,40)?)?;
    // VMA 0xc455c: lsl r2, r2, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,8)?)?;
    // VMA 0xc4560: asr r2, r2, #25
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,25)?)?;
    // VMA 0xc4564: strh r2, [r0, #74]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,74)?)?;
    // VMA 0xc4568: ldrh r2, [r1, #22]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::R1,22)?)?;
    // VMA 0xc456c: and r2, r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,3)?)?;
    // VMA 0xc4570: strh r2, [r0, #20]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,20)?)?;
    // VMA 0xc4574: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xc4578: ldr r3, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,24)?)?;
    // VMA 0xc457c: and r3, r2, r3, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,8,false)?)?;
    // VMA 0xc4580: strh r3, [r0, #28]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,28)?)?;
    // VMA 0xc4584: ldr r3, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,28)?)?;
    // VMA 0xc4588: and r3, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,3)?)?;
    // VMA 0xc458c: strh r3, [r0, #36]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,36)?)?;
    // VMA 0xc4590: ldrb r3, [r1, #0x1f]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,31)?)?;
    // VMA 0xc4594: and r3, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,3)?)?;
    // VMA 0xc4598: strh r3, [r0, #44]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,44)?)?;
    // VMA 0xc459c: ldrh r3, [r1, #34]
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::R1,34)?)?;
    // VMA 0xc45a0: and r3, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,3)?)?;
    // VMA 0xc45a4: strh r3, [r0, #52]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,52)?)?;
    // VMA 0xc45a8: ldr r3, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,36)?)?;
    // VMA 0xc45ac: and r3, r2, r3, lsr #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,8,false)?)?;
    // VMA 0xc45b0: strh r3, [r0, #60]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,60)?)?;
    // VMA 0xc45b4: ldr r3, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,40)?)?;
    // VMA 0xc45b8: and r3, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,3)?)?;
    // VMA 0xc45bc: strh r3, [r0, #68]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,68)?)?;
    // VMA 0xc45c0: ldrb r3, [r1, #0x2b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,43)?)?;
    // VMA 0xc45c4: and r3, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,3)?)?;
    // VMA 0xc45c8: strh r3, [r0, #76]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,76)?)?;
    // VMA 0xc45cc: ldr r3, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,20)?)?;
    // VMA 0xc45d0: and r3, r2, r3, lsr #18
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,18,false)?)?;
    // VMA 0xc45d4: strh r3, [r0, #22]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,22)?)?;
    // VMA 0xc45d8: ldr r3, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,24)?)?;
    // VMA 0xc45dc: and r3, r2, r3, lsr #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,10,false)?)?;
    // VMA 0xc45e0: strh r3, [r0, #30]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,30)?)?;
    // VMA 0xc45e4: ldr r3, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,28)?)?;
    // VMA 0xc45e8: and r3, r2, r3, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,2,false)?)?;
    // VMA 0xc45ec: strh r3, [r0, #38]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,38)?)?;
    // VMA 0xc45f0: ldr r3, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,28)?)?;
    // VMA 0xc45f4: and r3, r2, r3, lsr #26
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,26,false)?)?;
    // VMA 0xc45f8: strh r3, [r0, #46]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,46)?)?;
    // VMA 0xc45fc: ldr r3, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,32)?)?;
    // VMA 0xc4600: and r3, r2, r3, lsr #18
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,18,false)?)?;
    // VMA 0xc4604: strh r3, [r0, #54]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,54)?)?;
    // VMA 0xc4608: ldr r3, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,36)?)?;
    // VMA 0xc460c: and r3, r2, r3, lsr #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,10,false)?)?;
    // VMA 0xc4610: strh r3, [r0, #62]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,62)?)?;
    // VMA 0xc4614: ldr r3, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,40)?)?;
    // VMA 0xc4618: and r3, r2, r3, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::new(3)?,Shift::Lsr,2,false)?)?;
    // VMA 0xc461c: strh r3, [r0, #70]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,70)?)?;
    // VMA 0xc4620: ldr r3, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,40)?)?;
    // VMA 0xc4624: and r2, r2, r3, lsr #26
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsr,26,false)?)?;
    // VMA 0xc4628: mov r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15)?)?;
    // VMA 0xc462c: strh r2, [r0, #78]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R0,78)?)?;
    // VMA 0xc4630: ldr r2, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,20)?)?;
    // VMA 0xc4634: and r2, r3, r2, lsr #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,20,false)?)?;
    // VMA 0xc4638: str r2, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,80)?)?;
    // VMA 0xc463c: ldr r2, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,24)?)?;
    // VMA 0xc4640: and r2, r3, r2, lsr #12
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,12,false)?)?;
    // VMA 0xc4644: str r2, [r0, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,84)?)?;
    // VMA 0xc4648: ldr r2, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,28)?)?;
    // VMA 0xc464c: and r2, r3, r2, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,4,false)?)?;
    // VMA 0xc4650: str r2, [r0, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,88)?)?;
    // VMA 0xc4654: ldr r2, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,28)?)?;
    // VMA 0xc4658: lsr r2, r2, #28
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsr,28)?)?;
    // VMA 0xc465c: str r2, [r0, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,92)?)?;
    // VMA 0xc4660: ldr r2, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,32)?)?;
    // VMA 0xc4664: and r2, r3, r2, lsr #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,20,false)?)?;
    // VMA 0xc4668: str r2, [r0, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,96)?)?;
    // VMA 0xc466c: ldr r2, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,36)?)?;
    // VMA 0xc4670: and r2, r3, r2, lsr #12
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,12,false)?)?;
    // VMA 0xc4674: str r2, [r0, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,100)?)?;
    // VMA 0xc4678: ldr r2, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,40)?)?;
    // VMA 0xc467c: and r2, r3, r2, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsr,4,false)?)?;
    // VMA 0xc4680: str r2, [r0, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,104)?)?;
    // VMA 0xc4684: ldr r1, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,40)?)?;
    // VMA 0xc4688: lsr r1, r1, #28
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,28)?)?;
    // VMA 0xc468c: str r1, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,108)?)?;
    // VMA 0xc4690: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;

    c.check(0xc4694)
}
