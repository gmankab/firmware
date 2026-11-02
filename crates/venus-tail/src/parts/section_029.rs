
/// Complete codec frame-command state machine, update plane sizes, and publish the control register status packets
/// Runtime slice 0xb2000..0xb2a80, with 15 classified trailing data words.
fn finish_frame_command_queue_and_output_registers(c:&mut Code)->Result<(),String>{
    c.check(0xb2000)?;
    // ARM 0xb2000: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // ARM 0xb2004: mov r8, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,22)?)?;
    // ARM 0xb2008: str r1, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb200c: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb2010: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xb2014: ldr r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb2018: strb r2, [r5, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,110)?)?;
    // ARM 0xb201c: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb2020: and r1, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R2,255)?)?;
    // ARM 0xb2024: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // ARM 0xb2028: strb r0, [r5, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,111)?)?;
    // ARM 0xb202c: bhi 0xb2078 <entry+0x5928>
    c.b(C::Hi,0xb2078,false)?;
    // ARM 0xb2030: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // ARM 0xb2034: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // ARM 0xb2038: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM 0xb203c: bhi 0xb1c9c <entry+0x554c>
    c.b(C::Hi,0xb1c9c,false)?;
    // ARM 0xb2040: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb2044: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // ARM 0xb2048: str r1, [r9, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb204c: ldr r1, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2050: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // ARM 0xb2054: strb r1, [r5, #0x73]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,115)?)?;
    // ARM 0xb2058: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // ARM 0xb205c: sub r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,4)?)?;
    // ARM 0xb2060: cmp r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,13)?)?;
    // ARM 0xb2064: blo 0xb2080 <entry+0x5930>
    c.b(C::Cc,0xb2080,false)?;
    // ARM 0xb2068: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb206c: mov r8, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,25)?)?;
    // ARM 0xb2070: strb r0, [r5, #0x73]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,115)?)?;
    // ARM 0xb2074: b 0xb1c9c <entry+0x554c>
    c.b(C::Al,0xb1c9c,false)?;
    // ARM 0xb2078: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // ARM 0xb207c: b 0xb1c9c <entry+0x554c>
    c.b(C::Al,0xb1c9c,false)?;
    // ARM 0xb2080: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb2084: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xb2088: str r1, [r9, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb208c: ldr r0, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2090: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // ARM 0xb2094: str r2, [r5, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,160)?)?;
    // ARM 0xb2098: strb r2, [r5, #0x83]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,131)?)?;
    // ARM 0xb209c: strb r2, [r5, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,123)?)?;
    // ARM 0xb20a0: ldrb r0, [r5, #0x5e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,94)?)?;
    // ARM 0xb20a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb20a8: beq 0xb2160 <entry+0x5a10>
    c.b(C::Eq,0xb2160,false)?;
    // ARM 0xb20ac: mov r8, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,16777216)?)?;
    // ARM 0xb20b0: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xb20b4: mov r12, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // ARM 0xb20b8: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM 0xb20bc: add r3, r5, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb20c0: str r8, [r1, r12]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(8)?,R::R1,R::R12,Shift::Lsl,0)?)?;
    // ARM 0xb20c4: str r8, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,0)?)?;
    // ARM 0xb20c8: str r8, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,0)?)?;
    // ARM 0xb20cc: ldr r1, [r0, r12]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::R12,Shift::Lsl,0)?)?;
    // ARM 0xb20d0: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // ARM 0xb20d4: strb r1, [r3, #0x74]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,116)?)?;
    // ARM 0xb20d8: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // ARM 0xb20dc: strb r1, [r3, #0x7c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(3)?,124)?)?;
    // ARM 0xb20e0: add r1, r5, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(5)?,R::R2,Shift::Lsl,2,false)?)?;
    // ARM 0xb20e4: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // ARM 0xb20e8: str r0, [r1, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,132)?)?;
    // ARM 0xb20ec: ldrb r12, [r3, #0x74]
    c.emit(arm32::transfer_byte(C::Al,true,R::R12,R::new(3)?,116)?)?;
    // ARM 0xb20f0: sub r0, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R12,1)?)?;
    // ARM 0xb20f4: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // ARM 0xb20f8: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // ARM 0xb20fc: movhs r12, #16
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xb2100: strbhs r12, [r3, #0x74]
    c.emit(arm32::transfer_byte(C::Cs,false,R::R12,R::new(3)?,116)?)?;
    // ARM 0xb2104: ldrb lr, [r3, #0x7c]
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::new(3)?,124)?)?;
    // ARM 0xb2108: cmp lr, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::R12,Shift::Lsl,0,false)?)?;
    // ARM 0xb210c: strbhi r12, [r3, #0x7c]
    c.emit(arm32::transfer_byte(C::Hi,false,R::R12,R::new(3)?,124)?)?;
    // ARM 0xb2110: movhi lr, r12
    c.emit(arm32::mov_register(C::Hi,R::LR,R::R12))?;
    // ARM 0xb2114: ldrb r0, [r5, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,123)?)?;
    // ARM 0xb2118: cmp r0, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // ARM 0xb211c: strblo r12, [r5, #0x7b]
    c.emit(arm32::transfer_byte(C::Cc,false,R::R12,R::new(5)?,123)?)?;
    // ARM 0xb2120: ldrblo lr, [r3, #0x7c]
    c.emit(arm32::transfer_byte(C::Cc,true,R::LR,R::new(3)?,124)?)?;
    // ARM 0xb2124: ldrb r0, [r5, #0x83]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,131)?)?;
    // ARM 0xb2128: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // ARM 0xb212c: cmp r0, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xb2130: strblo lr, [r5, #0x83]
    c.emit(arm32::transfer_byte(C::Cc,false,R::LR,R::new(5)?,131)?)?;
    // ARM 0xb2134: ldr r1, [r1, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,132)?)?;
    // ARM 0xb2138: ldr r0, [r5, #0xa0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,160)?)?;
    // ARM 0xb213c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb2140: strlo r1, [r5, #0xa0]
    c.emit(arm32::transfer_word(C::Cc,false,R::R1,R::new(5)?,160)?)?;
    // ARM 0xb2144: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // ARM 0xb2148: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb214c: beq 0xb2160 <entry+0x5a10>
    c.b(C::Eq,0xb2160,false)?;
    // ARM 0xb2150: ldrb r1, [r5, #0x5e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,94)?)?;
    // ARM 0xb2154: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // ARM 0xb2158: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb215c: blt 0xb20b0 <entry+0x5960>
    c.b(C::Lt,0xb20b0,false)?;
    // ARM 0xb2160: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // ARM 0xb2164: mov r0, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16777216)?)?;
    // ARM 0xb2168: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xb216c: str r0, [r1, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb2170: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb2174: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb2178: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb217c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM 0xb2180: ldr r1, [r0, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb2184: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM 0xb2188: add r1, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,3)?)?;
    // ARM 0xb218c: and r12, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R12,R::R1,255)?)?;
    // ARM 0xb2190: strb r1, [r5, #0xa4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,164)?)?;
    // ARM 0xb2194: lsl r2, r2, r12
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R2,Shift::Lsl,R::R12,false)?)?;
    // ARM 0xb2198: strb r2, [r5, #0xa6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,166)?)?;
    // ARM 0xb219c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // ARM 0xb21a0: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb21a4: strb r0, [r5, #0xa5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,165)?)?;
    // ARM 0xb21a8: ldr r1, [r5, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,88)?)?;
    // ARM 0xb21ac: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // ARM 0xb21b0: cmp r1, #77
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,77)?)?;
    // ARM 0xb21b4: bne 0xb21cc <entry+0x5a7c>
    c.b(C::Ne,0xb21cc,false)?;
    // ARM 0xb21b8: and r1, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,255)?)?;
    // ARM 0xb21bc: mov r8, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,127)?)?;
    // ARM 0xb21c0: sub r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,4)?)?;
    // ARM 0xb21c4: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // ARM 0xb21c8: bhi 0xb1c9c <entry+0x554c>
    c.b(C::Hi,0xb1c9c,false)?;
    // ARM 0xb21cc: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // ARM 0xb21d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb21d4: lsl r1, r0, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R0,Shift::Lsl,R::R2,false)?)?;
    // ARM 0xb21d8: strb r1, [r5, #0xa7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,167)?)?;
    // ARM 0xb21dc: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // ARM 0xb21e0: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // ARM 0xb21e4: and r1, r1, r0, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsl,R::R2,false)?)?;
    // ARM 0xb21e8: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // ARM 0xb21ec: sub r8, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R1,1)?)?;
    // ARM 0xb21f0: ldrh r0, [r5, #98]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(5)?,98)?)?;
    // ARM 0xb21f4: str r12, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,16)?)?;
    // ARM 0xb21f8: add r0, r8, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb21fc: blx 0x82be4 <entry+0xfffffffffffd6494>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // ARM 0xb2200: mov r1, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,616)?)?;
    // ARM 0xb2204: strh r0, [r5, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(5)?,R::R1)?)?;
    // ARM 0xb2208: ldrh r0, [r5, #100]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(5)?,100)?)?;
    // ARM 0xb220c: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // ARM 0xb2210: add r0, r8, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2214: blx 0x82be4 <entry+0xfffffffffffd6494>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // ARM 0xb2218: mov r1, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,106)?)?;
    // ARM 0xb221c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // ARM 0xb2220: strh r0, [r5, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(5)?,R::R1)?)?;
    // ARM 0xb2224: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb2228: ldr r0, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb222c: add r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // ARM 0xb2230: strb r0, [r5, #0xa9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,169)?)?;
    // ARM 0xb2234: and r1, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,255)?)?;
    // ARM 0xb2238: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // ARM 0xb223c: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb2240: bhs 0xb232c <entry+0x5bdc>
    c.b(C::Cs,0xb232c,false)?;
    // ARM 0xb2244: mov r12, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // ARM 0xb2248: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // ARM 0xb224c: ldr r2, [r3, r12]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::new(3)?,R::R12,Shift::Lsl,0)?)?;
    // ARM 0xb2250: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2254: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // ARM 0xb2258: strb r0, [r5, #0xaa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,170)?)?;
    // ARM 0xb225c: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // ARM 0xb2260: sub r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb2264: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // ARM 0xb2268: strb r0, [r5, #0xa8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,168)?)?;
    // ARM 0xb226c: str r1, [r2, r12]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R12,Shift::Lsl,0)?)?;
    // ARM 0xb2270: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb2274: ldr r2, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb2278: strb r2, [r5, #0xad]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,173)?)?;
    // ARM 0xb227c: and r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,255)?)?;
    // ARM 0xb2280: ldr r1, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,0)?)?;
    // ARM 0xb2284: cmp r0, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb2288: strb r1, [r5, #0xae]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,174)?)?;
    // ARM 0xb228c: blo 0xb2338 <entry+0x5be8>
    c.b(C::Cc,0xb2338,false)?;
    // ARM 0xb2290: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // ARM 0xb2294: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2298: bhi 0xb2338 <entry+0x5be8>
    c.b(C::Hi,0xb2338,false)?;
    // ARM 0xb229c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb22a0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb22a4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xb22a8: str r1, [r9, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb22ac: strb r2, [r5, #0xb0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,176)?)?;
    // ARM 0xb22b0: ldr r2, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb22b4: tst r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,255)?)?;
    // ARM 0xb22b8: strb r2, [r5, #0xaf]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,175)?)?;
    // ARM 0xb22bc: beq 0xb2344 <entry+0x5bf4>
    c.b(C::Eq,0xb2344,false)?;
    // ARM 0xb22c0: str r1, [r9, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb22c4: ldr r0, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb22c8: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // ARM 0xb22cc: strb r0, [r5, #0xb0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,176)?)?;
    // ARM 0xb22d0: beq 0xb2354 <entry+0x5c04>
    c.b(C::Eq,0xb2354,false)?;
    // ARM 0xb22d4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb22d8: mov r8, r5
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(5)?))?;
    // ARM 0xb22dc: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // ARM 0xb22e0: ldr r0, [r8, #0x27c]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(8)?,636)?)?;
    // ARM 0xb22e4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb22e8: beq 0xb245c <entry+0x5d0c>
    c.b(C::Eq,0xb245c,false)?;
    // ARM 0xb22ec: bl 0x80170 <entry+0xfffffffffffd3a20>
    c.b(C::Al,0x80170,true)?;
    // ARM 0xb22f0: ldr r2, [pc, #0x758]
    c.literal(R::R2,0xb2a50)?;
    // ARM 0xb22f4: mov r1, #43
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,43)?)?;
    // ARM 0xb22f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb22fc: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // ARM 0xb2300: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb2304: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2560)?)?;
    // ARM 0xb2308: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xb230c: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // ARM 0xb2310: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM 0xb2314: blo 0xb2440 <entry+0x5cf0>
    c.b(C::Cc,0xb2440,false)?;
    // ARM 0xb2318: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // ARM 0xb231c: bl 0x7fed0 <entry+0xfffffffffffd3780>
    c.b(C::Al,0x7fed0,true)?;
    // ARM 0xb2320: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb2324: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // ARM 0xb2328: b 0xb245c <entry+0x5d0c>
    c.b(C::Al,0xb245c,false)?;
    // ARM 0xb232c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM 0xb2330: strb r0, [r5, #0xa9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,169)?)?;
    // ARM 0xb2334: b 0xb25e8 <entry+0x5e98>
    c.b(C::Al,0xb25e8,false)?;
    // ARM 0xb2338: strb r0, [r5, #0xae]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,174)?)?;
    // ARM 0xb233c: strb r0, [r5, #0xad]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,173)?)?;
    // ARM 0xb2340: b 0xb25e8 <entry+0x5e98>
    c.b(C::Al,0xb25e8,false)?;
    // ARM 0xb2344: ldr r0, [r5, #0x27c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,636)?)?;
    // ARM 0xb2348: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb234c: bne 0xb2360 <entry+0x5c10>
    c.b(C::Ne,0xb2360,false)?;
    // ARM 0xb2350: b 0xb236c <entry+0x5c1c>
    c.b(C::Al,0xb236c,false)?;
    // ARM 0xb2354: ldr r0, [r5, #0x27c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,636)?)?;
    // ARM 0xb2358: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb235c: beq 0xb2364 <entry+0x5c14>
    c.b(C::Eq,0xb2364,false)?;
    // ARM 0xb2360: bl 0x7fed0 <entry+0xfffffffffffd3780>
    c.b(C::Al,0x7fed0,true)?;
    // ARM 0xb2364: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb2368: str r0, [r5, #0x27c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,636)?)?;
    // ARM 0xb236c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb2370: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb2374: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // ARM 0xb2378: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // ARM 0xb237c: str r1, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb2380: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb2384: ldr r6, [r3, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(6)?,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb2388: strb r6, [r5, #0xb1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(5)?,177)?)?;
    // ARM 0xb238c: ldr r6, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(3)?,0)?)?;
    // ARM 0xb2390: strb r6, [r5, #0xb2]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(5)?,178)?)?;
    // ARM 0xb2394: mov r6, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,8)?)?;
    // ARM 0xb2398: strb r6, [r5, #0x71]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(5)?,113)?)?;
    // ARM 0xb239c: strb r6, [r5, #0x72]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(5)?,114)?)?;
    // ARM 0xb23a0: mov r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,7)?)?;
    // ARM 0xb23a4: strb r6, [r5, #0xab]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(5)?,171)?)?;
    // ARM 0xb23a8: mov r6, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,5)?)?;
    // ARM 0xb23ac: strb r6, [r5, #0xac]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::new(5)?,172)?)?;
    // ARM 0xb23b0: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb23b4: ldr r2, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb23b8: tst r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,255)?)?;
    // ARM 0xb23bc: strb r2, [r5, #0x70]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,112)?)?;
    // ARM 0xb23c0: beq 0xb25c4 <entry+0x5e74>
    c.b(C::Eq,0xb25c4,false)?;
    // ARM 0xb23c4: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // ARM 0xb23c8: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // ARM 0xb23cc: str r2, [r3, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb23d0: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb23d4: mov r2, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16777216)?)?;
    // ARM 0xb23d8: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb23dc: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb23e0: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // ARM 0xb23e4: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xb23e8: ldr r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb23ec: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // ARM 0xb23f0: strb r3, [r5, #0x71]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(5)?,113)?)?;
    // ARM 0xb23f4: and r3, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,255)?)?;
    // ARM 0xb23f8: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // ARM 0xb23fc: add r2, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,1)?)?;
    // ARM 0xb2400: strb r2, [r5, #0x72]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,114)?)?;
    // ARM 0xb2404: ldrb r1, [r5, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,110)?)?;
    // ARM 0xb2408: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // ARM 0xb240c: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb2410: bhs 0xb241c <entry+0x5ccc>
    c.b(C::Cs,0xb241c,false)?;
    // ARM 0xb2414: ldrb r2, [r5, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,111)?)?;
    // ARM 0xb2418: b 0xb2430 <entry+0x5ce0>
    c.b(C::Al,0xb2430,false)?;
    // ARM 0xb241c: and r3, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R2,255)?)?;
    // ARM 0xb2420: ldrb r2, [r5, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,111)?)?;
    // ARM 0xb2424: add r6, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R2,8)?)?;
    // ARM 0xb2428: cmp r3, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb242c: bls 0xb259c <entry+0x5e4c>
    c.b(C::Ls,0xb259c,false)?;
    // ARM 0xb2430: add r0, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,8)?)?;
    // ARM 0xb2434: strb r1, [r5, #0x71]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,113)?)?;
    // ARM 0xb2438: strb r0, [r5, #0x72]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,114)?)?;
    // ARM 0xb243c: b 0xb25e8 <entry+0x5e98>
    c.b(C::Al,0xb25e8,false)?;
    // ARM 0xb2440: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // ARM 0xb2444: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb2448: beq 0xb245c <entry+0x5d0c>
    c.b(C::Eq,0xb245c,false)?;
    // ARM 0xb244c: add r1, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,24)?)?;
    // ARM 0xb2450: bl 0x8019c <entry+0xfffffffffffd3a4c>
    c.b(C::Al,0x8019c,true)?;
    // ARM 0xb2454: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // ARM 0xb2458: b 0xb2478 <entry+0x5d28>
    c.b(C::Al,0xb2478,false)?;
    // ARM 0xb245c: ldr r0, [r4, #0x490]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1168)?)?;
    // ARM 0xb2460: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // ARM 0xb2464: add r2, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,24)?)?;
    // ARM 0xb2468: bl 0x7f974 <entry+0xfffffffffffd3224>
    c.b(C::Al,0x7f974,true)?;
    // ARM 0xb246c: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // ARM 0xb2470: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb2474: beq 0xb245c <entry+0x5d0c>
    c.b(C::Eq,0xb245c,false)?;
    // ARM 0xb2478: ldr r3, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // ARM 0xb247c: mov r8, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,3758096384)?)?;
    // ARM 0xb2480: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // ARM 0xb2484: orr r1, r3, #868
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(3)?,868)?)?;
    // ARM 0xb2488: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // ARM 0xb248c: str r2, [r1, r8]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb2490: orr r1, r3, #2912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(3)?,2912)?)?;
    // ARM 0xb2494: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // ARM 0xb2498: str r0, [r1, r8]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb249c: orr r0, r3, #3040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,3040)?)?;
    // ARM 0xb24a0: mvn r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,255)?)?;
    // ARM 0xb24a4: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // ARM 0xb24a8: str r2, [r0, r8]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb24ac: orr r0, r3, #988
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,988)?)?;
    // ARM 0xb24b0: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // ARM 0xb24b4: str r1, [r0, r8]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb24b8: ldr r0, [pc, #0x594]
    c.literal(R::R0,0xb2a54)?;
    // ARM 0xb24bc: str r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,56)?)?;
    // ARM 0xb24c0: ldr r0, [r4, #0x354]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,852)?)?;
    // ARM 0xb24c4: bl 0x293c <entry+0xfffffffffff561ec>
    c.b(C::Al,0x293c,true)?;
    // ARM 0xb24c8: bl 0x3070 <entry+0xfffffffffff56920>
    c.b(C::Al,0x3070,true)?;
    // ARM 0xb24cc: mov r0, #218103808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,218103808)?)?;
    // ARM 0xb24d0: str r0, [r9, r8]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::new(9)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb24d4: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // ARM 0xb24d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb24dc: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xb24e0: ldrne r1, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,56)?)?;
    // ARM 0xb24e4: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb24e8: bl 0x26658 <entry+0xfffffffffff79f08>
    c.b(C::Al,0x26658,true)?;
    // ARM 0xb24ec: bl 0x3074 <entry+0xfffffffffff56924>
    c.b(C::Al,0x3074,true)?;
    // ARM 0xb24f0: ldr r0, [r4, #0x354]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,852)?)?;
    // ARM 0xb24f4: bl 0x294c <entry+0xfffffffffff561fc>
    c.b(C::Al,0x294c,true)?;
    // ARM 0xb24f8: ldr r0, [pc, #0x558]
    c.literal(R::R0,0xb2a58)?;
    // ARM 0xb24fc: ldr r0, [r0, r6, lsl #2]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,2,false)?)?;
    // ARM 0xb2500: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb2504: beq 0xb2538 <entry+0x5de8>
    c.b(C::Eq,0xb2538,false)?;
    // ARM 0xb2508: ldr r1, [pc, #0x54c]
    c.literal(R::R1,0xb2a5c)?;
    // ARM 0xb250c: ldr r2, [pc, #0x534]
    c.literal(R::R2,0xb2a48)?;
    // ARM 0xb2510: mov r3, #600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,600)?)?;
    // ARM 0xb2514: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM 0xb2518: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // ARM 0xb251c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb2520: ldr r1, [pc, #0x524]
    c.literal(R::R1,0xb2a4c)?;
    // ARM 0xb2524: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM 0xb2528: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb252c: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // ARM 0xb2530: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xb2534: b 0xb1cc8 <entry+0x5578>
    c.b(C::Al,0xb1cc8,false)?;
    // ARM 0xb2538: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // ARM 0xb253c: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb2540: orr r0, r2, #120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,120)?)?;
    // ARM 0xb2544: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb2548: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb254c: tst r0, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // ARM 0xb2550: beq 0xb269c <entry+0x5f4c>
    c.b(C::Eq,0xb269c,false)?;
    // ARM 0xb2554: orr r2, r2, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,124)?)?;
    // ARM 0xb2558: mov r3, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1879048192)?)?;
    // ARM 0xb255c: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // ARM 0xb2560: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb2564: ldr r1, [pc, #0x4f4]
    c.literal(R::R1,0xb2a60)?;
    // ARM 0xb2568: ldr r2, [pc, #0x4d8]
    c.literal(R::R2,0xb2a48)?;
    // ARM 0xb256c: stm sp, {r0, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xb2570: mov r3, #93
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,93)?)?;
    // ARM 0xb2574: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb2578: orr r3, r3, #2560
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2560)?)?;
    // ARM 0xb257c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb2580: ldr r1, [pc, #0x4c4]
    c.literal(R::R1,0xb2a4c)?;
    // ARM 0xb2584: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb2588: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb258c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb2590: mov r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,255)?)?;
    // ARM 0xb2594: str r0, [r4, #0xfac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4012)?)?;
    // ARM 0xb2598: b 0xb25ec <entry+0x5e9c>
    c.b(C::Al,0xb25ec,false)?;
    // ARM 0xb259c: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb25a0: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // ARM 0xb25a4: ldr r1, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xb25a8: add r1, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,3)?)?;
    // ARM 0xb25ac: strb r1, [r5, #0xab]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,171)?)?;
    // ARM 0xb25b0: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // ARM 0xb25b4: add r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb25b8: strb r1, [r5, #0xac]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,172)?)?;
    // ARM 0xb25bc: ldr r1, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // ARM 0xb25c0: strb r1, [r5, #0xb3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,179)?)?;
    // ARM 0xb25c4: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // ARM 0xb25c8: str r1, [r9, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb25cc: ldr r0, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb25d0: and r1, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,255)?)?;
    // ARM 0xb25d4: strb r0, [r5, #0xb4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,180)?)?;
    // ARM 0xb25d8: cmp r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,64)?)?;
    // ARM 0xb25dc: bls 0xb25f8 <entry+0x5ea8>
    c.b(C::Ls,0xb25f8,false)?;
    // ARM 0xb25e0: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // ARM 0xb25e4: strb r0, [r5, #0xb4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,180)?)?;
    // ARM 0xb25e8: mov r8, #75
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,75)?)?;
    // ARM 0xb25ec: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // ARM 0xb25f0: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // ARM 0xb25f4: b 0xb1c9c <entry+0x554c>
    c.b(C::Al,0xb1c9c,false)?;
    // ARM 0xb25f8: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // ARM 0xb25fc: beq 0xb263c <entry+0x5eec>
    c.b(C::Eq,0xb263c,false)?;
    // ARM 0xb2600: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // ARM 0xb2604: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM 0xb2608: ldr r0, [pc, #0x454]
    c.literal(R::R0,0xb2a64)?;
    // ARM 0xb260c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xb2610: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // ARM 0xb2614: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xb2618: ldr r12, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,0)?)?;
    // ARM 0xb261c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xb2620: blx r12
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R12)?)?;
    // ARM 0xb2624: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb2628: bne 0xb25ec <entry+0x5e9c>
    c.b(C::Ne,0xb25ec,false)?;
    // ARM 0xb262c: ldrb r1, [r5, #0xb4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,180)?)?;
    // ARM 0xb2630: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // ARM 0xb2634: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb2638: blt 0xb2608 <entry+0x5eb8>
    c.b(C::Lt,0xb2608,false)?;
    // ARM 0xb263c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xb2640: bl 0xaf694 <entry+0x2f44>
    c.b(C::Al,0xaf694,true)?;
    // ARM 0xb2644: mov r8, #75
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,75)?)?;
    // ARM 0xb2648: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb264c: bne 0xb25ec <entry+0x5e9c>
    c.b(C::Ne,0xb25ec,false)?;
    // ARM 0xb2650: mov lr, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,3758096384)?)?;
    // ARM 0xb2654: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb2658: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb265c: str r0, [r9, lr]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::new(9)?,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xb2660: strb r1, [r5, #0x1bd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,445)?)?;
    // ARM 0xb2664: ldr r1, [r7, lr]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xb2668: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // ARM 0xb266c: strb r1, [r5, #0x1bc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,444)?)?;
    // ARM 0xb2670: beq 0xb2728 <entry+0x5fd8>
    c.b(C::Eq,0xb2728,false)?;
    // ARM 0xb2674: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // ARM 0xb2678: str r1, [r9, lr]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(9)?,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xb267c: ldr r1, [r7, lr]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xb2680: and r2, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,255)?)?;
    // ARM 0xb2684: strb r1, [r5, #0x1bd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,445)?)?;
    // ARM 0xb2688: cmp r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // ARM 0xb268c: bls 0xb26d8 <entry+0x5f88>
    c.b(C::Ls,0xb26d8,false)?;
    // ARM 0xb2690: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // ARM 0xb2694: strb r0, [r5, #0x1bd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,445)?)?;
    // ARM 0xb2698: b 0xb25ec <entry+0x5e9c>
    c.b(C::Al,0xb25ec,false)?;
    // ARM 0xb269c: ldr r0, [r7, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb26a0: mov r8, #34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,34)?)?;
    // ARM 0xb26a4: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // ARM 0xb26a8: str r0, [r5, #0x270]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,624)?)?;
    // ARM 0xb26ac: beq 0xb25ec <entry+0x5e9c>
    c.b(C::Eq,0xb25ec,false)?;
    // ARM 0xb26b0: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb26b4: ldr r1, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb26b8: cmn r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // ARM 0xb26bc: str r1, [r5, #0x274]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,628)?)?;
    // ARM 0xb26c0: beq 0xb25ec <entry+0x5e9c>
    c.b(C::Eq,0xb25ec,false)?;
    // ARM 0xb26c4: ldr r0, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb26c8: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // ARM 0xb26cc: str r0, [r5, #0x278]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,632)?)?;
    // ARM 0xb26d0: bne 0xb236c <entry+0x5c1c>
    c.b(C::Ne,0xb236c,false)?;
    // ARM 0xb26d4: b 0xb25ec <entry+0x5e9c>
    c.b(C::Al,0xb25ec,false)?;
    // ARM 0xb26d8: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // ARM 0xb26dc: beq 0xb2728 <entry+0x5fd8>
    c.b(C::Eq,0xb2728,false)?;
    // ARM 0xb26e0: add r2, r5, #576
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,576)?)?;
    // ARM 0xb26e4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xb26e8: mov r6, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // ARM 0xb26ec: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // ARM 0xb26f0: ldrb r0, [r5, #0x73]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,115)?)?;
    // ARM 0xb26f4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xb26f8: str r0, [r1, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb26fc: add r0, r2, r3, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::new(3)?,Shift::Lsl,2,false)?)?;
    // ARM 0xb2700: str r8, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,0)?)?;
    // ARM 0xb2704: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xb2708: ldr r12, [r1, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R12,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb270c: str r12, [r0, #-0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,-128)?)?;
    // ARM 0xb2710: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb2714: strb r0, [r2, r3]
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb2718: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // ARM 0xb271c: ldrb r0, [r5, #0x1bd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,445)?)?;
    // ARM 0xb2720: cmp r3, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2724: blt 0xb26f0 <entry+0x5fa0>
    c.b(C::Lt,0xb26f0,false)?;
    // ARM 0xb2728: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // ARM 0xb272c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb2730: str r1, [r0, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R0,R::LR,Shift::Lsl,0)?)?;
    // ARM 0xb2734: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // ARM 0xb2738: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // ARM 0xb273c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM 0xb2740: ldr r1, [r0, lr]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::LR,Shift::Lsl,0)?)?;
    // ARM 0xb2744: strb r1, [r5, #0x260]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,608)?)?;
    // ARM 0xb2748: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // ARM 0xb274c: strb r1, [r5, #0x261]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,609)?)?;
    // ARM 0xb2750: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // ARM 0xb2754: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // ARM 0xb2758: strb r0, [r5, #0x262]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,610)?)?;
    // ARM 0xb275c: beq 0xb2a1c <entry+0x62cc>
    c.b(C::Eq,0xb2a1c,false)?;
    // ARM 0xb2760: ldr r2, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,36)?)?;
    // ARM 0xb2764: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb2768: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb276c: strb r0, [r5, #0x29f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,671)?)?;
    // ARM 0xb2770: mov r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048)?)?;
    // ARM 0xb2774: orr r3, r0, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::R2,Shift::Lsl,15,false)?)?;
    // ARM 0xb2778: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb277c: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // ARM 0xb2780: str r1, [r3, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2784: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // ARM 0xb2788: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // ARM 0xb278c: orr r8, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // ARM 0xb2790: mov r3, #648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,648)?)?;
    // ARM 0xb2794: ldr r2, [r8, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2798: strh r2, [r5, r3]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?)?)?;
    // ARM 0xb279c: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // ARM 0xb27a0: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // ARM 0xb27a4: tst r2, r3
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb27a8: beq 0xb2810 <entry+0x60c0>
    c.b(C::Eq,0xb2810,false)?;
    // ARM 0xb27ac: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // ARM 0xb27b0: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // ARM 0xb27b4: str r2, [r3, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb27b8: mov r3, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,138)?)?;
    // ARM 0xb27bc: ldr r2, [r8, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb27c0: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // ARM 0xb27c4: strh r2, [r5, r3]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?)?)?;
    // ARM 0xb27c8: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // ARM 0xb27cc: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // ARM 0xb27d0: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb27d4: cmp r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,255)?)?;
    // ARM 0xb27d8: bne 0xb2810 <entry+0x60c0>
    c.b(C::Ne,0xb2810,false)?;
    // ARM 0xb27dc: ldr r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,16)?)?;
    // ARM 0xb27e0: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // ARM 0xb27e4: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // ARM 0xb27e8: str r3, [r6, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::new(6)?,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb27ec: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // ARM 0xb27f0: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // ARM 0xb27f4: mov r6, #652
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,652)?)?;
    // ARM 0xb27f8: ldr r2, [r3, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // ARM 0xb27fc: strh r2, [r5, r6]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(6)?)?)?;
    // ARM 0xb2800: ldr r2, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb2804: mov r3, #142
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,142)?)?;
    // ARM 0xb2808: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // ARM 0xb280c: strh r2, [r5, r3]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::new(5)?,R::new(3)?)?)?;
    // ARM 0xb2810: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // ARM 0xb2814: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2818: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb281c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb2820: ldrne r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::SP,16)?)?;
    // ARM 0xb2824: movne r0, #-536870912
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb2828: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb282c: strne r1, [r2, r0]
    c.emit(arm32::transfer_word_register_offset(C::Ne,false,R::R1,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2830: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb2834: ldrne r0, [r8, r0]
    c.emit(arm32::transfer_word_register_offset(C::Ne,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2838: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // ARM 0xb283c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb2840: str r0, [r2, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb2844: ldr r2, [r8, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb2848: tst r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,255)?)?;
    // ARM 0xb284c: strb r2, [r5, #0x6dc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(5)?,1756)?)?;
    // ARM 0xb2850: beq 0xb2900 <entry+0x61b0>
    c.b(C::Eq,0xb2900,false)?;
    // ARM 0xb2854: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // ARM 0xb2858: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // ARM 0xb285c: str r2, [r3, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xb2860: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // ARM 0xb2864: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // ARM 0xb2868: ldr r1, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xb286c: str r1, [r5, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,1760)?)?;
    // ARM 0xb2870: ldr r1, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // ARM 0xb2874: strb r1, [r5, #0x6dd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,1757)?)?;
    // ARM 0xb2878: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // ARM 0xb287c: ldr r1, [pc, #0x1e4]
    c.literal(R::R1,0xb2a68)?;
    // ARM 0xb2880: ldr r0, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,0)?)?;
    // ARM 0xb2884: strb r0, [r5, #0x6de]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,1758)?)?;
    // ARM 0xb2888: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // ARM 0xb288c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb2890: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb2894: ldr r1, [pc, #0x1b0]
    c.literal(R::R1,0xb2a4c)?;
    // ARM 0xb2898: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb289c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb28a0: ldrb r0, [r5, #0x6de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,1758)?)?;
    // ARM 0xb28a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb28a8: beq 0xb291c <entry+0x61cc>
    c.b(C::Eq,0xb291c,false)?;
    // ARM 0xb28ac: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // ARM 0xb28b0: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb28b4: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // ARM 0xb28b8: str r1, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb28bc: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb28c0: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb28c4: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // ARM 0xb28c8: ldr r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb28cc: str r2, [r5, #0x6e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,1764)?)?;
    // ARM 0xb28d0: ldr r3, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,0)?)?;
    // ARM 0xb28d4: str r3, [r5, #0x6e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,1768)?)?;
    // ARM 0xb28d8: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb28dc: ldr r1, [pc, #0x188]
    c.literal(R::R1,0xb2a6c)?;
    // ARM 0xb28e0: str r0, [r5, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1772)?)?;
    // ARM 0xb28e4: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xb28e8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb28ec: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb28f0: ldr r1, [pc, #0x154]
    c.literal(R::R1,0xb2a4c)?;
    // ARM 0xb28f4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb28f8: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb28fc: b 0xb291c <entry+0x61cc>
    c.b(C::Al,0xb291c,false)?;
    // ARM 0xb2900: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb2904: strb r0, [r5, #0x6dd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,1757)?)?;
    // ARM 0xb2908: strb r0, [r5, #0x6de]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,1758)?)?;
    // ARM 0xb290c: str r0, [r5, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1760)?)?;
    // ARM 0xb2910: str r0, [r5, #0x6e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1764)?)?;
    // ARM 0xb2914: str r0, [r5, #0x6e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1768)?)?;
    // ARM 0xb2918: str r0, [r5, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1772)?)?;
    // ARM 0xb291c: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // ARM 0xb2920: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb2924: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb2928: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb292c: ldr r1, [r8, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2930: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // ARM 0xb2934: strb r1, [r5, #0x6df]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,1759)?)?;
    // ARM 0xb2938: beq 0xb2960 <entry+0x6210>
    c.b(C::Eq,0xb2960,false)?;
    // ARM 0xb293c: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // ARM 0xb2940: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // ARM 0xb2944: str r1, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb2948: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb294c: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // ARM 0xb2950: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb2954: str r0, [r5, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1776)?)?;
    // ARM 0xb2958: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb295c: b 0xb2968 <entry+0x6218>
    c.b(C::Al,0xb2968,false)?;
    // ARM 0xb2960: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb2964: str r0, [r5, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1776)?)?;
    // ARM 0xb2968: str r0, [r5, #0x6f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,1780)?)?;
    // ARM 0xb296c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb2970: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb2974: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // ARM 0xb2978: str r1, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb297c: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb2980: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb2984: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb2988: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // ARM 0xb298c: ldr r3, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb2990: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // ARM 0xb2994: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // ARM 0xb2998: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // ARM 0xb299c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xb29a0: beq 0xb29d0 <entry+0x6280>
    c.b(C::Eq,0xb29d0,false)?;
    // ARM 0xb29a4: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // ARM 0xb29a8: mov r2, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16777216)?)?;
    // ARM 0xb29ac: str r2, [r3, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb29b0: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb29b4: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb29b8: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb29bc: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // ARM 0xb29c0: ldr r3, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb29c4: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // ARM 0xb29c8: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // ARM 0xb29cc: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // ARM 0xb29d0: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // ARM 0xb29d4: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb29d8: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb29dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb29e0: beq 0xb2a08 <entry+0x62b8>
    c.b(C::Eq,0xb2a08,false)?;
    // ARM 0xb29e4: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // ARM 0xb29e8: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb29ec: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // ARM 0xb29f0: str r1, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb29f4: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb29f8: ldr r0, [r8, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb29fc: str r0, [r5, #0x298]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,664)?)?;
    // ARM 0xb2a00: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // ARM 0xb2a04: b 0xb2a18 <entry+0x62c8>
    c.b(C::Al,0xb2a18,false)?;
    // ARM 0xb2a08: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // ARM 0xb2a0c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // ARM 0xb2a10: str r0, [r5, #0x298]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,664)?)?;
    // ARM 0xb2a14: strb r1, [r5, #0x29c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,668)?)?;
    // ARM 0xb2a18: str r0, [r5, #0x290]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,656)?)?;
    // ARM 0xb2a1c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb2a20: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb2a24: str r1, [r9, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2a28: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xb2a2c: ldr r0, [r7, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb2a30: str r0, [r5, #0x264]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,612)?)?;
    // ARM 0xb2a34: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xb2a38: bl 0xaf5c0 <entry+0x2e70>
    c.b(C::Al,0xaf5c0,true)?;
    // ARM 0xb2a3c: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // ARM 0xb2a40: b 0xb25ec <entry+0x5e9c>
    c.b(C::Al,0xb25ec,false)?;
    c.check(0xb2a44)?;
    // ARM 0xb2a44: literal or constant data (NOT executable)
    c.emit(0x000f8b1b)?;
    // ARM 0xb2a48: literal or constant data (NOT executable)
    c.emit(0x000f8b5c)?;
    // ARM 0xb2a4c: literal or constant data (NOT executable)
    c.emit(0x000e2788)?;
    // ARM 0xb2a50: literal or constant data (NOT executable)
    c.emit(0x000f88c7)?;
    // ARM 0xb2a54: literal or constant data (NOT executable)
    c.emit(0x05f5e100)?;
    // ARM 0xb2a58: literal or constant data (NOT executable)
    c.emit(0x0010408c)?;
    // ARM 0xb2a5c: literal or constant data (NOT executable)
    c.emit(0x000f8965)?;
    // ARM 0xb2a60: literal or constant data (NOT executable)
    c.emit(0x000f7058)?;
    // ARM 0xb2a64: literal or constant data (NOT executable)
    c.emit(0x0010dd88)?;
    // ARM 0xb2a68: literal or constant data (NOT executable)
    c.emit(0x000f8b6d)?;
    // ARM 0xb2a6c: literal or constant data (NOT executable)
    c.emit(0x000f8b99)?;
    // ARM 0xb2a70: literal or constant data (NOT executable)
    c.emit(0x000b0448)?;
    // ARM 0xb2a74: literal or constant data (NOT executable)
    c.emit(0x000f8991)?;
    // ARM 0xb2a78: literal or constant data (NOT executable)
    c.emit(0x000f7095)?;
    // ARM 0xb2a7c: literal or constant data (NOT executable)
    c.emit(0x000f7125)?;
    c.check(0xb2a80)
}
