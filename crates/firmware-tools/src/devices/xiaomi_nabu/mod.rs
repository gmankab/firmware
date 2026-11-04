// SPDX-License-Identifier: GPL-3.0-only
//! Xiaomi Pad 5 (`nabu`) reference image metadata and build settings.
#[path = "ath10k/firmware_metadata.rs"]
pub mod ath10k_firmware_metadata;
#[path = "a640/a640_shader.rs"]
pub mod a640_shader;
#[path = "a640/a640_launch.rs"]
pub mod a640_launch;
#[path = "a640/a640_plan.rs"]
pub mod a640_plan;
#[path = "formats/elf_specs.rs"]
pub mod elf_specs;
#[path = "modem/common/modemuw.rs"]
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

#[path = "venus/common/venus_boot.rs"]
pub mod venus_boot;

#[path = "venus/common/venus_runtime.rs"]
pub mod venus_runtime;
#[path = "venus/runtime/core/venus_runtime_forward.rs"]
pub mod venus_runtime_forward;
#[path = "venus/runtime/core/venus_runtime_tail.rs"]
mod venus_runtime_tail;
#[path = "venus/runtime/core/venus_runtime_init.rs"]
mod venus_runtime_init;
#[path = "venus/runtime/core/venus_runtime_services.rs"]
mod venus_runtime_services;
#[path = "venus/runtime/core/venus_runtime_services_next.rs"]
mod venus_runtime_services_next;
#[path = "venus/runtime/core/venus_runtime_dispatch.rs"]
mod venus_runtime_dispatch;
#[path = "venus/runtime/core/venus_runtime_queue.rs"]
mod venus_runtime_queue;
#[path = "venus/runtime/core/venus_runtime_allocator.rs"]
mod venus_runtime_allocator;
#[path = "venus/runtime/core/venus_runtime_mask.rs"]
mod venus_runtime_mask;
#[path = "venus/runtime/core/venus_runtime_wrappers.rs"]
mod venus_runtime_wrappers;
#[path = "venus/runtime/core/venus_runtime_setup.rs"]
mod venus_runtime_setup;
#[path = "venus/runtime/core/venus_runtime_object.rs"]
mod venus_runtime_object;
#[path = "venus/runtime/core/venus_runtime_object_ext.rs"]
mod venus_runtime_object_ext;
#[path = "venus/runtime/core/venus_runtime_eventmask.rs"]
mod venus_runtime_eventmask;
#[path = "venus/runtime/core/venus_runtime_alloc_aligned.rs"]
mod venus_runtime_alloc_aligned;
#[path = "venus/runtime/core/venus_runtime_alloc_variable.rs"]
mod venus_runtime_alloc_variable;
#[path = "venus/runtime/core/venus_runtime_callbacks.rs"]
mod venus_runtime_callbacks;
#[path = "venus/runtime/core/venus_runtime_ctors.rs"]
mod venus_runtime_ctors;
#[path = "venus/runtime/core/venus_runtime_memfill.rs"]
mod venus_runtime_memfill;
#[path = "venus/runtime/core/venus_runtime_strings.rs"]
mod venus_runtime_strings;
#[path = "venus/runtime/core/venus_runtime_exit.rs"]
mod venus_runtime_exit;
#[path = "venus/runtime/core/venus_runtime_resource_update.rs"]
mod venus_runtime_resource_update;
#[path = "venus/runtime/core/venus_runtime_formatter.rs"]
mod venus_runtime_formatter;
#[path = "venus/runtime/core/venus_runtime_format_dispatch.rs"]
mod venus_runtime_format_dispatch;
#[path = "venus/runtime/core/venus_runtime_format_numbers.rs"]
mod venus_runtime_format_numbers;
#[path = "venus/runtime/core/venus_runtime_format_output.rs"]
mod venus_runtime_format_output;
#[path = "venus/runtime/core/venus_runtime_format_strings.rs"]
mod venus_runtime_format_strings;
#[path = "venus/runtime/core/venus_runtime_format_finish.rs"]
mod venus_runtime_format_finish;
#[path = "venus/runtime/core/venus_runtime_format_small.rs"]
mod venus_runtime_format_small;
#[path = "venus/runtime/core/venus_runtime_format_helpers.rs"]
mod venus_runtime_format_helpers;
#[path = "venus/runtime/core/venus_runtime_format_args.rs"]
mod venus_runtime_format_args;
#[path = "venus/runtime/core/venus_runtime_diag_write.rs"]
mod venus_runtime_diag_write;
#[path = "venus/runtime/core/venus_runtime_diag_buffer.rs"]
mod venus_runtime_diag_buffer;
#[path = "venus/runtime/core/venus_runtime_diag_memory.rs"]
mod venus_runtime_diag_memory;
#[path = "venus/runtime/core/venus_runtime_memscan.rs"]
mod venus_runtime_memscan;
#[path = "venus/runtime/core/venus_runtime_memcpy.rs"]
mod venus_runtime_memcpy;
#[path = "venus/runtime/core/venus_runtime_memcpy_unaligned.rs"]
mod venus_runtime_memcpy_unaligned;
#[path = "venus/runtime/core/venus_runtime_memcpy_tail.rs"]
mod venus_runtime_memcpy_tail;
#[path = "venus/runtime/core/venus_runtime_errors.rs"]
mod venus_runtime_errors;
#[path = "venus/runtime/core/venus_runtime_stream_state.rs"]
mod venus_runtime_stream_state;
#[path = "venus/runtime/core/venus_runtime_utf8.rs"]
mod venus_runtime_utf8;
#[path = "venus/runtime/core/venus_runtime_flush.rs"]
mod venus_runtime_flush;
#[path = "venus/runtime/core/venus_runtime_lifecycle.rs"]
mod venus_runtime_lifecycle;
#[path = "venus/runtime/core/venus_runtime_ring.rs"]
mod venus_runtime_ring;
#[path = "venus/runtime/core/venus_runtime_service_handles.rs"]
mod venus_runtime_service_handles;
#[path = "venus/runtime/core/venus_runtime_service_init.rs"]
mod venus_runtime_service_init;
#[path = "venus/runtime/core/venus_runtime_resources.rs"]
mod venus_runtime_resources;
#[path = "venus/runtime/core/venus_runtime_state.rs"]
mod venus_runtime_state;
#[path = "venus/runtime/core/venus_runtime_diagnostics.rs"]
mod venus_runtime_diagnostics;
#[path = "venus/runtime/core/venus_runtime_events.rs"]
mod venus_runtime_events;
#[path = "venus/runtime/core/venus_runtime_event_loop.rs"]
mod venus_runtime_event_loop;
#[path = "venus/runtime/core/venus_runtime_event_setup.rs"]
mod venus_runtime_event_setup;
#[path = "venus/runtime/core/venus_runtime_event_links.rs"]
mod venus_runtime_event_links;
#[path = "venus/runtime/core/venus_runtime_event_teardown.rs"]
mod venus_runtime_event_teardown;
#[path = "venus/runtime/core/venus_runtime_device_setup.rs"]
mod venus_runtime_device_setup;
#[path = "venus/runtime/core/venus_runtime_device_init.rs"]
mod venus_runtime_device_init;
#[path = "venus/runtime/core/venus_runtime_device_access.rs"]
mod venus_runtime_device_access;
#[path = "venus/runtime/core/venus_runtime_device_commands.rs"]
mod venus_runtime_device_commands;
#[path = "venus/runtime/core/venus_runtime_device_registration.rs"]
mod venus_runtime_device_registration;
#[path = "venus/runtime/core/venus_runtime_device_properties.rs"]
mod venus_runtime_device_properties;
#[path = "venus/runtime/core/venus_runtime_device_cleanup.rs"]
mod venus_runtime_device_cleanup;
#[path = "venus/runtime/core/venus_runtime_device_alloc_begin.rs"]
mod venus_runtime_device_alloc_begin;
#[path = "venus/runtime/core/venus_runtime_device_alloc_finish.rs"]
mod venus_runtime_device_alloc_finish;
#[path = "venus/runtime/core/venus_runtime_device_release.rs"]
mod venus_runtime_device_release;
#[path = "venus/runtime/core/venus_runtime_bitmap.rs"]
mod venus_runtime_bitmap;
#[path = "venus/runtime/core/venus_runtime_bitmap_controls.rs"]
mod venus_runtime_bitmap_controls;

#[path = "venus/runtime/bridge/venus_runtime_bridge_lookup.rs"]
mod venus_runtime_bridge_lookup;
#[path = "venus/runtime/bridge/venus_runtime_bridge_records.rs"]
mod venus_runtime_bridge_records;
#[path = "venus/runtime/bridge/venus_runtime_bridge_state.rs"]
mod venus_runtime_bridge_state;
#[path = "venus/runtime/bridge/venus_runtime_bridge_slots.rs"]
mod venus_runtime_bridge_slots;
#[path = "venus/runtime/bridge/venus_runtime_bridge_irq_ready.rs"]
mod venus_runtime_bridge_irq_ready;
#[path = "venus/runtime/bridge/venus_runtime_bridge_resource.rs"]
mod venus_runtime_bridge_resource;
#[path = "venus/runtime/bridge/venus_runtime_bridge_queue_release.rs"]
mod venus_runtime_bridge_queue_release;
#[path = "venus/runtime/bridge/venus_runtime_bridge_controller_queues.rs"]
mod venus_runtime_bridge_controller_queues;
#[path = "venus/runtime/bridge/venus_runtime_bridge_service_link.rs"]
mod venus_runtime_bridge_service_link;
#[path = "venus/runtime/bridge/venus_runtime_bridge.rs"]
pub mod venus_runtime_bridge;
#[path = "venus/common/venus_plan.rs"]
pub mod venus_plan;

#[path = "venus/runtime/core/venus_runtime_mmio.rs"]
mod venus_runtime_mmio;
#[path = "venus/runtime/core/venus_runtime_platform.rs"]
mod venus_runtime_platform;
#[path = "venus/runtime/core/venus_runtime_controller.rs"]
mod venus_runtime_controller;
#[path = "venus/runtime/core/venus_runtime_modes.rs"]
mod venus_runtime_modes;
#[path = "venus/runtime/core/venus_runtime_service_link.rs"]
mod venus_runtime_service_link;
#[path = "venus/runtime/core/venus_runtime_descriptors.rs"]
mod venus_runtime_descriptors;
#[path = "venus/runtime/core/venus_runtime_resource_ops.rs"]
mod venus_runtime_resource_ops;
#[path = "venus/runtime/core/venus_runtime_resource_table.rs"]
mod venus_runtime_resource_table;
#[path = "venus/runtime/core/venus_runtime_keys.rs"]
mod venus_runtime_keys;
#[path = "venus/runtime/core/venus_runtime_bits.rs"]
mod venus_runtime_bits;
#[path = "venus/runtime/core/venus_runtime_controller_state.rs"]
mod venus_runtime_controller_state;
#[path = "venus/runtime/core/venus_runtime_service_controller.rs"]
mod venus_runtime_service_controller;
#[path = "venus/runtime/core/venus_runtime_service_frontends.rs"]
mod venus_runtime_service_frontends;
#[path = "venus/runtime/core/venus_runtime_controller_io.rs"]
mod venus_runtime_controller_io;
#[path = "venus/runtime/core/venus_runtime_registers.rs"]
mod venus_runtime_registers;
#[path = "venus/runtime/core/venus_runtime_registers_secondary.rs"]
mod venus_runtime_registers_secondary;
#[path = "venus/runtime/core/venus_runtime_interface_lookup.rs"]
mod venus_runtime_interface_lookup;

// Agent 03: independently verified disjoint late-Venus A32 routines.
#[path = "venus/runtime/late/venus_runtime_late.rs"]
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
#[path = "cdsp/late/cdsp_late_atomic_counter.rs"]
pub mod cdsp_late_atomic_counter;
#[path = "cdsp/late/cdsp_late_bounded_compare.rs"]
pub mod cdsp_late_bounded_compare;
#[path = "cdsp/late/cdsp_late_callback_completion.rs"]
pub mod cdsp_late_callback_completion;
#[path = "cdsp/late/cdsp_late_callback_dispatch.rs"]
pub mod cdsp_late_callback_dispatch;
#[path = "cdsp/late/cdsp_late_callback_poll.rs"]
pub mod cdsp_late_callback_poll;
#[path = "cdsp/late/cdsp_late_callback_registry.rs"]
pub mod cdsp_late_callback_registry;
#[path = "cdsp/late/cdsp_late_callback_service.rs"]
pub mod cdsp_late_callback_service;
#[path = "cdsp/late/cdsp_late_callback_sweeper.rs"]
pub mod cdsp_late_callback_sweeper;
#[path = "cdsp/late/cdsp_late_controller_bank_bitset_args.rs"]
pub mod cdsp_late_controller_bank_bitset_args;
#[path = "cdsp/late/cdsp_late_controller_bank_bitset_entry.rs"]
pub mod cdsp_late_controller_bank_bitset_entry;
#[path = "cdsp/late/cdsp_late_controller_bank_bitset_fields.rs"]
pub mod cdsp_late_controller_bank_bitset_fields;
#[path = "cdsp/late/cdsp_late_controller_bank_bitset_limits.rs"]
pub mod cdsp_late_controller_bank_bitset_limits;
#[path = "cdsp/late/cdsp_late_controller_bank_bitset_scan.rs"]
pub mod cdsp_late_controller_bank_bitset_scan;
#[path = "cdsp/late/cdsp_late_controller_bank_entry.rs"]
pub mod cdsp_late_controller_bank_entry;
#[path = "cdsp/late/cdsp_late_controller_bank_guards.rs"]
pub mod cdsp_late_controller_bank_guards;
#[path = "cdsp/late/cdsp_late_controller_bank_initialize.rs"]
pub mod cdsp_late_controller_bank_initialize;
#[path = "cdsp/late/cdsp_late_controller_bank_predicates.rs"]
pub mod cdsp_late_controller_bank_predicates;
#[path = "cdsp/late/cdsp_late_controller_bitloop.rs"]
pub mod cdsp_late_controller_bitloop;
#[path = "cdsp/late/cdsp_late_controller_byte_scan.rs"]
pub mod cdsp_late_controller_byte_scan;
#[path = "cdsp/late/cdsp_late_controller_error_tail.rs"]
pub mod cdsp_late_controller_error_tail;
#[path = "cdsp/late/cdsp_late_controller_frame_setup.rs"]
pub mod cdsp_late_controller_frame_setup;
#[path = "cdsp/late/cdsp_late_controller_init_dispatch.rs"]
pub mod cdsp_late_controller_init_dispatch;
#[path = "cdsp/late/cdsp_late_controller_mask_setup.rs"]
pub mod cdsp_late_controller_mask_setup;
#[path = "cdsp/late/cdsp_late_controller_mode_guard.rs"]
pub mod cdsp_late_controller_mode_guard;
#[path = "cdsp/late/cdsp_late_controller_record_advance.rs"]
pub mod cdsp_late_controller_record_advance;
#[path = "cdsp/late/cdsp_late_controller_record_bounds.rs"]
pub mod cdsp_late_controller_record_bounds;
#[path = "cdsp/late/cdsp_late_controller_record_diagnostics.rs"]
pub mod cdsp_late_controller_record_diagnostics;
#[path = "cdsp/late/cdsp_late_controller_record_lookup.rs"]
pub mod cdsp_late_controller_record_lookup;
#[path = "cdsp/late/cdsp_late_controller_return_paths.rs"]
pub mod cdsp_late_controller_return_paths;
#[path = "cdsp/late/cdsp_late_controller_secondary_bitloop.rs"]
pub mod cdsp_late_controller_secondary_bitloop;
#[path = "cdsp/late/cdsp_late_controller_secondary_completion.rs"]
pub mod cdsp_late_controller_secondary_completion;
#[path = "cdsp/late/cdsp_late_controller_selection.rs"]
pub mod cdsp_late_controller_selection;
#[path = "cdsp/late/cdsp_late_controller_status.rs"]
pub mod cdsp_late_controller_status;
#[path = "cdsp/late/cdsp_late_controller_transfer.rs"]
pub mod cdsp_late_controller_transfer;
#[path = "cdsp/late/cdsp_late_controller_validate.rs"]
pub mod cdsp_late_controller_validate;
#[path = "cdsp/late/cdsp_late_copy_delegate.rs"]
pub mod cdsp_late_copy_delegate;
#[path = "cdsp/late/cdsp_late_gp_state_multiplier.rs"]
pub mod cdsp_late_gp_state_multiplier;
#[path = "cdsp/late/cdsp_late_large_prologue.rs"]
pub mod cdsp_late_large_prologue;
#[path = "cdsp/late/cdsp_late_mmio.rs"]
pub mod cdsp_late_mmio;
#[path = "cdsp/late/cdsp_late_null_callback.rs"]
pub mod cdsp_late_null_callback;
#[path = "cdsp/late/cdsp_late_rng_init.rs"]
pub mod cdsp_late_rng_init;
#[path = "cdsp/late/cdsp_late_rng_seed_state.rs"]
pub mod cdsp_late_rng_seed_state;
#[path = "cdsp/late/cdsp_late_rng_update.rs"]
pub mod cdsp_late_rng_update;
#[path = "cdsp/late/cdsp_late_runtime.rs"]
pub mod cdsp_late_runtime;
#[path = "cdsp/late/cdsp_late_runtime_loader.rs"]
pub mod cdsp_late_runtime_loader;
#[path = "cdsp/late/cdsp_late_runtime_more.rs"]
pub mod cdsp_late_runtime_more;
#[path = "cdsp/late/cdsp_late_seed_handoff.rs"]
pub mod cdsp_late_seed_handoff;
#[path = "venus/runtime/mid/venus_runtime_mid.rs"]
pub mod venus_runtime_mid;
#[path = "venus/runtime/core/venus_runtime_seam.rs"]
pub mod venus_runtime_seam;
