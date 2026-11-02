// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus/upper/venus_upper_record_select.rs"]
mod selector;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let out=std::env::args().nth(1).ok_or("usage: venus_upper_record_select OUTPUT [PRIVATE_VENUS_MBN]")?;
    let b=selector::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let fw=std::fs::read(reference)?;
        let offset=selector::START+selector::FILE_BASE;
        if fw.get(offset..offset+b.len())!=Some(b.as_slice()){return Err("record selector byte mismatch".into());}
    }
    std::fs::write(out,b)?;Ok(())
}
