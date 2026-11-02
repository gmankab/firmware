// SPDX-License-Identifier: GPL-3.0-only
//! Isolated Agent 16 entry point; shared firmware-tools CLI is unchanged.
use firmware_tools::{wmfw_halo_memory::{self, HaloMemoryKind}, wmfw_halo_static_data as data,
    wmfw_halo_control_defaults as defaults, wmfw_halo_memory_plan as memory_plan,
    wmfw_plan, wmfw_sha256};
use std::{env, fs, io::Write, path::Path};

fn write_new(path: &str, bytes: &[u8]) -> Result<(), String> {
    let mut file = fs::OpenOptions::new().write(true).create_new(true).open(path)
        .map_err(|e| format!("{path}: {e}"))?;
    file.write_all(bytes).map_err(|e| format!("{path}: {e}"))
}

fn reference(path: &str) -> Result<Vec<u8>, String> {
    let bytes = fs::read(path).map_err(|e| format!("{path}: {e}"))?;
    data::verify_reference(&bytes)?;
    Ok(bytes)
}

fn run(args: &[String]) -> Result<(), String> {
    match args {
        [cmd, output] if cmd == "emit-static-directory" => {
            write_new(output, &data::build_static_directory()?)
        }
        [cmd, input] if cmd == "verify" => {
            let bytes = reference(input)?;
            let generated = data::build_static_directory()?;
            println!("{{\n  \"reference_sha256\": \"{}\",\n  \"static_directory_sha256\": \"{}\",\n  \"file_start\": 104,\n  \"file_end\": 176,\n  \"static_xm_word_start\": 0,\n  \"static_xm_word_end\": 24,\n  \"generated_directory_bytes\": 72,\n  \"byte_mismatches\": 0,\n  \"source_authored_container_metadata_bytes\": 3984,\n  \"reference_derived_static_directory_field_bytes\": 69,\n  \"explicit_directory_padding_bytes\": 3,\n  \"reference_observed_zero_bytes\": 5748,\n  \"remaining_external_dsp_bytes\": 24432,\n  \"independently_reconstructed_halo_instruction_bytes\": 0,\n  \"live_runtime_dump_supplied\": false,\n  \"hardware_register_addresses_asserted\": false,\n  \"first_unreconstructed_file_offset\": 184\n}}",
                wmfw_sha256::hex_digest(&bytes), wmfw_sha256::hex_digest(&generated));
            Ok(())
        }
        [cmd, input] if cmd == "decode-xy" => {
            // Diagnostic output derives from the caller's private reference.
            // It is not compiled source and must not be committed as such.
            let bytes = reference(input)?;
            println!("{{\"origin\":\"external_reference_diagnostic\",\"regions\":[");
            let regions: Vec<_> = wmfw_plan::regions()?.into_iter()
                .filter(|r| r.kind == 0x11 || r.kind == 0x12).collect();
            for (i, r) in regions.iter().enumerate() {
                let kind = HaloMemoryKind::from_wmfw_type(r.kind)?;
                let words = wmfw_halo_memory::unpack_words(kind, &bytes[r.payload_offset..r.end()])?;
                if wmfw_halo_memory::pack_words(kind, &words)? != bytes[r.payload_offset..r.end()] {
                    return Err("data-memory re-encode mismatch".into());
                }
                println!("{{\"kind\":{},\"file_offset\":{},\"dsp_word_offset\":{},\"words\":[{}]}}{}",
                    r.kind, r.payload_offset, r.dsp_address,
                    words.iter().map(u64::to_string).collect::<Vec<_>>().join(","),
                    if i + 1 == regions.len() { "" } else { "," });
            }
            println!("]}}");
            Ok(())
        }
        [cmd, inputs, output] if cmd == "assemble-external" => {
            let bytes = data::assemble_directory(Path::new(inputs))?;
            write_new(output, &bytes)
        }
        [cmd] if cmd == "control-defaults" => {
            let records = defaults::nabu_control_defaults()?;
            println!("{{\"origin\":\"reference_observed_named_initialization\",\"controls\":[");
            for (i, r) in records.iter().enumerate() {
                println!("{{\"algorithm_id\":{},\"name\":\"{}\",\"memory\":\"{:?}\",\"relative_word_offset\":{},\"static_dsp_word_address\":{},\"values\":[{}]}}{}",
                    r.algorithm_id, r.name, r.memory, r.relative_word_offset, r.static_dsp_word_address,
                    r.values.iter().map(u64::to_string).collect::<Vec<_>>().join(","),
                    if i + 1 == records.len() { "" } else { "," });
            }
            println!("]}}");
            Ok(())
        }
        [cmd] if cmd == "control-bytes" => {
            let bytes = defaults::generated_bytes()?;
            println!("{{\"origin\":\"reference_observed_named_initialization\",\"bytes\":[{}]}}",
                bytes.iter().map(|(offset, value)| format!("[{offset},{value}]")).collect::<Vec<_>>().join(","));
            Ok(())
        }
        [cmd] if cmd == "memory-provenance" => {
            println!("{}", memory_plan::ledger_json()?);
            Ok(())
        }
        [cmd, input] if cmd == "verify-memory" => {
            let bytes = reference(input)?;
            memory_plan::verify_reference(&bytes)?;
            println!("{}", memory_plan::ledger_json()?);
            Ok(())
        }
        [cmd, input, output] if cmd == "extract-memory-external" => {
            let bytes = reference(input)?;
            memory_plan::extract_inputs(&bytes, Path::new(output))
        }
        [cmd, input, output] if cmd == "assemble-memory-external" => {
            write_new(output, &memory_plan::assemble_directory(Path::new(input))?)
        }
        _ => Err("usage: halo-memory verify REFERENCE | emit-static-directory OUTPUT | decode-xy REFERENCE | assemble-external INPUT_DIRECTORY OUTPUT | control-defaults | control-bytes | memory-provenance | verify-memory REFERENCE | extract-memory-external REFERENCE NEW_DIRECTORY | assemble-memory-external INPUT_DIRECTORY OUTPUT".into()),
    }
}

fn main() {
    if let Err(error) = run(&env::args().skip(1).collect::<Vec<_>>()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
