// SPDX-License-Identifier: GPL-3.0-only
//! Reference-independent WMDR writer for the four Nabu speaker profiles.
//! These files are coefficient/configuration packages, not HALO DSP executables.
//! Numeric OEM coefficients are re-encoded, not independently recalibrated.
#[path = "tuning_coefficients.rs"]
mod coeffs;
use coeffs::{BL_BASE, BR_OVERRIDES, TL_OVERRIDES, TR_OVERRIDES};

pub const IMAGE_SIZE: usize = 5_340;
const YM_PACKED_SIZE: usize = 4_428;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Speaker { Bl, Br, Tl, Tr }
impl Speaker {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "BL" => Ok(Self::Bl), "BR" => Ok(Self::Br),
            "TL" => Ok(Self::Tl), "TR" => Ok(Self::Tr),
            _ => Err(format!("unknown speaker {value:?}; expected BL, BR, TL or TR")),
        }
    }
    pub fn as_str(self) -> &'static str {
        match self { Self::Bl => "BL", Self::Br => "BR", Self::Tl => "TL", Self::Tr => "TR" }
    }
    fn metadata_label(self) -> &'static str {
        match self {
            Self::Bl => "BL_SPKC", Self::Br => "BR_SPKD",
            Self::Tl => "TL_SPKA", Self::Tr => "TR_SPKB",
        }
    }
    fn overrides(self) -> &'static [(u16,u32)] {
        match self {
            Self::Bl => &[], Self::Br => BR_OVERRIDES,
            Self::Tl => TL_OVERRIDES, Self::Tr => TR_OVERRIDES,
        }
    }
    fn threshold(self) -> u32 {
        match self { Self::Bl | Self::Tr => 0x800000, Self::Br | Self::Tl => 0x7fffff }
    }
}

fn append_item(output: &mut Vec<u8>, offset: u16, kind: u16, alg: u32, version: u32, data: &[u8]) {
    output.extend_from_slice(&offset.to_le_bytes());
    output.extend_from_slice(&kind.to_le_bytes());
    output.extend_from_slice(&alg.to_le_bytes());
    output.extend_from_slice(&version.to_le_bytes());
    output.extend_from_slice(&0u32.to_le_bytes());
    output.extend_from_slice(&(data.len() as u32).to_le_bytes());
    output.extend_from_slice(data);
    while output.len() % 4 != 0 { output.push(0); }
}

fn ym_coefficients(speaker: Speaker) -> Vec<u8> {
    let mut words = BL_BASE;
    for &(index, word) in speaker.overrides() {
        assert!(word <= 0x00ff_ffff);
        words[index as usize] = word;
    }
    let mut output=Vec::with_capacity(YM_PACKED_SIZE);
    for word in words {
        assert!(word <= 0x00ff_ffff);
        output.extend_from_slice(&word.to_le_bytes()[..3]);
    }
    assert_eq!(output.len(), YM_PACKED_SIZE);
    output
}

pub fn rebuild(speaker: Speaker) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(IMAGE_SIZE);
    bytes.extend_from_slice(b"WMDR");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&0x0021_0001u32.to_be_bytes());
    bytes.extend_from_slice(&0x0000_0304u32.to_be_bytes());
    let name = format!(
        r"D:\XiaoMi's project\K82_P2\Bin\Actisonic128_PDL_in_DSP\{}-cs35l41-dsp1-spk-prot.bin",
        speaker.as_str()
    );
    assert_eq!(name.len(), 83);
    append_item(&mut bytes, 0, 0xfe00, 0, 0, name.as_bytes());
    append_item(&mut bytes, 0x0134, 0x12, 0xcd, 0x1d35_0000, &ym_coefficients(speaker));
    let mut thresholds=Vec::with_capacity(12);
    for word in [0x20u32, 0x0008_0000, speaker.threshold()] {
        thresholds.extend_from_slice(&word.to_be_bytes());
    }
    append_item(&mut bytes, 0x18a8, 0x06, 0xcd, 0x1d35_0000, &thresholds);
    const TEMPLATE: &str = include_str!("tuning_metadata.json");
    assert_eq!(TEMPLATE.matches("BL_SPKC").count(), 1);
    let mut json=TEMPLATE.replace("BL_SPKC",speaker.metadata_label()).into_bytes();
    json.push(0);
    assert_eq!(json.len(), 718);
    append_item(&mut bytes, 0, 0xfc00, 0, 0, &json);
    assert_eq!(bytes.len(), IMAGE_SIZE);
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_speakers_encode_structurally_complete_wmdr() {
        for name in ["BL","BR","TL","TR"] {
            let speaker=Speaker::parse(name).unwrap();
            let bytes=rebuild(speaker);
            assert_eq!(bytes.len(), IMAGE_SIZE);
            assert_eq!(&bytes[..4], b"WMDR");
            assert_eq!(u32::from_le_bytes(bytes[0x88..0x8c].try_into().unwrap()), 4428);
            assert_eq!(u32::from_le_bytes(bytes[0x1208..0x120c].try_into().unwrap()), 718);
            assert_eq!(&bytes[0x14da..], &[0,0]);
            assert!(bytes.windows(speaker.metadata_label().len())
                         .any(|window| window == speaker.metadata_label().as_bytes()));
        }
    }
    #[test]
    fn profile_specific_values_are_preserved() {
        assert!(Speaker::parse("XX").is_err());
        assert_ne!(rebuild(Speaker::Bl), rebuild(Speaker::Br));
        assert_ne!(rebuild(Speaker::Tl), rebuild(Speaker::Tr));
    }
}
