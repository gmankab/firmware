
/// Populate register words and structured channel status values.
/// This is part of a larger codec function; the return remains outside this span.
fn program_stream_base_descriptors(c:&mut Code)->Result<(),String>{
    c.check(0xbbe00)?;
    // VMA 0xbbe00: mov r2, #4080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4080)?)?;
    // VMA 0xbbe04: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // VMA 0xbbe08: orr r2, r2, #61440
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,61440)?)?;
    // VMA 0xbbe0c: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // VMA 0xbbe10: and r2, r4, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(4)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbbe14: orr r1, r1, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbe18: orr r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbbe1c: str r2, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbbe20: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // VMA 0xbbe24: orr r2, r8, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,4194304)?)?;
    // VMA 0xbbe28: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // VMA 0xbbe2c: orr r1, r1, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbe30: str r2, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbbe34: mov r1, #4080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4080)?)?;
    // VMA 0xbbe38: mov r2, #66584576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,66584576)?)?;
    // VMA 0xbbe3c: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xbbe40: orr r2, r2, #1006632960
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1006632960)?)?;
    // VMA 0xbbe44: and r1, r4, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbbe48: and r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbbe4c: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbbe50: mov r2, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6144)?)?;
    // VMA 0xbbe54: orr r2, r2, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbe58: str r1, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbbe5c: mov r1, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,12)?)?;
    // VMA 0xbbe60: mov r2, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32768)?)?;
    // VMA 0xbbe64: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,6144)?)?;
    // VMA 0xbbe68: orr r2, r2, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4194304)?)?;
    // VMA 0xbbe6c: orr r1, r1, r5, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(5)?,Shift::Lsl,15,false)?)?;
    // VMA 0xbbe70: orr r2, r2, r8, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(8)?,Shift::Lsl,4,false)?)?;
    // VMA 0xbbe74: str r2, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xbbe78: str r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,28)?)?;
    // VMA 0xbbe7c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // VMA 0xbbe80: ldrb r1, [r6, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,8)?)?;
    // VMA 0xbbe84: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbbe88: bne 0xbbea4 <entry+0xe2c>   @ imm = #0x14
    c.b(C::Ne,0xbbea4,false)?;
    // VMA 0xbbe8c: ldr r3, [r6, #0x250]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,592)?)?;
    // VMA 0xbbe90: mov lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // VMA 0xbbe94: cmp r3, #536870912
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,536870912)?)?;
    // VMA 0xbbe98: ldr r3, [pc, #0x8fc]        @ 0x1724 <entry+0x1724>
    c.literal(R::new(3)?,0xbc79c)?;
    // VMA 0xbbe9c: moveq lr, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::LR,R::R0,1)?)?;
    // VMA 0xbbea0: beq 0xbbeb4 <entry+0xe3c>   @ imm = #0xc
    c.b(C::Eq,0xbbeb4,false)?;
    // VMA 0xbbea4: mov r3, #12582912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,12582912)?)?;
    // VMA 0xbbea8: and r3, r3, r1, lsl #22
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,22,false)?)?;
    // VMA 0xbbeac: orr r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xbbeb0: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xbbeb4: ldr r10, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-36)?)?;
    // VMA 0xbbeb8: mov r9, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,3758096384)?)?;
    // VMA 0xbbebc: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // VMA 0xbbec0: orr r4, r10, #4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(10)?,4)?)?;
    // VMA 0xbbec4: orr r7, r4, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(4)?,6144)?)?;
    // VMA 0xbbec8: mov r4, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,384)?)?;
    // VMA 0xbbecc: and r4, r4, r12, lsl #7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(4)?,R::R12,Shift::Lsl,7,false)?)?;
    // VMA 0xbbed0: orr r3, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbbed4: str r3, [r7, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(7)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbbed8: ldr r3, [r6, #0x408]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,1032)?)?;
    // VMA 0xbbedc: str r12, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R11,-40)?)?;
    // VMA 0xbbee0: ldr r12, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,36)?)?;
    // VMA 0xbbee4: ldr r7, [r12, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R12,4)?)?;
    // VMA 0xbbee8: ldr r7, [r7, #0x908]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(7)?,2312)?)?;
    // VMA 0xbbeec: orr r3, r7, r3, lsl #31
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,31,false)?)?;
    // VMA 0xbbef0: orr r7, r10, #6272
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(10)?,6272)?)?;
    // VMA 0xbbef4: str r3, [r7, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(7)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbbef8: orr r3, r10, #2192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(10)?,2192)?)?;
    // VMA 0xbbefc: orr r7, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(3)?,4096)?)?;
    // VMA 0xbbf00: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xbbf04: str r3, [r7, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(7)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbbf08: orr r7, r10, #132
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(10)?,132)?)?;
    // VMA 0xbbf0c: ldr r0, [r6, #0x408]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1032)?)?;
    // VMA 0xbbf10: orr r7, r7, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,6144)?)?;
    // VMA 0xbbf14: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbbf18: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xbbf1c: str r0, [r7, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbbf20: mov r0, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4194304)?)?;
    // VMA 0xbbf24: orr r7, r10, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(10)?,7168)?)?;
    // VMA 0xbbf28: orr r0, r0, r8, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(8)?,Shift::Lsl,2,false)?)?;
    // VMA 0xbbf2c: str r0, [r7, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbbf30: orr r0, r10, #12
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(10)?,12)?)?;
    // VMA 0xbbf34: ldrb r7, [r6, #0x10]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::new(6)?,16)?)?;
    // VMA 0xbbf38: ldr r5, [r6, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,56)?)?;
    // VMA 0xbbf3c: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xbbf40: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xbbf44: ldr r7, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,52)?)?;
    // VMA 0xbbf48: moveq r2, #16
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,16)?)?;
    // VMA 0xbbf4c: cmp lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,0)?)?;
    // VMA 0xbbf50: mul r4, r2, r7
    c.emit(arm32::multiply_register(C::Al,R::new(4)?,R::R2,R::new(7)?)?)?;
    // VMA 0xbbf54: mov r2, #1016
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1016)?)?;
    // VMA 0xbbf58: orr r2, r2, #64512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,64512)?)?;
    // VMA 0xbbf5c: and r2, r4, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(4)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbbf60: orr r2, r2, r5, lsl #20
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(5)?,Shift::Lsl,20,false)?)?;
    // VMA 0xbbf64: mov r5, r12
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R12))?;
    // VMA 0xbbf68: str r2, [r0, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbbf6c: mov r0, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,192)?)?;
    // VMA 0xbbf70: mov r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // VMA 0xbbf74: andeq r2, r0, r1, lsl #6
    c.emit(arm32::data_register(C::Eq,Op::And,R::R2,R::R0,R::R1,Shift::Lsl,6,false)?)?;
    // VMA 0xbbf78: ldr r1, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // VMA 0xbbf7c: mov r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,48)?)?;
    // VMA 0xbbf80: and r0, r0, r1, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,4,false)?)?;
    // VMA 0xbbf84: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xbbf88: orr r0, r8, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xbbf8c: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbbf90: mov r2, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,268435456)?)?;
    // VMA 0xbbf94: ldr r1, [r1, #0x904]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,2308)?)?;
    // VMA 0xbbf98: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xbbf9c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbbfa0: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbbfa4: orr r0, r0, r1, lsl #30
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,30,false)?)?;
    // VMA 0xbbfa8: ldr r1, [r5, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,156)?)?;
    // VMA 0xbbfac: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xbbfb0: orr r1, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(10)?,8)?)?;
    // VMA 0xbbfb4: moveq r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // VMA 0xbbfb8: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // VMA 0xbbfbc: orr r0, r0, r3, lsl #23
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,23,false)?)?;
    // VMA 0xbbfc0: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // VMA 0xbbfc4: str r0, [r1, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbbfc8: orr r0, r10, #1440
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(10)?,1440)?)?;
    // VMA 0xbbfcc: ldrb r1, [r6, #0x16]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,22)?)?;
    // VMA 0xbbfd0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbbfd4: and r1, r2, r1, lsl #28
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,28,false)?)?;
    // VMA 0xbbfd8: orr r1, r1, #1073741824
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1073741824)?)?;
    // VMA 0xbbfdc: str r1, [r0, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbbfe0: orr r0, r10, #420
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(10)?,420)?)?;
    // VMA 0xbbfe4: mov r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2147483648)?)?;
    // VMA 0xbbfe8: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xbbfec: str r1, [r0, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbbff0: ldr r0, [r5, #0x820]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2080)?)?;
    // VMA 0xbbff4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbbff8: beq 0xbc098 <entry+0x1020>  @ imm = #0x98
    c.b(C::Eq,0xbc098,false)?;
    // VMA 0xbbffc: ldrb r1, [r6, #0xbc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,188)?)?;
    c.check(0xbc000)
}

/// Select DSP tuning source and copy fixed controller register sets.
/// This is part of a larger codec function; the return remains outside this span.
fn initialize_codec_tuning_buffers(c:&mut Code)->Result<(),String>{
    c.check(0xbc000)?;
    // VMA 0xbc000: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbc004: beq 0xbc02c <entry+0xfb4>   @ imm = #0x20
    c.b(C::Eq,0xbc02c,false)?;
    // VMA 0xbc008: add r1, r6, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,60)?)?;
    // VMA 0xbc00c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbc010: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xbc014: ldr r7, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(7)?,R::R1,4)?)?;
    // VMA 0xbc018: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xbc01c: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // VMA 0xbc020: str r7, [r3], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(7)?,R::new(3)?,4)?)?;
    // VMA 0xbc024: bne 0xbc014 <entry+0xf9c>   @ imm = #-0x18
    c.b(C::Ne,0xbc014,false)?;
    // VMA 0xbc028: b 0xbc04c <entry+0xfd4>   @ imm = #0x1c
    c.b(C::Al,0xbc04c,false)?;
    // VMA 0xbc02c: ldr r2, [pc, #0x76c]        @ 0x1728 <entry+0x1728>
    c.literal(R::R2,0xbc7a0)?;
    // VMA 0xbc030: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbc034: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xbc038: ldr r7, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(7)?,R::R2,4)?)?;
    // VMA 0xbc03c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xbc040: cmp r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // VMA 0xbc044: str r7, [r3], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(7)?,R::new(3)?,4)?)?;
    // VMA 0xbc048: bne 0xbc038 <entry+0xfc0>   @ imm = #-0x18
    c.b(C::Ne,0xbc038,false)?;
    // VMA 0xbc04c: ldrb r1, [r6, #0xbd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,189)?)?;
    // VMA 0xbc050: add r0, r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,64)?)?;
    // VMA 0xbc054: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xbc058: beq 0xbc07c <entry+0x1004>  @ imm = #0x1c
    c.b(C::Eq,0xbc07c,false)?;
    // VMA 0xbc05c: add r1, r6, #124
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,124)?)?;
    // VMA 0xbc060: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xbc064: ldr r3, [r1], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::R1,4)?)?;
    // VMA 0xbc068: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xbc06c: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // VMA 0xbc070: str r3, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // VMA 0xbc074: bne 0xbc064 <entry+0xfec>   @ imm = #-0x18
    c.b(C::Ne,0xbc064,false)?;
    // VMA 0xbc078: b 0xbc098 <entry+0x1020>  @ imm = #0x18
    c.b(C::Al,0xbc098,false)?;
    // VMA 0xbc07c: ldr r2, [pc, #0x720]        @ 0x172c <entry+0x172c>
    c.literal(R::R2,0xbc7a4)?;
    // VMA 0xbc080: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xbc084: ldr r3, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::R2,4)?)?;
    // VMA 0xbc088: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // VMA 0xbc08c: cmp r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // VMA 0xbc090: str r3, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R0,4)?)?;
    // VMA 0xbc094: bne 0xbc084 <entry+0x100c>  @ imm = #-0x18
    c.b(C::Ne,0xbc084,false)?;
    // VMA 0xbc098: ldrb r0, [r6, #0xf]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,15)?)?;
    // VMA 0xbc09c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xbc0a0: mov r4, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,512)?)?;
    // VMA 0xbc0a4: rsb r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,3)?)?;
    // VMA 0xbc0a8: lsl r0, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R1,Shift::Lsl,R::R0,false)?)?;
    // VMA 0xbc0ac: ldr r1, [r5, #0x820]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2080)?)?;
    // VMA 0xbc0b0: strb r0, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,0)?)?;
    // VMA 0xbc0b4: mov r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,512)?)?;
    // VMA 0xbc0b8: ldr r0, [r5, #0x820]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2080)?)?;
    // VMA 0xbc0bc: bl 0x205c <entry+0xfffffffffff46fe4> @ imm = #-0xba068
    c.b(C::Al,0x205c,true)?;
    // VMA 0xbc0c0: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // VMA 0xbc0c4: orr r0, r2, #852
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,852)?)?;
    // VMA 0xbc0c8: orr r1, r2, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,2896)?)?;
    // VMA 0xbc0cc: orr r12, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R0,14336)?)?;
    // VMA 0xbc0d0: mov r0, #98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,98)?)?;
    // VMA 0xbc0d4: orr r10, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::R1,12288)?)?;
    // VMA 0xbc0d8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xbc0dc: str r0, [r12, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R12,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc0e0: ldr r1, [r5, #0x820]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2080)?)?;
    // VMA 0xbc0e4: str r1, [r10, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(10)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc0e8: orr r1, r2, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,3024)?)?;
    // VMA 0xbc0ec: orr r5, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R1,12288)?)?;
    // VMA 0xbc0f0: str r0, [r5, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(5)?,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc0f4: orr r0, r2, #972
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,972)?)?;
    // VMA 0xbc0f8: orr lr, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::R0,14336)?)?;
    // VMA 0xbc0fc: orr r0, r2, #888
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,888)?)?;
    // VMA 0xbc100: str r4, [lr, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::LR,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc104: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbc108: ldr r1, [r6, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,544)?)?;
    // VMA 0xbc10c: str r1, [r0, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc110: orr r0, r2, #2928
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,2928)?)?;
    // VMA 0xbc114: ldr r1, [r6, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,544)?)?;
    // VMA 0xbc118: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xbc11c: str r1, [r0, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc120: orr r0, r2, #892
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,892)?)?;
    // VMA 0xbc124: ldr r1, [r6, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,544)?)?;
    // VMA 0xbc128: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbc12c: str r1, [r0, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc130: orr r0, r2, #884
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,884)?)?;
    // VMA 0xbc134: ldr r1, [r6, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,544)?)?;
    // VMA 0xbc138: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xbc13c: str r1, [r0, r9]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(9)?,Shift::Lsl,0)?)?;
    // VMA 0xbc140: ldr r0, [r6, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,732)?)?;
    // VMA 0xbc144: ldr r7, [r6, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,380)?)?;
    // VMA 0xbc148: ldr r4, [r6, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,384)?)?;
    // VMA 0xbc14c: ldr r2, [r6, #0x184]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,388)?)?;
    // VMA 0xbc150: ldr r1, [r6, #0x188]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,392)?)?;
    // VMA 0xbc154: str lr, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,32)?)?;
    // VMA 0xbc158: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xbc15c: bne 0xbc18c <entry+0x1114>  @ imm = #0x28
    c.b(C::Ne,0xbc18c,false)?;
    // VMA 0xbc160: ldr r0, [r6, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,544)?)?;
    // VMA 0xbc164: ldr r3, [r6, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,260)?)?;
    // VMA 0xbc168: add r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xbc16c: add r2, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xbc170: add r4, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbc174: add r7, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbc178: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xbc17c: str r2, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-40)?)?;
    // VMA 0xbc180: str r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,44)?)?;
    // VMA 0xbc184: str r7, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,40)?)?;
    // VMA 0xbc188: b 0xbc1a8 <entry+0x1130>  @ imm = #0x18
    c.b(C::Al,0xbc1a8,false)?;
    // VMA 0xbc18c: ldr r3, [r6, #0x104]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,260)?)?;
    // VMA 0xbc190: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xbc194: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // VMA 0xbc198: str r2, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-40)?)?;
    // VMA 0xbc19c: str r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,44)?)?;
    // VMA 0xbc1a0: str r7, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,40)?)?;
    // VMA 0xbc1a4: beq 0xbc1cc <entry+0x1154>  @ imm = #0x20
    c.b(C::Eq,0xbc1cc,false)?;
    // VMA 0xbc1a8: ldr r8, [r6, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(6)?,544)?)?;
    // VMA 0xbc1ac: ldr r0, [r6, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,264)?)?;
    // VMA 0xbc1b0: ldr r1, [r6, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,284)?)?;
    // VMA 0xbc1b4: add r1, r1, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbc1b8: add r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbc1bc: add r3, r8, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(8)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xbc1c0: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // VMA 0xbc1c4: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xbc1c8: b 0xbc1e0 <entry+0x1168>  @ imm = #0x10
    c.b(C::Al,0xbc1e0,false)?;
    // VMA 0xbc1cc: ldr r0, [r6, #0x108]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,264)?)?;
    // VMA 0xbc1d0: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xbc1d4: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // VMA 0xbc1d8: ldr r0, [r6, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,284)?)?;
    // VMA 0xbc1dc: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // VMA 0xbc1e0: ldr r0, [r6, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,288)?)?;
    // VMA 0xbc1e4: mov r9, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,3758096384)?)?;
    // VMA 0xbc1e8: mov r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,6)?)?;
    // VMA 0xbc1ec: mov lr, r12
    c.emit(arm32::mov_register(C::Al,R::LR,R::R12))?;
    // VMA 0xbc1f0: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xbc1f4: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xbc1f8: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // VMA 0xbc1fc: mov r0, r12
    c.emit(arm32::mov_register(C::Al,R::R0,R::R12))?;
    c.check(0xbc200)
}
