// SPDX-License-Identifier: GPL-3.0-only
//! Declarative **metadata-only** construction of the first 4 KiB of an ELF32 image.
//!
//! The output is NOT a firmware, contains no executable code or cryptographic
//! signatures, and has not been authenticated or tested on a device.

pub const ELF_PAGE_LEN: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub struct Elf32Program {
    pub kind: u32,
    pub offset: u32,
    pub vaddr: u32,
    pub paddr: u32,
    pub filesz: u32,
    pub memsz: u32,
    pub flags: u32,
    pub align: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Elf32Spec {
    pub name: &'static str,
    pub machine: u16,
    pub entry: u32,
    pub flags: u32,
    pub section_header_size: u16,
    pub headers: &'static [Elf32Program],
}

fn put16(out: &mut [u8], start: usize, n: u16) {
    out[start..start + 2].copy_from_slice(&n.to_le_bytes());
}
fn put32(out: &mut [u8], start: usize, n: u32) {
    out[start..start + 4].copy_from_slice(&n.to_le_bytes());
}

pub fn write_elf32_first_page(spec: &Elf32Spec) -> Result<[u8; ELF_PAGE_LEN], String> {
    let mut out = [0u8; ELF_PAGE_LEN];
    if 52usize + spec.headers.len() * 32 > out.len() {
        return Err("ELF program headers do not fit into first page".into());
    }
    if spec.headers.len() > u16::MAX as usize {
        return Err("ELF program-header count too large".into());
    }
    out[0..4].copy_from_slice(b"\x7fELF");
    out[4] = 1; // ELFCLASS32
    out[5] = 1; // ELFDATA2LSB
    out[6] = 1; // EV_CURRENT
    put16(&mut out, 16, 2); // ET_EXEC
    put16(&mut out, 18, spec.machine);
    put32(&mut out, 20, 1); // EV_CURRENT
    put32(&mut out, 24, spec.entry);
    put32(&mut out, 28, 52); // e_phoff
    put32(&mut out, 32, 0); // e_shoff
    put32(&mut out, 36, spec.flags);
    put16(&mut out, 40, 52); // e_ehsize
    put16(&mut out, 42, 32); // e_phentsize
    put16(&mut out, 44, spec.headers.len() as u16);
    put16(&mut out, 46, spec.section_header_size);
    put16(&mut out, 48, 0); // no sections
    put16(&mut out, 50, 0); // no shstrtab
    for (i, h) in spec.headers.iter().enumerate() {
        let off = 52 + i * 32;
        for (j, val) in [h.kind, h.offset, h.vaddr, h.paddr,
                         h.filesz, h.memsz, h.flags, h.align].iter().enumerate() {
            put32(&mut out, off + j * 4, *val);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_prefix_format() {
        let template = Elf32Spec { name: "test.mbn", machine: 164,
            entry: 0x5000, flags: 3, section_header_size: 0,
            headers: &[Elf32Program {kind: 1, offset: 0x1000, vaddr: 0x5000,
                paddr: 0x5000, filesz: 4, memsz: 8, flags: 7, align: 4096}] };
        let b = write_elf32_first_page(&template).unwrap();
        assert_eq!(&b[0..7], b"\x7fELF\x01\x01\x01");
        assert_eq!(u32::from_le_bytes(b[24..28].try_into().unwrap()), 0x5000);
        assert_eq!(u16::from_le_bytes(b[44..46].try_into().unwrap()), 1);
        assert!(b[84..].iter().all(|x| *x == 0));
    }
}

#[cfg(test)]
mod known_elf_specs_tests {
    use super::*;
    use crate::{elf_specs::KNOWN_ELFS, parse_elf32};

    #[test]
    fn all_six_generated_prefixes_roundtrip() {
        assert_eq!(KNOWN_ELFS.len(), 6);
        for spec in KNOWN_ELFS {
            let bytes = write_elf32_first_page(spec).unwrap();
            let parsed = parse_elf32(&bytes).unwrap();
            assert_eq!(parsed.machine, spec.machine, "{}", spec.name);
            assert_eq!(parsed.entry, spec.entry, "{}", spec.name);
            assert_eq!(parsed.program_headers.len(), spec.headers.len(), "{}", spec.name);
            for (actual, wanted) in parsed.program_headers.iter().zip(spec.headers) {
                assert_eq!(actual.kind, wanted.kind);
                assert_eq!(actual.offset, wanted.offset);
                assert_eq!(actual.file_size, wanted.filesz);
                assert_eq!(actual.mem_size, wanted.memsz);
                assert_eq!(actual.flags, wanted.flags);
            }
        }
    }
}
