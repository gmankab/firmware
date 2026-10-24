// SPDX-License-Identifier: GPL-3.0-only
//! Agent 06 standalone modem executable fragment generator/verifier.
#[path = "../devices/xiaomi_nabu/modem_hexagon.rs"]
mod modem_hexagon;
#[path = "../devices/xiaomi_nabu/modem_bootstrap.rs"]
mod modem_bootstrap;
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
