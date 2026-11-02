// SPDX-License-Identifier: GPL-3.0-only
//! Rebuild the Venus controller throughput/quality tier selection. The
//! original stores weighted sample counts, scales one sample by 100,
//! divides by a platform Thumb helper, and chooses discrete output tiers
//! using per-codec thresholds and a byte-sized controller adjustment.
//! Opcode words are independently derived from typed ARM32 operand values.
use arm32::{branch, branch_exchange_immediate, data_immediate,
    data_register, mov_register, mov_shifted_register, multiply_register,
    push_registers, transfer_byte, transfer_word, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START:usize=0x728cc;
pub const END:usize=0x72bc8;
pub const FILE_BASE:usize=0x4000;
const INVALID_SAMPLE_DIAGNOSTIC:u32=0x000f_2e4c;
const CODEC_TIER_THRESHOLD:u32=0x0011_93ff;

struct Code{at:usize,bytes:Vec<u8>}
impl Code{
    fn new()->Self{Self{at:START,bytes:Vec::new()}}
    fn check(&self,where_at:usize)->Result<(),String>{
        if self.at==where_at {Ok(())}else{Err(format!("codec throughput address 0x{:x} expected 0x{:x}",self.at,where_at))}
    }
    fn emit(&mut self,word:u32)->Result<(),String>{
        if self.at<START || self.at>=END || self.at%4!=0{return Err(format!("invalid throughput emission at 0x{:x}",self.at));}
        self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
    }
    fn branch(&mut self,condition:C,target:usize,link:bool)->Result<(),String>{
        self.emit(branch(condition,self.at as u32,target as u32,link)?)
    }
    fn literal(&mut self,destination:R,address:usize)->Result<(),String>{
        self.emit(transfer_word(C::Al,true,destination,R::PC,address as i32-self.at as i32-8)?)
    }
}
fn r(n:u8)->R{R::new(n).expect("A32 register number")}

fn choose_controller_tier(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x728cc: push {r4, r5, r6, r7, r11, lr}
    c.emit(push_registers(C::Al,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(11).bit()|R::LR.bit())?)?;
    // 0x728d0: add r11, sp, #16
    c.emit(data_immediate(C::Al,Op::Add,r(11),R::SP,16)?)?;
    // 0x728d4: mov r4, r0
    c.emit(mov_register(C::Al,r(4),r(0)))?;
    // 0x728d8: mov r3, #100
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,100)?)?;
    // 0x728dc: ldr r2, [r4, #0x60]
    c.emit(transfer_word(C::Al,true,r(2),r(4),96)?)?;
    // 0x728e0: ldr r0, [r4, #0x54]
    c.emit(transfer_word(C::Al,true,r(0),r(4),84)?)?;
    // 0x728e4: ldr r1, [r4, #0x58]
    c.emit(transfer_word(C::Al,true,r(1),r(4),88)?)?;
    // 0x728e8: ldr r7, [r4]
    c.emit(transfer_word(C::Al,true,r(7),r(4),0)?)?;
    // 0x728ec: mul r6, r0, r1
    c.emit(multiply_register(C::Al,r(6),r(0),r(1))?)?;
    // 0x728f0: mul r5, r2, r3
    c.emit(multiply_register(C::Al,r(5),r(2),r(3))?)?;
    // 0x728f4: add r0, r5, r6, lsr #1
    c.emit(data_register(C::Al,Op::Add,r(0),r(5),r(6),Shift::Lsr,1,false)?)?;
    // 0x728f8: mov r1, r6
    c.emit(mov_register(C::Al,r(1),r(6)))?;
    // 0x728fc: blx 0x83c4c <entry+0x11380> @ imm = #0x11348
    c.emit(branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x72900: mov r5, r0
    c.emit(mov_register(C::Al,r(5),r(0)))?;
    // 0x72904: cmp r7, #4096
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(7),4096)?)?;
    // 0x72908: bne 0x72948 <entry+0x7c>    @ imm = #0x38
    c.branch(C::Ne,0x72948,false)?;
    // 0x7290c: ldr r1, [pc, #0x2b0]        @ 0x2f8 <entry+0x2f8>
    c.literal(r(1),0x72bc4)?;
    // 0x72910: mvn r0, #921600
    c.emit(data_immediate(C::Al,Op::Mvn,r(0),R::R0,921600)?)?;
    // 0x72914: add r0, r6, r0
    c.emit(data_register(C::Al,Op::Add,r(0),r(6),r(0),Shift::Lsl,0,false)?)?;
    // 0x72918: cmp r0, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(0),r(1),Shift::Lsl,0,false)?)?;
    // 0x7291c: bhi 0x729e0 <entry+0x114>   @ imm = #0xbc
    c.branch(C::Hi,0x729e0,false)?;
    // 0x72920: mov r6, #127
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,127)?)?;
    // 0x72924: cmp r5, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),1)?)?;
    // 0x72928: movhi r6, #76
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,76)?)?;
    // 0x7292c: cmp r5, #8
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),8)?)?;
    // 0x72930: movhi r6, #54
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,54)?)?;
    // 0x72934: cmp r5, #21
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),21)?)?;
    // 0x72938: movhi r6, #22
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,22)?)?;
    // 0x7293c: cmp r5, #31
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),31)?)?;
    // 0x72940: movhi r6, #14
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,14)?)?;
    // 0x72944: b 0x72b34 <entry+0x268>   @ imm = #0x1e8
    c.branch(C::Al,0x72b34,false)?;
    // 0x72948: ldrb r0, [r4, #0x25]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),37)?)?;
    // 0x7294c: cmp r0, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),1)?)?;
    // 0x72950: bhi 0x72a1c <entry+0x150>   @ imm = #0xc4
    c.branch(C::Hi,0x72a1c,false)?;
    // 0x72954: mov r3, #20
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,20)?)?;
    // 0x72958: mov r0, #120
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,120)?)?;
    // 0x7295c: mov r2, #60
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,60)?)?;
    // 0x72960: mov r1, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,1)?)?;
    // 0x72964: cmp r7, #2
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(7),2)?)?;
    // 0x72968: beq 0x729ac <entry+0xe0>    @ imm = #0x3c
    c.branch(C::Eq,0x729ac,false)?;
    // 0x7296c: ldrb r6, [r4, #0x780]
    c.emit(transfer_byte(C::Al,true,r(6),r(4),1920)?)?;
    // 0x72970: mov r1, #1
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,1)?)?;
    // 0x72974: cmp r6, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(6),0)?)?;
    // 0x72978: bne 0x72988 <entry+0xbc>    @ imm = #0x8
    c.branch(C::Ne,0x72988,false)?;
    // 0x7297c: ldrb r1, [r4, #0x8a]
    c.emit(transfer_byte(C::Al,true,r(1),r(4),138)?)?;
    // 0x72980: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x72984: movne r1, #1
    c.emit(data_immediate(C::Ne,Op::Mov,r(1),R::R0,1)?)?;
    // 0x72988: cmp r7, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(7),8192)?)?;
    // 0x7298c: bne 0x729ac <entry+0xe0>    @ imm = #0x18
    c.branch(C::Ne,0x729ac,false)?;
    // 0x72990: mov r0, #30
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,30)?)?;
    // 0x72994: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x72998: mov r2, #20
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,20)?)?;
    // 0x7299c: mov r3, #10
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,10)?)?;
    // 0x729a0: movne r0, #96
    c.emit(data_immediate(C::Ne,Op::Mov,r(0),R::R0,96)?)?;
    // 0x729a4: movne r2, #48
    c.emit(data_immediate(C::Ne,Op::Mov,r(2),R::R0,48)?)?;
    // 0x729a8: movne r3, #16
    c.emit(data_immediate(C::Ne,Op::Mov,r(3),R::R0,16)?)?;
    // 0x729ac: mov r6, #35
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,35)?)?;
    // 0x729b0: cmp r5, r3
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(3),Shift::Lsl,0,false)?)?;
    // 0x729b4: bls 0x72b34 <entry+0x268>   @ imm = #0x178
    c.branch(C::Ls,0x72b34,false)?;
    // 0x729b8: cmp r5, r2
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(2),Shift::Lsl,0,false)?)?;
    // 0x729bc: bls 0x72ad4 <entry+0x208>   @ imm = #0x110
    c.branch(C::Ls,0x72ad4,false)?;
    // 0x729c0: mov r2, #25
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,25)?)?;
    // 0x729c4: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x729c8: mov r6, #28
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,28)?)?;
    // 0x729cc: movne r2, #10
    c.emit(data_immediate(C::Ne,Op::Mov,r(2),R::R0,10)?)?;
    // 0x729d0: movne r6, #20
    c.emit(data_immediate(C::Ne,Op::Mov,r(6),R::R0,20)?)?;
    // 0x729d4: cmp r5, r0
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(0),Shift::Lsl,0,false)?)?;
    // 0x729d8: movhi r6, r2
    c.emit(mov_register(C::Hi,r(6),r(2)))?;
    // 0x729dc: b 0x72b34 <entry+0x268>   @ imm = #0x150
    c.branch(C::Al,0x72b34,false)?;
    // 0x729e0: mvn r0, #307200
    c.emit(data_immediate(C::Al,Op::Mvn,r(0),R::R0,307200)?)?;
    // 0x729e4: add r0, r6, r0
    c.emit(data_register(C::Al,Op::Add,r(0),r(6),r(0),Shift::Lsl,0,false)?)?;
    // 0x729e8: lsr r0, r0, #13
    c.emit(mov_shifted_register(C::Al,r(0),r(0),Shift::Lsr,13)?)?;
    // 0x729ec: cmp r0, #74
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),74)?)?;
    // 0x729f0: bhi 0x72a7c <entry+0x1b0>   @ imm = #0x84
    c.branch(C::Hi,0x72a7c,false)?;
    // 0x729f4: mov r6, #127
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,127)?)?;
    // 0x729f8: cmp r5, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),1)?)?;
    // 0x729fc: movhi r6, #119
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,119)?)?;
    // 0x72a00: cmp r5, #8
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),8)?)?;
    // 0x72a04: movhi r6, #90
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,90)?)?;
    // 0x72a08: cmp r5, #21
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),21)?)?;
    // 0x72a0c: movhi r6, #46
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,46)?)?;
    // 0x72a10: cmp r5, #31
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),31)?)?;
    // 0x72a14: movhi r6, #30
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,30)?)?;
    // 0x72a18: b 0x72b34 <entry+0x268>   @ imm = #0x114
    c.branch(C::Al,0x72b34,false)?;
    // 0x72a1c: mov r0, #30
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,30)?)?;
    // 0x72a20: cmp r7, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(7),8192)?)?;
    // 0x72a24: mov r1, #20
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,20)?)?;
    // 0x72a28: mov r2, #15
    c.emit(data_immediate(C::Al,Op::Mov,r(2),R::R0,15)?)?;
    // 0x72a2c: moveq r0, #24
    c.emit(data_immediate(C::Eq,Op::Mov,r(0),R::R0,24)?)?;
    // 0x72a30: moveq r1, #16
    c.emit(data_immediate(C::Eq,Op::Mov,r(1),R::R0,16)?)?;
    // 0x72a34: moveq r2, #12
    c.emit(data_immediate(C::Eq,Op::Mov,r(2),R::R0,12)?)?;
    // 0x72a38: cmp r7, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(7),8192)?)?;
    // 0x72a3c: cmpne r7, #2
    c.emit(data_immediate(C::Ne,Op::Cmp,R::R0,r(7),2)?)?;
    // 0x72a40: bne 0x72ab8 <entry+0x1ec>   @ imm = #0x70
    c.branch(C::Ne,0x72ab8,false)?;
    // 0x72a44: mov r3, #10
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,10)?)?;
    // 0x72a48: cmp r7, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(7),8192)?)?;
    // 0x72a4c: mov r6, #35
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,35)?)?;
    // 0x72a50: moveq r3, #8
    c.emit(data_immediate(C::Eq,Op::Mov,r(3),R::R0,8)?)?;
    // 0x72a54: cmp r5, r3
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(3),Shift::Lsl,0,false)?)?;
    // 0x72a58: movhi r6, #30
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,30)?)?;
    // 0x72a5c: cmphi r5, r2
    c.emit(data_register(C::Hi,Op::Cmp,R::R0,r(5),r(2),Shift::Lsl,0,false)?)?;
    // 0x72a60: bls 0x72b34 <entry+0x268>   @ imm = #0xcc
    c.branch(C::Ls,0x72b34,false)?;
    // 0x72a64: mov r6, #15
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,15)?)?;
    // 0x72a68: cmp r5, r0
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(0),Shift::Lsl,0,false)?)?;
    // 0x72a6c: movls r6, #20
    c.emit(data_immediate(C::Ls,Op::Mov,r(6),R::R0,20)?)?;
    // 0x72a70: cmp r5, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(1),Shift::Lsl,0,false)?)?;
    // 0x72a74: movls r6, #25
    c.emit(data_immediate(C::Ls,Op::Mov,r(6),R::R0,25)?)?;
    // 0x72a78: b 0x72b34 <entry+0x268>   @ imm = #0xb4
    c.branch(C::Al,0x72b34,false)?;
    // 0x72a7c: mvn r0, #101376
    c.emit(data_immediate(C::Al,Op::Mvn,r(0),R::R0,101376)?)?;
    // 0x72a80: add r0, r6, r0
    c.emit(data_register(C::Al,Op::Add,r(0),r(6),r(0),Shift::Lsl,0,false)?)?;
    // 0x72a84: lsr r0, r0, #10
    c.emit(mov_shifted_register(C::Al,r(0),r(0),Shift::Lsr,10)?)?;
    // 0x72a88: cmp r0, #200
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),200)?)?;
    // 0x72a8c: bhi 0x72ae4 <entry+0x218>   @ imm = #0x50
    c.branch(C::Hi,0x72ae4,false)?;
    // 0x72a90: mov r6, #127
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,127)?)?;
    // 0x72a94: cmp r5, #1
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),1)?)?;
    // 0x72a98: movhi r6, #116
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,116)?)?;
    // 0x72a9c: cmp r5, #8
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),8)?)?;
    // 0x72aa0: movhi r6, #86
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,86)?)?;
    // 0x72aa4: cmp r5, #21
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),21)?)?;
    // 0x72aa8: movhi r6, #46
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,46)?)?;
    // 0x72aac: cmp r5, #31
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),31)?)?;
    // 0x72ab0: movhi r6, #28
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,28)?)?;
    // 0x72ab4: b 0x72b34 <entry+0x268>   @ imm = #0x78
    c.branch(C::Al,0x72b34,false)?;
    // 0x72ab8: ldr r2, [pc, #0x100]        @ 0x2f4 <entry+0x2f4>
    c.literal(r(2),0x72bc0)?;
    // 0x72abc: mov r1, #198
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,198)?)?;
    // 0x72ac0: mov r0, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,0)?)?;
    // 0x72ac4: mov r6, #0
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,0)?)?;
    // 0x72ac8: orr r1, r1, #6912
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),6912)?)?;
    // 0x72acc: bl 0x17b0 <entry+0xfffffffffff8eee4> @ imm = #-0x71324
    c.branch(C::Al,0x17b0,true)?;
    // 0x72ad0: b 0x72b34 <entry+0x268>   @ imm = #0x5c
    c.branch(C::Al,0x72b34,false)?;
    // 0x72ad4: mov r6, #32
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,32)?)?;
    // 0x72ad8: cmp r1, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(1),0)?)?;
    // 0x72adc: movne r6, #25
    c.emit(data_immediate(C::Ne,Op::Mov,r(6),R::R0,25)?)?;
    // 0x72ae0: b 0x72b34 <entry+0x268>   @ imm = #0x4c
    c.branch(C::Al,0x72b34,false)?;
    // 0x72ae4: mvn r0, #25344
    c.emit(data_immediate(C::Al,Op::Mvn,r(0),R::R0,25344)?)?;
    // 0x72ae8: mov r1, #255
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,255)?)?;
    // 0x72aec: add r0, r6, r0
    c.emit(data_register(C::Al,Op::Add,r(0),r(6),r(0),Shift::Lsl,0,false)?)?;
    // 0x72af0: orr r1, r1, #75776
    c.emit(data_immediate(C::Al,Op::Orr,r(1),r(1),75776)?)?;
    // 0x72af4: mov r6, #127
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,127)?)?;
    // 0x72af8: cmp r0, r1
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(0),r(1),Shift::Lsl,0,false)?)?;
    // 0x72afc: bhi 0x72b1c <entry+0x250>   @ imm = #0x18
    c.branch(C::Hi,0x72b1c,false)?;
    // 0x72b00: cmp r5, #8
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),8)?)?;
    // 0x72b04: movhi r6, #100
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,100)?)?;
    // 0x72b08: cmp r5, #21
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),21)?)?;
    // 0x72b0c: movhi r6, #56
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,56)?)?;
    // 0x72b10: cmp r5, #31
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),31)?)?;
    // 0x72b14: movhi r6, #36
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,36)?)?;
    // 0x72b18: b 0x72b34 <entry+0x268>   @ imm = #0x14
    c.branch(C::Al,0x72b34,false)?;
    // 0x72b1c: cmp r5, #8
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),8)?)?;
    // 0x72b20: movhi r6, #110
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,110)?)?;
    // 0x72b24: cmp r5, #21
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),21)?)?;
    // 0x72b28: movhi r6, #70
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,70)?)?;
    // 0x72b2c: cmp r5, #31
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),31)?)?;
    // 0x72b30: movhi r6, #50
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,50)?)?;
    // 0x72b34: ldr r0, [r4, #0x64]
    c.emit(transfer_word(C::Al,true,r(0),r(4),100)?)?;
    // 0x72b38: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72b3c: bne 0x72bac <entry+0x2e0>   @ imm = #0x68
    c.branch(C::Ne,0x72bac,false)?;
    // 0x72b40: ldr r0, [r4, #0x68]
    c.emit(transfer_word(C::Al,true,r(0),r(4),104)?)?;
    // 0x72b44: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72b48: bne 0x72bac <entry+0x2e0>   @ imm = #0x5c
    c.branch(C::Ne,0x72bac,false)?;
    // 0x72b4c: ldr r2, [r4]
    c.emit(transfer_word(C::Al,true,r(2),r(4),0)?)?;
    // 0x72b50: mov r3, #35
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,35)?)?;
    // 0x72b54: mov r0, #130
    c.emit(data_immediate(C::Al,Op::Mov,r(0),R::R0,130)?)?;
    // 0x72b58: mov r1, #75
    c.emit(data_immediate(C::Al,Op::Mov,r(1),R::R0,75)?)?;
    // 0x72b5c: mov r6, #32
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,32)?)?;
    // 0x72b60: cmp r2, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),8192)?)?;
    // 0x72b64: moveq r3, #30
    c.emit(data_immediate(C::Eq,Op::Mov,r(3),R::R0,30)?)?;
    // 0x72b68: moveq r0, #100
    c.emit(data_immediate(C::Eq,Op::Mov,r(0),R::R0,100)?)?;
    // 0x72b6c: moveq r1, #60
    c.emit(data_immediate(C::Eq,Op::Mov,r(1),R::R0,60)?)?;
    // 0x72b70: cmp r5, r3
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(3),Shift::Lsl,0,false)?)?;
    // 0x72b74: bls 0x72bac <entry+0x2e0>   @ imm = #0x30
    c.branch(C::Ls,0x72bac,false)?;
    // 0x72b78: mov r3, #50
    c.emit(data_immediate(C::Al,Op::Mov,r(3),R::R0,50)?)?;
    // 0x72b7c: cmp r2, #8192
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(2),8192)?)?;
    // 0x72b80: mov r6, #30
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,30)?)?;
    // 0x72b84: moveq r3, #40
    c.emit(data_immediate(C::Eq,Op::Mov,r(3),R::R0,40)?)?;
    // 0x72b88: cmp r5, r3
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(3),Shift::Lsl,0,false)?)?;
    // 0x72b8c: movhi r6, #28
    c.emit(data_immediate(C::Hi,Op::Mov,r(6),R::R0,28)?)?;
    // 0x72b90: cmphi r5, r1
    c.emit(data_register(C::Hi,Op::Cmp,R::R0,r(5),r(1),Shift::Lsl,0,false)?)?;
    // 0x72b94: bls 0x72bac <entry+0x2e0>   @ imm = #0x10
    c.branch(C::Ls,0x72bac,false)?;
    // 0x72b98: mov r6, #16
    c.emit(data_immediate(C::Al,Op::Mov,r(6),R::R0,16)?)?;
    // 0x72b9c: cmp r5, #151
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(5),151)?)?;
    // 0x72ba0: movlo r6, #24
    c.emit(data_immediate(C::Cc,Op::Mov,r(6),R::R0,24)?)?;
    // 0x72ba4: cmp r5, r0
    c.emit(data_register(C::Al,Op::Cmp,R::R0,r(5),r(0),Shift::Lsl,0,false)?)?;
    // 0x72ba8: movls r6, #26
    c.emit(data_immediate(C::Ls,Op::Mov,r(6),R::R0,26)?)?;
    // 0x72bac: ldrb r0, [r4, #0x7e]
    c.emit(transfer_byte(C::Al,true,r(0),r(4),126)?)?;
    // 0x72bb0: cmp r0, #0
    c.emit(data_immediate(C::Al,Op::Cmp,R::R0,r(0),0)?)?;
    // 0x72bb4: addne r6, r6, #12
    c.emit(data_immediate(C::Ne,Op::Add,r(6),r(6),12)?)?;
    // 0x72bb8: and r0, r6, #255
    c.emit(data_immediate(C::Al,Op::And,r(0),r(6),255)?)?;
    // 0x72bbc: pop {r4, r5, r6, r7, r11, pc}
    c.emit(transfer_multiple(C::Al,true,R::SP,r(4).bit()|r(5).bit()|r(6).bit()|r(7).bit()|r(11).bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;

    c.check(0x72bc0)?;
    c.emit(INVALID_SAMPLE_DIAGNOSTIC)?;
    c.emit(CODEC_TIER_THRESHOLD)?;
    c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();choose_controller_tier(&mut c)?;Ok(c.bytes)}

#[cfg(test)]mod tests{
    use super::*;
    #[test] fn throughput_tier_has_two_structured_literal_words(){
        let out=fragment().unwrap();assert_eq!(out.len(),END-START);
        assert_eq!(&out[out.len()-8..out.len()-4],&INVALID_SAMPLE_DIAGNOSTIC.to_le_bytes());
        assert_eq!(&out[out.len()-4..],&CODEC_TIER_THRESHOLD.to_le_bytes());
    }
}
