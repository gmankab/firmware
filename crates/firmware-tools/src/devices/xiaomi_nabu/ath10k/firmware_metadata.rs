// SPDX-License-Identifier: GPL-3.0-only
//! Typed, reference-independent reconstruction of the Nabu ath10k API-v5
//! firmware-5.bin. This 60-byte file contains *metadata only*, not WLAN code.
const MAGIC: &[u8] = b"QCA-ATH10K\0";
const ALIGNMENT: u8 = 0x77;
const TIMESTAMP: u32 = 0x5bbee4a4;

fn element(out: &mut Vec<u8>, kind: u32, value: &[u8]) {
    out.extend_from_slice(&kind.to_le_bytes());
    out.extend_from_slice(&(value.len() as u32).to_le_bytes());
    out.extend_from_slice(value);
    while out.len() % 4 != 0 { out.push(ALIGNMENT); }
}

pub fn rebuild() -> Vec<u8> {
    let mut data = MAGIC.to_vec();
    while data.len() % 4 != 0 { data.push(ALIGNMENT); }
    element(&mut data, 1, &TIMESTAMP.to_le_bytes());
    // Feature bits 6, 18 and 19, encoded as three bytes.
    element(&mut data, 2, &[0x40, 0x00, 0x0c]);
    element(&mut data, 5, &4u32.to_le_bytes());
    element(&mut data, 6, &3u32.to_le_bytes());
    assert_eq!(data.len(), 60);
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metadata_field_layout() {
        let d = rebuild();
        assert_eq!(d.len(), 60);
        assert_eq!(&d[..11], MAGIC);
        assert_eq!(d[11], ALIGNMENT);
        assert_eq!(&d[32..35], &[0x40, 0, 0x0c]);
        assert_eq!(d[35], ALIGNMENT);
        assert_eq!(u32::from_le_bytes(d[48..52].try_into().unwrap()), 6);
    }
}
