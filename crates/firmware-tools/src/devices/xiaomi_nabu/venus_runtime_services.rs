// SPDX-License-Identifier: GPL-3.0-only
//! Venus exception/service teardown and ARM CP15 cache-control entry points.
//! The instruction operands come from checked ARM32 encoders. State and
//! literal addresses are device-specific structured data.
use arm32::{
    branch, bx, data_immediate, data_register, mcr, mrc, mov_register, push_registers,
    transfer_multiple, transfer_word_postindexed_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_tail::{put, add, cmp, load, store, mov};

pub(super) struct Code<'a> { pub(super) bytes: &'a mut Vec<u8>, pub(super) at: usize }
impl Code<'_> {
    pub(super) fn emit(&mut self, word: u32) -> Result<(), String> {
        put(self.bytes, self.at, word)?;
        self.at += 4;
        Ok(())
    }
    pub(super) fn call(&mut self, destination: usize) -> Result<(), String> {
        self.emit(branch(C::Al, self.at as u32, destination as u32, true)?)
    }
    pub(super) fn branch(&mut self, cond: C, destination: usize) -> Result<(), String> {
        self.emit(branch(cond, self.at as u32, destination as u32, false)?)
    }
    pub(super) fn literal(&mut self, reg: R, location: usize) -> Result<(), String> {
        self.emit(load(reg, R::PC, (location as isize - self.at as isize - 8) as i32)?)
    }
    pub(super) fn check(&self, address: usize) -> Result<(), String> {
        if self.at != address { return Err(format!("Venus service layout mismatch: 0x{:x} != 0x{address:x}", self.at)); }
        Ok(())
    }
}

/// Flush/invalidate one cache range through a chosen ARM CP15 maintenance
/// operation; this is public ARM architecture behavior and reusable code.
fn cache_range(c: &mut Code<'_>, crm: u8) -> Result<(), String> {
    let loop_at = c.at + 8;
    c.emit(data_immediate(C::Al, Op::Bic, R::R0, R::R0, 31)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, crm, 1)?)?;
    c.emit(add(R::R0, R::R0, 32)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Lt, loop_at)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 10, 4)?)?;
    c.emit(bx(C::Al, R::LR))?;
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x1ec0 };
    let r4 = R::new(4)?;
    let r7 = R::new(7)?;

    // Install hardware initialization callbacks and hand control to a common
    // boot-state service, preserving the original conditional return paths.
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.literal(R::R0, 0x1f1c)?;
    c.emit(mov(R::R1, 5)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    for (address, location, value) in [
        (0x1ed4, 0x1f20, 0x1f24), (0x1ee0, 0x1f28, 0x1f2c),
    ] {
        c.check(address)?;
        c.literal(R::R0, location)?;
        c.literal(R::R1, value)?;
        c.emit(store(R::R1, R::R0, 0)?)?;
    }
    c.call(0x25780)?;
    c.literal(R::R1, 0x1f30)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x1f08)?;
    c.emit(mov(R::R2, 1)?)?;
    c.emit(store(R::R2, R::R1, 0)?)?;
    c.branch(C::Al, 0x1f14)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x295c)?;
    c.check(0x1f1c)?;
    for value in [0xe00d_2058, 0x0010_cffc, 0x0000_1994,
                  0x0010_cff8, 0x0000_1214, 0x0010_306c] { c.emit(value)?; }
    c.check(0x1f34)?;

    // Release firmware-owned services in order. The final branch tail-calls
    // the common object-release function after restoring preserved registers.
    c.emit(push_registers(C::Al, r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.call(0x2634c)?;
    c.literal(r4, 0x1fa8)?;
    for (at, offset, destination) in [
        (0x1f44, 0, 0x2aa24),
        (0x1f4c, 0, 0x2f1c),
        (0x1f54, 4, 0x2f7d0),
        (0x1f5c, 4, 0x2f1c),
        (0x1f64, 12, 0x250c),
        (0x1f6c, 8, 0x250c),
        (0x1f74, 20, 0x250c),
        (0x1f7c, 16, 0x250c),
        (0x1f84, 12, 0x2f1c),
        (0x1f8c, 8, 0x2f1c),
        (0x1f94, 20, 0x2f1c),
    ] {
        c.check(at)?;
        c.emit(load(R::R0, r4, offset)?)?;
        c.call(destination)?;
    }
    c.check(0x1f9c)?;
    c.emit(load(R::R0, r4, 16)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x2f1c)?;
    c.emit(0x0010_ceac)?; // Runtime object pointer
    c.check(0x1fac)?;

    // Two four-word indirect service entry points (including address literals).
    for (at, target) in [(0x1fac, 0x2b13c), (0x1fbc, 0x2b178)] {
        c.check(at)?;
        c.literal(R::R0, at + 12)?;
        c.emit(load(R::R0, R::R0, 0)?)?;
        c.branch(C::Al, target)?;
        c.emit(0x0010_ceac)?;
    }
    c.check(0x1fcc)?;

    // Service pointer resolution; nonzero state takes a mapped-memory path.
    c.literal(R::R0, 0x200c)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x2008)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0xe000_0000)?)?;
    c.emit(mov_register(C::Al, R::R2, R::R0))?;
    c.emit(transfer_word_postindexed_register(C::Al, true, R::R1, R::R2, R::R1)?)?;
    c.emit(cmp(R::R2, 0x0010_0000)?)?;
    c.emit(bx(C::Cc, R::LR))?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x2008)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R2, R::R1, 4)?)?;
    c.literal(R::R1, 0x2010)?;
    c.emit(add(R::R0, R::R0, 4)?)?;
    c.branch(C::Al, 0x3294)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(0xe00d_205c)?; // Platform pointer
    c.emit(0x0010_3084)?; // Firmware runtime diagnostic pointer
    c.check(0x2014)?;

    for (start, crm) in [(0x2014, 10), (0x2038, 6), (0x205c, 14)] {
        c.check(start)?;
        cache_range(&mut c, crm)?;
    }
    c.check(0x2080)?;
    // CP15 cache synchronization loops. Writing MRC to r15 updates APSR
    // condition flags instead of storing a general-purpose register value.
    c.emit(mrc(C::Al, 15, 0, R::PC, 7, 10, 3)?)?;
    c.branch(C::Ne, 0x2080)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 10, 4)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 6, 0)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 10, 4)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x20a0)?;
    c.emit(mrc(C::Al, 15, 0, R::PC, 7, 14, 3)?)?;
    c.branch(C::Ne, 0x20a0)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 10, 4)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 5, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x20c0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_previous_region() { assert!(extend(&mut vec![]).is_err()); }
}
