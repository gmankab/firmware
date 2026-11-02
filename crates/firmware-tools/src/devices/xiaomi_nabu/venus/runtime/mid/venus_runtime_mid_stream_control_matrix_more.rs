// SPDX-License-Identifier: GPL-3.0-only
//! Additional codec stream register/descriptor matrix entries. Every typed
//! A32 operation emits an actual instruction; no vendor bytes or unbounded
//! padding are copied into this partial stream initialization routine.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
const START:usize=0x3aa00;
const END:usize=0x3b800;

fn initialize_codec_output_planes(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3aa00: str r5, [sp, #0x420]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1056)?)?;
    // 0x3aa04: ldr r5, [pc, #-0x868]       @ 0x5d4 <entry+0x5d4>
    c.literal_cond(C::Al,R::new(5)?,0x3a1a4)?;
    // 0x3aa08: str r0, [sp, #0x424]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1060)?)?;
    // 0x3aa0c: str r5, [sp, #0x428]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1064)?)?;
    // 0x3aa10: add r5, r6, #110
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,110)?)?;
    // 0x3aa14: str r0, [sp, #0x42c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1068)?)?;
    // 0x3aa18: add r5, r5, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3584)?)?;
    // 0x3aa1c: str r5, [sp, #0x430]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1072)?)?;
    // 0x3aa20: ldr r5, [pc, #-0x880]       @ 0x5d8 <entry+0x5d8>
    c.literal_cond(C::Al,R::new(5)?,0x3a1a8)?;
    // 0x3aa24: str r0, [sp, #0x434]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1076)?)?;
    // 0x3aa28: str r5, [sp, #0x438]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1080)?)?;
    // 0x3aa2c: add r5, r6, #111
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,111)?)?;
    // 0x3aa30: str r0, [sp, #0x43c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1084)?)?;
    // 0x3aa34: add r5, r5, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3584)?)?;
    // 0x3aa38: str r5, [sp, #0x440]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1088)?)?;
    // 0x3aa3c: ldr r5, [pc, #-0x898]       @ 0x5dc <entry+0x5dc>
    c.literal_cond(C::Al,R::new(5)?,0x3a1ac)?;
    // 0x3aa40: str r0, [sp, #0x444]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1092)?)?;
    // 0x3aa44: str r5, [sp, #0x448]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1096)?)?;
    // 0x3aa48: add r5, r6, #3696
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,3696)?)?;
    // 0x3aa4c: str r0, [sp, #0x44c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1100)?)?;
    // 0x3aa50: str r5, [sp, #0x450]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1104)?)?;
    // 0x3aa54: ldr r5, [pc, #-0x8ac]       @ 0x5e0 <entry+0x5e0>
    c.literal_cond(C::Al,R::new(5)?,0x3a1b0)?;
    // 0x3aa58: str r0, [sp, #0x454]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1108)?)?;
    // 0x3aa5c: str r5, [sp, #0x458]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1112)?)?;
    // 0x3aa60: add r5, r6, #113
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,113)?)?;
    // 0x3aa64: str r0, [sp, #0x45c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1116)?)?;
    // 0x3aa68: add r5, r5, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3584)?)?;
    // 0x3aa6c: str r5, [sp, #0x460]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1120)?)?;
    // 0x3aa70: ldr r5, [pc, #-0x8c4]       @ 0x5e4 <entry+0x5e4>
    c.literal_cond(C::Al,R::new(5)?,0x3a1b4)?;
    // 0x3aa74: str r0, [sp, #0x464]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1124)?)?;
    // 0x3aa78: str r5, [sp, #0x468]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1128)?)?;
    // 0x3aa7c: add r5, r6, #114
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,114)?)?;
    // 0x3aa80: str r0, [sp, #0x46c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1132)?)?;
    // 0x3aa84: add r5, r5, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3584)?)?;
    // 0x3aa88: str r5, [sp, #0x470]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1136)?)?;
    // 0x3aa8c: ldr r5, [pc, #-0x8dc]       @ 0x5e8 <entry+0x5e8>
    c.literal_cond(C::Al,R::new(5)?,0x3a1b8)?;
    // 0x3aa90: str r0, [sp, #0x474]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1140)?)?;
    // 0x3aa94: str r5, [sp, #0x478]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1144)?)?;
    // 0x3aa98: add r5, r6, #876
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,876)?)?;
    // 0x3aa9c: str r1, [sp, #0x47c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1148)?)?;
    // 0x3aaa0: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3aaa4: str r5, [sp, #0x480]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1152)?)?;
    // 0x3aaa8: ldr r5, [pc, #-0x8f4]       @ 0x5ec <entry+0x5ec>
    c.literal_cond(C::Al,R::new(5)?,0x3a1bc)?;
    // 0x3aaac: str r0, [sp, #0x484]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1156)?)?;
    // 0x3aab0: str r5, [sp, #0x488]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1160)?)?;
    // 0x3aab4: add r5, r6, #99
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,99)?)?;
    // 0x3aab8: str r0, [sp, #0x48c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1164)?)?;
    // 0x3aabc: add r5, r5, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3584)?)?;
    // 0x3aac0: str r5, [sp, #0x490]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1168)?)?;
    // 0x3aac4: ldr r5, [pc, #-0x90c]       @ 0x5f0 <entry+0x5f0>
    c.literal_cond(C::Al,R::new(5)?,0x3a1c0)?;
    // 0x3aac8: str r0, [sp, #0x494]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1172)?)?;
    // 0x3aacc: str r5, [sp, #0x498]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1176)?)?;
    // 0x3aad0: add r5, r6, #101
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,101)?)?;
    // 0x3aad4: str r0, [sp, #0x49c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1180)?)?;
    // 0x3aad8: add r5, r5, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3584)?)?;
    // 0x3aadc: str r5, [sp, #0x4a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1184)?)?;
    // 0x3aae0: ldr r5, [pc, #-0x924]       @ 0x5f4 <entry+0x5f4>
    c.literal_cond(C::Al,R::new(5)?,0x3a1c4)?;
    // 0x3aae4: str r0, [sp, #0x4a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1188)?)?;
    // 0x3aae8: str r5, [sp, #0x4a8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1192)?)?;
    // 0x3aaec: add r5, r6, #102
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,102)?)?;
    // 0x3aaf0: str r0, [sp, #0x4ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1196)?)?;
    // 0x3aaf4: add r5, r5, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3584)?)?;
    // 0x3aaf8: str r5, [sp, #0x4b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1200)?)?;
    // 0x3aafc: ldr r5, [pc, #-0x93c]       @ 0x5f8 <entry+0x5f8>
    c.literal_cond(C::Al,R::new(5)?,0x3a1c8)?;
    // 0x3ab00: str r0, [sp, #0x4b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1204)?)?;
    // 0x3ab04: str r5, [sp, #0x4b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1208)?)?;
    // 0x3ab08: add r5, r6, #103
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,103)?)?;
    // 0x3ab0c: str r0, [sp, #0x4bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1212)?)?;
    // 0x3ab10: add r5, r5, #3584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3584)?)?;
    // 0x3ab14: str r5, [sp, #0x4c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1216)?)?;
    // 0x3ab18: ldr r5, [pc, #-0x954]       @ 0x5fc <entry+0x5fc>
    c.literal_cond(C::Al,R::new(5)?,0x3a1cc)?;
    // 0x3ab1c: str r0, [sp, #0x4c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1220)?)?;
    // 0x3ab20: str r5, [sp, #0x4c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1224)?)?;
    // 0x3ab24: add r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3ab28: str r1, [sp, #0x4cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1228)?)?;
    // 0x3ab2c: mov r3, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,72)?)?;
    // 0x3ab30: str r5, [sp, #0x4d0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1232)?)?;
    // 0x3ab34: ldr r5, [pc, #-0x96c]       @ 0x600 <entry+0x600>
    c.literal_cond(C::Al,R::new(5)?,0x3a1d0)?;
    // 0x3ab38: str r0, [sp, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1236)?)?;
    // 0x3ab3c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3ab40: str r5, [sp, #0x4d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1240)?)?;
    // 0x3ab44: add r5, r6, #4160
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,4160)?)?;
    // 0x3ab48: str r1, [sp, #0x4dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1244)?)?;
    // 0x3ab4c: str r5, [sp, #0x4e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1248)?)?;
    // 0x3ab50: ldr r5, [pc, #-0x984]       @ 0x604 <entry+0x604>
    c.literal_cond(C::Al,R::new(5)?,0x3a1d4)?;
    // 0x3ab54: str r0, [sp, #0x4e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1252)?)?;
    // 0x3ab58: str r5, [sp, #0x4e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1256)?)?;
    // 0x3ab5c: add r5, r6, #78
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,78)?)?;
    // 0x3ab60: str r0, [sp, #0x4ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1260)?)?;
    // 0x3ab64: add r5, r5, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4864)?)?;
    // 0x3ab68: str r5, [sp, #0x4f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1264)?)?;
    // 0x3ab6c: ldr r5, [pc, #-0x99c]       @ 0x608 <entry+0x608>
    c.literal_cond(C::Al,R::new(5)?,0x3a1d8)?;
    // 0x3ab70: str r0, [sp, #0x4f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1268)?)?;
    // 0x3ab74: str r5, [sp, #0x4f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1272)?)?;
    // 0x3ab78: add r5, r6, #3952
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,3952)?)?;
    // 0x3ab7c: str r0, [sp, #0x4fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1276)?)?;
    // 0x3ab80: str r5, [sp, #0x500]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1280)?)?;
    // 0x3ab84: ldr r5, [pc, #-0x9b0]       @ 0x60c <entry+0x60c>
    c.literal_cond(C::Al,R::new(5)?,0x3a1dc)?;
    // 0x3ab88: str r0, [sp, #0x504]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1284)?)?;
    // 0x3ab8c: str r5, [sp, #0x508]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1288)?)?;
    // 0x3ab90: add r5, r6, #117
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,117)?)?;
    // 0x3ab94: str r0, [sp, #0x50c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1292)?)?;
    // 0x3ab98: add r5, r5, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3840)?)?;
    // 0x3ab9c: str r5, [sp, #0x510]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1296)?)?;
    // 0x3aba0: ldr r5, [pc, #-0x9c8]       @ 0x610 <entry+0x610>
    c.literal_cond(C::Al,R::new(5)?,0x3a1e0)?;
    // 0x3aba4: str r0, [sp, #0x514]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1300)?)?;
    // 0x3aba8: str r5, [sp, #0x518]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1304)?)?;
    // 0x3abac: add r5, r6, #118
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,118)?)?;
    // 0x3abb0: str r0, [sp, #0x51c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1308)?)?;
    // 0x3abb4: add r5, r5, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3840)?)?;
    // 0x3abb8: str r5, [sp, #0x520]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1312)?)?;
    // 0x3abbc: ldr r5, [pc, #-0x9e0]       @ 0x614 <entry+0x614>
    c.literal_cond(C::Al,R::new(5)?,0x3a1e4)?;
    // 0x3abc0: str r0, [sp, #0x524]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1316)?)?;
    // 0x3abc4: str r5, [sp, #0x528]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1320)?)?;
    // 0x3abc8: add r5, r6, #900
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,900)?)?;
    // 0x3abcc: str r1, [sp, #0x52c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1324)?)?;
    // 0x3abd0: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3abd4: str r5, [sp, #0x530]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1328)?)?;
    // 0x3abd8: mov r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,8)?)?;
    // 0x3abdc: str r5, [sp, #0x534]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1332)?)?;
    // 0x3abe0: ldr r5, [pc, #-0xa00]       @ 0x618 <entry+0x618>
    c.literal_cond(C::Al,R::new(5)?,0x3a1e8)?;
    // 0x3abe4: str r5, [sp, #0x538]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1336)?)?;
    // 0x3abe8: add r5, r6, #680
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,680)?)?;
    // 0x3abec: str r1, [sp, #0x53c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1340)?)?;
    // 0x3abf0: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3abf4: str r5, [sp, #0x540]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1344)?)?;
    // 0x3abf8: ldr r5, [pc, #-0xa14]       @ 0x61c <entry+0x61c>
    c.literal_cond(C::Al,R::new(5)?,0x3a1ec)?;
    // 0x3abfc: str r0, [sp, #0x544]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1348)?)?;
    // 0x3ac00: str r5, [sp, #0x548]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1352)?)?;
    // 0x3ac04: add r5, r6, #844
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,844)?)?;
    // 0x3ac08: str r1, [sp, #0x54c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1356)?)?;
    // 0x3ac0c: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3ac10: str r5, [sp, #0x550]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1360)?)?;
    // 0x3ac14: ldr r5, [pc, #-0xa2c]       @ 0x620 <entry+0x620>
    c.literal_cond(C::Al,R::new(5)?,0x3a1f0)?;
    // 0x3ac18: str r0, [sp, #0x554]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1364)?)?;
    // 0x3ac1c: str r5, [sp, #0x558]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1368)?)?;
    // 0x3ac20: add r5, r6, #3920
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,3920)?)?;
    // 0x3ac24: str r1, [sp, #0x55c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1372)?)?;
    // 0x3ac28: str r5, [sp, #0x560]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1376)?)?;
    // 0x3ac2c: ldr r5, [pc, #-0xa40]       @ 0x624 <entry+0x624>
    c.literal_cond(C::Al,R::new(5)?,0x3a1f4)?;
    // 0x3ac30: str r0, [sp, #0x564]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1380)?)?;
    // 0x3ac34: str r5, [sp, #0x568]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1384)?)?;
    // 0x3ac38: add r5, r6, #956
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,956)?)?;
    // 0x3ac3c: str r1, [sp, #0x56c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1388)?)?;
    // 0x3ac40: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3ac44: str r5, [sp, #0x570]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1392)?)?;
    // 0x3ac48: ldr r5, [pc, #-0xa58]       @ 0x628 <entry+0x628>
    c.literal_cond(C::Al,R::new(5)?,0x3a1f8)?;
    // 0x3ac4c: str r0, [sp, #0x574]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1396)?)?;
    // 0x3ac50: str r5, [sp, #0x578]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1400)?)?;
    // 0x3ac54: add r5, r6, #4032
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,4032)?)?;
    // 0x3ac58: str r1, [sp, #0x57c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1404)?)?;
    // 0x3ac5c: str r5, [sp, #0x580]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1408)?)?;
    // 0x3ac60: ldr r5, [pc, #-0xa6c]       @ 0x62c <entry+0x62c>
    c.literal_cond(C::Al,R::new(5)?,0x3a1fc)?;
    // 0x3ac64: str r0, [sp, #0x584]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1412)?)?;
    // 0x3ac68: str r5, [sp, #0x588]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1416)?)?;
    // 0x3ac6c: add r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3ac70: str r1, [sp, #0x58c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1420)?)?;
    // 0x3ac74: mov r3, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,48)?)?;
    // 0x3ac78: str r5, [sp, #0x590]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1424)?)?;
    // 0x3ac7c: ldr r5, [pc, #-0xa84]       @ 0x630 <entry+0x630>
    c.literal_cond(C::Al,R::new(5)?,0x3a200)?;
    // 0x3ac80: str r0, [sp, #0x594]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1428)?)?;
    // 0x3ac84: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3ac88: str r5, [sp, #0x598]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1432)?)?;
    // 0x3ac8c: add r5, r6, #3760
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,3760)?)?;
    // 0x3ac90: str r1, [sp, #0x59c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1436)?)?;
    // 0x3ac94: str r5, [sp, #0x5a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1440)?)?;
    // 0x3ac98: ldr r5, [pc, #-0xa9c]       @ 0x634 <entry+0x634>
    c.literal_cond(C::Al,R::new(5)?,0x3a204)?;
    // 0x3ac9c: str r0, [sp, #0x5a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1444)?)?;
    // 0x3aca0: str r5, [sp, #0x5a8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1448)?)?;
    // 0x3aca4: add r5, r6, #692
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,692)?)?;
    // 0x3aca8: str r1, [sp, #0x5ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1452)?)?;
    // 0x3acac: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3acb0: str r5, [sp, #0x5b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1456)?)?;
    // 0x3acb4: ldr r5, [pc, #-0xab4]       @ 0x638 <entry+0x638>
    c.literal_cond(C::Al,R::new(5)?,0x3a208)?;
    // 0x3acb8: str r0, [sp, #0x5b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1460)?)?;
    // 0x3acbc: str r5, [sp, #0x5b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1464)?)?;
    // 0x3acc0: add r5, r6, #696
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,696)?)?;
    // 0x3acc4: str r1, [sp, #0x5bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1468)?)?;
    // 0x3acc8: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3accc: str r5, [sp, #0x5c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1472)?)?;
    // 0x3acd0: ldr r5, [pc, #-0xacc]       @ 0x63c <entry+0x63c>
    c.literal_cond(C::Al,R::new(5)?,0x3a20c)?;
    // 0x3acd4: str r0, [sp, #0x5c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1476)?)?;
    // 0x3acd8: str r5, [sp, #0x5c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1480)?)?;
    // 0x3acdc: add r5, r6, #700
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,700)?)?;
    // 0x3ace0: str r1, [sp, #0x5cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1484)?)?;
    // 0x3ace4: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3ace8: str r5, [sp, #0x5d0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1488)?)?;
    // 0x3acec: ldr r5, [pc, #-0xae4]       @ 0x640 <entry+0x640>
    c.literal_cond(C::Al,R::new(5)?,0x3a210)?;
    // 0x3acf0: str r0, [sp, #0x5d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1492)?)?;
    // 0x3acf4: str r5, [sp, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1496)?)?;
    // 0x3acf8: add r5, r6, #3776
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,3776)?)?;
    // 0x3acfc: str r1, [sp, #0x5dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1500)?)?;
    // 0x3ad00: str r5, [sp, #0x5e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1504)?)?;
    // 0x3ad04: ldr r5, [pc, #-0xaf8]       @ 0x644 <entry+0x644>
    c.literal_cond(C::Al,R::new(5)?,0x3a214)?;
    // 0x3ad08: str r0, [sp, #0x5e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1508)?)?;
    // 0x3ad0c: str r5, [sp, #0x5e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1512)?)?;
    // 0x3ad10: add r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3ad14: str r1, [sp, #0x5ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1516)?)?;
    // 0x3ad18: mov r3, #572
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,572)?)?;
    // 0x3ad1c: str r5, [sp, #0x5f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1520)?)?;
    // 0x3ad20: ldr r5, [pc, #-0xb10]       @ 0x648 <entry+0x648>
    c.literal_cond(C::Al,R::new(5)?,0x3a218)?;
    // 0x3ad24: str r0, [sp, #0x5f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1524)?)?;
    // 0x3ad28: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3ad2c: str r5, [sp, #0x5f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1528)?)?;
    // 0x3ad30: add r5, r6, #50
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,50)?)?;
    // 0x3ad34: str r1, [sp, #0x5fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1532)?)?;
    // 0x3ad38: add r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x3ad3c: str r5, [sp, #0x600]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1536)?)?;
    // 0x3ad40: ldr r5, [pc, #-0xb2c]       @ 0x64c <entry+0x64c>
    c.literal_cond(C::Al,R::new(5)?,0x3a21c)?;
    // 0x3ad44: str r0, [sp, #0x604]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1540)?)?;
    // 0x3ad48: str r5, [sp, #0x608]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1544)?)?;
    // 0x3ad4c: add r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3ad50: str r1, [sp, #0x60c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1548)?)?;
    // 0x3ad54: mov r3, #816
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,816)?)?;
    // 0x3ad58: str r5, [sp, #0x610]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1552)?)?;
    // 0x3ad5c: ldr r5, [pc, #-0xb44]       @ 0x650 <entry+0x650>
    c.literal_cond(C::Al,R::new(5)?,0x3a220)?;
    // 0x3ad60: str r0, [sp, #0x614]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1556)?)?;
    // 0x3ad64: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3ad68: str r5, [sp, #0x618]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1560)?)?;
    // 0x3ad6c: add r5, r6, #756
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,756)?)?;
    // 0x3ad70: str r1, [sp, #0x61c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1564)?)?;
    // 0x3ad74: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3ad78: str r5, [sp, #0x620]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1568)?)?;
    // 0x3ad7c: ldr r5, [pc, #-0xb60]       @ 0x654 <entry+0x654>
    c.literal_cond(C::Al,R::new(5)?,0x3a224)?;
    // 0x3ad80: str r0, [sp, #0x624]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1572)?)?;
    // 0x3ad84: str r5, [sp, #0x628]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1576)?)?;
    // 0x3ad88: add r5, r6, #836
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,836)?)?;
    // 0x3ad8c: str r1, [sp, #0x62c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1580)?)?;
    // 0x3ad90: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3ad94: str r5, [sp, #0x630]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1584)?)?;
    // 0x3ad98: ldr r5, [pc, #-0xb78]       @ 0x658 <entry+0x658>
    c.literal_cond(C::Al,R::new(5)?,0x3a228)?;
    // 0x3ad9c: str r0, [sp, #0x634]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1588)?)?;
    // 0x3ada0: str r5, [sp, #0x638]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1592)?)?;
    // 0x3ada4: add r5, r6, #840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,840)?)?;
    // 0x3ada8: str r1, [sp, #0x63c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1596)?)?;
    // 0x3adac: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3adb0: str r5, [sp, #0x640]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1600)?)?;
    // 0x3adb4: ldr r5, [pc, #-0xb90]       @ 0x65c <entry+0x65c>
    c.literal_cond(C::Al,R::new(5)?,0x3a22c)?;
    // 0x3adb8: str r0, [sp, #0x644]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1604)?)?;
    // 0x3adbc: str r5, [sp, #0x648]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1608)?)?;
    // 0x3adc0: add r5, r6, #456
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,456)?)?;
    // 0x3adc4: str r1, [sp, #0x64c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1612)?)?;
    // 0x3adc8: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3adcc: str r5, [sp, #0x650]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1616)?)?;
    // 0x3add0: ldr r5, [pc, #-0xba8]       @ 0x660 <entry+0x660>
    c.literal_cond(C::Al,R::new(5)?,0x3a230)?;
    // 0x3add4: str r0, [sp, #0x654]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1620)?)?;
    // 0x3add8: str r5, [sp, #0x658]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1624)?)?;
    // 0x3addc: add r5, r6, #460
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,460)?)?;
    // 0x3ade0: str r1, [sp, #0x65c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1628)?)?;
    // 0x3ade4: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3ade8: str r5, [sp, #0x660]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1632)?)?;
    // 0x3adec: ldr r5, [pc, #-0xbc0]       @ 0x664 <entry+0x664>
    c.literal_cond(C::Al,R::new(5)?,0x3a234)?;
    // 0x3adf0: str r0, [sp, #0x664]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1636)?)?;
    // 0x3adf4: str r5, [sp, #0x668]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1640)?)?;
    // 0x3adf8: add r5, r6, #209
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,209)?)?;
    // 0x3adfc: str r1, [sp, #0x66c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1644)?)?;
    // 0x3ae00: add r5, r5, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3328)?)?;
    // 0x3ae04: str r5, [sp, #0x670]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1648)?)?;
    // 0x3ae08: ldr r5, [pc, #-0xbd8]       @ 0x668 <entry+0x668>
    c.literal_cond(C::Al,R::new(5)?,0x3a238)?;
    // 0x3ae0c: str r0, [sp, #0x674]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1652)?)?;
    // 0x3ae10: str r5, [sp, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1656)?)?;
    // 0x3ae14: add r5, r6, #226
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,226)?)?;
    // 0x3ae18: str r0, [sp, #0x67c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1660)?)?;
    // 0x3ae1c: add r5, r5, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3328)?)?;
    // 0x3ae20: str r5, [sp, #0x680]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1664)?)?;
    // 0x3ae24: ldr r5, [pc, #-0xbf0]       @ 0x66c <entry+0x66c>
    c.literal_cond(C::Al,R::new(5)?,0x3a23c)?;
    // 0x3ae28: str r0, [sp, #0x684]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1668)?)?;
    // 0x3ae2c: str r5, [sp, #0x688]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1672)?)?;
    // 0x3ae30: add r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3ae34: str r0, [sp, #0x68c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1676)?)?;
    // 0x3ae38: mov r3, #864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,864)?)?;
    // 0x3ae3c: str r5, [sp, #0x690]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1680)?)?;
    // 0x3ae40: ldr r5, [pc, #-0xc08]       @ 0x670 <entry+0x670>
    c.literal_cond(C::Al,R::new(5)?,0x3a240)?;
    // 0x3ae44: str r0, [sp, #0x694]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1684)?)?;
    // 0x3ae48: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3ae4c: str r5, [sp, #0x698]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1688)?)?;
    // 0x3ae50: add r5, r6, #227
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,227)?)?;
    // 0x3ae54: str r0, [sp, #0x69c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1692)?)?;
    // 0x3ae58: add r5, r5, #3328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3328)?)?;
    // 0x3ae5c: str r5, [sp, #0x6a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1696)?)?;
    // 0x3ae60: ldr r5, [pc, #-0xc24]       @ 0x674 <entry+0x674>
    c.literal_cond(C::Al,R::new(5)?,0x3a244)?;
    // 0x3ae64: str r0, [sp, #0x6a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1700)?)?;
    // 0x3ae68: str r5, [sp, #0x6a8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1704)?)?;
    // 0x3ae6c: add r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3ae70: str r1, [sp, #0x6ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1708)?)?;
    // 0x3ae74: mov r3, #884
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,884)?)?;
    // 0x3ae78: str r5, [sp, #0x6b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1712)?)?;
    // 0x3ae7c: mov r5, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,5)?)?;
    // 0x3ae80: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3ae84: str r5, [sp, #0x6b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1716)?)?;
    // 0x3ae88: ldr r5, [pc, #-0xc48]       @ 0x678 <entry+0x678>
    c.literal_cond(C::Al,R::new(5)?,0x3a248)?;
    // 0x3ae8c: str r5, [sp, #0x6b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1720)?)?;
    // 0x3ae90: add r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3ae94: str r1, [sp, #0x6bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1724)?)?;
    // 0x3ae98: mov r3, #888
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,888)?)?;
    // 0x3ae9c: str r5, [sp, #0x6c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1728)?)?;
    // 0x3aea0: ldr r5, [pc, #-0xc5c]       @ 0x67c <entry+0x67c>
    c.literal_cond(C::Al,R::new(5)?,0x3a24c)?;
    // 0x3aea4: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3aea8: str r0, [sp, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1732)?)?;
    // 0x3aeac: str r5, [sp, #0x6c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1736)?)?;
    // 0x3aeb0: add r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3aeb4: str r0, [sp, #0x6cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1740)?)?;
    // 0x3aeb8: mov r3, #892
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,892)?)?;
    // 0x3aebc: str r5, [sp, #0x6d0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1744)?)?;
    // 0x3aec0: ldr r5, [pc, #-0xc78]       @ 0x680 <entry+0x680>
    c.literal_cond(C::Al,R::new(5)?,0x3a250)?;
    // 0x3aec4: str r0, [sp, #0x6d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1748)?)?;
    // 0x3aec8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3aecc: str r5, [sp, #0x6d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1752)?)?;
    // 0x3aed0: add r5, r6, #122
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,122)?)?;
    // 0x3aed4: str r2, [sp, #0x6dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,1756)?)?;
    // 0x3aed8: add r5, r5, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4864)?)?;
    // 0x3aedc: str r5, [sp, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1760)?)?;
    // 0x3aee0: ldr r5, [pc, #-0xc94]       @ 0x684 <entry+0x684>
    c.literal_cond(C::Al,R::new(5)?,0x3a254)?;
    // 0x3aee4: str r0, [sp, #0x6e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1764)?)?;
    // 0x3aee8: str r5, [sp, #0x6e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1768)?)?;
    // 0x3aeec: add r5, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3aef0: str r0, [sp, #0x6ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1772)?)?;
    // 0x3aef4: ldr r3, [pc, #-0xca4]       @ 0x688 <entry+0x688>
    c.literal_cond(C::Al,R::new(3)?,0x3a258)?;
    // 0x3aef8: str r5, [sp, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1776)?)?;
    // 0x3aefc: ldr r5, [pc, #-0xca8]       @ 0x68c <entry+0x68c>
    c.literal_cond(C::Al,R::new(5)?,0x3a25c)?;
    // 0x3af00: str r1, [sp, #0x6f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1780)?)?;
    // 0x3af04: str r5, [sp, #0x6f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1784)?)?;
    // 0x3af08: add r5, r6, #49
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,49)?)?;
    // 0x3af0c: str r0, [sp, #0x6fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1788)?)?;
    // 0x3af10: add r5, r5, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4864)?)?;
    // 0x3af14: str r5, [sp, #0x700]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1792)?)?;
    // 0x3af18: ldr r5, [pc, #-0xcc0]       @ 0x690 <entry+0x690>
    c.literal_cond(C::Al,R::new(5)?,0x3a260)?;
    // 0x3af1c: str r0, [sp, #0x704]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1796)?)?;
    // 0x3af20: str r5, [sp, #0x708]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1800)?)?;
    // 0x3af24: add r5, r6, #50
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,50)?)?;
    // 0x3af28: str r0, [sp, #0x70c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1804)?)?;
    // 0x3af2c: add r5, r5, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4864)?)?;
    // 0x3af30: str r5, [sp, #0x710]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1808)?)?;
    // 0x3af34: ldr r5, [pc, #-0xcd8]       @ 0x694 <entry+0x694>
    c.literal_cond(C::Al,R::new(5)?,0x3a264)?;
    // 0x3af38: str r0, [sp, #0x714]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1812)?)?;
    // 0x3af3c: str r5, [sp, #0x718]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1816)?)?;
    // 0x3af40: add r5, r6, #51
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,51)?)?;
    // 0x3af44: str r0, [sp, #0x71c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1820)?)?;
    // 0x3af48: add r5, r5, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4864)?)?;
    // 0x3af4c: str r5, [sp, #0x720]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1824)?)?;
    // 0x3af50: ldr r5, [pc, #-0xcf0]       @ 0x698 <entry+0x698>
    c.literal_cond(C::Al,R::new(5)?,0x3a268)?;
    // 0x3af54: str r0, [sp, #0x724]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1828)?)?;
    // 0x3af58: str r5, [sp, #0x728]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1832)?)?;
    // 0x3af5c: add r5, r6, #3680
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,3680)?)?;
    // 0x3af60: str r0, [sp, #0x72c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1836)?)?;
    // 0x3af64: str r5, [sp, #0x730]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1840)?)?;
    // 0x3af68: ldr r5, [pc, #-0xd04]       @ 0x69c <entry+0x69c>
    c.literal_cond(C::Al,R::new(5)?,0x3a26c)?;
    // 0x3af6c: str r0, [sp, #0x734]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1844)?)?;
    // 0x3af70: str r5, [sp, #0x738]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1848)?)?;
    // 0x3af74: add r5, r6, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,1004)?)?;
    // 0x3af78: str r0, [sp, #0x73c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1852)?)?;
    // 0x3af7c: add r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x3af80: str r5, [sp, #0x740]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1856)?)?;
    // 0x3af84: ldr r5, [pc, #-0xd1c]       @ 0x6a0 <entry+0x6a0>
    c.literal_cond(C::Al,R::new(5)?,0x3a270)?;
    // 0x3af88: str r0, [sp, #0x744]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1860)?)?;
    // 0x3af8c: str r5, [sp, #0x748]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1864)?)?;
    // 0x3af90: add r5, r6, #214
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,214)?)?;
    // 0x3af94: str r0, [sp, #0x74c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1868)?)?;
    // 0x3af98: add r5, r5, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3840)?)?;
    // 0x3af9c: str r5, [sp, #0x750]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1872)?)?;
    // 0x3afa0: ldr r5, [pc, #-0xd34]       @ 0x6a4 <entry+0x6a4>
    c.literal_cond(C::Al,R::new(5)?,0x3a274)?;
    // 0x3afa4: str r0, [sp, #0x754]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1876)?)?;
    // 0x3afa8: str r5, [sp, #0x758]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1880)?)?;
    // 0x3afac: add r5, r6, #221
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,221)?)?;
    // 0x3afb0: str r0, [sp, #0x75c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1884)?)?;
    // 0x3afb4: add r5, r5, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3840)?)?;
    // 0x3afb8: str r5, [sp, #0x760]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1888)?)?;
    // 0x3afbc: ldr r5, [pc, #-0xd4c]       @ 0x6a8 <entry+0x6a8>
    c.literal_cond(C::Al,R::new(5)?,0x3a278)?;
    // 0x3afc0: str r0, [sp, #0x764]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1892)?)?;
    // 0x3afc4: str r5, [sp, #0x768]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1896)?)?;
    // 0x3afc8: add r5, r6, #238
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,238)?)?;
    // 0x3afcc: str r0, [sp, #0x76c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1900)?)?;
    // 0x3afd0: add r5, r5, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3840)?)?;
    // 0x3afd4: str r5, [sp, #0x770]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1904)?)?;
    // 0x3afd8: ldr r5, [pc, #-0xd64]       @ 0x6ac <entry+0x6ac>
    c.literal_cond(C::Al,R::new(5)?,0x3a27c)?;
    // 0x3afdc: str r0, [sp, #0x774]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1908)?)?;
    // 0x3afe0: str r5, [sp, #0x778]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1912)?)?;
    // 0x3afe4: add r5, r6, #4080
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,4080)?)?;
    // 0x3afe8: str r0, [sp, #0x77c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1916)?)?;
    // 0x3afec: str r5, [sp, #0x780]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1920)?)?;
    // 0x3aff0: ldr r5, [pc, #-0xd78]       @ 0x6b0 <entry+0x6b0>
    c.literal_cond(C::Al,R::new(5)?,0x3a280)?;
    // 0x3aff4: str r0, [sp, #0x784]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1924)?)?;
    // 0x3aff8: str r5, [sp, #0x788]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1928)?)?;
    // 0x3affc: add r5, r6, #219
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,219)?)?;
    // 0x3b000: str r0, [sp, #0x78c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1932)?)?;
    // 0x3b004: add r5, r5, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3840)?)?;
    // 0x3b008: str r5, [sp, #0x790]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1936)?)?;
    // 0x3b00c: ldr r5, [pc, #-0xd90]       @ 0x6b4 <entry+0x6b4>
    c.literal_cond(C::Al,R::new(5)?,0x3a284)?;
    // 0x3b010: str r0, [sp, #0x794]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,1940)?)?;
    // 0x3b014: str r5, [sp, #0x798]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1944)?)?;
    // 0x3b018: add r5, r6, #250
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,250)?)?;
    // 0x3b01c: str r2, [sp, #0x79c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,1948)?)?;
    // 0x3b020: add r5, r5, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,3840)?)?;
    // 0x3b024: str r5, [sp, #0x7a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1952)?)?;
    // 0x3b028: mov r5, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,9)?)?;
    // 0x3b02c: str r5, [sp, #0x7a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1956)?)?;
    // 0x3b030: ldr r5, [pc, #-0xdb0]       @ 0x6b8 <entry+0x6b8>
    c.literal_cond(C::Al,R::new(5)?,0x3a288)?;
    // 0x3b034: str r5, [sp, #0x7a8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1960)?)?;
    // 0x3b038: add r5, r6, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3b03c: str r2, [sp, #0x7ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,1964)?)?;
    // 0x3b040: str r5, [sp, #0x7b0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1968)?)?;
    // 0x3b044: ldr r5, [pc, #-0xdc0]       @ 0x6bc <entry+0x6bc>
    c.literal_cond(C::Al,R::new(5)?,0x3a28c)?;
    // 0x3b048: str r1, [sp, #0x7b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,1972)?)?;
    // 0x3b04c: str r5, [sp, #0x7b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1976)?)?;
    // 0x3b050: add r5, r6, #18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,18)?)?;
    // 0x3b054: str r2, [sp, #0x7bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,1980)?)?;
    // 0x3b058: add r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x3b05c: str r5, [sp, #0x7c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,1984)?)?;
    // 0x3b060: str lr, [sp, #0x7c4]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,1988)?)?;
    // 0x3b064: str r3, [sp, #0x7c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,1992)?)?;
    // 0x3b068: str r2, [sp, #0x7cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,1996)?)?;
    // 0x3b06c: add r2, r6, #30
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,30)?)?;
    // 0x3b070: ldr r3, [pc, #-0xde8]       @ 0x6c0 <entry+0x6c0>
    c.literal_cond(C::Al,R::new(3)?,0x3a290)?;
    // 0x3b074: add lr, sp, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::SP,4096)?)?;
    // 0x3b078: mov r5, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,96)?)?;
    // 0x3b07c: add r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4096)?)?;
    // 0x3b080: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x3b084: str r2, [sp, #0x7d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2000)?)?;
    // 0x3b088: ldr r2, [pc, #-0xdfc]       @ 0x6c4 <entry+0x6c4>
    c.literal_cond(C::Al,R::R2,0x3a294)?;
    // 0x3b08c: str r7, [sp, #0x7d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,2004)?)?;
    // 0x3b090: mov r7, #224
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,224)?)?;
    // 0x3b094: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x3b098: str r2, [sp, #0x7d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2008)?)?;
    // 0x3b09c: add r2, r6, #233
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,233)?)?;
    // 0x3b0a0: str r0, [sp, #0x7dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2012)?)?;
    // 0x3b0a4: add r2, r2, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3840)?)?;
    // 0x3b0a8: str r2, [sp, #0x7e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2016)?)?;
    // 0x3b0ac: ldr r2, [pc, #-0xe1c]       @ 0x6c8 <entry+0x6c8>
    c.literal_cond(C::Al,R::R2,0x3a298)?;
    // 0x3b0b0: str r0, [sp, #0x7e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2020)?)?;
    // 0x3b0b4: str r2, [sp, #0x7e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2024)?)?;
    // 0x3b0b8: add r2, r6, #234
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,234)?)?;
    // 0x3b0bc: str r0, [sp, #0x7ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2028)?)?;
    // 0x3b0c0: add r2, r2, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3840)?)?;
    // 0x3b0c4: str r2, [sp, #0x7f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2032)?)?;
    // 0x3b0c8: ldr r2, [pc, #-0xe34]       @ 0x6cc <entry+0x6cc>
    c.literal_cond(C::Al,R::R2,0x3a29c)?;
    // 0x3b0cc: str r0, [sp, #0x7f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2036)?)?;
    // 0x3b0d0: str r2, [sp, #0x7f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2040)?)?;
    // 0x3b0d4: add r2, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3b0d8: str r1, [sp, #0x7fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2044)?)?;
    // 0x3b0dc: str r2, [sp, #0x800]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2048)?)?;
    // 0x3b0e0: ldr r2, [pc, #-0xe48]       @ 0x6d0 <entry+0x6d0>
    c.literal_cond(C::Al,R::R2,0x3a2a0)?;
    // 0x3b0e4: str r0, [sp, #0x804]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2052)?)?;
    // 0x3b0e8: str r2, [sp, #0x808]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2056)?)?;
    // 0x3b0ec: add r2, r6, #728
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,728)?)?;
    // 0x3b0f0: str r1, [sp, #0x80c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2060)?)?;
    // 0x3b0f4: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3b0f8: str r2, [sp, #0x810]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2064)?)?;
    // 0x3b0fc: ldr r2, [pc, #-0xe60]       @ 0x6d4 <entry+0x6d4>
    c.literal_cond(C::Al,R::R2,0x3a2a4)?;
    // 0x3b100: str r0, [sp, #0x814]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2068)?)?;
    // 0x3b104: str r2, [sp, #0x818]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2072)?)?;
    // 0x3b108: add r2, r6, #732
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,732)?)?;
    // 0x3b10c: str r1, [sp, #0x81c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2076)?)?;
    // 0x3b110: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3b114: str r2, [sp, #0x820]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2080)?)?;
    // 0x3b118: ldr r2, [pc, #-0xe78]       @ 0x6d8 <entry+0x6d8>
    c.literal_cond(C::Al,R::R2,0x3a2a8)?;
    // 0x3b11c: str r0, [sp, #0x824]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2084)?)?;
    // 0x3b120: str r2, [sp, #0x828]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2088)?)?;
    // 0x3b124: add r2, r6, #3808
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3808)?)?;
    // 0x3b128: str r1, [sp, #0x82c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2092)?)?;
    // 0x3b12c: str r2, [sp, #0x830]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2096)?)?;
    // 0x3b130: ldr r2, [pc, #-0xe8c]       @ 0x6dc <entry+0x6dc>
    c.literal_cond(C::Al,R::R2,0x3a2ac)?;
    // 0x3b134: str r0, [sp, #0x834]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2100)?)?;
    // 0x3b138: str r2, [sp, #0x838]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2104)?)?;
    // 0x3b13c: add r2, r6, #740
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,740)?)?;
    // 0x3b140: str r1, [sp, #0x83c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2108)?)?;
    // 0x3b144: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3b148: str r2, [sp, #0x840]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2112)?)?;
    // 0x3b14c: ldr r2, [pc, #-0xea4]       @ 0x6e0 <entry+0x6e0>
    c.literal_cond(C::Al,R::R2,0x3a2b0)?;
    // 0x3b150: str r0, [sp, #0x844]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2116)?)?;
    // 0x3b154: str r2, [sp, #0x848]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2120)?)?;
    // 0x3b158: add r2, r6, #748
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,748)?)?;
    // 0x3b15c: str r1, [sp, #0x84c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2124)?)?;
    // 0x3b160: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3b164: str r2, [sp, #0x850]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2128)?)?;
    // 0x3b168: ldr r2, [pc, #-0xebc]       @ 0x6e4 <entry+0x6e4>
    c.literal_cond(C::Al,R::R2,0x3a2b4)?;
    // 0x3b16c: str r0, [sp, #0x854]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2132)?)?;
    // 0x3b170: str r2, [sp, #0x858]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2136)?)?;
    // 0x3b174: add r2, r6, #744
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,744)?)?;
    // 0x3b178: str r1, [sp, #0x85c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2140)?)?;
    // 0x3b17c: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3b180: str r2, [sp, #0x860]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2144)?)?;
    // 0x3b184: ldr r2, [pc, #-0xed4]       @ 0x6e8 <entry+0x6e8>
    c.literal_cond(C::Al,R::R2,0x3a2b8)?;
    // 0x3b188: str r0, [sp, #0x864]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2148)?)?;
    // 0x3b18c: str r2, [sp, #0x868]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2152)?)?;
    // 0x3b190: add r2, r6, #77
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,77)?)?;
    // 0x3b194: str r4, [sp, #0x86c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,2156)?)?;
    // 0x3b198: mov r4, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,104)?)?;
    // 0x3b19c: add r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4864)?)?;
    // 0x3b1a0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3b1a4: str r2, [sp, #0x870]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2160)?)?;
    // 0x3b1a8: ldr r2, [pc, #-0xef4]       @ 0x6ec <entry+0x6ec>
    c.literal_cond(C::Al,R::R2,0x3a2bc)?;
    // 0x3b1ac: str r0, [sp, #0x874]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2164)?)?;
    // 0x3b1b0: str r2, [sp, #0x878]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2168)?)?;
    // 0x3b1b4: mov r2, #460
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,460)?)?;
    // 0x3b1b8: str r1, [sp, #0x87c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2172)?)?;
    // 0x3b1bc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b1c0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b1c4: str r2, [sp, #0x880]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2176)?)?;
    // 0x3b1c8: ldr r2, [pc, #-0xf10]       @ 0x6f0 <entry+0x6f0>
    c.literal_cond(C::Al,R::R2,0x3a2c0)?;
    // 0x3b1cc: str r0, [sp, #0x884]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2180)?)?;
    // 0x3b1d0: str r2, [sp, #0x888]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2184)?)?;
    // 0x3b1d4: mov r2, #464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,464)?)?;
    // 0x3b1d8: str r1, [sp, #0x88c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2188)?)?;
    // 0x3b1dc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b1e0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b1e4: str r2, [sp, #0x890]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2192)?)?;
    // 0x3b1e8: ldr r2, [pc, #-0xf2c]       @ 0x6f4 <entry+0x6f4>
    c.literal_cond(C::Al,R::R2,0x3a2c4)?;
    // 0x3b1ec: str r0, [sp, #0x894]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2196)?)?;
    // 0x3b1f0: str r2, [sp, #0x898]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2200)?)?;
    // 0x3b1f4: mov r2, #228
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,228)?)?;
    // 0x3b1f8: str r1, [sp, #0x89c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2204)?)?;
    // 0x3b1fc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b200: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b204: str r2, [sp, #0x8a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2208)?)?;
    // 0x3b208: ldr r2, [pc, #-0xf48]       @ 0x6f8 <entry+0x6f8>
    c.literal_cond(C::Al,R::R2,0x3a2c8)?;
    // 0x3b20c: str r0, [sp, #0x8a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2212)?)?;
    // 0x3b210: str r2, [sp, #0x8a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2216)?)?;
    // 0x3b214: mov r2, #232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,232)?)?;
    // 0x3b218: str r1, [sp, #0x8ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2220)?)?;
    // 0x3b21c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b220: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b224: str r2, [sp, #0x8b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2224)?)?;
    // 0x3b228: ldr r2, [pc, #-0xf64]       @ 0x6fc <entry+0x6fc>
    c.literal_cond(C::Al,R::R2,0x3a2cc)?;
    // 0x3b22c: str r0, [sp, #0x8b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2228)?)?;
    // 0x3b230: str r2, [sp, #0x8b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2232)?)?;
    // 0x3b234: mov r2, #236
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,236)?)?;
    // 0x3b238: str r1, [sp, #0x8bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2236)?)?;
    // 0x3b23c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b240: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b244: str r2, [sp, #0x8c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2240)?)?;
    // 0x3b248: ldr r2, [pc, #-0xf80]       @ 0x700 <entry+0x700>
    c.literal_cond(C::Al,R::R2,0x3a2d0)?;
    // 0x3b24c: str r0, [sp, #0x8c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2244)?)?;
    // 0x3b250: str r2, [sp, #0x8c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2248)?)?;
    // 0x3b254: mov r2, #240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,240)?)?;
    // 0x3b258: str r1, [sp, #0x8cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2252)?)?;
    // 0x3b25c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b260: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b264: str r2, [sp, #0x8d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2256)?)?;
    // 0x3b268: ldr r2, [pc, #-0xf9c]       @ 0x704 <entry+0x704>
    c.literal_cond(C::Al,R::R2,0x3a2d4)?;
    // 0x3b26c: str r0, [sp, #0x8d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2260)?)?;
    // 0x3b270: str r2, [sp, #0x8d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2264)?)?;
    // 0x3b274: add r2, r6, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x3b278: str r1, [sp, #0x8dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2268)?)?;
    // 0x3b27c: str r2, [sp, #0x8e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2272)?)?;
    // 0x3b280: ldr r2, [pc, #-0xfb0]       @ 0x708 <entry+0x708>
    c.literal_cond(C::Al,R::R2,0x3a2d8)?;
    // 0x3b284: str r0, [sp, #0x8e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2276)?)?;
    // 0x3b288: str r2, [sp, #0x8e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2280)?)?;
    // 0x3b28c: add r2, r6, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,4352)?)?;
    // 0x3b290: str r1, [sp, #0x8ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2284)?)?;
    // 0x3b294: str r2, [sp, #0x8f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2288)?)?;
    // 0x3b298: ldr r2, [pc, #-0xfc4]       @ 0x70c <entry+0x70c>
    c.literal_cond(C::Al,R::R2,0x3a2dc)?;
    // 0x3b29c: str r0, [sp, #0x8f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2292)?)?;
    // 0x3b2a0: str r2, [sp, #0x8f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2296)?)?;
    // 0x3b2a4: mov r2, #260
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,260)?)?;
    // 0x3b2a8: str r1, [sp, #0x8fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2300)?)?;
    // 0x3b2ac: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b2b0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b2b4: str r2, [sp, #0x900]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2304)?)?;
    // 0x3b2b8: ldr r2, [pc, #-0xfe0]       @ 0x710 <entry+0x710>
    c.literal_cond(C::Al,R::R2,0x3a2e0)?;
    // 0x3b2bc: str r0, [sp, #0x904]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2308)?)?;
    // 0x3b2c0: str r2, [sp, #0x908]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2312)?)?;
    // 0x3b2c4: mov r2, #264
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,264)?)?;
    // 0x3b2c8: str r1, [sp, #0x90c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2316)?)?;
    // 0x3b2cc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b2d0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b2d4: str r2, [sp, #0x910]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2320)?)?;
    // 0x3b2d8: ldr r2, [pc, #0xff0]        @ 0x2700 <entry+0x2700>
    c.literal_cond(C::Al,R::R2,0x3c2d0)?;
    // 0x3b2dc: str r0, [sp, #0x914]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2324)?)?;
    // 0x3b2e0: str r2, [sp, #0x918]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2328)?)?;
    // 0x3b2e4: mov r2, #268
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,268)?)?;
    // 0x3b2e8: str r1, [sp, #0x91c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2332)?)?;
    // 0x3b2ec: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b2f0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b2f4: str r2, [sp, #0x920]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2336)?)?;
    // 0x3b2f8: ldr r2, [pc, #0xfd4]        @ 0x2704 <entry+0x2704>
    c.literal_cond(C::Al,R::R2,0x3c2d4)?;
    // 0x3b2fc: str r0, [sp, #0x924]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2340)?)?;
    // 0x3b300: str r2, [sp, #0x928]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2344)?)?;
    // 0x3b304: mov r2, #272
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,272)?)?;
    // 0x3b308: str r1, [sp, #0x92c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2348)?)?;
    // 0x3b30c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b310: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b314: str r2, [sp, #0x930]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2352)?)?;
    // 0x3b318: ldr r2, [pc, #0xfb8]        @ 0x2708 <entry+0x2708>
    c.literal_cond(C::Al,R::R2,0x3c2d8)?;
    // 0x3b31c: str r0, [sp, #0x934]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2356)?)?;
    // 0x3b320: str r2, [sp, #0x938]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2360)?)?;
    // 0x3b324: mov r2, #276
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,276)?)?;
    // 0x3b328: str r1, [sp, #0x93c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2364)?)?;
    // 0x3b32c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b330: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b334: str r2, [sp, #0x940]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2368)?)?;
    // 0x3b338: ldr r2, [pc, #0xf9c]        @ 0x270c <entry+0x270c>
    c.literal_cond(C::Al,R::R2,0x3c2dc)?;
    // 0x3b33c: str r0, [sp, #0x944]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2372)?)?;
    // 0x3b340: str r2, [sp, #0x948]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2376)?)?;
    // 0x3b344: mov r2, #280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,280)?)?;
    // 0x3b348: str r1, [sp, #0x94c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2380)?)?;
    // 0x3b34c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b350: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b354: str r2, [sp, #0x950]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2384)?)?;
    // 0x3b358: ldr r2, [pc, #0xf80]        @ 0x2710 <entry+0x2710>
    c.literal_cond(C::Al,R::R2,0x3c2e0)?;
    // 0x3b35c: str r0, [sp, #0x954]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2388)?)?;
    // 0x3b360: str r2, [sp, #0x958]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2392)?)?;
    // 0x3b364: mov r2, #284
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,284)?)?;
    // 0x3b368: str r1, [sp, #0x95c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2396)?)?;
    // 0x3b36c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b370: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b374: str r2, [sp, #0x960]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2400)?)?;
    // 0x3b378: ldr r2, [pc, #0xf64]        @ 0x2714 <entry+0x2714>
    c.literal_cond(C::Al,R::R2,0x3c2e4)?;
    // 0x3b37c: str r0, [sp, #0x964]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2404)?)?;
    // 0x3b380: str r2, [sp, #0x968]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2408)?)?;
    // 0x3b384: mov r2, #288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,288)?)?;
    // 0x3b388: str r1, [sp, #0x96c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2412)?)?;
    // 0x3b38c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b390: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b394: str r2, [sp, #0x970]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2416)?)?;
    // 0x3b398: ldr r2, [pc, #0xf48]        @ 0x2718 <entry+0x2718>
    c.literal_cond(C::Al,R::R2,0x3c2e8)?;
    // 0x3b39c: str r0, [sp, #0x974]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2420)?)?;
    // 0x3b3a0: str r2, [sp, #0x978]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2424)?)?;
    // 0x3b3a4: mov r2, #292
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,292)?)?;
    // 0x3b3a8: str r1, [sp, #0x97c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2428)?)?;
    // 0x3b3ac: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b3b0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b3b4: str r2, [sp, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2432)?)?;
    // 0x3b3b8: ldr r2, [pc, #0xf2c]        @ 0x271c <entry+0x271c>
    c.literal_cond(C::Al,R::R2,0x3c2ec)?;
    // 0x3b3bc: str r0, [sp, #0x984]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2436)?)?;
    // 0x3b3c0: str r2, [sp, #0x988]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2440)?)?;
    // 0x3b3c4: mov r2, #296
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,296)?)?;
    // 0x3b3c8: str r1, [sp, #0x98c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2444)?)?;
    // 0x3b3cc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b3d0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b3d4: str r2, [sp, #0x990]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2448)?)?;
    // 0x3b3d8: ldr r2, [pc, #0xf10]        @ 0x2720 <entry+0x2720>
    c.literal_cond(C::Al,R::R2,0x3c2f0)?;
    // 0x3b3dc: str r0, [sp, #0x994]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2452)?)?;
    // 0x3b3e0: str r2, [sp, #0x998]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2456)?)?;
    // 0x3b3e4: mov r2, #300
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,300)?)?;
    // 0x3b3e8: str r1, [sp, #0x99c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2460)?)?;
    // 0x3b3ec: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b3f0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b3f4: str r2, [sp, #0x9a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2464)?)?;
    // 0x3b3f8: ldr r2, [pc, #0xef4]        @ 0x2724 <entry+0x2724>
    c.literal_cond(C::Al,R::R2,0x3c2f4)?;
    // 0x3b3fc: str r0, [sp, #0x9a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2468)?)?;
    // 0x3b400: str r2, [sp, #0x9a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2472)?)?;
    // 0x3b404: mov r2, #304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,304)?)?;
    // 0x3b408: str r1, [sp, #0x9ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2476)?)?;
    // 0x3b40c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b410: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b414: str r2, [sp, #0x9b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2480)?)?;
    // 0x3b418: ldr r2, [pc, #0xed8]        @ 0x2728 <entry+0x2728>
    c.literal_cond(C::Al,R::R2,0x3c2f8)?;
    // 0x3b41c: str r0, [sp, #0x9b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2484)?)?;
    // 0x3b420: str r2, [sp, #0x9b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2488)?)?;
    // 0x3b424: mov r2, #308
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,308)?)?;
    // 0x3b428: str r1, [sp, #0x9bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2492)?)?;
    // 0x3b42c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b430: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b434: str r2, [sp, #0x9c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2496)?)?;
    // 0x3b438: ldr r2, [pc, #0xebc]        @ 0x272c <entry+0x272c>
    c.literal_cond(C::Al,R::R2,0x3c2fc)?;
    // 0x3b43c: str r0, [sp, #0x9c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2500)?)?;
    // 0x3b440: str r2, [sp, #0x9c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2504)?)?;
    // 0x3b444: mov r2, #312
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,312)?)?;
    // 0x3b448: str r1, [sp, #0x9cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2508)?)?;
    // 0x3b44c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b450: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b454: str r2, [sp, #0x9d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2512)?)?;
    // 0x3b458: ldr r2, [pc, #0xea0]        @ 0x2730 <entry+0x2730>
    c.literal_cond(C::Al,R::R2,0x3c300)?;
    // 0x3b45c: str r0, [sp, #0x9d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2516)?)?;
    // 0x3b460: str r2, [sp, #0x9d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2520)?)?;
    // 0x3b464: mov r2, #316
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,316)?)?;
    // 0x3b468: str r1, [sp, #0x9dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2524)?)?;
    // 0x3b46c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b470: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b474: str r2, [sp, #0x9e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2528)?)?;
    // 0x3b478: ldr r2, [pc, #0xe84]        @ 0x2734 <entry+0x2734>
    c.literal_cond(C::Al,R::R2,0x3c304)?;
    // 0x3b47c: str r0, [sp, #0x9e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2532)?)?;
    // 0x3b480: str r2, [sp, #0x9e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2536)?)?;
    // 0x3b484: add r2, r6, #4416
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,4416)?)?;
    // 0x3b488: str r0, [sp, #0x9ec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2540)?)?;
    // 0x3b48c: str r2, [sp, #0x9f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2544)?)?;
    // 0x3b490: mov r2, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,23)?)?;
    // 0x3b494: str r2, [sp, #0x9f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2548)?)?;
    // 0x3b498: str r3, [sp, #0x9f8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,2552)?)?;
    // 0x3b49c: add r3, r6, #87
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,87)?)?;
    // 0x3b4a0: str r0, [sp, #0x9fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2556)?)?;
    // 0x3b4a4: add r3, r3, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4352)?)?;
    // 0x3b4a8: str r3, [sp, #0xa00]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,2560)?)?;
    // 0x3b4ac: ldr r3, [pc, #0xe54]        @ 0x2738 <entry+0x2738>
    c.literal_cond(C::Al,R::new(3)?,0x3c308)?;
    // 0x3b4b0: str r2, [sp, #0xa04]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2564)?)?;
    // 0x3b4b4: str r3, [sp, #0xa08]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,2568)?)?;
    // 0x3b4b8: add r3, r6, #110
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,110)?)?;
    // 0x3b4bc: str r0, [sp, #0xa0c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2572)?)?;
    // 0x3b4c0: add r3, r3, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4352)?)?;
    // 0x3b4c4: str r3, [sp, #0xa10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,2576)?)?;
    // 0x3b4c8: ldr r3, [pc, #0xe3c]        @ 0x273c <entry+0x273c>
    c.literal_cond(C::Al,R::new(3)?,0x3c30c)?;
    // 0x3b4cc: str r2, [sp, #0xa14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2580)?)?;
    // 0x3b4d0: str r3, [sp, #0xa18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,2584)?)?;
    // 0x3b4d4: add r3, r6, #133
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,133)?)?;
    // 0x3b4d8: str r0, [sp, #0xa1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2588)?)?;
    // 0x3b4dc: add r3, r3, #4352
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4352)?)?;
    // 0x3b4e0: str r3, [sp, #0xa20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,2592)?)?;
    // 0x3b4e4: ldr r3, [pc, #0xe24]        @ 0x2740 <entry+0x2740>
    c.literal_cond(C::Al,R::new(3)?,0x3c310)?;
    // 0x3b4e8: str r2, [sp, #0xa24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2596)?)?;
    // 0x3b4ec: str r3, [sp, #0xa28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,2600)?)?;
    // 0x3b4f0: mov r3, #412
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,412)?)?;
    // 0x3b4f4: str r0, [sp, #0xa2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2604)?)?;
    // 0x3b4f8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3b4fc: add r3, r6, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3b500: str r3, [sp, #0xa30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,2608)?)?;
    // 0x3b504: str r2, [sp, #0xa34]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2612)?)?;
    // 0x3b508: ldr r2, [pc, #0xe04]        @ 0x2744 <entry+0x2744>
    c.literal_cond(C::Al,R::R2,0x3c314)?;
    // 0x3b50c: ldr r3, [pc, #0xe04]        @ 0x2748 <entry+0x2748>
    c.literal_cond(C::Al,R::new(3)?,0x3c318)?;
    // 0x3b510: str r2, [sp, #0xa38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2616)?)?;
    // 0x3b514: mov r2, #472
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,472)?)?;
    // 0x3b518: str r1, [sp, #0xa3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2620)?)?;
    // 0x3b51c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b520: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b524: str r2, [sp, #0xa40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2624)?)?;
    // 0x3b528: ldr r2, [pc, #0xdec]        @ 0x274c <entry+0x274c>
    c.literal_cond(C::Al,R::R2,0x3c31c)?;
    // 0x3b52c: str r0, [sp, #0xa44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2628)?)?;
    // 0x3b530: str r2, [sp, #0xa48]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2632)?)?;
    // 0x3b534: mov r2, #476
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,476)?)?;
    // 0x3b538: str r1, [sp, #0xa4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2636)?)?;
    // 0x3b53c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b540: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b544: str r2, [sp, #0xa50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2640)?)?;
    // 0x3b548: ldr r2, [pc, #0xdd0]        @ 0x2750 <entry+0x2750>
    c.literal_cond(C::Al,R::R2,0x3c320)?;
    // 0x3b54c: str r0, [sp, #0xa54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2644)?)?;
    // 0x3b550: str r2, [sp, #0xa58]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2648)?)?;
    // 0x3b554: mov r2, #480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,480)?)?;
    // 0x3b558: str r1, [sp, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2652)?)?;
    // 0x3b55c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b560: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b564: str r2, [sp, #0xa60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2656)?)?;
    // 0x3b568: ldr r2, [pc, #0xdb4]        @ 0x2754 <entry+0x2754>
    c.literal_cond(C::Al,R::R2,0x3c324)?;
    // 0x3b56c: str r0, [sp, #0xa64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2660)?)?;
    // 0x3b570: str r2, [sp, #0xa68]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2664)?)?;
    // 0x3b574: mov r2, #484
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,484)?)?;
    // 0x3b578: str r1, [sp, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2668)?)?;
    // 0x3b57c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b580: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b584: str r2, [sp, #0xa70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2672)?)?;
    // 0x3b588: ldr r2, [pc, #0xd98]        @ 0x2758 <entry+0x2758>
    c.literal_cond(C::Al,R::R2,0x3c328)?;
    // 0x3b58c: str r0, [sp, #0xa74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2676)?)?;
    // 0x3b590: str r2, [sp, #0xa78]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2680)?)?;
    // 0x3b594: mov r2, #488
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,488)?)?;
    // 0x3b598: str r1, [sp, #0xa7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2684)?)?;
    // 0x3b59c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b5a0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b5a4: str r2, [sp, #0xa80]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2688)?)?;
    // 0x3b5a8: ldr r2, [pc, #0xd7c]        @ 0x275c <entry+0x275c>
    c.literal_cond(C::Al,R::R2,0x3c32c)?;
    // 0x3b5ac: str r0, [sp, #0xa84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2692)?)?;
    // 0x3b5b0: str r2, [sp, #0xa88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2696)?)?;
    // 0x3b5b4: mov r2, #492
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,492)?)?;
    // 0x3b5b8: str r1, [sp, #0xa8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2700)?)?;
    // 0x3b5bc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b5c0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b5c4: str r2, [sp, #0xa90]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2704)?)?;
    // 0x3b5c8: ldr r2, [pc, #0xd60]        @ 0x2760 <entry+0x2760>
    c.literal_cond(C::Al,R::R2,0x3c330)?;
    // 0x3b5cc: str r0, [sp, #0xa94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2708)?)?;
    // 0x3b5d0: str r2, [sp, #0xa98]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2712)?)?;
    // 0x3b5d4: mov r2, #496
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,496)?)?;
    // 0x3b5d8: str r1, [sp, #0xa9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2716)?)?;
    // 0x3b5dc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b5e0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b5e4: str r2, [sp, #0xaa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2720)?)?;
    // 0x3b5e8: ldr r2, [pc, #0xd44]        @ 0x2764 <entry+0x2764>
    c.literal_cond(C::Al,R::R2,0x3c334)?;
    // 0x3b5ec: str r0, [sp, #0xaa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2724)?)?;
    // 0x3b5f0: str r2, [sp, #0xaa8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2728)?)?;
    // 0x3b5f4: mov r2, #500
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,500)?)?;
    // 0x3b5f8: str r1, [sp, #0xaac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2732)?)?;
    // 0x3b5fc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b600: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b604: str r2, [sp, #0xab0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2736)?)?;
    // 0x3b608: ldr r2, [pc, #0xd28]        @ 0x2768 <entry+0x2768>
    c.literal_cond(C::Al,R::R2,0x3c338)?;
    // 0x3b60c: str r0, [sp, #0xab4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2740)?)?;
    // 0x3b610: str r2, [sp, #0xab8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2744)?)?;
    // 0x3b614: mov r2, #504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,504)?)?;
    // 0x3b618: str r1, [sp, #0xabc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2748)?)?;
    // 0x3b61c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b620: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b624: str r2, [sp, #0xac0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2752)?)?;
    // 0x3b628: ldr r2, [pc, #0xd0c]        @ 0x276c <entry+0x276c>
    c.literal_cond(C::Al,R::R2,0x3c33c)?;
    // 0x3b62c: str r0, [sp, #0xac4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2756)?)?;
    // 0x3b630: str r2, [sp, #0xac8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2760)?)?;
    // 0x3b634: mov r2, #508
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,508)?)?;
    // 0x3b638: str r1, [sp, #0xacc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2764)?)?;
    // 0x3b63c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b640: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b644: str r2, [sp, #0xad0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2768)?)?;
    // 0x3b648: ldr r2, [pc, #0xcf0]        @ 0x2770 <entry+0x2770>
    c.literal_cond(C::Al,R::R2,0x3c340)?;
    // 0x3b64c: str r0, [sp, #0xad4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2772)?)?;
    // 0x3b650: str r2, [sp, #0xad8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2776)?)?;
    // 0x3b654: add r2, r6, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,4608)?)?;
    // 0x3b658: str r1, [sp, #0xadc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2780)?)?;
    // 0x3b65c: str r2, [sp, #0xae0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2784)?)?;
    // 0x3b660: ldr r2, [pc, #0xcdc]        @ 0x2774 <entry+0x2774>
    c.literal_cond(C::Al,R::R2,0x3c344)?;
    // 0x3b664: str r0, [sp, #0xae4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2788)?)?;
    // 0x3b668: str r2, [sp, #0xae8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2792)?)?;
    // 0x3b66c: mov r2, #516
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,516)?)?;
    // 0x3b670: str r1, [sp, #0xaec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2796)?)?;
    // 0x3b674: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b678: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b67c: str r2, [sp, #0xaf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2800)?)?;
    // 0x3b680: ldr r2, [pc, #0xcc0]        @ 0x2778 <entry+0x2778>
    c.literal_cond(C::Al,R::R2,0x3c348)?;
    // 0x3b684: str r0, [sp, #0xaf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2804)?)?;
    // 0x3b688: str r2, [sp, #0xaf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2808)?)?;
    // 0x3b68c: mov r2, #520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,520)?)?;
    // 0x3b690: str r1, [sp, #0xafc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2812)?)?;
    // 0x3b694: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b698: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b69c: str r2, [sp, #0xb00]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2816)?)?;
    // 0x3b6a0: ldr r2, [pc, #0xca4]        @ 0x277c <entry+0x277c>
    c.literal_cond(C::Al,R::R2,0x3c34c)?;
    // 0x3b6a4: str r0, [sp, #0xb04]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2820)?)?;
    // 0x3b6a8: str r2, [sp, #0xb08]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2824)?)?;
    // 0x3b6ac: mov r2, #524
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,524)?)?;
    // 0x3b6b0: str r1, [sp, #0xb0c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2828)?)?;
    // 0x3b6b4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b6b8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b6bc: str r2, [sp, #0xb10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2832)?)?;
    // 0x3b6c0: ldr r2, [pc, #0xc88]        @ 0x2780 <entry+0x2780>
    c.literal_cond(C::Al,R::R2,0x3c350)?;
    // 0x3b6c4: str r0, [sp, #0xb14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2836)?)?;
    // 0x3b6c8: str r2, [sp, #0xb18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2840)?)?;
    // 0x3b6cc: mov r2, #528
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,528)?)?;
    // 0x3b6d0: str r1, [sp, #0xb1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2844)?)?;
    // 0x3b6d4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b6d8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b6dc: str r2, [sp, #0xb20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2848)?)?;
    // 0x3b6e0: ldr r2, [pc, #0xc6c]        @ 0x2784 <entry+0x2784>
    c.literal_cond(C::Al,R::R2,0x3c354)?;
    // 0x3b6e4: str r0, [sp, #0xb24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2852)?)?;
    // 0x3b6e8: str r2, [sp, #0xb28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2856)?)?;
    // 0x3b6ec: mov r2, #532
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,532)?)?;
    // 0x3b6f0: str r1, [sp, #0xb2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2860)?)?;
    // 0x3b6f4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b6f8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b6fc: str r2, [sp, #0xb30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2864)?)?;
    // 0x3b700: ldr r2, [pc, #0xc50]        @ 0x2788 <entry+0x2788>
    c.literal_cond(C::Al,R::R2,0x3c358)?;
    // 0x3b704: str r0, [sp, #0xb34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2868)?)?;
    // 0x3b708: str r2, [sp, #0xb38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2872)?)?;
    // 0x3b70c: mov r2, #536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,536)?)?;
    // 0x3b710: str r1, [sp, #0xb3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2876)?)?;
    // 0x3b714: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b718: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b71c: str r2, [sp, #0xb40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2880)?)?;
    // 0x3b720: ldr r2, [pc, #0xc34]        @ 0x278c <entry+0x278c>
    c.literal_cond(C::Al,R::R2,0x3c35c)?;
    // 0x3b724: str r0, [sp, #0xb44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2884)?)?;
    // 0x3b728: str r2, [sp, #0xb48]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2888)?)?;
    // 0x3b72c: mov r2, #540
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,540)?)?;
    // 0x3b730: str r1, [sp, #0xb4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2892)?)?;
    // 0x3b734: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b738: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b73c: str r2, [sp, #0xb50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2896)?)?;
    // 0x3b740: ldr r2, [pc, #0xc18]        @ 0x2790 <entry+0x2790>
    c.literal_cond(C::Al,R::R2,0x3c360)?;
    // 0x3b744: str r0, [sp, #0xb54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2900)?)?;
    // 0x3b748: str r2, [sp, #0xb58]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2904)?)?;
    // 0x3b74c: mov r2, #544
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,544)?)?;
    // 0x3b750: str r1, [sp, #0xb5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2908)?)?;
    // 0x3b754: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b758: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b75c: str r2, [sp, #0xb60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2912)?)?;
    // 0x3b760: ldr r2, [pc, #0xbfc]        @ 0x2794 <entry+0x2794>
    c.literal_cond(C::Al,R::R2,0x3c364)?;
    // 0x3b764: str r0, [sp, #0xb64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2916)?)?;
    // 0x3b768: str r2, [sp, #0xb68]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2920)?)?;
    // 0x3b76c: mov r2, #548
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,548)?)?;
    // 0x3b770: str r1, [sp, #0xb6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2924)?)?;
    // 0x3b774: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b778: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b77c: str r2, [sp, #0xb70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2928)?)?;
    // 0x3b780: ldr r2, [pc, #0xbe0]        @ 0x2798 <entry+0x2798>
    c.literal_cond(C::Al,R::R2,0x3c368)?;
    // 0x3b784: str r0, [sp, #0xb74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2932)?)?;
    // 0x3b788: str r2, [sp, #0xb78]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2936)?)?;
    // 0x3b78c: mov r2, #552
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,552)?)?;
    // 0x3b790: str r1, [sp, #0xb7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2940)?)?;
    // 0x3b794: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b798: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b79c: str r2, [sp, #0xb80]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2944)?)?;
    // 0x3b7a0: ldr r2, [pc, #0xbc4]        @ 0x279c <entry+0x279c>
    c.literal_cond(C::Al,R::R2,0x3c36c)?;
    // 0x3b7a4: str r0, [sp, #0xb84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2948)?)?;
    // 0x3b7a8: str r2, [sp, #0xb88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2952)?)?;
    // 0x3b7ac: mov r2, #556
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,556)?)?;
    // 0x3b7b0: str r1, [sp, #0xb8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2956)?)?;
    // 0x3b7b4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b7b8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b7bc: str r2, [sp, #0xb90]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2960)?)?;
    // 0x3b7c0: ldr r2, [pc, #0xba8]        @ 0x27a0 <entry+0x27a0>
    c.literal_cond(C::Al,R::R2,0x3c370)?;
    // 0x3b7c4: str r0, [sp, #0xb94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2964)?)?;
    // 0x3b7c8: str r2, [sp, #0xb98]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2968)?)?;
    // 0x3b7cc: mov r2, #560
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,560)?)?;
    // 0x3b7d0: str r1, [sp, #0xb9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2972)?)?;
    // 0x3b7d4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b7d8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b7dc: str r2, [sp, #0xba0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2976)?)?;
    // 0x3b7e0: ldr r2, [pc, #0xb8c]        @ 0x27a4 <entry+0x27a4>
    c.literal_cond(C::Al,R::R2,0x3c374)?;
    // 0x3b7e4: str r0, [sp, #0xba4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2980)?)?;
    // 0x3b7e8: str r2, [sp, #0xba8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2984)?)?;
    // 0x3b7ec: mov r2, #684
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,684)?)?;
    // 0x3b7f0: str r1, [sp, #0xbac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,2988)?)?;
    // 0x3b7f4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b7f8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b7fc: str r2, [sp, #0xbb0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,2992)?)?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{initialize_codec_output_planes(c)}

#[cfg(test)] mod tests {
 use super::*;
 #[test]fn additional_register_matrix_has_exact_layout(){
   let mut c=Code::new(START);emit(&mut c).unwrap();
   assert_eq!(c.bytes.len(),END-START);
 }
}
