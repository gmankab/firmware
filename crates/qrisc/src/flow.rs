// SPDX-License-Identifier: GPL-3.0-only
//! Conservative static control-flow checks for the publicly described A6xx
//! QRISC ISA. This models *possible* edges, not execution, and does not
//! implement GPU register behavior or encrypted/trusted operations.
use crate::codec::QriscWord;
use std::collections::BTreeSet;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub struct FlowSummary {
    /// Sorted executable word indices potentially reachable from `entry`.
    pub reachable_words: Vec<usize>,
    pub branch_targets: Vec<usize>,
    pub call_targets: Vec<usize>,
    pub returns: usize,
}

/// Follow reachable QRISC branch and call edges, skipping an explicit IR3
/// data range. A conditional branch explores both possible successors.
///
/// Relative branch displacements are word-index relative. Call destinations
/// use absolute word indices. The low 25 bits of `ret` are not interpreted;
/// a return is treated as ending the present control-flow path.
///
/// This validates an instruction graph, not its device behavior or copyright
/// provenance. No reference firmware is needed for unit tests.
pub fn inspect(stream: &[u8], entry: usize, shader: Option<Range<usize>>)
    -> Result<FlowSummary, String>
{
    if stream.is_empty() || stream.len() % 4 != 0 {
        return Err("QRISC byte stream must be nonempty whole words".into());
    }
    let words = stream.len() / 4;
    if let Some(range) = &shader {
        if range.start >= range.end || range.end > words {
            return Err("invalid embedded shader word range".into());
        }
    }
    let mut seen = BTreeSet::new();
    let mut pending = vec![entry];
    let mut branches = BTreeSet::new();
    let mut calls = BTreeSet::new();
    let mut returns = 0;
    while let Some(pc) = pending.pop() {
        if pc >= words {
            return Err(format!("control flow exits microcode at word {pc}"));
        }
        if shader.as_ref().is_some_and(|s| s.contains(&pc)) {
            return Err(format!("QRISC control flow enters IR3 shader at word {pc}"));
        }
        if !seen.insert(pc) { continue; }
        let raw = u32::from_le_bytes(stream[pc * 4..pc * 4 + 4].try_into().unwrap());
        let instruction = QriscWord::decode(raw)
            .ok_or_else(|| format!("unrecognized reachable QRISC instruction at word {pc}"))?;
        match instruction {
            QriscWord::Branch { opcode, src, condition, delta } => {
                let destination = i64::try_from(pc).map_err(|_| "word index too large")?
                    + i64::from(delta);
                let target = usize::try_from(destination)
                    .map_err(|_| format!("negative branch target from word {pc}"))?;
                branches.insert(target);
                pending.push(target);
                // The documented unconditional jump is brneb r0,b0.
                // All other forms may also fall through.
                if !(opcode == 0x32 && src == 0 && condition == 0) {
                    pending.push(pc + 1);
                }
            }
            QriscWord::Call { target } => {
                let target = target as usize;
                calls.insert(target);
                pending.push(target);
                // Conservatively explore the return site as well.
                pending.push(pc + 1);
            }
            QriscWord::Return { .. } | QriscWord::InterruptReturn { .. } => returns += 1,
            _ => pending.push(pc + 1),
        }
    }
    Ok(FlowSummary {
        reachable_words: seen.into_iter().collect(),
        branch_targets: branches.into_iter().collect(),
        call_targets: calls.into_iter().collect(),
        returns,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{codec::QriscWord, program::{assemble, Item}};

    #[test]
    fn follows_both_conditional_paths_but_skips_data() {
        let program = [
            Item::Instr(QriscWord::Nop { metadata: 0 }),
            Item::BranchTo { opcode: 0x31, src: 2, condition: 1, label: "done" },
            Item::Instr(QriscWord::MovImmediate { repeat: false, dst: 4, shift: 0, imm: 3 }),
            Item::Label("done"),
            Item::Instr(QriscWord::Return { ignored_bits: 0 }),
            Item::ShaderSlot { low: 0, high: 0 },
        ];
        let bytes = assemble(&program).unwrap();
        let result = inspect(&bytes, 0, Some(4..6)).unwrap();
        assert_eq!(result.reachable_words, [0, 1, 2, 3]);
        assert_eq!(result.branch_targets, [3]);
        assert_eq!(result.returns, 1);
    }

    #[test]
    fn unconditional_branch_skips_shader_and_still_returns() {
        let p = [
            Item::BranchTo { opcode: 0x32, src: 0, condition: 0, label: "code" },
            Item::ShaderSlot { low: 0xff, high: 0xff },
            Item::Label("code"),
            Item::Instr(QriscWord::Return { ignored_bits: 0 }),
        ];
        let flow = inspect(&assemble(&p).unwrap(), 0, Some(1..3)).unwrap();
        assert_eq!(flow.reachable_words, [0, 3]);
        assert_eq!(flow.branch_targets, [3]);
    }

    #[test]
    fn rejects_branch_into_shader() {
        let p = [
            Item::BranchTo { opcode: 0x32, src: 0, condition: 0, label: "data" },
            Item::Label("data"),
            Item::ShaderSlot { low: 0, high: 0 },
        ];
        assert!(inspect(&assemble(&p).unwrap(), 0, Some(1..3)).unwrap_err().contains("shader"));
    }

    #[test]
    fn rejects_out_of_range_target_and_opaque_instructions() {
        let escape = QriscWord::Branch { opcode: 0x32, src: 0, condition: 0, delta: -1 };
        assert!(inspect(&escape.encode().unwrap().to_le_bytes(), 0, None)
            .unwrap_err().contains("negative branch"));
        assert!(inspect(&u32::MAX.to_le_bytes(), 0, None)
            .unwrap_err().contains("unrecognized"));
        assert!(inspect(&[], 0, None).is_err());
    }

    #[test]
    fn calls_add_a_destination_and_return_site() {
        let p = [
            Item::CallTo { label: "callee" },
            Item::Instr(QriscWord::Return { ignored_bits: 0 }),
            Item::Label("callee"),
            Item::Instr(QriscWord::Return { ignored_bits: 0 }),
        ];
        let flow = inspect(&assemble(&p).unwrap(), 0, None).unwrap();
        assert_eq!(flow.call_targets, [2]);
        assert_eq!(flow.reachable_words, [0, 1, 2]);
        assert_eq!(flow.returns, 2);
    }
}
