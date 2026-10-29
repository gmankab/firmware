#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build two independent Hexagon v60 modem *later segment* function groups.

The generated fragments are source-derived, not copied from the OEM firmware.
The original reference is only needed for optional SHA-pinned verification.
All offsets below are ELF file offsets; virtual and physical addresses differ.
"""
from __future__ import annotations

import argparse
import hashlib
from pathlib import Path
import struct
import subprocess
import sys
import tempfile

EXPECTED_REFERENCE_SHA256 = "6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782"
ROOT = Path(__file__).resolve().parents[1]
ATOMIC_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_atomic.s"
AUX_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_boot.s"
CONTROL_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_control.s"
AUX_INIT_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_init.s"
AUX_GUARD_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_guard.s"
AUX_STAGE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_stage.s"
AUX_MMIO_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_mmio.s"
SERVICE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_service_probe.s"
SERVICE_MMIO_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_service_mmio.s"
SERVICE_FLAGS_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_service_flags.s"
SERVICE_ALLOC_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_service_alloc.s"
AUX_HEAP_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_heap.s"
AUX_WORKER_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_worker.s"
AUX_LOOKUP_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_registry_lookup.s"
AUX_REGISTRY_CORE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_registry_core.s"
AUX_NOTIFY_SCAN_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_notify_callbacks.s"
AUX_CLEANUP_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_cleanup.s"
AUX_FACTORY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_object_factory.s"
AUX_FIND_FORMAT_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_find_format.s"
AUX_FIND_CLASS_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_find_codec_class.s"
AUX_CONTROLLER_SLOT_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_controller_slot.s"
AUX_LARGE_CONTROLLER_PREFIX_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_prefix.s"
AUX_LARGE_CONTROLLER_MID_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_mid.s"
AUX_LARGE_CONTROLLER_PHASE2_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase2.s"
AUX_LARGE_CONTROLLER_PHASE3_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase3.s"
AUX_LARGE_CONTROLLER_PHASE4_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase4.s"
AUX_LARGE_CONTROLLER_PHASE5_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase5.s"
AUX_LARGE_CONTROLLER_PHASE6_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase6.s"
AUX_LARGE_CONTROLLER_PHASE7_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase7.s"
AUX_LARGE_CONTROLLER_PHASE8_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase8.s"
AUX_LARGE_CONTROLLER_PHASE9_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase9.s"
AUX_LARGE_CONTROLLER_PHASE10_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase10.s"
AUX_LARGE_CONTROLLER_PHASE11_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase11.s"
AUX_LARGE_CONTROLLER_PHASE12_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase12.s"
AUX_LARGE_CONTROLLER_PHASE13_GATE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase13_gate.s"
AUX_LARGE_CONTROLLER_PHASE13_GAP_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase13_gap.s"
AUX_LARGE_CONTROLLER_PHASE13_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase13.s"
AUX_LARGE_CONTROLLER_PHASE14_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase14.s"
AUX_LARGE_CONTROLLER_PHASE15_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase15.s"
AUX_LARGE_CONTROLLER_PHASE16_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase16.s"
AUX_LARGE_CONTROLLER_PHASE17_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase17.s"
AUX_LARGE_CONTROLLER_PHASE18_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase18.s"
AUX_LARGE_CONTROLLER_PHASE19A_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase19a.s"
AUX_LARGE_CONTROLLER_PHASE19B_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase19b.s"
AUX_LARGE_CONTROLLER_PHASE20_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase20.s"
AUX_LARGE_CONTROLLER_PHASE21_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase21.s"
AUX_LARGE_CONTROLLER_PHASE22_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase22.s"
AUX_LARGE_CONTROLLER_PHASE23_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase23.s"
AUX_LARGE_CONTROLLER_PHASE24_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase24.s"
AUX_LARGE_CONTROLLER_PHASE25_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase25.s"
AUX_LARGE_CONTROLLER_PHASE26_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase26.s"
AUX_LARGE_CONTROLLER_PHASE27_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase27.s"
AUX_LARGE_CONTROLLER_PHASE28_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase28.s"
AUX_LARGE_CONTROLLER_PHASE29_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase29.s"
AUX_LARGE_CONTROLLER_PHASE30_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase30.s"
AUX_LARGE_CONTROLLER_PHASE31_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase31.s"
AUX_LARGE_CONTROLLER_PHASE32_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase32.s"
AUX_LARGE_CONTROLLER_PHASE33_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase33.s"
AUX_LARGE_CONTROLLER_PHASE34_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase34.s"
AUX_LARGE_CONTROLLER_PHASE35_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase35.s"
AUX_LARGE_CONTROLLER_PHASE36_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase36.s"
AUX_LARGE_CONTROLLER_PHASE37_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase37.s"
AUX_LARGE_CONTROLLER_PHASE38_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase38.s"
AUX_LARGE_CONTROLLER_PHASE39_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase39.s"
AUX_LARGE_CONTROLLER_PHASE40_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase40.s"
AUX_LARGE_CONTROLLER_PHASE41_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase41.s"
AUX_LARGE_CONTROLLER_PHASE42_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase42.s"
AUX_LARGE_CONTROLLER_PHASE43_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase43.s"
AUX_LARGE_CONTROLLER_PHASE44_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase44.s"
AUX_LARGE_CONTROLLER_PHASE45_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase45.s"
AUX_LARGE_CONTROLLER_PHASE46_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_large_controller_phase46.s"
AUX_DISPATCH_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_dispatch_helpers.s"
AUX_TYPE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_type_config.s"
AUX_NOTIFY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_controller_notify.s"
AUX_ENTRY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_entry.s"
AUX_VALIDATOR_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_input_validator.s"
AUX_RUNTIME_INIT_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_runtime_init.s"
AUX_VENEERS_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_control_veneers.s"
AUX_REGISTRATION_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_controller_registration.s"
AUX_DIAGNOSTIC_HELPERS_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_diagnostic_helpers.s"
AUX_SESSION_PREFIX_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_session_prefix.s"
AUX_SESSION_VALIDATION_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_session_validation.s"
AUX_FIND_FREE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_find_free_entry.s"
AUX_RESOLVE_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_resolve_handle.s"
AUX_SESSION_REGISTRY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_session_registry_insert.s"
AUX_INSERT_CONTROLLER_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_insert_controller.s"
AUX_REMOVE_CONTROLLER_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_remove_controller.s"
AUX_CALLBACK_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_register_callback.s"
AUX_FIND_CONTROLLER_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_find_controller.s"
AUX_RELEASE_REGISTRY_SOURCE = ROOT / "crates/firmware-tools/src/devices/xiaomi_nabu/modem_late_aux_release_registry.s"
ATOMIC_START = 0x7C000
ATOMIC_END = 0x7C040
SERVICE_START = 0x7C2EC
SERVICE_END = 0x7C324
SERVICE_MMIO_START = 0x7C324
SERVICE_MMIO_END = 0x7C3A8
SERVICE_FLAGS_START = 0x7C3A8
SERVICE_FLAGS_END = 0x7C40C
SERVICE_ALLOC_START = 0x7C40C
SERVICE_ALLOC_END = 0x7C4B0
CONTROL_START = 0x7C040
CONTROL_END = 0x7C0A0
AUX_START = 0x446000
AUX_END = 0x4460D8
AUX_DYNAMIC_START = AUX_END  # source-derived ELF interpreter/dynsym/dynstr/SysV hash
AUX_DYNAMIC_END = 0x446200
AUX_INIT_START = 0x446200
AUX_INIT_END = 0x446254
AUX_INIT_ALIGN_START = AUX_INIT_END  # checked file alignment, not executable instructions
AUX_INIT_ALIGN_END = 0x446260
AUX_GUARD_START = 0x446260
AUX_GUARD_END = 0x446278
AUX_STAGE_START = 0x446278
AUX_STAGE_END = 0x4462E0
AUX_MMIO_START = 0x4462E0
AUX_MMIO_END = 0x4462F8
AUX_HEAP_START = 0x4462F8
AUX_HEAP_END = 0x4463B4
AUX_WORKER_START = 0x4463B4
AUX_WORKER_END = 0x446420
AUX_LOOKUP_START = 0x446420
AUX_LOOKUP_END = 0x446488
AUX_REGISTRY_CORE_START = 0x446488
AUX_REGISTRY_CORE_END = 0x446650
AUX_DISPATCH_START = 0x446650
AUX_DISPATCH_END = 0x446694
AUX_TYPE_START = 0x446694
AUX_TYPE_END = 0x4467A0
AUX_NOTIFY_START = 0x4467A0
AUX_NOTIFY_END = 0x4467F0
AUX_ENTRY_START = 0x4467F0
AUX_ENTRY_END = 0x446804
AUX_VALIDATOR_START = 0x446804
AUX_VALIDATOR_END = 0x446A68
AUX_RUNTIME_INIT_START = 0x446A68
AUX_RUNTIME_INIT_END = 0x446BDC
AUX_VENEERS_START = 0x446BDC
AUX_VENEERS_END = 0x446C18
AUX_REGISTRATION_START = 0x446C18
AUX_REGISTRATION_END = 0x446E1C
AUX_DIAGNOSTIC_HELPERS_START = 0x446E1C
AUX_DIAGNOSTIC_HELPERS_END = 0x446E90
AUX_SESSION_PREFIX_START = 0x446E90
AUX_SESSION_PREFIX_END = 0x446EE8
AUX_SESSION_VALIDATION_START = 0x446EE8
AUX_SESSION_VALIDATION_END = 0x447028
AUX_FIND_FREE_START = 0x447028
AUX_FIND_FREE_END = 0x447078
AUX_RESOLVE_START = 0x447078
AUX_RESOLVE_END = 0x447100
AUX_SESSION_REGISTRY_START = 0x447100
AUX_SESSION_REGISTRY_END = 0x447234
AUX_INSERT_CONTROLLER_START = 0x447234
AUX_INSERT_CONTROLLER_END = 0x4472D0
AUX_REMOVE_CONTROLLER_START = 0x4472D0
AUX_REMOVE_CONTROLLER_END = 0x447324
AUX_CALLBACK_START = 0x447324
AUX_CALLBACK_END = 0x447388
AUX_FIND_CONTROLLER_START = 0x447388
AUX_FIND_CONTROLLER_END = 0x447410
AUX_RELEASE_REGISTRY_START = 0x447410
AUX_RELEASE_REGISTRY_END = 0x447448
AUX_NOTIFY_SCAN_START = AUX_RELEASE_REGISTRY_END
AUX_NOTIFY_SCAN_END = 0x4474B0
AUX_CLEANUP_START = AUX_NOTIFY_SCAN_END
AUX_CLEANUP_END = 0x4475C0
AUX_FACTORY_START = AUX_CLEANUP_END
AUX_FACTORY_END = 0x447608
AUX_FIND_FORMAT_START = AUX_FACTORY_END
AUX_FIND_FORMAT_END = 0x447664
AUX_FIND_CLASS_START = AUX_FIND_FORMAT_END
AUX_FIND_CLASS_END = 0x4476B8
AUX_CONTROLLER_SLOT_START = AUX_FIND_CLASS_END
AUX_CONTROLLER_SLOT_END = 0x4476BC
AUX_LARGE_CONTROLLER_PREFIX_START = AUX_CONTROLLER_SLOT_END
AUX_LARGE_CONTROLLER_PREFIX_END = 0x447724
AUX_LARGE_CONTROLLER_MID_START = AUX_LARGE_CONTROLLER_PREFIX_END
AUX_LARGE_CONTROLLER_MID_END = 0x4477FC
AUX_LARGE_CONTROLLER_PHASE2_START = AUX_LARGE_CONTROLLER_MID_END
AUX_LARGE_CONTROLLER_PHASE2_END = 0x4478A4
AUX_LARGE_CONTROLLER_PHASE3_START = AUX_LARGE_CONTROLLER_PHASE2_END
AUX_LARGE_CONTROLLER_PHASE3_END = 0x44795C
AUX_LARGE_CONTROLLER_PHASE4_START = AUX_LARGE_CONTROLLER_PHASE3_END
AUX_LARGE_CONTROLLER_PHASE4_END = 0x4479A8
AUX_LARGE_CONTROLLER_PHASE5_START = AUX_LARGE_CONTROLLER_PHASE4_END
AUX_LARGE_CONTROLLER_PHASE5_END = 0x4479D0
AUX_LARGE_CONTROLLER_PHASE6_START = AUX_LARGE_CONTROLLER_PHASE5_END
AUX_LARGE_CONTROLLER_PHASE6_END = 0x447A24
AUX_LARGE_CONTROLLER_PHASE7_START = AUX_LARGE_CONTROLLER_PHASE6_END
AUX_LARGE_CONTROLLER_PHASE7_END = 0x447AAC
AUX_LARGE_CONTROLLER_PHASE8_START = AUX_LARGE_CONTROLLER_PHASE7_END
AUX_LARGE_CONTROLLER_PHASE8_END = 0x447AE4
AUX_LARGE_CONTROLLER_PHASE9_START = AUX_LARGE_CONTROLLER_PHASE8_END
AUX_LARGE_CONTROLLER_PHASE9_END = 0x447B5C
AUX_LARGE_CONTROLLER_PHASE10_START = AUX_LARGE_CONTROLLER_PHASE9_END
AUX_LARGE_CONTROLLER_PHASE10_END = 0x447BC4
AUX_LARGE_CONTROLLER_PHASE11_START = AUX_LARGE_CONTROLLER_PHASE10_END
AUX_LARGE_CONTROLLER_PHASE11_END = 0x447C74
AUX_LARGE_CONTROLLER_PHASE12_START = AUX_LARGE_CONTROLLER_PHASE11_END
AUX_LARGE_CONTROLLER_PHASE12_END = 0x447D20
AUX_LARGE_CONTROLLER_PHASE13_GATE_START = AUX_LARGE_CONTROLLER_PHASE12_END
AUX_LARGE_CONTROLLER_PHASE13_GATE_END = 0x447D28
AUX_LARGE_CONTROLLER_PHASE13_GAP_START = AUX_LARGE_CONTROLLER_PHASE13_GATE_END
AUX_LARGE_CONTROLLER_PHASE13_GAP_END = 0x447D38
AUX_LARGE_CONTROLLER_PHASE13_START = AUX_LARGE_CONTROLLER_PHASE13_GAP_END
AUX_LARGE_CONTROLLER_PHASE13_END = 0x447D90
AUX_LARGE_CONTROLLER_PHASE14_START = AUX_LARGE_CONTROLLER_PHASE13_END
AUX_LARGE_CONTROLLER_PHASE14_END = 0x447E1C
AUX_LARGE_CONTROLLER_PHASE15_START = AUX_LARGE_CONTROLLER_PHASE14_END
AUX_LARGE_CONTROLLER_PHASE15_END = 0x447E80
AUX_LARGE_CONTROLLER_PHASE16_START = AUX_LARGE_CONTROLLER_PHASE15_END
AUX_LARGE_CONTROLLER_PHASE16_END = 0x447EE0
AUX_LARGE_CONTROLLER_PHASE17_START = AUX_LARGE_CONTROLLER_PHASE16_END
AUX_LARGE_CONTROLLER_PHASE17_END = 0x447F38
AUX_LARGE_CONTROLLER_PHASE18_START = AUX_LARGE_CONTROLLER_PHASE17_END
AUX_LARGE_CONTROLLER_PHASE18_END = 0x447FA8
AUX_LARGE_CONTROLLER_PHASE19A_START = AUX_LARGE_CONTROLLER_PHASE18_END
AUX_LARGE_CONTROLLER_PHASE19A_END = 0x447FE4
AUX_LARGE_CONTROLLER_PHASE19B_START = AUX_LARGE_CONTROLLER_PHASE19A_END
AUX_LARGE_CONTROLLER_PHASE19B_END = 0x44800C
AUX_LARGE_CONTROLLER_PHASE20_START = AUX_LARGE_CONTROLLER_PHASE19B_END
AUX_LARGE_CONTROLLER_PHASE20_END = 0x448068
AUX_LARGE_CONTROLLER_PHASE21_START = AUX_LARGE_CONTROLLER_PHASE20_END
AUX_LARGE_CONTROLLER_PHASE21_END = 0x4480B8
AUX_LARGE_CONTROLLER_PHASE22_START = AUX_LARGE_CONTROLLER_PHASE21_END
AUX_LARGE_CONTROLLER_PHASE22_END = 0x448104
AUX_LARGE_CONTROLLER_PHASE23_START = AUX_LARGE_CONTROLLER_PHASE22_END
AUX_LARGE_CONTROLLER_PHASE23_END = 0x448138
AUX_LARGE_CONTROLLER_PHASE24_START = AUX_LARGE_CONTROLLER_PHASE23_END
AUX_LARGE_CONTROLLER_PHASE24_END = 0x44817C
AUX_LARGE_CONTROLLER_PHASE25_START = AUX_LARGE_CONTROLLER_PHASE24_END
AUX_LARGE_CONTROLLER_PHASE25_END = 0x4481C4
AUX_LARGE_CONTROLLER_PHASE26_START = AUX_LARGE_CONTROLLER_PHASE25_END
AUX_LARGE_CONTROLLER_PHASE26_END = 0x448228
AUX_LARGE_CONTROLLER_PHASE27_START = AUX_LARGE_CONTROLLER_PHASE26_END
AUX_LARGE_CONTROLLER_PHASE27_END = 0x448278
AUX_LARGE_CONTROLLER_PHASE28_START = AUX_LARGE_CONTROLLER_PHASE27_END
AUX_LARGE_CONTROLLER_PHASE28_END = 0x4482CC
AUX_LARGE_CONTROLLER_PHASE29_START = AUX_LARGE_CONTROLLER_PHASE28_END
AUX_LARGE_CONTROLLER_PHASE29_END = 0x448320
AUX_LARGE_CONTROLLER_PHASE30_START = AUX_LARGE_CONTROLLER_PHASE29_END
AUX_LARGE_CONTROLLER_PHASE30_END = 0x448394
AUX_LARGE_CONTROLLER_PHASE31_START = AUX_LARGE_CONTROLLER_PHASE30_END
AUX_LARGE_CONTROLLER_PHASE31_END = 0x4483D0
AUX_LARGE_CONTROLLER_PHASE32_START = AUX_LARGE_CONTROLLER_PHASE31_END
AUX_LARGE_CONTROLLER_PHASE32_END = 0x448464
AUX_LARGE_CONTROLLER_PHASE33_START = AUX_LARGE_CONTROLLER_PHASE32_END
AUX_LARGE_CONTROLLER_PHASE33_END = 0x4484B4
AUX_LARGE_CONTROLLER_PHASE34_START = AUX_LARGE_CONTROLLER_PHASE33_END
AUX_LARGE_CONTROLLER_PHASE34_END = 0x448520
AUX_LARGE_CONTROLLER_PHASE35_START = AUX_LARGE_CONTROLLER_PHASE34_END
AUX_LARGE_CONTROLLER_PHASE35_END = 0x448580
AUX_LARGE_CONTROLLER_PHASE36_START = AUX_LARGE_CONTROLLER_PHASE35_END
AUX_LARGE_CONTROLLER_PHASE36_END = 0x4485F8
AUX_LARGE_CONTROLLER_PHASE37_START = AUX_LARGE_CONTROLLER_PHASE36_END
AUX_LARGE_CONTROLLER_PHASE37_END = 0x448628
AUX_LARGE_CONTROLLER_PHASE38_START = AUX_LARGE_CONTROLLER_PHASE37_END
AUX_LARGE_CONTROLLER_PHASE38_END = 0x448668
AUX_LARGE_CONTROLLER_PHASE39_START = AUX_LARGE_CONTROLLER_PHASE38_END
AUX_LARGE_CONTROLLER_PHASE39_END = 0x4486CC
AUX_LARGE_CONTROLLER_PHASE40_START = AUX_LARGE_CONTROLLER_PHASE39_END
AUX_LARGE_CONTROLLER_PHASE40_END = 0x44873C
AUX_LARGE_CONTROLLER_PHASE41_START = AUX_LARGE_CONTROLLER_PHASE40_END
AUX_LARGE_CONTROLLER_PHASE41_END = 0x44878C
AUX_LARGE_CONTROLLER_PHASE42_START = AUX_LARGE_CONTROLLER_PHASE41_END
AUX_LARGE_CONTROLLER_PHASE42_END = 0x4487E0
AUX_LARGE_CONTROLLER_PHASE43_START = AUX_LARGE_CONTROLLER_PHASE42_END
AUX_LARGE_CONTROLLER_PHASE43_END = 0x44883C
AUX_LARGE_CONTROLLER_PHASE44_START = AUX_LARGE_CONTROLLER_PHASE43_END
AUX_LARGE_CONTROLLER_PHASE44_END = 0x448888
AUX_LARGE_CONTROLLER_PHASE45_START = AUX_LARGE_CONTROLLER_PHASE44_END
AUX_LARGE_CONTROLLER_PHASE45_END = 0x4488CC
AUX_LARGE_CONTROLLER_PHASE46_START = AUX_LARGE_CONTROLLER_PHASE45_END
AUX_LARGE_CONTROLLER_PHASE46_END = 0x448938
MARKER_MASK = 3 << 14
CONTINUE_MARKER = 1 << 14
END_MARKER = 3 << 14


def extract_text(obj: bytes) -> bytes:
    """Return exactly one executable Hexagon ELF32 little-endian PROGBITS."""
    if not obj.startswith(b"\x7fELF\x01\x01") or len(obj) < 52:
        raise ValueError("expected little-endian ELF32 assembler object")
    machine = struct.unpack_from("<H", obj, 18)[0]
    if machine != 164:
        raise ValueError(f"assembler object e_machine={machine}, expected Hexagon 164")
    shoff = struct.unpack_from("<I", obj, 32)[0]
    shentsize, shnum = struct.unpack_from("<HH", obj, 46)
    if shentsize != 40 or not shnum or shoff + shnum * shentsize > len(obj):
        raise ValueError("invalid assembler section directory")
    sections = [struct.unpack_from("<10I", obj, shoff + i * shentsize) for i in range(shnum)]
    text = [s for s in sections if s[1] == 1 and (s[2] & 4) and s[5]]
    if len(text) != 1:
        raise ValueError(f"expected one nonempty executable PROGBITS section, got {len(text)}")
    start, length = text[0][4:6]
    if start + length > len(obj):
        raise ValueError("assembler .text section exceeds object file")
    return obj[start : start + length]


# Auxiliary ELF32 symbol metadata is emitted from *its meaning*, not by
# pasting a reference slice. Hexagon executable code is assembled separately.
# ELF32_SYM fields: symbol, virtual address, size, type/binding, shndx.
AUX_EXPORTS = (
    ("dlsym",       0x1E400E90, 0x198, 2, 7),
    ("dladdr",      0x1E401100, 0x134, 2, 7),
    ("dlerror",     0x1E400650, 0x00C, 2, 7),
    ("dlopen",      0x1E400C18, 0x204, 2, 7),
    ("dlclose",     0x1E4003B4, 0x06C, 2, 7),
    ("dlinfo",      0x1E400694, 0x100, 2, 7),
    ("_rtld_debug", 0x1E419F1C, 0x010, 1, 15),
)


def elf32_sysv_hash(name: str) -> int:
    """ELF SysV .hash algorithm (ELF ABI), using ASCII symbol names."""
    result = 0
    for c in name.encode("ascii"):
        result = (result << 4) + c
        high = result & 0xF0000000
        if high:
            result ^= high >> 24
        result &= ~high
    return result & 0xFFFFFFFF


def make_elf32_sysv_hash(export_names: tuple[str, ...], buckets: int = 8) -> bytes:
    """ELF32 SysV hash: two sizes, buckets, then collision-linked chains."""
    if not export_names or len(set(export_names)) != len(export_names):
        raise ValueError("ELF hash requires distinct symbols")
    if not (1 <= buckets <= 65535):
        raise ValueError("ELF hash bucket count must be in 1..65535")
    chain = [0] * (len(export_names) + 1)  # zero ELF symbol is STN_UNDEF
    heads = [0] * buckets
    for index, name in enumerate(export_names, 1):
        bucket = elf32_sysv_hash(name) % buckets
        chain[index] = heads[bucket]
        heads[bucket] = index
    return struct.pack("<" + "I" * (2 + buckets + len(chain)),
                       buckets, len(chain), *heads, *chain)


def emit_auxiliary_elf_symbols() -> bytes:
    """Produce 296 source-authored ELF metadata bytes for auxiliary library.

    Layout: 36-byte interpreter string/padding; seven ELF32 Sym entries,
    56-byte NUL-prefixed name table, 8-byte hash alignment, 72-byte ELF SysV
    hash table, and 16-byte ending alignment. No executable code is copied.
    """
    interp = b"/usr/lib/libc.so.1\0"
    if len(interp) > 0x24:
        raise ValueError("auxiliary ELF interpreter name overflows reserved field")
    out = bytearray(interp.ljust(0x24, b"\0"))
    names = bytearray(b"\0")
    records=[]
    for name, address, size, kind, shndx in AUX_EXPORTS:
        string_offset = len(names)
        names.extend(name.encode("ascii") + b"\0")
        if not (0 < string_offset < 0x10000 and 0 <= address <= 0xFFFFFFFF
                and 0 < size <= 0xFFFFFFFF and kind in (1, 2)
                and shndx in (7, 15)):
            raise ValueError("invalid auxiliary ELF32 symbol value")
        records.append(struct.pack("<IIIBBH", string_offset, address, size,
                                   (1 << 4) | kind, 0, shndx))
    out.extend(b"".join(records))
    out.extend(names)
    while len(out) % 8:
        out.append(0)  # ELF hash table 8-byte alignment, not an instruction
    out.extend(make_elf32_sysv_hash(tuple(x[0] for x in AUX_EXPORTS)))
    out.extend(b"\0" * 16)  # file-backed linker section alignment
    if len(out) != AUX_DYNAMIC_END - AUX_DYNAMIC_START:
        raise ValueError(f"auxiliary ELF32 metadata must be {AUX_DYNAMIC_END - AUX_DYNAMIC_START} bytes, got {len(out)}")
    return bytes(out)


def emit_auxiliary_code_alignment() -> bytes:
    """Twelve file-backed zero bytes after init; not Hexagon opcodes."""
    if AUX_INIT_ALIGN_START != AUX_INIT_END or AUX_INIT_ALIGN_END-AUX_INIT_END != 12:
        raise ValueError("auxiliary init linker alignment changed")
    return bytes(AUX_INIT_ALIGN_END-AUX_INIT_END)


def restore_atomic_packet_order(assembled: bytes) -> bytes:
    """Correct LLVM6's legal, but non-byte-identical scheduling at 0x7c010.

    Original packet source states the predicated ADD *before* the compound
    `cmp.eq; if(p0.new)jump`. LLVM6 places that compound instruction first.
    Class, marker and register checks reject changes before restoring only
    the original source order and packet-end marker bits. No reference opcode
    bytes are inserted here.
    """
    if len(assembled) != ATOMIC_END - ATOMIC_START or len(assembled) % 4:
        raise ValueError(f"late modem atomics require {ATOMIC_END - ATOMIC_START} bytes")
    words = list(struct.unpack("<" + "I" * (len(assembled) // 4), assembled))
    branch, predicated_add = words[4:6]
    if branch >> 24 != 0x10 or predicated_add >> 24 != 0x74:
        raise ValueError("expected compound comparison-branch followed by predicated ADD")
    if branch & MARKER_MASK != CONTINUE_MARKER or predicated_add & MARKER_MASK != END_MARKER:
        raise ValueError("unexpected Hexagon packet parse markers at modem 0x7c010")
    words[4:6] = [(predicated_add & ~MARKER_MASK) | CONTINUE_MARKER,
                  (branch & ~MARKER_MASK) | END_MARKER]
    return struct.pack("<" + "I" * len(words), *words)


def restore_controller_packet_order(assembled: bytes) -> bytes:
    """Restore two compare+immediate packets and a three-word control packet.

    LLVM6 schedulers move new-value branch consumers ahead of the preceding
    extended immediate/assignment and terminate the nearby parameter move
    prematurely. Each rewrite is checked against documented Hexagon opcode
    *classes* and parse markers; no OEM opcode data is used.
    """
    if len(assembled) != CONTROL_END - CONTROL_START:
        raise ValueError("late modem controller requires 96 source-built bytes")
    words = list(struct.unpack("<" + "I" * (len(assembled)//4),assembled))
    for first, value_class in [(0x14//4, 0x78), (0x4c//4, 0x7e)]:
        pred, extender, value = words[first:first+3]
        if [w >> 24 for w in (pred,extender,value)] != [0x10,0x0c,value_class]:
            raise ValueError(f"controller conditional/extended packet classes changed at {CONTROL_START + first*4:#x}")
        if [w & MARKER_MASK for w in (pred,extender,value)] != [CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
            raise ValueError(f"controller conditional/extended packet markers changed at {CONTROL_START + first*4:#x}")
        words[first:first+3] = [extender,
                               (value & ~MARKER_MASK) | CONTINUE_MARKER,
                               (pred & ~MARKER_MASK) | END_MARKER]
    # With a valid *source-assembled* three-word sequence, change only the
    # MOV's end-of-packet to continue so it shares the following duplex
    # memory-load/immediate instruction, as the original producer ordering.
    extender, move, duplex = words[0x2c//4:0x38//4]
    if [w >> 24 for w in (extender,move,duplex)] != [0x0c,0x78,0x48]:
        raise ValueError("controller extended constant and duplex instruction classes changed")
    if [w & MARKER_MASK for w in (extender,move,duplex)] != [CONTINUE_MARKER,END_MARKER,0]:
        raise ValueError("unexpected controller param-load packet markers")
    words[0x30//4]=(move & ~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<"+"I"*len(words),*words)


def restore_aux_init_packet(assembled: bytes) -> bytes:
    """Join one scalar R2 init with its immediately following duplex packet.

    LLVM6 ends the scalar instruction packet; the architectural source
    specifies the same three zero register writes in one packet. The duplex
    uses the Hexagon 00 parse form, so only the scalar parse marker changes.
    """
    if len(assembled) != AUX_INIT_END - AUX_INIT_START:
        raise ValueError("modem auxiliary initializer must be 84 bytes")
    words = list(struct.unpack("<" + "I" * (len(assembled) // 4), assembled))
    scalar, duplex = words[0x18//4 : 0x20//4]
    if scalar >> 24 != 0x78 or duplex >> 24 != 0x28:
        raise ValueError("auxiliary startup scalar/duplex instruction class mismatch")
    if (scalar & MARKER_MASK) != END_MARKER or (duplex & MARKER_MASK) != 0:
        raise ValueError("auxiliary startup packet marker mismatch")
    words[0x18//4] = (scalar & ~MARKER_MASK) | CONTINUE_MARKER
    return struct.pack("<" + "I" * len(words), *words)


def restore_aux_stage_packets(assembled: bytes) -> bytes:
    """Re-create checked Hexagon packet grouping for auxiliary resource setup.

    LLVM6 disallows two branch-class instructions in the same packet and
    terminates three other packets earlier than the original target compiler.
    Each source-assembled instruction has matching semantics and relative
    displacement. Recombine only parse-marker bits, after checking classes.
    """
    if len(assembled) != AUX_STAGE_END - AUX_STAGE_START:
        raise ValueError("modem auxiliary stage must compile to 104 bytes")
    words = list(struct.unpack("<"+"I"*(len(assembled)//4),assembled))
    rewrites = [
        (0x1c, 0xB0, END_MARKER, CONTINUE_MARKER),  # scalar frame-relative pointer
        (0x28, 0x3C, END_MARKER, CONTINUE_MARKER),  # first extended MMIO write
        (0x44, 0x5A, END_MARKER, CONTINUE_MARKER),  # call and immediately extended args
        (0x5c, 0x5A, CONTINUE_MARKER, END_MARKER),  # stand-alone worker call
        (0x60, 0x78, END_MARKER, CONTINUE_MARKER),  # r0 clears before return
    ]
    for off, opcode, old_marker, new_marker in rewrites:
        index=off//4
        original=words[index]
        if original>>24 != opcode or original & MARKER_MASK != old_marker:
            raise ValueError(f"auxiliary stage instruction class or packet marker changed at {AUX_STAGE_START+off:#x}")
        words[index]=(original & ~MARKER_MASK)|new_marker
    if words[0x20//4]>>24 != 0x28 or words[0x20//4]&MARKER_MASK != 0:
        raise ValueError("auxiliary stage expected a valid final duplex instruction")
    return struct.pack("<"+"I"*len(words),*words)


def restore_service_probe_packets(assembled: bytes) -> bytes:
    """Preserve two source-semantic instruction packets combined by target compiler.

    LLVM6 prematurely ends a call before an extended absolute register load,
    and an absolute register load before another extender. The target compiler
    uses two longer legal packets. Refuse unexpected instruction types/parse
    fields and alter *only* the two terminating Hexagon parse bits.
    """
    if len(assembled) != SERVICE_END - SERVICE_START:
        raise ValueError("modem late service probe requires 56 compiled bytes")
    words=list(struct.unpack("<"+"I"*(len(assembled)//4),assembled))
    for at,opcode,following in [(0x14,0x5a,0x0c),(0x24,0x78,0x0c)]:
        i=at//4
        if (words[i]>>24,words[i+1]>>24)!=(opcode,following):
            raise ValueError(f"late modem service probe instruction classes changed at {SERVICE_START+at:#x}")
        if (words[i]&MARKER_MASK,words[i+1]&MARKER_MASK)!=(END_MARKER,CONTINUE_MARKER):
            raise ValueError(f"late modem service probe packet markers changed at {SERVICE_START+at:#x}")
        words[i]=(words[i]&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<"+"I"*len(words),*words)


def restore_service_mmio_packets(assembled: bytes) -> bytes:
    """Join two controller calls to the following immediate-extended move.

    The original compiler groups calls with the next constant-extended
    address declaration. LLVM6 terminates the calls separately; check both
    classes and markers then continue the packet without touching opcodes.
    """
    if len(assembled) != SERVICE_MMIO_END-SERVICE_MMIO_START:
        raise ValueError("late modem service MMIO requires 132 bytes")
    words=list(struct.unpack("<"+"I"*(len(assembled)//4),assembled))
    for off,opcode in [(0x14,0x5A),(0x20,0x5B)]:
        i=off//4
        if [words[i]>>24,words[i+1]>>24]!=[opcode,0x0c]:
            raise ValueError(f"MMIO service call/extender opcode classes changed at {SERVICE_MMIO_START+off:#x}")
        if [words[i]&MARKER_MASK,words[i+1]&MARKER_MASK]!=[END_MARKER,CONTINUE_MARKER]:
            raise ValueError(f"MMIO service packet markers changed at {SERVICE_MMIO_START+off:#x}")
        words[i]=(words[i]&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<"+"I"*len(words),*words)


def restore_service_flag_packets(assembled: bytes) -> bytes:
    """Preserve a producer/consumer Hexagon V60 packet for flag access.

    LLVM6 schedules the compound compare/branch ahead of its two-slot
    predicated register-assignment and constant extender. The original
    program instead orders extender, predicated move, comparison branch.
    Reorder only those three independently assembled instructions and set
    legal packet markers after checking opcode classes and source grouping.
    This NEVER inserts reference words into the generated fragment.
    """
    if len(assembled) != SERVICE_FLAGS_END - SERVICE_FLAGS_START:
        raise ValueError("service flag helpers require exactly 100 source bytes")
    words = list(struct.unpack("<25I", assembled))
    i = (0x7C3E8 - SERVICE_FLAGS_START) // 4
    cmp_jump, extender, predicated = words[i:i+3]
    if [x >> 24 for x in (cmp_jump, extender, predicated)] != [0x10, 0x0e, 0x7e]:
        raise ValueError("unexpected Hexagon service flag predicate opcode classes")
    if [x & MARKER_MASK for x in (cmp_jump, extender, predicated)] != [
        CONTINUE_MARKER, CONTINUE_MARKER, END_MARKER
    ]:
        raise ValueError("unexpected Hexagon service flag packet parse markers")
    words[i:i+3] = [extender,
                    (predicated & ~MARKER_MASK) | CONTINUE_MARKER,
                    (cmp_jump & ~MARKER_MASK) | END_MARKER]
    return struct.pack("<25I", *words)


def restore_service_alloc_return_packet(assembled: bytes) -> bytes:
    """Preserve source-ordered return register move in compound frame restore.

    LLVM6 terminates an otherwise valid standalone `r0=r16` packet before
    the following 64-bit frame-load and `dealloc_return`. Target Hexagon V60
    uses the scalar move as a continuation instead. Check both instruction
    classes and source parse markers before changing this *single* field.
    Neither external OEM bytes nor any replacement instruction are used.
    """
    if len(assembled) != SERVICE_ALLOC_END-SERVICE_ALLOC_START:
        raise ValueError("service allocator requires exactly 164 source bytes")
    words = list(struct.unpack("<41I", assembled))
    index=0x98//4
    move, load_pair, ret=words[index:index+3]
    if [x>>24 for x in (move,load_pair,ret)]!=[0x70,0x3e,0x96]:
        raise ValueError("service allocator return instruction classes changed")
    if [x&MARKER_MASK for x in (move,load_pair,ret)]!=[
        END_MARKER,0,END_MARKER
    ]:
        raise ValueError("service allocator return packet markers changed")
    words[index]=(move & ~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<41I", *words)


def restore_aux_heap_predicate_packet(assembled: bytes) -> bytes:
    """Preserve new-register compare/branch source order in heap slot scan.

    LLVM6 hoists the compound new-value compare/branch ahead of the scalar
    `r2=#0` that it tests, but the original target packer put the producer
    first. Validate opcode classes, packet marker fields and 188-byte size,
    then swap exactly the two independently assembled instructions.
    """
    if len(assembled) != AUX_HEAP_END-AUX_HEAP_START:
        raise ValueError("auxiliary heap allocator requires exactly 188 bytes")
    words=list(struct.unpack("<47I",assembled))
    idx=0x6c//4
    branch, zero = words[idx:idx+2]
    if (branch>>24, zero>>24)!=(0x10,0x78):
        raise ValueError("heap new-value branch and producer opcode class mismatch")
    if (branch&MARKER_MASK,zero&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("heap new-value producer packet markers changed")
    words[idx:idx+2]=[(zero&~MARKER_MASK)|CONTINUE_MARKER,
                      (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack("<47I",*words)


def restore_aux_worker_packets(assembled: bytes) -> bytes:
    """Restore original Hexagon packet position and PC-relative jump semantics.

    LLVM6 emits scalar -1 setup as a separate packet even though target
    Hexagon V60 pairs it with the following unconditional jump. The jump
    packet-relative displacement differs by one Hexagon branch unit for the
    target assembler's split packet address convention; retain the named
    `.Ldone` source target, validate the compiler's original field, and
    adjust *only* its immediate field by two encoded steps (8 bytes).
    A subsequent service call is likewise part of the following packet.
    """
    if len(assembled) != AUX_WORKER_END-AUX_WORKER_START:
        raise ValueError("auxiliary worker must assemble into exactly 108 bytes")
    words=list(struct.unpack("<27I",assembled))
    init_idx=0x24//4
    mov,jmp=words[init_idx:init_idx+2]
    if (mov>>24,jmp>>24)!=(0x78,0x58):
        raise ValueError("aux worker const/jump ISA classes changed")
    if (mov & MARKER_MASK,jmp & MARKER_MASK)!=(END_MARKER,END_MARKER):
        raise ValueError("aux worker const/jump packet markers changed")
    if jmp&0x3f != 0x1e:
        raise ValueError("aux worker PC-relative branch immediate no longer matches named target")
    words[init_idx] = (mov & ~MARKER_MASK) | CONTINUE_MARKER
    words[init_idx+1] = (jmp&~0x3f) | ((jmp&0x3f) + 2)
    call_idx=0x3c//4
    call,next_combine=words[call_idx:call_idx+2]
    if (call>>24,next_combine>>24)!=(0x5a,0x73):
        raise ValueError("aux worker controller update/combine instruction classes changed")
    if (call&MARKER_MASK,next_combine&MARKER_MASK)!=(END_MARKER,CONTINUE_MARKER):
        raise ValueError("aux worker controller update packet markers changed")
    words[call_idx] = (call&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<27I",*words)


def restore_aux_lookup_return_packet(assembled: bytes) -> bytes:
    """Rejoin a valid scalar return move with an adjacent duplex restore.

    LLVM6 emits the `r0=r16` instruction in an isolated packet. The original
    controller's return path has a continuation marker before a compound
    64-bit register load and `dealloc_return`. Only this parse marker changes
    after ISA class/length validation; code bytes are source-assembled.
    """
    if len(assembled) != AUX_LOOKUP_END-AUX_LOOKUP_START:
        raise ValueError("aux registry lookup requires exactly 104 source bytes")
    words=list(struct.unpack("<26I",assembled))
    i=0x60//4
    move,stack_restore=words[i:i+2]
    if [v>>24 for v in (move,stack_restore)]!=[0x70,0x3e]:
        raise ValueError("aux registry return opcode classes changed")
    if [v&MARKER_MASK for v in (move,stack_restore)]!=[
        END_MARKER,0
    ]:
        raise ValueError("aux registry return packet marker mismatch")
    words[i]=(move&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack("<26I",*words)


def restore_aux_type_return_packets(assembled: bytes) -> bytes:
    """Join error-return constants with following Hexagon frame restorations.

    Two codec format branches return -1 after a failed allocation. LLVM6
    marks the immediate move as a complete packet, whereas the original
    target scheduler uses it as a continuation preceding a duplex
    memory-restore/dealloc return. Validate both instruction pairs and
    reconstruct only the parse-marker bits; the three trailing `nop`
    instructions are explicit source and are retained as alignment code.
    """
    if len(assembled) != AUX_TYPE_END-AUX_TYPE_START:
        raise ValueError("aux codec-type routine must produce 268 bytes")
    words=list(struct.unpack('<67I',assembled))
    for at in (0x34,0x80):
        i=at//4
        scalar,restore=words[i:i+2]
        if (scalar>>24,restore>>24)!=(0x78,0x3e):
            raise ValueError(f"aux codec-type early return opcode classes changed at {at:#x}")
        if (scalar&MARKER_MASK,restore&MARKER_MASK)!=(END_MARKER,0):
            raise ValueError(f"aux codec-type early return packet markers changed at {at:#x}")
        words[i]=(scalar&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<67I',*words)


def restore_aux_controller_notify_packets(assembled: bytes) -> bytes:
    """Restore target producer-order packets for controller notification.

    LLVM6 moves the branch consumer before its `p0` comparison and source
    move, and moves the frame reload before its extended MMIO store. The
    target toolchain keeps each three-word group in explicit source order.
    Only ISA-checked instruction ordering and 2-bit parse markers change.
    No private OEM opcode word is read or copied into the generated result.
    """
    if len(assembled)!=AUX_NOTIFY_END-AUX_NOTIFY_START:
        raise ValueError("auxiliary controller notifier must emit exactly 80 bytes")
    words=list(struct.unpack('<20I',assembled))
    groups=[
       (0x10//4,[0x5c,0xf2,0x70]),
       (0x38//4,[0x91,0x01,0x48]),
    ]
    for i,classes in groups:
        seq=words[i:i+3]
        if [x>>24 for x in seq]!=classes:
            raise ValueError(f"aux notifier packet classes differ at offset {i*4:#x}")
        if [x&MARKER_MASK for x in seq]!=[
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER
        ]:
            raise ValueError(f"aux notifier packet marker mismatch at offset {i*4:#x}")
        consumer,first,second=seq
        words[i:i+3]=[first,(second&~MARKER_MASK)|CONTINUE_MARKER,
                      (consumer&~MARKER_MASK)|END_MARKER]
    return struct.pack('<20I',*words)


def restore_aux_validator_predicate_packets(assembled: bytes) -> bytes:
    """Place new-value producer comparisons before their branch consumers.

    LLVM6 packet scheduling sometimes emits the compound predicated branch
    before its generating compare in the same legal V60 packet. The target
    assembler emitted the reverse order, and parse markers signal the same
    one-packet ordering. Check lengths, opcode classes and parse fields
    before rearranging the *independently assembled* operations. No source
    firmware instruction bytes are imported.
    """
    if len(assembled) != AUX_VALIDATOR_END - AUX_VALIDATOR_START:
        raise ValueError("auxiliary validator expects exactly 612 source bytes")
    words=list(struct.unpack('<'+'I'*(len(assembled)//4),assembled))
    for relative in (0x4468F0 - AUX_VALIDATOR_START, 0x446994 - AUX_VALIDATOR_START):
        i=relative//4
        branch, comparison=words[i:i+2]
        if (branch>>24,comparison>>24) != (0x5c,0x75):
            raise ValueError(f"validator predicate instruction classes changed at {AUX_VALIDATOR_START+relative:#x}")
        old=(branch & MARKER_MASK,comparison & MARKER_MASK)
        if relative==0x4468F0-AUX_VALIDATOR_START:
            if old != (CONTINUE_MARKER,CONTINUE_MARKER):
                raise ValueError("validator first predicate packet markers changed")
        elif old!=(CONTINUE_MARKER,END_MARKER):
            raise ValueError("validator second predicate packet markers changed")
        words[i]=(comparison & ~MARKER_MASK)|CONTINUE_MARKER
        words[i+1]=(branch & ~MARKER_MASK)|(END_MARKER if relative==0x446994-AUX_VALIDATOR_START else CONTINUE_MARKER)
    return struct.pack('<'+'I'*len(words),*words)


def restore_aux_runtime_init_packets(assembled: bytes) -> bytes:
    """Preserve producer-first packets and one late controller-state store.

    LLVM6 schedules two single-register parameter producers after their
    predicated uses. It also places an independent memory load before the
    accompanying 32-bit absolute-address write. Validate the original
    SOURCE-assembled operation classes and packet markers, then reorder
    them to the target compiler's legal Hexagon V60 packet order. Each
    rewrite manipulates only assembled instructions and 2-bit parse fields.
    """
    if len(assembled) != AUX_RUNTIME_INIT_END - AUX_RUNTIME_INIT_START:
        raise ValueError("aux runtime initializer requires 372 source bytes")
    words=list(struct.unpack('<'+'I'*(len(assembled)//4),assembled))
    for off,branch_class,param_class in [(0x1c,0x5c,0x7e),(0x50,0x10,0x78)]:
        i=off//4
        branch,parameter=words[i:i+2]
        if (branch>>24,parameter>>24)!=(branch_class,param_class):
            raise ValueError("aux runtime predicate opcode classes changed")
        if (branch&MARKER_MASK,parameter&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
            raise ValueError("aux runtime predicate packet markers changed")
        words[i]=(parameter&~MARKER_MASK)|CONTINUE_MARKER
        words[i+1]=(branch&~MARKER_MASK)|END_MARKER
    i=0xe4//4
    early_load,extender,write=words[i:i+3]
    if (early_load>>24,extender>>24,write>>24)!=(0x91,0x01,0x48):
        raise ValueError("aux runtime MMIO packet opcode classes changed")
    if (early_load&MARKER_MASK,extender&MARKER_MASK,write&MARKER_MASK)!=       (CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError("aux runtime MMIO packet markers changed")
    words[i:i+3]=[extender,(write&~MARKER_MASK)|CONTINUE_MARKER,
                  (early_load&~MARKER_MASK)|END_MARKER]
    return struct.pack('<'+'I'*len(words),*words)


def compact_aux_controller_veneer_duplex(assembled: bytes) -> bytes:
    """Reassemble a scalar move+jump as one documented Hexagon duplex.

    LLVM6 expands the symbolic extended tail call into an immediate
    extender, scalar branch, and separate `r3=#0` move. Hexagon V60 also
    defines a 32-bit duplex encoding of `r3=#0 ; jump`, selected by the
    target compiler. Validate every instruction class, register and marker
    before computing the duplex from the SOURCE-assembled operand fields.
    The following tail-call packet is regrouped only by its parse markers.
    """
    if len(assembled) != AUX_VENEERS_END-AUX_VENEERS_START + 4:
        raise ValueError("auxiliary veneers expect 64 source-assembled bytes")
    words=list(struct.unpack('<'+'I'*(len(assembled)//4),assembled))
    # Source offset 0x24: extender, 0x28: scalar branch, 0x2c: scalar r3=#0.
    a=0x24//4
    extender,jump,move=words[a:a+3]
    if extender>>24 != 0x0f or jump>>24 != 0x58 or move>>24 != 0x78:
        raise ValueError("auxiliary veneer extender/branch/move opcode classes changed")
    if (extender&MARKER_MASK,jump&MARKER_MASK,move&MARKER_MASK) !=       (CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError("auxiliary veneer scalar packet markers changed")
    # A2_tfrsi r3,#0: register is low nibble, immediate and other bits zero.
    if (move&~MARKER_MASK) != (0x78000000 | 3):
        raise ValueError("auxiliary veneer source is not r3=#0")
    duplex=(0x16<<24)|(3<<16)|((jump&0x3fff)|END_MARKER)
    words[a:a+3]=[extender,duplex]
    # The second tail-call packet starts at output offset 0x34 after the
    # four-byte shrink, with a packed `r3:2=combine(0,0)` and branch.
    b=0x34//4
    branch,pair=words[b:b+2]
    if (branch>>24,pair>>24)!=(0x59,0x7c):
        raise ValueError("auxiliary second veneer opcode classes changed")
    if (branch&MARKER_MASK,pair&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("auxiliary second veneer packet markers changed")
    if (pair&~MARKER_MASK)!=0x7c000002:
        raise ValueError("auxiliary veneer source is not zero pair")
    words[b:b+2]=[(pair&~MARKER_MASK)|CONTINUE_MARKER,
                   (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<'+'I'*len(words),*words)


def restore_aux_registration_new_value_packet(assembled: bytes) -> bytes:
    """Move predicated flag write before its new-value comparison/branch.

    The source explicitly requests the original producer/consumer ordering.
    LLVM6 reschedules the compare before the conditional move, still in one
    packet. Reorder these two *assembler-emitted* instructions and retain
    their position-specific packet markers. No OEM instructions are copied.
    """
    if len(assembled) != AUX_REGISTRATION_END-AUX_REGISTRATION_START:
        raise ValueError("aux registration expects 516 source-assembled bytes")
    words=list(struct.unpack('<'+'I'*(len(assembled)//4),assembled))
    i=(0x446D04-AUX_REGISTRATION_START)//4
    compare,flag_write=words[i:i+2]
    if (compare>>24,flag_write>>24)!=(0x11,0xfd):
        raise ValueError("aux registration predicate opcode classes changed")
    if (compare&MARKER_MASK,flag_write&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("aux registration predicate packet markers changed")
    words[i:i+2]=[(flag_write&~MARKER_MASK)|CONTINUE_MARKER,
                  (compare&~MARKER_MASK)|END_MARKER]
    return struct.pack('<'+'I'*len(words),*words)


def encode_v60_framekey(rd: int, *, end_packet: bool = False) -> int:
    """A32-independent Hexagon V60 ``Rd = framekey`` ISA field encoder.

    Hexagon control-register instruction class 0x6a11 reads the frame-key
    special register; the 5-bit destination field has valid GPRs R1–R7 or
    R16–R23.  LLVM6's MC *disassembler* decodes this class, but its assembler
    does not accept the mnemonic. Build the class + GPR + packet parse bits
    from operands and verify the generated instruction against the reference,
    rather than importing a private OEM opcode word.
    """
    if 1 <= rd <= 7:
        encoded_rd=rd
    elif 16 <= rd <= 23:
        encoded_rd=(rd-16)|0x10
    else:
        raise ValueError("framekey Rd must be R1–R7 or R16–R23")
    # Stable architectural opcode bits, not OEM firmware bytes.
    return 0x6a110000 | encoded_rd | (END_MARKER if end_packet else CONTINUE_MARKER)


def restore_aux_session_validation_packets(assembled: bytes) -> bytes:
    """Reconcile checked Hexagon V60 source packets and encode framekey.

    The first four bytes of the independent LLVM6 source are an *alignment
    placeholder* solely because LLVM6 cannot encode `r3 = framekey` itself.
    Replace the zero placeholder with a public-ISA-encoded control-register
    instruction, then only reorder verified independently assembled packets,
    preserving source operand fields and updating short-branch immediates
    when the packet scheduler moves the source label within the same packet.
    """
    if len(assembled) != AUX_SESSION_VALIDATION_END-AUX_SESSION_VALIDATION_START:
        raise ValueError("session validation expects 320 independently assembled bytes")
    words=list(struct.unpack('<80I',assembled))
    if words[0]!=0:
        raise ValueError("framekey layout slot must be zero before ISA encoding")
    words[0]=encode_v60_framekey(3)

    def reassign_packet(offset, old_classes, old_markers, indexes):
        i=offset//4
        group=words[i:i+len(indexes)]
        if [w>>24 for w in group]!=old_classes:
            raise ValueError(f"session packet ISA opcode classes changed at {offset:#x}")
        if [w&MARKER_MASK for w in group]!=old_markers:
            raise ValueError(f"session packet parser bits changed at {offset:#x}")
        n=len(indexes)
        words[i:i+n]=[(group[source]&~MARKER_MASK)|
                         (END_MARKER if dest==n-1 else CONTINUE_MARKER)
                         for dest,source in enumerate(indexes)]

    # The target compiler puts source value producers ahead of their
    # compare-and-branch consumers. LLVM6 otherwise shuffles same-packet ops.
    reassign_packet(0x58,[0x10,0xfd],[CONTINUE_MARKER,END_MARKER],[1,0])
    reassign_packet(0x78,[0x5c,0x75,0xfd],
                    [CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER],[1,2,0])
    reassign_packet(0x84,[0x5c,0x75],[CONTINUE_MARKER,END_MARKER],[1,0])
    reassign_packet(0xa4,[0x46,0x58],[END_MARKER,END_MARKER],[1,0])
    # At 0xa4 the original jump shares its packet with the conditional store
    # that follows; the LLVM split boundary is corrected after opcode checks.
    words[0xa4//4]=(words[0xa4//4]&~MARKER_MASK)|CONTINUE_MARKER
    for off in (0xac,0xcc):
        i=off//4
        x=words[i]
        if x>>24!=0xb0 or x&MARKER_MASK!=END_MARKER:
            raise ValueError(f"session stack-pointer add/next packet differs at {off:#x}")
        words[i]=(x&~MARKER_MASK)|CONTINUE_MARKER
    i=0x118//4
    if words[i]>>24!=0x50 or words[i]&MARKER_MASK!=END_MARKER:
        raise ValueError("session callr opcode/parse state changed")
    words[i]=(words[i]&~MARKER_MASK)|CONTINUE_MARKER
    i=0x11c//4
    call,extender,packed=words[i:i+3]
    if [x>>24 for x in (call,extender,packed)]!=[0x5b,0x01,0x6b]:
        raise ValueError("session final diagnostic call opcode classes changed")
    if [x&MARKER_MASK for x in (call,extender,packed)]!=[CONTINUE_MARKER,CONTINUE_MARKER,0]:
        raise ValueError("session final diagnostic packet markers changed")
    words[i:i+3]=[extender,packed,call]

    # Adjust only ISA-defined short branch displacement fields.  Packet
    # boundaries from LLVM6 shift these named-target relocations by 2/4
    # encoded units compared with the target compiler's grouping.
    for offset, opcode, before, delta in [
        (0x48,0x16,0x34,+2),
        (0xa4,0x58,0x26,+4),
        (0xfc,0x58,0x1a,+2),
        (0x108,0x24,0x0e,+4),
        (0x124,0x5b,0x0e,-4),
    ]:
        i=offset//4
        x=words[i]
        if x>>24!=opcode or x&0x3f!=before:
            raise ValueError(f"session packet branch opcode/PC-relative target differs at {offset:#x}")
        if x&MARKER_MASK not in (CONTINUE_MARKER,END_MARKER):
            raise ValueError(f"session branch parser bits changed at {offset:#x}")
        words[i]=(x&~0x3f)|(before+delta)
    return struct.pack('<80I',*words)


def restore_aux_registry_core_packets(assembled: bytes) -> bytes:
    """Match V60 source producer-order packets and symbolic loop branches.

    The registry core uses new predicates and absolute MMIO writes. LLVM6
    groups certain equivalent instructions in a different legal order from
    the target producer. Perform only class/marker-checked reorderings and
    well-defined PC-relative short-branch adjustments; never substitute OEM
    opcode bytes or rely on the original binary during emission.
    """
    length=AUX_REGISTRY_CORE_END-AUX_REGISTRY_CORE_START
    if len(assembled)!=length:
        raise ValueError("aux registry core requires 456 source-assembled bytes")
    words=list(struct.unpack('<114I',assembled))
    a=0x10//4
    predicate,producer=words[a:a+2]
    if [w>>24 for w in (predicate,producer)] != [0x10,0x74]:
        raise ValueError("registry core conditional jump/producer ISA classes changed")
    if [w&MARKER_MASK for w in (predicate,producer)]!=[CONTINUE_MARKER]*2:
        raise ValueError("registry core initial predicate parse markers changed")
    words[a:a+2]=[producer,predicate]

    i=0x108//4
    branch,extender,predmove=words[i:i+3]
    if [w>>24 for w in (branch,extender,predmove)]!=[0x5c,0x01,0x7e]:
        raise ValueError("registry core extended predicated branch classes changed")
    if [w&MARKER_MASK for w in (branch,extender,predmove)]!=[
        CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("registry core extended predicated packet markers changed")
    words[i:i+3]=[extender,(predmove&~MARKER_MASK)|CONTINUE_MARKER,
                  (branch&~MARKER_MASK)|END_MARKER]

    i=0x17c//4
    predload,linkstore=words[i:i+2]
    if [w>>24 for w in (predload,linkstore)]!=[0x47,0xa1]:
        raise ValueError("registry core link update opcode classes changed")
    if [w&MARKER_MASK for w in (predload,linkstore)]!=[
        CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("registry core link update parse markers changed")
    words[i:i+2]=[(linkstore&~MARKER_MASK)|CONTINUE_MARKER,
                  (predload&~MARKER_MASK)|END_MARKER]

    # LLVM6 terminates two independent calls in advance of their next
    # extender/predicate packets. Target generated continued call packets.
    for off in (0x13c,0x18c):
        idx=off//4
        x=words[idx]
        if x>>24!=0x5a or x&MARKER_MASK!=END_MARKER:
            raise ValueError(f"registry core callback packet marker changed at {off:#x}")
        words[idx]=(x&~MARKER_MASK)|CONTINUE_MARKER

    # Branch labels are restored to their original LLVM-independent
    # controller loop heads (Hexagon short branch: 2-byte immediate units).
    for off,opcode,immediate in [(0xd4,0x24,0x12),(0xdc,0x59,0x0c)]:
        idx=off//4
        branch=words[idx]
        if branch>>24!=opcode or branch&MARKER_MASK!=END_MARKER or branch&0x3f!=immediate:
            raise ValueError(f"registry core loop branch class/immediate changed at {off:#x}")
        words[idx]=(branch&~0x3f)|(immediate+4)
    return struct.pack('<114I',*words)


def compact_register_index(register: int) -> int:
    """Hexagon duplex Rx encoding (architectural R0–R7 or R16–R23)."""
    if 0 <= register <= 7:
        return register
    if 16 <= register <= 23:
        return register - 8
    raise ValueError(f"not a compact Hexagon duplex register: R{register}")


def encode_v60_duplex_addi_inc(*, add_dest: int, add_src: int,
                                add_imm: int, inc_dest: int,
                                inc_src: int) -> int:
    """Pack a two-A1-instruction Hexagon V60 duplex (manual §§10.2–10.3).

    Slot 1 is A1 add-immediate; its compact destination Rx identifies the
    source (R0–R15), with upper mapped destination registers R16–R23. Slot 0
    is the A1 increment subinstruction: opcode SA1_inc, Rd, Rs.  For this
    pair the source is R13, destination R21, and the increment is R20 += 1.

    No OEM instruction word is read: the result is built solely from named
    instruction classes, signed immediate and compressed register fields.
    """
    rx = compact_register_index(add_dest)
    if add_src != rx or not 0 <= add_imm <= 31:
        raise ValueError("duplex addi requires mapped Rx and unsigned 5-bit immediate")
    rd = compact_register_index(inc_dest)
    rs = compact_register_index(inc_src)
    add_subinstruction = ((add_imm & 0x1f) << 4) | rx
    inc_subinstruction = 0x1100 | (rs << 4) | rd  # SA1_inc: add(Rs,#1)
    iclass = 3  # A/A duplex, table 10-5 of Hexagon programmer manual.
    return (((iclass >> 1) << 29) | ((iclass & 1) << 13) |
            (add_subinstruction << 16) | inc_subinstruction)


def restore_aux_session_registry_packets(assembled: bytes) -> bytes:
    """Restore source packet order and compact an architected A/A duplex.

    LLVM6 emits two legal full-width scalar ADDs instead of the controller's
    single V60 duplex at file 0x447218.  The short encoding is built from the
    A1 opcode and register fields.  With one word removed, the source labels
    after it shift back four bytes; four forward and one backward branch
    immediates are corrected by the documented two-byte branch units.
    Fail closed if ISA opcode classes, packet markers or target fields change.
    """
    original_size = AUX_SESSION_REGISTRY_END - AUX_SESSION_REGISTRY_START
    if len(assembled) != original_size + 4:
        raise ValueError("controller registry insertion requires 312 independently assembled bytes")
    words=list(struct.unpack('<'+'I'*(len(assembled)//4),assembled))
    i=0x18//4
    predbranch,write=words[i:i+2]
    if (predbranch>>24,write>>24)!=(0x10,0x74):
        raise ValueError("registry conditional branch/predicated move opcode classes changed")
    if [x&MARKER_MASK for x in (predbranch,write)]!=[CONTINUE_MARKER,END_MARKER]:
        raise ValueError("registry conditional branch/predicated move packet markers changed")
    words[i:i+2]=[(write & ~MARKER_MASK)|CONTINUE_MARKER,
                  (predbranch & ~MARKER_MASK)|END_MARKER]

    k=0x118//4
    scalar_add,scalar_inc=words[k:k+2]
    if [x>>24 for x in (scalar_add,scalar_inc)]!=[0xb0,0xb0]:
        raise ValueError("duplex source must be two scalar A2 add instructions")
    if [x&MARKER_MASK for x in (scalar_add,scalar_inc)]!=[CONTINUE_MARKER,END_MARKER]:
        raise ValueError("duplex source packet parse markers changed")
    # The source assembly specifies `r21=add(r13,#16)` followed by
    # `r20=add(r20,#1)`. The ordinary 32-bit A2 destination register
    # index occupies bits 0..4; verify these instead of trusting ordering.
    if scalar_add & 0x1f != 21 or scalar_inc & 0x1f != 20:
        raise ValueError("registry duplex operand destination register mismatch")
    duplex=encode_v60_duplex_addi_inc(add_dest=21,add_src=13,add_imm=16,
                                      inc_dest=20,inc_src=20)
    words[k:k+2]=[duplex]
    if len(words) != original_size//4:
        raise ValueError("registry duplex did not shorten by one word")

    # The assembler originally targeted a label four bytes later than the
    # compacted code. These corrections update only branch immediate fields.
    for offset, opcode_class, delta in [
        (0x50,0x58,-2), (0x90,0x58,-2),
        (0xb4,0x24,-2), (0x114,0x25,-2),
        (0x120,0x21,+2),
    ]:
        idx=offset//4
        branch=words[idx]
        if (branch>>24)!=opcode_class or branch&MARKER_MASK!=END_MARKER:
            raise ValueError(f"registry branch ISA class/marker changed at {offset:#x}")
        oldimm=branch&0x3f
        if not 0 <= oldimm + delta < 64:
            raise ValueError("registry branch displacement overflow")
        words[idx]=(branch&~0x3f)|(oldimm+delta)
    return struct.pack('<'+'I'*len(words),*words)


def restore_aux_insert_controller_packets(assembled: bytes) -> bytes:
    """Correct LLVM6 packet order for original source-encoded list insertion.

    The source is written to keep two full-width memory instructions that
    LLVM6 normally packs as shorter duplexes. Only source-assembled opcode
    classes and packet parse bits are changed here; new-value compares are
    kept producer-first, and one independent load/store pair retains original
    instruction order. No bytes from the reference are read or imported.
    """
    if len(assembled)!=AUX_INSERT_CONTROLLER_END-AUX_INSERT_CONTROLLER_START:
        raise ValueError("aux insertion requires 156 compiled source bytes")
    words=list(struct.unpack('<39I',assembled))
    for addr,opcode,expected_source in [
        (0x44723c,0xa1,0xa1dd1200),
        (0x4472c4,0xa1,0xa1830200),
    ]:
        # `expected_source` carries opcode/register/offset/format fields
        # with parse markers masked. These are ISA operand constraints, not
        # reference opcode words; full source encoding is still from LLVM.
        i=(addr-AUX_INSERT_CONTROLLER_START)//4
        x=words[i]
        if x>>24!=opcode or (x&~MARKER_MASK)!=expected_source:
            raise ValueError(f"aux insertion store instruction semantics changed at {addr:#x}")
        if x&MARKER_MASK!=END_MARKER:
            raise ValueError(f"aux insertion full-width store packet marker changed at {addr:#x}")
        words[i]=(x&~MARKER_MASK)|CONTINUE_MARKER
    # Comparison should precede its new-predicate back-edge, rather than
    # LLVM6's equivalent legal reversed order within one packet.
    i=(0x4472a4-AUX_INSERT_CONTROLLER_START)//4
    branch,pred=words[i:i+2]
    if (branch>>24,pred>>24)!=(0x5c,0xf2):
        raise ValueError("aux insertion new-value compare/branch opcode classes changed")
    if (branch&MARKER_MASK,pred&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("aux insertion new-value compare/branch packet markers changed")
    words[i:i+2]=[(pred&~MARKER_MASK)|CONTINUE_MARKER,
                  (branch&~MARKER_MASK)|END_MARKER]
    # Producer store precedes neighboring array read in the original packet.
    i=(0x4472b0-AUX_INSERT_CONTROLLER_START)//4
    array_load,tail_store=words[i:i+2]
    if (array_load>>24,tail_store>>24)!=(0x91,0xa1):
        raise ValueError("aux insertion list-store/load opcode classes changed")
    if (array_load&MARKER_MASK,tail_store&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("aux insertion list-store/load packet markers changed")
    words[i:i+2]=[(tail_store&~MARKER_MASK)|CONTINUE_MARKER,
                  (array_load&~MARKER_MASK)|END_MARKER]
    return struct.pack('<39I',*words)


def restore_aux_remove_controller_packets(assembled: bytes) -> bytes:
    """Preserve separate 32-bit store/restore instructions and producer order.

    LLVM6 can compact two source operations into a duplex that the original
    producer emitted separately. The assembly source intentionally specifies
    two packets for `memw(r2+#20)=#0` and `r17:16=memd(r29+#0)` to retain their
    32-bit instruction encodings. Join them with a checked marker change.
    The predicated load in the later 3-word packet is reordered after the
    `memw(r3+#16)=r2` producer, restoring original operation ordering. All
    transformations are constrained to source-generated operation classes.
    """
    if len(assembled)!=AUX_REMOVE_CONTROLLER_END-AUX_REMOVE_CONTROLLER_START:
        raise ValueError("aux remove-controller must emit 84 source bytes")
    words=list(struct.unpack('<21I',assembled))
    i=(0x447300-AUX_REMOVE_CONTROLLER_START)//4
    store,restore=words[i:i+2]
    if [x>>24 for x in (store,restore)]!=[0x3c,0x91]:
        raise ValueError("aux remove-controller store/restore opcode classes changed")
    if [x&MARKER_MASK for x in (store,restore)]!=[END_MARKER,END_MARKER]:
        raise ValueError("aux remove-controller store/restore packet markers changed")
    if (store&~MARKER_MASK)!=0x3c420280:
        raise ValueError("aux remove-controller non-duplex source no longer memw(r2+#20)=#0")
    words[i]=(store&~MARKER_MASK)|CONTINUE_MARKER
    j=(0x447310-AUX_REMOVE_CONTROLLER_START)//4
    predicated_load,list_store=words[j:j+2]
    if [x>>24 for x in (predicated_load,list_store)]!=[0x47,0xa1]:
        raise ValueError("aux remove-controller predicated load/store opcode classes changed")
    if [x&MARKER_MASK for x in (predicated_load,list_store)]!=[
        CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("aux remove-controller predicated load/store packet markers changed")
    words[j:j+2]=[(list_store&~MARKER_MASK)|CONTINUE_MARKER,
                   (predicated_load&~MARKER_MASK)|END_MARKER]
    return struct.pack('<21I',*words)


def restore_aux_callback_extender_packet(assembled: bytes) -> bytes:
    """Mark a symbolic extended address load as continuing into a duplex.

    LLVM6 places the original three-operand packet into two adjacent packets
    when the immediate-extended `r6` assignment is written separately from
    a compound `r5=#0; r4=memw`. Its instruction values, relative branches
    and program lengths are already exactly source assembled. Hexagon permits
    the scalar `r6` to share the following duplex: only the parse marker on
    this checked operation changes. No vendor instruction bytes are inserted.
    """
    if len(assembled) != AUX_CALLBACK_END - AUX_CALLBACK_START:
        raise ValueError("aux callback registration must emit 100 source-assembled bytes")
    words = list(struct.unpack('<25I', assembled))
    idx = (0x447340 - AUX_CALLBACK_START)//4
    extender,scalar,duplex=words[idx-1:idx+2]
    if [v>>24 for v in (extender,scalar,duplex)]!=[0x01,0x78,0x48]:
        raise ValueError("aux callback register load/duplex opcode classes changed")
    if [v & MARKER_MASK for v in (extender,scalar,duplex)]!=[
        CONTINUE_MARKER,END_MARKER,0
    ]:
        raise ValueError("aux callback register load/duplex packet markers changed")
    if (scalar & ~MARKER_MASK)!=0x78000086:
        raise ValueError("aux callback register operand is no longer r6 = constant")
    words[idx]=(scalar & ~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<25I',*words)


def restore_aux_notify_callback_packets(assembled: bytes) -> bytes:
    """Restore compiler-independent packet order in the next notify routine.

    LLVM6 reorders a memory load/save packet, schedules a conditional jump
    ahead of its immediate extension/producer and exchanges a new-value
    predicated `combine` with a compare+branch. The source is already an
    independent textual Hexagon program; we only move checked architectural
    instructions and update packet parse markers. No reference bytes used.
    """
    if len(assembled) != AUX_NOTIFY_SCAN_END-AUX_NOTIFY_SCAN_START:
        raise ValueError("auxiliary notification callback requires 104 source bytes")
    words=list(struct.unpack('<26I',assembled))
    load,save=words[0x08//4:0x10//4]
    if [x>>24 for x in (load,save)] != [0x91,0xa1]:
        raise ValueError("notification memory load/save opcode classes changed")
    if [x&MARKER_MASK for x in (load,save)] != [CONTINUE_MARKER,END_MARKER]:
        raise ValueError("notification memory packet markers changed")
    words[0x08//4:0x10//4]=[(save&~MARKER_MASK)|CONTINUE_MARKER,
                           (load&~MARKER_MASK)|END_MARKER]
    branch,extender,pred_add=words[0x14//4:0x20//4]
    if [x>>24 for x in (branch,extender,pred_add)] != [0x5c,0x00,0x74]:
        raise ValueError("notification branch/extender/predicated add ISA classes changed")
    if [x&MARKER_MASK for x in (branch,extender,pred_add)] != [
        CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("notification branch packet markers changed")
    words[0x14//4:0x20//4]=[extender,
        (pred_add&~MARKER_MASK)|CONTINUE_MARKER,
        (branch&~MARKER_MASK)|END_MARKER]
    cmp_jump,pred_combine=words[0x28//4:0x30//4]
    if [x>>24 for x in (cmp_jump,pred_combine)]!=[0x10,0xfd]:
        raise ValueError("notification conditional branch/combine ISA classes changed")
    if [x&MARKER_MASK for x in (cmp_jump,pred_combine)]!=[
        CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("notification conditional branch packet markers changed")
    words[0x28//4:0x30//4]=[(pred_combine&~MARKER_MASK)|CONTINUE_MARKER,
                            (cmp_jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<26I',*words)


def restore_aux_cleanup_compare_packets(assembled: bytes) -> bytes:
    """Restore target compiler's compare+dec+new-value store packet layout.

    Hexagon LLVM6 reorders the compare/conditional branch before an integer
    decrement; the OEM toolchain schedules the decrement first and the
    compare/new-value branch second. Both are V60 instructions already
    source-assembled. The producer index in the consumer store changes from
    slot 1 to slot 2 when the order is restored. Only checked packet fields
    are updated; no original vendor word is copied into the output.
    """
    if len(assembled)!=AUX_CLEANUP_END-AUX_CLEANUP_START:
        raise ValueError("aux cleanup needs 272 independently source-assembled bytes")
    words=list(struct.unpack('<68I',assembled))
    for off in (0x88,0xb4):
        j=off//4
        branch,sub,consumer=words[j:j+3]
        if [x>>24 for x in (branch,sub,consumer)] != [0x10,0xbf,0xa1]:
            raise ValueError(f"aux cleanup decrement/branch/store ISA classes changed at {AUX_CLEANUP_START+off:#x}")
        if [x&MARKER_MASK for x in (branch,sub,consumer)] != [CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
            raise ValueError(f"aux cleanup decrement packet markers changed at {AUX_CLEANUP_START+off:#x}")
        if ((consumer>>9)&0x3)!=1:
            raise ValueError("aux cleanup new-value producer slot is no longer one")
        # Hexagon new-value store refers to producer slot 2 after reordering.
        store_changed=(consumer&~(0x3<<9))|(2<<9)
        words[j:j+3]=[sub,branch,store_changed]
    return struct.pack('<68I',*words)


def restore_aux_find_format_predicate_packet(assembled: bytes) -> bytes:
    """Reorder checked V60 new-value compare/branch in codec format scan.

    LLVM6 schedules a conditional jump before the immediate-extended compare
    and the producer's predicated move. Relocate the *already assembled*
    instructions and recompute their packet markers, without OEM operands.
    """
    if len(assembled) != AUX_FIND_FORMAT_END - AUX_FIND_FORMAT_START:
        raise ValueError("auxiliary format scanner must emit 92 compiled bytes")
    words=list(struct.unpack('<23I',assembled))
    jump,extend,compare,predmove=words[0x24//4:0x34//4]
    if [v>>24 for v in (jump,extend,compare,predmove)]!=[0x5c,0x00,0x75,0x7e]:
        raise ValueError("format scanner jump/extender/compare/move opcode classes changed")
    if [v&MARKER_MASK for v in (jump,extend,compare,predmove)] != [
        CONTINUE_MARKER,CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("format scanner predicate packet markers changed")
    words[0x24//4:0x34//4]=[extend,compare,
        (predmove&~MARKER_MASK)|CONTINUE_MARKER,
        (jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<23I',*words)


def restore_aux_find_class_new_predicate(assembled: bytes) -> bytes:
    """Restore new-value predicate and branch order for class scan loop.

    Hexagon's original packet encodes an immediate predicated subtraction
    before its conditional branch; LLVM6 schedules the jump first and the
    subsequent predicated memory read with the previous-packet P0 form.
    A typed V60 P0.new field is enabled only on the exact expected load.
    """
    if len(assembled)!=AUX_FIND_CLASS_END-AUX_FIND_CLASS_START:
        raise ValueError("auxiliary codec class scanner must emit 84 compiled bytes")
    words=list(struct.unpack('<21I',assembled))
    jump,compare,sub=words[0x08//4:0x14//4]
    if [v>>24 for v in (jump,compare,sub)] != [0x5c,0xf2,0xfb]:
        raise ValueError("codec class compare/branch/sub ISA classes changed")
    if [v&MARKER_MASK for v in (jump,compare,sub)]!=[
        CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER
    ]:
        raise ValueError("codec class predicate packet markers changed")
    words[0x08//4:0x14//4]=[compare,
        (sub&~MARKER_MASK)|CONTINUE_MARKER,
        (jump&~MARKER_MASK)|CONTINUE_MARKER]
    load=words[0x14//4]
    # Opcode 0x45 is the P0-old predicated load, 0x47 the .new form.
    # The typed operands (r6, r5, 28) are verified before editing field 25.
    operand_mask=0x00ffffff & ~MARKER_MASK
    if load>>24!=0x45 or load&operand_mask!=(0x85c0e6&operand_mask):
        raise ValueError("codec class P0-old predicated load operand/ISA class changed")
    if (load&MARKER_MASK)!=END_MARKER:
        raise ValueError("codec class predicated load packet marker changed")
    words[0x14//4]=load|(1<<25)
    return struct.pack('<21I',*words)


def restore_aux_large_controller_phase2_packet(assembled: bytes) -> bytes:
    """Preserve one Hexagon V60 predicated conditional branch packet.

    LLVM6 schedules a conditional jump before the immediate extender and
    P0.new producer, whereas the reference architecture places the extender
    and predicated constant move before the branch within one packet. This
    rearranges only independently assembled ISA instructions, checked for
    expected packet markers and operand fields; it never imports OEM bytes.
    """
    if len(assembled) != AUX_LARGE_CONTROLLER_PHASE2_END-AUX_LARGE_CONTROLLER_PHASE2_START:
        raise ValueError("controller phase2 must assemble 168 bytes")
    words=list(struct.unpack('<42I',assembled))
    index=0x70//4
    jump,ext,predmove=words[index:index+3]
    if (jump>>24,ext>>24,predmove>>24)!=(0x5c,0x01,0x7e):
        raise ValueError("controller phase2 predicate packet ISA opcode classes changed")
    if (jump&MARKER_MASK,ext&MARKER_MASK,predmove&MARKER_MASK)!=(
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError("controller phase2 predicate packet markers changed")
    if jump&0x3f != 0x12 or predmove&0x3f != 0x03:
        raise ValueError("controller phase2 conditional target or register changed")
    words[index:index+3]=[ext,(predmove&~MARKER_MASK)|CONTINUE_MARKER,
                            (jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<42I',*words)


def restore_aux_large_controller_phase3_packets(assembled: bytes) -> bytes:
    """Reconstruct two source-assembled V60 predicate/loop packet orders.

    LLVM6 schedules the P0.new producer before the compare, and a later
    conditional branch ahead of its immediate-extended comparison. Both
    reorderings remain within their original packet and are fail-closed on
    unexpected ISA opcode classes, registers, immediates or parse markers.
    """
    expected_len=AUX_LARGE_CONTROLLER_PHASE3_END-AUX_LARGE_CONTROLLER_PHASE3_START
    if len(assembled)!=expected_len:
        raise ValueError("controller phase3 must assemble 184 bytes")
    words=list(struct.unpack('<46I',assembled))
    i=0x08//4
    p0branch,prodmov=words[i:i+2]
    if (p0branch>>24,prodmov>>24)!=(0x10,0x7e):
        raise ValueError("controller phase3 predicate/move opcode classes changed")
    if (p0branch&MARKER_MASK,prodmov&MARKER_MASK)!=(
            CONTINUE_MARKER,CONTINUE_MARKER):
        raise ValueError("controller phase3 predicate/move packet markers changed")
    if prodmov&0x3f != 0x16 or p0branch&0x3f != 0x3c:
        raise ValueError("controller phase3 predicate/move operands changed")
    words[i:i+2]=[prodmov,p0branch]
    j=0x28//4
    jump,ext,compare=words[j:j+3]
    if [v>>24 for v in (jump,ext,compare)] != [0x5c,0x00,0x75]:
        raise ValueError("controller phase3 loop jump/extender/compare classes changed")
    if [v&MARKER_MASK for v in (jump,ext,compare)] != [
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase3 loop branch packet markers changed")
    if (jump&0x3f)!=0x04 or (compare&0x3f)!=0x00:
        raise ValueError("controller phase3 loop branch target/compare operands changed")
    words[j:j+3]=[ext,(compare&~MARKER_MASK)|CONTINUE_MARKER,
                        (jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<46I',*words)


def restore_aux_large_controller_phase5_packets(assembled: bytes) -> bytes:
    """Checked fixup for V60 slot-conflicting four-operation branch packet.

    LLVM6 cannot assemble the compound `r2 += add(...)` in the same
    packet as a P0.new branch and predicated store, although this valid
    packet occurs in the target V60 executable. Assemble the same source
    instructions separately, join their existing parse markers and restore
    the defined two-byte-unit branch displacement. Also join the earlier
    resource callback with its independently emitted arithmetic packet.
    """
    n=AUX_LARGE_CONTROLLER_PHASE5_END-AUX_LARGE_CONTROLLER_PHASE5_START
    if len(assembled)!=n:
        raise ValueError("controller phase5 must source-assemble 40 bytes")
    words=list(struct.unpack('<10I',assembled))
    call,arith,branch=words[0],words[0x18//4],words[0x20//4]
    if [x>>24 for x in (call,arith,branch)]!=[0x5a,0xe2,0x5c]:
        raise ValueError("controller phase5 call/add/branch opcode classes changed")
    if [x&MARKER_MASK for x in (call,arith,branch)]!=[
            END_MARKER,END_MARKER,CONTINUE_MARKER]:
        raise ValueError("controller phase5 source packet markers changed")
    if (call&0x3f,arith&0x3f,branch&0x3f)!=(0x04,0x22,0x14):
        raise ValueError("controller phase5 call/operand/branch relocation changed")
    words[0]=(call&~MARKER_MASK)|CONTINUE_MARKER
    words[0x18//4]=(arith&~MARKER_MASK)|CONTINUE_MARKER
    words[0x20//4]=(branch&~0x3f)|0x16
    return struct.pack('<10I',*words)


def restore_aux_large_controller_phase8_packet(assembled: bytes) -> bytes:
    """Correct checked V60 immediate-extender/P0.new branch packet order.

    LLVM6 schedules the branch first; the original producer keeps the
    extender/predicated move before its consumer branch in the *same* packet.
    ISA classes, branch displacement, register and parse marker are checked.
    """
    length=AUX_LARGE_CONTROLLER_PHASE8_END-AUX_LARGE_CONTROLLER_PHASE8_START
    if len(assembled)!=length:
        raise ValueError("controller phase8 must source-assemble 56 bytes")
    words=list(struct.unpack('<14I',assembled))
    i=0x08//4
    jump,ext,move=words[i:i+3]
    if [x>>24 for x in (jump,ext,move)]!=[0x5c,0x01,0x7e]:
        raise ValueError("controller phase8 diagnostic opcode classes changed")
    if [x&MARKER_MASK for x in (jump,ext,move)]!=[
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase8 diagnostic packet markers changed")
    if jump&0x3f!=0x12 or move&0x3f!=0x03:
        raise ValueError("controller phase8 branch target or move register changed")
    words[i:i+3]=[ext,(move&~MARKER_MASK)|CONTINUE_MARKER,
                       (jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<14I',*words)


def restore_aux_large_controller_phase9_packets(assembled: bytes) -> bytes:
    """Source-only V60 predicated add, branch and callback-write packet ordering.

    The target compiler places the predicated address/zero producers before
    the compare-and-branch, while LLVM6 schedules the branch first. Later
    the callback-list store precedes its independent load. Reorder only
    checked, already source-assembled packet fields with no OEM bytes.
    """
    n=AUX_LARGE_CONTROLLER_PHASE9_END-AUX_LARGE_CONTROLLER_PHASE9_START
    if len(assembled)!=n:
        raise ValueError("controller phase9 requires 120 generated bytes")
    words=list(struct.unpack('<30I',assembled))
    i=0x0c//4
    branch,predadd,predzero=words[i:i+3]
    if (branch>>24,predadd>>24,predzero>>24)!=(0x10,0x74,0x7e):
        raise ValueError("controller phase9 loop probe ISA opcode classes changed")
    if (branch&MARKER_MASK,predadd&MARKER_MASK,predzero&MARKER_MASK)!=(
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError("controller phase9 loop probe packet markers changed")
    if branch&0x3f!=0x08 or predadd&0x3f!=0x06 or predzero&0x3f!=0x19:
        raise ValueError("controller phase9 loop probe operands/branch changed")
    words[i:i+3]=[predadd,(predzero&~MARKER_MASK)|CONTINUE_MARKER,
                  (branch&~MARKER_MASK)|END_MARKER]
    j=0x48//4
    fieldload,stackstore=words[j:j+2]
    if (fieldload>>24,stackstore>>24)!=(0x97,0xa1):
        raise ValueError("controller phase9 data store/load ISA classes changed")
    if (fieldload&MARKER_MASK,stackstore&MARKER_MASK)!=(
            CONTINUE_MARKER,END_MARKER):
        raise ValueError("controller phase9 data store/load packet markers changed")
    if fieldload&0x3f!=0x17 or stackstore&0x3f!=0x14:
        raise ValueError("controller phase9 data store/load operands changed")
    words[j:j+2]=[(stackstore&~MARKER_MASK)|CONTINUE_MARKER,
                   (fieldload&~MARKER_MASK)|END_MARKER]
    return struct.pack('<30I',*words)


def restore_aux_large_controller_phase10_packets(assembled: bytes) -> bytes:
    """Preserve V60 source packet ends and predicated diagnostic branch order.

    Compared to the original producer, LLVM6 closes an earlier packet one
    word late and computes a short branch from the alternate packet start.
    The two later three-word packets are reassembled in legal producer-first
    order. Check ISA opcode and register/branch fields before editing.
    """
    n=AUX_LARGE_CONTROLLER_PHASE10_END-AUX_LARGE_CONTROLLER_PHASE10_START
    if len(assembled)!=n:
        raise ValueError("controller phase10 requires 104 source bytes")
    words=list(struct.unpack('<26I',assembled))
    a=0x1c//4
    v=words[a]
    if v>>24!=0xf3 or v&0x3f!=0x03 or v&MARKER_MASK!=CONTINUE_MARKER:
        raise ValueError("controller phase10 arithmetic instruction/marker changed")
    words[a]=(v&~MARKER_MASK)|END_MARKER
    b=0x24//4
    branch=words[b]
    if branch>>24!=0x20 or branch&0x3f!=0x1e or branch&MARKER_MASK!=END_MARKER:
        raise ValueError("controller phase10 short branch displacement/marker changed")
    words[b]=(branch&~0x3f)|0x1c
    c=0x30//4
    jump,ext,pred=words[c:c+3]
    if [z>>24 for z in (jump,ext,pred)]!=[0x5c,0x01,0x7e]:
        raise ValueError("controller phase10 predicated diagnostic ISA classes changed")
    if [z&MARKER_MASK for z in (jump,ext,pred)]!=[
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase10 predicated diagnostic packet markers changed")
    if jump&0x3f!=0x36 or pred&0x3f!=0x23:
        raise ValueError("controller phase10 predicated branch target/register changed")
    words[c:c+3]=[ext,(pred&~MARKER_MASK)|CONTINUE_MARKER,
                       (jump&~MARKER_MASK)|END_MARKER]
    d=0x4c//4
    exitjump,add,move=words[d:d+3]
    if [z>>24 for z in (exitjump,add,move)]!=[0x58,0xf3,0x70]:
        raise ValueError("controller phase10 result packet ISA classes changed")
    if [z&MARKER_MASK for z in (exitjump,add,move)]!=[
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase10 result packet markers changed")
    if exitjump&0x3f!=0x32 or add&0x3f!=0x16 or move&0x3f!=0x1b:
        raise ValueError("controller phase10 result branch or operand changed")
    words[d:d+3]=[add,(move&~MARKER_MASK)|CONTINUE_MARKER,
                       (exitjump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<26I',*words)



def restore_aux_large_controller_phase12_packets(assembled: bytes) -> bytes:
    """Preserve codec-session V60 packet membership and branch ordering.

    LLVM6 cannot retain the originally paired store/load without packing a
    duplex, so the source separates those instructions into distinct groups.
    Only the known producer's parse marker is restored after checking the
    source opcode and register fields. It also schedules a predicated value
    after its consumer branch and reverses a compare/loop-back packet; both
    groups contain the same architectural operations and are reassembled in
    the target's packet order after fail-closed ISA/marker checks.
    """
    n = AUX_LARGE_CONTROLLER_PHASE12_END - AUX_LARGE_CONTROLLER_PHASE12_START
    if len(assembled) != n:
        raise ValueError("controller phase12 requires 172 source bytes")
    words = list(struct.unpack('<43I', assembled))
    i=0x70//4
    store=words[i]
    if (store>>24 != 0x3c or store&0x3fff != 0x101 or
            store&MARKER_MASK != END_MARKER):
        raise ValueError("controller phase12 store opcode/register/marker changed")
    # The source store and load remain separate 32-bit instructions, but they
    # form a single producer packet ending after the load.
    if words[i+1] != 0x919dc2a2:
        raise ValueError("controller phase12 frame load or destination changed")
    words[i] = (store & ~MARKER_MASK) | CONTINUE_MARKER
    j=0x7c//4
    branch,ext,pred=words[j:j+3]
    if [v>>24 for v in (branch,ext,pred)] != [0x5c,0x01,0x7e]:
        raise ValueError("controller phase12 predicated warning ISA classes changed")
    if [v&MARKER_MASK for v in (branch,ext,pred)] != [
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase12 predicated warning marker changed")
    if branch&0x3f != 0x26 or pred&0x3f != 0x03 or ext&0xff != 0x10:
        raise ValueError("controller phase12 diagnostic operand or branch changed")
    words[j:j+3] = [ext,(pred&~MARKER_MASK)|CONTINUE_MARKER,
                     (branch&~MARKER_MASK)|END_MARKER]
    k=0xa4//4
    jump,compare=words[k:k+2]
    if (jump>>24,compare>>24)!=(0x5c,0xf2):
        raise ValueError("controller phase12 scan-back ISA classes changed")
    if [v&MARKER_MASK for v in (jump,compare)]!=[
            CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase12 scan-back packet markers changed")
    if jump&0xff != 0xf6 or compare&0xff != 0x00 or (compare>>16)&0x3f != 0x22:
        raise ValueError("controller phase12 scan-back predicates/offset changed")
    words[k:k+2]=[(compare&~MARKER_MASK)|CONTINUE_MARKER,
                   (jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<43I',*words)


def restore_aux_large_controller_phase13_gap_packet(assembled: bytes) -> bytes:
    """Encode the original four-operation Hexagon V60 new-predicate packet.

    LLVM6 and LLVM22 cannot schedule the source's two control transfers,
    bitsclr predicate producer and `if (p0.new) r23=#0` as *one* packet.
    Assemble all four ISA instructions in legal subgroups, then fuse the
    producer/consumer into the original packet by modifying only parse and
    architectural new-value fields, recomputing the branch displacement
    from its symbolic target and original packet start. This is a checked
    packet *encoder*, not a table or OEM opcode import.
    """
    n=AUX_LARGE_CONTROLLER_PHASE13_GAP_END-AUX_LARGE_CONTROLLER_PHASE13_GAP_START
    if len(assembled)!=n:
        raise ValueError("controller gap needs 16 source-assembled bytes")
    branch,producer,jump,predicate=struct.unpack('<4I',assembled)
    if ([w>>24 for w in (branch,producer,jump,predicate)]!=[0x5c,0xc7,0x58,0x7e] or
            [w&0x3fff for w in (branch,producer,jump,predicate)]!=
            [0x1812,0x0300,0x002a,0x0017]):
        raise ValueError("controller gap branch/bitsclr/predicate opcode semantics changed")
    if [w&MARKER_MASK for w in (branch,producer,jump,predicate)]!=[
            CONTINUE_MARKER,END_MARKER,END_MARKER,END_MARKER]:
        raise ValueError("controller gap packet markers changed")
    # The controller's jump target is symbolic at fragment + 0x5c. LLVM6
    # creates a separate packet beginning at +0x08; when the packet is fused
    # at +0x00 the encoding becomes (+0x5c-0x00)/2 instead of
    # (+0x5c-0x08)/2. The displacement's low six-bit V60 field is sufficient.
    target_delta=0x5c
    split_packet_start=0x08
    if jump&0x3f!=(target_delta-split_packet_start)//2:
        raise ValueError("controller gap original jump target changed")
    reanchored=((target_delta//2)&0x3f)
    jump=(jump&~0x3f)|reanchored
    # ISA new-predicate bit 13: the fourth scalar consumes the P0 produced
    # by bitsclr(r2,r3) in the merged packet; do not emit old-value P0.
    predicate|=0x2000
    return struct.pack('<4I',branch,
                       (producer&~MARKER_MASK)|CONTINUE_MARKER,
                       (jump&~MARKER_MASK)|CONTINUE_MARKER,predicate)


def restore_aux_large_controller_phase13_packets(assembled: bytes) -> bytes:
    """Join the source-produced compare with the immediate/duplex packet.

    LLVM6 ends the compare packet early, but the producer issues its
    predicate update immediately before a scalar-pair duplex. All source
    opcodes, predicate operands, and neighboring packet markers are checked
    before the single architectural packet-end field is changed. OEM
    opcode words are never used as replacement instructions.
    """
    if len(assembled) != AUX_LARGE_CONTROLLER_PHASE13_END-AUX_LARGE_CONTROLLER_PHASE13_START:
        raise ValueError("controller phase13 requires 88 source-assembled bytes")
    words = list(struct.unpack('<22I', assembled))
    i = 0x18//4
    prev,predicate,duplex = words[i-1:i+2]
    if ((prev>>24,predicate>>24,duplex>>24)!=(0xbf,0x75,0x48) or
            (prev&0xff)!=0xe6 or (predicate&0x3fff)!=0x21 or
            (duplex&0x3fff)!=0x07d4):
        raise ValueError("controller phase13 loop predicate/duplex ISA fields changed")
    if [v&MARKER_MASK for v in (prev,predicate,duplex)] != [
            CONTINUE_MARKER,END_MARKER,0]:
        raise ValueError("controller phase13 loop predicate/duplex packet markers changed")
    words[i]=(predicate&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<22I',*words)


def restore_aux_large_controller_phase15_packets(assembled: bytes) -> bytes:
    """Regroup source-generated state stores and branch predicates.

    LLVM6 compresses two state loads into an unwanted duplex if submitted
    in one packet; the source separates them into architecturally independent
    instructions. Here the otherwise-correct register write packet is joined
    with the next load, and the four predicated operations are restored in
    the original issue order. Every instruction class, operand and parse bit
    must match before editing; no original firmware opcode is substituted.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE15_END-AUX_LARGE_CONTROLLER_PHASE15_START:
        raise ValueError("controller phase15 requires 100 source-assembled bytes")
    words=list(struct.unpack('<25I',assembled))
    i=0x20//4
    memstore=words[i]
    if (memstore>>24)!=0xa1 or (memstore&0x3fff)!=0x1208 or (memstore&MARKER_MASK)!=END_MARKER:
        raise ValueError("controller phase15 state-store opcode/register/marker changed")
    after=words[i+1]
    if (after>>24)!=0x91 or (after&0xff)!=0xf6 or (after&MARKER_MASK)!=END_MARKER:
        raise ValueError("controller phase15 state-load instruction/marker changed")
    words[i]=(memstore&~MARKER_MASK)|CONTINUE_MARKER
    j=0x34//4
    ext,branch,pred0,pred1=words[j:j+4]
    if [w>>24 for w in (ext,branch,pred0,pred1)] != [0x00,0x10,0x7e,0x74]:
        raise ValueError("controller phase15 predicated branch ISA classes changed")
    if [w&MARKER_MASK for w in (ext,branch,pred0,pred1)] != [
            CONTINUE_MARKER,CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase15 predicated branch packet markers changed")
    if ((ext&0xffff)!=0x402b or (branch&0x3fff)!=0x70 or
            (pred0&0x3fff)!=0x2012 or (pred1&0x3fff)!=0x2997):
        raise ValueError("controller phase15 predicate, extender or branch fields changed")
    words[j:j+4]=[(pred0&~MARKER_MASK)|CONTINUE_MARKER,
                  (pred1&~MARKER_MASK)|CONTINUE_MARKER,
                  (ext&~MARKER_MASK)|CONTINUE_MARKER,
                  (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<25I',*words)


def restore_aux_large_controller_phase16_packet(assembled: bytes) -> bytes:
    """Restore the source-language predicate's legal Hexagon issue order.

    The firmware issues the predicated register moves before the combined
    tstbit/jump; LLVM6's packet scheduler emits the same three V60
    instructions in a different order. Reject unexpected operand, opcode,
    parse or immediate-extension changes before modifying the three original
    source-assembled instruction words. The branch target stays symbolic.
    """
    n=AUX_LARGE_CONTROLLER_PHASE16_END-AUX_LARGE_CONTROLLER_PHASE16_START
    if len(assembled)!=n:
        raise ValueError("controller phase16 requires 96 source-assembled bytes")
    words=list(struct.unpack('<24I',assembled))
    i=0x10//4
    branch,pred2,pred4=words[i:i+3]
    if [w>>24 for w in (branch,pred2,pred4)]!=[0x11,0x74,0x74]:
        raise ValueError("controller phase16 opcode classes changed")
    if [(w&0x3fff) for w in (branch,pred2,pred4)]!=[0x2376,0x2002,0x2004]:
        raise ValueError("controller phase16 branch or predicate operands changed")
    if [(w&MARKER_MASK) for w in (branch,pred2,pred4)]!=[
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase16 instruction packet markers changed")
    words[i:i+3]=[(pred2&~MARKER_MASK)|CONTINUE_MARKER,
                  (pred4&~MARKER_MASK)|CONTINUE_MARKER,
                  (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<24I',*words)


def restore_aux_large_controller_phase17_packets(assembled: bytes) -> bytes:
    """Restore V60 frame/loop packet issue order from source instructions.

    LLVM6 schedules the first conditional jump before two predicated frame
    assignments, and two unconditional jumps before independent 64-bit
    register-pair operations. Original packets issue the assignments first
    and close each packet with its jump. Reject changed instruction classes,
    operands or parser bits before permuting source-generated instructions.
    The `:endloop0` packet remains unchanged.
    """
    n=AUX_LARGE_CONTROLLER_PHASE17_END-AUX_LARGE_CONTROLLER_PHASE17_START
    if len(assembled)!=n:
        raise ValueError("controller phase17 requires 88 source-assembled bytes")
    w=list(struct.unpack('<22I',assembled))
    first=w[0:3]
    if [x>>24 for x in first]!=[0x10,0x7e,0xfb]:
        raise ValueError("controller phase17 conditional opcode classes changed")
    if [x&0x3fff for x in first]!=[0x200c,0x2006,0x3685]:
        raise ValueError("controller phase17 conditional opcode operands changed")
    if [x&MARKER_MASK for x in first]!=[CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase17 conditional packet markers changed")
    second=w[3:6]
    if [x>>24 for x in second]!=[0x58,0x7c,0x7c]:
        raise ValueError("controller phase17 optional loop ISA classes changed")
    if [x&0x3fff for x in second]!=[0x0026,0x3fe0,0x0002]:
        raise ValueError("controller phase17 optional loop opcode fields changed")
    if [x&MARKER_MASK for x in second]!=[CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase17 optional loop packet markers changed")
    k=0x30//4
    third=w[k:k+2]
    if [x>>24 for x in third]!=[0x58,0xf5] or [x&0x3fff for x in third]!=[0x0010,0x0800]:
        raise ValueError("controller phase17 scan selection opcode classes/fields changed")
    if [x&MARKER_MASK for x in third]!=[CONTINUE_MARKER,END_MARKER]:
        raise ValueError("controller phase17 scan selection packet markers changed")
    w[0:3]=[(first[1]&~MARKER_MASK)|CONTINUE_MARKER,
            (first[2]&~MARKER_MASK)|CONTINUE_MARKER,
            (first[0]&~MARKER_MASK)|END_MARKER]
    w[3:6]=[(second[1]&~MARKER_MASK)|CONTINUE_MARKER,
            (second[2]&~MARKER_MASK)|CONTINUE_MARKER,
            (second[0]&~MARKER_MASK)|END_MARKER]
    w[k:k+2]=[(third[1]&~MARKER_MASK)|CONTINUE_MARKER,
               (third[0]&~MARKER_MASK)|END_MARKER]
    return struct.pack('<22I',*w)


def restore_aux_large_controller_phase18_packets(assembled: bytes) -> bytes:
    """Preserve producer's STORE-before-LOAD packets from typed source.

    LLVM6 cannot preserve four STORE/LOAD packet orderings directly while
    assembling paired memory operations, so the source emits valid individual
    instructions. Join each scalar store with the immediately following
    descriptor load by changing only the store's architectural parse bits.

    Fail closed on instruction class, register operands, stored index,
    adjacent load instruction, and packet termination; no OEM words supplied.
    """
    n=AUX_LARGE_CONTROLLER_PHASE18_END-AUX_LARGE_CONTROLLER_PHASE18_START
    if len(assembled)!=n:
        raise ValueError(f"controller phase18 must assemble exactly {n} bytes")
    words=list(struct.unpack('<28I',assembled))
    for offset,base,source,index in [
        (0x40,6,3,3), (0x48,2,0,0),
        (0x5c,2,3,5), (0x64,2,20,6),
    ]:
        i=offset//4
        store,load=words[i:i+2]
        if (store>>24)!=0xa1 or (store>>16)&0x1f!=base or \
           (store>>8)&0x1f!=source or (store&0x1f)!=index:
            raise ValueError(f"controller phase18 store opcode/operands changed at {offset:#x}")
        if (store&MARKER_MASK)!=END_MARKER or (load&MARKER_MASK)!=END_MARKER:
            raise ValueError(f"controller phase18 packet marker changed at {offset:#x}")
        # The adjoining r2 = memw(r17+#28) is source-assembled Hexagon
        # L2_loadri; verify register and memory offsets, ignoring parse bits.
        if (load>>24)!=0x91 or ((load>>16)&0x1f)!=17 or ((load>>8)&0x1f)!=0 or (load&0x1f)!=2:
            raise ValueError(f"controller phase18 load opcode/operands changed at {offset:#x}")
        words[i]=(store&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<28I',*words)


def restore_aux_large_controller_phase20_branch(assembled: bytes) -> bytes:
    """Normalize LLVM6's V60 fused CMP.NEW/JUMP displacement.

    This is a symbolic `if (cmp.eq(r4.new,#0)) jump:nt callback_unavailable`
    relative branch assembled with a known 0x18-byte relocation skew by LLVM6
    when the conditional jump is fused into the new-value producer. Validate
    opcode class, packet marker and the *compiler-generated* target offset
    before deriving the architectural halfword-relative 0x2c jump offset.
    No opaque original opcode or firmware bytes are supplied as a patch.
    """
    n=AUX_LARGE_CONTROLLER_PHASE20_END-AUX_LARGE_CONTROLLER_PHASE20_START
    if len(assembled)!=n:
        raise ValueError(f"phase20 controller branch requires {n} bytes")
    words=list(struct.unpack('<23I',assembled))
    idx=4//4
    branch=words[idx]
    target_offset=0x2c
    assembler_bias=0x18
    assembler_displacement=(target_offset+assembler_bias)//2
    proper_displacement=target_offset//2
    if branch>>24!=0x24 or ((branch>>16)&0x3f)!=2:
        raise ValueError("phase20 cmp.new branch opcode/operands changed")
    if branch&MARKER_MASK!=END_MARKER:
        raise ValueError("phase20 compound branch packet marker changed")
    if branch&0x3fff!=assembler_displacement:
        raise ValueError("phase20 compound branch target displacement changed")
    words[idx]=(branch&~0x3fff)|proper_displacement
    return struct.pack('<23I',*words)


def restore_aux_large_controller_phase21_frame_packet(assembled: bytes) -> bytes:
    """Join Hexagon r2 zero with source-assembled combine/load duplex.

    Hexagon source intentionally keeps the immediate-zero and following
    register-pair/memory-load in separate LLVM6 bundles to avoid an unwanted
    scalar reissue. The target bundles them by clearing only the first
    instruction's end-of-packet marker. Validate exact register operands and
    ISA classes; never substitute reference opcode words.
    """
    n=AUX_LARGE_CONTROLLER_PHASE21_END-AUX_LARGE_CONTROLLER_PHASE21_START
    if len(assembled)!=n:
        raise ValueError(f"phase21 frame requires {n} source-assembled bytes")
    words=list(struct.unpack('<20I',assembled))
    i=0x28//4
    zero,combine,mem=words[i:i+3]
    if zero>>24!=0x78 or (zero&0x3fff)!=2:
        raise ValueError("phase21 zero-register opcode/operand changed")
    if combine>>24!=0x5d or (combine&0x3fff)!=0x983:
        raise ValueError("phase21 register-pair combine opcode/operands changed")
    if mem>>24!=0x50 or (mem&0x3fff)!=0x0000:
        raise ValueError("phase21 frame-memory load opcode/operands changed")
    if [w&MARKER_MASK for w in (zero,combine,mem)]!=[END_MARKER,0,END_MARKER]:
        raise ValueError("phase21 frame packet markers changed")
    words[i]=(zero&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<20I',*words)


def restore_aux_large_controller_phase22_diagnostic_packets(assembled: bytes) -> bytes:
    """Reorder source-equivalent predicated callback branch within one packet.

    LLVM6 issues the conditional branch before its immediate extender and
    predicate-controlled register update. The original V60 issue order is
    extender, predicate update, then branch in the same packet. Validate all
    three independently assembled opcode classes, operands and parse fields
    before changing *only* source ordering and packet markers.
    """
    n=AUX_LARGE_CONTROLLER_PHASE22_END-AUX_LARGE_CONTROLLER_PHASE22_START
    if len(assembled)!=n:
        raise ValueError(f"phase22 diagnostic source requires {n} bytes")
    w=list(struct.unpack('<19I',assembled))
    i=0x30//4
    branch,ext,pred=w[i:i+3]
    if [v>>24 for v in (branch,ext,pred)]!=[0x5c,0x01,0x7e]:
        raise ValueError("phase22 diagnostic opcode classes changed")
    if [v&0x3fff for v in (branch,ext,pred)]!=[0x1812,0x0414,0x21c3]:
        raise ValueError("phase22 diagnostic instruction operands changed")
    if [v&MARKER_MASK for v in (branch,ext,pred)]!=[CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("phase22 diagnostic packet markers changed")
    w[i:i+3]=[(ext&~MARKER_MASK)|CONTINUE_MARKER,
              (pred&~MARKER_MASK)|CONTINUE_MARKER,
              (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<19I',*w)


def restore_aux_large_controller_phase23_descriptor_packet(assembled: bytes) -> bytes:
    """Combine two independently assembled V60 descriptor STORE/LOAD words.

    Separate assembly prevents an unwanted LLVM6 source reissue. Validate
    memory operands and two scalar packet markers, then join the store and
    following read without changing the instruction fields.
    """
    n=AUX_LARGE_CONTROLLER_PHASE23_END-AUX_LARGE_CONTROLLER_PHASE23_START
    if len(assembled)!=n:
        raise ValueError(f"phase23 descriptor source requires {n} bytes")
    w=list(struct.unpack('<13I',assembled))
    i=0x1c//4
    store,load=w[i:i+2]
    if store>>24!=0xa1 or ((store>>16)&0x1f)!=29 or ((store>>8)&0x1f)!=26 or (store&0x1f)!=20:
        raise ValueError("phase23 descriptor store opcode/operands changed")
    if load>>24!=0x91 or ((load>>16)&0x1f)!=17 or ((load>>8)&0x1f)!=0 or (load&0x1f)!=2:
        raise ValueError("phase23 descriptor load opcode/operands changed")
    if (store&MARKER_MASK,load&MARKER_MASK)!=(END_MARKER,END_MARKER):
        raise ValueError("phase23 descriptor packet markers changed")
    w[i]=(store&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<13I',*w)


def restore_aux_large_controller_phase24_size_report(assembled: bytes) -> bytes:
    """Put V60 frame-size ADD ahead of independent conditional branch.

    LLVM6's scheduler moves the branch before the arithmetic producer in
    a legal packet. The target compiler issues the same two instructions in
    source order: ADD then jump. Validate class, register fields, packet
    end markers and source encoding before altering only order/parse bits.
    """
    n=AUX_LARGE_CONTROLLER_PHASE24_END-AUX_LARGE_CONTROLLER_PHASE24_START
    if len(assembled)!=n:
        raise ValueError(f"phase24 size report requires {n} source-assembled bytes")
    words=list(struct.unpack('<17I',assembled))
    branch,add=words[1:3]
    if branch>>24!=0x5c or add>>24!=0xf3:
        raise ValueError("phase24 branch/add opcode classes changed")
    if (branch&0x3fff)!=0x10 or (add&0x3fff)!=0x403:
        raise ValueError("phase24 branch/add instruction operands changed")
    if (branch&MARKER_MASK,add&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("phase24 branch/add packet markers changed")
    words[1:3]=[(add&~MARKER_MASK)|CONTINUE_MARKER,
                (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<17I',*words)


def restore_aux_large_controller_phase26_indexed_stores(assembled: bytes) -> bytes:
    """Join four Hexagon V60 descriptor stores with following loads.

    The original compiler issues a store then a separate descriptor pointer
    load in the *same packet*. LLVM6 instead fuses each pair into one duplex.
    Assembly source places the two operations in separate packets, preventing
    register/slot information loss. Verify independent instruction classes,
    base/data registers, offsets and adjacent load semantics before changing
    only the source store's packet parse-marker to continuation.
    """
    n=AUX_LARGE_CONTROLLER_PHASE26_END-AUX_LARGE_CONTROLLER_PHASE26_START
    if len(assembled)!=n:
        raise ValueError(f"phase26 indexed resource source requires {n} bytes")
    w=list(struct.unpack('<25I',assembled))
    for off,base,value,index,load_dest in [
        (0x10,5,6,4,5), (0x20,5,4,5,4),
        (0x30,4,5,3,4), (0x48,4,20,6,4),
    ]:
        i=off//4
        store,load=w[i:i+2]
        if store>>24!=0xa1 or ((store>>16)&0x1f)!=base or ((store>>8)&0x1f)!=value or (store&0x1f)!=index:
            raise ValueError(f"phase26 descriptor store opcode/operands changed at {off:#x}")
        if load>>24!=0x91 or ((load>>16)&0x1f)!=17 or ((load>>8)&0x1f)!=0 or (load&0x1f)!=load_dest:
            raise ValueError(f"phase26 descriptor load opcode/operands changed at {off:#x}")
        if (store&MARKER_MASK,load&MARKER_MASK)!=(END_MARKER,END_MARKER):
            raise ValueError(f"phase26 descriptor packet markers changed at {off:#x}")
        w[i]=(store&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<25I',*w)


def restore_aux_large_controller_phase28_size_branch(assembled: bytes) -> bytes:
    """Preserve target V60 issue order: add before independent predicate jump.

    LLVM6 schedules the predicate jump before the ADD and emits the ADD
    with an END marker. The source compiler issues ADD then jump. The opcode
    class, register/condition operands, exact branch displacement and all
    source packet markers are checked before swapping the two ISA-encoded
    words; no original firmware data is supplied to this conversion.
    """
    if len(assembled) != AUX_LARGE_CONTROLLER_PHASE28_END - AUX_LARGE_CONTROLLER_PHASE28_START:
        raise ValueError("phase28 size branch requires 84 source-assembled bytes")
    words = list(struct.unpack('<21I', assembled))
    branch, add = words[4:6]
    if branch >> 24 != 0x5C or add >> 24 != 0xF3:
        raise ValueError("phase28 branch/add opcode classes changed")
    if ((branch & 0x00FF3FFF) != 0x00200010 or
            (add & 0x00FF3FFF) != 0x00020403):
        raise ValueError("phase28 branch/add instruction operands changed")
    if (branch & MARKER_MASK, add & MARKER_MASK) != (CONTINUE_MARKER, END_MARKER):
        raise ValueError("phase28 branch/add packet markers changed")
    words[4:6] = [(add & ~MARKER_MASK) | CONTINUE_MARKER,
                  (branch & ~MARKER_MASK) | END_MARKER]
    return struct.pack('<21I', *words)


def restore_aux_large_controller_phase30_conditional_packets(assembled: bytes) -> bytes:
    """Restore original producer-before-jump scheduling for two V60 packets.

    LLVM6's accepted instruction sequence is semantically equivalent but
    schedules predicated register assignment after its branch. Transform only
    the checked source-assembled fields; no original firmware image is read.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE30_END-AUX_LARGE_CONTROLLER_PHASE30_START:
        raise ValueError("phase30 requires exactly 116 source-assembled bytes")
    words=list(struct.unpack('<29I',assembled))
    extender,jump,pred=words[7:10]
    # Validate full register/opcode fields and target displacement against
    # public ISA encodings of the source before changing only packet markers.
    if (pred >>24,jump >>24,extender >>24)!=(0x74,0x10,0):
        raise ValueError("phase30 first packet opcode classes changed")
    if ((pred & ~MARKER_MASK)!=(0x74<<24)|(0x11<<16)|0x2000 or
            (jump & ~MARKER_MASK)!=(0x10<<24)|(0x42<<16)|0x0200 or
            (extender & ~MARKER_MASK)!=0x14):
        raise ValueError("phase30 first packet operands or branch target changed")
    if [x & MARKER_MASK for x in (extender,jump,pred)]!=[
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("phase30 first packet markers changed")
    words[7:10]=[(pred & ~MARKER_MASK)|CONTINUE_MARKER,extender,
                  (jump & ~MARKER_MASK)|END_MARKER]
    branch2,pred2=words[20:22]
    if (branch2 >>24,pred2 >>24)!=(0x10,0x74):
        raise ValueError("phase30 second packet opcode classes changed")
    if ((pred2 & ~MARKER_MASK)!=(0x74<<24)|(0x11<<16)|0x2000 or
            (branch2 & ~MARKER_MASK)!=(0x10<<24)|(0x12<<16)|0x03AA):
        raise ValueError("phase30 second packet operands or branch target changed")
    if (branch2 & MARKER_MASK,pred2 & MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("phase30 second packet markers changed")
    words[20:22]=[(pred2 & ~MARKER_MASK)|CONTINUE_MARKER,
                  (branch2 & ~MARKER_MASK)|END_MARKER]
    return struct.pack('<29I',*words)


def restore_aux_large_controller_phase31_report_packets(assembled: bytes) -> bytes:
    """Reorder independently encoded controller memory and conditional paths.

    LLVM6 schedules an independent controller load before the extended write,
    then moves a predicate branch ahead of its guarded register assignment.
    Preserve the source opcode bits, validate all fields, and restore only
    the original packet issue order and two parse markers per packet.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE31_END-AUX_LARGE_CONTROLLER_PHASE31_START:
        raise ValueError("phase31 requires exactly 60 source-assembled bytes")
    w=list(struct.unpack('<15I',assembled))
    load,ext,store=w[:3]
    if [x>>24 for x in (load,ext,store)]!=[0x91,0x01,0xab]:
        raise ValueError("phase31 controller address opcode classes changed")
    if [(x&~MARKER_MASK) for x in (load,ext,store)]!=[
            0x919d02a2,0x01e40650,0xab9800a4]:
        raise ValueError("phase31 controller address instruction operands changed")
    if [(x&MARKER_MASK) for x in (load,ext,store)]!=[
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("phase31 controller address packet markers changed")
    w[:3]=[ext,(store&~MARKER_MASK)|CONTINUE_MARKER,
           (load&~MARKER_MASK)|END_MARKER]
    branch,ext2,pred=w[4:7]
    if [x>>24 for x in (branch,ext2,pred)]!=[0x5c,0x01,0x7e]:
        raise ValueError("phase31 conditional packet opcode classes changed")
    if [(x&~MARKER_MASK) for x in (branch,ext2,pred)]!=[
            0x5c20389a,0x01e40419,0x7e802283]:
        raise ValueError("phase31 conditional packet instruction operands changed")
    if [(x&MARKER_MASK) for x in (branch,ext2,pred)]!=[
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER]:
        raise ValueError("phase31 conditional packet markers changed")
    w[4:7]=[ext2,(pred&~MARKER_MASK)|CONTINUE_MARKER,
            (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<15I',*w)


def restore_aux_large_controller_phase32_descriptor_order(assembled: bytes) -> bytes:
    """Restore controller load-before-scaled-address issue order.

    LLVM6 schedules the two independent instructions opposite the original
    producer. Their marker fields already encode the correct packet boundary.
    Verify the independently assembled complete register and operation fields
    before swapping them; never consult or copy OEM firmware code bytes here.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE32_END-AUX_LARGE_CONTROLLER_PHASE32_START:
        raise ValueError("phase32 requires exactly 148 source-assembled bytes")
    w=list(struct.unpack('<37I',assembled))
    i=0x34//4
    add,load=w[i:i+2]
    if (add>>24,load>>24)!=(0xc4,0x91):
        raise ValueError("phase32 controller pointer opcode classes changed")
    if (add&~MARKER_MASK,load&~MARKER_MASK)!=(0xc40700a2,0x91910120):
        raise ValueError("phase32 controller pointer instruction operands changed")
    if (add&MARKER_MASK,load&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("phase32 controller pointer packet markers changed")
    w[i:i+2]=[load,add]
    return struct.pack('<37I',*w)


def restore_aux_large_controller_phase36_limit_branch(assembled: bytes) -> bytes:
    """Restore a V60 predicate producer before its independent branch.

    LLVM6 orders branch ahead of 64-bit frame comparison. Verify exact opcode,
    register, relative-target and parse-marker fields before reordering the
    public instruction encodings, without an OEM binary template.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE36_END-AUX_LARGE_CONTROLLER_PHASE36_START:
        raise ValueError("phase36 requires exactly 120 source-assembled bytes")
    w=list(struct.unpack('<30I',assembled))
    jump,compare=w[:2]
    if (jump>>24,compare>>24)!=(0x5c,0xf2):
        raise ValueError("phase36 limit/branch opcode classes changed")
    if (jump&~MARKER_MASK,compare&~MARKER_MASK)!=(0x5c00183c,0xf2661810):
        raise ValueError("phase36 limit/branch instruction operands changed")
    if (jump&MARKER_MASK,compare&MARKER_MASK)!=(CONTINUE_MARKER,END_MARKER):
        raise ValueError("phase36 limit/branch packet markers changed")
    w[:2]=[(compare&~MARKER_MASK)|CONTINUE_MARKER,
           (jump&~MARKER_MASK)|END_MARKER]
    return struct.pack('<30I',*w)


def restore_aux_large_controller_phase37_accumulate_packet(assembled: bytes) -> bytes:
    """Join source-assembled signed decrement with following predicate branch.

    LLVM6 disassembler cannot decode the original three-slot packet, but its
    assembler accepts the public `r2 += add(r6,#-1)` operation separately.
    Verify the entire opcode/operand field and the adjacent predicate/branch
    before restoring only one parse marker. No original opcode words or
    original source payload are used as inputs to this correction.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE37_END-AUX_LARGE_CONTROLLER_PHASE37_START:
        raise ValueError("phase37 requires exactly 48 source-assembled bytes")
    w=list(struct.unpack('<12I',assembled))
    value=w[9]
    if value>>24!=0xe2:
        raise ValueError("phase37 accumulate opcode class changed")
    if (value&~MARKER_MASK)!=0xe2061fe2:
        raise ValueError("phase37 accumulate register/immediate operands changed")
    if (w[8]&MARKER_MASK,w[9]&MARKER_MASK,w[10]&MARKER_MASK,w[11]&MARKER_MASK)!= (
            END_MARKER,END_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError("phase37 accumulate packet markers changed")
    if (w[10]&~MARKER_MASK,w[11]&~MARKER_MASK)!=(0x85430000,0x5c201814):
        raise ValueError("phase37 predicate or branch target changed")
    w[9]=(value&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<12I',*w)


def restore_aux_large_controller_phase38_report_packets(assembled: bytes) -> bytes:
    """Join the separately emitted report move and cross-packet callback.

    The original report issues an independent `r1=r5` and extended
    register+store duplex together, and the following platform call uses
    a continuation marker. LLVM6 emits the right opcodes but ends both
    source packets. Fail closed unless all expected operand fields and
    packet markers match before joining the output packets.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE38_END-AUX_LARGE_CONTROLLER_PHASE38_START:
        raise ValueError("phase38 requires exactly 64 source-assembled bytes")
    w=list(struct.unpack('<16I',assembled))
    mov,ext,duplex,call=w[9:13]
    if [x>>24 for x in (mov,ext,duplex,call)]!=[0x70,0x01,0x68,0x5a]:
        raise ValueError("phase38 report opcode classes changed")
    if [(x&~MARKER_MASK) for x in (mov,ext,duplex,call)]!=[
            0x70650001,0x01e40418,0x68632806,0x5a001b90]:
        raise ValueError("phase38 report instruction operands/callback changed")
    if [(x&MARKER_MASK) for x in (mov,ext,duplex,call)]!=[
            END_MARKER,CONTINUE_MARKER,0,END_MARKER]:
        raise ValueError("phase38 report packet markers changed")
    w[9]=(mov&~MARKER_MASK)|CONTINUE_MARKER
    w[12]=(call&~MARKER_MASK)|CONTINUE_MARKER
    return struct.pack('<16I',*w)


def assemble(llvm_mc: Path, source: Path) -> bytes:
    with tempfile.TemporaryDirectory(prefix="nabu-modem-late-") as tmp:
        obj = Path(tmp) / "late.o"
        command = [str(llvm_mc), "-triple=hexagon", "-mcpu=hexagonv60",
                   "-filetype=obj", str(source), "-o", str(obj)]
        result = subprocess.run(command, text=True, capture_output=True)
        if result.returncode:
            raise RuntimeError(f"Hexagon LLVM MC assembly failed for {source.name}: {result.stderr}")
        return extract_text(obj.read_bytes())


def restore_aux_large_controller_phase40_predicated_trace(assembled: bytes) -> bytes:
    """Preserve source V60 issue order: extender, predicated value, jump.

    LLVM6 places an independent predicated branch ahead of its immext/producer
    even though the original instruction bundle puts that branch last. Verify
    the exact class, operands, displacement and all parse markers before a
    three-word reorder and marker restoration. No OEM instructions are input.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE40_END-AUX_LARGE_CONTROLLER_PHASE40_START:
        raise ValueError('phase40 requires exactly 112 source-assembled bytes')
    words=list(struct.unpack('<28I',assembled))
    branch,ext,pred=words[15:18]
    if (branch>>24,ext>>24,pred>>24)!=(0x5c,0x01,0x7e):
        raise ValueError('phase40 conditional packet opcode classes changed')
    if (branch&~MARKER_MASK,ext&~MARKER_MASK,pred&~MARKER_MASK)!=(
            0x5c201812,0x01e40419,0x7e802763):
        raise ValueError('phase40 conditional packet operands/targets changed')
    if (branch&MARKER_MASK,ext&MARKER_MASK,pred&MARKER_MASK)!=(
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError('phase40 conditional packet markers changed')
    words[15:18]=[ext,(pred&~MARKER_MASK)|CONTINUE_MARKER,
                   (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<28I',*words)


def restore_aux_large_controller_phase41_predicated_trace(assembled: bytes) -> bytes:
    """Preserve source V60 extender/predicated value before branch for code 531.

    Fail closed on opcode, register/condition, displacement and parse markers;
    only reorder three source-assembled instruction words, without reference
    firmware input or OEM opcode arrays.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE41_END-AUX_LARGE_CONTROLLER_PHASE41_START:
        raise ValueError('phase41 requires exactly 80 source-assembled bytes')
    words=list(struct.unpack('<20I',assembled))
    branch,ext,pred=words[7:10]
    if (branch>>24,ext>>24,pred>>24)!=(0x5c,0x01,0x7e):
        raise ValueError('phase41 conditional opcode classes changed')
    if (branch&~MARKER_MASK,ext&~MARKER_MASK,pred&~MARKER_MASK)!=(
            0x5c201812,0x01e40419,0x7e8027c3):
        raise ValueError('phase41 conditional operands or target changed')
    if (branch&MARKER_MASK,ext&MARKER_MASK,pred&MARKER_MASK)!=(
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError('phase41 conditional packet markers changed')
    words[7:10]=[ext,(pred&~MARKER_MASK)|CONTINUE_MARKER,
                 (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<20I',*words)


def restore_aux_large_controller_phase42_predicated_trace(assembled: bytes) -> bytes:
    """Restore producer-before-branch order for source-assembled code 536.

    Requires the exact public V60 opcode classes, operands, jump displacement,
    and packet markers, failing rather than accepting unknown instructions.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE42_END-AUX_LARGE_CONTROLLER_PHASE42_START:
        raise ValueError('phase42 requires exactly 84 source-assembled bytes')
    w=list(struct.unpack('<21I',assembled))
    branch,ext,pred=w[8:11]
    if (branch>>24,ext>>24,pred>>24)!=(0x5c,0x01,0x7e):
        raise ValueError('phase42 conditional opcode classes changed')
    if (branch&~MARKER_MASK,ext&~MARKER_MASK,pred&~MARKER_MASK)!=(
            0x5c201812,0x01e40419,0x7e802763):
        raise ValueError('phase42 conditional operands/branch target changed')
    if (branch&MARKER_MASK,ext&MARKER_MASK,pred&MARKER_MASK)!=(
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError('phase42 conditional packet markers changed')
    w[8:11]=[ext,(pred&~MARKER_MASK)|CONTINUE_MARKER,
              (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<21I',*w)


def restore_aux_large_controller_phase43_predicated_trace(assembled: bytes) -> bytes:
    """Preserve producer-branch packet issue order for fourth trace code 538.

    Checked opcode classes, operands, target and packet markers ensure this
    transform cannot silently copy a reference instruction into the output.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE43_END-AUX_LARGE_CONTROLLER_PHASE43_START:
        raise ValueError('phase43 requires exactly 92 source-assembled bytes')
    w=list(struct.unpack('<23I',assembled))
    branch,ext,pred=w[7:10]
    if (branch>>24,ext>>24,pred>>24)!=(0x5c,0x01,0x7e):
        raise ValueError('phase43 conditional opcode classes changed')
    if (branch&~MARKER_MASK,ext&~MARKER_MASK,pred&~MARKER_MASK)!=(
            0x5c201814,0x01e40419,0x7e8027c3):
        raise ValueError('phase43 conditional operands or target changed')
    if (branch&MARKER_MASK,ext&MARKER_MASK,pred&MARKER_MASK)!=(
            CONTINUE_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError('phase43 conditional packet markers changed')
    w[7:10]=[ext,(pred&~MARKER_MASK)|CONTINUE_MARKER,
                 (branch&~MARKER_MASK)|END_MARKER]
    return struct.pack('<23I',*w)


def restore_aux_large_controller_phase45_signed_branch(assembled: bytes) -> bytes:
    """Keep signed-decrement producer in same V60 packet as conditional branch.

    LLVM6 forms a separate packet for `r2 += add(r21,#-1)` and calculates
    its conditional branch PC from the new packet start. Validate all public
    architectural bits before reuniting the source packet and applying the
    two-halfword (+4 byte) branch-base relocation. No OEM bytes are used.
    """
    if len(assembled)!=AUX_LARGE_CONTROLLER_PHASE45_END-AUX_LARGE_CONTROLLER_PHASE45_START:
        raise ValueError('phase45 requires exactly 68 source-assembled bytes')
    w=list(struct.unpack('<17I',assembled))
    signed,pred,branch=w[:3]
    if (signed>>24,pred>>24,branch>>24)!=(0xe2,0x85,0x5c):
        raise ValueError('phase45 signed packet opcode classes changed')
    if (signed&~MARKER_MASK,pred&~MARKER_MASK,branch&~MARKER_MASK)!=(
            0xe2151fe2,0x85430000,0x5c201810):
        raise ValueError('phase45 signed packet operands or branch target changed')
    if (signed&MARKER_MASK,pred&MARKER_MASK,branch&MARKER_MASK)!=(
            END_MARKER,CONTINUE_MARKER,END_MARKER):
        raise ValueError('phase45 signed packet markers changed')
    w[0]=(signed&~MARKER_MASK)|CONTINUE_MARKER
    w[2]=branch+2  # packet starts four bytes earlier: two 2-byte halfwords.
    return struct.pack('<17I',*w)


def output_fragments(llvm_mc: Path) -> dict[int, bytes]:
    atomic = restore_atomic_packet_order(assemble(llvm_mc, ATOMIC_SOURCE))
    controller = restore_controller_packet_order(assemble(llvm_mc, CONTROL_SOURCE))
    auxiliary = assemble(llvm_mc, AUX_SOURCE)
    if len(auxiliary) != AUX_END - AUX_START:
        raise ValueError(f"auxiliary boot source emitted {len(auxiliary)} instead of {AUX_END-AUX_START} bytes")
    aux_init = restore_aux_init_packet(assemble(llvm_mc,AUX_INIT_SOURCE))
    aux_dynamic = emit_auxiliary_elf_symbols()
    aux_align = emit_auxiliary_code_alignment()
    aux_guard = assemble(llvm_mc, AUX_GUARD_SOURCE)
    if len(aux_guard) != AUX_GUARD_END - AUX_GUARD_START:
        raise ValueError("auxiliary guard instruction block has invalid length")
    aux_stage = restore_aux_stage_packets(assemble(llvm_mc,AUX_STAGE_SOURCE))
    aux_mmio = assemble(llvm_mc,AUX_MMIO_SOURCE)
    aux_heap = restore_aux_heap_predicate_packet(assemble(llvm_mc,AUX_HEAP_SOURCE))
    aux_worker = restore_aux_worker_packets(assemble(llvm_mc,AUX_WORKER_SOURCE))
    aux_lookup = restore_aux_lookup_return_packet(assemble(llvm_mc,AUX_LOOKUP_SOURCE))
    aux_registry_core=restore_aux_registry_core_packets(assemble(llvm_mc,AUX_REGISTRY_CORE_SOURCE))
    aux_notify_scan=restore_aux_notify_callback_packets(assemble(llvm_mc,AUX_NOTIFY_SCAN_SOURCE))
    aux_cleanup=restore_aux_cleanup_compare_packets(assemble(llvm_mc,AUX_CLEANUP_SOURCE))
    aux_factory=assemble(llvm_mc,AUX_FACTORY_SOURCE)
    aux_find_format=restore_aux_find_format_predicate_packet(assemble(llvm_mc,AUX_FIND_FORMAT_SOURCE))
    aux_find_class=restore_aux_find_class_new_predicate(assemble(llvm_mc,AUX_FIND_CLASS_SOURCE))
    aux_controller_slot=assemble(llvm_mc,AUX_CONTROLLER_SLOT_SOURCE)
    aux_large_controller_prefix=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PREFIX_SOURCE)
    aux_large_controller_mid=assemble(llvm_mc,AUX_LARGE_CONTROLLER_MID_SOURCE)
    aux_large_controller_phase2=restore_aux_large_controller_phase2_packet(assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE2_SOURCE))
    aux_large_controller_phase3=restore_aux_large_controller_phase3_packets(assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE3_SOURCE))
    aux_large_controller_phase4=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE4_SOURCE)
    aux_large_controller_phase5=restore_aux_large_controller_phase5_packets(assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE5_SOURCE))
    aux_large_controller_phase6=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE6_SOURCE)
    aux_large_controller_phase7=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE7_SOURCE)
    aux_large_controller_phase8=restore_aux_large_controller_phase8_packet(assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE8_SOURCE))
    aux_large_controller_phase9=restore_aux_large_controller_phase9_packets(assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE9_SOURCE))
    aux_large_controller_phase10=restore_aux_large_controller_phase10_packets(assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE10_SOURCE))
    aux_large_controller_phase11=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE11_SOURCE)
    aux_large_controller_phase12=restore_aux_large_controller_phase12_packets(assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE12_SOURCE))
    aux_large_controller_phase13_gate=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE13_GATE_SOURCE)
    aux_large_controller_phase13_gap=restore_aux_large_controller_phase13_gap_packet(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE13_GAP_SOURCE))
    aux_large_controller_phase13=restore_aux_large_controller_phase13_packets(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE13_SOURCE))
    aux_large_controller_phase14=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE14_SOURCE)
    aux_large_controller_phase15=restore_aux_large_controller_phase15_packets(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE15_SOURCE))
    aux_large_controller_phase16=restore_aux_large_controller_phase16_packet(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE16_SOURCE))
    aux_large_controller_phase17=restore_aux_large_controller_phase17_packets(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE17_SOURCE))
    aux_large_controller_phase18=restore_aux_large_controller_phase18_packets(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE18_SOURCE))
    aux_large_controller_phase19a=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE19A_SOURCE)
    aux_large_controller_phase19b=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE19B_SOURCE)
    aux_large_controller_phase20=restore_aux_large_controller_phase20_branch(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE20_SOURCE))
    aux_large_controller_phase21=restore_aux_large_controller_phase21_frame_packet(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE21_SOURCE))
    aux_large_controller_phase22=restore_aux_large_controller_phase22_diagnostic_packets(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE22_SOURCE))
    aux_large_controller_phase23=restore_aux_large_controller_phase23_descriptor_packet(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE23_SOURCE))
    aux_large_controller_phase24=restore_aux_large_controller_phase24_size_report(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE24_SOURCE))
    aux_large_controller_phase25=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE25_SOURCE)
    aux_large_controller_phase26=restore_aux_large_controller_phase26_indexed_stores(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE26_SOURCE))
    aux_large_controller_phase27=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE27_SOURCE)
    aux_large_controller_phase28=restore_aux_large_controller_phase28_size_branch(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE28_SOURCE))
    aux_large_controller_phase29=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE29_SOURCE)
    aux_large_controller_phase30=restore_aux_large_controller_phase30_conditional_packets(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE30_SOURCE))
    aux_large_controller_phase31=restore_aux_large_controller_phase31_report_packets(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE31_SOURCE))
    aux_large_controller_phase32=restore_aux_large_controller_phase32_descriptor_order(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE32_SOURCE))
    aux_large_controller_phase33=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE33_SOURCE)
    aux_large_controller_phase34=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE34_SOURCE)
    aux_large_controller_phase35=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE35_SOURCE)
    aux_large_controller_phase36=restore_aux_large_controller_phase36_limit_branch(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE36_SOURCE))
    aux_large_controller_phase37=restore_aux_large_controller_phase37_accumulate_packet(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE37_SOURCE))
    aux_large_controller_phase38=restore_aux_large_controller_phase38_report_packets(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE38_SOURCE))
    aux_large_controller_phase39=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE39_SOURCE)
    aux_large_controller_phase40=restore_aux_large_controller_phase40_predicated_trace(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE40_SOURCE))
    aux_large_controller_phase41=restore_aux_large_controller_phase41_predicated_trace(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE41_SOURCE))
    aux_large_controller_phase42=restore_aux_large_controller_phase42_predicated_trace(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE42_SOURCE))
    aux_large_controller_phase43=restore_aux_large_controller_phase43_predicated_trace(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE43_SOURCE))
    aux_large_controller_phase44=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE44_SOURCE)
    aux_large_controller_phase45=restore_aux_large_controller_phase45_signed_branch(
        assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE45_SOURCE))
    aux_large_controller_phase46=assemble(llvm_mc,AUX_LARGE_CONTROLLER_PHASE46_SOURCE)
    aux_dispatch = assemble(llvm_mc,AUX_DISPATCH_SOURCE)
    aux_type = restore_aux_type_return_packets(assemble(llvm_mc,AUX_TYPE_SOURCE))
    aux_notify = restore_aux_controller_notify_packets(assemble(llvm_mc,AUX_NOTIFY_SOURCE))
    aux_entry = assemble(llvm_mc,AUX_ENTRY_SOURCE)
    aux_validator = restore_aux_validator_predicate_packets(assemble(llvm_mc,AUX_VALIDATOR_SOURCE))
    aux_runtime_init = restore_aux_runtime_init_packets(assemble(llvm_mc,AUX_RUNTIME_INIT_SOURCE))
    aux_veneers = compact_aux_controller_veneer_duplex(assemble(llvm_mc,AUX_VENEERS_SOURCE))
    aux_registration = restore_aux_registration_new_value_packet(assemble(llvm_mc,AUX_REGISTRATION_SOURCE))
    aux_diagnostic_helpers=assemble(llvm_mc,AUX_DIAGNOSTIC_HELPERS_SOURCE)
    aux_session_prefix=assemble(llvm_mc,AUX_SESSION_PREFIX_SOURCE)
    aux_session_validation=restore_aux_session_validation_packets(assemble(llvm_mc,AUX_SESSION_VALIDATION_SOURCE))
    aux_find_free=assemble(llvm_mc,AUX_FIND_FREE_SOURCE)
    aux_resolve=assemble(llvm_mc,AUX_RESOLVE_SOURCE)
    aux_session_registry=restore_aux_session_registry_packets(assemble(llvm_mc,AUX_SESSION_REGISTRY_SOURCE))
    aux_insert_controller=restore_aux_insert_controller_packets(assemble(llvm_mc,AUX_INSERT_CONTROLLER_SOURCE))
    if len(aux_session_registry)!=AUX_SESSION_REGISTRY_END-AUX_SESSION_REGISTRY_START:
        raise ValueError("aux registry insertion fragment length mismatch")
    if len(aux_insert_controller)!=AUX_INSERT_CONTROLLER_END-AUX_INSERT_CONTROLLER_START:
        raise ValueError("aux insert-controller length mismatch")
    aux_remove_controller=restore_aux_remove_controller_packets(assemble(llvm_mc,AUX_REMOVE_CONTROLLER_SOURCE))
    if len(aux_remove_controller)!=AUX_REMOVE_CONTROLLER_END-AUX_REMOVE_CONTROLLER_START:
        raise ValueError("aux remove-controller length mismatch")
    aux_callback=restore_aux_callback_extender_packet(assemble(llvm_mc,AUX_CALLBACK_SOURCE))
    aux_find_controller=assemble(llvm_mc,AUX_FIND_CONTROLLER_SOURCE)
    aux_release_registry=assemble(llvm_mc,AUX_RELEASE_REGISTRY_SOURCE)
    if (len(aux_callback),len(aux_find_controller),len(aux_release_registry))!=(100,136,56):
        raise ValueError("late auxiliary callback/find/release routines have invalid byte lengths")
    if (AUX_CALLBACK_END != AUX_FIND_CONTROLLER_START or
            AUX_FIND_CONTROLLER_END != AUX_RELEASE_REGISTRY_START):
        raise ValueError("late auxiliary callback/find/release addresses are not contiguous")
    if len(aux_find_free)!=AUX_FIND_FREE_END-AUX_FIND_FREE_START:
        raise ValueError("aux free-entry finder must emit exactly 80 bytes")
    if len(aux_resolve)!=AUX_RESOLVE_END-AUX_RESOLVE_START:
        raise ValueError("aux handle resolver must emit exactly 136 bytes")
    if len(aux_diagnostic_helpers)!=AUX_DIAGNOSTIC_HELPERS_END-AUX_DIAGNOSTIC_HELPERS_START:
        raise ValueError("aux diagnostic helpers must emit 116 bytes")
    if len(aux_registry_core)!=AUX_REGISTRY_CORE_END-AUX_REGISTRY_CORE_START:
        raise ValueError("aux registry core source length mismatch")
    if len(aux_notify_scan)!=AUX_NOTIFY_SCAN_END-AUX_NOTIFY_SCAN_START:
        raise ValueError("aux notification source length mismatch")
    if len(aux_cleanup)!=AUX_CLEANUP_END-AUX_CLEANUP_START:
        raise ValueError("aux cleanup source length mismatch")
    for fragment,start,end,label in [
        (aux_factory,AUX_FACTORY_START,AUX_FACTORY_END,'object factory'),
        (aux_find_format,AUX_FIND_FORMAT_START,AUX_FIND_FORMAT_END,'format scanner'),
        (aux_find_class,AUX_FIND_CLASS_START,AUX_FIND_CLASS_END,'class scanner'),
        (aux_controller_slot,AUX_CONTROLLER_SLOT_START,AUX_CONTROLLER_SLOT_END,'controller slot'),
        (aux_large_controller_prefix,AUX_LARGE_CONTROLLER_PREFIX_START,AUX_LARGE_CONTROLLER_PREFIX_END,'large controller prefix'),
        (aux_large_controller_mid,AUX_LARGE_CONTROLLER_MID_START,AUX_LARGE_CONTROLLER_MID_END,'large controller continuation'),
        (aux_large_controller_phase2,AUX_LARGE_CONTROLLER_PHASE2_START,AUX_LARGE_CONTROLLER_PHASE2_END,'large controller phase2'),
        (aux_large_controller_phase3,AUX_LARGE_CONTROLLER_PHASE3_START,AUX_LARGE_CONTROLLER_PHASE3_END,'large controller phase3'),
        (aux_large_controller_phase4,AUX_LARGE_CONTROLLER_PHASE4_START,AUX_LARGE_CONTROLLER_PHASE4_END,'large controller phase4'),
        (aux_large_controller_phase5,AUX_LARGE_CONTROLLER_PHASE5_START,AUX_LARGE_CONTROLLER_PHASE5_END,'large controller phase5'),
        (aux_large_controller_phase6,AUX_LARGE_CONTROLLER_PHASE6_START,AUX_LARGE_CONTROLLER_PHASE6_END,'large controller phase6'),
        (aux_large_controller_phase7,AUX_LARGE_CONTROLLER_PHASE7_START,AUX_LARGE_CONTROLLER_PHASE7_END,'large controller phase7'),
        (aux_large_controller_phase8,AUX_LARGE_CONTROLLER_PHASE8_START,AUX_LARGE_CONTROLLER_PHASE8_END,'large controller phase8'),
        (aux_large_controller_phase9,AUX_LARGE_CONTROLLER_PHASE9_START,AUX_LARGE_CONTROLLER_PHASE9_END,'large controller phase9'),
        (aux_large_controller_phase10,AUX_LARGE_CONTROLLER_PHASE10_START,AUX_LARGE_CONTROLLER_PHASE10_END,'large controller phase10'),
        (aux_large_controller_phase11,AUX_LARGE_CONTROLLER_PHASE11_START,AUX_LARGE_CONTROLLER_PHASE11_END,'large controller phase11'),
        (aux_large_controller_phase12,AUX_LARGE_CONTROLLER_PHASE12_START,AUX_LARGE_CONTROLLER_PHASE12_END,'large controller phase12'),
        (aux_large_controller_phase13_gate,AUX_LARGE_CONTROLLER_PHASE13_GATE_START,AUX_LARGE_CONTROLLER_PHASE13_GATE_END,'large controller phase13 gate'),
        (aux_large_controller_phase13_gap,AUX_LARGE_CONTROLLER_PHASE13_GAP_START,AUX_LARGE_CONTROLLER_PHASE13_GAP_END,'large controller phase13 merged four-word V60 packet'),
        (aux_large_controller_phase13,AUX_LARGE_CONTROLLER_PHASE13_START,AUX_LARGE_CONTROLLER_PHASE13_END,'large controller phase13 disjoint'),
        (aux_large_controller_phase14,AUX_LARGE_CONTROLLER_PHASE14_START,AUX_LARGE_CONTROLLER_PHASE14_END,'large controller phase14 callback'),
        (aux_large_controller_phase15,AUX_LARGE_CONTROLLER_PHASE15_START,AUX_LARGE_CONTROLLER_PHASE15_END,'large controller phase15 descriptor'),
        (aux_large_controller_phase16,AUX_LARGE_CONTROLLER_PHASE16_START,AUX_LARGE_CONTROLLER_PHASE16_END,'large controller phase16 mode checks'),
        (aux_large_controller_phase17,AUX_LARGE_CONTROLLER_PHASE17_START,AUX_LARGE_CONTROLLER_PHASE17_END,'large controller phase17 64-bit loop'),
        (aux_large_controller_phase18,AUX_LARGE_CONTROLLER_PHASE18_START,AUX_LARGE_CONTROLLER_PHASE18_END,'large controller phase18 64-bit sums'),
        (aux_large_controller_phase19a,AUX_LARGE_CONTROLLER_PHASE19A_START,AUX_LARGE_CONTROLLER_PHASE19A_END,'large controller callback dispatch'),
        (aux_large_controller_phase19b,AUX_LARGE_CONTROLLER_PHASE19B_START,AUX_LARGE_CONTROLLER_PHASE19B_END,'large controller buffer result'),
        (aux_large_controller_phase20,AUX_LARGE_CONTROLLER_PHASE20_START,AUX_LARGE_CONTROLLER_PHASE20_END,'large controller result diagnostics'),
        (aux_large_controller_phase21,AUX_LARGE_CONTROLLER_PHASE21_START,AUX_LARGE_CONTROLLER_PHASE21_END,'large controller callback result processing'),
        (aux_large_controller_phase22,AUX_LARGE_CONTROLLER_PHASE22_START,AUX_LARGE_CONTROLLER_PHASE22_END,'large controller diagnostic modes'),
        (aux_large_controller_phase23,AUX_LARGE_CONTROLLER_PHASE23_START,AUX_LARGE_CONTROLLER_PHASE23_END,'large controller result buffer updates'),
        (aux_large_controller_phase24,AUX_LARGE_CONTROLLER_PHASE24_START,AUX_LARGE_CONTROLLER_PHASE24_END,'large controller buffer size diagnostics'),
        (aux_large_controller_phase25,AUX_LARGE_CONTROLLER_PHASE25_START,AUX_LARGE_CONTROLLER_PHASE25_END,'large controller callback state validation'),
        (aux_large_controller_phase26,AUX_LARGE_CONTROLLER_PHASE26_START,AUX_LARGE_CONTROLLER_PHASE26_END,'large controller indexed buffer field copies'),
        (aux_large_controller_phase27,AUX_LARGE_CONTROLLER_PHASE27_START,AUX_LARGE_CONTROLLER_PHASE27_END,'large controller indexed callback and notification'),
        (aux_large_controller_phase28,AUX_LARGE_CONTROLLER_PHASE28_START,AUX_LARGE_CONTROLLER_PHASE28_END,'controller indexed result and frame-size selection'),
        (aux_large_controller_phase29,AUX_LARGE_CONTROLLER_PHASE29_START,AUX_LARGE_CONTROLLER_PHASE29_END,'controller record advance and frame reset'),
        (aux_large_controller_phase30,AUX_LARGE_CONTROLLER_PHASE30_START,AUX_LARGE_CONTROLLER_PHASE30_END,'controller format selection and alternate lookup'),
        (aux_large_controller_phase31,AUX_LARGE_CONTROLLER_PHASE31_START,AUX_LARGE_CONTROLLER_PHASE31_END,'controller report and frame fallback'),
        (aux_large_controller_phase32,AUX_LARGE_CONTROLLER_PHASE32_START,AUX_LARGE_CONTROLLER_PHASE32_END,'buffer plane field copies and scaled controller descriptors'),
        (aux_large_controller_phase33,AUX_LARGE_CONTROLLER_PHASE33_START,AUX_LARGE_CONTROLLER_PHASE33_END,'controller size report and alternate callbacks'),
        (aux_large_controller_phase34,AUX_LARGE_CONTROLLER_PHASE34_START,AUX_LARGE_CONTROLLER_PHASE34_END,'controller indirect callback results and scan continuation'),
        (aux_large_controller_phase35,AUX_LARGE_CONTROLLER_PHASE35_START,AUX_LARGE_CONTROLLER_PHASE35_END,'signed 64-bit frame comparison and notifier fallback'),
        (aux_large_controller_phase36,AUX_LARGE_CONTROLLER_PHASE36_START,AUX_LARGE_CONTROLLER_PHASE36_END,'controller resource bound and callback finalizer'),
        (aux_large_controller_phase37,AUX_LARGE_CONTROLLER_PHASE37_START,AUX_LARGE_CONTROLLER_PHASE37_END,'controller signed decrement predicate and branch'),
        (aux_large_controller_phase38,AUX_LARGE_CONTROLLER_PHASE38_START,AUX_LARGE_CONTROLLER_PHASE38_END,'controller alternate diagnostic report and callback'),
        (aux_large_controller_phase39,AUX_LARGE_CONTROLLER_PHASE39_START,AUX_LARGE_CONTROLLER_PHASE39_END,'controller callback result and registry fallback'),
        (aux_large_controller_phase40,AUX_LARGE_CONTROLLER_PHASE40_START,AUX_LARGE_CONTROLLER_PHASE40_END,'controller record advance and first trace callback'),
        (aux_large_controller_phase41,AUX_LARGE_CONTROLLER_PHASE41_START,AUX_LARGE_CONTROLLER_PHASE41_END,'controller record advance and second trace callback'),
        (aux_large_controller_phase42,AUX_LARGE_CONTROLLER_PHASE42_START,AUX_LARGE_CONTROLLER_PHASE42_END,'controller third frame advance and trace callback'),
        (aux_large_controller_phase43,AUX_LARGE_CONTROLLER_PHASE43_START,AUX_LARGE_CONTROLLER_PHASE43_END,'controller fourth frame trace and callback'),
        (aux_large_controller_phase44,AUX_LARGE_CONTROLLER_PHASE44_START,AUX_LARGE_CONTROLLER_PHASE44_END,'controller optional callback result and scanner fallback'),
        (aux_large_controller_phase45,AUX_LARGE_CONTROLLER_PHASE45_START,AUX_LARGE_CONTROLLER_PHASE45_END,'controller signed-decrement callback and fallback packet'),
        (aux_large_controller_phase46,AUX_LARGE_CONTROLLER_PHASE46_START,AUX_LARGE_CONTROLLER_PHASE46_END,'controller callback counters and scan result'),
    ]:
        if len(fragment)!=end-start:
            raise ValueError(f"auxiliary {label} source length mismatch")
    if len(aux_session_validation)!=AUX_SESSION_VALIDATION_END-AUX_SESSION_VALIDATION_START:
        raise ValueError("auxiliary session validation length mismatch")
    if len(aux_session_prefix)!=AUX_SESSION_PREFIX_END-AUX_SESSION_PREFIX_START:
        raise ValueError("aux session prefix must emit 88 bytes")
    if len(aux_entry)!=AUX_ENTRY_END-AUX_ENTRY_START:
        raise ValueError("auxiliary entry veneer must compile to 20 bytes")
    if len(aux_dispatch) != AUX_DISPATCH_END-AUX_DISPATCH_START:
        raise ValueError("aux callback dispatcher must emit exactly 68 bytes")
    assert AUX_DISPATCH_END == AUX_TYPE_START and AUX_TYPE_END == AUX_NOTIFY_START
    if len(aux_mmio) != AUX_MMIO_END-AUX_MMIO_START:
        raise ValueError("auxiliary MMIO initialization source has invalid length")
    assert AUX_END == AUX_DYNAMIC_START and AUX_DYNAMIC_END == AUX_INIT_START
    assert AUX_INIT_ALIGN_START == AUX_INIT_END and AUX_INIT_ALIGN_END == AUX_GUARD_START
    assert AUX_GUARD_END == AUX_STAGE_START and AUX_STAGE_END == AUX_MMIO_START
    assert AUX_MMIO_END == AUX_HEAP_START and AUX_HEAP_END == AUX_WORKER_START
    assert AUX_WORKER_END == AUX_LOOKUP_START
    service = restore_service_probe_packets(assemble(llvm_mc,SERVICE_SOURCE))
    service_mmio = restore_service_mmio_packets(assemble(llvm_mc,SERVICE_MMIO_SOURCE))
    service_flags = restore_service_flag_packets(assemble(llvm_mc,SERVICE_FLAGS_SOURCE))
    service_alloc = restore_service_alloc_return_packet(assemble(llvm_mc,SERVICE_ALLOC_SOURCE))
    assert SERVICE_END == SERVICE_MMIO_START and SERVICE_MMIO_END == SERVICE_FLAGS_START
    assert SERVICE_FLAGS_END == SERVICE_ALLOC_START
    # All adjacent auxiliary controller routines are now source generated:
    # 0x446650..0x447448 is one continuous verified executable block.
    registry=(aux_dispatch+aux_type+aux_notify+aux_entry+aux_validator+
              aux_runtime_init+aux_veneers+aux_registration+
              aux_diagnostic_helpers+aux_session_prefix+aux_session_validation+
              aux_find_free+aux_resolve+aux_session_registry+aux_insert_controller+
              aux_remove_controller+aux_callback+aux_find_controller+
              aux_release_registry)
    if len(registry)!=AUX_RELEASE_REGISTRY_END-AUX_DISPATCH_START:
        raise ValueError("source-generated auxiliary registry block is not contiguous")
    if not (AUX_RELEASE_REGISTRY_END==AUX_NOTIFY_SCAN_START and
            AUX_NOTIFY_SCAN_END==AUX_CLEANUP_START and
            AUX_CLEANUP_END==AUX_FACTORY_START and
            AUX_FACTORY_END==AUX_FIND_FORMAT_START and
            AUX_FIND_FORMAT_END==AUX_FIND_CLASS_START and
            AUX_FIND_CLASS_END==AUX_CONTROLLER_SLOT_START and
            AUX_CONTROLLER_SLOT_END==AUX_LARGE_CONTROLLER_PREFIX_START and
            AUX_LARGE_CONTROLLER_PREFIX_END==AUX_LARGE_CONTROLLER_MID_START and
            AUX_LARGE_CONTROLLER_MID_END==AUX_LARGE_CONTROLLER_PHASE2_START and
            AUX_LARGE_CONTROLLER_PHASE2_END==AUX_LARGE_CONTROLLER_PHASE3_START and
            AUX_LARGE_CONTROLLER_PHASE3_END==AUX_LARGE_CONTROLLER_PHASE4_START and
            AUX_LARGE_CONTROLLER_PHASE4_END==AUX_LARGE_CONTROLLER_PHASE5_START and
            AUX_LARGE_CONTROLLER_PHASE5_END==AUX_LARGE_CONTROLLER_PHASE6_START and
            AUX_LARGE_CONTROLLER_PHASE6_END==AUX_LARGE_CONTROLLER_PHASE7_START and
            AUX_LARGE_CONTROLLER_PHASE7_END==AUX_LARGE_CONTROLLER_PHASE8_START and
            AUX_LARGE_CONTROLLER_PHASE8_END==AUX_LARGE_CONTROLLER_PHASE9_START and
            AUX_LARGE_CONTROLLER_PHASE9_END==AUX_LARGE_CONTROLLER_PHASE10_START and
            AUX_LARGE_CONTROLLER_PHASE10_END==AUX_LARGE_CONTROLLER_PHASE11_START and
            AUX_LARGE_CONTROLLER_PHASE11_END==AUX_LARGE_CONTROLLER_PHASE12_START):
        raise ValueError("missing source after controller registry block")
    if not (AUX_LARGE_CONTROLLER_PHASE12_END==AUX_LARGE_CONTROLLER_PHASE13_GATE_START and
            AUX_LARGE_CONTROLLER_PHASE13_GATE_END==AUX_LARGE_CONTROLLER_PHASE13_GAP_START and
            AUX_LARGE_CONTROLLER_PHASE13_GAP_END==AUX_LARGE_CONTROLLER_PHASE13_START and
            AUX_LARGE_CONTROLLER_PHASE13_END==AUX_LARGE_CONTROLLER_PHASE14_START and
            AUX_LARGE_CONTROLLER_PHASE14_END==AUX_LARGE_CONTROLLER_PHASE15_START and
            AUX_LARGE_CONTROLLER_PHASE15_END==AUX_LARGE_CONTROLLER_PHASE16_START and
            AUX_LARGE_CONTROLLER_PHASE16_END==AUX_LARGE_CONTROLLER_PHASE17_START and
            AUX_LARGE_CONTROLLER_PHASE17_END==AUX_LARGE_CONTROLLER_PHASE18_START and
            AUX_LARGE_CONTROLLER_PHASE18_END==AUX_LARGE_CONTROLLER_PHASE19A_START and
            AUX_LARGE_CONTROLLER_PHASE19A_END==AUX_LARGE_CONTROLLER_PHASE19B_START and
            AUX_LARGE_CONTROLLER_PHASE19B_END==AUX_LARGE_CONTROLLER_PHASE20_START and
            AUX_LARGE_CONTROLLER_PHASE20_END==AUX_LARGE_CONTROLLER_PHASE21_START and
            AUX_LARGE_CONTROLLER_PHASE21_END==AUX_LARGE_CONTROLLER_PHASE22_START and
            AUX_LARGE_CONTROLLER_PHASE22_END==AUX_LARGE_CONTROLLER_PHASE23_START and
            AUX_LARGE_CONTROLLER_PHASE23_END==AUX_LARGE_CONTROLLER_PHASE24_START and
            AUX_LARGE_CONTROLLER_PHASE24_END==AUX_LARGE_CONTROLLER_PHASE25_START and
            AUX_LARGE_CONTROLLER_PHASE25_END==AUX_LARGE_CONTROLLER_PHASE26_START and
            AUX_LARGE_CONTROLLER_PHASE26_END==AUX_LARGE_CONTROLLER_PHASE27_START and
            AUX_LARGE_CONTROLLER_PHASE27_END==AUX_LARGE_CONTROLLER_PHASE28_START and
            AUX_LARGE_CONTROLLER_PHASE28_END==AUX_LARGE_CONTROLLER_PHASE29_START and
            AUX_LARGE_CONTROLLER_PHASE29_END==AUX_LARGE_CONTROLLER_PHASE30_START and
            AUX_LARGE_CONTROLLER_PHASE30_END==AUX_LARGE_CONTROLLER_PHASE31_START and
            AUX_LARGE_CONTROLLER_PHASE31_END==AUX_LARGE_CONTROLLER_PHASE32_START and
            AUX_LARGE_CONTROLLER_PHASE32_END==AUX_LARGE_CONTROLLER_PHASE33_START and
            AUX_LARGE_CONTROLLER_PHASE33_END==AUX_LARGE_CONTROLLER_PHASE34_START and
            AUX_LARGE_CONTROLLER_PHASE34_END==AUX_LARGE_CONTROLLER_PHASE35_START and
            AUX_LARGE_CONTROLLER_PHASE35_END==AUX_LARGE_CONTROLLER_PHASE36_START and
            AUX_LARGE_CONTROLLER_PHASE36_END==AUX_LARGE_CONTROLLER_PHASE37_START and
            AUX_LARGE_CONTROLLER_PHASE37_END==AUX_LARGE_CONTROLLER_PHASE38_START and
            AUX_LARGE_CONTROLLER_PHASE38_END==AUX_LARGE_CONTROLLER_PHASE39_START and
            AUX_LARGE_CONTROLLER_PHASE39_END==AUX_LARGE_CONTROLLER_PHASE40_START and
            AUX_LARGE_CONTROLLER_PHASE40_END==AUX_LARGE_CONTROLLER_PHASE41_START and
            AUX_LARGE_CONTROLLER_PHASE41_END==AUX_LARGE_CONTROLLER_PHASE42_START and
            AUX_LARGE_CONTROLLER_PHASE42_END==AUX_LARGE_CONTROLLER_PHASE43_START and
            AUX_LARGE_CONTROLLER_PHASE43_END==AUX_LARGE_CONTROLLER_PHASE44_START and
            AUX_LARGE_CONTROLLER_PHASE44_END==AUX_LARGE_CONTROLLER_PHASE45_START and
            AUX_LARGE_CONTROLLER_PHASE45_END==AUX_LARGE_CONTROLLER_PHASE46_START):
        raise ValueError("controller phase13 gap was silently bridged")
    if AUX_LOOKUP_END!=AUX_REGISTRY_CORE_START or AUX_REGISTRY_CORE_END!=AUX_DISPATCH_START:
        raise ValueError("modem auxiliary registry source intervals have a gap")
    return {
        ATOMIC_START:atomic+controller,
        SERVICE_START:service+service_mmio+service_flags+service_alloc,
        AUX_START:auxiliary+aux_dynamic+aux_init+aux_align+
                  aux_guard+aux_stage+aux_mmio+aux_heap+aux_worker+
                        aux_lookup+aux_registry_core+registry+aux_notify_scan+aux_cleanup+
                        aux_factory+aux_find_format+aux_find_class+aux_controller_slot+
                        aux_large_controller_prefix+aux_large_controller_mid+aux_large_controller_phase2+
                        aux_large_controller_phase3+aux_large_controller_phase4+
                        aux_large_controller_phase5+aux_large_controller_phase6+
                        aux_large_controller_phase7+aux_large_controller_phase8+
                        aux_large_controller_phase9+aux_large_controller_phase10+
                        aux_large_controller_phase11+aux_large_controller_phase12+
                        aux_large_controller_phase13_gate+aux_large_controller_phase13_gap+
                        aux_large_controller_phase13+aux_large_controller_phase14+
                        aux_large_controller_phase15+aux_large_controller_phase16+
                        aux_large_controller_phase17+aux_large_controller_phase18+
                        aux_large_controller_phase19a+aux_large_controller_phase19b+
                        aux_large_controller_phase20+aux_large_controller_phase21+
                        aux_large_controller_phase22+aux_large_controller_phase23+
                        aux_large_controller_phase24+aux_large_controller_phase25+
                        aux_large_controller_phase26+aux_large_controller_phase27+
                        aux_large_controller_phase28+aux_large_controller_phase29+
                        aux_large_controller_phase30+aux_large_controller_phase31+
                        aux_large_controller_phase32+aux_large_controller_phase33+
                        aux_large_controller_phase34+aux_large_controller_phase35+
                        aux_large_controller_phase36+aux_large_controller_phase37+
                        aux_large_controller_phase38+aux_large_controller_phase39+
                        aux_large_controller_phase40+aux_large_controller_phase41+
                        aux_large_controller_phase42+aux_large_controller_phase43+
                        aux_large_controller_phase44+aux_large_controller_phase45+
                        aux_large_controller_phase46,
    }


def executable_segments(reference: bytes) -> dict[int, tuple[int, int, int, int]]:
    if reference[:6] != b"\x7fELF\x01\x01" or len(reference) < 52:
        raise ValueError("expected ELF32 little-endian original modem MBN")
    if struct.unpack_from("<H", reference, 18)[0] != 164:
        raise ValueError("original MBN is not Hexagon ELF")
    phoff = struct.unpack_from("<I", reference, 28)[0]
    phentsize, phnum = struct.unpack_from("<HH", reference, 42)
    if phentsize != 32 or phoff + phnum * phentsize > len(reference):
        raise ValueError("invalid original modem ELF program headers")
    segments = {}
    for i in range(phnum):
        ptype, offset, vma, pma, filesz, memsz, flags, align = struct.unpack_from(
            "<8I", reference, phoff + i * phentsize
        )
        if ptype == 1 and flags & 1:
            if offset in segments:
                raise ValueError(f"duplicate executable load at file offset {offset:#x}")
            if filesz > memsz or offset + filesz > len(reference):
                raise ValueError(f"invalid executable load at {offset:#x}")
            segments[offset] = (vma, pma, filesz, memsz)
    return segments


def verify_reference(original: bytes, fragments: dict[int, bytes]) -> None:
    digest = hashlib.sha256(original).hexdigest()
    if digest != EXPECTED_REFERENCE_SHA256:
        raise ValueError(f"wrong modem firmware revision, SHA256={digest}")
    segments = executable_segments(original)
    expected_map = {
        0x7C000: (0xC0890000, 0x8E090000, 0x183CF0),
        0x446000: (0x1E400000, 0x90000000, 0x11DA4),
    }
    for start, (vma, pma, min_filesz) in expected_map.items():
        actual = segments.get(start)
        if actual is None or actual[:3] != (vma, pma, min_filesz):
            raise ValueError(f"unexpected mapped executable segment {start:#x}: {actual}")
    for start, rebuilt in fragments.items():
        covering = [(s, p) for s, p in segments.items() if s <= start < s + p[2]]
        if len(covering) != 1:
            raise ValueError(f"fragment at {start:#x} is not wholly in exactly one executable segment")
        segment_start, (vma, pma, filesz, memsz) = covering[0]
        if start + len(rebuilt) > segment_start + filesz:
            raise ValueError(f"generated fragment at {start:#x} exceeds executable p_filesz")
        expected = original[start : start + len(rebuilt)]
        if expected != rebuilt:
            i = next(i for i, (a, b) in enumerate(zip(expected, rebuilt)) if a != b)
            raise ValueError(f"Hexagon mismatch at modem file {start+i:#x}: expected {expected[i]:02x}, generated {rebuilt[i]:02x}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--llvm-mc", required=True, type=Path)
    ap.add_argument("--reference", type=Path, help="original pinned modem.mbn stored outside Git")
    ap.add_argument("--output-dir", type=Path, help="directory for the generated, *partial* executable fragments")
    args = ap.parse_args()
    fragments = output_fragments(args.llvm_mc)
    if args.reference:
        verify_reference(args.reference.read_bytes(), fragments)
    if args.output_dir:
        args.output_dir.mkdir(parents=True, exist_ok=True)
        for start, blob in fragments.items():
            path = args.output_dir / f"modem-late-file-{start:06x}-{start+len(blob):06x}.bin"
            path.write_bytes(blob)
    for start, blob in fragments.items():
        print(f"Hexagon V60 independent source: file [{start:#x},{start+len(blob):#x}) "
              f"= {len(blob)} bytes, SHA256={hashlib.sha256(blob).hexdigest()}")
    metadata=AUX_DYNAMIC_END-AUX_DYNAMIC_START
    structural=AUX_INIT_ALIGN_END-AUX_INIT_ALIGN_START
    code_bytes=sum(len(x) for x in fragments.values())-metadata-structural
    print(f"Total generated {sum(len(x) for x in fragments.values())} PARTIAL modem bytes in "
          f"{len(fragments)} verified fragments, of which {code_bytes} are "
          f"source-encoded Hexagon executable code, {metadata} independently "
          f"derived ELF32 metadata and {structural} linker-alignment zeros; "
          f"original reference {'matched' if args.reference else 'not supplied'}; "
          "NOT complete, not new OEM authentication")
    return 0

if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, RuntimeError, OSError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        sys.exit(1)
