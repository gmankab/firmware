
/// Initialize device-backed controller records, reconcile plane descriptor
/// counters, and conditionally submit codec resource configuration. Runtime
/// pool at 0xb0444 points to the following helper; second pool is a logging
/// constant. These are *data* rather than ARM instructions.
fn prepare_codec_record_and_guard_callbacks(c:&mut Code)->Result<(),String>{
    c.check(0xb02b0)?;
    // ARM 0xb02b0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb02b4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM 0xb02b8: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // ARM 0xb02bc: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xb02c0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb02c4: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xb02c8: mov r0, #1824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1824)?)?;
    // ARM 0xb02cc: bl 0x2f28 <entry+0xfffffffffff567d8>
    c.b(C::Al,0x2f28,true)?;
    // ARM 0xb02d0: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xb02d4: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // ARM 0xb02d8: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // ARM 0xb02dc: beq 0xb0418 <entry+0x3cc8>
    c.b(C::Eq,0xb0418,false)?;
    // ARM 0xb02e0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xb02e4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb02e8: mov r2, #1824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1824)?)?;
    // ARM 0xb02ec: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM 0xb02f0: bl 0x31ec <entry+0xfffffffffff56a9c>
    c.b(C::Al,0x31ec,true)?;
    // ARM 0xb02f4: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // ARM 0xb02f8: bl 0x2f28 <entry+0xfffffffffff567d8>
    c.b(C::Al,0x2f28,true)?;
    // ARM 0xb02fc: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // ARM 0xb0300: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // ARM 0xb0304: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // ARM 0xb0308: beq 0xb0418 <entry+0x3cc8>
    c.b(C::Eq,0xb0418,false)?;
    // ARM 0xb030c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xb0310: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb0314: mov r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // ARM 0xb0318: bl 0x31ec <entry+0xfffffffffff56a9c>
    c.b(C::Al,0x31ec,true)?;
    // ARM 0xb031c: mov r0, #77
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,77)?)?;
    // ARM 0xb0320: mov r12, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3)?)?;
    // ARM 0xb0324: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // ARM 0xb0328: mov r3, #176
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,176)?)?;
    // ARM 0xb032c: mov r1, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
    // ARM 0xb0330: str r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,88)?)?;
    // ARM 0xb0334: mov r0, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,153)?)?;
    // ARM 0xb0338: strb r0, [r4, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3)?)?;
    // ARM 0xb033c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb0340: strb r7, [r4, #0x5c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,92)?)?;
    // ARM 0xb0344: strb r7, [r4, #0x5f]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,95)?)?;
    // ARM 0xb0348: strb r0, [r4, #0x5e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,94)?)?;
    // ARM 0xb034c: strb r0, [r4, #0x60]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,96)?)?;
    // ARM 0xb0350: strb r0, [r4, #0x260]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,608)?)?;
    // ARM 0xb0354: strb r7, [r4, #0xb0]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,176)?)?;
    // ARM 0xb0358: strb r12, [r4, #0xa4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(4)?,164)?)?;
    // ARM 0xb035c: strb r2, [r4, #0xa6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,166)?)?;
    // ARM 0xb0360: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // ARM 0xb0364: strb r2, [r4, #0x73]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,115)?)?;
    // ARM 0xb0368: strh r3, [r4, #98]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::new(4)?,98)?)?;
    // ARM 0xb036c: mov r3, #144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,144)?)?;
    // ARM 0xb0370: strh r3, [r4, #100]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::new(4)?,100)?)?;
    // ARM 0xb0374: strb r2, [r4, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,165)?)?;
    // ARM 0xb0378: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // ARM 0xb037c: mov r3, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,616)?)?;
    // ARM 0xb0380: strb r2, [r4, #0xa7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,167)?)?;
    // ARM 0xb0384: strh r1, [r4, r3]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(4)?,R::new(3)?)?)?;
    // ARM 0xb0388: mov r1, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,106)?)?;
    // ARM 0xb038c: mov r3, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,9)?)?;
    // ARM 0xb0390: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // ARM 0xb0394: strh r3, [r4, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::new(3)?,R::new(4)?,R::R1)?)?;
    // ARM 0xb0398: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM 0xb039c: strb r12, [r4, #0xa8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(4)?,168)?)?;
    // ARM 0xb03a0: strb r1, [r4, #0xa9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,169)?)?;
    // ARM 0xb03a4: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // ARM 0xb03a8: strb r1, [r4, #0xaa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,170)?)?;
    // ARM 0xb03ac: ldr r1, [pc, #0x90]
    c.literal(R::R1,0xb0444)?;
    // ARM 0xb03b0: strb r2, [r4, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,123)?)?;
    // ARM 0xb03b4: strb r2, [r4, #0x83]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(4)?,131)?)?;
    // ARM 0xb03b8: str r2, [r4, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,160)?)?;
    // ARM 0xb03bc: strb r7, [r4, #0x262]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,610)?)?;
    // ARM 0xb03c0: str r1, [r4, #0x26c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,620)?)?;
    // ARM 0xb03c4: mov r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // ARM 0xb03c8: str r4, [r5, #0x98c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,2444)?)?;
    // ARM 0xb03cc: str r4, [r5, #0xe74]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,3700)?)?;
    // ARM 0xb03d0: strb r7, [r6]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // ARM 0xb03d4: strb r7, [r6, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(6)?,1)?)?;
    // ARM 0xb03d8: strb r1, [r6, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,4)?)?;
    // ARM 0xb03dc: strb r1, [r6, #0x5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,5)?)?;
    // ARM 0xb03e0: mov r1, #26
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,26)?)?;
    // ARM 0xb03e4: strb r7, [r6, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(6)?,14)?)?;
    // ARM 0xb03e8: strb r7, [r6, #0xf]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(6)?,15)?)?;
    // ARM 0xb03ec: strb r1, [r6, #0x6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,6)?)?;
    // ARM 0xb03f0: strb r0, [r6, #0x22]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,34)?)?;
    // ARM 0xb03f4: strb r0, [r6, #0x7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,7)?)?;
    // ARM 0xb03f8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xb03fc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb0400: strb r7, [r6, #0x27]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(6)?,39)?)?;
    // ARM 0xb0404: str r6, [r5, #0x990]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,2448)?)?;
    // ARM 0xb0408: str r6, [r5, #0xf7c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,3964)?)?;
    // ARM 0xb040c: bl 0xaf694 <entry+0x2f44>
    c.b(C::Al,0xaf694,true)?;
    // ARM 0xb0410: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb0414: beq 0xb0434 <entry+0x3ce4>
    c.b(C::Eq,0xb0434,false)?;
    // ARM 0xb0418: mov r0, sp
    c.emit(arm32::mov_register(C::Al,R::R0,R::SP))?;
    // ARM 0xb041c: bl 0x3040 <entry+0xfffffffffff568f0>
    c.b(C::Al,0x3040,true)?;
    // ARM 0xb0420: add r0, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,4)?)?;
    // ARM 0xb0424: bl 0x3040 <entry+0xfffffffffff568f0>
    c.b(C::Al,0x3040,true)?;
    // ARM 0xb0428: mov r0, #45
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,45)?)?;
    // ARM 0xb042c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // ARM 0xb0430: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xb0434: ldr r1, [r4, #0xb8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,184)?)?;
    // ARM 0xb0438: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb043c: str r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    // ARM 0xb0440: b 0xb042c <entry+0x3cdc>
    c.b(C::Al,0xb042c,false)?;
    c.check(0xb0444)?;
    c.emit(0x000b_0448)?; // ARM 0xb0444: next helper address/data, not executable.
    c.check(0xb0448)?;
    // ARM 0xb0448: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb044c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM 0xb0450: ldrb r2, [r0, #0xa21]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2593)?)?;
    // ARM 0xb0454: ldr r12, [r0, #0xa34]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,2612)?)?;
    // ARM 0xb0458: ldr r1, [r0, #0xae8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2792)?)?;
    // ARM 0xb045c: ldr r3, [r0, #0xaec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,2796)?)?;
    // ARM 0xb0460: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM 0xb0464: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xb0468: ldr r2, [r0, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2444)?)?;
    // ARM 0xb046c: movne r3, r4
    c.emit(arm32::mov_register(C::Ne,R::new(3)?,R::new(4)?))?;
    // ARM 0xb0470: movne r1, r4
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(4)?))?;
    // ARM 0xb0474: cmp r12, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0478: bge 0xb0494 <entry+0x3d44>
    c.b(C::Ge,0xb0494,false)?;
    // ARM 0xb047c: ldrb lr, [r2, #0x73]
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::R2,115)?)?;
    // ARM 0xb0480: sub r5, r3, r12
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(5)?,R::new(3)?,R::R12,Shift::Lsl,0,false)?)?;
    // ARM 0xb0484: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // ARM 0xb0488: sub r6, lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::LR,1)?)?;
    // ARM 0xb048c: cmp r5, r4, lsl r6
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(4)?,Shift::Lsl,R::new(6)?,false)?)?;
    // ARM 0xb0490: bge 0xb04b8 <entry+0x3d68>
    c.b(C::Ge,0xb04b8,false)?;
    // ARM 0xb0494: cmp r12, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0498: ble 0xb04bc <entry+0x3d6c>
    c.b(C::Le,0xb04bc,false)?;
    // ARM 0xb049c: ldrb r2, [r2, #0x73]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,115)?)?;
    // ARM 0xb04a0: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // ARM 0xb04a4: sub r3, r12, r3
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::R12,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb04a8: sub r5, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R2,1)?)?;
    // ARM 0xb04ac: cmp r3, r6, lsl r5
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,R::new(5)?,false)?)?;
    // ARM 0xb04b0: subgt r1, r1, r6, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Gt,Op::Sub,R::R1,R::R1,R::new(6)?,Shift::Lsl,R::R2,false)?)?;
    // ARM 0xb04b4: b 0xb04bc <entry+0x3d6c>
    c.b(C::Al,0xb04bc,false)?;
    // ARM 0xb04b8: add r1, r1, r4, lsl lr
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R1,R::R1,R::new(4)?,Shift::Lsl,R::LR,false)?)?;
    // ARM 0xb04bc: ldrb r2, [r0, #0xa22]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2594)?)?;
    // ARM 0xb04c0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xb04c4: beq 0xb04d0 <entry+0x3d80>
    c.b(C::Eq,0xb04d0,false)?;
    // ARM 0xb04c8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb04cc: b 0xb04e8 <entry+0x3d98>
    c.b(C::Al,0xb04e8,false)?;
    // ARM 0xb04d0: ldrb r2, [r0, #0xa25]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2597)?)?;
    // ARM 0xb04d4: cmp r2, #21
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,21)?)?;
    // ARM 0xb04d8: bne 0xb04e8 <entry+0x3d98>
    c.b(C::Ne,0xb04e8,false)?;
    // ARM 0xb04dc: ldrb r2, [r0, #0xa00]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2560)?)?;
    // ARM 0xb04e0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xb04e4: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb04e8: add r2, r1, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // ARM 0xb04ec: str r2, [r0, #0xae4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,2788)?)?;
    // ARM 0xb04f0: ldrb r2, [r0, #0xa24]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2596)?)?;
    // ARM 0xb04f4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xb04f8: bne 0xb0528 <entry+0x3dd8>
    c.b(C::Ne,0xb0528,false)?;
    // ARM 0xb04fc: ldrb r2, [r0, #0xa25]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2597)?)?;
    // ARM 0xb0500: and r3, r2, #241
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R2,241)?)?;
    // ARM 0xb0504: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // ARM 0xb0508: andne r3, r2, #248
    c.emit(arm32::data_immediate(C::Ne,Op::And,R::new(3)?,R::R2,248)?)?;
    // ARM 0xb050c: cmpne r3, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(3)?,16)?)?;
    // ARM 0xb0510: bne 0xb0528 <entry+0x3dd8>
    c.b(C::Ne,0xb0528,false)?;
    // ARM 0xb0514: cmp r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,7)?)?;
    // ARM 0xb0518: beq 0xb0528 <entry+0x3dd8>
    c.b(C::Eq,0xb0528,false)?;
    // ARM 0xb051c: cmp r2, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,9)?)?;
    // ARM 0xb0520: strne r1, [r0, #0xae8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,2792)?)?;
    // ARM 0xb0524: strne r12, [r0, #0xaec]
    c.emit(arm32::transfer_word(C::Ne,false,R::R12,R::R0,2796)?)?;
    // ARM 0xb0528: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb052c: strb r1, [r0, #0xa00]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2560)?)?;
    // ARM 0xb0530: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb0534: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xb0538: add r1, r0, #3632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,3632)?)?;
    // ARM 0xb053c: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // ARM 0xb0540: ldr r3, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,0)?)?;
    // ARM 0xb0544: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xb0548: bne 0xb0560 <entry+0x3e10>
    c.b(C::Ne,0xb0560,false)?;
    // ARM 0xb054c: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // ARM 0xb0550: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // ARM 0xb0554: cmp r2, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,14)?)?;
    // ARM 0xb0558: ble 0xb0540 <entry+0x3df0>
    c.b(C::Le,0xb0540,false)?;
    // ARM 0xb055c: b 0xb0588 <entry+0x3e38>
    c.b(C::Al,0xb0588,false)?;
    // ARM 0xb0560: ldr r1, [r0, #0xe74]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,3700)?)?;
    // ARM 0xb0564: str r3, [r0, #0x98c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,2444)?)?;
    // ARM 0xb0568: ldrh r2, [r3, #98]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(3)?,98)?)?;
    // ARM 0xb056c: strh r2, [r1, #98]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R1,98)?)?;
    // ARM 0xb0570: ldrh r2, [r3, #100]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(3)?,100)?)?;
    // ARM 0xb0574: strh r2, [r1, #100]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::R1,100)?)?;
    // ARM 0xb0578: ldrb r2, [r3, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(3)?,165)?)?;
    // ARM 0xb057c: strb r2, [r1, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,165)?)?;
    // ARM 0xb0580: ldrb r2, [r3, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(3)?,123)?)?;
    // ARM 0xb0584: strb r2, [r1, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R1,123)?)?;
    // ARM 0xb0588: add r1, r0, #632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,632)?)?;
    // ARM 0xb058c: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // ARM 0xb0590: add r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,3072)?)?;
    // ARM 0xb0594: b 0xb05a8 <entry+0x3e58>
    c.b(C::Al,0xb05a8,false)?;
    // ARM 0xb0598: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // ARM 0xb059c: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // ARM 0xb05a0: cmp r2, #62
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,62)?)?;
    // ARM 0xb05a4: bxgt lr
    c.emit(arm32::bx(C::Gt,R::LR))?;
    // ARM 0xb05a8: ldr r3, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,0)?)?;
    // ARM 0xb05ac: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xb05b0: beq 0xb0598 <entry+0x3e48>
    c.b(C::Eq,0xb0598,false)?;
    // ARM 0xb05b4: str r3, [r0, #0x990]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,2448)?)?;
    // ARM 0xb05b8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // ARM 0xb05bc: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb05c0: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM 0xb05c4: ldr r2, [pc, #0x10c]
    c.literal(R::R2,0xb06d8)?;
    // ARM 0xb05c8: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // ARM 0xb05cc: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xb05d0: mov r1, #1120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1120)?)?;
    // ARM 0xb05d4: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // ARM 0xb05d8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xb05dc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb05e0: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xb05e4: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // ARM 0xb05e8: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb05ec: popeq {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xb05f0: ldrb r12, [r4, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(4)?,110)?)?;
    // ARM 0xb05f4: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // ARM 0xb05f8: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM 0xb05fc: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // ARM 0xb0600: bne 0xb0610 <entry+0x3ec0>
    c.b(C::Ne,0xb0610,false)?;
    // ARM 0xb0604: ldrb r3, [r4, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,111)?)?;
    // ARM 0xb0608: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xb060c: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM 0xb0610: ldrb r2, [r5, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,110)?)?;
    // ARM 0xb0614: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xb0618: bne 0xb0628 <entry+0x3ed8>
    c.b(C::Ne,0xb0628,false)?;
    // ARM 0xb061c: ldrb lr, [r5, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::new(5)?,111)?)?;
    // ARM 0xb0620: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // ARM 0xb0624: movne lr, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::LR,R::R0,1)?)?;
    // ARM 0xb0628: ldrh r1, [r5, #98]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,98)?)?;
    // ARM 0xb062c: ldrh r0, [r4, #98]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(4)?,98)?)?;
    // ARM 0xb0630: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb0634: bne 0xb0690 <entry+0x3f40>
    c.b(C::Ne,0xb0690,false)?;
    // ARM 0xb0638: ldrh r7, [r5, #100]
    c.emit(arm32::load_halfword(C::Al,R::new(7)?,R::new(5)?,100)?)?;
    // ARM 0xb063c: ldrh r6, [r4, #100]
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::new(4)?,100)?)?;
    // ARM 0xb0640: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0644: bne 0xb069c <entry+0x3f4c>
    c.b(C::Ne,0xb069c,false)?;
    // ARM 0xb0648: ldrb r0, [r5, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,165)?)?;
    // ARM 0xb064c: ldrb r7, [r4, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,165)?)?;
    // ARM 0xb0650: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0654: bne 0xb0698 <entry+0x3f48>
    c.b(C::Ne,0xb0698,false)?;
    // ARM 0xb0658: ldrb r0, [r5, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,123)?)?;
    // ARM 0xb065c: ldrb r7, [r4, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,123)?)?;
    // ARM 0xb0660: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0664: bne 0xb0698 <entry+0x3f48>
    c.b(C::Ne,0xb0698,false)?;
    // ARM 0xb0668: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xb066c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xb0670: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb0674: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM 0xb0678: tst r3, lr
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xb067c: bne 0xb0698 <entry+0x3f48>
    c.b(C::Ne,0xb0698,false)?;
    // ARM 0xb0680: orrs r3, r3, lr
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::LR,Shift::Lsl,0,true)?)?;
    // ARM 0xb0684: mov r7, r6
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // ARM 0xb0688: bne 0xb06bc <entry+0x3f6c>
    c.b(C::Ne,0xb06bc,false)?;
    // ARM 0xb068c: b 0xb069c <entry+0x3f4c>
    c.b(C::Al,0xb069c,false)?;
    // ARM 0xb0690: ldrh r7, [r5, #100]
    c.emit(arm32::load_halfword(C::Al,R::new(7)?,R::new(5)?,100)?)?;
    // ARM 0xb0694: b 0xb069c <entry+0x3f4c>
    c.b(C::Al,0xb069c,false)?;
    // ARM 0xb0698: mov r7, r6
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // ARM 0xb069c: strh r1, [r4, #98]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(4)?,98)?)?;
    // ARM 0xb06a0: strh r7, [r4, #100]
    c.emit(arm32::store_halfword(C::Al,R::new(7)?,R::new(4)?,100)?)?;
    // ARM 0xb06a4: ldrb r0, [r5, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,165)?)?;
    // ARM 0xb06a8: strb r0, [r4, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,165)?)?;
    // ARM 0xb06ac: ldrb r0, [r5, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,123)?)?;
    // ARM 0xb06b0: strb r0, [r4, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,123)?)?;
    // ARM 0xb06b4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb06b8: ldrb r2, [r5, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,110)?)?;
    // ARM 0xb06bc: cmp r12, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb06c0: strbne r2, [r4, #0x6e]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R2,R::new(4)?,110)?)?;
    // ARM 0xb06c4: ldrb r1, [r5, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,111)?)?;
    // ARM 0xb06c8: ldrb r2, [r4, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,111)?)?;
    // ARM 0xb06cc: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb06d0: strbne r1, [r4, #0x6f]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(4)?,111)?)?;
    // ARM 0xb06d4: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb06d8)?;
    c.emit(0x000f_88c7)?; // ARM 0xb06d8: diagnostic message pointer/data.
    c.check(0xb06dc)?;
    // ARM 0xb06dc: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb06e0: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xb06e4: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // ARM 0xb06e8: ldr r4, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,36)?)?;
    // ARM 0xb06ec: mov r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // ARM 0xb06f0: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // ARM 0xb06f4: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // ARM 0xb06f8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xb06fc: bl 0xaf694 <entry+0x2f44>
    c.b(C::Al,0xaf694,true)?;
    // ARM 0xb0700: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb0704: beq 0xb0748 <entry+0x3ff8>
    c.b(C::Eq,0xb0748,false)?;
    // ARM 0xb0708: mov r1, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,116)?)?;
    // ARM 0xb070c: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // ARM 0xb0710: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xb0714: orr r1, r1, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(4)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb0718: ldr r3, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb071c: tst r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,4)?)?;
    // ARM 0xb0720: bne 0xb0740 <entry+0x3ff0>
    c.b(C::Ne,0xb0740,false)?;
    // ARM 0xb0724: lsl r3, r4, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(4)?,Shift::Lsl,15)?)?;
    // ARM 0xb0728: orr r3, r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4)?)?;
    // ARM 0xb072c: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // ARM 0xb0730: ldr r7, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb0734: ldr r7, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb0738: tst r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,4)?)?;
    // ARM 0xb073c: beq 0xb0730 <entry+0x3fe0>
    c.b(C::Eq,0xb0730,false)?;
    // ARM 0xb0740: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xb0744: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb0748)
}
