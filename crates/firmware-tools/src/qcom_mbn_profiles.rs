// SPDX-License-Identifier: GPL-3.0-only
//! Source-described Qualcomm MBNv6 unsigned header/policy for Xiaomi Nabu.
//!
//! These values describe image IDs, SoC compatibility and digest-table
//! geometry. They are **signed fields** but *not* signatures, keys, or vendor
//! executable instructions. The original OEM authentication remains missing
//! until supplied separately by the image owner.
use crate::{elf32_prefix::Elf32Spec, image_plan::ImagePlan,
    elf_file_coverage::checked_elf_header_plan,
    qcom_mbn::{V6Header,V6Metadata120,SHA384_SIZE,V6_HEADER_SIZE}};

/// Reference-specific pointer **fields**, not seek offsets or host pointers.
/// Some images use 0xffffffff, others physical ROM/authentication addresses.
#[derive(Debug, Clone, Copy)]
pub struct MbnImagePolicy {
    pub filename: &'static str,
    pub image_id: u32,
    pub software_id: u32,
    pub oem_auth_pointer_field: u32,
    pub anti_rollback: u32,
}

pub const KNOWN_MBN_POLICIES: [MbnImagePolicy;6]=[
    MbnImagePolicy{ filename:"a640_zap.mbn", image_id:0, software_id:20,
        oem_auth_pointer_field:u32::MAX, anti_rollback:1 },
    MbnImagePolicy{ filename:"adsp.mbn", image_id:12, software_id:4,
        oem_auth_pointer_field:0x8e00_0208, anti_rollback:1 },
    MbnImagePolicy{ filename:"cdsp.mbn", image_id:12, software_id:23,
        oem_auth_pointer_field:0x9800_01a8, anti_rollback:1 },
    MbnImagePolicy{ filename:"modem.mbn", image_id:12, software_id:2,
        oem_auth_pointer_field:0x9100_0328, anti_rollback:1 },
    MbnImagePolicy{ filename:"venus.mbn", image_id:0, software_id:14,
        oem_auth_pointer_field:0x0fa0_00c8, anti_rollback:1 },
    MbnImagePolicy{ filename:"wlanmdsp.mbn", image_id:12, software_id:18,
        oem_auth_pointer_field:0xb07e_2108, anti_rollback:0 },
];

/// Construct the *unsigned*, deterministic MBNv6 header and compatibility
/// policy. Hashes are intentionally withheld until every hashed file-backed
/// segment is independently generated; OEM authentication is always external.
pub fn unsigned_header_and_policy(spec:&Elf32Spec)
    -> Result<(V6Header,V6Metadata120),String>
{
    let policy=KNOWN_MBN_POLICIES.iter()
        .find(|v|v.filename==spec.name).ok_or("unknown Xiaomi Nabu MBN profile")?;
    let mut hash_iter=spec.headers.iter()
        .filter(|h|h.flags & 0x0700_0000==0x0200_0000);
    let hash=hash_iter.next().ok_or("missing MBNv6 hash program header")?;
    if hash_iter.next().is_some(){return Err("multiple MBNv6 hash headers".into());}
    if spec.headers.is_empty() || hash.offset==0 {
        return Err("missing or misplaced MBNv6 hash segment".into());
    }
    let hash_size=spec.headers.len().checked_mul(SHA384_SIZE)
        .ok_or("hash count overflow")?;
    let signed_components=hash_size.checked_add(0x100+0x1800)
        .ok_or("MBN payload size overflow")?;
    let complete_segment_length=V6_HEADER_SIZE+V6Metadata120::SIZE+signed_components;
    if complete_segment_length!=hash.filesz as usize {
        return Err(format!("{}: MBNv6 hash-segment size contradicts header policy",spec.name));
    }
    let header=V6Header {
        image_id:policy.image_id,version:6,qti_signature_size:0,qti_certificate_size:0,
        declared_payload_size:u32::try_from(signed_components)
            .map_err(|_|"MBN payload does not fit u32")?,
        hash_table_size:u32::try_from(hash_size).map_err(|_|"hash size does not fit u32")?,
        oem_signature_pointer:policy.oem_auth_pointer_field,oem_signature_size:0x100,
        oem_certificate_pointer:policy.oem_auth_pointer_field,oem_certificate_size:0x1800,
        qti_metadata_size:0,oem_metadata_size:V6Metadata120::SIZE as u32,
    };
    // Only the six observed public compatibility fields vary among devices.
    // No certificates, signatures or OEM secret material are generated here.
    let mut metadata=V6Metadata120 {
        software_id:policy.software_id,hardware_id:0x0a50e1,oem_id:0x72,
        policy_flags:V6Metadata120::FLAG_USE_SOC_VERSION
            | (1 << V6Metadata120::DEBUG_POLICY_SHIFT),
        anti_rollback_version:policy.anti_rollback,..Default::default()
    };
    metadata.supported_soc_versions[0]=0x6003;
    Ok((header,metadata))
}

/// Produce the original ELF first page plus the 48-byte unsigned MBN header
/// and 120-byte structured policy. Everything else remains a visible hole.
/// Hash table bytes MUST be built later from independent generated segments.
pub fn unsigned_metadata_plan(
    spec:&Elf32Spec,total_file_bytes:usize,
) -> Result<ImagePlan,String> {
    let (mut plan, inventory)=checked_elf_header_plan(spec,total_file_bytes)?;
    let (header,metadata)=unsigned_header_and_policy(spec)?;
    let hash_start=inventory.authentication_file_range.start;
    let ranges=header.layout(inventory.authentication_file_range.len())?;
    if ranges.header != (0..V6_HEADER_SIZE) || ranges.oem_metadata.len()!=V6Metadata120::SIZE {
        return Err("unexpected MBNv6 layout".into());
    }
    plan.place("source-generated MBNv6 unsigned header",hash_start,&header.encode())?;
    plan.place("source-generated signed image policy",hash_start+ranges.oem_metadata.start,
        &metadata.encode())?;
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::xiaomi_nabu::elf_specs::KNOWN_ELFS;
    #[test]
    fn all_six_metadata_plans_have_explicit_hash_and_auth_holes() {
        for spec in KNOWN_ELFS {
            let size=spec.headers.iter().filter(|h|h.filesz>0)
                .map(|h|h.offset as usize+h.filesz as usize).max().unwrap().max(4096);
            let plan=unsigned_metadata_plan(spec,size).unwrap();
            assert_eq!(plan.coverage().generated_bytes,4096+48+120);
            assert_eq!(plan.coverage().missing_bytes,size-4264);
            assert_eq!(plan.coverage().externally_supplied_bytes,0);
            assert!(!plan.coverage().holes.is_empty());
            assert!(plan.build_complete().is_err());
        }
    }
    #[test]
    fn image_profiles_reject_unmatched_headers_and_unknown_names() {
        let a=KNOWN_ELFS[0];
        let mut wrong=a;
        wrong.name="unknown-device.mbn";
        assert!(unsigned_header_and_policy(&wrong).is_err());
        const SHORT:&[crate::elf32_prefix::Elf32Program]=&[
            crate::elf32_prefix::Elf32Program{kind:0,offset:0x1000,
                vaddr:0,paddr:0,filesz:10,memsz:0,flags:0x02000000,align:0x1000},
        ];
        wrong.name="a640_zap.mbn";
        wrong.headers=SHORT;
        assert!(unsigned_header_and_policy(&wrong).unwrap_err().contains("size contradicts"));
    }
}
