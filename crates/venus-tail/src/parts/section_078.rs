
/// Apply validated codec profile parameters to the controller's mapped
/// register arrays. Each path waits for status acknowledgements and returns
/// through the saved caller frame; six following words are metadata.
fn program_codec_profile_controller_registers(c:&mut Code)->Result<(),String>{
    c.check(0xc18e4)?;
    // VMA 0xc18e4: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc18e8: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc18ec: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // VMA 0xc18f0: ldr r2, [r0, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,256)?)?;
    // VMA 0xc18f4: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // VMA 0xc18f8: str r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
    // VMA 0xc18fc: ldr r3, [r1, #0xa1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,2588)?)?;
    // VMA 0xc1900: ldr r2, [r0, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,432)?)?;
    // VMA 0xc1904: ldr r10, [r0, #0x198]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,408)?)?;
    // VMA 0xc1908: str r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // VMA 0xc190c: bic r5, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::R2,255)?)?;
    // VMA 0xc1910: ldr r7, [r0, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,444)?)?;
    // VMA 0xc1914: ldr r6, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,8)?)?;
    // VMA 0xc1918: ldr r9, [r1, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R1,464)?)?;
    // VMA 0xc191c: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // VMA 0xc1920: ldr r2, [pc, #0x238]        @ 0x2ff0 <entry+0x2ff0>
    c.literal(R::R2,0xc1b60)?;
    // VMA 0xc1924: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc1928: mov r1, #78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,78)?)?;
    // VMA 0xc192c: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xc0184
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc1930: mov r0, #2176
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2176)?)?;
    // VMA 0xc1934: mvn r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(8)?,R::R0,0)?)?;
    // VMA 0xc1938: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // VMA 0xc193c: mov r3, #2912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2912)?)?;
    // VMA 0xc1940: cmp r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,1)?)?;
    // VMA 0xc1944: orr r1, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc1948: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xc194c: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // VMA 0xc1950: str r8, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(8)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1954: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // VMA 0xc1958: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xc195c: orr r2, r1, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc1960: orr r1, r3, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc1964: mov r3, #3040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3040)?)?;
    // VMA 0xc1968: str r12, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R12,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc196c: mov r2, #868
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,868)?)?;
    // VMA 0xc1970: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // VMA 0xc1974: str r9, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,8)?)?;
    // VMA 0xc1978: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // VMA 0xc197c: orr r4, r2, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R2,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc1980: mov r2, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,48)?)?;
    // VMA 0xc1984: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // VMA 0xc1988: moveq r2, #4224
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,4224)?)?;
    // VMA 0xc198c: cmp r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // VMA 0xc1990: str r2, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1994: str r5, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(5)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1998: orr r5, r3, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(3)?,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc199c: add r3, r10, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(10)?,255)?)?;
    // VMA 0xc19a0: mov r10, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,96)?)?;
    // VMA 0xc19a4: str r2, [r5, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc19a8: mov r2, #988
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,988)?)?;
    // VMA 0xc19ac: bic r3, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,255)?)?;
    // VMA 0xc19b0: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // VMA 0xc19b4: orr r2, r2, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc19b8: str r3, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc19bc: mov r3, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,28)?)?;
    // VMA 0xc19c0: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xc19c4: orr lr, r3, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::new(3)?,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc19c8: mov r3, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,24)?)?;
    // VMA 0xc19cc: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xc19d0: str r7, [lr, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::LR,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc19d4: orr r3, r3, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc19d8: str r12, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R12,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc19dc: str r10, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,0)?)?;
    // VMA 0xc19e0: ldr r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,16)?)?;
    // VMA 0xc19e4: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // VMA 0xc19e8: str r10, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,0)?)?;
    // VMA 0xc19ec: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // VMA 0xc19f0: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xc19f4: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // VMA 0xc19f8: blo 0xc1b54 <entry+0x2fe4>  @ imm = #0x154
    c.b(C::Cc,0xc1b54,false)?;
    // VMA 0xc19fc: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // VMA 0xc1a00: lsl r5, r6, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(6)?,Shift::Lsl,15)?)?;
    // VMA 0xc1a04: cmp r7, #33
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,33)?)?;
    // VMA 0xc1a08: orr r2, r5, #104
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(5)?,104)?)?;
    // VMA 0xc1a0c: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xc1a10: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // VMA 0xc1a14: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1a18: subhi r1, r7, #34
    c.emit(arm32::data_immediate(C::Hi,Op::Sub,R::R1,R::new(7)?,34)?)?;
    // VMA 0xc1a1c: movhi r7, #34
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(7)?,R::R0,34)?)?;
    // VMA 0xc1a20: orr r2, r5, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(5)?,2144)?)?;
    // VMA 0xc1a24: str r1, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1a28: orr r1, r5, #108
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,108)?)?;
    // VMA 0xc1a2c: str r7, [lr, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::LR,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1a30: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xc1a34: str r7, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1a38: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc1a3c: ldr r3, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,8)?)?;
    // VMA 0xc1a40: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // VMA 0xc1a44: moveq r1, #8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xc1a48: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // VMA 0xc1a4c: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1a50: orr r1, r5, #36
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,36)?)?;
    // VMA 0xc1a54: moveq r12, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R12,R::R0,2)?)?;
    // VMA 0xc1a58: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xc1a5c: orr r2, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R12,1)?)?;
    // VMA 0xc1a60: str r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1a64: mov r2, #520093696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,520093696)?)?;
    // VMA 0xc1a68: str r12, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,0)?)?;
    // VMA 0xc1a6c: orr r1, r5, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,124)?)?;
    // VMA 0xc1a70: orr r4, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::R1,2048)?)?;
    // VMA 0xc1a74: mvn r1, #-1946157056
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,2348810240)?)?;
    // VMA 0xc1a78: str r1, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1a7c: orr r1, r5, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,2048)?)?;
    // VMA 0xc1a80: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1a84: orr r1, r5, #120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,120)?)?;
    // VMA 0xc1a88: orr r7, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R1,2048)?)?;
    // VMA 0xc1a8c: mov r1, #33
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33)?)?;
    // VMA 0xc1a90: orr r1, r1, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,268435456)?)?;
    // VMA 0xc1a94: ldr r2, [r7, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1a98: tst r2, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc1a9c: beq 0xc1a94 <entry+0x2f24>  @ imm = #-0x10
    c.b(C::Eq,0xc1a94,false)?;
    // VMA 0xc1aa0: orr r0, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,4)?)?;
    // VMA 0xc1aa4: mov r9, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,3758096384)?)?;
    // VMA 0xc1aa8: ldr r1, [pc, #0xb4]         @ 0x2ff4 <entry+0x2ff4>
    c.literal(R::R1,0xc1b64)?;
    // VMA 0xc1aac: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xc1ab0: ldr r8, [r0, r9]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(8)?,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc1ab4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1ab8: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xc1abc: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92e30
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1ac0: ldr r1, [pc, #0xa0]         @ 0x2ff8 <entry+0x2ff8>
    c.literal(R::R1,0xc1b68)?;
    // VMA 0xc1ac4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1ac8: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92e3c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1acc: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xc1ad0: bge 0xc1b00 <entry+0x2f90>  @ imm = #0x28
    c.b(C::Ge,0xc1b00,false)?;
    // VMA 0xc1ad4: ldr r0, [r7, r9]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc1ad8: tst r0, #268435457
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,268435457)?)?;
    // VMA 0xc1adc: beq 0xc1b00 <entry+0x2f90>  @ imm = #0x1c
    c.b(C::Eq,0xc1b00,false)?;
    // VMA 0xc1ae0: ldr r1, [pc, #0x84]         @ 0x2ffc <entry+0x2ffc>
    c.literal(R::R1,0xc1b6c)?;
    // VMA 0xc1ae4: ldr r2, [pc, #0x84]         @ 0x3000 <entry+0x3000>
    c.literal(R::R2,0xc1b70)?;
    // VMA 0xc1ae8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1aec: mov r3, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,138)?)?;
    // VMA 0xc1af0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92e64
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1af4: ldr r1, [pc, #0x6c]         @ 0x2ff8 <entry+0x2ff8>
    c.literal(R::R1,0xc1b68)?;
    // VMA 0xc1af8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1afc: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92e70
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1b00: ldr r0, [pc, #0x6c]         @ 0x3004 <entry+0x3004>
    c.literal(R::R0,0xc1b74)?;
    // VMA 0xc1b04: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2)?)?;
    // VMA 0xc1b08: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1b0c: beq 0xc1b24 <entry+0x2fb4>  @ imm = #0x10
    c.b(C::Eq,0xc1b24,false)?;
    // VMA 0xc1b10: orr r0, r5, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,1040)?)?;
    // VMA 0xc1b14: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc1b18: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc1b1c: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc1b20: b 0xc1b54 <entry+0x2fe4>  @ imm = #0x2c
    c.b(C::Al,0xc1b54,false)?;
    // VMA 0xc1b24: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xc1b28: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xc1b2c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xc1b30: str r1, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1b34: orr r1, r5, #44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,44)?)?;
    // VMA 0xc1b38: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // VMA 0xc1b3c: str r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1b40: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc1b44: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // VMA 0xc1b48: ldr r1, [r7, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc1b4c: tst r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,2)?)?;
    // VMA 0xc1b50: beq 0xc1b48 <entry+0x2fd8>  @ imm = #-0x10
    c.b(C::Eq,0xc1b48,false)?;
    // VMA 0xc1b54: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc1b58: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc1b5c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc1b60)?;
    const CODEC_REGISTER_CONTROL_POINTERS:[u32;6]=[
       0x000faf54,0x000faf8c,0x000e2788,0x000fafbc,0x000fb001,0x0010408c,
    ];
    for pointer in CODEC_REGISTER_CONTROL_POINTERS {c.emit(pointer)?;}
    c.check(0xc1b78)
}

/// Drop resources from the selected codec output descriptor, release the
/// reference-counted controller slot, and restore its original caller frame.
fn finalize_codec_output_descriptor_resources(c:&mut Code)->Result<(),String>{
    c.check(0xc1b78)?;
    // VMA 0xc1b78: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc1b7c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc1b80: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // VMA 0xc1b84: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xc1b88: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xc1b8c: ldr r2, [pc, #0x170]        @ 0x3194 <entry+0x3194>
    c.literal(R::R2,0xc1d04)?;
    // VMA 0xc1b90: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc1b94: ldr r1, [r5, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,432)?)?;
    // VMA 0xc1b98: ldr r7, [r5, #0x198]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,408)?)?;
    // VMA 0xc1b9c: ldr r8, [r5, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(5)?,444)?)?;
    // VMA 0xc1ba0: ldr r4, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,8)?)?;
    // VMA 0xc1ba4: ldr r9, [r0, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R0,464)?)?;
    // VMA 0xc1ba8: bic r10, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(10)?,R::R1,255)?)?;
    // VMA 0xc1bac: mov r1, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,156)?)?;
    // VMA 0xc1bb0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xc1bb4: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xc040c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc1bb8: ldrb r1, [r5, #0x1b0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,432)?)?;
    // VMA 0xc1bbc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc1bc0: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xc1bc4: mvn r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,2)?)?;
    // VMA 0xc1bc8: orr r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // VMA 0xc1bcc: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc1bd0: str r3, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1bd4: lsl r0, r4, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(4)?,Shift::Lsl,15)?)?;
    // VMA 0xc1bd8: add r3, r1, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R1,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc1bdc: add r1, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,255)?)?;
    // VMA 0xc1be0: bic r6, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::R1,255)?)?;
    // VMA 0xc1be4: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // VMA 0xc1be8: ldr r7, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,16)?)?;
    // VMA 0xc1bec: cmp r7, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,16384)?)?;
    // VMA 0xc1bf0: bne 0xc1c34 <entry+0x30c4>  @ imm = #0x3c
    c.b(C::Ne,0xc1c34,false)?;
    // VMA 0xc1bf4: orr r7, r0, #868
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,868)?)?;
    // VMA 0xc1bf8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xc1bfc: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // VMA 0xc1c00: str r4, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1c04: orr r7, r0, #2912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,2912)?)?;
    // VMA 0xc1c08: ldr r5, [r5, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,260)?)?;
    // VMA 0xc1c0c: orr r7, r7, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,12288)?)?;
    // VMA 0xc1c10: str r5, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1c14: orr r7, r0, #3040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,3040)?)?;
    // VMA 0xc1c18: orr r7, r7, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,12288)?)?;
    // VMA 0xc1c1c: str r4, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1c20: orr r7, r0, #988
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,988)?)?;
    // VMA 0xc1c24: ldr r5, [r1, #0xa20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,2592)?)?;
    // VMA 0xc1c28: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // VMA 0xc1c2c: bic r5, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,255)?)?;
    // VMA 0xc1c30: str r5, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1c34: orr r7, r0, #2176
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,2176)?)?;
    // VMA 0xc1c38: mvn r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc1c3c: mov r4, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,48)?)?;
    // VMA 0xc1c40: cmp r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,1)?)?;
    // VMA 0xc1c44: str r5, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1c48: orr r7, r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,20)?)?;
    // VMA 0xc1c4c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc1c50: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // VMA 0xc1c54: orr r7, r7, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2048)?)?;
    // VMA 0xc1c58: moveq r4, #4224
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,4224)?)?;
    // VMA 0xc1c5c: cmp r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,1)?)?;
    // VMA 0xc1c60: str r5, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1c64: orr r7, r0, #868
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,868)?)?;
    // VMA 0xc1c68: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // VMA 0xc1c6c: str r4, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1c70: orr r7, r0, #2912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,2912)?)?;
    // VMA 0xc1c74: orr r7, r7, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,12288)?)?;
    // VMA 0xc1c78: str r10, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(10)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1c7c: orr r7, r0, #3040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,3040)?)?;
    // VMA 0xc1c80: orr r7, r7, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,12288)?)?;
    // VMA 0xc1c84: str r4, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1c88: orr r7, r0, #988
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,988)?)?;
    // VMA 0xc1c8c: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // VMA 0xc1c90: str r6, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1c94: orr r7, r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,28)?)?;
    // VMA 0xc1c98: orr r7, r7, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2048)?)?;
    // VMA 0xc1c9c: str r8, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(8)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1ca0: orr r7, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R0,24)?)?;
    // VMA 0xc1ca4: orr r7, r7, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2048)?)?;
    // VMA 0xc1ca8: str r3, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1cac: orr r3, r0, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R0,124)?)?;
    // VMA 0xc1cb0: mvn r7, #-1946157056
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(7)?,R::R0,2348810240)?)?;
    // VMA 0xc1cb4: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xc1cb8: str r7, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc1cbc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc1cc0: moveq r2, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xc1cc4: bne 0xc1cd8 <entry+0x3168>  @ imm = #0xc
    c.b(C::Ne,0xc1cd8,false)?;
    // VMA 0xc1cc8: ldr r1, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,16)?)?;
    // VMA 0xc1ccc: mov r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,8)?)?;
    // VMA 0xc1cd0: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // VMA 0xc1cd4: moveq r5, #7
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,7)?)?;
    // VMA 0xc1cd8: orr r1, r0, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,2144)?)?;
    // VMA 0xc1cdc: orr r0, r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,36)?)?;
    // VMA 0xc1ce0: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xc1ce4: str r5, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc1ce8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xc1cec: orr r1, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,1)?)?;
    // VMA 0xc1cf0: str r1, [r0, r3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc1cf4: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // VMA 0xc1cf8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc1cfc: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc1d00: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc1d04)?;
    const CODEC_DESCRIPTOR_RELEASE_ADDRESS:u32=0x000faf54;
    c.emit(CODEC_DESCRIPTOR_RELEASE_ADDRESS)?;
    c.check(0xc1d08)
}

/// Reset a short MMIO control handle and invoke the existing hardware
/// notify callback. Two trailing constants remain explicitly non-executable.
fn pulse_codec_controller_status(c:&mut Code)->Result<(),String>{
    c.check(0xc1d08)?;
    // VMA 0xc1d08: push {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc1d0c: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xc1d10: ldr r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,4)?)?;
    // VMA 0xc1d14: mov r2, #3120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3120)?)?;
    // VMA 0xc1d18: ldr r12, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,32)?)?;
    // VMA 0xc1d1c: ldr r3, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,36)?)?;
    // VMA 0xc1d20: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xc1d24: ldr r2, [r0, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2064)?)?;
    // VMA 0xc1d28: add r3, r12, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R12,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc1d2c: add r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc1d30: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xc1d34: str r2, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc1d38: ldr r1, [pc, #0x20]         @ 0x31f0 <entry+0x31f0>
    c.literal(R::R1,0xc1d60)?;
    // VMA 0xc1d3c: ldr r3, [r0, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,2064)?)?;
    // VMA 0xc1d40: ldr r2, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,32)?)?;
    // VMA 0xc1d44: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1d48: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x930bc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1d4c: ldr r1, [pc, #0x10]         @ 0x31f4 <entry+0x31f4>
    c.literal(R::R1,0xc1d64)?;
    // VMA 0xc1d50: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1d54: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x930c8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1d58: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1d5c: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc1d60)?;
    const CODEC_MMIO_SIGNAL_POINTERS:[u32;2]=[
        0x000fb01c,0x000e2788,
    ];
    for addr in CODEC_MMIO_SIGNAL_POINTERS {c.emit(addr)?;}
    c.check(0xc1d68)
}
