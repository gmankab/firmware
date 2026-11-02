// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus_upper_controller_limits_tail.rs"]
mod limits;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let out=std::env::args().nth(1).ok_or("usage: venus_upper_controller_limits_tail OUTPUT [PRIVATE_VENUS_MBN]")?;
    let b=limits::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let firmware=std::fs::read(reference)?;let off=limits::START+limits::FILE_BASE;
        if firmware.get(off..off+b.len())!=Some(b.as_slice()){return Err("controller tail differs from reference".into());}
    }
    std::fs::write(out,b)?;Ok(())
}
