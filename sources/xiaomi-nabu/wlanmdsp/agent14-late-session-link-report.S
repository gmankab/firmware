// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106d50,0x106d84); runtime [0xb00c6d50,0xb00c6d84).
// Report linked session state through a short tail callback.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_link_report
agent14_late_session_link_report:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1960488; allocframe(#0x18) }
pkt_0008:
// offset +0x0008, 3 encoded words
{ call 1064968; r17:16 = combine(r2,r0); r0 = #0x1; 	r18 = r1 }
pkt_0014:
// offset +0x0014, 2 encoded words
{ call 1403172; r19 = r0 }
pkt_001c:
// offset +0x001c, 3 encoded words
{ r2 = r0; r5 = #0x0; r1:0 = combine(#0x47,r19); r4 = memw(r18+#0x0) }
pkt_0028:
// offset +0x0028, 2 encoded words
{ call 1406428; memw(r29+#0x4) = r16; 	memw(r29+#0x0) = r17 }
pkt_0030:
// offset +0x0030, 1 encoded words
{ jump 1960660 }
