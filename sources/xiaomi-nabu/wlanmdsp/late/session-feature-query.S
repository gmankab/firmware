// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x1090f4,0x109114), runtime [0xb00c90f4,0xb00c9114).
// Read session feature and callback readiness flag
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_feature_query
agent14_late_session_feature_query:
pkt_0000:
// +0x0: 2 encoded word(s)
{ r0 = #0x0; r2 = memw(r0+#0xec) }
pkt_0008:
// +0x8: 2 encoded word(s)
{ p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt pkt_001c; if (!p0.new) r3 = memw(r2+#0xbc) }
pkt_0010:
// +0x10: 2 encoded word(s)
{ p0 = tstbit(r3,#0); if (!p0.new) jump:t pkt_001c; if (p0.new) r2 = memw(r2+#0xc0) }
pkt_0018:
// +0x18: 1 encoded word(s)
{ r0 = !cmp.eq(r2,#0x0) }
pkt_001c:
// +0x1c: 1 encoded word(s)
{ jumpr r31 }
