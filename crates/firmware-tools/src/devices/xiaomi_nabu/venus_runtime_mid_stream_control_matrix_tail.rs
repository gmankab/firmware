// SPDX-License-Identifier: GPL-3.0-only
//! Typed Venus A32 stream descriptor initializer tail and controller
//! diagnostic-pointer table. All 155 metadata addresses are classified
//! as non-executable data; no unknown source instruction bytes are copied.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
const START:usize=0x3b800;
const END:usize=0x3c53c;

/// Original, read-only diagnostic string-address table. These are static
/// device pointers, not A32 code. The enclosing initializer continues after
/// the table, so no artificial function return is inserted at this boundary.
const STREAM_OUTPUT_DIAGNOSTICS:[u32;155]=[
    0x000ecd38,0x000ecd4d,0x000ecd61,0x000ecd7a,0x000ecd92,0x000ecda5,0x000ecdb7,
    0x000ecdca,0x000ecde3,0x000ecdf6,0x000ece0d,0x000ece1e,0x000ece29,0x000ece34,
    0x000ece55,0x000ece64,0x000ece73,0x000ece82,0x000ed438,0x000ece91,0x000ecea0,
    0x000eceb1,0x000ecec2,0x000eced1,0x000ecee0,0x000ecef1,0x000ecf02,0x000ecf11,
    0x000ecf20,0x000ecf31,0x000ecf42,0x000ecf4f,0x000ecf5c,0x000ecf6a,0x000ecf7a,
    0x000ecf8a,0x000ecf98,0x000ecfa8,0x000ecfb8,0x000ecfc6,0x000ecfd6,0x000ecfe6,
    0x000ecff3,0x000ed006,0x000ed015,0x000ed02a,0x000ed039,0x000ed049,0x000ed05f,
    0x000ed071,0x000ed089,0x000ed09b,0x000ed0a5,0x000ed0af,0x000ed0ba,0x000ed0c5,
    0x000ed0d0,0x000ed0db,0x000ed0ea,0x000ed0fc,0x000ed10f,0x000ed11d,0x000ed12d,
    0x000ed13d,0x000ed14d,0x000ed15d,0x000ed16d,0x000ed17d,0x000ed18d,0x000ed19d,
    0x000ed1ad,0x000ed1bd,0x000ed1ce,0x000ed1df,0x000ed1f0,0x000ed201,0x000ed212,
    0x000ed223,0x000ed234,0x000ed245,0x000ed256,0x000ed267,0x000ed278,0x000ed289,
    0x000ed29a,0x000ed2ab,0x000ed2bc,0x000ed2cd,0x000ed2e1,0x000ed2f5,0x000ed30b,
    0x000ed321,0x000ed32e,0x000ed339,0x000ed345,0x000ed353,0x000ed35e,0x000ed368,
    0x000ed376,0x000ed381,0x000ed38b,0x000ed398,0x000ed3a2,0x000ed3ab,0x000ed3b8,
    0x000ed3c2,0x000ed3cb,0x000ed3e1,0x000ed3f5,0x000ed40a,0x000ed417,0x000ed42b,
    0x000ed445,0x000ed452,0x000ed45f,0x000ed46c,0x000ed479,0x000ed486,0x000ed493,
    0x000ed4a0,0x000ed4ad,0x000ed4b8,0x000ed4bf,0x000ed4c6,0x000ed4ce,0x000ed4d9,
    0x000ed4e6,0x000ed4f1,0x000ed787,0x000ed4ff,0x000ed50a,0x000ed515,0x000ed51f,
    0x000ed52e,0x000ed541,0x000ed552,0x000ed567,0x000ed575,0x000ed587,0x000ed597,
    0x000ed5ab,0x000ed5b8,0x000ed5c9,0x000ed5d6,0x000ed5e2,0x000ed5f1,0x000ed601,
    0x000ed60b,0x000ed618,0x000ed624,0x000ed634,0x000ed766,0x000ed645,0x000ed655,
    0x000ed664,
];

fn append_stream_descriptor_matrix(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3b800: ldr r2, [pc, #0xb70]        @ 0x27a8 <entry+0x27a8>
    c.literal_cond(C::Al,R::R2,0x3c378)?;
    // 0x3b804: str r0, [sp, #0xbb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,2996)?)?;
    // 0x3b808: str r2, [sp, #0xbb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3000)?)?;
    // 0x3b80c: mov r2, #688
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,688)?)?;
    // 0x3b810: str r1, [sp, #0xbbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3004)?)?;
    // 0x3b814: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b818: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b81c: str r2, [sp, #0xbc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3008)?)?;
    // 0x3b820: ldr r2, [pc, #0xb54]        @ 0x27ac <entry+0x27ac>
    c.literal_cond(C::Al,R::R2,0x3c37c)?;
    // 0x3b824: str r0, [sp, #0xbc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3012)?)?;
    // 0x3b828: str r2, [sp, #0xbc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3016)?)?;
    // 0x3b82c: mov r2, #692
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,692)?)?;
    // 0x3b830: str r1, [sp, #0xbcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3020)?)?;
    // 0x3b834: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b838: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b83c: str r2, [sp, #0xbd0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3024)?)?;
    // 0x3b840: ldr r2, [pc, #0xb38]        @ 0x27b0 <entry+0x27b0>
    c.literal_cond(C::Al,R::R2,0x3c380)?;
    // 0x3b844: str r0, [sp, #0xbd4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3028)?)?;
    // 0x3b848: str r2, [sp, #0xbd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3032)?)?;
    // 0x3b84c: mov r2, #696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,696)?)?;
    // 0x3b850: str r1, [sp, #0xbdc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3036)?)?;
    // 0x3b854: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b858: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b85c: str r2, [sp, #0xbe0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3040)?)?;
    // 0x3b860: ldr r2, [pc, #0xb1c]        @ 0x27b4 <entry+0x27b4>
    c.literal_cond(C::Al,R::R2,0x3c384)?;
    // 0x3b864: str r0, [sp, #0xbe4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3044)?)?;
    // 0x3b868: str r2, [sp, #0xbe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3048)?)?;
    // 0x3b86c: mov r2, #700
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,700)?)?;
    // 0x3b870: str r1, [sp, #0xbec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3052)?)?;
    // 0x3b874: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b878: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b87c: str r2, [sp, #0xbf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3056)?)?;
    // 0x3b880: ldr r2, [pc, #0xb00]        @ 0x27b8 <entry+0x27b8>
    c.literal_cond(C::Al,R::R2,0x3c388)?;
    // 0x3b884: str r0, [sp, #0xbf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3060)?)?;
    // 0x3b888: str r2, [sp, #0xbf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3064)?)?;
    // 0x3b88c: mov r2, #712
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,712)?)?;
    // 0x3b890: str r1, [sp, #0xbfc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3068)?)?;
    // 0x3b894: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b898: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b89c: str r2, [sp, #0xc00]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3072)?)?;
    // 0x3b8a0: ldr r2, [pc, #0xae4]        @ 0x27bc <entry+0x27bc>
    c.literal_cond(C::Al,R::R2,0x3c38c)?;
    // 0x3b8a4: str r0, [sp, #0xc04]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3076)?)?;
    // 0x3b8a8: str r2, [sp, #0xc08]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3080)?)?;
    // 0x3b8ac: mov r2, #716
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,716)?)?;
    // 0x3b8b0: str r1, [sp, #0xc0c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3084)?)?;
    // 0x3b8b4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b8b8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b8bc: str r2, [sp, #0xc10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3088)?)?;
    // 0x3b8c0: ldr r2, [pc, #0xac8]        @ 0x27c0 <entry+0x27c0>
    c.literal_cond(C::Al,R::R2,0x3c390)?;
    // 0x3b8c4: str r0, [sp, #0xc14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3092)?)?;
    // 0x3b8c8: str r2, [sp, #0xc18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3096)?)?;
    // 0x3b8cc: mov r2, #720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,720)?)?;
    // 0x3b8d0: str r1, [sp, #0xc1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3100)?)?;
    // 0x3b8d4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b8d8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b8dc: str r2, [sp, #0xc20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3104)?)?;
    // 0x3b8e0: ldr r2, [pc, #0xaac]        @ 0x27c4 <entry+0x27c4>
    c.literal_cond(C::Al,R::R2,0x3c394)?;
    // 0x3b8e4: str r0, [sp, #0xc24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3108)?)?;
    // 0x3b8e8: str r2, [sp, #0xc28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3112)?)?;
    // 0x3b8ec: mov r2, #724
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,724)?)?;
    // 0x3b8f0: str r1, [sp, #0xc2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3116)?)?;
    // 0x3b8f4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b8f8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b8fc: str r2, [sp, #0xc30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3120)?)?;
    // 0x3b900: ldr r2, [pc, #0xa90]        @ 0x27c8 <entry+0x27c8>
    c.literal_cond(C::Al,R::R2,0x3c398)?;
    // 0x3b904: str r0, [sp, #0xc34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3124)?)?;
    // 0x3b908: str r2, [sp, #0xc38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3128)?)?;
    // 0x3b90c: mov r2, #728
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,728)?)?;
    // 0x3b910: str r1, [sp, #0xc3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3132)?)?;
    // 0x3b914: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b918: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b91c: str r2, [sp, #0xc40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3136)?)?;
    // 0x3b920: ldr r2, [pc, #0xa74]        @ 0x27cc <entry+0x27cc>
    c.literal_cond(C::Al,R::R2,0x3c39c)?;
    // 0x3b924: str r0, [sp, #0xc44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3140)?)?;
    // 0x3b928: str r2, [sp, #0xc48]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3144)?)?;
    // 0x3b92c: mov r2, #732
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,732)?)?;
    // 0x3b930: str r1, [sp, #0xc4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3148)?)?;
    // 0x3b934: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b938: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b93c: str r2, [sp, #0xc50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3152)?)?;
    // 0x3b940: ldr r2, [pc, #0xa58]        @ 0x27d0 <entry+0x27d0>
    c.literal_cond(C::Al,R::R2,0x3c3a0)?;
    // 0x3b944: str r0, [sp, #0xc54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3156)?)?;
    // 0x3b948: str r2, [sp, #0xc58]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3160)?)?;
    // 0x3b94c: mov r2, #736
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,736)?)?;
    // 0x3b950: str r1, [sp, #0xc5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3164)?)?;
    // 0x3b954: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b958: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b95c: str r2, [sp, #0xc60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3168)?)?;
    // 0x3b960: ldr r2, [pc, #0xa3c]        @ 0x27d4 <entry+0x27d4>
    c.literal_cond(C::Al,R::R2,0x3c3a4)?;
    // 0x3b964: str r0, [sp, #0xc64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3172)?)?;
    // 0x3b968: str r2, [sp, #0xc68]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3176)?)?;
    // 0x3b96c: mov r2, #744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,744)?)?;
    // 0x3b970: str r1, [sp, #0xc6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3180)?)?;
    // 0x3b974: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b978: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b97c: str r2, [sp, #0xc70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3184)?)?;
    // 0x3b980: ldr r2, [pc, #0xa20]        @ 0x27d8 <entry+0x27d8>
    c.literal_cond(C::Al,R::R2,0x3c3a8)?;
    // 0x3b984: str r0, [sp, #0xc74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3188)?)?;
    // 0x3b988: str r2, [sp, #0xc78]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3192)?)?;
    // 0x3b98c: mov r2, #748
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,748)?)?;
    // 0x3b990: str r1, [sp, #0xc7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3196)?)?;
    // 0x3b994: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b998: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b99c: str r2, [sp, #0xc80]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3200)?)?;
    // 0x3b9a0: ldr r2, [pc, #0xa04]        @ 0x27dc <entry+0x27dc>
    c.literal_cond(C::Al,R::R2,0x3c3ac)?;
    // 0x3b9a4: str r0, [sp, #0xc84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3204)?)?;
    // 0x3b9a8: str r2, [sp, #0xc88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3208)?)?;
    // 0x3b9ac: mov r2, #756
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,756)?)?;
    // 0x3b9b0: str r1, [sp, #0xc8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3212)?)?;
    // 0x3b9b4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b9b8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b9bc: str r2, [sp, #0xc90]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3216)?)?;
    // 0x3b9c0: ldr r2, [pc, #0x9e8]        @ 0x27e0 <entry+0x27e0>
    c.literal_cond(C::Al,R::R2,0x3c3b0)?;
    // 0x3b9c4: str r0, [sp, #0xc94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3220)?)?;
    // 0x3b9c8: str r2, [sp, #0xc98]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3224)?)?;
    // 0x3b9cc: mov r2, #760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,760)?)?;
    // 0x3b9d0: str r1, [sp, #0xc9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3228)?)?;
    // 0x3b9d4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b9d8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b9dc: str r2, [sp, #0xca0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3232)?)?;
    // 0x3b9e0: ldr r2, [pc, #0x9cc]        @ 0x27e4 <entry+0x27e4>
    c.literal_cond(C::Al,R::R2,0x3c3b4)?;
    // 0x3b9e4: str r0, [sp, #0xca4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3236)?)?;
    // 0x3b9e8: str r2, [sp, #0xca8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3240)?)?;
    // 0x3b9ec: mov r2, #468
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,468)?)?;
    // 0x3b9f0: str r1, [sp, #0xcac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3244)?)?;
    // 0x3b9f4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3b9f8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3b9fc: str r2, [sp, #0xcb0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3248)?)?;
    // 0x3ba00: ldr r2, [pc, #0x9b0]        @ 0x27e8 <entry+0x27e8>
    c.literal_cond(C::Al,R::R2,0x3c3b8)?;
    // 0x3ba04: str r0, [sp, #0xcb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3252)?)?;
    // 0x3ba08: str r2, [sp, #0xcb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3256)?)?;
    // 0x3ba0c: mov r2, #680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,680)?)?;
    // 0x3ba10: str r1, [sp, #0xcbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3260)?)?;
    // 0x3ba14: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ba18: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3ba1c: str r2, [sp, #0xcc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3264)?)?;
    // 0x3ba20: ldr r2, [pc, #0x994]        @ 0x27ec <entry+0x27ec>
    c.literal_cond(C::Al,R::R2,0x3c3bc)?;
    // 0x3ba24: str r0, [sp, #0xcc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3268)?)?;
    // 0x3ba28: str r2, [sp, #0xcc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3272)?)?;
    // 0x3ba2c: mov r2, #564
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,564)?)?;
    // 0x3ba30: str r1, [sp, #0xccc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3276)?)?;
    // 0x3ba34: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ba38: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3ba3c: str r2, [sp, #0xcd0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3280)?)?;
    // 0x3ba40: ldr r2, [pc, #0x978]        @ 0x27f0 <entry+0x27f0>
    c.literal_cond(C::Al,R::R2,0x3c3c0)?;
    // 0x3ba44: str r0, [sp, #0xcd4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3284)?)?;
    // 0x3ba48: str r2, [sp, #0xcd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3288)?)?;
    // 0x3ba4c: mov r2, #568
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,568)?)?;
    // 0x3ba50: str r12, [sp, #0xcdc]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,3292)?)?;
    // 0x3ba54: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ba58: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3ba5c: str r2, [sp, #0xce0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3296)?)?;
    // 0x3ba60: ldr r2, [pc, #0x95c]        @ 0x27f4 <entry+0x27f4>
    c.literal_cond(C::Al,R::R2,0x3c3c4)?;
    // 0x3ba64: str r0, [sp, #0xce4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3300)?)?;
    // 0x3ba68: str r2, [sp, #0xce8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3304)?)?;
    // 0x3ba6c: add r2, r6, #4672
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,4672)?)?;
    // 0x3ba70: str r1, [sp, #0xcec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3308)?)?;
    // 0x3ba74: str r2, [sp, #0xcf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3312)?)?;
    // 0x3ba78: ldr r2, [pc, #0x948]        @ 0x27f8 <entry+0x27f8>
    c.literal_cond(C::Al,R::R2,0x3c3c8)?;
    // 0x3ba7c: str r0, [sp, #0xcf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3316)?)?;
    // 0x3ba80: str r2, [sp, #0xcf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3320)?)?;
    // 0x3ba84: mov r2, #580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,580)?)?;
    // 0x3ba88: str r1, [sp, #0xcfc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3324)?)?;
    // 0x3ba8c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3ba90: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3ba94: str r2, [sp, #0xd00]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3328)?)?;
    // 0x3ba98: ldr r2, [pc, #0x92c]        @ 0x27fc <entry+0x27fc>
    c.literal_cond(C::Al,R::R2,0x3c3cc)?;
    // 0x3ba9c: str r0, [sp, #0xd04]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3332)?)?;
    // 0x3baa0: str r2, [sp, #0xd08]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3336)?)?;
    // 0x3baa4: mov r2, #584
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,584)?)?;
    // 0x3baa8: str r1, [sp, #0xd0c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3340)?)?;
    // 0x3baac: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bab0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bab4: str r2, [sp, #0xd10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3344)?)?;
    // 0x3bab8: ldr r2, [pc, #0x910]        @ 0x2800 <entry+0x2800>
    c.literal_cond(C::Al,R::R2,0x3c3d0)?;
    // 0x3babc: str r0, [sp, #0xd14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3348)?)?;
    // 0x3bac0: str r2, [sp, #0xd18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3352)?)?;
    // 0x3bac4: mov r2, #588
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,588)?)?;
    // 0x3bac8: str r1, [sp, #0xd1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3356)?)?;
    // 0x3bacc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bad0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bad4: str r2, [sp, #0xd20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3360)?)?;
    // 0x3bad8: ldr r2, [pc, #0x8f4]        @ 0x2804 <entry+0x2804>
    c.literal_cond(C::Al,R::R2,0x3c3d4)?;
    // 0x3badc: str r0, [sp, #0xd24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3364)?)?;
    // 0x3bae0: str r2, [sp, #0xd28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3368)?)?;
    // 0x3bae4: mov r2, #592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,592)?)?;
    // 0x3bae8: str r1, [sp, #0xd2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3372)?)?;
    // 0x3baec: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3baf0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3baf4: str r2, [sp, #0xd30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3376)?)?;
    // 0x3baf8: ldr r2, [pc, #0x8d8]        @ 0x2808 <entry+0x2808>
    c.literal_cond(C::Al,R::R2,0x3c3d8)?;
    // 0x3bafc: str r0, [sp, #0xd34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3380)?)?;
    // 0x3bb00: str r2, [sp, #0xd38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3384)?)?;
    // 0x3bb04: mov r2, #596
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,596)?)?;
    // 0x3bb08: str r1, [sp, #0xd3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3388)?)?;
    // 0x3bb0c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bb10: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bb14: str r2, [sp, #0xd40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3392)?)?;
    // 0x3bb18: ldr r2, [pc, #0x8bc]        @ 0x280c <entry+0x280c>
    c.literal_cond(C::Al,R::R2,0x3c3dc)?;
    // 0x3bb1c: str r0, [sp, #0xd44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3396)?)?;
    // 0x3bb20: str r2, [sp, #0xd48]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3400)?)?;
    // 0x3bb24: mov r2, #600
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,600)?)?;
    // 0x3bb28: str r1, [sp, #0xd4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3404)?)?;
    // 0x3bb2c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bb30: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bb34: str r2, [sp, #0xd50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3408)?)?;
    // 0x3bb38: ldr r2, [pc, #0x8a0]        @ 0x2810 <entry+0x2810>
    c.literal_cond(C::Al,R::R2,0x3c3e0)?;
    // 0x3bb3c: str r0, [sp, #0xd54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3412)?)?;
    // 0x3bb40: str r2, [sp, #0xd58]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3416)?)?;
    // 0x3bb44: mov r2, #604
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,604)?)?;
    // 0x3bb48: str r1, [sp, #0xd5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3420)?)?;
    // 0x3bb4c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bb50: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bb54: str r2, [sp, #0xd60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3424)?)?;
    // 0x3bb58: ldr r2, [pc, #0x884]        @ 0x2814 <entry+0x2814>
    c.literal_cond(C::Al,R::R2,0x3c3e4)?;
    // 0x3bb5c: str r0, [sp, #0xd64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3428)?)?;
    // 0x3bb60: str r2, [sp, #0xd68]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3432)?)?;
    // 0x3bb64: mov r2, #608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,608)?)?;
    // 0x3bb68: str r1, [sp, #0xd6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3436)?)?;
    // 0x3bb6c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bb70: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bb74: str r2, [sp, #0xd70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3440)?)?;
    // 0x3bb78: ldr r2, [pc, #0x868]        @ 0x2818 <entry+0x2818>
    c.literal_cond(C::Al,R::R2,0x3c3e8)?;
    // 0x3bb7c: str r0, [sp, #0xd74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3444)?)?;
    // 0x3bb80: str r2, [sp, #0xd78]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3448)?)?;
    // 0x3bb84: mov r2, #612
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,612)?)?;
    // 0x3bb88: str r1, [sp, #0xd7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3452)?)?;
    // 0x3bb8c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bb90: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bb94: str r2, [sp, #0xd80]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3456)?)?;
    // 0x3bb98: ldr r2, [pc, #0x84c]        @ 0x281c <entry+0x281c>
    c.literal_cond(C::Al,R::R2,0x3c3ec)?;
    // 0x3bb9c: str r0, [sp, #0xd84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3460)?)?;
    // 0x3bba0: str r2, [sp, #0xd88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3464)?)?;
    // 0x3bba4: mov r2, #616
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,616)?)?;
    // 0x3bba8: str r1, [sp, #0xd8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3468)?)?;
    // 0x3bbac: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bbb0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bbb4: str r2, [sp, #0xd90]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3472)?)?;
    // 0x3bbb8: ldr r2, [pc, #0x830]        @ 0x2820 <entry+0x2820>
    c.literal_cond(C::Al,R::R2,0x3c3f0)?;
    // 0x3bbbc: str r0, [sp, #0xd94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3476)?)?;
    // 0x3bbc0: str r2, [sp, #0xd98]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3480)?)?;
    // 0x3bbc4: mov r2, #620
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,620)?)?;
    // 0x3bbc8: str r1, [sp, #0xd9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3484)?)?;
    // 0x3bbcc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bbd0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bbd4: str r2, [sp, #0xda0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3488)?)?;
    // 0x3bbd8: ldr r2, [pc, #0x814]        @ 0x2824 <entry+0x2824>
    c.literal_cond(C::Al,R::R2,0x3c3f4)?;
    // 0x3bbdc: str r0, [sp, #0xda4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3492)?)?;
    // 0x3bbe0: str r2, [sp, #0xda8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3496)?)?;
    // 0x3bbe4: mov r2, #624
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,624)?)?;
    // 0x3bbe8: str r1, [sp, #0xdac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3500)?)?;
    // 0x3bbec: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bbf0: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bbf4: str r2, [sp, #0xdb0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3504)?)?;
    // 0x3bbf8: ldr r2, [pc, #0x7f8]        @ 0x2828 <entry+0x2828>
    c.literal_cond(C::Al,R::R2,0x3c3f8)?;
    // 0x3bbfc: str r0, [sp, #0xdb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3508)?)?;
    // 0x3bc00: str r2, [sp, #0xdb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3512)?)?;
    // 0x3bc04: mov r2, #628
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,628)?)?;
    // 0x3bc08: str r1, [sp, #0xdbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3516)?)?;
    // 0x3bc0c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bc10: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bc14: str r2, [sp, #0xdc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3520)?)?;
    // 0x3bc18: ldr r2, [pc, #0x7dc]        @ 0x282c <entry+0x282c>
    c.literal_cond(C::Al,R::R2,0x3c3fc)?;
    // 0x3bc1c: str r0, [sp, #0xdc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3524)?)?;
    // 0x3bc20: str r2, [sp, #0xdc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3528)?)?;
    // 0x3bc24: mov r2, #632
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,632)?)?;
    // 0x3bc28: str r1, [sp, #0xdcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3532)?)?;
    // 0x3bc2c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bc30: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bc34: str r2, [sp, #0xdd0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3536)?)?;
    // 0x3bc38: ldr r2, [pc, #0x7c0]        @ 0x2830 <entry+0x2830>
    c.literal_cond(C::Al,R::R2,0x3c400)?;
    // 0x3bc3c: str r0, [sp, #0xdd4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3540)?)?;
    // 0x3bc40: str r2, [sp, #0xdd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3544)?)?;
    // 0x3bc44: mov r2, #636
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,636)?)?;
    // 0x3bc48: str r1, [sp, #0xddc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3548)?)?;
    // 0x3bc4c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bc50: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bc54: str r2, [sp, #0xde0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3552)?)?;
    // 0x3bc58: ldr r2, [pc, #0x7a4]        @ 0x2834 <entry+0x2834>
    c.literal_cond(C::Al,R::R2,0x3c404)?;
    // 0x3bc5c: str r0, [sp, #0xde4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3556)?)?;
    // 0x3bc60: str r2, [sp, #0xde8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3560)?)?;
    // 0x3bc64: add r2, r6, #4736
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,4736)?)?;
    // 0x3bc68: str r1, [sp, #0xdec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3564)?)?;
    // 0x3bc6c: str r2, [sp, #0xdf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3568)?)?;
    // 0x3bc70: ldr r2, [pc, #0x790]        @ 0x2838 <entry+0x2838>
    c.literal_cond(C::Al,R::R2,0x3c408)?;
    // 0x3bc74: str r0, [sp, #0xdf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3572)?)?;
    // 0x3bc78: str r2, [sp, #0xdf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3576)?)?;
    // 0x3bc7c: mov r2, #644
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,644)?)?;
    // 0x3bc80: str r1, [sp, #0xdfc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3580)?)?;
    // 0x3bc84: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bc88: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bc8c: str r2, [sp, #0xe00]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3584)?)?;
    // 0x3bc90: ldr r2, [pc, #0x774]        @ 0x283c <entry+0x283c>
    c.literal_cond(C::Al,R::R2,0x3c40c)?;
    // 0x3bc94: str r0, [sp, #0xe04]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3588)?)?;
    // 0x3bc98: str r2, [sp, #0xe08]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3592)?)?;
    // 0x3bc9c: mov r2, #648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,648)?)?;
    // 0x3bca0: str r1, [sp, #0xe0c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3596)?)?;
    // 0x3bca4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bca8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bcac: str r2, [sp, #0xe10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3600)?)?;
    // 0x3bcb0: ldr r2, [pc, #0x758]        @ 0x2840 <entry+0x2840>
    c.literal_cond(C::Al,R::R2,0x3c410)?;
    // 0x3bcb4: str r0, [sp, #0xe14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3604)?)?;
    // 0x3bcb8: str r2, [sp, #0xe18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3608)?)?;
    // 0x3bcbc: mov r2, #652
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,652)?)?;
    // 0x3bcc0: str r1, [sp, #0xe1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3612)?)?;
    // 0x3bcc4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bcc8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bccc: str r2, [sp, #0xe20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3616)?)?;
    // 0x3bcd0: ldr r2, [pc, #0x73c]        @ 0x2844 <entry+0x2844>
    c.literal_cond(C::Al,R::R2,0x3c414)?;
    // 0x3bcd4: str r0, [sp, #0xe24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3620)?)?;
    // 0x3bcd8: str r2, [sp, #0xe28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3624)?)?;
    // 0x3bcdc: mov r2, #656
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,656)?)?;
    // 0x3bce0: str r1, [sp, #0xe2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3628)?)?;
    // 0x3bce4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bce8: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bcec: str r2, [sp, #0xe30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3632)?)?;
    // 0x3bcf0: ldr r2, [pc, #0x720]        @ 0x2848 <entry+0x2848>
    c.literal_cond(C::Al,R::R2,0x3c418)?;
    // 0x3bcf4: str r0, [sp, #0xe34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3636)?)?;
    // 0x3bcf8: str r2, [sp, #0xe38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3640)?)?;
    // 0x3bcfc: mov r2, #660
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,660)?)?;
    // 0x3bd00: str r1, [sp, #0xe3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3644)?)?;
    // 0x3bd04: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bd08: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bd0c: str r2, [sp, #0xe40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3648)?)?;
    // 0x3bd10: ldr r2, [pc, #0x704]        @ 0x284c <entry+0x284c>
    c.literal_cond(C::Al,R::R2,0x3c41c)?;
    // 0x3bd14: str r0, [sp, #0xe44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3652)?)?;
    // 0x3bd18: str r2, [sp, #0xe48]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3656)?)?;
    // 0x3bd1c: mov r2, #664
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,664)?)?;
    // 0x3bd20: str r1, [sp, #0xe4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3660)?)?;
    // 0x3bd24: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bd28: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bd2c: str r2, [sp, #0xe50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3664)?)?;
    // 0x3bd30: ldr r2, [pc, #0x6e8]        @ 0x2850 <entry+0x2850>
    c.literal_cond(C::Al,R::R2,0x3c420)?;
    // 0x3bd34: str r0, [sp, #0xe54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3668)?)?;
    // 0x3bd38: str r2, [sp, #0xe58]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3672)?)?;
    // 0x3bd3c: mov r2, #668
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,668)?)?;
    // 0x3bd40: str r1, [sp, #0xe5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3676)?)?;
    // 0x3bd44: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bd48: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bd4c: str r2, [sp, #0xe60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3680)?)?;
    // 0x3bd50: ldr r2, [pc, #0x6cc]        @ 0x2854 <entry+0x2854>
    c.literal_cond(C::Al,R::R2,0x3c424)?;
    // 0x3bd54: str r0, [sp, #0xe64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3684)?)?;
    // 0x3bd58: str r2, [sp, #0xe68]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3688)?)?;
    // 0x3bd5c: mov r2, #672
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,672)?)?;
    // 0x3bd60: str r1, [sp, #0xe6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3692)?)?;
    // 0x3bd64: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bd68: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bd6c: str r2, [sp, #0xe70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3696)?)?;
    // 0x3bd70: ldr r2, [pc, #0x6b0]        @ 0x2858 <entry+0x2858>
    c.literal_cond(C::Al,R::R2,0x3c428)?;
    // 0x3bd74: str r0, [sp, #0xe74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3700)?)?;
    // 0x3bd78: str r2, [sp, #0xe78]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3704)?)?;
    // 0x3bd7c: mov r2, #676
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,676)?)?;
    // 0x3bd80: str r1, [sp, #0xe7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3708)?)?;
    // 0x3bd84: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bd88: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bd8c: str r2, [sp, #0xe80]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3712)?)?;
    // 0x3bd90: ldr r2, [pc, #0x694]        @ 0x285c <entry+0x285c>
    c.literal_cond(C::Al,R::R2,0x3c42c)?;
    // 0x3bd94: str r0, [sp, #0xe84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3716)?)?;
    // 0x3bd98: str r2, [sp, #0xe88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3720)?)?;
    // 0x3bd9c: add r2, r6, #3936
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,3936)?)?;
    // 0x3bda0: str r12, [sp, #0xe8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,3724)?)?;
    // 0x3bda4: str r2, [sp, #0xe90]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3728)?)?;
    // 0x3bda8: ldr r2, [pc, #0x680]        @ 0x2860 <entry+0x2860>
    c.literal_cond(C::Al,R::R2,0x3c430)?;
    // 0x3bdac: str r0, [sp, #0xe94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3732)?)?;
    // 0x3bdb0: str r2, [sp, #0xe98]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3736)?)?;
    // 0x3bdb4: add r2, r6, #860
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,860)?)?;
    // 0x3bdb8: str r12, [sp, #0xe9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,3740)?)?;
    // 0x3bdbc: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3bdc0: str r2, [sp, #0xea0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3744)?)?;
    // 0x3bdc4: ldr r2, [pc, #0x668]        @ 0x2864 <entry+0x2864>
    c.literal_cond(C::Al,R::R2,0x3c434)?;
    // 0x3bdc8: str r0, [sp, #0xea4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3748)?)?;
    // 0x3bdcc: str r2, [sp, #0xea8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3752)?)?;
    // 0x3bdd0: add r2, r6, #856
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,856)?)?;
    // 0x3bdd4: str r12, [sp, #0xeac]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,3756)?)?;
    // 0x3bdd8: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3bddc: str r2, [sp, #0xeb0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3760)?)?;
    // 0x3bde0: ldr r2, [pc, #0x650]        @ 0x2868 <entry+0x2868>
    c.literal_cond(C::Al,R::R2,0x3c438)?;
    // 0x3bde4: str r0, [sp, #0xeb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3764)?)?;
    // 0x3bde8: str r2, [sp, #0xeb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3768)?)?;
    // 0x3bdec: add r2, r6, #852
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,852)?)?;
    // 0x3bdf0: str r12, [sp, #0xebc]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,3772)?)?;
    // 0x3bdf4: mov r12, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,88)?)?;
    // 0x3bdf8: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,3072)?)?;
    // 0x3bdfc: orr r12, r12, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4096)?)?;
    // 0x3be00: str r2, [sp, #0xec0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3776)?)?;
    // 0x3be04: ldr r2, [pc, #0x630]        @ 0x286c <entry+0x286c>
    c.literal_cond(C::Al,R::R2,0x3c43c)?;
    // 0x3be08: str r0, [sp, #0xec4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3780)?)?;
    // 0x3be0c: str r2, [sp, #0xec8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3784)?)?;
    // 0x3be10: mov r2, #904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,904)?)?;
    // 0x3be14: str r0, [sp, #0xecc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3788)?)?;
    // 0x3be18: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3be1c: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3be20: str r2, [sp, #0xed0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3792)?)?;
    // 0x3be24: ldr r2, [pc, #0x614]        @ 0x2870 <entry+0x2870>
    c.literal_cond(C::Al,R::R2,0x3c440)?;
    // 0x3be28: str r0, [sp, #0xed4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3796)?)?;
    // 0x3be2c: str r2, [sp, #0xed8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3800)?)?;
    // 0x3be30: mov r2, #908
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,908)?)?;
    // 0x3be34: str r1, [sp, #0xedc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3804)?)?;
    // 0x3be38: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3be3c: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3be40: str r2, [sp, #0xee0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3808)?)?;
    // 0x3be44: ldr r2, [pc, #0x5f8]        @ 0x2874 <entry+0x2874>
    c.literal_cond(C::Al,R::R2,0x3c444)?;
    // 0x3be48: str r0, [sp, #0xee4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3812)?)?;
    // 0x3be4c: str r2, [sp, #0xee8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3816)?)?;
    // 0x3be50: mov r2, #912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,912)?)?;
    // 0x3be54: str r1, [sp, #0xeec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,3820)?)?;
    // 0x3be58: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3be5c: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3be60: str r2, [sp, #0xef0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3824)?)?;
    // 0x3be64: ldr r2, [pc, #0x5dc]        @ 0x2878 <entry+0x2878>
    c.literal_cond(C::Al,R::R2,0x3c448)?;
    // 0x3be68: str r0, [sp, #0xef4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3828)?)?;
    // 0x3be6c: str r2, [sp, #0xef8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3832)?)?;
    // 0x3be70: add r2, r6, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,4864)?)?;
    // 0x3be74: str r0, [sp, #0xefc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3836)?)?;
    // 0x3be78: str r2, [sp, #0xf00]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3840)?)?;
    // 0x3be7c: ldr r2, [pc, #0x5c8]        @ 0x287c <entry+0x287c>
    c.literal_cond(C::Al,R::R2,0x3c44c)?;
    // 0x3be80: str r0, [sp, #0xf04]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3844)?)?;
    // 0x3be84: str r2, [sp, #0xf08]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3848)?)?;
    // 0x3be88: mov r2, #772
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,772)?)?;
    // 0x3be8c: str r0, [sp, #0xf0c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3852)?)?;
    // 0x3be90: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3be94: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3be98: str r2, [sp, #0xf10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3856)?)?;
    // 0x3be9c: ldr r2, [pc, #0x5ac]        @ 0x2880 <entry+0x2880>
    c.literal_cond(C::Al,R::R2,0x3c450)?;
    // 0x3bea0: str r0, [sp, #0xf14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3860)?)?;
    // 0x3bea4: str r2, [sp, #0xf18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3864)?)?;
    // 0x3bea8: mov r2, #776
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,776)?)?;
    // 0x3beac: str r0, [sp, #0xf1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3868)?)?;
    // 0x3beb0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3beb4: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3beb8: str r2, [sp, #0xf20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3872)?)?;
    // 0x3bebc: ldr r2, [pc, #0x590]        @ 0x2884 <entry+0x2884>
    c.literal_cond(C::Al,R::R2,0x3c454)?;
    // 0x3bec0: str r0, [sp, #0xf24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3876)?)?;
    // 0x3bec4: str r2, [sp, #0xf28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3880)?)?;
    // 0x3bec8: mov r2, #780
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,780)?)?;
    // 0x3becc: str r0, [sp, #0xf2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3884)?)?;
    // 0x3bed0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bed4: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bed8: str r2, [sp, #0xf30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3888)?)?;
    // 0x3bedc: ldr r2, [pc, #0x574]        @ 0x2888 <entry+0x2888>
    c.literal_cond(C::Al,R::R2,0x3c458)?;
    // 0x3bee0: str r0, [sp, #0xf34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3892)?)?;
    // 0x3bee4: str r2, [sp, #0xf38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3896)?)?;
    // 0x3bee8: mov r2, #784
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,784)?)?;
    // 0x3beec: str r0, [sp, #0xf3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3900)?)?;
    // 0x3bef0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bef4: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bef8: str r2, [sp, #0xf40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3904)?)?;
    // 0x3befc: ldr r2, [pc, #0x558]        @ 0x288c <entry+0x288c>
    c.literal_cond(C::Al,R::R2,0x3c45c)?;
    // 0x3bf00: str r0, [sp, #0xf44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3908)?)?;
    // 0x3bf04: str r2, [sp, #0xf48]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3912)?)?;
    // 0x3bf08: mov r2, #788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,788)?)?;
    // 0x3bf0c: str r0, [sp, #0xf4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3916)?)?;
    // 0x3bf10: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bf14: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bf18: str r2, [sp, #0xf50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3920)?)?;
    // 0x3bf1c: ldr r2, [pc, #0x53c]        @ 0x2890 <entry+0x2890>
    c.literal_cond(C::Al,R::R2,0x3c460)?;
    // 0x3bf20: str r0, [sp, #0xf54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3924)?)?;
    // 0x3bf24: str r2, [sp, #0xf58]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3928)?)?;
    // 0x3bf28: mov r2, #792
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,792)?)?;
    // 0x3bf2c: str r0, [sp, #0xf5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3932)?)?;
    // 0x3bf30: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bf34: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bf38: str r2, [sp, #0xf60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3936)?)?;
    // 0x3bf3c: ldr r2, [pc, #0x520]        @ 0x2894 <entry+0x2894>
    c.literal_cond(C::Al,R::R2,0x3c464)?;
    // 0x3bf40: str r0, [sp, #0xf64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3940)?)?;
    // 0x3bf44: str r2, [sp, #0xf68]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3944)?)?;
    // 0x3bf48: mov r2, #796
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,796)?)?;
    // 0x3bf4c: str r0, [sp, #0xf6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3948)?)?;
    // 0x3bf50: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bf54: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bf58: str r2, [sp, #0xf70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3952)?)?;
    // 0x3bf5c: ldr r2, [pc, #0x504]        @ 0x2898 <entry+0x2898>
    c.literal_cond(C::Al,R::R2,0x3c468)?;
    // 0x3bf60: str r0, [sp, #0xf74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3956)?)?;
    // 0x3bf64: str r2, [sp, #0xf78]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3960)?)?;
    // 0x3bf68: mov r2, #800
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,800)?)?;
    // 0x3bf6c: str r0, [sp, #0xf7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3964)?)?;
    // 0x3bf70: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bf74: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bf78: str r2, [sp, #0xf80]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3968)?)?;
    // 0x3bf7c: ldr r2, [pc, #0x4e8]        @ 0x289c <entry+0x289c>
    c.literal_cond(C::Al,R::R2,0x3c46c)?;
    // 0x3bf80: str r0, [sp, #0xf84]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3972)?)?;
    // 0x3bf84: str r2, [sp, #0xf88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3976)?)?;
    // 0x3bf88: mov r2, #804
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,804)?)?;
    // 0x3bf8c: str r0, [sp, #0xf8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3980)?)?;
    // 0x3bf90: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bf94: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bf98: str r2, [sp, #0xf90]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3984)?)?;
    // 0x3bf9c: ldr r2, [pc, #0x4cc]        @ 0x28a0 <entry+0x28a0>
    c.literal_cond(C::Al,R::R2,0x3c470)?;
    // 0x3bfa0: str r0, [sp, #0xf94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3988)?)?;
    // 0x3bfa4: str r2, [sp, #0xf98]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,3992)?)?;
    // 0x3bfa8: mov r2, #808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,808)?)?;
    // 0x3bfac: str r0, [sp, #0xf9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,3996)?)?;
    // 0x3bfb0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bfb4: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bfb8: str r2, [sp, #0xfa0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4000)?)?;
    // 0x3bfbc: ldr r2, [pc, #0x4b0]        @ 0x28a4 <entry+0x28a4>
    c.literal_cond(C::Al,R::R2,0x3c474)?;
    // 0x3bfc0: str r0, [sp, #0xfa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4004)?)?;
    // 0x3bfc4: str r2, [sp, #0xfa8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4008)?)?;
    // 0x3bfc8: mov r2, #812
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,812)?)?;
    // 0x3bfcc: str r0, [sp, #0xfac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4012)?)?;
    // 0x3bfd0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bfd4: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bfd8: str r2, [sp, #0xfb0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4016)?)?;
    // 0x3bfdc: ldr r2, [pc, #0x494]        @ 0x28a8 <entry+0x28a8>
    c.literal_cond(C::Al,R::R2,0x3c478)?;
    // 0x3bfe0: str r0, [sp, #0xfb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4020)?)?;
    // 0x3bfe4: str r2, [sp, #0xfb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4024)?)?;
    // 0x3bfe8: mov r2, #820
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,820)?)?;
    // 0x3bfec: str r0, [sp, #0xfbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4028)?)?;
    // 0x3bff0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3bff4: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3bff8: str r2, [sp, #0xfc0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4032)?)?;
    // 0x3bffc: ldr r2, [pc, #0x478]        @ 0x28ac <entry+0x28ac>
    c.literal_cond(C::Al,R::R2,0x3c47c)?;
    // 0x3c000: str r0, [sp, #0xfc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4036)?)?;
    // 0x3c004: str r2, [sp, #0xfc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4040)?)?;
    // 0x3c008: add r2, r6, #53
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,53)?)?;
    // 0x3c00c: str r0, [sp, #0xfcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4044)?)?;
    // 0x3c010: add r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4864)?)?;
    // 0x3c014: str r2, [sp, #0xfd0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4048)?)?;
    // 0x3c018: ldr r2, [pc, #0x460]        @ 0x28b0 <entry+0x28b0>
    c.literal_cond(C::Al,R::R2,0x3c480)?;
    // 0x3c01c: str r0, [sp, #0xfd4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4052)?)?;
    // 0x3c020: str r2, [sp, #0xfd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4056)?)?;
    // 0x3c024: add r2, r6, #54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,54)?)?;
    // 0x3c028: str r0, [sp, #0xfdc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4060)?)?;
    // 0x3c02c: add r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4864)?)?;
    // 0x3c030: str r2, [sp, #0xfe0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4064)?)?;
    // 0x3c034: ldr r2, [pc, #0x448]        @ 0x28b4 <entry+0x28b4>
    c.literal_cond(C::Al,R::R2,0x3c484)?;
    // 0x3c038: str r0, [sp, #0xfe4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4068)?)?;
    // 0x3c03c: str r2, [sp, #0xfe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4072)?)?;
    // 0x3c040: add r2, r6, #55
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,55)?)?;
    // 0x3c044: str r0, [sp, #0xfec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4076)?)?;
    // 0x3c048: add r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4864)?)?;
    // 0x3c04c: str r2, [sp, #0xff0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4080)?)?;
    // 0x3c050: ldr r2, [pc, #0x430]        @ 0x28b8 <entry+0x28b8>
    c.literal_cond(C::Al,R::R2,0x3c488)?;
    // 0x3c054: str r0, [sp, #0xff4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4084)?)?;
    // 0x3c058: str r2, [sp, #0xff8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4088)?)?;
    // 0x3c05c: mov r2, #824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,824)?)?;
    // 0x3c060: str r0, [sp, #0xffc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4092)?)?;
    // 0x3c064: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c068: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3c06c: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x3c070: ldr r2, [pc, #0x414]        @ 0x28bc <entry+0x28bc>
    c.literal_cond(C::Al,R::R2,0x3c48c)?;
    // 0x3c074: add lr, sp, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::SP,4096)?)?;
    // 0x3c078: str r0, [lr, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,4)?)?;
    // 0x3c07c: add lr, sp, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::SP,4096)?)?;
    // 0x3c080: str r2, [lr, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,8)?)?;
    // 0x3c084: add r2, r6, #57
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,57)?)?;
    // 0x3c088: add lr, sp, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::SP,4096)?)?;
    // 0x3c08c: str r0, [lr, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,12)?)?;
    // 0x3c090: add r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4864)?)?;
    // 0x3c094: add lr, sp, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::SP,4096)?)?;
    // 0x3c098: str r2, [lr, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,16)?)?;
    // 0x3c09c: add lr, sp, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::SP,4096)?)?;
    // 0x3c0a0: mov r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4096)?)?;
    // 0x3c0a4: str r0, [lr, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::LR,20)?)?;
    // 0x3c0a8: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c0ac: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x3c0b0: mov r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8)?)?;
    // 0x3c0b4: mov lr, #92
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,92)?)?;
    // 0x3c0b8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c0bc: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c0c0: orr lr, lr, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,4096)?)?;
    // 0x3c0c4: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c0c8: add r2, r6, #58
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,58)?)?;
    // 0x3c0cc: add r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4864)?)?;
    // 0x3c0d0: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c0d4: ldr r3, [pc, #0x3b4]        @ 0x28c0 <entry+0x28c0>
    c.literal_cond(C::Al,R::new(3)?,0x3c490)?;
    // 0x3c0d8: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x3c0dc: str r0, [r10, r8]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x3c0e0: mov r8, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,100)?)?;
    // 0x3c0e4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c0e8: orr r8, r8, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,4096)?)?;
    // 0x3c0ec: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c0f0: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x3c0f4: mov r3, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,24)?)?;
    // 0x3c0f8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c0fc: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c100: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c104: add r2, r6, #59
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,59)?)?;
    // 0x3c108: add r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4864)?)?;
    // 0x3c10c: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c110: mov r2, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // 0x3c114: ldr r3, [pc, #0x378]        @ 0x28c4 <entry+0x28c4>
    c.literal_cond(C::Al,R::new(3)?,0x3c494)?;
    // 0x3c118: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c11c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c120: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x3c124: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c128: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c12c: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x3c130: mov r3, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,40)?)?;
    // 0x3c134: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c138: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c13c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c140: mov r2, #828
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,828)?)?;
    // 0x3c144: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c148: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3c14c: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c150: mov r2, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,44)?)?;
    // 0x3c154: mov r3, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,48)?)?;
    // 0x3c158: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c15c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c160: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c164: ldr r2, [pc, #0x32c]        @ 0x28c8 <entry+0x28c8>
    c.literal_cond(C::Al,R::R2,0x3c498)?;
    // 0x3c168: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c16c: mov r2, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,52)?)?;
    // 0x3c170: mov r3, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,56)?)?;
    // 0x3c174: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c178: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c17c: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c180: add r2, r6, #61
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,61)?)?;
    // 0x3c184: add r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4864)?)?;
    // 0x3c188: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c18c: ldr r3, [pc, #0x308]        @ 0x28cc <entry+0x28cc>
    c.literal_cond(C::Al,R::new(3)?,0x3c49c)?;
    // 0x3c190: mov r2, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,60)?)?;
    // 0x3c194: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c198: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c19c: mov r2, #4160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4160)?)?;
    // 0x3c1a0: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c1a4: mov r2, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,68)?)?;
    // 0x3c1a8: mov r3, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,72)?)?;
    // 0x3c1ac: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c1b0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c1b4: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c1b8: add r2, r6, #62
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,62)?)?;
    // 0x3c1bc: add r2, r2, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4864)?)?;
    // 0x3c1c0: str r2, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c1c4: mov r2, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,76)?)?;
    // 0x3c1c8: ldr r3, [pc, #0x2d0]        @ 0x28d0 <entry+0x28d0>
    c.literal_cond(C::Al,R::new(3)?,0x3c4a0)?;
    // 0x3c1cc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c1d0: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c1d4: mov r2, #80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,80)?)?;
    // 0x3c1d8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c1dc: str r3, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c1e0: mov r2, #84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,84)?)?;
    // 0x3c1e4: add r3, r6, #63
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,63)?)?;
    // 0x3c1e8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3c1ec: add r3, r3, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4864)?)?;
    // 0x3c1f0: str r0, [r10, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3c1f4: str r3, [r10, r12]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::R12,Shift::Lsl,0)?)?;
    // 0x3c1f8: ldr r3, [pc, #0x2a4]        @ 0x28d4 <entry+0x28d4>
    c.literal_cond(C::Al,R::new(3)?,0x3c4a4)?;
    // 0x3c1fc: str r0, [r10, lr]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::LR,Shift::Lsl,0)?)?;
    // 0x3c200: add r2, r6, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3c204: str r3, [r10, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x3c208: add r3, r6, #4928
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,4928)?)?;
    // 0x3c20c: str r0, [r10, r8]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x3c210: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c214: mov r3, #108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,108)?)?;
    // 0x3c218: mov r4, #112
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,112)?)?;
    // 0x3c21c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c220: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c224: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c228: ldr r3, [pc, #0x278]        @ 0x28d8 <entry+0x28d8>
    c.literal_cond(C::Al,R::new(3)?,0x3c4a8)?;
    // 0x3c22c: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c230: mov r3, #116
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,116)?)?;
    // 0x3c234: mov r4, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,120)?)?;
    // 0x3c238: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c23c: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c240: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c244: add r3, r6, #65
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,65)?)?;
    // 0x3c248: add r3, r3, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4864)?)?;
    // 0x3c24c: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c250: ldr r4, [pc, #0x254]        @ 0x28dc <entry+0x28dc>
    c.literal_cond(C::Al,R::new(4)?,0x3c4ac)?;
    // 0x3c254: mov r3, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,124)?)?;
    // 0x3c258: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c25c: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c260: mov r3, #4224
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4224)?)?;
    // 0x3c264: str r4, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c268: mov r3, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,132)?)?;
    // 0x3c26c: mov r4, #136
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,136)?)?;
    // 0x3c270: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c274: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c278: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c27c: add r3, r6, #66
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,66)?)?;
    // 0x3c280: add r3, r3, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4864)?)?;
    // 0x3c284: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c288: mov r3, #140
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,140)?)?;
    // 0x3c28c: mov r4, #144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,144)?)?;
    // 0x3c290: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c294: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c298: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c29c: ldr r3, [pc, #0x20c]        @ 0x28e0 <entry+0x28e0>
    c.literal_cond(C::Al,R::new(3)?,0x3c4b0)?;
    // 0x3c2a0: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c2a4: mov r3, #148
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,148)?)?;
    // 0x3c2a8: mov r4, #152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,152)?)?;
    // 0x3c2ac: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x3c2b0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x3c2b4: str r0, [r10, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3c2b8: add r3, r6, #67
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,67)?)?;
    // 0x3c2bc: add r3, r3, #4864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4864)?)?;
    // 0x3c2c0: str r3, [r10, r4]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(10)?,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3c2c4: mov r3, #156
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,156)?)?;
    // 0x3c2c8: mov r4, #160
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,160)?)?;
    // 0x3c2cc: b 0x3c53c <entry+0x296c>  @ imm = #0x268
    c.b(C::Al,0x3c53c)?;
    c.check(0x3c2d0)?;
    for address in STREAM_OUTPUT_DIAGNOSTICS {c.emit(address)?;}
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{append_stream_descriptor_matrix(c)}

#[cfg(test)]mod tests {
 use super::*;
 #[test]fn matrix_tail_and_diagnostic_table_contiguous(){
    let mut c=Code::new(START);emit(&mut c).unwrap();
    assert_eq!(c.bytes.len(),END-START);
    assert_eq!(STREAM_OUTPUT_DIAGNOSTICS.len(),155);
    assert_eq!(STREAM_OUTPUT_DIAGNOSTICS[0],0x000e_cd38);
    assert_eq!(STREAM_OUTPUT_DIAGNOSTICS[154],0x000e_d664);
 }
}
