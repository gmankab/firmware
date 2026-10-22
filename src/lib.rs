// SPDX-License-Identifier: GPL-3.0-only
//! Restricted, source-level reconstruction of the *text configuration* modemuw.jsn.
//! This is not a reconstruction of the proprietary executable firmware.
use std::fmt::Write;

#[derive(Copy, Clone)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}
#[derive(Copy, Clone)]
struct Domain {
    soc: &'static str,
    domain: &'static str,
    subdomain: &'static str,
    qmi_instance_id: u32,
}
#[derive(Copy, Clone)]
struct Service {
    provider: &'static str,
    service: &'static str,
    service_data_valid: u32,
    service_data: u32,
}

// Semantic values read from the postmarketOS 1-r2 configuration.
// Their presence does not establish ownership of vendor firmware copyrights.
const VERSION: Version = Version { major: 1, minor: 1, patch: 1 };
const DOMAIN: Domain = Domain {
    soc: "msm", domain: "modem", subdomain: "wlan_pd", qmi_instance_id: 180,
};
const SERVICES: &[Service] = &[
    Service { provider: "kernel", service: "elf_loader", service_data_valid: 0, service_data: 0 },
    Service { provider: "tms", service: "servreg", service_data_valid: 0, service_data: 0 },
    Service { provider: "wlan", service: "fw", service_data_valid: 0, service_data: 0 },
];

fn json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{0000}'..='\u{001f}' => write!(out, "\\u{:04x}", ch as u32).unwrap(),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

/// Render using the original config's space indentation, key order and no final newline.
/// This generates the config from typed values and formatting logic, without embedding
/// any opaque firmware blob or reading source firmware at build time.
pub fn build_modemuw() -> String {
    let mut out = String::new();
    writeln!(out, "{{").unwrap();
    writeln!(out, "    \"sr_version\": {{").unwrap();
    writeln!(out, "        \"major\": {},", VERSION.major).unwrap();
    writeln!(out, "        \"minor\": {},", VERSION.minor).unwrap();
    writeln!(out, "        \"patch\": {}", VERSION.patch).unwrap();
    writeln!(out, "    }},").unwrap();
    writeln!(out, "    \"sr_domain\": {{").unwrap();
    writeln!(out, "        \"soc\": {},", json_string(DOMAIN.soc)).unwrap();
    writeln!(out, "        \"domain\": {},", json_string(DOMAIN.domain)).unwrap();
    writeln!(out, "        \"subdomain\": {},", json_string(DOMAIN.subdomain)).unwrap();
    writeln!(out, "        \"qmi_instance_id\": {}", DOMAIN.qmi_instance_id).unwrap();
    writeln!(out, "    }},").unwrap();
    writeln!(out, "    \"sr_service\": [").unwrap();
    for (index, service) in SERVICES.iter().enumerate() {
        writeln!(out, "        {{").unwrap();
        writeln!(out, "            \"provider\": {},", json_string(service.provider)).unwrap();
        writeln!(out, "            \"service\": {},", json_string(service.service)).unwrap();
        writeln!(out, "            \"service_data_valid\": {},", service.service_data_valid).unwrap();
        writeln!(out, "            \"service_data\": {}", service.service_data).unwrap();
        writeln!(out, "        }}{}", if index + 1 < SERVICES.len() { "," } else { "" }).unwrap();
    }
    write!(out, "    ]\n}}").unwrap();
    out
}

/// Parse only the bounded ELF32 little-endian header and segment table.
/// Neither this metadata inspector nor its output is an executable decompiler.
#[derive(Debug, PartialEq, Eq)]
pub struct ElfHeader {
    pub machine: u16,
    pub entry: u32,
    pub program_headers: Vec<ProgramHeader>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ProgramHeader {
    pub kind: u32,
    pub offset: u32,
    pub vaddr: u32,
    pub paddr: u32,
    pub file_size: u32,
    pub mem_size: u32,
    pub flags: u32,
}
fn read_u16(b: &[u8], i: usize) -> Result<u16, String> {
    let x = b.get(i..i.checked_add(2).ok_or("offset overflow")?).ok_or("truncated ELF")?;
    Ok(u16::from_le_bytes(x.try_into().unwrap()))
}
fn read_u32(b: &[u8], i: usize) -> Result<u32, String> {
    let x = b.get(i..i.checked_add(4).ok_or("offset overflow")?).ok_or("truncated ELF")?;
    Ok(u32::from_le_bytes(x.try_into().unwrap()))
}
pub fn parse_elf32(b: &[u8]) -> Result<ElfHeader, String> {
    if b.get(..4) != Some(&b"\x7fELF"[..]) { return Err("not ELF".into()); }
    if b.get(4) != Some(&1) || b.get(5) != Some(&1) {
        return Err("only little-endian ELF32 supported".into());
    }
    if b.len() < 52 { return Err("truncated ELF header".into()); }
    let machine = read_u16(b, 18)?;
    let entry = read_u32(b, 24)?;
    let phoff = read_u32(b, 28)? as usize;
    let phentsize = read_u16(b, 42)? as usize;
    let phnum = read_u16(b, 44)? as usize;
    if phnum > 0 && phentsize < 32 { return Err("program-header record too short".into()); }
    let mut program_headers = Vec::with_capacity(phnum);
    for n in 0..phnum {
        let at = phoff.checked_add(n.checked_mul(phentsize).ok_or("index overflow")?)
            .ok_or("offset overflow")?;
        let _ = b.get(at..at.checked_add(phentsize).ok_or("record overflow")?)
            .ok_or("truncated program-header table")?;
        program_headers.push(ProgramHeader {
            kind: read_u32(b, at)?, offset: read_u32(b, at+4)?,
            vaddr: read_u32(b, at+8)?, paddr: read_u32(b, at+12)?,
            file_size: read_u32(b, at+16)?, mem_size: read_u32(b, at+20)?,
            flags: read_u32(b, at+24)?,
        });
    }
    Ok(ElfHeader { machine, entry, program_headers })
}

pub fn machine_name(machine: u16) -> &'static str {
    match machine { 40 => "ARM", 164 => "Qualcomm Hexagon DSP", _ => "other" }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bytes_and_layout() {
        let data = build_modemuw();
        assert_eq!(data.len(), 713);
        assert!(data.starts_with("{\n    \"sr_version\":"));
        assert!(data.ends_with("    ]\n}"));
        assert!(!data.ends_with('\n'));
    }
    #[test]
    fn escapes_json_strings() {
        assert_eq!(json_string("a\"\\\n"), "\"a\\\"\\\\\\n\"");
    }
    #[test]
    fn rejects_truncated_elf() {
        assert!(parse_elf32(b"\x7fELF").is_err());
        assert!(parse_elf32(b"not an ELF").is_err());
    }
}
