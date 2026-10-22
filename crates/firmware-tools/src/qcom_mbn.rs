// SPDX-License-Identifier: GPL-3.0-only
//! Qualcomm MBN version 6 hash-segment metadata and SHA-384 hash tables.
//!
//! The public format describes a header, optional signer metadata, segment
//! hashes, and independently authenticated QTI/OEM signatures and certificates.
//! This module reconstructs only the **unsigned** structural information.
//! It cannot produce or validate a hardware-trusted signature.

use std::ops::Range;

use crate::sha384;

pub const V6_HEADER_SIZE: usize = 12 * 4;
pub const SHA384_SIZE: usize = 48;

/// Public MBNv6 12-word header. Pointer words are preserved as metadata; they
/// are not interpreted as in-file offsets or trusted host memory addresses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V6Header {
    pub image_id: u32,
    pub version: u32,
    pub qti_signature_size: u32,
    pub qti_certificate_size: u32,
    pub declared_payload_size: u32,
    pub hash_table_size: u32,
    pub oem_signature_pointer: u32,
    pub oem_signature_size: u32,
    pub oem_certificate_pointer: u32,
    pub oem_certificate_size: u32,
    pub qti_metadata_size: u32,
    pub oem_metadata_size: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct V6Layout {
    pub header: Range<usize>,
    pub qti_metadata: Range<usize>,
    pub oem_metadata: Range<usize>,
    pub hashes: Range<usize>,
    pub qti_signature: Range<usize>,
    pub qti_certificate: Range<usize>,
    pub oem_signature: Range<usize>,
    pub oem_certificate: Range<usize>,
    pub padding: Range<usize>,
}

impl V6Header {
    pub fn encode(&self) -> [u8; V6_HEADER_SIZE] {
        let fields = [
            self.image_id, self.version, self.qti_signature_size,
            self.qti_certificate_size, self.declared_payload_size,
            self.hash_table_size, self.oem_signature_pointer,
            self.oem_signature_size, self.oem_certificate_pointer,
            self.oem_certificate_size, self.qti_metadata_size,
            self.oem_metadata_size,
        ];
        let mut result = [0u8; V6_HEADER_SIZE];
        for (slot, value) in result.chunks_exact_mut(4).zip(fields) {
            slot.copy_from_slice(&value.to_le_bytes());
        }
        result
    }

    pub fn parse(input: &[u8]) -> Result<Self, String> {
        let bytes = input.get(..V6_HEADER_SIZE).ok_or("truncated MBNv6 header")?;
        let mut fields = [0u32; 12];
        for (destination, chunk) in fields.iter_mut().zip(bytes.chunks_exact(4)) {
            *destination = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        if fields[1] != 6 { return Err(format!("unsupported MBN version {}", fields[1])); }
        Ok(Self {
            image_id: fields[0], version: fields[1],
            qti_signature_size: fields[2], qti_certificate_size: fields[3],
            declared_payload_size: fields[4], hash_table_size: fields[5],
            oem_signature_pointer: fields[6], oem_signature_size: fields[7],
            oem_certificate_pointer: fields[8], oem_certificate_size: fields[9],
            qti_metadata_size: fields[10], oem_metadata_size: fields[11],
        })
    }

    /// Resolve a byte-range layout with overflow, version, and size checking.
    /// A valid layout says nothing about whether any signature is authentic.
    pub fn layout(&self, segment_size: usize) -> Result<V6Layout, String> {
        if self.version != 6 { return Err("expected MBN version 6".into()); }
        if self.hash_table_size == 0 || self.hash_table_size as usize % SHA384_SIZE != 0 {
            return Err("MBNv6 SHA-384 table must contain whole nonempty digests".into());
        }
        let declared = [self.hash_table_size, self.qti_signature_size,
            self.qti_certificate_size, self.oem_signature_size, self.oem_certificate_size]
            .into_iter().try_fold(0usize, |sum, size| {
                sum.checked_add(size as usize).ok_or("MBN payload size overflow")
            })?;
        if declared != self.declared_payload_size as usize {
            return Err("MBNv6 declared payload size disagrees with component sizes".into());
        }
        if segment_size < V6_HEADER_SIZE { return Err("truncated MBN segment".into()); }
        let mut cursor = 0usize;
        let mut take = |length: usize| -> Result<Range<usize>, String> {
            let start = cursor;
            cursor = cursor.checked_add(length).ok_or("MBN segment offset overflow")?;
            if cursor > segment_size { return Err("MBN segment component exceeds bounds".into()); }
            Ok(start..cursor)
        };
        let header = take(V6_HEADER_SIZE)?;
        let qti_metadata = take(self.qti_metadata_size as usize)?;
        let oem_metadata = take(self.oem_metadata_size as usize)?;
        let hashes = take(self.hash_table_size as usize)?;
        let qti_signature = take(self.qti_signature_size as usize)?;
        let qti_certificate = take(self.qti_certificate_size as usize)?;
        let oem_signature = take(self.oem_signature_size as usize)?;
        let oem_certificate = take(self.oem_certificate_size as usize)?;
        Ok(V6Layout {
            header, qti_metadata, oem_metadata, hashes, qti_signature,
            qti_certificate, oem_signature, oem_certificate,
            padding: cursor..segment_size,
        })
    }
}

/// Generate the hash table for file-backed segments, with a zero digest for
/// segments that are deliberately omitted from hashing (such as the MBN hash
/// segment itself). The caller supplies the exact bytes and entry ordering;
/// otherwise the result will not match the target image.
pub fn sha384_table(entries: &[Option<&[u8]>]) -> Result<Vec<u8>, String> {
    if entries.is_empty() { return Err("no MBN hash entries".into()); }
    let capacity = entries.len().checked_mul(SHA384_SIZE)
        .ok_or("MBN hash-table size overflow")?;
    let mut result = Vec::with_capacity(capacity);
    for entry in entries {
        match entry {
            Some(data) => result.extend_from_slice(&sha384::digest(data)),
            None => result.extend_from_slice(&[0u8; SHA384_SIZE]),
        }
    }
    Ok(result)
}

/// MBNv6 120-byte image policy metadata, as documented by public Qualcomm
/// tooling. These fields constrain image identity and hardware compatibility.
/// Reproducing them does not bypass their signed enforcement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct V6Metadata120 {
    pub major_version: u32,
    pub minor_version: u32,
    pub software_id: u32,
    pub hardware_id: u32,
    pub oem_id: u32,
    pub model_id: u32,
    pub application_id: u32,
    pub policy_flags: u32,
    pub supported_soc_versions: [u32; 12],
    pub bound_serial_numbers: [u32; 8],
    pub multi_root_certificate_index: u32,
    pub anti_rollback_version: u32,
}

impl V6Metadata120 {
    pub const SIZE: usize = 120;
    pub const FLAG_USE_SOC_VERSION: u32 = 1 << 1;
    pub const DEBUG_POLICY_SHIFT: u32 = 8;

    pub fn encode(&self) -> [u8; Self::SIZE] {
        let mut result = [0u8; Self::SIZE];
        let words = [
            self.major_version, self.minor_version, self.software_id,
            self.hardware_id, self.oem_id, self.model_id,
            self.application_id, self.policy_flags,
        ];
        let mut offset = 0usize;
        for word in words.iter().chain(self.supported_soc_versions.iter())
            .chain(self.bound_serial_numbers.iter())
            .chain([self.multi_root_certificate_index, self.anti_rollback_version].iter()) {
            result[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
            offset += 4;
        }
        result
    }

    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let bytes = data.get(..Self::SIZE).ok_or("truncated MBNv6 OEM metadata")?;
        let mut words = [0u32; 30];
        for (word, chunk) in words.iter_mut().zip(bytes.chunks_exact(4)) {
            *word = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        Ok(Self {
            major_version: words[0], minor_version: words[1],
            software_id: words[2], hardware_id: words[3],
            oem_id: words[4], model_id: words[5],
            application_id: words[6], policy_flags: words[7],
            supported_soc_versions: words[8..20].try_into().unwrap(),
            bound_serial_numbers: words[20..28].try_into().unwrap(),
            multi_root_certificate_index: words[28],
            anti_rollback_version: words[29],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> V6Header {
        V6Header {
            image_id: 1, version: 6, qti_signature_size: 0,
            qti_certificate_size: 0, declared_payload_size: 48 * 3 + 256 + 512,
            hash_table_size: 48 * 3, oem_signature_pointer: u32::MAX,
            oem_signature_size: 256, oem_certificate_pointer: u32::MAX,
            oem_certificate_size: 512, qti_metadata_size: 4, oem_metadata_size: 8,
        }
    }

    #[test]
    fn header_encode_parse_and_complete_layout() {
        let value = sample();
        assert_eq!(V6Header::parse(&value.encode()).unwrap(), value);
        let len = V6_HEADER_SIZE + 4 + 8 + 144 + 256 + 512 + 16;
        let ranges = value.layout(len).unwrap();
        assert_eq!(ranges.header, 0..48);
        assert_eq!(ranges.qti_metadata, 48..52);
        assert_eq!(ranges.oem_metadata, 52..60);
        assert_eq!(ranges.hashes, 60..204);
        assert_eq!(ranges.qti_signature, 204..204);
        assert_eq!(ranges.qti_certificate, 204..204);
        assert_eq!(ranges.oem_signature, 204..460);
        assert_eq!(ranges.oem_certificate, 460..972);
        assert_eq!(ranges.padding, 972..988);
    }

    #[test]
    fn rejects_inconsistent_malformed_and_truncated_layouts() {
        let base = sample();
        assert!(V6Header::parse(&base.encode()[..47]).is_err());
        let mut wrong_version = base;
        wrong_version.version = 7;
        assert!(V6Header::parse(&wrong_version.encode()).is_err());
        assert!(wrong_version.layout(2000).is_err());
        let mut invalid = base;
        invalid.hash_table_size = 145;
        assert!(invalid.layout(2000).is_err());
        invalid = base;
        invalid.declared_payload_size -= 1;
        assert!(invalid.layout(2000).is_err());
        assert!(base.layout(50).is_err());
    }

    #[test]
    fn oem_metadata_v6_named_fields_round_trip_without_payloads() {
        let mut original = V6Metadata120 {
            software_id: 20, hardware_id: 0xa50e1,
            oem_id: 0x72, policy_flags: V6Metadata120::FLAG_USE_SOC_VERSION | (1 << V6Metadata120::DEBUG_POLICY_SHIFT),
            anti_rollback_version: 1, ..Default::default()
        };
        original.supported_soc_versions[0] = 0x6003;
        assert_eq!(V6Metadata120::parse(&original.encode()).unwrap(), original);
        assert_eq!(original.encode().len(), 120);
        assert_eq!(&original.encode()[0x74..], &1u32.to_le_bytes());
        assert!(V6Metadata120::parse(&original.encode()[..119]).is_err());
    }

    #[test]
    fn sha384_table_preserves_null_entries_and_entry_order() {
        let out = sha384_table(&[Some(b"abc"), None, Some(b"")]).unwrap();
        assert_eq!(out.len(), 3 * SHA384_SIZE);
        assert_eq!(&out[..48], &sha384::digest(b"abc"));
        assert_eq!(&out[48..96], &[0u8; 48]);
        assert_eq!(&out[96..], &sha384::digest(b""));
        assert!(sha384_table(&[]).is_err());
    }
}
