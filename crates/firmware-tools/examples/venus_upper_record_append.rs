// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus/upper/venus_upper_record_append.rs"]
mod append;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let output=std::env::args().nth(1).ok_or("usage: venus_upper_record_append OUTPUT [PRIVATE_VENUS_MBN]")?;
    let bytes=append::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let original=std::fs::read(reference)?;
        let off=append::START+append::FILE_BASE;
        if original.get(off..off+bytes.len())!=Some(bytes.as_slice()) {return Err("record-append byte mismatch".into());}
    }
    std::fs::write(output,bytes)?;Ok(())
}
