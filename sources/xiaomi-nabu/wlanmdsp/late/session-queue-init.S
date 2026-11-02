// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x10691c,0x1069c4); runtime [0xb00c691c,0xb00c69c4).
// Initialize buffers and register session event handling.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_queue_init
agent14_late_session_queue_init:
pkt_0000:
// offset +0x0000, 2 encoded words
{ r16 = r0; memd(r29+#-0x10) = r17:16; 	allocframe(#0x10) }
pkt_0008:
// offset +0x0008, 3 encoded words
{ call -550564; r0 = memw(gp+#0x78c); r1 = memuh(r16+#0x0) }
pkt_0014:
// offset +0x0014, 2 encoded words
{ call 1400000; r17 = r0 }
pkt_001c:
// offset +0x001c, 3 encoded words
{ r1 = #0x2451; r0 = ##0x67f0; 	memd(r29+#0x0) = r1:0 }
pkt_0028:
// offset +0x0028, 1 encoded words
{ call 1394800 }
pkt_002c:
// offset +0x002c, 1 encoded words
{ call 1399976 }
pkt_0030:
// offset +0x0030, 1 encoded words
{ r3:2 = combine(r1,r0) }
pkt_0034:
// offset +0x0034, 3 encoded words
{ call -802056; r1:0 = combine(r2,##-0x4fd48818) }
pkt_0040:
// offset +0x0040, 4 encoded words
{ call -582792; r1:0 = combine(##-0x4ff392b0,r17); r2 = r16 }
pkt_0050:
// offset +0x0050, 2 encoded words
{ r1:0 = combine(#0x0,#0x0); memw(r16+#0x44) = #0x0 }
pkt_0058:
// offset +0x0058, 2 encoded words
{ call 1399932; memd(r16+#0x38) = r1:0 }
pkt_0060:
// offset +0x0060, 4 encoded words
{ call 896; r0 = r16; memw(r16+#0x40) = r0; memw(r16+#0x48) = #0x0 }
pkt_0070:
// offset +0x0070, 4 encoded words
{ call 956; r0 = r16; r17 = add(r16,#0x18); memw(r16+#0x4c) = r0 }
pkt_0080:
// offset +0x0080, 4 encoded words
{ call 1396672; r0 = r17; memw(r16+#0x50) = r0; memw(r16+#0x54) = #0x1 }
pkt_0090:
// offset +0x0090, 4 encoded words
{ r0 = r17; r4 = #0x0; r17:16 = memd(r29+#0x8); r3:2 = memd(gp+#0xd58) }
pkt_00a0:
// offset +0x00a0, 2 encoded words
{ jump 1396516; deallocframe }
