// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x1065a8,0x106638); runtime [0xb00c65a8,0xb00c6638).
// Attach session, report status, and preserve load/store packet ordering.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_attach
agent14_late_session_attach:
pkt_0000:
// offset +0x0000, 3 encoded words
{ r16 = r0; r5:4 = combine(#0x0,#0x0); memd(r29+#-0x10) = r17:16; 	allocframe(#0x18) }
pkt_000c:
// offset +0x000c, 4 encoded words
{ p0 = cmp.eq(r1,#0x0); if (!p0.new) r6 = add(r29,#0x0); r3 = memw(gp+#0x78c); r2 = memuh(r16+#0x0) }
pkt_001c:
// offset +0x001c, 2 encoded words
{ if (p0) jump:nt pkt_0068; memd(r29+#0x8) = r5:4; 	memd(r29+#0x0) = r5:4 }
pkt_0024:
// offset +0x0024, 3 encoded words
{ r1:0 = combine(#0x1,#-0x1); memb(r29+#0xe) = r2; memb(r6+#0xf) = #0x7 }
pkt_0030:
// offset +0x0030, 3 encoded words
{ r1:0 = combine(#0x13,#0x2); r2 = ##0xb00c5bac; 	memd(r29+#0x0) = r1:0 }
pkt_003c:
// offset +0x003c, 2 encoded words
{ r4 = ##0xb00c6ab4; 	r5 = add(r29,#0x0) }
pkt_0044:
// offset +0x0044, 2 encoded words
{ call -735856; memw(r6+#0x8) = #0x1 }
pkt_004c:
// offset +0x004c, 3 encoded words
{ call 160; r0 = r16; r1 = memub(r16+#0x14) }
pkt_0058:
// offset +0x0058, 3 encoded words
{ call 284; r0 = r16; r1 = memub(r16+#0x15) }
pkt_0064:
// offset +0x0064, 1 encoded words
{ r2 = #0x1 ; jump pkt_0084 }
pkt_0068:
// offset +0x0068, 2 encoded words
{ call -735196; r0 = #0x13; 	r1 = and(r2,#255) }
pkt_0070:
// offset +0x0070, 2 encoded words
{ call 124; r1:0 = combine(#0x2,r16) }
pkt_0078:
// offset +0x0078, 2 encoded words
{ call 252; r1:0 = combine(#0x2,r16) }
pkt_0080:
// offset +0x0080, 1 encoded words
{ r2 = #0x0 }
pkt_0084:
// offset +0x0084, 2 encoded words
{ memw(r16+#0x84) = r2; r17:16 = memd(r29+#0x10) }:mem_noshuf
pkt_008c:
// offset +0x008c, 1 encoded words
{ dealloc_return }
