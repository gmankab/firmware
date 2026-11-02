
/// Prepare codec-format output buffers and frame dimensions.
/// Part of the ongoing codec routine; no invented function return.
fn prepare_codec_mode_output_buffers(c:&mut Code)->Result<(),String>{
    c.check(0xbe000)?;
    // VMA 0xbe000: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe004: bne 0xbe634 <entry+0x1e58>  @ imm = #0x628
    c.b(C::Ne,0xbe634,false)?;
    // VMA 0xbe008: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // VMA 0xbe00c: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe010: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xbe014: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xbe018: bhi 0xbeab4 <entry+0x22d8>  @ imm = #0xa94
    c.b(C::Hi,0xbeab4,false)?;
    // VMA 0xbe01c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe020: beq 0xbeab4 <entry+0x22d8>  @ imm = #0xa8c
    c.b(C::Eq,0xbeab4,false)?;
    // VMA 0xbe024: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbe028: ldr r2, [pc, #0xc04]        @ 0x2458 <entry+0x2458>
    c.literal(R::R2,0xbec34)?;
    // VMA 0xbe02c: mov r3, #35
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,35)?)?;
    // VMA 0xbe030: str r1, [r9, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,188)?)?;
    // VMA 0xbe034: ldr r1, [pc, #0xc04]        @ 0x2464 <entry+0x2464>
    c.literal(R::R1,0xbec40)?;
    // VMA 0xbe038: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xbe03c: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xbe040: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe044: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f3b8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe048: ldr r1, [pc, #0xbe8]        @ 0x245c <entry+0x245c>
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbe04c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe050: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f3c4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe054: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // VMA 0xbe058: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe05c: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // VMA 0xbe060: bl 0xe1e60 <entry+0x25684> @ imm = #0x23df8
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe064: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe068: bne 0xbe638 <entry+0x1e5c>  @ imm = #0x5c8
    c.b(C::Ne,0xbe638,false)?;
    // VMA 0xbe06c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbe070: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe074: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe078: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xbe07c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe080: bl 0xe1e60 <entry+0x25684> @ imm = #0x23dd8
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe084: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe088: bne 0xbe638 <entry+0x1e5c>  @ imm = #0x5a8
    c.b(C::Ne,0xbe638,false)?;
    // VMA 0xbe08c: ldr r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    // VMA 0xbe090: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe094: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // VMA 0xbe098: bl 0xe1e60 <entry+0x25684> @ imm = #0x23dc0
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe09c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbe0a0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe0a4: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe0a8: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xbe0ac: b 0xbe638 <entry+0x1e5c>  @ imm = #0x584
    c.b(C::Al,0xbe638,false)?;
    // VMA 0xbe0b0: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbe0b4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xbe0b8: beq 0xbe0d4 <entry+0x18f8>  @ imm = #0x14
    c.b(C::Eq,0xbe0d4,false)?;
    // VMA 0xbe0bc: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // VMA 0xbe0c0: beq 0xbe288 <entry+0x1aac>  @ imm = #0x1c0
    c.b(C::Eq,0xbe288,false)?;
    // VMA 0xbe0c4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe0c8: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // VMA 0xbe0cc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbe0d0: bne 0xbde14 <entry+0x1638>  @ imm = #-0x2c4
    c.b(C::Ne,0xbde14,false)?;
    // VMA 0xbe0d4: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe0d8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe0dc: add r1, r9, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,24)?)?;
    // VMA 0xbe0e0: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe0e4: str r4, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-32)?)?;
    // VMA 0xbe0e8: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe0ec: bl 0xbec78 <entry+0x249c>  @ imm = #0xb84
    c.b(C::Al,0xbec78,true)?;
    // VMA 0xbe0f0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbe0f4: bne 0xbd744 <entry+0xf68>   @ imm = #-0x9b8
    c.b(C::Ne,0xbd744,false)?;
    // VMA 0xbe0f8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe0fc: add r1, r9, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,88)?)?;
    // VMA 0xbe100: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe104: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbe108: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbe10c: strb r0, [r9, #0x98]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,152)?)?;
    // VMA 0xbe110: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbe114: strb r0, [r9, #0x18]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,24)?)?;
    // VMA 0xbe118: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe11c: bl 0xbec78 <entry+0x249c>  @ imm = #0xb54
    c.b(C::Al,0xbec78,true)?;
    // VMA 0xbe120: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbe124: bne 0xbd744 <entry+0xf68>   @ imm = #-0x9e8
    c.b(C::Ne,0xbd744,false)?;
    // VMA 0xbe128: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe12c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbe130: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbe134: strb r0, [r9, #0x99]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(9)?,153)?)?;
    // VMA 0xbe138: b 0xbde08 <entry+0x162c>  @ imm = #-0x338
    c.b(C::Al,0xbde08,false)?;
    // VMA 0xbe13c: ldr r1, [pc, #0xb20]        @ 0x2488 <entry+0x2488>
    c.literal(R::R1,0xbec64)?;
    // VMA 0xbe140: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbe144: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8f4b8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbe148: ldr r1, [pc, #0xae8]        @ 0x245c <entry+0x245c>
    c.literal(R::R1,0xbec38)?;
    // VMA 0xbe14c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbe150: b 0xbde04 <entry+0x1628>  @ imm = #-0x354
    c.b(C::Al,0xbde04,false)?;
    // VMA 0xbe154: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbe158: str r0, [r9, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,216)?)?;
    // VMA 0xbe15c: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbe160: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xbe164: beq 0xbe180 <entry+0x19a4>  @ imm = #0x14
    c.b(C::Eq,0xbe180,false)?;
    // VMA 0xbe168: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // VMA 0xbe16c: beq 0xbe288 <entry+0x1aac>  @ imm = #0x114
    c.b(C::Eq,0xbe288,false)?;
    // VMA 0xbe170: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe174: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // VMA 0xbe178: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbe17c: bne 0xbde14 <entry+0x1638>  @ imm = #-0x370
    c.b(C::Ne,0xbde14,false)?;
    // VMA 0xbe180: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe184: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe188: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbe18c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe190: bl 0xe1e60 <entry+0x25684> @ imm = #0x23cc8
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe194: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe198: bne 0xbe498 <entry+0x1cbc>  @ imm = #0x2f8
    c.b(C::Ne,0xbe498,false)?;
    // VMA 0xbe19c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe1a0: str r0, [r9, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,220)?)?;
    // VMA 0xbe1a4: b 0xbde08 <entry+0x162c>  @ imm = #-0x3a4
    c.b(C::Al,0xbde08,false)?;
    // VMA 0xbe1a8: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbe1ac: sub r1, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,3)?)?;
    // VMA 0xbe1b0: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xbe1b4: bhs 0xbe278 <entry+0x1a9c>  @ imm = #0xbc
    c.b(C::Cs,0xbe278,false)?;
    // VMA 0xbe1b8: ldrb r1, [r5, #0x11]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,17)?)?;
    // VMA 0xbe1bc: ldrb r2, [r5, #0x17]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,23)?)?;
    // VMA 0xbe1c0: ldrb r0, [r5, #0x10]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // VMA 0xbe1c4: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // VMA 0xbe1c8: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xbe1cc: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe1d0: add r8, r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(5)?,32)?)?;
    // VMA 0xbe1d4: add r7, r5, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(5)?,28)?)?;
    // VMA 0xbe1d8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe1dc: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xbe1e0: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbe1e4: moveq r1, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xbe1e8: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // VMA 0xbe1ec: movne r1, r3
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(3)?))?;
    // VMA 0xbe1f0: moveq r6, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // VMA 0xbe1f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbe1f8: ldr r0, [r9, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,168)?)?;
    // VMA 0xbe1fc: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    c.check(0xbe200)
}

/// Dispatch controller input-plane policy and handle validation failures.
/// Part of the ongoing codec routine; no invented function return.
fn select_codec_plane_policy(c:&mut Code)->Result<(),String>{
    c.check(0xbe200)?;
    // VMA 0xbe200: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe204: moveq r6, r1
    c.emit(arm32::mov_register(C::Eq,R::new(6)?,R::R1))?;
    // VMA 0xbe208: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe20c: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // VMA 0xbe210: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xbe214: bl 0xe1e60 <entry+0x25684> @ imm = #0x23c44
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe218: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe21c: bne 0xbe498 <entry+0x1cbc>  @ imm = #0x274
    c.b(C::Ne,0xbe498,false)?;
    // VMA 0xbe220: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe224: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe228: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe22c: bl 0xe1e60 <entry+0x25684> @ imm = #0x23c2c
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe230: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe234: bne 0xbe498 <entry+0x1cbc>  @ imm = #0x25c
    c.b(C::Ne,0xbe498,false)?;
    // VMA 0xbe238: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe23c: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // VMA 0xbe240: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // VMA 0xbe244: bl 0xe1e60 <entry+0x25684> @ imm = #0x23c14
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe248: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe24c: bne 0xbe498 <entry+0x1cbc>  @ imm = #0x244
    c.b(C::Ne,0xbe498,false)?;
    // VMA 0xbe250: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe254: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe258: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe25c: bl 0xe1e60 <entry+0x25684> @ imm = #0x23bfc
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe260: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe264: bne 0xbe498 <entry+0x1cbc>  @ imm = #0x22c
    c.b(C::Ne,0xbe498,false)?;
    // VMA 0xbe268: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xbe26c: cmp r4, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbe270: blo 0xbe208 <entry+0x1a2c>  @ imm = #-0x70
    c.b(C::Cc,0xbe208,false)?;
    // VMA 0xbe274: b 0xbde08 <entry+0x162c>  @ imm = #-0x474
    c.b(C::Al,0xbde08,false)?;
    // VMA 0xbe278: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe27c: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // VMA 0xbe280: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbe284: bne 0xbde14 <entry+0x1638>  @ imm = #-0x478
    c.b(C::Ne,0xbde14,false)?;
    // VMA 0xbe288: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xbe28c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbe290: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe294: str r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbe298: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe29c: ldr r0, [r5, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,464)?)?;
    // VMA 0xbe2a0: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xbe2a4: lsl r0, r0, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,3)?)?;
    // VMA 0xbe2a8: str r0, [r5, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,996)?)?;
    // VMA 0xbe2ac: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbe2b0: bne 0xbd758 <entry+0xf7c>   @ imm = #-0xb60
    c.b(C::Ne,0xbd758,false)?;
    // VMA 0xbe2b4: b 0xbd744 <entry+0xf68>   @ imm = #-0xb78
    c.b(C::Al,0xbd744,false)?;
    // VMA 0xbe2b8: ldr r0, [r9, #0xf0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,240)?)?;
    // VMA 0xbe2bc: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // VMA 0xbe2c0: beq 0xbe288 <entry+0x1aac>  @ imm = #-0x40
    c.b(C::Eq,0xbe288,false)?;
    // VMA 0xbe2c4: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xbe2c8: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xbe2cc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xbe2d0: bne 0xbde14 <entry+0x1638>  @ imm = #-0x4c4
    c.b(C::Ne,0xbde14,false)?;
    // VMA 0xbe2d4: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe2d8: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe2dc: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbe2e0: mov r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // VMA 0xbe2e4: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe2e8: bl 0xe1e60 <entry+0x25684> @ imm = #0x23b70
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe2ec: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe2f0: bne 0xbec28 <entry+0x244c>  @ imm = #0x930
    c.b(C::Ne,0xbec28,false)?;
    // VMA 0xbe2f4: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe2f8: cmp r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,14)?)?;
    // VMA 0xbe2fc: bhi 0xbe9d0 <entry+0x21f4>  @ imm = #0x6cc
    c.b(C::Hi,0xbe9d0,false)?;
    // VMA 0xbe300: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbe304: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe308: orr r2, r2, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,31744)?)?;
    // VMA 0xbe30c: tst r2, r1, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::R0,false)?)?;
    // VMA 0xbe310: beq 0xbe9d0 <entry+0x21f4>  @ imm = #0x6b8
    c.b(C::Eq,0xbe9d0,false)?;
    // VMA 0xbe314: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // VMA 0xbe318: strb r1, [r5, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,11)?)?;
    // VMA 0xbe31c: str r0, [r5, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,568)?)?;
    // VMA 0xbe320: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe324: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe328: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbe32c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe330: bl 0xe1e60 <entry+0x25684> @ imm = #0x23b28
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe334: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe338: bne 0xbec28 <entry+0x244c>  @ imm = #0x8e8
    c.b(C::Ne,0xbec28,false)?;
    // VMA 0xbe33c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe340: cmp r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,14)?)?;
    // VMA 0xbe344: bhi 0xbe9e0 <entry+0x2204>  @ imm = #0x694
    c.b(C::Hi,0xbe9e0,false)?;
    // VMA 0xbe348: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbe34c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe350: orr r2, r2, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,31744)?)?;
    // VMA 0xbe354: tst r2, r1, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::R0,false)?)?;
    // VMA 0xbe358: beq 0xbe9e0 <entry+0x2204>  @ imm = #0x680
    c.b(C::Eq,0xbe9e0,false)?;
    // VMA 0xbe35c: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // VMA 0xbe360: strb r1, [r5, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,12)?)?;
    // VMA 0xbe364: str r0, [r5, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,568)?)?;
    // VMA 0xbe368: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe36c: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe370: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbe374: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe378: bl 0xe1e60 <entry+0x25684> @ imm = #0x23ae0
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe37c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe380: bne 0xbec28 <entry+0x244c>  @ imm = #0x8a0
    c.b(C::Ne,0xbec28,false)?;
    // VMA 0xbe384: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe388: cmp r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,14)?)?;
    // VMA 0xbe38c: bhi 0xbea98 <entry+0x22bc>  @ imm = #0x704
    c.b(C::Hi,0xbea98,false)?;
    // VMA 0xbe390: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbe394: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe398: orr r2, r2, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,31744)?)?;
    // VMA 0xbe39c: tst r2, r1, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::R0,false)?)?;
    // VMA 0xbe3a0: beq 0xbea98 <entry+0x22bc>  @ imm = #0x6f0
    c.b(C::Eq,0xbea98,false)?;
    // VMA 0xbe3a4: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // VMA 0xbe3a8: strb r1, [r5, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,13)?)?;
    // VMA 0xbe3ac: str r0, [r5, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,568)?)?;
    // VMA 0xbe3b0: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe3b4: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe3b8: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbe3bc: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe3c0: bl 0xe1e60 <entry+0x25684> @ imm = #0x23a98
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe3c4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe3c8: bne 0xbec28 <entry+0x244c>  @ imm = #0x858
    c.b(C::Ne,0xbec28,false)?;
    // VMA 0xbe3cc: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe3d0: cmp r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,14)?)?;
    // VMA 0xbe3d4: bhi 0xbeaac <entry+0x22d0>  @ imm = #0x6d0
    c.b(C::Hi,0xbeaac,false)?;
    // VMA 0xbe3d8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbe3dc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbe3e0: orr r2, r2, #31744
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,31744)?)?;
    // VMA 0xbe3e4: tst r2, r1, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::R0,false)?)?;
    // VMA 0xbe3e8: beq 0xbeaac <entry+0x22d0>  @ imm = #0x6bc
    c.b(C::Eq,0xbeaac,false)?;
    // VMA 0xbe3ec: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // VMA 0xbe3f0: strb r1, [r5, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,14)?)?;
    // VMA 0xbe3f4: str r0, [r5, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,568)?)?;
    // VMA 0xbe3f8: ldr r10, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,28)?)?;
    // VMA 0xbe3fc: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe400: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbe404: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe408: bl 0xe1e60 <entry+0x25684> @ imm = #0x23a50
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe40c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe410: bne 0xbec28 <entry+0x244c>  @ imm = #0x810
    c.b(C::Ne,0xbec28,false)?;
    // VMA 0xbe414: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe418: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // VMA 0xbe41c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xbe420: strb r0, [r5, #0xf]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,15)?)?;
    // VMA 0xbe424: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xbe428: bl 0xe1e60 <entry+0x25684> @ imm = #0x23a30
    c.b(C::Al,0xe1e60,true)?;
    // VMA 0xbe42c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbe430: bne 0xbec28 <entry+0x244c>  @ imm = #0x7f0
    c.b(C::Ne,0xbec28,false)?;
    // VMA 0xbe434: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbe438: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xbe43c: bhi 0xbeae8 <entry+0x230c>  @ imm = #0x6a4
    c.b(C::Hi,0xbeae8,false)?;
    // VMA 0xbe440: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xbe444: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // VMA 0xbe448: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0xbe44c)
}
