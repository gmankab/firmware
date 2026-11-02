// SPDX-License-Identifier: GPL-3.0-only
//! Controller initialization and independently encoded runtime-service stubs.
use arm32::{
    bx, data_immediate, mov_register, push_registers, transfer_multiple,
    transfer_word_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

const SERVICE_NAME: u32 = 0x0010_394c;
const SERVICE_OBJECT: u32 = 0x0010_39c4;
const SERVICE_MAGIC: u32 = 0xda1f_da1f;
const SERVICE_VTABLE: u32 = 0x000e_3538;
const CONTROLLER_REGISTER_BASE: u32 = 0xfc30_7000;

fn pop(c: &mut Code<'_>, saved: u16) -> Result<(), String> {
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true)?)
}
fn zero_stub(c: &mut Code<'_>) -> Result<(), String> {
    c.emit(mov(R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x7b80 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    // Create and publish a controller runtime object with linked-list sentinels.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.emit(mov(R::R1, 0)?)?;
    c.literal(r5, 0x7be0)?;
    c.emit(store(R::R1, r4, 0)?)?;
    c.literal(R::R1, 0x7bdc)?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.call(0x84574)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Ne, 0x7bd4)?;
    c.literal(R::R0, 0x7be4)?;
    c.emit(mov_register(C::Al, r7, r5))?;
    c.emit(transfer_word_preindexed_writeback(C::Al, false, R::R0, r7, 16)?)?;
    c.literal(R::R0, 0x7be8)?;
    c.emit(store(R::R0, r5, 20)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(store(r5, r5, 24)?)?;
    c.call(0x84804)?;
    c.emit(store(r7, r4, 0)?)?;
    c.check(0x7bd4)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    pop(&mut c, saved)?;
    for data in [SERVICE_NAME, SERVICE_OBJECT, SERVICE_MAGIC, SERVICE_VTABLE] {
        c.emit(data)?;
    }
    c.check(0x7bec)?;
    zero_stub(&mut c)?;
    zero_stub(&mut c)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.branch(C::Al, 0x84838)?;
    c.check(0x7c04)?;
    // Bind two MMIO subregions and register them with shared services.
    let short_saved = r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, short_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(add(R::R2, R::SP, 4)?)?;
    c.emit(load(r5, R::R0, 12)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, R::SP, 4)?)?;
    c.literal(r4, 0x7c70)?;
    c.emit(add(R::R1, r5, 88)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x7eb4)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x7c68)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    c.call(0x7f18)?;
    c.emit(add(R::R0, r4, 110_592)?)?;
    c.emit(add(R::R1, r5, 92)?)?;
    c.emit(add(R::R2, R::SP, 4)?)?;
    c.call(0x7eb4)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x7c68)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    c.call(0x8138)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    pop(&mut c, short_saved)?;
    c.emit(CONTROLLER_REGISTER_BASE)?;
    c.check(0x7c74)?;
    // Unregister the two controller subregions on shutdown.
    let destructor_saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, destructor_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(load(r4, R::R0, 12)?)?;
    c.emit(load(R::R0, r4, 88)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x7c94)?;
    c.call(0x4f1c)?;
    c.emit(load(R::R0, r4, 92)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x7ca4)?;
    c.call(0x4f1c)?;
    c.emit(mov(R::R0, 0)?)?;
    pop(&mut c, destructor_saved)?;
    c.check(0x7cac)?;
    zero_stub(&mut c)?;
    zero_stub(&mut c)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 65_536)?)?;
    c.emit(store(R::R0, R::R1, 4)?)?;
    zero_stub(&mut c)?;
    zero_stub(&mut c)?;
    zero_stub(&mut c)?;
    c.check(0x7ce0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn controller_region_and_stubs_are_contiguous() {
        let mut bytes = vec![0; 0x7b80-0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x7ce0-0x1000);
    }
    #[test]
    fn controller_block_requires_exact_layout() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
