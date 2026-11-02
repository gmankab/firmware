// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3e000,0x3e00c): source-encoded epilogue/return continuation
// of a prior controller routine, not an independently callable function.
.text
.globl adsp_late_controller_priority_tail
adsp_late_controller_priority_tail:
{ call clear_registry_records }
{ r2 = #0 }
{ r0 = r2; dealloc_return }
.set clear_registry_records, adsp_late_controller_priority_tail-0x3e0
