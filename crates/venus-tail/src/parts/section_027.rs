
/// Load and validate codec frame-plane state; update controller register
/// notifications and release inconsistent stream buffers. This interval has
/// two original ARM routines and two explicitly non-instruction literal pools.
fn synchronize_frame_planes_and_dispatch_notifications(c:&mut Code)->Result<(),String>{
    c.check(0xb11b4)?;
    // ARM 0xb11b4: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb11b8: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xb11bc: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // ARM 0xb11c0: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM 0xb11c4: ldr r6, [r4, #0xf78]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,3960)?)?;
    // ARM 0xb11c8: ldr r5, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,36)?)?;
    // ARM 0xb11cc: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // ARM 0xb11d0: bne 0xb11fc <entry+0x4aac>
    c.b(C::Ne,0xb11fc,false)?;
    // ARM 0xb11d4: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // ARM 0xb11d8: bl 0x2f28 <entry+0xfffffffffff567d8>
    c.b(C::Al,0x2f28,true)?;
    // ARM 0xb11dc: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // ARM 0xb11e0: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // ARM 0xb11e4: beq 0xb165c <entry+0x4f0c>
    c.b(C::Eq,0xb165c,false)?;
    // ARM 0xb11e8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xb11ec: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb11f0: mov r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // ARM 0xb11f4: bl 0x31ec <entry+0xfffffffffff56a9c>
    c.b(C::Al,0x31ec,true)?;
    // ARM 0xb11f8: str r6, [r4, #0xf78]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,3960)?)?;
    // ARM 0xb11fc: mov r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2048)?)?;
    // ARM 0xb1200: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb1204: mov r7, #74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,74)?)?;
    // ARM 0xb1208: orr r10, r0, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(10)?,R::R0,R::new(5)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb120c: mov r0, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16777216)?)?;
    // ARM 0xb1210: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // ARM 0xb1214: str r0, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xb1218: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // ARM 0xb121c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb1220: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb1224: orr r8, r0, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::R0,R::new(5)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb1228: ldr r0, [r8, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb122c: and r2, r0, #192
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,192)?)?;
    // ARM 0xb1230: strb r0, [r6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // ARM 0xb1234: cmp r2, #63
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,63)?)?;
    // ARM 0xb1238: bhi 0xb1660 <entry+0x4f10>
    c.b(C::Hi,0xb1660,false)?;
    // ARM 0xb123c: ldr r1, [r8, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1240: and r2, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,255)?)?;
    // ARM 0xb1244: strb r1, [r6, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,1)?)?;
    // ARM 0xb1248: cmp r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,15)?)?;
    // ARM 0xb124c: bhi 0xb1660 <entry+0x4f10>
    c.b(C::Hi,0xb1660,false)?;
    // ARM 0xb1250: add r1, r4, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(4)?,R::R2,Shift::Lsl,2,false)?)?;
    // ARM 0xb1254: ldr r9, [r1, #0xe30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R1,3632)?)?;
    // ARM 0xb1258: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // ARM 0xb125c: bne 0xb1288 <entry+0x4b38>
    c.b(C::Ne,0xb1288,false)?;
    // ARM 0xb1260: ldr r9, [r4, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,2444)?)?;
    // ARM 0xb1264: and r3, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R0,255)?)?;
    // ARM 0xb1268: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb126c: ldrb r1, [r9, #0x5f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,95)?)?;
    // ARM 0xb1270: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // ARM 0xb1274: ldr r1, [pc, #0x78c]
    c.literal(R::R1,0xb1a08)?;
    // ARM 0xb1278: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb127c: ldr r1, [pc, #0x788]
    c.literal(R::R1,0xb1a0c)?;
    // ARM 0xb1280: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb1284: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1288: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb128c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb1290: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // ARM 0xb1294: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // ARM 0xb1298: str r1, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb129c: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb12a0: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // ARM 0xb12a4: mov r3, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16777216)?)?;
    // ARM 0xb12a8: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb12ac: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb12b0: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // ARM 0xb12b4: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // ARM 0xb12b8: mov r3, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,33554432)?)?;
    // ARM 0xb12bc: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    // ARM 0xb12c0: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb12c4: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb12c8: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb12cc: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // ARM 0xb12d0: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb12d4: strb r0, [r6, #0x12]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,18)?)?;
    // ARM 0xb12d8: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb12dc: strb r0, [r6, #0x10]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // ARM 0xb12e0: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb12e4: strb r0, [r6, #0x2a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,42)?)?;
    // ARM 0xb12e8: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb12ec: strb r0, [r6, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,2)?)?;
    // ARM 0xb12f0: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb12f4: strb r0, [r6, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,3)?)?;
    // ARM 0xb12f8: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb12fc: add r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,1)?)?;
    // ARM 0xb1300: strb r2, [r6, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,4)?)?;
    // ARM 0xb1304: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb1308: and r1, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R2,255)?)?;
    // ARM 0xb130c: cmp r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,15)?)?;
    // ARM 0xb1310: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xb1314: strb r0, [r6, #0x5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,5)?)?;
    // ARM 0xb1318: andls r0, r0, #240
    c.emit(arm32::data_immediate(C::Ls,Op::And,R::R0,R::R0,240)?)?;
    // ARM 0xb131c: cmpls r0, #16
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,16)?)?;
    // ARM 0xb1320: blo 0xb1334 <entry+0x4be4>
    c.b(C::Cc,0xb1334,false)?;
    // ARM 0xb1324: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // ARM 0xb1328: orr r0, r0, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3840)?)?;
    // ARM 0xb132c: strh r0, [r6, #4]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(6)?,4)?)?;
    // ARM 0xb1330: b 0xb1660 <entry+0x4f10>
    c.b(C::Al,0xb1660,false)?;
    // ARM 0xb1334: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb1338: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // ARM 0xb133c: ldr r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb1340: add r2, r2, #26
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,26)?)?;
    // ARM 0xb1344: strb r2, [r6, #0x6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,6)?)?;
    // ARM 0xb1348: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // ARM 0xb134c: strb r2, [r6, #0x7]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,7)?)?;
    // ARM 0xb1350: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // ARM 0xb1354: strb r2, [r6, #0x8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,8)?)?;
    // ARM 0xb1358: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM 0xb135c: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // ARM 0xb1360: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // ARM 0xb1364: strb r1, [r6, #0x9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,9)?)?;
    // ARM 0xb1368: strb r2, [r6, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,10)?)?;
    // ARM 0xb136c: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // ARM 0xb1370: movne r1, #16777216
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,16777216)?)?;
    // ARM 0xb1374: strne r1, [r10, r0]
    c.emit(arm32::transfer_word_register_offset(C::Ne,false,R::R1,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb1378: ldrne r1, [r8, r0]
    c.emit(arm32::transfer_word_register_offset(C::Ne,true,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb137c: strbne r1, [r6, #0xa]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(6)?,10)?)?;
    // ARM 0xb1380: mov r1, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33554432)?)?;
    // ARM 0xb1384: str r1, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb1388: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb138c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb1390: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb1394: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb1398: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb139c: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb13a0: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb13a4: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // ARM 0xb13a8: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // ARM 0xb13ac: ldr r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // ARM 0xb13b0: strb r2, [r6, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,11)?)?;
    // ARM 0xb13b4: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // ARM 0xb13b8: add r1, r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,12)?)?;
    // ARM 0xb13bc: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // ARM 0xb13c0: cmp r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,24)?)?;
    // ARM 0xb13c4: strb r0, [r6, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // ARM 0xb13c8: bhi 0xb13dc <entry+0x4c8c>
    c.b(C::Hi,0xb13dc,false)?;
    // ARM 0xb13cc: mov r1, #201326592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,201326592)?)?;
    // ARM 0xb13d0: add r0, r1, r0, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,24,false)?)?;
    // ARM 0xb13d4: cmp r0, #402653185
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,402653185)?)?;
    // ARM 0xb13d8: blo 0xb13ec <entry+0x4c9c>
    c.b(C::Cc,0xb13ec,false)?;
    // ARM 0xb13dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb13e0: strb r0, [r6, #0xc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // ARM 0xb13e4: strb r0, [r6, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,11)?)?;
    // ARM 0xb13e8: b 0xb1660 <entry+0x4f10>
    c.b(C::Al,0xb1660,false)?;
    // ARM 0xb13ec: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // ARM 0xb13f0: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // ARM 0xb13f4: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // ARM 0xb13f8: ldr r1, [r0, r3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R1,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // ARM 0xb13fc: strb r1, [r6, #0xd]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,13)?)?;
    // ARM 0xb1400: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // ARM 0xb1404: strb r1, [r6, #0xe]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,14)?)?;
    // ARM 0xb1408: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // ARM 0xb140c: strb r1, [r6, #0xf]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,15)?)?;
    // ARM 0xb1410: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // ARM 0xb1414: strb r1, [r6, #0x11]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,17)?)?;
    // ARM 0xb1418: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // ARM 0xb141c: strb r1, [r6, #0x13]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,19)?)?;
    // ARM 0xb1420: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // ARM 0xb1424: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // ARM 0xb1428: strb r0, [r6, #0x14]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // ARM 0xb142c: strb r12, [r6, #0x20]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::new(6)?,32)?)?;
    // ARM 0xb1430: beq 0xb15b0 <entry+0x4e60>
    c.b(C::Eq,0xb15b0,false)?;
    // ARM 0xb1434: mov r0, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16777216)?)?;
    // ARM 0xb1438: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // ARM 0xb143c: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // ARM 0xb1440: str r0, [r1, r3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // ARM 0xb1444: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb1448: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb144c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb1450: ldr r0, [r2, r3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // ARM 0xb1454: mov r3, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,616)?)?;
    // ARM 0xb1458: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // ARM 0xb145c: strh r0, [r6, #22]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(6)?,22)?)?;
    // ARM 0xb1460: ldr r1, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // ARM 0xb1464: add r12, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R1,1)?)?;
    // ARM 0xb1468: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // ARM 0xb146c: strh r12, [r6, #24]
    c.emit(arm32::store_halfword(C::Al,R::R12,R::new(6)?,24)?)?;
    // ARM 0xb1470: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // ARM 0xb1474: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // ARM 0xb1478: strb r2, [r6, #0x15]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,21)?)?;
    // ARM 0xb147c: and r2, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1480: ldrh r3, [r9, r3]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(3)?,R::new(9)?,R::new(3)?)?)?;
    // ARM 0xb1484: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb1488: blo 0xb16a8 <entry+0x4f58>
    c.b(C::Cc,0xb16a8,false)?;
    // ARM 0xb148c: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1490: beq 0xb16a8 <entry+0x4f58>
    c.b(C::Eq,0xb16a8,false)?;
    // ARM 0xb1494: mov r2, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,106)?)?;
    // ARM 0xb1498: sub r3, r12, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R12,1)?)?;
    // ARM 0xb149c: orr r2, r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,512)?)?;
    // ARM 0xb14a0: and r3, r3, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb14a4: ldrh r2, [r9, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R2,R::new(9)?,R::R2)?)?;
    // ARM 0xb14a8: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb14ac: bhs 0xb16a8 <entry+0x4f58>
    c.b(C::Cs,0xb16a8,false)?;
    // ARM 0xb14b0: mov r7, r6
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // ARM 0xb14b4: ldr r2, [r7, #0x1c]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R2,R::new(7)?,28)?)?;
    // ARM 0xb14b8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xb14bc: beq 0xb14d8 <entry+0x4d88>
    c.b(C::Eq,0xb14d8,false)?;
    // ARM 0xb14c0: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // ARM 0xb14c4: mov r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // ARM 0xb14c8: bl 0x3040 <entry+0xfffffffffff568f0>
    c.b(C::Al,0x3040,true)?;
    // ARM 0xb14cc: ldrh r12, [r6, #24]
    c.emit(arm32::load_halfword(C::Al,R::R12,R::new(6)?,24)?)?;
    // ARM 0xb14d0: ldrh r0, [r6, #22]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(6)?,22)?)?;
    // ARM 0xb14d4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xb14d8: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb14dc: and r1, r12, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R12,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb14e0: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb14e4: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // ARM 0xb14e8: bl 0x2f28 <entry+0xfffffffffff567d8>
    c.b(C::Al,0x2f28,true)?;
    // ARM 0xb14ec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb14f0: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // ARM 0xb14f4: beq 0xb165c <entry+0x4f0c>
    c.b(C::Eq,0xb165c,false)?;
    // ARM 0xb14f8: ldrb r1, [r6, #0x15]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,21)?)?;
    // ARM 0xb14fc: mov r3, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // ARM 0xb1500: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xb1504: bne 0xb1584 <entry+0x4e34>
    c.b(C::Ne,0xb1584,false)?;
    // ARM 0xb1508: ldrh r1, [r6, #22]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(6)?,22)?)?;
    // ARM 0xb150c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // ARM 0xb1510: blo 0xb1544 <entry+0x4df4>
    c.b(C::Cc,0xb1544,false)?;
    // ARM 0xb1514: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb1518: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // ARM 0xb151c: mov r12, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16777216)?)?;
    // ARM 0xb1520: str r12, [r10, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R12,R::new(10)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb1524: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // ARM 0xb1528: ldr r7, [r8, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(7)?,R::new(8)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb152c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // ARM 0xb1530: strh r7, [r0], #2
    c.emit(arm32::store_halfword_postindexed(C::Al,R::new(7)?,R::R0,2)?)?;
    // ARM 0xb1534: ldrh r7, [r6, #22]
    c.emit(arm32::load_halfword(C::Al,R::new(7)?,R::new(6)?,22)?)?;
    // ARM 0xb1538: sub r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::new(7)?,1)?)?;
    // ARM 0xb153c: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1540: blt 0xb1520 <entry+0x4dd0>
    c.b(C::Lt,0xb1520,false)?;
    // ARM 0xb1544: ldrh r1, [r6, #24]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(6)?,24)?)?;
    // ARM 0xb1548: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // ARM 0xb154c: blo 0xb1584 <entry+0x4e34>
    c.b(C::Cc,0xb1584,false)?;
    // ARM 0xb1550: add r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // ARM 0xb1554: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb1558: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // ARM 0xb155c: mov r12, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,16777216)?)?;
    // ARM 0xb1560: str r12, [r10, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R12,R::new(10)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb1564: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // ARM 0xb1568: ldr r7, [r8, r2]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(7)?,R::new(8)?,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb156c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // ARM 0xb1570: strh r7, [r0], #2
    c.emit(arm32::store_halfword_postindexed(C::Al,R::new(7)?,R::R0,2)?)?;
    // ARM 0xb1574: ldrh r7, [r6, #24]
    c.emit(arm32::load_halfword(C::Al,R::new(7)?,R::new(6)?,24)?)?;
    // ARM 0xb1578: sub r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::new(7)?,1)?)?;
    // ARM 0xb157c: cmp r1, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1580: blt 0xb1560 <entry+0x4e10>
    c.b(C::Lt,0xb1560,false)?;
    // ARM 0xb1584: ldrh r0, [r6, #22]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(6)?,22)?)?;
    // ARM 0xb1588: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // ARM 0xb158c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xb1590: ldrheq r0, [r6, #24]
    c.emit(arm32::load_halfword(C::Eq,R::R0,R::new(6)?,24)?)?;
    // ARM 0xb1594: cmpeq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM 0xb1598: beq 0xb15b0 <entry+0x4e60>
    c.b(C::Eq,0xb15b0,false)?;
    // ARM 0xb159c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb15a0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM 0xb15a4: str r1, [r10, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::new(10)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb15a8: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb15ac: strb r0, [r6, #0x20]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,32)?)?;
    // ARM 0xb15b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb15b4: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // ARM 0xb15b8: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // ARM 0xb15bc: str r0, [r1, r3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // ARM 0xb15c0: ldr r3, [r2, r3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // ARM 0xb15c4: strb r3, [r6, #0x21]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(6)?,33)?)?;
    // ARM 0xb15c8: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // ARM 0xb15cc: ldr r1, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // ARM 0xb15d0: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // ARM 0xb15d4: strb r1, [r6, #0x22]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,34)?)?;
    // ARM 0xb15d8: strb r12, [r2, #0x23]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R12,R::R2,35)?)?;
    // ARM 0xb15dc: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // ARM 0xb15e0: strb r12, [r2, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R2,3)?)?;
    // ARM 0xb15e4: strb r12, [r2, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R2,2)?)?;
    // ARM 0xb15e8: strb r12, [r2, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R12,R::R2,1)?)?;
    // ARM 0xb15ec: beq 0xb16e8 <entry+0x4f98>
    c.b(C::Eq,0xb16e8,false)?;
    // ARM 0xb15f0: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb15f4: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // ARM 0xb15f8: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // ARM 0xb15fc: str r0, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xb1600: ldr r7, [r3, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(7)?,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xb1604: strb r7, [r6, #0x23]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(6)?,35)?)?;
    // ARM 0xb1608: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // ARM 0xb160c: ldr r2, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb1610: tst r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,255)?)?;
    // ARM 0xb1614: strb r2, [r6, #0x24]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,36)?)?;
    // ARM 0xb1618: bne 0xb16e8 <entry+0x4f98>
    c.b(C::Ne,0xb16e8,false)?;
    // ARM 0xb161c: mov r2, #33554432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,33554432)?)?;
    // ARM 0xb1620: mov r3, r10
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(10)?))?;
    // ARM 0xb1624: str r2, [r3, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xb1628: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // ARM 0xb162c: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // ARM 0xb1630: ldr r3, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // ARM 0xb1634: strb r3, [r6, #0x25]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(6)?,37)?)?;
    // ARM 0xb1638: ldr r1, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // ARM 0xb163c: lsl r2, r3, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(3)?,Shift::Lsl,24)?)?;
    // ARM 0xb1640: asr r3, r2, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Asr,24)?)?;
    // ARM 0xb1644: mov r2, #250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,250)?)?;
    // ARM 0xb1648: cmn r3, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(3)?,6)?)?;
    // ARM 0xb164c: strb r1, [r6, #0x26]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,38)?)?;
    // ARM 0xb1650: bge 0xb16b4 <entry+0x4f64>
    c.b(C::Ge,0xb16b4,false)?;
    // ARM 0xb1654: strb r2, [r6, #0x25]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(6)?,37)?)?;
    // ARM 0xb1658: b 0xb16c0 <entry+0x4f70>
    c.b(C::Al,0xb16c0,false)?;
    // ARM 0xb165c: mov r7, #45
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,45)?)?;
    // ARM 0xb1660: mov r0, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,116)?)?;
    // ARM 0xb1664: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb1668: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb166c: orr r0, r0, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(5)?,Shift::Lsl,15,false)?)?;
    // ARM 0xb1670: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1674: tst r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,4)?)?;
    // ARM 0xb1678: bne 0xb1698 <entry+0x4f48>
    c.b(C::Ne,0xb1698,false)?;
    // ARM 0xb167c: lsl r2, r5, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(5)?,Shift::Lsl,15)?)?;
    // ARM 0xb1680: orr r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4)?)?;
    // ARM 0xb1684: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // ARM 0xb1688: ldr r3, [r2, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb168c: ldr r3, [r0, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::new(3)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1690: tst r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,4)?)?;
    // ARM 0xb1694: beq 0xb1688 <entry+0x4f38>
    c.b(C::Eq,0xb1688,false)?;
    // ARM 0xb1698: str r7, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,2432)?)?;
    // ARM 0xb169c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb16a0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xb16a4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xb16a8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb16ac: strb r0, [r6, #0x13]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,19)?)?;
    // ARM 0xb16b0: b 0xb1660 <entry+0x4f10>
    c.b(C::Al,0xb1660,false)?;
    // ARM 0xb16b4: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // ARM 0xb16b8: cmp r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,7)?)?;
    // ARM 0xb16bc: strbge r2, [r6, #0x25]
    c.emit(arm32::transfer_byte(C::Ge,false,R::R2,R::new(6)?,37)?)?;
    // ARM 0xb16c0: lsl r1, r1, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsl,24)?)?;
    // ARM 0xb16c4: asr r1, r1, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Asr,24)?)?;
    // ARM 0xb16c8: cmn r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,7)?)?;
    // ARM 0xb16cc: bgt 0xb16d8 <entry+0x4f88>
    c.b(C::Gt,0xb16d8,false)?;
    // ARM 0xb16d0: mov r1, #250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,250)?)?;
    // ARM 0xb16d4: b 0xb16e4 <entry+0x4f94>
    c.b(C::Al,0xb16e4,false)?;
    // ARM 0xb16d8: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // ARM 0xb16dc: blt 0xb16e8 <entry+0x4f98>
    c.b(C::Lt,0xb16e8,false)?;
    // ARM 0xb16e0: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // ARM 0xb16e4: strb r1, [r6, #0x26]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,38)?)?;
    // ARM 0xb16e8: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb16ec: lsl r2, r5, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(5)?,Shift::Lsl,15)?)?;
    // ARM 0xb16f0: str r0, [r10, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::new(10)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb16f4: ldr r0, [r8, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb16f8: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // ARM 0xb16fc: strb r0, [r6, #0x27]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,39)?)?;
    // ARM 0xb1700: str r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // ARM 0xb1704: beq 0xb175c <entry+0x500c>
    c.b(C::Eq,0xb175c,false)?;
    // ARM 0xb1708: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb170c: mov r9, r6
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(6)?))?;
    // ARM 0xb1710: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // ARM 0xb1714: ldr r0, [r9, #0x3c]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(9)?,60)?)?;
    // ARM 0xb1718: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb171c: beq 0xb1828 <entry+0x50d8>
    c.b(C::Eq,0xb1828,false)?;
    // ARM 0xb1720: bl 0x80170 <entry+0xfffffffffffd3a20>
    c.b(C::Al,0x80170,true)?;
    // ARM 0xb1724: ldr r2, [pc, #0x2e4]
    c.literal(R::R2,0xb1a10)?;
    // ARM 0xb1728: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // ARM 0xb172c: mov r1, #234
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,234)?)?;
    // ARM 0xb1730: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // ARM 0xb1734: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // ARM 0xb1738: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb173c: bl 0x17b0 <entry+0xfffffffffff55060>
    c.b(C::Al,0x17b0,true)?;
    // ARM 0xb1740: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // ARM 0xb1744: blo 0xb180c <entry+0x50bc>
    c.b(C::Cc,0xb180c,false)?;
    // ARM 0xb1748: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // ARM 0xb174c: bl 0x7fed0 <entry+0xfffffffffffd3780>
    c.b(C::Al,0x7fed0,true)?;
    // ARM 0xb1750: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb1754: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // ARM 0xb1758: b 0xb1828 <entry+0x50d8>
    c.b(C::Al,0xb1828,false)?;
    // ARM 0xb175c: ldr r0, [r6, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,60)?)?;
    // ARM 0xb1760: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb1764: beq 0xb1778 <entry+0x5028>
    c.b(C::Eq,0xb1778,false)?;
    // ARM 0xb1768: bl 0x7fed0 <entry+0xfffffffffffd3780>
    c.b(C::Al,0x7fed0,true)?;
    // ARM 0xb176c: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // ARM 0xb1770: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xb1774: str r0, [r6, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,60)?)?;
    // ARM 0xb1778: mov r9, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,3758096384)?)?;
    // ARM 0xb177c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb1780: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // ARM 0xb1784: str r0, [r10, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(10)?,R::new(9)?,Shift::Lsl,0)?)?;
    // ARM 0xb1788: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,0)?)?;
    // ARM 0xb178c: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // ARM 0xb1790: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    // ARM 0xb1794: ldr r0, [r8, r9]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(8)?,R::new(9)?,Shift::Lsl,0)?)?;
    // ARM 0xb1798: strb r0, [r6, #0x28]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,40)?)?;
    // ARM 0xb179c: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // ARM 0xb17a0: strb r0, [r6, #0x29]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,41)?)?;
    // ARM 0xb17a4: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // ARM 0xb17a8: strb r0, [r6, #0x2b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,43)?)?;
    // ARM 0xb17ac: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // ARM 0xb17b0: str r0, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,44)?)?;
    // ARM 0xb17b4: orr r0, r2, #120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,120)?)?;
    // ARM 0xb17b8: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb17bc: ldr r0, [r0, r9]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb17c0: tst r0, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // ARM 0xb17c4: beq 0xb194c <entry+0x51fc>
    c.b(C::Eq,0xb194c,false)?;
    // ARM 0xb17c8: orr r1, r2, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,124)?)?;
    // ARM 0xb17cc: mov r2, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1879048192)?)?;
    // ARM 0xb17d0: mov r3, #840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,840)?)?;
    // ARM 0xb17d4: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // ARM 0xb17d8: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // ARM 0xb17dc: str r2, [r1, r9]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb17e0: stm sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xb17e4: ldr r1, [pc, #0x238]
    c.literal(R::R1,0xb1a24)?;
    // ARM 0xb17e8: ldr r2, [pc, #0x230]
    c.literal(R::R2,0xb1a20)?;
    // ARM 0xb17ec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb17f0: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb17f4: ldr r1, [pc, #0x210]
    c.literal(R::R1,0xb1a0c)?;
    // ARM 0xb17f8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb17fc: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1800: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // ARM 0xb1804: mov r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,255)?)?;
    // ARM 0xb1808: b 0xb1950 <entry+0x5200>
    c.b(C::Al,0xb1950,false)?;
    // ARM 0xb180c: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // ARM 0xb1810: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb1814: beq 0xb1828 <entry+0x50d8>
    c.b(C::Eq,0xb1828,false)?;
    // ARM 0xb1818: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // ARM 0xb181c: bl 0x8019c <entry+0xfffffffffffd3a4c>
    c.b(C::Al,0x8019c,true)?;
    // ARM 0xb1820: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // ARM 0xb1824: b 0xb1848 <entry+0x50f8>
    c.b(C::Al,0xb1848,false)?;
    // ARM 0xb1828: add r7, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::SP,16)?)?;
    // ARM 0xb182c: ldr r0, [r4, #0x490]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1168)?)?;
    // ARM 0xb1830: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // ARM 0xb1834: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // ARM 0xb1838: bl 0x7f974 <entry+0xfffffffffffd3224>
    c.b(C::Al,0x7f974,true)?;
    // ARM 0xb183c: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // ARM 0xb1840: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb1844: beq 0xb182c <entry+0x50dc>
    c.b(C::Eq,0xb182c,false)?;
    // ARM 0xb1848: ldr r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    // ARM 0xb184c: mov r7, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3758096384)?)?;
    // ARM 0xb1850: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // ARM 0xb1854: orr r1, r3, #868
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(3)?,868)?)?;
    // ARM 0xb1858: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // ARM 0xb185c: str r2, [r1, r7]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1860: orr r1, r3, #2912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(3)?,2912)?)?;
    // ARM 0xb1864: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // ARM 0xb1868: str r0, [r1, r7]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb186c: orr r0, r3, #3040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,3040)?)?;
    // ARM 0xb1870: mvn r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,255)?)?;
    // ARM 0xb1874: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // ARM 0xb1878: str r2, [r0, r7]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R2,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb187c: orr r0, r3, #988
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(3)?,988)?)?;
    // ARM 0xb1880: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // ARM 0xb1884: str r1, [r0, r7]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R1,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1888: ldr r0, [pc, #0x184]
    c.literal(R::R0,0xb1a14)?;
    // ARM 0xb188c: str r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,56)?)?;
    // ARM 0xb1890: ldr r0, [r4, #0x354]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,852)?)?;
    // ARM 0xb1894: bl 0x293c <entry+0xfffffffffff561ec>
    c.b(C::Al,0x293c,true)?;
    // ARM 0xb1898: bl 0x3070 <entry+0xfffffffffff56920>
    c.b(C::Al,0x3070,true)?;
    // ARM 0xb189c: mov r0, #218103808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,218103808)?)?;
    // ARM 0xb18a0: str r0, [r10, r7]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::R0,R::new(10)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb18a4: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // ARM 0xb18a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb18ac: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xb18b0: ldrne r1, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,56)?)?;
    // ARM 0xb18b4: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xb18b8: bl 0x26658 <entry+0xfffffffffff79f08>
    c.b(C::Al,0x26658,true)?;
    // ARM 0xb18bc: bl 0x3074 <entry+0xfffffffffff56924>
    c.b(C::Al,0x3074,true)?;
    // ARM 0xb18c0: ldr r0, [r4, #0x354]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,852)?)?;
    // ARM 0xb18c4: bl 0x294c <entry+0xfffffffffff561fc>
    c.b(C::Al,0x294c,true)?;
    // ARM 0xb18c8: ldr r0, [pc, #0x148]
    c.literal(R::R0,0xb1a18)?;
    // ARM 0xb18cc: ldr r0, [r0, r5, lsl #2]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(5)?,Shift::Lsl,2,false)?)?;
    // ARM 0xb18d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xb18d4: beq 0xb1908 <entry+0x51b8>
    c.b(C::Eq,0xb1908,false)?;
    // ARM 0xb18d8: ldr r1, [pc, #0x13c]
    c.literal(R::R1,0xb1a1c)?;
    // ARM 0xb18dc: ldr r2, [pc, #0x13c]
    c.literal(R::R2,0xb1a20)?;
    // ARM 0xb18e0: mov r3, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,23)?)?;
    // ARM 0xb18e4: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM 0xb18e8: orr r3, r3, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // ARM 0xb18ec: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb18f0: ldr r1, [pc, #0x114]
    c.literal(R::R1,0xb1a0c)?;
    // ARM 0xb18f4: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM 0xb18f8: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb18fc: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // ARM 0xb1900: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // ARM 0xb1904: b 0xb169c <entry+0x4f4c>
    c.b(C::Al,0xb169c,false)?;
    // ARM 0xb1908: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // ARM 0xb190c: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // ARM 0xb1910: orr r0, r2, #120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,120)?)?;
    // ARM 0xb1914: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb1918: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb191c: tst r0, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1879048192)?)?;
    // ARM 0xb1920: beq 0xb19cc <entry+0x527c>
    c.b(C::Eq,0xb19cc,false)?;
    // ARM 0xb1924: orr r2, r2, #124
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,124)?)?;
    // ARM 0xb1928: mov r3, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1879048192)?)?;
    // ARM 0xb192c: orr r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,2048)?)?;
    // ARM 0xb1930: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1934: ldr r1, [pc, #0xe8]
    c.literal(R::R1,0xb1a24)?;
    // ARM 0xb1938: ldr r2, [pc, #0xe0]
    c.literal(R::R2,0xb1a20)?;
    // ARM 0xb193c: stm sp, {r0, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xb1940: mov r3, #796
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,796)?)?;
    // ARM 0xb1944: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // ARM 0xb1948: b 0xb19b0 <entry+0x5260>
    c.b(C::Al,0xb19b0,false)?;
    // ARM 0xb194c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM 0xb1950: orr r0, r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,116)?)?;
    // ARM 0xb1954: orr r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // ARM 0xb1958: ldr r0, [r0, r9]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb195c: and r1, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,5)?)?;
    // ARM 0xb1960: cmp r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // ARM 0xb1964: bne 0xb199c <entry+0x524c>
    c.b(C::Ne,0xb199c,false)?;
    // ARM 0xb1968: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // ARM 0xb196c: bne 0xb1660 <entry+0x4f10>
    c.b(C::Ne,0xb1660,false)?;
    // ARM 0xb1970: ldrb r0, [r6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // ARM 0xb1974: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM 0xb1978: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // ARM 0xb197c: ldr r1, [r0, #0xe78]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,3704)?)?;
    // ARM 0xb1980: str r1, [r4, #0xf78]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,3960)?)?;
    // ARM 0xb1984: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // ARM 0xb1988: ldr r2, [r4, #0x990]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,2448)?)?;
    // ARM 0xb198c: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb1990: streq r6, [r4, #0x990]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(4)?,2448)?)?;
    // ARM 0xb1994: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // ARM 0xb1998: b 0xb1660 <entry+0x4f10>
    c.b(C::Al,0xb1660,false)?;
    // ARM 0xb199c: ldr r1, [pc, #0x84]
    c.literal(R::R1,0xb1a28)?;
    // ARM 0xb19a0: ldr r2, [pc, #0x78]
    c.literal(R::R2,0xb1a20)?;
    // ARM 0xb19a4: mov r3, #73
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,73)?)?;
    // ARM 0xb19a8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM 0xb19ac: orr r3, r3, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // ARM 0xb19b0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb19b4: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb19b8: ldr r1, [pc, #0x4c]
    c.literal(R::R1,0xb1a0c)?;
    // ARM 0xb19bc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // ARM 0xb19c0: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb19c4: mov r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,255)?)?;
    // ARM 0xb19c8: b 0xb1660 <entry+0x4f10>
    c.b(C::Al,0xb1660,false)?;
    // ARM 0xb19cc: ldr r0, [r8, r1]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // ARM 0xb19d0: mov r7, #34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,34)?)?;
    // ARM 0xb19d4: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // ARM 0xb19d8: str r0, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,48)?)?;
    // ARM 0xb19dc: beq 0xb1660 <entry+0x4f10>
    c.b(C::Eq,0xb1660,false)?;
    // ARM 0xb19e0: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // ARM 0xb19e4: ldr r1, [r8, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R1,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb19e8: cmn r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // ARM 0xb19ec: str r1, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,52)?)?;
    // ARM 0xb19f0: beq 0xb1660 <entry+0x4f10>
    c.b(C::Eq,0xb1660,false)?;
    // ARM 0xb19f4: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register_offset(C::Al,true,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb19f8: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // ARM 0xb19fc: str r0, [r6, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,56)?)?;
    // ARM 0xb1a00: bne 0xb1778 <entry+0x5028>
    c.b(C::Ne,0xb1778,false)?;
    // ARM 0xb1a04: b 0xb1660 <entry+0x4f10>
    c.b(C::Al,0xb1660,false)?;
    c.check(0xb1a08)?;
    // ARM 0xb1a08: classified non-executable diagnostic/controller data
    c.emit(0x000f8928)?;
    // ARM 0xb1a0c: classified non-executable diagnostic/controller data
    c.emit(0x000e2788)?;
    // ARM 0xb1a10: classified non-executable diagnostic/controller data
    c.emit(0x000f88c7)?;
    // ARM 0xb1a14: classified non-executable diagnostic/controller data
    c.emit(0x05f5e100)?;
    // ARM 0xb1a18: classified non-executable diagnostic/controller data
    c.emit(0x0010408c)?;
    // ARM 0xb1a1c: classified non-executable diagnostic/controller data
    c.emit(0x000f8965)?;
    // ARM 0xb1a20: classified non-executable diagnostic/controller data
    c.emit(0x000f897e)?;
    // ARM 0xb1a24: classified non-executable diagnostic/controller data
    c.emit(0x000f7058)?;
    // ARM 0xb1a28: classified non-executable diagnostic/controller data
    c.emit(0x000f7095)?;
    c.check(0xb1a2c)?;
    // ARM 0xb1a2c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb1a30: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xb1a34: str r3, [sp, #-0x4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::new(3)?,R::SP,-4)?)?;
    // ARM 0xb1a38: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1a3c: bge 0xb1b0c <entry+0x53bc>
    c.b(C::Ge,0xb1b0c,false)?;
    // ARM 0xb1a40: ldr r0, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,36)?)?;
    // ARM 0xb1a44: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // ARM 0xb1a48: add r12, r1, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R1,R::R2,Shift::Lsl,2,false)?)?;
    // ARM 0xb1a4c: add r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // ARM 0xb1a50: mov r4, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,2048)?)?;
    // ARM 0xb1a54: mov r5, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // ARM 0xb1a58: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // ARM 0xb1a5c: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // ARM 0xb1a60: orr r7, r7, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2048)?)?;
    // ARM 0xb1a64: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // ARM 0xb1a68: orr r9, r4, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(9)?,R::new(4)?,R::R0,Shift::Lsl,15,false)?)?;
    // ARM 0xb1a6c: orr lr, r7, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::new(7)?,R::R0,Shift::Lsl,15,false)?)?;
    // ARM 0xb1a70: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // ARM 0xb1a74: mov r7, r9
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // ARM 0xb1a78: mov r6, lr
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::LR))?;
    // ARM 0xb1a7c: str r10, [r7, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(10)?,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb1a80: str r10, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(7)?,0)?)?;
    // ARM 0xb1a84: add r7, r1, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1a88: ldr r8, [r6, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(8)?,R::new(6)?,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb1a8c: strb r8, [r7, #-0x8]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(7)?,-8)?)?;
    // ARM 0xb1a90: tst r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,255)?)?;
    // ARM 0xb1a94: ldr r7, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,0)?)?;
    // ARM 0xb1a98: strb r7, [r1, r4]
    c.emit(arm32::transfer_byte_register_offset(C::Al,false,R::new(7)?,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1a9c: beq 0xb1ac8 <entry+0x5378>
    c.b(C::Eq,0xb1ac8,false)?;
    // ARM 0xb1aa0: mov r6, r9
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // ARM 0xb1aa4: mov r7, lr
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::LR))?;
    // ARM 0xb1aa8: str r3, [r6, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::new(6)?,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb1aac: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // ARM 0xb1ab0: add r6, r12, r4, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R12,R::new(4)?,Shift::Lsl,2,false)?)?;
    // ARM 0xb1ab4: ldr r0, [r7, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb1ab8: str r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // ARM 0xb1abc: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // ARM 0xb1ac0: str r0, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,48)?)?;
    // ARM 0xb1ac4: ldrb r7, [r1, r4]
    c.emit(arm32::transfer_byte_register_offset(C::Al,true,R::new(7)?,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // ARM 0xb1ac8: tst r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,255)?)?;
    // ARM 0xb1acc: beq 0xb1af8 <entry+0x53a8>
    c.b(C::Eq,0xb1af8,false)?;
    // ARM 0xb1ad0: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // ARM 0xb1ad4: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // ARM 0xb1ad8: str r6, [r0, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb1adc: mov r6, lr
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::LR))?;
    // ARM 0xb1ae0: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // ARM 0xb1ae4: add r0, r12, r4, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::new(4)?,Shift::Lsl,2,false)?)?;
    // ARM 0xb1ae8: ldr r7, [r6, r5]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::new(7)?,R::new(6)?,R::new(5)?,Shift::Lsl,0)?)?;
    // ARM 0xb1aec: str r7, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,80)?)?;
    // ARM 0xb1af0: ldr r6, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(6)?,0)?)?;
    // ARM 0xb1af4: str r6, [r0, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,112)?)?;
    // ARM 0xb1af8: ldr r0, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // ARM 0xb1afc: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // ARM 0xb1b00: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // ARM 0xb1b04: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xb1b08: bne 0xb1a74 <entry+0x5324>
    c.b(C::Ne,0xb1a74,false)?;
    // ARM 0xb1b0c: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xb1b10: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xb1b14: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xb1b18: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // ARM 0xb1b1c: ldr r1, [pc, #0x18]
    c.literal(R::R1,0xb1b3c)?;
    // ARM 0xb1b20: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb1b24: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1b28: ldr r1, [pc, #0x10]
    c.literal(R::R1,0xb1b40)?;
    // ARM 0xb1b2c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // ARM 0xb1b30: bl 0x2ec94 <entry+0xfffffffffff82544>
    c.b(C::Al,0x2ec94,true)?;
    // ARM 0xb1b34: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xb1b38: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb1b3c)?;
    // ARM 0xb1b3c: classified non-executable diagnostic/controller data
    c.emit(0x000f70f5)?;
    // ARM 0xb1b40: classified non-executable diagnostic/controller data
    c.emit(0x000e2788)?;
    c.check(0xb1b44)
}
