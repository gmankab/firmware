// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3bac0,0x3bb3c): bounded controller command cases 55/56,
// field validation, forwarded callbacks and sign-normalized result.
.text
.globl adsp_late_controller_case_router
adsp_late_controller_case_router:
{ p0 = cmp.eq(r2,#56); if (p0.new) jump:nt case_56; allocframe(#24) }
{ p0 = cmp.eq(r2,#55); if (!p0.new) jump:nt case_other; r7 = memw(r29+#36); r6 = memw(r29+#32) }
{ p0 = cmp.eq(r0,#3); if (!p0.new) jump:t normalize_result; r0 = #13; r7 = and(r7,#255) }
{ r1:0 = combine(r3,#3); r3:2 = combine(r5,r4) }
{ call route_55; r5:4 = combine(r7,r6) }
{ jump normalize_result }
case_56:
{ call route_56; r1 = r3; r3:2 = combine(r5,r4) }
normalize_result:
{ r2 = sub(#0,r0); p0 = cmp.gt(r0,#-1) }
{ if (!p0) r2 = add(r0,#0) }
{ r0 = r2; dealloc_return }
case_other:
{ r8 = memw(r29+#52); memw(r29+#20) = r8.new }
{ r12 = memw(r29+#44); memw(r29+#12) = r12.new }
{ r9 = memw(r29+#48); memw(r29+#16) = r9.new }
{ r13 = memw(r29+#40); memw(r29+#8) = r13.new }
{ call case_other_platform; memw(r29+#4) = r7; memw(r29+#0) = r6 }
{ dealloc_return }
.set route_55, adsp_late_controller_case_router-0x190
.set route_56, adsp_late_controller_case_router-0xc4
.set case_other_platform, adsp_late_controller_case_router+0xeee08
