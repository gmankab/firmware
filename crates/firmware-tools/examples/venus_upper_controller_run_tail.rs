// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus_upper_controller_run_tail.rs"]mod tail;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let filename=std::env::args().nth(1).ok_or("usage: venus_upper_controller_run_tail OUTPUT [PRIVATE_VENUS_MBN]")?;
    let bytes=tail::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let firmware=std::fs::read(reference)?;
        let offset=tail::START+tail::FILE_BASE;
        if firmware.get(offset..offset+bytes.len())!=Some(bytes.as_slice()){
            return Err("controller run tail reference bytes differ".into());
        }
    }
    std::fs::write(filename,bytes)?;Ok(())
}
