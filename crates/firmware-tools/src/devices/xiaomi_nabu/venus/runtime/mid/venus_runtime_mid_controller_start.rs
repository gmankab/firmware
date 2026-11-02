// SPDX-License-Identifier: GPL-3.0-only
//! Codec controller startup and forced restart. Saves/restores MMIO state,
//! applies IRQ clocks, and uses a checked ARM CP15 cache-control transfer.
//! Data words after POP/PC are address constants, not executable opcodes.
use arm32::{Condition as C, DataOp as Op, Register as R, BlockMode};
use super::Code;
const START:usize=0x2b304;
const END:usize=0x2b474;

/// Initialize controller shadow registers, flush CP15 barriers and update
/// IRQ-driven hardware state before releasing the frame.
fn start_codec_controller(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2b304: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2b308: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x2b30c: bl 0x2693c <entry+0xffffffffffffc53c> @ imm = #-0x49d8
    c.bl(0x2693c)?;
    // 0x2b310: ldr r8, [pc, #0x12c]        @ 0x1044 <entry+0x1044>
    c.literal(R::new(8)?,0x2b444)?;
    // 0x2b314: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2b318: ldr r4, [pc, #0x12c]        @ 0x104c <entry+0x104c>
    c.literal(R::new(4)?,0x2b44c)?;
    // 0x2b31c: str r7, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,0)?)?;
    // 0x2b320: ldr r6, [pc, #0x120]        @ 0x1048 <entry+0x1048>
    c.literal(R::new(6)?,0x2b448)?;
    // 0x2b324: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // 0x2b328: str r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,128)?)?;
    // 0x2b32c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x2b330: str r0, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,132)?)?;
    // 0x2b334: bl 0x38c <entry+0xfffffffffffd5f8c> @ imm = #-0x2afb0
    c.bl(0x38c)?;
    // 0x2b338: ldr r0, [pc, #0x110]        @ 0x1050 <entry+0x1050>
    c.literal(R::R0,0x2b450)?;
    // 0x2b33c: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x2b340: ldr r5, [pc, #0x10c]        @ 0x1054 <entry+0x1054>
    c.literal(R::new(5)?,0x2b454)?;
    // 0x2b344: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x2b348: bne 0x2b394 <entry+0xf94>   @ imm = #0x44
    c.b(C::Ne,0x2b394)?;
    // 0x2b34c: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    // 0x2b350: ldr r0, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,132)?)?;
    // 0x2b354: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x2b358: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // 0x2b35c: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x2b360: str r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // 0x2b364: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2b368: str r0, [r5, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,-60)?)?;
    // 0x2b36c: str r4, [r5, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,212)?)?;
    // 0x2b370: str r0, [r5, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,180)?)?;
    // 0x2b374: bl 0x25c8c <entry+0xffffffffffffb88c> @ imm = #-0x56f0
    c.bl(0x25c8c)?;
    // 0x2b378: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b37c: bl 0x20e8 <entry+0xfffffffffffd7ce8> @ imm = #-0x2929c
    c.bl(0x20e8)?;
    // 0x2b380: bl 0x2571c <entry+0xffffffffffffb31c> @ imm = #-0x5c6c
    c.bl(0x2571c)?;
    // 0x2b384: str r4, [r5, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,-52)?)?;
    // 0x2b388: bl 0x26764 <entry+0xffffffffffffc364> @ imm = #-0x4c2c
    c.bl(0x26764)?;
    // 0x2b38c: str r4, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,0)?)?;
    // 0x2b390: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2b394: ldr r6, [pc, #0xbc]         @ 0x1058 <entry+0x1058>
    c.literal(R::new(6)?,0x2b458)?;
    // 0x2b398: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b39c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x2b3a0: bl 0x20a0 <entry+0xfffffffffffd7ca0> @ imm = #-0x29308
    c.bl(0x20a0)?;
    // 0x2b3a4: bl 0x8e98 <entry+0xfffffffffffdea98> @ imm = #-0x22514
    c.bl(0x8e98)?;
    // 0x2b3a8: ldr r7, [pc, #0xac]         @ 0x105c <entry+0x105c>
    c.literal(R::new(7)?,0x2b45c)?;
    // 0x2b3ac: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x2b3b0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2b3b4: bne 0x2b3bc <entry+0xfbc>   @ imm = #0x0
    c.b(C::Ne,0x2b3bc)?;
    // 0x2b3b8: bl 0x25c48 <entry+0xffffffffffffb848> @ imm = #-0x5778
    c.bl(0x25c48)?;
    // 0x2b3bc: bl 0xe2248 <entry+0xb7e48> @ imm = #0xb6e84
    c.bl(0xe2248)?;
    // 0x2b3c0: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2b3c4: ldr r1, [pc, #0x94]         @ 0x1060 <entry+0x1060>
    c.literal(R::R1,0x2b460)?;
    // 0x2b3c8: ldr r2, [pc, #0x94]         @ 0x1064 <entry+0x1064>
    c.literal(R::R2,0x2b464)?;
    // 0x2b3cc: mov r3, #47
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,47)?)?;
    // 0x2b3d0: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x2b3d4: orr r3, r3, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // 0x2b3d8: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x2b3dc: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x2b3e0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b3e4: mcr p15, #0x0, r4, c7, c0, #0x4
    c.emit(arm32::mcr(C::Al,15,0,R::new(4)?,7,0,4)?)?;
    // 0x2b3e8: bl 0x2ec94 <entry+0x4894>  @ imm = #0x38a4
    c.bl(0x2ec94)?;
    // 0x2b3ec: ldr r1, [pc, #0x74]         @ 0x1068 <entry+0x1068>
    c.literal(R::R1,0x2b468)?;
    // 0x2b3f0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b3f4: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3898
    c.bl(0x2ec94)?;
    // 0x2b3f8: str r4, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,0)?)?;
    // 0x2b3fc: bl 0x8e98 <entry+0xfffffffffffdea98> @ imm = #-0x2256c
    c.bl(0x8e98)?;
    // 0x2b400: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // 0x2b404: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2b408: bne 0x2b424 <entry+0x1024>  @ imm = #0x14
    c.b(C::Ne,0x2b424)?;
    // 0x2b40c: ldr r0, [pc, #0x58]         @ 0x106c <entry+0x106c>
    c.literal(R::R0,0x2b46c)?;
    // 0x2b410: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2b414: tst r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,3)?)?;
    // 0x2b418: bne 0x2b424 <entry+0x1024>  @ imm = #0x4
    c.b(C::Ne,0x2b424)?;
    // 0x2b41c: ldr r0, [pc, #0x4c]         @ 0x1070 <entry+0x1070>
    c.literal(R::R0,0x2b470)?;
    // 0x2b420: bl 0x25c24 <entry+0xffffffffffffb824> @ imm = #-0x5804
    c.bl(0x25c24)?;
    // 0x2b424: bl 0xe2248 <entry+0xb7e48> @ imm = #0xb6e1c
    c.bl(0xe2248)?;
    // 0x2b428: bl 0x26764 <entry+0xffffffffffffc364> @ imm = #-0x4ccc
    c.bl(0x26764)?;
    // 0x2b42c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b430: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x2b434: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2b438: bic r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,256)?)?;
    // 0x2b43c: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x2b440: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2b444)?;
    const MMIO_CONTROLLER_SETTINGS:[u32;12]=[
        0x001040d8,0xe00df00c,0x0010cf08,0x00103078,
        0xe00d204c,0x00103074,0x00104088,0x000e8cb1,
        0x000e8ce6,0x000e2788,0xe00c1008,0x02bde780,
    ];
    for word in MMIO_CONTROLLER_SETTINGS {c.emit(word)?;}
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    start_codec_controller(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_init_and_cache_barrier_have_exact_pool(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|vma:usize|u32::from_le_bytes(c.bytes[vma-START..vma-START+4].try_into().unwrap());
        assert_eq!(word(0x2b3e4),arm32::mcr(C::Al,15,0,R::new(4).unwrap(),7,0,4).unwrap());
        assert_eq!(word(0x2b444),0x001040d8);
        assert_eq!(word(0x2b470),0x02bde780);
    }
}
