// SPDX-License-Identifier: GPL-3.0-only
// Agent 13: run an optional per-record callback unless disabled by an
// object mode bit; restore the caller frame on all exit paths.
// File [0x42f74,0x42fac), VMA [0xb0002f74,0xb0002fac).
.text
.globl agent13_runtime_record_callback_guard
agent13_runtime_record_callback_guard:
{ call 0x3bdd4; memd(r29+#-16) = r17:16; allocframe(#8) }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt done }
{ r16 = memw(r0+#52); if (cmp.eq(r16.new,#0)) jump:nt done }
{ r2 = memub(r16+#19) }
{ p0 = tstbit(r2,#5); if (!p0.new) r0 = add(r16,#24); if (p0.new) jump:t done }
{ call 0x2189c4 }
{ r1:0 = combine(#1,r16); r17:16 = memd(r29+#0); deallocframe }
{ jump after }
done:
{ r17:16 = memd(r29+#0); dealloc_return }
after:
