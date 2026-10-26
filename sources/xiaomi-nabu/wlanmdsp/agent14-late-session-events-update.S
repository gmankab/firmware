// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x1067d4,0x1068bc); runtime [0xb00c67d4,0xb00c68bc).
// Process session events and report status with packed code.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_events_update
agent14_late_session_events_update:
pkt_0000:
// offset +0x0000, 3 encoded words
{ call 1961892; r1:0 = combine(#-0x1,#-0x2); r7:6 = combine(#0,#0x2); 	allocframe(#0x20) }
pkt_000c:
// offset +0x000c, 2 encoded words
{ r1:0 = add(r3:2,r1:0); r17:16 = combine(r5,r4) }
pkt_0014:
// offset +0x0014, 2 encoded words
{ p0 = cmp.gtu(r7:6,r1:0); if (!p0.new) jump:t pkt_004c }
pkt_001c:
// offset +0x001c, 2 encoded words
{ call -1808; r0 = memw(r16+#0x8); 	r1 = memuh(r16+#0x0) }
pkt_0024:
// offset +0x0024, 2 encoded words
{ r18 = #0x0; p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_00a8 }
pkt_002c:
// offset +0x002c, 2 encoded words
{ r2 = memb(r16+#0x14); if (!cmp.eq(r2.new,#0x4)) jump:t pkt_003c }
pkt_0034:
// offset +0x0034, 2 encoded words
{ r18 = #0x3; memb(r16+#0x16) = setbit(#0x0) }
pkt_003c:
// offset +0x003c, 2 encoded words
{ r2 = memub(r16+#0x15); if (cmp.eq(r2.new,#0x8)) jump:nt pkt_0080 }
pkt_0044:
// offset +0x0044, 1 encoded words
{ p0 = cmp.eq(r2,#0x4); if (p0.new) jump:t pkt_0094 }
pkt_0048:
// offset +0x0048, 1 encoded words
{ jump pkt_009c }
pkt_004c:
// offset +0x004c, 1 encoded words
{ r1:0 = combine(#0x0,#0x1) }
pkt_0050:
// offset +0x0050, 3 encoded words
{ p0 = cmp.eq(r3:2,r1:0); if (!p0.new) jump:t pkt_00a8; if (p0.new) r2 = memb(r16+#0x15) }
pkt_005c:
// offset +0x005c, 2 encoded words
{ p0 = cmp.eq(r2,#0x8); if (p0.new) r0 = add(r16,#0x0) }
pkt_0064:
// offset +0x0064, 2 encoded words
{ if (p0) call ##132 }
pkt_006c:
// offset +0x006c, 2 encoded words
{ call -1320; r1 = #0x7; 	r0 = memub(r17+#0x3) }
pkt_0074:
// offset +0x0074, 1 encoded words
{ r2 = #0xfc }
pkt_0078:
// offset +0x0078, 2 encoded words
{ jump pkt_00a8; memb(r16+#0x16) &= r2 }
pkt_0080:
// offset +0x0080, 2 encoded words
{ call -1908; r0 = memw(r16+#0x8); 	r1 = memub(r17+#0x3) }
pkt_0088:
// offset +0x0088, 2 encoded words
{ if (!p0.new) r0 = add(r16,#0x0); p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_009c }
pkt_0090:
// offset +0x0090, 1 encoded words
{ call 184 }
pkt_0094:
// offset +0x0094, 2 encoded words
{ r18 = setbit(r18,#0x2); memb(r16+#0x16) = setbit(#0x1) }
pkt_009c:
// offset +0x009c, 3 encoded words
{ call -1480; r1 = r18; r2 = #0x4; r0 = memub(r17+#0x3) }
pkt_00a8:
// offset +0x00a8, 4 encoded words
{ r0 = ##0x6798; r1 = #0x2413; r2 = memub(r16+#0x15) }
pkt_00b8:
// offset +0x00b8, 2 encoded words
{ r3 = memub(r16+#0x14); r4 = memub(r16+#0x16) }
pkt_00c0:
// offset +0x00c0, 1 encoded words
{ memw(r29+#0x8) = r4; 	memw(r29+#0x4) = r3 }
pkt_00c4:
// offset +0x00c4, 2 encoded words
{ call 1394972; memw(r29+#0x0) = r2 }
pkt_00cc:
// offset +0x00cc, 4 encoded words
{ r0 = ##-0x4fd4889c; r1 = memub(r16+#0x15); r2 = memub(r16+#0x14) }
pkt_00dc:
// offset +0x00dc, 2 encoded words
{ call 1961788; r3 = memub(r16+#0x16) }
pkt_00e4:
// offset +0x00e4, 1 encoded words
{ jump -803748 }
