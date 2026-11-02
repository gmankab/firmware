// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x1068bc,0x10691c); runtime [0xb00c68bc,0xb00c691c).
// Initialize a session event queue and callback registrations.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_queue_create
agent14_late_session_queue_create:
pkt_0000:
// offset +0x0000, 2 encoded words
{ r16 = r0; memd(r29+#-0x10) = r17:16; 	allocframe(#0x10) }
pkt_0008:
// offset +0x0008, 3 encoded words
{ call -550468; r0 = memw(gp+#0x78c); r1 = memuh(r16+#0x0) }
pkt_0014:
// offset +0x0014, 3 encoded words
{ call 1400096; r17 = r0; memw(r16+#0x54) = #0x0 }
pkt_0020:
// offset +0x0020, 3 encoded words
{ r1 = #0x2451; r0 = ##0x67f8; 	memd(r29+#0x0) = r1:0 }
pkt_002c:
// offset +0x002c, 1 encoded words
{ call 1394892 }
pkt_0030:
// offset +0x0030, 1 encoded words
{ call 1400068 }
pkt_0034:
// offset +0x0034, 1 encoded words
{ r3:2 = combine(r1,r0) }
pkt_0038:
// offset +0x0038, 3 encoded words
{ call -801964; r1:0 = combine(r2,##-0x4fd4880c) }
pkt_0044:
// offset +0x0044, 4 encoded words
{ call -580408; r1:0 = combine(##-0x4ff392b0,r17); r2 = r16 }
pkt_0054:
// offset +0x0054, 2 encoded words
{ r0 = add(r16,#0x18); r17:16 = memd(r29+#0x8); 	deallocframe }
pkt_005c:
// offset +0x005c, 1 encoded words
{ jump 1396804 }
