// SPDX-License-Identifier: GPL-3.0-only
//! Agent 20-19 standalone data emitter and envelope/provenance verifier.
//! Auto-discovered Cargo bin; no central module or CLI registration required.
#![allow(dead_code)]
#[path = "../devices/xiaomi_nabu/novatek_layout.rs"] mod novatek_layout;
#[path = "../devices/xiaomi_nabu/novatek_vector.rs"] mod novatek_vector;
#[path = "../devices/xiaomi_nabu/novatek_crc.rs"] mod novatek_crc;
#[path = "../devices/xiaomi_nabu/novatek_nds32.rs"] mod novatek_nds32;
#[path = "../devices/xiaomi_nabu/novatek_dlm.rs"] mod novatek_dlm;
#[path = "../devices/xiaomi_nabu/novatek_dlm_gaussian.rs"] mod novatek_dlm_gaussian;
#[path = "../devices/xiaomi_nabu/novatek_dlm_schedule_links.rs"] mod novatek_dlm_schedule_links;
#[path = "../devices/xiaomi_nabu/novatek_dlm_state_defaults.rs"] mod novatek_dlm_state_defaults;
#[path = "../devices/xiaomi_nabu/novatek_dlm_bank_transform.rs"] mod novatek_dlm_bank_transform;
#[path = "../devices/xiaomi_nabu/novatek_dlm_control_candidates.rs"] mod novatek_dlm_control_candidates;
#[path = "../devices/xiaomi_nabu/novatek_dlm_calibration_alias.rs"] mod novatek_dlm_calibration_alias;
#[path = "../devices/xiaomi_nabu/novatek_dlm_bank_aliases.rs"] mod novatek_dlm_bank_aliases;
#[path = "../devices/xiaomi_nabu/novatek_dlm_envelope.rs"] mod novatek_dlm_envelope;
#[path = "../devices/xiaomi_nabu/novatek_dlm_provenance.rs"] mod novatek_dlm_provenance;

use firmware_tools::wmfw_sha256::hex_digest;
use novatek_dlm::DataFragment;
use novatek_dlm_provenance::{build_ledger, DlmClaim, ProvenanceLedger};
use std::{env,fs,path::Path,process};

const REFERENCE_SHA256: &str = "8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5";

fn generated_fragments() -> Result<Vec<DataFragment>,String> {
    let mut parts=novatek_dlm::fragments()?;
    for kernel in novatek_dlm_gaussian::fragments()? {
        parts.push(DataFragment { name:"gaussian-half-kernel", file_start:kernel.file_start,
            bytes:kernel.bytes, evidence:kernel.evidence });
    }
    parts.extend(novatek_dlm_schedule_links::fragments()?);
    parts.extend(novatek_dlm_state_defaults::fragments()?);
    parts.sort_by_key(|part|part.file_start);
    // Build the same ledger used in reports to reject gaps in classification
    // and overlapping or out-of-scope source claims before any output write.
    ledger(&parts)?;
    Ok(parts)
}

fn ledger(parts:&[DataFragment]) -> Result<ProvenanceLedger,String> {
    build_ledger(&parts.iter().map(|part|DlmClaim {
        file_start:part.file_start,file_end:part.file_end(),evidence:part.evidence,
    }).collect::<Vec<_>>())
}

fn json_string(value:&str)->String {
    let mut out=String::from("\"");
    for c in value.chars() {
        match c {
            '\\'=>out.push_str("\\\\"),'"'=>out.push_str("\\\""),
            '\n'=>out.push_str("\\n"),'\r'=>out.push_str("\\r"),'\t'=>out.push_str("\\t"),
            c if c<' '=>out.push_str(&format!("\\u{:04x}",c as u32)),
            c=>out.push(c),
        }
    }
    out.push('"');out
}

fn file_name(part:&DataFragment)->String {
    format!("novatek-dlm-{:06x}-{:06x}.bin",part.file_start,part.file_end())
}

fn report(parts:&[DataFragment],verified:bool)->Result<String,String> {
    let accounting=ledger(parts)?;
    let t=accounting.totals;
    let mut out=format!(concat!("{{\n  \"schema\": \"agent20-19-novatek-dlm-v1\",\n",
        "  \"profile\": \"postmarketos-nabu\",\n  \"reference_sha256\": \"{}\",\n",
        "  \"reference_verified\": {},\n  \"flashable\": false,\n",
        "  \"source_layout_bytes\": {},\n  \"prior_source_executable_bytes\": {},\n",
        "  \"new_executable_bytes\": 0,\n  \"source_dlm_data_bytes\": {},\n",
        "  \"external_metadata_bytes\": {},\n  \"unknown_ilm_bytes\": {},\n",
        "  \"unknown_dlm_bytes\": {},\n  \"derived_mirror_bytes\": {},\n",
        "  \"mirror_from_source_bytes\": {},\n  \"mirror_from_external_bytes\": {},\n",
        "  \"independent_unique_bytes\": {},\n  \"total_bytes\": {},\n",
        "  \"first_unknown_dlm\": {},\n  \"crc_stored_fields_checked\": {},\n",
        "  \"mismatched_source_bytes\": {},\n  \"fragments\": [\n"),
        REFERENCE_SHA256,verified,t.source_layout_bytes,t.source_executable_bytes,
        t.source_dlm_data_bytes,t.external_metadata_bytes,t.unknown_ilm_bytes,t.unknown_dlm_bytes,
        t.derived_mirror_bytes,t.mirror_from_source_bytes,t.mirror_from_external_bytes,
        t.independent_bytes(),t.total_bytes(),
        accounting.first_unknown_dlm.map(|n|n.to_string()).unwrap_or_else(||"null".into()),
        if verified {12} else {0},if verified {"0"} else {"null"});
    for (i,part) in parts.iter().enumerate() {
        if i>0 {out.push_str(",\n");}
        out.push_str(&format!(concat!("    {{\"name\": {}, \"file\": {}, \"file_start\": {}, ",
            "\"file_end\": {}, \"sram_start\": {}, \"bytes\": {}, \"sha256\": \"{}\", \"evidence\": {}}}"),
            json_string(part.name),json_string(&file_name(part)),part.file_start,part.file_end(),
            part.sram_start(),part.bytes.len(),hex_digest(&part.bytes),json_string(part.evidence)));
    }
    out.push_str("\n  ],\n  \"regions\": [\n");
    for (i,row) in accounting.rows.iter().enumerate() {
        if i>0 {out.push_str(",\n");}
        out.push_str(&format!("    {{\"file_start\": {}, \"file_end\": {}, \"kind\": {}, \"evidence\": {}",
            row.file_start,row.file_end,json_string(row.kind.as_str()),json_string(row.evidence)));
        if let Some(source)=row.mirror_source {
            out.push_str(&format!(", \"source_file_start\": {}, \"source_file_end\": {}, \"source_independent\": {}",
                source.file_start,source.file_end,source.independently_authored));
        }
        out.push('}');
    }
    out.push_str("\n  ]\n}\n");Ok(out)
}

fn verify_reference(image:&[u8],parts:&[DataFragment])->Result<(),String> {
    let actual=hex_digest(image);
    if actual!=REFERENCE_SHA256 {return Err(format!("reference SHA-256 mismatch: {actual}"));}
    novatek_dlm_envelope::verify_envelope(image)?;
    // Validation of a reference-assisted cross-bank transform does not promote
    // any of the dependent descriptor bytes to independently authored source.
    novatek_dlm_bank_transform::verify_against_supplied_reference(image)?;
    novatek_dlm_calibration_alias::verify_reference_assisted_copy(image)?;
    novatek_dlm_bank_aliases::verify_reference_assisted_aliases(image)?;
    novatek_dlm_bank_aliases::assert_uncredited_on_ledger(&ledger(parts)?)?;
    for part in parts {
        let reference=image.get(part.file_start..part.file_end()).ok_or("DLM fragment outside reference")?;
        if let Some(index)=part.bytes.iter().zip(reference).position(|(a,b)|a!=b) {
            return Err(format!("DLM source mismatch at file 0x{:x}",part.file_start+index));
        }
    }
    Ok(())
}

fn run()->Result<(),String> {
    let args:Vec<_>=env::args().skip(1).collect();
    let command=args.first().map(String::as_str);
    match command {
        Some("ledger") if args.len()==1 => print!("{}",report(&generated_fragments()?,false)?),
        Some("verify-reference") if args.len()==2 => {
            let image=fs::read(&args[1]).map_err(|e|e.to_string())?;
            let parts=generated_fragments()?;
            verify_reference(&image,&parts)?;
            print!("{}",report(&parts,true)?);
        }
        Some("verify-bank-pairs") if args.len()==2 => {
            let image=fs::read(&args[1]).map_err(|e|e.to_string())?;
            if hex_digest(&image)!=REFERENCE_SHA256 {return Err("reference SHA-256 mismatch".into());}
            let count=novatek_dlm_bank_transform::verify_against_supplied_reference(&image)?;
            println!("Verified {count} reference-dependent DLM descriptor bytes in two bank pairs; zero new independent source-byte credit");
        }
        Some("inspect-control-candidates") if args.len()==2 => {
            let image=fs::read(&args[1]).map_err(|e|e.to_string())?;
            if hex_digest(&image)!=REFERENCE_SHA256 {return Err("reference SHA-256 mismatch".into());}
            let (mirrored,classified)=novatek_dlm_control_candidates::verify_reference_assisted_candidates(&image)?;
            println!("Two candidate 40-bit recurrences match {mirrored} plus {classified} reference bytes; transferred fields show 0x190 strides and selectors. ZERO new source-authored DLM credit; the controller operation bits and payload semantics remain unproven.");
        }
        Some("verify-calibration-alias") if args.len()==2 => {
            let image=fs::read(&args[1]).map_err(|e|e.to_string())?;
            if hex_digest(&image)!=REFERENCE_SHA256 {return Err("reference SHA-256 mismatch".into());}
            let bytes=novatek_dlm_calibration_alias::verify_reference_assisted_copy(&image)?;
            let primary=image.get(novatek_dlm_calibration_alias::PRIMARY_FILE_START..
                novatek_dlm_calibration_alias::PRIMARY_FILE_END).ok_or("missing primary calibration")?;
            let headers=novatek_dlm_calibration_alias::observed_headers(primary)?;
            println!("Verified {bytes} dependent calibration bytes at 0x{:x}–0x{:x}; profile headers {:?}. Source credit remains ZERO until original values are independently reconstructed.",
                novatek_dlm_calibration_alias::MIRROR_FILE_START,
                novatek_dlm_calibration_alias::MIRROR_FILE_END,headers);
        }
        Some("verify-bank-aliases") if args.len()==2 => {
            let image=fs::read(&args[1]).map_err(|e|e.to_string())?;
            let bytes=novatek_dlm_bank_aliases::verify_reference_assisted_aliases(&image)?;
            novatek_dlm_bank_aliases::assert_uncredited_on_ledger(&ledger(&generated_fragments()?)?)?;
            println!("Verified {bytes} dependent DLM bank-copy bytes using externally supplied source; independent source credit remains ZERO");
        }
        Some("check-envelope") if args.len()==2 => {
            let image=fs::read(&args[1]).map_err(|e|e.to_string())?;
            let a=novatek_dlm_envelope::verify_envelope(&image)?;
            println!("Consistent postmarketOS envelope: {} CRC fields, {} candidate vectors; payload authorship and hardware behavior are not established by this check.",a.crc.stored_fields(),a.vector_slots);
        }
        Some("emit") if args.len()==2 => {
            let parts=generated_fragments()?;
            let directory=Path::new(&args[1]);
            fs::create_dir_all(directory).map_err(|e|e.to_string())?;
            for part in &parts {
                fs::write(directory.join(file_name(part)),&part.bytes).map_err(|e|e.to_string())?;
            }
            let summary=report(&parts,false)?;
            fs::write(directory.join("manifest.json"),summary).map_err(|e|e.to_string())?;
            println!("Emitted {} data fragments ({} bytes), with explicit holes retained in the provenance manifest.",
                parts.len(),parts.iter().map(|part|part.bytes.len()).sum::<usize>());
        }
        _=>return Err("usage: novatek-dlm ledger | emit <directory> | verify-reference <pinned-image.bin> | check-envelope <image.bin> | verify-bank-pairs <pinned-image.bin> | verify-calibration-alias <pinned-image.bin> | verify-bank-aliases <pinned-image.bin> | inspect-control-candidates <pinned-image.bin>".into()),
    }
    Ok(())
}

fn main(){if let Err(error)=run(){eprintln!("novatek-dlm: {error}");process::exit(1);}}

#[cfg(test)]
mod integration_tests {
    use super::*;
    #[test]
    fn every_generated_span_is_dlm_data_and_mirror_credit_is_not_duplicated() {
        let parts=generated_fragments().unwrap();
        let t=ledger(&parts).unwrap().totals;
        assert_eq!(parts.len(),69);
        assert_eq!(t.source_dlm_data_bytes,9353);
        assert_eq!(t.mirror_from_source_bytes,648);
        assert_eq!(t.mirror_from_external_bytes,1656);
        assert_eq!(t.unknown_dlm_bytes,40199);
        assert_eq!(t.unknown_payload_bytes(),148253);
        assert_eq!(t.total_bytes(),196608);
        assert_eq!(t.independent_bytes(),45027);
        let r=report(&parts,false).unwrap();
        assert!(r.contains("\"mismatched_source_bytes\": null"));
        assert!(r.contains("\"reference_verified\": false"));
    }
    #[test]
    fn reference_mode_rejects_unpinned_input_before_legacy_parsers() {
        assert!(verify_reference(&[],&generated_fragments().unwrap()).unwrap_err().contains("SHA-256 mismatch"));
    }
}
