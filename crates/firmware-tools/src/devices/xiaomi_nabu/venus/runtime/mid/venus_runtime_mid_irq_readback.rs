// SPDX-License-Identifier: GPL-3.0-only
//! Read back/acknowledge seven peripheral controller interrupt sources.
//! Each small A32 routine has separately modeled data words for controller
//! host address and callback table pointers. Emitted opcodes use architecture
//! operands and are not copied from OEM reference firmware.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::Code;

const START:usize=0x29474;
const END:usize=0x296f0;

/// Extract the active interrupt/status mask using one condition-dependent bit.
fn read_irq_status(c:&mut Code)->Result<(),String>{
    c.check(0x29474)?;
    // 0x29474: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29478: ldr r0, [pc, #0x10]         @ 0x1f4 <entry+0x1f4>
    c.literal(R::R0,0x29490)?;
    // 0x2947c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x29480: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x29484: ldr r0, [r0, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,84)?)?;
    // 0x29488: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2948c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x29490)?;
    // Deliberate literal-pool word(s), not ARM machine instructions.
    const DATA:[u32;1]=[0x0010ceac];
    for value in DATA {c.emit(value)?;}
    c.check(0x29494)
}

/// Acknowledge latched stream bits and perform controller callback readback.
fn clear_irq_latched_status(c:&mut Code)->Result<(),String>{
    c.check(0x29494)?;
    // 0x29494: ldr r3, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // 0x29498: mov r1, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,120)?)?;
    // 0x2949c: mov r12, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // 0x294a0: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x294a4: orr r2, r1, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x294a8: ldr r2, [r2, r12]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R12,Shift::Lsl,0)?)?;
    // 0x294ac: ands r2, r2, #272
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::R2,272,true)?)?;
    // 0x294b0: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x294b4: lsl r3, r3, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsl,15)?)?;
    // 0x294b8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x294bc: orr r3, r3, #20
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,20)?)?;
    // 0x294c0: str r1, [r3, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R12,Shift::Lsl,0)?)?;
    // 0x294c4: mov r3, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,124)?)?;
    // 0x294c8: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x294cc: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x294d0: orr r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // 0x294d4: str r2, [r1, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R12,Shift::Lsl,0)?)?;
    // 0x294d8: ldr r1, [pc, #0xc]          @ 0x250 <entry+0x250>
    c.literal(R::R1,0x294ec)?;
    // 0x294dc: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x294e0: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x294e4: add r0, r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,28)?)?;
    // 0x294e8: b 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x26bc4
    c.b(C::Al,0x292c)?;
    c.check(0x294ec)?;
    // Deliberate literal-pool word(s), not ARM machine instructions.
    const DATA:[u32;1]=[0x0010ceac];
    for value in DATA {c.emit(value)?;}
    c.check(0x294f0)
}

/// Acknowledge DMA channel one and clear stream state.
fn clear_dma_channel_status(c:&mut Code)->Result<(),String>{
    c.check(0x294f0)?;
    // 0x294f0: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x294f4: mov r1, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,56)?)?;
    // 0x294f8: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x294fc: orr r3, r1, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R1,R::R2,Shift::Lsl,15,false)?)?;
    // 0x29500: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x29504: ldr r3, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x29508: tst r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,1)?)?;
    // 0x2950c: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x29510: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // 0x29514: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x29518: orr r2, r2, #60
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,60)?)?;
    // 0x2951c: orr r2, r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    // 0x29520: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x29524: mov r3, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,24)?)?;
    // 0x29528: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x2952c: orr r2, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // 0x29530: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x29534: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x29538: ldr r1, [pc, #0xc]          @ 0x2b0 <entry+0x2b0>
    c.literal(R::R1,0x2954c)?;
    // 0x2953c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x29540: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x29544: add r0, r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,68)?)?;
    // 0x29548: b 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x26c24
    c.b(C::Al,0x292c)?;
    c.check(0x2954c)?;
    // Deliberate literal-pool word(s), not ARM machine instructions.
    const DATA:[u32;1]=[0x0010ceac];
    for value in DATA {c.emit(value)?;}
    c.check(0x29550)
}

/// Acknowledge DMA channel two and notify platform.
fn clear_dma_channel_followup(c:&mut Code)->Result<(),String>{
    c.check(0x29550)?;
    // 0x29550: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x29554: mov r1, #2208
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2208)?)?;
    // 0x29558: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x2955c: orr r3, r1, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R1,R::R2,Shift::Lsl,15,false)?)?;
    // 0x29560: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x29564: ldr r3, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x29568: tst r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,1)?)?;
    // 0x2956c: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x29570: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // 0x29574: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x29578: orr r2, r2, #164
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,164)?)?;
    // 0x2957c: orr r2, r2, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,10240)?)?;
    // 0x29580: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x29584: mov r3, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,24)?)?;
    // 0x29588: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x2958c: orr r2, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    // 0x29590: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x29594: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x29598: ldr r1, [pc, #0xc]          @ 0x310 <entry+0x310>
    c.literal(R::R1,0x295ac)?;
    // 0x2959c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x295a0: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x295a4: add r0, r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,68)?)?;
    // 0x295a8: b 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x26c84
    c.b(C::Al,0x292c)?;
    c.check(0x295ac)?;
    // Deliberate literal-pool word(s), not ARM machine instructions.
    const DATA:[u32;1]=[0x0010ceac];
    for value in DATA {c.emit(value)?;}
    c.check(0x295b0)
}

/// Clear codec warning/status bits and enqueue the corresponding event.
fn clear_codec_warning_bits(c:&mut Code)->Result<(),String>{
    c.check(0x295b0)?;
    // 0x295b0: ldr r3, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // 0x295b4: mov r1, #3648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3648)?)?;
    // 0x295b8: mov r12, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // 0x295bc: orr r2, r1, r3, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::new(3)?,Shift::Lsl,15,false)?)?;
    // 0x295c0: ldr r2, [r2, r12]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R2,R::R12,Shift::Lsl,0)?)?;
    // 0x295c4: ands r2, r2, #3
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::R2,3,true)?)?;
    // 0x295c8: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x295cc: lsl r3, r3, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsl,15)?)?;
    // 0x295d0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x295d4: orr r3, r3, #28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,28)?)?;
    // 0x295d8: str r1, [r3, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R12,Shift::Lsl,0)?)?;
    // 0x295dc: mov r3, #580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,580)?)?;
    // 0x295e0: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x295e4: orr r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,3072)?)?;
    // 0x295e8: orr r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // 0x295ec: str r2, [r1, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R12,Shift::Lsl,0)?)?;
    // 0x295f0: ldr r1, [pc, #0xc]          @ 0x368 <entry+0x368>
    c.literal(R::R1,0x29604)?;
    // 0x295f4: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x295f8: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x295fc: add r0, r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,44)?)?;
    // 0x29600: b 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x26cdc
    c.b(C::Al,0x292c)?;
    c.check(0x29604)?;
    // Deliberate literal-pool word(s), not ARM machine instructions.
    const DATA:[u32;1]=[0x0010ceac];
    for value in DATA {c.emit(value)?;}
    c.check(0x29608)
}

/// Handle a masked interrupt readback and conditional channel reset.
fn acknowledge_irq_condition(c:&mut Code)->Result<(),String>{
    c.check(0x29608)?;
    // 0x29608: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x2960c: mov r1, #2208
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2208)?)?;
    // 0x29610: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x29614: orr r3, r1, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R1,R::R2,Shift::Lsl,15,false)?)?;
    // 0x29618: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x2961c: ldr r3, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x29620: tst r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,1)?)?;
    // 0x29624: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x29628: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // 0x2962c: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x29630: orr r2, r2, #164
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,164)?)?;
    // 0x29634: orr r2, r2, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,10240)?)?;
    // 0x29638: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x2963c: ldr r2, [pc, #0x4c]         @ 0x3f4 <entry+0x3f4>
    c.literal(R::R2,0x29690)?;
    // 0x29640: ldr r1, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // 0x29644: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x29648: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // 0x2964c: ldrb r2, [r0, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,8)?)?;
    // 0x29650: tst r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,1)?)?;
    // 0x29654: bne 0x29660 <entry+0x3c4>   @ imm = #0x4
    c.b(C::Ne,0x29660)?;
    // 0x29658: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x2965c: bne 0x2968c <entry+0x3f0>   @ imm = #0x28
    c.b(C::Ne,0x2968c)?;
    // 0x29660: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x29664: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x29668: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2966c: orr r1, r2, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    // 0x29670: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x29674: str r3, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    // 0x29678: ldr r1, [pc, #0x14]         @ 0x3f8 <entry+0x3f8>
    c.literal(R::R1,0x29694)?;
    // 0x2967c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x29680: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x29684: add r0, r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,68)?)?;
    // 0x29688: b 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x26d64
    c.b(C::Al,0x292c)?;
    // 0x2968c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x29690)?;
    // Deliberate literal-pool word(s), not ARM machine instructions.
    const DATA:[u32;2]=[0x001040cc,0x0010ceac];
    for value in DATA {c.emit(value)?;}
    c.check(0x29698)
}

/// Check a streaming warning bit and complete status acknowledgement.
fn acknowledge_stream_warning(c:&mut Code)->Result<(),String>{
    c.check(0x29698)?;
    // 0x29698: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x2969c: mov r1, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,360)?)?;
    // 0x296a0: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // 0x296a4: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x296a8: orr r1, r1, r2, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,15,false)?)?;
    // 0x296ac: ldr r1, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x296b0: tst r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,4)?)?;
    // 0x296b4: bne 0x296e8 <entry+0x44c>   @ imm = #0x2c
    c.b(C::Ne,0x296e8)?;
    // 0x296b8: tst r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1)?)?;
    // 0x296bc: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x296c0: lsl r2, r2, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    // 0x296c4: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // 0x296c8: orr r2, r2, #364
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,364)?)?;
    // 0x296cc: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x296d0: str r1, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x296d4: ldr r1, [pc, #0x10]         @ 0x450 <entry+0x450>
    c.literal(R::R1,0x296ec)?;
    // 0x296d8: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x296dc: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x296e0: add r0, r0, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,60)?)?;
    // 0x296e4: b 0x292c <entry+0xfffffffffffd9690> @ imm = #-0x26dc0
    c.b(C::Al,0x292c)?;
    // 0x296e8: b 0x28568 <entry+0xfffffffffffff2cc> @ imm = #-0x1188
    c.b(C::Al,0x28568)?;
    c.check(0x296ec)?;
    // Deliberate literal-pool word(s), not ARM machine instructions.
    const DATA:[u32;1]=[0x0010ceac];
    for value in DATA {c.emit(value)?;}
    c.check(0x296f0)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    read_irq_status(c)?;
    clear_irq_latched_status(c)?;
    clear_dma_channel_status(c)?;
    clear_dma_channel_followup(c)?;
    clear_codec_warning_bits(c)?;
    acknowledge_irq_condition(c)?;
    acknowledge_stream_warning(c)?;
    c.check(END)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn seven_irq_handlers_and_literals_are_adjacent(){
        let mut c=Code::new(START);
        emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|addr:usize|u32::from_le_bytes(c.bytes[addr-START..addr-START+4].try_into().unwrap());
        assert_eq!(word(0x29490),0x0010_ceac);
        assert_eq!(word(0x29690),0x0010_40cc);
        assert_eq!(word(0x29694),0x0010_ceac);
    }
}
