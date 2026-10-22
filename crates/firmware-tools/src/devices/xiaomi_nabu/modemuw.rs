// SPDX-License-Identifier: GPL-3.0-only
//! Xiaomi Nabu WLAN PD text configuration, rendered from typed values.
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_renders_deterministically() {
        let data = build_modemuw();
        assert_eq!(data.len(), 713);
        assert!(data.starts_with("{\n    \"sr_version\":"));
        assert!(!data.ends_with('\n'));
    }
}
