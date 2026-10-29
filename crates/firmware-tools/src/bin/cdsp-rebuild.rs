// SPDX-License-Identifier: GPL-3.0-only
//! CDSP partial reconstruction command. Reference bytes stay outside Git.
use firmware_tools::devices::xiaomi_nabu::{cdsp_boot as cdsp, cdsp_privilege as privileged, cdsp_tlb as tlb, cdsp_tlb_extension as extension, cdsp_copyfast as cache_copy, cdsp_cache_controller as cache_ctl, cdsp_dma_copy as dma_copy};
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_icache_tags as icache_tags,cdsp_early_cache_helpers as cache_helpers};
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_format_guard as format_guard,cdsp_early_format_guard_tail as format_tail,cdsp_early_format_guard_final as format_final};
use std::{env, fs};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    match arguments.as_slice() {
        [action, output] if action=="emit-format-classifier"=>{
            fs::write(output,format_final::build_full_format_classifier()?)?;
            println!("Emitted complete 1016-byte CDSP codec format classifier at file 0x5488..0x5880.");
        }
        [action, reference] if action=="verify-format-classifier"=>{
            format_final::verify_reference(&fs::read(reference)?)?;
            println!("Verified joined 1016-byte CDSP format classifier and all preceding source-built early CDSP fragments.");
        }
        [action, output] if action=="emit-format-guard-final"=>{
            fs::write(output,format_final::build_format_guard_final()?)?;
            println!("Emitted {} CDSP source-built final classifier bytes at file {:#x}..{:#x}.",format_final::FUNCTION_BYTES,format_final::FILE_OFFSET,format_final::REGION_END);
        }
        [action, reference] if action=="verify-format-guard-final"=>{
            format_final::verify_reference(&fs::read(reference)?)?;
            println!("Verified {} CDSP source-built classifier final bytes and earlier linked CDSP regions.",format_final::FUNCTION_BYTES);
        }
        [action, output] if action=="emit-format-guard-tail"=>{
            fs::write(output,format_tail::build_format_guard_continuation()?)?;
            println!("Emitted {} CDSP codec guard continuation bytes, file {:#x}..{:#x}; partial function.",
                format_tail::FUNCTION_BYTES,format_tail::FILE_OFFSET,format_tail::REGION_END);
        }
        [action, reference] if action=="verify-format-guard-tail"=>{
            format_tail::verify_reference(&fs::read(reference)?)?;
            println!("Verified {} format guard continuation bytes and all earlier source-built early CDSP regions.",format_tail::FUNCTION_BYTES);
        }
        [action, output] if action=="emit-format-guard"=>{
            fs::write(output,format_guard::build_format_guard_prefix()?)?;
            println!("Emitted {} source-encoded CDSP format-guard bytes at file {:#x}..{:#x}; partial function.",
               format_guard::FUNCTION_BYTES,format_guard::FILE_OFFSET,format_guard::REGION_END);
        }
        [action, reference] if action=="verify-format-guard"=>{
            format_guard::verify_reference(&fs::read(reference)?)?;
            println!("Verified {} CDSP format-guard bytes and all preceding joined early regions.",
               format_guard::FUNCTION_BYTES);
        }
        [action, output] if action == "emit-cache-helpers" => {
            fs::write(output,cache_helpers::build_cache_helper_group()?)?;
            println!("Emitted {} CDSP source-built helper bytes, file {:#x}..{:#x}.",
                cache_helpers::FUNCTION_BYTES,cache_helpers::FILE_OFFSET,cache_helpers::REGION_END);
        }
        [action, original] if action == "verify-cache-helpers" => {
            cache_helpers::verify_reference(&fs::read(original)?)?;
            println!("Verified {} source-built helper bytes and all preceding joined CDSP regions.",
                cache_helpers::FUNCTION_BYTES);
        }
        [action, output] if action == "emit-icache-tags" => {
            fs::write(output,icache_tags::build_icache_tag_recovery()?)?;
            println!("Emitted {} source-encoded CDSP I-cache tag sweep bytes at {:#x}..{:#x}.",
                icache_tags::FUNCTION_BYTES,icache_tags::FILE_OFFSET,icache_tags::REGION_END);
        }
        [action, original] if action == "verify-icache-tags" => {
            icache_tags::verify_reference(&fs::read(original)?)?;
            println!("Verified {} CDSP I-cache tag sweep bytes and all preceding joined CDSP regions.",
                icache_tags::FUNCTION_BYTES);
        }
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
        [action, output] if action == "emit-dma-copy" => {
            fs::write(output,dma_copy::build_copy()?)?;
            println!("Emitted 64 independently reconstructed CDSP doubleword-copy bytes file 0x41a0..0x41e0.");
        }
        [action, reference] if action == "verify-dma-copy" => {
            dma_copy::verify_copy(&fs::read(reference)?)?;
            println!("Verified 64 CDSP doubleword-copy bytes file 0x41a0..0x41e0.");
        }
        [action, output] if action == "emit-cluster-with-dma-copy" => {
            fs::write(output,dma_copy::build_cluster_with_dma_copy()?)?;
            println!("Emitted 1184 contiguous CDSP bytes including 12 ELF alignment bytes; 1172 are reconstructed instructions.");
        }
        [action, reference] if action == "verify-cluster-with-dma-copy" => {
            dma_copy::verify_cluster_with_dma_copy(&fs::read(reference)?)?;
            println!("Verified CDSP 1184-byte supervisor/TLB/cache/DMA span including 12 alignment bytes.");
        }
        [action, output] if action == "emit-remaining-controller-descriptors" => {
            fs::write(output,cache_ctl::build_remaining_descriptors_and_return()?)?;
            println!("Emitted typed 316-byte CDSP controller descriptors plus SYSCFG return, file 0x4058..0x4194.");
        }
        [action, reference] if action == "verify-remaining-controller-descriptors" => {
            cache_ctl::verify_remaining_descriptors(&fs::read(reference)?)?;
            println!("Verified 316 new CDSP controller descriptor bytes, file 0x4058..0x4194.");
        }
        [action, output] if action == "emit-cluster-with-controller-return" => {
            fs::write(output,cache_ctl::build_joined_complete_cache_controller()?)?;
            println!("Emitted 1108 contiguous CDSP supervisor/TLB/cache/controller bytes file 0x3d40..0x4194.");
        }
        [action, reference] if action == "verify-cluster-with-controller-return" => {
            cache_ctl::verify_complete_cache_controller(&fs::read(reference)?)?;
            println!("Verified 1108 contiguous CDSP supervisor/TLB/cache/controller bytes file 0x3d40..0x4194.");
        }
        [action, output] if action == "emit-first-controller-descriptor" => {
            fs::write(output,cache_ctl::build_first_controller_descriptor()?)?;
            println!("Generated 108 checked Hexagon descriptor bytes at CDSP file 0x3fec..0x4058.");
        }
        [action, reference] if action == "verify-first-controller-descriptor" => {
            cache_ctl::verify_first_descriptor(&fs::read(reference)?)?;
            println!("Verified CDSP first controller descriptor, 108 bytes, file 0x3fec..0x4058.");
        }
        [action, output] if action == "emit-cluster-with-first-descriptor" => {
            fs::write(output,cache_ctl::build_joined_first_descriptor()?)?;
            println!("Generated 792 contiguous CDSP supervisor/TLB/cache/descriptor bytes 0x3d40..0x4058.");
        }
        [action, reference] if action == "verify-cluster-with-first-descriptor" => {
            cache_ctl::verify_joined_first_descriptor(&fs::read(reference)?)?;
            println!("Verified 792 contiguous CDSP supervisor/TLB/cache/descriptor bytes 0x3d40..0x4058.");
        }
        [action, output] if action == "emit-controller-table-clear" => {
            fs::write(output,cache_ctl::build_controller_table_clear()?)?;
            println!("Emitted independently encoded 32-byte CDSP table-zero loop, file 0x3fcc..0x3fec.");
        }
        [action, reference] if action == "verify-controller-table-clear" => {
            cache_ctl::verify_table_clear(&fs::read(reference)?)?;
            println!("Verified 32 CDSP controller table-clear bytes, file 0x3fcc..0x3fec.");
        }
        [action, output] if action == "emit-cluster-with-table-clear" => {
            fs::write(output,cache_ctl::build_joined_cache_and_table_clear()?)?;
            println!("Emitted 684 contiguous CDSP supervisor/TLB/cache/table-clear bytes 0x3d40..0x3fec.");
        }
        [action, reference] if action == "verify-cluster-with-table-clear" => {
            cache_ctl::verify_joined_table_clear(&fs::read(reference)?)?;
            println!("Verified 684 contiguous CDSP bytes 0x3d40..0x3fec.");
        }
        [action, output] if action == "emit-cache-maintenance" => {
            fs::write(output,cache_ctl::build_cache_maintenance()?)?;
            println!("Emitted 88 source-rebuilt CDSP cache maintenance bytes file 0x3f74..0x3fcc.");
        }
        [action, reference] if action == "verify-cache-maintenance" => {
            cache_ctl::verify_cache_maintenance(&fs::read(reference)?)?;
            println!("Verified 88 CDSP cache maintenance bytes file 0x3f74..0x3fcc.");
        }
        [action, output] if action == "emit-cluster-with-cache-maintenance" => {
            fs::write(output,cache_ctl::build_joined_cache_cluster()?)?;
            println!("Emitted 652 contiguous supervisor/TLB/cache-config bytes at file 0x3d40..0x3fcc.");
        }
        [action, reference] if action == "verify-cluster-with-cache-maintenance" => {
            cache_ctl::verify_joined_cache_cluster(&fs::read(reference)?)?;
            println!("Verified 652 contiguous CDSP supervisor/TLB/cache-config bytes.");
        }
        [action, output] if action == "emit-cache-controller-prefix" => {
            fs::write(output,cache_ctl::build_prefix()?)?;
            println!("Emitted typed 80-byte CFGBASE setup prefix at CDSP file 0x3f24..0x3f74; routine incomplete.");
        }
        [action, reference] if action == "verify-cache-controller-prefix" => {
            cache_ctl::verify_prefix(&fs::read(reference)?)?;
            println!("Verified 80 reconstructed CDSP CFGBASE setup bytes at 0x3f24..0x3f74.");
        }
        [action, output] if action == "emit-cluster-with-cache-controller" => {
            fs::write(output,cache_ctl::build_joined_cache_controller_prefix()?)?;
            println!("Emitted 564 contiguous reconstructed CDSP supervisor/TLB/cache setup bytes, file 0x3d40..0x3f74.");
        }
        [action, reference] if action == "verify-cluster-with-cache-controller" => {
            cache_ctl::verify_joined_prefix(&fs::read(reference)?)?;
            println!("Verified 564 contiguous CDSP bytes file 0x3d40..0x3f74.");
        }
        [action, output] if action == "emit-copyfast" => {
            fs::write(output,cache_copy::build_complete()?)?;
            println!("Emitted 108 source-rebuilt Hexagon fast-copy bytes at file 0x3eb8..0x3f24.");
        }
        [action, reference] if action == "verify-copyfast" => {
            cache_copy::verify_complete(&fs::read(reference)?)?;
            println!("Verified 108 Hexagon fast-copy bytes against private CDSP image.");
        }
        [action, output] if action == "emit-cluster-with-copyfast" => {
            fs::write(output,cache_copy::build_cluster_with_copyfast()?)?;
            println!("Emitted contiguous 484-byte CDSP supervisor/TLB/cache cluster at 0x3d40..0x3f24.");
        }
        [action, reference] if action == "verify-cluster-with-copyfast" => {
            cache_copy::verify_full_cluster(&fs::read(reference)?)?;
            println!("Verified contiguous 484-byte supervisor/TLB/cache cluster.");
        }
        [action, output] if action == "emit-copyfast-prefix" => {
            fs::write(output,cache_copy::build_prefix()?)?;
            println!("Emitted 32 typed Hexagon cache-copy entry bytes at file 0x3eb8..0x3ed8; body incomplete.");
        }
        [action, original] if action == "verify-copyfast-prefix" => {
            cache_copy::verify_prefix(&fs::read(original)?)?;
            println!("Verified 32 CDSP cache-copy bytes against the original reference.");
        }
        [action, output] if action == "emit-tlb-tables" => {
            fs::write(output,extension::build_tlb_table_walks()?)?;
            println!("Emitted 64 independently generated CDSP TLB table-walk bytes at file 0x3e78..0x3eb8.");
        }
        [action, output] if action == "emit-tlb-extended" => {
            fs::write(output, extension::build_extended_cluster()?)?;
            println!("Emitted {} independently reconstructed CDSP supervisor/TLB bytes at 0x3d40–0x3eb8; entry is separate.",
                tlb::COMPLETE_CLUSTER_SIZE+extension::TOTAL_BYTES);
        }
        [action, reference] if action == "verify-tlb-extended" => {
            extension::verify_extended_cluster(&fs::read(reference)?)?;
            println!("Verified {} contiguous Hexagon V66 supervisor/TLB bytes.",
                tlb::COMPLETE_CLUSTER_SIZE+extension::TOTAL_BYTES);
        }
        [action, output] if action == "emit-tlb-cluster" => {
            let code=tlb::build_complete_tlb_cluster()?;
            fs::write(output,code)?;
            println!("Generated {} contiguous supervisor/TLB bytes file {:#x}..{:#x}.",
                tlb::COMPLETE_CLUSTER_SIZE,tlb::COMPLETE_CLUSTER_OFFSET,
                tlb::COMPLETE_CLUSTER_OFFSET+tlb::COMPLETE_CLUSTER_SIZE);
        }
        [action, input] if action == "verify-tlb-cluster" => {
            let ref_bytes=fs::read(input)?;
            tlb::verify_complete_tlb_cluster(&ref_bytes)?;
            println!("Verified {} contiguous CDSP supervisor/TLB bytes.",tlb::COMPLETE_CLUSTER_SIZE);
        }
        [action, output] if action == "emit-tlb-walk" => {
            let code=tlb::build_tlb_walk()?;
            fs::write(output,code)?;
            println!("Generated {} CDSP TLB walk bytes file {:#x}..{:#x}.",
                tlb::WALK_SIZE,tlb::WALK_OFFSET,tlb::WALK_OFFSET+tlb::WALK_SIZE);
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
        _ => return Err("Usage: cargo run -p firmware-tools --bin cdsp-rebuild -- [inspect REFERENCE | verify-entry REFERENCE | emit-entry OUTPUT | verify-init-prefix REFERENCE | emit-init-prefix OUTPUT | emit-syscfg-reset OUTPUT | verify-syscfg-reset REFERENCE | emit-syscfg-disable OUTPUT | verify-syscfg-disable REFERENCE | emit-relocator OUTPUT | verify-relocator REFERENCE | emit-supervisor-span OUTPUT | verify-supervisor-span REFERENCE | emit-tlb-maintenance OUTPUT | verify-tlb-maintenance REFERENCE | emit-tlb-full OUTPUT | verify-tlb-full REFERENCE | emit-tlb-cluster OUTPUT | verify-tlb-cluster REFERENCE | emit-tlb-walk OUTPUT]".into()),
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("CDSP: {error}");
        std::process::exit(1);
    }
}
