// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106694,0x10671c); runtime [0xb00c6694,0xb00c671c).
// Validate session flags and dispatch status changes.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_mode_toggle
agent14_late_session_mode_toggle:
pkt_0000:
// offset +0x0000, 3 encoded words
{ p0 = cmp.eq(r1,#0x4); r16 = r0; memd(r29+#-0x10) = r17:16; 	allocframe(#0x10) }
pkt_000c:
// offset +0x000c, 2 encoded words
{ if (p0) jump:nt pkt_0030; if (p0) r2 = memub(r16+#0x16) }
pkt_0014:
// offset +0x0014, 2 encoded words
{ p0 = cmp.eq(r1,#0x2); if (!p0.new) jump:nt pkt_0050; if (p0.new) r2 = memub(r16+#0x16) }
pkt_001c:
// offset +0x001c, 1 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t pkt_0050 }
pkt_0020:
// offset +0x0020, 2 encoded words
{ call -924; r1 = #0x3; 	r0 = memub(r16+#0x0) }
pkt_0028:
// offset +0x0028, 2 encoded words
{ jump pkt_0050; memb(r16+#0x16) = clrbit(#0x0) }
pkt_0030:
// offset +0x0030, 1 encoded words
{ p0 = tstbit(r2,#0); if (p0.new) jump:t pkt_0050 }
pkt_0034:
// offset +0x0034, 2 encoded words
{ call -1512; r0 = memw(r16+#0x8); 	r1 = memuh(r16+#0x0) }
pkt_003c:
// offset +0x003c, 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0050 }
pkt_0040:
// offset +0x0040, 3 encoded words
{ call -1068; r1 = #0x3; r2 = #0x4; r0 = memub(r16+#0x0) }
pkt_004c:
// offset +0x004c, 1 encoded words
{ memb(r16+#0x16) = setbit(#0x0) }
pkt_0050:
// offset +0x0050, 4 encoded words
{ r0 = ##0x67a0; r1 = #0x2412; r2 = memub(r16+#0x14) }
pkt_0060:
// offset +0x0060, 2 encoded words
{ r3 = memub(r16+#0x16); memw(r29+#0x4) = r3.new }
pkt_0068:
// offset +0x0068, 2 encoded words
{ call 1395384; memw(r29+#0x0) = r2 }
pkt_0070:
// offset +0x0070, 4 encoded words
{ r0 = ##-0x4fd48890; r1 = memub(r16+#0x14); r2 = memub(r16+#0x16) }
pkt_0080:
// offset +0x0080, 2 encoded words
{ jump -803828; r17:16 = memd(r29+#0x8); 	deallocframe }
