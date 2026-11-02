// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus_upper_magnitude.rs"]
mod m;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let dest=std::env::args().nth(1).ok_or("usage: venus_upper_magnitude OUT [REF]")?;
    let bytes=m::fragment()?;
    if let Some(ref_file)=std::env::args().nth(2){
        let orig=std::fs::read(ref_file)?;let off=m::START+m::FILE_BASE;
        if orig.get(off..off+bytes.len())!=Some(&bytes[..]){return Err("reference mismatch".into());}
    }
    std::fs::write(dest,bytes)?;Ok(())
}
