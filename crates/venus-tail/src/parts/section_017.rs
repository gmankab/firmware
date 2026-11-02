
/// Populate codec session configuration, conditional reference-buffer copies and hardware command fields with validated input dimensions.
/// Runtime slice 0xad728..0xadc14, with 1 classified trailing data words.
fn construct_codec_configuration_record(c:&mut Code)->Result<(),String>{
    c.check(0xad728)?;
    // ARM 0xad728: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM 0xad72c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // ARM 0xad730: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // ARM 0xad734: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // ARM 0xad738: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // ARM 0xad73c: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // ARM 0xad740: ldr r8, [r9, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(9)?,2444)?)?;
    // ARM 0xad744: ldr r1, [r8, #0x26c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,620)?)?;
    // ARM 0xad748: blx r1
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    // ARM 0xad74c: ldr r2, [r9, #0xae4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,2788)?)?;
    // ARM 0xad750: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xad754: str r2, [r9, #0xb28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,2856)?)?;
    // ARM 0xad758: ldrb r3, [r9, #0xa22]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,2594)?)?;
    // ARM 0xad75c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xad760: bne 0xad774 <entry+0x1024>
    c.b(C::Ne,0xad774,false)?;
    // ARM 0xad764: ldrb r1, [r9, #0xa20]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,2592)?)?;
    // ARM 0xad768: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xad76c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xad770: beq 0xadbf4 <entry+0x14a4>
    c.b(C::Eq,0xadbf4,false)?;
    // ARM 0xad774: mov r1, #2480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2480)?)?;
    // ARM 0xad778: strh r0, [r9, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(9)?,R::R1)?)?;
    // ARM 0xad77c: str r2, [r9, #0x9b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,2484)?)?;
    // ARM 0xad780: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // ARM 0xad784: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xad788: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // ARM 0xad78c: beq 0xad7a8 <entry+0x1058>
    c.b(C::Eq,0xad7a8,false)?;
    // ARM 0xad790: ldr r0, [r9, #0x9b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2484)?)?;
    // ARM 0xad794: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // ARM 0xad798: bge 0xad7a8 <entry+0x1058>
    c.b(C::Ge,0xad7a8,false)?;
    // ARM 0xad79c: ldrb r0, [r9, #0xa23]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,2595)?)?;
    // ARM 0xad7a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xad7a4: movne r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // ARM 0xad7a8: ldr r6, [r9, #0x9b8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,2488)?)?;
    // ARM 0xad7ac: ldrb r7, [r9, #0xa21]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(9)?,2593)?)?;
    // ARM 0xad7b0: add r0, r9, #2880
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,2880)?)?;
    // ARM 0xad7b4: add r1, r9, #2816
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,2816)?)?;
    // ARM 0xad7b8: cmn r6, #-2147483647
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(6)?,2147483649)?)?;
    // ARM 0xad7bc: beq 0xad7c8 <entry+0x1078>
    c.b(C::Eq,0xad7c8,false)?;
    // ARM 0xad7c0: orrs r3, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,true)?)?;
    // ARM 0xad7c4: beq 0xadbb8 <entry+0x1468>
    c.b(C::Eq,0xadbb8,false)?;
    // ARM 0xad7c8: ldr r6, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,0)?)?;
    // ARM 0xad7cc: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // ARM 0xad7d0: beq 0xad7dc <entry+0x108c>
    c.b(C::Eq,0xad7dc,false)?;
    // ARM 0xad7d4: mov r2, #-2147483647
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2147483649)?)?;
    // ARM 0xad7d8: b 0xad7e8 <entry+0x1098>
    c.b(C::Al,0xad7e8,false)?;
    // ARM 0xad7dc: ldrb r3, [r9, #0xa20]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(9)?,2592)?)?;
    // ARM 0xad7e0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // ARM 0xad7e4: beq 0xadc08 <entry+0x14b8>
    c.b(C::Eq,0xadc08,false)?;
    // ARM 0xad7e8: str r2, [r9, #0x9b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,2488)?)?;
    // ARM 0xad7ec: bl 0xa8db4 <entry+0xffffffffffffc664>
    c.b(C::Al,0xa8db4,true)?;
    // ARM 0xad7f0: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // ARM 0xad7f4: str r7, [r6, #0x448]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,1096)?)?;
    // ARM 0xad7f8: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,0)?)?;
    // ARM 0xad7fc: str r0, [r9, #0xae0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,2784)?)?;
    // ARM 0xad800: ldr r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // ARM 0xad804: str r0, [r7, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,48)?)?;
    // ARM 0xad808: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xad80c: ldrb r1, [r9, #0xa21]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,2593)?)?;
    // ARM 0xad810: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xad814: bne 0xad824 <entry+0x10d4>
    c.b(C::Ne,0xad824,false)?;
    // ARM 0xad818: ldrb r0, [r9, #0xa22]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,2594)?)?;
    // ARM 0xad81c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xad820: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xad824: ldr r1, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,4)?)?;
    // ARM 0xad828: ldrb r2, [r8, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(8)?,111)?)?;
    // ARM 0xad82c: orr r0, r0, r1, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,8,false)?)?;
    // ARM 0xad830: ldrb r1, [r8, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,110)?)?;
    // ARM 0xad834: orr r0, r0, r1, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,4,false)?)?;
    // ARM 0xad838: orr r0, r0, r2, lsl #6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,6,false)?)?;
    // ARM 0xad83c: str r0, [r7, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,40)?)?;
    // ARM 0xad840: ldrh r0, [r8, #98]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(8)?,98)?)?;
    // ARM 0xad844: str r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // ARM 0xad848: ldrh r0, [r8, #100]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(8)?,100)?)?;
    // ARM 0xad84c: str r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // ARM 0xad850: ldr r0, [r9, #0x25c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,604)?)?;
    // ARM 0xad854: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xad858: beq 0xad880 <entry+0x1130>
    c.b(C::Eq,0xad880,false)?;
    // ARM 0xad85c: ldr r0, [r9, #0xf9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,3996)?)?;
    // ARM 0xad860: ldr r1, [r0, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,992)?)?;
    // ARM 0xad864: str r1, [r7, #0x75c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,1884)?)?;
    // ARM 0xad868: ldr r1, [r0, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,996)?)?;
    // ARM 0xad86c: str r1, [r7, #0x758]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,1880)?)?;
    // ARM 0xad870: ldr r1, [r0, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,984)?)?;
    // ARM 0xad874: str r1, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,12)?)?;
    // ARM 0xad878: ldr r0, [r0, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,988)?)?;
    // ARM 0xad87c: str r0, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,16)?)?;
    // ARM 0xad880: ldrh r0, [r8, #106]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(8)?,106)?)?;
    // ARM 0xad884: mov r1, #868
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,868)?)?;
    // ARM 0xad888: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // ARM 0xad88c: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // ARM 0xad890: strh r0, [r7, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(7)?,R::R1)?)?;
    // ARM 0xad894: mov r1, #1888
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1888)?)?;
    // ARM 0xad898: ldrh r0, [r8, #102]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(8)?,102)?)?;
    // ARM 0xad89c: strh r0, [r7, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(7)?,R::R1)?)?;
    // ARM 0xad8a0: mov r1, #102
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,102)?)?;
    // ARM 0xad8a4: ldrh r0, [r8, #108]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(8)?,108)?)?;
    // ARM 0xad8a8: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // ARM 0xad8ac: strh r0, [r7, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(7)?,R::R1)?)?;
    // ARM 0xad8b0: mov r1, #98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,98)?)?;
    // ARM 0xad8b4: ldrh r0, [r8, #104]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(8)?,104)?)?;
    // ARM 0xad8b8: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // ARM 0xad8bc: strh r0, [r7, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(7)?,R::R1)?)?;
    // ARM 0xad8c0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xad8c4: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // ARM 0xad8c8: str r0, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,20)?)?;
    // ARM 0xad8cc: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // ARM 0xad8d0: str r0, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,24)?)?;
    // ARM 0xad8d4: ldr r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // ARM 0xad8d8: str r0, [r7, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,28)?)?;
    // ARM 0xad8dc: ldr r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    // ARM 0xad8e0: str r0, [r7, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,32)?)?;
    // ARM 0xad8e4: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // ARM 0xad8e8: str r0, [r7, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,36)?)?;
    // ARM 0xad8ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xad8f0: str r0, [r7, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,44)?)?;
    // ARM 0xad8f4: add r0, r7, #872
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,872)?)?;
    // ARM 0xad8f8: str r5, [r6, #0x44c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,1100)?)?;
    // ARM 0xad8fc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // ARM 0xad900: add r0, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // ARM 0xad904: bl 0x31ec <entry+0xfffffffffff56a9c>
    c.b(C::Al,0x31ec,true)?;
    // ARM 0xad908: ldr r0, [r9, #0x150]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,336)?)?;
    // ARM 0xad90c: str r0, [r7, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,52)?)?;
    // ARM 0xad910: ldr r0, [r9, #0x154]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,340)?)?;
    // ARM 0xad914: str r0, [r7, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,56)?)?;
    // ARM 0xad918: mov r0, #668
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,668)?)?;
    // ARM 0xad91c: ldr r1, [r9, #0x98c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2444)?)?;
    // ARM 0xad920: signed byte load, not ordinary LDRB.
    c.emit(arm32::load_signed_byte_register(C::Al,R::R0,R::R1,R::R0)?)?;
    // ARM 0xad924: str r0, [r7, #0x7c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,1992)?)?;
    // ARM 0xad928: ldr r0, [r1, #0x290]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,656)?)?;
    // ARM 0xad92c: str r0, [r7, #0x7cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,1996)?)?;
    // ARM 0xad930: ldr r6, [r1, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,664)?)?;
    // ARM 0xad934: str r6, [r7, #0x7d0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,2000)?)?;
    // ARM 0xad938: ldr r2, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,40)?)?;
    // ARM 0xad93c: str r5, [r7, #0x7e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,2016)?)?;
    // ARM 0xad940: str r2, [r7, #0x7e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,2020)?)?;
    // ARM 0xad944: ldrb r2, [r1, #0x262]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,610)?)?;
    // ARM 0xad948: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xad94c: beq 0xad9c8 <entry+0x1278>
    c.b(C::Eq,0xad9c8,false)?;
    // ARM 0xad950: ldrb r2, [r1, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,110)?)?;
    // ARM 0xad954: add r2, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,8)?)?;
    // ARM 0xad958: str r2, [r7, #0x79c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,1948)?)?;
    // ARM 0xad95c: ldrb r2, [r1, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,111)?)?;
    // ARM 0xad960: add r2, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,8)?)?;
    // ARM 0xad964: str r2, [r7, #0x7a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,1952)?)?;
    // ARM 0xad968: ldrb r2, [r1, #0x60]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,96)?)?;
    // ARM 0xad96c: str r2, [r7, #0x7b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,1968)?)?;
    // ARM 0xad970: ldrb r2, [r1, #0x61]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,97)?)?;
    // ARM 0xad974: str r2, [r7, #0x7b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,1972)?)?;
    // ARM 0xad978: ldrb r2, [r1, #0x6dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,1756)?)?;
    // ARM 0xad97c: strb r2, [r7, #0x794]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(7)?,1940)?)?;
    // ARM 0xad980: ldr r2, [r1, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,1760)?)?;
    // ARM 0xad984: str r2, [r7, #0x798]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,1944)?)?;
    // ARM 0xad988: ldrb r2, [r1, #0x6dd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,1757)?)?;
    // ARM 0xad98c: strb r2, [r7, #0x795]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(7)?,1941)?)?;
    // ARM 0xad990: ldrb r2, [r1, #0x6de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,1758)?)?;
    // ARM 0xad994: strb r2, [r7, #0x796]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(7)?,1942)?)?;
    // ARM 0xad998: ldr r2, [r1, #0x6e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,1764)?)?;
    // ARM 0xad99c: str r2, [r7, #0x7a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,1956)?)?;
    // ARM 0xad9a0: ldr r2, [r1, #0x6e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,1768)?)?;
    // ARM 0xad9a4: str r2, [r7, #0x7a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,1960)?)?;
    // ARM 0xad9a8: ldr r2, [r1, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,1772)?)?;
    // ARM 0xad9ac: str r2, [r7, #0x7ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,1964)?)?;
    // ARM 0xad9b0: ldrb r2, [r1, #0x6df]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,1759)?)?;
    // ARM 0xad9b4: strb r2, [r7, #0x797]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(7)?,1943)?)?;
    // ARM 0xad9b8: ldr r2, [r1, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,1776)?)?;
    // ARM 0xad9bc: str r2, [r7, #0x7b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,1976)?)?;
    // ARM 0xad9c0: ldr r1, [r1, #0x6f4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,1780)?)?;
    // ARM 0xad9c4: str r1, [r7, #0x7bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,1980)?)?;
    // ARM 0xad9c8: mov r1, #267386880
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,267386880)?)?;
    // ARM 0xad9cc: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // ARM 0xad9d0: orr r1, r1, #-1342177280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2952790016)?)?;
    // ARM 0xad9d4: cmnne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmn,R::R0,R::R0,1)?)?;
    // ARM 0xad9d8: beq 0xada08 <entry+0x12b8>
    c.b(C::Eq,0xada08,false)?;
    // ARM 0xad9dc: blx 0x82de4 <entry+0xfffffffffffd6694>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82de4)?)?;
    // ARM 0xad9e0: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xad9e4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // ARM 0xad9e8: mov r10, r1
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // ARM 0xad9ec: blx 0x82de4 <entry+0xfffffffffffd6694>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82de4)?)?;
    // ARM 0xad9f0: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // ARM 0xad9f4: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // ARM 0xad9f8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM 0xad9fc: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // ARM 0xada00: blx 0x823b0 <entry+0xfffffffffffd5c60>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x823b0)?)?;
    // ARM 0xada04: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM 0xada08: str r5, [r7, #0x7d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,2008)?)?;
    // ARM 0xada0c: str r1, [r7, #0x7dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,2012)?)?;
    // ARM 0xada10: mov r1, #648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,648)?)?;
    // ARM 0xada14: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // ARM 0xada18: ldrh r1, [r8, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(8)?,R::R1)?)?;
    // ARM 0xada1c: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // ARM 0xada20: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM 0xada24: beq 0xada4c <entry+0x12fc>
    c.b(C::Eq,0xada4c,false)?;
    // ARM 0xada28: mov r1, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,138)?)?;
    // ARM 0xada2c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // ARM 0xada30: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // ARM 0xada34: ldrh r1, [r8, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(8)?,R::R1)?)?;
    // ARM 0xada38: str r2, [r7, #0x7c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,1984)?)?;
    // ARM 0xada3c: cmp r1, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,13)?)?;
    // ARM 0xada40: blo 0xadb88 <entry+0x1438>
    c.b(C::Cc,0xadb88,false)?;
    // ARM 0xada44: cmp r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,255)?)?;
    // ARM 0xada48: beq 0xadb88 <entry+0x1438>
    c.b(C::Eq,0xadb88,false)?;
    // ARM 0xada4c: mov r1, #964
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,964)?)?;
    // ARM 0xada50: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // ARM 0xada54: strh r0, [r7, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(7)?,R::R1)?)?;
    // ARM 0xada58: ldr r5, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,0)?)?;
    // ARM 0xada5c: mov r1, #198
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,198)?)?;
    // ARM 0xada60: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // ARM 0xada64: strh r0, [r7, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(7)?,R::R1)?)?;
    // ARM 0xada68: ldr r10, [r9, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(9)?,2528)?)?;
    // ARM 0xada6c: str r10, [r7, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(7)?,60)?)?;
    // ARM 0xada70: ldr r0, [r5, #0x44c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1100)?)?;
    // ARM 0xada74: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xada78: bne 0xada8c <entry+0x133c>
    c.b(C::Ne,0xada8c,false)?;
    // ARM 0xada7c: ldrb r0, [r4, #0x14]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,20)?)?;
    // ARM 0xada80: tst r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,128)?)?;
    // ARM 0xada84: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xada88: streq r0, [r9, #0x9e0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(9)?,2528)?)?;
    // ARM 0xada8c: tst r10, #4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(10)?,4)?)?;
    // ARM 0xada90: beq 0xadaa4 <entry+0x1354>
    c.b(C::Eq,0xadaa4,false)?;
    // ARM 0xada94: ldr r1, [r9, #0x9e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2532)?)?;
    // ARM 0xada98: add r0, r7, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,64)?)?;
    // ARM 0xada9c: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // ARM 0xadaa0: bl 0x471c <entry+0xfffffffffff57fcc>
    c.b(C::Al,0x471c,true)?;
    // ARM 0xadaa4: tst r10, #64
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(10)?,64)?)?;
    // ARM 0xadaa8: beq 0xadac8 <entry+0x1378>
    c.b(C::Eq,0xadac8,false)?;
    // ARM 0xadaac: ldr r0, [r9, #0x9e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2536)?)?;
    // ARM 0xadab0: add r1, r7, #136
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(7)?,136)?)?;
    // ARM 0xadab4: ldm r0!, {r2, r3, r4, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xadab8: stm r1!, {r2, r3, r4, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xadabc: ldm r0, {r2, r3, r4, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xadac0: stm r1, {r2, r3, r4, r5, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // ARM 0xadac4: ldr r5, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,0)?)?;
    // ARM 0xadac8: tst r10, #256
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(10)?,256)?)?;
    // ARM 0xadacc: beq 0xadae4 <entry+0x1394>
    c.b(C::Eq,0xadae4,false)?;
    // ARM 0xadad0: ldr r1, [r9, #0x9f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2552)?)?;
    // ARM 0xadad4: mov r2, #668
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,668)?)?;
    // ARM 0xadad8: add r0, r7, #176
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,176)?)?;
    // ARM 0xadadc: orr r2, r2, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1024)?)?;
    // ARM 0xadae0: bl 0x471c <entry+0xfffffffffff57fcc>
    c.b(C::Al,0x471c,true)?;
    // ARM 0xadae4: tst r10, #128
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(10)?,128)?)?;
    // ARM 0xadae8: ldrne r0, [r9, #0x9ec]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(9)?,2540)?)?;
    // ARM 0xadaec: strne r0, [r7, #0x78c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(7)?,1932)?)?;
    // ARM 0xadaf0: ldrne r0, [r9, #0x9f0]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(9)?,2544)?)?;
    // ARM 0xadaf4: strne r0, [r7, #0x790]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(7)?,1936)?)?;
    // ARM 0xadaf8: tst r10, #512
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(10)?,512)?)?;
    // ARM 0xadafc: ldrne r0, [r9, #0x9f4]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(9)?,2548)?)?;
    // ARM 0xadb00: strne r0, [r7, #0x74c]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(7)?,1868)?)?;
    // ARM 0xadb04: tst r10, #32
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(10)?,32)?)?;
    // ARM 0xadb08: beq 0xadb24 <entry+0x13d4>
    c.b(C::Eq,0xadb24,false)?;
    // ARM 0xadb0c: ldr r0, [r7, #0x750]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,1872)?)?;
    // ARM 0xadb10: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xadb14: bne 0xadb24 <entry+0x13d4>
    c.b(C::Ne,0xadb24,false)?;
    // ARM 0xadb18: ldr r0, [r9, #0x9fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2556)?)?;
    // ARM 0xadb1c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM 0xadb20: strne r0, [r7, #0x750]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(7)?,1872)?)?;
    // ARM 0xadb24: mov r1, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,106)?)?;
    // ARM 0xadb28: mov r0, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,616)?)?;
    // ARM 0xadb2c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // ARM 0xadb30: ldrh r0, [r8, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(8)?,R::R0)?)?;
    // ARM 0xadb34: ldrh r1, [r8, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(8)?,R::R1)?)?;
    // ARM 0xadb38: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // ARM 0xadb3c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM 0xadb40: str r2, [r5, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,128)?)?;
    // ARM 0xadb44: ldr r0, [r8, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,88)?)?;
    // ARM 0xadb48: ldrb r2, [r8, #0x6e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(8)?,110)?)?;
    // ARM 0xadb4c: cmp r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,88)?)?;
    // ARM 0xadb50: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM 0xadb54: and r2, r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,3)?)?;
    // ARM 0xadb58: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM 0xadb5c: orr r0, r10, r0, lsl #31
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(10)?,R::R0,Shift::Lsl,31,false)?)?;
    // ARM 0xadb60: orr r0, r0, r2, lsl #29
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,29,false)?)?;
    // ARM 0xadb64: ldrb r2, [r8, #0x6f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(8)?,111)?)?;
    // ARM 0xadb68: and r2, r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,3)?)?;
    // ARM 0xadb6c: orr r0, r0, r2, lsl #27
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,27,false)?)?;
    // ARM 0xadb70: str r0, [r7, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,60)?)?;
    // ARM 0xadb74: str r1, [r5, #0x470]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,1136)?)?;
    // ARM 0xadb78: str r1, [r5, #0x474]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,1140)?)?;
    // ARM 0xadb7c: ldr r0, [r9, #0xae0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2784)?)?;
    // ARM 0xadb80: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // ARM 0xadb84: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM 0xadb88: ldr r5, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,0)?)?;
    // ARM 0xadb8c: cmp r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,255)?)?;
    // ARM 0xadb90: bne 0xadbd4 <entry+0x1484>
    c.b(C::Ne,0xadbd4,false)?;
    // ARM 0xadb94: mov r0, #652
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,652)?)?;
    // ARM 0xadb98: mov r1, #964
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,964)?)?;
    // ARM 0xadb9c: ldrh r0, [r8, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(8)?,R::R0)?)?;
    // ARM 0xadba0: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // ARM 0xadba4: strh r0, [r7, r1]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(7)?,R::R1)?)?;
    // ARM 0xadba8: mov r0, #142
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,142)?)?;
    // ARM 0xadbac: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // ARM 0xadbb0: ldrh r0, [r8, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(8)?,R::R0)?)?;
    // ARM 0xadbb4: b 0xada5c <entry+0x130c>
    c.b(C::Al,0xada5c,false)?;
    // ARM 0xadbb8: cmp r2, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // ARM 0xadbbc: bge 0xadc00 <entry+0x14b0>
    c.b(C::Ge,0xadc00,false)?;
    // ARM 0xadbc0: ldrb r2, [r9, #0xa23]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,2595)?)?;
    // ARM 0xadbc4: ldr r6, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,0)?)?;
    // ARM 0xadbc8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM 0xadbcc: movne r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // ARM 0xadbd0: b 0xad7ec <entry+0x109c>
    c.b(C::Al,0xad7ec,false)?;
    // ARM 0xadbd4: lsl r0, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Lsl,1)?)?;
    // ARM 0xadbd8: ldr r1, [pc, #0x30]
    c.literal(R::R1,0xadc10)?;
    // ARM 0xadbdc: mov r2, #964
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,964)?)?;
    // ARM 0xadbe0: orr r2, r2, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1024)?)?;
    // ARM 0xadbe4: ldrh r0, [r1, r0]!
    c.emit(arm32::load_halfword_register_preindexed_writeback(C::Al,R::R0,R::R1,R::R0)?)?;
    // ARM 0xadbe8: strh r0, [r7, r2]
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(7)?,R::R2)?)?;
    // ARM 0xadbec: ldrsh r0, [r1, #32]
    c.emit(arm32::load_signed_halfword(C::Al,R::R0,R::R1,32)?)?;
    // ARM 0xadbf0: b 0xada5c <entry+0x130c>
    c.b(C::Al,0xada5c,false)?;
    // ARM 0xadbf4: mov r0, #2480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2480)?)?;
    // ARM 0xadbf8: ldrh r0, [r9, r0]
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(9)?,R::R0)?)?;
    // ARM 0xadbfc: b 0xad780 <entry+0x1030>
    c.b(C::Al,0xad780,false)?;
    // ARM 0xadc00: ldr r6, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,0)?)?;
    // ARM 0xadc04: b 0xad7ec <entry+0x109c>
    c.b(C::Al,0xad7ec,false)?;
    // ARM 0xadc08: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // ARM 0xadc0c: b 0xad7ec <entry+0x109c>
    c.b(C::Al,0xad7ec,false)?;
    c.check(0xadc10)?;
    // ARM 0xadc10: literal or constant data (NOT executable)
    c.emit(0x000f8726)?;
    c.check(0xadc14)
}
