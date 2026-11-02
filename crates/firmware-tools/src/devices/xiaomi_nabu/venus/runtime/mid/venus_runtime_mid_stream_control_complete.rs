// SPDX-License-Identifier: GPL-3.0-only
//! Complete the large stream-controller parameter matrix and copy results
//! through a code-selected callback. The nine literal pools below are
//! non-executable codec string pointers and branch tables; all machine code
//! between them uses public checked typed ARM32 operand constructors.
use super::Code;
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
const START:usize=0x3dc00;
const END:usize=0x3dfec;

/// Public ARMv7 signed byte/halfword preindexed immediate transfer wrapper.
/// Writeback is explicit and PC operands are rejected.
fn encode_signed_transfer_wb(c:C,dest:R,base:R,offset:i32,byte:bool)->Result<u32,String>{
    if dest==R::PC||base==R::PC||dest==base {
        return Err("invalid signed load with base writeback".into());
    }
    let opcode=if byte {arm32::load_signed_byte(c,dest,base,offset)?}
               else {arm32::load_signed_halfword(c,dest,base,offset)?};
    Ok(opcode|(1<<21))
}

const QUEUE_ENTRY_DIAGNOSTIC:[u32;1]=[
    0x000ed672,
];

const QUEUE_INPUT_DISPATCH:[u32;11]=[
    0x0003dce4, 0x0003de94, 0x0003dd40, 0x0003dd40, 0x0003dec0, 0x0003dec0,
    0x0003dc70, 0x0003dec0, 0x0003dec0, 0x0003dce4, 0x000ed681,
];

const QUEUE_OUTPUT_DISPATCH:[u32;11]=[
    0x000ed68f, 0x0003dd30, 0x0003dd4c, 0x0003dd64, 0x0003dd90, 0x0003dd90,
    0x0003dd90, 0x0003de74, 0x0003dd90, 0x0003dd90, 0x0003dd30,
];

const SIGNED_PLANE_MESSAGES:[u32;2]=[
    0x000ed6a0, 0x000ed6b0,
];

const QUEUE_COPY_DISPATCH:[u32;8]=[
    0x000ed6be, 0x0003dde8, 0x0003ddfc, 0x0003de0c, 0x0003de30, 0x0003de30,
    0x0003de30, 0x0003de18,
];

const COPY_BAD_FORMAT:[u32;1]=[
    0x000ed6cb,
];

const COPY_SKIP_FORMAT:[u32;1]=[
    0x000ed6db,
];

const COPY_COMPLETION_MESSAGES:[u32;2]=[
    0x000ed6ea, 0x000ed6f8,
];

const STREAM_TYPE_DIAGNOSTICS:[u32;60]=[
    0x000ed705, 0x000ed715, 0x000ed724, 0x000ed735, 0x000ed745, 0x000ed756,
    0x000ed777, 0x000ed798, 0x000ed7a8, 0x000ed7b9, 0x000ed7c9, 0x000ed7da,
    0x000ed7ea, 0x000ed7fb, 0x000ed80b, 0x000ed81c, 0x000ed82c, 0x000ed837,
    0x000ed841, 0x000ed84e, 0x000ed863, 0x000ed876, 0x000ed890, 0x000ed89a,
    0x000ed8a8, 0x000ed8ba, 0x000ed8d3, 0x000ed8e0, 0x000ed8e8, 0x000ed8f7,
    0x000ed908, 0x000ed917, 0x000ed926, 0x000ed93d, 0x000ed94e, 0x000ed95e,
    0x000ed96e, 0x000ed97e, 0x000ed990, 0x000ed99a, 0x000ed9a5, 0x000ed9b0,
    0x000ed9bb, 0x000ed9cc, 0x000ed9dc, 0x000ed9ee, 0x000eda06, 0x000eda1d,
    0x000eda34, 0x000eda4d, 0x000eda61, 0x000eda74, 0x000eda8f, 0x000eda97,
    0x000edaa8, 0x000edab3, 0x000edaac, 0x000edac0, 0x000edabc, 0x000e2788,
];

/// Finish writing the packed stream-configuration parameters, resolve data
/// type cases, send completed records, and restore the saved controller state.
fn finalize_controller_stream_matrix(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3dc00: add r1, r6, #5056
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,5056)?)?;
    // 0x3dc04: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dc08: mov r1, #700
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,700)?)?;
    // 0x3dc0c: mov r2, #5824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5824)?)?;
    // 0x3dc10: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x3dc14: str r0, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R1,Shift::Lsl,0)?)?;
    // 0x3dc18: ldr r1, [pc, #0x3b4]        @ 0x4404 <entry+0x4404>
    c.literal_cond(C::Al,R::R1,0x3dfd4)?;
    // 0x3dc1c: str r1, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dc20: mov r2, #708
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,708)?)?;
    // 0x3dc24: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3dc28: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dc2c: mov r2, #712
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,712)?)?;
    // 0x3dc30: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3dc34: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dc38: mov r2, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,716)?)?;
    // 0x3dc3c: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x3dc40: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3dc44: ldr r9, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,24)?)?;
    // 0x3dc48: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x3dc4c: bl 0x841e4 <entry+0x4a614> @ imm = #0x46590
    c.bl(0x841e4)?;
    // 0x3dc50: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3dc54: beq 0x3dee0 <entry+0x4310>  @ imm = #0x284
    c.b(C::Eq,0x3dee0)?;
    // 0x3dc58: ldr r5, [pc, #0x374]        @ 0x4404 <entry+0x4404>
    c.literal_cond(C::Al,R::new(5)?,0x3dfd4)?;
    // 0x3dc5c: add r10, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,8)?)?;
    // 0x3dc60: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x3dc64: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x3dc68: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x3dc6c: b 0x3dc8c <entry+0x40bc>  @ imm = #0x18
    c.b(C::Al,0x3dc8c)?;
    // 0x3dc70: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x3dc74: ldr r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    // 0x3dc78: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // 0x3dc7c: ldr r2, [pc, #0x354]        @ 0x4408 <entry+0x4408>
    c.literal_cond(C::Al,R::R2,0x3dfd8)?;
    // 0x3dc80: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x3dc84: b 0x3dea8 <entry+0x42d8>  @ imm = #0x21c
    c.b(C::Al,0x3dea8)?;
    c.check(0x3dc88)?;
    for entry in QUEUE_ENTRY_DIAGNOSTIC {c.emit(entry)?;}
    c.check(0x3dc8c)?;
    // 0x3dc8c: ldr r0, [r10, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,-4)?)?;
    // 0x3dc90: ldr r1, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,4)?)?;
    // 0x3dc94: add r6, r6, #14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,14)?)?;
    // 0x3dc98: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x3dc9c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x3dca0: bne 0x3dcf0 <entry+0x4120>  @ imm = #0x48
    c.b(C::Ne,0x3dcf0)?;
    // 0x3dca4: cmp r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,9)?)?;
    // 0x3dca8: bhi 0x3dec0 <entry+0x42f0>  @ imm = #0x210
    c.b(C::Hi,0x3dec0)?;
    // 0x3dcac: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x3dcb0: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x3dcb4: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x3dcb8)?;
    for entry in QUEUE_INPUT_DISPATCH {c.emit(entry)?;}
    c.check(0x3dce4)?;
    // 0x3dce4: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x3dce8: ldrsb r0, [r0]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::R0,0)?)?;
    // 0x3dcec: b 0x3de9c <entry+0x42cc>  @ imm = #0x1a8
    c.b(C::Al,0x3de9c)?;
    // 0x3dcf0: cmp r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,9)?)?;
    // 0x3dcf4: bhi 0x3dd90 <entry+0x41c0>  @ imm = #0x94
    c.b(C::Hi,0x3dd90)?;
    // 0x3dcf8: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x3dcfc: add r1, pc, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,4)?)?;
    // 0x3dd00: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x3dd04)?;
    for entry in QUEUE_OUTPUT_DISPATCH {c.emit(entry)?;}
    c.check(0x3dd30)?;
    // 0x3dd30: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x3dd34: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x3dd38: ldrsb r0, [r0]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::R0,0)?)?;
    // 0x3dd3c: b 0x3dd6c <entry+0x419c>  @ imm = #0x28
    c.b(C::Al,0x3dd6c)?;
    // 0x3dd40: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x3dd44: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x3dd48: b 0x3de9c <entry+0x42cc>  @ imm = #0x14c
    c.b(C::Al,0x3de9c)?;
    // 0x3dd4c: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x3dd50: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x3dd54: ldrsh r0, [r0]
    c.emit(arm32::load_signed_halfword(C::Al,R::R0,R::R0,0)?)?;
    // 0x3dd58: b 0x3dd6c <entry+0x419c>  @ imm = #0xc
    c.b(C::Al,0x3dd6c)?;
    c.check(0x3dd5c)?;
    for entry in SIGNED_PLANE_MESSAGES {c.emit(entry)?;}
    c.check(0x3dd64)?;
    // 0x3dd64: ldr r8, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(10)?,0)?)?;
    // 0x3dd68: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x3dd6c: ldr r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    // 0x3dd70: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // 0x3dd74: ldr r2, [pc, #0x260]        @ 0x440c <entry+0x440c>
    c.literal_cond(C::Al,R::R2,0x3dfdc)?;
    // 0x3dd78: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x3dd7c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x3dd80: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // 0x3dd84: bl 0x31c0 <entry+0xfffffffffffc95f0> @ imm = #-0x3abcc
    c.bl(0x31c0)?;
    // 0x3dd88: add r4, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3dd8c: str r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,12)?)?;
    // 0x3dd90: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x3dd94: mov r9, r5
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(5)?))?;
    // 0x3dd98: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x3dd9c: blt 0x3de4c <entry+0x427c>  @ imm = #0xa8
    c.b(C::Lt,0x3de4c)?;
    // 0x3dda0: ldr r7, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,12)?)?;
    // 0x3dda4: ldr r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,8)?)?;
    // 0x3dda8: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x3ddac: ldr r0, [r10, #-0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,-4)?)?;
    // 0x3ddb0: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x3ddb4: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x3ddb8: bhi 0x3de30 <entry+0x4260>  @ imm = #0x70
    c.b(C::Hi,0x3de30)?;
    // 0x3ddbc: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x3ddc0: add r1, pc, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,4)?)?;
    // 0x3ddc4: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x3ddc8)?;
    for entry in QUEUE_COPY_DISPATCH {c.emit(entry)?;}
    c.check(0x3dde8)?;
    // 0x3dde8: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x3ddec: ldrsb r3, [r0, #1]!
    c.emit(encode_signed_transfer_wb(C::Al,R::new(3)?,R::R0,1,true)?)?;
    // 0x3ddf0: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x3ddf4: b 0x3de10 <entry+0x4240>  @ imm = #0x14
    c.b(C::Al,0x3de10)?;
    c.check(0x3ddf8)?;
    for entry in COPY_BAD_FORMAT {c.emit(entry)?;}
    c.check(0x3ddfc)?;
    // 0x3ddfc: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x3de00: ldrsh r3, [r0, #2]!
    c.emit(encode_signed_transfer_wb(C::Al,R::new(3)?,R::R0,2,false)?)?;
    // 0x3de04: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x3de08: b 0x3de10 <entry+0x4240>  @ imm = #0x0
    c.b(C::Al,0x3de10)?;
    // 0x3de0c: ldr r3, [r8, #0x4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(3)?,R::new(8)?,4)?)?;
    // 0x3de10: ldr r2, [pc, #0x1cc]        @ 0x4414 <entry+0x4414>
    c.literal_cond(C::Al,R::R2,0x3dfe4)?;
    // 0x3de14: b 0x3de20 <entry+0x4250>  @ imm = #0x4
    c.b(C::Al,0x3de20)?;
    // 0x3de18: ldr r3, [r8, #0x4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(3)?,R::new(8)?,4)?)?;
    // 0x3de1c: ldr r2, [pc, #0x1bc]        @ 0x4410 <entry+0x4410>
    c.literal_cond(C::Al,R::R2,0x3dfe0)?;
    // 0x3de20: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x3de24: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x3de28: bl 0x31c0 <entry+0xfffffffffffc95f0> @ imm = #-0x3ac70
    c.bl(0x31c0)?;
    // 0x3de2c: add r7, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3de30: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x3de34: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x3de38: add r6, r6, #13
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,13)?)?;
    // 0x3de3c: cmp r4, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3de40: blt 0x3ddac <entry+0x41dc>  @ imm = #-0x9c
    c.b(C::Lt,0x3ddac)?;
    // 0x3de44: b 0x3de54 <entry+0x4284>  @ imm = #0x8
    c.b(C::Al,0x3de54)?;
    c.check(0x3de48)?;
    for entry in COPY_SKIP_FORMAT {c.emit(entry)?;}
    c.check(0x3de4c)?;
    // 0x3de4c: ldr r7, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,12)?)?;
    // 0x3de50: ldr r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,8)?)?;
    // 0x3de54: ldr r2, [pc, #0x18c]        @ 0x4418 <entry+0x4418>
    c.literal_cond(C::Al,R::R2,0x3dfe8)?;
    // 0x3de58: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x3de5c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x3de60: bl 0x31c0 <entry+0xfffffffffffc95f0> @ imm = #-0x3aca8
    c.bl(0x31c0)?;
    // 0x3de64: add r7, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3de68: mov r5, r9
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(9)?))?;
    // 0x3de6c: str r7, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
    // 0x3de70: b 0x3dec0 <entry+0x42f0>  @ imm = #0x48
    c.b(C::Al,0x3dec0)?;
    // 0x3de74: ldr r8, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(10)?,0)?)?;
    // 0x3de78: ldr r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    // 0x3de7c: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // 0x3de80: ldr r2, [pc, #0x150]        @ 0x4408 <entry+0x4408>
    c.literal_cond(C::Al,R::R2,0x3dfd8)?;
    // 0x3de84: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x3de88: b 0x3dd78 <entry+0x41a8>  @ imm = #-0x118
    c.b(C::Al,0x3dd78)?;
    c.check(0x3de8c)?;
    for entry in COPY_COMPLETION_MESSAGES {c.emit(entry)?;}
    c.check(0x3de94)?;
    // 0x3de94: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x3de98: ldrsh r0, [r0]
    c.emit(arm32::load_signed_halfword(C::Al,R::R0,R::R0,0)?)?;
    // 0x3de9c: ldr r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    // 0x3dea0: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // 0x3dea4: ldr r2, [pc, #0x130]        @ 0x440c <entry+0x440c>
    c.literal_cond(C::Al,R::R2,0x3dfdc)?;
    // 0x3dea8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x3deac: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x3deb0: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // 0x3deb4: bl 0x31c0 <entry+0xfffffffffffc95f0> @ imm = #-0x3acfc
    c.bl(0x31c0)?;
    // 0x3deb8: add r4, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3debc: str r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,12)?)?;
    // 0x3dec0: ldr r9, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(10)?,8)?)?;
    // 0x3dec4: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x3dec8: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x3decc: bl 0x841e4 <entry+0x4a614> @ imm = #0x46310
    c.bl(0x841e4)?;
    // 0x3ded0: add r10, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,16)?)?;
    // 0x3ded4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3ded8: bne 0x3dc8c <entry+0x40bc>  @ imm = #-0x254
    c.b(C::Ne,0x3dc8c)?;
    // 0x3dedc: b 0x3dee4 <entry+0x4314>  @ imm = #0x0
    c.b(C::Al,0x3dee4)?;
    // 0x3dee0: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x3dee4: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x3dee8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3deec: strb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x3def0: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x3def4: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x3def8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x3defc)?;
    for entry in STREAM_TYPE_DIAGNOSTICS {c.emit(entry)?;}
    c.check(0x3dfec)?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{finalize_controller_stream_matrix(c)}

#[cfg(test)]mod tests{
 use super::*;
 #[test]fn stream_controller_epilogue_and_diagnostics_are_exact(){
    let mut c=Code::new(START);emit(&mut c).unwrap();
    assert_eq!(c.bytes.len(),END-START);
    assert_eq!(STREAM_TYPE_DIAGNOSTICS.len(),60);
    assert_eq!(QUEUE_INPUT_DISPATCH.len(),11);
    assert_eq!(QUEUE_OUTPUT_DISPATCH.len(),11);
    assert_eq!(QUEUE_COPY_DISPATCH.len(),8);
    assert_eq!(STREAM_TYPE_DIAGNOSTICS.last().copied(),Some(0x000e_2788));
 }
 #[test]fn signed_register_writeback_loads_have_valid_isa_bits(){
    assert_eq!(encode_signed_transfer_wb(C::Al,R::new(3).unwrap(),R::R0,1,true).unwrap(),0xe1f0_30d1);
    assert_eq!(encode_signed_transfer_wb(C::Al,R::new(3).unwrap(),R::R0,2,false).unwrap(),0xe1f0_30f2);
    assert!(encode_signed_transfer_wb(C::Al,R::R0,R::R0,1,true).is_err());
 }
}
