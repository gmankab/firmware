// SPDX-License-Identifier: GPL-3.0-only
// Independent test of offline Rust code generation for Qualcomm Hexagon.
// This is a standalone Hexagon compiler validation, not device firmware.
#![no_std]

/// Deterministic, architecture-neutral test function in a Hexagon object.
#[unsafe(no_mangle)]
pub extern "C" fn hexagon_compiler_probe(x: u32) -> u32 {
    x.rotate_left(5) ^ 0xDEC0_ADDE
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop {} }
