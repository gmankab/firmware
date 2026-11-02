// SPDX-License-Identifier: GPL-3.0-only
// Initialize the codec/controller runtime's descriptor scratch record, then
// install the platform callback table with bounded retry and fatal fallback.
// All PC-relative functions are expressed symbolically from this function's
// original entry; device MMIO values are explicit immediate operands.
.text
.globl adsp_late_controller_runtime_init
adsp_late_controller_runtime_init:
{ allocframe(#16) }
{ call prepare_runtime_stack; r0 = add(r29,#0) }
{ call populate_runtime_record; r1 = #1; r0 = add(r29,#0) }
{ p0 = cmp.eq(r0,#-1); if (p0.new) jump:nt initialization_failed; immext(#4029473856); memw(##4029473896) = r0 }
{ call register_controller_mode; immext(#4029460352); r1:0 = combine(#1,##4029460352) }
retry_registration:
{ call install_controller_state; r2 = #0; r1 = #1; r0 = add(r29,#0) }
{ r0 = add(r29,#0); r2 = r0 }
{ call probe_controller_service; r1 = r2 }
{ call dispatch_controller_setup; immext(#4029473984); r0 = ##-265493296 }
{ call record_init_result }
{ call finalize_controller_mode; immext(#4029473984); r0 = ##-265493296 }
{ jump retry_registration }
initialization_failed:
{ call cleanup_bad_controller_state; immext(#4028151744); r0 = ##-266815544 }
{ call publish_controller_failure; immext(#4028151744); r0 = ##-266815502; r1 = #420 }
.set prepare_runtime_stack, adsp_late_controller_runtime_init+0xec420
.set populate_runtime_record, adsp_late_controller_runtime_init+0xed7a0
.set register_controller_mode, adsp_late_controller_runtime_init+0xecddc
.set install_controller_state, adsp_late_controller_runtime_init+0xed2b8
.set probe_controller_service, adsp_late_controller_runtime_init+0xec374
.set dispatch_controller_setup, adsp_late_controller_runtime_init+0xe87d0
.set record_init_result, adsp_late_controller_runtime_init-0x4b0
.set finalize_controller_mode, adsp_late_controller_runtime_init+0xe8910
.set cleanup_bad_controller_state, adsp_late_controller_runtime_init+0xe8d5c
.set publish_controller_failure, adsp_late_controller_runtime_init+0x824
