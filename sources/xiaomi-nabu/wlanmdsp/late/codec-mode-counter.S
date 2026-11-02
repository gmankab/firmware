// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106318,0x106360); runtime [0xb00c6318,0xb00c6360).
// Parser of variable-size script operands, opcode predicates and bounded hardware loops.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_codec_mode_counter
agent14_late_codec_mode_counter:
pkt_0000:
// offset +0x0000, 2 encoded words
{ r17:16 = combine(r1,r0); memd(r29+#-0x10) = r17:16; 	allocframe(#0x8) }
pkt_0008:
// offset +0x0008, 2 encoded words
{ if (p0.new) r2 = #0x1; p0 = tstbit(r17,#0); if (!p0.new) jump:t pkt_0018 }
pkt_0010:
// offset +0x0010, 2 encoded words
{ call -730748; r1:0 = combine(#0x28,r16) }
pkt_0018:
// offset +0x0018, 3 encoded words
{ p0 = !tstbit(r17,#0x1); if (!p0.new) r2 = #0x2; if (p0.new) jump:t pkt_002c }
pkt_0024:
// offset +0x0024, 2 encoded words
{ call -730768; r1:0 = combine(#0xa,r16) }
pkt_002c:
// offset +0x002c, 3 encoded words
{ p0 = !tstbit(r17,#0x2); r1:0 = combine(#0x5,r16); if (!p0.new) jump:t pkt_003c }
pkt_0038:
// offset +0x0038, 1 encoded words
{ r17:16 = memd(r29+#0x0); 	dealloc_return }
pkt_003c:
// offset +0x003c, 2 encoded words
{ r2 = #0x0; r17:16 = memd(r29+#0x0); 	deallocframe }
pkt_0044:
// offset +0x0044, 1 encoded words
{ jump -730800 }
