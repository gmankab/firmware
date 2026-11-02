// SPDX-License-Identifier: GPL-3.0-only
//! Stream class-specific controller request handlers and descriptor table
//! dispatch; literal pools and route targets are separated from executable
//! typed ARM A32 code. The enclosing larger function continues afterwards.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2c024;
const END:usize=0x2c5ac;

/// Route codec buffer-class messages, invoke synchronous platform callbacks
/// and clean up invalid/missing request payloads.
fn dispatch_codec_buffer_classes(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2c024: ldr r1, [pc, #0xff8]        @ 0x1424 <entry+0x1424>
    c.literal(R::R1,0x2d024)?;
    // 0x2c028: b 0x2c060 <entry+0x460>   @ imm = #0x30
    c.b(C::Al,0x2c060)?;
    // 0x2c02c: ldr r1, [pc, #0xff4]        @ 0x1428 <entry+0x1428>
    c.literal(R::R1,0x2d028)?;
    // 0x2c030: b 0x2c060 <entry+0x460>   @ imm = #0x28
    c.b(C::Al,0x2c060)?;
    // 0x2c034: ldr r1, [pc, #0xff0]        @ 0x142c <entry+0x142c>
    c.literal(R::R1,0x2d02c)?;
    // 0x2c038: b 0x2c060 <entry+0x460>   @ imm = #0x20
    c.b(C::Al,0x2c060)?;
    // 0x2c03c: ldr r1, [pc, #0xfec]        @ 0x1430 <entry+0x1430>
    c.literal(R::R1,0x2d030)?;
    // 0x2c040: b 0x2c060 <entry+0x460>   @ imm = #0x18
    c.b(C::Al,0x2c060)?;
    // 0x2c044: ldr r1, [pc, #0xfe8]        @ 0x1434 <entry+0x1434>
    c.literal(R::R1,0x2d034)?;
    // 0x2c048: b 0x2c060 <entry+0x460>   @ imm = #0x10
    c.b(C::Al,0x2c060)?;
    // 0x2c04c: ldr r1, [pc, #0xfe4]        @ 0x1438 <entry+0x1438>
    c.literal(R::R1,0x2d038)?;
    // 0x2c050: b 0x2c060 <entry+0x460>   @ imm = #0x8
    c.b(C::Al,0x2c060)?;
    // 0x2c054: ldr r1, [pc, #0xfe0]        @ 0x143c <entry+0x143c>
    c.literal(R::R1,0x2d03c)?;
    // 0x2c058: b 0x2c060 <entry+0x460>   @ imm = #0x0
    c.b(C::Al,0x2c060)?;
    // 0x2c05c: ldr r1, [pc, #0xfdc]        @ 0x1440 <entry+0x1440>
    c.literal(R::R1,0x2d040)?;
    // 0x2c060: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c064: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2c28
    c.bl(0x2ec94)?;
    // 0x2c068: ldr r1, [pc, #0x500]        @ 0x970 <entry+0x970>
    c.literal(R::R1,0x2c570)?;
    // 0x2c06c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c070: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2c1c
    c.bl(0x2ec94)?;
    // 0x2c074: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x2c078: b 0x2c5cc <entry+0x9cc>   @ imm = #0x54c
    c.b(C::Al,0x2c5cc)?;
    // 0x2c07c: ldr r1, [pc, #0xfe4]        @ 0x1468 <entry+0x1468>
    c.literal(R::R1,0x2d068)?;
    // 0x2c080: b 0x2c5b0 <entry+0x9b0>   @ imm = #0x528
    c.b(C::Al,0x2c5b0)?;
    // 0x2c084: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2c088: orr r0, r0, #2162688
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2162688)?)?;
    // 0x2c08c: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2c090: bne 0x2c0f4 <entry+0x4f4>   @ imm = #0x5c
    c.b(C::Ne,0x2c0f4)?;
    // 0x2c094: ldr r1, [pc, #0xff4]        @ 0x1490 <entry+0x1490>
    c.literal(R::R1,0x2d090)?;
    // 0x2c098: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c09c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2bf0
    c.bl(0x2ec94)?;
    // 0x2c0a0: ldr r1, [pc, #0xff8]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c0a4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c0a8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2be4
    c.bl(0x2ec94)?;
    // 0x2c0ac: ldr r1, [pc, #0xfe0]        @ 0x1494 <entry+0x1494>
    c.literal(R::R1,0x2d094)?;
    // 0x2c0b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c0b4: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2c0b8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2bd4
    c.bl(0x2ec94)?;
    // 0x2c0bc: ldr r1, [pc, #0xfdc]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c0c0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c0c4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2bc8
    c.bl(0x2ec94)?;
    // 0x2c0c8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2c0cc: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2c0d0: orr r0, r0, #2228224
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2228224)?)?;
    // 0x2c0d4: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x2c0d8: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x2c0dc: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2c0e0: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x2c0e4: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2c0e8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2c0ec: bl 0x2a878 <entry+0xffffffffffffec78> @ imm = #-0x187c
    c.bl(0x2a878)?;
    // 0x2c0f0: b 0x2b4ec <entry+0xfffffffffffff8ec> @ imm = #-0xc0c
    c.b(C::Al,0x2b4ec)?;
    // 0x2c0f4: ldr r1, [pc, #0xf9c]        @ 0x1498 <entry+0x1498>
    c.literal(R::R1,0x2d098)?;
    // 0x2c0f8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c0fc: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x2c100: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2b8c
    c.bl(0x2ec94)?;
    // 0x2c104: ldr r1, [pc, #0xf94]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c108: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c10c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2b80
    c.bl(0x2ec94)?;
    // 0x2c110: cmp r7, #2162688
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2162688)?)?;
    // 0x2c114: ble 0x2caec <entry+0xeec>   @ imm = #0x9d0
    c.b(C::Le,0x2caec)?;
    // 0x2c118: ldr r0, [pc, #0xf7c]        @ 0x149c <entry+0x149c>
    c.literal(R::R0,0x2d09c)?;
    // 0x2c11c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2c120: orr r2, r2, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,135168)?)?;
    // 0x2c124: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2c128: cmp r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,13)?)?;
    // 0x2c12c: bhi 0x2d044 <entry+0x1444>  @ imm = #0xf10
    c.b(C::Hi,0x2d044)?;
    // 0x2c130: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2c134: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x2c138: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2c13c)?;
    const CODEC_CONTROL_EVENT_TYPES:[u32;14]=[
        0x0002c6f4,0x0002c728,0x0002c754,0x0002c788,0x0002c7b8,
        0x0002c7e4,0x0002c818,0x0002c848,0x0002c878,0x0002c5cc,
        0x0002c8a4,0x0002c8d4,0x0002c900,0x0002c934,
    ];
    for v in CODEC_CONTROL_EVENT_TYPES {c.emit(v)?;}
    c.check(0x2c174)?;
    // 0x2c174: ldr r1, [pc, #0xf40]        @ 0x14bc <entry+0x14bc>
    c.literal(R::R1,0x2d0bc)?;
    // 0x2c178: b 0x2c5b0 <entry+0x9b0>   @ imm = #0x430
    c.b(C::Al,0x2c5b0)?;
    // 0x2c17c: ldr r1, [pc, #0xf34]        @ 0x14b8 <entry+0x14b8>
    c.literal(R::R1,0x2d0b8)?;
    // 0x2c180: b 0x2c5b0 <entry+0x9b0>   @ imm = #0x428
    c.b(C::Al,0x2c5b0)?;
    // 0x2c184: ldr r1, [pc, #0xf28]        @ 0x14b4 <entry+0x14b4>
    c.literal(R::R1,0x2d0b4)?;
    // 0x2c188: b 0x2c5b0 <entry+0x9b0>   @ imm = #0x420
    c.b(C::Al,0x2c5b0)?;
    // 0x2c18c: ldr r1, [pc, #0xf1c]        @ 0x14b0 <entry+0x14b0>
    c.literal(R::R1,0x2d0b0)?;
    // 0x2c190: b 0x2c5b0 <entry+0x9b0>   @ imm = #0x418
    c.b(C::Al,0x2c5b0)?;
    // 0x2c194: ldr r1, [pc, #0xf10]        @ 0x14ac <entry+0x14ac>
    c.literal(R::R1,0x2d0ac)?;
    // 0x2c198: b 0x2c5b0 <entry+0x9b0>   @ imm = #0x410
    c.b(C::Al,0x2c5b0)?;
    // 0x2c19c: ldr r1, [pc, #0xf78]        @ 0x151c <entry+0x151c>
    c.literal(R::R1,0x2d11c)?;
    // 0x2c1a0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c1a4: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x2c1a8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2ae4
    c.bl(0x2ec94)?;
    // 0x2c1ac: ldr r1, [pc, #0xeec]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c1b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c1b4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2ad8
    c.bl(0x2ec94)?;
    // 0x2c1b8: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x2c1bc: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x2c1c0: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2c1c4: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2c1c8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2c1cc: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2c1d0: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x2c1d4: str r6, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,28)?)?;
    // 0x2c1d8: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x2c1dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2c1e0: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2c1e4: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2c1e8: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2c1ec: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2c1f0: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x2c1f4: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2c1f8: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2c1fc: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x206c
    c.bl(0x2a198)?;
    // 0x2c200: ldr r1, [pc, #0xf64]        @ 0x156c <entry+0x156c>
    c.literal(R::R1,0x2d16c)?;
    // 0x2c204: ldr r2, [pc, #0xf4c]        @ 0x1558 <entry+0x1558>
    c.literal(R::R2,0x2d158)?;
    // 0x2c208: mov r3, #207
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,207)?)?;
    // 0x2c20c: orr r3, r3, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // 0x2c210: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2c214: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2a78
    c.bl(0x2ec94)?;
    // 0x2c218: ldr r1, [pc, #0xe80]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c21c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2c220: b 0x2bd9c <entry+0x19c>   @ imm = #-0x48c
    c.b(C::Al,0x2bd9c)?;
    // 0x2c224: ldr r1, [pc, #0xeec]        @ 0x1518 <entry+0x1518>
    c.literal(R::R1,0x2d118)?;
    // 0x2c228: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c22c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2a60
    c.bl(0x2ec94)?;
    // 0x2c230: ldr r1, [pc, #0xe68]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c234: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c238: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2a54
    c.bl(0x2ec94)?;
    // 0x2c23c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2c240: ldr r1, [pc, #0xf20]        @ 0x1568 <entry+0x1568>
    c.literal(R::R1,0x2d168)?;
    // 0x2c244: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2c248: add r0, r10, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,65536)?)?;
    // 0x2c24c: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x2c250: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x2c254: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2c258: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2c25c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2a30
    c.bl(0x2ec94)?;
    // 0x2c260: ldr r1, [pc, #0xe38]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c264: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2c268: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2a24
    c.bl(0x2ec94)?;
    // 0x2c26c: b 0x2cc7c <entry+0x107c>  @ imm = #0xa08
    c.b(C::Al,0x2cc7c)?;
    // 0x2c270: ldr r1, [pc, #0xe9c]        @ 0x1514 <entry+0x1514>
    c.literal(R::R1,0x2d114)?;
    // 0x2c274: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c278: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x2c27c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2a10
    c.bl(0x2ec94)?;
    // 0x2c280: ldr r1, [pc, #0xe18]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c284: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c288: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2a04
    c.bl(0x2ec94)?;
    // 0x2c28c: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x2c290: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2c294: bne 0x2c9d4 <entry+0xdd4>   @ imm = #0x738
    c.b(C::Ne,0x2c9d4)?;
    // 0x2c298: mov r6, r9
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // 0x2c29c: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x2c2a0: ldr r0, [r6, #0x10]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // 0x2c2a4: add r0, r1, r0, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    // 0x2c2a8: ldr r1, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x2c2ac: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2c2b0: bne 0x2cb50 <entry+0xf50>   @ imm = #0x898
    c.b(C::Ne,0x2cb50)?;
    // 0x2c2b4: bl 0x2f808 <entry+0x3c08>  @ imm = #0x354c
    c.bl(0x2f808)?;
    // 0x2c2b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2c2bc: beq 0x2cf34 <entry+0x1334>  @ imm = #0xc70
    c.b(C::Eq,0x2cf34)?;
    // 0x2c2c0: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x2c2c4: bl 0x2f96c <entry+0x3d6c>  @ imm = #0x36a0
    c.bl(0x2f96c)?;
    // 0x2c2c8: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0xde8
    c.b(C::Al,0x2b4e8)?;
    // 0x2c2cc: ldr r1, [pc, #0xe3c]        @ 0x1510 <entry+0x1510>
    c.literal(R::R1,0x2d110)?;
    // 0x2c2d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c2d4: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x2c2d8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x29b4
    c.bl(0x2ec94)?;
    // 0x2c2dc: ldr r1, [pc, #0xdbc]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c2e0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c2e4: bl 0x2ec94 <entry+0x3094>  @ imm = #0x29a8
    c.bl(0x2ec94)?;
    // 0x2c2e8: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x2c2ec: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x2c2f0: bne 0x2ca48 <entry+0xe48>   @ imm = #0x750
    c.b(C::Ne,0x2ca48)?;
    // 0x2c2f4: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x2c2f8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x2c2fc: bne 0x2cbcc <entry+0xfcc>   @ imm = #0x8c8
    c.b(C::Ne,0x2cbcc)?;
    // 0x2c300: bl 0x2f808 <entry+0x3c08>  @ imm = #0x3500
    c.bl(0x2f808)?;
    // 0x2c304: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2c308: beq 0x2cfb4 <entry+0x13b4>  @ imm = #0xca4
    c.b(C::Eq,0x2cfb4)?;
    // 0x2c30c: bl 0x2fa34 <entry+0x3e34>  @ imm = #0x3720
    c.bl(0x2fa34)?;
    // 0x2c310: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0xe30
    c.b(C::Al,0x2b4e8)?;
    // 0x2c314: ldr r1, [pc, #0xdf0]        @ 0x150c <entry+0x150c>
    c.literal(R::R1,0x2d10c)?;
    // 0x2c318: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c31c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2970
    c.bl(0x2ec94)?;
    // 0x2c320: ldr r1, [pc, #0xd78]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c324: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c328: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2964
    c.bl(0x2ec94)?;
    // 0x2c32c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2c330: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c334: bl 0x2aad8 <entry+0xffffffffffffeed8> @ imm = #-0x1864
    c.bl(0x2aad8)?;
    // 0x2c338: b 0x2b4e8 <entry+0xfffffffffffff8e8> @ imm = #-0xe58
    c.b(C::Al,0x2b4e8)?;
    // 0x2c33c: ldr r1, [pc, #0xdc4]        @ 0x1508 <entry+0x1508>
    c.literal(R::R1,0x2d108)?;
    // 0x2c340: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c344: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2948
    c.bl(0x2ec94)?;
    // 0x2c348: ldr r1, [pc, #0xd50]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c34c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c350: bl 0x2ec94 <entry+0x3094>  @ imm = #0x293c
    c.bl(0x2ec94)?;
    // 0x2c354: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,8)?)?;
    // 0x2c358: ldr r2, [pc, #0xdc4]        @ 0x1524 <entry+0x1524>
    c.literal(R::R2,0x2d124)?;
    // 0x2c35c: mov r1, #217
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,217)?)?;
    // 0x2c360: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x2c364: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2c368: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c36c: bl 0x17b0 <entry+0xfffffffffffd5bb0> @ imm = #-0x2abc4
    c.bl(0x17b0)?;
    // 0x2c370: ldr r8, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(9)?,8)?)?;
    // 0x2c374: add r0, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,16)?)?;
    // 0x2c378: mov r5, #244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,244)?)?;
    // 0x2c37c: add r7, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,16)?)?;
    // 0x2c380: add r0, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,12)?)?;
    // 0x2c384: add r9, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(6)?,4)?)?;
    // 0x2c388: orr r5, r5, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,1024)?)?;
    // 0x2c38c: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2c390: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // 0x2c394: ldr r1, [r10], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R1,R::new(10)?,4)?)?;
    // 0x2c398: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // 0x2c39c: bne 0x2c3dc <entry+0x7dc>   @ imm = #0x38
    c.b(C::Ne,0x2c3dc)?;
    // 0x2c3a0: ldr r1, [pc, #0xdd0]        @ 0x1578 <entry+0x1578>
    c.literal(R::R1,0x2d178)?;
    // 0x2c3a4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c3a8: str r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
    // 0x2c3ac: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x2c3b0: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2c3b4: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x2c3b8: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2c3bc: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x2c3c0: bl 0x471c <entry+0xfffffffffffd8b1c> @ imm = #-0x27cac
    c.bl(0x471c)?;
    // 0x2c3c4: mov r0, #144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,144)?)?;
    // 0x2c3c8: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2c3cc: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2c3d0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2c3d4: bl 0x2a878 <entry+0xffffffffffffec78> @ imm = #-0x1b64
    c.bl(0x2a878)?;
    // 0x2c3d8: b 0x2c418 <entry+0x818>   @ imm = #0x38
    c.b(C::Al,0x2c418)?;
    // 0x2c3dc: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x2c3e0: ldr r1, [pc, #0xd88]        @ 0x1570 <entry+0x1570>
    c.literal(R::R1,0x2d170)?;
    // 0x2c3e4: ldr r2, [pc, #0xd88]        @ 0x1574 <entry+0x1574>
    c.literal(R::R2,0x2d174)?;
    // 0x2c3e8: mov r3, #243
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,243)?)?;
    // 0x2c3ec: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x2c3f0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2c3f4: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x2c3f8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2894
    c.bl(0x2ec94)?;
    // 0x2c3fc: ldr r1, [pc, #0xc9c]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c400: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x2c404: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2888
    c.bl(0x2ec94)?;
    // 0x2c408: ldr r2, [pc, #0xd14]        @ 0x1524 <entry+0x1524>
    c.literal(R::R2,0x2d124)?;
    // 0x2c40c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2c410: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x2c414: bl 0x17b0 <entry+0xfffffffffffd5bb0> @ imm = #-0x2ac6c
    c.bl(0x17b0)?;
    // 0x2c418: subs r8, r8, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::new(8)?,R::new(8)?,1,true)?)?;
    // 0x2c41c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x2c420: bne 0x2c394 <entry+0x794>   @ imm = #-0x94
    c.b(C::Ne,0x2c394)?;
    // 0x2c424: b 0x2b4ec <entry+0xfffffffffffff8ec> @ imm = #-0xf40
    c.b(C::Al,0x2b4ec)?;
    // 0x2c428: ldr r1, [pc, #0xcd4]        @ 0x1504 <entry+0x1504>
    c.literal(R::R1,0x2d104)?;
    // 0x2c42c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c430: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x2c434: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2858
    c.bl(0x2ec94)?;
    // 0x2c438: ldr r1, [pc, #0xc60]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c43c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c440: bl 0x2ec94 <entry+0x3094>  @ imm = #0x284c
    c.bl(0x2ec94)?;
    // 0x2c444: bl 0xa908 <entry+0xfffffffffffded08> @ imm = #-0x21b44
    c.bl(0xa908)?;
    // 0x2c448: ldr r1, [pc, #0xd34]        @ 0x1584 <entry+0x1584>
    c.literal(R::R1,0x2d184)?;
    // 0x2c44c: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x2c450: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c454: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2838
    c.bl(0x2ec94)?;
    // 0x2c458: ldr r1, [pc, #0xc40]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c45c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c460: bl 0x2ec94 <entry+0x3094>  @ imm = #0x282c
    c.bl(0x2ec94)?;
    // 0x2c464: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2c468: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x2c46c: ldr r1, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x2c470: cmp r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,12)?)?;
    // 0x2c474: bne 0x2ca6c <entry+0xe6c>   @ imm = #0x5f0
    c.b(C::Ne,0x2ca6c)?;
    // 0x2c478: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,8)?)?;
    // 0x2c47c: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2c480: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2c484: bl 0x2d2b4 <entry+0x16b4>  @ imm = #0xe28
    c.bl(0x2d2b4)?;
    // 0x2c488: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2c48c: beq 0x2cc94 <entry+0x1094>  @ imm = #0x800
    c.b(C::Eq,0x2cc94)?;
    // 0x2c490: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x2c494: add r8, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x2c498: ldr r10, [r8, #0x30]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(10)?,R::new(8)?,48)?)?;
    // 0x2c49c: ldr r0, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,24)?)?;
    // 0x2c4a0: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x2c4a4: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,16)?)?;
    // 0x2c4a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2c4ac: beq 0x2c4d4 <entry+0x8d4>   @ imm = #0x20
    c.b(C::Eq,0x2c4d4)?;
    // 0x2c4b0: ldr r2, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,0)?)?;
    // 0x2c4b4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2c4b8: cmp r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // 0x2c4bc: movhs r2, #32
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x2c4c0: bl 0x471c <entry+0xfffffffffffd8b1c> @ imm = #-0x27dac
    c.bl(0x471c)?;
    // 0x2c4c4: ldr r0, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,24)?)?;
    // 0x2c4c8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2c4cc: bic r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2c4d0: str r0, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,24)?)?;
    // 0x2c4d4: ldm r10, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(10)?,R::R0.bit()|R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2c4d8: ldr r1, [r1, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,60)?)?;
    // 0x2c4dc: blx r1
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    // 0x2c4e0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2c4e4: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x2c4e8: bne 0x2cca0 <entry+0x10a0>  @ imm = #0x7b0
    c.b(C::Ne,0x2cca0)?;
    // 0x2c4ec: add r5, r10, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(10)?,16)?)?;
    // 0x2c4f0: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2c4f4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2c4f8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2c4fc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2c500: beq 0x2c50c <entry+0x90c>   @ imm = #0x4
    c.b(C::Eq,0x2c50c)?;
    // 0x2c504: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2c508: bl 0x3040 <entry+0xfffffffffffd7440> @ imm = #-0x294d0
    c.bl(0x3040)?;
    // 0x2c50c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x2c510: str r6, [r5], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(6)?,R::new(5)?,4)?)?;
    // 0x2c514: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x2c518: bne 0x2c4f8 <entry+0x8f8>   @ imm = #-0x28
    c.b(C::Ne,0x2c4f8)?;
    // 0x2c51c: add r5, r10, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(10)?,28)?)?;
    // 0x2c520: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2c524: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2c528: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2c52c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2c530: beq 0x2c53c <entry+0x93c>   @ imm = #0x4
    c.b(C::Eq,0x2c53c)?;
    // 0x2c534: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2c538: bl 0x3040 <entry+0xfffffffffffd7440> @ imm = #-0x29500
    c.bl(0x3040)?;
    // 0x2c53c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x2c540: str r6, [r5], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(6)?,R::new(5)?,4)?)?;
    // 0x2c544: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x2c548: bne 0x2c528 <entry+0x928>   @ imm = #-0x28
    c.b(C::Ne,0x2c528)?;
    // 0x2c54c: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x2c550: bl 0x2f1c <entry+0xfffffffffffd731c> @ imm = #-0x2963c
    c.bl(0x2f1c)?;
    // 0x2c554: mov r6, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,6)?)?;
    // 0x2c558: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2c55c: orr r6, r6, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,131072)?)?;
    // 0x2c560: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x2c564: b 0x2cca0 <entry+0x10a0>  @ imm = #0x734
    c.b(C::Al,0x2cca0)?;
    c.check(0x2c568)?;
    const CODEC_DIAGNOSTIC_FIELDS:[u32;3]=[
        0x0400ffff,0x000e8d1c,0x000e2788,
    ];
    for v in CODEC_DIAGNOSTIC_FIELDS {c.emit(v)?;}
    c.check(0x2c574)?;
    // 0x2c574: ldr r1, [pc, #0xb84]        @ 0x1500 <entry+0x1500>
    c.literal(R::R1,0x2d100)?;
    // 0x2c578: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c57c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2710
    c.bl(0x2ec94)?;
    // 0x2c580: ldr r1, [pc, #0xb18]        @ 0x14a0 <entry+0x14a0>
    c.literal(R::R1,0x2d0a0)?;
    // 0x2c584: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2c588: bl 0x2ec94 <entry+0x3094>  @ imm = #0x2704
    c.bl(0x2ec94)?;
    // 0x2c58c: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // 0x2c590: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2c594: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x2c598: bne 0x2b4ec <entry+0xfffffffffffff8ec> @ imm = #-0x10b4
    c.b(C::Ne,0x2b4ec)?;
    // 0x2c59c: ldr r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,20)?)?;
    // 0x2c5a0: ldr r0, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,112)?)?;
    // 0x2c5a4: b 0x2b640 <entry+0xfffffffffffffa40> @ imm = #-0xf6c
    c.b(C::Al,0x2b640)?;
    c.check(0x2c5a8)?;
    const CODEC_MESSAGE_POINTER:[u32;1]=[
        0x000e8cfd,
    ];
    for v in CODEC_MESSAGE_POINTER {c.emit(v)?;}
    c.check(0x2c5ac)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;dispatch_codec_buffer_classes(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_class_handler_literals_are_distinct(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let w=|v:usize|u32::from_le_bytes(c.bytes[v-START..v-START+4].try_into().unwrap());
        assert_eq!(w(0x2c13c),0x0002c6f4);
        assert_eq!(w(0x2c170),0x0002c934);
        assert_eq!(w(0x2c568),0x0400ffff);
        assert_eq!(w(0x2c570),0x000e2788);
        assert_eq!(w(0x2c5a8),0x000e8cfd);
    }
}
