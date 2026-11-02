// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus/upper/venus_upper_controller_limits.rs"]
mod limits;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let out=std::env::args().nth(1).ok_or("usage: venus_upper_controller_limits OUTPUT [PRIVATE_VENUS_MBN]")?;
    let rebuilt=limits::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let bytes=std::fs::read(reference)?;
        let file_offset=limits::START+limits::FILE_BASE;
        if bytes.get(file_offset..file_offset+rebuilt.len())!=Some(rebuilt.as_slice()){
            return Err("controller limits prefix differs from reference".into());
        }
    }
    std::fs::write(out,rebuilt)?;Ok(())
}
