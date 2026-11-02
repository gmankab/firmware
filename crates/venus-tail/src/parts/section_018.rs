
/// Bind the codec's initial surface state to an available runtime slot,
/// then publish initial input plane sizes and release any former state.
fn bind_surface_state_and_reset_slots(c:&mut Code)->Result<(),String>{
    c.check(0xadc14)?;
    // ARM 0xadc14: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xadc18: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM 0xadc1c: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xadc20: ldr r0, [r2, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,120)?)?;
    // ARM 0xadc24: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // ARM 0xadc28: ldr r6, [r5, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,2444)?)?;
    // ARM 0xadc2c: add r1, r5, #2816
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,2816)?)?;
    // ARM 0xadc30: str r0, [r5, #0xae4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2788)?)?;
    // ARM 0xadc34: str r0, [r5, #0xb28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2856)?)?;
    // ARM 0xadc38: add r0, r5, #2880
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,2880)?)?;
    // ARM 0xadc3c: bl 0xa8de4 <entry+0xffffffffffffc694>
    c.b(C::Al,0xa8de4,true)?;
    // ARM 0xadc40: mov r2, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,106)?)?;
    // ARM 0xadc44: str r0, [r5, #0xb4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2892)?)?;
    // ARM 0xadc48: str r0, [r5, #0xae0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,2784)?)?;
    // ARM 0xadc4c: mov r1, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,616)?)?;
    // ARM 0xadc50: orr r2, r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,512)?)?;
    // ARM 0xadc54: ldrh r1, [r6, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(6)?,R::R1)?)?;
    // ARM 0xadc58: ldrh r2, [r6, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R2,R::new(6)?,R::R2)?)?;
    // ARM 0xadc5c: mul r3, r2, r1
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R1)?)?;
    // ARM 0xadc60: str r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,128)?)?;
    // ARM 0xadc64: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xadc68: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM 0xadc6c: mov r2, #2592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2592)?)?;
    // ARM 0xadc70: strb r1, [r0, #0xa30]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2608)?)?;
    // ARM 0xadc74: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xadc78: strb r1, [r0, #0xa22]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2594)?)?;
    // ARM 0xadc7c: strh r1, [r0, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::R0,R::R2)?)?;
    // ARM 0xadc80: strb r1, [r0, #0xa24]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2596)?)?;
    // ARM 0xadc84: strb r1, [r0, #0xa27]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2599)?)?;
    // ARM 0xadc88: strb r1, [r0, #0xa28]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2600)?)?;
    // ARM 0xadc8c: ldrb r2, [r0, #0xb0c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2828)?)?;
    // ARM 0xadc90: strb r2, [r0, #0xa31]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2609)?)?;
    // ARM 0xadc94: ldr r2, [r0, #0xb10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2832)?)?;
    // ARM 0xadc98: str r2, [r0, #0xa34]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,2612)?)?;
    // ARM 0xadc9c: ldrb r2, [r0, #0xb14]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2836)?)?;
    // ARM 0xadca0: strb r2, [r0, #0xa38]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2616)?)?;
    // ARM 0xadca4: ldrb r2, [r0, #0xb17]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2839)?)?;
    // ARM 0xadca8: strb r2, [r0, #0xa7e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2686)?)?;
    // ARM 0xadcac: strb r1, [r0, #0xa7f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2687)?)?;
    // ARM 0xadcb0: strb r1, [r0, #0xa80]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2688)?)?;
    // ARM 0xadcb4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xadcb8: b 0xab964 <entry+0xfffffffffffff214>
    c.b(C::Al,0xab964,false)?;
    c.check(0xadcbc)
}

/// Initialize and validate per-channel output buffers. Source status checks
/// keep actual control-flow branches and original stack frame epilogue.
fn activate_codec_channel_buffers(c:&mut Code)->Result<(),String>{
    c.check(0xadcbc)?;
    // ARM 0xadcbc: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xadcc0: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xadcc4: sub sp, sp, #60
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,60)?)?;
    // ARM 0xadcc8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xadccc: sub r1, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,32)?)?;
    // ARM 0xadcd0: ldr r0, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,980)?)?;
    // ARM 0xadcd4: ldr r8, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,68)?)?;
    // ARM 0xadcd8: bl 0xd9a6c <entry+0x2d31c>
    c.b(C::Al,0xd9a6c,true)?;
    // ARM 0xadcdc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xadce0: beq 0xadf00 <entry+0x17b0>
    c.b(C::Eq,0xadf00,false)?;
    // ARM 0xadce4: add r0, r4, #904
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,904)?)?;
    // ARM 0xadce8: add r6, r4, #2880
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(4)?,2880)?)?;
    // ARM 0xadcec: sub r5, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,32)?)?;
    // ARM 0xadcf0: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // ARM 0xadcf4: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM 0xadcf8: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // ARM 0xadcfc: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // ARM 0xadd00: add r0, r4, #948
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,948)?)?;
    // ARM 0xadd04: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // ARM 0xadd08: add r0, r4, #932
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,932)?)?;
    // ARM 0xadd0c: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // ARM 0xadd10: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // ARM 0xadd14: ldr r9, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-32)?)?;
    // ARM 0xadd18: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // ARM 0xadd1c: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // ARM 0xadd20: beq 0xadedc <entry+0x178c>
    c.b(C::Eq,0xadedc,false)?;
    // ARM 0xadd24: ldr r0, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,20)?)?;
    // ARM 0xadd28: ror r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Ror,1)?)?;
    // ARM 0xadd2c: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // ARM 0xadd30: bhi 0xaded0 <entry+0x1780>
    c.b(C::Hi,0xaded0,false)?;
    // ARM 0xadd34: mov r1, #85
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,85)?)?;
    // ARM 0xadd38: tst r1, r7, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R1,R::new(7)?,Shift::Lsl,R::R0,false)?)?;
    // ARM 0xadd3c: beq 0xadd80 <entry+0x1630>
    c.b(C::Eq,0xadd80,false)?;
    // ARM 0xadd40: ldr r2, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,4)?)?;
    // ARM 0xadd44: ldr r1, [pc, #0x1c4]
    c.literal(R::R1,0xadf10)?;
    // ARM 0xadd48: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xadd4c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xadd50: ldr r1, [pc, #0x1bc]
    c.literal(R::R1,0xadf14)?;
    // ARM 0xadd54: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xadd58: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xadd5c: ldr r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // ARM 0xadd60: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xadd64: bl 0xa9194 <entry+0xffffffffffffca44>
    c.b(C::Al,0xa9194,true)?;
    // ARM 0xadd68: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xadd6c: beq 0xadda4 <entry+0x1654>
    c.b(C::Eq,0xadda4,false)?;
    // ARM 0xadd70: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xadd74: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xadd78: bl 0xab784 <entry+0xfffffffffffff034>
    c.b(C::Al,0xab784,true)?;
    // ARM 0xadd7c: b 0xaded0 <entry+0x1780>
    c.b(C::Al,0xaded0,false)?;
    // ARM 0xadd80: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xadd84: bne 0xaded0 <entry+0x1780>
    c.b(C::Ne,0xaded0,false)?;
    // ARM 0xadd88: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xadd8c: ldr r2, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-40)?)?;
    // ARM 0xadd90: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xadd94: bl 0xda7e0 <entry+0x2e090>
    c.b(C::Al,0xda7e0,true)?;
    // ARM 0xadd98: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xadd9c: bne 0xaded0 <entry+0x1780>
    c.b(C::Ne,0xaded0,false)?;
    // ARM 0xadda0: b 0xadf04 <entry+0x17b4>
    c.b(C::Al,0xadf04,false)?;
    // ARM 0xadda4: ldr r0, [r4, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2880)?)?;
    // ARM 0xadda8: ldr r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // ARM 0xaddac: mov r2, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // ARM 0xaddb0: mla r3, r1, r2, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R1,R::R2,R::R0)?)?;
    // ARM 0xaddb4: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xaddb8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xaddbc: add r2, r3, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,40)?)?;
    // ARM 0xaddc0: add r3, r3, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,60)?)?;
    // ARM 0xaddc4: bl 0xda614 <entry+0x2dec4>
    c.b(C::Al,0xda614,true)?;
    // ARM 0xaddc8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaddcc: beq 0xadf04 <entry+0x17b4>
    c.b(C::Eq,0xadf04,false)?;
    // ARM 0xaddd0: ldr r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // ARM 0xaddd4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xaddd8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xadddc: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // ARM 0xadde0: bl 0xa90dc <entry+0xffffffffffffc98c>
    c.b(C::Al,0xa90dc,true)?;
    // ARM 0xadde4: orr r7, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(8)?,4)?)?;
    // ARM 0xadde8: cmp r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,5)?)?;
    // ARM 0xaddec: beq 0xade04 <entry+0x16b4>
    c.b(C::Eq,0xade04,false)?;
    // ARM 0xaddf0: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xaddf4: ldr r1, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,8)?)?;
    // ARM 0xaddf8: mov r3, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // ARM 0xaddfc: mla r2, r1, r3, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R1,R::new(3)?,R::R0)?)?;
    // ARM 0xade00: strb r10, [r2, #0x50]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(10)?,R::R2,80)?)?;
    // ARM 0xade04: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xade08: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xade0c: bl 0xda778 <entry+0x2e028>
    c.b(C::Al,0xda778,true)?;
    // ARM 0xade10: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xade14: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xade18: bl 0xab784 <entry+0xfffffffffffff034>
    c.b(C::Al,0xab784,true)?;
    // ARM 0xade1c: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // ARM 0xade20: ldr r1, [r4, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2880)?)?;
    // ARM 0xade24: mov r3, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,84)?)?;
    // ARM 0xade28: cmp r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,5)?)?;
    // ARM 0xade2c: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // ARM 0xade30: mla r2, r0, r3, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R0,R::new(3)?,R::R1)?)?;
    // ARM 0xade34: addeq r1, r2, #48
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::R2,48)?)?;
    // ARM 0xade38: addne r1, r2, #68
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R2,68)?)?;
    // ARM 0xade3c: ldrb r2, [r4, #0x241]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,577)?)?;
    // ARM 0xade40: tst r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,64)?)?;
    // ARM 0xade44: beq 0xadeb8 <entry+0x1768>
    c.b(C::Eq,0xadeb8,false)?;
    // ARM 0xade48: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // ARM 0xade4c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xade50: ldrne r1, [r9, #0x448]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(9)?,1096)?)?;
    // ARM 0xade54: ldrne r1, [r1, #0x3c]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::R1,60)?)?;
    // ARM 0xade58: andsne r1, r1, #16
    c.emit(arm32::data_immediate_with_flags(C::Ne,Op::And,R::R1,R::R1,16,true)?)?;
    // ARM 0xade5c: beq 0xadeb8 <entry+0x1768>
    c.b(C::Eq,0xadeb8,false)?;
    // ARM 0xade60: ldr r0, [r9, #0x2f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,752)?)?;
    // ARM 0xade64: add lr, r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(9)?,32)?)?;
    // ARM 0xade68: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // ARM 0xade6c: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // ARM 0xade70: ldm lr, {r1, r3, r7, r12, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::LR,R::R1.bit() | R::new(3)?.bit() | R::new(7)?.bit() | R::R12.bit() | R::LR.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xade74: ldr r10, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(9)?,52)?)?;
    // ARM 0xade78: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // ARM 0xade7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xade80: stm sp, {r1, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit() | R::new(3)?.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xade84: str r12, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,12)?)?;
    // ARM 0xade88: str lr, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,16)?)?;
    // ARM 0xade8c: str r10, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,20)?)?;
    // ARM 0xade90: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // ARM 0xade94: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM 0xade98: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xade9c: mov r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8192)?)?;
    // ARM 0xadea0: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // ARM 0xadea4: ldr r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // ARM 0xadea8: ldr r2, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // ARM 0xadeac: bl 0xe192c <entry+0x351dc>
    c.b(C::Al,0xe192c,true)?;
    // ARM 0xadeb0: ldr r1, [r9, #0x448]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1096)?)?;
    // ARM 0xadeb4: b 0xadecc <entry+0x177c>
    c.b(C::Al,0xadecc,false)?;
    // ARM 0xadeb8: ldr r2, [r4, #0xb44]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2884)?)?;
    // ARM 0xadebc: mov r3, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,360)?)?;
    // ARM 0xadec0: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // ARM 0xadec4: mla r1, r0, r3, r2
    c.emit(arm32::multiply_accumulate(C::Al,R::R1,R::R0,R::new(3)?,R::R2)?)?;
    // ARM 0xadec8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xadecc: str r0, [r1, #0x754]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,1876)?)?;
    // ARM 0xaded0: str r8, [r9, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(9)?,92)?)?;
    // ARM 0xaded4: ldr r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // ARM 0xaded8: str r0, [r4, #0x300]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,768)?)?;
    // ARM 0xadedc: ldr r0, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,980)?)?;
    // ARM 0xadee0: bl 0xd9ac0 <entry+0x2d370>
    c.b(C::Al,0xd9ac0,true)?;
    // ARM 0xadee4: ldr r0, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,980)?)?;
    // ARM 0xadee8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xadeec: bl 0xd9a6c <entry+0x2d31c>
    c.b(C::Al,0xd9a6c,true)?;
    // ARM 0xadef0: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // ARM 0xadef4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xadef8: bne 0xadd14 <entry+0x15c4>
    c.b(C::Ne,0xadd14,false)?;
    // ARM 0xadefc: b 0xadf04 <entry+0x17b4>
    c.b(C::Al,0xadf04,false)?;
    // ARM 0xadf00: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM 0xadf04: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // ARM 0xadf08: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xadf0c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xadf10)?;
    const ACTIVATE_STATUS:[u32;2]=[0x000f_6c34,0x000e_2788];
    for value in ACTIVATE_STATUS {c.emit(value)?;}
    c.check(0xadf18)
}

/// Deactivate a selected codec channel and publish any errors through
/// the platform runtime's existing diagnostic callouts.
fn deactivate_codec_channel_buffers(c:&mut Code)->Result<(),String>{
    c.check(0xadf18)?;
    // ARM 0xadf18: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xadf1c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xadf20: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // ARM 0xadf24: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xadf28: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // ARM 0xadf2c: ldr r0, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,980)?)?;
    // ARM 0xadf30: bl 0xd9b08 <entry+0x2d3b8>
    c.b(C::Al,0xd9b08,true)?;
    // ARM 0xadf34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xadf38: beq 0xae008 <entry+0x18b8>
    c.b(C::Eq,0xae008,false)?;
    // ARM 0xadf3c: ldr r5, [pc, #0xd8]
    c.literal(R::new(5)?,0xae01c)?;
    // ARM 0xadf40: add r0, r4, #948
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,948)?)?;
    // ARM 0xadf44: add r6, r4, #2880
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(4)?,2880)?)?;
    // ARM 0xadf48: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // ARM 0xadf4c: add r7, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::SP,4)?)?;
    // ARM 0xadf50: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM 0xadf54: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xadf58: ldr r8, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,4)?)?;
    // ARM 0xadf5c: ldr r0, [r8, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,32)?)?;
    // ARM 0xadf60: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // ARM 0xadf64: beq 0xadfe4 <entry+0x1894>
    c.b(C::Eq,0xadfe4,false)?;
    // ARM 0xadf68: str r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,16)?)?;
    // ARM 0xadf6c: str r9, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,12)?)?;
    // ARM 0xadf70: str r9, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,8)?)?;
    // ARM 0xadf74: ldr r0, [r8, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,20)?)?;
    // ARM 0xadf78: ror r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Ror,2)?)?;
    // ARM 0xadf7c: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // ARM 0xadf80: bhi 0xadfe4 <entry+0x1894>
    c.b(C::Hi,0xadfe4,false)?;
    // ARM 0xadf84: ldr r2, [r8, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,4)?)?;
    // ARM 0xadf88: ldr r1, [pc, #0x88]
    c.literal(R::R1,0xae018)?;
    // ARM 0xadf8c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xadf90: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xadf94: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xadf98: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xadf9c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xadfa0: ldr r1, [r8, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,8)?)?;
    // ARM 0xadfa4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xadfa8: bl 0xa91a8 <entry+0xffffffffffffca58>
    c.b(C::Al,0xa91a8,true)?;
    // ARM 0xadfac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xadfb0: bne 0xadfd8 <entry+0x1888>
    c.b(C::Ne,0xadfd8,false)?;
    // ARM 0xadfb4: ldr r0, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // ARM 0xadfb8: add r1, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,8)?)?;
    // ARM 0xadfbc: bl 0xda718 <entry+0x2dfc8>
    c.b(C::Al,0xda718,true)?;
    // ARM 0xadfc0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xadfc4: beq 0xae00c <entry+0x18bc>
    c.b(C::Eq,0xae00c,false)?;
    // ARM 0xadfc8: ldr r1, [r8, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,8)?)?;
    // ARM 0xadfcc: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xadfd0: add r2, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,8)?)?;
    // ARM 0xadfd4: bl 0xa9120 <entry+0xffffffffffffc9d0>
    c.b(C::Al,0xa9120,true)?;
    // ARM 0xadfd8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xadfdc: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // ARM 0xadfe0: bl 0xab8ec <entry+0xfffffffffffff19c>
    c.b(C::Al,0xab8ec,true)?;
    // ARM 0xadfe4: ldr r0, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,980)?)?;
    // ARM 0xadfe8: bl 0xd9b5c <entry+0x2d40c>
    c.b(C::Al,0xd9b5c,true)?;
    // ARM 0xadfec: ldr r0, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,980)?)?;
    // ARM 0xadff0: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM 0xadff4: bl 0xd9b08 <entry+0x2d3b8>
    c.b(C::Al,0xd9b08,true)?;
    // ARM 0xadff8: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // ARM 0xadffc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae000: bne 0xadf58 <entry+0x1808>
    c.b(C::Ne,0xadf58,false)?;
    // ARM 0xae004: b 0xae00c <entry+0x18bc>
    c.b(C::Al,0xae00c,false)?;
    // ARM 0xae008: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // ARM 0xae00c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // ARM 0xae010: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xae014: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xae018)?;
    const DEACTIVATE_STATUS:[u32;2]=[0x000f_6c57,0x000e_2788];
    for value in DEACTIVATE_STATUS {c.emit(value)?;}
    c.check(0xae020)
}

/// Clear/retire the codec session's active command objects, preserving
/// error checks and the original controller callback paths.
fn reset_active_codec_output_buffers(c:&mut Code)->Result<(),String>{
    c.check(0xae020)?;
    // ARM 0xae020: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xae024: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xae028: sub sp, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // ARM 0xae02c: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xae030: mov r10, r1
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // ARM 0xae034: ldr r0, [r5, #0xc40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,3136)?)?;
    // ARM 0xae038: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xae03c: blt 0xae100 <entry+0x19b0>
    c.b(C::Lt,0xae100,false)?;
    // ARM 0xae040: add r9, r5, #728
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(5)?,728)?)?;
    // ARM 0xae044: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM 0xae048: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM 0xae04c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM 0xae050: ldr r0, [r5, #0xb40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2880)?)?;
    // ARM 0xae054: add r8, r0, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xae058: ldr r0, [r8, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,20)?)?;
    // ARM 0xae05c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae060: beq 0xae0d8 <entry+0x1988>
    c.b(C::Eq,0xae0d8,false)?;
    // ARM 0xae064: ldr r0, [r8, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,56)?)?;
    // ARM 0xae068: add r2, r8, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,40)?)?;
    // ARM 0xae06c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xae070: bne 0xae08c <entry+0x193c>
    c.b(C::Ne,0xae08c,false)?;
    // ARM 0xae074: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xae078: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xae07c: mov r3, r10
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // ARM 0xae080: bl 0xe0d28 <entry+0x345d8>
    c.b(C::Al,0xe0d28,true)?;
    // ARM 0xae084: str r7, [r8, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,56)?)?;
    // ARM 0xae088: b 0xae0a4 <entry+0x1954>
    c.b(C::Al,0xae0a4,false)?;
    // ARM 0xae08c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xae090: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xae094: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xae098: str r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // ARM 0xae09c: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // ARM 0xae0a0: bl 0xabbf8 <entry+0xfffffffffffff4a8>
    c.b(C::Al,0xabbf8,true)?;
    // ARM 0xae0a4: ldrb r0, [r8, #0x50]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,80)?)?;
    // ARM 0xae0a8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xae0ac: bne 0xae0d4 <entry+0x1984>
    c.b(C::Ne,0xae0d4,false)?;
    // ARM 0xae0b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xae0b4: str r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // ARM 0xae0b8: add r2, r8, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,60)?)?;
    // ARM 0xae0bc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xae0c0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xae0c4: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM 0xae0c8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xae0cc: bl 0xabbf8 <entry+0xfffffffffffff4a8>
    c.b(C::Al,0xabbf8,true)?;
    // ARM 0xae0d0: strb r7, [r8, #0x50]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(8)?,80)?)?;
    // ARM 0xae0d4: str r7, [r8, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,20)?)?;
    // ARM 0xae0d8: ldr r0, [r8, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,32)?)?;
    // ARM 0xae0dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xae0e0: beq 0xae0ec <entry+0x199c>
    c.b(C::Eq,0xae0ec,false)?;
    // ARM 0xae0e4: bl 0x7fed0 <entry+0xfffffffffffd3780>
    c.b(C::Al,0x7fed0,true)?;
    // ARM 0xae0e8: str r7, [r8, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,32)?)?;
    // ARM 0xae0ec: ldr r0, [r5, #0xc40]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,3136)?)?;
    // ARM 0xae0f0: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xae0f4: add r6, r6, #84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,84)?)?;
    // ARM 0xae0f8: cmp r4, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xae0fc: blt 0xae050 <entry+0x1900>
    c.b(C::Lt,0xae050,false)?;
    // ARM 0xae100: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xae104: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xae108)
}
