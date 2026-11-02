
const CODEC_FIFO_LITERALS:[u32;7]=[
    0x000f_8487, // codec trace message
    0x000e_2788, // log format
    0x000f_8334, // controller command diagnostics
    0x000f_842b,
    0x000f_84b7,
    0x0010_dd64, // shared codec state global
    0x000f_65d4, // firmware callback name
];

/// Normalize up to sixteen runtime controller codec buffer pointer slots
/// after an MMIO rebind. Includes two separately classified global addresses.
fn remap_codec_channel_slots(c:&mut Code)->Result<(),String>{
    c.check(0xab784)?;
    // VMA 0xab784
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xab788
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xab78c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xab790
    c.literal(R::new(9)?,0xab8e4)?;
    // VMA 0xab794
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xab798
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xab79c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // VMA 0xab7a0
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,8)?)?;
    // VMA 0xab7a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(7)?,2880)?)?;
    // VMA 0xab7a8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xab7ac
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,0)?)?;
    // VMA 0xab7b0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xab7b4
    c.emit(arm32::branch_exchange_with_link(C::Al,R::new(3)?)?)?;
    // VMA 0xab7b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,56)?)?;
    // VMA 0xab7bc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // VMA 0xab7c0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab7c4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,608)?)?;
    // VMA 0xab7c8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,64)?)?;
    // VMA 0xab7cc
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab7d0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,616)?)?;
    // VMA 0xab7d4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,624)?)?;
    // VMA 0xab7d8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,72)?)?;
    // VMA 0xab7dc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab7e0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,632)?)?;
    // VMA 0xab7e4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab7e8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,612)?)?;
    // VMA 0xab7ec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,80)?)?;
    // VMA 0xab7f0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xab7f4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,620)?)?;
    // VMA 0xab7f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,628)?)?;
    // VMA 0xab7fc
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab800
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,636)?)?;
    // VMA 0xab804
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,604)?)?;
    // VMA 0xab808
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab80c
    c.b(C::Eq,0xab870,false)?;
    // VMA 0xab810
    c.literal(R::R0,0xab8e8)?;
    // VMA 0xab814
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,656)?)?;
    // VMA 0xab818
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xab81c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xab820
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xab824
    c.emit(arm32::branch_exchange_with_link(C::Al,R::new(3)?)?)?;
    // VMA 0xab828
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,656)?)?;
    // VMA 0xab82c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,60)?)?;
    // VMA 0xab830
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab834
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,640)?)?;
    // VMA 0xab838
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,68)?)?;
    // VMA 0xab83c
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab840
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,648)?)?;
    // VMA 0xab844
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,76)?)?;
    // VMA 0xab848
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab84c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,664)?)?;
    // VMA 0xab850
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xab854
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,644)?)?;
    // VMA 0xab858
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,84)?)?;
    // VMA 0xab85c
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xab860
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,652)?)?;
    // VMA 0xab864
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xab868
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,660)?)?;
    // VMA 0xab86c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,668)?)?;
    // VMA 0xab870
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,352)?)?;
    // VMA 0xab874
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xab878
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xab87c
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R2,128)?)?;
    // VMA 0xab880
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // VMA 0xab884
    c.b(C::Eq,0xab8a0,false)?;
    // VMA 0xab888
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,0)?)?;
    // VMA 0xab88c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xab890
    c.emit(arm32::branch_exchange_with_link(C::Al,R::new(3)?)?)?;
    // VMA 0xab894
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,128)?)?;
    // VMA 0xab898
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab89c
    c.b(C::Ne,0xab8a8,false)?;
    // VMA 0xab8a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xab8a4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,128)?)?;
    // VMA 0xab8a8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,56)?)?;
    // VMA 0xab8ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xab8b0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,16)?)?;
    // VMA 0xab8b4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab8b8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xab8bc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,64)?)?;
    // VMA 0xab8c0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab8c4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,64)?)?;
    // VMA 0xab8c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,72)?)?;
    // VMA 0xab8cc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xab8d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,192)?)?;
    // VMA 0xab8d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,4)?)?;
    // VMA 0xab8d8
    c.emit(arm32::data_immediate(C::Eq,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xab8dc
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xab8e0
    c.b(C::Al,0xab878,false)?;
    c.check(0xab8e4)
}

const SLOT_REMAP_GLOBALS:[u32;2]=[0x0010_dd68,0x0010_dd6c];

/// Bind a codec slot by tag and store the resolved controller pointer.
fn link_codec_slot_runtime(c:&mut Code)->Result<(),String>{
    c.check(0xab8ec)?;
    // VMA 0xab8ec
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xab8f0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // VMA 0xab8f4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xab8f8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // VMA 0xab8fc
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // VMA 0xab900
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // VMA 0xab904
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // VMA 0xab908
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,2880)?)?;
    // VMA 0xab90c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xab910
    c.b(C::Al,0xa9138,true)?;
    // VMA 0xab914
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xab918
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // VMA 0xab91c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,140)?)?;
    // VMA 0xab920
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::new(5)?,136)?)?;
    // VMA 0xab924
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R1,1)?)?;
    // VMA 0xab928
    c.b(C::Eq,0xab958,false)?;
    // VMA 0xab92c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // VMA 0xab930
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xab934
    c.b(C::Al,0xa92c8,true)?;
    // VMA 0xab938
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,136)?)?;
    // VMA 0xab93c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xab940
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xab944
    c.b(C::Al,0xa9138,true)?;
    // VMA 0xab948
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,136)?)?;
    // VMA 0xab94c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xab950
    c.b(C::Ne,0xab95c,false)?;
    // VMA 0xab954
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // VMA 0xab958
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xab95c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // VMA 0xab960
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xab964)
}

/// Publish per-session display metadata and process mode-specific parameter
/// defaults. This helper terminates at its original POP {...,PC}.
fn publish_codec_parameter_modes(c:&mut Code)->Result<(),String>{
    c.check(0xab964)?;
    // VMA 0xab964
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xab968
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // VMA 0xab96c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2448)?)?;
    // VMA 0xab970
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2816)?)?;
    // VMA 0xab974
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xab978
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,19)?)?;
    // VMA 0xab97c
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2821)?)?;
    // VMA 0xab980
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,616)?)?;
    // VMA 0xab984
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,2444)?)?;
    // VMA 0xab988
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R2,R::R12,R::R2)?)?;
    // VMA 0xab98c
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::R0,R::new(3)?)?)?;
    // VMA 0xab990
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,106)?)?;
    // VMA 0xab994
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // VMA 0xab998
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,512)?)?;
    // VMA 0xab99c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2816)?)?;
    // VMA 0xab9a0
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R2,R::R12,R::R2)?)?;
    // VMA 0xab9a4
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R2,R::R0,R::new(3)?)?)?;
    // VMA 0xab9a8
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R12,167)?)?;
    // VMA 0xab9ac
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2820)?)?;
    // VMA 0xab9b0
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2592)?)?;
    // VMA 0xab9b4
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2822)?)?;
    // VMA 0xab9b8
    c.emit(arm32::transfer_byte(C::Al,true,R::LR,R::R0,2594)?)?;
    // VMA 0xab9bc
    c.emit(arm32::transfer_byte(C::Al,false,R::LR,R::R0,2823)?)?;
    // VMA 0xab9c0
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,2593)?)?;
    // VMA 0xab9c4
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,2824)?)?;
    // VMA 0xab9c8
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,2596)?)?;
    // VMA 0xab9cc
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::R0,2825)?)?;
    // VMA 0xab9d0
    c.b(C::Eq,0xaba04,false)?;
    // VMA 0xab9d4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::LR,Shift::Lsl,0,true)?)?;
    // VMA 0xab9d8
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2599)?)?;
    // VMA 0xab9dc
    c.b(C::Ne,0xaba00,false)?;
    // VMA 0xab9e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xab9e4
    c.b(C::Eq,0xab9fc,false)?;
    // VMA 0xab9e8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,2488)?)?;
    // VMA 0xab9ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xab9f0
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R2,2147483649)?)?;
    // VMA 0xab9f4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xab9f8
    c.b(C::Al,0xaba00,false)?;
    // VMA 0xab9fc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xaba00
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2838)?)?;
    // VMA 0xaba04
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2600)?)?;
    // VMA 0xaba08
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,131)?)?;
    // VMA 0xaba0c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,572)?)?;
    // VMA 0xaba10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,2560)?)?;
    // VMA 0xaba14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,2048)?)?;
    // VMA 0xaba18
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2826)?)?;
    // VMA 0xaba1c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2608)?)?;
    // VMA 0xaba20
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2827)?)?;
    // VMA 0xaba24
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2609)?)?;
    // VMA 0xaba28
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2828)?)?;
    // VMA 0xaba2c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2612)?)?;
    // VMA 0xaba30
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2832)?)?;
    // VMA 0xaba34
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2616)?)?;
    // VMA 0xaba38
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2836)?)?;
    // VMA 0xaba3c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2686)?)?;
    // VMA 0xaba40
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2839)?)?;
    // VMA 0xaba44
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2687)?)?;
    // VMA 0xaba48
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2840)?)?;
    // VMA 0xaba4c
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,2688)?)?;
    // VMA 0xaba50
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2841)?)?;
    // VMA 0xaba54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,129)?)?;
    // VMA 0xaba58
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,2560)?)?;
    // VMA 0xaba5c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2844)?)?;
    // VMA 0xaba60
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,2848)?)?;
    // VMA 0xaba64
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,2852)?)?;
    // VMA 0xaba68
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R12,131)?)?;
    // VMA 0xaba6c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2860)?)?;
    // VMA 0xaba70
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,160)?)?;
    // VMA 0xaba74
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,2864)?)?;
    // VMA 0xaba78
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,2868)?)?;
    // VMA 0xaba7c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xaba80)
}

/// Emit one codec notification from the selected channel buffer, including
/// mode flags and command metadata. The real function ends in a tail branch
/// to the logging routine; five literal words follow at 0xabbe4.
fn dispatch_controller_codec_events(c:&mut Code)->Result<(),String>{
    c.check(0xaba80)?;
    // VMA 0xaba80
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xaba84
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // VMA 0xaba88
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // VMA 0xaba8c
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // VMA 0xaba90
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // VMA 0xaba94
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // VMA 0xaba98
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    // VMA 0xaba9c
    c.literal(R::new(7)?,0xabbe4)?;
    // VMA 0xabaa0
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // VMA 0xabaa4
    c.b(C::Eq,0xabb44,false)?;
    // VMA 0xabaa8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabaac
    c.emit(arm32::data_immediate(C::Eq,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xabab0
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xabab4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,84)?)?;
    // VMA 0xabab8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // VMA 0xababc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,8)?)?;
    // VMA 0xabac0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,8)?)?;
    // VMA 0xabac4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,4)?)?;
    // VMA 0xabac8
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,12)?)?;
    // VMA 0xabacc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,40)?)?;
    // VMA 0xabad0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,24)?)?;
    // VMA 0xabad4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // VMA 0xabad8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,44)?)?;
    // VMA 0xabadc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xabae0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // VMA 0xabae4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1096)?)?;
    // VMA 0xabae8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1000)?)?;
    // VMA 0xabaec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xabaf0
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xabaf4
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xabaf8
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xabafc
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xabb00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,1096)?)?;
    // VMA 0xabb04
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,2400)?)?;
    // VMA 0xabb08
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,72)?)?;
    // VMA 0xabb0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xabb10
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,76)?)?;
    // VMA 0xabb14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // VMA 0xabb18
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,40)?)?;
    // VMA 0xabb1c
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,5,false)?)?;
    // VMA 0xabb20
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,80)?)?;
    // VMA 0xabb24
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,720)?)?;
    // VMA 0xabb28
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,864)?)?;
    // VMA 0xabb2c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xabb30
    c.b(C::Eq,0xabba0,false)?;
    // VMA 0xabb34
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xabb38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xabb3c
    c.b(C::Al,0x2a18c,true)?;
    // VMA 0xabb40
    c.b(C::Al,0xabba8,false)?;
    // VMA 0xabb44
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,720)?)?;
    // VMA 0xabb48
    c.literal(R::R2,0xabbe8)?;
    // VMA 0xabb4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,217)?)?;
    // VMA 0xabb50
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xabb54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // VMA 0xabb58
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xabb5c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xabb60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,6)?)?;
    // VMA 0xabb64
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // VMA 0xabb68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,8)?)?;
    // VMA 0xabb6c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // VMA 0xabb70
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,40)?)?;
    // VMA 0xabb74
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // VMA 0xabb78
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,864)?)?;
    // VMA 0xabb7c
    c.b(C::Al,0x2a878,true)?;
    // VMA 0xabb80
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(8)?,8)?)?;
    // VMA 0xabb84
    c.b(C::Al,0x26754,true)?;
    // VMA 0xabb88
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xabb8c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // VMA 0xabb90
    c.literal(R::R1,0xabbec)?;
    // VMA 0xabb94
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xabb98
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xabb9c
    c.b(C::Al,0xabbc8,false)?;
    // VMA 0xabba0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xabba4
    c.b(C::Al,0x2a878,true)?;
    // VMA 0xabba8
    c.b(C::Al,0x26754,true)?;
    // VMA 0xabbac
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,24)?)?;
    // VMA 0xabbb0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // VMA 0xabbb4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // VMA 0xabbb8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,28)?)?;
    // VMA 0xabbbc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xabbc0
    c.literal(R::R1,0xabbf4)?;
    // VMA 0xabbc4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // VMA 0xabbc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xabbcc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xabbd0
    c.literal(R::R1,0xabbf0)?;
    // VMA 0xabbd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xabbd8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // VMA 0xabbdc
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xabbe0
    c.b(C::Al,0x2ec94,false)?;
    c.check(0xabbe4)
}

const CODEC_RELAY_DIAGNOSTICS:[u32;5]=[
    0x0022_1007, // codec status command tag
    0x000f_8519, // diagnostic string
    0x000f_681d,
    0x000e_2788, // common logger context
    0x000f_683c,
];
