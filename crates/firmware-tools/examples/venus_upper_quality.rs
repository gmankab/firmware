// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus_upper_quality.rs"]
mod quality;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let path=std::env::args().nth(1).ok_or("usage: venus_upper_quality OUT [REF]")?;
    let b=quality::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let o=std::fs::read(reference)?;
        let start=quality::START+quality::FILE_BASE;
        if o.get(start..start+b.len())!=Some(&b[..]){return Err("reference differs".into());}
    }
    std::fs::write(path,b)?;Ok(())
}
