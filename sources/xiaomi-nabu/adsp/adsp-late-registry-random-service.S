// SPDX-License-Identifier: GPL-3.0-only
// ADSP firmware file [0x39320,0x39448): controller randomized probe/service.
// Based on symbolic Hexagon register and branch reconstruction.
.text
.globl adsp_late_registry_random_service
adsp_late_registry_random_service:
{ immext(#4029485888); r2 = memw(##4029485924); allocframe(#48) }
{ p0 = cmp.eq(r2,#0); r17:16 = combine(r0,r3); memd(r29+#40) = r17:16; memd(r29+#32) = r19:18 }
{ immext(#4029460288); if (!p0) r20 = ##-265506984; memd(r29+#24) = r21:20; memd(r29+#16) = r23:22 }
{ if (!p0) r23 = #1; if (p0) r0 = #0; if (p0) jump:nt restore_frame; memd(r29+#8) = r25:24 }
{ immext(#4029460288); r18 = ##-265506992; immext(#4029460288); r19 = ##-265507000 }
{ r22 = upcyclelo }
loop_body:
{ call platform_cycle; immext(#4029460224); r0 = ##-265507024 }
{ immext(#4029460288); r2 = memub(##4029460288) }
{ p0 = r2; if (!p0.new) r0 = #-2; if (p0.new) jump:nt xor_update; if (p0.new) r2 = memw(r20+#0) }
{ call platform_step }
{ call platform_step; r2 = add(r0,#1); r0 = #-2; memw(r19+#0) = r2.new }
{ call platform_step; r2 = add(r0,#1); r0 = #-2; memw(r18+#0) = r2.new }
{ r2 = add(r0,#1); memw(r20+#0) = r2.new }
{ immext(#4029460288); memb(##4029460288) = r23 }
xor_update:
{ r2 ^= asl(r2,#3); r6 = asl(r2,#17); r3 = memw(r19+#0); r4 = memw(r18+#0) }
{ r3 ^= asl(r3,#13); r24 = asl(r3,#12); r5 = #-128 }
{ r3 = lsr(r3,#19); r2 = lsr(r2,#11); immext(#4029460224); r0 = ##-265507024 }
{ r5 &= asl(r4,#4); r24 = insert(r3,#13,#0); memw(r19+#0) = r24.new }
{ r6 = insert(r2,#21,#0); r4 ^= asl(r4,#2); r3 = r5; memw(r20+#0) = r6.new }
{ r24 ^= xor(r6,r22); r2 = lsr(r4,#25) }
{ r3 = insert(r2,#7,#0); r24 ^= or(r2,r5); memw(r18+#0) = r3.new }
{ call platform_compare }
{ p0 = cmp.eq(r24,#0); if (p0.new) jump:t loop_body; if (!p0.new) memw(r29+#4) = r24 }
{ r3:2 = combine(r16,#83); r5 = #4; r4 = add(r29,#4); r1:0 = combine(r17,#0) }
{ call platform_report }
restore_frame:
{ r17:16 = memd(r29+#40); r19:18 = memd(r29+#32) }
{ r21:20 = memd(r29+#24); r23:22 = memd(r29+#16) }
{ r25:24 = memd(r29+#8); dealloc_return }
.set platform_cycle, adsp_late_registry_random_service+0xed580
.set platform_step, adsp_late_registry_random_service-0xf0
.set platform_compare, adsp_late_registry_random_service+0xed6c0
.set platform_report, adsp_late_registry_random_service+0xf1b0c
