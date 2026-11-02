// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus/upper/venus_upper_control_flags.rs"]
mod flags;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let dest=std::env::args().nth(1).ok_or("usage: venus_upper_control_flags OUT [REF]")?;
    let bytes=flags::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let orig=std::fs::read(reference)?;
        let off=flags::START+flags::FILE_BASE;
        if orig.get(off..off+bytes.len())!=Some(bytes.as_slice()){
            return Err("control flag helper byte mismatch".into());
        }
    }
    std::fs::write(dest,bytes)?;Ok(())
}
