// SPDX-License-Identifier: GPL-3.0-only
// ADSP [0x3c2f4,0x3c358): validate controller codec mode, probe current
// descriptor, invoke optional callback and restore the caller frame.
.text
.globl adsp_late_controller_check_mode
adsp_late_controller_check_mode:
{ call platform_enter }
{ r17:16 = combine(r0,r2); r0 = #29; memd(r29+#-16) = r17:16; allocframe(#24) }
{ call platform_query; r18 = r1; memd(r29+#8) = r19:18; memd(r29+#0) = r21:20 }
{ r21:20 = combine(r1,r0) }
{ if (p0.new) r20 = #0; p0 = cmp.eq(r20,#0); if (p0.new) jump:nt return_mode }
{ p0 = cmp.gt(r20,#-1); if (!p0.new) jump:nt return_mode }
{ immext(#4029477376); r2 = memw(r18<<#2+##4029477384) }
{ r1 = memw(r2+#4); if (!cmp.eq(r1.new,r17)) jump:t invalid_mode }
{ call next_handler; r1:0 = combine(r16,r17); r20 = #0 }
{ jump return_mode }
invalid_mode:
{ call platform_bad_mode; r3:2 = combine(#0,#101); r0 = #0; r20 = #0 }
return_mode:
{ r0 = r20; r17:16 = memd(r29+#16); r19:18 = memd(r29+#8) }
{ r21:20 = memd(r29+#0); dealloc_return }
.set platform_enter, adsp_late_controller_check_mode+0xea8f4
.set platform_query, adsp_late_controller_check_mode+0x3358
.set next_handler, adsp_late_controller_check_mode+0x6c
.set platform_bad_mode, adsp_late_controller_check_mode+0xeeb38
