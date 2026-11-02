

/// Reconcile controller buffer resources and conditional fallbacks, then
/// notify the session bridge. Original failure paths reenter previous code.
/// Three isolated diagnostic pointers and one dense 28-word table are data;
/// no firmware instruction bytes are imported as literal opcode arrays.
fn finish_controller_error_routing(c:&mut Code)->Result<(),String>{
    c.check(0xc5fac)?;
    // VMA 0xc5fac: cmp r0, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc5fb0: cmphs r1, r3
    c.emit(arm32::data_register(C::Cs,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xc5fb4: bhs 0xc61c0 <entry+0x4e8>
    c.b(C::Cs,0xc61c0,false)?;
    // VMA 0xc5fb8: ldr r2, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,140)?)?;
    // VMA 0xc5fbc: mov r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,120)?)?;
    // VMA 0xc5fc0: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xc5fc4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc5fc8: beq 0xc5fd8 <entry+0x300>
    c.b(C::Eq,0xc5fd8,false)?;
    // VMA 0xc5fcc: ldr r2, [r4, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,456)?)?;
    // VMA 0xc5fd0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc5fd4: beq 0xc6234 <entry+0x55c>
    c.b(C::Eq,0xc6234,false)?;
    // VMA 0xc5fd8: ldr r2, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,144)?)?;
    // VMA 0xc5fdc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc5fe0: beq 0xc5ff0 <entry+0x318>
    c.b(C::Eq,0xc5ff0,false)?;
    // VMA 0xc5fe4: ldr r2, [r4, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,452)?)?;
    // VMA 0xc5fe8: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // VMA 0xc5fec: beq 0xc6234 <entry+0x55c>
    c.b(C::Eq,0xc6234,false)?;
    // VMA 0xc5ff0: ldr r1, [pc, #0x2dc]
    c.literal(R::R1,0xc62d4)?;
    // VMA 0xc5ff4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc5ff8: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // VMA 0xc5ffc: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // VMA 0xc6000: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc6004: ldr r1, [pc, #0x2bc]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc6008: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc600c: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc6010: ldr r0, [r6, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,972)?)?;
    // VMA 0xc6014: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc6018: beq 0xc5b30 <entry+0xfffffffffffffe58>
    c.b(C::Eq,0xc5b30,false)?;
    // VMA 0xc601c: str r4, [r10, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(10)?,384)?)?;
    // VMA 0xc6020: add r5, sp, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,48)?)?;
    // VMA 0xc6024: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xc6028: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // VMA 0xc602c: ldr r0, [r6, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,992)?)?;
    // VMA 0xc6030: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // VMA 0xc6034: ldr r0, [r6, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,996)?)?;
    // VMA 0xc6038: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // VMA 0xc603c: ldr r0, [r6, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,472)?)?;
    // VMA 0xc6040: ldr r4, [r6, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,468)?)?;
    // VMA 0xc6044: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xc6048: ldr r0, [r6, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,476)?)?;
    // VMA 0xc604c: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // VMA 0xc6050: ldr r0, [r6, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,480)?)?;
    // VMA 0xc6054: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xc6058: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc605c: bl 0x31ec <entry+0xfffffffffff3d514>
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xc6060: ldr r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,112)?)?;
    // VMA 0xc6064: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // VMA 0xc6068: str r0, [r6, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,468)?)?;
    // VMA 0xc606c: ldr r1, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,108)?)?;
    // VMA 0xc6070: str r1, [r6, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,472)?)?;
    // VMA 0xc6074: str r0, [r6, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,476)?)?;
    // VMA 0xc6078: add r0, r6, #280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,280)?)?;
    // VMA 0xc607c: str r1, [r6, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,480)?)?;
    // VMA 0xc6080: add r7, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,2048)?)?;
    // VMA 0xc6084: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xc6088: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xc608c: bl 0x471c <entry+0xfffffffffff3ea44>
    c.b(C::Al,0x471c,true)?;
    // VMA 0xc6090: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xc6094: bl 0xd2adc <entry+0xce04>
    c.b(C::Al,0xd2adc,true)?;
    // VMA 0xc6098: mov r1, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,19)?)?;
    // VMA 0xc609c: orr r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1)?)?;
    // VMA 0xc60a0: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xc60a4: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xc60a8: bne 0xc60d0 <entry+0x3f8>
    c.b(C::Ne,0xc60d0,false)?;
    // VMA 0xc60ac: ldr r1, [pc, #0x224]
    c.literal(R::R1,0xc62d8)?;
    // VMA 0xc60b0: ldr r2, [pc, #0x214]
    c.literal(R::R2,0xc62cc)?;
    // VMA 0xc60b4: mov r3, #59
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,59)?)?;
    // VMA 0xc60b8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc60bc: orr r3, r3, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    // VMA 0xc60c0: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc60c4: ldr r1, [pc, #0x1fc]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc60c8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xc60cc: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc60d0: add r0, r10, #400
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,400)?)?;
    // VMA 0xc60d4: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xc60d8: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // VMA 0xc60dc: bl 0x471c <entry+0xfffffffffff3ea44>
    c.b(C::Al,0x471c,true)?;
    // VMA 0xc60e0: ldr r0, [r6, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,992)?)?;
    // VMA 0xc60e4: add r1, sp, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,48)?)?;
    // VMA 0xc60e8: mov r2, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,116)?)?;
    // VMA 0xc60ec: str r0, [r10, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,268)?)?;
    // VMA 0xc60f0: ldr r0, [r6, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,996)?)?;
    // VMA 0xc60f4: str r0, [r10, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,272)?)?;
    // VMA 0xc60f8: str r4, [r6, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,468)?)?;
    // VMA 0xc60fc: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // VMA 0xc6100: str r0, [r6, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,472)?)?;
    // VMA 0xc6104: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xc6108: str r0, [r6, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,476)?)?;
    // VMA 0xc610c: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // VMA 0xc6110: str r0, [r6, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,480)?)?;
    // VMA 0xc6114: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // VMA 0xc6118: str r0, [r6, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,992)?)?;
    // VMA 0xc611c: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // VMA 0xc6120: str r0, [r6, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,996)?)?;
    // VMA 0xc6124: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xc6128: bl 0x471c <entry+0xfffffffffff3ea44>
    c.b(C::Al,0x471c,true)?;
    // VMA 0xc612c: b 0xc5b30 <entry+0xfffffffffffffe58>
    c.b(C::Al,0xc5b30,false)?;
    // VMA 0xc6130: ldr r1, [pc, #0x19c]
    c.literal(R::R1,0xc62d4)?;
    // VMA 0xc6134: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc6138: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc613c: ldr r1, [pc, #0x184]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc6140: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc6144: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc6148: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc614c: b 0xc511c <entry+0xfffffffffffff444>
    c.b(C::Al,0xc511c,false)?;
    c.check(0xc6150)?;
    const FALLBACK_WARNING_ONE:u32=0x000f_b7da;
    c.emit(FALLBACK_WARNING_ONE)?;
    c.check(0xc6154)?;
    // VMA 0xc6154: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xc6158: cmphs r3, r2
    c.emit(arm32::data_register(C::Cs,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xc615c: bhs 0xc61fc <entry+0x524>
    c.b(C::Cs,0xc61fc,false)?;
    // VMA 0xc6160: ldr r0, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,140)?)?;
    // VMA 0xc6164: mov r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,120)?)?;
    // VMA 0xc6168: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xc616c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc6170: beq 0xc6180 <entry+0x4a8>
    c.b(C::Eq,0xc6180,false)?;
    // VMA 0xc6174: ldr r0, [r4, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,456)?)?;
    // VMA 0xc6178: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc617c: beq 0xc6250 <entry+0x578>
    c.b(C::Eq,0xc6250,false)?;
    // VMA 0xc6180: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,144)?)?;
    // VMA 0xc6184: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc6188: beq 0xc6198 <entry+0x4c0>
    c.b(C::Eq,0xc6198,false)?;
    // VMA 0xc618c: ldr r0, [r4, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,452)?)?;
    // VMA 0xc6190: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc6194: beq 0xc6250 <entry+0x578>
    c.b(C::Eq,0xc6250,false)?;
    // VMA 0xc6198: ldr r1, [pc, #0x16c]
    c.literal(R::R1,0xc630c)?;
    // VMA 0xc619c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc61a0: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc61a4: ldr r1, [pc, #0x11c]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc61a8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xc61ac: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc61b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc61b4: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // VMA 0xc61b8: b 0xc53bc <entry+0xfffffffffffff6e4>
    c.b(C::Al,0xc53bc,false)?;
    c.check(0xc61bc)?;
    const FALLBACK_WARNING_TWO:u32=0x000f_b9b0;
    c.emit(FALLBACK_WARNING_TWO)?;
    c.check(0xc61c0)?;
    // VMA 0xc61c0: ldr r0, [r4, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2700)?)?;
    // VMA 0xc61c4: mov r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,120)?)?;
    // VMA 0xc61c8: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // VMA 0xc61cc: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xc61d0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc61d4: bne 0xc5b30 <entry+0xfffffffffffffe58>
    c.b(C::Ne,0xc5b30,false)?;
    // VMA 0xc61d8: ldr r1, [pc, #0x11c]
    c.literal(R::R1,0xc62fc)?;
    // VMA 0xc61dc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc61e0: str r0, [r6, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,504)?)?;
    // VMA 0xc61e4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc61e8: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc61ec: ldr r1, [pc, #0xd4]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc61f0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc61f4: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc61f8: b 0xc5b30 <entry+0xfffffffffffffe58>
    c.b(C::Al,0xc5b30,false)?;
    // VMA 0xc61fc: ldr r0, [r4, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2700)?)?;
    // VMA 0xc6200: mov r8, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,120)?)?;
    // VMA 0xc6204: orr r8, r8, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,2048)?)?;
    // VMA 0xc6208: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xc620c: bne 0xc53bc <entry+0xfffffffffffff6e4>
    c.b(C::Ne,0xc53bc,false)?;
    // VMA 0xc6210: ldr r1, [pc, #0xe4]
    c.literal(R::R1,0xc62fc)?;
    // VMA 0xc6214: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc6218: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // VMA 0xc621c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc6220: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc6224: ldr r1, [pc, #0x9c]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc6228: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc622c: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc6230: b 0xc53bc <entry+0xfffffffffffff6e4>
    c.b(C::Al,0xc53bc,false)?;
    // VMA 0xc6234: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xc6238: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // VMA 0xc623c: str r2, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,504)?)?;
    // VMA 0xc6240: str r0, [r4, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,468)?)?;
    // VMA 0xc6244: str r1, [r4, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,472)?)?;
    // VMA 0xc6248: b 0xc5b30 <entry+0xfffffffffffffe58>
    c.b(C::Al,0xc5b30,false)?;
    c.check(0xc624c)?;
    const FALLBACK_WARNING_THREE:u32=0x000f_b7f8;
    c.emit(FALLBACK_WARNING_THREE)?;
    c.check(0xc6250)?;
    // VMA 0xc6250: ldr r1, [pc, #0xb0]
    c.literal(R::R1,0xc6308)?;
    // VMA 0xc6254: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc6258: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc625c: ldr r1, [pc, #0x64]
    c.literal(R::R1,0xc62c8)?;
    // VMA 0xc6260: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xc6264: bl 0x2ec94 <entry+0xfffffffffff68fbc>
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xc6268: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xc626c: str r0, [r4, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,504)?)?;
    // VMA 0xc6270: b 0xc53bc <entry+0xfffffffffffff6e4>
    c.b(C::Al,0xc53bc,false)?;
    // VMA 0xc6274: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xc6278: add r1, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xc627c: ldr r1, [r1, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,640)?)?;
    // VMA 0xc6280: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xc6284: bne 0xc62bc <entry+0x5e4>
    c.b(C::Ne,0xc62bc,false)?;
    // VMA 0xc6288: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xc628c: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // VMA 0xc6290: bls 0xc6278 <entry+0x5a0>
    c.b(C::Ls,0xc6278,false)?;
    // VMA 0xc6294: ldr r0, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2028)?)?;
    // VMA 0xc6298: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xc629c: bne 0xc55d4 <entry+0xfffffffffffff8fc>
    c.b(C::Ne,0xc55d4,false)?;
    // VMA 0xc62a0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xc62a4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xc62a8: bl 0xc6334 <entry+0x65c>
    c.b(C::Al,0xc6334,true)?;
    // VMA 0xc62ac: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xc62b0: str r0, [r10, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,516)?)?;
    // VMA 0xc62b4: str r10, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(10)?,2028)?)?;
    // VMA 0xc62b8: b 0xc55d4 <entry+0xfffffffffffff8fc>
    c.b(C::Al,0xc55d4,false)?;
    // VMA 0xc62bc: str r1, [r10, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,2028)?)?;
    // VMA 0xc62c0: b 0xc55d4 <entry+0xfffffffffffff8fc>
    c.b(C::Al,0xc55d4,false)?;
    c.check(0xc62c4)?;
    const CONTROLLER_ERROR_ADDRESSES:[u32;28]=[
        0x000fb3df,0x000e2788,0x000fb641,0x000fb772,
        0x000fb732,0x000fa1ac,0x000fb7f8,0x000fb97a,
        0x000fb9e9,0x000fbaa7,0x000fbaed,0x000fba15,
        0x000fba5d,0x000fdc88,0x000fb91b,0x000fbb19,
        0x000fb81f,0x000fb88a,0x000fb8dc,0x000fb849,
        0x000fbb62,0x000fbbc5,0x000fb41a,0x000fb464,
        0x000fad42,0x000fb4d4,0x000fb4fa,0x0010408c
    ];
    for addr in CONTROLLER_ERROR_ADDRESSES { c.emit(addr)?; }
    c.check(0xc6334)
}
