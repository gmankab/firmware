
/// Complete the controller's output command stack and optional tuned DSP
/// buffer copying, preserving the original conditional POP/PC return and
/// tail branch back to the dedicated controller readback helper.
fn finish_stream_controller_command_buffer(c:&mut Code)->Result<(),String>{
    c.check(0xbc598)?;
    // VMA 0xbc598: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbc59c: ldrb r0, [r0, #0x494]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1172)?)?;
    // VMA 0xbc5a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbc5a4: bne 0xbc72c <entry+0x16b4>  @ imm = #0x180
    c.b(C::Ne,0xbc72c,false)?;
    // VMA 0xbc5a8: ldr r4, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,0)?)?;
    // VMA 0xbc5ac: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbc5b0: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbc5b4: mov lr, r7
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(7)?))?;
    // VMA 0xbc5b8: str r0, [r4, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbc5bc: ldr r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,4)?)?;
    // VMA 0xbc5c0: ldr r5, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,40)?)?;
    // VMA 0xbc5c4: str r5, [r3, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(5)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbc5c8: ldr r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    // VMA 0xbc5cc: str r0, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbc5d0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xbc5d4: ldr r7, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,32)?)?;
    // VMA 0xbc5d8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbc5dc: str r10, [r7, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(10)?,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbc5e0: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc5e4: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xbc5e8: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc5ec: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // VMA 0xbc5f0: str r10, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(7)?,0)?)?;
    // VMA 0xbc5f4: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc5f8: ldr r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,44)?)?;
    // VMA 0xbc5fc: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbc600: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc604: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // VMA 0xbc608: str r12, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(7)?,0)?)?;
    // VMA 0xbc60c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbc610: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc614: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbc618: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc61c: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // VMA 0xbc620: str r12, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(7)?,0)?)?;
    // VMA 0xbc624: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc628: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xbc62c: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc630: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // VMA 0xbc634: str r10, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(7)?,0)?)?;
    // VMA 0xbc638: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbc63c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc640: str r5, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(3)?,0)?)?;
    // VMA 0xbc644: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc648: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // VMA 0xbc64c: str r10, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(7)?,0)?)?;
    // VMA 0xbc650: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc654: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbc658: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc65c: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // VMA 0xbc660: str r12, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(7)?,0)?)?;
    // VMA 0xbc664: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbc668: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc66c: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbc670: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc674: mov r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,72)?)?;
    // VMA 0xbc678: str r12, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(7)?,0)?)?;
    // VMA 0xbc67c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc680: ldr r1, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // VMA 0xbc684: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbc688: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc68c: mov r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,72)?)?;
    // VMA 0xbc690: str r9, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(7)?,0)?)?;
    // VMA 0xbc694: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbc698: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc69c: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbc6a0: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc6a4: mov r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,88)?)?;
    // VMA 0xbc6a8: str r9, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(7)?,0)?)?;
    // VMA 0xbc6ac: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc6b0: str lr, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // VMA 0xbc6b4: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc6b8: mov r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,88)?)?;
    // VMA 0xbc6bc: str r8, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(7)?,0)?)?;
    // VMA 0xbc6c0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbc6c4: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc6c8: str lr, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // VMA 0xbc6cc: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc6d0: mov r0, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,104)?)?;
    // VMA 0xbc6d4: str r8, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(7)?,0)?)?;
    // VMA 0xbc6d8: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc6dc: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbc6e0: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc6e4: mov r0, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,104)?)?;
    // VMA 0xbc6e8: str r9, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(7)?,0)?)?;
    // VMA 0xbc6ec: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbc6f0: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc6f4: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xbc6f8: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc6fc: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // VMA 0xbc700: str r9, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(7)?,0)?)?;
    // VMA 0xbc704: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc708: str lr, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // VMA 0xbc70c: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc710: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // VMA 0xbc714: str r8, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(7)?,0)?)?;
    // VMA 0xbc718: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbc71c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xbc720: str lr, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // VMA 0xbc724: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // VMA 0xbc728: str r8, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(7)?,0)?)?;
    // VMA 0xbc72c: ldr r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xbc730: ldr r4, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,36)?)?;
    // VMA 0xbc734: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xbc738: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xbc73c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xbc740: bl 0xbbba4 <entry+0xb2c>   @ imm = #-0xba4
    c.b(C::Al,0xbbba4,true)?;
    // VMA 0xbc744: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // VMA 0xbc748: ldr r1, [r6, #0xf8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,248)?)?;
    // VMA 0xbc74c: ldrh r2, [r6, #236]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(6)?,236)?)?;
    // VMA 0xbc750: orr r0, r0, #108
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,108)?)?;
    // VMA 0xbc754: orr r1, r2, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,16,false)?)?;
    // VMA 0xbc758: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xbc75c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbc760: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbc764: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xbc768: ldr r0, [r0, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2308)?)?;
    // VMA 0xbc76c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xbc770: subeq sp, r11, #28
    c.emit(arm32::data_immediate(C::Eq,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbc774: popeq {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xbc778: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xbc77c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xbc780: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xbc784: pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xbc788: b 0xbbabc <entry+0xa44>   @ imm = #-0xcd4
    c.b(C::Al,0xbbabc,false)?;
    c.check(0xbc78c)
}

// Seven non-executable pointers/format values referenced by PC-relative
// loads inside the original controller command routine.
const STREAM_CONTROLLER_COMMAND_LITERALS:[u32;7]=[
    0x000f_a034,0x000f_a088,0x000e_2788,0x000f_9ffa,
    0x0040_0201,0x000f_a0f0,0x000f_a130,
];

/// Three adjacent helper routines for a single controller MMIO command,
/// an enabled-mode flag and an empty callback stub. Their BX LR returns
/// are preserved; the following routine begins precisely at 0xbc7dc.
fn controller_command_register_helpers(c:&mut Code)->Result<(),String>{
    c.check(0xbc7a8)?;
    // VMA 0xbc7a8: mov r2, #3120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3120)?)?;
    // VMA 0xbc7ac: orr r0, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbc7b0: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xbc7b4: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xbc7b8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xbc7bc: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // VMA 0xbc7c0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbc7c4: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // VMA 0xbc7c8: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // VMA 0xbc7cc: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xbc7d0: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xbc7d4: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xbc7d8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xbc7dc)
}

/// Initialize controller channel masks and MMIO register words.
fn begin_hardware_codec_profile(c:&mut Code)->Result<(),String>{
    c.check(0xbc7dc)?;
    // VMA 0xbc7dc: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xbc7e0: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xbc7e4: sub sp, sp, #52
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,52)?)?;
    // VMA 0xbc7e8: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // VMA 0xbc7ec: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xbc7f0: ldr r9, [r10, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(10)?,364)?)?;
    // VMA 0xbc7f4: ldr r7, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,8)?)?;
    // VMA 0xbc7f8: ldr r1, [r9, #0x134]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,308)?)?;
    // VMA 0xbc7fc: ldr r2, [r9, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,732)?)?;
    // VMA 0xbc800: ldr r3, [r9, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,236)?)?;
    // VMA 0xbc804: ldr r0, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,52)?)?;
    // VMA 0xbc808: ldr r8, [r9, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(9)?,56)?)?;
    // VMA 0xbc80c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // VMA 0xbc810: lsl r1, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,1)?)?;
    // VMA 0xbc814: str r3, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,0)?)?;
    // VMA 0xbc818: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xbc81c: lsl r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,4)?)?;
    // VMA 0xbc820: str r3, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,32)?)?;
    // VMA 0xbc824: str r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,40)?)?;
    // VMA 0xbc828: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // VMA 0xbc82c: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // VMA 0xbc830: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbc834: lsr r5, r0, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(5)?,R::R0,R::R0,Shift::Lsr,R::R1,false)?)?;
    // VMA 0xbc838: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbc83c: lsl r3, r5, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(3)?,R::R0,R::new(5)?,Shift::Lsl,R::R1,false)?)?;
    // VMA 0xbc840: ldr r1, [pc, #0x790]        @ 0x7fc <entry+0x7fc>
    c.literal(R::R1,0xbcfd8)?;
    // VMA 0xbc844: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8dbb8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbc848: ldr r1, [pc, #0x78c]        @ 0x800 <entry+0x800>
    c.literal(R::R1,0xbcfdc)?;
    // VMA 0xbc84c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbc850: bl 0x2ec94 <entry+0xfffffffffff724b8> @ imm = #-0x8dbc4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xbc854: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xbc858: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // VMA 0xbc85c: mov r3, #129
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,129)?)?;
    // VMA 0xbc860: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xbc864: orr r0, r0, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,15360)?)?;
    // VMA 0xbc868: orr r1, r0, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc86c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xbc870: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc874: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // VMA 0xbc878: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // VMA 0xbc87c: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc880: str r3, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc884: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // VMA 0xbc888: mov r1, #788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,788)?)?;
    // VMA 0xbc88c: ldr r3, [r4, #0x51c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,1308)?)?;
    // VMA 0xbc890: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbc894: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc898: str r3, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc89c: mov r1, #340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,340)?)?;
    // VMA 0xbc8a0: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // VMA 0xbc8a4: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc8a8: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc8ac: mov r1, #2256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2256)?)?;
    // VMA 0xbc8b0: ldr r2, [r4, #0x8fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2300)?)?;
    // VMA 0xbc8b4: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xbc8b8: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc8bc: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc8c0: mov r1, #232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,232)?)?;
    // VMA 0xbc8c4: ldr r2, [r4, #0x900]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2304)?)?;
    // VMA 0xbc8c8: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbc8cc: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc8d0: str r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc8d4: mov r2, #340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,340)?)?;
    // VMA 0xbc8d8: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // VMA 0xbc8dc: orr r2, r2, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc8e0: str r6, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc8e4: ldr r2, [r4, #0x900]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2304)?)?;
    // VMA 0xbc8e8: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // VMA 0xbc8ec: mov r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14336)?)?;
    // VMA 0xbc8f0: mov r2, #2112
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2112)?)?;
    // VMA 0xbc8f4: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc8f8: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc8fc: mov r1, #14592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14592)?)?;
    // VMA 0xbc900: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xbc904: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc908: orr r2, r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,512)?)?;
    // VMA 0xbc90c: str r2, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc910: mov r1, #2864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2864)?)?;
    // VMA 0xbc914: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xbc918: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc91c: str r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc920: mov r1, #264
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,264)?)?;
    // VMA 0xbc924: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbc928: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc92c: str r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc930: mov r1, #2832
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2832)?)?;
    // VMA 0xbc934: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xbc938: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc93c: str r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc940: mov r1, #2464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2464)?)?;
    // VMA 0xbc944: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xbc948: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc94c: str r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc950: mov r1, #420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,420)?)?;
    // VMA 0xbc954: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbc958: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc95c: str r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xbc960: mov r1, #424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,424)?)?;
    // VMA 0xbc964: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // VMA 0xbc968: orr r1, r1, r7, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(7)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbc96c: str r6, [r1, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    c.check(0xbc970)
}
