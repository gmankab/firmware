// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x101694, 0x101724),
// runtime [0xb00c1694, 0xb00c1724).
// Combine external timing readings, a 16-bit interval shifted left 10, an input
// offset, and a stored count. Update both the count and output-time pointer.
// The unsigned 10000 threshold uses the OLD timing delta before count/offset
// are added; preserve that packet dependency. External clock APIs are unnamed.
// Return the updated count in r0. Store the count before the output time; if
// the two output pointers alias, the later time store overwrites the count.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_timing_count_adjust
wlan_late_timing_count_adjust:
{ call 0x1e40a8; allocframe(#0x38) }
{ r17:16 = combine(r3,r0); r19:18 = combine(r1,r2) }
{ r4 = memw(r16+#0x14); r0 = memw(r16+#0x2c) }
{ r24 = memw(r19+#0x0) }
{ r4 = memuh(r4+#0x44) }
{ call 0x141134; r20 = asl(r4,#0xa) }
// r23:r22 preserve the prior timing result while r0 loads the next input.
{ call 0x13f684; r23:22 = combine(r1,r0); r0 = memw(r16+#0x2c) }
{ call 0x155708; r16 = r0 }
// r21 saves the incoming clock result while r0 becomes a diagnostic ID.
{ r1 = #0x2413; r0 = ##0x6508; r21 = r0 }
{ r16 = sub(r22,r16); memw(r29+#0x8) = r21 }
{ call 0x159ad4; memw(r29+#0x4) = r16; memw(r29+#0x0) = r20 }
{ call -0xbf1d4; r1:0 = combine(r20,##-0x4fd48fc0); r3:2 = combine(r21,r16) }
{ r1 = mpyi(r20,r24); r2 = #0x2710 }
// p0 compares 10000 with OLD r16; r16 simultaneously gains count*interval
// and the input offset. If the old delta is large, r20 is zeroed.
{ r16 += add(r1,r18); p0 = cmp.gtu(r2,r16); if (!p0.new) r20 = #0x0 }
{ r20 += add(r16,r21); r3 = mux(p0,#0x1,#0x0) }
{ r0 = add(r24,r3); memw(r19+#0x0) = r0.new }
{ jump 0x1e40fc; memw(r17+#0x0) = r20 }
