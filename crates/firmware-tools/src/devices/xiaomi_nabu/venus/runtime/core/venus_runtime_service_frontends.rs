// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller service frontends and ordered initialization sequences.
use arm32::{
    data_immediate, mov_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, mov, store};

fn pop(c: &mut Code<'_>, saved: u16) -> Result<(), String> {
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true)?)
}
fn frame(c: &mut Code<'_>, saved: u16, offset: u32) -> Result<(), String> {
    c.emit(push_registers(C::Al, saved)?)?;
    if offset == 0 { c.emit(mov_register(C::Al, R::R11, R::SP)) }
    else { c.emit(add(R::R11, R::SP, offset)?) }
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x7ce0 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let short_saved = r4.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit();
    let normal_saved = r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit();
    // Return a normalized power/control state from the firmware service.
    frame(&mut c, short_saved, 8)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.call(0x7f80)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x7d14)?;
    c.emit(cmp(R::R0, 2)?)?;
    c.branch(C::Ne, 0x7d1c)?;
    c.call(0x849d4)?;
    c.emit(mov(R::R1, 1)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 2)?)?;
    c.branch(C::Al, 0x7d18)?;
    c.check(0x7d14)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(store(R::R1, r4, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    pop(&mut c, short_saved)?;
    c.check(0x7d24)?;
    // Apply one controller mode, restoring the previous privilege state.
    frame(&mut c, normal_saved, 8)?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.call(0x7f28)?;
    c.emit(cmp(r5, 1)?)?;
    c.branch(C::Eq, 0x7d58)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Ne, 0x7d64)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(r4, 0)?)?;
    c.call(0x7f60)?;
    c.branch(C::Al, 0x7d64)?;
    c.check(0x7d58)?;
    c.emit(mov(R::R0, 2)?)?;
    c.call(0x7f60)?;
    c.emit(mov(r4, 0)?)?;
    c.call(0x7f44)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    pop(&mut c, normal_saved)?;
    c.check(0x7d70)?;
    // Query paired state values through a guarded, shared controller service.
    frame(&mut c, normal_saved, 8)?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.call(0x7f28)?;
    c.call(0x826c)?;
    c.emit(store(R::R0, r5, 0)?)?;
    c.call(0x8148)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.call(0x7f44)?;
    c.emit(mov(R::R0, 0)?)?;
    pop(&mut c, normal_saved)?;
    c.check(0x7da0)?;
    // Write two independent configuration values with the same guard.
    frame(&mut c, normal_saved, 8)?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.call(0x8174)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x8244)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x8160)?;
    c.call(0x8190)?;
    c.emit(mov(R::R0, 0)?)?;
    pop(&mut c, normal_saved)?;
    c.check(0x7dd0)?;
    // Initialize related controller services in an explicit deterministic order.
    frame(&mut c, R::R11.bit() | R::LR.bit(), 0)?;
    c.call(0x8174)?;
    c.emit(mov(R::R0, 0)?)?;
    c.call(0x81ac)?;
    c.emit(mov(R::R0, 15)?)?;
    for service in [0x81cc, 0x81f8, 0x8220, 0x831c, 0x82a0,
                    0x8190, 0x7f28, 0x80a8, 0x8074, 0x7f44] {
        c.call(service)?;
    }
    c.emit(mov(R::R0, 0)?)?;
    pop(&mut c, R::R11.bit() | R::LR.bit())?;
    c.check(0x7e18)?;
    // Service finalizer.
    frame(&mut c, R::R11.bit() | R::LR.bit(), 0)?;
    c.call(0x8054)?;
    c.emit(mov(R::R0, 0)?)?;
    pop(&mut c, R::R11.bit() | R::LR.bit())?;
    c.check(0x7e2c)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn controller_frontends_are_contiguous() {
        let mut bytes = vec![0; 0x7ce0-0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x7e2c-0x1000);
    }
    #[test]
    fn frontends_refuse_unanchored_segments() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
