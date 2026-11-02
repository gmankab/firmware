
/// Initialize the codec plane allocator and its hardware profile table,
/// including conditional dispatch of short and long output rows. The
/// original diagnostic and format offsets are structured non-opcode data.
fn initialize_codec_profile_allocator(c:&mut Code)->Result<(),String>{
    c.check(0xbfe54)?;
    // VMA 0xbfe54: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xbfe58: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xbfe5c: sub sp, sp, #60
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,60)?)?;
    // VMA 0xbfe60: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xbfe64: ldr r3, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,12)?)?;
    // VMA 0xbfe68: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // VMA 0xbfe6c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xbfe70: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // VMA 0xbfe74: ldr r8, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,0)?)?;
    // VMA 0xbfe78: ldr r7, [pc, #0x5f0]        @ 0x1900 <entry+0x1900>
    c.literal(R::new(7)?,0xc0470)?;
    // VMA 0xbfe7c: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbfe80: beq 0xc02a0 <entry+0x1730>  @ imm = #0x418
    c.b(C::Eq,0xc02a0,false)?;
    // VMA 0xbfe84: add r9, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(6)?,32)?)?;
    // VMA 0xbfe88: add r10, r8, #464
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(8)?,464)?)?;
    // VMA 0xbfe8c: mvn r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(5)?,R::R0,0)?)?;
    // VMA 0xbfe90: mov r4, r9
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(9)?))?;
    // VMA 0xbfe94: ldr r0, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,28)?)?;
    // VMA 0xbfe98: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xbfe9c: cmp r5, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbfea0: bhs 0xbfedc <entry+0x136c>  @ imm = #0x34
    c.b(C::Cs,0xbfedc,false)?;
    // VMA 0xbfea4: ldr r2, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,16)?)?;
    // VMA 0xbfea8: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // VMA 0xbfeac: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbfeb0: bl 0xdc6e4 <entry+0x1db74> @ imm = #0x1c82c
    c.b(C::Al,0xdc6e4,true)?;
    // VMA 0xbfeb4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbfeb8: beq 0xbff28 <entry+0x13b8>  @ imm = #0x68
    c.b(C::Eq,0xbff28,false)?;
    // VMA 0xbfebc: ldr r3, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,12)?)?;
    // VMA 0xbfec0: add r0, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    // VMA 0xbfec4: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2)?)?;
    // VMA 0xbfec8: addeq r0, r4, #8
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::new(4)?,8)?)?;
    // VMA 0xbfecc: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbfed0: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xbfed4: bne 0xbfe94 <entry+0x1324>  @ imm = #-0x48
    c.b(C::Ne,0xbfe94,false)?;
    // VMA 0xbfed8: b 0xc02a0 <entry+0x1730>  @ imm = #0x3c0
    c.b(C::Al,0xc02a0,false)?;
    // VMA 0xbfedc: cmp r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4)?)?;
    // VMA 0xbfee0: beq 0xbff58 <entry+0x13e8>  @ imm = #0x70
    c.b(C::Eq,0xbff58,false)?;
    // VMA 0xbfee4: cmp r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,6)?)?;
    // VMA 0xbfee8: beq 0xc01d0 <entry+0x1660>  @ imm = #0x2e0
    c.b(C::Eq,0xc01d0,false)?;
    // VMA 0xbfeec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbfef0: orr r0, r0, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,33554432)?)?;
    // VMA 0xbfef4: cmp r3, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbfef8: bne 0xc02a0 <entry+0x1730>  @ imm = #0x3a0
    c.b(C::Ne,0xc02a0,false)?;
    // VMA 0xbfefc: ldr r0, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // VMA 0xbff00: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // VMA 0xbff04: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbff08: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // VMA 0xbff0c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xbff10: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xbff14: str r0, [r1, #0xcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,204)?)?;
    // VMA 0xbff18: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // VMA 0xbff1c: ldr r1, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,16)?)?;
    // VMA 0xbff20: str r1, [r0, #0x9bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2492)?)?;
    // VMA 0xbff24: b 0xc02b4 <entry+0x1744>  @ imm = #0x388
    c.b(C::Al,0xc02b4,false)?;
    // VMA 0xbff28: ldr r1, [pc, #0x568]        @ 0x1928 <entry+0x1928>
    c.literal(R::R1,0xc0498)?;
    // VMA 0xbff2c: ldr r2, [pc, #0x55c]        @ 0x1920 <entry+0x1920>
    c.literal(R::R2,0xc0490)?;
    // VMA 0xbff30: mov r3, #182
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,182)?)?;
    // VMA 0xbff34: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbff38: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // VMA 0xbff3c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x912b0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbff40: ldr r1, [pc, #0x534]        @ 0x190c <entry+0x190c>
    c.literal(R::R1,0xc047c)?;
    // VMA 0xbff44: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbff48: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x912bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbff4c: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // VMA 0xbff50: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // VMA 0xbff54: b 0xc02b4 <entry+0x1744>  @ imm = #0x358
    c.b(C::Al,0xc02b4,false)?;
    // VMA 0xbff58: ldr r5, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,32)?)?;
    // VMA 0xbff5c: ldr r2, [pc, #0x510]        @ 0x1904 <entry+0x1904>
    c.literal(R::R2,0xc0474)?;
    // VMA 0xbff60: mov r1, #214
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,214)?)?;
    // VMA 0xbff64: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xbff68: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xbff6c: str r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-32)?)?;
    // VMA 0xbff70: tst r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,255)?)?;
    // VMA 0xbff74: ldr r4, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,16)?)?;
    // VMA 0xbff78: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbff7c: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbe7d4
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xbff80: ldr r1, [pc, #0x4f0]        @ 0x1908 <entry+0x1908>
    c.literal(R::R1,0xc0478)?;
    // VMA 0xbff84: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbff88: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xbff8c: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // VMA 0xbff90: str r4, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-40)?)?;
    // VMA 0xbff94: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91308
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbff98: ldr r1, [pc, #0x4dc]        @ 0x190c <entry+0x190c>
    c.literal(R::R1,0xc047c)?;
    // VMA 0xbff9c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbffa0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91314
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbffa4: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // VMA 0xbffa8: str r5, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,44)?)?;
    // VMA 0xbffac: str r5, [r0, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,196)?)?;
    // VMA 0xbffb0: ldr r0, [r8, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,972)?)?;
    // VMA 0xbffb4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbffb8: beq 0xc0008 <entry+0x1498>  @ imm = #0x48
    c.b(C::Eq,0xc0008,false)?;
    // VMA 0xbffbc: ldr r0, [r8, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,988)?)?;
    // VMA 0xbffc0: ldr r1, [r8, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,468)?)?;
    // VMA 0xbffc4: ldr r2, [r8, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,472)?)?;
    // VMA 0xbffc8: ldr r3, [r8, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,984)?)?;
    // VMA 0xbffcc: add r4, r8, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,1004)?)?;
    // VMA 0xbffd0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbffd4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xbffd8: bl 0xd7a48 <entry+0x18ed8> @ imm = #0x17a68
    c.b(C::Al,0xd7a48,true)?;
    // VMA 0xbffdc: ldr r0, [r8, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,984)?)?;
    // VMA 0xbffe0: ldr r1, [r8, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,988)?)?;
    // VMA 0xbffe4: ldr r2, [r8, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,468)?)?;
    // VMA 0xbffe8: ldr r3, [r8, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,472)?)?;
    // VMA 0xbffec: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xbfff0: add r0, r8, #612
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,612)?)?;
    // VMA 0xbfff4: add r1, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,2048)?)?;
    // VMA 0xbfff8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xbfffc: bl 0xd7bf8 <entry+0x19088> @ imm = #0x17bf4
    c.b(C::Al,0xd7bf8,true)?;
    // VMA 0xc0000: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc0004: bl 0xd2adc <entry+0x13f6c> @ imm = #0x12ad0
    c.b(C::Al,0xd2adc,true)?;
    // VMA 0xc0008: ldr r0, [r8, #0x278]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,632)?)?;
    // VMA 0xc000c: str r0, [r8, #0xa90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,2704)?)?;
    // VMA 0xc0010: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc0014: bl 0xc7aa0 <entry+0x8f30>  @ imm = #0x7a84
    c.b(C::Al,0xc7aa0,true)?;
    // VMA 0xc0018: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // VMA 0xc001c: mov r7, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,63)?)?;
    // VMA 0xc0020: mov r4, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,6)?)?;
    // VMA 0xc0024: ldr r0, [r8, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,16)?)?;
    // VMA 0xc0028: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // VMA 0xc002c: ldr r6, [r8, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(8)?,472)?)?;
    // VMA 0xc0030: ldr r5, [r8, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(8)?,468)?)?;
    // VMA 0xc0034: add r0, r6, #63
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,63)?)?;
    // VMA 0xc0038: add r12, r6, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(6)?,15)?)?;
    // VMA 0xc003c: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // VMA 0xc0040: bic r1, r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R0,63)?)?;
    // VMA 0xc0044: add r0, r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,31)?)?;
    // VMA 0xc0048: lsr r9, r12, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(9)?,R::R12,Shift::Lsr,4)?)?;
    // VMA 0xc004c: lsr r0, r0, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,5)?)?;
    // VMA 0xc0050: lsl r2, r0, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsl,6)?)?;
    // VMA 0xc0054: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xc0058: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc005c: mov r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,63)?)?;
    // VMA 0xc0060: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // VMA 0xc0064: movhi r1, #31
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R1,R::R0,31)?)?;
    // VMA 0xc0068: movhi r2, #5
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R2,R::R0,5)?)?;
    // VMA 0xc006c: add r3, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0070: lsl r1, r9, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(9)?,Shift::Lsl,6)?)?;
    // VMA 0xc0074: lsr r0, r3, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(3)?,Shift::Lsr,R::R2,false)?)?;
    // VMA 0xc0078: cmp r1, r0, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,6,false)?)?;
    // VMA 0xc007c: mov r1, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,254)?)?;
    // VMA 0xc0080: movhi r3, r12
    c.emit(arm32::mov_register(C::Hi,R::new(3)?,R::R12))?;
    // VMA 0xc0084: movhi r2, #4
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xc0088: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xc008c: lsr r0, r3, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(3)?,Shift::Lsr,R::R2,false)?)?;
    // VMA 0xc0090: mov r3, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,63)?)?;
    // VMA 0xc0094: add r0, r1, r0, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,7,false)?)?;
    // VMA 0xc0098: ldr r1, [pc, #0x3e0]        @ 0x1910 <entry+0x1910>
    c.literal(R::R1,0xc0480)?;
    // VMA 0xc009c: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc00a0: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xc00a4: mvn r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,63)?)?;
    // VMA 0xc00a8: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xc00ac: add r0, r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,31)?)?;
    // VMA 0xc00b0: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xc00b4: and r1, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xc00b8: add r0, r5, #63
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,63)?)?;
    // VMA 0xc00bc: bic lr, r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::LR,R::R0,63)?)?;
    // VMA 0xc00c0: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // VMA 0xc00c4: cmp r1, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xc00c8: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // VMA 0xc00cc: movhi r3, #31
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(3)?,R::R0,31)?)?;
    // VMA 0xc00d0: movhi r1, #5
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R1,R::R0,5)?)?;
    // VMA 0xc00d4: add r3, r3, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc00d8: add r5, r5, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,15)?)?;
    // VMA 0xc00dc: lsr r2, r5, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(5)?,Shift::Lsr,4)?)?;
    // VMA 0xc00e0: lsr r0, r3, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(3)?,Shift::Lsr,R::R1,false)?)?;
    // VMA 0xc00e4: lsl r10, r2, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::new(10)?,R::R2,Shift::Lsl,6)?)?;
    // VMA 0xc00e8: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // VMA 0xc00ec: cmp r10, r0, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R0,Shift::Lsl,6,false)?)?;
    // VMA 0xc00f0: mov r10, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,255)?)?;
    // VMA 0xc00f4: movhi r3, r5
    c.emit(arm32::mov_register(C::Hi,R::new(3)?,R::new(5)?))?;
    // VMA 0xc00f8: movhi r1, #4
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xc00fc: lsr r0, r3, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(3)?,Shift::Lsr,R::R1,false)?)?;
    // VMA 0xc0100: ldr r3, [pc, #0x37c]        @ 0x1914 <entry+0x1914>
    c.literal(R::new(3)?,0xc0484)?;
    // VMA 0xc0104: add r1, r10, r5, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(10)?,R::new(5)?,Shift::Lsr,4,false)?)?;
    // VMA 0xc0108: add r0, r10, r0, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::R0,Shift::Lsl,6,false)?)?;
    // VMA 0xc010c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc0110: and r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0114: ldr r3, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,32)?)?;
    // VMA 0xc0118: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc011c: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // VMA 0xc0120: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc0124: mvn r1, #-268435453
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,4026531843)?)?;
    // VMA 0xc0128: and r1, r1, r3, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsr,4,false)?)?;
    // VMA 0xc012c: ldr r3, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,28)?)?;
    // VMA 0xc0130: lsl r3, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsl,1)?)?;
    // VMA 0xc0134: cmp r3, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc0138: movhi r7, #31
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(7)?,R::R0,31)?)?;
    // VMA 0xc013c: movhi r4, #5
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // VMA 0xc0140: add r1, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0144: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // VMA 0xc0148: lsr r3, r1, r4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(3)?,R::R0,R::R1,Shift::Lsr,R::new(4)?,false)?)?;
    // VMA 0xc014c: movhi r6, #2
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // VMA 0xc0150: mul r5, r3, r6
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::new(3)?,R::new(6)?)?)?;
    // VMA 0xc0154: lsl r3, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(6)?,Shift::Lsl,1)?)?;
    // VMA 0xc0158: cmp r9, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc015c: mov r5, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,254)?)?;
    // VMA 0xc0160: movhi r1, r12
    c.emit(arm32::mov_register(C::Hi,R::R1,R::R12))?;
    // VMA 0xc0164: movhi r4, #4
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // VMA 0xc0168: orr r5, r5, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,256)?)?;
    // VMA 0xc016c: movhi r3, #2
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // VMA 0xc0170: lsr r1, r1, r4
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsr,R::new(4)?,false)?)?;
    // VMA 0xc0174: mla r7, r3, r1, r5
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::new(3)?,R::R1,R::new(5)?)?)?;
    // VMA 0xc0178: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // VMA 0xc017c: and r1, r7, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc0180: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc0184: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // VMA 0xc0188: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // VMA 0xc018c: bne 0xc02c0 <entry+0x1750>  @ imm = #0x12c
    c.b(C::Ne,0xc02c0,false)?;
    // VMA 0xc0190: ldr r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    // VMA 0xc0194: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // VMA 0xc0198: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xc019c: add r1, r1, r2, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,5,false)?)?;
    // VMA 0xc01a0: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // VMA 0xc01a4: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xc01a8: bic lr, r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::LR,R::R2,31)?)?;
    // VMA 0xc01ac: add r3, lr, lr, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::LR,R::LR,Shift::Lsl,2,false)?)?;
    // VMA 0xc01b0: add r3, r10, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(10)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // VMA 0xc01b4: bic r3, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,255)?)?;
    // VMA 0xc01b8: add r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc01bc: mov r3, #176
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,176)?)?;
    // VMA 0xc01c0: mul r7, lr, r3
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::LR,R::new(3)?)?)?;
    // VMA 0xc01c4: orr r3, r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(7)?,256)?)?;
    // VMA 0xc01c8: ldr r7, [pc, #0x2b0]        @ 0x1910 <entry+0x1910>
    c.literal(R::new(7)?,0xc0480)?;
    // VMA 0xc01cc: b 0xc02f0 <entry+0x1780>  @ imm = #0x11c
    c.b(C::Al,0xc02f0,false)?;
    // VMA 0xc01d0: ldr r1, [r8, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,468)?)?;
    // VMA 0xc01d4: ldr r2, [r8, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,472)?)?;
    // VMA 0xc01d8: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // VMA 0xc01dc: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // VMA 0xc01e0: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // VMA 0xc01e4: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // VMA 0xc01e8: add r2, r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,15)?)?;
    // VMA 0xc01ec: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xc01f0: bic r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,15)?)?;
    // VMA 0xc01f4: bic r2, r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,15)?)?;
    // VMA 0xc01f8: mul r3, r2, r1
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R1)?)?;
    // VMA 0xc01fc: mov r2, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1048576)?)?;
    // VMA 0xc0200: add r3, r3, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // VMA 0xc0204: lsr r1, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(3)?,Shift::Lsr,1)?)?;
    // VMA 0xc0208: cmp r1, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1048576)?)?;
    // VMA 0xc020c: add r1, r8, #2640
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,2640)?)?;
    // VMA 0xc0210: lsrhi r2, r3, #1
    c.emit(arm32::mov_shifted_register(C::Hi,R::R2,R::new(3)?,Shift::Lsr,1)?)?;
    // VMA 0xc0214: cmp r3, #2097152
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,2097152)?)?;
    // VMA 0xc0218: str r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // VMA 0xc021c: movls r3, #2097152
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(3)?,R::R0,2097152)?)?;
    // VMA 0xc0220: str r2, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,44)?)?;
    // VMA 0xc0224: str r3, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,40)?)?;
    // VMA 0xc0228: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // VMA 0xc022c: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // VMA 0xc0230: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc0234: add r8, r1, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0238: str r0, [r8, #0x108]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,264)?)?;
    // VMA 0xc023c: ldr r5, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,44)?)?;
    // VMA 0xc0240: ldr r4, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,36)?)?;
    // VMA 0xc0244: str r5, [r4, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(5)?,R::new(4)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc0248: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc024c: ldr r7, [r8, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(8)?,264)?)?;
    // VMA 0xc0250: ldr r0, [r8, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,172)?)?;
    // VMA 0xc0254: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xc0258: bl 0xd9da8 <entry+0x1b238> @ imm = #0x19b48
    c.b(C::Al,0xd9da8,true)?;
    // VMA 0xc025c: add r0, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0260: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // VMA 0xc0264: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc0268: str r0, [r8, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,272)?)?;
    // VMA 0xc026c: ldr r5, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,40)?)?;
    // VMA 0xc0270: str r5, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,8)?)?;
    // VMA 0xc0274: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0278: ldr r7, [r8, #0x110]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(8)?,272)?)?;
    // VMA 0xc027c: ldr r0, [r8, #0xb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,180)?)?;
    // VMA 0xc0280: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xc0284: bl 0xd9da8 <entry+0x1b238> @ imm = #0x19b1c
    c.b(C::Al,0xd9da8,true)?;
    // VMA 0xc0288: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    // VMA 0xc028c: add r0, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0290: add r9, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,4)?)?;
    // VMA 0xc0294: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // VMA 0xc0298: bne 0xc0228 <entry+0x16b8>  @ imm = #-0x78
    c.b(C::Ne,0xc0228,false)?;
    // VMA 0xc029c: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xc02a0: ldr r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-40)?)?;
    // VMA 0xc02a4: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc02a8: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xc02ac: bl 0xd0b98 <entry+0x12028> @ imm = #0x108e4
    c.b(C::Al,0xd0b98,true)?;
    // VMA 0xc02b0: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xc02b4: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xc02b8: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc02bc: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc02c0: ldr r1, [pc, #0x1b8]        @ 0x1910 <entry+0x1910>
    c.literal(R::R1,0xc0480)?;
    // VMA 0xc02c4: ldr r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    // VMA 0xc02c8: and r1, r1, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,3,false)?)?;
    // VMA 0xc02cc: add r3, lr, lr, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::LR,R::LR,Shift::Lsl,2,false)?)?;
    // VMA 0xc02d0: add r3, r10, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(10)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // VMA 0xc02d4: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xc02d8: bic r3, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,255)?)?;
    // VMA 0xc02dc: add r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc02e0: mov r3, #176
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,176)?)?;
    // VMA 0xc02e4: mul r7, lr, r3
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::LR,R::new(3)?)?)?;
    // VMA 0xc02e8: orr r3, r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(7)?,256)?)?;
    // VMA 0xc02ec: ldr r7, [pc, #0x194]        @ 0x1918 <entry+0x1918>
    c.literal(R::new(7)?,0xc0488)?;
    // VMA 0xc02f0: sub r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xc02f4: and r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc02f8: ldr r7, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,40)?)?;
    // VMA 0xc02fc: ldr r9, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xc0300: ldr r6, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,-40)?)?;
    // VMA 0xc0304: add r2, lr, lr, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::LR,R::LR,Shift::Lsl,2,false)?)?;
    // VMA 0xc0308: add r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc030c: add r2, r5, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(5)?,R::R2,Shift::Lsl,1,false)?)?;
    // VMA 0xc0310: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // VMA 0xc0314: ldr r3, [pc, #0x164]        @ 0x1910 <entry+0x1910>
    c.literal(R::new(3)?,0xc0480)?;
    // VMA 0xc0318: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc031c: mov r3, #251
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,251)?)?;
    // VMA 0xc0320: add r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc0324: ldr r2, [pc, #0x164]        @ 0x1920 <entry+0x1920>
    c.literal(R::R2,0xc0490)?;
    // VMA 0xc0328: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // VMA 0xc032c: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc0330: ldr r1, [pc, #0x154]        @ 0x191c <entry+0x191c>
    c.literal(R::R1,0xc048c)?;
    // VMA 0xc0334: add r0, r0, #46080
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,46080)?)?;
    // VMA 0xc0338: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xc033c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc0340: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x916b4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0344: ldr r1, [pc, #0x130]        @ 0x190c <entry+0x190c>
    c.literal(R::R1,0xc047c)?;
    // VMA 0xc0348: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc034c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x916c0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc0350: ldr r5, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,44)?)?;
    // VMA 0xc0354: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xc0358: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xc035c: mov r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,256)?)?;
    // VMA 0xc0360: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xc0364: bl 0xc9278 <entry+0xa708>  @ imm = #0x8f0c
    c.b(C::Al,0xc9278,true)?;
    // VMA 0xc0368: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xc036c: add r0, r5, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc0370: mov r5, r9
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // VMA 0xc0374: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xc0378: ldr r0, [r5, #0x170]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,368)?)?;
    // VMA 0xc037c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc0380: bne 0xc03a8 <entry+0x1838>  @ imm = #0x20
    c.b(C::Ne,0xc03a8,false)?;
    // VMA 0xc0384: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // VMA 0xc0388: mov r1, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3840)?)?;
    // VMA 0xc038c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xc0390: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc0394: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xc0398: bl 0x7e544 <entry+0xfffffffffffbf9d4> @ imm = #-0x41e5c
    c.b(C::Al,0x7e544,true)?;
    // VMA 0xc039c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc03a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc03a4: beq 0xc0460 <entry+0x18f0>  @ imm = #0xb4
    c.b(C::Eq,0xc0460,false)?;
    // VMA 0xc03a8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xc03ac: sub r3, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R11,32)?)?;
    // VMA 0xc03b0: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // VMA 0xc03b4: mov r2, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3840)?)?;
    // VMA 0xc03b8: sub r4, r6, r4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc03bc: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // VMA 0xc03c0: bl 0x7f600 <entry+0xfffffffffffc0a90> @ imm = #-0x40dc8
    c.b(C::Al,0x7f600,true)?;
    // VMA 0xc03c4: ldr r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xc03c8: ldr r1, [pc, #0xc4]         @ 0x1924 <entry+0x1924>
    c.literal(R::R1,0xc0494)?;
    // VMA 0xc03cc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc03d0: mov r3, #46080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,46080)?)?;
    // VMA 0xc03d4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91748
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc03d8: ldr r1, [pc, #0x9c]         @ 0x190c <entry+0x190c>
    c.literal(R::R1,0xc047c)?;
    // VMA 0xc03dc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc03e0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x91754
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc03e4: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xc03e8: add r1, r0, #46080
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,46080)?)?;
    // VMA 0xc03ec: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // VMA 0xc03f0: ldr r1, [r8, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,16)?)?;
    // VMA 0xc03f4: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // VMA 0xc03f8: bne 0xc0440 <entry+0x18d0>  @ imm = #0x40
    c.b(C::Ne,0xc0440,false)?;
    // VMA 0xc03fc: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // VMA 0xc0400: mov r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,512)?)?;
    // VMA 0xc0404: add r0, r0, #46080
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,46080)?)?;
    // VMA 0xc0408: str r1, [r8, #0xa20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,2592)?)?;
    // VMA 0xc040c: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // VMA 0xc0410: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc0414: str r0, [r9, #0x104]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,260)?)?;
    // VMA 0xc0418: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // VMA 0xc041c: str r1, [r8, #0xa1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,2588)?)?;
    // VMA 0xc0420: add r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,512)?)?;
    // VMA 0xc0424: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc0428: str r0, [r9, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,256)?)?;
    // VMA 0xc042c: ldr r0, [r9, #0xcc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,204)?)?;
    // VMA 0xc0430: add r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // VMA 0xc0434: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xc0438: sub r0, r4, #46848
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(4)?,46848)?)?;
    // VMA 0xc043c: b 0xc0444 <entry+0x18d4>  @ imm = #0x0
    c.b(C::Al,0xc0444,false)?;
    // VMA 0xc0440: sub r0, r4, #46080
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(4)?,46080)?)?;
    // VMA 0xc0444: ldr r2, [pc, #0x28]         @ 0x1904 <entry+0x1904>
    c.literal(R::R2,0xc0474)?;
    // VMA 0xc0448: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc044c: eor r0, r1, r0, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::R1,R::R0,Shift::Lsr,31,false)?)?;
    // VMA 0xc0450: mov r1, #58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,58)?)?;
    // VMA 0xc0454: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // VMA 0xc0458: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbecb0
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc045c: b 0xc02b4 <entry+0x1744>  @ imm = #-0x1b0
    c.b(C::Al,0xc02b4,false)?;
    // VMA 0xc0460: ldr r2, [pc, #0xc]          @ 0x1904 <entry+0x1904>
    c.literal(R::R2,0xc0474)?;
    // VMA 0xc0464: mov r1, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    // VMA 0xc0468: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc046c: b 0xc0454 <entry+0x18e4>  @ imm = #-0x20
    c.b(C::Al,0xc0454,false)?;
    c.check(0xc0470)?;
    const CODEC_PROFILE_GLOBALS_AND_FORMATS:[u32;11]=[
        0x02000102,0x000fa71c,0x000fa815,0x000e2788,
        0xfffffe00,0x1fffff00,0xfffffc00,0x000fa838,
        0x000fa800,0x000fa876,0x000f558e,
    ];
    for word in CODEC_PROFILE_GLOBALS_AND_FORMATS {c.emit(word)?;}
    c.check(0xc049c)
}
