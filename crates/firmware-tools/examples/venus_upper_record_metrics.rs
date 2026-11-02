// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus_upper_record_metrics.rs"]
mod metrics;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let dest=std::env::args().nth(1).ok_or("usage: venus_upper_record_metrics OUT [REF]")?;
    let b=metrics::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let orig=std::fs::read(reference)?;let off=metrics::START+metrics::FILE_BASE;
        if orig.get(off..off+b.len())!=Some(&b[..]){return Err("reference differs".into());}
    }
    std::fs::write(dest,b)?;Ok(())
}
