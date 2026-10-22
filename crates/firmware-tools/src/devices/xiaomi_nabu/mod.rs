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
pub mod a640_pipeline;
pub mod a640_finalize;
pub mod a640_container;

pub mod venus_boot;

pub mod venus_runtime;
mod venus_runtime_tail;
mod venus_runtime_init;
mod venus_runtime_services;
mod venus_runtime_services_next;
mod venus_runtime_dispatch;
mod venus_runtime_queue;
mod venus_runtime_allocator;
mod venus_runtime_mask;
mod venus_runtime_wrappers;
mod venus_runtime_setup;
mod venus_runtime_object;
mod venus_runtime_object_ext;
mod venus_runtime_eventmask;
mod venus_runtime_alloc_aligned;
mod venus_runtime_alloc_variable;
mod venus_runtime_callbacks;
mod venus_runtime_ctors;
mod venus_runtime_memfill;
mod venus_runtime_strings;
mod venus_runtime_exit;
mod venus_runtime_resource_update;
mod venus_runtime_formatter;
mod venus_runtime_format_dispatch;
mod venus_runtime_format_numbers;

pub mod venus_plan;
