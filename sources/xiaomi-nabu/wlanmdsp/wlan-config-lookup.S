// SPDX-License-Identifier: GPL-3.0-only
// DSP configuration/table selector: search for a matching 8-bit hardware
// tag in a packed constant table, then extract size, class and direction
// fields into the caller's five-word structure. A missing tag returns 1,
// a match returns 0, with bounds checks for supplied index/count.
// Hexagon packet braces are architecturally significant. In particular
// both mux and the conditional table-address selection share a four-word
// packet with their two immediate extenders; the new-value branches/stores
// below similarly require their producers in the same packet.

.text
.globl wlan_lookup_config
wlan_lookup_config:
{ r4 = #761; r5 = #0; p0 = cmp.eq(r0,#0) }
{ r6 = mux(p0,r4,##1183); if (p0) r4 = ##-1338758272 }
{ if (!p0) r4 = ##-1338763008 }
lookup_loop:
{ r7 = memw(r4+r5<<#2) }
{ r7 = lsr(r7,#24); r8 = xor(r7,r1) }
{ r8 = extractu(r8,#24,#0); if (cmp.eq(r8.new,#0)) jump:nt match_found }
{ r7 += add(r5,#1) }
{ r5 = r7; p0 = cmp.gtu(r6,r7); if (p0.new) jump:t lookup_loop }
{ r0 = #1; jump return_result }
match_found:
{ p0 = cmp.eq(r2,#-1); if (p0.new) jump:nt return_result; r0 = #0; memw(r3+#20) = r7 }
{ r5 = add(r2,r5); p0 = !cmp.gtu(r7,r2); r0 = #1; if (p0.new) jump:t return_result }
{ r0 = #0; memw(r3+#0) = r2 }
{ r4 = addasl(r4,r5,#2) }
{ r2 = memw(r4+#4) }
{ r2 = extractu(r2,#12,#0); memw(r3+#4) = r2.new }
{ r2 = memw(r4+#4) }
{ r2 = extractu(r2,#9,#12); memw(r3+#8) = r2.new }
{ r2 = memw(r4+#4) }
{ r2 = extractu(r2,#1,#30); memw(r3+#12) = r2.new }
{ r2 = memw(r4+#4) }
{ r2 = extractu(r2,#9,#21); jumpr r31; memw(r3+#16) = r2.new }
return_result:
{ jumpr r31 }
