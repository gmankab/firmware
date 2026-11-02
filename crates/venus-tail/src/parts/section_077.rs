
/// Allocate the controller's software channel object, clear its packed
/// capability fields, attach codec input/output buffers and preserve the
/// source function's failure paths. Trailing diagnostics are runtime data.
fn initialize_codec_control_channel_object(c:&mut Code)->Result<(),String>{
    c.check(0xc1514)?;
    // VMA 0xc1514: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc1518: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xc151c: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // VMA 0xc1520: ldr r4, [pc, #0x398]        @ 0x2d50 <entry+0x2d50>
    c.literal(R::new(4)?,0xc18c0)?;
    // VMA 0xc1524: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xc1528: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xc152c: mov r1, #780
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,780)?)?;
    // VMA 0xc1530: mov r10, r2
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R2))?;
    // VMA 0xc1534: mov r9, r3
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(3)?))?;
    // VMA 0xc1538: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xc153c: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xc1540: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1544: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xc1548: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbfda0
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc154c: mov r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    // VMA 0xc1550: cmp r6, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,256)?)?;
    // VMA 0xc1554: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc1558: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xc155c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc1560: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc1564: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xc1568: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbfdc0
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc156c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1570: bl 0x2f28 <entry+0xfffffffffff443b8> @ imm = #-0xbe650
    c.b(C::Al,0x2f28,true)?;
    // VMA 0xc1574: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xc1578: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc157c: beq 0xc177c <entry+0x2c0c>  @ imm = #0x1f8
    c.b(C::Eq,0xc177c,false)?;
    // VMA 0xc1580: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc1584: strb r5, [r4, #0x5]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,5)?)?;
    // VMA 0xc1588: strb r5, [r4, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,3)?)?;
    // VMA 0xc158c: strb r5, [r4, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,2)?)?;
    // VMA 0xc1590: strb r5, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,1)?)?;
    // VMA 0xc1594: strb r5, [r4]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // VMA 0xc1598: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xc159c: strb r5, [r0, #0x4]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::new(5)?,R::R0,4)?)?;
    // VMA 0xc15a0: strb r5, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R0,3)?)?;
    // VMA 0xc15a4: strb r5, [r0, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(5)?,R::R0,2)?)?;
    // VMA 0xc15a8: ldr r0, [pc, #0x314]        @ 0x2d54 <entry+0x2d54>
    c.literal(R::R0,0xc18c4)?;
    // VMA 0xc15ac: str r4, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(7)?,0)?)?;
    // VMA 0xc15b0: str r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // VMA 0xc15b4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xc15b8: bl 0xd035c <entry+0x117ec> @ imm = #0xed9c
    c.b(C::Al,0xd035c,true)?;
    // VMA 0xc15bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc15c0: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc15c4: beq 0xc17a4 <entry+0x2c34>  @ imm = #0x1d8
    c.b(C::Eq,0xc17a4,false)?;
    // VMA 0xc15c8: mov r0, #952
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,952)?)?;
    // VMA 0xc15cc: bl 0x2efc <entry+0xfffffffffff4438c> @ imm = #-0xbe6d8
    c.b(C::Al,0x2efc,true)?;
    // VMA 0xc15d0: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xc15d4: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc15d8: beq 0xc17d0 <entry+0x2c60>  @ imm = #0x1f0
    c.b(C::Eq,0xc17d0,false)?;
    // VMA 0xc15dc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc15e0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc15e4: mov r2, #952
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,952)?)?;
    // VMA 0xc15e8: bl 0x31ec <entry+0xfffffffffff4467c> @ imm = #-0xbe404
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xc15ec: mov r0, #712
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,712)?)?;
    // VMA 0xc15f0: str r5, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,4)?)?;
    // VMA 0xc15f4: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // VMA 0xc15f8: bl 0x2efc <entry+0xfffffffffff4438c> @ imm = #-0xbe704
    c.b(C::Al,0x2efc,true)?;
    // VMA 0xc15fc: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xc1600: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xc1604: beq 0xc17f4 <entry+0x2c84>  @ imm = #0x1e8
    c.b(C::Eq,0xc17f4,false)?;
    // VMA 0xc1608: mov r2, #712
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,712)?)?;
    // VMA 0xc160c: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc1610: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc1614: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // VMA 0xc1618: bl 0x31ec <entry+0xfffffffffff4467c> @ imm = #-0xbe434
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xc161c: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xc1620: str r8, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R0,0)?)?;
    // VMA 0xc1624: mov r0, #3792
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3792)?)?;
    // VMA 0xc1628: bl 0x2efc <entry+0xfffffffffff4438c> @ imm = #-0xbe734
    c.b(C::Al,0x2efc,true)?;
    // VMA 0xc162c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1630: str r0, [r5, #0x280]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,640)?)?;
    // VMA 0xc1634: beq 0xc181c <entry+0x2cac>  @ imm = #0x1e0
    c.b(C::Eq,0xc181c,false)?;
    // VMA 0xc1638: mov r0, #3792
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3792)?)?;
    // VMA 0xc163c: bl 0x2efc <entry+0xfffffffffff4438c> @ imm = #-0xbe748
    c.b(C::Al,0x2efc,true)?;
    // VMA 0xc1640: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1644: str r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,644)?)?;
    // VMA 0xc1648: beq 0xc1830 <entry+0x2cc0>  @ imm = #0x1e0
    c.b(C::Eq,0xc1830,false)?;
    // VMA 0xc164c: mov r0, #3792
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3792)?)?;
    // VMA 0xc1650: bl 0x2efc <entry+0xfffffffffff4438c> @ imm = #-0xbe75c
    c.b(C::Al,0x2efc,true)?;
    // VMA 0xc1654: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1658: str r0, [r5, #0x288]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,648)?)?;
    // VMA 0xc165c: beq 0xc1844 <entry+0x2cd4>  @ imm = #0x1e0
    c.b(C::Eq,0xc1844,false)?;
    // VMA 0xc1660: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1664: cmp r10, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,4096)?)?;
    // VMA 0xc1668: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc166c: moveq r0, #7
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xc1670: and r2, r1, r9, lsr #9
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::new(9)?,Shift::Lsr,9,false)?)?;
    // VMA 0xc1674: str r0, [r5, #0x248]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,584)?)?;
    // VMA 0xc1678: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc167c: str r0, [r8, #0xaa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,2720)?)?;
    // VMA 0xc1680: str r0, [r8, #0xaa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,2724)?)?;
    // VMA 0xc1684: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc1688: str r0, [r5, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,456)?)?;
    // VMA 0xc168c: str r2, [r8, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,8)?)?;
    // VMA 0xc1690: str r2, [r8, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,12)?)?;
    // VMA 0xc1694: str r10, [r8, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(8)?,16)?)?;
    // VMA 0xc1698: str r0, [r8, #0x910]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,2320)?)?;
    // VMA 0xc169c: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xc16a0: str r1, [r8, #0xa98]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,2712)?)?;
    // VMA 0xc16a4: str r0, [r8, #0x424]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,1060)?)?;
    // VMA 0xc16a8: str r1, [r5, #0x3a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,936)?)?;
    // VMA 0xc16ac: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc16b0: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xc16b4: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xc16b8: cmpne r0, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xc16bc: bne 0xc16d4 <entry+0x2b64>  @ imm = #0x10
    c.b(C::Ne,0xc16d4,false)?;
    // VMA 0xc16c0: add r2, r5, #364
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,364)?)?;
    // VMA 0xc16c4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc16c8: mov r1, #3792
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3792)?)?;
    // VMA 0xc16cc: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xc16d0: bl 0x7e544 <entry+0xfffffffffffbf9d4> @ imm = #-0x43194
    c.b(C::Al,0x7e544,true)?;
    // VMA 0xc16d4: ldr r0, [r5, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,364)?)?;
    // VMA 0xc16d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc16dc: beq 0xc1864 <entry+0x2cf4>  @ imm = #0x180
    c.b(C::Eq,0xc1864,false)?;
    // VMA 0xc16e0: ldr r6, [pc, #0x1d8]        @ 0x2d50 <entry+0x2d50>
    c.literal(R::new(6)?,0xc18c0)?;
    // VMA 0xc16e4: add r5, r5, #180
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,180)?)?;
    // VMA 0xc16e8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xc16ec: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc16f0: ldr r1, [r0, #-0x8]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,-8)?)?;
    // VMA 0xc16f4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc16f8: bne 0xc1714 <entry+0x2ba4>  @ imm = #0x14
    c.b(C::Ne,0xc1714,false)?;
    // VMA 0xc16fc: bl 0xd9cf4 <entry+0x1b184> @ imm = #0x185f0
    c.b(C::Al,0xd9cf4,true)?;
    // VMA 0xc1700: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc1704: mov r1, #239
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,239)?)?;
    // VMA 0xc1708: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xc170c: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc1710: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbff68
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc1714: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc1718: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc171c: bne 0xc173c <entry+0x2bcc>  @ imm = #0x18
    c.b(C::Ne,0xc173c,false)?;
    // VMA 0xc1720: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc1724: bl 0xd9cf4 <entry+0x1b184> @ imm = #0x185c8
    c.b(C::Al,0xd9cf4,true)?;
    // VMA 0xc1728: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc172c: mov r1, #244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,244)?)?;
    // VMA 0xc1730: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xc1734: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc1738: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbff90
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc173c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xc1740: add r5, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4)?)?;
    // VMA 0xc1744: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // VMA 0xc1748: bne 0xc16ec <entry+0x2b7c>  @ imm = #-0x64
    c.b(C::Ne,0xc16ec,false)?;
    // VMA 0xc174c: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xc1750: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc1754: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc1758: strb r1, [r0, #0x226]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,550)?)?;
    // VMA 0xc175c: strb r1, [r0, #0x227]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,551)?)?;
    // VMA 0xc1760: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xc1764: ldr r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // VMA 0xc1768: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // VMA 0xc176c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc1770: moveq r1, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xc1774: str r1, [r0, #0x404]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1028)?)?;
    // VMA 0xc1778: b 0xc18b4 <entry+0x2d44>  @ imm = #0x134
    c.b(C::Al,0xc18b4,false)?;
    // VMA 0xc177c: ldr r1, [pc, #0x15c]        @ 0x2d70 <entry+0x2d70>
    c.literal(R::R1,0xc18e0)?;
    // VMA 0xc1780: ldr r2, [pc, #0x154]        @ 0x2d6c <entry+0x2d6c>
    c.literal(R::R2,0xc18dc)?;
    // VMA 0xc1784: mov r3, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,18)?)?;
    // VMA 0xc1788: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc178c: orr r3, r3, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // VMA 0xc1790: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92b04
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1794: ldr r1, [pc, #0x134]        @ 0x2d60 <entry+0x2d60>
    c.literal(R::R1,0xc18d0)?;
    // VMA 0xc1798: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc179c: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92b10
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc17a0: b 0xc18b4 <entry+0x2d44>  @ imm = #0x10c
    c.b(C::Al,0xc18b4,false)?;
    // VMA 0xc17a4: ldr r1, [pc, #0x12c]        @ 0x2d68 <entry+0x2d68>
    c.literal(R::R1,0xc18d8)?;
    // VMA 0xc17a8: ldr r2, [pc, #0x12c]        @ 0x2d6c <entry+0x2d6c>
    c.literal(R::R2,0xc18dc)?;
    // VMA 0xc17ac: mov r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,31)?)?;
    // VMA 0xc17b0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc17b4: orr r3, r3, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // VMA 0xc17b8: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92b2c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc17bc: ldr r1, [pc, #0x10c]        @ 0x2d60 <entry+0x2d60>
    c.literal(R::R1,0xc18d0)?;
    // VMA 0xc17c0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc17c4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92b38
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc17c8: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // VMA 0xc17cc: b 0xc18b4 <entry+0x2d44>  @ imm = #0xe0
    c.b(C::Al,0xc18b4,false)?;
    // VMA 0xc17d0: ldr r1, [pc, #0xfc]         @ 0x2d64 <entry+0x2d64>
    c.literal(R::R1,0xc18d4)?;
    // VMA 0xc17d4: ldr r2, [pc, #0xf0]         @ 0x2d5c <entry+0x2d5c>
    c.literal(R::R2,0xc18cc)?;
    // VMA 0xc17d8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc17dc: mov r3, #149
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,149)?)?;
    // VMA 0xc17e0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92b54
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc17e4: ldr r1, [pc, #0xe4]         @ 0x2d60 <entry+0x2d60>
    c.literal(R::R1,0xc18d0)?;
    // VMA 0xc17e8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc17ec: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92b60
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc17f0: b 0xc18a4 <entry+0x2d34>  @ imm = #0xac
    c.b(C::Al,0xc18a4,false)?;
    // VMA 0xc17f4: ldr r1, [pc, #0xd8]         @ 0x2d64 <entry+0x2d64>
    c.literal(R::R1,0xc18d4)?;
    // VMA 0xc17f8: ldr r2, [pc, #0xcc]         @ 0x2d5c <entry+0x2d5c>
    c.literal(R::R2,0xc18cc)?;
    // VMA 0xc17fc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1800: mov r3, #160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,160)?)?;
    // VMA 0xc1804: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92b78
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1808: ldr r1, [pc, #0xc0]         @ 0x2d60 <entry+0x2d60>
    c.literal(R::R1,0xc18d0)?;
    // VMA 0xc180c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1810: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92b84
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1814: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc1818: b 0xc18a0 <entry+0x2d30>  @ imm = #0x80
    c.b(C::Al,0xc18a0,false)?;
    // VMA 0xc181c: ldr r1, [pc, #0xa4]         @ 0x2d58 <entry+0x2d58>
    c.literal(R::R1,0xc18c8)?;
    // VMA 0xc1820: ldr r2, [pc, #0xa4]         @ 0x2d5c <entry+0x2d5c>
    c.literal(R::R2,0xc18cc)?;
    // VMA 0xc1824: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1828: mov r3, #172
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,172)?)?;
    // VMA 0xc182c: b 0xc1854 <entry+0x2ce4>  @ imm = #0x20
    c.b(C::Al,0xc1854,false)?;
    // VMA 0xc1830: ldr r1, [pc, #0x90]         @ 0x2d58 <entry+0x2d58>
    c.literal(R::R1,0xc18c8)?;
    // VMA 0xc1834: ldr r2, [pc, #0x90]         @ 0x2d5c <entry+0x2d5c>
    c.literal(R::R2,0xc18cc)?;
    // VMA 0xc1838: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc183c: mov r3, #179
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,179)?)?;
    // VMA 0xc1840: b 0xc1854 <entry+0x2ce4>  @ imm = #0xc
    c.b(C::Al,0xc1854,false)?;
    // VMA 0xc1844: ldr r1, [pc, #0x7c]         @ 0x2d58 <entry+0x2d58>
    c.literal(R::R1,0xc18c8)?;
    // VMA 0xc1848: ldr r2, [pc, #0x7c]         @ 0x2d5c <entry+0x2d5c>
    c.literal(R::R2,0xc18cc)?;
    // VMA 0xc184c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1850: mov r3, #186
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,186)?)?;
    // VMA 0xc1854: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92bc8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1858: ldr r1, [pc, #0x70]         @ 0x2d60 <entry+0x2d60>
    c.literal(R::R1,0xc18d0)?;
    // VMA 0xc185c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc1860: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92bd4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1864: ldr r0, [r5, #0x288]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,648)?)?;
    // VMA 0xc1868: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc186c: beq 0xc1874 <entry+0x2d04>  @ imm = #0x0
    c.b(C::Eq,0xc1874,false)?;
    // VMA 0xc1870: bl 0x2f1c <entry+0xfffffffffff443ac> @ imm = #-0xbe95c
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xc1874: ldr r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,644)?)?;
    // VMA 0xc1878: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc187c: beq 0xc1884 <entry+0x2d14>  @ imm = #0x0
    c.b(C::Eq,0xc1884,false)?;
    // VMA 0xc1880: bl 0x2f1c <entry+0xfffffffffff443ac> @ imm = #-0xbe96c
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xc1884: ldr r0, [r5, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,640)?)?;
    // VMA 0xc1888: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc188c: beq 0xc1894 <entry+0x2d24>  @ imm = #0x0
    c.b(C::Eq,0xc1894,false)?;
    // VMA 0xc1890: bl 0x2f1c <entry+0xfffffffffff443ac> @ imm = #-0xbe97c
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xc1894: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc1898: bl 0x2f1c <entry+0xfffffffffff443ac> @ imm = #-0xbe984
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xc189c: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xc18a0: bl 0x2f1c <entry+0xfffffffffff443ac> @ imm = #-0xbe98c
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xc18a4: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc18a8: bl 0xd0f18 <entry+0x123a8> @ imm = #0xf668
    c.b(C::Al,0xd0f18,true)?;
    // VMA 0xc18ac: mov r5, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,9)?)?;
    // VMA 0xc18b0: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // VMA 0xc18b4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc18b8: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xc18bc: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc18c0)?;
    const CONTROLLER_CHANNEL_REGISTRY:[u32;9]=[
      0x000fa71c,0x0010bbbc,0x000faf13,
      0x000faf05,0x000e2788,0x000faec6,
      0x000fab5b,0x000fab46,0x000f4f2d,
    ];
    for address in CONTROLLER_CHANNEL_REGISTRY {c.emit(address)?;}
    c.check(0xc18e4)
}
