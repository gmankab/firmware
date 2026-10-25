// SPDX-License-Identifier: GPL-3.0-only
//! Source-derived Novatek NT36xxx hardware CRC algorithm and checked writer.
//!
//! The observed format uses **CRC-32/MPEG-2** (poly=0x04C11DB7,
//! init=0xffffffff, refin=false, refout=false, xorout=0). This is confirmed
//! independently across both Nabu revisions for ILM, DLM, all six information
//! descriptors, and both metadata bank footers. Each bank footer at +0xfc
//! covers the first 252 bytes of its own 256-byte metadata bank, rather than
//! the separately encoded `file_offset`/`size_minus_one` inside the footer.
//!
//! Correct hardware CRCs do not establish authenticity or MCU ISA semantics.
use super::novatek_layout::{NabuNovatekLayout,BANK_BYTES,IMAGE_LENGTH};

const POLYNOMIAL: u32 = 0x04c1_1db7;
const INITIAL: u32 = 0xffff_ffff;
const BANK_CRC_FIELD: usize = 0xfc;

/// Nonreflected CRC32 with no final XOR. Standard vector `123456789` is
/// `0x0376e6e7`. The bit-serial algorithm avoids external dependencies.
pub fn crc32_mpeg2(input: &[u8]) -> u32 {
    let mut crc = INITIAL;
    for &byte in input {
        crc ^= (byte as u32) << 24;
        for _ in 0..8 {
            crc = (crc << 1) ^ if crc & 0x8000_0000 != 0 { POLYNOMIAL } else { 0 };
        }
    }
    crc
}

fn read_u32(input: &[u8], offset: usize) -> Result<u32, String> {
    let range = input.get(offset..offset.checked_add(4).ok_or("offset overflow")?)
        .ok_or_else(|| format!("missing CRC word at offset 0x{offset:x}"))?;
    Ok(u32::from_le_bytes(range.try_into().expect("four bytes")))
}
fn write_u32(out: &mut [u8], offset: usize, value: u32) -> Result<(), String> {
    out.get_mut(offset..offset.checked_add(4).ok_or("offset overflow")?)
        .ok_or_else(|| format!("missing header word at 0x{offset:x}"))?
        .copy_from_slice(&value.to_le_bytes());
    Ok(())
}
fn crc_for_range(input:&[u8],start:u32,size_minus_one:u32)->Result<u32,String>{
    let end = (start as usize).checked_add((size_minus_one as usize).checked_add(1)
        .ok_or("CRC length overflow")?).ok_or("CRC file offset overflow")?;
    let data = input.get(start as usize..end)
        .ok_or_else(|| format!("CRC region 0x{start:x}..0x{end:x} outside firmware"))?;
    Ok(crc32_mpeg2(data))
}

/// First two common partitions + three info entries in each metadata bank +
/// one bank checksum per 256-byte metadata bank: ten logical checks and 12
/// physical 32-bit stored fields (core CRCs are mirrored).
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct CrcAudit {
    pub core: usize,
    pub info: usize,
    pub bank: usize,
}
impl CrcAudit {
    pub fn total_checks(self) -> usize { self.core+self.info+self.bank }
    pub fn stored_fields(self) -> usize { self.core*2 + self.info+self.bank }
}

fn check_word(input:&[u8],file_offset:usize,expected:u32,name:&str)->Result<(),String>{
    let actual=read_u32(input,file_offset)?;
    if actual!=expected {
        return Err(format!("{name} CRC mismatch at 0x{file_offset:x}: stored 0x{actual:08x}, computed 0x{expected:08x}"));
    }
    Ok(())
}

/// Validate all 12 stored checksum words against *authored payload bytes*, or
/// against the OEM reference for investigation. This method never substitutes
/// missing input and fails rather than treating the OEM bytes as source code.
pub fn verify_all_crcs(layout:&NabuNovatekLayout,input:&[u8])->Result<CrcAudit,String>{
    if input.len()!=IMAGE_LENGTH || input.len()!=layout.image_len {
        return Err("Novatek CRC reference image length mismatch".into());
    }
    layout.coverage()?;
    let core=[
        ("ILM",layout.ilm.file_offset,layout.ilm.size_minus_one,0x18),
        ("DLM",layout.dlm.file_offset,layout.dlm.size_minus_one,0x1c),
    ];
    for (name,start,len,offset) in core {
        let expected=crc_for_range(input,start,len)?;
        for bank in [0,BANK_BYTES] {
            check_word(input,bank+offset,expected,name)?;
        }
    }
    for (i,bank) in layout.banks.iter().enumerate() {
        let base=i*BANK_BYTES;
        for (j,info) in bank.info.iter().enumerate() {
            let expected=crc_for_range(input,info.file_offset,info.size_minus_one)?;
            check_word(input,base+0x3c+j*16,expected,"info")?;
        }
        let crc=crc32_mpeg2(&input[base..base+BANK_CRC_FIELD]);
        check_word(input,base+BANK_CRC_FIELD,crc,"bank metadata")?;
    }
    Ok(CrcAudit{core:2,info:6,bank:2})
}

/// Deterministically compute *all* stored CRCs from the completed caller-
/// authored image, including both bank footers. The function does not
/// independently author the underlying ILM or DLM instruction bytes.
/// All pre-existing CRC words are overwritten; changing payload bytes therefore
/// produces different stored CRCs without needing proprietary checksum inputs.
pub fn recompute_all_crcs(image:&mut [u8])->Result<CrcAudit,String>{
    if image.len()!=IMAGE_LENGTH {return Err("wrong Nabu firmware size".into());}
    let layout=NabuNovatekLayout::parse(image)?;
    layout.coverage()?;
    for (start,len,offset) in [
        (layout.ilm.file_offset,layout.ilm.size_minus_one,0x18),
        (layout.dlm.file_offset,layout.dlm.size_minus_one,0x1c),
    ] {
        let value=crc_for_range(image,start,len)?;
        for bank in [0,BANK_BYTES] {write_u32(image,bank+offset,value)?;}
    }
    for (i,bank) in layout.banks.iter().enumerate() {
        let base=i*BANK_BYTES;
        for (j,info) in bank.info.iter().enumerate() {
            let value=crc_for_range(image,info.file_offset,info.size_minus_one)?;
            write_u32(image,base+0x3c+j*16,value)?;
        }
    }
    for bank in [0,BANK_BYTES] {
        let value=crc32_mpeg2(&image[bank..bank+BANK_CRC_FIELD]);
        write_u32(image,bank+BANK_CRC_FIELD,value)?;
    }
    let verified=NabuNovatekLayout::parse(image)?;
    verify_all_crcs(&verified,image)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::novatek_layout::{CorePartition,InfoDescriptor,DescriptorBank,END_MARKER_OFFSET,HEADER_BYTES};
    fn synthetic()->Vec<u8>{
        let ilm=CorePartition{file_offset:0x200,sram_address:0,size_minus_one:0x1aa7f,stored_crc:0};
        let dlm=CorePartition{file_offset:0x22000,sram_address:0x20000,size_minus_one:0xc18f,stored_crc:0};
        let info=InfoDescriptor{sram_address:0x20000,size_minus_one:0,file_offset:0x22000,stored_crc:0};
        let layout=NabuNovatekLayout{ilm,dlm,image_len:IMAGE_LENGTH,banks:[
            DescriptorBank{mode:3,options:0,info:[info;3],auxiliary:info},
            DescriptorBank{mode:1,options:0,info:[info;3],auxiliary:info}]};
        let mut image=vec![0;IMAGE_LENGTH];
        image[..HEADER_BYTES].copy_from_slice(&layout.write_header());
        image[0x450]=0x23; image[0x22000]=0x41;
        image[0x2f000]=0x41; // mirror of first authored DLM byte
        image[END_MARKER_OFFSET..].copy_from_slice(b"NVT");
        image
    }
    #[test]
    fn mpeg2_crc_known_public_vectors(){
        assert_eq!(crc32_mpeg2(b""),0xffff_ffff);
        assert_eq!(crc32_mpeg2(b"123456789"),0x0376_e6e7);
    }
    #[test]
    fn roundtrips_independently_authored_test_payload_and_all_crc_fields(){
        let mut image=synthetic();
        let audit=recompute_all_crcs(&mut image).unwrap();
        assert_eq!(audit.total_checks(),10);
        assert_eq!(audit.stored_fields(),12);
        let layout=NabuNovatekLayout::parse(&image).unwrap();
        assert_eq!(verify_all_crcs(&layout,&image).unwrap(),audit);
        assert_eq!(read_u32(&image,0x18).unwrap(),read_u32(&image,0x118).unwrap());
        assert_ne!(read_u32(&image,0xfc).unwrap(),read_u32(&image,0x1fc).unwrap());
    }
    #[test]
    fn bank_footer_crc_excludes_its_last_four_bytes() {
        let mut image=synthetic();
        recompute_all_crcs(&mut image).unwrap();
        let bank_crc=read_u32(&image,0xfc).unwrap();
        assert_eq!(bank_crc,crc32_mpeg2(&image[..0xfc]));
        image[0xfc]^=1;
        let layout=NabuNovatekLayout::parse(&image).unwrap();
        assert!(verify_all_crcs(&layout,&image).unwrap_err().contains("bank metadata"));
        recompute_all_crcs(&mut image).unwrap();
        assert_eq!(read_u32(&image,0xfc).unwrap(),bank_crc);
    }
    #[test]
    fn tampered_payload_or_metadata_is_rejected_and_repaired(){
        let mut image=synthetic();
        recompute_all_crcs(&mut image).unwrap();
        image[0x470]^=1;
        let layout=NabuNovatekLayout::parse(&image).unwrap();
        assert!(verify_all_crcs(&layout,&image).unwrap_err().contains("ILM"));
        recompute_all_crcs(&mut image).unwrap();
        let layout=NabuNovatekLayout::parse(&image).unwrap();
        verify_all_crcs(&layout,&image).unwrap();
        image[0x4c]^=1; // bank0 info-1 CRC field: protected by per-item CRC
        let layout=NabuNovatekLayout::parse(&image).unwrap();
        assert!(verify_all_crcs(&layout,&image).unwrap_err().contains("info"));
        recompute_all_crcs(&mut image).unwrap();
        let layout=NabuNovatekLayout::parse(&image).unwrap();
        verify_all_crcs(&layout,&image).unwrap();
    }
}
