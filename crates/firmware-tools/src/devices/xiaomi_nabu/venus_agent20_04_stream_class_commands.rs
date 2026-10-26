// SPDX-License-Identifier: GPL-3.0-only
//! Controller stream/class command-buffer writer for Venus runtime
//! [0x4a000, 0x4c000). Reconstruct per-class packet layout and pixel
//! stride control from typed A32 instructions, with one explicit branch-
//! skipped, PC-referenced callback literal at 0x4ba08. The surrounding
//! long codec routine continues beyond this segment; no epilogue fabricated.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};

pub const START:usize=0x4a000;
pub const END:usize=0x4c000;
const DIAGNOSTIC_CALLBACK:u32=0x000e_e028;

struct Code { at:usize, bytes:Vec<u8> }
impl Code {
    fn new()->Self { Self{ at:START, bytes:Vec::with_capacity(END-START) } }
    fn check(&self,a:usize)->Result<(),String> {
        if self.at!=a {return Err(format!("codec class A32 at {:#x}; expected {a:#x}",self.at));}
        Ok(())
    }
    fn emit(&mut self,w:u32)->Result<(),String> {
        if !(0x40000..0x68000).contains(&self.at) || self.at%4!=0 {
            return Err("emission outside Agent04 address range".into());
        }
        self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,c:C,target:usize)->Result<(),String> {
        self.emit(arm32::branch(c,self.at as u32,target as u32,false)?)
    }
    fn literal(&mut self,c:C,rd:R,target:usize)->Result<(),String> {
        let offset=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"ARM literal displacement overflow")?;
        self.emit(arm32::transfer_word(c,true,rd,R::PC,offset)?)
    }
}

/// Serialize controller buffers by stream/class, with masked geometry and
/// optional callback handling. The class-control message `0x000ee028` is
/// PC-loaded from a branch-skipped word; it is never executed as an opcode.
fn emit_stream_class_commands(c:&mut Code)->Result<(),String> {
    c.check(START)?;
    // 0x4a000: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a004: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a008: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a00c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a010: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a014: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a018: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a01c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a020: ldr r2, [pc, #-0x1c4]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a024: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a028: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a02c: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x4a030: mov r1, #69
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,69)?)?;
    // 0x4a034: and r4, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4a038: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a03c: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a040: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4a044: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a048: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x488a0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a04c: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a050: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x4a054: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a058: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a05c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4a060: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4a064: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4a068: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x4a06c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a070: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4a074: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a078: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a07c: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a080: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a084: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a088: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a08c: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a090: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a094: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a098: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a09c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a0a0: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a0a4: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a0a8: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a0ac: ldr r2, [pc, #-0x250]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a0b0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a0b4: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4a0b8: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a0bc: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x4a0c0: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4a0c4: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4a0c8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a0cc: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4a0d0: mov r1, #69
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,69)?)?;
    // 0x4a0d4: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a0d8: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a0dc: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48934
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a0e0: ldr r0, [sp, #0x228]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,552)?)?;
    // 0x4a0e4: add r9, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(4)?,4)?)?;
    // 0x4a0e8: and r1, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,768)?)?;
    // 0x4a0ec: cmp r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,512)?)?;
    // 0x4a0f0: beq 0x4a234 <entry+0x1234>  @ imm = #0x13c
    c.branch(C::Eq,0x4a234)?;
    // 0x4a0f4: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a0f8: mov r5, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,900)?)?;
    // 0x4a0fc: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x4a100: orr r5, r5, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,14336)?)?;
    // 0x4a104: orr r4, r4, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,65280)?)?;
    // 0x4a108: and r1, r9, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4a10c: orr r1, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x4a110: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a114: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a118: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4a11c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4a120: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4a124: mov r2, #34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,34)?)?;
    // 0x4a128: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x4a12c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4a130: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a134: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a138: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a13c: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a140: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a144: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a148: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a14c: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a150: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a154: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a158: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a15c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a160: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a164: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a168: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a16c: ldr r2, [pc, #-0x310]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a170: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a174: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a178: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x4a17c: mov r1, #70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,70)?)?;
    // 0x4a180: and r4, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4a184: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a188: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a18c: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4a190: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a194: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x489ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a198: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a19c: mov r2, #34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,34)?)?;
    // 0x4a1a0: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a1a4: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a1a8: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4a1ac: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4a1b0: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4a1b4: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x4a1b8: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a1bc: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4a1c0: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a1c4: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a1c8: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a1cc: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a1d0: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a1d4: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a1d8: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a1dc: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a1e0: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a1e4: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a1e8: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a1ec: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a1f0: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a1f4: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a1f8: ldr r2, [pc, #-0x39c]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a1fc: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a200: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4a204: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a208: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x4a20c: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4a210: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4a214: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a218: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4a21c: mov r1, #70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,70)?)?;
    // 0x4a220: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a224: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a228: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48a80
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a22c: ldr r0, [sp, #0x228]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,552)?)?;
    // 0x4a230: add r9, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(4)?,4)?)?;
    // 0x4a234: and r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,12288)?)?;
    // 0x4a238: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x4a23c: beq 0x4a37c <entry+0x137c>  @ imm = #0x138
    c.branch(C::Eq,0x4a37c)?;
    // 0x4a240: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a244: mov r5, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,900)?)?;
    // 0x4a248: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x4a24c: orr r5, r5, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,14336)?)?;
    // 0x4a250: orr r4, r4, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,65280)?)?;
    // 0x4a254: and r1, r9, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4a258: orr r1, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x4a25c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a260: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a264: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4a268: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4a26c: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4a270: mov r2, #50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,50)?)?;
    // 0x4a274: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x4a278: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4a27c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a280: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a284: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a288: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a28c: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a290: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a294: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a298: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a29c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a2a0: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a2a4: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a2a8: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a2ac: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a2b0: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a2b4: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a2b8: ldr r2, [pc, #-0x45c]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a2bc: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a2c0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a2c4: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x4a2c8: mov r1, #71
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,71)?)?;
    // 0x4a2cc: and r4, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4a2d0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a2d4: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a2d8: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4a2dc: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a2e0: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48b38
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a2e4: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a2e8: mov r2, #50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,50)?)?;
    // 0x4a2ec: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a2f0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a2f4: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4a2f8: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4a2fc: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4a300: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x4a304: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a308: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4a30c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a310: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a314: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a318: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a31c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a320: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a324: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a328: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a32c: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a330: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a334: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a338: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a33c: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a340: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a344: ldr r2, [pc, #-0x4e8]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a348: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a34c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4a350: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a354: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x4a358: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4a35c: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4a360: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a364: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4a368: mov r1, #71
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,71)?)?;
    // 0x4a36c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a370: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a374: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48bcc
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a378: add r9, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(4)?,4)?)?;
    // 0x4a37c: ldr r10, [pc, #-0x520]      @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::new(10)?,0x49e64)?;
    // 0x4a380: ldr r4, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,104)?)?;
    // 0x4a384: mov r8, #328
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,328)?)?;
    // 0x4a388: mov r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,255)?)?;
    // 0x4a38c: mov r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4)?)?;
    // 0x4a390: mov r7, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,64)?)?;
    // 0x4a394: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x4a398: str r9, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,100)?)?;
    // 0x4a39c: orr r8, r8, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,10240)?)?;
    // 0x4a3a0: orr r6, r6, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,65280)?)?;
    // 0x4a3a4: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x4a3a8: mov r12, r4
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(4)?))?;
    // 0x4a3ac: and r9, r0, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4a3b0: mov lr, r6
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(6)?))?;
    // 0x4a3b4: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x4a3b8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x4a3bc: and r0, r0, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // 0x4a3c0: orr r3, r7, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4a3c4: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x4a3c8: mov r4, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,900)?)?;
    // 0x4a3cc: orr r4, r4, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,14336)?)?;
    // 0x4a3d0: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4a3d4: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x4a3d8: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4a3dc: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x4a3e0: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // 0x4a3e4: str r3, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x4a3e8: orr r2, r9, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(9)?,262144)?)?;
    // 0x4a3ec: ldr r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // 0x4a3f0: add r6, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(3)?,1)?)?;
    // 0x4a3f4: str r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-36)?)?;
    // 0x4a3f8: add r6, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,8)?)?;
    // 0x4a3fc: str r6, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R12,0)?)?;
    // 0x4a400: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x4a404: ldreq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(6)?,0)?)?;
    // 0x4a408: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x4a40c: streq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(6)?,0)?)?;
    // 0x4a410: mov r4, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,15232)?)?;
    // 0x4a414: cmp r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,768)?)?;
    // 0x4a418: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a41c: mov r6, lr
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::LR))?;
    // 0x4a420: str r4, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,8)?)?;
    // 0x4a424: str r2, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,12)?)?;
    // 0x4a428: add r2, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,2)?)?;
    // 0x4a42c: add r1, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,16)?)?;
    // 0x4a430: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a434: mov r4, r12
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R12))?;
    // 0x4a438: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4a43c: str r1, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x4a440: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x4a444: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x4a448: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48ca0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a44c: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4a450: add r7, r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,16)?)?;
    // 0x4a454: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x4a458: cmp r5, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,13)?)?;
    // 0x4a45c: bne 0x4a3a4 <entry+0x13a4>  @ imm = #-0xc0
    c.branch(C::Ne,0x4a3a4)?;
    // 0x4a460: ldr r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,100)?)?;
    // 0x4a464: ldr r10, [pc, #-0x608]      @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::new(10)?,0x49e64)?;
    // 0x4a468: mov r8, #73
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,73)?)?;
    // 0x4a46c: mov r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4)?)?;
    // 0x4a470: mov r7, #66
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,66)?)?;
    // 0x4a474: orr r8, r8, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,10496)?)?;
    // 0x4a478: add r0, r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,36)?)?;
    // 0x4a47c: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x4a480: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4a484: and r9, r0, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4a488: mov lr, r6
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(6)?))?;
    // 0x4a48c: mov r12, r4
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(4)?))?;
    // 0x4a490: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x4a494: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4a498: and r0, r0, lr
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // 0x4a49c: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x4a4a0: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x4a4a4: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4a4a8: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x4a4ac: stm r1, {r3, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::new(3)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x4a4b0: orr r2, r9, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(9)?,262144)?)?;
    // 0x4a4b4: ldr r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // 0x4a4b8: add r6, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(3)?,1)?)?;
    // 0x4a4bc: str r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-36)?)?;
    // 0x4a4c0: add r6, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R1,8)?)?;
    // 0x4a4c4: str r6, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R12,0)?)?;
    // 0x4a4c8: tst r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(6)?,31)?)?;
    // 0x4a4cc: ldreq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(6)?,0)?)?;
    // 0x4a4d0: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x4a4d4: streq r4, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(6)?,0)?)?;
    // 0x4a4d8: mov r4, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,15232)?)?;
    // 0x4a4dc: cmp r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,768)?)?;
    // 0x4a4e0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a4e4: mov r6, lr
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::LR))?;
    // 0x4a4e8: str r4, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,8)?)?;
    // 0x4a4ec: str r2, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,12)?)?;
    // 0x4a4f0: add r2, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(3)?,2)?)?;
    // 0x4a4f4: add r1, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,16)?)?;
    // 0x4a4f8: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a4fc: mov r4, r12
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R12))?;
    // 0x4a500: str r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-36)?)?;
    // 0x4a504: str r1, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,0)?)?;
    // 0x4a508: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x4a50c: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x4a510: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48d68
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a514: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4a518: add r7, r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,16)?)?;
    // 0x4a51c: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x4a520: cmp r5, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,12)?)?;
    // 0x4a524: bne 0x4a47c <entry+0x147c>  @ imm = #-0xb0
    c.branch(C::Ne,0x4a47c)?;
    // 0x4a528: ldr r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,100)?)?;
    // 0x4a52c: add r9, r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R0,68)?)?;
    // 0x4a530: ldr r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,88)?)?;
    // 0x4a534: ldr r0, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2744)?)?;
    // 0x4a538: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x4a53c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4a540: bne 0x4ae54 <entry+0x1e54>  @ imm = #0x90c
    c.branch(C::Ne,0x4ae54)?;
    // 0x4a544: ldr r0, [sp, #0x22c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,556)?)?;
    // 0x4a548: and r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,3)?)?;
    // 0x4a54c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x4a550: beq 0x4a6a0 <entry+0x16a0>  @ imm = #0x148
    c.branch(C::Eq,0x4a6a0)?;
    // 0x4a554: ldr r8, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,100)?)?;
    // 0x4a558: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a55c: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4a560: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4a564: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4a568: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4a56c: add r0, r8, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,68)?)?;
    // 0x4a570: and r1, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4a574: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a578: orr r1, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x4a57c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a580: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4a584: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4a588: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4a58c: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x4a590: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4a594: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4a598: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a59c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a5a0: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a5a4: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a5a8: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a5ac: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a5b0: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a5b4: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a5b8: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a5bc: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a5c0: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a5c4: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a5c8: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a5cc: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a5d0: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a5d4: ldr r2, [pc, #-0x778]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a5d8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a5dc: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a5e0: add r0, r8, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,72)?)?;
    // 0x4a5e4: mov r1, #77
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,77)?)?;
    // 0x4a5e8: and r4, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4a5ec: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a5f0: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a5f4: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4a5f8: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a5fc: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48e54
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a600: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a604: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4a608: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x4a60c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4a610: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a614: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a618: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4a61c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4a620: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4a624: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4a628: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a62c: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4a630: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a634: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a638: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a63c: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a640: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a644: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a648: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a64c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a650: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a654: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a658: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a65c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a660: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a664: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a668: ldr r2, [pc, #-0x80c]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a66c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a670: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4a674: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a678: add r0, r8, #76
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,76)?)?;
    // 0x4a67c: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4a680: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4a684: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a688: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4a68c: mov r1, #77
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,77)?)?;
    // 0x4a690: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a694: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a698: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48ef0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a69c: add r9, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(4)?,4)?)?;
    // 0x4a6a0: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a6a4: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4a6a8: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4a6ac: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4a6b0: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4a6b4: and r1, r9, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4a6b8: orr r1, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x4a6bc: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a6c0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a6c4: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4a6c8: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4a6cc: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4a6d0: mov r2, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,19)?)?;
    // 0x4a6d4: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4a6d8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4a6dc: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a6e0: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a6e4: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a6e8: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a6ec: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a6f0: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a6f4: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a6f8: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a6fc: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a700: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a704: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a708: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a70c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a710: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a714: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a718: ldr r2, [pc, #-0x8bc]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a71c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a720: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a724: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x4a728: mov r1, #78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,78)?)?;
    // 0x4a72c: and r8, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(8)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4a730: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a734: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a738: cmp r8, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,768)?)?;
    // 0x4a73c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a740: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x48f98
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a744: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a748: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4a74c: mov r2, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,19)?)?;
    // 0x4a750: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x4a754: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4a758: orr r4, r4, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,65280)?)?;
    // 0x4a75c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a760: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a764: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4a768: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4a76c: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4a770: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4a774: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a778: orr r1, r8, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(8)?,262144)?)?;
    // 0x4a77c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a780: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a784: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a788: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a78c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a790: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a794: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a798: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a79c: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a7a0: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a7a4: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a7a8: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a7ac: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a7b0: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a7b4: ldr r2, [pc, #-0x958]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a7b8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a7bc: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a7c0: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x4a7c4: mov r1, #78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,78)?)?;
    // 0x4a7c8: and r5, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4a7cc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a7d0: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a7d4: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4a7d8: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a7dc: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49034
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a7e0: ldr r0, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,560)?)?;
    // 0x4a7e4: and r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,3)?)?;
    // 0x4a7e8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x4a7ec: beq 0x4a928 <entry+0x1928>  @ imm = #0x134
    c.branch(C::Eq,0x4a928)?;
    // 0x4a7f0: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a7f4: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x4a7f8: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4a7fc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4a800: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4a804: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a808: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4a80c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4a810: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4a814: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4a818: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a81c: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4a820: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4a824: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a828: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4a82c: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a830: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a834: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a838: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a83c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a840: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a844: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a848: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a84c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a850: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a854: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a858: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a85c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a860: ldr r2, [pc, #-0xa04]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a864: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a868: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a86c: add r0, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,12)?)?;
    // 0x4a870: mov r1, #2384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2384)?)?;
    // 0x4a874: and r8, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(8)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4a878: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a87c: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x4a880: cmp r8, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,768)?)?;
    // 0x4a884: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a888: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x490e0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a88c: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a890: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4a894: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x4a898: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x4a89c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4a8a0: orr r4, r4, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,65280)?)?;
    // 0x4a8a4: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a8a8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a8ac: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4a8b0: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4a8b4: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4a8b8: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4a8bc: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a8c0: orr r1, r8, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(8)?,262144)?)?;
    // 0x4a8c4: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a8c8: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a8cc: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a8d0: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a8d4: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a8d8: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a8dc: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a8e0: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a8e4: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a8e8: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a8ec: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a8f0: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a8f4: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a8f8: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a8fc: ldr r2, [pc, #-0xaa0]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a900: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a904: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a908: add r0, r9, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,16)?)?;
    // 0x4a90c: and r1, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4a910: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a914: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4a918: mov r1, #2384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2384)?)?;
    // 0x4a91c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a920: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x4a924: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4917c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a928: add r0, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,4)?)?;
    // 0x4a92c: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4a930: and r1, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4a934: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a938: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4a93c: orr r1, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x4a940: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a944: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4a948: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4a94c: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4a950: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x4a954: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4a958: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4a95c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a960: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a964: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4a968: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4a96c: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4a970: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4a974: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4a978: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a97c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4a980: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4a984: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4a988: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4a98c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4a990: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4a994: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4a998: ldr r2, [pc, #-0xb3c]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4a99c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4a9a0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4a9a4: add r0, r8, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,8)?)?;
    // 0x4a9a8: mov r1, #81
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,81)?)?;
    // 0x4a9ac: and r4, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4a9b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4a9b4: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4a9b8: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4a9bc: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4a9c0: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49218
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4a9c4: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4a9c8: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4a9cc: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x4a9d0: add r9, r8, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(8)?,12)?)?;
    // 0x4a9d4: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4a9d8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4a9dc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4a9e0: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4a9e4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4a9e8: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4a9ec: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4a9f0: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4a9f4: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4a9f8: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4a9fc: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4aa00: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4aa04: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4aa08: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4aa0c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4aa10: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4aa14: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4aa18: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4aa1c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4aa20: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4aa24: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4aa28: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4aa2c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4aa30: ldr r2, [pc, #-0xbd4]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4aa34: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4aa38: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4aa3c: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4aa40: mov r1, #81
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,81)?)?;
    // 0x4aa44: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x4aa48: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4aa4c: and r5, r9, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4aa50: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4aa54: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4aa58: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4aa5c: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x492b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4aa60: ldr r0, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,560)?)?;
    // 0x4aa64: and r1, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,768)?)?;
    // 0x4aa68: cmp r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,512)?)?;
    // 0x4aa6c: beq 0x4abb0 <entry+0x1bb0>  @ imm = #0x13c
    c.branch(C::Eq,0x4abb0)?;
    // 0x4aa70: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4aa74: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x4aa78: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4aa7c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4aa80: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4aa84: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4aa88: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4aa8c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4aa90: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4aa94: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4aa98: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4aa9c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4aaa0: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4aaa4: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4aaa8: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4aaac: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4aab0: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4aab4: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4aab8: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4aabc: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4aac0: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4aac4: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4aac8: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4aacc: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4aad0: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4aad4: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4aad8: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4aadc: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4aae0: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4aae4: ldr r2, [pc, #-0xc88]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4aae8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4aaec: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4aaf0: add r0, r8, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,16)?)?;
    // 0x4aaf4: mov r1, #82
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,82)?)?;
    // 0x4aaf8: and r4, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4aafc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4ab00: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4ab04: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4ab08: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4ab0c: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49364
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4ab10: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4ab14: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4ab18: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x4ab1c: add r9, r8, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(8)?,20)?)?;
    // 0x4ab20: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4ab24: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4ab28: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4ab2c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4ab30: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4ab34: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4ab38: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4ab3c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4ab40: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4ab44: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4ab48: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4ab4c: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4ab50: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4ab54: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4ab58: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4ab5c: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4ab60: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4ab64: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4ab68: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4ab6c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4ab70: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4ab74: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4ab78: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4ab7c: ldr r2, [pc, #-0xd20]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4ab80: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4ab84: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4ab88: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x4ab8c: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x4ab90: and r1, r9, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4ab94: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4ab98: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4ab9c: mov r1, #82
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,82)?)?;
    // 0x4aba0: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4aba4: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4aba8: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49400
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4abac: ldr r0, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,560)?)?;
    // 0x4abb0: and r1, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,12288)?)?;
    // 0x4abb4: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x4abb8: beq 0x4ad04 <entry+0x1d04>  @ imm = #0x144
    c.branch(C::Eq,0x4ad04)?;
    // 0x4abbc: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4abc0: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4abc4: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4abc8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4abcc: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4abd0: and r1, r9, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4abd4: orr r1, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x4abd8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4abdc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4abe0: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4abe4: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4abe8: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4abec: mov r2, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,52)?)?;
    // 0x4abf0: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4abf4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4abf8: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4abfc: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4ac00: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4ac04: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4ac08: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4ac0c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4ac10: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4ac14: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4ac18: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4ac1c: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4ac20: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4ac24: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4ac28: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4ac2c: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4ac30: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4ac34: ldr r2, [pc, #-0xdd8]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4ac38: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4ac3c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4ac40: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x4ac44: mov r1, #83
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,83)?)?;
    // 0x4ac48: and r5, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4ac4c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4ac50: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4ac54: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4ac58: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4ac5c: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x494b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4ac60: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4ac64: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4ac68: mov r2, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,52)?)?;
    // 0x4ac6c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4ac70: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4ac74: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4ac78: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4ac7c: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4ac80: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4ac84: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4ac88: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4ac8c: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4ac90: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4ac94: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4ac98: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4ac9c: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4aca0: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4aca4: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4aca8: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4acac: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4acb0: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4acb4: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4acb8: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4acbc: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4acc0: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4acc4: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4acc8: ldr r2, [pc, #-0xe6c]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4accc: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4acd0: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4acd4: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4acd8: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x4acdc: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4ace0: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4ace4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4ace8: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4acec: mov r1, #83
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,83)?)?;
    // 0x4acf0: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4acf4: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4acf8: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49550
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4acfc: ldr r0, [sp, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,560)?)?;
    // 0x4ad00: add r9, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(5)?,4)?)?;
    // 0x4ad04: and r0, r0, #196608
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,196608)?)?;
    // 0x4ad08: cmp r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,131072)?)?;
    // 0x4ad0c: beq 0x4ae54 <entry+0x1e54>  @ imm = #0x140
    c.branch(C::Eq,0x4ae54)?;
    // 0x4ad10: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4ad14: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4ad18: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4ad1c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4ad20: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4ad24: and r1, r9, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4ad28: orr r1, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x4ad2c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4ad30: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4ad34: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4ad38: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4ad3c: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4ad40: mov r2, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,68)?)?;
    // 0x4ad44: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4ad48: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4ad4c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4ad50: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4ad54: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4ad58: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4ad5c: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4ad60: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4ad64: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4ad68: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4ad6c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4ad70: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4ad74: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4ad78: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4ad7c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4ad80: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4ad84: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4ad88: ldr r2, [pc, #-0xf2c]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4ad8c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4ad90: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4ad94: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x4ad98: mov r1, #340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,340)?)?;
    // 0x4ad9c: and r5, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4ada0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4ada4: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4ada8: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4adac: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4adb0: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49608
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4adb4: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4adb8: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4adbc: mov r2, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,68)?)?;
    // 0x4adc0: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4adc4: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4adc8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4adcc: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4add0: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4add4: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4add8: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4addc: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4ade0: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4ade4: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4ade8: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4adec: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4adf0: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4adf4: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4adf8: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4adfc: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4ae00: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4ae04: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4ae08: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4ae0c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4ae10: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4ae14: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4ae18: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4ae1c: ldr r2, [pc, #-0xfc0]       @ 0xe64 <entry+0xe64>
    c.literal(C::Al,R::R2,0x49e64)?;
    // 0x4ae20: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4ae24: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4ae28: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4ae2c: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x4ae30: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4ae34: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4ae38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4ae3c: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4ae40: mov r1, #340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,340)?)?;
    // 0x4ae44: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4ae48: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4ae4c: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x496a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4ae50: add r9, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(5)?,4)?)?;
    // 0x4ae54: ldr r0, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,564)?)?;
    // 0x4ae58: and r1, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,3)?)?;
    // 0x4ae5c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x4ae60: beq 0x4afac <entry+0x1fac>  @ imm = #0x144
    c.branch(C::Eq,0x4afac)?;
    // 0x4ae64: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4ae68: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4ae6c: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4ae70: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4ae74: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4ae78: and r1, r9, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4ae7c: orr r1, r1, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,524288)?)?;
    // 0x4ae80: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4ae84: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4ae88: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4ae8c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4ae90: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4ae94: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x4ae98: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4ae9c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4aea0: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4aea4: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4aea8: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4aeac: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4aeb0: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4aeb4: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4aeb8: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4aebc: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4aec0: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4aec4: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4aec8: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4aecc: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4aed0: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4aed4: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4aed8: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4aedc: ldr r2, [pc, #0xb24]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4aee0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4aee4: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4aee8: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x4aeec: mov r1, #87
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,87)?)?;
    // 0x4aef0: and r5, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4aef4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4aef8: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4aefc: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4af00: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4af04: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4975c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4af08: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4af0c: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4af10: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x4af14: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4af18: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4af1c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4af20: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4af24: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4af28: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4af2c: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4af30: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4af34: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4af38: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4af3c: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4af40: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4af44: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4af48: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4af4c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4af50: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4af54: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4af58: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4af5c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4af60: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4af64: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4af68: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4af6c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4af70: ldr r2, [pc, #0xa90]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4af74: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4af78: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4af7c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4af80: add r0, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,12)?)?;
    // 0x4af84: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4af88: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4af8c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4af90: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4af94: mov r1, #87
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,87)?)?;
    // 0x4af98: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4af9c: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4afa0: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x497f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4afa4: ldr r0, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,564)?)?;
    // 0x4afa8: add r9, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(5)?,4)?)?;
    // 0x4afac: and r0, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,48)?)?;
    // 0x4afb0: cmp r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32)?)?;
    // 0x4afb4: bne 0x4afc8 <entry+0x1fc8>  @ imm = #0xc
    c.branch(C::Ne,0x4afc8)?;
    // 0x4afb8: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4afbc: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x4afc0: orr r4, r4, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,65280)?)?;
    // 0x4afc4: b 0x4b114 <entry+0x2114>  @ imm = #0x148
    c.branch(C::Al,0x4b114)?;
    // 0x4afc8: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4afcc: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4afd0: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4afd4: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4afd8: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4afdc: and r1, r9, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4afe0: orr r1, r1, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,524288)?)?;
    // 0x4afe4: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4afe8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4afec: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4aff0: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4aff4: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4aff8: mov r2, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,21)?)?;
    // 0x4affc: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b000: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4b004: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b008: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b00c: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b010: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b014: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b018: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b01c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b020: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b024: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b028: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b02c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b030: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b034: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b038: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b03c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b040: ldr r2, [pc, #0x9c0]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b044: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b048: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b04c: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x4b050: mov r1, #344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,344)?)?;
    // 0x4b054: and r5, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4b058: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b05c: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4b060: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b064: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b068: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x498c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b06c: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b070: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b074: mov r2, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,21)?)?;
    // 0x4b078: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b07c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b080: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b084: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b088: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b08c: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b090: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b094: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b098: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4b09c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b0a0: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b0a4: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b0a8: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b0ac: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b0b0: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b0b4: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b0b8: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b0bc: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b0c0: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b0c4: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b0c8: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b0cc: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b0d0: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b0d4: ldr r2, [pc, #0x92c]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b0d8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b0dc: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4b0e0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b0e4: add r0, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,12)?)?;
    // 0x4b0e8: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4b0ec: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x4b0f0: and r1, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4b0f4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b0f8: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4b0fc: mov r1, #344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,344)?)?;
    // 0x4b100: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b104: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4b108: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49960
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b10c: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b110: add r9, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(5)?,4)?)?;
    // 0x4b114: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b118: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b11c: and r1, r9, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(9)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4b120: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b124: orr r1, r1, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,524288)?)?;
    // 0x4b128: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b12c: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4b130: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4b134: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4b138: mov r2, #37
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,37)?)?;
    // 0x4b13c: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b140: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4b144: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b148: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b14c: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b150: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b154: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b158: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b15c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b160: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b164: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b168: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b16c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b170: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b174: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b178: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b17c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b180: ldr r2, [pc, #0x880]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b184: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b188: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b18c: add r0, r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // 0x4b190: mov r1, #89
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,89)?)?;
    // 0x4b194: and r5, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4b198: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b19c: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b1a0: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b1a4: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b1a8: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49a00
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b1ac: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b1b0: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b1b4: mov r2, #37
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,37)?)?;
    // 0x4b1b8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b1bc: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b1c0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b1c4: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b1c8: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b1cc: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b1d0: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b1d4: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b1d8: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4b1dc: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b1e0: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b1e4: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b1e8: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b1ec: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b1f0: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b1f4: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b1f8: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b1fc: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b200: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b204: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b208: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b20c: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b210: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b214: ldr r2, [pc, #0x7ec]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b218: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b21c: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4b220: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b224: add r0, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,12)?)?;
    // 0x4b228: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4b22c: and r5, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4b230: mov r1, #89
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,89)?)?;
    // 0x4b234: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b238: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b23c: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b240: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b244: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49a9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b248: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b24c: mov r2, #53
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,53)?)?;
    // 0x4b250: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b254: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4b258: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b25c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b260: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b264: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b268: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b26c: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b270: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b274: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b278: orr r1, r5, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,524288)?)?;
    // 0x4b27c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b280: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b284: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b288: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b28c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b290: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b294: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b298: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b29c: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b2a0: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b2a4: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b2a8: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b2ac: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b2b0: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b2b4: ldr r2, [pc, #0x74c]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b2b8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b2bc: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4b2c0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b2c4: add r0, r9, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,20)?)?;
    // 0x4b2c8: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4b2cc: and r5, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4b2d0: mov r1, #90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,90)?)?;
    // 0x4b2d4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b2d8: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b2dc: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b2e0: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b2e4: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49b3c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b2e8: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b2ec: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b2f0: mov r2, #53
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,53)?)?;
    // 0x4b2f4: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b2f8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b2fc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b300: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b304: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b308: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b30c: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b310: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b314: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4b318: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b31c: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b320: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b324: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b328: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b32c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b330: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b334: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b338: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b33c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b340: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b344: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b348: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b34c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b350: ldr r2, [pc, #0x6b0]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b354: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b358: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4b35c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b360: add r0, r9, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,24)?)?;
    // 0x4b364: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4b368: and r6, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4b36c: mov r1, #90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,90)?)?;
    // 0x4b370: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b374: cmp r6, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,768)?)?;
    // 0x4b378: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b37c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b380: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49bd8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b384: ldr r0, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,564)?)?;
    // 0x4b388: and r1, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,12288)?)?;
    // 0x4b38c: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x4b390: beq 0x4b4d4 <entry+0x24d4>  @ imm = #0x13c
    c.branch(C::Eq,0x4b4d4)?;
    // 0x4b394: ldr r5, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,104)?)?;
    // 0x4b398: mov r2, #69
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,69)?)?;
    // 0x4b39c: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b3a0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4b3a4: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b3a8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x4b3ac: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b3b0: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b3b4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b3b8: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b3bc: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b3c0: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b3c4: orr r1, r6, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(6)?,524288)?)?;
    // 0x4b3c8: mov r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,255)?)?;
    // 0x4b3cc: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b3d0: orr r6, r6, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,65280)?)?;
    // 0x4b3d4: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b3d8: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b3dc: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b3e0: str r3, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,0)?)?;
    // 0x4b3e4: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b3e8: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b3ec: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b3f0: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b3f4: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b3f8: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b3fc: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b400: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b404: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b408: ldr r2, [pc, #0x5f8]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b40c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b410: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x4b414: add r0, r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,32)?)?;
    // 0x4b418: mov r1, #91
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,91)?)?;
    // 0x4b41c: and r5, r0, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x4b420: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b424: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b428: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b42c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b430: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49c88
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b434: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b438: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b43c: mov r2, #69
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,69)?)?;
    // 0x4b440: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b444: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b448: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b44c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b450: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b454: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b458: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b45c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b460: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4b464: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b468: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b46c: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b470: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b474: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b478: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b47c: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b480: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b484: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b488: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b48c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b490: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b494: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b498: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b49c: ldr r2, [pc, #0x564]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b4a0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b4a4: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4b4a8: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b4ac: add r0, r9, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,36)?)?;
    // 0x4b4b0: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4b4b4: and r1, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4b4b8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b4bc: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4b4c0: mov r1, #91
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,91)?)?;
    // 0x4b4c4: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b4c8: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b4cc: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49d24
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b4d0: ldr r0, [sp, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,564)?)?;
    // 0x4b4d4: and r0, r0, #196608
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,196608)?)?;
    // 0x4b4d8: add r8, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(5)?,4)?)?;
    // 0x4b4dc: cmp r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,131072)?)?;
    // 0x4b4e0: bne 0x4b4f4 <entry+0x24f4>  @ imm = #0xc
    c.branch(C::Ne,0x4b4f4)?;
    // 0x4b4e4: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b4e8: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x4b4ec: orr r4, r4, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,65280)?)?;
    // 0x4b4f0: b 0x4b640 <entry+0x2640>  @ imm = #0x148
    c.branch(C::Al,0x4b640)?;
    // 0x4b4f4: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b4f8: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b4fc: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4b500: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b504: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4b508: and r1, r8, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(8)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4b50c: orr r1, r1, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,524288)?)?;
    // 0x4b510: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b514: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b518: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4b51c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4b520: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4b524: mov r2, #85
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,85)?)?;
    // 0x4b528: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b52c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4b530: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b534: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b538: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b53c: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b540: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b544: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b548: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b54c: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b550: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b554: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b558: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b55c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b560: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b564: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b568: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b56c: ldr r2, [pc, #0x494]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b570: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b574: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b578: add r0, r8, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,8)?)?;
    // 0x4b57c: mov r1, #348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,348)?)?;
    // 0x4b580: and r5, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4b584: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b588: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4b58c: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b590: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b594: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49dec
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b598: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b59c: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b5a0: mov r2, #85
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,85)?)?;
    // 0x4b5a4: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b5a8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b5ac: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b5b0: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b5b4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b5b8: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b5bc: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b5c0: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b5c4: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4b5c8: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b5cc: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b5d0: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b5d4: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b5d8: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b5dc: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b5e0: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b5e4: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b5e8: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b5ec: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b5f0: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b5f4: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b5f8: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b5fc: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b600: ldr r2, [pc, #0x400]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b604: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b608: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4b60c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b610: add r0, r8, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,12)?)?;
    // 0x4b614: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4b618: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x4b61c: and r1, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4b620: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b624: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4b628: mov r1, #348
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,348)?)?;
    // 0x4b62c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b630: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4b634: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49e8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b638: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b63c: add r8, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(5)?,4)?)?;
    // 0x4b640: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b644: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b648: and r1, r8, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(8)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4b64c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b650: orr r1, r1, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,524288)?)?;
    // 0x4b654: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b658: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4b65c: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4b660: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4b664: mov r2, #101
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,101)?)?;
    // 0x4b668: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b66c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4b670: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b674: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b678: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b67c: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b680: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b684: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b688: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b68c: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b690: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b694: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b698: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b69c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b6a0: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b6a4: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b6a8: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b6ac: ldr r2, [pc, #0x354]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b6b0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b6b4: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b6b8: add r0, r8, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,8)?)?;
    // 0x4b6bc: mov r1, #93
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,93)?)?;
    // 0x4b6c0: and r5, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4b6c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b6c8: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b6cc: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b6d0: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b6d4: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49f2c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b6d8: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b6dc: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b6e0: mov r2, #101
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,101)?)?;
    // 0x4b6e4: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b6e8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b6ec: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b6f0: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b6f4: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b6f8: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b6fc: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b700: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b704: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4b708: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b70c: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b710: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b714: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b718: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b71c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b720: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b724: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b728: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b72c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b730: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b734: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b738: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b73c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b740: ldr r2, [pc, #0x2c0]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b744: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b748: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4b74c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b750: add r0, r8, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,12)?)?;
    // 0x4b754: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4b758: and r5, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4b75c: mov r1, #93
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,93)?)?;
    // 0x4b760: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b764: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b768: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b76c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b770: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x49fc8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b774: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b778: mov r2, #117
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,117)?)?;
    // 0x4b77c: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b780: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4b784: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b788: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b78c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b790: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b794: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b798: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b79c: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b7a0: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b7a4: orr r1, r5, #524288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,524288)?)?;
    // 0x4b7a8: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b7ac: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b7b0: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b7b4: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b7b8: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b7bc: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b7c0: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b7c4: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b7c8: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b7cc: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b7d0: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b7d4: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b7d8: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b7dc: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b7e0: ldr r2, [pc, #0x220]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b7e4: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b7e8: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4b7ec: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b7f0: add r0, r8, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,20)?)?;
    // 0x4b7f4: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4b7f8: and r5, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4b7fc: mov r1, #94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,94)?)?;
    // 0x4b800: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b804: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b808: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b80c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b810: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a068
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b814: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b818: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b81c: mov r2, #117
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,117)?)?;
    // 0x4b820: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b824: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b828: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b82c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b830: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b834: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b838: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b83c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b840: orr r1, r5, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,262144)?)?;
    // 0x4b844: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b848: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b84c: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b850: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b854: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b858: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b85c: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b860: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b864: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b868: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b86c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b870: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b874: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b878: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b87c: ldr r2, [pc, #0x184]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b880: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b884: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4b888: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b88c: add r0, r8, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,24)?)?;
    // 0x4b890: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4b894: and r5, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4b898: mov r1, #94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,94)?)?;
    // 0x4b89c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b8a0: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b8a4: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b8a8: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b8ac: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a104
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b8b0: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b8b4: ldrb r3, [sp, #0x130]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::SP,304)?)?;
    // 0x4b8b8: orr r1, r5, #8388608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,8388608)?)?;
    // 0x4b8bc: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b8c0: tst r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,16)?)?;
    // 0x4b8c4: and r2, r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,31)?)?;
    // 0x4b8c8: bne 0x4ba0c <entry+0x2a0c>  @ imm = #0x13c
    c.branch(C::Ne,0x4ba0c)?;
    // 0x4b8cc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4b8d0: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b8d4: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4b8d8: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4b8dc: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b8e0: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4b8e4: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4b8e8: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4b8ec: mov r2, #38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,38)?)?;
    // 0x4b8f0: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b8f4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4b8f8: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b8fc: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b900: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b904: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b908: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b90c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b910: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b914: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b918: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b91c: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b920: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b924: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b928: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b92c: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b930: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b934: ldr r2, [pc, #0xcc]         @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b938: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b93c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b940: add r0, r8, #152
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,152)?)?;
    // 0x4b944: mov r1, #97
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,97)?)?;
    // 0x4b948: and r5, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4b94c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b950: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b954: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4b958: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b95c: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a1b4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4b960: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4b964: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x4b968: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4b96c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4b970: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4b974: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4b978: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4b97c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4b980: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4b984: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4b988: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4b98c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4b990: orr r1, r5, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,4194304)?)?;
    // 0x4b994: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4b998: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4b99c: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4b9a0: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4b9a4: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4b9a8: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4b9ac: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b9b0: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4b9b4: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4b9b8: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4b9bc: mov r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,255)?)?;
    // 0x4b9c0: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4b9c4: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4b9c8: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4b9cc: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4b9d0: orr r7, r7, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,65280)?)?;
    // 0x4b9d4: ldr r2, [pc, #0x2c]         @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4b9d8: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4b9dc: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4b9e0: add r0, r8, #216
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,216)?)?;
    // 0x4b9e4: and r1, r0, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x4b9e8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4b9ec: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4b9f0: mov r1, #98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,98)?)?;
    // 0x4b9f4: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4b9f8: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4b9fc: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a254
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4ba00: add r4, r5, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(5)?,64)?)?;
    // 0x4ba04: b 0x4baa8 <entry+0x2aa8>  @ imm = #0x9c
    c.branch(C::Al,0x4baa8)?;
    c.check(0x4ba08)?;
    c.emit(DIAGNOSTIC_CALLBACK)?; // Non-executable literal (branch skips it).
    c.check(0x4ba0c)?;
    // 0x4ba0c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x4ba10: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4ba14: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x4ba18: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4ba1c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4ba20: orr r4, r4, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,65280)?)?;
    // 0x4ba24: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4ba28: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4ba2c: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x4ba30: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4ba34: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4ba38: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4ba3c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4ba40: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4ba44: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4ba48: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4ba4c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4ba50: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4ba54: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4ba58: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4ba5c: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4ba60: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4ba64: mov r7, r4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(4)?))?;
    // 0x4ba68: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4ba6c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4ba70: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4ba74: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4ba78: ldr r2, [pc, #-0x78]        @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4ba7c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4ba80: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4ba84: add r0, r8, #152
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,152)?)?;
    // 0x4ba88: and r1, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4ba8c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4ba90: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x4ba94: mov r1, #101
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,101)?)?;
    // 0x4ba98: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4ba9c: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4baa0: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a2f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4baa4: add r4, r5, #128
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(5)?,128)?)?;
    // 0x4baa8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4baac: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4bab0: and r1, r4, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(4)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x4bab4: mov r5, r7
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(7)?))?;
    // 0x4bab8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4babc: orr r1, r1, #3145728
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3145728)?)?;
    // 0x4bac0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4bac4: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4bac8: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4bacc: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4bad0: mov r2, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,22)?)?;
    // 0x4bad4: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4bad8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4badc: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4bae0: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4bae4: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4bae8: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4baec: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4baf0: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4baf4: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4baf8: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bafc: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4bb00: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bb04: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4bb08: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4bb0c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4bb10: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4bb14: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4bb18: ldr r2, [pc, #-0x118]       @ 0x2a08 <entry+0x2a08>
    c.literal(C::Al,R::R2,0x4ba08)?;
    // 0x4bb1c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4bb20: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4bb24: add r0, r4, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,48)?)?;
    // 0x4bb28: mov r1, #103
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,103)?)?;
    // 0x4bb2c: and r5, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4bb30: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4bb34: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4bb38: cmp r5, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,768)?)?;
    // 0x4bb3c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4bb40: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a398
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4bb44: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4bb48: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4bb4c: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x4bb50: add r4, r4, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,64)?)?;
    // 0x4bb54: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4bb58: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4bb5c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4bb60: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4bb64: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4bb68: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4bb6c: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4bb70: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4bb74: orr r1, r5, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,1048576)?)?;
    // 0x4bb78: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4bb7c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4bb80: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4bb84: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4bb88: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4bb8c: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4bb90: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4bb94: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4bb98: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bb9c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4bba0: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bba4: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4bba8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x4bbac: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4bbb0: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4bbb4: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4bbb8: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4bbbc: ldr r2, [pc, #0xfac]        @ 0x3b70 <entry+0x3b70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4bbc0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4bbc4: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4bbc8: and r0, r4, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4bbcc: mov r1, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,360)?)?;
    // 0x4bbd0: cmp r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,768)?)?;
    // 0x4bbd4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4bbd8: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4bbdc: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4bbe0: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a438
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4bbe4: mov r12, r4
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(4)?))?;
    // 0x4bbe8: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x4bbec: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x4bbf0: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4bbf4: mov r9, r12
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R12))?;
    // 0x4bbf8: mov r4, r5
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // 0x4bbfc: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x4bc00: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4bc04: and r10, r9, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(10)?,R::new(9)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4bc08: orr r2, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x4bc0c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4bc10: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4bc14: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4bc18: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4bc1c: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4bc20: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4bc24: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4bc28: orr r1, r10, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(10)?,262144)?)?;
    // 0x4bc2c: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4bc30: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4bc34: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4bc38: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4bc3c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4bc40: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4bc44: ldreq r5, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::new(3)?,0)?)?;
    // 0x4bc48: andeq r5, r5, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x4bc4c: streq r5, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::new(3)?,0)?)?;
    // 0x4bc50: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4bc54: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x4bc58: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4bc5c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4bc60: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4bc64: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4bc68: ldr r2, [pc, #0xf00]        @ 0x3b70 <entry+0x3b70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4bc6c: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4bc70: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4bc74: add r0, r12, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R12,4)?)?;
    // 0x4bc78: mov r1, #106
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,106)?)?;
    // 0x4bc7c: and r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x4bc80: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4bc84: cmp r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,768)?)?;
    // 0x4bc88: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4bc8c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4bc90: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a4e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4bc94: add r8, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // 0x4bc98: add r7, r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,16)?)?;
    // 0x4bc9c: add r12, r10, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(10)?,4)?)?;
    // 0x4bca0: cmp r8, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,3)?)?;
    // 0x4bca4: bne 0x4bbec <entry+0x2bec>  @ imm = #-0xc0
    c.branch(C::Ne,0x4bbec)?;
    // 0x4bca8: add r0, r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,4)?)?;
    // 0x4bcac: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4bcb0: mov r8, r12
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R12))?;
    // 0x4bcb4: and r1, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4bcb8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4bcbc: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4bcc0: orr r1, r1, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,262144)?)?;
    // 0x4bcc4: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4bcc8: ldreq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
    // 0x4bccc: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x4bcd0: streq r2, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
    // 0x4bcd4: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // 0x4bcd8: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4bcdc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4bce0: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4bce4: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4bce8: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4bcec: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4bcf0: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4bcf4: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4bcf8: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4bcfc: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bd00: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4bd04: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bd08: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4bd0c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4bd10: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4bd14: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4bd18: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4bd1c: ldr r2, [pc, #0xe4c]        @ 0x3b70 <entry+0x3b70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4bd20: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4bd24: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4bd28: add r0, r12, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R12,4)?)?;
    // 0x4bd2c: mov r1, #364
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,364)?)?;
    // 0x4bd30: and r4, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4bd34: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4bd38: orr r1, r1, #10240
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10240)?)?;
    // 0x4bd3c: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4bd40: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4bd44: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a59c
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4bd48: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4bd4c: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4bd50: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // 0x4bd54: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4bd58: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4bd5c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4bd60: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4bd64: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4bd68: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4bd6c: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4bd70: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4bd74: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4bd78: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4bd7c: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4bd80: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4bd84: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4bd88: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4bd8c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4bd90: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bd94: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4bd98: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bd9c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4bda0: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4bda4: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4bda8: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4bdac: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4bdb0: ldr r2, [pc, #0xdb8]        @ 0x3b70 <entry+0x3b70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4bdb4: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4bdb8: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4bdbc: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4bdc0: add r0, r8, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,8)?)?;
    // 0x4bdc4: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4bdc8: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4bdcc: mov r1, #109
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,109)?)?;
    // 0x4bdd0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4bdd4: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4bdd8: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4bddc: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4bde0: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a638
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4bde4: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4bde8: mov r2, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,88)?)?;
    // 0x4bdec: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4bdf0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4bdf4: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4bdf8: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4bdfc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4be00: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4be04: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4be08: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4be0c: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4be10: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4be14: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4be18: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4be1c: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4be20: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4be24: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4be28: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4be2c: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4be30: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4be34: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4be38: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4be3c: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4be40: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4be44: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4be48: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4be4c: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4be50: ldr r2, [pc, #0xd18]        @ 0x3b70 <entry+0x3b70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4be54: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4be58: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4be5c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4be60: add r0, r8, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,12)?)?;
    // 0x4be64: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4be68: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4be6c: mov r1, #110
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,110)?)?;
    // 0x4be70: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4be74: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4be78: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4be7c: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4be80: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a6d8
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4be84: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4be88: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4be8c: mov r2, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,88)?)?;
    // 0x4be90: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4be94: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4be98: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4be9c: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4bea0: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4bea4: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4bea8: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4beac: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4beb0: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4beb4: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4beb8: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4bebc: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4bec0: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4bec4: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4bec8: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4becc: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bed0: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4bed4: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bed8: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4bedc: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4bee0: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4bee4: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4bee8: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4beec: ldr r2, [pc, #0xc7c]        @ 0x3b70 <entry+0x3b70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4bef0: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4bef4: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x4bef8: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4befc: add r0, r8, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,16)?)?;
    // 0x4bf00: orr r1, r1, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    // 0x4bf04: and r4, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x4bf08: mov r1, #111
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,111)?)?;
    // 0x4bf0c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4bf10: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4bf14: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4bf18: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4bf1c: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a774
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4bf20: ldr r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,88)?)?;
    // 0x4bf24: ldr r0, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2744)?)?;
    // 0x4bf28: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x4bf2c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x4bf30: beq 0x4c69c <entry+0x369c>  @ imm = #0x764
    c.branch(C::Eq,0x4c69c)?;
    // 0x4bf34: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4bf38: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x4bf3c: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4bf40: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x4bf44: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x4bf48: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4bf4c: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x4bf50: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4bf54: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4bf58: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4bf5c: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4bf60: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4bf64: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    // 0x4bf68: orr r1, r4, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(4)?,262144)?)?;
    // 0x4bf6c: mov r4, r8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
    // 0x4bf70: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x4bf74: add r3, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,1)?)?;
    // 0x4bf78: str r3, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-36)?)?;
    // 0x4bf7c: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x4bf80: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x4bf84: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x4bf88: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bf8c: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x4bf90: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x4bf94: mov r3, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15232)?)?;
    // 0x4bf98: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x4bf9c: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    // 0x4bfa0: add r1, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,2)?)?;
    // 0x4bfa4: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,16)?)?;
    // 0x4bfa8: ldr r2, [pc, #0xbc0]        @ 0x3b70 <entry+0x3b70>
    c.literal(C::Al,R::R2,0x4cb70)?;
    // 0x4bfac: str r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-36)?)?;
    // 0x4bfb0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x4bfb4: add r0, r4, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,20)?)?;
    // 0x4bfb8: mov r1, #115
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,115)?)?;
    // 0x4bfbc: and r4, r0, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x4bfc0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x4bfc4: orr r1, r1, #10496
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,10496)?)?;
    // 0x4bfc8: cmp r4, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,768)?)?;
    // 0x4bfcc: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x4bfd0: bl 0x17b0 <entry+0xfffffffffffb87b0> @ imm = #-0x4a828
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x4bfd4: ldr r6, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,104)?)?;
    // 0x4bfd8: mov r3, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,900)?)?;
    // 0x4bfdc: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x4bfe0: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x4bfe4: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x4bfe8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x4bfec: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x4bff0: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x4bff4: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x4bff8: str r3, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    // 0x4bffc: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String> {
    let mut c=Code::new();emit_stream_class_commands(&mut c)?;Ok(c.bytes)
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn class_controller_literal_is_not_code() {
        let buf=fragment().unwrap();
        assert_eq!(buf.len(),END-START);
        let i=0x4ba08-START;
        assert_eq!(u32::from_le_bytes(buf[i..i+4].try_into().unwrap()),DIAGNOSTIC_CALLBACK);
        let callsite=0x4b9d4-START;
        assert_eq!(u32::from_le_bytes(buf[callsite..callsite+4].try_into().unwrap()),
            arm32::transfer_word(C::Al,true,R::R2,R::PC,0x2c).unwrap());
        assert_eq!(u32::from_le_bytes(buf[0x4ba04-START..0x4ba08-START].try_into().unwrap()),
            arm32::branch(C::Al,0x4ba04,0x4baa8,false).unwrap());
    }
}
