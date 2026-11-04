// SPDX-License-Identifier: GPL-3.0-only

pub const IMAGE_BYTES: usize = 37_680;
pub const INITIAL_PREFIX_BYTES: usize = 400;
pub const MODELLED_BYTES: usize = 540;
pub const STARTUP_BYTES: usize = 60;
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
    assert!(register<16 && matches!(kind,8|9));
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
    assert!(register<13);
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
    assert_eq!(out.len(),STARTUP_BYTES);
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
            2=>data.extend_from_slice(&startup()),
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
        assert_eq!(startup().len(),60);
        assert_eq!(spans()[2].offset+16,416);
        assert_eq!(spans()[2].data.len(),76);
        assert_eq!(MODELLED_BYTES,540);
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
