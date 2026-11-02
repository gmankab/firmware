// SPDX-License-Identifier: GPL-3.0-only
//! Agent 06 standalone modem executable fragment generator/verifier.
#[path = "../devices/xiaomi_nabu/modem/common/modem_hexagon.rs"]
mod modem_hexagon;
#[path = "../devices/xiaomi_nabu/modem/common/modem_bootstrap.rs"]
mod modem_bootstrap;
#[path = "../devices/xiaomi_nabu/modem/common/modem_boot_manifest.rs"]
mod modem_boot_manifest;
#[path = "../devices/xiaomi_nabu/modem/common/modem_tlb_tail.rs"]
mod modem_tlb_tail;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args=env::args().skip(1);
    let action=args.next().ok_or("usage: modem-rebuild (emit OUTPUT | verify MODEM_MBN)")?;
    let path=args.next().ok_or("missing input/output path")?;
    if args.next().is_some() { return Err("unexpected additional argument".into()); }
    match action.as_str() {
        "emit" => {
            let bytes=modem_bootstrap::rebuild()?;
            fs::write(&path, &bytes)?;
            println!("Generated {} independent Hexagon bootstrap bytes, ELF file offset 0x{:x}, physical 0x{:08x}",
                     bytes.len(),modem_bootstrap::ELF_OFFSET,modem_bootstrap::PHYSICAL_ADDR);
        },
        "emit-tlb-tail" => {
            let bytes=modem_tlb_tail::rebuild()?;
            fs::write(&path,&bytes)?;
            println!("Generated {} independent Hexagon TLB/cache-management bytes at file offset 0x{:x}; disconnected fragment",
                     bytes.len(),modem_tlb_tail::FILE_START);
        },
        "verify-tlb-tail" => {
            let reference=fs::read(path)?;
            modem_tlb_tail::verify(&reference)?;
            println!("Verified {} independently reconstructed TLB/cache bytes at file 0x{:x}, physical 0x{:08x}; partial firmware",
                     modem_tlb_tail::CODE_LENGTH,modem_tlb_tail::FILE_START,modem_tlb_tail::PHYSICAL_ADDRESS);
        },
        "emit-layout" => {
            let mut bytes=modem_bootstrap::rebuild()?;
            bytes.extend(modem_boot_manifest::rebuild()?);
            fs::write(&path, &bytes)?;
            println!("Generated {} bytes of modem bootstrap instructions + typed configuration records and padding; PARTIAL",bytes.len());
        },
        "verify-layout" => {
            let image=fs::read(path)?;
            modem_bootstrap::verify(&image)?;
            let expected=modem_boot_manifest::rebuild()?;
            let start=modem_boot_manifest::FILE_START;
            let actual=image.get(start..start+expected.len()).ok_or("reference truncated in boot manifest")?;
            if let Some(i)=expected.iter().zip(actual.iter()).position(|(a,b)|a!=b) {
                return Err(format!("modem boot manifest mismatch at file 0x{:x}: expected 0x{:02x}, got 0x{:02x}",start+i,actual[i],expected[i]).into());
            }
            println!("Verified {} contiguous bytes at modem file [0x3000,0x3500): 68 code bytes, {} metadata/fill bytes; PARTIAL",0x500,expected.len());
        },
        "verify" => {
            let image=fs::read(path)?;
            modem_bootstrap::verify(&image)?;
            println!("Verified {} reconstructed Hexagon entry bytes at file offset 0x{:x} (physical 0x{:08x}, ELF vaddr 0x{:08x}); PARTIAL firmware",
                     modem_bootstrap::CODE_LENGTH, modem_bootstrap::ELF_OFFSET,
                     modem_bootstrap::PHYSICAL_ADDR, modem_bootstrap::VIRTUAL_ADDR);
        },
        _ => return Err("unknown action; use emit or verify".into()),
    }
    Ok(())
}
