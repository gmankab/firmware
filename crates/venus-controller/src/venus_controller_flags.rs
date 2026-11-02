// SPDX-License-Identifier: GPL-3.0-only
//! Reconstructed A32 codec-controller capability and command flag writer.
//! Function `[0x424c0,0x42728)` emits controller bit fields and optionally
//! writes additional mode-dependent command packets, followed by a named
//! one-word non-executable literal pool `[0x42728,0x4272c)`.
//!
//! This is independent, disjoint from Agent 20-04's earlier `[0x40a2c,
//! 0x40c14)` runtime fragment. It does not implicitly fill `[0x40c14,
//! 0x424c0)` with unknown vendor instructions.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};

pub const START:usize=0x424c0;
pub const INSTRUCTIONS_END:usize=0x42728;
pub const END:usize=0x4272c;
pub const FILE_BASE:usize=0x4000;
pub const CAPABILITY_MASK:u32=0xfff8_01ff; // non-instruction data word

struct Code { at:usize, bytes:Vec<u8> }
impl Code {
    fn new()->Self {Self {at:START,bytes:Vec::new()}}
    fn check(&self,expected:usize)->Result<(),String>{
        if self.at!=expected {return Err(format!("controller flags address 0x{:x} expected 0x{expected:x}",self.at));}
        Ok(())
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if !(0x40000..0x68000).contains(&self.at) || self.at&3!=0 {return Err(format!("agent04 invalid runtime address 0x{:x}",self.at));}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,c:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(c,self.at as u32,target as u32,false)?)
    }
    fn literal(&mut self,rd:R,target:usize)->Result<(),String>{
        let diff=i32::try_from(target as isize-self.at as isize-8)
            .map_err(|_|"Venus controller literal out of range")?;
        self.emit(arm32::transfer_word(C::Al,true,rd,R::PC,diff)?)
    }
}

/// Derive capability bits from source descriptors, quantization/memory
/// fields, command mode, optional flag masks and the selected controller
/// subtype; output controller command records through the writable pointer.
fn classify_and_write_flags(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x424c0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x424c4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x424c8: ldrb r2, [r0, #0x99e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2462)?)?;
    // 0x424cc: ldr r12, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,44)?)?;
    // 0x424d0: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x424d4: cmp r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // 0x424d8: ldr r2, [r0, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,644)?)?;
    // 0x424dc: moveq r3, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x424e0: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x424e4: beq 0x424fc <entry+0x25fc>
    c.branch(C::Eq,0x424fc)?;
    // 0x424e8: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x424ec: bne 0x42518 <entry+0x2618>
    c.branch(C::Ne,0x42518)?;
    // 0x424f0: ldr r5, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,2744)?)?;
    // 0x424f4: ldr r5, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,44)?)?;
    // 0x424f8: b 0x4250c <entry+0x260c>
    c.branch(C::Al,0x4250c)?;
    // 0x424fc: ldrb r4, [r0, #0x544]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R0,1348)?)?;
    // 0x42500: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x42504: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x42508: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x4250c: mov r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,16)?)?;
    // 0x42510: and r5, r4, r5, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,4,false)?)?;
    // 0x42514: orr r3, r5, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x42518: ldrb r5, [r12, #0x3e9]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R12,1001)?)?;
    // 0x4251c: ldr lr, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,2744)?)?;
    // 0x42520: mov r4, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,128)?)?;
    // 0x42524: mov r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,32)?)?;
    // 0x42528: bic r3, r3, #224
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,224)?)?;
    // 0x4252c: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x42530: and r4, r4, r5, lsl #7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(4)?,R::new(5)?,Shift::Lsl,7,false)?)?;
    // 0x42534: ldr r5, [lr, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::LR,16)?)?;
    // 0x42538: and r5, r6, r5, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(6)?,R::new(5)?,Shift::Lsl,5,false)?)?;
    // 0x4253c: orr r3, r5, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x42540: orr r3, r3, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x42544: beq 0x4256c <entry+0x266c>
    c.branch(C::Eq,0x4256c)?;
    // 0x42548: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x4254c: beq 0x42580 <entry+0x2680>
    c.branch(C::Eq,0x42580)?;
    // 0x42550: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x42554: bne 0x42594 <entry+0x2694>
    c.branch(C::Ne,0x42594)?;
    // 0x42558: ldr r6, [lr, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::LR,56)?)?;
    // 0x4255c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x42560: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x42564: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x42568: b 0x42584 <entry+0x2684>
    c.branch(C::Al,0x42584)?;
    // 0x4256c: ldrb r6, [r0, #0x544]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R0,1348)?)?;
    // 0x42570: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x42574: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x42578: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x4257c: b 0x42584 <entry+0x2684>
    c.branch(C::Al,0x42584)?;
    // 0x42580: ldrb r2, [r0, #0x339]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,825)?)?;
    // 0x42584: mov r6, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,256)?)?;
    // 0x42588: bic r3, r3, #320
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,320)?)?;
    // 0x4258c: and r2, r6, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,8,false)?)?;
    // 0x42590: orr r3, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x42594: ldr r2, [pc, #0x18c]
    c.literal(R::R2,0x42728)?;
    // 0x42598: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x4259c: and r3, r3, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x425a0: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // 0x425a4: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x425a8: ldreq r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R2,0)?)?;
    // 0x425ac: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x425b0: streq r6, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R2,0)?)?;
    // 0x425b4: mov r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,8192)?)?;
    // 0x425b8: str r6, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R2,0)?)?;
    // 0x425bc: str r3, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,4)?)?;
    // 0x425c0: add r3, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,8)?)?;
    // 0x425c4: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // 0x425c8: ldr r4, [r0, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,644)?)?;
    // 0x425cc: cmp r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,4096)?)?;
    // 0x425d0: bne 0x42648 <entry+0x2748>
    c.branch(C::Ne,0x42648)?;
    // 0x425d4: ldrb r6, [r12, #0x33e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R12,830)?)?;
    // 0x425d8: mov r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16)?)?;
    // 0x425dc: ldrb r4, [r12, #0x340]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R12,832)?)?;
    // 0x425e0: ldrb lr, [r12, #0x33f]
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::R12,831)?)?;
    // 0x425e4: and r5, r5, r6, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(6)?,Shift::Lsl,4,false)?)?;
    // 0x425e8: ldrb r6, [r0, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R0,988)?)?;
    // 0x425ec: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x425f0: movne r6, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x425f4: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x425f8: lsl r7, r6, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::new(6)?,Shift::Lsl,16)?)?;
    // 0x425fc: orr r6, r7, r6, lsl #19
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(7)?,R::new(6)?,Shift::Lsl,19,false)?)?;
    // 0x42600: and r7, lr, #63
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::LR,63)?)?;
    // 0x42604: mov lr, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,2)?)?;
    // 0x42608: orr r5, r6, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4260c: mov r6, #224
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,224)?)?;
    // 0x42610: and r4, r6, r4, lsl #5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,5,false)?)?;
    // 0x42614: orr r4, r5, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x42618: orr r7, r4, r7, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(4)?,R::new(7)?,Shift::Lsl,8,false)?)?;
    // 0x4261c: orr r7, r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,7)?)?;
    // 0x42620: orr r5, r7, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(7)?,4194304)?)?;
    // 0x42624: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x42628: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4262c: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x42630: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // 0x42634: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x42638: str r3, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,8)?)?;
    // 0x4263c: add r3, r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,16)?)?;
    // 0x42640: str r5, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,12)?)?;
    // 0x42644: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // 0x42648: ldr r2, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2744)?)?;
    // 0x4264c: ldrb r4, [r2, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R2,339)?)?;
    // 0x42650: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x42654: bne 0x42720 <entry+0x2820>
    c.branch(C::Ne,0x42720)?;
    // 0x42658: ldrb r4, [r0, #0x494]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R0,1172)?)?;
    // 0x4265c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x42660: beq 0x42720 <entry+0x2820>
    c.branch(C::Eq,0x42720)?;
    // 0x42664: ldr r6, [r12, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R12,400)?)?;
    // 0x42668: ldr r4, [r0, #0x548]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,1352)?)?;
    // 0x4266c: mov r5, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,120)?)?;
    // 0x42670: add lr, lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::LR,1)?)?;
    // 0x42674: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x42678: mov r7, r6
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // 0x4267c: and r4, r5, r4, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(5)?,R::new(4)?,Shift::Lsl,2,false)?)?;
    // 0x42680: mov r5, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,131072)?)?;
    // 0x42684: movne r7, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x42688: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x4268c: ldr r6, [r0, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,644)?)?;
    // 0x42690: moveq r7, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x42694: orr r12, r7, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R12,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x42698: ldr r4, [r0, #0x54c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,1356)?)?;
    // 0x4269c: mov r7, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,262144)?)?;
    // 0x426a0: cmp r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,8192)?)?;
    // 0x426a4: ldreq r5, [r2, #0x30]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::R2,48)?)?;
    // 0x426a8: moveq r6, #131072
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,131072)?)?;
    // 0x426ac: lsl r4, r4, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(4)?,Shift::Lsl,6)?)?;
    // 0x426b0: and r4, r4, #1920
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(4)?,1920)?)?;
    // 0x426b4: andeq r5, r6, r5, lsl #17
    c.emit(arm32::data_register(C::Eq,Op::And,R::new(5)?,R::new(6)?,R::new(5)?,Shift::Lsl,17,false)?)?;
    // 0x426b8: ldr r6, [r2, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,172)?)?;
    // 0x426bc: ldr r2, [r2, #0x244]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,580)?)?;
    // 0x426c0: and r6, r7, r6, lsl #18
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(7)?,R::new(6)?,Shift::Lsl,18,false)?)?;
    // 0x426c4: orr r7, r12, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R12,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x426c8: orr r7, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x426cc: mov r5, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1048576)?)?;
    // 0x426d0: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x426d4: ldrb r6, [r0, #0x3cc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::R0,972)?)?;
    // 0x426d8: ldrb r0, [r0, #0xb7a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,2938)?)?;
    // 0x426dc: and r6, r5, r6, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::new(6)?,Shift::Lsl,20,false)?)?;
    // 0x426e0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x426e4: orr r7, r7, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x426e8: mov r6, #8388608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,8388608)?)?;
    // 0x426ec: and r2, r6, r2, lsl #23
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(6)?,R::R2,Shift::Lsl,23,false)?)?;
    // 0x426f0: orr r2, r7, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x426f4: bic r7, r2, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::R2,4278190080)?)?;
    // 0x426f8: orrne r2, r7, #-268435456
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R2,R::new(7)?,4026531840)?)?;
    // 0x426fc: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x42700: ldreq r0, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(3)?,0)?)?;
    // 0x42704: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x42708: streq r0, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(3)?,0)?)?;
    // 0x4270c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x42710: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x42714: stm r3, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit() | R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x42718: add r0, r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,8)?)?;
    // 0x4271c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x42720: mov r0, lr
    c.emit(arm32::mov_register(C::Al,R::R0,R::LR))?;
    // 0x42724: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(INSTRUCTIONS_END)?;
    c.emit(CAPABILITY_MASK)?;
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new();classify_and_write_flags(&mut c)?;Ok(c.bytes)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]
    fn command_flags_are_contiguous_and_literal_separate(){
        let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
        assert_eq!(INSTRUCTIONS_END-START,154*4);
        assert_eq!(u32::from_le_bytes(bytes[bytes.len()-4..].try_into().unwrap()),CAPABILITY_MASK);
        assert!(END<=0x68000 && START>=0x40000);
    }
}
