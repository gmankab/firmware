// SPDX-License-Identifier: GPL-3.0-only
//! Partially reconstructed A640 security shader for this reference revision.
//! Encodes semantic IR3 operations, never copies OEM instruction bytes.
//! Output fragments are not a complete shader or executable firmware.
use ir3::{Control, Gpr, MovU32Immediate, relative_slots};
use ir3::scalar::{ScalarAlu, ScalarSource, StoreLocalU32};
use ir3::synchronization::{MemoryBarrier, MemoryScope, BarrierFormat};

#[derive(Debug)]
pub struct Fragment {
    /// Offset from the beginning of the embedded shader, in 8-byte slots.
    pub first_slot: usize,
    pub purpose: &'static str,
    pub bytes: Vec<u8>,
}

fn setup_instruction(dst: Gpr, immediate: u32, repeat: u8) -> Result<[u8; 8], String> {
    MovU32Immediate { dst, immediate, repeat }.encode().map_err(str::to_string)
}

/// Typed IR3 operations that have been independently encoded and compared
/// with the A640 image. All unimplemented gaps are intentionally omitted.
pub fn verified_candidates() -> Result<Vec<Fragment>, String> {
    // The shader computes and writes a small security-state record, then
    // clears vector registers. Each operation is encoded using the public ISA.
    let g = |vector, component| Gpr { vector, component };
    let mut first = Vec::new();
    first.extend_from_slice(&ScalarAlu::MultiplyU24 {
        dst: g(0, 3), left: ScalarSource::Register(g(0, 0)),
        right: ScalarSource::Immediate(96), sync: true,
    }.encode().map_err(str::to_string)?);

    let mut startup = Vec::new();
    startup.extend_from_slice(&setup_instruction(Gpr { vector: 0, component: 1 }, 0, 0)?);
    startup.extend_from_slice(&setup_instruction(Gpr { vector: 0, component: 2 }, 6, 0)?);
    startup.extend_from_slice(&setup_instruction(Gpr { vector: 1, component: 0 }, 0, 3)?);
    startup.extend_from_slice(&Control::Nop { repeat: 3, jump_point: false }.encode().map_err(str::to_string)?);

    let mut predicate_compare = Vec::new();
    predicate_compare.extend_from_slice(&ScalarAlu::CompareU32Equal {
        predicate_component: 0,
        left: ScalarSource::Register(g(0, 1)),
        right: ScalarSource::Register(g(0, 2)),
    }.encode().map_err(str::to_string)?);

    let mut branch_and_nops = Vec::new();
    branch_and_nops.extend_from_slice(&Control::Nop { repeat: 5, jump_point: false }.encode().map_err(str::to_string)?);
    branch_and_nops.extend_from_slice(&Control::BranchPredicate {
        delta_slots: relative_slots(7, 12).map_err(str::to_string)?, component: 0, inverted: false,
    }.encode().map_err(str::to_string)?);

    let mut local_record = Vec::new();
    local_record.extend_from_slice(&StoreLocalU32 {
        address: g(0, 3), source: g(1, 0), offset: 0, count: 4,
    }.encode().map_err(str::to_string)?);
    local_record.extend_from_slice(&ScalarAlu::AddU32 {
        dst: g(0, 3), left: ScalarSource::Register(g(0, 3)),
        right: ScalarSource::Immediate(16), scalar_sync: true,
    }.encode().map_err(str::to_string)?);
    local_record.extend_from_slice(&ScalarAlu::AddU32 {
        dst: g(0, 1), left: ScalarSource::Register(g(0, 1)),
        right: ScalarSource::Immediate(1), scalar_sync: false,
    }.encode().map_err(str::to_string)?);

    let mut loop_return = Vec::new();
    loop_return.extend_from_slice(&Control::Jump {
        delta_slots: relative_slots(11, 4).map_err(str::to_string)?,
    }.encode().map_err(str::to_string)?);
    loop_return.extend_from_slice(&Control::Nop { repeat: 5, jump_point: true }.encode().map_err(str::to_string)?);

    // Candidate pre-marker A6xx global barrier. The encoding fields are
    // independently generated, but the missing Mesa discriminator and actual
    // synchronization semantics are unverified. Do not infer boot safety.
    let synchronization = MemoryBarrier {
        scope: MemoryScope { global: true, ..MemoryScope::default() },
        format: BarrierFormat::LegacyA6xxCandidate,
        synchronize: false, scalar_synchronize: false, jump_point: false,
    }.encode().map_err(str::to_string)?;

    let mut ending = Vec::new();
    ending.extend_from_slice(&Control::End.encode().map_err(str::to_string)?);
    ending.extend_from_slice(&Control::Nop { repeat: 0, jump_point: false }.encode().map_err(str::to_string)?);

    Ok(vec![
        Fragment { first_slot: 0, purpose: "local-memory address initialization", bytes: first },
        Fragment { first_slot: 1, purpose: "shader register setup and delay", bytes: startup },
        Fragment { first_slot: 5, purpose: "shader predicate comparison", bytes: predicate_compare },
        Fragment { first_slot: 6, purpose: "predicate branch with delay", bytes: branch_and_nops },
        Fragment { first_slot: 8, purpose: "write local security-state record and advance index", bytes: local_record },
        Fragment { first_slot: 11, purpose: "loop backedge and join point", bytes: loop_return },
        Fragment { first_slot: 13, purpose: "zero-initialize 48 vector registers", bytes: ir3::zero_vector_registers(0, 48)? },
        Fragment { first_slot: 61, purpose: "tentative legacy global memory barrier encoding", bytes: synchronization.to_vec() },
        Fragment { first_slot: 62, purpose: "shader termination and padding", bytes: ending },
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragments_do_not_overlap_and_cover_expected_slots() {
        let fragments = verified_candidates().unwrap();
        let mut last = 0;
        let mut total = 0;
        for f in fragments {
            assert!(f.first_slot >= last);
            assert_eq!(f.bytes.len() % 8, 0);
            assert!(!f.purpose.is_empty());
            last = f.first_slot + f.bytes.len() / 8;
            total += f.bytes.len();
        }
        assert_eq!(last, 64);
        assert_eq!(total, 64 * 8); // exact encoding, but barrier semantics remain provisional
    }
}
