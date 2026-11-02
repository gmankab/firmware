
/// Write mode-specific codec control registers, initialize per-plane
/// structures and route controller error conditions through designated
/// platform callbacks. Includes a separate nine-word literal pool.
fn program_codec_register_packets(c:&mut Code)->Result<(),String>{
    c.check(0xabbf8)?;
    // VMA 0xabbf8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // VMA 0xabbfc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // VMA 0xabc00
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // VMA 0xabc04
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // VMA 0xabc08
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,12)?)?;
    // VMA 0xabc0c
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // VMA 0xabc10
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,8)?)?;
    // VMA 0xabc14
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // VMA 0xabc18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // VMA 0xabc1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // VMA 0xabc20
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // VMA 0xabc24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,984)?)?;
    // VMA 0xabc28
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xabc2c
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::new(10)?,988)?)?;
    // VMA 0xabc30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xabc34
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,0)?)?;
    // VMA 0xabc38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,104)?)?;
    // VMA 0xabc3c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // VMA 0xabc40
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // VMA 0xabc44
    c.literal(R::R0,0xac72c)?;
    // VMA 0xabc48
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // VMA 0xabc4c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // VMA 0xabc50
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // VMA 0xabc54
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // VMA 0xabc58
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,12)?)?;
    // VMA 0xabc5c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,76)?)?;
    // VMA 0xabc60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xabc64
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,20)?)?;
    // VMA 0xabc68
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,48)?)?;
    // VMA 0xabc6c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,52)?)?;
    // VMA 0xabc70
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,56)?)?;
    // VMA 0xabc74
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xabc78
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,24)?)?;
    // VMA 0xabc7c
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,28)?)?;
    // VMA 0xabc80
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,32)?)?;
    // VMA 0xabc84
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,36)?)?;
    // VMA 0xabc88
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,40)?)?;
    // VMA 0xabc8c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // VMA 0xabc90
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,4)?)?;
    // VMA 0xabc94
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,92)?)?;
    // VMA 0xabc98
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,8)?)?;
    // VMA 0xabc9c
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,96)?)?;
    // VMA 0xabca0
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,12)?)?;
    // VMA 0xabca4
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,48)?)?;
    // VMA 0xabca8
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(6)?,0)?)?;
    // VMA 0xabcac
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,84)?)?;
    // VMA 0xabcb0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // VMA 0xabcb4
    c.b(C::Eq,0xac200,false)?;
    // VMA 0xabcb8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,28)?)?;
    // VMA 0xabcbc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,36)?)?;
    // VMA 0xabcc0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // VMA 0xabcc4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,40)?)?;
    // VMA 0xabcc8
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,48)?)?;
    // VMA 0xabccc
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // VMA 0xabcd0
    c.b(C::Ne,0xac584,false)?;
    // VMA 0xabcd4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1900)?)?;
    // VMA 0xabcd8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabcdc
    c.b(C::Ne,0xac5f0,false)?;
    // VMA 0xabce0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,48)?)?;
    // VMA 0xabce4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,32)?)?;
    // VMA 0xabce8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xabcec
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,32)?)?;
    // VMA 0xabcf0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,36)?)?;
    // VMA 0xabcf4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,76)?)?;
    // VMA 0xabcf8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,20)?)?;
    // VMA 0xabcfc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // VMA 0xabd00
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,24)?)?;
    // VMA 0xabd04
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // VMA 0xabd08
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,44)?)?;
    // VMA 0xabd0c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,88)?)?;
    // VMA 0xabd10
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,92)?)?;
    // VMA 0xabd14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabd18
    c.b(C::Eq,0xac200,false)?;
    // VMA 0xabd1c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1900)?)?;
    // VMA 0xabd20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabd24
    c.b(C::Ne,0xac200,false)?;
    // VMA 0xabd28
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,12)?)?;
    // VMA 0xabd2c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,24)?)?;
    // VMA 0xabd30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1888)?)?;
    // VMA 0xabd34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // VMA 0xabd38
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(7)?,R::new(9)?,R::R0)?)?;
    // VMA 0xabd3c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xabd40
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // VMA 0xabd44
    c.b(C::Ne,0xac640,false)?;
    // VMA 0xabd48
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(7)?,Shift::Lsl,1)?)?;
    // VMA 0xabd4c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,68)?)?;
    // VMA 0xabd50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,868)?)?;
    // VMA 0xabd54
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // VMA 0xabd58
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xabd5c
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Lsl,1)?)?;
    // VMA 0xabd60
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,72)?)?;
    // VMA 0xabd64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,98)?)?;
    // VMA 0xabd68
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xabd6c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,4)?)?;
    // VMA 0xabd70
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(9)?,R::R1)?)?;
    // VMA 0xabd74
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xabd78
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R2,R::R1,Shift::Lsl,1,false)?)?;
    // VMA 0xabd7c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,60)?)?;
    // VMA 0xabd80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,102)?)?;
    // VMA 0xabd84
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xabd88
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(9)?,8)?)?;
    // VMA 0xabd8c
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R1,R::new(9)?,R::R1)?)?;
    // VMA 0xabd90
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // VMA 0xabd94
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(10)?,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xabd98
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,64)?)?;
    // VMA 0xabd9c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,24)?)?;
    // VMA 0xabda0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,576)?)?;
    // VMA 0xabda4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabda8
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,96)?)?;
    // VMA 0xabdac
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xabdb0
    c.b(C::Eq,0xac510,false)?;
    // VMA 0xabdb4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(6)?,64)?)?;
    // VMA 0xabdb8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,16)?)?;
    // VMA 0xabdbc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,32)?)?;
    // VMA 0xabdc0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1876)?)?;
    // VMA 0xabdc4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // VMA 0xabdc8
    c.literal(R::R1,0xac730)?;
    // VMA 0xabdcc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // VMA 0xabdd0
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xabdd4
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R0,4)?)?;
    // VMA 0xabdd8
    c.b(C::Eq,0xabe2c,false)?;
    // VMA 0xabddc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,92)?)?;
    // VMA 0xabde0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xabde4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xabde8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // VMA 0xabdec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // VMA 0xabdf0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xabdf4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xabdf8
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // VMA 0xabdfc
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,4)?)?;
    // VMA 0xabe00
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,8)?)?;
    // VMA 0xabe04
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xabe08
    c.literal(R::R2,0xac734)?;
    // VMA 0xabe0c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xabe10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,272)?)?;
    // VMA 0xabe14
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xabe18
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,20)?)?;
    // VMA 0xabe1c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,64)?)?;
    // VMA 0xabe20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // VMA 0xabe24
    c.b(C::Al,0x471c,true)?;
    // VMA 0xabe28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,92)?)?;
    // VMA 0xabe2c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,576)?)?;
    // VMA 0xabe30
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,524288)?)?;
    // VMA 0xabe34
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xabe38
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R0,64)?)?;
    // VMA 0xabe3c
    c.b(C::Eq,0xabec4,false)?;
    // VMA 0xabe40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,60)?)?;
    // VMA 0xabe44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xabe48
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xabe4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,21)?)?;
    // VMA 0xabe50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // VMA 0xabe54
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xabe58
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xabe5c
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // VMA 0xabe60
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,4)?)?;
    // VMA 0xabe64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,8)?)?;
    // VMA 0xabe68
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xabe6c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,148)?)?;
    // VMA 0xabe70
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,20)?)?;
    // VMA 0xabe74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xabe78
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,-12)?)?;
    // VMA 0xabe7c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // VMA 0xabe80
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // VMA 0xabe84
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    // VMA 0xabe88
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // VMA 0xabe8c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // VMA 0xabe90
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,12)?)?;
    // VMA 0xabe94
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // VMA 0xabe98
    c.b(C::Ne,0xabe78,false)?;
    // VMA 0xabe9c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,160)?)?;
    // VMA 0xabea0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,44)?)?;
    // VMA 0xabea4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,164)?)?;
    // VMA 0xabea8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,48)?)?;
    // VMA 0xabeac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,168)?)?;
    // VMA 0xabeb0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,52)?)?;
    // VMA 0xabeb4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,172)?)?;
    // VMA 0xabeb8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,56)?)?;
    // VMA 0xabebc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,60)?)?;
    // VMA 0xabec0
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,576)?)?;
    // VMA 0xabec4
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,1048576)?)?;
    // VMA 0xabec8
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xabecc
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R0,128)?)?;
    // VMA 0xabed0
    c.b(C::Eq,0xabf18,false)?;
    // VMA 0xabed4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // VMA 0xabed8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xabedc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // VMA 0xabee0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xabee4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xabee8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xabeec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // VMA 0xabef0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // VMA 0xabef4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xabef8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xabefc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // VMA 0xabf00
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xabf04
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1932)?)?;
    // VMA 0xabf08
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xabf0c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1936)?)?;
    // VMA 0xabf10
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,24)?)?;
    // VMA 0xabf14
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,28)?)?;
    // VMA 0xabf18
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,4194304)?)?;
    // VMA 0xabf1c
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,61)?)?;
    // VMA 0xabf20
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R0,1)?)?;
    // VMA 0xabf24
    c.b(C::Eq,0xabf84,false)?;
    // VMA 0xabf28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1712)?)?;
    // VMA 0xabf2c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,668)?)?;
    // VMA 0xabf30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xabf34
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // VMA 0xabf38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // VMA 0xabf3c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // VMA 0xabf40
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xabf44
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,1024)?)?;
    // VMA 0xabf48
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // VMA 0xabf4c
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,4)?)?;
    // VMA 0xabf50
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,8)?)?;
    // VMA 0xabf54
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xabf58
    c.literal(R::R2,0xac734)?;
    // VMA 0xabf5c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,90)?)?;
    // VMA 0xabf60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xabf64
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xabf68
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xabf6c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,20)?)?;
    // VMA 0xabf70
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,176)?)?;
    // VMA 0xabf74
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // VMA 0xabf78
    c.b(C::Al,0x471c,true)?;
    // VMA 0xabf7c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,576)?)?;
    // VMA 0xabf80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1712)?)?;
    // VMA 0xabf84
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,262144)?)?;
    // VMA 0xabf88
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xabf8c
    c.emit(arm32::data_immediate(C::Ne,Op::Tst,R::R0,R::R0,32)?)?;
    // VMA 0xabf90
    c.b(C::Eq,0xabfe4,false)?;
    // VMA 0xabf94
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1872)?)?;
    // VMA 0xabf98
    c.literal(R::R2,0xac734)?;
    // VMA 0xabf9c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,230)?)?;
    // VMA 0xabfa0
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // VMA 0xabfa4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xabfa8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xabfac
    c.b(C::Al,0x17b0,true)?;
    // VMA 0xabfb0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1872)?)?;
    // VMA 0xabfb4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // VMA 0xabfb8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // VMA 0xabfbc
    c.b(C::Al,0xe159c,true)?;
    // VMA 0xabfc0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // VMA 0xabfc4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1872)?)?;
    // VMA 0xabfc8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1184)?)?;
    // VMA 0xabfcc
    c.b(C::Al,0x7fd1c,true)?;
    // VMA 0xabfd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xabfd4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1872)?)?;
    // VMA 0xabfd8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // VMA 0xabfdc
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,576)?)?;
    // VMA 0xabfe0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,20)?)?;
    // VMA 0xabfe4
    c.literal(R::new(10)?,0xac738)?;
    // VMA 0xabfe8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,32)?)?;
    // VMA 0xabfec
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,12)?)?;
    // VMA 0xabff0
    c.b(C::Eq,0xac070,false)?;
    // VMA 0xabff4
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(9)?,2008)?)?;
    // VMA 0xabff8
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,2012)?)?;
    // VMA 0xabffc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,267386880)?)?;
    // VMA 0xac000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xac004
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2952790016)?)?;
    // VMA 0xac008
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xac00c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xac010
    c.b(C::Al,0x816f8,true)?;
    // VMA 0xac014
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac018
    c.b(C::Ne,0xac070,false)?;
    // VMA 0xac01c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,24)?)?;
    // VMA 0xac020
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,12)?)?;
    // VMA 0xac024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac028
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // VMA 0xac02c
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xac030
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // VMA 0xac034
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xac038
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // VMA 0xac03c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac040
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,4)?)?;
    // VMA 0xac044
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,8)?)?;
    // VMA 0xac048
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac04c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15728640)?)?;
    // VMA 0xac050
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // VMA 0xac054
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // VMA 0xac058
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xac05c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1073741824)?)?;
    // VMA 0xac060
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83018)?)?;
    // VMA 0xac064
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82d30)?)?;
    // VMA 0xac068
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xac06c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,24)?)?;
    // VMA 0xac070
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,1024)?)?;
    // VMA 0xac074
    c.b(C::Eq,0xac710,false)?;
    // VMA 0xac078
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,12)?)?;
    // VMA 0xac07c
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xac080
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac084
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac088
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac08c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac090
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xac094
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(10)?,1)?)?;
    // VMA 0xac098
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // VMA 0xac09c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // VMA 0xac0a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xac0a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // VMA 0xac0a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xac0ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1908)?)?;
    // VMA 0xac0b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xac0b4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,24)?)?;
    // VMA 0xac0b8
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,65536)?)?;
    // VMA 0xac0bc
    c.b(C::Eq,0xac104,false)?;
    // VMA 0xac0c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac0c4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac0c8
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac0cc
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac0d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xac0d4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // VMA 0xac0d8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // VMA 0xac0dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // VMA 0xac0e0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xac0e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // VMA 0xac0e8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xac0ec
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2016)?)?;
    // VMA 0xac0f0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,24)?)?;
    // VMA 0xac0f4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2020)?)?;
    // VMA 0xac0f8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,3)?)?;
    // VMA 0xac0fc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xac100
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,28)?)?;
    // VMA 0xac104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,964)?)?;
    // VMA 0xac108
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // VMA 0xac10c
    c.emit(arm32::load_signed_halfword_register(C::Al,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xac110
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // VMA 0xac114
    c.emit(arm32::data_immediate_with_flags(C::Ne,Op::And,R::R1,R::new(7)?,2048,true)?)?;
    // VMA 0xac118
    c.b(C::Eq,0xac17c,false)?;
    // VMA 0xac11c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // VMA 0xac120
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // VMA 0xac128
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac12c
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xac130
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,8)?)?;
    // VMA 0xac134
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,12)?)?;
    // VMA 0xac138
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,16)?)?;
    // VMA 0xac13c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,44)?)?;
    // VMA 0xac140
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // VMA 0xac144
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(10)?,1)?)?;
    // VMA 0xac148
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,20)?)?;
    // VMA 0xac14c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,32)?)?;
    // VMA 0xac150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // VMA 0xac154
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,24)?)?;
    // VMA 0xac158
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,198)?)?;
    // VMA 0xac15c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,28)?)?;
    // VMA 0xac160
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    // VMA 0xac164
    c.emit(arm32::load_signed_halfword_register(C::Al,R::R1,R::new(9)?,R::R1)?)?;
    // VMA 0xac168
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,40)?)?;
    // VMA 0xac16c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,36)?)?;
    // VMA 0xac170
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,44)?)?;
    // VMA 0xac174
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xac178
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,576)?)?;
    // VMA 0xac17c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,131072)?)?;
    // VMA 0xac180
    c.b(C::Eq,0xac294,false)?;
    // VMA 0xac184
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,44)?)?;
    // VMA 0xac188
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xac18c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // VMA 0xac190
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xac194
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1948)?)?;
    // VMA 0xac198
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,28)?)?;
    // VMA 0xac19c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1952)?)?;
    // VMA 0xac1a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,32)?)?;
    // VMA 0xac1a4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1968)?)?;
    // VMA 0xac1a8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,60)?)?;
    // VMA 0xac1ac
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1972)?)?;
    // VMA 0xac1b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,64)?)?;
    // VMA 0xac1b4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,1940)?)?;
    // VMA 0xac1b8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // VMA 0xac1bc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1944)?)?;
    // VMA 0xac1c0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,24)?)?;
    // VMA 0xac1c4
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,1941)?)?;
    // VMA 0xac1c8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,36)?)?;
    // VMA 0xac1cc
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,1942)?)?;
    // VMA 0xac1d0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac1d4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,40)?)?;
    // VMA 0xac1d8
    c.b(C::Eq,0xac218,false)?;
    // VMA 0xac1dc
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1956)?)?;
    // VMA 0xac1e0
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,44)?)?;
    // VMA 0xac1e4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1964)?)?;
    // VMA 0xac1e8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,52)?)?;
    // VMA 0xac1ec
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,61)?)?;
    // VMA 0xac1f0
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,2)?)?;
    // VMA 0xac1f4
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::new(9)?,1960)?)?;
    // VMA 0xac1f8
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::new(9)?,1868)?)?;
    // VMA 0xac1fc
    c.b(C::Al,0xac22c,false)?;
    // VMA 0xac200
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // VMA 0xac204
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac208
    c.b(C::Eq,0xac584,false)?;
    // VMA 0xac20c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,580)?)?;
    // VMA 0xac210
    c.b(C::Al,0x2038,true)?;
    // VMA 0xac214
    c.b(C::Al,0xac584,false)?;
    // VMA 0xac218
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac21c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,18)?)?;
    // VMA 0xac220
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // VMA 0xac224
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,44)?)?;
    // VMA 0xac228
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,52)?)?;
    // VMA 0xac22c
    c.literal(R::R1,0xac73c)?;
    // VMA 0xac230
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,48)?)?;
    // VMA 0xac234
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // VMA 0xac238
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xac23c
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac240
    c.literal(R::R1,0xac740)?;
    // VMA 0xac244
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xac248
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac24c
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,1943)?)?;
    // VMA 0xac250
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,76)?)?;
    // VMA 0xac258
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(10)?,4)?)?;
    // VMA 0xac25c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // VMA 0xac260
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,56)?)?;
    // VMA 0xac264
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1976)?)?;
    // VMA 0xac268
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,68)?)?;
    // VMA 0xac26c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1980)?)?;
    // VMA 0xac270
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,72)?)?;
    // VMA 0xac274
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac278
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::new(3)?.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac27c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,8)?)?;
    // VMA 0xac280
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,76)?)?;
    // VMA 0xac284
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac288
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac28c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xac290
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,576)?)?;
    // VMA 0xac294
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,32768)?)?;
    // VMA 0xac298
    c.b(C::Eq,0xac318,false)?;
    // VMA 0xac29c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac2a0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac2a4
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac2a8
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac2ac
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xac2b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // VMA 0xac2b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // VMA 0xac2b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // VMA 0xac2bc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,16)?)?;
    // VMA 0xac2c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // VMA 0xac2c4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // VMA 0xac2c8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,1912)?)?;
    // VMA 0xac2cc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,20)?)?;
    // VMA 0xac2d0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(9)?,1916)?)?;
    // VMA 0xac2d4
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,24)?)?;
    // VMA 0xac2d8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1920)?)?;
    // VMA 0xac2dc
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,28)?)?;
    // VMA 0xac2e0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1924)?)?;
    // VMA 0xac2e4
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,32)?)?;
    // VMA 0xac2e8
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,1928)?)?;
    // VMA 0xac2ec
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,36)?)?;
    // VMA 0xac2f0
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac2f4
    c.literal(R::R1,0xac744)?;
    // VMA 0xac2f8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac2fc
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac300
    c.literal(R::R1,0xac740)?;
    // VMA 0xac304
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac308
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac30c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // VMA 0xac310
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,40)?)?;
    // VMA 0xac314
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,576)?)?;
    // VMA 0xac318
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,2097152)?)?;
    // VMA 0xac31c
    c.b(C::Eq,0xac71c,false)?;
    // VMA 0xac320
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac324
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac328
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,316)?)?;
    // VMA 0xac32c
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac330
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac334
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac338
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xac33c
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(6)?,12)?)?;
    // VMA 0xac340
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,16)?)?;
    // VMA 0xac344
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,336)?)?;
    // VMA 0xac348
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // VMA 0xac34c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // VMA 0xac350
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,117440512)?)?;
    // VMA 0xac354
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,20)?)?;
    // VMA 0xac358
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,32)?)?;
    // VMA 0xac35c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,312)?)?;
    // VMA 0xac360
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,24)?)?;
    // VMA 0xac364
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,28)?)?;
    // VMA 0xac368
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,72)?)?;
    // VMA 0xac36c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,40)?)?;
    // VMA 0xac370
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,68)?)?;
    // VMA 0xac374
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,36)?)?;
    // VMA 0xac378
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,60)?)?;
    // VMA 0xac37c
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,44)?)?;
    // VMA 0xac380
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,64)?)?;
    // VMA 0xac384
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,48)?)?;
    // VMA 0xac388
    c.b(C::Eq,0xac39c,false)?;
    // VMA 0xac38c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,12)?)?;
    // VMA 0xac390
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,52)?)?;
    // VMA 0xac394
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,16)?)?;
    // VMA 0xac398
    c.b(C::Al,0xac3a8,false)?;
    // VMA 0xac39c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,4)?)?;
    // VMA 0xac3a0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,52)?)?;
    // VMA 0xac3a4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,8)?)?;
    // VMA 0xac3a8
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // VMA 0xac3ac
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,72)?)?;
    // VMA 0xac3b0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,56)?)?;
    // VMA 0xac3b4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,40)?)?;
    // VMA 0xac3b8
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Asr,8)?)?;
    // VMA 0xac3bc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,60)?)?;
    // VMA 0xac3c0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // VMA 0xac3c4
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R1,R::R0,Shift::Lsr,4,false)?)?;
    // VMA 0xac3c8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsr,6,false)?)?;
    // VMA 0xac3cc
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,64)?)?;
    // VMA 0xac3d0
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,68)?)?;
    // VMA 0xac3d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,88)?)?;
    // VMA 0xac3d8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // VMA 0xac3dc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,2048)?)?;
    // VMA 0xac3e0
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // VMA 0xac3e4
    c.b(C::Al,0x471c,true)?;
    // VMA 0xac3e8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,204)?)?;
    // VMA 0xac3ec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xac3f0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,132)?)?;
    // VMA 0xac3f4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // VMA 0xac3f8
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xac3fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,72)?)?;
    // VMA 0xac400
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac404
    c.b(C::Eq,0xac458,false)?;
    // VMA 0xac408
    c.literal(R::new(7)?,0xac748)?;
    // VMA 0xac40c
    c.literal(R::new(10)?,0xac740)?;
    // VMA 0xac410
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(6)?,76)?)?;
    // VMA 0xac414
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,32)?)?;
    // VMA 0xac418
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,64)?)?;
    // VMA 0xac41c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,96)?)?;
    // VMA 0xac420
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,0)?)?;
    // VMA 0xac424
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac428
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac42c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // VMA 0xac430
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // VMA 0xac434
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac438
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac43c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // VMA 0xac440
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac444
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // VMA 0xac448
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // VMA 0xac44c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,4)?)?;
    // VMA 0xac450
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // VMA 0xac454
    c.b(C::Cc,0xac414,false)?;
    // VMA 0xac458
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,24)?)?;
    // VMA 0xac45c
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,8)?)?;
    // VMA 0xac460
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,12)?)?;
    // VMA 0xac464
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xac468
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,336)?)?;
    // VMA 0xac46c
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,576)?)?;
    // VMA 0xac470
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,8388608)?)?;
    // VMA 0xac474
    c.b(C::Eq,0xac4dc,false)?;
    // VMA 0xac478
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,20)?)?;
    // VMA 0xac47c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac480
    c.b(C::Ne,0xac724,false)?;
    // VMA 0xac484
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,1000)?)?;
    // VMA 0xac488
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1024)?)?;
    // VMA 0xac48c
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xac490
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xac494
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac498
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xac49c
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac4a0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,48)?)?;
    // VMA 0xac4a4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    // VMA 0xac4a8
    c.b(C::Al,0x31ec,true)?;
    // VMA 0xac4ac
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,76)?)?;
    // VMA 0xac4b0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac4b4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac4b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,25)?)?;
    // VMA 0xac4bc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // VMA 0xac4c0
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // VMA 0xac4c4
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac4c8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,8)?)?;
    // VMA 0xac4cc
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xac4d0
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    // VMA 0xac4d4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,76)?)?;
    // VMA 0xac4d8
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac4dc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // VMA 0xac4e0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // VMA 0xac4e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // VMA 0xac4e8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // VMA 0xac4ec
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3)?)?;
    // VMA 0xac4f0
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R2.bit()|R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    // VMA 0xac4f4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,8)?)?;
    // VMA 0xac4f8
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,12)?)?;
    // VMA 0xac4fc
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,16)?)?;
    // VMA 0xac500
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,580)?)?;
    // VMA 0xac504
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,96)?)?;
    // VMA 0xac508
    c.b(C::Al,0x205c,true)?;
    // VMA 0xac50c
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,12)?)?;
    // VMA 0xac510
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1908)?)?;
    // VMA 0xac514
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac518
    c.b(C::Eq,0xac52c,false)?;
    // VMA 0xac51c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac520
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8)?)?;
    // VMA 0xac524
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac528
    c.b(C::Al,0xac538,false)?;
    // VMA 0xac52c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1904)?)?;
    // VMA 0xac530
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac534
    c.b(C::Ne,0xac51c,false)?;
    // VMA 0xac538
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,2516)?)?;
    // VMA 0xac53c
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac540
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac544
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // VMA 0xac548
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xac54c
    c.b(C::Eq,0xac6e0,false)?;
    // VMA 0xac550
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,88)?)?;
    // VMA 0xac554
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // VMA 0xac558
    c.b(C::Eq,0xac6f0,false)?;
    // VMA 0xac55c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1896)?)?;
    // VMA 0xac560
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac564
    c.b(C::Ne,0xac700,false)?;
    // VMA 0xac568
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // VMA 0xac56c
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // VMA 0xac570
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,52)?)?;
    // VMA 0xac574
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,52)?)?;
    // VMA 0xac578
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,60)?)?;
    // VMA 0xac57c
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,27)?)?;
    // VMA 0xac580
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,44)?)?;
    // VMA 0xac584
    c.b(C::Al,0x26754,true)?;
    // VMA 0xac588
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // VMA 0xac58c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac590
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,52)?)?;
    // VMA 0xac594
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,92)?)?;
    // VMA 0xac598
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // VMA 0xac59c
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // VMA 0xac5a0
    c.literal(R::R1,0xac74c)?;
    // VMA 0xac5a4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,12)?)?;
    // VMA 0xac5a8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xac5ac
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac5b0
    c.literal(R::R1,0xac740)?;
    // VMA 0xac5b4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // VMA 0xac5b8
    c.b(C::Al,0x2ec94,true)?;
    // VMA 0xac5bc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,720)?)?;
    // VMA 0xac5c0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,864)?)?;
    // VMA 0xac5c4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xac5c8
    c.b(C::Eq,0xac5dc,false)?;
    // VMA 0xac5cc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xac5d0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // VMA 0xac5d4
    c.b(C::Al,0x2a18c,true)?;
    // VMA 0xac5d8
    c.b(C::Al,0xac5e4,false)?;
    // VMA 0xac5dc
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // VMA 0xac5e0
    c.b(C::Al,0x2a878,true)?;
    // VMA 0xac5e4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // VMA 0xac5e8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // VMA 0xac5ec
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // VMA 0xac5f0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,1872)?)?;
    // VMA 0xac5f4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // VMA 0xac5f8
    c.b(C::Eq,0xac60c,false)?;
    // VMA 0xac5fc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,1184)?)?;
    // VMA 0xac600
    c.b(C::Al,0x7fd1c,true)?;
    // VMA 0xac604
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xac608
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1872)?)?;
    // VMA 0xac60c
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,1)?)?;
    // VMA 0xac610
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(9)?,28)?)?;
    // VMA 0xac614
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac618
    c.b(C::Ne,0xabce0,false)?;
    // VMA 0xac61c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,32)?)?;
    // VMA 0xac620
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // VMA 0xac624
    c.b(C::Ne,0xabce0,false)?;
    // VMA 0xac628
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,948)?)?;
    // VMA 0xac62c
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xac630
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // VMA 0xac634
    c.b(C::Al,0xda564,true)?;
    // VMA 0xac638
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // VMA 0xac63c
    c.b(C::Al,0xac5e8,false)?;
    // VMA 0xac640
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,1884)?)?;
    // VMA 0xac644
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,4)?)?;
    // VMA 0xac648
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(5)?,R::new(7)?)?)?;
    // VMA 0xac64c
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // VMA 0xac650
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xac654
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // VMA 0xac658
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,68)?)?;
    // VMA 0xac65c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,868)?)?;
    // VMA 0xac660
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1024)?)?;
    // VMA 0xac664
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(9)?,1880)?)?;
    // VMA 0xac668
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::new(4)?,R::new(9)?,R::R0)?)?;
    // VMA 0xac66c
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,12)?)?;
    // VMA 0xac670
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(9)?,8)?)?;
    // VMA 0xac674
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(10)?,R::new(4)?)?)?;
    // VMA 0xac678
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xac67c
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xac680
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,8)?)?;
    // VMA 0xac684
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,1)?)?;
    // VMA 0xac688
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,72)?)?;
    // VMA 0xac68c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,98)?)?;
    // VMA 0xac690
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1792)?)?;
    // VMA 0xac694
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xac698
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // VMA 0xac69c
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R1,R::new(5)?)?)?;
    // VMA 0xac6a0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // VMA 0xac6a4
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xac6a8
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,8)?)?;
    // VMA 0xac6ac
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R0,Shift::Lsl,1,false)?)?;
    // VMA 0xac6b0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,12)?)?;
    // VMA 0xac6b4
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,60)?)?;
    // VMA 0xac6b8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,102)?)?;
    // VMA 0xac6bc
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1792)?)?;
    // VMA 0xac6c0
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(9)?,R::R0)?)?;
    // VMA 0xac6c4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // VMA 0xac6c8
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,8)?)?;
    // VMA 0xac6cc
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R1,R::new(10)?)?)?;
    // VMA 0xac6d0
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // VMA 0xac6d4
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,12)?)?;
    // VMA 0xac6d8
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // VMA 0xac6dc
    c.b(C::Al,0xabd94,false)?;
    // VMA 0xac6e0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac6e4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // VMA 0xac6e8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac6ec
    c.b(C::Al,0xac550,false)?;
    // VMA 0xac6f0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac6f4
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32)?)?;
    // VMA 0xac6f8
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac6fc
    c.b(C::Al,0xac55c,false)?;
    // VMA 0xac700
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac704
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,536870912)?)?;
    // VMA 0xac708
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    // VMA 0xac70c
    c.b(C::Al,0xac568,false)?;
    // VMA 0xac710
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,12)?)?;
    // VMA 0xac714
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // VMA 0xac718
    c.b(C::Al,0xac0b8,false)?;
    // VMA 0xac71c
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,24)?)?;
    // VMA 0xac720
    c.b(C::Al,0xac470,false)?;
    // VMA 0xac724
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,2080)?)?;
    // VMA 0xac728
    c.b(C::Al,0xac48c,false)?;
    c.check(0xac72c)
}
