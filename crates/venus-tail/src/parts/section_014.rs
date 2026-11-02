
const CODEC_REGISTER_PACKET_LITERALS:[u32;9]=[
    0x0022_1008, // codec capability mask/ID
    0x0101_0200, // packed buffer capability data
    0x000f_8519, // diagnostic string
    0x7f10_0002, // opcode/tag data
    0x000f_8551, // diagnostic
    0x000e_2788, // common logger
    0x000f_6864,
    0x000f_859f,
    0x000f_85d7,
];

/// Build codec command metadata, validate input dimensions, invoke release callbacks and publish the remaining frame-header fields.
/// Runtime slice 0xac750..0xac9f0, with 3 classified trailing data words.
fn serialize_codec_resource_release(c:&mut Code)->Result<(),String>{
    c.check(0xac750)?;
    // ARM 0xac750: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xac754: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xac758: sub sp, sp, #68
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,68)?)?;
    // ARM 0xac75c: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xac760: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // ARM 0xac764: mov r0, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,156)?)?;
    // ARM 0xac768: ldr r6, [r5, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,2444)?)?;
    // ARM 0xac76c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // ARM 0xac770: bl 0x2f8d0 <entry+0xfffffffffff83180>
    c.b(C::Al,0x2f8d0,true)?;
    // ARM 0xac774: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xac778: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // ARM 0xac77c: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // ARM 0xac780: movne r1, #24
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,24)?)?;
    // ARM 0xac784: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xac788: cmp r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,256)?)?;
    // ARM 0xac78c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // ARM 0xac790: bls 0xac7a8 <entry+0x58>
    c.b(C::Ls,0xac7a8,false)?;
    // ARM 0xac794: ldr r2, [pc, #0x248]
    c.literal(R::R2,0xac9e4)?;
    // ARM 0xac798: mov r1, #50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,50)?)?;
    // ARM 0xac79c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xac7a0: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // ARM 0xac7a4: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xac7a8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xac7ac: mov r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // ARM 0xac7b0: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // ARM 0xac7b4: orr r0, r0, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,135168)?)?;
    // ARM 0xac7b8: orr r9, r9, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,16777216)?)?;
    // ARM 0xac7bc: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // ARM 0xac7c0: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // ARM 0xac7c4: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // ARM 0xac7c8: add r0, r9, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,2)?)?;
    // ARM 0xac7cc: str r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // ARM 0xac7d0: str r8, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,20)?)?;
    // ARM 0xac7d4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM 0xac7d8: ldrb r1, [r6, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,110)?)?;
    // ARM 0xac7dc: ldr r7, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,76)?)?;
    // ARM 0xac7e0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xac7e4: bne 0xac7f4 <entry+0xa4>
    c.b(C::Ne,0xac7f4,false)?;
    // ARM 0xac7e8: ldrb r0, [r6, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,111)?)?;
    // ARM 0xac7ec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xac7f0: movne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM 0xac7f4: str r0, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,76)?)?;
    // ARM 0xac7f8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xac7fc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xac800: bl 0xac9f0 <entry+0x2a0>
    c.b(C::Al,0xac9f0,true)?;
    // ARM 0xac804: ldr r1, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,76)?)?;
    // ARM 0xac808: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // ARM 0xac80c: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // ARM 0xac810: cmp r7, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xac814: movne r10, r8
    c.emit(arm32::mov_register(C::Ne,R::new(10)?,R::new(8)?))?;
    // ARM 0xac818: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xac81c: beq 0xac828 <entry+0xd8>
    c.b(C::Eq,0xac828,false)?;
    // ARM 0xac820: cmp r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,1)?)?;
    // ARM 0xac824: beq 0xac8b8 <entry+0x168>
    c.b(C::Eq,0xac8b8,false)?;
    // ARM 0xac828: add r0, r5, #948
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,948)?)?;
    // ARM 0xac82c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xac830: ldr r0, [r5, #0x48c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1164)?)?;
    // ARM 0xac834: bl 0xd9e10 <entry+0x2d6c0>
    c.b(C::Al,0xd9e10,true)?;
    // ARM 0xac838: ldr r0, [r5, #0x488]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1160)?)?;
    // ARM 0xac83c: bl 0xd9e10 <entry+0x2d6c0>
    c.b(C::Al,0xd9e10,true)?;
    // ARM 0xac840: add r7, r5, #1152
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(5)?,1152)?)?;
    // ARM 0xac844: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // ARM 0xac848: ldr r0, [r7, #-0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,-8)?)?;
    // ARM 0xac84c: bl 0xd9e10 <entry+0x2d6c0>
    c.b(C::Al,0xd9e10,true)?;
    // ARM 0xac850: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // ARM 0xac854: bl 0xd9e10 <entry+0x2d6c0>
    c.b(C::Al,0xd9e10,true)?;
    // ARM 0xac858: add r8, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // ARM 0xac85c: add r7, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,4)?)?;
    // ARM 0xac860: cmp r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // ARM 0xac864: bne 0xac848 <entry+0xf8>
    c.b(C::Ne,0xac848,false)?;
    // ARM 0xac868: ldr r0, [r5, #0x494]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1172)?)?;
    // ARM 0xac86c: bl 0x7ecc0 <entry+0xfffffffffffd2570>
    c.b(C::Al,0x7ecc0,true)?;
    // ARM 0xac870: ldr r0, [r5, #0x49c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1180)?)?;
    // ARM 0xac874: bl 0x7ecc0 <entry+0xfffffffffffd2570>
    c.b(C::Al,0x7ecc0,true)?;
    // ARM 0xac878: ldr r3, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,32)?)?;
    // ARM 0xac87c: cmp r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,2)?)?;
    // ARM 0xac880: bne 0xac9b8 <entry+0x268>
    c.b(C::Ne,0xac9b8,false)?;
    // ARM 0xac884: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xac888: beq 0xac9b8 <entry+0x268>
    c.b(C::Eq,0xac9b8,false)?;
    // ARM 0xac88c: ldr r2, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,0)?)?;
    // ARM 0xac890: add r0, r5, #728
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,728)?)?;
    // ARM 0xac894: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xac898: bl 0xdb350 <entry+0x2ec00>
    c.b(C::Al,0xdb350,true)?;
    // ARM 0xac89c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xac8a0: beq 0xac8b8 <entry+0x168>
    c.b(C::Eq,0xac8b8,false)?;
    // ARM 0xac8a4: ldr r2, [pc, #0x138]
    c.literal(R::R2,0xac9e4)?;
    // ARM 0xac8a8: mov r1, #95
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,95)?)?;
    // ARM 0xac8ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xac8b0: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // ARM 0xac8b4: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xac8b8: ldr r3, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,32)?)?;
    // ARM 0xac8bc: add r0, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,16)?)?;
    // ARM 0xac8c0: str r9, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R0,0)?)?;
    // ARM 0xac8c4: ldr r1, [pc, #0x11c]
    c.literal(R::R1,0xac9e8)?;
    // ARM 0xac8c8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM 0xac8cc: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // ARM 0xac8d0: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xac8d4: ldr r1, [pc, #0x110]
    c.literal(R::R1,0xac9ec)?;
    // ARM 0xac8d8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // ARM 0xac8dc: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xac8e0: sub r1, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,32)?)?;
    // ARM 0xac8e4: sub r2, r11, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,36)?)?;
    // ARM 0xac8e8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xac8ec: bl 0xaf440 <entry+0x2cf0>
    c.b(C::Al,0xaf440,true)?;
    // ARM 0xac8f0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // ARM 0xac8f4: str r0, [r5, #0x2ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,684)?)?;
    // ARM 0xac8f8: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // ARM 0xac8fc: str r0, [r5, #0x2b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,688)?)?;
    // ARM 0xac900: ldrh r0, [r6, #98]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(6)?,98)?)?;
    // ARM 0xac904: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM 0xac908: ldrh r1, [r6, #100]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(6)?,100)?)?;
    // ARM 0xac90c: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // ARM 0xac910: ldrh r2, [r6, #102]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(6)?,102)?)?;
    // ARM 0xac914: lsl r3, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsl,1)?)?;
    // ARM 0xac918: str r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // ARM 0xac91c: ldrh r3, [r6, #106]
    c.emit(arm32::load_halfword(C::Al,R::new(3)?,R::new(6)?,106)?)?;
    // ARM 0xac920: lsl r7, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(3)?,Shift::Lsl,1)?)?;
    // ARM 0xac924: str r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // ARM 0xac928: ldrh r7, [r6, #104]
    c.emit(arm32::load_halfword(C::Al,R::new(7)?,R::new(6)?,104)?)?;
    // ARM 0xac92c: add r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xac930: sub r0, r0, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,1,false)?)?;
    // ARM 0xac934: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // ARM 0xac938: ldrh r0, [r6, #108]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(6)?,108)?)?;
    // ARM 0xac93c: add r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xac940: sub r0, r1, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,1,false)?)?;
    // ARM 0xac944: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // ARM 0xac948: ldrb r0, [r6, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,110)?)?;
    // ARM 0xac94c: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM 0xac950: ldrb r0, [r6, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,111)?)?;
    // ARM 0xac954: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // ARM 0xac958: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xac95c: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // ARM 0xac960: ldr r0, [r6, #0x6e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1764)?)?;
    // ARM 0xac964: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // ARM 0xac968: ldrb r0, [r6, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,123)?)?;
    // ARM 0xac96c: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // ARM 0xac970: ldrb r0, [r6, #0x83]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,131)?)?;
    // ARM 0xac974: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // ARM 0xac978: bl 0x2f8d0 <entry+0xfffffffffff83180>
    c.b(C::Al,0x2f8d0,true)?;
    // ARM 0xac97c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xac980: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // ARM 0xac984: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // ARM 0xac988: ldrbeq r0, [r6, #0x7b]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(6)?,123)?)?;
    // ARM 0xac98c: ldrne r0, [r5, #0xc48]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,3144)?)?;
    // ARM 0xac990: str r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // ARM 0xac994: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // ARM 0xac998: add r0, r4, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,24)?)?;
    // ARM 0xac99c: bl 0xe13b0 <entry+0x34c60>
    c.b(C::Al,0xe13b0,true)?;
    // ARM 0xac9a0: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // ARM 0xac9a4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // ARM 0xac9a8: ldr r0, [r5, #0x360]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,864)?)?;
    // ARM 0xac9ac: bl 0x2a878 <entry+0xfffffffffff7e128>
    c.b(C::Al,0x2a878,true)?;
    // ARM 0xac9b0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xac9b4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xac9b8: add r0, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,16)?)?;
    // ARM 0xac9bc: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // ARM 0xac9c0: bne 0xac8c0 <entry+0x170>
    c.b(C::Ne,0xac8c0,false)?;
    // ARM 0xac9c4: add r1, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,1)?)?;
    // ARM 0xac9c8: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // ARM 0xac9cc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xac9d0: str r0, [r5, #0x144]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,324)?)?;
    // ARM 0xac9d4: str r0, [r5, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,332)?)?;
    // ARM 0xac9d8: str r0, [r5, #0x21c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,540)?)?;
    // ARM 0xac9dc: str r0, [r5, #0x214]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,532)?)?;
    // ARM 0xac9e0: b 0xac8c4 <entry+0x174>
    c.b(C::Al,0xac8c4,false)?;
    c.check(0xac9e4)?;
    // ARM 0xac9e4: literal or constant data (NOT executable)
    c.emit(0x000f8519)?;
    // ARM 0xac9e8: literal or constant data (NOT executable)
    c.emit(0x000f8615)?;
    // ARM 0xac9ec: literal or constant data (NOT executable)
    c.emit(0x000e2788)?;
    c.check(0xac9f0)
}
