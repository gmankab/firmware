// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 [file 0x3e5a8,0x3e658) controller allocator:
// Reserve descriptor, install callback table,
// register platform state, and report allocation/activation failures.
.text
.globl adsp_late_controller_allocate
adsp_late_controller_allocate:
{ r17 = r0; memd(r29+#-16) = r17:16; allocframe(#40) }
{ immext(#0xf02cb000); r0 = ##-265493296; memd(r29+#24) = r19:18; memd(r29+#16) = r21:20 }
{ call read_platform; r21:20 = combine(r4,r5); r19:18 = combine(r3,r2) }
{ call allocate_descriptor; r0 = #72 }
{ r3:2 = combine(r19,r18); r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt allocation_failed }
{ r1:0 = combine(#0,#0); r5:4 = combine(#0,#0); memw(r16+#28) = r21; memw(r16+#32) = r20 }
{ r0 = r16; immext(#4026773248); memw(r16+#24) = ##-268194024; memd(r29+#0) = r1:0 }
{ call create_platform_registry; memw(r29+#8) = #0 }
{ r3:2 = combine(r16,#80); immext(#4027742400); r4 = ##4027742408; r1:0 = combine(r17,#0) }
{ immext(#5373888); r6 = ##5373950; memw(r16+#0) = r4 }
{ r17 = #0; immext(#4026774592); memw(r16+#8) = ##-268192644; memw(r16+#4) = r6 }
{ call activate_controller }
{ p0 = cmp.gt(r0,#-1); if (!p0.new) jump:nt registration_failed; jump report_final; if (p0.new) memw(r16+#12) = r0 }
allocation_failed:
{ r17 = #2; jump report_final }
registration_failed:
{ call free_controller; r17 = #12; r0 = r16 }
report_final:
{ call publish_result; immext(#0xf02cb000); r0 = ##-265493296 }
{ r0 = r17; r17:16 = memd(r29+#32); r19:18 = memd(r29+#24) }
{ r21:20 = memd(r29+#16); dealloc_return }
.set read_platform, adsp_late_controller_allocate+0xe82f8
.set allocate_descriptor, adsp_late_controller_allocate-0x3a3c
.set create_platform_registry, adsp_late_controller_allocate-0xad8
.set activate_controller, adsp_late_controller_allocate+0xec884
.set free_controller, adsp_late_controller_allocate-0x3608
.set publish_result, adsp_late_controller_allocate+0xe8438
