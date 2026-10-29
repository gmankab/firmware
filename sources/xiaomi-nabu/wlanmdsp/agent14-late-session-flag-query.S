// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x1090dc,0x1090f4), runtime [0xb00c90dc,0xb00c90f4).
// Extract session activation flag from linked controller context
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_flag_query
agent14_late_session_flag_query:
pkt_0000:
// +0x0: 2 encoded word(s)
{ r2 = memw(r0+#0xec); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_0014 }
pkt_0008:
// +0x8: 1 encoded word(s)
{ r2 = memw(r2+#0xb8) }
pkt_000c:
// +0xc: 2 encoded word(s)
{ r0 = extractu(r2,#0x1,#0x1); jumpr r31 }
pkt_0014:
// +0x14: 1 encoded word(s)
{ r0 = #0x0; 	jumpr r31 }
