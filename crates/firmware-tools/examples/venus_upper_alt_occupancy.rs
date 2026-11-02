// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus/upper/venus_upper_alt_occupancy.rs"]
mod accumulator;
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let dest=std::env::args().nth(1).ok_or("usage: venus_upper_alt_occupancy OUTPUT [PRIVATE_VENUS_MBN]")?;
    let reconstructed=accumulator::fragment()?;
    if let Some(input)=std::env::args().nth(2){
        let original=std::fs::read(input)?;
        let off=accumulator::START+accumulator::FILE_BASE;
        let bytes=original.get(off..off+reconstructed.len()).ok_or("reference image too short")?;
        if bytes!=reconstructed {return Err("reference bytes differ from reconstructed accumulator".into());}
    }
    std::fs::write(dest,reconstructed)?;Ok(())
}
