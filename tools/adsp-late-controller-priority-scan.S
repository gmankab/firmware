// SPDX-License-Identifier: GPL-3.0-only
// ADSP executable [0x3e010,0x3e0a4): find and reorder an active priority
// list using checked controller-state and a tail-call to a platform cleanup.
.text
.globl adsp_late_controller_priority_scan
adsp_late_controller_priority_scan:
{ r16 = r0; r0 = #4; memd(r29+#-16) = r17:16; allocframe(#8) }
{ if (!p0.new) r0 = add(r16,#0); p0 = cmp.gtu(r16,#4); if (p0.new) jump:t finish }
{ call acquire_priority_lock }
{ immext(#4029473856); r2 = memw(r16<<#2+##4029473904); if (cmp.eq(r2.new,#0)) jump:nt unlock_return }
{ immext(#4029473856); r16 = add(##4029473904,asl(r16,#2)); immext(#4028567744); r3 = ##-266399544 }
repeat_priority:
{ r4 = #0; r5 = memw(r3+#0); if (cmp.eq(r5.new,#0)) jump:nt unlink_found }
{ r1:0 = memd(r2+#56) }
compare_priority:
{ r7:6 = memd(r5+#56) }
{ p0 = cmp.gtu(r7:6,r1:0); if (!p0.new) r3 = add(r5,#64); if (p0.new) jump:nt select_previous; if (!p0.new) r6 = memw(r5+#64) }
{ r5 = r6; p0 = cmp.eq(r6,#0); if (!p0.new) jump:t compare_priority }
{ jump unlink_found }
select_previous:
{ r4 = r5 }
unlink_found:
{ r5 = memw(r2+#64); memw(r2+#64) = r4 }
{ r2 = r5; p0 = cmp.eq(r5,#0); r4 = memw(r16+#0); memw(r3+#0) = r4.new }
{ if (!p0) jump:nt repeat_priority; memw(r16+#0) = r5 }
unlock_return:
{ call release_priority_lock }
{ r0 = #0 }
finish:
{ r17:16 = memd(r29+#0); dealloc_return }
.set acquire_priority_lock, adsp_late_controller_priority_scan-0xd0
.set release_priority_lock, adsp_late_controller_priority_scan-0x3f0
