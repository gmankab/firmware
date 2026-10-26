// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106044,0x1060e0); runtime [0xb00c6044,0xb00c60e0).
// Parser of variable-size script operands, opcode predicates and bounded hardware loops.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_record_release
agent14_late_session_record_release:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1963828; allocframe(#0x10) }
pkt_0008:
// offset +0x0008, 2 encoded words
{ call 1324; r18 = r1; 	r16 = r0 }
pkt_0010:
// offset +0x0010, 2 encoded words
{ r17 = r0; if (cmp.eq(r17.new,#0x0)) jump:nt pkt_0078 }
pkt_0018:
// offset +0x0018, 2 encoded words
{ r1 = memw(r17+#0x78); if (cmp.eq(r1.new,#0x0)) jump:nt pkt_002c }
pkt_0020:
// offset +0x0020, 2 encoded words
{ call 1425376; r0 = memw(r16+#0x14) }
pkt_0028:
// offset +0x0028, 1 encoded words
{ memw(r17+#0x78) = #0x0 }
pkt_002c:
// offset +0x002c, 2 encoded words
{ call 1492; r1:0 = combine(r18,r16) }
pkt_0034:
// offset +0x0034, 3 encoded words
{ r3 = add(r17,#0x88); r1:0 = combine(#0x0,#0x0); r2 = memw(r17+#0x90) }
pkt_0040:
// offset +0x0040, 3 encoded words
{ p0 = cmp.eq(r2,#0x0); if (!p0.new) memw(r16+#0x20) = #0x0; if (!p0.new) memd(r3+#0x0) = r1:0 }
pkt_004c:
// offset +0x004c, 1 encoded words
{ if (!p0) memw(r3+#0x8) = #0x0 }
pkt_0050:
// offset +0x0050, 2 encoded words
{ r3 = memw(r16+#0xc); if (cmp.eq(r3.new,r17)) jump:nt pkt_007c }
pkt_0058:
// offset +0x0058, 3 encoded words
{ r2 = r3; r3 = memw(r3+#0x4); if (!cmp.eq(r3.new,r17)) jump:t pkt_0058 }
pkt_0064:
// offset +0x0064, 2 encoded words
{ r3 = memw(r17+#0x4); memw(r2+#0x4) = r3.new }
pkt_006c:
// offset +0x006c, 3 encoded words
{ p0 = cmp.eq(r3,#0x0); if (!p0.new) jump:t pkt_0090; jump pkt_008c; r2 = add(r2,#0x4) }
pkt_0078:
// offset +0x0078, 1 encoded words
{ jump 1963928 }
pkt_007c:
// offset +0x007c, 2 encoded words
{ r2 = memw(r17+#0x4); memw(r16+#0xc) = r2.new }
pkt_0084:
// offset +0x0084, 2 encoded words
{ if (p0.new) r2 = add(r16,#0xc); p0 = cmp.eq(r2,#0x0); if (!p0.new) jump:t pkt_0090 }
pkt_008c:
// offset +0x008c, 1 encoded words
{ memw(r16+#0x10) = r2 }
pkt_0090:
// offset +0x0090, 2 encoded words
{ call 1963800; r1 = r17; 	r0 = memw(r16+#0x8) }
pkt_0098:
// offset +0x0098, 1 encoded words
{ jump 1425256 }
