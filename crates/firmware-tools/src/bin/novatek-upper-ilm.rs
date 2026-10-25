// SPDX-License-Identifier: GPL-3.0-only
//! Isolated Agent 20-18 emitter; no shared module or CLI registration required.

#[path = "../devices/xiaomi_nabu/novatek_upper_isa.rs"]
mod novatek_upper_isa;
#[path = "../devices/xiaomi_nabu/novatek_upper_ilm.rs"]
mod novatek_upper_ilm;

use std::{env, fs, process};

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [command, output] if command == "emit" => {
            let bytes = novatek_upper_ilm::emit()?;
            fs::write(output, &bytes).map_err(|e| format!("write {output}: {e}"))?;
            println!("Emitted {} typed NDS32 bytes at file [{:#x},{:#x}), runtime [{:#x},{:#x})",
                bytes.len(), novatek_upper_ilm::SOURCE_START, novatek_upper_ilm::SOURCE_END,
                novatek_upper_ilm::SOURCE_START - novatek_upper_ilm::ILM_FILE_BIAS,
                novatek_upper_ilm::SOURCE_END - novatek_upper_ilm::ILM_FILE_BIAS);
            Ok(())
        }
        [command] if command == "inspect" => {
            let _ = novatek_upper_ilm::emit()?;
            println!("reserved_file=[{:#x},{:#x}) source_file=[{:#x},{:#x}) instructions={} bytes={} seam_bytes={}",
                novatek_upper_ilm::RESERVED_START, novatek_upper_ilm::RESERVED_END,
                novatek_upper_ilm::SOURCE_START, novatek_upper_ilm::SOURCE_END,
                novatek_upper_ilm::CODE.len(),
                novatek_upper_ilm::SOURCE_END - novatek_upper_ilm::SOURCE_START,
                novatek_upper_ilm::SOURCE_START - novatek_upper_ilm::RESERVED_START);
            Ok(())
        }
        _ => Err("usage: novatek-upper-ilm emit OUTPUT | inspect\nVerification: tools/devices/xiaomi-nabu/verify-agent20-18.py".into()),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}
