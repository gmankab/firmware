// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 [file 0x3e7a0,0x3e7ac): default-mode tail-call trampoline.
.text
.globl adsp_late_controller_mode_default
adsp_late_controller_mode_default:
{ immext(#4028151936); r1:0 = combine(#10,##4028151984); jump dispatch_default }
.set dispatch_default, adsp_late_controller_mode_default+0xfcc
