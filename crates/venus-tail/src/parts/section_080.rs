
/// Serialize codec controller frame descriptor flags, update the indexed
/// plane table and synchronize stream buffer command/state entries. Keep
/// the original tail return and its five diagnostic words as a data pool.
fn configure_runtime_codec_frame_plane_tables(c:&mut Code)->Result<(),String>{
    c.check(0xc1f90)?;
    // VMA 0xc1f90: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc1f94: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc1f98: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // VMA 0xc1f9c: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // VMA 0xc1fa0: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // VMA 0xc1fa4: mov r0, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,458752)?)?;
    // VMA 0xc1fa8: mov r2, #3670016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3670016)?)?;
    // VMA 0xc1fac: mov r3, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,192)?)?;
    // VMA 0xc1fb0: ldr r1, [r6, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,192)?)?;
    // VMA 0xc1fb4: ldr r10, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(6)?,4)?)?;
    // VMA 0xc1fb8: ldr r7, [r6, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,76)?)?;
    // VMA 0xc1fbc: and r0, r0, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,16,false)?)?;
    // VMA 0xc1fc0: and r1, r2, r1, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,19,false)?)?;
    // VMA 0xc1fc4: ldr r2, [r6, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,1296)?)?;
    // VMA 0xc1fc8: and r2, r3, r2, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,6,false)?)?;
    // VMA 0xc1fcc: ldr r3, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    // VMA 0xc1fd0: ldr r3, [r3, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,16)?)?;
    // VMA 0xc1fd4: str r7, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,28)?)?;
    // VMA 0xc1fd8: cmp r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4096)?)?;
    // VMA 0xc1fdc: bne 0xc1fe8 <entry+0x3478>  @ imm = #0x4
    c.b(C::Ne,0xc1fe8,false)?;
    // VMA 0xc1fe0: ldrb r4, [r6, #0x508]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,1288)?)?;
    // VMA 0xc1fe4: b 0xc1ff4 <entry+0x3484>  @ imm = #0x8
    c.b(C::Al,0xc1ff4,false)?;
    // VMA 0xc1fe8: ldr r7, [r6, #0x50c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,1292)?)?;
    // VMA 0xc1fec: ldrb r4, [r6, #0x508]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::new(6)?,1288)?)?;
    // VMA 0xc1ff0: orr r4, r4, r7, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(4)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // VMA 0xc1ff4: mov r12, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,7)?)?;
    // VMA 0xc1ff8: mov r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // VMA 0xc1ffc: cmp r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,4096)?)?;
    // VMA 0xc2000: mov r9, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,3758096384)?)?;
    // VMA 0xc2004: mov r3, #458752
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,458752)?)?;
    // VMA 0xc2008: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xc200c: orr r12, r12, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,16384)?)?;
    // VMA 0xc2010: orr r7, r7, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,16384)?)?;
    // VMA 0xc2014: moveq r7, r12
    c.emit(arm32::mov_register(C::Eq,R::new(7)?,R::R12))?;
    // VMA 0xc2018: orr r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc201c: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc2020: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc2024: mov r1, #25165824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,25165824)?)?;
    // VMA 0xc2028: and r1, r1, r4, lsl #23
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(4)?,Shift::Lsl,23,false)?)?;
    // VMA 0xc202c: ldr r4, [pc, #0x82c]        @ 0x3cf0 <entry+0x3cf0>
    c.literal(R::new(4)?,0xc2860)?;
    // VMA 0xc2030: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc2034: mov r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3072)?)?;
    // VMA 0xc2038: orr r1, r1, r10, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(10)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc203c: str r0, [r1, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc2040: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc2044: ldr r2, [r6, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,112)?)?;
    // VMA 0xc2048: ldr r1, [r6, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,108)?)?;
    // VMA 0xc204c: orr r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3072)?)?;
    // VMA 0xc2050: orr r0, r0, r10, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(10)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc2054: add r2, r3, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,16,false)?)?;
    // VMA 0xc2058: mov r3, #66584576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,66584576)?)?;
    // VMA 0xc205c: add r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,7)?)?;
    // VMA 0xc2060: orr r3, r3, #-67108864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4227858432)?)?;
    // VMA 0xc2064: bic r1, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,7)?)?;
    // VMA 0xc2068: and r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc206c: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc2070: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xc2074: str r1, [r0, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc2078: mov r0, #3504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3504)?)?;
    // VMA 0xc207c: mov r1, #324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,324)?)?;
    // VMA 0xc2080: orr r0, r0, r10, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(10)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc2084: str r8, [r0, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(8)?,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc2088: ldr r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,20)?)?;
    // VMA 0xc208c: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xc08e4
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc2090: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // VMA 0xc2094: mov r1, #70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,70)?)?;
    // VMA 0xc2098: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xc209c: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xc20a0: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xc08f8
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc20a4: mov r0, #860
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,860)?)?;
    // VMA 0xc20a8: mov r1, #97
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,97)?)?;
    // VMA 0xc20ac: mov r2, #984
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,984)?)?;
    // VMA 0xc20b0: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc20b4: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc20b8: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // VMA 0xc20bc: orr r12, r0, r10, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R12,R::R0,R::new(10)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc20c0: mov r0, #856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,856)?)?;
    // VMA 0xc20c4: orr r7, r2, r10, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R2,R::new(10)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc20c8: str r1, [r12, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R12,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc20cc: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xc20d0: str r6, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,32)?)?;
    // VMA 0xc20d4: orr lr, r0, r10, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::R0,R::new(10)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc20d8: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    // VMA 0xc20dc: str r0, [lr, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::LR,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc20e0: str r1, [r7, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(7)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc20e4: mov r1, #980
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,980)?)?;
    // VMA 0xc20e8: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xc20ec: orr r4, r1, r10, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R1,R::new(10)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc20f0: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // VMA 0xc20f4: lsl r10, r10, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(10)?,R::new(10)?,Shift::Lsl,15)?)?;
    // VMA 0xc20f8: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xc20fc: str r1, [r4, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc2100: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // VMA 0xc2104: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // VMA 0xc2108: ldr r2, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,16)?)?;
    // VMA 0xc210c: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // VMA 0xc2110: bne 0xc214c <entry+0x35dc>  @ imm = #0x34
    c.b(C::Ne,0xc214c,false)?;
    // VMA 0xc2114: mov r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // VMA 0xc2118: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc211c: mov r3, #112
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,112)?)?;
    // VMA 0xc2120: mov r5, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xc2124: orr r9, r9, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,4096)?)?;
    // VMA 0xc2128: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xc212c: orr r6, r3, r9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(3)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2130: add r3, r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,16)?)?;
    // VMA 0xc2134: cmp r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8)?)?;
    // VMA 0xc2138: str r6, [r12, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R12,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc213c: str r0, [lr, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::LR,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc2140: str r6, [r7, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc2144: str r1, [r4, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xc2148: bne 0xc2128 <entry+0x35b8>  @ imm = #-0x28
    c.b(C::Ne,0xc2128,false)?;
    // VMA 0xc214c: add r0, r10, #3440
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,3440)?)?;
    // VMA 0xc2150: str r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // VMA 0xc2154: mov r9, #97
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,97)?)?;
    // VMA 0xc2158: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // VMA 0xc215c: str r12, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,20)?)?;
    // VMA 0xc2160: str lr, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,16)?)?;
    // VMA 0xc2164: str r7, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
    // VMA 0xc2168: add r10, r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::R0,3758096384)?)?;
    // VMA 0xc216c: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xc2170: add r6, r0, #180
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,180)?)?;
    // VMA 0xc2174: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // VMA 0xc2178: mov r7, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3758096384)?)?;
    // VMA 0xc217c: str r9, [r1, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(9)?,R::R1,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2180: ldr r0, [r6, #-0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,-8)?)?;
    // VMA 0xc2184: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,16)?)?;
    // VMA 0xc2188: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xc218c: str r2, [r3, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2190: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // VMA 0xc2194: str r9, [r2, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(9)?,R::R2,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc2198: ldr r5, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,16)?)?;
    // VMA 0xc219c: ldr r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,8)?)?;
    // VMA 0xc21a0: bic r5, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(5)?,R::new(5)?,255)?)?;
    // VMA 0xc21a4: str r5, [r4, r7]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(5)?,R::new(4)?,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xc21a8: add r5, r9, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(9)?,16)?)?;
    // VMA 0xc21ac: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // VMA 0xc21b0: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // VMA 0xc21b4: ldr r7, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    // VMA 0xc21b8: str r7, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(3)?,0)?)?;
    // VMA 0xc21bc: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // VMA 0xc21c0: ldr r1, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,16)?)?;
    // VMA 0xc21c4: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xc21c8: str r1, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // VMA 0xc21cc: bl 0xd9e60 <entry+0x1b2f0> @ imm = #0x17c8c
    c.b(C::Al,0xd9e60,true)?;
    // VMA 0xc21d0: bic r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // VMA 0xc21d4: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // VMA 0xc21d8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc21dc: bl 0xd9e60 <entry+0x1b2f0> @ imm = #0x17c7c
    c.b(C::Al,0xd9e60,true)?;
    // VMA 0xc21e0: bic r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    // VMA 0xc21e4: str r0, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,16)?)?;
    // VMA 0xc21e8: ldr r0, [r6, #-0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,-8)?)?;
    // VMA 0xc21ec: bl 0xd9e60 <entry+0x1b2f0> @ imm = #0x17c6c
    c.b(C::Al,0xd9e60,true)?;
    // VMA 0xc21f0: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xc21f4: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc21f8: bl 0xd9e60 <entry+0x1b2f0> @ imm = #0x17c60
    c.b(C::Al,0xd9e60,true)?;
    // VMA 0xc21fc: ldr r1, [pc, #0x660]        @ 0x3cf4 <entry+0x3cf4>
    c.literal(R::R1,0xc2864)?;
    // VMA 0xc2200: bic r3, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::R0,15)?)?;
    // VMA 0xc2204: bic r2, r5, #15
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::new(5)?,15)?)?;
    // VMA 0xc2208: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc220c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x93580
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc2210: ldr r1, [pc, #0x650]        @ 0x3cf8 <entry+0x3cf8>
    c.literal(R::R1,0xc2868)?;
    // VMA 0xc2214: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc2218: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x9358c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc221c: add r8, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // VMA 0xc2220: add r10, r10, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,4)?)?;
    // VMA 0xc2224: add r6, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,4)?)?;
    // VMA 0xc2228: add r9, r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,32)?)?;
    // VMA 0xc222c: cmp r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // VMA 0xc2230: bne 0xc2174 <entry+0x3604>  @ imm = #-0xc4
    c.b(C::Ne,0xc2174,false)?;
    // VMA 0xc2234: ldr r1, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,0)?)?;
    // VMA 0xc2238: ldr lr, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,32)?)?;
    // VMA 0xc223c: mov r6, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // VMA 0xc2240: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xc2244: orr r0, r1, #360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R1,360)?)?;
    // VMA 0xc2248: ldr r5, [lr, #0x204]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::LR,516)?)?;
    // VMA 0xc224c: orr r1, r1, #364
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,364)?)?;
    // VMA 0xc2250: add r7, lr, #1936
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::LR,1936)?)?;
    // VMA 0xc2254: orr r8, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::R0,14336)?)?;
    // VMA 0xc2258: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xc225c: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xc2260: add r10, r0, #172
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::R0,172)?)?;
    // VMA 0xc2264: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc2268: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xc226c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc2270: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // VMA 0xc2274: ldr r2, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,0)?)?;
    // VMA 0xc2278: ldr r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,12)?)?;
    // VMA 0xc227c: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xc2280: sub r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2284: ldr r3, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,8)?)?;
    // VMA 0xc2288: ldr r0, [r3, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,12)?)?;
    // VMA 0xc228c: ldr r3, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // VMA 0xc2290: streq r2, [r7, #-0x8]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(7)?,-8)?)?;
    // VMA 0xc2294: sub r12, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R12,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2298: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc229c: streq r12, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::R12,R::new(7)?,0)?)?;
    // VMA 0xc22a0: ldreq r5, [lr, #0x204]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::LR,516)?)?;
    // VMA 0xc22a4: str r7, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,28)?)?;
    // VMA 0xc22a8: orr r7, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc22ac: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc22b0: str r7, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc22b4: beq 0xc22c8 <entry+0x3758>  @ imm = #0xc
    c.b(C::Eq,0xc22c8,false)?;
    // VMA 0xc22b8: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // VMA 0xc22bc: mov r7, r12
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R12))?;
    // VMA 0xc22c0: moveq r7, r2
    c.emit(arm32::mov_register(C::Eq,R::new(7)?,R::R2))?;
    // VMA 0xc22c4: b 0xc22ec <entry+0x377c>  @ imm = #0x20
    c.b(C::Al,0xc22ec,false)?;
    // VMA 0xc22c8: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // VMA 0xc22cc: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // VMA 0xc22d0: mov r7, r12
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R12))?;
    // VMA 0xc22d4: moveq r7, r2
    c.emit(arm32::mov_register(C::Eq,R::new(7)?,R::R2))?;
    // VMA 0xc22d8: ldr lr, [r0, #0x20c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,524)?)?;
    // VMA 0xc22dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc22e0: cmp lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,1)?)?;
    // VMA 0xc22e4: ldr lr, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,32)?)?;
    // VMA 0xc22e8: movne r7, r0
    c.emit(arm32::mov_register(C::Ne,R::new(7)?,R::R0))?;
    // VMA 0xc22ec: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
    // VMA 0xc22f0: add r3, r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,16)?)?;
    // VMA 0xc22f4: str r7, [r8, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(8)?,R::new(6)?,Shift::Lsl,0)?)?;
    // VMA 0xc22f8: cmp r9, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,2)?)?;
    // VMA 0xc22fc: bne 0xc22a8 <entry+0x3738>  @ imm = #-0x5c
    c.b(C::Ne,0xc22a8,false)?;
    // VMA 0xc2300: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xc2304: ldr r7, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,28)?)?;
    // VMA 0xc2308: add r10, r10, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,4)?)?;
    // VMA 0xc230c: add r4, r4, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,65536)?)?;
    // VMA 0xc2310: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xc2314: add r7, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,4)?)?;
    // VMA 0xc2318: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xc231c: bne 0xc2268 <entry+0x36f8>  @ imm = #-0xbc
    c.b(C::Ne,0xc2268,false)?;
    // VMA 0xc2320: ldr r0, [lr, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,1296)?)?;
    // VMA 0xc2324: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc2328: beq 0xc238c <entry+0x381c>  @ imm = #0x5c
    c.b(C::Eq,0xc238c,false)?;
    // VMA 0xc232c: ldr r10, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,4)?)?;
    // VMA 0xc2330: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // VMA 0xc2334: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // VMA 0xc2338: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xc233c: bne 0xc23e8 <entry+0x3878>  @ imm = #0xa4
    c.b(C::Ne,0xc23e8,false)?;
    // VMA 0xc2340: mov r4, r10
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(10)?))?;
    // VMA 0xc2344: ldr r0, [r4, #0x1d4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(4)?,468)?)?;
    // VMA 0xc2348: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc234c: bne 0xc2374 <entry+0x3804>  @ imm = #0x20
    c.b(C::Ne,0xc2374,false)?;
    // VMA 0xc2350: ldr r0, [lr, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,1272)?)?;
    // VMA 0xc2354: ldr r1, [pc, #0x510]        @ 0x3cfc <entry+0x3cfc>
    c.literal(R::R1,0xc286c)?;
    // VMA 0xc2358: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc235c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc2360: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x936d4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc2364: ldr r1, [pc, #0x4fc]        @ 0x3cf8 <entry+0x3cf8>
    c.literal(R::R1,0xc2868)?;
    // VMA 0xc2368: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc236c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x936e0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc2370: ldr lr, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,32)?)?;
    // VMA 0xc2374: ldr r3, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // VMA 0xc2378: mov r1, #65
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,65)?)?;
    // VMA 0xc237c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xc2380: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc2384: str r1, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2388: b 0xc2440 <entry+0x38d0>  @ imm = #0xb0
    c.b(C::Al,0xc2440,false)?;
    // VMA 0xc238c: ldr r0, [lr, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,1272)?)?;
    // VMA 0xc2390: ldr r2, [pc, #0x4c8]        @ 0x3cf0 <entry+0x3cf0>
    c.literal(R::R2,0xc2860)?;
    // VMA 0xc2394: mov r1, #404
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,404)?)?;
    // VMA 0xc2398: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xc0bf0
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc239c: ldr r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xc23a0: ldr lr, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,32)?)?;
    // VMA 0xc23a4: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc23a8: mov r2, #49
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,49)?)?;
    // VMA 0xc23ac: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // VMA 0xc23b0: str r2, [r3, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc23b4: ldr r0, [lr, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,1272)?)?;
    // VMA 0xc23b8: ldr r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,16)?)?;
    // VMA 0xc23bc: str r0, [r6, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc23c0: ldr r7, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,12)?)?;
    // VMA 0xc23c4: str r2, [r7, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc23c8: mov r2, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3840)?)?;
    // VMA 0xc23cc: ldr r8, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,8)?)?;
    // VMA 0xc23d0: str r2, [r8, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(8)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc23d4: mov r1, #65
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,65)?)?;
    // VMA 0xc23d8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc23dc: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xc23e0: ldr r10, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,4)?)?;
    // VMA 0xc23e4: b 0xc247c <entry+0x390c>  @ imm = #0x90
    c.b(C::Al,0xc247c,false)?;
    // VMA 0xc23e8: ldr r5, [pc, #0x470]        @ 0x3cf0 <entry+0x3cf0>
    c.literal(R::new(5)?,0xc2860)?;
    // VMA 0xc23ec: ldr r0, [lr, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,1272)?)?;
    // VMA 0xc23f0: mov r1, #185
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,185)?)?;
    // VMA 0xc23f4: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xc23f8: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xc23fc: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xc0c54
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc2400: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // VMA 0xc2404: add r4, r10, #492
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(10)?,492)?)?;
    // VMA 0xc2408: mov r1, #186
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,186)?)?;
    // VMA 0xc240c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xc2410: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xc2414: ldrb r0, [r0, #0x571]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1393)?)?;
    // VMA 0xc2418: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xc241c: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xc0c74
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc2420: ldr r3, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // VMA 0xc2424: ldr lr, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,32)?)?;
    // VMA 0xc2428: mov r1, #65
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,65)?)?;
    // VMA 0xc242c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xc2430: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc2434: str r1, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc2438: ldrb r0, [lr, #0x571]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::LR,1393)?)?;
    // VMA 0xc243c: add r4, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc2440: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc2444: ldr r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,16)?)?;
    // VMA 0xc2448: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xc244c: mov r1, #49
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,49)?)?;
    // VMA 0xc2450: mov r5, r3
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // VMA 0xc2454: str r0, [r6, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(6)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc2458: mov r0, #65
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,65)?)?;
    // VMA 0xc245c: ldr r7, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,12)?)?;
    // VMA 0xc2460: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xc2464: str r0, [r7, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc2468: mov r0, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3840)?)?;
    // VMA 0xc246c: ldr r8, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,8)?)?;
    // VMA 0xc2470: str r0, [r8, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(8)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc2474: str r1, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc2478: ldr r0, [lr, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,1272)?)?;
    // VMA 0xc247c: mov r9, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,3758096384)?)?;
    // VMA 0xc2480: ldr r4, [pc, #0x3d8]        @ 0x3cf0 <entry+0x3cf0>
    c.literal(R::new(4)?,0xc2860)?;
    // VMA 0xc2484: str r0, [r6, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(6)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc2488: mov r0, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3840)?)?;
    // VMA 0xc248c: str r1, [r7, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::new(7)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc2490: mov r1, #203
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,203)?)?;
    // VMA 0xc2494: str r0, [r8, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(8)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc2498: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xc249c: ldr r0, [lr, #0x210]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::LR,528)?)?;
    // VMA 0xc24a0: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xc24a4: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xc0cfc
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc24a8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc24ac: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xc24b0: str r0, [r5, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(5)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc24b4: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xc24b8: ldr r1, [r1, #0x210]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,528)?)?;
    // VMA 0xc24bc: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // VMA 0xc24c0: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc24c4: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // VMA 0xc24c8: ldr r0, [r0, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,676)?)?;
    // VMA 0xc24cc: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc24d0: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xc24d4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc24d8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xc24dc: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc24e0: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // VMA 0xc24e4: ldr r1, [r1, #0x210]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,528)?)?;
    // VMA 0xc24e8: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // VMA 0xc24ec: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc24f0: mov r1, #464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,464)?)?;
    // VMA 0xc24f4: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // VMA 0xc24f8: ldr r0, [r0, #0x2a4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,676)?)?;
    // VMA 0xc24fc: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc2500: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xc2504: ldr r0, [r10, #0xcc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,204)?)?;
    // VMA 0xc2508: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xc0d60
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc250c: mov r0, #81
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,81)?)?;
    // VMA 0xc2510: ldr r4, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,32)?)?;
    // VMA 0xc2514: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc2518: ldr r1, [r10, #0xcc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,204)?)?;
    // VMA 0xc251c: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // VMA 0xc2520: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc2524: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,0)?)?;
    // VMA 0xc2528: ldr r0, [r1, #0x9bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,2492)?)?;
    // VMA 0xc252c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc2530: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xc2534: mov r0, #49
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,49)?)?;
    // VMA 0xc2538: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xc253c: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc2540: ldr r2, [r10, #0xcc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,204)?)?;
    // VMA 0xc2544: str r2, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // VMA 0xc2548: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc254c: ldr r0, [r1, #0x9bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,2492)?)?;
    // VMA 0xc2550: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xc2554: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xc2558: ldr r3, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,0)?)?;
    // VMA 0xc255c: orr r0, r3, #28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,28)?)?;
    // VMA 0xc2560: mov r8, r3
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(3)?))?;
    // VMA 0xc2564: orr r2, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,3072)?)?;
    // VMA 0xc2568: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc256c: str r0, [r2, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R2,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc2570: ldrb lr, [r4, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::new(4)?,1312)?)?;
    // VMA 0xc2574: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // VMA 0xc2578: movne r3, #33554432
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,33554432)?)?;
    // VMA 0xc257c: strne r3, [r2, r9]
    c.emit(arm32::transfer_word_register(C::Ne,false,R::new(3)?,R::R2,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc2580: orr r2, r8, #436
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,436)?)?;
    // VMA 0xc2584: ldr r3, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,80)?)?;
    // VMA 0xc2588: orr r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3072)?)?;
    // VMA 0xc258c: str r3, [r2, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xc2590: ldr r1, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,16)?)?;
    // VMA 0xc2594: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // VMA 0xc2598: beq 0xc25b8 <entry+0x3a48>  @ imm = #0x18
    c.b(C::Eq,0xc25b8,false)?;
    // VMA 0xc259c: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // VMA 0xc25a0: bne 0xc2854 <entry+0x3ce4>  @ imm = #0x2ac
    c.b(C::Ne,0xc2854,false)?;
    // VMA 0xc25a4: ldr r0, [r4, #0x780]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1920)?)?;
    // VMA 0xc25a8: orr r1, r8, #3712
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(8)?,3712)?)?;
    // VMA 0xc25ac: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xc25b0: str r0, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xc25b4: b 0xc2854 <entry+0x3ce4>  @ imm = #0x298
    c.b(C::Al,0xc2854,false)?;
    // VMA 0xc25b8: orr r1, r8, #56
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(8)?,56)?)?;
    // VMA 0xc25bc: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xc25c0: mov r7, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,32768)?)?;
    // VMA 0xc25c4: mov r6, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2048)?)?;
    // VMA 0xc25c8: mov r5, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1024)?)?;
    // VMA 0xc25cc: orr r2, r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R1,3072)?)?;
    // VMA 0xc25d0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc25d4: str r1, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc25d8: mov r3, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,262144)?)?;
    // VMA 0xc25dc: ldr r2, [r4, #0x514]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1300)?)?;
    // VMA 0xc25e0: and r2, r3, r2, lsl #18
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,18,false)?)?;
    // VMA 0xc25e4: ldrb r3, [r4, #0x546]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1350)?)?;
    // VMA 0xc25e8: and r3, r7, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,15,false)?)?;
    // VMA 0xc25ec: mov r7, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,65536)?)?;
    // VMA 0xc25f0: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc25f4: ldrb r3, [r4, #0x547]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1351)?)?;
    // VMA 0xc25f8: and r3, r7, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // VMA 0xc25fc: mov r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8192)?)?;
    // VMA 0xc2600: orr r3, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2604: ldrb r2, [r4, #0x522]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1314)?)?;
    // VMA 0xc2608: and r7, r7, r2, lsl #13
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(7)?,R::R2,Shift::Lsl,13,false)?)?;
    // VMA 0xc260c: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2610: ldrb r7, [r4, #0x570]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(4)?,1392)?)?;
    // VMA 0xc2614: and r7, r6, r7, lsl #11
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,11,false)?)?;
    // VMA 0xc2618: mov r6, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,131072)?)?;
    // VMA 0xc261c: orr r7, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2620: ldrb r3, [r4, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1355)?)?;
    // VMA 0xc2624: and r6, r6, r3, lsl #17
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(6)?,R::new(3)?,Shift::Lsl,17,false)?)?;
    // VMA 0xc2628: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc262c: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2630: ldrb r6, [r4, #0x56c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,1388)?)?;
    // VMA 0xc2634: and r6, r5, r6, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,10,false)?)?;
    // VMA 0xc2638: mov r5, #896
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,896)?)?;
    // VMA 0xc263c: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2640: ldrb r6, [r4, #0x56e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,1390)?)?;
    // VMA 0xc2644: and r6, r5, r6, lsl #7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,7,false)?)?;
    // VMA 0xc2648: mov r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4096)?)?;
    // VMA 0xc264c: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2650: ldrb r6, [r4, #0x56f]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,1391)?)?;
    // VMA 0xc2654: and r6, r5, r6, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,12,false)?)?;
    // VMA 0xc2658: mov r5, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,524288)?)?;
    // VMA 0xc265c: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2660: ldrb r6, [r4, #0x54f]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,1359)?)?;
    // VMA 0xc2664: and r6, r5, r6, lsl #19
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,19,false)?)?;
    // VMA 0xc2668: mov r5, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1048576)?)?;
    // VMA 0xc266c: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2670: ldrb r6, [r4, #0x550]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,1360)?)?;
    // VMA 0xc2674: and r6, r5, r6, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,20,false)?)?;
    // VMA 0xc2678: mov r5, #2097152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2097152)?)?;
    // VMA 0xc267c: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2680: ldrb r6, [r4, #0x551]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,1361)?)?;
    // VMA 0xc2684: and r6, r5, r6, lsl #21
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,21,false)?)?;
    // VMA 0xc2688: mov r5, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16777216)?)?;
    // VMA 0xc268c: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2690: ldrb r6, [r4, #0x549]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(4)?,1353)?)?;
    // VMA 0xc2694: and r6, r5, r6, lsl #24
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,24,false)?)?;
    // VMA 0xc2698: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc269c: bne 0xc26c4 <entry+0x3b54>  @ imm = #0x20
    c.b(C::Ne,0xc26c4,false)?;
    // VMA 0xc26a0: ldrb r3, [r4, #0x54a]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,1354)?)?;
    // VMA 0xc26a4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc26a8: ldr r3, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1296)?)?;
    // VMA 0xc26ac: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc26b0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // VMA 0xc26b4: beq 0xc26c4 <entry+0x3b54>  @ imm = #0x8
    c.b(C::Eq,0xc26c4,false)?;
    // VMA 0xc26b8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc26bc: cmpeq lr, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::LR,0)?)?;
    // VMA 0xc26c0: beq 0xc26c8 <entry+0x3b58>  @ imm = #0x0
    c.b(C::Eq,0xc26c8,false)?;
    // VMA 0xc26c4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc26c8: ldr r2, [r4, #0x5d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1492)?)?;
    // VMA 0xc26cc: ldr r3, [r4, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1496)?)?;
    // VMA 0xc26d0: orr r1, r7, r1, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(7)?,R::R1,Shift::Lsl,25,false)?)?;
    // VMA 0xc26d4: mov r6, #201326592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,201326592)?)?;
    // VMA 0xc26d8: mov r12, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xc26dc: mov r5, #65024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,65024)?)?;
    // VMA 0xc26e0: orr r5, r5, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65536)?)?;
    // VMA 0xc26e4: and r2, r2, #63
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,63)?)?;
    // VMA 0xc26e8: and r3, r6, r3, lsl #26
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,26,false)?)?;
    // VMA 0xc26ec: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc26f0: ldrb r2, [r4, #0x710]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1808)?)?;
    // VMA 0xc26f4: orr r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc26f8: mov r3, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4194304)?)?;
    // VMA 0xc26fc: and r2, r3, r2, lsl #22
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,22,false)?)?;
    // VMA 0xc2700: mov r3, #8388608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8388608)?)?;
    // VMA 0xc2704: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2708: ldrb r2, [r4, #0x711]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1809)?)?;
    // VMA 0xc270c: and r2, r3, r2, lsl #23
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,23,false)?)?;
    // VMA 0xc2710: mov r3, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,268435456)?)?;
    // VMA 0xc2714: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2718: ldrb r2, [r4, #0x700]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1792)?)?;
    // VMA 0xc271c: and r2, r3, r2, lsl #28
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,28,false)?)?;
    // VMA 0xc2720: mov r3, #448
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,448)?)?;
    // VMA 0xc2724: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2728: orr r2, r8, #3696
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,3696)?)?;
    // VMA 0xc272c: str r1, [r2, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xc2730: ldr r1, [r4, #0x5c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1476)?)?;
    // VMA 0xc2734: ldr r0, [r4, #0x6fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1788)?)?;
    // VMA 0xc2738: ldr r2, [r4, #0x5c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1480)?)?;
    // VMA 0xc273c: ldr r7, [r4, #0x5cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,1484)?)?;
    // VMA 0xc2740: ldr r6, [r4, #0x5d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,1488)?)?;
    // VMA 0xc2744: and r1, r5, r1, lsl #9
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(5)?,R::R1,Shift::Lsl,9,false)?)?;
    // VMA 0xc2748: ldr r5, [r4, #0x6f8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,1784)?)?;
    // VMA 0xc274c: and r3, r3, r0, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,6,false)?)?;
    // VMA 0xc2750: and r5, r5, #63
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,63)?)?;
    // VMA 0xc2754: orr r3, r3, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2758: orr r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc275c: mov r3, #4063232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4063232)?)?;
    // VMA 0xc2760: and r2, r3, r2, lsl #17
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,17,false)?)?;
    // VMA 0xc2764: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2768: mov r2, #130023424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,130023424)?)?;
    // VMA 0xc276c: and r2, r2, r7, lsl #22
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,22,false)?)?;
    // VMA 0xc2770: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc2774: orr r2, r8, #628
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,628)?)?;
    // VMA 0xc2778: orr r1, r1, r6, lsl #27
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,27,false)?)?;
    // VMA 0xc277c: orr r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3072)?)?;
    // VMA 0xc2780: str r1, [r2, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xc2784: orr r1, r8, #3712
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(8)?,3712)?)?;
    // VMA 0xc2788: ldr r2, [r4, #0x780]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1920)?)?;
    // VMA 0xc278c: str r2, [r1, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xc2790: ldr r1, [r4, #0x5d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1492)?)?;
    // VMA 0xc2794: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc2798: bne 0xc27a8 <entry+0x3c38>  @ imm = #0x8
    c.b(C::Ne,0xc27a8,false)?;
    // VMA 0xc279c: ldr r1, [r4, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1496)?)?;
    // VMA 0xc27a0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc27a4: beq 0xc2854 <entry+0x3ce4>  @ imm = #0xa8
    c.b(C::Eq,0xc2854,false)?;
    // VMA 0xc27a8: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // VMA 0xc27ac: bne 0xc2854 <entry+0x3ce4>  @ imm = #0xa0
    c.b(C::Ne,0xc2854,false)?;
    // VMA 0xc27b0: ldr r1, [r4, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1776)?)?;
    // VMA 0xc27b4: mov r2, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,254)?)?;
    // VMA 0xc27b8: ldr r0, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1772)?)?;
    // VMA 0xc27bc: orr r2, r2, #32512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,32512)?)?;
    // VMA 0xc27c0: and r1, r2, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xc27c4: ldr r2, [pc, #0xa4]         @ 0x3d00 <entry+0x3d00>
    c.literal(R::R2,0xc2870)?;
    // VMA 0xc27c8: and r0, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xc27cc: orr r2, r8, #3104
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,3104)?)?;
    // VMA 0xc27d0: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc27d4: orr r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,1)?)?;
    // VMA 0xc27d8: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xc27dc: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc27e0: ldr r1, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1772)?)?;
    // VMA 0xc27e4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc27e8: beq 0xc2818 <entry+0x3ca8>  @ imm = #0x28
    c.b(C::Eq,0xc2818,false)?;
    // VMA 0xc27ec: add r1, r4, #476
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,476)?)?;
    // VMA 0xc27f0: orr r2, r8, #36
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,36)?)?;
    // VMA 0xc27f4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xc27f8: add r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // VMA 0xc27fc: orr r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3072)?)?;
    // VMA 0xc2800: ldr r7, [r1, r3, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R1,R::new(3)?,Shift::Lsl,2)?)?;
    // VMA 0xc2804: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xc2808: str r7, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xc280c: ldr r7, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,1772)?)?;
    // VMA 0xc2810: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2814: blo 0xc2800 <entry+0x3c90>  @ imm = #-0x1c
    c.b(C::Cc,0xc2800,false)?;
    // VMA 0xc2818: ldr r0, [r4, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1776)?)?;
    // VMA 0xc281c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc2820: beq 0xc2854 <entry+0x3ce4>  @ imm = #0x2c
    c.b(C::Eq,0xc2854,false)?;
    // VMA 0xc2824: add r0, r4, #732
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,732)?)?;
    // VMA 0xc2828: orr r1, r8, #40
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(8)?,40)?)?;
    // VMA 0xc282c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc2830: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xc2834: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xc2838: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    // VMA 0xc283c: ldr r7, [r0, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::R2,Shift::Lsl,2)?)?;
    // VMA 0xc2840: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xc2844: str r7, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xc2848: ldr r7, [r4, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,1776)?)?;
    // VMA 0xc284c: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc2850: blo 0xc283c <entry+0x3ccc>  @ imm = #-0x1c
    c.b(C::Cc,0xc283c,false)?;
    // VMA 0xc2854: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc2858: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc285c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc2860)?;
    const CODEC_FRAME_TABLE_DIAGNOSTICS:[u32;5]=[
       0x000faf54,0x000fb18f,0x000e2788,0x000fb1c2,0x7fff8000,
    ];
    for word in CODEC_FRAME_TABLE_DIAGNOSTICS {c.emit(word)?;}
    c.check(0xc2874)
}
