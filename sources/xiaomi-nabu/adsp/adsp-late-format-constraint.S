// ADSP V66 [file 0x3bdd4,0x3be70): validate codec input format bits,
// dispatch a bounded format conversion through SIMD packed shift and
// format-specific platform callbacks. Source reconstructs Hexagon packets.
.text
.globl adsp_late_format_constraint
adsp_late_format_constraint:
{ r17:16 = combine(r4,r5); memd(r29+#-16) = r17:16; allocframe(#40) }
{ r5 = and(r16,#15); r19:18 = combine(r1,r3); memd(r29+#24) = r19:18; memd(r29+#16) = r21:20 }
{ p0 = cmp.gtu(r5,#6); r21:20 = combine(r0,r2); r22 = memw(r29+#48); memd(r29+#8) = r23:22 }
{ if (!p0) r2 = #67; if (p0) jump:nt apply_fallback }
{ p0 = tstbit(r2,r5); r1:0 = combine(r19,r21); if (!p0.new) jump:nt apply_fallback; if (p0.new) memw(r29+#0) = r22 }
{ r2 = #7; r5 = r16; r4 = #3 }
{ r5 = or(r2,and(r5,#48)); call platform_convert; r3:2 = combine(r18,r20) }
{ r3:2 = vaslw(r19:18,#12); p0 = cmp.eq(r0,#0); if (p0.new) jump:nt apply_alternative }
{ r17:16 = memd(r29+#32); r19:18 = memd(r29+#24) }
{ r21:20 = memd(r29+#16); r23:22 = memd(r29+#8) }
{ dealloc_return }
apply_alternative:
{ call platform_recover; r1:0 = combine(r2,r3); r3:2 = combine(#1,#2) }
{ call platform_finalize; r1:0 = combine(r19,r21); r3 = r18 }
apply_fallback:
{ r1:0 = combine(r19,r21); r3:2 = combine(r18,r20); r5:4 = combine(r16,r17); memw(r29+#0) = r22 }
{ call platform_convert }
{ r17:16 = memd(r29+#32); r19:18 = memd(r29+#24) }
{ r21:20 = memd(r29+#16); r23:22 = memd(r29+#8) }
{ dealloc_return }
{ jump platform_finalize }
.set platform_convert, adsp_late_format_constraint-0x1b4
.set platform_recover, adsp_late_format_constraint+0xeb978
.set platform_finalize, adsp_late_format_constraint-0x74
