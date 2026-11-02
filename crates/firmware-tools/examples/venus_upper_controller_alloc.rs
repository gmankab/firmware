// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus_upper_controller_alloc.rs"]mod alloc;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let dest=std::env::args().nth(1).ok_or("usage: venus_upper_controller_alloc OUTPUT [PRIVATE_VENUS_MBN]")?;
    let bytes=alloc::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let firmware=std::fs::read(reference)?;let off=alloc::START+alloc::FILE_BASE;
        if firmware.get(off..off+bytes.len())!=Some(bytes.as_slice()){return Err("controller allocator differs from reference".into());}
    }
    std::fs::write(dest,bytes)?;Ok(())
}
