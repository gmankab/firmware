// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106644,0x106694); runtime [0xb00c6644,0xb00c6694).
// Find session record by token and dispatch cleanup.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_seek
agent14_late_session_seek:
pkt_0000:
// offset +0x0000, 2 encoded words
{ r17 = r1; memd(r29+#-0x10) = r17:16; 	allocframe(#0x8) }
pkt_0008:
// offset +0x0008, 2 encoded words
{ r16 = memw(r0+#0xc); if (cmp.eq(r16.new,#0x0)) jump:nt pkt_0030 }
pkt_0010:
// offset +0x0010, 2 encoded words
{ r2 = memuh(r16+#0x0); if (cmp.eq(r2.new,r17)) jump:nt pkt_0024 }
pkt_0018:
// offset +0x0018, 2 encoded words
{ r16 = memw(r16+#0x4); if (!cmp.eq(r16.new,#0x0)) jump:t pkt_0010 }
pkt_0020:
// offset +0x0020, 1 encoded words
{ jump pkt_0030 }
pkt_0024:
// offset +0x0024, 2 encoded words
{ call -1416; r1 = r17 }
pkt_002c:
// offset +0x002c, 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0034 }
pkt_0030:
// offset +0x0030, 1 encoded words
{ r17:16 = memd(r29+#0x0); 	dealloc_return }
pkt_0034:
// offset +0x0034, 2 encoded words
{ call -735300; r0 = #0x13; 	r1 = and(r17,#255) }
pkt_003c:
// offset +0x003c, 2 encoded words
{ call 20; r1:0 = combine(#0x2,r16) }
pkt_0044:
// offset +0x0044, 2 encoded words
{ r1:0 = combine(#0x2,r16); r17:16 = memd(r29+#0x0); 	deallocframe }
pkt_004c:
// offset +0x004c, 1 encoded words
{ jump 140 }
