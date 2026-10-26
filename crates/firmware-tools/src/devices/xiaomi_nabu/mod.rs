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
pub mod venus_agent01_forward;
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

mod venus_runtime_bridge_lookup;
mod venus_runtime_bridge_records;
mod venus_runtime_bridge_state;
mod venus_runtime_bridge_slots;
mod venus_runtime_bridge_irq_ready;
mod venus_runtime_bridge_resource;
mod venus_runtime_bridge_queue_release;
mod venus_runtime_bridge_controller_queues;
mod venus_runtime_bridge_service_link;
pub mod venus_runtime_bridge;
pub mod venus_plan;

mod venus_runtime_mmio;
mod venus_runtime_platform;
mod venus_runtime_controller;
mod venus_runtime_modes;
mod venus_runtime_service_link;
mod venus_runtime_descriptors;
mod venus_runtime_resource_ops;
mod venus_runtime_resource_table;
mod venus_runtime_keys;
mod venus_runtime_bits;
mod venus_runtime_controller_state;
mod venus_runtime_service_controller;
mod venus_runtime_service_frontends;
mod venus_runtime_controller_io;
mod venus_runtime_registers;
mod venus_runtime_registers_secondary;
mod venus_runtime_interface_lookup;

// Agent 03: independently verified disjoint late-Venus A32 routines.
pub mod venus_runtime_late;
// Agent 05: separately reconstructed CDSP Hexagon routines.
pub mod cdsp_boot;
pub mod cdsp_privilege;
pub mod cdsp_tlb;
pub mod cdsp_tlb_extension;
pub mod cdsp_copyfast;
pub mod cdsp_cache_controller;
pub mod cdsp_dma_copy;

pub mod cdsp_early_tlb; // Agent 20-09: 0x41e0-forward continuation; integration-only registration
pub mod cdsp_early_controllers; // Agent 20-09: 0x4290 startup, disjoint from prior owner
pub mod cdsp_early_copier; // Agent 20-09: 0x42bc cache-zero copier
pub mod cdsp_early_physcopy; // Agent 20-09: 0x4318 physical copy
pub mod cdsp_early_compare; // Agent 20-09: 0x4364 controller physical-memory compare
pub mod cdsp_early_cfgbase;
pub mod cdsp_early_system_start;
// Agent20-09: appended CDSP interrupt/debug gate at 0x4568..0x45c0.
pub mod cdsp_early_interrupt_debug; // Agent 20-09: 0x43f4 CFGBASE probe, 0x4418..0x4500 structural fill
pub mod cdsp_early_supervisor_vectors; // Agent 20-09 exception/interrupt vectors to 0x471c
pub mod cdsp_early_exception_entry; // Agent 20-09 exception thread context start at 0x471c
pub mod cdsp_early_exception_context; // Hexagon V66 context save to 0x481c
pub mod cdsp_early_exception_setup; // Agent20-09 controller setup through 0x4860
pub mod cdsp_early_exception_transition; // Agent20-09 transition through 0x48a4
pub mod cdsp_early_exception_wait; // Agent20-09 MODECTL status/retry loop to 0x48e8
pub mod cdsp_early_exception_finalize; // Agent20-09 SYSCFG initialization to 0x495c
pub mod cdsp_early_context_snapshot;
pub mod cdsp_early_l2_sync;
pub mod cdsp_early_phys_dump;
pub mod cdsp_early_runtime_maintenance;
pub mod cdsp_early_memory_dispatch;
pub mod cdsp_early_worker_bootstrap;
pub mod cdsp_early_worker_runtime;
pub mod cdsp_early_worker_state;
pub mod cdsp_early_worker_trap;
pub mod cdsp_early_worker_idle;
pub mod cdsp_early_worker_thread;
pub mod cdsp_early_worker_guard;
pub mod cdsp_early_worker_privileged_state;
pub mod cdsp_early_worker_callbacks;
pub mod cdsp_early_worker_command;
pub mod cdsp_early_worker_decision;
pub mod cdsp_early_worker_lock; // Agent20-09 variable-width memory selector // Agent20-09 CDSP stop/restart and TLB save through 0x4b1c // Agent20-09 physical-state copy and cache flush to 0x4ab0 // Agent20-09 L2 cache global helper through 0x4a38 // CDSP read privileged state into exception frame to 0x4a08
