
/// Initialize the next codec allocator, combine configured pixel dimensions, compute scaled plane sizes and perform checked 64-bit arithmetic.
/// Runtime slice 0xac9f0..0xace00, with 0 classified trailing data words.
fn begin_codec_allocation_and_scaled_dimensions(c:&mut Code)->Result<(),String>{
    c.check(0xac9f0)?;
    // ARM 0xac9f0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xac9f4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xac9f8: sub sp, sp, #116
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,116)?)?;
    // ARM 0xac9fc: str r1, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-56)?)?;
    // ARM 0xaca00: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xaca04: mov r1, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,106)?)?;
    // ARM 0xaca08: mov r0, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,616)?)?;
    // ARM 0xaca0c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xaca10: mvn r5, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(5)?,R::R0,127)?)?;
    // ARM 0xaca14: ldr r3, [r4, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2444)?)?;
    // ARM 0xaca18: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // ARM 0xaca1c: ldrh r0, [r3, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(3)?,R::R0)?)?;
    // ARM 0xaca20: ldrh r1, [r3, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(3)?,R::R1)?)?;
    // ARM 0xaca24: mul r7, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::R1,R::R0)?)?;
    // ARM 0xaca28: str r7, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,216)?)?;
    // ARM 0xaca2c: ldrb r0, [r3, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(3)?,165)?)?;
    // ARM 0xaca30: subs r1, r0, #4
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R1,R::R0,4,true)?)?;
    // ARM 0xaca34: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // ARM 0xaca38: movlo r1, #0
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xaca3c: str r1, [r4, #0x208]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,520)?)?;
    // ARM 0xaca40: mov r1, #942080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,942080)?)?;
    // ARM 0xaca44: cmp r7, r1, lsr r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsr,R::R0,false)?)?;
    // ARM 0xaca48: ldrh r9, [r3, #100]
    c.emit(arm32::load_halfword(C::Al,R::new(9)?,R::new(3)?,100)?)?;
    // ARM 0xaca4c: ldrh r8, [r3, #98]
    c.emit(arm32::load_halfword(C::Al,R::new(8)?,R::new(3)?,98)?)?;
    // ARM 0xaca50: movle r2, #1
    c.emit(arm32::data_immediate(C::Le,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM 0xaca54: str r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,20)?)?;
    // ARM 0xaca58: ldr r0, [r4, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,696)?)?;
    // ARM 0xaca5c: add r10, r8, #63
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(8)?,63)?)?;
    // ARM 0xaca60: add lr, r9, #63
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(9)?,63)?)?;
    // ARM 0xaca64: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM 0xaca68: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xaca6c: streq r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,20)?)?;
    // ARM 0xaca70: mov r0, #53504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,53504)?)?;
    // ARM 0xaca74: ldrb r2, [r3, #0xa7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(3)?,167)?)?;
    // ARM 0xaca78: ldr r6, [pc, #0xc70]
    c.literal(R::new(6)?,0xad6f0)?;
    // ARM 0xaca7c: orr r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65536)?)?;
    // ARM 0xaca80: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // ARM 0xaca84: mov r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,64)?)?;
    // ARM 0xaca88: cmp r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,64)?)?;
    // ARM 0xaca8c: blo 0xacaa0 <entry+0x350>
    c.b(C::Cc,0xacaa0,false)?;
    // ARM 0xaca90: strb r6, [r3, #0xa7]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(3)?,167)?)?;
    // ARM 0xaca94: mov r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,256)?)?;
    // ARM 0xaca98: mov r12, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xaca9c: b 0xacad4 <entry+0x384>
    c.b(C::Al,0xacad4,false)?;
    // ARM 0xacaa0: mov r12, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xacaa4: cmp r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // ARM 0xacaa8: bne 0xacab4 <entry+0x364>
    c.b(C::Ne,0xacab4,false)?;
    // ARM 0xacaac: mov r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // ARM 0xacab0: b 0xacad4 <entry+0x384>
    c.b(C::Al,0xacad4,false)?;
    // ARM 0xacab4: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // ARM 0xacab8: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // ARM 0xacabc: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM 0xacac0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xacac4: movls r12, #16
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xacac8: movls r6, #64
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(6)?,R::R0,64)?)?;
    // ARM 0xacacc: movls r2, #16
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R2,R::R0,16)?)?;
    // ARM 0xacad0: strbls r12, [r3, #0xa7]
    c.emit(arm32::transfer_byte(C::Ls,false,R::R12,R::new(3)?,167)?)?;
    // ARM 0xacad4: add r1, r0, #11968
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,11968)?)?;
    // ARM 0xacad8: add r0, r9, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,15)?)?;
    // ARM 0xacadc: str r0, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-44)?)?;
    // ARM 0xacae0: add r0, r8, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,15)?)?;
    // ARM 0xacae4: str r0, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-68)?)?;
    // ARM 0xacae8: and r0, r5, r10, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::new(10)?,Shift::Lsl,1,false)?)?;
    // ARM 0xacaec: lsr r5, lr, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::LR,Shift::Lsr,6)?)?;
    // ARM 0xacaf0: str r0, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-64)?)?;
    // ARM 0xacaf4: mul r0, r2, r7
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::new(7)?)?)?;
    // ARM 0xacaf8: str r8, [r4, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,220)?)?;
    // ARM 0xacafc: str r9, [r4, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,224)?)?;
    // ARM 0xacb00: add r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,255)?)?;
    // ARM 0xacb04: mul r0, r8, r9
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(8)?,R::new(9)?)?)?;
    // ARM 0xacb08: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xacb0c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // ARM 0xacb10: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,76)?)?;
    // ARM 0xacb14: str r3, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,56)?)?;
    // ARM 0xacb18: str r7, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,52)?)?;
    // ARM 0xacb1c: str r6, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,48)?)?;
    // ARM 0xacb20: str r12, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,44)?)?;
    // ARM 0xacb24: str r9, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R11,-52)?)?;
    // ARM 0xacb28: str r1, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-60)?)?;
    // ARM 0xacb2c: str r5, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,64)?)?;
    // ARM 0xacb30: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // ARM 0xacb34: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM 0xacb38: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,60)?)?;
    // ARM 0xacb3c: bne 0xacb9c <entry+0x44c>
    c.b(C::Ne,0xacb9c,false)?;
    // ARM 0xacb40: ldr r0, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // ARM 0xacb44: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xacb48: beq 0xacbc4 <entry+0x474>
    c.b(C::Eq,0xacbc4,false)?;
    // ARM 0xacb4c: str r10, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R11,-48)?)?;
    // ARM 0xacb50: add r0, r8, #191
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,191)?)?;
    // ARM 0xacb54: ldr r1, [pc, #0xb9c]
    c.literal(R::R1,0xad6f8)?;
    // ARM 0xacb58: umull r2, r3, r0, r1
    c.emit(arm32::multiply_long(C::Al,false,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // ARM 0xacb5c: ldr r1, [pc, #0xb98]
    c.literal(R::R1,0xad6fc)?;
    // ARM 0xacb60: lsr r0, r3, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(3)?,Shift::Lsr,7)?)?;
    // ARM 0xacb64: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xacb68: lsl r0, r0, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,8)?)?;
    // ARM 0xacb6c: smull r2, r3, r0, r1
    c.emit(arm32::multiply_long(C::Al,true,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // ARM 0xacb70: ldr r1, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,120)?)?;
    // ARM 0xacb74: add r7, r3, r3, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(3)?,R::new(3)?,Shift::Lsr,31,false)?)?;
    // ARM 0xacb78: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacb7c: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // ARM 0xacb80: add r5, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacb84: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacb88: beq 0xacce4 <entry+0x594>
    c.b(C::Eq,0xacce4,false)?;
    // ARM 0xacb8c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacb90: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacb94: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacb98: b 0xaccec <entry+0x59c>
    c.b(C::Al,0xaccec,false)?;
    // ARM 0xacb9c: ldr r1, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,120)?)?;
    // ARM 0xacba0: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacba4: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // ARM 0xacba8: add r5, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacbac: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacbb0: beq 0xacbec <entry+0x49c>
    c.b(C::Eq,0xacbec,false)?;
    // ARM 0xacbb4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacbb8: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacbbc: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacbc0: b 0xacbf4 <entry+0x4a4>
    c.b(C::Al,0xacbf4,false)?;
    // ARM 0xacbc4: ldr r1, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,120)?)?;
    // ARM 0xacbc8: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacbcc: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // ARM 0xacbd0: add r5, r0, r8, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(8)?,Shift::Lsl,1,false)?)?;
    // ARM 0xacbd4: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacbd8: beq 0xacdb8 <entry+0x668>
    c.b(C::Eq,0xacdb8,false)?;
    // ARM 0xacbdc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacbe0: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacbe4: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacbe8: b 0xacdc0 <entry+0x670>
    c.b(C::Al,0xacdc0,false)?;
    // ARM 0xacbec: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacbf0: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacbf4: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // ARM 0xacbf8: ldr r1, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,124)?)?;
    // ARM 0xacbfc: add r0, r9, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc00: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // ARM 0xacc04: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacc08: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc0c: beq 0xacc20 <entry+0x4d0>
    c.b(C::Eq,0xacc20,false)?;
    // ARM 0xacc10: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacc14: bl 0x81850 <entry+0xfffffffffffd5100>
    c.b(C::Al,0x81850,true)?;
    // ARM 0xacc18: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc1c: b 0xacc28 <entry+0x4d8>
    c.b(C::Al,0xacc28,false)?;
    // ARM 0xacc20: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacc24: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacc28: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // ARM 0xacc2c: ldr r1, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,132)?)?;
    // ARM 0xacc30: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacc34: add r5, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacc38: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc3c: beq 0xacc50 <entry+0x500>
    c.b(C::Eq,0xacc50,false)?;
    // ARM 0xacc40: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacc44: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacc48: sub r9, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(9)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc4c: b 0xacc58 <entry+0x508>
    c.b(C::Al,0xacc58,false)?;
    // ARM 0xacc50: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacc54: and r9, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacc58: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // ARM 0xacc5c: ldr r1, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,136)?)?;
    // ARM 0xacc60: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xacc64: add r0, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // ARM 0xacc68: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // ARM 0xacc6c: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacc70: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc74: beq 0xacc88 <entry+0x538>
    c.b(C::Eq,0xacc88,false)?;
    // ARM 0xacc78: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacc7c: bl 0x81850 <entry+0xfffffffffffd5100>
    c.b(C::Al,0x81850,true)?;
    // ARM 0xacc80: sub r3, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacc84: b 0xacc90 <entry+0x540>
    c.b(C::Al,0xacc90,false)?;
    // ARM 0xacc88: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacc8c: and r3, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacc90: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // ARM 0xacc94: ldr r1, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // ARM 0xacc98: ldr r12, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-60)?)?;
    // ARM 0xacc9c: ldr r6, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,64)?)?;
    // ARM 0xacca0: mul r7, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::R1,R::R0)?)?;
    // ARM 0xacca4: ldr r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,88)?)?;
    // ARM 0xacca8: str r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // ARM 0xaccac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaccb0: beq 0xaccd8 <entry+0x588>
    c.b(C::Eq,0xaccd8,false)?;
    // ARM 0xaccb4: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // ARM 0xaccb8: orr r1, r1, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3840)?)?;
    // ARM 0xaccbc: mla r2, r3, r9, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::new(3)?,R::new(9)?,R::R1)?)?;
    // ARM 0xaccc0: ldr r3, [pc, #0xa2c]
    c.literal(R::new(3)?,0xad6f4)?;
    // ARM 0xaccc4: add r1, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaccc8: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacccc: and r7, r1, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaccd0: add r5, r2, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaccd4: b 0xace9c <entry+0x74c>
    c.b(C::Al,0xace9c,false)?;
    // ARM 0xaccd8: mla r5, r3, r9, r7
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(3)?,R::new(9)?,R::new(7)?)?)?;
    // ARM 0xaccdc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xacce0: b 0xace9c <entry+0x74c>
    c.b(C::Al,0xace9c,false)?;
    // ARM 0xacce4: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacce8: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaccec: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // ARM 0xaccf0: ldr r1, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,124)?)?;
    // ARM 0xaccf4: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xaccf8: add r5, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaccfc: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd00: beq 0xacd14 <entry+0x5c4>
    c.b(C::Eq,0xacd14,false)?;
    // ARM 0xacd04: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacd08: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacd0c: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd10: b 0xacd1c <entry+0x5cc>
    c.b(C::Al,0xacd1c,false)?;
    // ARM 0xacd14: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacd18: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacd1c: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // ARM 0xacd20: ldr r1, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,132)?)?;
    // ARM 0xacd24: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacd28: add r5, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacd2c: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd30: beq 0xacd44 <entry+0x5f4>
    c.b(C::Eq,0xacd44,false)?;
    // ARM 0xacd34: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacd38: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacd3c: sub r9, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(9)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd40: b 0xacd4c <entry+0x5fc>
    c.b(C::Al,0xacd4c,false)?;
    // ARM 0xacd44: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacd48: and r9, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacd4c: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // ARM 0xacd50: ldr r1, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,136)?)?;
    // ARM 0xacd54: sub r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R1,1)?)?;
    // ARM 0xacd58: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xacd5c: add r5, r2, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R0,Shift::Lsr,1,false)?)?;
    // ARM 0xacd60: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd64: beq 0xacd78 <entry+0x628>
    c.b(C::Eq,0xacd78,false)?;
    // ARM 0xacd68: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacd6c: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacd70: sub r2, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacd74: b 0xacd80 <entry+0x630>
    c.b(C::Al,0xacd80,false)?;
    // ARM 0xacd78: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacd7c: and r2, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacd80: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // ARM 0xacd84: str r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // ARM 0xacd88: ldr r10, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-48)?)?;
    // ARM 0xacd8c: ldr r12, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,-60)?)?;
    // ARM 0xacd90: ldr r6, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,64)?)?;
    // ARM 0xacd94: orr r1, r1, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3840)?)?;
    // ARM 0xacd98: mla r0, r2, r9, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R2,R::new(9)?,R::R1)?)?;
    // ARM 0xacd9c: ldr r2, [pc, #0x950]
    c.literal(R::R2,0xad6f4)?;
    // ARM 0xacda0: ldr r7, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,68)?)?;
    // ARM 0xacda4: ldr r5, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-40)?)?;
    // ARM 0xacda8: mla r3, r5, r7, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::new(5)?,R::new(7)?,R::R1)?)?;
    // ARM 0xacdac: and r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xacdb0: and r7, r3, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xacdb4: b 0xace94 <entry+0x744>
    c.b(C::Al,0xace94,false)?;
    // ARM 0xacdb8: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacdbc: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacdc0: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // ARM 0xacdc4: lsl r7, r8, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(8)?,Shift::Lsl,1)?)?;
    // ARM 0xacdc8: ldr r1, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,124)?)?;
    // ARM 0xacdcc: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // ARM 0xacdd0: add r5, r0, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xacdd4: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacdd8: beq 0xacdec <entry+0x69c>
    c.b(C::Eq,0xacdec,false)?;
    // ARM 0xacddc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xacde0: bl 0x81838 <entry+0xfffffffffffd50e8>
    c.b(C::Al,0x81838,true)?;
    // ARM 0xacde4: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xacde8: b 0xacdf4 <entry+0x6a4>
    c.b(C::Al,0xacdf4,false)?;
    // ARM 0xacdec: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // ARM 0xacdf0: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xacdf4: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // ARM 0xacdf8: ldr r1, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,132)?)?;
    // ARM 0xacdfc: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    c.check(0xace00)
}
