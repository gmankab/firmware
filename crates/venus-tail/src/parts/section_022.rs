
/// Build codec format helpers: host plane layout setup, a bounded codec
/// class 8-way switch, pixel-format reverse lookup, and allocator callback.
/// Distinct data pools hold handler addresses, scaling factors and diagnostic
/// metadata. No original executable words are copied into source arrays.
fn create_codec_format_accessors(c:&mut Code)->Result<(),String>{
    c.check(0xaf290)?;
    // ARM 0xaf290: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xaf294: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // ARM 0xaf298: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // ARM 0xaf29c: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // ARM 0xaf2a0: mov r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8192)?)?;
    // ARM 0xaf2a4: ldr r0, [r1, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,2444)?)?;
    // ARM 0xaf2a8: ldrh r2, [r0, #98]
    c.emit(arm32::load_halfword(C::Al,R::R2,R::R0,98)?)?;
    // ARM 0xaf2ac: str r2, [r1, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,220)?)?;
    // ARM 0xaf2b0: add r2, r1, #728
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,728)?)?;
    // ARM 0xaf2b4: ldrh r0, [r0, #100]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::R0,100)?)?;
    // ARM 0xaf2b8: str r0, [r1, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,224)?)?;
    // ARM 0xaf2bc: add r0, r1, #4000
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,4000)?)?;
    // ARM 0xaf2c0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xaf2c4: add r0, r1, #924
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,924)?)?;
    // ARM 0xaf2c8: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // ARM 0xaf2cc: bl 0xe0960 <entry+0x34210>
    c.b(C::Al,0xe0960,true)?;
    // ARM 0xaf2d0: mov sp, r11
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // ARM 0xaf2d4: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xaf2d8: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xaf2dc: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // ARM 0xaf2e0: ldr lr, [r1, #0x464]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R1,1124)?)?;
    // ARM 0xaf2e4: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // ARM 0xaf2e8: beq 0xaf324 <entry+0x2bd4>
    c.b(C::Eq,0xaf324,false)?;
    // ARM 0xaf2ec: add r1, r1, #84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,84)?)?;
    // ARM 0xaf2f0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // ARM 0xaf2f4: mov r12, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,84)?)?;
    // ARM 0xaf2f8: add r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // ARM 0xaf2fc: add r2, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaf300: ldrsb r2, [r2]
    c.emit(arm32::load_signed_byte(C::Al,R::R2,R::R2,0)?)?;
    // ARM 0xaf304: smlabb r2, r2, r12, r0
    c.emit(arm32::signed_multiply_accumulate_bottom_halves(C::Al,R::R2,R::R2,R::R12,R::R0)?)?;
    // ARM 0xaf308: ldrb r2, [r2, #0x51]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,81)?)?;
    // ARM 0xaf30c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xaf310: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xaf314: popeq {r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xaf318: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // ARM 0xaf31c: cmp r3, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // ARM 0xaf320: blo 0xaf2fc <entry+0x2bac>
    c.b(C::Cc,0xaf2fc,false)?;
    // ARM 0xaf324: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xaf328: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xaf32c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xaf330: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // ARM 0xaf334: mov r3, #77
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,77)?)?;
    // ARM 0xaf338: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // ARM 0xaf33c: moveq r3, #88
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,88)?)?;
    // ARM 0xaf340: cmp r2, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,127)?)?;
    // ARM 0xaf344: str r3, [r0, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,88)?)?;
    // ARM 0xaf348: bgt 0xaf398 <entry+0x2c48>
    c.b(C::Gt,0xaf398,false)?;
    // ARM 0xaf34c: cmp r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,15)?)?;
    // ARM 0xaf350: bgt 0xaf3c0 <entry+0x2c70>
    c.b(C::Gt,0xaf3c0,false)?;
    // ARM 0xaf354: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    // ARM 0xaf358: cmp r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,7)?)?;
    // ARM 0xaf35c: bhi 0xaf3fc <entry+0x2cac>
    c.b(C::Hi,0xaf3fc,false)?;
    // ARM 0xaf360: lsl r2, r2, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,2)?)?;
    // ARM 0xaf364: add r3, pc, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::PC,4)?)?;
    // ARM 0xaf368: mov r1, #30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,30)?)?;
    // ARM 0xaf36c: ldr pc, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    c.check(0xaf370)?;
    const CODEC_FORMAT_JUMPS:[u32;8]=[
        0x000af438, 0x000af390, 0x000af3fc, 0x000af404, 0x000af3fc, 0x000af3fc, 0x000af3fc, 0x000af40c,
    ];
    for target in CODEC_FORMAT_JUMPS {c.emit(target)?;}
    c.check(0xaf390)?;
    // ARM 0xaf390: mov r1, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,60)?)?;
    // ARM 0xaf394: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf398: cmp r2, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1024)?)?;
    // ARM 0xaf39c: bge 0xaf3e0 <entry+0x2c90>
    c.b(C::Ge,0xaf3e0,false)?;
    // ARM 0xaf3a0: cmp r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,128)?)?;
    // ARM 0xaf3a4: beq 0xaf414 <entry+0x2cc4>
    c.b(C::Eq,0xaf414,false)?;
    // ARM 0xaf3a8: cmp r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,256)?)?;
    // ARM 0xaf3ac: beq 0xaf3fc <entry+0x2cac>
    c.b(C::Eq,0xaf3fc,false)?;
    // ARM 0xaf3b0: cmp r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,512)?)?;
    // ARM 0xaf3b4: bne 0xaf3fc <entry+0x2cac>
    c.b(C::Ne,0xaf3fc,false)?;
    // ARM 0xaf3b8: mov r1, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,156)?)?;
    // ARM 0xaf3bc: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf3c0: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // ARM 0xaf3c4: beq 0xaf41c <entry+0x2ccc>
    c.b(C::Eq,0xaf41c,false)?;
    // ARM 0xaf3c8: cmp r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // ARM 0xaf3cc: beq 0xaf424 <entry+0x2cd4>
    c.b(C::Eq,0xaf424,false)?;
    // ARM 0xaf3d0: cmp r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,64)?)?;
    // ARM 0xaf3d4: bne 0xaf3fc <entry+0x2cac>
    c.b(C::Ne,0xaf3fc,false)?;
    // ARM 0xaf3d8: mov r1, #123
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,123)?)?;
    // ARM 0xaf3dc: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf3e0: beq 0xaf42c <entry+0x2cdc>
    c.b(C::Eq,0xaf42c,false)?;
    // ARM 0xaf3e4: cmp r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2048)?)?;
    // ARM 0xaf3e8: beq 0xaf434 <entry+0x2ce4>
    c.b(C::Eq,0xaf434,false)?;
    // ARM 0xaf3ec: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // ARM 0xaf3f0: bne 0xaf3fc <entry+0x2cac>
    c.b(C::Ne,0xaf3fc,false)?;
    // ARM 0xaf3f4: mov r1, #186
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,186)?)?;
    // ARM 0xaf3f8: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf3fc: mov r1, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,153)?)?;
    // ARM 0xaf400: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf404: mov r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,63)?)?;
    // ARM 0xaf408: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf40c: mov r1, #90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,90)?)?;
    // ARM 0xaf410: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf414: mov r1, #150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,150)?)?;
    // ARM 0xaf418: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf41c: mov r1, #93
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,93)?)?;
    // ARM 0xaf420: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf424: mov r1, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,120)?)?;
    // ARM 0xaf428: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf42c: mov r1, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,180)?)?;
    // ARM 0xaf430: b 0xaf438 <entry+0x2ce8>
    c.b(C::Al,0xaf438,false)?;
    // ARM 0xaf434: mov r1, #183
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,183)?)?;
    // ARM 0xaf438: strb r1, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,3)?)?;
    // ARM 0xaf43c: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // ARM 0xaf440: ldr r12, [r0, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,88)?)?;
    // ARM 0xaf444: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM 0xaf448: cmp r12, #88
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,88)?)?;
    // ARM 0xaf44c: moveq r3, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // ARM 0xaf450: cmp r12, #77
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,77)?)?;
    // ARM 0xaf454: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // ARM 0xaf458: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // ARM 0xaf45c: ldrb r0, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,3)?)?;
    // ARM 0xaf460: cmp r0, #122
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,122)?)?;
    // ARM 0xaf464: ble 0xaf490 <entry+0x2d40>
    c.b(C::Le,0xaf490,false)?;
    // ARM 0xaf468: cmp r0, #155
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,155)?)?;
    // ARM 0xaf46c: ble 0xaf4bc <entry+0x2d6c>
    c.b(C::Le,0xaf4bc,false)?;
    // ARM 0xaf470: cmp r0, #180
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,180)?)?;
    // ARM 0xaf474: bgt 0xaf4f8 <entry+0x2da8>
    c.b(C::Gt,0xaf4f8,false)?;
    // ARM 0xaf478: cmp r0, #156
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,156)?)?;
    // ARM 0xaf47c: beq 0xaf510 <entry+0x2dc0>
    c.b(C::Eq,0xaf510,false)?;
    // ARM 0xaf480: cmp r0, #180
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,180)?)?;
    // ARM 0xaf484: bne 0xaf4d0 <entry+0x2d80>
    c.b(C::Ne,0xaf4d0,false)?;
    // ARM 0xaf488: mov r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1024)?)?;
    // ARM 0xaf48c: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf490: cmp r0, #89
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,89)?)?;
    // ARM 0xaf494: bgt 0xaf4d8 <entry+0x2d88>
    c.b(C::Gt,0xaf4d8,false)?;
    // ARM 0xaf498: cmp r0, #30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,30)?)?;
    // ARM 0xaf49c: beq 0xaf518 <entry+0x2dc8>
    c.b(C::Eq,0xaf518,false)?;
    // ARM 0xaf4a0: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM 0xaf4a4: cmp r0, #60
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,60)?)?;
    // ARM 0xaf4a8: beq 0xaf544 <entry+0x2df4>
    c.b(C::Eq,0xaf544,false)?;
    // ARM 0xaf4ac: cmp r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,63)?)?;
    // ARM 0xaf4b0: bne 0xaf4d0 <entry+0x2d80>
    c.b(C::Ne,0xaf4d0,false)?;
    // ARM 0xaf4b4: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // ARM 0xaf4b8: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf4bc: cmp r0, #123
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,123)?)?;
    // ARM 0xaf4c0: beq 0xaf520 <entry+0x2dd0>
    c.b(C::Eq,0xaf520,false)?;
    // ARM 0xaf4c4: cmp r0, #150
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,150)?)?;
    // ARM 0xaf4c8: beq 0xaf528 <entry+0x2dd8>
    c.b(C::Eq,0xaf528,false)?;
    // ARM 0xaf4cc: cmp r0, #153
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,153)?)?;
    // ARM 0xaf4d0: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // ARM 0xaf4d4: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf4d8: cmp r0, #90
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,90)?)?;
    // ARM 0xaf4dc: beq 0xaf530 <entry+0x2de0>
    c.b(C::Eq,0xaf530,false)?;
    // ARM 0xaf4e0: cmp r0, #93
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,93)?)?;
    // ARM 0xaf4e4: beq 0xaf538 <entry+0x2de8>
    c.b(C::Eq,0xaf538,false)?;
    // ARM 0xaf4e8: cmp r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,120)?)?;
    // ARM 0xaf4ec: bne 0xaf4d0 <entry+0x2d80>
    c.b(C::Ne,0xaf4d0,false)?;
    // ARM 0xaf4f0: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // ARM 0xaf4f4: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf4f8: cmp r0, #181
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,181)?)?;
    // ARM 0xaf4fc: beq 0xaf540 <entry+0x2df0>
    c.b(C::Eq,0xaf540,false)?;
    // ARM 0xaf500: cmp r0, #182
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,182)?)?;
    // ARM 0xaf504: bne 0xaf4d0 <entry+0x2d80>
    c.b(C::Ne,0xaf4d0,false)?;
    // ARM 0xaf508: mov r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4096)?)?;
    // ARM 0xaf50c: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf510: mov r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,512)?)?;
    // ARM 0xaf514: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf518: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xaf51c: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf520: mov r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // ARM 0xaf524: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf528: mov r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // ARM 0xaf52c: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf530: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // ARM 0xaf534: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf538: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // ARM 0xaf53c: b 0xaf544 <entry+0x2df4>
    c.b(C::Al,0xaf544,false)?;
    // ARM 0xaf540: mov r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2048)?)?;
    // ARM 0xaf544: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xaf548: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // ARM 0xaf54c: mov r2, #8912896
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8912896)?)?;
    // ARM 0xaf550: cmp r1, #30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,30)?)?;
    // ARM 0xaf554: blo 0xaf57c <entry+0x2e2c>
    c.b(C::Cc,0xaf57c,false)?;
    // ARM 0xaf558: cmp r1, #186
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,186)?)?;
    // ARM 0xaf55c: bhi 0xaf57c <entry+0x2e2c>
    c.b(C::Hi,0xaf57c,false)?;
    // ARM 0xaf560: ldr r2, [pc, #0x50]
    c.literal(R::R2,0xaf5b8)?;
    // ARM 0xaf564: umull r3, r12, r1, r2
    c.emit(arm32::multiply_long(C::Al,false,R::new(3)?,R::R12,R::R1,R::R2)?)?;
    // ARM 0xaf568: ldr r2, [pc, #0x4c]
    c.literal(R::R2,0xaf5bc)?;
    // ARM 0xaf56c: mvn r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,3)?)?;
    // ARM 0xaf570: and r1, r1, r12, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::R12,Shift::Lsl,1,false)?)?;
    // ARM 0xaf574: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xaf578: ldr r2, [r1, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,-40)?)?;
    // ARM 0xaf57c: lsr r3, r2, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsr,2)?)?;
    // ARM 0xaf580: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // ARM 0xaf584: cmp r3, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaf588: bhs 0xaf5b0 <entry+0x2e60>
    c.b(C::Cs,0xaf5b0,false)?;
    // ARM 0xaf58c: lsr r3, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsr,1)?)?;
    // ARM 0xaf590: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // ARM 0xaf594: cmp r3, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaf598: bhs 0xaf5b0 <entry+0x2e60>
    c.b(C::Cs,0xaf5b0,false)?;
    // ARM 0xaf59c: add r1, r2, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    // ARM 0xaf5a0: lsr r2, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsr,2)?)?;
    // ARM 0xaf5a4: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // ARM 0xaf5a8: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xaf5ac: movhs r1, #8
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,8)?)?;
    // ARM 0xaf5b0: and r0, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R1,255)?)?;
    // ARM 0xaf5b4: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xaf5b8)?;
    const CODEC_FORMAT_LOOKUP:[u32;2]=[
        0xaaaaaaab, 0x000f8768,
    ];
    for value in CODEC_FORMAT_LOOKUP {c.emit(value)?;}
    c.check(0xaf5c0)?;
    // ARM 0xaf5c0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xaf5c4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM 0xaf5c8: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // ARM 0xaf5cc: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // ARM 0xaf5d0: ldrh r0, [r4, #98]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(4)?,98)?)?;
    // ARM 0xaf5d4: ldrh r1, [r4, #100]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(4)?,100)?)?;
    // ARM 0xaf5d8: mul r5, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::R1,R::R0)?)?;
    // ARM 0xaf5dc: ldrb r1, [r4, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,3)?)?;
    // ARM 0xaf5e0: cmp r1, #30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,30)?)?;
    // ARM 0xaf5e4: blo 0xaf5f0 <entry+0x2ea0>
    c.b(C::Cc,0xaf5f0,false)?;
    // ARM 0xaf5e8: cmp r1, #187
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,187)?)?;
    // ARM 0xaf5ec: blo 0xaf5f8 <entry+0x2ea8>
    c.b(C::Cc,0xaf5f8,false)?;
    // ARM 0xaf5f0: mov r1, #186
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,186)?)?;
    // ARM 0xaf5f4: strb r1, [r4, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,3)?)?;
    // ARM 0xaf5f8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xaf5fc: bl 0xaf54c <entry+0x2dfc>
    c.b(C::Al,0xaf54c,true)?;
    // ARM 0xaf600: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // ARM 0xaf604: cmp r5, #8912896
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8912896)?)?;
    // ARM 0xaf608: str r6, [r7, #0xc48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,3144)?)?;
    // ARM 0xaf60c: mov r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16)?)?;
    // ARM 0xaf610: movhi r7, #12
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(7)?,R::R0,12)?)?;
    // ARM 0xaf614: cmphi r5, #17825792
    c.emit(arm32::data_immediate(C::Hi,Op::Cmp,R::R0,R::new(5)?,17825792)?)?;
    // ARM 0xaf618: bls 0xaf628 <entry+0x2ed8>
    c.b(C::Ls,0xaf628,false)?;
    // ARM 0xaf61c: mov r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,6)?)?;
    // ARM 0xaf620: cmp r5, #26738688
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,26738688)?)?;
    // ARM 0xaf624: movls r7, #8
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // ARM 0xaf628: ldrb r2, [r4, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,123)?)?;
    // ARM 0xaf62c: and r3, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(7)?,255)?)?;
    // ARM 0xaf630: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xaf634: bls 0xaf660 <entry+0x2f10>
    c.b(C::Ls,0xaf660,false)?;
    // ARM 0xaf638: ldr r1, [pc, #0x50]
    c.literal(R::R1,0xaf690)?;
    // ARM 0xaf63c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaf640: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaf644: ldr r1, [pc, #0x40]
    c.literal(R::R1,0xaf68c)?;
    // ARM 0xaf648: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaf64c: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaf650: mov r0, #186
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,186)?)?;
    // ARM 0xaf654: strb r7, [r4, #0x7b]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,123)?)?;
    // ARM 0xaf658: strb r0, [r4, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3)?)?;
    // ARM 0xaf65c: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xaf660: cmp r6, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xaf664: pophs {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Cs,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xaf668: ldr r1, [pc, #0x18]
    c.literal(R::R1,0xaf688)?;
    // ARM 0xaf66c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaf670: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // ARM 0xaf674: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xaf678: ldr r1, [pc, #0xc]
    c.literal(R::R1,0xaf68c)?;
    // ARM 0xaf67c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xaf680: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xaf684: b 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,false)?;
    c.check(0xaf688)?;
    const CODEC_FORMAT_MESSAGES:[u32;3]=[
        0x000f8872, 0x000e2788, 0x000f883c,
    ];
    for value in CODEC_FORMAT_MESSAGES {c.emit(value)?;}
    c.check(0xaf694)?;
    // ARM 0xaf694: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xaf698: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // ARM 0xaf69c: add r4, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // ARM 0xaf6a0: ldr r0, [r4, #0xb8]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(4)?,184)?)?;
    // ARM 0xaf6a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaf6a8: bne 0xaf6c0 <entry+0x2f70>
    c.b(C::Ne,0xaf6c0,false)?;
    // ARM 0xaf6ac: mov r0, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,100)?)?;
    // ARM 0xaf6b0: bl 0x2f28 <entry+0xfffffffffff567d8>
    c.b(C::Al,0x2f28,true)?;
    // ARM 0xaf6b4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xaf6b8: beq 0xaf6d8 <entry+0x2f88>
    c.b(C::Eq,0xaf6d8,false)?;
    // ARM 0xaf6bc: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // ARM 0xaf6c0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xaf6c4: mov r2, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,100)?)?;
    // ARM 0xaf6c8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM 0xaf6cc: bl 0x31ec <entry+0xfffffffffff56a9c>
    c.b(C::Al,0x31ec,true)?;
    // ARM 0xaf6d0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM 0xaf6d4: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xaf6d8: mov r4, #45
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,45)?)?;
    // ARM 0xaf6dc: b 0xaf6d0 <entry+0x2f80>
    c.b(C::Al,0xaf6d0,false)?;
    c.check(0xaf6e0)
}
