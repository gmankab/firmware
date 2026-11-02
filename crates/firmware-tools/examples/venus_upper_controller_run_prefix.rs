// SPDX-License-Identifier: GPL-3.0-only
#[path="../src/devices/xiaomi_nabu/venus/upper/venus_upper_controller_run_prefix.rs"]mod run;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let file=std::env::args().nth(1).ok_or("usage: venus_upper_controller_run_prefix OUTPUT [PRIVATE_VENUS_MBN]")?;
    let generated=run::fragment()?;
    if let Some(reference)=std::env::args().nth(2){
        let fw=std::fs::read(reference)?;
        let off=run::START+run::FILE_BASE;
        if fw.get(off..off+generated.len())!=Some(generated.as_slice()) {
            return Err("controller run prefix differs from supplied reference".into());
        }
    }
    std::fs::write(file,generated)?;Ok(())
}
