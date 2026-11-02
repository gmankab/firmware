// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus/upper/venus_upper_throughput_tiers.rs"]mod tiers;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let dest=std::env::args().nth(1).ok_or("usage: venus_upper_throughput_tiers OUTPUT [PRIVATE_VENUS_MBN]")?;
    let bytes=tiers::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let firmware=std::fs::read(reference)?;
        let off=tiers::START+tiers::FILE_BASE;
        if firmware.get(off..off+bytes.len()) != Some(bytes.as_slice()) {return Err("codec throughput tier differs from Venus reference".into());}
    }
    std::fs::write(dest,bytes)?;Ok(())
}
