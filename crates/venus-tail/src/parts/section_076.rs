
/// Check codec plane channel resources and notify the controller of
/// active/inactive input paths. The original five data pointers reside
/// after a branch-only cleanup return and are not A32 instructions.
fn validate_controller_profile_buffer_availability(c:&mut Code)->Result<(),String>{
    c.check(0xc11bc)?;
    // VMA 0xc11bc: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc11c0: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xc11c4: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xc11c8: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // VMA 0xc11cc: ldr r2, [pc, #0x2a0]        @ 0x2904 <entry+0x2904>
    c.literal(R::R2,0xc1474)?;
    // VMA 0xc11d0: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc11d4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc11d8: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xc11dc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc11e0: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xc11e4: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xc11e8: and r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc11ec: mov r1, #83
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,83)?)?;
    // VMA 0xc11f0: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xc11f4: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbfa4c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc11f8: add r0, r4, #252
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,252)?)?;
    // VMA 0xc11fc: ldr r7, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,0)?)?;
    // VMA 0xc1200: add r1, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1024)?)?;
    // VMA 0xc1204: add r0, r4, #248
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,248)?)?;
    // VMA 0xc1208: add r5, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,1024)?)?;
    // VMA 0xc120c: ldr r0, [r6, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,368)?)?;
    // VMA 0xc1210: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xc1214: bl 0x7f974 <entry+0xfffffffffffc0e04> @ imm = #-0x418a8
    c.b(C::Al,0x7f974,true)?;
    // VMA 0xc1218: ldr r0, [r4, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1272)?)?;
    // VMA 0xc121c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1220: beq 0xc133c <entry+0x27cc>  @ imm = #0x114
    c.b(C::Eq,0xc133c,false)?;
    // VMA 0xc1224: ldr r0, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,16)?)?;
    // VMA 0xc1228: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // VMA 0xc122c: beq 0xc136c <entry+0x27fc>  @ imm = #0x138
    c.b(C::Eq,0xc136c,false)?;
    // VMA 0xc1230: cmp r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16384)?)?;
    // VMA 0xc1234: bne 0xc1470 <entry+0x2900>  @ imm = #0x234
    c.b(C::Ne,0xc1470,false)?;
    // VMA 0xc1238: ldr r1, [r4, #0x818]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2072)?)?;
    // VMA 0xc123c: ldr r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1296)?)?;
    // VMA 0xc1240: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xc1244: bne 0xc1250 <entry+0x26e0>  @ imm = #0x4
    c.b(C::Ne,0xc1250,false)?;
    // VMA 0xc1248: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc124c: ldrbeq r0, [r4, #0x54b]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R0,R::new(4)?,1355)?)?;
    // VMA 0xc1250: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1254: beq 0xc127c <entry+0x270c>  @ imm = #0x20
    c.b(C::Eq,0xc127c,false)?;
    // VMA 0xc1258: ldrb r0, [r4, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1355)?)?;
    // VMA 0xc125c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1260: bne 0xc127c <entry+0x270c>  @ imm = #0x14
    c.b(C::Ne,0xc127c,false)?;
    // VMA 0xc1264: ldrb r0, [r4, #0x522]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1314)?)?;
    // VMA 0xc1268: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc126c: bne 0xc127c <entry+0x270c>  @ imm = #0x8
    c.b(C::Ne,0xc127c,false)?;
    // VMA 0xc1270: ldrb r0, [r4, #0x54c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1356)?)?;
    // VMA 0xc1274: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xc1278: bne 0xc12f0 <entry+0x2780>  @ imm = #0x70
    c.b(C::Ne,0xc12f0,false)?;
    // VMA 0xc127c: add r0, r4, #260
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,260)?)?;
    // VMA 0xc1280: add r2, r4, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,1280)?)?;
    // VMA 0xc1284: add r1, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1024)?)?;
    // VMA 0xc1288: ldr r0, [r6, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,368)?)?;
    // VMA 0xc128c: bl 0x7f974 <entry+0xfffffffffffc0e04> @ imm = #-0x41920
    c.b(C::Al,0x7f974,true)?;
    // VMA 0xc1290: ldr r0, [r4, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1280)?)?;
    // VMA 0xc1294: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1298: bne 0xc12c8 <entry+0x2758>  @ imm = #0x28
    c.b(C::Ne,0xc12c8,false)?;
    // VMA 0xc129c: ldr r1, [pc, #0x1d4]        @ 0x2908 <entry+0x2908>
    c.literal(R::R1,0xc1478)?;
    // VMA 0xc12a0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc12a4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92618
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc12a8: ldr r1, [pc, #0x1cc]        @ 0x290c <entry+0x290c>
    c.literal(R::R1,0xc147c)?;
    // VMA 0xc12ac: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc12b0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92624
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc12b4: ldr r2, [pc, #0x1b8]        @ 0x2904 <entry+0x2904>
    c.literal(R::R2,0xc1474)?;
    // VMA 0xc12b8: mov r1, #888
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,888)?)?;
    // VMA 0xc12bc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc12c0: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xc12c4: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbfb1c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc12c8: ldr r2, [r6, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,472)?)?;
    // VMA 0xc12cc: ldr r1, [pc, #0x1ac]        @ 0x2910 <entry+0x2910>
    c.literal(R::R1,0xc1480)?;
    // VMA 0xc12d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc12d4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92648
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc12d8: ldr r1, [pc, #0x19c]        @ 0x290c <entry+0x290c>
    c.literal(R::R1,0xc147c)?;
    // VMA 0xc12dc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc12e0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92654
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc12e4: ldr r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1296)?)?;
    // VMA 0xc12e8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc12ec: beq 0xc12fc <entry+0x278c>  @ imm = #0x8
    c.b(C::Eq,0xc12fc,false)?;
    // VMA 0xc12f0: ldrb r0, [r4, #0x54b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1355)?)?;
    // VMA 0xc12f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc12f8: beq 0xc1430 <entry+0x28c0>  @ imm = #0x130
    c.b(C::Eq,0xc1430,false)?;
    // VMA 0xc12fc: ldr r1, [pc, #0x180]        @ 0x2914 <entry+0x2914>
    c.literal(R::R1,0xc1484)?;
    // VMA 0xc1300: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc1304: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92678
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1308: ldr r1, [pc, #0x16c]        @ 0x290c <entry+0x290c>
    c.literal(R::R1,0xc147c)?;
    // VMA 0xc130c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc1310: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92684
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1314: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc1318: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc131c: beq 0xc1448 <entry+0x28d8>  @ imm = #0x124
    c.b(C::Eq,0xc1448,false)?;
    // VMA 0xc1320: bl 0xc65ac <entry+0x7a3c>  @ imm = #0x5284
    c.b(C::Al,0xc65ac,true)?;
    // VMA 0xc1324: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc1328: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc132c: beq 0xc1448 <entry+0x28d8>  @ imm = #0x114
    c.b(C::Eq,0xc1448,false)?;
    // VMA 0xc1330: mov r1, #3648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3648)?)?;
    // VMA 0xc1334: bl 0x205c <entry+0xfffffffffff434ec> @ imm = #-0xbf2e0
    c.b(C::Al,0x205c,true)?;
    // VMA 0xc1338: b 0xc1448 <entry+0x28d8>  @ imm = #0x108
    c.b(C::Al,0xc1448,false)?;
    // VMA 0xc133c: ldr r1, [pc, #0x134]        @ 0x2908 <entry+0x2908>
    c.literal(R::R1,0xc1478)?;
    // VMA 0xc1340: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc1344: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x926b8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1348: ldr r1, [pc, #0x12c]        @ 0x290c <entry+0x290c>
    c.literal(R::R1,0xc147c)?;
    // VMA 0xc134c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc1350: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x926c4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc1354: ldr r2, [pc, #0x118]        @ 0x2904 <entry+0x2904>
    c.literal(R::R2,0xc1474)?;
    // VMA 0xc1358: mov r1, #94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,94)?)?;
    // VMA 0xc135c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc1360: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xc1364: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc1368: b 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbfbc0
    c.b(C::Al,0x17b0,false)?;
    // VMA 0xc136c: ldr r0, [r4, #0x818]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2072)?)?;
    // VMA 0xc1370: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc1374: bne 0xc13e8 <entry+0x2878>  @ imm = #0x6c
    c.b(C::Ne,0xc13e8,false)?;
    // VMA 0xc1378: add r0, r4, #260
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,260)?)?;
    // VMA 0xc137c: add r1, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1024)?)?;
    // VMA 0xc1380: ldr r0, [r6, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,368)?)?;
    // VMA 0xc1384: add r6, r4, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(4)?,1280)?)?;
    // VMA 0xc1388: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xc138c: bl 0x7f974 <entry+0xfffffffffffc0e04> @ imm = #-0x41a20
    c.b(C::Al,0x7f974,true)?;
    // VMA 0xc1390: ldr r1, [r4, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1280)?)?;
    // VMA 0xc1394: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc1398: bne 0xc13cc <entry+0x285c>  @ imm = #0x2c
    c.b(C::Ne,0xc13cc,false)?;
    // VMA 0xc139c: ldr r1, [pc, #0xd4]         @ 0x2908 <entry+0x2908>
    c.literal(R::R1,0xc1478)?;
    // VMA 0xc13a0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc13a4: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92718
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc13a8: ldr r1, [pc, #0xcc]         @ 0x290c <entry+0x290c>
    c.literal(R::R1,0xc147c)?;
    // VMA 0xc13ac: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc13b0: bl 0x2ec94 <entry+0xfffffffffff70124> @ imm = #-0x92724
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc13b4: ldr r2, [pc, #0xb8]         @ 0x2904 <entry+0x2904>
    c.literal(R::R2,0xc1474)?;
    // VMA 0xc13b8: mov r1, #179
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,179)?)?;
    // VMA 0xc13bc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc13c0: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xc13c4: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbfc1c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc13c8: ldrb r1, [r6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // VMA 0xc13cc: ldr r2, [pc, #0xa0]         @ 0x2904 <entry+0x2904>
    c.literal(R::R2,0xc1474)?;
    // VMA 0xc13d0: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // VMA 0xc13d4: mov r1, #181
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,181)?)?;
    // VMA 0xc13d8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc13dc: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc13e0: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xc13e4: bl 0x17b0 <entry+0xfffffffffff42c40> @ imm = #-0xbfc3c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xc13e8: ldr r0, [r4, #0x510]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1296)?)?;
    // VMA 0xc13ec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc13f0: beq 0xc1408 <entry+0x2898>  @ imm = #0x10
    c.b(C::Eq,0xc1408,false)?;
    // VMA 0xc13f4: ldr r0, [r4, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1280)?)?;
    // VMA 0xc13f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc13fc: popeq {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc1400: bl 0xc643c <entry+0x78cc>  @ imm = #0x5034
    c.b(C::Al,0xc643c,true)?;
    // VMA 0xc1404: b 0xc1458 <entry+0x28e8>  @ imm = #0x4c
    c.b(C::Al,0xc1458,false)?;
    // VMA 0xc1408: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc140c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1410: beq 0xc13f4 <entry+0x2884>  @ imm = #-0x24
    c.b(C::Eq,0xc13f4,false)?;
    // VMA 0xc1414: bl 0xc643c <entry+0x78cc>  @ imm = #0x5020
    c.b(C::Al,0xc643c,true)?;
    // VMA 0xc1418: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc141c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1420: beq 0xc13f4 <entry+0x2884>  @ imm = #-0x34
    c.b(C::Eq,0xc13f4,false)?;
    // VMA 0xc1424: mov r1, #3648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3648)?)?;
    // VMA 0xc1428: bl 0x205c <entry+0xfffffffffff434ec> @ imm = #-0xbf3d4
    c.b(C::Al,0x205c,true)?;
    // VMA 0xc142c: b 0xc13f4 <entry+0x2884>  @ imm = #-0x40
    c.b(C::Al,0xc13f4,false)?;
    // VMA 0xc1430: ldrb r0, [r4, #0x522]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1314)?)?;
    // VMA 0xc1434: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1438: beq 0xc1448 <entry+0x28d8>  @ imm = #0x8
    c.b(C::Eq,0xc1448,false)?;
    // VMA 0xc143c: ldrb r0, [r4, #0x520]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1312)?)?;
    // VMA 0xc1440: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1444: beq 0xc12fc <entry+0x278c>  @ imm = #-0x150
    c.b(C::Eq,0xc12fc,false)?;
    // VMA 0xc1448: ldr r0, [r4, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1280)?)?;
    // VMA 0xc144c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1450: beq 0xc1470 <entry+0x2900>  @ imm = #0x18
    c.b(C::Eq,0xc1470,false)?;
    // VMA 0xc1454: bl 0xc65ac <entry+0x7a3c>  @ imm = #0x5150
    c.b(C::Al,0xc65ac,true)?;
    // VMA 0xc1458: ldr r0, [r4, #0x500]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1280)?)?;
    // VMA 0xc145c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc1460: beq 0xc1470 <entry+0x2900>  @ imm = #0x8
    c.b(C::Eq,0xc1470,false)?;
    // VMA 0xc1464: mov r1, #3648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3648)?)?;
    // VMA 0xc1468: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xc146c: b 0x205c <entry+0xfffffffffff434ec> @ imm = #-0xbf418
    c.b(C::Al,0x205c,false)?;
    // VMA 0xc1470: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc1474)?;
    const CONTROLLER_RESOURCE_CHECK_MESSAGES:[u32;5]=[
        0x000fa71c,0x000fab8e,0x000e2788,0x000fabba,0x000fabdc,
    ];
    for value in CONTROLLER_RESOURCE_CHECK_MESSAGES {c.emit(value)?;}
    c.check(0xc1488)
}

/// Release codec context handles and reset the two channel state records
/// using the actual hardware cleanup callbacks and caller epilogue.
fn release_controller_profile_buffer_handles(c:&mut Code)->Result<(),String>{
    c.check(0xc1488)?;
    // VMA 0xc1488: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::DecrementBefore,true)?)?;
    // VMA 0xc148c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xc1490: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xc1494: ldr r5, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,4)?)?;
    // VMA 0xc1498: add r0, r5, #364
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,364)?)?;
    // VMA 0xc149c: bl 0x7e7d4 <entry+0xfffffffffffbfc64> @ imm = #-0x42cd0
    c.b(C::Al,0x7e7d4,true)?;
    // VMA 0xc14a0: ldr r0, [r5, #0x288]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,648)?)?;
    // VMA 0xc14a4: bl 0x2f1c <entry+0xfffffffffff443ac> @ imm = #-0xbe590
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xc14a8: ldr r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,644)?)?;
    // VMA 0xc14ac: bl 0x2f1c <entry+0xfffffffffff443ac> @ imm = #-0xbe598
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xc14b0: ldr r0, [r5, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,640)?)?;
    // VMA 0xc14b4: bl 0x2f1c <entry+0xfffffffffff443ac> @ imm = #-0xbe5a0
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xc14b8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc14bc: bl 0x2f1c <entry+0xfffffffffff443ac> @ imm = #-0xbe5a8
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xc14c0: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc14c4: bl 0x2f1c <entry+0xfffffffffff443ac> @ imm = #-0xbe5b0
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xc14c8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xc14cc: bl 0xd0f18 <entry+0x123a8> @ imm = #0xfa44
    c.b(C::Al,0xd0f18,true)?;
    // VMA 0xc14d0: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xc14d4: add r5, r5, #172
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,172)?)?;
    // VMA 0xc14d8: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xc14dc: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xc14e0: bl 0xd9e10 <entry+0x1b2a0> @ imm = #0x18928
    c.b(C::Al,0xd9e10,true)?;
    // VMA 0xc14e4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc14e8: bl 0xd9d44 <entry+0x1b1d4> @ imm = #0x18854
    c.b(C::Al,0xd9d44,true)?;
    // VMA 0xc14ec: ldr r0, [r5, #0x8]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // VMA 0xc14f0: bl 0xd9e10 <entry+0x1b2a0> @ imm = #0x18918
    c.b(C::Al,0xd9e10,true)?;
    // VMA 0xc14f4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc14f8: bl 0xd9d44 <entry+0x1b1d4> @ imm = #0x18844
    c.b(C::Al,0xd9d44,true)?;
    // VMA 0xc14fc: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xc1500: sub r5, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::new(5)?,4)?)?;
    // VMA 0xc1504: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // VMA 0xc1508: bne 0xc14dc <entry+0x296c>  @ imm = #-0x34
    c.b(C::Ne,0xc14dc,false)?;
    // VMA 0xc150c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xc1510: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xc1514)
}
