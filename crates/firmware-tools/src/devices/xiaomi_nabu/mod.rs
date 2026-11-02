// SPDX-License-Identifier: GPL-3.0-only
//! Xiaomi Pad 5 (`nabu`) reference image metadata and build settings.
#[path = "a640/a640_shader.rs"]
pub mod a640_shader;
#[path = "a640/a640_launch.rs"]
pub mod a640_launch;
#[path = "a640/a640_plan.rs"]
pub mod a640_plan;
pub mod elf_specs;
pub mod modemuw;

/// GPU-family identifier stored in the reference A640 SQE/ZAP microcode.
pub const A640_FAMILY: u16 = 0x6ff;
/// OEM reference firmware revision (non-executable metadata).
pub const A640_REVISION: u16 = 0x020;
#[path = "a640/a640_pipeline.rs"]
pub mod a640_pipeline;
#[path = "a640/a640_finalize.rs"]
pub mod a640_finalize;
#[path = "a640/a640_container.rs"]
pub mod a640_container;

pub mod venus_boot;

pub mod venus_runtime;
pub mod venus_runtime_forward;
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
#[path = "cdsp/common/cdsp_boot.rs"]
pub mod cdsp_boot;
#[path = "cdsp/common/cdsp_privilege.rs"]
pub mod cdsp_privilege;
#[path = "cdsp/common/cdsp_tlb.rs"]
pub mod cdsp_tlb;
#[path = "cdsp/common/cdsp_tlb_extension.rs"]
pub mod cdsp_tlb_extension;
#[path = "cdsp/common/cdsp_copyfast.rs"]
pub mod cdsp_copyfast;
#[path = "cdsp/common/cdsp_cache_controller.rs"]
pub mod cdsp_cache_controller;
#[path = "cdsp/common/cdsp_dma_copy.rs"]
pub mod cdsp_dma_copy;

#[path = "cdsp/early/cdsp_early_tlb.rs"]
pub mod cdsp_early_tlb; // Agent 20-09: 0x41e0-forward continuation; integration-only registration
#[path = "cdsp/early/cdsp_early_controllers.rs"]
pub mod cdsp_early_controllers; // Agent 20-09: 0x4290 startup, disjoint from prior owner
#[path = "cdsp/early/cdsp_early_copier.rs"]
pub mod cdsp_early_copier; // Agent 20-09: 0x42bc cache-zero copier
#[path = "cdsp/early/cdsp_early_physcopy.rs"]
pub mod cdsp_early_physcopy; // Agent 20-09: 0x4318 physical copy
#[path = "cdsp/early/cdsp_early_compare.rs"]
pub mod cdsp_early_compare; // Agent 20-09: 0x4364 controller physical-memory compare
#[path = "cdsp/early/cdsp_early_cfgbase.rs"]
pub mod cdsp_early_cfgbase;
#[path = "cdsp/early/cdsp_early_system_start.rs"]
pub mod cdsp_early_system_start;
// Agent20-09: appended CDSP interrupt/debug gate at 0x4568..0x45c0.
#[path = "cdsp/early/cdsp_early_interrupt_debug.rs"]
pub mod cdsp_early_interrupt_debug; // Agent 20-09: 0x43f4 CFGBASE probe, 0x4418..0x4500 structural fill
#[path = "cdsp/early/cdsp_early_supervisor_vectors.rs"]
pub mod cdsp_early_supervisor_vectors; // Agent 20-09 exception/interrupt vectors to 0x471c
#[path = "cdsp/early/cdsp_early_exception_entry.rs"]
pub mod cdsp_early_exception_entry; // Agent 20-09 exception thread context start at 0x471c
#[path = "cdsp/early/cdsp_early_exception_context.rs"]
pub mod cdsp_early_exception_context; // Hexagon V66 context save to 0x481c
#[path = "cdsp/early/cdsp_early_exception_setup.rs"]
pub mod cdsp_early_exception_setup; // Agent20-09 controller setup through 0x4860
#[path = "cdsp/early/cdsp_early_exception_transition.rs"]
pub mod cdsp_early_exception_transition; // Agent20-09 transition through 0x48a4
#[path = "cdsp/early/cdsp_early_exception_wait.rs"]
pub mod cdsp_early_exception_wait; // Agent20-09 MODECTL status/retry loop to 0x48e8
#[path = "cdsp/early/cdsp_early_exception_finalize.rs"]
pub mod cdsp_early_exception_finalize; // Agent20-09 SYSCFG initialization to 0x495c
#[path = "cdsp/early/cdsp_early_context_snapshot.rs"]
pub mod cdsp_early_context_snapshot;
#[path = "cdsp/early/cdsp_early_l2_sync.rs"]
pub mod cdsp_early_l2_sync;
#[path = "cdsp/early/cdsp_early_phys_dump.rs"]
pub mod cdsp_early_phys_dump;
#[path = "cdsp/early/cdsp_early_runtime_maintenance.rs"]
pub mod cdsp_early_runtime_maintenance;
#[path = "cdsp/early/cdsp_early_memory_dispatch.rs"]
pub mod cdsp_early_memory_dispatch;
#[path = "cdsp/early/cdsp_early_worker_bootstrap.rs"]
pub mod cdsp_early_worker_bootstrap;
#[path = "cdsp/early/cdsp_early_worker_runtime.rs"]
pub mod cdsp_early_worker_runtime;
#[path = "cdsp/early/cdsp_early_worker_state.rs"]
pub mod cdsp_early_worker_state;
#[path = "cdsp/early/cdsp_early_worker_trap.rs"]
pub mod cdsp_early_worker_trap;
#[path = "cdsp/early/cdsp_early_worker_idle.rs"]
pub mod cdsp_early_worker_idle;
#[path = "cdsp/early/cdsp_early_worker_thread.rs"]
pub mod cdsp_early_worker_thread;
#[path = "cdsp/early/cdsp_early_worker_guard.rs"]
pub mod cdsp_early_worker_guard;
#[path = "cdsp/early/cdsp_early_worker_snapshot.rs"]
pub mod cdsp_early_worker_snapshot;
#[path = "cdsp/early/cdsp_early_worker_final_calls.rs"]
pub mod cdsp_early_worker_final_calls;
#[path = "cdsp/early/cdsp_early_worker_guard_exit.rs"]
pub mod cdsp_early_worker_guard_exit;
#[path = "cdsp/early/cdsp_early_worker_sync.rs"]
pub mod cdsp_early_worker_sync;
#[path = "cdsp/early/cdsp_early_worker_handoff.rs"]
pub mod cdsp_early_worker_handoff;
#[path = "cdsp/early/cdsp_early_worker_recovery.rs"]
pub mod cdsp_early_worker_recovery;
#[path = "cdsp/early/cdsp_early_revision_guard.rs"]
pub mod cdsp_early_revision_guard;
#[path = "cdsp/early/cdsp_early_dcache_tags.rs"]
pub mod cdsp_early_dcache_tags;
#[path = "cdsp/early/cdsp_early_icache_guard.rs"]
pub mod cdsp_early_icache_guard;
#[path = "cdsp/early/cdsp_early_icache_tags.rs"]
pub mod cdsp_early_icache_tags;
#[path = "cdsp/early/cdsp_early_cache_helpers.rs"]
pub mod cdsp_early_cache_helpers;
#[path = "cdsp/early/cdsp_early_format_guard.rs"]
pub mod cdsp_early_format_guard;
#[path = "cdsp/early/cdsp_early_format_guard_tail.rs"]
pub mod cdsp_early_format_guard_tail;
#[path = "cdsp/early/cdsp_early_format_guard_final.rs"]
pub mod cdsp_early_format_guard_final;
#[path = "cdsp/early/cdsp_early_hvx_init.rs"]
pub mod cdsp_early_hvx_init;
#[path = "cdsp/early/cdsp_early_tlb_state.rs"]
pub mod cdsp_early_tlb_state;
#[path = "cdsp/early/cdsp_early_controller_restoration.rs"]
pub mod cdsp_early_controller_restoration;
#[path = "cdsp/early/cdsp_early_supervisor_dispatch.rs"]
pub mod cdsp_early_supervisor_dispatch;
#[path = "cdsp/early/cdsp_early_dispatch_helpers.rs"]
pub mod cdsp_early_dispatch_helpers;
#[path = "cdsp/early/cdsp_early_tlb_utilities.rs"]
pub mod cdsp_early_tlb_utilities;
#[path = "cdsp/early/cdsp_early_tlb_encode.rs"]
pub mod cdsp_early_tlb_encode;
#[path = "cdsp/early/cdsp_early_tlb_control.rs"]
pub mod cdsp_early_tlb_control;
#[path = "cdsp/early/cdsp_early_worker_privileged_state.rs"]
pub mod cdsp_early_worker_privileged_state;
#[path = "cdsp/early/cdsp_early_worker_callbacks.rs"]
pub mod cdsp_early_worker_callbacks;
#[path = "cdsp/early/cdsp_early_worker_command.rs"]
pub mod cdsp_early_worker_command;
#[path = "cdsp/early/cdsp_early_worker_decision.rs"]
pub mod cdsp_early_worker_decision;
#[path = "cdsp/early/cdsp_early_worker_lock.rs"]
pub mod cdsp_early_worker_lock; // Agent20-09 variable-width memory selector // Agent20-09 CDSP stop/restart and TLB save through 0x4b1c // Agent20-09 physical-state copy and cache flush to 0x4ab0 // Agent20-09 L2 cache global helper through 0x4a38 // CDSP read privileged state into exception frame to 0x4a08

// Own large A32 generator groups in independent, low-memory Rust crates.
pub use venus_controller::*;
pub use venus_tail::venus_tail_program;

// Recovered public modules from disjoint source-authoring branches.
pub mod cdsp_late_atomic_counter;
pub mod cdsp_late_bounded_compare;
pub mod cdsp_late_callback_completion;
pub mod cdsp_late_callback_dispatch;
pub mod cdsp_late_callback_poll;
pub mod cdsp_late_callback_registry;
pub mod cdsp_late_callback_service;
pub mod cdsp_late_callback_sweeper;
pub mod cdsp_late_controller_bank_bitset_args;
pub mod cdsp_late_controller_bank_bitset_entry;
pub mod cdsp_late_controller_bank_bitset_fields;
pub mod cdsp_late_controller_bank_bitset_limits;
pub mod cdsp_late_controller_bank_bitset_scan;
pub mod cdsp_late_controller_bank_entry;
pub mod cdsp_late_controller_bank_guards;
pub mod cdsp_late_controller_bank_initialize;
pub mod cdsp_late_controller_bank_predicates;
pub mod cdsp_late_controller_bitloop;
pub mod cdsp_late_controller_byte_scan;
pub mod cdsp_late_controller_error_tail;
pub mod cdsp_late_controller_frame_setup;
pub mod cdsp_late_controller_init_dispatch;
pub mod cdsp_late_controller_mask_setup;
pub mod cdsp_late_controller_mode_guard;
pub mod cdsp_late_controller_record_advance;
pub mod cdsp_late_controller_record_bounds;
pub mod cdsp_late_controller_record_diagnostics;
pub mod cdsp_late_controller_record_lookup;
pub mod cdsp_late_controller_return_paths;
pub mod cdsp_late_controller_secondary_bitloop;
pub mod cdsp_late_controller_secondary_completion;
pub mod cdsp_late_controller_selection;
pub mod cdsp_late_controller_status;
pub mod cdsp_late_controller_transfer;
pub mod cdsp_late_controller_validate;
pub mod cdsp_late_copy_delegate;
pub mod cdsp_late_gp_state_multiplier;
pub mod cdsp_late_large_prologue;
pub mod cdsp_late_mmio;
pub mod cdsp_late_null_callback;
pub mod cdsp_late_rng_init;
pub mod cdsp_late_rng_seed_state;
pub mod cdsp_late_rng_update;
pub mod cdsp_late_runtime;
pub mod cdsp_late_runtime_loader;
pub mod cdsp_late_runtime_more;
pub mod cdsp_late_seed_handoff;
pub mod venus_runtime_mid;
pub mod venus_runtime_seam;
