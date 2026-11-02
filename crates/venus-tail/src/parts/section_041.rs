
/// Reconstruct controller input/output offset calculations, mode-specific
/// buffer validation, codec event callbacks and dispatch. The original
/// A32 instruction sequence is structurally grouped while its five literal
/// pools (including the six-way handler table) are non-executable data.
fn build_controller_plane_layout_and_event_dispatch(c:&mut Code)->Result<(),String>{
    c.check(0xb68bc)?;
    // VMA 0xb68bc
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb68c0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xb68c4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // VMA 0xb68c8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // VMA 0xb68cc
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // VMA 0xb68d0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xb68d4
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // VMA 0xb68d8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // VMA 0xb68dc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // VMA 0xb68e0
    c.b(C::Al,0x81850,true)?;
    // VMA 0xb68e4
    c.literal(R::R2,0xb6ab4)?;
    // VMA 0xb68e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(5)?,4)?)?;
    // VMA 0xb68ec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xb68f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,34&0xffffffff)?)?;
    // VMA 0xb68f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb68f8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb68fc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // VMA 0xb6900
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb6904
    c.literal(R::R1,0xb6ab8)?;
    // VMA 0xb6908
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb690c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6910
    c.literal(R::R1,0xb6abc)?;
    // VMA 0xb6914
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb6918
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb691c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,972)?)?;
    // VMA 0xb6920
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(4)?))?;
    // VMA 0xb6924
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb6928
    c.b(C::Eq,0xb6a90,false)?;
    // VMA 0xb692c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::LR,R::new(6)?,1)?)?;
    // VMA 0xb6930
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::LR,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6934
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::new(9)?,R::new(6)?,0)?)?;
    // VMA 0xb6938
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb693c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,2008)?)?;
    // VMA 0xb6940
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb6944
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,2664)?)?;
    // VMA 0xb6948
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb694c
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::R2,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6950
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6954
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2012)?)?;
    // VMA 0xb6958
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,2664)?)?;
    // VMA 0xb695c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2724)?)?;
    // VMA 0xb6960
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // VMA 0xb6964
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // VMA 0xb6968
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R12,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb696c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6970
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb6974
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb6978
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(3)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb697c
    c.b(C::Eq,0xb6a04,false)?;
    // VMA 0xb6980
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(5)?,2048)?)?;
    // VMA 0xb6984
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,512&0xffffffff)?)?;
    // VMA 0xb6988
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,-32)?)?;
    // VMA 0xb698c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb6990
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(8)?,2668)?)?;
    // VMA 0xb6994
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6998
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(6)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb699c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,-24)?)?;
    // VMA 0xb69a0
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,2668)?)?;
    // VMA 0xb69a4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb69a8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb69ac
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb69b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,-8)?)?;
    // VMA 0xb69b4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,2680)?)?;
    // VMA 0xb69b8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb69bc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb69c0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb69c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // VMA 0xb69c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4)?)?;
    // VMA 0xb69cc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,2680)?)?;
    // VMA 0xb69d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(8)?,2724)?)?;
    // VMA 0xb69d4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb69d8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb69dc
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb69e0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R12,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb69e4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb69e8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb69ec
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R12,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb69f0
    c.literal(R::R0,0xb6ac0)?;
    // VMA 0xb69f4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb69f8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xb69fc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(10)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a00
    c.b(C::Cc,0xb6988,false)?;
    // VMA 0xb6a04
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // VMA 0xb6a08
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a0c
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,2032)?)?;
    // VMA 0xb6a10
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(8)?,2676)?)?;
    // VMA 0xb6a14
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a18
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a1c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2036)?)?;
    // VMA 0xb6a20
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,2676)?)?;
    // VMA 0xb6a24
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a28
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a2c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a30
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2056)?)?;
    // VMA 0xb6a34
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,2604)?)?;
    // VMA 0xb6a38
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a3c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a40
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a44
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2060)?)?;
    // VMA 0xb6a48
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,2616)?)?;
    // VMA 0xb6a4c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(4)?,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a50
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a54
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a58
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2064)?)?;
    // VMA 0xb6a5c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(8)?,2628)?)?;
    // VMA 0xb6a60
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(5)?,R::LR,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a64
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a68
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(9)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a6c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,2068)?)?;
    // VMA 0xb6a70
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R12,R::new(6)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a74
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a78
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,2636)?)?;
    // VMA 0xb6a7c
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a80
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a84
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,8)?)?;
    // VMA 0xb6a88
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a8c
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xb6a90
    c.literal(R::R2,0xb6ab4)?;
    // VMA 0xb6a94
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6a98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,376&0xffffffff)?)?;
    // VMA 0xb6a9c
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::R0,R::new(7)?,Shift::Lsr,31,false)?)?;
    // VMA 0xb6aa0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // VMA 0xb6aa4
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb6aa8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(4)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6aac
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xb6ab0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb6ab4)?;
    const OUTPUT_PLANE_DIAGNOSTICS:[u32;4]=[0x000f8fbc,0x000f94c5,0x000e2788,0xfffffe01];
    for value in OUTPUT_PLANE_DIAGNOSTICS {c.emit(value)?;}
    c.check(0xb6ac4)?;
    // VMA 0xb6ac4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb6ac8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xb6acc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // VMA 0xb6ad0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xb6ad4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // VMA 0xb6ad8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2&0xffffffff)?)?;
    // VMA 0xb6adc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // VMA 0xb6ae0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,28)?)?;
    // VMA 0xb6ae4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb6ae8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xb6aec
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,1)?)?;
    // VMA 0xb6af0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // VMA 0xb6af4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32&0xffffffff)?)?;
    // VMA 0xb6af8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,3&0xffffffff)?)?;
    // VMA 0xb6afc
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Add,R::new(7)?,R::R1,R::R2,Shift::Lsl,R::new(3)?,false)?)?;
    // VMA 0xb6b00
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xb6b04
    c.b(C::Ne,0xb6b7c,false)?;
    // VMA 0xb6b08
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xb6b0c
    c.b(C::Al,0xd0f38,true)?;
    // VMA 0xb6b10
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // VMA 0xb6b14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // VMA 0xb6b18
    c.b(C::Ne,0xb6c20,false)?;
    // VMA 0xb6b1c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2072)?)?;
    // VMA 0xb6b20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb6b24
    c.b(C::Eq,0xb6bdc,false)?;
    // VMA 0xb6b28
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,32)?)?;
    // VMA 0xb6b2c
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xb6b30
    c.b(C::Eq,0xb6b6c,false)?;
    // VMA 0xb6b34
    c.literal(R::R1,0xb6c40)?;
    // VMA 0xb6b38
    c.literal(R::R2,0xb6c38)?;
    // VMA 0xb6b3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,9&0xffffffff)?)?;
    // VMA 0xb6b40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb6b44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // VMA 0xb6b48
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6b4c
    c.literal(R::R1,0xb6c3c)?;
    // VMA 0xb6b50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb6b54
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6b58
    c.literal(R::R2,0xb6c44)?;
    // VMA 0xb6b5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10&0xffffffff)?)?;
    // VMA 0xb6b60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6b64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // VMA 0xb6b68
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb6b6c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // VMA 0xb6b70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6b74
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,456)?)?;
    // VMA 0xb6b78
    c.b(C::Al,0xb6c20,false)?;
    // VMA 0xb6b7c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // VMA 0xb6b80
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,128)?)?;
    // VMA 0xb6b84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14&0xffffffff)?)?;
    // VMA 0xb6b88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6b8c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // VMA 0xb6b90
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // VMA 0xb6b94
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // VMA 0xb6b98
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // VMA 0xb6b9c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2&0xffffffff)?)?;
    // VMA 0xb6ba0
    c.b(C::Al,0x2a990,true)?;
    // VMA 0xb6ba4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb6ba8
    c.literal(R::R1,0xb6c34)?;
    // VMA 0xb6bac
    c.literal(R::R2,0xb6c38)?;
    // VMA 0xb6bb0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,242&0xffffffff)?)?;
    // VMA 0xb6bb4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // VMA 0xb6bb8
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xb6bbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb6bc0
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6bc4
    c.literal(R::R1,0xb6c3c)?;
    // VMA 0xb6bc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8&0xffffffff)?)?;
    // VMA 0xb6bcc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6bd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6bd4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // VMA 0xb6bd8
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb6bdc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,4)?)?;
    // VMA 0xb6be0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6be4
    c.literal(R::R2,0xb6c38)?;
    // VMA 0xb6be8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6bec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // VMA 0xb6bf0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,456)?)?;
    // VMA 0xb6bf4
    c.literal(R::R1,0xb6c48)?;
    // VMA 0xb6bf8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb6bfc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6c00
    c.literal(R::R1,0xb6c3c)?;
    // VMA 0xb6c04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb6c08
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6c0c
    c.literal(R::R2,0xb6c44)?;
    // VMA 0xb6c10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3&0xffffffff)?)?;
    // VMA 0xb6c14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6c18
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // VMA 0xb6c1c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb6c20
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xb6c24
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xb6c28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // VMA 0xb6c2c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb6c30
    c.b(C::Al,0xd2f4c,false)?;
    c.check(0xb6c34)?;
    const BUFFER_VALIDATION_MESSAGES:[u32;6]=[0x000f91e5,0x000f9231,0x000e2788,0x000f927a,0x000f8fbc,0x000f9250];
    for value in BUFFER_VALIDATION_MESSAGES {c.emit(value)?;}
    c.check(0xb6c4c)?;
    // VMA 0xb6c4c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6c50
    c.b(C::Al,0xd3d54,false)?;
    // VMA 0xb6c54
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // VMA 0xb6c58
    c.b(C::Al,0xd4268,false)?;
    // VMA 0xb6c5c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb6c60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xb6c64
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xb6c68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xb6c6c
    c.b(C::Al,0xd3c58,true)?;
    // VMA 0xb6c70
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xb6c74
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb6c78
    c.b(C::Ne,0xb6c8c,false)?;
    // VMA 0xb6c7c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6c80
    c.b(C::Al,0xb6c94,true)?;
    // VMA 0xb6c84
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6c88
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xb6c8c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xb6c90
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb6c94
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb6c98
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xb6c9c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xb6ca0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,356)?)?;
    // VMA 0xb6ca4
    c.b(C::Al,0x7e7d4,true)?;
    // VMA 0xb6ca8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,244)?)?;
    // VMA 0xb6cac
    c.b(C::Al,0x808d0,true)?;
    // VMA 0xb6cb0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,300)?)?;
    // VMA 0xb6cb4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb6cb8
    c.b(C::Al,0x808d0,false)?;
    // VMA 0xb6cbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6cc0
    c.emit(arm32::bx(C::Al,R::LR))?;
    // VMA 0xb6cc4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb6cc8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // VMA 0xb6ccc
    c.literal(R::R1,0xb6d28)?;
    // VMA 0xb6cd0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xb6cd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb6cd8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6cdc
    c.literal(R::R1,0xb6d2c)?;
    // VMA 0xb6ce0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb6ce4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6ce8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // VMA 0xb6cec
    c.b(C::Al,0xd3cac,true)?;
    // VMA 0xb6cf0
    c.literal(R::R2,0xb6d30)?;
    // VMA 0xb6cf4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xb6cf8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,114&0xffffffff)?)?;
    // VMA 0xb6cfc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6d00
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xb6d04
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // VMA 0xb6d08
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6d0c
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb6d10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6d14
    c.b(C::Al,0xb6c94,true)?;
    // VMA 0xb6d18
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6d1c
    c.b(C::Al,0x2f1c,true)?;
    // VMA 0xb6d20
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xb6d24
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb6d28)?;
    const BUFFER_ALLOCATOR_MESSAGES:[u32;3]=[0x000f92c6,0x000e2788,0x000f8fbc];
    for value in BUFFER_ALLOCATOR_MESSAGES {c.emit(value)?;}
    c.check(0xb6d34)?;
    // VMA 0xb6d34
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xb6d38
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xb6d3c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // VMA 0xb6d40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2&0xffffffff)?)?;
    // VMA 0xb6d44
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,4)?)?;
    // VMA 0xb6d48
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xb6d4c
    c.b(C::Al,0x3084,true)?;
    // VMA 0xb6d50
    c.literal(R::R2,0xb6ec8)?;
    // VMA 0xb6d54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,133&0xffffffff)?)?;
    // VMA 0xb6d58
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // VMA 0xb6d5c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6d60
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1&0xffffffff)?)?;
    // VMA 0xb6d64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // VMA 0xb6d68
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb6d6c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // VMA 0xb6d70
    c.b(C::Al,0x293c,true)?;
    // VMA 0xb6d74
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,104)?)?;
    // VMA 0xb6d78
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,9)?)?;
    // VMA 0xb6d7c
    c.b(C::Eq,0xb6d90,false)?;
    // VMA 0xb6d80
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // VMA 0xb6d84
    c.b(C::Eq,0xb6d9c,false)?;
    // VMA 0xb6d88
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // VMA 0xb6d8c
    c.b(C::Ne,0xb6da8,false)?;
    // VMA 0xb6d90
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6d94
    c.b(C::Al,0xb76f4,true)?;
    // VMA 0xb6d98
    c.b(C::Al,0xb6df0,false)?;
    // VMA 0xb6d9c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6da0
    c.b(C::Al,0xb79e8,true)?;
    // VMA 0xb6da4
    c.b(C::Al,0xb6df0,false)?;
    // VMA 0xb6da8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // VMA 0xb6dac
    c.b(C::Hi,0xb6dcc,false)?;
    // VMA 0xb6db0
    c.literal(R::R1,0xb6ecc)?;
    // VMA 0xb6db4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb6db8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6dbc
    c.literal(R::R1,0xb6ed0)?;
    // VMA 0xb6dc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4&0xffffffff)?)?;
    // VMA 0xb6dc4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6dc8
    c.b(C::Al,0xb6df0,false)?;
    // VMA 0xb6dcc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // VMA 0xb6dd0
    c.b(C::Eq,0xb6de8,false)?;
    // VMA 0xb6dd4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,10)?)?;
    // VMA 0xb6dd8
    c.b(C::Ne,0xb6df4,false)?;
    // VMA 0xb6ddc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6de0
    c.b(C::Al,0xb7b10,true)?;
    // VMA 0xb6de4
    c.b(C::Al,0xb6df0,false)?;
    // VMA 0xb6de8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6dec
    c.b(C::Al,0xb7b98,true)?;
    // VMA 0xb6df0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,104)?)?;
    // VMA 0xb6df4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // VMA 0xb6df8
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xb6dfc
    c.literal(R::new(8)?,0xb6ed4)?;
    // VMA 0xb6e00
    c.literal(R::new(9)?,0xb6ed8)?;
    // VMA 0xb6e04
    c.literal(R::new(5)?,0xb6ed0)?;
    // VMA 0xb6e08
    c.literal(R::new(6)?,0xb6ec8)?;
    // VMA 0xb6e0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,199&0xffffffff)?)?;
    // VMA 0xb6e10
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,768)?)?;
    // VMA 0xb6e14
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,228)?)?;
    // VMA 0xb6e18
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // VMA 0xb6e1c
    c.b(C::Hi,0xb6e68,false)?;
    // VMA 0xb6e20
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // VMA 0xb6e24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // VMA 0xb6e28
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0xb6e2c)?;
    const MODE_HANDLERS:[u32;6]=[0x000b6e44,0x000b6e9c,0x000b6e50,0x000b6ea8,0x000b6e5c,0x000b6eb4];
    for value in MODE_HANDLERS {c.emit(value)?;}
    c.check(0xb6e44)?;
    // VMA 0xb6e44
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6e48
    c.b(C::Al,0xb7cb4,true)?;
    // VMA 0xb6e4c
    c.b(C::Al,0xb6ebc,false)?;
    // VMA 0xb6e50
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6e54
    c.b(C::Al,0xb80d8,true)?;
    // VMA 0xb6e58
    c.b(C::Al,0xb6ebc,false)?;
    // VMA 0xb6e5c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6e60
    c.b(C::Al,0xb86c8,true)?;
    // VMA 0xb6e64
    c.b(C::Al,0xb6ebc,false)?;
    // VMA 0xb6e68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb6e6c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xb6e70
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // VMA 0xb6e74
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // VMA 0xb6e78
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6e7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16&0xffffffff)?)?;
    // VMA 0xb6e80
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xb6e84
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xb6e88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0&0xffffffff)?)?;
    // VMA 0xb6e8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,968&0xffffffff)?)?;
    // VMA 0xb6e90
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xb6e94
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xb6e98
    c.b(C::Al,0xb6e14,false)?;
    // VMA 0xb6e9c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6ea0
    c.b(C::Al,0xb7d98,true)?;
    // VMA 0xb6ea4
    c.b(C::Al,0xb6ebc,false)?;
    // VMA 0xb6ea8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6eac
    c.b(C::Al,0xb84fc,true)?;
    // VMA 0xb6eb0
    c.b(C::Al,0xb6ebc,false)?;
    // VMA 0xb6eb4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // VMA 0xb6eb8
    c.b(C::Al,0xb87b4,true)?;
    // VMA 0xb6ebc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xb6ec0
    c.b(C::Eq,0xb6e14,false)?;
    // VMA 0xb6ec4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb6ec8)?;
    const EVENT_CONTROLLER_DIAGNOSTICS:[u32;5]=[0x000f8fbc,0x000f92e6,0x000e2788,0x000f9315,0x000f9340];
    for value in EVENT_CONTROLLER_DIAGNOSTICS {c.emit(value)?;}
    c.check(0xb6edc)
}
