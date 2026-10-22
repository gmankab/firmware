// SPDX-License-Identifier: GPL-3.0-only
//! Xiaomi Pad 5 (`nabu`) reference image metadata and build settings.
pub mod a640_shader;
pub mod a640_launch;
pub mod a640_plan;
pub mod elf_specs;
pub mod modemuw;

/// GPU-family identifier stored in the reference A640 SQE/ZAP microcode.
pub const A640_FAMILY: u16 = 0x6ff;
/// OEM reference firmware revision (non-executable metadata).
pub const A640_REVISION: u16 = 0x020;
