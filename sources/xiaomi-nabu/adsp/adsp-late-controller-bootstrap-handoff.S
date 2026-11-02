// SPDX-License-Identifier: GPL-3.0-only
// [file 0x39448,0x39464): one-shot ADSP controller platform bootstrap.
// Sets platform address and control flag, then tail-jumps to the dispatcher.
.text
.globl adsp_late_controller_bootstrap_handoff
adsp_late_controller_bootstrap_handoff:
{ call request_platform_init; immext(#4029460224); r0 = ##4029460272; allocframe(#0) }
{ immext(#4028146240); r1:0 = combine(#1,##4028146288); deallocframe }
{ jump controller_handoff }
.set request_platform_init, adsp_late_controller_bootstrap_handoff+0xef664
.set controller_handoff, adsp_late_controller_bootstrap_handoff+0x6324
