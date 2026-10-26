// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x1062a8,0x106318); runtime [0xb00c62a8,0xb00c6318).
// Parser of variable-size script operands, opcode predicates and bounded hardware loops.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_codec_mode_notify
agent14_late_codec_mode_notify:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1963216; allocframe(#0x18) }
pkt_0008:
// offset +0x0008, 2 encoded words
{ r18 = r1; r17:16 = combine(r0,r2) }
pkt_0010:
// offset +0x0010, 3 encoded words
{ r3:2 = combine(#0x1,#0xb); r1:0 = combine(#0x28,r17); p0 = tstbit(r18,#0); if (!p0.new) jump:t pkt_002c }
pkt_001c:
// offset +0x001c, 3 encoded words
{ r5:4 = combine(#0x0,r16); memw(r29+#0x4) = #0x0; memw(r29+#0x0) = #0x0 }
pkt_0028:
// offset +0x0028, 1 encoded words
{ call -730804 }
pkt_002c:
// offset +0x002c, 4 encoded words
{ p0 = !tstbit(r18,#0x1); r3:2 = combine(#0x2,#0xb); r1:0 = combine(#0xa,r17); if (p0.new) jump:t pkt_004c }
pkt_003c:
// offset +0x003c, 3 encoded words
{ r5:4 = combine(#0x0,r16); memw(r29+#0x4) = #0x0; memw(r29+#0x0) = #0x0 }
pkt_0048:
// offset +0x0048, 1 encoded words
{ call -730836 }
pkt_004c:
// offset +0x004c, 4 encoded words
{ p0 = !tstbit(r18,#0x2); r3:2 = combine(#0x0,#0xb); r1:0 = combine(#0x5,r17); if (p0.new) jump:t pkt_006c }
pkt_005c:
// offset +0x005c, 3 encoded words
{ r5:4 = combine(#0x0,r16); memw(r29+#0x4) = #0x0; memw(r29+#0x0) = #0x0 }
pkt_0068:
// offset +0x0068, 1 encoded words
{ call -730868 }
pkt_006c:
// offset +0x006c, 1 encoded words
{ jump 1963328 }
