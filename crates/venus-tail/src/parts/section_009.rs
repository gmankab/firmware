
/// Program the per-channel codec MMIO command list and auxiliary quality
/// tables from initialized buffer descriptors. The original routine ends in
/// a POP {...,PC}, followed by seven non-instruction pointers/constants.
fn encode_codec_channel_command_queue(c:&mut Code)->Result<(),String>{
    c.check(0xaa6f0)?;
    // VMA 0xaa6f0
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xaa6f4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xaa6f8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,108)?)?;
    // VMA 0xaa6fc
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // VMA 0xaa700
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // VMA 0xaa704
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // VMA 0xaa708
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // VMA 0xaa70c
    c.literal(R::R1,0xab030)?;
    // VMA 0xaa710
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1164)?)?;
    // VMA 0xaa714
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // VMA 0xaa718
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xaa71c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,4)?)?;
    // VMA 0xaa720
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-32)?)?;
    // VMA 0xaa724
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,0)?)?;
    // VMA 0xaa728
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // VMA 0xaa72c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaa730
    c.literal(R::R1,0xab034)?;
    // VMA 0xaa734
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xaa738
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaa73c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,852)?)?;
    // VMA 0xaa740
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2896)?)?;
    // VMA 0xaa744
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // VMA 0xaa748
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xaa74c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xaa750
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(8)?,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa754
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xaa758
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R1,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa75c
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-44)?)?;
    // VMA 0xaa760
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(8)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xaa764
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,608)?)?;
    // VMA 0xaa768
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-48)?)?;
    // VMA 0xaa76c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::new(5)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xaa770
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3024)?)?;
    // VMA 0xaa774
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // VMA 0xaa778
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R1,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa77c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,64)?)?;
    // VMA 0xaa780
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xaa784
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,972)?)?;
    // VMA 0xaa788
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xaa78c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa790
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,612)?)?;
    // VMA 0xaa794
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,60)?)?;
    // VMA 0xaa798
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa79c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // VMA 0xaa7a0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // VMA 0xaa7a4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaa7a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa7ac
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,608)?)?;
    // VMA 0xaa7b0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa7b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa7b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,612)?)?;
    // VMA 0xaa7bc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa7c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa7c4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // VMA 0xaa7c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa7cc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,616)?)?;
    // VMA 0xaa7d0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa7d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa7d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,620)?)?;
    // VMA 0xaa7dc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa7e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa7e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // VMA 0xaa7e8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaa7ec
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa7f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,616)?)?;
    // VMA 0xaa7f4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa7f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa7fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,620)?)?;
    // VMA 0xaa800
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa804
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xaa80c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa810
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,624)?)?;
    // VMA 0xaa814
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa818
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa81c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,628)?)?;
    // VMA 0xaa820
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa824
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa828
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // VMA 0xaa82c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaa830
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa834
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,624)?)?;
    // VMA 0xaa838
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa83c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa840
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,628)?)?;
    // VMA 0xaa844
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa848
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa84c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,23)?)?;
    // VMA 0xaa850
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa854
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,632)?)?;
    // VMA 0xaa858
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa85c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa860
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,636)?)?;
    // VMA 0xaa864
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa868
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa86c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,23)?)?;
    // VMA 0xaa870
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaa874
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // VMA 0xaa878
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,632)?)?;
    // VMA 0xaa87c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaa880
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaa884
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xaa888
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,636)?)?;
    // VMA 0xaa88c
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // VMA 0xaa890
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa894
    c.literal(R::R0,0xab038)?;
    // VMA 0xaa898
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // VMA 0xaa89c
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // VMA 0xaa8a0
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // VMA 0xaa8a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,98)?)?;
    // VMA 0xaa8a8
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,255)?)?;
    // VMA 0xaa8ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xaa8b0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaa8b4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // VMA 0xaa8b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xaa8bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xaa8c0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,0)?)?;
    // VMA 0xaa8c4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // VMA 0xaa8c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // VMA 0xaa8cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1024)?)?;
    // VMA 0xaa8d0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xaa8d4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::new(6)?,Shift::Lsl,15)?)?;
    // VMA 0xaa8d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xaa8dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // VMA 0xaa8e0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,316)?)?;
    // VMA 0xaa8e4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaa8e8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa8ec
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaa8f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xaa8f4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,308)?)?;
    // VMA 0xaa8f8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaa8fc
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa900
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaa904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1440)?)?;
    // VMA 0xaa908
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,312)?)?;
    // VMA 0xaa90c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaa910
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,15,false)?)?;
    // VMA 0xaa914
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaa918
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,40)?)?;
    // VMA 0xaa91c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,256)?)?;
    // VMA 0xaa920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa924
    c.b(C::Eq,0xaaa88,false)?;
    // VMA 0xaa928
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,324)?)?;
    // VMA 0xaa92c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // VMA 0xaa930
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65280)?)?;
    // VMA 0xaa934
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R1,Shift::Lsr,15,false)?)?;
    // VMA 0xaa938
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(3)?,3)?)?;
    // VMA 0xaa93c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // VMA 0xaa940
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // VMA 0xaa944
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,64)?)?;
    // VMA 0xaa948
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,3)?)?;
    // VMA 0xaa94c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // VMA 0xaa950
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xaa954
    c.b(C::Cc,0xaa9a8,false)?;
    // VMA 0xaa958
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,768)?)?;
    // VMA 0xaa95c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(9)?,2)?)?;
    // VMA 0xaa960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // VMA 0xaa964
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(6)?,R::new(9)?,R::new(6)?)?)?;
    // VMA 0xaa968
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,768)?)?;
    // VMA 0xaa96c
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaa970
    c.emit(arm32::load_halfword(C::Al,R::new(6)?,R::new(7)?,0)?)?;
    // VMA 0xaa974
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa978
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaa97c
    c.b(C::Cc,0xaaa30,false)?;
    // VMA 0xaa980
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // VMA 0xaa984
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,2)?)?;
    // VMA 0xaa988
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa98c
    c.b(C::Cc,0xaa96c,false)?;
    // VMA 0xaa990
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,40)?)?;
    // VMA 0xaa994
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xaa998
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xaa99c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // VMA 0xaa9a0
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xaa9a4
    c.b(C::Al,0xaa9c0,false)?;
    // VMA 0xaa9a8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,40)?)?;
    // VMA 0xaa9ac
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaa9b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3758096384)?)?;
    // VMA 0xaa9b4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // VMA 0xaa9b8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    // VMA 0xaa9bc
    c.b(C::Eq,0xaa9e8,false)?;
    // VMA 0xaa9c0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(8)?,1056)?)?;
    // VMA 0xaa9c4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(9)?,768)?)?;
    // VMA 0xaa9c8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xaa9cc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3758096384)?)?;
    // VMA 0xaa9d0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaa9d4
    c.emit(arm32::load_halfword_postindexed(C::Al,R::new(6)?,R::R2,2)?)?;
    // VMA 0xaa9d8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // VMA 0xaa9dc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaa9e0
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R1,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xaa9e4
    c.b(C::Ne,0xaa9d4,false)?;
    // VMA 0xaa9e8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,256)?)?;
    // VMA 0xaa9ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,255)?)?;
    // VMA 0xaa9f0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,16128)?)?;
    // VMA 0xaa9f4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsr,1,false)?)?;
    // VMA 0xaa9f8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xaa9fc
    c.b(C::Eq,0xaaa88,false)?;
    // VMA 0xaaa00
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,36)?)?;
    // VMA 0xaaa04
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(9)?,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xaaa08
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xaaa0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3758096384)?)?;
    // VMA 0xaaa10
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // VMA 0xaaa14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,768)?)?;
    // VMA 0xaaa18
    c.emit(arm32::load_halfword_postindexed(C::Al,R::new(6)?,R::new(3)?,2)?)?;
    // VMA 0xaaa1c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // VMA 0xaaa20
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaaa24
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R2,R::new(7)?,Shift::Lsl,0)?)?;
    // VMA 0xaaa28
    c.b(C::Cc,0xaaa18,false)?;
    // VMA 0xaaa2c
    c.b(C::Al,0xaaa88,false)?;
    // VMA 0xaaa30
    c.literal(R::R1,0xab03c)?;
    // VMA 0xaaa34
    c.literal(R::R2,0xab040)?;
    // VMA 0xaaa38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xaaa3c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,884)?)?;
    // VMA 0xaaa40
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaa44
    c.literal(R::R1,0xab034)?;
    // VMA 0xaaa48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xaaa4c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaa50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,40)?)?;
    // VMA 0xaaa54
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // VMA 0xaaa58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xaaa5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xaaa60
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaa64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,32768)?)?;
    // VMA 0xaaa68
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaaa6c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,1056)?)?;
    // VMA 0xaaa70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xaaa74
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaaa78
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaaa7c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,36)?)?;
    // VMA 0xaaa80
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaa84
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaaa88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,1376)?)?;
    // VMA 0xaaa8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,36)?)?;
    // VMA 0xaaa90
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(9)?,352)?)?;
    // VMA 0xaaa94
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(9)?))?;
    // VMA 0xaaa98
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xaaa9c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // VMA 0xaaaa0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,420)?)?;
    // VMA 0xaaaa4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,228)?)?;
    // VMA 0xaaaa8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaaac
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaab0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,424)?)?;
    // VMA 0xaaab4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,232)?)?;
    // VMA 0xaaab8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaabc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaac0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,428)?)?;
    // VMA 0xaaac4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,236)?)?;
    // VMA 0xaaac8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaacc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaad0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,8192)?)?;
    // VMA 0xaaad4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,340)?)?;
    // VMA 0xaaad8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaadc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,4)?)?;
    // VMA 0xaaae0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,344)?)?;
    // VMA 0xaaae4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // VMA 0xaaae8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaaec
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,8)?)?;
    // VMA 0xaaaf0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,336)?)?;
    // VMA 0xaaaf4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // VMA 0xaaaf8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaafc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,16)?)?;
    // VMA 0xaab00
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,348)?)?;
    // VMA 0xaab04
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // VMA 0xaab08
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab0c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,6144)?)?;
    // VMA 0xaab10
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,320)?)?;
    // VMA 0xaab14
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab18
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,4)?)?;
    // VMA 0xaab1c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,324)?)?;
    // VMA 0xaab20
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // VMA 0xaab24
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,12)?)?;
    // VMA 0xaab2c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,328)?)?;
    // VMA 0xaab30
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // VMA 0xaab34
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab38
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,6272)?)?;
    // VMA 0xaab3c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,120)?)?;
    // VMA 0xaab40
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(8)?,2192)?)?;
    // VMA 0xaab48
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,4096)?)?;
    // VMA 0xaab4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xaab50
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab54
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,32)?)?;
    // VMA 0xaab58
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(8)?,7168)?)?;
    // VMA 0xaab5c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,332)?)?;
    // VMA 0xaab60
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab64
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,24)?)?;
    // VMA 0xaab68
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,64)?)?;
    // VMA 0xaab6c
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,60)?)?;
    // VMA 0xaab70
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-44)?)?;
    // VMA 0xaab74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,166)?)?;
    // VMA 0xaab78
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::LR))?;
    // VMA 0xaab7c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xaab80
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaab84
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,16)?)?;
    // VMA 0xaab88
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaab8c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,182)?)?;
    // VMA 0xaab90
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaab94
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaab98
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,0)?)?;
    // VMA 0xaab9c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-48)?)?;
    // VMA 0xaaba0
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::new(5)?,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaaba4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // VMA 0xaaba8
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(7)?,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaabac
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,612)?)?;
    // VMA 0xaabb0
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // VMA 0xaabb4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(9)?,R::R12,Shift::Lsl,0)?)?;
    // VMA 0xaabb8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaabbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,167)?)?;
    // VMA 0xaabc0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // VMA 0xaabc4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaabc8
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(4)?,64)?)?;
    // VMA 0xaabcc
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(5)?,0)?)?;
    // VMA 0xaabd0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // VMA 0xaabd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3758096384)?)?;
    // VMA 0xaabd8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,620)?)?;
    // VMA 0xaabdc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // VMA 0xaabe0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // VMA 0xaabe4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaabe8
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // VMA 0xaabec
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,128)?)?;
    // VMA 0xaabf0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    // VMA 0xaabf4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // VMA 0xaabf8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,628)?)?;
    // VMA 0xaabfc
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xaac00
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // VMA 0xaac04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,183)?)?;
    // VMA 0xaac08
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaac0c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaac10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // VMA 0xaac14
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xaac18
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,192)?)?;
    // VMA 0xaac1c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4)?)?;
    // VMA 0xaac20
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,0)?)?;
    // VMA 0xaac24
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // VMA 0xaac28
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,636)?)?;
    // VMA 0xaac2c
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xaac30
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(9)?,0)?)?;
    // VMA 0xaac34
    c.b(C::Ne,0xaab70,false)?;
    // VMA 0xaac38
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,32)?)?;
    // VMA 0xaac3c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,224)?)?;
    // VMA 0xaac40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xaac44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xaac48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // VMA 0xaac4c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,108)?)?;
    // VMA 0xaac50
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xaac54
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaac58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xaac5c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,36)?)?;
    // VMA 0xaac60
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,712)?)?;
    // VMA 0xaac64
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xaac68
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xaac6c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // VMA 0xaac70
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaac74
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // VMA 0xaac78
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xaac7c
    c.b(C::Al,0x3027c,true)?;
    // VMA 0xaac80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,120)?)?;
    // VMA 0xaac84
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-44)?)?;
    // VMA 0xaac88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xaac8c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1024)?)?;
    // VMA 0xaac90
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-56)?)?;
    // VMA 0xaac94
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,116)?)?;
    // VMA 0xaac98
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaac9c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // VMA 0xaaca0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,104)?)?;
    // VMA 0xaaca4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // VMA 0xaaca8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,48)?)?;
    // VMA 0xaacac
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,2400)?)?;
    // VMA 0xaacb0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::R0,12288)?)?;
    // VMA 0xaacb4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,356)?)?;
    // VMA 0xaacb8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // VMA 0xaacbc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // VMA 0xaacc0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,1152)?)?;
    // VMA 0xaacc4
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(10)?))?;
    // VMA 0xaacc8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // VMA 0xaaccc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(6)?,128)?)?;
    // VMA 0xaacd0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // VMA 0xaacd4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,172)?)?;
    // VMA 0xaacd8
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-48)?)?;
    // VMA 0xaacdc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-60)?)?;
    // VMA 0xaace0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,178)?)?;
    // VMA 0xaace4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // VMA 0xaace8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-64)?)?;
    // VMA 0xaacec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xaacf0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xaacf4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xaacf8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // VMA 0xaacfc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // VMA 0xaad00
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-40)?)?;
    // VMA 0xaad04
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // VMA 0xaad08
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xaad0c
    c.emit(arm32::data_register(C::Al,Op::Add,R::LR,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaad10
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaad14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,216)?)?;
    // VMA 0xaad18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R0,160)?)?;
    // VMA 0xaad1c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-52)?)?;
    // VMA 0xaad20
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // VMA 0xaad24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // VMA 0xaad28
    c.b(C::Ne,0xaad90,false)?;
    // VMA 0xaad2c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-56)?)?;
    // VMA 0xaad30
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-40)?)?;
    // VMA 0xaad34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaad38
    c.emit(arm32::mov_register(C::Al,R::R2,R::LR))?;
    // VMA 0xaad3c
    c.emit(arm32::mov_register(C::Al,R::R1,R::R12))?;
    // VMA 0xaad40
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-36)?)?;
    // VMA 0xaad44
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::R0,152)?)?;
    // VMA 0xaad48
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xaad4c
    c.emit(arm32::data_register(C::Eq,Op::Add,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xaad50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // VMA 0xaad54
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaad58
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xaad5c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // VMA 0xaad60
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaad64
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // VMA 0xaad68
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaad6c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::R2,4096)?)?;
    // VMA 0xaad70
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(10)?,R::new(9)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaad74
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-48)?)?;
    // VMA 0xaad78
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(8)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaad7c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // VMA 0xaad80
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-52)?)?;
    // VMA 0xaad84
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R11,-44)?)?;
    // VMA 0xaad88
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::R0,208)?)?;
    // VMA 0xaad8c
    c.b(C::Al,0xaadbc,false)?;
    // VMA 0xaad90
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // VMA 0xaad94
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-36)?)?;
    // VMA 0xaad98
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // VMA 0xaad9c
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaada0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R1,4096)?)?;
    // VMA 0xaada4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaada8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xaadac
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(8)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaadb0
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // VMA 0xaadb4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-60)?)?;
    // VMA 0xaadb8
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::new(4)?,168)?)?;
    // VMA 0xaadbc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xaadc0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // VMA 0xaadc4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,16)?)?;
    // VMA 0xaadc8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // VMA 0xaadcc
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(8)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaadd0
    c.b(C::Ne,0xaad20,false)?;
    // VMA 0xaadd4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-64)?)?;
    // VMA 0xaadd8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // VMA 0xaaddc
    c.emit(arm32::mov_register(C::Al,R::R12,R::LR))?;
    // VMA 0xaade0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(10)?))?;
    // VMA 0xaade4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R1,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaade8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R12,-8)?)?;
    // VMA 0xaadec
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-52)?)?;
    // VMA 0xaadf0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xaadf4
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::new(7)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaadf8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,64)?)?;
    // VMA 0xaadfc
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaae00
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(6)?,16)?)?;
    // VMA 0xaae04
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::LR,R::new(6)?,255)?)?;
    // VMA 0xaae08
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,60)?)?;
    // VMA 0xaae0c
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::LR,R::new(6)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaae10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R2,16)?)?;
    // VMA 0xaae14
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,0)?)?;
    // VMA 0xaae18
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,0)?)?;
    // VMA 0xaae1c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xaae20
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // VMA 0xaae24
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(3)?,0)?)?;
    // VMA 0xaae28
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,16)?)?;
    // VMA 0xaae2c
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // VMA 0xaae30
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // VMA 0xaae34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // VMA 0xaae38
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,68)?)?;
    // VMA 0xaae3c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,56)?)?;
    // VMA 0xaae40
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::R2,Shift::Lsl,R::new(6)?,false)?)?;
    // VMA 0xaae44
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaae48
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-52)?)?;
    // VMA 0xaae4c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,152)?)?;
    // VMA 0xaae50
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,0)?)?;
    // VMA 0xaae54
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaae58
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,48)?)?;
    // VMA 0xaae5c
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaae60
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // VMA 0xaae64
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,160)?)?;
    // VMA 0xaae68
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaae6c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,52)?)?;
    // VMA 0xaae70
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::new(5)?,Shift::Lsl,0)?)?;
    // VMA 0xaae74
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,152)?)?;
    // VMA 0xaae78
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,160)?)?;
    // VMA 0xaae7c
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,208)?)?;
    // VMA 0xaae80
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,216)?)?;
    // VMA 0xaae84
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(7)?,0)?)?;
    // VMA 0xaae88
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // VMA 0xaae8c
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R2.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xaae90
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // VMA 0xaae94
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,12)?)?;
    // VMA 0xaae98
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // VMA 0xaae9c
    c.literal(R::R1,0xab044)?;
    // VMA 0xaaea0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xaaea4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xaaea8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaeac
    c.literal(R::R1,0xab034)?;
    // VMA 0xaaeb0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xaaeb4
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaeb8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // VMA 0xaaebc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-40)?)?;
    // VMA 0xaaec0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // VMA 0xaaec4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // VMA 0xaaec8
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,68)?)?;
    // VMA 0xaaecc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,65536)?)?;
    // VMA 0xaaed0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // VMA 0xaaed4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // VMA 0xaaed8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-64)?)?;
    // VMA 0xaaedc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,65536)?)?;
    // VMA 0xaaee0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-64)?)?;
    // VMA 0xaaee4
    c.b(C::Ne,0xaacfc,false)?;
    // VMA 0xaaee8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,56)?)?;
    // VMA 0xaaeec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // VMA 0xaaef0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xaaef4
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaef8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,114)?)?;
    // VMA 0xaaefc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // VMA 0xaaf00
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaf04
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,28)?)?;
    // VMA 0xaaf08
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // VMA 0xaaf0c
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(10)?,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaf10
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,64)?)?;
    // VMA 0xaaf14
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaf18
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,16)?)?;
    // VMA 0xaaf1c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,60)?)?;
    // VMA 0xaaf20
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,255)?)?;
    // VMA 0xaaf24
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaf28
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,32)?)?;
    // VMA 0xaaf2c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // VMA 0xaaf30
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,240)?)?;
    // VMA 0xaaf34
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(7)?,100)?)?;
    // VMA 0xaaf38
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaaf3c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // VMA 0xaaf40
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // VMA 0xaaf44
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,148)?)?;
    // VMA 0xaaf48
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,240)?)?;
    // VMA 0xaaf4c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,4)?)?;
    // VMA 0xaaf50
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xaaf54
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xaaf58
    c.b(C::Cc,0xaaf8c,false)?;
    // VMA 0xaaf5c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // VMA 0xaaf60
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,16)?)?;
    // VMA 0xaaf64
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // VMA 0xaaf68
    c.literal(R::R1,0xab048)?;
    // VMA 0xaaf6c
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xaaf70
    c.literal(R::R2,0xab040)?;
    // VMA 0xaaf74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xaaf78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1012)?)?;
    // VMA 0xaaf7c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaf80
    c.literal(R::R1,0xab034)?;
    // VMA 0xaaf84
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xaaf88
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xaaf8c
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,36)?)?;
    // VMA 0xaaf90
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // VMA 0xaaf94
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // VMA 0xaaf98
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,672)?)?;
    // VMA 0xaaf9c
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(6)?,1244)?)?;
    // VMA 0xaafa0
    c.b(C::Al,0xdff3c,true)?;
    // VMA 0xaafa4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,604)?)?;
    // VMA 0xaafa8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaafac
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,20)?)?;
    // VMA 0xaafb0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaafb4
    c.b(C::Eq,0xab004,false)?;
    // VMA 0xaafb8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,1246)?)?;
    // VMA 0xaafbc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xaafc0
    c.b(C::Ne,0xab004,false)?;
    // VMA 0xaafc4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,3424)?)?;
    // VMA 0xaafc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // VMA 0xaafcc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // VMA 0xaafd0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // VMA 0xaafd4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // VMA 0xaafd8
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    // VMA 0xaafdc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // VMA 0xaafe0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,324)?)?;
    // VMA 0xaafe4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,100)?)?;
    // VMA 0xaafe8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,3)?)?;
    // VMA 0xaafec
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // VMA 0xaaff0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,32)?)?;
    // VMA 0xaaff4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // VMA 0xaaff8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // VMA 0xaaffc
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,64)?)?;
    // VMA 0xab000
    c.b(C::Al,0x31250,true)?;
    // VMA 0xab004
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,864)?)?;
    // VMA 0xab008
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,8)?)?;
    // VMA 0xab00c
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xab010
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,148)?)?;
    // VMA 0xab014
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,4)?)?;
    // VMA 0xab018
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // VMA 0xab01c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // VMA 0xab020
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(5)?.bit()|R::new(6)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xab024
    c.b(C::Al,0x31dc8,true)?;
    // VMA 0xab028
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xab02c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xab030)
}
