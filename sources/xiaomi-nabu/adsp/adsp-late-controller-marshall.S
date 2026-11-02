// SPDX-License-Identifier: GPL-3.0-only
.text
.globl adsp_late_controller_marshall
adsp_late_controller_marshall:
{ r2 = #33; r16 = r5; memd(r29+#-16) = r17:16; allocframe(#64) }
{ r19:18 = combine(r3,r0); r20 = r4; memd(r29+#48) = r19:18; memd(r29+#40) = r21:20 }
{ p0 = cmp.gtu(r2,r19); if (p0.new) jump:t good_range }
{ r17 = #-4; r19 = #32; r2 = add(r19,#-1) }
{ p0 = cmp.gtu(r2,#32); if (p0.new) jump:t done }
good_range:
{ r5:4 = combine(r19,r20); p0 = cmp.eq(r18,#3); if (!p0.new) jump:t setup_argument; if (!p0.new) memw(r29+#4) = #0 }
{ call platform_query; r1:0 = combine(r19,r20) }
{ r17 = #-4; p0 = cmp.eq(r0,#0); if (p0.new) jump:nt done }
{ r17 = #0; jump done; r2 = memw(r0+#20); memw(r16+#0) = r2.new }
setup_argument:
{ r2 = #82; r3 = add(r29,#8); r1:0 = combine(r18,#0) }
{ call update_parameter }
{ r17 = #-4; p0 = cmp.gt(r0,#-1); if (!p0.new) jump:nt done }
{ call platform_push; immext(#4028154560); r1:0 = combine(##-266812719,r20); r2 = r19 }
{ if (!p0.new) r1:0 = combine(r19,r20); p0 = cmp.eq(r0,#0); if (p0.new) jump:nt failed_init }
{ call platform_query }
{ r6 = r0; memw(r29+#4) = r6.new }
{ p0 = cmp.eq(r6,#0); if (!p0.new) jump:t success_init }
{ jump done }
failed_init:
{ call update_parameter; r2 = #90; r3 = add(r29,#4); r1:0 = combine(r18,#0) }
{ r6 = memw(r29+#4); if (cmp.eq(r6.new,#0)) jump:nt done }
success_init:
{ r3:2 = combine(r16,#83); r5 = #4; r4 = add(r29,#0); r1:0 = combine(r18,#0) }
{ call update_parameter; r6 = memw(r6+#20); memw(r29+#0) = r6.new }
{ p0 = cmp.gt(r0,#-1); if (p0.new) r17 = #0 }
done:
{ r0 = r17; r17:16 = memd(r29+#56); r19:18 = memd(r29+#48) }
{ r21:20 = memd(r29+#40); dealloc_return }
.set platform_query, adsp_late_controller_marshall+0x2f4
.set update_parameter, adsp_late_controller_marshall+0xe9ce0
.set platform_push, adsp_late_controller_marshall+0xfe484
