// SPDX-License-Identifier: GPL-3.0-only
//! Snapshot MMIO codec register banks and report their status to the platform.
//! Control register value assembly and the optional secondary bank are encoded
//! using typed ARM32 operations; the four trailing words are log pointers,
//! not executable instructions. Reference bytes are never embedded here.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2929c;
const END:usize=0x29474;
const REG_LOG_ADDRESSES:[u32;4]=[
    0x000e_8798, // codec MMIO log
    0x000e_2788, // standard record footer
    0x000e_87b9, // optional codec status
    0x000e_87da, // accumulated frame-length report
];

fn record_mmio_banks(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2929c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x292a0: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x292a4: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // 0x292a8: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x292ac: mov r0, #3088
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3088)?)?;
    // 0x292b0: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x292b4: mov r8, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,3758096384)?)?;
    // 0x292b8: mov r9, r3
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(3)?))?;
    // 0x292bc: mov r3, r2
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // 0x292c0: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x292c4: mov r4, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,28)?)?;
    // 0x292c8: mov r5, #3104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3104)?)?;
    // 0x292cc: mov r7, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,36)?)?;
    // 0x292d0: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x292d4: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // 0x292d8: rsb r10, r3, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::new(10)?,R::new(3)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x292dc: orr r4, r4, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,15360)?)?;
    // 0x292e0: orr r5, r5, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,12288)?)?;
    // 0x292e4: orr r7, r7, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,15360)?)?;
    // 0x292e8: orr r0, r0, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // 0x292ec: orr r1, r1, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,15,false)?)?;
    // 0x292f0: orr r4, r4, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(4)?,R::new(6)?,Shift::Lsl,15,false)?)?;
    // 0x292f4: orr r5, r5, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(5)?,R::new(6)?,Shift::Lsl,15,false)?)?;
    // 0x292f8: orr r7, r7, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,15,false)?)?;
    // 0x292fc: ldr r2, [r0, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x29300: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x29304: str r2, [r0, r10, lsl #2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R0,R::new(10)?,Shift::Lsl,2)?)?;
    // 0x29308: ldr r3, [r1, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R1,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x2930c: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x29310: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // 0x29314: orr r1, r1, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,15,false)?)?;
    // 0x29318: str r3, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // 0x2931c: ldr r12, [r1, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R12,R::R1,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x29320: mov r1, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,40)?)?;
    // 0x29324: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // 0x29328: orr r1, r1, r6, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(6)?,Shift::Lsl,15,false)?)?;
    // 0x2932c: str r12, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,8)?)?;
    // 0x29330: ldr r4, [r4, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::new(4)?,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x29334: str r4, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,12)?)?;
    // 0x29338: ldr r5, [r5, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::new(5)?,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x2933c: str r5, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,16)?)?;
    // 0x29340: ldr r7, [r7, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(7)?,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x29344: str r7, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,20)?)?;
    // 0x29348: ldr r1, [r1, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x2934c: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    // 0x29350: str r12, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,0)?)?;
    // 0x29354: stmib sp, {r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x29358: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x2935c: ldr r1, [pc, #0x100]        @ 0x1c8 <entry+0x1c8>
    c.literal(R::R1,0x29464)?;
    // 0x29360: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x29364: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x5928
    c.bl(0x2ec94)?;
    // 0x29368: ldr r1, [pc, #0xf8]         @ 0x1cc <entry+0x1cc>
    c.literal(R::R1,0x29468)?;
    // 0x2936c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x29370: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x591c
    c.bl(0x2ec94)?;
    // 0x29374: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x29378: lsl r6, r6, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(6)?,Shift::Lsl,15)?)?;
    // 0x2937c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29380: beq 0x29420 <entry+0x184>   @ imm = #0x98
    c.b(C::Eq,0x29420)?;
    // 0x29384: orr r0, r6, #3120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,3120)?)?;
    // 0x29388: orr r1, r6, #52
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(6)?,52)?)?;
    // 0x2938c: orr r7, r6, #60
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(6)?,60)?)?;
    // 0x29390: orr r5, r6, #15424
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(6)?,15424)?)?;
    // 0x29394: orr r4, r6, #68
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(6)?,68)?)?;
    // 0x29398: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x2939c: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // 0x293a0: orr r7, r7, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,15360)?)?;
    // 0x293a4: orr r4, r4, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,15360)?)?;
    // 0x293a8: ldr r2, [r0, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x293ac: add r0, r9, r10, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::new(10)?,Shift::Lsl,2,false)?)?;
    // 0x293b0: str r2, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,56)?)?;
    // 0x293b4: ldr r3, [r1, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R1,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x293b8: orr r1, r6, #56
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(6)?,56)?)?;
    // 0x293bc: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // 0x293c0: str r3, [r0, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,60)?)?;
    // 0x293c4: ldr r12, [r1, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R12,R::R1,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x293c8: orr r1, r6, #72
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(6)?,72)?)?;
    // 0x293cc: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // 0x293d0: str r12, [r0, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,64)?)?;
    // 0x293d4: ldr r7, [r7, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(7)?,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x293d8: str r7, [r0, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,68)?)?;
    // 0x293dc: ldr r5, [r5, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::new(5)?,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x293e0: str r5, [r0, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,72)?)?;
    // 0x293e4: ldr r4, [r4, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::new(4)?,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x293e8: str r4, [r0, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,76)?)?;
    // 0x293ec: ldr r1, [r1, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x293f0: str r1, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,80)?)?;
    // 0x293f4: str r12, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,0)?)?;
    // 0x293f8: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x293fc: str r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,8)?)?;
    // 0x29400: str r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,12)?)?;
    // 0x29404: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x29408: ldr r1, [pc, #0x5c]         @ 0x1d0 <entry+0x1d0>
    c.literal(R::R1,0x2946c)?;
    // 0x2940c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x29410: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x587c
    c.bl(0x2ec94)?;
    // 0x29414: ldr r1, [pc, #0x4c]         @ 0x1cc <entry+0x1cc>
    c.literal(R::R1,0x29468)?;
    // 0x29418: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x2941c: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x5870
    c.bl(0x2ec94)?;
    // 0x29420: orr r0, r6, #2320
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,2320)?)?;
    // 0x29424: ldr r1, [pc, #0x44]         @ 0x1d4 <entry+0x1d4>
    c.literal(R::R1,0x29470)?;
    // 0x29428: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x2942c: ldr r2, [r0, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x29430: orr r0, r6, #280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,280)?)?;
    // 0x29434: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x29438: ldr r3, [r0, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R0,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x2943c: add r0, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x29440: lsl r0, r0, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,8)?)?;
    // 0x29444: str r0, [r9, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,376)?)?;
    // 0x29448: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x2944c: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x5840
    c.bl(0x2ec94)?;
    // 0x29450: ldr r1, [pc, #0x10]         @ 0x1cc <entry+0x1cc>
    c.literal(R::R1,0x29468)?;
    // 0x29454: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x29458: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x2945c: pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x29460: b 0x2ec94 <entry+0x59f8>  @ imm = #0x582c
    c.b(C::Al,0x2ec94)?;
    c.check(0x29464)?;
    for log in REG_LOG_ADDRESSES {c.emit(log)?;}
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{ record_mmio_banks(c) }

#[cfg(test)] mod tests{
    use super::*;
    #[test] fn callback_lifetime_and_pool_exact_layout(){
        let mut c=Code::new(START);
        emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        assert_eq!(&c.bytes[c.bytes.len()-16..],
            &REG_LOG_ADDRESSES.map(|x|x.to_le_bytes()).concat()[..]);
    }
}
