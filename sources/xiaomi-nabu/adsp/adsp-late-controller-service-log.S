// SPDX-License-Identifier: GPL-3.0-only
// [file 0x41080,0x41098) short runtime logging/report trampoline.
.text
.globl adsp_late_controller_service_log
adsp_late_controller_service_log:
{ call record_controller_error; immext(#4028154560); r1:0 = combine(#11,##4028154592); allocframe(#0) }
{ jump forward_to_report; deallocframe }
.set record_controller_error, adsp_late_controller_service_log-0x1914
.set forward_to_report, adsp_late_controller_service_log+0x4d10
