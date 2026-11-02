// SPDX-License-Identifier: GPL-3.0-only
.text
.globl adsp_late_controller_mode_validate
adsp_late_controller_mode_validate:
{ p0 = cmp.eq(r0,#3); r16 = r5; memd(r29+#-16) = r17:16; allocframe(#24) }
{ r19 = r3; r0 = #-1; if (!p0) jump:nt done; memd(r29+#8) = r19:18 }
{ r2 = or(r4,r19); if (p0.new) r0 = #0; p0 = cmp.eq(r4,#0); if (p0.new) jump:nt done }
{ r2 = extractu(r2,#12,#0); if (!cmp.eq(r2.new,#0)) jump:t done }
{ call precondition; r17 = lsr(r4,#12); r0 = #3 }
{ call init_service; immext(#4029470912); r0 = ##4029470952; r18 = r0 }
{ r19 = lsr(r19,#12); p0 = cmp.eq(r17,#0); if (p0.new) jump:nt skip_calls }
loop_body:
{ r2 = asl(r19,#12); r1 = add(r29,#4); r3 = #-1; r0 = memw(r18+#12) }
{ call query_state }
{ if (p0.new) r5:4 = combine(r16,r17); p0 = cmp.eq(r0,#0); if (!p0.new) jump:nt failed }
{ immext(#192); r3:2 = combine(r19,##195); r0 = #0; r1 = memw(r29+#4) }
{ call dispatch_state }
{ p0 = cmp.gt(r0,#0); if (!p0.new) jump:nt failed }
{ r17 = sub(r17,r0); r19 = add(r0,r19) }
{ p0 = cmp.eq(r17,#0); if (!p0.new) jump:t loop_body }
skip_calls:
{ call stop_service; immext(#4029470912); r0 = ##-265496344 }
{ r0 = #0; jump done }
failed:
{ call stop_service; immext(#4029470912); r0 = ##-265496344 }
{ r0 = #2 }
done:
{ r17:16 = memd(r29+#16); r19:18 = memd(r29+#8) }
{ dealloc_return }
.set precondition, adsp_late_controller_mode_validate+0x2368
.set init_service, adsp_late_controller_mode_validate+0xe91b8
.set query_state, adsp_late_controller_mode_validate-0x700
.set dispatch_state, adsp_late_controller_mode_validate+0xe9d8c
.set stop_service, adsp_late_controller_mode_validate+0xe9300
