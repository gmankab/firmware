// SPDX-License-Identifier: GPL-3.0-only

pub const IMAGE_BYTES: usize = 37_680;
pub const INITIAL_PREFIX_BYTES: usize = 400;
pub const MODELLED_BYTES: usize = 1288;
const PENDSV_EXTENSION_BYTES: usize = 118;
pub const STARTUP_BYTES: usize = 204;
const BLOCKS: [(u32,u32);7] = [
    (0x0,48), (0x200,320), (0x400,14_524), (0x4000,14_936),
    (0x40000,6_944), (0x43fe0,20), (0x57000,776),
];
const LOAD_MARKER: u32 = 0xefbe_adde;
pub struct Span { pub offset:usize, pub data:Vec<u8> }

fn ldr_literal(register:u8,distance:u16)->u16 {
    assert!(register<8 && distance<=1020 && distance%4==0);
    0x4800|(u16::from(register)<<8)|(distance>>2)
}
fn msr_special(register:u8,kind:u8)->[u16;2] {
    assert!(register<16 && matches!(kind,8|9|17));
    [0xf380|u16::from(register),0x8800|u16::from(kind)]
}
fn str_word(src:u8,base:u8,offset:u8)->u16 {
    assert!(src<8 && base<8 && offset<=124 && offset%4==0);
    0x6000|((u16::from(offset)>>2)<<6)|(u16::from(base)<<3)|u16::from(src)
}
fn b_w(from:u32,to:u32)->[u16;2] {
    let delta=to as i64-from as i64-4;
    assert!((-16_777_216..=16_777_214).contains(&delta) && delta%2==0);
    let imm=delta as u32;
    let s=(imm>>24)&1;
    let i1=(imm>>23)&1;
    let i2=(imm>>22)&1;
    let j1=1^i1^s;
    let j2=1^i2^s;
    [(0xf000|(s<<10)|((imm>>12)&0x3ff)) as u16,
     (0x9000|(j1<<13)|0x1000|(j2<<11)|((imm>>1)&0x7ff)) as u16]
}
fn bl(from:u32,to:u32)->[u16;2] {
    assert!(from%2==0 && to%2==0);
    let mut words=b_w(from,to);
    words[1]|=0x4000;
    words
}
fn adr(register:u8,from:u32,to:u32)->u16 {
    let pc=(from.checked_add(4).unwrap())&!3;
    let distance=to.checked_sub(pc).unwrap();
    assert!(register<8 && from%2==0 && distance<=1020 && distance%4==0);
    0xa000|(u16::from(register)<<8)|(distance as u16>>2)
}
fn ldm_w(base:u8,registers:u16,writeback:bool)->[u16;2] {
    assert!(base<15 && registers.count_ones()>=2 && registers&0xa000==0);
    assert!(!writeback || registers&(1u16<<base)==0);
    [0xe890|(u16::from(writeback)<<5)|u16::from(base),registers]
}
fn add_register(dst:u8,src:u8)->u16 {
    assert!(dst<15 && src<15);
    0x4400|((u16::from(dst)&8)<<4)|(u16::from(src)<<3)|(u16::from(dst)&7)
}
fn sub_immediate_w(dst:u8,src:u8,value:u8)->[u16;2] {
    assert!(dst<13 && src<13);
    [0xf1a0|u16::from(src),(u16::from(dst)<<8)|u16::from(value)]
}
fn cmp_register(lhs:u8,rhs:u8)->u16 {
    assert!(lhs<15 && rhs<15);
    0x4500|((u16::from(lhs)&8)<<4)|(u16::from(rhs)<<3)|(u16::from(lhs)&7)
}
fn cmp_register_low(lhs:u8,rhs:u8)->u16 {
    assert!(lhs<8 && rhs<8);
    0x4280|(u16::from(rhs)<<3)|u16::from(lhs)
}
fn bne(from:u32,to:u32)->u16 {
    let delta=i64::from(to)-i64::from(from)-4;
    assert!(from%2==0 && to%2==0 && (-256..=254).contains(&delta));
    0xd100|u16::from((delta/2) as i8 as u8)
}
fn adr_sub_w(dst:u8,from:u32,to:u32)->[u16;2] {
    let pc=from.checked_add(4).unwrap()&!3;
    let distance=pc.checked_sub(to).unwrap();
    assert!(dst<15 && dst!=13 && from%2==0 && distance<=4095);
    [0xf2af|(((distance>>11) as u16)<<10),
     (((distance>>8) as u16&7)<<12)|(u16::from(dst)<<8)|(distance as u16&255)]
}
fn tst_immediate_w(register:u8,value:u8)->[u16;2] {
    assert!(register<13 || register==14);
    [0xf010|u16::from(register),0x0f00|u16::from(value)]
}
fn it_ne()->u16 { 0xbf18 }
fn sub_register(dst:u8,lhs:u8,rhs:u8)->u16 {
    assert!(dst<8 && lhs<8 && rhs<8);
    0x1a00|(u16::from(rhs)<<6)|(u16::from(lhs)<<3)|u16::from(dst)
}
fn orr_immediate_w(dst:u8,src:u8,value:u8)->[u16;2] {
    assert!(dst<13 && src<13);
    [0xf040|u16::from(src),(u16::from(dst)<<8)|u16::from(value)]
}
fn bx(register:u8)->u16 {
    assert!(register<15);
    0x4700|(u16::from(register)<<3)
}
#[derive(Clone,Copy)]
enum Condition { Eq=0, Ne=1, Hs=2, Lo=3, Mi=4, Pl=5, Hi=8, Ge=10 }
#[derive(Clone,Copy)]
enum ItBlock { One, Then, Else, ThenThen, ElseElse, ThenThenThen, ThenElseElse }
fn it(condition:Condition,block:ItBlock)->u16 {
    let (mask,invert)=match block {
        ItBlock::One=>(8,0), ItBlock::Then=>(4,8), ItBlock::Else=>(12,8),
        ItBlock::ThenThen=>(2,12), ItBlock::ElseElse=>(14,12),
        ItBlock::ThenThenThen=>(1,14), ItBlock::ThenElseElse=>(7,14),
    };
    0xbf00|((condition as u16)<<4)|(mask^if condition as u16&1!=0 {invert} else {0})
}
fn b_cond(condition:Condition,from:u32,to:u32)->u16 {
    let delta=i64::from(to)-i64::from(from)-4;
    assert!(from%2==0 && to%2==0 && (-256..=254).contains(&delta));
    0xd000|((condition as u16)<<8)|u16::from((delta/2) as i8 as u8)
}
fn b_cond_w(condition:Condition,from:u32,to:u32)->[u16;2] {
    let delta=i64::from(to)-i64::from(from)-4;
    assert!(from%2==0 && to%2==0 && (-1_048_576..=1_048_574).contains(&delta));
    let imm=delta as u32;
    [(0xf000|(((imm>>20)&1)<<10)|((condition as u32)<<6)|((imm>>12)&0x3f)) as u16,
     (0x8000|(((imm>>18)&1)<<13)|(((imm>>19)&1)<<11)|((imm>>1)&0x7ff)) as u16]
}
fn subs_immediate(register:u8,value:u8)->u16 {
    assert!(register<8);
    0x3800|(u16::from(register)<<8)|u16::from(value)
}
fn movs_immediate(register:u8,value:u8)->u16 {
    assert!(register<8);
    0x2000|(u16::from(register)<<8)|u16::from(value)
}
fn cmp_immediate(register:u8,value:u8)->u16 {
    assert!(register<8);
    0x2800|(u16::from(register)<<8)|u16::from(value)
}
fn lsls_immediate(dst:u8,src:u8,shift:u8)->u16 {
    assert!(dst<8 && src<8 && shift<32);
    (u16::from(shift)<<6)|(u16::from(src)<<3)|u16::from(dst)
}
fn ldm(base:u8,registers:u8)->u16 {
    assert!(base<8 && registers!=0 && registers&(1u8<<base)==0);
    0xc800|(u16::from(base)<<8)|u16::from(registers)
}
fn stm(base:u8,registers:u8)->u16 {
    assert!(base<8 && registers!=0);
    assert!(registers&(1u8<<base)==0 || registers.trailing_zeros()==u32::from(base));
    0xc000|(u16::from(base)<<8)|u16::from(registers)
}
fn ldr_word(dst:u8,base:u8,offset:u8)->u16 {
    assert!(dst<8 && base<8 && offset<=124 && offset%4==0);
    0x6800|((u16::from(offset)>>2)<<6)|(u16::from(base)<<3)|u16::from(dst)
}
fn push(registers:u16)->u16 {
    assert!(registers!=0 && registers&!0x40ff==0);
    0xb400|((registers>>6)&0x100)|(registers&0xff)
}
fn pop(registers:u16)->u16 {
    assert!(registers!=0 && registers&!0x80ff==0);
    0xbc00|((registers>>7)&0x100)|(registers&0xff)
}
fn mov_register(dst:u8,src:u8)->u16 {
    assert!(dst<15 && src<15);
    0x4600|((u16::from(dst)&8)<<4)|(u16::from(src)<<3)|(u16::from(dst)&7)
}
fn mrs_special(dst:u8,kind:u8)->[u16;2] {
    assert!(dst<13 && matches!(kind,8|9|17));
    [0xf3ef,0x8000|(u16::from(dst)<<8)|u16::from(kind)]
}
fn stmdb_w(base:u8,registers:u16)->[u16;2] {
    assert!(base<15 && registers.count_ones()>=2 && registers&0xa000==0);
    assert!(registers&(1u16<<base)==0);
    [0xe920|u16::from(base),registers]
}
fn str_word_w(src:u8,base:u8,offset:u16)->[u16;2] {
    assert!(src<15 && src!=13 && base<15 && offset<=4095);
    [0xf8c0|u16::from(base),(u16::from(src)<<12)|offset]
}
fn add_immediate_w(dst:u8,src:u8,value:u8)->[u16;2] {
    assert!(dst<13 && src<13);
    [0xf100|u16::from(src),(u16::from(dst)<<8)|u16::from(value)]
}
fn b(from:u32,to:u32)->u16 {
    let delta=i64::from(to)-i64::from(from)-4;
    assert!(from%2==0 && to%2==0 && (-2048..=2046).contains(&delta));
    0xe000|((delta/2) as u16&0x7ff)
}
#[derive(Clone,Copy)]
enum Alu { And, Bic, Orr, Adc, Sbc, Sub, Add }
#[derive(Clone,Copy)]
enum Shift { Lsl=0, Lsr=1 }
fn alu_w(op:Alu,set_flags:bool,dst:u8,lhs:u8,rhs:u8,shift:Shift,amount:u8)->[u16;2] {
    assert!(dst<15 && dst!=13 && lhs<15 && lhs!=13 && rhs<15 && rhs!=13 && amount<32);
    let opcode=match op { Alu::And=>0xea00, Alu::Bic=>0xea20, Alu::Orr=>0xea40,
        Alu::Add=>0xeb00, Alu::Adc=>0xeb40, Alu::Sbc=>0xeb60, Alu::Sub=>0xeba0 };
    [opcode|(u16::from(set_flags)<<4)|u16::from(lhs),
     (u16::from(amount>>2)<<12)|(u16::from(dst)<<8)|(u16::from(amount&3)<<6)|((shift as u16)<<4)|u16::from(rhs)]
}
fn alu_immediate_w(op:Alu,dst:u8,src:u8,value:u8)->[u16;2] {
    assert!(dst<13 && src<13);
    let opcode=match op { Alu::And=>0xf000, Alu::Bic=>0xf020, Alu::Orr=>0xf040,
        Alu::Add=>0xf100, Alu::Adc=>0xf140, Alu::Sbc=>0xf160, Alu::Sub=>0xf1a0 };
    [opcode|u16::from(src),(u16::from(dst)<<8)|u16::from(value)]
}
fn mov_immediate_w(dst:u8,value:u16)->[u16;2] {
    assert!(dst<13);
    let immediate=if value<256 {value} else {
        assert!(value.is_power_of_two());
        let rotation=39-value.trailing_zeros() as u16;
        assert!((8..=31).contains(&rotation));
        rotation<<7
    };
    [0xf04f|((immediate>>11)<<10),((immediate>>8&7)<<12)|(u16::from(dst)<<8)|(immediate&255)]
}
fn rsb_immediate_w(dst:u8,src:u8,value:u8)->[u16;2] {
    assert!(dst<13 && src<13);
    [0xf1c0|u16::from(src),(u16::from(dst)<<8)|u16::from(value)]
}
fn shift_register_w(kind:Shift,dst:u8,src:u8,amount:u8)->[u16;2] {
    assert!(dst<13 && src<13 && amount<13);
    [0xfa00|((kind as u16)<<5)|u16::from(src),0xf000|(u16::from(dst)<<8)|u16::from(amount)]
}
fn lsr_immediate_w(dst:u8,src:u8,amount:u8)->[u16;2] {
    assert!(dst<13 && src<13 && (1..=32).contains(&amount));
    let amount=amount&31;
    [0xea4f,(u16::from(amount>>2)<<12)|(u16::from(dst)<<8)|(u16::from(amount&3)<<6)|0x10|u16::from(src)]
}
fn clz(dst:u8,src:u8)->[u16;2] {
    assert!(dst<13 && src<13);
    [0xfab0|u16::from(src),0xf080|(u16::from(dst)<<8)|u16::from(src)]
}
fn udiv(dst:u8,lhs:u8,rhs:u8)->[u16;2] {
    assert!(dst<13 && lhs<13 && rhs<13);
    [0xfbb0|u16::from(lhs),0xf0f0|(u16::from(dst)<<8)|u16::from(rhs)]
}
fn umull(low:u8,high:u8,lhs:u8,rhs:u8)->[u16;2] {
    assert!(low<13 && high<13 && low!=high && lhs<13 && rhs<13);
    [0xfba0|u16::from(lhs),(u16::from(low)<<12)|(u16::from(high)<<8)|u16::from(rhs)]
}
fn multiply_accumulate(subtract:bool,dst:u8,lhs:u8,rhs:u8,acc:u8)->[u16;2] {
    assert!(dst<13 && lhs<13 && rhs<13 && acc<13);
    [0xfb00|u16::from(lhs),(u16::from(acc)<<12)|(u16::from(dst)<<8)|(u16::from(subtract)<<4)|u16::from(rhs)]
}
fn stack_registers_w(load:bool,registers:u16)->[u16;2] {
    assert!(registers.count_ones()>=2 && registers&0x2000==0);
    assert!(if load {registers&0xc000!=0xc000} else {registers&0x8000==0});
    [if load {0xe8bd} else {0xe92d},registers]
}
fn adds_small(dst:u8,src:u8,value:u8)->u16 {
    assert!(dst<8 && src<8 && value<8);
    0x1c00|(u16::from(value)<<6)|(u16::from(src)<<3)|u16::from(dst)
}
fn adds_immediate(dst:u8,value:u8)->u16 {
    assert!(dst<8);
    0x3000|(u16::from(dst)<<8)|u16::from(value)
}
fn logical_low(exclusive:bool,dst:u8,src:u8)->u16 {
    assert!(dst<8 && src<8);
    (if exclusive {0x4040} else {0x4300})|(u16::from(src)<<3)|u16::from(dst)
}
fn cbz(register:u8,from:u32,to:u32)->u16 {
    let delta=to.checked_sub(from+4).unwrap();
    assert!(register<8 && from%2==0 && delta<=126 && delta%2==0);
    0xb100|(((delta as u16>>6)&1)<<9)|(((delta as u16>>1)&31)<<3)|u16::from(register)
}
fn blx(register:u8)->u16 { assert!(register<15); 0x4780|(u16::from(register)<<3) }
fn stack_address(dst:u8,offset:u16)->u16 {
    assert!(dst<8 && offset<=1020 && offset%4==0);
    0xa800|(u16::from(dst)<<8)|(offset>>2)
}
fn stack_load(dst:u8,offset:u16)->u16 {
    assert!(dst<8 && offset<=1020 && offset%4==0);
    0x9800|(u16::from(dst)<<8)|(offset>>2)
}
fn load_pair(first:u8,second:u8,base:u8,offset:u16)->[u16;2] {
    assert!(first<13 && second<13 && first!=second && base<15 && offset<=1020 && offset%4==0);
    [0xe9d0|u16::from(base),(u16::from(first)<<12)|(u16::from(second)<<8)|(offset>>2)]
}
fn load_postindexed(dst:u8,base:u8,offset:u8)->[u16;2] {
    assert!(dst<16 && base<15 && dst!=base);
    [0xf850|u16::from(base),(u16::from(dst)<<12)|0x0b00|u16::from(offset)]
}
fn negate(dst:u8,src:u8)->u16 {
    assert!(dst<8 && src<8);
    0x4240|(u16::from(src)<<3)|u16::from(dst)
}
fn store_byte_register(src:u8,base:u8,index:u8)->u16 {
    assert!(src<8 && base<8 && index<8);
    0x5400|(u16::from(index)<<6)|(u16::from(base)<<3)|u16::from(src)
}
fn nop_w()->[u16;2] { [0xf3af,0x8000] }
fn wfi()->u16 { 0xbf30 }
fn push16(out:&mut Vec<u8>,word:u16) { out.extend_from_slice(&word.to_le_bytes()); }
fn push32(out:&mut Vec<u8>,words:[u16;2]) { for word in words { push16(out,word); } }

/// Reset vectors, typed Thumb-2 prologue and PC-relative MMIO literal pool.
pub fn reset_block()->Vec<u8> {
    let mut out=Vec::with_capacity(48);
    out.extend_from_slice(&0x43f80u32.to_le_bytes()); // initial MSP
    out.extend_from_slice(&9u32.to_le_bytes()); // Thumb reset handler
    push16(&mut out,ldr_literal(0,20));
    push32(&mut out,msr_special(0,8)); // initialize MSP
    push16(&mut out,ldr_literal(0,20));
    push32(&mut out,msr_special(0,9)); // initialize PSP
    push16(&mut out,ldr_literal(0,16));
    push16(&mut out,ldr_literal(1,20));
    push16(&mut out,str_word(0,1,0)); // SCB->VTOR
    push32(&mut out,b_w(0x1a,0x400));
    push16(&mut out,0xbf00); // NOP
    for word in [0x43f80u32,0xdeadbeef,0x200,0xe000ed08] {
        out.extend_from_slice(&word.to_le_bytes());
    }
    assert_eq!(out.len(),48);
    out
}

/// Eighty typed Cortex-M vector pointers (64 peripheral vectors included).
pub fn vector_table()->Vec<u8> {
    let mut entries=[0x53du32;80];
    entries[0]=0;
    entries[1]=9;
    entries[2..7].fill(0x4b5);
    entries[7..11].fill(0);
    entries[11]=0x37f9;
    entries[12..14].fill(0);
    entries[14]=0x4cd;
    entries[15]=0x37fb;
    let mut out=Vec::with_capacity(320);
    for entry in entries { out.extend_from_slice(&entry.to_le_bytes()); }
    out
}

pub fn startup()->Vec<u8> {
    let mut out=Vec::with_capacity(STARTUP_BYTES);
    push32(&mut out,bl(0x400,0x408));
    push32(&mut out,bl(0x404,0x496));
    push16(&mut out,adr(0,0x408,0x434));
    push32(&mut out,ldm_w(0,(1<<10)|(1<<11),false));
    push16(&mut out,add_register(10,0));
    push16(&mut out,add_register(11,0));
    push32(&mut out,sub_immediate_w(7,10,1));
    push16(&mut out,cmp_register(10,11));
    push16(&mut out,bne(0x418,0x41e));
    push32(&mut out,bl(0x41a,0x496));
    push32(&mut out,adr_sub_w(14,0x41e,0x417));
    push32(&mut out,ldm_w(10,(1<<0)|(1<<1)|(1<<2)|(1<<3),true));
    push32(&mut out,tst_immediate_w(3,1));
    push16(&mut out,it_ne());
    push16(&mut out,sub_register(3,7,3));
    push32(&mut out,orr_immediate_w(3,3,1));
    push16(&mut out,bx(3));
    assert_eq!(out.len(),0x434-0x400);
    for address in [0x3c7cu32,0x3cbc] {
        out.extend_from_slice(&(address-0x434).to_le_bytes());
    }
    assert_eq!(out.len(),0x43c-0x400);
    push16(&mut out,subs_immediate(2,16));
    push16(&mut out,it(Condition::Hs,ItBlock::Then));
    push16(&mut out,ldm(0,0x78));
    push16(&mut out,stm(1,0x78));
    push16(&mut out,b_cond(Condition::Hi,0x444,0x43c));
    push16(&mut out,lsls_immediate(2,2,29));
    push16(&mut out,it(Condition::Hs,ItBlock::Then));
    push16(&mut out,ldm(0,0x30));
    push16(&mut out,stm(1,0x30));
    push16(&mut out,it(Condition::Mi,ItBlock::Then));
    push16(&mut out,ldr_word(4,0,0));
    push16(&mut out,str_word(4,1,0));
    push16(&mut out,bx(14));
    assert_eq!(out.len(),0x456-0x400);
    out.extend_from_slice(&0u16.to_le_bytes());
    assert_eq!(out.len(),0x458-0x400);
    for register in 3..=6 { push16(&mut out,movs_immediate(register,0)); }
    push16(&mut out,subs_immediate(2,16));
    push16(&mut out,it(Condition::Hs,ItBlock::One));
    push16(&mut out,stm(1,0x78));
    push16(&mut out,b_cond(Condition::Hi,0x466,0x460));
    push16(&mut out,lsls_immediate(2,2,29));
    push16(&mut out,it(Condition::Hs,ItBlock::One));
    push16(&mut out,stm(1,0x30));
    push16(&mut out,it(Condition::Mi,ItBlock::One));
    push16(&mut out,str_word(3,1,0));
    push16(&mut out,bx(14));
    assert_eq!(out.len(),0x474-0x400);
    push16(&mut out,cmp_immediate(1,100));
    push32(&mut out,b_cond_w(Condition::Eq,0x476,0x6b0));
    push16(&mut out,movs_immediate(0,0));
    push16(&mut out,bx(14));
    assert_eq!(out.len(),0x47e-0x400);
    push16(&mut out,push(0x401f));
    push32(&mut out,ldm_w(13,3,false));
    push32(&mut out,bl(0x484,0x1020));
    push32(&mut out,bl(0x488,0xa2c));
    push16(&mut out,pop(0x801f));
    assert_eq!(out.len(),0x48e-0x400);
    push16(&mut out,push(0x4010));
    push32(&mut out,bl(0x490,0xafe));
    push16(&mut out,pop(0x8010));
    assert_eq!(out.len(),0x496-0x400);
    push32(&mut out,bl(0x496,0xba0));
    push16(&mut out,mov_register(1,2));
    push32(&mut out,bl(0x49c,0x47e));
    push32(&mut out,bl(0x4a0,0x2c9c));
    push32(&mut out,bl(0x4a4,0xf8c));
    push16(&mut out,push(3));
    push32(&mut out,bl(0x4aa,0x48e));
    push16(&mut out,pop(3));
    push32(&mut out,bl(0x4b0,0x2b24));
    assert_eq!(out.len(),0x4b4-0x400);
    push32(&mut out,tst_immediate_w(14,4));
    push16(&mut out,it(Condition::Eq,ItBlock::Else));
    push32(&mut out,mrs_special(0,8));
    push32(&mut out,mrs_special(0,9));
    push16(&mut out,mov_register(1,14));
    push32(&mut out,bl(0x4c4,0x46c0));
    push16(&mut out,wfi());
    push16(&mut out,bx(14));
    assert_eq!(out.len(),0x4cc-0x400);
    assert_eq!(out.len(),STARTUP_BYTES);
    out
}

fn pendsv_extension()->Vec<u8> {
    let mut out=Vec::with_capacity(PENDSV_EXTENSION_BYTES);
    push16(&mut out,ldr_literal(1,0x52c-0x4d0));
    push16(&mut out,ldr_word(0,1,0));
    push16(&mut out,cmp_immediate(0,0));
    push16(&mut out,b_cond(Condition::Eq,0x4d2,0x51a));
    push16(&mut out,ldr_literal(0,0x530-0x4d8));
    push16(&mut out,ldr_word(0,0,0));
    push16(&mut out,ldr_literal(1,0x534-0x4dc));
    push16(&mut out,ldr_word(1,1,0));
    push16(&mut out,cmp_register_low(0,1));
    push16(&mut out,b_cond(Condition::Eq,0x4de,0x518));
    push16(&mut out,ldr_literal(2,0x538-0x4e4));
    push16(&mut out,ldr_word(2,2,0));
    push16(&mut out,cmp_immediate(2,0));
    push16(&mut out,b_cond(Condition::Eq,0x4e6,0x518));
    push32(&mut out,mrs_special(1,9));
    push16(&mut out,cmp_immediate(1,0));
    push16(&mut out,b_cond(Condition::Eq,0x4ee,0x502));
    push32(&mut out,mrs_special(3,17));
    push32(&mut out,stmdb_w(1,0x0ff8));
    push32(&mut out,msr_special(1,9));
    push16(&mut out,ldr_literal(0,0x530-0x500));
    push16(&mut out,ldr_word(2,0,0));
    push16(&mut out,str_word(1,2,0));
    assert_eq!(out.len(),0x502-0x4cc);
    push16(&mut out,ldr_literal(0,0x530-0x504));
    push16(&mut out,ldr_literal(1,0x534-0x508));
    push16(&mut out,ldr_word(1,1,0));
    push16(&mut out,str_word(1,0,0));
    push16(&mut out,ldr_word(1,1,0));
    push32(&mut out,ldm_w(1,0x0ff8,true));
    push32(&mut out,msr_special(3,17));
    push32(&mut out,msr_special(1,9));
    assert_eq!(out.len(),0x518-0x4cc);
    push16(&mut out,bx(14));
    assert_eq!(out.len(),0x51a-0x4cc);
    push32(&mut out,str_word_w(1,1,1));
    push32(&mut out,mrs_special(1,9));
    push32(&mut out,add_immediate_w(1,1,36));
    push32(&mut out,msr_special(1,9));
    push16(&mut out,b(0x52a,0x502));
    assert_eq!(out.len(),0x52c-0x4cc);
    for address in [0x40280u32,0x40298,0x4029c,0x40288] {
        out.extend_from_slice(&address.to_le_bytes());
    }
    assert_eq!(out.len(),0x53c-0x4cc);
    push32(&mut out,b_w(0x53c,0x728));
    assert_eq!(out.len(),0x540-0x4cc);
    push16(&mut out,bx(14));
    assert_eq!(out.len(),0x542-0x4cc);
    assert_eq!(out.len(),PENDSV_EXTENSION_BYTES);
    out
}

fn arithmetic_and_formatter()->Vec<u8> {
    let mut out=Vec::with_capacity(486);
    push32(&mut out,alu_w(Alu::Orr,true,12,3,2,Shift::Lsl,0));
    push32(&mut out,b_cond_w(Condition::Eq,0x546,0x61c));
    push32(&mut out,stack_registers_w(false,0x4bf0));
    push32(&mut out,mov_immediate_w(6,0));
    push16(&mut out,cmp_immediate(3,0));
    push16(&mut out,it(Condition::Ne,ItBlock::ThenThenThen));
    push32(&mut out,clz(5,3));
    push32(&mut out,shift_register_w(Shift::Lsl,4,3,5));
    push32(&mut out,shift_register_w(Shift::Lsr,6,4,5));
    push16(&mut out,logical_low(true,6,3));
    push16(&mut out,it(Condition::Ne,ItBlock::ElseElse));
    push16(&mut out,logical_low(false,6,2));
    push32(&mut out,clz(5,2));
    push32(&mut out,shift_register_w(Shift::Lsl,4,2,5));
    push32(&mut out,rsb_immediate_w(5,5,32));
    push16(&mut out,it(Condition::Ne,ItBlock::ThenThen));
    push32(&mut out,shift_register_w(Shift::Lsr,12,2,5));
    push32(&mut out,alu_w(Alu::Orr,false,4,4,12,Shift::Lsl,0));
    push16(&mut out,adds_immediate(5,32));
    push32(&mut out,alu_w(Alu::Orr,true,12,6,4,Shift::Lsl,16));
    push32(&mut out,lsr_immediate_w(4,4,16));
    push16(&mut out,it(Condition::Ne,ItBlock::One));
    push16(&mut out,adds_small(4,4,1));
    push32(&mut out,mov_immediate_w(8,0));
    push32(&mut out,mov_immediate_w(9,0));
    push16(&mut out,cmp_register_low(0,2));
    push32(&mut out,alu_w(Alu::Sbc,true,12,1,3,Shift::Lsl,0));
    push16(&mut out,b_cond(Condition::Lo,0x59a,0x610));
    push16(&mut out,cmp_immediate(1,0));
    push16(&mut out,it(Condition::Ne,ItBlock::ThenElseElse));
    push32(&mut out,clz(7,1));
    push32(&mut out,shift_register_w(Shift::Lsl,6,1,7));
    push32(&mut out,clz(7,0));
    push32(&mut out,shift_register_w(Shift::Lsl,6,0,7));
    push32(&mut out,rsb_immediate_w(7,7,32));
    push16(&mut out,it(Condition::Ne,ItBlock::ThenThen));
    push32(&mut out,shift_register_w(Shift::Lsr,12,0,7));
    push32(&mut out,alu_w(Alu::Orr,false,6,6,12,Shift::Lsl,0));
    push16(&mut out,adds_immediate(7,32));
    push32(&mut out,udiv(12,6,4));
    push32(&mut out,alu_w(Alu::Sub,false,7,7,5,Shift::Lsl,0));
    push16(&mut out,subs_immediate(7,16));
    push32(&mut out,alu_immediate_w(Alu::And,11,7,31));
    push32(&mut out,rsb_immediate_w(6,11,32));
    push32(&mut out,shift_register_w(Shift::Lsl,11,12,11));
    push32(&mut out,shift_register_w(Shift::Lsr,6,12,6));
    push16(&mut out,it(Condition::Mi,ItBlock::Then));
    push16(&mut out,mov_register(11,6));
    push16(&mut out,movs_immediate(6,0));
    push16(&mut out,cmp_immediate(7,32));
    push16(&mut out,it(Condition::Ge,ItBlock::Then));
    push16(&mut out,mov_register(6,11));
    push32(&mut out,mov_immediate_w(11,0));
    push32(&mut out,alu_w(Alu::Orr,true,12,11,6,Shift::Lsl,0));
    push16(&mut out,it(Condition::Eq,ItBlock::One));
    push32(&mut out,mov_immediate_w(11,1));
    push32(&mut out,alu_w(Alu::Add,true,9,9,11,Shift::Lsl,0));
    push32(&mut out,umull(7,12,11,2));
    push32(&mut out,alu_w(Alu::Adc,false,8,8,6,Shift::Lsl,0));
    push16(&mut out,sub_register(0,0,7));
    push32(&mut out,multiply_accumulate(false,12,6,2,12));
    push32(&mut out,multiply_accumulate(false,12,11,3,12));
    push32(&mut out,alu_w(Alu::Sbc,true,1,1,12,Shift::Lsl,0));
    push16(&mut out,b(0x60e,0x594));
    push16(&mut out,mov_register(3,1));
    push16(&mut out,mov_register(2,0));
    push16(&mut out,mov_register(1,8));
    push16(&mut out,mov_register(0,9));
    push32(&mut out,stack_registers_w(true,0x8bf0));
    push16(&mut out,push(0x4013));
    push32(&mut out,mov_immediate_w(0,0));
    push32(&mut out,mov_immediate_w(1,0));
    push32(&mut out,nop_w());
    push32(&mut out,stack_registers_w(true,0x401c));
    push16(&mut out,bx(14));
    push16(&mut out,push(15));
    push16(&mut out,ldr_literal(1,0x644-0x634));
    push16(&mut out,push(0x4010));
    push16(&mut out,stack_address(2,12));
    push16(&mut out,stack_load(0,8));
    push32(&mut out,bl(0x63a,0x97c));
    push16(&mut out,pop(0x10));
    push32(&mut out,load_postindexed(15,13,20));
    assert_eq!(out.len(),0x644-0x542);
    out.extend_from_slice(&0x4227cu32.to_le_bytes());
    push32(&mut out,stack_registers_w(false,0x41f0));
    push16(&mut out,mov_register(6,1));
    push16(&mut out,mov_register(4,0));
    push16(&mut out,movs_immediate(0,0));
    push16(&mut out,str_word(0,4,32));
    push16(&mut out,mov_register(0,4));
    push16(&mut out,ldr_word(1,4,12));
    push16(&mut out,blx(1));
    push16(&mut out,cbz(0,0x65a,0x6aa));
    push16(&mut out,cmp_immediate(0,37));
    push16(&mut out,b_cond(Condition::Eq,0x65e,0x66c));
    push32(&mut out,load_pair(2,1,4,4));
    push16(&mut out,blx(2));
    push16(&mut out,ldr_word(0,4,32));
    push16(&mut out,adds_small(0,0,1));
    push16(&mut out,b(0x66a,0x652));
    push16(&mut out,ldr_word(1,4,12));
    push16(&mut out,mov_register(0,4));
    push16(&mut out,movs_immediate(7,0));
    push16(&mut out,blx(1));
    push16(&mut out,lsls_immediate(5,0,0));
    push16(&mut out,b_cond(Condition::Eq,0x676,0x6aa));
    push32(&mut out,sub_immediate_w(0,5,65));
    push16(&mut out,cmp_immediate(0,25));
    push16(&mut out,b_cond(Condition::Hi,0x67e,0x686));
    push16(&mut out,adds_immediate(5,32));
    push32(&mut out,mov_immediate_w(7,0x800));
    push16(&mut out,mov_register(2,6));
    push16(&mut out,mov_register(1,5));
    push16(&mut out,mov_register(0,4));
    push16(&mut out,str_word(7,4,0));
    push32(&mut out,bl(0x68e,0x474));
    push16(&mut out,cbz(0,0x692,0x6a6));
    push16(&mut out,cmp_immediate(0,1));
    push16(&mut out,b_cond(Condition::Eq,0x696,0x6a2));
    push16(&mut out,adds_small(6,6,7));
    push32(&mut out,alu_immediate_w(Alu::Bic,6,6,7));
    push16(&mut out,adds_immediate(6,8));
    push16(&mut out,b(0x6a0,0x654));
    push16(&mut out,adds_small(6,6,4));
    push16(&mut out,b(0x6a4,0x654));
    push16(&mut out,mov_register(0,5));
    push16(&mut out,b(0x6a8,0x660));
    push16(&mut out,ldr_word(0,4,32));
    push32(&mut out,stack_registers_w(true,0x81f0));
    push32(&mut out,stack_registers_w(false,0x41f0));
    push16(&mut out,mov_register(6,0));
    push16(&mut out,movs_immediate(4,0));
    push16(&mut out,ldr_word(0,2,0));
    push16(&mut out,cmp_immediate(1,b'u'));
    push16(&mut out,mov_register(1,6));
    push16(&mut out,adr(5,0x6be,0x718));
    push16(&mut out,b_cond(Condition::Eq,0x6c0,0x6e4));
    push32(&mut out,nop_w());
    push16(&mut out,cmp_immediate(0,0));
    push16(&mut out,b_cond(Condition::Ge,0x6c8,0x6d0));
    push16(&mut out,negate(0,0));
    push16(&mut out,adr(5,0x6cc,0x71c));
    push16(&mut out,b(0x6ce,0x6e0));
    push16(&mut out,ldr_word(1,6,0));
    push16(&mut out,lsls_immediate(2,1,30));
    push16(&mut out,b_cond(Condition::Pl,0x6d4,0x6da));
    push16(&mut out,adr(5,0x6d6,0x720));
    push16(&mut out,b(0x6d8,0x6e0));
    push16(&mut out,lsls_immediate(1,1,29));
    push16(&mut out,b_cond(Condition::Pl,0x6dc,0x6e8));
    push16(&mut out,adr(5,0x6de,0x724));
    push16(&mut out,movs_immediate(4,1));
    push16(&mut out,b(0x6e2,0x6e8));
    push32(&mut out,nop_w());
    push16(&mut out,movs_immediate(1,0));
    push16(&mut out,movs_immediate(2,10));
    push32(&mut out,add_immediate_w(7,6,36));
    push16(&mut out,b(0x6f0,0x706));
    push32(&mut out,udiv(12,0,2));
    push32(&mut out,udiv(3,0,2));
    push32(&mut out,multiply_accumulate(true,0,2,12,0));
    push16(&mut out,adds_immediate(0,b'0'));
    push16(&mut out,store_byte_register(0,7,1));
    push16(&mut out,mov_register(0,3));
    push16(&mut out,adds_small(1,1,1));
    push16(&mut out,cmp_immediate(0,0));
    push16(&mut out,b_cond(Condition::Ne,0x708,0x6f2));
    push16(&mut out,mov_register(3,4));
    push16(&mut out,mov_register(2,5));
    push16(&mut out,mov_register(0,6));
    push32(&mut out,stack_registers_w(true,0x41f0));
    push32(&mut out,b_w(0x714,0x8ca));
    assert_eq!(out.len(),0x718-0x542);
    for prefix in [0,b'-',b'+',b' '] {
        out.extend_from_slice(&[prefix,0,0,0]);
    }
    assert_eq!(out.len(),486);
    out
}

fn header(address:u32,size:u32)->Vec<u8> {
    let mut out=Vec::with_capacity(16);
    for word in [address,size,LOAD_MARKER,LOAD_MARKER] {
        out.extend_from_slice(&word.to_le_bytes());
    }
    out
}

pub fn spans()->Vec<Span> {
    let mut result=Vec::with_capacity(7);
    let mut offset=0;
    for (index,(address,size)) in BLOCKS.into_iter().enumerate() {
        let mut data=header(address,size);
        match index {
            0=>data.extend_from_slice(&reset_block()),
            1=>data.extend_from_slice(&vector_table()),
            2=>{
                data.extend_from_slice(&startup());
                assert_eq!(offset+data.len(),0x26c);
                data.extend_from_slice(&pendsv_extension());
                assert_eq!(offset+data.len(),0x2e2);
                data.extend_from_slice(&arithmetic_and_formatter());
                assert_eq!(offset+data.len(),0x4c8);
            },
            _=>{},
        }
        result.push(Span{offset,data});
        offset+=16+size as usize;
    }
    assert_eq!(offset,IMAGE_BYTES);
    assert_eq!(result.iter().map(|span|span.data.len()).sum::<usize>(),MODELLED_BYTES);
    result
}

pub fn first_two_blocks()->Vec<u8> {
    let all=spans();
    let mut result=all[0].data.clone();
    assert_eq!(result.len(),all[1].offset);
    result.extend_from_slice(&all[1].data);
    assert_eq!(result.len(),INITIAL_PREFIX_BYTES);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn typed_reset_vectors_and_thumb_encoding() {
        assert_eq!(ldr_literal(0,20),0x4805);
        assert_eq!(msr_special(0,8),[0xf380,0x8808]);
        assert_eq!(msr_special(0,9),[0xf380,0x8809]);
        assert_eq!(str_word(0,1,0),0x6008);
        assert_eq!(b_w(0x1a,0x400),[0xf000,0xb9f1]);
        assert_eq!(reset_block().len(),48);
        assert_eq!(vector_table().len(),320);
        assert_eq!(spans().len(),7);
        assert_eq!(first_two_blocks().len(),400);
        assert_eq!(startup().len(),204);
        assert_eq!(spans()[2].offset+16,416);
        assert_eq!(spans()[2].data.len(),824);
        assert_eq!(MODELLED_BYTES,1288);
    }

    #[test]
    fn startup_extension_encoding_and_bounds() {
        assert_eq!(it(Condition::Hs,ItBlock::Then),0xbf24);
        assert_eq!(it(Condition::Eq,ItBlock::Else),0xbf0c);
        assert_eq!(b_cond_w(Condition::Eq,0x476,0x6b0),[0xf000,0x811b]);
        assert_eq!(b_cond_w(Condition::Eq,0x100000,4),[0xf400,0x8000]);
        assert_eq!(b_cond_w(Condition::Eq,0,0x100002),[0xf03f,0xafff]);
        assert_eq!(subs_immediate(2,16),0x3a10);
        assert_eq!(movs_immediate(3,0),0x2300);
        assert_eq!(lsls_immediate(2,2,29),0x0752);
        assert_eq!(ldm(0,0x78),0xc878);
        assert_eq!(stm(1,0x78),0xc178);
        assert_eq!(push(0x401f),0xb51f);
        assert_eq!(pop(0x801f),0xbd1f);
        assert_eq!(mrs_special(0,8),[0xf3ef,0x8008]);
        assert_eq!(mrs_special(0,9),[0xf3ef,0x8009]);
        assert_eq!(startup()[0x56..0x58],[0,0]);
        assert_eq!(0x400+startup().len(),0x4cc);
        assert_eq!(spans()[2].offset+16+startup().len(),0x26c);
        assert_eq!(spans()[2].offset+spans()[2].data.len(),0x4c8);
        for encode in [
            || { b_cond(Condition::Hi,1,4); },
            || { b_cond(Condition::Hi,0,260); },
            || { b_cond_w(Condition::Eq,0,0x100004); },
            || { b_cond_w(Condition::Eq,0x100002,4); },
            || { b_cond_w(Condition::Eq,0,5); },
            || { subs_immediate(8,0); },
            || { movs_immediate(8,0); },
            || { cmp_immediate(8,0); },
            || { lsls_immediate(8,0,0); },
            || { lsls_immediate(0,8,0); },
            || { lsls_immediate(0,0,32); },
            || { ldm(8,1); },
            || { ldm(0,0); },
            || { ldm(0,1); },
            || { stm(8,1); },
            || { stm(0,0); },
            || { stm(1,3); },
            || { ldr_word(8,0,0); },
            || { ldr_word(0,8,0); },
            || { ldr_word(0,0,126); },
            || { ldr_word(0,0,2); },
            || { push(0); },
            || { push(0x8000); },
            || { pop(0); },
            || { pop(0x4000); },
            || { mov_register(15,0); },
            || { mov_register(0,15); },
            || { mrs_special(13,8); },
            || { mrs_special(0,7); },
        ] {
            assert!(std::panic::catch_unwind(encode).is_err());
        }
    }

    #[test]
    fn pendsv_encoding_and_protected_pool() {
        assert_eq!(cmp_register_low(0,1),0x4288);
        assert_eq!(mrs_special(3,17),[0xf3ef,0x8311]);
        assert_eq!(msr_special(3,17),[0xf383,0x8811]);
        assert_eq!(stmdb_w(1,0x0ff8),[0xe921,0x0ff8]);
        assert_eq!(str_word_w(1,1,1),[0xf8c1,0x1001]);
        assert_eq!(str_word_w(14,14,4095),[0xf8ce,0xefff]);
        assert_eq!(add_immediate_w(1,1,36),[0xf101,0x0124]);
        assert_eq!(b(0x52a,0x502),0xe7ea);
        assert_eq!(b(0x800,4),0xe400);
        assert_eq!(b(0,0x802),0xe3ff);
        let extension=pendsv_extension();
        assert_eq!(extension.len(),118);
        let mut pool=Vec::new();
        for address in [0x40280u32,0x40298,0x4029c,0x40288] {
            pool.extend_from_slice(&address.to_le_bytes());
        }
        assert_eq!(&extension[0x60..0x70],pool);
        assert_eq!(&extension[0x4e..0x52],[0xc1,0xf8,1,0x10]);
        assert_eq!(&extension[0x70..],[0,0xf0,0xf4,0xb8,0x70,0x47]);
        let plan=spans();
        let span=&plan[2];
        assert_eq!(span.offset+16,0x1a0);
        assert_eq!(&span.data[16..16+STARTUP_BYTES],startup());
        assert_eq!(&span.data[0x26c-span.offset..0x2e2-span.offset],extension);
        assert_eq!(plan.iter().map(|span|span.data.len()).sum::<usize>(),684+118+486);
        for encode in [
            || { cmp_register_low(8,0); },
            || { cmp_register_low(0,8); },
            || { msr_special(0,16); },
            || { msr_special(16,17); },
            || { mrs_special(0,18); },
            || { stmdb_w(15,3); },
            || { stmdb_w(0,2); },
            || { stmdb_w(0,3); },
            || { stmdb_w(0,0x2006); },
            || { stmdb_w(0,0x8006); },
            || { str_word_w(15,0,0); },
            || { str_word_w(13,0,0); },
            || { str_word_w(0,15,0); },
            || { str_word_w(0,0,4096); },
            || { add_immediate_w(13,0,0); },
            || { add_immediate_w(0,13,0); },
            || { b(1,4); },
            || { b(0,5); },
            || { b(0,0x804); },
            || { b(0x802,4); },
        ] {
            assert!(std::panic::catch_unwind(encode).is_err());
        }
    }

    #[test]
    fn arithmetic_formatter_code_data_and_bounds() {
        let extension=arithmetic_and_formatter();
        let at=|start:usize,end:usize| &extension[start-0x542..end-0x542];
        assert_eq!(extension.len(),486);
        assert_eq!((0x644-0x542)+(0x718-0x648),466);
        assert_eq!((0x648-0x644)+(0x728-0x718),20);
        assert_eq!(at(0x644,0x648),0x4227cu32.to_le_bytes());
        assert_eq!(at(0x718,0x728),[0,0,0,0,b'-',0,0,0,b'+',0,0,0,b' ',0,0,0]);
        for address in [0x626,0x6c2,0x6e4] {
            assert_eq!(at(address,address+4),[0xaf,0xf3,0,0x80]);
        }
        assert_eq!(at(0x61c,0x630),[0x13,0xb5,0x4f,0xf0,0,0,0x4f,0xf0,0,1,
                                  0xaf,0xf3,0,0x80,0xbd,0xe8,0x1c,0x40,0x70,0x47]);
        assert_eq!(it(Condition::Ne,ItBlock::ThenThenThen),0xbf1f);
        assert_eq!(it(Condition::Ne,ItBlock::ElseElse),0xbf12);
        assert_eq!(it(Condition::Ne,ItBlock::ThenThen),0xbf1e);
        assert_eq!(it(Condition::Ne,ItBlock::ThenElseElse),0xbf19);
        assert_eq!(mov_immediate_w(7,0x800),[0xf44f,0x6700]);
        assert_eq!(alu_w(Alu::Orr,true,12,6,4,Shift::Lsl,16),[0xea56,0x4c04]);
        assert_eq!(udiv(12,0,2),[0xfbb0,0xfcf2]);
        assert_eq!(umull(7,12,11,2),[0xfbab,0x7c02]);
        assert_eq!(multiply_accumulate(true,0,2,12,0),[0xfb02,0x001c]);
        let plan=spans();
        let span=&plan[2];
        assert_eq!(&span.data[0x2e2-span.offset..],extension);
        assert_eq!(span.offset+span.data.len(),0x4c8);
        for encode in [
            || { alu_w(Alu::Orr,false,13,0,0,Shift::Lsl,0); },
            || { alu_w(Alu::Add,true,0,0,0,Shift::Lsl,32); },
            || { alu_immediate_w(Alu::And,13,0,0); },
            || { mov_immediate_w(0,0x801); },
            || { rsb_immediate_w(0,13,32); },
            || { shift_register_w(Shift::Lsr,0,0,13); },
            || { lsr_immediate_w(0,0,0); },
            || { clz(13,0); },
            || { udiv(0,13,0); },
            || { umull(0,0,1,2); },
            || { multiply_accumulate(false,0,0,0,13); },
            || { stack_registers_w(true,0xc000); },
            || { adds_small(0,0,8); },
            || { adds_immediate(8,0); },
            || { logical_low(false,0,8); },
            || { cbz(0,0,132); },
            || { blx(15); },
            || { stack_address(0,2); },
            || { stack_load(8,0); },
            || { load_pair(0,0,1,0); },
            || { load_postindexed(1,1,4); },
            || { negate(8,0); },
            || { store_byte_register(0,0,8); },
        ] {
            assert!(std::panic::catch_unwind(encode).is_err());
        }
    }

    #[test]
    fn startup_operand_bounds() {
        for encode in [
            || { adr(8,0x408,0x434); },
            || { adr(0,0x408,0x435); },
            || { bl(0x401,0x408); },
            || { bl(0x400,0x1000404); },
            || { ldm_w(10,(1<<10)|(1<<11),true); },
            || { ldm_w(0,0xa001,false); },
            || { add_register(15,0); },
            || { sub_immediate_w(13,0,1); },
            || { cmp_register(0,15); },
            || { bne(0x418,0x51c); },
            || { adr_sub_w(14,0x41e,0x421); },
            || { tst_immediate_w(13,1); },
            || { sub_register(8,0,0); },
            || { orr_immediate_w(13,0,1); },
            || { bx(15); },
        ] {
            assert!(std::panic::catch_unwind(encode).is_err());
        }
    }

    #[test]
    #[ignore = "requires authenticated OEM image in GMU_REFERENCE"]
    fn modelled_spans_match_reference() {
        let reference=std::fs::read(std::env::var_os("GMU_REFERENCE").unwrap()).unwrap();
        assert_eq!(reference.len(),IMAGE_BYTES);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
                   "3ff0c02708bbe78641db887fa62f3a7f9337934d0c2ce0b961ef7c43172591d2");
        for span in spans() {
            assert_eq!(span.data,reference[span.offset..span.offset+span.data.len()],
                       "span at file offset {:#x}",span.offset);
        }
    }
}
