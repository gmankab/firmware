// SPDX-License-Identifier: GPL-3.0-only
//! Isolated Agent 20-18 emitter; no shared module or CLI registration required.

#[path = "../devices/xiaomi_nabu/novatek/ilm/novatek_upper_isa.rs"]
mod novatek_upper_isa;
#[path = "../devices/xiaomi_nabu/novatek/ilm/novatek_upper_ilm.rs"]
mod novatek_upper_ilm;
#[path = "../devices/xiaomi_nabu/novatek/ilm/novatek_upper_ex9.rs"]
mod novatek_upper_ex9;
#[path = "../devices/xiaomi_nabu/novatek/ilm/novatek_upper_log_data.rs"]
mod novatek_upper_log_data;
#[path = "../devices/xiaomi_nabu/novatek/ilm/novatek_upper_math_data.rs"]
mod novatek_upper_math_data;
#[path = "../devices/xiaomi_nabu/novatek/ilm/novatek_upper_allocations.rs"]
mod novatek_upper_allocations;
#[path = "../devices/xiaomi_nabu/novatek/ilm/novatek_upper_boundary.rs"]
mod novatek_upper_boundary;
#[path = "../devices/xiaomi_nabu/novatek/ilm/novatek_upper_aux_tables.rs"]
mod novatek_upper_aux_tables;
#[path = "../devices/xiaomi_nabu/novatek/ilm/novatek_upper_catalog.rs"]
mod novatek_upper_catalog;

use std::{env, fs, process};

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [command, output] if command == "emit" => {
            let bytes = novatek_upper_ilm::emit()?;
            fs::write(output, &bytes).map_err(|e| format!("write {output}: {e}"))?;
            println!("Emitted {} upper-ILM bytes at file [{:#x},{:#x}), runtime [{:#x},{:#x})",
                bytes.len(), novatek_upper_ilm::SOURCE_START, novatek_upper_ilm::SOURCE_END,
                novatek_upper_ilm::SOURCE_START - novatek_upper_ilm::ILM_FILE_BIAS,
                novatek_upper_ilm::SOURCE_END - novatek_upper_ilm::ILM_FILE_BIAS);
            Ok(())
        }
        [command, directory] if command == "export" => {
            let fragments = novatek_upper_catalog::fragments()?;
            let output = std::path::Path::new(directory);
            fs::create_dir_all(output).map_err(|e| format!("create {directory}: {e}"))?;
            for fragment in &fragments {
                fs::write(output.join(fragment.filename()), &fragment.bytes)
                    .map_err(|e| format!("write fragment: {e}"))?;
            }
            let manifest = novatek_upper_catalog::manifest(&fragments)?;
            fs::write(output.join("manifest.json"), manifest)
                .map_err(|e| format!("write manifest: {e}"))?;
            println!("Exported {} source fragments; {} covered bytes; {} unresolved bytes",
                fragments.len(), novatek_upper_catalog::GENERATED_BYTES,
                novatek_upper_catalog::UNRESOLVED_BYTES);
            Ok(())
        }
        [command] if command == "inspect" => {
            println!("{}", novatek_upper_catalog::manifest(&novatek_upper_catalog::fragments()?)?);
            Ok(())
        }
        _ => Err("usage: novatek-upper-ilm emit MAIN_OUTPUT | export DIRECTORY | inspect\nVerification: tools/devices/xiaomi-nabu/verify-novatek-upper.py".into()),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}
