// SPDX-License-Identifier: GPL-3.0-only
//! Strict reference-pinned audit for CS35L41 WMFW v3 HALO program memory.
//! Outputs *counts and hashes only*. No original DSP instructions are exported
//! or copied into this project, and no HALO ISA decoding is claimed.
use firmware_tools::{parse_wmfw,wmfw_halo_pm_transport as pm,
    wmfw_sha256,wmfw_program_patterns,wmfw_halo_pm_oracle,wmfw_halo_memory::HaloMemoryKind};
use std::{error::Error,env,fs,path::Path};

const REFERENCE_SHA: &str="d1bea27e41001024eb3878276292b5bcc0cda4484ffced0880e12c03ec9bac2c";
const PM_REFERENCE_SHA: &str="0edacef35965ea2bd8c709f8cad73fd08714bb47922065d25e3b9ba1bf34c517";

fn main()->Result<(),Box<dyn Error>> {
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=1 && args.len()!=3 {
        return Err("usage: cargo run --example agent15_pm_audit -- /private/cs35l41.wmfw [--json-out /path/report.json]".into());
    }
    if args.len()==3 && args[1]!="--json-out" {
        return Err("use --json-out to save counts/hashes; input is never written".into());
    }
    let input=Path::new(&args[0]);
    if args.len()==3 {
        let output=Path::new(&args[2]);
        if input.canonicalize().ok()==output.canonicalize().ok() && input.exists() {
            return Err("refusing to overwrite the reference WMFW input".into());
        }
    }
    let firmware=fs::read(input)?;
    let hash=wmfw_sha256::hex_digest(&firmware);
    if firmware.len()!=34_236 || hash!=REFERENCE_SHA {
        return Err(format!("wrong CS35L41 firmware revision or corrupted WMFW: len={}, SHA-256={hash}",firmware.len()).into());
    }
    let container=parse_wmfw(&firmware)?;
    if container.region_headers.len()!=11 || container.version!=3 || container.core!=4 {
        return Err("unexpected HALO WMFW container metadata".into());
    }
    let region=container.region_headers.iter().find(|r|r.region_type==0x10)
        .ok_or("missing HALO PM packed region")?;
    if region.file_offset+8 != pm::PM_PAYLOAD_FILE_START ||
        region.payload_length != pm::PM_PAYLOAD_BYTES || region.dsp_offset!=0 {
        return Err("PM region bounds or DSP offset differ from pinned WMFW layout".into());
    }
    let pm_bytes=firmware.get(pm::PM_PAYLOAD_FILE_START..pm::PM_PAYLOAD_FILE_END)
        .ok_or("truncated PM payload")?;
    let pm_hash=wmfw_sha256::hex_digest(pm_bytes);
    if pm_hash!=PM_REFERENCE_SHA {return Err("PM payload digest differs from pinned firmware".into());}
    let writes=pm::host_writes_for_blocks(0,pm_bytes)?;
    let recreated=pm::blocks_from_host_writes(0,&writes)?;
    if recreated!=pm_bytes {return Err("regmap host register sequence changed PM bytes".into());}
    if writes.len()!=5_070 || writes.first().unwrap().register_address!=0x0380_0000 ||
        writes.last().unwrap().register_address!=0x0380_4f34 {
        return Err("unexpected PM host-write count or bounds".into());
    }
    let source_words=firmware_tools::wmfw_halo_memory::unpack_words(HaloMemoryKind::PackedProgram,pm_bytes)?;
    if source_words.len()!=pm::PM_WORD_COUNT {return Err("PM 40-bit word count differs".into());}
    let mut zero_words=0usize;
    for (block_index,chunk) in pm_bytes.chunks_exact(pm::PM_BLOCK_BYTES).enumerate() {
        let block=pm::PackedPmBlock::from_slice(chunk)?;
        let roundtripped=pm::PackedPmBlock::from_words(block.words())?;
        if roundtripped!=block || pm::PackedPmBlock::from_host_registers(block.host_registers())!=block {
            return Err(format!("HALO PM transport mismatch in block {block_index}").into());
        }
        if block.words().iter().any(|&w|w>pm::PM_MAX_WORD) {return Err("PM word exceeds 40 bits".into());}
        zero_words+=block.words().into_iter().filter(|&w|w==0).count();
    }
    if zero_words!=29 {return Err("PM zero-word inventory differs from pinned reference".into());}
    let patterns=wmfw_program_patterns::analyze_packed_program(pm_bytes)?;
    if patterns.distinct_words!=3_701 || patterns.all_zero_words!=29 || patterns.total_words!=4_056 {
        return Err("PM pattern inventory differs from pinned reference".into());
    }
    // No independent HALO assembler evidence has been provided. This verified
    // zero-source-coverage ledger must not be upgraded by transport matching.
    let verified=wmfw_halo_pm_oracle::compare_candidates(pm_bytes,&[])?;
    if verified.matched_20bit_slots!=0 || verified.fully_covered_storage_words!=0 ||
        verified.first_uncovered_storage_word!=Some(0) ||
        verified.first_uncovered_half!=Some(0) {
        return Err("unexpected HALO source-coverage evidence".into());
    }
    // No opcode labels appear: these are storage words, not decoded code.
    let report=format!(concat!(
        "{{\n",
        "  \"firmware_sha256\": \"{}\",\n",
        "  \"pm_payload_sha256\": \"{}\",\n",
        "  \"pm_file_start\": \"0x278c\",\n",
        "  \"pm_file_end_exclusive\": \"0x76c4\",\n",
        "  \"dsp_pm_address_start\": 0,\n",
        "  \"pm_payload_bytes\": {},\n",
        "  \"40bit_storage_words\": {},\n",
        "  \"20byte_transport_blocks\": {},\n",
        "  \"32bit_host_writes\": {},\n",
        "  \"register_base\": \"0x03800000\",\n",
        "  \"register_end_exclusive\": \"0x03804f38\",\n",
        "  \"distinct_storage_words\": {},\n",
        "  \"all_zero_storage_words\": {},\n",
        "  \"transport_byte_mismatches\": 0,\n",
        "  \"source_verified_20bit_half_slots\": 0,\n",
        "  \"first_uncovered_pm_word\": 0,\n",
        "  \"first_uncovered_pm_half\": 0,\n",
        "  \"independently_rebuilt_executable_bytes\": 0,\n",
        "  \"unresolved_instruction_isa\": true\n",
        "}}\n"
    ),hash,pm_hash,pm_bytes.len(),source_words.len(),pm::PM_BLOCK_COUNT,writes.len(),patterns.distinct_words,zero_words);
    print!("{report}");
    if args.len()==3 {fs::write(&args[2],report.as_bytes())?;}
    Ok(())
}
