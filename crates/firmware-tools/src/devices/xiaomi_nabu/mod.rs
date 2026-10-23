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
mod venus_runtime_format_output;
mod venus_runtime_format_strings;
mod venus_runtime_format_finish;
mod venus_runtime_format_small;
mod venus_runtime_format_helpers;
mod venus_runtime_format_args;
mod venus_runtime_diag_write;
mod venus_runtime_diag_buffer;
mod venus_runtime_diag_memory;
mod venus_runtime_memscan;
mod venus_runtime_memcpy;
mod venus_runtime_memcpy_unaligned;
mod venus_runtime_memcpy_tail;
mod venus_runtime_errors;
mod venus_runtime_stream_state;
mod venus_runtime_utf8;
mod venus_runtime_flush;
mod venus_runtime_lifecycle;
mod venus_runtime_ring;
mod venus_runtime_service_handles;
mod venus_runtime_service_init;
mod venus_runtime_resources;
mod venus_runtime_state;
mod venus_runtime_diagnostics;
mod venus_runtime_events;
mod venus_runtime_event_loop;
mod venus_runtime_event_setup;
mod venus_runtime_event_links;
mod venus_runtime_event_teardown;
mod venus_runtime_device_setup;
mod venus_runtime_device_init;
mod venus_runtime_device_access;
mod venus_runtime_device_commands;
mod venus_runtime_device_registration;
mod venus_runtime_device_properties;
mod venus_runtime_device_cleanup;
mod venus_runtime_device_alloc_begin;
mod venus_runtime_device_alloc_finish;
mod venus_runtime_device_release;
mod venus_runtime_bitmap;
mod venus_runtime_bitmap_controls;

pub mod venus_plan;

mod venus_runtime_mmio;
