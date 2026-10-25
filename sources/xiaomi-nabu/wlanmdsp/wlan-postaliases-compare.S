// SPDX-License-Identifier: GPL-3.0-only
// Hexagon consumer-retry branch followed by fieldwise record equality check.
// A disconnected runtime interval [0xb0001fc0,0xb0002010).
// The retry branch targets the existing entry at 0xb0001c80; this is a
// relative -0x340 displacement because this source is assembled separately.
.text
.globl wlan_record_retry
wlan_record_retry:
{ r2 = #1; jump ##-832; r1:0 = combine(r2,r1) }
wlan_record_compare:
{ r2 = r0; r0 = #0; r4 = memw(r1+#8) }
{ r3 = memw(r2+#8); if (!cmp.eq(r3.new,r4)) jump:t not_equal }
{ r3 = memw(r2+#12) }
{ r4 = memw(r1+#12); if (!cmp.eq(r4.new,r3)) jump:t not_equal }
{ r3 = memw(r2+#16) }
{ r4 = memw(r1+#16); if (!cmp.eq(r4.new,r3)) jump:t not_equal }
{ r3 = memw(r2+#20) }
{ r4 = memw(r1+#20); if (!cmp.eq(r4.new,r3)) jump:t not_equal }
{ r2 = memw(r2+#0); r3 = memw(r1+#0) }
{ p0 = !cmp.eq(r2,r3); if (!p0.new) r0 = #1 }
not_equal:
{ jumpr r31 }
