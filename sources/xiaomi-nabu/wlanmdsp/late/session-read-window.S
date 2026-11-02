// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106d84,0x106e8c); runtime [0xb00c6d84,0xb00c6e8c).
// Process buffered session window with timeout and error reporting.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_read_window
agent14_late_session_read_window:
pkt_0000:
// offset +0x0000, 3 encoded words
{ r1:0 = combine(#-0x1,#-0x2); r7:6 = combine(#0x0,#0x2); memd(r29+#-0x10) = r17:16; 	allocframe(#0x10) }
pkt_000c:
// offset +0x000c, 2 encoded words
{ r1:0 = add(r3:2,r1:0); r17:16 = combine(r5,r4) }
pkt_0014:
// offset +0x0014, 3 encoded words
{ p0 = cmp.gtu(r7:6,r1:0); if (p0.new) jump:nt pkt_00a8; if (p0.new) r2 = memb(r16+#0x15) }
pkt_0020:
// offset +0x0020, 1 encoded words
{ r1:0 = combine(#0x0,#0xf) }
pkt_0024:
// offset +0x0024, 2 encoded words
{ p0 = cmp.eq(r3:2,r1:0); if (p0.new) jump:nt pkt_00cc }
pkt_002c:
// offset +0x002c, 1 encoded words
{ r1:0 = combine(#0x0,#0xe) }
pkt_0030:
// offset +0x0030, 2 encoded words
{ p0 = cmp.eq(r3:2,r1:0); if (!p0.new) jump:nt pkt_00c8 }
pkt_0038:
// offset +0x0038, 2 encoded words
{ r2 = memw(r16+#0x54); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_00c8 }
pkt_0040:
// offset +0x0040, 1 encoded words
{ call 1398828 }
pkt_0044:
// offset +0x0044, 3 encoded words
{ r1 = #0x2451; r0 = ##0x6800; 	memd(r29+#0x0) = r1:0 }
pkt_0050:
// offset +0x0050, 1 encoded words
{ call 1393632 }
pkt_0054:
// offset +0x0054, 1 encoded words
{ call 1398808 }
pkt_0058:
// offset +0x0058, 1 encoded words
{ r3:2 = combine(r1,r0) }
pkt_005c:
// offset +0x005c, 3 encoded words
{ call -803224; r1:0 = combine(r2,##-0x4fd48800) }
pkt_0068:
// offset +0x0068, 1 encoded words
{ call 1398788 }
pkt_006c:
// offset +0x006c, 4 encoded words
{ r3:2 = combine(r1,r0); r5 = #0x7d; r4 = memw(r16+#0x40); r0 = memw(r16+#0x48) }
pkt_007c:
// offset +0x007c, 2 encoded words
{ r3 = sub(r2,r4); memw(r16+#0x44) = r2 }
pkt_0084:
// offset +0x0084, 2 encoded words
{ r3 = add(r0,r3); r0 = add(r16,#0x18) }
pkt_008c:
// offset +0x008c, 1 encoded words
{ r4 = lsr(r3,#0x3) }
pkt_0090:
// offset +0x0090, 2 encoded words
{ p0 = cmp.gtu(r5,r4); if (!p0.new) r3 = #0x3e8 }
pkt_0098:
// offset +0x0098, 2 encoded words
{ memw(r16+#0x48) = r3; r17:16 = memd(r29+#0x8) }:mem_noshuf
pkt_00a0:
// offset +0x00a0, 2 encoded words
{ jump 1395512; deallocframe }
pkt_00a8:
// offset +0x00a8, 2 encoded words
{ p0 = cmp.eq(r2,#0x8); if (p0.new) r0 = add(r16,#0x0) }
pkt_00b0:
// offset +0x00b0, 2 encoded words
{ if (p0) call ##-1400 }
pkt_00b8:
// offset +0x00b8, 2 encoded words
{ call -2852; r1 = #0x7; 	r0 = memub(r17+#0x3) }
pkt_00c0:
// offset +0x00c0, 1 encoded words
{ r2 = #0xfc }
pkt_00c4:
// offset +0x00c4, 1 encoded words
{ memb(r16+#0x16) &= r2 }
pkt_00c8:
// offset +0x00c8, 1 encoded words
{ r17:16 = memd(r29+#0x8); 	dealloc_return }
pkt_00cc:
// offset +0x00cc, 1 encoded words
{ call 1398688 }
pkt_00d0:
// offset +0x00d0, 3 encoded words
{ r1 = #0x2451; r0 = ##0x6808; 	memd(r29+#0x0) = r1:0 }
pkt_00dc:
// offset +0x00dc, 1 encoded words
{ call 1393492 }
pkt_00e0:
// offset +0x00e0, 1 encoded words
{ call 1398668 }
pkt_00e4:
// offset +0x00e4, 1 encoded words
{ r3:2 = combine(r1,r0) }
pkt_00e8:
// offset +0x00e8, 3 encoded words
{ call -803364; r1:0 = combine(r2,##-0x4fd487f4) }
pkt_00f4:
// offset +0x00f4, 2 encoded words
{ r2 = memw(r16+#0x54); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_00c8 }
pkt_00fc:
// offset +0x00fc, 2 encoded words
{ r0 = r16; r17:16 = memd(r29+#0x8); 	deallocframe }
pkt_0104:
// offset +0x0104, 1 encoded words
{ jump 4 }
