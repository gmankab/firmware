// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106578,0x106598); runtime [0xb00c6578,0xb00c6598).
// Traverse a linked list and locate matching session.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_list_find
agent14_late_session_list_find:
pkt_0000:
// offset +0x0000, 1 encoded words
{ r0 = #0x0; 	r2 = memw(r0+#0xc) }
pkt_0004:
// offset +0x0004, 1 encoded words
{ p0 = cmp.eq(r2,#0x0); 	if (p0.new) jumpr:nt r31 }
pkt_0008:
// offset +0x0008, 2 encoded words
{ r3 = memuh(r2+#0x0); if (cmp.eq(r3.new,r1)) jump:nt pkt_001c }
pkt_0010:
// offset +0x0010, 2 encoded words
{ r2 = memw(r2+#0x4); if (!cmp.eq(r2.new,#0x0)) jump:t pkt_0008 }
pkt_0018:
// offset +0x0018, 1 encoded words
{ jumpr r31 }
pkt_001c:
// offset +0x001c, 1 encoded words
{ r0 = r2; 	jumpr r31 }
