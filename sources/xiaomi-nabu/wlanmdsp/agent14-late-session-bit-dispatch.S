// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x10671c,0x1067d4); runtime [0xb00c671c,0xb00c67d4).
// Update per-session status flag bits and invoke callbacks.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_bit_dispatch
agent14_late_session_bit_dispatch:
pkt_0000:
// offset +0x0000, 2 encoded words
{ r17:16 = combine(r1,r0); memd(r29+#-0x10) = r17:16; 	allocframe(#0x10) }
pkt_0008:
// offset +0x0008, 1 encoded words
{ r2 = memb(r16+#0x15) }
pkt_000c:
// offset +0x000c, 2 encoded words
{ p0 = cmp.eq(r2,#0x8); if (p0.new) r0 = add(r16,#0x0) }
pkt_0014:
// offset +0x0014, 2 encoded words
{ if (p0) call ##396 }
pkt_001c:
// offset +0x001c, 1 encoded words
{ p0 = cmp.eq(r17,#0x8); if (p0.new) jump:nt pkt_0048 }
pkt_0020:
// offset +0x0020, 2 encoded words
{ p0 = cmp.eq(r17,#0x4); if (p0.new) jump:nt pkt_0060; if (p0.new) r2 = memub(r16+#0x16) }
pkt_0028:
// offset +0x0028, 2 encoded words
{ p0 = cmp.eq(r17,#0x2); if (!p0.new) jump:t pkt_0080; if (p0.new) r2 = memub(r16+#0x16) }
pkt_0030:
// offset +0x0030, 2 encoded words
{ p0 = !tstbit(r2,#0x1); if (p0.new) jump:t pkt_0080 }
pkt_0038:
// offset +0x0038, 2 encoded words
{ call -1084; r1 = #0x4; 	r0 = memub(r16+#0x0) }
pkt_0040:
// offset +0x0040, 2 encoded words
{ jump pkt_0080; memb(r16+#0x16) = clrbit(#0x1) }
pkt_0048:
// offset +0x0048, 2 encoded words
{ call -1668; r0 = memw(r16+#0x8); 	r1 = memuh(r16+#0x0) }
pkt_0050:
// offset +0x0050, 2 encoded words
{ if (!p0.new) r0 = add(r16,#0x0); p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0080 }
pkt_0058:
// offset +0x0058, 1 encoded words
{ call 424 }
pkt_005c:
// offset +0x005c, 1 encoded words
{ jump pkt_0074 }
pkt_0060:
// offset +0x0060, 2 encoded words
{ p0 = tstbit(r2,#0x1); if (p0.new) jump:t pkt_0080 }
pkt_0068:
// offset +0x0068, 2 encoded words
{ call -1700; r0 = memw(r16+#0x8); 	r1 = memuh(r16+#0x0) }
pkt_0070:
// offset +0x0070, 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0080 }
pkt_0074:
// offset +0x0074, 2 encoded words
{ call 564; r0 = memub(r16+#0x0) }
pkt_007c:
// offset +0x007c, 1 encoded words
{ memb(r16+#0x16) = setbit(#0x1) }
pkt_0080:
// offset +0x0080, 4 encoded words
{ r0 = ##0x67a8; r1 = #0x2412; r2 = memub(r16+#0x15) }
pkt_0090:
// offset +0x0090, 2 encoded words
{ r3 = memub(r16+#0x16); memw(r29+#0x4) = r3.new }
pkt_0098:
// offset +0x0098, 2 encoded words
{ call 1395200; memw(r29+#0x0) = r2 }
pkt_00a0:
// offset +0x00a0, 4 encoded words
{ r0 = ##-0x4fd48884; r1 = memub(r16+#0x15); r2 = memub(r16+#0x16) }
pkt_00b0:
// offset +0x00b0, 2 encoded words
{ jump -804012; r17:16 = memd(r29+#0x8); 	deallocframe }
