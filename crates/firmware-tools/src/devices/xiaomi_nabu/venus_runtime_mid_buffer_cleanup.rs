// SPDX-License-Identifier: GPL-3.0-only
//! Codec buffer cleanup and advanced stream-type routing. This includes
//! indirect BLX completion callbacks, postindexed stores, diagnostic frames,
//! and a nine-way switch table. The enclosing event handler continues later.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2be44;
const END:usize=0x2c024;
fn finalize_codec_buffer_and_dispatch(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2be44: ldr r10, [r8, #0x30]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(10)?,R::new(8)?,48)?)?;
    // 0x2be48: ldm r10, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R0.bit()|R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2be4c: ldr r1, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,68)?)?;
    // 0x2be50: blx r1
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    // 0x2be54: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2be58: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2be5c: bne 0x2cc58 <entry+0x1058>  @ imm = #0xdf4
    c.b(C::Ne,0x2cc58)?;
    // 0x2be60: add r5, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(10)?,16)?)?;
    // 0x2be64: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2be68: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2be6c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2be70: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2be74: beq 0x2be80 <entry+0x280>   @ imm = #0x4
    c.b(C::Eq,0x2be80)?;
    // 0x2be78: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2be7c: bl 0x3040 <entry+0xfffffffffffd7440> @ imm = #-0x28e44
    c.bl(0x3040)?;
    // 0x2be80: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x2be84: str r6, [r5], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(6)?,R::new(5)?,4)?)?;
    // 0x2be88: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x2be8c: bne 0x2be6c <entry+0x26c>   @ imm = #-0x28
    c.b(C::Ne,0x2be6c)?;
    // 0x2be90: add r5, r10, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(10)?,28)?)?;
    // 0x2be94: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2be98: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2be9c: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2bea0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2bea4: beq 0x2beb0 <entry+0x2b0>   @ imm = #0x4
    c.b(C::Eq,0x2beb0)?;
    // 0x2bea8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2beac: bl 0x3040 <entry+0xfffffffffffd7440> @ imm = #-0x28e74
    c.bl(0x3040)?;
    // 0x2beb0: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x2beb4: str r6, [r5], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(6)?,R::new(5)?,4)?)?;
    // 0x2beb8: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x2bebc: bne 0x2be9c <entry+0x29c>   @ imm = #-0x28
    c.b(C::Ne,0x2be9c)?;
    // 0x2bec0: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x2bec4: bl 0x2f1c <entry+0xfffffffffffd731c> @ imm = #-0x28fb0
    c.bl(0x2f1c)?;
    // 0x2bec8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2becc: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x2bed0: b 0x2cc58 <entry+0x1058>  @ imm = #0xd80
    c.b(C::Al,0x2cc58)?;
    // 0x2bed4: mov r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x2bed8: orr r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65536)?)?;
    // 0x2bedc: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2bee0: beq 0x2c5ac <entry+0x9ac>   @ imm = #0x6c4
    c.b(C::Eq,0x2c5ac)?;
    // 0x2bee4: ldr r0, [pc, #0x820]        @ 0xb0c <entry+0xb0c>
    c.literal(R::R0,0x2c70c)?;
    // 0x2bee8: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2beec: bne 0x2c0f4 <entry+0x4f4>   @ imm = #0x200
    c.b(C::Ne,0x2c0f4)?;
    // 0x2bef0: ldr r1, [pc, #0xfd4]        @ 0x12cc <entry+0x12cc>
    c.literal(R::R1,0x2cecc)?;
    // 0x2bef4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2bef8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2d94
    c.bl(0x2ec94)?;
    // 0x2befc: ldr r1, [pc, #0x66c]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2bf00: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2bf04: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2d88
    c.bl(0x2ec94)?;
    // 0x2bf08: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x2bf0c: bl 0x2b1cc <entry+0xfffffffffffff5cc> @ imm = #-0xd48
    c.bl(0x2b1cc)?;
    // 0x2bf10: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2bf14: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2bf18: cmp r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,1)?)?;
    // 0x2bf1c: beq 0x2b4ec <entry+0xfffffffffffff8ec> @ imm = #-0xa38
    c.b(C::Eq,0x2b4ec)?;
    // 0x2bf20: ldr r0, [pc, #0xfd4]        @ 0x12fc <entry+0x12fc>
    c.literal(R::R0,0x2cefc)?;
    // 0x2bf24: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2bf28: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2bf2c: orr r1, r1, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,135168)?)?;
    // 0x2bf30: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x2bf34: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x2bf38: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x2bf3c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2bf40: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2bf44: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x2bf48: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2bf4c: str r4, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,32)?)?;
    // 0x2bf50: str r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x2bf54: mov r1, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28)?)?;
    // 0x2bf58: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x2bf5c: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2bf60: ldr r3, [r0, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,128)?)?;
    // 0x2bf64: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2bf68: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2bf6c: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x1ddc
    c.bl(0x2a198)?;
    // 0x2bf70: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // 0x2bf74: b 0x2b4ec <entry+0xfffffffffffff8ec> @ imm = #-0xa90
    c.b(C::Al,0x2b4ec)?;
    // 0x2bf78: ldr r0, [pc, #0xfac]        @ 0x132c <entry+0x132c>
    c.literal(R::R0,0x2cf2c)?;
    // 0x2bf7c: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2bf80: bne 0x2c0f4 <entry+0x4f4>   @ imm = #0x16c
    c.b(C::Ne,0x2c0f4)?;
    // 0x2bf84: ldr r1, [pc, #0xfa4]        @ 0x1330 <entry+0x1330>
    c.literal(R::R1,0x2cf30)?;
    // 0x2bf88: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2bf8c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2d00
    c.bl(0x2ec94)?;
    // 0x2bf90: ldr r1, [pc, #0x5d8]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2bf94: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2bf98: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2cf4
    c.bl(0x2ec94)?;
    // 0x2bf9c: ldr r2, [pc, #0x8b8]        @ 0xc5c <entry+0xc5c>
    c.literal(R::R2,0x2c85c)?;
    // 0x2bfa0: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x2bfa4: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x2bfa8: mov r1, #1424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1424)?)?;
    // 0x2bfac: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2bfb0: bl 0x17b0 <entry+0xfffffffffffd5bb0> @ imm = #-0x2a808
    c.bl(0x17b0)?;
    // 0x2bfb4: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x2bfb8: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x2bfbc: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x2bfc0: bhi 0x2cd68 <entry+0x1168>  @ imm = #0xda0
    c.b(C::Hi,0x2cd68)?;
    // 0x2bfc4: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2bfc8: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x2bfcc: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2bfd0)?;
    const ADVANCED_STREAM_CODES:[u32;9]=[
        0x0002bff4,0x0002cd84,0x0002cde4,0x0002ce10,0x0002ce3c,
        0x0002ce74,0x0002cea0,0x0002ced0,0x0002cf00,
    ];
    for route in ADVANCED_STREAM_CODES {c.emit(route)?;}
    c.check(0x2bff4)?;
    // 0x2bff4: ldr r1, [pc, #0xfac]        @ 0x13a8 <entry+0x13a8>
    c.literal(R::R1,0x2cfa8)?;
    // 0x2bff8: ldr r2, [pc, #0xfac]        @ 0x13ac <entry+0x13ac>
    c.literal(R::R2,0x2cfac)?;
    // 0x2bffc: mov r3, #149
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,149)?)?;
    // 0x2c000: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2c004: orr r3, r3, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1280)?)?;
    // 0x2c008: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2c84
    c.bl(0x2ec94)?;
    // 0x2c00c: ldr r1, [pc, #0x55c]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2c010: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2c014: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2c78
    c.bl(0x2ec94)?;
    // 0x2c018: ldr r0, [pc, #0xf90]        @ 0x13b0 <entry+0x13b0>
    c.literal(R::R0,0x2cfb0)?;
    // 0x2c01c: bl 0x32064 <entry+0x6464>  @ imm = #0x6040
    c.bl(0x32064)?;
    // 0x2c020: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0xb40
    c.b(C::Al,0x2b4e8)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;finalize_codec_buffer_and_dispatch(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn advanced_codec_buffer_dispatch_is_aligned(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|v:usize|u32::from_le_bytes(c.bytes[v-START..v-START+4].try_into().unwrap());
        assert_eq!(word(0x2be50),arm32::branch_exchange_with_link(C::Al,R::R1).unwrap());
        assert_eq!(word(0x2bfd0),0x2bff4);
        assert_eq!(word(0x2bff0),0x2cf00);
    }
}
