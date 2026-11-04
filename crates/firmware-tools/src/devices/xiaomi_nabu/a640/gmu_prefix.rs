// SPDX-License-Identifier: GPL-3.0-only
//! Independently encoded Adreno A640 GMU load headers, Cortex-M reset code,
//! and exception vectors. Later GMU executable/data payloads remain absent.
//!
//! The first two blocks (400 contiguous bytes) are complete and independent;
//! the other five blocks contribute only their typed 16-byte load headers.

pub const IMAGE_BYTES: usize = 37_680;
pub const INITIAL_PREFIX_BYTES: usize = 400;
pub const MODELLED_BYTES: usize = 480;
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
fn push16(out:&mut Vec<u8>,word:u16) { out.extend_from_slice(&word.to_le_bytes()); }
fn push32(out:&mut Vec<u8>,words:[u16;2]) { for word in words { push16(out,word); } }

/// Reset vectors, typed Thumb-2 prologue and PC-relative MMIO literal pool.
/// The branch targets GMU ITCM 0x400, whose implementation is not yet built.
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

fn header(address:u32,size:u32)->Vec<u8> {
    let mut out=Vec::with_capacity(16);
    for word in [address,size,LOAD_MARKER,LOAD_MARKER] {
        out.extend_from_slice(&word.to_le_bytes());
    }
    out
}

/// Seven sparse spans: all headers plus the first two block payloads.
pub fn spans()->Vec<Span> {
    let mut result=Vec::with_capacity(7);
    let mut offset=0;
    for (index,(address,size)) in BLOCKS.into_iter().enumerate() {
        let mut data=header(address,size);
        match index {
            0=>data.extend_from_slice(&reset_block()),
            1=>data.extend_from_slice(&vector_table()),
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
    }
}
