// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus_agent20_05_occupancy_commit.rs"]
mod updater;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let output=std::env::args().nth(1).ok_or("usage: agent20_05_occupancy_commit OUTPUT [PRIVATE_VENUS_MBN]")?;
    let bytes=updater::fragment()?;
    if let Some(reference)=std::env::args().nth(2) {
        let fw=std::fs::read(reference)?;let off=updater::START+updater::FILE_BASE;
        if fw.get(off..off+bytes.len())!=Some(bytes.as_slice()){return Err("occupancy commit byte mismatch".into());}
    }
    std::fs::write(output,bytes)?;Ok(())
}
