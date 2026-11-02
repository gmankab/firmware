// SPDX-License-Identifier: GPL-3.0-only
// File [0x1056e0,0x105744): normalize controller channel count and reset per-channel status.
.text
.globl agent14_late_channel_count_reset
agent14_late_channel_count_reset:
{ r16 = r0; r3 = #0x7; memd(r29+#-0x10) = r17:16; allocframe(#0x8) }
{ r2 = memw(r16+#0x1dc) }
{ p0 = cmp.gtu(r3,r2); r3 = #0x6; if (!p0.new) memw(r16+##0x1dc) = r3.new }
{ if (!p0) jump:nt initialize }
{ r3 = r2; p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt done }
initialize:
{ r1 = +mpyi(r3,##0x130); call 0x157224; r0 = #0x3 }
{ r2 = memw(r16+#0x1dc); memw(r16+#0x2d8) = r0 }
{ p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt done }
{ r3 = #0x0; r2 = add(r0,#0x1) }
loop:
{ r2 = add(r2,#0x130); r3 = add(r3,#0x1); memb(r2+#0x0) = #1 }
{ r4 = and(r3,#0xff) }
{ r5 = memw(r16+#0x1dc); if (cmp.gtu(r5.new,r4)) jump:t loop }
done:
{ r17:16 = memd(r29+#0x0); dealloc_return }
