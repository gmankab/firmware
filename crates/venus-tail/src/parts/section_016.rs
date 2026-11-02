
/// Finish large per-plane codec allocation, align DMA sizes and populate
/// device-visible stride tables. Follow-up callback getter is a separate
/// six-instruction tail function, with the intermediate controller math
/// and diagnostic constants explicitly represented as non-executable data.
fn finish_codec_allocator_and_callback_getter(c:&mut Code)->Result<(),String>{
    c.check(0xace00)?;
    // ARM 0xace00: add r5, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xace04: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xace08: beq 0xace1c <entry+0x6cc>
    c.b(C::Eq,0xace1c,false)?;
    // ARM 0xace0c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xace10: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xace14: sub r9, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(9)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xace18: b 0xace24 <entry+0x6d4>
    c.b(C::Al,0xace24,false)?;
    // ARM 0xace1c: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xace20: and r9, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xace24: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // ARM 0xace28: ldr r1, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,136)?)?;
    // ARM 0xace2c: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // ARM 0xace30: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xace34: add r5, r2, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsr,1,false)?)?;
    // ARM 0xace38: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xace3c: beq 0xace50 <entry+0x700>
    c.b(C::Eq,0xace50,false)?;
    // ARM 0xace40: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xace44: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xace48: sub r3, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xace4c: b 0xace58 <entry+0x708>
    c.b(C::Al,0xace58,false)?;
    // ARM 0xace50: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xace54: and r3, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xace58: ldr r0, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,140)?)?;
    // ARM 0xace5c: ldr r1, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,128)?)?;
    // ARM 0xace60: str r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // ARM 0xace64: ldr r7, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-40)?)?;
    // ARM 0xace68: ldr r12, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-60)?)?;
    // ARM 0xace6c: ldr r6, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,64)?)?;
    // ARM 0xace70: mla r2, r3, r9, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::new(3)?,R::new(9)?,R::R0)?)?;
    // ARM 0xace74: ldr r3, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,68)?)?;
    // ARM 0xace78: rsb r0, r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,0)?)?;
    // ARM 0xace7c: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // ARM 0xace80: and r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xace84: mla r2, r7, r3, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::new(7)?,R::new(3)?,R::R1)?)?;
    // ARM 0xace88: rsb r1, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R1,0)?)?;
    // ARM 0xace8c: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // ARM 0xace90: and r7, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xace94: add r5, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xace98: ldr r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,88)?)?;
    // ARM 0xace9c: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // ARM 0xacea0: and r3, r10, r12
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(10)?,R::R12,Shift::Lsl,0,false)?)?;
    // ARM 0xacea4: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // ARM 0xacea8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaceac: str r7, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-44)?)?;
    // ARM 0xaceb0: str r3, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,20)?)?;
    // ARM 0xaceb4: lsr r10, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(10)?,R::R1,Shift::Lsr,4)?)?;
    // ARM 0xaceb8: ldr r1, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-68)?)?;
    // ARM 0xacebc: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // ARM 0xacec0: str r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // ARM 0xacec4: ldr r1, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-64)?)?;
    // ARM 0xacec8: mul r3, r1, r6
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R1,R::new(6)?)?)?;
    // ARM 0xacecc: lsl r1, r6, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(6)?,Shift::Lsl,7)?)?;
    // ARM 0xaced0: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // ARM 0xaced4: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // ARM 0xaced8: str r3, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,28)?)?;
    // ARM 0xacedc: and r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacee0: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,64)?)?;
    // ARM 0xacee4: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xacee8: lsr r1, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,1)?)?;
    // ARM 0xaceec: str r1, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-60)?)?;
    // ARM 0xacef0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xacef4: str r5, [r4, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,336)?)?;
    // ARM 0xacef8: str r7, [r4, #0x29c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,668)?)?;
    // ARM 0xacefc: str r1, [r4, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,676)?)?;
    // ARM 0xacf00: str r9, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,40)?)?;
    // ARM 0xacf04: str r5, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-48)?)?;
    // ARM 0xacf08: str r10, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,36)?)?;
    // ARM 0xacf0c: beq 0xacf64 <entry+0x814>
    c.b(C::Eq,0xacf64,false)?;
    // ARM 0xacf10: ldr r6, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,60)?)?;
    // ARM 0xacf14: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // ARM 0xacf18: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // ARM 0xacf1c: moveq r1, #48
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,48)?)?;
    // ARM 0xacf20: add r0, r1, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacf24: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // ARM 0xacf28: blx 0x83c4c <entry+0xfffffffffffd74fc>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // ARM 0xacf2c: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,144)?)?;
    // ARM 0xacf30: mov r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16)?)?;
    // ARM 0xacf34: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // ARM 0xacf38: moveq r5, #24
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,24)?)?;
    // ARM 0xacf3c: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // ARM 0xacf40: add r7, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacf44: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xacf48: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacf4c: beq 0xacf74 <entry+0x824>
    c.b(C::Eq,0xacf74,false)?;
    // ARM 0xacf50: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xacf54: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM 0xacf58: bl 0x81850 <entry+0xfffffffffffd5100>
    c.b(C::Al,0x81850,true)?;
    // ARM 0xacf5c: sub r10, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(10)?,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacf60: b 0xacf80 <entry+0x830>
    c.b(C::Al,0xacf80,false)?;
    // ARM 0xacf64: ldr r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // ARM 0xacf68: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xacf6c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xacf70: b 0xad0a4 <entry+0x954>
    c.b(C::Al,0xad0a4,false)?;
    // ARM 0xacf74: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xacf78: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacf7c: and r10, r7, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(10)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacf80: ldr r2, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,60)?)?;
    // ARM 0xacf84: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // ARM 0xacf88: ldr r1, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,148)?)?;
    // ARM 0xacf8c: mov r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,7)?)?;
    // ARM 0xacf90: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // ARM 0xacf94: moveq r6, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // ARM 0xacf98: movne r2, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,3)?)?;
    // ARM 0xacf9c: add r0, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacfa0: mov r9, r2
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R2))?;
    // ARM 0xacfa4: add r0, r1, r0, lsr r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,R::R2,false)?)?;
    // ARM 0xacfa8: sub r7, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R0,1)?)?;
    // ARM 0xacfac: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacfb0: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacfb4: beq 0xacfc8 <entry+0x878>
    c.b(C::Eq,0xacfc8,false)?;
    // ARM 0xacfb8: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM 0xacfbc: bl 0x81850 <entry+0xfffffffffffd5100>
    c.b(C::Al,0x81850,true)?;
    // ARM 0xacfc0: sub r0, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacfc4: b 0xacfd0 <entry+0x880>
    c.b(C::Al,0xacfd0,false)?;
    // ARM 0xacfc8: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacfcc: and r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacfd0: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // ARM 0xacfd4: ldr r2, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // ARM 0xacfd8: orr r3, r3, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,3840)?)?;
    // ARM 0xacfdc: mla r1, r0, r10, r3
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::R0,R::new(10)?,R::new(3)?)?)?;
    // ARM 0xacfe0: ldr r0, [pc, #0x70c]
    c.literal(R::R0,0xad6f4)?;
    // ARM 0xacfe4: mov r10, r3
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(3)?))?;
    // ARM 0xacfe8: and r7, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacfec: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xacff0: add r0, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xacff4: str r7, [r4, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,676)?)?;
    // ARM 0xacff8: str r0, [r4, #0x29c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,668)?)?;
    // ARM 0xacffc: add r0, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,1)?)?;
    // ARM 0xad000: add r0, r5, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsr,1,false)?)?;
    // ARM 0xad004: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // ARM 0xad008: blx 0x83c4c <entry+0xfffffffffffd74fc>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // ARM 0xad00c: ldr r1, [r4, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,156)?)?;
    // ARM 0xad010: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // ARM 0xad014: add r5, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xad018: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad01c: beq 0xad030 <entry+0x8e0>
    c.b(C::Eq,0xad030,false)?;
    // ARM 0xad020: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xad024: bl 0x81850 <entry+0xfffffffffffd5100>
    c.b(C::Al,0x81850,true)?;
    // ARM 0xad028: sub r8, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(8)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad02c: b 0xad038 <entry+0x8e8>
    c.b(C::Al,0xad038,false)?;
    // ARM 0xad030: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xad034: and r8, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(8)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xad038: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // ARM 0xad03c: ldr r1, [r4, #0xa0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,160)?)?;
    // ARM 0xad040: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xad044: add r0, r6, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsr,1,false)?)?;
    // ARM 0xad048: add r0, r1, r0, lsr r9
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,R::new(9)?,false)?)?;
    // ARM 0xad04c: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // ARM 0xad050: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xad054: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad058: beq 0xad06c <entry+0x91c>
    c.b(C::Eq,0xad06c,false)?;
    // ARM 0xad05c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xad060: bl 0x81850 <entry+0xfffffffffffd5100>
    c.b(C::Al,0x81850,true)?;
    // ARM 0xad064: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad068: b 0xad074 <entry+0x924>
    c.b(C::Al,0xad074,false)?;
    // ARM 0xad06c: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xad070: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xad074: ldr r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // ARM 0xad078: ldr r2, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // ARM 0xad07c: ldr r3, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-48)?)?;
    // ARM 0xad080: mla r1, r0, r8, r10
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::R0,R::new(8)?,R::new(10)?)?)?;
    // ARM 0xad084: ldr r0, [pc, #0x668]
    c.literal(R::R0,0xad6f4)?;
    // ARM 0xad088: and r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xad08c: add r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad090: add r3, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad094: add r2, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad098: str r3, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-48)?)?;
    // ARM 0xad09c: str r3, [r4, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,336)?)?;
    // ARM 0xad0a0: str r2, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-44)?)?;
    // ARM 0xad0a4: add r0, r4, #2880
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,2880)?)?;
    // ARM 0xad0a8: bl 0xa9254 <entry+0xffffffffffffcb04>
    c.b(C::Al,0xa9254,true)?;
    // ARM 0xad0ac: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // ARM 0xad0b0: mov r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,255)?)?;
    // ARM 0xad0b4: orr r6, r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(5)?,8)?)?;
    // ARM 0xad0b8: ldr r0, [r4, #0xdc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,220)?)?;
    // ARM 0xad0bc: ldr r1, [r4, #0x23c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,572)?)?;
    // ARM 0xad0c0: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // ARM 0xad0c4: str r1, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,60)?)?;
    // ARM 0xad0c8: mov r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,65280)?)?;
    // ARM 0xad0cc: bic r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // ARM 0xad0d0: orr r1, r1, #4128768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4128768)?)?;
    // ARM 0xad0d4: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // ARM 0xad0d8: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // ARM 0xad0dc: str r0, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,416)?)?;
    // ARM 0xad0e0: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // ARM 0xad0e4: mov r0, #176
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,176)?)?;
    // ARM 0xad0e8: ldr r2, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,72)?)?;
    // ARM 0xad0ec: str r3, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-52)?)?;
    // ARM 0xad0f0: smlabb r0, r2, r0, r7
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::R2,R::R0,R::new(7)?)?)?;
    // ARM 0xad0f4: and r10, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(10)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad0f8: mov r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,65280)?)?;
    // ARM 0xad0fc: add r0, r7, r2, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R2,Shift::Lsl,6,false)?)?;
    // ARM 0xad100: mov r2, #64768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64768)?)?;
    // ARM 0xad104: orr r1, r1, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,458752)?)?;
    // ARM 0xad108: orr r2, r2, #2031616
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2031616)?)?;
    // ARM 0xad10c: str r10, [r4, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,428)?)?;
    // ARM 0xad110: add r1, r1, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,524288)?)?;
    // ARM 0xad114: and r8, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(8)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad118: add r0, r7, r6, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(6)?,Shift::Lsl,4,false)?)?;
    // ARM 0xad11c: and r9, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad120: add r0, r7, r6, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(6)?,Shift::Lsl,3,false)?)?;
    // ARM 0xad124: str r8, [r4, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,380)?)?;
    // ARM 0xad128: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // ARM 0xad12c: and r5, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad130: str r9, [r4, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,392)?)?;
    // ARM 0xad134: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // ARM 0xad138: str r5, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,400)?)?;
    // ARM 0xad13c: str r5, [r4, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,408)?)?;
    // ARM 0xad140: ldr r0, [r4, #0x2c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,712)?)?;
    // ARM 0xad144: add r6, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xad148: ldr r1, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-68)?)?;
    // ARM 0xad14c: mul r0, r6, r3
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::new(3)?)?)?;
    // ARM 0xad150: lsr r7, r1, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R1,Shift::Lsr,6)?)?;
    // ARM 0xad154: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xad158: blx 0x83c4c <entry+0xfffffffffffd74fc>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // ARM 0xad15c: str r0, [r4, #0x1f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,500)?)?;
    // ARM 0xad160: mul r0, r6, r10
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::new(10)?)?)?;
    // ARM 0xad164: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xad168: blx 0x83c4c <entry+0xfffffffffffd74fc>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // ARM 0xad16c: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // ARM 0xad170: mul r0, r6, r8
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::new(8)?)?)?;
    // ARM 0xad174: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xad178: blx 0x83c4c <entry+0xfffffffffffd74fc>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // ARM 0xad17c: str r0, [r4, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,484)?)?;
    // ARM 0xad180: mul r0, r6, r9
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::new(9)?)?)?;
    // ARM 0xad184: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xad188: blx 0x83c4c <entry+0xfffffffffffd74fc>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // ARM 0xad18c: str r0, [r4, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,488)?)?;
    // ARM 0xad190: mul r0, r6, r5
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(6)?,R::new(5)?)?)?;
    // ARM 0xad194: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xad198: blx 0x83c4c <entry+0xfffffffffffd74fc>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // ARM 0xad19c: str r0, [r4, #0x1ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,492)?)?;
    // ARM 0xad1a0: str r0, [r4, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,496)?)?;
    // ARM 0xad1a4: mov r3, #89
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,89)?)?;
    // ARM 0xad1a8: mov lr, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,65280)?)?;
    // ARM 0xad1ac: mov r12, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,255)?)?;
    // ARM 0xad1b0: ldr r6, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,36)?)?;
    // ARM 0xad1b4: ldr r7, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,72)?)?;
    // ARM 0xad1b8: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // ARM 0xad1bc: orr lr, lr, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,458752)?)?;
    // ARM 0xad1c0: ldr r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // ARM 0xad1c4: smulbb r0, r6, r7
    c.emit(arm32::signed_multiply_bottom_halves(C::Al,R::R0,R::new(6)?,R::new(7)?)?)?;
    // ARM 0xad1c8: add r1, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // ARM 0xad1cc: lsl r2, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsl,1)?)?;
    // ARM 0xad1d0: cmp r2, #600
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,600)?)?;
    // ARM 0xad1d4: sub r2, lr, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::LR,262144)?)?;
    // ARM 0xad1d8: lslls r3, r1, #1
    c.emit(arm32::mov_shifted_register(C::Ls,R::new(3)?,R::R1,Shift::Lsl,1)?)?;
    // ARM 0xad1dc: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // ARM 0xad1e0: add r1, r1, r3, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,7,false)?)?;
    // ARM 0xad1e4: and r2, r1, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad1e8: mov r1, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,254)?)?;
    // ARM 0xad1ec: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // ARM 0xad1f0: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // ARM 0xad1f4: sub r1, r5, #256
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(5)?,256)?)?;
    // ARM 0xad1f8: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad1fc: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // ARM 0xad200: stmib sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // ARM 0xad204: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad208: mov r2, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1016)?)?;
    // ARM 0xad20c: add r1, r1, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,1,false)?)?;
    // ARM 0xad210: str r8, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,72)?)?;
    // ARM 0xad214: orr r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3072)?)?;
    // ARM 0xad218: and r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad21c: ldr r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,64)?)?;
    // ARM 0xad220: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,1,false)?)?;
    // ARM 0xad224: str r1, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-68)?)?;
    // ARM 0xad228: add r0, r0, r1, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,6,false)?)?;
    // ARM 0xad22c: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // ARM 0xad230: add r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,255)?)?;
    // ARM 0xad234: and r1, r1, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad238: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // ARM 0xad23c: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad240: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // ARM 0xad244: add r10, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::R1,2)?)?;
    // ARM 0xad248: ldr r1, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,60)?)?;
    // ARM 0xad24c: cmp r10, #18
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,18)?)?;
    // ARM 0xad250: movge r10, #18
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::new(10)?,R::R0,18)?)?;
    // ARM 0xad254: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xad258: movne r10, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // ARM 0xad25c: mla r1, r10, r2, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::new(10)?,R::R2,R::R0)?)?;
    // ARM 0xad260: ldr r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // ARM 0xad264: mov r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // ARM 0xad268: smlabb r0, r0, r6, r2
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R0,R::R0,R::new(6)?,R::R2)?)?;
    // ARM 0xad26c: and r3, r0, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xad270: add r0, r7, r7, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // ARM 0xad274: mov r7, lr
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::LR))?;
    // ARM 0xad278: add r0, r2, r0, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,7,false)?)?;
    // ARM 0xad27c: mov r2, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,65280)?)?;
    // ARM 0xad280: str r3, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,44)?)?;
    // ARM 0xad284: orr r2, r2, #4128768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4128768)?)?;
    // ARM 0xad288: add r2, r2, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4194304)?)?;
    // ARM 0xad28c: and r2, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad290: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // ARM 0xad294: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // ARM 0xad298: add r9, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // ARM 0xad29c: add r0, r8, r9, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::new(9)?,Shift::Lsl,8,false)?)?;
    // ARM 0xad2a0: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad2a4: ldr r2, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-64)?)?;
    // ARM 0xad2a8: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,255)?)?;
    // ARM 0xad2ac: and r2, r2, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xad2b0: str r2, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-64)?)?;
    // ARM 0xad2b4: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad2b8: ldr r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // ARM 0xad2bc: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,255)?)?;
    // ARM 0xad2c0: and r2, r2, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad2c4: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad2c8: str r2, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,60)?)?;
    // ARM 0xad2cc: ldr r2, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,48)?)?;
    // ARM 0xad2d0: add r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad2d4: mov r3, #64768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,64768)?)?;
    // ARM 0xad2d8: orr r3, r3, #2031616
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2031616)?)?;
    // ARM 0xad2dc: add r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,512)?)?;
    // ARM 0xad2e0: smlabb r2, r2, r6, r12
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R2,R::R2,R::new(6)?,R::R12)?)?;
    // ARM 0xad2e4: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad2e8: ldr r3, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // ARM 0xad2ec: str r2, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,48)?)?;
    // ARM 0xad2f0: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad2f4: mov r2, #224
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,224)?)?;
    // ARM 0xad2f8: add r2, r2, r3, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Lsl,5,false)?)?;
    // ARM 0xad2fc: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // ARM 0xad300: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // ARM 0xad304: add r0, r0, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,1,false)?)?;
    // ARM 0xad308: ldr r2, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-52)?)?;
    // ARM 0xad30c: ldr r3, [r4, #0x25c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,604)?)?;
    // ARM 0xad310: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad314: ldr r2, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,352)?)?;
    // ARM 0xad318: add r0, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xad31c: ldr r1, [r4, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,308)?)?;
    // ARM 0xad320: mla r6, r1, r2, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::R1,R::R2,R::R0)?)?;
    // ARM 0xad324: add r0, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,255)?)?;
    // ARM 0xad328: add r8, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R0,512)?)?;
    // ARM 0xad32c: ldr r0, [pc, #0x3cc]
    c.literal(R::R0,0xad700)?;
    // ARM 0xad330: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xad334: ldr r2, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-60)?)?;
    // ARM 0xad338: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xad33c: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // ARM 0xad340: addlo r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Cc,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xad344: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // ARM 0xad348: and r6, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad34c: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // ARM 0xad350: lsrlo r0, r1, #2
    c.emit(arm32::mov_shifted_register(C::Cc,R::R0,R::R1,Shift::Lsr,2)?)?;
    // ARM 0xad354: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // ARM 0xad358: str r6, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,36)?)?;
    // ARM 0xad35c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xad360: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // ARM 0xad364: and r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad368: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // ARM 0xad36c: add r6, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xad370: ldr r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-48)?)?;
    // ARM 0xad374: rsb r7, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(7)?,R::R1,0)?)?;
    // ARM 0xad378: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad37c: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // ARM 0xad380: beq 0xad3a4 <entry+0xc54>
    c.b(C::Eq,0xad3a4,false)?;
    // ARM 0xad384: ldr r2, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-56)?)?;
    // ARM 0xad388: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xad38c: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // ARM 0xad390: bl 0xe01ec <entry+0x33a9c>
    c.b(C::Al,0xe01ec,true)?;
    // ARM 0xad394: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xad398: ldr r0, [r4, #0x25c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,604)?)?;
    // ARM 0xad39c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xad3a0: bne 0xad3b0 <entry+0xc60>
    c.b(C::Ne,0xad3b0,false)?;
    // ARM 0xad3a4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xad3a8: str r0, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,296)?)?;
    // ARM 0xad3ac: str r0, [r4, #0x148]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,328)?)?;
    // ARM 0xad3b0: lsl r0, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(6)?,Shift::Lsl,1)?)?;
    // ARM 0xad3b4: ldr r2, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-60)?)?;
    // ARM 0xad3b8: bic lr, r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::LR,R::new(8)?,255)?)?;
    // ARM 0xad3bc: and r3, r5, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad3c0: lsl r8, r9, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::new(9)?,Shift::Lsl,7)?)?;
    // ARM 0xad3c4: str r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-48)?)?;
    // ARM 0xad3c8: ldr r0, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-68)?)?;
    // ARM 0xad3cc: ldr r6, [r4, #0x2c4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,708)?)?;
    // ARM 0xad3d0: lsl r12, r0, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R12,R::R0,Shift::Lsl,6)?)?;
    // ARM 0xad3d4: lsl r0, r9, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(9)?,Shift::Lsl,6)?)?;
    // ARM 0xad3d8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // ARM 0xad3dc: beq 0xad3f0 <entry+0xca0>
    c.b(C::Eq,0xad3f0,false)?;
    // ARM 0xad3e0: ldr r7, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,292)?)?;
    // ARM 0xad3e4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM 0xad3e8: cmp r10, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad3ec: bgt 0xad550 <entry+0xe00>
    c.b(C::Gt,0xad550,false)?;
    // ARM 0xad3f0: ldr r7, [r4, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,324)?)?;
    // ARM 0xad3f4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM 0xad3f8: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad3fc: bgt 0xad550 <entry+0xe00>
    c.b(C::Gt,0xad550,false)?;
    // ARM 0xad400: ldr r1, [r4, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,544)?)?;
    // ARM 0xad404: ldr r7, [r4, #0x21c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,540)?)?;
    // ARM 0xad408: add r1, r1, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xad40c: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad410: bgt 0xad550 <entry+0xe00>
    c.b(C::Gt,0xad550,false)?;
    // ARM 0xad414: ldr r1, [r4, #0x214]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,532)?)?;
    // ARM 0xad418: ldr r5, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-48)?)?;
    // ARM 0xad41c: cmp r5, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad420: bgt 0xad550 <entry+0xe00>
    c.b(C::Gt,0xad550,false)?;
    // ARM 0xad424: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,64)?)?;
    // ARM 0xad428: ldr r6, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,284)?)?;
    // ARM 0xad42c: ldr r7, [r4, #0x158]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,344)?)?;
    // ARM 0xad430: ldr r2, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,348)?)?;
    // ARM 0xad434: mul r5, r6, r7
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::new(6)?,R::new(7)?)?)?;
    // ARM 0xad438: mul r9, r10, r1
    c.emit(arm32::multiply_register(C::Al,R::new(9)?,R::new(10)?,R::R1)?)?;
    // ARM 0xad43c: str r2, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-68)?)?;
    // ARM 0xad440: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // ARM 0xad444: cmp r9, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad448: ldr r5, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-68)?)?;
    // ARM 0xad44c: ldr r2, [r4, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,360)?)?;
    // ARM 0xad450: add r9, r4, #432
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(4)?,432)?)?;
    // ARM 0xad454: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad458: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad45c: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // ARM 0xad460: ldr r7, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,388)?)?;
    // ARM 0xad464: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad468: cmp r1, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad46c: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // ARM 0xad470: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad474: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xad478: ldr r1, [r4, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,364)?)?;
    // ARM 0xad47c: ldr r2, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,36)?)?;
    // ARM 0xad480: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad484: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad488: ldr r1, [r4, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,368)?)?;
    // ARM 0xad48c: ldr r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    // ARM 0xad490: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad494: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad498: ldr r1, [r4, #0x174]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,372)?)?;
    // ARM 0xad49c: ldr r2, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,72)?)?;
    // ARM 0xad4a0: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad4a4: cmp r12, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad4a8: ldr r1, [r4, #0x178]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,376)?)?;
    // ARM 0xad4ac: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad4b0: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad4b4: ldr r1, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,384)?)?;
    // ARM 0xad4b8: ldr r2, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,48)?)?;
    // ARM 0xad4bc: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad4c0: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad4c4: ldr r1, [r4, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,396)?)?;
    // ARM 0xad4c8: ldr r2, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-52)?)?;
    // ARM 0xad4cc: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad4d0: cmp r8, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad4d4: ldr r7, [r4, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,424)?)?;
    // ARM 0xad4d8: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad4dc: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad4e0: ldr r1, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,404)?)?;
    // ARM 0xad4e4: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad4e8: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad4ec: ldr r1, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,412)?)?;
    // ARM 0xad4f0: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad4f4: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad4f8: ldr r1, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,420)?)?;
    // ARM 0xad4fc: ldr r2, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // ARM 0xad500: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad504: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad508: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // ARM 0xad50c: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad510: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad514: ldm r9, {r1, r7, r9}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(9)?,R::R1.bit() | R::new(7)?.bit() | R::new(9)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xad518: ldr r2, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-64)?)?;
    // ARM 0xad51c: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad520: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xad524: ldr r1, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,60)?)?;
    // ARM 0xad528: ldr r2, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-60)?)?;
    // ARM 0xad52c: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad530: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad534: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // ARM 0xad538: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad53c: cmp r1, r9
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad540: ldr r1, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,0)?)?;
    // ARM 0xad544: movgt r6, #2
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xad548: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xad54c: movne r6, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM 0xad550: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // ARM 0xad554: mov r5, #921600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,921600)?)?;
    // ARM 0xad558: str r1, [r4, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,228)?)?;
    // ARM 0xad55c: ldr r1, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // ARM 0xad560: str r1, [r4, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,232)?)?;
    // ARM 0xad564: ldr r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // ARM 0xad568: str r1, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,236)?)?;
    // ARM 0xad56c: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // ARM 0xad570: str r1, [r4, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,240)?)?;
    // ARM 0xad574: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // ARM 0xad578: str r1, [r4, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,244)?)?;
    // ARM 0xad57c: str r2, [r4, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,356)?)?;
    // ARM 0xad580: str r10, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,284)?)?;
    // ARM 0xad584: ldr r1, [r4, #0x224]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,548)?)?;
    // ARM 0xad588: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xad58c: moveq r1, #53504
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,53504)?)?;
    // ARM 0xad590: orreq r1, r1, #65536
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R1,R::R1,65536)?)?;
    // ARM 0xad594: streq r1, [r4, #0x224]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(4)?,548)?)?;
    // ARM 0xad598: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,56)?)?;
    // ARM 0xad59c: ldr r7, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,52)?)?;
    // ARM 0xad5a0: ldrb r1, [r1, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,165)?)?;
    // ARM 0xad5a4: lsl r1, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,1)?)?;
    // ARM 0xad5a8: cmp r7, r5, lsr r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsr,R::R1,false)?)?;
    // ARM 0xad5ac: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xad5b0: ldr r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // ARM 0xad5b4: lsrge r2, r1, #2
    c.emit(arm32::mov_shifted_register(C::Ge,R::R2,R::R1,Shift::Lsr,2)?)?;
    // ARM 0xad5b8: add r1, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,255)?)?;
    // ARM 0xad5bc: and r1, r1, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xad5c0: str r1, [r4, #0x13c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,316)?)?;
    // ARM 0xad5c4: ldr r1, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-56)?)?;
    // ARM 0xad5c8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xad5cc: cmpeq r6, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // ARM 0xad5d0: bne 0xad680 <entry+0xf30>
    c.b(C::Ne,0xad680,false)?;
    // ARM 0xad5d4: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,64)?)?;
    // ARM 0xad5d8: str r1, [r4, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,344)?)?;
    // ARM 0xad5dc: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // ARM 0xad5e0: str r1, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,348)?)?;
    // ARM 0xad5e4: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // ARM 0xad5e8: str r1, [r4, #0x168]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,360)?)?;
    // ARM 0xad5ec: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // ARM 0xad5f0: str r1, [r4, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,364)?)?;
    // ARM 0xad5f4: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // ARM 0xad5f8: str r1, [r4, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,368)?)?;
    // ARM 0xad5fc: str r12, [r4, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,372)?)?;
    // ARM 0xad600: ldr r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // ARM 0xad604: str r1, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,420)?)?;
    // ARM 0xad608: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // ARM 0xad60c: str r1, [r4, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,424)?)?;
    // ARM 0xad610: ldr r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,72)?)?;
    // ARM 0xad614: str r1, [r4, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,376)?)?;
    // ARM 0xad618: ldr r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,48)?)?;
    // ARM 0xad61c: str r1, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,384)?)?;
    // ARM 0xad620: str r8, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,388)?)?;
    // ARM 0xad624: str r0, [r4, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,396)?)?;
    // ARM 0xad628: str r0, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,404)?)?;
    // ARM 0xad62c: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // ARM 0xad630: str r0, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,412)?)?;
    // ARM 0xad634: ldr r0, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-64)?)?;
    // ARM 0xad638: str r0, [r4, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,432)?)?;
    // ARM 0xad63c: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,60)?)?;
    // ARM 0xad640: str r0, [r4, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,436)?)?;
    // ARM 0xad644: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // ARM 0xad648: str r0, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,508)?)?;
    // ARM 0xad64c: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // ARM 0xad650: str r0, [r4, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,440)?)?;
    // ARM 0xad654: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xad658: str r0, [r4, #0x114]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,276)?)?;
    // ARM 0xad65c: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // ARM 0xad660: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xad664: bgt 0xad694 <entry+0xf44>
    c.b(C::Gt,0xad694,false)?;
    // ARM 0xad668: ldr r0, [r4, #0x23c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,572)?)?;
    // ARM 0xad66c: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // ARM 0xad670: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xad674: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xad678: str r1, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,280)?)?;
    // ARM 0xad67c: b 0xad694 <entry+0xf44>
    c.b(C::Al,0xad694,false)?;
    // ARM 0xad680: ldr r1, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-56)?)?;
    // ARM 0xad684: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xad688: bne 0xad6c4 <entry+0xf74>
    c.b(C::Ne,0xad6c4,false)?;
    // ARM 0xad68c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // ARM 0xad690: beq 0xad6c4 <entry+0xf74>
    c.b(C::Eq,0xad6c4,false)?;
    // ARM 0xad694: ldr r0, [r4, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2444)?)?;
    // ARM 0xad698: ldr r1, [pc, #0x64]
    c.literal(R::R1,0xad704)?;
    // ARM 0xad69c: ldrh r3, [r0, #100]
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::R0,100)?)?;
    // ARM 0xad6a0: ldrh r2, [r0, #98]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::R0,98)?)?;
    // ARM 0xad6a4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xad6a8: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xad6ac: ldr r1, [pc, #0x54]
    c.literal(R::R1,0xad708)?;
    // ARM 0xad6b0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xad6b4: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xad6b8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xad6bc: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xad6c0: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xad6c4: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // ARM 0xad6c8: str r1, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,348)?)?;
    // ARM 0xad6cc: str r3, [r4, #0x140]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,320)?)?;
    // ARM 0xad6d0: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,64)?)?;
    // ARM 0xad6d4: str r1, [r4, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,344)?)?;
    // ARM 0xad6d8: ldr r1, [r4, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,544)?)?;
    // ARM 0xad6dc: add r1, r1, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xad6e0: str r1, [r4, #0x218]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,536)?)?;
    // ARM 0xad6e4: ldr r1, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // ARM 0xad6e8: str r1, [r4, #0x210]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,528)?)?;
    // ARM 0xad6ec: b 0xad5e4 <entry+0xe94>
    c.b(C::Al,0xad5e4,false)?;
    c.check(0xad6f0)?;
    const SCALING_AND_DIAGNOSTIC_WORDS:[u32;7]=[
        0x7fff_ff00, 0xffff_f000, 0xaaaa_aaab, 0x5555_5556,
        0x002b_2002, 0x000f_8650, 0x000e_2788,
    ];
    for constant in SCALING_AND_DIAGNOSTIC_WORDS {c.emit(constant)?;}
    c.check(0xad70c)?;
    // ARM 0xad70c: ldr r1, [pc, #0x10]
    c.literal(R::R1,0xad724)?;
    // ARM 0xad710: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // ARM 0xad714: ldr r1, [r0, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2444)?)?;
    // ARM 0xad718: add r0, r0, #2880
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2880)?)?;
    // ARM 0xad71c: ldrb r1, [r1, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,123)?)?;
    // ARM 0xad720: bx r2
    c.emit(arm32::bx(C::Al,R::R2))?;
    c.check(0xad724)?;
    c.emit(0x0010_dd70)?; // controller callback pointer literal, not code.
    c.check(0xad728)
}
