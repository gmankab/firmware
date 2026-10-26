// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106cfc,0x106d48); runtime [0xb00c6cfc,0xb00c6d48).
// Index session slots and check linked descriptor availability.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_slot_index
agent14_late_session_slot_index:
pkt_0000:
// offset +0x0000, 2 encoded words
{ r1 = #0x0 ; jump ##pkt_0008 }
pkt_0008:
// offset +0x0008, 2 encoded words
{ r16 = r1; memd(r29+#-0x10) = r17:16; 	allocframe(#0x8) }
pkt_0010:
// offset +0x0010, 2 encoded words
{ r2 = memw(gp+#0x78c); r1 = memuh(r0+#0x0) }
pkt_0018:
// offset +0x0018, 2 encoded words
{ call -551572; r0 = r2 }
pkt_0020:
// offset +0x0020, 2 encoded words
{ r2 = memw(r0+#0xc); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_003c }
pkt_0028:
// offset +0x0028, 2 encoded words
{ r0 = memw(r2+#0x1c); if (cmp.eq(r0.new,#0x0)) jump:nt pkt_003c }
pkt_0030:
// offset +0x0030, 1 encoded words
{ p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t pkt_0044 }
pkt_0034:
// offset +0x0034, 2 encoded words
{ jump 628296; r17:16 = memd(r29+#0x0); 	deallocframe }
pkt_003c:
// offset +0x003c, 2 encoded words
{ r0 = #0x0; r17:16 = memd(r29+#0x0); 	dealloc_return }
pkt_0044:
// offset +0x0044, 2 encoded words
{ jump 628304; r17:16 = memd(r29+#0x0); 	deallocframe }
