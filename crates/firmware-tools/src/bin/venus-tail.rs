// SPDX-License-Identifier: GPL-3.0-only
//! Standalone bounded Venus tail emission and exact reference comparison.
use firmware_tools::devices::xiaomi_nabu::venus_tail_program as segment;
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let arg=env::args().skip(1).collect::<Vec<_>>();
    if arg.len()!=2 || !(arg[0]=="emit" || arg[0]=="verify") {
        return Err("usage: venus-tail <emit output.bin | verify original/venus.mbn>".into());
    }
    let generated=segment::fragment()?;
    if arg[0]=="emit" {fs::write(&arg[1],&generated)?;}else {
        let reference=fs::read(&arg[1])?;
        let slice=reference.get(segment::FILE_OFFSET..segment::FILE_OFFSET+generated.len()).ok_or("truncated reference image")?;
        if slice!=generated {
            let idx=slice.iter().zip(&generated).position(|(a,b)|a!=b).ok_or("mismatch accounting")?;
            return Err(format!("at runtime 0x{:x}, expected {:02x}, generated {:02x}",segment::START+idx,slice[idx],generated[idx]).into());
        }
    }
    println!("Venus tail: {} exact bytes at runtime [0x{:x},0x{:x}), original MBN file [0x{:x},0x{:x}) ({})",generated.len(),segment::START,segment::END,segment::FILE_OFFSET,segment::FILE_OFFSET+generated.len(),arg[0]);
    Ok(())
}
