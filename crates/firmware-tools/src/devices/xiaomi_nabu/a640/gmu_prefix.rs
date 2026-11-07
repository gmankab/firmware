// SPDX-License-Identifier: GPL-3.0-only

pub const IMAGE_BYTES: usize = 37_680;
pub const INITIAL_PREFIX_BYTES: usize = 400;
pub const MODELLED_BYTES: usize = 4312;
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
    assert!(dst<15 && src<16);
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
fn tst_immediate_w(register:u8,value:u32)->[u16;2] {
    assert!(register<13 || register==14);
    let immediate=modified_immediate(value);
    [0xf010|((immediate>>11)<<10)|u16::from(register),0x0f00|((immediate>>8&7)<<12)|(immediate&255)]
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
enum Condition { Eq=0, Ne=1, Hs=2, Lo=3, Mi=4, Pl=5, Hi=8, Ls=9, Ge=10, Lt=11, Gt=12, Le=13 }
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
fn modified_immediate(value:u32)->u16 {
    for immediate in 0u16..4096 {
        let byte=u32::from(immediate&255);
        let expanded=if immediate>>10==0 {
            match immediate>>8 {
                0=>byte,
                1 if byte!=0=>(byte<<16)|byte,
                2 if byte!=0=>(byte<<24)|(byte<<8),
                3 if byte!=0=>byte*0x01010101,
                _=>continue,
            }
        } else {
            (0x80|u32::from(immediate&127)).rotate_right(u32::from(immediate>>7))
        };
        if expanded==value { return immediate; }
    }
    panic!("unencodable Thumb immediate {value:#x}");
}
fn mov_immediate_w(dst:u8,value:u32)->[u16;2] {
    assert!(dst<13);
    let immediate=modified_immediate(value);
    [0xf04f|((immediate>>11)<<10),((immediate>>8&7)<<12)|(u16::from(dst)<<8)|(immediate&255)]
}
fn alu_immediate_flags_w(op:Alu,set_flags:bool,dst:u8,src:u8,value:u32)->[u16;2] {
    assert!(dst<13 && src<13);
    let immediate=modified_immediate(value);
    let mut words=alu_immediate_w(op,dst,src,(immediate&255) as u8);
    words[0]|=(u16::from(set_flags)<<4)|((immediate>>11)<<10);
    words[1]|=(immediate>>8&7)<<12;
    words
}
fn cmp_immediate_w(src:u8,value:u32)->[u16;2] {
    assert!(src<13);
    let immediate=modified_immediate(value);
    [0xf1b0|((immediate>>11)<<10)|u16::from(src),
     0x0f00|((immediate>>8&7)<<12)|(immediate&255)]
}
fn lsls_immediate_w(dst:u8,src:u8,amount:u8)->[u16;2] {
    assert!(dst<13 && src<13 && amount<32);
    [0xea5f,(u16::from(amount>>2)<<12)|(u16::from(dst)<<8)|(u16::from(amount&3)<<6)|u16::from(src)]
}
fn memory_postindexed(load:bool,width:u8,register:u8,base:u8,offset:u8)->[u16;2] {
    assert!(matches!(width,1|2|4) && register<15 && base<15 && register!=base);
    assert!(width==4 || register!=13);
    let size=match width {1=>0,2=>0x20,4=>0x40,_=>unreachable!()};
    [0xf800|size|(u16::from(load)<<4)|u16::from(base),(u16::from(register)<<12)|0x0b00|u16::from(offset)]
}
fn stm_w(base:u8,registers:u16)->[u16;2] {
    assert!(base<15 && registers.count_ones()>=2 && registers&0xa000==0);
    assert!(registers&(1u16<<base)==0);
    [0xe8a0|u16::from(base),registers]
}
fn load_byte(dst:u8,base:u8,offset:u8)->u16 {
    assert!(dst<8 && base<8 && offset<32);
    0x7800|(u16::from(offset)<<6)|(u16::from(base)<<3)|u16::from(dst)
}
fn load_byte_register_w(dst:u8,base:u8,index:u8)->[u16;2] {
    assert!(dst<13 && base<15 && index<13);
    [0xf810|u16::from(base),(u16::from(dst)<<12)|u16::from(index)]
}
fn subs_small(dst:u8,src:u8,value:u8)->u16 {
    assert!(dst<8 && src<8 && value<8);
    0x1e00|(u16::from(value)<<6)|(u16::from(src)<<3)|u16::from(dst)
}
fn adds_register(dst:u8,lhs:u8,rhs:u8)->u16 {
    assert!(dst<8 && lhs<8 && rhs<8);
    0x1800|(u16::from(rhs)<<6)|(u16::from(lhs)<<3)|u16::from(dst)
}
fn bics(dst:u8,src:u8)->u16 {
    assert!(dst<8 && src<8);
    0x4380|(u16::from(src)<<3)|u16::from(dst)
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
fn stack_adjust(subtract:bool,bytes:u16)->u16 {
    assert!(bytes<=508 && bytes%4==0);
    0xb000|(u16::from(subtract)<<7)|(bytes>>2)
}
fn stack_store(src:u8,offset:u16)->u16 {
    assert!(src<8 && offset<=1020 && offset%4==0);
    0x9000|(u16::from(src)<<8)|(offset>>2)
}
fn store_pair(first:u8,second:u8,base:u8,offset:u16)->[u16;2] {
    assert!(first<13 && second<13 && base<15 && offset<=1020 && offset%4==0);
    [0xe9c0|u16::from(base),(u16::from(first)<<12)|(u16::from(second)<<8)|(offset>>2)]
}
fn cbnz(register:u8,from:u32,to:u32)->u16 { cbz(register,from,to)|0x0800 }
fn movs_register(dst:u8,src:u8)->u16 { lsls_immediate(dst,src,0) }
fn nop_w()->[u16;2] { [0xf3af,0x8000] }
fn wfi()->u16 { 0xbf30 }
fn movw(dst:u8,value:u16)->[u16;2] {
    assert!(dst<13);
    [0xf240|((value>>11&1)<<10)|(value>>12),
     ((value>>8&7)<<12)|(u16::from(dst)<<8)|(value&255)]
}
fn mvn_immediate_w(dst:u8,value:u32)->[u16;2] {
    let mut words=mov_immediate_w(dst,value);
    words[0]|=0x20;
    words
}
fn ands(dst:u8,src:u8)->u16 {
    assert!(dst<8 && src<8);
    0x4000|(u16::from(src)<<3)|u16::from(dst)
}
fn mvns(dst:u8,src:u8)->u16 {
    assert!(dst<8 && src<8);
    0x43c0|(u16::from(src)<<3)|u16::from(dst)
}
fn lsrs_immediate(dst:u8,src:u8,amount:u8)->u16 {
    assert!(dst<8 && src<8 && (1..=32).contains(&amount));
    0x0800|(u16::from(amount&31)<<6)|(u16::from(src)<<3)|u16::from(dst)
}
fn uxtb(dst:u8,src:u8)->u16 {
    assert!(dst<8 && src<8);
    0xb2c0|(u16::from(src)<<3)|u16::from(dst)
}
fn load_halfword(dst:u8,base:u8,offset:u8)->u16 {
    assert!(dst<8 && base<8 && offset<=62 && offset%2==0);
    0x8800|(u16::from(offset/2)<<6)|(u16::from(base)<<3)|u16::from(dst)
}
fn store_byte(src:u8,base:u8,offset:u8)->u16 {
    assert!(src<8 && base<8 && offset<32);
    0x7000|(u16::from(offset)<<6)|(u16::from(base)<<3)|u16::from(src)
}
fn load_byte_negative(dst:u8,base:u8,offset:u8)->[u16;2] {
    assert!(dst<13 && base<15 && offset!=0);
    [0xf810|u16::from(base),(u16::from(dst)<<12)|0x0c00|u16::from(offset)]
}

fn nop()->u16 { 0xbf00 }
fn tst_register_low(lhs:u8,rhs:u8)->u16 {
    assert!(lhs<8 && rhs<8);
    0x4200|(u16::from(rhs)<<3)|u16::from(lhs)
}
fn store_byte_w(src:u8,base:u8,offset:u16)->[u16;2] {
    assert!(src<15 && src!=13 && base<15 && offset<4096);
    [0xf880|u16::from(base),(u16::from(src)<<12)|offset]
}

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

fn runtime_memory_and_formatter()->Vec<u8> {
    let mut out=Vec::with_capacity(726);
    macro_rules! h { ($at:expr,$word:expr) => {{ assert_eq!(0x728+out.len(),$at); push16(&mut out,$word); }}; }
    macro_rules! w { ($at:expr,$words:expr) => {{ assert_eq!(0x728+out.len(),$at); push32(&mut out,$words); }}; }
    h!(0x728,push(0x4010));
    h!(0x72a,movs_immediate(0,0));
    w!(0x72c,bl(0x72c,0x9a0));
    w!(0x730,bl(0x730,0x48e));
    w!(0x734,stack_registers_w(true,0x4010));
    h!(0x738,movs_immediate(0,1));
    w!(0x73a,b_w(0x73a,0x2b24));
    h!(0x73e,push(0x4010));
    h!(0x740,adds_small(3,0,1));
    h!(0x742,b(0x742,0x74a));
    w!(0x744,memory_postindexed(true,1,1,0,1));
    h!(0x748,cbz(1,0x748,0x768));
    h!(0x74a,lsls_immediate(1,0,30));
    h!(0x74c,b_cond(Condition::Ne,0x74c,0x744));
    w!(0x74e,mov_immediate_w(2,0x01010101));
    h!(0x752,ldm(0,2));
    h!(0x754,sub_register(4,1,2));
    h!(0x756,bics(4,1));
    w!(0x758,alu_w(Alu::And,true,1,4,2,Shift::Lsl,7));
    h!(0x75c,b_cond(Condition::Eq,0x75c,0x752));
    h!(0x75e,sub_register(0,0,3));
    h!(0x760,lsls_immediate(2,1,24));
    h!(0x762,b_cond(Condition::Eq,0x762,0x76c));
    h!(0x764,subs_small(0,0,3));
    h!(0x766,pop(0x8010));
    h!(0x768,sub_register(0,0,3));
    h!(0x76a,pop(0x8010));
    h!(0x76c,lsls_immediate(2,1,16));
    h!(0x76e,b_cond(Condition::Eq,0x76e,0x774));
    h!(0x770,subs_small(0,0,2));
    h!(0x772,pop(0x8010));
    h!(0x774,lsls_immediate(1,1,8));
    h!(0x776,b_cond(Condition::Eq,0x776,0x772));
    h!(0x778,subs_small(0,0,1));
    h!(0x77a,pop(0x8010));
    h!(0x77c,cmp_immediate(2,3));
    w!(0x77e,b_cond_w(Condition::Ls,0x77e,0x7e2));
    w!(0x782,alu_immediate_flags_w(Alu::And,true,12,0,3));
    w!(0x786,b_cond_w(Condition::Eq,0x786,0x7b4));
    w!(0x78a,memory_postindexed(true,1,3,1,1));
    w!(0x78e,cmp_immediate_w(12,2));
    h!(0x792,add_register(2,12));
    h!(0x794,it(Condition::Ls,ItBlock::One));
    w!(0x796,memory_postindexed(true,1,12,1,1));
    w!(0x79a,memory_postindexed(false,1,3,0,1));
    h!(0x79e,it(Condition::Lo,ItBlock::One));
    w!(0x7a0,memory_postindexed(true,1,3,1,1));
    w!(0x7a4,sub_immediate_w(2,2,4));
    h!(0x7a8,it(Condition::Ls,ItBlock::One));
    w!(0x7aa,memory_postindexed(false,1,12,0,1));
    h!(0x7ae,it(Condition::Lo,ItBlock::One));
    w!(0x7b0,memory_postindexed(false,1,3,0,1));
    w!(0x7b4,alu_immediate_flags_w(Alu::And,true,3,1,3));
    w!(0x7b8,b_cond_w(Condition::Eq,0x7b8,0x806));
    h!(0x7bc,subs_immediate(2,8));
    w!(0x7be,b_cond_w(Condition::Lo,0x7be,0x7d2));
    w!(0x7c2,load_postindexed(3,1,4));
    h!(0x7c6,subs_immediate(2,8));
    w!(0x7c8,load_postindexed(12,1,4));
    w!(0x7cc,stm_w(0,0x1008));
    h!(0x7d0,b(0x7d0,0x7be));
    h!(0x7d2,adds_small(2,2,4));
    h!(0x7d4,it(Condition::Pl,ItBlock::Then));
    w!(0x7d6,load_postindexed(3,1,4));
    w!(0x7da,memory_postindexed(false,4,3,0,4));
    w!(0x7de,nop_w());
    h!(0x7e2,lsls_immediate(2,2,31));
    h!(0x7e4,it(Condition::Hs,ItBlock::Then));
    w!(0x7e6,memory_postindexed(true,1,3,1,1));
    w!(0x7ea,memory_postindexed(true,1,12,1,1));
    h!(0x7ee,it(Condition::Mi,ItBlock::One));
    w!(0x7f0,memory_postindexed(true,1,2,1,1));
    h!(0x7f4,it(Condition::Hs,ItBlock::Then));
    w!(0x7f6,memory_postindexed(false,1,3,0,1));
    w!(0x7fa,memory_postindexed(false,1,12,0,1));
    h!(0x7fe,it(Condition::Mi,ItBlock::One));
    w!(0x800,memory_postindexed(false,1,2,0,1));
    h!(0x804,bx(14));
    h!(0x806,push(0x4010));
    h!(0x808,subs_immediate(2,32));
    w!(0x80a,b_cond_w(Condition::Lo,0x80a,0x824));
    w!(0x80e,ldm_w(1,0x5018,true));
    h!(0x812,subs_immediate(2,32));
    w!(0x814,stm_w(0,0x5018));
    w!(0x818,ldm_w(1,0x5018,true));
    w!(0x81c,stm_w(0,0x5018));
    w!(0x820,b_cond_w(Condition::Hs,0x820,0x80e));
    w!(0x824,lsls_immediate_w(12,2,28));
    h!(0x828,it(Condition::Hs,ItBlock::Then));
    w!(0x82a,ldm_w(1,0x5018,true));
    w!(0x82e,stm_w(0,0x5018));
    h!(0x832,it(Condition::Mi,ItBlock::Then));
    h!(0x834,ldm(1,0x18));
    h!(0x836,stm(0,0x18));
    w!(0x838,stack_registers_w(true,0x4010));
    w!(0x83c,lsls_immediate_w(12,2,30));
    h!(0x840,it(Condition::Hs,ItBlock::Then));
    w!(0x842,load_postindexed(3,1,4));
    w!(0x846,memory_postindexed(false,4,3,0,4));
    h!(0x84a,it(Condition::Eq,ItBlock::One));
    h!(0x84c,bx(14));
    h!(0x84e,lsls_immediate(2,2,31));
    h!(0x850,it(Condition::Hs,ItBlock::One));
    w!(0x852,memory_postindexed(true,2,3,1,2));
    h!(0x856,it(Condition::Mi,ItBlock::One));
    w!(0x858,memory_postindexed(true,1,2,1,1));
    h!(0x85c,it(Condition::Hs,ItBlock::One));
    w!(0x85e,memory_postindexed(false,2,3,0,2));
    h!(0x862,it(Condition::Mi,ItBlock::One));
    w!(0x864,memory_postindexed(false,1,2,0,1));
    h!(0x868,bx(14));
    w!(0x86a,alu_immediate_w(Alu::And,3,2,255));
    w!(0x86e,alu_w(Alu::Orr,false,2,3,3,Shift::Lsl,8));
    w!(0x872,alu_w(Alu::Orr,false,2,2,2,Shift::Lsl,16));
    w!(0x876,b_w(0x876,0x9b2));
    w!(0x87a,mov_immediate_w(2,0));
    h!(0x87e,push(0x4000));
    h!(0x880,mov_register(3,2));
    h!(0x882,mov_register(12,2));
    h!(0x884,mov_register(14,2));
    h!(0x886,subs_immediate(1,32));
    h!(0x888,it(Condition::Hs,ItBlock::ThenThen));
    w!(0x88a,stm_w(0,0x500c));
    w!(0x88e,stm_w(0,0x500c));
    w!(0x892,alu_immediate_flags_w(Alu::Sub,true,1,1,32));
    w!(0x896,b_cond_w(Condition::Hs,0x896,0x888));
    h!(0x89a,lsls_immediate(1,1,28));
    h!(0x89c,it(Condition::Hs,ItBlock::One));
    w!(0x89e,stm_w(0,0x500c));
    h!(0x8a2,it(Condition::Mi,ItBlock::One));
    h!(0x8a4,stm(0,12));
    w!(0x8a6,load_postindexed(14,13,4));
    h!(0x8aa,lsls_immediate(1,1,2));
    h!(0x8ac,it(Condition::Hs,ItBlock::One));
    w!(0x8ae,memory_postindexed(false,4,2,0,4));
    h!(0x8b2,it(Condition::Eq,ItBlock::One));
    h!(0x8b4,bx(14));
    h!(0x8b6,it(Condition::Mi,ItBlock::One));
    w!(0x8b8,memory_postindexed(false,2,2,0,2));
    w!(0x8bc,tst_immediate_w(1,0x40000000));
    h!(0x8c0,it(Condition::Ne,ItBlock::One));
    w!(0x8c2,memory_postindexed(false,1,2,0,1));
    h!(0x8c6,bx(14));
    h!(0x8c8,bx(14));
    w!(0x8ca,stack_registers_w(false,0x47f0));
    h!(0x8ce,mov_register(5,1));
    h!(0x8d0,mov_register(9,3));
    h!(0x8d2,mov_register(10,2));
    h!(0x8d4,mov_register(4,0));
    w!(0x8d6,add_immediate_w(8,0,36));
    h!(0x8da,ldr_word(1,0,0));
    h!(0x8dc,lsls_immediate(0,1,26));
    h!(0x8de,b_cond(Condition::Pl,0x8de,0x8ea));
    h!(0x8e0,ldr_word(0,4,28));
    w!(0x8e2,alu_immediate_w(Alu::Bic,1,1,16));
    h!(0x8e6,str_word(1,4,0));
    h!(0x8e8,b(0x8e8,0x8ec));
    h!(0x8ea,movs_immediate(0,1));
    h!(0x8ec,cmp_register_low(0,5));
    h!(0x8ee,b_cond(Condition::Le,0x8ee,0x8f4));
    h!(0x8f0,sub_register(7,0,5));
    h!(0x8f2,b(0x8f2,0x8f6));
    h!(0x8f4,movs_immediate(7,0));
    h!(0x8f6,ldr_word(1,4,24));
    h!(0x8f8,adds_register(2,7,5));
    w!(0x8fa,alu_w(Alu::Add,false,0,2,9,Shift::Lsl,0));
    h!(0x8fe,sub_register(0,1,0));
    h!(0x900,str_word(0,4,24));
    h!(0x902,load_byte(0,4,0));
    h!(0x904,lsls_immediate(0,0,27));
    h!(0x906,b_cond(Condition::Mi,0x906,0x90e));
    h!(0x908,mov_register(0,4));
    w!(0x90a,nop_w());
    h!(0x90e,movs_immediate(6,0));
    h!(0x910,b(0x910,0x924));
    w!(0x912,load_pair(2,1,4,4));
    w!(0x916,load_byte_register_w(0,10,6));
    h!(0x91a,blx(2));
    h!(0x91c,ldr_word(0,4,32));
    h!(0x91e,adds_small(0,0,1));
    h!(0x920,adds_small(6,6,1));
    h!(0x922,str_word(0,4,32));
    h!(0x924,cmp_register(6,9));
    h!(0x926,b_cond(Condition::Lt,0x926,0x912));
    h!(0x928,load_byte(0,4,0));
    h!(0x92a,lsls_immediate(0,0,27));
    h!(0x92c,b_cond(Condition::Pl,0x92c,0x944));
    h!(0x92e,mov_register(0,4));
    w!(0x930,nop_w());
    h!(0x934,b(0x934,0x944));
    w!(0x936,load_pair(2,1,4,4));
    h!(0x93a,movs_immediate(0,b'0'));
    h!(0x93c,blx(2));
    h!(0x93e,ldr_word(0,4,32));
    h!(0x940,adds_small(0,0,1));
    h!(0x942,str_word(0,4,32));
    h!(0x944,subs_small(0,7,0));
    w!(0x946,sub_immediate_w(7,7,1));
    h!(0x94a,b_cond(Condition::Gt,0x94a,0x936));
    h!(0x94c,b(0x94c,0x95e));
    w!(0x94e,load_pair(2,1,4,4));
    w!(0x952,load_byte_register_w(0,8,5));
    h!(0x956,blx(2));
    h!(0x958,ldr_word(0,4,32));
    h!(0x95a,adds_small(0,0,1));
    h!(0x95c,str_word(0,4,32));
    h!(0x95e,subs_small(0,5,0));
    w!(0x960,sub_immediate_w(5,5,1));
    h!(0x964,b_cond(Condition::Gt,0x964,0x94e));
    h!(0x966,mov_register(0,4));
    w!(0x968,nop_w());
    h!(0x96c,load_byte(0,4,0));
    h!(0x96e,lsls_immediate(0,0,24));
    h!(0x970,b_cond(Condition::Pl,0x970,0x978));
    h!(0x972,movs_immediate(0,2));
    w!(0x974,stack_registers_w(true,0x87f0));
    h!(0x978,movs_immediate(0,1));
    h!(0x97a,b(0x97a,0x974));
    h!(0x97c,ldr_literal(3,0x99c-0x980));
    h!(0x97e,push(0x4070));
    h!(0x980,mov_register(5,1));
    h!(0x982,add_register(3,15));
    w!(0x984,bl(0x984,0x9fe));
    h!(0x988,mov_register(4,0));
    h!(0x98a,mov_register(0,5));
    w!(0x98c,bl(0x98c,0xa24));
    h!(0x990,cbz(0,0x990,0x998));
    w!(0x992,mov_immediate_w(0,u32::MAX));
    h!(0x996,pop(0x8070));
    h!(0x998,mov_register(0,4));
    h!(0x99a,pop(0x8070));
    assert_eq!(out.len(),0x99c-0x728);
    out.extend_from_slice(&0x2213u32.to_le_bytes());
    h!(0x9a0,push(0x4010));
    w!(0x9a2,bl(0x9a2,0xb70));
    w!(0x9a6,stack_registers_w(true,0x4010));
    w!(0x9aa,b_w(0x9aa,0xb64));
    w!(0x9ae,mov_immediate_w(2,0));
    h!(0x9b2,cmp_immediate(1,4));
    w!(0x9b4,b_cond_w(Condition::Lo,0x9b4,0x9dc));
    w!(0x9b8,alu_immediate_flags_w(Alu::And,true,12,0,3));
    w!(0x9bc,b_cond_w(Condition::Eq,0x9bc,0x87e));
    w!(0x9c0,rsb_immediate_w(12,12,4));
    w!(0x9c4,cmp_immediate_w(12,2));
    h!(0x9c8,it(Condition::Ne,ItBlock::One));
    w!(0x9ca,memory_postindexed(false,1,2,0,1));
    h!(0x9ce,it(Condition::Ge,ItBlock::One));
    w!(0x9d0,memory_postindexed(false,2,2,0,2));
    w!(0x9d4,alu_w(Alu::Sub,false,1,1,12,Shift::Lsl,0));
    w!(0x9d8,b_w(0x9d8,0x87e));
    w!(0x9dc,lsls_immediate_w(12,1,31));
    h!(0x9e0,it(Condition::Hs,ItBlock::Then));
    w!(0x9e2,memory_postindexed(false,1,2,0,1));
    w!(0x9e6,memory_postindexed(false,1,2,0,1));
    h!(0x9ea,it(Condition::Mi,ItBlock::One));
    w!(0x9ec,memory_postindexed(false,1,2,0,1));
    h!(0x9f0,bx(14));
    assert_eq!(out.len(),0x9f2-0x728);
    out.resize(out.len()+2,0);
    h!(0x9f4,ldr_word(1,0,16));
    h!(0x9f6,adds_small(2,1,1));
    h!(0x9f8,str_word(2,0,16));
    h!(0x9fa,load_byte(0,1,0));
    h!(0x9fc,bx(14));
    assert_eq!(out.len(),726);
    out
}

fn runtime_streams_and_arena()->Vec<u8> {
    let mut out=Vec::with_capacity(570);
    macro_rules! h { ($at:expr,$word:expr) => {{ assert_eq!(0x9fe+out.len(),$at); push16(&mut out,$word); }}; }
    macro_rules! w { ($at:expr,$words:expr) => {{ assert_eq!(0x9fe+out.len(),$at); push32(&mut out,$words); }}; }
    h!(0x9fe,push(0x4000));
    h!(0xa00,stack_adjust(true,60));
    w!(0xa02,store_pair(3,1,13,4));
    h!(0xa06,movs_immediate(1,0));
    h!(0xa08,stack_store(1,20));
    h!(0xa0a,ldr_literal(1,0xa20-0xa0c));
    h!(0xa0c,add_register(1,15));
    w!(0xa0e,store_pair(1,0,13,12));
    h!(0xa12,mov_register(1,2));
    h!(0xa14,mov_register(0,13));
    w!(0xa16,bl(0xa16,0x648));
    h!(0xa1a,stack_adjust(false,60));
    h!(0xa1c,pop(0x8000));
    assert_eq!(out.len(),0xa1e-0x9fe);
    out.resize(out.len()+2,0);
    out.extend_from_slice(&0x9f5u32.wrapping_sub(0xa0c+4).to_le_bytes());
    h!(0xa24,load_byte(0,0,12));
    w!(0xa26,alu_immediate_w(Alu::And,0,0,128));
    h!(0xa2a,bx(14));
    h!(0xa2c,push(0x4010));
    h!(0xa2e,ldr_literal(0,0xb38-0xa30));
    h!(0xa30,movs_immediate(1,84));
    w!(0xa32,bl(0xa32,0x87a));
    h!(0xa36,ldr_literal(0,0xb3c-0xa38));
    h!(0xa38,movs_immediate(1,84));
    w!(0xa3a,bl(0xa3a,0x87a));
    h!(0xa3e,ldr_literal(0,0xb40-0xa40));
    h!(0xa40,movs_immediate(1,84));
    w!(0xa42,bl(0xa42,0x87a));
    h!(0xa46,ldr_literal(1,0xb44-0xa48));
    h!(0xa48,ldr_literal(0,0xb38-0xa4c));
    h!(0xa4a,ldr_literal(2,0xb3c-0xa4c));
    h!(0xa4c,ldr_literal(3,0xb4c-0xa50));
    h!(0xa4e,str_word(0,1,0));
    h!(0xa50,ldr_literal(1,0xb48-0xa54));
    h!(0xa52,str_word(2,1,0));
    h!(0xa54,ldr_literal(1,0xb40-0xa58));
    h!(0xa56,str_word(1,3,0));
    h!(0xa58,adds_small(3,2,1));
    h!(0xa5a,str_word(3,0,80));
    h!(0xa5c,adds_small(0,1,1));
    h!(0xa5e,str_word(0,2,80));
    h!(0xa60,movs_immediate(0,1));
    h!(0xa62,str_word(0,1,80));
    h!(0xa64,ldr_literal(0,0xb54-0xa68));
    h!(0xa66,ldr_literal(2,0xb38-0xa68));
    h!(0xa68,adr(1,0xa68,0xb50));
    h!(0xa6a,add_register(0,15));
    w!(0xa6c,bl(0xa6c,0xe54));
    h!(0xa70,cbnz(0,0xa70,0xa7c));
    h!(0xa72,ldr_literal(0,0xb54-0xa74));
    h!(0xa74,add_register(0,15));
    h!(0xa76,subs_immediate(0,10));
    w!(0xa78,bl(0xa78,0xfd0));
    h!(0xa7c,ldr_literal(0,0xb5c-0xa80));
    h!(0xa7e,ldr_literal(2,0xb3c-0xa80));
    h!(0xa80,adr(1,0xa80,0xb58));
    h!(0xa82,add_register(0,15));
    w!(0xa84,bl(0xa84,0xe54));
    h!(0xa88,cbnz(0,0xa88,0xa94));
    h!(0xa8a,ldr_literal(0,0xb5c-0xa8c));
    h!(0xa8c,add_register(0,15));
    h!(0xa8e,subs_immediate(0,10));
    w!(0xa90,bl(0xa90,0xfd0));
    h!(0xa94,ldr_literal(0,0xb60-0xa98));
    h!(0xa96,ldr_literal(2,0xb40-0xa98));
    h!(0xa98,adr(1,0xa98,0xb58));
    h!(0xa9a,add_register(0,15));
    w!(0xa9c,bl(0xa9c,0xe54));
    h!(0xaa0,cbnz(0,0xaa0,0xaac));
    h!(0xaa2,ldr_literal(0,0xb60-0xaa4));
    h!(0xaa4,add_register(0,15));
    h!(0xaa6,subs_immediate(0,10));
    w!(0xaa8,bl(0xaa8,0xfd0));
    h!(0xaac,movs_immediate(3,64));
    h!(0xaae,ldr_literal(0,0xb38-0xab0));
    h!(0xab0,lsls_immediate(4,3,3));
    h!(0xab2,movs_immediate(1,0));
    h!(0xab4,mov_register(2,4));
    w!(0xab6,bl(0xab6,0xe0e));
    h!(0xaba,cbz(0,0xaba,0xac6));
    h!(0xabc,ldr_literal(0,0xb54-0xac0));
    h!(0xabe,add_register(0,15));
    h!(0xac0,subs_immediate(0,84));
    w!(0xac2,bl(0xac2,0xfd0));
    h!(0xac6,ldr_literal(0,0xb3c-0xac8));
    h!(0xac8,mov_register(2,4));
    h!(0xaca,movs_immediate(3,64));
    h!(0xacc,movs_immediate(1,0));
    w!(0xace,bl(0xace,0xe0e));
    h!(0xad2,cbz(0,0xad2,0xade));
    h!(0xad4,ldr_literal(0,0xb5c-0xad8));
    h!(0xad6,add_register(0,15));
    h!(0xad8,subs_immediate(0,84));
    w!(0xada,bl(0xada,0xfd0));
    h!(0xade,ldr_literal(0,0xb40-0xae0));
    h!(0xae0,mov_register(2,4));
    h!(0xae2,movs_immediate(3,16));
    h!(0xae4,movs_immediate(1,0));
    w!(0xae6,bl(0xae6,0xe0e));
    h!(0xaea,cmp_immediate(0,0));
    h!(0xaec,b_cond(Condition::Eq,0xaec,0xafc));
    h!(0xaee,ldr_literal(0,0xb60-0xaf0));
    h!(0xaf0,add_register(0,15));
    w!(0xaf2,stack_registers_w(true,0x4010));
    h!(0xaf6,subs_immediate(0,86));
    w!(0xaf8,b_w(0xaf8,0xfd0));
    h!(0xafc,pop(0x8010));
    h!(0xafe,ldr_literal(0,0xb40-0xb00));
    h!(0xb00,push(0x4070));
    h!(0xb02,ldr_word(0,0,80));
    w!(0xb04,alu_immediate_w(Alu::Bic,4,0,1));
    h!(0xb08,ldr_literal(0,0xb38-0xb0c));
    w!(0xb0a,bl(0xb0a,0xf40));
    h!(0xb0e,ldr_literal(0,0xb3c-0xb10));
    w!(0xb10,bl(0xb10,0xf40));
    h!(0xb14,ldr_literal(0,0xb40-0xb18));
    w!(0xb16,bl(0xb16,0xf40));
    h!(0xb1a,b(0xb1a,0xb30));
    h!(0xb1c,ldr_word(0,4,80));
    w!(0xb1e,alu_immediate_w(Alu::Bic,5,0,1));
    h!(0xb22,mov_register(0,4));
    w!(0xb24,bl(0xb24,0xf40));
    h!(0xb28,mov_register(0,4));
    w!(0xb2a,bl(0xb2a,0xbea));
    h!(0xb2e,mov_register(4,5));
    h!(0xb30,cmp_immediate(4,0));
    h!(0xb32,b_cond(Condition::Ne,0xb32,0xb1c));
    h!(0xb34,pop(0x8070));
    assert_eq!(out.len(),0xb36-0x9fe);
    out.resize(out.len()+2,0);
    for value in [0x42228u32,0x4227c,0x422d0,0x402a4,0x402a8,0x402ac,
                  0x72,0x2dae,0x77,0x2d9a,0x2d86] {
        out.extend_from_slice(&value.to_le_bytes());
    }
    h!(0xb64,cmp_immediate(0,0));
    h!(0xb66,b_cond(Condition::Eq,0xb66,0xb6c));
    w!(0xb68,b_w(0xb68,0x2b24));
    h!(0xb6c,bx(14));
    out.resize(out.len()+2,0);
    h!(0xb70,push(0x4010));
    h!(0xb72,movs_immediate(1,0));
    h!(0xb74,adr(0,0xb74,0xb80));
    w!(0xb76,bl(0xb76,0xf9e));
    h!(0xb7a,movs_immediate(0,1));
    h!(0xb7c,pop(0x8010));
    out.resize(out.len()+2,0);
    assert_eq!(out.len(),0xb80-0x9fe);
    out.extend_from_slice(b"SIGABRT: Abnormal termination");
    assert_eq!(out.len(),0xb9d-0x9fe);
    out.resize(out.len()+3,0);
    h!(0xba0,mov_register(5,14));
    w!(0xba2,bl(0xba2,0xfe0));
    h!(0xba6,mov_register(14,5));
    h!(0xba8,movs_register(5,0));
    h!(0xbaa,mov_register(1,13));
    h!(0xbac,mov_register(3,10));
    w!(0xbae,alu_immediate_w(Alu::Bic,0,0,7));
    h!(0xbb2,mov_register(13,0));
    h!(0xbb4,stack_adjust(false,96));
    h!(0xbb6,push(0x4020));
    w!(0xbb8,bl(0xbb8,0x2b0c));
    w!(0xbbc,stack_registers_w(true,0x4020));
    w!(0xbc0,mov_immediate_w(6,0));
    w!(0xbc4,mov_immediate_w(7,0));
    w!(0xbc8,mov_immediate_w(8,0));
    w!(0xbcc,mov_immediate_w(11,0));
    w!(0xbd0,alu_immediate_w(Alu::Bic,1,1,7));
    h!(0xbd4,mov_register(12,5));
    w!(0xbd6,stm_w(12,0x09c0));
    w!(0xbda,stm_w(12,0x09c0));
    w!(0xbde,stm_w(12,0x09c0));
    w!(0xbe2,stm_w(12,0x09c0));
    h!(0xbe6,mov_register(13,1));
    h!(0xbe8,bx(14));
    h!(0xbea,push(0x4070));
    h!(0xbec,mov_register(4,0));
    h!(0xbee,subs_small(5,0,4));
    w!(0xbf0,bl(0xbf0,0xfe8));
    h!(0xbf4,ldr_word(0,0,0));
    h!(0xbf6,cmp_immediate(4,0));
    h!(0xbf8,it(Condition::Ne,ItBlock::Else));
    h!(0xbfa,ldr_word(4,0,4));
    h!(0xbfc,pop(0x8070));
    h!(0xbfe,cmp_immediate(4,0));
    h!(0xc00,it(Condition::Ne,ItBlock::One));
    h!(0xc02,cmp_register_low(4,5));
    h!(0xc04,it(Condition::Lo,ItBlock::Then));
    h!(0xc06,mov_register(0,4));
    h!(0xc08,ldr_word(4,4,4));
    h!(0xc0a,b_cond(Condition::Lo,0xc0a,0xbfe));
    h!(0xc0c,ldr_word(1,0,0));
    h!(0xc0e,adds_register(2,1,0));
    h!(0xc10,cmp_register_low(2,5));
    h!(0xc12,it(Condition::Ne,ItBlock::One));
    h!(0xc14,str_word(5,0,4));
    h!(0xc16,b_cond(Condition::Ne,0xc16,0xc20));
    h!(0xc18,ldr_word(2,5,0));
    h!(0xc1a,mov_register(5,0));
    h!(0xc1c,add_register(1,2));
    h!(0xc1e,str_word(1,0,0));
    h!(0xc20,ldr_word(0,5,0));
    h!(0xc22,adds_register(1,0,5));
    h!(0xc24,cmp_register_low(1,4));
    h!(0xc26,it(Condition::Ne,ItBlock::Then));
    h!(0xc28,str_word(4,5,4));
    h!(0xc2a,pop(0x8070));
    h!(0xc2c,ldr_word(1,4,4));
    h!(0xc2e,str_word(1,5,4));
    h!(0xc30,ldr_word(1,4,0));
    h!(0xc32,add_register(0,1));
    h!(0xc34,str_word(0,5,0));
    h!(0xc36,pop(0x8070));
    assert_eq!(out.len(),570);
    out
}

fn runtime_stream_io()->Vec<u8> {
    let mut out=Vec::with_capacity(960);
    macro_rules! h { ($at:expr,$word:expr) => {{ assert_eq!(0xc38+out.len(),$at); push16(&mut out,$word); }}; }
    macro_rules! w { ($at:expr,$words:expr) => {{ assert_eq!(0xc38+out.len(),$at); push32(&mut out,$words); }}; }
    w!(0xc38,stack_registers_w(false,0x47f0));
    h!(0xc3c,mov_register(5,0));
    h!(0xc3e,mov_register(4,1));
    h!(0xc40,ldr_word(0,1,12));
    w!(0xc42,alu_immediate_flags_w(Alu::Bic,false,0,0,2621440));
    w!(0xc46,alu_immediate_flags_w(Alu::Orr,false,0,0,4194304));
    h!(0xc4a,str_word(0,1,12));
    h!(0xc4c,lsls_immediate(0,0,26));
    h!(0xc4e,b_cond(Condition::Pl,0xc4e,0xc56));
    h!(0xc50,mov_register(0,1));
    w!(0xc52,bl(0xc52,0x1282));
    h!(0xc56,ldr_word(0,4,12));
    w!(0xc58,movw(1,4226));
    w!(0xc5c,mov_immediate_w(9,4294967295));
    h!(0xc60,ands(1,0));
    h!(0xc62,cmp_immediate(1,2));
    h!(0xc64,b_cond(Condition::Eq,0xc64,0xc6e));
    h!(0xc66,mov_register(0,4));
    w!(0xc68,bl(0xc68,0x11d4));
    h!(0xc6c,b(0xc6c,0xdda));
    w!(0xc6e,alu_immediate_flags_w(Alu::And,false,1,0,40960));
    w!(0xc72,cmp_immediate_w(1,32768));
    h!(0xc76,b_cond(Condition::Ne,0xc76,0xc98));
    h!(0xc78,lsls_immediate(0,0,15));
    h!(0xc7a,b_cond(Condition::Pl,0xc7a,0xc8a));
    h!(0xc7c,ldr_word(1,4,4));
    h!(0xc7e,ldr_word(0,4,44));
    h!(0xc80,cmp_register_low(0,1));
    h!(0xc82,b_cond(Condition::Hi,0xc82,0xc86));
    h!(0xc84,mov_register(0,1));
    h!(0xc86,str_word(0,4,4));
    h!(0xc88,b(0xc88,0xc98));
    h!(0xc8a,ldr_word(0,4,16));
    h!(0xc8c,str_word(0,4,44));
    h!(0xc8e,str_word(0,4,4));
    h!(0xc90,ldr_word(0,4,20));
    w!(0xc92,bl(0xc92,0x2b26));
    h!(0xc96,str_word(0,4,24));
    h!(0xc98,ldr_word(0,4,8));
    h!(0xc9a,movs_immediate(2,0));
    h!(0xc9c,uxtb(6,5));
    h!(0xc9e,cmp_immediate(0,0));
    h!(0xca0,b_cond(Condition::Ge,0xca0,0xcbe));
    h!(0xca2,ldr_word(1,4,12));
    h!(0xca4,lsls_immediate(3,1,22));
    h!(0xca6,b_cond(Condition::Mi,0xca6,0xcbe));
    h!(0xca8,mvns(0,0));
    h!(0xcaa,str_word(0,4,8));
    w!(0xcac,alu_immediate_flags_w(Alu::Orr,false,0,1,73728));
    h!(0xcb0,str_word(2,4,0));
    h!(0xcb2,str_word(0,4,12));
    h!(0xcb4,ldr_word(0,4,4));
    h!(0xcb6,adds_small(1,0,1));
    h!(0xcb8,str_word(1,4,4));
    h!(0xcba,store_byte(6,0,0));
    h!(0xcbc,b(0xcbc,0xe0a));
    h!(0xcbe,ldr_word(0,4,12));
    h!(0xcc0,mov_register(8,2));
    h!(0xcc2,str_word(2,4,0));
    w!(0xcc4,alu_immediate_flags_w(Alu::Orr,false,0,0,8192));
    h!(0xcc8,str_word(0,4,12));
    h!(0xcca,ldr_word(0,4,16));
    h!(0xccc,cbnz(0,0xccc,0xd2a));
    h!(0xcce,ldr_word(0,4,20));
    w!(0xcd0,bl(0xcd0,0x2b2c));
    h!(0xcd4,movs_immediate(7,1));
    w!(0xcd6,add_immediate_w(5,4,36));
    h!(0xcda,cbz(0,0xcda,0xd06));
    h!(0xcdc,load_halfword(0,4,12));
    w!(0xcde,tst_immediate_w(0,768));
    h!(0xce2,b_cond(Condition::Eq,0xce2,0xcf6));
    h!(0xce4,ldr_word(0,4,28));
    w!(0xce6,bl(0xce6,0x107e));
    h!(0xcea,str_word(0,4,16));
    h!(0xcec,cbz(0,0xcec,0xcf6));
    h!(0xcee,ldr_word(0,4,12));
    w!(0xcf0,alu_immediate_flags_w(Alu::Orr,false,0,0,2048));
    h!(0xcf4,b(0xcf4,0xd24));
    h!(0xcf6,str_word(7,4,28));
    h!(0xcf8,str_word(5,4,16));
    h!(0xcfa,ldr_word(0,4,12));
    w!(0xcfc,alu_immediate_flags_w(Alu::Bic,false,0,0,768));
    w!(0xd00,alu_immediate_flags_w(Alu::Orr,false,0,0,1024));
    h!(0xd04,b(0xd04,0xd24));
    h!(0xd06,ldr_word(0,4,28));
    w!(0xd08,bl(0xd08,0x107e));
    h!(0xd0c,cmp_immediate(0,0));
    h!(0xd0e,str_word(0,4,16));
    h!(0xd10,b_cond(Condition::Eq,0xd10,0xcf6));
    h!(0xd12,ldr_word(0,4,12));
    w!(0xd14,alu_immediate_flags_w(Alu::Orr,false,0,0,2048));
    w!(0xd18,tst_immediate_w(0,768));
    h!(0xd1c,str_word(0,4,12));
    h!(0xd1e,b_cond(Condition::Ne,0xd1e,0xd26));
    w!(0xd20,alu_immediate_flags_w(Alu::Orr,false,0,0,256));
    h!(0xd24,str_word(0,4,12));
    h!(0xd26,ldr_word(0,4,16));
    h!(0xd28,str_word(0,4,4));
    h!(0xd2a,ldr_word(0,4,12));
    h!(0xd2c,ldr_word(5,4,16));
    w!(0xd2e,mvn_immediate_w(10,1));
    h!(0xd32,lsls_immediate(1,0,23));
    h!(0xd34,b_cond(Condition::Pl,0xd34,0xd76));
    h!(0xd36,ldr_word(1,4,4));
    h!(0xd38,ldr_word(0,4,44));
    h!(0xd3a,cmp_register_low(0,1));
    h!(0xd3c,b_cond(Condition::Hi,0xd3c,0xd40));
    h!(0xd3e,mov_register(0,1));
    h!(0xd40,sub_register(1,0,5));
    h!(0xd42,b_cond(Condition::Eq,0xd42,0xd4e));
    h!(0xd44,mov_register(2,4));
    h!(0xd46,mov_register(0,5));
    w!(0xd48,bl(0xd48,0x11e8));
    h!(0xd4c,cbnz(0,0xd4c,0xdcc));
    h!(0xd4e,ldr_word(0,4,12));
    h!(0xd50,lsls_immediate(1,0,8));
    h!(0xd52,b_cond(Condition::Pl,0xd52,0xd5c));
    h!(0xd54,str_word(5,4,44));
    w!(0xd56,store_pair(5,8,4,4));
    h!(0xd5a,b(0xd5a,0xe06));
    h!(0xd5c,adds_small(1,5,1));
    h!(0xd5e,str_word(1,4,44));
    h!(0xd60,str_word(1,4,4));
    h!(0xd62,ldr_word(1,4,28));
    w!(0xd64,alu_immediate_flags_w(Alu::Orr,false,0,0,65536));
    h!(0xd68,subs_small(1,1,1));
    w!(0xd6a,store_pair(1,0,4,8));
    h!(0xd6e,mov_register(0,6));
    h!(0xd70,store_byte(6,5,0));
    w!(0xd72,stack_registers_w(true,0x87f0));
    h!(0xd76,lsls_immediate(1,0,8));
    h!(0xd78,b_cond(Condition::Pl,0xd78,0xd80));
    w!(0xd7a,alu_immediate_flags_w(Alu::And,false,2,0,65536));
    h!(0xd7e,b(0xd7e,0xd96));
    h!(0xd80,ldr_word(0,4,4));
    h!(0xd82,adds_small(1,0,1));
    h!(0xd84,str_word(1,4,4));
    h!(0xd86,store_byte(6,0,0));
    h!(0xd88,ldr_word(0,4,12));
    h!(0xd8a,cmp_immediate(6,10));
    w!(0xd8c,alu_immediate_flags_w(Alu::Orr,false,0,0,65536));
    h!(0xd90,str_word(0,4,12));
    h!(0xd92,b_cond(Condition::Eq,0xd92,0xda2));
    h!(0xd94,movs_immediate(2,0));
    h!(0xd96,ldr_word(1,4,44));
    h!(0xd98,ldr_word(0,4,4));
    h!(0xd9a,cmp_register_low(1,0));
    h!(0xd9c,b_cond(Condition::Ls,0xd9c,0xda6));
    h!(0xd9e,mov_register(3,1));
    h!(0xda0,b(0xda0,0xda8));
    h!(0xda2,movs_immediate(2,1));
    h!(0xda4,b(0xda4,0xd96));
    h!(0xda6,mov_register(3,0));
    h!(0xda8,load_halfword(7,4,12));
    h!(0xdaa,sub_register(3,3,5));
    h!(0xdac,lsls_immediate(7,7,21));
    h!(0xdae,b_cond(Condition::Mi,0xdae,0xdb8));
    h!(0xdb0,ldr_word(7,4,28));
    h!(0xdb2,cmp_register_low(7,3));
    h!(0xdb4,b_cond(Condition::Le,0xdb4,0xdb8));
    h!(0xdb6,cbz(2,0xdb6,0xdf2));
    h!(0xdb8,cmp_register_low(1,0));
    h!(0xdba,b_cond(Condition::Hi,0xdba,0xdbe));
    h!(0xdbc,mov_register(1,0));
    h!(0xdbe,str_word(5,4,44));
    h!(0xdc0,sub_register(7,1,0));
    w!(0xdc2,store_pair(5,8,4,4));
    h!(0xdc6,cmp_immediate(3,0));
    h!(0xdc8,b_cond(Condition::Le,0xdc8,0xdde));
    h!(0xdca,b(0xdca,0xdce));
    h!(0xdcc,b(0xdcc,0xdda));
    h!(0xdce,mov_register(2,4));
    h!(0xdd0,mov_register(1,3));
    h!(0xdd2,mov_register(0,5));
    w!(0xdd4,bl(0xdd4,0x11e8));
    h!(0xdd8,cbz(0,0xdd8,0xdde));
    h!(0xdda,mov_register(0,9));
    h!(0xddc,b(0xddc,0xd72));
    h!(0xdde,cbz(7,0xdde,0xdf2));
    h!(0xde0,ldr_word(0,4,24));
    h!(0xde2,str_word(5,4,44));
    h!(0xde4,str_word(5,4,4));
    h!(0xde6,sub_register(0,0,7));
    h!(0xde8,str_word(0,4,24));
    h!(0xdea,ldr_word(0,4,12));
    w!(0xdec,alu_immediate_flags_w(Alu::Orr,false,0,0,16));
    h!(0xdf0,str_word(0,4,12));
    h!(0xdf2,ldr_word(0,4,12));
    h!(0xdf4,lsls_immediate(0,0,8));
    h!(0xdf6,b_cond(Condition::Pl,0xdf6,0xe0a));
    h!(0xdf8,ldr_word(1,4,4));
    h!(0xdfa,ldr_word(0,4,24));
    h!(0xdfc,str_word(5,4,44));
    h!(0xdfe,sub_register(1,1,5));
    h!(0xe00,add_register(0,1));
    h!(0xe02,str_word(5,4,4));
    h!(0xe04,str_word(0,4,24));
    h!(0xe06,mov_register(0,10));
    h!(0xe08,b(0xe08,0xd72));
    h!(0xe0a,mov_register(0,6));
    h!(0xe0c,b(0xe0c,0xd72));
    h!(0xe0e,push(0x4070));
    h!(0xe10,ldr_word(4,0,12));
    h!(0xe12,lsls_immediate(5,4,30));
    h!(0xe14,b_cond(Condition::Eq,0xe14,0xe3e));
    h!(0xe16,lsls_immediate(5,4,9));
    h!(0xe18,b_cond(Condition::Mi,0xe18,0xe3e));
    w!(0xe1a,cmp_immediate_w(2,256));
    h!(0xe1e,b_cond(Condition::Eq,0xe1e,0xe34));
    w!(0xe20,cmp_immediate_w(2,512));
    h!(0xe24,b_cond(Condition::Eq,0xe24,0xe34));
    w!(0xe26,cmp_immediate_w(2,1024));
    h!(0xe2a,b_cond(Condition::Ne,0xe2a,0xe3e));
    h!(0xe2c,movs_immediate(3,1));
    w!(0xe2e,add_immediate_w(1,0,36));
    h!(0xe32,b(0xe32,0xe42));
    h!(0xe34,subs_small(5,3,1));
    w!(0xe36,mvn_immediate_w(6,4278190080));
    h!(0xe3a,cmp_register_low(5,6));
    h!(0xe3c,b_cond(Condition::Lo,0xe3c,0xe42));
    h!(0xe3e,movs_immediate(0,1));
    h!(0xe40,pop(0x8070));
    h!(0xe42,str_word(1,0,16));
    h!(0xe44,str_word(3,0,28));
    h!(0xe46,str_word(1,0,4));
    w!(0xe48,alu_immediate_flags_w(Alu::Bic,false,1,4,3840));
    h!(0xe4c,logical_low(false,1,2));
    h!(0xe4e,str_word(1,0,12));
    h!(0xe50,movs_immediate(0,0));
    h!(0xe52,pop(0x8070));
    w!(0xe54,stack_registers_w(false,0x41f0));
    h!(0xe58,mov_register(4,1));
    h!(0xe5a,mov_register(7,0));
    h!(0xe5c,mov_register(6,2));
    h!(0xe5e,mov_register(0,2));
    w!(0xe60,bl(0xe60,0xf40));
    h!(0xe64,load_byte(0,4,0));
    h!(0xe66,adds_small(1,4,1));
    h!(0xe68,cmp_immediate(0,97));
    h!(0xe6a,b_cond(Condition::Eq,0xe6a,0xe82));
    h!(0xe6c,cmp_immediate(0,114));
    h!(0xe6e,b_cond(Condition::Eq,0xe6e,0xe76));
    h!(0xe70,cmp_immediate(0,119));
    h!(0xe72,b_cond(Condition::Ne,0xe72,0xeee));
    h!(0xe74,b(0xe74,0xe7c));
    h!(0xe76,movs_immediate(5,1));
    h!(0xe78,movs_immediate(4,0));
    h!(0xe7a,b(0xe7a,0xe88));
    h!(0xe7c,movs_immediate(5,2));
    h!(0xe7e,movs_immediate(4,4));
    h!(0xe80,b(0xe80,0xe88));
    h!(0xe82,movs_immediate(4,8));
    w!(0xe84,movw(5,32770));
    w!(0xe88,memory_postindexed(true,1,0,1,1));
    h!(0xe8c,cmp_immediate(0,43));
    h!(0xe8e,b_cond(Condition::Eq,0xe8e,0xe9e));
    h!(0xe90,cmp_immediate(0,98));
    h!(0xe92,b_cond(Condition::Eq,0xe92,0xea8));
    w!(0xe94,load_byte_negative(0,1,1));
    h!(0xe98,cmp_immediate(0,116));
    h!(0xe9a,b_cond(Condition::Eq,0xe9a,0xeb2));
    h!(0xe9c,b(0xe9c,0xeb6));
    w!(0xe9e,alu_immediate_flags_w(Alu::Orr,false,5,5,3));
    w!(0xea2,alu_immediate_flags_w(Alu::Orr,false,4,4,2));
    h!(0xea6,b(0xea6,0xe88));
    w!(0xea8,alu_immediate_flags_w(Alu::Orr,false,5,5,4));
    w!(0xeac,alu_immediate_flags_w(Alu::Orr,false,4,4,1));
    h!(0xeb0,b(0xeb0,0xe88));
    w!(0xeb2,alu_immediate_flags_w(Alu::Orr,false,4,4,16));
    h!(0xeb6,mov_register(0,7));
    h!(0xeb8,mov_register(1,4));
    w!(0xeba,bl(0xeba,0x2b30));
    h!(0xebe,adds_small(1,0,1));
    h!(0xec0,b_cond(Condition::Eq,0xec0,0xeee));
    h!(0xec2,movs_immediate(1,0));
    h!(0xec4,str_word(1,6,16));
    h!(0xec6,str_word(1,6,4));
    w!(0xec8,mov_immediate_w(1,512));
    h!(0xecc,str_word(5,6,12));
    h!(0xece,str_word(1,6,28));
    h!(0xed0,str_word(0,6,20));
    h!(0xed2,lsls_immediate(0,4,28));
    h!(0xed4,b_cond(Condition::Pl,0xed4,0xee0));
    h!(0xed6,mov_register(0,6));
    h!(0xed8,movs_immediate(2,2));
    h!(0xeda,movs_immediate(1,0));
    w!(0xedc,bl(0xedc,0x10dc));
    h!(0xee0,ldr_word(0,6,80));
    w!(0xee2,alu_immediate_flags_w(Alu::Orr,false,0,0,1));
    h!(0xee6,str_word(0,6,80));
    h!(0xee8,mov_register(0,6));
    w!(0xeea,stack_registers_w(true,0x81f0));
    h!(0xeee,movs_immediate(0,0));
    h!(0xef0,b(0xef0,0xeea));
    w!(0xef2,stack_registers_w(false,0x41f0));
    h!(0xef6,mov_register(5,1));
    h!(0xef8,mov_register(6,0));
    h!(0xefa,ldr_literal(4,0xf3c-0xefc));
    h!(0xefc,ldr_word(0,4,80));
    h!(0xefe,lsls_immediate(1,0,31));
    h!(0xf00,b_cond(Condition::Eq,0xf00,0xf0c));
    h!(0xf02,lsrs_immediate(1,0,1));
    h!(0xf04,b_cond(Condition::Eq,0xf04,0xf10));
    w!(0xf06,alu_immediate_flags_w(Alu::Bic,false,4,0,1));
    h!(0xf0a,b(0xf0a,0xefc));
    h!(0xf0c,mov_register(2,4));
    h!(0xf0e,b(0xf0e,0xf2e));
    h!(0xf10,movs_immediate(0,84));
    w!(0xf12,bl(0xf12,0x107e));
    h!(0xf16,movs_register(7,0));
    h!(0xf18,b_cond(Condition::Eq,0xf18,0xf38));
    h!(0xf1a,ldr_word(0,4,80));
    w!(0xf1c,alu_immediate_flags_w(Alu::Orr,false,1,7,1));
    h!(0xf20,logical_low(false,0,1));
    h!(0xf22,movs_immediate(1,84));
    h!(0xf24,str_word(0,4,80));
    h!(0xf26,mov_register(0,7));
    w!(0xf28,bl(0xf28,0x87a));
    h!(0xf2c,mov_register(2,7));
    h!(0xf2e,mov_register(1,5));
    h!(0xf30,mov_register(0,6));
    w!(0xf32,stack_registers_w(true,0x41f0));
    h!(0xf36,b(0xf36,0xe54));
    h!(0xf38,movs_immediate(0,0));
    h!(0xf3a,b(0xf3a,0xeea));
    assert_eq!(0xc38+out.len(),0xf3c);
    out.extend_from_slice(&0x42228u32.to_le_bytes());
    w!(0xf40,stack_registers_w(false,0x47f0));
    h!(0xf44,mov_register(4,0));
    w!(0xf46,mov_immediate_w(9,0));
    w!(0xf4a,mov_immediate_w(7,4294967295));
    h!(0xf4e,ldr_word(5,0,12));
    w!(0xf50,load_pair(8,6,4,16));
    h!(0xf54,lsls_immediate(0,5,30));
    h!(0xf56,b_cond(Condition::Eq,0xf56,0xf68));
    h!(0xf58,mov_register(0,4));
    w!(0xf5a,bl(0xf5a,0x123c));
    h!(0xf5e,mov_register(0,6));
    w!(0xf60,bl(0xf60,0x2b20));
    h!(0xf64,cmp_immediate(0,0));
    h!(0xf66,b_cond(Condition::Ge,0xf66,0xf6e));
    h!(0xf68,mov_register(0,7));
    w!(0xf6a,stack_registers_w(true,0x87f0));
    h!(0xf6e,lsls_immediate(0,5,20));
    h!(0xf70,b_cond(Condition::Pl,0xf70,0xf78));
    h!(0xf72,mov_register(0,8));
    w!(0xf74,bl(0xf74,0xbea));
    h!(0xf78,movs_immediate(1,76));
    h!(0xf7a,mov_register(0,4));
    w!(0xf7c,bl(0xf7c,0x87a));
    h!(0xf80,ldr_word(0,4,80));
    w!(0xf82,alu_immediate_flags_w(Alu::Bic,false,0,0,1));
    h!(0xf86,str_word(0,4,80));
    h!(0xf88,mov_register(0,9));
    h!(0xf8a,b(0xf8a,0xf6a));
    h!(0xf8c,push(0x4010));
    h!(0xf8e,mov_register(4,0));
    w!(0xf90,nop_w());
    h!(0xf94,mov_register(0,4));
    w!(0xf96,stack_registers_w(true,0x4010));
    w!(0xf9a,b_w(0xf9a,0x4a8));
    h!(0xf9e,push(0x4070));
    h!(0xfa0,mov_register(5,0));
    h!(0xfa2,mov_register(4,1));
    h!(0xfa4,movs_immediate(0,10));
    h!(0xfa6,b(0xfa6,0xfaa));
    h!(0xfa8,adds_small(5,5,1));
    w!(0xfaa,bl(0xfaa,0x2b3e));
    h!(0xfae,cbz(5,0xfae,0xfbe));
    h!(0xfb0,load_byte(0,5,0));
    h!(0xfb2,cmp_immediate(0,0));
    h!(0xfb4,b_cond(Condition::Ne,0xfb4,0xfa8));
    h!(0xfb6,b(0xfb6,0xfbe));
    h!(0xfb8,adds_small(4,4,1));
    w!(0xfba,bl(0xfba,0x2b3e));
    h!(0xfbe,cbz(4,0xfbe,0xfc6));
    h!(0xfc0,load_byte(0,4,0));
    h!(0xfc2,cmp_immediate(0,0));
    h!(0xfc4,b_cond(Condition::Ne,0xfc4,0xfb8));
    w!(0xfc6,stack_registers_w(true,0x4070));
    h!(0xfca,movs_immediate(0,10));
    w!(0xfcc,b_w(0xfcc,0x2b3e));
    h!(0xfd0,push(0x4010));
    w!(0xfd2,bl(0xfd2,0x12c4));
    w!(0xfd6,stack_registers_w(true,0x4010));
    w!(0xfda,b_w(0xfda,0xb64));
    assert_eq!(0xc38+out.len(),0xfde);
    out.resize(out.len()+2,0);
    h!(0xfe0,ldr_literal(0,0xfe4-0xfe4));
    h!(0xfe2,bx(14));
    assert_eq!(0xc38+out.len(),0xfe4);
    out.extend_from_slice(&0x42324u32.to_le_bytes());
    h!(0xfe8,ldr_literal(0,0xfec-0xfec));
    h!(0xfea,bx(14));
    assert_eq!(0xc38+out.len(),0xfec);
    out.extend_from_slice(&0x4232cu32.to_le_bytes());
    h!(0xff0,bx(14));
    h!(0xff2,bx(14));
    h!(0xff4,bx(14));
    h!(0xff6,bx(14));
    assert_eq!(out.len(),960);
    out
}

fn runtime_arena_and_stream_position()->Vec<u8> {
    let mut out=Vec::with_capacity(768);
    macro_rules! h { ($at:expr,$word:expr) => {{ assert_eq!(0xff8+out.len(),$at); push16(&mut out,$word); }}; }
    macro_rules! w { ($at:expr,$words:expr) => {{ assert_eq!(0xff8+out.len(),$at); push32(&mut out,$words); }}; }
    h!(0xff8,push(0x4038));
    h!(0xffa,mov_register(4,0));
    h!(0xffc,mov_register(2,1));
    h!(0xffe,movs_immediate(0,0));
    h!(0x1000,mov_register(1,13));
    w!(0x1002,bl(0x1002,0x13f2));
    h!(0x1006,cmp_immediate(0,0));
    h!(0x1008,it(Condition::Eq,ItBlock::One));
    h!(0x100a,pop(0x8038));
    h!(0x100c,mov_register(2,0));
    h!(0x100e,stack_load(1,0));
    h!(0x1010,mov_register(0,4));
    w!(0x1012,bl(0x1012,0x1306));
    h!(0x1016,movs_immediate(0,1));
    h!(0x1018,pop(0x8038));
    h!(0x101a,movs_immediate(0,1));
    w!(0x101c,b_w(0x101c,0x13cc));
    h!(0x1020,push(0x40f8));
    w!(0x1022,bl(0x1022,0x13dc));
    h!(0x1026,mov_register(5,0));
    h!(0x1028,movs_immediate(0,0));
    h!(0x102a,mov_register(4,1));
    h!(0x102c,movs_immediate(0,16));
    h!(0x102e,nop());
    h!(0x1030,mov_register(6,0));
    h!(0x1032,adds_register(0,5,6));
    h!(0x1034,cmp_register_low(0,4));
    h!(0x1036,b_cond(Condition::Ls,0x1036,0x1054));
    h!(0x1038,mov_register(2,6));
    h!(0x103a,mov_register(1,13));
    h!(0x103c,movs_immediate(0,0));
    w!(0x103e,bl(0x103e,0x13f2));
    h!(0x1042,movs_register(7,0));
    h!(0x1044,it(Condition::Eq,ItBlock::One));
    w!(0x1046,bl(0x1046,0x13cc));
    h!(0x104a,stack_load(0,0));
    h!(0x104c,cmp_register_low(0,4));
    h!(0x104e,it(Condition::Ne,ItBlock::One));
    h!(0x1050,mov_register(5,0));
    h!(0x1052,adds_register(4,0,7));
    w!(0x1054,bl(0x1054,0xfe8));
    h!(0x1058,str_word(5,0,0));
    h!(0x105a,mov_register(7,0));
    h!(0x105c,adds_small(0,6,7));
    w!(0x105e,alu_immediate_flags_w(Alu::Bic,false,0,0,7));
    h!(0x1062,adds_register(6,0,5));
    h!(0x1064,mov_register(0,5));
    w!(0x1066,bl(0x1066,0x12f8));
    h!(0x106a,cmp_register_low(4,6));
    h!(0x106c,it(Condition::Eq,ItBlock::One));
    h!(0x106e,pop(0x80f8));
    h!(0x1070,sub_register(2,4,6));
    h!(0x1072,mov_register(1,6));
    h!(0x1074,ldr_word(0,7,0));
    w!(0x1076,stack_registers_w(true,0x40f8));
    w!(0x107a,b_w(0x107a,0x1306));
    h!(0x107e,push(0x4070));
    h!(0x1080,mov_register(6,0));
    w!(0x1082,bl(0x1082,0xfe8));
    h!(0x1086,ldr_word(5,0,0));
    w!(0x1088,add_immediate_w(0,6,11));
    w!(0x108c,alu_immediate_flags_w(Alu::Bic,false,4,0,7));
    h!(0x1090,cmp_register_low(4,6));
    h!(0x1092,it(Condition::Ls,ItBlock::Then));
    h!(0x1094,movs_immediate(0,0));
    h!(0x1096,pop(0x8070));
    h!(0x1098,mov_register(3,5));
    h!(0x109a,ldr_word(2,5,4));
    h!(0x109c,cbz(2,0x109c,0x10ce));
    h!(0x109e,ldr_word(0,2,0));
    h!(0x10a0,cmp_register_low(0,4));
    h!(0x10a2,b_cond(Condition::Lo,0x10a2,0x10c6));
    w!(0x10a4,add_immediate_w(1,4,8));
    h!(0x10a8,cmp_register_low(0,1));
    h!(0x10aa,it(Condition::Lo,ItBlock::Then));
    h!(0x10ac,ldr_word(0,2,4));
    h!(0x10ae,str_word(0,3,4));
    h!(0x10b0,b_cond(Condition::Lo,0x10b0,0x10c2));
    h!(0x10b2,ldr_word(1,2,4));
    h!(0x10b4,adds_register(0,2,4));
    h!(0x10b6,str_word(1,0,4));
    h!(0x10b8,ldr_word(1,2,0));
    h!(0x10ba,sub_register(1,1,4));
    h!(0x10bc,str_word(1,0,0));
    h!(0x10be,str_word(0,3,4));
    h!(0x10c0,str_word(4,2,0));
    h!(0x10c2,adds_small(0,2,4));
    h!(0x10c4,pop(0x8070));
    h!(0x10c6,mov_register(3,2));
    h!(0x10c8,ldr_word(2,2,4));
    h!(0x10ca,cmp_immediate(2,0));
    h!(0x10cc,b_cond(Condition::Ne,0x10cc,0x109e));
    h!(0x10ce,mov_register(1,4));
    h!(0x10d0,mov_register(0,5));
    w!(0x10d2,bl(0x10d2,0xff8));
    h!(0x10d6,cmp_immediate(0,0));
    h!(0x10d8,b_cond(Condition::Ne,0x10d8,0x1098));
    h!(0x10da,pop(0x8070));
    w!(0x10dc,stack_registers_w(false,0x41f0));
    h!(0x10e0,mov_register(4,0));
    h!(0x10e2,mov_register(6,2));
    h!(0x10e4,mov_register(5,1));
    h!(0x10e6,ldr_word(7,0,20));
    h!(0x10e8,load_byte(0,0,12));
    h!(0x10ea,lsls_immediate(0,0,30));
    h!(0x10ec,b_cond(Condition::Eq,0x10ec,0x114c));
    h!(0x10ee,mov_register(0,7));
    w!(0x10f0,bl(0x10f0,0x2b2c));
    h!(0x10f4,cbnz(0,0x10f4,0x114c));
    h!(0x10f6,cbz(6,0x10f6,0x1148));
    h!(0x10f8,cmp_immediate(6,1));
    h!(0x10fa,b_cond(Condition::Eq,0x10fa,0x1102));
    h!(0x10fc,cmp_immediate(6,2));
    h!(0x10fe,b_cond(Condition::Ne,0x10fe,0x114c));
    h!(0x1100,b(0x1100,0x110c));
    h!(0x1102,mov_register(0,4));
    w!(0x1104,bl(0x1104,0x133a));
    h!(0x1108,add_register(5,0));
    h!(0x110a,b(0x110a,0x1148));
    h!(0x110c,mov_register(0,7));
    w!(0x110e,bl(0x110e,0x2b26));
    h!(0x1112,ldr_word(1,4,12));
    h!(0x1114,cmp_immediate(0,0));
    w!(0x1116,alu_immediate_flags_w(Alu::Orr,false,1,1,16));
    h!(0x111a,str_word(1,4,12));
    h!(0x111c,b_cond(Condition::Ge,0x111c,0x112a));
    h!(0x111e,mov_register(0,4));
    w!(0x1120,bl(0x1120,0x11d4));
    h!(0x1124,movs_immediate(0,1));
    w!(0x1126,stack_registers_w(true,0x81f0));
    h!(0x112a,lsls_immediate(1,1,15));
    h!(0x112c,b_cond(Condition::Pl,0x112c,0x1146));
    h!(0x112e,ldr_word(2,4,4));
    h!(0x1130,ldr_word(1,4,44));
    h!(0x1132,cmp_register_low(1,2));
    h!(0x1134,b_cond(Condition::Hi,0x1134,0x1138));
    h!(0x1136,mov_register(1,2));
    h!(0x1138,ldr_word(2,4,24));
    h!(0x113a,add_register(1,2));
    h!(0x113c,ldr_word(2,4,16));
    h!(0x113e,sub_register(1,1,2));
    h!(0x1140,cmp_register_low(1,0));
    h!(0x1142,b_cond(Condition::Le,0x1142,0x1146));
    h!(0x1144,mov_register(0,1));
    h!(0x1146,add_register(5,0));
    h!(0x1148,cmp_immediate(5,0));
    h!(0x114a,b_cond(Condition::Ge,0x114a,0x1150));
    h!(0x114c,movs_immediate(0,2));
    h!(0x114e,b(0x114e,0x1126));
    h!(0x1150,ldr_word(1,4,44));
    h!(0x1152,ldr_word(0,4,4));
    h!(0x1154,ldr_word(2,4,12));
    h!(0x1156,cmp_register_low(1,0));
    h!(0x1158,b_cond(Condition::Hs,0x1158,0x1168));
    h!(0x115a,lsls_immediate(1,2,14));
    h!(0x115c,str_word(0,4,44));
    h!(0x115e,b_cond(Condition::Pl,0x115e,0x1168));
    w!(0x1160,alu_immediate_flags_w(Alu::Bic,false,1,2,131072));
    w!(0x1164,alu_immediate_flags_w(Alu::Orr,false,2,1,16));
    h!(0x1168,ldr_word(6,4,24));
    h!(0x116a,movs_immediate(7,0));
    h!(0x116c,cmp_register_low(6,5));
    h!(0x116e,b_cond(Condition::Gt,0x116e,0x1198));
    h!(0x1170,ldr_word(1,4,44));
    h!(0x1172,cmp_register_low(1,0));
    h!(0x1174,b_cond(Condition::Ls,0x1174,0x117a));
    h!(0x1176,mov_register(3,1));
    h!(0x1178,b(0x1178,0x117c));
    h!(0x117a,mov_register(3,0));
    w!(0x117c,alu_w(Alu::Add,false,12,3,6,Shift::Lsl,0));
    h!(0x1180,ldr_word(3,4,16));
    w!(0x1182,alu_w(Alu::Sub,false,12,12,3,Shift::Lsl,0));
    h!(0x1186,cmp_register(12,5));
    h!(0x1188,b_cond(Condition::Le,0x1188,0x1198));
    h!(0x118a,cmp_register_low(1,0));
    h!(0x118c,b_cond(Condition::Ls,0x118c,0x1192));
    h!(0x118e,mov_register(12,1));
    h!(0x1190,b(0x1190,0x1194));
    h!(0x1192,mov_register(12,0));
    h!(0x1194,cmp_register(12,3));
    h!(0x1196,b_cond(Condition::Ne,0x1196,0x11a4));
    h!(0x1198,str_word(7,4,8));
    h!(0x119a,str_word(7,4,0));
    w!(0x119c,alu_immediate_flags_w(Alu::Orr,false,0,2,32));
    h!(0x11a0,str_word(5,4,40));
    h!(0x11a2,b(0x11a2,0x11c0));
    h!(0x11a4,sub_register(5,5,6));
    h!(0x11a6,ldr_word(6,4,28));
    h!(0x11a8,sub_register(6,5,6));
    h!(0x11aa,cmp_register_low(1,0));
    h!(0x11ac,str_word(6,4,8));
    h!(0x11ae,b_cond(Condition::Ls,0x11ae,0x11b2));
    h!(0x11b0,mov_register(0,1));
    h!(0x11b2,sub_register(0,0,3));
    h!(0x11b4,sub_register(0,5,0));
    h!(0x11b6,str_word(0,4,0));
    h!(0x11b8,adds_register(0,3,5));
    h!(0x11ba,str_word(0,4,4));
    w!(0x11bc,alu_immediate_flags_w(Alu::Bic,false,0,2,32));
    h!(0x11c0,ldr_literal(1,0x11d0-0x11c4));
    h!(0x11c2,ands(0,1));
    h!(0x11c4,str_word(0,4,12));
    h!(0x11c6,movs_immediate(0,0));
    w!(0x11c8,store_byte_w(7,4,72));
    h!(0x11cc,b(0x11cc,0x1126));
    assert_eq!(0xff8+out.len(),0x11ce);
    out.resize(out.len()+2,0);
    out.extend_from_slice(&0xffd7cfbfu32.to_le_bytes());
    h!(0x11d4,ldr_word(1,0,12));
    w!(0x11d6,alu_immediate_flags_w(Alu::Bic,false,1,1,2097152));
    w!(0x11da,alu_immediate_flags_w(Alu::Orr,false,1,1,128));
    h!(0x11de,str_word(1,0,12));
    h!(0x11e0,movs_immediate(1,0));
    h!(0x11e2,str_word(1,0,8));
    h!(0x11e4,str_word(1,0,0));
    h!(0x11e6,bx(14));
    w!(0x11e8,stack_registers_w(false,0x47f0));
    h!(0x11ec,mov_register(9,0));
    h!(0x11ee,mov_register(4,2));
    h!(0x11f0,mov_register(6,1));
    w!(0x11f2,mov_immediate_w(8,4294967295));
    h!(0x11f6,ldr_literal(0,0x12c0-0x11f8));
    h!(0x11f8,ldr_word(5,2,12));
    h!(0x11fa,ldr_word(7,2,20));
    h!(0x11fc,tst_register_low(5,0));
    h!(0x11fe,b_cond(Condition::Eq,0x11fe,0x1214));
    h!(0x1200,mov_register(0,7));
    h!(0x1202,ldr_word(1,4,24));
    w!(0x1204,bl(0x1204,0x2b34));
    h!(0x1208,cmp_immediate(0,0));
    h!(0x120a,b_cond(Condition::Lt,0x120a,0x1230));
    h!(0x120c,ldr_literal(0,0x12c0-0x1210));
    h!(0x120e,mvns(0,0));
    h!(0x1210,ands(5,0));
    h!(0x1212,str_word(5,4,12));
    h!(0x1214,mov_register(3,5));
    h!(0x1216,mov_register(2,6));
    h!(0x1218,mov_register(1,9));
    h!(0x121a,mov_register(0,7));
    w!(0x121c,bl(0x121c,0x2b3a));
    h!(0x1220,ldr_word(1,4,24));
    w!(0x1222,alu_immediate_flags_w(Alu::Bic,false,2,0,2147483648));
    h!(0x1226,sub_register(2,6,2));
    h!(0x1228,add_register(1,2));
    h!(0x122a,cmp_immediate(0,0));
    h!(0x122c,str_word(1,4,24));
    h!(0x122e,b_cond(Condition::Eq,0x122e,0x1238));
    h!(0x1230,mov_register(0,4));
    w!(0x1232,bl(0x1232,0x11d4));
    h!(0x1236,mov_register(0,8));
    w!(0x1238,stack_registers_w(true,0x87f0));
    h!(0x123c,push(0x4070));
    h!(0x123e,mov_register(4,0));
    h!(0x1240,ldr_word(5,0,16));
    h!(0x1242,ldr_word(0,0,44));
    h!(0x1244,ldr_word(1,4,4));
    h!(0x1246,cmp_register_low(0,1));
    h!(0x1248,b_cond(Condition::Hi,0x1248,0x124c));
    h!(0x124a,mov_register(0,1));
    h!(0x124c,ldr_word(1,4,12));
    w!(0x124e,alu_immediate_flags_w(Alu::Bic,false,1,1,2621440));
    h!(0x1252,str_word(1,4,12));
    h!(0x1254,lsls_immediate(1,1,15));
    h!(0x1256,b_cond(Condition::Pl,0x1256,0x127e));
    h!(0x1258,cmp_register_low(0,5));
    h!(0x125a,b_cond(Condition::Eq,0x125a,0x126e));
    h!(0x125c,mov_register(2,4));
    h!(0x125e,sub_register(1,0,5));
    h!(0x1260,mov_register(0,5));
    w!(0x1262,bl(0x1262,0x11e8));
    h!(0x1266,cbz(0,0x1266,0x126e));
    w!(0x1268,mov_immediate_w(0,4294967295));
    h!(0x126c,pop(0x8070));
    h!(0x126e,movs_immediate(0,0));
    h!(0x1270,str_word(5,4,44));
    w!(0x1272,store_pair(5,0,4,4));
    h!(0x1276,ldr_word(0,4,12));
    w!(0x1278,alu_immediate_flags_w(Alu::Bic,false,0,0,65536));
    h!(0x127c,str_word(0,4,12));
    h!(0x127e,movs_immediate(0,0));
    h!(0x1280,pop(0x8070));
    h!(0x1282,push(0x4010));
    h!(0x1284,mov_register(4,0));
    h!(0x1286,ldr_word(0,0,12));
    w!(0x1288,alu_immediate_flags_w(Alu::Bic,false,0,0,32));
    h!(0x128c,str_word(0,4,12));
    h!(0x128e,ldr_word(1,4,40));
    h!(0x1290,ldr_word(0,4,24));
    h!(0x1292,cmp_register_low(0,1));
    h!(0x1294,b_cond(Condition::Eq,0x1294,0x12b2));
    h!(0x1296,mov_register(0,4));
    w!(0x1298,bl(0x1298,0x123c));
    h!(0x129c,ldr_word(0,4,12));
    w!(0x129e,alu_immediate_flags_w(Alu::Bic,false,0,0,12288));
    w!(0x12a2,alu_immediate_flags_w(Alu::Orr,false,0,0,16));
    h!(0x12a6,str_word(0,4,12));
    h!(0x12a8,ldr_word(0,4,40));
    h!(0x12aa,str_word(0,4,24));
    h!(0x12ac,ldr_word(0,4,16));
    h!(0x12ae,str_word(0,4,44));
    h!(0x12b0,str_word(0,4,4));
    w!(0x12b2,movw(1,16448));
    h!(0x12b6,ldr_word(0,4,12));
    h!(0x12b8,bics(0,1));
    h!(0x12ba,str_word(0,4,12));
    h!(0x12bc,pop(0x8010));
    assert_eq!(0xff8+out.len(),0x12be);
    out.resize(out.len()+2,0);
    out.extend_from_slice(&0x20010u32.to_le_bytes());
    h!(0x12c4,push(0x4010));
    h!(0x12c6,mov_register(1,0));
    h!(0x12c8,adr(0,0x12c8,0x12d4));
    w!(0x12ca,bl(0x12ca,0xf9e));
    h!(0x12ce,movs_immediate(0,1));
    h!(0x12d0,pop(0x8010));
    assert_eq!(0xff8+out.len(),0x12d2);
    out.resize(out.len()+2,0);
    out.extend_from_slice(b"SIGRTRED: Redirect: can't open: ");
    out.resize(768,0);
    assert_eq!(0xff8+out.len(),0x12f8);
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
                data.extend_from_slice(&runtime_memory_and_formatter());
                assert_eq!(offset+data.len(),0x79e);
                data.extend_from_slice(&runtime_streams_and_arena());
                assert_eq!(offset+data.len(),0x9d8);
                data.extend_from_slice(&runtime_stream_io());
                assert_eq!(offset+data.len(),0xd98);
                data.extend_from_slice(&runtime_arena_and_stream_position());
                assert_eq!(offset+data.len(),0x1098);
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
        assert_eq!(spans()[2].data.len(),3848);
        assert_eq!(MODELLED_BYTES,4312);
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
        assert_eq!(spans()[2].offset+spans()[2].data.len(),0x1098);
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
        assert_eq!(plan.iter().map(|span|span.data.len()).sum::<usize>(),684+118+486+726+570+960+768);
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
        assert_eq!(&span.data[0x2e2-span.offset..0x4c8-span.offset],extension);
        assert_eq!(span.offset+span.data.len(),0x1098);
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
    fn runtime_memory_formatter_structure_and_operands() {
        let extension=runtime_memory_and_formatter();
        let at=|start:usize,end:usize| &extension[start-0x728..end-0x728];
        let code=[(0x728,0x73e),(0x73e,0x77c),(0x77c,0x86a),
                  (0x86a,0x87a),(0x87a,0x8c8),(0x8c8,0x8ca),
                  (0x8ca,0x97c),(0x97c,0x99c),(0x9a0,0x9ae),
                  (0x9ae,0x9f2),(0x9f4,0x9fe)];
        assert_eq!(code.iter().map(|(start,end)|end-start).sum::<usize>(),720);
        let mut occupied=vec![false;726];
        for (start,end) in code.into_iter().chain([(0x99c,0x9a0),(0x9f2,0x9f4)]) {
            assert_eq!(start%2,0);
            assert_eq!(end%2,0);
            for byte in &mut occupied[start-0x728..end-0x728] {
                assert!(!*byte);
                *byte=true;
            }
        }
        assert!(occupied.into_iter().all(|byte|byte));
        assert_eq!(extension.len(),720+4+2);
        assert_eq!(at(0x99c,0x9a0),0x2213u32.to_le_bytes());
        assert_eq!(0x982+4+u32::from_le_bytes(at(0x99c,0x9a0).try_into().unwrap()),0x2b99);
        assert_eq!(at(0x9f2,0x9f4),[0,0]);
        for address in [0x7de,0x90a,0x930,0x968] {
            assert_eq!(at(address,address+4),[0xaf,0xf3,0,0x80]);
        }
        assert_eq!(at(0x8c8,0x8ca),bx(14).to_le_bytes());
        assert_eq!(modified_immediate(0x01010101),0x301);
        assert_eq!(modified_immediate(u32::MAX),0x3ff);
        assert_eq!(modified_immediate(0x40000000),0x480);
        assert_eq!(tst_immediate_w(1,0x40000000),[0xf011,0x4f80]);
        assert_eq!(alu_immediate_flags_w(Alu::Sub,true,1,1,32),[0xf1b1,0x0120]);
        assert_eq!(stm_w(0,0x5018),[0xe8a0,0x5018]);
        assert_eq!(stm_w(0,0x500c),[0xe8a0,0x500c]);
        assert_eq!(load_byte_register_w(0,10,6),[0xf81a,6]);
        assert_eq!(memory_postindexed(true,2,3,1,2),[0xf831,0x3b02]);
        assert_eq!(memory_postindexed(false,1,2,0,1),[0xf800,0x2b01]);
        let plan=spans();
        let span=&plan[2];
        assert_eq!(&span.data[0x4c8-span.offset..0x79e-span.offset],extension);
        assert_eq!(plan.iter().map(|span|span.data.len()).sum::<usize>(),4312);
        for encode in [
            || { modified_immediate(0x12345678); },
            || { alu_immediate_flags_w(Alu::Sub,true,13,1,32); },
            || { cmp_immediate_w(13,2); },
            || { lsls_immediate_w(12,2,32); },
            || { memory_postindexed(true,3,0,1,1); },
            || { memory_postindexed(true,1,1,1,1); },
            || { memory_postindexed(false,4,15,0,4); },
            || { memory_postindexed(true,1,13,0,1); },
            || { memory_postindexed(false,1,13,0,1); },
            || { memory_postindexed(true,2,13,0,2); },
            || { memory_postindexed(false,2,13,0,2); },
            || { stm_w(0,0x500d); },
            || { stm_w(0,0xa00c); },
            || { stm_w(15,0x500c); },
            || { load_byte(0,0,32); },
            || { load_byte_register_w(0,15,0); },
            || { load_byte_register_w(0,0,13); },
            || { subs_small(0,0,8); },
            || { adds_register(0,8,0); },
            || { bics(8,0); },
            || { add_register(0,16); },
        ] {
            assert!(std::panic::catch_unwind(encode).is_err());
        }
    }

    #[test]
    fn runtime_streams_arena_structure_and_operands() {
        let extension=runtime_streams_and_arena();
        let at=|start:usize,end:usize| &extension[start-0x9fe..end-0x9fe];
        let code=[(0x9fe,0xa1e),(0xa24,0xa2c),(0xa2c,0xafe),(0xafe,0xb36),
                  (0xb64,0xb6e),(0xb70,0xb7e),(0xba0,0xbea),(0xbea,0xc38)];
        let data=[(0xa1e,0xa20),(0xa20,0xa24),(0xb36,0xb38),(0xb38,0xb64),
                  (0xb6e,0xb70),(0xb7e,0xb80),(0xb80,0xb9d),(0xb9d,0xba0)];
        assert_eq!(code.iter().map(|(start,end)|end-start).sum::<usize>(),482);
        assert_eq!(data.iter().map(|(start,end)|end-start).sum::<usize>(),88);
        let mut occupied=vec![false;570];
        for (start,end) in code.into_iter().chain(data) {
            for byte in &mut occupied[start-0x9fe..end-0x9fe] {
                assert!(!*byte);
                *byte=true;
            }
        }
        assert!(occupied.into_iter().all(|byte|byte));
        assert_eq!(extension.len(),570);
        for (start,end) in [(0xa1e,0xa20),(0xb36,0xb38),(0xb6e,0xb70),
                            (0xb7e,0xb80),(0xb9d,0xba0)] {
            assert!(at(start,end).iter().all(|byte|*byte==0));
        }
        assert_eq!(at(0xa20,0xa24),0xffffffe5u32.to_le_bytes());
        assert_eq!((0xa0cu32+4).wrapping_add(u32::from_le_bytes(at(0xa20,0xa24).try_into().unwrap())),0x9f5);
        for (index,value) in [0x42228u32,0x4227c,0x422d0,0x402a4,0x402a8,0x402ac,
                              0x72,0x2dae,0x77,0x2d9a,0x2d86].into_iter().enumerate() {
            assert_eq!(at(0xb38+index*4,0xb3c+index*4),value.to_le_bytes());
        }
        assert_eq!(at(0xb80,0xb9d),b"SIGABRT: Abnormal termination");
        assert_eq!(at(0xba8,0xbaa),[5,0]);
        assert_eq!(at(0xbb2,0xbb6),[0x85,0x46,0x18,0xb0]);
        assert_eq!(stack_adjust(true,60),0xb08f);
        assert_eq!(stack_adjust(false,508),0xb07f);
        assert_eq!(stack_store(1,20),0x9105);
        assert_eq!(store_pair(3,1,13,4),[0xe9cd,0x3101]);
        assert_eq!(cbnz(0,0xa70,0xa7c),0xb920);
        assert_eq!(movs_register(5,0),5);
        let plan=spans();
        let span=&plan[2];
        assert_eq!(&span.data[0x79e-span.offset..0x9d8-span.offset],extension);
        assert_eq!(span.offset+span.data.len(),0x1098);
        assert_eq!(plan.iter().map(|span|span.data.len()).sum::<usize>(),4312);
        for encode in [
            || { stack_adjust(false,510); },
            || { stack_adjust(true,512); },
            || { stack_store(8,0); },
            || { stack_store(0,2); },
            || { stack_store(0,1024); },
            || { store_pair(13,0,1,0); },
            || { store_pair(0,13,1,0); },
            || { store_pair(0,1,15,0); },
            || { store_pair(0,1,13,2); },
            || { store_pair(0,1,13,1024); },
            || { cbnz(8,0,4); },
            || { cbnz(0,1,4); },
            || { cbnz(0,0,5); },
            || { cbnz(0,0,132); },
            || { cbnz(0,8,4); },
            || { movs_register(8,0); },
            || { movs_register(0,8); },
        ] {
            assert!(std::panic::catch_unwind(encode).is_err());
        }
    }

    #[test]
    fn runtime_stream_io_structure_and_operands() {
        let extension=runtime_stream_io();
        let at=|start:usize,end:usize| &extension[start-0xc38..end-0xc38];
        let code=[(0xc38,0xe0e),(0xe0e,0xe54),(0xe54,0xef2),(0xef2,0xf3c),
                  (0xf40,0xf8c),(0xf8c,0xf9e),(0xf9e,0xfd0),(0xfd0,0xfde),
                  (0xfe0,0xfe4),(0xfe8,0xfec),(0xff0,0xff2),(0xff2,0xff4),
                  (0xff4,0xff6),(0xff6,0xff8)];
        let data=[(0xf3c,0xf40),(0xfe4,0xfe8),(0xfec,0xff0)];
        let alignment=[(0xfde,0xfe0)];
        assert_eq!(code.iter().map(|(start,end)|end-start).sum::<usize>(),946);
        assert_eq!(data.iter().map(|(start,end)|end-start).sum::<usize>(),12);
        assert_eq!(alignment.iter().map(|(start,end)|end-start).sum::<usize>(),2);
        let mut occupied=vec![false;960];
        for (start,end) in code.into_iter().chain(data).chain(alignment) {
            assert!(start>=0xc38 && start<end && end<=0xff8);
            for byte in &mut occupied[start-0xc38..end-0xc38] {
                assert!(!*byte);
                *byte=true;
            }
        }
        assert!(occupied.into_iter().all(|byte|byte));
        assert_eq!(extension.len(),960);
        for (address,value) in [(0xf3c,0x42228u32),(0xfe4,0x42324),(0xfec,0x4232c)] {
            assert_eq!(at(address,address+4),value.to_le_bytes());
        }
        assert_eq!(at(0xfde,0xfe0),[0,0]);
        assert_eq!(at(0xf90,0xf94),[0xaf,0xf3,0,0x80]);
        assert_eq!(at(0xe94,0xe98),[0x11,0xf8,1,0x0c]);
        assert_ne!(load_byte_negative(0,1,1),memory_postindexed(true,1,0,1,1));
        assert_eq!(at(0xd72,0xd76),[0xbd,0xe8,0xf0,0x87]);
        assert_eq!(at(0xf32,0xf36),[0xbd,0xe8,0xf0,0x41]);
        assert_eq!(at(0xf3a,0xf3c),b(0xf3a,0xeea).to_le_bytes());
        assert_eq!(at(0xef0,0xef2),b(0xef0,0xeea).to_le_bytes());
        for address in [0xff0,0xff2,0xff4,0xff6] {
            assert_eq!(at(address,address+2),bx(14).to_le_bytes());
        }
        assert_eq!(movw(1,4226),[0xf241,0x0182]);
        assert_eq!(movw(5,32770),[0xf248,0x0502]);
        assert_eq!(movw(12,65535),[0xf64f,0x7cff]);
        assert_eq!(mvn_immediate_w(6,0xff000000),[0xf06f,0x467f]);
        assert_eq!(cmp_immediate_w(1,32768),[0xf5b1,0x4f00]);
        assert_eq!(cmp_immediate_w(2,256),[0xf5b2,0x7f80]);
        assert_eq!(ands(1,0),0x4001);
        assert_eq!(mvns(0,0),0x43c0);
        assert_eq!(logical_low(false,1,2),0x4311);
        assert_eq!(lsrs_immediate(1,0,1),0x0841);
        assert_eq!(lsrs_immediate(1,0,32),0x0801);
        assert_eq!(uxtb(6,5),0xb2ee);
        assert_eq!(load_halfword(0,4,12),0x89a0);
        let plan=spans();
        let span=&plan[2];
        assert_eq!(&span.data[0x9d8-span.offset..0xd98-span.offset],extension);
        assert_eq!(span.offset+span.data.len(),0x1098);
        assert_eq!(plan.iter().map(|span|span.data.len()).sum::<usize>(),4312);
        for encode in [
            || { movw(13,0); },
            || { mvn_immediate_w(13,1); },
            || { mvn_immediate_w(0,0x12345678); },
            || { cmp_immediate_w(0,0x12345678); },
            || { ands(8,0); },
            || { ands(0,8); },
            || { mvns(8,0); },
            || { mvns(0,8); },
            || { logical_low(false,8,0); },
            || { logical_low(false,0,8); },
            || { lsrs_immediate(8,0,1); },
            || { lsrs_immediate(0,8,1); },
            || { lsrs_immediate(0,0,0); },
            || { lsrs_immediate(0,0,33); },
            || { uxtb(8,0); },
            || { uxtb(0,8); },
            || { load_halfword(8,0,0); },
            || { load_halfword(0,8,0); },
            || { load_halfword(0,0,1); },
            || { load_halfword(0,0,64); },
            || { store_byte(8,0,0); },
            || { store_byte(0,8,0); },
            || { store_byte(0,0,32); },
            || { load_byte_negative(13,0,1); },
            || { load_byte_negative(0,15,1); },
            || { load_byte_negative(0,1,0); },
            || { stack_registers_w(true,0xc010); },
        ] {
            assert!(std::panic::catch_unwind(encode).is_err());
        }
    }

    #[test]
    fn runtime_arena_stream_position_structure_and_operands() {
        let extension=runtime_arena_and_stream_position();
        let at=|start:usize,end:usize| &extension[start-0xff8..end-0xff8];
        let code=[(0xff8,0x101a),(0x101a,0x1020),(0x1020,0x107e),
                  (0x107e,0x10dc),(0x10dc,0x11ce),(0x11d4,0x11e8),
                  (0x11e8,0x123c),(0x123c,0x1282),(0x1282,0x12be),
                  (0x12c4,0x12d2)];
        let literals=[(0x11d0,0x11d4),(0x12c0,0x12c4)];
        let alignment=[(0x11ce,0x11d0),(0x12be,0x12c0),(0x12d2,0x12d4)];
        let text=[(0x12d4,0x12f5)];
        let padding=[(0x12f5,0x12f8)];
        assert_eq!(code.iter().map(|(start,end)|end-start).sum::<usize>(),718);
        assert_eq!(literals.iter().map(|(start,end)|end-start).sum::<usize>(),8);
        assert_eq!(alignment.iter().map(|(start,end)|end-start).sum::<usize>(),6);
        assert_eq!(text.iter().map(|(start,end)|end-start).sum::<usize>(),33);
        assert_eq!(padding.iter().map(|(start,end)|end-start).sum::<usize>(),3);
        let mut occupied=vec![false;768];
        for (start,end) in code.into_iter().chain(literals).chain(alignment).chain(text).chain(padding) {
            assert!(start>=0xff8 && start<end && end<=0x12f8);
            for byte in &mut occupied[start-0xff8..end-0xff8] {
                assert!(!*byte);
                *byte=true;
            }
        }
        assert!(occupied.into_iter().all(|byte|byte));
        assert_eq!(extension.len(),768);
        for (start,end) in alignment.into_iter().chain(padding) {
            assert!(at(start,end).iter().all(|byte|*byte==0));
        }
        assert_eq!(at(0x11d0,0x11d4),0xffd7cfbfu32.to_le_bytes());
        assert_eq!(at(0x12c0,0x12c4),0x20010u32.to_le_bytes());
        assert_eq!(at(0x12d4,0x12f5),b"SIGRTRED: Redirect: can't open: \0");
        assert_eq!(nop(),0xbf00);
        assert_eq!(tst_register_low(5,0),0x4205);
        assert_eq!(tst_register_low(7,7),0x423f);
        assert_eq!(store_byte_w(7,4,72),[0xf884,0x7048]);
        assert_eq!(store_byte_w(14,13,4095),[0xf88d,0xefff]);
        assert_eq!(at(0x102e,0x1030),nop().to_le_bytes());
        assert_eq!(at(0x11fc,0x11fe),tst_register_low(5,0).to_le_bytes());
        assert_eq!(at(0x11c8,0x11cc),[0x84,0xf8,0x48,0x70]);
        assert_eq!(at(0x1044,0x1046),it(Condition::Eq,ItBlock::One).to_le_bytes());
        assert_eq!(at(0x1046,0x104a),[0,0xf0,0xc1,0xf9]);
        assert_eq!(at(0x1092,0x1094),it(Condition::Ls,ItBlock::Then).to_le_bytes());
        assert_eq!(at(0x10aa,0x10ac),it(Condition::Lo,ItBlock::Then).to_le_bytes());
        assert_eq!(at(0x1076,0x107a),[0xbd,0xe8,0xf8,0x40]);
        assert_eq!(at(0x114e,0x1150),b(0x114e,0x1126).to_le_bytes());
        assert_eq!(at(0x11cc,0x11ce),b(0x11cc,0x1126).to_le_bytes());
        assert_eq!(at(0x12c8,0x12ca),adr(0,0x12c8,0x12d4).to_le_bytes());
        let plan=spans();
        let span=&plan[2];
        assert_eq!(&span.data[0xd98-span.offset..0x1098-span.offset],extension);
        assert_eq!(span.data.len(),3848);
        assert_eq!(span.offset+span.data.len(),0x1098);
        assert_eq!(plan.iter().map(|span|span.data.len()).sum::<usize>(),4312);
        for encode in [
            || { tst_register_low(8,0); },
            || { tst_register_low(0,8); },
            || { store_byte_w(13,0,0); },
            || { store_byte_w(15,0,0); },
            || { store_byte_w(0,15,0); },
            || { store_byte_w(0,0,4096); },
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
