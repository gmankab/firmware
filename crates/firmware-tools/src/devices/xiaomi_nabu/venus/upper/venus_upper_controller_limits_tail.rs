// SPDX-License-Identifier: GPL-3.0-only
//! Controller adaptive-limit selection: finish the source's exhaustive
//! multi-plane codec statistics and limit fitting state machine, including
//! signed fixed-point variance, rolling thresholds and negative-clamp paths.
//! This is a tail of the routine beginning at 0x71ed0; the prior verified
//! module covers the prologue. No placeholder bytes or source OEM opcodes.
use arm32::{branch, branch_exchange_immediate, data_immediate,
    data_immediate_with_flags, data_register, mov_register, mov_shifted_register,
    multiply_register, load_halfword, load_signed_halfword,
    store_halfword, transfer_byte,
    transfer_byte_preindexed_writeback, transfer_word,
    transfer_word_register,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op, Register as R,
    Shift};

pub const START:usize=0x720f4;
pub const END:usize=0x728cc;
pub const FILE_BASE:usize=0x4000;
const RANGE_DIAGNOSTIC:u32=0x000f_3172;
const TRACE_FOOTER:u32=0x000e_2788;
const DIVIDE_BY_THREE_RECIPROCAL:u32=0x5555_5556;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,addr:usize)->Result<(),String>{
        if self.at==addr {Ok(())}else{Err(format!("adaptive controller tail 0x{:x} != 0x{addr:x}",self.at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START||self.at>=END||self.at%4!=0{return Err(format!("controller tail output at 0x{:x}",self.at));}
        self.bytes.extend(word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize,link:bool)->Result<(),String>{
        self.emit(branch(cond,self.at as u32,target as u32,link)?)
    }
    fn literal(&mut self,dst:R,target:usize)->Result<(),String>{
        self.emit(transfer_word(C::Al,true,dst,R::PC,target as i32-self.at as i32-8)?)
    }
}
fn r(n:u8)->R{R::new(n).expect("register number in 0..15")}
fn checked_mul(c:&mut Code,base:u32,lo:R,hi:R,left:R,right:R)->Result<(),String>{
    if lo==hi||[lo,hi,left,right].iter().any(|x|x.number()>=13) {return Err("unsupported signed/unsigned long multiply registers".into());}
    c.emit(base|(hi.number()<<16)|(lo.number()<<12)|(right.number()<<8)|left.number())
}
fn umull(c:&mut Code,lo:R,hi:R,left:R,right:R)->Result<(),String>{
    checked_mul(c,0xe080_0090,lo,hi,left,right)
}
fn umlal(c:&mut Code,lo:R,hi:R,left:R,right:R)->Result<(),String>{
    checked_mul(c,0xe0a0_0090,lo,hi,left,right)
}
fn smull(c:&mut Code,lo:R,hi:R,left:R,right:R)->Result<(),String>{
    checked_mul(c,0xe0c0_0090,lo,hi,left,right)
}

fn finish_controller_limits(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x720f4: add r6, r3, #4
    c.emit(data_immediate(C::Al,Op::Add,r(6),r(3),4)?)?;
    // 0x720f8: cmp r6, r7
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(6),r(7),Shift::Lsl,0,false)?)?;
    // 0x720fc: bhs 0x72338 <entry+0xd4c>   @ imm = #0x234
    c.branch(C::Cs,0x72338,false)?;
    // 0x72100: ldr r1, [pc, #0x7c0]        @ 0x12dc <entry+0x12dc>
    c.literal(r(1),0x728c8)?;
    // 0x72104: sub r0, r7, r3
    c.emit(data_register(C::Al,Op::Sub,r(0),r(7),r(3),Shift::Lsl,0,false)?)?;
    // 0x72108: lsl r0, r0, #1
    c.emit(mov_shifted_register(C::Al,r(0),r(0),Shift::Lsl,1)?)?;
    // 0x7210c: smull r2, r7, r0, r1
    smull(c,r(2),r(7),r(0),r(1))?;
    // 0x72110: mov r1, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,1)?)?;
    // 0x72114: add r0, r7, r7, lsr #31
    c.emit(data_register(C::Al,Op::Add,r(0),r(7),r(7),Shift::Lsr,31,false)?)?;
    // 0x72118: add r0, r0, r3
    c.emit(data_register(C::Al,Op::Add,r(0),r(0),r(3),Shift::Lsl,0,false)?)?;
    // 0x7211c: b 0x72360 <entry+0xd74>   @ imm = #0x23c
    c.branch(C::Al,0x72360,false)?;
    // 0x72120: ldr r10, [r7, #0x4]
    c.emit(transfer_word(C::Al,true,r(10),r(7),4)?)?;
    // 0x72124: cmp r10, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(10),0)?)?;
    // 0x72128: beq 0x723bc <entry+0xdd0>   @ imm = #0x28c
    c.branch(C::Eq,0x723bc,false)?;
    // 0x7212c: str r3, [sp, #0x24]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,36)?)?;
    // 0x72130: str r9, [sp, #0x20]
    c.emit(transfer_word(C::Al,false,r(9),R::SP,32)?)?;
    // 0x72134: mov r0, r4
    c.emit(mov_register(C::Al,r(0),r(4)))?;
    // 0x72138: mov r1, r7
    c.emit(mov_register(C::Al,r(1),r(7)))?;
    // 0x7213c: ldrb r2, [r6, #0x2]
    c.emit(transfer_byte(C::Al,true,r(2),r(6),2)?)?;
    // 0x72140: ldr r5, [r6, #0x70]
    c.emit(transfer_word(C::Al,true,r(5),r(6),112)?)?;
    // 0x72144: str r6, [r11, #-0x20]
    c.emit(transfer_word(C::Al,false,r(6),r(11),-32)?)?;
    // 0x72148: ldr r9, [r6, #0x74]
    c.emit(transfer_word(C::Al,true,r(9),r(6),116)?)?;
    // 0x7214c: str r2, [sp, #0x10]
    c.emit(transfer_word(C::Al,false,r(2),R::SP,16)?)?;
    // 0x72150: bl 0x70e80 <entry+0xfffffffffffff894> @ imm = #-0x12d8
    c.branch(C::Al,0x70e80,true)?;
    // 0x72154: cmp r5, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),0)?)?;
    // 0x72158: str r0, [sp, #0x14]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,20)?)?;
    // 0x7215c: beq 0x721b4 <entry+0xbc8>   @ imm = #0x50
    c.branch(C::Eq,0x721b4,false)?;
    // 0x72160: ldr r6, [r7, #0x18]
    c.emit(transfer_word(C::Al,true,r(6),r(7),24)?)?;
    // 0x72164: cmp r6, #21
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(6),21)?)?;
    // 0x72168: blo 0x721b4 <entry+0xbc8>   @ imm = #0x44
    c.branch(C::Cc,0x721b4,false)?;
    // 0x7216c: umull r0, r3, r5, r5
    umull(c,r(0),r(3),r(5),r(5))?;
    // 0x72170: umlal r0, r3, r9, r9
    umlal(c,r(0),r(3),r(9),r(9))?;
    // 0x72174: lsr r0, r0, #8
    c.emit(mov_shifted_register(C::Al,r(0),r(0),Shift::Lsr,8)?)?;
    // 0x72178: orr r2, r0, r3, lsl #24
    c.emit(data_register(C::Al,Op::Orr,r(2),r(0),r(3),Shift::Lsl,24,false)?)?;
    // 0x7217c: orr r0, r2, r3, lsr #8
    c.emit(data_register(C::Al,Op::Orr,r(0),r(2),r(3),Shift::Lsr,8,false)?)?;
    // 0x72180: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72184: beq 0x721b4 <entry+0xbc8>   @ imm = #0x28
    c.branch(C::Eq,0x721b4,false)?;
    // 0x72188: umull r0, r1, r6, r5
    umull(c,r(0),r(1),r(6),r(5))?;
    // 0x7218c: mov r6, r7
    c.emit(mov_register(C::Al,r(6),r(7)))?;
    // 0x72190: ldr r7, [r6, #0x1c]
    c.emit(transfer_word(C::Al,true,r(7),r(6),28)?)?;
    // 0x72194: umlal r0, r1, r7, r9
    umlal(c,r(0),r(1),r(7),r(9))?;
    // 0x72198: orrs r7, r0, r1
    c.emit(data_register(C::Al,Op::Orr,r(7),r(0),r(1),Shift::Lsl,0,true)?)?;
    // 0x7219c: mov r7, r6
    c.emit(mov_register(C::Al,r(7),r(6)))?;
    // 0x721a0: beq 0x721b4 <entry+0xbc8>   @ imm = #0xc
    c.branch(C::Eq,0x721b4,false)?;
    // 0x721a4: lsr r3, r3, #8
    c.emit(mov_shifted_register(C::Al,r(3),r(3),Shift::Lsr,8)?)?;
    // 0x721a8: bl 0x8187c <entry+0x10290> @ imm = #0xf6cc
    c.branch(C::Al,0x8187c,true)?;
    // 0x721ac: ldr r1, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(1),r(11),-32)?)?;
    // 0x721b0: str r0, [r1, #0x88]
    c.emit(transfer_word(C::Al,false,r(0),r(1),136)?)?;
    // 0x721b4: ldr r6, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(6),r(11),-32)?)?;
    // 0x721b8: str r5, [sp, #0x18]
    c.emit(transfer_word(C::Al,false,r(5),R::SP,24)?)?;
    // 0x721bc: mov r9, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(9),R::R0,0)?)?;
    // 0x721c0: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x721c4: ldr r5, [r4, #0x2e4]
    c.emit(transfer_word(C::Al,true,r(5),r(4),740)?)?;
    // 0x721c8: ldr r1, [r6, #0x88]
    c.emit(transfer_word(C::Al,true,r(1),r(6),136)?)?;
    // 0x721cc: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x721d0: beq 0x721e4 <entry+0xbf8>   @ imm = #0xc
    c.branch(C::Eq,0x721e4,false)?;
    // 0x721d4: subs r2, r5, r10
    c.emit(data_register(C::Al,Op::Sub,r(2),r(5),r(10),Shift::Lsl,0,true)?)?;
    // 0x721d8: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x721dc: lslhi r0, r2, #8
    c.emit(mov_shifted_register(C::Hi,r(0),r(2),Shift::Lsl,8)?)?;
    // 0x721e0: blx 0x83c4c <entry+0x12660> @ imm = #0x11a64
    c.emit(branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x721e4: str r0, [r6, #0x80]
    c.emit(transfer_word(C::Al,false,r(0),r(6),128)?)?;
    // 0x721e8: ldrb r1, [r4, #0x30]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),48)?)?;
    // 0x721ec: str r7, [sp, #0x1c]
    c.emit(transfer_word(C::Al,false,r(7),R::SP,28)?)?;
    // 0x721f0: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x721f4: beq 0x723fc <entry+0xe10>   @ imm = #0x200
    c.branch(C::Eq,0x723fc,false)?;
    // 0x721f8: ldr lr, [sp, #0x20]
    c.emit(transfer_word(C::Al,true,R::LR,R::SP,32)?)?;
    // 0x721fc: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x72200: cmp r5, r10
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(10),Shift::Lsl,0,false)?)?;
    // 0x72204: bls 0x723cc <entry+0xde0>   @ imm = #0x1c0
    c.branch(C::Ls,0x723cc,false)?;
    // 0x72208: ldr r1, [sp, #0x18]
    c.emit(transfer_word(C::Al,true,r(1),R::SP,24)?)?;
    // 0x7220c: cmp r1, #4
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),4)?)?;
    // 0x72210: blo 0x724c8 <entry+0xedc>   @ imm = #0x2b0
    c.branch(C::Cc,0x724c8,false)?;
    // 0x72214: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72218: beq 0x724c8 <entry+0xedc>   @ imm = #0x2a8
    c.branch(C::Eq,0x724c8,false)?;
    // 0x7221c: cmp r9, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(9),0)?)?;
    // 0x72220: str r9, [sp, #0x18]
    c.emit(transfer_word(C::Al,false,r(9),R::SP,24)?)?;
    // 0x72224: str r3, [sp, #0xc]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,12)?)?;
    // 0x72228: beq 0x724ec <entry+0xf00>   @ imm = #0x2bc
    c.branch(C::Eq,0x724ec,false)?;
    // 0x7222c: ldr r3, [sp, #0x10]
    c.emit(transfer_word(C::Al,true,r(3),R::SP,16)?)?;
    // 0x72230: ldr r1, [pc, #0x688]        @ 0x12d4 <entry+0x12d4>
    c.literal(r(1),0x728c0)?;
    // 0x72234: sub r0, r7, r3
    c.emit(data_register(C::Al,Op::Sub,r(0),r(7),r(3),Shift::Lsl,0,false)?)?;
    // 0x72238: str r0, [sp, #0x8]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,8)?)?;
    // 0x7223c: lsl r5, r0, #24
    c.emit(mov_shifted_register(C::Al,r(5),r(0),Shift::Lsl,24)?)?;
    // 0x72240: ldrb r2, [r6, #0x28c]
    c.emit(transfer_byte(C::Al,true,r(2),r(6),652)?)?;
    // 0x72244: asr r0, r5, #24
    c.emit(mov_shifted_register(C::Al,r(0),r(5),Shift::Asr,24)?)?;
    // 0x72248: str r0, [sp]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,0)?)?;
    // 0x7224c: mov r0, #2
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,2)?)?;
    // 0x72250: bl 0x2ec94 <entry+0xfffffffffffbd6a8> @ imm = #-0x435c4
    c.branch(C::Al,0x2ec94,true)?;
    // 0x72254: ldr r1, [pc, #0x668]        @ 0x12d8 <entry+0x12d8>
    c.literal(r(1),0x728c4)?;
    // 0x72258: mov r0, #2
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,2)?)?;
    // 0x7225c: bl 0x2ec94 <entry+0xfffffffffffbd6a8> @ imm = #-0x435d0
    c.branch(C::Al,0x2ec94,true)?;
    // 0x72260: ldr lr, [sp, #0x20]
    c.emit(transfer_word(C::Al,true,R::LR,R::SP,32)?)?;
    // 0x72264: ldr r0, [sp, #0x24]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,36)?)?;
    // 0x72268: ldrb r10, [r6, #0xb]
    c.emit(transfer_byte(C::Al,true,r(10),r(6),11)?)?;
    // 0x7226c: and r1, lr, #255
    c.emit(data_immediate(C::Al,Op::And,r(1),R::LR,255)?)?;
    // 0x72270: and r0, r0, #255
    c.emit(data_immediate(C::Al,Op::And,r(0),r(0),255)?)?;
    // 0x72274: mov r9, r10
    c.emit(mov_register(C::Al,r(9),r(10)))?;
    // 0x72278: add r1, r1, r5, asr #24
    c.emit(data_register(C::Al,Op::Add,r(1),r(1),r(5),Shift::Asr,24,false)?)?;
    // 0x7227c: add r0, r0, r5, asr #24
    c.emit(data_register(C::Al,Op::Add,r(0),r(0),r(5),Shift::Asr,24,false)?)?;
    // 0x72280: and r2, r1, #255
    c.emit(data_immediate(C::Al,Op::And,r(2),r(1),255)?)?;
    // 0x72284: cmp r2, r10
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(10),Shift::Lsl,0,false)?)?;
    // 0x72288: blo 0x7229c <entry+0xcb0>   @ imm = #0xc
    c.branch(C::Cc,0x7229c,false)?;
    // 0x7228c: ldrb r3, [r6, #0xc]
    c.emit(transfer_byte(C::Al,true,r(3),r(6),12)?)?;
    // 0x72290: mov r9, r1
    c.emit(mov_register(C::Al,r(9),r(1)))?;
    // 0x72294: cmp r2, r3
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(3),Shift::Lsl,0,false)?)?;
    // 0x72298: movhi r9, r3
    c.emit(mov_register(C::Hi,r(9),r(3)))?;
    // 0x7229c: and r1, r0, #255
    c.emit(data_immediate(C::Al,Op::And,r(1),r(0),255)?)?;
    // 0x722a0: cmp r1, r10
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(1),r(10),Shift::Lsl,0,false)?)?;
    // 0x722a4: blo 0x724fc <entry+0xf10>   @ imm = #0x250
    c.branch(C::Cc,0x724fc,false)?;
    // 0x722a8: ldrb r10, [r6, #0xc]
    c.emit(transfer_byte(C::Al,true,r(10),r(6),12)?)?;
    // 0x722ac: cmp r1, r10
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(1),r(10),Shift::Lsl,0,false)?)?;
    // 0x722b0: movls r10, r0
    c.emit(mov_register(C::Ls,r(10),r(0)))?;
    // 0x722b4: b 0x724fc <entry+0xf10>   @ imm = #0x240
    c.branch(C::Al,0x724fc,false)?;
    // 0x722b8: ldr r1, [r4, #0x2d8]
    c.emit(transfer_word(C::Al,true,r(1),r(4),728)?)?;
    // 0x722bc: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x722c0: beq 0x723e0 <entry+0xdf4>   @ imm = #0x118
    c.branch(C::Eq,0x723e0,false)?;
    // 0x722c4: cmp r1, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),2)?)?;
    // 0x722c8: bne 0x72010 <entry+0xa24>   @ imm = #-0x2c0
    c.branch(C::Ne,0x72010,false)?;
    // 0x722cc: mov r1, r4
    c.emit(mov_register(C::Al,r(1),r(4)))?;
    // 0x722d0: ldrb r2, [r1, #0x2d1]!
    c.emit(transfer_byte_preindexed_writeback(C::Al,true,r(2),r(1),721)?)?;
    // 0x722d4: ldr r3, [r1, #-0x39]
    c.emit(transfer_word(C::Al,true,r(3),r(1),-57)?)?;
    // 0x722d8: add r0, r2, #2
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(2),2)?)?;
    // 0x722dc: cmp r3, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),0)?)?;
    // 0x722e0: beq 0x72010 <entry+0xa24>   @ imm = #-0x2d8
    c.branch(C::Eq,0x72010,false)?;
    // 0x722e4: mov r3, r6
    c.emit(mov_register(C::Al,r(3),r(6)))?;
    // 0x722e8: mov r5, r7
    c.emit(mov_register(C::Al,r(5),r(7)))?;
    // 0x722ec: ldrb r7, [r3, #0xb]!
    c.emit(transfer_byte_preindexed_writeback(C::Al,true,r(7),r(3),11)?)?;
    // 0x722f0: cmp r2, r7
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(7),Shift::Lsl,0,false)?)?;
    // 0x722f4: ldrb r10, [r3, #0x1]
    c.emit(transfer_byte(C::Al,true,r(10),r(3),1)?)?;
    // 0x722f8: mov r7, r5
    c.emit(mov_register(C::Al,r(7),r(5)))?;
    // 0x722fc: movls r1, r3
    c.emit(mov_register(C::Ls,r(1),r(3)))?;
    // 0x72300: ldrb r9, [r1]
    c.emit(transfer_byte(C::Al,true,r(9),r(1),0)?)?;
    // 0x72304: cmp r0, r10
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(0),r(10),Shift::Lsl,0,false)?)?;
    // 0x72308: movlo r10, r0
    c.emit(mov_register(C::Cc,r(10),r(0)))?;
    // 0x7230c: b 0x72010 <entry+0xa24>   @ imm = #-0x304
    c.branch(C::Al,0x72010,false)?;
    // 0x72310: ldr r2, [r4, #0x34]
    c.emit(transfer_word(C::Al,true,r(2),r(4),52)?)?;
    // 0x72314: ldrb r1, [r4, #0x2d2]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),722)?)?;
    // 0x72318: sub r2, r2, #1
    c.emit(data_immediate(C::Al,Op::Sub,r(2),r(2),1)?)?;
    // 0x7231c: cmp r2, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(1),Shift::Lsl,0,false)?)?;
    // 0x72320: bne 0x72024 <entry+0xa38>   @ imm = #-0x304
    c.branch(C::Ne,0x72024,false)?;
    // 0x72324: ldrb r1, [r4, #0x7d]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),125)?)?;
    // 0x72328: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x7232c: ldrbeq r0, [r4, #0x2d1]
    c.emit(transfer_byte(C::Eq,true,r(0),r(4),721)?)?;
    // 0x72330: addeq r0, r0, #2
    c.emit(data_immediate(C::Eq,Op::Add,r(0),r(0),2)?)?;
    // 0x72334: b 0x72024 <entry+0xa38>   @ imm = #-0x318
    c.branch(C::Al,0x72024,false)?;
    // 0x72338: ldrb r3, [r4, #0x27e]
    c.emit(transfer_byte(C::Al,true,r(3),r(4),638)?)?;
    // 0x7233c: cmp r3, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),0)?)?;
    // 0x72340: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x72344: moveq r3, #1
    c.emit(data_immediate(C::Eq,Op::Mov,r(3),R::R0,1)?)?;
    // 0x72348: orrs r2, r3, r2
    c.emit(data_register(C::Al,Op::Orr,r(2),r(3),r(2),Shift::Lsl,0,true)?)?;
    // 0x7234c: bne 0x72360 <entry+0xd74>   @ imm = #0xc
    c.branch(C::Ne,0x72360,false)?;
    // 0x72350: ldrb r2, [r4, #0x1c]
    c.emit(transfer_byte(C::Al,true,r(2),r(4),28)?)?;
    // 0x72354: cmp r2, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),0)?)?;
    // 0x72358: ldrbne r0, [r4, #0x2d1]
    c.emit(transfer_byte(C::Ne,true,r(0),r(4),721)?)?;
    // 0x7235c: addne r0, r0, #255
    c.emit(data_immediate(C::Ne,Op::Add,r(0),r(0),255)?)?;
    // 0x72360: strb r1, [r4, #0x27e]
    c.emit(transfer_byte(C::Al,false,r(1),r(4),638)?)?;
    // 0x72364: ldr r1, [r4, #0x64]
    c.emit(transfer_word(C::Al,true,r(1),r(4),100)?)?;
    // 0x72368: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x7236c: bne 0x7237c <entry+0xd90>   @ imm = #0x8
    c.branch(C::Ne,0x7237c,false)?;
    // 0x72370: ldr r1, [r4, #0x68]
    c.emit(transfer_word(C::Al,true,r(1),r(4),104)?)?;
    // 0x72374: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x72378: beq 0x723c0 <entry+0xdd4>   @ imm = #0x40
    c.branch(C::Eq,0x723c0,false)?;
    // 0x7237c: ldrb r1, [r4, #0x7d]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),125)?)?;
    // 0x72380: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x72384: beq 0x723ac <entry+0xdc0>   @ imm = #0x20
    c.branch(C::Eq,0x723ac,false)?;
    // 0x72388: mov r1, #2864
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,2864)?)?;
    // 0x7238c: ldrb r2, [r4, #0x2d2]
    c.emit(transfer_byte(C::Al,true,r(2),r(4),722)?)?;
    // 0x72390: orr r1, r1, #4096
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),4096)?)?;
    // 0x72394: ldr r1, [r4, r1]
    c.emit(transfer_word_register(C::Al,true,r(1),r(4),r(1),Shift::Lsl,0)?)?;
    // 0x72398: add r1, r1, r2, lsl #3
    c.emit(data_register(C::Al,Op::Add,r(1),r(1),r(2),Shift::Lsl,3,false)?)?;
    // 0x7239c: ldr r1, [r1, #0x9c]
    c.emit(transfer_word(C::Al,true,r(1),r(1),156)?)?;
    // 0x723a0: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x723a4: movlt r0, r10
    c.emit(mov_register(C::Lt,r(0),r(10)))?;
    // 0x723a8: b 0x723c0 <entry+0xdd4>   @ imm = #0x10
    c.branch(C::Al,0x723c0,false)?;
    // 0x723ac: ldr r1, [r4, #0x2c4]
    c.emit(transfer_word(C::Al,true,r(1),r(4),708)?)?;
    // 0x723b0: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x723b4: movlt r0, r10
    c.emit(mov_register(C::Lt,r(0),r(10)))?;
    // 0x723b8: b 0x723c0 <entry+0xdd4>   @ imm = #0x0
    c.branch(C::Al,0x723c0,false)?;
    // 0x723bc: mov r0, r8
    c.emit(mov_register(C::Al,r(0),r(8)))?;
    // 0x723c0: and r0, r0, #255
    c.emit(data_immediate(C::Al,Op::And,r(0),r(0),255)?)?;
    // 0x723c4: sub sp, r11, #28
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,r(11),28)?)?;
    // 0x723c8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x723cc: ldr r10, [sp, #0x24]
    c.emit(transfer_word(C::Al,true,r(10),R::SP,36)?)?;
    // 0x723d0: ldr r0, [sp, #0x14]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,20)?)?;
    // 0x723d4: ldr r7, [sp, #0x1c]
    c.emit(transfer_word(C::Al,true,r(7),R::SP,28)?)?;
    // 0x723d8: mov r9, lr
    c.emit(mov_register(C::Al,r(9),R::LR))?;
    // 0x723dc: b 0x72004 <entry+0xa18>   @ imm = #-0x3e0
    c.branch(C::Al,0x72004,false)?;
    // 0x723e0: ldr r1, [r4, #0x68]
    c.emit(transfer_word(C::Al,true,r(1),r(4),104)?)?;
    // 0x723e4: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x723e8: beq 0x72010 <entry+0xa24>   @ imm = #-0x3e0
    c.branch(C::Eq,0x72010,false)?;
    // 0x723ec: ldrb r0, [r4, #0x2d1]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),721)?)?;
    // 0x723f0: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x723f4: subne r0, r0, #1
    c.emit(data_immediate(C::Ne,Op::Sub,r(0),r(0),1)?)?;
    // 0x723f8: b 0x72010 <entry+0xa24>   @ imm = #-0x3f0
    c.branch(C::Al,0x72010,false)?;
    // 0x723fc: ldrb r12, [r4, #0x8a]
    c.emit(transfer_byte(C::Al,true,r(12),r(4),138)?)?;
    // 0x72400: ldr lr, [sp, #0x20]
    c.emit(transfer_word(C::Al,true,R::LR,R::SP,32)?)?;
    // 0x72404: mov r9, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(9),R::R0,0)?)?;
    // 0x72408: cmp r12, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(12),0)?)?;
    // 0x7240c: beq 0x72448 <entry+0xe5c>   @ imm = #0x34
    c.branch(C::Eq,0x72448,false)?;
    // 0x72410: ldrb r1, [r4, #0x89]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),137)?)?;
    // 0x72414: ldrb r3, [r4, #0x780]
    c.emit(transfer_byte(C::Al,true,r(3),r(4),1920)?)?;
    // 0x72418: cmp r1, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),1)?)?;
    // 0x7241c: bne 0x72440 <entry+0xe54>   @ imm = #0x1c
    c.branch(C::Ne,0x72440,false)?;
    // 0x72420: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72424: beq 0x72440 <entry+0xe54>   @ imm = #0x14
    c.branch(C::Eq,0x72440,false)?;
    // 0x72428: cmp r3, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),0)?)?;
    // 0x7242c: bne 0x72440 <entry+0xe54>   @ imm = #0xc
    c.branch(C::Ne,0x72440,false)?;
    // 0x72430: ldr r7, [r4]
    c.emit(transfer_word(C::Al,true,r(7),r(4),0)?)?;
    // 0x72434: mov r9, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(9),R::R0,0)?)?;
    // 0x72438: cmp r7, #4096
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(7),4096)?)?;
    // 0x7243c: bne 0x7257c <entry+0xf90>   @ imm = #0x138
    c.branch(C::Ne,0x7257c,false)?;
    // 0x72440: cmp r3, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),0)?)?;
    // 0x72444: beq 0x72568 <entry+0xf7c>   @ imm = #0x11c
    c.branch(C::Eq,0x72568,false)?;
    // 0x72448: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x7244c: ldr r1, [r4]
    c.emit(transfer_word(C::Al,true,r(1),r(4),0)?)?;
    // 0x72450: cmp r1, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),2)?)?;
    // 0x72454: beq 0x7246c <entry+0xe80>   @ imm = #0x10
    c.branch(C::Eq,0x7246c,false)?;
    // 0x72458: cmp r1, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),8192)?)?;
    // 0x7245c: bne 0x72200 <entry+0xc14>   @ imm = #-0x264
    c.branch(C::Ne,0x72200,false)?;
    // 0x72460: ldrb r1, [r4, #0x22]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),34)?)?;
    // 0x72464: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x72468: bne 0x72478 <entry+0xe8c>   @ imm = #0x8
    c.branch(C::Ne,0x72478,false)?;
    // 0x7246c: ldrb r1, [r4, #0x23]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),35)?)?;
    // 0x72470: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x72474: beq 0x72200 <entry+0xc14>   @ imm = #-0x27c
    c.branch(C::Eq,0x72200,false)?;
    // 0x72478: cmp r12, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(12),0)?)?;
    // 0x7247c: bne 0x72200 <entry+0xc14>   @ imm = #-0x284
    c.branch(C::Ne,0x72200,false)?;
    // 0x72480: ldrb r1, [r4, #0x780]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),1920)?)?;
    // 0x72484: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x72488: bne 0x72200 <entry+0xc14>   @ imm = #-0x290
    c.branch(C::Ne,0x72200,false)?;
    // 0x7248c: ldr r1, [r4, #0x78]
    c.emit(transfer_word(C::Al,true,r(1),r(4),120)?)?;
    // 0x72490: cmp r1, #4
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),4)?)?;
    // 0x72494: blo 0x72200 <entry+0xc14>   @ imm = #-0x29c
    c.branch(C::Cc,0x72200,false)?;
    // 0x72498: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x7249c: beq 0x72744 <entry+0x1158>  @ imm = #0x2a0
    c.branch(C::Eq,0x72744,false)?;
    // 0x724a0: ldrb r7, [r6, #0xb]
    c.emit(transfer_byte(C::Al,true,r(7),r(6),11)?)?;
    // 0x724a4: ldrb r2, [r6, #0x28c]
    c.emit(transfer_byte(C::Al,true,r(2),r(6),652)?)?;
    // 0x724a8: cmp r2, r7
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(7),Shift::Lsl,0,false)?)?;
    // 0x724ac: bls 0x7274c <entry+0x1160>  @ imm = #0x298
    c.branch(C::Ls,0x7274c,false)?;
    // 0x724b0: ldrb r1, [r6, #0xc]
    c.emit(transfer_byte(C::Al,true,r(1),r(6),12)?)?;
    // 0x724b4: ldr r3, [sp, #0x18]
    c.emit(transfer_word(C::Al,true,r(3),R::SP,24)?)?;
    // 0x724b8: cmp r2, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(1),Shift::Lsl,0,false)?)?;
    // 0x724bc: ldrbhs r2, [r4, #0x2c]
    c.emit(transfer_byte(C::Cs,true,r(2),r(4),44)?)?;
    // 0x724c0: subhs r2, r1, r2
    c.emit(data_register(C::Cs,Op::Sub,r(2),r(1),r(2),Shift::Lsl,0,false)?)?;
    // 0x724c4: b 0x72758 <entry+0x116c>  @ imm = #0x28c
    c.branch(C::Al,0x72758,false)?;
    // 0x724c8: ldrb r0, [r4, #0x2d]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),45)?)?;
    // 0x724cc: ldr r1, [sp, #0x10]
    c.emit(transfer_word(C::Al,true,r(1),R::SP,16)?)?;
    // 0x724d0: ldr r10, [sp, #0x24]
    c.emit(transfer_word(C::Al,true,r(10),R::SP,36)?)?;
    // 0x724d4: mov r9, lr
    c.emit(mov_register(C::Al,r(9),R::LR))?;
    // 0x724d8: sub r8, r1, r0, lsr #1
    c.emit(data_register(C::Al,Op::Sub,r(8),r(1),r(0),Shift::Lsr,1,false)?)?;
    // 0x724dc: and r0, lr, #255
    c.emit(data_immediate(C::Al,Op::And,r(0),R::LR,255)?)?;
    // 0x724e0: cmp r8, r0
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(8),r(0),Shift::Lsl,0,false)?)?;
    // 0x724e4: movlt r8, r0
    c.emit(mov_register(C::Lt,r(8),r(0)))?;
    // 0x724e8: b 0x725d0 <entry+0xfe4>   @ imm = #0xe0
    c.branch(C::Al,0x725d0,false)?;
    // 0x724ec: ldr r10, [sp, #0x24]
    c.emit(transfer_word(C::Al,true,r(10),R::SP,36)?)?;
    // 0x724f0: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x724f4: mov r9, lr
    c.emit(mov_register(C::Al,r(9),R::LR))?;
    // 0x724f8: str r0, [sp, #0x8]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,8)?)?;
    // 0x724fc: and r0, r10, #255
    c.emit(data_immediate(C::Al,Op::And,r(0),r(10),255)?)?;
    // 0x72500: and r2, r9, #255
    c.emit(data_immediate(C::Al,Op::And,r(2),r(9),255)?)?;
    // 0x72504: cmp r2, r0
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(0),Shift::Lsl,0,false)?)?;
    // 0x72508: bhi 0x725b0 <entry+0xfc4>   @ imm = #0xa0
    c.branch(C::Hi,0x725b0,false)?;
    // 0x7250c: ldr r12, [r6, #0x80]
    c.emit(transfer_word(C::Al,true,r(12),r(6),128)?)?;
    // 0x72510: add r1, r6, r2, lsl #2
    c.emit(data_register(C::Al,Op::Add,r(1),r(6),r(2),Shift::Lsl,2,false)?)?;
    // 0x72514: sub r3, r2, #1
    c.emit(data_immediate(C::Al,Op::Sub,r(3),r(2),1)?)?;
    // 0x72518: mvn r5, #-2147483648
    c.emit(data_immediate(C::Al,Op::Mvn,r(5),R::R0,0x8000_0000)?)?;
    // 0x7251c: add r1, r1, #140
    c.emit(data_immediate(C::Al,Op::Add,r(1),r(1),140)?)?;
    // 0x72520: ldr r2, [r1]
    c.emit(transfer_word(C::Al,true,r(2),r(1),0)?)?;
    // 0x72524: mov r7, r5
    c.emit(mov_register(C::Al,r(7),r(5)))?;
    // 0x72528: add r3, r3, #1
    c.emit(data_immediate(C::Al,Op::Add,r(3),r(3),1)?)?;
    // 0x7252c: subs r2, r2, r12
    c.emit(data_register(C::Al,Op::Sub,r(2),r(2),r(12),Shift::Lsl,0,true)?)?;
    // 0x72530: mov r6, r2
    c.emit(mov_register(C::Al,r(6),r(2)))?;
    // 0x72534: rsbmi r6, r2, #0
    c.emit(data_immediate(C::Mi,Op::Rsb,r(6),r(2),0)?)?;
    // 0x72538: cmp r5, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),0)?)?;
    // 0x7253c: rsbmi r7, r5, #0
    c.emit(data_immediate(C::Mi,Op::Rsb,r(7),r(5),0)?)?;
    // 0x72540: cmp r6, r7
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(6),r(7),Shift::Lsl,0,false)?)?;
    // 0x72544: movlt r8, r3
    c.emit(mov_register(C::Lt,r(8),r(3)))?;
    // 0x72548: cmp r2, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),0)?)?;
    // 0x7254c: blt 0x725a4 <entry+0xfb8>   @ imm = #0x50
    c.branch(C::Lt,0x725a4,false)?;
    // 0x72550: cmp r6, r7
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(6),r(7),Shift::Lsl,0,false)?)?;
    // 0x72554: add r1, r1, #4
    c.emit(data_immediate(C::Al,Op::Add,r(1),r(1),4)?)?;
    // 0x72558: movlt r5, r6
    c.emit(mov_register(C::Lt,r(5),r(6)))?;
    // 0x7255c: cmp r3, r0
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(3),r(0),Shift::Lsl,0,false)?)?;
    // 0x72560: blo 0x72520 <entry+0xf34>   @ imm = #-0x48
    c.branch(C::Cc,0x72520,false)?;
    // 0x72564: b 0x725b0 <entry+0xfc4>   @ imm = #0x44
    c.branch(C::Al,0x725b0,false)?;
    // 0x72568: mov r9, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(9),R::R0,0)?)?;
    // 0x7256c: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72570: beq 0x7271c <entry+0x1130>  @ imm = #0x1a4
    c.branch(C::Eq,0x7271c,false)?;
    // 0x72574: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x72578: bne 0x72448 <entry+0xe5c>   @ imm = #-0x138
    c.branch(C::Ne,0x72448,false)?;
    // 0x7257c: ldrb r7, [r6, #0xb]
    c.emit(transfer_byte(C::Al,true,r(7),r(6),11)?)?;
    // 0x72580: ldrb r3, [r6, #0x28c]
    c.emit(transfer_byte(C::Al,true,r(3),r(6),652)?)?;
    // 0x72584: cmp r3, r7
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(3),r(7),Shift::Lsl,0,false)?)?;
    // 0x72588: bls 0x726d4 <entry+0x10e8>  @ imm = #0x144
    c.branch(C::Ls,0x726d4,false)?;
    // 0x7258c: ldrb r1, [r6, #0xc]
    c.emit(transfer_byte(C::Al,true,r(1),r(6),12)?)?;
    // 0x72590: ldr r2, [sp, #0x18]
    c.emit(transfer_word(C::Al,true,r(2),R::SP,24)?)?;
    // 0x72594: cmp r3, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(3),r(1),Shift::Lsl,0,false)?)?;
    // 0x72598: ldrbhs r3, [r4, #0x2c]
    c.emit(transfer_byte(C::Cs,true,r(3),r(4),44)?)?;
    // 0x7259c: subhs r3, r1, r3
    c.emit(data_register(C::Cs,Op::Sub,r(3),r(1),r(3),Shift::Lsl,0,false)?)?;
    // 0x725a0: b 0x726e0 <entry+0x10f4>  @ imm = #0x138
    c.branch(C::Al,0x726e0,false)?;
    // 0x725a4: ldrb r0, [r4, #0x1f]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),31)?)?;
    // 0x725a8: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x725ac: subne r8, r8, #1
    c.emit(data_immediate(C::Ne,Op::Sub,r(8),r(8),1)?)?;
    // 0x725b0: ldr r0, [sp, #0x18]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,24)?)?;
    // 0x725b4: ldr r3, [sp, #0xc]
    c.emit(transfer_word(C::Al,true,r(3),R::SP,12)?)?;
    // 0x725b8: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x725bc: ldr r0, [sp, #0x24]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,36)?)?;
    // 0x725c0: movne r9, lr
    c.emit(mov_register(C::Ne,r(9),R::LR))?;
    // 0x725c4: movne r10, r0
    c.emit(mov_register(C::Ne,r(10),r(0)))?;
    // 0x725c8: ldr r0, [sp, #0x8]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,8)?)?;
    // 0x725cc: subne r8, r8, r0
    c.emit(data_register(C::Ne,Op::Sub,r(8),r(8),r(0),Shift::Lsl,0,false)?)?;
    // 0x725d0: ldrb r0, [r4, #0x8a]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),138)?)?;
    // 0x725d4: ldr r7, [sp, #0x1c]
    c.emit(transfer_word(C::Al,true,r(7),R::SP,28)?)?;
    // 0x725d8: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x725dc: beq 0x726c8 <entry+0x10dc>  @ imm = #0xe4
    c.branch(C::Eq,0x726c8,false)?;
    // 0x725e0: ldr r0, [r4, #0x2c0]
    c.emit(transfer_word(C::Al,true,r(0),r(4),704)?)?;
    // 0x725e4: ldr r1, [r4, #0x2c4]
    c.emit(transfer_word(C::Al,true,r(1),r(4),708)?)?;
    // 0x725e8: subs r0, r0, #1
    c.emit(data_immediate_with_flags(C::Al,Op::Sub,r(0),r(0),1,true)?)?;
    // 0x725ec: sbcs r0, r1, #0
    c.emit(data_immediate_with_flags(C::Al,Op::Sbc,r(0),r(1),0,true)?)?;
    // 0x725f0: blt 0x726c8 <entry+0x10dc>  @ imm = #0xd0
    c.branch(C::Lt,0x726c8,false)?;
    // 0x725f4: ldrb r0, [r4, #0x89]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),137)?)?;
    // 0x725f8: ldr r6, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(6),r(11),-32)?)?;
    // 0x725fc: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72600: bne 0x726cc <entry+0x10e0>  @ imm = #0xc4
    c.branch(C::Ne,0x726cc,false)?;
    // 0x72604: ldr r0, [r4, #0xc]
    c.emit(transfer_word(C::Al,true,r(0),r(4),12)?)?;
    // 0x72608: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x7260c: bne 0x726cc <entry+0x10e0>  @ imm = #0xb8
    c.branch(C::Ne,0x726cc,false)?;
    // 0x72610: cmp r3, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),0)?)?;
    // 0x72614: bge 0x7263c <entry+0x1050>  @ imm = #0x20
    c.branch(C::Ge,0x7263c,false)?;
    // 0x72618: ldr r2, [sp, #0x14]
    c.emit(transfer_word(C::Al,true,r(2),R::SP,20)?)?;
    // 0x7261c: and r1, r8, #255
    c.emit(data_immediate(C::Al,Op::And,r(1),r(8),255)?)?;
    // 0x72620: add r0, r1, #2
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(1),2)?)?;
    // 0x72624: cmp r0, r2
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(0),r(2),Shift::Lsl,0,false)?)?;
    // 0x72628: blo 0x72724 <entry+0x1138>  @ imm = #0xf4
    c.branch(C::Cc,0x72724,false)?;
    // 0x7262c: ldr r3, [sp, #0x14]
    c.emit(transfer_word(C::Al,true,r(3),R::SP,20)?)?;
    // 0x72630: sub r2, r1, #2
    c.emit(data_immediate(C::Al,Op::Sub,r(2),r(1),2)?)?;
    // 0x72634: cmp r2, r3
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(3),Shift::Lsl,0,false)?)?;
    // 0x72638: bgt 0x72724 <entry+0x1138>  @ imm = #0xe4
    c.branch(C::Gt,0x72724,false)?;
    // 0x7263c: ldrb r0, [r6, #0x2]
    c.emit(transfer_byte(C::Al,true,r(0),r(6),2)?)?;
    // 0x72640: and r5, r8, #255
    c.emit(data_immediate(C::Al,Op::And,r(5),r(8),255)?)?;
    // 0x72644: cmp r5, r0
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(0),Shift::Lsl,0,false)?)?;
    // 0x72648: bne 0x72850 <entry+0x1264>  @ imm = #0x200
    c.branch(C::Ne,0x72850,false)?;
    // 0x7264c: ldr r0, [r4, #0x2dc]
    c.emit(transfer_word(C::Al,true,r(0),r(4),732)?)?;
    // 0x72650: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72654: beq 0x72850 <entry+0x1264>  @ imm = #0x1f4
    c.branch(C::Eq,0x72850,false)?;
    // 0x72658: ldr r0, [r7, #0x24]
    c.emit(transfer_word(C::Al,true,r(0),r(7),36)?)?;
    // 0x7265c: ldr r1, [r4, #0x2e4]
    c.emit(transfer_word(C::Al,true,r(1),r(4),740)?)?;
    // 0x72660: mov r3, #100
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,100)?)?;
    // 0x72664: subs r2, r1, r0
    c.emit(data_register(C::Al,Op::Sub,r(2),r(1),r(0),Shift::Lsl,0,true)?)?;
    // 0x72668: rsbmi r2, r2, #0
    c.emit(data_immediate(C::Mi,Op::Rsb,r(2),r(2),0)?)?;
    // 0x7266c: mul r0, r2, r3
    c.emit(multiply_register(C::Al,r(0),r(2),r(3))?)?;
    // 0x72670: blx 0x83c4c <entry+0x12660> @ imm = #0x115d4
    c.emit(branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x72674: cmp r0, #19
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),19)?)?;
    // 0x72678: bhi 0x72850 <entry+0x1264>  @ imm = #0x1d0
    c.branch(C::Hi,0x72850,false)?;
    // 0x7267c: ldrb r2, [r4, #0x25]
    c.emit(transfer_byte(C::Al,true,r(2),r(4),37)?)?;
    // 0x72680: ldrsh r0, [r7, #42]
    c.emit(load_signed_halfword(C::Al,r(0),r(7),42)?)?;
    // 0x72684: mvn r1, #14
    c.emit(data_immediate(C::Al,Op::Mvn,r(1),R::R0,14)?)?;
    // 0x72688: cmp r2, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),2)?)?;
    // 0x7268c: bhi 0x72794 <entry+0x11a8>  @ imm = #0x100
    c.branch(C::Hi,0x72794,false)?;
    // 0x72690: cmp r0, #11
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),11)?)?;
    // 0x72694: bge 0x7279c <entry+0x11b0>  @ imm = #0x100
    c.branch(C::Ge,0x7279c,false)?;
    // 0x72698: add r1, r1, #7
    c.emit(data_immediate(C::Al,Op::Add,r(1),r(1),7)?)?;
    // 0x7269c: cmp r0, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(0),r(1),Shift::Lsl,0,false)?)?;
    // 0x726a0: bge 0x72850 <entry+0x1264>  @ imm = #0x1a8
    c.branch(C::Ge,0x72850,false)?;
    // 0x726a4: ldrh r0, [r7, #48]
    c.emit(load_halfword(C::Al,r(0),r(7),48)?)?;
    // 0x726a8: mov r2, #255
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,255)?)?;
    // 0x726ac: orr r2, r2, #65280
    c.emit(data_immediate(C::Al,Op::Orr,r(2),r(2),65280)?)?;
    // 0x726b0: add r0, r0, #1
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(0),1)?)?;
    // 0x726b4: strh r0, [r7, #48]
    c.emit(store_halfword(C::Al,r(0),r(7),48)?)?;
    // 0x726b8: and r0, r0, r2
    c.emit(data_register(C::Al,Op::And,r(0),r(0),r(2),Shift::Lsl,0,false)?)?;
    // 0x726bc: cmp r0, #3
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),3)?)?;
    // 0x726c0: bhs 0x7281c <entry+0x1230>  @ imm = #0x154
    c.branch(C::Cs,0x7281c,false)?;
    // 0x726c4: b 0x72858 <entry+0x126c>  @ imm = #0x18c
    c.branch(C::Al,0x72858,false)?;
    // 0x726c8: ldr r6, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(6),r(11),-32)?)?;
    // 0x726cc: mov r0, r8
    c.emit(mov_register(C::Al,r(0),r(8)))?;
    // 0x726d0: b 0x72004 <entry+0xa18>   @ imm = #-0x6d4
    c.branch(C::Al,0x72004,false)?;
    // 0x726d4: ldrb r1, [r4, #0x2d]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),45)?)?;
    // 0x726d8: ldr r2, [sp, #0x18]
    c.emit(transfer_word(C::Al,true,r(2),R::SP,24)?)?;
    // 0x726dc: add r3, r1, r7
    c.emit(data_register(C::Al,Op::Add,r(3),r(1),r(7),Shift::Lsl,0,false)?)?;
    // 0x726e0: cmp r3, r7
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(3),r(7),Shift::Lsl,0,false)?)?;
    // 0x726e4: blt 0x726f8 <entry+0x110c>  @ imm = #0xc
    c.branch(C::Lt,0x726f8,false)?;
    // 0x726e8: ldrb r1, [r6, #0xc]
    c.emit(transfer_byte(C::Al,true,r(1),r(6),12)?)?;
    // 0x726ec: mov r7, r3
    c.emit(mov_register(C::Al,r(7),r(3)))?;
    // 0x726f0: cmp r3, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(3),r(1),Shift::Lsl,0,false)?)?;
    // 0x726f4: movgt r7, r1
    c.emit(mov_register(C::Gt,r(7),r(1)))?;
    // 0x726f8: add r1, r6, r7, lsl #2
    c.emit(data_register(C::Al,Op::Add,r(1),r(6),r(7),Shift::Lsl,2,false)?)?;
    // 0x726fc: mov r9, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(9),R::R0,1)?)?;
    // 0x72700: ldr r1, [r1, #0x8c]
    c.emit(transfer_word(C::Al,true,r(1),r(1),140)?)?;
    // 0x72704: sub r1, r1, r2
    c.emit(data_register(C::Al,Op::Sub,r(1),r(1),r(2),Shift::Lsl,0,false)?)?;
    // 0x72708: add r3, r1, r0
    c.emit(data_register(C::Al,Op::Add,r(3),r(1),r(0),Shift::Lsl,0,false)?)?;
    // 0x7270c: cmp r3, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),1)?)?;
    // 0x72710: movge r0, r3
    c.emit(mov_register(C::Ge,r(0),r(3)))?;
    // 0x72714: str r0, [r6, #0x80]
    c.emit(transfer_word(C::Al,false,r(0),r(6),128)?)?;
    // 0x72718: b 0x7244c <entry+0xe60>   @ imm = #-0x2d4
    c.branch(C::Al,0x7244c,false)?;
    // 0x7271c: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x72720: b 0x72448 <entry+0xe5c>   @ imm = #-0x2e0
    c.branch(C::Al,0x72448,false)?;
    // 0x72724: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x72728: ldrb r2, [r6, #0x2]
    c.emit(transfer_byte(C::Al,true,r(2),r(6),2)?)?;
    // 0x7272c: strh r3, [r7, #48]
    c.emit(store_halfword(C::Al,r(3),r(7),48)?)?;
    // 0x72730: ldr r3, [sp, #0x14]
    c.emit(transfer_word(C::Al,true,r(3),R::SP,20)?)?;
    // 0x72734: add r2, r2, #2
    c.emit(data_immediate(C::Al,Op::Add,r(2),r(2),2)?)?;
    // 0x72738: cmp r2, r3
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(3),Shift::Lsl,0,false)?)?;
    // 0x7273c: addhs r0, r1, #255
    c.emit(data_immediate(C::Cs,Op::Add,r(0),r(1),255)?)?;
    // 0x72740: b 0x7285c <entry+0x1270>  @ imm = #0x114
    c.branch(C::Al,0x7285c,false)?;
    // 0x72744: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x72748: b 0x72200 <entry+0xc14>   @ imm = #-0x550
    c.branch(C::Al,0x72200,false)?;
    // 0x7274c: ldrb r1, [r4, #0x2d]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),45)?)?;
    // 0x72750: ldr r3, [sp, #0x18]
    c.emit(transfer_word(C::Al,true,r(3),R::SP,24)?)?;
    // 0x72754: add r2, r1, r7
    c.emit(data_register(C::Al,Op::Add,r(2),r(1),r(7),Shift::Lsl,0,false)?)?;
    // 0x72758: cmp r2, r7
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(7),Shift::Lsl,0,false)?)?;
    // 0x7275c: blt 0x72770 <entry+0x1184>  @ imm = #0xc
    c.branch(C::Lt,0x72770,false)?;
    // 0x72760: ldrb r1, [r6, #0xc]
    c.emit(transfer_byte(C::Al,true,r(1),r(6),12)?)?;
    // 0x72764: mov r7, r2
    c.emit(mov_register(C::Al,r(7),r(2)))?;
    // 0x72768: cmp r2, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(1),Shift::Lsl,0,false)?)?;
    // 0x7276c: movgt r7, r1
    c.emit(mov_register(C::Gt,r(7),r(1)))?;
    // 0x72770: add r1, r6, r7, lsl #2
    c.emit(data_register(C::Al,Op::Add,r(1),r(6),r(7),Shift::Lsl,2,false)?)?;
    // 0x72774: mov r9, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(9),R::R0,1)?)?;
    // 0x72778: ldr r1, [r1, #0x8c]
    c.emit(transfer_word(C::Al,true,r(1),r(1),140)?)?;
    // 0x7277c: sub r1, r1, r3
    c.emit(data_register(C::Al,Op::Sub,r(1),r(1),r(3),Shift::Lsl,0,false)?)?;
    // 0x72780: add r3, r1, r0
    c.emit(data_register(C::Al,Op::Add,r(3),r(1),r(0),Shift::Lsl,0,false)?)?;
    // 0x72784: cmp r3, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),1)?)?;
    // 0x72788: movge r0, r3
    c.emit(mov_register(C::Ge,r(0),r(3)))?;
    // 0x7278c: str r0, [r6, #0x80]
    c.emit(transfer_word(C::Al,false,r(0),r(6),128)?)?;
    // 0x72790: b 0x72200 <entry+0xc14>   @ imm = #-0x598
    c.branch(C::Al,0x72200,false)?;
    // 0x72794: cmp r0, #11
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),11)?)?;
    // 0x72798: blt 0x727e8 <entry+0x11fc>  @ imm = #0x48
    c.branch(C::Lt,0x727e8,false)?;
    // 0x7279c: ldrh r1, [r7, #48]
    c.emit(load_halfword(C::Al,r(1),r(7),48)?)?;
    // 0x727a0: mov r2, #255
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,255)?)?;
    // 0x727a4: orr r2, r2, #65280
    c.emit(data_immediate(C::Al,Op::Orr,r(2),r(2),65280)?)?;
    // 0x727a8: add r1, r1, #1
    c.emit(data_immediate(C::Al,Op::Add,r(1),r(1),1)?)?;
    // 0x727ac: strh r1, [r7, #48]
    c.emit(store_halfword(C::Al,r(1),r(7),48)?)?;
    // 0x727b0: and r1, r1, r2
    c.emit(data_register(C::Al,Op::And,r(1),r(1),r(2),Shift::Lsl,0,false)?)?;
    // 0x727b4: cmp r1, #3
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),3)?)?;
    // 0x727b8: blo 0x72858 <entry+0x126c>  @ imm = #0x98
    c.branch(C::Cc,0x72858,false)?;
    // 0x727bc: cmp r0, #16
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),16)?)?;
    // 0x727c0: ldrshge r0, [r7, #44]
    c.emit(load_signed_halfword(C::Ge,r(0),r(7),44)?)?;
    // 0x727c4: cmpge r0, #16
    c.emit(data_immediate(C::Ge,Op::Cmp,R::R0,r(0),16)?)?;
    // 0x727c8: blt 0x72858 <entry+0x126c>  @ imm = #0x88
    c.branch(C::Lt,0x72858,false)?;
    // 0x727cc: ldrsh r0, [r7, #46]
    c.emit(load_signed_halfword(C::Al,r(0),r(7),46)?)?;
    // 0x727d0: cmp r0, #16
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),16)?)?;
    // 0x727d4: mov r0, r8
    c.emit(mov_register(C::Al,r(0),r(8)))?;
    // 0x727d8: movge r0, #0
    c.emit(data_immediate(C::Ge,Op::Mov,r(0),R::R0,0)?)?;
    // 0x727dc: strhge r0, [r7, #48]
    c.emit(store_halfword(C::Ge,r(0),r(7),48)?)?;
    // 0x727e0: addge r0, r5, #255
    c.emit(data_immediate(C::Ge,Op::Add,r(0),r(5),255)?)?;
    // 0x727e4: b 0x7285c <entry+0x1270>  @ imm = #0x70
    c.branch(C::Al,0x7285c,false)?;
    // 0x727e8: add r2, r1, #5
    c.emit(data_immediate(C::Al,Op::Add,r(2),r(1),5)?)?;
    // 0x727ec: cmp r0, r2
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(0),r(2),Shift::Lsl,0,false)?)?;
    // 0x727f0: bge 0x72850 <entry+0x1264>  @ imm = #0x58
    c.branch(C::Ge,0x72850,false)?;
    // 0x727f4: ldrh r2, [r7, #48]
    c.emit(load_halfword(C::Al,r(2),r(7),48)?)?;
    // 0x727f8: mov r3, #255
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,255)?)?;
    // 0x727fc: orr r3, r3, #65280
    c.emit(data_immediate(C::Al,Op::Orr,r(3),r(3),65280)?)?;
    // 0x72800: add r2, r2, #1
    c.emit(data_immediate(C::Al,Op::Add,r(2),r(2),1)?)?;
    // 0x72804: strh r2, [r7, #48]
    c.emit(store_halfword(C::Al,r(2),r(7),48)?)?;
    // 0x72808: and r2, r2, r3
    c.emit(data_register(C::Al,Op::And,r(2),r(2),r(3),Shift::Lsl,0,false)?)?;
    // 0x7280c: cmp r2, #3
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),3)?)?;
    // 0x72810: blo 0x72858 <entry+0x126c>  @ imm = #0x40
    c.branch(C::Cc,0x72858,false)?;
    // 0x72814: cmp r0, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(0),r(1),Shift::Lsl,0,false)?)?;
    // 0x72818: bge 0x728b8 <entry+0x12cc>  @ imm = #0x98
    c.branch(C::Ge,0x728b8,false)?;
    // 0x7281c: ldrsh r0, [r7, #44]
    c.emit(load_signed_halfword(C::Al,r(0),r(7),44)?)?;
    // 0x72820: mov r2, r7
    c.emit(mov_register(C::Al,r(2),r(7)))?;
    // 0x72824: cmp r0, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(0),r(1),Shift::Lsl,0,false)?)?;
    // 0x72828: bge 0x728a8 <entry+0x12bc>  @ imm = #0x78
    c.branch(C::Ge,0x728a8,false)?;
    // 0x7282c: mov r7, r2
    c.emit(mov_register(C::Al,r(7),r(2)))?;
    // 0x72830: ldr r6, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(6),r(11),-32)?)?;
    // 0x72834: ldrsh r0, [r7, #46]
    c.emit(load_signed_halfword(C::Al,r(0),r(7),46)?)?;
    // 0x72838: cmp r0, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(0),r(1),Shift::Lsl,0,false)?)?;
    // 0x7283c: mov r0, r8
    c.emit(mov_register(C::Al,r(0),r(8)))?;
    // 0x72840: movlt r0, #0
    c.emit(data_immediate(C::Lt,Op::Mov,r(0),R::R0,0)?)?;
    // 0x72844: strhlt r0, [r7, #48]
    c.emit(store_halfword(C::Lt,r(0),r(7),48)?)?;
    // 0x72848: addlt r0, r5, #1
    c.emit(data_immediate(C::Lt,Op::Add,r(0),r(5),1)?)?;
    // 0x7284c: b 0x7285c <entry+0x1270>  @ imm = #0x8
    c.branch(C::Al,0x7285c,false)?;
    // 0x72850: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x72854: strh r0, [r7, #48]
    c.emit(store_halfword(C::Al,r(0),r(7),48)?)?;
    // 0x72858: mov r0, r8
    c.emit(mov_register(C::Al,r(0),r(8)))?;
    // 0x7285c: ldrsh r1, [r7, #42]
    c.emit(load_signed_halfword(C::Al,r(1),r(7),42)?)?;
    // 0x72860: cmp r1, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),1)?)?;
    // 0x72864: blt 0x72878 <entry+0x128c>  @ imm = #0xc
    c.branch(C::Lt,0x72878,false)?;
    // 0x72868: ldrsh r2, [r7, #44]
    c.emit(load_signed_halfword(C::Al,r(2),r(7),44)?)?;
    // 0x7286c: cmp r2, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),0)?)?;
    // 0x72870: bge 0x72880 <entry+0x1294>  @ imm = #0x8
    c.branch(C::Ge,0x72880,false)?;
    // 0x72874: b 0x72894 <entry+0x12a8>  @ imm = #0x18
    c.branch(C::Al,0x72894,false)?;
    // 0x72878: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x7287c: blt 0x72888 <entry+0x129c>  @ imm = #0x4
    c.branch(C::Lt,0x72888,false)?;
    // 0x72880: ldrh r2, [r7, #44]
    c.emit(load_halfword(C::Al,r(2),r(7),44)?)?;
    // 0x72884: b 0x7289c <entry+0x12b0>  @ imm = #0x10
    c.branch(C::Al,0x7289c,false)?;
    // 0x72888: ldrsh r2, [r7, #44]
    c.emit(load_signed_halfword(C::Al,r(2),r(7),44)?)?;
    // 0x7288c: cmp r2, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),1)?)?;
    // 0x72890: blt 0x7289c <entry+0x12b0>  @ imm = #0x4
    c.branch(C::Lt,0x7289c,false)?;
    // 0x72894: mov r3, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,0)?)?;
    // 0x72898: strh r3, [r7, #48]
    c.emit(store_halfword(C::Al,r(3),r(7),48)?)?;
    // 0x7289c: strh r2, [r7, #46]
    c.emit(store_halfword(C::Al,r(2),r(7),46)?)?;
    // 0x728a0: strh r1, [r7, #44]
    c.emit(store_halfword(C::Al,r(1),r(7),44)?)?;
    // 0x728a4: b 0x72004 <entry+0xa18>   @ imm = #-0x8a8
    c.branch(C::Al,0x72004,false)?;
    // 0x728a8: ldr r6, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(6),r(11),-32)?)?;
    // 0x728ac: mov r0, r8
    c.emit(mov_register(C::Al,r(0),r(8)))?;
    // 0x728b0: mov r7, r2
    c.emit(mov_register(C::Al,r(7),r(2)))?;
    // 0x728b4: b 0x7285c <entry+0x1270>  @ imm = #-0x60
    c.branch(C::Al,0x7285c,false)?;
    // 0x728b8: ldr r6, [r11, #-0x20]
    c.emit(transfer_word(C::Al,true,r(6),r(11),-32)?)?;
    // 0x728bc: b 0x72858 <entry+0x126c>  @ imm = #-0x6c
    c.branch(C::Al,0x72858,false)?;

    c.check(0x728c0)?;
    for word in [RANGE_DIAGNOSTIC,TRACE_FOOTER,DIVIDE_BY_THREE_RECIPROCAL] {
        c.emit(word)?;
    }
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();finish_controller_limits(&mut c)?;Ok(c.bytes)}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn adaptive_tail_has_named_literals(){
        let bytes=fragment().unwrap();assert_eq!(bytes.len(),END-START);
        for (i,word) in [RANGE_DIAGNOSTIC,TRACE_FOOTER,DIVIDE_BY_THREE_RECIPROCAL].into_iter().enumerate(){
            let off=bytes.len()-12+i*4;
            assert_eq!(&bytes[off..off+4],&word.to_le_bytes());
        }
    }
}
