// SPDX-License-Identifier: GPL-3.0-only
//! Sparse source fragments for integration, with explicit holes and byte classes.
//! No original firmware, opaque calibration bytes, or guessed gap fill is read.

use super::{novatek_upper_ex9 as ex9, novatek_upper_ilm as ilm,
    novatek_upper_log_data as logarithm, novatek_upper_math_data as math,
    novatek_upper_allocations as allocations, novatek_upper_boundary as boundary};

pub const GENERATED_BYTES: usize = 39_640;
pub const UNRESOLVED_BYTES: usize = 4_520;

#[derive(Debug)]
pub struct Fragment {
    pub name: &'static str,
    pub file: u32,
    pub bytes: Vec<u8>,
    pub instructions: usize,
    pub executable: usize,
    pub data: usize,
    pub fill: usize,
}

impl Fragment {
    pub fn end(&self) -> u32 { self.file + self.bytes.len() as u32 }
    pub fn filename(&self) -> String {
        format!("agent20-18-{:05x}-{:05x}.bin", self.file, self.end())
    }
}

pub fn fragments() -> Result<Vec<Fragment>, String> {
    let mut all = vec![
        Fragment { name: "source-rebuilt upper-ILM ownership seam (MOVI suffix plus six instructions)",
            file: boundary::FILE_START, bytes: boundary::emit()?,
            instructions: boundary::INSTRUCTION_COUNT, executable: boundary::OWNED_BYTES, data: 0, fill: 0 },
        Fragment { name: "upper routines and switch tables", file: ilm::SOURCE_START,
            bytes: ilm::emit()?, instructions: ilm::CODE.len(),
            executable: ilm::EXECUTABLE_BYTES, data: ilm::SWITCH_DATA_BYTES,
            fill: ilm::ALIGNMENT_BYTES },
        Fragment { name: "EX9 physical instruction table", file: ex9::SOURCE_START as u32,
            bytes: ex9::emit()?, instructions: ex9::CODE.len(),
            executable: (ex9::SOURCE_END - ex9::SOURCE_START) as usize, data: 0, fill: 0 },
    ];
    all.push(Fragment {
        name: "reference-decoded controller allocation addresses (measured origins, checked strides)",
        file: allocations::FILE_START, bytes: allocations::emit()?,
        instructions: 0, executable: 0, data: allocations::GENERATED_BYTES, fill: 0,
    });
    for (file, bytes) in math::emit()? {
        let fill = if file >= math::SINE_FILE { 2 } else { 0 };
        let data = bytes.len() - fill;
        all.push(Fragment { name: "mathematically constructed scalar table", file,
            bytes, instructions: 0, executable: 0, data, fill });
    }
    all.push(Fragment { name: "Arm binary64 logarithm constants", file: logarithm::SOURCE_START as u32,
        bytes: logarithm::emit(), instructions: 0, executable: 0,
        data: logarithm::SOURCE_END - logarithm::SOURCE_START, fill: 0 });
    all.sort_by_key(|fragment| fragment.file);
    validate(&all)?;
    Ok(all)
}

pub fn validate(all: &[Fragment]) -> Result<(), String> {
    let mut end = ilm::RESERVED_START;
    for fragment in all {
        if fragment.bytes.is_empty() || fragment.file < end || fragment.end() > ilm::RESERVED_END {
            return Err(format!("empty, overlapping or out-of-reservation fragment at {:#x}", fragment.file));
        }
        if fragment.executable + fragment.data + fragment.fill != fragment.bytes.len() {
            return Err(format!("byte classes do not sum to the fragment at {:#x}", fragment.file));
        }
        end = fragment.end();
    }
    Ok(())
}

pub fn gaps(all: &[Fragment]) -> Result<Vec<(u32, u32)>, String> {
    validate(all)?;
    let mut previous = ilm::RESERVED_START;
    let mut gaps = Vec::new();
    for fragment in all {
        if previous < fragment.file { gaps.push((previous, fragment.file)); }
        previous = fragment.end();
    }
    if previous < ilm::RESERVED_END { gaps.push((previous, ilm::RESERVED_END)); }
    Ok(gaps)
}

pub fn manifest(all: &[Fragment]) -> Result<String, String> {
    validate(all)?;
    let mut out = format!(concat!(
        "{{\n  \"schema\": \"agent20-18.source-fragments.v1\",\n",
        "  \"reserved_start\": {},\n  \"reserved_end\": {},\n",
        "  \"complete_image\": false,\n  \"checkpoint50_integration\": false,\n",
        "  \"covered_bytes\": {},\n  \"executable_bytes\": {},\n",
        "  \"data_bytes\": {},\n  \"layout_fill_bytes\": {},\n  \"fragments\": [\n"),
        ilm::RESERVED_START, ilm::RESERVED_END, all.iter().map(|f| f.bytes.len()).sum::<usize>(),
        all.iter().map(|f| f.executable).sum::<usize>(), all.iter().map(|f| f.data).sum::<usize>(),
        all.iter().map(|f| f.fill).sum::<usize>());
    for (index, fragment) in all.iter().enumerate() {
        out.push_str(&format!(concat!(
            "    {{\"name\": \"{}\", \"file\": \"{}\", \"file_start\": {}, \"file_end\": {}, ",
            "\"runtime_start\": {}, \"runtime_end\": {}, \"instructions\": {}, ",
            "\"executable_bytes\": {}, \"data_bytes\": {}, \"layout_fill_bytes\": {}}}{}\n"),
            fragment.name, fragment.filename(), fragment.file, fragment.end(),
            fragment.file-ilm::ILM_FILE_BIAS, fragment.end()-ilm::ILM_FILE_BIAS,
            fragment.instructions, fragment.executable, fragment.data, fragment.fill,
            if index+1 == all.len() { "" } else { "," }));
    }
    out.push_str("  ],\n  \"unresolved\": [");
    for (index, (start, end)) in gaps(all)?.into_iter().enumerate() {
        out.push_str(&format!("{}{{\"file_start\": {start}, \"file_end\": {end}, \"bytes\": {}}}",
            if index == 0 { "" } else { ", " }, end-start));
    }
    out.push_str("]\n}\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_union_does_not_count_any_hole_as_padding() {
        let f = fragments().unwrap();
        assert_eq!(f.iter().map(|f| f.bytes.len()).sum::<usize>(), GENERATED_BYTES);
        assert_eq!(f.iter().map(|f| f.executable).sum::<usize>(), 34_172);
        assert_eq!(f.iter().map(|f| f.data).sum::<usize>(), 5_459);
        assert_eq!(f.iter().map(|f| f.fill).sum::<usize>(), 9);
        let holes = gaps(&f).unwrap();
        assert_eq!(holes, vec![(0x18620,0x195b0),
            (0x196f8,0x198dc), (0x19bbc,0x19bf0)]);
        assert_eq!(holes.iter().map(|(s,e)| (e-s) as usize).sum::<usize>(), UNRESOLVED_BYTES);
        assert_eq!(GENERATED_BYTES + UNRESOLVED_BYTES, (ilm::RESERVED_END-ilm::RESERVED_START) as usize);
    }

    #[test]
    fn overlapping_or_misclassified_fragments_are_rejected() {
        let mut f = fragments().unwrap();
        f[1].file = f[0].file;
        assert!(validate(&f).is_err());
        let mut f = fragments().unwrap();
        f[0].fill += 1;
        assert!(validate(&f).is_err());
    }

    #[test]
    fn ex9_references_resolve_and_upper_jump_targets_land_on_instructions() {
        use super::super::novatek_upper_isa::Instruction;
        for &(_, instruction) in ilm::CODE {
            if let Instruction::Ex9It(index) = instruction {
                assert!(ex9::CODE.get(usize::from(index)).is_some());
            }
        }
        let mut internal = 0;
        for instruction in ex9::CODE {
            let target = match instruction {
                Instruction::Ex9J(runtime) | Instruction::Ex9Jal(runtime) =>
                    runtime.checked_add(ilm::ILM_FILE_BIAS).unwrap(),
                _ => continue,
            };
            if (ilm::SOURCE_START..ilm::SOURCE_END).contains(&target) {
                assert!(ilm::CODE.iter().any(|(file, _)| *file == target));
                internal += 1;
            }
        }
        assert_eq!(internal, 25);
    }
}
