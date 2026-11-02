// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x1060e0,0x106128); runtime [0xb00c60e0,0xb00c6128).
// Parser of variable-size script operands, opcode predicates and bounded hardware loops.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_status_guard
agent14_late_session_status_guard:
pkt_0000:
// offset +0x0000, 1 encoded words
{ r0 = #0x0; 	r2 = r0 }
pkt_0004:
// offset +0x0004, 1 encoded words
{ p0 = cmp.eq(r2,#0x0); 	if (p0.new) jumpr:nt r31 }
pkt_0008:
// offset +0x0008, 2 encoded words
{ call 1168; r0 = r2; 	allocframe(#0x0) }
pkt_0010:
// offset +0x0010, 2 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0044; deallocframe }
pkt_0018:
// offset +0x0018, 2 encoded words
{ r2 = memw(r0+#0x78); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_0044 }
pkt_0020:
// offset +0x0020, 2 encoded words
{ r2 = memw(r0+#0x80); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_0044 }
pkt_0028:
// offset +0x0028, 2 encoded words
{ r2 = memw(r0+#0x6c); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_0044 }
pkt_0030:
// offset +0x0030, 2 encoded words
{ r0 = #0x1; r2 = memuh(r0+#0x66) }
pkt_0038:
// offset +0x0038, 2 encoded words
{ p0 = tstbit(r2,#0x0); if (p0.new) r0 = #0x0 }
pkt_0040:
// offset +0x0040, 1 encoded words
{ jumpr r31 }
pkt_0044:
// offset +0x0044, 1 encoded words
{ r0 = #0x0; 	jumpr r31 }
