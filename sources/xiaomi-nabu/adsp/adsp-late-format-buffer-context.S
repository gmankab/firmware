// ADSP V66 [file 0x3a7f0,0x3a86c): scan controller buffer descriptors,
// allocate a context on demand, trace mismatched pointers, and traverse
// chained records until the terminator. Source packet groupings are authored
// from Hexagon operations, not from OEM instruction word imports.
.text
.globl adsp_late_format_buffer_context
adsp_late_format_buffer_context:
{ r17:16 = combine(r0,r1); memd(r29+#-16) = r17:16; allocframe(#40) }
{ immext(#0xf02cb000); r21 = memw(##4029462752); memd(r29+#16) = r21:20 }
{ p0 = cmp.eq(r21,#0); if (p0.new) jump:nt finish; memd(r29+#24) = r19:18 }
{ r18 = lsr(r2,#12) }
scan_cycle:
{ r2 = memw(r21+#16); if (!cmp.eq(r2.new,r17)) jump:t restore_outer }
{ call platform_acquire; r1:0 = combine(r18,#0) }
{ r19 = r0; if (cmp.eq(r19.new,#0)) jump:nt restore_outer }
{ immext(#0xf01fc000); r0 = ##-266820299; r2 = memw(r16+#4); memw(r16+#0) = r19 }
{ r20 = lsr(r2,#12); r3 = memw(r21+#16); memw(r29+#8) = r3.new }
{ r2 = r20; memw(r29+#0) = r19 }
{ r2 += add(r19,#-1); call platform_log; memw(r29+#4) = r2.new }
{ call platform_publish; r0 = add(r21,#24); r1 = r19; r2 = r20 }
restore_outer:
{ r21 = memw(r21+#12); if (!cmp.eq(r21.new,#0)) jump:t scan_cycle }
finish:
{ r17:16 = memd(r29+#32); r19:18 = memd(r29+#24) }
{ r21:20 = memd(r29+#16); dealloc_return }
.set platform_acquire, adsp_late_format_buffer_context+0x1278
.set platform_log, adsp_late_format_buffer_context+0xec63c
.set platform_publish, adsp_late_format_buffer_context+0x7a60
