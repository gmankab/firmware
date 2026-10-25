// SPDX-License-Identifier: GPL-3.0-only
//! CDSP partial reconstruction command. Reference bytes stay outside Git.
use firmware_tools::devices::xiaomi_nabu::{cdsp_boot as cdsp, cdsp_privilege as privileged, cdsp_tlb as tlb};
use std::{env, fs};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    match arguments.as_slice() {
        [action, output] if action == "emit-entry" => {
            let bytes = cdsp::build_entry()?;
            fs::write(output, bytes)?;
            println!("Emitted {} independently encoded CDSP Hexagon entry bytes, file offsets {:#x}..{:#x}; partial firmware only.",
                cdsp::REBUILT_LEN, cdsp::START_FILE_OFFSET, cdsp::START_FILE_OFFSET + cdsp::REBUILT_LEN);
        }
        [action, output] if action == "emit-syscfg-reset" => {
            let bytes = privileged::build_syscfg_reset()?;
            fs::write(output, bytes)?;
            println!("Emitted {} CDSP Hexagon privileged-reset instruction bytes at file {:#x}..{:#x}. Disconnected fragment; no flashable image.",
                privileged::FUNCTION_SIZE, privileged::FILE_OFFSET,
                privileged::FILE_OFFSET + privileged::FUNCTION_SIZE);
        }
        [action, original] if action == "verify-syscfg-reset" => {
            let image = fs::read(original)?;
            privileged::verify_syscfg_reset(&image)?;
            println!("Verified {} CDSP Hexagon supervisor-reset bytes at file {:#x}..{:#x}, separate from the entry prefix.",
                privileged::FUNCTION_SIZE, privileged::FILE_OFFSET,
                privileged::FILE_OFFSET + privileged::FUNCTION_SIZE);
        }
        [action, output] if action == "emit-tlb-full" => {
            let code=tlb::build_full_tlb_span()?;
            fs::write(output,code)?;
            println!("Generated {} adjacent CDSP TLB supervisor bytes at file {:#x}..{:#x}.",
                tlb::FULL_TLB_SIZE,tlb::CONDITIONAL_OFFSET,
                tlb::CONDITIONAL_OFFSET+tlb::FULL_TLB_SIZE);
        }
        [action, input] if action == "verify-tlb-full" => {
            let reference=fs::read(input)?;
            tlb::verify_full_tlb_span(&reference)?;
            println!("Verified {} contiguous TLB supervisor bytes at file {:#x}..{:#x}.",
                tlb::FULL_TLB_SIZE,tlb::CONDITIONAL_OFFSET,
                tlb::CONDITIONAL_OFFSET+tlb::FULL_TLB_SIZE);
        }
        [action, output] if action == "emit-tlb-maintenance" => {
            let bytes=tlb::build_tlb_maintenance()?;
            fs::write(output, bytes)?;
            println!("Emitted {} CDSP TLB maintenance bytes at {:#x}..{:#x} (disjoint firmware fragment).",
                tlb::REGION_SIZE,tlb::FILE_OFFSET,tlb::FILE_OFFSET+tlb::REGION_SIZE);
        }
        [action, input] if action == "verify-tlb-maintenance" => {
            let image=fs::read(input)?;
            tlb::verify_tlb_maintenance(&image)?;
            println!("Verified {} CDSP TLB maintenance bytes at {:#x}..{:#x} (disjoint fragment).",
                tlb::REGION_SIZE,tlb::FILE_OFFSET,tlb::FILE_OFFSET+tlb::REGION_SIZE);
        }
        [action, output] if action == "emit-relocator" => {
            let data=privileged::build_relocator_trampoline()?;
            fs::write(output,data)?;
            println!("Generated {} CDSP Hexagon relocator bytes at file {:#x}..{:#x}",
                privileged::RELOCATOR_SIZE,privileged::RELOCATOR_OFFSET,
                privileged::RELOCATOR_OFFSET+privileged::RELOCATOR_SIZE);
        }
        [action, input] if action == "verify-relocator" => {
            let reference=fs::read(input)?;
            privileged::verify_relocator_trampoline(&reference)?;
            println!("Verified {} Hexagon relocator bytes at file {:#x}..{:#x}",
                privileged::RELOCATOR_SIZE,privileged::RELOCATOR_OFFSET,
                privileged::RELOCATOR_OFFSET+privileged::RELOCATOR_SIZE);
        }
        [action, output] if action == "emit-supervisor-span" => {
            let data=privileged::build_supervisor_span()?;
            fs::write(output,data)?;
            println!("Generated {} contiguous CDSP Hexagon supervisor bytes, file {:#x}..{:#x}",
                privileged::SUPERVISOR_SPAN_SIZE,privileged::FILE_OFFSET,
                privileged::FILE_OFFSET+privileged::SUPERVISOR_SPAN_SIZE);
        }
        [action, input] if action == "verify-supervisor-span" => {
            let reference=fs::read(input)?;
            privileged::verify_supervisor_span(&reference)?;
            println!("Verified {} CDSP supervisor bytes, file {:#x}..{:#x}",
                privileged::SUPERVISOR_SPAN_SIZE,privileged::FILE_OFFSET,
                privileged::FILE_OFFSET+privileged::SUPERVISOR_SPAN_SIZE);
        }
        [action, output] if action == "emit-syscfg-disable" => {
            let bytes = privileged::build_syscfg_disable()?;
            fs::write(output, bytes)?;
            println!("Emitted {} separately reconstructed CDSP SYSCFG-disable bytes, file {:#x}..{:#x}.",
                privileged::DISABLE_SIZE,privileged::DISABLE_OFFSET,
                privileged::DISABLE_OFFSET+privileged::DISABLE_SIZE);
        }
        [action, original] if action == "verify-syscfg-disable" => {
            let image=fs::read(original)?;
            privileged::verify_syscfg_disable(&image)?;
            println!("Verified {} disjoint CDSP SYSCFG-disable function bytes at file {:#x}..{:#x}.",
                privileged::DISABLE_SIZE,privileged::DISABLE_OFFSET,
                privileged::DISABLE_OFFSET+privileged::DISABLE_SIZE);
        }
        [action, output] if action == "emit-init-prefix" => {
            let bytes = cdsp::build_init_prefix()?;
            fs::write(output, bytes)?;
            println!("Emitted {} independently encoded CDSP initialization bytes, file offsets {:#x}..{:#x}; partial firmware only.",
                cdsp::INIT_PREFIX_LEN, cdsp::START_FILE_OFFSET, cdsp::START_FILE_OFFSET + cdsp::INIT_PREFIX_LEN);
        }
        [action, original] if action == "verify-init-prefix" => {
            let image = fs::read(original)?;
            cdsp::verify_init_prefix(&image)?;
            println!("Verified {} CDSP Hexagon entry/privileged packets at file offsets {:#x}..{:#x}; V66 privilege instructions decoded separately.",
                cdsp::INIT_PREFIX_LEN, cdsp::START_FILE_OFFSET, cdsp::START_FILE_OFFSET + cdsp::INIT_PREFIX_LEN);
        }
        [action, original] if action == "verify-entry" => {
            let image = fs::read(original)?;
            cdsp::verify_reference(&image)?;
            println!("Verified {} CDSP Hexagon instruction bytes at file offsets {:#x}..{:#x} against private reference; remaining CDSP executable not rebuilt.",
                cdsp::REBUILT_LEN, cdsp::START_FILE_OFFSET, cdsp::START_FILE_OFFSET + cdsp::REBUILT_LEN);
        }
        [action, original] if action == "inspect" => {
            let image = fs::read(original)?;
            cdsp::validate_reference(&image)?;
            println!("CDSP machine={} ELF flags={:#x} entry physical={:#x}; first executable PT_LOAD file offset={:#x}, vaddr={:#x}, paddr={:#x}, filesz={:#x}.",
                cdsp::HEXAGON_ELF_MACHINE, cdsp::CDSP_ELF_FLAGS, cdsp::ENTRY_PHYS,
                cdsp::START_FILE_OFFSET, cdsp::ENTRY_ELF_VADDR, cdsp::ENTRY_PHYS, cdsp::FIRST_EXECUTABLE_SIZE);
        }
        _ => return Err("Usage: cargo run -p firmware-tools --bin cdsp-rebuild -- [inspect REFERENCE | verify-entry REFERENCE | emit-entry OUTPUT | verify-init-prefix REFERENCE | emit-init-prefix OUTPUT | emit-syscfg-reset OUTPUT | verify-syscfg-reset REFERENCE | emit-syscfg-disable OUTPUT | verify-syscfg-disable REFERENCE | emit-relocator OUTPUT | verify-relocator REFERENCE | emit-supervisor-span OUTPUT | verify-supervisor-span REFERENCE | emit-tlb-maintenance OUTPUT | verify-tlb-maintenance REFERENCE | emit-tlb-full OUTPUT | verify-tlb-full REFERENCE]".into()),
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("CDSP: {error}");
        std::process::exit(1);
    }
}
