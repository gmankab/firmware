
/// Calculate bounded plane scaling factors from two packed codec plane
/// counts and report the chosen live buffer mode. The conditional callback
/// returns through a tail branch, followed by six original noninstruction
/// diagnostic-address words maintained as a named data pool.
fn calculate_frame_scaling_and_policy(c:&mut Code)->Result<(),String>{
    c.check(0xc4694)?;
    // VMA 0xc4694: push {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc4698: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xc469c: ldr r12, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,8)?)?;
    // VMA 0xc46a0: add lr, r12, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R12,7)?)?;
    // VMA 0xc46a4: asr lr, lr, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::LR,R::LR,Shift::Asr,3)?)?;
    // VMA 0xc46a8: mul r4, lr, r2
    c.emit(arm32::multiply_register(C::Al,R::new(4)?,R::LR,R::R2)?)?;
    // VMA 0xc46ac: asr r4, r4, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(4)?,R::R0,R::new(4)?,Shift::Asr,R::new(3)?,false)?)?;
    // VMA 0xc46b0: lsl r5, r4, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(4)?,Shift::Lsl,3)?)?;
    // VMA 0xc46b4: cmp r5, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xc46b8: mov r5, r12
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R12))?;
    // VMA 0xc46bc: lsllt r5, r4, #3
    c.emit(arm32::mov_shifted_register(C::Lt,R::new(5)?,R::new(4)?,Shift::Lsl,3)?)?;
    // VMA 0xc46c0: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc46c4: add r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1)?)?;
    // VMA 0xc46c8: mul r2, lr, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::LR,R::R0)?)?;
    // VMA 0xc46cc: asr r0, r2, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R2,Shift::Asr,R::new(3)?,false)?)?;
    // VMA 0xc46d0: lsl r2, r0, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsl,3)?)?;
    // VMA 0xc46d4: cmp r2, r12
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xc46d8: lsllt r12, r0, #3
    c.emit(arm32::mov_shifted_register(C::Lt,R::R12,R::R0,Shift::Lsl,3)?)?;
    // VMA 0xc46dc: str r12, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,0)?)?;
    // VMA 0xc46e0: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc46e4: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc46e8: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xc46ec: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xc46f0: lsr r0, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Lsr,4)?)?;
    // VMA 0xc46f4: lsr r1, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R2,Shift::Lsr,4)?)?;
    // VMA 0xc46f8: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // VMA 0xc46fc: cmp r2, #3680
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3680)?)?;
    // VMA 0xc4700: bls 0xc4710 <entry+0x710>   @ imm = #0x8
    c.b(C::Ls,0xc4710,false)?;
    // VMA 0xc4704: ldr r0, [r4, #0x444]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1092)?)?;
    // VMA 0xc4708: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc470c: beq 0xc4758 <entry+0x758>   @ imm = #0x44
    c.b(C::Eq,0xc4758,false)?;
    // VMA 0xc4710: ldr r0, [r4, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2308)?)?;
    // VMA 0xc4714: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc4718: beq 0xc4740 <entry+0x740>   @ imm = #0x20
    c.b(C::Eq,0xc4740,false)?;
    // VMA 0xc471c: ldr r1, [pc, #0x8c]         @ 0x7b0 <entry+0x7b0>
    c.literal(R::R1,0xc47b0)?;
    // VMA 0xc4720: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc4724: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95a98
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4728: ldr r1, [pc, #0x84]         @ 0x7b4 <entry+0x7b4>
    c.literal(R::R1,0xc47b4)?;
    // VMA 0xc472c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc4730: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95aa4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4734: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc4738: str r0, [r4, #0x904]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2308)?)?;
    // VMA 0xc473c: b 0xc4790 <entry+0x790>   @ imm = #0x4c
    c.b(C::Al,0xc4790,false)?;
    // VMA 0xc4740: ldr r1, [pc, #0x70]         @ 0x7b8 <entry+0x7b8>
    c.literal(R::R1,0xc47b8)?;
    // VMA 0xc4744: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4748: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95abc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc474c: ldr r1, [pc, #0x60]         @ 0x7b4 <entry+0x7b4>
    c.literal(R::R1,0xc47b4)?;
    // VMA 0xc4750: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4754: b 0xc478c <entry+0x78c>   @ imm = #0x30
    c.b(C::Al,0xc478c,false)?;
    // VMA 0xc4758: ldr r0, [r4, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2308)?)?;
    // VMA 0xc475c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc4760: bne 0xc476c <entry+0x76c>   @ imm = #0x4
    c.b(C::Ne,0xc476c,false)?;
    // VMA 0xc4764: ldr r1, [pc, #0x54]         @ 0x7c0 <entry+0x7c0>
    c.literal(R::R1,0xc47c0)?;
    // VMA 0xc4768: b 0xc477c <entry+0x77c>   @ imm = #0xc
    c.b(C::Al,0xc477c,false)?;
    // VMA 0xc476c: ldr r1, [pc, #0x48]         @ 0x7bc <entry+0x7bc>
    c.literal(R::R1,0xc47bc)?;
    // VMA 0xc4770: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4774: str r0, [r4, #0x904]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2308)?)?;
    // VMA 0xc4778: str r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,144)?)?;
    // VMA 0xc477c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc4780: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95af4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4784: ldr r1, [pc, #0x28]         @ 0x7b4 <entry+0x7b4>
    c.literal(R::R1,0xc47b4)?;
    // VMA 0xc4788: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc478c: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95b00
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4790: ldr r2, [r4, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2308)?)?;
    // VMA 0xc4794: ldr r1, [pc, #0x28]         @ 0x7c4 <entry+0x7c4>
    c.literal(R::R1,0xc47c4)?;
    // VMA 0xc4798: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc479c: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95b10
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc47a0: ldr r1, [pc, #0xc]          @ 0x7b4 <entry+0x7b4>
    c.literal(R::R1,0xc47b4)?;
    // VMA 0xc47a4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc47a8: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc47ac: b 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95b20
    c.b(C::Al,0x2ec94,false)?;

    c.check(0xc47b0)?;
    const FRAME_SCALING_LITERALS:[u32;6]=[
        0x000f_b34e,0x000e_2788,0x000f_b369,
        0x000f_b318,0x000f_b334,0x000f_b382,
    ];
    for word in FRAME_SCALING_LITERALS {c.emit(word)?;}
    c.check(0xc47c8)
}

/// Measure codec frame-plane bounding ranges over packed controller
/// metadata. Preserve the early no-buffer callback and large-plane
/// reduction loop, including the original return paths, then emit three
/// diagnostic string addresses as separately classified data.
fn measure_frame_plane_allocation_span(c:&mut Code)->Result<(),String>{
    c.check(0xc47c8)?;
    // VMA 0xc47c8: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc47cc: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xc47d0: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xc47d4: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xc47d8: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // VMA 0xc47dc: ldr r0, [r5, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,260)?)?;
    // VMA 0xc47e0: ldr r2, [r1, #0xa20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,2592)?)?;
    // VMA 0xc47e4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc47e8: bl 0x31ec <entry+0xfffffffffff3f1ec> @ imm = #-0xc1604
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xc47ec: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // VMA 0xc47f0: ldr r0, [r5, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,260)?)?;
    // VMA 0xc47f4: ldr r1, [r1, #0xa20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,2592)?)?;
    // VMA 0xc47f8: add r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,255)?)?;
    // VMA 0xc47fc: bic r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xc4800: bl 0x205c <entry+0xfffffffffff3e05c> @ imm = #-0xc27ac
    c.b(C::Al,0x205c,true)?;
    // VMA 0xc4804: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc4808: mov r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2048)?)?;
    // VMA 0xc480c: mov r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,255)?)?;
    // VMA 0xc4810: orr r7, r7, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,65280)?)?;
    // VMA 0xc4814: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // VMA 0xc4818: orr r2, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // VMA 0xc481c: ldr r1, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,112)?)?;
    // VMA 0xc4820: add r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc4824: and r1, r1, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc4828: orr r3, r1, #285212672
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R1,285212672)?)?;
    // VMA 0xc482c: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc4830: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4834: ldr r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,8)?)?;
    // VMA 0xc4838: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // VMA 0xc483c: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xc4840: orr r0, r0, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,15,false)?)?;
    // VMA 0xc4844: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // VMA 0xc4848: ldr r3, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc484c: tst r3, #268435465
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,268435465)?)?;
    // VMA 0xc4850: beq 0xc4848 <entry+0x848>   @ imm = #-0x10
    c.b(C::Eq,0xc4848,false)?;
    // VMA 0xc4854: orr r1, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,4)?)?;
    // VMA 0xc4858: orr r2, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R1,2048)?)?;
    // VMA 0xc485c: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xc4860: ldr r2, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4864: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc4868: blt 0xc4a64 <entry+0xa64>   @ imm = #0x1f4
    c.b(C::Lt,0xc4a64,false)?;
    // VMA 0xc486c: ldr r6, [r5, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,260)?)?;
    // VMA 0xc4870: add r0, r5, #708
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,708)?)?;
    // VMA 0xc4874: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc4878: bl 0xc4280 <entry+0x280>   @ imm = #-0x600
    c.b(C::Al,0xc4280,true)?;
    // VMA 0xc487c: add r0, r5, #732
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,732)?)?;
    // VMA 0xc4880: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc4884: bl 0xc43a8 <entry+0x3a8>   @ imm = #-0x4e4
    c.b(C::Al,0xc43a8,true)?;
    // VMA 0xc4888: ldmib r6, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(6)?,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xc488c: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xc4890: lsr r0, r0, #25
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,25)?)?;
    // VMA 0xc4894: orr r0, r0, r1, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,7,false)?)?;
    // VMA 0xc4898: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // VMA 0xc489c: str r0, [r4, #0x5c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1476)?)?;
    // VMA 0xc48a0: ldrd r0, r1, [r6, #8]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xc48a4: lsrs r1, r1, #1
    c.emit(arm32::data_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsr,1,true)?)?;
    // VMA 0xc48a8: rrx r0, r0
    c.emit(arm32::rotate_right_with_extend(C::Al,R::R0,R::R0,false)?)?;
    // VMA 0xc48ac: mov r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,63)?)?;
    // VMA 0xc48b0: lsl r0, r0, #27
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,27)?)?;
    // VMA 0xc48b4: asr r0, r0, #27
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,27)?)?;
    // VMA 0xc48b8: str r0, [r4, #0x5c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1480)?)?;
    // VMA 0xc48bc: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xc48c0: lsl r0, r0, #21
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,21)?)?;
    // VMA 0xc48c4: asr r0, r0, #27
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,27)?)?;
    // VMA 0xc48c8: str r0, [r4, #0x5cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1484)?)?;
    // VMA 0xc48cc: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xc48d0: lsl r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,16)?)?;
    // VMA 0xc48d4: asr r0, r0, #27
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,27)?)?;
    // VMA 0xc48d8: str r0, [r4, #0x5d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1488)?)?;
    // VMA 0xc48dc: ldr r0, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,44)?)?;
    // VMA 0xc48e0: and r1, r1, r0, lsr #9
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R0,Shift::Lsr,9,false)?)?;
    // VMA 0xc48e4: str r1, [r4, #0x5d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1492)?)?;
    // VMA 0xc48e8: cmp r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,31)?)?;
    // VMA 0xc48ec: ldr r0, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,44)?)?;
    // VMA 0xc48f0: and r0, r2, r0, lsr #7
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsr,7,false)?)?;
    // VMA 0xc48f4: str r0, [r4, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1496)?)?;
    // VMA 0xc48f8: ldr r2, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,44)?)?;
    // VMA 0xc48fc: and r2, r7, r2, lsr #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(7)?,R::R2,Shift::Lsr,15,false)?)?;
    // VMA 0xc4900: str r2, [r4, #0x780]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1920)?)?;
    // VMA 0xc4904: beq 0xc49a4 <entry+0x9a4>   @ imm = #0x98
    c.b(C::Eq,0xc49a4,false)?;
    // VMA 0xc4908: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc490c: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc4910: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc4914: beq 0xc4b14 <entry+0xb14>   @ imm = #0x1f8
    c.b(C::Eq,0xc4b14,false)?;
    // VMA 0xc4918: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc491c: mov r12, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,7)?)?;
    // VMA 0xc4920: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // VMA 0xc4924: ldr r7, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,112)?)?;
    // VMA 0xc4928: add r6, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,7)?)?;
    // VMA 0xc492c: add r7, r12, r6, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R12,R::new(6)?,Shift::Lsr,3,false)?)?;
    // VMA 0xc4930: lsr r5, r7, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(7)?,Shift::Lsr,3)?)?;
    // VMA 0xc4934: mul r7, r5, r0
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::new(5)?,R::R0)?)?;
    // VMA 0xc4938: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xc493c: asr r2, r7, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::new(7)?,Shift::Asr,R::R1,false)?)?;
    // VMA 0xc4940: lsr r7, r6, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(6)?,Shift::Lsr,3)?)?;
    // VMA 0xc4944: lsl r3, r2, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsl,3)?)?;
    // VMA 0xc4948: cmp r3, r6, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsr,3,false)?)?;
    // VMA 0xc494c: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // VMA 0xc4950: lsllt r3, r2, #3
    c.emit(arm32::mov_shifted_register(C::Lt,R::new(3)?,R::R2,Shift::Lsl,3)?)?;
    // VMA 0xc4954: mul r2, r5, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::new(5)?,R::R0)?)?;
    // VMA 0xc4958: asr r2, r2, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R2,Shift::Asr,R::R1,false)?)?;
    // VMA 0xc495c: lsl r5, r2, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R2,Shift::Lsl,3)?)?;
    // VMA 0xc4960: cmp r5, r6, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(6)?,Shift::Lsr,3,false)?)?;
    // VMA 0xc4964: lsllt r7, r2, #3
    c.emit(arm32::mov_shifted_register(C::Lt,R::new(7)?,R::R2,Shift::Lsl,3)?)?;
    // VMA 0xc4968: cmp r7, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc496c: ble 0xc4990 <entry+0x990>   @ imm = #0x1c
    c.b(C::Le,0xc4990,false)?;
    // VMA 0xc4970: ldr r1, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1772)?)?;
    // VMA 0xc4974: lsl r2, r3, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(3)?,Shift::Lsl,3)?)?;
    // VMA 0xc4978: add r1, r4, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(4)?,R::R1,Shift::Lsl,2,false)?)?;
    // VMA 0xc497c: str r2, [r1, #0x5dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,1500)?)?;
    // VMA 0xc4980: ldr r1, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1772)?)?;
    // VMA 0xc4984: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xc4988: str r1, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1772)?)?;
    // VMA 0xc498c: ldr r1, [r4, #0x5d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1492)?)?;
    // VMA 0xc4990: cmp r0, lr, lsl r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::R0,R::LR,Shift::Lsl,R::R1,false)?)?;
    // VMA 0xc4994: blt 0xc4924 <entry+0x924>   @ imm = #-0x78
    c.b(C::Lt,0xc4924,false)?;
    // VMA 0xc4998: ldr r0, [r4, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1496)?)?;
    // VMA 0xc499c: cmp r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,31)?)?;
    // VMA 0xc49a0: beq 0xc4a24 <entry+0xa24>   @ imm = #0x7c
    c.b(C::Eq,0xc4a24,false)?;
    // VMA 0xc49a4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc49a8: mov r12, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,7)?)?;
    // VMA 0xc49ac: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // VMA 0xc49b0: ldr r7, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,108)?)?;
    // VMA 0xc49b4: add r6, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,7)?)?;
    // VMA 0xc49b8: add r7, r12, r6, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R12,R::new(6)?,Shift::Lsr,3,false)?)?;
    // VMA 0xc49bc: lsr r5, r7, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(7)?,Shift::Lsr,3)?)?;
    // VMA 0xc49c0: mul r7, r5, r1
    c.emit(arm32::multiply_register(C::Al,R::new(7)?,R::new(5)?,R::R1)?)?;
    // VMA 0xc49c4: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xc49c8: asr r2, r7, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::new(7)?,Shift::Asr,R::R0,false)?)?;
    // VMA 0xc49cc: lsr r7, r6, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(6)?,Shift::Lsr,3)?)?;
    // VMA 0xc49d0: lsl r3, r2, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsl,3)?)?;
    // VMA 0xc49d4: cmp r3, r6, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsr,3,false)?)?;
    // VMA 0xc49d8: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // VMA 0xc49dc: lsllt r3, r2, #3
    c.emit(arm32::mov_shifted_register(C::Lt,R::new(3)?,R::R2,Shift::Lsl,3)?)?;
    // VMA 0xc49e0: mul r2, r5, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::new(5)?,R::R1)?)?;
    // VMA 0xc49e4: asr r2, r2, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R2,Shift::Asr,R::R0,false)?)?;
    // VMA 0xc49e8: lsl r5, r2, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R2,Shift::Lsl,3)?)?;
    // VMA 0xc49ec: cmp r5, r6, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(6)?,Shift::Lsr,3,false)?)?;
    // VMA 0xc49f0: lsllt r7, r2, #3
    c.emit(arm32::mov_shifted_register(C::Lt,R::new(7)?,R::R2,Shift::Lsl,3)?)?;
    // VMA 0xc49f4: cmp r7, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc49f8: ble 0xc4a1c <entry+0xa1c>   @ imm = #0x1c
    c.b(C::Le,0xc4a1c,false)?;
    // VMA 0xc49fc: ldr r0, [r4, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1776)?)?;
    // VMA 0xc4a00: lsl r2, r3, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(3)?,Shift::Lsl,3)?)?;
    // VMA 0xc4a04: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc4a08: str r2, [r0, #0x6dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1756)?)?;
    // VMA 0xc4a0c: ldr r0, [r4, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1776)?)?;
    // VMA 0xc4a10: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xc4a14: str r0, [r4, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1776)?)?;
    // VMA 0xc4a18: ldr r0, [r4, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1496)?)?;
    // VMA 0xc4a1c: cmp r1, lr, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::R1,R::LR,Shift::Lsl,R::R0,false)?)?;
    // VMA 0xc4a20: blt 0xc49b0 <entry+0x9b0>   @ imm = #-0x78
    c.b(C::Lt,0xc49b0,false)?;
    // VMA 0xc4a24: ldr lr, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(4)?,112)?)?;
    // VMA 0xc4a28: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc4a2c: add r0, lr, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::LR,7)?)?;
    // VMA 0xc4a30: bic r0, r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,7)?)?;
    // VMA 0xc4a34: str r0, [r4, #0x6f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1780)?)?;
    // VMA 0xc4a38: ldr r8, [r4, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,1772)?)?;
    // VMA 0xc4a3c: cmp r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // VMA 0xc4a40: blo 0xc4afc <entry+0xafc>   @ imm = #0xb4
    c.b(C::Cc,0xc4afc,false)?;
    // VMA 0xc4a44: ldr r0, [r4, #0x5dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1500)?)?;
    // VMA 0xc4a48: sub r2, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(8)?,1)?)?;
    // VMA 0xc4a4c: cmp r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // VMA 0xc4a50: and r12, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R12,R::R2,1)?)?;
    // VMA 0xc4a54: bne 0xc4a98 <entry+0xa98>   @ imm = #0x3c
    c.b(C::Ne,0xc4a98,false)?;
    // VMA 0xc4a58: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc4a5c: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // VMA 0xc4a60: b 0xc4ae0 <entry+0xae0>   @ imm = #0x78
    c.b(C::Al,0xc4ae0,false)?;
    // VMA 0xc4a64: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xc4a68: tst r0, #268435457
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,268435457)?)?;
    // VMA 0xc4a6c: beq 0xc486c <entry+0x86c>   @ imm = #-0x208
    c.b(C::Eq,0xc486c,false)?;
    // VMA 0xc4a70: ldr r1, [pc, #0xa4]         @ 0xb1c <entry+0xb1c>
    c.literal(R::R1,0xc4b1c)?;
    // VMA 0xc4a74: ldr r2, [pc, #0xa4]         @ 0xb20 <entry+0xb20>
    c.literal(R::R2,0xc4b20)?;
    // VMA 0xc4a78: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc4a7c: mov r3, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,716)?)?;
    // VMA 0xc4a80: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95df4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4a84: ldr r1, [pc, #0x98]         @ 0xb24 <entry+0xb24>
    c.literal(R::R1,0xc4b24)?;
    // VMA 0xc4a88: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc4a8c: bl 0x2ec94 <entry+0xfffffffffff6ac94> @ imm = #-0x95e00
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc4a90: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc4a94: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc4a98: sub r5, r2, r12
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(5)?,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // VMA 0xc4a9c: add r3, r4, #1504
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,1504)?)?;
    // VMA 0xc4aa0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xc4aa4: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // VMA 0xc4aa8: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xc4aac: ldr r1, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(3)?,0)?)?;
    // VMA 0xc4ab0: ldr r0, [r3, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,4)?)?;
    // VMA 0xc4ab4: add r3, r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,8)?)?;
    // VMA 0xc4ab8: add r6, r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,2)?)?;
    // VMA 0xc4abc: sub r7, r1, r7
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc4ac0: sub r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc4ac4: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc4ac8: movlo r2, r7
    c.emit(arm32::mov_register(C::Cc,R::R2,R::new(7)?))?;
    // VMA 0xc4acc: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xc4ad0: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc4ad4: movlo r2, r1
    c.emit(arm32::mov_register(C::Cc,R::R2,R::R1))?;
    // VMA 0xc4ad8: subs r5, r5, #2
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::new(5)?,R::new(5)?,2,true)?)?;
    // VMA 0xc4adc: bne 0xc4aac <entry+0xaac>   @ imm = #-0x38
    c.b(C::Ne,0xc4aac,false)?;
    // VMA 0xc4ae0: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // VMA 0xc4ae4: beq 0xc4afc <entry+0xafc>   @ imm = #0x10
    c.b(C::Eq,0xc4afc,false)?;
    // VMA 0xc4ae8: add r3, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(4)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // VMA 0xc4aec: ldr r3, [r3, #0x5dc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,1500)?)?;
    // VMA 0xc4af0: sub r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc4af4: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc4af8: movlo r2, r0
    c.emit(arm32::mov_register(C::Cc,R::R2,R::R0))?;
    // VMA 0xc4afc: add r0, r4, r8, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(8)?,Shift::Lsl,2,false)?)?;
    // VMA 0xc4b00: ldr r0, [r0, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,1496)?)?;
    // VMA 0xc4b04: sub r0, lr, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::LR,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc4b08: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc4b0c: movlo r2, r0
    c.emit(arm32::mov_register(C::Cc,R::R2,R::R0))?;
    // VMA 0xc4b10: str r2, [r4, #0x6f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,1780)?)?;
    // VMA 0xc4b14: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc4b18: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;

    c.check(0xc4b1c)?;
    const FRAME_PLANE_BOUNDS_DIAGNOSTICS:[u32;3]=[
        0x000f_b38f,0x000f_b3bf,0x000e_2788,
    ];
    for value in FRAME_PLANE_BOUNDS_DIAGNOSTICS {c.emit(value)?;}
    c.check(0xc4b28)
}
