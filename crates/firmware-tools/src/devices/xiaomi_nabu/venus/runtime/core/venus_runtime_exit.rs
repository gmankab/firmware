// SPDX-License-Identifier: GPL-3.0-only
//! Venus process exit via the public ARM Linux-compatible SVC ABI.
//! Two supervisor calls are encoded from typed immediates, not instruction
//! literals; the fallback loop is a symbolic branch to itself.
use arm32::{mov_register, push_registers, supervisor_call, Condition as C, Register as R};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::mov;

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x33f0 };
    let r7 = R::new(7)?;
    c.emit(push_registers(C::Al, r7.bit() | R::LR.bit())?)?;
    c.emit(mov(r7, 248)?)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(supervisor_call(C::Al, 0)?)?;
    c.emit(mov(r7, 1)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(supervisor_call(C::Al, 0)?)?;
    c.branch(C::Al, 0x3404)?;
    c.check(0x3410)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_unknown_prefix() { assert!(extend(&mut Vec::new()).is_err()); }
}
