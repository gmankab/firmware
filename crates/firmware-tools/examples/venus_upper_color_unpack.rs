// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus/upper/venus_upper_color_unpack.rs"]
mod un;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let dest=std::env::args().nth(1).ok_or("usage: venus_upper_color_unpack OUT [REF]")?;
    let b=un::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let o=std::fs::read(reference)?;
        let off=un::START+un::FILE_BASE;
        if o.get(off..off+b.len())!=Some(&b[..]){return Err("reference mismatch".into());}
    }
    std::fs::write(dest,b)?;Ok(())
}
