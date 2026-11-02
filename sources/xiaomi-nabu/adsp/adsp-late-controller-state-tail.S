// SPDX-License-Identifier: GPL-3.0-only
// Tail-call variant forwarding a controller handle and a fixed register
// base to the platform. The symbolic target is PC-relative.
.text
.globl adsp_late_controller_state_tail
adsp_late_controller_state_tail:
{ r2 = r0 }
{ immext(#4029460480); r1:0 = combine(r2,##-265506816); jump platform_state_setter }
.set platform_state_setter, adsp_late_controller_state_tail+0xa54
