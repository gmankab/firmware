// SPDX-License-Identifier: GPL-3.0-only
#[path = "../src/devices/xiaomi_nabu/venus/upper/venus_upper.rs"]
mod upper;
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let path=std::env::args().nth(1).ok_or("usage: venus_upper_emit OUTPUT [ORIGINAL_VENUS_MBN]")?;
    let bytes=upper::fragment()?;
    if let Some(reference)=std::env::args().nth(2) {
        let reference=std::fs::read(reference)?;
        let offset=upper::VERIFIED_START+upper::FILE_MAPPING_DELTA;
        let expected=reference.get(offset..offset+bytes.len()).ok_or("short reference")?;
        if expected!=bytes.as_slice() { return Err("byte mismatch with original Venus".into()); }
        eprintln!("verified {} bytes at ARM 0x{:x}",bytes.len(),upper::VERIFIED_START);
    }
    std::fs::write(path,bytes)?;
    Ok(())
}
