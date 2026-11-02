// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3bd60,0x3bdd4): bounded format-plane allocation and iteration.
// Power-of-two capacity update, checked platform parameters and early exits.
.text
.globl adsp_late_controller_format_iterations
adsp_late_controller_format_iterations:
{ r17:16 = combine(r3,#0); memd(r29+#-16) = r17:16; allocframe(#48) }
{ p0 = cmp.eq(r17,#0); r19:18 = combine(r0,r1); memd(r29+#32) = r19:18; memd(r29+#24) = r21:20 }
{ if (p0) jump:nt finish; nop; nop }
retry_iteration:
{ r0 = asl(r18,#12); r1 = add(r29,#8); r2 = add(r29,#4); r3 = memw(r19+#8) }
{ call query_controller_format }
{ r20 = #1; p0 = cmp.eq(r0,#0); if (!p0.new) jump:t update_records; if (p0.new) r2 = memw(r29+#8) }
{ r3 = sub(#0,r2) }
{ r2 = and(r2,r3) }
{ r20 = mpyi(r2,r2); if (cmp.gtu(r20.new,r17)) jump:nt fail_allocation }
{ call reserve_input_storage; r0 = memw(r29+#4) }
update_records:
{ r17 = sub(r17,r20); r18 = add(r20,r18) }
{ p0 = cmp.eq(r17,#0); if (!p0.new) jump:t retry_iteration }
{ jump finish }
fail_allocation:
{ r16 = #4 }
finish:
{ r0 = r16; r17:16 = memd(r29+#40); r19:18 = memd(r29+#32) }
{ r21:20 = memd(r29+#24); dealloc_return }
.set query_controller_format, adsp_late_controller_format_iterations+0xee2f8
.set reserve_input_storage, adsp_late_controller_format_iterations+0xee258
