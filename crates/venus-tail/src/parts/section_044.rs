
/// Transition codec contexts into active runtime: release stale frame buffers,
/// switch to the new codec mode, iterate the active stream list and emit
/// platform diagnostics when an unexpected controller state is encountered.
fn activate_codec_runtime_and_recheck_buffers(c:&mut Code)->Result<(),String>{
    c.check(0xb76f4)?;
    // ARM VMA 0xb76f4
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb76f8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // ARM VMA 0xb76fc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,104)?)?;
    // ARM VMA 0xb7700
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM VMA 0xb7704
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,236)?)?;
    // ARM VMA 0xb7708
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,4)?)?;
    // ARM VMA 0xb770c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb7710
    c.b(C::Eq,0xb7734,false)?;
    // ARM VMA 0xb7714
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7718
    c.b(C::Ne,0xb7930,false)?;
    // ARM VMA 0xb771c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7720
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,236)?)?;
    // ARM VMA 0xb7724
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    // ARM VMA 0xb7728
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // ARM VMA 0xb772c
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb7730
    c.b(C::Al,0x292c,false)?;
    // ARM VMA 0xb7734
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,108)?)?;
    // ARM VMA 0xb7738
    c.literal(R::new(6)?,0xb7938)?;
    // ARM VMA 0xb773c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // ARM VMA 0xb7740
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,16777216)?)?;
    // ARM VMA 0xb7744
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // ARM VMA 0xb7748
    c.b(C::Eq,0xb77d8,false)?;
    // ARM VMA 0xb774c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // ARM VMA 0xb7750
    c.b(C::Eq,0xb7880,false)?;
    // ARM VMA 0xb7754
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7758
    c.b(C::Ne,0xb7900,false)?;
    // ARM VMA 0xb775c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,228)?)?;
    // ARM VMA 0xb7760
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7764
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,2120)?)?;
    // ARM VMA 0xb7768
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb776c
    c.b(C::Eq,0xb77c4,false)?;
    // ARM VMA 0xb7770
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,80)?)?;
    // ARM VMA 0xb7774
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM VMA 0xb7778
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // ARM VMA 0xb777c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM VMA 0xb7780
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb7784
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // ARM VMA 0xb7788
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // ARM VMA 0xb778c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2116)?)?;
    // ARM VMA 0xb7790
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM VMA 0xb7794
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,2124)?)?;
    // ARM VMA 0xb7798
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // ARM VMA 0xb779c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // ARM VMA 0xb77a0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,80)?)?;
    // ARM VMA 0xb77a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // ARM VMA 0xb77a8
    c.b(C::Al,0x2a878,true)?;
    // ARM VMA 0xb77ac
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2092)?)?;
    // ARM VMA 0xb77b0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,228)?)?;
    // ARM VMA 0xb77b4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2108)?)?;
    // ARM VMA 0xb77b8
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2112)?)?;
    // ARM VMA 0xb77bc
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2116)?)?;
    // ARM VMA 0xb77c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2120)?)?;
    // ARM VMA 0xb77c4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,116)?)?;
    // ARM VMA 0xb77c8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,128)?)?;
    // ARM VMA 0xb77cc
    c.b(C::Al,0xb794c,true)?;
    // ARM VMA 0xb77d0
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R11,-28)?)?;
    // ARM VMA 0xb77d4
    c.b(C::Al,0xb78ac,false)?;
    // ARM VMA 0xb77d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,228)?)?;
    // ARM VMA 0xb77dc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb77e0
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,2120)?)?;
    // ARM VMA 0xb77e4
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb77e8
    c.b(C::Eq,0xb783c,false)?;
    // ARM VMA 0xb77ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,80)?)?;
    // ARM VMA 0xb77f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM VMA 0xb77f4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // ARM VMA 0xb77f8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // ARM VMA 0xb77fc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // ARM VMA 0xb7800
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,8)?)?;
    // ARM VMA 0xb7804
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,2116)?)?;
    // ARM VMA 0xb7808
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // ARM VMA 0xb780c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // ARM VMA 0xb7810
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,16)?)?;
    // ARM VMA 0xb7814
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,80)?)?;
    // ARM VMA 0xb7818
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // ARM VMA 0xb781c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // ARM VMA 0xb7820
    c.b(C::Al,0x2a878,true)?;
    // ARM VMA 0xb7824
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2092)?)?;
    // ARM VMA 0xb7828
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,228)?)?;
    // ARM VMA 0xb782c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2108)?)?;
    // ARM VMA 0xb7830
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2112)?)?;
    // ARM VMA 0xb7834
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2116)?)?;
    // ARM VMA 0xb7838
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,2120)?)?;
    // ARM VMA 0xb783c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,116)?)?;
    // ARM VMA 0xb7840
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,128)?)?;
    // ARM VMA 0xb7844
    c.b(C::Al,0xb794c,true)?;
    // ARM VMA 0xb7848
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb784c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb7850
    c.b(C::Al,0xd482c,true)?;
    // ARM VMA 0xb7854
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,448)?)?;
    // ARM VMA 0xb7858
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb785c
    c.b(C::Eq,0xb786c,false)?;
    // ARM VMA 0xb7860
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb7864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM VMA 0xb7868
    c.b(C::Al,0xd482c,true)?;
    // ARM VMA 0xb786c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,3)?)?;
    // ARM VMA 0xb7870
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-28)?)?;
    // ARM VMA 0xb7874
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,356)?)?;
    // ARM VMA 0xb7878
    c.b(C::Al,0x80268,true)?;
    // ARM VMA 0xb787c
    c.b(C::Al,0xb78ac,false)?;
    // ARM VMA 0xb7880
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb7884
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb7888
    c.b(C::Al,0xd482c,true)?;
    // ARM VMA 0xb788c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,448)?)?;
    // ARM VMA 0xb7890
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7894
    c.b(C::Eq,0xb78a4,false)?;
    // ARM VMA 0xb7898
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb789c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM VMA 0xb78a0
    c.b(C::Al,0xd482c,true)?;
    // ARM VMA 0xb78a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,1)?)?;
    // ARM VMA 0xb78a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-28)?)?;
    // ARM VMA 0xb78ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM VMA 0xb78b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // ARM VMA 0xb78b4
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,0)?)?;
    // ARM VMA 0xb78b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,104)?)?;
    // ARM VMA 0xb78bc
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // ARM VMA 0xb78c0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM VMA 0xb78c4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,104)?)?;
    // ARM VMA 0xb78c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,236)?)?;
    // ARM VMA 0xb78cc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // ARM VMA 0xb78d0
    c.b(C::Al,0x292c,true)?;
    // ARM VMA 0xb78d4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb78d8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,44)?)?;
    // ARM VMA 0xb78dc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // ARM VMA 0xb78e0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(6)?,1)?)?;
    // ARM VMA 0xb78e4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // ARM VMA 0xb78e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // ARM VMA 0xb78ec
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // ARM VMA 0xb78f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-44)?)?;
    // ARM VMA 0xb78f4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // ARM VMA 0xb78f8
    c.b(C::Al,0x2a878,true)?;
    // ARM VMA 0xb78fc
    c.b(C::Al,0xb7930,false)?;
    // ARM VMA 0xb7900
    c.literal(R::R1,0xb793c)?;
    // ARM VMA 0xb7904
    c.literal(R::R2,0xb7940)?;
    // ARM VMA 0xb7908
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb790c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,186)?)?;
    // ARM VMA 0xb7910
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7914
    c.literal(R::R1,0xb7944)?;
    // ARM VMA 0xb7918
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb791c
    c.b(C::Al,0x2ec94,true)?;
    // ARM VMA 0xb7920
    c.literal(R::R2,0xb7948)?;
    // ARM VMA 0xb7924
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7928
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,188)?)?;
    // ARM VMA 0xb792c
    c.b(C::Al,0x17b0,true)?;
    // ARM VMA 0xb7930
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // ARM VMA 0xb7934
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb7938)?;
    for value in RESTART_CODEC_LOGS {c.emit(value)?;}
    c.check(0xb794c)
}

/// Apply a format descriptor repeatedly to all ready output records. The
/// original code is an iterative queue walker with a compact event code pool.
fn issue_codec_stream_commands(c:&mut Code)->Result<(),String>{
    c.check(0xb794c)?;
    // ARM VMA 0xb794c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb7950
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // ARM VMA 0xb7954
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,144)?)?;
    // ARM VMA 0xb7958
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // ARM VMA 0xb795c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,84)?)?;
    // ARM VMA 0xb7960
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // ARM VMA 0xb7964
    c.b(C::Al,0x80994,true)?;
    // ARM VMA 0xb7968
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb796c
    c.b(C::Ne,0xb79dc,false)?;
    // ARM VMA 0xb7970
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // ARM VMA 0xb7974
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,80)?)?;
    // ARM VMA 0xb7978
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,4)?)?;
    // ARM VMA 0xb797c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::SP,84)?)?;
    // ARM VMA 0xb7980
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // ARM VMA 0xb7984
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // ARM VMA 0xb7988
    c.literal(R::R0,0xb79e4)?;
    // ARM VMA 0xb798c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // ARM VMA 0xb7990
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,128)?)?;
    // ARM VMA 0xb7994
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM VMA 0xb7998
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,92)?)?;
    // ARM VMA 0xb799c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // ARM VMA 0xb79a0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,132)?)?;
    // ARM VMA 0xb79a4
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,4)?)?;
    // ARM VMA 0xb79a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // ARM VMA 0xb79ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,116)?)?;
    // ARM VMA 0xb79b0
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,80)?)?;
    // ARM VMA 0xb79b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // ARM VMA 0xb79b8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,124)?)?;
    // ARM VMA 0xb79bc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // ARM VMA 0xb79c0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // ARM VMA 0xb79c4
    c.b(C::Al,0x2a878,true)?;
    // ARM VMA 0xb79c8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb79cc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // ARM VMA 0xb79d0
    c.b(C::Al,0x80994,true)?;
    // ARM VMA 0xb79d4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb79d8
    c.b(C::Eq,0xb7980,false)?;
    // ARM VMA 0xb79dc
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // ARM VMA 0xb79e0
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb79e4)?;
    c.emit(CODEC_EVENT_READY)?;
    c.check(0xb79e8)
}

/// Begin a codec stream, create the callback record and notify the device.
fn begin_codec_stream_record(c:&mut Code)->Result<(),String>{
    c.check(0xb79e8)?;
    // ARM VMA 0xb79e8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb79ec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // ARM VMA 0xb79f0
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,80)?)?;
    // ARM VMA 0xb79f4
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // ARM VMA 0xb79f8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,236)?)?;
    // ARM VMA 0xb79fc
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,4)?)?;
    // ARM VMA 0xb7a00
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // ARM VMA 0xb7a04
    c.b(C::Eq,0xb7a28,false)?;
    // ARM VMA 0xb7a08
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7a0c
    c.b(C::Ne,0xb7b04,false)?;
    // ARM VMA 0xb7a10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7a14
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,236)?)?;
    // ARM VMA 0xb7a18
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,28)?)?;
    // ARM VMA 0xb7a1c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // ARM VMA 0xb7a20
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb7a24
    c.b(C::Al,0x292c,false)?;
    // ARM VMA 0xb7a28
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,228)?)?;
    // ARM VMA 0xb7a2c
    c.literal(R::new(6)?,0xb7b0c)?;
    // ARM VMA 0xb7a30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7a34
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,2120)?)?;
    // ARM VMA 0xb7a38
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7a3c
    c.b(C::Eq,0xb7a94,false)?;
    // ARM VMA 0xb7a40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,80)?)?;
    // ARM VMA 0xb7a44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM VMA 0xb7a48
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // ARM VMA 0xb7a4c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // ARM VMA 0xb7a50
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // ARM VMA 0xb7a54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,4)?)?;
    // ARM VMA 0xb7a58
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // ARM VMA 0xb7a5c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,2116)?)?;
    // ARM VMA 0xb7a60
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // ARM VMA 0xb7a64
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // ARM VMA 0xb7a68
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
    // ARM VMA 0xb7a6c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,76)?)?;
    // ARM VMA 0xb7a70
    c.emit(arm32::mov_register(C::Al,R::R1,R::SP))?;
    // ARM VMA 0xb7a74
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,128)?)?;
    // ARM VMA 0xb7a78
    c.b(C::Al,0x2a878,true)?;
    // ARM VMA 0xb7a7c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,2092)?)?;
    // ARM VMA 0xb7a80
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,228)?)?;
    // ARM VMA 0xb7a84
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,2108)?)?;
    // ARM VMA 0xb7a88
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,2112)?)?;
    // ARM VMA 0xb7a8c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,2116)?)?;
    // ARM VMA 0xb7a90
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,2120)?)?;
    // ARM VMA 0xb7a94
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,116)?)?;
    // ARM VMA 0xb7a98
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,128)?)?;
    // ARM VMA 0xb7a9c
    c.b(C::Al,0xb794c,true)?;
    // ARM VMA 0xb7aa0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb7aa4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // ARM VMA 0xb7aa8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // ARM VMA 0xb7aac
    c.b(C::Al,0xd482c,true)?;
    // ARM VMA 0xb7ab0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,448)?)?;
    // ARM VMA 0xb7ab4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // ARM VMA 0xb7ab8
    c.b(C::Eq,0xb7ac8,false)?;
    // ARM VMA 0xb7abc
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // ARM VMA 0xb7ac0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // ARM VMA 0xb7ac4
    c.b(C::Al,0xd482c,true)?;
    // ARM VMA 0xb7ac8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // ARM VMA 0xb7acc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,104)?)?;
    // ARM VMA 0xb7ad0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,236)?)?;
    // ARM VMA 0xb7ad4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,40)?)?;
    // ARM VMA 0xb7ad8
    c.b(C::Al,0x292c,true)?;
    // ARM VMA 0xb7adc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // ARM VMA 0xb7ae0
    c.emit(arm32::mov_register(C::Al,R::R1,R::SP))?;
    // ARM VMA 0xb7ae4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // ARM VMA 0xb7ae8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb7aec
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // ARM VMA 0xb7af0
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
    // ARM VMA 0xb7af4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM VMA 0xb7af8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,128)?)?;
    // ARM VMA 0xb7afc
    c.b(C::Al,0x2a878,true)?;
    // ARM VMA 0xb7b00
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,0)?)?;
    // ARM VMA 0xb7b04
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // ARM VMA 0xb7b08
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb7b0c)?;
    c.emit(CODEC_EVENT_STREAM_ON)?;
    c.check(0xb7b10)
}

/// Stop a codec stream while retaining input buffer ownership flags.
fn stop_codec_stream_record(c:&mut Code)->Result<(),String>{
    c.check(0xb7b10)?;
    // ARM VMA 0xb7b10
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // ARM VMA 0xb7b14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // ARM VMA 0xb7b18
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // ARM VMA 0xb7b1c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,236)?)?;
    // ARM VMA 0xb7b20
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,4)?)?;
    // ARM VMA 0xb7b24
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // ARM VMA 0xb7b28
    c.b(C::Eq,0xb7b4c,false)?;
    // ARM VMA 0xb7b2c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // ARM VMA 0xb7b30
    c.b(C::Ne,0xb7b8c,false)?;
    // ARM VMA 0xb7b34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // ARM VMA 0xb7b38
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,236)?)?;
    // ARM VMA 0xb7b3c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    // ARM VMA 0xb7b40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // ARM VMA 0xb7b44
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // ARM VMA 0xb7b48
    c.b(C::Al,0x292c,false)?;
    // ARM VMA 0xb7b4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // ARM VMA 0xb7b50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // ARM VMA 0xb7b54
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,104)?)?;
    // ARM VMA 0xb7b58
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,236)?)?;
    // ARM VMA 0xb7b5c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // ARM VMA 0xb7b60
    c.b(C::Al,0x292c,true)?;
    // ARM VMA 0xb7b64
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // ARM VMA 0xb7b68
    c.emit(arm32::mov_register(C::Al,R::R1,R::SP))?;
    // ARM VMA 0xb7b6c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // ARM VMA 0xb7b70
    c.literal(R::R0,0xb7b94)?;
    // ARM VMA 0xb7b74
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // ARM VMA 0xb7b78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // ARM VMA 0xb7b7c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    // ARM VMA 0xb7b80
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // ARM VMA 0xb7b84
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // ARM VMA 0xb7b88
    c.b(C::Al,0x2a878,true)?;
    // ARM VMA 0xb7b8c
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // ARM VMA 0xb7b90
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xb7b94)?;
    c.emit(CODEC_EVENT_STREAM_OFF)?;
    c.check(0xb7b98)
}

/// Suspend and resume the short input-response mode, reporting whether the
/// controller's retained record table required resetting the stream context.
fn select_codec_stream_state(c:&mut Code)->Result<(),String>{
    c.check(0xb7b98)?;
    // ARM VMA 0xb7b98
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,236)?)?;
    // ARM VMA 0xb7b9c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,4)?)?;
    // ARM VMA 0xb7ba0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,9)?)?;
    // ARM VMA 0xb7ba4
    c.b(C::Eq,0xb7bc0,false)?;
    // ARM VMA 0xb7ba8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // ARM VMA 0xb7bac
    c.emit(arm32::bx(C::Ne,R::LR))?;
    // ARM VMA 0xb7bb0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // ARM VMA 0xb7bb4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,236)?)?;
    // ARM VMA 0xb7bb8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,28)?)?;
    // ARM VMA 0xb7bbc
    c.b(C::Al,0x292c,false)?;
    // ARM VMA 0xb7bc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // ARM VMA 0xb7bc4
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,104)?)?;
    // ARM VMA 0xb7bc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // ARM VMA 0xb7bcc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,236)?)?;
    // ARM VMA 0xb7bd0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,40)?)?;
    // ARM VMA 0xb7bd4
    c.b(C::Al,0x292c,false)?;
    c.check(0xb7bd8)
}

const BUFFER_CRC_TABLE_AND_POLYNOMIAL:[u32;3]=[0x0010_bb38,0x0010_b738,0x04c1_1db7];
const ACTIVE_BUFFER_TRACE:[u32;2]=[0x000f_959c,0x000e_2788];
const CODEC_WORKER_TRACE:[u32;4]=[0x000f_97d1,0x000e_2788,0x0022_1007,0x0100_1005];
