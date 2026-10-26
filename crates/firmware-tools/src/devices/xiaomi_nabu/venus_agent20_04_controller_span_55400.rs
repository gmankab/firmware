// SPDX-License-Identifier: GPL-3.0-only
//! Venus codec-controller state and format-field serialization.
//! This emits architectural A32 instructions from typed operands, not
//! extracted vendor instruction-byte arrays. The enclosing firmware
//! function may continue beyond the chunk boundary.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x55400;
pub const END:usize=0x55800;
pub const FILE_OFFSET:usize=0x4000+START;

/// A32 LDRB/STRB with a positive register offset and preindex writeback.
/// The base is updated by the address calculation before the access.
fn transfer_byte_register_writeback(c:C,load:bool,rd:R,base:R,index:R,
                                    shift:Shift,amount:u8)->Result<u32,String>{
 if base==R::PC || index==R::PC || (load && rd==base){
    return Err("invalid byte preindexed writeback operands".into());
 }
 Ok(arm32::transfer_byte_register(c,load,rd,base,index,shift,amount)?|(1<<21))
}

struct Code {at:usize,bytes:Vec<u8>}
impl Code {
 fn new()->Self {Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,address:usize)->Result<(),String>{
    if self.at!=address{return Err(format!("expected 0x{address:x} got 0x{:x}",self.at));}Ok(())
 }
 fn emit(&mut self,word:u32)->Result<(),String>{
    if !(0x40000..0x68000).contains(&self.at)||self.at%4!=0 || self.at>=END{
       return Err("outside Agent20-04 exclusive range".into());
    }
    self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
 }
 fn literal(&mut self,cond:C,rd:R,target:usize)->Result<(),String>{
    let offset=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"literal outside signed range")?;
    self.emit(arm32::transfer_word(cond,true,rd,R::PC,offset)?)
 }
}

/// Update per-plane codec fields, clamp dimensions and copy codec controller
/// configuration from source state into active command/register descriptors.
fn serialize_controller_properties(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x55400: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x55404: moveq r3, #127
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,127)?)?;
    // 0x55408: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5540c: moveq r1, #127
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,127)?)?;
    // 0x55410: strb r3, [r4, #0x408]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1032)?)?;
    // 0x55414: cmp r1, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,127)?)?;
    // 0x55418: movhs r1, r8
    c.emit(arm32::mov_register(C::Cs,R::R1,R::new(8)?))?;
    // 0x5541c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55420: moveq r0, #127
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,127)?)?;
    // 0x55424: strb r1, [r4, #0x406]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1030)?)?;
    // 0x55428: cmp r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,127)?)?;
    // 0x5542c: movhs r0, r8
    c.emit(arm32::mov_register(C::Cs,R::R0,R::new(8)?))?;
    // 0x55430: cmp r3, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,127)?)?;
    // 0x55434: b 0x554c0 <entry+0xc0>    @ imm = #0x84
    c.emit(arm32::branch(C::Al,c.at as u32,0x554c0,false)?)?;
    // 0x55438: ldrb r2, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,988)?)?;
    // 0x5543c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x55440: beq 0x55484 <entry+0x84>    @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55484,false)?)?;
    // 0x55444: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x55448: mov r8, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,63)?)?;
    // 0x5544c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x55450: moveq r3, #63
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,63)?)?;
    // 0x55454: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x55458: moveq r1, #63
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,63)?)?;
    // 0x5545c: strb r3, [r4, #0x408]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1032)?)?;
    // 0x55460: cmp r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,63)?)?;
    // 0x55464: movhs r1, r8
    c.emit(arm32::mov_register(C::Cs,R::R1,R::new(8)?))?;
    // 0x55468: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5546c: moveq r0, #63
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,63)?)?;
    // 0x55470: strb r1, [r4, #0x406]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1030)?)?;
    // 0x55474: cmp r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,63)?)?;
    // 0x55478: movhs r0, r8
    c.emit(arm32::mov_register(C::Cs,R::R0,R::new(8)?))?;
    // 0x5547c: cmp r3, #63
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,63)?)?;
    // 0x55480: b 0x554c0 <entry+0xc0>    @ imm = #0x38
    c.emit(arm32::branch(C::Al,c.at as u32,0x554c0,false)?)?;
    // 0x55484: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x55488: mov r8, #51
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,51)?)?;
    // 0x5548c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x55490: moveq r3, #51
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,51)?)?;
    // 0x55494: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x55498: moveq r1, #51
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,51)?)?;
    // 0x5549c: strb r3, [r4, #0x408]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1032)?)?;
    // 0x554a0: cmp r1, #51
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,51)?)?;
    // 0x554a4: movhs r1, r8
    c.emit(arm32::mov_register(C::Cs,R::R1,R::new(8)?))?;
    // 0x554a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x554ac: moveq r0, #51
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,51)?)?;
    // 0x554b0: strb r1, [r4, #0x406]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1030)?)?;
    // 0x554b4: cmp r0, #51
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,51)?)?;
    // 0x554b8: movhs r0, r8
    c.emit(arm32::mov_register(C::Cs,R::R0,R::new(8)?))?;
    // 0x554bc: cmp r3, #51
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,51)?)?;
    // 0x554c0: strb r0, [r4, #0x407]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1031)?)?;
    // 0x554c4: movlo r2, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x554c8: ldr r7, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,20)?)?;
    // 0x554cc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x554d0: moveq r3, r8
    c.emit(arm32::mov_register(C::Eq,R::new(3)?,R::new(8)?))?;
    // 0x554d4: cmp r9, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(9)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x554d8: movls r1, r9
    c.emit(arm32::mov_register(C::Ls,R::R1,R::new(9)?))?;
    // 0x554dc: cmp r12, r9
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x554e0: strb r3, [r4, #0x408]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(4)?,1032)?)?;
    // 0x554e4: movhi r1, r12
    c.emit(arm32::mov_register(C::Hi,R::R1,R::R12))?;
    // 0x554e8: strh r1, [lr]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::LR,0)?)?;
    // 0x554ec: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // 0x554f0: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x554f4: movls r0, r1
    c.emit(arm32::mov_register(C::Ls,R::R0,R::R1))?;
    // 0x554f8: cmp r10, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(10)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x554fc: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x55500: movhi r0, r10
    c.emit(arm32::mov_register(C::Hi,R::R0,R::new(10)?))?;
    // 0x55504: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x55508: movhs r3, r7
    c.emit(arm32::mov_register(C::Cs,R::new(3)?,R::new(7)?))?;
    // 0x5550c: cmp r5, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x55510: movhi r3, r5
    c.emit(arm32::mov_register(C::Hi,R::new(3)?,R::new(5)?))?;
    // 0x55514: strh r0, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,0)?)?;
    // 0x55518: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x5551c: strh r3, [r0]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R0,0)?)?;
    // 0x55520: ldr r0, [r4, #0xeb0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3760)?)?;
    // 0x55524: str r0, [r4, #0x46c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1132)?)?;
    // 0x55528: ldr r0, [r4, #0xeb4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3764)?)?;
    // 0x5552c: str r0, [r4, #0x470]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1136)?)?;
    // 0x55530: ldr r0, [r4, #0xeb8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3768)?)?;
    // 0x55534: str r0, [r4, #0x474]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1140)?)?;
    // 0x55538: ldr r0, [r4, #0xebc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3772)?)?;
    // 0x5553c: str r0, [r4, #0x478]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1144)?)?;
    // 0x55540: mov r0, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    // 0x55544: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x55548: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x5554c: strb r0, [r4, #0x491]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1169)?)?;
    // 0x55550: ldr r0, [r4, #0xea0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3744)?)?;
    // 0x55554: strb r0, [r4, #0x469]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1129)?)?;
    // 0x55558: ldr r5, [r4, #0xd90]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,3472)?)?;
    // 0x5555c: str r5, [r4, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,744)?)?;
    // 0x55560: ldrb r8, [r4, #0xdf0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(4)?,3568)?)?;
    // 0x55564: strb r8, [r4, #0x348]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(8)?,R::new(4)?,840)?)?;
    // 0x55568: ldr r0, [r4, #0xde8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3560)?)?;
    // 0x5556c: str r0, [r4, #0x340]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,832)?)?;
    // 0x55570: ldr r0, [r4, #0xdec]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3564)?)?;
    // 0x55574: str r0, [r4, #0x344]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,836)?)?;
    // 0x55578: mov r0, #964
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,964)?)?;
    // 0x5557c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x55580: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55584: strb r0, [r4, #0x3f0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1008)?)?;
    // 0x55588: mov r0, #197
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,197)?)?;
    // 0x5558c: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x55590: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55594: strb r0, [r4, #0x3f1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1009)?)?;
    // 0x55598: ldr r0, [r4, #0xab0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2736)?)?;
    // 0x5559c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x555a0: beq 0x555b8 <entry+0x1b8>   @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x555b8,false)?)?;
    // 0x555a4: ldrb r0, [r4, #0xdfb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3579)?)?;
    // 0x555a8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x555ac: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x555b0: strb r0, [r4, #0x3f0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1008)?)?;
    // 0x555b4: strb r0, [r4, #0x3f1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1009)?)?;
    // 0x555b8: ldrb r0, [r4, #0xf75]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3957)?)?;
    // 0x555bc: mov r10, #2560
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,2560)?)?;
    // 0x555c0: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x555c4: orr r10, r10, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,65536)?)?;
    // 0x555c8: strb r0, [r4, #0x482]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1154)?)?;
    // 0x555cc: ldrb r0, [r4, #0xf76]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3958)?)?;
    // 0x555d0: strb r0, [r4, #0x483]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1155)?)?;
    // 0x555d4: ldrb r0, [r4, #0xf71]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3953)?)?;
    // 0x555d8: strb r0, [r4, #0x47e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1150)?)?;
    // 0x555dc: ldrb r0, [r4, #0xf72]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3954)?)?;
    // 0x555e0: strb r0, [r4, #0x47f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1151)?)?;
    // 0x555e4: ldrb r0, [r4, #0xf73]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3955)?)?;
    // 0x555e8: strb r0, [r4, #0x480]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1152)?)?;
    // 0x555ec: ldrb r0, [r4, #0xf74]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3956)?)?;
    // 0x555f0: strb r0, [r4, #0x481]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1153)?)?;
    // 0x555f4: mov r0, #50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,50)?)?;
    // 0x555f8: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x555fc: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55600: strb r0, [r4, #0xa94]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2708)?)?;
    // 0x55604: mov r0, #49
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,49)?)?;
    // 0x55608: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x5560c: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55610: strb r0, [r4, #0xa95]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,2709)?)?;
    // 0x55614: ldr r0, [r4, #0xed0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3792)?)?;
    // 0x55618: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x5561c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x55620: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x55624: strb r0, [r4, #0x339]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,825)?)?;
    // 0x55628: mov r0, #51
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,51)?)?;
    // 0x5562c: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x55630: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55634: strb r0, [r4, #0x492]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1170)?)?;
    // 0x55638: ldrb r0, [r4, #0xe88]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3720)?)?;
    // 0x5563c: strb r0, [r4, #0x3e0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,992)?)?;
    // 0x55640: ldrb r0, [r4, #0xe87]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,3719)?)?;
    // 0x55644: strb r0, [r4, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,991)?)?;
    // 0x55648: ldr r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2732)?)?;
    // 0x5564c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x55650: beq 0x55780 <entry+0x380>   @ imm = #0x128
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55780,false)?)?;
    // 0x55654: mov r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    // 0x55658: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x5565c: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x55660: and r0, r9, r5, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(9)?,R::new(5)?,Shift::Lsr,4,false)?)?;
    // 0x55664: strb r0, [r4, #0x494]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1172)?)?;
    // 0x55668: mov r0, #193
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,193)?)?;
    // 0x5566c: strb r7, [r4, #0x493]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1171)?)?;
    // 0x55670: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x55674: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55678: strb r0, [r4, #0xc85]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3205)?)?;
    // 0x5567c: mov r0, #194
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,194)?)?;
    // 0x55680: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x55684: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55688: strb r0, [r4, #0xc86]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3206)?)?;
    // 0x5568c: mov r0, #195
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,195)?)?;
    // 0x55690: orr r0, r0, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4864)?)?;
    // 0x55694: ldrb r0, [r4, r0]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x55698: strb r0, [r4, #0xc88]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,3208)?)?;
    // 0x5569c: add r0, r4, #140
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,140)?)?;
    // 0x556a0: strb r7, [r4, #0xc87]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,3207)?)?;
    // 0x556a4: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3072)?)?;
    // 0x556a8: bl 0x31ec <entry+0xfffffffffffaddec> @ imm = #-0x524c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
    // 0x556ac: and r0, r9, r5, lsr #5
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(9)?,R::new(5)?,Shift::Lsr,5,false)?)?;
    // 0x556b0: strb r0, [r4, #0x495]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1173)?)?;
    // 0x556b4: and r0, r9, r5, lsr #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(9)?,R::new(5)?,Shift::Lsr,6,false)?)?;
    // 0x556b8: strb r0, [r4, #0x496]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1174)?)?;
    // 0x556bc: ldr r0, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,716)?)?;
    // 0x556c0: cmp r0, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1280)?)?;
    // 0x556c4: movlo r7, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x556c8: strb r7, [r4, #0x497]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(4)?,1175)?)?;
    // 0x556cc: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x556d0: ldrb r1, [r0, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,224)?)?;
    // 0x556d4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x556d8: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x556dc: movne r1, #65536
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,65536)?)?;
    // 0x556e0: str r1, [r4, #0x49c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1180)?)?;
    // 0x556e4: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x556e8: strb r1, [r4, #0x498]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1176)?)?;
    // 0x556ec: and r1, r5, #6
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(5)?,6)?)?;
    // 0x556f0: orr r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1)?)?;
    // 0x556f4: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // 0x556f8: add r1, r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,2)?)?;
    // 0x556fc: moveq r8, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(8)?,R::R0,2)?)?;
    // 0x55700: and r2, r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,7)?)?;
    // 0x55704: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x55708: strbeq r8, [r4, #0x348]
    c.emit(arm32::transfer_byte(C::Eq,false,R::new(8)?,R::new(4)?,840)?)?;
    // 0x5570c: cmp r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // 0x55710: ldrls r1, [pc, #0xfcc]        @ 0x12e4 <entry+0x12e4>
    c.literal(C::Ls,R::R1,0x566e4)?;
    // 0x55714: ldrls r1, [r1, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Ls,true,R::R1,R::R1,R::R2,Shift::Lsl,2)?)?;
    // 0x55718: cmp r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // 0x5571c: str r1, [r4, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,744)?)?;
    // 0x55720: blo 0x55784 <entry+0x384>   @ imm = #0x5c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x55784,false)?)?;
    // 0x55724: ldr r1, [r4, #0xd48]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,3400)?)?;
    // 0x55728: cmp r1, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3840)?)?;
    // 0x5572c: bhi 0x55784 <entry+0x384>   @ imm = #0x50
    c.emit(arm32::branch(C::Hi,c.at as u32,0x55784,false)?)?;
    // 0x55730: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // 0x55734: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x55738: beq 0x55784 <entry+0x384>   @ imm = #0x44
    c.emit(arm32::branch(C::Eq,c.at as u32,0x55784,false)?)?;
    // 0x5573c: mov r1, #221
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,221)?)?;
    // 0x55740: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x55744: ldrb r1, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x55748: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5574c: bne 0x55784 <entry+0x384>   @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x55784,false)?)?;
    // 0x55750: mov r1, #3648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3648)?)?;
    // 0x55754: ldrsb r1, [r4, r1]
    c.emit(super::venus_agent20_04_format_registers::signed_byte_register(C::Al,R::R1,R::new(4)?,R::R1)?)?;
    // 0x55758: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x5575c: bgt 0x55784 <entry+0x384>   @ imm = #0x20
    c.emit(arm32::branch(C::Gt,c.at as u32,0x55784,false)?)?;
    // 0x55760: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x55764: cmp r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // 0x55768: strb r1, [r4, #0xc20]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,3104)?)?;
    // 0x5576c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x55770: str r1, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,40)?)?;
    // 0x55774: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x55778: strbeq r1, [r4, #0x494]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R1,R::new(4)?,1172)?)?;
    // 0x5577c: b 0x55784 <entry+0x384>   @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x55784,false)?)?;
    // 0x55780: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x55784: mov r1, #976
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,976)?)?;
    // 0x55788: ldr r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
    // 0x5578c: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x55790: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x55794: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x55798: ldrb r1, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x5579c: strb r1, [r4, #0x3fc]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1020)?)?;
    // 0x557a0: mov r1, #206
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,206)?)?;
    // 0x557a4: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x557a8: ldrb r1, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x557ac: strb r1, [r4, #0x3fa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1018)?)?;
    // 0x557b0: mov r1, #205
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,205)?)?;
    // 0x557b4: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x557b8: ldrb r1, [r2, r1]!
    c.emit(transfer_byte_register_writeback(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x557bc: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x557c0: strb r1, [r4, #0x3f9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1017)?)?;
    // 0x557c4: mov r1, #207
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,207)?)?;
    // 0x557c8: orr r1, r1, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4864)?)?;
    // 0x557cc: ldrb r1, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x557d0: strb r1, [r4, #0x3fb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1019)?)?;
    // 0x557d4: mov r1, #968
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,968)?)?;
    // 0x557d8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x557dc: ldr r1, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x557e0: str r1, [r4, #0x3f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1012)?)?;
    // 0x557e4: mov r1, #5312
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5312)?)?;
    // 0x557e8: ldrb r1, [r4, r1]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x557ec: strb r1, [r0, #0x49e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1182)?)?;
    // 0x557f0: ldr r0, [r4, #0xe34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3636)?)?;
    // 0x557f4: str r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,908)?)?;
    // 0x557f8: ldr r0, [r4, #0xe38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,3640)?)?;
    // 0x557fc: str r0, [r4, #0x390]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,912)?)?;

 c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();serialize_controller_properties(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn property_chunk_is_contiguous(){
   assert_eq!(fragment().unwrap().len(),END-START);
 }
 #[test]fn checked_byte_register_writeback(){
   assert_eq!(transfer_byte_register_writeback(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,0).unwrap(),0xe7f2_1001);
   assert!(transfer_byte_register_writeback(C::Al,true,R::R2,R::R2,R::R1,Shift::Lsl,0).is_err());
 }
 #[test]fn compare_original_when_provided(){
   if let Ok(path)=std::env::var("AGENT04_PRIVATE_VENUS"){
      let original=std::fs::read(path).unwrap();let bytes=fragment().unwrap();
      assert_eq!(&bytes[..],&original[FILE_OFFSET..FILE_OFFSET+bytes.len()]);
   }
 }
}
