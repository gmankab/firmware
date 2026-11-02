// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 [file 0x3e7ac,0x3e8f4): dispatch opcode into QuRT session handlers,
// respecting ABI spill slots, saved-register frames and 87-entry table bound.
.text
.globl adsp_late_controller_switch
adsp_late_controller_switch:
{ r14 = add(r2,#-3); r17 = r3; memd(r29+#-16) = r17:16; allocframe(#32) }
{ r7:6 = combine(r4,r5); p0 = cmp.gtu(r14,#86); r9 = memw(r29+#60); r12 = memw(r29+#52) }
{ r3 = r0; r13 = memw(r29+#44); r8 = memw(r29+#56) }
{ if (p0) jump:nt generic_case; r4 = memw(r29+#48); r5 = memw(r29+#40) }
{ immext(#4028151360); r14 = memw(r14<<#2+##4028151404) }
{ jumpr r14 }
case_interface:
{ r1:0 = combine(r3,#0); r3:2 = combine(r17,#81); r17:16 = memd(r29+#24); deallocframe }
{ jump interface_dispatch }
generic_case:
{ r0 = r3; r3 = r17; memw(r29+#8) = r4; memw(r29+#0) = r5 }
{ r5:4 = combine(r6,r7); memw(r29+#20) = r9; memw(r29+#16) = r8 }
{ call forward_session; memw(r29+#12) = r12; memw(r29+#4) = r13 }
{ r17:16 = memd(r29+#24); dealloc_return }
case1:
{ r1 = r17; r3:2 = combine(r6,r7); r17:16 = memd(r29+#24); deallocframe }
{ jump handler1 }
case2:
{ r0 = r17; r17:16 = memd(r29+#24); deallocframe }
{ jump handler2 }
case3:
{ r0 = r17; r17:16 = memd(r29+#24); deallocframe }
{ jump handler3 }
case4:
{ r0 = r17; r3:2 = combine(r7,r6); r17:16 = memd(r29+#24); deallocframe }
{ jump handler4 }
case5:
{ r16 = r7; r4 = r6 }
{ r3:2 = combine(r17,r16); r17:16 = memd(r29+#24); deallocframe }
{ jump handler5 }
case6:
{ r1:0 = combine(r7,r17); r17:16 = memd(r29+#24); deallocframe }
{ jump handler6 }
case7:
{ r0 = r17; r17:16 = memd(r29+#24); deallocframe }
{ jump handler7 }
case8:
{ call platform_get; immext(#4029473984); r0 = ##-265493296 }
{ call mode_handler8; r0 = r17 }
{ call platform_log; immext(#4029473984); r0 = ##4029474000; r16 = r0 }
{ r0 = r16; r17:16 = memd(r29+#24); dealloc_return }
case9:
{ call platform_get; immext(#4029473984); r0 = ##-265493296 }
{ call mode_handler9; r0 = r17 }
{ call platform_log; immext(#4029473984); r0 = ##4029474000; r16 = r0 }
{ r0 = r16; r17:16 = memd(r29+#24); dealloc_return }
case10:
{ call platform_get; immext(#4029473984); r0 = ##-265493296 }
{ call mode_handler10 }
{ call platform_log; immext(#4029473984); r0 = ##-265493296 }
{ r0 = #0; r17:16 = memd(r29+#24); dealloc_return }
.set interface_dispatch, adsp_late_controller_switch+0xec680
.set forward_session, adsp_late_controller_switch+0xec11c
.set handler1, adsp_late_controller_switch-0x54c
.set handler2, adsp_late_controller_switch-0x444
.set handler3, adsp_late_controller_switch-0x3dc
.set handler4, adsp_late_controller_switch-0x2dc
.set handler5, adsp_late_controller_switch-0x204
.set handler6, adsp_late_controller_switch-0x154
.set handler7, adsp_late_controller_switch-0x5c
.set platform_get, adsp_late_controller_switch+0xe80f4
.set platform_log, adsp_late_controller_switch+0xe8234
.set mode_handler8, adsp_late_controller_switch-0x79c
.set mode_handler9, adsp_late_controller_switch-0x80c
.set mode_handler10, adsp_late_controller_switch-0xb8c
