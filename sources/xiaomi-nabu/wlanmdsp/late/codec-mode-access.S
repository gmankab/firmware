// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106360,0x1063f4); runtime [0xb00c6360,0xb00c63f4).
// Parser of variable-size script operands, opcode predicates and bounded hardware loops.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_codec_mode_access
agent14_late_codec_mode_access:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1963032; allocframe(#0x18) }
pkt_0008:
// offset +0x0008, 2 encoded words
{ call 1067512; r0 = #0x0; 	r16 = r0 }
pkt_0010:
// offset +0x0010, 3 encoded words
{ call 1409392; r1 = #0x3e; r2 = #0x20; r17 = r0 }
pkt_001c:
// offset +0x001c, 2 encoded words
{ r18 = r0; if (cmp.eq(r18.new,#0x0)) jump:nt pkt_0070 }
pkt_0024:
// offset +0x0024, 3 encoded words
{ call -803804; r0 = add(r18,#0xc); r1 = #0x0; 	r2 = #0x14 }
pkt_0030:
// offset +0x0030, 4 encoded words
{ r2 = ##0x36001; r1 = ##0x40018 }
pkt_0040:
// offset +0x0040, 3 encoded words
{ r0 = ##0x1bd0014; memw(r18+#0x8) = r0.new }
pkt_004c:
// offset +0x004c, 2 encoded words
{ r0 = r17; memw(r18+#0x0) = r2; 	memw(r18+#0x4) = r1 }
pkt_0054:
// offset +0x0054, 2 encoded words
{ r2 = memw(r16+#0x0); memw(r18+#0xc) = r2.new }
pkt_005c:
// offset +0x005c, 2 encoded words
{ memw(r18+#0x10) = #0x1; r2 = memw(r16+#0x18) }:mem_noshuf
pkt_0064:
// offset +0x0064, 2 encoded words
{ call 1963048; memw(r18+#0x14) = r2 }
pkt_006c:
// offset +0x006c, 1 encoded words
{ jump 1409332 }
pkt_0070:
// offset +0x0070, 4 encoded words
{ r0 = ##0x6778; r1 = #0x3e51; memw(r29+#0x0) = #0x20 }
pkt_0080:
// offset +0x0080, 1 encoded words
{ call 1396180 }
pkt_0084:
// offset +0x0084, 3 encoded words
{ call 1963016; r1:0 = combine(#0x20,##0xb02b76b8) }
pkt_0090:
// offset +0x0090, 1 encoded words
{ jump -800680 }
