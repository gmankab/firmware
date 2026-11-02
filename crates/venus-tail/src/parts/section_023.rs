
/// Build the codec command state block, enqueue paired descriptor messages, and publish buffer accounting through the platform callback
/// Runtime slice 0xaf6e0..0xb02b0, with 10 classified trailing data words.
fn assemble_codec_command_and_report(c:&mut Code)->Result<(),String>{
    c.check(0xaf6e0)?;
    // ARM 0xaf6e0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xaf6e4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xaf6e8: sub sp, sp, #76
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,76)?)?;
    // ARM 0xaf6ec: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xaf6f0: str r1, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,52)?)?;
    // ARM 0xaf6f4: mov r1, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,124)?)?;
    // ARM 0xaf6f8: mov r6, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // ARM 0xaf6fc: mov r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,8)?)?;
    // ARM 0xaf700: mov r12, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xaf704: add r0, r4, #508
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,508)?)?;
    // ARM 0xaf708: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xaf70c: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2048)?)?;
    // ARM 0xaf710: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // ARM 0xaf714: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,36)?)?;
    // ARM 0xaf718: str r4, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-36)?)?;
    // ARM 0xaf71c: orr r1, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // ARM 0xaf720: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // ARM 0xaf724: mov r1, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,116)?)?;
    // ARM 0xaf728: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xaf72c: orr r1, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // ARM 0xaf730: str r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,48)?)?;
    // ARM 0xaf734: mov r1, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,120)?)?;
    // ARM 0xaf738: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xaf73c: orr lr, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // ARM 0xaf740: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // ARM 0xaf744: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xaf748: str lr, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R11,-48)?)?;
    // ARM 0xaf74c: orr r8, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // ARM 0xaf750: mov r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2048)?)?;
    // ARM 0xaf754: orr r9, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(9)?,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // ARM 0xaf758: add r1, r4, #948
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,948)?)?;
    // ARM 0xaf75c: lsl r0, r0, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,15)?)?;
    // ARM 0xaf760: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // ARM 0xaf764: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // ARM 0xaf768: b 0xb01b0 <entry+0x3a60>
    c.b(C::Al,0xb01b0,false)?;
    // ARM 0xaf76c: ldr r2, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2432)?)?;
    // ARM 0xaf770: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaf774: bne 0xb0280 <entry+0x3b30>
    c.b(C::Ne,0xb0280,false)?;
    // ARM 0xaf778: ldr r3, [pc, #0xb14]
    c.literal(R::new(3)?,0xb0294)?;
    // ARM 0xaf77c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xaf780: str r10, [r9, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(10)?,R::new(9)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf784: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // ARM 0xaf788: add r3, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,255)?)?;
    // ARM 0xaf78c: ldr r7, [r8, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(8)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf790: add r2, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,255)?)?;
    // ARM 0xaf794: cmp r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,255)?)?;
    // ARM 0xaf798: beq 0xaf780 <entry+0x3030>
    c.b(C::Eq,0xaf780,false)?;
    // ARM 0xaf79c: ldr r4, [lr, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::LR,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf7a0: tst r4, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,1879048192)?)?;
    // ARM 0xaf7a4: bne 0xb01fc <entry+0x3aac>
    c.b(C::Ne,0xb01fc,false)?;
    // ARM 0xaf7a8: ldr r4, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // ARM 0xaf7ac: ldr r2, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2432)?)?;
    // ARM 0xaf7b0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaf7b4: bne 0xb0280 <entry+0x3b30>
    c.b(C::Ne,0xb0280,false)?;
    // ARM 0xaf7b8: adds r2, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::new(5)?,Shift::Lsl,0,true)?)?;
    // ARM 0xaf7bc: beq 0xb0280 <entry+0x3b30>
    c.b(C::Eq,0xb0280,false)?;
    // ARM 0xaf7c0: add r2, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaf7c4: ldr r1, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,36)?)?;
    // ARM 0xaf7c8: add r0, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaf7cc: mov r3, #2096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2096)?)?;
    // ARM 0xaf7d0: add r5, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,16)?)?;
    // ARM 0xaf7d4: orr r7, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // ARM 0xaf7d8: lsl r1, r1, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,15)?)?;
    // ARM 0xaf7dc: ldr r3, [r7, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(7)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf7e0: str r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-44)?)?;
    // ARM 0xaf7e4: str r5, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,44)?)?;
    // ARM 0xaf7e8: str r7, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,40)?)?;
    // ARM 0xaf7ec: str r3, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,36)?)?;
    // ARM 0xaf7f0: ldr r3, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,52)?)?;
    // ARM 0xaf7f4: cmp r3, #39
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,39)?)?;
    // ARM 0xaf7f8: bne 0xaf960 <entry+0x3210>
    c.b(C::Ne,0xaf960,false)?;
    // ARM 0xaf7fc: cmp r2, #136
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,136)?)?;
    // ARM 0xaf800: ble 0xafb5c <entry+0x340c>
    c.b(C::Le,0xafb5c,false)?;
    // ARM 0xaf804: cmp r2, #143
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,143)?)?;
    // ARM 0xaf808: bgt 0xafcf8 <entry+0x35a8>
    c.b(C::Gt,0xafcf8,false)?;
    // ARM 0xaf80c: mov r7, #668
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,668)?)?;
    // ARM 0xaf810: cmp r2, #137
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,137)?)?;
    // ARM 0xaf814: orr r7, r7, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,1024)?)?;
    // ARM 0xaf818: beq 0xafd54 <entry+0x3604>
    c.b(C::Eq,0xafd54,false)?;
    // ARM 0xaf81c: cmp r2, #142
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,142)?)?;
    // ARM 0xaf820: bne 0xafef0 <entry+0x37a0>
    c.b(C::Ne,0xafef0,false)?;
    // ARM 0xaf824: ldr r0, [r4, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2528)?)?;
    // ARM 0xaf828: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // ARM 0xaf82c: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // ARM 0xaf830: str r0, [r4, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2528)?)?;
    // ARM 0xaf834: ldr r12, [r4, #0x9f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(4)?,2552)?)?;
    // ARM 0xaf838: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // ARM 0xaf83c: bne 0xaf86c <entry+0x311c>
    c.b(C::Ne,0xaf86c,false)?;
    // ARM 0xaf840: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM 0xaf844: bl 0x2f28 <entry+0xfffffffffff567d8>
    c.b(C::Al,0x2f28,true)?;
    // ARM 0xaf848: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaf84c: beq 0xaff00 <entry+0x37b0>
    c.b(C::Eq,0xaff00,false)?;
    // ARM 0xaf850: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xaf854: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // ARM 0xaf858: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xaf85c: bl 0x31ec <entry+0xfffffffffff56a9c>
    c.b(C::Al,0x31ec,true)?;
    // ARM 0xaf860: mov r12, r4
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(4)?))?;
    // ARM 0xaf864: ldr r4, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // ARM 0xaf868: str r12, [r4, #0x9f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,2552)?)?;
    // ARM 0xaf86c: ldr r2, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // ARM 0xaf870: mov r0, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16777216)?)?;
    // ARM 0xaf874: orr r1, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,2048)?)?;
    // ARM 0xaf878: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // ARM 0xaf87c: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // ARM 0xaf880: str r0, [r1, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf884: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaf888: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xaf88c: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // ARM 0xaf890: orr r2, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaf894: str r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // ARM 0xaf898: ldr r3, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf89c: str r3, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,0)?)?;
    // ARM 0xaf8a0: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // ARM 0xaf8a4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaf8a8: str r2, [r12, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,4)?)?;
    // ARM 0xaf8ac: bne 0xaf97c <entry+0x322c>
    c.b(C::Ne,0xaf97c,false)?;
    // ARM 0xaf8b0: ldr r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // ARM 0xaf8b4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xaf8b8: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // ARM 0xaf8bc: str r0, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf8c0: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // ARM 0xaf8c4: ldr lr, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,12)?)?;
    // ARM 0xaf8c8: mov r2, lr
    c.emit(arm32::mov_register(C::Al,R::R2,R::LR))?;
    // ARM 0xaf8cc: ldr r3, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf8d0: str r3, [r12, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,8)?)?;
    // ARM 0xaf8d4: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // ARM 0xaf8d8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaf8dc: str r2, [r12, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,12)?)?;
    // ARM 0xaf8e0: beq 0xaf91c <entry+0x31cc>
    c.b(C::Eq,0xaf91c,false)?;
    // ARM 0xaf8e4: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // ARM 0xaf8e8: str r0, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf8ec: str r10, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R2,0)?)?;
    // ARM 0xaf8f0: str r10, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R2,0)?)?;
    // ARM 0xaf8f4: str r10, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R2,0)?)?;
    // ARM 0xaf8f8: mov r2, lr
    c.emit(arm32::mov_register(C::Al,R::R2,R::LR))?;
    // ARM 0xaf8fc: ldr r3, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf900: str r3, [r12, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,16)?)?;
    // ARM 0xaf904: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // ARM 0xaf908: str r3, [r12, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,20)?)?;
    // ARM 0xaf90c: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // ARM 0xaf910: str r3, [r12, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,24)?)?;
    // ARM 0xaf914: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // ARM 0xaf918: str r2, [r12, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,28)?)?;
    // ARM 0xaf91c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // ARM 0xaf920: mov r3, lr
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::LR))?;
    // ARM 0xaf924: str r10, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(10)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf928: str r10, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R2,0)?)?;
    // ARM 0xaf92c: ldr r2, [r3, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::new(3)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf930: str r2, [r12, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,32)?)?;
    // ARM 0xaf934: sub r7, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R2,8)?)?;
    // ARM 0xaf938: ldr r3, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // ARM 0xaf93c: cmp r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
    // ARM 0xaf940: str r3, [r12, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,36)?)?;
    // ARM 0xaf944: subls r7, r3, #8
    c.emit(arm32::data_immediate(C::Ls,Op::Sub,R::new(7)?,R::new(3)?,8)?)?;
    // ARM 0xaf948: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xaf94c: cmpls r7, #9
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::new(7)?,9)?)?;
    // ARM 0xaf950: blo 0xaff10 <entry+0x37c0>
    c.b(C::Cc,0xaff10,false)?;
    // ARM 0xaf954: mov r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,127)?)?;
    // ARM 0xaf958: str r0, [r1, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2432)?)?;
    // ARM 0xaf95c: b 0xaf97c <entry+0x322c>
    c.b(C::Al,0xaf97c,false)?;
    // ARM 0xaf960: ldr r1, [pc, #0x930]
    c.literal(R::R1,0xb0298)?;
    // ARM 0xaf964: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaf968: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaf96c: ldr r1, [pc, #0x91c]
    c.literal(R::R1,0xb0290)?;
    // ARM 0xaf970: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaf974: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaf978: ldr r4, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-44)?)?;
    // ARM 0xaf97c: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // ARM 0xaf980: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xaf984: orr r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaf988: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf98c: tst r0, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // ARM 0xaf990: beq 0xaf9dc <entry+0x328c>
    c.b(C::Eq,0xaf9dc,false)?;
    // ARM 0xaf994: mov r1, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,124)?)?;
    // ARM 0xaf998: mov r2, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192)?)?;
    // ARM 0xaf99c: mov r3, #126
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,126)?)?;
    // ARM 0xaf9a0: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xaf9a4: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM 0xaf9a8: orr r1, r4, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaf9ac: str r2, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf9b0: stm sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xaf9b4: ldr r1, [pc, #0x8cc]
    c.literal(R::R1,0xb0288)?;
    // ARM 0xaf9b8: ldr r2, [pc, #0x8e8]
    c.literal(R::R2,0xb02a8)?;
    // ARM 0xaf9bc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xaf9c0: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaf9c4: ldr r1, [pc, #0x8c4]
    c.literal(R::R1,0xb0290)?;
    // ARM 0xaf9c8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xaf9cc: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaf9d0: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xaf9d4: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xaf9d8: str r0, [r1, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2432)?)?;
    // ARM 0xaf9dc: mov r0, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,116)?)?;
    // ARM 0xaf9e0: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xaf9e4: orr r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaf9e8: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaf9ec: and r1, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,5)?)?;
    // ARM 0xaf9f0: cmp r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // ARM 0xaf9f4: bne 0xafa8c <entry+0x333c>
    c.b(C::Ne,0xafa8c,false)?;
    // ARM 0xaf9f8: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // ARM 0xaf9fc: ldr r4, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // ARM 0xafa00: ldr lr, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-48)?)?;
    // ARM 0xafa04: mov r12, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xafa08: mov r7, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,32)?)?;
    // ARM 0xafa0c: ldr r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xafa10: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xafa14: bne 0xafac8 <entry+0x3378>
    c.b(C::Ne,0xafac8,false)?;
    // ARM 0xafa18: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // ARM 0xafa1c: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafa20: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // ARM 0xafa24: ldr r2, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,44)?)?;
    // ARM 0xafa28: sub r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xafa2c: rsb r0, r1, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R0,R::R1,R::R2,Shift::Lsl,3,false)?)?;
    // ARM 0xafa30: lsl r2, r2, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,3)?)?;
    // ARM 0xafa34: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xafa38: movlo r0, #0
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xafa3c: cmp r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32)?)?;
    // ARM 0xafa40: blo 0xafa6c <entry+0x331c>
    c.b(C::Cc,0xafa6c,false)?;
    // ARM 0xafa44: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // ARM 0xafa48: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // ARM 0xafa4c: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xafa50: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xafa54: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // ARM 0xafa58: str r7, [r2, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafa5c: sub r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,32)?)?;
    // ARM 0xafa60: ldr r3, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafa64: cmp r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,31)?)?;
    // ARM 0xafa68: bhi 0xafa58 <entry+0x3308>
    c.b(C::Hi,0xafa58,false)?;
    // ARM 0xafa6c: orr r1, r5, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,2048)?)?;
    // ARM 0xafa70: and r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,255)?)?;
    // ARM 0xafa74: str r0, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafa78: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xafa7c: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xafa80: orr r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xafa84: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafa88: b 0xafac8 <entry+0x3378>
    c.b(C::Al,0xafac8,false)?;
    // ARM 0xafa8c: ldr r1, [pc, #0x818]
    c.literal(R::R1,0xb02ac)?;
    // ARM 0xafa90: ldr r2, [pc, #0x810]
    c.literal(R::R2,0xb02a8)?;
    // ARM 0xafa94: mov r3, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,127)?)?;
    // ARM 0xafa98: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xafa9c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xafaa0: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM 0xafaa4: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xafaa8: ldr r1, [pc, #0x7e0]
    c.literal(R::R1,0xb0290)?;
    // ARM 0xafaac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xafab0: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xafab4: ldr r4, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // ARM 0xafab8: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xafabc: mov r12, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xafac0: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xafac4: ldr lr, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-48)?)?;
    // ARM 0xafac8: ldr r0, [lr, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::LR,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafacc: tst r0, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // ARM 0xafad0: beq 0xafb1c <entry+0x33cc>
    c.b(C::Eq,0xafb1c,false)?;
    // ARM 0xafad4: ldr r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    // ARM 0xafad8: mov r2, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192)?)?;
    // ARM 0xafadc: mov r3, #214
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,214)?)?;
    // ARM 0xafae0: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM 0xafae4: str r2, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafae8: stm sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xafaec: ldr r1, [pc, #0x794]
    c.literal(R::R1,0xb0288)?;
    // ARM 0xafaf0: ldr r2, [pc, #0x794]
    c.literal(R::R2,0xb028c)?;
    // ARM 0xafaf4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xafaf8: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xafafc: ldr r1, [pc, #0x78c]
    c.literal(R::R1,0xb0290)?;
    // ARM 0xafb00: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xafb04: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xafb08: ldr r4, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-36)?)?;
    // ARM 0xafb0c: ldr lr, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,-48)?)?;
    // ARM 0xafb10: mov r12, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xafb14: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xafb18: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xafb1c: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,48)?)?;
    // ARM 0xafb20: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafb24: and r1, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,5)?)?;
    // ARM 0xafb28: cmp r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // ARM 0xafb2c: bne 0xb0250 <entry+0x3b00>
    c.b(C::Ne,0xb0250,false)?;
    // ARM 0xafb30: ldr r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xafb34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xafb38: moveq r0, #134217728
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,134217728)?)?;
    // ARM 0xafb3c: streq r0, [r9, r6]
    c.emit(arm32::transfer_word_register(C::Eq,false,R::R0,R::new(9)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafb40: ldreq r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xafb44: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xafb48: bne 0xb0280 <entry+0x3b30>
    c.b(C::Ne,0xb0280,false)?;
    // ARM 0xafb4c: ldr r0, [r8, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(8)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafb50: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xafb54: bne 0xb01b0 <entry+0x3a60>
    c.b(C::Ne,0xb01b0,false)?;
    // ARM 0xafb58: b 0xb0280 <entry+0x3b30>
    c.b(C::Al,0xb0280,false)?;
    // ARM 0xafb5c: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // ARM 0xafb60: beq 0xafe28 <entry+0x36d8>
    c.b(C::Eq,0xafe28,false)?;
    // ARM 0xafb64: cmp r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // ARM 0xafb68: beq 0xafe60 <entry+0x3710>
    c.b(C::Eq,0xafe60,false)?;
    // ARM 0xafb6c: cmp r2, #45
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,45)?)?;
    // ARM 0xafb70: bne 0xafef0 <entry+0x37a0>
    c.b(C::Ne,0xafef0,false)?;
    // ARM 0xafb74: ldr r0, [r4, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2528)?)?;
    // ARM 0xafb78: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // ARM 0xafb7c: orr r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4)?)?;
    // ARM 0xafb80: str r0, [r4, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2528)?)?;
    // ARM 0xafb84: ldr r4, [r4, #0x9e4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,2532)?)?;
    // ARM 0xafb88: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // ARM 0xafb8c: bne 0xafbbc <entry+0x346c>
    c.b(C::Ne,0xafbbc,false)?;
    // ARM 0xafb90: mov r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,72)?)?;
    // ARM 0xafb94: bl 0x2f28 <entry+0xfffffffffff567d8>
    c.b(C::Al,0x2f28,true)?;
    // ARM 0xafb98: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xafb9c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // ARM 0xafba0: beq 0xaff00 <entry+0x37b0>
    c.b(C::Eq,0xaff00,false)?;
    // ARM 0xafba4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xafba8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xafbac: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // ARM 0xafbb0: bl 0x31ec <entry+0xfffffffffff56a9c>
    c.b(C::Al,0x31ec,true)?;
    // ARM 0xafbb4: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // ARM 0xafbb8: str r4, [r0, #0x9e4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,2532)?)?;
    // ARM 0xafbbc: ldr r3, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-44)?)?;
    // ARM 0xafbc0: mov r2, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16777216)?)?;
    // ARM 0xafbc4: orr r0, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,2048)?)?;
    // ARM 0xafbc8: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // ARM 0xafbcc: str r2, [r1, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafbd0: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,0)?)?;
    // ARM 0xafbd4: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // ARM 0xafbd8: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xafbdc: orr r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xafbe0: mov r2, r1
    c.emit(arm32::mov_register(C::Al,R::R2,R::R1))?;
    // ARM 0xafbe4: ldr r3, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafbe8: str r3, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,0)?)?;
    // ARM 0xafbec: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // ARM 0xafbf0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xafbf4: str r2, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,4)?)?;
    // ARM 0xafbf8: bne 0xafce8 <entry+0x3598>
    c.b(C::Ne,0xafce8,false)?;
    // ARM 0xafbfc: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // ARM 0xafc00: mov r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,7)?)?;
    // ARM 0xafc04: str r3, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafc08: mov r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6)?)?;
    // ARM 0xafc0c: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // ARM 0xafc10: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // ARM 0xafc14: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // ARM 0xafc18: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // ARM 0xafc1c: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // ARM 0xafc20: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // ARM 0xafc24: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // ARM 0xafc28: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // ARM 0xafc2c: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // ARM 0xafc30: ldr r7, [r3, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(7)?,R::new(3)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafc34: str r7, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,8)?)?;
    // ARM 0xafc38: cmp r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,5)?)?;
    // ARM 0xafc3c: ldr r2, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,0)?)?;
    // ARM 0xafc40: str r2, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,12)?)?;
    // ARM 0xafc44: ldr r5, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,0)?)?;
    // ARM 0xafc48: str r5, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,16)?)?;
    // ARM 0xafc4c: ldr r5, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,0)?)?;
    // ARM 0xafc50: str r5, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,20)?)?;
    // ARM 0xafc54: ldr r5, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,0)?)?;
    // ARM 0xafc58: str r5, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,24)?)?;
    // ARM 0xafc5c: ldr r5, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,0)?)?;
    // ARM 0xafc60: str r5, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,28)?)?;
    // ARM 0xafc64: ldr r5, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,0)?)?;
    // ARM 0xafc68: str r5, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,32)?)?;
    // ARM 0xafc6c: ldr r5, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(3)?,0)?)?;
    // ARM 0xafc70: str r5, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,36)?)?;
    // ARM 0xafc74: ldr r3, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // ARM 0xafc78: str r3, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,40)?)?;
    // ARM 0xafc7c: beq 0xafcc4 <entry+0x3574>
    c.b(C::Eq,0xafcc4,false)?;
    // ARM 0xafc80: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xafc84: bne 0xafcc4 <entry+0x3574>
    c.b(C::Ne,0xafcc4,false)?;
    // ARM 0xafc88: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // ARM 0xafc8c: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // ARM 0xafc90: str r3, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafc94: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // ARM 0xafc98: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // ARM 0xafc9c: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // ARM 0xafca0: mov r2, r1
    c.emit(arm32::mov_register(C::Al,R::R2,R::R1))?;
    // ARM 0xafca4: ldr r3, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafca8: str r3, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,44)?)?;
    // ARM 0xafcac: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // ARM 0xafcb0: str r3, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,48)?)?;
    // ARM 0xafcb4: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,0)?)?;
    // ARM 0xafcb8: str r3, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,52)?)?;
    // ARM 0xafcbc: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // ARM 0xafcc0: str r2, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,56)?)?;
    // ARM 0xafcc4: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // ARM 0xafcc8: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // ARM 0xafccc: str r10, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(10)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafcd0: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // ARM 0xafcd4: mov r2, r1
    c.emit(arm32::mov_register(C::Al,R::R2,R::R1))?;
    // ARM 0xafcd8: ldr r3, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafcdc: str r3, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,60)?)?;
    // ARM 0xafce0: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // ARM 0xafce4: str r2, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,64)?)?;
    // ARM 0xafce8: str r5, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(5)?,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafcec: ldr r0, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafcf0: str r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,68)?)?;
    // ARM 0xafcf4: b 0xaf978 <entry+0x3228>
    c.b(C::Al,0xaf978,false)?;
    // ARM 0xafcf8: cmp r2, #144
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,144)?)?;
    // ARM 0xafcfc: beq 0xafeb0 <entry+0x3760>
    c.b(C::Eq,0xafeb0,false)?;
    // ARM 0xafd00: cmp r2, #147
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,147)?)?;
    // ARM 0xafd04: bne 0xafef0 <entry+0x37a0>
    c.b(C::Ne,0xafef0,false)?;
    // ARM 0xafd08: ldr r0, [r4, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2528)?)?;
    // ARM 0xafd0c: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // ARM 0xafd10: str r0, [r4, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2528)?)?;
    // ARM 0xafd14: ldr r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-44)?)?;
    // ARM 0xafd18: orr r0, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R1,2048)?)?;
    // ARM 0xafd1c: str r10, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(10)?,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafd20: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xafd24: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xafd28: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xafd2c: ldr r2, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafd30: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xafd34: str r2, [r4, #0x9f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,2548)?)?;
    // ARM 0xafd38: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // ARM 0xafd3c: ldr r1, [pc, #0x558]
    c.literal(R::R1,0xb029c)?;
    // ARM 0xafd40: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xafd44: ldr r1, [pc, #0x544]
    c.literal(R::R1,0xb0290)?;
    // ARM 0xafd48: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xafd4c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xafd50: b 0xaf97c <entry+0x322c>
    c.b(C::Al,0xaf97c,false)?;
    // ARM 0xafd54: ldr r0, [r4, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2528)?)?;
    // ARM 0xafd58: orr r0, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,64)?)?;
    // ARM 0xafd5c: str r0, [r4, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2528)?)?;
    // ARM 0xafd60: ldr r3, [r4, #0x9e8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2536)?)?;
    // ARM 0xafd64: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xafd68: bne 0xafd9c <entry+0x364c>
    c.b(C::Ne,0xafd9c,false)?;
    // ARM 0xafd6c: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // ARM 0xafd70: bl 0x2f28 <entry+0xfffffffffff567d8>
    c.b(C::Al,0x2f28,true)?;
    // ARM 0xafd74: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xafd78: beq 0xaff00 <entry+0x37b0>
    c.b(C::Eq,0xaff00,false)?;
    // ARM 0xafd7c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xafd80: mov r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // ARM 0xafd84: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xafd88: bl 0x31ec <entry+0xfffffffffff56a9c>
    c.b(C::Al,0x31ec,true)?;
    // ARM 0xafd8c: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // ARM 0xafd90: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // ARM 0xafd94: mov r12, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16)?)?;
    // ARM 0xafd98: str r3, [r0, #0x9e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,2536)?)?;
    // ARM 0xafd9c: ldr r2, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // ARM 0xafda0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xafda4: mov lr, r3
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(3)?))?;
    // ARM 0xafda8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xafdac: orr r4, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::R2,2048)?)?;
    // ARM 0xafdb0: orr r5, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xafdb4: add r2, r3, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,12)?)?;
    // ARM 0xafdb8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xafdbc: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // ARM 0xafdc0: str r12, [r1, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafdc4: str r12, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,0)?)?;
    // ARM 0xafdc8: ldr r7, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafdcc: str r7, [r2, #-0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,-12)?)?;
    // ARM 0xafdd0: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xafdd4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xafdd8: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // ARM 0xafddc: ldr r7, [r1, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(7)?,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafde0: cmp r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,3)?)?;
    // ARM 0xafde4: str r7, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,0)?)?;
    // ARM 0xafde8: str r12, [r0, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafdec: add r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
    // ARM 0xafdf0: str r12, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,0)?)?;
    // ARM 0xafdf4: ldr r7, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    // ARM 0xafdf8: bne 0xafdcc <entry+0x367c>
    c.b(C::Ne,0xafdcc,false)?;
    // ARM 0xafdfc: str r7, [lr, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::LR,24)?)?;
    // ARM 0xafe00: ldr r0, [r5, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafe04: str r0, [lr, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,28)?)?;
    // ARM 0xafe08: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // ARM 0xafe0c: str r0, [r4, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafe10: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // ARM 0xafe14: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // ARM 0xafe18: str r0, [lr, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,32)?)?;
    // ARM 0xafe1c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // ARM 0xafe20: str r0, [lr, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,36)?)?;
    // ARM 0xafe24: b 0xaf978 <entry+0x3228>
    c.b(C::Al,0xaf978,false)?;
    // ARM 0xafe28: ldr r0, [r4, #0x9dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2524)?)?;
    // ARM 0xafe2c: ldr r2, [r4, #0x99c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2460)?)?;
    // ARM 0xafe30: ldr r3, [r4, #0x9d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,2520)?)?;
    // ARM 0xafe34: stm sp, {r0, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xafe38: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xafe3c: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // ARM 0xafe40: bl 0xe1718 <entry+0x34fc8>
    c.b(C::Al,0xe1718,true)?;
    // ARM 0xafe44: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xafe48: ldr r4, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-44)?)?;
    // ARM 0xafe4c: ldrne r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::R11,-36)?)?;
    // ARM 0xafe50: ldrne r0, [r1, #0x9e0]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::R1,2528)?)?;
    // ARM 0xafe54: orrne r0, r0, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,16)?)?;
    // ARM 0xafe58: strne r0, [r1, #0x9e0]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::R1,2528)?)?;
    // ARM 0xafe5c: b 0xaf97c <entry+0x322c>
    c.b(C::Al,0xaf97c,false)?;
    // ARM 0xafe60: ldr r1, [r4, #0x4a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1184)?)?;
    // ARM 0xafe64: ldr r4, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-44)?)?;
    // ARM 0xafe68: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xafe6c: beq 0xaf97c <entry+0x322c>
    c.b(C::Eq,0xaf97c,false)?;
    // ARM 0xafe70: ldr r2, [pc, #0x428]
    c.literal(R::R2,0xb02a0)?;
    // ARM 0xafe74: mov r1, #66
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,66)?)?;
    // ARM 0xafe78: cmp r0, #241
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,241)?)?;
    // ARM 0xafe7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xafe80: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xafe84: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // ARM 0xafe88: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xafe8c: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // ARM 0xafe90: ldr r1, [r0, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2528)?)?;
    // ARM 0xafe94: orr r1, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,32)?)?;
    // ARM 0xafe98: str r1, [r0, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2528)?)?;
    // ARM 0xafe9c: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // ARM 0xafea0: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // ARM 0xafea4: ldr r3, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,44)?)?;
    // ARM 0xafea8: bl 0xe15f0 <entry+0x34ea0>
    c.b(C::Al,0xe15f0,true)?;
    // ARM 0xafeac: b 0xaf97c <entry+0x322c>
    c.b(C::Al,0xaf97c,false)?;
    // ARM 0xafeb0: ldr r0, [r4, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2528)?)?;
    // ARM 0xafeb4: orr r0, r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,128)?)?;
    // ARM 0xafeb8: str r0, [r4, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2528)?)?;
    // ARM 0xafebc: ldr r2, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-44)?)?;
    // ARM 0xafec0: orr r0, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,2048)?)?;
    // ARM 0xafec4: str r12, [r0, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafec8: str r12, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,0)?)?;
    // ARM 0xafecc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xafed0: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xafed4: orr r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xafed8: ldr r1, [r0, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xafedc: str r1, [r4, #0x9ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,2540)?)?;
    // ARM 0xafee0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // ARM 0xafee4: str r0, [r4, #0x9f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2544)?)?;
    // ARM 0xafee8: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // ARM 0xafeec: b 0xaf97c <entry+0x322c>
    c.b(C::Al,0xaf97c,false)?;
    // ARM 0xafef0: ldr r1, [pc, #0x3ac]
    c.literal(R::R1,0xb02a4)?;
    // ARM 0xafef4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xafef8: mov r3, #39
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,39)?)?;
    // ARM 0xafefc: b 0xaf968 <entry+0x3218>
    c.b(C::Al,0xaf968,false)?;
    // ARM 0xaff00: mov r0, #45
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,45)?)?;
    // ARM 0xaff04: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xaff08: str r0, [r1, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2432)?)?;
    // ARM 0xaff0c: b 0xaf978 <entry+0x3228>
    c.b(C::Al,0xaf978,false)?;
    // ARM 0xaff10: add r3, r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,7)?)?;
    // ARM 0xaff14: add r2, r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,7)?)?;
    // ARM 0xaff18: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM 0xaff1c: and r0, r2, #248
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R2,248)?)?;
    // ARM 0xaff20: and r3, r3, #248
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,248)?)?;
    // ARM 0xaff24: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xaff28: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // ARM 0xaff2c: str r10, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(10)?,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaff30: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // ARM 0xaff34: add r7, r12, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R12,R::R2,Shift::Lsl,2,false)?)?;
    // ARM 0xaff38: ldr r2, [lr, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::LR,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaff3c: cmp r2, #33
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,33)?)?;
    // ARM 0xaff40: str r2, [r7, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,40)?)?;
    // ARM 0xaff44: bhs 0xb0070 <entry+0x3920>
    c.b(C::Cs,0xb0070,false)?;
    // ARM 0xaff48: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaff4c: beq 0xaffa8 <entry+0x3858>
    c.b(C::Eq,0xaffa8,false)?;
    // ARM 0xaff50: mov r10, r3
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(3)?))?;
    // ARM 0xaff54: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // ARM 0xaff58: mov r3, lr
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::LR))?;
    // ARM 0xaff5c: ldr r1, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // ARM 0xaff60: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xaff64: add lr, r12, r4, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::LR,R::R12,R::new(4)?,Shift::Lsl,2,false)?)?;
    // ARM 0xaff68: mov r5, r12
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R12))?;
    // ARM 0xaff6c: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // ARM 0xaff70: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xaff74: str r1, [r0, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaff78: mov r1, r3
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(3)?))?;
    // ARM 0xaff7c: ldr r12, [r1, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R12,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xaff80: str r12, [lr, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::LR,52)?)?;
    // ARM 0xaff84: mov r12, r5
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(5)?))?;
    // ARM 0xaff88: ldr r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // ARM 0xaff8c: str r10, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,0)?)?;
    // ARM 0xaff90: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xaff94: str r0, [lr, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,448)?)?;
    // ARM 0xaff98: ldr r0, [r7, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,40)?)?;
    // ARM 0xaff9c: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaffa0: blo 0xaff5c <entry+0x380c>
    c.b(C::Cc,0xaff5c,false)?;
    // ARM 0xaffa4: b 0xaffe4 <entry+0x3894>
    c.b(C::Al,0xaffe4,false)?;
    // ARM 0xaffa8: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // ARM 0xaffac: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xaffb0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM 0xaffb4: mvn r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(7)?,R::R0,0)?)?;
    // ARM 0xaffb8: mov r10, r3
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(3)?))?;
    // ARM 0xaffbc: mov r3, lr
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::LR))?;
    // ARM 0xaffc0: add r0, r12, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::R0,Shift::Lsl,3,false)?)?;
    // ARM 0xaffc4: str r1, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,52)?)?;
    // ARM 0xaffc8: str r1, [r0, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,448)?)?;
    // ARM 0xaffcc: ldr r1, [r12, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,32)?)?;
    // ARM 0xaffd0: add r1, r7, r2, lsl r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R2,Shift::Lsl,R::R1,false)?)?;
    // ARM 0xaffd4: str r1, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,56)?)?;
    // ARM 0xaffd8: ldr r1, [r12, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,36)?)?;
    // ARM 0xaffdc: add r1, r7, r2, lsl r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R2,Shift::Lsl,R::R1,false)?)?;
    // ARM 0xaffe0: str r1, [r0, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,452)?)?;
    // ARM 0xaffe4: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // ARM 0xaffe8: mov lr, r3
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(3)?))?;
    // ARM 0xaffec: mov r3, r10
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // ARM 0xafff0: mov r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,8)?)?;
    // ARM 0xafff4: add r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,1)?)?;
    // ARM 0xafff8: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // ARM 0xafffc: mov r1, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33554432)?)?;
    // ARM 0xb0000: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // ARM 0xb0004: blt 0xaff2c <entry+0x37dc>
    c.b(C::Lt,0xaff2c,false)?;
    // ARM 0xb0008: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb000c: str r3, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-40)?)?;
    // ARM 0xb0010: str r0, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb0014: ldr r0, [lr, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::LR,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb0018: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb001c: str r0, [r12, #0x34c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,844)?)?;
    // ARM 0xb0020: beq 0xb0078 <entry+0x3928>
    c.b(C::Eq,0xb0078,false)?;
    // ARM 0xb0024: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb0028: add r3, r12, #852
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R12,852)?)?;
    // ARM 0xb002c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM 0xb0030: str r0, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb0034: ldr r0, [lr, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::LR,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb0038: str r0, [r12, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,848)?)?;
    // ARM 0xb003c: mov r2, r3
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // ARM 0xb0040: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM 0xb0044: str r1, [r5, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(5)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb0048: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xb004c: ldr r0, [lr, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::LR,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb0050: cmp r4, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,3)?)?;
    // ARM 0xb0054: str r0, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R0,R::R2,4)?)?;
    // ARM 0xb0058: bne 0xb0044 <entry+0x38f4>
    c.b(C::Ne,0xb0044,false)?;
    // ARM 0xb005c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // ARM 0xb0060: add r3, r3, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,12)?)?;
    // ARM 0xb0064: cmp r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // ARM 0xb0068: bne 0xb003c <entry+0x38ec>
    c.b(C::Ne,0xb003c,false)?;
    // ARM 0xb006c: b 0xb00c4 <entry+0x3974>
    c.b(C::Al,0xb00c4,false)?;
    // ARM 0xb0070: mov r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,127)?)?;
    // ARM 0xb0074: b 0xaff04 <entry+0x37b4>
    c.b(C::Al,0xaff04,false)?;
    // ARM 0xb0078: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // ARM 0xb007c: add r3, r12, #852
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R12,852)?)?;
    // ARM 0xb0080: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM 0xb0084: str r0, [r12, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,848)?)?;
    // ARM 0xb0088: mov r2, r3
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // ARM 0xb008c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM 0xb0090: ldr r1, [r12, #0x350]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,848)?)?;
    // ARM 0xb0094: cmp r7, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0098: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb009c: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xb00a0: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb00a4: cmp r4, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,3)?)?;
    // ARM 0xb00a8: lsl r0, r0, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsl,R::R1,false)?)?;
    // ARM 0xb00ac: str r0, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R0,R::R2,4)?)?;
    // ARM 0xb00b0: bne 0xb0090 <entry+0x3940>
    c.b(C::Ne,0xb0090,false)?;
    // ARM 0xb00b4: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // ARM 0xb00b8: add r3, r3, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,12)?)?;
    // ARM 0xb00bc: cmp r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // ARM 0xb00c0: bne 0xb0088 <entry+0x3938>
    c.b(C::Ne,0xb0088,false)?;
    // ARM 0xb00c4: ldr r4, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-44)?)?;
    // ARM 0xb00c8: ldr r3, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // ARM 0xb00cc: mov r1, #222
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,222)?)?;
    // ARM 0xb00d0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // ARM 0xb00d4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM 0xb00d8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // ARM 0xb00dc: str r10, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(10)?,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb00e0: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // ARM 0xb00e4: ldr r2, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb00e8: cmp r2, #33
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,33)?)?;
    // ARM 0xb00ec: str r2, [r12, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R12,R::R1,Shift::Lsl,2)?)?;
    // ARM 0xb00f0: bhs 0xb01a8 <entry+0x3a58>
    c.b(C::Cs,0xb01a8,false)?;
    // ARM 0xb00f4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xb00f8: str r5, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,24)?)?;
    // ARM 0xb00fc: beq 0xb0150 <entry+0x3a00>
    c.b(C::Eq,0xb0150,false)?;
    // ARM 0xb0100: ldr lr, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,12)?)?;
    // ARM 0xb0104: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // ARM 0xb0108: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // ARM 0xb010c: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // ARM 0xb0110: mov r0, lr
    c.emit(arm32::mov_register(C::Al,R::R0,R::LR))?;
    // ARM 0xb0114: add r4, r12, r7, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R12,R::new(7)?,Shift::Lsl,2,false)?)?;
    // ARM 0xb0118: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // ARM 0xb011c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // ARM 0xb0120: str r3, [r1, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb0124: ldr r3, [r0, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb0128: str r3, [r4, #0x384]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,900)?)?;
    // ARM 0xb012c: ldr r3, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // ARM 0xb0130: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // ARM 0xb0134: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xb0138: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // ARM 0xb013c: str r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1296)?)?;
    // ARM 0xb0140: ldr r0, [r12, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R12,R::R1,Shift::Lsl,2)?)?;
    // ARM 0xb0144: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0148: blo 0xb0108 <entry+0x39b8>
    c.b(C::Cc,0xb0108,false)?;
    // ARM 0xb014c: b 0xb0184 <entry+0x3a34>
    c.b(C::Al,0xb0184,false)?;
    // ARM 0xb0150: add r0, r12, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::new(5)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb0154: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // ARM 0xb0158: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb015c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM 0xb0160: mvn r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,0)?)?;
    // ARM 0xb0164: str r1, [r0, #0x384]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,900)?)?;
    // ARM 0xb0168: str r1, [r0, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1296)?)?;
    // ARM 0xb016c: ldr r1, [r12, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,36)?)?;
    // ARM 0xb0170: add r1, r3, r2, lsl r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R2,Shift::Lsl,R::R1,false)?)?;
    // ARM 0xb0174: ldr r3, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-40)?)?;
    // ARM 0xb0178: str r1, [r0, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,904)?)?;
    // ARM 0xb017c: str r1, [r0, #0x514]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1300)?)?;
    // ARM 0xb0180: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xb0184: ldr r5, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,24)?)?;
    // ARM 0xb0188: ldr r4, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-44)?)?;
    // ARM 0xb018c: add r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,1)?)?;
    // ARM 0xb0190: sub r1, r1, #222
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,222)?)?;
    // ARM 0xb0194: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // ARM 0xb0198: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // ARM 0xb019c: add r5, r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,8)?)?;
    // ARM 0xb01a0: blt 0xb00d8 <entry+0x3988>
    c.b(C::Lt,0xb00d8,false)?;
    // ARM 0xb01a4: b 0xaf97c <entry+0x322c>
    c.b(C::Al,0xaf97c,false)?;
    // ARM 0xb01a8: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xb01ac: b 0xaf954 <entry+0x3204>
    c.b(C::Al,0xaf954,false)?;
    // ARM 0xb01b0: mvn r0, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,254)?)?;
    // ARM 0xb01b4: str r10, [r9, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(10)?,R::new(9)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb01b8: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // ARM 0xb01bc: ldr r1, [r8, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(8)?,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb01c0: cmp r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,255)?)?;
    // ARM 0xb01c4: beq 0xb01b4 <entry+0x3a64>
    c.b(C::Eq,0xb01b4,false)?;
    // ARM 0xb01c8: ldr r2, [lr, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::LR,R::new(6)?,Shift::Lsl,0)?)?;
    // ARM 0xb01cc: tst r2, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,1879048192)?)?;
    // ARM 0xb01d0: beq 0xaf76c <entry+0x301c>
    c.b(C::Eq,0xaf76c,false)?;
    // ARM 0xb01d4: ldr r3, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,32)?)?;
    // ARM 0xb01d8: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb01dc: mov r1, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1879048192)?)?;
    // ARM 0xb01e0: str r1, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb01e4: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // ARM 0xb01e8: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // ARM 0xb01ec: ldr r1, [pc, #0x94]
    c.literal(R::R1,0xb0288)?;
    // ARM 0xb01f0: ldr r2, [pc, #0x94]
    c.literal(R::R2,0xb028c)?;
    // ARM 0xb01f4: mov r3, #178
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,178)?)?;
    // ARM 0xb01f8: b 0xb0260 <entry+0x3b10>
    c.b(C::Al,0xb0260,false)?;
    // ARM 0xb01fc: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // ARM 0xb0200: mov r0, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,124)?)?;
    // ARM 0xb0204: mov r2, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192)?)?;
    // ARM 0xb0208: mov r3, #968
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,968)?)?;
    // ARM 0xb020c: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb0210: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb0214: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb0218: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xb021c: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // ARM 0xb0220: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // ARM 0xb0224: ldr r1, [pc, #0x5c]
    c.literal(R::R1,0xb0288)?;
    // ARM 0xb0228: ldr r2, [pc, #0x5c]
    c.literal(R::R2,0xb028c)?;
    // ARM 0xb022c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0230: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0234: ldr r1, [pc, #0x54]
    c.literal(R::R1,0xb0290)?;
    // ARM 0xb0238: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb023c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0240: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // ARM 0xb0244: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xb0248: str r0, [r1, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2432)?)?;
    // ARM 0xb024c: b 0xb0280 <entry+0x3b30>
    c.b(C::Al,0xb0280,false)?;
    // ARM 0xb0250: ldr r1, [pc, #0x54]
    c.literal(R::R1,0xb02ac)?;
    // ARM 0xb0254: ldr r2, [pc, #0x30]
    c.literal(R::R2,0xb028c)?;
    // ARM 0xb0258: mov r3, #215
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,215)?)?;
    // ARM 0xb025c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xb0260: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // ARM 0xb0264: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0268: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb026c: ldr r1, [pc, #0x1c]
    c.literal(R::R1,0xb0290)?;
    // ARM 0xb0270: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb0274: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb0278: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xb027c: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xb0280: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xb0284: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb0288)?;
    // ARM 0xb0288: literal or constant data (NOT executable)
    c.emit(0x000f7058)?;
    // ARM 0xb028c: literal or constant data (NOT executable)
    c.emit(0x000f88b7)?;
    // ARM 0xb0290: literal or constant data (NOT executable)
    c.emit(0x000e2788)?;
    // ARM 0xb0294: literal or constant data (NOT executable)
    c.emit(0xfffffef1)?;
    // ARM 0xb0298: literal or constant data (NOT executable)
    c.emit(0x000f8a41)?;
    // ARM 0xb029c: literal or constant data (NOT executable)
    c.emit(0x000f89de)?;
    // ARM 0xb02a0: literal or constant data (NOT executable)
    c.emit(0x000f88c7)?;
    // ARM 0xb02a4: literal or constant data (NOT executable)
    c.emit(0x000f8a0a)?;
    // ARM 0xb02a8: literal or constant data (NOT executable)
    c.emit(0x000f8a77)?;
    // ARM 0xb02ac: literal or constant data (NOT executable)
    c.emit(0x000f7095)?;
    c.check(0xb02b0)
}
