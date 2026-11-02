// SPDX-License-Identifier: GPL-3.0-only
//! Agent 20-02 Venus seam emitter and strict reference comparator.
//! Original OEM inputs remain private; the source-built fragment contains
//! A32 instructions and explicit data words, not complete signed firmware.
use firmware_tools::devices::xiaomi_nabu::{venus_runtime_seam as seam, venus_runtime_late as late};
use std::{env,fs};

fn joined_with_agent03() -> Result<Vec<u8>,Box<dyn std::error::Error>> {
    if seam::END != late::START {
        return Err(format!("seam end 0x{:x} is not Agent 03 start 0x{:x}",seam::END,late::START).into());
    }
    let mut out=seam::fragment()?;
    out.extend_from_slice(&late::fragment()?);
    if out.len()!=late::END-seam::START {
        return Err("unexpected joined Venus late span length".into());
    }
    Ok(out)
}
fn main()->Result<(),Box<dyn std::error::Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: cargo run --example venus_seam -- <emit|verify|emit-joined-late|verify-joined-late> path".into());}
    let (bytes,starting_runtime) = match args[0].as_str(){
      "emit"=> (seam::fragment()?,seam::START),
      "verify"=> {seam::verify_reference(&fs::read(&args[1])?)?; (seam::fragment()?,seam::START)},
      "emit-joined-late"|"verify-joined-late"=> (joined_with_agent03()?,seam::START),
      _=>return Err("unknown mode".into()),
    };
    let target=&args[1];
    let file_offset=seam::FILE_BASE+starting_runtime;
    if args[0]=="emit"||args[0]=="emit-joined-late" {
        fs::write(target,&bytes)?;
    } else {
        let reference=fs::read(target)?;
        let actual=reference.get(file_offset..file_offset+bytes.len()).ok_or("reference MBN does not cover requested span")?;
        if let Some((i,(reference_byte,generated_byte)))=actual.iter().zip(&bytes).enumerate().find(|(_, (a,b))|a!=b){
            return Err(format!("byte mismatch at firmware file 0x{:x}, runtime 0x{:x}: reference={reference_byte:02x} generated={generated_byte:02x}",file_offset+i,starting_runtime+i).into());
        }
    }
    println!("{}: {} byte-identical typed ARM32/data bytes, runtime [0x{:x},0x{:x}), MBN file [0x{:x},0x{:x}); NOT flashable",args[0],bytes.len(),starting_runtime,starting_runtime+bytes.len(),file_offset,file_offset+bytes.len());
    Ok(())
}
