// SPDX-License-Identifier: GPL-3.0-only
//! Initialize controller DMA descriptors, release optional queue resources,
//! and select the next state. All controller/diagnostic pointer words are
//! classified separately from typed executable A32 instructions.
use arm32::{Condition as C,DataOp as Op,Register as R,BlockMode};
use super::Code;
const START:usize=0x2f5b0;
const END:usize=0x2f7d0;
fn configure_stream_cache_and_dma(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2f5b0: push {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2f5b4: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2f5b8: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x2f5bc: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x2f5c0: ldr r0, [pc, #0x90]         @ 0x2058 <entry+0x2058>
    c.literal(R::R0,0x2f658)?;
    // 0x2f5c4: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x2f5c8: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x2f5cc: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x2f5d0: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x2f5d4: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x2f5d8: mov r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1024)?)?;
    // 0x2f5dc: str r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // 0x2f5e0: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x2f5e4: add r0, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,4)?)?;
    // 0x2f5e8: str r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,24)?)?;
    // 0x2f5ec: str r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,28)?)?;
    // 0x2f5f0: bl 0x29b4 <entry+0xfffffffffffd53b4> @ imm = #-0x2cc44
    c.bl(0x29b4)?;
    // 0x2f5f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2f5f8: beq 0x2f62c <entry+0x202c>  @ imm = #0x2c
    c.b(C::Eq,0x2f62c)?;
    // 0x2f5fc: ldr r1, [pc, #0x58]         @ 0x205c <entry+0x205c>
    c.literal(R::R1,0x2f65c)?;
    // 0x2f600: ldr r2, [pc, #0x58]         @ 0x60 <entry+0x60>
    c.literal(R::R2,0x2f660)?;
    // 0x2f604: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f608: mov r3, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,46)?)?;
    // 0x2f60c: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x980
    c.bl(0x2ec94)?;
    // 0x2f610: ldr r1, [pc, #0x4c]         @ 0x64 <entry+0x64>
    c.literal(R::R1,0x2f664)?;
    // 0x2f614: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f618: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x98c
    c.bl(0x2ec94)?;
    // 0x2f61c: ldr r2, [pc, #0x44]         @ 0x68 <entry+0x68>
    c.literal(R::R2,0x2f668)?;
    // 0x2f620: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2f624: mov r1, #47
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,47)?)?;
    // 0x2f628: bl 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x2de80
    c.bl(0x17b0)?;
    // 0x2f62c: ldr r1, [r5], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R1,R::new(5)?,4)?)?;
    // 0x2f630: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2f634: bl 0x2c44 <entry+0xfffffffffffd3644> @ imm = #-0x2c9f8
    c.bl(0x2c44)?;
    // 0x2f638: ldr r2, [pc, #0x28]         @ 0x68 <entry+0x68>
    c.literal(R::R2,0x2f668)?;
    // 0x2f63c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2f640: mov r1, #51
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,51)?)?;
    // 0x2f644: moveq r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x2f648: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2f64c: bl 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x2dea4
    c.bl(0x17b0)?;
    // 0x2f650: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x2f654: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2f658)?;
    const DMA_INITIALIZER_DATA_2F658:[u32;5]=[0x0002f66c,0x000eae70,0x000eaeac,0x000e2788,0x000eaec8];
    for word in DMA_INITIALIZER_DATA_2F658 {c.emit(word)?;}
    c.check(0x2f66c)?;
    // 0x2f66c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2f670: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x2f674: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x2f678: ldr r0, [pc, #0x108]        @ 0x188 <entry+0x188>
    c.literal(R::R0,0x2f788)?;
    // 0x2f67c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2f680: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2f684: beq 0x2f68c <entry+0x8c>    @ imm = #0x0
    c.b(C::Eq,0x2f68c)?;
    // 0x2f688: bl 0x257c0 <entry+0xffffffffffff61c0> @ imm = #-0x9ed0
    c.bl(0x257c0)?;
    // 0x2f68c: ldr r0, [pc, #0xf8]         @ 0x18c <entry+0x18c>
    c.literal(R::R0,0x2f78c)?;
    // 0x2f690: ldr r8, [pc, #0xf8]         @ 0x190 <entry+0x190>
    c.literal(R::new(8)?,0x2f790)?;
    // 0x2f694: ldr r6, [pc, #0xf8]         @ 0x194 <entry+0x194>
    c.literal(R::new(6)?,0x2f794)?;
    // 0x2f698: ldr r7, [pc, #0xf8]         @ 0x198 <entry+0x198>
    c.literal(R::new(7)?,0x2f798)?;
    // 0x2f69c: ldr r5, [pc, #0x118]        @ 0x1bc <entry+0x1bc>
    c.literal(R::new(5)?,0x2f7bc)?;
    // 0x2f6a0: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x2f6a4: add r4, r0, #76
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,76)?)?;
    // 0x2f6a8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2f6ac: bl 0x293c <entry+0xfffffffffffd333c> @ imm = #-0x2cd78
    c.bl(0x293c)?;
    // 0x2f6b0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2f6b4: bl 0x294c <entry+0xfffffffffffd334c> @ imm = #-0x2cd70
    c.bl(0x294c)?;
    // 0x2f6b8: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x2f6bc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f6c0: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x2f6c4: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x2f6c8: mov r3, #89
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,89)?)?;
    // 0x2f6cc: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xa40
    c.bl(0x2ec94)?;
    // 0x2f6d0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f6d4: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x2f6d8: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xa4c
    c.bl(0x2ec94)?;
    // 0x2f6dc: tst r9, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(9)?,16777216)?)?;
    // 0x2f6e0: beq 0x2f728 <entry+0x128>   @ imm = #0x40
    c.b(C::Eq,0x2f728)?;
    // 0x2f6e4: ldr r1, [pc, #0xb0]         @ 0x19c <entry+0x19c>
    c.literal(R::R1,0x2f79c)?;
    // 0x2f6e8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f6ec: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x2f6f0: mov r3, #93
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,93)?)?;
    // 0x2f6f4: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xa68
    c.bl(0x2ec94)?;
    // 0x2f6f8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f6fc: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x2f700: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xa74
    c.bl(0x2ec94)?;
    // 0x2f704: bl 0x275f4 <entry+0xffffffffffff7ff4> @ imm = #-0x8118
    c.bl(0x275f4)?;
    // 0x2f708: ldr r0, [pc, #0x90]         @ 0x1a0 <entry+0x1a0>
    c.literal(R::R0,0x2f7a0)?;
    // 0x2f70c: str r10, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,0)?)?;
    // 0x2f710: ldr r0, [pc, #0x8c]         @ 0x1a4 <entry+0x1a4>
    c.literal(R::R0,0x2f7a4)?;
    // 0x2f714: str r10, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,0)?)?;
    // 0x2f718: ldr r0, [pc, #0x88]         @ 0x1a8 <entry+0x1a8>
    c.literal(R::R0,0x2f7a8)?;
    // 0x2f71c: str r10, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,0)?)?;
    // 0x2f720: ldr r0, [pc, #0x84]         @ 0x1ac <entry+0x1ac>
    c.literal(R::R0,0x2f7ac)?;
    // 0x2f724: str r10, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,0)?)?;
    // 0x2f728: tst r9, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(9)?,33554432)?)?;
    // 0x2f72c: beq 0x2f764 <entry+0x164>   @ imm = #0x30
    c.b(C::Eq,0x2f764)?;
    // 0x2f730: ldr r1, [pc, #0x78]         @ 0x1b0 <entry+0x1b0>
    c.literal(R::R1,0x2f7b0)?;
    // 0x2f734: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f738: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x2f73c: mov r3, #99
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,99)?)?;
    // 0x2f740: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xab4
    c.bl(0x2ec94)?;
    // 0x2f744: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f748: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x2f74c: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xac0
    c.bl(0x2ec94)?;
    // 0x2f750: bl 0x27518 <entry+0xffffffffffff7f18> @ imm = #-0x8240
    c.bl(0x27518)?;
    // 0x2f754: ldr r0, [pc, #0x58]         @ 0x1b4 <entry+0x1b4>
    c.literal(R::R0,0x2f7b4)?;
    // 0x2f758: str r10, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,0)?)?;
    // 0x2f75c: ldr r0, [pc, #0x54]         @ 0x1b8 <entry+0x1b8>
    c.literal(R::R0,0x2f7b8)?;
    // 0x2f760: str r10, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,0)?)?;
    // 0x2f764: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f768: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x2f76c: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x2f770: mov r3, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,104)?)?;
    // 0x2f774: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xae8
    c.bl(0x2ec94)?;
    // 0x2f778: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2f77c: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x2f780: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0xaf4
    c.bl(0x2ec94)?;
    // 0x2f784: b 0x2f6a8 <entry+0xa8>    @ imm = #-0xe4
    c.b(C::Al,0x2f6a8)?;
    c.check(0x2f788)?;
    const DMA_INITIALIZER_DATA_2F788:[u32;14]=[0x0010306c,0x0010ceac,0x000eaefb,0x000eaf32,0x000e2788,0x000eaf4c,0x00104094,0x00104098,0x00104090,0x0010408c,0x000eaf62,0x001040a0,0x0010409c,0x000eaf75];
    for word in DMA_INITIALIZER_DATA_2F788 {c.emit(word)?;}
    c.check(0x2f7c0)?;
    // 0x2f7c0: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x2f7c4: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x2f7c8: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2f7cc: b 0x2b58 <entry+0xfffffffffffd3558> @ imm = #-0x2cc7c
    c.b(C::Al,0x2b58)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;configure_stream_cache_and_dma(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn dma_initializer_literals_and_boundaries(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let data=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(data(0x2f658),0x0002f66c);
        assert_eq!(data(0x2f668),0x000eaec8);
        assert_eq!(data(0x2f788),0x0010306c);
        assert_eq!(data(0x2f7bc),0x000eaf75);
    }
}
