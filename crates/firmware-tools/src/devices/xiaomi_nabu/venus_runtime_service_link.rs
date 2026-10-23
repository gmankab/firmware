// SPDX-License-Identifier: GPL-3.0-only
//! Link a Venus service resource into the runtime's callback registry.
//! Service pointers and sentinel fields are device-specific metadata.
use arm32::{
    bx, mov_register, push_registers, transfer_multiple,
    transfer_word_preindexed_writeback, BlockMode, Condition as C,
    Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

const SERVICE_TYPE: u32 = 0x0010_38b0;
const SERVICE_POOL: u32 = 0x0010_3928;
const SERVICE_SENTINEL: u32 = 0xda1f_da1f;
const SERVICE_CALLBACK: u32 = 0x000e_34d8;

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x7198 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.literal(R::R2, 0x71fc)?;
    c.literal(r5, 0x7200)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(mov_register(C::Al, R::R1, R::R2))?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.call(0x846f8)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Ne, 0x71f4)?;
    c.literal(R::R0, 0x7204)?;
    c.emit(mov_register(C::Al, r7, r5))?;
    c.emit(transfer_word_preindexed_writeback(C::Al, false,
        R::R0, r7, 16)?)?;
    c.literal(R::R0, 0x7208)?;
    c.emit(store(R::R0, r5, 20)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(store(r5, r5, 24)?)?;
    c.call(0x84804)?;
    c.emit(store(r7, r4, 0)?)?;
    c.check(0x71f4)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    for pointer in [SERVICE_TYPE, SERVICE_POOL, SERVICE_SENTINEL, SERVICE_CALLBACK] {
        c.emit(pointer)?;
    }
    c.check(0x720c)?;
    // Two disabled service interface hooks return a null handle.
    for offset in [0x720c, 0x7214] {
        c.check(offset)?;
        c.emit(mov(R::R0, 0)?)?;
        c.emit(bx(C::Al, R::LR))?;
    }
    c.check(0x721c)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.branch(C::Al, 0x84838)?;
    c.check(0x7224)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn service_registry_layout_and_callback_addresses() {
        let mut bytes = vec![0; 0x7198 - 0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x7224 - 0x1000);
        assert_eq!(u32::from_le_bytes(bytes[0x7208-0x1000..0x720c-0x1000].try_into().unwrap()), SERVICE_CALLBACK);
    }
    #[test]
    fn service_registry_rejects_gapped_inputs() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
