// SPDX-License-Identifier: GPL-3.0-only
//! Descriptor release, resource updates, and runtime interface stubs.
use arm32::{
    bx, data_immediate, data_register, mov_register, push_registers,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

// Signed address adjustment recorded as a single constant/pointer field,
// not an instruction: this value is part of the firmware's resource ABI.
const SERVICE_DESCRIPTOR_BIAS: u32 = 0xfdff_feb1;

fn descriptor_shutdown(c: &mut Code<'_>) -> Result<(), String> {
    let r4 = R::new(4)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(load(R::R0, R::R0, 12)?)?;
    c.emit(load(R::R0, R::R0, 4)?)?;
    c.literal(R::R1, 0x7428)?;
    c.emit(data_register(C::Al, Op::Add, r4, R::R0, R::R1,
        Shift::Lsl, 0, false)?)?;
    for service in [0x75f4, 0x796c] {
        c.emit(mov_register(C::Al, R::R0, r4))?;
        c.call(service)?;
    }
    c.emit(mov(R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R1, r4))?;
    c.call(0x7a0c)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x7758)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(SERVICE_DESCRIPTOR_BIAS)?;
    Ok(())
}

fn trivial_service(c: &mut Code<'_>) -> Result<(), String> {
    c.emit(mov(R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))
}

/// Apply one of three resource-management actions (release, query, attach),
/// unwinding through one shared error epilogue.
fn dispatch_resource(c: &mut Code<'_>) -> Result<(), String> {
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(sub(R::SP, R::SP, 16)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(mov_register(C::Al, r6, R::R1))?;
    c.emit(mov_register(C::Al, r5, R::R2))?;
    c.emit(load(R::R1, R::R0, 12)?)?;
    c.emit(load(R::R0, R::R1, 4)?)?;
    c.emit(add(r7, R::R1, 16)?)?;
    c.emit(mov_register(C::Al, R::R1, r7))?;
    c.call(0x83e0)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x74b4)?;
    c.emit(load(R::R1, r5, 0)?)?;
    c.emit(add(R::new(3)?, R::SP, 4)?)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x883c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x74c0)?;
    c.check(0x74b4)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(sub(R::SP, R::R11, 16)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x74c0)?;
    c.emit(cmp(r6, 2)?)?;
    c.branch(C::Eq, 0x74f8)?;
    c.emit(cmp(r6, 1)?)?;
    c.branch(C::Eq, 0x7508)?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Ne, 0x74b4)?;
    c.emit(load(R::R0, r5, 4)?)?;
    c.emit(load(r5, R::SP, 12)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x753c)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x75ac)?;
    c.emit(mov(r4, 0)?)?;
    c.branch(C::Al, 0x74b4)?;
    c.check(0x74f8)?;
    c.emit(load(R::R0, R::SP, 12)?)?;
    c.call(0x7578)?;
    c.emit(store(R::R0, r5, 4)?)?;
    c.branch(C::Al, 0x74b4)?;
    c.check(0x7508)?;
    c.emit(load(r6, R::SP, 12)?)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Eq, 0x74b4)?;
    c.emit(load(R::R0, r6, 0)?)?;
    c.emit(load(r5, r5, 4)?)?;
    c.call(0x75f4)?;
    c.emit(load(R::R2, r6, 0)?)?;
    c.emit(load(R::R0, r6, 4)?)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.call(0x79ac)?;
    c.emit(load(r5, r6, 0)?)?;
    c.branch(C::Al, 0x7560)?;
    c.check(0x753c)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x74b4)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.call(0x75f4)?;
    c.emit(load(R::R1, r5, 0)?)?;
    c.emit(load(R::R0, r5, 4)?)?;
    c.call(0x7910)?;
    c.emit(load(r5, r5, 0)?)?;
    c.check(0x7560)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.call(0x7a0c)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x7758)?;
    c.branch(C::Al, 0x74b4)?;
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x73e0 };
    descriptor_shutdown(&mut c)?;
    c.check(0x742c)?;
    for offset in [0x742c, 0x7434] {
        c.check(offset)?;
        trivial_service(&mut c)?;
    }
    c.check(0x743c)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x10000)?)?;
    c.emit(store(R::R0, R::R1, 4)?)?;
    trivial_service(&mut c)?;
    for offset in [0x7450, 0x7458] {
        c.check(offset)?;
        trivial_service(&mut c)?;
    }
    c.check(0x7460)?;
    dispatch_resource(&mut c)?;
    c.check(0x7578)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn typed_resource_handlers_cover_first_dispatcher() {
        let mut code = vec![0; 0x73e0 - 0x1000];
        extend(&mut code).unwrap();
        assert_eq!(code.len(), 0x7578 - 0x1000);
        assert_eq!(u32::from_le_bytes(code[0x7428-0x1000..0x742c-0x1000].try_into().unwrap()), SERVICE_DESCRIPTOR_BIAS);
    }
    #[test]
    fn reject_disconnected_resource_handlers() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
