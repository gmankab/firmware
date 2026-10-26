// SPDX-License-Identifier: GPL-3.0-only
//! Isolated Agent17 CLI: no central registry or Cargo manifest changes.
#[path="../devices/xiaomi_nabu/novatek_ilmlow.rs"]
mod lower;
use std::{env,fs,process};
fn run()->Result<(),String>{
    let a:Vec<_>=env::args().skip(1).collect();
    if a.len()<2{return Err("usage: novatek-ilm-low emit|verify|listing PROFILE [FILE]".into());}
    let profile=lower::Profile::parse(&a[1])?;
    match a[0].as_str(){
        "emit" if a.len()==3=>{let bytes=lower::build(profile)?;fs::write(&a[2],&bytes).map_err(|e|e.to_string())?;
            println!("{}: {} typed source bytes (instructions plus distinguished jump-table data), file [0x{:x},0x{:x}), runtime [0x{:x},0x{:x})",profile.name(),bytes.len(),lower::FILE_START,profile.file_end(),lower::FILE_START-lower::ILM_FILE_BASE,profile.file_end()-lower::ILM_FILE_BASE);},
        "verify" if a.len()==3=>{let firmware=fs::read(&a[2]).map_err(|e|e.to_string())?;lower::compare(&firmware,profile)?;
            println!("{}: zero byte mismatches in {} bytes; caller must pin whole-image SHA256 {}",profile.name(),profile.file_end()-lower::FILE_START,profile.reference_sha256());},
        "verify-contiguous" if a.len()==3 && profile==lower::Profile::Postmarketos=>{
            let firmware=fs::read(&a[2]).map_err(|e|e.to_string())?;
            lower::contiguous::compare_postmarketos(&firmware)?;
            println!("postmarketos: full source-generated contiguous [0x{:x},0x{:x}) exactly matches {} bytes; zero external instruction bytes",
                     lower::FILE_START,lower::contiguous::FILE_END,
                     lower::contiguous::FILE_END-lower::FILE_START);
        },
        "emit-contiguous" if a.len()==3 && profile==lower::Profile::Postmarketos=>{
            let generated=lower::contiguous::build_postmarketos()?;
            fs::write(&a[2],&generated).map_err(|e|e.to_string())?;
            println!("postmarketos: emitted {} source-generated adjacent lower-ILM bytes",generated.len());
        },
        "verify-disjoint" if a.len()==3 && profile==lower::Profile::Postmarketos=>{
            let firmware=fs::read(&a[2]).map_err(|e|e.to_string())?;
            let sections=[lower::extension6_disjoint::first_after_unresolved(),
                          lower::extension6_disjoint::second_after_unresolved(),
                          lower::extension7_disjoint::fragment(),
                          lower::extension8_disjoint::fragment(),
                          lower::extension9_disjoint::fragment(),
                          lower::extension10_disjoint::fragment()];
            let mut total=0;
            for segment in sections {
                let built=lower::extension6_disjoint::emit(&segment)?;
                let original=firmware.get(segment.file_start..segment.file_end)
                    .ok_or("truncated firmware at disjoint code region")?;
                if original!=built.as_slice() {
                    let mismatch=built.iter().zip(original).position(|(a,b)|a!=b).unwrap_or(0);
                    return Err(format!("disjoint source mismatch at file 0x{:x}",segment.file_start+mismatch));
                }
                println!("postmarketos: disjoint [0x{:x},0x{:x}) {} independently encoded bytes match",
                         segment.file_start,segment.file_end,built.len());
                total+=built.len();
            }
            println!("{} disjoint exact bytes; the two 16-bit MOVD44 gap instructions are reconstructed by verify-contiguous; sixth region ends at 0x7ab0",total);
        },
        "emit-disjoint" if a.len()==3 && profile==lower::Profile::Postmarketos=>{
            let sections=[lower::extension6_disjoint::first_after_unresolved(),
                          lower::extension6_disjoint::second_after_unresolved(),
                          lower::extension7_disjoint::fragment(),
                          lower::extension8_disjoint::fragment(),
                          lower::extension9_disjoint::fragment(),
                          lower::extension10_disjoint::fragment()];
            for section in sections {
                let name=format!("{}-{:05x}-{:05x}.bin",a[2],section.file_start,section.file_end);
                fs::write(&name,lower::extension6_disjoint::emit(&section)?).map_err(|e|e.to_string())?;
                println!("wrote {name}");
            }
        },
        "listing" if a.len()==2=>{println!("# profile={} gp=0x{:x} itb=0x{:x} file_start=0x{:x} file_end=0x{:x}",profile.name(),profile.gp(),profile.itb(),lower::FILE_START,profile.file_end());
            for item in lower::program(profile){let b=item.instruction.encode(item.pc,profile.gp())?;
                println!("{:x}\t{}\t{}",item.pc,b.len(),item.instruction.assembly(item.pc,profile.gp()));}},
        _=>return Err("invalid novatek-ilm-low command or argument count".into()),
    }Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");process::exit(1)}}
