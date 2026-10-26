// SPDX-License-Identifier: GPL-3.0-only
//! Completion of the Venus controller-run state machine beginning at 0x72d14.
//! This section dispatches allocation & codec worker callbacks, writes
//! controller/buffer metadata, handles status and failure paths, and returns
//! through the saved ARM32 frame. The 18 trailing words are typed device
//! data (masks or diagnostic address references), not OEM opcodes.
use arm32::{branch, data_immediate, data_register, mov_register,
    mov_shifted_register, transfer_doubleword, transfer_byte,
    transfer_byte_register, transfer_word, transfer_word_register,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op, Register as R,
    Shift};

pub const START:usize=0x73000;
pub const END:usize=0x7389c;
pub const FILE_BASE:usize=0x4000;
const CONTROLLER_STATUS_MASK:u32=0x00ffffe0;
const ALLOCATOR_DIAGNOSTIC:u32=0x000f32d4;
const WORKER_RESOURCE_MASK:u32=0x7fe00002;
const WORKER_START_DIAGNOSTIC:u32=0x000f353a;
const PLATFORM_LOG_FOOTER:u32=0x000e2788;
const WORKER_EXIT_DIAGNOSTIC:u32=0x000f354f;
const CODEC_RUNTIME_STATUS:u32=0x00221008;
const CODEC_QUEUE_DIAGNOSTIC:u32=0x000f3332;
const CODEC_BUFFER_DIAGNOSTIC:u32=0x000f337a;
const CODEC_CONTROL_DIAGNOSTIC:u32=0x000f33b5;
const CODEC_READY_DIAGNOSTIC:u32=0x000f3433;
const CODEC_RESET_DIAGNOSTIC:u32=0x000f3404;
const CODEC_DISABLED_DIAGNOSTIC:u32=0x000f33d5;
const CODEC_SOURCE_CONTEXT:u32=0x000ebf23;
const CODEC_BUSY_DIAGNOSTIC:u32=0x000f3461;
const CODEC_TIMEOUT_DIAGNOSTIC:u32=0x000f347c;
const CODEC_COMPLETE_DIAGNOSTIC:u32=0x000f34df;
const CODEC_WORKER_DIAGNOSTIC:u32=0x000f350d;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self, expected:usize)->Result<(),String>{
        if self.at==expected{Ok(())}else{Err(format!("controller-run tail location 0x{:x}, expected 0x{expected:x}",self.at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START||self.at>=END||self.at%4!=0{return Err(format!("controller-run tail output at 0x{:x}",self.at));}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,cond:C,target:usize,link:bool)->Result<(),String>{
        self.emit(branch(cond,self.at as u32,target as u32,link)?)
    }
    fn literal(&mut self,dst:R,target:usize)->Result<(),String>{
        self.emit(transfer_word(C::Al,true,dst,R::PC,target as i32-self.at as i32-8)?)
    }
}
fn r(x:u8)->R{R::new(x).expect("A32 register")}
fn finish_controller_run(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x73000: ldr r0, [r7, #0x68]
    c.emit(transfer_word(C::Al,true,r(0),r(7),104)?)?;
    // 0x73004: bl 0x205c <entry+0xfffffffffff8f790> @ imm = #-0x70fb0
    c.branch(C::Al,0x205c,true)?;
    // 0x73008: ldrb r0, [r4, #0x3ed]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1005)?)?;
    // 0x7300c: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73010: ldrne r0, [r7, #0xf8]
    c.emit(transfer_word(C::Ne,true,r(0),r(7),248)?)?;
    // 0x73014: cmpne r0, #0
    c.emit(data_immediate(C::Ne,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73018: beq 0x7306c <entry+0x7a0>   @ imm = #0x4c
    c.branch(C::Eq,0x7306c,false)?;
    // 0x7301c: ldr r5, [r7, #0x68]
    c.emit(transfer_word(C::Al,true,r(5),r(7),104)?)?;
    // 0x73020: mov r6, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,0)?)?;
    // 0x73024: add r0, r7, r6, lsl #2
    c.emit(data_register(C::Al,Op::Add,r(0),r(7),r(6),Shift::Lsl,2,false)?)?;
    // 0x73028: ldr r1, [r0, #0x78]
    c.emit(transfer_word(C::Al,true,r(1),r(0),120)?)?;
    // 0x7302c: ldr r2, [r0, #0xb8]
    c.emit(transfer_word(C::Al,true,r(2),r(0),184)?)?;
    // 0x73030: sub r1, r1, #4
    c.emit(data_immediate(C::Al,Op::Sub,r(1),r(1),4)?)?;
    // 0x73034: add r0, r5, r2
    c.emit(data_register(C::Al,Op::Add,r(0),r(5),r(2),Shift::Lsl,0,false)?)?;
    // 0x73038: lsr r3, r1, #8
    c.emit(mov_shifted_register(C::Al,r(3),r(1),Shift::Lsr,8)?)?;
    // 0x7303c: strb r1, [r0, #0x3]
    c.emit(transfer_byte(C::Al,false,r(1),r(0),3)?)?;
    // 0x73040: strb r3, [r0, #0x2]
    c.emit(transfer_byte(C::Al,false,r(3),r(0),2)?)?;
    // 0x73044: lsr r3, r1, #16
    c.emit(mov_shifted_register(C::Al,r(3),r(1),Shift::Lsr,16)?)?;
    // 0x73048: lsr r1, r1, #24
    c.emit(mov_shifted_register(C::Al,r(1),r(1),Shift::Lsr,24)?)?;
    // 0x7304c: strb r3, [r0, #0x1]
    c.emit(transfer_byte(C::Al,false,r(3),r(0),1)?)?;
    // 0x73050: strb r1, [r5, r2]
    c.emit(transfer_byte_register(C::Al,false,r(1),r(5),r(2),Shift::Lsl,0)?)?;
    // 0x73054: mov r1, #4
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,4)?)?;
    // 0x73058: bl 0x205c <entry+0xfffffffffff8f790> @ imm = #-0x71004
    c.branch(C::Al,0x205c,true)?;
    // 0x7305c: ldr r0, [r7, #0xf8]
    c.emit(transfer_word(C::Al,true,r(0),r(7),248)?)?;
    // 0x73060: add r6, r6, #1
    c.emit(data_immediate(C::Al,Op::Add,r(6),r(6),1)?)?;
    // 0x73064: cmp r6, r0
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(6),r(0),Shift::Lsl,0,false)?)?;
    // 0x73068: blo 0x73024 <entry+0x758>   @ imm = #-0x4c
    c.branch(C::Cc,0x73024,false)?;
    // 0x7306c: ldrb r0, [r4, #0x57b]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1403)?)?;
    // 0x73070: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73074: beq 0x732dc <entry+0xa10>   @ imm = #0x260
    c.branch(C::Eq,0x732dc,false)?;
    // 0x73078: ldr r0, [r4, #0x38]
    c.emit(transfer_word(C::Al,true,r(0),r(4),56)?)?;
    // 0x7307c: ldr r0, [r0, #0x8]
    c.emit(transfer_word(C::Al,true,r(0),r(0),8)?)?;
    // 0x73080: bl 0x35d44 <entry+0xfffffffffffc3478> @ imm = #-0x3d344
    c.branch(C::Al,0x35d44,true)?;
    // 0x73084: mov r10, r0
    c.emit(mov_register(C::Al,r(10),r(0)))?;
    // 0x73088: cmp r10, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(10),0)?)?;
    // 0x7308c: beq 0x731e4 <entry+0x918>   @ imm = #0x150
    c.branch(C::Eq,0x731e4,false)?;
    // 0x73090: ldr r9, [r10, #0x70]
    c.emit(transfer_word(C::Al,true,r(9),r(10),112)?)?;
    // 0x73094: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x73098: str r0, [r9]
    c.emit(transfer_word(C::Al,false,r(0),r(9),0)?)?;
    // 0x7309c: ldr r0, [r10, #0x58]
    c.emit(transfer_word(C::Al,true,r(0),r(10),88)?)?;
    // 0x730a0: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x730a4: beq 0x731fc <entry+0x930>   @ imm = #0x150
    c.branch(C::Eq,0x731fc,false)?;
    // 0x730a8: ldr r1, [r10, #0x54]
    c.emit(transfer_word(C::Al,true,r(1),r(10),84)?)?;
    // 0x730ac: ldr r2, [pc, #0x7a4]        @ 0xf8c <entry+0xf8c>
    c.literal(r(2),0x73858)?;
    // 0x730b0: add r0, r0, r1
    c.emit(data_register(C::Al,Op::Add,r(0),r(0),r(1),Shift::Lsl,0,false)?)?;
    // 0x730b4: ldr r1, [r4, #0x63c]
    c.emit(transfer_word(C::Al,true,r(1),r(4),1596)?)?;
    // 0x730b8: add r0, r0, #27
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(0),27)?)?;
    // 0x730bc: bic r0, r0, #3
    c.emit(data_immediate(C::Al,Op::Bic,r(0),r(0),3)?)?;
    // 0x730c0: cmp r1, r0
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(1),r(0),Shift::Lsl,0,false)?)?;
    // 0x730c4: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x730c8: mov r1, #276
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,276)?)?;
    // 0x730cc: movhs r0, #1
    c.emit(data_immediate(C::Cs,Op::Mov,r(0),R::R0,1)?)?;
    // 0x730d0: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x71928
    c.branch(C::Al,0x17b0,true)?;
    // 0x730d4: ldrb r0, [r4, #0x3f9]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1017)?)?;
    // 0x730d8: cmp r0, #16
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),16)?)?;
    // 0x730dc: beq 0x73140 <entry+0x874>   @ imm = #0x5c
    c.branch(C::Eq,0x73140,false)?;
    // 0x730e0: ldr r0, [r10, #0x70]
    c.emit(transfer_word(C::Al,true,r(0),r(10),112)?)?;
    // 0x730e4: ldr r1, [pc, #0x770]        @ 0xf90 <entry+0xf90>
    c.literal(r(1),0x7385c)?;
    // 0x730e8: str r1, [r9, #0xc]
    c.emit(transfer_word(C::Al,false,r(1),r(9),12)?)?;
    // 0x730ec: ldr r1, [r4, #0x5c8]
    c.emit(transfer_word(C::Al,true,r(1),r(4),1480)?)?;
    // 0x730f0: str r1, [r9, #0x4]
    c.emit(transfer_word(C::Al,false,r(1),r(9),4)?)?;
    // 0x730f4: ldr r1, [r4, #0x5cc]
    c.emit(transfer_word(C::Al,true,r(1),r(4),1484)?)?;
    // 0x730f8: str r1, [r9, #0x8]
    c.emit(transfer_word(C::Al,false,r(1),r(9),8)?)?;
    // 0x730fc: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x73100: str r1, [r9, #0x10]
    c.emit(transfer_word(C::Al,false,r(1),r(9),16)?)?;
    // 0x73104: ldr r1, [r4, #0x568]
    c.emit(transfer_word(C::Al,true,r(1),r(4),1384)?)?;
    // 0x73108: sub r1, r1, #20
    c.emit(data_immediate(C::Al,Op::Sub,r(1),r(1),20)?)?;
    // 0x7310c: str r1, [r9]
    c.emit(transfer_word(C::Al,false,r(1),r(9),0)?)?;
    // 0x73110: add r2, r0, r1
    c.emit(data_register(C::Al,Op::Add,r(2),r(0),r(1),Shift::Lsl,0,false)?)?;
    // 0x73114: ldr r3, [r4, #0x5b8]
    c.emit(transfer_word(C::Al,true,r(3),r(4),1464)?)?;
    // 0x73118: str r3, [r2, #0xc]
    c.emit(transfer_word(C::Al,false,r(3),r(2),12)?)?;
    // 0x7311c: ldr r3, [r4, #0x5b0]
    c.emit(transfer_word(C::Al,true,r(3),r(4),1456)?)?;
    // 0x73120: str r3, [r2, #0x4]
    c.emit(transfer_word(C::Al,false,r(3),r(2),4)?)?;
    // 0x73124: ldr r3, [r4, #0x5b4]
    c.emit(transfer_word(C::Al,true,r(3),r(4),1460)?)?;
    // 0x73128: str r3, [r2, #0x8]
    c.emit(transfer_word(C::Al,false,r(3),r(2),8)?)?;
    // 0x7312c: ldr r3, [r4, #0x6e8]
    c.emit(transfer_word(C::Al,true,r(3),r(4),1768)?)?;
    // 0x73130: str r3, [r2, #0x10]
    c.emit(transfer_word(C::Al,false,r(3),r(2),16)?)?;
    // 0x73134: add r3, r3, #20
    c.emit(data_immediate(C::Al,Op::Add,r(3),r(3),20)?)?;
    // 0x73138: add r9, r2, r3
    c.emit(data_register(C::Al,Op::Add,r(9),r(2),r(3),Shift::Lsl,0,false)?)?;
    // 0x7313c: str r3, [r0, r1]
    c.emit(transfer_word_register(C::Al,false,r(3),r(0),r(1),Shift::Lsl,0)?)?;
    // 0x73140: ldrb r0, [r4, #0x3fa]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1018)?)?;
    // 0x73144: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x73148: bne 0x73180 <entry+0x8b4>   @ imm = #0x30
    c.branch(C::Ne,0x73180,false)?;
    // 0x7314c: ldr r0, [r4, #0x5ac]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1452)?)?;
    // 0x73150: str r0, [r9, #0xc]
    c.emit(transfer_word(C::Al,false,r(0),r(9),12)?)?;
    // 0x73154: ldr r0, [r4, #0x5a4]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1444)?)?;
    // 0x73158: str r0, [r9, #0x4]
    c.emit(transfer_word(C::Al,false,r(0),r(9),4)?)?;
    // 0x7315c: ldr r0, [r4, #0x5a8]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1448)?)?;
    // 0x73160: str r0, [r9, #0x8]
    c.emit(transfer_word(C::Al,false,r(0),r(9),8)?)?;
    // 0x73164: ldr r0, [r10, #0x14]
    c.emit(transfer_word(C::Al,true,r(0),r(10),20)?)?;
    // 0x73168: str r0, [r9, #0x14]
    c.emit(transfer_word(C::Al,false,r(0),r(9),20)?)?;
    // 0x7316c: mov r0, #4
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,4)?)?;
    // 0x73170: str r0, [r9, #0x10]
    c.emit(transfer_word(C::Al,false,r(0),r(9),16)?)?;
    // 0x73174: mov r0, #24
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,24)?)?;
    // 0x73178: str r0, [r9]
    c.emit(transfer_word(C::Al,false,r(0),r(9),0)?)?;
    // 0x7317c: add r9, r9, #24
    c.emit(data_immediate(C::Al,Op::Add,r(9),r(9),24)?)?;
    // 0x73180: ldrb r0, [r4, #0x3fc]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1020)?)?;
    // 0x73184: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x73188: bne 0x73250 <entry+0x984>   @ imm = #0xc0
    c.branch(C::Ne,0x73250,false)?;
    // 0x7318c: ldr r0, [r4, #0x5dc]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1500)?)?;
    // 0x73190: str r0, [r9, #0xc]
    c.emit(transfer_word(C::Al,false,r(0),r(9),12)?)?;
    // 0x73194: ldr r0, [r4, #0x5d4]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1492)?)?;
    // 0x73198: str r0, [r9, #0x4]
    c.emit(transfer_word(C::Al,false,r(0),r(9),4)?)?;
    // 0x7319c: ldr r0, [r4, #0x5d8]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1496)?)?;
    // 0x731a0: str r0, [r9, #0x8]
    c.emit(transfer_word(C::Al,false,r(0),r(9),8)?)?;
    // 0x731a4: ldr r0, [r4, #0x284]
    c.emit(transfer_word(C::Al,true,r(0),r(4),644)?)?;
    // 0x731a8: cmp r0, #4096
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),4096)?)?;
    // 0x731ac: beq 0x731dc <entry+0x910>   @ imm = #0x28
    c.branch(C::Eq,0x731dc,false)?;
    // 0x731b0: cmp r0, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),8192)?)?;
    // 0x731b4: bne 0x73208 <entry+0x93c>   @ imm = #0x4c
    c.branch(C::Ne,0x73208,false)?;
    // 0x731b8: ldr r0, [r4, #0x298]
    c.emit(transfer_word(C::Al,true,r(0),r(4),664)?)?;
    // 0x731bc: cmp r0, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),2)?)?;
    // 0x731c0: blo 0x73208 <entry+0x93c>   @ imm = #0x40
    c.branch(C::Cc,0x73208,false)?;
    // 0x731c4: ldrb r0, [r4, #0x495]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1173)?)?;
    // 0x731c8: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x731cc: bne 0x731dc <entry+0x910>   @ imm = #0x8
    c.branch(C::Ne,0x731dc,false)?;
    // 0x731d0: ldrb r0, [r4, #0x47c]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1148)?)?;
    // 0x731d4: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x731d8: beq 0x73208 <entry+0x93c>   @ imm = #0x28
    c.branch(C::Eq,0x73208,false)?;
    // 0x731dc: ldrb r2, [r10, #0x1]
    c.emit(transfer_byte(C::Al,true,r(2),r(10),1)?)?;
    // 0x731e0: b 0x7320c <entry+0x940>   @ imm = #0x24
    c.branch(C::Al,0x7320c,false)?;
    // 0x731e4: ldr r2, [pc, #0x66c]        @ 0xf8c <entry+0xf8c>
    c.literal(r(2),0x73858)?;
    // 0x731e8: mov r1, #9
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,9)?)?;
    // 0x731ec: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x731f0: orr r1, r1, #256
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),256)?)?;
    // 0x731f4: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x71a4c
    c.branch(C::Al,0x17b0,true)?;
    // 0x731f8: b 0x732d8 <entry+0xa0c>   @ imm = #0xd8
    c.branch(C::Al,0x732d8,false)?;
    // 0x731fc: mov r0, r9
    c.emit(mov_register(C::Al,r(0),r(9)))?;
    // 0x73200: mov r1, #4
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,4)?)?;
    // 0x73204: b 0x732d4 <entry+0xa08>   @ imm = #0xc8
    c.branch(C::Al,0x732d4,false)?;
    // 0x73208: ldr r2, [r10, #0x108]
    c.emit(transfer_word(C::Al,true,r(2),r(10),264)?)?;
    // 0x7320c: ldr r1, [pc, #0x64c]        @ 0xf94 <entry+0xf94>
    c.literal(r(1),0x73860)?;
    // 0x73210: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x73214: str r2, [r9, #0x14]
    c.emit(transfer_word(C::Al,false,r(2),r(9),20)?)?;
    // 0x73218: str r0, [r9, #0x18]
    c.emit(transfer_word(C::Al,false,r(0),r(9),24)?)?;
    // 0x7321c: str r0, [r9, #0x1c]
    c.emit(transfer_word(C::Al,false,r(0),r(9),28)?)?;
    // 0x73220: str r0, [r9, #0x20]
    c.emit(transfer_word(C::Al,false,r(0),r(9),32)?)?;
    // 0x73224: str r0, [r9, #0x24]
    c.emit(transfer_word(C::Al,false,r(0),r(9),36)?)?;
    // 0x73228: mov r0, #2
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,2)?)?;
    // 0x7322c: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x445a0
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73230: ldr r1, [pc, #0x62c]        @ 0xf98 <entry+0xf98>
    c.literal(r(1),0x73864)?;
    // 0x73234: mov r0, #2
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,2)?)?;
    // 0x73238: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x445ac
    c.branch(C::Al,0x2ec94,true)?;
    // 0x7323c: mov r0, #20
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,20)?)?;
    // 0x73240: str r0, [r9, #0x10]
    c.emit(transfer_word(C::Al,false,r(0),r(9),16)?)?;
    // 0x73244: mov r0, #44
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,44)?)?;
    // 0x73248: str r0, [r9]
    c.emit(transfer_word(C::Al,false,r(0),r(9),0)?)?;
    // 0x7324c: add r9, r9, #44
    c.emit(data_immediate(C::Al,Op::Add,r(9),r(9),44)?)?;
    // 0x73250: ldrb r0, [r4, #0x3fb]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1019)?)?;
    // 0x73254: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x73258: bne 0x732b0 <entry+0x9e4>   @ imm = #0x50
    c.branch(C::Ne,0x732b0,false)?;
    // 0x7325c: ldr r0, [r4, #0x5c4]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1476)?)?;
    // 0x73260: ldr r1, [pc, #0x600]        @ 0xf9c <entry+0xf9c>
    c.literal(r(1),0x73868)?;
    // 0x73264: str r0, [r9, #0xc]
    c.emit(transfer_word(C::Al,false,r(0),r(9),12)?)?;
    // 0x73268: ldr r0, [r4, #0x5bc]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1468)?)?;
    // 0x7326c: str r0, [r9, #0x4]
    c.emit(transfer_word(C::Al,false,r(0),r(9),4)?)?;
    // 0x73270: ldr r0, [r4, #0x5c0]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1472)?)?;
    // 0x73274: str r0, [r9, #0x8]
    c.emit(transfer_word(C::Al,false,r(0),r(9),8)?)?;
    // 0x73278: mov r0, #8
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,8)?)?;
    // 0x7327c: str r0, [r9, #0x10]
    c.emit(transfer_word(C::Al,false,r(0),r(9),16)?)?;
    // 0x73280: mov r0, #24
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,24)?)?;
    // 0x73284: str r0, [r9]
    c.emit(transfer_word(C::Al,false,r(0),r(9),0)?)?;
    // 0x73288: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x7328c: ldr r3, [r10, #0x110]
    c.emit(transfer_word(C::Al,true,r(3),r(10),272)?)?;
    // 0x73290: ldr r2, [r10, #0x114]
    c.emit(transfer_word(C::Al,true,r(2),r(10),276)?)?;
    // 0x73294: str r2, [r9, #0x14]
    c.emit(transfer_word(C::Al,false,r(2),r(9),20)?)?;
    // 0x73298: str r3, [r9, #0x18]
    c.emit(transfer_word(C::Al,false,r(3),r(9),24)?)?;
    // 0x7329c: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44610
    c.branch(C::Al,0x2ec94,true)?;
    // 0x732a0: ldr r1, [pc, #0x5bc]        @ 0xf98 <entry+0xf98>
    c.literal(r(1),0x73864)?;
    // 0x732a4: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x732a8: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x4461c
    c.branch(C::Al,0x2ec94,true)?;
    // 0x732ac: add r9, r9, #24
    c.emit(data_immediate(C::Al,Op::Add,r(9),r(9),24)?)?;
    // 0x732b0: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x732b4: str r1, [r9, #0xc]
    c.emit(transfer_word(C::Al,false,r(1),r(9),12)?)?;
    // 0x732b8: ldr r0, [r4, #0x5cc]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1484)?)?;
    // 0x732bc: str r0, [r9, #0x8]
    c.emit(transfer_word(C::Al,false,r(0),r(9),8)?)?;
    // 0x732c0: ldr r0, [r4, #0x5c8]
    c.emit(transfer_word(C::Al,true,r(0),r(4),1480)?)?;
    // 0x732c4: str r0, [r9, #0x4]
    c.emit(transfer_word(C::Al,false,r(0),r(9),4)?)?;
    // 0x732c8: str r1, [r9, #0x10]
    c.emit(transfer_word(C::Al,false,r(1),r(9),16)?)?;
    // 0x732cc: ldr r1, [r4, #0x5e8]
    c.emit(transfer_word(C::Al,true,r(1),r(4),1512)?)?;
    // 0x732d0: ldr r0, [r10, #0x70]
    c.emit(transfer_word(C::Al,true,r(0),r(10),112)?)?;
    // 0x732d4: bl 0x205c <entry+0xfffffffffff8f790> @ imm = #-0x71280
    c.branch(C::Al,0x205c,true)?;
    // 0x732d8: ldr r10, [pc, #0x584]       @ 0xf98 <entry+0xf98>
    c.literal(r(10),0x73864)?;
    // 0x732dc: ldr r0, [r8, #0x4]
    c.emit(transfer_word(C::Al,true,r(0),r(8),4)?)?;
    // 0x732e0: ldrb r0, [r0, #0x15e]
    c.emit(transfer_byte(C::Al,true,r(0),r(0),350)?)?;
    // 0x732e4: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x732e8: ldreq r0, [r4, #0x34]
    c.emit(transfer_word(C::Eq,true,r(0),r(4),52)?)?;
    // 0x732ec: ldrne r0, [r4, #0x30]
    c.emit(transfer_word(C::Ne,true,r(0),r(4),48)?)?;
    // 0x732f0: ldr r0, [r0, #0x4]
    c.emit(transfer_word(C::Al,true,r(0),r(0),4)?)?;
    // 0x732f4: bl 0x292c <entry+0xfffffffffff90060> @ imm = #-0x709d0
    c.branch(C::Al,0x292c,true)?;
    // 0x732f8: ldrb r0, [r4, #0x41c]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1052)?)?;
    // 0x732fc: ldr r9, [pc, #0x568]        @ 0xfa0 <entry+0xfa0>
    c.literal(r(9),0x7386c)?;
    // 0x73300: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73304: beq 0x7335c <entry+0xa90>   @ imm = #0x50
    c.branch(C::Eq,0x7335c,false)?;
    // 0x73308: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x7330c: sub r0, r9, #1
    c.emit(data_immediate(C::Al,Op::Sub,r(0),r(9),1)?)?;
    // 0x73310: mov r2, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,0)?)?;
    // 0x73314: str r1, [sp, #0x84]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,132)?)?;
    // 0x73318: str r0, [sp, #0x7c]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,124)?)?;
    // 0x7331c: ldr r0, [r4, #0x14]
    c.emit(transfer_word(C::Al,true,r(0),r(4),20)?)?;
    // 0x73320: str r0, [sp, #0x80]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,128)?)?;
    // 0x73324: ldr r0, [r7, #0x44]
    c.emit(transfer_word(C::Al,true,r(0),r(7),68)?)?;
    // 0x73328: str r0, [sp, #0x94]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,148)?)?;
    // 0x7332c: mov r0, #80
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,80)?)?;
    // 0x73330: str r0, [sp, #0x78]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,120)?)?;
    // 0x73334: ldr r0, [r7, #0x4c]
    c.emit(transfer_word(C::Al,true,r(0),r(7),76)?)?;
    // 0x73338: str r0, [sp, #0x8c]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,140)?)?;
    // 0x7333c: ldr r0, [r7, #0x48]
    c.emit(transfer_word(C::Al,true,r(0),r(7),72)?)?;
    // 0x73340: str r0, [sp, #0x88]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,136)?)?;
    // 0x73344: ldr r0, [r7, #0x2c]
    c.emit(transfer_word(C::Al,true,r(0),r(7),44)?)?;
    // 0x73348: str r0, [sp, #0x90]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,144)?)?;
    // 0x7334c: str r1, [sp, #0xc4]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,196)?)?;
    // 0x73350: add r1, sp, #120
    c.emit(data_immediate(C::Al,Op::Add,r(1),R::SP,120)?)?;
    // 0x73354: ldr r0, [r4, #0x20]
    c.emit(transfer_word(C::Al,true,r(0),r(4),32)?)?;
    // 0x73358: bl 0x2a18c <entry+0xfffffffffffb78c0> @ imm = #-0x491d4
    c.branch(C::Al,0x2a18c,true)?;
    // 0x7335c: str r9, [sp, #0x30]
    c.emit(transfer_word(C::Al,false,r(9),R::SP,48)?)?;
    // 0x73360: ldr r0, [r7, #0xfc]
    c.emit(transfer_word(C::Al,true,r(0),r(7),252)?)?;
    // 0x73364: str r0, [sp, #0x40]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,64)?)?;
    // 0x73368: ldr r0, [r7, #0x68]
    c.emit(transfer_word(C::Al,true,r(0),r(7),104)?)?;
    // 0x7336c: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73370: ldrne r0, [r8, #0x4]
    c.emit(transfer_word(C::Ne,true,r(0),r(8),4)?)?;
    // 0x73374: ldrne r0, [r0, #0x3bc]
    c.emit(transfer_word(C::Ne,true,r(0),r(0),956)?)?;
    // 0x73378: moveq r0, #0
    c.emit(data_immediate(C::Eq,Op::Mov,r(0),R::R0,0)?)?;
    // 0x7337c: str r0, [sp, #0x58]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,88)?)?;
    // 0x73380: ldr r0, [r7, #0x58]
    c.emit(transfer_word(C::Al,true,r(0),r(7),88)?)?;
    // 0x73384: str r0, [sp, #0x5c]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,92)?)?;
    // 0x73388: ldr r1, [r7, #0x54]
    c.emit(transfer_word(C::Al,true,r(1),r(7),84)?)?;
    // 0x7338c: str r1, [sp, #0x54]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,84)?)?;
    // 0x73390: ldr r1, [r4, #0x14]
    c.emit(transfer_word(C::Al,true,r(1),r(4),20)?)?;
    // 0x73394: str r1, [sp, #0x34]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,52)?)?;
    // 0x73398: ldr r1, [r7, #0x6c]
    c.emit(transfer_word(C::Al,true,r(1),r(7),108)?)?;
    // 0x7339c: str r1, [sp, #0x6c]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,108)?)?;
    // 0x733a0: ldr r1, [r7, #0x70]
    c.emit(transfer_word(C::Al,true,r(1),r(7),112)?)?;
    // 0x733a4: str r1, [sp, #0x70]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,112)?)?;
    // 0x733a8: mov r1, #76
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,76)?)?;
    // 0x733ac: ldrd r2, r3, [r7, #56]
    c.emit(transfer_doubleword(C::Al,true,r(2),r(7),56)?)?;
    // 0x733b0: str r3, [sp, #0x38]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,56)?)?;
    // 0x733b4: str r2, [sp, #0x3c]
    c.emit(transfer_word(C::Al,false,r(2),R::SP,60)?)?;
    // 0x733b8: str r1, [sp, #0x2c]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,44)?)?;
    // 0x733bc: ldr r1, [r7, #0x20]
    c.emit(transfer_word(C::Al,true,r(1),r(7),32)?)?;
    // 0x733c0: bic r3, r1, #1
    c.emit(data_immediate(C::Al,Op::Bic,r(3),r(1),1)?)?;
    // 0x733c4: str r3, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,68)?)?;
    // 0x733c8: ldrb r2, [r4, #0x350]
    c.emit(transfer_byte(C::Al,true,r(2),r(4),848)?)?;
    // 0x733cc: cmp r2, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),0)?)?;
    // 0x733d0: beq 0x73408 <entry+0xb3c>   @ imm = #0x30
    c.branch(C::Eq,0x73408,false)?;
    // 0x733d4: tst r1, #1
    c.emit(data_immediate(C::Al,Op::Tst,R::R0,r(1),1)?)?;
    // 0x733d8: ldrbeq r1, [r7, #0xa]
    c.emit(transfer_byte(C::Eq,true,r(1),r(7),10)?)?;
    // 0x733dc: cmpeq r1, #0
    c.emit(data_immediate(C::Eq,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x733e0: beq 0x7344c <entry+0xb80>   @ imm = #0x64
    c.branch(C::Eq,0x7344c,false)?;
    // 0x733e4: ldr r1, [r4, #0xd14]
    c.emit(transfer_word(C::Al,true,r(1),r(4),3348)?)?;
    // 0x733e8: ldr r2, [r7, #0x100]
    c.emit(transfer_word(C::Al,true,r(2),r(7),256)?)?;
    // 0x733ec: sub r1, r1, #1
    c.emit(data_immediate(C::Al,Op::Sub,r(1),r(1),1)?)?;
    // 0x733f0: cmp r2, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(2),r(1),Shift::Lsl,0,false)?)?;
    // 0x733f4: bne 0x73450 <entry+0xb84>   @ imm = #0x54
    c.branch(C::Ne,0x73450,false)?;
    // 0x733f8: ldrb r1, [r7, #0xa]
    c.emit(transfer_byte(C::Al,true,r(1),r(7),10)?)?;
    // 0x733fc: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x73400: orrne r3, r3, #1
    c.emit(data_immediate(C::Ne,Op::Orr,r(3),r(3),1)?)?;
    // 0x73404: b 0x73450 <entry+0xb84>   @ imm = #0x44
    c.branch(C::Al,0x73450,false)?;
    // 0x73408: ldr r1, [r7, #0x50]
    c.emit(transfer_word(C::Al,true,r(1),r(7),80)?)?;
    // 0x7340c: cmp r1, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),1)?)?;
    // 0x73410: beq 0x73424 <entry+0xb58>   @ imm = #0xc
    c.branch(C::Eq,0x73424,false)?;
    // 0x73414: ldr r1, [r8, #0x4]
    c.emit(transfer_word(C::Al,true,r(1),r(8),4)?)?;
    // 0x73418: ldrb r1, [r1, #0x193]
    c.emit(transfer_byte(C::Al,true,r(1),r(1),403)?)?;
    // 0x7341c: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x73420: bne 0x7342c <entry+0xb60>   @ imm = #0x4
    c.branch(C::Ne,0x7342c,false)?;
    // 0x73424: orr r3, r3, #16
    c.emit(data_immediate(C::Al,Op::Orr,r(3),r(3),16)?)?;
    // 0x73428: str r3, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,68)?)?;
    // 0x7342c: ldrb r2, [r7, #0xa]
    c.emit(transfer_byte(C::Al,true,r(2),r(7),10)?)?;
    // 0x73430: and r1, r3, #16
    c.emit(data_immediate(C::Al,Op::And,r(1),r(3),16)?)?;
    // 0x73434: cmp r2, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),0)?)?;
    // 0x73438: movne r2, #1
    c.emit(data_immediate(C::Ne,Op::Mov,r(2),R::R0,1)?)?;
    // 0x7343c: and r1, r2, r1, lsr #4
    c.emit(data_register(C::Al,Op::And,r(1),r(2),r(1),Shift::Lsr,4,false)?)?;
    // 0x73440: orr r1, r3, r1
    c.emit(data_register(C::Al,Op::Orr,r(1),r(3),r(1),Shift::Lsl,0,false)?)?;
    // 0x73444: str r1, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,68)?)?;
    // 0x73448: b 0x73474 <entry+0xba8>   @ imm = #0x24
    c.branch(C::Al,0x73474,false)?;
    // 0x7344c: ldr r2, [r7, #0x100]
    c.emit(transfer_word(C::Al,true,r(2),r(7),256)?)?;
    // 0x73450: ldr r1, [pc, #0x418]        @ 0xfa4 <entry+0xfa4>
    c.literal(r(1),0x73870)?;
    // 0x73454: str r3, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(3),R::SP,68)?)?;
    // 0x73458: str r0, [sp]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,0)?)?;
    // 0x7345c: mov r0, #32
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,32)?)?;
    // 0x73460: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x447d4
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73464: mov r0, #32
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,32)?)?;
    // 0x73468: mov r1, r10
    c.emit(mov_register(C::Al,r(1),r(10)))?;
    // 0x7346c: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x447e0
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73470: ldr r0, [r7, #0x58]
    c.emit(transfer_word(C::Al,true,r(0),r(7),88)?)?;
    // 0x73474: ldr r1, [r7, #0x2c]
    c.emit(transfer_word(C::Al,true,r(1),r(7),44)?)?;
    // 0x73478: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x7347c: str r1, [sp, #0x60]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,96)?)?;
    // 0x73480: ldr r1, [r7, #0x30]
    c.emit(transfer_word(C::Al,true,r(1),r(7),48)?)?;
    // 0x73484: str r1, [sp, #0x64]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,100)?)?;
    // 0x73488: ldr r1, [r7, #0x28]
    c.emit(transfer_word(C::Al,true,r(1),r(7),40)?)?;
    // 0x7348c: str r1, [sp, #0x4c]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,76)?)?;
    // 0x73490: ldr r1, [r7, #0x24]
    c.emit(transfer_word(C::Al,true,r(1),r(7),36)?)?;
    // 0x73494: str r1, [sp, #0x48]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,72)?)?;
    // 0x73498: beq 0x73538 <entry+0xc6c>   @ imm = #0x98
    c.branch(C::Eq,0x73538,false)?;
    // 0x7349c: cmp r0, #13
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),13)?)?;
    // 0x734a0: blo 0x734c0 <entry+0xbf4>   @ imm = #0x18
    c.branch(C::Cc,0x734c0,false)?;
    // 0x734a4: ldr r1, [r4, #0x284]
    c.emit(transfer_word(C::Al,true,r(1),r(4),644)?)?;
    // 0x734a8: cmp r1, #4096
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),4096)?)?;
    // 0x734ac: ldreq r1, [r7, #0x54]
    c.emit(transfer_word(C::Eq,true,r(1),r(7),84)?)?;
    // 0x734b0: subeq r0, r0, #12
    c.emit(data_immediate(C::Eq,Op::Sub,r(0),r(0),12)?)?;
    // 0x734b4: addeq r1, r1, #12
    c.emit(data_immediate(C::Eq,Op::Add,r(1),r(1),12)?)?;
    // 0x734b8: streq r1, [sp, #0x54]
    c.emit(transfer_word(C::Eq,false,r(1),R::SP,84)?)?;
    // 0x734bc: streq r0, [sp, #0x5c]
    c.emit(transfer_word(C::Eq,false,r(0),R::SP,92)?)?;
    // 0x734c0: ldrb r0, [r7, #0x7]
    c.emit(transfer_byte(C::Al,true,r(0),r(7),7)?)?;
    // 0x734c4: mov r1, #8
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,8)?)?;
    // 0x734c8: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x734cc: movne r0, #0
    c.emit(data_immediate(C::Ne,Op::Mov,r(0),R::R0,0)?)?;
    // 0x734d0: beq 0x734ec <entry+0xc20>   @ imm = #0x14
    c.branch(C::Eq,0x734ec,false)?;
    // 0x734d4: ldrb r2, [r7]
    c.emit(transfer_byte(C::Al,true,r(2),r(7),0)?)?;
    // 0x734d8: mov r1, #4
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,4)?)?;
    // 0x734dc: cmp r2, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),1)?)?;
    // 0x734e0: moveq r1, #2
    c.emit(data_immediate(C::Eq,Op::Mov,r(1),R::R0,2)?)?;
    // 0x734e4: cmp r2, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),0)?)?;
    // 0x734e8: moveq r1, #1
    c.emit(data_immediate(C::Eq,Op::Mov,r(1),R::R0,1)?)?;
    // 0x734ec: str r1, [sp, #0x68]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,104)?)?;
    // 0x734f0: ldr r2, [r8, #0x4]
    c.emit(transfer_word(C::Al,true,r(2),r(8),4)?)?;
    // 0x734f4: ldr r5, [sp, #0x44]
    c.emit(transfer_word(C::Al,true,r(5),R::SP,68)?)?;
    // 0x734f8: ldrb r3, [r2, #0x2fb]
    c.emit(transfer_byte(C::Al,true,r(3),r(2),763)?)?;
    // 0x734fc: bic r1, r5, #64
    c.emit(data_immediate(C::Al,Op::Bic,r(1),r(5),64)?)?;
    // 0x73500: cmp r3, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),1)?)?;
    // 0x73504: orreq r1, r5, #64
    c.emit(data_immediate(C::Eq,Op::Orr,r(1),r(5),64)?)?;
    // 0x73508: str r1, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,68)?)?;
    // 0x7350c: ldrb r3, [r4, #0x350]
    c.emit(transfer_byte(C::Al,true,r(3),r(4),848)?)?;
    // 0x73510: cmp r3, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),0)?)?;
    // 0x73514: beq 0x73548 <entry+0xc7c>   @ imm = #0x2c
    c.branch(C::Eq,0x73548,false)?;
    // 0x73518: ldr r5, [r4, #0xd14]
    c.emit(transfer_word(C::Al,true,r(5),r(4),3348)?)?;
    // 0x7351c: ldr r3, [r7, #0x100]
    c.emit(transfer_word(C::Al,true,r(3),r(7),256)?)?;
    // 0x73520: sub r5, r5, #1
    c.emit(data_immediate(C::Al,Op::Sub,r(5),r(5),1)?)?;
    // 0x73524: cmp r3, r5
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(3),r(5),Shift::Lsl,0,false)?)?;
    // 0x73528: mov r3, #1024
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,1024)?)?;
    // 0x7352c: moveq r3, #16
    c.emit(data_immediate(C::Eq,Op::Mov,r(3),R::R0,16)?)?;
    // 0x73530: orr r1, r3, r1
    c.emit(data_register(C::Al,Op::Orr,r(1),r(3),r(1),Shift::Lsl,0,false)?)?;
    // 0x73534: b 0x73560 <entry+0xc94>   @ imm = #0x24
    c.branch(C::Al,0x73560,false)?;
    // 0x73538: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x7353c: str r0, [sp, #0x68]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,104)?)?;
    // 0x73540: ldr r1, [sp, #0x44]
    c.emit(transfer_word(C::Al,true,r(1),R::SP,68)?)?;
    // 0x73544: b 0x7358c <entry+0xcc0>   @ imm = #0x40
    c.branch(C::Al,0x7358c,false)?;
    // 0x73548: ldr r3, [r7, #0x50]
    c.emit(transfer_word(C::Al,true,r(3),r(7),80)?)?;
    // 0x7354c: cmp r3, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(3),1)?)?;
    // 0x73550: ldrbne r3, [r2, #0x193]
    c.emit(transfer_byte(C::Ne,true,r(3),r(2),403)?)?;
    // 0x73554: cmpne r3, #0
    c.emit(data_immediate(C::Ne,Op::Cmp,R::R0,r(3),0)?)?;
    // 0x73558: bne 0x73564 <entry+0xc98>   @ imm = #0x4
    c.branch(C::Ne,0x73564,false)?;
    // 0x7355c: orr r1, r1, #16
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),16)?)?;
    // 0x73560: str r1, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,68)?)?;
    // 0x73564: ldr r2, [r2, #0x4]
    c.emit(transfer_word(C::Al,true,r(2),r(2),4)?)?;
    // 0x73568: cmp r2, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),2)?)?;
    // 0x7356c: beq 0x73580 <entry+0xcb4>   @ imm = #0xc
    c.branch(C::Eq,0x73580,false)?;
    // 0x73570: ldrb r0, [r7]
    c.emit(transfer_byte(C::Al,true,r(0),r(7),0)?)?;
    // 0x73574: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73578: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x7357c: moveq r0, #1
    c.emit(data_immediate(C::Eq,Op::Mov,r(0),R::R0,1)?)?;
    // 0x73580: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73584: orrne r1, r1, #32
    c.emit(data_immediate(C::Ne,Op::Orr,r(1),r(1),32)?)?;
    // 0x73588: str r1, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,68)?)?;
    // 0x7358c: tst r1, #1
    c.emit(data_immediate(C::Al,Op::Tst,R::R0,r(1),1)?)?;
    // 0x73590: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x73594: conditional diagnostic pointer for alternate codec state
    c.emit(transfer_word(C::Eq,true,r(1),R::PC,0x2dc)?)?;
    // 0x73598: complementary diagnostic pointer
    c.emit(transfer_word(C::Ne,true,r(1),R::PC,0x2d4)?)?;
    // 0x7359c: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44910
    c.branch(C::Al,0x2ec94,true)?;
    // 0x735a0: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x735a4: mov r1, r10
    c.emit(mov_register(C::Al,r(1),r(10)))?;
    // 0x735a8: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x4491c
    c.branch(C::Al,0x2ec94,true)?;
    // 0x735ac: ldr r0, [sp, #0x44]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,68)?)?;
    // 0x735b0: tst r0, #1
    c.emit(data_immediate(C::Al,Op::Tst,R::R0,r(0),1)?)?;
    // 0x735b4: ldrne r1, [sp, #0x6c]
    c.emit(transfer_word(C::Ne,true,r(1),R::SP,108)?)?;
    // 0x735b8: cmpne r1, #0
    c.emit(data_immediate(C::Ne,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x735bc: beq 0x7362c <entry+0xd60>   @ imm = #0x68
    c.branch(C::Eq,0x7362c,false)?;
    // 0x735c0: ldrb r2, [r7, #0xa]
    c.emit(transfer_byte(C::Al,true,r(2),r(7),10)?)?;
    // 0x735c4: cmp r2, #3
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),3)?)?;
    // 0x735c8: bne 0x7362c <entry+0xd60>   @ imm = #0x5c
    c.branch(C::Ne,0x7362c,false)?;
    // 0x735cc: bic r0, r0, #1
    c.emit(data_immediate(C::Al,Op::Bic,r(0),r(0),1)?)?;
    // 0x735d0: str r0, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,68)?)?;
    // 0x735d4: ldr r0, [r4, #0x7c]
    c.emit(transfer_word(C::Al,true,r(0),r(4),124)?)?;
    // 0x735d8: bl 0x7ec70 <entry+0xc3a4>  @ imm = #0xb690
    c.branch(C::Al,0x7ec70,true)?;
    // 0x735dc: ldr r0, [r4, #0x1c]
    c.emit(transfer_word(C::Al,true,r(0),r(4),28)?)?;
    // 0x735e0: add r1, sp, #44
    c.emit(data_immediate(C::Al,Op::Add,r(1),R::SP,44)?)?;
    // 0x735e4: sub r0, r0, #1
    c.emit(data_immediate(C::Al,Op::Sub,r(0),r(0),1)?)?;
    // 0x735e8: str r0, [r4, #0x1c]
    c.emit(transfer_word(C::Al,false,r(0),r(4),28)?)?;
    // 0x735ec: ldr r0, [r4, #0x20]
    c.emit(transfer_word(C::Al,true,r(0),r(4),32)?)?;
    // 0x735f0: bl 0x2a878 <entry+0xfffffffffffb7fac> @ imm = #-0x48d80
    c.branch(C::Al,0x2a878,true)?;
    // 0x735f4: ldr r0, [sp, #0x28]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,40)?)?;
    // 0x735f8: mov r1, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,0)?)?;
    // 0x735fc: mov r2, #64
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,64)?)?;
    // 0x73600: bl 0x31ec <entry+0xfffffffffff90920> @ imm = #-0x7041c
    c.branch(C::Al,0x31ec,true)?;
    // 0x73604: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x73608: str r9, [sp, #0x30]
    c.emit(transfer_word(C::Al,false,r(9),R::SP,48)?)?;
    // 0x7360c: str r0, [sp, #0x40]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,64)?)?;
    // 0x73610: str r0, [sp, #0x68]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,104)?)?;
    // 0x73614: ldr r0, [r4, #0x14]
    c.emit(transfer_word(C::Al,true,r(0),r(4),20)?)?;
    // 0x73618: str r0, [sp, #0x34]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,52)?)?;
    // 0x7361c: mov r0, #76
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,76)?)?;
    // 0x73620: str r0, [sp, #0x2c]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,44)?)?;
    // 0x73624: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x73628: str r0, [sp, #0x44]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,68)?)?;
    // 0x7362c: ldrb r0, [r4, #0x57b]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1403)?)?;
    // 0x73630: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73634: beq 0x73644 <entry+0xd78>   @ imm = #0x8
    c.branch(C::Eq,0x73644,false)?;
    // 0x73638: ldr r0, [r7, #0x6c]
    c.emit(transfer_word(C::Al,true,r(0),r(7),108)?)?;
    // 0x7363c: ldr r1, [sp, #0x58]
    c.emit(transfer_word(C::Al,true,r(1),R::SP,88)?)?;
    // 0x73640: bl 0x205c <entry+0xfffffffffff8f790> @ imm = #-0x715ec
    c.branch(C::Al,0x205c,true)?;
    // 0x73644: ldrb r0, [r4, #0x41c]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1052)?)?;
    // 0x73648: mov r5, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(5),R::R0,1)?)?;
    // 0x7364c: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x73650: beq 0x73660 <entry+0xd94>   @ imm = #0x8
    c.branch(C::Eq,0x73660,false)?;
    // 0x73654: ldrb r0, [r7, #0x20]
    c.emit(transfer_byte(C::Al,true,r(0),r(7),32)?)?;
    // 0x73658: tst r0, #1
    c.emit(data_immediate(C::Al,Op::Tst,R::R0,r(0),1)?)?;
    // 0x7365c: moveq r5, #0
    c.emit(data_immediate(C::Eq,Op::Mov,r(5),R::R0,0)?)?;
    // 0x73660: ldr r1, [sp, #0x6c]
    c.emit(transfer_word(C::Al,true,r(1),R::SP,108)?)?;
    // 0x73664: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x73668: beq 0x73680 <entry+0xdb4>   @ imm = #0x10
    c.branch(C::Eq,0x73680,false)?;
    // 0x7366c: ldr r0, [r4, #0x7c]
    c.emit(transfer_word(C::Al,true,r(0),r(4),124)?)?;
    // 0x73670: bl 0x7ec70 <entry+0xc3a4>  @ imm = #0xb5f8
    c.branch(C::Al,0x7ec70,true)?;
    // 0x73674: ldr r0, [r4, #0x1c]
    c.emit(transfer_word(C::Al,true,r(0),r(4),28)?)?;
    // 0x73678: sub r0, r0, #1
    c.emit(data_immediate(C::Al,Op::Sub,r(0),r(0),1)?)?;
    // 0x7367c: str r0, [r4, #0x1c]
    c.emit(transfer_word(C::Al,false,r(0),r(4),28)?)?;
    // 0x73680: ldr r0, [r4, #0x20]
    c.emit(transfer_word(C::Al,true,r(0),r(4),32)?)?;
    // 0x73684: add r1, sp, #44
    c.emit(data_immediate(C::Al,Op::Add,r(1),R::SP,44)?)?;
    // 0x73688: mov r2, r5
    c.emit(mov_register(C::Al,r(2),r(5)))?;
    // 0x7368c: bl 0x2a18c <entry+0xfffffffffffb78c0> @ imm = #-0x49508
    c.branch(C::Al,0x2a18c,true)?;
    // 0x73690: ldr r0, [r4, #0x284]
    c.emit(transfer_word(C::Al,true,r(0),r(4),644)?)?;
    // 0x73694: cmp r0, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),8192)?)?;
    // 0x73698: beq 0x736c4 <entry+0xdf8>   @ imm = #0x24
    c.branch(C::Eq,0x736c4,false)?;
    // 0x7369c: cmp r0, #4096
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),4096)?)?;
    // 0x736a0: beq 0x736dc <entry+0xe10>   @ imm = #0x34
    c.branch(C::Eq,0x736dc,false)?;
    // 0x736a4: cmp r0, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),2)?)?;
    // 0x736a8: bne 0x73710 <entry+0xe44>   @ imm = #0x60
    c.branch(C::Ne,0x73710,false)?;
    // 0x736ac: ldr r5, [r4, #0x14]
    c.emit(transfer_word(C::Al,true,r(5),r(4),20)?)?;
    // 0x736b0: bl 0x26754 <entry+0xfffffffffffb3e88> @ imm = #-0x4cf64
    c.branch(C::Al,0x26754,true)?;
    // 0x736b4: mov r3, r0
    c.emit(mov_register(C::Al,r(3),r(0)))?;
    // 0x736b8: ldr r0, [sp, #0x6c]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,108)?)?;
    // 0x736bc: ldr r1, [pc, #0x1c0]        @ 0xfb8 <entry+0xfb8>
    c.literal(r(1),0x73884)?;
    // 0x736c0: b 0x736f0 <entry+0xe24>   @ imm = #0x28
    c.branch(C::Al,0x736f0,false)?;
    // 0x736c4: ldr r5, [r4, #0x14]
    c.emit(transfer_word(C::Al,true,r(5),r(4),20)?)?;
    // 0x736c8: bl 0x26754 <entry+0xfffffffffffb3e88> @ imm = #-0x4cf7c
    c.branch(C::Al,0x26754,true)?;
    // 0x736cc: mov r3, r0
    c.emit(mov_register(C::Al,r(3),r(0)))?;
    // 0x736d0: ldr r0, [sp, #0x6c]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,108)?)?;
    // 0x736d4: ldr r1, [pc, #0x1a4]        @ 0xfb4 <entry+0xfb4>
    c.literal(r(1),0x73880)?;
    // 0x736d8: b 0x736f0 <entry+0xe24>   @ imm = #0x10
    c.branch(C::Al,0x736f0,false)?;
    // 0x736dc: ldr r5, [r4, #0x14]
    c.emit(transfer_word(C::Al,true,r(5),r(4),20)?)?;
    // 0x736e0: bl 0x26754 <entry+0xfffffffffffb3e88> @ imm = #-0x4cf94
    c.branch(C::Al,0x26754,true)?;
    // 0x736e4: mov r3, r0
    c.emit(mov_register(C::Al,r(3),r(0)))?;
    // 0x736e8: ldr r0, [sp, #0x6c]
    c.emit(transfer_word(C::Al,true,r(0),R::SP,108)?)?;
    // 0x736ec: ldr r1, [pc, #0x188]        @ 0xfb0 <entry+0xfb0>
    c.literal(r(1),0x7387c)?;
    // 0x736f0: str r0, [sp]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,0)?)?;
    // 0x736f4: mov r0, #32
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,32)?)?;
    // 0x736f8: mov r2, r5
    c.emit(mov_register(C::Al,r(2),r(5)))?;
    // 0x736fc: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44a70
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73700: mov r0, #32
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,32)?)?;
    // 0x73704: mov r1, r10
    c.emit(mov_register(C::Al,r(1),r(10)))?;
    // 0x73708: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44a7c
    c.branch(C::Al,0x2ec94,true)?;
    // 0x7370c: b 0x73744 <entry+0xe78>   @ imm = #0x30
    c.branch(C::Al,0x73744,false)?;
    // 0x73710: ldr r1, [pc, #0x170]        @ 0xfbc <entry+0xfbc>
    c.literal(r(1),0x73888)?;
    // 0x73714: ldr r2, [pc, #0x170]        @ 0xfc0 <entry+0xfc0>
    c.literal(r(2),0x7388c)?;
    // 0x73718: mov r3, #167
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,167)?)?;
    // 0x7371c: mov r0, #8
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,8)?)?;
    // 0x73720: orr r3, r3, #512
    c.emit(data_immediate(C::Al,Op::Orr,r(3),r(3),512)?)?;
    // 0x73724: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44a98
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73728: mov r0, #8
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,8)?)?;
    // 0x7372c: mov r1, r10
    c.emit(mov_register(C::Al,r(1),r(10)))?;
    // 0x73730: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44aa4
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73734: ldr r2, [pc, #0x11c]        @ 0xf8c <entry+0xf8c>
    c.literal(r(2),0x73858)?;
    // 0x73738: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x7373c: mov r1, #680
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,680)?)?;
    // 0x73740: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x71f98
    c.branch(C::Al,0x17b0,true)?;
    // 0x73744: ldrb r0, [r4, #0x413]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1043)?)?;
    // 0x73748: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x7374c: beq 0x73794 <entry+0xec8>   @ imm = #0x40
    c.branch(C::Eq,0x73794,false)?;
    // 0x73750: ldr r5, [r4, #0x14]
    c.emit(transfer_word(C::Al,true,r(5),r(4),20)?)?;
    // 0x73754: bl 0x26754 <entry+0xfffffffffffb3e88> @ imm = #-0x4d008
    c.branch(C::Al,0x26754,true)?;
    // 0x73758: mov r3, r0
    c.emit(mov_register(C::Al,r(3),r(0)))?;
    // 0x7375c: ldr r0, [r7, #0x10]
    c.emit(transfer_word(C::Al,true,r(0),r(7),16)?)?;
    // 0x73760: ldr r1, [r7, #0x58]
    c.emit(transfer_word(C::Al,true,r(1),r(7),88)?)?;
    // 0x73764: ldr r2, [r7, #0x104]
    c.emit(transfer_word(C::Al,true,r(2),r(7),260)?)?;
    // 0x73768: ldr r6, [sp, #0x6c]
    c.emit(transfer_word(C::Al,true,r(6),R::SP,108)?)?;
    // 0x7376c: str r6, [sp]
    c.emit(transfer_word(C::Al,false,r(6),R::SP,0)?)?;
    // 0x73770: stmib sp, {r0, r2}
    c.emit(transfer_multiple(C::Al,false,R::SP,r(0).bit()|r(2).bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x73774: str r1, [sp, #0xc]
    c.emit(transfer_word(C::Al,false,r(1),R::SP,12)?)?;
    // 0x73778: ldr r1, [pc, #0x110]        @ 0xfc4 <entry+0xfc4>
    c.literal(r(1),0x73890)?;
    // 0x7377c: mov r0, #32
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,32)?)?;
    // 0x73780: mov r2, r5
    c.emit(mov_register(C::Al,r(2),r(5)))?;
    // 0x73784: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44af8
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73788: mov r0, #32
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,32)?)?;
    // 0x7378c: mov r1, r10
    c.emit(mov_register(C::Al,r(1),r(10)))?;
    // 0x73790: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44b04
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73794: ldrb r0, [r4, #0x41c]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),1052)?)?;
    // 0x73798: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x7379c: ldrbne r0, [r7, #0x20]
    c.emit(transfer_byte(C::Ne,true,r(0),r(7),32)?)?;
    // 0x737a0: tstne r0, #32
    c.emit(data_immediate(C::Ne,Op::Tst,R::R0,r(0),32)?)?;
    // 0x737a4: beq 0x737f4 <entry+0xf28>   @ imm = #0x48
    c.branch(C::Eq,0x737f4,false)?;
    // 0x737a8: mov r0, #20
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,20)?)?;
    // 0x737ac: add r1, sp, #120
    c.emit(data_immediate(C::Al,Op::Add,r(1),R::SP,120)?)?;
    // 0x737b0: str r0, [sp, #0x78]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,120)?)?;
    // 0x737b4: add r0, r9, #5
    c.emit(data_immediate(C::Al,Op::Add,r(0),r(9),5)?)?;
    // 0x737b8: str r0, [sp, #0x7c]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,124)?)?;
    // 0x737bc: ldr r0, [r4, #0x14]
    c.emit(transfer_word(C::Al,true,r(0),r(4),20)?)?;
    // 0x737c0: str r0, [sp, #0x80]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,128)?)?;
    // 0x737c4: ldr r0, [r7, #0x40]
    c.emit(transfer_word(C::Al,true,r(0),r(7),64)?)?;
    // 0x737c8: str r0, [sp, #0x84]
    c.emit(transfer_word(C::Al,false,r(0),R::SP,132)?)?;
    // 0x737cc: ldr r0, [r4, #0x20]
    c.emit(transfer_word(C::Al,true,r(0),r(4),32)?)?;
    // 0x737d0: bl 0x2a878 <entry+0xfffffffffffb7fac> @ imm = #-0x48f60
    c.branch(C::Al,0x2a878,true)?;
    // 0x737d4: ldr r2, [r4, #0x14]
    c.emit(transfer_word(C::Al,true,r(2),r(4),20)?)?;
    // 0x737d8: ldr r3, [sp, #0x84]
    c.emit(transfer_word(C::Al,true,r(3),R::SP,132)?)?;
    // 0x737dc: ldr r1, [pc, #0xb0]         @ 0xfc8 <entry+0xfc8>
    c.literal(r(1),0x73894)?;
    // 0x737e0: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x737e4: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44b58
    c.branch(C::Al,0x2ec94,true)?;
    // 0x737e8: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x737ec: mov r1, r10
    c.emit(mov_register(C::Al,r(1),r(10)))?;
    // 0x737f0: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44b64
    c.branch(C::Al,0x2ec94,true)?;
    // 0x737f4: ldr r0, [r4, #0x284]
    c.emit(transfer_word(C::Al,true,r(0),r(4),644)?)?;
    // 0x737f8: mov r5, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(5),R::R0,0)?)?;
    // 0x737fc: cmp r0, #4096
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),4096)?)?;
    // 0x73800: ldr r0, [r4, #0x38]
    c.emit(transfer_word(C::Al,true,r(0),r(4),56)?)?;
    // 0x73804: ldreq r5, [r7, #0x74]
    c.emit(transfer_word(C::Eq,true,r(5),r(7),116)?)?;
    // 0x73808: ldr r0, [r0, #0x8]
    c.emit(transfer_word(C::Al,true,r(0),r(0),8)?)?;
    // 0x7380c: bl 0x35df4 <entry+0xfffffffffffc3528> @ imm = #-0x3da20
    c.branch(C::Al,0x35df4,true)?;
    // 0x73810: ldr r0, [r4, #0x284]
    c.emit(transfer_word(C::Al,true,r(0),r(4),644)?)?;
    // 0x73814: cmp r0, #4096
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),4096)?)?;
    // 0x73818: streq r5, [r7, #0x74]
    c.emit(transfer_word(C::Eq,false,r(5),r(7),116)?)?;
    // 0x7381c: ldr r8, [r4, #0x38]
    c.emit(transfer_word(C::Al,true,r(8),r(4),56)?)?;
    // 0x73820: ldr r0, [r8, #0x8]
    c.emit(transfer_word(C::Al,true,r(0),r(8),8)?)?;
    // 0x73824: bl 0x35d44 <entry+0xfffffffffffc3478> @ imm = #-0x3dae8
    c.branch(C::Al,0x35d44,true)?;
    // 0x73828: mov r7, r0
    c.emit(mov_register(C::Al,r(7),r(0)))?;
    // 0x7382c: cmp r7, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(7),0)?)?;
    // 0x73830: bne 0x72e00 <entry+0x534>   @ imm = #-0xa38
    c.branch(C::Ne,0x72e00,false)?;
    // 0x73834: ldr r1, [pc, #0x5c]         @ 0xfcc <entry+0xfcc>
    c.literal(r(1),0x73898)?;
    // 0x73838: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x7383c: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44bb0
    c.branch(C::Al,0x2ec94,true)?;
    // 0x73840: ldr r1, [pc, #0x1c]         @ 0xf98 <entry+0xf98>
    c.literal(r(1),0x73864)?;
    // 0x73844: mov r0, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,1)?)?;
    // 0x73848: bl 0x2ec94 <entry+0xfffffffffffbc3c8> @ imm = #-0x44bbc
    c.branch(C::Al,0x2ec94,true)?;
    // 0x7384c: sub sp, r11, #28
    c.emit(data_immediate(C::Al,Op::Sub,R::SP,r(11),28)?)?;
    // 0x73850: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(8).bit()|r(9).bit()|r(10).bit()|r(11).bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;

    c.check(0x73854)?;
    for word in [CONTROLLER_STATUS_MASK,ALLOCATOR_DIAGNOSTIC,WORKER_RESOURCE_MASK,WORKER_START_DIAGNOSTIC,PLATFORM_LOG_FOOTER,WORKER_EXIT_DIAGNOSTIC,CODEC_RUNTIME_STATUS,CODEC_QUEUE_DIAGNOSTIC,CODEC_BUFFER_DIAGNOSTIC,CODEC_CONTROL_DIAGNOSTIC,CODEC_READY_DIAGNOSTIC,CODEC_RESET_DIAGNOSTIC,CODEC_DISABLED_DIAGNOSTIC,CODEC_SOURCE_CONTEXT,CODEC_BUSY_DIAGNOSTIC,CODEC_TIMEOUT_DIAGNOSTIC,CODEC_COMPLETE_DIAGNOSTIC,CODEC_WORKER_DIAGNOSTIC] {c.emit(word)?;}
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();finish_controller_run(&mut c)?;Ok(c.bytes)}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn runtime_run_tail_contains_18_data_words(){
        let b=fragment().unwrap();assert_eq!(b.len(),END-START);
        for (i,word) in [CONTROLLER_STATUS_MASK,ALLOCATOR_DIAGNOSTIC,WORKER_RESOURCE_MASK,WORKER_START_DIAGNOSTIC,PLATFORM_LOG_FOOTER,WORKER_EXIT_DIAGNOSTIC,CODEC_RUNTIME_STATUS,CODEC_QUEUE_DIAGNOSTIC,CODEC_BUFFER_DIAGNOSTIC,CODEC_CONTROL_DIAGNOSTIC,CODEC_READY_DIAGNOSTIC,CODEC_RESET_DIAGNOSTIC,CODEC_DISABLED_DIAGNOSTIC,CODEC_SOURCE_CONTEXT,CODEC_BUSY_DIAGNOSTIC,CODEC_TIMEOUT_DIAGNOSTIC,CODEC_COMPLETE_DIAGNOSTIC,CODEC_WORKER_DIAGNOSTIC].into_iter().enumerate(){
            let start=b.len()-18*4+i*4;
            assert_eq!(&b[start..start+4],&word.to_le_bytes());
        }
    }
}
