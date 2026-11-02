// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus/upper/venus_upper_controller_small_helpers.rs"]mod helpers;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let file=std::env::args().nth(1).ok_or("usage: venus_upper_controller_small_helpers OUTPUT [PRIVATE_VENUS_MBN]")?;
    let bytes=helpers::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let fw=std::fs::read(reference)?;let off=helpers::START+helpers::FILE_BASE;
        if fw.get(off..off+bytes.len())!=Some(bytes.as_slice()){return Err("controller helpers do not match reference".into());}
    }
    std::fs::write(file,bytes)?;Ok(())
}
